# Changelog

## Unreleased

### Added

- Add support for downloading attachments ([#122])
- Add release build for `aarch64-unknown-musl` ([#126])
- Show qrcode in terminal instead of PNG viewer ([#128])
- Document key bindings and packages ([#130])

## Changed

- Add cursor tracking and multiline input navigation ([#131])
- New visual style for receipts ([#135], [#142], [#144])

### Fixed

- Bug: infinite loop while skkiping words on input box ([#129], [#131])

[#122]: https://github.com/boxdot/gurk-rs/pull/122
[#126]: https://github.com/boxdot/gurk-rs/pull/126
[#128]: https://github.com/boxdot/gurk-rs/pull/128
[#129]: https://github.com/boxdot/gurk-rs/pull/129
[#130]: https://github.com/boxdot/gurk-rs/pull/130
[#131]: https://github.com/boxdot/gurk-rs/pull/131
[#135]: https://github.com/boxdot/gurk-rs/pull/135
[#142]: https://github.com/boxdot/gurk-rs/pull/142

## 0.2.3

### Added

- Add help panel ([#107])
- Basic multiline editing support ([#109])
- Add search bar + receipt notifications ([#114])

[#107]: https://github.com/boxdot/gurk-rs/pull/107
[#109]: https://github.com/boxdot/gurk-rs/pull/109
[#114]: https://github.com/boxdot/gurk-rs/pull/114

### Fixed

- Fix linking device ([#101], [#102])
- Fix and isolate message receipts ([#116])

[#101]: https://github.com/boxdot/gurk-rs/pull/101
[#102]: https://github.com/boxdot/gurk-rs/pull/102
[#116]: https://github.com/boxdot/gurk-rs/pull/116

## 0.2.2

### Added

- Add basic emojis and reactions support. ([#91])
- Open URL (if any) in selected message on Enter when input is empty. ([#99])
- Send attachments from file:// paths (#[100]).

### Fixed

[#91]: https://github.com/boxdot/gurk-rs/pull/91
[#99]: https://github.com/boxdot/gurk-rs/pull/99
[#100]: https://github.com/boxdot/gurk-rs/pull/100

## 0.2.1

### Fixed

- Fix formatting of phone number and update user name on start. ([#78])
- Fix an overflow error and crash by adding a subtraction check. ([#88])

[#78]: https://github.com/boxdot/gurk-rs/pull/78
[#88]: https://github.com/boxdot/gurk-rs/pull/88

## 0.2.0

The highlight of this release is the usage of the native implementation of the Signal client
protocol via [presage]. This removes the dependency on [signal-cli] and makes `gurk` fully
standalone. For more defails, see [#41].

⚠️ This release has a breaking change of the data storage.

### Added

- Ctrl+J/K for channel up/down navigation ([#74])
- Added option to disable looping back when scrolling through messages. ([#72])
- Allow inter-word navigation with Alt/Ctrl+←→ ([#66])
- Handle reactions and show them as suffix of messages. ([#53])
- Keyboard shortcuts for word navigation ([#38])
- Scrolling messages ([#21])
- Mouse navigation of channels ([#24])
- New message notifications using notify-rust ([#19])

### Changed

- Change quoted reply-to text to a darker gray. ([#73])
- 🦀 Port to [presage]: native implementation of Signal client. ([#41])

### Fixed

- Fix init of data file by adding creation of default when none exists ([#48])
- Use local time zone when rendering time. ([#46])

[#19]: https://github.com/boxdot/gurk-rs/pull/19
[#24]: https://github.com/boxdot/gurk-rs/pull/24
[#21]: https://github.com/boxdot/gurk-rs/pull/21
[#38]: https://github.com/boxdot/gurk-rs/pull/38
[#41]: https://github.com/boxdot/gurk-rs/pull/41
[#46]: https://github.com/boxdot/gurk-rs/pull/46
[#48]: https://github.com/boxdot/gurk-rs/pull/48
[#53]: https://github.com/boxdot/gurk-rs/pull/53
[#66]: https://github.com/boxdot/gurk-rs/pull/66
[#72]: https://github.com/boxdot/gurk-rs/pull/72
[#73]: https://github.com/boxdot/gurk-rs/pull/73
[#74]: https://github.com/boxdot/gurk-rs/pull/74
[presage]: https://github.com/whisperfish/presage

## 0.1.1 (Oct 1, 2020)

### Added

- Fix cli linking to phone instruction link. ([#13])
- Respect `XDG_CONFIG_HOME` and `XDG_DATA_HOME`. ([#5])

### Fixed

- Invalid handling of empty channels list ([#7])

[#5]: https://github.com/boxdot/gurk-rs/pull/5
[#7]: https://github.com/boxdot/gurk-rs/pull/7
[#13]: https://github.com/boxdot/gurk-rs/pull/13

## 0.1.0 (Aug 2, 2020)

- Initial release based on [signal-cli]

[signal-cli]: https://github.com/AsamK/signal-cli
