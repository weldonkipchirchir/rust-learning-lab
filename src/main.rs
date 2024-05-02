mod control_flow;
mod ownership;
mod references;
mod slices;
mod structs;
mod methods;
mod enumarations;
mod collections;

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

}

