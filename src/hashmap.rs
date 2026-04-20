use ::std::collections::HashMap;
use ::std::collections::HashSet;

pub fn hashmap() {
    let mut menu: HashMap<String, f64> = HashMap::new();
    menu.insert(String::from("Burger"), 10.45);
    menu.insert(String::from("Taco"), 11.34);
    println!("{:#?}", menu);

    let mut capital_city: HashMap<&str, &str> = HashMap::new();
    capital_city.insert("Nepal", "ktm");
    capital_city.insert("Finland", "Helsinki");
    capital_city.insert("India", "Delhi");

    println!("{:#?}", capital_city);

    let students_class = [("Avi", 10), ("Hari", 12), ("Ram", 6), ("Gita", 11)];

    // We can also add hashmap with from keyword.
    let mut student_list: HashMap<&str, i32> = HashMap::from(students_class);
    println!("{:#?}", student_list);

    // Lets see Remove method which removes the key and also store the value of it pairs and is returns option type.
    let r = student_list.remove("Gita");
    match r {
        Some(key) => println!("The removed pair is:{}", key),

        None => println!("NO PROPER KEY FOUND"),
    };

    // Access the value by key.
    let first_student = student_list.get("Avi").copied();

    match first_student {
        Some(key) => println!("The student is {}", key),
        None => println!("Nobody found"),
    };

    let second_student = student_list.get("Hari").copied().unwrap_or(0);
    println!("{}", second_student);

    // Overwriting the value with existing key.

    let mut item_with_prices: HashMap<&String, i32> = HashMap::new();

    let shoes = String::from("Addidas");
    let shoe_prices = 21;
    let tshirt: String = String::from("Long-tshirt");
    let tshirt_price: i32 = 33;

    item_with_prices.insert(&shoes, shoe_prices);
    item_with_prices.insert(&tshirt, tshirt_price);

    println!("{:?}", item_with_prices);

    item_with_prices.insert(&shoes, 33);
    println!("{:?}", item_with_prices);

    item_with_prices.insert(&tshirt, 21);
    println!("{:?}", item_with_prices);

    // The entry method.
    let new_product: String = String::from("Hoodie");
    item_with_prices.entry(&new_product).or_insert(41);
    println!("{:#?}", item_with_prices);

    // HashSet........
    let mut my_set: HashSet<&str> = HashSet::new();
    my_set.insert("algorithm");
    my_set.insert("logic");
    my_set.insert("apple");
    println!("{:?}", my_set);

    // We can also apply remove method...
    let rmou = my_set.remove("apple"); //. -> remove return bool in hashset.
    println!("{}", rmou);
    println!("{:?}", my_set);

    // if we try to  insert the  value that exist in hashset, it will not do it.
    my_set.insert("logic"); // This does not work becoz it exist in hashset.

    let length = my_set.len();
    println!("{}", length);

    let find = my_set.get("logic"); // -> get method return options.
    match find {
        Some(val) => println!("The value is {}", val),
        None => println!("No value"),
    }

    my_set.insert("Data");
    let if_avail = my_set.contains("Data"); // -> return bool
    println!("{}", if_avail);

    // Exercise.....................

    let mut sauces_to_meals: HashMap<&str, Vec<&str>> = HashMap::from([
        ("ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
        ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]),
    ]);

    sauces_to_meals.insert("Mustard", vec!["Hot dog", "Burgers", "Pretzels"]);

    println!("{:#?}", sauces_to_meals);

    let v = sauces_to_meals.remove("Mayonnaise");
    match v {
        Some(val) => println!("{:#?}", val),
        None => println!("No value"),
    }

    let g = sauces_to_meals.get("Mustard");
    match g {
        Some(val) => println!("{:#?}", val),
        None => println!("No value"),
    }

    sauces_to_meals
        .entry("Soy Sauce")
        .or_insert(vec!["Sushi", "Dumplings"]);

    println!("{:#?}", sauces_to_meals);
}
