use tauri::{Manager, LogicalPosition, LogicalSize};
use tauri::webview::{WebviewBuilder, WebviewUrl};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;
use url::Url;

// ── State ─────────────────────────────────────────────────────────────────────

/// Tracks all open tab IDs so we can reposition their webviews on resize.
#[derive(Default)]
pub struct TabRegistry(pub Mutex<HashMap<String, bool>>); // tab_id -> visible

// ── Event payloads ────────────────────────────────────────────────────────────

#[derive(Clone, Serialize)]
pub struct NavEvent {
    pub tab_id: String,
    pub url: String,
    pub title: Option<String>,
    pub favicon: Option<String>,
    pub can_go_back: bool,
    pub can_go_forward: bool,
}

#[derive(Clone, Serialize)]
pub struct LoadEvent {
    pub tab_id: String,
    pub loading: bool,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

const CHROME_HEIGHT: f64 = 92.0;

fn tab_label(tab_id: &str) -> String {
    format!("tab-{}", tab_id)
}

fn content_size(window: &tauri::WebviewWindow) -> (f64, f64) {
    let size = window.inner_size().unwrap_or_default();
    let scale = window.scale_factor().unwrap_or(1.0);
    let w = (size.width as f64) / scale;
    let h = ((size.height as f64) / scale - CHROME_HEIGHT).max(0.0);
    (w, h)
}

// ── Commands ──────────────────────────────────────────────────────────────────

#[tauri::command]
async fn create_tab(
    app: tauri::AppHandle,
    tab_id: String,
    url: String,
    state: tauri::State<'_, TabRegistry>,
) -> Result<(), String> {
    let parsed_url = Url::parse(&url).map_err(|e| e.to_string())?;
    let window = app
        .get_webview_window("main")
        .ok_or("main window not found")?;

    let (w, h) = content_size(&window);
    let label = tab_label(&tab_id);

    window
        .add_child(
            WebviewBuilder::new(&label, WebviewUrl::External(parsed_url)),
            LogicalPosition::new(0.0, CHROME_HEIGHT),
            LogicalSize::new(w, h),
        )
        .map_err(|e| e.to_string())?;

    let mut registry = state.0.lock().unwrap();
    registry.insert(tab_id, true);

    Ok(())
}

#[tauri::command]
async fn navigate_tab(
    app: tauri::AppHandle,
    tab_id: String,
    url: String,
    state: tauri::State<'_, TabRegistry>,
) -> Result<(), String> {
    let parsed_url = Url::parse(&url).map_err(|e| e.to_string())?;
    let label = tab_label(&tab_id);

    if let Some(wv) = app.get_webview(&label) {
        wv.navigate(parsed_url).map_err(|e| e.to_string())?;
    } else {
        // Lazily create webview on first navigation
        create_tab(app, tab_id, url, state).await?;
    }
    Ok(())
}

#[tauri::command]
async fn tab_back(app: tauri::AppHandle, tab_id: String) -> Result<(), String> {
    if let Some(wv) = app.get_webview(&tab_label(&tab_id)) {
        wv.eval("window.history.back()").map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn tab_forward(app: tauri::AppHandle, tab_id: String) -> Result<(), String> {
    if let Some(wv) = app.get_webview(&tab_label(&tab_id)) {
        wv.eval("window.history.forward()").map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn tab_reload(app: tauri::AppHandle, tab_id: String) -> Result<(), String> {
    if let Some(wv) = app.get_webview(&tab_label(&tab_id)) {
        wv.eval("window.location.reload()").map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn tab_stop(app: tauri::AppHandle, tab_id: String) -> Result<(), String> {
    if let Some(wv) = app.get_webview(&tab_label(&tab_id)) {
        wv.eval("window.stop()").map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn close_tab(
    app: tauri::AppHandle,
    tab_id: String,
    state: tauri::State<'_, TabRegistry>,
) -> Result<(), String> {
    if let Some(wv) = app.get_webview(&tab_label(&tab_id)) {
        wv.close().map_err(|e| e.to_string())?;
    }
    state.0.lock().unwrap().remove(&tab_id);
    Ok(())
}

#[tauri::command]
async fn activate_tab(
    app: tauri::AppHandle,
    tab_id: String,
    state: tauri::State<'_, TabRegistry>,
) -> Result<(), String> {
    let target = tab_label(&tab_id);
    let mut registry = state.0.lock().unwrap();

    for (id, visible) in registry.iter_mut() {
        let label = tab_label(id);
        if let Some(wv) = app.get_webview(&label) {
            if label == target {
                let _ = wv.show();
                *visible = true;
            } else {
                let _ = wv.hide();
                *visible = false;
            }
        }
    }
    Ok(())
}

#[tauri::command]
async fn resize_content(
    app: tauri::AppHandle,
    chrome_height: f64,
    state: tauri::State<'_, TabRegistry>,
) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("main window not found")?;
    let size = window.inner_size().map_err(|e| e.to_string())?;
    let scale = window.scale_factor().unwrap_or(1.0);
    let w = (size.width as f64) / scale;
    let h = ((size.height as f64) / scale - chrome_height).max(0.0);

    let registry = state.0.lock().unwrap();
    for id in registry.keys() {
        if let Some(wv) = app.get_webview(&tab_label(id)) {
            let _ = wv.set_size(LogicalSize::new(w, h));
            let _ = wv.set_position(LogicalPosition::new(0.0, chrome_height));
        }
    }
    Ok(())
}

// ── App setup ─────────────────────────────────────────────────────────────────

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(TabRegistry::default())
        .invoke_handler(tauri::generate_handler![
            create_tab,
            navigate_tab,
            tab_back,
            tab_forward,
            tab_reload,
            tab_stop,
            close_tab,
            activate_tab,
            resize_content,
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Resized(size) = event {
                let app = window.app_handle();
                let scale = window.scale_factor().unwrap_or(1.0);
                let w = (size.width as f64) / scale;
                let h = ((size.height as f64) / scale - CHROME_HEIGHT).max(0.0);

                if let Ok(registry) = window.try_state::<TabRegistry>() {
                    let tabs = registry.0.lock().unwrap();
                    for id in tabs.keys() {
                        if let Some(wv) = app.get_webview(&tab_label(id)) {
                            let _ = wv.set_size(LogicalSize::new(w, h));
                            let _ = wv.set_position(LogicalPosition::new(0.0, CHROME_HEIGHT));
                        }
                    }
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running Lume Browser");
}
