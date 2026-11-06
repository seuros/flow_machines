# frozen_string_literal: true

require 'bundler/gem_tasks'
require 'minitest/test_task'
require 'shellwords'
require 'rb_sys/extensiontask'

Minitest::TestTask.create(:minitest) do |task|
  task.libs << 'test'
  task.libs << 'lib'
  task.test_globs = ['test/**/*_test.rb']
end

GEMSPEC = Gem::Specification.load('flow_machines.gemspec')

RbSys::ExtensionTask.new('flow_machines_native', GEMSPEC) do |extension|
  extension.lib_dir = 'lib/flow_machines_native'
  extension.tmp_dir = 'tmp/rb_sys'
end

desc 'Compile the extension and run the Ruby boundary tests'
task test: :compile do
  sh 'padded_cell', *Shellwords.split(ENV.fetch('TESTOPTS', ''))
end

namespace :native do
  desc 'Compile the native extension in release mode'
  task build: ['rb_sys:env:release', 'compile']
end

desc 'Test the standalone Rust core'
task :cargo do
  sh 'cargo', 'test', '-p', 'flow-machines'
end

desc 'Lint the core and native binding'
task :clippy do
  sh 'cargo', 'clippy', '--workspace', '--all-targets', '--', '-D', 'warnings'
end

desc 'Validate Ruby type signatures'
task :rbs do
  sh 'rbs', '-I', 'sig', 'validate'
end

task default: %i[test cargo rbs]

Dir[File.join(__dir__, 'tasks/*.rake')].sort.each { |f| load f }
