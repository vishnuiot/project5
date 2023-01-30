fn main() {
    for i in 1..11 {
        println!("The number is {}", i);
    }

    for i in 21..31 {
        println!("The number is {}", i);
    }

    let numbers = 70..91;
    for i in numbers {
        println!("The number is {}", i);
    }

    let animals=vec!["cat","rhino","Elephant"];

    for (index,a) in animals.iter().enumerate(){
        println!("The index is {} and the animal is {}",index,a)
    }


}