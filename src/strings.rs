use std::io;

pub fn strings() {
    fn make_money(val: &mut String) {
        val.push_str("$$$");
        
    }

    let mut money: String = String::from("100");
    make_money(&mut money);
    println!("{}", money);

    // We can also concatenate with + sign operator but with this owneership will move.

    let first_name: String = String::from("Avi");
    let last_name: &str = "Ahola";
    let full_name = first_name.clone() + " " + last_name; // -> clone helps to hold the value in first name.
    println!("{}", full_name);

    // we do have other ways to add to string using format macros.

    let name = format!("{} {}", first_name, last_name);
    println!("{}", name);

    // trim and uppercase methods
    fn trim_and_capitalize(val: &str) -> String {
        val.trim().to_uppercase()
    }
    println!("{}", trim_and_capitalize("   bikash   "));

    // Spliting the strings..
    fn element(word: &str) -> Vec<&str> {
        word.split("!").collect::<Vec<&str>>()
    }
    let w: &str = "qw!qe!er!sw";
    println!("{:?}", element(w));

    // Accepting input value from the user.
    fn get_identity() -> String {
        let mut fname = String::new();
        let mut lname = String::new();
        println!("Enter your First Name...");
        io::stdin()
            .read_line(&mut fname)
            .expect("failed to collect first name.");

        println!("Enter your Last Name...");
        io::stdin()
            .read_line(&mut lname)
            .expect("failed to collect last name.");

        format!("{} {}", fname.trim(), lname.trim())
    }

    let name: String = get_identity();
    println!("{:?}", name);
}
