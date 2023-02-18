// Traits: Defining Shared Behavior
// 특성은 특정 유형이 가지고 있고 다른 유형과 공유할 수 있는 기능을 정의합니다. 
// 특성을 사용하여 추상적인 방식으로 공유 동작을 정의할 수 있습니다
// --> 인터페이스와 유사

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

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
