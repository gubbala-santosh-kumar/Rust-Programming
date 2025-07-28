// 2. Write a function test_divisibility_by_3_4 which will check whether a given integer
// number is divisible by 3 or 4.
// a. If the number is divisible by both return 0
// b. If the number is divisible by 3 only return 1
// c. If the number is divisible by 4 only return 2
// d. If the number is not divisible by both, return -1

fn isDivisible(mut num:i32)->i32{
    if num%3==0 && num%4==0{
        return 0;
    }
    else if num%3==0 && num%4!=0{
        return 1;
    }
    else if num%3!=0 && num%4==0{
        return 2;
    }
    else{
        return -1;
    }
}
fn main() {
    println!("{}",isDivisible(12));
}
