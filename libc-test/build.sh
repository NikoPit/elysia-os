#!/bin/sh

# 1. 配置路径 (根据你的实际存放位置修改)
RELIBC_PATH="../relibc/target/x86_64-unknown-linux-gnu/release"
RELIBC_INCLUDE="../relibc/include"
RELIBC_LIB="$RELIBC_PATH"

# 2. 编译参数
# --target: 指定目标为 x86_64-unknown-redox
# -ffreestanding: 不使用宿主机的标准库
TARGET="x86_64-unknown-none"

# 3. 编译源文件为对象文件
clang --target=$TARGET \
      -ffreestanding \
      -fno-stack-protector \
      -I$RELIBC_INCLUDE \
      -c main.c -o main.o

# 4. 链接阶段 (核心步骤)
# -static: 静态链接
# -nostdlib: 禁用所有默认库，我们要手动指定
# 顺序极其重要: crt0.o 必须在最前面
ld.lld -nostdlib -static \
       ./crt0.o \
       main.o \
       $RELIBC_LIB/librelibc.a \
       -o test.elf
