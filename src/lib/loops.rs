pub fn run() {
    let mut count = 1;

    loop {
        count += 1;

        if count == 30 {
            break;
        }
    }

    println!("I'm out of there!");

    while count <= 40 {
        count += 1;
    }

    println!("I'm out of there!");

    for i in 0..5 {
        println!("{}", i)
    }
}