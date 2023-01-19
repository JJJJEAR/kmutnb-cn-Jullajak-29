fn main() {
    let word = "this cat this bat this rat";
    let split : Vec<&str> = word.split(" ").collect();
    let uniqed = unique(split);
    println!("{:?}", uniqed);
    let length = uniqed.len();
    println!("{}", length);
}

fn unique(split: Vec<&str>) -> Vec<&str> {
    let mut unique = Vec::new();
    for i in &split {
        if !unique.contains(i) {
            unique.push(i);
        }
    }
    unique
}
#[cfg(test)]
mod tests {
   use super::*;
   #[test]
   fn test() {
       let x = vec!["this", "cat" , "this" ,  "bat" ,  "this" ,  "rat"];
       let y = ["this", "cat" , "bat" ,  "rat"];
       assert_eq!(unique(x), y);
   }
}