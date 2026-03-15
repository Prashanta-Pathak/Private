pub fn ownership() {
    let is_concert: bool = true;
    let is_event: &bool = &is_concert; //Referencing the value of is_concert also called borrowing.
    println!("{is_concert}");
    println!("{is_event}");

    let sushi: String = String::from("Salmomn");
    let dinner: String = sushi;
    println!("{dinner}");
    // println!("{sushi}");

    let sushi: &str = "Salmomn";
    // let dinner: &str = sushi;
    println!("{sushi}");
    println!("{dinner}");

    let find_dinner = eat_meal(dinner);
    println!("{find_dinner}");

    fn eat_meal(mut meal: String) -> String {
        meal.clear(); // clears the variable value
        meal.push_str("Pizza Ready"); // push the new value
        meal
    }
    /*********************************************************************************************************/
    // lets learn mutubale and immutable reference now.
    // Using referencing method we dont loose the value used by function and does not to save funtion call to the variable.

    // Parameter syntax to remember...
    // 1. meal: String -> Passing String where ownership moves to parameter.
    // 2. mut meal: String -> Passing mutable String where ownership moves to parameter.
    // 3. meal: &String -> Passing reference to parameter where ownership does not move.
    // 4. meal: &mut String -> Passing mutable reference to parameter where ownership does not move.

    let mut my_meal: String = String::new();

    fn display_meal(meal: &mut String) {
        // here we are passing mutable reference to the function.
        meal.push_str("Chicken Burger meal");
    }

    fn give_meal(meal: &String) {
        // here we are passing immutable references to the function.
        println!("{meal}");
    }

    display_meal(&mut my_meal);
    give_meal(&my_meal);

    /*****************************Exercise*************************

        fn start_trip() -> String {
            String::from("The Plan is ")
        }

        fn visit_philadelphia(location: &mut String) {
            location.push_str("Philadephia");
        }

        fn visit_new_york(location: &mut String) {
            location.push_str("Newyork");
        }

        fn visit_boston(location: &mut String) {
            location.push_str("Boston");
        }

        let mut trip: String = start_trip();

        visit_philadelphia(&mut trip);
        trip.push_str(" and ");

        visit_new_york(&mut trip);
        trip.push_str(" and ");

        visit_boston(&mut trip);
        trip.push('.');

        println!("{trip}");
    */

    fn start_trip() -> String {
        String::from("The Plan is ")
    }

    fn visit_philadelphia(location: &mut String) -> &mut String {
        location.push_str("Philadelphia"); // Fixed spelling
        location
    }

    fn visit_new_york(location: &mut String) -> &mut String {
        location.push_str("New York"); // Added space
        location
    }

    fn visit_boston(location: &mut String) -> &mut String {
        location.push_str("Boston");
        location
    }

    let mut trip: String = start_trip();

    // Each step "consumes" the old string and returns a new one
    let plan: &mut String = visit_philadelphia(&mut trip);
    plan.push_str(" and ");

    let next_plan: &mut String = visit_new_york(plan);
    next_plan.push_str(" and ");

    let final_plan: &mut String = visit_boston(next_plan);
    final_plan.push('.');

    println!("{final_plan}");
}
