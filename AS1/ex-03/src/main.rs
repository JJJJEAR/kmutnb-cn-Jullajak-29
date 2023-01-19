use std::io;
use std::io::Write;
fn main() {
    let mut input = String::new();
    print!("Input : x = ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read!!!");
    let x :i32=input.trim().parse().expect("Please Type A Number!!!");
    
    for i in 1..=x{
        for _ in 0..=i-1{
            print!("  ");
        }
        for _ in 0..=(xr-i)*(2){
            print!("* ");
        }
        println!();
    }

}
