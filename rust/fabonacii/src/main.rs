use std::io;

fn main() {
    loop {
        println!("Please input your number.");
        let mut number = String::new();

        io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
        
        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}", fabonacii(number));
        println!("Next");           
    }
}

fn fabonacii(n: u32) -> u32 {
    let mut a = 1;
    let mut b = 1;
    if n <= 2 {
        a
    }
    else {
        for _i in 2..n {
           let c = a + b;
           a = b;   
           b = c; 
        }

        b
    }
}
