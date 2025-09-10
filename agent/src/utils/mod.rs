mod pid_file;
mod environment;

use std::thread;
use std::time::Duration;

pub fn clean_and_exit(code: i32) {
    thread::sleep(Duration::from_secs(1));

    #[cfg(any(target_os = "linux", target_os = "android"))]
    pid_file::close();

    std::process::exit(code);
}