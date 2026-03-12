# Render 部署指南

## 📋 架构概述

```
┌─────────────────┐         ┌─────────────────┐
│   Frontend      │ ──────→ │    Backend      │
│   (Static Site) │  API    │  (Rust/Actix)   │
└─────────────────┘         └────────┬────────┘
                                    │
                                    ↓
                           ┌─────────────────┐
                           │   PostgreSQL    │
                           │  (Managed DB)   │
                           └─────────────────┘
```

## 🔧 需要修改的代码

### 1. 前端 - 环境变量支持 ✅ 已完成

前端已配置环境变量注入，构建时会自动替换 API URL:

**文件**: `frontend/index.html`
```javascript
// 构建时会被替换为实际的 API URL
const API_BASE = 'https://hello-claw-backend.onrender.com/api';
```

**构建脚本**: `frontend/build.sh`
- 自动读取 `VITE_API_URL` 环境变量
- 在构建时注入到 `index.html`
- 本地开发默认使用 `http://localhost:10000/api`

### 2. 后端 - 数据库配置（可选）

如果要用 PostgreSQL，修改 `backend/Cargo.toml`:

```toml
[dependencies]
actix-web = "4"
actix-cors = "0.7"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }  # 改为 postgres
tokio = { version = "1", features = ["full"] }
```

修改数据库初始化代码:

```rust
use sqlx::postgres::PgPoolOptions;
let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&std::env::var("DATABASE_URL")?)
    .await?;
```

### 3. 后端 - 添加迁移脚本（如果用 PostgreSQL）

创建 `backend/migrations/001_init.sql`:

```sql
CREATE TABLE IF NOT EXISTS tasks (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    priority INTEGER DEFAULT 0,
    status VARCHAR(50) DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

在 `main.rs` 中运行迁移:

```rust
sqlx::migrate!("./migrations").run(&pool).await?;
```

## 🚀 部署步骤

### Step 1: 初始化 Git 仓库
```bash
cd /home/bot/.openclaw/workspace/repos/2026-03-11-hello-claw
git add .
git commit -m "feat: optimize render deployment with env vars"
git push origin main
```

### Step 2: 连接 Render

1. 登录 [Render Dashboard](https://dashboard.render.com)
2. 点击 **New +** → **Blueprint**
3. 连接你的 GitHub 仓库
4. 选择 `render.yaml` 文件
5. 点击 **Apply**

### Step 3: 验证部署

Render 会自动:
- 创建 PostgreSQL 数据库（可选）
- 部署后端服务
- 部署前端静态站点（自动注入后端 API URL）
- 配置服务间通信

部署完成后，你会得到两个 URL:
- 前端：`https://hello-claw-frontend.onrender.com`
- 后端：`https://hello-claw-backend.onrender.com`

前端会自动调用后端的 API，无需手动配置！✨

## 📊 服务配置说明

| 服务 | 类型 | 区域 | 计划 | 说明 |
|------|------|------|------|------|
| hello-claw-backend | Web Service | Oregon | Free | Rust 后端 API |
| hello-claw-frontend | Static Site | Oregon | Free | 前端静态页面 |
| hello-claw-db | PostgreSQL | Oregon | Free | 托管数据库 (1GB，可选) |

## 🔌 服务间通信

### 前端 → 后端

通过环境变量自动配置:

```yaml
# render.yaml
envVars:
  - key: VITE_API_URL
    value: https://hello-claw-backend.onrender.com
```

构建脚本 `frontend/build.sh` 会在构建时读取 `VITE_API_URL`，并注入到 `index.html` 中。

### 后端 → 数据库

```yaml
# render.yaml
envVars:
  - key: DATABASE_URL
    fromDatabase:
      name: hello-claw-db
      property: connectionString
```

## ⚠️ 注意事项

1. **Free Plan 限制**:
   - 后端服务 15 分钟无请求会休眠
   - 首次请求需要 30-50 秒冷启动
   - 数据库 1GB 存储限制

2. **环境变量**:
   - `DATABASE_URL` 自动从数据库服务注入（如果用 PostgreSQL）
   - `VITE_API_URL` 在 render.yaml 中配置，构建时注入前端

3. **构建缓存**:
   - Rust 构建会使用缓存，后续部署更快
   - 使用 `cargo install --path .` 确保正确的 binary

4. **CORS 配置**:
   - ✅ 后端已配置 `allow_any_origin()`，允许所有来源访问
   - 生产环境建议限制为特定域名更安全

## 🔍 故障排查

### 查看日志
```bash
# Render Dashboard → Services → 点击服务 → Logs
```

### 常见问题

**后端启动失败**:
- 检查 `DATABASE_URL` 是否正确配置
- 确保迁移脚本正确执行
- 查看构建日志确认 cargo build 成功

**前端无法连接后端**:
- 检查 CORS 配置
- 确认 API URL 正确
- 查看浏览器控制台错误

**数据库连接错误**:
- 确保 sqlx features 包含 `postgres`
- 检查迁移是否执行

## 📚 参考资料

- [Render 多服务架构文档](https://render.com/docs/multi-service-architecture)
- [Render Blueprint 规范](https://render.com/docs/blueprint-spec)
- [sqlx PostgreSQL](https://docs.rs/sqlx/latest/sqlx/postgres/index.html)
