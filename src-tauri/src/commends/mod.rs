/// Tauri 命令函数，用于向用户打招呼
/// 接收一个名字参数，返回问候语字符串
/// 可以从前端 JavaScript 通过 IPC 调用: invoke('greet', { name: 'World' })
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Tauri 命令函数，用于获取应用程序目录路径
/// 返回应用程序目录的字符串表示
/// 可以从前端 JavaScript 通过 IPC 调用: invoke('get_app_dir')
#[tauri::command]
pub fn get_app_dir() -> String {
    crate::utils::app_dir().display().to_string()
}
