mod trident;

use std::panic;
use anyhow::Result;
use log::error;
use clap::{ArgAction,Parser};

#[derive(Parser)]
struct Opts {
    /// Specify config file location
    #[clap(
        short = 'f',
        visible_short_alias = 'c',
        long,
        default_value = "/etc/deepflow-agent.yaml"
    )]
    config_file: String,

    /// Enable standalone mode, default config path is /etc/deepflow-agent-standalone.yaml
    #[clap(long)]
    standalone: bool,

    /// Display the version
    #[clap(short, long, action = ArgAction::SetTrue)]
    version: bool,

    /// Dump interface info
    #[clap(long = "dump-ifs")]
    dump_interfaces: bool,

    // TODO: use enum type
    /// Interface mac source type, used with '--dump-ifs'
    #[clap(long, default_value = "mac")]
    if_mac_source: String,

    /// Libvirt XML path, used with '--dump-ifs' and '--if-mac-source xml'
    #[clap(long, default_value = "/etc/libvirt/qemu")]
    xml_path: String,

    /// Check privileges under kubernetes
    #[clap(long)]
    check_privileges: bool,

    /// Grant capabilities including cap_net_admin, cap_net_raw,cap_net_bind_service
    #[clap(long)]
    add_cap: bool,

    /// Run agent in sidecar mode.
    /// Environment variable `CTRL_NETWORK_INTERFACE` must be specified and
    /// optionally `K8S_POD_IP_FOR_DEEPFLOW` can be set to override ip address.
    #[clap(long)]
    sidecar: bool,

    /// Disable cgroups, deepflow-agent will default to checking the CPU and memory resource usage in a loop every 10 seconds to prevent resource usage from exceeding limits.
    #[clap(long)]
    cgroups_disabled: bool,
}

const VERSION_INFO: &'static trident::VersionInfo = &trident::VersionInfo {
    name: env!("AGENT_NAME"),
    branch: env!("BRANCH"),
    commit_id: env!("COMMIT_ID"),
    rev_count: env!("REV_COUNT"),
    compiler: env!("RUSTC_VERSION"),
    compile_time: env!("COMPILE_TIME"),

    revision: concat!(
    env!("BRANCH"),
    " ",
    env!("REV_COUNT"),
    "-",
    env!("COMMIT_ID")
    ),
};

fn main() -> Result<()>{
    panic::set_hook(Box::new(|panic_info| {
        error!("{panic_info}");
        error!("{}", std::backtrace::Backtrace::force_capture());
    }));
    let opts = Opts::parse();
    if opts.version {
        println!("{}", VERSION_INFO);
        return Ok(());
    }
    Ok(())
}