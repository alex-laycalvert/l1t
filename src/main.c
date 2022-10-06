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

    if (!has_colors()) {
        endwin();
        err_exit("your terminal does not support colors");
    }

    start_color();
    use_default_colors();

    int level = 0;
    do {
        init_level(level);
    } while (play());

    endwin();
    exit(EXIT_SUCCESS);
}
