# 0.4.0

- Changed: the plugin now runs in the `PreUpdate` schedule, instead of the `Update` schedule.
- Changed: all systems have been made public. This will allows users to rearrange the plugin for
  their needs, either running in another schedule, or building something entirely custom.

# 0.3.0

- Changed: relaxed bounds to support static `FnMut` closures for `On` methods instead of only `fn`
- Added: new `event_listener` example to guide users through how to use the supplied event listener methods.
- Fixed: prelude now exports `ListenerInput`