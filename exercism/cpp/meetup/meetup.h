#ifndef MEETUP_H
#define MEETUP_H
#define EXERCISM_RUN_ALL_TESTS

#include <boost/date_time/gregorian/gregorian.hpp>

using boost::gregorian::date;
using boost::gregorian::day_iterator;
using boost::date_time::weekdays;
using nth = boost::gregorian::nth_day_of_the_week_in_month;
using last = boost::gregorian::last_day_of_the_week_in_month;
using first_after = boost::gregorian::first_day_of_the_week_after;

namespace meetup
{
  class Scheduler
  {
    unsigned int m_month;
    unsigned int m_year;

  public:
    explicit Scheduler(unsigned int month, unsigned int year):
      m_month{month},
      m_year{year}
    {}

#define NTH_DAY(NTH, DAY, Day) \
    date NTH ## _ ## DAY ## day() const \
    { \
      return nth(nth::NTH, weekdays::Day ## day, m_month).get_date(m_year); \
    }

#define LAST_DAY(DAY, Day) \
    date last_ ## DAY ## day() const \
    { \
      return last(weekdays::Day ## day, m_month).get_date(m_year); \
    }

#define TEENTH_DAY(DAY, Day) \
    date DAY ## teenth() const \
    { \
      return first_after{weekdays::Day ## day}.get_date({m_year, m_month, 12}); \
    }

#define ALL_DAY(DAY, Day) \
    NTH_DAY(first, DAY, Day) \
    NTH_DAY(second, DAY, Day) \
    NTH_DAY(third, DAY, Day) \
    NTH_DAY(fourth, DAY, Day) \
    NTH_DAY(fifth, DAY, Day) \
    LAST_DAY(DAY, Day) \
    TEENTH_DAY(DAY, Day)

    ALL_DAY(mon, Mon)
    ALL_DAY(tues, Tues)
    ALL_DAY(wednes, Wednes)
    ALL_DAY(thurs, Thurs)
    ALL_DAY(fri, Fri)
    ALL_DAY(satur, Satur)
    ALL_DAY(sun, Sun)
  };

  using scheduler = Scheduler;
} // meetup
#endif
