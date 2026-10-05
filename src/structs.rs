use systemstat::{CPULoad, DelayedMeasurement};

pub struct Cpu {
    pub current: f32,
    pub next: DelayedMeasurement<CPULoad>
}

