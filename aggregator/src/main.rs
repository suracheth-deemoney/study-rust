use aggregator::{NewArticle, SocailPost, Summary};
fn main() {
    let post = SocailPost {
        username: String::from("horses_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());

    let article = NewArticle {
        headline: String::from("Penguins with the Stanley Cup championship!"),
        location: String::from("Pittburgh, PA, ,USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittburgh Penguins once again are the best \
        hockey team i the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}
