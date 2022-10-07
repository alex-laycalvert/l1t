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

    init_color(COLOR_DIMMED_YELLOW, 250, 250, 0);
    init_color(COLOR_DIMMED_MAGENTA, 179, 179, 24);
    init_color(COLOR_DIMMED_RED, 250, 0, 0);

    init_pair(PLAYER_COLOR_PAIR, COLOR_GREEN, COLOR_GREEN);
    init_pair(WALL_COLOR_PAIR, COLOR_WHITE, COLOR_WHITE);
    init_pair(BLOCK_COLOR_PAIR, COLOR_BLACK, COLOR_BLACK);
    init_pair(STATUE_ON_COLOR_PAIR, COLOR_YELLOW, COLOR_YELLOW);
    init_pair(STATUE_OFF_COLOR_PAIR, COLOR_DIMMED_YELLOW, COLOR_DIMMED_YELLOW);
    init_pair(REVERSE_STATUE_ON_COLOR_PAIR, COLOR_MAGENTA, COLOR_MAGENTA);
    init_pair(REVERSE_STATUE_OFF_COLOR_PAIR, COLOR_DIMMED_MAGENTA, COLOR_DIMMED_MAGENTA);
    init_pair(TOGGLE_BLOCK_COLOR_PAIR, COLOR_BLUE, COLOR_BLUE);
    init_pair(LASER_ON_COLOR_PAIR, COLOR_RED, COLOR_RED);
    init_pair(LASER_OFF_COLOR_PAIR, COLOR_DIMMED_RED, COLOR_DIMMED_RED);
    init_pair(LASER_BEAM_COLOR_PAIR, COLOR_RED, -1);
    init_pair(LOGO_COLOR_PAIR, COLOR_GREEN, -1);

    return true;
}
