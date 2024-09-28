#ifndef GRAPHICS_PIXEL_H
#define GRAPHICS_PIXEL_H

#include <stddef.h>
#include <stdint.h>

#include <SDL2/SDL.h>

#define WIDTH 640
#define HEIGHT 480
#define POS_AT(x, y) ((y) * WIDTH + (x))

typedef struct {
    uint8_t blue;
    uint8_t green;
    uint8_t red;
    uint8_t alpha;
} Color;

void draw_pixel(uint32_t *pixels, int32_t x, int32_t y, Color *color);

#endif
