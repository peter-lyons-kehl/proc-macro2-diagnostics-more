use crate::{Displayish, DisplayishResult, Seal};
use alloc::{format, string::String, string::ToString};

use core::fmt::Display;

pub trait ToStringExt: ToString {
    fn to_error(&self) -> Displayish;
    fn to_error_with<FD: Display, F: Fn() -> FD>(&self, f: F) -> Displayish;

    #[allow(private_interfaces)]
    fn _seal(&self, _: Seal);
}

impl<T: ToString> ToStringExt for T {
    fn to_error(&self) -> Displayish {
        Displayish::new_from_display(self.to_string())
    }
    fn to_error_with<FD: Display, F: Fn() -> FD>(&self, f: F) -> Displayish {
        let s = format!("{} {}", f(), self.to_string());
        Displayish::new_from_display(s)
    }

    #[allow(private_interfaces)]
    fn _seal(&self, _: Seal) {}
}

pub trait ResultErrToDisplayExt<T> {
    fn map_error_to(self) -> DisplayishResult<T>;
    fn map_error_to_with<FD: Display, F: Fn() -> FD>(self, f: F) -> DisplayishResult<T>;

    #[allow(private_interfaces)]
    fn _seal(&self, _: Seal);
}
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

    #[allow(private_interfaces)]
    fn _seal(&self, _: Seal) {}
}

pub trait ResultDisplayToString<T, EX> {
    fn to_string_based(self) -> DisplayishResult<T, String, EX>;

    #[allow(private_interfaces)]
    fn _seal(&self, _: Seal) {}
}
impl<T, D: Display, EX> ResultDisplayToString<T, EX> for DisplayishResult<T, D, EX> {
    fn to_string_based(self) -> DisplayishResult<T, String, EX> {
        self.map_err(|e| e.to_string_based())
    }

    #[allow(private_interfaces)]
    fn _seal(&self, _: Seal) {}
}
