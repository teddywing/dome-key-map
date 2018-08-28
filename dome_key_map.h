/* Test */

/* Generated with cbindgen:0.6.2 */

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef enum {
  HeadphoneButton_Play,
  HeadphoneButton_Up,
  HeadphoneButton_Down,
} HeadphoneButton;

typedef enum {
  MapKind_Map,
  MapKind_Command,
} MapKind;

typedef struct {
  const char *action;
  const MapKind *kind;
} CKeyActionResult;

typedef struct {
  const HeadphoneButton *buttons;
  size_t length;
} Trigger;

const CKeyActionResult *c_run_key_action(Trigger trigger);
