#pragma once

#include <string>

using std::string;

namespace bob
{
  constexpr char FINE[] = "Fine. Be that way!";
  constexpr char WHOA[] = "Whoa, chill out!";
  constexpr char SURE[] = "Sure.";
  constexpr char WE[] = "Whatever.";

  string hey(string s)
  {
    char last;
    int na = 0; // Alpha count
    int nc = 0; // Uppercase count
    for (char c: s) {
      if (isgraph(c)) last = c;
      if (isalpha(c)) ++na;
      if (isupper(c)) ++nc;
    }
    return !last ? FINE : ((float(nc)/na)>.5 ? WHOA : (last=='?' ? SURE : WE));
  }
}

