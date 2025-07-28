// 5. Write a function arr_square() that returns the Array of Squares
fn arr_square(arr: &[i32]) -> Vec<i32> {
    arr.iter().map(|&x| x * x).collect()
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let squares = arr_square(&numbers);
    println!("{:?}", squares);
}

// Output: [1, 4, 9, 16, 25]
