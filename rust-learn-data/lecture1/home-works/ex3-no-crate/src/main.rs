use std::{io, u64};

fn main() { 

    println!("Program start!");

    loop {
        println!("Please input your number of 20 or less digits.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let input_num: u64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: you have to input number of 20 or less digits!");
                continue
            },
        };

        //
        //예를 들어 1234567 을 터미널에서 입력했으면 000000000001,234,567 와 같이 출력하세요.
        //
        let input_string_num = input_num.to_string();
        let mut output_string_num = String::new();

        let len = input_string_num.chars().count(); // note: String.len() means count in bytes.

        let mut i = 0; // for from input
        let mut k = 0; // for to output

        while i < len  {
           //let b: u8 = input_string_num.as_bytes()[i];
           //let c: char = b as char;
           output_string_num.insert(k, input_string_num.as_bytes()[i] as char);

           if len > 3 {
                /*
                k = k + 1;
                if i == (len - 1) {
                    break;
                }*/

                if (i%3 == 2) && (len % 3 == 0){ //ex: 123456 => 123,456
                    k = k + 1;
                    if i == (len - 1) {
                        break;
                    }
                    output_string_num.insert(k, ',');

                } else if (i%3 == 0) && (len % 3 == 1) { // ex: 1234567 => 1,234,567
                    k = k + 1;
                    if i == (len - 1) {
                        break;
                    }
                    output_string_num.insert(k, ',');

                } else if (i%3 == 1) && (len % 3 == 2){ // ex: 12345678=> 12,345,678
                    k = k + 1;
                    if i == (len - 1) {
                        break;
                    }
                    output_string_num.insert(k, ',');
                }
           }
           
           i = i + 1;
           k = k + 1;
        }

        println!("The result is {:0>width$}", output_string_num, width = 20);

    }

}
