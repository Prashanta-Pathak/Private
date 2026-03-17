//#[derive(Debug)]
pub fn structure() {
    struct Drink {
        name: String,
        price: f64,
        is_hot: bool,
    }

    let mut mocha: Drink = Drink {
        name: String::from("Coffee"),
        price: 6.99,
        is_hot: true,
    };

    println!(
        "I had a {} in the morning from starbucks which was hot was {} and cost {}.",
        mocha.name, mocha.is_hot, mocha.price
    );

    // We mutate the field value inside struct with . method.

    mocha.name = String::from("Hot-Chocolate");
    mocha.price = 4.99;
    println!(
        "I had a {} in the morning from starbucks which was hot was {} and cost {}.",
        mocha.name, mocha.is_hot, mocha.price
    );

    fn make_drink(name: String, price: f64, is_hot: bool) -> Drink {
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
}
