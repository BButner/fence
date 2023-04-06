#include <stdint.h>
#include <stdbool.h>

typedef struct MouseLocation
{
	int32_t x;
	int32_t y;
} MouseLocation;

bool init_fence();

void update_mouse_location(int32_t x, int32_t y);