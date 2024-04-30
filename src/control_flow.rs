pub fn print_string() -> String {
    let s = String::from("Helllo");
    s
}

pub fn control_flow() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
}

pub fn control_flow2() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of the number is: {number}");
}

pub fn control_flow3() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

pub fn control_flow4(){
    let mut count =0;
    'counting_up: loop {
        println!("Count = {count}")   ;
        let mut remaining= 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining ==9{
                break;
            }
            if count ==2{
                break 'counting_up;
            }
            remaining -=1;
        }
        count+=1;
    }
    println!("End count = {count}")

}

pub fn while_loop(){
    let condition = true;
    let mut i = 0;
    while condition {
        println!("Condition is true");
        if i>10{
            println!("End of the loop");
            break;
        }
        i += 1;
    }
}

pub fn for_loop(){
    let a = [2,3,5,6,23,5,77,32];
    for element in a{
        println!("This is the value: {element}");
    }
}

pub fn for_loop_rev(){
    for number in (1..5).rev(){
        println!("{number}");
    }
    println!("LIFTOFF!!!")
}