use crate::meteor::Meteor;
use std::cmp::{max, min};
use std::collections::HashMap;

pub struct Distribution(Vec<u32>);

impl Distribution {
    pub fn new() -> Distribution {
        let counts = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        Distribution(counts)
    }

    pub fn add_meteor(&mut self, meteor: &Meteor) {
        let mag = max(-60, min(70, meteor.magnitude));
        let counts = &mut self.0;
        if mag % 10 == 0 {
            counts[(mag / 10 + 6) as usize] += 10;
        } else {
            counts[((mag - 5) / 10 + 6) as usize] += 5;
            counts[((mag + 5) / 10 + 6) as usize] += 5;
        }
    }

    pub fn to_map(&self) -> HashMap<i32, u32> {
        let mut map = HashMap::new();
        map.insert(-6, self.0[0]);
        map.insert(-5, self.0[1]);
        map.insert(-4, self.0[2]);
        map.insert(-3, self.0[3]);
        map.insert(-2, self.0[4]);
        map.insert(-1, self.0[5]);
        map.insert(0, self.0[6]);
        map.insert(1, self.0[7]);
        map.insert(2, self.0[8]);
        map.insert(3, self.0[9]);
        map.insert(4, self.0[10]);
        map.insert(5, self.0[11]);
        map.insert(6, self.0[12]);
        map.insert(7, self.0[13]);
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meteor::Shower;

    #[test]
    fn test_distr_1() {
        let mut distr = Distribution::new();
        distr.add_meteor(&Meteor {
            shower: Shower::Perseids,
            magnitude: 20,
        });
        assert_eq!(distr.0, vec![0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0]);
        let map = distr.to_map();
        assert_eq!(map.get(&2), Some(&10));
    }

    #[test]
    fn test_distr_2() {
        let mut distr = Distribution::new();
        distr.add_meteor(&Meteor {
            shower: Shower::Perseids,
            magnitude: 15,
        });
        assert_eq!(distr.0, vec![0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0]);
        let map = distr.to_map();
        assert_eq!(map.get(&1), Some(&5));
        assert_eq!(map.get(&2), Some(&5));
    }

    #[test]
    fn test_distr_3() {
        let mut distr = Distribution::new();
        distr.add_meteor(&Meteor {
            shower: Shower::Perseids,
            magnitude: -15,
        });
        assert_eq!(distr.0, vec![0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0]);
        let map = distr.to_map();
        assert_eq!(map.get(&-1), Some(&5));
        assert_eq!(map.get(&-2), Some(&5));
    }

    #[test]
    fn test_distr_4() {
        let mut distr = Distribution::new();
        distr.add_meteor(&Meteor {
            shower: Shower::Perseids,
            magnitude: 75,
        });
        assert_eq!(distr.0, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]);
        let map = distr.to_map();
        assert_eq!(map.get(&7), Some(&10));
    }

    #[test]
    fn test_distr_5() {
        let mut distr = Distribution::new();
        distr.add_meteor(&Meteor {
            shower: Shower::Perseids,
            magnitude: -65,
        });
        assert_eq!(distr.0, vec![10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let map = distr.to_map();
        assert_eq!(map.get(&-6), Some(&10));
    }

    #[test]
    fn test_distr_6() {
        let mut distr = Distribution::new();
        distr.add_meteor(&Meteor {
            shower: Shower::Perseids,
            magnitude: 30,
        });
        distr.add_meteor(&Meteor {
            shower: Shower::Perseids,
            magnitude: 30,
        });
        distr.add_meteor(&Meteor {
            shower: Shower::Perseids,
            magnitude: 10,
        });
        distr.add_meteor(&Meteor {
            shower: Shower::Perseids,
            magnitude: 25,
        });
        distr.add_meteor(&Meteor {
            shower: Shower::Perseids,
            magnitude: -15,
        });
        distr.add_meteor(&Meteor {
            shower: Shower::Perseids,
            magnitude: 50,
        });
        distr.add_meteor(&Meteor {
            shower: Shower::Perseids,
            magnitude: 40,
        });
        distr.add_meteor(&Meteor {
            shower: Shower::Perseids,
            magnitude: 45,
        });
        distr.add_meteor(&Meteor {
            shower: Shower::Perseids,
            magnitude: 0,
        });
        distr.add_meteor(&Meteor {
            shower: Shower::Perseids,
            magnitude: -5,
        });
        assert_eq!(
            distr.0,
            vec![0, 0, 0, 0, 5, 10, 15, 10, 5, 25, 15, 15, 0, 0]
        );
    }
}
