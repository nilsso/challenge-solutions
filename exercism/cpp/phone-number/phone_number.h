#ifndef PHONE_NUMBER_H
#define PHONE_NUMBER_H
//#define PHONE_NUMBER_MAIN
#define EXERCISM_RUN_ALL_TESTS

#include <string>
#include <regex>

using std::string;

class PhoneNumber
{
public:
  PhoneNumber(const string& s);

  const string& number() const
  { return m_number; }

  const string& area_code() const
  { return m_area_code; }

  explicit operator string() const
  { return m_pretty; }

  static const std::regex pattern;

private:
  PhoneNumber() = delete;

  static bool validate_number(const string& s);

  string m_number;
  string m_area_code;
  string m_pretty;
};

using phone_number = PhoneNumber;

#endif

