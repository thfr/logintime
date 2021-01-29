use std::string::String;
use std::thread;
use std::time::Duration;
use sysinfo::{ProcessExt, SystemExt};

const LOCKSCREEN_PROCESS_NAMES: &[&str; 6] = &[
    "i3lock",
    "xlock",
    "xsecurelock",
    "xscreensaver-command",
    "slock",
    "kscreenlocker",
];

fn main() {
    let mut system = sysinfo::System::new_all();

    loop {
        system.refresh_processes();

        let mut found_lock = false;
        for (_, proc_) in system.get_processes() {
            if LOCKSCREEN_PROCESS_NAMES
                .iter()
                .any(move |x: &&str| String::from(proc_.name()) == String::from(*x))
            {
                found_lock = true;
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
