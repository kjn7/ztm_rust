// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
enum Color {
    Red,
    Green,
    Blue
}
struct Box {
    weight: f64,
    color: Color,
    dims: (i32, i32, i32)
}

impl Box {
    fn create(w: f64, c: Color, d: (i32,i32,i32)) -> Self {
        Self {
            weight: w,
            color: c,
            dims: d
        }
    }

    fn print(&self) {
        println!("weight: {:?}", self.weight);
        match self.color {
            Color::Red => println!("color: red"),
            Color::Green => println!("color: green"),
            Color::Blue => println!("color: blue"),
        }
        let (x, y, z) = self.dims;
        println!("x: {:?}", x);
        println!("y: {:?}", y);
        println!("z: {:?}", z);
    }
}
fn main() {
    let b = Box::create(1.5, Color::Green, (10, 20, 30));
    b.print();
    let b2 = Box::create(2.3, Color::Blue, (1, 2, 3));
    b2.print();
}
