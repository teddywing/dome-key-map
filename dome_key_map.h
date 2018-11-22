// Copyright (c) 2018 Teddy Wing
//
// This file is part of DomeKey.
//
// *Purchasing policy notice:* All users of the software are expected to
// purchase a license from Teddy Wing unless they have a good reason not to
// pay. Users who can't purchase a license may apply to receive one for free
// at inquiry@domekey.teddywing.com. Users are free to:
//
// * download, build, and modify the app;
// * share the modified source code;
// * share the purchased or custom-built binaries (with unmodified license
//   and contact info), provided that the purchasing policy is explained to
//   all potential users.
//
// This software is available under a modified version of the Open Community
// Indie Software License:
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose is hereby granted, subject to the following conditions:
//
// * all copies retain the above copyright notice, the above purchasing
//   policy notice and this permission notice unmodified;
//
// * all copies retain the name of the software (DomeKey), the name of the
//   author (Teddy Wing), and contact information (including, but not limited
//   to, inquiry@domekey.teddywing.com, and domekey.teddywing.com URLs)
//   unmodified;
//
// * no fee is charged for distribution of the software;
//
// * the best effort is made to explain the purchasing policy to all users of
//   the software.
//
// THE SOFTWARE IS PROVIDED "AS IS", AND THE AUTHOR AND COPYRIGHT HOLDERS
// DISCLAIM ALL WARRANTIES, EXPRESS OR IMPLIED, WITH REGARD TO THIS SOFTWARE,
// INCLUDING BUT NOT LIMITED TO WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE. IN NO EVENT SHALL THE AUTHOR OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY
// DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA, OR PROFITS, WHETHER
// IN AN ACTION OF CONTRACT, NEGLIGENCE, OR OTHER TORTIOUS ACTION, ARISING
// OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

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
  ModeChange_Activated,
  ModeChange_Deactivated,
} ModeChange;

typedef struct State State;

typedef struct {
  bool reload;
  bool daemon;
  bool audio;
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

void dome_key_run_key_action(State *state, Trigger trigger, void (*on_mode_change)(ModeChange));

void dome_key_state_free(State *ptr);

void dome_key_state_load_map_group(State *ptr);

State *dome_key_state_new(void);

#endif /* DOME_KEY_MAP_H */
