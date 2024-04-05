fn main() {
    //rust has three type of loops: <loop>, <while>, and <for>
    //LOOP:effectively a while(true) loop
    loop {
        println!("again");
        break;
    }
   
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            //break can be used to exit a loop to the last stament with the break is returned
            break counter * 2;
        }
    };
    println!("The result is {result}");

    //loop labels
    let mut count = 0;
    'counting_up :loop {
        println!("count == {count}");
        let mut remaining =10;
        
        loop {
            println!("remaining == {remaining}");
            if remaining == 9{
                //breaks inner loop
                break;
            }
            if count == 2 {
                //breaks outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    //while loop 
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //for loops 
    let a = [10,20,30,40,50];

    for element in a{
        println!("the value is {element}");
    }

    //better liftoff based on previous learning
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
