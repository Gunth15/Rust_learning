enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin : Coin) -> u8 {
    match coin {//match arms
        //can use any value over just a boolean like if
        Coin::Penny => 1,//can write an expression {} here for longer values
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main(){
    /*pattern matching is a very powerful feature which allows you to compare values with a series
     * of patterns*/



} 
