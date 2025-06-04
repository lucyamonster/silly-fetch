/*
get info about the host
kernel, distro, etc
*/

use crate::library::{environment::read_env_var, file::get_line};

#[doc = "reads `/proc/sys/kernel`'s `ostype` and `osrelease` files to get the kernel version and os (os is probably always going to be `Linux` but hey it's here"]
pub fn get_kernel() -> String {
    let os = get_line("/proc/sys/kernel/ostype");
    let version = get_line("/proc/sys/kernel/osrelease");
    let kernel = format!("{} {}", os, version);
    return kernel;
}

#[doc = "read the hostname from `/proc/sys/kernel/hostname`"]
pub fn get_hostname() -> String {
    let hostname = get_line("/proc/sys/kernel/hostname");
    return hostname;
}

#[doc = "get the user (wow)"]
pub fn get_user() -> String {
    read_env_var("USER")
}

#[doc = "get the device like `Surface Pro` or if there isn't one return `None`"]
pub fn get_device_model() -> Option<String> {
    let name = get_line("/sys/devices/virtual/dmi/id/product_name");

    return if name != "System Product Name".to_string() {
        Some(name)
    } else {
        None
    }
}