use chrono::Utc;
use systemstat::{Platform, System, saturating_sub_bytes};
use systemstat::{DelayedMeasurement, CPULoad};
use std::process::Command;
use crate::{color, structs::*};
use crate::error_handling::*;

pub async fn cpu_usage(system: &System, load: Option<DelayedMeasurement<CPULoad>>) -> WidgetDisplay {
    let mut current_usage = 0.0;
    if let Some(current_cpu) = load {
        let current_cpu = current_cpu.done().unwrap(); //  TODO: Maybe this needs error handling?
        current_usage = (100.0 - current_cpu.idle * 1000.0).round() / 10.0;
    }

    match system.cpu_load_aggregate() {
        Ok(next_cpu) => {
            WidgetDisplay {
                current: format!("CPU: {current_usage}"),
                next: Some(next_cpu)
            }
        }
        Err(_) => {throw_error("failed to measure CPU load...\n", 6); unreachable!()}
    }
}

pub async fn cpu_temp(system: &System) -> WidgetDisplay {
    match system.cpu_temp() {
        Ok(temp) => WidgetDisplay { current: format!("CPU temp: {}", (temp * 10.0).round() / 10.0), next: None},
        Err(_) => {
            throw_error("Could not get CPU temperature...", 19); unreachable!()
        }
    }
}

pub async fn mem_usage(system: &System) -> WidgetDisplay {
    match system.memory() {
        Ok(mem) => {
            WidgetDisplay {
                current: format!("RAM: {} / {}", saturating_sub_bytes(mem.total, mem.free), mem.total),
                next: None
            }
        },
        Err(x) => {
            throw_error(format!("memory error\n{}", x).as_str(), 7); unreachable!()
        }
    }
}

pub async fn uptime(system: &System) -> WidgetDisplay {
    match system.uptime() {
        Ok(uptime) => {
            WidgetDisplay {
                current: format!("Up for {} seconds", uptime.as_secs().to_string().parse::<f32>().expect("eh?") / 3600.0),
                next: None
            }
        },
        Err(_) => {throw_error("could not get uptime...", 8); unreachable!()}
    }
}

// Sorry to laptop users if this doesn't work...
pub async fn battery(system: &System) -> WidgetDisplay {
    match system.battery_life() {
        Ok(battery) => {
            WidgetDisplay {
                current: format!("Battery: {}", battery.remaining_capacity * 100.0),
                next: None
            }
        }
        Err(_) => {
            throw_error("Could not get battery life. You do have a battery right?", 0); unreachable!()
        }
    }
}

pub async fn time(display: bool, merican: bool) -> String {
    if display {
        let time = Utc::now();
        if merican {
            time.format("%M-%D-%Y  %H:%M:%S").to_string()
        } else {
            time.format("%D-%M-%Y  %H:%M:%S").to_string()
        }
    } else {
        "".to_string()
    }
}

// NOTE: I made this function with AI... I did check for it to not be trash tho
// It's also Linux-only because I don't want to wrap my head around making this multiplatform
// juuuuust yet... Maybe never actually...
// NOTE: oh also I'd like to change the dependency to wpctl, but atm idc tbh
pub async fn sound_level() -> WidgetDisplay {
    let raw_alsa = Command::new("amixer")
        .args(["get", "Master"]).output();

    if let Ok(raw_alsa) = raw_alsa {
        let alsa_output = unsafe { String::from_utf8_unchecked(raw_alsa.stdout) }; // The unsafe method was actually proposed by Neovim, not AI
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

        WidgetDisplay {
            current: format!("Volume: {volume}"),
            next: None
        }
    } else {
        throw_error("We weren't able to get your volume level.\nIf you're on Linux, try installing alsa-utils.\nIf you're on Windows, we sadly can't provide sound level...", 0); unreachable!()
    }
}

/*  TODO: gpu usage */ /*
pub async fn gpu_usage() -> String {

} */

pub async fn get(system: &System, widget: &Widget, last_next: Option<DelayedMeasurement<CPULoad>>) -> WidgetDisplay {
    let mut widget_display = match widget.widget_type.as_str() {
        "no" => WidgetDisplay { current: "".to_string(), next: None },
        "cu" => cpu_usage(system, last_next).await,
        "ct" => cpu_temp(system).await,
        "gu" => todo!("sorry, it's not yet implemented..."),
        "mu" => mem_usage(system).await,
        "up" => uptime(system).await,
        &_ => { throw_error(format!("Unexpected widget in your config file... Make sure it's well written or make it again with {}", color::get_painted("green", "easy_resources delete <set> && easy_resources make <set>").await).as_str(), 25); unreachable!()}
    };
    widget_display.current = color::get_painted(widget.color.as_str(), widget_display.current.as_str()).await;
    widget_display
}
