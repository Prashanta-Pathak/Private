mod controlflow;
mod datatypes;
mod exercise;
mod functions;
mod muteshadow;
mod scope;
mod variable;
use datatypes::datatypes;
use exercise::exercise;
use functions::alphabets;
use functions::apply_to_jobs;
use functions::is_even;
use muteshadow::mutable_and_shadowing;
use scope::scope;
use variable::variable_example;

const AUTHOR_NAME: &str = "Prashanta";

fn hi() {
    println!("this is some random text");
}

fn main() {
    println!("Hello world!\nI go by name {AUTHOR_NAME} & I am learning Rust!!!.");
    variable_example();
    mutable_and_shadowing();
    scope();
    exercise();
    datatypes();
    apply_to_jobs(10, "IT");
    let result = is_even(7);
    println!("Result: {result}");
    let (contains_a, contains_z) = alphabets("apple");
    println!("word contains a:{contains_a},word contains z:{contains_z}");
    hi()
}
