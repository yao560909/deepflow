use std::env;
use std::net::IpAddr;
use log::warn;
use public::utils::net::{MacAddr};
use crate::error::{Error, Result};

const ENV_INTERFACE_NAME: &str = "CTRL_NETWORK_INTERFACE";
const K8S_POD_IP_FOR_DEEPFLOW: &str = "K8S_POD_IP_FOR_DEEPFLOW";

pub fn get_ctrl_ip_and_mac(dest: &IpAddr) -> Result<(IpAddr, MacAddr)> {
    // Steps to find ctrl ip and mac:
    // 1. If environment variable `ENV_INTERFACE_NAME` exists, use it as ctrl interface
    //    a) Use environment variable `K8S_POD_IP_FOR_DEEPFLOW` as ctrl ip if it exists
    //    b) If not, find addresses on the ctrl interface
    // 2. Use env.K8S_NODE_IP_FOR_DEEPFLOW as the ctrl_ip reported by deepflow-agent if available
    // 3. Find ctrl ip and mac from controller address
    //TODO:直接赋值
    let ip: IpAddr = "127.0.0.1".parse().unwrap();
    let mac = MacAddr::ZERO;
    Ok((ip,mac))
}