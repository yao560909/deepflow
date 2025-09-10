#[cfg(target_os = "linux")]
mod platform_consts {
    pub const DEFAULT_LOG_FILE: &'static str = "/var/log/deepflow-agent/deepflow-agent.log";
    pub const DEFAULT_CONF_FILE: &'static str = "/etc/deepflow-agent.yaml";
    pub const DEFAULT_TRIDENT_CONF_FILE: &'static str = "/etc/trident.yaml";
    pub const COREFILE_FORMAT: &'static str = "core";
    pub const DEFAULT_COREFILE_PATH: &'static str = "/tmp";
    pub const DEFAULT_LIBVIRT_XML_PATH: &'static str = "/etc/libvirt/qemu";
}

//TODO:for macos
#[cfg(target_os = "macos")]
mod platform_consts {
    pub const DEFAULT_LOG_FILE: &'static str = "./log/deepflow-agent/deepflow-agent.log";
    pub const DEFAULT_CONF_FILE: &'static str = "./deepflow-agent.yaml";
    pub const DEFAULT_TRIDENT_CONF_FILE: &'static str = "./trident.yaml";
    pub const COREFILE_FORMAT: &'static str = "core";
    pub const DEFAULT_COREFILE_PATH: &'static str = "./tmp";
    pub const DEFAULT_LIBVIRT_XML_PATH: &'static str = "/etc/libvirt/qemu";
}

pub use platform_consts::*;