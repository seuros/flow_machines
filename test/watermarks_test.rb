# frozen_string_literal: true

require 'test_helper'

module FlowMachines
  class WatermarksTest < Minitest::Test
    def test_edges_and_hysteresis_include_both_thresholds
      control = Watermarks.new(2, 5)

      assert_equal :running, control.state
      assert_equal :unchanged, control.observe(4)
      assert_equal :pause, control.observe(5)
      assert_equal :paused, control.state
      assert_equal :unchanged, control.observe(5)
      assert_equal :unchanged, control.observe(3)
      assert_equal :resume, control.observe(2)
      assert_equal :running, control.state
      assert_equal :unchanged, control.observe(0)
    end

    def test_zero_low_threshold_and_maximum_occupancy
      control = Watermarks.new(0, 1)

      assert_equal :pause, control.observe((2**32) - 1)
      assert_equal :unchanged, control.observe(1)
      assert_equal :resume, control.observe(0)
    end

    def test_invalid_thresholds_and_positional_arguments
      [[2, 2], [3, 2], [0, 0], [-1, 2], [0, 2**32]].each do |bounds|
        assert_raises(RangeError) { Watermarks.new(*bounds) }
      end
      assert_raises(ArgumentError) { Watermarks.new(1) }
      assert_raises(ArgumentError) { Watermarks.new(1, 2, 3) }
      assert_raises(ArgumentError) { Watermarks.new(low: 1, high: 2) }
      assert_raises(TypeError) { Watermarks.new(1.0, 2) }
      error = assert_raises(TypeError) { Watermarks.new(0, nil) }
      assert_equal 'high must be an Integer, not NilClass', error.message
      error = assert_raises(TypeError) { Watermarks.new({ low: 1 }, 2) }
      assert_equal 'low must be an Integer, not Hash', error.message
      error = assert_raises(RangeError) { Watermarks.new(-1, 2) }
      assert_equal 'low must be in 0..=4294967295', error.message
    end

    def test_only_the_core_operations_are_exposed
      assert_equal %i[observe state], Watermarks.public_instance_methods(false).sort
    end

    def test_invalid_observations_leave_state_unchanged
      control = Watermarks.new(1, 3)

      [nil, true, '3', 3.0, Float::INFINITY].each do |value|
        assert_raises(TypeError) { control.observe(value) }
      end
      [-1, 2**32].each do |value|
        assert_raises(RangeError) { control.observe(value) }
      end
      assert_equal :running, control.state
      assert_equal :pause, control.observe(3)
    end

    def test_freeze_guards_the_native_state
      control = Watermarks.new(1, 3).freeze

      assert_raises(FrozenError) { control.observe(3) }
      assert_equal :running, control.state
    end

    def test_no_instance_exists_without_the_core
      assert_raises(TypeError) { Watermarks.allocate }
      assert_raises(TypeError) { FlowMachines::CreditWindow.allocate }
    end

    def test_randomized_trace_matches_a_reference_state_machine
      random = Random.new(4210)
      control = Watermarks.new(2, 7)
      paused = false

      500.times do
        occupancy = random.rand(0..10)
        next_paused = paused ? occupancy > 2 : occupancy >= 7
        signal = next_paused == paused ? :unchanged : (next_paused ? :pause : :resume)
        assert_equal signal, control.observe(occupancy)
        assert_equal(next_paused ? :paused : :running, control.state)
        paused = next_paused
      end
    end
  end
end
