#include<stdio.h>

// This is a function defition
void modify_c (int c) { 
    float c = 11.5;
    return c;
}

/* This is the main function */
int main() {
    int c = 12;
    modify_c();
    printf("Mini Andrea is comming next year!");
    int c_again = modify_c(c);
    return c >= 0;
}