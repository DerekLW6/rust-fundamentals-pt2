
fn main() {
    let mut message = String::from("Name: Alfredo, Height: "); //Let mut, makes the variable mutable
    message.clear();
    let mut height = 190; // You don't need to use let again if you are reassigning a variable 
    height = 189;
    println!("{}{}", message, height);

}
