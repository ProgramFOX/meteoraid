use crate::areas::Area;
use crate::distribution::Distribution;
use crate::field::Field;
use crate::meteor::{Meteor, Shower};
use crate::timestamp::Timestamp;
use std::collections::HashMap;
use std::option::NoneError;

#[derive(Clone, PartialEq, Debug)]
pub enum Event {
    Clouds(u8),
    AreasCounted(Vec<(usize, Area)>),
    BreakStart,
    BreakEnd,
    NewPeriod,
    Meteor(Meteor),
    Field(Field),
    PeriodStart,
    PeriodEnd,
    PeriodDate(String),
    Showers(Vec<Shower>),
}

pub struct TimestampedEvent(pub Timestamp, pub Event);

pub struct Period {
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub date: String,
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

impl Period {
    pub fn get_count_and_distribution(&self) -> HashMap<Shower, (u32, Distribution)> {
        let mut map = HashMap::new();
        for shower in &self.showers {
            map.insert(*shower, (0, Distribution::new()));
        }

        for meteor in &self.meteors {
            let mut cd = map.get_mut(&meteor.shower).unwrap();
            cd.0 += 1;
            cd.1.add_meteor(&meteor);
        }
        map
    }

    pub fn get_distribution_csv(&self) -> Result<String, NoneError> {
        let mut lines: Vec<String> = vec![];
        let count_and_dist = self.get_count_and_distribution();
        for shower in &self.showers {
            let shower_dist = count_and_dist.get(&shower)?.1.to_map();
            lines.push(format!(
                "{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{}",
                self.date,
                self.start_time.to_shorthand_int_notation(),
                self.end_time.to_shorthand_int_notation(),
                shower.to_imo_code(),
                *shower_dist.get(&-6)? as f64 / 10.0,
                *shower_dist.get(&-5)? as f64 / 10.0,
                *shower_dist.get(&-4)? as f64 / 10.0,
                *shower_dist.get(&-3)? as f64 / 10.0,
                *shower_dist.get(&-2)? as f64 / 10.0,
                *shower_dist.get(&-1)? as f64 / 10.0,
                *shower_dist.get(&0)? as f64 / 10.0,
                *shower_dist.get(&1)? as f64 / 10.0,
                *shower_dist.get(&2)? as f64 / 10.0,
                *shower_dist.get(&3)? as f64 / 10.0,
                *shower_dist.get(&4)? as f64 / 10.0,
                *shower_dist.get(&5)? as f64 / 10.0,
                *shower_dist.get(&6)? as f64 / 10.0,
                *shower_dist.get(&7)? as f64 / 10.0,
            ));
        }
        Ok(lines.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_count_and_distribution_1() {
        let period = Period {
            start_time: Timestamp { hour: 0, minute: 0 },
            end_time: Timestamp { hour: 0, minute: 0 },
            date: "12 Aug 2019".to_owned(),
            teff: 0.0,
            limiting_magnitude: 0.0,
            field: Field { ra: 0.0, dec: 0.0 },
            cloud_factor: 1.0,
            showers: vec![Shower::Perseids, Shower::Sporadic],
            meteors: vec![
                Meteor {
                    shower: Shower::Perseids,
                    magnitude: 30,
                },
                Meteor {
                    shower: Shower::Perseids,
                    magnitude: 20,
                },
                Meteor {
                    shower: Shower::Perseids,
                    magnitude: -5,
                },
                Meteor {
                    shower: Shower::Sporadic,
                    magnitude: 40,
                },
                Meteor {
                    shower: Shower::Sporadic,
                    magnitude: -25,
                },
                Meteor {
                    shower: Shower::Perseids,
                    magnitude: 30,
                },
                Meteor {
                    shower: Shower::Perseids,
                    magnitude: 30,
                },
                Meteor {
                    shower: Shower::Sporadic,
                    magnitude: 50,
                },
                Meteor {
                    shower: Shower::Perseids,
                    magnitude: 30,
                },
                Meteor {
                    shower: Shower::Perseids,
                    magnitude: 30,
                },
            ],
        };
        let cd = period.get_count_and_distribution();
        let perseid_info = cd.get(&Shower::Perseids).unwrap();
        assert_eq!(perseid_info.0, 7);
        let perseid_distr_map = perseid_info.1.to_map();
        assert_eq!(*perseid_distr_map.get(&3).unwrap(), 50);
        assert_eq!(*perseid_distr_map.get(&2).unwrap(), 10);
        assert_eq!(*perseid_distr_map.get(&0).unwrap(), 5);
        assert_eq!(*perseid_distr_map.get(&-1).unwrap(), 5);
        assert_eq!(*perseid_distr_map.get(&-6).unwrap(), 0);

        let sporadic_info = cd.get(&Shower::Sporadic).unwrap();
        assert_eq!(sporadic_info.0, 3);
        let sporadic_distr_map = sporadic_info.1.to_map();
        assert_eq!(*sporadic_distr_map.get(&-3).unwrap(), 5);
        assert_eq!(*sporadic_distr_map.get(&-2).unwrap(), 5);
        assert_eq!(*sporadic_distr_map.get(&4).unwrap(), 10);
        assert_eq!(*sporadic_distr_map.get(&5).unwrap(), 10);
        assert_eq!(*sporadic_distr_map.get(&1).unwrap(), 0);
    }

    #[test]
    fn test_period_distribution_csv_1() {
        let period = Period {
            start_time: Timestamp { hour: 0, minute: 0 },
            end_time: Timestamp {
                hour: 0,
                minute: 30,
            },
            date: "12 Aug 2019".to_owned(),
            teff: 0.0,
            limiting_magnitude: 0.0,
            field: Field { ra: 0.0, dec: 0.0 },
            cloud_factor: 1.0,
            showers: vec![Shower::Perseids, Shower::Sporadic],
            meteors: vec![
                Meteor {
                    shower: Shower::Perseids,
                    magnitude: 30,
                },
                Meteor {
                    shower: Shower::Perseids,
                    magnitude: 20,
                },
                Meteor {
                    shower: Shower::Perseids,
                    magnitude: -5,
                },
                Meteor {
                    shower: Shower::Sporadic,
                    magnitude: 40,
                },
                Meteor {
                    shower: Shower::Sporadic,
                    magnitude: -25,
                },
                Meteor {
                    shower: Shower::Perseids,
                    magnitude: 30,
                },
                Meteor {
                    shower: Shower::Perseids,
                    magnitude: 30,
                },
                Meteor {
                    shower: Shower::Sporadic,
                    magnitude: 50,
                },
                Meteor {
                    shower: Shower::Perseids,
                    magnitude: 30,
                },
                Meteor {
                    shower: Shower::Perseids,
                    magnitude: 30,
                },
            ],
        };
        assert_eq!(
            period.get_distribution_csv().unwrap(),
            "12 Aug 2019;0;30;PER;0;0;0;0;0;0.5;0.5;0;1;5;0;0;0;0\n12 Aug 2019;0;30;SPO;0;0;0;0.5;0.5;0;0;0;0;0;1;1;0;0".to_string()
        );
    }
}
