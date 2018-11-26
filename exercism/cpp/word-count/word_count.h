#pragma once
#define EXERCISM_RUN_ALL_TESTS

#include <string>
#include <map>
#include <utility>
#include <algorithm>

using namespace std;

namespace word_count
{
  bool iswordchar(char c)
  {
    return isalnum(c) || c=='\'';
  }

  template<typename T>
  decltype(auto) next_word(T a, T end)
  {
    a = find_if(a, end, isalnum);
    auto b = find_if_not(a, end, iswordchar);
    if (*prev(b) == '\'') --b;
    return std::make_pair(a, b);
  }

  decltype(auto) words(string s)
  {
    map<string,int> word_counts;
    auto end = s.end();
    auto word = next_word(s.begin(), end);
    while (word.first != end) {
      transform(word.first, word.second, word.first, tolower);
      ++word_counts[string(word.first, word.second)];
      word = next_word(word.second, end);
    }
    return word_counts;
  }
}
