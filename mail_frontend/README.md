# 邮件服务管理平台

一个基于 Vue 3 + TypeScript + Tailwind CSS 的现代化邮件服务管理前端应用。

## 功能特性

### 🔐 授权管理
- 授权码验证登录
- 安全的身份认证机制
- 自动获取关联邮箱账号

### 📧 邮箱管理
- 多邮箱账号统一管理
- 邮件收发功能
- 邮件列表查看和搜索
- 邮件详情查看和回复
- 实时邮件状态更新



### 🌐 域名管理
- 140+ 域名统一管理
- 域名状态监控
- 邮箱数量统计
- 域名搜索和过滤
- 分页浏览

## 技术栈

- **前端框架**: Vue 3 + TypeScript
- **状态管理**: Pinia
- **路由**: Vue Router 4
- **样式**: Tailwind CSS
- **构建工具**: Vite
- **HTTP 客户端**: Axios
- **代码规范**: ESLint + Prettier

## 项目结构

```
src/
├── components/          # 公共组件
├── views/              # 页面组件
│   ├── Authorization.vue    # 授权登录页
│   ├── Dashboard.vue       # 仪表板
│   ├── MailboxManagement.vue # 邮箱管理

│   └── DomainManagement.vue  # 域名管理
├── stores/             # Pinia 状态管理
│   ├── auth.ts         # 认证状态
│   └── mail.ts         # 邮件相关状态
├── services/           # API 服务
│   └── api.ts          # API 接口定义
├── types/              # TypeScript 类型定义
│   └── index.ts        # 通用类型
├── router/             # 路由配置
│   └── index.ts
├── style.css           # 全局样式
└── main.ts             # 应用入口
```

## 快速开始

### 安装依赖

```bash
npm install
```

### 开发环境运行

```bash
npm run dev
```

应用将在 http://localhost:3000 启动

### 构建生产版本

```bash
npm run build
```

### 预览生产版本

```bash
npm run preview
```

## API 接口

### 认证相关
- `POST /api/auth/verify` - 验证授权码
- `GET /api/auth/mailboxes` - 获取邮箱列表

### 邮件相关
- `GET /api/mailboxes/:id/emails` - 获取邮件列表
- `POST /api/emails/send` - 发送邮件
- `PATCH /api/emails/:id` - 更新邮件状态



### 域名相关
- `GET /api/domains` - 获取域名列表
- `GET /api/domains/:id` - 获取域名详情
- `PATCH /api/domains/:id` - 更新域名状态

## 环境配置

在 `src/services/api.ts` 中配置 API 基础地址：

```typescript
const api = axios.create({
  baseURL: process.env.NODE_ENV === 'production' 
    ? 'https://your-api-domain.com/api'  // 生产环境 API 地址
    : 'http://localhost:8080/api',       // 开发环境 API 地址
})
```

## 设计特点

### 🎨 现代化 UI
- 简洁优雅的界面设计
- 响应式布局，支持移动端
- 一致的视觉风格和交互体验

### 🚀 性能优化
- 组件懒加载
- 数据分页加载
- 优化的打包体积

### 🔧 开发体验
- TypeScript 类型安全
- 组件化开发
- 热重载开发环境

## 浏览器支持

- Chrome >= 87
- Firefox >= 78
- Safari >= 14
- Edge >= 88

## 许可证

MIT License
