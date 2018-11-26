#pragma once

#include <string>
#include <vector>
#include <set>
#include <algorithm>

using std::string;
using std::vector;
using std::multiset;
using std::for_each;
using std::transform;

namespace anagram
{
  inline string str_tolower(string s)
  {
    transform(s.begin(), s.end(), s.begin(), [](char c){ return tolower(c); });
    return s;
  }

  inline string str_sorted(string s)
  {
    sort(s.begin(), s.end());
    return s;
  }

  class Anagram
  {
  private:
    Anagram() = delete;

    string m_original;
    string m_sorted;

  public:
    Anagram(const string& word):
      m_original{word},
      m_sorted{str_sorted(str_tolower(word))}
    {}

    vector<string> matches(vector<string> words)
    {
      auto comp = [&](string word)
      {
        word = str_tolower(word);
        return (str_tolower(m_original) == word) || (m_sorted != str_sorted(word));
      };
      words.erase(remove_if(words.begin(), words.end(), comp), words.end());
      return words;
    }
  };

  using anagram = Anagram;
}

