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
            log::info!("Creating new embedded webview for URL: {}", parsed_url);
            // Get main window position and size to calculate embedded position
            let main_window = app.get_webview_window("main").ok_or("Main window not found")?;
            let main_pos = main_window.outer_position().unwrap_or_default();
            let main_size = main_window.outer_size().unwrap_or_default();
            
            // Position webview inside main window content area (below chrome)
            let webview_x = main_pos.x as f64;
            let webview_y = main_pos.y as f64 + 60.0;  // Below browser chrome
            let webview_width = main_size.width as f64;
            let webview_height = main_size.height as f64 - 60.0;  // Account for chrome
            
            // Create positioned webview window
            let _webview = WebviewWindowBuilder::new(&app, "browser-webview", WebviewUrl::External(parsed_url))
                .title("")
                .inner_size(webview_width, webview_height)
                .position(webview_x, webview_y)
                .decorations(false)         // No title bar
                .always_on_top(false)
                .resizable(false)
                .skip_taskbar(true)         // Don't show in dock/taskbar
                .build()
                .map_err(|e| e.to_string())?;
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
