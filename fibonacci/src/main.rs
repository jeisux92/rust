use std::io;
use std::time::Instant;
fn main() {
    loop {
        println!("Type a number to the sequence");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Type a number");

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("result loop");
        let now = Instant::now();
        println!("time elapsed {} ns", now.elapsed().as_nanos());
        println!("{}", fibonacci_loop(number));

        // println!("result");
        // let now = Instant::now();
        // println!("time elapsed {} ns", now.elapsed().as_nanos());
        // println!("{}", fibonacci_sequence(number));

        break;
    }
}

// fn fibonacci_sequence(number: i32) -> i32 {
//     match number {
//         0 => 0,
//         1 => 1,
//         _ => {
//             fibonacci_sequence(number - 2) + fibonacci_sequence(number - 1) //2
//         }
//     }
// }

fn fibonacci_loop(number: i32) -> i32 {
    if number < 2 {
        return number;
    }

    let mut fib = 1;
    let mut prev_fib = 1;

    for _n in 2..number {
        let temp = fib;
        fib += prev_fib;
        prev_fib = temp;
    }
    fib
}
