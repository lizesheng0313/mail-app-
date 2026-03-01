#!/bin/bash
# 本地模拟 Jenkins 镜像环境验证脚本
# 用法: cd mail_desktop && bash test-build-mirror.sh
# 跑通后再推代码到 Jenkins 构建

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TAURI_DIR="$SCRIPT_DIR/src-tauri"
BACKUP_CONFIG=""

echo "========================================="
echo "  Cargo 镜像源构建验证"
echo "========================================="

# 备份现有 cargo config
if [ -f "$HOME/.cargo/config.toml" ]; then
    BACKUP_CONFIG="$HOME/.cargo/config.toml.bak.$$"
    cp "$HOME/.cargo/config.toml" "$BACKUP_CONFIG"
    echo "[1/5] 已备份现有 cargo 配置 -> $BACKUP_CONFIG"
else
    echo "[1/5] 无现有 cargo 配置，跳过备份"
fi

# 还原函数
restore_config() {
    if [ -n "$BACKUP_CONFIG" ] && [ -f "$BACKUP_CONFIG" ]; then
        mv "$BACKUP_CONFIG" "$HOME/.cargo/config.toml"
        echo ""
        echo "[恢复] 已还原原始 cargo 配置"
    elif [ -z "$BACKUP_CONFIG" ]; then
        rm -f "$HOME/.cargo/config.toml"
    fi
}
trap restore_config EXIT

# 写入和 Jenkins 一样的镜像配置
echo "[2/5] 写入中科大镜像配置（与 Jenkins 一致）..."
mkdir -p "$HOME/.cargo"
cat > "$HOME/.cargo/config.toml" << 'EOF'
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"

[net]
git-fetch-with-cli = true
EOF

# 测试镜像连通性
echo "[3/5] 测试镜像连通性..."
if curl -sf --connect-timeout 10 "https://mirrors.ustc.edu.cn/crates.io-index/config.json" > /dev/null 2>&1; then
    echo "      中科大镜像连通 OK"
else
    echo "      中科大镜像连通失败!"
    exit 1
fi

# cargo fetch - 下载所有依赖（不编译，速度快）
echo "[4/5] cargo fetch（通过镜像下载全部依赖）..."
cd "$TAURI_DIR"
cargo fetch 2>&1

echo "[5/5] cargo check（类型检查，不生成二进制）..."
cargo check 2>&1

echo ""
echo "========================================="
echo "  全部通过! 可以放心推代码构建了"
echo "========================================="
