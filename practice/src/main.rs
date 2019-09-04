mod school;
extern crate chrono;
use std::collections::HashMap;
fn main() {
    let mut vector: Vec<u32> = Vec::new();
    let mut reference = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    vector.push(1);
    vector.push(2);
    vector.push(3);
    for v in vector.iter_mut() {
        *v += 2;
    }

    match reference.get_mut(1) {
        Some(s) => println!("{}", s),
        None => println!("There is not a number"),
    }

    let me = school::Person::create(26, String::from("Gabriel"), school::Day::Wednesday);

    let you = school::Person {
        name: String::from("Luisa"),
        ..me
    };

    let day = if let school::Day::Monday = you.born_day {
        5
    } else {
        0
    };
    println!("{:#?}", you);

    let mut x = 0;
    let response = loop {
        if x == 5 {
            break x;
        }
        x += 1;
    };

    println!("{}", response);

    let mut scores = HashMap::new();
    scores.insert("one", 2);
    let value = scores.entry("one").or_insert(3);
    println!("{}",value);
    match scores.get("one"){
        Some(x)=>println!("{}",x),
        None=>println!("none")
    }
}
