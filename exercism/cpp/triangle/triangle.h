#ifndef TRIANGLE_H
#define TRIANGLE_H
#define EXERCISM_RUN_ALL_TESTS

#include <stdexcept>

namespace triangle
{
  enum TriangleType { equilateral, scalene, isosceles };

  TriangleType kind(float a, float b, float c)
  {
    if (a+b <= c || b+c <= a || c+a <= b)
      throw std::domain_error("Invalid triangle");
    if (a == b && b == c && c == a)
      return TriangleType::equilateral;
    if (a != b && b != c && c != a)
      return TriangleType::scalene;
    return TriangleType::isosceles;
  }
}
#endif
