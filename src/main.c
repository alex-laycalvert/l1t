// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "l1t.h"
#include "colors.h"
#include "utils.h"
#include "menus.h"
#include "config.h"
#include <stdlib.h>
#include <ncurses.h>
#include <stdbool.h>

int main(int argc, char **argv) {
    (void) argc, (void) argv;

    char *home_dir = getenv("HOME");
    char config_path[LINE_BUFFER_SIZE / 2] = { 0 };
    char levels_path[LINE_BUFFER_SIZE / 2] = { 0 };
    sprintf(config_path, "%s%s", home_dir, L1T_CONFIG_FILE);
    sprintf(levels_path, "%s%s", home_dir, L1T_LEVELS_DIR);
    Configuration config = read_configuration(config_path);
    init_config(config);
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

    MenuOption menu_selection = main_menu(terminal_rows, terminal_columns, config);

    if (menu_selection == QUIT_OPTION) {
        endwin();
        exit(EXIT_SUCCESS);
    }

    if (menu_selection == HELP_OPTION) {
        /* TODO */
    }

    int level = 1;
    char current_level[LINE_BUFFER_SIZE];
    char current_level_name[LINE_BUFFER_SIZE];
    bool keep_playing = true;
    bool won = false;
    do {
        if (level > MAX_LEVELS) {
            break;
        }
        sprintf(current_level, "%s%d.l1t", levels_path, level);
        sprintf(current_level_name, "Level: %d", level);
        init_level(current_level, current_level_name, terminal_rows, terminal_columns);
        won = play();
        if (!won) {
            keep_playing = false;
        } else {
            menu_selection = next_level_menu(terminal_rows, terminal_columns, config);
            if (menu_selection == QUIT_OPTION) {
                keep_playing = false;
            } else {
                level++;
            }
        }
    } while (keep_playing);
    endwin();
    if (level > MAX_LEVELS) {
        printf("You've completed all of the levels, stay tuned for more.\n");
    }
    exit(EXIT_SUCCESS);
}
