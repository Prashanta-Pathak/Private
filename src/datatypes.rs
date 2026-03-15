#![allow(unused_variables)]
pub fn datatypes() {
    let my_name: &str = "Avinash";
    // lets us print with tab
    println!("Name after the tab is\t{my_name}.");
    //lets do more examples with printing the file path. We will use r as "Raw string literal"
    let filepath: &str = r"/users/Desktop/Rust/Hello-World";
    println!("The file is located at {filepath}.");
    // lets do some examples with casting.Casting is done adding as syntax.
    let miles_away: i32 = 50;
    let miles_away: i8 = miles_away as i8;
    let miles_away: u8 = miles_away as u8;
    println!("The new value with {miles_away}.");
    // we can also change the data types with casting
    let miles_away: i32 = 51;
    let miles_away: f32 = miles_away as f32;

    // Character (:char) type
    let first_character: char = 'A'; // should be only one character. it is only 4 byte.
    let emoji_character: char = '👌'; // Character type is enclosed in single quotes.

    println!(
        "{} {}",
        first_character.is_alphabetic(),
        emoji_character.is_alphabetic()
    );
    // Debug macro
    dbg!(2 + 2 + 2); // Strictly dor developer propose.
    // Array type
    #[allow(unused_assignments)]
    let mut numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];
    numbers = [200, 3, 4, 5, 6, 7];
    let products: [&str; 3] = ["Mac", "Hp", "Dell"];
    let my_product: &str = products[0];
    println!(
        "This is my new {}\nThis is my previous laptop {}\nThis the the price{}",
        my_product,
        products[1],
        numbers[0] // Accesing array value just a simple example
    );
    let currency_rates: [f64; 0] = [];
    println!(
        "The test array numbers are \n{:?}\nThe tested products are \n{:#?}",
        numbers, products
    );

    println!("You can see the changes with the casting.\nThe final value is {miles_away}.");

    // Tuple types
    let info: (&str, i32, &str) = ("Prashanta", 29, "Rust");

    /*let name = info.0;
    let age = info.1;
    let course = info.2;
    */
    let (name, age, course) = info; // The above method can be written in this way too.
    println!("Name: {name}\nAge: {age}\nCourse: {course} ");
}
