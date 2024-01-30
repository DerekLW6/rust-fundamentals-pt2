// There are other conditionals that we can explore in Rust. Like using `if let`

fn main() {
    let maybe_number: Option<Option<()>>  = Some(None);

    if let Some(inner_option) = maybe_number {
        if let Some(()) = inner_option {
            println!("The number is present");
        } else {
            println!("The inner option is None");
        }
    } else {
        println!("The outer option is None");
    }
}
 
