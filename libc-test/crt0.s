# crt0.s
.section .text
.global _start
.global _init
.global _fini

_start:
# 1. 按照 ABI 规范，清空 rbp
    xor %rbp, %rbp
    
    # 2. 从栈顶获取 argc (此时 RSP 指向的就是你内核压入的第一个值)
    # AT&T 语法: mov (源), (目的)。寄存器要加 %
    mov (%rsp), %rdi

    # 3. 获取 argv 的地址 (即 argc 后面的那个位置)
    # AT&T 语法: 8(%rsp) 表示 [rsp + 8]
    lea 8(%rsp), %rsi

    # 4. 获取 envp 地址 (argv + argc * 8 + 8)
    # 这里我们先让 RDX 等于 RSI，然后加上 argc * 8，再跳过末尾的 NULL
    mov %rsi, %rdx
    mov %rdi, %rax
    shl $3, %rax        # rax = argc * 8
    add %rax, %rdx      # rdx = argv + argc * 8
    add $8, %rdx        # rdx 指向 envp (跨过 argv 的 NULL 结束符)

    # 5. 16 字节栈对齐
    # 注意：$-16 在 AT&T 中通常写为 $0xfffffffffffffff0
    and $-16, %rsp

    # 根据 relibc 编译出来的符号名，可能是这个：
    call relibc_start_v1  
    
    # 如果 main 返回了，挂起
1:  hlt
    jmp 1b
