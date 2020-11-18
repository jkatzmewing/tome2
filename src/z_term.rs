use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut arg_bigtile: bool_;
    #[no_mangle]
    static mut use_bigtile: bool_;
    #[no_mangle]
    static mut angband_term: [*mut term; 8];
    #[no_mangle]
    fn cmovie_record_line(y: libc::c_int);
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn ralloc(len: huge_hack) -> vptr;
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type time_t = __time_t;
pub type vptr = *mut libc::c_void;
pub type cptr = *const libc::c_char;
pub type errr = libc::c_int;
pub type byte_hack = libc::c_uchar;
pub type bool_ = libc::c_char;
pub type huge_hack = libc::c_ulong;
pub type u16b = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct term_win {
    pub cu: bool_,
    pub cv: bool_,
    pub cx: byte_hack,
    pub cy: byte_hack,
    pub a: *mut *mut byte_hack,
    pub c: *mut *mut libc::c_char,
    pub va: *mut byte_hack,
    pub vc: *mut libc::c_char,
    pub ta: *mut *mut byte_hack,
    pub tc: *mut *mut libc::c_char,
    pub vta: *mut byte_hack,
    pub vtc: *mut libc::c_char,
    pub ea: *mut *mut byte_hack,
    pub ec: *mut *mut libc::c_char,
    pub vea: *mut byte_hack,
    pub vec: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct term {
    pub user: vptr,
    pub data: vptr,
    pub user_flag: bool_,
    pub data_flag: bool_,
    pub active_flag: bool_,
    pub mapped_flag: bool_,
    pub total_erase: bool_,
    pub fixed_shape: bool_,
    pub icky_corner: bool_,
    pub soft_cursor: bool_,
    pub always_pict: bool_,
    pub higher_pict: bool_,
    pub always_text: bool_,
    pub unused_flag: bool_,
    pub never_bored: bool_,
    pub never_frosh: bool_,
    pub attr_blank: byte_hack,
    pub char_blank: libc::c_char,
    pub key_queue: *mut libc::c_char,
    pub key_head: u16b,
    pub key_tail: u16b,
    pub key_xtra: u16b,
    pub key_size: u16b,
    pub wid: byte_hack,
    pub hgt: byte_hack,
    pub y1: byte_hack,
    pub y2: byte_hack,
    pub x1: *mut byte_hack,
    pub x2: *mut byte_hack,
    pub old: *mut term_win,
    pub scr: *mut term_win,
    pub tmp: *mut term_win,
    pub mem: *mut term_win,
    pub init_hook: Option<unsafe extern "C" fn(_: *mut term) -> ()>,
    pub nuke_hook: Option<unsafe extern "C" fn(_: *mut term) -> ()>,
    pub user_hook: Option<unsafe extern "C" fn(_: libc::c_int) -> errr>,
    pub xtra_hook: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int)
                              -> errr>,
    pub curs_hook: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int)
                              -> errr>,
    pub wipe_hook: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                               _: libc::c_int) -> errr>,
    pub text_hook: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                               _: libc::c_int, _: byte_hack,
                                               _: cptr) -> errr>,
    pub resize_hook: Option<unsafe extern "C" fn() -> ()>,
    pub pict_hook: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                               _: libc::c_int,
                                               _: *const byte_hack,
                                               _: *const libc::c_char,
                                               _: *const byte_hack,
                                               _: *const libc::c_char,
                                               _: *const byte_hack,
                                               _: *const libc::c_char)
                              -> errr>,
}
/* File: z-term.c */
/*
 * Copyright (c) 1997 Ben Harrison
 *
 * This software may be copied and distributed for educational, research,
 * and not for profit purposes provided that this copyright and statement
 * are included in all such copies.
 */
/* Purpose: a generic, efficient, terminal window package -BEN- */
/*
 * This file provides a generic, efficient, terminal window package,
 * which can be used not only on standard terminal environments such
 * as dumb terminals connected to a Unix box, but also in more modern
 * "graphic" environments, such as the Macintosh or Unix/X11.
 *
 * Each "window" works like a standard "dumb terminal", that is, it
 * can display a two dimensional array of grids containing colored
 * textual symbols, plus an optional cursor, and it can be used to
 * get keypress events from the user.
 *
 * In fact, this package can simply be used, if desired, to support
 * programs which will look the same on a dumb terminal as they do
 * on a graphic platform such as the Macintosh.
 *
 * This package was designed to help port the game "Angband" to a wide
 * variety of different platforms.  Angband, like many other games in
 * the "rogue-like" heirarchy, requires, at the minimum, the ability
 * to display "colored textual symbols" in a standard 80x24 "window",
 * such as that provided by most dumb terminals, and many old personal
 * computers, and to check for "keypresses" from the user.  The major
 * concerns were thus portability and efficiency, so Angband could be
 * easily ported to many different systems, with minimal effort, and
 * yet would run quickly on each of these systems, no matter what kind
 * of underlying hardware/software support was being used.
 *
 * It is important to understand the differences between the older
 * "dumb terminals" and the newer "graphic interface" machines, since
 * this package was designed to work with both types of systems.
 *
 * New machines:
 *   waiting for a keypress is complex
 *   checking for a keypress is often cheap
 *   changing "colors" may be expensive
 *   the "color" of a "blank" is rarely important
 *   moving the "cursor" is relatively cheap
 *   use a "software" cursor (only moves when requested)
 *   drawing characters normally will not erase old ones
 *   drawing a character on the cursor often erases it
 *   may have fast routines for "clear a region"
 *   the bottom right corner is usually not special
 *
 * Old machines:
 *   waiting for a keypress is simple
 *   checking for a keypress is often expensive
 *   changing "colors" is usually cheap
 *   the "color" of a "blank" may be important
 *   moving the "cursor" may be expensive
 *   use a "hardware" cursor (moves during screen updates)
 *   drawing new symbols automatically erases old ones
 *   characters may only be drawn at the cursor location
 *   drawing a character on the cursor will move the cursor
 *   may have fast routines for "clear entire window"
 *   may have fast routines for "clear to end of line"
 *   the bottom right corner is often dangerous
 *
 *
 * This package provides support for multiple windows, each of an
 * arbitrary size (up to 255x255), each with its own set of flags,
 * and its own hooks to handle several low-level procedures which
 * differ from platform to platform.  Then the main program simply
 * creates one or more "term" structures, setting the various flags
 * and hooks in a manner appropriate for the current platform, and
 * then it can use the various "term" structures without worrying
 * about the underlying platform.
 *
 *
 * This package allows each "grid" in each window to hold an attr/char
 * pair, with each ranging from 0 to 255, and makes very few assumptions
 * about the meaning of any attr/char values.  Normally, we assume that
 * "attr 0" is "black", with the semantics that "black" text should be
 * sent to "Term_wipe()" instead of "Term_text()", but this sematics is
 * modified if either the "always_pict" or the "always_text" flags are
 * set.  We assume that "char 0" is "dangerous", since placing such a
 * "char" in the middle of a string "terminates" the string, and usually
 * we prevent its use.
 *
 * Finally, we use a special attr/char pair, defaulting to "attr 0" and
 * "char 32", also known as "black space", when we "erase" or "clear"
 * any window, but this pair can be redefined to any pair, including
 * the standard "white space", or the bizarre "emptiness" ("attr 0"
 * and "char 0"), as long as various obscure restrictions are met.
 *
 *
 * This package provides several functions which allow a program to
 * interact with the "term" structures.  Most of the functions allow
 * the program to "request" certain changes to the current "term",
 * such as moving the cursor, drawing an attr/char pair, erasing a
 * region of grids, hiding the cursor, etc.  Then there is a special
 * function which causes all of the "pending" requests to be performed
 * in an efficient manner.  There is another set of functions which
 * allow the program to query the "requested state" of the current
 * "term", such as asking for the cursor location, or what attr/char
 * is at a given location, etc.  There is another set of functions
 * dealing with "keypress" events, which allows the program to ask if
 * the user has pressed any keys, or to forget any keys the user pressed.
 * There is a pair of functions to allow this package to memorize the
 * contents of the current "term", and to restore these contents at
 * a later time.  There is a special function which allows the program
 * to specify which "term" structure should be the "current" one.  At
 * the lowest level, there is a set of functions which allow a new
 * "term" to be initialized or destroyed, and which allow this package,
 * or a program, to access the special "hooks" defined for the current
 * "term", and a set of functions which those "hooks" can use to inform
 * this package of the results of certain occurances, for example, one
 * such function allows this package to learn about user keypresses,
 * detected by one of the special "hooks".
 *
 * We provide, among other things, the functions "Term_keypress()"
 * to "react" to keypress events, and "Term_redraw()" to redraw the
 * entire window, plus "Term_resize()" to note a new size.
 *
 *
 * Note that the current "term" contains two "window images".  One of
 * these images represents the "requested" contents of the "term", and
 * the other represents the "actual" contents of the "term", at the time
 * of the last performance of pending requests.  This package uses these
 * two images to determine the "minimal" amount of work needed to make
 * the "actual" contents of the "term" match the "requested" contents of
 * the "term".  This method is not perfect, but it often reduces the
 * amount of work needed to perform the pending requests, which thus
 * increases the speed of the program itself.  This package promises
 * that the requested changes will appear to occur either "all at once"
 * or in a "top to bottom" order.  In addition, a "cursor" is maintained,
 * and this cursor is updated along with the actual window contents.
 *
 * Currently, the "Term_fresh()" routine attempts to perform the "minimum"
 * number of physical updates, in terms of total "work" done by the hooks
 * Term_wipe(), Term_text(), and Term_pict(), making use of the fact that
 * adjacent characters of the same color can both be drawn together using
 * the "Term_text()" hook, and that "black" text can often be sent to the
 * "Term_wipe()" hook instead of the "Term_text()" hook, and if something
 * is already displayed in a window, then it is not necessary to display
 * it again.  Unfortunately, this may induce slightly non-optimal results
 * in some cases, in particular, those in which, say, a string of ten
 * characters needs to be written, but the fifth character has already
 * been displayed.  Currently, this will cause the "Term_text()" routine
 * to be called once for each half of the string, instead of once for the
 * whole string, which, on some machines, may be non-optimal behavior.
 *
 * The new formalism includes a "displayed" screen image (old) which
 * is actually seen by the user, a "requested" screen image (scr)
 * which is being prepared for display, a "memorized" screen image
 * (mem) which is used to save and restore screen images, and a
 * "temporary" screen image (tmp) which is currently unused.
 *
 *
 * Several "flags" are available in each "term" to allow the underlying
 * visual system (which initializes the "term" structure) to "optimize"
 * the performance of this package for the given system, or to request
 * certain behavior which is helpful/required for the given system.
 *
 * The "soft_cursor" flag indicates the use of a "soft" cursor, which
 * only moves when explicitly requested,and which is "erased" when
 * any characters are drawn on top of it.  This flag is used for all
 * "graphic" systems which handle the cursor by "drawing" it.
 *
 * The "icky_corner" flag indicates that the bottom right "corner"
 * of the windows are "icky", and "printing" anything there may
 * induce "messy" behavior, such as "scrolling".  This flag is used
 * for most old "dumb terminal" systems.
 *
 *
 * The "term" structure contains the following function "hooks":
 *
 *   Term->init_hook = Init the term
 *   Term->nuke_hook = Nuke the term
 *   Term->user_hook = Perform user actions
 *   Term->xtra_hook = Perform extra actions
 *   Term->curs_hook = Draw (or Move) the cursor
 *   Term->wipe_hook = Draw some blank spaces
 *   Term->text_hook = Draw some text in the window
 *   Term->pict_hook = Draw some attr/chars in the window
 *
 * The "Term->user_hook" hook provides a simple hook to an implementation
 * defined function, with application defined semantics.  It is available
 * to the program via the "Term_user()" function.
 *
 * The "Term->xtra_hook" hook provides a variety of different functions,
 * based on the first parameter (which should be taken from the various
 * TERM_XTRA_* defines) and the second parameter (which may make sense
 * only for some first parameters).  It is available to the program via
 * the "Term_xtra()" function, though some first parameters are only
 * "legal" when called from inside this package.
 *
 * The "Term->curs_hook" hook provides this package with a simple way
 * to "move" or "draw" the cursor to the grid "x,y", depending on the
 * setting of the "soft_cursor" flag.  Note that the cursor is never
 * redrawn if "nothing" has happened to the screen (even temporarily).
 * This hook is required.
 *
 * The "Term->wipe_hook" hook provides this package with a simple way
 * to "erase", starting at "x,y", the next "n" grids.  This hook assumes
 * that the input is valid.  This hook is required, unless the setting
 * of the "always_pict" or "always_text" flags makes it optional.
 *
 * The "Term->text_hook" hook provides this package with a simple way
 * to "draw", starting at "x,y", the "n" chars contained in "cp", using
 * the attr "a".  This hook assumes that the input is valid, and that
 * "n" is between 1 and 256 inclusive, but it should NOT assume that
 * the contents of "cp" are null-terminated.  This hook is required,
 * unless the setting of the "always_pict" flag makes it optional.
 *
 * The "Term->pict_hook" hook provides this package with a simple way
 * to "draw", starting at "x,y", the "n" attr/char pairs contained in
 * the arrays "ap" and "cp".  This hook assumes that the input is valid,
 * and that "n" is between 1 and 256 inclusive, but it should NOT assume
 * that the contents of "cp" are null-terminated.  This hook is optional,
 * unless the setting of the "always_pict" or "higher_pict" flags make
 * it required.  Note that recently, this hook was changed from taking
 * a byte "a" and a char "c" to taking a length "n", an array of bytes
 * "ap" and an array of chars "cp".  Old implementations of this hook
 * should now iterate over all "n" attr/char pairs.
 *
 *
 * The game "Angband" uses a set of files called "main-xxx.c", for
 * various "xxx" suffixes.  Most of these contain a function called
 * "init_xxx()", that will prepare the underlying visual system for
 * use with Angband, and then create one or more "term" structures,
 * using flags and hooks appropriate to the given platform, so that
 * the "main()" function can call one (or more) of the "init_xxx()"
 * functions, as appropriate, to prepare the required "term" structs
 * (one for each desired sub-window), and these "init_xxx()" functions
 * are called from a centralized "main()" function in "main.c".  Other
 * "main-xxx.c" systems contain their own "main()" function which, in
 * addition to doing everything needed to initialize the actual program,
 * also does everything that the normal "init_xxx()" functions would do.
 *
 * The game "Angband" defines, in addition to "attr 0", all of the
 * attr codes from 1 to 15, using definitions in "defines.h", and
 * thus the "main-xxx.c" files used by Angband must handle these
 * attr values correctly.  Also, they must handle all other attr
 * values, though they may do so in any way they wish, for example,
 * by always taking every attr code mod 16.  Many of the "main-xxx.c"
 * files use "white space" ("attr 1" / "char 32") to "erase" or "clear"
 * any window, for efficiency.
 *
 * The game "Angband" uses the "Term_user" hook to allow any of the
 * "main-xxx.c" files to interact with the user, by calling this hook
 * whenever the user presses the "!" key when the game is waiting for
 * a new command.  This could be used, for example, to provide "unix
 * shell commands" to the Unix versions of the game.
 *
 * See "main-xxx.c" for a simple skeleton file which can be used to
 * create a "visual system" for a new platform when porting Angband.
 */
/*
 * The current "term"
 */
#[no_mangle]
pub static mut Term: *mut term = 0 as *const term as *mut term;
/* File handler for saving movies */
#[no_mangle]
pub static mut movfile: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut do_movies: libc::c_int = 0 as libc::c_int;
/* Later set this as a global */
/* set to 1 if you want movies made */
#[no_mangle]
pub static mut lastc: time_t = 0;
#[no_mangle]
pub static mut last_paused: libc::c_int = 0 as libc::c_int;
/* Record cmovies with millisecond frame rate */
#[no_mangle]
pub static mut cmov_last_time_msec: libc::c_long = 0;
#[no_mangle]
pub static mut cmov_delta_time_msec: libc::c_long = 0;
/* ** Local routines ***/
/*
 * Nuke a term_win (see below)
 */
unsafe extern "C" fn term_win_nuke(mut s: *mut term_win, mut w: libc::c_int,
                                   mut h: libc::c_int) -> errr {
    /* Free the window access arrays */
    (*s).a =
        rnfree((*s).a as vptr,
               (h as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut byte_hack>()
                                                    as libc::c_ulong)) as
            *mut *mut byte_hack;
    (*s).c =
        rnfree((*s).c as vptr,
               (h as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut *mut libc::c_char;
    /* Free the window content arrays */
    (*s).va =
        rnfree((*s).va as vptr,
               ((h * w) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                    as libc::c_ulong)) as
            *mut byte_hack;
    (*s).vc =
        rnfree((*s).vc as vptr,
               ((h * w) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* Free the terrain access arrays */
    (*s).ta =
        rnfree((*s).ta as vptr,
               (h as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut byte_hack>()
                                                    as libc::c_ulong)) as
            *mut *mut byte_hack;
    (*s).tc =
        rnfree((*s).tc as vptr,
               (h as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut *mut libc::c_char;
    /* Free the terrain content arrays */
    (*s).vta =
        rnfree((*s).vta as vptr,
               ((h * w) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                    as libc::c_ulong)) as
            *mut byte_hack;
    (*s).vtc =
        rnfree((*s).vtc as vptr,
               ((h * w) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* Free the ego graphics access arrays */
    (*s).ea =
        rnfree((*s).ea as vptr,
               (h as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut byte_hack>()
                                                    as libc::c_ulong)) as
            *mut *mut byte_hack;
    (*s).ec =
        rnfree((*s).ec as vptr,
               (h as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut *mut libc::c_char;
    /* Free the ego graphics content arrays */
    (*s).vea =
        rnfree((*s).vea as vptr,
               ((h * w) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                    as libc::c_ulong)) as
            *mut byte_hack;
    (*s).vec =
        rnfree((*s).vec as vptr,
               ((h * w) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialize a "term_win" (using the given window size)
 */
unsafe extern "C" fn term_win_init(mut s: *mut term_win, mut w: libc::c_int,
                                   mut h: libc::c_int) -> errr {
    let mut y: libc::c_int = 0;
    /* Make the window access arrays */
    (*s).a =
        memset(ralloc((h as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut byte_hack>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (h as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut byte_hack>()
                                                    as libc::c_ulong)) as
            *mut *mut byte_hack;
    (*s).c =
        memset(ralloc((h as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (h as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut *mut libc::c_char;
    /* Make the window content arrays */
    (*s).va =
        memset(ralloc(((h * w) as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((h * w) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                    as libc::c_ulong)) as
            *mut byte_hack;
    (*s).vc =
        memset(ralloc(((h * w) as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((h * w) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* Make the terrain access arrays */
    (*s).ta =
        memset(ralloc((h as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut byte_hack>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (h as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut byte_hack>()
                                                    as libc::c_ulong)) as
            *mut *mut byte_hack;
    (*s).tc =
        memset(ralloc((h as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (h as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut *mut libc::c_char;
    /* Make the terrain content arrays */
    (*s).vta =
        memset(ralloc(((h * w) as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((h * w) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                    as libc::c_ulong)) as
            *mut byte_hack;
    (*s).vtc =
        memset(ralloc(((h * w) as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((h * w) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* Make the ego graphics access arrays */
    (*s).ea =
        memset(ralloc((h as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut byte_hack>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (h as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut byte_hack>()
                                                    as libc::c_ulong)) as
            *mut *mut byte_hack;
    (*s).ec =
        memset(ralloc((h as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (h as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut *mut libc::c_char;
    /* Make the ego graphics content arrays */
    (*s).vea =
        memset(ralloc(((h * w) as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((h * w) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                    as libc::c_ulong)) as
            *mut byte_hack;
    (*s).vec =
        memset(ralloc(((h * w) as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((h * w) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* Prepare the window access arrays */
    y = 0 as libc::c_int;
    while y < h {
        let ref mut fresh0 = *(*s).a.offset(y as isize);
        *fresh0 = (*s).va.offset((w * y) as isize);
        let ref mut fresh1 = *(*s).c.offset(y as isize);
        *fresh1 = (*s).vc.offset((w * y) as isize);
        let ref mut fresh2 = *(*s).ta.offset(y as isize);
        *fresh2 = (*s).vta.offset((w * y) as isize);
        let ref mut fresh3 = *(*s).tc.offset(y as isize);
        *fresh3 = (*s).vtc.offset((w * y) as isize);
        let ref mut fresh4 = *(*s).ea.offset(y as isize);
        *fresh4 = (*s).vea.offset((w * y) as isize);
        let ref mut fresh5 = *(*s).ec.offset(y as isize);
        *fresh5 = (*s).vec.offset((w * y) as isize);
        y += 1
    }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Copy a "term_win" from another
 */
unsafe extern "C" fn term_win_copy(mut s: *mut term_win, mut f: *mut term_win,
                                   mut w: libc::c_int, mut h: libc::c_int)
 -> errr {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    /* Copy contents */
    y = 0 as libc::c_int;
    while y < h {
        let mut f_aa: *mut byte_hack = *(*f).a.offset(y as isize);
        let mut f_cc: *mut libc::c_char = *(*f).c.offset(y as isize);
        let mut s_aa: *mut byte_hack = *(*s).a.offset(y as isize);
        let mut s_cc: *mut libc::c_char = *(*s).c.offset(y as isize);
        let mut f_taa: *mut byte_hack = *(*f).ta.offset(y as isize);
        let mut f_tcc: *mut libc::c_char = *(*f).tc.offset(y as isize);
        let mut s_taa: *mut byte_hack = *(*s).ta.offset(y as isize);
        let mut s_tcc: *mut libc::c_char = *(*s).tc.offset(y as isize);
        let mut f_eaa: *mut byte_hack = *(*f).ea.offset(y as isize);
        let mut f_ecc: *mut libc::c_char = *(*f).ec.offset(y as isize);
        let mut s_eaa: *mut byte_hack = *(*s).ea.offset(y as isize);
        let mut s_ecc: *mut libc::c_char = *(*s).ec.offset(y as isize);
        x = 0 as libc::c_int;
        while x < w {
            let fresh6 = f_aa;
            f_aa = f_aa.offset(1);
            let fresh7 = s_aa;
            s_aa = s_aa.offset(1);
            *fresh7 = *fresh6;
            let fresh8 = f_cc;
            f_cc = f_cc.offset(1);
            let fresh9 = s_cc;
            s_cc = s_cc.offset(1);
            *fresh9 = *fresh8;
            let fresh10 = f_taa;
            f_taa = f_taa.offset(1);
            let fresh11 = s_taa;
            s_taa = s_taa.offset(1);
            *fresh11 = *fresh10;
            let fresh12 = f_tcc;
            f_tcc = f_tcc.offset(1);
            let fresh13 = s_tcc;
            s_tcc = s_tcc.offset(1);
            *fresh13 = *fresh12;
            let fresh14 = f_eaa;
            f_eaa = f_eaa.offset(1);
            let fresh15 = s_eaa;
            s_eaa = s_eaa.offset(1);
            *fresh15 = *fresh14;
            let fresh16 = f_ecc;
            f_ecc = f_ecc.offset(1);
            let fresh17 = s_ecc;
            s_ecc = s_ecc.offset(1);
            *fresh17 = *fresh16;
            x += 1
        }
        y += 1
    }
    /* Copy cursor */
    (*s).cx = (*f).cx;
    (*s).cy = (*f).cy;
    (*s).cu = (*f).cu;
    (*s).cv = (*f).cv;
    /* Success */
    return 0 as libc::c_int;
}
/* ** External hooks ***/
/*
 * Execute the "Term->user_hook" hook, if available (see above).
 */
#[no_mangle]
pub unsafe extern "C" fn Term_user(mut n: libc::c_int) -> errr {
    /* Verify the hook */
    if (*Term).user_hook.is_none() { return -(1 as libc::c_int) }
    /* Call the hook */
    return Some((*Term).user_hook.expect("non-null function pointer")).expect("non-null function pointer")(n);
}
/*
 * Execute the "Term->xtra_hook" hook, if available (see above).
 * And *hacky* get a return code
 */
#[no_mangle]
pub static mut Term_xtra_long: libc::c_long = 0;
#[no_mangle]
pub static mut scansubdir_dir: [libc::c_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut scansubdir_max: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut scansubdir_result: [cptr; 255] =
    [0 as *const libc::c_char; 255];
#[no_mangle]
pub unsafe extern "C" fn Term_xtra(mut n: libc::c_int, mut v: libc::c_int)
 -> errr {
    /* Verify the hook */
    if (*Term).xtra_hook.is_none() { return -(1 as libc::c_int) }
    /* Call the hook */
    return Some((*Term).xtra_hook.expect("non-null function pointer")).expect("non-null function pointer")(n,
                                                                                                           v);
}
/* ** Fake hooks ***/
/*
 * Hack -- fake hook for "Term_curs()" (see above)
 */
unsafe extern "C" fn Term_curs_hack(mut x: libc::c_int, mut y: libc::c_int)
 -> errr {
    /* Compiler silliness */
    if x != 0 || y != 0 { return -(2 as libc::c_int) }
    /* Oops */
    return -(1 as libc::c_int);
}
/*
 * Hack -- fake hook for "Term_wipe()" (see above)
 */
unsafe extern "C" fn Term_wipe_hack(mut x: libc::c_int, mut y: libc::c_int,
                                    mut n: libc::c_int) -> errr {
    /* Compiler silliness */
    if x != 0 || y != 0 || n != 0 { return -(2 as libc::c_int) }
    /* Oops */
    return -(1 as libc::c_int);
}
/*
 * Hack -- fake hook for "Term_text()" (see above)
 */
unsafe extern "C" fn Term_text_hack(mut x: libc::c_int, mut y: libc::c_int,
                                    mut n: libc::c_int, mut a: byte_hack,
                                    mut cp: *const libc::c_char) -> errr {
    /* Compiler silliness */
    if x != 0 || y != 0 || n != 0 || a as libc::c_int != 0 || !cp.is_null() {
        return -(2 as libc::c_int)
    }
    /* Oops */
    return -(1 as libc::c_int);
}
/*
 * Hack -- fake hook for "Term_pict()" (see above)
 */
unsafe extern "C" fn Term_pict_hack(mut x: libc::c_int, mut y: libc::c_int,
                                    mut n: libc::c_int,
                                    mut ap: *const byte_hack,
                                    mut cp: *const libc::c_char,
                                    mut tap: *const byte_hack,
                                    mut tcp: *const libc::c_char,
                                    mut eap: *const byte_hack,
                                    mut ecp: *const libc::c_char) -> errr {
    /* Compiler silliness */
    if x != 0 || y != 0 || n != 0 || !ap.is_null() || !cp.is_null() ||
           !tap.is_null() || !tcp.is_null() || !eap.is_null() ||
           !ecp.is_null() {
        return -(2 as libc::c_int)
    }
    /* Oops */
    return -(1 as libc::c_int);
}
/* ** Efficient routines ***/
/*
 * Mentally draw an attr/char at a given location
 *
 * Assumes given location and values are valid.
 */
#[no_mangle]
pub unsafe extern "C" fn Term_queue_char(mut x: libc::c_int,
                                         mut y: libc::c_int, mut a: byte_hack,
                                         mut c: libc::c_char,
                                         mut ta: byte_hack,
                                         mut tc: libc::c_char,
                                         mut ea: byte_hack,
                                         mut ec: libc::c_char) {
    let mut scrn: *mut term_win = (*Term).scr;
    let mut scr_aa: *mut byte_hack =
        &mut *(*(*scrn).a.offset(y as isize)).offset(x as isize) as
            *mut byte_hack;
    let mut scr_cc: *mut libc::c_char =
        &mut *(*(*scrn).c.offset(y as isize)).offset(x as isize) as
            *mut libc::c_char;
    let mut scr_taa: *mut byte_hack =
        &mut *(*(*scrn).ta.offset(y as isize)).offset(x as isize) as
            *mut byte_hack;
    let mut scr_tcc: *mut libc::c_char =
        &mut *(*(*scrn).tc.offset(y as isize)).offset(x as isize) as
            *mut libc::c_char;
    let mut scr_eaa: *mut byte_hack =
        &mut *(*(*scrn).ea.offset(y as isize)).offset(x as isize) as
            *mut byte_hack;
    let mut scr_ecc: *mut libc::c_char =
        &mut *(*(*scrn).ec.offset(y as isize)).offset(x as isize) as
            *mut libc::c_char;
    /* Hack -- Ignore non-changes */
    if *scr_aa as libc::c_int == a as libc::c_int &&
           *scr_cc as libc::c_int == c as libc::c_int &&
           *scr_taa as libc::c_int == ta as libc::c_int &&
           *scr_tcc as libc::c_int == tc as libc::c_int &&
           *scr_eaa as libc::c_int == ea as libc::c_int &&
           *scr_ecc as libc::c_int == ec as libc::c_int {
        return
    }
    /* Save the "literal" information */
    *scr_aa = a;
    *scr_cc = c;
    *scr_taa = ta;
    *scr_tcc = tc;
    *scr_eaa = ea;
    *scr_ecc = ec;
    /* Check for new min/max row info */
    if y < (*Term).y1 as libc::c_int { (*Term).y1 = y as byte_hack }
    if y > (*Term).y2 as libc::c_int { (*Term).y2 = y as byte_hack }
    /* Check for new min/max col info for this row */
    if x < *(*Term).x1.offset(y as isize) as libc::c_int {
        *(*Term).x1.offset(y as isize) = x as byte_hack
    }
    if x > *(*Term).x2.offset(y as isize) as libc::c_int {
        *(*Term).x2.offset(y as isize) = x as byte_hack
    };
}
/*
 * Mentally draw a string of attr/chars at a given location
 *
 * Assumes given location and values are valid.
 *
 * This function is designed to be fast, with no consistancy checking.
 * It is used to update the map in the game.
 */
#[no_mangle]
pub unsafe extern "C" fn Term_queue_line(mut x: libc::c_int,
                                         mut y: libc::c_int,
                                         mut n: libc::c_int,
                                         mut a: *mut byte_hack,
                                         mut c: *mut libc::c_char,
                                         mut ta: *mut byte_hack,
                                         mut tc: *mut libc::c_char,
                                         mut ea: *mut byte_hack,
                                         mut ec: *mut libc::c_char) {
    let mut scrn: *mut term_win = (*Term).scr;
    let mut x1: libc::c_int = -(1 as libc::c_int);
    let mut x2: libc::c_int = -(1 as libc::c_int);
    let mut scr_aa: *mut byte_hack =
        &mut *(*(*scrn).a.offset(y as isize)).offset(x as isize) as
            *mut byte_hack;
    let mut scr_cc: *mut libc::c_char =
        &mut *(*(*scrn).c.offset(y as isize)).offset(x as isize) as
            *mut libc::c_char;
    let mut scr_taa: *mut byte_hack =
        &mut *(*(*scrn).ta.offset(y as isize)).offset(x as isize) as
            *mut byte_hack;
    let mut scr_tcc: *mut libc::c_char =
        &mut *(*(*scrn).tc.offset(y as isize)).offset(x as isize) as
            *mut libc::c_char;
    let mut scr_eaa: *mut byte_hack =
        &mut *(*(*scrn).ea.offset(y as isize)).offset(x as isize) as
            *mut byte_hack;
    let mut scr_ecc: *mut libc::c_char =
        &mut *(*(*scrn).ec.offset(y as isize)).offset(x as isize) as
            *mut libc::c_char;
    loop  {
        let fresh18 = n;
        n = n - 1;
        if !(fresh18 != 0) { break ; }
        /* Hack -- Ignore non-changes */
        if *scr_aa as libc::c_int == *a as libc::c_int &&
               *scr_cc as libc::c_int == *c as libc::c_int &&
               *scr_taa as libc::c_int == *ta as libc::c_int &&
               *scr_tcc as libc::c_int == *tc as libc::c_int &&
               *scr_eaa as libc::c_int == *ea as libc::c_int &&
               *scr_ecc as libc::c_int == *ec as libc::c_int {
            x += 1;
            a = a.offset(1);
            c = c.offset(1);
            ta = ta.offset(1);
            tc = tc.offset(1);
            ea = ea.offset(1);
            ec = ec.offset(1);
            scr_aa = scr_aa.offset(1);
            scr_cc = scr_cc.offset(1);
            scr_taa = scr_taa.offset(1);
            scr_tcc = scr_tcc.offset(1);
            scr_eaa = scr_eaa.offset(1);
            scr_ecc = scr_ecc.offset(1)
        } else {
            /* Save the "literal" information */
            let fresh19 = ta;
            ta = ta.offset(1);
            let fresh20 = scr_taa;
            scr_taa = scr_taa.offset(1);
            *fresh20 = *fresh19;
            let fresh21 = tc;
            tc = tc.offset(1);
            let fresh22 = scr_tcc;
            scr_tcc = scr_tcc.offset(1);
            *fresh22 = *fresh21;
            /* Save the "literal" information */
            let fresh23 = ea;
            ea = ea.offset(1);
            let fresh24 = scr_eaa;
            scr_eaa = scr_eaa.offset(1);
            *fresh24 = *fresh23;
            let fresh25 = ec;
            ec = ec.offset(1);
            let fresh26 = scr_ecc;
            scr_ecc = scr_ecc.offset(1);
            *fresh26 = *fresh25;
            /* Save the "literal" information */
            let fresh27 = a;
            a = a.offset(1);
            let fresh28 = scr_aa;
            scr_aa = scr_aa.offset(1);
            *fresh28 = *fresh27;
            let fresh29 = c;
            c = c.offset(1);
            let fresh30 = scr_cc;
            scr_cc = scr_cc.offset(1);
            *fresh30 = *fresh29;
            /* Track minimum changed column */
            if x1 < 0 as libc::c_int { x1 = x }
            /* Track maximum changed column */
            x2 = x;
            x += 1
        }
    }
    /* Expand the "change area" as needed */
    if x1 >= 0 as libc::c_int {
        /* Check for new min/max row info */
        if y < (*Term).y1 as libc::c_int { (*Term).y1 = y as byte_hack }
        if y > (*Term).y2 as libc::c_int { (*Term).y2 = y as byte_hack }
        /* Check for new min/max col info in this row */
        if x1 < *(*Term).x1.offset(y as isize) as libc::c_int {
            *(*Term).x1.offset(y as isize) = x1 as byte_hack
        }
        if x2 > *(*Term).x2.offset(y as isize) as libc::c_int {
            *(*Term).x2.offset(y as isize) = x2 as byte_hack
        }
    };
}
/*
 * Mentally draw some attr/chars at a given location
 *
 * Assumes that (x,y) is a valid location, that the first "n" characters
 * of the string "s" are all valid (non-zero), and that (x+n-1,y) is also
 * a valid location, so the first "n" characters of "s" can all be added
 * starting at (x,y) without causing any illegal operations.
 */
#[no_mangle]
pub unsafe extern "C" fn Term_queue_chars(mut x: libc::c_int,
                                          mut y: libc::c_int,
                                          mut n: libc::c_int,
                                          mut a: byte_hack, mut s: cptr) {
    let mut x1: libc::c_int = -(1 as libc::c_int);
    let mut x2: libc::c_int = -(1 as libc::c_int);
    let mut scr_aa: *mut byte_hack = *(*(*Term).scr).a.offset(y as isize);
    let mut scr_cc: *mut libc::c_char = *(*(*Term).scr).c.offset(y as isize);
    let mut scr_taa: *mut byte_hack = *(*(*Term).scr).ta.offset(y as isize);
    let mut scr_tcc: *mut libc::c_char =
        *(*(*Term).scr).tc.offset(y as isize);
    let mut scr_eaa: *mut byte_hack = *(*(*Term).scr).ea.offset(y as isize);
    let mut scr_ecc: *mut libc::c_char =
        *(*(*Term).scr).ec.offset(y as isize);
    /* Queue the attr/chars */
    while n != 0 {
        let mut oa: libc::c_int = *scr_aa.offset(x as isize) as libc::c_int;
        let mut oc: libc::c_int = *scr_cc.offset(x as isize) as libc::c_int;
        let mut ota: libc::c_int = *scr_taa.offset(x as isize) as libc::c_int;
        let mut otc: libc::c_int = *scr_tcc.offset(x as isize) as libc::c_int;
        let mut oea: libc::c_int = *scr_eaa.offset(x as isize) as libc::c_int;
        let mut oec: libc::c_int = *scr_ecc.offset(x as isize) as libc::c_int;
        /* Hack -- Ignore non-changes */
        if !(oa == a as libc::c_int && oc == *s as libc::c_int &&
                 ota == 0 as libc::c_int && otc == 0 as libc::c_int &&
                 oea == 0 as libc::c_int && oec == 0 as libc::c_int) {
            /* Save the "literal" information */
            *scr_aa.offset(x as isize) = a;
            *scr_cc.offset(x as isize) = *s;
            *scr_taa.offset(x as isize) = 0 as libc::c_int as byte_hack;
            *scr_tcc.offset(x as isize) = 0 as libc::c_int as libc::c_char;
            *scr_taa.offset(x as isize) = 0 as libc::c_int as byte_hack;
            *scr_tcc.offset(x as isize) = 0 as libc::c_int as libc::c_char;
            /* Note the "range" of window updates */
            if x1 < 0 as libc::c_int { x1 = x }
            x2 = x
        }
        x += 1;
        s = s.offset(1);
        n -= 1
    }
    /* Expand the "change area" as needed */
    if x1 >= 0 as libc::c_int {
        /* Check for new min/max row info */
        if y < (*Term).y1 as libc::c_int { (*Term).y1 = y as byte_hack }
        if y > (*Term).y2 as libc::c_int { (*Term).y2 = y as byte_hack }
        /* Check for new min/max col info in this row */
        if x1 < *(*Term).x1.offset(y as isize) as libc::c_int {
            *(*Term).x1.offset(y as isize) = x1 as byte_hack
        }
        if x2 > *(*Term).x2.offset(y as isize) as libc::c_int {
            *(*Term).x2.offset(y as isize) = x2 as byte_hack
        }
    };
}
/* ** Refresh routines ***/
/*
 * Flush a row of the current window (see "Term_fresh")
 *
 * Display text using "Term_pict()"
 */
unsafe extern "C" fn Term_fresh_row_pict(mut y: libc::c_int,
                                         mut x1: libc::c_int,
                                         mut x2: libc::c_int) {
    let mut x: libc::c_int = 0;
    let mut old_aa: *mut byte_hack = *(*(*Term).old).a.offset(y as isize);
    let mut old_cc: *mut libc::c_char = *(*(*Term).old).c.offset(y as isize);
    let mut scr_aa: *mut byte_hack = *(*(*Term).scr).a.offset(y as isize);
    let mut scr_cc: *mut libc::c_char = *(*(*Term).scr).c.offset(y as isize);
    let mut old_taa: *mut byte_hack = *(*(*Term).old).ta.offset(y as isize);
    let mut old_tcc: *mut libc::c_char =
        *(*(*Term).old).tc.offset(y as isize);
    let mut scr_taa: *mut byte_hack = *(*(*Term).scr).ta.offset(y as isize);
    let mut scr_tcc: *mut libc::c_char =
        *(*(*Term).scr).tc.offset(y as isize);
    let mut ota: byte_hack = 0;
    let mut otc: libc::c_char = 0;
    let mut nta: byte_hack = 0;
    let mut ntc: libc::c_char = 0;
    let mut old_eaa: *mut byte_hack = *(*(*Term).old).ea.offset(y as isize);
    let mut old_ecc: *mut libc::c_char =
        *(*(*Term).old).ec.offset(y as isize);
    let mut scr_eaa: *mut byte_hack = *(*(*Term).scr).ea.offset(y as isize);
    let mut scr_ecc: *mut libc::c_char =
        *(*(*Term).scr).ec.offset(y as isize);
    let mut oea: byte_hack = 0;
    let mut oec: libc::c_char = 0;
    let mut nea: byte_hack = 0;
    let mut nec: libc::c_char = 0;
    /* Pending length */
    let mut fn_0: libc::c_int = 0 as libc::c_int;
    /* Pending start */
    let mut fx: libc::c_int = 0 as libc::c_int;
    let mut oa: byte_hack = 0;
    let mut oc: libc::c_char = 0;
    let mut na: byte_hack = 0;
    let mut nc: libc::c_char = 0;
    /* Scan "modified" columns */
    x = x1;
    while x <= x2 {
        /* See what is currently here */
        oa = *old_aa.offset(x as isize);
        oc = *old_cc.offset(x as isize);
        /* See what is desired there */
        na = *scr_aa.offset(x as isize);
        nc = *scr_cc.offset(x as isize);
        ota = *old_taa.offset(x as isize);
        otc = *old_tcc.offset(x as isize);
        nta = *scr_taa.offset(x as isize);
        ntc = *scr_tcc.offset(x as isize);
        oea = *old_eaa.offset(x as isize);
        oec = *old_ecc.offset(x as isize);
        nea = *scr_eaa.offset(x as isize);
        nec = *scr_ecc.offset(x as isize);
        /* Handle unchanged grids */
        if na as libc::c_int == oa as libc::c_int &&
               nc as libc::c_int == oc as libc::c_int &&
               nta as libc::c_int == ota as libc::c_int &&
               ntc as libc::c_int == otc as libc::c_int &&
               nea as libc::c_int == oea as libc::c_int &&
               nec as libc::c_int == oec as libc::c_int {
            /* Flush */
            if fn_0 != 0 {
                /* Draw pending attr/char pairs */
                Some((*Term).pict_hook.expect("non-null function pointer")).expect("non-null function pointer")(fx,
                                                                                                                y,
                                                                                                                fn_0,
                                                                                                                &mut *scr_aa.offset(fx
                                                                                                                                        as
                                                                                                                                        isize),
                                                                                                                &mut *scr_cc.offset(fx
                                                                                                                                        as
                                                                                                                                        isize),
                                                                                                                &mut *scr_taa.offset(fx
                                                                                                                                         as
                                                                                                                                         isize),
                                                                                                                &mut *scr_tcc.offset(fx
                                                                                                                                         as
                                                                                                                                         isize),
                                                                                                                &mut *scr_eaa.offset(fx
                                                                                                                                         as
                                                                                                                                         isize),
                                                                                                                &mut *scr_ecc.offset(fx
                                                                                                                                         as
                                                                                                                                         isize));
                /* Forget */
                fn_0 = 0 as libc::c_int
            }
        } else {
            /* Save new contents */
            *old_aa.offset(x as isize) = na;
            *old_cc.offset(x as isize) = nc;
            *old_taa.offset(x as isize) = nta;
            *old_tcc.offset(x as isize) = ntc;
            *old_eaa.offset(x as isize) = nea;
            *old_ecc.offset(x as isize) = nec;
            /* Restart and Advance */
            let fresh31 = fn_0;
            fn_0 = fn_0 + 1;
            if fresh31 == 0 as libc::c_int { fx = x }
        }
        /* Skip */
        x += 1
    }
    /* Flush */
    if fn_0 != 0 {
        /* Draw pending attr/char pairs */
        Some((*Term).pict_hook.expect("non-null function pointer")).expect("non-null function pointer")(fx,
                                                                                                        y,
                                                                                                        fn_0,
                                                                                                        &mut *scr_aa.offset(fx
                                                                                                                                as
                                                                                                                                isize),
                                                                                                        &mut *scr_cc.offset(fx
                                                                                                                                as
                                                                                                                                isize),
                                                                                                        &mut *scr_taa.offset(fx
                                                                                                                                 as
                                                                                                                                 isize),
                                                                                                        &mut *scr_tcc.offset(fx
                                                                                                                                 as
                                                                                                                                 isize),
                                                                                                        &mut *scr_eaa.offset(fx
                                                                                                                                 as
                                                                                                                                 isize),
                                                                                                        &mut *scr_ecc.offset(fx
                                                                                                                                 as
                                                                                                                                 isize));
    };
}
/*
 * Flush a row of the current window (see "Term_fresh")
 *
 * Display text using "Term_text()" and "Term_wipe()",
 * but use "Term_pict()" for high-bit attr/char pairs
 */
unsafe extern "C" fn Term_fresh_row_both(mut y: libc::c_int,
                                         mut x1: libc::c_int,
                                         mut x2: libc::c_int) {
    let mut x: libc::c_int = 0;
    let mut old_aa: *mut byte_hack = *(*(*Term).old).a.offset(y as isize);
    let mut old_cc: *mut libc::c_char = *(*(*Term).old).c.offset(y as isize);
    let mut scr_aa: *mut byte_hack = *(*(*Term).scr).a.offset(y as isize);
    let mut scr_cc: *mut libc::c_char = *(*(*Term).scr).c.offset(y as isize);
    let mut old_taa: *mut byte_hack = *(*(*Term).old).ta.offset(y as isize);
    let mut old_tcc: *mut libc::c_char =
        *(*(*Term).old).tc.offset(y as isize);
    let mut scr_taa: *mut byte_hack = *(*(*Term).scr).ta.offset(y as isize);
    let mut scr_tcc: *mut libc::c_char =
        *(*(*Term).scr).tc.offset(y as isize);
    let mut ota: byte_hack = 0;
    let mut otc: libc::c_char = 0;
    let mut nta: byte_hack = 0;
    let mut ntc: libc::c_char = 0;
    let mut old_eaa: *mut byte_hack = *(*(*Term).old).ea.offset(y as isize);
    let mut old_ecc: *mut libc::c_char =
        *(*(*Term).old).ec.offset(y as isize);
    let mut scr_eaa: *mut byte_hack = *(*(*Term).scr).ea.offset(y as isize);
    let mut scr_ecc: *mut libc::c_char =
        *(*(*Term).scr).ec.offset(y as isize);
    let mut oea: byte_hack = 0;
    let mut oec: libc::c_char = 0;
    let mut nea: byte_hack = 0;
    let mut nec: libc::c_char = 0;
    /* The "always_text" flag */
    let mut always_text: libc::c_int = (*Term).always_text as libc::c_int;
    /* Pending length */
    let mut fn_0: libc::c_int = 0 as libc::c_int;
    /* Pending start */
    let mut fx: libc::c_int = 0 as libc::c_int;
    /* Pending attr */
    let mut fa: byte_hack = (*Term).attr_blank;
    let mut oa: byte_hack = 0;
    let mut oc: libc::c_char = 0;
    let mut na: byte_hack = 0;
    let mut nc: libc::c_char = 0;
    /* Scan "modified" columns */
    x = x1;
    while x <= x2 {
        /* See what is currently here */
        oa = *old_aa.offset(x as isize);
        oc = *old_cc.offset(x as isize);
        /* See what is desired there */
        na = *scr_aa.offset(x as isize);
        nc = *scr_cc.offset(x as isize);
        ota = *old_taa.offset(x as isize);
        otc = *old_tcc.offset(x as isize);
        nta = *scr_taa.offset(x as isize);
        ntc = *scr_tcc.offset(x as isize);
        oea = *old_eaa.offset(x as isize);
        oec = *old_ecc.offset(x as isize);
        nea = *scr_eaa.offset(x as isize);
        nec = *scr_ecc.offset(x as isize);
        /* Handle unchanged grids */
        if na as libc::c_int == oa as libc::c_int &&
               nc as libc::c_int == oc as libc::c_int &&
               nta as libc::c_int == ota as libc::c_int &&
               ntc as libc::c_int == otc as libc::c_int &&
               nea as libc::c_int == oea as libc::c_int &&
               nec as libc::c_int == oec as libc::c_int {
            /* Flush */
            if fn_0 != 0 {
                /* Draw pending chars (normal) */
                if fa as libc::c_int != 0 || always_text != 0 {
                    Some((*Term).text_hook.expect("non-null function pointer")).expect("non-null function pointer")(fx,
                                                                                                                    y,
                                                                                                                    fn_0,
                                                                                                                    fa,
                                                                                                                    &mut *scr_cc.offset(fx
                                                                                                                                            as
                                                                                                                                            isize)
                                                                                                                        as
                                                                                                                        *mut libc::c_char
                                                                                                                        as
                                                                                                                        cptr);
                } else {
                    /* Draw pending chars (black) */
                    Some((*Term).wipe_hook.expect("non-null function pointer")).expect("non-null function pointer")(fx,
                                                                                                                    y,
                                                                                                                    fn_0);
                }
                /* Forget */
                fn_0 = 0 as libc::c_int
            }
        } else {
            /* Save new contents */
            *old_aa.offset(x as isize) = na;
            *old_cc.offset(x as isize) = nc;
            *old_taa.offset(x as isize) = nta;
            *old_tcc.offset(x as isize) = ntc;
            *old_eaa.offset(x as isize) = nea;
            *old_ecc.offset(x as isize) = nec;
            /* 2nd byte of bigtile */
            if !(na as libc::c_int == 255 as libc::c_int) {
                /* Handle high-bit attr/chars */
                if na as libc::c_int & 0x80 as libc::c_int != 0 {
                    /* Flush */
                    if fn_0 != 0 {
                        /* Draw pending chars (normal) */
                        if fa as libc::c_int != 0 || always_text != 0 {
                            Some((*Term).text_hook.expect("non-null function pointer")).expect("non-null function pointer")(fx,
                                                                                                                            y,
                                                                                                                            fn_0,
                                                                                                                            fa,
                                                                                                                            &mut *scr_cc.offset(fx
                                                                                                                                                    as
                                                                                                                                                    isize)
                                                                                                                                as
                                                                                                                                *mut libc::c_char
                                                                                                                                as
                                                                                                                                cptr);
                        } else {
                            /* Draw pending chars (black) */
                            Some((*Term).wipe_hook.expect("non-null function pointer")).expect("non-null function pointer")(fx,
                                                                                                                            y,
                                                                                                                            fn_0);
                        }
                        /* Forget */
                        fn_0 = 0 as libc::c_int
                    }
                    /* Hack -- Draw the special attr/char pair */
                    Some((*Term).pict_hook.expect("non-null function pointer")).expect("non-null function pointer")(x,
                                                                                                                    y,
                                                                                                                    1
                                                                                                                        as
                                                                                                                        libc::c_int,
                                                                                                                    &mut na,
                                                                                                                    &mut nc,
                                                                                                                    &mut nta,
                                                                                                                    &mut ntc,
                                                                                                                    &mut nea,
                                                                                                                    &mut nec);
                } else {
                    /* Notice new color */
                    if fa as libc::c_int != na as libc::c_int {
                        /* Flush */
                        if fn_0 != 0 {
                            /* Draw the pending chars */
                            if fa as libc::c_int != 0 || always_text != 0 {
                                Some((*Term).text_hook.expect("non-null function pointer")).expect("non-null function pointer")(fx,
                                                                                                                                y,
                                                                                                                                fn_0,
                                                                                                                                fa,
                                                                                                                                &mut *scr_cc.offset(fx
                                                                                                                                                        as
                                                                                                                                                        isize)
                                                                                                                                    as
                                                                                                                                    *mut libc::c_char
                                                                                                                                    as
                                                                                                                                    cptr);
                            } else {
                                /* Hack -- Erase "leading" spaces */
                                Some((*Term).wipe_hook.expect("non-null function pointer")).expect("non-null function pointer")(fx,
                                                                                                                                y,
                                                                                                                                fn_0);
                            }
                            /* Forget */
                            fn_0 = 0 as libc::c_int
                        }
                        /* Save the new color */
                        fa = na
                    }
                    /* Restart and Advance */
                    let fresh32 = fn_0;
                    fn_0 = fn_0 + 1;
                    if fresh32 == 0 as libc::c_int { fx = x }
                }
            }
        }
        /* Skip */
        x += 1
    }
    /* Flush */
    if fn_0 != 0 {
        /* Draw pending chars (normal) */
        if fa as libc::c_int != 0 || always_text != 0 {
            Some((*Term).text_hook.expect("non-null function pointer")).expect("non-null function pointer")(fx,
                                                                                                            y,
                                                                                                            fn_0,
                                                                                                            fa,
                                                                                                            &mut *scr_cc.offset(fx
                                                                                                                                    as
                                                                                                                                    isize)
                                                                                                                as
                                                                                                                *mut libc::c_char
                                                                                                                as
                                                                                                                cptr);
        } else {
            /* Draw pending chars (black) */
            Some((*Term).wipe_hook.expect("non-null function pointer")).expect("non-null function pointer")(fx,
                                                                                                            y,
                                                                                                            fn_0);
        }
    };
}
/*
 * Flush a row of the current window (see "Term_fresh")
 *
 * Display text using "Term_text()" and "Term_wipe()"
 */
unsafe extern "C" fn Term_fresh_row_text(mut y: libc::c_int,
                                         mut x1: libc::c_int,
                                         mut x2: libc::c_int) {
    let mut x: libc::c_int = 0;
    let mut old_aa: *mut byte_hack = *(*(*Term).old).a.offset(y as isize);
    let mut old_cc: *mut libc::c_char = *(*(*Term).old).c.offset(y as isize);
    let mut scr_aa: *mut byte_hack = *(*(*Term).scr).a.offset(y as isize);
    let mut scr_cc: *mut libc::c_char = *(*(*Term).scr).c.offset(y as isize);
    /* The "always_text" flag */
    let mut always_text: libc::c_int = (*Term).always_text as libc::c_int;
    /* Pending length */
    let mut fn_0: libc::c_int = 0 as libc::c_int;
    /* Pending start */
    let mut fx: libc::c_int = 0 as libc::c_int;
    /* Pending attr */
    let mut fa: byte_hack = (*Term).attr_blank;
    let mut oa: byte_hack = 0;
    let mut oc: libc::c_char = 0;
    let mut na: byte_hack = 0;
    let mut nc: libc::c_char = 0;
    /* Scan "modified" columns */
    x = x1;
    while x <= x2 {
        /* See what is currently here */
        oa = *old_aa.offset(x as isize);
        oc = *old_cc.offset(x as isize);
        /* See what is desired there */
        na = *scr_aa.offset(x as isize);
        nc = *scr_cc.offset(x as isize);
        /* Handle unchanged grids */
        if na as libc::c_int == oa as libc::c_int &&
               nc as libc::c_int == oc as libc::c_int {
            /* Flush */
            if fn_0 != 0 {
                /* Draw pending chars (normal) */
                if fa as libc::c_int != 0 || always_text != 0 {
                    Some((*Term).text_hook.expect("non-null function pointer")).expect("non-null function pointer")(fx,
                                                                                                                    y,
                                                                                                                    fn_0,
                                                                                                                    fa,
                                                                                                                    &mut *scr_cc.offset(fx
                                                                                                                                            as
                                                                                                                                            isize)
                                                                                                                        as
                                                                                                                        *mut libc::c_char
                                                                                                                        as
                                                                                                                        cptr);
                } else {
                    /* Draw pending chars (black) */
                    Some((*Term).wipe_hook.expect("non-null function pointer")).expect("non-null function pointer")(fx,
                                                                                                                    y,
                                                                                                                    fn_0);
                }
                /* Forget */
                fn_0 = 0 as libc::c_int
            }
        } else {
            /* Save new contents */
            *old_aa.offset(x as isize) = na;
            *old_cc.offset(x as isize) = nc;
            /* Notice new color */
            if fa as libc::c_int != na as libc::c_int {
                /* Flush */
                if fn_0 != 0 {
                    /* Draw the pending chars */
                    if fa as libc::c_int != 0 || always_text != 0 {
                        Some((*Term).text_hook.expect("non-null function pointer")).expect("non-null function pointer")(fx,
                                                                                                                        y,
                                                                                                                        fn_0,
                                                                                                                        fa,
                                                                                                                        &mut *scr_cc.offset(fx
                                                                                                                                                as
                                                                                                                                                isize)
                                                                                                                            as
                                                                                                                            *mut libc::c_char
                                                                                                                            as
                                                                                                                            cptr);
                    } else {
                        /* Hack -- Erase "leading" spaces */
                        Some((*Term).wipe_hook.expect("non-null function pointer")).expect("non-null function pointer")(fx,
                                                                                                                        y,
                                                                                                                        fn_0);
                    }
                    /* Forget */
                    fn_0 = 0 as libc::c_int
                }
                /* Save the new color */
                fa = na
            }
            /* Restart and Advance */
            let fresh33 = fn_0;
            fn_0 = fn_0 + 1;
            if fresh33 == 0 as libc::c_int { fx = x }
        }
        /* Skip */
        x += 1
    }
    /* Flush */
    if fn_0 != 0 {
        /* Draw pending chars (normal) */
        if fa as libc::c_int != 0 || always_text != 0 {
            Some((*Term).text_hook.expect("non-null function pointer")).expect("non-null function pointer")(fx,
                                                                                                            y,
                                                                                                            fn_0,
                                                                                                            fa,
                                                                                                            &mut *scr_cc.offset(fx
                                                                                                                                    as
                                                                                                                                    isize)
                                                                                                                as
                                                                                                                *mut libc::c_char
                                                                                                                as
                                                                                                                cptr);
        } else {
            /* Draw pending chars (black) */
            Some((*Term).wipe_hook.expect("non-null function pointer")).expect("non-null function pointer")(fx,
                                                                                                            y,
                                                                                                            fn_0);
        }
    };
}
/*
 * Actually perform all requested changes to the window
 *
 * If absolutely nothing has changed, not even temporarily, or if the
 * current "Term" is not mapped, then this function will return 1 and
 * do absolutely nothing.
 *
 * Note that when "soft_cursor" is true, we erase the cursor (if needed)
 * whenever anything has changed, and redraw it (if needed) after all of
 * the screen updates are complete.  This will induce a small amount of
 * "cursor flicker" but only when the screen has been updated.  If the
 * screen is updated and then restored, you may still get this flicker.
 *
 * When "soft_cursor" is not true, we make the cursor invisible before
 * doing anything else if it is supposed to be invisible by the time we
 * are done, and we make it visible after moving it to its final location
 * after all of the screen updates are complete.
 *
 * Note that "Term_xtra(TERM_XTRA_CLEAR,0)" must erase the entire screen,
 * including the cursor, if needed, and may place the cursor anywhere.
 *
 * Note that "Term_xtra(TERM_XTRA_FROSH,y)" will be always be called
 * after any row "y" has been "flushed", unless the "Term->never_frosh"
 * flag is set, and "Term_xtra(TERM_XTRA_FRESH,0)" will be called after
 * all of the rows have been "flushed".
 *
 * Note the use of three different functions to handle the actual flush,
 * based on the settings of the "Term->always_pict" and "Term->higher_pict"
 * flags (see below).
 *
 * The three helper functions (above) work by collecting similar adjacent
 * grids into stripes, and then sending each stripe to "Term->pict_hook",
 * "Term->text_hook", or "Term->wipe_hook", based on the settings of the
 * "Term->always_pict" and "Term->higher_pict" flags, which select which
 * of the helper functions to call to flush each row.
 *
 * The helper functions currently "skip" any grids which already contain
 * the desired contents.  This may or may not be the best method, especially
 * when the desired content fits nicely into the current stripe.  For example,
 * it might be better to go ahead and queue them while allowed, but keep a
 * count of the "trailing skipables", then, when time to flush, or when a
 * "non skippable" is found, force a flush if there are too many skippables.
 *
 * Perhaps an "initialization" stage, where the "text" (and "attr")
 * buffers are "filled" with information, converting "blanks" into
 * a convenient representation, and marking "skips" with "zero chars",
 * and then some "processing" is done to determine which chars to skip.
 *
 * Currently, the helper functions are optimal for systems which prefer
 * to "print a char + move a char + print a char" to "print three chars",
 * and for applications that do a lot of "detailed" color printing.
 *
 * In the two "queue" functions, total "non-changes" are "pre-skipped".
 * The helper functions must also handle situations in which the contents
 * of a grid are changed, but then changed back to the original value,
 * and situations in which two grids in the same row are changed, but
 * the grids between them are unchanged.
 *
 * If the "Term->always_pict" flag is set, then "Term_fresh_row_pict()"
 * will be used instead of "Term_fresh_row_text()".  This allows all the
 * modified grids to be collected into stripes of attr/char pairs, which
 * are then sent to the "Term->pict_hook" hook, which can draw these pairs
 * in whatever way it would like.
 *
 * If the "Term->higher_pict" flag is set, then "Term_fresh_row_both()"
 * will be used instead of "Term_fresh_row_text()".  This allows all the
 * "special" attr/char pairs (in which both the attr and char have the
 * high-bit set) to be sent (one pair at a time) to the "Term->pict_hook"
 * hook, which can draw these pairs in whatever way it would like.
 *
 * Normally, the "Term_wipe()" function is used only to display "blanks"
 * that were induced by "Term_clear()" or "Term_erase()", and then only
 * if the "attr_blank" and "char_blank" fields have not been redefined
 * to use "white space" instead of the default "black space".  Actually,
 * the "Term_wipe()" function is used to display all "black" text, such
 * as the default "spaces" created by "Term_clear()" and "Term_erase()".
 *
 * Note that the "Term->always_text" flag will disable the use of the
 * "Term_wipe()" function hook entirely, and force all text, even text
 * drawn in the color "black", to be explicitly drawn.  This is useful
 * for machines which implement "Term_wipe()" by just drawing spaces.
 *
 * Note that the "Term->always_pict" flag will disable the use of the
 * "Term_wipe()" function entirely, and force everything, even text
 * drawn in the attr "black", to be explicitly drawn.
 *
 * Note that if no "black" text is ever drawn, and if "attr_blank" is
 * not "zero", then the "Term_wipe" hook will never be used, even if
 * the "Term->always_text" flag is not set.
 *
 * This function does nothing unless the "Term" is "mapped", which allows
 * certain systems to optimize the handling of "closed" windows.
 *
 * On systems with a "soft" cursor, we must explicitly erase the cursor
 * before flushing the output, if needed, to prevent a "jumpy" refresh.
 * The actual method for this is horrible, but there is very little that
 * we can do to simplify it efficiently.  XXX XXX XXX
 *
 * On systems with a "hard" cursor, we will "hide" the cursor before
 * flushing the output, if needed, to avoid a "flickery" refresh.  It
 * would be nice to *always* hide the cursor during the refresh, but
 * this might be expensive (and/or ugly) on some machines.
 *
 * The "Term->icky_corner" flag is used to avoid calling "Term_wipe()"
 * or "Term_pict()" or "Term_text()" on the bottom right corner of the
 * window, which might induce "scrolling" or other nasty stuff on old
 * dumb terminals.  This flag is handled very efficiently.  We assume
 * that the "Term_curs()" call will prevent placing the cursor in the
 * corner, if needed, though I doubt such placement is ever a problem.
 * Currently, the use of "Term->icky_corner" and "Term->soft_cursor"
 * together may result in undefined behavior.
 */
#[no_mangle]
pub unsafe extern "C" fn Term_fresh() -> errr {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = (*Term).wid as libc::c_int;
    let mut h: libc::c_int = (*Term).hgt as libc::c_int;
    let mut y1: libc::c_int = (*Term).y1 as libc::c_int;
    let mut y2: libc::c_int = (*Term).y2 as libc::c_int;
    let mut old: *mut term_win = (*Term).old;
    let mut scr: *mut term_win = (*Term).scr;
    /* Do nothing unless "mapped" */
    if (*Term).mapped_flag == 0 { return 1 as libc::c_int }
    /* Trivial Refresh */
    if y1 > y2 && (*scr).cu as libc::c_int == (*old).cu as libc::c_int &&
           (*scr).cv as libc::c_int == (*old).cv as libc::c_int &&
           (*scr).cx as libc::c_int == (*old).cx as libc::c_int &&
           (*scr).cy as libc::c_int == (*old).cy as libc::c_int &&
           (*Term).total_erase == 0 {
        /* Nothing */
        return 1 as libc::c_int
    }
    /* Paranoia -- use "fake" hooks to prevent core dumps */
    if (*Term).curs_hook.is_none() {
        (*Term).curs_hook =
            Some(Term_curs_hack as
                     unsafe extern "C" fn(_: libc::c_int, _: libc::c_int)
                         -> errr)
    }
    if (*Term).wipe_hook.is_none() {
        (*Term).wipe_hook =
            Some(Term_wipe_hack as
                     unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                          _: libc::c_int) -> errr)
    }
    if (*Term).text_hook.is_none() {
        (*Term).text_hook =
            Some(Term_text_hack as
                     unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                          _: libc::c_int, _: byte_hack,
                                          _: *const libc::c_char) -> errr)
    }
    if (*Term).pict_hook.is_none() {
        (*Term).pict_hook =
            Some(Term_pict_hack as
                     unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                          _: libc::c_int, _: *const byte_hack,
                                          _: *const libc::c_char,
                                          _: *const byte_hack,
                                          _: *const libc::c_char,
                                          _: *const byte_hack,
                                          _: *const libc::c_char) -> errr)
    }
    /* Handle "total erase" */
    if (*Term).total_erase != 0 {
        let mut na: byte_hack = (*Term).attr_blank;
        let mut nc: libc::c_char = (*Term).char_blank;
        if do_movies == 1 as libc::c_int &&
               Term == angband_term[0 as libc::c_int as usize] {
            if cmovie_get_msecond() == 0 {
                fprintf(movfile,
                        b"S:%ld:\n\x00" as *const u8 as *const libc::c_char,
                        cmov_delta_time_msec);
            }
            fprintf(movfile, b"C:\n\x00" as *const u8 as *const libc::c_char);
            last_paused = 0 as libc::c_int
        }
        /* Physically erase the entire window */
        Term_xtra(3 as libc::c_int, 0 as libc::c_int);
        /* Hack -- clear all "cursor" data */
        (*old).cy = 0 as libc::c_int as byte_hack;
        (*old).cx = (*old).cy;
        (*old).cu = (*old).cx as bool_;
        (*old).cv = (*old).cu;
        /* Wipe each row */
        y = 0 as libc::c_int;
        while y < h {
            let mut aa: *mut byte_hack = *(*old).a.offset(y as isize);
            let mut cc: *mut libc::c_char = *(*old).c.offset(y as isize);
            let mut taa: *mut byte_hack = *(*old).ta.offset(y as isize);
            let mut tcc: *mut libc::c_char = *(*old).tc.offset(y as isize);
            let mut eaa: *mut byte_hack = *(*old).ea.offset(y as isize);
            let mut ecc: *mut libc::c_char = *(*old).ec.offset(y as isize);
            /* Wipe each column */
            x = 0 as libc::c_int;
            while x < w {
                /* Wipe each grid */
                let fresh34 = aa;
                aa = aa.offset(1);
                *fresh34 = na;
                let fresh35 = cc;
                cc = cc.offset(1);
                *fresh35 = nc;
                let fresh36 = taa;
                taa = taa.offset(1);
                *fresh36 = na;
                let fresh37 = tcc;
                tcc = tcc.offset(1);
                *fresh37 = nc;
                let fresh38 = eaa;
                eaa = eaa.offset(1);
                *fresh38 = na;
                let fresh39 = ecc;
                ecc = ecc.offset(1);
                *fresh39 = nc;
                x += 1
            }
            y += 1
        }
        /* Redraw every row */
        y1 = 0 as libc::c_int;
        (*Term).y1 = y1 as byte_hack;
        y2 = h - 1 as libc::c_int;
        (*Term).y2 = y2 as byte_hack;
        /* Redraw every column */
        y = 0 as libc::c_int;
        while y < h {
            *(*Term).x1.offset(y as isize) = 0 as libc::c_int as byte_hack;
            *(*Term).x2.offset(y as isize) =
                (w - 1 as libc::c_int) as byte_hack;
            y += 1
        }
        /* Forget "total erase" */
        (*Term).total_erase = 0 as libc::c_int as bool_
    }
    /* Cursor update -- Erase old Cursor */
    if (*Term).soft_cursor != 0 {
        /* Cursor was visible */
        if (*old).cu == 0 && (*old).cv as libc::c_int != 0 {
            let mut tx: libc::c_int = (*old).cx as libc::c_int;
            let mut ty: libc::c_int = (*old).cy as libc::c_int;
            let mut old_aa: *mut byte_hack = *(*old).a.offset(ty as isize);
            let mut old_cc: *mut libc::c_char = *(*old).c.offset(ty as isize);
            let mut oa: byte_hack = *old_aa.offset(tx as isize);
            let mut oc: libc::c_char = *old_cc.offset(tx as isize);
            let mut old_taa: *mut byte_hack = *(*old).ta.offset(ty as isize);
            let mut old_tcc: *mut libc::c_char =
                *(*old).tc.offset(ty as isize);
            let mut ota: byte_hack = *old_taa.offset(tx as isize);
            let mut otc: libc::c_char = *old_tcc.offset(tx as isize);
            let mut old_eaa: *mut byte_hack = *(*old).ea.offset(ty as isize);
            let mut old_ecc: *mut libc::c_char =
                *(*old).ec.offset(ty as isize);
            let mut oea: byte_hack = *old_eaa.offset(tx as isize);
            let mut oec: libc::c_char = *old_ecc.offset(tx as isize);
            /* Hack -- use "Term_pict()" always */
            if (*Term).always_pict != 0 {
                Some((*Term).pict_hook.expect("non-null function pointer")).expect("non-null function pointer")(tx,
                                                                                                                ty,
                                                                                                                1
                                                                                                                    as
                                                                                                                    libc::c_int,
                                                                                                                &mut oa,
                                                                                                                &mut oc,
                                                                                                                &mut ota,
                                                                                                                &mut otc,
                                                                                                                &mut oea,
                                                                                                                &mut oec);
            } else if (*Term).higher_pict as libc::c_int != 0 &&
                          oa as libc::c_int & 0x80 as libc::c_int != 0 {
                Some((*Term).pict_hook.expect("non-null function pointer")).expect("non-null function pointer")(tx,
                                                                                                                ty,
                                                                                                                1
                                                                                                                    as
                                                                                                                    libc::c_int,
                                                                                                                &mut oa,
                                                                                                                &mut oc,
                                                                                                                &mut ota,
                                                                                                                &mut otc,
                                                                                                                &mut oea,
                                                                                                                &mut oec);
            } else if oa as libc::c_int != 0 ||
                          (*Term).always_text as libc::c_int != 0 {
                Some((*Term).text_hook.expect("non-null function pointer")).expect("non-null function pointer")(tx,
                                                                                                                ty,
                                                                                                                1
                                                                                                                    as
                                                                                                                    libc::c_int,
                                                                                                                oa,
                                                                                                                &mut oc
                                                                                                                    as
                                                                                                                    *mut libc::c_char
                                                                                                                    as
                                                                                                                    cptr);
            } else {
                /* Hack -- use "Term_pict()" sometimes */
                /* Hack -- restore the actual character */
                /* Hack -- erase the grid */
                Some((*Term).wipe_hook.expect("non-null function pointer")).expect("non-null function pointer")(tx,
                                                                                                                ty,
                                                                                                                1
                                                                                                                    as
                                                                                                                    libc::c_int);
            }
        }
    } else if (*scr).cu as libc::c_int != 0 || (*scr).cv == 0 {
        /* Cursor Update -- Erase old Cursor */
        /* Cursor will be invisible */
        /* Make the cursor invisible */
        Term_xtra(4 as libc::c_int, 0 as libc::c_int);
    }
    /* Something to update */
    if y1 <= y2 {
        /* Handle "icky corner" */
        if (*Term).icky_corner != 0 {
            /* Avoid the corner */
            if y2 >= h - 1 as libc::c_int {
                /* Avoid the corner */
                if *(*Term).x2.offset((h - 1 as libc::c_int) as isize) as
                       libc::c_int > w - 2 as libc::c_int {
                    /* Avoid the corner */
                    *(*Term).x2.offset((h - 1 as libc::c_int) as isize) =
                        (w - 2 as libc::c_int) as byte_hack
                }
            }
        }
        /* Scan the "modified" rows */
        y = y1;
        while y <= y2 {
            let mut x1: libc::c_int =
                *(*Term).x1.offset(y as isize) as libc::c_int;
            let mut x2: libc::c_int =
                *(*Term).x2.offset(y as isize) as libc::c_int;
            /* Flush each "modified" row */
            if x1 <= x2 {
                if do_movies == 1 as libc::c_int &&
                       Term == angband_term[0 as libc::c_int as usize] {
                    /* Most magic happens here */
                    cmovie_record_line(y);
                    last_paused = 0 as libc::c_int
                }
                /* Always use "Term_pict()" */
                if (*Term).always_pict != 0 {
                    /* Flush the row */
                    Term_fresh_row_pict(y, x1, x2);
                } else if (*Term).higher_pict != 0 {
                    /* Sometimes use "Term_pict()" */
                    /* Flush the row */
                    Term_fresh_row_both(y, x1, x2);
                } else {
                    /* Never use "Term_pict()" */
                    /* Flush the row */
                    Term_fresh_row_text(y, x1, x2);
                }
                /* This row is all done */
                *(*Term).x1.offset(y as isize) = w as byte_hack;
                *(*Term).x2.offset(y as isize) =
                    0 as libc::c_int as byte_hack;
                /* Hack -- Flush that row (if allowed) */
                if (*Term).never_frosh == 0 {
                    Term_xtra(5 as libc::c_int, y);
                }
            }
            y += 1
        }
        /* No rows are invalid */
        (*Term).y1 = h as byte_hack;
        (*Term).y2 = 0 as libc::c_int as byte_hack
    }
    /* Cursor update -- Show new Cursor */
    if (*Term).soft_cursor != 0 {
        /* Draw the cursor */
        if (*scr).cu == 0 && (*scr).cv as libc::c_int != 0 {
            /* Call the cursor display routine */
            Some((*Term).curs_hook.expect("non-null function pointer")).expect("non-null function pointer")((*scr).cx
                                                                                                                as
                                                                                                                libc::c_int,
                                                                                                            (*scr).cy
                                                                                                                as
                                                                                                                libc::c_int);
        }
    } else if (*scr).cu != 0 {
        /* Cursor Update -- Show new Cursor */
        /* The cursor is useless, hide it */
        /* Paranoia -- Put the cursor NEAR where it belongs */
        Some((*Term).curs_hook.expect("non-null function pointer")).expect("non-null function pointer")(w
                                                                                                            -
                                                                                                            1
                                                                                                                as
                                                                                                                libc::c_int,
                                                                                                        (*scr).cy
                                                                                                            as
                                                                                                            libc::c_int);
        /* Make the cursor invisible */
			/* Term_xtra(TERM_XTRA_SHAPE, 0); */
    } else if (*scr).cv == 0 {
        /* The cursor is invisible, hide it */
        /* Paranoia -- Put the cursor where it belongs */
        Some((*Term).curs_hook.expect("non-null function pointer")).expect("non-null function pointer")((*scr).cx
                                                                                                            as
                                                                                                            libc::c_int,
                                                                                                        (*scr).cy
                                                                                                            as
                                                                                                            libc::c_int);
        /* Make the cursor invisible */
			/* Term_xtra(TERM_XTRA_SHAPE, 0); */
    } else {
        /* The cursor is visible, display it correctly */
        /* Put the cursor where it belongs */
        Some((*Term).curs_hook.expect("non-null function pointer")).expect("non-null function pointer")((*scr).cx
                                                                                                            as
                                                                                                            libc::c_int,
                                                                                                        (*scr).cy
                                                                                                            as
                                                                                                            libc::c_int);
        Term_xtra(4 as libc::c_int, 1 as libc::c_int);
    }
    /* Make the cursor visible */
    /* Save the "cursor state" */
    (*old).cu = (*scr).cu;
    (*old).cv = (*scr).cv;
    (*old).cx = (*scr).cx;
    (*old).cy = (*scr).cy;
    /* Actually flush the output */
    Term_xtra(6 as libc::c_int, 0 as libc::c_int);
    /* Success */
    return 0 as libc::c_int;
}
/* ** Output routines ***/
/*
 * Set the cursor visibility
 */
#[no_mangle]
pub unsafe extern "C" fn Term_set_cursor(mut v: libc::c_int) -> errr {
    /* Already done */
    if (*(*Term).scr).cv as libc::c_int == v { return 1 as libc::c_int }
    /* Change */
    (*(*Term).scr).cv = v as bool_;
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Place the cursor at a given location
 *
 * Note -- "illegal" requests do not move the cursor.
 */
#[no_mangle]
pub unsafe extern "C" fn Term_gotoxy(mut x: libc::c_int, mut y: libc::c_int)
 -> errr {
    let mut w: libc::c_int = (*Term).wid as libc::c_int;
    let mut h: libc::c_int = (*Term).hgt as libc::c_int;
    /* Verify */
    if x < 0 as libc::c_int || x >= w { return -(1 as libc::c_int) }
    if y < 0 as libc::c_int || y >= h { return -(1 as libc::c_int) }
    /* Remember the cursor */
    (*(*Term).scr).cx = x as byte_hack;
    (*(*Term).scr).cy = y as byte_hack;
    /* The cursor is not useless */
    (*(*Term).scr).cu = 0 as libc::c_int as bool_;
    /* Success */
    return 0 as libc::c_int;
}
/*
 * At a given location, place an attr/char
 * Do not change the cursor position
 * No visual changes until "Term_fresh()".
 */
#[no_mangle]
pub unsafe extern "C" fn Term_draw(mut x: libc::c_int, mut y: libc::c_int,
                                   mut a: byte_hack, mut c: libc::c_char)
 -> errr {
    let mut w: libc::c_int = (*Term).wid as libc::c_int;
    let mut h: libc::c_int = (*Term).hgt as libc::c_int;
    /* Verify location */
    if x < 0 as libc::c_int || x >= w { return -(1 as libc::c_int) }
    if y < 0 as libc::c_int || y >= h { return -(1 as libc::c_int) }
    /* Paranoia -- illegal char */
    if c == 0 { return -(2 as libc::c_int) }
    /* Queue it for later */
    Term_queue_char(x, y, a, c, 0 as libc::c_int as byte_hack,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as byte_hack,
                    0 as libc::c_int as libc::c_char);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Using the given attr, add the given char at the cursor.
 *
 * We return "-2" if the character is "illegal". XXX XXX
 *
 * We return "-1" if the cursor is currently unusable.
 *
 * We queue the given attr/char for display at the current
 * cursor location, and advance the cursor to the right,
 * marking it as unuable and returning "1" if it leaves
 * the screen, and otherwise returning "0".
 *
 * So when this function, or the following one, return a
 * positive value, future calls to either function will
 * return negative ones.
 */
#[no_mangle]
pub unsafe extern "C" fn Term_addch(mut a: byte_hack, mut c: libc::c_char)
 -> errr {
    let mut w: libc::c_int = (*Term).wid as libc::c_int;
    /* Handle "unusable" cursor */
    if (*(*Term).scr).cu != 0 { return -(1 as libc::c_int) }
    /* Paranoia -- no illegal chars */
    if c == 0 { return -(2 as libc::c_int) }
    /* Queue the given character for display */
    Term_queue_char((*(*Term).scr).cx as libc::c_int,
                    (*(*Term).scr).cy as libc::c_int, a, c,
                    0 as libc::c_int as byte_hack,
                    0 as libc::c_int as libc::c_char,
                    0 as libc::c_int as byte_hack,
                    0 as libc::c_int as libc::c_char);
    /* Advance the cursor */
    (*(*Term).scr).cx = (*(*Term).scr).cx.wrapping_add(1);
    /* Success */
    if ((*(*Term).scr).cx as libc::c_int) < w { return 0 as libc::c_int }
    /* Note "Useless" cursor */
    (*(*Term).scr).cu = 1 as libc::c_int as bool_;
    /* Note "Useless" cursor */
    return 1 as libc::c_int;
}
/*
 * At the current location, using an attr, add a string
 *
 * We also take a length "n", using negative values to imply
 * the largest possible value, and then we use the minimum of
 * this length and the "actual" length of the string as the
 * actual number of characters to attempt to display, never
 * displaying more characters than will actually fit, since
 * we do NOT attempt to "wrap" the cursor at the screen edge.
 *
 * We return "-1" if the cursor is currently unusable.
 * We return "N" if we were "only" able to write "N" chars,
 * even if all of the given characters fit on the screen,
 * and mark the cursor as unusable for future attempts.
 *
 * So when this function, or the preceding one, return a
 * positive value, future calls to either function will
 * return negative ones.
 */
#[no_mangle]
pub unsafe extern "C" fn Term_addstr(mut n: libc::c_int, mut a: byte_hack,
                                     mut s: cptr) -> errr {
    let mut k: libc::c_int = 0;
    let mut w: libc::c_int = (*Term).wid as libc::c_int;
    let mut res: errr = 0 as libc::c_int;
    /* Handle "unusable" cursor */
    if (*(*Term).scr).cu != 0 { return -(1 as libc::c_int) }
    /* Obtain maximal length */
    k = if n < 0 as libc::c_int { (w) + 1 as libc::c_int } else { n };
    /* Obtain the usable string length */
    n = 0 as libc::c_int;
    while n < k && *s.offset(n as isize) as libc::c_int != 0 {
        /* loop */
        n += 1
    }
    /* React to reaching the edge of the screen */
    if (*(*Term).scr).cx as libc::c_int + n >= w {
        n = w - (*(*Term).scr).cx as libc::c_int;
        res = n
    }
    /* Queue the first "n" characters for display */
    Term_queue_chars((*(*Term).scr).cx as libc::c_int,
                     (*(*Term).scr).cy as libc::c_int, n, a, s);
    /* Advance the cursor */
    (*(*Term).scr).cx = ((*(*Term).scr).cx as libc::c_int + n) as byte_hack;
    /* Hack -- Notice "Useless" cursor */
    if res != 0 { (*(*Term).scr).cu = 1 as libc::c_int as bool_ }
    /* Success (usually) */
    return res;
}
/*
 * Move to a location and, using an attr, add a char
 */
#[no_mangle]
pub unsafe extern "C" fn Term_putch(mut x: libc::c_int, mut y: libc::c_int,
                                    mut a: byte_hack, mut c: libc::c_char)
 -> errr {
    let mut res: errr = 0;
    /* Move first */
    res = Term_gotoxy(x, y);
    if res != 0 as libc::c_int { return res }
    /* Then add the char */
    res = Term_addch(a, c);
    if res != 0 as libc::c_int { return res }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Move to a location and, using an attr, add a string
 */
#[no_mangle]
pub unsafe extern "C" fn Term_putstr(mut x: libc::c_int, mut y: libc::c_int,
                                     mut n: libc::c_int, mut a: byte_hack,
                                     mut s: cptr) -> errr {
    let mut res: errr = 0;
    /* Move first */
    res = Term_gotoxy(x, y);
    if res != 0 as libc::c_int { return res }
    /* Then add the string */
    res = Term_addstr(n, a, s);
    if res != 0 as libc::c_int { return res }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Place cursor at (x,y), and clear the next "n" chars
 */
#[no_mangle]
pub unsafe extern "C" fn Term_erase(mut x: libc::c_int, mut y: libc::c_int,
                                    mut n: libc::c_int) -> errr {
    let mut i: libc::c_int = 0;
    let mut w: libc::c_int = (*Term).wid as libc::c_int;
    /* int h = Term->hgt; */
    let mut x1: libc::c_int = -(1 as libc::c_int);
    let mut x2: libc::c_int = -(1 as libc::c_int);
    let mut na: libc::c_int = (*Term).attr_blank as libc::c_int;
    let mut nc: libc::c_int = (*Term).char_blank as libc::c_int;
    let mut scr_aa: *mut byte_hack = 0 as *mut byte_hack;
    let mut scr_cc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut scr_taa: *mut byte_hack = 0 as *mut byte_hack;
    let mut scr_tcc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut scr_eaa: *mut byte_hack = 0 as *mut byte_hack;
    let mut scr_ecc: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Place cursor */
    if Term_gotoxy(x, y) != 0 { return -(1 as libc::c_int) }
    /* Force legal size */
    if x + n > w { n = w - x }
    /* Fast access */
    scr_aa = *(*(*Term).scr).a.offset(y as isize);
    scr_cc = *(*(*Term).scr).c.offset(y as isize);
    scr_taa = *(*(*Term).scr).ta.offset(y as isize);
    scr_tcc = *(*(*Term).scr).tc.offset(y as isize);
    scr_eaa = *(*(*Term).scr).ea.offset(y as isize);
    scr_ecc = *(*(*Term).scr).ec.offset(y as isize);
    if n > 0 as libc::c_int &&
           *scr_cc.offset(x as isize) as byte_hack as libc::c_int ==
               255 as libc::c_int &&
           *scr_aa.offset(x as isize) as libc::c_int == 255 as libc::c_int {
        x -= 1;
        n += 1
    }
    /* Scan every column */
    i = 0 as libc::c_int;
    while i < n {
        let mut oa: libc::c_int = *scr_aa.offset(x as isize) as libc::c_int;
        let mut oc: libc::c_int = *scr_cc.offset(x as isize) as libc::c_int;
        /* Hack -- Ignore "non-changes" */
        if !(oa == na && oc == nc) {
            /* Save the "literal" information */
            *scr_aa.offset(x as isize) = na as byte_hack;
            *scr_cc.offset(x as isize) = nc as libc::c_char;
            *scr_taa.offset(x as isize) = 0 as libc::c_int as byte_hack;
            *scr_tcc.offset(x as isize) = 0 as libc::c_int as libc::c_char;
            *scr_eaa.offset(x as isize) = 0 as libc::c_int as byte_hack;
            *scr_ecc.offset(x as isize) = 0 as libc::c_int as libc::c_char;
            /* Track minimum changed column */
            if x1 < 0 as libc::c_int { x1 = x }
            /* Track maximum changed column */
            x2 = x
        }
        i += 1;
        x += 1
    }
    /* Expand the "change area" as needed */
    if x1 >= 0 as libc::c_int {
        /* Check for new min/max row info */
        if y < (*Term).y1 as libc::c_int { (*Term).y1 = y as byte_hack }
        if y > (*Term).y2 as libc::c_int { (*Term).y2 = y as byte_hack }
        /* Check for new min/max col info in this row */
        if x1 < *(*Term).x1.offset(y as isize) as libc::c_int {
            *(*Term).x1.offset(y as isize) = x1 as byte_hack
        }
        if x2 > *(*Term).x2.offset(y as isize) as libc::c_int {
            *(*Term).x2.offset(y as isize) = x2 as byte_hack
        }
    }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Clear the entire window, and move to the top left corner
 *
 * Note the use of the special "total_erase" code
 */
#[no_mangle]
pub unsafe extern "C" fn Term_clear() -> errr {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = (*Term).wid as libc::c_int;
    let mut h: libc::c_int = (*Term).hgt as libc::c_int;
    let mut na: byte_hack = (*Term).attr_blank;
    let mut nc: libc::c_char = (*Term).char_blank;
    /* Cursor usable */
    (*(*Term).scr).cu = 0 as libc::c_int as bool_;
    /* Cursor to the top left */
    (*(*Term).scr).cy = 0 as libc::c_int as byte_hack;
    (*(*Term).scr).cx = (*(*Term).scr).cy;
    /* Wipe each row */
    y = 0 as libc::c_int;
    while y < h {
        let mut scr_aa: *mut byte_hack = *(*(*Term).scr).a.offset(y as isize);
        let mut scr_cc: *mut libc::c_char =
            *(*(*Term).scr).c.offset(y as isize);
        let mut scr_taa: *mut byte_hack =
            *(*(*Term).scr).ta.offset(y as isize);
        let mut scr_tcc: *mut libc::c_char =
            *(*(*Term).scr).tc.offset(y as isize);
        let mut scr_eaa: *mut byte_hack =
            *(*(*Term).scr).ea.offset(y as isize);
        let mut scr_ecc: *mut libc::c_char =
            *(*(*Term).scr).ec.offset(y as isize);
        /* Wipe each column */
        x = 0 as libc::c_int;
        while x < w {
            *scr_aa.offset(x as isize) = na;
            *scr_cc.offset(x as isize) = nc;
            *scr_taa.offset(x as isize) = 0 as libc::c_int as byte_hack;
            *scr_tcc.offset(x as isize) = 0 as libc::c_int as libc::c_char;
            *scr_eaa.offset(x as isize) = 0 as libc::c_int as byte_hack;
            *scr_ecc.offset(x as isize) = 0 as libc::c_int as libc::c_char;
            x += 1
        }
        /* This row has changed */
        *(*Term).x1.offset(y as isize) = 0 as libc::c_int as byte_hack;
        *(*Term).x2.offset(y as isize) = (w - 1 as libc::c_int) as byte_hack;
        y += 1
    }
    /* Every row has changed */
    (*Term).y1 = 0 as libc::c_int as byte_hack;
    (*Term).y2 = (h - 1 as libc::c_int) as byte_hack;
    /* Force "total erase" */
    (*Term).total_erase = 1 as libc::c_int as bool_;
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Redraw (and refresh) the whole window.
 */
#[no_mangle]
pub unsafe extern "C" fn Term_redraw() -> errr {
    /* Pat */
    if do_movies == 1 as libc::c_int &&
           Term == angband_term[0 as libc::c_int as usize] {
        if cmovie_get_msecond() == 0 {
            fprintf(movfile,
                    b"S:%ld:\n\x00" as *const u8 as *const libc::c_char,
                    cmov_delta_time_msec);
        }
        last_paused = 1 as libc::c_int
    }
    /* Endpat */
    /* Force "total erase" */
    (*Term).total_erase = 1 as libc::c_int as bool_;
    /* Hack -- Refresh */
    Term_fresh();
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Redraw part of a window.
 */
#[no_mangle]
pub unsafe extern "C" fn Term_redraw_section(mut x1: libc::c_int,
                                             mut y1: libc::c_int,
                                             mut x2: libc::c_int,
                                             mut y2: libc::c_int) -> errr {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Bounds checking */
    if y2 >= (*Term).hgt as libc::c_int {
        y2 = (*Term).hgt as libc::c_int - 1 as libc::c_int
    }
    if x2 >= (*Term).wid as libc::c_int {
        x2 = (*Term).wid as libc::c_int - 1 as libc::c_int
    }
    if y1 < 0 as libc::c_int { y1 = 0 as libc::c_int }
    if x1 < 0 as libc::c_int { x1 = 0 as libc::c_int }
    /* Set y limits */
    (*Term).y1 = y1 as byte_hack;
    (*Term).y2 = y2 as byte_hack;
    /* Set the x limits */
    i = (*Term).y1 as libc::c_int;
    while i <= (*Term).y2 as libc::c_int {
        *(*Term).x1.offset(i as isize) = x1 as byte_hack;
        *(*Term).x2.offset(i as isize) = x2 as byte_hack;
        c_ptr = *(*(*Term).old).c.offset(i as isize);
        /* Clear the section so it is redrawn */
        j = x1;
        while j <= x2 {
            /* Hack - set the old character to "none" */
            *c_ptr.offset(j as isize) = 0 as libc::c_int as libc::c_char;
            j += 1
        }
        i += 1
    }
    /* Hack -- Refresh */
    Term_fresh();
    /* Success */
    return 0 as libc::c_int;
}
/* ** Access routines ***/
/*
 * Extract the cursor visibility
 */
#[no_mangle]
pub unsafe extern "C" fn Term_get_cursor(mut v: *mut libc::c_int) -> errr {
    /* Extract visibility */
    *v = (*(*Term).scr).cv as libc::c_int;
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Extract the current window size
 */
#[no_mangle]
pub unsafe extern "C" fn Term_get_size(mut w: *mut libc::c_int,
                                       mut h: *mut libc::c_int) -> errr {
    /* Access the cursor */
    *w = (*Term).wid as libc::c_int;
    *h = (*Term).hgt as libc::c_int;
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Extract the current cursor location
 */
#[no_mangle]
pub unsafe extern "C" fn Term_locate(mut x: *mut libc::c_int,
                                     mut y: *mut libc::c_int) -> errr {
    /* Access the cursor */
    *x = (*(*Term).scr).cx as libc::c_int;
    *y = (*(*Term).scr).cy as libc::c_int;
    /* Warn about "useless" cursor */
    if (*(*Term).scr).cu != 0 { return 1 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * At a given location, determine the "current" attr and char
 * Note that this refers to what will be on the window after the
 * next call to "Term_fresh()".  It may or may not already be there.
 */
#[no_mangle]
pub unsafe extern "C" fn Term_what(mut x: libc::c_int, mut y: libc::c_int,
                                   mut a: *mut byte_hack,
                                   mut c: *mut libc::c_char) -> errr {
    let mut w: libc::c_int = (*Term).wid as libc::c_int;
    let mut h: libc::c_int = (*Term).hgt as libc::c_int;
    /* Verify location */
    if x < 0 as libc::c_int || x >= w { return -(1 as libc::c_int) }
    if y < 0 as libc::c_int || y >= h { return -(1 as libc::c_int) }
    /* Direct access */
    *a = *(*(*(*Term).scr).a.offset(y as isize)).offset(x as isize);
    *c = *(*(*(*Term).scr).c.offset(y as isize)).offset(x as isize);
    /* Success */
    return 0 as libc::c_int;
}
/* ** Input routines ***/
/*
 * Flush and forget the input
 */
#[no_mangle]
pub unsafe extern "C" fn Term_flush() -> errr {
    /* Hack -- Flush all events */
    Term_xtra(2 as libc::c_int, 0 as libc::c_int);
    /* Forget all keypresses */
    (*Term).key_tail = 0 as libc::c_int as u16b;
    (*Term).key_head = (*Term).key_tail;
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Add a keypress to the "queue"
 */
#[no_mangle]
pub unsafe extern "C" fn Term_keypress(mut k: libc::c_int) -> errr {
    /* Hack -- Refuse to enqueue non-keys */
    if k == 0 { return -(1 as libc::c_int) }
    /* Store the char, advance the queue */
    let fresh40 = (*Term).key_head;
    (*Term).key_head = (*Term).key_head.wrapping_add(1);
    *(*Term).key_queue.offset(fresh40 as isize) = k as libc::c_char;
    /* Circular queue, handle wrap */
    if (*Term).key_head as libc::c_int == (*Term).key_size as libc::c_int {
        (*Term).key_head = 0 as libc::c_int as u16b
    }
    /* Success (unless overflow) */
    if (*Term).key_head as libc::c_int != (*Term).key_tail as libc::c_int {
        return 0 as libc::c_int
    }
    /* Problem */
    return 1 as libc::c_int;
}
/*
 * Add a keypress to the FRONT of the "queue"
 */
#[no_mangle]
pub unsafe extern "C" fn Term_key_push(mut k: libc::c_int) -> errr {
    /* Hack -- Refuse to enqueue non-keys */
    if k == 0 { return -(1 as libc::c_int) }
    /* Hack -- Overflow may induce circular queue */
    if (*Term).key_tail as libc::c_int == 0 as libc::c_int {
        (*Term).key_tail = (*Term).key_size
    }
    /* Back up, Store the char */
    (*Term).key_tail = (*Term).key_tail.wrapping_sub(1);
    *(*Term).key_queue.offset((*Term).key_tail as isize) = k as libc::c_char;
    /* Success (unless overflow) */
    if (*Term).key_head as libc::c_int != (*Term).key_tail as libc::c_int {
        return 0 as libc::c_int
    }
    /* Problem */
    return 1 as libc::c_int;
}
/*
 * Check for a pending keypress on the key queue.
 *
 * Store the keypress, if any, in "ch", and return "0".
 * Otherwise store "zero" in "ch", and return "1".
 *
 * Wait for a keypress if "wait" is true.
 *
 * Remove the keypress if "take" is true.
 */
#[no_mangle]
pub unsafe extern "C" fn Term_inkey(mut ch: *mut libc::c_char,
                                    mut wait: bool_, mut take: bool_)
 -> errr {
    /* Assume no key */
    *ch = '\u{0}' as i32 as libc::c_char;
    /* Hack -- get bored */
    if (*Term).never_bored == 0 {
        /* Process random events */
        Term_xtra(9 as libc::c_int, 0 as libc::c_int);
    }
    /* PatN */
    if do_movies == 1 as libc::c_int && last_paused == 0 as libc::c_int &&
           cmovie_get_msecond() == 0 {
        fprintf(movfile, b"S:%ld:\n\x00" as *const u8 as *const libc::c_char,
                cmov_delta_time_msec);
        last_paused = 1 as libc::c_int
    }
    /* PatNEnd */
    /* Wait */
    if wait != 0 {
        /* Process pending events while necessary */
        while (*Term).key_head as libc::c_int ==
                  (*Term).key_tail as libc::c_int {
            /* Process events (wait for one) */
            Term_xtra(1 as libc::c_int, 1 as libc::c_int);
        }
    } else if (*Term).key_head as libc::c_int ==
                  (*Term).key_tail as libc::c_int {
        /* Do not Wait */
        /* Process pending events if necessary */
        /* Process events (do not wait) */
        Term_xtra(1 as libc::c_int, 0 as libc::c_int);
    }
    /* No keys are ready */
    if (*Term).key_head as libc::c_int == (*Term).key_tail as libc::c_int {
        return 1 as libc::c_int
    }
    /* Extract the next keypress */
    *ch = *(*Term).key_queue.offset((*Term).key_tail as isize);
    /* If requested, advance the queue, wrap around if necessary */
    if take as libc::c_int != 0 &&
           {
               (*Term).key_tail = (*Term).key_tail.wrapping_add(1);
               ((*Term).key_tail as libc::c_int) ==
                   (*Term).key_size as libc::c_int
           } {
        (*Term).key_tail = 0 as libc::c_int as u16b
    }
    /* Success */
    return 0 as libc::c_int;
}
/* ** Extra routines ***/
/*
 * Save the "requested" screen into the "memorized" screen
 *
 * Every "Term_save()" should match exactly one "Term_load()"
 */
#[no_mangle]
pub unsafe extern "C" fn Term_save() -> errr {
    let mut w: libc::c_int = (*Term).wid as libc::c_int;
    let mut h: libc::c_int = (*Term).hgt as libc::c_int;
    /* Create */
    if (*Term).mem.is_null() {
        /* Allocate window */
        (*Term).mem =
            memset(ralloc(::std::mem::size_of::<term_win>() as libc::c_ulong)
                       as *mut libc::c_char as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<term_win>() as libc::c_ulong) as
                *mut term_win;
        /* Initialize window */
        term_win_init((*Term).mem, w, h);
    }
    /* Grab */
    term_win_copy((*Term).mem, (*Term).scr, w, h);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Same as before but can save more than once
 */
#[no_mangle]
pub unsafe extern "C" fn Term_save_to() -> *mut term_win {
    let mut w: libc::c_int = (*Term).wid as libc::c_int;
    let mut h: libc::c_int = (*Term).hgt as libc::c_int;
    let mut save: *mut term_win = 0 as *mut term_win;
    /* Allocate window */
    save =
        memset(ralloc(::std::mem::size_of::<term_win>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<term_win>() as libc::c_ulong) as
            *mut term_win;
    /* Initialize window */
    term_win_init(save, w, h);
    /* Grab */
    term_win_copy(save, (*Term).scr, w, h);
    /* Success */
    return save;
}
/*
 * Restore the "requested" contents (see above).
 *
 * Every "Term_save()" should match exactly one "Term_load()"
 */
#[no_mangle]
pub unsafe extern "C" fn Term_load() -> errr {
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = (*Term).wid as libc::c_int;
    let mut h: libc::c_int = (*Term).hgt as libc::c_int;
    /* Create */
    if (*Term).mem.is_null() {
        /* Allocate window */
        (*Term).mem =
            memset(ralloc(::std::mem::size_of::<term_win>() as libc::c_ulong)
                       as *mut libc::c_char as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<term_win>() as libc::c_ulong) as
                *mut term_win;
        /* Initialize window */
        term_win_init((*Term).mem, w, h);
    }
    /* Load */
    term_win_copy((*Term).scr, (*Term).mem, w, h);
    /* Assume change */
    y = 0 as libc::c_int;
    while y < h {
        /* Assume change */
        *(*Term).x1.offset(y as isize) = 0 as libc::c_int as byte_hack;
        *(*Term).x2.offset(y as isize) = (w - 1 as libc::c_int) as byte_hack;
        y += 1
    }
    /* Assume change */
    (*Term).y1 = 0 as libc::c_int as byte_hack;
    (*Term).y2 = (h - 1 as libc::c_int) as byte_hack;
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Same as previous but allow to save more than one
 */
#[no_mangle]
pub unsafe extern "C" fn Term_load_from(mut save: *mut term_win,
                                        mut final_0: bool_) -> errr {
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = (*Term).wid as libc::c_int;
    let mut h: libc::c_int = (*Term).hgt as libc::c_int;
    /* Create */
    if save.is_null() { return 1 as libc::c_int }
    /* Load */
    term_win_copy((*Term).scr, save, w, h);
    /* Assume change */
    y = 0 as libc::c_int;
    while y < h {
        /* Assume change */
        *(*Term).x1.offset(y as isize) = 0 as libc::c_int as byte_hack;
        *(*Term).x2.offset(y as isize) = (w - 1 as libc::c_int) as byte_hack;
        y += 1
    }
    /* Assume change */
    (*Term).y1 = 0 as libc::c_int as byte_hack;
    (*Term).y2 = (h - 1 as libc::c_int) as byte_hack;
    /* Free is requested */
    if final_0 != 0 {
        rnfree(save as vptr,
               ::std::mem::size_of::<term_win>() as libc::c_ulong);
    }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Exchange the "requested" screen with the "tmp" screen
 */
#[no_mangle]
pub unsafe extern "C" fn Term_exchange() -> errr {
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = (*Term).wid as libc::c_int;
    let mut h: libc::c_int = (*Term).hgt as libc::c_int;
    let mut exchanger: *mut term_win = 0 as *mut term_win;
    /* Create */
    if (*Term).tmp.is_null() {
        /* Allocate window */
        (*Term).tmp =
            memset(ralloc(::std::mem::size_of::<term_win>() as libc::c_ulong)
                       as *mut libc::c_char as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<term_win>() as libc::c_ulong) as
                *mut term_win;
        /* Initialize window */
        term_win_init((*Term).tmp, w, h);
    }
    /* Swap */
    exchanger = (*Term).scr;
    (*Term).scr = (*Term).tmp;
    (*Term).tmp = exchanger;
    /* Assume change */
    y = 0 as libc::c_int;
    while y < h {
        /* Assume change */
        *(*Term).x1.offset(y as isize) = 0 as libc::c_int as byte_hack;
        *(*Term).x2.offset(y as isize) = (w - 1 as libc::c_int) as byte_hack;
        y += 1
    }
    /* Assume change */
    (*Term).y1 = 0 as libc::c_int as byte_hack;
    (*Term).y2 = (h - 1 as libc::c_int) as byte_hack;
    /* Success */
    return 0 as libc::c_int;
}
/*
 * React to a new physical window size.
 */
#[no_mangle]
pub unsafe extern "C" fn Term_resize(mut w: libc::c_int, mut h: libc::c_int)
 -> errr {
    let mut i: libc::c_int = 0;
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    let mut hold_x1: *mut byte_hack = 0 as *mut byte_hack;
    let mut hold_x2: *mut byte_hack = 0 as *mut byte_hack;
    let mut hold_old: *mut term_win = 0 as *mut term_win;
    let mut hold_scr: *mut term_win = 0 as *mut term_win;
    let mut hold_mem: *mut term_win = 0 as *mut term_win;
    let mut hold_tmp: *mut term_win = 0 as *mut term_win;
    /* Resizing is forbidden */
    if (*Term).fixed_shape != 0 { return -(1 as libc::c_int) }
    /* Ignore illegal changes */
    if w < 1 as libc::c_int || h < 1 as libc::c_int {
        return -(1 as libc::c_int)
    }
    /* Ignore non-changes */
    if (*Term).wid as libc::c_int == w && (*Term).hgt as libc::c_int == h &&
           arg_bigtile as libc::c_int == use_bigtile as libc::c_int {
        return 1 as libc::c_int
    }
    use_bigtile = arg_bigtile;
    /* Minimum dimensions */
    wid =
        if (*Term).wid as libc::c_int > w {
            w
        } else { (*Term).wid as libc::c_int };
    hgt =
        if (*Term).hgt as libc::c_int > h {
            h
        } else { (*Term).hgt as libc::c_int };
    /* Save scanners */
    hold_x1 = (*Term).x1;
    hold_x2 = (*Term).x2;
    /* Save old window */
    hold_old = (*Term).old;
    /* Save old window */
    hold_scr = (*Term).scr;
    /* Save old window */
    hold_mem = (*Term).mem;
    /* Save old window */
    hold_tmp = (*Term).tmp;
    /* Create new scanners */
    (*Term).x1 =
        memset(ralloc((h as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (h as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                    as libc::c_ulong)) as
            *mut byte_hack;
    (*Term).x2 =
        memset(ralloc((h as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (h as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                    as libc::c_ulong)) as
            *mut byte_hack;
    /* Create new window */
    (*Term).old =
        memset(ralloc(::std::mem::size_of::<term_win>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<term_win>() as libc::c_ulong) as
            *mut term_win;
    /* Initialize new window */
    term_win_init((*Term).old, w, h);
    /* Save the contents */
    term_win_copy((*Term).old, hold_old, wid, hgt);
    /* Create new window */
    (*Term).scr =
        memset(ralloc(::std::mem::size_of::<term_win>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<term_win>() as libc::c_ulong) as
            *mut term_win;
    /* Initialize new window */
    term_win_init((*Term).scr, w, h);
    /* Save the contents */
    term_win_copy((*Term).scr, hold_scr, wid, hgt);
    /* If needed */
    if !hold_mem.is_null() {
        /* Create new window */
        (*Term).mem =
            memset(ralloc(::std::mem::size_of::<term_win>() as libc::c_ulong)
                       as *mut libc::c_char as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<term_win>() as libc::c_ulong) as
                *mut term_win;
        /* Initialize new window */
        term_win_init((*Term).mem, w, h);
        /* Save the contents */
        term_win_copy((*Term).mem, hold_mem, wid, hgt);
    }
    /* If needed */
    if !hold_tmp.is_null() {
        /* Create new window */
        (*Term).tmp =
            memset(ralloc(::std::mem::size_of::<term_win>() as libc::c_ulong)
                       as *mut libc::c_char as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<term_win>() as libc::c_ulong) as
                *mut term_win;
        /* Initialize new window */
        term_win_init((*Term).tmp, w, h);
        /* Save the contents */
        term_win_copy((*Term).tmp, hold_tmp, wid, hgt);
    }
    /* Free some arrays */
    hold_x1 =
        rnfree(hold_x1 as vptr,
               ((*Term).hgt as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                    as libc::c_ulong)) as
            *mut byte_hack;
    hold_x2 =
        rnfree(hold_x2 as vptr,
               ((*Term).hgt as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                    as libc::c_ulong)) as
            *mut byte_hack;
    /* Nuke */
    term_win_nuke(hold_old, (*Term).wid as libc::c_int,
                  (*Term).hgt as libc::c_int);
    /* Kill */
    hold_old =
        rnfree(hold_old as vptr,
               ::std::mem::size_of::<term_win>() as libc::c_ulong) as
            *mut term_win;
    /* Illegal cursor */
    if (*(*Term).old).cx as libc::c_int >= w {
        (*(*Term).old).cu = 1 as libc::c_int as bool_
    }
    if (*(*Term).old).cy as libc::c_int >= h {
        (*(*Term).old).cu = 1 as libc::c_int as bool_
    }
    /* Nuke */
    term_win_nuke(hold_scr, (*Term).wid as libc::c_int,
                  (*Term).hgt as libc::c_int);
    /* Kill */
    hold_scr =
        rnfree(hold_scr as vptr,
               ::std::mem::size_of::<term_win>() as libc::c_ulong) as
            *mut term_win;
    /* Illegal cursor */
    if (*(*Term).scr).cx as libc::c_int >= w {
        (*(*Term).scr).cu = 1 as libc::c_int as bool_
    }
    if (*(*Term).scr).cy as libc::c_int >= h {
        (*(*Term).scr).cu = 1 as libc::c_int as bool_
    }
    /* If needed */
    if !hold_mem.is_null() {
        /* Nuke */
        term_win_nuke(hold_mem, (*Term).wid as libc::c_int,
                      (*Term).hgt as libc::c_int);
        /* Kill */
        hold_mem =
            rnfree(hold_mem as vptr,
                   ::std::mem::size_of::<term_win>() as libc::c_ulong) as
                *mut term_win;
        /* Illegal cursor */
        if (*(*Term).mem).cx as libc::c_int >= w {
            (*(*Term).mem).cu = 1 as libc::c_int as bool_
        }
        if (*(*Term).mem).cy as libc::c_int >= h {
            (*(*Term).mem).cu = 1 as libc::c_int as bool_
        }
    }
    /* If needed */
    if !hold_tmp.is_null() {
        /* Nuke */
        term_win_nuke(hold_tmp, (*Term).wid as libc::c_int,
                      (*Term).hgt as libc::c_int);
        /* Kill */
        hold_tmp =
            rnfree(hold_tmp as vptr,
                   ::std::mem::size_of::<term_win>() as libc::c_ulong) as
                *mut term_win;
        /* Illegal cursor */
        if (*(*Term).tmp).cx as libc::c_int >= w {
            (*(*Term).tmp).cu = 1 as libc::c_int as bool_
        }
        if (*(*Term).tmp).cy as libc::c_int >= h {
            (*(*Term).tmp).cu = 1 as libc::c_int as bool_
        }
    }
    /* Save new size */
    (*Term).wid = w as byte_hack;
    (*Term).hgt = h as byte_hack;
    /* Force "total erase" */
    (*Term).total_erase = 1 as libc::c_int as bool_;
    /* Assume change */
    i = 0 as libc::c_int;
    while i < h {
        /* Assume change */
        *(*Term).x1.offset(i as isize) = 0 as libc::c_int as byte_hack;
        *(*Term).x2.offset(i as isize) = (w - 1 as libc::c_int) as byte_hack;
        i += 1
    }
    /* Assume change */
    (*Term).y1 = 0 as libc::c_int as byte_hack;
    (*Term).y2 = (h - 1 as libc::c_int) as byte_hack;
    /* Execute the "resize_hook" hook, if available */
    if (*Term).resize_hook.is_some() {
        (*Term).resize_hook.expect("non-null function pointer")();
    }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Activate a new Term (and deactivate the current Term)
 *
 * This function is extremely important, and also somewhat bizarre.
 * It is the only function that should "modify" the value of "Term".
 *
 * To "create" a valid "term", one should do "term_init(t)", then
 * set the various flags and hooks, and then do "Term_activate(t)".
 */
#[no_mangle]
pub unsafe extern "C" fn Term_activate(mut t: *mut term) -> errr {
    /* Hack -- already done */
    if Term == t { return 1 as libc::c_int }
    /* Deactivate the old Term */
    if !Term.is_null() { Term_xtra(12 as libc::c_int, 0 as libc::c_int); }
    /* Hack -- Call the special "init" hook */
    if !t.is_null() && (*t).active_flag == 0 {
        /* Call the "init" hook */
        if (*t).init_hook.is_some() {
            Some((*t).init_hook.expect("non-null function pointer")).expect("non-null function pointer")(t);
        }
        /* Remember */
        (*t).active_flag = 1 as libc::c_int as bool_;
        /* Assume mapped */
        (*t).mapped_flag = 1 as libc::c_int as bool_
    }
    /* Remember the Term */
    Term = t;
    /* Activate the new Term */
    if !Term.is_null() { Term_xtra(12 as libc::c_int, 1 as libc::c_int); }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Nuke a term
 */
#[no_mangle]
pub unsafe extern "C" fn term_nuke(mut t: *mut term) -> errr {
    let mut w: libc::c_int = (*t).wid as libc::c_int;
    let mut h: libc::c_int = (*t).hgt as libc::c_int;
    /* Hack -- Call the special "nuke" hook */
    if (*t).active_flag != 0 {
        /* Call the "nuke" hook */
        if (*t).nuke_hook.is_some() {
            Some((*t).nuke_hook.expect("non-null function pointer")).expect("non-null function pointer")(t);
        }
        /* Remember */
        (*t).active_flag = 0 as libc::c_int as bool_;
        /* Assume not mapped */
        (*t).mapped_flag = 0 as libc::c_int as bool_
    }
    /* Nuke "displayed" */
    term_win_nuke((*t).old, w, h);
    /* Kill "displayed" */
    (*t).old =
        rnfree((*t).old as vptr,
               ::std::mem::size_of::<term_win>() as libc::c_ulong) as
            *mut term_win;
    /* Nuke "requested" */
    term_win_nuke((*t).scr, w, h);
    /* Kill "requested" */
    (*t).scr =
        rnfree((*t).scr as vptr,
               ::std::mem::size_of::<term_win>() as libc::c_ulong) as
            *mut term_win;
    /* If needed */
    if !(*t).mem.is_null() {
        /* Nuke "memorized" */
        term_win_nuke((*t).mem, w, h);
        /* Kill "memorized" */
        (*t).mem =
            rnfree((*t).mem as vptr,
                   ::std::mem::size_of::<term_win>() as libc::c_ulong) as
                *mut term_win
    }
    /* If needed */
    if !(*t).tmp.is_null() {
        /* Nuke "temporary" */
        term_win_nuke((*t).tmp, w, h);
        /* Kill "temporary" */
        (*t).tmp =
            rnfree((*t).tmp as vptr,
                   ::std::mem::size_of::<term_win>() as libc::c_ulong) as
                *mut term_win
    }
    /* Free some arrays */
    (*t).x1 =
        rnfree((*t).x1 as vptr,
               (h as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                    as libc::c_ulong)) as
            *mut byte_hack;
    (*t).x2 =
        rnfree((*t).x2 as vptr,
               (h as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                    as libc::c_ulong)) as
            *mut byte_hack;
    /* Free the input queue */
    (*t).key_queue =
        rnfree((*t).key_queue as vptr,
               ((*t).key_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialize a term, using a window of the given size.
 * Also prepare the "input queue" for "k" keypresses
 * By default, the cursor starts out "invisible"
 * By default, we "erase" using "black spaces"
 */
#[no_mangle]
pub unsafe extern "C" fn term_init(mut t: *mut term, mut w: libc::c_int,
                                   mut h: libc::c_int, mut k: libc::c_int)
 -> errr {
    let mut y: libc::c_int = 0;
    /* Wipe it */
    memset(t as *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<term>() as libc::c_ulong);
    /* Prepare the input queue */
    (*t).key_tail = 0 as libc::c_int as u16b;
    (*t).key_head = (*t).key_tail;
    /* Determine the input queue size */
    (*t).key_size = k as u16b;
    /* Allocate the input queue */
    (*t).key_queue =
        memset(ralloc(((*t).key_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*t).key_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* Save the size */
    (*t).wid = w as byte_hack;
    (*t).hgt = h as byte_hack;
    /* Allocate change arrays */
    (*t).x1 =
        memset(ralloc((h as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (h as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                    as libc::c_ulong)) as
            *mut byte_hack;
    (*t).x2 =
        memset(ralloc((h as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (h as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                    as libc::c_ulong)) as
            *mut byte_hack;
    /* Allocate "displayed" */
    (*t).old =
        memset(ralloc(::std::mem::size_of::<term_win>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<term_win>() as libc::c_ulong) as
            *mut term_win;
    /* Initialize "displayed" */
    term_win_init((*t).old, w, h);
    /* Allocate "requested" */
    (*t).scr =
        memset(ralloc(::std::mem::size_of::<term_win>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<term_win>() as libc::c_ulong) as
            *mut term_win;
    /* Initialize "requested" */
    term_win_init((*t).scr, w, h);
    /* Assume change */
    y = 0 as libc::c_int;
    while y < h {
        /* Assume change */
        *(*t).x1.offset(y as isize) = 0 as libc::c_int as byte_hack;
        *(*t).x2.offset(y as isize) = (w - 1 as libc::c_int) as byte_hack;
        y += 1
    }
    /* Assume change */
    (*t).y1 = 0 as libc::c_int as byte_hack;
    (*t).y2 = (h - 1 as libc::c_int) as byte_hack;
    /* Force "total erase" */
    (*t).total_erase = 1 as libc::c_int as bool_;
    /* Default "blank" */
    (*t).attr_blank = 0 as libc::c_int as byte_hack;
    (*t).char_blank = ' ' as i32 as libc::c_char;
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Determine if we are called in the same second as the last time?
 * This *ASSUMES* that time_t is seconds past something. Is this portable?
 */
#[no_mangle]
pub unsafe extern "C" fn cmovie_get_msecond() -> libc::c_int {
    /* Very precise but needs main-foo.c to define TERM_XTRA_GET_DELAY */
    Term_xtra(14 as libc::c_int, 0 as libc::c_int);
    cmov_delta_time_msec = Term_xtra_long - cmov_last_time_msec;
    cmov_last_time_msec = Term_xtra_long;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cmovie_init_second() {
    /* Precise but need main-foo.c to have TERM_XTRA_GET_DELAY */
    Term_xtra(14 as libc::c_int, 0 as libc::c_int);
    cmov_last_time_msec = Term_xtra_long;
}
