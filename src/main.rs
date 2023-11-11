use clap::{Arg, Command};
use std::{
    io::Write,
    time::{Duration, Instant},
};
use tokio::time::sleep;

async fn run_pomodoro_cycle(work_duration: u64, break_duration: u64, cycles: usize) {
    for cycle in 1..=cycles {
        println!("Pomodoro Cycle {} - Work", cycle);
        run_timer(work_duration).await;

        if cycle < cycles {
            println!("Pomodoro Cycle {} - Break", cycle);
            run_timer(break_duration).await;
        }
    }
}

async fn run_timer(duration: u64) {
    let start_time = Instant::now();
    let end_time = start_time + Duration::from_secs(duration);

    while Instant::now() < end_time {
        print!(
            "\rTime left: {} seconds",
            (end_time - Instant::now()).as_secs()
        );
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1)).await;
    }

    println!("\nTimer completed!");
}

#[tokio::main]
async fn main() {
    let matches = Command::new("Pomodoro Timer")
        .version("1.0")
        .author("Your Name")
        .about("Simple Pomodoro Timer implemented in Rust")
        .arg(
            Arg::new("work")
                .short('w')
                .long("work")
                .value_name("WORK_DURATION")
                .help("Sets the duration of the work interval in seconds")
                .default_value("25"),
        )
        .arg(
            Arg::new("break")
                .short('b')
                .long("break")
                .value_name("BREAK_DURATION")
                .help("Sets the duration of the break interval in seconds")
                .default_value("5"),
        )
        .arg(
            Arg::new("cycles")
                .short('c')
                .long("cycles")
                .value_name("CYCLES")
                .help("Sets the number of Pomodoro cycles")
                .default_value("4"),
        )
        .get_matches();

    let work_duration: u64 = matches.get_one::<String>("work").unwrap().parse().unwrap();
    let break_duration: u64 = matches.get_one::<String>("break").unwrap().parse().unwrap();
    let cycles: usize = matches
        .get_one::<String>("cycles")
        .unwrap()
        .parse()
        .unwrap();

    run_pomodoro_cycle(work_duration, break_duration, cycles).await;
}
