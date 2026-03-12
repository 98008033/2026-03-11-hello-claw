#!/bin/bash
# 前端构建脚本 - 注入环境变量

set -e

echo "🔧 Building frontend..."

# 如果环境变量存在，替换占位符
if [ -n "$VITE_API_URL" ]; then
    echo "📡 Using API URL from environment: $VITE_API_URL"
    # 直接替换整行 API_BASE 定义（使用#分隔符避免 URL 中的/冲突）
    sed -i "s#const API_BASE = .*#const API_BASE = '${VITE_API_URL}/api';#g" index.html
else
    echo "⚠️  No VITE_API_URL set, using localhost"
    sed -i "s#const API_BASE = .*#const API_BASE = 'http://localhost:10000/api';#g" index.html
fi

echo "✅ Build complete!"
