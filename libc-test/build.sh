#!/bin/sh

# 1. 路径配置
RELIBC_PATH="../relibc/target/x86_64-unknown-linux-gnu/release"
RELIBC_INCLUDE="../relibc/include"

# 2. 编译器设置 (Nix pkgsCross 提供的名称)
CC="x86_64-elf-gcc"
LD="x86_64-elf-ld"

# 3. 编译参数
# -mno-sse: 禁用 SSE，防止生成 ldmxcsr 等对齐敏感指令
# -mno-red-zone: 禁止使用栈下方 128 字节，这对内核/底层开发是安全的
# -fno-stack-protector: 禁用栈保护，避免链接到不存在的符号
CFLAGS="-ffreestanding \
        -mno-sse \
        -mno-red-zone \
        -fno-stack-protector \
        -fno-builtin \
        -nostdinc \
        -I$RELIBC_INCLUDE"

echo "Compiling main.c..."
$CC $CFLAGS -c main.c -o main.o

echo "Linking test.elf..."
# 注意：relibc 自带的 crt0 可能会调用 SSE 指令，
# 如果链接后仍然崩在 _start，说明 crt0 需要重新用 -mno-sse 编译
$LD -static -nostdlib \
    --allow-multiple-definition \
    main.o \
    $RELIBC_PATH/crt0.o \
    $RELIBC_PATH/crti.o \
    $RELIBC_PATH/librelibc.a \
    $RELIBC_PATH/crtn.o \
    -o test.elf

echo "Build Done. Entry Point:"
readelf -h test.elf | grep Entry
