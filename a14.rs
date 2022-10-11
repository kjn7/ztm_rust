// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
struct Person {
    color: String,
    name: String,
    age: i32,
}

fn print_name(name: &str) {
    println!("name: {:?}", name);
}
fn print_color(color: &str) {
    println!("color: {:?}", color);
}
fn main() {
    let persons = vec![
        Person {color:"red".to_owned(), name: "Jason".to_owned(), age:10},
        Person {color:"green".to_owned(), name:"Bill".to_owned(), age:5},
        Person {color:"blue".to_owned(), name:"Alla".to_owned(), age:20}
    ];

    for p in &persons {
        if p.age <= 10 {
            print_name(&p.name);
            print_color(&p.color);
            println!("age: {:?}", p.age);
        }
    }
}
