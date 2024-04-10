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
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height//passes a refernece to another
                                                              //rectangle type
    }
    //constructors dont use self and are called like String::from() was called in orevious chapter
    fn square(size: u32) -> Self {//Self in a return type wich returns an instance of the type
                                  //rectangle in this case 
        Self {//considered an alias
            width: size,
            height: size,
        }
    }
}

fn main() {
    //struct made code more human and readable 
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
     let rect2 = Rectangle {
        width: 10,
        height: 40,
    };   
     let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    //constructor used to make square  
    let sqr = Rectangle::square(20);// :: used for associated funtions and namespaces created by
                                    // modules
    println!("rect1 is Rectangle {:?}",rect1);//debug mode allows us to print struct as such usig
                                              //:? dbg!() can alaso be used
    println!("The area of the rectangle {} is square pixels.", rect1.area());//uses method syntax
                                                                             //to call area funtion

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

