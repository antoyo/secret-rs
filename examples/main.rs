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

    clear!(passwords, |result| {
        println!("{:?}", result);
        gtk::main_quit();
    }, {
        number: 8,
        even: true,
    });

    gtk::main();
}
