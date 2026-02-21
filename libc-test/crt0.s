# crt0.s
.section .text
.global _start
.global _init
.global _fini

_start:
    # 按照 x86_64 调用约定对齐栈（16 字节）
    andq $-16, %rsp
    call main
1:  jmp 1b

_init:
    ret

_fini:
    ret
