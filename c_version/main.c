#include <stdint.h>
#include <stdio.h>

typedef struct {
  uint32_t _pad[1];
  int32_t data;
} Data;

typedef struct {
  uint8_t *data;
} LowBits;

int32_t untag_bits(LowBits self) {
  uint8_t tag = (uint8_t)((uint64_t)self.data & 0b111);
  uint8_t *ptr = (uint8_t *)((uint64_t)self.data - tag);
  if (tag == 1) {
    return ((Data *)ptr)->data;
  } else {
    return 13;
  }
}

int main() {
  printf("Hello, World!\n");
  return 5;
}
