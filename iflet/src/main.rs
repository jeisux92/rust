fn main() {
    let mut some_u8_value = Some(3);
    

    check_number_match(&mut some_u8_value);
    let number = if_let(some_u8_value);
    println!("{:#?}{}",some_u8_value, number);
}

fn check_number_match(number:&mut Option<u32>){
    match number {
            Some(s) =>{ 
               *s+=2;
                println!("three {}",s);
            },
            _ => (),
        }
}

fn if_let(number:Option<u32>)->u32{
   if let Some(s)= number{
        s+2
    }
    else{
        4
    }
}