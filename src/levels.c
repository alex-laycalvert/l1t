// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "levels.h"
#include "l1t.h"
#include "node.h"
#include "utils.h"
#include <stdbool.h>
#include <stdio.h>
#include <string.h>

#include <ncurses.h>
#include <stdlib.h>

FileDimensions get_file_dimensions(const char *name) {
    FILE *level_file = fopen(name, "r");
    if (!level_file) {
        err_exit("failed to open file");
    }
    char char_grid[MAX_LEVEL_ROWS][MAX_LEVEL_COLUMNS];
    char line_buf[MAX_LEVEL_COLUMNS];
    size_t rows = 0;
    size_t columns = 0;

    while (fgets(line_buf, MAX_LEVEL_COLUMNS, level_file) != NULL) {
        strncpy(char_grid[rows], line_buf, MAX_LEVEL_COLUMNS);
        if (rows <= 0) {
            for (size_t i = 0; i < MAX_LEVEL_COLUMNS; i++) {
                if (char_grid[rows][i] == '\n') {
                    break; 
                }
                columns++;
            }
        }
        rows++;
    }
    fclose(level_file);
    FileDimensions fd;
    fd.rows = rows;
    fd.columns = columns;
    return fd;
}

Node ** generate_level_grid(const char *name) {
    FILE *level_file = fopen(name, "r");
    if (!level_file) {
        err_exit("failed to open file");
    }

    Node **grid;
    char char_grid[MAX_LEVEL_ROWS][MAX_LEVEL_COLUMNS];
    char line_buf[MAX_LEVEL_COLUMNS];
    int rows = 0;
    int columns = 0;

    while (fgets(line_buf, MAX_LEVEL_COLUMNS, level_file) != NULL) {
        strncpy(char_grid[rows], line_buf, MAX_LEVEL_COLUMNS);
        if (rows <= 0) {
            for (int i = 0; i < MAX_LEVEL_COLUMNS; i++) {
                if (char_grid[rows][i] == '\n') {
                    break; 
                }
                columns++;
            }
        }
        rows++;
    }
    fclose(level_file);

    grid = (Node **)malloc(rows * sizeof(Node *));
    if (!grid) {
        err_exit("failed to allocate memory for grid");
    }
    for (int r = 0; r < rows; r++) {
        grid[r] = (Node *)malloc(columns * sizeof(Node));
        if (grid[r] == NULL) {
            err_exit("failed to allocate memory for grid row");
        }
        for (int c = 0; c < columns; c++) {
            Node node;
            node.row = r;
            node.column = c;
            node.dir = UP;
            node.on = false;
            switch (char_grid[r][c]) {
                case 'I':
                    node.type = WALL;
                    node.ch = WALL_CH;
                    break;
                case 'X':
                    node.type = PLAYER;
                    node.ch = PLAYER_CH;
                    break;
                case '/':
                    node.type = MIRROR_FORWARD;
                    node.ch = MIRROR_FORWARD_CH;
                    break;
                case '\\':
                    node.type = MIRROR_BACKWARD;
                    node.ch = MIRROR_BACKWARD_CH;
                    break;
                case 'K':
                    node.type = BLOCK;
                    node.ch = BLOCK_CH;
                    break;
                case 'S':
                    node.type = STATUE;
                    node.ch = STATUE_CH;
                    break;
                case '1':
                    node.type = LASER;
                    node.ch = LASER_CH;
                    node.on = true;
                    break;
                case '2':
                    node.type = LASER;
                    node.ch = LASER_CH;
                    node.dir = DOWN;
                    node.on = true;
                    break;
                case '3':
                    node.type = LASER;
                    node.ch = LASER_CH;
                    node.dir = LEFT;
                    node.on = true;
                    break;
                case '4':
                    node.type = LASER;
                    node.ch = LASER_CH;
                    node.dir = RIGHT;
                    node.on = true;
                    break;
                case '5':
                    node.type = LASER;
                    node.ch = LASER_CH;
                    break;
                case '6':
                    node.type = LASER;
                    node.ch = LASER_CH;
                    node.dir = DOWN;
                    break;
                case '7':
                    node.type = LASER;
                    node.ch = LASER_CH;
                    node.dir = LEFT;
                    break;
                case '8':
                    node.type = LASER;
                    node.ch = LASER_CH;
                    node.dir = RIGHT;
                    break;
                default:
                    node.type = EMPTY;
                    node.ch = EMPTY_CH;
                    break;
            }
            grid[r][c] = node;
        }
    }
    return grid;
}
