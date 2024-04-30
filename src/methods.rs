pub fn methods() {
    let area: u32 = 800;
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 40,
        height: 60,
    };

    if rect1.area(area) {
        println!("the rectangle has an area greater than {}", area);
    }
    println!("the rectangle has an area less than {}", area);

    println!("Can rect1 hold react2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold react3? {}", rect1.can_hold(&rect3));

    println!("associated functions");
    let name = String::from("Weldon");
    let age = 25;
    let details = Details::personal_details(name, age);
    dbg!(details);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self, area: u32) -> bool {
        let rectangle_area = self.width * self.height;
        rectangle_area > area
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        let result = self.width > other.width && self.height > other.height;
        result
    }
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let x_squared = f64::powi(other.x - self.x, 2); //convert float to int of power 2
        let y_squared = f64::powi(other.y - self.y, 2);
        let result = f64::sqrt(x_squared + y_squared);
        return result;
    }
}

pub fn point_method() {
    let p1 = Point { x: 3.6, y: 8.3 };
    let p2 = Point { x: 6.4, y: 4.9 };
    let sqrt_result = p1.distance(&p2);
    println!("square root {}", sqrt_result)
}

//assoociated functions
#[derive(Debug)]
struct Details {
    name: String,
    age: u32,
}

impl Details {
    fn personal_details(name: String, age: u32) -> Self {
        Self { name, age }
    }
}
