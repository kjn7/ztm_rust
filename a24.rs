// Topic: Iterator
//
// Requirements:
// * Triple the value of each item in a vector.
// * Filter the data to only include values > 10.
// * Print out each element using a for loop.
//
// Notes:
// * Use an iterator chain to accomplish the task.

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let data2: Vec<i32> = data
        .iter()
        .map(|i| i * 3)
        .collect();
    let data3: Vec<_> = data2
        .iter()
        .filter(|i| i > &&10)
        .collect();

    for d in data3 {
        println!("{:?}", d);
    }
}
