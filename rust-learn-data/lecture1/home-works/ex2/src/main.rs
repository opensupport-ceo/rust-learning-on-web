use std::io;
use std::str;

fn main() {
    loop {
        println!("Please input your string.");
        let mut instr = String::new();

        io::stdin().read_line(&mut instr)
            .expect("Failed to read line");

        print!("Reversed string is: ");

        
        //let mut len = str.len() as usize;
        let bytes = instr.as_bytes();
        let mut len = bytes.len();

        while len != 0 {
            if let Ok(s) = str::from_utf8(&[bytes[len-1]]) { // byte 배열의 char 값을 string을 출력
                print!("{}", s);
            }

            len = len - 1;
        }
        println!("\n");
    }
}
