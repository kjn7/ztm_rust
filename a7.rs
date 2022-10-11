// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
enum Color {
    Red,
    Green,
    Black
}

fn print_color(c: Color) {
    match c {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Black => println!("black")
    }
}
fn main() {
    let c = Color::Green;
    print_color(c);
}
