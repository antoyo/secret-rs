/*
 * Copyright (c) 2016 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

extern crate gtk;
#[macro_use]
extern crate secret;

use secret::{Collection, Schema, Service};
use secret::SchemaAttributeType::{self, Boolean, Integer};

fn main() {
    gtk::init().unwrap();

    let attribute_types = hash! {
        number => Integer,
        string => SchemaAttributeType::String,
        even => Boolean,
    };

    let schema = Schema::new("org.example.Password", attribute_types);

    Collection::create("Test collection", move |collection| {
        println!("{:?}", collection);
        gtk::main_quit();
    });

    gtk::main();

    Service::get(move |service| {
        gtk::main_quit();

        let service = service.unwrap();
        service.load_collections(|result| {
            println!("{:?}", result);
            gtk::main_quit();
        });

        gtk::main();

        let collections = service.get_collections();
        for collection in collections {
            println!("{:?}", collection.get_label());

            if collection.get_label() == Some("Test collection".to_string()) {
                collection.item_create(&schema, "Label 2", "Pass2", &str_hash! {
                    number => 8,
                    string => "huit",
                    even => true,
                }, |result| {
                    println!("Second: {:?}", result);
                    gtk::main_quit();
                });

                gtk::main();

                collection.search(&schema, &str_hash! {
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
                        item.delete(|result| {
                            println!("Item deleted: {:?}", result);
                        });
                    }
                    gtk::main_quit();
                });

                gtk::main();

                collection.delete(|result| {
                    println!("Deleted: {:?}", result);
                    gtk::main_quit();
                });
                gtk::main();
            }
        }

        gtk::main_quit();
    });

    gtk::main();
}
