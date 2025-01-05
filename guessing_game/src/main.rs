use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("猜数游戏！");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("神秘数据是：{}", secret_number);

    loop {
        println!("猜数一个数！");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");

        // 类型转换
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
