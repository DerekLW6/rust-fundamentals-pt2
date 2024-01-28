

fn main() {
    let message = "Name: Derek, Weight: "; //Variables are immutable by default (use let mut to make them mutable)
    let weight = 205.0;

    let kilos = weight / 2.2;
    println!("{}{}", message, weight);
    println!("{}{}", message, kilos);
    
}
