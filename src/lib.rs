#![cfg_attr(not(feature = "proc-macro2-diagnostics"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::string::String;

use core::any::Any;
use core::fmt::{self, Display, Formatter};

#[cfg(feature = "proc-macro2-diagnostics")]
use proc_macro2::Span;
#[cfg(feature = "proc-macro2-diagnostics")]
use proc_macro2_diagnostics::{Diagnostic, Level};

#[cfg(feature = "proc-macro2-diagnostics")]
pub type MacroResult<T> = Result<T, Diagnostic>;

#[cfg(feature = "alloc")]
pub type MacroDeepResult<T, M = String> = Result<T, DeepDiagnostic<M>>;
#[cfg(not(feature = "alloc"))]
pub type MacroDeepResult<T, M> = Result<T, DeepDiagnostic<M>>;

// @TODO SEAL!
#[cfg(feature = "alloc")]
#[derive(Clone, Debug)]
pub struct DeepDiagnostic<M: Display = String> {
    #[cfg(feature = "proc-macro2-diagnostics")]
    level: Level,

    message: M,
}
#[cfg(not(feature = "alloc"))]
#[derive(Clone, Debug)]
pub struct DeepDiagnostic<M: Display> {
    #[cfg(feature = "proc-macro2-diagnostics")]
    level: Level,

    message: M,
}
impl<M: Display> DeepDiagnostic<M> {
    // @TODO macro_rules and also generate: pub fn warning, note, help
    #[cfg(feature = "alloc")]
    pub fn new_error<T: Into<M>>(message: T) -> Self {
        Self {
            #[cfg(feature = "proc-macro2-diagnostics")]
            level: Level::Error,

            message: message.into(),
        }
    }

    pub fn into_msg(self) -> M {
        self.message
    }

    // @TODO if implemented in proc_macro2_diagnostics, make it accept MultiSpan:
    //
    // pub fn spanned<S: MultiSpan>(self, s: S) -> Diagnostic
    #[cfg(feature = "proc-macro2-diagnostics")]
    pub fn spanned(self, span: Span) -> Diagnostic {
        Diagnostic::spanned(span, self.level, self.to_string())
    }
}
impl<M: Display> Display for DeepDiagnostic<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}
#[cfg(feature = "alloc")]
impl DeepDiagnostic<String> {
    pub fn to_string_move(self) -> String {
        self.message
    }
}
#[cfg(feature = "alloc")]
impl<M: Display + 'static> DeepDiagnostic<M> {
    pub fn to_string_move_maybe(mut self) -> String {
        let self_mut = &mut self as &mut dyn Any;
        match self_mut.downcast_mut::<DeepDiagnostic<String>>() {
            Some(as_string_based) => core::mem::take(&mut as_string_based.message),
            None => self.to_string(),
        }
    }
}
impl<M: Display> From<M> for DeepDiagnostic<M> {
    /// Move-and-construct/convert. If using `proc-macro2-diagnostics`, then [DeepDiagnostic::level]
    /// will be set to [Level::Error].
    fn from(message: M) -> Self {
        Self {
            #[cfg(feature = "proc-macro2-diagnostics")]
            level: Level::Error,
            message,
        }
    }
}

/// Intentionally not public - used to indicate a sealed trait.
enum SealedTraitFunParam {}
//--------
/// Similar (but only partially) to [enum_dispatch](https://crates.io/crates/enum_dispatch) and
/// [enum_delegate](https://crates.io/crates/enum_delegate).
///
/// ### dyn-compatible only
///
/// Only for `dyn`-compatible traits.
///
/// ### One trait only
///
/// If you'd like to implement multiple traits, define a join trait that inherits all of them, and
/// blanket implement it:
///
/// ```rust
/// # use core::fmt::Display;
/// pub trait DisplayAndError: Display + core::error::Error {}
/// impl<T: Display + core::error::Error> DisplayAndError for T {}
/// ```
pub mod by_dyn {
    /*macro_rules! variants {
        ($vis:vis $enum:ident : $trait:path:ty
            (
                $(
                    $constructor:ident -> $variant:ident
                )*
            )
            (
                //@TODO assert that trit is dyn-compatible - needed even if we use `impl`
                $(
                        (
                            fn $method:ident(
                                // leading `mut` is NOT needed - the variable itself will
                                // never be modified, because we're just forwarding the call.
                                //
                                // Exactly ONE of the following three should match.
                                $( &mut $mut_self:ident )? // &mut self
                                $( &$ref_self:ident )? // &self
                                $(  $val_self:ident )? // self

                                ( $other_args:tt )*
                            ) -> $result:ty
                        )
                        (
                            ( $method_invocation:tt )*
                        )
                ),+
            )
        ) => {};
    }*/
}

pub mod ext_all {
    #[cfg(feature = "alloc")]
    use crate::DeepDiagnostic;
    #[cfg(feature = "proc-macro2-diagnostics")]
    use crate::MacroResult;
    use crate::{MacroDeepResult, SealedTraitFunParam};

    #[cfg(feature = "alloc")]
    use alloc::format;
    #[cfg(feature = "alloc")]
    use alloc::string::{String, ToString};
    use core::fmt::Debug;

    #[cfg(feature = "proc-macro2-diagnostics")]
    use proc_macro2::Span;
    #[cfg(feature = "proc-macro2-diagnostics")]
    use proc_macro2_diagnostics::{Diagnostic, SpanDiagnosticExt as _};

    #[cfg(feature = "proc-macro2-diagnostics")]
    pub trait MacroDeepResultExt<T> {
        // @TODO if implemented in proc_macro2_diagnostics, make it accept MultiSpan.
        /// Add the given [Span], and transform to [MacroResult].
        fn spanned(self, span: Span) -> MacroResult<T>;
        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }
    #[cfg(feature = "proc-macro2-diagnostics")]
    impl<T> MacroDeepResultExt<T> for MacroDeepResult<T> {
        fn spanned(self, span: Span) -> MacroResult<T> {
            self.map_err(|deep_err| deep_err.spanned(span))
        }
        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    pub struct Auto;
    impl AsRef<usize> for Auto {
        fn as_ref(&self) -> &usize {
            todo!()
        }
    }
    impl core::ops::Deref for Auto {
        type Target = usize;
        fn deref(&self) -> &Self::Target {
            todo!()
        }
    }
    fn _auto_convert(a: &Auto) -> usize {
        **a
    }

    #[cfg(feature = "alloc")]
    pub trait IntoStringExt: Into<String> {
        fn into_error(self) -> DeepDiagnostic;
        /// Param `f` is a closure/function that returns a [String]. This method returns
        /// [DeepDiagnostic] that [String], with a space appended, and ONLY THEN [String]
        /// generated by `self.into()`.
        ///
        /// We do NOT need alternative methods that would save allocating a [String], like `fn
        /// into_error_with_str(self, &str)`, because errors, and especially proc macro errors,
        /// are on a cold execution path.
        fn into_error_with<F: Fn() -> String>(self, f: F) -> DeepDiagnostic;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn into_error_at(self, span: Span) -> Diagnostic;
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn into_error_with_at<F: Fn() -> String>(self, f: F, span: Span) -> Diagnostic;

        // Sealing is not really necessary, because we have a blanket impl that covers any and
        // all eligible types, so no other types can implement this trait.
        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }
    #[cfg(feature = "alloc")]
    impl<T: Into<String>> IntoStringExt for T {
        fn into_error(self) -> DeepDiagnostic {
            DeepDiagnostic::new_error(self.into())
        }
        fn into_error_with<F: Fn() -> String>(self, f: F) -> DeepDiagnostic {
            let mut s = f();
            s.push(' ');
            s.push_str(&self.into());
            DeepDiagnostic::new_error(s)
        }

        /// Convenience function: same as into_error().span(span)
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn into_error_at(self, span: Span) -> Diagnostic {
            span.error(self)
        }
        /// Convenience function: same as into_error_with(f).span(span)
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn into_error_with_at<F: Fn() -> String>(self, f: F, span: Span) -> Diagnostic {
            let mut s = f();
            s.push(' ');
            s.push_str(&self.into());
            span.error(s)
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    #[cfg(feature = "alloc")]
    pub trait ResultErrIntoStringExt<T> {
        //@TODO was IntoStringResultExt
        fn map_error_into(self) -> MacroDeepResult<T>;
        fn map_error_into_with<F: Fn() -> String>(self, f: F) -> MacroDeepResult<T>;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_into_at(self, span: Span) -> MacroResult<T>;
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_into_with_at<F: Fn() -> String>(self, f: F, span: Span) -> MacroResult<T>;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }
    #[cfg(feature = "alloc")]
    impl<T, E: Into<String>> ResultErrIntoStringExt<T> for Result<T, E> {
        fn map_error_into(self) -> MacroDeepResult<T> {
            self.map_err(|e| DeepDiagnostic::new_error(e))
        }
        fn map_error_into_with<F: Fn() -> String>(self, f: F) -> MacroDeepResult<T> {
            self.map_err(|e| {
                let mut s = f();
                s.push(' ');
                s.push_str(&e.into());
                DeepDiagnostic::new_error(s)
            })
        }

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_into_at(self, span: Span) -> MacroResult<T> {
            self.map_err(|e| span.error(e))
        }
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_into_with_at<F: Fn() -> String>(self, f: F, span: Span) -> MacroResult<T> {
            self.map_err(|e| {
                let mut s = f();
                s.push(' ');
                s.push_str(&e.into());
                span.error(s)
            })
        }
        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    pub trait OptionOrBoolExt<T> {
        #[cfg(feature = "alloc")]
        fn ok_or_error_with<F: Fn() -> String>(self, f: F) -> MacroDeepResult<T>;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn ok_or_error_with_at<F: Fn() -> String>(self, f: F, span: Span) -> MacroResult<T>;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }
    impl<T> OptionOrBoolExt<T> for Option<T> {
        #[cfg(feature = "alloc")]
        fn ok_or_error_with<F: Fn() -> String>(self, f: F) -> MacroDeepResult<T> {
            self.ok_or_else(|| DeepDiagnostic::new_error(f()))
        }

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn ok_or_error_with_at<F: Fn() -> String>(self, f: F, span: Span) -> MacroResult<T> {
            self.ok_or_else(|| span.error(f()))
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }
    impl OptionOrBoolExt<()> for bool {
        #[cfg(feature = "alloc")]
        fn ok_or_error_with<F: Fn() -> String>(self, f: F) -> MacroDeepResult<()> {
            // bool::ok_or_else is unstable: https://github.com/rust-lang/rust/issues/142748
            if self {
                Ok(())
            } else {
                Err(DeepDiagnostic::new_error(f()))
            }
        }

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn ok_or_error_with_at<F: Fn() -> String>(self, f: F, span: Span) -> MacroResult<()> {
            if self {
                Ok(())
            } else {
                Err(span.error(f()))
            }
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    #[cfg(feature = "alloc")]
    pub trait ToStringExt: ToString {
        fn to_error(&self) -> DeepDiagnostic;
        fn to_error_with<F: Fn() -> String>(&self, f: F) -> DeepDiagnostic;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn to_error_at(&self, span: Span) -> Diagnostic;
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn to_error_with_at<F: Fn() -> String>(&self, f: F, span: Span) -> Diagnostic;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }
    #[cfg(feature = "alloc")]
    impl<T: ToString> ToStringExt for T {
        fn to_error(&self) -> DeepDiagnostic {
            DeepDiagnostic::new_error(self.to_string())
        }
        fn to_error_with<F: Fn() -> String>(&self, f: F) -> DeepDiagnostic {
            let mut s = f();
            s.push(' ');
            s.push_str(&self.to_string());
            DeepDiagnostic::new_error(s)
        }

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn to_error_at(&self, span: Span) -> Diagnostic {
            span.error(self.to_string())
        }
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn to_error_with_at<F: Fn() -> String>(&self, f: F, span: Span) -> Diagnostic {
            let mut s = f();
            s.push(' ');
            s.push_str(&self.to_string());
            span.error(s)
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    #[cfg(feature = "alloc")]
    pub trait ResultErrToStringExt<T> {
        //@TODO was ToStringResultExt
        fn map_error_to(self) -> MacroDeepResult<T>;
        fn map_error_to_with<F: Fn() -> String>(self, f: F) -> MacroDeepResult<T>;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_to_at(self, span: Span) -> MacroResult<T>;
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_to_with_at<F: Fn() -> String>(self, f: F, span: Span) -> MacroResult<T>;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }
    #[cfg(feature = "alloc")]
    impl<T, E: ToString> ResultErrToStringExt<T> for Result<T, E> {
        fn map_error_to(self) -> MacroDeepResult<T> {
            self.map_err(|e| DeepDiagnostic::new_error(e.to_string()))
        }
        fn map_error_to_with<F: Fn() -> String>(self, f: F) -> MacroDeepResult<T> {
            self.map_err(|e| {
                let mut s = f();
                s.push(' ');
                s.push_str(&e.to_string());
                DeepDiagnostic::new_error(s)
            })
        }

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_to_at(self, span: Span) -> MacroResult<T> {
            self.map_err(|e| span.error(e.to_string()))
        }
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_to_with_at<F: Fn() -> String>(self, f: F, span: Span) -> MacroResult<T> {
            self.map_err(|e| {
                let mut s = f();
                s.push(' ');
                s.push_str(&e.to_string());
                span.error(s)
            })
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    pub trait DebugExt: Debug {
        #[cfg(feature = "alloc")]
        fn dbg_error(&self) -> DeepDiagnostic;

        #[cfg(feature = "alloc")]
        fn dbg_error_with<F: Fn() -> String>(self, f: F) -> DeepDiagnostic;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn dbg_error_at(&self, span: Span) -> Diagnostic;
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn dbg_error_with_at<F: Fn() -> String>(self, f: F, span: Span) -> Diagnostic;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }

    #[cfg(feature = "alloc")]
    impl<T: Debug> DebugExt for T {
        #[cfg(feature = "alloc")]
        fn dbg_error(&self) -> DeepDiagnostic {
            DeepDiagnostic::new_error(format!("{self:?}"))
        }
        fn dbg_error_with<F: Fn() -> String>(self, f: F) -> DeepDiagnostic {
            let mut s = f();
            s.push(' ');
            s.push_str(&format!("{self:?}"));
            DeepDiagnostic::new_error(s)
        }

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn dbg_error_at(&self, span: Span) -> Diagnostic {
            span.error(format!("{self:?}"))
        }
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn dbg_error_with_at<F: Fn() -> String>(self, f: F, span: Span) -> Diagnostic {
            let mut s = f();
            s.push(' ');
            s.push_str(&format!("{self:?}"));
            span.error(s)
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    #[cfg(feature = "alloc")]
    pub trait ResultErrDebugExt<T> {
        //@TODO was DbgResultExt
        fn map_error_dbg(self) -> MacroDeepResult<T>;
        fn map_error_dbg_with<F: Fn() -> String>(self, f: F) -> MacroDeepResult<T>;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_dbg_at(self, span: Span) -> MacroResult<T>;
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_dbg_with_at<F: Fn() -> String>(self, f: F, span: Span) -> MacroResult<T>;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }
    #[cfg(feature = "alloc")]
    impl<T, E: Debug> ResultErrDebugExt<T> for Result<T, E> {
        fn map_error_dbg(self) -> MacroDeepResult<T> {
            self.map_err(|e| DeepDiagnostic::new_error(format!("{e:?}")))
        }
        fn map_error_dbg_with<F: Fn() -> String>(self, f: F) -> MacroDeepResult<T> {
            self.map_err(|e| {
                let mut s = f();
                s.push(' ');
                s.push_str(&format!("{e:?}"));
                DeepDiagnostic::new_error(s)
            })
        }

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_dbg_at(self, span: Span) -> MacroResult<T> {
            self.map_err(|e| span.error(format!("{e:?}")))
        }
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_dbg_with_at<F: Fn() -> String>(self, f: F, span: Span) -> MacroResult<T> {
            self.map_err(|e| {
                let mut s = f();
                s.push(' ');
                s.push_str(&format!("{e:?}"));
                span.error(s)
            })
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }
}

pub mod assert {
    #[cfg(feature = "alloc")]
    use crate::ext_all::OptionOrBoolExt;
    #[cfg(feature = "alloc")]
    use crate::MacroDeepResult;
    #[cfg(feature = "proc-macro2-diagnostics")]
    use crate::MacroResult;

    #[cfg(feature = "alloc")]
    use alloc::string::String;

    #[cfg(feature = "proc-macro2-diagnostics")]
    use proc_macro2::Span;

    // @TODO was true_or_error
    #[cfg(feature = "alloc")]
    pub fn true_or_error_with<F: Fn() -> String>(b: bool, f: F) -> MacroDeepResult<()> {
        b.ok_or_error_with(f)
    }
    #[cfg(feature = "proc-macro2-diagnostics")] //@TODO was true_or_error_at
    pub fn true_or_error_with_at<F: Fn() -> String>(b: bool, f: F, span: Span) -> MacroResult<()> {
        b.ok_or_error_with_at(f, span)
    }
}
