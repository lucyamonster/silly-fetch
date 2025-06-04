/*
All things time
uptime, local time, etc
*/

use crate::library::file::get_file;

pub struct Time {
    pub days: u64,
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
}
impl Time {
    fn new(days: u64, hours: u64, minutes: u64, seconds: u64) -> Self {
        Time {
            days,
            hours,
            minutes,
            seconds,
        }
    }
}

#[doc = "reads `/proc/uptime` and returns `(uptime, total_cpu_idle)` in seconds"]
fn get_uptime() -> (String, String) {
    // The `/proc/uptime` contains to values: uptime and the idle time off all cores together
    let uptime_file = get_file("/proc/uptime");
    let times = uptime_file.split(" ");
    let times = times.collect::<Vec<&str>>();
    let uptime = times[0].to_string();
    let total_cpu_idle = times[1].to_string();
    return (uptime, total_cpu_idle);
}

#[doc = "turn seconds to days, hours, minutes, seconds"]
fn format_time(input_seconds: u64) -> Time {
    let mut total_seconds = input_seconds;
    let days = total_seconds / 86400;
    total_seconds %= 86400;

    let hours = total_seconds / 3600;
    total_seconds %= 3600;

    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;

    Time::new(days, hours, minutes, seconds)
}

#[doc = "get uptime in hours, minutes, etc."]
pub fn get_uptime_human() -> Time {
    let (mut uptime, _idle) = get_uptime();

    uptime.truncate(uptime.len() - 3); // Remove the .??

    let uptime = uptime
        .parse::<u64>()
        .expect("couldn't convert uptime to int");
    return format_time(uptime);
}
