trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn add_text<'a>(&self, _text: &'a str) -> &'a str {
        ""
    }
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn add_text<'a>(&self, text: &'a str) -> &'a str {
        text
    }
}

struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingPublish {})
    }
}

struct PendingPublish {}
impl State for PendingPublish {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}
impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(self.state.as_ref().unwrap().add_text(text));
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}
