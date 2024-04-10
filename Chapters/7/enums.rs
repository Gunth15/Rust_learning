//defined custom date type that can be defined elsewhere in code 
enum IpAddr {
    V4(u8, u8, u8, u8),//can specify that 4 bytes are used specifically in the form of a tuple
    V6(String),//associates enum with string value
}

enum Message {
    Quit,
    Move (x: i32, y: i32),
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


    /*option enum apart of base library. Uses the value. Rust does not have the Null value, instead
    uses the enume Some() or None. This ensures type daftey from the vagueness of Null*/
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;//when using None, must annotate type of <option> enum

    //the following code will not run because the types are not the same 
    /*let x: i8 =5;
      let y: Option<i8> = Some(5);

      let sum = x + y
    */
     // forces user to handle the case where python is not null

}

