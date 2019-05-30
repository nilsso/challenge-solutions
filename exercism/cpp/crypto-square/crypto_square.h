#ifndef CRYPTO_SQUARE_H
#define CRYPTO_SQUARE_H
//#define CRYPTO_SQUARE_MAIN
#define EXERCISM_RUN_ALL_TESTS

#include <cmath>
#include <string>
#include <vector>
#include <algorithm>
#include <sstream>

namespace crypto_square
{
  using std::string;
  using std::string_view;
  using std::vector;
  using std::stringstream;

  class Cipher
  {
  private:
    string m_plaintext;
    vector<string> m_segments;
    string m_normalizedciphertext;
    string m_ciphertext;

  public:
    Cipher (string s);

    const string& normalize_plain_text() const
    { return m_plaintext; }

    const vector<string>& plain_text_segments() const
    { return m_segments; }

    const string& cipher_text() const
    { return m_ciphertext; }

    const string& normalized_cipher_text() const
    { return m_normalizedciphertext; }
  };

  using cipher = Cipher;
}
#endif
