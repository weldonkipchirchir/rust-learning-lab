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
