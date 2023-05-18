use std::io;

fn main() {
    print!("Give me an integer in base 10 or in Roman numeral format: ");
    io::Write::flush(&mut io::stdout()).expect("Flush failed!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");

    let test = &input.trim().parse::<i32>();
    match test {
        Ok(ok) => integer_to_roman(ok),
        Err(_) => roman_to_integer(input.to_uppercase().as_str()),
    }
}

fn integer_to_roman(num: &i32) {
    let mut new_num = *num;

    if new_num < 1 {
        println!("Integer input cannot be below 1. Please try again.");
        main()
    } else {
        const VALUES_LIST: [i32; 13] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        const LETTERS_LIST: [&str; 13] = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];

        let mut result = String::from("");

        for (i, _) in VALUES_LIST.iter().enumerate() {
            while new_num >= VALUES_LIST[i] {
                new_num -= VALUES_LIST[i]; result.push_str(LETTERS_LIST[i]);
            }
        }

        println!("Your number in Roman numerals is \"{}\".", result)
    }
}

fn roman_to_integer(str: &str) {
    let mut test: bool = true;

    for char in str.chars() {
        if char != 'I' && char != 'V' && char != 'X' && char != 'L' && char != 'C' && char != 'D' && char != 'M' && char != '\n' {
            test = false; break;
        }
    }

    if !test {
        println!("Input contains non-Roman-numeric characters. Please try again.");
        main()
    } else {
        let mut result = 0;

        for (i, char) in str.chars().enumerate() {
            let num = translate(char);
            if i + 1 < str.len() {
                let num2 = translate(str.chars().nth(i + 1).unwrap());
                if num >= num2 {
                    result += num;
                } else {
                    result -= num;
                }
            } else {
                result += num;
            }
        }

        println!("Your number in base 10 is \"{}\".", result)
    }
}

fn translate(letter: char) -> i32 {
    match letter {
        'M' => 1000,
        'D' => 500,
        'C' => 100,
        'L' => 50,
        'X' => 10,
        'V' => 5,
        'I' => 1,
        _ => 0
    }
}