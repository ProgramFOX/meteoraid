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
    pub fn new() -> Self {
        Self {
            periods: vec![],
            current: IncompletePeriod::new(),
        }
    }

    pub fn into_session(mut self) -> Result<Session, BuilderError> {
        self.periods.push(self.current.into_period()?);
        Ok(Session {
            periods: self.periods,
        })
    }

    pub fn register_event(&mut self, time_and_event: TimestampedEvent) -> Result<(), BuilderError> {
        let TimestampedEvent(timestamp, event) = time_and_event;

        if self.current.current_break.is_some() {
            match event {
                Event::BreakEnd => {}
                _ => return Err(BuilderError::InBreak),
            }
        }

        match event {
            Event::NewPeriod => {
                let mut c = IncompletePeriod::new();
                std::mem::swap(&mut c, &mut self.current);
                self.periods.push(c.into_period()?);
            }
            Event::PeriodStart => {
                self.current.start_time = Some(timestamp);
            }
            Event::PeriodEnd => {
                self.current.end_time = Some(timestamp);
            }
            Event::PeriodDate(date) => {
                if self.current.date.is_some() {
                    return Err(BuilderError::AlreadyDate);
                }

                self.current.date = Some(date);
            }
            Event::Meteor(meteor) => {
                if !self
                    .current
                    .showers
                    .as_ref()
                    .unwrap_or(&vec![])
                    .contains(&meteor.shower)
                {
                    return Err(BuilderError::NotObservingShower);
                }
                self.current.meteors.push(meteor);
            }
            Event::Field(field) => {
                if self.current.field.is_some() {
                    return Err(BuilderError::AlreadyField);
                } else {
                    self.current.field = Some(field);
                }
            }
            Event::AreasCounted(counts) => {
                let maybe_lm_avg = get_limiting_magnitude_avg(&counts);
                if let Some(lm_avg) = maybe_lm_avg {
                    self.current.limiting_magnitudes.push((lm_avg, timestamp));
                } else {
                    return Err(BuilderError::InvalidLm);
                }
            }
            Event::Clouds(clouds) => {
                self.current.clouds.push((clouds, timestamp));
            }
            Event::BreakStart => {
                self.current.current_break = Some(timestamp);
            }
            Event::BreakEnd => match self.current.current_break {
                Some(br) => {
                    self.current.breaks.push((br, timestamp));
                    self.current.current_break = None;
                }
                None => return Err(BuilderError::NoBreakToEnd),
            },
            Event::Showers(showers) => {
                if self.current.showers.is_some() {
                    return Err(BuilderError::AlreadyShowers);
                }

                self.current.showers = Some(showers);
            }
        };
        Ok(())
    }
}

struct IncompletePeriod {
    start_time: Option<Timestamp>,
    end_time: Option<Timestamp>,
    date: Option<String>,
    field: Option<Field>,
    showers: Option<Vec<Shower>>,
    meteors: Vec<Meteor>,
    limiting_magnitudes: Vec<(f64, Timestamp)>,
    clouds: Vec<(u8, Timestamp)>,
    breaks: Vec<(Timestamp, Timestamp)>,
    current_break: Option<Timestamp>,
}

impl IncompletePeriod {
    fn new() -> Self {
        Self {
            start_time: None,
            end_time: None,
            date: None,
            field: None,
            showers: None,
            meteors: vec![],
            limiting_magnitudes: vec![],
            clouds: vec![],
            breaks: vec![],
            current_break: None,
        }
    }

    fn into_period(self) -> Result<Period, BuilderError> {
        if self.clouds.is_empty() {
            return Err(BuilderError::NoF);
        }
        if self.limiting_magnitudes.is_empty() {
            return Err(BuilderError::NoLm);
        }
        if self.current_break.is_some() {
            return Err(BuilderError::UnfinishedBreak);
        }

        if let (Some(start_time), Some(end_time), Some(field), Some(date)) =
            (&self.start_time, &self.end_time, &self.field, &self.date)
        {
            let teff_minutes = if let Some(x) =
                timestamp::effective_time_minutes(*start_time, *end_time, &self.breaks)
            {
                x
            } else {
                return Err(BuilderError::InvalidBreaks);
            };

            let lms = if let Some(x) =
                checkpoints_to_durations(&self.limiting_magnitudes, *end_time, &self.breaks)
            {
                x
            } else {
                return Err(BuilderError::InvalidBreaks);
            };
            let clouds =
                if let Some(x) = checkpoints_to_durations(&self.clouds, *end_time, &self.breaks) {
                    x
                } else {
                    return Err(BuilderError::InvalidBreaks);
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
                date: date.to_owned(),
                teff: f64::from(teff_minutes) / 60_f64,
                limiting_magnitude: lm_avg,
                field: *field,
                cloud_factor,
                showers: self.showers.unwrap_or_else(|| vec![]),
                meteors: self.meteors,
            })
        } else {
            match (
                &self.start_time.is_none(),
                &self.end_time.is_none(),
                &self.field.is_none(),
                &self.date.is_none(),
            ) {
                (true, _, _, _) => Err(BuilderError::NoStartTime),
                (_, true, _, _) => Err(BuilderError::NoEndTime),
                (_, _, true, _) => Err(BuilderError::NoField),
                (_, _, _, true) => Err(BuilderError::NoDate),
                _ => Err(BuilderError::Unknown),
            }
        }
    }
}

fn checkpoints_to_durations<T>(
    cs: &[(T, Timestamp)],
    end: Timestamp,
    breaks: &[(Timestamp, Timestamp)],
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

        let teff = if let Some(x) = timestamp::effective_time_minutes(curr.1, next.1, &breaks) {
            x
        } else {
            return None;
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
    NoDate,
    AlreadyDate,
    AlreadyField,
    AlreadyShowers,
    InvalidLm,
    InBreak,
    NoBreakToEnd,
    UnfinishedBreak,
    InvalidBreaks,
    LmInsufficientTeff,
    FInsufficientTeff,
    NotObservingShower,
    Unknown,
}

impl std::fmt::Display for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NoStartTime => "No start time given for this period.",
                Self::NoEndTime => "No end time given for this period.",
                Self::NoLm => "No areas for the calculation of limiting magnitude are counted.",
                Self::NoField => "No field given for this period.",
                Self::NoF => "No cloud information given for this period.",
                Self::NoDate => "No date specified for this period.",
                Self::AlreadyDate => "You already specified a date for this period.",
                Self::AlreadyField => "You already specified a field for this period.",
                Self::AlreadyShowers => "You already specified showers for this period.",
                Self::InvalidLm => "Invalid data for calculating limiting magnitude.",
                Self::InBreak => "You can't register events during a break.",
                Self::NoBreakToEnd => "There is no ongoing break to end.",
                Self::UnfinishedBreak => "You started a break that didn't end.",
                Self::InvalidBreaks => "Your breaks do not adhere to the break rules.",
                Self::LmInsufficientTeff => {
                    "Your recorded limiting magnitudes do not span your whole period."
                }
                Self::FInsufficientTeff => {
                    "Your recorded cloud estimates do not span your whole period."
                }
                Self::NotObservingShower => {
                    "Meteor belongs to a shower that you are not observing."
                }
                Self::Unknown => "unexpected error",
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

    use crate::areas::Area;

    fn round(a: f64) -> f64 {
        (a * 100_f64).round() / 100_f64
    }

    #[test]
    fn test_builder_1() {
        let mut builder = SessionBuilder::new();
        let start = Timestamp {
            hour: 22,
            minute: 55,
        };
        let end = Timestamp {
            hour: 1,
            minute: 20,
        };
        builder
            .register_event(TimestampedEvent(start, Event::PeriodStart))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::PeriodDate("12 Aug 2019".to_owned()),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Showers(vec![Shower::Perseids]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::AreasCounted(vec![(10, Area(14))]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(start, Event::Clouds(0)))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Field(Field {
                    ra: 290.0,
                    dec: 55.0,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 22,
                    minute: 57,
                },
                Event::Meteor(Meteor {
                    shower: Shower::Perseids,
                    magnitude: 35,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(end, Event::PeriodEnd))
            .unwrap();

        let session = builder.into_session().unwrap();
        assert_eq!(session.periods.len(), 1);
        let period = &session.periods[0];
        assert_eq!(period.start_time, start);
        assert_eq!(period.end_time, end);
        assert_eq!(period.meteors.len(), 1);
        assert_eq!(period.cloud_factor, 1.0);
        assert_eq!(period.limiting_magnitude, 5.58);
        assert_eq!(round(period.teff), 2.42);
    }

    #[test]
    fn test_builder_2() {
        let mut builder = SessionBuilder::new();
        let start = Timestamp {
            hour: 22,
            minute: 55,
        };
        let end = Timestamp {
            hour: 1,
            minute: 20,
        };
        builder
            .register_event(TimestampedEvent(start, Event::PeriodStart))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::PeriodDate("12 Aug 2019".to_owned()),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Showers(vec![Shower::Perseids]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::AreasCounted(vec![(10, Area(14))]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(start, Event::Clouds(0)))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Field(Field {
                    ra: 290.0,
                    dec: 55.0,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 22,
                    minute: 57,
                },
                Event::Meteor(Meteor {
                    shower: Shower::Perseids,
                    magnitude: 35,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(end, Event::PeriodEnd))
            .unwrap();
        builder
            .register_event(TimestampedEvent(end, Event::NewPeriod))
            .unwrap();

        let start2 = Timestamp {
            hour: 1,
            minute: 30,
        };
        let end2 = Timestamp { hour: 2, minute: 0 };
        builder
            .register_event(TimestampedEvent(start2, Event::PeriodStart))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start2,
                Event::PeriodDate("12 Aug 2019".to_owned()),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start2,
                Event::AreasCounted(vec![(11, Area(14))]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(start2, Event::Clouds(5)))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start2,
                Event::Field(Field {
                    ra: 336.0,
                    dec: 52.3,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(end2, Event::PeriodEnd))
            .unwrap();

        let session = builder.into_session().unwrap();
        assert_eq!(session.periods.len(), 2);
        let period = &session.periods[0];
        assert_eq!(period.start_time, start);
        assert_eq!(period.end_time, end);
        assert_eq!(period.meteors.len(), 1);
        assert_eq!(period.cloud_factor, 1.0);
        assert_eq!(period.limiting_magnitude, 5.58);
        assert_eq!(period.field.ra, 290.0);
        assert_eq!(round(period.teff), 2.42);

        let period2 = &session.periods[1];
        assert_eq!(period2.start_time, start2);
        assert_eq!(period2.end_time, end2);
        assert_eq!(period2.meteors.len(), 0);
        assert_eq!(period2.cloud_factor, 1.05);
        assert_eq!(period2.limiting_magnitude, 5.64);
        assert_eq!(period2.field.ra, 336.0);
        assert_eq!(period2.teff, 0.5);
    }

    #[test]
    fn test_builder_3() {
        let mut builder = SessionBuilder::new();
        let start = Timestamp {
            hour: 22,
            minute: 55,
        };
        let end = Timestamp {
            hour: 1,
            minute: 20,
        };
        builder
            .register_event(TimestampedEvent(start, Event::PeriodStart))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::PeriodDate("12 Aug 2019".to_owned()),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Showers(vec![Shower::Perseids]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(start, Event::Clouds(0)))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Field(Field {
                    ra: 290.0,
                    dec: 55.0,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 22,
                    minute: 57,
                },
                Event::Meteor(Meteor {
                    shower: Shower::Perseids,
                    magnitude: 35,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(end, Event::PeriodEnd))
            .unwrap();

        match builder.into_session() {
            Err(BuilderError::NoLm) => {}
            _ => panic!("into_session does not return NoLm"),
        };
    }

    #[test]
    fn test_builder_4() {
        let mut builder = SessionBuilder::new();
        let start = Timestamp {
            hour: 22,
            minute: 55,
        };
        let end = Timestamp {
            hour: 1,
            minute: 20,
        };
        builder
            .register_event(TimestampedEvent(start, Event::PeriodStart))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::PeriodDate("12 Aug 2019".to_owned()),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Showers(vec![Shower::Perseids]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::AreasCounted(vec![(10, Area(14))]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Field(Field {
                    ra: 290.0,
                    dec: 55.0,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 22,
                    minute: 57,
                },
                Event::Meteor(Meteor {
                    shower: Shower::Perseids,
                    magnitude: 35,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(end, Event::PeriodEnd))
            .unwrap();

        match builder.into_session() {
            Err(BuilderError::NoF) => {}
            _ => panic!("into_session does not return NoF"),
        };
    }

    #[test]
    fn test_builder_5() {
        let mut builder = SessionBuilder::new();
        let start = Timestamp {
            hour: 22,
            minute: 55,
        };
        let end = Timestamp {
            hour: 1,
            minute: 20,
        };
        builder
            .register_event(TimestampedEvent(
                start,
                Event::PeriodDate("12 Aug 2019".to_owned()),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Showers(vec![Shower::Perseids]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::AreasCounted(vec![(10, Area(14))]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(start, Event::Clouds(0)))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Field(Field {
                    ra: 290.0,
                    dec: 55.0,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 22,
                    minute: 57,
                },
                Event::Meteor(Meteor {
                    shower: Shower::Perseids,
                    magnitude: 35,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(end, Event::PeriodEnd))
            .unwrap();

        match builder.into_session() {
            Err(BuilderError::NoStartTime) => {}
            _ => panic!("into_session does not return NoStartTime"),
        };
    }

    #[test]
    fn test_builder_6() {
        let mut builder = SessionBuilder::new();
        let start = Timestamp {
            hour: 22,
            minute: 55,
        };
        builder
            .register_event(TimestampedEvent(start, Event::PeriodStart))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::PeriodDate("12 Aug 2019".to_owned()),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Showers(vec![Shower::Perseids]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::AreasCounted(vec![(10, Area(14))]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(start, Event::Clouds(0)))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Field(Field {
                    ra: 290.0,
                    dec: 55.0,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 22,
                    minute: 57,
                },
                Event::Meteor(Meteor {
                    shower: Shower::Perseids,
                    magnitude: 35,
                }),
            ))
            .unwrap();

        match builder.into_session() {
            Err(BuilderError::NoEndTime) => {}
            _ => panic!("into_session does not return NoEndTime"),
        };
    }

    #[test]
    fn test_builder_7() {
        let mut builder = SessionBuilder::new();
        let start = Timestamp {
            hour: 22,
            minute: 55,
        };
        let end = Timestamp {
            hour: 1,
            minute: 20,
        };
        builder
            .register_event(TimestampedEvent(start, Event::PeriodStart))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::PeriodDate("12 Aug 2019".to_owned()),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Showers(vec![Shower::Perseids]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::AreasCounted(vec![(10, Area(14))]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(start, Event::Clouds(0)))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 22,
                    minute: 57,
                },
                Event::Meteor(Meteor {
                    shower: Shower::Perseids,
                    magnitude: 35,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(end, Event::PeriodEnd))
            .unwrap();

        match builder.into_session() {
            Err(BuilderError::NoField) => {}
            _ => panic!("into_session does not return NoField"),
        };
    }

    #[test]
    fn test_builder_8() {
        let mut builder = SessionBuilder::new();
        let start = Timestamp {
            hour: 22,
            minute: 55,
        };
        let end = Timestamp {
            hour: 1,
            minute: 20,
        };
        builder
            .register_event(TimestampedEvent(start, Event::PeriodStart))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::PeriodDate("12 Aug 2019".to_owned()),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Showers(vec![Shower::Perseids]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::AreasCounted(vec![(10, Area(14))]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(start, Event::Clouds(0)))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Field(Field {
                    ra: 290.0,
                    dec: 55.0,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 22,
                    minute: 57,
                },
                Event::Meteor(Meteor {
                    shower: Shower::Perseids,
                    magnitude: 35,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp { hour: 0, minute: 1 },
                Event::BreakStart,
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 0,
                    minute: 31,
                },
                Event::BreakEnd,
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(end, Event::PeriodEnd))
            .unwrap();

        let session = builder.into_session().unwrap();
        assert_eq!(session.periods.len(), 1);
        let period = &session.periods[0];
        assert_eq!(period.start_time, start);
        assert_eq!(period.end_time, end);
        assert_eq!(period.meteors.len(), 1);
        assert_eq!(period.cloud_factor, 1.0);
        assert_eq!(period.limiting_magnitude, 5.58);
        assert_eq!(round(period.teff), 1.92);
    }

    #[test]
    fn test_builder_9() {
        let mut builder = SessionBuilder::new();
        let start = Timestamp {
            hour: 22,
            minute: 55,
        };
        builder
            .register_event(TimestampedEvent(start, Event::PeriodStart))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::PeriodDate("12 Aug 2019".to_owned()),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Showers(vec![Shower::Perseids]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::AreasCounted(vec![(10, Area(14))]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(start, Event::Clouds(0)))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Field(Field {
                    ra: 290.0,
                    dec: 55.0,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 22,
                    minute: 57,
                },
                Event::Meteor(Meteor {
                    shower: Shower::Perseids,
                    magnitude: 35,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp { hour: 0, minute: 1 },
                Event::BreakStart,
            ))
            .unwrap();
        match builder.register_event(TimestampedEvent(
            Timestamp { hour: 0, minute: 2 },
            Event::Clouds(5),
        )) {
            Err(BuilderError::InBreak) => {}
            _ => panic!("register_event didn't return InBreak"),
        };
    }

    #[test]
    fn test_builder_10() {
        let mut builder = SessionBuilder::new();
        let start = Timestamp {
            hour: 22,
            minute: 55,
        };
        let end = Timestamp {
            hour: 1,
            minute: 20,
        };
        builder
            .register_event(TimestampedEvent(start, Event::PeriodStart))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::PeriodDate("12 Aug 2019".to_owned()),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Showers(vec![Shower::Perseids]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::AreasCounted(vec![(10, Area(14)), (10, Area(7)), (8, Area(6))]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(start, Event::Clouds(0)))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Field(Field {
                    ra: 290.0,
                    dec: 55.0,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 22,
                    minute: 57,
                },
                Event::Meteor(Meteor {
                    shower: Shower::Perseids,
                    magnitude: 35,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp { hour: 0, minute: 0 },
                Event::AreasCounted(vec![(12, Area(14)), (12, Area(7)), (10, Area(6))]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp { hour: 0, minute: 0 },
                Event::Clouds(15),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(end, Event::PeriodEnd))
            .unwrap();

        let session = builder.into_session().unwrap();
        assert_eq!(session.periods.len(), 1);
        let period = &session.periods[0];
        assert_eq!(period.start_time, start);
        assert_eq!(period.end_time, end);
        assert_eq!(period.meteors.len(), 1);
        assert_eq!(period.cloud_factor, 1.09);
        assert_eq!(period.limiting_magnitude, 5.76);
        assert_eq!(round(period.teff), 2.42);
    }

    #[test]
    fn test_builder_11() {
        let mut builder = SessionBuilder::new();
        let start = Timestamp {
            hour: 22,
            minute: 55,
        };
        let end = Timestamp {
            hour: 1,
            minute: 20,
        };
        builder
            .register_event(TimestampedEvent(start, Event::PeriodStart))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::PeriodDate("12 Aug 2019".to_owned()),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Showers(vec![Shower::Perseids]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::AreasCounted(vec![(10, Area(14)), (10, Area(7)), (8, Area(6))]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(start, Event::Clouds(0)))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Field(Field {
                    ra: 290.0,
                    dec: 55.0,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 22,
                    minute: 57,
                },
                Event::Meteor(Meteor {
                    shower: Shower::Perseids,
                    magnitude: 35,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 23,
                    minute: 15,
                },
                Event::BreakStart,
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 23,
                    minute: 55,
                },
                Event::BreakEnd,
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp { hour: 0, minute: 0 },
                Event::AreasCounted(vec![(12, Area(14)), (12, Area(7)), (10, Area(6))]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp { hour: 0, minute: 0 },
                Event::Clouds(15),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(end, Event::PeriodEnd))
            .unwrap();

        let session = builder.into_session().unwrap();
        assert_eq!(session.periods.len(), 1);
        let period = &session.periods[0];
        assert_eq!(period.start_time, start);
        assert_eq!(period.end_time, end);
        assert_eq!(period.meteors.len(), 1);
        assert_eq!(period.cloud_factor, 1.13);
        assert_eq!(period.limiting_magnitude, 5.83);
        assert_eq!(period.teff, 1.75);
    }

    #[test]
    fn test_builder_12() {
        let mut builder = SessionBuilder::new();
        let start = Timestamp {
            hour: 22,
            minute: 55,
        };
        builder
            .register_event(TimestampedEvent(start, Event::PeriodStart))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::PeriodDate("12 Aug 2019".to_owned()),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Showers(vec![Shower::Perseids]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::AreasCounted(vec![(10, Area(14))]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(start, Event::Clouds(0)))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Field(Field {
                    ra: 290.0,
                    dec: 55.0,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 22,
                    minute: 57,
                },
                Event::Meteor(Meteor {
                    shower: Shower::Perseids,
                    magnitude: 35,
                }),
            ))
            .unwrap();
        match builder.register_event(TimestampedEvent(
            Timestamp {
                hour: 22,
                minute: 57,
            },
            Event::Field(Field { ra: 0.0, dec: 0.0 }),
        )) {
            Err(BuilderError::AlreadyField) => {}
            _ => panic!("register_event does not return AlreadyField"),
        };
    }

    #[test]
    fn test_builder_13() {
        let mut builder = SessionBuilder::new();
        let start = Timestamp {
            hour: 22,
            minute: 55,
        };
        builder
            .register_event(TimestampedEvent(start, Event::PeriodStart))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::PeriodDate("12 Aug 2019".to_owned()),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Showers(vec![Shower::Perseids]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::AreasCounted(vec![(10, Area(14))]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(start, Event::Clouds(0)))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Field(Field {
                    ra: 290.0,
                    dec: 55.0,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 22,
                    minute: 57,
                },
                Event::Meteor(Meteor {
                    shower: Shower::Perseids,
                    magnitude: 35,
                }),
            ))
            .unwrap();
        match builder.register_event(TimestampedEvent(
            Timestamp {
                hour: 22,
                minute: 57,
            },
            Event::Showers(vec![Shower::Leonids]),
        )) {
            Err(BuilderError::AlreadyShowers) => {}
            _ => panic!("register_event does not return AlreadyShowers"),
        };
    }

    #[test]
    fn test_builder_14() {
        let mut builder = SessionBuilder::new();
        let start = Timestamp {
            hour: 22,
            minute: 55,
        };
        builder
            .register_event(TimestampedEvent(start, Event::PeriodStart))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::PeriodDate("12 Aug 2019".to_owned()),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Showers(vec![Shower::Perseids]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::AreasCounted(vec![(10, Area(14))]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(start, Event::Clouds(0)))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Field(Field {
                    ra: 290.0,
                    dec: 55.0,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 22,
                    minute: 57,
                },
                Event::Meteor(Meteor {
                    shower: Shower::Perseids,
                    magnitude: 35,
                }),
            ))
            .unwrap();
        match builder.register_event(TimestampedEvent(
            Timestamp {
                hour: 22,
                minute: 57,
            },
            Event::Meteor(Meteor {
                shower: Shower::KappaCygnids,
                magnitude: 20,
            }),
        )) {
            Err(BuilderError::NotObservingShower) => {}
            _ => panic!("register_event does not return NotObservingShower"),
        };
    }

    #[test]
    fn test_builder_15() {
        let mut builder = SessionBuilder::new();
        let start = Timestamp {
            hour: 22,
            minute: 55,
        };
        let end = Timestamp {
            hour: 1,
            minute: 20,
        };
        builder
            .register_event(TimestampedEvent(start, Event::PeriodStart))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::PeriodDate("12 Aug 2019".to_owned()),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Showers(vec![Shower::Perseids]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(start, Event::Clouds(0)))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Field(Field {
                    ra: 290.0,
                    dec: 55.0,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 22,
                    minute: 57,
                },
                Event::Meteor(Meteor {
                    shower: Shower::Perseids,
                    magnitude: 35,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 22,
                    minute: 57,
                },
                Event::AreasCounted(vec![(10, Area(14))]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(end, Event::PeriodEnd))
            .unwrap();

        match builder.into_session() {
            Err(BuilderError::LmInsufficientTeff) => {}
            _ => panic!("into_session does not return LmInsufficientTeff"),
        }
    }

    #[test]
    fn test_builder_16() {
        let mut builder = SessionBuilder::new();
        let start = Timestamp {
            hour: 22,
            minute: 55,
        };
        let end = Timestamp {
            hour: 1,
            minute: 20,
        };
        builder
            .register_event(TimestampedEvent(start, Event::PeriodStart))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::PeriodDate("12 Aug 2019".to_owned()),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::AreasCounted(vec![(10, Area(14))]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Showers(vec![Shower::Perseids]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Field(Field {
                    ra: 290.0,
                    dec: 55.0,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 22,
                    minute: 57,
                },
                Event::Meteor(Meteor {
                    shower: Shower::Perseids,
                    magnitude: 35,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 22,
                    minute: 57,
                },
                Event::Clouds(0),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(end, Event::PeriodEnd))
            .unwrap();

        match builder.into_session() {
            Err(BuilderError::FInsufficientTeff) => {}
            _ => panic!("into_session does not return FInsufficientTeff"),
        }
    }

    #[test]
    fn test_builder_17() {
        let mut builder = SessionBuilder::new();
        let start = Timestamp {
            hour: 22,
            minute: 55,
        };
        let end = Timestamp {
            hour: 1,
            minute: 20,
        };
        builder
            .register_event(TimestampedEvent(start, Event::PeriodStart))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Showers(vec![Shower::Perseids]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::AreasCounted(vec![(10, Area(14))]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(start, Event::Clouds(0)))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Field(Field {
                    ra: 290.0,
                    dec: 55.0,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                Timestamp {
                    hour: 22,
                    minute: 57,
                },
                Event::Meteor(Meteor {
                    shower: Shower::Perseids,
                    magnitude: 35,
                }),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(end, Event::PeriodEnd))
            .unwrap();

        match builder.into_session() {
            Err(BuilderError::NoDate) => {}
            _ => panic!("into_session does not return NoDate"),
        };
    }

    #[test]
    fn test_builder_18() {
        let mut builder = SessionBuilder::new();
        let start = Timestamp {
            hour: 22,
            minute: 55,
        };
        builder
            .register_event(TimestampedEvent(start, Event::PeriodStart))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::Showers(vec![Shower::Perseids]),
            ))
            .unwrap();
        builder
            .register_event(TimestampedEvent(
                start,
                Event::PeriodDate("12 Aug 2019".to_owned()),
            ))
            .unwrap();
        match builder.register_event(TimestampedEvent(
            start,
            Event::PeriodDate("13 Aug 2019".to_owned()),
        )) {
            Err(BuilderError::AlreadyDate) => {}
            _ => panic!("register_event does not return AlreadyDate"),
        };
    }
}
