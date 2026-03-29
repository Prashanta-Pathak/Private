pub fn vectors() {
    let car_brands: Vec<&str> = vec!["Audi", "BMW", "Toyota", "lexus", "Mercedes"];
    let registration_numbers: Vec<i32> = vec![1221, 3212, 3233, 4321, 4325];

    // Vectors does not contains display trait but contains debug trait.

    println!("Car brands: {:?}", car_brands);
    println!("Registration numbers: {:?}", registration_numbers);

    // Lets see some vectors builtin methods.

    let mut score: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];

    score.push(10);

    println!("{:?}", score);

    score.insert(0, 12);

    println!("{:?}", score);

    let removed = score.pop();
    println!("{:?}", removed);

    let value = match removed {
        Some(value) => value,
        None => 0,
    };

    println!("{:?}", value);
    println!("{:?}", score);
}
