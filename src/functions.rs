pub fn functions() {
    fn apply_to_jobs(number: i32, title: &str) {
        println!("I'm applying to {number} {title} jobs.");
    }
    apply_to_jobs(10, "IT");

    fn is_even(number: i32) -> bool {
        number % 2 == 0
    }
    let result = is_even(7);
    println!("Result: {result}");

    fn alphabets(text: &str) -> (bool, bool) {
        let has_a = text.contains('a');
        let has_z = text.contains('z');
        (has_a, has_z)
        //.     (text.contains('a'),text.contains('z'))-> also gives result (tuple way structure)
        //.     println!("{:?}",alphabets("example"));
    }
    let (contains_a, contains_z) = alphabets("apple");
    println!("word contains a:{contains_a},word contains z:{contains_z}");
}
