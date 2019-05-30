#ifndef SUM_OF_MULTIPLES_H
#define SUM_OF_MULTIPLES_H
#define EXERCISM_RUN_ALL_TESTS

#include <vector>
#include <algorithm>

using std::vector;

namespace sum_of_multiples
{
  unsigned int to(vector<unsigned int> g, unsigned int lim)
  {
    unsigned int sum = 0;
    for (unsigned int i = 1; i < lim; ++i) {
      if (std::any_of(g.begin(), g.end(),
          [&](unsigned int n) { return i % n == 0; })) {
        sum += i;
      }
    }
    return sum;
  }
}

#endif
