// 1. Write a program to Find The Factorial using functions. 
fn add(mut num:i32)->i32{
    let mut sum=1;
    while num!=0{
        sum*=num;
        num-=1;
    }
    return sum;
}
fn main() {
    println!("{}",add(5));
}

//Ouput:
// 120
