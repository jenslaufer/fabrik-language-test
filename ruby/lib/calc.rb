# frozen_string_literal: true

# Starter module. Every Ruby ticket adds a sibling method here.
module Calc
  module_function

  def double_value(n)
    n * 2
  end

  def triple_value(n)
    n * 3
  end

  def ruby_version_ok?
    Gem::Version.new(RUBY_VERSION) >= Gem::Version.new("3.2")
  end
end
