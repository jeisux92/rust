fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];

    let third: &i32 = &v1[2];
    println!("{}", third);
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    match v.get(1) {
        Some(s) => println!("{:?}", s),
        None => println!("0"),
    }

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 10;
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:#?}", row[1]);
}
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
