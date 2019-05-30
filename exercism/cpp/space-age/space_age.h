#ifndef SPACE_AGE_H
#define SPACE_AGE_H
#define EXERCISM_RUN_ALL_TESTS

namespace space_age
{
  class SpaceAge
  {
    unsigned long m_seconds;

    float
      m_earth_years,
      m_mercury_years,
      m_venus_years,
      m_mars_years,
      m_jupiter_years,
      m_saturn_years,
      m_uranus_years,
      m_neptune_years;

    static constexpr float earth_year_seconds = 365.25*24*60*60;
    static constexpr float mercury_ratio      = 0.2408467;
    static constexpr float venus_ratio        = 0.61519726;
    static constexpr float mars_ratio         = 1.8808158;
    static constexpr float jupiter_ratio      = 11.862615;
    static constexpr float saturn_ratio       = 29.447498;
    static constexpr float uranus_ratio       = 84.016846;
    static constexpr float neptune_ratio      = 164.79132;

  public:
    explicit SpaceAge(unsigned long s):
      m_seconds       { s },
      m_earth_years   { s/earth_year_seconds },
      m_mercury_years { m_earth_years/mercury_ratio },
      m_venus_years   { m_earth_years/venus_ratio },
      m_mars_years    { m_earth_years/mars_ratio },
      m_jupiter_years { m_earth_years/jupiter_ratio },
      m_saturn_years  { m_earth_years/saturn_ratio },
      m_uranus_years  { m_earth_years/uranus_ratio },
      m_neptune_years { m_earth_years/neptune_ratio }
    {}

    unsigned int seconds() const  { return m_seconds; } 
    float on_earth() const        { return m_earth_years; } 
    float on_mercury() const      { return m_mercury_years; } 
    float on_venus() const        { return m_venus_years; } 
    float on_mars() const         { return m_mars_years; } 
    float on_jupiter() const      { return m_jupiter_years; } 
    float on_saturn() const       { return m_saturn_years; } 
    float on_uranus() const       { return m_uranus_years; } 
    float on_neptune() const      { return m_neptune_years; } };

  using space_age = SpaceAge;
}

#endif
