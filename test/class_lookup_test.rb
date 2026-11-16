# frozen_string_literal: true

require 'minitest/autorun'
require 'open3'
require 'rbconfig'

# The classes of the values the extension wraps (CreditWindow, Watermarks)
# are fixed when it loads. Nothing a caller does later, redefining a constant
# or `Object.const_get`, reaches a controller being built. Each case runs in
# a fresh process: the class is fixed once per process, at load.
class ClassLookupTest < Minitest::Test
  LIB = File.expand_path('../lib', __dir__)

  PRELUDE = <<~RUBY
    require 'flow_machines'
    F = FlowMachines
    CreditWindow = F::CreditWindow
    Watermarks = F::Watermarks
  RUBY

  # Runs `body` after PRELUDE in a new Ruby; answers its last line of output.
  def isolated(body)
    out, err, status = Open3.capture3(RbConfig.ruby, '-I', LIB, '-e', PRELUDE + body)
    assert status.success?, "exited #{status.exitstatus}: #{err}"
    out.lines.last&.chomp
  end

  # Bug: a CreditWindow constant redefined before the first window made it
  # an instance of the new class, which has none of the window's methods.
  def test_a_redefined_credit_window_constant_does_not_change_the_class
    assert_equal 'true acquired 1', isolated(<<~RUBY)
      F.send(:remove_const, :CreditWindow)
      F.const_set(:CreditWindow, Class.new)
      window = CreditWindow.new(3)
      puts [window.instance_of?(CreditWindow), window.try_acquire(2), window.available].join(' ')
    RUBY
  end

  # Bug: the same for Watermarks.
  def test_a_redefined_watermarks_constant_does_not_change_the_class
    assert_equal 'true pause paused', isolated(<<~RUBY)
      F.send(:remove_const, :Watermarks)
      F.const_set(:Watermarks, Class.new)
      control = Watermarks.new(2, 5)
      puts [control.instance_of?(Watermarks), control.observe(5), control.state].join(' ')
    RUBY
  end

  # Bug: building the first controller ran `Object.const_get`, Ruby code in
  # the middle of a native call. No Ruby runs while one is built now.
  def test_building_a_controller_runs_no_const_get
    assert_equal '0 CreditWindow Watermarks', isolated(<<~RUBY)
      hits = 0
      Object.define_singleton_method(:const_get) do |*args|
        hits += 1
        super(*args)
      end
      window = CreditWindow.new(3)
      control = Watermarks.new(2, 5)
      puts [hits, window.class.name.split('::').last, control.class.name.split('::').last].join(' ')
    RUBY
  end

  # Bug: a redefined `Object.const_get` that raised hit an `unwrap` in the
  # extension: an uncatchable fatal, and the process exited.
  def test_a_raising_const_get_does_not_kill_the_process
    assert_equal 'acquired pause', isolated(<<~RUBY)
      Object.define_singleton_method(:const_get) { |*| raise IOError, 'lookup ran in a native call' }
      window = CreditWindow.new(3)
      control = Watermarks.new(2, 5)
      puts [window.try_acquire(1), control.observe(5)].join(' ')
    RUBY
  end
end
