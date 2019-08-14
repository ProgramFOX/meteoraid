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

    pub fn register_event(&mut self, event: &Event) {
    }
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

    fn to_period(self) -> Result<Period, ()> {
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
            Err(())
        }
    }
}
