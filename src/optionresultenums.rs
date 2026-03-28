use std::ops::Index;

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
        Option::None => println!("NOne"),
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
}
