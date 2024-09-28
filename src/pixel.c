#include "graphics/pixel.h"

void draw_pixel(uint32_t *pixels, int32_t x, int32_t y, Color *color)
{
	pixels[POS_AT(x, y)] = *(uint32_t *)color;
}
