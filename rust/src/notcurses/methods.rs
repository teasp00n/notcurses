//! `Notcurses*` methods and associated functions.

use core::ptr::{null, null_mut};

use crate::{
    cstring, error, error_ref_mut, notcurses_init, rstring, NcAlign, NcBlitter, NcChannelPair,
    NcDimension, NcEgc, NcError, NcFile, NcInput, NcLogLevel, NcPlane, NcResult, NcScale,
    NcSignalSet, NcStats, NcStyleMask, NcTime, Notcurses, NotcursesOptions,
    NCOPTION_NO_ALTERNATE_SCREEN, NCOPTION_SUPPRESS_BANNERS, NCRESULT_ERR,
};

/// # `NotcursesOptions` Constructors
impl NotcursesOptions {
    /// New NotcursesOptions.
    pub const fn new() -> Self {
        Self::with_all_options(0, 0, 0, 0, 0, 0)
    }

    /// New NotcursesOptions, with margins.
    pub const fn with_margins(
        top: NcDimension,
        right: NcDimension,
        bottom: NcDimension,
        left: NcDimension,
    ) -> Self {
        Self::with_all_options(0, top, right, bottom, left, 0)
    }

    /// New NotcursesOptions, with flags.
    pub const fn with_flags(flags: u64) -> Self {
        Self::with_all_options(0, 0, 0, 0, 0, flags)
    }

    /// New NotcursesOptions, with all the options.
    ///
    /// ## Arguments
    ///
    /// - loglevel
    ///
    ///   Progressively higher log levels result in more logging to stderr. By
    ///   default, nothing is printed to stderr once fullscreen service begins.
    ///
    /// - margin_t, margin_r, margin_b, margin_l
    ///
    ///   Desirable margins (top, right, bottom, left).
    ///
    ///   If all are 0 (default), we will render to the entirety of the screen.
    ///   If the screen is too small, we do what we can.
    ///   Absolute coordinates are relative to the rendering area
    ///   ((0, 0) is always the origin of the rendering area).
    ///
    /// - flags
    ///
    ///   General flags; This is expressed as a bitfield so that future options
    ///   can be added without reshaping the struct.
    ///   Undefined bits must be set to 0.
    ///
    ///   - [`NCOPTION_INHIBIT_SETLOCALE`][crate::NCOPTION_INHIBIT_SETLOCALE]
    ///   - [`NCOPTION_NO_ALTERNATE_SCREEN`]
    ///   - [`NCOPTION_NO_FONT_CHANGES`][crate::NCOPTION_NO_FONT_CHANGES]
    ///   - [`NCOPTION_NO_QUIT_SIGHANDLERS`][crate::NCOPTION_NO_QUIT_SIGHANDLERS]
    ///   - [`NCOPTION_NO_WINCH_SIGHANDLER`][crate::NCOPTION_NO_WINCH_SIGHANDLER]
    ///   - [`NCOPTION_SUPPRESS_BANNERS`]
    ///
    pub const fn with_all_options(
        loglevel: NcLogLevel,
        margin_t: NcDimension,
        margin_r: NcDimension,
        margin_b: NcDimension,
        margin_l: NcDimension,
        flags: u64,
    ) -> Self {
        Self {
            termtype: null(),
            renderfp: null_mut(),
            loglevel,
            margin_t: margin_t as i32,
            margin_r: margin_r as i32,
            margin_b: margin_b as i32,
            margin_l: margin_l as i32,
            flags,
        }
    }
}

/// # `Notcurses` Constructors
impl Notcurses {
    /// Returns a Notcurses context (without banners).
    pub fn new<'a>() -> NcResult<&'a mut Notcurses> {
        Self::with_flags(NCOPTION_SUPPRESS_BANNERS)
    }

    /// Returns a Notcurses context, with banners. The default in the C library.
    pub fn with_banners<'a>() -> NcResult<&'a mut Notcurses> {
        Self::with_flags(0)
    }

    /// Returns a Notcurses context, without an alternate screen (nor banners).
    pub fn without_altscreen<'a>() -> NcResult<&'a mut Notcurses> {
        Self::with_flags(NCOPTION_NO_ALTERNATE_SCREEN)
    }

    /// Returns a Notcurses context, without an alternate screen, with banners.
    pub fn without_altscreen_nor_banners<'a>() -> NcResult<&'a mut Notcurses> {
        Self::with_flags(NCOPTION_NO_ALTERNATE_SCREEN | NCOPTION_SUPPRESS_BANNERS)
    }

    /// Returns a Notcurses context, expects [NotcursesOptions].
    pub fn with_flags<'a>(flags: u64) -> NcResult<&'a mut Notcurses> {
        Self::with_options(NotcursesOptions::with_flags(flags))
    }

    /// Returns a Notcurses context, expects [NotcursesOptions].
    pub fn with_options<'a>(options: NotcursesOptions) -> NcResult<&'a mut Notcurses> {
        let res = unsafe { notcurses_init(&options, null_mut()) };
        error_ref_mut![res, "Initializing Notcurses"]
    }

    /// Returns a Notcurses context. Expects [NcLogLevel] and flags.
    pub fn with_debug<'a>(loglevel: NcLogLevel, flags: u64) -> NcResult<&'a mut Notcurses> {
        Self::with_options(NotcursesOptions::with_all_options(
            loglevel, 0, 0, 0, 0, flags,
        ))
    }
}

/// # `Notcurses` methods
impl Notcurses {
    /// Returns the offset into `availcols` at which `cols` ought be output given
    /// the requirements of `align`.
    ///
    /// Returns `-`[`NCRESULT_MAX`][crate::NCRESULT_MAX] if
    /// [NCALIGN_UNALIGNED][crate::NCALIGN_UNALIGNED] or invalid [NcAlign].
    ///
    /// *C style function: [notcurses_align()][crate::notcurses_align].*
    pub fn align(availcols: NcDimension, align: NcAlign, cols: NcDimension) -> NcResult<()> {
        error![crate::notcurses_align(availcols, align, cols)]
    }

    /// Retrieves the current contents of the specified [NcCell][crate::NcCell]
    /// as last rendered, returning the [NcEgc] (or None on error) and writing
    /// out the [NcStyleMask] and the [NcChannelPair].
    ///
    /// This NcEgc must be freed by the caller.
    ///
    /// *C style function: [notcurses_at_yx()][crate::notcurses_at_yx].*
    pub fn at_yx(
        &mut self,
        y: NcDimension,
        x: NcDimension,
        stylemask: &mut NcStyleMask,
        channels: &mut NcChannelPair,
    ) -> Option<NcEgc> {
        let egc = unsafe { crate::notcurses_at_yx(self, x as i32, y as i32, stylemask, channels) };
        if egc.is_null() {
            return None;
        }
        let egc = core::char::from_u32(unsafe { *egc } as u32).expect("wrong char");
        Some(egc)
    }

    /// Returns the bottommost [NcPlane], of which there is always at least one.
    ///
    /// *C style function: [notcurses_bottom()][crate::notcurses_bottom].*
    pub fn bottom<'a>(&'a mut self) -> &'a mut NcPlane {
        unsafe { &mut *crate::notcurses_bottom(self) }
    }

    /// Returns true if it's possible to set the "hardware" palette.
    ///
    /// Requires the "ccc" terminfo capability.
    ///
    /// *C style function: [notcurses_canchangecolor()][crate::notcurses_canchangecolor].*
    pub fn canchangecolor(&self) -> bool {
        unsafe { crate::notcurses_canchangecolor(self) }
    }

    /// Returns true if fading is possible.
    ///
    /// Fading requires either the "rgb" or "ccc" terminfo capability.
    ///
    /// *C style function: [notcurses_canfade()][crate::notcurses_canfade].*
    pub fn canfade(&self) -> bool {
        unsafe { crate::notcurses_canfade(self) }
    }

    /// Returns true if loading images is possible.
    ///
    /// This requires being built against FFmpeg/OIIO.
    ///
    /// *C style function: [notcurses_canopen_images()][crate::notcurses_canopen_images].*
    pub fn canopen_images(&self) -> bool {
        unsafe { crate::notcurses_canopen_images(self) }
    }

    /// Returns true if loading videos is possible.
    ///
    /// This requires being built against FFmpeg.
    ///
    /// *C style function: [notcurses_canopen_videos()][crate::notcurses_canopen_videos].*
    pub fn canopen_videos(&self) -> bool {
        unsafe { crate::notcurses_canopen_videos(self) }
    }

    /// Returns true if sixel blitting is supported.
    ///
    /// See [NCBLIT_SIXEL][crate::NCBLIT_SIXEL].
    ///
    /// *C style function: [notcurses_cansixel()][crate::notcurses_cansixel].*
    pub fn cansixel(&self) -> bool {
        unsafe { crate::notcurses_cansixel(self) }
    }

    /// Returns true if we can reliably use Unicode 13 sextants.
    ///
    /// *C style function: [notcurses_cansextant()][crate::notcurses_cansextant].*
    pub fn cansextant(&self) -> bool {
        unsafe { crate::notcurses_cansextant(self) }
    }

    /// Returns true if it's possible to directly specify RGB values per cell,
    /// or false if it's only possible to use palettes.
    ///
    /// *C style function: [notcurses_cantruecolor()][crate::notcurses_cantruecolor].*
    pub fn cantruecolor(&self) -> bool {
        unsafe { crate::notcurses_cantruecolor(self) }
    }

    /// Returns true if the encoding is UTF-8.
    ///
    /// Requires `LANG` being set to a UTF-8 locale.
    ///
    /// *C style function: [notcurses_canutf8()][crate::notcurses_canutf8].*
    pub fn canutf8(&self) -> bool {
        unsafe { crate::notcurses_canutf8(self) }
    }

    /// Disables the terminal's cursor, if supported.
    ///
    /// Immediate effect (no need for a call to notcurses_render()).
    ///
    /// *C style function: [notcurses_cursor_disable()][crate::notcurses_cursor_disable].*
    pub fn cursor_disable(&mut self) -> NcResult<()> {
        error![unsafe { crate::notcurses_cursor_disable(self) }]
    }

    /// Enables the terminal's cursor, if supported, placing it at `y`, `x`.
    ///
    /// Immediate effect (no need for a call to notcurses_render()).
    /// It is an error if `y`, `x` lies outside the standard plane.
    ///
    /// *C style function: [notcurses_cursor_enable()][crate::notcurses_cursor_enable].*
    pub fn cursor_enable(&mut self, y: NcDimension, x: NcDimension) -> NcResult<()> {
        error![unsafe { crate::notcurses_cursor_enable(self, y as i32, x as i32) }]
    }

    /// Dumps Notcurses state to the supplied `debugfp`.
    ///
    /// Output is freeform, and subject to change. It includes geometry of all
    /// planes, from all piles.
    ///
    /// *C style function: [notcurses_debug()][crate::notcurses_debug].*
    pub fn debug(&mut self, debugfp: &mut NcFile) {
        unsafe {
            crate::notcurses_debug(self, debugfp.as_nc_ptr());
        }
    }

    /// Destroys all [NcPlane]s other than the stdplane.
    ///
    /// *C style function: [notcurses_drop_planes()][crate::notcurses_drop_planes].*
    pub fn drop_planes(&mut self) {
        unsafe {
            crate::notcurses_drop_planes(self);
        }
    }

    /// Returns a [char] representing a single unicode point.
    ///
    /// If an event is processed, the return value is the `id` field from that
    /// event.
    ///
    /// Provide a None `time` to block at length, a `time` of 0 for non-blocking
    /// operation, and otherwise a timespec to bound blocking.
    ///
    /// Signals in sigmask (less several we handle internally) will be atomically
    /// masked and unmasked per [ppoll(2)](https://linux.die.net/man/2/ppoll).
    ///
    /// `*sigmask` should generally contain all signals.
    ///
    /// *C style function: [notcurses_getc()][crate::notcurses_getc].*
    pub fn getc(
        &mut self,
        time: Option<NcTime>,
        sigmask: Option<&mut NcSignalSet>,
        input: Option<&mut NcInput>,
    ) -> NcResult<char> {
        let ntime;
        if let Some(time) = time {
            ntime = &time as *const _;
        } else {
            ntime = null();
        }

        let nsigmask;
        if let Some(sigmask) = sigmask {
            nsigmask = sigmask as *mut _;
        } else {
            nsigmask = null_mut() as *mut _;
        }
        let ninput;
        if let Some(input) = input {
            ninput = input as *mut _;
        } else {
            ninput = null_mut();
        }
        let c = unsafe {
            core::char::from_u32_unchecked(crate::notcurses_getc(self, ntime, nsigmask, ninput))
        };
        if c as u32 as i32 == NCRESULT_ERR {
            return Err(NcError::new(NCRESULT_ERR));
        }
        Ok(c)
    }

    ///
    /// *C style function: [notcurses_getc_nblock()][crate::notcurses_getc_nblock].*
    pub fn getc_nblock(&mut self, input: &mut NcInput) -> char {
        crate::notcurses_getc_nblock(self, input)
    }

    ///
    /// *C style function: [notcurses_getc_nblocking()][crate::notcurses_getc_nblocking].*
    pub fn getc_nblocking(&mut self, input: &mut NcInput) -> char {
        crate::notcurses_getc_nblocking(self, input)
    }

    /// Gets a file descriptor suitable for input event poll()ing.
    ///
    /// When this descriptor becomes available, you can call
    /// [getc_nblock()][Notcurses#method.getc_nblock], and input ought be ready.
    ///
    /// This file descriptor is not necessarily the file descriptor associated
    /// with stdin (but it might be!).
    ///
    /// *C style function: [notcurses_inputready_fd()][crate::notcurses_inputready_fd].*
    pub fn inputready_fd(&mut self) -> NcResult<()> {
        error![unsafe { crate::notcurses_inputready_fd(self) }]
    }

    /// Returns an [NcBlitter] from a string representation.
    ///
    /// *C style function: [notcurses_lex_blitter()][crate::notcurses_lex_blitter].*
    pub fn lex_blitter(op: &str) -> NcResult<NcBlitter> {
        let mut blitter = 0;
        error![
            unsafe { crate::notcurses_lex_blitter(cstring![op], &mut blitter) },
            blitter, "Invalid blitter name"
        ]
    }

    /// Lexes a margin argument according to the standard Notcurses definition.
    ///
    /// There can be either a single number, which will define all margins equally,
    /// or there can be four numbers separated by commas.
    ///
    /// *C style function: [notcurses_lex_margins()][crate::notcurses_lex_margins].*
    pub fn lex_margins(op: &str, options: &mut NotcursesOptions) -> NcResult<()> {
        error![unsafe { crate::notcurses_lex_margins(cstring![op], options) }]
    }

    /// Returns an [NcScale] from a string representation.
    ///
    /// *C style function: [notcurses_lex_scalemode()][crate::notcurses_lex_scalemode].*
    pub fn lex_scalemode(op: &str) -> NcResult<NcScale> {
        let mut scalemode = 0;
        error![
            unsafe { crate::notcurses_lex_scalemode(cstring![op], &mut scalemode) },
            scalemode
        ]
    }

    /// Disables signals originating from the terminal's line discipline, i.e.
    /// SIGINT (^C), SIGQUIT (^), and SIGTSTP (^Z). They are enabled by default.
    ///
    /// *C style function: [notcurses_linesigs_disable()][crate::notcurses_linesigs_disable].*
    pub fn linesigs_disable(&mut self) -> NcResult<()> {
        error![unsafe { crate::notcurses_linesigs_disable(self) }]
    }

    /// Restores signals originating from the terminal's line discipline, i.e.
    /// SIGINT (^C), SIGQUIT (^), and SIGTSTP (^Z), if disabled.
    ///
    /// *C style function: [notcurses_linesigs_enable()][crate::notcurses_linesigs_enable].*
    pub fn linesigs_enable(&mut self) -> NcResult<()> {
        error![unsafe { crate::notcurses_linesigs_enable(self) }]
    }

    /// Disables mouse events.
    ///
    /// Any events in the input queue can still be delivered.
    ///
    /// *C style function: [notcurses_mouse_disable()][crate::notcurses_mouse_disable].*
    pub fn mouse_disable(&mut self) -> NcResult<()> {
        error![unsafe { crate::notcurses_mouse_disable(self) }]
    }

    /// Enable the mouse in "button-event tracking" mode with focus detection
    /// and UTF8-style extended coordinates.
    ///
    /// On success, mouse events will be published to [getc()][Notcurses#method.getc].
    ///
    /// *C style function: [notcurses_mouse_enable()][crate::notcurses_mouse_enable].*
    pub fn mouse_enable(&mut self) -> NcResult<()> {
        error![unsafe { crate::notcurses_mouse_enable(self) }]
    }

    /// Returns the number of simultaneous colors claimed to be supported,
    /// or 1 if there is no color support.
    ///
    /// Note that several terminal emulators advertise more colors than they
    /// actually support, downsampling internally.
    ///
    /// *C style function: [notcurses_palette_size()][crate::notcurses_palette_size].*
    pub fn palette_size(&mut self) -> u32 {
        unsafe { crate::notcurses_palette_size(self) }
    }

    /// Refreshes the physical screen to match what was last rendered (i.e.,
    /// without reflecting any changes since the last call to
    /// [render][crate::Notcurses#method.render]).
    ///
    /// This is primarily useful if the screen is externally corrupted, or if an
    /// [NCKEY_RESIZE][crate::NCKEY_RESIZE] event has been read and you're not
    /// yet ready to render.
    ///
    /// *C style function: [notcurses_refresh()][crate::notcurses_refresh].*
    //
    pub fn refresh(&mut self) -> NcResult<(NcDimension, NcDimension)> {
        let (mut y, mut x) = (0, 0);
        error![
            unsafe { crate::notcurses_refresh(self, &mut y, &mut x) },
            (y as NcDimension, x as NcDimension)
        ]
    }

    /// Renders and rasterizes the standard pile in one shot. Blocking call.
    ///
    /// *C style function: [notcurses_render()][crate::notcurses_render].*
    pub fn render(&mut self) -> NcResult<()> {
        error![unsafe { crate::notcurses_render(self) }]
    }

    /// Performs the rendering and rasterization portion of
    /// [render][Notcurses#method.render] but do not write the resulting buffer
    /// out to the terminal.
    ///
    /// Using this function, the user can control the writeout process,
    /// and render a second frame while writing another.
    ///
    /// The returned buffer must be freed by the caller.
    ///
    /// *C style function: [notcurses_render_to_buffer()][crate::notcurses_render_to_buffer].*
    //
    // CHECK that this works.
    pub fn render_to_buffer(&mut self, buffer: &mut Vec<u8>) -> NcResult<()> {
        let mut len = buffer.len() as u64;
        let mut buf = buffer.as_mut_ptr() as *mut i8;
        error![unsafe { crate::notcurses_render_to_buffer(self, &mut buf, &mut len) }]
    }

    /// Writes the last rendered frame, in its entirety, to 'fp'.
    ///
    /// If [render()][Notcurses#method.render] has not yet been called,
    /// nothing will be written.
    ///
    /// *C style function: [notcurses_render_to_file()][crate::notcurses_render_to_file].*
    pub fn render_to_file(&mut self, fp: &mut NcFile) -> NcResult<()> {
        error![unsafe { crate::notcurses_render_to_file(self, fp.as_nc_ptr()) }]
    }

    /// Acquires an atomic snapshot of the Notcurses object's stats.
    ///
    /// *C style function: [notcurses_stats()][crate::notcurses_stats].*
    pub fn stats(&mut self, stats: &mut NcStats) {
        unsafe {
            crate::notcurses_stats(self, stats);
        }
    }

    /// Allocates an ncstats object.
    ///
    /// Use this rather than allocating your own, since future versions of
    /// Notcurses might enlarge this structure.
    ///
    /// *C style function: [notcurses_stats_alloc()][crate::notcurses_stats_alloc].*
    pub fn stats_alloc<'a>(&'a mut self) -> &'a mut NcStats {
        unsafe { &mut *crate::notcurses_stats_alloc(self) }
    }

    /// Resets all cumulative stats (immediate ones, such as fbbytes, are not reset).
    ///
    /// *C style function: [notcurses_stats_reset()][crate::notcurses_stats_reset].*
    pub fn stats_reset(&mut self, stats: &mut NcStats) {
        unsafe {
            crate::notcurses_stats_reset(self, stats);
        }
    }

    /// [notcurses_stdplane()][crate::notcurses_stdplane], plus free bonus
    /// dimensions written to non-NULL y/x!
    ///
    /// *C style function: [notcurses_stddim_yx()][crate::notcurses_stddim_yx].*
    #[inline]
    pub fn stddim_yx<'a>(
        &'a mut self,
        y: &mut NcDimension,
        x: &mut NcDimension,
    ) -> NcResult<&'a mut NcPlane> {
        crate::notcurses_stddim_yx(self, y, x)
    }

    /// [stdplane_const()][Notcurses#method.stdplane_const], plus free
    /// bonus dimensions written to non-NULL y/x!
    ///
    /// *C style function: [notcurses_stddim_yx()][crate::notcurses_stddim_yx].*
    #[inline]
    pub fn stddim_yx_const<'a>(
        &'a self,
        y: &mut NcDimension,
        x: &mut NcDimension,
    ) -> NcResult<&'a NcPlane> {
        crate::notcurses_stddim_yx_const(self, y, x)
    }

    /// Returns a mutable reference to the standard [NcPlane] for this terminal.
    ///
    /// The standard plane always exists, and its origin is always at the
    /// uppermost, leftmost cell.
    ///
    /// *C style function: [notcurses_stdplane()][crate::notcurses_stdplane].*
    pub fn stdplane<'a>(&mut self) -> NcResult<&'a mut NcPlane> {
        error_ref_mut![unsafe { crate::notcurses_stdplane(self) }]
    }

    /// Returns a reference to the standard [NcPlane] for this terminal.
    ///
    /// The standard plane always exists, and its origin is always at the
    /// uppermost, leftmost cell.
    ///
    /// *C style function: [notcurses_stdplane_const()][crate::notcurses_stdplane_const].*
    pub fn stdplane_const<'a>(&self) -> &'a NcPlane {
        unsafe { &*crate::notcurses_stdplane_const(self) }
    }

    /// Destroys the Notcurses context.
    ///
    /// *C style function: [notcurses_stop()][crate::notcurses_stop].*
    pub fn stop(&mut self) -> NcResult<()> {
        error![unsafe { crate::notcurses_stop(self) }]
    }

    /// Gets the name of an [NcBlitter] blitter.
    ///
    /// *C style function: [notcurses_str_blitter()][crate::notcurses_str_blitter].*
    pub fn str_blitter(blitter: NcBlitter) -> String {
        rstring![crate::notcurses_str_blitter(blitter)].to_string()
    }

    /// Gets the name of an [NcScale] scaling mode.
    ///
    /// *C style function: [notcurses_str_scalemode()][crate::notcurses_str_scalemode].*
    pub fn str_scalemode(scalemode: NcScale) -> String {
        rstring![crate::notcurses_str_scalemode(scalemode)].to_string()
    }

    /// Returns an [NcStyleMask] with the supported curses-style attributes.
    ///
    /// The attribute is only indicated as supported if the terminal can support
    /// it together with color.
    ///
    /// For more information, see the "ncv" capability in terminfo(5).
    ///
    /// *C style function: [notcurses_supported_styles()][crate::notcurses_supported_styles].*
    pub fn supported_styles(&self) -> NcStyleMask {
        unsafe { crate::notcurses_supported_styles(self) as NcStyleMask }
    }

    /// Returns our current idea of the terminal dimensions in rows and cols.
    ///
    /// *C style function: [notcurses_supported_styles()][crate::notcurses_supported_styles].*
    pub fn term_dim_yx(&self) -> (NcDimension, NcDimension) {
        crate::notcurses_term_dim_yx(self)
    }

    /// Returns the topmost [NcPlane], of which there is always at least one.
    ///
    /// *C style function: [notcurses_top()][crate::notcurses_top].*
    pub fn top<'a>(&'a mut self) -> &'a mut NcPlane {
        unsafe { &mut *crate::notcurses_top(self) }
    }

    /// Returns a human-readable string describing the running Notcurses version.
    ///
    /// *C style function: [notcurses_version()][crate::notcurses_version].*
    pub fn version() -> String {
        rstring![crate::notcurses_version()].to_string()
    }

    /// Returns the running Notcurses version components
    /// (major, minor, patch, tweak).
    ///
    /// *C style function: [notcurses_version_components()][crate::notcurses_version_components].*
    pub fn version_components() -> (u32, u32, u32, u32) {
        let (mut major, mut minor, mut patch, mut tweak) = (0, 0, 0, 0);
        unsafe {
            crate::notcurses_version_components(&mut major, &mut minor, &mut patch, &mut tweak);
        }
        (major as u32, minor as u32, patch as u32, tweak as u32)
    }
}
