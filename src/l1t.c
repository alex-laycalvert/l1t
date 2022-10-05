// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "l1t.h"
#include "levels.h"
#include "utils.h"
#include <stdlib.h>
#include <ncurses.h>
#include <stdbool.h>

bool is_grid_initialized = false;
int current_level = -1;
int rows, columns;
Node **grid;
Node *player;

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
    is_grid_initialized = true;
}

void init_level(const int level, const int terminal_rows, const int terminal_columns) {
    if (!is_grid_initialized) {
        init_grid(terminal_rows, terminal_columns);
    }
    current_level = level;
    switch (level) {
        case 1:
            init_level_001(rows, columns, grid);
            break;
        default:
            init_level_000(rows, columns, grid);
            break;
    }
    for (int r = 0; r < rows; r++) {
        for (int c = 0; c < columns; c++) {
            if (grid[r][c].type == PLAYER) {
                player = &grid[r][c];
                break;
            }
        }
    }
}

void print_grid() {
    if (!is_grid_initialized) {
        err_exit("grid is not initialized");
    }
    for (int r = 0; r < rows; r++) {
        for (int c = 0; c < columns; c++) {
            print_node(r, c, &grid[r][c]);
            if (grid[r][c].type == LASER && grid[r][c].on) {
                print_laser(r, c, grid[r][c].dir);
            }
        }
    }
}

void print_laser(const int row, const int column, const Direction dir) {
    if (!is_grid_initialized) {
        err_exit("grid is not initialized");
    }
    int current_row, current_column;
    switch (dir) {
        case UP:
            current_row = row - 1;
            if (current_row <= 0) {
                break;
            }
            while (grid[current_row][column].type == EMPTY) {
                mvprintw(current_row, column, "|");
                current_row -= 1;
            }
            if (grid[current_row + 1][column].type == EMPTY) {
                mvprintw(current_row + 1, column, "^");
            }
            break;
        case DOWN:
            current_row = row + 1;
            if (current_row >= rows - 1) {
                break;
            }
            while (grid[current_row][column].type == EMPTY) {
                mvprintw(current_row, column, "|");
                current_row += 1;
            }
            if (grid[current_row - 1][column].type == EMPTY) {
                mvprintw(current_row - 1, column, "v");
            }
            break;
        case LEFT:
            current_column = column - 1;
            if (current_column <= 0) {
                break;
            }
            while (grid[row][current_column].type == EMPTY) {
                mvprintw(row, current_column, "-");
                current_column -= 1;
            }
            if (grid[row][current_column + 1].type == EMPTY) {
                mvprintw(row, current_column + 1, "<");
            }
            break;
        case RIGHT:
            current_column = column + 1;
            if (current_column >= columns - 1) {
                break;
            }
            while (grid[row][current_column].type == EMPTY) {
                mvprintw(row, current_column, "-");
                current_column += 1;
            }
            if (grid[row][current_column - 1].type == EMPTY) {
                mvprintw(row, current_column - 1, ">");
            }
            break;
    }
}

void clear_grid() {
    if (!is_grid_initialized) {
        err_exit("grid is not initialized");
    }
    for (int r = 0; r < rows; r++) {
        for (int c = 0; c < columns; c++) {
            mvprintw(r, c, " ");
        }
    }
}

void destroy_grid() {
    if (!is_grid_initialized) {
        return;
    }
    for (int r = 0; r < rows; r++) {
        free(grid[r]);
    }
    free(grid);
}

void move_player(Direction dir) {
    if (!is_grid_initialized) {
        err_exit("grid is not initialized");
    }
    int row_offset = 0;
    int column_offset = 0;
    switch (dir) {
        case UP:
            row_offset = -1;
            break;
        case DOWN:
            row_offset = 1;
            break;
        case LEFT:
            column_offset = -1;
            break;
        case RIGHT:
            column_offset = 1;
            break;
        default:
            break;
    }

    Node *tmp;
    switch (grid[player->row + row_offset][player->column + column_offset].type) {
        case EMPTY:
            tmp = &grid[player->row + row_offset][player->column + column_offset];
            tmp->type = PLAYER;
            tmp->ch = PLAYER_CH;
            player->type = EMPTY;
            player->ch = EMPTY_CH;
            player = tmp;
            break;
        default:
            break;
    }
}

bool play() {
    if (!is_grid_initialized) {
        err_exit("grid is not initialized");
    }
    bool playing = true;
    while (playing) {
        print_grid();
        char input = getch();
        switch (input) {
            case QUIT_KEY:
                playing = false;
                break;
            case MOVE_UP_KEY:
                move_player(UP);
                break;
            case MOVE_DOWN_KEY:
                move_player(DOWN);
                break;
            case MOVE_LEFT_KEY:
                move_player(LEFT);
                break;
            case MOVE_RIGHT_KEY:
                move_player(RIGHT);
                break;
            default:
                break;
        }
    }
    return playing;
}
