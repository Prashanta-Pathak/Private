pub fn variable_example() {
    let apples: i32 = 24;
    let oranges: i32 = 36;
    let fruits: i32 = apples + oranges;
    const AUTHOR_NAME: &str = "Prashanta";
    println!("Hello world!\nI go by name {AUTHOR_NAME} & I am learning Rust!!!.");
    println!(
        "We have {0} apples, {1} oranges and total of {2} number of fruits in the store",
        apples, oranges, fruits
    );
}
