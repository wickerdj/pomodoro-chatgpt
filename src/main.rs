use std::env;
use std::io::Write;
use std::time::{Duration, Instant};
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
    let args: Vec<String> = env::args().collect();

    let work_duration: u64 = args.get(1).and_then(|arg| arg.parse().ok()).unwrap_or(25);
    let break_duration: u64 = args.get(2).and_then(|arg| arg.parse().ok()).unwrap_or(5);
    let cycles: usize = args.get(3).and_then(|arg| arg.parse().ok()).unwrap_or(4);

    run_pomodoro_cycle(work_duration, break_duration, cycles).await;
}
