#include <string>

using std::string;

namespace hamming {
  int compute(string a, string b)
  {
    if (a.size() != b.size())
      throw std::domain_error("strings must be equal size");
    int h = 0;
    for (auto ai = a.begin(), bi = b.begin();
        ai != a.end() && bi != b.end();
        ++ai, ++bi) {
      if (*ai != *bi)
        h += 1;
    }
    return h;
  }
}
