#ifndef PARSE_SV_F
#define PARSE_SV_F

//[saves]
//pcsx2=/home/fanty/.config/PCSX2/memcards/
//ppsspp=/home/fanty/.config/ppsspp/PSP/SAVEDATA/

//[backup]
//bak_path=/mnt/hd_ext4/backup/saves/

typedef struct{
    char *pcsx2;
    char *ppsspp;
}saves_path;
typedef struct{
    char *f_back;
    char *s_back;
}bak_path;

typedef struct{
    saves_path s_paths;
    bak_path b_paths;
}config_file;

char* parse_to_saves(char *content_file);
char* parse_to_bak(char *content_file);
saves_path get_saves_paths(char *content);
bak_path get_backup_paths(char *content);

#endif//PARSE_SV_F
