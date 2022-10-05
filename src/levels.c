// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "levels.h"
#include "l1t.h"
#include "utils.h"
#include <stdlib.h>
#include <ncurses.h>

int is_grid_initialized = 0;
int current_level = -1;
int rows, columns;
Node **grid;

void init_grid(const int terminal_rows, const int terminal_columns) {
    rows = terminal_rows;
    columns = terminal_columns;
    grid = (Node **)malloc(rows * sizeof(Node *));
    if (grid == NULL) {
        err_exit("failed to allocate memory for grid");
    }
    for (int i = 0; i < rows; i++) {
        grid[i] = (Node *)malloc(columns * sizeof(Node));
        if (grid[i] == NULL) {
            err_exit("failed to allocate memory for grid row");
        }
    }
    is_grid_initialized = 1;
}

void init_level(const int level, const int terminal_rows, const int terminal_columns) {
    if (is_grid_initialized == 0) {
        init_grid(terminal_rows, terminal_columns);
    }
    current_level = level;
    init_walls();
}

void init_walls() {
    if (is_grid_initialized == 0) {
        err_exit("grid is not initialized");
    }
    for (int r = 0; r < rows; r++) {
        for (int c = 0; c < columns; c++) {
            Node node;
            node.row = r;
            node.column = c;
            if (r == 0 || r == rows - 1 || c == 0 || c == columns - 1) {
                node.type = WALL;
                node.ch = WALL_CH;
            } else {
                node.type = EMPTY;
                node.ch = EMPTY_CH;
            }
            grid[r][c] = node;
        }
    }
}

void print_grid() {
    if (is_grid_initialized == 0) {
        err_exit("grid is not initialized");
    }
    for (int r = 0; r < rows; r++) {
        for (int c = 0; c < columns; c++) {
            attron(A_BOLD);
            switch (grid[r][c].type) {
                case EMPTY:
                    mvprintw(r, c, "%c", grid[r][c].ch);
                    break;
                case PLAYER:
                    mvprintw(r, c, "%c", grid[r][c].ch);
                    break;
                case WALL:
                    mvprintw(r, c, "%c", grid[r][c].ch);
                    break;
                case MIRROR_FORWARD:
                    mvprintw(r, c, "%c", grid[r][c].ch);
                    break;
                case MIRROR_BACKWARD:
                    mvprintw(r, c, "%c", grid[r][c].ch);
                    break;
                case BLOCK:
                    mvprintw(r, c, "%c", grid[r][c].ch);
                    break;
                case STATUE:
                    mvprintw(r, c, "%c", grid[r][c].ch);
                    break;
                case TOGGLE_BLOCK:
                    mvprintw(r, c, "%c", grid[r][c].ch);
                    break;
                case BUTTON:
                    mvprintw(r, c, "%c", grid[r][c].ch);
                    break;
                case SWITCH:
                    mvprintw(r, c, "%c", grid[r][c].ch);
                    break;
                case LASER:
                    mvprintw(r, c, "%c", grid[r][c].ch);
                    break;
                default:
                    break;
            }
            attroff(A_BOLD);
        }
    }
}

void clear_grid() {
    if (is_grid_initialized == 0) {
        err_exit("grid is not initialized");
    }
    for (int r = 0; r < rows; r++) {
        for (int c = 0; c < columns; c++) {
        }
    }
}

void destroy_grid() {
    if (is_grid_initialized == 0) {
        return;
    }
    for (int r = 0; r < rows; r++) {
        free(grid[r]);
    }
    free(grid);
}
