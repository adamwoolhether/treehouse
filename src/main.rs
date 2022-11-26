use std::io::stdin;

fn name() -> String{
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name
        .trim() // remove non-print chars like \n, \r, etc.
        .to_lowercase()
}

fn main() {
    println!("Hello, what's your name?");
    let name = name();
    println!("Hello, {}", name); // use {:?} for debugging.

    let guest_list = ["bert", "steve", "fred"];
    let mut allow_them_in = false;
    for visitor in &guest_list { // use & to borrow, not copy the data.
        if visitor == &name {
            allow_them_in = true;
        }
    }

    if allow_them_in {
        println!("Welcome to the treehouse, {}", name);
    } else {
        println!("Sorry, you aren't on the list.");
    }
}
