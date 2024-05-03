// Unrecoverable Errors with panic!
//Rust has the panic! macro. There are two ways to cause a panic in practice: by taking an action that causes our code to panic (such as accessing an array past the end) or by explicitly calling the panic! macro.

//Recoverable Errors with Result

use std::error;
use std::fs::{read_to_string, File};
use std::io::{self, ErrorKind, Read};

// The return type of File::open is a Result<T, E>.
// Note that, like the Option enum, the Result enum and its variants have been brought into scope by the prelude, so we don’t need to specify Result:: before the Ok and Err variants in the match arms.'

pub fn errors_handling_rust() {
    recoverale_error();
    matching_on_different_errors();
    propagating_errors();
}

// there’s no file named hello.txt in our current directory and we run this code, we’ll see the following output from the panic!
fn recoverale_error() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn matching_on_different_errors() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() { 
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creatingd the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },

    };

    // Using match works well enough, but it can be a bit verbose and doesn’t always communicate intent well. The Result<T, E> type has many helper methods defined on it to do various, more specific tasks. The unwrap method is a shortcut method implemented just like the match expression we wrote in Listing 9-4. If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us. 
    let greeting = File::open("hello.txt").unwrap();
    // or
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}


fn propagating_errors(){
    let username = read_username_from_file();
    println!("{:?}", username);
    let username_two = read_username_from_file_shortcut();
    println!("{:?}", username_two);
    let result_chars = last_char_of_first_line("Subscribe to my channel\n more content added");
    println!("{:?}", result_chars);
}


fn read_username_from_file()-> Result<String, io::Error>{
    let username_file_result = File::open("Hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    // creates a new String in variable username and calls the read_to_string method on the file handle in username_file to read the contents of the file into username.
    let mut username = String::new();
    // However, we don’t need to explicitly say return, because this is the last expression in the function.
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shortcut()-> Result<String, io::Error>{
     //or

     let mut username = String::new();

     File::open("hello.txt")?.read_to_string(&mut username)?;
     Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}


fn creating_custom_types_for_validation(){

}