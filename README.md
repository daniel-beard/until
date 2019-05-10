# until

Small utility for exiting a long running shell command when a line in stdout matches a given regex.
When an early exit occurs, this util returns a `0` exitCode. Otherwise, it respects the output of the child command.

This utility forwards both `stderr` and `stdout` from the child command.

## Usage

```bash
until REGEX COMMAND [COMMAND_OPT, ...]

until '^Ld .* MyApp' xcodebuild -workspace MyApp.xcworkspace -scheme MyApp clean build
```

## License

MIT
