#ifndef DIFFERENCE_OF_SQUARES_H
#define DIFFERENCE_OF_SQUARES_H

#define EXERCISM_RUN_ALL_TESTS

namespace squares
{
  constexpr int square_of_sums(int n)
  {
    return n*n*(n+1)*(n+1)/4;
  }

  constexpr int sum_of_squares(int n)
  {
    return n*(n+1)*(2*n+1)/6;
  }

  constexpr int difference(int n)
  {
    return square_of_sums(n)-sum_of_squares(n);
  }

}

#endif
