// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "levels.h"
#include "l1t.h"
#include "utils.h"
#include <stdlib.h>
#include <ncurses.h>
#include<stdbool.h>

bool is_grid_initialized = false;
int current_level = -1;
int rows, columns;
Node **grid;
Node *player;

void init_level_001(const int trows, const int tcolumns, Node **grid) {
    init_walls();
    (&grid[trows / 2][tcolumns / 2])->type = PLAYER;
    (&grid[trows / 2][tcolumns / 2])->ch = PLAYER_CH;
    player = &grid[trows / 2][tcolumns / 2];
}

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
            break;
    }
}

void init_walls() {
    if (!is_grid_initialized) {
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
    if (!is_grid_initialized) {
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
    if (!is_grid_initialized) {
        err_exit("grid is not initialized");
    }
    for (int r = 0; r < rows; r++) {
        for (int c = 0; c < columns; c++) {
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
