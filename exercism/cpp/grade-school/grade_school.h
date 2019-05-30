#ifndef GRADE_SCHOOL_H
#define GRADE_SCHOOL_H
#define EXERCISM_RUN_ALL_TESTS

#include <map>
#include <vector>
#include <string>
#include <algorithm>

using std::map;
using std::vector;
using std::string;

namespace grade_school
{
  using GradeT = vector<string>;
  using RosterT = map<int, GradeT>;

  class school
  {
  public:
    school() {};

    void add(const string& name, int grade);

    const RosterT& roster() const
    { return m_roster; }

    GradeT grade(int grade);

  private:
    RosterT m_roster;
  };
}

#endif

