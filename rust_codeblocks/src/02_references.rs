fn main() {
    let mut x = 10;

    let xr = &x;
    println!("The value of x is {} ", xr);
    let mut y = 100;
    {
        let dom = &mut y;
        *dom += 1;
        //  println!("The value of y is {} ",y);
    }

    println!("The value of y is {} ", y);
}