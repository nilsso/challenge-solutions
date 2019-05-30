#include "grade_school.h"

using grade_school::school;
using grade_school::GradeT;
using grade_school::RosterT;

void school::add(const string& name, int grade)
{
  auto &g = m_roster[grade]; // Didn't know this acts as a default
  g.push_back(name);
  std::sort(g.begin(), g.end());
}

GradeT school::grade(int grade)
{
  if (!m_roster.count(grade))
    return {}; // Eww
  return m_roster.at(grade);
}

