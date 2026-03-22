use std::ops::Sub;

pub fn enumurate() {
    // enums Practise *****************************************************************************************************
    #[derive(Debug)]
    enum CardSuit {
        Hearts,
        Spades,
        Club,
        Diamond,
    }
    #[derive(Debug)]
    struct Card {
        card: String,
        class: CardSuit,
    }

    let first_card: CardSuit = CardSuit::Hearts;
    let mut second_card: CardSuit = CardSuit::Spades;
    second_card = CardSuit::Club;

    let card_details = (CardSuit::Hearts, CardSuit::Club);
    let card_array = [CardSuit::Club, CardSuit::Hearts, CardSuit::Diamond];

    let new_card: Card = Card {
        card: String::from("A"),
        class: CardSuit::Club,
    };

    println!("{:?}", first_card);
    println!("{:?}", new_card);
    println!("{:#?}", second_card);
    println!("{:#?}", card_details);
    println!("{:#?}", card_array);

    #[derive(Debug)]
    enum PaymentMethodType {
        CreditCard(String),
        DebitCard(String),
        PayPal(String, String),                         // Tuple type Variant
        Revolut { username: String, password: String }, // Struct type variants
    }
    let my_debit: PaymentMethodType =
        PaymentMethodType::DebitCard(String::from("1234-1236-1236-1254"));

    let my_pay: PaymentMethodType =
        PaymentMethodType::CreditCard(String::from("1236-1256-1598-0025"));

    let my_paypal = PaymentMethodType::PayPal(String::from("abc@q.com"), String::from("letmepay"));

    let my_revolut: PaymentMethodType = PaymentMethodType::Revolut {
        username: String::from("Asd@q.com"),
        password: String::from("iampass"),
    };

    println!("{:?}", my_pay);
    println!("{:?}", my_paypal);
    println!("{:?}", my_debit);
    println!("{:?}", my_revolut);

    // Lets see other example *******************************************************************************************************

    #[derive(Debug)]
    enum Meat {
        Chicken,
        Steak,
    }
    #[derive(Debug)]
    enum RestuarentItem {
        Burrito(Meat),
        Bowl(Meat),
        Veganplate,
    }

    let lunch: RestuarentItem = RestuarentItem::Burrito(Meat::Chicken);
    let dinner: RestuarentItem = RestuarentItem::Bowl(Meat::Steak);
    let starter: RestuarentItem = RestuarentItem::Veganplate;
    println!(
        "Dinner: {:?} | Lunch: {:?} | Starter: {:?}",
        dinner, lunch, starter
    );

    #[derive(Debug)]
    enum Size {
        Small,
        Medium,
        Large,
    }

    #[derive(Debug)]
    enum ConcessionItem {
        Popcorn(Size),
        Drink(Size),
        Hotdog,
    }
    #[derive(Debug)]
    enum Combo {
        Combo(ConcessionItem, ConcessionItem),
    }

    let snack: ConcessionItem = ConcessionItem::Popcorn(Size::Large);
    let beverage: ConcessionItem = ConcessionItem::Drink(Size::Small);
    let side: ConcessionItem = ConcessionItem::Hotdog;
    let combo_drink: Combo = Combo::Combo(
        ConcessionItem::Drink(Size::Medium),
        ConcessionItem::Popcorn(Size::Medium),
    );

    println!(
        "Snack: {:?} | Beverage: {:?} | Side:{:?}",
        snack, beverage, side
    );

    println!("ComboMenu : {:?}", combo_drink);

    // Lets learn Match patterns with enums.********************************************************************************

    #[derive(Debug)]
    enum OperatingSystem {
        Windows,
        MacOs,
        Linux,
    }

    fn yrs_since_release(os: OperatingSystem) -> i32 {
        match os {
            OperatingSystem::Windows => 40,
            OperatingSystem::MacOs => 23,
            OperatingSystem::Linux => {
                // example of executing block of code with match enum.
                println!("A fantastic Operating system");
                39
            }
        }
    }

    let my_os_yrs: OperatingSystem = OperatingSystem::Linux;
    let age = yrs_since_release(my_os_yrs);
    println!("{}", age);

    let my_frnd_os: OperatingSystem = OperatingSystem::Linux;
    let fnd_os_yr: i32 = yrs_since_release(my_frnd_os);
    println!("Friend OS : {}", fnd_os_yr);

    // Lets see another example with enum with match. **************************************************************************

    #[derive(Debug)]
    enum LaundryCycle {
        Cold,
        Hot { temp: i32 },
        Delicate(String, String),
    }

    // Lets create an wash_laundry fn.

    fn wash_laundry(cycle: LaundryCycle) {
        match cycle {
            LaundryCycle::Cold => {
                println!("Washing in Cold temp");
            }

            LaundryCycle::Hot { temp } => {
                println!("Running in {} temp.", temp);
            }

            LaundryCycle::Delicate(clothes, ctemp) => {
                println!("Washing {} Clothes at {} temp", clothes, ctemp);
            }
        }
    }

    let wash: LaundryCycle = LaundryCycle::Cold;
    wash_laundry(wash);
    wash_laundry(LaundryCycle::Delicate(
        String::from("Silk"),
        String::from("30 degree"),
    ));
    wash_laundry(LaundryCycle::Hot { temp: 60 });

    // Lets define methods on top of enums
    // Practise Traffic light System ***************************************************************************************

    #[derive(Debug)]
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    impl TrafficLight {
        fn duration(&self) -> u32 {
            match self {
                TrafficLight::Red => 30,
                TrafficLight::Yellow => 5,
                TrafficLight::Green => 20,
            }
        }

        fn instruction(&self) -> &str {
            match self {
                TrafficLight::Red => "Stop",
                TrafficLight::Yellow => "Slow Down",
                TrafficLight::Green => "Go",
            }
        }

        fn show_info(&self) {
            let duration = self.duration();
            let instruction = self.instruction();
            let next_light = self.next();
            println!(
                "{:?}: {} | Duration: {}s | Next: {:?} ",
                self, instruction, duration, next_light
            );
        }

        fn next(&self) -> &str {
            match self {
                TrafficLight::Red => "Green",
                TrafficLight::Green => "Yellow",
                TrafficLight::Yellow => "Red",
            }
        }
    }

    TrafficLight::Red.show_info();
    TrafficLight::Yellow.show_info();
    TrafficLight::Green.show_info();

    // ***********************************************Lets see one more example.**********************************************

    #[derive(Debug)]
    enum OnlineOrder {
        Ordered,
        Packed,
        Shipped,
        Delivered,
    }

    impl OnlineOrder {
        fn check(&self) {
            match self {
                OnlineOrder::Ordered => {
                    println!("Your item has been ordered.");
                }
                // _ => {
                //     println!("Your item is still in processing"); // This catches all the remaining left.
                // }
                orderstatus => {
                    // This is better approach since it can show the varaint.
                    println!("it's still in processing, Query for: {:?}", orderstatus);
                }
            }
        }
    }

    OnlineOrder::Delivered.check();

    // ***********************-  Lets Practise other example with match with exact value. **************************************

    enum Milk {
        Lowfat(i32),
        Whole,
    }

    impl Milk {
        fn drink(&self) {
            match self {
                Milk::Lowfat(percent) => {
                    println!(" I have got {} percent fat in my milk.", percent)
                }
                Milk::Lowfat(2) => println!("I bought milk with 2 percent fat"),
                Milk::Whole => println!("I've got whole milk."),
            }
        }
    }

    Milk::Lowfat(20).drink();
    Milk::Lowfat(2).drink();
    Milk::Whole.drink();

    // Lets practise if let and let else now******************************************************************************.

    enum Milky {
        Lowfat(i32),
        Whole,
        Nondairy { kind: String },
    }

    let my_drink: Milky = Milky::Whole;
    if let Milky::Lowfat(2) = my_drink {
        println!("Oh yeah");
    } else {
        println!("Nothing");
    }

    if let Milky::Lowfat(percent) = Milky::Lowfat(2) {
        println!("I have {}percent lowfat milk", percent);
    }

    if let Milky::Nondairy { kind } = (Milky::Nondairy {
        kind: String::from("oats"),
    }) {
        println!("I have {} milk.", kind);
    }

    // ************************let else examples*******************************************
    //The rule of thumb:
    //Use let else when a mismatch should genuinely stop the function
    //Use if let or match when you just want to handle both cases and continue

    let Milky::Lowfat(percent) = Milky::Lowfat(3) else {
        println!("I am running becoz let else condition is false. ");
        return;
    };
    println!("{percent}");

    let Milky::Nondairy { kind } = (Milky::Nondairy {
        kind: String::from("oats"),
    }) else {
        println!("I am running becoz the condition is false.");
        return;
    };

    println!("I got {} milk", kind);
    /* 
    let Milky::Nondairy { kind } = Milky::Whole else {
        println!("I am running becoz the condition is false.");
        return;
    };
    println!("{}", kind);  
    
    
    */

    // Let do some practise..................................................................
    #[derive(Debug)]
    enum Tier {
        Gold,
        Silver,
        Platinum,
    }

    #[derive(Debug)]
    enum Subscription {
        Free,
        Basic(f64, u32),
        Premium { tier: Tier },
    }

    impl Subscription {
        fn summarize(&self) {
            match &self {
                Subscription::Free => println!("You have limited access to the site."),
                Subscription::Basic(price, months) => println!(
                    "You have limited access to the site's premium features for {:?} for {:?} months.",
                    price, months
                ),
                Subscription::Premium { tier } => {
                    println!(
                        "You have full access to the site's premium features. Your tier is {:?}.",
                        tier
                    )
                }
            }
        }
    }

    Subscription::Free.summarize();
    Subscription::Basic(99.99, 6).summarize();
    Subscription::Premium {
        tier: Tier::Platinum,
    }
    .summarize();
}
