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

void init_level(const int level) {
    current_level = level;
    size_t *file_dimensions;
    switch (level) {
        case 1:
            /* init_level_001(rows, columns, grid); */
            break;
        default:
            /* init_level_000(rows, columns, grid); */
            file_dimensions = get_file_dimensions("src/levels/000.l1t");
            rows = file_dimensions[0];
            columns = file_dimensions[1];
            grid = generate_level_grid("src/levels/000.l1t");
            is_grid_initialized = true;
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
    for (int r = 0; r < rows; r++) {
        for (int c = 0; c < columns; c++) {
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
    Direction current_dir = dir;
    int row_offset, column_offset;
    char laser_dir_ch = ' ';
    char laser_line_ch = ' ';
    switch (current_dir) {
        case UP:
            row_offset = -1;
            column_offset = 0;
            laser_dir_ch = UP_LASER_CH;
            laser_line_ch = VERTICAL_LINE_CH;
            break;
        case DOWN:
            row_offset = 1;
            column_offset = 0;
            laser_dir_ch = DOWN_LASER_CH;
            laser_line_ch = VERTICAL_LINE_CH;
            break;
        case LEFT:
            row_offset = 0;
            column_offset = -1;
            laser_dir_ch = LEFT_LASER_CH;
            laser_line_ch = HORIZONTAL_LINE_CH;
            break;
        case RIGHT:
            row_offset = 0;
            column_offset = 1;
            laser_dir_ch = RIGHT_LASER_CH;
            laser_line_ch = HORIZONTAL_LINE_CH;
            break;
    }
    int current_row = row + row_offset;
    int current_column = column + column_offset;
    while (
        grid[current_row][current_column].type == EMPTY ||
        grid[current_row][current_column].type == MIRROR_FORWARD ||
        grid[current_row][current_column].type == MIRROR_BACKWARD
    ) {
        if (grid[current_row][current_column].type == EMPTY) {
            mvprintw(current_row, current_column, "%c", laser_line_ch);
            current_row += row_offset;
            current_column += column_offset;
        }
        if (grid[current_row][current_column].type == MIRROR_FORWARD) {
            switch (current_dir) {
                case UP:
                    current_dir = RIGHT;
                    laser_dir_ch = RIGHT_LASER_CH;
                    laser_line_ch = HORIZONTAL_LINE_CH;
                    row_offset = 0;
                    column_offset = 1;
                    break;
                case DOWN:
                    current_dir = LEFT;
                    laser_dir_ch = LEFT_LASER_CH;
                    laser_line_ch = HORIZONTAL_LINE_CH;
                    row_offset = 0;
                    column_offset = -1;
                    break;
                case LEFT:
                    current_dir = DOWN;
                    laser_dir_ch = DOWN_LASER_CH;
                    laser_line_ch = VERTICAL_LINE_CH;
                    row_offset = 1;
                    column_offset = 0;
                    break;
                case RIGHT:
                    current_dir = UP;
                    laser_dir_ch = UP_LASER_CH;
                    laser_line_ch = VERTICAL_LINE_CH;
                    row_offset = -1;
                    column_offset = 0;
                    break;
            }
            current_row += row_offset;
            current_column += column_offset;
        } else if (grid[current_row][current_column].type == MIRROR_BACKWARD) {
            switch (current_dir) {
                case UP:
                    current_dir = LEFT;
                    laser_dir_ch = LEFT_LASER_CH;
                    laser_line_ch = HORIZONTAL_LINE_CH;
                    row_offset = 0;
                    column_offset = -1;
                    break;
                case DOWN:
                    current_dir = RIGHT;
                    laser_dir_ch = RIGHT_LASER_CH;
                    laser_line_ch = HORIZONTAL_LINE_CH;
                    row_offset = 0;
                    column_offset = 1;
                    break;
                case LEFT:
                    current_dir = UP;
                    laser_dir_ch = UP_LASER_CH;
                    laser_line_ch = VERTICAL_LINE_CH;
                    row_offset = -1;
                    column_offset = 0;
                    break;
                case RIGHT:
                    current_dir = DOWN;
                    laser_dir_ch = DOWN_LASER_CH;
                    laser_line_ch = VERTICAL_LINE_CH;
                    row_offset = 1;
                    column_offset = 0;
                    break;
            }
            current_row += row_offset;
            current_column += column_offset;
        }
    }
    if (grid[current_row - row_offset][current_column - column_offset].type == EMPTY) {
        mvprintw(current_row - row_offset, current_column - column_offset, "%c", laser_dir_ch);
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
    Node *block_tmp;
    switch (grid[player->row + row_offset][player->column + column_offset].type) {
        case EMPTY:
            tmp = &grid[player->row + row_offset][player->column + column_offset];
            tmp->type = PLAYER;
            tmp->ch = PLAYER_CH;
            player->type = EMPTY;
            player->ch = EMPTY_CH;
            player = tmp;
            break;
        case BLOCK:
            if (
                player->row + row_offset * 2 < 0 ||
                player->row + row_offset * 2 >= rows ||
                player->column + column_offset * 2 < 0 ||
                player->column + column_offset * 2 >= columns
            ) {
                break;
            }
            if (grid[player->row + row_offset * 2][player->column + column_offset * 2].type != EMPTY) {
                break;
            }
            block_tmp = &grid[player->row + row_offset * 2][player->column + column_offset * 2];
            block_tmp->type = BLOCK;
            block_tmp->ch = BLOCK_CH;
            tmp = &grid[player->row + row_offset][player->column + column_offset];
            tmp->type = PLAYER;
            tmp->ch = PLAYER_CH;
            player->type = EMPTY;
            player->ch = EMPTY_CH;
            player = tmp;
            break;
        case MIRROR_FORWARD:
            if (
                player->row + row_offset * 2 < 0 ||
                player->row + row_offset * 2 >= rows ||
                player->column + column_offset * 2 < 0 ||
                player->column + column_offset * 2 >= columns
            ) {
                break;
            }
            if (grid[player->row + row_offset * 2][player->column + column_offset * 2].type != EMPTY) {
                break;
            }
            block_tmp = &grid[player->row + row_offset * 2][player->column + column_offset * 2];
            block_tmp->type = MIRROR_FORWARD;
            block_tmp->ch = MIRROR_FORWARD_CH;
            tmp = &grid[player->row + row_offset][player->column + column_offset];
            tmp->type = PLAYER;
            tmp->ch = PLAYER_CH;
            player->type = EMPTY;
            player->ch = EMPTY_CH;
            player = tmp;
            break;
        case MIRROR_BACKWARD:
            if (
                player->row + row_offset * 2 < 0 ||
                player->row + row_offset * 2 >= rows ||
                player->column + column_offset * 2 < 0 ||
                player->column + column_offset * 2 >= columns
            ) {
                break;
            }
            if (grid[player->row + row_offset * 2][player->column + column_offset * 2].type != EMPTY) {
                break;
            }
            block_tmp = &grid[player->row + row_offset * 2][player->column + column_offset * 2];
            block_tmp->type = MIRROR_BACKWARD;
            block_tmp->ch = MIRROR_BACKWARD_CH;
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
