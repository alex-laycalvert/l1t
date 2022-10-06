// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "utils.h"
#include <stdlib.h>
#include <ncurses.h>
#include <string.h>

void err_exit(const char *message) {
    if (stdscr != NULL) {
        endwin();
    }
    fprintf(stderr, "Error: %s\n", message);
    exit(EXIT_FAILURE);
}
