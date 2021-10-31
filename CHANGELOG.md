# Changelog

## 0.7.0 (2021-10-25)

* Allow dynamic functions invocation through `Box<dyn FnRes>`. ([#3], [#4])

[#3]: https://github.com/azriel91/resman/issues/3
[#4]: https://github.com/azriel91/resman/pull/4

## 0.6.0 (2021-10-16)

* `Resources` debug impl prints out type names instead of type ID. ([#1], [#2])

[#1]: https://github.com/azriel91/resman/issues/1
[#2]: https://github.com/azriel91/resman/pull/2

## 0.5.0 (2021-08-08)

* `Resources` implements `Debug`.

## 0.4.0 (2021-08-01)

* `Resources` implements `new`, `with_capacity` and `capacity`.

## 0.3.0 (2021-06-28)

* Fix `Resources::entry` to accept typed `R`.

## 0.2.0 (2021-06-28)

* Use [`rt_map`] to back `Resources` map.

[`rt_map`]: https://github.com/azriel91/rt_map

## 0.1.0 (2021-04-11)

* Add `Resources` &ndash; runtime managed borrow map.