// This is some examples of control flow statements
pub fn controlflow() {
    // Lets see if condition examples.
    let some_condition: bool = false;
    if some_condition {
        println!("This line will be output.");
    }
    if !some_condition {
        println!("This line will not be output.");
    }

    fn is_even(number: i32) {
        let result: &str = if number % 2 == 0 { "even" } else { "odd" };
        println!("The number is {result}");
    }
    is_even(19);

    // Lets see some match examples now.
    let evaluation: bool = true;
    match evaluation {
        true => println!("its true"),
        false => println!("its false"),
    }

    // Lets see another examples with assigning the variable

    let is_available: bool = true;
    let vegs_products: &str = match is_available {
        true => "We have vegetable products in store",
        false => "out of stock",
    };
    println!("Status:{vegs_products}");

    // Lets do one more examples with match.

    let season: &str = "winter";
    let weather = match season {
        "summer" => "Wow, Lets gets some drinks",
        "winter" => "Oh fuck, its cold",
        "spring" => "hell, its raining",
        _ => "its unpredicttable",
    };
    println!("Weather update:{weather}");

    // Lets see one more examples with conditional statements inside match.

    let number_status: i32 = 13;

    match number_status {
        val if val % 2 == 0 => println!("The number {val} is even."),
        val if val % 2 != 0 => println!("The number {val} is odd."),
        _ => println!("Unknown"),
    }

    // Lets see one more examples of comparing multiple values

    let number: i32 = 3;
    match number {
        1..=4 => println!("it occurs in first match"),
        9 | 8 => println!("its occurs in second match"),
        _ => println!("does not exists"),
    }

    // Lets learn loop now
    let mut seconds: i32 = 10;
    loop {
        if seconds == 0 {
            println!("Blastoff!!!");
            break;
        }
        println!("{seconds} seconds to blast off.......");
        seconds -= 1;
    }

    // Lets see other example with continue keyword inside loop.

    let mut seconds_left: i32 = 27;
    loop {
        if seconds_left <= 0 {
            println!("Blastoff completed");
            break;
        }
        if seconds_left % 2 == 0 {
            println!("The seconds {seconds_left} is even.");
            seconds_left -= 3;
            continue;
        }
        println!("{seconds_left} seconds to Blastoff!!!");
        seconds_left -= 1;
    }

    // lets practise with the while loop now.
    // Write a program that calculates how many months it takes to reach a savings goal, but with a twist: every month, you &&&&increase the amount you save.

    let mut starting_balance: i32 = 100;
    let mut monthly_deposit: i32 = 10;
    let saving_target: i32 = 1000;
    let mut months: i32 = 0;

    while starting_balance <= saving_target {
        starting_balance += monthly_deposit;
        months += 1;
        monthly_deposit += 5;
        println!("The current saving is ${starting_balance} on {months} month.");
    }

    println!("Complete saving achieved in {months} months.");

    // Lets learn recursion.
    fn sec_countdown(seconds: i32) {
        if seconds == 0 {
            println!("Balstoff!!!");
        } else {
            println!("Blastoff at {seconds}seconds.");
            sec_countdown(seconds - 1);
        }
    }
    sec_countdown(12);

    // Lets do some exercise now.

    fn color_to_number(color: &str) {
        let result = if color == "red" {
            1
        } else if color == "green" {
            2
        } else {
            3
        };

        println!("The {color} represent {result} number.");

        // lets write the above functions using match.
        let result = match color {
            "red" => 1,
            "green" => 2,
            "blue" => 3,
            _ => unreachable!(),
        };
        println!("The {color} represent {result} number.");
    }
    color_to_number("green");

    fn factorial(number: i32) -> i32 {
        // let mut result: i32 = 1;
        // while number > 1 {
        //     result *= number;
        //     number -= 1;
        // }
        // println!("factorial is {result}");

        // if number <= 1 {
        //     1
        // } else {
        //     // 1. WINDING (Going down)
        //     println!("Computing: {number} * factorial({})", number - 1);

        //     let result = number * factorial(number - 1);

        //     // 2. UNWINDING (Coming back up)
        //     println!("Result for {number}! is {result}");
        //     result
        // }

        let result: i32 = match number {
            // Base Case: The "Ground Floor"
            0 | 1 => 1,

            // Recursive Case: The "Dive"
            _ => number * factorial(number - 1),
        };

        println!("{result}");
        result
    }
    factorial(5);
}
