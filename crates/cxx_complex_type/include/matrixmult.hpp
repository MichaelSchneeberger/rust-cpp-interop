#include <memory>
#include <vector>
#include <Eigen/Dense>

#include <rust/cxx.h>

// std::unique_ptr<std::vector<double>> matrix_multiply(
std::array<double, 3> matrix_multiply(
  rust::Vec<double> m,
  rust::Vec<double> v
) {
  auto m_eig = Eigen::Map<const Eigen::Matrix3d>(m.data());
  auto v_eig = Eigen::Map<const Eigen::Vector3d>(v.data());

  Eigen::Vector3d r_eig = m_eig * v_eig;

  std::array<double, 3> result;
  Eigen::Map<Eigen::Vector3d>(result.data()) = r_eig;
  return result;
  // auto r = std::make_unique<std::vector<double>>(r_eig.data(), r_eig.data() + r_eig.size());
  // return r;
}

