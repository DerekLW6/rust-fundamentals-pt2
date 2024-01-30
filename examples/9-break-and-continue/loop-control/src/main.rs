// more on break statements: https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops

fn main() { 
    for i in 1..=100 {
        if i % 2 == 0 {
            // Skip even numbers
            continue;
        }
        println!("i = {}", i);
        if i == 7 {
            // Exit loop when i is 7
            break;
        }
    }
}
