// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#ifndef COLORS_H_
#define COLORS_H_

#include <stdbool.h>

typedef enum L1T_CUSTOM_COLORS {
    COLOR_DIMMED_YELLOW = 100,
    COLOR_DIMMED_MAGENTA,
    COLOR_DIMMED_RED,
} L1tCusomColors;

typedef enum L1T_COLOR_PAIRS {
    PLAYER_COLOR_PAIR = 100, 
    WALL_COLOR_PAIR, 
    BLOCK_COLOR_PAIR,
    STATUE_ON_COLOR_PAIR,
    STATUE_OFF_COLOR_PAIR,
    REVERSE_STATUE_ON_COLOR_PAIR,
    REVERSE_STATUE_OFF_COLOR_PAIR,
    TOGGLE_BLOCK_COLOR_PAIR,
    LASER_ON_COLOR_PAIR,
    LASER_OFF_COLOR_PAIR,
    LASER_BEAM_COLOR_PAIR,
    LOGO_COLOR_PAIR,
    KILL_COLOR_PAIR
} L1tColorPairs;

bool initialize_colors();

#endif // COLORS_H_
