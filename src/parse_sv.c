#include "parse_sv.h"

char *trim(char *s) {
    while (isspace((unsigned char)*s))
        s++;

    char *end = s + strlen(s);
    while (end > s && isspace((unsigned char)end[-1]))
        end--;

    *end = '\0';
    return s;
}

char *copy_string(const char *s) {
    size_t len = strlen(s);
    char *copy = malloc(len + 1);

    if (copy)
        memcpy(copy, s, len + 1);

    return copy;
}

int load_config(const char *filename, config_file *config) {
    FILE *file = fopen(filename, "r");
    if (!file)
        return 0;

    enum { SECTION_NONE, SECTION_SAVES, SECTION_BACKUP } section = SECTION_NONE;
    char line[4096];
    int success = 1;

    while (fgets(line, sizeof line, file)) {
        char *text = trim(line);

        if (*text == '\0' || *text == '#' || *text == ';')
            continue;

        if (strcmp(text, "[saves]") == 0) {
            section = SECTION_SAVES;
            continue;
        }

        if (strcmp(text, "[backup]") == 0) {
            section = SECTION_BACKUP;
            continue;
        }

        char *equals = strchr(text, '=');
        if (!equals)
            continue;

        *equals = '\0';
        char *key = trim(text);
        char *value = trim(equals + 1);

        if (section == SECTION_SAVES) {
            char **new_paths = realloc(
                config->s_paths.paths,
                (config->s_paths.count + 1) * sizeof *config->s_paths.paths
            );

            if (!new_paths) {
                success = 0;
                break;
            }

            config->s_paths.paths = new_paths;
            config->s_paths.paths[config->s_paths.count] = copy_string(value);

            if (!config->s_paths.paths[config->s_paths.count]) {
                success = 0;
                break;
            }

            config->s_paths.count++;
        } else if (section == SECTION_BACKUP &&
                   strcmp(key, "bak_path") == 0) {
            free(config->b_paths.bak_path);
            config->b_paths.bak_path = copy_string(value);

            if (!config->b_paths.bak_path) {
                success = 0;
                break;
            }
        }
    }
    fclose(file);
    return success;
}

