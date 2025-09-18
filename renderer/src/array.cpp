#include "array.h"
#include <cstddef>
#include <cstdlib>

#define ARRAY_RAW_DATA(array) ((int *)(array) - 2)
#define ARRAY_CAPACITY(array) (ARRAY_RAW_DATA(array)[0])
#define ARRAY_OCCUPIED(array) (ARRAY_RAW_DATA(array)[1])

void *array_hold(void *array, int count, int item_size) {
  if (array == NULL) {
    int raw_size = (sizeof(int) * 2) + (item_size * count);
    int *base = (int *)malloc(raw_size);
    base[0] = count;
    base[1] = count;
    return base + 2;
  } else if (ARRAY_OCCUPIED(array) + count <= ARRAY_CAPACITY(array)) {
    ARRAY_OCCUPIED(array) += count;
    return array;
  } else {
    int needed_size = ARRAY_OCCUPIED(array) + count;
    int float_curr = ARRAY_CAPACITY(array) * 2;
    int capacity = needed_size > float_curr ? needed_size : float_curr;
  }
}
