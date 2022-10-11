// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats
use chrono::*;
fn main() {
    let dt: Date<Local> = Local::today();
    println!("{}", dt.format("%d/%m/%Y"));
    
    let dt2: DateTime<Local> = Local::now();
    println!("{}", dt2.format("%d/%m/%Y %H:%M"));

}
