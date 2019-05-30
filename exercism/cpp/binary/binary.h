#ifndef BINARY_H
#define BINARY_H
#define EXERCISM_RUN_ALL_TESTS

#include <algorithm>
#include <string>
#include <cmath>

using std::string;

namespace binary
{
  size_t convert(const string &s)
  {
    size_t n = 0, p = 0;
    for (auto itr = s.rbegin(); itr != s.rend(); ++itr, ++p) {
      if (*itr == '1')
        n |= 1 << p;
      else if(*itr != '0')
        return 0;
    }
    return n;
  }
}
#endif
