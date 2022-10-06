// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#ifndef LEVELS_H_
#define LEVELS_H_

#define MAX_LEVEL_ROWS 70
#define MAX_LEVEL_COLUMNS 180

#include "node.h"
#include <stdbool.h>
#include <stdlib.h>

size_t * get_file_dimensions(const char *name);
Node ** generate_level_grid(const char *name);

#endif // LEVELS_H_

