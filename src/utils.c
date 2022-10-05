// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "utils.h"
#include <stdlib.h>
#include <ncurses.h>

void err_exit(const char *message) {
    fprintf(stderr, "Error: %s\n", message);
    if (stdscr != NULL) {
        endwin();
    }
    exit(EXIT_FAILURE);
}
