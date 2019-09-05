fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> =Option::Some(2);

    let result = get_number(&some_number);
    let colombia= Country::Argentina(20,20);

    match colombia{
        Country::Argentina(ss,tt)=>println!("{:#?}",ss),
        Country::Colombia=>println!("Colombia"),
        _=>println!("Venezuela")
    }
    println!("end {:#?}",result);
}

fn get_number(number:& Option<i32>)->i32{
    match number{
        Some(s)=>*s,
        None=>0
    }
}

enum Country{
    Colombia,
    Venezuela,
    Argentina(u32,u32)
}