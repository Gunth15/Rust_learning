//defined custom date type that can be defined elsewhere in code 
enum IpAddr {
    V4(u8, u8, u8, u8),//can specify that 4 bytes are used specifically
    V6(String),//associates enum with string value
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}
impl Message {
    //enums can have methods aswell
    fn call(&self) {
        //dumb method
        1
    }
}
fn main() {
    /*enumerations also referred to as enums allows you to define a type by enumerating its
    possible variants*/

    /*useful for when a variable can be multiple different type ex.
    ip address, there are two types, Ipv4 &Ipv6
    both are ip addresses, but have diffrent amount of bytes dedicated to them
    ex.*/
    
    //enum variant namespaced under identifier
    let four = IpAddr::V4(127,0,0,1);
    let six = IpAddr::V6(String::from("::1"));//returns an instance of IpAddrKind using the
          

    let m = Message::Write(String::from("hello"));
    m.call();
   

}

