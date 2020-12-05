mod day_1;
mod day_2;

pub fn run(day: i32) {
    match day {
        1 => {
            println!("Running puzzle from day 1...");
            day_1::part_1();
            day_1::part_2();
        }
        2 => {
            println!("Running puzzle from day 2...");
            day_2::part_1();
            day_2::part_2();
        }
        _ => println!("Sorry, this puzzle is not implemented yet."),
    }
}
