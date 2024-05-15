// Object-oriented programs are made up of objects. An object packages both data and the procedures that operate on that data. The procedures are typically called methods or operations.

// Using this definition, Rust is object-oriented: structs and enums have data, and impl blocks provide methods on structs and enums. Even though structs and enums with methods aren’t called objects, they provide the same functionality, according to the Gang of Four’s definition of objects.

// Encapsulation that Hides Implementation Details


//We discussed how to control encapsulation in Chapter 7: we can use the pub keyword to decide which modules, types, functions, and methods in our code should be public, and by default everything else is private. For example, we can define a struct AveragedCollection that has a field containing a vector of i32 values.

pub fn opp(){
    draw_objects()
}

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

//The struct is marked pub so that other code can use it, but the fields within the struct remain private.

impl AveragedCollection{
    pub fn add(&mut self, value: i32){
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32>{
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None
        }
    }
    pub fn average(&self) -> f64{
        self.average
    }
    fn update_average(&mut self){
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

//The public methods add, remove, and average are the only ways to access or modify data in an instance of AveragedCollection.

// When an item is added to list using the add method or removed using the remove method, the implementations of each call the private update_average method that handles updating the average field as well.

//We leave the list and average fields private so there is no way for external code to add or remove items to or from the list field directly; otherwise, the average field might become out of sync when the list changes. The average method returns the value in the average field, allowing external code to read the average but not modify it.


// Inheritance as a Type System and as Code Sharing

//Inheritance is a mechanism whereby an object can inherit elements from another object’s definition, thus gaining the parent object’s data and behavior without you having to define them again.

//If a language must have inheritance to be an object-oriented language, then Rust is not one. There is no way to define a struct that inherits the parent struct’s fields and method implementations without using a macro.

//You can do this in a limited way in Rust code using default trait method implementations,



// Using Trait Objects That Allow for Values of Different Types

// Defining a Trait for Common Behavior

//To implement the behavior we want gui to have, we’ll define a trait named Draw that will have one method named draw. Then we can define a vector that takes a trait object. A trait object points to both an instance of a type implementing our specified trait and a table used to look up trait methods on that type at runtime.

//We create a trait object by specifying some sort of pointer, such as a & reference or a Box<T> smart pointer, then the dyn keyword, and then specifying the relevant trait.

// in Rust, we refrain from calling structs and enums “objects” to distinguish them from other languages’ objects. In a struct or enum, the data in the struct fields and the behavior in impl blocks are separated, whereas in other languages, the data and behavior combined into one concept is often labeled an object. However, trait objects are more like objects in other languages in the sense that they combine data and behavior. But trait objects differ from traditional objects in that we can’t add data to a trait object. Trait objects aren’t as generally useful as objects in other languages: their specific purpose is to allow abstraction across common behavior.

//A trait object in Rust is a way to represent a trait as an opaque type that can be used to store values of different types that implement that trait. It allows you to work with instances of types that implement a specific trait without knowing the concrete types at compile-time.
pub trait Draw{
    fn draw(&self);
}

pub struct screen{
    pub components: Vec<Box<dyn Draw>>, // vector named components. This vector is of type Box<dyn Draw>, which is a trait object; it’s a stand-in for any type inside a Box that implements the Draw trait.
}
//dyn Draw is a trait object that represents the Draw trait. It's a way to say "any type that implements the Draw trait."
// Box<dyn Draw> is a boxed trait object. The Box part means that the trait object is allocated on the heap, and the value is a pointer to that heap-allocated data. This is necessary because trait objects have an unknown size at compile-time, so they cannot be stored on the stack.
// The Vec<Box<dyn Draw>> part declares a vector that can hold boxed trait objects implementing the Draw trait. This allows you to store instances of different types (e.g., Circle, Rectangle, Triangle) that implement the Draw trait in the same vector. When you want to call a method from the Draw trait on an element of the vector, you can do so without knowing the concrete type of the element, as long as it implements the Draw trait.



//On the Screen struct, we’ll define a method named run that will call the draw method on each of its components
impl screen{
    pub fn run(&self){ //The run method is defined on the Screen struct. It iterates over the components vector and calls the draw method on each component. This works because all components in the vector implement the Draw trait.
        for component in self.components.iter(){
            component.draw();
        }
    }
}

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
//This restricts us to a Screen instance that has a list of components all of type Button or all of type TextField. 


// implementing trait


pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

//The Button struct represents a button component with fields for width, height, and label. It implements the Draw trait by providing an implementation for the draw method.
impl Draw for Button {
    fn draw(&self) {
        println!("Draw in Button");
    }
}

//The width, height, and label fields on Button will differ from the fields on other components; for example, a TextField type might have those same fields plus a placeholder field. Each of the types we want to draw on the screen will implement the Draw trait but will use different code in the draw method to define how to draw that particular type, as Button has here (without the actual GUI code, as mentioned).



struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Draw in SelectBox");
    }
}


//Our library’s user can now write their main function to create a Screen instance. To the Screen instance, they can add a SelectBox and a Button by putting each in a Box<T> to become a trait object. They can then call the run method on the Screen instance, which will call draw on each of the components.

fn draw_objects() {
    let screen = Screen {
        components: vec![
            SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            },
        ],
    };
    // When you call screen.run(), it will invoke the draw method on each component in the components vector.
    screen.run();
}




// Implementing an Object-Oriented Design Pattern
 // The state pattern is an object-oriented design pattern. The crux of the pattern is that we define a set of states a value can have internally. The states are represented by a set of state objects, and the value’s behavior changes based on its state. 

// in Rust, of course, we use structs and traits rather than objects and inheritance. Each state object is responsible for its own behavior and for governing when it should change into another state. The value that holds a state object knows nothing about the different behavior of the states or when to transition between states.

/*
A blog post starts as an empty draft.
When the draft is done, a review of the post is requested.
When the post is approved, it gets published.
Only published blog posts return content to print, so unapproved posts can’t accidentally be published.
Any other changes attempted on a post should have no effect. For example, if we try to approve a draft blog post before we’ve requested a review, the post should remain an unpublished draft.
*/