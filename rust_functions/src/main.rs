fn main() {
    let mut n=10;
    increment(n);
    if is_even(n) {
        println!("The number is even");
    } else {
        println!("The number is odd");
    }

    println!("Hello, world!");
}

fn increment(num: u32) {
    for i in 1..num {
        println!("{}", i);
    }
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}