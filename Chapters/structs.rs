struct User {//like a template
    //struct contains pieces of data called field
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//unit struct
struct AlwaysEqual;


struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

fn main() {
    //structs similar to tuples
    //however, structs allows you to name each value, core to creating new datatypes this, makes
    //them more flexible than tuples


   //creating instance of struct, no specific order to declare variables in struct
    let user1 = User {
        //
        active:true,
        username: String::from("someusername123"),
        email: String::from("blahblah@ex.com"),
        sign_in_count: 1,
    };

    //can also be made mutable
    let mut user2 = User {
        //
        active:true,
        username: String::from("someusername123"),
        email: String::from("blahblah@ex.com"),
        sign_in_count: 3,
    };
    
    user2.email = String::from("anothereamil@blah.com");

    //NOTE* all fields must be mutable in rust

    let user3 = User {
        email: String::from("Emall@ex.com"),
        //tells the program that the remaining fields are the same data as user1
        ..user1
    };//this operation moves the data, so will no longer be able to use  user1 after creation,
      //however primitves like bool, or in t can easily be copied and we make user 1 still valid
    


    //rust supports struct that look like tuples(tuple structs)
    //tuple structs dont have names, but do have types of fields
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    //useful for seperating tuples form other tuples by giving it a types
    

    //unit structs, useful when you need to implement a trait but don't have data
    let subject = AlwaysEqual;
}


//can also be used in functions 
fn build_user(email: String, usename: String) -> User{
    user{
        active:true,
        //can write like this, but repetitive
        //username: username,
        //email: email,
        //alternative:
        username,
        email,
        sign_in_count: 1,
    }

}
