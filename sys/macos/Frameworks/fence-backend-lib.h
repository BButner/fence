#include <stdint.h>
#include <stdbool.h>

typedef struct Display
{
	int32_t width;
	int32_t height;
	int32_t top;
	int32_t left;
	bool is_primary;
} Display;

typedef struct DisplayArray
{
	Display *displays;
	int32_t array_length;
} DisplayArray;

typedef DisplayArray (*get_displays_fn)(void);

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

bool init_fence(get_displays_fn get_displays);

UpdateMouseLocationResult try_update_mouse_location(int32_t x, int32_t y);