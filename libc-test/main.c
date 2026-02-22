//extern int write(int fd, const void *buf, unsigned long count);
#include <stdint.h>
// 声明 relibc 真正的初始化逻辑入口
extern void relibc_crt0(uintptr_t* sp);

// 使用 naked 属性，防止 GCC 自动生成 push %rbp 等导致栈进一步乱掉的指令
__attribute__((naked)) void _start() {
    __asm__ volatile (
        "xor %rbp, %rbp\n"          // 1. 按照 ABI 规范清空 rbp，标记调用栈结束
        "mov %rsp, %rdi\n"          // 2. 将当前栈指针（包含 argc/argv 等）存入第一个参数寄存器
        
        // --- 核心修复：对齐 ---
        "and $-16, %rsp\n"          // 3. 强行将 RSP 16 字节对齐（把低 4 位抹零）
                                    //    不管你的内核传过来的是 ...fc8 还是 ...fb8
                                    //    这一步之后都会变成 ...fc0 或 ...fb0
        
        "sub $16, %rsp\n"           // 4. 预留一些局部空间，防止 relibc_crt0 内部指令越界
        
        "call relibc_crt0\n"        // 5. 进入真正的 relibc 初始化
        
        "1: hlt\n"                  // 6. 防御性死循环
        "jmp 1b\n"
    );
}

void main() {
//    write(1, "Hello Relibc!\n", 14);
    while(1);
}
