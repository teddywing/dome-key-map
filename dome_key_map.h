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

typedef struct State State;

typedef struct {
  bool reload;
  bool daemon;
} Args;

typedef struct {
  Args args;
} Config;

typedef struct {
  const HeadphoneButton *buttons;
  size_t length;
} Trigger;

Config *c_parse_args(const char *const *args, size_t length);

void c_run_key_action(State *state, Trigger trigger, const Trigger *mode);

void config_free(Config *ptr);

extern void dkess_press_key(int16_t key, int16_t modifier_flags);

void logger_init(void);

void state_free(State *ptr);

void state_load_map_group(State *ptr);

State *state_new(void);
