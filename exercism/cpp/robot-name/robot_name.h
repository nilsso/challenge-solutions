#ifndef ROBOT_NAME_H
#define ROBOT_NAME_H
//#define ROBOT_NAME_MAIN
#define EXERCISM_RUN_ALL_TESTS

#include <vector>
#include <string>
#include <algorithm>
#include <random>

using std::vector;
using std::string;

namespace robot_name
{
  class Robot
  {
  public:
    Robot():
      m_name{pop_name()}
    {}

    const string& name() const
    { return m_name; }

    void reset()
    { m_name = pop_name(); }

    static size_t names_remaining()
    { return s_names.size(); }

  private: 
    string m_name;

    // eg: RX811
    // 26*26*10*10*10 = 676000 possibilities
    static void init_names();
    static string pop_name();

    static const size_t s_num_names = 676000;
    static vector<string> s_names;
    static std::random_device s_rd;
    static std::default_random_engine s_dre;
    static std::uniform_int_distribution<size_t> s_uid;
  };

  using robot = Robot;
}
#endif

