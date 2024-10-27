use tauri::{AppHandle, Emitter};
use tauri_plugin_http::reqwest;
use tokio::task;
use crate::socket::socket;

#[tauri::command]
pub async fn login(
    app: AppHandle,
    address: String,
    username: String,
    password: String,
) -> Result<(String, bool), String> {
    // Check server version
    let version_check = reqwest::get(&address).await.map_err(|e| e.to_string())?;
    let version_body = version_check.text().await.map_err(|e| e.to_string())?;

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
    let login_response = client
        .post(&login_url)
        .json(&login_details)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if login_response.status() != reqwest::StatusCode::ACCEPTED {
        return Err("Login failed".into());
    }

    let login_body: serde_json::Value = login_response.json().await.map_err(|e| e.to_string())?;

    // Check token and admin status
    if let (Some(token), Some(admin)) = (login_body.get("token"), login_body.get("admin")) {
        println!("Token received: {}", token);

        app.emit("closeDrawer", ()).map_err(|e| e.to_string())?;

        let t = token.to_string();

        task::spawn(async move {
            socket(app.clone(), t.clone()).await;
        });

        Ok((token.to_string(), admin.as_bool().unwrap_or(false)))
    } else {
        Err("Invalid login details".into())
    }
}
