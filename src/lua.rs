use crate::areas::Area;
use crate::field::Field;
use crate::meteor::{Meteor, Shower};
use crate::session::Event;
use rlua;
use rlua::{Function, Lua, UserData};

impl UserData for Area {}
impl UserData for Meteor {}
impl UserData for Shower {}
impl UserData for Event {}

#[derive(Copy, Clone)]
struct Count(usize, Area);

impl UserData for Count {}

macro_rules! register_shower {
    ($shower_enum:ident, $lua_ctx:ident, $globals:ident) => {
        let shower_fn = $lua_ctx.create_function(|_, mag: f64| {
            let meteor = Meteor {
                shower: Shower::$shower_enum,
                magnitude: (mag * 10.0) as i32,
            };
            if meteor.magnitude % 5 == 0 {
                Ok(Event::Meteor(meteor))
            } else {
                Err(runtime_error("Invalid magnitude for given meteor"))
            }
        })?;
        $globals.set(Shower::$shower_enum.to_imo_code().to_lowercase(), shower_fn)?;
        $globals.set(Shower::$shower_enum.to_imo_code(), Shower::$shower_enum)?;
    };
}

fn runtime_error(desc: &str) -> rlua::Error {
    rlua::Error::RuntimeError(String::from(desc))
}

#[allow(clippy::type_complexity)]
pub fn new_lua() -> Result<Lua, rlua::Error> {
    let l = Lua::new();
    l.context(|lua_ctx| -> Result<(), rlua::Error> {
        let globals = lua_ctx.globals();

        register_shower!(Quadrantids, lua_ctx, globals);
        register_shower!(Lyrids, lua_ctx, globals);
        register_shower!(EtaAquarids, lua_ctx, globals);
        register_shower!(JuneBootids, lua_ctx, globals);
        register_shower!(DeltaAquariids, lua_ctx, globals);
        register_shower!(AlphaCapricornids, lua_ctx, globals);
        register_shower!(Perseids, lua_ctx, globals);
        register_shower!(KappaCygnids, lua_ctx, globals);
        register_shower!(AlphaAurigids, lua_ctx, globals);
        register_shower!(SeptemberEpsilonPerseids, lua_ctx, globals);
        register_shower!(OctoberCameloparalids, lua_ctx, globals);
        register_shower!(Draconids, lua_ctx, globals);
        register_shower!(EpsilonGeminids, lua_ctx, globals);
        register_shower!(Orionids, lua_ctx, globals);
        register_shower!(SouthernTaurids, lua_ctx, globals);
        register_shower!(NorthernTaurids, lua_ctx, globals);
        register_shower!(Leonids, lua_ctx, globals);
        register_shower!(DecemberAlphaDraconids, lua_ctx, globals);
        register_shower!(Monocerotids, lua_ctx, globals);
        register_shower!(SigmaHydrids, lua_ctx, globals);
        register_shower!(Geminids, lua_ctx, globals);
        register_shower!(DecemberLeoMinorids, lua_ctx, globals);
        register_shower!(ComaBerenicids, lua_ctx, globals);
        register_shower!(Ursids, lua_ctx, globals);
        register_shower!(Antihelion, lua_ctx, globals);
        register_shower!(Sporadic, lua_ctx, globals);

        globals.set("break_start", Event::BreakStart)?;
        globals.set("break_end", Event::BreakEnd)?;
        globals.set("new_period", Event::NewPeriod)?;
        globals.set("period_start", Event::PeriodStart)?;
        globals.set("period_end", Event::PeriodEnd)?;

        for i in 1..31 {
            let area_fn =
                lua_ctx.create_function(move |_, count: usize| Ok(Count(count, Area(i))))?;
            globals.set("area".to_owned() + &i.to_string(), area_fn)?;
        }

        let cloud_fn = lua_ctx.create_function(|_, cloud_percentage: u8| {
            if cloud_percentage > 99 {
                Err(runtime_error("Clouds cannot be more than 99%"))
            } else {
                Ok(Event::Clouds(cloud_percentage))
            }
        })?;
        globals.set("clouds", cloud_fn)?;

        let area_fn = lua_ctx.create_function(
            |_,
             counts: (
                Option<Count>,
                Option<Count>,
                Option<Count>,
                Option<Count>,
                Option<Count>,
                Option<Count>,
                Option<Count>,
                Option<Count>,
                Option<Count>,
                Option<Count>,
                Option<Count>,
                Option<Count>,
            )| {
                Ok(Event::AreasCounted(
                    [
                        counts.0, counts.1, counts.2, counts.3, counts.4, counts.5, counts.6,
                        counts.7, counts.8, counts.9, counts.10, counts.11,
                    ]
                    .iter()
                    .flatten()
                    .map(|c| (c.0, c.1))
                    .collect(),
                ))
            },
        )?;
        globals.set("areas", area_fn)?;

        let field_fn = lua_ctx
            .create_function(|_, (ra, dec): (f64, f64)| Ok(Event::Field(Field { ra, dec })))?;
        globals.set("fieldC", field_fn)?;

        let showers_fn = lua_ctx.create_function(
            |_,
             showers: (
                Option<Shower>,
                Option<Shower>,
                Option<Shower>,
                Option<Shower>,
                Option<Shower>,
                Option<Shower>,
                Option<Shower>,
                Option<Shower>,
                Option<Shower>,
                Option<Shower>,
                Option<Shower>,
                Option<Shower>,
            )| {
                Ok(Event::Showers(
                    [
                        showers.0, showers.1, showers.2, showers.3, showers.4, showers.5,
                        showers.6, showers.7, showers.8, showers.9, showers.10, showers.11,
                    ]
                    .iter()
                    .flatten()
                    .copied()
                    .collect(),
                ))
            },
        )?;
        globals.set("showers", showers_fn)?;

        let date_fn = lua_ctx.create_function(|_, date: String| Ok(Event::PeriodDate(date)))?;
        globals.set("date", date_fn)?;

        Ok(())
    })?;
    Ok(l)
}

pub fn run_code(code: &str, l: &Lua) -> Result<Event, rlua::Error> {
    l.context(|lua_ctx| {
        let f: Function = lua_ctx
            .load(&("function() return ".to_owned() + code + " end"))
            .eval()?;
        f.call::<_, Event>(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lua_1() {
        let l = new_lua().unwrap();
        let event = run_code("per(3.5)", &l).unwrap();
        assert_eq!(
            event,
            Event::Meteor(Meteor {
                shower: Shower::Perseids,
                magnitude: 35,
            })
        );

        let event = run_code("spo(-2)", &l).unwrap();
        assert_eq!(
            event,
            Event::Meteor(Meteor {
                shower: Shower::Sporadic,
                magnitude: -20
            })
        );
    }

    #[test]
    fn test_lua_2() {
        let l = new_lua().unwrap();

        assert_eq!(run_code("break_start", &l).unwrap(), Event::BreakStart);
        assert_eq!(run_code("break_end", &l).unwrap(), Event::BreakEnd);
        assert_eq!(run_code("new_period", &l).unwrap(), Event::NewPeriod);
        assert_eq!(run_code("period_start", &l).unwrap(), Event::PeriodStart);
        assert_eq!(run_code("period_end", &l).unwrap(), Event::PeriodEnd);
    }

    #[test]
    fn test_lua_3() {
        let l = new_lua().unwrap();

        assert_eq!(
            run_code("fieldC(336, 52.3)", &l).unwrap(),
            Event::Field(Field {
                ra: 336.0,
                dec: 52.3
            })
        );
    }

    #[test]
    fn test_lua_4() {
        let l = new_lua().unwrap();

        assert_eq!(run_code("clouds(5)", &l).unwrap(), Event::Clouds(5));
    }

    #[test]
    fn test_lua_5() {
        let l = new_lua().unwrap();

        assert!(run_code("clouds(101)", &l).is_err());
    }

    #[test]
    fn test_lua_6() {
        let l = new_lua().unwrap();

        let ac = run_code("areas(area14(10), area7(11), area6(7))", &l).unwrap();
        assert_eq!(
            ac,
            Event::AreasCounted(vec![(10, Area(14)), (11, Area(7)), (7, Area(6))])
        );
    }

    #[test]
    fn test_lua_7() {
        let l = new_lua().unwrap();

        let showers = run_code("showers(PER, ANT, KCG, SPO)", &l).unwrap();
        assert_eq!(
            showers,
            Event::Showers(vec![
                Shower::Perseids,
                Shower::Antihelion,
                Shower::KappaCygnids,
                Shower::Sporadic
            ])
        );
    }

    #[test]
    fn test_lua_8() {
        let l = new_lua().unwrap();
        assert!(run_code("per(3.7)", &l).is_err());
    }
}
