fn retrieve_number() -> i32 {
    return 8;
}
fn main() {
    let mut _x = 2;
    let mut count = 0;
    let _return_loop = loop {
        count += 1;
        if count == 5 {
            break 9;
        }
    };

    let _greeting = {
        let _s = 3;
        _s + 1
    };
    _x = if _x == 2 { 4 } else { 3 };
    println!("Hello, world from Rust!");
    println!("Xs:{0}, Greeting:{1}", _x, _greeting);
    println!("retrieve numbers {0}", retrieve_number());

    // Rust ownership
    //Borrowing
    let s1 = String::from("hello");
    let mut s2 = s1;

    hello(&mut s2);
    println!("Borrowing: {}, world", s2);
    //References
    println!("References: {1}{0}", get_greeting(), "holi ");
    //Slice
    let mut s = String::from("hello world");
    let world = first_word(&s);
    s.clear();

    println!("{}", world);
}

fn get_greeting() -> String {
    let name = String::from("Gabriel");
    name
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn hello(s: &mut String) {
    println!("Good {}", s);
    s.push_str("Holi");
}
