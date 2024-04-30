pub fn ownership(){
    let s = String::from("Hello");
    take_ownership(s);

    let x = 5;

    makes_copy(x);
}

fn take_ownership(some_string : String){ //some_string comes into scope
    println!{"{}", some_string}
} //here some_string goes out of scope and 'drop' is called. The backing memory is freed

fn makes_copy(some_integer:i32){ //some integer comes into scope 
    println!("{}", some_integer)
} // here some_integer goes out of scope

pub fn return_ownership(){
    let _s1 = gives_ownership(); //gives_ownership() moves its return value into s1
    let s2 = String::from("hello");// s2 comes into scope
     let _s3 = take_and_gives_back(s2); //s2 is moved takes_and_give_back which is also moves its returned value into s3
}

fn gives_ownership()-> String{ // gives_ownership will move its return  value into the function that calls it
    let some_string = String::from("yours"); //some_string comes into scope
    some_string //some_string is returned and moves out to the calling function
}

fn take_and_gives_back(a_string: String) ->String{ //a_string comes into scope
    a_string //a_string is returned and moves out to the calling function
}