use tauri::{AppHandle, Emitter};
use tauri_plugin_http::reqwest;

#[tauri::command]
async fn login(
    app: AppHandle,
    address: String,
    username: String,
    password: String,
) -> Result<String, String> {
    // Check server version
    let version_check = reqwest::get(&address)
        .await
        .map_err(|e| e.to_string())?;

    let version_body = version_check.text()
        .await
        .map_err(|e| e.to_string())?;

    println!("{}", version_body);

    if !version_body.eq("City Runners, version 0.1.0 \n") {
        return Err("Invalid server response".into());
    }

    // Prepare login details
    let login_url = format!("{}/login", address);
    let login_details = serde_json::json!({
        "username": username,
        "password": password,
    });

    // Send login request
    let client = reqwest::Client::new();
    let login_response = client.post(&login_url)
        .json(&login_details)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if login_response.status() != reqwest::StatusCode::ACCEPTED {
        return Err("Login failed".into());
    }

    let login_body: serde_json::Value = login_response.json()
        .await
        .map_err(|e| e.to_string())?;

    // Check token
    if let Some(token) = login_body.get("token") {
        println!("Token received: {}", token);

        app.emit("closeDrawer", ())
            .map_err(|e| e.to_string())?;

        Ok(token.to_string())
    } else {
        Err("invalid login details".into())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
            .setup(|app| {
        use tauri_plugin_notification::NotificationExt;
        app.notification()
            .builder()
            .title("Tauri")
            .body("Tauri is awesome")
            .show()
            .unwrap();

        Ok(())
    })
        .invoke_handler(tauri::generate_handler![login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
