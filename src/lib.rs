#![cfg_attr(not(feature = "proc-macro2"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::string::{String, ToString as _};

#[cfg(any(feature = "alloc", feature = "proc-macro2-diagnostics"))]
use core::any::Any;

#[cfg(feature = "proc-macro2")]
use proc_macro2::Span;

use core::fmt::{self, Display, Formatter};

#[cfg(feature = "proc-macro2-diagnostics")]
use proc_macro2_diagnostics::{Diagnostic as PmDiagnostic, Level};

#[cfg(not(feature = "alloc"))]
pub type DisplayishResult<T, D, EX = ()> = Result<T, Displayish<D, EX>>;

#[cfg(feature = "alloc")]
pub type DisplayishResult<T, D = String, EX = ()> = Result<T, Displayish<D, EX>>;

/// Intentionally NOT public - it will change if we ever support macro diagnostic levels other than [Level::Error].
#[cfg(feature = "proc-macro2-diagnostics")]
#[derive(Clone, Debug)]
pub struct LevelLike {
    _seal: Seal,
}
const LEVEL_LIKE: LevelLike = LevelLike { _seal: SEAL };

//-----
#[cfg(feature = "proc-macro2-diagnostics")]
pub type MacroDeepDiagnostic<D = String> = Displayish<D, LevelLike>;

#[cfg(feature = "proc-macro2-diagnostics")]
pub type MacroDeepResult<T, D = String> = Result<T, MacroDeepDiagnostic<D>>;

/// Like [proc_macro2_diagnostics::Diagnostic], but its [Displayish::display] is NOT converted to
/// [String], so that we convert it only at the top function call tree level. To convert use
/// [SpannedDiagnostic::into_diagnostic]. <--- @TODO this docs
#[cfg(feature = "proc-macro2-diagnostics")]
pub type MacroSpannedDiagnostic<D = String> = Displayish<D, (LevelLike, Span)>;

#[cfg(feature = "proc-macro2-diagnostics")]
pub type MacroDiagnosticResult<T> = Result<T, PmDiagnostic>;
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
impl<D: Display> Displayish<D> {
    pub fn new_from_display(display: impl Into<D>) -> Self {
        let display = display.into();
        let extra = ();
        Self { display, extra }
    }
    pub fn and_extra<E>(self, extra: E) -> Displayish<D, E> {
        let display = self.display;
        Displayish { display, extra }
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
impl<D: Display> From<D> for Displayish<D> {
    //@TODO DOC old:
    /// Move-and-construct/convert. If using `proc-macro2-diagnostics`, then [DeepDiagnostic::level]
    /// will be set to [Level::Error].
    fn from(display: D) -> Self {
        let extra = ();
        Self { display, extra }
    }
}

//------

#[cfg(feature = "proc-macro2-diagnostics")]
impl<D: Display> MacroDeepDiagnostic<D> {
    pub fn spanned(self, span: Span) -> MacroSpannedDiagnostic<D> {
        MacroSpannedDiagnostic {
            display: self.display,
            extra: (self.extra, span),
        }
    }
}

/// Intentionally NOT public - used to indicate a sealed trait/struct.
#[derive(Clone, Debug)]
struct Seal;
const SEAL: Seal = Seal;
//--------

#[cfg(feature = "proc-macro2-diagnostics")]
impl<D: Display> MacroSpannedDiagnostic<D> {
    pub fn into_diagnostic(self) -> PmDiagnostic {
        PmDiagnostic::spanned(self.extra.1, Level::Error, self.display.to_string())
    }
}

pub mod ext;

pub mod prelude_ext;

pub mod assert {
    use crate::ext::core::OptionOrBoolExt;
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
