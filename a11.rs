// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter
struct Item {
    quantity: i32,
    id: i32,
}

fn display_quantity(i: &Item) {
    println!("quantity {:?}", i.quantity);
}

fn display_id(i: &Item) {
    println!("id {:?}", i.id);
}
fn main() {
    let i = Item {
        quantity: 12,
        id: 100
    };
    display_quantity(&i);
    display_id(&i);
}
