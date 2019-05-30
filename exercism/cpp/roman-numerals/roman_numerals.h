#ifndef ROMAN_NUMERALS_H
#define ROMAN_NUMERALS_H
#define EXERCISM_RUN_ALL_TESTS

#include <sstream>
#include <string>
#include <array>
#include <utility>

using std::stringstream;
using std::string;
using std::array;
using std::pair;

namespace roman
{
  array<pair<int,string>,13> numerals = {
    std::make_pair(1000,"M"),
    {900,"CM"}, {500,"D"}, {400,"CD"}, {100, "C"},
    {90, "XC"}, {50, "L"}, {40, "XL"}, {10,  "X"},
    {9,  "IX"}, {5,  "V"}, {4,  "IV"}, {1,   "I"}
  };

  string convert(int n)
  {
    stringstream ss;
    for (auto rn: numerals) {
      while (n >= rn.first) {
        n -= rn.first;
        ss << rn.second;
      }
    }
    return ss.str();
  }
}
#endif
