#![cfg_attr(not(feature = "proc-macro2"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::string::{String, ToString as _};

#[cfg(any(feature = "alloc", feature = "proc-macro2-diagnostics"))]
use core::any::Any;

#[cfg(feature = "proc-macro2-diagnostics")]
use proc_macro2::Span;

use core::fmt::{self, Display, Formatter};

#[cfg(feature = "proc-macro2-diagnostics")]
use proc_macro2_diagnostics::{Diagnostic as PmDiagnostic, Level};

#[cfg(not(feature = "alloc"))]
pub type DisplayishResult<T, D, EX = ()> = Result<T, Displayish<D, EX>>;

#[cfg(feature = "alloc")]
pub type DisplayishResult<T, D = String, EX = ()> = Result<T, Displayish<D, EX>>;

//-----
#[cfg(feature = "proc-macro2-diagnostics")]
pub type MacroDeepDiagnostic<D = String> = Displayish<D, Level>;

/// Like [proc_macro2_diagnostics::Diagnostic], but its [Displayish::display] is NOT converted to
/// [String], so that we convert it only at the top function call tree level. To convert use
/// [SpannedDiagnostic::into_diagnostic]. <--- @TODO this docs
#[cfg(feature = "proc-macro2-diagnostics")]
pub type MacroSpannedDiagnostic<D = String> = Displayish<D, (Level, Span)>;

#[cfg(feature = "proc-macro2-diagnostics")]
pub type MacroDeepResult<T, D = String> = Result<T, MacroDeepDiagnostic<D>>;

#[cfg(feature = "proc-macro2-diagnostics")]
pub type MacroResult<T> = Result<T, PmDiagnostic>;
//-----

/*#[cfg(feature = "proc-macro2")]
pub type MacroSpannedResult<T, D> = Result<T, SpannedDiagnostic<D>>;*/

#[cfg(feature = "alloc")]
#[derive(Clone, Debug)]
pub struct Displayish<D: Display = String, EX = ()> {
    display: D,
    extra: EX,
}
#[cfg(not(feature = "alloc"))]
#[derive(Clone, Debug)]
pub struct Displayish<D: Display, EX = ()> {
    display: D,
    extra: EX,
}
impl<D: Display, EX> Displayish<D, EX> {
    pub const fn new_from_pair(display: D, extra: EX) -> Self {
        Self { display, extra }
    }
    pub fn new_from_into_pair(display: impl Into<D>, extra: impl Into<EX>) -> Self {
        Self::new_from_pair(display.into(), extra.into())
    }
    pub fn into_display(self) -> D {
        self.display
    }
    pub fn display(&self) -> &D {
        &self.display
    }
    pub fn into_extra(self) -> EX {
        self.extra
    }
    pub fn extra(&self) -> &EX {
        &self.extra
    }
    pub fn into_pair(self) -> (D, EX) {
        (self.display, self.extra)
    }

    #[cfg(feature = "alloc")]
    pub fn to_string_based(self) -> Displayish<String, EX> {
        let display = self.display.to_string();
        let extra = self.extra;
        Displayish { display, extra }
    }
}
impl<D: Display> Displayish<D, ()> {
    pub fn new_from_display(display: impl Into<D>) -> Self {
        let display = display.into();
        let extra = ();
        Self { display, extra }
    }
}
impl<D: Display, EX> Display for Displayish<D, EX> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.display.fmt(f)
    }
}
#[cfg(feature = "alloc")]
impl Displayish {
    pub fn to_string_move(self) -> String {
        self.display
    }
}
#[cfg(feature = "alloc")]
impl<D: Display + 'static> Displayish<D> {
    pub fn to_string_move_maybe(mut self) -> String {
        let self_mut = &mut self as &mut dyn Any;
        match self_mut.downcast_mut::<Displayish<String>>() {
            Some(as_string_based) => core::mem::take(&mut as_string_based.display),
            None => self.to_string(),
        }
    }
}
impl<D: Display> From<D> for Displayish<D, ()> {
    //@TODO DOC old:
    /// Move-and-construct/convert. If using `proc-macro2-diagnostics`, then [DeepDiagnostic::level]
    /// will be set to [Level::Error].
    fn from(display: D) -> Self {
        let extra = ();
        Self { display, extra }
    }
}

#[cfg(feature = "proc-macro2-diagnostics")]
impl<D: Display> MacroDeepDiagnostic<D> {
    pub fn spanned(self, span: Span) -> MacroSpannedDiagnostic<D> {
        MacroSpannedDiagnostic {
            display: self.display,
            extra: (self.extra, span),
        }
    }
}

/// Intentionally not public - used to indicate a sealed trait.
enum SealedTraitFunParam {}
//--------

#[cfg(feature = "proc-macro2-diagnostics")]
impl<D: Display> MacroSpannedDiagnostic<D> {
    pub fn into_diagnostic(self) -> PmDiagnostic {
        PmDiagnostic::spanned(self.extra.1, self.extra.0, self.display.to_string())
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
    use crate::{MacroDeepResult, MacroResult};

    use crate::{Displayish, DisplayishResult, SealedTraitFunParam};

    #[cfg(feature = "alloc")]
    use alloc::{format, string::String, string::ToString};

    use core::fmt::{Debug, Display, Formatter, Result as FmtResult};

    #[cfg(feature = "proc-macro2-diagnostics")]
    use proc_macro2::Span;

    #[cfg(feature = "proc-macro2-diagnostics")]
    use proc_macro2_diagnostics::{Diagnostic as PmDiagnostic, SpanDiagnosticExt as _};

    #[cfg(feature = "proc-macro2-diagnostics")]
    pub trait MacroDeepResultExt<T> {
        // @TODO if implemented in proc_macro2_diagnostics, make it accept MultiSpan.
        /// Add the given [Span], and transform to [MacroResult].
        fn spanned(self, span: Span) -> MacroResult<T>;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }
    #[cfg(feature = "proc-macro2-diagnostics")]
    impl<T, D: Display> MacroDeepResultExt<T> for MacroDeepResult<T, D> {
        fn spanned(self, span: Span) -> MacroResult<T> {
            self.map_err(|deep_err| deep_err.spanned(span).into_diagnostic())
        }
        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    pub trait ContentIntoDisplayExt<D: Display>: Into<D> {
        fn into_dis(self) -> Displayish<D>;

        /// Param `f` is a closure/function that returns an implementation of [Display]. This method
        /// returns [DeepDiagnostic] with a [Displayish::display]
        /// - starting with that [Display] returned from `f()`, then
        /// - a space appended, and
        /// - ONLY THEN [Display] generated by `self.into()`.
        fn into_dis_with<FD: Display, F: Fn() -> FD>(self, f: F) -> Displayish<impl Display>;

        fn into_dis_and<EX>(self, extra: EX) -> Displayish<D, EX>;

        fn into_dis_with_and<FD: Display, F: Fn() -> FD, EX>(
            self,
            f: F,
            extra: EX,
        ) -> Displayish<impl Display, EX>;

        // Sealing is not really necessary, because we have a blanket impl that covers any and
        // all eligible types, so no other types can implement this trait.
        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }
    struct DisplayFromFn<F>(F)
    where
        F: Fn(&mut Formatter) -> FmtResult;
    impl<F: Fn(&mut Formatter) -> FmtResult> Display for DisplayFromFn<F> {
        fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
            self.0(fmt)
        }
    }
    impl<D: Display, T: Into<D>> ContentIntoDisplayExt<D> for T {
        fn into_dis(self) -> Displayish<D> {
            Displayish::new_from_display(self.into())
        }

        fn into_dis_with<FD: Display, F: Fn() -> FD>(self, f: F) -> Displayish<impl Display> {
            let s = self.into();
            let display = DisplayFromFn(move |fmt| {
                f().fmt(fmt)?;
                Display::fmt(&' ', fmt)?; // ' '.fmt(fmt) is ambiguous
                s.fmt(fmt)
            });
            Displayish::<DisplayFromFn<_>>::new_from_display(display)
        }

        fn into_dis_and<EX>(self, extra: EX) -> Displayish<D, EX> {
            Displayish::new_from_pair(self.into(), extra)
        }
        fn into_dis_with_and<FD: Display, F: Fn() -> FD, EX>(
            self,
            f: F,
            extra: EX,
        ) -> Displayish<impl Display, EX> {
            let s = self.into();
            let display = DisplayFromFn(move |fmt| {
                f().fmt(fmt)?;
                Display::fmt(&' ', fmt)?; // ' '.fmt(fmt) is ambiguous
                s.fmt(fmt)
            });
            Displayish::<DisplayFromFn<_>, _>::new_from_pair(display, extra)
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    pub trait ResultErrIntoDisplayExt<D: Display, T> {
        fn map_error_into(self) -> DisplayishResult<T, D>;
        fn map_error_into_with<FD: Display, F: Fn() -> FD>(
            self,
            f: F,
        ) -> DisplayishResult<T, impl Display>;

        fn map_error_into_and<EX>(self, extra: EX) -> DisplayishResult<T, impl Display, EX>;

        fn map_error_into_with_and<FD: Display, F: Fn() -> FD, EX>(
            self,
            f: F,
            extra: EX,
        ) -> DisplayishResult<T, impl Display, EX>;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }
    impl<D: Display, T, ERR: Into<D>> ResultErrIntoDisplayExt<D, T> for Result<T, ERR> {
        fn map_error_into(self) -> DisplayishResult<T, D> {
            self.map_err(|err| Displayish::new_from_display(err))
        }
        fn map_error_into_with<FD: Display, F: Fn() -> FD>(
            self,
            f: F,
        ) -> DisplayishResult<T, impl Display> {
            self.map_err(|err| {
                let err = err.into();
                let display = DisplayFromFn(move |fmt| {
                    f().fmt(fmt)?;
                    Display::fmt(&' ', fmt)?; // ' '.fmt(fmt) is ambiguous
                    err.fmt(fmt)
                });
                Displayish::<DisplayFromFn<_>>::new_from_display(display)
            })
        }

        fn map_error_into_and<EX>(self, extra: EX) -> DisplayishResult<T, impl Display, EX> {
            self.map_err(|err| Displayish::new_from_into_pair(err, extra))
        }
        fn map_error_into_with_and<FD: Display, F: Fn() -> FD, EX>(
            self,
            f: F,
            extra: EX,
        ) -> DisplayishResult<T, impl Display, EX> {
            self.map_err(|err| {
                let err = err.into();
                let display = DisplayFromFn(move |fmt| {
                    f().fmt(fmt)?;
                    Display::fmt(&' ', fmt)?; // ' '.fmt(fmt) is ambiguous
                    err.fmt(fmt)
                });
                Displayish::new_from_pair(display, extra)
            })
        }
        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    pub trait OptionOrBoolExt<T> {
        fn ok_or_error_with<FD: Display, F: Fn() -> FD>(self, f: F) -> DisplayishResult<T, FD>;

        fn ok_or_error_with_and<FD: Display, F: Fn() -> FD, EX>(
            self,
            f: F,
            extra: EX,
        ) -> DisplayishResult<T, FD, EX>;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }
    impl<T> OptionOrBoolExt<T> for Option<T> {
        fn ok_or_error_with<FD: Display, F: Fn() -> FD>(self, f: F) -> DisplayishResult<T, FD> {
            self.ok_or_else(|| Displayish::new_from_display(f()))
        }

        fn ok_or_error_with_and<FD: Display, F: Fn() -> FD, EX>(
            self,
            f: F,
            extra: EX,
        ) -> DisplayishResult<T, FD, EX> {
            self.ok_or_else(|| Displayish::new_from_pair(f(), extra))
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }
    impl OptionOrBoolExt<()> for bool {
        fn ok_or_error_with<FD: Display, F: Fn() -> FD>(self, f: F) -> DisplayishResult<(), FD> {
            // bool::ok_or_else is unstable: https://github.com/rust-lang/rust/issues/142748
            if self {
                Ok(())
            } else {
                Err(Displayish::new_from_display(f()))
            }
        }

        fn ok_or_error_with_and<FD: Display, F: Fn() -> FD, EX>(
            self,
            f: F,
            extra: EX,
        ) -> DisplayishResult<(), FD, EX> {
            if self {
                Ok(())
            } else {
                Err(Displayish::new_from_pair(f(), extra))
            }
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    #[cfg(feature = "alloc")]
    pub trait ToStringExt: ToString {
        fn to_error(&self) -> Displayish;
        fn to_error_with<FD: Display, F: Fn() -> FD>(&self, f: F) -> Displayish;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn to_error_at(&self, span: Span) -> PmDiagnostic;
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn to_error_with_at<FD: Display, F: Fn() -> FD>(&self, f: F, span: Span) -> PmDiagnostic;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }
    #[cfg(feature = "alloc")]
    impl<T: ToString> ToStringExt for T {
        fn to_error(&self) -> Displayish {
            Displayish::new_from_display(self.to_string())
        }
        fn to_error_with<FD: Display, F: Fn() -> FD>(&self, f: F) -> Displayish {
            let s = format!("{} {}", f(), self.to_string());
            Displayish::new_from_display(s)
        }

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn to_error_at(&self, span: Span) -> PmDiagnostic {
            span.error(self.to_string())
        }
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn to_error_with_at<FD: Display, F: Fn() -> FD>(&self, f: F, span: Span) -> PmDiagnostic {
            let s = format!("{} {}", f(), self.to_string());
            span.error(s)
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    #[cfg(feature = "alloc")]
    pub trait ResultErrToDisplayExt<T> {
        fn map_error_to(self) -> DisplayishResult<T>;
        fn map_error_to_with<FD: Display, F: Fn() -> FD>(self, f: F) -> DisplayishResult<T>;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_to_at(self, span: Span) -> MacroResult<T>;
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_to_with_at<FD: Display, F: Fn() -> FD>(
            self,
            f: F,
            span: Span,
        ) -> MacroResult<T>;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }
    #[cfg(feature = "alloc")]
    impl<T, ERR: ToString> ResultErrToDisplayExt<T> for Result<T, ERR> {
        fn map_error_to(self) -> DisplayishResult<T> {
            self.map_err(|e| Displayish::new_from_display(e.to_string()))
        }
        fn map_error_to_with<FD: Display, F: Fn() -> FD>(self, f: F) -> DisplayishResult<T> {
            self.map_err(|e| {
                let s = format!("{} {}", f(), e.to_string());
                Displayish::new_from_display(s)
            })
        }

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_to_at(self, span: Span) -> MacroResult<T> {
            self.map_err(|e| span.error(e.to_string()))
        }
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn map_error_to_with_at<FD: Display, F: Fn() -> FD>(
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
        fn dbg_error(&self) -> Displayish<impl Display>;

        fn dbg_error_with<FD: Display, F: Fn() -> FD>(self, f: F) -> Displayish<impl Display>;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn dbg_error_at(&self, span: Span) -> PmDiagnostic;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn dbg_error_with_at<FD: Display, F: Fn() -> FD>(self, f: F, span: Span) -> PmDiagnostic;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }

    impl<T: Debug> DebugExt for T {
        fn dbg_error(&self) -> Displayish<impl Display> {
            let display = DisplayFromFn(move |fmt| fmt.write_fmt(format_args!("{self:?}")));
            Displayish::<DisplayFromFn<_>>::new_from_display(display)
        }
        fn dbg_error_with<FD: Display, F: Fn() -> FD>(self, f: F) -> Displayish<impl Display> {
            let display = DisplayFromFn(move |fmt| {
                f().fmt(fmt)?;
                Display::fmt(&' ', fmt)?; // ' '.fmt(fmt) is ambiguous
                fmt.write_fmt(format_args!("{self:?}"))
            });
            Displayish::<DisplayFromFn<_>>::new_from_display(display)
        }

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn dbg_error_at(&self, span: Span) -> PmDiagnostic {
            span.error(format!("{self:?}"))
        }
        #[cfg(feature = "proc-macro2-diagnostics")]
        fn dbg_error_with_at<FD: Display, F: Fn() -> FD>(self, f: F, span: Span) -> PmDiagnostic {
            let s = format!("{} {:?}", f(), self);
            span.error(s)
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    pub trait ResultErrDebugExt<T> {
        fn map_error_dbg(self) -> DisplayishResult<T, impl Display>;
        fn map_error_dbg_with<FD: Display, F: Fn() -> FD>(
            self,
            f: F,
        ) -> DisplayishResult<T, impl Display>;

        fn map_error_dbg_and<EX>(self, extra: EX) -> DisplayishResult<T, impl Display, EX>;
        fn map_error_dbg_with_and<FD: Display, F: Fn() -> FD, EX>(
            self,
            f: F,
            extra: EX,
        ) -> DisplayishResult<T, impl Display, EX>;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam);
    }
    impl<T, ERR: Debug> ResultErrDebugExt<T> for Result<T, ERR> {
        fn map_error_dbg(self) -> DisplayishResult<T, impl Display> {
            self.map_err(|e| {
                let display = DisplayFromFn(move |fmt| fmt.write_fmt(format_args!("{e:?}")));
                Displayish::<DisplayFromFn<_>>::new_from_display(display)
            })
        }
        fn map_error_dbg_with<FD: Display, F: Fn() -> FD>(
            self,
            f: F,
        ) -> DisplayishResult<T, impl Display> {
            self.map_err(|e| {
                let display = DisplayFromFn(move |fmt| {
                    f().fmt(fmt)?;
                    Display::fmt(&' ', fmt)?; // ' '.fmt(fmt) is ambiguous
                    fmt.write_fmt(format_args!("{e:?}"))
                });
                Displayish::<DisplayFromFn<_>>::new_from_display(display)
            })
        }

        fn map_error_dbg_and<EX>(self, extra: EX) -> DisplayishResult<T, impl Display, EX> {
            self.map_err(|e| {
                let display = DisplayFromFn(move |fmt| fmt.write_fmt(format_args!("{e:?}")));
                Displayish::<DisplayFromFn<_>, _>::new_from_pair(display, extra)
            })
        }
        fn map_error_dbg_with_and<FD: Display, F: Fn() -> FD, EX>(
            self,
            f: F,
            extra: EX,
        ) -> DisplayishResult<T, impl Display, EX> {
            self.map_err(|e| {
                let display = DisplayFromFn(move |fmt| {
                    f().fmt(fmt)?;
                    Display::fmt(&' ', fmt)?; // ' '.fmt(fmt) is ambiguous
                    fmt.write_fmt(format_args!("{e:?}"))
                });
                Displayish::<DisplayFromFn<_>, _>::new_from_pair(display, extra)
            })
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }

    #[cfg(feature = "alloc")]
    pub trait ResultDisplayToString<T, EX> {
        fn to_string_based(self) -> DisplayishResult<T, String, EX>;

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }
    #[cfg(feature = "alloc")]
    impl<T, D: Display, EX> ResultDisplayToString<T, EX> for DisplayishResult<T, D, EX> {
        fn to_string_based(self) -> DisplayishResult<T, String, EX> {
            self.map_err(|e| e.to_string_based())
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: SealedTraitFunParam) {}
    }
}

pub mod assert {
    use crate::ext::OptionOrBoolExt;
    use crate::DisplayishResult;
    use core::fmt::Display;

    pub fn true_or_error_with<FD: Display, F: Fn() -> FD>(
        b: bool,
        f: F,
    ) -> DisplayishResult<(), FD> {
        b.ok_or_error_with(f)
    }
    pub fn true_or_error_with_at<FD: Display, F: Fn() -> FD, EX>(
        b: bool,
        f: F,
        extra: EX,
    ) -> DisplayishResult<(), FD, EX> {
        b.ok_or_error_with_and(f, extra)
    }
}
