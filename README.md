# circleci-junit-fix

CircleCI supports rendering test failures through the use of [JUnit files](https://llg.cubic.org/docs/junit/), but it only supports a subset of features.

One such feature is failure test outputs. The spec suggests that `stdout` and `stderr` contents be captured and added to the `<system-out>` and `<system-err>` elements respectively.
CircleCI instead only reads the data from the `<failure message="...">` attribute only.

## Installation

### With Cargo

```sh
cargo install circleci-junit-fix --locked --version 0.2.0
```

### With Prebuilt Binary

```sh
curl -sSL https://github.com/conradludgate/circleci-junit-fix/releases/download/v0.2.0/circleci-junit-fix-v0.2.0-x86_64-unknown-linux-gnu.tar.gz | tar -xz --directory=/usr/bin
```

## Usage

```sh
cat path/to/junit/report.xml | circleci-junit-fix > fixed-report.xml
```
