// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "colors.h"
#include <stdbool.h>
#include <ncurses.h>

bool initialize_colors() {
    if (!has_colors()) {
        return false;
    }

    start_color();
    use_default_colors();

    /* init_pair(EMPTY_COLOR_PAIR, COLOR_WHITE, COLOR_WHITE); */
    init_pair(PLAYER_COLOR_PAIR, COLOR_GREEN, COLOR_GREEN);
    init_pair(WALL_COLOR_PAIR, COLOR_WHITE, COLOR_WHITE);
    init_pair(BLOCK_COLOR_PAIR, COLOR_BLACK, COLOR_BLACK);
    init_pair(STATUE_COLOR_PAIR, COLOR_YELLOW, COLOR_YELLOW);
    init_pair(TOGGLE_BLOCK_COLOR_PAIR, COLOR_BLUE, COLOR_BLUE);
    /* init_pair(BUTTON_COLOR_PAIR, COLOR_RED, COLOR_RED); */
    /* init_pair(SWITCH_COLOR_PAIR, */
    init_pair(LASER_COLOR_PAIR, COLOR_RED, COLOR_RED);

    return true;
}
