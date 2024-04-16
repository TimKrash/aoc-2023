pub mod solutions;

pub fn run_day(day: i32) {
    match day {
        2 => solutions::two::main(),
        _ => panic!("Other days not implemented yet!")
    };
}
