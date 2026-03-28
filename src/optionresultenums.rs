use std::collections::btree_map::Values;

pub fn optionresult() {
    // Quick intro to options and enums.

    let a = Option::Some(5);
    let b: Option<i8> = Option::Some(10);
    let c = Option::<&str>::Some("Hello");
    let d = Option::Some(String::from("I am String."));
    let e: Option<i32> = Option::None;
    let f = Option::Some(true);

    // Lets see real example of Option enum.

    let musical_instruments = [
        String::from("Piano"),
        String::from("Guitar"),
        String::from("ukulele"),
        String::from("Drum"),
    ];

    /* This was commented out to show other idiomatic rust way.
    println!(
        "{:?} | {:?}",
        my_instrument.expect("No value exists"), // Better not to use unwrap() -> (Used for debug porpose).
        invalid_instrument.expect("No value exists")
    );
    */
    let my_instrument = musical_instruments.get(2);
    match my_instrument {
        Option::Some(instrument) => println!("{}", instrument),
        Option::None => println!("None"),
    }

    let invalid_instrument = musical_instruments.get(5);
    match invalid_instrument {
        Option::Some(instrument) => println!("{}", instrument),
        Option::None => println!("None"),
    }

    // We can narrow it down to a simple function.

    fn is_got_instrument(instrument: Option<&String>) {
        match instrument {
            Option::Some(instrument) => println!("You got {:?}", instrument),
            Option::None => println!("You got Nothing"),
        }
    }

    is_got_instrument(my_instrument);
    is_got_instrument(invalid_instrument);

    // Lets see example of retruning Option enum from a function.

    fn is_in_warehouse(insystem: bool, inwarehouse: bool) -> Option<bool> {
        if insystem && inwarehouse {
            Some(true)
        } else if insystem {
            Some(false)
        } else {
            None
        }
    }

    let is_available = is_in_warehouse(false, true);

    match is_available {
        Some(true) => println!("The item is available."),
        Some(false) => println!("The item is not available in stock."),
        None => println!("its not available in the system."),
    }

    let in_store = Some("apples");
    let out_of_store: Option<&str> = None;

    println!("we have {:?} in the store.", in_store);
    println!("we dont have this product {:?}.", out_of_store);

    // lets write our own Option enum.

    #[derive(Debug, Copy, Clone)]
    enum MyOption {
        Some(i32),
        None,
    }

    impl MyOption {
        fn no_wrap(self) -> i32 {
            match self {
                MyOption::Some(param) => param,
                MyOption::None => panic!("oh noooo"),
            }
        }
        fn no_wrap_or(self, callback: i32) -> i32 {
            match self {
                MyOption::Some(param) => param,
                MyOption::None => callback,
            }
        }
    }

    let new_doc = MyOption::Some(10);
    println!("{:?}", new_doc.no_wrap());

    let doc1 = MyOption::None;
    println!("{}", doc1.no_wrap_or(11));

    // Lets move on to Result enum now.

    let is_valid: Result<i32, &str> = Result::Ok(5);
    let is_not_valid: Result<i32, &str> = Result::Err("Empty");

    println!("{:?}", is_valid);
    println!("{:?}", is_not_valid);

    let string_interger1: &str = "32";
    let converted_string_integer1 = string_interger1.parse::<i32>();
    println!("{}", string_interger1);
    println!("{:?}", converted_string_integer1);

    let string_interger: &str = "hello";
    let converted_string_integer = string_interger.parse::<i32>();

    println!("{}", string_interger);
    println!("{:?}", converted_string_integer);

    // lets see some exapmles of result enum in functions,

    fn calculate(num1: f64, num2: f64) -> Result<f64, String> {
        if num2 == 0.0 {
            Err("Cannot do calculation".to_string())
        } else {
            Ok(num1 / num2)
        }
    }

    let calc = calculate(10.0, 0.0);
    //println!("{:?}", calc);

    match calc {
        Err(messg) => println!("{}", messg),
        Ok(value) => println!("Division answer {}", value),
    }

    let mut item = vec!["app", "Dap", "pap", "Zap"];
    while let Some(items) = item.pop() {
        // Similar to if let expression
        println!("The next item is {}", items);
    }

    // Exercises.........................................................
    #[derive(Debug)]
    struct Food {
        name: String,
    }
    #[derive(Debug)]
    struct Restuarant {
        reservation: u32,
        has_mice_infestation: bool,
    }

    impl Restuarant {
        fn chef_special(&self) -> Option<Food> {
            if self.has_mice_infestation {
                return None;
            }
            if self.reservation <= 12 {
                Some(Food {
                    name: String::from("Uni Sashimi"),
                })
            } else {
                Some(Food {
                    name: String::from("Strip Steak"),
                })
            }
        }

        fn deliver_burger(&self, address: &str) -> Result<Food, String> {
            if self.has_mice_infestation {
                return Err("Sorry, we have a mice problem".to_string());
            }

            if address.is_empty() {
                Err("No delivery address found".to_string())
            } else {
                Ok(Food {
                    name: String::from("Chicken Berger"),
                })
            }
        }
    }
    let r1 = Restuarant {
        reservation: 11,
        has_mice_infestation: true,
    };
    let r2 = Restuarant {
        reservation: 15,
        has_mice_infestation: false,
    };
    let r3 = Restuarant {
        reservation: 15,
        has_mice_infestation: false,
    };
    println!("{:?}", r1.chef_special());
    println!("{:?}", r3.chef_special());
    println!("{:?}", r2.deliver_burger(""));
    println!("{:?}", r2.deliver_burger("HOme"));
}
