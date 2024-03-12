// Developed by Andre Paiva (C) 2024
// Rust Book-Based Personal Exercises

use aggregator::{notify, NewsArticle, Summary, Tweet};

fn tweet() -> impl Summary {
    Tweet {
        username: String::from("xornotor"),
        content: String::from(
            "Programar é coisa de corno, no geral. Não programem. Vão tocar grama.",
        ),
        reply: false,
        retweet: false,
    }
}

fn news() -> impl Summary {
    NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    }
}

fn main() {
    notify(&tweet());
    notify(&news());
}
