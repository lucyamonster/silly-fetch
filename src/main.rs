use crate::library::modules::{
    distro::get_distro_info, host::{get_device_model, get_hostname, get_kernel, get_user}, time::get_uptime_human
};

mod library;

fn main() {
    let distro_info = get_distro_info();

    println!("{}@{}", get_user(), get_hostname());

    println!("--------");

    println!("OS: {}", distro_info.pretty_name);
    
    let device = get_device_model();
    if device != None {
        println!("Host: {}", device.unwrap());
    }
    println!("Kernel: {}", get_kernel());

    print_uptime();

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
