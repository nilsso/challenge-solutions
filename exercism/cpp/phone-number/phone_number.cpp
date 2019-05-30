#include "phone_number.h"

PhoneNumber::PhoneNumber(const string& s)
{
  std::smatch m;
  if (!std::regex_match(s, m, pattern)) {
    m_number = "0000000000";
    m_area_code = "000";
    m_pretty = "(000) (000)-(0000)";
  } else {
    string m1=m[1].str(), m2=m[2].str(), m3=m[3].str();
    m_number = m1+m2+m3;
    m_area_code = m1;
    m_pretty = "("+m1+") "+m2+"-"+m3;
  }
}

const std::regex PhoneNumber::pattern{R"(1?\D*(\d{3})\D*(\d{3})\D*(\d{4})$)"};

#ifdef PHONE_NUMBER_MAIN
#include <iostream>
using std::cout;
int main(int argc, char *argv[])
{
  if (argc < 2)
    return 0;

  string s{argv[1]};
  PhoneNumber pn{s};
  cout
    << pn.number() << '\n'
    << pn.area_code() << '\n'
    << (string)pn << '\n';

  return 0;
}
#endif
