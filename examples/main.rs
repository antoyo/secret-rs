extern crate gtk;
extern crate secret;

use std::collections::HashMap;

use secret::{Schema, Passwords};
use secret::SchemaAttributeType::{self, Boolean, Integer};

fn main() {
    gtk::init().unwrap();

    let mut attribute_types = HashMap::new();
    attribute_types.insert("number".to_string(), Integer);
    attribute_types.insert("string".to_string(), SchemaAttributeType::String);
    attribute_types.insert("even".to_string(), Boolean);

    let schema = Schema::new("org.example.Password", attribute_types);

    let mut attributes = HashMap::new();
    attributes.insert("number".to_string(), "8".to_string());
    attributes.insert("string".to_string(), "eight".to_string());
    attributes.insert("even".to_string(), "true".to_string());

    let passwords = Passwords::new(schema);
    passwords.store("The label", "the password", &attributes, |result| {
        println!("{:?}", result);
        gtk::main_quit();
    });

    gtk::main();

    passwords.lookup(&attributes, |password| {
        println!("{:?}", password);
        gtk::main_quit();
    });

    gtk::main();

    passwords.search(&attributes, |items| {
        println!("****************");
        let items = items.unwrap();
        for item in items {
            println!("Label: {:?}", item.get_label());
            println!("Secret: {:?}", item.get_secret().and_then(|secret| secret.get_text()));
            let attributes = item.get_attributes();
            for (key, value) in attributes {
                println!("{}: {}", key, value);
            }
            println!("****************");
        }
        gtk::main_quit();
    });

    gtk::main();

    passwords.clear(&attributes, |result| {
        println!("{:?}", result);
        gtk::main_quit();
    });

    gtk::main();
}
