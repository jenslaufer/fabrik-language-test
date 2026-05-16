test_that("double_value doubles its input", {
  expect_equal(double_value(0), 0)
  expect_equal(double_value(3), 6)
  expect_equal(double_value(-2), -4)
})
