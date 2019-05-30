#ifndef SCRABBLE_SCORE_H
#define SCRABBLE_SCORE_H
#define EXERCISM_RUN_ALL_TESTS

#include <string>
#include <numeric>

using std::string;

namespace scrabble_score
{
  const unsigned int letter_values[] = {
    // A  B  C  D  E  F  G  H  I  J  K  L  M  N  O  P  Q   R  S  T  U  V  W  X  Y  Z
       1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10
  };

  unsigned int score(const string &s)
  {
    return std::accumulate(s.begin(), s.end(), 0,
        [&](unsigned int t, char c){ return t+letter_values[toupper(c)-'A']; });
  }

} // scrabble_score

#endif // SCRABBLE_SCORE_H
