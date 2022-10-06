// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "l1t.h"
#include "colors.h"
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

    if (!initialize_colors()) {
        err_exit("your terminal does not support colors");
    }

    int level = 0;
    do {
        init_level(level);
    } while (play());

    endwin();
    exit(EXIT_SUCCESS);
}
