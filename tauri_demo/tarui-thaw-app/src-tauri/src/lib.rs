use tauri_plugin_http::reqwest;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
async fn http(name: String) -> String {
    let res = reqwest::get("http://10.5.8.81:8000/").await;

    let status = match res {
        Ok(data) => data.status().to_string(),
        Err(e) => e.to_string(),
    };

    println!("来自 Rust 的信息, name: {}!", name);
    format!("type_name is: {}, status: {}!", name, status)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![http])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
