use std::fs::File;
use std::io::{Write, BufWriter};

const FROM_NUMBER: i32 = 0;
const TO_NUMBER: i32 = 50;
const SIGNS: [char; 4] = ['+', '-', '/', '*'];

fn main() {
    let file = create_file();
    generate(&mut BufWriter::new(file));
}

fn create_file() -> File {
    File::create("./calculator.py").expect("Couldn't create file!")
}

fn generate(file: &mut BufWriter<File>) {
    let starting_data = format!(
        "print(\"welcome to the (best) calculator!\")
print(\"I can calculate numbers, between {} and {}\")
num1 = int(input(\"Choose your first operand: \"))
sign = input(\"Choose your operator (+ - / *): \")
num2 = int(input(\"Choose your second operand: \"))\n
", FROM_NUMBER, TO_NUMBER);
    
    write!(file, "{}", starting_data).expect("Failed to write: ERROR");
    for sign in SIGNS {
        for num1 in FROM_NUMBER..TO_NUMBER {
            for num2 in FROM_NUMBER..TO_NUMBER {
                let equals = match sign {
                    '+' => (num1 + num2).to_string(),
                    '-' => (num1 - num2).to_string(),
                    '/' => if num1.checked_div(num2).is_none() { String::from("ERROR") } else { (num1 / num2).to_string() },
                    '*' => (num1 * num2).to_string(),
                    _ => unreachable!(),
                };
                let if_statement = format!("if num1 == {} and sign == '{}' and num2 == {}:\n", num1, sign, num2); // 44
                write!(file, "{}", if_statement).expect("Failed to write if statements");
                let result = format!("  print(\"{}{}{} = {}\")\n", num1, sign, num2, equals); // 18
                write!(file, "{}", result).expect("Failed to write print statements");
            }
        }
    }
}