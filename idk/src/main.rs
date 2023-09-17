use std::{ env, fs };
#[derive(Debug)]
struct TOKENS {
    Literal: usize,
    PRINT: usize,
    SEMI: usize,
    LEFT_PRAM: usize,
    RIGHT_PRAM: usize,
}
fn main() {
    let mut array: Vec<String> = vec![String::new(); 126];
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    let file_name: String = args[1].clone();
    let contents = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let my_vec: Vec<char> = contents.chars().collect();
    let _tokens1 = TOKENS {
        Literal: 1,
        PRINT: 1,
        SEMI: 1,
        LEFT_PRAM: 1,
        RIGHT_PRAM: 1,
    };
    for index in 0..14 {
        let mut check = "";
        println!(" Value {}", my_vec[index]);
        if my_vec[index] == ' ' || my_vec[index] == ';' {
            println!(" BREAKING");
            break;
        }
    }
}
