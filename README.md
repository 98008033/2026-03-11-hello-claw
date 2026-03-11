# 🧞 Hello Claw

一个动态计数前端页面，带有炫酷的点击特效！

## ✨ 功能特点

- 🔥 **动态按钮效果** - 点击时有波纹、粒子、弹跳动画
- 💫 **连击系统** - 快速点击显示 Combo 计数
- ⌨️ **键盘支持** - 空格键即可点击
- 🦀 **Rust 后端** - 高性能 Actix-web 服务器
- 💾 **SQLite 数据库** - 持久化存储计数

## 🚀 快速开始

### 后端启动

```bash
cd backend
cargo build --release
./target/release/hello_claw_backend
```

服务地址：`http://localhost:10000`

### API 端点

| 方法 | 端点 | 说明 |
|------|------|------|
| GET | `/api/count` | 获取当前计数 |
| POST | `/api/increment` | 增加计数 |
| POST | `/api/reset` | 重置计数 |

### 前端使用

直接在浏览器打开 `frontend/index.html` 即可！

## 📁 项目结构

```
hello-claw/
├── backend/
│   ├── Cargo.toml
│   └── src/main.rs
├── frontend/
│   ├── index.html
│   └── package.json
└── README.md
```

## 🎨 特效展示

- ✨ 粒子飞散效果 (emoji: ✨🌟⭐💫🧞🪄)
- 🌊 点击波纹扩散
- 💫 数字缩放弹跳
- 🔥 Combo 连击显示

## 🛠️ 技术栈

**后端:**
- Rust 🦀
- Actix-web
- SQLite + SQLx

**前端:**
- HTML5 + CSS3
- Vanilla JavaScript
- CSS Animations

## 📋 开发规范

遵循 `development-standard.md` 中的开发流程。

---

**Made with 🧞 by 98008033**
