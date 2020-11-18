use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ldat;
    pub type __dirstream;
    #[no_mangle]
    static mut arg_graphics: bool_;
    #[no_mangle]
    static mut use_graphics: bool_;
    #[no_mangle]
    static mut angband_term: [*mut term; 8];
    #[no_mangle]
    static mut angband_color_table: [[byte_hack; 4]; 256];
    #[no_mangle]
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    static mut quit_aux: Option<unsafe extern "C" fn(_: cptr) -> ()>;
    #[no_mangle]
    static mut core_aux: Option<unsafe extern "C" fn(_: cptr) -> ()>;
    #[no_mangle]
    fn prefix(s: cptr, t: cptr) -> bool_;
    #[no_mangle]
    fn quit(str: cptr);
    #[no_mangle]
    fn string_make(str: cptr) -> cptr;
    #[no_mangle]
    fn string_free(str: cptr) -> errr;
    #[no_mangle]
    fn plog_fmt(fmt: cptr, _: ...);
    #[no_mangle]
    static mut Term: *mut term;
    #[no_mangle]
    fn Term_xtra(n: libc::c_int, v: libc::c_int) -> errr;
    #[no_mangle]
    static mut Term_xtra_long: libc::c_long;
    #[no_mangle]
    static mut scansubdir_dir: [libc::c_char; 1024];
    #[no_mangle]
    static mut scansubdir_max: libc::c_int;
    #[no_mangle]
    static mut scansubdir_result: [cptr; 255];
    #[no_mangle]
    fn Term_keypress(k: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_activate(t: *mut term) -> errr;
    #[no_mangle]
    fn term_init(t: *mut term, w: libc::c_int, h: libc::c_int, k: libc::c_int)
     -> errr;
    #[no_mangle]
    fn strncat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut acs_map: [chtype; 0];
    #[no_mangle]
    fn can_change_color() -> bool;
    #[no_mangle]
    fn cbreak() -> libc::c_int;
    #[no_mangle]
    fn delwin(_: *mut WINDOW) -> libc::c_int;
    #[no_mangle]
    fn echo() -> libc::c_int;
    #[no_mangle]
    fn endwin() -> libc::c_int;
    #[no_mangle]
    fn has_colors() -> bool;
    #[no_mangle]
    fn initscr() -> *mut WINDOW;
    #[no_mangle]
    fn init_color(_: libc::c_short, _: libc::c_short, _: libc::c_short,
                  _: libc::c_short) -> libc::c_int;
    #[no_mangle]
    fn init_pair(_: libc::c_short, _: libc::c_short, _: libc::c_short)
     -> libc::c_int;
    #[no_mangle]
    fn mvcur(_: libc::c_int, _: libc::c_int, _: libc::c_int, _: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn newwin(_: libc::c_int, _: libc::c_int, _: libc::c_int, _: libc::c_int)
     -> *mut WINDOW;
    #[no_mangle]
    fn nl() -> libc::c_int;
    #[no_mangle]
    fn nocbreak() -> libc::c_int;
    #[no_mangle]
    fn noecho() -> libc::c_int;
    #[no_mangle]
    fn nonl() -> libc::c_int;
    #[no_mangle]
    fn start_color() -> libc::c_int;
    #[no_mangle]
    fn waddch(_: *mut WINDOW, _: chtype) -> libc::c_int;
    #[no_mangle]
    fn wattrset(_: *mut WINDOW, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wclear(_: *mut WINDOW) -> libc::c_int;
    #[no_mangle]
    fn wclrtoeol(_: *mut WINDOW) -> libc::c_int;
    #[no_mangle]
    fn wmove(_: *mut WINDOW, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn wrefresh(_: *mut WINDOW) -> libc::c_int;
    #[no_mangle]
    fn wtouchln(_: *mut WINDOW, _: libc::c_int, _: libc::c_int,
                _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut curscr: *mut WINDOW;
    #[no_mangle]
    static mut stdscr: *mut WINDOW;
    #[no_mangle]
    static mut COLORS: libc::c_int;
    #[no_mangle]
    static mut COLOR_PAIRS: libc::c_int;
    #[no_mangle]
    static mut COLS: libc::c_int;
    #[no_mangle]
    static mut LINES: libc::c_int;
    #[no_mangle]
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    #[no_mangle]
    fn tcsetattr(__fd: libc::c_int, __optional_actions: libc::c_int,
                 __termios_p: *const termios) -> libc::c_int;
    #[no_mangle]
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    #[no_mangle]
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type vptr = *mut libc::c_void;
pub type cptr = *const libc::c_char;
pub type errr = libc::c_int;
pub type byte_hack = libc::c_uchar;
pub type bool_ = libc::c_char;
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
pub type chtype = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _win_st {
    pub _cury: libc::c_short,
    pub _curx: libc::c_short,
    pub _maxy: libc::c_short,
    pub _maxx: libc::c_short,
    pub _begy: libc::c_short,
    pub _begx: libc::c_short,
    pub _flags: libc::c_short,
    pub _attrs: attr_t,
    pub _bkgd: chtype,
    pub _notimeout: bool,
    pub _clear: bool,
    pub _leaveok: bool,
    pub _scroll: bool,
    pub _idlok: bool,
    pub _idcok: bool,
    pub _immed: bool,
    pub _sync: bool,
    pub _use_keypad: bool,
    pub _delay: libc::c_int,
    pub _line: *mut ldat,
    pub _regtop: libc::c_short,
    pub _regbottom: libc::c_short,
    pub _parx: libc::c_int,
    pub _pary: libc::c_int,
    pub _parent: *mut WINDOW,
    pub _pad: pdat,
    pub _yoffset: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdat {
    pub _pad_y: libc::c_short,
    pub _pad_x: libc::c_short,
    pub _pad_top: libc::c_short,
    pub _pad_left: libc::c_short,
    pub _pad_bottom: libc::c_short,
    pub _pad_right: libc::c_short,
}
pub type WINDOW = _win_st;
pub type attr_t = chtype;
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct term_data {
    pub t: term,
    pub win: *mut WINDOW,
}
/* File: main-gcu.c */
/*
 * Copyright (c) 1997 Ben Harrison, and others
 *
 * This software may be copied and distributed for educational, research,
 * and not for profit purposes provided that this copyright and statement
 * are included in all such copies.
 */
/*
 * This file helps Angband run on Unix/Curses machines.
 *
 *
 * To use this file, you must define "USE_GCU" in the Makefile.
 *
 *
 * Note that this file is not "intended" to support non-Unix machines,
 * nor is it intended to support VMS or other bizarre setups.
 *
 * Also, this package assumes that the underlying "curses" handles both
 * the "nonl()" and "cbreak()" commands correctly, see the "OPTION" below.
 *
 * This code should work with most versions of "curses" or "ncurses",
 * and the "main-ncu.c" file (and USE_NCU define) are no longer used.
 *
 * This file provides up to 4 term windows.
 *
 * This file will attempt to redefine the screen colors to conform to
 * standard Angband colors.  It will only do so if the terminal type
 * indicates that it can do so.  See the page:
 *
 *     http://www.umr.edu/~keldon/ang-patch/ncurses_color.html
 *
 * for information on this.
 *
 * Consider the use of "savetty()" and "resetty()".  XXX XXX XXX
 */
/*
 * Hack -- play games with "bool" and "term"
 */
/* Avoid 'struct term' name conflict with <curses.h> (via <term.h>) on AIX */
/*
 * Include the proper "header" file
 */
/*
 * Try redefining the colors at startup.
 */
/*
 * Hack -- try to guess which systems use what commands
 * Hack -- allow one of the "USE_Txxxxx" flags to be pre-set.
 * Mega-Hack -- try to guess when "POSIX" is available.
 * If the user defines two of these, we will probably crash.
 */
/*
 * POSIX stuff
 */
/*
 * One version needs these files
 */
/*
 * The other needs these files
 */
/* /me pffts Solaris */
/*
 * XXX XXX Hack -- POSIX uses "O_NONBLOCK" instead of "O_NDELAY"
 *
 * They should both work due to the "(i != 1)" test below.
 */
/*
 * OPTION: some machines lack "cbreak()"
 * On these machines, we use an older definition
 */
/* #define cbreak() crmode() */
/*
 * OPTION: some machines cannot handle "nonl()" and "nl()"
 * On these machines, we can simply ignore those commands.
 */
/* #define nonl() */
/* #define nl() */
/*
 * Save the "normal" and "angband" terminal settings
 */
static mut norm_termios: termios =
    termios{c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,};
static mut game_termios: termios =
    termios{c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,};
/* Information about our windows */
static mut data: [term_data; 4] =
    [term_data{t:
                   term{user: 0 as *const libc::c_void as *mut libc::c_void,
                        data: 0 as *const libc::c_void as *mut libc::c_void,
                        user_flag: 0,
                        data_flag: 0,
                        active_flag: 0,
                        mapped_flag: 0,
                        total_erase: 0,
                        fixed_shape: 0,
                        icky_corner: 0,
                        soft_cursor: 0,
                        always_pict: 0,
                        higher_pict: 0,
                        always_text: 0,
                        unused_flag: 0,
                        never_bored: 0,
                        never_frosh: 0,
                        attr_blank: 0,
                        char_blank: 0,
                        key_queue:
                            0 as *const libc::c_char as *mut libc::c_char,
                        key_head: 0,
                        key_tail: 0,
                        key_xtra: 0,
                        key_size: 0,
                        wid: 0,
                        hgt: 0,
                        y1: 0,
                        y2: 0,
                        x1: 0 as *const byte_hack as *mut byte_hack,
                        x2: 0 as *const byte_hack as *mut byte_hack,
                        old: 0 as *const term_win as *mut term_win,
                        scr: 0 as *const term_win as *mut term_win,
                        tmp: 0 as *const term_win as *mut term_win,
                        mem: 0 as *const term_win as *mut term_win,
                        init_hook: None,
                        nuke_hook: None,
                        user_hook: None,
                        xtra_hook: None,
                        curs_hook: None,
                        wipe_hook: None,
                        text_hook: None,
                        resize_hook: None,
                        pict_hook: None,},
               win: 0 as *const WINDOW as *mut WINDOW,}; 4];
/*
 * Hack -- Number of initialized "term" structures
 */
static mut active: libc::c_int = 0 as libc::c_int;
/*
 * Software flag -- we are allowed to use color
 */
static mut can_use_color: libc::c_int = 0 as libc::c_int;
/*
 * Software flag -- we are allowed to change the colors
 */
static mut can_fix_color: libc::c_int = 0 as libc::c_int;
/*
 * Simple Angband to Curses color conversion table
 */
static mut colortable: [libc::c_int; 16] = [0; 16];
/*
 * Place the "keymap" into its "normal" state
 */
unsafe extern "C" fn keymap_norm() {
    /* restore the saved values of the special chars */
    tcsetattr(0 as libc::c_int, 2 as libc::c_int, &mut norm_termios);
}
/*
 * Place the "keymap" into the "game" state
 */
unsafe extern "C" fn keymap_game() {
    /* restore the saved values of the special chars */
    tcsetattr(0 as libc::c_int, 2 as libc::c_int, &mut game_termios);
}
/*
 * Save the normal keymap
 */
unsafe extern "C" fn keymap_norm_prepare() {
    /* Get the normal keymap */
    tcgetattr(0 as libc::c_int, &mut norm_termios);
}
/*
 * Save the keymaps (normal and game)
 */
unsafe extern "C" fn keymap_game_prepare() {
    /* Acquire the current mapping */
    tcgetattr(0 as libc::c_int, &mut game_termios);
    /* Force "Ctrl-C" to interupt */
    game_termios.c_cc[0 as libc::c_int as usize] =
        3 as libc::c_int as libc::c_char as cc_t;
    /* Force "Ctrl-Z" to suspend */
    game_termios.c_cc[10 as libc::c_int as usize] =
        26 as libc::c_int as libc::c_char as cc_t;
    /* Hack -- Leave "VSTART/VSTOP" alone */
    /* Disable the standard control characters */
    game_termios.c_cc[1 as libc::c_int as usize] =
        -(1 as libc::c_int) as libc::c_char as cc_t;
    game_termios.c_cc[2 as libc::c_int as usize] =
        -(1 as libc::c_int) as libc::c_char as cc_t;
    game_termios.c_cc[3 as libc::c_int as usize] =
        -(1 as libc::c_int) as libc::c_char as cc_t;
    game_termios.c_cc[4 as libc::c_int as usize] =
        -(1 as libc::c_int) as libc::c_char as cc_t;
    game_termios.c_cc[11 as libc::c_int as usize] =
        -(1 as libc::c_int) as libc::c_char as cc_t;
    /* Normally, block until a character is read */
    game_termios.c_cc[6 as libc::c_int as usize] = 1 as libc::c_int as cc_t;
    game_termios.c_cc[5 as libc::c_int as usize] = 0 as libc::c_int as cc_t;
}
/*
 * Suspend/Resume
 */
unsafe extern "C" fn Term_xtra_gcu_alive(mut v: libc::c_int) -> errr {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    /* Suspend */
    if v == 0 {
        /* Go to normal keymap mode */
        keymap_norm();
        /* Restore modes */
        nocbreak();
        echo();
        nl();
        /* Hack -- make sure the cursor is visible */
        Term_xtra(4 as libc::c_int, 1 as libc::c_int);
        /* Flush the curses buffer */
        wrefresh(stdscr);
        /* Get current cursor position */
        y =
            (if !(curscr as *const libc::c_void).is_null() {
                 (*curscr)._cury as libc::c_int
             } else { -(1 as libc::c_int) });
        x =
            (if !(curscr as *const libc::c_void).is_null() {
                 (*curscr)._curx as libc::c_int
             } else { -(1 as libc::c_int) });
        /* Move the cursor to bottom right corner */
        mvcur(y, x, LINES - 1 as libc::c_int, 0 as libc::c_int);
        /* Exit curses */
        endwin();
        /* Flush the output */
        fflush(stdout);
    } else {
        /* Resume */
        /* Refresh */
		/* (void)touchwin(curscr); */
		/* (void)wrefresh(curscr); */
        /* Restore the settings */
        cbreak();
        noecho();
        nonl();
        keymap_game();
    }
    /* Go to angband keymap mode */
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Init the "curses" system
 */
unsafe extern "C" fn Term_init_gcu(mut t: *mut term) {
    let mut td: *mut term_data = (*t).data as *mut term_data;
    /* Count init's, handle first */
    let fresh0 = active;
    active = active + 1;
    if fresh0 != 0 as libc::c_int { return }
    /* Erase the window */
    wclear((*td).win);
    /* Reset the cursor */
    wmove((*td).win, 0 as libc::c_int, 0 as libc::c_int);
    /* Flush changes */
    wrefresh((*td).win);
    /* Game keymap */
    keymap_game();
}
/*
 * Nuke the "curses" system
 */
unsafe extern "C" fn Term_nuke_gcu(mut t: *mut term) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut td: *mut term_data = (*t).data as *mut term_data;
    /* Delete this window */
    delwin((*td).win);
    /* Count nuke's, handle last */
    active -= 1;
    if active != 0 as libc::c_int { return }
    /* Hack -- make sure the cursor is visible */
    Term_xtra(4 as libc::c_int, 1 as libc::c_int);
    /* Reset colors to defaults */
    start_color();
    /* Get current cursor position */
    y =
        (if !(curscr as *const libc::c_void).is_null() {
             (*curscr)._cury as libc::c_int
         } else { -(1 as libc::c_int) });
    x =
        (if !(curscr as *const libc::c_void).is_null() {
             (*curscr)._curx as libc::c_int
         } else { -(1 as libc::c_int) });
    /* Move the cursor to bottom right corner */
    mvcur(y, x, LINES - 1 as libc::c_int, 0 as libc::c_int);
    /* Flush the curses buffer */
    wrefresh(stdscr);
    /* Exit curses */
    endwin();
    /* Flush the output */
    fflush(stdout);
    /* Normal keymap */
    keymap_norm();
}
/* USE_GETCH */
/*
* Process events (with optional wait)
*/
unsafe extern "C" fn Term_xtra_gcu_event(mut v: libc::c_int) -> errr {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut buf: [libc::c_char; 2] = [0; 2];
    /* Wait */
    if v != 0 {
        /* Wait for one byte */
        i =
            read(0 as libc::c_int, buf.as_mut_ptr() as *mut libc::c_void,
                 1 as libc::c_int as size_t) as libc::c_int;
        /* Hack -- Handle bizarre "errors" */
        if i <= 0 as libc::c_int && *__errno_location() != 4 as libc::c_int {
            abort();
        }
    } else {
        /* Do not wait */
        /* Get the current flags for stdin */
        k = fcntl(0 as libc::c_int, 3 as libc::c_int, 0 as libc::c_int);
        if k < 0 as libc::c_int { return 1 as libc::c_int }
        if fcntl(0 as libc::c_int, 4 as libc::c_int,
                 k | 0o4000 as libc::c_int) < 0 as libc::c_int {
            return 1 as libc::c_int
        }
        i =
            read(0 as libc::c_int, buf.as_mut_ptr() as *mut libc::c_void,
                 1 as libc::c_int as size_t) as libc::c_int;
        if fcntl(0 as libc::c_int, 4 as libc::c_int, k) != 0 {
            return 1 as libc::c_int
        }
    }
    /* Oops */
    /* Tell stdin not to block */
    /* Read one byte, if possible */
    /* Replace the flags for stdin */
    /* Ignore "invalid" keys */
    if i != 1 as libc::c_int || buf[0 as libc::c_int as usize] == 0 {
        return 1 as libc::c_int
    }
    /* Enqueue the keypress */
    Term_keypress(buf[0 as libc::c_int as usize] as libc::c_int);
    /* Success */
    return 0 as libc::c_int;
}
/* USE_GETCH */
/*
 * React to changes
 */
unsafe extern "C" fn Term_xtra_gcu_react() -> errr {
    let mut i: libc::c_int = 0;
    /* Cannot handle color redefinition */
    if can_fix_color == 0 { return 0 as libc::c_int }
    /* Set the colors */
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        /* Set one color (note scaling) */
        init_color(i as libc::c_short,
                   (angband_color_table[i as usize][1 as libc::c_int as usize]
                        as libc::c_int * 1000 as libc::c_int /
                        255 as libc::c_int) as libc::c_short,
                   (angband_color_table[i as usize][2 as libc::c_int as usize]
                        as libc::c_int * 1000 as libc::c_int /
                        255 as libc::c_int) as libc::c_short,
                   (angband_color_table[i as usize][3 as libc::c_int as usize]
                        as libc::c_int * 1000 as libc::c_int /
                        255 as libc::c_int) as libc::c_short);
        i += 1
    }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Handle a "special request"
 */
unsafe extern "C" fn Term_xtra_gcu(mut n: libc::c_int, mut v: libc::c_int)
 -> errr {
    let mut td: *mut term_data = (*Term).data as *mut term_data;
    's_188:
        {
            /* Analyze the request */
            match n {
                3 => {
                    /* Clear screen */
                    wtouchln((*td).win, 0 as libc::c_int,
                             if !((*td).win as *const libc::c_void).is_null()
                                {
                                 ((*(*td).win)._maxy as libc::c_int) +
                                     1 as libc::c_int
                             } else { -(1 as libc::c_int) },
                             1 as libc::c_int);
                    wclear((*td).win);
                    return 0 as libc::c_int
                }
                7 => {
                    /* Make a noise */
                    write(1 as libc::c_int,
                          b"\x07\x00" as *const u8 as *const libc::c_char as
                              *const libc::c_void,
                          1 as libc::c_int as size_t);
                    return 0 as libc::c_int
                }
                6 => {
                    /* Flush the Curses buffer */
                    wrefresh((*td).win);
                    return 0 as libc::c_int
                }
                11 => {
                    /* Suspend/Resume curses */
                    return Term_xtra_gcu_alive(v)
                }
                1 => {
                    /* Process events */
                    return Term_xtra_gcu_event(v)
                }
                2 => {
                    /* Flush events */
                    while Term_xtra_gcu_event(0 as libc::c_int) == 0 { }
                    return 0 as libc::c_int
                }
                13 => {
                    /* Delay */
                    usleep((1000 as libc::c_int * v) as __useconds_t);
                    return 0 as libc::c_int
                }
                14 => {
                    /* Get Delay of some milliseconds */
                    let mut ret: libc::c_int = 0;
                    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
                    ret = gettimeofday(&mut tv, 0 as *mut libc::c_void);
                    Term_xtra_long =
                        tv.tv_sec * 1000 as libc::c_int as libc::c_long +
                            tv.tv_usec / 1000 as libc::c_int as libc::c_long;
                    return ret
                }
                15 => {
                    /* Subdirectory scan */
                    let mut directory: *mut DIR = 0 as *mut DIR;
                    let mut entry: *mut dirent = 0 as *mut dirent;
                    scansubdir_max = 0 as libc::c_int;
                    directory = opendir(scansubdir_dir.as_mut_ptr());
                    if directory.is_null() { return 1 as libc::c_int }
                    loop  {
                        entry = readdir(directory);
                        if entry.is_null() { break ; }
                        let mut file: [libc::c_char; 4353] = [0; 4353];
                        let mut filedata: stat =
                            stat{st_dev: 0,
                                 st_ino: 0,
                                 st_nlink: 0,
                                 st_mode: 0,
                                 st_uid: 0,
                                 st_gid: 0,
                                 __pad0: 0,
                                 st_rdev: 0,
                                 st_size: 0,
                                 st_blksize: 0,
                                 st_blocks: 0,
                                 st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
                                 st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
                                 st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
                                 __glibc_reserved: [0; 3],};
                        file[(4096 as libc::c_int + 255 as libc::c_int) as
                                 usize] = 0 as libc::c_int as libc::c_char;
                        strncpy(file.as_mut_ptr(),
                                scansubdir_dir.as_mut_ptr(),
                                4096 as libc::c_int as libc::c_ulong);
                        strncat(file.as_mut_ptr(),
                                b"/\x00" as *const u8 as *const libc::c_char,
                                2 as libc::c_int as libc::c_ulong);
                        strncat(file.as_mut_ptr(),
                                (*entry).d_name.as_mut_ptr(),
                                255 as libc::c_int as libc::c_ulong);
                        if stat(file.as_mut_ptr(), &mut filedata) == 0 &&
                               filedata.st_mode &
                                   0o170000 as libc::c_int as libc::c_uint ==
                                   0o40000 as libc::c_int as libc::c_uint {
                            string_free(scansubdir_result[scansubdir_max as
                                                              usize]);
                            scansubdir_result[scansubdir_max as usize] =
                                string_make((*entry).d_name.as_mut_ptr() as
                                                cptr);
                            scansubdir_max += 1
                        }
                    }
                }
                10 => { }
                _ => { break 's_188 ; }
            }
            /* React to events */
            Term_xtra_gcu_react();
            return 0 as libc::c_int
        }
    /* Unknown */
    return 1 as libc::c_int;
}
/*
 * Actually MOVE the hardware cursor
 */
unsafe extern "C" fn Term_curs_gcu(mut x: libc::c_int, mut y: libc::c_int)
 -> errr {
    let mut td: *mut term_data = (*Term).data as *mut term_data;
    /* Literally move the cursor */
    wmove((*td).win, y, x);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Erase a grid of space
 * Hack -- try to be "semi-efficient".
 */
unsafe extern "C" fn Term_wipe_gcu(mut x: libc::c_int, mut y: libc::c_int,
                                   mut n: libc::c_int) -> errr {
    let mut td: *mut term_data = (*Term).data as *mut term_data;
    /* Place cursor */
    wmove((*td).win, y, x);
    /* Clear to end of line */
    if x + n >= (*td).t.wid as libc::c_int {
        wclrtoeol((*td).win);
    } else {
        /* Clear some characters */
        loop  {
            let fresh1 = n;
            n = n - 1;
            if !(fresh1 > 0 as libc::c_int) { break ; }
            waddch((*td).win, ' ' as i32 as chtype);
        }
    }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Place some text on the screen using an attribute
 */
unsafe extern "C" fn Term_text_gcu(mut x: libc::c_int, mut y: libc::c_int,
                                   mut n: libc::c_int, mut a: byte_hack,
                                   mut s: cptr) -> errr {
    let mut td: *mut term_data = (*Term).data as *mut term_data;
    let mut i: libc::c_int = 0;
    let mut pic: libc::c_int = 0;
    /* Set the color */
    if can_use_color != 0 {
        wattrset((*td).win,
                 colortable[(a as libc::c_int & 0xf as libc::c_int) as
                                usize]);
    }
    /* Move the cursor */
    wmove((*td).win, y, x);
    /* Draw each character */
    i = 0 as libc::c_int;
    while i < n {
        /* Special character */
        if use_graphics as libc::c_int != 0 &&
               *s.offset(i as isize) as libc::c_int & 0x80 as libc::c_int != 0
           {
            /* Determine picture to use */
            match *s.offset(i as isize) as libc::c_int & 0x7f as libc::c_int {
                35 => {
                    /* Wall */
                    pic =
                        *acs_map.as_mut_ptr().offset('a' as i32 as
                                                         libc::c_uchar as
                                                         isize) as libc::c_int
                }
                37 => {
                    /* ACS_CKBOARD */
                    /* Mineral vein */
                    pic =
                        *acs_map.as_mut_ptr().offset('h' as i32 as
                                                         libc::c_uchar as
                                                         isize) as libc::c_int
                }
                _ => {
                    /* ACS_BOARD */
                    /* XXX */
                    pic = '?' as i32
                }
            }
            /* Draw the picture */
            waddch((*td).win, pic as chtype);
        } else {
            /* Draw a normal character */
            waddch((*td).win, *s.offset(i as isize) as byte_hack as chtype);
        }
        /* Next character */
        i += 1
    }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Create a window for the given "term_data" argument.
 *
 * Assumes legal arguments.
 */
unsafe extern "C" fn term_data_init_gcu(mut td: *mut term_data,
                                        mut rows: libc::c_int,
                                        mut cols: libc::c_int,
                                        mut y: libc::c_int,
                                        mut x: libc::c_int) -> errr {
    let mut t: *mut term = &mut (*td).t;
    /* Create new window */
    (*td).win = newwin(rows, cols, y, x);
    /* Check for failure */
    if (*td).win.is_null() {
        /* Error */
        quit(b"Failed to setup curses window.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialize the term */
    term_init(t, cols, rows, 256 as libc::c_int);
    /* Avoid bottom right corner */
    (*t).icky_corner = 1 as libc::c_int as bool_;
    /* Erase with "white space" */
    (*t).attr_blank = 1 as libc::c_int as byte_hack;
    (*t).char_blank = ' ' as i32 as libc::c_char;
    /* Set some hooks */
    (*t).init_hook =
        Some(Term_init_gcu as unsafe extern "C" fn(_: *mut term) -> ());
    (*t).nuke_hook =
        Some(Term_nuke_gcu as unsafe extern "C" fn(_: *mut term) -> ());
    /* Set some more hooks */
    (*t).text_hook =
        Some(Term_text_gcu as
                 unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                      _: libc::c_int, _: byte_hack, _: cptr)
                     -> errr);
    (*t).wipe_hook =
        Some(Term_wipe_gcu as
                 unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                      _: libc::c_int) -> errr);
    (*t).curs_hook =
        Some(Term_curs_gcu as
                 unsafe extern "C" fn(_: libc::c_int, _: libc::c_int)
                     -> errr);
    (*t).xtra_hook =
        Some(Term_xtra_gcu as
                 unsafe extern "C" fn(_: libc::c_int, _: libc::c_int)
                     -> errr);
    /* Save the data */
    (*t).data = td as vptr;
    /* Activate it */
    Term_activate(t);
    /* Success */
    return 0 as libc::c_int;
}
unsafe extern "C" fn hook_quit(mut str: cptr) {
    /* Unused */
    /* Exit curses */
    endwin();
}
/*
 * Prepare "curses" for use by the file "z-term.c"
 *
 * Installs the "hook" functions defined above, and then activates
 * the main screen "term", which clears the screen and such things.
 *
 * Someone should really check the semantics of "initscr()"
 */
#[no_mangle]
pub unsafe extern "C" fn init_gcu(mut argc: libc::c_int,
                                  mut argv: *mut *mut libc::c_char) -> errr {
    let mut i: libc::c_int = 0;
    let mut num_term: libc::c_int = 4 as libc::c_int;
    let mut next_win: libc::c_int = 0 as libc::c_int;
    let mut use_big_screen: bool_ = 0 as libc::c_int as bool_;
    /* Parse args */
    i = 1 as libc::c_int;
    while i < argc {
        if prefix(*argv.offset(i as isize) as cptr,
                  b"-b\x00" as *const u8 as *const libc::c_char) != 0 {
            use_big_screen = 1 as libc::c_int as bool_
        } else {
            plog_fmt(b"Ignoring option: %s\x00" as *const u8 as
                         *const libc::c_char, *argv.offset(i as isize));
        }
        i += 1
    }
    /* Extract the normal keymap */
    keymap_norm_prepare();
    /* Initialize for other systems */
    if initscr() == -(1 as libc::c_int) as *mut WINDOW {
        return -(1 as libc::c_int)
    }
    /* Activate hooks */
    quit_aux = Some(hook_quit as unsafe extern "C" fn(_: cptr) -> ());
    core_aux = Some(hook_quit as unsafe extern "C" fn(_: cptr) -> ());
    /* Require standard size screen */
    if LINES < 24 as libc::c_int || COLS < 80 as libc::c_int {
        quit(b"Angband needs at least an 80x24 \'curses\' screen\x00" as
                 *const u8 as *const libc::c_char);
    }
    /* Set graphics flag */
    use_graphics = arg_graphics;
    /* ** Init the Color-pairs and set up a translation table ***/
    /* Do we have color, and enough color, available? */
    can_use_color =
        (start_color() != -(1 as libc::c_int) &&
             has_colors() as libc::c_int != 0 && COLORS >= 8 as libc::c_int &&
             COLOR_PAIRS >= 8 as libc::c_int) as libc::c_int;
    /* Can we change colors? */
    can_fix_color =
        (can_use_color != 0 && can_change_color() as libc::c_int != 0 &&
             COLORS >= 16 as libc::c_int && COLOR_PAIRS > 8 as libc::c_int) as
            libc::c_int;
    /* Attempt to use customized colors */
    if can_fix_color != 0 {
        /* Prepare the color pairs */
        i = 1 as libc::c_int;
        while i <= 8 as libc::c_int {
            /* Reset the color */
            if init_pair(i as libc::c_short,
                         (i - 1 as libc::c_int) as libc::c_short,
                         0 as libc::c_int as libc::c_short) ==
                   -(1 as libc::c_int) {
                quit(b"Color pair init failed\x00" as *const u8 as
                         *const libc::c_char);
            }
            /* Set up the colormap */
            colortable[(i - 1 as libc::c_int) as usize] =
                ((i as chtype) << 0 as libc::c_int + 8 as libc::c_int &
                     ((1 as libc::c_uint) <<
                          8 as libc::c_int).wrapping_sub(1 as libc::c_uint) <<
                         0 as libc::c_int + 8 as libc::c_int |
                     (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint)) as
                    libc::c_int;
            colortable[(i + 7 as libc::c_int) as usize] =
                ((i as chtype) << 0 as libc::c_int + 8 as libc::c_int &
                     ((1 as libc::c_uint) <<
                          8 as libc::c_int).wrapping_sub(1 as libc::c_uint) <<
                         0 as libc::c_int + 8 as libc::c_int |
                     (1 as libc::c_uint) <<
                         13 as libc::c_int + 8 as libc::c_int) as libc::c_int;
            i += 1
        }
        /* Take account of "gamma correction" XXX XXX XXX */
        /* Prepare the "Angband Colors" */
        Term_xtra_gcu_react();
    } else if can_use_color != 0 {
        /* Attempt to use colors */
        /* Color-pair 0 is *always* WHITE on BLACK */
        /* Prepare the color pairs */
        init_pair(1 as libc::c_int as libc::c_short,
                  1 as libc::c_int as libc::c_short,
                  0 as libc::c_int as libc::c_short);
        init_pair(2 as libc::c_int as libc::c_short,
                  2 as libc::c_int as libc::c_short,
                  0 as libc::c_int as libc::c_short);
        init_pair(3 as libc::c_int as libc::c_short,
                  3 as libc::c_int as libc::c_short,
                  0 as libc::c_int as libc::c_short);
        init_pair(4 as libc::c_int as libc::c_short,
                  4 as libc::c_int as libc::c_short,
                  0 as libc::c_int as libc::c_short);
        init_pair(5 as libc::c_int as libc::c_short,
                  5 as libc::c_int as libc::c_short,
                  0 as libc::c_int as libc::c_short);
        init_pair(6 as libc::c_int as libc::c_short,
                  6 as libc::c_int as libc::c_short,
                  0 as libc::c_int as libc::c_short);
        init_pair(7 as libc::c_int as libc::c_short,
                  0 as libc::c_int as libc::c_short,
                  0 as libc::c_int as libc::c_short);
        /* Light Umber XXX */
        colortable[0 as libc::c_int as usize] =
            ((7 as libc::c_int as chtype) <<
                 0 as libc::c_int + 8 as libc::c_int &
                 ((1 as libc::c_uint) <<
                      8 as libc::c_int).wrapping_sub(1 as libc::c_uint) <<
                     0 as libc::c_int + 8 as libc::c_int |
                 (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint)) as
                libc::c_int;
        colortable[1 as libc::c_int as usize] =
            ((0 as libc::c_int as chtype) <<
                 0 as libc::c_int + 8 as libc::c_int &
                 ((1 as libc::c_uint) <<
                      8 as libc::c_int).wrapping_sub(1 as libc::c_uint) <<
                     0 as libc::c_int + 8 as libc::c_int |
                 (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint)) as
                libc::c_int;
        colortable[2 as libc::c_int as usize] =
            ((6 as libc::c_int as chtype) <<
                 0 as libc::c_int + 8 as libc::c_int &
                 ((1 as libc::c_uint) <<
                      8 as libc::c_int).wrapping_sub(1 as libc::c_uint) <<
                     0 as libc::c_int + 8 as libc::c_int |
                 (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint)) as
                libc::c_int;
        colortable[3 as libc::c_int as usize] =
            ((1 as libc::c_int as chtype) <<
                 0 as libc::c_int + 8 as libc::c_int &
                 ((1 as libc::c_uint) <<
                      8 as libc::c_int).wrapping_sub(1 as libc::c_uint) <<
                     0 as libc::c_int + 8 as libc::c_int |
                 (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int)
                as libc::c_int;
        colortable[4 as libc::c_int as usize] =
            ((1 as libc::c_int as chtype) <<
                 0 as libc::c_int + 8 as libc::c_int &
                 ((1 as libc::c_uint) <<
                      8 as libc::c_int).wrapping_sub(1 as libc::c_uint) <<
                     0 as libc::c_int + 8 as libc::c_int |
                 (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint)) as
                libc::c_int;
        colortable[5 as libc::c_int as usize] =
            ((2 as libc::c_int as chtype) <<
                 0 as libc::c_int + 8 as libc::c_int &
                 ((1 as libc::c_uint) <<
                      8 as libc::c_int).wrapping_sub(1 as libc::c_uint) <<
                     0 as libc::c_int + 8 as libc::c_int |
                 (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint)) as
                libc::c_int;
        colortable[6 as libc::c_int as usize] =
            ((4 as libc::c_int as chtype) <<
                 0 as libc::c_int + 8 as libc::c_int &
                 ((1 as libc::c_uint) <<
                      8 as libc::c_int).wrapping_sub(1 as libc::c_uint) <<
                     0 as libc::c_int + 8 as libc::c_int |
                 (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint)) as
                libc::c_int;
        colortable[7 as libc::c_int as usize] =
            ((3 as libc::c_int as chtype) <<
                 0 as libc::c_int + 8 as libc::c_int &
                 ((1 as libc::c_uint) <<
                      8 as libc::c_int).wrapping_sub(1 as libc::c_uint) <<
                     0 as libc::c_int + 8 as libc::c_int |
                 (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint)) as
                libc::c_int;
        colortable[8 as libc::c_int as usize] =
            ((7 as libc::c_int as chtype) <<
                 0 as libc::c_int + 8 as libc::c_int &
                 ((1 as libc::c_uint) <<
                      8 as libc::c_int).wrapping_sub(1 as libc::c_uint) <<
                     0 as libc::c_int + 8 as libc::c_int |
                 (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int)
                as libc::c_int;
        colortable[9 as libc::c_int as usize] =
            ((6 as libc::c_int as chtype) <<
                 0 as libc::c_int + 8 as libc::c_int &
                 ((1 as libc::c_uint) <<
                      8 as libc::c_int).wrapping_sub(1 as libc::c_uint) <<
                     0 as libc::c_int + 8 as libc::c_int |
                 (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int)
                as libc::c_int;
        colortable[10 as libc::c_int as usize] =
            ((5 as libc::c_int as chtype) <<
                 0 as libc::c_int + 8 as libc::c_int &
                 ((1 as libc::c_uint) <<
                      8 as libc::c_int).wrapping_sub(1 as libc::c_uint) <<
                     0 as libc::c_int + 8 as libc::c_int |
                 (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint)) as
                libc::c_int;
        colortable[11 as libc::c_int as usize] =
            ((3 as libc::c_int as chtype) <<
                 0 as libc::c_int + 8 as libc::c_int &
                 ((1 as libc::c_uint) <<
                      8 as libc::c_int).wrapping_sub(1 as libc::c_uint) <<
                     0 as libc::c_int + 8 as libc::c_int |
                 (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int)
                as libc::c_int;
        colortable[12 as libc::c_int as usize] =
            ((5 as libc::c_int as chtype) <<
                 0 as libc::c_int + 8 as libc::c_int &
                 ((1 as libc::c_uint) <<
                      8 as libc::c_int).wrapping_sub(1 as libc::c_uint) <<
                     0 as libc::c_int + 8 as libc::c_int |
                 (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int)
                as libc::c_int;
        colortable[13 as libc::c_int as usize] =
            ((2 as libc::c_int as chtype) <<
                 0 as libc::c_int + 8 as libc::c_int &
                 ((1 as libc::c_uint) <<
                      8 as libc::c_int).wrapping_sub(1 as libc::c_uint) <<
                     0 as libc::c_int + 8 as libc::c_int |
                 (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int)
                as libc::c_int;
        colortable[14 as libc::c_int as usize] =
            ((4 as libc::c_int as chtype) <<
                 0 as libc::c_int + 8 as libc::c_int &
                 ((1 as libc::c_uint) <<
                      8 as libc::c_int).wrapping_sub(1 as libc::c_uint) <<
                     0 as libc::c_int + 8 as libc::c_int |
                 (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int)
                as libc::c_int;
        colortable[15 as libc::c_int as usize] =
            ((3 as libc::c_int as chtype) <<
                 0 as libc::c_int + 8 as libc::c_int &
                 ((1 as libc::c_uint) <<
                      8 as libc::c_int).wrapping_sub(1 as libc::c_uint) <<
                     0 as libc::c_int + 8 as libc::c_int |
                 (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint)) as
                libc::c_int
    }
    /* Prepare the "Angband Colors" -- Bright white is too bright */
    /* Black */
    /* White */
    /* Grey XXX */
    /* Orange XXX */
    /* Red */
    /* Green */
    /* Blue */
    /* Umber */
    /* Dark-grey XXX */
    /* Light-grey XXX */
    /* Purple */
    /* Yellow */
    /* Light Red XXX */
    /* Light Green */
    /* Light Blue */
    /* ** Low level preparation ***/
    /* Prepare */
    cbreak();
    noecho();
    nonl();
    /* Extract the game keymap */
    keymap_game_prepare();
    /* ** Now prepare the term(s) ***/
    /* Big screen -- one big term */
    if use_big_screen != 0 {
        /* Create a term */
        term_data_init_gcu(&mut *data.as_mut_ptr().offset(0 as libc::c_int as
                                                              isize), LINES,
                           COLS, 0 as libc::c_int, 0 as libc::c_int);
        /* Remember the term */
        angband_term[0 as libc::c_int as usize] =
            &mut (*data.as_mut_ptr().offset(0 as libc::c_int as isize)).t
    } else {
        /* No big screen -- create as many term windows as possible */
        /* Create several terms */
        i = 0 as libc::c_int;
        while i < num_term {
            let mut rows: libc::c_int = 0;
            let mut cols: libc::c_int = 0;
            let mut y: libc::c_int = 0;
            let mut x: libc::c_int = 0;
            /* Decide on size and position */
            match i {
                0 => {
                    /* Upper left */
                    rows = 24 as libc::c_int;
                    cols = 80 as libc::c_int;
                    x = 0 as libc::c_int;
                    y = x
                }
                1 => {
                    /* Lower left */
                    rows = LINES - 25 as libc::c_int;
                    cols = 80 as libc::c_int;
                    y = 25 as libc::c_int;
                    x = 0 as libc::c_int
                }
                2 => {
                    /* Upper right */
                    rows = 24 as libc::c_int;
                    cols = COLS - 81 as libc::c_int;
                    y = 0 as libc::c_int;
                    x = 81 as libc::c_int
                }
                3 => {
                    /* Lower right */
                    rows = LINES - 25 as libc::c_int;
                    cols = COLS - 81 as libc::c_int;
                    y = 25 as libc::c_int;
                    x = 81 as libc::c_int
                }
                _ => {
                    /* XXX */
                    x = 0 as libc::c_int;
                    y = x;
                    cols = y;
                    rows = cols
                }
            }
            /* Skip non-existant windows */
            if !(rows <= 0 as libc::c_int || cols <= 0 as libc::c_int) {
                /* Create a term */
                term_data_init_gcu(&mut *data.as_mut_ptr().offset(next_win as
                                                                      isize),
                                   rows, cols, y, x);
                /* Remember the term */
                angband_term[next_win as usize] =
                    &mut (*data.as_mut_ptr().offset(next_win as isize)).t;
                /* One more window */
                next_win += 1
            }
            i += 1
        }
    }
    /* Activate the "Angband" window screen */
    Term_activate(&mut (*data.as_mut_ptr().offset(0 as libc::c_int as
                                                      isize)).t);
    /* Remember the active screen */
    angband_term[0 as libc::c_int as usize] =
        &mut (*data.as_mut_ptr().offset(0 as libc::c_int as isize)).t;
    /* Success */
    return 0 as libc::c_int;
}
/* USE_GCU */
