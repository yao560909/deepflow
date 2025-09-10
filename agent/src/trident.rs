use std::{fmt, thread};
use std::fmt::Formatter;
use std::net::IpAddr;
use std::path::Path;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::AtomicBool;
use std::thread::JoinHandle;
use log::error;
use anyhow::{anyhow, Result};
use public::consts::DEFAULT_TRIDENT_CONF_FILE;
use crate::config::config::{Config, ConfigError};
use crate::config::handler::ConfigHandler;
use crate::utils::environment::get_ctrl_ip_and_mac;
use crate::utils::command::get_hostname;
pub struct VersionInfo {
    pub name: &'static str,
    pub branch: &'static str,
    pub commit_id: &'static str,
    pub rev_count: &'static str,
    pub compiler: &'static str,
    pub compile_time: &'static str,

    pub revision: &'static str,
}


impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}
Name: {}
Branch: {}
CommitId: {}
RevCount: {}
Compiler: {}
CompileTime: {}",
            self.rev_count,
            self.commit_id,
            match self.name {
                "deepflow-agent-ce" => "deepflow-agent community edition",
                "deepflow-agent-ee" => "deepflow-agent enterprise edition",
                _ => panic!("{:?} unknown deepflow-agent edition", &self.name),
            },
            self.branch,
            self.commit_id,
            self.rev_count,
            self.compiler,
            self.compile_time
        )
    }
}

#[derive(Copy, Clone, Debug)]
struct InnerState {
    enabled: bool,
    melted_down: bool,
}

#[derive(Default)]
pub struct AgentState {
    // terminated is outside of Mutex because during termination, state will be locked in main thread,
    // and the main thread will try to stop other threads, in which may lock and update agent state,
    // causing a deadlock. Checking terminated state before locking inner state will avoid this deadlock.
    terminated: AtomicBool,
    // state: Mutex<(InnerState, Option<ChangedConfig>)>,
    notifier: Condvar,
}

pub struct Trident {
    state: Arc<AgentState>,
    handle: Option<JoinHandle<()>>,
}

#[derive(Clone, Default, Copy, PartialEq, Eq, Debug)]
pub enum RunningMode {
    #[default]
    Managed,
    Standalone,
}

impl Trident{
    pub fn start<P: AsRef<Path>>(
        config_path: P,
        version_info: &'static VersionInfo,
        agent_mode: RunningMode,
        sidecar_mode: bool,
        cgroups_disabled: bool,
    ) -> Result<Trident>{
        let config = match agent_mode {
            RunningMode::Managed => {
                match Config::load_from_file(config_path.as_ref()) {
                    Ok(conf)=>conf,
                    Err(e) => {
                        if let ConfigError::YamlConfigInvalid(_) = e {
                            // try to load config file from trident.yaml to support upgrading from trident
                            if let Ok(conf) = Config::load_from_file(DEFAULT_TRIDENT_CONF_FILE) {
                                conf
                            } else {
                                // return the original error instead of loading trident conf
                                return Err(e.into());
                            }
                        } else {
                            return Err(e.into());
                        }
                    }
                }
            }
            RunningMode::Standalone => {
                let mut conf = Config::default();
                conf.controller_ips = vec!["127.0.0.1".into()];
                conf.agent_mode = agent_mode;
                conf
            }
        };
        #[cfg(target_os = "linux")]
        if !config.pid_file.is_empty() {
            if let Err(e) = crate::utils::pid_file::open(&config.pid_file) {
                return Err(anyhow!("Create pid file {} failed: {}", config.pid_file, e));
            }
        };
        let controller_ip: IpAddr = config.controller_ips[0].parse()?;
        let (ctrl_ip, ctrl_mac) = match get_ctrl_ip_and_mac(&controller_ip) {
            Ok(tuple) => tuple,
            Err(e) => return Err(anyhow!("get ctrl ip and mac failed: {}", e)),
        };
        let mut config_handler = ConfigHandler::new(config, ctrl_ip, ctrl_mac);
        let config = &config_handler.static_config;
        let cgroups_disabled = cgroups_disabled || config.cgroups_disabled;
        let hostname = match config.override_os_hostname.as_ref() {
            Some(name) => name.to_owned(),
            None => get_hostname().unwrap_or("Unknown".to_string()),
        };

        let state = Arc::new(AgentState::default());
        let main_loop = thread::Builder::new()
            .name("main-loop".to_owned())
            .spawn(move || {println!("start main-loop")});
        let handle = match main_loop {
            Ok(h) => Some(h),
            Err(e) => {
                error!("Failed to create main-loop thread: {}", e);
                crate::utils::clean_and_exit(1);
                None
            }
        };
        Ok(Trident { state, handle })
    }
}
