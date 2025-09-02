// CEF Browser Management Module
// This module handles embedded CEF browser views for true Chromium browsing experience

use cef::{
    app::CefApp,
    browser::{CefBrowser, CefBrowserSettings},
    client::{CefClient, CefBrowserView, CefBrowserViewDelegate},
    task::CefTask,
    window::{CefWindow, CefWindowDelegate},
    types::{CefString, CefRect},
    Error,
};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use url::Url;

// Global registry for CEF browser instances
static mut CEF_BROWSERS: Option<Arc<Mutex<HashMap<String, CefBrowserView>>>> = None;

// Initialize CEF browser management
pub fn initialize_cef() -> Result<(), Error> {
    unsafe {
        CEF_BROWSERS = Some(Arc::new(Mutex::new(HashMap::new())));
    }
    
    // Initialize CEF with default settings
    let app = CefApp::new();
    app.initialize_cef().map_err(|e| {
        log::error!("Failed to initialize CEF: {:?}", e);
        e
    })?;
    
    log::info!("CEF browser engine initialized successfully");
    Ok(())
}

// Create embedded browser view within the Tauri window
pub fn create_embedded_browser(
    browser_id: &str,
    url: &Url,
    parent_window_handle: i64, // Native window handle from Tauri
) -> Result<(), Box<dyn std::error::Error>> {
    
    log::info!("Creating embedded CEF browser for URL: {}", url);
    
    // Create CEF client and browser view delegate
    let client = CefClient::new();
    let view_delegate = CefBrowserViewDelegate::new();
    
    // Browser settings for optimal performance
    let mut browser_settings = CefBrowserSettings::default();
    browser_settings.set_web_security(false); // Allow cross-origin requests for browser functionality
    browser_settings.set_universal_access_from_file_urls(true);
    
    // Create the browser view (this is embedded, not a separate window)
    let browser_view = CefBrowserView::create(
        &client,
        &CefString::from(url.as_str()),
        &browser_settings,
        None, // Use default request context
        Some(view_delegate),
    ).ok_or("Failed to create CEF browser view")?;
    
    // Store the browser view in our registry
    unsafe {
        if let Some(browsers) = &CEF_BROWSERS {
            let mut browsers = browsers.lock().unwrap();
            browsers.insert(browser_id.to_string(), browser_view.clone());
        }
    }
    
    // Get the parent window and add the browser view as a child
    // This is where the magic happens - embedding the browser in the existing Tauri window
    if let Some(parent_window) = get_window_from_handle(parent_window_handle) {
        parent_window.add_child_view(&browser_view);
        
        // Set the browser view to fill the content area (below the address bar)
        let content_bounds = CefRect {
            x: 0,
            y: 60,  // Start below the address bar
            width: 1200, // Full width
            height: 740, // Fill remaining height
        };
        browser_view.set_bounds(content_bounds);
        
        log::info!("Successfully embedded CEF browser view in Tauri window");
    } else {
        return Err("Failed to get parent window handle".into());
    }
    
    Ok(())
}

// Navigate existing browser or create new one
pub fn navigate_browser(browser_id: &str, url: &Url) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        if let Some(browsers) = &CEF_BROWSERS {
            let browsers = browsers.lock().unwrap();
            
            if let Some(browser_view) = browsers.get(browser_id) {
                // Navigate existing browser
                if let Some(browser) = browser_view.get_browser() {
                    let main_frame = browser.get_main_frame()
                        .ok_or("Failed to get main frame")?;
                    main_frame.load_url(&CefString::from(url.as_str()));
                    log::info!("Navigated existing CEF browser to: {}", url);
                    return Ok(());
                }
            }
        }
    }
    
    // If no existing browser found, this is handled by the calling function
    Err("Browser view not found".into())
}

// Browser navigation controls
pub fn browser_go_back(browser_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        if let Some(browsers) = &CEF_BROWSERS {
            let browsers = browsers.lock().unwrap();
            if let Some(browser_view) = browsers.get(browser_id) {
                if let Some(browser) = browser_view.get_browser() {
                    if browser.can_go_back() {
                        browser.go_back();
                        log::info!("CEF browser went back");
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn browser_go_forward(browser_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        if let Some(browsers) = &CEF_BROWSERS {
            let browsers = browsers.lock().unwrap();
            if let Some(browser_view) = browsers.get(browser_id) {
                if let Some(browser) = browser_view.get_browser() {
                    if browser.can_go_forward() {
                        browser.go_forward();
                        log::info!("CEF browser went forward");
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn browser_refresh(browser_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        if let Some(browsers) = &CEF_BROWSERS {
            let browsers = browsers.lock().unwrap();
            if let Some(browser_view) = browsers.get(browser_id) {
                if let Some(browser) = browser_view.get_browser() {
                    browser.reload();
                    log::info!("CEF browser refreshed");
                }
            }
        }
    }
    Ok(())
}

// Helper function to get window from native handle (platform-specific)
#[cfg(target_os = "macos")]
fn get_window_from_handle(handle: i64) -> Option<CefWindow> {
    // macOS implementation - convert NSWindow handle to CefWindow
    // This requires platform-specific CEF APIs
    None // Placeholder for now
}

#[cfg(target_os = "windows")]
fn get_window_from_handle(handle: i64) -> Option<CefWindow> {
    // Windows implementation - convert HWND to CefWindow
    None // Placeholder for now
}

#[cfg(target_os = "linux")]
fn get_window_from_handle(handle: i64) -> Option<CefWindow> {
    // Linux implementation - convert X11 window to CefWindow
    None // Placeholder for now
}

// Cleanup CEF on application exit
pub fn shutdown_cef() {
    unsafe {
        if let Some(browsers) = &CEF_BROWSERS {
            let mut browsers = browsers.lock().unwrap();
            browsers.clear();
        }
    }
    
    // Shutdown CEF
    cef::shutdown_cef();
    log::info!("CEF browser engine shut down");
}
