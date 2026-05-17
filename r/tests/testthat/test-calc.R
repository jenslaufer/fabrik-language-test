test_that("double_value doubles its input", {
  expect_equal(double_value(0), 0)
  expect_equal(double_value(3), 6)
  expect_equal(double_value(-2), -4)
})

test_that("triple_value triples its input", {
  expect_equal(triple_value(0), 0)
  expect_equal(triple_value(4), 12)
  expect_equal(triple_value(-3), -9)
})
