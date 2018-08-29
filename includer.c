#include <stdio.h>
#include "dome_key_map.h"

#define SIZE 2

int main() {
	HeadphoneButton buttons[SIZE] = {HeadphoneButton_Play, HeadphoneButton_Down};
	Trigger trigger = {
		.buttons = buttons,
		.length = SIZE
	};
	const CKeyActionResult *result = c_run_key_action(&trigger);
	printf("%s", result->action);

	return 0;
}
