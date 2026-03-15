mod controlflow;
mod datatypes;
mod exercise;
mod functions;
mod muteshadow;
mod ownership;
mod scope;
mod slices;
mod variable;

fn main() {
    variable::variable_example();
    muteshadow::mutable_and_shadowing();
    scope::scope();
    exercise::exercise();
    functions::functions();
    datatypes::datatypes();
    controlflow::controlflow();
    ownership::ownership();
    slices::slices();
}
