#include <stdio.h>
#include "dome_key_map.h"

#define SIZE 2

int main() {
	HeadphoneButton mode_buttons[SIZE] = {HeadphoneButton_Play, HeadphoneButton_Up};
	Trigger mode_trigger = {
		.buttons = mode_buttons,
		.length = SIZE
	};
	const CKeyActionResult *mode = c_run_key_action(mode_trigger, NULL);
	/* printf("%d\n", *mode->kind); */

	HeadphoneButton buttons[] = {HeadphoneButton_Down};
	Trigger trigger = {
		.buttons = buttons,
		.length = 1
	};
	const CKeyActionResult *result = c_run_key_action(trigger, mode->in_mode);

	printf("%d\n", *result->kind);
	printf("%s", result->action);

	return 0;
}
