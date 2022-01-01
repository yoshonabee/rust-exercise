pub fn run() {
    // let hello = String::from("Hello!");
    let mut hello: String = String::from("123");

    println!("say {}", hello);

    for c in hello.chars() {
        println!("{}", c);
    }

    hello.push('a');

    println!("{}, {:?}", hello, (hello.len(), hello.capacity()));
    hello.push_str(" abc");

    hello.push_str("asdfasdfasdf");

    println!("{}, {:?}", hello, (hello.len(), hello.capacity()));


}
