use crate::args::{parse, Arguments};
use crate::commands::run::run;
use crate::commands::scaffold::scaffold;
use std::process::exit;

mod commands;

mod args {
    use anyhow::*;
    use chrono::{Datelike, Utc};
    use chrono_tz::Tz::America__Los_Angeles;
    use std::env;

    pub enum Arguments {
        Run {
            day: String,
            part: i32,
        },
        Scaffold {
            day: String,
        },
    }

    fn get_day(args: &[String]) -> String {
        let mut current_day = Utc::now().with_timezone(&America__Los_Angeles).day().to_string();

        if args.len() > 2 {
            current_day = args[2].to_string();
        }

        if current_day.len() == 1 {
            current_day.insert(0, '0'.into());
        }

        current_day
    }

    pub fn parse() -> Result<Arguments> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            panic!("Not enough arguments supplied");
        }

        let day = get_day(&args);

        match args[1].as_ref() {
            "run" => Ok(Arguments::Run { day, part: args.iter().nth(3).map(|x| x.parse::<i32>().unwrap_or(0)).unwrap_or(0) }),
            "scaffold" => Ok(Arguments::Scaffold { day }),
            _ => Err(anyhow!("Not a valid argument")),
        }
    }
}

fn main() {
    match parse() {
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
        Ok(args) => match args {
            Arguments::Run { day, part } => run(day, part),
            Arguments::Scaffold { day } => scaffold(day)
        }
    }
}