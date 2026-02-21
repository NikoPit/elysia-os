extern int write(int fd, const void *buf, unsigned long count);

void main() {
    write(1, "Hello Relibc!\n", 14);
    while(1);
}
