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

typedef struct State_K State_K;

typedef struct {
  const HeadphoneButton *buttons;
  size_t length;
} Trigger;

typedef struct {
  const char *action;
  const ActionKind *kind;
  const Trigger *in_mode;
} CKeyActionResult;

const CKeyActionResult *c_run_key_action(State_K *state, Trigger trigger, const Trigger *mode);

void logger_init(void);

void state_free(State_K *ptr);

void state_load_map_group(State_K *ptr);

State_K *state_new(void);
