use std::io::{Read, Write};
use std::net::TcpStream;
// 引入字节序处理库
use byteorder::{ByteOrder, LittleEndian};

// Rust 客户端解析结构体
use std::mem::size_of;

const CMD_OK: i32 = 5;

#[repr(C, packed)] // 与 C 结构体对齐一致
#[derive(Debug, Clone, Copy)]
struct Message {
    id: u32,
    cmd: u32,
    result: i32,
}

fn send_message(stream: &mut TcpStream, message: Message) -> std::io::Result<()> {
    let mut buffer = [0u8; size_of::<Message>()];
    LittleEndian::write_u32(&mut buffer[0..4], message.id);
    LittleEndian::write_u32(&mut buffer[4..8], message.cmd);
    LittleEndian::write_i32(&mut buffer[8..12], message.result);
    stream.write_all(&buffer)?;
    Ok(())
}

fn read_message(stream: &mut TcpStream) -> std::io::Result<Message> {
    let mut buffer = [0u8; size_of::<Message>()];
    stream.read_exact(&mut buffer)?;
    let mut message = Message {
        cmd: 0,
        id: 0,
        result: 0,
    };
    message.id = LittleEndian::read_u32(&buffer[0..4]);
    message.cmd = LittleEndian::read_u32(&buffer[4..8]);
    message.result = LittleEndian::read_i32(&buffer[8..12]);
    Ok(message)
}

#[tauri::command]
fn tcp(id: u32, cmd: u32) -> String {
    println!("tcp called with id: {}, cmd: {}", id, cmd);
    // 连接服务器
    let mut stream = TcpStream::connect("192.168.31.221:8080").unwrap();
    println!("已连接到服务器");

    // 发送消息
    let message = Message { id, cmd, result: 0 };
    send_message(&mut stream, message).unwrap();
    println!("发送消息: {:?}", message);
    // 接收消息
    let response = read_message(&mut stream).unwrap();
    println!("收到消息: {:?}", response);
    if response.result == CMD_OK {
        println!("命令执行成功");
        format!("命令执行成功, {:?}", response)
    } else {
        println!("命令执行失败");
        format!("命令执行失败, {:?}", response)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![tcp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
