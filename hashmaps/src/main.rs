use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, &field_value);

    println!("{}", field_value);
    let score = match scores.get("Blue") {
        Some(x) => *x,
        None => 0,
    };
    // println!("score {}",score.is_some());
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    map.insert(String::from("One"), 45);

    let count = map.entry(String::from("One")).or_insert(0);
    *count+=2;
    println!("{}", count);
    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }

    println!("{:?}", map);
}
