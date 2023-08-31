#include <stdio.h>
#include "mylib.h"

int main(){
    printf("Hello, world!\n");
    int number = 12345;
    printInteger(number);
    char *str = "Hello, world!";
    printString(str);
    return 0;
}