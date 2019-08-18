use crate::areas::get_limiting_magnitude_avg;
use crate::factors;
use crate::field::Field;
use crate::meteor::{Meteor, Shower};
use crate::session::*;
use crate::timestamp;
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
            Event::PeriodStart => {
                self.current.start_time = Some(timestamp);
            }
            Event::PeriodEnd => {
                self.current.end_time = Some(timestamp);
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
    field: Option<Field>,
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
            field: None,
            showers: vec![],
            meteors: vec![],
            limiting_magnitudes: vec![],
            clouds: vec![],
            breaks: vec![],
            current_break: None,
        }
    }

    fn to_period(self) -> Result<Period, BuilderError> {
        if self.clouds.len() == 0 {
            return Err(BuilderError::NoF);
        }
        if self.limiting_magnitudes.len() == 0 {
            return Err(BuilderError::NoLm);
        }
        if self.current_break.is_some() {
            return Err(BuilderError::UnfinishedBreak);
        }

        if let (Some(start_time), Some(end_time), Some(field)) =
            (&self.start_time, &self.end_time, &self.field)
        {
            let teff_minutes =
                match timestamp::effective_time_minutes(*start_time, *end_time, &self.breaks) {
                    Some(teff) => teff,
                    None => {
                        return Err(BuilderError::InvalidBreaks);
                    }
                };

            let lms = match checkpoints_to_durations(
                &self.limiting_magnitudes,
                *end_time,
                &self.breaks,
            ) {
                Some(x) => x,
                None => {
                    return Err(BuilderError::InvalidBreaks);
                }
            };
            let clouds = match checkpoints_to_durations(&self.clouds, *end_time, &self.breaks) {
                Some(x) => x,
                None => {
                    return Err(BuilderError::InvalidBreaks);
                }
            };

            let lm_avg = factors::limiting_magnitude(&lms);
            let lm_teff: u32 = lms.iter().map(|x| x.1).sum();
            if lm_teff != teff_minutes {
                return Err(BuilderError::LmInsufficientTeff);
            }

            let cloud_factor = factors::cloud_factor(&clouds);
            let f_teff: u32 = clouds.iter().map(|x| x.1).sum();
            if f_teff != teff_minutes {
                return Err(BuilderError::FInsufficientTeff);
            }

            Ok(Period {
                start_time: *start_time,
                end_time: *end_time,
                teff: teff_minutes as f64 / 60f64,
                limiting_magnitude: lm_avg,
                field: *field,
                cloud_factor,
                showers: self.showers,
                meteors: self.meteors,
            })
        } else {
            match (
                &self.start_time.is_none(),
                &self.end_time.is_none(),
                &self.field.is_none(),
            ) {
                (true, _, _) => Err(BuilderError::NoStartTime),
                (_, true, _) => Err(BuilderError::NoEndTime),
                (_, _, true) => Err(BuilderError::NoField),
                _ => Err(BuilderError::Unknown),
            }
        }
    }
}

fn checkpoints_to_durations<T>(
    cs: &Vec<(T, Timestamp)>,
    end: Timestamp,
    breaks: &Vec<(Timestamp, Timestamp)>,
) -> Option<Vec<(T, u32)>>
where
    T: Default + std::ops::Sub + Copy,
{
    let mut result: Vec<(T, u32)> = vec![];

    for i in 0..cs.len() {
        let curr = cs[i];
        let next = match cs.get(i + 1) {
            Some(x) => *x,
            _ => (T::default(), end),
        };

        let teff = match timestamp::effective_time_minutes(curr.1, next.1, &breaks) {
            Some(x) => x,
            None => {
                return None;
            }
        };
        result.push((curr.0, teff));
    }
    Some(result)
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
    UnfinishedBreak,
    InvalidBreaks,
    LmInsufficientTeff,
    FInsufficientTeff,
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
                BuilderError::UnfinishedBreak => "You started a break that didn't end.",
                BuilderError::InvalidBreaks => "Your breaks do not adhere to the break rules.",
                BuilderError::LmInsufficientTeff => {
                    "Your recorded limiting magnitudes do not span your whole period."
                }
                BuilderError::FInsufficientTeff => {
                    "Your recorded cloud estimates do not span your whole period."
                }
                BuilderError::Unknown => "unexpected error",
            }
        )
    }
}

impl std::error::Error for BuilderError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctd_1() {
        let checkpoints: Vec<(f64, Timestamp)> = vec![
            (
                5.6,
                Timestamp {
                    hour: 23,
                    minute: 50,
                },
            ),
            (6.0, Timestamp { hour: 0, minute: 1 }),
            (
                5.3,
                Timestamp {
                    hour: 1,
                    minute: 15,
                },
            ),
        ];
        let end = Timestamp {
            hour: 1,
            minute: 30,
        };
        let breaks: Vec<(Timestamp, Timestamp)> = vec![
            (
                Timestamp {
                    hour: 23,
                    minute: 59,
                },
                Timestamp { hour: 0, minute: 0 },
            ),
            (
                Timestamp {
                    hour: 3,
                    minute: 15,
                },
                Timestamp {
                    hour: 4,
                    minute: 14,
                },
            ),
        ];

        let expected: Vec<(f64, u32)> = vec![(5.6, 10), (6.0, 74), (5.3, 15)];
        assert_eq!(
            checkpoints_to_durations(&checkpoints, end, &breaks),
            Some(expected)
        );
    }

    #[test]
    fn test_ctd_2() {
        let checkpoints: Vec<(f64, Timestamp)> = vec![(
            5.6,
            Timestamp {
                hour: 23,
                minute: 50,
            },
        )];
        let end = Timestamp {
            hour: 1,
            minute: 30,
        };

        let expected: Vec<(f64, u32)> = vec![(5.6, 100)];
        assert_eq!(
            checkpoints_to_durations(&checkpoints, end, &vec![]),
            Some(expected)
        );
    }

    #[test]
    fn test_ctd_3() {
        let checkpoints: Vec<(f64, Timestamp)> = vec![];
        let end = Timestamp {
            hour: 1,
            minute: 30,
        };

        let expected: Vec<(f64, u32)> = vec![];
        assert_eq!(
            checkpoints_to_durations(&checkpoints, end, &vec![]),
            Some(expected)
        );
    }

    #[test]
    fn test_ctd_4() {
        let checkpoints: Vec<(u8, Timestamp)> = vec![
            (
                56,
                Timestamp {
                    hour: 23,
                    minute: 50,
                },
            ),
            (60, Timestamp { hour: 0, minute: 1 }),
            (
                53,
                Timestamp {
                    hour: 1,
                    minute: 15,
                },
            ),
        ];
        let end = Timestamp {
            hour: 1,
            minute: 30,
        };
        let breaks: Vec<(Timestamp, Timestamp)> = vec![(
            Timestamp {
                hour: 23,
                minute: 59,
            },
            Timestamp { hour: 0, minute: 0 },
        )];

        let expected: Vec<(u8, u32)> = vec![(56, 10), (60, 74), (53, 15)];
        assert_eq!(
            checkpoints_to_durations(&checkpoints, end, &breaks),
            Some(expected)
        );
    }

    #[test]
    fn test_ctd_5() {
        let checkpoints: Vec<(u8, Timestamp)> = vec![(
            56,
            Timestamp {
                hour: 23,
                minute: 50,
            },
        )];
        let end = Timestamp {
            hour: 1,
            minute: 30,
        };

        let expected: Vec<(u8, u32)> = vec![(56, 100)];
        assert_eq!(
            checkpoints_to_durations(&checkpoints, end, &vec![]),
            Some(expected)
        );
    }

    #[test]
    fn test_ctd_6() {
        let checkpoints: Vec<(u8, Timestamp)> = vec![];
        let end = Timestamp {
            hour: 1,
            minute: 30,
        };

        let expected: Vec<(u8, u32)> = vec![];
        assert_eq!(
            checkpoints_to_durations(&checkpoints, end, &vec![]),
            Some(expected)
        );
    }

    #[test]
    fn test_ctd_7() {
        let checkpoints: Vec<(u8, Timestamp)> = vec![(
            56,
            Timestamp {
                hour: 1,
                minute: 15,
            },
        )];
        let end = Timestamp {
            hour: 1,
            minute: 30,
        };
        let breaks: Vec<(Timestamp, Timestamp)> = vec![(
            Timestamp {
                hour: 1,
                minute: 20,
            },
            Timestamp {
                hour: 1,
                minute: 35,
            },
        )];

        assert_eq!(checkpoints_to_durations(&checkpoints, end, &breaks), None);
    }
}
