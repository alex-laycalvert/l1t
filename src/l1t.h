// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#ifndef L1T_H_
#define L1T_H_

#include "node.h"
#include <stdbool.h>

#define TERMINAL_ROW_OFFSET 0
#define TERMINAL_COLUMN_OFFSET 0

#define QUIT_KEY 'q'
#define MOVE_UP_KEY 'k'
#define MOVE_DOWN_KEY 'j'
#define MOVE_LEFT_KEY 'h'
#define MOVE_RIGHT_KEY 'l'

void init_level(const int level);
void print_grid();
void clear_grid();
void destroy_grid();
void move_player(Direction dir);
void print_laser(const int row, const int column, const Direction dir);
bool play();

#endif // L1T_H_
