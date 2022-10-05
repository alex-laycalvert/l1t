// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#ifndef LEVELS_H_
#define LEVELS_H_

#include "l1t.h"

void init_grid(const int terminal_rows, const int terminal_columns);
void init_level(const int level, const int terminal_rows, const int terminal_columns);
void init_walls();
void print_grid();
void clear_grid();
void destroy_grid();

#endif // LEVELS_H_

