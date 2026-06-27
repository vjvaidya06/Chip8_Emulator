#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>
#include <arpa/inet.h>

static FILE *fp = NULL;

void cleanup(){
    if (fp != NULL){
        fclose(fp);
    }
}

int main(int argc, char **argv){
    if (argc < 2){
        printf("Not enough arguments\n");
        return 1;
    }
    atexit(cleanup);
    printf("Opening %s\n", argv[1]);
    fp = fopen(argv[1], "wb");
    if (fp == NULL){
        //printf("Error creating file. Does it already exist?\n");
        printf("Error opening file\n");
        return 1;
    }

    char hex[6];
    while (1){
        printf("Type a Chip8 Instruction, or quit to exit\n");
        if (fgets(hex, 6, stdin) == NULL){
            printf("Fgets fail\n");
            exit(1);
        }
        hex[4] = '\0';
        if (strncmp(hex, "quit", 4) == 0){
            printf("Writing file\n");
            break;
        }
        char *endptr;
        unsigned int hexint = strtoul(hex, &endptr, 16);
        //Confirm it's in range, cast to uint8_t, write
        if (endptr == hex){
            printf("Invalid hex: %s\n", hex);
        }
        else if (hexint > 0xFFFF){
            printf("Too big, Chip 8 Instructions must be 16 bit\n");
            return 1;
        }
        else{
            printf("Int: %d\n", hexint);
            uint16_t hexint8a = htons((uint16_t)hexint);
            fwrite(&hexint8a, sizeof(uint16_t), 1, fp);
        }
    }
    return 0;
}
