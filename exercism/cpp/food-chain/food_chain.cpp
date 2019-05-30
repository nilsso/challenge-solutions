#include "food_chain.h"

string food_chain::verse(size_t i)
{
  stringstream ss;
  auto k = next(kernels.begin(), kernels.size()-i);
  auto bird = prev(kernels.end(), 2);
  auto end = prev(kernels.end(), 1);

  // First time through
  ss << "I know an old lady who swallowed a " << k->subject << ".\n" << k->context;

  if (i == kernels.size()) // Horse (bad end)
    return ss.str();

  if (i > 2) { // Anything bird and above
    while (k != bird) {
      ss << "She swallowed the " << k->subject << " to catch the " << (++k)->subject;
      if (k == bird)
        ss << " that wriggled and jiggled and tickled inside her";
      ss << ".\n";
    }
  }

  while (k != end) // "Everything" below bird
    ss << "She swallowed the " << k->subject << " to catch the " << (++k)->subject << ".\n";

  ss << "I don't know why she swallowed the fly. Perhaps she'll die.\n";
  return ss.str();
}

string food_chain::verses(size_t i, size_t j)
{
  stringstream ss;
  for (; i <= j; ++i)
    ss << verse(i) << '\n';
  return ss.str();
}

string food_chain::sing()
{
  return verses(1, kernels.size());
}

#ifdef FOOD_CHAIN_MAIN
#include <iostream>
using std::cout;
int main(int argc, char *argv[])
{
  if (argc < 2)
    cout << food_chain::sing();
  else if (argc > 2)
    cout << food_chain::verses(std::stoi(argv[1]), std::stoi(argv[2]));
  else
    cout << food_chain::verse(std::stoi(argv[1]));
  return 0;
}
#endif

