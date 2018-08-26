#include <stdio.h>
#include "dome_key_map.h"

#define SIZE 2

int main() {
	HeadphoneButton trigger[SIZE] = {Play, Down};
	const CKeyActionResult *result = c_run_key_action(trigger, SIZE);
	printf("%s", result->action);

	return 0;
}
