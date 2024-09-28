#include "graphics/line.h"

#include "graphics/pixel.h"

void mid_point_v1(uint32_t *pixels, SDL_Color *color, uint32_t x0, uint32_t y0,
		  uint32_t x1, uint32_t y1)
{
    uint32_t dy = y1 - y0, dx = x1 - x0;
    uint32_t d = 2 * dy - dx;
    uint32_t x = x1, y = y1;
    for (uint32_t i = 0; i < dx; ++i) {
        pixels[POS_AT(x, y)] = *(uint32_t*)color;
        if (d > 0) {
            y = y + 1;
            d = d + 2 * (dy - dx);
        } else {
            d = d + 2 * dy;
        }
        ++x;
    }
}
