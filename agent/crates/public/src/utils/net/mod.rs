mod error;
mod linux;
#[cfg(any(target_os = "linux", target_os = "android"))]
mod arp;

//TODO:for macos
#[cfg(any(target_os = "macos"))]
mod arp;

use std::array::TryFromSliceError;
use std::fmt;
use std::str::FromStr;
use serde::Serialize;

pub use error::{Error, Result};

pub const MAC_ADDR_LEN: usize = 6;

#[derive(Debug, Default, Clone)]
pub struct LinkStats {
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_dropped: u64,
    pub tx_dropped: u64,
}

#[derive(Debug, Default, Clone)]
pub struct Link {
    pub if_index: u32,
    pub mac_addr: MacAddr,
    #[cfg(target_os = "windows")]
    pub adapter_id: String,
    #[cfg(target_os = "windows")]
    pub device_name: String,
    pub name: String,
    pub flags: LinkStats,
    #[cfg(any(target_os = "linux", target_os = "android"))]
    pub if_type: Option<String>,
    pub peer_index: Option<u32>,
    pub link_netnsid: Option<u32>,
    pub stats: LinkStats,
}

impl PartialEq for Link {
    fn eq(&self, other: &Self) -> bool {
        self.if_index.eq(&other.if_index)
    }
}

impl Eq for Link {}

impl PartialOrd for Link {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.if_index.partial_cmp(&other.if_index)
    }
}

impl Ord for Link {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.if_index.cmp(&other.if_index)
    }
}

#[derive(Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Copy, Hash)]
// slice is in bigendian
pub struct MacAddr([u8; 6]);

impl MacAddr {
    pub const ZERO: MacAddr = MacAddr([0, 0, 0, 0, 0, 0]);

    const BROADCAST: u64 = 0xffffffffffff;
    const MULTICAST: u64 = 0x010000000000;
    pub fn octets(&self) -> &[u8; 6] {
        &self.0
    }

    pub fn to_lower_32b(&self) -> u32 {
        u32::from_be_bytes(self.0[2..6].try_into().unwrap()) as u32
    }

    pub fn is_multicast<T: AsRef<[u8]>>(octets: T) -> bool {
        assert!(octets.as_ref().len() > MAC_ADDR_LEN);
        octets.as_ref()[0] & 0x1 == 1
    }

    pub fn is_unicast(mac: MacAddr) -> bool {
        let mac_num = u64::from(mac);
        mac_num != Self::BROADCAST && mac_num & Self::MULTICAST != Self::MULTICAST
    }

    pub fn get_suffix(&self) -> u16 {
        (self.0[4] as u16) << 8 | self.0[5] as u16
    }
}

impl fmt::Debug for MacAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

impl fmt::Display for MacAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

impl From<MacAddr> for u64 {
    fn from(mac: MacAddr) -> Self {
        ((u16::from_be_bytes(mac.0[0..2].try_into().unwrap()) as u64) << 32)
            | u32::from_be_bytes(mac.0[2..6].try_into().unwrap()) as u64
    }
}

impl From<[u8; 6]> for MacAddr {
    fn from(octets: [u8; 6]) -> Self {
        MacAddr(octets)
    }
}

impl TryFrom<&[u8]> for MacAddr {
    type Error = TryFromSliceError;
    fn try_from(octets: &[u8]) -> Result<Self, Self::Error> {
        <[u8; 6]>::try_from(octets).map(Self::from)
    }
}

impl TryFrom<u64> for MacAddr {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value & 0xFFFF_0000_0000_0000 != 0 {
            return Err(value);
        }
        Ok(MacAddr(value.to_be_bytes()[2..].try_into().unwrap()))
    }
}

impl FromStr for MacAddr {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut addr = [0u8; 6];
        for (idx, n_s) in s.split(":").enumerate() {
            if idx >= MAC_ADDR_LEN {
                return Err(Error::ParseMacFailed(s.to_string()));
            }
            match u8::from_str_radix(n_s, 16) {
                Ok(n) => addr[idx] = n,
                Err(_) => return Err(Error::ParseMacFailed(s.to_string())),
            }
        }
        Ok(MacAddr(addr))
    }
}