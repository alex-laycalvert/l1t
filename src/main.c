// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "l1t.h"
#include "utils.h"
#include <stdlib.h>
#include <ncurses.h>

int main(int argc, char **argv) {
    (void) argc, (void) argv;

    initscr();
    noecho();
    raw();
    keypad(stdscr, true);
    curs_set(0);

    int rows, columns;
    resizeterm(TERMINAL_ROWS, TERMINAL_COLUMNS);
    getmaxyx(stdscr, rows, columns);

    if (!has_colors()) {
        endwin();
        err_exit("your terminal does not support colors");
    }

    start_color();
    use_default_colors();

    int level = 0;
    do {
        init_level(level, rows, columns);
    } while (play());
}
