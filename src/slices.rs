pub fn slices() {
    let name: String = String::from("Rajesh Hamal");
    let first_name: &str = &name[0..6];
    let hero_name: &str = &name[..];
    println!("The first name of Hero is {first_name}.");
    println!("The fullname of hero is {hero_name}.");

    let last_name: &str = &name[7..];
    println!("The last name is {last_name}.");

    let myname: &str = "Prashanta";
    let my_lastname: &str = "Pathak";
    println!("{},{}", myname.len(), my_lastname.len());

    // Example of String slices as Functional Paramenters
    // here we pass refstring as parameter.
    fn string_slice(something: &String) {
        println!("The output is {something}.");
    }
    let name_to_be: String = String::from("Example");
    let given_name: &String = &name_to_be;
    string_slice(given_name);

    let another_value: &str = &name_to_be;
    //string_slice(another_value); // As we can, this creates error, we cannot pass &str value type to &String parameter but
    // we can do opposite of it if we change parameter value to &str. But for reference i Will create new function to represent.
    fn str_slice(parameter: &str) {
        println!("see the result with &str as parameter value type: {parameter}");
    }
    str_slice(another_value);
    str_slice(given_name); // this value is &String type but it can be exucated since &String -> &str is possible.

    // Now lets look at some Array Slicing.

    let values: [i32; 7] = [1, 2, 3, 5, 6, 7, 9];
    let new_value: &[i32; 7] = &values;
    print_array(values);
    print_array_slice(new_value); // this gives us little freedom to pass  reference slices-array without worrying its len.

    let slice_value: &[i32] = &values[0..3]; // This line is basically known as array slice.
    println!("The array slice value are {slice_value:?}");
    print_array_slice(slice_value);

    let slice_value: &[i32] = &values[0..6];
    println!(
        "The array slice value are {slice_value:?}and length is {}",
        slice_value.len()
    );
    print_array_slice(slice_value);

    let slice_value: &[i32] = &values[..]; // Here we are slicing all its value of the memory.This is different than borrowing
    println!(
        "The full-array-slice value are {slice_value:?} and length is {}",
        slice_value.len()
    );
    print_array_slice(slice_value);

    let slice_value: &[i32; 7] = &values; // here we are borrowing its value.
    println!(
        "The borrowed-array-slice value are {slice_value:?} and the length is {}",
        slice_value.len()
    );
    print_array_slice(slice_value);
    // Now lets try to pass array types and arrayslice to the function.

    fn print_array(array: [i32; 7]) {
        // here we should pass specigic array type or fixed length array type will inly be exucated.
        println! {"The array elements are {array:?}"};
    }
    fn print_array_slice(array: &[i32]) {
        println! {"The Slice-type-array elements are {array:?}"};
    }

    // lets see now mutable array.

    let mut my_array: [i32; 5] = [2, 3, 7, 8, 9];
    let my_mutable_array_slice: &mut [i32] = &mut my_array[1..5];
    println!("My mutable array slices are {my_mutable_array_slice:?}.");
    my_mutable_array_slice[0] = 1;
    my_mutable_array_slice[1] = 1;
    my_mutable_array_slice[2] = 1;
    my_mutable_array_slice[3] = 1;
    println!("My new mutable array-slices are {my_mutable_array_slice:?}");
    println!("The new values stored in mutable my array are {my_array:?}");

    /*
    Some important notes..
    &values — Borrowing

    Creates a reference to the entire array, typed as &[i32; 7]
    The type is &[i32; 7] — a reference to a fixed-size array
    Rust coerces it into &[i32] here only because of the explicit type annotation
    The reference points to the array as a whole unit


    &values[..] — Full Slice

    Explicitly creates a slice from index 0 to the end
    The type is natively &[i32] — a fat pointer (pointer + length)
    More flexible: slices can point to any portion of memory, not just whole arrays
    This is what you'd naturally pass to functions expecting &[i32]
    */

    // Lets do some exercise projects.

    let mut cereals: [String; 5] = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Froasted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch"),
    ];

    let first_two: &[String] = &cereals[0..2];
    println!("The first two variablel slices are {first_two:?}");

    let mid_three: &[String] = &cereals[1..4];
    println!("The middle threee variable are {mid_three:?}");

    let last_three: &mut [String] = &mut cereals[2..];
    println!("The last threee variable are {last_three:?}");

    last_three[2] = String::from("Lucky Charms");
    println!("The new cereals are {cereals:?}");

    let cookie_crisp: &String = &cereals[0];
    println!("{cookie_crisp}");

    let cookie: &str = &cookie_crisp[0..6];
    println!("{cookie}");

    let cocoa_puffs: &String = &cereals[3];
    let puffs: &str = &cocoa_puffs[6..];
    println!("{puffs}");
}
