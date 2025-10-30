use std::path::PathBuf;

/// 应用程序主目录名称
const APP_DIR: &str = ".hn";
/// 日志目录名称
const LOG_DIR: &str = "log";
/// 缓存目录名称
const CACHE_DIR: &str = "cache";
/// 数据库目录名称
const DB_DIR: &str = "db";
/// 配置目录名称
const CONFIG_DIR: &str = "config";

/// 内联函数，获取应用程序主目录路径
/// 返回用户主目录下的 .hn 目录路径
#[inline]
pub(crate) fn app_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Failed to get home dir")
        .join(APP_DIR)
}

/// 内联函数，获取日志目录路径
/// 返回应用程序主目录下的 log 子目录路径
#[inline]
pub(crate) fn log_dir() -> PathBuf {
    app_dir().join(LOG_DIR)
}

/// 内联函数，获取缓存目录路径
/// 返回应用程序主目录下的 cache 子目录路径
#[inline]
pub(crate) fn cache_dir() -> PathBuf {
    app_dir().join(CACHE_DIR)
}

/// 内联函数，获取数据库目录路径
/// 返回应用程序主目录下的 db 子目录路径
#[inline]
pub(crate) fn db_dir() -> PathBuf {
    app_dir().join(DB_DIR)
}

/// 内联函数，获取配置目录路径
/// 返回应用程序主目录下的 config 子目录路径
#[inline]
pub(crate) fn config_dir() -> PathBuf {
    app_dir().join(CONFIG_DIR)
}
