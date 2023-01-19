fn main() {
    let n = 100;
    let sum = ((n)*(n+1))/2;
    println!("1 + 2 + 3 + ... + {} = {}", n, sum);
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test() {
       let n = 100;
       let sum =((n)*(n+1))/2;
       assert_eq!(sum,5050)
   }
}