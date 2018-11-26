#pragma once
#define EXERCISM_RUN_ALL_TESTS

#include <vector>
#include <map>

using std::vector;
using std::map;

namespace etl
{
  using InT = map<int,vector<char>>;
  using OutT = map<char,int>;

  OutT transform(InT in)
  {
    OutT out;
    for (const InT::value_type &p: in)
      for (char c: p.second)
        out.emplace(std::make_pair(tolower(c), p.first));
    return out;
  }
}

