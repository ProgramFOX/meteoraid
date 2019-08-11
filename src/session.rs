use crate::areas::Area;
use crate::field::Field;
use crate::meteor::{Meteor, Shower};
use crate::timestamp::Timestamp;

pub enum Event {
    Clouds(u8),
    AreaCounting(usize, Area),
    BeginPeriod,
    BreakStart,
    BreakEnd,
    EndPeriod,
    Meteor(Meteor),
}

pub struct TimestampedEvent(Timestamp, Event);

pub struct Period {
    pub start_time: u32,
    pub end_time: u32,
    pub teff: f64,
    pub limiting_magnitude: f64,
    pub field: Field,
    pub cloud_factor: f64,
    pub showers: Vec<Shower>,
    pub meteors: Vec<Meteor>,
}

pub struct Session {
    pub periods: Vec<Period>,
}
