extern crate gtk;
#[macro_use]
extern crate secret;

use secret::{Schema, Passwords};
use secret::SchemaAttributeType::{self, Boolean, Integer};

fn main() {
    gtk::init().unwrap();

    let attribute_types = hash! {
        number => Integer,
        string => SchemaAttributeType::String,
        even => Boolean,
    };

    let schema = Schema::new("org.example.Password", attribute_types);

    let attributes = str_hash! {
        number => 8,
        string => "eight",
        even => true,
    };

    let passwords = Passwords::new(schema);
    passwords.store("The label", "the password", &attributes, |result| {
        println!("{:?}", result);
        gtk::main_quit();
    });

    gtk::main();

    passwords.store("Label 2", "Pass2", &str_hash! {
        number => 8,
        string => "huit",
        even => true,
    }, |result| {
        println!("Second: {:?}", result);
        gtk::main_quit();
    });

    gtk::main();

    passwords.lookup(&attributes, |password| {
        println!("{:?}", password);
        gtk::main_quit();
    });

    gtk::main();

    passwords.search(&str_hash! {
        number => 8,
    }, |items| {
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

    passwords.clear(&str_hash! { even => true, }, |result| {
        println!("{:?}", result);
        gtk::main_quit();
    });

    gtk::main();
}
