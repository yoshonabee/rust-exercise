pub fn run() {
    let name = "Joe";

    let name2 = "abc";
    let mut age = 24; // default i32
    println!("My name is {}, and I'm {}", name, age);

    age = 123;
    println!("My name is {:?}, and I'm {}", name2.chars().nth(2), age);

    let age2: i64 = 256;
    println!("My name is {:?}, and I'm {}", name2.chars().nth(2), age2);

    let age3: f32 = 333.222; // default f64
    println!("My name is {:?}, and I'm {}", name2.chars().nth(2), age3);

    println!("My name is {:?}, and I'm {}", name2.chars().nth(2), std::i32::MAX);

    let is_child: bool = false;

    let ccc: char = '\u{1F600}';

    println!("I am child: {}", ccc);
    println!("I am child: {}", is_child);
}