# ClientApps Project 🚀

这是一个使用 Rust 和 Dioxus 构建的客户端应用程序集合项目，目前包含了 Hacker News 客户端。

## 项目结构 📁

- `hackernews/` - Hacker News 客户端应用 (Web 版本)
- `src-tauri/` - Tauri 桌面应用框架
- `project_schedule/` - 项目进度跟踪
- 其他配置文件和文档

## 已完成功能 ✅

### Hacker News 客户端 (Web 版本)

1. **数据结构定义**

   - `StoryItem`: 新闻条目结构
   - `Comment`: 评论结构
   - `StroryData`: 包含新闻和评论的数据结构

2. **API 功能实现**

   - 获取热门新闻列表
   - 根据 ID 获取特定新闻
   - 获取新闻的所有评论
   - 根据 ID 获取特定评论

3. **UI 组件实现**

   - StoryItem 组件：用于显示新闻条目信息

4. **技术要点**
   - 使用 `reqwest` 发起 HTTP 请求获取 Hacker News API 数据
   - 使用 `serde` 进行数据序列化和反序列化
   - 使用 `futures` 和 `join_all` 实现并发请求提高性能
   - 正确处理 Rust 生命周期问题确保内存安全
   - 添加单元测试保证代码质量
   - 修复 Dioxus RSX 语法问题确保 UI 正确渲染

### Hacker News 客户端 (桌面版本)

1. **Tauri 框架集成**

   - 初始化 Tauri 2.x 桌面应用框架
   - 配置应用窗口属性和基本行为

2. **命令功能实现**

   - `greet` 命令：基础的前后端通信示例
   - `get_app_dir` 命令：获取应用数据目录路径

3. **日志系统**

   - 集成 `tauri-plugin-log` 插件
   - 配置多目标日志输出（Webview、文件、标准输出）

4. **应用数据管理**

   - 实现跨平台应用数据目录管理
   - 为日志、缓存等创建专用子目录

5. **窗口管理**

   - 实现关闭窗口时隐藏而非退出应用
   - 添加窗口事件和页面加载日志

6. **代码质量改进**
   - 修复了 Tauri 命令函数的编译错误
   - 为所有 Tauri 源文件添加了详细的行级注释
   - 优化了错误处理和类型匹配问题

## 运行项目 ▶️

请参考各个子项目目录下的 README.md 文件了解如何运行对应的应用。

对于 Hacker News 客户端，请查看 [hackernews/README.md](./hackernews/README.md)。

## 项目进度 📅

详细进度请查看 [project_schedule/](./project_schedule/) 目录下的每日进度报告。
