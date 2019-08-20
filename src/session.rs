use crate::areas::Area;
use crate::distribution::Distribution;
use crate::field::Field;
use crate::meteor::{Meteor, Shower};
use crate::timestamp::Timestamp;
use std::collections::{HashMap, HashSet};
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

    pub fn get_distribution_csv(&self, count_and_dist: &HashMap<Shower, (u32, Distribution)>) -> Result<String, NoneError> {
        let mut lines: Vec<String> = vec![];

        let mut showers_sorted = self.showers.clone();
        showers_sorted.sort_by(|a, b| a.to_imo_code().cmp(b.to_imo_code()));

        for shower in showers_sorted {
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

impl Session {
    fn all_showers(&self) -> HashSet<Shower> {
        let mut result = HashSet::new();
        for period in &self.periods {
            for shower in &period.showers {
                result.insert(*shower);
            }
        }
        result
    }

    pub fn get_csvs(&self) -> Result<(String, String), NoneError> {
        let mut showers: Vec<Shower> = self.all_showers().into_iter().collect();
        showers.sort_by(|a, b| a.to_imo_code().cmp(b.to_imo_code()));

        let mut count_csv_parts: Vec<String> = vec![];
        count_csv_parts.push(format!(
            "DATE UT;START;END;Teff;RA;Dec;F;Lm;{}",
            &showers
                .iter()
                .map(|s| s.to_imo_code())
                .collect::<Vec<&str>>()
                .join(";;")
        ));

        let mut distr_csv_parts: Vec<String> = vec![];
        distr_csv_parts
            .push("DATE UT;START;END;SHOWER;-6;-5;-4;-3;-2;-1;0;1;2;3;4;5;6;7".to_owned());
        for period in &self.periods {
            let count_and_dist = period.get_count_and_distribution();
            distr_csv_parts.push(period.get_distribution_csv(&count_and_dist)?);

            let mut count_parts: Vec<String> = vec![];
            for shower in &showers {
                match count_and_dist.get(&shower) {
                    Some((count, _)) => count_parts.push(format!("C;{}", count)),
                    _ => count_parts.push("-;".to_owned()),
                };
            }
            count_csv_parts.push(format!(
                "{};{};{};{};{};{};{};{};{}",
                period.date,
                period.start_time.to_shorthand_int_notation(),
                period.end_time.to_shorthand_int_notation(),
                period.teff,
                period.field.ra,
                period.field.dec,
                period.cloud_factor,
                period.limiting_magnitude,
                count_parts.join(";")
            ));
        }

        Ok((count_csv_parts.join("\n"), distr_csv_parts.join("\n")))
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
            period.get_distribution_csv(&period.get_count_and_distribution()).unwrap(),
            "12 Aug 2019;0;30;PER;0;0;0;0;0;0.5;0.5;0;1;5;0;0;0;0\n12 Aug 2019;0;30;SPO;0;0;0;0.5;0.5;0;0;0;0;0;1;1;0;0".to_string()
        );
    }

    #[test]
    fn test_all_showers() {
        let period1 = Period {
            start_time: Timestamp { hour: 0, minute: 0 },
            end_time: Timestamp { hour: 0, minute: 0 },
            date: "12 Aug 2019".to_owned(),
            teff: 0.0,
            limiting_magnitude: 0.0,
            field: Field { ra: 0.0, dec: 0.0 },
            cloud_factor: 1.0,
            showers: vec![Shower::Perseids, Shower::Antihelion, Shower::Sporadic],
            meteors: vec![],
        };

        let period2 = Period {
            start_time: Timestamp { hour: 0, minute: 0 },
            end_time: Timestamp { hour: 0, minute: 0 },
            date: "12 Aug 2019".to_owned(),
            teff: 0.0,
            limiting_magnitude: 0.0,
            field: Field { ra: 0.0, dec: 0.0 },
            cloud_factor: 1.0,
            showers: vec![Shower::KappaCygnids, Shower::Sporadic],
            meteors: vec![],
        };

        let session = Session {
            periods: vec![period1, period2],
        };

        let mut expected = HashSet::new();
        expected.insert(Shower::Perseids);
        expected.insert(Shower::Antihelion);
        expected.insert(Shower::KappaCygnids);
        expected.insert(Shower::Sporadic);
        assert_eq!(session.all_showers(), expected);
    }

    #[test]
    fn test_session_to_csv() {
        let period1 = Period {
            start_time: Timestamp {
                hour: 23,
                minute: 30,
            },
            end_time: Timestamp {
                hour: 0,
                minute: 30,
            },
            date: "12 Aug 2019".to_owned(),
            teff: 1.0,
            limiting_magnitude: 5.52,
            field: Field {
                ra: 336.0,
                dec: 52.3,
            },
            cloud_factor: 1.05,
            showers: vec![Shower::Perseids, Shower::Sporadic, Shower::KappaCygnids],
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

        let period2 = Period {
            start_time: Timestamp {
                hour: 1,
                minute: 30,
            },
            end_time: Timestamp { hour: 2, minute: 0 },
            date: "13 Aug 2019".to_owned(),
            teff: 0.5,
            limiting_magnitude: 5.91,
            field: Field {
                ra: 298.0,
                dec: 56.0,
            },
            cloud_factor: 1.08,
            showers: vec![Shower::Antihelion, Shower::Sporadic, Shower::KappaCygnids],
            meteors: vec![
                Meteor {
                    shower: Shower::Sporadic,
                    magnitude: 30,
                },
                Meteor {
                    shower: Shower::Sporadic,
                    magnitude: -10,
                },
                Meteor {
                    shower: Shower::Sporadic,
                    magnitude: 5,
                },
            ],
        };

        let session = Session {
            periods: vec![period1, period2],
        };
        let (count_csv, distr_csv) = session.get_csvs().unwrap();

        assert_eq!(
            count_csv,
            "DATE UT;START;END;Teff;RA;Dec;F;Lm;ANT;;KCG;;PER;;SPO\n12 Aug 2019;2330;30;1;336;52.3;1.05;5.52;-;;C;0;C;7;C;3\n13 Aug 2019;130;200;0.5;298;56;1.08;5.91;C;0;C;0;-;;C;3"
        );

        assert_eq!(
            distr_csv,
            "DATE UT;START;END;SHOWER;-6;-5;-4;-3;-2;-1;0;1;2;3;4;5;6;7
12 Aug 2019;2330;30;KCG;0;0;0;0;0;0;0;0;0;0;0;0;0;0
12 Aug 2019;2330;30;PER;0;0;0;0;0;0.5;0.5;0;1;5;0;0;0;0
12 Aug 2019;2330;30;SPO;0;0;0;0.5;0.5;0;0;0;0;0;1;1;0;0
13 Aug 2019;130;200;ANT;0;0;0;0;0;0;0;0;0;0;0;0;0;0
13 Aug 2019;130;200;KCG;0;0;0;0;0;0;0;0;0;0;0;0;0;0
13 Aug 2019;130;200;SPO;0;0;0;0;0;1;0.5;0.5;0;1;0;0;0;0"
        );
    }
}
