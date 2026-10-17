use systemstat::{CPULoad, DelayedMeasurement};

pub struct WidgetDisplay {
    pub current: String,
    pub next: Option<DelayedMeasurement<CPULoad>>
} // This is because I'm not going to use different types for displaying CPU from any other widget; it just feels... Not good...
  // If someone wants to make it well and not use this monstruosity, go ahead =D

pub struct Date {
    pub display: bool,
    pub american: bool,
    pub color: String
}

pub struct Widget {
    pub widget_type: String,
    pub color: String
}
