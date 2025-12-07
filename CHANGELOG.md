# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-12-07

### Added

- GitHub Actions CI workflow
- Demo GIF for README

### Changed

- Promoted to stable 1.0.0 release

## [0.1.0] - 2024-12-07

### Added

- Initial release
- Big text rendering using tui-big-text with three font scales
- Automatic text wrapping to fit terminal width
- Smooth scrolling from bottom to top
- Multiple input methods: file, stdin pipe, or interactive editor
- Keyboard controls: pause, speed adjustment, manual scroll, reset
- Customizable colors (named colors and hex codes)
- Configuration file support (~/.config/tui_prompter/config.toml)
- Man page generation
- justfile for easy installation
