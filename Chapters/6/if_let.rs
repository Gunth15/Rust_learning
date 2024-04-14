
fn main() {
    /*if let syntax lets is another way to handle values that match one pattern whilst ignoring
     * others(less robust than pattern matching)*/

    //this 
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("There maximum is configured to be {max}"),
        _ => (),
    }

    //turns into this
    if let Some(max) = config_ max {
        println!("There maximum is configured to be {max}")
    }

    /*can also use else statement with if let*/
    if let Coin::Quarter(state) = coin {

    } else {
        count += 1
    }
    //Use to reduce unnexessary code, usually when one option is all you care about
}
