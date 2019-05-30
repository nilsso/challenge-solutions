#ifndef RAINDROPS_H
#define RAINDROPS_H
#define EXERCISM_RUN_ALL_TESTS

#include <string>

using std::string;
using std::to_string;

namespace raindrops
{
  string convert(unsigned int n)
  {
    bool a = !(n % 3), b = !(n % 5), c = !(n % 7);
    return string()+
      (a ? "Pling" : "")+
      (b ? "Plang" : "")+
      (c ? "Plong" : "")+
      (a||b||c ? "" : to_string(n));
  }
}
#endif
