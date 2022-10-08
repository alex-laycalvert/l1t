// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "l1t.h"
#include "levels.h"
#include "utils.h"
#include "colors.h"
#include <stdlib.h>
#include <ncurses.h>
#include <stdbool.h>

bool is_grid_initialized = false;
int current_level = -1;
int rows, columns, terminal_rows, terminal_columns, terminal_row_offset, terminal_column_offset;
Node *player;
int num_statues;
Node **statues;
int num_reverse_statues;
Node **reverse_statues;
Node **grid;

void init_level(const int level, const int term_rows, const int term_columns) {
    current_level = level;
    LevelInfo info;
    switch (level) {
        default:
            info = generate_level("src/levels/000.l1t");
            break;
    }
    terminal_rows = term_rows;
    terminal_columns = term_columns;
    rows = info.rows;
    columns = info.columns;
    terminal_row_offset = terminal_rows / 2 - rows / 2;
    terminal_column_offset = terminal_columns / 2 - columns / 2;
    player = info.player; 
    num_statues = info.num_statues;
    statues = info.statues;
    num_reverse_statues = info.num_reverse_statues;
    reverse_statues = info.reverse_statues;
    grid = info.grid;
    is_grid_initialized = true;
    clear();
    resizeterm(terminal_rows, terminal_columns);
    /* destroy_level(); */
    /* endwin(); */
    /* printf("ROWS: %d COLUMNS: %d\n", rows, columns); */
    /* printf("TROWS: %d TCOLUMNS: %d\n", terminal_rows, terminal_columns); */
    /* printf("TROWSO: %d TCOLUMNSO: %d\n", terminal_row_offset, terminal_column_offset); */
    /* exit(0); */
}

void print_border() {
    mvhline(terminal_row_offset, terminal_column_offset + 1, 0, columns - 2);
    mvhline(terminal_row_offset + rows - 1, terminal_column_offset + 1, 0, columns - 2);
    mvvline(terminal_row_offset + 1, terminal_column_offset, 0, rows - 2);
    mvvline(terminal_row_offset + 1, terminal_column_offset + columns - 1, 0, rows - 2);
    mvaddch(terminal_row_offset, terminal_column_offset, ACS_ULCORNER);
    mvaddch(terminal_row_offset, terminal_column_offset + columns - 1, ACS_URCORNER);
    mvaddch(terminal_row_offset + rows - 1, terminal_column_offset, ACS_LLCORNER);
    mvaddch(terminal_row_offset + rows - 1, terminal_column_offset + columns - 1, ACS_LRCORNER);
}

void print_grid() {
    if (!is_grid_initialized) {
        err_exit("grid is not initialized");
    }
    for (int r = 1; r < rows - 1; r++) {
        for (int c = 1; c < columns - 1; c++) {
            print_node(r + terminal_row_offset, c + terminal_column_offset, &grid[r][c]);
        }
    }
}

void print_lasers() {
    if (!is_grid_initialized) {
        err_exit("grid is not initialized");
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
    attron(COLOR_PAIR(LASER_BEAM_COLOR_PAIR));
    while (
        grid[current_row][current_column].type == EMPTY ||
        grid[current_row][current_column].type == MIRROR_FORWARD ||
        grid[current_row][current_column].type == MIRROR_BACKWARD
    ) {
        if (grid[current_row][current_column].type == EMPTY) {
            mvprintw(
                current_row + terminal_row_offset,
                current_column + terminal_column_offset,
                "%c", laser_line_ch
            );
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
        mvprintw(
            current_row - row_offset + terminal_row_offset,
            current_column - column_offset + terminal_column_offset,
            "%c", laser_dir_ch
        );
    }
    attroff(COLOR_PAIR(LASER_BEAM_COLOR_PAIR));
    if (grid[current_row][current_column].type == STATUE) {
        (&grid[current_row][current_column])->on = true;
    }
    if (grid[current_row][current_column].type == REVERSE_STATUE) {
        (&grid[current_row][current_column])->on = false;
    }
    if (grid[current_row][current_column].type == LASER) {
        (&grid[current_row][current_column])->on = false;
    }
}

void clear_grid() {
    if (!is_grid_initialized) {
        err_exit("grid is not initialized");
    }
    for (int r = 0; r < rows; r++) {
        for (int c = 0; c < columns; c++) {
            mvprintw(r + terminal_row_offset, c + terminal_column_offset, " ");
        }
    }
}

void restart_level() {
    destroy_level();
    init_level(current_level, terminal_rows, terminal_columns);
}

void destroy_level() {
    if (!is_grid_initialized) {
        return;
    }
    if (statues) {
        free(statues);
    }
    if (reverse_statues) {
        free(reverse_statues);
    }
    for (int r = 0; r < rows; r++) {
        free(grid[r]);
    }
    free(grid);
}

void reset_statues() {
    if (!is_grid_initialized) {
        err_exit("grid is not initialized");
    }
    if (num_statues > 0) {
        for (int i = 0; i < num_statues; i++) {
            statues[i]->on = false;
        }
    }
    if (num_reverse_statues > 0) {
        for (int i = 0; i < num_reverse_statues; i++) {
            reverse_statues[i]->on = true;
        }
    }
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

void perform_player_interaction() {
    if (!is_grid_initialized) {
        err_exit("grid is not initialized");
    }
    for (int r = player->row - 1; r < player->row + 2; r++) {
        for (int c = player->column - 1; c < player->column + 2; c++) {
            if (r == player->row && c == player->column) {
                continue;
            }
            Node *grid_item = &grid[r][c];
            switch (grid_item->type) {
                case LASER:
                    grid_item->on = !grid_item->on;
                    break;
                default:
                    break;
            }
        }
    }
}

bool check_win() {
    if (!is_grid_initialized) {
        err_exit("grid is not initialized");
    }
    for (int i = 0; i < num_statues; i++) {
        if (!statues[i]->on) {
            return false;
        }
    }
    for (int i = 0; i < num_reverse_statues; i++) {
        if (!reverse_statues[i]->on) {
            return false;
        }
    }
    return true;
}

bool play() {
    if (!is_grid_initialized) {
        err_exit("grid is not initialized");
    }
    bool playing = true;
    bool won = false;
    print_border();
    while (playing) {
        print_grid();
        reset_statues();
        print_lasers();
        char input = getch();
        switch (input) {
            case QUIT_KEY:
                playing = false;
                break;
            case RESTART_KEY:
                restart_level();
                break;
            case INTERACT_KEY:
                perform_player_interaction();
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
        reset_statues();
        print_lasers();
        if (check_win()) {
            won = true;
            playing = false;
            break;
        }
    }
    return won;
}
