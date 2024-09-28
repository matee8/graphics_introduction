#ifndef GRAPHICS_LINE_H
#define GRAPHICS_LINE_H

#include <stdint.h>

#include <SDL2/SDL.h>

#include "graphics/pixel.h"

void mid_point_v1(uint32_t *pixels, Color *color, int32_t x0, int32_t y0,
		  int32_t x1, int32_t y1);
void mid_point_v2(uint32_t *pixels, Color *color, int32_t x0, int32_t y0,
		  int32_t x1, int32_t y1);

#endif
