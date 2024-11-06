use codewars::primes;

fn main() {
    let Some(arg) = std::env::args().nth(1) else {
        eprintln!("Expected natural number N as an argument");
        std::process::exit(1);
    };

    let Ok(n) = arg.parse::<usize>() else {
        eprintln!("Expected natural number, got {arg}");
        std::process::exit(2);
    };

    if let Some(p) = primes::stream().nth(n.saturating_sub(1)) {
        println!("The {n}-th prime is {p}");
    }
}
