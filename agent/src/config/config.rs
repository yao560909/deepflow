use std::{env, fs};
use std::net::{IpAddr, ToSocketAddrs};
use std::path::Path;
use crate::trident::RunningMode;

use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer,
};
use thiserror::Error;
use crate::common::DEFAULT_LOG_FILE;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum AgentIdType {
    #[default]
    IpMac,
    Ip,
}
impl<'de> Deserialize<'de> for AgentIdType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "ip-and-mac" | "ip_and_mac" => Ok(Self::IpMac),
            "ip" => Ok(Self::Ip),
            other => Err(de::Error::invalid_value(
                Unexpected::Str(other),
                &"ip|ip-and-mac|ip_and_mac",
            )),
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("controller-ips is empty")]
    ControllerIpsEmpty,
    #[error("controller-ips invalid")]
    ControllerIpsInvalid,
    #[error("runtime config invalid: {0}")]
    RuntimeConfigInvalid(String),
    #[error("yaml config invalid: {0}")]
    YamlConfigInvalid(String),
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(default, rename_all = "kebab-case")]
pub struct Config {
    pub controller_ips: Vec<String>,
    pub controller_port: u16,
    pub controller_tls_port: u16,
    pub controller_cert_file_prefix: String,
    pub log_file: String,
    pub kubernetes_cluster_id: String,
    pub kubernetes_cluster_name: Option<String>,
    pub vtap_group_id_request: String,
    pub controller_domain_name: Vec<String>,
    #[serde(skip)]
    pub agent_mode: RunningMode,
    pub override_os_hostname: Option<String>,
    pub async_worker_thread_number: u16,
    pub agent_unique_identifier: AgentIdType,
    #[cfg(target_os = "linux")]
    pub pid_file: String,
    pub team_id: String,
    pub cgroups_disabled: bool,
}

impl Config{
    pub fn load_from_file<T:AsRef<Path>>(path: T)->Result<Self,ConfigError>{
        let contents = fs::read_to_string(path).map_err(|e| ConfigError::YamlConfigInvalid(e.to_string()))?;
        Self::load(&contents)
    }
    pub fn load<C: AsRef<str>>(contents:C)->Result<Self,ConfigError>{
        let contents = contents.as_ref();
        if contents.len()==0{
            Ok(Self::default())
        }else {
            let mut cfg: Self = serde_yaml::from_str(contents)
                .map_err(|e| ConfigError::YamlConfigInvalid(e.to_string()))?;
            for i in 0..cfg.controller_ips.len() {
                if cfg.controller_ips[i].parse::<IpAddr>().is_err() {
                    let ip = resolve_domain(&cfg.controller_ips[i]);
                    if ip.is_none() {
                        return Err(ConfigError::ControllerIpsInvalid);
                    }

                    cfg.controller_domain_name
                        .push(cfg.controller_ips[i].clone());
                    cfg.controller_ips[i] = ip.unwrap();
                }
            }
            // convert relative path to absolute
            if Path::new(&cfg.log_file).is_relative() {
                let Ok(mut pb) = env::current_dir() else {
                    return Err(ConfigError::YamlConfigInvalid("get cwd failed".to_owned()));
                };
                pb.push(&cfg.log_file);
                match pb.to_str() {
                    Some(s) => cfg.log_file = s.to_owned(),
                    None => {
                        return Err(ConfigError::YamlConfigInvalid(format!(
                            "invalid log path {}",
                            cfg.log_file
                        )));
                    }
                }
            }

            Ok(cfg)
        }

    }
}


impl Default for Config {
    fn default() -> Self {
        Self {
            controller_ips: vec![],
            controller_port: 30035,
            controller_tls_port: 30135,
            controller_cert_file_prefix: "".into(),
            log_file: DEFAULT_LOG_FILE.into(),
            kubernetes_cluster_id: "".into(),
            kubernetes_cluster_name: Default::default(),
            vtap_group_id_request: "".into(),
            controller_domain_name: vec![],
            agent_mode: Default::default(),
            override_os_hostname: None,
            async_worker_thread_number: 16,
            agent_unique_identifier: Default::default(),
            #[cfg(target_os = "linux")]
            pid_file: Default::default(),
            team_id: "".into(),
            cgroups_disabled: false,
        }
    }
}

fn resolve_domain(addr: &str) -> Option<String> {
    match format!("{}:1", addr).to_socket_addrs() {
        Ok(mut addr) => match addr.next() {
            Some(addr) => Some(addr.ip().to_string()),
            None => None,
        },
        Err(e) => {
            eprintln!("{:?}", e);
            None
        }
    }
}