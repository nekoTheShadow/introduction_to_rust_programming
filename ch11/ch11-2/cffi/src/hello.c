#include<stdio.h>
void c_hello() {
    printf("Hello, World from C!\n");
}

typedef unsigned int uint;
uint c_fib(uint n) {
    if (n < 2) {
        return 1;
    } else {
        return c_fib(n-1) + c_fib(n-2);
    }
}