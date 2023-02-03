fn main() {
    let n = 10;
    increment(n);

    println!("Hello, world!");
}

fn increment(num: u32) {
    for i in 1..num {
        println!("{}", i);
        if is_even(i){
            println!("{} is even",i);
        }
        else {
            println!("{} is odd",i);
        }

    
    }
}

fn is_even(num: u32) ->bool{

    return num%2==0;
    }


