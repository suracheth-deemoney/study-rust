pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct SocailPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocailPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
