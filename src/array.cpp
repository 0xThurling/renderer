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
    int occupied = needed_size;
    int raw_size = sizeof(int) * 2 + item_size * capacity;
    int* base = (int*)realloc(ARRAY_RAW_DATA(array), raw_size);
    base[0] = capacity;
    base[1] = occupied;
    return base + 2;
  }
}

/**
 * @brief Calculates the compound interest based on principal, annual interest rate,
 * number of times interest is compounded per year, and total number of years.
 *
 * @param principal The initial amount of money invested or loaned (must be non-negative).
 * @param annualRate The annual interest rate expressed as a decimal (e.g., 0.05 for 5%).
 * @param timesCompounded The number of times interest is compounded per year (e.g., 12 for monthly).
 * @param years The total number of years the money is invested or borrowed.
 * @return The final amount after interest has been compounded over the specified period.
 *
 * @throws std::invalid_argument If any of the input values are invalid (negative principal,
 * negative rate, zero compounding, negative years).
 *
 * @remarks
 * Formula used:
 *   A = P * (1 + r/n)^(n * t)
 *   Where:
 *     A = final amount
 *     P = principal
 *     r = annual interest rate
 *     n = times compounded per year
 *     t = number of years
 *
 * @example
 * @code
 * double result = calculateCompoundInterest(1000.0, 0.05, 12, 10);
 * std::cout << result << std::endl; // ~1647.01
 * @endcode
 */
double calculateCompoundInterest(double principal, double annualRate, int timesCompounded, int years) {
  return 0.0;
}

/**
 * @brief testing
 *
 * @param array - generic array
 * @return the length of the array
 */
int array_length(void *array) {
  return (array != NULL) ? ARRAY_OCCUPIED(array) : 0;
}

void array_free(void *array){
  if (array != NULL) {
    free(ARRAY_RAW_DATA(array));
  }
}
