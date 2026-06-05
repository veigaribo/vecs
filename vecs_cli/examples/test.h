#include <stdint.h>

struct transform {
  double x;
  double y;
};

struct render {
  int texture;
};

typedef struct mouse_click {
  double x;
  double y;
  uint8_t button;
} mouse_click_t;

struct layout {
  int mode;
};
