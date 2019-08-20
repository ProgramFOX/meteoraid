use crate::builder::{BuilderError, SessionBuilder};
use crate::lua;
use crate::session::*;
use crate::timestamp::Timestamp;
use rlua::Lua;

pub struct Interpreter {
    session_builder: SessionBuilder,
    time_checkpoint: Option<Timestamp>,
    lua: Lua,
}

impl Interpreter {
    pub fn new() -> Result<Self, rlua::Error> {
        Ok(Self {
            session_builder: SessionBuilder::new(),
            time_checkpoint: None,
            lua: lua::new_lua()?,
        })
    }

    pub fn execute_one_line(&mut self, line: &str) -> Result<(), Box<dyn std::error::Error>> {
        let code = line
            .split("--")
            .next()
            .expect("[supposedly unreachable] Split has no values")
            .trim();

        if code.is_empty() {
            return Ok(());
        }

        let mut split = code.split("<<");
        let code = split
            .next()
            .expect("[supposedly unreachable] Split has no values (2)")
            .trim();
        let maybe_exact_timestamp = match split.next() {
            Some(time) => Some(Timestamp::from_shorthand_int_notation(
                time.trim().parse::<u32>()?,
            )),
            None => None,
        };

        if let Ok(number) = code.parse::<u32>() {
            self.time_checkpoint = Some(Timestamp::from_shorthand_int_notation(number));
        } else {
            if let Some(exact_timestamp) = maybe_exact_timestamp {
                self.time_checkpoint = Some(exact_timestamp);
            }
            match self.time_checkpoint {
                Some(time) => {
                    let time_and_event = TimestampedEvent(time, lua::run_code(code, &self.lua)?);
                    self.session_builder.register_event(time_and_event)?;
                }
                None => return Err(Box::new(InterpreterError::NoTimeCheckpoint)),
            }
        }
        Ok(())
    }

    #[cfg(test)]
    pub fn execute_multiple_lines(&mut self, code: &str) -> Result<(), Box<dyn std::error::Error>> {
        for line in code.split('\n') {
            self.execute_one_line(line)?;
        }
        Ok(())
    }

    pub fn get_session(self) -> Result<Session, BuilderError> {
        self.session_builder.into_session()
    }
}

#[derive(Debug, Clone)]
pub enum InterpreterError {
    NoTimeCheckpoint,
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NoTimeCheckpoint => "No time checkpoint has been given.",
            }
        )
    }
}

impl std::error::Error for InterpreterError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpreter_1() {
        let mut interpreter = Interpreter::new().unwrap();
        interpreter
            .execute_multiple_lines(
                "2237
                 period_start -- and a comment
                 date(\"12 Aug 2019\")
                 clouds(0)
                 showers(PER, ANT, KCG, SPO)
                 areas(area14(11))
                 fieldC(336, 52.3)
                 2337
                 period_end",
            )
            .unwrap();
        let session = interpreter.get_session().unwrap();
        let period = &session.periods[0];
        assert_eq!(period.teff, 1.0);
        assert_eq!(period.showers.len(), 4);
        assert_eq!(period.cloud_factor, 1.0);
    }

    #[test]
    fn test_interpreter_2() {
        let mut interpreter = Interpreter::new().unwrap();
        interpreter
            .execute_multiple_lines(
                "period_start << 2237
                 date(\"12 Aug 2019\")
                 clouds(0)
                 showers(PER, ANT, KCG, SPO)
                 areas(area14(11))
                 fieldC(336, 52.3)
                period_end << 2337",
            )
            .unwrap();
        let session = interpreter.get_session().unwrap();
        let period = &session.periods[0];
        assert_eq!(period.teff, 1.0);
        assert_eq!(period.showers.len(), 4);
        assert_eq!(period.cloud_factor, 1.0);
    }

    #[test]
    fn test_interpreter_3() {
        let mut interpreter = Interpreter::new().unwrap();
        assert!(interpreter
            .execute_multiple_lines(
                "period_start
                 date(\"12 Aug 2019\")
                 clouds(0)
                 showers(PER, ANT, KCG, SPO)
                 areas(area14(11))
                 fieldC(336, 52.3)
                 period_end << 2337",
            )
            .is_err());
    }

    #[test]
    fn test_interpreter_4() {
        let mut interpreter = Interpreter::new().unwrap();
        interpreter
            .execute_multiple_lines(
                "period_start << 2237
                 date(\"12 Aug 2019\")
                 clouds(0)
                 showers(PER, ANT, KCG, SPO)
                 areas(area14(11))
                 fieldC(336, 52.3)
                 2307
                 areas(area14(9))
                 clouds(10)
                 per(3.5)
                 period_end << 2337",
            )
            .unwrap();
        let session = interpreter.get_session().unwrap();
        let period = &session.periods[0];
        assert_eq!(period.teff, 1.0);
        assert_eq!(period.showers.len(), 4);
        assert_eq!(period.cloud_factor, 1.05);
        assert_eq!(period.limiting_magnitude, 5.52);
        assert_eq!(period.meteors.len(), 1);
    }

    #[test]
    fn test_interpreter_5() {
        let mut interpreter = Interpreter::new().unwrap();
        interpreter
            .execute_multiple_lines(
                "period_start << 2237
                 date(\"12 Aug 2019\")
                 clouds(0)
                 showers(PER, ANT, KCG, SPO)
                 areas(area14(11))
                 fieldC(336, 52.3)
                 2307
                 areas(area14(9))
                 clouds(10)
                 per(3.5)
                 period_end << 2337
                 
                 new_period

                 0015
                 period_start
                 date(\"13 Aug 2019\")
                 clouds(5)
                 showers(PER, SPO)
                 areas(area14(10), area7(10))
                 fieldC(298, 56)
                 period_end << 0030
                 ",
            )
            .unwrap();
        let session = interpreter.get_session().unwrap();
        assert_eq!(session.periods.len(), 2);
        let period = &session.periods[0];
        assert_eq!(period.teff, 1.0);
        assert_eq!(period.showers.len(), 4);
        assert_eq!(period.cloud_factor, 1.05);
        assert_eq!(period.limiting_magnitude, 5.52);
        assert_eq!(period.meteors.len(), 1);

        let period2 = &session.periods[1];
        assert_eq!(period2.teff, 0.25);
    }

    #[test]
    fn test_interpreter_6() {
        let mut interpreter = Interpreter::new().unwrap();
        interpreter
            .execute_multiple_lines(
                "2237
                 period_start
                 date(\"12 Aug 2019\")
                 clouds(0)
                 showers(PER, ANT, KCG, SPO) << 2238
                 areas(area14(11))
                 fieldC(336, 52.3)
                 2337
                 period_end",
            )
            .unwrap();
        assert!(interpreter.get_session().is_err());
    }
}
