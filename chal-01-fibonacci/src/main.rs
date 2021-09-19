use std::io;

fn fibonacci(n: u32) -> u32 {

    let mut i: u32 = 0;

    // first two numbers
    let mut x: [u32; 2] = [0, 1];
    
    while i < n {
        let tmp = x[0];
        x[0] = x[1];
        x[1] += tmp;
        i += 1;
    }
    
    x[1]
}    

fn main() {

    loop {
        let mut number = String::new();
        println!("Enter a number...");
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");
        
        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let x = if number > 0 { fibonacci(number-1) } else { 0 };
        println!("the fibonacci number is: {}", x);
        break;
    }
}


    



