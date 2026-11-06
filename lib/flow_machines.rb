# frozen_string_literal: true

require_relative 'flow_machines/version'

# One implementation: Ruby delegates every policy decision to the Rust core.
# A missing extension is an installation error, not a request for a fallback.
require 'flow_machines_native/flow_machines_native'
