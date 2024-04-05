fn main() {
    let x = 8;
    let num = fib(x); 
    println!("The fibbonacci number for f({x}) is {num}");

    let num = fibr(x);
    println!("The fibbonacci number for f({x}) is {num} when recursively called");
    
}

fn fib(x: u32) -> u32 {
    //Declaring the forst two fibbonacci numbers
    let mut fib1 = 0;
    let mut fib2 = 1;

    //if fib(0), return 0
    if x == 0{
      return fib1;
   } 

    //if fib(1) -> 1, else fib(x) = fib1 + fib2 
    for _ in 1..x+1 {
        
        let p = fib2;
        fib2 += fib1;
        fib1 = p;

    }
    fib1
}

fn fibr(x: u32) -> u32 {
  if x == 0 {
    return 0;
  }
  if x == 1 {
    return 1;
  }
  fibr(x-1)+fibr(x-2) 
}
