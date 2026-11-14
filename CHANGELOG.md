# Changelog

## [0.3.2](https://github.com/seuros/flow_machines/compare/flow_machines/v0.3.1...flow_machines/v0.3.2) (2026-11-14)


### Bug Fixes

* **deps:** require matryona-machines &gt;= 0.4 ([40763b7](https://github.com/seuros/flow_machines/commit/40763b7164a56dcc37442e6820f870448924d7f4))

## [0.3.1](https://github.com/seuros/flow_machines/compare/flow_machines/v0.3.0...flow_machines/v0.3.1) (2026-11-14)


### Bug Fixes

* **deps:** require matryona-machines &gt;= 0.3 with a names_only refusal table and symbols! tables ([3230754](https://github.com/seuros/flow_machines/commit/3230754ffbe7ee763b01d597e96d8feec0a68b9d))

## [0.3.0](https://github.com/seuros/flow_machines/compare/flow_machines/v0.2.1...flow_machines/v0.3.0) (2026-11-14)


### ⚠ BREAKING CHANGES

* adopt matryona-machines 0.2 (shared converters, frozen-first)

### Features

* adopt matryona-machines 0.2 (shared converters, frozen-first) ([63ccf35](https://github.com/seuros/flow_machines/commit/63ccf3565543bdf6634015c188ceb6eda1e09af0))

## [0.2.1](https://github.com/seuros/flow_machines/compare/flow_machines/v0.2.0...flow_machines/v0.2.1) (2026-11-14)


### Bug Fixes

* name the offending argument in every raised message and undefine allocate on native classes ([ebf4d13](https://github.com/seuros/flow_machines/commit/ebf4d13bdb180fbc58112ea7a78fd82d18345fc8))

## [0.2.0](https://github.com/seuros/flow_machines/compare/flow_machines/v0.1.4...flow_machines/v0.2.0) (2026-11-13)


### ⚠ BREAKING CHANGES

* converted to the fleet conventions. One exhaustive Refusal per crate, Result from every operation that can refuse, a refusal changes nothing, provable idempotent repeats are honoured, Ruby mirrors the core. No compatibility aliases. try_acquire and release return Result<(), Refusal>; in Ruby they answer symbols, which are all truthy.

### Features

* answer credit refusals as Refusal, exhaustive enums, positional Ruby constructors ([7f96f70](https://github.com/seuros/flow_machines/commit/7f96f70933ea1e07dc00c1c160f670c03423fef2))

## [0.1.4](https://github.com/seuros/flow_machines/compare/flow_machines/v0.1.3...flow_machines/v0.1.4) (2026-11-06)


### Bug Fixes

* crates.io keywords and categories for the core ([14830b5](https://github.com/seuros/flow_machines/commit/14830b52d44337c74302591f4c73ebcbe8bac3e4))

## [0.1.3](https://github.com/seuros/flow_machines/compare/flow_machines/v0.1.2...flow_machines/v0.1.3) (2026-11-06)


### Bug Fixes

* keep test sources out of the packaged gem and crate ([3c9e544](https://github.com/seuros/flow_machines/commit/3c9e544f3b2140a179455b34236fd0f2ab9747a7))

## [0.1.2](https://github.com/seuros/flow_machines/compare/flow_machines/v0.1.1...flow_machines/v0.1.2) (2026-11-06)


### Bug Fixes

* cross builds no longer compile for the host ([5a0009f](https://github.com/seuros/flow_machines/commit/5a0009fdef9267dc83ca7704b1e9f0f6373d3a5a))

## [0.1.1](https://github.com/seuros/flow_machines/compare/flow_machines-v0.1.0...flow_machines/v0.1.1) (2026-11-06)


### Features

* initial extraction ([e087042](https://github.com/seuros/flow_machines/commit/e0870427dde8239fc30fc2fa10e6e204ebab9289))
