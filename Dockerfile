# 使用 rust:alpine 作为构建镜像
FROM rust:alpine AS builder

# 设置工作目录
WORKDIR /workspace

# 安装必要的系统依赖
RUN apk add --no-cache musl-dev gcc libc-dev make

# 复制项目文件
COPY . .

# 构建项目
RUN cargo install --path .

# 使用 alpine 作为运行时镜像
FROM alpine

# 设置工作目录
WORKDIR /app

# 从构建阶段复制编译好的二进制文件
COPY --from=builder /usr/local/cargo/bin/pixel-pulse .

# 创建非 root 用户
RUN adduser -D app_user

# 复制 webp 图片
COPY --from=builder /workspace/images/webp /app/images/webp

# 设置文件所有者
RUN chown -R app_user:app_user /app

# 切换到非 root 用户
USER app_user

# 暴露端口
EXPOSE 8544

# 运行程序
CMD ["./pixel-pulse"]