use std::io::stdin;
/*
Debug placeholders for can be used for any type that supports the Debug trait.

{:?}
{:?}    raw
{:#?}   pretty-print
*/

#[derive(Debug)]
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
    let mut guest_list = vec![
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi Steve. Your milk is expired."),
        Visitor::new("fred", "Fred, you were invited?!"),
    ];

    loop {
        println!("Hello, what's your name? (Leave empty and press ENTER to quit");

        let name = name();

        let known_guest = guest_list
            .iter() // using an iterator instead of looping and returning bool!
            .find(|visitor| visitor.name == name); // find() will run a `closure`, returning an `Option`

        match known_guest { // run a match statement on the `Option` returned by find()
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the guest list.", name);
                    guest_list.push(Visitor::new(&name, "New friend"));
                }
            }
        }
    }

    println!("The final list of visitors:");
    println!("{:?}", guest_list)
}
