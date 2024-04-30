pub fn reference(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String)->usize{
    s.len()
}

//mutable references
pub fn mutable_references(){
    let mut s =String::from("hello");
    change(&mut s);
    println!("Mutable reference {}: ", s)
}

fn change(some_string: &mut String){
    some_string.push_str(", world");
}