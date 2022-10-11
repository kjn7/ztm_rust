// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info
enum Ticket {
    Backstage(String, i32),
    Vip(String, i32),
    Standard(String, i32),
}

fn main() {
    let tickets = vec![
        Ticket::Backstage("Bob".to_owned(), 1000),
        Ticket::Vip("Vov".to_owned(), 10000),
        Ticket::Standard("User".to_owned(), 10),
    ];

    for t in tickets {
        match t {
            Ticket::Backstage(name, price) => println!("backstage {:?} / {:?}",name, price),
            Ticket::Vip(name,price) => println!("vip {:?} {:?}", name, price),
            Ticket::Standard(name, price) => println!("std {:?} {:?}",name,price),
            _ => (),
        }

    }

}
