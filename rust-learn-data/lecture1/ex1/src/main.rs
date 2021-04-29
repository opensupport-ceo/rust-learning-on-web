fn main() {
    let mut num = 1;
    let mut sum = 0;

    while num <= 100 {
        
        sum += num;
        println!("the sum value is: {:?} after adding value {:?}", sum, num);

        num = num + 1;
    }

    println!("the final sum value is: {}", sum);
}
