// match arms

pub fn patter_matching() {
    favorite_color();
    while_loops();
    for_loops();
    function_patterns();
    matching_named_variable();
    multiple_patterns();
    match_ranges();
    char_range();
    destructure_struct();
    destructure_enum();
    nested_structs_enums();
    ignoring_parts_of_a_value();
    bindings();
    ignoring_values();
    extra_conditionals_with_match_guards();
    ignoring_remaining_parts_of_a_value();
}

// match VALUE {
//     PATTERN => EXPRESSION,
//     PATTERN => EXPRESSION,
//     PATTERN => EXPRESSION,
// }

/*
The patterns in this match expression are the None and Some(i) on the left of each arrow.

One requirement for match expressions is that they need to be exhaustive in the sense that all possibilities for the value in the match expression must be accounted for.

The particular pattern _ will match anything, but it never binds to a variable, so it’s often used in the last match arm. The _ pattern can be useful when you want to ignore any value not specified
*/

// conditional if let expressions

// use if let expressions mainly as a shorter way to write the equivalent of a match that only matches one case. Optionally, if let can have a corresponding else containing code to run if the pattern in the if let doesn’t match.

fn favorite_color() {
    let favorite_color: Option<&str> = None;

    let is_tuesday = false;

    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

//while let conditional loops

//Similar in construction to if let, the while let conditional loop allows a while loop to run for as long as a pattern continues to match.
fn while_loops() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    /*
    This example prints 3, 2, and then 1. The pop method takes the last element out of the vector and returns Some(value). If the vector is empty, pop returns None. The while loop continues running the code in its block as long as pop returns Some. When pop returns None, the loop stops. We can use while let to pop every element off our stack.
     */
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

// for loops
//In a for loop, the value that directly follows the keyword for is a pattern. For example, in for x in y the x is the pattern.

fn for_loops() {
    let a = [10, 20, 30, 40, 50];

    //We adapt an iterator using the enumerate method so it produces a value and the index for that value, placed into a tuple.
    for (index, value) in a.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // Rust compares the value (1, 2, 3) to the pattern (x, y, z) and sees that the value matches the pattern, so Rust binds 1 to x, 2 to y, and 3 to z. You can think of this tuple pattern as nesting three individual variable patterns inside it.
    let (x, y, z) = (1, 2, 3);
}

// Function Parameters
// Function parameters can also be patterns

fn foo(x: i32) {
    // code goes here
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn function_patterns() {
    let point = (3, 5);
    print_coordinates(&point);
}
//This code prints Current location: (3, 5). The values &(3, 5) match the pattern &(x, y), so x is the value 3 and y is the value 5.

/*
Refutability: Whether a Pattern Might Fail to Match
Patterns come in two forms: refutable and irrefutable. Patterns that will match for any possible value passed are irrefutable. An example would be x in the statement let x = 5; because x matches anything and therefore cannot fail to match. Patterns that can fail to match for some possible value are refutable. An example would be Some(x) in the expression if let Some(x) = a_value because if the value in the a_value variable is None rather than Some, the Some(x) pattern will not match.

*/

// Function parameters, let statements, and for loops can only accept irrefutable patterns, because the program cannot do anything meaningful when values don’t match.

//The if let and while let expressions accept refutable and irrefutable patterns, but the compiler warns against irrefutable patterns because by definition they’re intended to handle possible failure: the functionality of a conditional is in its ability to perform differently depending on success or failure.

//pattern syntax

// Matching Literals

/*
Example

    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

*/

/*
Matching Named Variables
// Named variables are irrefutable patterns that match any value, and we’ve used them many times in the book. However, there is a complication when you use named variables in match expressions. Because match starts a new scope, variables declared as part of a pattern inside the match expression will shadow those with the same name outside the match construct, as is the case with all variables.
*/

fn matching_named_variable() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);
}

/*
The pattern in the first match arm doesn’t match the defined value of x, so the code continues.

The pattern in the second match arm introduces a new variable named y that will match any value inside a Some value. Because we’re in a new scope inside the match expression, this is a new y variable, not the y we declared at the beginning with the value 10. This new y binding will match any value inside a Some, which is what we have in x. Therefore, this new y binds to the inner value of the Some in x. That value is 5, so the expression for that arm executes and prints Matched, y = 5.
*/

//Multiple Patterns

// In match expressions, you can match multiple patterns using the | syntax, which is the pattern or operator.

fn multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

// Matching Ranges of Values with ..=
// The ..= syntax allows us to match to an inclusive range of values.

fn match_ranges() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}

//The compiler checks that the range isn’t empty at compile time, and because the only types for which Rust can tell if a range is empty or not are char and numeric values, ranges are only allowed with numeric or char values.

fn char_range() {
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

// Destructuring to Break Apart Values

//We can also use patterns to destructure structs, enums, and tuples to use different parts of these values.

//destructuring structs

struct Point {
    x: i32,
    y: i32,
}

fn destructure_struct() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    //or
    let Point { x, y } = p;

    print!("{}", a);
    print!("{}", b);

    //we have a match expression that separates Point values into three cases: points that lie directly on the x axis (which is true when y = 0), on the y axis (x = 0), or neither.

    match p {
        Point { x, y: 0 } => println!("on the x axis at {}", x),
        Point { x: 0, y } => println!("on the y axis at {}", y),
        Point { x, y } => println!("somewhere else at ({}, {})", x, y),
    }

    /*
    The first arm will match any point that lies on the x axis by specifying that the y field matches if its value matches the literal 0. The pattern still creates an x variable that we can use in the code for this arm.

    Similarly, the second arm matches any point on the y axis by specifying that the x field matches if its value is 0 and creates a variable y for the value of the y field. The third arm doesn’t specify any literals, so it matches any other Point and creates variables for both the x and y fields.
     */
}

//destructuring enums

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destructure_enum() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
    }
}

//matching nested structs and enums

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message1 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn nested_structs_enums() {
    let msg = Message1::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message1::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message1::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
}

//destructuring structs and tuples

// let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

// ignoring values in a pattern

// Ignoring an Entire Value with _

fn boo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
fn ignoring_values() {
    boo(3, 5);
}

// Ignoring Parts of a Value with a Nested _

/*
 This code will print Can't overwrite an existing customized value and then setting is Some(5). In the first match arm, we don’t need to match on or use the values inside either Some variant, but we do need to test for the case when setting_value and new_setting_value are the Some variant. In that case, we print the reason for not changing setting_value, and it doesn’t get changed.

In all other cases (if either setting_value or new_setting_value are None) expressed by the _ pattern in the second arm, we want to allow new_setting_value to become setting_value.
 */
fn ignoring_parts_of_a_value() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
}

//ignoring unused variable by starting its Name with _

fn unused_variable() {
    //The syntax _x still binds the value to the variable, whereas _ doesn’t bind at all.
    let _x = 5;
}

// Ignoring Remaining Parts of a Value with ..

fn ignoring_remaining_parts_of_a_value() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}

// Extra Conditionals with Match Guards

fn extra_conditionals_with_match_guards() {
    // A match guard is an additional if condition, specified after the pattern in a match arm, that must also match for that arm to be chosen. Match guards are useful for expressing more complex ideas than a pattern alone allows.
    // The condition can use variables created in the pattern.
    // The downside of this additional expressiveness is that the compiler doesn't try to check for exhaustiveness when match guard expressions are involved.
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    /*
    The match guard if n == y is not a pattern and therefore doesn’t introduce new variables. This y is the outer y rather than a new shadowed y, and we can look for a value that has the same value as the outer y by comparing n to y.

    You can also use the or operator | in a match guard to specify multiple patterns; the match guard condition will apply to all the patterns. Listing 18-28 shows the precedence when combining a pattern that uses | with a match guard. The important part of this example is that the if y match guard applies to 4, 5, and 6, even though it might look like if y only applies to 6.
         */

    let z = 4;
    let y = false;

    match z {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

// @ bindings

// The at operator @ lets us create a variable that holds a value at the same time as we’re testing that value for a pattern match.

enum Message3 {
    Hello { id: i32 },
}

fn bindings() {
    let msg = Message3::Hello { id: 5 };

    match msg {
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message3::Hello { id } => println!("Found some other id: {}", id),
    }
}

/*
This example will print Found an id in range: 5. By specifying id_variable @ before the range 3..=7, we’re capturing whatever value matched the range while also testing that the value matched the range pattern.

In the second arm, where we only have a range specified in the pattern, the code associated with the arm doesn’t have a variable that contains the actual value of the id field. The id field’s value could have been 10, 11, or 12, but the code that goes with that pattern doesn’t know which it is. The pattern code isn’t able to use the value from the id field, because we haven’t saved the id value in a variable.

In the last arm, where we’ve specified a variable without a range, we do have the value available to use in the arm’s code in a variable named id. The reason is that we’ve used the struct field shorthand syntax. But we haven’t applied any test to the value in the id field in this arm, as we did with the first two arms: any value would match this pattern.

Using @ lets us test a value and save it in a variable within one pattern.
*/
