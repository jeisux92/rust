fn main() {
    println!("Hello, world!");
    another_function(5, 6);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    println!("{}", five(4));
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five(number: i32) -> i32 {
    5 * number
}
