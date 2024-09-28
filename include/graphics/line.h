#ifndef GRAPHICS_LINE_H
#define GRAPHICS_LINE_H

#include <stdint.h>

#include <SDL2/SDL.h>

void mid_point_v1(uint32_t *pixels, SDL_Color *color, uint32_t x0, uint32_t y0,
		  uint32_t x1, uint32_t y1);
void mid_point_v2(uint32_t *pixels, SDL_Color *color, uint32_t x0, uint32_t y0,
		  uint32_t x1, uint32_t y1);

#endif
