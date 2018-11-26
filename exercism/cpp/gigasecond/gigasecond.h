#include <string>
#include "boost/date_time/posix_time/posix_time.hpp"

using std:: string;
using namespace boost::posix_time;

namespace gigasecond {
  const ptime advance(ptime t)
  {
    return t+time_duration(0, 0, pow(10,9));
  }
}

