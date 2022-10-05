// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#ifndef L1T_H_
#define L1T_H_

#include "node.h"
#include <stdbool.h>

#define QUIT_KEY 'q'
#define MOVE_UP_KEY 'k'
#define MOVE_DOWN_KEY 'j'
#define MOVE_LEFT_KEY 'h'
#define MOVE_RIGHT_KEY 'l'

#define EMPTY_CH ' '
#define PLAYER_CH 'X'
#define WALL_CH 'I'
#define MIRROR_FORWARD_CH '/'
#define MIRROR_BACKWARD_CH '\\'
#define BLOCK_CH 'K'
#define STATUE_CH 'S'
#define TOGGLE_BLOCK_CH 'T'
#define BUTTON_CH 'B'
#define SWITCH_CH 'W'
#define LASER_CH 'L'

void init_grid(const int terminal_rows, const int terminal_columns);
void init_level(const int level, const int terminal_rows, const int terminal_columns);
void init_walls();
void print_grid();
void clear_grid();
void destroy_grid();
void move_player(Direction dir);
bool play();

#endif // L1T_H_
