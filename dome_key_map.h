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

typedef struct MapKind MapKind;

typedef struct Option_CString Option_CString;

typedef struct {
  const char *action;
  const MapKind *kind;
} CKeyActionResult;

typedef struct {
  Option_CString action;
  MapKind kind;
} KeyActionResult;

const CKeyActionResult *c_run_key_action(const HeadphoneButton *trigger, size_t length);
