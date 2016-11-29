extern crate gio_sys;
extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;
extern crate gtk;
extern crate libc;
#[macro_use]
extern crate secret;
extern crate secret_sys;

use secret::Passwords;
use secret::SchemaAttributeType::{Boolean, Integer};

fn main() {
    gtk::init().unwrap();

    let schema = new_schema!("org.example.Password", {
        number: Integer,
        //string: SchemaAttributeType::String, // FIXME: String type not working in the store! macro.
        even: Boolean,
    });
    let passwords = Passwords::new(schema);
    store!(passwords, "The label", "the password", |result| {
        println!("{:?}", result);
        gtk::main_quit();
    }, {
        number: 8,
        even: true,
    });

    gtk::main();

    lookup!(passwords, |password| {
        println!("{:?}", password);
        gtk::main_quit();
    }, {
        number: 8,
        even: true,
    });

    gtk::main();

    passwords.search(|items| {
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

    clear!(passwords, |result| {
        println!("{:?}", result);
        gtk::main_quit();
    }, {
        number: 8,
        even: true,
    });

    gtk::main();
}
