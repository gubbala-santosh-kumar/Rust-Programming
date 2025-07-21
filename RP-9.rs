fn main() {
    const MY_INT: i32 = 10;
    const MY_FLOAT: f64 = 3.14159;
    const MY_BOOL: bool = true;
    const MY_CHAR: char = 'R';
    const MY_STRING: &str = "Hello, Rust!";
    const MY_ARRAY: [i32; 3] = [1, 2, 3];
    const MY_TUPLE: (i32, f64, char) = (42, 3.14, 'A');
    
    println!("Integer Constant: {}", MY_INT);
    println!("Floating-Point Constant: {}", MY_FLOAT);
    println!("Boolean Constant: {}", MY_BOOL);
    println!("Character Constant: {}", MY_CHAR);
    println!("String Constant: {}", MY_STRING);
    println!("Array Constant: {:?}", MY_ARRAY);
    println!("Tuple Constant: {:?}", MY_TUPLE);
}
