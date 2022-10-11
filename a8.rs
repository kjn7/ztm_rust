// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Beer,
    Wine
}

struct Drink {
    oz: f64,
    flavor: Flavor
}

fn print_drink(d: Drink ) {
    match d.flavor {
        Flavor::Beer => println!("it's beer"),
        Flavor::Wine => println!("it's wine")
    }
    println!("oz {:?}", d.oz)
}
fn main() {
    let d = Drink {
        oz: 1.11,
        flavor: Flavor::Wine
    };
    print_drink(d);
}
