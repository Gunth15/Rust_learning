fn main() {
    another_funtion(5);
    print_labled_measurement(5, 'h');

    //STATEMENTS: instructions that define some value but do not return a value
    let x = 6 //this is a STATEMENTS
    //EXPRESSIONS: return a value
    let x = 6 + 7
    //Diffrent from other languages b/c a statement can not be usede as an argument b/c it fdoes
    //not return a value 
    
    //expressions , like the following do not include ending simicolns, but when you end it with
    //<;>, it becomes a statement
    let y = {
        //expression
        let x = 3;
        x + 1
    };

    let y = five();
}
//must specify type of arguments
fn another_funtion(x: i32) {
    print!("The number is {x} ");
}

fn print_labled_measurement(value :i32, unit_label: char){
    print!("The measurement is: {value}{unit_label}");
}
// most funtions return the last statment explicitly, but can return early using <return> it does
// this b/c it is an expressions
fn five() -> i32{
    5
}

