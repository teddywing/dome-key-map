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

typedef struct Config Config;

typedef struct State State;

typedef struct {
  const HeadphoneButton *buttons;
  size_t length;
} Trigger;

void c_run_key_action(State *state, Trigger trigger, const Trigger *mode);

void config_free(Config *ptr);

void logger_init(void);

const Config *parse_args(const char *args, size_t length);

void state_free(State *ptr);

void state_load_map_group(State *ptr);

State *state_new(void);
