pub fn structure() {
    struct Drink {
        // This is called struct defination.
        name: String,
        price: f64,
        is_hot: bool,
    }

    let mut mocha: Drink = Drink {
        // This is called struct instantiation.
        name: String::from("Coffee"),
        price: 6.99,
        is_hot: true,
    };

    println!(
        "I had a {} in the morning from starbucks which was hot: {} and cost {}.",
        mocha.name, mocha.is_hot, mocha.price
    );

    // We mutate the field value inside struct with . method.

    mocha.name = String::from("Hot-Chocolate");
    mocha.price = 4.99;

    println!(
        "I had a {} in the morning from starbucks which was hot: {} and cost {}.",
        mocha.name, mocha.is_hot, mocha.price
    );

    fn make_drink(name: String, price: f64, is_hot: bool) -> Drink {
        // This is a free function or associated funtion.
        Drink {
            name,
            price,
            is_hot,
        }
    }

    let my_drink: Drink = make_drink(String::from("Whisky"), 29.99, false);
    println!(
        "I had a {} in the Night from Ale-Club which was hot was {} and cost {}.",
        my_drink.name, my_drink.is_hot, my_drink.price
    );

    let new_drink = make_drink(String::from("Vodka"), 45.99, false);
    println!(
        "I had a {} in the Night from Ale-Pub which was hot was {} and cost {}.",
        new_drink.name, new_drink.is_hot, new_drink.price
    );

    // lets do some practise.

    struct Book {
        title: String,
        author: String,
        pages: u32,
        is_available: bool,
    }

    let mut first_book = Book {
        title: String::from("Rust"),
        author: String::from("Rust-man"),
        pages: 399,
        is_available: false,
    };

    let second_book = Book {
        title: String::from("Rustaceans"),
        author: String::from("Rustybits"),
        pages: 789,
        is_available: true,
    };

    fn create_book(title: String, author: String, pages: u32, is_available: bool) -> Book {
        Book {
            title,
            author,
            pages,
            is_available,
        }
    }

    let mut third_book: Book = create_book(
        String::from("Learning Rust"),
        String::from("Rustee"),
        745,
        false,
    );
    third_book.is_available = true;
    third_book.pages = 987;

    fn print_book(book: &Book) {
        println!(
            "{} by {} | Pages: {} | Available {}",
            book.title, book.author, book.pages, book.is_available
        );
    }

    print_book(&first_book);
    print_book(&second_book);
    print_book(&third_book);

    fn is_long_book(book: &Book) -> bool {
        book.pages > 300
    }
    fn checkout_book(book: &mut Book) -> bool {
        book.is_available = false;
        book.is_available
    }
    println!("is long book? {}", is_long_book(&first_book));
    println!("Is book Available? {}", checkout_book(&mut first_book));

    // let s see some more examples.
    #[derive(Debug)]
    struct MyFavouriteSong {
        title: String,
        release_year: i32,
        artist: String,
        duration: i32,
    }

    let song = MyFavouriteSong {
        title: String::from("It will rain"),
        release_year: 2012,
        artist: String::from("Bruno Mars"),
        duration: 400,
    };

    let new_song = MyFavouriteSong {
        title: String::from("Talking to the moon"),
        release_year: 2013,
        artist: String::from("Bruno Mars"),
        duration: 500,
    };

    let album_song = MyFavouriteSong {
        title: String::from("Grenade"),
        release_year: 2016,
        artist: String::from("Bruno Mars"),
        duration: 700,
    };

    let mut favourite_song = MyFavouriteSong {
        title: String::from("When we were young"),
        release_year: 2015,
        artist: String::from("Adele"),
        duration: 700,
    };

    impl MyFavouriteSong {
        // This ís an example of immutable struct value & value is moved.
        fn display_info(self) {
            println!(
                "Song title:{} | Realese year: {} | Artist: {} | Duration {}",
                self.title, self.release_year, self.artist, self.duration
            );
        }

        //Mutable Struct value( Self parameter takes ownership and can be mutable)
        fn change_duration_to_mins(mut self) {
            self.duration /= 60;
            println!("{} mins ", self.duration);
        }

        // Immutable reference to thwe struct instance (no ownership moved)

        fn display_artist(&self) {
            println!("Artist name: {}", self.artist);
        }

        // Mutable reference to the struct instance (no ownership moved but is mutable)
        fn display_release_year(&mut self) {
            println!("Release year {} | by {}", self.release_year, self.artist);
        }

        // Methods with Multiple Parameters.

        fn is_newer_than(&self, other: &Self) {
            if self.release_year > other.release_year {
                println!(
                    "latest song Release {} | Album name {} | Years since releaesed :{}",
                    self.release_year,
                    self.title,
                    self.yrs_since_released(),
                );
            } else if self.release_year < other.release_year {
                println!(
                    "latest song Release {} | Album name {} | Years since released: {}",
                    other.release_year,
                    other.title,
                    other.yrs_since_released(),
                );
            } else {
                println!(" Both song released at same {} year.", self.release_year,);
            }
        }
        // Calling method from other method.
        fn yrs_since_released(&self) -> i32 {
            2026 - self.release_year
        }

        // lets do now constructor function or also called associated funtion, This creates new Struct instance.
        fn new(title: String, release_year: i32, artist: String, duration: i32) -> Self {
            Self {
                title,
                release_year,
                artist,
                duration,
            }
        }
    }
    let song1: MyFavouriteSong =
        MyFavouriteSong::new(String::from("Hello"), 2015, String::from("Adele"), 650);

    song.display_info();
    // println!("{:#?}", song); remember this call error bcoz the value of self moved to self parameter.
    new_song.change_duration_to_mins();
    album_song.display_artist();
    favourite_song.display_release_year();
    album_song.is_newer_than(&favourite_song);
    song1.is_newer_than(&favourite_song);

    // Lets see some builder pattern.
    #[derive(Debug)]
    struct Computer {
        brand: String,
        cpu: String,
        memory: i32,
        ssd: i32,
    }

    impl Computer {
        fn new(brand: String, cpu: String, memory: i32, ssd: i32) -> Self {
            Self {
                brand,
                cpu,
                memory,
                ssd,
            }
        }
        fn change_brand(&mut self, new_brand: String) -> &mut Self {
            self.brand = new_brand;
            self
        }
        fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
            self.cpu = new_cpu;
            self
        }

        fn upgrade_memory(&mut self, new_memory: i32) -> &mut Self {
            self.memory = new_memory;
            self
        }
        fn upgrade_ssd(&mut self, new_ssd: i32) -> &mut Self {
            self.ssd = new_ssd;
            self
        }
    }

    let mut my_computer: Computer =
        Computer::new(String::from(" MAcbook"), String::from("M1 Pro"), 16, 500);
    println!("{:#?}", my_computer);

    my_computer
        .change_brand(String::from("Macbook Pro M6"))
        .upgrade_cpu(String::from("M6 Pro"))
        .upgrade_memory(32)
        .upgrade_ssd(1000);

    println!(
        "
    Computer Brand: {} CPU: {} |Memory: {} GB | SSD: {} GB
    ",
        my_computer.brand, my_computer.cpu, my_computer.memory, my_computer.ssd,
    );
    println!("{:#?}", my_computer);
}
