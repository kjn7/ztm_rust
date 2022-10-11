// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
struct Assignment {
    name: String,
    locker: Option<i32>
}
fn main() {
    let bob = Assignment {
        name: String::from("Bob"),
        locker: Some(11)
    };
    match bob.locker {
        Some(locker) => println!("locker {:?}",locker),
        None => println!("no locker"),
    }
    println!("name {:?}", bob.name);
}
