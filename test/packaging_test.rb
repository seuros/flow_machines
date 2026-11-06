# frozen_string_literal: true

require 'test_helper'
require 'fileutils'
require 'open3'
require 'tmpdir'

module FlowMachines
  class PackagingTest < Minitest::Test
    def test_native_state_survives_compaction_and_cannot_be_implicitly_copied
      window = CreditWindow.new(capacity: 3)
      control = Watermarks.new(low: 1, high: 3)
      assert window.try_acquire(3)
      assert_equal :pause, control.observe(3)

      GC.compact

      assert_equal 3, window.in_flight
      assert control.paused?
      [window, control].each do |object|
        assert_raises(TypeError) { object.dup }
        assert_raises(TypeError) { object.clone }
      end
      assert_nil window.method(:try_acquire).source_location
      assert_nil control.method(:observe).source_location
    end

    def test_a_missing_extension_raises_load_error_instead_of_using_a_fallback
      Dir.mktmpdir('flow-loader-') do |directory|
        FileUtils.cp_r(File.expand_path('../lib/flow_machines', __dir__), directory)
        FileUtils.cp(File.expand_path('../lib/flow_machines.rb', __dir__), directory)
        code = <<~RUBY
          begin
            require 'flow_machines'
            abort 'unexpected fallback'
          rescue LoadError => error
            abort error.message unless error.path == 'flow_machines_native/flow_machines_native'
          end
        RUBY
        _stdout, stderr, status = Open3.capture3(
          { 'RUBYOPT' => nil, 'RUBYLIB' => nil },
          RbConfig.ruby, '--disable-gems', "-I#{directory}", '-e', code
        )
        assert status.success?, stderr
      end
    end
  end
end
