mod scramblegeneration;
use crate::scramblegeneration::generate_scramble;


fn main() {
    for i in 0..6 {
        println!("Generating scramble {}...", i + 1);
        
        let scramble = generate_scramble(20);
        let scramble_str: Vec<String> = scramble.iter().map(|m| m.to_string()).collect();
        println!("Scramble {}: {}", i + 1, scramble_str.join(" "));
    }
}
