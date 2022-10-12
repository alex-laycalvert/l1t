// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#ifndef L1T_H_
#define L1T_H_

#include "node.h"
#include "config.h"
#include <stdbool.h>

#define L1T_CONFIG_PATH "/.config/l1t/"
#define L1T_LEVELS_DIR "/.config/l1t/levels/"
#define L1T_CONFIG_FILE "/.config/l1t/l1t.conf"
#define MAX_LEVELS 4

#define QUIT_KEY 'q'
#define RESTART_KEY 'r'
#define INTERACT_KEY ' '
#define MOVE_UP_KEY 'k'
#define MOVE_DOWN_KEY 'j'
#define MOVE_LEFT_KEY 'h'
#define MOVE_RIGHT_KEY 'l'

void init_config(const Configuration config);
void init_level(const char *level_file, const char *level_info, const int terminal_rows, const int terminal_columns);
void print_border();
void print_grid();
void print_lasers();
void print_laser(const int row, const int column, const Direction dir);
void clear_grid();
void restart_level();
void destroy_level();
void reset_statues();
void move_player(Direction dir);
void perform_player_interaction();
bool check_win();
bool play();

#endif // L1T_H_
