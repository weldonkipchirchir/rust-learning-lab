//A trait defines functionality a particular type has and can share with other types.
// We can use traits to define shared behavior in an abstract way.

pub fn traits_demo() {
    println!("Traits Demo");
    aggregator();
    default_implementaon();
    return_traits();
}

// implementing a trait on a type

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// implementation of the Summary trait on the NewsArticle struct
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn aggregator() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

//One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate.

//default implementations

trait Summarizable {
    fn summarize_author(&self )-> String;
    fn summarize(&self) -> String {
        // String::from("(Read more...)")
        format!("Read more from {}", self.summarize_author())
    }
}

struct Article {
    headline: String,
}

impl Summarizable for Article {
    fn summarize_author(&self )-> String {
        format!("{}", self.headline)
    }
}

fn default_implementaon() {
    let article = Article {
        headline: String::from("Penguins win the Stanley Cup Championship!")
    };
    println!("New article available! {}", article.summarize());
}

// function to implement default trait


//traits as parameters

//This parameter accepts any type that implements the specified trait.
fn notify(item: &impl Summarizable){
    println!("Breaking news! {}", item.summarize())
}

// returning types that implemenents traits
fn returns_summarizable() -> impl Summarizable{ //returns some type that implements the Summary trait without naming the concrete type. 
    Article{ //you can only use impl Trait if you’re returning a single type. 
        headline: String::from("Hello summary"),
    }
}

fn return_traits(){
    let article = returns_summarizable();
    notify(&article);
}