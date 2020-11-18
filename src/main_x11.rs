use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _XGC;
    pub type _XDisplay;
    pub type _XPrivate;
    pub type _XrmHashBucketRec;
    pub type __dirstream;
    #[no_mangle]
    static mut arg_graphics: bool_;
    #[no_mangle]
    static mut arg_bigtile: bool_;
    #[no_mangle]
    static mut character_generated: bool_;
    #[no_mangle]
    static mut use_graphics: bool_;
    #[no_mangle]
    static mut use_bigtile: bool_;
    #[no_mangle]
    static mut angband_term: [*mut term; 8];
    #[no_mangle]
    static mut angband_term_name: [[libc::c_char; 80]; 8];
    #[no_mangle]
    static mut angband_color_table: [[byte_hack; 4]; 256];
    #[no_mangle]
    static mut ANGBAND_GRAF: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_XTRA: cptr;
    #[no_mangle]
    fn save_dungeon();
    #[no_mangle]
    fn save_player() -> bool_;
    #[no_mangle]
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strncat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn feof(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn getc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn term_init(t: *mut term, w: libc::c_int, h: libc::c_int, k: libc::c_int)
     -> errr;
    #[no_mangle]
    fn Term_activate(t: *mut term) -> errr;
    #[no_mangle]
    fn Term_resize(w: libc::c_int, h: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_keypress(k: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_what(x: libc::c_int, y: libc::c_int, a: *mut byte_hack,
                 c: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn Term_redraw_section(x1: libc::c_int, y1: libc::c_int, x2: libc::c_int,
                           y2: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_redraw() -> errr;
    #[no_mangle]
    static mut scansubdir_result: [cptr; 255];
    #[no_mangle]
    static mut scansubdir_max: libc::c_int;
    #[no_mangle]
    static mut scansubdir_dir: [libc::c_char; 1024];
    #[no_mangle]
    static mut Term_xtra_long: libc::c_long;
    #[no_mangle]
    static mut Term: *mut term;
    #[no_mangle]
    fn quit_fmt(fmt: cptr, _: ...);
    #[no_mangle]
    fn plog_fmt(fmt: cptr, _: ...);
    #[no_mangle]
    fn string_free(str: cptr) -> errr;
    #[no_mangle]
    fn string_make(str: cptr) -> cptr;
    #[no_mangle]
    fn ralloc(len: huge_hack) -> vptr;
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn quit(str: cptr);
    #[no_mangle]
    fn prefix(s: cptr, t: cptr) -> bool_;
    #[no_mangle]
    fn streq(s: cptr, t: cptr) -> bool_;
    #[no_mangle]
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn path_build(buf: *mut libc::c_char, max: libc::c_int, path: cptr,
                  file: cptr) -> errr;
    #[no_mangle]
    fn fd_open(file: cptr, flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn fd_close(fd: libc::c_int) -> errr;
    #[no_mangle]
    fn bell();
    #[no_mangle]
    fn macro_add(pat: cptr, act: cptr) -> errr;
    #[no_mangle]
    fn macro_find_exact(pat: cptr) -> sint;
    #[no_mangle]
    fn XLoadQueryFont(_: *mut Display, _: *const libc::c_char)
     -> *mut XFontStruct;
    #[no_mangle]
    fn XCreateImage(_: *mut Display, _: *mut Visual, _: libc::c_uint,
                    _: libc::c_int, _: libc::c_int, _: *mut libc::c_char,
                    _: libc::c_uint, _: libc::c_uint, _: libc::c_int,
                    _: libc::c_int) -> *mut XImage;
    #[no_mangle]
    fn XOpenDisplay(_: *const libc::c_char) -> *mut Display;
    #[no_mangle]
    fn XCreateGC(_: *mut Display, _: Drawable, _: libc::c_ulong,
                 _: *mut XGCValues) -> GC;
    #[no_mangle]
    fn XCreateSimpleWindow(_: *mut Display, _: Window, _: libc::c_int,
                           _: libc::c_int, _: libc::c_uint, _: libc::c_uint,
                           _: libc::c_uint, _: libc::c_ulong,
                           _: libc::c_ulong) -> Window;
    #[no_mangle]
    fn XGetSelectionOwner(_: *mut Display, _: Atom) -> Window;
    #[no_mangle]
    fn XSetIOErrorHandler(_: XIOErrorHandler) -> XIOErrorHandler;
    #[no_mangle]
    fn XAllocColor(_: *mut Display, _: Colormap, _: *mut XColor)
     -> libc::c_int;
    #[no_mangle]
    fn XBell(_: *mut Display, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn XChangeProperty(_: *mut Display, _: Window, _: Atom, _: Atom,
                       _: libc::c_int, _: libc::c_int,
                       _: *const libc::c_uchar, _: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn XClearWindow(_: *mut Display, _: Window) -> libc::c_int;
    #[no_mangle]
    fn XDeleteProperty(_: *mut Display, _: Window, _: Atom) -> libc::c_int;
    #[no_mangle]
    fn XDrawImageString(_: *mut Display, _: Drawable, _: GC, _: libc::c_int,
                        _: libc::c_int, _: *const libc::c_char,
                        _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn XDrawRectangle(_: *mut Display, _: Drawable, _: GC, _: libc::c_int,
                      _: libc::c_int, _: libc::c_uint, _: libc::c_uint)
     -> libc::c_int;
    #[no_mangle]
    fn XFillRectangle(_: *mut Display, _: Drawable, _: GC, _: libc::c_int,
                      _: libc::c_int, _: libc::c_uint, _: libc::c_uint)
     -> libc::c_int;
    #[no_mangle]
    fn XFlush(_: *mut Display) -> libc::c_int;
    #[no_mangle]
    fn XFree(_: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn XFreeFont(_: *mut Display, _: *mut XFontStruct) -> libc::c_int;
    #[no_mangle]
    fn XGetGeometry(_: *mut Display, _: Drawable, _: *mut Window,
                    _: *mut libc::c_int, _: *mut libc::c_int,
                    _: *mut libc::c_uint, _: *mut libc::c_uint,
                    _: *mut libc::c_uint, _: *mut libc::c_uint)
     -> libc::c_int;
    #[no_mangle]
    fn XGetWindowProperty(_: *mut Display, _: Window, _: Atom,
                          _: libc::c_long, _: libc::c_long, _: libc::c_int,
                          _: Atom, _: *mut Atom, _: *mut libc::c_int,
                          _: *mut libc::c_ulong, _: *mut libc::c_ulong,
                          _: *mut *mut libc::c_uchar) -> libc::c_int;
    #[no_mangle]
    fn XGetWindowAttributes(_: *mut Display, _: Window,
                            _: *mut XWindowAttributes) -> libc::c_int;
    #[no_mangle]
    fn XMapWindow(_: *mut Display, _: Window) -> libc::c_int;
    #[no_mangle]
    fn XMoveWindow(_: *mut Display, _: Window, _: libc::c_int, _: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn XNextEvent(_: *mut Display, _: *mut XEvent) -> libc::c_int;
    #[no_mangle]
    fn XPending(_: *mut Display) -> libc::c_int;
    #[no_mangle]
    fn XPutImage(_: *mut Display, _: Drawable, _: GC, _: *mut XImage,
                 _: libc::c_int, _: libc::c_int, _: libc::c_int,
                 _: libc::c_int, _: libc::c_uint, _: libc::c_uint)
     -> libc::c_int;
    #[no_mangle]
    fn XRaiseWindow(_: *mut Display, _: Window) -> libc::c_int;
    #[no_mangle]
    fn XRefreshKeyboardMapping(_: *mut XMappingEvent) -> libc::c_int;
    #[no_mangle]
    fn XSelectInput(_: *mut Display, _: Window, _: libc::c_long)
     -> libc::c_int;
    #[no_mangle]
    fn XSendEvent(_: *mut Display, _: Window, _: libc::c_int, _: libc::c_long,
                  _: *mut XEvent) -> libc::c_int;
    #[no_mangle]
    fn XSetFont(_: *mut Display, _: GC, _: Font) -> libc::c_int;
    #[no_mangle]
    fn XSetForeground(_: *mut Display, _: GC, _: libc::c_ulong)
     -> libc::c_int;
    #[no_mangle]
    fn XSetSelectionOwner(_: *mut Display, _: Atom, _: Window, _: Time)
     -> libc::c_int;
    #[no_mangle]
    fn XSync(_: *mut Display, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn XAllocClassHint() -> *mut XClassHint;
    #[no_mangle]
    fn XAllocSizeHints() -> *mut XSizeHints;
    #[no_mangle]
    fn XLookupString(_: *mut XKeyEvent, _: *mut libc::c_char, _: libc::c_int,
                     _: *mut KeySym, _: *mut XComposeStatus) -> libc::c_int;
    #[no_mangle]
    fn XSetClassHint(_: *mut Display, _: Window, _: *mut XClassHint)
     -> libc::c_int;
    #[no_mangle]
    fn XSetWMName(_: *mut Display, _: Window, _: *mut XTextProperty);
    #[no_mangle]
    fn XSetWMNormalHints(_: *mut Display, _: Window, _: *mut XSizeHints);
    #[no_mangle]
    fn XStringListToTextProperty(_: *mut *mut libc::c_char, _: libc::c_int,
                                 _: *mut XTextProperty) -> libc::c_int;
    #[no_mangle]
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    #[no_mangle]
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type sint = libc::c_int;
pub type uint_hack = libc::c_uint;
pub type huge_hack = libc::c_ulong;
pub type s16b = libc::c_short;
pub type u16b = libc::c_ushort;
pub type u32b = libc::c_uint;
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
pub type XID = libc::c_ulong;
pub type Atom = libc::c_ulong;
pub type VisualID = libc::c_ulong;
pub type Time = libc::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type Pixmap = XID;
pub type Colormap = XID;
pub type KeySym = XID;
pub type XPointer = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: libc::c_int,
    pub next: *mut _XExtData,
    pub free_private: Option<unsafe extern "C" fn(_: *mut _XExtData)
                                 -> libc::c_int>,
    pub private_data: XPointer,
}
pub type XExtData = _XExtData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGCValues {
    pub function: libc::c_int,
    pub plane_mask: libc::c_ulong,
    pub foreground: libc::c_ulong,
    pub background: libc::c_ulong,
    pub line_width: libc::c_int,
    pub line_style: libc::c_int,
    pub cap_style: libc::c_int,
    pub join_style: libc::c_int,
    pub fill_style: libc::c_int,
    pub fill_rule: libc::c_int,
    pub arc_mode: libc::c_int,
    pub tile: Pixmap,
    pub stipple: Pixmap,
    pub ts_x_origin: libc::c_int,
    pub ts_y_origin: libc::c_int,
    pub font: Font,
    pub subwindow_mode: libc::c_int,
    pub graphics_exposures: libc::c_int,
    pub clip_x_origin: libc::c_int,
    pub clip_y_origin: libc::c_int,
    pub clip_mask: Pixmap,
    pub dash_offset: libc::c_int,
    pub dashes: libc::c_char,
}
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub bits_per_rgb: libc::c_int,
    pub map_entries: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: libc::c_int,
    pub nvisuals: libc::c_int,
    pub visuals: *mut Visual,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mwidth: libc::c_int,
    pub mheight: libc::c_int,
    pub ndepths: libc::c_int,
    pub depths: *mut Depth,
    pub root_depth: libc::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: libc::c_ulong,
    pub black_pixel: libc::c_ulong,
    pub max_maps: libc::c_int,
    pub min_maps: libc::c_int,
    pub backing_store: libc::c_int,
    pub save_unders: libc::c_int,
    pub root_input_mask: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub scanline_pad: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XWindowAttributes {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub depth: libc::c_int,
    pub visual: *mut Visual,
    pub root: Window,
    pub class: libc::c_int,
    pub bit_gravity: libc::c_int,
    pub win_gravity: libc::c_int,
    pub backing_store: libc::c_int,
    pub backing_planes: libc::c_ulong,
    pub backing_pixel: libc::c_ulong,
    pub save_under: libc::c_int,
    pub colormap: Colormap,
    pub map_installed: libc::c_int,
    pub map_state: libc::c_int,
    pub all_event_masks: libc::c_long,
    pub your_event_mask: libc::c_long,
    pub do_not_propagate_mask: libc::c_long,
    pub override_redirect: libc::c_int,
    pub screen: *mut Screen,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XImage {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub xoffset: libc::c_int,
    pub format: libc::c_int,
    pub data: *mut libc::c_char,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub depth: libc::c_int,
    pub bytes_per_line: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub obdata: XPointer,
    pub f: funcs,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct funcs {
    pub create_image: Option<unsafe extern "C" fn(_: *mut _XDisplay,
                                                  _: *mut Visual,
                                                  _: libc::c_uint,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: *mut libc::c_char,
                                                  _: libc::c_uint,
                                                  _: libc::c_uint,
                                                  _: libc::c_int,
                                                  _: libc::c_int)
                                 -> *mut _XImage>,
    pub destroy_image: Option<unsafe extern "C" fn(_: *mut _XImage)
                                  -> libc::c_int>,
    pub get_pixel: Option<unsafe extern "C" fn(_: *mut _XImage,
                                               _: libc::c_int, _: libc::c_int)
                              -> libc::c_ulong>,
    pub put_pixel: Option<unsafe extern "C" fn(_: *mut _XImage,
                                               _: libc::c_int, _: libc::c_int,
                                               _: libc::c_ulong)
                              -> libc::c_int>,
    pub sub_image: Option<unsafe extern "C" fn(_: *mut _XImage,
                                               _: libc::c_int, _: libc::c_int,
                                               _: libc::c_uint,
                                               _: libc::c_uint)
                              -> *mut _XImage>,
    pub add_pixel: Option<unsafe extern "C" fn(_: *mut _XImage,
                                               _: libc::c_long)
                              -> libc::c_int>,
}
pub type XImage = _XImage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColor {
    pub pixel: libc::c_ulong,
    pub red: libc::c_ushort,
    pub green: libc::c_ushort,
    pub blue: libc::c_ushort,
    pub flags: libc::c_char,
    pub pad: libc::c_char,
}
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: libc::c_int,
    pub private2: libc::c_int,
    pub proto_major_version: libc::c_int,
    pub proto_minor_version: libc::c_int,
    pub vendor: *mut libc::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: libc::c_int,
    pub resource_alloc: Option<unsafe extern "C" fn(_: *mut _XDisplay)
                                   -> XID>,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub nformats: libc::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: libc::c_int,
    pub release: libc::c_int,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: libc::c_int,
    pub last_request_read: libc::c_ulong,
    pub request: libc::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: libc::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option<unsafe extern "C" fn(_: *mut _XDisplay)
                              -> libc::c_int>,
    pub display_name: *mut libc::c_char,
    pub default_screen: libc::c_int,
    pub nscreens: libc::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: libc::c_ulong,
    pub private16: libc::c_ulong,
    pub min_keycode: libc::c_int,
    pub max_keycode: libc::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: libc::c_int,
    pub xdefaults: *mut libc::c_char,
}
pub type _XPrivDisplay = *mut C2RustUnnamed_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub keycode: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XButtonEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub button: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMotionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub is_hint: libc::c_char,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
    pub same_screen: libc::c_int,
    pub focus: libc::c_int,
    pub state: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XReparentEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGravityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub detail: libc::c_int,
    pub value_mask: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColormapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: libc::c_int,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: libc::c_int,
    pub data: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub b: [libc::c_char; 20],
    pub s: [libc::c_short; 10],
    pub l: [libc::c_long; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMappingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub request: libc::c_int,
    pub first_keycode: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_0: libc::c_int,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: libc::c_ulong,
    pub error_code: libc::c_uchar,
    pub request_code: libc::c_uchar,
    pub minor_code: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
    pub cookie: libc::c_uint,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union _XEvent {
    pub type_0: libc::c_int,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [libc::c_long; 24],
}
pub type XEvent = _XEvent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCharStruct {
    pub lbearing: libc::c_short,
    pub rbearing: libc::c_short,
    pub width: libc::c_short,
    pub ascent: libc::c_short,
    pub descent: libc::c_short,
    pub attributes: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFontProp {
    pub name: Atom,
    pub card32: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFontStruct {
    pub ext_data: *mut XExtData,
    pub fid: Font,
    pub direction: libc::c_uint,
    pub min_char_or_byte2: libc::c_uint,
    pub max_char_or_byte2: libc::c_uint,
    pub min_byte1: libc::c_uint,
    pub max_byte1: libc::c_uint,
    pub all_chars_exist: libc::c_int,
    pub default_char: libc::c_uint,
    pub n_properties: libc::c_int,
    pub properties: *mut XFontProp,
    pub min_bounds: XCharStruct,
    pub max_bounds: XCharStruct,
    pub per_char: *mut XCharStruct,
    pub ascent: libc::c_int,
    pub descent: libc::c_int,
}
pub type XIOErrorHandler
    =
    Option<unsafe extern "C" fn(_: *mut Display) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSizeHints {
    pub flags: libc::c_long,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub min_width: libc::c_int,
    pub min_height: libc::c_int,
    pub max_width: libc::c_int,
    pub max_height: libc::c_int,
    pub width_inc: libc::c_int,
    pub height_inc: libc::c_int,
    pub min_aspect: C2RustUnnamed_2,
    pub max_aspect: C2RustUnnamed_2,
    pub base_width: libc::c_int,
    pub base_height: libc::c_int,
    pub win_gravity: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XTextProperty {
    pub value: *mut libc::c_uchar,
    pub encoding: Atom,
    pub format: libc::c_int,
    pub nitems: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClassHint {
    pub res_name: *mut libc::c_char,
    pub res_class: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XComposeStatus {
    pub compose_ptr: XPointer,
    pub chars_matched: libc::c_int,
}
pub type XComposeStatus = _XComposeStatus;
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
pub struct BITMAPFILEHEADER {
    pub bfType: u16b,
    pub bfSize: u32b,
    pub bfReserved1: u16b,
    pub bfReserved2: u16b,
    pub bfOffBits: u32b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BITMAPINFOHEADER {
    pub biSize: u32b,
    pub biWidth: u32b,
    pub biHeight: u32b,
    pub biPlanes: u16b,
    pub biBitCount: u16b,
    pub biCompresion: u32b,
    pub biSizeImage: u32b,
    pub biXPelsPerMeter: u32b,
    pub biYPelsPerMeter: u32b,
    pub biClrUsed: u32b,
    pub biClrImportand: u32b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RGBQUAD {
    pub b: libc::c_uchar,
    pub g: libc::c_uchar,
    pub r: libc::c_uchar,
    pub filler: libc::c_uchar,
}
/* File: main-x11.c */
/*
 * Copyright (c) 1997 Ben Harrison, and others
 *
 * This software may be copied and distributed for educational, research,
 * and not for profit purposes provided that this copyright and statement
 * are included in all such copies.
 */
/*
 * This file helps Angband work with UNIX/X11 computers.
 *
 * To use this file, compile with "USE_X11" defined, and link against all
 * the various "X11" libraries which may be needed.
 *
 * See also "main-xaw.c".
 *
 * Part of this file provides a user interface package composed of several
 * pseudo-objects, including "metadpy" (a display), "infowin" (a window),
 * "infoclr" (a color), and "infofnt" (a font).  Actually, the package was
 * originally much more interesting, but it was bastardized to keep this
 * file simple.
 *
 * The rest of this file is an implementation of "main-xxx.c" for X11.
 *
 * Most of this file is by Ben Harrison (benh@phial.com).
 */
/*
 * The following shell script can be used to launch Angband, assuming that
 * it was extracted into "~/Angband", and compiled using "USE_X11", on a
 * Linux machine, with a 1280x1024 screen, using 6 windows (with the given
 * characteristics), with gamma correction of 1.8 -> (1 / 1.8) * 256 = 142,
 * and without graphics (add "-g" for graphics).  Just copy this comment
 * into a file, remove the leading " * " characters (and the head/tail of
 * this comment), and make the file executable.
 *
 *
 * #!/bin/csh
 *
 * # Describe attempt
 * echo "Launching angband..."
 * sleep 2
 *
 * # Main window
 * setenv ANGBAND_X11_FONT_0 10x20
 * setenv ANGBAND_X11_AT_X_0 5
 * setenv ANGBAND_X11_AT_Y_0 510
 *
 * # Message window
 * setenv ANGBAND_X11_FONT_1 8x13
 * setenv ANGBAND_X11_AT_X_1 5
 * setenv ANGBAND_X11_AT_Y_1 22
 * setenv ANGBAND_X11_ROWS_1 35
 *
 * # Inventory window
 * setenv ANGBAND_X11_FONT_2 8x13
 * setenv ANGBAND_X11_AT_X_2 635
 * setenv ANGBAND_X11_AT_Y_2 182
 * setenv ANGBAND_X11_ROWS_3 23
 *
 * # Equipment window
 * setenv ANGBAND_X11_FONT_3 8x13
 * setenv ANGBAND_X11_AT_X_3 635
 * setenv ANGBAND_X11_AT_Y_3 22
 * setenv ANGBAND_X11_ROWS_3 12
 *
 * # Monster recall window
 * setenv ANGBAND_X11_FONT_4 6x13
 * setenv ANGBAND_X11_AT_X_4 817
 * setenv ANGBAND_X11_AT_Y_4 847
 * setenv ANGBAND_X11_COLS_4 76
 * setenv ANGBAND_X11_ROWS_4 11
 *
 * # Object recall window
 * setenv ANGBAND_X11_FONT_5 6x13
 * setenv ANGBAND_X11_AT_X_5 817
 * setenv ANGBAND_X11_AT_Y_5 520
 * setenv ANGBAND_X11_COLS_5 76
 * setenv ANGBAND_X11_ROWS_5 24
 *
 * # The build directory
 * cd ~/Angband
 *
 * # Gamma correction
 * setenv ANGBAND_X11_GAMMA 142
 *
 * # Launch Angband
 * ./src/angband -mx11 -- -n6 &
 *
 */
/* __MAKEDEPEND__ */
/* /me pffts Solaris */
/*
 * Include some helpful X11 code.
 */
/*
 * Hack -- avoid some compiler warnings
 */
/*
 * Notes on Colors:
 *
 *   1) On a monochrome (or "fake-monochrome") display, all colors
 *   will be "cast" to "fg," except for the bg color, which is,
 *   obviously, cast to "bg".  Thus, one can ignore this setting.
 *
 *   2) Because of the inner functioning of the color allocation
 *   routines, colors may be specified as (a) a typical color name,
 *   (b) a hexidecimal color specification (preceded by a pound sign),
 *   or (c) by strings such as "fg", "bg", "zg".
 *
 *   3) Due to the workings of the init routines, many colors
 *   may also be dealt with by their actual pixel values.  Note that
 *   the pixel with all bits set is "zg = (1<<metadpy->depth)-1", which
 *   is not necessarily either black or white.
 */
/* *** Generic Types ****/
/*
 * An X11 pixell specifier
 */
pub type Pixell = libc::c_ulong;
/*
 * A structure summarizing a given Display.
 *
 *	- The Display itself
 *	- The default Screen for the display
 *	- The virtual root (usually just the root)
 *	- The default colormap (from a macro)
 *
 *	- The "name" of the display
 *
 *	- The socket to listen to for events
 *
 *	- The width of the display screen (from a macro)
 *	- The height of the display screen (from a macro)
 *	- The bit depth of the display screen (from a macro)
 *
 *	- The black Pixell (from a macro)
 *	- The white Pixell (from a macro)
 *
 *	- The background Pixell (default: black)
 *	- The foreground Pixell (default: white)
 *	- The maximal Pixell (Equals: ((2 ^ depth)-1), is usually ugly)
 *
 *	- Bit Flag: Force all colors to black and white (default: !color)
 *	- Bit Flag: Allow the use of color (default: depth > 1)
 *	- Bit Flag: We created 'dpy', and so should nuke it when done.
 */
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct metadpy {
    pub dpy: *mut Display,
    pub screen: *mut Screen,
    pub root: Window,
    pub cmap: Colormap,
    pub name: *mut libc::c_char,
    pub fd: libc::c_int,
    pub width: uint_hack,
    pub height: uint_hack,
    pub depth: uint_hack,
    pub black: Pixell,
    pub white: Pixell,
    pub bg: Pixell,
    pub fg: Pixell,
    pub zg: Pixell,
    #[bitfield(name = "mono", ty = "uint_hack", bits = "0..=0")]
    #[bitfield(name = "color", ty = "uint_hack", bits = "1..=1")]
    #[bitfield(name = "nuke", ty = "uint_hack", bits = "2..=2")]
    pub mono_color_nuke: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
/*
 * A Structure summarizing Window Information.
 *
 * I assume that a window is at most 30000 pixels on a side.
 * I assume that the root windw is also at most 30000 square.
 *
 *	- The Window
 *	- The current Input Event Mask
 *
 *	- The location of the window
 *	- The width, height of the window
 *	- The border width of this window
 *
 *	- Byte: 1st Extra byte
 *
 *	- Bit Flag: This window is currently Mapped
 *	- Bit Flag: This window needs to be redrawn
 *	- Bit Flag: This window has been resized
 *
 *	- Bit Flag: We should nuke 'win' when done with it
 *
 *	- Bit Flag: 1st extra flag
 *	- Bit Flag: 2nd extra flag
 *	- Bit Flag: 3rd extra flag
 *	- Bit Flag: 4th extra flag
 */
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct infowin {
    pub win: Window,
    pub mask: libc::c_long,
    pub ox: s16b,
    pub oy: s16b,
    pub x: s16b,
    pub y: s16b,
    pub w: s16b,
    pub h: s16b,
    pub b: u16b,
    pub byte1: byte_hack,
    #[bitfield(name = "mapped", ty = "uint_hack", bits = "0..=0")]
    #[bitfield(name = "redraw", ty = "uint_hack", bits = "1..=1")]
    #[bitfield(name = "resize", ty = "uint_hack", bits = "2..=2")]
    #[bitfield(name = "nuke", ty = "uint_hack", bits = "3..=3")]
    #[bitfield(name = "flag1", ty = "uint_hack", bits = "4..=4")]
    #[bitfield(name = "flag2", ty = "uint_hack", bits = "5..=5")]
    #[bitfield(name = "flag3", ty = "uint_hack", bits = "6..=6")]
    #[bitfield(name = "flag4", ty = "uint_hack", bits = "7..=7")]
    pub mapped_redraw_resize_nuke_flag1_flag2_flag3_flag4: [u8; 1],
}
/*
 * A Structure summarizing Operation+Color Information
 *
 *	- The actual GC corresponding to this info
 *
 *	- The Foreground Pixell Value
 *	- The Background Pixell Value
 *
 *	- Num (0-15): The operation code (As in Clear, Xor, etc)
 *	- Bit Flag: The GC is in stipple mode
 *	- Bit Flag: Destroy 'gc' at Nuke time.
 */
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct infoclr {
    pub gc: GC,
    pub fg: Pixell,
    pub bg: Pixell,
    #[bitfield(name = "code", ty = "uint_hack", bits = "0..=3")]
    #[bitfield(name = "stip", ty = "uint_hack", bits = "4..=4")]
    #[bitfield(name = "nuke", ty = "uint_hack", bits = "5..=5")]
    pub code_stip_nuke: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
/*
 * A Structure to Hold Font Information
 *
 *	- The 'XFontStruct*' (yields the 'Font')
 *
 *	- The font name
 *
 *	- The default character width
 *	- The default character height
 *	- The default character ascent
 *
 *	- Byte: Pixel offset used during fake mono
 *
 *	- Flag: Force monospacing via 'wid'
 *	- Flag: Nuke info when done
 */
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct infofnt {
    pub info: *mut XFontStruct,
    pub name: cptr,
    pub wid: s16b,
    pub twid: s16b,
    pub hgt: s16b,
    pub asc: s16b,
    pub off: byte_hack,
    #[bitfield(name = "mono", ty = "uint_hack", bits = "0..=0")]
    #[bitfield(name = "nuke", ty = "uint_hack", bits = "1..=1")]
    pub mono_nuke: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
/*
 * A structure for each "term"
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct term_data {
    pub t: term,
    pub fnt: *mut infofnt,
    pub win: *mut infowin,
    pub tiles: *mut XImage,
    pub TmpImage: *mut XImage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct co_ord {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x11_selection_type {
    pub select: bool_,
    pub drawn: bool_,
    pub t: *mut term,
    pub init: co_ord,
    pub cur: co_ord,
    pub old: co_ord,
    pub time: Time,
}
unsafe extern "C" fn create_pixel(mut dpy: *mut Display, mut red: byte_hack,
                                  mut green: byte_hack, mut blue: byte_hack)
 -> libc::c_ulong {
    let mut cmap: Colormap =
        (*(*(dpy as
                 _XPrivDisplay)).screens.offset((*(dpy as
                                                       _XPrivDisplay)).default_screen
                                                    as isize)).cmap;
    let mut cname: [libc::c_char; 8] = [0; 8];
    let mut xcolour: XColor =
        XColor{pixel: 0, red: 0, green: 0, blue: 0, flags: 0, pad: 0,};
    xcolour.red =
        (red as libc::c_int * 255 as libc::c_int + red as libc::c_int) as
            libc::c_ushort;
    xcolour.green =
        (green as libc::c_int * 255 as libc::c_int + green as libc::c_int) as
            libc::c_ushort;
    xcolour.blue =
        (blue as libc::c_int * 255 as libc::c_int + blue as libc::c_int) as
            libc::c_ushort;
    xcolour.flags =
        ((1 as libc::c_int) << 0 as libc::c_int |
             (1 as libc::c_int) << 1 as libc::c_int |
             (1 as libc::c_int) << 2 as libc::c_int) as libc::c_char;
    if XAllocColor(dpy, cmap, &mut xcolour) == 0 {
        quit_fmt(b"Couldn\'t allocate bitmap color \'%s\'\n\x00" as *const u8
                     as *const libc::c_char, cname.as_mut_ptr());
    }
    return xcolour.pixel;
}
unsafe extern "C" fn get_byte(mut fff: *mut FILE) -> byte_hack {
    return (getc(fff) & 0xff as libc::c_int) as byte_hack;
}
unsafe extern "C" fn rd_byte(mut fff: *mut FILE, mut ip: *mut byte_hack) {
    *ip = get_byte(fff);
}
unsafe extern "C" fn rd_u16b(mut fff: *mut FILE, mut ip: *mut u16b) {
    *ip = get_byte(fff) as u16b;
    *ip =
        (*ip as libc::c_int |
             (get_byte(fff) as u16b as libc::c_int) << 8 as libc::c_int) as
            u16b;
}
unsafe extern "C" fn rd_u32b(mut fff: *mut FILE, mut ip: *mut u32b) {
    *ip = get_byte(fff) as u32b;
    *ip |= (get_byte(fff) as u32b) << 8 as libc::c_int;
    *ip |= (get_byte(fff) as u32b) << 16 as libc::c_int;
    *ip |= (get_byte(fff) as u32b) << 24 as libc::c_int;
}
unsafe extern "C" fn ReadBMP(mut dpy: *mut Display,
                             mut Name: *mut libc::c_char) -> *mut XImage {
    let mut visual: *mut Visual =
        (*(*(dpy as
                 _XPrivDisplay)).screens.offset((*(dpy as
                                                       _XPrivDisplay)).default_screen
                                                    as isize)).root_visual;
    let mut depth: libc::c_int =
        (*(*(dpy as
                 _XPrivDisplay)).screens.offset((*(dpy as
                                                       _XPrivDisplay)).default_screen
                                                    as isize)).root_depth;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut fileheader: BITMAPFILEHEADER =
        BITMAPFILEHEADER{bfType: 0,
                         bfSize: 0,
                         bfReserved1: 0,
                         bfReserved2: 0,
                         bfOffBits: 0,};
    let mut infoheader: BITMAPINFOHEADER =
        BITMAPINFOHEADER{biSize: 0,
                         biWidth: 0,
                         biHeight: 0,
                         biPlanes: 0,
                         biBitCount: 0,
                         biCompresion: 0,
                         biSizeImage: 0,
                         biXPelsPerMeter: 0,
                         biYPelsPerMeter: 0,
                         biClrUsed: 0,
                         biClrImportand: 0,};
    let mut Res: *mut XImage = 0 as *mut XImage;
    let mut Data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ncol: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut x: u32b = 0;
    let mut y: u32b = 0;
    let mut clr_pixels: [libc::c_ulong; 256] = [0; 256];
    f = fopen(Name, b"r\x00" as *const u8 as *const libc::c_char);
    if f.is_null() { return 0 as *mut XImage }
    rd_u16b(f, &mut fileheader.bfType);
    rd_u32b(f, &mut fileheader.bfSize);
    rd_u16b(f, &mut fileheader.bfReserved1);
    rd_u16b(f, &mut fileheader.bfReserved2);
    rd_u32b(f, &mut fileheader.bfOffBits);
    rd_u32b(f, &mut infoheader.biSize);
    rd_u32b(f, &mut infoheader.biWidth);
    rd_u32b(f, &mut infoheader.biHeight);
    rd_u16b(f, &mut infoheader.biPlanes);
    rd_u16b(f, &mut infoheader.biBitCount);
    rd_u32b(f, &mut infoheader.biCompresion);
    rd_u32b(f, &mut infoheader.biSizeImage);
    rd_u32b(f, &mut infoheader.biXPelsPerMeter);
    rd_u32b(f, &mut infoheader.biYPelsPerMeter);
    rd_u32b(f, &mut infoheader.biClrUsed);
    rd_u32b(f, &mut infoheader.biClrImportand);
    if feof(f) != 0 ||
           fileheader.bfType as libc::c_int != 19778 as libc::c_int ||
           infoheader.biSize != 40 as libc::c_int as libc::c_uint {
        quit_fmt(b"Incorrect BMP file format %s\x00" as *const u8 as
                     *const libc::c_char, Name);
    }
    ncol =
        fileheader.bfOffBits.wrapping_sub(54 as libc::c_int as
                                              libc::c_uint).wrapping_div(4 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
            as libc::c_int;
    i = 0 as libc::c_int;
    while i < ncol {
        let mut clrg: RGBQUAD = RGBQUAD{b: 0, g: 0, r: 0, filler: 0,};
        rd_byte(f, &mut clrg.b);
        rd_byte(f, &mut clrg.g);
        rd_byte(f, &mut clrg.r);
        rd_byte(f, &mut clrg.filler);
        clr_pixels[i as usize] = create_pixel(dpy, clrg.r, clrg.g, clrg.b);
        i += 1
    }
    i = 1 as libc::c_int;
    j = depth - 1 as libc::c_int >> 2 as libc::c_int;
    loop  {
        j >>= 1 as libc::c_int;
        if !(j != 0) { break ; }
        i <<= 1 as libc::c_int
    }
    total =
        infoheader.biWidth.wrapping_mul(infoheader.biHeight).wrapping_mul(i as
                                                                              libc::c_uint)
            as libc::c_int;
    Data =
        memset(ralloc((total as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (total as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    Res =
        XCreateImage(dpy, visual, depth as libc::c_uint, 2 as libc::c_int,
                     0 as libc::c_int, Data, infoheader.biWidth,
                     infoheader.biHeight, 8 as libc::c_int, 0 as libc::c_int);
    if Res.is_null() {
        Data =
            rnfree(Data as vptr,
                   (total as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                        as libc::c_ulong)) as
                *mut libc::c_char;
        fclose(f);
        return 0 as *mut XImage
    }
    y = 0 as libc::c_int as u32b;
    while y < infoheader.biHeight {
        let mut y2: libc::c_int =
            infoheader.biHeight.wrapping_sub(y).wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                as libc::c_int;
        x = 0 as libc::c_int as u32b;
        while x < infoheader.biWidth {
            let mut ch: libc::c_int = getc(f);
            if feof(f) != 0 {
                quit_fmt(b"Unexpected end of file in %s\x00" as *const u8 as
                             *const libc::c_char, Name);
            }
            if infoheader.biBitCount as libc::c_int == 24 as libc::c_int {
                let mut c2: libc::c_int = getc(f);
                let mut c3: libc::c_int = getc(f);
                if feof(f) != 0 {
                    quit_fmt(b"Unexpected end of file in %s\x00" as *const u8
                                 as *const libc::c_char, Name);
                }
                Some((*Res).f.put_pixel.expect("non-null function pointer")).expect("non-null function pointer")(Res,
                                                                                                                 x
                                                                                                                     as
                                                                                                                     libc::c_int,
                                                                                                                 y2,
                                                                                                                 create_pixel(dpy,
                                                                                                                              ch
                                                                                                                                  as
                                                                                                                                  byte_hack,
                                                                                                                              c2
                                                                                                                                  as
                                                                                                                                  byte_hack,
                                                                                                                              c3
                                                                                                                                  as
                                                                                                                                  byte_hack));
            } else if infoheader.biBitCount as libc::c_int == 8 as libc::c_int
             {
                Some((*Res).f.put_pixel.expect("non-null function pointer")).expect("non-null function pointer")(Res,
                                                                                                                 x
                                                                                                                     as
                                                                                                                     libc::c_int,
                                                                                                                 y2,
                                                                                                                 clr_pixels[ch
                                                                                                                                as
                                                                                                                                usize]);
            } else if infoheader.biBitCount as libc::c_int == 4 as libc::c_int
             {
                Some((*Res).f.put_pixel.expect("non-null function pointer")).expect("non-null function pointer")(Res,
                                                                                                                 x
                                                                                                                     as
                                                                                                                     libc::c_int,
                                                                                                                 y2,
                                                                                                                 clr_pixels[(ch
                                                                                                                                 /
                                                                                                                                 16
                                                                                                                                     as
                                                                                                                                     libc::c_int)
                                                                                                                                as
                                                                                                                                usize]);
                x = x.wrapping_add(1);
                Some((*Res).f.put_pixel.expect("non-null function pointer")).expect("non-null function pointer")(Res,
                                                                                                                 x
                                                                                                                     as
                                                                                                                     libc::c_int,
                                                                                                                 y2,
                                                                                                                 clr_pixels[(ch
                                                                                                                                 %
                                                                                                                                 16
                                                                                                                                     as
                                                                                                                                     libc::c_int)
                                                                                                                                as
                                                                                                                                usize]);
            } else {
                quit_fmt(b"Illegal biBitCount %d in %s\x00" as *const u8 as
                             *const libc::c_char,
                         infoheader.biBitCount as libc::c_int, Name);
            }
            x = x.wrapping_add(1)
        }
        y = y.wrapping_add(1)
    }
    fclose(f);
    return Res;
}
static mut redMask: libc::c_ulong = 0;
static mut greenMask: libc::c_ulong = 0;
static mut blueMask: libc::c_ulong = 0;
static mut redShift: libc::c_int = 0;
static mut greenShift: libc::c_int = 0;
static mut blueShift: libc::c_int = 0;
static mut smoothRescaling: bool_ = 1 as libc::c_int as bool_;
unsafe extern "C" fn GetScaledRow(mut Im: *mut XImage, mut x: libc::c_int,
                                  mut y: libc::c_int, mut iw: libc::c_int,
                                  mut ow: libc::c_int,
                                  mut redScan: *mut libc::c_ulong,
                                  mut greenScan: *mut libc::c_ulong,
                                  mut blueScan: *mut libc::c_ulong) {
    let mut xi: libc::c_int = 0;
    let mut si: libc::c_int = 0;
    let mut sifrac: libc::c_int = 0;
    let mut ci: libc::c_int = 0;
    let mut cifrac: libc::c_int = 0;
    let mut addWhole: libc::c_int = 0;
    let mut addFrac: libc::c_int = 0;
    let mut pix: libc::c_ulong = 0;
    let mut prevRed: libc::c_int = 0;
    let mut prevGreen: libc::c_int = 0;
    let mut prevBlue: libc::c_int = 0;
    let mut nextRed: libc::c_int = 0;
    let mut nextGreen: libc::c_int = 0;
    let mut nextBlue: libc::c_int = 0;
    let mut getNextPix: bool_ = 0;
    if iw == ow {
        xi = 0 as libc::c_int;
        while xi < ow {
            pix =
                Some((*Im).f.get_pixel.expect("non-null function pointer")).expect("non-null function pointer")(Im,
                                                                                                                x
                                                                                                                    +
                                                                                                                    xi,
                                                                                                                y);
            *redScan.offset(xi as isize) = pix >> redShift & redMask;
            *greenScan.offset(xi as isize) = pix >> greenShift & greenMask;
            *blueScan.offset(xi as isize) = pix >> blueShift & blueMask;
            xi += 1
        }
    } else if iw < ow {
        iw -= 1;
        ow -= 1;
        pix =
            Some((*Im).f.get_pixel.expect("non-null function pointer")).expect("non-null function pointer")(Im,
                                                                                                            x,
                                                                                                            y);
        nextRed = (pix >> redShift & redMask) as libc::c_int;
        nextGreen = (pix >> greenShift & greenMask) as libc::c_int;
        nextBlue = (pix >> blueShift & blueMask) as libc::c_int;
        prevRed = nextRed;
        prevGreen = nextGreen;
        prevBlue = nextBlue;
        si = x;
        sifrac = 0 as libc::c_int;
        getNextPix = 1 as libc::c_int as bool_;
        xi = 0 as libc::c_int;
        while xi <= ow {
            if getNextPix != 0 {
                prevRed = nextRed;
                prevGreen = nextGreen;
                prevBlue = nextBlue;
                if xi < ow {
                    pix =
                        Some((*Im).f.get_pixel.expect("non-null function pointer")).expect("non-null function pointer")(Im,
                                                                                                                        si
                                                                                                                            +
                                                                                                                            1
                                                                                                                                as
                                                                                                                                libc::c_int,
                                                                                                                        y);
                    nextRed = (pix >> redShift & redMask) as libc::c_int;
                    nextGreen =
                        (pix >> greenShift & greenMask) as libc::c_int;
                    nextBlue = (pix >> blueShift & blueMask) as libc::c_int
                }
            }
            *redScan.offset(xi as isize) =
                (prevRed * (ow - sifrac) + nextRed * sifrac) as libc::c_ulong;
            *greenScan.offset(xi as isize) =
                (prevGreen * (ow - sifrac) + nextGreen * sifrac) as
                    libc::c_ulong;
            *blueScan.offset(xi as isize) =
                (prevBlue * (ow - sifrac) + nextBlue * sifrac) as
                    libc::c_ulong;
            sifrac += iw;
            if sifrac >= ow {
                si += 1;
                sifrac -= ow;
                getNextPix = 1 as libc::c_int as bool_
            } else { getNextPix = 0 as libc::c_int as bool_ }
            xi += 1
        }
    } else {
        addWhole = iw / ow;
        addFrac = iw % ow;
        si = x;
        sifrac = 0 as libc::c_int;
        pix =
            Some((*Im).f.get_pixel.expect("non-null function pointer")).expect("non-null function pointer")(Im,
                                                                                                            x,
                                                                                                            y);
        nextRed = (pix >> redShift & redMask) as libc::c_int;
        nextGreen = (pix >> greenShift & greenMask) as libc::c_int;
        nextBlue = (pix >> blueShift & blueMask) as libc::c_int;
        xi = 0 as libc::c_int;
        while xi < ow {
            ci = si + addWhole;
            cifrac = sifrac + addFrac;
            if cifrac >= ow { ci += 1; cifrac -= ow }
            *redScan.offset(xi as isize) =
                (nextRed * (ow - sifrac)) as libc::c_ulong;
            *greenScan.offset(xi as isize) =
                (nextGreen * (ow - sifrac)) as libc::c_ulong;
            *blueScan.offset(xi as isize) =
                (nextBlue * (ow - sifrac)) as libc::c_ulong;
            si += 1;
            while si < ci {
                pix =
                    Some((*Im).f.get_pixel.expect("non-null function pointer")).expect("non-null function pointer")(Im,
                                                                                                                    si,
                                                                                                                    y);
                let ref mut fresh0 = *redScan.offset(xi as isize);
                *fresh0 =
                    (*fresh0).wrapping_add((pix >> redShift &
                                                redMask).wrapping_mul(ow as
                                                                          libc::c_ulong));
                let ref mut fresh1 = *greenScan.offset(xi as isize);
                *fresh1 =
                    (*fresh1).wrapping_add((pix >> greenShift &
                                                greenMask).wrapping_mul(ow as
                                                                            libc::c_ulong));
                let ref mut fresh2 = *blueScan.offset(xi as isize);
                *fresh2 =
                    (*fresh2).wrapping_add((pix >> blueShift &
                                                blueMask).wrapping_mul(ow as
                                                                           libc::c_ulong));
                si += 1
            }
            if xi < ow - 1 as libc::c_int {
                pix =
                    Some((*Im).f.get_pixel.expect("non-null function pointer")).expect("non-null function pointer")(Im,
                                                                                                                    si,
                                                                                                                    y);
                nextRed = (pix >> redShift & redMask) as libc::c_int;
                nextGreen = (pix >> greenShift & greenMask) as libc::c_int;
                nextBlue = (pix >> blueShift & blueMask) as libc::c_int
            }
            sifrac = cifrac;
            if sifrac > 0 as libc::c_int {
                let ref mut fresh3 = *redScan.offset(xi as isize);
                *fresh3 =
                    (*fresh3).wrapping_add((nextRed * sifrac) as
                                               libc::c_ulong);
                let ref mut fresh4 = *greenScan.offset(xi as isize);
                *fresh4 =
                    (*fresh4).wrapping_add((nextGreen * sifrac) as
                                               libc::c_ulong);
                let ref mut fresh5 = *blueScan.offset(xi as isize);
                *fresh5 =
                    (*fresh5).wrapping_add((nextBlue * sifrac) as
                                               libc::c_ulong)
            }
            xi += 1
        }
    };
}
unsafe extern "C" fn PutRGBScan(mut Im: *mut XImage, mut x: libc::c_int,
                                mut y: libc::c_int, mut w: libc::c_int,
                                mut div: libc::c_int,
                                mut redScan: *mut libc::c_ulong,
                                mut greenScan: *mut libc::c_ulong,
                                mut blueScan: *mut libc::c_ulong) {
    let mut xi: libc::c_int = 0;
    let mut pix: libc::c_ulong = 0;
    let mut adj: libc::c_ulong = (div / 2 as libc::c_int) as libc::c_ulong;
    xi = 0 as libc::c_int;
    while xi < w {
        pix =
            (((*redScan.offset(xi as
                                   isize)).wrapping_add(adj).wrapping_div(div
                                                                              as
                                                                              libc::c_ulong)
                  & redMask) <<
                 redShift).wrapping_add(((*greenScan.offset(xi as
                                                                isize)).wrapping_add(adj).wrapping_div(div
                                                                                                           as
                                                                                                           libc::c_ulong)
                                             & greenMask) <<
                                            greenShift).wrapping_add(((*blueScan.offset(xi
                                                                                            as
                                                                                            isize)).wrapping_add(adj).wrapping_div(div
                                                                                                                                       as
                                                                                                                                       libc::c_ulong)
                                                                          &
                                                                          blueMask)
                                                                         <<
                                                                         blueShift);
        Some((*Im).f.put_pixel.expect("non-null function pointer")).expect("non-null function pointer")(Im,
                                                                                                        x
                                                                                                            +
                                                                                                            xi,
                                                                                                        y,
                                                                                                        pix);
        xi += 1
    };
}
unsafe extern "C" fn ScaleIcon(mut ImIn: *mut XImage, mut ImOut: *mut XImage,
                               mut x1: libc::c_int, mut y1: libc::c_int,
                               mut x2: libc::c_int, mut y2: libc::c_int,
                               mut ix: libc::c_int, mut iy: libc::c_int,
                               mut ox: libc::c_int, mut oy: libc::c_int) {
    let mut div: libc::c_int = 0;
    let mut xi: libc::c_int = 0;
    let mut yi: libc::c_int = 0;
    let mut si: libc::c_int = 0;
    let mut sifrac: libc::c_int = 0;
    let mut ci: libc::c_int = 0;
    let mut cifrac: libc::c_int = 0;
    let mut addWhole: libc::c_int = 0;
    let mut addFrac: libc::c_int = 0;
    let mut prevRed: [libc::c_ulong; 32] = [0; 32];
    let mut prevGreen: [libc::c_ulong; 32] = [0; 32];
    let mut prevBlue: [libc::c_ulong; 32] = [0; 32];
    let mut nextRed: [libc::c_ulong; 32] = [0; 32];
    let mut nextGreen: [libc::c_ulong; 32] = [0; 32];
    let mut nextBlue: [libc::c_ulong; 32] = [0; 32];
    let mut tempRed: [libc::c_ulong; 32] = [0; 32];
    let mut tempGreen: [libc::c_ulong; 32] = [0; 32];
    let mut tempBlue: [libc::c_ulong; 32] = [0; 32];
    let mut getNextRow: bool_ = 0;
    if ix == ox {
        div = 1 as libc::c_int
    } else if ix < ox { div = ox - 1 as libc::c_int } else { div = ix }
    if iy == oy {
        yi = 0 as libc::c_int;
        while yi < oy {
            GetScaledRow(ImIn, x1, y1 + yi, ix, ox, tempRed.as_mut_ptr(),
                         tempGreen.as_mut_ptr(), tempBlue.as_mut_ptr());
            PutRGBScan(ImOut, x2, y2 + yi, ox, div, tempRed.as_mut_ptr(),
                       tempGreen.as_mut_ptr(), tempBlue.as_mut_ptr());
            yi += 1
        }
    } else if iy < oy {
        iy -= 1;
        oy -= 1;
        div *= oy;
        GetScaledRow(ImIn, x1, y1, ix, ox, nextRed.as_mut_ptr(),
                     nextGreen.as_mut_ptr(), nextBlue.as_mut_ptr());
        si = y1;
        sifrac = 0 as libc::c_int;
        getNextRow = 1 as libc::c_int as bool_;
        yi = 0 as libc::c_int;
        while yi <= oy {
            if getNextRow != 0 {
                xi = 0 as libc::c_int;
                while xi < ox {
                    prevRed[xi as usize] = nextRed[xi as usize];
                    prevGreen[xi as usize] = nextGreen[xi as usize];
                    prevBlue[xi as usize] = nextBlue[xi as usize];
                    xi += 1
                }
                if yi < oy {
                    GetScaledRow(ImIn, x1, si + 1 as libc::c_int, ix, ox,
                                 nextRed.as_mut_ptr(), nextGreen.as_mut_ptr(),
                                 nextBlue.as_mut_ptr());
                }
            }
            xi = 0 as libc::c_int;
            while xi < ox {
                tempRed[xi as usize] =
                    prevRed[xi as
                                usize].wrapping_mul((oy - sifrac) as
                                                        libc::c_ulong).wrapping_add(nextRed[xi
                                                                                                as
                                                                                                usize].wrapping_mul(sifrac
                                                                                                                        as
                                                                                                                        libc::c_ulong));
                tempGreen[xi as usize] =
                    prevGreen[xi as
                                  usize].wrapping_mul((oy - sifrac) as
                                                          libc::c_ulong).wrapping_add(nextGreen[xi
                                                                                                    as
                                                                                                    usize].wrapping_mul(sifrac
                                                                                                                            as
                                                                                                                            libc::c_ulong));
                tempBlue[xi as usize] =
                    prevBlue[xi as
                                 usize].wrapping_mul((oy - sifrac) as
                                                         libc::c_ulong).wrapping_add(nextBlue[xi
                                                                                                  as
                                                                                                  usize].wrapping_mul(sifrac
                                                                                                                          as
                                                                                                                          libc::c_ulong));
                xi += 1
            }
            PutRGBScan(ImOut, x2, y2 + yi, ox, div, tempRed.as_mut_ptr(),
                       tempGreen.as_mut_ptr(), tempBlue.as_mut_ptr());
            sifrac += iy;
            if sifrac >= oy {
                si += 1;
                sifrac -= oy;
                getNextRow = 1 as libc::c_int as bool_
            } else { getNextRow = 0 as libc::c_int as bool_ }
            yi += 1
        }
    } else {
        div *= iy;
        addWhole = iy / oy;
        addFrac = iy % oy;
        si = y1;
        sifrac = 0 as libc::c_int;
        GetScaledRow(ImIn, x1, y1, ix, ox, nextRed.as_mut_ptr(),
                     nextGreen.as_mut_ptr(), nextBlue.as_mut_ptr());
        yi = 0 as libc::c_int;
        while yi < oy {
            ci = si + addWhole;
            cifrac = sifrac + addFrac;
            if cifrac >= oy { ci += 1; cifrac -= oy }
            xi = 0 as libc::c_int;
            while xi < ox {
                tempRed[xi as usize] =
                    nextRed[xi as
                                usize].wrapping_mul((oy - sifrac) as
                                                        libc::c_ulong);
                tempGreen[xi as usize] =
                    nextGreen[xi as
                                  usize].wrapping_mul((oy - sifrac) as
                                                          libc::c_ulong);
                tempBlue[xi as usize] =
                    nextBlue[xi as
                                 usize].wrapping_mul((oy - sifrac) as
                                                         libc::c_ulong);
                xi += 1
            }
            si += 1;
            while si < ci {
                GetScaledRow(ImIn, x1, si, ix, ox, nextRed.as_mut_ptr(),
                             nextGreen.as_mut_ptr(), nextBlue.as_mut_ptr());
                xi = 0 as libc::c_int;
                while xi < ox {
                    tempRed[xi as usize] =
                        tempRed[xi as
                                    usize].wrapping_add(nextRed[xi as
                                                                    usize].wrapping_mul(oy
                                                                                            as
                                                                                            libc::c_ulong));
                    tempGreen[xi as usize] =
                        tempGreen[xi as
                                      usize].wrapping_add(nextGreen[xi as
                                                                        usize].wrapping_mul(oy
                                                                                                as
                                                                                                libc::c_ulong));
                    tempBlue[xi as usize] =
                        tempBlue[xi as
                                     usize].wrapping_add(nextBlue[xi as
                                                                      usize].wrapping_mul(oy
                                                                                              as
                                                                                              libc::c_ulong));
                    xi += 1
                }
                si += 1
            }
            if yi < oy - 1 as libc::c_int {
                GetScaledRow(ImIn, x1, si, ix, ox, nextRed.as_mut_ptr(),
                             nextGreen.as_mut_ptr(), nextBlue.as_mut_ptr());
            }
            sifrac = cifrac;
            xi = 0 as libc::c_int;
            while xi < ox {
                tempRed[xi as usize] =
                    tempRed[xi as
                                usize].wrapping_add(nextRed[xi as
                                                                usize].wrapping_mul(sifrac
                                                                                        as
                                                                                        libc::c_ulong));
                tempGreen[xi as usize] =
                    tempGreen[xi as
                                  usize].wrapping_add(nextGreen[xi as
                                                                    usize].wrapping_mul(sifrac
                                                                                            as
                                                                                            libc::c_ulong));
                tempBlue[xi as usize] =
                    tempBlue[xi as
                                 usize].wrapping_add(nextBlue[xi as
                                                                  usize].wrapping_mul(sifrac
                                                                                          as
                                                                                          libc::c_ulong));
                xi += 1
            }
            PutRGBScan(ImOut, x2, y2 + yi, ox, div, tempRed.as_mut_ptr(),
                       tempGreen.as_mut_ptr(), tempBlue.as_mut_ptr());
            yi += 1
        }
    };
}
unsafe extern "C" fn ResizeImageSmooth(mut dpy: *mut Display,
                                       mut Im: *mut XImage,
                                       mut ix: libc::c_int,
                                       mut iy: libc::c_int,
                                       mut ox: libc::c_int,
                                       mut oy: libc::c_int) -> *mut XImage {
    let mut visual: *mut Visual =
        (*(*(dpy as
                 _XPrivDisplay)).screens.offset((*(dpy as
                                                       _XPrivDisplay)).default_screen
                                                    as isize)).root_visual;
    let mut width1: libc::c_int = 0;
    let mut height1: libc::c_int = 0;
    let mut width2: libc::c_int = 0;
    let mut height2: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut Tmp: *mut XImage = 0 as *mut XImage;
    let mut Data: *mut libc::c_char = 0 as *mut libc::c_char;
    width1 = (*Im).width;
    height1 = (*Im).height;
    width2 = ox * width1 / ix;
    height2 = oy * height1 / iy;
    Data =
        malloc((width2 * height2 * (*Im).bits_per_pixel / 8 as libc::c_int) as
                   libc::c_ulong) as *mut libc::c_char;
    Tmp =
        XCreateImage(dpy, visual, (*Im).depth as libc::c_uint,
                     2 as libc::c_int, 0 as libc::c_int, Data,
                     width2 as libc::c_uint, height2 as libc::c_uint,
                     32 as libc::c_int, 0 as libc::c_int);
    redMask = (*Im).red_mask;
    redShift = 0 as libc::c_int;
    while redMask & 1 as libc::c_int as libc::c_ulong ==
              0 as libc::c_int as libc::c_ulong {
        redShift += 1;
        redMask >>= 1 as libc::c_int
    }
    greenMask = (*Im).green_mask;
    greenShift = 0 as libc::c_int;
    while greenMask & 1 as libc::c_int as libc::c_ulong ==
              0 as libc::c_int as libc::c_ulong {
        greenShift += 1;
        greenMask >>= 1 as libc::c_int
    }
    blueMask = (*Im).blue_mask;
    blueShift = 0 as libc::c_int;
    while blueMask & 1 as libc::c_int as libc::c_ulong ==
              0 as libc::c_int as libc::c_ulong {
        blueShift += 1;
        blueMask >>= 1 as libc::c_int
    }
    y1 = 0 as libc::c_int;
    y2 = 0 as libc::c_int;
    while y1 < height1 && y2 < height2 {
        x1 = 0 as libc::c_int;
        x2 = 0 as libc::c_int;
        while x1 < width1 && x2 < width2 {
            ScaleIcon(Im, Tmp, x1, y1, x2, y2, ix, iy, ox, oy);
            x1 += ix;
            x2 += ox
        }
        y1 += iy;
        y2 += oy
    }
    return Tmp;
}
unsafe extern "C" fn ResizeImage(mut dpy: *mut Display, mut Im: *mut XImage,
                                 mut ix: libc::c_int, mut iy: libc::c_int,
                                 mut ox: libc::c_int, mut oy: libc::c_int)
 -> *mut XImage {
    let mut visual: *mut Visual =
        (*(*(dpy as
                 _XPrivDisplay)).screens.offset((*(dpy as
                                                       _XPrivDisplay)).default_screen
                                                    as isize)).root_visual;
    let mut width1: libc::c_int = 0;
    let mut height1: libc::c_int = 0;
    let mut width2: libc::c_int = 0;
    let mut height2: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut Tx: libc::c_int = 0;
    let mut Ty: libc::c_int = 0;
    let mut px1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut px2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dx1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dx2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut py1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut py2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dy1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dy2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut Tmp: *mut XImage = 0 as *mut XImage;
    let mut Data: *mut libc::c_char = 0 as *mut libc::c_char;
    if smoothRescaling as libc::c_int != 0 && (ix != ox || iy != oy) &&
           (*visual).class == 4 as libc::c_int {
        return ResizeImageSmooth(dpy, Im, ix, iy, ox, oy)
    }
    width1 = (*Im).width;
    height1 = (*Im).height;
    width2 = ox * width1 / ix;
    height2 = oy * height1 / iy;
    Data =
        malloc((width2 * height2 * (*Im).bits_per_pixel / 8 as libc::c_int) as
                   libc::c_ulong) as *mut libc::c_char;
    Tmp =
        XCreateImage(dpy, visual, (*Im).depth as libc::c_uint,
                     2 as libc::c_int, 0 as libc::c_int, Data,
                     width2 as libc::c_uint, height2 as libc::c_uint,
                     32 as libc::c_int, 0 as libc::c_int);
    if ix > ox {
        px1 = &mut x1;
        px2 = &mut x2;
        dx1 = &mut ix;
        dx2 = &mut ox
    } else { px1 = &mut x2; px2 = &mut x1; dx1 = &mut ox; dx2 = &mut ix }
    if iy > oy {
        py1 = &mut y1;
        py2 = &mut y2;
        dy1 = &mut iy;
        dy2 = &mut oy
    } else { py1 = &mut y2; py2 = &mut y1; dy1 = &mut oy; dy2 = &mut iy }
    Ty = *dy1 / 2 as libc::c_int;
    y1 = 0 as libc::c_int;
    y2 = 0 as libc::c_int;
    while y1 < height1 && y2 < height2 {
        Tx = *dx1 / 2 as libc::c_int;
        x1 = 0 as libc::c_int;
        x2 = 0 as libc::c_int;
        while x1 < width1 && x2 < width2 {
            Some((*Tmp).f.put_pixel.expect("non-null function pointer")).expect("non-null function pointer")(Tmp,
                                                                                                             x2,
                                                                                                             y2,
                                                                                                             Some((*Im).f.get_pixel.expect("non-null function pointer")).expect("non-null function pointer")(Im,
                                                                                                                                                                                                             x1,
                                                                                                                                                                                                             y1));
            *px1 += 1;
            Tx -= *dx2;
            if Tx < 0 as libc::c_int { Tx += *dx1; *px2 += 1 }
        }
        *py1 += 1;
        Ty -= *dy2;
        if Ty < 0 as libc::c_int { Ty += *dy1; *py2 += 1 }
    }
    return Tmp;
}
/* Initialize 'M' using the standard Display */
/* Init an infowin by giving father as an (info_win*) (or NULL), and data */
/* Init a top level infowin by pos,size,bord,Colors */
/* Request a new standard window by giving Dad infowin and X,Y,W,H */
/* Set the current Infowin */
/* Set the current Infoclr */
/* Set the current infofnt */
/* Errr: Expose Infowin */
/* Errr: Unxpose Infowin */
/* *** Generic Globals ****/
/*
 * The "default" values
 */
static mut metadpy_default: metadpy =
    metadpy{dpy: 0 as *const Display as *mut Display,
            screen: 0 as *const Screen as *mut Screen,
            root: 0,
            cmap: 0,
            name: 0 as *const libc::c_char as *mut libc::c_char,
            fd: 0,
            width: 0,
            height: 0,
            depth: 0,
            black: 0,
            white: 0,
            bg: 0,
            fg: 0,
            zg: 0,
            mono_color_nuke: [0; 1],
            c2rust_padding: [0; 7],};
/*
 * The "current" variables
 */
static mut Metadpy: *mut metadpy =
    unsafe { &metadpy_default as *const metadpy as *mut metadpy };
static mut Infowin: *mut infowin =
    0 as *const libc::c_void as *mut libc::c_void as *mut infowin;
static mut Infoclr: *mut infoclr =
    0 as *const libc::c_void as *mut libc::c_void as *mut infoclr;
static mut Infofnt: *mut infofnt =
    0 as *const libc::c_void as *mut libc::c_void as *mut infofnt;
/* *** Generic code ****/
/*
 * Simple routine to save the state of the game when the display connection
 * is broken. Remember, you cannot do anything in this function that will
 * generate X protocol requests.
 */
#[no_mangle]
pub unsafe extern "C" fn x_io_error_handler(mut d: *mut Display)
 -> libc::c_int {
    /* We have nothing to save */
    if character_generated == 0 { return 0 as libc::c_int }
    save_dungeon();
    save_player();
    return 0 as libc::c_int;
}
/*
 * Calculate how much space there is in the key queue for the current term.
 */
#[no_mangle]
pub unsafe extern "C" fn Term_queue_space() -> libc::c_int {
    /* Find the gap if the tail is before the head. */
    let mut space: libc::c_int =
        (*Term).key_tail as libc::c_int - (*Term).key_head as libc::c_int;
    /* Otherwise, add in the extra for looping. */
    if space <= 0 as libc::c_int {
        space = (*Term).key_size as libc::c_int - space
    }
    /* The last space is never used as that would be interpreted as leaving
	 * no pending keypresses. */
    return space - 1 as libc::c_int;
}
/*
 * Add a series of keypresses to the "queue".
 *
 * Return any errors generated by Term_keypress() in doing so, or SUCCESS
 * if there are none.
 *
 * Catch the "out of space" error before anything is printed.
 *
 * NB: The keys added here will be interpreted by any macros or keymaps.
 */
#[no_mangle]
pub unsafe extern "C" fn type_string(mut str: *mut libc::c_char,
                                     mut len: uint_hack) -> errr {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut old: *mut term = Term;
    /* Paranoia - no string. */
    if str.is_null() { return 5 as libc::c_int }
    /* Hack - calculate the string length here if none given. */
    if len == 0 { len = strlen(str) as uint_hack }
    /* Activate the main window, as all pastes go there. */
    Term_activate(angband_term[0 as libc::c_int as usize]);
    /* Not enough space for the string. */
    if Term_queue_space() <= len as libc::c_int { return 7 as libc::c_int }
    s = str;
    while s < str.offset(len as isize) {
        let mut err: errr = Term_keypress(*s as libc::c_int);
        /* Catch errors other than "str[i] == 0", which is ignored. */
        if err != 0 && err != -(1 as libc::c_int) { return err }
        s = s.offset(1)
    }
    /* Activate the original window. */
    Term_activate(old);
    return 0 as libc::c_int;
}
/*
 * Init the current metadpy, with various initialization stuff.
 *
 * Inputs:
 *	dpy:  The Display* to use (if NULL, create it)
 *	name: The name of the Display (if NULL, the current)
 *
 * Notes:
 *	If 'name' is NULL, but 'dpy' is set, extract name from dpy
 *	If 'dpy' is NULL, then Create the named Display
 *	If 'name' is NULL, and so is 'dpy', use current Display
 *
 * Return -1 if no Display given, and none can be opened.
 */
unsafe extern "C" fn Metadpy_init_2(mut dpy: *mut Display, mut name: cptr)
 -> errr {
    let mut m: *mut metadpy = Metadpy;
    /* ** Open the display if needed ***/
    /* If no Display given, attempt to Create one */
    if dpy.is_null() {
        /* Attempt to open the display */
        dpy = XOpenDisplay(name);
        /* Failure */
        if dpy.is_null() { return -(1 as libc::c_int) }
        /* We will have to nuke it when done */
        (*m).set_nuke(1 as libc::c_int as uint_hack)
    } else {
        /* Since the Display was given, use it */
        /* We will not have to nuke it when done */
        (*m).set_nuke(0 as libc::c_int as uint_hack)
    }
    XSetIOErrorHandler(Some(x_io_error_handler as
                                unsafe extern "C" fn(_: *mut Display)
                                    -> libc::c_int));
    /* ** Save some information ***/
    /* Save the Display itself */
    (*m).dpy = dpy;
    /* Get the Screen and Virtual Root Window */
    (*m).screen =
        &mut *(*(dpy as
                     _XPrivDisplay)).screens.offset((*(dpy as
                                                           _XPrivDisplay)).default_screen
                                                        as isize) as
            *mut Screen;
    (*m).root = (*(*m).screen).root;
    /* Get the default colormap */
    (*m).cmap = (*(*m).screen).cmap;
    /* Extract the true name of the display */
    (*m).name = (*(dpy as _XPrivDisplay)).display_name;
    /* Extract the fd */
    (*m).fd = (*((*Metadpy).dpy as _XPrivDisplay)).fd;
    /* Save the Size and Depth of the screen */
    (*m).width = (*(*m).screen).width as uint_hack;
    (*m).height = (*(*m).screen).height as uint_hack;
    (*m).depth = (*(*m).screen).root_depth as uint_hack;
    /* Save the Standard Colors */
    (*m).black = (*(*m).screen).black_pixel;
    (*m).white = (*(*m).screen).white_pixel;
    /* ** Make some clever Guesses ***/
    /* Guess at the desired 'fg' and 'bg' Pixell's */
    (*m).bg = (*m).black;
    (*m).fg = (*m).white;
    /* Calculate the Maximum allowed Pixel value.  */
    (*m).zg =
        (((1 as libc::c_int) << (*m).depth) - 1 as libc::c_int) as Pixell;
    /* Save various default Flag Settings */
    (*m).set_color(if (*m).depth > 1 as libc::c_int as libc::c_uint {
                       1 as libc::c_int
                   } else { 0 as libc::c_int } as uint_hack);
    (*m).set_mono(if (*m).color() as libc::c_int != 0 {
                      0 as libc::c_int
                  } else { 1 as libc::c_int } as uint_hack);
    /* Return "success" */
    return 0 as libc::c_int;
}
/* IGNORE_UNUSED_FUNCTIONS */
/*
 * General Flush/ Sync/ Discard routine
 */
unsafe extern "C" fn Metadpy_update(mut flush: libc::c_int,
                                    mut sync: libc::c_int,
                                    mut discard: libc::c_int) -> errr {
    /* Flush if desired */
    if flush != 0 { XFlush((*Metadpy).dpy); }
    /* Sync if desired, using 'discard' */
    if sync != 0 { XSync((*Metadpy).dpy, discard); }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Make a simple beep
 */
unsafe extern "C" fn Metadpy_do_beep() -> errr {
    /* Make a simple beep */
    XBell((*Metadpy).dpy, 100 as libc::c_int);
    return 0 as libc::c_int;
}
/*
 * Set the name (in the title bar) of Infowin
 */
unsafe extern "C" fn Infowin_set_name(mut name: cptr) -> errr {
    let mut st: libc::c_int = 0;
    let mut tp: XTextProperty =
        XTextProperty{value: 0 as *mut libc::c_uchar,
                      encoding: 0,
                      format: 0,
                      nitems: 0,};
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut bp: *mut libc::c_char = buf.as_mut_ptr();
    strcpy(buf.as_mut_ptr(), name);
    st = XStringListToTextProperty(&mut bp, 1 as libc::c_int, &mut tp);
    if st != 0 { XSetWMName((*Metadpy).dpy, (*Infowin).win, &mut tp); }
    return 0 as libc::c_int;
}
/* IGNORE_UNUSED_FUNCTIONS */
/*
 * Prepare a new 'infowin'.
 */
unsafe extern "C" fn Infowin_prepare(mut xid: Window) -> errr {
    let mut iwin: *mut infowin = Infowin;
    let mut tmp_win: Window = 0;
    let mut xwa: XWindowAttributes =
        XWindowAttributes{x: 0,
                          y: 0,
                          width: 0,
                          height: 0,
                          border_width: 0,
                          depth: 0,
                          visual: 0 as *mut Visual,
                          root: 0,
                          class: 0,
                          bit_gravity: 0,
                          win_gravity: 0,
                          backing_store: 0,
                          backing_planes: 0,
                          backing_pixel: 0,
                          save_under: 0,
                          colormap: 0,
                          map_installed: 0,
                          map_state: 0,
                          all_event_masks: 0,
                          your_event_mask: 0,
                          do_not_propagate_mask: 0,
                          override_redirect: 0,
                          screen: 0 as *mut Screen,};
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_uint = 0;
    let mut h: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut d: libc::c_uint = 0;
    /* Assign stuff */
    (*iwin).win = xid;
    /* Check For Error XXX Extract some ACTUAL data from 'xid' */
    XGetGeometry((*Metadpy).dpy, xid, &mut tmp_win, &mut x, &mut y, &mut w,
                 &mut h, &mut b, &mut d);
    /* Apply the above info */
    (*iwin).x = x as s16b;
    (*iwin).y = y as s16b;
    (*iwin).w = w as s16b;
    (*iwin).h = h as s16b;
    (*iwin).b = b as u16b;
    /* Check Error XXX Extract some more ACTUAL data */
    XGetWindowAttributes((*Metadpy).dpy, xid, &mut xwa);
    /* Apply the above info */
    (*iwin).mask = xwa.your_event_mask;
    (*iwin).set_mapped(if xwa.map_state == 0 as libc::c_int {
                           0 as libc::c_int
                       } else { 1 as libc::c_int } as uint_hack);
    /* And assume that we are exposed */
    (*iwin).set_redraw(1 as libc::c_int as uint_hack);
    /* Success */
    return 0 as libc::c_int;
}
/* IGNORE_UNUSED_FUNCTIONS */
/*
 * Init an infowin by giving some data.
 *
 * Inputs:
 *	dad: The Window that should own this Window (if any)
 *	x,y: The position of this Window
 *	w,h: The size of this Window
 *	b,d: The border width and pixel depth
 *
 * Notes:
 *	If 'dad == None' assume 'dad == root'
 */
unsafe extern "C" fn Infowin_init_data(mut dad: Window, mut x: libc::c_int,
                                       mut y: libc::c_int, mut w: libc::c_int,
                                       mut h: libc::c_int, mut b: libc::c_int,
                                       mut fg: Pixell, mut bg: Pixell)
 -> errr {
    let mut xid: Window = 0;
    /* Wipe it clean */
    memset(Infowin as *mut libc::c_char as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<infowin>() as libc::c_ulong);
    /* ** Error Check XXX ***/
    /* ** Create the Window 'xid' from data ***/
    /* What happened here?  XXX XXX XXX */
    /* If no parent given, depend on root */
    if dad == 0 as libc::c_long as libc::c_ulong {
        /* #ifdef USE_GRAPHICS

				xid = XCreateWindow(Metadpy->dpy, Metadpy->root, x, y, w, h, b, 8, InputOutput, CopyFromParent, 0, 0);

			else
		*/
        /* #else */
        dad = (*Metadpy).root
    }
    /* #endif */
    /* Create the Window XXX Error Check */
    xid =
        XCreateSimpleWindow((*Metadpy).dpy, dad, x, y, w as libc::c_uint,
                            h as libc::c_uint, b as libc::c_uint, fg, bg);
    /* Start out selecting No events */
    XSelectInput((*Metadpy).dpy, xid, 0 as libc::c_long);
    /* ** Prepare the new infowin ***/
    /* Mark it as nukable */
    (*Infowin).set_nuke(1 as libc::c_int as uint_hack);
    /* Attempt to Initialize the infowin */
    return Infowin_prepare(xid);
}
/*
 * Modify the event mask of an Infowin
 */
unsafe extern "C" fn Infowin_set_mask(mut mask: libc::c_long) -> errr {
    /* Save the new setting */
    (*Infowin).mask = mask;
    /* Execute the Mapping */
    XSelectInput((*Metadpy).dpy, (*Infowin).win, (*Infowin).mask);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Request that Infowin be mapped
 */
unsafe extern "C" fn Infowin_map() -> errr {
    /* Execute the Mapping */
    XMapWindow((*Metadpy).dpy, (*Infowin).win);
    /* Success */
    return 0 as libc::c_int;
}
/* IGNORE_UNUSED_FUNCTIONS */
/*
 * Request that Infowin be raised
 */
unsafe extern "C" fn Infowin_raise() -> errr {
    /* Raise towards visibility */
    XRaiseWindow((*Metadpy).dpy, (*Infowin).win);
    /* Success */
    return 0 as libc::c_int;
}
/* IGNORE_UNUSED_FUNCTIONS */
/*
 * Request that Infowin be moved to a new location
 */
unsafe extern "C" fn Infowin_impell(mut x: libc::c_int, mut y: libc::c_int)
 -> errr {
    /* Execute the request */
    XMoveWindow((*Metadpy).dpy, (*Infowin).win, x, y);
    /* Success */
    return 0 as libc::c_int;
}
/* IGNORE_UNUSED_FUNCTIONS */
/*
 * Visually clear Infowin
 */
unsafe extern "C" fn Infowin_wipe() -> errr {
    /* Execute the request */
    XClearWindow((*Metadpy).dpy, (*Infowin).win);
    /* Success */
    return 0 as libc::c_int;
}
/* IGNORE_UNUSED_FUNCTIONS */
/*
 * A NULL terminated pair list of legal "operation names"
 *
 * Pairs of values, first is texttual name, second is the string
 * holding the decimal value that the operation corresponds to.
 */
static mut opcode_pairs: [cptr; 37] =
    [b"cpy\x00" as *const u8 as *const libc::c_char,
     b"3\x00" as *const u8 as *const libc::c_char,
     b"xor\x00" as *const u8 as *const libc::c_char,
     b"6\x00" as *const u8 as *const libc::c_char,
     b"and\x00" as *const u8 as *const libc::c_char,
     b"1\x00" as *const u8 as *const libc::c_char,
     b"ior\x00" as *const u8 as *const libc::c_char,
     b"7\x00" as *const u8 as *const libc::c_char,
     b"nor\x00" as *const u8 as *const libc::c_char,
     b"8\x00" as *const u8 as *const libc::c_char,
     b"inv\x00" as *const u8 as *const libc::c_char,
     b"10\x00" as *const u8 as *const libc::c_char,
     b"clr\x00" as *const u8 as *const libc::c_char,
     b"0\x00" as *const u8 as *const libc::c_char,
     b"set\x00" as *const u8 as *const libc::c_char,
     b"15\x00" as *const u8 as *const libc::c_char,
     b"src\x00" as *const u8 as *const libc::c_char,
     b"3\x00" as *const u8 as *const libc::c_char,
     b"dst\x00" as *const u8 as *const libc::c_char,
     b"5\x00" as *const u8 as *const libc::c_char,
     b"+andReverse\x00" as *const u8 as *const libc::c_char,
     b"2\x00" as *const u8 as *const libc::c_char,
     b"+andInverted\x00" as *const u8 as *const libc::c_char,
     b"4\x00" as *const u8 as *const libc::c_char,
     b"+noop\x00" as *const u8 as *const libc::c_char,
     b"5\x00" as *const u8 as *const libc::c_char,
     b"+equiv\x00" as *const u8 as *const libc::c_char,
     b"9\x00" as *const u8 as *const libc::c_char,
     b"+orReverse\x00" as *const u8 as *const libc::c_char,
     b"11\x00" as *const u8 as *const libc::c_char,
     b"+copyInverted\x00" as *const u8 as *const libc::c_char,
     b"12\x00" as *const u8 as *const libc::c_char,
     b"+orInverted\x00" as *const u8 as *const libc::c_char,
     b"13\x00" as *const u8 as *const libc::c_char,
     b"+nand\x00" as *const u8 as *const libc::c_char,
     b"14\x00" as *const u8 as *const libc::c_char, 0 as cptr];
/*
 * Parse a word into an operation "code"
 *
 * Inputs:
 *	str: A string, hopefully representing an Operation
 *
 * Output:
 *	0-15: if 'str' is a valid Operation
 *	-1:   if 'str' could not be parsed
 */
unsafe extern "C" fn Infoclr_Opcode(mut str: cptr) -> libc::c_int {
    let mut i: libc::c_int = 0;
    /* Scan through all legal operation names */
    i = 0 as libc::c_int;
    while !opcode_pairs[(i * 2 as libc::c_int) as usize].is_null() {
        /* Is this the right oprname? */
        if streq(opcode_pairs[(i * 2 as libc::c_int) as usize], str) != 0 {
            /* Convert the second element in the pair into a Code */
            return atoi(opcode_pairs[(i * 2 as libc::c_int + 1 as libc::c_int)
                                         as usize])
        }
        i += 1
    }
    /* The code was not found, return -1 */
    return -(1 as libc::c_int);
}
/* IGNORE_UNUSED_FUNCTIONS */
/*
 * Initialize an infoclr with some data
 *
 * Inputs:
 *	fg:   The Pixell for the requested Foreground (see above)
 *	bg:   The Pixell for the requested Background (see above)
 *	op:   The Opcode for the requested Operation (see above)
 *	stip: The stipple mode
 */
unsafe extern "C" fn Infoclr_init_data(mut fg: Pixell, mut bg: Pixell,
                                       mut op: libc::c_int,
                                       mut stip: libc::c_int) -> errr {
    let mut iclr: *mut infoclr = Infoclr;
    let mut gc: GC = 0 as *mut _XGC;
    let mut gcv: XGCValues =
        XGCValues{function: 0,
                  plane_mask: 0,
                  foreground: 0,
                  background: 0,
                  line_width: 0,
                  line_style: 0,
                  cap_style: 0,
                  join_style: 0,
                  fill_style: 0,
                  fill_rule: 0,
                  arc_mode: 0,
                  tile: 0,
                  stipple: 0,
                  ts_x_origin: 0,
                  ts_y_origin: 0,
                  font: 0,
                  subwindow_mode: 0,
                  graphics_exposures: 0,
                  clip_x_origin: 0,
                  clip_y_origin: 0,
                  clip_mask: 0,
                  dash_offset: 0,
                  dashes: 0,};
    let mut gc_mask: libc::c_ulong = 0;
    /* ** Simple error checking of opr and clr ***/
    /* Check the 'Pixells' for realism */
    if bg > (*Metadpy).zg { return -(1 as libc::c_int) }
    if fg > (*Metadpy).zg { return -(1 as libc::c_int) }
    /* Check the data for trueness */
    if op < 0 as libc::c_int || op > 15 as libc::c_int {
        return -(1 as libc::c_int)
    }
    /* ** Create the requested 'GC' ***/
    /* Assign the proper GC function */
    gcv.function = op;
    /* Assign the proper GC background */
    gcv.background = bg;
    /* Assign the proper GC foreground */
    gcv.foreground = fg;
    /* Hack -- Handle XOR (xor is code 6) by hacking bg and fg */
    if op == 6 as libc::c_int {
        gcv.background = 0 as libc::c_int as libc::c_ulong
    }
    if op == 6 as libc::c_int { gcv.foreground = bg ^ fg }
    /* Assign the proper GC Fill Style */
    gcv.fill_style =
        if stip != 0 { 2 as libc::c_int } else { 0 as libc::c_int };
    /* Turn off 'Give exposure events for pixmap copying' */
    gcv.graphics_exposures = 0 as libc::c_int;
    /* Set up the GC mask */
    gc_mask =
        ((1 as libc::c_long) << 0 as libc::c_int |
             (1 as libc::c_long) << 3 as libc::c_int |
             (1 as libc::c_long) << 2 as libc::c_int |
             (1 as libc::c_long) << 8 as libc::c_int |
             (1 as libc::c_long) << 16 as libc::c_int) as libc::c_ulong;
    /* Create the GC detailed above */
    gc = XCreateGC((*Metadpy).dpy, (*Metadpy).root, gc_mask, &mut gcv);
    /* ** Initialize ***/
    /* Wipe the iclr clean */
    memset(iclr as *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<infoclr>() as libc::c_ulong);
    /* Assign the GC */
    (*iclr).gc = gc;
    /* Nuke it when done */
    (*iclr).set_nuke(1 as libc::c_int as uint_hack);
    /* Assign the parms */
    (*iclr).fg = fg;
    (*iclr).bg = bg;
    (*iclr).set_code(op as uint_hack);
    (*iclr).set_stip(if stip != 0 {
                         1 as libc::c_int
                     } else { 0 as libc::c_int } as uint_hack);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Change the 'fg' for an infoclr
 *
 * Inputs:
 *	fg:   The Pixell for the requested Foreground (see above)
 */
unsafe extern "C" fn Infoclr_change_fg(mut fg: Pixell) -> errr {
    let mut iclr: *mut infoclr = Infoclr;
    /* ** Simple error checking of opr and clr ***/
    /* Check the 'Pixells' for realism */
    if fg > (*Metadpy).zg { return -(1 as libc::c_int) }
    /* ** Change ***/
    /* Change */
    XSetForeground((*Metadpy).dpy, (*iclr).gc, fg);
    /* Success */
    return 0 as libc::c_int;
}
/* IGNORE_UNUSED_FUNCTIONS */
/*
 * Prepare a new 'infofnt'
 */
unsafe extern "C" fn Infofnt_prepare(mut info: *mut XFontStruct) -> errr {
    let mut ifnt: *mut infofnt = Infofnt;
    let mut cs: *mut XCharStruct = 0 as *mut XCharStruct;
    /* Assign the struct */
    (*ifnt).info = info;
    /* Jump into the max bouonds thing */
    cs = &mut (*info).max_bounds;
    /* Extract default sizing info */
    (*ifnt).asc = (*info).ascent as s16b;
    (*ifnt).hgt = ((*info).ascent + (*info).descent) as s16b;
    (*ifnt).wid = (*cs).width;
    if use_bigtile != 0 {
        (*ifnt).twid = (2 as libc::c_int * (*ifnt).wid as libc::c_int) as s16b
    } else { (*ifnt).twid = (*ifnt).wid }
    /* Success */
    return 0 as libc::c_int;
}
/* IGNORE_UNUSED_FUNCTIONS */
/*
 * Init an infofnt by its Name
 *
 * Inputs:
 *	name: The name of the requested Font
 */
unsafe extern "C" fn Infofnt_init_data(mut name: cptr) -> errr {
    let mut info: *mut XFontStruct = 0 as *mut XFontStruct;
    /* ** Load the info Fresh, using the name ***/
    /* If the name is not given, report an error */
    if name.is_null() { return -(1 as libc::c_int) }
    /* Attempt to load the font */
    info = XLoadQueryFont((*Metadpy).dpy, name);
    /* The load failed, try to recover */
    if info.is_null() { return -(1 as libc::c_int) }
    /* ** Init the font ***/
    /* Wipe the thing */
    memset(Infofnt as *mut libc::c_char as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<infofnt>() as libc::c_ulong);
    /* Attempt to prepare it */
    if Infofnt_prepare(info) != 0 {
        /* Free the font */
        XFreeFont((*Metadpy).dpy, info);
        /* Fail */
        return -(1 as libc::c_int)
    }
    /* Save a copy of the font name */
    (*Infofnt).name = string_make(name);
    /* Mark it as nukable */
    (*Infofnt).set_nuke(1 as libc::c_int as uint_hack);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Standard Text
 */
unsafe extern "C" fn Infofnt_text_std(mut x: libc::c_int, mut y: libc::c_int,
                                      mut str: cptr, mut len: libc::c_int)
 -> errr {
    let mut i: libc::c_int = 0;
    /* ** Do a brief info analysis ***/
    /* Do nothing if the string is null */
    if str.is_null() || *str == 0 { return -(1 as libc::c_int) }
    /* Get the length of the string */
    if len < 0 as libc::c_int { len = strlen(str) as libc::c_int }
    /* ** Decide where to place the string, vertically ***/
    /* Ignore Vertical Justifications */
    y =
        y * (*Infofnt).hgt as libc::c_int + (*Infofnt).asc as libc::c_int +
            (*Infowin).oy as libc::c_int;
    /* ** Decide where to place the string, horizontally ***/
    /* Line up with x at left edge of column 'x' */
    x = x * (*Infofnt).wid as libc::c_int + (*Infowin).ox as libc::c_int;
    /* ** Actually draw 'str' onto the infowin ***/
    /* Be sure the correct font is ready */
    XSetFont((*Metadpy).dpy, (*Infoclr).gc, (*(*Infofnt).info).fid);
    /* ** Handle the fake mono we can enforce on fonts ***/
    /* Monotize the font */
    if (*Infofnt).mono() != 0 {
        /* Do each character */
        i = 0 as libc::c_int;
        while i < len {
            /* Note that the Infoclr is set up to contain the Infofnt */
            XDrawImageString((*Metadpy).dpy, (*Infowin).win, (*Infoclr).gc,
                             x + i * (*Infofnt).wid as libc::c_int +
                                 (*Infofnt).off as libc::c_int, y,
                             str.offset(i as isize), 1 as libc::c_int);
            i += 1
        }
    } else {
        /* Assume monoospaced font */
        /* Note that the Infoclr is set up to contain the Infofnt */
        XDrawImageString((*Metadpy).dpy, (*Infowin).win, (*Infoclr).gc, x, y,
                         str, len);
    }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Painting where text would be
 */
unsafe extern "C" fn Infofnt_text_non(mut x: libc::c_int, mut y: libc::c_int,
                                      mut str: cptr, mut len: libc::c_int)
 -> errr {
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    /* ** Find the width ***/
    /* Negative length is a flag to count the characters in str */
    if len < 0 as libc::c_int { len = strlen(str) as libc::c_int }
    /* The total width will be 'len' chars * standard width */
    w = len * (*Infofnt).wid as libc::c_int;
    /* ** Find the X dimensions ***/
    /* Line up with x at left edge of column 'x' */
    x = x * (*Infofnt).wid as libc::c_int + (*Infowin).ox as libc::c_int;
    /* ** Find other dimensions ***/
    /* Simply do 'Infofnt->hgt' (a single row) high */
    h = (*Infofnt).hgt as libc::c_int;
    /* Simply do "at top" in row 'y' */
    y = y * h + (*Infowin).oy as libc::c_int;
    /* ** Actually 'paint' the area ***/
    /* Just do a Fill Rectangle */
    XFillRectangle((*Metadpy).dpy, (*Infowin).win, (*Infoclr).gc, x, y,
                   w as libc::c_uint, h as libc::c_uint);
    /* Success */
    return 0 as libc::c_int;
}
/* ************************************************************************/
/*
 * Angband specific code follows... (ANGBAND)
 */
/*
 * Hack -- cursor color
 */
static mut xor: *mut infoclr = 0 as *const infoclr as *mut infoclr;
/*
 * Actual color table
 */
static mut clr: [*mut infoclr; 256] =
    [0 as *const infoclr as *mut infoclr; 256];
/*
 * Color info (unused, red, green, blue).
 */
static mut color_table: [[byte_hack; 4]; 256] = [[0; 4]; 256];
/*
 * The array of term data structures
 */
static mut data: [term_data; 8] =
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
               fnt: 0 as *const infofnt as *mut infofnt,
               win: 0 as *const infowin as *mut infowin,
               tiles: 0 as *const XImage as *mut XImage,
               TmpImage: 0 as *const XImage as *mut XImage,}; 8];
/* The time at which the selection was finalised. */
static mut s_ptr: [x11_selection_type; 1] =
    [x11_selection_type{select: 0,
                        drawn: 0,
                        t: 0 as *const term as *mut term,
                        init: co_ord{x: 0, y: 0,},
                        cur: co_ord{x: 0, y: 0,},
                        old: co_ord{x: 0, y: 0,},
                        time: 0,}; 1];
/*
 * Process a keypress event
 *
 * Also appears in "main-xaw.c".
 */
unsafe extern "C" fn react_keypress(mut xev: *mut XKeyEvent) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut mc: libc::c_int = 0;
    let mut ms: libc::c_int = 0;
    let mut mo: libc::c_int = 0;
    let mut mx: libc::c_int = 0;
    let mut ks1: uint_hack = 0;
    let mut ev: *mut XKeyEvent = xev;
    let mut ks: KeySym = 0;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut msg: [libc::c_char; 128] = [0; 128];
    /* Check for "normal" keypresses */
    n =
        XLookupString(ev, buf.as_mut_ptr(), 125 as libc::c_int, &mut ks,
                      0 as *mut XComposeStatus);
    /* Terminate */
    buf[n as usize] = '\u{0}' as i32 as libc::c_char;
    /* Hack -- Ignore "modifier keys" */
    if ks >= 0xffe1 as libc::c_int as libc::c_ulong &&
           ks <= 0xffee as libc::c_int as libc::c_ulong ||
           ks >= 0xfe01 as libc::c_int as libc::c_ulong &&
               ks <= 0xfe13 as libc::c_int as libc::c_ulong ||
           ks == 0xff7e as libc::c_int as libc::c_ulong ||
           ks == 0xff7f as libc::c_int as libc::c_ulong {
        return
    }
    /* Hack -- convert into an unsigned int */
    ks1 = ks as uint_hack;
    /* Extract four "modifier flags" */
    mc =
        if (*ev).state &
               ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    ms =
        if (*ev).state &
               ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    mo =
        if (*ev).state &
               ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    mx =
        if (*ev).state &
               ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    /* Normal keys with no modifiers */
    if n != 0 && mo == 0 && mx == 0 &&
           !(ks as libc::c_uint >= 0xff00 as libc::c_int as libc::c_uint) {
        /* Enqueue the normal key(s) */
        i = 0 as libc::c_int;
        while buf[i as usize] != 0 {
            Term_keypress(buf[i as usize] as libc::c_int);
            i += 1
        }
        /* All done */
        return
    }
    /* Handle a few standard keys (bypass modifiers) XXX XXX XXX */
    match ks1 {
        65307 => { Term_keypress('\u{1b}' as i32); return }
        65293 => { Term_keypress('\r' as i32); return }
        65289 => { Term_keypress('\t' as i32); return }
        65535 | 65288 => { Term_keypress('\u{8}' as i32); return }
        _ => { }
    }
    /* Hack -- Use the KeySym */
    if ks != 0 {
        sprintf(msg.as_mut_ptr(),
                b"%c%s%s%s%s_%lX%c\x00" as *const u8 as *const libc::c_char,
                31 as libc::c_int,
                if mc != 0 {
                    b"N\x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char },
                if ms != 0 {
                    b"S\x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char },
                if mo != 0 {
                    b"O\x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char },
                if mx != 0 {
                    b"M\x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char }, ks,
                13 as libc::c_int);
    } else {
        /* Hack -- Use the Keycode */
        sprintf(msg.as_mut_ptr(),
                b"%c%s%s%s%sK_%X%c\x00" as *const u8 as *const libc::c_char,
                31 as libc::c_int,
                if mc != 0 {
                    b"N\x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char },
                if ms != 0 {
                    b"S\x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char },
                if mo != 0 {
                    b"O\x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char },
                if mx != 0 {
                    b"M\x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char },
                (*ev).keycode, 13 as libc::c_int);
    }
    /* Enqueue the "macro trigger" string */
    i = 0 as libc::c_int;
    while msg[i as usize] != 0 {
        Term_keypress(msg[i as usize] as libc::c_int);
        i += 1
    }
    /* Hack -- auto-define macros as needed */
    if n != 0 && macro_find_exact(msg.as_mut_ptr() as cptr) < 0 as libc::c_int
       {
        /* Create a macro */
        macro_add(msg.as_mut_ptr() as cptr, buf.as_mut_ptr() as cptr);
    };
}
/*
 * Find the square a particular pixel is part of.
 */
unsafe extern "C" fn pixel_to_square(x: *mut libc::c_int, y: *mut libc::c_int,
                                     ox: libc::c_int, oy: libc::c_int) {
    *x = (ox - (*Infowin).ox as libc::c_int) / (*Infofnt).wid as libc::c_int;
    *y = (oy - (*Infowin).oy as libc::c_int) / (*Infofnt).hgt as libc::c_int;
}
/*
 * Find the pixel at the top-left corner of a square.
 */
unsafe extern "C" fn square_to_pixel(x: *mut libc::c_int, y: *mut libc::c_int,
                                     ox: libc::c_int, oy: libc::c_int) {
    *x = ox * (*Infofnt).wid as libc::c_int + (*Infowin).ox as libc::c_int;
    *y = oy * (*Infofnt).hgt as libc::c_int + (*Infowin).oy as libc::c_int;
}
/*
 * Convert co-ordinates from starting corner/opposite corner to minimum/maximum.
 */
unsafe extern "C" fn sort_co_ord(mut min: *mut co_ord, mut max: *mut co_ord,
                                 mut b: *const co_ord, mut a: *const co_ord) {
    (*min).x = if (*a).x > (*b).x { (*b).x } else { (*a).x };
    (*min).y = if (*a).y > (*b).y { (*b).y } else { (*a).y };
    (*max).x = if (*a).x < (*b).x { (*b).x } else { (*a).x };
    (*max).y = if (*a).y < (*b).y { (*b).y } else { (*a).y };
}
/*
 * Remove the selection by redrawing it.
 */
unsafe extern "C" fn mark_selection_clear(mut x1: libc::c_int,
                                          mut y1: libc::c_int,
                                          mut x2: libc::c_int,
                                          mut y2: libc::c_int) {
    Term_redraw_section(x1, y1, x2, y2);
}
/*
 * Select an area by drawing a grey box around it.
 * NB. These two functions can cause flicker as the selection is modified,
 * as the game redraws the entire marked section.
 */
unsafe extern "C" fn mark_selection_mark(mut x1: libc::c_int,
                                         mut y1: libc::c_int,
                                         mut x2: libc::c_int,
                                         mut y2: libc::c_int) {
    square_to_pixel(&mut x1, &mut y1, x1, y1);
    square_to_pixel(&mut x2, &mut y2, x2, y2);
    XDrawRectangle((*Metadpy).dpy, (*Infowin).win,
                   (*clr[2 as libc::c_int as usize]).gc, x1, y1,
                   (x2 - x1 + (*Infofnt).wid as libc::c_int -
                        1 as libc::c_int) as libc::c_uint,
                   (y2 - y1 + (*Infofnt).hgt as libc::c_int -
                        1 as libc::c_int) as libc::c_uint);
}
/*
 * Mark a selection by drawing boxes around it (for now).
 */
unsafe extern "C" fn mark_selection() {
    let mut min: co_ord = co_ord{x: 0, y: 0,};
    let mut max: co_ord = co_ord{x: 0, y: 0,};
    let mut old: *mut term = Term;
    let mut draw: bool_ = (*s_ptr.as_mut_ptr()).select;
    let mut clear: bool_ = (*s_ptr.as_mut_ptr()).drawn;
    /* Open the correct term if necessary. */
    if (*s_ptr.as_mut_ptr()).t != old {
        Term_activate((*s_ptr.as_mut_ptr()).t);
    }
    if clear != 0 {
        sort_co_ord(&mut min, &mut max, &mut (*s_ptr.as_mut_ptr()).init,
                    &mut (*s_ptr.as_mut_ptr()).old);
        mark_selection_clear(min.x, min.y, max.x, max.y);
    }
    if draw != 0 {
        sort_co_ord(&mut min, &mut max, &mut (*s_ptr.as_mut_ptr()).init,
                    &mut (*s_ptr.as_mut_ptr()).cur);
        mark_selection_mark(min.x, min.y, max.x, max.y);
    }
    /* Finish on the current term. */
    if (*s_ptr.as_mut_ptr()).t != old { Term_activate(old); }
    (*s_ptr.as_mut_ptr()).old.x = (*s_ptr.as_mut_ptr()).cur.x;
    (*s_ptr.as_mut_ptr()).old.y = (*s_ptr.as_mut_ptr()).cur.y;
    (*s_ptr.as_mut_ptr()).drawn = (*s_ptr.as_mut_ptr()).select;
}
/*
 * Forget a selection for one reason or another.
 */
unsafe extern "C" fn copy_x11_release() {
    /* Deselect the current selection. */
    (*s_ptr.as_mut_ptr()).select = 0 as libc::c_int as bool_;
    /* Remove its graphical represesntation. */
    mark_selection();
}
/*
 * Start to select some text on the screen.
 */
unsafe extern "C" fn copy_x11_start(mut x: libc::c_int, mut y: libc::c_int) {
    if (*s_ptr.as_mut_ptr()).select != 0 { copy_x11_release(); }
    /* Remember where the selection started. */
    let ref mut fresh6 = (*s_ptr.as_mut_ptr()).t;
    *fresh6 = Term;
    let ref mut fresh7 = (*s_ptr.as_mut_ptr()).old.x;
    *fresh7 = x;
    let ref mut fresh8 = (*s_ptr.as_mut_ptr()).cur.x;
    *fresh8 = *fresh7;
    (*s_ptr.as_mut_ptr()).init.x = *fresh8;
    let ref mut fresh9 = (*s_ptr.as_mut_ptr()).old.y;
    *fresh9 = y;
    let ref mut fresh10 = (*s_ptr.as_mut_ptr()).cur.y;
    *fresh10 = *fresh9;
    (*s_ptr.as_mut_ptr()).init.y = *fresh10;
}
/*
 * Respond to movement of the mouse when selecting text.
 */
unsafe extern "C" fn copy_x11_cont(mut x: libc::c_int, mut y: libc::c_int,
                                   mut buttons: libc::c_uint) {
    /* Use the nearest square within bounds if the mouse is outside. */
    x =
        if (if x < 0 as libc::c_int { 0 as libc::c_int } else { x }) >
               (*Term).wid as libc::c_int - 1 as libc::c_int {
            ((*Term).wid as libc::c_int) - 1 as libc::c_int
        } else if x < 0 as libc::c_int { 0 as libc::c_int } else { x };
    y =
        if (if y < 0 as libc::c_int { 0 as libc::c_int } else { y }) >
               (*Term).hgt as libc::c_int - 1 as libc::c_int {
            ((*Term).hgt as libc::c_int) - 1 as libc::c_int
        } else if y < 0 as libc::c_int { 0 as libc::c_int } else { y };
    /* The left mouse button isn't pressed. */
    if !buttons & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint !=
           0 {
        return
    }
    /* Not a selection in this window. */
    if (*s_ptr.as_mut_ptr()).t != Term { return }
    /* Not enough movement. */
    if x == (*s_ptr.as_mut_ptr()).old.x && y == (*s_ptr.as_mut_ptr()).old.y &&
           (*s_ptr.as_mut_ptr()).select as libc::c_int != 0 {
        return
    }
    /* Something is being selected. */
    (*s_ptr.as_mut_ptr()).select = 1 as libc::c_int as bool_;
    /* Track the selection. */
    (*s_ptr.as_mut_ptr()).cur.x = x;
    (*s_ptr.as_mut_ptr()).cur.y = y;
    /* Hack - display it inefficiently. */
    mark_selection();
}
/*
 * Respond to release of the left mouse button by putting the selected text in
 * the primary buffer.
 */
unsafe extern "C" fn copy_x11_end(time: Time) {
    /* No selection. */
    if (*s_ptr.as_mut_ptr()).select == 0 { return }
    /* Not a selection in this window. */
    if (*s_ptr.as_mut_ptr()).t != Term { return }
    /* Remember when the selection was finalised. */
    (*s_ptr.as_mut_ptr()).time = time;
    /* Acquire the primary selection. */
    XSetSelectionOwner((*Metadpy).dpy, 1 as libc::c_int as Atom,
                       (*Infowin).win, time);
    if XGetSelectionOwner((*Metadpy).dpy, 1 as libc::c_int as Atom) !=
           (*Infowin).win {
        /* Failed to acquire the selection, so forget it. */
        bell();
        (*s_ptr.as_mut_ptr()).select = 0 as libc::c_int as bool_;
        mark_selection();
    };
}
/*
 * Send a message to request that the PRIMARY buffer be sent here.
 */
unsafe extern "C" fn paste_x11_request(time: Time) {
    let mut event: [XEvent; 1] = [_XEvent{type_0: 0,}; 1];
    let mut ptr: *mut XSelectionRequestEvent =
        &mut (*event.as_mut_ptr()).xselectionrequest;
    /* Set various things. */
    (*ptr).type_0 = 30 as libc::c_int; /* Unused */
    (*ptr).display = (*Metadpy).dpy;
    (*ptr).owner =
        XGetSelectionOwner((*Metadpy).dpy, 1 as libc::c_int as Atom);
    (*ptr).requestor = (*Infowin).win;
    (*ptr).selection = 1 as libc::c_int as Atom;
    (*ptr).target = 31 as libc::c_int as Atom;
    (*ptr).property = 31 as libc::c_int as Atom;
    (*ptr).time = time;
    /* Check the owner. */
    if (*ptr).owner == 0 as libc::c_long as libc::c_ulong {
        /* No selection. */
        bell();
        return
    }
    /* Send the SelectionRequest event. */
    XSendEvent((*Metadpy).dpy, (*ptr).owner, 0 as libc::c_int,
               0 as libc::c_long, event.as_mut_ptr());
}
/*
 * Add a character to a string in preparation for sending it to another
 * client as a STRING.
 * This doesn't change anything, as clients tend not to have difficulty in
 * receiving this format (although the standard specifies a restricted set).
 * Strings do not have a colour.
 */
unsafe extern "C" fn add_char_string(mut buf: *mut libc::c_char,
                                     mut a: byte_hack, mut c: libc::c_char)
 -> libc::c_int {
    *buf = c;
    return 1 as libc::c_int;
}
/*
 * Send some text requested by another X client.
 */
unsafe extern "C" fn paste_x11_send(mut rq: *mut XSelectionRequestEvent) {
    let mut current_block: u64;
    let mut event: XEvent = _XEvent{type_0: 0,};
    let mut ptr: *mut XSelectionEvent = &mut event.xselection;
    let mut add:
            Option<unsafe extern "C" fn(_: *mut libc::c_char, _: byte_hack,
                                        _: libc::c_char) -> libc::c_int> =
        None;
    /* Set the event parameters. */
    (*ptr).type_0 = 31 as libc::c_int;
    (*ptr).property = (*rq).property;
    (*ptr).display = (*rq).display;
    (*ptr).requestor = (*rq).requestor;
    (*ptr).selection = (*rq).selection;
    (*ptr).target = (*rq).target;
    (*ptr).time = (*rq).time;
    /* Determine the correct "add a character" function.
	 * As Term->wid is at most 255, these can add up to 4 characters of
	 * output per character of input without problem.
	 * The mechanism will need to change if much more than this is needed.
	 */
    match (*rq).target {
        31 => {
            add =
                Some(add_char_string as
                         unsafe extern "C" fn(_: *mut libc::c_char,
                                              _: byte_hack, _: libc::c_char)
                             -> libc::c_int);
            /* Reply to a known target received recently with data. */
            if (*rq).time >= (*s_ptr.as_mut_ptr()).time && add.is_some() {
                let mut buf: [libc::c_char; 1024] = [0; 1024];
                let mut max: co_ord = co_ord{x: 0, y: 0,};
                let mut min: co_ord = co_ord{x: 0, y: 0,};
                let mut x: libc::c_int = 0;
                let mut y: libc::c_int = 0;
                let mut l: libc::c_int = 0;
                let mut a: byte_hack = 0;
                let mut c: libc::c_char = 0;
                /* Work out which way around to paste. */
                sort_co_ord(&mut min, &mut max,
                            &mut (*s_ptr.as_mut_ptr()).init,
                            &mut (*s_ptr.as_mut_ptr()).cur);
                /* Paranoia. */
                if XGetSelectionOwner((*Metadpy).dpy,
                                      1 as libc::c_int as Atom) !=
                       (*Infowin).win {
                    bell();
                    current_block = 6952046617971891259;
                } else {
                    /* Delete the old value of the property. */
                    XDeleteProperty((*Metadpy).dpy, (*rq).requestor,
                                    (*rq).property);
                    y = 0 as libc::c_int;
                    while y < (*Term).hgt as libc::c_int {
                        if !(y < min.y) {
                            if y > max.y { break ; }
                            l = 0 as libc::c_int;
                            x = l;
                            while x < (*Term).wid as libc::c_int {
                                if !(x < min.x) {
                                    if x > max.x { break ; }
                                    /* Find the character. */
                                    Term_what(x, y, &mut a, &mut c);
                                    /* Add it. */
                                    l +=
                                        Some(add.expect("non-null function pointer")).expect("non-null function pointer")(buf.as_mut_ptr().offset(l
                                                                                                                                                      as
                                                                                                                                                      isize),
                                                                                                                          a,
                                                                                                                          c)
                                }
                                x += 1
                            }
                            /* Terminate all but the last line in an appropriate way. */
                            if y != max.y {
                                l +=
                                    Some(add.expect("non-null function pointer")).expect("non-null function pointer")(buf.as_mut_ptr().offset(l
                                                                                                                                                  as
                                                                                                                                                  isize),
                                                                                                                      1
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          byte_hack,
                                                                                                                      '\n'
                                                                                                                          as
                                                                                                                          i32
                                                                                                                          as
                                                                                                                          libc::c_char)
                            }
                            /* Send the (non-empty) string. */
                            XChangeProperty((*Metadpy).dpy, (*rq).requestor,
                                            (*rq).property, (*rq).target,
                                            8 as libc::c_int,
                                            2 as libc::c_int,
                                            buf.as_mut_ptr() as
                                                *mut libc::c_uchar, l);
                        }
                        y += 1
                    }
                    current_block = 6450636197030046351;
                }
            } else { current_block = 6952046617971891259; }
        }
        _ => { current_block = 6952046617971891259; }
    }
    match current_block {
        6952046617971891259 =>
        /* Respond to all bad requests with property None. */
        {
            (*ptr).property = 0 as libc::c_long as Atom
        }
        _ => { }
    }
    /* Send whatever event we're left with. */
    XSendEvent((*Metadpy).dpy, (*rq).requestor, 0 as libc::c_int,
               0 as libc::c_long, &mut event);
}
/*
 * Add the contents of the PRIMARY buffer to the input queue.
 *
 * Hack - This doesn't use the "time" of the event, and so accepts anything a
 * client tries to send it.
 */
unsafe extern "C" fn paste_x11_accept(mut ptr: *const XSelectionEvent) {
    let mut offset: libc::c_long = 0;
    let mut left: libc::c_ulong = 0;
    /* Failure. */
    if (*ptr).property == 0 as libc::c_long as libc::c_ulong {
        bell();
        return
    }
    if (*ptr).selection != 1 as libc::c_int as Atom { bell(); return }
    if (*ptr).target != 31 as libc::c_int as Atom { bell(); return }
    offset = 0 as libc::c_int as libc::c_long;
    loop  {
        let mut err: errr = 0;
        /* A pointer for the pasted information. */
        let mut data_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut type_0: Atom = 0;
        let mut fmt: libc::c_int = 0;
        let mut nitems: libc::c_ulong = 0;
        /* Set data to the string, and catch errors. */
        if XGetWindowProperty((*Metadpy).dpy, (*Infowin).win,
                              31 as libc::c_int as Atom, offset,
                              32767 as libc::c_int as libc::c_long,
                              1 as libc::c_int, 31 as libc::c_int as Atom,
                              &mut type_0, &mut fmt, &mut nitems, &mut left,
                              &mut data_0) != 0 as libc::c_int {
            break ;
        }
        /* Paste the text. */
        err = type_string(data_0 as *mut libc::c_char, nitems as uint_hack);
        /* Free the data pasted. */
        XFree(data_0 as *mut libc::c_void);
        /* No room. */
        if err == 7 as libc::c_int {
            bell();
            break ;
        } else {
            /* Paranoia? - strange errors. */
            if err != 0 { break ; }
            /* Pasted everything. */
            if left == 0 { return }
            offset =
                (offset as libc::c_ulong).wrapping_add(left) as libc::c_long
                    as libc::c_long
        }
    }
    /* An error has occurred, so free the last bit of data before returning. */
    XFree(data.as_mut_ptr() as *mut libc::c_void);
}
/*
 * Handle various events conditional on presses of a mouse button.
 */
unsafe extern "C" fn handle_button(mut time: Time, mut x: libc::c_int,
                                   mut y: libc::c_int,
                                   mut button: libc::c_int,
                                   mut press: bool_) {
    /* The co-ordinates are only used in Angband format. */
    pixel_to_square(&mut x, &mut y, x, y);
    if press as libc::c_int != 0 && button == 1 as libc::c_int {
        copy_x11_start(x, y);
    }
    if press == 0 && button == 1 as libc::c_int { copy_x11_end(time); }
    if press == 0 && button == 2 as libc::c_int { paste_x11_request(time); };
}
/*
 * Process events
 */
unsafe extern "C" fn CheckEvent(mut wait: bool_) -> errr {
    let mut old_td: *mut term_data = (*Term).data as *mut term_data;
    let mut xev_body: XEvent = _XEvent{type_0: 0,};
    let mut xev: *mut XEvent = &mut xev_body;
    let mut td: *mut term_data = 0 as *mut term_data;
    let mut iwin: *mut infowin = 0 as *mut infowin;
    let mut i: libc::c_int = 0;
    /* Do not wait unless requested */
    if wait == 0 && XPending((*Metadpy).dpy) == 0 { return 1 as libc::c_int }
    /* Hack - redraw the selection, if needed.
	* This doesn't actually check that one of its squares was drawn to,
	* only that this may have happened.
	*/
    if (*s_ptr.as_mut_ptr()).select as libc::c_int != 0 &&
           (*s_ptr.as_mut_ptr()).drawn == 0 {
        mark_selection();
    }
    /* Load the Event */
    XNextEvent((*Metadpy).dpy, xev);
    /* Notice new keymaps */
    if (*xev).type_0 == 34 as libc::c_int {
        XRefreshKeyboardMapping(&mut (*xev).xmapping);
        return 0 as libc::c_int
    }
    /* Scan the windows */
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if (*xev).xany.window == (*data[i as usize].win).win {
            td = &mut *data.as_mut_ptr().offset(i as isize) as *mut term_data;
            iwin = (*td).win;
            break ;
        } else { i += 1 }
    }
    /* Unknown window */
    if td.is_null() || iwin.is_null() { return 0 as libc::c_int }
    /* Hack -- activate the Term */
    Term_activate(&mut (*td).t);
    /* Hack -- activate the window */
    Infowin = iwin;
    /* Switch on the Type */
    match (*xev).type_0 {
        4 | 5 => {
            let mut press: bool_ =
                ((*xev).type_0 == 4 as libc::c_int) as libc::c_int as bool_;
            /* Where is the mouse */
            let mut x: libc::c_int = (*xev).xbutton.x;
            let mut y: libc::c_int = (*xev).xbutton.y;
            let mut z: libc::c_int = 0;
            /* Which button is involved */
            if (*xev).xbutton.button == 1 as libc::c_int as libc::c_uint {
                z = 1 as libc::c_int
            } else if (*xev).xbutton.button ==
                          2 as libc::c_int as libc::c_uint {
                z = 2 as libc::c_int
            } else if (*xev).xbutton.button ==
                          3 as libc::c_int as libc::c_uint {
                z = 3 as libc::c_int
            } else if (*xev).xbutton.button ==
                          4 as libc::c_int as libc::c_uint {
                z = 4 as libc::c_int
            } else if (*xev).xbutton.button ==
                          5 as libc::c_int as libc::c_uint {
                z = 5 as libc::c_int
            } else { z = 0 as libc::c_int }
            /* Where is the mouse */
            x = (*xev).xbutton.x;
            y = (*xev).xbutton.y;
            /* XXX Handle */
            handle_button((*xev).xbutton.time, x, y, z, press);
        }
        7 | 8 => { }
        6 => {
            let mut x_0: libc::c_int = (*xev).xmotion.x;
            let mut y_0: libc::c_int = (*xev).xmotion.y;
            let mut z_0: libc::c_uint = (*xev).xmotion.state;
            /* Convert to co-ordinates Angband understands. */
            pixel_to_square(&mut x_0, &mut y_0, x_0, y_0);
            /* Alter the selection if appropriate. */
            copy_x11_cont(x_0, y_0, z_0);
        }
        31 => { paste_x11_accept(&mut (*xev).xselection); }
        30 => { paste_x11_send(&mut (*xev).xselectionrequest); }
        29 => {
            (*s_ptr.as_mut_ptr()).select = 0 as libc::c_int as bool_;
            mark_selection();
        }
        2 => {
            /* Hack -- use "old" term */
            Term_activate(&mut (*old_td).t);
            /* Process the key */
            react_keypress(&mut (*xev).xkey);
        }
        12 => {
            /* Ignore "extra" exposes */
            if !((*xev).xexpose.count != 0) {
                /* Clear the window */
                Infowin_wipe();
                /* Redraw */
                Term_redraw();
            }
        }
        19 => {
            (*Infowin).set_mapped(1 as libc::c_int as uint_hack);
            (*Term).mapped_flag = 1 as libc::c_int as bool_
        }
        18 => {
            (*Infowin).set_mapped(0 as libc::c_int as uint_hack);
            (*Term).mapped_flag = 0 as libc::c_int as bool_
        }
        22 => {
            /* Move and/or Resize */
            let mut cols: libc::c_int = 0;
            let mut rows: libc::c_int = 0;
            let mut ox: libc::c_int = (*Infowin).ox as libc::c_int;
            let mut oy: libc::c_int = (*Infowin).oy as libc::c_int;
            /* Save the new Window Parms */
            (*Infowin).x = (*xev).xconfigure.x as s16b;
            (*Infowin).y = (*xev).xconfigure.y as s16b;
            (*Infowin).w = (*xev).xconfigure.width as s16b;
            (*Infowin).h = (*xev).xconfigure.height as s16b;
            /* Determine "proper" number of rows/cols */
            cols =
                ((*Infowin).w as libc::c_int - (ox + ox)) /
                    (*(*td).fnt).wid as libc::c_int;
            rows =
                ((*Infowin).h as libc::c_int - (oy + oy)) /
                    (*(*td).fnt).hgt as libc::c_int;
            /* Hack -- minimal size */
            if td ==
                   &mut *data.as_mut_ptr().offset(0 as libc::c_int as isize)
                       as *mut term_data {
                if cols < 80 as libc::c_int { cols = 80 as libc::c_int }
                if rows < 24 as libc::c_int { rows = 24 as libc::c_int }
            } else {
                if cols < 1 as libc::c_int { cols = 1 as libc::c_int }
                if rows < 1 as libc::c_int { rows = 1 as libc::c_int }
            }
            /* Paranoia */
            if cols > 255 as libc::c_int { cols = 255 as libc::c_int }
            if rows > 255 as libc::c_int { rows = 255 as libc::c_int }
            /* Resize the Term (if needed) */
            Term_resize(cols, rows);
        }
        3 | _ => { }
    }
    /* Hack -- Activate the old term */
    Term_activate(&mut (*old_td).t);
    /* Hack -- Activate the proper window */
    Infowin = (*old_td).win;
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Handle "activation" of a term
 */
unsafe extern "C" fn Term_xtra_x11_level(mut v: libc::c_int) -> errr {
    let mut td: *mut term_data = (*Term).data as *mut term_data;
    /* Handle "activate" */
    if v != 0 {
        /* Activate the window */
        Infowin = (*td).win;
        /* Activate the font */
        Infofnt = (*td).fnt
    }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * React to changes
 */
unsafe extern "C" fn Term_xtra_x11_react() -> errr {
    let mut i: libc::c_int = 0;
    if (*Metadpy).color() != 0 {
        /* Check the colors */
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            if color_table[i as usize][0 as libc::c_int as usize] as
                   libc::c_int !=
                   angband_color_table[i as usize][0 as libc::c_int as usize]
                       as libc::c_int ||
                   color_table[i as usize][1 as libc::c_int as usize] as
                       libc::c_int !=
                       angband_color_table[i as
                                               usize][1 as libc::c_int as
                                                          usize] as
                           libc::c_int ||
                   color_table[i as usize][2 as libc::c_int as usize] as
                       libc::c_int !=
                       angband_color_table[i as
                                               usize][2 as libc::c_int as
                                                          usize] as
                           libc::c_int ||
                   color_table[i as usize][3 as libc::c_int as usize] as
                       libc::c_int !=
                       angband_color_table[i as
                                               usize][3 as libc::c_int as
                                                          usize] as
                           libc::c_int {
                let mut pixel: Pixell = 0;
                /* Save new values */
                color_table[i as usize][0 as libc::c_int as usize] =
                    angband_color_table[i as
                                            usize][0 as libc::c_int as usize];
                color_table[i as usize][1 as libc::c_int as usize] =
                    angband_color_table[i as
                                            usize][1 as libc::c_int as usize];
                color_table[i as usize][2 as libc::c_int as usize] =
                    angband_color_table[i as
                                            usize][2 as libc::c_int as usize];
                color_table[i as usize][3 as libc::c_int as usize] =
                    angband_color_table[i as
                                            usize][3 as libc::c_int as usize];
                /* Create pixel */
                pixel =
                    create_pixel((*Metadpy).dpy,
                                 color_table[i as
                                                 usize][1 as libc::c_int as
                                                            usize],
                                 color_table[i as
                                                 usize][2 as libc::c_int as
                                                            usize],
                                 color_table[i as
                                                 usize][3 as libc::c_int as
                                                            usize]);
                /* Change the foreground */
                Infoclr = clr[i as usize];
                Infoclr_change_fg(pixel);
            }
            i += 1
        }
    }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Handle a "special request"
 */
unsafe extern "C" fn Term_xtra_x11(mut n: libc::c_int, mut v: libc::c_int)
 -> errr {
    /* Handle a subset of the legal requests */
    match n {
        7 => {
            /* Make a noise */
            Metadpy_do_beep();
            return 0 as libc::c_int
        }
        6 => {
            /* Flush the output XXX XXX */
            Metadpy_update(1 as libc::c_int, 0 as libc::c_int,
                           0 as libc::c_int);
            return 0 as libc::c_int
        }
        9 => {
            /* Process random events XXX */
            return CheckEvent(0 as libc::c_int as bool_)
        }
        1 => {
            /* Process Events XXX */
            return CheckEvent(v as bool_)
        }
        2 => {
            /* Flush the events XXX */
            while CheckEvent(0 as libc::c_int as bool_) == 0 { }
            return 0 as libc::c_int
        }
        12 => {
            /* Handle change in the "level" */
            return Term_xtra_x11_level(v)
        }
        3 => {
            /* Clear the screen, and redraw any selection later. */
            Infowin_wipe();
            (*s_ptr.as_mut_ptr()).drawn = 0 as libc::c_int as bool_;
            return 0 as libc::c_int
        }
        13 => {
            /* Delay for some milliseconds */
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
                file[(4096 as libc::c_int + 255 as libc::c_int) as usize] =
                    0 as libc::c_int as libc::c_char;
                strncpy(file.as_mut_ptr(), scansubdir_dir.as_mut_ptr(),
                        4096 as libc::c_int as libc::c_ulong);
                strncat(file.as_mut_ptr(),
                        b"/\x00" as *const u8 as *const libc::c_char,
                        2 as libc::c_int as libc::c_ulong);
                strncat(file.as_mut_ptr(), (*entry).d_name.as_mut_ptr(),
                        255 as libc::c_int as libc::c_ulong);
                if stat(file.as_mut_ptr(), &mut filedata) == 0 &&
                       filedata.st_mode &
                           0o170000 as libc::c_int as libc::c_uint ==
                           0o40000 as libc::c_int as libc::c_uint {
                    string_free(scansubdir_result[scansubdir_max as usize]);
                    scansubdir_result[scansubdir_max as usize] =
                        string_make((*entry).d_name.as_mut_ptr() as cptr);
                    scansubdir_max += 1
                }
            }
            closedir(directory);
            return 0 as libc::c_int
        }
        10 => {
            /* React to changes */
            return Term_xtra_x11_react()
        }
        16 => {
            /* Rename main window */
            Infowin_set_name(angband_term_name[0 as libc::c_int as
                                                   usize].as_mut_ptr() as
                                 cptr);
            return 0 as libc::c_int
        }
        _ => { }
    }
    /* Unknown */
    return 1 as libc::c_int;
}
/*
 * Draw the cursor as an inverted rectangle.
 *
 * Consider a rectangular outline like "main-mac.c".  XXX XXX
 */
unsafe extern "C" fn Term_curs_x11(mut x: libc::c_int, mut y: libc::c_int)
 -> errr {
    /* Draw the cursor */
    Infoclr = xor;
    if use_bigtile as libc::c_int != 0 &&
           (x + 1 as libc::c_int) < (*Term).wid as libc::c_int &&
           *(*(*(*Term).old).a.offset(y as
                                          isize)).offset((x +
                                                              1 as
                                                                  libc::c_int)
                                                             as isize) as
               libc::c_int == 255 as libc::c_int {
        Infofnt_text_non(x, y, b"  \x00" as *const u8 as *const libc::c_char,
                         2 as libc::c_int);
    } else {
        /* Hilite the cursor character */
        Infofnt_text_non(x, y, b" \x00" as *const u8 as *const libc::c_char,
                         1 as libc::c_int);
    }
    /* Redraw the selection if any, as it may have been obscured. (later) */
    (*s_ptr.as_mut_ptr()).drawn = 0 as libc::c_int as bool_;
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Erase some characters.
 */
unsafe extern "C" fn Term_wipe_x11(mut x: libc::c_int, mut y: libc::c_int,
                                   mut n: libc::c_int) -> errr {
    /* Erase (use black) */
    Infoclr = clr[0 as libc::c_int as usize];
    /* Mega-Hack -- Erase some space */
    Infofnt_text_non(x, y, b"\x00" as *const u8 as *const libc::c_char, n);
    /* Redraw the selection if any, as it may have been obscured. (later) */
    (*s_ptr.as_mut_ptr()).drawn = 0 as libc::c_int as bool_;
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Draw some textual characters.
 */
unsafe extern "C" fn Term_text_x11(mut x: libc::c_int, mut y: libc::c_int,
                                   mut n: libc::c_int, mut a: byte_hack,
                                   mut s: cptr) -> errr {
    /* Draw the text */
    Infoclr = clr[a as usize];
    /* Draw the text */
    Infofnt_text_std(x, y, s, n);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Draw some graphical characters.
 */
unsafe extern "C" fn Term_pict_x11(mut x: libc::c_int, mut y: libc::c_int,
                                   mut n: libc::c_int,
                                   mut ap: *const byte_hack,
                                   mut cp: *const libc::c_char,
                                   mut tap: *const byte_hack,
                                   mut tcp: *const libc::c_char,
                                   mut eap: *const byte_hack,
                                   mut ecp: *const libc::c_char) -> errr {
    let mut i: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut a: byte_hack = 0;
    let mut c: libc::c_char = 0;
    let mut ta: byte_hack = 0;
    let mut tc: libc::c_char = 0;
    let mut x2: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut ea: byte_hack = 0;
    let mut ec: libc::c_char = 0;
    let mut x3: libc::c_int = 0;
    let mut y3: libc::c_int = 0;
    let mut has_overlay: bool_ = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut pixel: libc::c_ulong = 0;
    let mut blank: libc::c_ulong = 0;
    let mut td: *mut term_data = (*Term).data as *mut term_data;
    y *= (*Infofnt).hgt as libc::c_int;
    x *= (*Infofnt).wid as libc::c_int;
    /* Add in affect of window boundaries */
    y += (*Infowin).oy as libc::c_int;
    x += (*Infowin).ox as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        let fresh11 = ap;
        ap = ap.offset(1);
        a = *fresh11;
        let fresh12 = cp;
        cp = cp.offset(1);
        c = *fresh12;
        /* For extra speed - cache these values */
        x1 =
            (c as libc::c_int & 0x7f as libc::c_int) *
                (*(*td).fnt).twid as libc::c_int;
        y1 =
            (a as libc::c_int & 0x7f as libc::c_int) *
                (*(*td).fnt).hgt as libc::c_int;
        let fresh13 = tap;
        tap = tap.offset(1);
        ta = *fresh13;
        let fresh14 = tcp;
        tcp = tcp.offset(1);
        tc = *fresh14;
        /* For extra speed - cache these values */
        x2 =
            (tc as libc::c_int & 0x7f as libc::c_int) *
                (*(*td).fnt).twid as libc::c_int;
        y2 =
            (ta as libc::c_int & 0x7f as libc::c_int) *
                (*(*td).fnt).hgt as libc::c_int;
        let fresh15 = eap;
        eap = eap.offset(1);
        ea = *fresh15;
        let fresh16 = ecp;
        ecp = ecp.offset(1);
        ec = *fresh16;
        has_overlay =
            (ea as libc::c_int != 0 && ec as libc::c_int != 0) as libc::c_int
                as bool_;
        /* For extra speed - cache these values too */
        x3 =
            (ec as libc::c_int & 0x7f as libc::c_int) *
                (*(*td).fnt).twid as libc::c_int;
        y3 =
            (ea as libc::c_int & 0x7f as libc::c_int) *
                (*(*td).fnt).hgt as libc::c_int;
        /* Optimise the common case */
        if x1 == x2 && y1 == y2 {
            /* Draw object / terrain */
            if has_overlay == 0 {
                XPutImage((*Metadpy).dpy, (*(*td).win).win,
                          (*clr[0 as libc::c_int as usize]).gc, (*td).tiles,
                          x1, y1, x, y, (*(*td).fnt).twid as libc::c_uint,
                          (*(*td).fnt).hgt as libc::c_uint);
            } else {
                /* There's a terrain overlay */
                /* Mega Hack^2 - assume the top left corner is "black" */
                blank =
                    Some((*(*td).tiles).f.get_pixel.expect("non-null function pointer")).expect("non-null function pointer")((*td).tiles,
                                                                                                                             0
                                                                                                                                 as
                                                                                                                                 libc::c_int,
                                                                                                                             (*(*td).fnt).hgt
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 *
                                                                                                                                 6
                                                                                                                                     as
                                                                                                                                     libc::c_int);
                k = 0 as libc::c_int;
                while k < (*(*td).fnt).twid as libc::c_int {
                    l = 0 as libc::c_int;
                    while l < (*(*td).fnt).hgt as libc::c_int {
                        /* If mask set in overlay... */
                        pixel =
                            Some((*(*td).tiles).f.get_pixel.expect("non-null function pointer")).expect("non-null function pointer")((*td).tiles,
                                                                                                                                     x3
                                                                                                                                         +
                                                                                                                                         k,
                                                                                                                                     y3
                                                                                                                                         +
                                                                                                                                         l);
                        if pixel == blank {
                            /* Output from the terrain */
                            pixel =
                                Some((*(*td).tiles).f.get_pixel.expect("non-null function pointer")).expect("non-null function pointer")((*td).tiles,
                                                                                                                                         x1
                                                                                                                                             +
                                                                                                                                             k,
                                                                                                                                         y1
                                                                                                                                             +
                                                                                                                                             l)
                        }
                        /* Store into the temp storage. */
                        Some((*(*td).TmpImage).f.put_pixel.expect("non-null function pointer")).expect("non-null function pointer")((*td).TmpImage,
                                                                                                                                    k,
                                                                                                                                    l,
                                                                                                                                    pixel);
                        l += 1
                    }
                    k += 1
                }
                XPutImage((*Metadpy).dpy, (*(*td).win).win,
                          (*clr[0 as libc::c_int as usize]).gc,
                          (*td).TmpImage, 0 as libc::c_int, 0 as libc::c_int,
                          x, y, (*(*td).fnt).twid as libc::c_uint,
                          (*(*td).fnt).hgt as libc::c_uint);
            }
        } else {
            /* Draw to screen */
            /* Mega Hack^2 - assume the top left corner is "black" */
            blank =
                Some((*(*td).tiles).f.get_pixel.expect("non-null function pointer")).expect("non-null function pointer")((*td).tiles,
                                                                                                                         0
                                                                                                                             as
                                                                                                                             libc::c_int,
                                                                                                                         (*(*td).fnt).hgt
                                                                                                                             as
                                                                                                                             libc::c_int
                                                                                                                             *
                                                                                                                             6
                                                                                                                                 as
                                                                                                                                 libc::c_int);
            k = 0 as libc::c_int;
            while k < (*(*td).fnt).twid as libc::c_int {
                l = 0 as libc::c_int;
                while l < (*(*td).fnt).hgt as libc::c_int {
                    /* Overlay */
                    if has_overlay != 0 {
                        pixel =
                            Some((*(*td).tiles).f.get_pixel.expect("non-null function pointer")).expect("non-null function pointer")((*td).tiles,
                                                                                                                                     x3
                                                                                                                                         +
                                                                                                                                         k,
                                                                                                                                     y3
                                                                                                                                         +
                                                                                                                                         l)
                    } else {
                        /* Hack -- No overlay */
                        pixel = blank
                    }
                    /* If it's blank... */
                    if pixel == blank {
                        /* Look at mon/obj */
                        pixel =
                            Some((*(*td).tiles).f.get_pixel.expect("non-null function pointer")).expect("non-null function pointer")((*td).tiles,
                                                                                                                                     x1
                                                                                                                                         +
                                                                                                                                         k,
                                                                                                                                     y1
                                                                                                                                         +
                                                                                                                                         l)
                    }
                    /* If it's blank too, use terrain */
                    if pixel == blank {
                        pixel =
                            Some((*(*td).tiles).f.get_pixel.expect("non-null function pointer")).expect("non-null function pointer")((*td).tiles,
                                                                                                                                     x2
                                                                                                                                         +
                                                                                                                                         k,
                                                                                                                                     y2
                                                                                                                                         +
                                                                                                                                         l)
                    }
                    /* Store into the temp storage. */
                    Some((*(*td).TmpImage).f.put_pixel.expect("non-null function pointer")).expect("non-null function pointer")((*td).TmpImage,
                                                                                                                                k,
                                                                                                                                l,
                                                                                                                                pixel);
                    l += 1
                }
                k += 1
            }
            /* Draw to screen */
            XPutImage((*Metadpy).dpy, (*(*td).win).win,
                      (*clr[0 as libc::c_int as usize]).gc, (*td).TmpImage,
                      0 as libc::c_int, 0 as libc::c_int, x, y,
                      (*(*td).fnt).twid as libc::c_uint,
                      (*(*td).fnt).hgt as libc::c_uint);
        }
        x += (*(*td).fnt).wid as libc::c_int;
        i += 1;
        x += (*(*td).fnt).wid as libc::c_int
    }
    /* Redraw the selection if any, as it may have been obscured. (later) */
    (*s_ptr.as_mut_ptr()).drawn = 0 as libc::c_int as bool_;
    /* Success */
    return 0 as libc::c_int;
}
/* USE_GRAPHICS */
/*
 * Initialize a term_data
 */
unsafe extern "C" fn term_data_init(mut td: *mut term_data,
                                    mut i: libc::c_int) -> errr {
    let mut t: *mut term = &mut (*td).t;
    let mut name: cptr = angband_term_name[i as usize].as_mut_ptr() as cptr;
    let mut font: cptr = 0 as *const libc::c_char;
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut cols: libc::c_int = 80 as libc::c_int;
    let mut rows: libc::c_int = 24 as libc::c_int;
    let mut ox: libc::c_int = 1 as libc::c_int;
    let mut oy: libc::c_int = 1 as libc::c_int;
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut str: cptr = 0 as *const libc::c_char;
    let mut val: libc::c_int = 0;
    let mut ch: *mut XClassHint = 0 as *mut XClassHint;
    let mut res_name: [libc::c_char; 20] = [0; 20];
    let mut res_class: [libc::c_char; 20] = [0; 20];
    let mut sh: *mut XSizeHints = 0 as *mut XSizeHints;
    /* Window specific font name */
    sprintf(buf.as_mut_ptr(),
            b"ANGBAND_X11_FONT_%d\x00" as *const u8 as *const libc::c_char,
            i);
    /* Check environment for that font */
    font = getenv(buf.as_mut_ptr()) as cptr;
    /* Check environment for "base" font */
    if font.is_null() {
        font =
            getenv(b"ANGBAND_X11_FONT\x00" as *const u8 as
                       *const libc::c_char) as cptr
    }
    /* No environment variables, use default font */
    if font.is_null() {
        match i {
            0 => { font = b"9x15\x00" as *const u8 as *const libc::c_char }
            1 => { font = b"9x15\x00" as *const u8 as *const libc::c_char }
            2 => { font = b"9x15\x00" as *const u8 as *const libc::c_char }
            3 => { font = b"9x15\x00" as *const u8 as *const libc::c_char }
            4 => { font = b"9x15\x00" as *const u8 as *const libc::c_char }
            5 => { font = b"9x15\x00" as *const u8 as *const libc::c_char }
            6 => { font = b"9x15\x00" as *const u8 as *const libc::c_char }
            7 => { font = b"9x15\x00" as *const u8 as *const libc::c_char }
            _ => { font = b"9x15\x00" as *const u8 as *const libc::c_char }
        }
    }
    /* Window specific location (x) */
    sprintf(buf.as_mut_ptr(),
            b"ANGBAND_X11_AT_X_%d\x00" as *const u8 as *const libc::c_char,
            i);
    str = getenv(buf.as_mut_ptr()) as cptr;
    x = if !str.is_null() { atoi(str) } else { -(1 as libc::c_int) };
    /* Window specific location (y) */
    sprintf(buf.as_mut_ptr(),
            b"ANGBAND_X11_AT_Y_%d\x00" as *const u8 as *const libc::c_char,
            i);
    str = getenv(buf.as_mut_ptr()) as cptr;
    y = if !str.is_null() { atoi(str) } else { -(1 as libc::c_int) };
    /* Window specific cols */
    sprintf(buf.as_mut_ptr(),
            b"ANGBAND_X11_COLS_%d\x00" as *const u8 as *const libc::c_char,
            i);
    str = getenv(buf.as_mut_ptr()) as cptr;
    val = if !str.is_null() { atoi(str) } else { -(1 as libc::c_int) };
    if val > 0 as libc::c_int { cols = val }
    /* Window specific rows */
    sprintf(buf.as_mut_ptr(),
            b"ANGBAND_X11_ROWS_%d\x00" as *const u8 as *const libc::c_char,
            i);
    str = getenv(buf.as_mut_ptr()) as cptr;
    val = if !str.is_null() { atoi(str) } else { -(1 as libc::c_int) };
    if val > 0 as libc::c_int { rows = val }
    /* Window specific inner border offset (ox) */
    sprintf(buf.as_mut_ptr(),
            b"ANGBAND_X11_IBOX_%d\x00" as *const u8 as *const libc::c_char,
            i);
    str = getenv(buf.as_mut_ptr()) as cptr;
    val = if !str.is_null() { atoi(str) } else { -(1 as libc::c_int) };
    if val > 0 as libc::c_int { ox = val }
    /* Window specific inner border offset (oy) */
    sprintf(buf.as_mut_ptr(),
            b"ANGBAND_X11_IBOY_%d\x00" as *const u8 as *const libc::c_char,
            i);
    str = getenv(buf.as_mut_ptr()) as cptr;
    val = if !str.is_null() { atoi(str) } else { -(1 as libc::c_int) };
    if val > 0 as libc::c_int { oy = val }
    /* Prepare the standard font */
    (*td).fnt =
        memset(ralloc(::std::mem::size_of::<infofnt>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<infofnt>() as libc::c_ulong) as
            *mut infofnt;
    Infofnt = (*td).fnt;
    Infofnt_init_data(font);
    /* Hack -- key buffer size */
    num =
        if i == 0 as libc::c_int {
            1024 as libc::c_int
        } else { 16 as libc::c_int };
    /* Assume full size windows */
    wid = cols * (*(*td).fnt).wid as libc::c_int + (ox + ox);
    hgt = rows * (*(*td).fnt).hgt as libc::c_int + (oy + oy);
    /* Create a top-window */
    (*td).win =
        memset(ralloc(::std::mem::size_of::<infowin>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<infowin>() as libc::c_ulong) as
            *mut infowin;
    Infowin = (*td).win;
    Infowin_init_data(0 as libc::c_long as Window, x, y, wid, hgt,
                      0 as libc::c_int, (*Metadpy).fg, (*Metadpy).bg);
    /* Ask for certain events */
    Infowin_set_mask((1 as libc::c_long) << 15 as libc::c_int |
                         (1 as libc::c_long) << 17 as libc::c_int |
                         (1 as libc::c_long) << 0 as libc::c_int |
                         (1 as libc::c_long) << 6 as libc::c_int |
                         (1 as libc::c_long) << 2 as libc::c_int |
                         (1 as libc::c_long) << 3 as libc::c_int);
    /* Set the window name */
    Infowin_set_name(name);
    /* Save the inner border */
    (*Infowin).ox = ox as s16b;
    (*Infowin).oy = oy as s16b;
    /* Make Class Hints */
    ch = XAllocClassHint();
    if ch.is_null() {
        quit(b"XAllocClassHint failed\x00" as *const u8 as
                 *const libc::c_char);
    }
    strcpy(res_name.as_mut_ptr(), name);
    res_name[0 as libc::c_int as usize] =
        if *(*__ctype_b_loc()).offset(res_name[0 as libc::c_int as usize] as
                                          libc::c_int as isize) as libc::c_int
               & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
           {
            tolower(res_name[0 as libc::c_int as usize] as libc::c_int)
        } else { res_name[0 as libc::c_int as usize] as libc::c_int } as
            libc::c_char;
    (*ch).res_name = res_name.as_mut_ptr();
    strcpy(res_class.as_mut_ptr(),
           b"Angband\x00" as *const u8 as *const libc::c_char);
    (*ch).res_class = res_class.as_mut_ptr();
    XSetClassHint((*Metadpy).dpy, (*Infowin).win, ch);
    /* Make Size Hints */
    sh = XAllocSizeHints();
    /* Oops */
    if sh.is_null() {
        quit(b"XAllocSizeHints failed\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Fixed window size */
    if i == 0 as libc::c_int {
        /* Main window: 80x24 -- 255x255 */
        (*sh).flags =
            (1 as libc::c_long) << 4 as libc::c_int |
                (1 as libc::c_long) << 5 as libc::c_int;
        (*sh).min_width =
            80 as libc::c_int * (*(*td).fnt).wid as libc::c_int + (ox + ox);
        (*sh).min_height =
            24 as libc::c_int * (*(*td).fnt).hgt as libc::c_int + (oy + oy);
        (*sh).max_width =
            255 as libc::c_int * (*(*td).fnt).wid as libc::c_int + (ox + ox);
        (*sh).max_height =
            255 as libc::c_int * (*(*td).fnt).hgt as libc::c_int + (oy + oy)
    } else {
        /* Variable window size */
        /* Subwindows: 1x1 -- 255x255 */
        (*sh).flags =
            (1 as libc::c_long) << 4 as libc::c_int |
                (1 as libc::c_long) << 5 as libc::c_int;
        (*sh).min_width = (*(*td).fnt).wid as libc::c_int + (ox + ox);
        (*sh).min_height = (*(*td).fnt).hgt as libc::c_int + (oy + oy);
        (*sh).max_width =
            255 as libc::c_int * (*(*td).fnt).wid as libc::c_int + (ox + ox);
        (*sh).max_height =
            255 as libc::c_int * (*(*td).fnt).hgt as libc::c_int + (oy + oy)
    }
    /* Resize increment */
    (*sh).flags |= (1 as libc::c_long) << 6 as libc::c_int;
    (*sh).width_inc = (*(*td).fnt).wid as libc::c_int;
    (*sh).height_inc = (*(*td).fnt).hgt as libc::c_int;
    /* Base window size */
    (*sh).flags |= (1 as libc::c_long) << 8 as libc::c_int;
    (*sh).base_width = ox + ox;
    (*sh).base_height = oy + oy;
    /* Use the size hints */
    XSetWMNormalHints((*Metadpy).dpy, (*Infowin).win, sh);
    /* Map the window */
    Infowin_map();
    /* Move the window to requested location */
    if x >= 0 as libc::c_int && y >= 0 as libc::c_int {
        Infowin_impell(x, y);
    }
    /* Initialize the term */
    term_init(t, cols, rows, num);
    /* Use a "soft" cursor */
    (*t).soft_cursor = 1 as libc::c_int as bool_;
    /* Erase with "white space" */
    (*t).attr_blank = 1 as libc::c_int as byte_hack;
    (*t).char_blank = ' ' as i32 as libc::c_char;
    /* Hooks */
    (*t).xtra_hook =
        Some(Term_xtra_x11 as
                 unsafe extern "C" fn(_: libc::c_int, _: libc::c_int)
                     -> errr);
    (*t).curs_hook =
        Some(Term_curs_x11 as
                 unsafe extern "C" fn(_: libc::c_int, _: libc::c_int)
                     -> errr);
    (*t).wipe_hook =
        Some(Term_wipe_x11 as
                 unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                      _: libc::c_int) -> errr);
    (*t).text_hook =
        Some(Term_text_x11 as
                 unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                      _: libc::c_int, _: byte_hack, _: cptr)
                     -> errr);
    /* Save the data */
    (*t).data = td as vptr;
    /* Activate (important) */
    Term_activate(t);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialization function for an "X11" module to Angband
 */
#[no_mangle]
pub unsafe extern "C" fn init_x11(mut argc: libc::c_int,
                                  mut argv: *mut *mut libc::c_char) -> errr {
    let mut i: libc::c_int = 0;
    let mut dpy_name: cptr = b"\x00" as *const u8 as *const libc::c_char;
    let mut num_term: libc::c_int = 1 as libc::c_int;
    let mut filename: [libc::c_char; 1024] = [0; 1024];
    let mut pict_wid: libc::c_int = 0 as libc::c_int;
    let mut pict_hgt: libc::c_int = 0 as libc::c_int;
    let mut force_old_graphics: bool_ = 0 as libc::c_int as bool_;
    let mut TmpData: *mut libc::c_char = 0 as *mut libc::c_char;
    /* USE_GRAPHICS */
    /* Parse args */
    i = 1 as libc::c_int;
    while i < argc {
        if prefix(*argv.offset(i as isize) as cptr,
                  b"-d\x00" as *const u8 as *const libc::c_char) != 0 {
            dpy_name =
                &mut *(*argv.offset(i as
                                        isize)).offset(2 as libc::c_int as
                                                           isize) as
                    *mut libc::c_char as cptr
        } else if prefix(*argv.offset(i as isize) as cptr,
                         b"-s\x00" as *const u8 as *const libc::c_char) != 0 {
            smoothRescaling = 0 as libc::c_int as bool_
        } else if prefix(*argv.offset(i as isize) as cptr,
                         b"-o\x00" as *const u8 as *const libc::c_char) != 0 {
            force_old_graphics = 1 as libc::c_int as bool_
        } else if prefix(*argv.offset(i as isize) as cptr,
                         b"-b\x00" as *const u8 as *const libc::c_char) != 0 {
            use_bigtile = 1 as libc::c_int as bool_;
            arg_bigtile = use_bigtile
        } else if prefix(*argv.offset(i as isize) as cptr,
                         b"-n\x00" as *const u8 as *const libc::c_char) != 0 {
            num_term =
                atoi(&mut *(*argv.offset(i as
                                             isize)).offset(2 as libc::c_int
                                                                as isize));
            if num_term > 8 as libc::c_int {
                num_term = 8 as libc::c_int
            } else if num_term < 1 as libc::c_int {
                num_term = 1 as libc::c_int
            }
        } else {
            plog_fmt(b"Ignoring option: %s\x00" as *const u8 as
                         *const libc::c_char, *argv.offset(i as isize));
        }
        i += 1
    }
    /* USE_GRAPHICS */
    /* Init the Metadpy if possible */
    if Metadpy_init_2(0 as *mut libc::c_void as *mut Display, dpy_name) != 0 {
        return -(1 as libc::c_int)
    }
    /* Prepare cursor color */
    xor =
        memset(ralloc(::std::mem::size_of::<infoclr>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<infoclr>() as libc::c_ulong) as
            *mut infoclr;
    Infoclr = xor;
    Infoclr_init_data((*Metadpy).fg, (*Metadpy).bg,
                      Infoclr_Opcode(b"xor\x00" as *const u8 as
                                         *const libc::c_char),
                      0 as libc::c_int);
    /* Prepare normal colors */
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        let mut pixel: Pixell = 0;
        clr[i as usize] =
            memset(ralloc(::std::mem::size_of::<infoclr>() as libc::c_ulong)
                       as *mut libc::c_char as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<infoclr>() as libc::c_ulong) as
                *mut infoclr;
        Infoclr = clr[i as usize];
        /* Acquire Angband colors */
        color_table[i as usize][0 as libc::c_int as usize] =
            angband_color_table[i as usize][0 as libc::c_int as usize];
        color_table[i as usize][1 as libc::c_int as usize] =
            angband_color_table[i as usize][1 as libc::c_int as usize];
        color_table[i as usize][2 as libc::c_int as usize] =
            angband_color_table[i as usize][2 as libc::c_int as usize];
        color_table[i as usize][3 as libc::c_int as usize] =
            angband_color_table[i as usize][3 as libc::c_int as usize];
        /* Default to monochrome */
        pixel =
            if i == 0 as libc::c_int { (*Metadpy).bg } else { (*Metadpy).fg };
        /* Handle color */
        if (*Metadpy).color() != 0 {
            /* Create pixel */
            pixel =
                create_pixel((*Metadpy).dpy,
                             color_table[i as
                                             usize][1 as libc::c_int as
                                                        usize],
                             color_table[i as
                                             usize][2 as libc::c_int as
                                                        usize],
                             color_table[i as
                                             usize][3 as libc::c_int as
                                                        usize])
        }
        /* Initialize the color */
        Infoclr_init_data(pixel, (*Metadpy).bg,
                          Infoclr_Opcode(b"cpy\x00" as *const u8 as
                                             *const libc::c_char),
                          0 as libc::c_int);
        i += 1
    }
    /* Initialize the windows */
    i = 0 as libc::c_int;
    while i < num_term {
        let mut td: *mut term_data =
            &mut *data.as_mut_ptr().offset(i as isize) as *mut term_data;
        /* Initialize the term_data */
        term_data_init(td, i);
        /* Save global entry */
        angband_term[i as usize] = Term;
        i += 1
    }
    /* Raise the "Angband" window */
    Infowin = data[0 as libc::c_int as usize].win;
    Infowin_raise();
    /* Activate the "Angband" window screen */
    Term_activate(&mut (*data.as_mut_ptr().offset(0 as libc::c_int as
                                                      isize)).t);
    /* Try graphics */
    if arg_graphics != 0 {
        /* Try the "16x16.bmp" file */
        path_build(filename.as_mut_ptr(), 1024 as libc::c_int,
                   ANGBAND_DIR_XTRA,
                   b"graf/16x16.bmp\x00" as *const u8 as *const libc::c_char);
        /* Use the "16x16.bmp" file if it exists */
        if force_old_graphics == 0 &&
               0 as libc::c_int ==
                   fd_close(fd_open(filename.as_mut_ptr() as cptr,
                                    0 as libc::c_int)) {
            /* Use graphics */
            use_graphics = 1 as libc::c_int as bool_;
            pict_hgt = 16 as libc::c_int;
            pict_wid = pict_hgt;
            ANGBAND_GRAF = b"new\x00" as *const u8 as *const libc::c_char
        } else {
            /* Try the "8x8.bmp" file */
            path_build(filename.as_mut_ptr(), 1024 as libc::c_int,
                       ANGBAND_DIR_XTRA,
                       b"graf/8x8.bmp\x00" as *const u8 as
                           *const libc::c_char);
            /* Use the "8x8.bmp" file if it exists */
            if 0 as libc::c_int ==
                   fd_close(fd_open(filename.as_mut_ptr() as cptr,
                                    0 as libc::c_int)) {
                /* Use graphics */
                use_graphics = 1 as libc::c_int as bool_;
                pict_hgt = 8 as libc::c_int;
                pict_wid = pict_hgt;
                ANGBAND_GRAF = b"old\x00" as *const u8 as *const libc::c_char
            }
        }
    }
    /* Load graphics */
    if use_graphics != 0 {
        let mut dpy: *mut Display = (*Metadpy).dpy;
        let mut tiles_raw: *mut XImage = 0 as *mut XImage;
        /* Free tiles_raw? XXX XXX */
        tiles_raw = ReadBMP(dpy, filename.as_mut_ptr());
        i = 0 as libc::c_int;
        while i < num_term {
            let mut td_0: *mut term_data =
                &mut *data.as_mut_ptr().offset(i as isize) as *mut term_data;
            let mut t: *mut term = &mut (*td_0).t;
            /* Load the graphical tiles */
            /* Initialize the windows */
            /* Graphics hook */
            (*t).pict_hook =
                Some(Term_pict_x11 as
                         unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                              _: libc::c_int,
                                              _: *const byte_hack,
                                              _: *const libc::c_char,
                                              _: *const byte_hack,
                                              _: *const libc::c_char,
                                              _: *const byte_hack,
                                              _: *const libc::c_char)
                             -> errr);
            /* Use graphics sometimes */
            (*t).higher_pict = 1 as libc::c_int as bool_;
            /* Resize tiles */
            (*td_0).tiles =
                ResizeImage(dpy, tiles_raw, pict_wid, pict_hgt,
                            (*(*td_0).fnt).twid as libc::c_int,
                            (*(*td_0).fnt).hgt as libc::c_int);
            i += 1
        }
        i = 0 as libc::c_int;
        while i < num_term {
            let mut td_1: *mut term_data =
                &mut *data.as_mut_ptr().offset(i as isize) as *mut term_data;
            let mut ii: libc::c_int = 0;
            let mut jj: libc::c_int = 0;
            let mut depth: libc::c_int =
                (*(*(dpy as
                         _XPrivDisplay)).screens.offset((*(dpy as
                                                               _XPrivDisplay)).default_screen
                                                            as
                                                            isize)).root_depth;
            let mut visual: *mut Visual =
                (*(*(dpy as
                         _XPrivDisplay)).screens.offset((*(dpy as
                                                               _XPrivDisplay)).default_screen
                                                            as
                                                            isize)).root_visual;
            let mut total: libc::c_int = 0;
            /* Initialize the transparency masks */
            /* Determine total bytes needed for image */
            ii = 1 as libc::c_int;
            jj = depth - 1 as libc::c_int >> 2 as libc::c_int;
            loop  {
                jj >>= 1 as libc::c_int;
                if !(jj != 0) { break ; }
                ii <<= 1 as libc::c_int
            }
            total =
                (*(*td_1).fnt).twid as libc::c_int *
                    (*(*td_1).fnt).hgt as libc::c_int * ii;
            TmpData = malloc(total as libc::c_ulong) as *mut libc::c_char;
            (*td_1).TmpImage =
                XCreateImage(dpy, visual, depth as libc::c_uint,
                             2 as libc::c_int, 0 as libc::c_int, TmpData,
                             (*(*td_1).fnt).twid as libc::c_uint,
                             (*(*td_1).fnt).hgt as libc::c_uint,
                             8 as libc::c_int, 0 as libc::c_int);
            i += 1
        }
    }
    /* USE_GRAPHICS */
    /* Success */
    return 0 as libc::c_int;
}
/* USE_X11 */
