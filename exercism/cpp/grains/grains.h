#ifndef GRAINS_H
#define GRAINS_H
#define EXERCISM_RUN_ALL_TESTS

#include <cmath>

namespace grains
{
  unsigned long long square(unsigned int l)
  { return pow(2, l-1); }

  unsigned long long total()
  { return square(65)-1; }
}
#endif
