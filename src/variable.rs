pub fn variable_example() {
    let apples: i32 = 24;
    let oranges: i32 = 36;
    let fruits: i32 = apples + oranges;

    println!(
        "We have {0} apples, {1} oranges and total of {2} number of fruits in the store",
        apples, oranges, fruits
    );
}
