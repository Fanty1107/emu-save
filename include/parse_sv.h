#ifndef PARSE_SV_H
#define PARSE_SV_H
//[saves]
//pcsx2=/home/fanty/.config/PCSX2/memcards/
//ppsspp=/home/fanty/.config/ppsspp/PSP/SAVEDATA/

//[backup]
//bak_path=/mnt/hd_ext4/backup/saves/

#include <stddef.h>
#include <stdio.h>
#include <string.h>
#include <ctype.h>
#include <stdlib.h>
typedef struct {
    char **paths;
    size_t count;
} saves_path;

typedef struct {
    char *bak_path;
} backup_path;

typedef struct {
    saves_path s_paths;
    backup_path b_paths;
} config_file;

char *trim(char *s);
char *copy_string(const char *s);
int load_config(const char *filename, config_file *config);
#endif//PARSE_SV_F
