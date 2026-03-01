# Mail Desktop (Tauri)

邮件管理桌面客户端，使用 Tauri 构建。

## 架构说明

```
mail_desktop/
├── src-tauri/              # Rust 后端
│   ├── src/
│   │   ├── lib.rs          # 入口，注册命令
│   │   ├── commands.rs     # Tauri 命令（前端调用的接口）
│   │   └── mail/           # 邮件模块
│   │       ├── mod.rs
│   │       ├── types.rs    # 类型定义
│   │       ├── imap.rs     # IMAP 实现
│   │       └── pop3.rs     # POP3 实现
│   ├── tauri.conf.json     # Tauri 配置
│   └── Cargo.toml          # Rust 依赖
└── package.json            # npm 脚本
```

## 前提条件

- Rust 1.77+
- Node.js 18+
- pnpm

## 开发

### 1. 启动前端（另一个终端）

```bash
cd ../mail_frontend
pnpm dev
```

### 2. 启动 Tauri 开发模式

```bash
cd mail_desktop
pnpm dev
# 或
cargo tauri dev
```

## 构建

```bash
pnpm build
# 或
cargo tauri build
```

构建产物位置：
- macOS: `src-tauri/target/release/bundle/dmg/`
- Windows: `src-tauri/target/release/bundle/msi/`

## 前端调用示例

```typescript
import { invoke } from '@tauri-apps/api/tauri';

// 检查是否在 Tauri 环境
const isTauriEnv = () => '__TAURI__' in window;

// 添加外部邮箱（本地验证登录）
async function addExternalMailbox(data: {
  email: string;
  password: string;
  protocol: 'imap' | 'pop3';
  host?: string;
  port?: number;
}) {
  if (isTauriEnv()) {
    return await invoke('add_external_mailbox', data);
  } else {
    // Web 端走远程 API
    return await httpClient.post('/api/unified/external-mailboxes', data);
  }
}

// 收取邮件（本地连接邮箱服务器）
async function fetchEmails(data: {
  mailboxId: number;
  email: string;
  password: string;
  protocol: string;
  host: string;
  port: number;
  token: string;
  serverUrl: string;
}) {
  if (isTauriEnv()) {
    return await invoke('fetch_emails', {
      mailbox_id: data.mailboxId,
      email: data.email,
      password: data.password,
      protocol: data.protocol,
      host: data.host,
      port: data.port,
      token: data.token,
      server_url: data.serverUrl,
    });
  } else {
    return await httpClient.post(`/api/unified/external-mailboxes/${data.mailboxId}/fetch`);
  }
}
```

## 数据流

```
用户操作 → 前端 → IPC → Rust 后端
                          ↓
              用户本地 IP 连接邮箱服务器
                          ↓
                    收取邮件成功
                          ↓
              HTTP 同步到远程 mail_backend
                          ↓
                    存入数据库
```
