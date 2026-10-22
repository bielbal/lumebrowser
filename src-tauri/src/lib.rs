use tauri::{Manager, WebviewBuilder, WebviewUrl, Emitter};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use url::Url;

// ── State ─────────────────────────────────────────────────────────────────────

#[derive(Default)]
pub struct TabRegistry(pub Mutex<HashMap<String, TabMeta>>);

pub struct TabMeta {
    pub url: String,
    pub title: String,
}

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
        .ok_or_else(|| "main window not found".to_string())?;

    let size = window.inner_size().map_err(|e| e.to_string())?;
    let scale = window.scale_factor().unwrap_or(1.0);
    let chrome_px = (92.0 * scale) as u32;
    let content_h = size.height.saturating_sub(chrome_px);

    let label = format!("tab-{}", tab_id);

    window
        .add_child(
            WebviewBuilder::new(&label, WebviewUrl::External(parsed_url))
                .bounds(tauri::Rect {
                    x: 0.0,
                    y: 92.0,
                    width: (size.width as f64) / scale,
                    height: (content_h as f64) / scale,
                })
                .build(),
        )
        .map_err(|e| e.to_string())?;

    let mut registry = state.0.lock().unwrap();
    registry.insert(
        tab_id.clone(),
        TabMeta { url: url.clone(), title: url.clone() },
    );

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
    let label = format!("tab-{}", tab_id);

    if let Some(wv) = app.get_webview(&label) {
        wv.navigate(parsed_url).map_err(|e| e.to_string())?;
    } else {
        // Tab webview doesn't exist yet — create it
        create_tab(app.clone(), tab_id.clone(), url.clone(), state).await?;
    }

    Ok(())
}

#[tauri::command]
async fn tab_back(app: tauri::AppHandle, tab_id: String) -> Result<(), String> {
    let label = format!("tab-{}", tab_id);
    if let Some(wv) = app.get_webview(&label) {
        wv.eval("window.history.back()").map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn tab_forward(app: tauri::AppHandle, tab_id: String) -> Result<(), String> {
    let label = format!("tab-{}", tab_id);
    if let Some(wv) = app.get_webview(&label) {
        wv.eval("window.history.forward()").map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn tab_reload(app: tauri::AppHandle, tab_id: String) -> Result<(), String> {
    let label = format!("tab-{}", tab_id);
    if let Some(wv) = app.get_webview(&label) {
        wv.eval("window.location.reload()").map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn tab_stop(app: tauri::AppHandle, tab_id: String) -> Result<(), String> {
    let label = format!("tab-{}", tab_id);
    if let Some(wv) = app.get_webview(&label) {
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
    let label = format!("tab-{}", tab_id);
    if let Some(wv) = app.get_webview(&label) {
        wv.close().map_err(|e| e.to_string())?;
    }
    let mut registry = state.0.lock().unwrap();
    registry.remove(&tab_id);
    Ok(())
}

#[tauri::command]
async fn activate_tab(app: tauri::AppHandle, tab_id: String) -> Result<(), String> {
    // Show only the active tab's webview; hide others
    let target_label = format!("tab-{}", tab_id);
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "main window not found".to_string())?;

    for (label, wv) in app.webviews() {
        if label.starts_with("tab-") {
            let visible = label == target_label;
            let _ = if visible { wv.show() } else { wv.hide() };
        }
    }

    Ok(())
}

#[tauri::command]
async fn resize_content(
    app: tauri::AppHandle,
    chrome_height: f64,
) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "main window not found".to_string())?;

    let size = window.inner_size().map_err(|e| e.to_string())?;
    let scale = window.scale_factor().unwrap_or(1.0);
    let w = (size.width as f64) / scale;
    let h = (size.height as f64) / scale - chrome_height;

    for (label, wv) in app.webviews() {
        if label.starts_with("tab-") {
            let _ = wv.set_bounds(tauri::Rect {
                x: 0.0,
                y: chrome_height,
                width: w,
                height: h.max(0.0),
            });
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
                let chrome_h = 92.0_f64;
                let w = (size.width as f64) / scale;
                let h = (size.height as f64) / scale - chrome_h;

                for (label, wv) in app.webviews() {
                    if label.starts_with("tab-") {
                        let _ = wv.set_bounds(tauri::Rect {
                            x: 0.0,
                            y: chrome_h,
                            width: w,
                            height: h.max(0.0),
                        });
                    }
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running Lume Browser");
}
