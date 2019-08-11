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
}
