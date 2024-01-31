#include <stdio.h>
#include <stdbool.h>
#include <string.h>

#define CONTENT_LEN 100000
#define POINTER_LEN 2000

void get_string(char cur_line[],char subString[], int start, int end){
    
    int i = 0;
    while (start < end) {
        subString[i] = cur_line[start];

        i++;
        start++;
    }
 
    subString[i] = '\0';
 
}


int find_match(char* cur_line) {
    // printf("%s", cur_line);

    char first_match = '0';
    char last_match = '0';

    int i = 0;
    char cur_char;

    while ((cur_char = cur_line[i]) != '\n' && (cur_line[i] != '\0')) {
        int j = i + 6;

        while (j > i) {
            char sub_string[10];

            get_string(cur_line ,sub_string, i, j);
            // printf("%s" ,sub_string);
            if (strcmp(sub_string, "1\0") == 0) {
                last_match = '1';
            } else if (strcmp (sub_string, "2\0") == 0) {
                last_match = '2';
            } else if (strcmp(sub_string, "3\0") == 0) {
                last_match = '3';
            } else if (strcmp(sub_string, "4\0") == 0) {
                last_match = '4';
            } else if (strcmp(sub_string, "5\0") == 0) {
                last_match = '5';
            } else if (strcmp(sub_string, "6\0") == 0) {
                last_match = '6';
            } else if (strcmp(sub_string, "7\0") == 0) {
                last_match = '7';
            } else if (strcmp(sub_string, "8\0") == 0) {
                last_match = '8';
            } else if (strcmp(sub_string, "9\0") == 0) {
                last_match = '9';
            } else if (strcmp(sub_string, "one\0") == 0) {
                last_match = '1';
            } else if (strcmp(sub_string, "two\0") == 0) {
                last_match = '2';
            } else if (strcmp(sub_string, "three\0") == 0) {
                last_match = '3';
            } else if (strcmp(sub_string, "four\0") == 0) {
                last_match = '4';
            } else if (strcmp(sub_string, "five\0") == 0) {
                last_match = '5';
            } else if (strcmp(sub_string, "six\0") == 0) {
                last_match = '6';
            } else if (strcmp(sub_string, "seven\0") == 0) {
                last_match = '7';
            } else if (strcmp(sub_string, "eight\0") == 0) {
                last_match = '8';
            } else if (strcmp(sub_string, "nine\0") == 0) {
                last_match = '9';
            }

            if (first_match == '0') {
                first_match = last_match;
            }
            
            j--;
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
        // printf("%d\n", cur_match);
        sum += cur_match;
    }

    printf("First answer: %d\n", sum);
}