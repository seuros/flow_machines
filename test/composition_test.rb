# frozen_string_literal: true

require 'test_helper'

module FlowMachines
  class CompositionTest < Minitest::Test
    def test_caller_owned_queue_accounts_for_cancelled_work_before_resuming
      window = CreditWindow.new(4)
      control = Watermarks.new(1, 3)
      queue = []
      signals = []

      3.times do |id|
        assert_equal :acquired, window.try_acquire(1)
        queue << id
        signals << control.observe(queue.length)
      end
      assert_equal %i[unchanged unchanged pause], signals
      assert_equal :paused, control.state

      # Cancellation removes a queued item, then returns its capacity exactly once.
      queue.pop
      assert_equal :released, window.release(1)
      assert_equal :unchanged, control.observe(queue.length)
      queue.shift
      assert_equal :released, window.release(1)
      assert_equal :resume, control.observe(queue.length)
      assert_equal queue.length, window.in_flight
    end
  end
end
