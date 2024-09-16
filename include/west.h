#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct List_u8 {
  const uint8_t *ptr;
  uintptr_t len;
} List_u8;

typedef struct Config {
  struct List_u8 wasm;
} Config;

uintptr_t error_take(char *buf, uintptr_t len);

uintptr_t error_len(void);

void *instance_new(struct Config config);

void instance_free(void *instance);

bool instance_call(void *instance_ptr, const char *instance, const char *name, void *const *args);
