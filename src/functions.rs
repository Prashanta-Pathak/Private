pub fn apply_to_jobs(number: i32, title: &str) {
    println!("I'm applying to {number} {title} jobs.");
}

pub fn is_even(number: i32) -> bool {
    number % 2 == 0
}
pub fn alphabets(text: &str) -> (bool, bool) {
    let has_a = text.contains('a');
    let has_z = text.contains('z');
    (has_a, has_z)

    //.     (text.contains('a'),text.contains('z'))-> also gives result (tuple way structure)
    //.     println!("{:?}",alphabets("example"));
}
