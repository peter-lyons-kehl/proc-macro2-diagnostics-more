#![cfg_attr(not(feature = "proc-macro2-diagnostics"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::string::String;

use core::fmt::{self, Display, Formatter};

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

#[cfg(feature = "alloc")]
#[derive(Clone, Debug)]
struct DeepDiagnosticMessage(#[cfg(feature = "alloc")] String);

#[derive(Clone, Debug)]
pub struct DeepDiagnostic {
    #[cfg(feature = "proc-macro2-diagnostics")]
    level: proc_macro2_diagnostics::Level,
    #[cfg(feature = "alloc")]
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

    #[cfg(feature = "alloc")]
    pub fn to_string_direct(&self) -> String {
        self.message.0.clone()
    }
    #[cfg(feature = "alloc")]
    pub fn to_string_move(self) -> String {
        self.message.0
    }
}
impl Display for DeepDiagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "alloc")]
        self.message.0.fmt(f)?;
        #[cfg(not(feature = "alloc"))]
        "proc_macro2_diagnostics_more::DeepDiagnostic".fmt(f)?;
        Ok(())
    }
}

fn _f<R: Display, F: Fn() -> R>(_callback: F) {}

//pub trait DeepMessage
fn _caller() -> Result<(), (impl Display, impl Display)> {
    _callee()
}
fn _callee() -> Result<(), (impl Display, impl Display)> {
    Ok::<(), (&str, bool)>(())
}

pub trait Displays02Trait: Display {
    type T01: Display;
    type T02: Display;
}
pub trait DisplayOther: Display {} //@TODO make sealed
impl<T: Display> DisplayOther for T {}

/// Default type param for [Displays02Plus]'s generic param `OTHER`. We can't use unit type `()`,
/// because Rust may add [Display] `impl` for it later.
///
/// But we need [Display] to by auto-implemented for any types that `impl` [DisplayOther]. The
/// choices are
/// - [DisplayOther] NOT extending [Display], and a blanket `impl` of [DisplayOther] for any type
///   implementing [Display] - but then not possible to use unit type `()` as a default generic
///   param for [Displays02Plus]; or
/// - [DisplayOther] extending [Display], and the same blanket `impl` as considered above; then
///   [Displays02Plus] has the default generic param [Never] that we we manually implement [Display]
///   for, so that [Never] does gets its blanket [DisplayOther].
///
/// NOT exactly like Rust's "never" type. Currently will most likely NOT be optimized out in enum
/// variants etc. But it may be replaced with Rust "never" type once that is stable.
pub struct Never(());
impl Display for Never {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

pub enum Displays02Plus<T01: Display, T02: Display, OTHER: DisplayOther = Never> {
    //@TODO separate enum, and wrap transparent
    T01(T01),
    T02(T02),
    Other(OTHER),
}
/// Like [core::convert::From], but NOT reflective, so that we don't get conflicts concerning
/// [Displays02Plus] and friends.
///
/// Just like with [core::convert::From], prefer to implement [ImplFrom] over [IntoImpl], as there
/// is a blanket `impl` of [IntoImpl] for any type that implements [ImplFrom].
pub trait ImplFrom<F> {
    //@TODO seal?
    fn impl_from(f: F) -> Self;
}
/// Like [core::convert::Into], but NOT reflective, so that we don't get conflicts concerning
/// [Displays02Plus] and friends.
///
/// Just like with [core::convert::From], prefer to implement [ImplFrom] over [IntoImpl], as there
/// is a blanket `impl` of [IntoImpl] for any type that implements [ImplFrom].
pub trait IntoImpl<I> {
    //@TODO seal?
    fn into_impl(self) -> I;
}
impl<F, I: ImplFrom<F>> IntoImpl<I> for F {
    fn into_impl(self) -> I {
        I::impl_from(self)
    }
}
/*impl<T01: Display, T02: Display, OTHER: DisplayOther, FROM> From<FROM>
for Displays02Plus<T01, T02, OTHER>
// \--- that was generating conflicts with core::convert blanket impl of From<T> for T.
*/
impl<T01: Display, T02: Display, OTHER: DisplayOther, FROM> ImplFrom<FROM>
    for Displays02Plus<T01, T02, OTHER>
where
    OTHER: From<FROM>,
{
    fn impl_from(f: FROM) -> Self {
        Self::Other(f.into())
    }
}

pub type Displays02<T01, T02> = Displays02Plus<T01, T02, Never>;

impl<T01: Display, T02: Display, OTHER: Display> Displays02Plus<T01, T02, OTHER> {
    pub fn new_01(v: T01) -> Self {
        Self::T01(v)
    }
    pub fn new_02(v: T02) -> Self {
        Self::T02(v)
    }

    // @TODO separate function name for each trait; OR: support one trait only - user can have blanket impl.
    //
    // @TODO inner = by impl only; inner_mut
    fn inner_ref(&self) -> &dyn Display {
        match self {
            Self::T01(inner) => inner,
            Self::T02(inner) => inner,
            Self::Other(inner) => inner,
        }
    }

    fn by_ref<R, F: FnOnce(&dyn Display) -> R>(&self, apply: F) -> R {
        apply(self.inner_ref())
    }
    /* Not possible: fn pointer can't use `impl TraitXyz`

    fn by_impl_01<A01, R>(&self, apply: fn(&impl Display, A01), a01: A01) -> R {
        apply(self.inner_ref())
    }*/
}
impl<T01: Display, T02: Display> Display for Displays02Plus<T01, T02> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Self::by_ref(&self, |s| s.fmt(f))
    }
}
impl<T01: Display, T02: Display> Displays02Trait for Displays02Plus<T01, T02> {
    type T01 = T01;
    type T02 = T02;
}
/*impl<T01: Display, T02: Display> From<T01> for Displays02<T01, T02> {
    fn from(value: T01) -> Self {
        Self::new_01(value)
    }
}*/
// CONFLICTING:
/*impl<T01: Display, T02: Display> From<T02> for Displays02<T01, T02> {
    fn from(value: T02) -> Self {
        Self::new_02(value)
    }
}
// foreign trait:
impl<T01: Display, T02: Display> Into<Displays02<T01, T02>> for T01 {
    fn into(self) -> Displays02<T01, T02> {
        Displays02::new_01(self)
    }
}*/
/*pub trait MoveIntoDisplays02 {
    fn move_into<T01: Display, T02: Display>(self) -> Displays02<T01, T02>;
}
impl<T01: Display> MoveIntoDisplays02 for T01 {
    fn move_into<T02: Display>(self) -> Displays02<T01, T02> {
        todo!()
    }
}*/

/// 8-bit [Display] values. Not excellent for mass storage as the enum determinant also takes 8 bits.
pub enum Display8Bits {
    Bool(bool),
    U8(u8),
    I8(i8),
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
/// pub trait DisplayAndError: Display + core::error::Error {}
/// impl<T: Display + core::error::Error> DisplayAndError for T {}
/// ```
pub mod by_dyn {
    macro_rules! variants {
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
    }
}

pub fn ret_disp() -> impl Display {
    let _ = if true {
        Displays02::new_01(true) //@TODO:
                                 // -extension method for T01, T02... - blanket for all Sized
                                 // -extension method for Result<..., ...> success
                                 // -extension method for Result<..., ...> error
    } else {
        Displays02::new_02("hi")
    };
    if true {
        Displays02::new_01(1.2)
    } else {
        let value = 1;
        Displays02::new_02(DisplayFromFn::new(move |f| write!(f, "hi {value}")))
    }
}

pub fn ret_result_displ() -> Result<(), impl Display> {
    ret_result_displ2trait()
}
pub fn ret_result_displ2trait() -> Result<(), impl Displays02Trait> {
    //pub fn ret_result() -> Result<(), impl Display> {
    let result_1 = Err(if true {
        // @TODO Displays02: take a generic param like Display8, Display16, Display32...
        // - all implement a tiny trait DisplayFixed
        //
        // then have blanket:
        //
        // impl<F, DF: DisplayFixed + From<F>> From<F> for Displays02<DF> { forward-here }
        Displays02Plus::new_01(true) //@TODO:
                                     // -extension method for T01, T02... - blanket for all Sized
                                     // -extension method for Result<..., ...> success
                                     // -extension method for Result<..., ...> error
    } else {
        Displays02Plus::new_02("hi")
    });
    let _ = result_1?;

    let result_2 = Err(if true {
        Displays02Plus::new_01(false)
    } else {
        //let value = 1;
        Displays02Plus::new_02("bye")
        // DisplayFromFn::new(move |f| write!(f, "hi {value}"))
    });
    //let _ = result_2?;
    //
    //Ok(())
    result_2
}

#[repr(transparent)]
pub struct DisplayFromFn<F: Fn(&mut Formatter<'_>) -> Result<(), core::fmt::Error>>(F);
impl<F: Fn(&mut Formatter<'_>) -> Result<(), core::fmt::Error>> DisplayFromFn<F> {
    pub fn new(f: F) -> Self {
        Self(f)
    }
}
impl<F: Fn(&mut Formatter<'_>) -> Result<(), core::fmt::Error>> Display for DisplayFromFn<F> {
    fn fmt(&self, fm: &mut Formatter<'_>) -> fmt::Result {
        self.0(fm)
    }
}
pub fn display_from_fn(
    f: impl Fn(&mut Formatter<'_>) -> Result<(), core::fmt::Error>,
) -> impl Display {
    DisplayFromFn::new(f)
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
    fn from(deep: DeepDiagnostic) -> Self {
        deep.message.0
    }
}

pub mod ext {
    #[cfg(feature = "proc-macro2-diagnostics")]
    use crate::MacroResult;
    use crate::{sealed, MacroDeepResult};

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
    #[cfg(feature = "alloc")]
    use crate::ext::OptionOrBoolExt;
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

pub mod sealed {
    /// Intentionally NOT public.
    pub(crate) struct TraitParam;
    pub trait Trait {
        #[allow(private_interfaces)]
        fn _seal(&self, _: &TraitParam);
    }
}
