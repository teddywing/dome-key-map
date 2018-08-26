/* Test */

/* Generated with cbindgen:0.6.2 */

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef enum {
  Play,
  Up,
  Down,
} HeadphoneButton;

typedef enum {
  Map,
  Command,
} MapKind;

typedef struct {
  const char *action;
  const MapKind *kind;
} CKeyActionResult;

const CKeyActionResult *c_run_key_action(const HeadphoneButton *trigger, size_t length);
