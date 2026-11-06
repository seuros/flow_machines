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

SUPPORTED_NATIVE_PLATFORMS = %w[
  arm64-darwin
  x86_64-darwin
  aarch64-linux
  x86_64-linux
  x86_64-linux-musl
].freeze

native_ext = RbSys::ExtensionTask.new('flow_machines_native', GEMSPEC) do |extension|
  extension.lib_dir = 'lib/flow_machines_native'
  extension.tmp_dir = 'tmp/rb_sys'
  extension.cross_platform = SUPPORTED_NATIVE_PLATFORMS if ENV.key?('RUBY_TARGET')
end

# A single-version cross build stages its binary at the host binary's path,
# which the gem package depends on, so packaging would also compile for the
# host and fail where the host linker can't target the platform.
if ENV.key?('RUBY_TARGET')
  host_binary = "#{native_ext.lib_dir}/#{native_ext.name}.#{RbConfig::CONFIG['DLEXT']}"
  Rake::Task[host_binary].clear_prerequisites if Rake::Task.task_defined?(host_binary)
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
