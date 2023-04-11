#include <stdint.h>
#include <stdbool.h>

typedef struct MouseLocation
{
	int32_t x;
	int32_t y;
} MouseLocation;

typedef struct UpdateMouseLocationResult
{
	bool updated;
	MouseLocation location;
} UpdateMouseLocationResult;

bool init_fence();

UpdateMouseLocationResult try_update_mouse_location(int32_t x, int32_t y);