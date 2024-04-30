struct User{
    active:bool,
    user_name: String,
    email:String,
    sign_in_count: u32,
}

pub fn my_struct() {
    let mut user1 = User {
        active: true,
        user_name: String::from("weldon"),
        email: "weldon@gmail.com".to_string(), // Convert the string literal to a String
        sign_in_count: 1,
    };

    println!("username is {}", user1.user_name);
    
    user1.user_name = String::from("Weldonmylene");
    
    println!("username is {}", user1.user_name);

    let user2  = User{
        sign_in_count:2,
        ..user1
    };

    println!("{}", user2.user_name);
    println!("{}", user2.active);
    println!("{}", user2.email);
    println!("{}", user2.sign_in_count);
}


//tuple struct 

struct Color(i32, i32, i32);
struct Paint(i32, i32, i32);

fn tuple_structs(){
    let black = Color(0,0,0);
    let origin: Paint = Paint(0,0,0);
}