use tauri::{AppHandle, Emitter};

#[tauri::command]
fn login(
    app: AppHandle,
    address: String,
    username: String,
    password: String,
) -> Result<String, String> {
    println!("{}, {}, {}", address, username, password);

    let res = reqwest::get("http://my.api.host/data.json").await;

    app.emit("closeDrawer", ()).unwrap();

    Ok(format!("Logged in successfully as {}.", username))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
