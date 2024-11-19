use std::env;
use std::io;
use std::io::Write;

fn convert_26_to_9(input_string: &str) -> String {
    let mut result = String::new();
    let mapping: [(&str, &str); 8] = [
        ("abc", "2"),
        ("def", "3"),
        ("ghi", "4"),
        ("jkl", "5"),
        ("mno", "6"),
        ("pqrs", "7"),
        ("tuv", "8"),
        ("wxyz", "9"),
    ];

    for char in input_string.chars() {
        if char.is_alphabetic() {
            for &(ref keys, ref value) in &mapping {
                if keys.contains(char) {
                    result.push_str(value);
                    break;
                }
            }
        } else {
            result.push_str("1");
        }
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_str: String;
    if args.len() > 1 {
        input_str = args[1].clone();
    } else {
        print!("输入要转换的密码:");
        std::io::stdout().flush().unwrap();
        let mut input_str_console = String::new();
        io::stdin()
            .read_line(&mut input_str_console)
            .expect("读取输入失败");

        input_str = input_str_console.trim().to_string();
    }

    let output_str = convert_26_to_9(&input_str);
    println!("{}", output_str);
}