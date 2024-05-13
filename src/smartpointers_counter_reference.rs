// Rc<T>, the Reference Counted Smart Pointer
/*
there are cases when a single value might have multiple owners. For example, in graph data structures, multiple edges might point to the same node, and that node is conceptually owned by all of the edges that point to it. A node shouldn’t be cleaned up unless it doesn’t have any edges pointing to it and so has no owners.
*/

//Note that Rc<T> is only for use in single-threaded scenarios.

pub fn smart_pointers_references() {
    rc_smart_pointer();
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::smartpointers_counter_reference::List::{Cons, Nil};

use std::rc::Rc;
fn rc_smart_pointer() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a)); // Cloning an Rc<T> Increases the Reference Count
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a)); // Rc::clone doesn’t make a deep copy of all the data like most types’ implementations of clone do.
        println!("count after creating c = {}", Rc::strong_count(&a));
        println!("c is {:?}", c);
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// RefCell<T> and the Interior Mutability Pattern
// Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; normally, this action is disallowed by the borrowing rules.
//To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing. Unsafe code indicates to the compiler that we’re checking the rules manually instead of relying on the compiler to check them for us;

//Enforcing Borrowing Rules at Runtime with RefCell<T>
// Unlike Rc<T>, the RefCell<T> type represents single ownership over the data it holds.
// At any given time, you can have either (but not both) one mutable reference or any number of immutable references.
// RefCell<T> is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context.

/*
reasons to choose Box<T>, Rc<T>, or RefCell<T>:

Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
Mutating the value inside an immutable value is the interior mutability pattern.
*/

//Interior Mutability: A Mutable Borrow to an Immutable Value

// A consequence of the borrowing rules is that when you have an immutable value, you can’t borrow it mutably.

//  there are situations in which it would be useful for a value to mutate itself in its methods but appear immutable to other code. Code outside the value’s methods would not be able to mutate the value.

/*
Using RefCell<T> is one way to get the ability to have interior mutability, but RefCell<T> doesn’t get around the borrowing rules completely: the borrow checker in the compiler allows this interior mutability, and the borrowing rules are checked at runtime instead. If you violate the rules, you’ll get a panic! instead of a compiler error.
*/
// A Use Case for Interior Mutability: Mock Objects

// library that tracks a value against a maximum value and sends messages based on how close to the maximum value the current value is. This library could be used to keep track of a user’s quota for the number of API calls they’re allowed to make, for example.

trait Messenger {
    fn send(&self, msg: &str);
}

struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
} 

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        } 
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    } 
  
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}