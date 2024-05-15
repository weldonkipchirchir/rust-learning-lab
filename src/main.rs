mod control_flow;
mod ownership;
mod references;
mod slices;
mod structs;
mod methods;
mod enumarations;
mod collections;
mod error_handling;
mod generics;
mod traits;
mod lifetime;
mod closures;
mod iterator;
mod smart_pointers;
mod smartpointers_counter_reference;
mod concurrency;
mod oop;
mod oop_blog_post;

use control_flow::print_string;
use control_flow::control_flow;
use control_flow::control_flow2;
use control_flow::control_flow3;
use control_flow::control_flow4;
use control_flow::while_loop;
use control_flow::for_loop;
use control_flow::for_loop_rev;
use ownership::ownership;
use ownership::return_ownership;
use references::reference;
use references::mutable_references;
use slices::slices;
use structs::my_struct;
use methods::methods;
use methods::point_method;
use enumarations::null_function;
use enumarations::lucky_coin;
use collections::collections;
use error_handling::errors_handling_rust;
use generics::generics;
use traits::traits_demo;
use lifetime::lifetimes_examples;
use closures::closures;
use iterator::iterators;
use smart_pointers::smart_pointers;
use smartpointers_counter_reference::smart_pointers_references;
use concurrency::concurrency;
use oop::opp;
use oop_blog_post::post;

fn main() {
    println!("Hello, world!");
    let str = print_string();
    println!("The string is {}", str);

    control_flow();
    control_flow2();
    control_flow3();
    control_flow4();
    while_loop();
    for_loop();
    for_loop_rev();

    //ownership
    ownership();
    return_ownership();

    //references
    reference();
    mutable_references();

    //slices
    slices();

    //structs
    my_struct();

    //methods
    methods();
    point_method();

    //enumerations
    null_function();
    lucky_coin();

    //collections
    collections();

    // error handling
    errors_handling_rust();

    //generics
    generics();

    //traits
    traits_demo();

    //lifetimes
    lifetimes_examples();

    //closures
    closures();

    //iterators
    iterators();

    //smart pointers
    smart_pointers();

    //smartpointer counter references
    smart_pointers_references();

    //concurrency
    concurrency();

    //opp
    opp();

    // opp blog post example
    post();
}

