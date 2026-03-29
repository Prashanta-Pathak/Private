use std::{collections::btree_map::Range, vec};

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

    let removed = score.pop(); // Pop() and get() method will return Option enum.
    if let Some(ans) = removed {
        println!("The value is {}", ans);
    }
    // -****************Or with if let pattern-****************

    /*let value = match removed {
        Some(value) => value,
        None => 0,
        println!("{:?}", value);
    };*/

    println!("{:?}", score);

    // Reading Vector elements.

    let elements: Vec<i32> = vec![12, 23, 4, 3456, 6, 56, 67, 78, 45, 4];

    let v1 = String::from("Apple");
    let v2 = String::from("Ball");
    let v3 = String::from("van");
    let v4 = String::from("egg");

    let product1: Vec<String> = vec![v1, v2, v3, v4];

    let p1 = &product1[0]; // Borrowing is done to prohibit move since string does not implement copy trait.
    let e1 = elements[2]; // I32 implements copy trait so no move occurs here.
    let p3 = &product1[1..3]; // We can also take string slices.

    println!("{} | {} | {:?}", p1, e1, p3);

    // Lets use get() method to retrive data.

    let o1 = product1.get(1); // We can see it return OPtion type enum , very safer to use.
    match o1 {
        Some(val) => println!(" The value is: {:?}", val),
        None => println!("NO value exists."),
    }
    // Also with if let method we can retrive the value,
    if let Some(value) = o1 {
        println!("See i told you 😀: {}", value);
    }

    // lets revise borrow, ownership & writing to vec.

    let color1: String = String::from("Red");
    let color2 = String::from("blue");
    let color3 = String::from("Green");
    let color4 = String::from("yellow");
    let color5 = String::from("pink");

    let mut rainbow: Vec<String> = vec![color1, color2, color3, color4, color5];

    // Lets us see first writing into vectors.

    rainbow[2] = String::from("purple"); // Appending value to 2nd index specifically.
    println!("{:?}", rainbow);

    // lets see another method to concatinate string to the end of vector value.

    let colorpair = &mut rainbow[2];
    colorpair.push_str(" & Voilet"); // -> goes out of acope.
    println!("{:?}", rainbow);
    let another_pair = &rainbow[3]; //-> This is valid. Since anychanges to the heap memory compleled in line 85.
    let newpair = &rainbow; // This is valid. This is lifetime concept.
    println!(
        "Newpair: {:#?}| Another pair:{:#?} | Rainbow:{:#?}",
        newpair, another_pair, rainbow
    );
    // let another_pair = &rainbow[3]; -> Compiler through error as it can borrow mutable references. but line can execute in line 87.let s see.

    // Lets see vector capacity behind the hood.

    // [REALLOCATION WATCH]
    // When capacity is exceeded (len > capacity):
    // 1. A new, larger memory block is allocated on the heap (usually 2x size).
    // 2. Existing elements are moved to the new location.
    // 3. The old memory block is deallocated.
    //
    // NOTE: Rust's Borrow Checker prevents "dangling references" by ensuring
    // no references to elements exist while the Vec is being mutated.
    // Using `with_capacity` is preferred to avoid this O(n) copy overhead.

    let mut memory: Vec<i32> = Vec::with_capacity(4);
    memory.push(1);
    memory.push(1);
    memory.push(1);
    memory.push(1);
    println!(
        "length: {:?} | Capacity: {:?} | Memory:{:?} ",
        memory.len(),
        memory.capacity(),
        memory
    );

    // This creates move of all vectors to new heap location, increasing capacity by double and may create dangling reference as first memory links to first heap memory location while moving data to new heap location.so its better not to use multiple mutable reference to mutable vectors.

    memory.push(1);
    println!(
        "length: {:?} | Capacity: {:?} | Memory:{:?} ",
        memory.len(),
        memory.capacity(),
        memory
    );

    // Exercise..A file system on a computer.............**********************************

    #[derive(Debug)]
    struct File {
        name: String,
    }

    #[derive(Debug)]
    struct Folder {
        name: String,
        content: Vec<File>,
    }

    impl Folder {
        fn new(name: String) -> Self {
            Self {
                name,
                content: vec![],
            }
        }

        fn create_file(&mut self, name: String) {
            let file = File { name };
            self.content.push(file);
        }

        fn delete_file(&mut self, i: usize) -> File {
            self.content.remove(i)
        }

        fn get_file(&self, i: usize) -> Option<&File> {
            self.content.get(i)
        }
    }
}
