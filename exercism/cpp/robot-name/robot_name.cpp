#include "robot_name.h"

using robot_name::Robot;

// Static members

void Robot::init_names()
{
  s_names.reserve(s_num_names);
  for (char a = 'A'; a <= 'Z'; ++a)
    for (char b = 'A'; b <= 'Z'; ++b)
      for (char c = '0'; c <= '9'; ++c)
        for (char d = '0'; d <= '9'; ++d)
          for (char e = '0'; e <= '9'; ++e)
            s_names.push_back(string()+a+b+c+d+e);
}

string Robot::pop_name()
{
  if (!Robot::s_names.size())
    Robot::init_names();
  return *s_names.erase(next(s_names.begin(), s_uid(s_dre)));
}

vector<string> Robot::s_names;
std::random_device Robot::s_rd;
std::default_random_engine Robot::s_dre{s_rd()};
std::uniform_int_distribution<size_t> Robot::s_uid{0, s_num_names};

#ifdef ROBOT_NAME_MAIN
#include <iostream>
using std::cout;
int main(int argc, char *argv[])
{
  Robot r;
  cout << r.name() << ' ' << Robot::names_remaining() << '\n';
  r.reset();
  cout << r.name() << ' ' << Robot::names_remaining() << '\n';
  return 0;
}
#endif
