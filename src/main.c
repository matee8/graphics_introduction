#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <SDL2/SDL.h>

#define WIDTH 640
#define HEIGHT 480
#define POS_AT(x, y) ((y) * WIDTH + (x))

int32_t main(void)
{
	uint8_t quit = 0, lmb_down = 0;

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

	uint32_t x, y;
	while (!quit) {
		SDL_UpdateTexture(texture, NULL, pixels,
				  WIDTH * sizeof(pixels[0]));

		while (SDL_PollEvent(&event)) {
			switch (event.type) {
			case SDL_QUIT:
				quit = 1;
				break;
			case SDL_MOUSEBUTTONUP:
				if (event.button.button == SDL_BUTTON_LEFT)
					lmb_down = 0;
				break;
			case SDL_MOUSEBUTTONDOWN:
				if (event.button.button == SDL_BUTTON_LEFT)
					lmb_down = 1;
				break;
			case SDL_MOUSEMOTION:
				x = SDL_clamp(event.motion.x, 0, WIDTH);
				y = SDL_clamp(event.motion.y, 0, HEIGHT);
				if (lmb_down) {
					pixels[POS_AT(x, y)] = 0;
				}
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
