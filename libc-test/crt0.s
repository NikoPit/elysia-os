# crt0.s
.section .text
.global _start
.global _init
.global _fini

_start:
# 1. 按照 ABI 规范，清空 rbp
    xor %rbp, %rbp
    
    # 2. 此时 rsp 已经由内核设置好了，指向 argc
    # 我们不需要手动 push 东西，直接跳转到 relibc 的入口
    # relibc 会从当前 rsp 开始读取 argc, argv, envp
    
    # 根据 relibc 编译出来的符号名，可能是这个：
    call relibc_start_v1  
    
    # 如果 main 返回了，挂起
1:  hlt
    jmp 1b

_init:
    ret

_fini:
    ret
