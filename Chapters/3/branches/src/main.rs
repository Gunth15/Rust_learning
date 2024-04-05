fn main() {
    //IF STATEMENT quirks
    let number = 2;

    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    let condition = true;
    
    //if statement is an expression, so it can be used as follows, variable will be bound based on
    //outcome this also means each ar must have the same type
    let x = if condition {5} else {6};
}
