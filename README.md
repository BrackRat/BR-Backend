# [Rust] Actix-Web + JWT + Prisma 后端模板

## 部署

1. 生成 `prisma.rs`
```bash
cargo prisma generate
cargo prisma db push
```

2. 编译后端
```bash
cargo build --release
```