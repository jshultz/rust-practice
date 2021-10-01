fn main() {
    let oper_fn: String = std::env::args().nth(1).expect("operator not found.");
    let numbers: Vec<u32> = std::env::args()
        .skip(2)
        .map(|arg| match arg.trim().parse() {
            Ok(n) => n,
            Err(_) => 0
        })
        .collect::<Vec<u32>>();
    
    
    match oper_fn.trim() {    
        "help" => {
            println!("Options: lcm, gcf, prime, factorial");
            println!("lcm <n1> <n2>");
            println!("gcf <n1> <n2>");
            println!("prime <n>");
            println!("factorial <n>");
        }
        "lcm" => {
            let r = lcm(numbers[0], numbers[1]);
            println!("The least common multiple of {} and {} is {}.",
                     numbers[0],
                     numbers[1],
                     r);
        }
        "gcf" => {
            let r = gcf(numbers[0], numbers[1]);
            println!("The greatest common factor of {} and {} is {}.",
                     numbers[0],
                     numbers[1],
                     r);
        }
        "prime" => {
            let r = prime(numbers[0]);
            println!("The prime factors of {} are {:?}.", numbers[0], r);
        }
        "factorial" => {
            let r = factorial(numbers[0]);
            println!("The factorial of {} is {}", numbers[0], r);
        }
        _ => {
            println!("operator not defined!!");
            println!("use \"help\" to get a list of all operations.");
        }
    }
}

fn gcf(n1: u32, n2: u32) -> u32 {
    for x in (std::ops::Range { start: 1, end: n1.max(n2) }).rev() {
        if n1 % x == 0 && n2 % x == 0 {
            return x as u32
        }
    }
    1
}

fn lcm(n1: u32, n2: u32) -> u32 {
    for x in (std::ops::Range { start: 1, end: n1*n2 }) {
        if x % n1 == 0 && x % n2 == 0 {
            return x as u32
        }
    }
    n1*n2
}

fn prime(n: u32) -> Vec<u32> {
    let mut primes = vec![];
    let mut i = n;
    for x in 2..n+1 {
        while i%x == 0 {
            primes.push(x);
            i /= x;
        }
    }
    if i > 1 {
        primes.push(i);
    }
    primes
}

fn factorial(n: u32) -> u32 {
    let mut x = 1;
    for i in 1..n+1 {
        x *= i;
    }
    x
}
