// 包含生成的绑定
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    unsafe {
        // 调用 C 函数
        let result = add(5, 3);
        println!("5 + 3 = {}", result);

        let product = multiply(4.5, 2.0);
        println!("4.5 * 2.0 = {}", product);

        print_number(42);

        // 使用结构体
        let p1 = create_point(0, 0);
        let p2 = create_point(3, 4);
        let distance = point_distance(p1, p2);
        println!("Distance between points: {}", distance);
    }
}
