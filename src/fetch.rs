use chrono::Utc;
use systemstat::{Platform, System, saturating_sub_bytes};
use std::process::Command;
use crate::structs::Cpu;
use crate::error_handling::*;

pub async fn cpu_usage(system: &System, load: Option<Cpu>) -> Cpu {
    let mut current_usage = 0.0;
    if let Some(current_cpu) = load {
        let current_cpu = current_cpu.next.done().unwrap(); //  TODO: Maybe this needs error handling?
        current_usage = (100.0 - current_cpu.idle * 1000.0).round() / 10.0;
    }

    match system.cpu_load_aggregate() {
        Ok(next_cpu) => {
            Cpu {
                current: current_usage,
                next: next_cpu
            }
        }
        Err(_) => {throw_error("failed to measure CPU load...\n", 6).await; unreachable!()}
    }
}

pub async fn mem_usage(system: &System) -> String {
    match system.memory() {
        Ok(mem) => {
            format!("{} / {}", saturating_sub_bytes(mem.total, mem.free), mem.total)
        },
        Err(x) => {
            throw_error(format!("memory error\n{}", x).as_str(), 8).await; unreachable!()
        }
    }
}

pub async fn uptime(system: &System) -> String {
    match system.uptime() {
        Ok(uptime) => {format!("Awake for {} seconds", uptime.as_secs().to_string().parse::<f32>().expect("eh?") / 3600.0)},
        Err(_) => {throw_error("could not get uptime...", 8).await; unreachable!()}
    }
}

pub async fn battery(system: &System) -> String {
    match system.battery_life() {
        Ok(battery) => {
            format!("Battery: {}", battery.remaining_capacity * 100.0)
        }
        Err(_) => {
            throw_error("Could not get battery life. You do have a battery right?", 0); unreachable!()
        }
    }
}

pub async fn time(merican: bool) -> String {
    let time = Utc::now();
    if merican {
        time.format("%M-%D-%Y  %H:%M:%S").to_string()
    } else {
        time.format("%D-%M-%Y  %H:%M:%S").to_string()
    }
}

// NOTE: I made this function with AI... I did check for it to not be trash tho
// It's also Linux-only because I don't want to wrap my head around making this multiplatform
// juuuuust yet... Maybe never actually...
pub async fn sound_level() -> String {
    let raw_alsa = Command::new("amixer")
        .args(["get", "Master"]).output();

    if let Ok(raw_alsa) = raw_alsa {
    let alsa_output = unsafe { String::from_utf8_unchecked(raw_alsa.stdout) };
    let alsa_line = alsa_output
        .lines()
        .find(|line| line.contains('%'))
        .ok_or_else(|| panic!("something went wrong getting the volume..."))
        .unwrap();

    let volume = alsa_line
        .split('[')
        .nth(1)
        .ok_or_else(|| panic!("something went wrong with getting the volume..."))
        .unwrap()
        .split('%')
        .next()
        .ok_or_else(|| panic!("something went wrong with getting the volume..."))
        .unwrap();
    volume.to_string()
    } else {
        throw_error("We weren't able to get your volume level.\nIf you're on Linux, try installing alsa-utils.\nIf you're on Windows, we sadly can't provide sound level...", 0).await; unreachable!()
    }
}

//pub async fn 
