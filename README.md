# ClientApps Project 🚀

这是一个使用 Rust 和 Dioxus 构建的客户端应用程序集合项目，目前包含了 Hacker News 客户端。

## 项目结构 📁

- `hackernews/` - Hacker News 客户端应用
- `project_schedule/` - 项目进度跟踪
- 其他配置文件和文档

## 已完成功能 ✅

### Hacker News 客户端

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

## 运行项目 ▶️

请参考各个子项目目录下的 README.md 文件了解如何运行对应的应用。

对于 Hacker News 客户端，请查看 [hackernews/README.md](./hackernews/README.md)。

## 项目进度 📅

详细进度请查看 [project_schedule/progress.md](./project_schedule/progress.md)。
