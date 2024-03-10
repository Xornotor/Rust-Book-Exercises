fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn last_char_of_second_line(text: &str) -> Option<char> {
    let mut iterator = text.lines();
    iterator.next()?;
    iterator.next()?.chars().last()
}

fn main() {
    let teste = "My\nOwn\nGoodness";

    println!("{}", last_char_of_first_line(&teste).unwrap());
    println!("{}", last_char_of_second_line(&teste).unwrap());
}