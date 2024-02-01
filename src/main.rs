use fizzbuzz::fizzbuzzer::fizzbuzzer;

fn main() {
    for c in 1..101 {
        let answer = fizzbuzzer(c);
        println!("{} -> {}", c, answer);
    }
}
