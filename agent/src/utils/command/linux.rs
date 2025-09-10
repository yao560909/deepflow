use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

const ROOT_UTS_PATH: &'static str = "/proc/1/ns/uts";
const ORIGIN_UTS_PATH: &'static str = "/proc/self/ns/uts";

pub fn get_hostname() -> Result<String> {
    fn hostname() -> Result<String> {
        hostname::get()?
            .into_string()
            .map_err(|_| Error::new(ErrorKind::Other, "get hostname failed"))
    }
   hostname()
}