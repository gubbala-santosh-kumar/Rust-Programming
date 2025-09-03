// Program to print the fibonacci series upto range
fn main() {
    println!("Fibonacci Series upto range");
    let a = 6;
    for i in 0..a{
        print!("{} ",fib(i));
    }
}

fn fib(x: i32) -> i32{
    if x<=1{
        return x;
    }
    else{
        return fib(x-1)+fib(x-2);
    }
}
