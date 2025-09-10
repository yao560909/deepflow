use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("parse mac address failed from: {0}")]
    ParseMacFailed(String),
    #[error("call try_from() failed from {0}")]
    TryFromFailed(String),
    #[cfg(target_os = "linux")]
    #[error(transparent)]
    KubeWatcher(#[from] kube::runtime::watcher::Error),
    #[error("PlatformSynchronizer failed: {0} ")]
    PlatformSynchronizer(String),
    #[error("data not found: {0}")]
    NotFound(String),
    #[error("Kubernetes ApiWatcher error: {0}")]
    KubernetesApiWatcher(String),
    #[error("system: {0}")]
    SysMonitor(String),
    #[error("environment error: {0}")]
    Environment(String),
    #[error("parse packet failed from: {0}")]
    ParsePacketFailed(String),
    #[error("invalid tpacket version: {0}")]
    InvalidTpVersion(isize),
    #[error("windows error: {0}")]
    Windows(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;