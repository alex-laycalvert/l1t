// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "l1t.h"
#include "levels.h"
#include <stdlib.h>
#include <ncurses.h>

int main(int argc, char **argv) {
    initscr();
    noecho();
    raw();
    keypad(stdscr, true);
    curs_set(0);
    int rows, columns;
    getmaxyx(stdscr, rows, columns);

    int level = 1;
    do {
        init_level(level, rows, columns);
    } while (play());

}
