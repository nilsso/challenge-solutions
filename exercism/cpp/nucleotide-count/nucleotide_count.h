#ifndef NUCLEOTIDE_COUNT_H
#define NUCLEOTIDE_COUNT_H

#define EXERCISM_RUN_ALL_TESTS

#include <string>
#include <map>

using std::string;
using std::map;

namespace dna
{
  class counter
  {
  public:
    counter(const string &s);
    const map<char,int>& nucleotide_counts() const;
    size_t count(char c) const;
    void validate_nucleotide(char c) const;

  private:
    map<char,int> m_counts;
  };
}

#endif
