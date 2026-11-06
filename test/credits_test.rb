# frozen_string_literal: true

require 'test_helper'

module FlowMachines
  class CreditsTest < Minitest::Test
    def test_starts_with_all_capacity_available
      window = CreditWindow.new(capacity: 3)

      assert_equal [3, 0, 3], [window.capacity, window.in_flight, window.available]
    end

    def test_acquisition_and_release_are_all_or_nothing
      window = CreditWindow.new(capacity: 3)

      assert window.try_acquire(2)
      refute window.try_acquire(2)
      assert_equal [2, 1], [window.in_flight, window.available]
      refute window.release(3)
      assert_equal 2, window.in_flight
      assert window.release(2)
      assert_equal [0, 3], [window.in_flight, window.available]
      refute window.release(1)
    end

    def test_full_u32_capacity_does_not_wrap
      window = CreditWindow.new(capacity: (2**32) - 1)

      assert window.try_acquire((2**32) - 1)
      refute window.try_acquire(1)
      assert_equal 0, window.available
      assert window.release((2**32) - 1)
      assert_equal((2**32) - 1, window.available)
    end

    def test_counts_are_positive_integers_without_coercion
      [nil, true, '2', 2.0, Float::NAN, Object.new].each do |value|
        assert_raises(TypeError) { CreditWindow.new(capacity: value) }
      end
      [0, -1, 2**32].each do |value|
        assert_raises(ArgumentError, RangeError) { CreditWindow.new(capacity: value) }
      end
      window = CreditWindow.new(capacity: 2)
      assert window.try_acquire(1)
      %i[try_acquire release].each do |operation|
        assert_raises(TypeError) { window.public_send(operation, 1.0) }
        assert_raises(ArgumentError) { window.public_send(operation, 0) }
        assert_raises(RangeError) { window.public_send(operation, -1) }
        assert_raises(RangeError) { window.public_send(operation, 2**32) }
      end
      assert_equal 1, window.in_flight
    end

    def test_rejects_missing_unknown_and_positional_constructor_arguments
      assert_raises(ArgumentError) { CreditWindow.new }
      assert_raises(ArgumentError) { CreditWindow.new(3) }
      assert_raises(ArgumentError) { CreditWindow.new(capacity: 3, typo: 1) }
    end

    def test_freezing_prevents_native_mutation
      window = CreditWindow.new(capacity: 2)
      assert window.try_acquire(1)
      window.freeze

      assert_raises(FrozenError) { window.try_acquire(1) }
      assert_raises(FrozenError) { window.release(1) }
      assert_equal 1, window.in_flight
    end

    def test_independent_windows_do_not_share_balances
      first = CreditWindow.new(capacity: 2)
      second = CreditWindow.new(capacity: 2)

      assert first.try_acquire(2)
      assert_equal 2, second.available
      assert_match(/CreditWindow capacity=2 in_flight=2 available=0/, first.inspect)
    end

    def test_randomized_trace_matches_an_independent_balance_model
      random = Random.new(4209)
      window = CreditWindow.new(capacity: 17)
      balance = 0

      500.times do
        units = random.rand(1..20)
        if random.rand(2).zero?
          accepted = units <= 17 - balance
          assert_equal accepted, window.try_acquire(units)
          balance += units if accepted
        else
          accepted = units <= balance
          assert_equal accepted, window.release(units)
          balance -= units if accepted
        end
        assert_equal [balance, 17 - balance], [window.in_flight, window.available]
      end
    end
  end
end
