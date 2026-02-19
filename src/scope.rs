pub fn scope() {
    let shoe_price: i32 = 32; // This value can be accessed inside whole function block.
    println!("The price of the shoes is {shoe_price} dollar.");

    {
        let sandle_price: i32 = 45; // the value inside the block cannot be accessed outside this block.
        println!("the value is {sandle_price}.");
    }
}
