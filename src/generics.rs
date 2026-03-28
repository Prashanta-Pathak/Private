pub fn generics() {
    // Lets learn gererics.

    // This is defining gerenic function.
    fn identity<T>(value: T) -> T {
        value
    }
    #[derive(Debug)]
    struct Genome {
        name: String,
    }

    println!("Value: {}", identity::<i32>(17)); // Utilizing Turbofish Operator (::<Type>) -> Inside type we mention our type.
    println!("Value: {}", identity::<f32>(21.09));
    println!("Value: {}", identity::<&str>("Generics"));
    println!(
        "Value: {}",
        identity::<String>(String::from("Hello Generics Example!"))
    );
    println!(
        "Value: {:?}",
        identity::<Genome>(Genome {
            name: String::from("I am Struct")
        })
    );

    // lets see few variations in gerenics type.

    fn work_days<T, U>(val1: T, val2: U) -> (T, U) {
        (val1, val2)
    }
    // This above function takes two generic types, which could be of any type.for-eg:

    println!(
        "Generics Types: {:?}",
        work_days(String::from("Sunday"), "Friday")
    ); // This is an example of utilizing different types.

    // Lets create one more eg;
    fn holiday<T>(day1: String, day2: T) -> (String, T) {
        (day1, day2)
    }

    let holiday = holiday(String::from("Monday"), "Sunday");
    println!("Example with specific type with gerenic type:{:?}", holiday);

    /* Note:
    -> When we pass arguments to the gerenics, it should match the parameter placement. for instance (&str,String)
    is not accepted, it should match the parameter placement.
    -> if we have a single generic type with two parameters like <T>(v1,v2) both v1 and v2 should have same type.*/

    // Lets see generics in Structs.************************************************************
    #[derive(Debug)]
    struct Winner<T> {
        position: String,
        medal: T,
    }

    impl Winner<String> {
        // This is implicit impl type meaning it can only handle String type as mentioned Winner<String>. and is not scalable.
        fn first_place(&mut self) -> &Self {
            self.position = self.position.trim().to_string();
            self.medal = self.medal.to_uppercase();
            self
        }
    }

    let mut player1 = Winner {
        position: String::from("         First Position         "),
        medal: String::from("Gold"),
    };
    player1.first_place();
    println!("Player1: {:?}", player1);

    // LETS SEE OTHER WAYS TO MAKE MORE SCALABLE.
    impl<T> Winner<T> {
        fn second_place(&mut self) -> &mut Self {
            self.position = self.position.trim().to_string();
            self
        }
    }

    let mut player2 = Winner {
        position: String::from("Second"),
        medal: 10,
    };

    player2.second_place(); // In this example i32 value was passed to impl 
    println!("{:?}", player2);

    // Lets see generic in enums.
    #[derive(Debug)]
    enum Pizza<T> {
        Plain,
        Topping(T),
    }
    let mushroom = Pizza::Topping(String::from("Fresh-Mushroom"));
    let onions = Pizza::Topping("Fried onions");
    let topping = String::from("Bacon");
    //let plain: Pizza<String> = Pizza::Plain;
    let plain = Pizza::Topping(String::from("Ananas"));
    println!(
        " Mushroom:{:?} | Onions: {:?} | Topping: {:?} | Plain: {:?}",
        mushroom, onions, topping, plain
    );

    // Lets do some exercises now.
    #[derive(Debug)]
    enum DigitalContent {
        AudioFile,
        VideoFile,
    }
    #[derive(Debug)]
    struct ChatMessage<T> {
        content: T,
        time: String,
    }
    impl ChatMessage<DigitalContent> {
        fn consume_entertainment(&self) -> &Self {
            println!("Watching the {:?}", self.content);
            self
        }
    }

    impl<T> ChatMessage<T> {
        fn retrive_time(&self) -> String {
            self.time.clone()
        }
    }

    let messg = ChatMessage {
        content: "Nature",
        time: String::from("2min"),
    };
    let messg1 = ChatMessage {
        content: String::from("I am string"),
        time: String::from("2min"),
    };

    let messg2 = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("5min"),
    };

    println!(
        " Generics with Enum Value: {:?} | String Generics {} | String Generics: {} | String Generics: {}",
        messg2.consume_entertainment(),
        messg1.retrive_time(),
        messg2.retrive_time(),
        messg.retrive_time()
    );
}
