#ifndef GIGASECOND_H
#define GIGASECOND_H

#include "boost/date_time/posix_time/posix_time.hpp"

using boost::posix_time::ptime;
using boost::posix_time::time_duration;

namespace gigasecond {
  const ptime advance(ptime t)
  {
    constexpr unsigned int giga = 1000000000;
    return t+time_duration(0, 0, giga);
  }
}
#endif
