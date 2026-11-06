# frozen_string_literal: true

require_relative 'lib/flow_machines/version'

Gem::Specification.new do |spec|
  spec.name = 'flow_machines'
  spec.version = FlowMachines::VERSION
  spec.authors = ['Abdelkader Boudih']
  spec.email = ['oss@seuros.com']
  spec.summary = 'Capacity credits and backpressure decisions for Ruby'
  spec.description = 'A native binding to the allocation-free flow-machines Rust core. ' \
                     'Tracks aggregate capacity and high/low-watermark producer signals; ' \
                     'the caller owns queues, credit ownership, and enforcement.'
  spec.homepage = 'https://github.com/seuros/flow_machines'
  spec.license = 'MIT'
  spec.required_ruby_version = '>= 4.0.0'

  spec.metadata['source_code_uri'] = spec.homepage
  spec.metadata['bug_tracker_uri'] = "#{spec.homepage}/issues"
  spec.metadata['rubygems_mfa_required'] = 'true'
  spec.metadata['cargo_crate_name'] = 'flow_machines_native'
  spec.metadata['cargo_manifest_path'] = 'ext/flow_machines_native/ffi/Cargo.toml'

  # The core README is included by Rust at compile time. It must ship with sources.
  spec.files = Dir.glob(%w[
                          lib/**/*.rb
                          sig/**/*.rbs
                          ext/**/*.{rb,rs,toml,md}
                          Cargo.toml
                          Cargo.lock
                          README.md
                          CHANGELOG.md
                          LICENSE.txt
                        ]).select { |path| File.file?(path) }
  spec.require_paths = ['lib']
  spec.extensions = ['ext/flow_machines_native/extconf.rb']
  spec.add_dependency 'rb_sys', '~> 0.9', '>= 0.9.130'
end
