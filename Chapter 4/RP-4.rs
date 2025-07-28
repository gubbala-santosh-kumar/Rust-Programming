// 4. Write a function calculate_area_perimeter() that takes x and y( length and width of a
// rectangle) as a parameter to the function and returns a tuple (area, perimeter).
fn calculate_area_perimeter(x:i32, y:i32)-> (i32, i32){
    return (x*y,2*(x+y));
}
fn main() {
    println!("{:?}",calculate_area_perimeter(12,2));
}

//Output
//(24, 28)
