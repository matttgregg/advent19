use crate::utils;
use std::time::SystemTime;

pub fn solve() {
    utils::print_day(1);
    let data = include_str!("data/day1.dat");
    let modules: Vec<i32> = data.lines().map(|c| c.parse::<i32>().unwrap()).collect();
    let start = SystemTime::now();
    let all_fuel = modules.iter().map(fuel_for_module).collect::<Vec<i32>>();
    let sum_fuel: i32 = all_fuel.iter().sum();
    let all_recursed_fuel = modules.iter().map(recursed_fuel).collect::<Vec<i32>>();
    let sum_recursed_fuel: i32 = all_recursed_fuel.iter().sum();
    let timed = SystemTime::now().duration_since(start).unwrap();
    println!("Total fuel {}, including fuel+ {}",
             utils::fmt_bright(&sum_fuel), utils::fmt_bright(&sum_recursed_fuel));
    utils::print_duration(timed);
}

fn fuel_for_module(module: &i32) -> i32 {
    (*module / 3) - 2
}

fn recursed_fuel(module: &i32) -> i32 {
    let mut total_fuel = fuel_for_module(module);
    let mut next_fuel = fuel_for_module(&total_fuel);
    while next_fuel > 0 {
        total_fuel += next_fuel;
        next_fuel = fuel_for_module(&next_fuel);
    }
    total_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        assert_eq!(fuel_for_module(&12), 2);
        assert_eq!(fuel_for_module(&14), 2);
        assert_eq!(fuel_for_module(&1969), 654);
        assert_eq!(fuel_for_module(&100756), 33583);
        assert_eq!(recursed_fuel(&14), 2);
        assert_eq!(recursed_fuel(&1969), 966);
        assert_eq!(recursed_fuel(&100756), 50346);
    }
}
