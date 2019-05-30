#ifndef PRIME_FACTORS_H
#define PRIME_FACTORS_H
#define EXERCISM_RUN_ALL_TESTS

#include <vector>

namespace prime_factors
{
  using std::vector;

  template <typename T>
  vector<T> of(T n)
  {
    vector<T> pfs;
    while (n % 2 == 0) {
      pfs.push_back(2);
      n /= 2;
    }
    for (T d = 3; n > d; d += 2) {
      while (n % d == 0) {
        pfs.push_back(d);
        n /= d;
      }
    }
    if (n > 1)
      pfs.push_back(n);
    return pfs;
  }
}
#endif
