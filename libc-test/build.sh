
as crt0.s -o crt0.o
clang --target=x86_64-unknown-none -ffreestanding -fno-stack-protector -c main.c -o main.o
ld.lld -nostdlib -static crt0.o main.o ../relibc/target/x86_64-unknown-linux-gnu/release/librelibc.a -o test.elf
