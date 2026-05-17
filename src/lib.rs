#![cfg_attr(not(feature = "proc-macro2-diagnostics"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::string::{String, ToString as _};

#[cfg(any(feature = "alloc", feature = "proc-macro2-diagnostics"))]
use core::any::Any;

use core::fmt::{self, Display, Formatter};

#[cfg(feature = "proc-macro2")]
use proc_macro2::Span;

#[cfg(feature = "proc-macro2-diagnostics")]
use proc_macro2_diagnostics::{Diagnostic, Level};

#[cfg(feature = "proc-macro2-diagnostics")]
pub type MacroResult<T> = Result<T, Diagnostic>;

#[cfg(feature = "alloc")]
pub type MacroDeepResult<T, M = String> = Result<T, DeepDiagnostic<M>>;

#[cfg(not(feature = "alloc"))]
pub type MacroDeepResult<T, M> = Result<T, DeepDiagnostic<M>>;

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
    pub fn new_error<T: Into<M>>(message: T) -> Self {
        Self {
            #[cfg(feature = "proc-macro2-diagnostics")]
            level: Level::Error,

            message: message.into(),
        }
    }
    /*pub fn new_warning<T: Into<M>>(message: T) -> Self {
        Self {
            #[cfg(feature = "proc-macro2-diagnostics")]
            level: Level::Warning,

            message: message.into(),
        }
    }*/
    /*pub fn new_note<T: Into<M>>(message: T) -> Self {
        Self {
            #[cfg(feature = "proc-macro2-diagnostics")]
            level: Level::Note,

            message: message.into(),
        }
    }*/
    /*pub fn new_help<T: Into<M>>(message: T) -> Self {
        Self {
            #[cfg(feature = "proc-macro2-diagnostics")]
            level: Level::Help,

            message: message.into(),
        }
    }*/

    pub fn into_msg(self) -> M {
        self.message
    }
    pub fn msg(&self) -> &M {
        &self.message
    }
    #[cfg(feature = "proc-macro2-diagnostics")]
    pub fn level(&self) -> Level {
        self.level
    }

    // @TODO if implemented in proc_macro2_diagnostics, make it accept MultiSpan:
    //
    // pub fn spanned<S: MultiSpan>(self, s: S) -> Diagnostic
    #[cfg(feature = "proc-macro2-diagnostics")]
    pub fn spanned(self, span: Span) -> SpannedDiagnostic<M> {
        SpannedDiagnostic { deep: self, span }
    }

    #[cfg(feature = "alloc")]
    pub fn to_string_based(self) -> DeepDiagnostic {
        DeepDiagnostic {
            #[cfg(feature = "proc-macro2-diagnostics")]
            level: self.level,

            message: self.message.to_string(),
        }
    }
}
impl<M: Display> Display for DeepDiagnostic<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}
#[cfg(feature = "alloc")]
impl DeepDiagnostic {
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

/// Like [proc_macro2_diagnostics::Diagnostic], but the message is not converted to [String], so
/// that we convert it only at the top function call tree level. To convert use
/// [SpannedDiagnostic::into_diagnostic].
#[cfg(feature = "proc-macro2")]
#[derive(Clone, Debug)]
pub struct SpannedDiagnostic<M: Display> {
    deep: DeepDiagnostic<M>,
    span: Span,
}
#[cfg(feature = "proc-macro2")]
impl<M: Display> SpannedDiagnostic<M> {
    #[cfg(feature = "proc-macro2-diagnostics")]
    pub fn into_diagnostic(self) -> Diagnostic {
        Diagnostic::spanned(self.span, self.deep.level, self.deep.message.to_string())
    }
}

#[cfg(feature = "proc-macro2")]
impl<M: Display> Display for SpannedDiagnostic<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.deep.fmt(f)
    }
}
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

pub mod ext {
    #[cfg(feature = "proc-macro2-diagnostics")]
    use crate::MacroResult;

    use crate::{DeepDiagnostic, MacroDeepResult, SealedTraitFunParam};

    #[cfg(feature = "alloc")]
    use alloc::{format, string::ToString};

    use core::fmt::{Debug, Display, Formatter, Result as FmtResult};

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
    impl<T, M: Display> MacroDeepResultExt<T> for MacroDeepResult<T, M> {
        fn spanned(self, span: Span) -> MacroResult<T> {
            self.map_err(|deep_err| deep_err.spanned(span).into_diagnostic())
        }
        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    pub trait MsgIntoDisplayExt<M: Display>: Into<M> {
        fn into_error(self) -> DeepDiagnostic<M>;

        /// Param `f` is a closure/function that returns an implementation of [Display]. This method
        /// returns [DeepDiagnostic] with a message
        /// - starting with that [Display] returned from `f()`, then
        /// - a space appended, and
        /// - ONLY THEN [Display] generated by `self.into()`.
        fn into_error_with<FM: Display, F: Fn() -> FM>(self, f: F) -> DeepDiagnostic<impl Display>;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn into_error_at(self, span: Span) -> Diagnostic;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn into_error_with_at<FM: Display, F: Fn() -> FM>(self, f: F, span: Span) -> Diagnostic;

        // Sealing is not really necessary, because we have a blanket impl that covers any and
        // all eligible types, so no other types can implement this trait.
        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }
    struct DisplayFromFn<F: Fn(&mut Formatter) -> FmtResult>(F);
    impl<F: Fn(&mut Formatter) -> FmtResult> Display for DisplayFromFn<F> {
        fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
            self.0(fmt)
        }
    }
    impl<M: Display, T: Into<M>> MsgIntoDisplayExt<M> for T {
        fn into_error(self) -> DeepDiagnostic<M> {
            DeepDiagnostic::new_error(self.into())
        }

        fn into_error_with<FM: Display, F: Fn() -> FM>(self, f: F) -> DeepDiagnostic<impl Display> {
            let s = self.into();
            let display = DisplayFromFn(move |fmt| {
                f().fmt(fmt)?;
                Display::fmt(&' ', fmt)?; // ' '.fmt(fmt) is ambiguous
                s.fmt(fmt)
            });
            DeepDiagnostic::<DisplayFromFn<_>>::new_error(display)
        }

        /// Convenience function: same as into_error().span(span)
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn into_error_at(self, span: Span) -> Diagnostic {
            let m = self.into();
            span.error(m.to_string())
        }
        /// Convenience function: same as into_error_with(f).span(span)
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn into_error_with_at<FM: Display, F: Fn() -> FM>(self, f: F, span: Span) -> Diagnostic {
            let mut s = f().to_string();
            s.push(' ');
            s.push_str(&self.into().to_string());
            span.error(s)
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    pub trait ResultErrIntoDisplayExt<M: Display, T> {
        fn map_error_into(self) -> MacroDeepResult<T, M>;
        fn map_error_into_with<FM: Display, F: Fn() -> FM>(
            self,
            f: F,
        ) -> MacroDeepResult<T, impl Display>;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_into_at(self, span: Span) -> MacroResult<T>;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_into_with_at<FM: Display, F: Fn() -> FM>(
            self,
            f: F,
            span: Span,
        ) -> MacroResult<T>;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }
    impl<M: Display, T, E: Into<M>> ResultErrIntoDisplayExt<M, T> for Result<T, E> {
        fn map_error_into(self) -> MacroDeepResult<T, M> {
            self.map_err(|e| DeepDiagnostic::new_error(e))
        }
        fn map_error_into_with<FM: Display, F: Fn() -> FM>(
            self,
            f: F,
        ) -> MacroDeepResult<T, impl Display> {
            self.map_err(|e| {
                let e = e.into();
                let display = DisplayFromFn(move |fmt| {
                    f().fmt(fmt)?;
                    Display::fmt(&' ', fmt)?; // ' '.fmt(fmt) is ambiguous
                    e.fmt(fmt)
                });
                DeepDiagnostic::<DisplayFromFn<_>>::new_error(display)
            })
        }

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_into_at(self, span: Span) -> MacroResult<T> {
            self.map_err(|e| span.error(e.into().to_string()))
        }
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_into_with_at<FM: Display, F: Fn() -> FM>(
            self,
            f: F,
            span: Span,
        ) -> MacroResult<T> {
            self.map_err(|e| {
                let s = format!("{} {}", f(), e.into());
                span.error(s)
            })
        }
        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    pub trait OptionOrBoolExt<T> {
        fn ok_or_error_with<FM: Display, F: Fn() -> FM>(self, f: F) -> MacroDeepResult<T, FM>;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn ok_or_error_with_at<FM: Display, F: Fn() -> FM>(
            self,
            f: F,
            span: Span,
        ) -> MacroResult<T>;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }
    impl<T> OptionOrBoolExt<T> for Option<T> {
        fn ok_or_error_with<FM: Display, F: Fn() -> FM>(self, f: F) -> MacroDeepResult<T, FM> {
            self.ok_or_else(|| DeepDiagnostic::new_error(f()))
        }

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn ok_or_error_with_at<FM: Display, F: Fn() -> FM>(
            self,
            f: F,
            span: Span,
        ) -> MacroResult<T> {
            self.ok_or_else(|| span.error(f().to_string()))
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }
    impl OptionOrBoolExt<()> for bool {
        fn ok_or_error_with<FM: Display, F: Fn() -> FM>(self, f: F) -> MacroDeepResult<(), FM> {
            // bool::ok_or_else is unstable: https://github.com/rust-lang/rust/issues/142748
            if self {
                Ok(())
            } else {
                Err(DeepDiagnostic::new_error(f()))
            }
        }

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn ok_or_error_with_at<FM: Display, F: Fn() -> FM>(
            self,
            f: F,
            span: Span,
        ) -> MacroResult<()> {
            if self {
                Ok(())
            } else {
                Err(span.error(f().to_string()))
            }
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    #[cfg(feature = "alloc")]
    pub trait ToStringExt: ToString {
        fn to_error(&self) -> DeepDiagnostic;
        fn to_error_with<FM: Display, F: Fn() -> FM>(&self, f: F) -> DeepDiagnostic;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn to_error_at(&self, span: Span) -> Diagnostic;
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn to_error_with_at<FM: Display, F: Fn() -> FM>(&self, f: F, span: Span) -> Diagnostic;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }
    #[cfg(feature = "alloc")]
    impl<T: ToString> ToStringExt for T {
        fn to_error(&self) -> DeepDiagnostic {
            DeepDiagnostic::new_error(self.to_string())
        }
        fn to_error_with<FM: Display, F: Fn() -> FM>(&self, f: F) -> DeepDiagnostic {
            let s = format!("{} {}", f(), self.to_string());
            DeepDiagnostic::new_error(s)
        }

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn to_error_at(&self, span: Span) -> Diagnostic {
            span.error(self.to_string())
        }
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn to_error_with_at<FM: Display, F: Fn() -> FM>(&self, f: F, span: Span) -> Diagnostic {
            let s = format!("{} {}", f(), self.to_string());
            span.error(s)
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    #[cfg(feature = "alloc")]
    pub trait ResultErrToDisplayExt<T> {
        fn map_error_to(self) -> MacroDeepResult<T>;
        fn map_error_to_with<FM: Display, F: Fn() -> FM>(self, f: F) -> MacroDeepResult<T>;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_to_at(self, span: Span) -> MacroResult<T>;
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_to_with_at<FM: Display, F: Fn() -> FM>(
            self,
            f: F,
            span: Span,
        ) -> MacroResult<T>;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }
    #[cfg(feature = "alloc")]
    impl<T, E: ToString> ResultErrToDisplayExt<T> for Result<T, E> {
        fn map_error_to(self) -> MacroDeepResult<T> {
            self.map_err(|e| DeepDiagnostic::new_error(e.to_string()))
        }
        fn map_error_to_with<FM: Display, F: Fn() -> FM>(self, f: F) -> MacroDeepResult<T> {
            self.map_err(|e| {
                let s = format!("{} {}", f(), e.to_string());
                DeepDiagnostic::new_error(s)
            })
        }

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_to_at(self, span: Span) -> MacroResult<T> {
            self.map_err(|e| span.error(e.to_string()))
        }
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_to_with_at<FM: Display, F: Fn() -> FM>(
            self,
            f: F,
            span: Span,
        ) -> MacroResult<T> {
            self.map_err(|e| {
                let s = format!("{} {}", f(), e.to_string());
                span.error(s)
            })
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    pub trait DebugExt: Debug {
        fn dbg_error(&self) -> DeepDiagnostic<impl Display>;

        fn dbg_error_with<FM: Display, F: Fn() -> FM>(self, f: F) -> DeepDiagnostic<impl Display>;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn dbg_error_at(&self, span: Span) -> Diagnostic;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn dbg_error_with_at<FM: Display, F: Fn() -> FM>(self, f: F, span: Span) -> Diagnostic;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }

    impl<T: Debug> DebugExt for T {
        fn dbg_error(&self) -> DeepDiagnostic<impl Display> {
            let display = DisplayFromFn(move |fmt| fmt.write_fmt(format_args!("{self:?}")));
            DeepDiagnostic::<DisplayFromFn<_>>::new_error(display)
        }
        fn dbg_error_with<FM: Display, F: Fn() -> FM>(self, f: F) -> DeepDiagnostic<impl Display> {
            let display = DisplayFromFn(move |fmt| {
                f().fmt(fmt)?;
                Display::fmt(&' ', fmt)?; // ' '.fmt(fmt) is ambiguous
                fmt.write_fmt(format_args!("{self:?}"))
            });
            DeepDiagnostic::<DisplayFromFn<_>>::new_error(display)
        }

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn dbg_error_at(&self, span: Span) -> Diagnostic {
            span.error(format!("{self:?}"))
        }
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn dbg_error_with_at<FM: Display, F: Fn() -> FM>(self, f: F, span: Span) -> Diagnostic {
            let s = format!("{} {:?}", f(), self);
            span.error(s)
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    pub trait ResultErrDebugExt<T> {
        fn map_error_dbg(self) -> MacroDeepResult<T, impl Display>;
        fn map_error_dbg_with<FM: Display, F: Fn() -> FM>(
            self,
            f: F,
        ) -> MacroDeepResult<T, impl Display>;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_dbg_at(self, span: Span) -> MacroResult<T>;
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_dbg_with_at<FM: Display, F: Fn() -> FM>(
            self,
            f: F,
            span: Span,
        ) -> MacroResult<T>;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }
    impl<T, E: Debug> ResultErrDebugExt<T> for Result<T, E> {
        fn map_error_dbg(self) -> MacroDeepResult<T, impl Display> {
            self.map_err(|e| {
                let display = DisplayFromFn(move |fmt| fmt.write_fmt(format_args!("{e:?}")));
                DeepDiagnostic::<DisplayFromFn<_>>::new_error(display)
            })
        }
        fn map_error_dbg_with<FM: Display, F: Fn() -> FM>(
            self,
            f: F,
        ) -> MacroDeepResult<T, impl Display> {
            self.map_err(|e| {
                let display = DisplayFromFn(move |fmt| {
                    f().fmt(fmt)?;
                    Display::fmt(&' ', fmt)?; // ' '.fmt(fmt) is ambiguous
                    fmt.write_fmt(format_args!("{e:?}"))
                });
                DeepDiagnostic::<DisplayFromFn<_>>::new_error(display)
            })
        }

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_dbg_at(self, span: Span) -> MacroResult<T> {
            self.map_err(|e| span.error(format!("{e:?}")))
        }
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_dbg_with_at<FM: Display, F: Fn() -> FM>(
            self,
            f: F,
            span: Span,
        ) -> MacroResult<T> {
            self.map_err(|e| {
                let s = format!("{} {:?}", f(), e);
                span.error(s)
            })
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    #[cfg(feature = "alloc")]
    pub trait MacroDeepResultDisplayToString<T> {
        fn to_string_based(self) -> MacroDeepResult<T>;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }
    #[cfg(feature = "alloc")]
    impl<T, M: Display> MacroDeepResultDisplayToString<T> for MacroDeepResult<T, M> {
        fn to_string_based(self) -> MacroDeepResult<T> {
            self.map_err(|e| e.to_string_based())
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }
}

pub mod assert {
    use crate::ext::OptionOrBoolExt;

    use crate::MacroDeepResult;

    #[cfg(feature = "proc-macro2-diagnostics")]
    use crate::MacroResult;

    use core::fmt::Display;

    #[cfg(feature = "proc-macro2-diagnostics")]
    use proc_macro2::Span;

    pub fn true_or_error_with<FM: Display, F: Fn() -> FM>(
        b: bool,
        f: F,
    ) -> MacroDeepResult<(), impl Display> {
        b.ok_or_error_with(f)
    }
    #[cfg(feature = "proc-macro2-diagnostics")]
    pub fn true_or_error_with_at<FM: Display, F: Fn() -> FM>(
        b: bool,
        f: F,
        span: Span,
    ) -> MacroResult<()> {
        b.ok_or_error_with_at(f, span)
    }
}
