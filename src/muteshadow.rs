pub fn mutable_and_shadowing() {
    let _mass_of_car_in_kg: &str = "500";
    let _mass_of_a_car_in_kg: f64 = 400.00;
    let mut _mass_of_a_car_in_kg: i32 = 501; // Here variable assaigned as mutable for future changes.
    let _mass_of_a_car_in_kg = 300; // Shadowing takes place by overriding other variable values. 
    println!(
        "The final legal variable value after muting and shadowing is {}.",
        _mass_of_a_car_in_kg
    );
}
