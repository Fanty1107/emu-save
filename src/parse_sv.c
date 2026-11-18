#include "parse_sv.h"
#include "fylib.h"
#include <stdlib.h>

//typedef struct{
//    char *pcsx2;
//    char *ppsspp;
//}saves_path;
//typedef struct{
//    char *f_back;
//    char *s_back;
//}bak_path;

//typedef struct{
//    saves_path s_paths;
//    bak_path b_paths;
//}config_file;

//content here is the file

//[saves]
//pcsx2=/home/fanty/.config/PCSX2/memcards/
//ppsspp=/home/fanty/.config/ppsspp/PSP/SAVEDATA/

//[backup]
//bak_path=/mnt/hd_ext4/backup/saves/
char* parse_to_saves(char *content_file){
    size_t ch_count = 0;
    char *cont_saves = malloc(sizeof(char) * KiB(1));
    printf("%zu",strlen(content_file));
    while(ch_count < strlen(content_file)){

    }
    return cont_saves;
}

saves_path get_saves_paths(char *content){
    
}
bak_path get_backup_paths(char *content){


}

