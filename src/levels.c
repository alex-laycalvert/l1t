// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "levels.h"
#include "l1t.h"
#include "node.h"

void init_walls(const int rows, const int columns, Node **grid) {
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

void place_item(NodeType item, const int row, const int column, Node **grid) {
    (&grid[row][column])->type = item;
    switch (item) {
        case EMPTY:
            (&grid[row][column])->ch = EMPTY_CH;
            break;
        case PLAYER:
            (&grid[row][column])->ch = PLAYER_CH;
            break;
        case WALL:
            (&grid[row][column])->ch = WALL_CH; break;
        case MIRROR_FORWARD:
            (&grid[row][column])->ch = MIRROR_FORWARD_CH;
            break;
        case MIRROR_BACKWARD:
            (&grid[row][column])->ch = MIRROR_BACKWARD_CH;
            break;
        case BLOCK:
            (&grid[row][column])->ch = BLOCK_CH;
            break;
        case STATUE:
            (&grid[row][column])->ch = STATUE_CH;
            break;
        case TOGGLE_BLOCK:
            (&grid[row][column])->ch = TOGGLE_BLOCK_CH;
            break;
        case BUTTON:
            (&grid[row][column])->ch = BUTTON_CH;
            break;
        case SWITCH:
            (&grid[row][column])->ch = SWITCH_CH;
            break;
        case LASER:
            (&grid[row][column])->ch = LASER_CH;
            break;
        default:
            break;
    }
}

/*
 * Level 000: Lonely
 * Description: A playground/testing area for development of new features.
 */
void init_level_000(const int rows, const int columns, Node **grid) {
    init_walls(rows, columns, grid);
    place_item(PLAYER, rows / 2, columns / 2, grid);
    place_item(WALL, rows / 4, columns / 2, grid);
}

/*
 * Level 001: The Basics
 * Description: The first official level of the game. Get used to game mechanics.
 */
void init_level_001(const int rows, const int columns, Node **grid) {
    init_walls(rows, columns, grid);
    place_item(PLAYER, rows / 2, columns / 2, grid);
}
