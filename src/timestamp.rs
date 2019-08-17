use std::cmp::max;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Timestamp {
    hour: u32,
    minute: u32,
}

impl Timestamp {
    pub fn from_shorthand_int_notation(n: u32) -> Timestamp {
        Timestamp {
            hour: n / 100,
            minute: n % 100,
        }
    }

    pub fn incl_is_between(&self, a: Timestamp, b: Timestamp) -> bool {
        let b_hour = if b.hour < a.hour { b.hour + 24 } else { b.hour };
        let self_hour = if self.hour < a.hour {
            self.hour + 24
        } else {
            self.hour
        };
        let a_total_min = a.hour * 60 + a.minute;
        let self_total_min = self_hour * 60 + self.minute;
        let b_total_min = b_hour * 60 + b.minute;
        a_total_min <= self_total_min && self_total_min <= b_total_min
    }
}

impl std::ops::Sub for Timestamp {
    type Output = u32;

    // self - other
    // if self < other, assume self is the next day
    fn sub(self, other: Timestamp) -> u32 {
        let self_hour =
            if self.hour < other.hour || (self.hour == other.hour && self.minute < other.minute) {
                self.hour + 24
            } else {
                self.hour
            };
        self_hour * 60 + self.minute - (other.hour * 60 + other.minute)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_from_shorthand_notation_1() {
        assert_eq!(
            Timestamp::from_shorthand_int_notation(1554),
            Timestamp {
                hour: 15,
                minute: 54
            }
        )
    }

    #[test]
    pub fn test_from_shorthand_notation_2() {
        assert_eq!(
            Timestamp::from_shorthand_int_notation(0107),
            Timestamp { hour: 1, minute: 7 }
        );
    }

    #[test]
    pub fn test_sub_1() {
        let t1 = Timestamp {
            hour: 15,
            minute: 54,
        };
        let t2 = Timestamp {
            hour: 15,
            minute: 59,
        };
        assert_eq!(t2 - t1, 5);
    }

    #[test]
    pub fn test_sub_2() {
        let t1 = Timestamp {
            hour: 22,
            minute: 34,
        };
        let t2 = Timestamp {
            hour: 23,
            minute: 10,
        };
        assert_eq!(t2 - t1, 36);
    }

    #[test]
    pub fn test_sub_3() {
        let t1 = Timestamp {
            hour: 21,
            minute: 34,
        };
        let t2 = Timestamp {
            hour: 23,
            minute: 10,
        };
        assert_eq!(t2 - t1, 96);
    }

    #[test]
    pub fn test_sub_4() {
        let t1 = Timestamp {
            hour: 22,
            minute: 10,
        };
        let t2 = Timestamp {
            hour: 0,
            minute: 15,
        };
        assert_eq!(t2 - t1, 125);
    }

    #[test]
    pub fn test_sub_5() {
        let t1 = Timestamp {
            hour: 22,
            minute: 10,
        };
        let t2 = Timestamp {
            hour: 2,
            minute: 15,
        };
        assert_eq!(t2 - t1, 245);
    }

    #[test]
    pub fn test_sub_6() {
        let t1 = Timestamp {
            hour: 23,
            minute: 18,
        };
        let t2 = Timestamp {
            hour: 23,
            minute: 18,
        };
        assert_eq!(t2 - t1, 0);
    }

    #[test]
    pub fn test_sub_7() {
        let t1 = Timestamp {
            hour: 23,
            minute: 18,
        };
        let t2 = Timestamp {
            hour: 23,
            minute: 17,
        };
        assert_eq!(t2 - t1, 1439);
    }

    #[test]
    pub fn test_in_between_1() {
        let t1 = Timestamp {
            hour: 15,
            minute: 18,
        };
        let t2 = Timestamp {
            hour: 16,
            minute: 7,
        };
        let t3 = Timestamp {
            hour: 20,
            minute: 50,
        };
        assert!(t2.incl_is_between(t1, t3));
        assert!(!t1.incl_is_between(t2, t3));
        assert!(!t3.incl_is_between(t1, t2));
    }

    #[test]
    pub fn test_in_between_2() {
        let t1 = Timestamp {
            hour: 22,
            minute: 18,
        };
        let t2 = Timestamp {
            hour: 23,
            minute: 7,
        };
        let t3 = Timestamp {
            hour: 0,
            minute: 50,
        };
        assert!(t2.incl_is_between(t1, t3));
        assert!(!t1.incl_is_between(t2, t3));
        assert!(!t3.incl_is_between(t1, t2));
    }

    #[test]
    pub fn test_in_between_3() {
        let t1 = Timestamp {
            hour: 22,
            minute: 18,
        };
        let t2 = Timestamp { hour: 0, minute: 7 };
        let t3 = Timestamp {
            hour: 00,
            minute: 50,
        };
        assert!(t2.incl_is_between(t1, t3));
        assert!(!t1.incl_is_between(t2, t3));
        assert!(!t3.incl_is_between(t1, t2));
    }

    #[test]
    pub fn test_in_between_4() {
        let t1 = Timestamp {
            hour: 23,
            minute: 18,
        };
        let t2 = Timestamp { hour: 0, minute: 7 };
        let t3 = Timestamp {
            hour: 23,
            minute: 50,
        };
        assert!(!t2.incl_is_between(t1, t3));
    }

    #[test]
    pub fn test_in_between_5() {
        let t1 = Timestamp {
            hour: 23,
            minute: 18,
        };
        let t2 = Timestamp {
            hour: 23,
            minute: 18,
        };
        let t3 = Timestamp {
            hour: 23,
            minute: 50,
        };
        assert!(t2.incl_is_between(t1, t3));
        assert!(t1.incl_is_between(t2, t3));
    }

    #[test]
    pub fn test_in_between_6() {
        let t1 = Timestamp {
            hour: 23,
            minute: 18,
        };
        let t2 = Timestamp {
            hour: 23,
            minute: 50,
        };
        let t3 = Timestamp {
            hour: 23,
            minute: 50,
        };
        assert!(t2.incl_is_between(t1, t3));
        assert!(t3.incl_is_between(t1, t2));
    }

    #[test]
    pub fn test_in_between_7() {
        let t1 = Timestamp {
            hour: 23,
            minute: 18,
        };
        assert!(t1.incl_is_between(t1, t1));
    }
}
