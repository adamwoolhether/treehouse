use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}
impl Visitor {
    fn new(name: &str, greeting: &str) -> Self { // accept str, but store as String.
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

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

    let guest_list = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi Steve. Your milk is expired."),
        Visitor::new("fred", "Fred, you were invited?!"),
    ];
    let known_guest = guest_list
        .iter() // using an iterator instead of looping and returning bool!
        .find(|visitor| visitor.name == name); // find() will run a `closure`, returning an `Option`
    match known_guest { // run a match statement on the `Option` returned by find()
        Some(visitor) => visitor.greet_visitor(),
        None => println!("You aren't on the guest list, please leave.")
    }

}
