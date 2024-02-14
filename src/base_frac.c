#include "base_frac.h"

void calc_base_frac(const char *sequence, size_t length, double *frac) {
  size_t counts[4] = {0};

  for (size_t i = 0; i < length; i++) {
    switch (sequence[i]) {
    case 'A':
      counts[0]++;
      break;
    case 'C':
      counts[1]++;
      break;
    case 'G':
      counts[2]++;
      break;
    case 'T':
      counts[3]++;
      break;
    }
  }

  for (int i = 0; i < 4; i++) {
    frac[i] = (double)counts[i] / length;
  }
}
