//extern int write(int fd, const void *buf, unsigned long count);
#include <stdint.h>
#include <stdio.h>
// 声明 relibc 真正的初始化逻辑入口
extern long write(int fd, const void *buf, unsigned long count);
extern void relibc_crt0(uintptr_t* sp);

int main() {
	printf("Hello world from Relibc printf LETSGOOOOOOOOO!!!!!!");
    return 0;
}
