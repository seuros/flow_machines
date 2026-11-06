# frozen_string_literal: true

require_relative 'release'

namespace :gems do
  desc 'Fetch and checksum-verify GitHub release gems into pkg/ (default: current version)'
  task :fetch, [:tag] do |_task, args|
    FlowMachinesRelease.run { FlowMachinesRelease.fetch(args[:tag]) }
  end

  desc 'Fetch verified release gems and push every platform to RubyGems (resumable)'
  task :push_all, [:tag] do |_task, args|
    FlowMachinesRelease.run { FlowMachinesRelease.push_all(args[:tag]) }
  end

  desc 'Alias for gems:push_all'
  task :push, [:tag] do |_task, args|
    FlowMachinesRelease.run { FlowMachinesRelease.push_all(args[:tag]) }
  end
end
