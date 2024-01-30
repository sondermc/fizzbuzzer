use fizzbuzz::fizzbuzzer::fizzbuzzer;
fn main() {
    for c in 1..100 {
        let answer = fizzbuzzer(c);
        println!("{} -> {}", c, answer);
    }
}
