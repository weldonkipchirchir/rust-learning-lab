pub struct Post {
    state: Option<Box<dyn State>>, //state: An Option<Box<dyn State>> that represents the current state of the post.
    content: String,
}

//we’ll start by defining just the Draft state because that is the state we want a post to start in.

// When we create a new Post, we set its state field to a Some value that holds a Box. This Box points to a new instance of the Draft struct. This ensures whenever we create a new instance of Post, it will start out as a draft. Because the state field of Post is private, there is no way to create a Post in any other state! In the Post::new function, we set the content field to a new, empty String.
impl Post {
    //state: An Option<Box<dyn State>> that represents the current state of the post.
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})), //It initializes the state field with Some(Box::new(Draft {})), which means the initial state of a new post is Draft.
            content: String::new(),
        }
    }

    //The add_text method takes a mutable reference to self, because we’re changing the Post instance that we’re calling add_text on. We then call push_str on the String in content and pass the text argument to add to the saved content.
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text); //It calls the push_str method on self.content and passes the text argument to append the new text to the existing content.
    }

    //Even after we’ve called add_text and added some content to our post, we still want the content method to return an empty string slice because the post is still in the draft state,
    // Now we need to update the content method on Post. We want the value returned from content to depend on the current state of the Post, so we’re going to have the Post delegate to a content method defined on its state
    pub fn content(&self) -> &str {
        /*
        To get the content, it calls the content method on the current state object (self.state).
        Since state is an Option<Box<dyn State>>, it first calls as_ref() to get a reference to the Box<dyn State> inside the Option (Option<&Box<dyn State>>).
        It then calls unwrap() to get the &Box<dyn State> out of the Option.
        Finally, it calls the content method on the &Box<dyn State>, passing self as an argument. This will call the content method of the concrete state implementation that the Box is pointing to.
                */
        self.state.as_ref().unwrap().content(self)
    }

    //We give Post a public method named request_review that will take a mutable reference to self. Then we call an internal request_review method on the current state of Post, and this second request_review method consumes the current state and returns a new state.
    // To consume the old state, the request_review method needs to take ownership of the state value. This is where the Option in the state field of Post comes in: we call the take method to take the Some value out of the state field and leave a None in its place, because Rust doesn’t let us have unpopulated fields in structs. This lets us move the state value out of Post rather than borrowing it. Then we’ll set the post’s state value to the result of this operation.
    pub fn request_review(&mut self) {
        /*
        It uses an if let pattern to destructure the Option in self.state.
        if self.state is Some(s), it calls the take() method on self.state, which takes ownership of the Box<dyn State> and leaves None in its place.
        It then calls the request_review method on the Box<dyn State> (s), which transitions the state to the next appropriate state (e.g., from Draft to PendingReview).
        The result of s.request_review() is a new Box<dyn State> representing the new state.
        It then sets self.state to Some of this new Box<dyn State>.
         */
        if let Some(s) = self.state.take() {
            //We need to set state to None temporarily rather than setting it directly with code like self.state = self.state.request_review(); to get ownership of the state value. This ensures Post can’t use the old state value after we’ve transformed it into a new state.
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

//The State trait defines the behavior shared by different post states.
trait State {
    //request_review: This method takes ownership of self (self: Box<Self>), which means it consumes the current state object.
    //Both methods return a Box<dyn State>, which represents the new state after the transition.
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    // The request_review method on Draft returns a new, boxed instance of a new PendingReview struct, which represents the state when a post is waiting for a review.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    //Similar to the way request_review on PendingReview works, if we call the approve method on a Draft, it will have no effect because approve will return self.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
struct PendingReview {}

//We add the request_review method to the State trait; all types that implement the trait will now need to implement the request_review method. Note that rather than having self, &self, or &mut self as the first parameter of the method, we have self: Box<Self>. This syntax means the method is only valid when called on a Box holding the type.
// The request_review method on Draft returns a new, boxed instance of a new PendingReview struct, which represents the state when a post is waiting for a review. The PendingReview struct also implements the request_review method but doesn’t do any transformations. Rather, it returns itself, because when we request a review on a post already in the PendingReview state, it should stay in the PendingReview state.
impl State for PendingReview {
    // it returns itself, because when we request a review on a post already in the PendingReview state, it should stay in the PendingReview state.
    //This syntax takes ownership of Box<Self>, invalidating the old state so the state value of the Post can transform into a new state.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    //When we call approve on PendingReview, it returns a new, boxed instance of the Published struct. The Published struct implements the State trait, and for both the request_review method and the approve method, it returns itself, because the post should stay in the Published state in those cases.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}
//The state objects are Draft, PendingReview, and Published, and they will all implement the State trait.

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

pub fn post() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    post.request_review();

    post.approve();

    println!("{}", post.content());
}
