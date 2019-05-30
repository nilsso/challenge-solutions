#include "nucleotide_count.h"

using dna::counter;

counter::counter(const string &s):
  m_counts{{'A',0},{'T',0},{'C',0},{'G',0}}
{
  for (char c: s) {
    validate_nucleotide(c);
    ++m_counts[c];
  }
}

const map<char,int>& counter::nucleotide_counts() const
{
  return m_counts;
}

size_t counter::count(char c) const
{
  validate_nucleotide(c);
  return m_counts.at(c);
}

void counter::validate_nucleotide(char c) const
{
  if (!(c == 'A' || c == 'T' || c == 'C' || c == 'G'))
    throw std::invalid_argument("Invalid nucleotide");
}

