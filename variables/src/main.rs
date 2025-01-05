const MAX_POINTS: u32 = 10;

fn main() {
    println!("Hello, world!");

    let mut x = 6;
    println!("the value of x is {}", x);
    x = 4;
    println!("the value of x is {}", x);

    println!("the value of x is {}", MAX_POINTS);

    let x = x * 2;
    println!("the value of x is {}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("the value of x is {}", spaces);

    let guess: u32 = "53".parse().expect("Not a number");
    println!("{}", guess);

    let tup: (i32, f64, u8) = (400, 5.1, 1);
    let (x, y, z) = tup;

    println!("{}, {}, {}", x, y, z);
}
