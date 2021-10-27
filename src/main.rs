fn main() {
    let mut line = String::new();
    println!( "Please enter your name: ");
    std::io::stdin().read_line(&mut line).unwrap();
    println!("{}", print_hello(line));
}


fn print_hello(name: String) -> String {
    let message = "Hello ".to_owned() + &name;
    return message;
}

#[test]
fn test_build_message(){
    //panic!();
    let name = String::from("Jack");
    assert_eq!(print_hello(name), "Hello Jack")
}