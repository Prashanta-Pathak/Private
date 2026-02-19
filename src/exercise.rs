/*
Declare a `season` variable set to a string with
your favorite season. Provide an explicit type annotation.
The type of a string is a `&str`. We'll discuss what
the & symbol means later in the course.

Declare a `points_scored` variable set to 28.
Provide an explicit type annotation. The type of
an integer is `i32`.

It's time to update the team's score. Declare the
`points_scored` variable to be mutable. Set its
new value to 35.

Declare a `TOUCHDOWN_POINTS` constant at the file
level set to the value 6.

Declare a `event_time` variable set to a string of
"06:00".

Use variable shadowing to redeclare `event_time` set
to a integer of 6.

Use interpolation to print out all of the
declared variables and constants in a println! call.
Practice using direct interpolation {value}, sequential
arguments ( {} ), and numeric arguments ( {0} ).

Declare a `favorite_beverage` variable set to a string
of your favorite drink. Use an underscore to silence
the compiler warning about the variable being unused.

Remove the underscore. Provide a compiler directive
to silence the compiler warning about the variable
being unused.
*/
#![allow(unused_variables)]

const TOUCHDOWN_POINTS: i32 = 6;

pub fn exercise() {
    let season: &str = "Basketball";
    #[allow(unused_assignments)]
    let mut points_scored: i32 = 28;
    points_scored = 35; // value is muted here.
    let event_time: &str = "6:00"; // value nomore exists bcoz of shadowing😂.

    let event_time: i32 = 6;

    println!(
        "My Favourite sports season is {0}, Points scored in the match was {1} and Touchdown points was {2}. The event started at {3}.",
        season, points_scored, TOUCHDOWN_POINTS, event_time
    );

    let favourite_drink: &str = "Olut";

    // Exercise 2

    let underscore_number: i32 = 1_337;
    let number: i16 = underscore_number as i16; // perfoirming casting.
    let my_floating_number: f64 = 3.2145658;
    println!(
        "The floating number with 3 digits of pricision is {:.3}", // prints 3 digit precision sfter decimal.
        my_floating_number
    );

    let with_milk: bool = true;
    let with_sugar: bool = true;

    let is_my_type_of_coffee: bool = with_milk && with_sugar;
    let is_acceptable_coffee: bool = with_milk || with_sugar;

    let int_array: [i8; 4] = [1, 2, 3, 4];
    println!("{:?}", int_array);

    // tuple declaration
    let values: (i16, f64, bool, bool, [i8; 4]) =
        (number, my_floating_number, with_milk, with_sugar, int_array);

    println!("{values:?}");
}
