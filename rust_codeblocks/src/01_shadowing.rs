fn main() {
    let mut x=10;

    {
        let x=15;
        println!("The value of x inside a code block is {}",x);
    }
    let x="x is a string";
    println!(" x is {} ",x);

    let x=50;
    println!(" value of x outside a codeblock is {} ",x);

}