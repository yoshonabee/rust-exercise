use std::mem;

pub fn run() {
    let mut t: (&str, &str, i32) = ("Cool", "man", 123);

    println!("A {} {} is {}", t.0, t.1, t.2);

    t.1 = "234";
    println!("A {} {} is {}", t.0, t.1, t.2);

    let mut arr1: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", arr1);

    arr1[2] = 234;
    println!("{:?}", arr1);

    let idx = 3;
    arr1[idx] = 1;
    println!("{}", arr1[idx]);

    println!("arr occupies {} bytes", mem::size_of_val(&arr1));

    let slice: &[i32] = &arr1[1..3];

    println!("Slice: {:?}", slice);
}