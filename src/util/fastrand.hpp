#ifndef FASTRAND_HPP_VIBES3D4
#define FASTRAND_HPP_VIBES3D4

#include <random>
#include <type_traits>

class FastRand {
  public:
    FastRand() : m_engine(std::random_device()()) { }

    float operator()() {
      return std::generate_canonical<float, std::numeric_limits<float>::digits>(m_engine);
    }

  private:
    std::mt19937 m_engine;
};

#endif /* end of include guard: FASTRAND_HPP_VIBES3D4 */
