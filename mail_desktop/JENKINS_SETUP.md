# Jenkins 桌面应用部署配置

## 1. 在 Jenkins 创建新任务

1. 访问 Jenkins: http://localhost:8081
2. 点击"新建任务"
3. 输入任务名称：`mail_desktop_deploy`
4. 选择"流水线"
5. 点击"确定"

## 2. 配置任务

### 基本配置
- 描述：`邮件管理桌面应用自动构建和部署`

### 参数化构建
已在 Jenkinsfile 中定义，会自动显示：
- `DESKTOP_VERSION`: 版本号（例如：0.2.0）
- `SIGNING_PASSWORD`: 签名私钥密码（密码类型）
- `AUTO_DEPLOY`: 是否自动部署（默认：true）

### 流水线配置
- 定义：Pipeline script from SCM
- SCM：Git
- Repository URL：你的 Git 仓库地址
- Credentials：选择你的 Git 凭据
- 分支：`*/main` 或 `*/master`
- 脚本路径：`mail_desktop/Jenkinsfile.simple`

## 3. 配置 SSH 凭据

如果还没有配置服务器 SSH 凭据：

1. Jenkins 首页 → 系统管理 → Manage Credentials
2. 选择 "全局凭据"
3. 点击 "添加凭据"
4. 类型：SSH Username with private key
5. ID：`server-ssh-key`
6. Username：`root`
7. Private Key：选择"直接输入"，粘贴你的私钥
8. 点击"确定"

## 4. 准备签名密钥

在 Jenkins 服务器上生成签名密钥（只需一次）：

```bash
# SSH 到 Jenkins 服务器
ssh jenkins@your-jenkins-server

# 切换到 Jenkins 用户
sudo su - jenkins

# 生成签名密钥
cd /path/to/workspace/mail_desktop/src-tauri
pnpm tauri signer generate -w ~/.tauri/mailapp.key

# 记住你设置的密码！
```

## 5. 配置服务器目录

在部署服务器上创建必要的目录：

```bash
ssh root@8.140.27.159

# 创建下载目录
mkdir -p /var/www/html/downloads

# 创建更新清单目录
mkdir -p /var/www/html/desktop-updates/{darwin-aarch64,darwin-x86_64,windows-x86_64}

# 设置权限
chmod 755 /var/www/html/downloads
chmod 755 /var/www/html/desktop-updates
```

## 6. 配置 Nginx

在服务器的 Nginx 配置中添加：

```nginx
server {
    listen 443 ssl;
    server_name zjkdongao.cn;

    # 下载文件
    location /downloads/ {
        alias /var/www/html/downloads/;
        autoindex off;
        add_header Content-Disposition 'attachment';
    }

    # 更新检查接口
    location /desktop-updates/ {
        alias /var/www/html/desktop-updates/;
        default_type application/json;
        add_header Access-Control-Allow-Origin *;
    }
}
```

重启 Nginx：
```bash
nginx -t
systemctl reload nginx
```

## 7. 首次构建

1. 访问 Jenkins 任务页面
2. 点击"Build with Parameters"
3. 填写参数：
   - DESKTOP_VERSION: `0.1.0`
   - SIGNING_PASSWORD: 你的签名密钥密码
   - AUTO_DEPLOY: ✓
4. 点击"开始构建"

## 8. 构建流程

Jenkins 会自动执行以下步骤：

1. ✅ 准备环境，检查签名密钥
2. ✅ 更新版本号
3. ✅ 安装依赖
4. ✅ 构建前端资源
5. ✅ 构建桌面应用（10-15分钟）
6. ✅ 上传安装包到服务器
7. ✅ 生成自动更新清单
8. ✅ 更新下载页面版本号
9. ✅ 自动触发前端部署（更新下载页面）

**注意**：整个流程会自动完成，包括前端部署，无需手动操作。

## 9. 发布新版本

每次发布新版本：

1. 修改代码
2. 提交到 Git
3. 访问 Jenkins
4. 点击"Build with Parameters"
5. 输入新版本号（例如：0.2.0）
6. 输入签名密钥密码
7. 开始构建
8. **等待完成**（会自动触发前端部署，更新下载页面）

**整个流程完全自动化**，无需手动部署前端！

## 10. 验证部署

构建完成后：

1. 访问下载页面：https://zjkdongao.cn/download
2. 检查版本号是否更新
3. 下载安装包测试
4. 测试自动更新功能（用旧版本打开，应该提示更新）

## 11. 查看更新清单

```bash
# macOS Apple Silicon
curl https://zjkdongao.cn/desktop-updates/darwin-aarch64/0.1.0

# macOS Intel
curl https://zjkdongao.cn/desktop-updates/darwin-x86_64/0.1.0

# Windows
curl https://zjkdongao.cn/desktop-updates/windows-x86_64/0.1.0
```

## 12. 故障排查

### 构建失败
- 检查 Jenkins 日志
- 确认签名密钥密码正确
- 确认依赖已安装

### 上传失败
- 检查 SSH 凭据配置
- 确认服务器目录权限
- 检查网络连接

### 自动更新不工作
- 检查更新清单是否生成
- 确认 Nginx 配置正确
- 检查签名是否正确

## 注意事项

⚠️ **重要**：
1. 签名密钥密码必须保密
2. 每次构建都需要输入密码
3. 构建时间较长（10-15分钟），请耐心等待
4. 发布新版本后，记得重新部署前端（更新下载页面）
