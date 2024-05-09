// lifetimes ensure that references are valid as long as we need them to be.

//preventing dangling references with lifrtimes
// The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference.

//orrow checker
// The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.

pub fn lifetimes_examples(){
    lifetimes_fn();
}

fn lifrtime_example() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |        x has the lifetime 'b, which in this case is larger than 'a. This means r can reference x because Rust knows that the reference in r will always be valid while x is valid.
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+


//generic lifetimes in functions

fn lifetimes_fn() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest2(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// lifetime syntax annotation
// lifetime annotation - they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
// names of lifetime parameters must start with an apostrophe (') and are usually all lowercase and very short, like generic types.
// We place lifetime parameter annotations after the & of a reference, using a space to separate the annotation from the reference’s type.
// exmaple 
//a reference to an i32 without a lifetime parameter, a reference to an i32 that has a lifetime parameter named 'a, and a mutable reference to an i32 that also has the lifetime 'a.

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime


// Lifetime Annotations in Function Signatures
//We want the signature to express the following constraint: the returned reference will be valid as long as both the parameters are valid. This is the relationship between lifetimes of the parameters and the return value. 
fn longest2<'a>(x: &'a str, y: &'a str)->&'a str{
    if x.len() > y.len() {
        x
    } else {                         // The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a. 
        y                            // The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a.
    }                                // it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments. 
}

fn lifetime_structs(){
    let novel = String::from("callme Weldon. Some time back ...");
    let first_sentence = novel.split('.').next().expect("could not find a .");
    let i = ImportantExcerpt{
        part: first_sentence,
    };
}

//lifetime Elision
//The 3 rules
// The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

// The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

// The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

/*
The signature starts without any lifetimes associated with the references:

fn first_word(s: &str) -> &str {
Then the compiler applies the first rule, which specifies that each parameter gets its own lifetime. We’ll call it 'a as usual, so now the signature is this:

fn first_word<'a>(s: &'a str) -> &str {
The second rule applies because there is exactly one input lifetime. The second rule specifies that the lifetime of the one input parameter gets assigned to the output lifetime, so the signature is now this:

fn first_word<'a>(s: &'a str) -> &'a str {
Now all the references in this function signature have lifetimes, and the compiler can continue its analysis without needing the programmer to annotate the lifetimes in this function signature.

Let’s look at another example, this time using the longest function that had no lifetime parameters when we started working with it in Listing 10-20:

fn longest(x: &str, y: &str) -> &str {
Let’s apply the first rule: each parameter gets its own lifetime. This time we have two parameters instead of one, so we have two lifetimes:

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
You can see that the second rule doesn’t apply because there is more than one input lifetime. The third rule doesn’t apply either, because longest is a function rather than a method, so none of the parameters are self

*/


//lifetimes annotations in structs definitions
//structs  to hold references

struct ImportantExcerpt<'a>{ //annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
    part: &'a str
}

// lifetime annotations in methods definitions

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// The Static Lifetime
// One special lifetime we need to discuss is 'static, which denotes that the affected reference can live for the entire duration of the program. All string literals have the 'static lifetime, which we can annotate as follows:

// let s: &'static str = "I have a static lifetime.";

//The text of this string is stored directly in the program’s binary, which is always available. Therefore, the lifetime of all string literals is 'static.


