use std::io;
use std::io::Write;
fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }

    true
}
fn main() {
    let mut input = String::new();
    print!("Enter number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read!!!");
    let x:i32=input.trim().parse().expect("Please Type A Number!!!");

    print!("{} =",x);
    let mut cal:i32 = x;
    if is_prime(x){
        print!("{}", x);
        println!();
        
    }else{
       
        let mut prime = 2;
        loop {
            if is_prime(prime){
               if is_prime(cal) || cal <= 2{
                print!(" {}", cal);
                break;
                }else if cal % prime == 0 {
                    cal = cal / prime;
                    print!(" {} *", prime);
                }else{
                    prime = prime + 1;
                }  
            }
           else{
                prime = prime + 1;
            }
        }
    }
}