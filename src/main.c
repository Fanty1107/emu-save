#include "parse_sv.h"
#define FYLIB_ARENA_IMPLEMENTATION
#define FYLIB_STR_IMPLEMENTATION
#include "fylib.h"

int main(){
    mem_arena *arena_ini = arena_create(KiB(2)); 
    char *content = arena_push(arena_ini, KiB(2), false);
    READ_FILE("config/conf.ini", &content);
    if(!content){
        fprintf(stderr, "unable to read file");
        return 1;
    }
    char *saves_content = parse_to_saves(content); 
    char *bak_content = parse_to_bak(content);
    printf("%s", saves_content);
    printf("%s", bak_content);
    arena_destroy(arena_ini);
    return 0;
}
