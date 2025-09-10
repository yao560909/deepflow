use std::env;
use std::net::IpAddr;
use log::warn;
use public::utils::net::{MacAddr};
use crate::error::{Error, Result};

const ENV_INTERFACE_NAME: &str = "CTRL_NETWORK_INTERFACE";
const K8S_POD_IP_FOR_DEEPFLOW: &str = "K8S_POD_IP_FOR_DEEPFLOW";
