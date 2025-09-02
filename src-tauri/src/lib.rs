use tauri::Manager;
use url::Url;
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::LazyLock;

// mod cef_browser; // TODO: Fix CEF API integration
// CEF integration temporarily disabled while we research the correct API

// Global registry for CEF browser views (keeping for compatibility)
static CEF_BROWSERS: LazyLock<Mutex<HashMap<String, i32>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

#[tauri::command]
async fn navigate_to_url(url: String, app: tauri::AppHandle) -> Result<(), String> {
    // Parse and validate URL
    let parsed_url: Url = url.parse().map_err(|e: url::ParseError| e.to_string())?;
    
    log::info!("Navigating to URL: {}", parsed_url);
    
    // For now, create a separate window until we implement CEF properly
    // This is a temporary fallback while we set up CEF integration
    let _main_window = app.get_webview_window("main")
        .ok_or("Main window not found")?;
    
    // Get existing browser webview or create new one
    if let Some(browser_window) = app.get_webview_window("browser-webview") {
        log::info!("Navigating existing webview to: {}", parsed_url);
        browser_window.navigate(parsed_url).map_err(|e| e.to_string())?;
    } else {
        log::info!("Creating new browser webview for: {}", parsed_url);
        
        // Temporary: Use WebviewWindowBuilder as fallback
        // TODO: Replace with CEF embedded browser view
        use tauri::{WebviewUrl, WebviewWindowBuilder};
        let _browser_window = WebviewWindowBuilder::new(
            &app,
            "browser-webview", 
            WebviewUrl::External(parsed_url)
        )
        .title("Browser Content")
        .inner_size(800.0, 600.0)
        .position(100.0, 100.0)
        .resizable(true)
        .build()
        .map_err(|e| format!("Failed to create browser window: {}", e))?;
        
        log::info!("Created fallback browser window (will be replaced with CEF)");
    }
    
    Ok(())
}

#[tauri::command]
async fn go_back(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(webview) = app.get_webview("browser-webview") {
        webview.eval("window.history.back()").map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn go_forward(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(webview) = app.get_webview("browser-webview") {
        webview.eval("window.history.forward()").map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn refresh_page(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(webview) = app.get_webview("browser-webview") {
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
