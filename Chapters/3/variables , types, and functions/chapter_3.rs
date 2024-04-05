//many diffrent word in rust languge, can be found in Appendix A
fn main(){
    //by default, variables in rust are immutable(cant be changed) to take advatage if its safety and easy
    //concurrency 
    // let x = 5;
    //println!("The value of x is {x}");
    //x = 6;
    //println!("The value of x is {x}");
    // the previous line of code would not work b/c of this reason

    let mut x = 5; // adding <mut> in front of a variable make it mutable
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    
    // <const> constants, like varibale,  are immutable, but they're are laways immutable 
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    //SHADOWING, var is shaddowed by another instance of let, changing what the compiler sees
    //can on;ly change values if prefix <let> is added which means we basically create a new variable.
    let x = 5;
    let x = x+1;

    {
        //onyly changes the value of x in the inner scope to x *2
        let x = x * 2;

        println!("The value of x in the inner scope is {x}");

    }
   //diffrence between SHADOWING and <mut> is that SHADOWING creates a new variable
    println!("The value of x is: {x}");

    //EX cerate variables spaces and creates second var spaces that has the string's length of the
    //first 
    let spaces = "   ";
    let spaces = spaces.len();
    //could not do this with mut b/c it will produce a compiler error
    //let mut spaces = "   "
    //spaces = spaces.len()
    
    //compiler can usually infer typr of values, but sometimes, must do type annotation
    let guess: u32 = "42".parse().expect("Not a number");







    //Rust has four primitive scalar types [Integers,Floating-point,Booleans,characters]
   
    //INTEGERS: number w/o fractional components
    //<i,u> sigend or unsigned followed by the number of bits 8,16,32,64,128 ex. <u32>
    // <isize> or <usize> uses the amount a bits based on cpu architecture 64,32
    // Also takes number literals such as 
    // Decimal: 98_222 = 98222
    // Hex: 0xfff
    // Octal: 0o77
    // Binary: 0b1111_000
    // Byte (u8 only) b'a'
    

    //Floating-point: floating types are f32 and f64
    //default is f64 b/c they are roughly the same speed

    let x = 2.0; //f64

    let y: f32 = 3.0; //f32
    

    //Basic numeric operations ex.
    let sum = 5 + 10;
    
    let difference = 95.5 - 4.3;

    let product = 4 * 30;
    
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; //Results in -1 

    let remainder = 43 % 5;

    //char in rust are a little diffrent representing more than just ascii







    //Compund types has two primitives: TUPLES and ARRAYS
    let tup = (500, 6.4, 1);
    //DESTRUCTURING allows tuple to be broken down 
    let (x,y,z) = tup;

    println!("The value of y is: {y}");
    
    //can access tuple values directly using <.>
    let x: (u32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    //ARRAY, every elemnt must have the same type
    //explicit
    let a = [1,2,3,4,5];

    //explcit type declaration
    let a: [i32; 5] = [1,2,3,4,5];
    
    //literal_declaration of 3 with array size of 5, so makes [3,3,3,3,3]
    let a = [3; 5];

    //fn call
    blah();
}


//functions specified using <fn> keyword and uses snake case for it's naming convention
fn blah() {
    println!("blah blah");
}
