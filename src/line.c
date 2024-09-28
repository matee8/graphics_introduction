#include "graphics/line.h"

#include <stdlib.h>

#include "graphics/pixel.h"

void mid_point_v1(uint32_t *pixels, Color *color, int32_t x0, int32_t y0,
		  int32_t x1, int32_t y1)
{
	int32_t dy = y0 - y1;
    int32_t dx = x1 - x0;
	int32_t d = 2 * dy - dx;

	for (int32_t i = 0, x = x0, y = y0; i < dx; ++i, ++x) {
		draw_pixel(pixels, x, y, color);

		if (d > 0) {
			--y;
			d += 2 * (dy - dx);
		} else {
			d += 2 * dy;
		}
	}
}

void mid_point_v2(uint32_t *pixels, Color *color, int32_t x0, int32_t y0,
		  int32_t x1, int32_t y1)
{
	int32_t dx = abs(x1 - x0);
	int32_t dy = abs(y0 - y1);
	int8_t sx = x1 - x0 >= 0 ? 1 : -1;
	int8_t sy = y0 - y1 >= 0 ? 1 : -1;
	uint8_t swapped = 0;

	if (dx < dy) {
		int32_t tmp = dx;
		dx = dy;
		dy = tmp;
		swapped = 1;
	}
	int32_t d = 2 * dy - dx;
	int32_t x = x0;
	int32_t y = y0;
	draw_pixel(pixels, x, y, color);

	while (x != x1 || y != y1) {
		if (d > 0) {
			if (swapped)
				x += sx;
			else
				y -= sy;
			d -= 2 * dx;
		}
		if (swapped)
			y -= sy;
		else
			x += sx;
		d += 2 * dy;
		draw_pixel(pixels, x, y, color);
	}
}
