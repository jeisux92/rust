fn main() {
    println!("Hello, world!");
    let ipv_4:IpAddrKind=IpAddrKind::V4(String::from("192.168.0.4"));
    let ipv_6:IpAddrKind=IpAddrKind::V6(String::from("92.12.135"));
    let messsage = Message::Move{x:2,y:2};
    route(&ipv_4);
    let m = Message::Write(String::from("hello"));

    println!("{:#?}{:#?}",ipv_4,ipv_6);
    println!("{:#?}",messsage);
}
#[derive(Debug)]
enum IpAddrKind{
    V4(String),
    V6(String)
}

fn route(ip_kind:&IpAddrKind){
    println!("{:#?}",ip_kind);
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        self.
    }
}