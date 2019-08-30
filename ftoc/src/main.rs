use std::io;
fn main() {
    loop {
        println!("Please type your F° temperature to convert to C°");
        let mut farenheit = String::new();
        io::stdin()
            .read_line(&mut farenheit)
            .expect("Please type the number");

        let farenheit: f64 = match farenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("The number is nor correct");
                continue;
            }
        };

        let _celsius = (farenheit - 32.0) *( 5.0 / 9.0);
        println!("The temperature in °C is {}", _celsius);
        break;
    }
}
