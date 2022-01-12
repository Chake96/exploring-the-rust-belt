
//the trait
pub trait Summary{
    fn summarize(&self)->String;
}


//using the trait
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn main(){
    let article = NewsArticle {
        headline: String::from("Boobies"),
        location: String::from("ATL, USA"),
        author: String::from("Carson"),
        content: String::from(
            "are great!",
        ),
    };
    println!("News: {}",article.summarize());
}