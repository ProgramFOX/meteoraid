use crate::areas::Area;
use crate::distribution::Distribution;
use crate::field::Field;
use crate::meteor::{Meteor, Shower};
use crate::timestamp::Timestamp;
use std::collections::HashMap;

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
    Showers(Vec<Shower>),
}

pub struct TimestampedEvent(pub Timestamp, pub Event);

pub struct Period {
    pub start_time: Timestamp,
    pub end_time: Timestamp,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_count_and_distribution_1() {
        let period = Period {
            start_time: Timestamp { hour: 0, minute: 0 },
            end_time: Timestamp { hour: 0, minute: 0 },
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
        assert_eq!(*perseid_distr_map.get(&30).unwrap(), 50);
        assert_eq!(*perseid_distr_map.get(&20).unwrap(), 10);
        assert_eq!(*perseid_distr_map.get(&0).unwrap(), 5);
        assert_eq!(*perseid_distr_map.get(&-10).unwrap(), 5);
        assert_eq!(*perseid_distr_map.get(&-60).unwrap(), 0);

        let sporadic_info = cd.get(&Shower::Sporadic).unwrap();
        assert_eq!(sporadic_info.0, 3);
        let sporadic_distr_map = sporadic_info.1.to_map();
        assert_eq!(*sporadic_distr_map.get(&-30).unwrap(), 5);
        assert_eq!(*sporadic_distr_map.get(&-20).unwrap(), 5);
        assert_eq!(*sporadic_distr_map.get(&40).unwrap(), 10);
        assert_eq!(*sporadic_distr_map.get(&50).unwrap(), 10);
        assert_eq!(*sporadic_distr_map.get(&10).unwrap(), 0);
    }
}
