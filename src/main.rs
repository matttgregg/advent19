mod day1;
mod utils;

use std::env;
use std::process;

fn main() {
    match day_arg(env::args()) {
        Some(1) => day1::solve(),
        Some(0) | None => {
            day1::solve();
        },
        Some(x) => { eprintln!("Unimplemented day {}", x); process::exit(1);},
    }
}

fn day_arg(mut args: env::Args) -> Option<i32> {
    args.next();

    let day = match args.next() {
        Some(day) => day.parse::<i32>(),
        None => return None,
    };

    if let Ok(i) = day {
        return Some(i);
    }

    None
}
