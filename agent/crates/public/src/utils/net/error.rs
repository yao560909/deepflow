use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("parse mac address failed: {0}")]
    ParseMacFailed(String),
    #[error("neighbor lookup failed from: {0}")]
    NeighborLookup(String),
    #[error("link not found: {0}")]
    LinkNotFound(String),
    #[error("link not found index: {0}")]
    LinkNotFoundIndex(u32),
    #[error("link regex invalid")]
    LinkRegexInvalid(#[from] regex::Error),
    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[error("netlink error: {0}")]
    NetlinkError(String),
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("no route to host: {0}")]
    NoRouteToHost(String),
    #[error("Windows related error:{0}")]
    Windows(String),
    #[error("{0}")]
    LinkIdxNotFoundByIP(String),
    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[error(transparent)]
    Errno(#[from] nix::errno::Errno),
    #[error("ethtool: {0}")]
    Ethtool(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;