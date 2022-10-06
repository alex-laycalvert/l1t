// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "l1t.h"
#include "colors.h"
#include "utils.h"
#include <stdlib.h>
#include <ncurses.h>
#include <stdbool.h>

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
    bool keep_playing = true;
    bool won = false;
    do {
        init_level(level);
        won = play();
        /* TODO */
        // Display a menu to determine whether to keep playing
        // or move onto the next level.
        keep_playing = false;
    } while (keep_playing);

    endwin();
    if (won) {
        printf("YOU WON! 😄\n");
    } else {
        printf("Sorry, you didn't win. 😥\n");
    }
    exit(EXIT_SUCCESS);
}
