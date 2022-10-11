// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "menus.h"
#include "colors.h"
#include "l1t.h"
#include "config.h"
#include <menu.h>
#include <string.h>
#include <stdlib.h>

MenuOption main_menu(const int rows, const int columns, const Configuration config) {
    const int num_options = 4;
    MenuDisplayOption menu_options[] = {
        { "     Play     ", PLAY_OPTION },
        { "     Help     ", HELP_OPTION },
        { "     Quit     ", QUIT_OPTION },
        { (char *)NULL, QUIT_OPTION }
    };
    ITEM **menu_items = (ITEM **)calloc(num_options, sizeof(ITEM *));
    for(int i = 0; i < num_options; ++i) {
        menu_items[i] = new_item(menu_options[i].label, NULL);
    }

	MENU *main_menu = new_menu((ITEM **)menu_items);
    WINDOW *main_menu_win = newwin(
        MAIN_MENU_HEIGHT,
        MAIN_MENU_WIDTH,
        rows / 2 - MAIN_MENU_HEIGHT / 2,
        columns / 2 - MAIN_MENU_WIDTH / 2
    );
    int menu_row_offset = 5;
    int menu_column_offset = MAIN_MENU_WIDTH / 2 - 10;
    WINDOW *main_menu_sub_win = derwin(
        main_menu_win,
        menu_row_offset, MAIN_MENU_WIDTH - menu_column_offset,
        MAIN_MENU_HEIGHT - menu_row_offset, menu_column_offset
    );
    keypad(main_menu_win, TRUE);
     
    set_menu_win(main_menu, main_menu_win);
    set_menu_sub(main_menu, main_menu_sub_win);
    set_menu_mark(main_menu, " > ");

    int sep_line_offset = 7;
    box(main_menu_win, 0, 0);
	mvwaddch(main_menu_win, MAIN_MENU_HEIGHT - sep_line_offset, 0, ACS_LTEE);
	mvwhline(main_menu_win, MAIN_MENU_HEIGHT - sep_line_offset, 1, ACS_HLINE, MAIN_MENU_WIDTH - 2);
	mvwaddch(main_menu_win, MAIN_MENU_HEIGHT - sep_line_offset, MAIN_MENU_WIDTH - 1, ACS_RTEE);
    print_logo(main_menu_win);
	refresh();
        
	post_menu(main_menu);
	wrefresh(main_menu_win);
    int c;
	while((c = getch()) != ENTER_KEY && c != config.quit_key) {
        if (c == config.move_left_key || c == config.move_up_key || c == KEY_UP || c == KEY_LEFT) {
            menu_driver(main_menu, REQ_UP_ITEM);
        }
        if (c == config.move_right_key || c == config.move_down_key || c == KEY_DOWN || c == KEY_LEFT) {
            menu_driver(main_menu, REQ_DOWN_ITEM);
        }
        wrefresh(main_menu_win);
    }

	/* Unpost and free all the memory taken up */
	endwin();

    ITEM *selected_item = current_item(main_menu);
    if (!selected_item) {
        return QUIT_OPTION;
    }
    int selected_item_index = item_index(selected_item);

    unpost_menu(main_menu);
    free_menu(main_menu);
    for(int i = 0; i < num_options; ++i) {
        free_item(menu_items[i]);
    }
    free(menu_items);

    if (c == QUIT_KEY) {
        return QUIT_OPTION;
    }

    return menu_options[selected_item_index].value;
}

MenuOption next_level_menu(const int rows, const int columns, const Configuration config) {
    const int num_options = 3;
    MenuDisplayOption menu_options[] = {
        { " YES ", PLAY_OPTION },
        { " NO", QUIT_OPTION },
        { (char *)NULL, QUIT_OPTION }
    };
    ITEM **menu_items = (ITEM **)calloc(num_options, sizeof(ITEM *));
    for(int i = 0; i < num_options; ++i) {
        menu_items[i] = new_item(menu_options[i].label, NULL);
    }

	MENU *next_level_menu = new_menu((ITEM **)menu_items);
    WINDOW *next_level_menu_win = newwin(
        NEXT_LEVEL_MENU_HEIGHT,
        NEXT_LEVEL_MENU_WIDTH,
        rows / 2 - NEXT_LEVEL_MENU_HEIGHT,
        columns / 2 - NEXT_LEVEL_MENU_WIDTH / 2
    );
    int menu_column_offset = 7;
    WINDOW *next_level_menu_sub_win = derwin(
        next_level_menu_win,
        2, NEXT_LEVEL_MENU_WIDTH - menu_column_offset,
        4, menu_column_offset
    );
    keypad(next_level_menu_win, TRUE);
     
    set_menu_win(next_level_menu, next_level_menu_win);
    set_menu_sub(next_level_menu, next_level_menu_sub_win);
    set_menu_mark(next_level_menu, "");
    set_menu_format(next_level_menu, 1, 2);

    box(next_level_menu_win, 0, 0);
    mvwprintw(next_level_menu_win, 2, 2, "YOU WON! Next Level?");
	refresh();
        
	post_menu(next_level_menu);
	wrefresh(next_level_menu_win);
    int c;
	while((c = getch()) != ENTER_KEY && c != config.quit_key) {
        if (c == config.move_left_key || c == config.move_up_key || c == KEY_UP || c == KEY_LEFT) {
            menu_driver(next_level_menu, REQ_LEFT_ITEM);
        }
        if (c == config.move_right_key || c == config.move_down_key || c == KEY_DOWN || c == KEY_LEFT) {
            menu_driver(next_level_menu, REQ_RIGHT_ITEM);
        }
        wrefresh(next_level_menu_win);
    }

	/* Unpost and free all the memory taken up */
	endwin();

    ITEM *selected_item = current_item(next_level_menu);
    if (!selected_item) {
        return QUIT_OPTION;
    }
    int selected_item_index = item_index(selected_item);

    unpost_menu(next_level_menu);
    free_menu(next_level_menu);
    for(int i = 0; i < num_options; ++i) {
        free_item(menu_items[i]);
    }
    free(menu_items);

    if (c == QUIT_KEY) {
        return QUIT_OPTION;
    }

    return menu_options[selected_item_index].value;
}

void print_logo(WINDOW *window) {
    int logo_column = MAIN_MENU_WIDTH / 2 - 12;
    int logo_start_row = 3;
    wattron(window, COLOR_PAIR(LASER_BEAM_COLOR_PAIR));
    mvwprintw(window, logo_start_row + 0, logo_column,  "          /-------L       ");
    wattroff(window, COLOR_PAIR(LASER_BEAM_COLOR_PAIR));
    wattron(window, COLOR_PAIR(LOGO_COLOR_PAIR));
    mvwprintw(window, logo_start_row + 1, logo_column,  " ___      |__      _      ");
    mvwprintw(window, logo_start_row + 2, logo_column,  "|_  | S<--/  |    | \\_   ");
    mvwprintw(window, logo_start_row + 3, logo_column,  "  | |     `| |    | __|   ");
    mvwprintw(window, logo_start_row + 4, logo_column,  "  | |      | |    | |     ");
    mvwprintw(window, logo_start_row + 5, logo_column,  "  | |_    _|_|_   | |_    ");
    mvwprintw(window, logo_start_row + 6, logo_column,  "L-\\___\\  |_____| --\\__|");
    wattroff(window, COLOR_PAIR(LOGO_COLOR_PAIR));
    wattron(window, COLOR_PAIR(LASER_BEAM_COLOR_PAIR));
    mvwprintw(window, logo_start_row + 7, logo_column,  "  |                |      ");
    mvwprintw(window, logo_start_row + 8, logo_column,  "  |                v      ");
    mvwprintw(window, logo_start_row + 9, logo_column,  "  v                       ");
    mvwprintw(window, logo_start_row + 10, logo_column, "  S                       ");
    wattroff(window, COLOR_PAIR(LASER_BEAM_COLOR_PAIR));

    mvwprintw(window, logo_start_row, logo_column + 10, "/");
    mvwprintw(window, logo_start_row + 6, logo_column + 2,  "\\");
    mvwprintw(window, logo_start_row + 6, logo_column + 19,  "\\");
    wattron(window, COLOR_PAIR(STATUE_ON_COLOR_PAIR));
    mvwprintw(window, logo_start_row + 10, logo_column + 2, "S");
    mvwprintw(window, logo_start_row + 2, logo_column + 6, "S");
    wattroff(window, COLOR_PAIR(STATUE_ON_COLOR_PAIR));
    wattron(window, COLOR_PAIR(LASER_ON_COLOR_PAIR));
    mvwprintw(window, logo_start_row, logo_column + 18, "L");
    mvwprintw(window, logo_start_row + 6, logo_column,  "L");
    wattroff(window, COLOR_PAIR(LASER_ON_COLOR_PAIR));
    mvwprintw(window, logo_start_row + 2, logo_column + 10,  "/");
    wattron(window, COLOR_PAIR(LASER_BEAM_COLOR_PAIR));
    mvwprintw(window, logo_start_row + 1, logo_column + 10, "|");
    mvwprintw(window, logo_start_row + 2, logo_column + 7, "<--");
    mvwprintw(window, logo_start_row + 6, logo_column + 1,  "-");
    mvwprintw(window, logo_start_row + 6, logo_column + 17,  "--");
    wattroff(window, COLOR_PAIR(LASER_BEAM_COLOR_PAIR));
}

void print_in_middle(
    WINDOW *window,
    const int start_y,
    const int start_x,
    int width, 
    const char *string,
    const chtype color
) {
    int length, x, y;
    float tmp;
    if (!window) {
        window = stdscr;
    }
    getmaxyx(window, y, x);
    if (start_x != 0) {
        x = start_x;
    }
    if (start_y != 0) {
        y = start_y;
    }
    if (width == 0) {
        width = 80;
    }

    length = strlen(string);
    tmp = (width - length) / 2;
    x = start_x + (int)tmp;
    wattron(window, color);
    mvwprintw(window, y, x, "%s", string);
    wattroff(window, color);
    refresh();
}
