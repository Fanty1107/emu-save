#include "parse_sv.h"
#include <dirent.h>
#include <stdio.h>

void free_config(config_file *config){
    for (size_t i = 0; i < config->s_paths.count; i++)
        free(config->s_paths.paths[i]);

    free(config->s_paths.paths);
    free(config->b_paths.bak_path);
}

int main(){
    config_file config = {0};
    if(!load_config("config/conf.ini", &config)){
        fprintf(stderr, "Error loading config");
    }
    struct dirent *de;
    DIR *dr; 
    for (size_t i = 0; i < config.s_paths.count; i++){
    dr = opendir(config.s_paths.paths[i]);
        while((de = readdir(dr)) != NULL){
            printf("%s\n", de->d_name);
        }
    }
    closedir(dr);
    free_config(&config);
    return 0;
}
