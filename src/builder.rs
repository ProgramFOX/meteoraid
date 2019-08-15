use crate::field::Field;
use crate::meteor::{Meteor, Shower};
use crate::session::*;
use crate::timestamp::Timestamp;

pub struct SessionBuilder {
    periods: Vec<Period>,
    current: IncompletePeriod,
}

impl SessionBuilder {
    pub fn new() -> SessionBuilder {
        SessionBuilder {
            periods: vec![],
            current: IncompletePeriod::new(),
        }
    }

    pub fn to_session(mut self) -> Result<Session, BuilderError> {
        self.periods.push(self.current.to_period()?);
        Ok(Session {
            periods: self.periods,
        })
    }

    pub fn register_event(&mut self, event: &Event) -> Result<(), BuilderError> {
        match event {
            Event::NewPeriod => {
                let mut c = IncompletePeriod::new();
                std::mem::swap(&mut c, &mut self.current);
                self.periods.push(c.to_period()?);
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

struct IncompletePeriod {
    start_time: Option<Timestamp>,
    end_time: Option<Timestamp>,
    limiting_magnitude: Option<f64>,
    field: Option<Field>,
    cloud_factor: Option<f64>,
    showers: Vec<Shower>,
    meteors: Vec<Meteor>,
}

impl IncompletePeriod {
    fn new() -> IncompletePeriod {
        IncompletePeriod {
            start_time: None,
            end_time: None,
            limiting_magnitude: None,
            field: None,
            cloud_factor: None,
            showers: vec![],
            meteors: vec![],
        }
    }

    fn to_period(self) -> Result<Period, BuilderError> {
        if let (
            Some(start_time),
            Some(end_time),
            Some(limiting_magnitude),
            Some(field),
            Some(cloud_factor),
        ) = (
            &self.start_time,
            &self.end_time,
            &self.limiting_magnitude,
            &self.field,
            &self.cloud_factor,
        ) {
            Ok(Period {
                start_time: *start_time,
                end_time: *end_time,
                teff: (*end_time - *start_time) as f64 / 60f64,
                limiting_magnitude: *limiting_magnitude,
                field: *field,
                cloud_factor: *cloud_factor,
                showers: self.showers,
                meteors: self.meteors,
            })
        } else {
            match (
                &self.start_time.is_none(),
                &self.end_time.is_none(),
                &self.limiting_magnitude.is_none(),
                &self.field.is_none(),
                &self.cloud_factor.is_none(),
            ) {
                (true, _, _, _, _) => Err(BuilderError::NoStartTime),
                (_, true, _, _, _) => Err(BuilderError::NoEndTime),
                (_, _, true, _, _) => Err(BuilderError::NoLm),
                (_, _, _, true, _) => Err(BuilderError::NoField),
                (_, _, _, _, true) => Err(BuilderError::NoF),
                _ => Err(BuilderError::Unknown),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BuilderError {
    NoStartTime,
    NoEndTime,
    NoLm,
    NoField,
    NoF,
    Unknown,
}

impl std::fmt::Display for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BuilderError::NoStartTime => "No start time given for this period.",
                BuilderError::NoEndTime => "No end time given for this period.",
                BuilderError::NoLm => {
                    "No areas for the calculation of limiting magnitude are counted."
                }
                BuilderError::NoField => "No field given for this period.",
                BuilderError::NoF => "No cloud information given for this period.",
                BuilderError::Unknown => "unexpected error",
            }
        )
    }
}

impl std::error::Error for BuilderError {}
