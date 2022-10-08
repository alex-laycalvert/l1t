// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#ifndef LEVELS_H_
#define LEVELS_H_

#include "node.h"
#include <stdbool.h>
#include <stdlib.h>

#define MAX_LEVEL_ROWS 200
#define MAX_LEVEL_COLUMNS 500

typedef struct level_info {
    int rows;
    int columns;
    Node *player;
    int num_statues;
    Node **statues;
    int num_reverse_statues;
    Node **reverse_statues;
    Node **grid;
} LevelInfo;

void copy_level_files(char *source_path, char *target_path);
LevelInfo generate_level(const char *name);

#endif // LEVELS_H_

