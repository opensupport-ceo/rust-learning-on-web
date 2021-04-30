/*
use std::io;

fn main() {
    println!("Program start!");

    loop {
        println!("Please input your number.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let in_num: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: you have to input number!");
                continue
            },
        };

        let len = guess.len() - 1;
        let mut v :Vec<u8> = Vec::new();

        let mut i = 0;
        while i < 20 {
            v.push(0);
            i = i + 1;
        }
        
        i = 0;
        while  i < len {
            v.pop();
            i = i + 1;
        }

        // for c in guess.trim().parse(){
        //     v.push(c);
        // }
       
        guess.

        v.pop();

        println!("The result number is {:?}", v);

        //let s = std::string str(vec.begin(), vec.end());
        let s = match String::from_utf8(v.to_vec()) {
            Ok(path) => Ok(path),
            Err(e) => Err(format!("Invalid UTF-8 sequence: {}", e)),
        };
    
        println!("result: {:?}", s);

    }
}

*/
use std::io;

fn main(){

    println!("Program start!");

    loop {
        println!("Please input your number.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let in_num: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: you have to input number!");
                continue
            },
        };

        println!("The result is {:>0width$}", in_num, width=20);

    }

}