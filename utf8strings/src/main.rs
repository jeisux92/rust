fn main() {
    let mut s = String::new();
    let s = String::from("initial contents");

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let mut s = 32.to_string();
    s.push_str("sa");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s2 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s2 is {}", s);

    let s1 = String::from("hello");
    //let h = &s1[0];

    for c in "नमस्ते".as_bytes() {
        println!("{}", c);
    }
}
