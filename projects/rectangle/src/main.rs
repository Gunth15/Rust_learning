#[derive (Debug)]//debug feature
struct Rectangle {
    width: u32,
    height: u32,

}
//method implementation , impl defines the function in context to the rectangle
impl Rectangle{
    //similar to funtion but defined within a struct or enum, first parameter always self
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    //struct made code more human and readable 
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("rect1 is Rectangle {:?}",rect1);//debug mode allows us to print struct as such usig
                                              //:? dbg!() can alaso be used
    println!("The area of the rectangle {} is square pixels.", rect1.area());//uses method syntax
                                                                             //to call area funtion
}

