fn main() {
    //Ownership: set of rules that govern how rust manages memory
    //used in place of normal garbage collector or diy memory removal
    //Rust manages memory by checking a set of rules at compiler time
    
    //Rules of Ownership:
        //1.Each value in rust has a owner
        //2.There can only be one owner at a time
        //3.when the owner goes out of scope, value lost


    //Variable Scope 
    //scope:range within a program that a varibale is valid
    let s = "hello"; //var is valid until end of scope

    {   //v is not valid here since it has not been declared
        let v = "hello"; //v valid from this point foward

        //blah blah
        
        
    }//this scope is now over, and v is no longer valid

    //Besides string literlas, rust has another string data type called String which manages data
    //stored on the heap which allows it to store more text
    
    //uses a string literal to make a String
    //:: allows us to namespace a paticular funtion so we can us that instead of something like
    //string_from
    let s = String::from("hello");

    //unlike string literals, this one can be mutated
     let mut s = String::from("hello");
     s.push_str(", world!"); //push_str() append a literal to a String

     println!("{s}"); //expected output: "hello, world"
    
     //string literals are hardcoded into the final exec
     //String type on the other hand is mutable, so memory put on the heap
     //This means:
         //1.Mem must be requested from mem allocater at runtime
         //2.Need a way of returning this memory to allocator when we're done w/ our string
    
    //CONCEPT
    //b/c int values are known and simple, 5 is pushed onto stack for y
    let x = 5;
    let y = x;
    
    //str made up of three parts ptr,len,capacity, only those parts are copied to s2, not the data     
    let s1 = String::from("hello");
    let s2 = s1;

    //double free error: when memory is freed twice which can corrupt data 
    //Rust handles this by MOVEing the pointer,len, and capacity to the new variable
    //EX:
    //
    //let s1 = String::from("hello")
    //let s2 = s1 //referred to as shallow copy in most languages shallow copy data is MOVEed
    //
    //println!("{s1}, world!");
    //would not run b/c the data has been moved


    //Deep copy(copy data and create new pointer,etc.) by using common method called clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s = {s1}, s2 = {s2}");


    //NOTE* when a more complex data types are moved to funtions, the data is freed afterwards(can
    //no longer use passed data)

    //What of we want to referece a data type w/o using it?
    //REFERENCE: is like an address to access the data stored at that address defined by <&> symbol
    //act of creating a refernec is called BORROWING
    //EX
    let s1 = String::from("hello");

    let len = calculate_length(&s1);//notice how <&> is use for a refernce so calculate_length

    println!("The length of '{s1}' is {len}.");

    //Rules of refernce
    //1.At any given time, 1 mutable refernce OR any number of immutable references
    //2.referneces must always be valid
    



    //Slices//
    //slices alloew you to refernce a contiguous sequence of elements in a collection
    //type of reference does not have ownership 

    //STRING SLICE: reference to part of a string
    let s = String::from("hello world");
    let hello = &s[0..5];//specify slice by [starting_index..ending_index]
    let world = &s[6..11];

    //can drop leading zero if index starts at zero
    //these are equal
    let hello = &s[0..5];
    let hello = &s[..5];

    //can drop trailing number if index includes last byte of string
    let world = &s[6..11];
    let world = &s[6..];

    //can even drop both to get entire string
    let hello_world = &s[0..11];
    let hello_world = &s[..];

    //Other slices
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice,&[2,3]);
}

fn calculate_length(s :&String) -> usize {//funtion defined to take referneces by <&> 
    s.len()
}//cant modify refernce  unless the variable is mutable and the refernce is declared with mut as
 //done in previous chapters, cannot declare multiple mutable references at the same time 
 //restricts mutations to avoid data races
 //
 //references assure us we cannot create dangling pointers in rust(pointers that don't point to any
 //data)
 


fn first_words(s: &String) -> usize {
    let bytes = s.as_bytes();//converts string to array of bytes

    //bytes.iter creates a iterator over the array of bytes, enumerte wraps returns each elemet as
    //part of a tuple instead where the first element is the index and the second is a reference to
    //the element
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {//byte literal syntax
            return i
        }
    }
    s.len()
}//problem with this code is that it is independent from s meaning if s changes, the first word
 //index pointer becomes irrelevant
 
fn first_words2(s: &String) -> &str {//notice we return a string slice instead of a String sting
                                     //literals are string slices,so use &str where it is
                                     //applicable
    let bytes = s.as_bytes();//converts string to array of bytes

    //bytes.iter creates a iterator over the array of bytes, enumerte wraps returns each elemet as
    //part of a tuple instead where the first element is the index and the second is a reference to
    //the element
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {//byte literal syntax
            return &s[..i];//now returns string slice
        }
    }
    &s[..]
