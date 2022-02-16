fn main() {
    let mut args:Vec<String> = std::env::args().collect();
    println!("{:?}", &mut args[1..]);
    println!("{:?}", &mut args);
}
