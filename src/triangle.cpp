#include "triangle.h"
#include <cstdint>

void int_swap(int* a, int* b) {
  int tmp = *a;
  *a = *b;
  *b = tmp;
}

void fill_flat_bottom_triangle(int x0, int y0, int x1, int y1, int x2, int y2, uint32_t color) {
  // TODO: Implement the draw filled triangle function
}

void fill_flat_top_triangle(int x0, int y0, int x1, int y1, int x2, int y2, uint32_t color) {
  // TODO: Implement the draw filled triangle function
}

void draw_filled_triangle(int x0, int y0, int x1, int y1, int x2, int y2, uint32_t color) {
  // Sort the vertices by y-coords ascending y0 < y1 < y2
  if (y0 > y1) {
    int_swap(&y0, &y1);
    int_swap(&x0, &x1);
  }

  if (y1 > y2) {
    int_swap(&y1, &y2);
    int_swap(&x1, &x2);
  }

  if (y0 > y1) {
    int_swap(&y0, &y1);
    int_swap(&x0, &x1);
  }

  // Calculate the new vertex (mx, my) using triangle similarity
  int My = y1;
  int Mx = (static_cast<float>(x2 - x0) * static_cast<float>(y1 - y0) / static_cast<float>(y2 - y0)) + x0;
  
  fill_flat_bottom_triangle(x0, y0, x1, y1, Mx, My, 0xFFFFFF00);
  fill_flat_top_triangle(x1, y1, Mx, My, x2, x2, 0xFFFF00FF);
}
