# dis(playish)

## Goal

Trait extensions and asserts to use with
[proc-macro2-diagnostics](https://crates.io/crates/proc-macro2-diagnostics)
([SergioBenitez/proc-macro2-diagnostics](https://github.com/SergioBenitez/proc-macro2-diagnostics/)).

## Extension method naming convention

- `**_with` takes a function/closure that generates `Display` content; it invokes the closure and
  prepends that generated content in front of the existing content already stored/coming from `self`
  (with an extra space between them).
- `**_and` sets an `extra`.
- `**_with_and` takes a closure, invokes it, and prepends that generated content `Display` content,
  and sets an `extra`.

`bool` and `Option` are special: They don't carry any error message. So they have ONLY methods that
have suffixes `_with` or `with_and` - but not suffixless, since we have to specify/generate the
content.

## no_std subset

Default featureset is `no_std`-compatible. It's limited, but it can be used even without
`proc-macro2-diagnostics`. To use `proc-macro2-diagnostics` functionality (`Diagnostic` etc.),
enable cargo feature `proc-macro2-diagnostics`.
