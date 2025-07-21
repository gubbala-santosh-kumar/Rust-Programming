//Implicit
fn main() {
    let a = 10;
    let b = 10.5;
    let c = "Hello";
    println!("a: {}\nb: {}\nc: {}",a,b,c);
}

//Explicit
fn main() {
    let a: i32 = 10;
    let b: f32 = 10.5;
    let c: &str = "Hello";
    println!("a: {}\nb: {}\nc: {}",a,b,c);
}
