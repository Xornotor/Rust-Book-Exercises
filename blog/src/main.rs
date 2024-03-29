use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.add_text("\nYesterday, I ate a hamburger");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    //post.add_text("\nTomorrow, I will eat pizza");
    //assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today\nYesterday, I ate a hamburger",
        post.content());

    //post.add_text("\nLast weekend I ate sushi");
    //assert_eq!("I ate a salad for lunch today\nYesterday, I ate a hamburger",
        //post.content());

    println!("{}", post.content());
}