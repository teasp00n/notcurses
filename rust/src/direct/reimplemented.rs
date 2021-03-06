//! `ncdirect_*` reimplemented functions.

use core::ptr::null;

use crate::{NcColor, NcDirect, NcInput, NcIntResult, NcRgb, NcSignalSet, NcTime};

///
/// If no event is ready, returns 0.
///
/// *Method: NcDirect.[getc_nblock()][NcDirect#method.getc_nblock].*
//
// `input` may be NULL if the caller is uninterested in event details.
#[inline]
pub fn ncdirect_getc_nblock(nc: &mut NcDirect, input: &mut NcInput) -> char {
    unsafe {
        let mut sigmask = NcSignalSet::new();
        sigmask.fillset();
        let ts = NcTime::new();
        core::char::from_u32_unchecked(crate::ncdirect_getc(nc, &ts, &mut sigmask, input))
    }
}

/// 'input' may be NULL if the caller is uninterested in event details.
/// Blocks until an event is processed or a signal is received.
///
/// *Method: NcDirect.[getc_nblocking()][NcDirect#method.getc_nblocking].*
#[inline]
pub fn ncdirect_getc_nblocking(nc: &mut NcDirect, input: &mut NcInput) -> char {
    unsafe {
        let mut sigmask = NcSignalSet::new();
        sigmask.emptyset();
        core::char::from_u32_unchecked(crate::ncdirect_getc(nc, null(), &mut sigmask, input))
    }
}

/// Sets the foreground [NcColor] components.
///
/// *Method: NcDirect.[set_fg_rgb8()][NcDirect#method.set_fg_rgb8].*
#[inline]
pub fn ncdirect_set_fg_rgb8(
    ncd: &mut NcDirect,
    red: NcColor,
    green: NcColor,
    blue: NcColor,
) -> NcIntResult {
    let rgb = (red as NcRgb) << 16 | (green as NcRgb) << 8 | blue as NcRgb;
    unsafe { crate::ncdirect_set_fg_rgb(ncd, rgb) }
}

/// Sets the background [NcColor] components.
///
/// *Method: NcDirect.[set_bg_rgb8()][NcDirect#method.set_bg_rgb8].*
#[inline]
pub fn ncdirect_set_bg_rgb8(
    ncd: &mut NcDirect,
    red: NcColor,
    green: NcColor,
    blue: NcColor,
) -> NcIntResult {
    let rgb = (red as NcRgb) << 16 | (green as NcRgb) << 8 | blue as NcRgb;
    unsafe { crate::ncdirect_set_bg_rgb(ncd, rgb) }
}
