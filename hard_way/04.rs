fn main() {
    let array : [u8;3] = [23,24,25];
    let initial_array : [&str;3] = ["hello";3];

    println!("{:?}", array);
    println!("{:?}, size : {1}", initial_array, initial_array.len());
}