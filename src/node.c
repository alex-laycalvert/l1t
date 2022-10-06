// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "node.h"
#include "l1t.h"
#include "colors.h"
#include <ncurses.h>

void print_node(const int row, const int column, const Node *node) {
    attron(A_BOLD);
    switch (node->type) {
        case EMPTY:
            mvprintw(
                row + TERMINAL_ROW_OFFSET,
                column + TERMINAL_COLUMN_OFFSET,
                "%c", node->ch
            );
            break;
        case PLAYER:
            attron(COLOR_PAIR(PLAYER_COLOR_PAIR));
            mvprintw(
                row + TERMINAL_ROW_OFFSET,
                column + TERMINAL_COLUMN_OFFSET,
                "%c", node->ch
            );
            attroff(COLOR_PAIR(PLAYER_COLOR_PAIR));
            break;
        case WALL:
            attron(COLOR_PAIR(WALL_COLOR_PAIR));
            mvprintw(
                row + TERMINAL_ROW_OFFSET,
                column + TERMINAL_COLUMN_OFFSET,
                "%c", node->ch
            );
            attroff(COLOR_PAIR(WALL_COLOR_PAIR));
            break;
        case MIRROR_FORWARD:
            mvprintw(
                row + TERMINAL_ROW_OFFSET,
                column + TERMINAL_COLUMN_OFFSET,
                "%c", node->ch
            );
            break;
        case MIRROR_BACKWARD:
            mvprintw(
                row + TERMINAL_ROW_OFFSET,
                column + TERMINAL_COLUMN_OFFSET,
                "%c", node->ch
            );
            break;
        case BLOCK:
            attron(COLOR_PAIR(BLOCK_COLOR_PAIR));
            mvprintw(
                row + TERMINAL_ROW_OFFSET,
                column + TERMINAL_COLUMN_OFFSET,
                "%c", node->ch
            );
            attroff(COLOR_PAIR(BLOCK_COLOR_PAIR));
            break;
        case STATUE:
            attron(COLOR_PAIR(STATUE_COLOR_PAIR));
            mvprintw(
                row + TERMINAL_ROW_OFFSET,
                column + TERMINAL_COLUMN_OFFSET,
                "%c", node->ch
            );
            attroff(COLOR_PAIR(STATUE_COLOR_PAIR));
            break;
        case TOGGLE_BLOCK:
            attron(COLOR_PAIR(TOGGLE_BLOCK_COLOR_PAIR));
            mvprintw(
                row + TERMINAL_ROW_OFFSET,
                column + TERMINAL_COLUMN_OFFSET,
                "%c", node->ch
            );
            attroff(COLOR_PAIR(TOGGLE_BLOCK_COLOR_PAIR));
            break;
        case BUTTON:
            mvprintw(
                row + TERMINAL_ROW_OFFSET,
                column + TERMINAL_COLUMN_OFFSET,
                "%c", node->ch
            );
            break;
        case SWITCH:
            mvprintw(
                row + TERMINAL_ROW_OFFSET,
                column + TERMINAL_COLUMN_OFFSET,
                "%c", node->ch
            );
            break;
        case LASER:
            attron(COLOR_PAIR(LASER_COLOR_PAIR));
            mvprintw(
                row + TERMINAL_ROW_OFFSET,
                column + TERMINAL_COLUMN_OFFSET,
                "%c", node->ch
            );
            attroff(COLOR_PAIR(LASER_COLOR_PAIR));
            break;
        default:
            break;
    }
    attroff(A_BOLD);
}
