fn main() {
    let mut n=1;
    let mut y=5;
    
    // while n<50{
    //     println!("The value of n is {} ",n);
    //     n=n+1;
    // }

    while n <= 50{
        // if n is a multiple of 5
        if n % 5==0{
            println!("The multiple of {} is {}",y,n);

        }
        n=n+1;
    }

    println!("Hello, world!");
}
