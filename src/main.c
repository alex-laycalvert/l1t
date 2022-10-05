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

    init_level(0, rows, columns);

    while (true) {
        print_grid();
        char input = getch();
        switch (input) {
            case QUIT_KEY:
                destroy_grid();
                endwin();
                exit(0);
                break;
            default:
                break;
        }
    }
}
