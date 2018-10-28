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

typedef enum {
  PlayAudio_Yes,
  PlayAudio_No,
} PlayAudio;

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

extern void dkess_press_key(int16_t key, CGEventFlags modifier_flags);

void dome_key_config_free(Config *ptr);

Config *dome_key_config_get(void);

void dome_key_do_trial(void);

void dome_key_logger_init(void);

Config *dome_key_parse_args(const char *const *args, size_t length, Config *config_ptr);

void dome_key_run_key_action(State *state, Trigger trigger, PlayAudio play_audio);

void dome_key_state_free(State *ptr);

void dome_key_state_load_map_group(State *ptr);

State *dome_key_state_new(void);

#endif /* DOME_KEY_MAP_H */
