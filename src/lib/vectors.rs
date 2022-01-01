pub fn run() {
    let mut v1: Vec<i32> = vec![1, 2, 3];

    println!("{:?}", v1);

    for n in 0..3 {
        v1.push(n);
    }

    println!("{:?}", v1);

    for n in v1.iter() {
        println!("{}", n);
    }

    for x in v1.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", v1);
}