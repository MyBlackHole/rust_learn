use std::env;
use std::path::PathBuf;

fn main() {
    // 告诉 Cargo 如果头文件改变就重新运行
    println!("cargo:rerun-if-changed=math_utils.h");

    // 编译 C 代码
    cc::Build::new()
        .file("src/math_utils.c")
        .compile("math_utils");

    // 生成绑定
    let bindings = bindgen::Builder::default()
        .header("src/math_utils.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // 输出绑定到文件
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
