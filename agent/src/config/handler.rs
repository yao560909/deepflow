use std::net::IpAddr;
use std::sync::Arc;
use arc_swap::ArcSwap;
use flexi_logger::LoggerHandle;
use public::utils::net::MacAddr;
use crate::config::config::Config;

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleConfig {
    pub enabled: bool,
}

pub struct ConfigHandler {
    pub ctrl_ip: IpAddr,
    pub ctrl_mac: MacAddr,
    pub container_cpu_limit: u32, // unit: milli-core
    pub container_mem_limit: u64, // unit: bytes
    pub logger_handle: Option<LoggerHandle>,
    // need update
    pub static_config: Config,
    pub candidate_config: ModuleConfig,
    pub current_config: Arc<ArcSwap<ModuleConfig>>,
}
impl ConfigHandler {
    pub fn new(config: Config, ctrl_ip: IpAddr, ctrl_mac: MacAddr) -> Self {

        #[cfg(any(target_os = "linux", target_os = "android"))]
        let (container_cpu_limit, container_mem_limit) = get_container_resource_limits();
        #[cfg(target_os = "windows")]
        let (container_cpu_limit, container_mem_limit) = (0, 0);
        //TODO:for macos
        #[cfg(target_os = "macos")]
        let (container_cpu_limit, container_mem_limit) = (0, 0);
        let candidate_config =
            ModuleConfig{
                enabled: false,
            };
        let current_config = Arc::new(ArcSwap::from_pointee(candidate_config.clone()));

        Self{
            static_config: config,
            ctrl_ip,
            ctrl_mac,
            container_cpu_limit,
            container_mem_limit,
            candidate_config,
            current_config,
            logger_handle: None,
        }
    }
}
