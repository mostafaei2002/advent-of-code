#include <stdio.h>
#include <stdbool.h>

#define CONTENT_LEN 100000
#define POINTER_LEN 2000


int find_match(char* cur_line) {
    // printf("%s", cur_line);

    char first_match = '0';
    char last_match = '0';

    int i = 0;
    char target;
    while ((target = cur_line[i]) != '\n' && (cur_line[i] != '\0')) {
        if (target == '1' || target == '2' || target == '3' || target== '4' || target == '5' || target == '6' || target == '7' || target =='8' || target == '9') {
            last_match = target;
        }

        if (first_match == '0') {
            first_match = last_match;
        }
        i++;
    }

    int res = (first_match - '0') * 10 + (last_match - '0');
    return res;
}

void main () {
    FILE* fptr;
    char c;

    fptr = fopen("../question",  "r");

    if (fptr == NULL) {
        printf("File cannot be opened\n");
    } else {
        printf("File opened successfully!\n");
    }

    char* lines[POINTER_LEN];

    char content[CONTENT_LEN];
    bool first = true;

    int i = 0;
    int j = 0;

    while (!feof(fptr)) {
        c = fgetc(fptr);
        content[i] = c;
        // printf("%c`" ,c);

        if (first == true) {
            lines[j] = &content[i];
            j++;
            first = false;

            if (j > POINTER_LEN) {
                printf("POINTER OVERFLOW\n");
            }
        }
        if (c == '\n') {
            first = true;
        }

        i++;
        if (i > CONTENT_LEN) {
            printf("CONTENT OVERFLOW \n");
        }
    }
    printf("%d characters read\n", i);
    printf("%d lines read\n", j);
    printf("\n");
    printf("File was read successfully\n");


    int sum = 0;
    for (int i = 0; i < sizeof(lines) / sizeof(lines[0]); i++) {
        char* cur_line = lines[i];
        if (cur_line == NULL) {
            continue;
        }

        int cur_match = find_match(cur_line);
        printf("%d\n", cur_match);
        sum += cur_match;
    }

    printf("First answer: %d\n", sum);
}

