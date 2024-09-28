#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <SDL2/SDL.h>

#include "graphics/pixel.h"
#include "graphics/line.h"

int32_t main(void)
{
	uint8_t quit = 0;

	SDL_Init(SDL_INIT_VIDEO);

	SDL_Window *window = SDL_CreateWindow(
		"Bevezetés a számítógépi grafikába", SDL_WINDOWPOS_UNDEFINED,
		SDL_WINDOWPOS_UNDEFINED, WIDTH, HEIGHT, 0);
	SDL_Renderer *renderer = SDL_CreateRenderer(window, -1, 0);
	SDL_Texture *texture =
		SDL_CreateTexture(renderer, SDL_PIXELFORMAT_ARGB8888,
				  SDL_TEXTUREACCESS_STATIC, WIDTH, HEIGHT);
	SDL_SetRenderDrawColor(renderer, 8, 8, 8, 8);
	SDL_Event event;

	uint32_t pixels[WIDTH * HEIGHT];
	memset(pixels, 255, WIDTH * HEIGHT * sizeof(pixels[0]));

	Color red = { .alpha = 255, .red = 0, .green = 0, .blue = 255 };

	mid_point_v2(pixels, &red, WIDTH / 2, HEIGHT / 2, WIDTH - 1, 0);

	while (!quit) {
		SDL_UpdateTexture(texture, NULL, pixels,
				  WIDTH * sizeof(pixels[0]));

		while (SDL_PollEvent(&event)) {
			switch (event.type) {
			case SDL_QUIT:
				quit = 1;
				break;
			}
		}

		SDL_RenderClear(renderer);
		SDL_RenderCopy(renderer, texture, NULL, NULL);
		SDL_RenderPresent(renderer);
	}

	SDL_DestroyTexture(texture);
	SDL_DestroyRenderer(renderer);
	SDL_DestroyWindow(window);
	SDL_Quit();

	return EXIT_SUCCESS;
}
