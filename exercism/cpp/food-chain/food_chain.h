#ifndef FOOD_CHAIN_H
#define FOOD_CHAIN_H
//#define FOOD_CHAIN_MAIN
#define EXERCISM_RUN_ALL_TESTS

#include <string>
#include <vector>
#include <sstream>

using std::string;
using std::vector;
using std::stringstream;

namespace food_chain
{
  struct Kernel { string subject, context; };

  const vector<Kernel> kernels {
    {"horse", "She's dead, of course!\n"},
    {"cow", "I don't know how she swallowed a cow!\n"},
    {"goat", "Just opened her throat and swallowed a goat!\n"},
    {"dog", "What a hog, to swallow a dog!\n"},
    {"cat", "Imagine that, to swallow a cat!\n"},
    {"bird", "How absurd to swallow a bird!\n"},
    {"spider", "It wriggled and jiggled and tickled inside her.\n"},
    {"fly", ""}
  };

  string verse(size_t i);
  string verses(size_t i, size_t j);
  string sing();
}
#endif

