// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#ifndef LEVELS_H_
#define LEVELS_H_

#include "node.h"
#include <stdbool.h>

Node** read_level(const char *name);
void init_walls(const int rows, const int columns, Node **grid);
void place_item(NodeType item, Direction dir, bool on, const int row, const int column, Node **grid);
void init_level_000(const int rows, const int columns, Node **grid);
void init_level_001(const int rows, const int columns, Node **grid);

#endif // LEVELS_H_

