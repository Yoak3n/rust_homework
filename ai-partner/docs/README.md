# AI对话助手

基于Tauri的Windows桌面应用，结合Rust后端与Vue3前端，提供简便的智能对话功能。

## 功能特性

✅ 消息历史管理  
✅ 流式响应交互  
✅ 对话上下文重试  
✅ 自适应布局界面  
✅ 文本生成暂停控制  
✅ 全局唤出快捷对话

## 技术栈

- 前端框架：Vue 3 + TypeScript
- UI组件库：Naive UI
- 状态管理：Pinia
- 后端框架：Rust + Tauri2
- AI集成：OpenAI API

## 安装使用
### 发行版
目前仅提供Windows x64版本的发行版，可在[Release](https://github.com/Yoak3n/rust_homework/releases)页面下载，如需要其他版本可尝试自行编译构建

### 环境要求
- Rust 1.70+
- Node.js 18+
- Pnpm

### 快速启动
```bash
# 安装依赖
pnpm install

# 开发模式
pnpm run tauri dev

# 生产构建
pnpm run tauri build
```

## 注意事项
1. 首次运行前请检查运行环境是否满足要求：Windows系统需安装 WebView2，系统版本在Windows10及以上，否则可能无法正常运行
2. API调用需要有效的OpenAI凭证或兼容OpenAI API的AI对话服务