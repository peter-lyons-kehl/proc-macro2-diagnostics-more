# proc-macro2-diagnostics-more

## Goal

Trait extensions and asserts to use with
[proc-macro2-diagnostics](https://crates.io/crates/proc-macro2-diagnostics)
([SergioBenitez/proc-macro2-diagnostics](https://github.com/SergioBenitez/proc-macro2-diagnostics/)).

## Extension method naming convention

- **_with takes a `String` message and prepends it in front of the existing error (with an extra
  space between them).
- **_for adds a `Span`.
- **_with_for takes and prepends a `String` message, and adds a `Span`.

`bool` and `Option` are special: They don't carry any error message. So they have ONLY methods that
have suffixes `_with` or `with_for` - but not suffixless, since we have to add the message.

## no_std subset

Default featureset is no_std-only. It's limited, but it can be used without
`proc-macro2-diagnostics`. To use `proc-macro2-diagnostics` functionality (`Diagnostic` etc.),
enable cargo feature `proc-macro2-diagnostics`.
