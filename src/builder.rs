use crate::areas::get_limiting_magnitude_avg;
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

    pub fn register_event(&mut self, time_and_event: TimestampedEvent) -> Result<(), BuilderError> {
        let TimestampedEvent(timestamp, event) = time_and_event;

        if self.current.current_break.is_some() {
            match event {
                Event::BreakEnd => {}
                _ => Err(BuilderError::InBreak)?,
            }
        }

        match event {
            Event::NewPeriod => {
                let mut c = IncompletePeriod::new();
                std::mem::swap(&mut c, &mut self.current);
                self.periods.push(c.to_period()?);
            }
            Event::Meteor(meteor) => {
                self.current.meteors.push(meteor);
            }
            Event::Field(field) => match self.current.field {
                Some(_) => {
                    Err(BuilderError::AlreadyField)?;
                }
                None => {
                    self.current.field = Some(field);
                }
            },
            Event::AreasCounted(counts) => {
                let maybe_lm_avg = get_limiting_magnitude_avg(counts);
                match maybe_lm_avg {
                    Some(lm_avg) => {
                        self.current.limiting_magnitudes.push((lm_avg, timestamp));
                    }
                    None => Err(BuilderError::InvalidLm)?,
                }
            }
            Event::Clouds(clouds) => {
                self.current.clouds.push((clouds, timestamp));
            }
            Event::Meteor(meteor) => {
                self.current.meteors.push(meteor);
            }
            Event::BreakStart => {
                self.current.current_break = Some(timestamp);
            }
            Event::BreakEnd => match self.current.current_break {
                Some(br) => {
                    self.current.breaks.push((br, timestamp));
                    self.current.current_break = None;
                }
                None => {
                    Err(BuilderError::NoBreakToEnd)?;
                }
            },
        };
        Ok(())
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
    limiting_magnitudes: Vec<(f64, Timestamp)>,
    clouds: Vec<(u8, Timestamp)>,
    breaks: Vec<(Timestamp, Timestamp)>,
    current_break: Option<Timestamp>,
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
            limiting_magnitudes: vec![],
            clouds: vec![],
            breaks: vec![],
            current_break: None,
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
    AlreadyField,
    InvalidLm,
    InBreak,
    NoBreakToEnd,
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
                BuilderError::AlreadyField => "You already specified a field for this period.",
                BuilderError::InvalidLm => "Invalid data for calculating limiting magnitude.",
                BuilderError::InBreak => "You can't register events during a break.",
                BuilderError::NoBreakToEnd => "There is no ongoing break to end.",
                BuilderError::Unknown => "unexpected error",
            }
        )
    }
}

impl std::error::Error for BuilderError {}
