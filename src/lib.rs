#![cfg_attr(not(feature = "proc-macro2-diagnostics"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::string::String;

#[cfg(feature = "proc-macro2-diagnostics")]
use proc_macro2::Span;
#[cfg(feature = "proc-macro2-diagnostics")]
use proc_macro2_diagnostics::Diagnostic;

#[cfg(feature = "proc-macro2-diagnostics")]
pub type MacroResult<T> = Result<T, Diagnostic>;

pub type MacroDeepResult<T> = Result<T, DeepDiagnostic>;

/*pub type Star = &'static str;
pub type RefStar = &'static Star;

pub type SliStar = &'static [Star];
pub type SloStar = &'static [Option<Star>];

pub type SliSliStar = &'static [SliStar];
pub type SloSloStar = &'static [Option<SloStar>];
pub type RefSliSliStar = &'static SliSliStar;
pub type RefSloSloStar = &'static SloSloStar;

pub type SliSliSliStar = &'static [SliSliStar];
pub type SloSloSloStar = &'static [Option<SloSloStar>];

#[derive(Clone, Debug)]
#[repr(transparent)]
struct FewSliStar([SliStar; 2]);
impl FewSliStar {
    fn description(&self) -> SliStar {
        self.0[0]
    }
    fn error(&self) -> SliStar {
        self.0[1]
    }
}
#[derive(Clone, Debug)]
#[repr(transparent)]
struct FewOptSliStar([SloStar; 2]);
impl FewOptSliStar {
    fn description(&self) -> SloStar {
        self.0[0]
    }
    fn error(&self) -> SloStar {
        self.0[1]
    }
}

// @TODO field total_length
// - BUT, when composing, SKIP that level for internal parts - point direct = one level below
// - Overlays: `usize`` param to assert the expected number of items in the overlayed, so that later
//   modifications trigger changes.
// - `const`
*/
//----

#[derive(Clone, Debug)]
struct DeepDiagnosticMessage(#[cfg(feature = "alloc")] String);

#[derive(Clone, Debug)]
pub struct DeepDiagnostic {
    #[cfg(feature = "proc-macro2-diagnostics")]
    level: proc_macro2_diagnostics::Level,

    message: DeepDiagnosticMessage,
}
impl DeepDiagnostic {
    // @TODO macro_rules and also generate: pub fn warning, note, help
    #[cfg(feature = "alloc")]
    pub fn error_string<T: Into<String>>(message: T) -> Self {
        Self {
            #[cfg(feature = "proc-macro2-diagnostics")]
            level: proc_macro2_diagnostics::Level::Error,

            message: DeepDiagnosticMessage(message.into()),
        }
    }
    #[cfg(feature = "alloc")]
    pub fn message_string(self) -> String {
        self.message.0
    }

    // @TODO if implemented in proc_macro2_diagnostics, make it accept MultiSpan:
    //
    // pub fn spanned<S: MultiSpan>(self, s: S) -> Diagnostic
    #[cfg(feature = "proc-macro2-diagnostics")]
    pub fn spanned(self, span: Span) -> Diagnostic {
        Diagnostic::spanned(span, self.level, Into::<String>::into(self))
    }
}

/*
use core::iter::{self, Cloned, Flatten, Once};

enum MessageStarIterEnum<'a> {
    OwnString(Once<Star>), //@TODO - LEAKS!
    Sli(core::slice::Iter<'a, Star>),
    SliSliSli(Flatten<Cloned<Flatten<Cloned<core::slice::Iter<'a, SliSliStar>>>>>),
}
#[repr(transparent)]
pub struct MessageStarIter<'a>(MessageStarIterEnum<'a>);
impl<'a> Iterator for MessageStarIter<'a> {
    type Item = Star;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            MessageStarIterEnum::OwnString(ref mut once) => once.next(),
            MessageStarIterEnum::Sli(ref mut sli) => sli.next().map(|v| *v),
            MessageStarIterEnum::SliSliSli(ref mut sli_sli_sli) => sli_sli_sli.next().map(|v| *v),
        }
    }
}*/

#[cfg(feature = "alloc")]
impl From<DeepDiagnostic> for String {
    fn from(_deep: DeepDiagnostic) -> Self {
        todo!()
    }
}

pub mod ext {
    #[cfg(feature = "proc-macro2-diagnostics")]
    use crate::MacroResult;
    use crate::{sealed, DeepDiagnostic, MacroDeepResult};

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
    pub trait MacroDeepResultExt<T>: sealed::Trait {
        // @TODO if implemented in proc_macro2_diagnostics, make it accept MultiSpan.
        /// Add the given [Span], and transform to [MacroResult].
        fn spanned(self, span: Span) -> MacroResult<T>;
    }
    #[cfg(feature = "proc-macro2-diagnostics")]
    impl<T> MacroDeepResultExt<T> for MacroDeepResult<T> {
        fn spanned(self, span: Span) -> MacroResult<T> {
            self.map_err(|deep_err| deep_err.spanned(span))
        }
    }
    impl<T> sealed::Trait for MacroDeepResult<T> {
        #[allow(private_interfaces)]
        fn _seal(&self, _: &sealed::TraitParam) {}
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
        fn _seal(&self, _: &sealed::TraitParam);
    }
    #[cfg(feature = "alloc")]
    impl<T: Into<String>> IntoStringExt for T {
        fn into_error(self) -> DeepDiagnostic {
            DeepDiagnostic::error_string(self.into())
        }
        fn into_error_with<F: Fn() -> String>(self, f: F) -> DeepDiagnostic {
            let mut s = f();
            s.push(' ');
            s.push_str(&self.into());
            DeepDiagnostic::error_string(s)
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
        fn _seal(&self, _: &sealed::TraitParam) {}
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
        fn _seal(&self, _: &sealed::TraitParam);
    }
    #[cfg(feature = "alloc")]
    impl<T, E: Into<String>> ResultErrIntoStringExt<T> for Result<T, E> {
        fn map_error_into(self) -> MacroDeepResult<T> {
            self.map_err(|e| DeepDiagnostic::error_string(e))
        }
        fn map_error_into_with<F: Fn() -> String>(self, f: F) -> MacroDeepResult<T> {
            self.map_err(|e| {
                let mut s = f();
                s.push(' ');
                s.push_str(&e.into());
                DeepDiagnostic::error_string(s)
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
        fn _seal(&self, _: &sealed::TraitParam) {}
    }

    pub trait OptionOrBoolExt<T> {
        #[cfg(feature = "alloc")]
        fn ok_or_error_with<F: Fn() -> String>(self, f: F) -> MacroDeepResult<T>;

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn ok_or_error_with_at<F: Fn() -> String>(self, f: F, span: Span) -> MacroResult<T>;

        #[allow(private_interfaces)]
        fn _seal(&self, _: &sealed::TraitParam);
    }
    impl<T> OptionOrBoolExt<T> for Option<T> {
        #[cfg(feature = "alloc")]
        fn ok_or_error_with<F: Fn() -> String>(self, f: F) -> MacroDeepResult<T> {
            self.ok_or_else(|| DeepDiagnostic::error_string(f()))
        }

        #[cfg(feature = "proc-macro2-diagnostics")]
        fn ok_or_error_with_at<F: Fn() -> String>(self, f: F, span: Span) -> MacroResult<T> {
            self.ok_or_else(|| span.error(f()))
        }

        #[allow(private_interfaces)]
        fn _seal(&self, _: &sealed::TraitParam) {}
    }
    impl OptionOrBoolExt<()> for bool {
        #[cfg(feature = "alloc")]
        fn ok_or_error_with<F: Fn() -> String>(self, f: F) -> MacroDeepResult<()> {
            // bool::ok_or_else is unstable: https://github.com/rust-lang/rust/issues/142748
            if self {
                Ok(())
            } else {
                Err(DeepDiagnostic::error_string(f()))
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
        fn _seal(&self, _: &sealed::TraitParam) {}
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
        fn _seal(&self, _: &sealed::TraitParam);
    }
    #[cfg(feature = "alloc")]
    impl<T: ToString> ToStringExt for T {
        fn to_error(&self) -> DeepDiagnostic {
            DeepDiagnostic::error_string(self.to_string())
        }
        fn to_error_with<F: Fn() -> String>(&self, f: F) -> DeepDiagnostic {
            let mut s = f();
            s.push(' ');
            s.push_str(&self.to_string());
            DeepDiagnostic::error_string(s)
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
        fn _seal(&self, _: &sealed::TraitParam) {}
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
        fn _seal(&self, _: &sealed::TraitParam);
    }
    #[cfg(feature = "alloc")]
    impl<T, E: ToString> ResultErrToStringExt<T> for Result<T, E> {
        fn map_error_to(self) -> MacroDeepResult<T> {
            self.map_err(|e| DeepDiagnostic::error_string(e.to_string()))
        }
        fn map_error_to_with<F: Fn() -> String>(self, f: F) -> MacroDeepResult<T> {
            self.map_err(|e| {
                let mut s = f();
                s.push(' ');
                s.push_str(&e.to_string());
                DeepDiagnostic::error_string(s)
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
        fn _seal(&self, _: &sealed::TraitParam) {}
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
        fn _seal(&self, _: &sealed::TraitParam);
    }

    #[cfg(feature = "alloc")]
    impl<T: Debug> DebugExt for T {
        #[cfg(feature = "alloc")]
        fn dbg_error(&self) -> DeepDiagnostic {
            DeepDiagnostic::error_string(format!("{self:?}"))
        }
        fn dbg_error_with<F: Fn() -> String>(self, f: F) -> DeepDiagnostic {
            let mut s = f();
            s.push(' ');
            s.push_str(&format!("{self:?}"));
            DeepDiagnostic::error_string(s)
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
        fn _seal(&self, _: &sealed::TraitParam) {}
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
        fn _seal(&self, _: &sealed::TraitParam);
    }
    #[cfg(feature = "alloc")]
    impl<T, E: Debug> ResultErrDebugExt<T> for Result<T, E> {
        fn map_error_dbg(self) -> MacroDeepResult<T> {
            self.map_err(|e| DeepDiagnostic::error_string(format!("{e:?}")))
        }
        fn map_error_dbg_with<F: Fn() -> String>(self, f: F) -> MacroDeepResult<T> {
            self.map_err(|e| {
                let mut s = f();
                s.push(' ');
                s.push_str(&format!("{e:?}"));
                DeepDiagnostic::error_string(s)
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
        fn _seal(&self, _: &sealed::TraitParam) {}
    }
}

pub mod assert {
    use crate::ext::OptionOrBoolExt;
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

pub mod sealed {
    /// Intentionally NOT public.
    pub(crate) struct TraitParam;
    pub trait Trait {
        #[allow(private_interfaces)]
        fn _seal(&self, _: &TraitParam);
    }
}
