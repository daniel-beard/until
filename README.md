# Until

Small utility for exiting a long running shell command when a line in stdout matches a given regex.
When an early exit occurs, this util returns a `0` exitCode. Otherwise, it respects the output of the sub-command.

## Usage

```bash
until REGEX COMMAND [COMMAND_OPT, ...]

until 'Linking MyApp' xcodebuild -workspace MyApp.xcworkspace -scheme MyApp clean build
```

## License

MIT
