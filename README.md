# Rust-React 全栈项目

这是一个使用 Rust (Axum 框架) 作为后端和 React (Vite, TypeScript) 作为前端的全栈项目。

## 技术栈

### 后端
- **Rust**: 后端主要编程语言。
- **Axum**: 用于构建 Web 服务的 Rust 框架。
- **SQLx**: 异步 Rust SQL 库，用于数据库 ORM (此处使用 SQLite)。
- **Argon2**: 用于密码哈希。
- **Config**: 用于管理应用程序配置。
- **Tokio**: 异步运行时。

### 前端
- **React**: 用于构建用户界面的 JavaScript 库。
- **Vite**: 极速的现代前端构建工具。
- **TypeScript**: 强类型 JavaScript 的超集。
- **React Router DOM**: 用于前端路由。

## 项目结构

- `backend/`: 包含 Rust 后端服务的所有代码。
- `frontend/`: 包含 React 前端应用的所有代码。

## 设置与安装

请确保你已安装以下工具：
- [Node.js](https://nodejs.org/) (推荐 LTS 版本)
- [npm](https://www.npmjs.com/) (通常随 Node.js 一起安装)
- [Rust 和 Cargo](https://www.rust-lang.org/tools/install)
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) (用于数据库迁移)
  ```bash
  cargo install sqlx-cli --no-default-features --features postgres,mysql,sqlite
  ```

### 1. 克隆仓库

```bash
git clone YOUR_GITHUB_REPO_URL # 替换为你的实际 GitHub 仓库地址
cd project_test
```

### 2. 后端设置

进入 `backend` 目录，安装 Rust 依赖并运行数据库迁移：

```bash
cd backend
cargo build # 编译项目以确保所有依赖都已下载和编译
sqlx migrate run # 运行数据库迁移，这将创建 sqlite.db 文件和 users 表
cd ..
```

### 3. 前端设置

进入 `frontend` 目录，安装 npm 依赖：

```bash
cd frontend
npm install
cd ..
```

## 运行应用程序

### 1. 启动后端服务

在项目根目录打开一个终端，进入 `backend` 目录并运行后端服务：

```bash
cd backend
cargo run
```

后端服务将会在 `http://0.0.0.0:3000` 监听。

### 2. 启动前端服务

在项目根目录打开**另一个**终端，进入 `frontend` 目录并运行前端开发服务器：

```bash
cd frontend
npx vite
```

前端应用通常会在 `http://localhost:5173` (或类似端口，请查看终端输出) 监听。

## API 接口

后端提供了以下 API 接口：

- `POST /register`: 用户注册接口。
  - Request Body: `{
    "email": "user@example.com",
    "password": "your_password"
  }`
  - Response: `{
    "message": "User registered successfully"
  }` 或错误信息。

- `POST /login`: 用户登录接口。
  - Request Body: `{
    "email": "user@example.com",
    "password": "your_password"
  }`
  - Response: `{
    "message": "Login successful!"
  }` 或错误信息。

## 数据库

本项目使用 SQLite 数据库，数据库文件 `sqlite.db` 将在 `backend/` 目录下生成。 