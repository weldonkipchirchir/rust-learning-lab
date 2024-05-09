// We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types

pub fn generics() {
    main_generics();
    struct_generics();
    struct_generics_methods();
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    println!("The address of index 0 is {}", largest);
    println!("The address of index 0 is {}", largest);

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main_generics() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let mut num = 5;
    println!("The value of num is {}", num);
    let num2 = &num;
    println!("The value of num is {}", num2);

    let z = 10;
    let raw_ptr_to_z: *const i32 = &z; // Immutable raw pointer to z
    let value_at_ptr = unsafe { *raw_ptr_to_z }; // Dereferencing inside unsafe block
    println!("Value at raw pointer: {}", value_at_ptr);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest_char(&char_list);
    // println!("The largest char is {}", result);
}

struct Point_two<T, U>{
    x:T,
    y:U
}


enum Option<T>{
    Some(T),
    None
}

enum Result<T, E>{
    Ok(T),
    Err(E)
}


struct Point<T>{
    x:T,
    y:T
}

// we have to declare T just after impl so we can use T to specify that we’re implementing methods on the type Point<T>
// By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.
impl<T> Point<T>  {
    fn x(&self) -> &T{
        &self.x
    }
}

fn struct_generics(){
    let interger =Point{x:5, y:10};
    let float = Point{x:3.4, y:8.3};
    println!("p.x = {}", interger.x());

    let both_integer = Point_two { x: 5, y: 10 };
    let both_float = Point_two { x: 1.0, y: 4.0 };
    let integer_and_float = Point_two { x: 5, y: 4.0 };

}

struct PointThree<X1, Y1>{
    x: X1,
    y: Y1
}

impl<X1, Y1> PointThree<X1, Y1> {
    // <X2, Y2>. These are independent of the type parameters of the struct. They represent the types of the x and y fields of the other parameter.
    fn mixup<X2, Y2> (self, other: PointThree<X2, Y2>)->PointThree<X1, Y2>{
        PointThree{
            x: self.x,
            y: other.y
        }
    }
}

fn struct_generics_methods(){
    let p1 = PointThree{x:5, y:4.5};
    let p2 = PointThree{x:"Hello", y:'r'};

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


