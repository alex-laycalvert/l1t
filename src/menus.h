// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#ifndef MENUS_H_
#define MENUS_H_

#include "config.h"
#include <menu.h>

#define MAIN_MENU_WIDTH 34
#define MAIN_MENU_HEIGHT 22
#define NEXT_LEVEL_MENU_WIDTH 24
#define NEXT_LEVEL_MENU_HEIGHT 7
#define ENTER_KEY '\n'

typedef enum {
    PLAY_OPTION,
    HELP_OPTION,
    QUIT_OPTION,
} MenuOption;

typedef struct {
    char *label;
    MenuOption value;
} MenuDisplayOption;

MenuOption main_menu(const int rows, const int columns, const Configuration config);
MenuOption next_level_menu(const int rows, const int columns, const Configuration config);
void print_logo(WINDOW *window);
void print_in_middle(
    WINDOW *window,
    const int start_x,
    const int start_y,
    const int width, 
    const char *string,
    const chtype color
);

#endif // MENUS_H_
