// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "node.h"
#include "l1t.h"
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
            mvprintw(
                row + TERMINAL_ROW_OFFSET,
                column + TERMINAL_COLUMN_OFFSET,
                "%c", node->ch
            );
            break;
        case WALL:
            mvprintw(
                row + TERMINAL_ROW_OFFSET,
                column + TERMINAL_COLUMN_OFFSET,
                "%c", node->ch
            );
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
            mvprintw(
                row + TERMINAL_ROW_OFFSET,
                column + TERMINAL_COLUMN_OFFSET,
                "%c", node->ch
            );
            break;
        case STATUE:
            mvprintw(
                row + TERMINAL_ROW_OFFSET,
                column + TERMINAL_COLUMN_OFFSET,
                "%c", node->ch
            );
            break;
        case TOGGLE_BLOCK:
            mvprintw(
                row + TERMINAL_ROW_OFFSET,
                column + TERMINAL_COLUMN_OFFSET,
                "%c", node->ch
            );
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
            mvprintw(
                row + TERMINAL_ROW_OFFSET,
                column + TERMINAL_COLUMN_OFFSET,
                "%c", node->ch
            );
            break;
        default:
            break;
    }
    attroff(A_BOLD);
}
