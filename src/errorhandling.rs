use ::std::fs::{self, File};
use ::std::io::{self, Read, stdin};
use ::std::process;
use core::error;

pub fn errorhand() {
    let file = match File::open("test.txt") {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Sorry, Can't Find a File:{:#?}", error);
            process::exit(1)
        }
    };
    println!("File info:{:#?}", file);

    // lets try adding input from the user.
    /* Lets try to write this in  idoimatic rust way.
    fn read_user_req() {
        println!("Please, Enter the File name!!!");
        let mut file_name = String::new();
        let user_req_file = stdin().read_line(&mut file_name);

        /*We do not need Ok variant to read the file since it returns usize so we
        proceed with if let to verify any error with the user input......

        match user_req_file {
            Ok(file) => println!("The file in usize:{}", file),
            Err(error) => {
                eprint!("Eror occured with status:{}", error);
                process::exit(1)
            }
        }
        */

        // This checks the error in the user input to check the file...
        if let Err(error) = user_req_file {
            eprintln!("Something went wrong collecting the user input:{error}");
            process::exit(1)
        }
        // This finally opens the file..
        let mut file = match File::open(file_name.trim()) {
            Ok(file) => file,
            Err(error) => {
                eprint!("Eror occured with status:{}", error);
                process::exit(1)
            }
        };

        // Reading the file contents
        let mut file_contents = String::new();
        let read_operation = file.read_to_string(&mut file_contents);
        if let Err(error) = read_operation {
            eprintln!("Error readinf the file contents: {error}");
            process::exit(1)
        }
        println!("File info:{:#?}", file);
        println!("File content: {file_contents}");
    }
    read_user_req();
    */

    let file_result = read_file_contents();
    match file_result {
        Ok(string) => println!("File contains :{string}"),
        Err(error) => println!("Error occured, Status:{:#?}", error),
    };

    fn read_file_contents() -> Result<String, io::Error> {
        println!("Please, Enter the File name!!!");

        let mut file_name = String::new();
        stdin().read_line(&mut file_name)?;

        // let mut file_contents = String::new();
        // File::open(file_name.trim())?.read_to_string(&mut file_contents)?;
        // Ok(file_contents)

        fs::read_to_string(file_name.trim()) // -> Simple and effective way.
    }

    // We can also use ? try operator for option enum..lets see with an example.

    let mut my_collection: Vec<&str> = vec!["London", "Norway", "Helsinki", "Nepal"];
    fn show_output(item: &mut Vec<&str>) -> Option<usize> {
        let output: &str = item.pop()?;
        let final_output = output.len();
        Some(final_output)
    }
    println!("{:?}", show_output(&mut my_collection));

    // Exercise.................

    fn write_to_file() -> io::Result<String> {
        println!("What file would you like to write to?");
        let mut file_entry = String::new();
        stdin().read_line(&mut file_entry)?;

        println!("What would you like to write to the file?");
        let mut content_entry = String::new();
        stdin().read_line(&mut content_entry)?;

        fs::write(file_entry.trim(), content_entry.trim())?;
        Ok(file_entry)
    }

    match write_to_file() {
        Ok(val) => println!("Successfully wrote to file {}", val),
        Err(error) => eprintln!("There was an error:{error}"),
    }
}
