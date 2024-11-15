mod game;
mod login;
mod socket;
mod teams;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    //#[cfg(debug_assertions)]
    let devtools = tauri_plugin_devtools::init();

    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
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
        .invoke_handler(tauri::generate_handler![
            login::login,
            teams::join,
            teams::leave,
            teams::ready,
            teams::new,
            teams::get,
            game::start,
            socket::connect
        ]);

    //#[cfg(debug_assertions)]
    builder = builder.plugin(devtools);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
