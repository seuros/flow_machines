# frozen_string_literal: true

require 'test_helper'

module FlowMachines
  class CreditsTest < Minitest::Test
    MAX = (2**32) - 1

    def snapshot(window) = [window.capacity, window.in_flight, window.available]

    def assert_refused(expected, window)
      before = snapshot(window)
      assert_equal expected, yield
      assert_equal before, snapshot(window)
    end

    def test_acquisition_and_release_are_all_or_nothing
      window = CreditWindow.new(3)

      assert_equal :acquired, window.try_acquire(2)
      assert_refused(:insufficient, window) { window.try_acquire(2) }
      assert_equal [2, 1], [window.in_flight, window.available]
      assert_refused(:exceeds_in_flight, window) { window.release(3) }
      assert_equal :released, window.release(2)
      assert_equal [0, 3], [window.in_flight, window.available]
      assert_refused(:exceeds_in_flight, window) { window.release(1) }
    end

    def test_a_repeated_release_is_not_recognised_as_a_repeat
      window = CreditWindow.new(4)

      assert_equal :acquired, window.try_acquire(2)
      assert_equal :released, window.release(1)
      assert_equal :released, window.release(1)
      assert_refused(:exceeds_in_flight, window) { window.release(1) }
    end

    def test_full_u32_capacity_does_not_wrap
      window = CreditWindow.new(MAX)

      assert_equal :acquired, window.try_acquire(MAX)
      assert_refused(:insufficient, window) { window.try_acquire(1) }
      assert_equal 0, window.available
      assert_equal :released, window.release(MAX)
      assert_equal MAX, window.available
    end

    def test_counts_are_positive_integers_without_coercion
      [nil, true, '2', 2.0, Float::NAN, Object.new].each do |value|
        assert_raises(TypeError) { CreditWindow.new(value) }
      end
      [0, -1, 2**32].each do |value|
        assert_raises(RangeError) { CreditWindow.new(value) }
      end
      window = CreditWindow.new(2)
      assert_equal :acquired, window.try_acquire(1)
      %i[try_acquire release].each do |operation|
        assert_raises(TypeError) { window.public_send(operation, 1.0) }
        [0, -1, 2**32].each do |value|
          assert_raises(RangeError) { window.public_send(operation, value) }
        end
      end
      assert_equal [2, 1, 1], snapshot(window)
    end

    def test_the_constructor_is_positional_like_the_core
      assert_raises(ArgumentError) { CreditWindow.new }
      assert_raises(ArgumentError) { CreditWindow.new(3, 4) }
      error = assert_raises(TypeError) { CreditWindow.new(capacity: 3) }
      assert_equal 'capacity must be an Integer', error.message
      error = assert_raises(TypeError) { CreditWindow.new(3).try_acquire(nil) }
      assert_equal 'units must be an Integer', error.message
      error = assert_raises(RangeError) { CreditWindow.new(0) }
      assert_equal 'capacity must be in 1..=4294967295', error.message
    end

    def test_only_the_core_operations_are_exposed
      assert_equal %i[available capacity in_flight release try_acquire],
                   CreditWindow.public_instance_methods(false).sort
    end

    def test_freezing_prevents_native_mutation
      window = CreditWindow.new(2)
      assert_equal :acquired, window.try_acquire(1)
      window.freeze

      assert_raises(FrozenError) { window.try_acquire(1) }
      assert_raises(FrozenError) { window.release(1) }
      assert_raises(FrozenError) { window.try_acquire(nil) }
      assert_raises(FrozenError) { window.release(0) }
      assert_equal 1, window.in_flight
    end

    def test_randomized_trace_matches_an_independent_balance_model
      random = Random.new(4209)
      window = CreditWindow.new(17)
      balance = 0

      500.times do
        units = random.rand(1..20)
        before = snapshot(window)
        if random.rand(2).zero?
          accepted = units <= 17 - balance
          assert_equal(accepted ? :acquired : :insufficient, window.try_acquire(units))
          balance += units if accepted
        else
          accepted = units <= balance
          assert_equal(accepted ? :released : :exceeds_in_flight, window.release(units))
          balance -= units if accepted
        end
        assert_equal before, snapshot(window) unless accepted
        assert_equal [17, balance, 17 - balance], snapshot(window)
      end
    end
  end
end
