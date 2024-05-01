enum IpAddrKind {
    v4,
    v6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn enum_function() {
    //instance of IpAddrKind
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    let home = IpAddr {
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::v6,
        address: String::from("::1"),
    };
}

enum SearchResults<T> {
    Found(T),
    NotFound,
}

fn find_element_index(elements: &[i32], target: i32) -> SearchResults<usize> {
    for (index, &element) in elements.iter().enumerate() {
        if element == target {
            return SearchResults::Found(index);
        }
    }
    SearchResults::NotFound
}

pub fn null_function() {
    enum_function();

    let numbers = vec![10, 20, 25, 30, 40, 50]; //creates a new Vec<T> (a growable array) with initial values
    let target = 25;
    match find_element_index(&numbers, target) {
        SearchResults::Found(index) => println!("The index of {} is: {}", target, index),
        SearchResults::NotFound => println!("{} not found in the list", target),
    }
}


//example coin mint
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Texas,
}

enum Coin {
    Penny, 
    Nickel,
    Dime, 
    Quater(UsState),
    Commemorative,
    Half_dollar,
    Dollar_coin,
}

fn value_in_cents(coin: Coin) -> u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime=> {
            println!("Lucky Penny!");
            1
        }
        Coin::Quater(state)=> {
            println!("State quater from {:?}!", state);
            25
        },
        _ => reroll(),
    }
}

//using if let instead of match

fn if_let(coin: Coin)-> u8{
    let mut count = 0;
    if let Coin::Quater(state) = coin {
        println!("State quater from {:?}!", state);
        count
    } else {
        count + 1
    }
}

pub fn lucky_coin(){
    let my_coin = Coin::Dime;
    let my_coin2 = Coin::Quater(UsState::Alabama);
    let my_coin3 = Coin::Half_dollar;
    let my_coin4 = Coin::Quater(UsState::Alabama);
    value_in_cents(my_coin);
    value_in_cents(my_coin2);
    value_in_cents(my_coin3);
    if_let(my_coin4);
}

fn reroll()-> u8{
    println!("Rerolling");
    78
}