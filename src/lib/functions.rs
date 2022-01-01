pub fn run() {
    let a = "1111";
    print123(5, a);

    let (n1, n2, sum) = add(1, 23333);

    println!("{:?}", (n1, n2, sum));
    println!("{}", a);
}

fn print123(count: i32, msg: &str) {
    for _ in 0..count {
        print!("{}", msg);
    }

    println!();
}

fn add(n1: i32, n2: i32) -> (i32, i32, i32) {
    return (n1, n2, n1 + n2);
}
