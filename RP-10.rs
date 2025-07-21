fn main() {
    let str_literal = "Hello, Rust!";
    let str_object = String::from(str_literal);
    
    let object = str_literal.to_string();
    
    println!("{}", str_object);
    println!("{}", object);
}
