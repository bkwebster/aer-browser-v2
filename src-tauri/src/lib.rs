use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use url::Url;

#[tauri::command]
async fn navigate_to_url(url: String, app: tauri::AppHandle) -> Result<(), String> {
    // Parse and validate URL
    let parsed_url: Url = url.parse().map_err(|e: url::ParseError| e.to_string())?;
    
    // Get or create the browser webview window
    let webview_window = app.get_webview_window("browser-webview");
    
    match webview_window {
        Some(window) => {
            // Navigate existing webview
            log::info!("Navigating existing webview to: {}", parsed_url);
            window.navigate(parsed_url).map_err(|e| e.to_string())?;
        }
        None => {
            log::info!("Creating new webview window for URL: {}", parsed_url);
            
            // Create a new webview window for the browser content
            // We'll position it to appear as if it's embedded in the main window
            let _webview_window = WebviewWindowBuilder::new(
                &app, 
                "browser-webview", 
                WebviewUrl::External(parsed_url)
            )
            .title("Browser Content")
            .inner_size(800.0, 600.0)
            .position(100.0, 100.0)
            .resizable(true)
            .build()
            .map_err(|e| format!("Failed to create webview window: {}", e))?;
            
            log::info!("Successfully created webview window");
        }
    }
    
    Ok(())
}

#[tauri::command]
async fn go_back(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(webview) = app.get_webview_window("browser-webview") {
        webview.eval("window.history.back()").map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn go_forward(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(webview) = app.get_webview_window("browser-webview") {
        webview.eval("window.history.forward()").map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn refresh_page(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(webview) = app.get_webview_window("browser-webview") {
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
