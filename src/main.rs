use std::io::stdout;

use crossterm::{cursor::MoveTo, execute, terminal::{Clear, ClearType}};

use crate::library::{config::config, modules::{
    distro::get_distro_info, host::{get_device_model, get_hostname, get_kernel, get_user}, time::get_uptime_human
}};

mod library;


fn main() {
    let config = config();
    let mut stdout = stdout();

    if config.clear {
        execute!(
            stdout,
            MoveTo(0, 0),
            Clear(ClearType::All)
        ).unwrap();
    }

    let distro_info = get_distro_info();
    for item in config.layout {
        match item.as_str() {
            "userHostname" => {
                println!("{}@{}", get_user(), get_hostname());
            }
            "divider" => {
                println!("--------");
            }
            "os" => {
                println!("OS: {}", distro_info.pretty_name);
            }
            "host" => {
                let device = get_device_model();
                if device != None {
                    println!("Host: {}", device.unwrap());
                }
            }
            "kernel" => {
                println!("Kernel: {}", get_kernel());
            }
            "uptime" => {
                print_uptime();
            }
            _ => {
                panic!("unknown item {}", item);
            }
        }
    }
}

fn print_uptime() {
    let uptime = get_uptime_human();
    print!("Uptime: ");
    if uptime.days != 0 {
        print!("{} Days, ", uptime.days)
    }
    if uptime.hours != 0 {
        print!("{} Hours, ", uptime.hours)
    }
    println!("{} Minutes", uptime.minutes);
}
