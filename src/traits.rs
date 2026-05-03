use std::collections::HashMap;

pub fn traits() {
    trait Accommodation {
        fn get_description(&self) -> String;
        fn book(&mut self, name: &str, nights: u32);
    }

    #[derive(Debug)]
    struct Hotel {
        name: String,
        reservation: HashMap<String, u32>,
    }

    impl Hotel {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                reservation: HashMap::new(),
            }
        }
    }
    impl Accommodation for Hotel {
        fn get_description(&self) -> String {
            format!("{} is the luxuxy place to be.", self.name)
        }
        fn book(&mut self, name: &str, nights: u32) {
            self.reservation.insert(name.to_string(), nights);
        }
    }

    #[derive(Debug)]
    struct Airbnb {
        host: String,
        guests: Vec<(String, u32)>,
    }

    impl Airbnb {
        fn new(host: &str) -> Self {
            Self {
                host: host.to_string(),
                guests: vec![],
            }
        }
    }
    impl Accommodation for Airbnb {
        fn get_description(&self) -> String {
            format!("{} is the pro hotel", self.host)
        }
        fn book(&mut self, name: &str, nights: u32) {
            self.guests.push((name.to_string(), nights))
        }
    }

    // Passing traits as a funtion parameter.....
    fn book_for_one_night<T: Accommodation>(entity: &mut T, guest: &str) {
        entity.book(guest, 1);
    }

    // Trait bound Syntax.......
    fn mix_and_match<T: Accommodation, U: Accommodation>(
        first: &mut T,
        second: &mut U,
        guest: &str,
    ) {
        first.book(guest, 1);
        second.book(guest, 2);
    }

    let mut hotel = Hotel::new("Hilton");
    println!("{}", hotel.get_description());
    book_for_one_night(&mut hotel, "Avi");
    println!("{:#?}", hotel);

    // hotel.book("Hari", 2);

    let mut airbnb = Airbnb::new("Sweethome");
    println!("{}", airbnb.get_description());
    book_for_one_night(&mut airbnb, "hari");
    mix_and_match(&mut hotel, &mut airbnb, "Prashanta");
    println!("{:#?}", airbnb);

    // airbnb.book("Sita", 4);
}
