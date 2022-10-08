// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "config.h"
#include "utils.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

Configuration default_config = { 'k', 'j', 'h', 'l', ' ', 'r', 'q' };

Configuration read_configuration(char *filename) {
    Configuration config = default_config;
    FILE *config_file = fopen(filename, "r");
    if (!config_file) {
        config_file = fopen(filename, "w");
        fclose(config_file);
        return config;
    }
    char line_buf[LINE_BUFFER_SIZE] = { 0 };
    char *key, *value;
    while (fgets(line_buf, LINE_BUFFER_SIZE, config_file) != NULL) {
        key = strtok(line_buf, "= ");
        if (key[0] == '#') {
            continue;
        }
        value = strtok(NULL, "=# ");
        if (!key || !value) {
            err_exit("failed to parse configuration file");
        }
        if (strncmp(key, MOVE_UP_KEY_CONFIG, strlen(MOVE_UP_KEY_CONFIG)) == 0) {
            config.move_up_key = value[0];
        }
        if (strncmp(key, MOVE_DOWN_KEY_CONFIG, strlen(MOVE_DOWN_KEY_CONFIG)) == 0) {
            config.move_down_key = value[0];
        }
        if (strncmp(key, MOVE_LEFT_KEY_CONFIG, strlen(MOVE_LEFT_KEY_CONFIG)) == 0) {
            config.move_left_key = value[0];
        }
        if (strncmp(key, MOVE_RIGHT_KEY_CONFIG, strlen(MOVE_RIGHT_KEY_CONFIG)) == 0) {
            config.move_right_key = value[0];
        }
        if (strncmp(key, INTERACTION_KEY_CONFIG, strlen(INTERACTION_KEY_CONFIG)) == 0) {
            config.interaction_key = value[0];
        }
        if (strncmp(key, RESTART_KEY_CONFIG, strlen(RESTART_KEY_CONFIG)) == 0) {
            config.restart_key = value[0];
        }
        if (strncmp(key, QUIT_KEY_CONFIG, strlen(QUIT_KEY_CONFIG)) == 0) {
            config.quit_key = value[0];
        }
    }
    return config;
}
