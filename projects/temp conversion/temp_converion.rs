use std::io;

fn main() {
    loop {
        //prompts user for the mode 
        println!("To Fahrenheit or Celsius(f/c) or (q) to quit");
        
        //string for mode converter is supposed to be in
        let mut mode = String::new();
        
        //grabs the user's input
        io::stdin()
            .read_line(&mut mode)
            .expect("Failed to get answer");
        //removes space from mode
        let mode = mode.trim();

        //input of q exits program
        if mode == "q" {
            break
        }
        //grabs number for conversion
        let mut num = String::new();

        println!("What number to convert?");
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to get answer");

        //processees answer into correct int format
        let num: u32 = match num
            .trim()
            .parse()
            {
                Ok(num) => num,
                Err(_)  => continue,
            };

        let result: u32 = {
            //modes
            if mode == "f" {
               //Celsius -> Fahrenheit
               (num * 9/5) + 32 
            } else if mode == "c" {
                //Fahrenheit -> Celsius
                (num - 32) * 5/9 
            } else {
                0
            }
        };

        
        println!("Conversion: {result}");
    }
    println!("Exiting!");
}
