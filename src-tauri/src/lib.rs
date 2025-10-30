use std::str::FromStr;

use anyhow::Result;
use tauri::{
    menu::{
        CheckMenuItem, IconMenuItem, Menu, MenuId, MenuItem, PredefinedMenuItem, Submenu,
        SubmenuBuilder,
    },
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    webview::PageLoadPayload,
    App, AppHandle, Builder, Manager, Runtime, Webview, WebviewUrl, WebviewWindowBuilder, Window,
    WindowEvent, Wry,
};
mod commends;
mod utils;
const APP_NAME: &str = "hn";
use tauri_plugin_log::{
    log::{debug, info},
    Target, TargetKind,
};

use commends::{get_app_dir, greet};

/// Tauri 应用程序的主构建函数
/// 返回一个配置好的 Tauri Builder 实例
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub fn app() -> Result<Builder<Wry>> {
    let builder = tauri::Builder::default()
        // 初始化 opener 插件，用于打开外部链接
        .plugin(tauri_plugin_opener::init())
        // 初始化日志插件
        .plugin(logger().build())
        // 注册可以从前端调用的命令处理函数
        .invoke_handler(tauri::generate_handler![greet, get_app_dir])
        // 设置应用启动时的初始化逻辑
        .setup(setup)
        // 页面加载事件处理函数
        .on_page_load(on_page_load)
        // 窗口事件处理函数
        .on_window_event(on_window_event_handler);
    Ok(builder)
}

/// 页面加载事件处理函数
/// 当 webview 页面加载完成时会被调用
fn on_page_load<'a>(webview: &Webview, _payload: &PageLoadPayload<'_>) {
    info!("Page loaded: {}", webview.label());
}

/// 窗口事件处理函数
/// 处理窗口相关的事件，如关闭请求等
fn on_window_event_handler(window: &Window, event: &WindowEvent) {
    debug!("Window event: {:?} on {:?}", event, window.label());

    if let WindowEvent::CloseRequested { api, .. } = event {
        info!("CloseRequested event received on {:?}", window.label());
        // 对于主窗口，防止真正关闭，而是隐藏窗口
        if window.label() == "main" {
            api.prevent_close();
            window.hide().unwrap();
        }
    }
}

/// 应用程序设置函数
/// 在应用启动时调用，用于初始化各种组件
fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up app...");
    let handle = app.handle();

    #[cfg(desktop)]
    {
        // 在桌面平台上初始化窗口状态插件
        handle.plugin(tauri_plugin_window_state::Builder::default().build())?;
    }
    // 设置应用菜单
    setup_menu(handle)?;
    // 创建主 webview 窗口
    let mut builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default());

    #[cfg(desktop)]
    {
        builder = builder
            // 设置用户代理字符串
            .user_agent(&format!("HN app - {}", std::env::consts::OS))
            // 设置窗口标题
            .title("HackerNews")
            // 设置窗口初始大小
            .inner_size(1200., 800.)
            // 设置窗口最小大小
            .min_inner_size(800., 600.)
            // 设置内容保护（防止截屏）
            .content_protected(true)
            // 设置窗口可调整大小
            .resizable(true);
    }

    let webview = builder.build();

    #[cfg(debug_assertions)]
    {
        // 在调试模式下打开开发者工具
        webview.expect("Failed to build webview").open_devtools();
    }

    Ok(())
}

/// 日志配置函数
/// 配置日志输出目标和级别
fn logger() -> tauri_plugin_log::Builder {
    tauri_plugin_log::Builder::default()
        .targets([
            // 在 webview 中输出日志
            Target::new(TargetKind::Webview),
            // 输出到文件
            Target::new(TargetKind::Folder {
                path: utils::log_dir(),
                file_name: Some("app.log".to_string()),
            }),
            // 输出到标准输出
            Target::new(TargetKind::Stdout),
        ])
        .level(tracing::log::LevelFilter::Info)
}

/// 菜单设置函数
/// 创建应用菜单和托盘菜单
pub fn setup_menu<R: Runtime>(app: &AppHandle<R>) -> Result<(), tauri::Error> {
    // 创建文件菜单
    let file_menu = SubmenuBuilder::with_id(app, "file", "File")
        // 添加打开菜单项
        .item(&MenuItem::with_id(
            app,
            "file_open",
            "打开",
            true,
            Some("CmdOrCtrl+o"),
        )?)
        // 添加保存菜单项
        .item(&MenuItem::with_id(
            app,
            "file_save",
            "保存",
            true,
            Some("CmdOrCtrl+s"),
        )?)
        // 添加另存为菜单项
        .item(&MenuItem::with_id(
            app,
            "file_save_as",
            "保存为",
            true,
            Some("CmdOrCtrl+Shift+s"),
        )?)
        // 添加分隔线
        .separator()
        // 添加退出菜单项
        .quit()
        .build()?;

    // 创建编辑菜单
    let edit_menu = SubmenuBuilder::with_id(app, "edit", "Edit")
        // 添加处理菜单项
        .item(&MenuItem::with_id(
            app,
            "edit_process",
            "处理",
            true,
            Some("CmdOrCtrl+p"),
        )?)
        // 添加分隔线
        .separator()
        // 添加撤销菜单项
        .undo()
        // 添加重做菜单项
        .redo()
        // 添加分隔线
        .separator()
        // 添加剪切菜单项
        .cut()
        // 添加复制菜单项
        .copy()
        // 添加粘贴菜单项
        .paste()
        // 添加分隔线
        .separator()
        // 添加全选菜单项
        .select_all()
        // 添加复选菜单项
        .item(&CheckMenuItem::with_id(
            app,
            "edit_check_me",
            "检查",
            true,
            false,
            None::<&str>,
        )?)
        .build()?;

    // 获取默认窗口图标
    let icon = app.default_window_icon().unwrap().clone();

    // 创建托盘菜单
    let tray_menu = SubmenuBuilder::with_id(app, "tray", "Tray")
        // 添加打开菜单项
        .item(&MenuItem::with_id(
            app,
            "tray_open",
            "打开",
            true,
            None::<&str>,
        )?)
        // 添加隐藏菜单项
        .item(&MenuItem::with_id(
            app,
            "tray_hide",
            "隐藏",
            true,
            None::<&str>,
        )?)
        // 添加分隔线
        .separator()
        // 添加退出菜单项
        .quit()
        .build()?;

    // 创建托盘图标
    TrayIconBuilder::with_id(format!("{}-tray", APP_NAME))
        // 设置托盘图标提示文本
        .tooltip("Hacker News")
        // 设置托盘图标
        .icon(icon)
        // 设置托盘菜单
        .menu(&tray_menu)
        // 设置左键点击时显示菜单
        .show_menu_on_left_click(true)
        // 设置托盘图标事件处理函数
        .on_tray_icon_event(|tray, event| {
            info!("Tray icon event: {:?}", event);
            // 右键点击时打开主窗口
            if let TrayIconEvent::Click {
                button: MouseButton::Right,
                ..
            } = event
            {
                open_main(tray.app_handle()).unwrap();
            }
        })
        .build(app)?;

    // 创建应用菜单
    let menu = Menu::with_items(app, &[&file_menu, &edit_menu])?;
    app.set_menu(menu)?;

    // 设置菜单事件处理函数
    app.on_menu_event(|app, event: tauri::menu::MenuEvent| {
        info!("Menu event: {:?}", event);
        match event.id.as_ref() {
            "file_open" => open_main(app).unwrap(),
            "file_save" => info!("Save"),
            "file_save_as" => info!("Save as"),
            "edit_process" => info!("Process"),
            "edit_check_me" => info!("Check me"),
            "tray_open" => open_main(app).unwrap(),
            "tray_hide" => info!("Hide"),
            _ => {}
        }
    });
    Ok(())
}

/// 菜单项类型枚举
/// 定义了不同类型的菜单项
pub enum MenuItemKind<R: Runtime> {
    /// 普通菜单项
    MenuItem(MenuItem<R>),
    /// 子菜单项
    Submenu(Submenu<R>),
    /// 预定义菜单项
    Predefined(PredefinedMenuItem<R>),
    /// 复选菜单项
    Check(CheckMenuItem<R>),
    /// 图标菜单项
    Icon(IconMenuItem<R>),
}

/// 打开主窗口函数
/// 显示隐藏的主窗口
fn open_main<R: Runtime>(handle: &AppHandle<R>) -> Result<(), tauri::Error> {
    handle
        .get_webview_window("main")
        .ok_or_else(|| tauri::Error::WindowNotFound)?
        .show()?;
    Ok(())
}
