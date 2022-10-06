// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#ifndef COLORS_H_
#define COLORS_H_

#include <stdbool.h>

typedef enum L1T_COLOR_PAIRS {
    EMPTY_COLOR_PAIR, 
    PLAYER_COLOR_PAIR, 
    WALL_COLOR_PAIR, 
    /* MIRROR_FORWARD_COLOR_PAIR, */
    /* MIRROR_BACKWARD_COLOR_PAIR, */
    BLOCK_COLOR_PAIR,
    STATUE_COLOR_PAIR,
    TOGGLE_BLOCK_COLOR_PAIR,
    /* BUTTON_COLOR_PAIR, */
    /* SWITCH_COLOR_PAIR, */
    LASER_COLOR_PAIR,
} L1tColorPairs;

bool initialize_colors();

#endif // COLORS_H_
