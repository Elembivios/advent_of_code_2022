mod utils;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

use std::{error::Error, fs};
use structopt::StructOpt;
use owo_colors::OwoColorize;
use owo_colors::colors::{Magenta, Cyan};
use std::time::{Duration, Instant};
use anyhow::Context;
use humantime::format_duration;

fn get_time<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let start = Instant::now();
    let result = f();
    let time = start.elapsed();

    (result, time)
}
trait Advent {
    fn new(data: &str) -> Self
    where 
        Self: Sized;
    fn part_01(&self) -> String;
    fn part_02(&self) -> String;
}

struct Solution {
    event: Box<dyn Advent>,
    time: Duration,
}

impl Solution {
    fn new<Event: Advent + 'static>(content: &str) -> Self {
        let (event, time) = get_time(|| Event::new(content));

        Solution {
            event: Box::new(event),
            time,
        }
    }

    fn get_result(&self, day: u32) -> Duration {
        let (part1, time1) = get_time(|| self.event.part_01());
        let (part2, time2) = get_time(|| self.event.part_02());

        println!("--------------------------");
        println!("Solution for day {}", day.fg::<Cyan>());
        println!(
            "Collected data in {}",
            format_duration(self.time).fg::<Magenta>()    
        );
        println!(
            "Part 1: {} in {}",
            part1.fg::<Cyan>(),
            format_duration(time1).fg::<Magenta>()
        );
        println!(
            "Part 2: {} in {}",
            part2.fg::<Cyan>(),
            format_duration(time2).fg::<Magenta>()
        );
        self.time + time1 + time2
    }
}


#[derive(StructOpt)]
struct Cli {
    day: Option<u32>,

    #[structopt(short, long, help = "Uses example file provided by AOC")]
    example: bool,

}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();

    let main_file = if args.example { "example" } else { "input" };
    
    let days = if let Some(day) = args.day {
        day..=day
    } else {
        1u32..=7u32
    };
    let mut duration = Duration::new(0, 0);

    for day in days {
        let filename = format!("src/day_{:02}/{}.txt", day, main_file);

        let mut content: &str = &fs::read_to_string(filename)
            .with_context(|| format!("Could not read {} file for day {}", main_file, day))?;
        content = content.trim_end();

        let solution = match day {
            1 => Solution::new::<day_01::CalorieCounting>(content),
            2 => Solution::new::<day_02::RockPaperScissors>(content),
            3 => Solution::new::<day_03::RucksackReorganization>(content),
            4 => Solution::new::<day_04::CampCleanup>(content),
            5 => Solution::new::<day_05::SupplyStacks>(content),
            6 => Solution::new::<day_06::TuningTrouble>(content),
            7 => Solution::new::<day_07::NoSpaceLeftOnDevice>(content),
            _ => unreachable!(),
        };

        duration += solution.get_result(day);        
    }

    println!("--------------------------");
    println!(
        "Duration sum: {}",
        format_duration(duration).fg::<Magenta>()
    );
    println!("--------------------------");

    Ok(())
}
