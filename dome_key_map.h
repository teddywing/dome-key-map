/* Test */

/* Generated with cbindgen:0.6.2 */

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef enum {
  ActionKind_Map,
  ActionKind_Command,
  ActionKind_Mode,
} ActionKind;

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
  const ActionKind *kind;
  const HeadphoneButton *in_mode;
} CKeyActionResult;

typedef struct {
  const HeadphoneButton *buttons;
  size_t length;
} Trigger;

const CKeyActionResult *c_run_key_action(Trigger trigger, const Trigger *mode);
