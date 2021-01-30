use std::thread;
use std::time::Duration;
use sysinfo::{ProcessExt, ProcessStatus, SystemExt};

#[allow(dead_code)]
const LOCKSCREEN_PROCESS_NAMES_LINUX: &[&str] = &[
    "i3lock",
    "xlock",
    "xsecurelock",
    "xscreensaver-command",
    "slock",
    "kscreenlocker",
];

#[allow(dead_code)]
const LOCKSCREEN_PROCESS_NAMES_WINDOWS: &[&str] = &["LockApp.exe"];

#[cfg(target_os = "windows")]
const LOCKSCREEN_PROCESS_NAMES: &[&str] = LOCKSCREEN_PROCESS_NAMES_WINDOWS;

#[cfg(target_os = "linux")]
const LOCKSCREEN_PROCESS_NAMES: &[&str] = LOCKSCREEN_PROCESS_NAMES_LINUX;

fn main() {
    let mut system = sysinfo::System::new_all();

    loop {
        system.refresh_processes();

        let mut found_lock = false;
        for (_, proc_) in system.get_processes() {
            let proc_name = proc_.name();
            let proc_status = proc_.status();
            let is_lockscreen_process = |x: &&str| {
                proc_name == *x
                    && match proc_status {
                        ProcessStatus::Sleep | ProcessStatus::Run | ProcessStatus::Idle => true,
                        _ => false,
                    }
            };
            if LOCKSCREEN_PROCESS_NAMES.iter().any(is_lockscreen_process) {
                found_lock = true;
                println!("{:?} is {:?}", proc_name, proc_.status());
                break;
            }
        }

        if found_lock {
            println!("Lockscreen running");
        } else {
            println!("Lockscreen not running");
        }

        thread::sleep(Duration::from_secs(5));
    }
}
