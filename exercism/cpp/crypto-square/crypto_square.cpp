#include "crypto_square.h"

namespace crypto_square
{
  Cipher::Cipher (string s):
    m_plaintext{s}
  {
    // Normalize text (plain)
    std::transform(m_plaintext.begin(), m_plaintext.end(), m_plaintext.begin(), tolower);
    m_plaintext.erase(
        std::remove_if(m_plaintext.begin(), m_plaintext.end(),
          [](char c){ return !isalnum(c); }),
        m_plaintext.end());

    // Make segments
    float n = sqrt(m_plaintext.size());
    size_t cols = ceil(n);
    size_t rows = round(n);
    for (size_t r = 0; r < rows; ++r) {
      auto s = next(m_plaintext.begin(), r*cols);
      auto e = next(m_plaintext.begin(), std::min(r*cols+cols, m_plaintext.size()));
      m_segments.emplace_back(s, e);
    }

    // Make cipher text
    stringstream ss;
    for (size_t c = 0; c < cols; ++c) {
      if (c > 0)
        ss << ' ';
      size_t r = 0;
      for (; r < rows; ++r) {
        auto row = m_segments[r];
        if (c < row.size()) {
          if (isalpha(row[c])) {
            ss << row[c];
          }
        } else {
          ss << ' ';
        }
      }
    }
    m_normalizedciphertext = ss.str();
    m_ciphertext = m_normalizedciphertext;
    m_ciphertext.erase(
        std::remove_if(m_ciphertext.begin(), m_ciphertext.end(),
          [](char c){ return !isalnum(c); }),
        m_ciphertext.end());
  }
}

#ifdef CRYPTO_SQUARE_MAIN
int main()
{
  using crypto_square::Cipher;

  Cipher("This is fun!");
  Cipher("If man was meant to stay on the ground, god would have given us roots.");

  return 0;
}
#endif
