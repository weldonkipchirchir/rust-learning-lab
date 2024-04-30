pub fn slices() {
    let s = String::from("Hello this rust programming language");
    let len = first_word(&s);
    println!("Length is {}", len);

    string_slices();

    let my_string = "Hello, world!";
    print_first_word(my_string);
}
fn first_word(s: &str) -> &str {
    //convert to bytes [72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]
    let bytes = s.as_bytes(); //convert string to an array of bytes
    /*
    The iter() method returns an iterator over the elements of the array.
    The enumerate() method adapts an iterator into an iterator that yields tuples (index, value), where index is the index of the element in the original collection and value is the reference to the element itself.
    In Rust, the b' ' syntax is used to represent a byte literal. It denotes the ASCII value of the space character, which is 32.
    */
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '  {
            return &s[..i];
        }
    }
    s
}


//string slices
fn string_slices(){
    let s = String::from("Hello world");
    let hello = &s[0..5];
    println!("hello = {}",hello);
    let world = &s[6..11];
    println!("world = {}",world);
}

fn print_first_word(s: &str) {
    let first_space = s.find(' ').unwrap_or(s.len()); // Find the index of the first space or use the length of the string if no space is found
    let word = &s[..first_space]; // Take a substring from the start of the string up to the first space
    println!("The first word is: {}", word);
}