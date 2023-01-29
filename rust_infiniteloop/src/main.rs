// example for infinite loop in rust


fn main(){
    let mut x=5;
    // println!("The initial value is  {}",x);
    // println!("The initial value is  {}",x);

    loop{
        x=x+1;

        if x>10{
           
            break;
        }

        if x==7{
            continue;
        }
    
        println!("The incremented value of n is {} ",x);
    }

}