# Changelog

## unreleased

* ***Breaking:*** Change `Resources::remove::<R>` to return `R`.
* Add `Resources::try_remove::<R>` to return `Result<R, ResourceFetchError>`.
* Add `Resources::into_inner()`.

## 0.16.1 (2023-04-06)

* Panic when `Resources::insert_raw` is called with mismatching `TypeId` and `Box<T>`. ([#11])

[#11]: https://github.com/azriel91/resman/pull/11

## 0.16.0 (2022-12-26)

* Add `Resources::insert_raw`. ([#9])
* Add `Resource::type_id`. ([#9])

[#9]: https://github.com/azriel91/resman/pull/9

## 0.15.0 (2022-07-05)

* Derive `Deref` and `DerefMut` for `Resources`. ([#7])

[#7]: https://github.com/azriel91/resman/pull/7

## 0.14.0 (2022-05-26)

* Update `fn_meta` to `0.7.0`.
* Implement `FnMeta` for `dyn FnRes`.
* `fn_meta` is **not** automatically enabled (reverts change from `0.13.0`).

## 0.13.0 (2022-05-23)

* Update `fn_meta` to `0.6.0`.
* `fn_meta` is automatically enabled if any of the `fn_res*` features are enabled.

## 0.12.0 (2022-05-22)

* Update `fn_meta` to `0.5.0`.

## 0.11.0 (2021-11-14)

* Add `"fn_res_mut"` feature, gating `FnResMut` and `IntoFnResMut`.
* Add `"fn_res_once"` feature, gating `FnResOnce` and `IntoFnResOnce`.

## 0.10.0 (2021-11-07)

* Feature gate 7 and 8 arguments behind `"high_arg_count"` feature.

## 0.9.0 (2021-11-06)

* Add `"fn_meta"` feature which requires `FnRes: FnMeta`.
* Implement `FnRes` for `Box<T>` where `T: FnRes`.

## 0.8.0 (2021-10-31)

* Support `FnRes` for functions with zero arguments.
* Update to Rust 2021 edition.

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
