use crate::field::Field;
use crate::meteor::{Meteor, Shower};
use crate::session::*;

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

    pub fn to_session(mut self) -> Result<Session, ()> {
        self.periods.push(self.current.to_period()?);
        Ok(Session {
            periods: self.periods,
        })
    }

    pub fn register_event(&mut self, event: &Event) {}
}

struct IncompletePeriod {
    start_time: Option<u32>,
    end_time: Option<u32>,
    teff: Option<f64>,
    limiting_magnitude: Option<f64>,
    field: Option<Field>,
    cloud_factor: Option<f64>,
    showers: Option<Vec<Shower>>,
    meteors: Vec<Meteor>,
}

impl IncompletePeriod {
    fn new() -> IncompletePeriod {
        IncompletePeriod {
            start_time: None,
            end_time: None,
            teff: None,
            limiting_magnitude: None,
            field: None,
            cloud_factor: None,
            showers: None,
            meteors: vec![],
        }
    }

    fn to_period(self) -> Result<Period, BuilderError> {
        if let (
            Some(start_time),
            Some(end_time),
            Some(teff),
            Some(limiting_magnitude),
            Some(field),
            Some(cloud_factor),
            Some(showers),
        ) = (
            self.start_time,
            self.end_time,
            self.teff,
            self.limiting_magnitude,
            self.field,
            self.cloud_factor,
            self.showers,
        ) {
            Ok(Period {
                start_time,
                end_time,
                teff,
                limiting_magnitude,
                field,
                cloud_factor,
                showers,
                meteors: self.meteors,
            })
        } else {
            match (
                self.start_time.is_none(),
                self.end_time.is_none(),
                self.limiting_magnitude.is_none(),
                self.field.is_none(),
                self.cloud_factor.is_none(),
                self.showers.is_none(),
            ) {
                (true, _, _, _, _, _) => Err(BuilderError::NoStartTime),
                (_, true, _, _, _, _) => Err(BuilderError::NoEndTime),
                (_, _, true, _, _, _) => Err(BuilderError::NoLm),
                (_, _, _, true, _, _) => Err(BuilderError::NoField),
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
}

impl std::fmt::Display for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            match self {
                BuilderError::NoStartTime => "No start time given for this period.",
                BuilderError::NoEndTime => "No end time given for this period.",
                BuilderError::NoLm => {
                    "No areas for the calculation of limiting magnitude are counted."
                }
                BuilderError::NoField => "No field given for this period.",
                BuilderError::NoF => "No cloud information given for this period.",
            }
        )
    }
}
