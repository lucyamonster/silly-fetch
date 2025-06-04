use crate::library::{file::get_file, text::{just_quotes, just_value}};

pub struct DistroInfo {
    pub pretty_name: String,
    pub build_id: String,
}

pub fn get_distro_info() -> DistroInfo {
    let os_release = get_file("/etc/os-release");
    let reader = os_release.split("\n");

    let mut pretty_name = String::new();
    let mut build_id = String::new();
    
    for line in reader {
        if line.starts_with("PRETTY_NAME=") {
            pretty_name = just_quotes(line.to_string());
        } else if line.starts_with("BUILD_ID=") {
            build_id = just_value(line.to_string());
        }
    }

    DistroInfo {
        pretty_name,
        build_id,
    }
}