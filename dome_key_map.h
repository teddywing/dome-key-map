#ifndef DOME_KEY_MAP_H
#define DOME_KEY_MAP_H

/* Generated with cbindgen:0.6.6 */

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef enum {
  HeadphoneButton_Play,
  HeadphoneButton_Up,
  HeadphoneButton_Down,
} HeadphoneButton;

typedef struct State State;

typedef struct {
  bool reload;
  bool daemon;
  bool version;
  char *license;
} Args;

typedef uint16_t Milliseconds;

typedef struct {
  Args args;
  Milliseconds timeout;
} Config;

typedef struct {
  const HeadphoneButton *buttons;
  size_t length;
} Trigger;

Config *c_parse_args(const char *const *args, size_t length, Config *config_ptr);

void c_run_key_action(State *state, Trigger trigger);

void config_free(Config *ptr);

Config *config_get(void);

extern void dkess_press_key(int16_t key, CGEventFlags modifier_flags);

void do_trial(void);

void logger_init(void);

void state_free(State *ptr);

void state_load_map_group(State *ptr);

State *state_new(void);

#endif /* DOME_KEY_MAP_H */
