// A pointer is a general concept for a variable that contains an address in memory.
// Smart pointers, on the other hand, are data structures that act like a pointer but also have additional metadata and capabilities.

//Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart pointers implement the Deref and Drop traits. The Deref trait allows an instance of the smart pointer struct to behave like a reference so you can write your code to work with either references or smart pointers. The Drop trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope.

//examples of smart pointers
// reference counting smart pointer type. This pointer enables you to allow data to have multiple owners by keeping track of the number of owners and, when no owners remain, cleaning up the data.
//  while references only borrow data, in many cases, smart pointers own the data they point to.

// Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart pointers implement the Deref and Drop traits. The Deref trait allows an instance of the smart pointer struct to behave like a reference so you can write your code to work with either references or smart pointers.

pub fn smart_pointers() {
    store_in_heap();
    cons_list();
    custom_box();
    defer_coercion();
    drop_custom_smartpointer();
}

//Using Box<T> to Point to Data on the Heap

// Box<T> is a smart pointer that stores a value of type T on the heap rather than the stack. Box<T> is useful when you want to own a value on the heap but don’t want to take ownership of it.

/*
 You’ll use them most often in these situations:

When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
*/

//Using a Box<T> to Store Data on the Heap
//using a box to store i32 in heap

fn store_in_heap() {
    let b = Box::new(5);
    println!("b = {}", b);
    println!("b = {}", &b);
}

// Enabling Recursive Types with Boxes
// A value of recursive type can have another value of the same type as part of itself. Recursive types pose an issue because at compile time Rust needs to know how much space a type takes up.

//Cons List
// Each item in a cons list contains two elements: the value of the current item and the next item. The last item in the list contains only a value called Nil without a next item.
//(1, (2, (3, Nil)))

use crate::smart_pointers::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn cons_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let list2 = Cons(4, Box::new(list));
    println!("{:#? }", list2);
}

// treating smartpinter like regular references with defer trait
// Implementing the Deref trait allows you to customize the behavior of the dereference operator *

// using a Box<T> like a reference

/*
1.
    let x = 5;
    let y = &x;

2. Using Box<T> Like a Reference
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
*/

use std::ops::Deref;
// defining our own smart pointer
#[derive(Debug)]
struct MyBox<T>(T);

//The deref method gives the compiler the ability to take a value of any type that implements Deref and call the deref method to get a & reference that it knows how to dereference.
impl<T> Deref for MyBox<T> {
    type Target = T;
    //If the deref method returned the value directly instead of a reference to the value, the value would be moved out of self. We don’t want to take ownership of the inner value inside MyBox<T> in this case or in most cases where we use the dereference operator.
    fn deref(&self) -> &Self::Target {
        //We fill in the body of the deref method with &self.0 so deref returns a reference to the value we want to access with the * operator
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
fn custom_box() {
    let x = 5;
    let y = MyBox::new(x);
    println!("{:#?}", y);
    //Rust substitutes the * operator with a call to the deref method and then a plain dereference so we don’t have to think about whether or not we need to call the deref method.
    println!("{:#?}", *y);
    println!("{:#?}", *(y.deref()));
}

//implicit Defer coercions with function and methods
//Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type.
//For example, deref coercion can convert &String to &str because String implements the Deref trait such that it returns &str.

fn hello(name: &str) {
    println!("Hello, {name}!");
}

//Because we implemented the Deref trait on MyBox<T> in Listing 15-10, Rust can turn &MyBox<String> into &String by calling deref.
// Rust calls deref again to turn the &String into &str, which matches the hello function’s definition.
fn defer_coercion() {
    let m = MyBox::new(String::from("Rust"));
    let msg = "hello";
    hello(&m);
    //    hello(&(*m)[..]);
    //The (*m) dereferences the MyBox<String> into a String. Then the & and [..] take a string slice of the String that is equal to the whole string to match the signature of hello
}

/*
Rust does deref coercion when it finds types and trait implementations in three cases:
From &T to &U when T: Deref<Target=U>
From &mut T to &mut U when T: DerefMut<Target=U>
From &mut T to &U when T: Deref<Target=U>
*/

//running code on cleanup with drop trait
// The second trait important to the smart pointer pattern is Drop, which lets you customize what happens when a value is about to go out of scope.

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn drop_custom_smartpointer() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    //    let d = c;
    //    println!("CustomSmartPointers dropped before the end of the function.");

    //So, if we need to force a value to be cleaned up early, we use the std::mem::drop function.
    //The std::mem::drop function is different from the drop method in the Drop trait. We call it by passing as an argument the value we want to force drop.
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
    // Rust automatically called drop for us when our instances went out of scope, calling the code we specified.
}


