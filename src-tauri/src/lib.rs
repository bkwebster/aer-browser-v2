use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use url::Url;

#[tauri::command]
async fn navigate_to_url(url: String, app: tauri::AppHandle) -> Result<(), String> {
    let webview_window = app.get_webview_window("browser-view");
    
    match webview_window {
        Some(window) => {
            let parsed_url: Url = url.parse().map_err(|e: url::ParseError| e.to_string())?;
            window.navigate(parsed_url).map_err(|e| e.to_string())?;
            Ok(())
        }
        None => {
            // Create new webview window if it doesn't exist
            let parsed_url: Url = url.parse().map_err(|e: url::ParseError| e.to_string())?;
            let _webview = WebviewWindowBuilder::new(&app, "browser-view", WebviewUrl::External(parsed_url))
                .title("Browser")
                .inner_size(1200.0, 740.0)
                .position(0.0, 60.0)
                .build()
                .map_err(|e| e.to_string())?;
            Ok(())
        }
    }
}

#[tauri::command]
async fn go_back(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(webview) = app.get_webview_window("browser-view") {
        webview.eval("window.history.back()").map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn go_forward(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(webview) = app.get_webview_window("browser-view") {
        webview.eval("window.history.forward()").map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn refresh_page(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(webview) = app.get_webview_window("browser-view") {
        webview.eval("window.location.reload()").map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![navigate_to_url, go_back, go_forward, refresh_page])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
