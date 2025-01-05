fn f1() {
    let mut a = 10;
    a += 1;
    println!("f1 The value of a is {}", a);
}

fn f2() {
    let a = "hello";
    println!("f2 The value of a is {}", a.len());
}

fn f3() {
    println!("f3 **********");
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("f3 The value of c is {}", c);
    let d = a - b;
    println!("f3 The value of d is {}", d);
    let e = a * b;
    println!("f3 The value of e is {}", e);
    let f = a / b;
    println!("f3 The value of f is {}", f);
    let g = a % b;
    println!("f3 The value of g is {}", g);
    println!("f3 **********");
}

fn f4() {
    println!("f4 **********");
    // 数组
    let a = [1, 2, 3, 4, 5];
    println!("f4 The value of a is {:?}", a);
    // 元组
    let b = (1, 2, 3, 4, 5);
    println!("f4 The value of b is {:?}", b);
    // 结构体
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let c = Point { x: 1, y: 2 };
    println!("f4 The value of c is {:?}", c);
    // 枚举
    #[derive(Debug)]
    enum Color {
        Red,
        Green,
        Blue,
    }
    let d = Color::Red;
    println!("f4 The value of d is {:?}", d);
    println!("f4 **********");
}

fn f5() {
    println!("f5 **********");
    // 条件语句
    let a = 10;
    if a > 5 {
        println!("f5 The value of a is greater than 5");
    } else if a < 5 {
        println!("f5 The value of a is less than 5");
    } else {
        println!("f5 The value of a is equal to 5");
    }
    // 循环语句
    let mut i = 0;
    while i < 5 {
        println!("f5 The value of i is {}", i);
        i += 1;
    }
    for j in 0..5 {
        println!("f5 The value of j is {}", j);
    }
    println!("f5 **********");
}

fn f6() {
    println!("f6 **********");
    // 函数
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    let c = add(10, 20);
    println!("f6 The value of c is {}", c);
    // 闭包
    let add_closure = |a: i32, b: i32| -> i32 {
        a + b
    };
    let d = add_closure(10, 20);
    println!("f6 The value of d is {}", d);
    println!("f6 **********");
}

fn f7() {
    println!("f7 **********");
    // 指针
    let mut a = 10;
    let b = &mut a;
    *b = 20;
    println!("f7 The value of a is {}", a);
    // 引用
    let mut a = 10;
    let b = &mut a;
    *b = 20;
    println!("f7 The value of a is {}", a);
    // 生命周期
    let x = 10;
    let y = 20;
    let z = x + y;
    println!("f7 The value of z is {}", z);
    println!("f7 **********");
}

fn main() {
    println!("Hello, world!");

    f1();
    f2();
    f3();
    f4();
    f5();
    f6();
}
