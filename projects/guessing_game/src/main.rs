//library added using [use] keyword
use std::io;
use std::cmp::Ordering;
//used add rand cargo library to by adding it to Cargo.toml and then running card build
use rand::Rng;


fn main() {
    //println is macro?
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);//get_range() takes range expression
                                                              //as argument <start..=end> and is
                                                              //inclusive on lower and upper bound
    loop{

        println!("Please input your guess. ");
        //variable are immutable by default, so [mut] prefix added after [let] to denote the variable
        //is mutable
        // [new()] creates new instance of string in theis scenario (strings are UTF-8)
        // :: syntax means the it is a associative function: funtion implemnted on a type 
        let mut guess = String::new();

        //io imported from std library (still coud have used  if we calld std:io:stdin()) type called
        //std(standard) that has a type called io that contains a funtion stdin() which contains a
        //funtion that reads standard input(in this case get user text input)
        io::stdin()
            .read_line(&mut guess)//calls read_line and reads user input and appends(does not overwrite) guess by passing <&mut guess> as a refernece
            // <&> REFERENCE in rust- allows multiple parts of code to use data w/o creating data multiple times in memory
            .expect("Failed to read line");
            //expect retruns a RESULT value, a ENUMERATION often shortened to ENUM. A type that can be in multiple
            //possible states called VARIANTS. The results are used to encode error handling here(ok and Err)
            //OK means the operation was successful, and the inside OK is the genrated value, Err
            //contains info on why it failed. RESULT,types like any value type have methods defined on
            //them
        
        //integers in rust can be i32(defult 32-bit), u32(unsigned 32 bit int), i64
        //code creates a new varible calle guess via SHADOWING the previous varible string named guess
        //guess is binded to expression
        // <:> tells rust that we will annotate variable type. in this expression compares u32 to guess
        // and makes sure we have the appropriate typing
        let guess: u32 = match guess
            .trim() //whitespace is trimmed
            .parse() //converts string to another type
            {
                //if type is valid, return number else, skip and loop
                // <_> is a catch-all, continue goes to next iteration of loop
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {guess}");// {} is a place holder, can either be placed inside or at the
                                        // end in order startng after a comma 


        //<match> expression used to decide what is returned based on wich variant will be returned 
        match guess.cmp(&secret_number){//takes secret_number as refernece
           //Ordering also a enum and has variants <Less,Greater,Equal> used to compare values 
           //based on the ARM pattern, diffrent values returned SYNTAX: blah => 
           Ordering::Less => println!("Too small"), 
           Ordering::Greater => println!("Too big"),
           Ordering::Equal => {
                //exits loop on correct answer
                println!("You win!");
                break;
           }
        }    
    }
}
