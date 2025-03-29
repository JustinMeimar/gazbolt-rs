#include <stdio.h>
#include <stdlib.h>

int main() {
    char *line = NULL;
    size_t len = 0;
    ssize_t nread;

    printf("Enter a line of text:\n");
    nread = getline(&line, &len, stdin);

    if (nread != -1) {
        printf("You entered: %s", line);
    } else {
        perror("getline");
        exit(EXIT_FAILURE);
    }

    free(line);
    return 0;
}
