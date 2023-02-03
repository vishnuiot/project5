fn main() {
    let tup1 = (55, 7.9, "test",(1,4,7));
    let (a, b, c,d) = tup1;
    println!("{}",a);
    println!("{}",b);
    println!("{}",c);
    println!("{}",d.0);
    println!("{}",d.1);
    println!("{}",d.2);
    println!("TEst program");
}

// ,(100,200,300)