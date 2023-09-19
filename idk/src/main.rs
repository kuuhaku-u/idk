use std::{ env, fs };
#[derive(Debug)]
struct TOKENS {
    Literal: usize,
    print: usize,
    SEMI: usize,
    LEFT_PRAM: usize,
    RIGHT_PRAM: usize,
}
impl TOKENS {
    // Function to check if 'check' is in the struct.
    fn contains(&self, check: &str) -> bool {
        check == "Literal" || check == "print" || check == ";" || check == "(" || check == ")"
    }
    fn is_string(&self, check: &str) -> bool {
        check == '"'.to_string()
    }
}
fn main() {
    // Create a vector of empty strings with a capacity of 126.
    let mut array: Vec<String> = vec![String::new(); 126];
    // Collect command-line arguments into a Vec<String>.
    let args: Vec<String> = env::args().collect();
    // Check if a command-line argument was provided.
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    // Get the filename from the command-line arguments.
    let file_name: &String = &args[1];
    // Read the contents of the file into a String.
    let contents = fs::read_to_string(file_name).expect("Unable to read the file");
    // Convert the contents to a Vec<char>.
    let my_vec: Vec<char> = contents.chars().collect();
    // Define TOKENS struct (unused for now).
    let tokens = TOKENS {
        Literal: 1,
        print: 1,
        SEMI: 1,
        LEFT_PRAM: 1,
        RIGHT_PRAM: 1,
    };
    // Iterate over the first 14 characters in my_vec.
    let mut check = String::new(); // Use String::new() to create an empty string.
    let flag = false;
    let mut printWhat = String::new(); // Use String::new() to create an empty string.
    for index in 0..120 {
        check.push(my_vec[index]); // Use push method to append a character.
        if tokens.contains(&check) {
            // println!("'{}' is present in the TOKENS struct.", check);
            check.clear();
        }
        if check.starts_with('"') && !check.ends_with('"') {
            // println!("'{}' is present in the TOKENS struct.", printWhat);
            if check.ends_with(')') {
                // flag
                check.clear();
            }
            printWhat.push(my_vec[index]); // Use push method to append a character.
        }
        println!("Value {}", printWhat);
        // else {
        // println!("'{}' is not present in the TOKENS struct.", check);
        // }
        // if check == "print" {
        //     println!("FOUND it {}", check);
        // //     // check = "".to_string();
        // //     println!("FOUND it {}", check);
        //     }
        // }
        if my_vec[index] == ' ' || my_vec[index] == ';' {
            println!("BREAKING");
            break;
        }
    }
}
