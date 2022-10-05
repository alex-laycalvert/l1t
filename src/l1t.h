// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#ifndef L1T_H_
#define L1T_H_

#define TERMINAL_ROWS 40
#define TERMINAL_COLUMNS 150
#define TERMINAL_ROW_OFFSET 0
#define TERMINAL_COLUMN_OFFSET 0

#include "node.h"
#include <stdbool.h>

#define QUIT_KEY 'q'
#define MOVE_UP_KEY 'k'
#define MOVE_DOWN_KEY 'j'
#define MOVE_LEFT_KEY 'h'
#define MOVE_RIGHT_KEY 'l'

void init_grid(const int terminal_rows, const int terminal_columns);
void init_level(const int level, const int terminal_rows, const int terminal_columns);
void print_grid();
void clear_grid();
void destroy_grid();
void move_player(Direction dir);
void print_laser(const int row, const int column, const Direction dir);
bool play();

#endif // L1T_H_
