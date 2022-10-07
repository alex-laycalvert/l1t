// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "l1t.h"
#include "colors.h"
#include "utils.h"
#include "menus.h"
#include <stdlib.h>
#include <ncurses.h>
#include <stdbool.h>

int main(int argc, char **argv) {
    (void) argc, (void) argv;

    initscr();
    noecho();
    raw();
    cbreak();
    curs_set(0);
    keypad(stdscr, true);

    if (!initialize_colors()) {
        err_exit("your terminal does not support colors");
    }

    int terminal_rows, terminal_columns;
    getmaxyx(stdscr, terminal_rows, terminal_columns);

    MenuOption menu_selection = main_menu(terminal_rows, terminal_columns);

    if (menu_selection == QUIT_OPTION) {
        endwin();
        exit(EXIT_SUCCESS);
    }

    if (menu_selection == HELP_OPTION) {
        /* TODO */
    }

    int level = 0;
    bool keep_playing = true;
    bool won = false;
    do {
        init_level(level, terminal_rows, terminal_columns);
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
