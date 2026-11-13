# Changelog

## [0.3.0](https://github.com/seuros/flow_machines/compare/flow-machines-v0.2.2...flow-machines-v0.3.0) (2026-11-13)


### ⚠ BREAKING CHANGES

* converted to the fleet conventions. One exhaustive Refusal per crate, Result from every operation that can refuse, a refusal changes nothing, provable idempotent repeats are honoured, Ruby mirrors the core. No compatibility aliases. try_acquire and release return Result<(), Refusal>; in Ruby they answer symbols, which are all truthy.

### Features

* answer credit refusals as Refusal, exhaustive enums, positional Ruby constructors ([7f96f70](https://github.com/seuros/flow_machines/commit/7f96f70933ea1e07dc00c1c160f670c03423fef2))

## [0.2.2](https://github.com/seuros/flow_machines/compare/flow-machines-v0.2.1...flow-machines-v0.2.2) (2026-11-06)


### Bug Fixes

* crates.io keywords and categories for the core ([14830b5](https://github.com/seuros/flow_machines/commit/14830b52d44337c74302591f4c73ebcbe8bac3e4))

## [0.2.1](https://github.com/seuros/flow_machines/compare/flow-machines-v0.2.0...flow-machines-v0.2.1) (2026-11-06)


### Bug Fixes

* keep test sources out of the packaged gem and crate ([3c9e544](https://github.com/seuros/flow_machines/commit/3c9e544f3b2140a179455b34236fd0f2ab9747a7))

## [0.2.0](https://github.com/seuros/flow_machines/compare/flow-machines-v0.1.0...flow-machines-v0.2.0) (2026-11-06)


### Features

* initial extraction ([e087042](https://github.com/seuros/flow_machines/commit/e0870427dde8239fc30fc2fa10e6e204ebab9289))
