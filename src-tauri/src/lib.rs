use anyhow::Result;
use tauri::{
    webview::PageLoadPayload, App, Builder, Webview, WebviewUrl, WebviewWindowBuilder, Window,
    WindowEvent, Wry,
};
mod commends;
mod utils;

use tauri_plugin_log::{
    log::{debug, info},
    Target, TargetKind,
};

use commends::{get_app_dir, greet};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub fn app() -> Result<Builder<Wry>> {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(logger().build())
        .invoke_handler(tauri::generate_handler![greet, get_app_dir])
        .setup(setup)
        .on_page_load(on_page_load)
        .on_window_event(on_window_event_handler);
    Ok(builder)
}

fn on_page_load<'a>(webview: &Webview, _payload: &PageLoadPayload<'_>) {
    info!("Page loaded: {}", webview.label());
}

fn on_window_event_handler(window: &Window, event: &WindowEvent) {
    debug!("Window event: {:?} on {:?}", event, window.label());

    if let WindowEvent::CloseRequested { api, .. } = event {
        info!("CloseRequested event received on {:?}", window.label());
        if window.label() == "main" {
            api.prevent_close();
            window.hide().unwrap();
        }
    }
}

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up app...");
    let handle = app.handle();

    #[cfg(desktop)]
    {
        handle.plugin(tauri_plugin_window_state::Builder::default().build())?;
    }

    let mut builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default());

    #[cfg(desktop)]
    {
        builder = builder
            .user_agent(&format!("HN app - {}", std::env::consts::OS))
            .title("HackerNews")
            .inner_size(1200., 800.)
            .min_inner_size(800., 600.)
            .content_protected(true)
            .resizable(true);
    }

    let webview = builder.build();

    #[cfg(debug_assertions)]
    {
        webview.expect("Failed to build webview").open_devtools();
    }

    Ok(())
}
fn logger() -> tauri_plugin_log::Builder {
    tauri_plugin_log::Builder::default()
        .targets([
            Target::new(TargetKind::Webview),
            Target::new(TargetKind::Folder {
                path: utils::log_dir(),
                file_name: Some("app.log".to_string()),
            }),
            Target::new(TargetKind::Stdout),
        ])
        .level(tracing::log::LevelFilter::Info)
}
