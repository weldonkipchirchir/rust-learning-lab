// closures: anonymous functions that capture their environment
// rust closures are anonymous fucntions that you can save in a variable or pass as arguments to other functions
// Unlike functions, closures can capture values from the scope in which they’re defined. 

pub fn closures(){
    giveaway_shirt();
    variable_closures();
    move_ownership();
    closure_traits();
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum SHirtColor{
    Red,
    Blue
}

struct Inventory{
    shirts: Vec<SHirtColor>
}
/*
we get the user preference as a parameter of type Option<ShirtColor> and call the unwrap_or_else method on user_preference. The unwrap_or_else method on Option<T> is defined by the standard library. It takes one argument: a closure without any arguments that returns a value T (the same type stored in the Some variant of the Option<T>, in this case ShirtColor). 
If the Option<T> is the Some variant, unwrap_or_else returns the value from within the Some. If the Option<T> is the None variant, unwrap_or_else calls the closure and returns the value returned by the closure.
This is a closure that takes no parameters itself (if the closure had parameters, they would appear between the two vertical bars). 
*/
impl Inventory{
    fn giveaway(&self, user_preference: Option<SHirtColor>)->SHirtColor{
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self)-> SHirtColor{
        let mut num_red =0;
        let mut num_blue =0;

        for color in &self.shirts{
            match color{
                SHirtColor::Red => num_red+=1,
                SHirtColor::Blue => num_blue+=1,
            }
        }

        if num_red> num_blue{
            SHirtColor::Red
        }else{
            SHirtColor::Blue
        }
    }
}

fn giveaway_shirt(){
    let store = Inventory{
        shirts: vec![SHirtColor::Blue, SHirtColor::Red, SHirtColor::Blue],
    };
    let user_pref1 = Some(SHirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}


// Closures don’t usually require you to annotate the types of the parameters or the return value like fn functions do. 
/*
Type annotations are required on functions because the types are part of an explicit interface exposed to your users. Defining this interface rigidly is important for ensuring that everyone agrees on what types of values a function uses and returns. 
Closures, on the other hand, aren’t used in an exposed interface like this: they’re stored in variables and used without naming them and exposing them to users of our library.
*/

//example of closure
use std::thread;
use std::time::Duration;
fn generate_workout(intensity: i32, random_number: i32){
    //cliosure stored in a variable
    let expensive_closure = |num: i32| -> i32{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity <25{
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} pushups!", expensive_closure(intensity));
    } else{
        if random_number == 3{
            println!("Take a break today! Remember to stay hydrated!");
        }
        else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
 }

 fn variable_closures() {
    let simulated_user_specified_value = 100;
    let simulated_random_number = 3;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// capturing references or moving ownership


// moving ownership

fn move_ownership(){
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list); 

    thread::spawn(move || println!("From thread: {:?}", list))
    .join()
    .unwrap()
}

//moving captures values out of closures and the Fn traits

/*
FnOnce applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
FnMut applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
Fn applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.
*/
#[derive(Debug)]

struct Rectangle{
    width: u32,
    height: u32,
}

fn closure_traits(){
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list)
}
