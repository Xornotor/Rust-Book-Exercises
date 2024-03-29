use blog_idiomatic::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let mut post = post.reject();

    post.add_text("\nYesterday, I ate hamburger");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today\nYesterday, I ate hamburger",
        post.content());

    println!("{}", post.content());
}