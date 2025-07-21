fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let slice_2_and_3 = &arr[1..3];
    println!("a. Slice of 2nd and 3rd element: {:?}", slice_2_and_3);

    let slice_omit_start = &arr[..5];
    println!("b. Slice with start index omitted: {:?}", slice_omit_start);

    let slice_omit_end = &arr[5..];
    println!("c. Slice with end index omitted: {:?}", slice_omit_end);

    let slice_omit_both = &arr[..];
    println!("d. Slice with both start and end index omitted: {:?}", slice_omit_both);
}
