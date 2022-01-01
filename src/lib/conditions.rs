pub fn run() {
    let age = 18;
    let is_boss = true;
    let msg = "cool man";

    if age > 20 {
        println!("{}", msg);
    } else if is_boss && age >= 18 {
        println!("hi boss");
    } else {
        println!("go to hell");
    }

    let mmsg = if is_boss {"GOOOOOOD!!!!"} else {"BAAAAAAAD"};
    println!("{}", mmsg)
}