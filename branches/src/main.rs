fn main() {
    println!("Hello, world!");

    let condition = true;

    let number = if condition { 5 } else { 65 };

    println!("{}", number);

    let mut counter = 0;

    let result = loop {
        counter = counter - 1;
    };
}
