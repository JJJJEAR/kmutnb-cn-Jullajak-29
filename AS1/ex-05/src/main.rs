use std::io;
use std::io::Write;
fn main() {
    let mut input = String::new();
    print!("Input : x = ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read!!!");
    let x:i32=input.trim().parse().expect("Please Type A Number!!!");
    
    
    for i in 1..=x{
        for k in 0..=x - 1{
            if k == 0 || k == x - 1{
                print!("X ");
            }else if k == (i-1){
                print!("X ");
            }else{
                print!("O ");
            }
            
        }
        println!();
    }

}