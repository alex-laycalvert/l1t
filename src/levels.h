// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#ifndef LEVELS_H_
#define LEVELS_H_

#include "node.h"
#include <stdbool.h>
#include <stdlib.h>

#define MAX_LEVEL_ROWS 70
#define MAX_LEVEL_COLUMNS 180

typedef struct level_info {
    int rows;
    int columns;
    Node *player;
    int num_statues;
    Node **statues;
    Node **grid;
} LevelInfo;

LevelInfo generate_level(const char *name);

#endif // LEVELS_H_

