#include <stdio.h>
#include <unistd.h>

int main() {
    char buf[64];

    while (1) {
        ssize_t n = read(0, buf, sizeof(buf) - 1);
        if (n > 0) {
            buf[n] = '\0';
            printf("userspace key: %s\n", buf);
        }
    }
    return 0;
}
