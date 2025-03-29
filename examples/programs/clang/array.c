#include <stdio.h>

int main() {
    for (int i=0; i<100; i++) {
        printf("[");
        for (int j=0; j<100; j++) {
            printf("%d", j); 
            if (j != 99) {
                printf(" "); 
            }
        }
        printf("]\n");
    }
    return 0;
}

