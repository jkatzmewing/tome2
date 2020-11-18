use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut max_macrotrigger: libc::c_int;
    #[no_mangle]
    static mut macro_template: *mut libc::c_char;
    #[no_mangle]
    static mut macro_modifier_chr: *mut libc::c_char;
    #[no_mangle]
    static mut macro_modifier_name: [*mut libc::c_char; 12];
    #[no_mangle]
    static mut macro_trigger_name: [*mut libc::c_char; 200];
    #[no_mangle]
    static mut macro_trigger_keycode: [[*mut libc::c_char; 200]; 2];
    #[no_mangle]
    static mut hexsym: [libc::c_char; 16];
    #[no_mangle]
    static mut month_day: [libc::c_int; 9];
    #[no_mangle]
    static mut month_name: [cptr; 9];
    #[no_mangle]
    static mut cli_info: *mut cli_comm;
    #[no_mangle]
    static mut cli_total: libc::c_int;
    #[no_mangle]
    static mut character_generated: bool_;
    #[no_mangle]
    static mut character_saved: bool_;
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut command_cmd: s16b;
    #[no_mangle]
    static mut command_arg: s16b;
    #[no_mangle]
    static mut command_dir: s16b;
    #[no_mangle]
    static mut command_new: s16b;
    #[no_mangle]
    static mut msg_flag: bool_;
    #[no_mangle]
    static mut use_sound: bool_;
    #[no_mangle]
    static mut signal_count: s16b;
    #[no_mangle]
    static mut inkey_base: bool_;
    #[no_mangle]
    static mut inkey_xtra: bool_;
    #[no_mangle]
    static mut inkey_scan: bool_;
    #[no_mangle]
    static mut inkey_flag: bool_;
    #[no_mangle]
    static mut text_out_file: *mut FILE;
    #[no_mangle]
    static mut text_out_hook:
           Option<unsafe extern "C" fn(_: byte_hack, _: cptr) -> ()>;
    #[no_mangle]
    static mut text_out_wrap: libc::c_int;
    #[no_mangle]
    static mut text_out_indent: libc::c_int;
    #[no_mangle]
    static mut rogue_like_commands: bool_;
    #[no_mangle]
    static mut quick_messages: bool_;
    #[no_mangle]
    static mut always_repeat: bool_;
    #[no_mangle]
    static mut hilite_player: bool_;
    #[no_mangle]
    static mut ring_bell: bool_;
    #[no_mangle]
    static mut flush_command: bool_;
    #[no_mangle]
    static mut fresh_message: bool_;
    #[no_mangle]
    static mut alert_failure: bool_;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn flock(__fd: libc::c_int, __operation: libc::c_int) -> libc::c_int;
    /* File: util.c */
    /* Purpose: Angband utilities -BEN- */
    /*
* Hack -- External functions
*/
    #[no_mangle]
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    #[no_mangle]
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int)
     -> __off_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn getuid() -> __uid_t;
    #[no_mangle]
    fn streq(s: cptr, t: cptr) -> bool_;
    #[no_mangle]
    static mut message__head: u16b;
    #[no_mangle]
    static mut message__buf: *mut libc::c_char;
    #[no_mangle]
    static mut message__count: *mut u16b;
    #[no_mangle]
    static mut message__type: *mut byte_hack;
    #[no_mangle]
    static mut message__color: *mut byte_hack;
    #[no_mangle]
    static mut message__ptr: *mut u16b;
    #[no_mangle]
    static mut message__last: u16b;
    #[no_mangle]
    static mut message__next: u16b;
    #[no_mangle]
    static mut message__tail: u16b;
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn ralloc(len: huge_hack) -> vptr;
    #[no_mangle]
    fn string_make(str: cptr) -> cptr;
    #[no_mangle]
    fn string_free(str: cptr) -> errr;
    #[no_mangle]
    fn vstrnfmt(buf: *mut libc::c_char, max: uint_hack, fmt: cptr,
                vp: ::std::ffi::VaList) -> uint_hack;
    #[no_mangle]
    fn strnfmt(buf: *mut libc::c_char, max: uint_hack, fmt: cptr, _: ...)
     -> uint_hack;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    static mut Term: *mut term;
    #[no_mangle]
    static mut do_movies: libc::c_int;
    #[no_mangle]
    fn Term_xtra(n: libc::c_int, v: libc::c_int) -> errr;
    #[no_mangle]
    static mut scansubdir_dir: [libc::c_char; 1024];
    #[no_mangle]
    fn Term_fresh() -> errr;
    #[no_mangle]
    fn Term_set_cursor(v: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_gotoxy(x: libc::c_int, y: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_addch(a: byte_hack, c: libc::c_char) -> errr;
    #[no_mangle]
    fn Term_addstr(n: libc::c_int, a: byte_hack, s: cptr) -> errr;
    #[no_mangle]
    fn Term_putch(x: libc::c_int, y: libc::c_int, a: byte_hack,
                  c: libc::c_char) -> errr;
    #[no_mangle]
    fn Term_putstr(x: libc::c_int, y: libc::c_int, n: libc::c_int,
                   a: byte_hack, s: cptr) -> errr;
    #[no_mangle]
    fn Term_erase(x: libc::c_int, y: libc::c_int, n: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_get_cursor(v: *mut libc::c_int) -> errr;
    #[no_mangle]
    fn Term_get_size(w: *mut libc::c_int, h: *mut libc::c_int) -> errr;
    #[no_mangle]
    fn Term_locate(x: *mut libc::c_int, y: *mut libc::c_int) -> errr;
    #[no_mangle]
    fn Term_what(x: libc::c_int, y: libc::c_int, a: *mut byte_hack,
                 c: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn Term_flush() -> errr;
    #[no_mangle]
    fn Term_key_push(k: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_inkey(ch: *mut libc::c_char, wait: bool_, take: bool_) -> errr;
    #[no_mangle]
    fn Term_save() -> errr;
    #[no_mangle]
    fn Term_load() -> errr;
    #[no_mangle]
    fn Term_activate(t: *mut term) -> errr;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn tmpnam(__s: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    static mut macro__num: s16b;
    #[no_mangle]
    static mut macro__pat: *mut cptr;
    #[no_mangle]
    static mut macro__act: *mut cptr;
    #[no_mangle]
    static mut quark__num: s16b;
    #[no_mangle]
    static mut quark__str: *mut cptr;
    #[no_mangle]
    static mut angband_term: [*mut term; 8];
    #[no_mangle]
    static mut keymap_act: [[cptr; 256]; 2];
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut k_name: *mut libc::c_char;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut r_name: *mut libc::c_char;
    #[no_mangle]
    static mut re_info: *mut monster_ego;
    #[no_mangle]
    static mut re_name: *mut libc::c_char;
    #[no_mangle]
    static mut race_info: *mut player_race;
    #[no_mangle]
    static mut rp_name: *mut libc::c_char;
    #[no_mangle]
    static mut race_mod_info: *mut player_race_mod;
    #[no_mangle]
    static mut rmp_name: *mut libc::c_char;
    #[no_mangle]
    static mut max_r_idx: u16b;
    #[no_mangle]
    static mut max_re_idx: u16b;
    #[no_mangle]
    static mut max_k_idx: u16b;
    #[no_mangle]
    static mut auto_more: bool_;
    #[no_mangle]
    static mut gl_timers: *mut timer_type;
    #[no_mangle]
    fn do_cmovie_insert();
    #[no_mangle]
    fn do_cmd_html_dump();
    #[no_mangle]
    fn macro_recorder_add(c: libc::c_char);
    #[no_mangle]
    fn color_char_to_attr(c: libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn window_stuff();
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn toupper(_: libc::c_int) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
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
pub type s32b = libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct obj_theme {
    pub treasure: byte_hack,
    pub combat: byte_hack,
    pub magic: byte_hack,
    pub tools: byte_hack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct object_kind {
    pub name: u32b,
    pub text: u32b,
    pub tval: byte_hack,
    pub sval: byte_hack,
    pub pval: s32b,
    pub pval2: s32b,
    pub to_h: s16b,
    pub to_d: s16b,
    pub to_a: s16b,
    pub activate: s16b,
    pub ac: s16b,
    pub dd: byte_hack,
    pub ds: byte_hack,
    pub weight: s32b,
    pub cost: s32b,
    pub flags1: u32b,
    pub flags2: u32b,
    pub flags3: u32b,
    pub flags4: u32b,
    pub flags5: u32b,
    pub oflags1: u32b,
    pub oflags2: u32b,
    pub oflags3: u32b,
    pub oflags4: u32b,
    pub oflags5: u32b,
    pub locale: [byte_hack; 4],
    pub chance: [byte_hack; 4],
    pub level: byte_hack,
    pub extra: byte_hack,
    pub d_attr: byte_hack,
    pub d_char: libc::c_char,
    pub x_attr: byte_hack,
    pub x_char: libc::c_char,
    pub flavor: byte_hack,
    pub easy_know: bool_,
    pub aware: bool_,
    pub tried: bool_,
    pub know: bool_,
    pub esp: u32b,
    pub oesp: u32b,
    pub btval: byte_hack,
    pub bsval: byte_hack,
    pub artifact: bool_,
    pub power: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_blow {
    pub method: byte_hack,
    pub effect: byte_hack,
    pub d_dice: byte_hack,
    pub d_side: byte_hack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_race {
    pub name: u32b,
    pub text: u32b,
    pub hdice: u16b,
    pub hside: u16b,
    pub ac: s16b,
    pub sleep: s16b,
    pub aaf: byte_hack,
    pub speed: byte_hack,
    pub mexp: s32b,
    pub weight: s32b,
    pub freq_inate: byte_hack,
    pub freq_spell: byte_hack,
    pub flags1: u32b,
    pub flags2: u32b,
    pub flags3: u32b,
    pub flags4: u32b,
    pub flags5: u32b,
    pub flags6: u32b,
    pub flags7: u32b,
    pub flags8: u32b,
    pub flags9: u32b,
    pub blow: [monster_blow; 4],
    pub body_parts: [byte_hack; 6],
    pub level: byte_hack,
    pub rarity: byte_hack,
    pub d_attr: byte_hack,
    pub d_char: libc::c_char,
    pub x_attr: byte_hack,
    pub x_char: libc::c_char,
    pub max_num: s16b,
    pub cur_num: byte_hack,
    pub r_sights: s16b,
    pub r_deaths: s16b,
    pub r_pkills: s16b,
    pub r_tkills: s16b,
    pub r_wake: byte_hack,
    pub r_ignore: byte_hack,
    pub r_xtra1: byte_hack,
    pub r_xtra2: byte_hack,
    pub r_drop_gold: byte_hack,
    pub r_drop_item: byte_hack,
    pub r_cast_inate: byte_hack,
    pub r_cast_spell: byte_hack,
    pub r_blows: [byte_hack; 4],
    pub r_flags1: u32b,
    pub r_flags2: u32b,
    pub r_flags3: u32b,
    pub r_flags4: u32b,
    pub r_flags5: u32b,
    pub r_flags6: u32b,
    pub r_flags7: u32b,
    pub r_flags8: u32b,
    pub r_flags9: u32b,
    pub on_saved: bool_,
    pub total_visible: byte_hack,
    pub drops: obj_theme,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_ego {
    pub name: u32b,
    pub before: bool_,
    pub blow: [monster_blow; 4],
    pub blowm: [[byte_hack; 2]; 4],
    pub hdice: s16b,
    pub hside: s16b,
    pub ac: s16b,
    pub sleep: s16b,
    pub aaf: s16b,
    pub speed: s16b,
    pub mexp: s32b,
    pub weight: s32b,
    pub freq_inate: byte_hack,
    pub freq_spell: byte_hack,
    pub flags1: u32b,
    pub flags2: u32b,
    pub flags3: u32b,
    pub flags7: u32b,
    pub flags8: u32b,
    pub flags9: u32b,
    pub hflags1: u32b,
    pub hflags2: u32b,
    pub hflags3: u32b,
    pub hflags7: u32b,
    pub hflags8: u32b,
    pub hflags9: u32b,
    pub mflags1: u32b,
    pub mflags2: u32b,
    pub mflags3: u32b,
    pub mflags4: u32b,
    pub mflags5: u32b,
    pub mflags6: u32b,
    pub mflags7: u32b,
    pub mflags8: u32b,
    pub mflags9: u32b,
    pub nflags1: u32b,
    pub nflags2: u32b,
    pub nflags3: u32b,
    pub nflags4: u32b,
    pub nflags5: u32b,
    pub nflags6: u32b,
    pub nflags7: u32b,
    pub nflags8: u32b,
    pub nflags9: u32b,
    pub level: s16b,
    pub rarity: s16b,
    pub d_attr: byte_hack,
    pub d_char: libc::c_char,
    pub g_attr: byte_hack,
    pub g_char: libc::c_char,
    pub r_char: [libc::c_char; 5],
    pub nr_char: [libc::c_char; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct object_type {
    pub k_idx: s16b,
    pub iy: byte_hack,
    pub ix: byte_hack,
    pub tval: byte_hack,
    pub sval: byte_hack,
    pub pval: s32b,
    pub pval2: s16b,
    pub pval3: s32b,
    pub discount: byte_hack,
    pub number: byte_hack,
    pub weight: s32b,
    pub elevel: byte_hack,
    pub exp: s32b,
    pub name1: byte_hack,
    pub name2: s16b,
    pub name2b: s16b,
    pub xtra1: byte_hack,
    pub xtra2: s16b,
    pub to_h: s16b,
    pub to_d: s16b,
    pub to_a: s16b,
    pub ac: s16b,
    pub dd: byte_hack,
    pub ds: byte_hack,
    pub timeout: s16b,
    pub ident: byte_hack,
    pub marked: byte_hack,
    pub note: u16b,
    pub art_name: u16b,
    pub art_flags1: u32b,
    pub art_flags2: u32b,
    pub art_flags3: u32b,
    pub art_flags4: u32b,
    pub art_flags5: u32b,
    pub art_esp: u32b,
    pub art_oflags1: u32b,
    pub art_oflags2: u32b,
    pub art_oflags3: u32b,
    pub art_oflags4: u32b,
    pub art_oflags5: u32b,
    pub art_oesp: u32b,
    pub next_o_idx: s16b,
    pub held_m_idx: s16b,
    pub sense: byte_hack,
    pub found: byte_hack,
    pub found_aux1: s16b,
    pub found_aux2: s16b,
    pub found_aux3: s16b,
    pub found_aux4: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_race {
    pub title: s32b,
    pub desc: s32b,
    pub r_adj: [s16b; 6],
    pub luck: libc::c_char,
    pub r_dis: s16b,
    pub r_dev: s16b,
    pub r_sav: s16b,
    pub r_stl: s16b,
    pub r_srh: s16b,
    pub r_fos: s16b,
    pub r_thn: s16b,
    pub r_thb: s16b,
    pub r_mhp: byte_hack,
    pub r_exp: u16b,
    pub b_age: byte_hack,
    pub m_age: byte_hack,
    pub m_b_ht: byte_hack,
    pub m_m_ht: byte_hack,
    pub m_b_wt: byte_hack,
    pub m_m_wt: byte_hack,
    pub f_b_ht: byte_hack,
    pub f_m_ht: byte_hack,
    pub f_b_wt: byte_hack,
    pub f_m_wt: byte_hack,
    pub infra: byte_hack,
    pub choice: [u32b; 2],
    pub powers: [s16b; 4],
    pub body_parts: [byte_hack; 6],
    pub chart: s16b,
    pub flags1: u32b,
    pub flags2: u32b,
    pub oflags1: [u32b; 51],
    pub oflags2: [u32b; 51],
    pub oflags3: [u32b; 51],
    pub oflags4: [u32b; 51],
    pub oflags5: [u32b; 51],
    pub oesp: [u32b; 51],
    pub opval: [s16b; 51],
    pub skill_basem: [libc::c_char; 200],
    pub skill_base: [u32b; 200],
    pub skill_modm: [libc::c_char; 200],
    pub skill_mod: [s16b; 200],
    pub obj_tval: [s16b; 5],
    pub obj_sval: [s16b; 5],
    pub obj_pval: [s16b; 5],
    pub obj_dd: [s16b; 5],
    pub obj_ds: [s16b; 5],
    pub obj_num: s16b,
    pub abilities: [C2RustUnnamed_0; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub ability: s16b,
    pub level: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_race_mod {
    pub title: s32b,
    pub desc: s32b,
    pub place: bool_,
    pub r_adj: [s16b; 6],
    pub luck: libc::c_char,
    pub mana: s16b,
    pub r_dis: s16b,
    pub r_dev: s16b,
    pub r_sav: s16b,
    pub r_stl: s16b,
    pub r_srh: s16b,
    pub r_fos: s16b,
    pub r_thn: s16b,
    pub r_thb: s16b,
    pub r_mhp: libc::c_char,
    pub r_exp: s16b,
    pub b_age: libc::c_char,
    pub m_age: libc::c_char,
    pub m_b_ht: libc::c_char,
    pub m_m_ht: libc::c_char,
    pub m_b_wt: libc::c_char,
    pub m_m_wt: libc::c_char,
    pub f_b_ht: libc::c_char,
    pub f_m_ht: libc::c_char,
    pub f_b_wt: libc::c_char,
    pub f_m_wt: libc::c_char,
    pub infra: libc::c_char,
    pub choice: [u32b; 2],
    pub pclass: [u32b; 2],
    pub mclass: [u32b; 2],
    pub powers: [s16b; 4],
    pub body_parts: [libc::c_char; 6],
    pub flags1: u32b,
    pub flags2: u32b,
    pub oflags1: [u32b; 51],
    pub oflags2: [u32b; 51],
    pub oflags3: [u32b; 51],
    pub oflags4: [u32b; 51],
    pub oflags5: [u32b; 51],
    pub oesp: [u32b; 51],
    pub opval: [s16b; 51],
    pub g_attr: byte_hack,
    pub g_char: libc::c_char,
    pub skill_basem: [libc::c_char; 200],
    pub skill_base: [u32b; 200],
    pub skill_modm: [libc::c_char; 200],
    pub skill_mod: [s16b; 200],
    pub obj_tval: [s16b; 5],
    pub obj_sval: [s16b; 5],
    pub obj_pval: [s16b; 5],
    pub obj_dd: [s16b; 5],
    pub obj_ds: [s16b; 5],
    pub obj_num: s16b,
    pub abilities: [C2RustUnnamed_1; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub ability: s16b,
    pub level: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct help_info {
    pub enabled: bool_,
    pub help1: u32b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_type {
    pub lives: s32b,
    pub oldpy: s16b,
    pub oldpx: s16b,
    pub py: s16b,
    pub px: s16b,
    pub psex: byte_hack,
    pub prace: byte_hack,
    pub pracem: byte_hack,
    pub pclass: byte_hack,
    pub pspec: byte_hack,
    pub mimic_form: byte_hack,
    pub mimic_level: s16b,
    pub oops: byte_hack,
    pub inventory: [object_type; 52],
    pub hitdie: byte_hack,
    pub expfact: u16b,
    pub maximize: byte_hack,
    pub preserve: byte_hack,
    pub special: byte_hack,
    pub allow_one_death: byte_hack,
    pub age: s16b,
    pub ht: s16b,
    pub wt: s16b,
    pub sc: s16b,
    pub au: s32b,
    pub max_exp: s32b,
    pub exp: s32b,
    pub exp_frac: u16b,
    pub lev: s16b,
    pub town_num: s16b,
    pub arena_number: s16b,
    pub inside_arena: s16b,
    pub inside_quest: s16b,
    pub exit_bldg: bool_,
    pub wilderness_x: s32b,
    pub wilderness_y: s32b,
    pub wild_mode: bool_,
    pub old_wild_mode: bool_,
    pub mhp: s16b,
    pub chp: s16b,
    pub chp_frac: u16b,
    pub hp_mod: s16b,
    pub msp: s16b,
    pub csp: s16b,
    pub csp_frac: u16b,
    pub msane: s16b,
    pub csane: s16b,
    pub csane_frac: u16b,
    pub grace: s32b,
    pub pgod: byte_hack,
    pub praying: bool_,
    pub melkor_sacrifice: s16b,
    pub max_plv: s16b,
    pub stat_max: [s16b; 6],
    pub stat_cur: [s16b; 6],
    pub luck_cur: s16b,
    pub luck_max: s16b,
    pub luck_base: s16b,
    pub speed_factor: s16b,
    pub fast: s16b,
    pub lightspeed: s16b,
    pub slow: s16b,
    pub blind: s16b,
    pub paralyzed: s16b,
    pub confused: s16b,
    pub afraid: s16b,
    pub image: s16b,
    pub poisoned: s16b,
    pub cut: s16b,
    pub stun: s16b,
    pub protevil: s16b,
    pub protgood: s16b,
    pub protundead: s16b,
    pub invuln: s16b,
    pub hero: s16b,
    pub shero: s16b,
    pub shield: s16b,
    pub shield_power: s16b,
    pub shield_opt: s16b,
    pub shield_power_opt: s16b,
    pub shield_power_opt2: s16b,
    pub blessed: s16b,
    pub tim_invis: s16b,
    pub tim_infra: s16b,
    pub oppose_acid: s16b,
    pub oppose_elec: s16b,
    pub oppose_fire: s16b,
    pub oppose_cold: s16b,
    pub oppose_pois: s16b,
    pub oppose_ld: s16b,
    pub oppose_cc: s16b,
    pub oppose_ss: s16b,
    pub oppose_nex: s16b,
    pub rush: s16b,
    pub tim_esp: s16b,
    pub tim_wraith: s16b,
    pub tim_ffall: s16b,
    pub tim_fly: s16b,
    pub tim_fire_aura: s16b,
    pub tim_poison: s16b,
    pub tim_thunder: s16b,
    pub tim_thunder_p1: s16b,
    pub tim_thunder_p2: s16b,
    pub tim_project: s16b,
    pub tim_project_dam: s16b,
    pub tim_project_gf: s16b,
    pub tim_project_rad: s16b,
    pub tim_project_flag: s16b,
    pub tim_roots: s16b,
    pub tim_roots_ac: s16b,
    pub tim_roots_dam: s16b,
    pub resist_magic: s16b,
    pub tim_invisible: s16b,
    pub tim_inv_pow: s16b,
    pub tim_mimic: s16b,
    pub tim_lite: s16b,
    pub tim_regen: s16b,
    pub tim_regen_pow: s16b,
    pub holy: s16b,
    pub walk_water: s16b,
    pub tim_mental_barrier: s16b,
    pub strike: s16b,
    pub meditation: s16b,
    pub tim_reflect: s16b,
    pub tim_res_time: s16b,
    pub tim_deadly: s16b,
    pub prob_travel: s16b,
    pub disrupt_shield: s16b,
    pub parasite: s16b,
    pub parasite_r_idx: s16b,
    pub loan: s32b,
    pub loan_time: s32b,
    pub absorb_soul: s16b,
    pub tim_magic_breath: s16b,
    pub tim_water_breath: s16b,
    pub immov_cntr: s16b,
    pub chaos_patron: s16b,
    pub recall_dungeon: s16b,
    pub word_recall: s16b,
    pub energy: s32b,
    pub food: s16b,
    pub confusing: byte_hack,
    pub searching: byte_hack,
    pub new_spells: s16b,
    pub old_spells: s16b,
    pub xtra_spells: s16b,
    pub old_cumber_armor: bool_,
    pub old_cumber_glove: bool_,
    pub old_heavy_wield: bool_,
    pub old_heavy_shoot: bool_,
    pub old_icky_wield: bool_,
    pub old_lite: s16b,
    pub old_view: s16b,
    pub old_food_aux: s16b,
    pub cumber_armor: bool_,
    pub cumber_glove: bool_,
    pub heavy_wield: bool_,
    pub heavy_shoot: bool_,
    pub icky_wield: bool_,
    pub immovable: bool_,
    pub cur_lite: s16b,
    pub notice: u32b,
    pub update: u32b,
    pub redraw: u32b,
    pub window: u32b,
    pub stat_use: [s16b; 6],
    pub stat_top: [s16b; 6],
    pub stat_add: [s16b; 6],
    pub stat_ind: [s16b; 6],
    pub stat_cnt: [s16b; 6],
    pub stat_los: [s16b; 6],
    pub immune_acid: bool_,
    pub immune_elec: bool_,
    pub immune_fire: bool_,
    pub immune_cold: bool_,
    pub immune_neth: bool_,
    pub resist_acid: bool_,
    pub resist_elec: bool_,
    pub resist_fire: bool_,
    pub resist_cold: bool_,
    pub resist_pois: bool_,
    pub resist_conf: bool_,
    pub resist_sound: bool_,
    pub resist_lite: bool_,
    pub resist_dark: bool_,
    pub resist_chaos: bool_,
    pub resist_disen: bool_,
    pub resist_shard: bool_,
    pub resist_nexus: bool_,
    pub resist_blind: bool_,
    pub resist_neth: bool_,
    pub resist_fear: bool_,
    pub resist_continuum: bool_,
    pub sensible_fire: bool_,
    pub sensible_lite: bool_,
    pub reflect: bool_,
    pub sh_fire: bool_,
    pub sh_elec: bool_,
    pub wraith_form: bool_,
    pub anti_magic: bool_,
    pub anti_tele: bool_,
    pub sustain_str: bool_,
    pub sustain_int: bool_,
    pub sustain_wis: bool_,
    pub sustain_dex: bool_,
    pub sustain_con: bool_,
    pub sustain_chr: bool_,
    pub aggravate: bool_,
    pub teleport: bool_,
    pub exp_drain: bool_,
    pub drain_mana: byte_hack,
    pub drain_life: byte_hack,
    pub magical_breath: bool_,
    pub water_breath: bool_,
    pub climb: bool_,
    pub fly: bool_,
    pub ffall: bool_,
    pub lite: bool_,
    pub free_act: bool_,
    pub see_inv: bool_,
    pub regenerate: bool_,
    pub hold_life: bool_,
    pub telepathy: u32b,
    pub slow_digest: bool_,
    pub bless_blade: bool_,
    pub xtra_might: byte_hack,
    pub impact: bool_,
    pub auto_id: bool_,
    pub invis: s16b,
    pub dis_to_h: s16b,
    pub dis_to_d: s16b,
    pub dis_to_a: s16b,
    pub dis_ac: s16b,
    pub to_l: s16b,
    pub to_m: s16b,
    pub to_s: s16b,
    pub to_h: s16b,
    pub to_d: s16b,
    pub to_h_melee: s16b,
    pub to_d_melee: s16b,
    pub to_h_ranged: s16b,
    pub to_d_ranged: s16b,
    pub to_a: s16b,
    pub ac: s16b,
    pub antimagic: byte_hack,
    pub antimagic_dis: byte_hack,
    pub see_infra: s16b,
    pub skill_dis: s16b,
    pub skill_dev: s16b,
    pub skill_sav: s16b,
    pub skill_stl: s16b,
    pub skill_srh: s16b,
    pub skill_fos: s16b,
    pub skill_thn: s16b,
    pub skill_thb: s16b,
    pub skill_tht: s16b,
    pub skill_dig: s16b,
    pub num_blow: s16b,
    pub num_fire: s16b,
    pub xtra_crit: s16b,
    pub throw_mult: byte_hack,
    pub tval_xtra: byte_hack,
    pub tval_ammo: byte_hack,
    pub pspeed: s16b,
    pub mimic_extra: u32b,
    pub antimagic_extra: u32b,
    pub druid_extra: u32b,
    pub druid_extra2: u32b,
    pub druid_extra3: u32b,
    pub music_extra: u32b,
    pub music_extra2: u32b,
    pub necro_extra: u32b,
    pub necro_extra2: u32b,
    pub race_extra1: u32b,
    pub race_extra2: u32b,
    pub race_extra3: u32b,
    pub race_extra4: u32b,
    pub race_extra5: u32b,
    pub race_extra6: u32b,
    pub race_extra7: u32b,
    pub dodge_chance: s16b,
    pub maintain_sum: u32b,
    pub spellbinder_num: byte_hack,
    pub spellbinder: [u32b; 4],
    pub spellbinder_trigger: byte_hack,
    pub mimic_name: cptr,
    pub tactic: libc::c_char,
    pub movement: libc::c_char,
    pub companion_killed: s16b,
    pub no_mortal: bool_,
    pub black_breath: bool_,
    pub precognition: bool_,
    pub xtra_f1: u32b,
    pub xtra_f2: u32b,
    pub xtra_f3: u32b,
    pub xtra_f4: u32b,
    pub xtra_f5: u32b,
    pub xtra_esp: u32b,
    pub corruptions: *mut bool_,
    pub pet_follow_distance: byte_hack,
    pub pet_open_doors: byte_hack,
    pub pet_pickup_items: byte_hack,
    pub control: s16b,
    pub control_dir: byte_hack,
    pub body_monster: u16b,
    pub disembodied: bool_,
    pub body_parts: [byte_hack; 28],
    pub astral: bool_,
    pub powers: *mut bool_,
    pub powers_mod: [bool_; 62],
    pub skill_points: s16b,
    pub skill_last_level: s16b,
    pub melee_style: s16b,
    pub use_piercing_shots: s16b,
    pub help: help_info,
    pub did_nothing: bool_,
    pub leaving: bool_,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_comm {
    pub comm: cptr,
    pub descrip: cptr,
    pub key: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timer_type {
    pub next: *mut timer_type,
    pub enabled: bool_,
    pub delay: s32b,
    pub countdown: s32b,
    pub callback: cptr,
}
/*
* Find a default user name from the system.
*/
#[no_mangle]
pub unsafe extern "C" fn user_name(mut buf: *mut libc::c_char,
                                   mut id: libc::c_int) {
    let mut pw: *mut passwd = 0 as *mut passwd;
    /* Look up the user name */
    pw = getpwuid(id as __uid_t);
    if !pw.is_null() {
        strcpy(buf, (*pw).pw_name);
        *buf.offset(16 as libc::c_int as isize) =
            '\u{0}' as i32 as libc::c_char;
        return
    }
    /* SET_UID */
    /* Oops.  Hack -- default to "PLAYER" */
    strcpy(buf, b"PLAYER\x00" as *const u8 as *const libc::c_char);
}
/* SET_UID */
/*
* The concept of the "file" routines below (and elsewhere) is that all
* file handling should be done using as few routines as possible, since
* every machine is slightly different, but these routines always have the
* same semantics.
*
* In fact, perhaps we should use the "path_parse()" routine below to convert
* from "canonical" filenames (optional leading tilde's, internal wildcards,
* slash as the path seperator, etc) to "system" filenames (no special symbols,
* system-specific path seperator, etc).  This would allow the program itself
* to assume that all filenames are "Unix" filenames, and explicitly "extract"
* such filenames if needed (by "path_parse()", or perhaps "path_canon()").
*
* Note that "path_temp" should probably return a "canonical" filename.
*
* Note that "my_fopen()" and "my_open()" and "my_make()" and "my_kill()"
* and "my_move()" and "my_copy()" should all take "canonical" filenames.
*
* Note that "canonical" filenames use a leading "slash" to indicate an absolute
* path, and a leading "tilde" to indicate a special directory, and default to a
* relative path, but MSDOS uses a leading "drivename plus colon" to indicate the
* use of a "special drive", and then the rest of the path is parsed "normally",
* and MACINTOSH uses a leading colon to indicate a relative path, and an embedded
* colon to indicate a "drive plus absolute path", and finally defaults to a file
* in the current working directory, which may or may not be defined.
*
* We should probably parse a leading "~~/" as referring to "ANGBAND_DIR". (?)
*/
/*
* Extract a "parsed" path from an initial filename
* Normally, we simply copy the filename into the buffer
* But leading tilde symbols must be handled in a special way
* Replace "~/" by the home directory of the current user
*/
#[no_mangle]
pub unsafe extern "C" fn path_parse(mut buf: *mut libc::c_char,
                                    mut max: libc::c_int, mut file: cptr)
 -> errr {
    let mut u: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut pw: *mut passwd = 0 as *mut passwd;
    /* Assume no result */
    *buf.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    /* No file? */
    if file.is_null() { return -(1 as libc::c_int) }
    /* File needs no parsing */
    if *file.offset(0 as libc::c_int as isize) as libc::c_int != '~' as i32 {
        strcpy(buf, file);
        return 0 as libc::c_int
    }
    /* Point at the user */
    u = file.offset(1 as libc::c_int as isize);
    /* Look for non-user portion of the file */
    s = strstr(u, b"/\x00" as *const u8 as *const libc::c_char) as cptr;
    /* Look up password data for user */
    pw = getpwuid(getuid());
    /* Nothing found? */
    if pw.is_null() { return 1 as libc::c_int }
    /* Make use of the info */
    strcpy(buf, (*pw).pw_dir);
    /* Append the rest of the filename, if any */
    if !s.is_null() { strcat(buf, s); }
    /* Success */
    return 0 as libc::c_int;
}
/* SET_UID */
/* SET_UID */
/*
* Hack -- acquire a "temporary" file name if possible
*
* This filename is always in "system-specific" form.
*/
#[no_mangle]
pub unsafe extern "C" fn path_temp(mut buf: *mut libc::c_char,
                                   mut max: libc::c_int) -> errr {
    let mut s: cptr = 0 as *const libc::c_char;
    /* Temp file */
    s = tmpnam(0 as *mut libc::c_char) as cptr;
    /* Oops */
    if s.is_null() { return -(1 as libc::c_int) }
    /* Format to length */
    strnfmt(buf, max as uint_hack,
            b"%s\x00" as *const u8 as *const libc::c_char, s);
    /* Success */
    return 0 as libc::c_int;
}
/*
* Create a new path by appending a file (or directory) to a path
*
* This requires no special processing on simple machines, except
* for verifying the size of the filename, but note the ability to
* bypass the given "path" with certain special file-names.
*
* Note that the "file" may actually be a "sub-path", including
* a path and a file.
*
* Note that this function yields a path which must be "parsed"
* using the "parse" function above.
*/
#[no_mangle]
pub unsafe extern "C" fn path_build(mut buf: *mut libc::c_char,
                                    mut max: libc::c_int, mut path: cptr,
                                    mut file: cptr) -> errr {
    /* Special file */
    if *file.offset(0 as libc::c_int as isize) as libc::c_int == '~' as i32 {
        /* Use the file itself */
        strnfmt(buf, max as uint_hack,
                b"%s\x00" as *const u8 as *const libc::c_char, file);
    } else if prefix(file, b"/\x00" as *const u8 as *const libc::c_char) as
                  libc::c_int != 0 &&
                  streq(b"/\x00" as *const u8 as *const libc::c_char,
                        b"\x00" as *const u8 as *const libc::c_char) == 0 {
        /* Absolute file, on "normal" systems */
        /* Use the file itself */
        strnfmt(buf, max as uint_hack,
                b"%s\x00" as *const u8 as *const libc::c_char, file);
    } else if *path.offset(0 as libc::c_int as isize) == 0 {
        /* No path given */
        /* Use the file itself */
        strnfmt(buf, max as uint_hack,
                b"%s\x00" as *const u8 as *const libc::c_char, file);
    } else {
        /* Path and File */
        /* Build the new path */
        strnfmt(buf, max as uint_hack,
                b"%s%s%s\x00" as *const u8 as *const libc::c_char, path,
                b"/\x00" as *const u8 as *const libc::c_char, file);
    }
    /* Success */
    return 0 as libc::c_int;
}
/*
* Hack -- replacement for "fopen()"
*/
#[no_mangle]
pub unsafe extern "C" fn my_fopen(mut file: cptr, mut mode: cptr)
 -> *mut FILE {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Hack -- Try to parse the path */
    if path_parse(buf.as_mut_ptr(), 1024 as libc::c_int, file) != 0 {
        return 0 as *mut FILE
    }
    /* Attempt to fopen the file anyway */
    return fopen(buf.as_mut_ptr(), mode);
    /* MACH_O_CARBON */
    /* MACH_O_CARBON */
}
/*
* Hack -- replacement for "fclose()"
*/
#[no_mangle]
pub unsafe extern "C" fn my_fclose(mut fff: *mut FILE) -> errr {
    /* Require a file */
    if fff.is_null() { return -(1 as libc::c_int) }
    /* Close, check for error */
    if fclose(fff) == -(1 as libc::c_int) { return 1 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
/*
* Hack -- replacement for "fgets()"
*
* Read a string, without a newline, to a file
*
* Process tabs, strip internal non-printables
*/
#[no_mangle]
pub unsafe extern "C" fn my_fgets(mut fff: *mut FILE,
                                  mut buf: *mut libc::c_char,
                                  mut n: huge_hack) -> errr {
    let mut i: huge_hack = 0 as libc::c_int as huge_hack;
    loop  {
        let mut c: libc::c_int = fgetc(fff);
        if c == -(1 as libc::c_int) {
            /* Terminate */
            *buf.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
            /* Success (0) if some characters were read */
            return (i == 0 as libc::c_int as libc::c_ulong) as libc::c_int
        } else if c == '\r' as i32 {
            c = fgetc(fff);
            if c != '\n' as i32 { ungetc(c, fff); }
            /* Handle newline -- DOS (\015\012), Mac (\015), UNIX (\012) */
            /* Terminate */
            *buf.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
            /* Success */
            return 0 as libc::c_int
        } else if c == '\n' as i32 {
            c = fgetc(fff);
            if c != '\r' as i32 { ungetc(c, fff); }
            /* Terminate */
            *buf.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
            /* Success */
            return 0 as libc::c_int
        } else if c == '\t' as i32 {
            /* Handle tabs */
            /* Hack -- require room */
            if i.wrapping_add(8 as libc::c_int as libc::c_ulong) >= n {
                break ;
            }
            loop 
                 /* Append 1-8 spaces */
                 {
                let fresh0 = i;
                i = i.wrapping_add(1);
                *buf.offset(fresh0 as isize) = ' ' as i32 as libc::c_char;
                if !(i.wrapping_rem(8 as libc::c_int as libc::c_ulong) != 0) {
                    break ;
                }
            }
        } else {
            /* Handle printables */
            if !(*(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                     _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                     != 0) {
                continue ;
            }
            /* Copy */
            let fresh1 = i;
            i = i.wrapping_add(1);
            *buf.offset(fresh1 as isize) = c as libc::c_char;
            /* Check length */
            if i >= n { break ; }
        }
    }
    /* Nothing */
    *buf.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    /* Failure */
    return 1 as libc::c_int;
}
/*
* Hack -- replacement for "fputs()"
*
* Dump a string, plus a newline, to a file
*
* XXX XXX XXX Process internal weirdness?
*/
#[no_mangle]
pub unsafe extern "C" fn my_fputs(mut fff: *mut FILE, mut buf: cptr,
                                  mut n: huge_hack) -> errr {
    /* XXX XXX */
    n = if n != 0 { n } else { 0 as libc::c_int as libc::c_ulong };
    /* Dump, ignore errors */
    fprintf(fff, b"%s\n\x00" as *const u8 as *const libc::c_char, buf);
    /* Success */
    return 0 as libc::c_int;
}
/* O_BINARY */
/*
* Hack -- attempt to delete a file
*/
#[no_mangle]
pub unsafe extern "C" fn fd_kill(mut file: cptr) -> errr {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Hack -- Try to parse the path */
    if path_parse(buf.as_mut_ptr(), 1024 as libc::c_int, file) != 0 {
        return -(1 as libc::c_int)
    }
    /* Remove */
    remove(buf.as_mut_ptr());
    /* XXX XXX XXX */
    return 0 as libc::c_int;
}
/*
* Hack -- attempt to move a file
*/
#[no_mangle]
pub unsafe extern "C" fn fd_move(mut file: cptr, mut what: cptr) -> errr {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut aux: [libc::c_char; 1024] = [0; 1024];
    /* Hack -- Try to parse the path */
    if path_parse(buf.as_mut_ptr(), 1024 as libc::c_int, file) != 0 {
        return -(1 as libc::c_int)
    }
    /* Hack -- Try to parse the path */
    if path_parse(aux.as_mut_ptr(), 1024 as libc::c_int, what) != 0 {
        return -(1 as libc::c_int)
    }
    /* Rename */
    rename(buf.as_mut_ptr(), aux.as_mut_ptr());
    /* XXX XXX XXX */
    return 0 as libc::c_int;
}
/*
* Hack -- attempt to copy a file
*/
#[no_mangle]
pub unsafe extern "C" fn fd_copy(mut file: cptr, mut what: cptr) -> errr {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut aux: [libc::c_char; 1024] = [0; 1024];
    /* Hack -- Try to parse the path */
    if path_parse(buf.as_mut_ptr(), 1024 as libc::c_int, file) != 0 {
        return -(1 as libc::c_int)
    }
    /* Hack -- Try to parse the path */
    if path_parse(aux.as_mut_ptr(), 1024 as libc::c_int, what) != 0 {
        return -(1 as libc::c_int)
    }
    /* Copy XXX XXX XXX */
	/* (void)rename(buf, aux); */
    /* XXX XXX XXX */
    return 1 as libc::c_int;
}
/*
* Hack -- attempt to open a file descriptor (create file)
*
* This function should fail if the file already exists
*
* Note that we assume that the file should be "binary"
*
* XXX XXX XXX The horrible "BEN_HACK" code is for compiling under
* the CodeWarrior compiler, in which case, for some reason, none
* of the "O_*" flags are defined, and we must fake the definition
* of "O_RDONLY", "O_WRONLY", and "O_RDWR" in "A-win-h", and then
* we must simulate the effect of the proper "open()" call below.
*/
#[no_mangle]
pub unsafe extern "C" fn fd_make(mut file: cptr, mut mode: libc::c_int)
 -> libc::c_int {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Hack -- Try to parse the path */
    if path_parse(buf.as_mut_ptr(), 1024 as libc::c_int, file) != 0 {
        return -(1 as libc::c_int)
    }
    /* BEN_HACK */
    /* MACH_O_CARBON */
    /* Create the file, fail if exists, write-only, binary */
    return open(buf.as_mut_ptr(),
                0o100 as libc::c_int | 0o200 as libc::c_int |
                    0o1 as libc::c_int | 0 as libc::c_int, mode);
    /* MACH_O_CARBON */
    /* BEN_HACK */
}
/*
* Hack -- attempt to open a file descriptor (existing file)
*
* Note that we assume that the file should be "binary"
*/
#[no_mangle]
pub unsafe extern "C" fn fd_open(mut file: cptr, mut flags: libc::c_int)
 -> libc::c_int {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Hack -- Try to parse the path */
    if path_parse(buf.as_mut_ptr(), 1024 as libc::c_int, file) != 0 {
        return -(1 as libc::c_int)
    }
    /* MACH_O_CARBON */
    /* Attempt to open the file */
    return open(buf.as_mut_ptr(), flags | 0 as libc::c_int, 0 as libc::c_int);
    /* MACH_O_CARBON */
}
/*
* Hack -- attempt to lock a file descriptor
*
* Legal lock types -- F_UNLCK, F_RDLCK, F_WRLCK
*/
#[no_mangle]
pub unsafe extern "C" fn fd_lock(mut fd: libc::c_int, mut what: libc::c_int)
 -> errr {
    /* XXX XXX */
    what = if what != 0 { what } else { 0 as libc::c_int };
    /* Verify the fd */
    if fd < 0 as libc::c_int { return -(1 as libc::c_int) }
    /* Un-Lock */
    if what == 2 as libc::c_int {
        /* Unlock it, Ignore errors */
        flock(fd, 8 as libc::c_int);
    } else if flock(fd, 2 as libc::c_int) != 0 as libc::c_int {
        return 1 as libc::c_int
    }
    /* Lock */
    /* Lock the score file */
    /* Success */
    return 0 as libc::c_int;
}
/*
* Hack -- attempt to seek on a file descriptor
*/
#[no_mangle]
pub unsafe extern "C" fn fd_seek(mut fd: libc::c_int, mut n: huge_hack)
 -> errr {
    let mut p: s32b = 0;
    /* Verify fd */
    if fd < 0 as libc::c_int { return -(1 as libc::c_int) }
    /* Seek to the given position */
    p = lseek(fd, n as __off_t, 0 as libc::c_int) as s32b;
    /* Failure */
    if p < 0 as libc::c_int { return 1 as libc::c_int }
    /* Failure */
    if p as huge_hack != n { return 1 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
/*
* Hack -- attempt to read data from a file descriptor
*/
#[no_mangle]
pub unsafe extern "C" fn fd_read(mut fd: libc::c_int,
                                 mut buf: *mut libc::c_char, mut n: huge_hack)
 -> errr {
    /* Verify the fd */
    if fd < 0 as libc::c_int { return -(1 as libc::c_int) }
    /* Read the final piece */
    if read(fd, buf as *mut libc::c_void, n) as huge_hack != n {
        return 1 as libc::c_int
    }
    /* Success */
    return 0 as libc::c_int;
}
/*
* Hack -- Attempt to write data to a file descriptor
*/
#[no_mangle]
pub unsafe extern "C" fn fd_write(mut fd: libc::c_int, mut buf: cptr,
                                  mut n: huge_hack) -> errr {
    /* Verify the fd */
    if fd < 0 as libc::c_int { return -(1 as libc::c_int) }
    /* Write the final piece */
    if write(fd, buf as *const libc::c_void, n) as huge_hack != n {
        return 1 as libc::c_int
    }
    /* Success */
    return 0 as libc::c_int;
}
/*
* Hack -- attempt to close a file descriptor
*/
#[no_mangle]
pub unsafe extern "C" fn fd_close(mut fd: libc::c_int) -> errr {
    /* Verify the fd */
    if fd < 0 as libc::c_int { return -(1 as libc::c_int) }
    /* Close */
    close(fd);
    /* XXX XXX XXX */
    return 0 as libc::c_int;
}
/*
* XXX XXX XXX Important note about "colors" XXX XXX XXX
*
* The "TERM_*" color definitions list the "composition" of each
* "Angband color" in terms of "quarters" of each of the three color
* components (Red, Green, Blue), for example, TERM_UMBER is defined
* as 2/4 Red, 1/4 Green, 0/4 Blue.
*
* The following info is from "Torbjorn Lindgren" (see "main-xaw.c").
*
* These values are NOT gamma-corrected.  On most machines (with the
* Macintosh being an important exception), you must "gamma-correct"
* the given values, that is, "correct for the intrinsic non-linearity
* of the phosphor", by converting the given intensity levels based
* on the "gamma" of the target screen, which is usually 1.7 (or 1.5).
*
* The actual formula for conversion is unknown to me at this time,
* but you can use the table below for the most common gamma values.
*
* So, on most machines, simply convert the values based on the "gamma"
* of the target screen, which is usually in the range 1.5 to 1.7, and
* usually is closest to 1.7.  The converted value for each of the five
* different "quarter" values is given below:
*
*  Given     Gamma 1.0       Gamma 1.5       Gamma 1.7     Hex 1.7
*  -----       ----            ----            ----          ---
*   0/4        0.00            0.00            0.00          #00
*   1/4        0.25            0.27            0.28          #47
*   2/4        0.50            0.55            0.56          #8f
*   3/4        0.75            0.82            0.84          #d7
*   4/4        1.00            1.00            1.00          #ff
*
* Note that some machines (i.e. most IBM machines) are limited to a
* hard-coded set of colors, and so the information above is useless.
*
* Also, some machines are limited to a pre-determined set of colors,
* for example, the IBM can only display 16 colors, and only 14 of
* those colors resemble colors used by Angband, and then only when
* you ignore the fact that "Slate" and "cyan" are not really matches,
* so on the IBM, we use "orange" for both "Umber", and "Light Umber"
* in addition to the obvious "Orange", since by combining all of the
* "indeterminate" colors into a single color, the rest of the colors
* are left with "meaningful" values.
*/
/*
* Move the cursor
*/
#[no_mangle]
pub unsafe extern "C" fn move_cursor(mut row: libc::c_int,
                                     mut col: libc::c_int) {
    Term_gotoxy(col, row);
}
/*
* Convert a decimal to a single digit octal number
*/
unsafe extern "C" fn octify(mut i: uint_hack) -> libc::c_char {
    return hexsym[i.wrapping_rem(8 as libc::c_int as libc::c_uint) as usize];
}
/*
* Convert a decimal to a single digit hex number
*/
unsafe extern "C" fn hexify(mut i: uint_hack) -> libc::c_char {
    return hexsym[i.wrapping_rem(16 as libc::c_int as libc::c_uint) as usize];
}
/*
* Convert a octal-digit into a decimal
*/
unsafe extern "C" fn deoct(mut c: libc::c_char) -> libc::c_int {
    if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int &
           _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        return c as libc::c_int - '0' as i32
    }
    return 0 as libc::c_int;
}
/*
* Convert a hexidecimal-digit into a decimal
*/
unsafe extern "C" fn dehex(mut c: libc::c_char) -> libc::c_int {
    if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int &
           _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        return c as libc::c_int - '0' as i32
    }
    if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int &
           _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        return c as libc::c_int - 'a' as i32 + 10 as libc::c_int
    }
    if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int &
           _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        return tolower(c as libc::c_int) - 'a' as i32 + 10 as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn my_stricmp(mut a: cptr, mut b: cptr) -> libc::c_int {
    let mut s1: cptr = 0 as *const libc::c_char;
    let mut s2: cptr = 0 as *const libc::c_char;
    let mut z1: libc::c_char = 0;
    let mut z2: libc::c_char = 0;
    /* Scan the strings */
    s1 = a;
    s2 = b;
    loop  {
        z1 =
            if *(*__ctype_b_loc()).offset(*s1 as libc::c_int as isize) as
                   libc::c_int &
                   _ISlower as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 {
                toupper(*s1 as libc::c_int)
            } else { *s1 as libc::c_int } as libc::c_char;
        z2 =
            if *(*__ctype_b_loc()).offset(*s2 as libc::c_int as isize) as
                   libc::c_int &
                   _ISlower as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 {
                toupper(*s2 as libc::c_int)
            } else { *s2 as libc::c_int } as libc::c_char;
        if (z1 as libc::c_int) < z2 as libc::c_int {
            return -(1 as libc::c_int)
        }
        if z1 as libc::c_int > z2 as libc::c_int { return 1 as libc::c_int }
        if z1 == 0 { return 0 as libc::c_int }
        s1 = s1.offset(1);
        s2 = s2.offset(1)
    };
}
unsafe extern "C" fn my_strnicmp(mut a: cptr, mut b: cptr, mut n: libc::c_int)
 -> libc::c_int {
    let mut s1: cptr = 0 as *const libc::c_char;
    let mut s2: cptr = 0 as *const libc::c_char;
    let mut z1: libc::c_char = 0;
    let mut z2: libc::c_char = 0;
    /* Scan the strings */
    s1 = a;
    s2 = b;
    while n > 0 as libc::c_int {
        z1 =
            if *(*__ctype_b_loc()).offset(*s1 as libc::c_int as isize) as
                   libc::c_int &
                   _ISlower as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 {
                toupper(*s1 as libc::c_int)
            } else { *s1 as libc::c_int } as libc::c_char;
        z2 =
            if *(*__ctype_b_loc()).offset(*s2 as libc::c_int as isize) as
                   libc::c_int &
                   _ISlower as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 {
                toupper(*s2 as libc::c_int)
            } else { *s2 as libc::c_int } as libc::c_char;
        if (z1 as libc::c_int) < z2 as libc::c_int {
            return -(1 as libc::c_int)
        }
        if z1 as libc::c_int > z2 as libc::c_int { return 1 as libc::c_int }
        if z1 == 0 { return 0 as libc::c_int }
        s1 = s1.offset(1);
        s2 = s2.offset(1);
        n -= 1
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn trigger_text_to_ascii(mut bufptr: *mut *mut libc::c_char,
                                           mut strptr: *mut cptr) {
    let mut s: *mut libc::c_char = *bufptr;
    let mut str: cptr = *strptr;
    let mut mod_status: [bool_; 12] = [0; 12];
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut shiftstatus: libc::c_int = 0 as libc::c_int;
    let mut key_code: cptr = 0 as *const libc::c_char;
    if macro_template.is_null() { return }
    i = 0 as libc::c_int;
    while *macro_modifier_chr.offset(i as isize) != 0 {
        mod_status[i as usize] = 0 as libc::c_int as bool_;
        i += 1
    }
    str = str.offset(1);
    loop 
         /* Examine modifier keys */
         {
        i = 0 as libc::c_int;
        while *macro_modifier_chr.offset(i as isize) != 0 {
            len = strlen(macro_modifier_name[i as usize]) as libc::c_int;
            if my_strnicmp(str, macro_modifier_name[i as usize] as cptr, len)
                   == 0 {
                break ;
            }
            i += 1
        }
        if *macro_modifier_chr.offset(i as isize) == 0 { break ; }
        str = str.offset(len as isize);
        mod_status[i as usize] = 1 as libc::c_int as bool_;
        if 'S' as i32 == *macro_modifier_chr.offset(i as isize) as libc::c_int
           {
            shiftstatus = 1 as libc::c_int
        }
    }
    i = 0 as libc::c_int;
    while i < max_macrotrigger {
        len = strlen(macro_trigger_name[i as usize]) as libc::c_int;
        if my_strnicmp(str, macro_trigger_name[i as usize] as cptr, len) == 0
               && ']' as i32 == *str.offset(len as isize) as libc::c_int {
            break ;
        }
        i += 1
    }
    /* Invalid trigger name? */
    if i == max_macrotrigger {
        str = strchr(str, ']' as i32) as cptr;
        if !str.is_null() {
            let fresh2 = s;
            s = s.offset(1);
            *fresh2 = 31 as libc::c_int as libc::c_char;
            let fresh3 = s;
            s = s.offset(1);
            *fresh3 = 13 as libc::c_int as libc::c_char;
            *bufptr = s;
            *strptr = str
            /* where **strptr == ']' */
        } /* where **strptr == ']' */
        return
    }
    key_code =
        macro_trigger_keycode[shiftstatus as usize][i as usize] as cptr;
    str = str.offset(len as isize);
    let fresh4 = s;
    s = s.offset(1);
    *fresh4 = 31 as libc::c_int as libc::c_char;
    i = 0 as libc::c_int;
    while *macro_template.offset(i as isize) != 0 {
        let mut ch: libc::c_char = *macro_template.offset(i as isize);
        let mut j: libc::c_int = 0;
        match ch as libc::c_int {
            38 => {
                j = 0 as libc::c_int;
                while *macro_modifier_chr.offset(j as isize) != 0 {
                    if mod_status[j as usize] != 0 {
                        let fresh5 = s;
                        s = s.offset(1);
                        *fresh5 = *macro_modifier_chr.offset(j as isize)
                    }
                    j += 1
                }
            }
            35 => {
                strcpy(s, key_code);
                s = s.offset(strlen(key_code) as isize)
            }
            _ => { let fresh6 = s; s = s.offset(1); *fresh6 = ch }
        }
        i += 1
    }
    let fresh7 = s;
    s = s.offset(1);
    *fresh7 = 13 as libc::c_int as libc::c_char;
    *bufptr = s;
    *strptr = str;
}
/*
* Hack -- convert a printable string into real ascii
*
* I have no clue if this function correctly handles, for example,
* parsing "\xFF" into a (signed) char.  Whoever thought of making
* the "sign" of a "char" undefined is a complete moron.  Oh well.
*/
#[no_mangle]
pub unsafe extern "C" fn text_to_ascii(mut buf: *mut libc::c_char,
                                       mut str: cptr) {
    let mut s: *mut libc::c_char = buf;
    /* Analyze the "ascii" string */
    while *str != 0 {
        /* Backslash codes */
        if *str as libc::c_int == '\\' as i32 {
            /* Skip the backslash */
            str = str.offset(1);
            /* Macro Trigger */
            if *str as libc::c_int == '[' as i32 {
                trigger_text_to_ascii(&mut s, &mut str);
            } else if *str as libc::c_int == 'x' as i32 {
                str = str.offset(1);
                *s = (16 as libc::c_int * dehex(*str)) as libc::c_char;
                str = str.offset(1);
                let fresh8 = s;
                s = s.offset(1);
                *fresh8 =
                    (*fresh8 as libc::c_int + dehex(*str)) as libc::c_char
            } else if *str as libc::c_int == '\\' as i32 {
                let fresh9 = s;
                s = s.offset(1);
                *fresh9 = '\\' as i32 as libc::c_char
            } else if *str as libc::c_int == '^' as i32 {
                let fresh10 = s;
                s = s.offset(1);
                *fresh10 = '^' as i32 as libc::c_char
            } else if *str as libc::c_int == 's' as i32 {
                let fresh11 = s;
                s = s.offset(1);
                *fresh11 = ' ' as i32 as libc::c_char
            } else if *str as libc::c_int == 'e' as i32 {
                let fresh12 = s;
                s = s.offset(1);
                *fresh12 = '\u{1b}' as i32 as libc::c_char
            } else if *str as libc::c_int == 'b' as i32 {
                let fresh13 = s;
                s = s.offset(1);
                *fresh13 = '\u{8}' as i32 as libc::c_char
            } else if *str as libc::c_int == 'n' as i32 {
                let fresh14 = s;
                s = s.offset(1);
                *fresh14 = '\n' as i32 as libc::c_char
            } else if *str as libc::c_int == 'r' as i32 {
                let fresh15 = s;
                s = s.offset(1);
                *fresh15 = '\r' as i32 as libc::c_char
            } else if *str as libc::c_int == 't' as i32 {
                let fresh16 = s;
                s = s.offset(1);
                *fresh16 = '\t' as i32 as libc::c_char
            } else if *str as libc::c_int == '0' as i32 {
                str = str.offset(1);
                *s = (8 as libc::c_int * deoct(*str)) as libc::c_char;
                str = str.offset(1);
                let fresh17 = s;
                s = s.offset(1);
                *fresh17 =
                    (*fresh17 as libc::c_int + deoct(*str)) as libc::c_char
            } else if *str as libc::c_int == '1' as i32 {
                str = str.offset(1);
                *s =
                    (64 as libc::c_int + 8 as libc::c_int * deoct(*str)) as
                        libc::c_char;
                str = str.offset(1);
                let fresh18 = s;
                s = s.offset(1);
                *fresh18 =
                    (*fresh18 as libc::c_int + deoct(*str)) as libc::c_char
            } else if *str as libc::c_int == '2' as i32 {
                str = str.offset(1);
                *s =
                    (64 as libc::c_int * 2 as libc::c_int +
                         8 as libc::c_int * deoct(*str)) as libc::c_char;
                str = str.offset(1);
                let fresh19 = s;
                s = s.offset(1);
                *fresh19 =
                    (*fresh19 as libc::c_int + deoct(*str)) as libc::c_char
            } else if *str as libc::c_int == '3' as i32 {
                str = str.offset(1);
                *s =
                    (64 as libc::c_int * 3 as libc::c_int +
                         8 as libc::c_int * deoct(*str)) as libc::c_char;
                str = str.offset(1);
                let fresh20 = s;
                s = s.offset(1);
                *fresh20 =
                    (*fresh20 as libc::c_int + deoct(*str)) as libc::c_char
            }
            /* Hex-mode XXX */
            /* Hack -- simple way to specify "backslash" */
            /* Hack -- simple way to specify "caret" */
            /* Hack -- simple way to specify "space" */
            /* Hack -- simple way to specify Escape */
            /* Backspace */
            /* Newline */
            /* Return */
            /* Tab */
            /* Octal-mode */
            /* Octal-mode */
            /* Octal-mode */
            /* Octal-mode */
            /* Skip the final char */
            str = str.offset(1)
        } else if *str as libc::c_int == '^' as i32 {
            str = str.offset(1);
            let fresh21 = str;
            str = str.offset(1);
            let fresh22 = s;
            s = s.offset(1);
            *fresh22 =
                (*fresh21 as libc::c_int & 0o37 as libc::c_int) as
                    libc::c_char
        } else {
            /* Normal Control codes */
            /* Normal chars */
            let fresh23 = str;
            str = str.offset(1);
            let fresh24 = s;
            s = s.offset(1);
            *fresh24 = *fresh23
        }
    }
    /* Terminate */
    *s = '\u{0}' as i32 as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn trigger_ascii_to_text(mut bufptr:
                                                   *mut *mut libc::c_char,
                                               mut strptr: *mut cptr)
 -> bool_ {
    let mut s: *mut libc::c_char = *bufptr;
    let mut str: cptr = *strptr;
    let mut key_code: [libc::c_char; 100] = [0; 100];
    let mut i: libc::c_int = 0;
    let mut tmp: cptr = 0 as *const libc::c_char;
    if macro_template.is_null() { return 0 as libc::c_int as bool_ }
    let fresh25 = s;
    s = s.offset(1);
    *fresh25 = '\\' as i32 as libc::c_char;
    let fresh26 = s;
    s = s.offset(1);
    *fresh26 = '[' as i32 as libc::c_char;
    i = 0 as libc::c_int;
    while *macro_template.offset(i as isize) != 0 {
        let mut j: libc::c_int = 0;
        let mut ch: libc::c_char = *macro_template.offset(i as isize);
        match ch as libc::c_int {
            38 => {
                loop  {
                    tmp =
                        strchr(macro_modifier_chr, *str as libc::c_int) as
                            cptr;
                    if tmp.is_null() { break ; }
                    j =
                        tmp.wrapping_offset_from(macro_modifier_chr) as
                            libc::c_long as libc::c_int;
                    tmp = macro_modifier_name[j as usize] as cptr;
                    while *tmp != 0 {
                        let fresh27 = tmp;
                        tmp = tmp.offset(1);
                        let fresh28 = s;
                        s = s.offset(1);
                        *fresh28 = *fresh27
                    }
                    str = str.offset(1)
                }
            }
            35 => {
                j = 0 as libc::c_int;
                while *str as libc::c_int != 0 &&
                          *str as libc::c_int !=
                              13 as libc::c_int as libc::c_char as libc::c_int
                      {
                    let fresh29 = str;
                    str = str.offset(1);
                    key_code[j as usize] = *fresh29;
                    j += 1
                }
                key_code[j as usize] = '\u{0}' as i32 as libc::c_char
            }
            _ => {
                if ch as libc::c_int != *str as libc::c_int {
                    return 0 as libc::c_int as bool_
                }
                str = str.offset(1)
            }
        }
        i += 1
    }
    let fresh30 = str;
    str = str.offset(1);
    if *fresh30 as libc::c_int !=
           13 as libc::c_int as libc::c_char as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    i = 0 as libc::c_int;
    while i < max_macrotrigger {
        if my_stricmp(key_code.as_mut_ptr() as cptr,
                      macro_trigger_keycode[0 as libc::c_int as
                                                usize][i as usize] as cptr) ==
               0 ||
               my_stricmp(key_code.as_mut_ptr() as cptr,
                          macro_trigger_keycode[1 as libc::c_int as
                                                    usize][i as usize] as
                              cptr) == 0 {
            break ;
        }
        i += 1
    }
    if i == max_macrotrigger { return 0 as libc::c_int as bool_ }
    tmp = macro_trigger_name[i as usize] as cptr;
    while *tmp != 0 {
        let fresh31 = tmp;
        tmp = tmp.offset(1);
        let fresh32 = s;
        s = s.offset(1);
        *fresh32 = *fresh31
    }
    let fresh33 = s;
    s = s.offset(1);
    *fresh33 = ']' as i32 as libc::c_char;
    *bufptr = s;
    *strptr = str;
    return 1 as libc::c_int as bool_;
}
/*
* Hack -- convert a string into a printable form
*/
#[no_mangle]
pub unsafe extern "C" fn ascii_to_text(mut buf: *mut libc::c_char,
                                       mut str: cptr) {
    let mut s: *mut libc::c_char = buf;
    /* Analyze the "ascii" string */
    while *str != 0 {
        let fresh34 = str;
        str = str.offset(1);
        let mut i: byte_hack = *fresh34 as byte_hack;
        /* Macro Trigger */
        if i as libc::c_int == 31 as libc::c_int {
            if trigger_ascii_to_text(&mut s, &mut str) == 0 {
                let fresh35 = s;
                s = s.offset(1);
                *fresh35 = '^' as i32 as libc::c_char;
                let fresh36 = s;
                s = s.offset(1);
                *fresh36 = '_' as i32 as libc::c_char
            }
        } else if i as libc::c_int == '\u{1b}' as i32 {
            let fresh37 = s;
            s = s.offset(1);
            *fresh37 = '\\' as i32 as libc::c_char;
            let fresh38 = s;
            s = s.offset(1);
            *fresh38 = 'e' as i32 as libc::c_char
        } else if i as libc::c_int == ' ' as i32 {
            let fresh39 = s;
            s = s.offset(1);
            *fresh39 = '\\' as i32 as libc::c_char;
            let fresh40 = s;
            s = s.offset(1);
            *fresh40 = 's' as i32 as libc::c_char
        } else if i as libc::c_int == '\u{8}' as i32 {
            let fresh41 = s;
            s = s.offset(1);
            *fresh41 = '\\' as i32 as libc::c_char;
            let fresh42 = s;
            s = s.offset(1);
            *fresh42 = 'b' as i32 as libc::c_char
        } else if i as libc::c_int == '\t' as i32 {
            let fresh43 = s;
            s = s.offset(1);
            *fresh43 = '\\' as i32 as libc::c_char;
            let fresh44 = s;
            s = s.offset(1);
            *fresh44 = 't' as i32 as libc::c_char
        } else if i as libc::c_int == '\n' as i32 {
            let fresh45 = s;
            s = s.offset(1);
            *fresh45 = '\\' as i32 as libc::c_char;
            let fresh46 = s;
            s = s.offset(1);
            *fresh46 = 'n' as i32 as libc::c_char
        } else if i as libc::c_int == '\r' as i32 {
            let fresh47 = s;
            s = s.offset(1);
            *fresh47 = '\\' as i32 as libc::c_char;
            let fresh48 = s;
            s = s.offset(1);
            *fresh48 = 'r' as i32 as libc::c_char
        } else if i as libc::c_int == '^' as i32 {
            let fresh49 = s;
            s = s.offset(1);
            *fresh49 = '\\' as i32 as libc::c_char;
            let fresh50 = s;
            s = s.offset(1);
            *fresh50 = '^' as i32 as libc::c_char
        } else if i as libc::c_int == '\\' as i32 {
            let fresh51 = s;
            s = s.offset(1);
            *fresh51 = '\\' as i32 as libc::c_char;
            let fresh52 = s;
            s = s.offset(1);
            *fresh52 = '\\' as i32 as libc::c_char
        } else if (i as libc::c_int) < 32 as libc::c_int {
            let fresh53 = s;
            s = s.offset(1);
            *fresh53 = '^' as i32 as libc::c_char;
            let fresh54 = s;
            s = s.offset(1);
            *fresh54 = (i as libc::c_int + 64 as libc::c_int) as libc::c_char
        } else if (i as libc::c_int) < 127 as libc::c_int {
            let fresh55 = s;
            s = s.offset(1);
            *fresh55 = i as libc::c_char
        } else if (i as libc::c_int) < 64 as libc::c_int {
            let fresh56 = s;
            s = s.offset(1);
            *fresh56 = '\\' as i32 as libc::c_char;
            let fresh57 = s;
            s = s.offset(1);
            *fresh57 = '0' as i32 as libc::c_char;
            let fresh58 = s;
            s = s.offset(1);
            *fresh58 =
                octify((i as libc::c_int / 8 as libc::c_int) as uint_hack);
            let fresh59 = s;
            s = s.offset(1);
            *fresh59 =
                octify((i as libc::c_int % 8 as libc::c_int) as uint_hack)
        } else {
            let fresh60 = s;
            s = s.offset(1);
            *fresh60 = '\\' as i32 as libc::c_char;
            let fresh61 = s;
            s = s.offset(1);
            *fresh61 = 'x' as i32 as libc::c_char;
            let fresh62 = s;
            s = s.offset(1);
            *fresh62 =
                hexify((i as libc::c_int / 16 as libc::c_int) as uint_hack);
            let fresh63 = s;
            s = s.offset(1);
            *fresh63 =
                hexify((i as libc::c_int % 16 as libc::c_int) as uint_hack)
        }
    }
    /* Terminate */
    *s = '\u{0}' as i32 as libc::c_char;
}
/*
* The "macro" package
*
* Functions are provided to manipulate a collection of macros, each
* of which has a trigger pattern string and a resulting action string
* and a small set of flags.
*/
/*
* Determine if any macros have ever started with a given character.
*/
static mut macro__use: [bool_; 256] = [0; 256];
/*
* Find the macro (if any) which exactly matches the given pattern
*/
#[no_mangle]
pub unsafe extern "C" fn macro_find_exact(mut pat: cptr) -> sint {
    let mut i: libc::c_int = 0;
    /* Nothing possible */
    if macro__use[*pat.offset(0 as libc::c_int as isize) as byte_hack as
                      usize] == 0 {
        return -(1 as libc::c_int)
    }
    /* Scan the macros */
    i = 0 as libc::c_int;
    while i < macro__num as libc::c_int {
        /* Skip macros which do not match the pattern */
        if streq(*macro__pat.offset(i as isize), pat) == 0 {
            i += 1
        } else {
            /* Found one */
            return i
        }
    }
    /* No matches */
    return -(1 as libc::c_int);
}
/*
* Find the first macro (if any) which contains the given pattern
*/
unsafe extern "C" fn macro_find_check(mut pat: cptr) -> sint {
    let mut i: libc::c_int = 0;
    /* Nothing possible */
    if macro__use[*pat.offset(0 as libc::c_int as isize) as byte_hack as
                      usize] == 0 {
        return -(1 as libc::c_int)
    }
    /* Scan the macros */
    i = 0 as libc::c_int;
    while i < macro__num as libc::c_int {
        /* Skip macros which do not contain the pattern */
        if prefix(*macro__pat.offset(i as isize), pat) == 0 {
            i += 1
        } else {
            /* Found one */
            return i
        }
    }
    /* Nothing */
    return -(1 as libc::c_int);
}
/*
* Find the first macro (if any) which contains the given pattern and more
*/
unsafe extern "C" fn macro_find_maybe(mut pat: cptr) -> sint {
    let mut i: libc::c_int = 0;
    /* Nothing possible */
    if macro__use[*pat.offset(0 as libc::c_int as isize) as byte_hack as
                      usize] == 0 {
        return -(1 as libc::c_int)
    }
    /* Scan the macros */
    i = 0 as libc::c_int;
    while i < macro__num as libc::c_int {
        /* Skip macros which do not contain the pattern */
        if !(prefix(*macro__pat.offset(i as isize), pat) == 0) {
            /* Skip macros which exactly match the pattern XXX XXX */
            if !(streq(*macro__pat.offset(i as isize), pat) != 0) {
                /* Found one */
                return i
            }
        }
        i += 1
    }
    /* Nothing */
    return -(1 as libc::c_int);
}
/*
* Find the longest macro (if any) which starts with the given pattern
*/
unsafe extern "C" fn macro_find_ready(mut pat: cptr) -> sint {
    let mut i: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut n: libc::c_int = -(1 as libc::c_int);
    let mut s: libc::c_int = -(1 as libc::c_int);
    /* Nothing possible */
    if macro__use[*pat.offset(0 as libc::c_int as isize) as byte_hack as
                      usize] == 0 {
        return -(1 as libc::c_int)
    }
    /* Scan the macros */
    i = 0 as libc::c_int;
    while i < macro__num as libc::c_int {
        /* Skip macros which are not contained by the pattern */
        if !(prefix(pat, *macro__pat.offset(i as isize)) == 0) {
            /* Obtain the length of this macro */
            t = strlen(*macro__pat.offset(i as isize)) as libc::c_int;
            /* Only track the "longest" pattern */
            if !(n >= 0 as libc::c_int && s > t) {
                /* Track the entry */
                n = i;
                s = t
            }
        }
        i += 1
    }
    /* Result */
    return n;
}
/*
* Add a macro definition (or redefinition).
*
* We should use "act == NULL" to "remove" a macro, but this might make it
* impossible to save the "removal" of a macro definition.  XXX XXX XXX
*
* We should consider refusing to allow macros which contain existing macros,
* or which are contained in existing macros, because this would simplify the
* macro analysis code.  XXX XXX XXX
*
* We should consider removing the "command macro" crap, and replacing it
* with some kind of "powerful keymap" ability, but this might make it hard
* to change the "roguelike" option from inside the game.  XXX XXX XXX
*/
#[no_mangle]
pub unsafe extern "C" fn macro_add(mut pat: cptr, mut act: cptr) -> errr {
    let mut n: libc::c_int = 0;
    /* Paranoia -- require data */
    if pat.is_null() || act.is_null() { return -(1 as libc::c_int) }
    /* Look for any existing macro */
    n = macro_find_exact(pat);
    /* Replace existing macro */
    if n >= 0 as libc::c_int {
        /* Free the old macro action */
        string_free(*macro__act.offset(n as isize));
    } else {
        /* Create a new macro */
        /* Acquire a new index */
        let fresh64 = macro__num;
        macro__num = macro__num + 1;
        n = fresh64 as libc::c_int;
        let ref mut fresh65 = *macro__pat.offset(n as isize);
        *fresh65 = string_make(pat)
    }
    /* Save the pattern */
    /* Save the action */
    let ref mut fresh66 = *macro__act.offset(n as isize);
    *fresh66 = string_make(act);
    /* Efficiency */
    macro__use[*pat.offset(0 as libc::c_int as isize) as byte_hack as usize] =
        1 as libc::c_int as bool_;
    /* Success */
    return 0 as libc::c_int;
}
/*
* Initialize the "macro" package
*/
#[no_mangle]
pub unsafe extern "C" fn macro_init() -> errr {
    /* Macro patterns */
    macro__pat =
        memset(ralloc((256 as libc::c_int as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<cptr>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (256 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<cptr>()
                                                    as libc::c_ulong)) as
            *mut cptr;
    /* Macro actions */
    macro__act =
        memset(ralloc((256 as libc::c_int as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<cptr>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (256 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<cptr>()
                                                    as libc::c_ulong)) as
            *mut cptr;
    /* Success */
    return 0 as libc::c_int;
}
/*
* Local "need flush" variable
*/
static mut flush_later: bool_ = 0 as libc::c_int as bool_;
/*
* Local variable -- we are inside a "macro action"
*
* Do not match any macros until "ascii 30" is found.
*/
static mut parse_macro: bool_ = 0 as libc::c_int as bool_;
/*
* Local variable -- we are inside a "macro trigger"
*
* Strip all keypresses until a low ascii value is found.
*/
static mut parse_under: bool_ = 0 as libc::c_int as bool_;
/*
* Flush all input chars.  Actually, remember the flush,
* and do a "special flush" before the next "inkey()".
*
* This is not only more efficient, but also necessary to make sure
* that various "inkey()" codes are not "lost" along the way.
*/
#[no_mangle]
pub unsafe extern "C" fn flush() {
    /* Do it later */
    flush_later = 1 as libc::c_int as bool_;
}
/*
* Flush the screen, make a noise
*/
#[no_mangle]
pub unsafe extern "C" fn bell() {
    /* Mega-Hack -- Flush the output */
    Term_fresh();
    /* Make a bell noise (if allowed) */
    if ring_bell != 0 { Term_xtra(7 as libc::c_int, 0 as libc::c_int); }
    /* Flush the input (later!) */
    flush();
}
/*
* Hack -- Make a (relevant?) sound
*/
#[no_mangle]
pub unsafe extern "C" fn sound(mut val: libc::c_int) {
    /* No sound */
    if use_sound == 0 { return }
    /* Make a sound (if allowed) */
    Term_xtra(8 as libc::c_int, val);
}
/*
* Helper function called only from "inkey()"
*
* This function does almost all of the "macro" processing.
*
* We use the "Term_key_push()" function to handle "failed" macros, as well
* as "extra" keys read in while choosing the proper macro, and also to hold
* the action for the macro, plus a special "ascii 30" character indicating
* that any macro action in progress is complete.  Embedded macros are thus
* illegal, unless a macro action includes an explicit "ascii 30" character,
* which would probably be a massive hack, and might break things.
*
* Only 500 (0+1+2+...+29+30) milliseconds may elapse between each key in
* the macro trigger sequence.  If a key sequence forms the "prefix" of a
* macro trigger, 500 milliseconds must pass before the key sequence is
* known not to be that macro trigger.  XXX XXX XXX
*/
unsafe extern "C" fn inkey_aux() -> libc::c_char {
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0;
    let mut p: libc::c_int = 0 as libc::c_int;
    let mut w: libc::c_int = 0 as libc::c_int;
    let mut ch: libc::c_char = 0;
    let mut pat: cptr = 0 as *const libc::c_char;
    let mut act: cptr = 0 as *const libc::c_char;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Wait for a keypress */
    Term_inkey(&mut ch, 1 as libc::c_int as bool_, 1 as libc::c_int as bool_);
    /* End "macro action" */
    if ch as libc::c_int == 30 as libc::c_int {
        parse_macro = 0 as libc::c_int as bool_
    }
    /* Inside "macro action" */
    if ch as libc::c_int == 30 as libc::c_int { return ch }
    /* Inside "macro action" */
    if parse_macro != 0 { return ch }
    /* Inside "macro trigger" */
    if parse_under != 0 { return ch }
    /* Save the first key, advance */
    let fresh67 = p;
    p = p + 1;
    buf[fresh67 as usize] = ch;
    buf[p as usize] = '\u{0}' as i32 as libc::c_char;
    /* Check for possible macro */
    k = macro_find_check(buf.as_mut_ptr() as cptr);
    /* No macro pending */
    if k < 0 as libc::c_int { return ch }
    loop 
         /* Wait for a macro, or a timeout */
         /* Check for pending macro */
         {
        k = macro_find_maybe(buf.as_mut_ptr() as cptr);
        /* No macro pending */
        if k < 0 as libc::c_int { break ; }
        /* Check for (and remove) a pending key */
        if 0 as libc::c_int ==
               Term_inkey(&mut ch, 0 as libc::c_int as bool_,
                          1 as libc::c_int as bool_) {
            /* Append the key */
            let fresh68 = p;
            p = p + 1;
            buf[fresh68 as usize] = ch;
            buf[p as usize] = '\u{0}' as i32 as libc::c_char;
            /* Restart wait */
            w = 0 as libc::c_int
        } else {
            /* No key ready */
            /* Increase "wait" */
            w += 10 as libc::c_int;
            /* Excessive delay */
            if w >= 100 as libc::c_int { break ; }
            /* Delay */
            Term_xtra(13 as libc::c_int, w);
        }
    }
    /* Check for available macro */
    k = macro_find_ready(buf.as_mut_ptr() as cptr);
    /* No macro available */
    if k < 0 as libc::c_int {
        /* Push all the keys back on the queue */
        while p > 0 as libc::c_int {
            /* Push the key, notice over-flow */
            p -= 1;
            if Term_key_push(buf[p as usize] as libc::c_int) != 0 {
                return 0 as libc::c_int as libc::c_char
            }
        }
        /* Wait for (and remove) a pending key */
        Term_inkey(&mut ch, 1 as libc::c_int as bool_,
                   1 as libc::c_int as bool_);
        /* Return the key */
        return ch
    }
    /* Get the pattern */
    pat = *macro__pat.offset(k as isize);
    /* Get the length of the pattern */
    n = strlen(pat) as libc::c_int;
    /* Push the "extra" keys back on the queue */
    while p > n {
        /* Push the key, notice over-flow */
        p -= 1;
        if Term_key_push(buf[p as usize] as libc::c_int) != 0 {
            return 0 as libc::c_int as libc::c_char
        }
    }
    /* Begin "macro action" */
    parse_macro = 1 as libc::c_int as bool_;
    /* Push the "end of macro action" key */
    if Term_key_push(30 as libc::c_int) != 0 {
        return 0 as libc::c_int as libc::c_char
    }
    /* Access the macro action */
    act = *macro__act.offset(k as isize);
    /* Get the length of the action */
    n = strlen(act) as libc::c_int;
    /* Push the macro "action" onto the key queue */
    while n > 0 as libc::c_int {
        /* Push the key, notice over-flow */
        n -= 1;
        if Term_key_push(*act.offset(n as isize) as libc::c_int) != 0 {
            return 0 as libc::c_int as libc::c_char
        }
    }
    /* Hack -- Force "inkey()" to call us again */
    return 0 as libc::c_int as libc::c_char;
}
/*
* Mega-Hack -- special "inkey_next" pointer.  XXX XXX XXX
*
* This special pointer allows a sequence of keys to be "inserted" into
* the stream of keys returned by "inkey()".  This key sequence will not
* trigger any macros, and cannot be bypassed by the Borg.  It is used
* in Angband to handle "keymaps".
*/
static mut inkey_next: cptr = 0 as cptr;
/*
* Get a keypress from the user.
*
* This function recognizes a few "global parameters".  These are variables
* which, if set to TRUE before calling this function, will have an effect
* on this function, and which are always reset to FALSE by this function
* before this function returns.  Thus they function just like normal
* parameters, except that most calls to this function can ignore them.
*
* If "inkey_xtra" is TRUE, then all pending keypresses will be flushed,
* and any macro processing in progress will be aborted.  This flag is
* set by the "flush()" function, which does not actually flush anything
* itself, but rather, triggers delayed input flushing via "inkey_xtra".
*
* If "inkey_scan" is TRUE, then we will immediately return "zero" if no
* keypress is available, instead of waiting for a keypress.
*
* If "inkey_base" is TRUE, then all macro processing will be bypassed.
* If "inkey_base" and "inkey_scan" are both TRUE, then this function will
* not return immediately, but will wait for a keypress for as long as the
* normal macro matching code would, allowing the direct entry of macro
* triggers.  The "inkey_base" flag is extremely dangerous!
*
* If "inkey_flag" is TRUE, then we will assume that we are waiting for a
* normal command, and we will only show the cursor if "hilite_player" is
* TRUE (or if the player is in a store), instead of always showing the
* cursor.  The various "main-xxx.c" files should avoid saving the game
* in response to a "menu item" request unless "inkey_flag" is TRUE, to
* prevent savefile corruption.
*
* If we are waiting for a keypress, and no keypress is ready, then we will
* refresh (once) the window which was active when this function was called.
*
* Note that "back-quote" is automatically converted into "escape" for
* convenience on machines with no "escape" key.  This is done after the
* macro matching, so the user can still make a macro for "backquote".
*
* Note the special handling of "ascii 30" (ctrl-caret, aka ctrl-shift-six)
* and "ascii 31" (ctrl-underscore, aka ctrl-shift-minus), which are used to
* provide support for simple keyboard "macros".  These keys are so strange
* that their loss as normal keys will probably be noticed by nobody.  The
* "ascii 30" key is used to indicate the "end" of a macro action, which
* allows recursive macros to be avoided.  The "ascii 31" key is used by
* some of the "main-xxx.c" files to introduce macro trigger sequences.
*
* Hack -- we use "ascii 29" (ctrl-right-bracket) as a special "magic" key,
* which can be used to give a variety of "sub-commands" which can be used
* any time.  These sub-commands could include commands to take a picture of
* the current screen, to start/stop recording a macro action, etc.
*
* If "angband_term[0]" is not active, we will make it active during this
* function, so that the various "main-xxx.c" files can assume that input
* is only requested (via "Term_inkey()") when "angband_term[0]" is active.
*
* Mega-Hack -- This function is used as the entry point for clearing the
* "signal_count" variable, and of the "character_saved" variable.
*
* Hack -- Note the use of "inkey_next" to allow "keymaps" to be processed.
*
* Mega-Hack -- Note the use of "inkey_hack" to allow the "Borg" to steal
* control of the keyboard from the user.
*/
#[no_mangle]
pub unsafe extern "C" fn inkey() -> libc::c_char {
    let mut v: libc::c_int = 0;
    let mut kk: libc::c_char = 0;
    let mut ch: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut done: bool_ = 0 as libc::c_int as bool_;
    let mut old: *mut term = Term;
    /* Hack -- Use the "inkey_next" pointer */
    if !inkey_next.is_null() && *inkey_next as libc::c_int != 0 &&
           inkey_xtra == 0 {
        /* Get next character, and advance */
        let fresh69 = inkey_next;
        inkey_next = inkey_next.offset(1);
        ch = *fresh69;
        /* Cancel the various "global parameters" */
        inkey_scan = 0 as libc::c_int as bool_;
        inkey_flag = inkey_scan;
        inkey_xtra = inkey_flag;
        inkey_base = inkey_xtra;
        /* Accept result */
        macro_recorder_add(ch);
        return ch
    }
    /* Forget pointer */
    inkey_next = 0 as cptr;
    /* Hack -- handle delayed "flush()" */
    if inkey_xtra != 0 {
        /* End "macro action" */
        parse_macro = 0 as libc::c_int as bool_;
        /* End "macro trigger" */
        parse_under = 0 as libc::c_int as bool_;
        /* Forget old keypresses */
        Term_flush();
    }
    /* Access cursor state */
    Term_get_cursor(&mut v);
    /* Show the cursor if waiting, except sometimes in "command" mode */
    if inkey_scan == 0 &&
           (inkey_flag == 0 || hilite_player as libc::c_int != 0 ||
                character_icky as libc::c_int != 0) {
        /* Show the cursor */
        Term_set_cursor(1 as libc::c_int);
    }
    /* Hack -- Activate main screen */
    Term_activate(angband_term[0 as libc::c_int as usize]);
    /* Get a key */
    while ch == 0 {
        /* Hack -- Handle "inkey_scan" */
        if inkey_base == 0 && inkey_scan as libc::c_int != 0 &&
               0 as libc::c_int !=
                   Term_inkey(&mut kk, 0 as libc::c_int as bool_,
                              0 as libc::c_int as bool_) {
            break ;
        }
        /* Hack -- Flush output once when no key ready */
        if done == 0 &&
               0 as libc::c_int !=
                   Term_inkey(&mut kk, 0 as libc::c_int as bool_,
                              0 as libc::c_int as bool_) {
            /* Hack -- activate proper term */
            Term_activate(old);
            /* Flush output */
            Term_fresh();
            /* Hack -- activate main screen */
            Term_activate(angband_term[0 as libc::c_int as usize]);
            /* Mega-Hack -- reset saved flag */
            character_saved = 0 as libc::c_int as bool_;
            /* Mega-Hack -- reset signal counter */
            signal_count = 0 as libc::c_int as s16b;
            /* Only once */
            done = 1 as libc::c_int as bool_
        }
        /* Hack -- Handle "inkey_base" */
        if inkey_base != 0 {
            let mut w: libc::c_int = 0 as libc::c_int;
            /* Wait forever */
            if inkey_scan == 0 {
                /* Wait for (and remove) a pending key */
                if 0 as libc::c_int ==
                       Term_inkey(&mut ch, 1 as libc::c_int as bool_,
                                  1 as libc::c_int as bool_) {
                    /* Done */
                    break ;
                } else {
                    /* Oops */
                    break ;
                }
            } else {
                /* Wait */
                /* Check for (and remove) a pending key */
                while !(0 as libc::c_int ==
                            Term_inkey(&mut ch, 0 as libc::c_int as bool_,
                                       1 as libc::c_int as bool_)) {
                    /* No key ready */
                    /* Increase "wait" */
                    w += 10 as libc::c_int;
                    /* Excessive delay */
                    if w >= 100 as libc::c_int { break ; }
                    /* Delay */
                    Term_xtra(13 as libc::c_int, w);
                }
                break ;
            }
        } else {
            /* Get a key (see above) */
            ch = inkey_aux();
            /* Handle "control-right-bracket" */
            if ch as libc::c_int == 29 as libc::c_int ||
                   rogue_like_commands == 0 &&
                       ch as libc::c_int == 'D' as i32 & 0x1f as libc::c_int {
                /* Strip this key */
                ch = 0 as libc::c_int as libc::c_char;
                if do_movies == 0 {
                    /* Do an html dump */
                    do_cmd_html_dump();
                } else {
                    /* Do a text box in the cmovie */
                    do_cmovie_insert();
                }
            } else {
                /* Treat back-quote as escape */
                if ch as libc::c_int == '`' as i32 {
                    ch = '\u{1b}' as i32 as libc::c_char
                }
                /* End "macro trigger" */
                if parse_under as libc::c_int != 0 &&
                       ch as libc::c_int <= 32 as libc::c_int {
                    /* Strip this key */
                    ch = 0 as libc::c_int as libc::c_char;
                    /* End "macro trigger" */
                    parse_under = 0 as libc::c_int as bool_
                }
                /* Handle "control-caret" */
                if ch as libc::c_int == 30 as libc::c_int {
                    /* Strip this key */
                    ch = 0 as libc::c_int as libc::c_char
                } else if ch as libc::c_int == 31 as libc::c_int {
                    /* Handle "control-underscore" */
                    /* Strip this key */
                    ch = 0 as libc::c_int as libc::c_char;
                    /* Begin "macro trigger" */
                    parse_under = 1 as libc::c_int as bool_
                } else if parse_under != 0 {
                    /* Inside "macro trigger" */
                    /* Strip this key */
                    ch = 0 as libc::c_int as libc::c_char
                }
            }
        }
    }
    /* Hack -- restore the term */
    Term_activate(old);
    /* Restore the cursor */
    Term_set_cursor(v);
    /* Cancel the various "global parameters" */
    inkey_scan = 0 as libc::c_int as bool_;
    inkey_flag = inkey_scan;
    inkey_xtra = inkey_flag;
    inkey_base = inkey_xtra;
    /* Return the keypress */
    macro_recorder_add(ch);
    return ch;
}
/*
* We use a global array for all inscriptions to reduce the memory
* spent maintaining inscriptions.  Of course, it is still possible
* to run out of inscription memory, especially if too many different
* inscriptions are used, but hopefully this will be rare.
*
* We use dynamic string allocation because otherwise it is necessary
* to pre-guess the amount of quark activity.  We limit the total
* number of quarks, but this is much easier to "expand" as needed.
*
* Any two items with the same inscription will have the same "quark"
* index, which should greatly reduce the need for inscription space.
*
* Note that "quark zero" is NULL and should not be "dereferenced".
*/
/*
* Add a new "quark" to the set of quarks.
*/
#[no_mangle]
pub unsafe extern "C" fn quark_add(mut str: cptr) -> s16b {
    let mut i: libc::c_int = 0;
    /* Look for an existing quark */
    i = 1 as libc::c_int;
    while i < quark__num as libc::c_int {
        /* Check for equality */
        if streq(*quark__str.offset(i as isize), str) != 0 {
            return i as s16b
        }
        i += 1
    }
    /* Paranoia -- Require room */
    if quark__num as libc::c_int == 768 as libc::c_int {
        return 0 as libc::c_int as s16b
    }
    /* New maximal quark */
    quark__num = (i + 1 as libc::c_int) as s16b;
    /* Add a new quark */
    let ref mut fresh70 = *quark__str.offset(i as isize);
    *fresh70 = string_make(str);
    /* Return the index */
    return i as s16b;
}
/*
* This function looks up a quark
*/
#[no_mangle]
pub unsafe extern "C" fn quark_str(mut i: s16b) -> cptr {
    let mut q: cptr = 0 as *const libc::c_char;
    /* Verify */
    if (i as libc::c_int) < 0 as libc::c_int ||
           i as libc::c_int >= quark__num as libc::c_int {
        i = 0 as libc::c_int as s16b
    }
    /* Access the quark */
    q = *quark__str.offset(i as isize);
    /* Return the quark */
    return q;
}
/*
* Second try for the "message" handling routines.
*
* Each call to "message_add(s)" will add a new "most recent" message
* to the "message recall list", using the contents of the string "s".
*
* The messages will be stored in such a way as to maximize "efficiency",
* that is, we attempt to maximize the number of sequential messages that
* can be retrieved, given a limited amount of storage space.
*
* We keep a buffer of chars to hold the "text" of the messages, not
* necessarily in "order", and an array of offsets into that buffer,
* representing the actual messages.  This is made more complicated
* by the fact that both the array of indexes, and the buffer itself,
* are both treated as "circular arrays" for efficiency purposes, but
* the strings may not be "broken" across the ends of the array.
*
* The "message_add()" function is rather "complex", because it must be
* extremely efficient, both in space and time, for use with the Borg.
*/
/*
* How many messages are "available"?
*/
#[no_mangle]
pub unsafe extern "C" fn message_num() -> s16b {
    let mut last: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    /* Extract the indexes */
    last = message__last as libc::c_int;
    next = message__next as libc::c_int;
    /* Handle "wrap" */
    if next < last { next += 2048 as libc::c_int }
    /* Extract the space */
    n = next - last;
    /* Return the result */
    return n as s16b;
}
/*
* Recall the "text" of a saved message
*/
#[no_mangle]
pub unsafe extern "C" fn message_str(mut age: libc::c_int) -> cptr {
    static mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut x: s16b = 0;
    let mut o: s16b = 0;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Forgotten messages have no text */
    if age < 0 as libc::c_int || age >= message_num() as libc::c_int {
        return b"\x00" as *const u8 as *const libc::c_char
    }
    /* Acquire the "logical" index */
    x =
        ((message__next as libc::c_int + 2048 as libc::c_int -
              (age + 1 as libc::c_int)) % 2048 as libc::c_int) as s16b;
    /* Get the "offset" for the message */
    o = *message__ptr.offset(x as isize) as s16b;
    /* Access the message text */
    s = &mut *message__buf.offset(o as isize) as *mut libc::c_char as cptr;
    /* Hack -- Handle repeated messages */
    if *message__count.offset(x as isize) as libc::c_int > 1 as libc::c_int {
        strnfmt(buf.as_mut_ptr(), 1024 as libc::c_int as uint_hack,
                b"%s <%dx>\x00" as *const u8 as *const libc::c_char, s,
                *message__count.offset(x as isize) as libc::c_int);
        s = buf.as_mut_ptr() as cptr
    }
    /* Return the message text */
    return s;
}
/*
* Recall the color of a saved message
*/
#[no_mangle]
pub unsafe extern "C" fn message_color(mut age: libc::c_int) -> byte_hack {
    let mut x: s16b = 0;
    let mut color: byte_hack = 1 as libc::c_int as byte_hack;
    /* Forgotten messages have no text */
    if age < 0 as libc::c_int || age >= message_num() as libc::c_int {
        return 1 as libc::c_int as byte_hack
    }
    /* Acquire the "logical" index */
    x =
        ((message__next as libc::c_int + 2048 as libc::c_int -
              (age + 1 as libc::c_int)) % 2048 as libc::c_int) as s16b;
    /* Get the "offset" for the message */
    color = *message__color.offset(x as isize);
    /* Return the message text */
    return color;
}
/*
 * Recall the type of a saved message
 */
#[no_mangle]
pub unsafe extern "C" fn message_type(mut age: libc::c_int) -> byte_hack {
    let mut x: s16b = 0;
    let mut type_0: byte_hack = 0;
    /* Forgotten messages have no text */
    if age < 0 as libc::c_int || age >= message_num() as libc::c_int {
        return 0 as libc::c_int as byte_hack
    }
    /* Acquire the "logical" index */
    x =
        ((message__next as libc::c_int + 2048 as libc::c_int -
              (age + 1 as libc::c_int)) % 2048 as libc::c_int) as s16b;
    /* Get the "offset" for the message */
    type_0 = *message__type.offset(x as isize);
    /* Return the message text */
    return type_0;
}
/*
* Add a new message, with great efficiency
*/
#[no_mangle]
pub unsafe extern "C" fn message_add(mut type_0: byte_hack, mut str: cptr,
                                     mut color: byte_hack) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut s: cptr = 0 as *const libc::c_char;
    /* ** Step 1 -- Analyze the message ***/
    /* Hack -- Ignore "non-messages" */
    if str.is_null() { return }
    /* Message length */
    n = strlen(str) as libc::c_int;
    /* Important Hack -- Ignore "long" messages */
    if n >= 32768 as libc::c_int / 4 as libc::c_int { return }
    /* ** Step 2 -- Handle repeated messages ***/
    /* Acquire the "logical" last index */
    x =
        (message__next as libc::c_int + 2048 as libc::c_int -
             1 as libc::c_int) % 2048 as libc::c_int;
    /* Get the last message text */
    s =
        &mut *message__buf.offset(*message__ptr.offset(x as isize) as isize)
            as *mut libc::c_char as cptr;
    /* Last message repeated? */
    if streq(str, s) != 0 {
        /* Increase the message count */
        let ref mut fresh71 = *message__count.offset(x as isize);
        *fresh71 = (*fresh71).wrapping_add(1);
        /* Success */
        return
    }
    /* ** Step 3 -- Attempt to optimize ***/
    /* Limit number of messages to check */
    k = message_num() as libc::c_int / 4 as libc::c_int;
    /* Limit number of messages to check */
    if k > 2048 as libc::c_int / 32 as libc::c_int {
        k = 2048 as libc::c_int / 32 as libc::c_int
    }
    /* Check the last few messages (if any to count) */
    i = message__next as libc::c_int;
    while k != 0 {
        let mut q: u16b = 0;
        let mut old: cptr = 0 as *const libc::c_char;
        /* Back up and wrap if needed */
        let fresh72 = i;
        i = i - 1;
        if fresh72 == 0 as libc::c_int {
            i = 2048 as libc::c_int - 1 as libc::c_int
        }
        /* Stop before oldest message */
        if i == message__last as libc::c_int { break ; }
        /* Extract "distance" from "head" */
        q =
            ((message__head as libc::c_int + 32768 as libc::c_int -
                  *message__ptr.offset(i as isize) as libc::c_int) %
                 32768 as libc::c_int) as u16b;
        /* Do not optimize over large distance */
        if !(q as libc::c_int > 32768 as libc::c_int / 2 as libc::c_int) {
            /* Access the old string */
            old =
                &mut *message__buf.offset(*message__ptr.offset(i as isize) as
                                              isize) as *mut libc::c_char as
                    cptr;
            /* Compare */
            if !(streq(old, str) == 0) {
                /* Get the next message index, advance */
                let fresh73 = message__next;
                message__next = message__next.wrapping_add(1);
                x = fresh73 as libc::c_int;
                /* Handle wrap */
                if message__next as libc::c_int == 2048 as libc::c_int {
                    message__next = 0 as libc::c_int as u16b
                }
                /* Kill last message if needed */
                if message__next as libc::c_int ==
                       message__last as libc::c_int {
                    message__last = message__last.wrapping_add(1)
                }
                /* Handle wrap */
                if message__last as libc::c_int == 2048 as libc::c_int {
                    message__last = 0 as libc::c_int as u16b
                }
                /* Assign the starting address */
                *message__ptr.offset(x as isize) =
                    *message__ptr.offset(i as isize);
                *message__color.offset(x as isize) = color;
                *message__type.offset(x as isize) = type_0;
                *message__count.offset(x as isize) = 1 as libc::c_int as u16b;
                /* Success */
                return
            }
        }
        k -= 1
    }
    /* ** Step 4 -- Ensure space before end of buffer ***/
    /* Kill messages and Wrap if needed */
    if message__head as libc::c_int + n + 1 as libc::c_int >=
           32768 as libc::c_int {
        /* Kill all "dead" messages */
        i = message__last as libc::c_int;
        loop  {
            /* Wrap if needed */
            if i == 2048 as libc::c_int { i = 0 as libc::c_int }
            /* Stop before the new message */
            if i == message__next as libc::c_int { break ; }
            /* Kill "dead" messages */
            if *message__ptr.offset(i as isize) as libc::c_int >=
                   message__head as libc::c_int {
                /* Track oldest message */
                message__last = (i + 1 as libc::c_int) as u16b
            }
            i += 1
        }
        /* Wrap "tail" if needed */
        if message__tail as libc::c_int >= message__head as libc::c_int {
            message__tail = 0 as libc::c_int as u16b
        }
        /* Start over */
        message__head = 0 as libc::c_int as u16b
    }
    /* ** Step 5 -- Ensure space before next message ***/
    /* Kill messages if needed */
    if message__head as libc::c_int + n + 1 as libc::c_int >
           message__tail as libc::c_int {
        /* Grab new "tail" */
        message__tail =
            (message__head as libc::c_int + n + 1 as libc::c_int) as u16b;
        /* Advance tail while possible past first "nul" */
        while *message__buf.offset((message__tail as libc::c_int -
                                        1 as libc::c_int) as isize) != 0 {
            message__tail = message__tail.wrapping_add(1)
        }
        /* Kill all "dead" messages */
        i = message__last as libc::c_int;
        loop  {
            /* Wrap if needed */
            if i == 2048 as libc::c_int { i = 0 as libc::c_int }
            /* Stop before the new message */
            if i == message__next as libc::c_int { break ; }
            /* Kill "dead" messages */
            if *message__ptr.offset(i as isize) as libc::c_int >=
                   message__head as libc::c_int &&
                   (*message__ptr.offset(i as isize) as libc::c_int) <
                       message__tail as libc::c_int {
                /* Track oldest message */
                message__last = (i + 1 as libc::c_int) as u16b
            }
            i += 1
        }
    }
    /* ** Step 6 -- Grab a new message index ***/
    /* Get the next message index, advance */
    let fresh74 = message__next;
    message__next = message__next.wrapping_add(1);
    x = fresh74 as libc::c_int;
    /* Handle wrap */
    if message__next as libc::c_int == 2048 as libc::c_int {
        message__next = 0 as libc::c_int as u16b
    }
    /* Kill last message if needed */
    if message__next as libc::c_int == message__last as libc::c_int {
        message__last = message__last.wrapping_add(1)
    }
    /* Handle wrap */
    if message__last as libc::c_int == 2048 as libc::c_int {
        message__last = 0 as libc::c_int as u16b
    }
    /* ** Step 7 -- Insert the message text ***/
    /* Assign the starting address */
    *message__ptr.offset(x as isize) = message__head;
    *message__color.offset(x as isize) = color;
    *message__type.offset(x as isize) = type_0;
    *message__count.offset(x as isize) = 1 as libc::c_int as u16b;
    /* Append the new part of the message */
    i = 0 as libc::c_int;
    while i < n {
        /* Copy the message */
        *message__buf.offset((message__head as libc::c_int + i) as isize) =
            *str.offset(i as isize);
        i += 1
    }
    /* Terminate */
    *message__buf.offset((message__head as libc::c_int + i) as isize) =
        '\u{0}' as i32 as libc::c_char;
    /* Advance the "head" pointer */
    message__head =
        (message__head as libc::c_int + (n + 1 as libc::c_int)) as u16b;
}
/*
* Hack -- flush
*/
unsafe extern "C" fn msg_flush(mut x: libc::c_int) {
    let mut a: byte_hack = 14 as libc::c_int as byte_hack;
    /* Pause for response */
    Term_putstr(x, 0 as libc::c_int, -(1 as libc::c_int), a,
                b"-more-\x00" as *const u8 as *const libc::c_char);
    loop 
         /* Get an acceptable keypress */
         {
        let mut cmd: libc::c_int = inkey() as libc::c_int;
        if quick_messages != 0 { break ; }
        if cmd == '\u{1b}' as i32 || cmd == ' ' as i32 { break ; }
        if cmd == '\n' as i32 || cmd == '\r' as i32 { break ; }
        bell();
    }
    /* Clear the line */
    Term_erase(0 as libc::c_int, 0 as libc::c_int, 255 as libc::c_int);
}
/* Display a message */
#[no_mangle]
pub unsafe extern "C" fn display_message(mut x: libc::c_int,
                                         mut y: libc::c_int,
                                         mut split: libc::c_int,
                                         mut color: byte_hack, mut t: cptr) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    while i < split {
        if *t.offset(i as isize) as libc::c_int == '#' as i32 {
            if *t.offset((i + 1 as libc::c_int) as isize) as libc::c_int ==
                   '#' as i32 {
                Term_putstr(x + j, y, 1 as libc::c_int, color,
                            b"#\x00" as *const u8 as *const libc::c_char);
                i += 2 as libc::c_int;
                j += 1
            } else {
                color =
                    color_char_to_attr(*t.offset((i + 1 as libc::c_int) as
                                                     isize)) as byte_hack;
                i += 2 as libc::c_int
            }
        } else {
            Term_putstr(x + j, y, 1 as libc::c_int, color,
                        t.offset(i as isize));
            i += 1;
            j += 1
        }
    };
}
/*
* Output a message to the top line of the screen.
*
* Break long messages into multiple pieces (40-72 chars).
*
* Allow multiple short messages to "share" the top line.
*
* Prompt the user to make sure he has a chance to read them.
*
* These messages are memorized for later reference (see above).
*
* We could do "Term_fresh()" to provide "flicker" if needed.
*
* The global "msg_flag" variable can be cleared to tell us to
* "erase" any "pending" messages still on the screen.
*
* XXX XXX XXX Note that we must be very careful about using the
* "msg_print()" functions without explicitly calling the special
* "msg_print(NULL)" function, since this may result in the loss
* of information if the screen is cleared, or if anything is
* displayed on the top line.
*
* XXX XXX XXX Note that "msg_print(NULL)" will clear the top line
* even if no messages are pending.  This is probably a hack.
*/
#[no_mangle]
pub unsafe extern "C" fn cmsg_print(mut color: byte_hack, mut msg: cptr) {
    static mut p: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut lim: libc::c_int = (*Term).wid as libc::c_int - 8 as libc::c_int;
    /* Hack -- Reset */
    if msg_flag == 0 { p = 0 as libc::c_int }
    /* Message Length */
    n =
        if !msg.is_null() {
            strlen(msg)
        } else { 0 as libc::c_int as libc::c_ulong } as libc::c_int;
    /* Hack -- flush when requested or needed */
    if p != 0 && (msg.is_null() || p + n > lim) {
        /* Flush */
        msg_flush(p);
        /* Forget it */
        msg_flag = 0 as libc::c_int as bool_;
        /* Reset */
        p = 0 as libc::c_int
    }
    /* No message */
    if msg.is_null() { return }
    /* Paranoia */
    if n > 1000 as libc::c_int { return }
    /* Memorize the message */
    if character_generated != 0 {
        message_add(1 as libc::c_int as byte_hack, msg, color);
    }
    /* Handle "auto_more" */
    if auto_more != 0 {
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x40 as libc::c_long) as u32b;
        /* Force window update */
        window_stuff();
        /* Done */
        return
    }
    /* Copy it */
    strcpy(buf.as_mut_ptr(), msg);
    /* Analyze the buffer */
    t = buf.as_mut_ptr();
    /* Split message */
    while n > lim {
        let mut oops: libc::c_char = 0;
        let mut check: libc::c_int = 0;
        let mut split: libc::c_int = 0;
        /* Default split */
        split = lim;
        /* Find the "best" split point */
        check = 40 as libc::c_int;
        while check < lim {
            /* Found a valid split point */
            if *t.offset(check as isize) as libc::c_int == ' ' as i32 {
                split = check
            }
            check += 1
        }
        /* Save the split character */
        oops = *t.offset(split as isize);
        /* Split the message */
        *t.offset(split as isize) = '\u{0}' as i32 as libc::c_char;
        /* Display part of the message */
        display_message(0 as libc::c_int, 0 as libc::c_int, split, color,
                        t as cptr);
        /* Flush it */
        msg_flush(split + 1 as libc::c_int);
        /* Memorize the piece */
		/* if (character_generated) message_add(t); */
        /* Restore the split character */
        *t.offset(split as isize) = oops;
        /* Insert a space */
        split -= 1;
        *t.offset(split as isize) = ' ' as i32 as libc::c_char;
        /* Prepare to recurse on the rest of "buf" */
        t = t.offset(split as isize);
        n -= split
    }
    /* Display the tail of the message */
    display_message(p, 0 as libc::c_int, n, color, t as cptr);
    /* Memorize the tail */
	/* if (character_generated) message_add(t); */
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x40 as libc::c_long) as u32b;
    /* Remember the message */
    msg_flag = 1 as libc::c_int as bool_;
    /* Remember the position */
    p += n + 1 as libc::c_int;
    /* Optional refresh */
    if fresh_message != 0 { Term_fresh(); };
}
/* Hack -- for compatibility and easy sake */
#[no_mangle]
pub unsafe extern "C" fn msg_print(mut msg: cptr) {
    cmsg_print(1 as libc::c_int as byte_hack, msg);
}
/*
 * Hack -- prevent "accidents" in "screen_save()" or "screen_load()"
 */
static mut screen_depth: libc::c_int = 0 as libc::c_int;
/*
 * Save the screen, and increase the "icky" depth.
 *
 * This function must match exactly one call to "screen_load()".
 */
#[no_mangle]
pub unsafe extern "C" fn screen_save() {
    /* Hack -- Flush messages */
    msg_print(0 as cptr);
    /* Save the screen (if legal) */
    let fresh75 = screen_depth;
    screen_depth = screen_depth + 1;
    if fresh75 == 0 as libc::c_int { Term_save(); }
    /* Increase "icky" depth */
    character_icky += 1;
}
/*
 * Load the screen, and decrease the "icky" depth.
 *
 * This function must match exactly one call to "screen_save()".
 */
#[no_mangle]
pub unsafe extern "C" fn screen_load() {
    /* Hack -- Flush messages */
    msg_print(0 as cptr);
    /* Load the screen (if legal) */
    screen_depth -= 1;
    if screen_depth == 0 as libc::c_int { Term_load(); }
    /* Decrease "icky" depth */
    character_icky -= 1;
}
/*
* Display a formatted message, using "vstrnfmt()" and "msg_print()".
*/
#[no_mangle]
pub unsafe extern "C" fn msg_format(mut fmt: cptr, mut args: ...) {
    let mut vp: ::std::ffi::VaListImpl;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Begin the Varargs Stuff */
    vp = args.clone();
    /* Format the args, save the length */
    vstrnfmt(buf.as_mut_ptr(), 1024 as libc::c_int as uint_hack, fmt,
             vp.as_va_list());
    /* End the Varargs Stuff */
    /* Display */
    cmsg_print(1 as libc::c_int as byte_hack, buf.as_mut_ptr() as cptr);
}
#[no_mangle]
pub unsafe extern "C" fn cmsg_format(mut color: byte_hack, mut fmt: cptr,
                                     mut args: ...) {
    let mut vp: ::std::ffi::VaListImpl;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Begin the Varargs Stuff */
    vp = args.clone();
    /* Format the args, save the length */
    vstrnfmt(buf.as_mut_ptr(), 1024 as libc::c_int as uint_hack, fmt,
             vp.as_va_list());
    /* End the Varargs Stuff */
    /* Display */
    cmsg_print(color, buf.as_mut_ptr() as cptr);
}
/*
* Display a string on the screen using an attribute.
*
* At the given location, using the given attribute, if allowed,
* add the given string.  Do not clear the line.
*/
#[no_mangle]
pub unsafe extern "C" fn c_put_str(mut attr: byte_hack, mut str: cptr,
                                   mut row: libc::c_int,
                                   mut col: libc::c_int) {
    /* Position cursor, Dump the attr/text */
    Term_putstr(col, row, -(1 as libc::c_int), attr, str);
}
/*
* As above, but in "white"
*/
#[no_mangle]
pub unsafe extern "C" fn put_str(mut str: cptr, mut row: libc::c_int,
                                 mut col: libc::c_int) {
    /* Spawn */
    Term_putstr(col, row, -(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                str);
}
/*
* Display a string on the screen using an attribute, and clear
* to the end of the line.
*/
#[no_mangle]
pub unsafe extern "C" fn c_prt(mut attr: byte_hack, mut str: cptr,
                               mut row: libc::c_int, mut col: libc::c_int) {
    /* Clear line, position cursor */
    Term_erase(col, row, 255 as libc::c_int);
    /* Dump the attr/text */
    Term_addstr(-(1 as libc::c_int), attr, str);
}
/*
* As above, but in "white"
*/
#[no_mangle]
pub unsafe extern "C" fn prt(mut str: cptr, mut row: libc::c_int,
                             mut col: libc::c_int) {
    /* Spawn */
    c_prt(1 as libc::c_int as byte_hack, str, row, col);
}
/*
 * Print some (colored) text to the screen at the current cursor position,
 * automatically "wrapping" existing text (at spaces) when necessary to
 * avoid placing any text into the last column, and clearing every line
 * before placing any text in that line.  Also, allow "newline" to force
 * a "wrap" to the next line.  Advance the cursor as needed so sequential
 * calls to this function will work correctly.
 *
 * Once this function has been called, the cursor should not be moved
 * until all the related "text_out()" calls to the window are complete.
 *
 * This function will correctly handle any width up to the maximum legal
 * value of 256, though it works best for a standard 80 character width.
 */
#[no_mangle]
pub unsafe extern "C" fn text_out_to_screen(mut a: byte_hack, mut str: cptr) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut wid: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut wrap: libc::c_int = 0;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Obtain the size */
    Term_get_size(&mut wid, &mut h);
    /* Obtain the cursor */
    Term_locate(&mut x, &mut y);
    /* Use special wrapping boundary? */
    if text_out_wrap > 0 as libc::c_int && text_out_wrap < wid {
        wrap = text_out_wrap
    } else { wrap = wid }
    /* Process the string */
    s = str;
    while *s != 0 {
        let mut ch: libc::c_char = 0;
        /* Force wrap */
        if *s as libc::c_int == '\n' as i32 {
            /* Wrap */
            x = text_out_indent;
            y += 1;
            /* Clear line, move cursor */
            Term_erase(x, y, 255 as libc::c_int);
        } else {
            /* Clean up the char */
            ch =
                if *(*__ctype_b_loc()).offset(*s as libc::c_uchar as
                                                  libc::c_int as isize) as
                       libc::c_int &
                       _ISprint as libc::c_int as libc::c_ushort as
                           libc::c_int != 0 {
                    *s as libc::c_int
                } else { ' ' as i32 } as libc::c_char;
            /* Wrap words as needed */
            if x >= wrap - 1 as libc::c_int && ch as libc::c_int != ' ' as i32
               {
                let mut i: libc::c_int = 0;
                let mut n: libc::c_int = 0 as libc::c_int;
                let mut av: [byte_hack; 256] = [0; 256];
                let mut cv: [libc::c_char; 256] = [0; 256];
                /* Wrap word */
                if x < wrap {
                    /* Scan existing text */
                    i = wrap - 2 as libc::c_int;
                    while i >= 0 as libc::c_int {
                        /* Grab existing attr/char */
                        Term_what(i, y,
                                  &mut *av.as_mut_ptr().offset(i as isize),
                                  &mut *cv.as_mut_ptr().offset(i as isize));
                        /* Break on space */
                        if cv[i as usize] as libc::c_int == ' ' as i32 {
                            break ;
                        }
                        /* Track current word */
                        n = i;
                        i -= 1
                    }
                }
                /* Special case */
                if n == 0 as libc::c_int { n = wrap }
                /* Clear line */
                Term_erase(n, y, 255 as libc::c_int);
                /* Wrap */
                x = text_out_indent;
                y += 1;
                /* Clear line, move cursor */
                Term_erase(x, y, 255 as libc::c_int);
                /* Wrap the word (if any) */
                i = n;
                while i < wrap - 1 as libc::c_int {
                    /* Dump */
                    Term_addch(av[i as usize], cv[i as usize]);
                    /* Advance (no wrap) */
                    x += 1;
                    if x > wrap { x = wrap }
                    i += 1
                }
            }
            /* Dump */
            Term_addch(a, ch);
            /* Advance */
            x += 1;
            if x > wrap { x = wrap }
        }
        s = s.offset(1)
    };
}
/*
 * Write text to the given file and apply line-wrapping.
 *
 * Hook function for text_out(). Make sure that text_out_file points
 * to an open text-file.
 *
 * Long lines will be wrapped at text_out_wrap, or at column 75 if that
 * is not set; or at a newline character.
 *
 * You must be careful to end all file output with a newline character
 * to "flush" the stored line position.
 */
#[no_mangle]
pub unsafe extern "C" fn text_out_to_file(mut a: byte_hack, mut str: cptr) {
    /* Current position on the line */
    static mut pos: libc::c_int = 0 as libc::c_int;
    /* Wrap width */
    let mut wrap: libc::c_int =
        if text_out_wrap != 0 { text_out_wrap } else { 75 as libc::c_int };
    /* Current location within "str" */
    let mut s: cptr = str;
    /* Unused parameter */
    /* Process the string */
    while *s != 0 {
        let mut ch: libc::c_char = 0;
        let mut n: libc::c_int = 0 as libc::c_int;
        let mut len: libc::c_int = wrap - pos;
        let mut l_space: libc::c_int = 0 as libc::c_int;
        /* If we are at the start of the line... */
        if pos == 0 as libc::c_int {
            let mut i: libc::c_int = 0;
            /* Output the indent */
            i = 0 as libc::c_int;
            while i < text_out_indent {
                fputc(' ' as i32, text_out_file);
                pos += 1;
                i += 1
            }
        }
        /* Find length of line up to next newline or end-of-string */
        while n < len &&
                  !(*s.offset(n as isize) as libc::c_int == '\n' as i32 ||
                        *s.offset(n as isize) as libc::c_int ==
                            '\u{0}' as i32) {
            /* Mark the most recent space in the string */
            if *s.offset(n as isize) as libc::c_int == ' ' as i32 {
                l_space = n
            }
            /* Increment */
            n += 1
        }
        /* If we have encountered no spaces */
        if l_space == 0 as libc::c_int && n == len {
            /* If we are at the start of a new line */
            if pos == text_out_indent {
                len = n
            } else {
                /* Begin a new line */
                fputc('\n' as i32, text_out_file);
                /* Reset */
                pos = 0 as libc::c_int;
                continue ;
            }
        } else if *s.offset(n as isize) as libc::c_int == '\n' as i32 ||
                      *s.offset(n as isize) as libc::c_int == '\u{0}' as i32 {
            len = n
        } else {
            /* Wrap at the newline */
            /* Wrap at the last space */
            len = l_space
        }
        /* Write that line to file */
        n = 0 as libc::c_int;
        while n < len {
            /* Ensure the character is printable */
            ch =
                if *(*__ctype_b_loc()).offset(*s.offset(n as isize) as
                                                  libc::c_int as isize) as
                       libc::c_int &
                       _ISprint as libc::c_int as libc::c_ushort as
                           libc::c_int != 0 {
                    *s.offset(n as isize) as libc::c_int
                } else { ' ' as i32 } as libc::c_char;
            /* Write out the character */
            fputc(ch as libc::c_int, text_out_file);
            /* Increment */
            pos += 1;
            n += 1
        }
        /* Move 's' past the stuff we've written */
        s = s.offset(len as isize);
        /* If we are at the end of the string, end */
        if *s as libc::c_int == '\u{0}' as i32 { return }
        /* Skip newlines */
        if *s as libc::c_int == '\n' as i32 { s = s.offset(1) }
        /* Begin a new line */
        fputc('\n' as i32, text_out_file);
        /* Reset */
        pos = 0 as libc::c_int;
        /* Skip whitespace */
        while *s as libc::c_int == ' ' as i32 { s = s.offset(1) }
    };
}
/*
 * Output text to the screen or to a file depending on the selected
 * text_out hook.
 */
#[no_mangle]
pub unsafe extern "C" fn text_out(mut str: cptr) {
    text_out_c(1 as libc::c_int as byte_hack, str);
}
/*
 * Output text to the screen (in color) or to a file depending on the
 * selected hook.
 */
#[no_mangle]
pub unsafe extern "C" fn text_out_c(mut a: byte_hack, mut str: cptr) {
    text_out_hook.expect("non-null function pointer")(a, str);
}
/*
* Clear part of the screen
*/
#[no_mangle]
pub unsafe extern "C" fn clear_from(mut row: libc::c_int) {
    let mut y: libc::c_int = 0;
    /* Erase requested rows */
    y = row;
    while y < (*Term).hgt as libc::c_int {
        /* Erase part of the screen */
        Term_erase(0 as libc::c_int, y, 255 as libc::c_int);
        y += 1
    };
}
/*
 * Try to find a matching command completion.
 * Note that this is not so friendly since it doesn't give
 * a list of possible completions.
 *
 * First arg is the string to be completed, second is it's length,
 * third is it's maximum length.
 */
static mut complete_where: libc::c_int = 0 as libc::c_int;
static mut complete_buf: [libc::c_char; 100] = [0; 100];
unsafe extern "C" fn complete_command(mut buf: *mut libc::c_char,
                                      mut clen: libc::c_int,
                                      mut mlen: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 1 as libc::c_int;
    let mut max: libc::c_int = clen;
    let mut gotone: bool_ = 0 as libc::c_int as bool_;
    /* Forget the characters after the end of the string. */
    complete_buf[clen as usize] = '\u{0}' as i32 as libc::c_char;
    i = 0 as libc::c_int;
    while i < cli_total {
        let mut cli_ptr: *mut cli_comm = cli_info.offset(i as isize);
        if strncmp((*cli_ptr).comm, complete_buf.as_mut_ptr(),
                   clen as libc::c_ulong) == 0 {
            Term_erase(0 as libc::c_int, j, 80 as libc::c_int);
            let fresh76 = j;
            j = j + 1;
            Term_putstr(0 as libc::c_int, fresh76, -(1 as libc::c_int),
                        1 as libc::c_int as byte_hack, (*cli_ptr).comm);
            /* For the first match, copy the whole string to buf. */
            if gotone == 0 {
                sprintf(buf, b"%.*s\x00" as *const u8 as *const libc::c_char,
                        mlen, (*cli_ptr).comm);
                gotone = 1 as libc::c_int as bool_
            } else {
                /* For later matches, simply notice how much of buf it
			 * matches. */
                max = clen;
                while max < mlen {
                    if *(*cli_ptr).comm.offset(max as isize) as libc::c_int ==
                           '\u{0}' as i32 {
                        break ;
                    }
                    if *(*cli_ptr).comm.offset(max as isize) as libc::c_int !=
                           *buf.offset(max as isize) as libc::c_int {
                        break ;
                    }
                    max += 1
                }
                if max < mlen {
                    *buf.offset(max as isize) = '\u{0}' as i32 as libc::c_char
                }
            }
        }
        i += 1
    }
    return strlen(buf).wrapping_add(1 as libc::c_int as libc::c_ulong) as
               libc::c_int;
}
/*
* Get some input at the cursor location.
* Assume the buffer is initialized to a default string.
* Note that this string is often "empty" (see below).
* The default buffer is displayed in yellow until cleared.
* Pressing RETURN right away accepts the default entry.
* Normal chars clear the default and append the char.
* Backspace clears the default or deletes the final char.
* ESCAPE clears the buffer and the window and returns FALSE.
* RETURN accepts the current buffer contents and returns TRUE.
*/
#[no_mangle]
pub static mut askfor_aux_complete: bool_ = 0 as libc::c_int as bool_;
#[no_mangle]
pub unsafe extern "C" fn askfor_aux(mut buf: *mut libc::c_char,
                                    mut len: libc::c_int) -> bool_ {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    let mut done: bool_ = 0 as libc::c_int as bool_;
    /* Locate the cursor */
    Term_locate(&mut x, &mut y);
    /* Get terminal size */
    Term_get_size(&mut wid, &mut hgt);
    /* Paranoia -- check column */
    if x < 0 as libc::c_int || x >= wid { x = 0 as libc::c_int }
    /* Restrict the length */
    if x + len > wid { len = wid - x }
    /* Paranoia -- Clip the default entry */
    *buf.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    /* Display the default answer */
    Term_erase(x, y, len);
    Term_putstr(x, y, -(1 as libc::c_int), 11 as libc::c_int as byte_hack,
                buf as cptr);
    if askfor_aux_complete != 0 {
        screen_save();
        complete_where = 0 as libc::c_int;
        strncpy(complete_buf.as_mut_ptr(), buf,
                100 as libc::c_int as libc::c_ulong);
    }
    /* Process input */
    while done == 0 {
        /* Place cursor */
        Term_gotoxy(x + k, y);
        /* Get a key */
        i = inkey() as libc::c_int;
        let mut current_block_35: u64;
        /* Analyze the key */
        match i {
            27 => {
                k = 0 as libc::c_int;
                done = 1 as libc::c_int as bool_;
                current_block_35 = 7226443171521532240;
            }
            10 | 13 => {
                k = strlen(buf) as libc::c_int;
                done = 1 as libc::c_int as bool_;
                current_block_35 = 7226443171521532240;
            }
            9 => {
                if askfor_aux_complete as libc::c_int != 0 && k != 0 {
                    screen_load();
                    screen_save();
                    k = complete_command(buf, k, len)
                } else { bell(); }
                current_block_35 = 7295411273779428945;
            }
            127 | 8 => { current_block_35 = 7295411273779428945; }
            _ => {
                if k < len &&
                       *(*__ctype_b_loc()).offset(i as isize) as libc::c_int &
                           _ISprint as libc::c_int as libc::c_ushort as
                               libc::c_int != 0 {
                    let fresh77 = k;
                    k = k + 1;
                    *buf.offset(fresh77 as isize) = i as libc::c_char;
                    strncpy(complete_buf.as_mut_ptr(), buf,
                            k as libc::c_ulong);
                } else { bell(); }
                current_block_35 = 7226443171521532240;
            }
        }
        match current_block_35 {
            7295411273779428945 => {
                if k > 0 as libc::c_int { k -= 1 }
                strncpy(complete_buf.as_mut_ptr(), buf, k as libc::c_ulong);
            }
            _ => { }
        }
        /* Terminate */
        *buf.offset(k as isize) = '\u{0}' as i32 as libc::c_char;
        /* Update the entry */
        Term_erase(x, y, len);
        Term_putstr(x, y, -(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                    buf as cptr);
    }
    if askfor_aux_complete != 0 { screen_load(); }
    /* Aborted */
    if i == '\u{1b}' as i32 { return 0 as libc::c_int as bool_ }
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
* Get a string from the user
*
* The "prompt" should take the form "Prompt: "
*
* Note that the initial contents of the string is used as
* the default response, so be sure to "clear" it if needed.
*
* We clear the input, and return FALSE, on "ESCAPE".
*/
#[no_mangle]
pub unsafe extern "C" fn get_string(mut prompt: cptr,
                                    mut buf: *mut libc::c_char,
                                    mut len: libc::c_int) -> bool_ {
    let mut res: bool_ = 0;
    /* Paranoia XXX XXX XXX */
    msg_print(0 as cptr);
    /* Display prompt */
    prt(prompt, 0 as libc::c_int, 0 as libc::c_int);
    /* Ask the user for a string */
    res = askfor_aux(buf, len);
    /* Clear prompt */
    prt(b"\x00" as *const u8 as *const libc::c_char, 0 as libc::c_int,
        0 as libc::c_int);
    /* Result */
    return res;
}
/*
* Verify something with the user
*
* The "prompt" should take the form "Query? "
*
* Note that "[y/n]" is appended to the prompt.
*/
#[no_mangle]
pub unsafe extern "C" fn get_check(mut prompt: cptr) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 80] = [0; 80];
    /* Paranoia XXX XXX XXX */
    msg_print(0 as cptr);
    /* Hack -- Build a "useful" prompt */
    strnfmt(buf.as_mut_ptr(), 78 as libc::c_int as uint_hack,
            b"%.70s[y/n] \x00" as *const u8 as *const libc::c_char, prompt);
    /* Prompt for it */
    prt(buf.as_mut_ptr() as cptr, 0 as libc::c_int, 0 as libc::c_int);
    loop 
         /* Get an acceptable answer */
         {
        i = inkey() as libc::c_int;
        if quick_messages != 0 { break ; }
        if i == '\u{1b}' as i32 { break ; }
        if !strchr(b"YyNn\x00" as *const u8 as *const libc::c_char,
                   i).is_null() {
            break ;
        }
        bell();
    }
    /* Erase the prompt */
    prt(b"\x00" as *const u8 as *const libc::c_char, 0 as libc::c_int,
        0 as libc::c_int);
    /* Normal negation */
    if i != 'Y' as i32 && i != 'y' as i32 { return 0 as libc::c_int as bool_ }
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
* Prompts for a keypress
*
* The "prompt" should take the form "Command: "
*
* Returns TRUE unless the character is "Escape"
*/
#[no_mangle]
pub unsafe extern "C" fn get_com(mut prompt: cptr,
                                 mut command: *mut libc::c_char) -> bool_ {
    /* Paranoia XXX XXX XXX */
    msg_print(0 as cptr);
    /* Display a prompt */
    prt(prompt, 0 as libc::c_int, 0 as libc::c_int);
    /* Get a key */
    *command = inkey();
    /* Clear the prompt */
    prt(b"\x00" as *const u8 as *const libc::c_char, 0 as libc::c_int,
        0 as libc::c_int);
    /* Handle "cancel" */
    if *command as libc::c_int == '\u{1b}' as i32 {
        return 0 as libc::c_int as bool_
    }
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
* Request a "quantity" from the user
*
* Hack -- allow "command_arg" to specify a quantity
*/
#[no_mangle]
pub unsafe extern "C" fn get_quantity(mut prompt: cptr, mut max: s32b)
 -> s32b {
    let mut amt: s32b = 0;
    let mut aamt: libc::c_int = 0;
    let mut tmp: [libc::c_char; 80] = [0; 80];
    let mut buf: [libc::c_char; 80] = [0; 80];
    /* Use "command_arg" */
    if command_arg != 0 {
        /* Extract a number */
        amt = command_arg as s32b;
        /* Clear "command_arg" */
        command_arg = 0 as libc::c_int as s16b;
        /* Enforce the maximum */
        if amt > max { amt = max }
        /* Use it */
        return amt
    }
    /* Get the item index */
    if max != 1 as libc::c_int && repeat_pull(&mut aamt) as libc::c_int != 0 {
        amt = aamt;
        /* Enforce the maximum */
        if amt > max { amt = max }
        /* Enforce the minimum */
        if amt < 0 as libc::c_int { amt = 0 as libc::c_int }
        /* Use it */
        return amt
    }
    /* Build a prompt if needed */
    if prompt.is_null() {
        /* Build a prompt */
        sprintf(tmp.as_mut_ptr(),
                b"Quantity (1-%ld): \x00" as *const u8 as *const libc::c_char,
                max as libc::c_long);
        /* Use that prompt */
        prompt = tmp.as_mut_ptr() as cptr
    }
    /* Default to one */
    amt = 1 as libc::c_int;
    /* Build the default */
    sprintf(buf.as_mut_ptr(), b"%ld\x00" as *const u8 as *const libc::c_char,
            amt as libc::c_long);
    /* Ask for a quantity */
    if get_string(prompt, buf.as_mut_ptr(), 9 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    /* Extract a number */
    amt = atoi(buf.as_mut_ptr());
    /* A letter means "all" */
    if *(*__ctype_b_loc()).offset(buf[0 as libc::c_int as usize] as
                                      libc::c_int as isize) as libc::c_int &
           _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        amt = max
    }
    /* Enforce the maximum */
    if amt > max { amt = max }
    /* Enforce the minimum */
    if amt < 0 as libc::c_int { amt = 0 as libc::c_int }
    if amt != 0 { repeat_push(amt); }
    /* Return the result */
    return amt;
}
/*
* Pause for user response XXX XXX XXX
*/
#[no_mangle]
pub unsafe extern "C" fn pause_line(mut row: libc::c_int) {
    prt(b"\x00" as *const u8 as *const libc::c_char, row, 0 as libc::c_int);
    put_str(b"[Press any key to continue]\x00" as *const u8 as
                *const libc::c_char, row, 23 as libc::c_int);
    inkey();
    prt(b"\x00" as *const u8 as *const libc::c_char, row, 0 as libc::c_int);
}
/*
* Hack -- special buffer to hold the action of the current keymap
*/
static mut request_command_buffer: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut request_command_ignore_keymaps: [libc::c_char; 12] = [0; 12];
/*
* Mega-Hack -- flag set by do_cmd_{inven,equip}() to allow keymaps in
* auto-command mode.
*/
#[no_mangle]
pub static mut request_command_inven_mode: bool_ = 0 as libc::c_int as bool_;
/*
* Request a command from the user.
*
* Sets p_ptr->command_cmd, p_ptr->command_dir, p_ptr->command_rep,
* p_ptr->command_arg.  May modify p_ptr->command_new.
*
* Note that "caret" ("^") is treated specially, and is used to
* allow manual input of control characters.  This can be used
* on many machines to request repeated tunneling (Ctrl-H) and
* on the Macintosh to request "Control-Caret".
*
* Note that "backslash" is treated specially, and is used to bypass any
* keymap entry for the following character.  This is useful for macros.
*
* Note that this command is used both in the dungeon and in
* stores, and must be careful to work in both situations.
*
* Note that "p_ptr->command_new" may not work any more.  XXX XXX XXX
*/
#[no_mangle]
pub unsafe extern "C" fn request_command(mut shopping: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut cmd: s16b = 0;
    let mut cmd_char: libc::c_char = 0;
    let mut mode: libc::c_int = 0;
    let mut act: cptr = 0 as *const libc::c_char;
    /* Keymap mode */
    mode = get_keymap_mode();
    /* No command yet */
    command_cmd = 0 as libc::c_int as s16b;
    /* No "argument" yet */
    command_arg = 0 as libc::c_int as s16b;
    /* No "direction" yet */
    command_dir = 0 as libc::c_int as s16b;
    loop 
         /* Get command */
         /* Hack -- auto-commands */
         {
        if command_new != 0 {
            /* Flush messages */
            msg_print(0 as cptr);
            /* Use auto-command */
            cmd = command_new;
            /* Forget it */
            command_new = 0 as libc::c_int as s16b;
            /* Hack - bypass keymaps, unless asked not to */
            if inkey_next.is_null() && request_command_inven_mode == 0 {
                inkey_next = b"\x00" as *const u8 as *const libc::c_char
            }
            /* Mega-Hack -- turn off this flag immediately */
            request_command_inven_mode = 0 as libc::c_int as bool_
        } else {
            /* Get a keypress in "command" mode */
            /* Hack -- no flush needed */
            msg_flag = 0 as libc::c_int as bool_;
            inkey_flag = 1 as libc::c_int as bool_;
            cmd = inkey() as s16b
        }
        /* Activate "command mode" */
        /* Get a command */
        /* Clear top line */
        prt(b"\x00" as *const u8 as *const libc::c_char, 0 as libc::c_int,
            0 as libc::c_int);
        /* Command Count */
        if cmd as libc::c_int == '0' as i32 {
            let mut old_arg: libc::c_int = command_arg as libc::c_int;
            /* Reset */
            command_arg = 0 as libc::c_int as s16b;
            /* Begin the input */
            prt(b"Count: \x00" as *const u8 as *const libc::c_char,
                0 as libc::c_int, 0 as libc::c_int);
            loop 
                 /* Get a command count */
                 /* Get a new keypress */
                 {
                cmd = inkey() as s16b;
                /* Simple editing (delete or backspace) */
                if cmd as libc::c_int == 0x7f as libc::c_int ||
                       cmd as libc::c_int == 'H' as i32 & 0x1f as libc::c_int
                   {
                    /* Delete a digit */
                    command_arg =
                        (command_arg as libc::c_int / 10 as libc::c_int) as
                            s16b;
                    /* Show current count */
                    prt(format(b"Count: %d\x00" as *const u8 as
                                   *const libc::c_char,
                               command_arg as libc::c_int) as cptr,
                        0 as libc::c_int, 0 as libc::c_int);
                } else {
                    /* Actual numeric data */
                    if !(cmd as libc::c_int >= '0' as i32 &&
                             cmd as libc::c_int <= '9' as i32) {
                        break ;
                    }
                    /* Stop count at 9999 */
                    if command_arg as libc::c_int >= 1000 as libc::c_int {
                        /* Warn */
                        bell();
                        /* Limit */
                        command_arg = 9999 as libc::c_int as s16b
                    } else {
                        /* Increase count */
                        /* Incorporate that digit */
                        command_arg =
                            (command_arg as libc::c_int * 10 as libc::c_int +
                                 (cmd as libc::c_int - '0' as i32)) as s16b
                    }
                    /* Show current count */
                    prt(format(b"Count: %d\x00" as *const u8 as
                                   *const libc::c_char,
                               command_arg as libc::c_int) as cptr,
                        0 as libc::c_int, 0 as libc::c_int);
                }
            }
            /* Hack -- Handle "zero" */
            if command_arg as libc::c_int == 0 as libc::c_int {
                /* Default to 99 */
                command_arg = 99 as libc::c_int as s16b;
                /* Show current count */
                prt(format(b"Count: %d\x00" as *const u8 as
                               *const libc::c_char,
                           command_arg as libc::c_int) as cptr,
                    0 as libc::c_int, 0 as libc::c_int);
            }
            /* Hack -- Handle "old_arg" */
            if old_arg != 0 as libc::c_int {
                /* Restore old_arg */
                command_arg = old_arg as s16b;
                /* Show current count */
                prt(format(b"Count: %d\x00" as *const u8 as
                               *const libc::c_char,
                           command_arg as libc::c_int) as cptr,
                    0 as libc::c_int, 0 as libc::c_int);
            }
            /* Hack -- white-space means "enter command now" */
            if cmd as libc::c_int == ' ' as i32 ||
                   cmd as libc::c_int == '\n' as i32 ||
                   cmd as libc::c_int == '\r' as i32 {
                /* Get a real command */
                let mut temp: bool_ =
                    get_com(b"Command: \x00" as *const u8 as
                                *const libc::c_char, &mut cmd_char);
                cmd = cmd_char as s16b;
                if temp == 0 {
                    /* Clear count */
                    command_arg = 0 as libc::c_int as s16b;
                    /* Continue */
                    continue ;
                }
            }
        }
        /* Allow "keymaps" to be bypassed */
        if cmd as libc::c_int == '\\' as i32 {
            /* Get a real command */
            get_com(b"Command: \x00" as *const u8 as *const libc::c_char,
                    &mut cmd_char);
            cmd = cmd_char as s16b;
            /* Hack -- bypass keymaps */
            if inkey_next.is_null() {
                inkey_next = b"\x00" as *const u8 as *const libc::c_char
            }
        }
        /* Allow "control chars" to be entered */
        if cmd as libc::c_int == '^' as i32 {
            /* Get a new command and controlify it */
            if get_com(b"Control: \x00" as *const u8 as *const libc::c_char,
                       &mut cmd_char) != 0 {
                cmd = (cmd_char as libc::c_int & 0x1f as libc::c_int) as s16b
            } else { cmd = 0 as libc::c_int as s16b }
        }
        /* Look up applicable keymap */
        act = keymap_act[mode as usize][cmd as byte_hack as usize];
        /* Mega-Hack -- Ignore certain keymaps */
        if shopping != 0 && cmd as libc::c_int > 0 as libc::c_int {
            i = 0 as libc::c_int;
            while i < 12 as libc::c_int {
                if cmd as libc::c_int ==
                       request_command_ignore_keymaps[i as usize] as
                           libc::c_int {
                    act = 0 as cptr;
                    break ;
                } else { i += 1 }
            }
        }
        /* Apply keymap if not inside a keymap already */
        if !act.is_null() && inkey_next.is_null() {
            /* Install the keymap (limited buffer size) */
            strnfmt(request_command_buffer.as_mut_ptr(),
                    256 as libc::c_int as uint_hack,
                    b"%s\x00" as *const u8 as *const libc::c_char, act);
            /* Start using the buffer */
            inkey_next = request_command_buffer.as_mut_ptr() as cptr
        } else {
            /* Paranoia */
            if cmd == 0 { continue ; }
            /* Use command */
            command_cmd = cmd;
            break ;
        }
    }
    /* Hack -- Auto-repeat certain commands */
    if always_repeat as libc::c_int != 0 &&
           command_arg as libc::c_int <= 0 as libc::c_int {
        /* Hack -- auto repeat certain commands */
        if !strchr(b"TBDoc+\x00" as *const u8 as *const libc::c_char,
                   command_cmd as libc::c_int).is_null() {
            /* Repeat 99 times */
            command_arg = 99 as libc::c_int as s16b
        }
    }
    /* Hack -- Scan equipment */
    i = 24 as libc::c_int;
    while i < 52 as libc::c_int {
        let mut s: cptr = 0 as *const libc::c_char;
        let mut o_ptr: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Skip non-objects */
        if !((*o_ptr).k_idx == 0) {
            /* No inscription */
            if !((*o_ptr).note == 0) {
                /* Obtain the inscription */
                s = quark_str((*o_ptr).note as s16b);
                /* Find a '^' */
                s = strchr(s, '^' as i32) as cptr;
                /* Process preventions */
                while !s.is_null() {
                    /* Check the "restriction" character */
                    if *s.offset(1 as libc::c_int as isize) as libc::c_int ==
                           command_cmd as libc::c_int ||
                           *s.offset(1 as libc::c_int as isize) as libc::c_int
                               == '*' as i32 {
                        /* Hack -- Verify command */
                        if get_check(b"Are you sure? \x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                            /* Hack -- Use space */
                            command_cmd = ' ' as i32 as s16b
                        }
                    }
                    /* Find another '^' */
                    s =
                        strchr(s.offset(1 as libc::c_int as isize),
                               '^' as i32) as cptr
                }
            }
        }
        i += 1
    }
    /* Hack -- erase the message line. */
    prt(b"\x00" as *const u8 as *const libc::c_char, 0 as libc::c_int,
        0 as libc::c_int);
}
/*
 * Check a char for "vowel-hood"
 */
#[no_mangle]
pub unsafe extern "C" fn is_a_vowel(mut ch: libc::c_int) -> bool_ {
    match ch {
        97 | 101 | 105 | 111 | 117 | 65 | 69 | 73 | 79 | 85 => {
            return 1 as libc::c_int as bool_
        }
        _ => { }
    }
    return 0 as libc::c_int as bool_;
}
/*
 * GH
 * Called from cmd4.c and a few other places. Just extracts
 * a direction from the keymap for ch (the last direction,
 * in fact) byte or char here? I'm thinking that keymaps should
 * generally only apply to single keys, which makes it no more
 * than 128, so a char should suffice... but keymap_act is 256...
 */
#[no_mangle]
pub unsafe extern "C" fn get_keymap_dir(mut ch: libc::c_char) -> libc::c_int {
    let mut d: libc::c_int = 0 as libc::c_int;
    let mut mode: libc::c_int = 0;
    let mut act: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Already a direction? */
    if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int &
           _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        d = ch as libc::c_int - '0' as i32
    } else {
        /* Keymap mode */
        mode = get_keymap_mode();
        /* Extract the action (if any) */
        act = keymap_act[mode as usize][ch as byte_hack as usize];
        /* Analyze */
        if !act.is_null() {
            /* Convert to a direction */
            s = act;
            while *s != 0 {
                /* Use any digits in keymap */
                if *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as
                       libc::c_int &
                       _ISdigit as libc::c_int as libc::c_ushort as
                           libc::c_int != 0 {
                    d = *s as libc::c_int - '0' as i32
                }
                s = s.offset(1)
            }
        }
    }
    /* Paranoia */
    if d == 5 as libc::c_int { d = 0 as libc::c_int }
    /* Return direction */
    return d;
}
/* Number of chars saved */
static mut repeat__cnt: libc::c_int = 0 as libc::c_int;
/* Current index */
static mut repeat__idx: libc::c_int = 0 as libc::c_int;
/* Saved "stuff" */
static mut repeat__key: [libc::c_int; 20] = [0; 20];
#[no_mangle]
pub unsafe extern "C" fn repeat_push(mut what: libc::c_int) {
    /* Too many keys */
    if repeat__cnt == 20 as libc::c_int { return }
    /* Push the "stuff" */
    let fresh78 = repeat__cnt;
    repeat__cnt = repeat__cnt + 1;
    repeat__key[fresh78 as usize] = what;
    /* Prevents us from pulling keys */
    repeat__idx += 1;
}
#[no_mangle]
pub unsafe extern "C" fn repeat_pull(mut what: *mut libc::c_int) -> bool_ {
    /* All out of keys */
    if repeat__idx == repeat__cnt { return 0 as libc::c_int as bool_ }
    /* Grab the next key, advance */
    let fresh79 = repeat__idx;
    repeat__idx = repeat__idx + 1;
    *what = repeat__key[fresh79 as usize];
    /* Success */
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn repeat_check() {
    let mut what: libc::c_int = 0;
    /* Ignore some commands */
    if command_cmd as libc::c_int == '\u{1b}' as i32 { return }
    if command_cmd as libc::c_int == ' ' as i32 { return }
    if command_cmd as libc::c_int == '\r' as i32 { return }
    if command_cmd as libc::c_int == '\n' as i32 { return }
    /* Repeat Last Command */
    if command_cmd as libc::c_int == 'n' as i32 {
        /* Reset */
        repeat__idx = 0 as libc::c_int;
        /* Get the command */
        if repeat_pull(&mut what) != 0 {
            /* Save the command */
            command_cmd = what as s16b
        }
    } else {
        /* Start saving new command */
        /* Reset */
        repeat__cnt = 0 as libc::c_int;
        repeat__idx = 0 as libc::c_int;
        what = command_cmd as libc::c_int;
        repeat_push(what);
    };
}
/* Save this command */
/*
 * Read a number at a specific location on the screen
 *
 * Allow numbers of any size and save the last keypress.
 */
#[no_mangle]
pub unsafe extern "C" fn get_number(mut def: u32b, mut max: u32b,
                                    mut y: libc::c_int, mut x: libc::c_int,
                                    mut cmd: *mut libc::c_char) -> u32b {
    let mut res: u32b = def;
    /* Player has not typed anything yet */
    let mut no_keys: bool_ = 1 as libc::c_int as bool_;
    /* Begin the input with default */
    prt(format(b"%lu\x00" as *const u8 as *const libc::c_char, def) as cptr,
        y, x);
    loop 
         /* Get a command count */
         /* Get a new keypress */
         {
        *cmd = inkey();
        /* Simple editing (delete or backspace) */
        if *cmd as libc::c_int == 0x7f as libc::c_int ||
               *cmd as libc::c_int == 'H' as i32 & 0x1f as libc::c_int {
            /* Override the default */
            no_keys = 0 as libc::c_int as bool_;
            /* Delete a digit */
            res = res.wrapping_div(10 as libc::c_int as libc::c_uint);
            prt(format(b"%lu\x00" as *const u8 as *const libc::c_char, res) as
                    cptr, y, x);
        } else if *cmd as libc::c_int >= '0' as i32 &&
                      *cmd as libc::c_int <= '9' as i32 {
            /* Actual numeric data */
            /* Override the default */
            if no_keys != 0 {
                no_keys = 0 as libc::c_int as bool_;
                res = 0 as libc::c_int as u32b
            }
            /* Don't overflow */
            if ((0 as libc::c_int - 1 as libc::c_int) as
                    u32b).wrapping_sub((*cmd as libc::c_int - '0' as i32) as
                                           libc::c_uint).wrapping_div(10 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                   < res {
                /* Warn */
                bell();
                /* Limit */
                res =
                    if max.wrapping_add(1 as libc::c_int as libc::c_uint) ==
                           0 as libc::c_int as libc::c_uint {
                        (0 as libc::c_int - 1 as libc::c_int) as u32b
                    } else { max }
            } else if res.wrapping_mul(10 as libc::c_int as
                                           libc::c_uint).wrapping_add((*cmd as
                                                                           libc::c_int
                                                                           -
                                                                           '0'
                                                                               as
                                                                               i32)
                                                                          as
                                                                          libc::c_uint)
                          > max {
                /* Stop count at maximum */
                /* Warn */
                bell();
                /* Limit */
                res = max
            } else {
                /* Increase count */
                /* Incorporate that digit */
                res =
                    res.wrapping_mul(10 as libc::c_int as
                                         libc::c_uint).wrapping_add((*cmd as
                                                                         libc::c_int
                                                                         -
                                                                         '0'
                                                                             as
                                                                             i32)
                                                                        as
                                                                        libc::c_uint)
            }
            /* Show current count */
            prt(format(b"%lu\x00" as *const u8 as *const libc::c_char, res) as
                    cptr, y, x);
        } else {
            /* Escape cancels */
            if !(*cmd as libc::c_int == '\u{1b}' as i32) { break ; }
            res = 0 as libc::c_int as u32b;
            break ;
        }
    }
    return res;
}
/*
 * Allow the user to select multiple items without pressing '0'
 */
#[no_mangle]
pub unsafe extern "C" fn get_count(mut number: libc::c_int,
                                   mut max: libc::c_int) {
    let mut cmd: libc::c_char = 0;
    /* Use the default */
    command_arg = number as s16b;
    /* Hack -- Optional flush */
    if flush_command != 0 { flush(); }
    /* Clear top line */
    prt(b"\x00" as *const u8 as *const libc::c_char, 0 as libc::c_int,
        0 as libc::c_int);
    /* Begin the input */
    prt(b"How many?\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int, 0 as libc::c_int);
    /* Actually get a number */
    command_arg =
        get_number(command_arg as u32b, max as u32b, 0 as libc::c_int,
                   10 as libc::c_int, &mut cmd) as s16b;
    prt(b"\x00" as *const u8 as *const libc::c_char, 0 as libc::c_int,
        0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn count_bits(mut array: u32b) -> byte_hack {
    let mut k: byte_hack = 0 as libc::c_int as byte_hack;
    let mut i: byte_hack = 0;
    if array != 0 {
        i = 0 as libc::c_int as byte_hack;
        while (i as libc::c_int) < 32 as libc::c_int {
            if array &
                   ((1 as libc::c_int) << i as libc::c_int) as libc::c_uint !=
                   0 {
                k = k.wrapping_add(1)
            }
            i = i.wrapping_add(1)
        }
    }
    return k;
}
/* Return the lowered string */
#[no_mangle]
pub unsafe extern "C" fn strlower(mut buf: *mut libc::c_char) {
    let mut i: u16b = 0;
    i = 0 as libc::c_int as u16b;
    while *buf.offset(i as isize) as libc::c_int != 0 as libc::c_int &&
              (i as libc::c_int) < 256 as libc::c_int {
        if *(*__ctype_b_loc()).offset(*buf.offset(i as isize) as libc::c_int
                                          as isize) as libc::c_int &
               _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0 {
            *buf.offset(i as isize) =
                tolower(*buf.offset(i as isize) as libc::c_int) as
                    libc::c_char
        }
        i = i.wrapping_add(1)
    };
}
/*
 * Given monster name as string, return the index in r_info array. Name
 * must exactly match (look out for commas and the like!), or else 0 is
 * returned. Case doesn't matter. -GSN-
 */
#[no_mangle]
pub unsafe extern "C" fn test_monster_name(mut name: cptr) -> libc::c_int {
    let mut i: libc::c_int = 0;
    /* Scan the monsters */
    i = 1 as libc::c_int;
    while i < max_r_idx as libc::c_int {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset(i as isize) as *mut monster_race;
        let mut mon_name: cptr =
            r_name.offset((*r_ptr).name as isize) as cptr;
        /* If name matches, give us the number */
        if strcasecmp(name, mon_name) == 0 as libc::c_int { return i }
        i += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn test_mego_name(mut name: cptr) -> libc::c_int {
    let mut i: libc::c_int = 0;
    /* Scan the monsters */
    i = 1 as libc::c_int;
    while i < max_re_idx as libc::c_int {
        let mut re_ptr: *mut monster_ego =
            &mut *re_info.offset(i as isize) as *mut monster_ego;
        let mut mon_name: cptr =
            re_name.offset((*re_ptr).name as isize) as cptr;
        /* If name matches, give us the number */
        if strcasecmp(name, mon_name) == 0 as libc::c_int { return i }
        i += 1
    }
    return 0 as libc::c_int;
}
/*
 * Given item name as string, return the index in k_info array. Name
 * must exactly match (look out for commas and the like!), or else 0 is
 * returned. Case doesn't matter. -DG-
 */
#[no_mangle]
pub unsafe extern "C" fn test_item_name(mut name: cptr) -> libc::c_int {
    let mut i: libc::c_int = 0;
    /* Scan the items */
    i = 1 as libc::c_int;
    while i < max_k_idx as libc::c_int {
        let mut k_ptr: *mut object_kind =
            &mut *k_info.offset(i as isize) as *mut object_kind;
        let mut obj_name: cptr =
            k_name.offset((*k_ptr).name as isize) as cptr;
        /* If name matches, give us the number */
        if strcasecmp(name, obj_name) == 0 as libc::c_int { return i }
        i += 1
    }
    return 0 as libc::c_int;
}
/*
 * Break scalar time
 */
#[no_mangle]
pub unsafe extern "C" fn bst(mut what: s32b, mut t: s32b) -> s32b {
    let mut turns: s32b =
        t +
            10 as libc::c_int *
                (11520 as libc::c_int / 24 as libc::c_int * 6 as libc::c_int);
    match what {
        8 => {
            return turns / 10 as libc::c_int /
                       (11520 as libc::c_int / 24 as libc::c_int /
                            60 as libc::c_int) % 60 as libc::c_int
        }
        480 => {
            return turns / 10 as libc::c_int /
                       (11520 as libc::c_int / 24 as libc::c_int) %
                       24 as libc::c_int
        }
        11520 => {
            return turns / 10 as libc::c_int / 11520 as libc::c_int %
                       365 as libc::c_int
        }
        4204800 => {
            return turns / 10 as libc::c_int /
                       (11520 as libc::c_int * 365 as libc::c_int)
        }
        _ => { return 0 as libc::c_int }
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_month_name(mut day: libc::c_int, mut full: bool_,
                                        mut compact: bool_) -> cptr {
    let mut i: libc::c_int = 8 as libc::c_int;
    static mut buf: [libc::c_char; 40] = [0; 40];
    /* Find the period name */
    while i > 0 as libc::c_int && day < month_day[i as usize] { i -= 1 }
    match i {
        0 | 8 => {
            /* Yestare/Mettare */
            let mut buf2: [libc::c_char; 20] = [0; 20];
            sprintf(buf2.as_mut_ptr(),
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    get_day(day + 1 as libc::c_int));
            if full != 0 {
                sprintf(buf.as_mut_ptr(),
                        b"%s (%s day)\x00" as *const u8 as
                            *const libc::c_char, month_name[i as usize],
                        buf2.as_mut_ptr());
            } else {
                sprintf(buf.as_mut_ptr(),
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        month_name[i as usize]);
            }
        }
        _ => {
            /* 'Normal' months + Enderi */
            let mut buf2_0: [libc::c_char; 20] = [0; 20];
            let mut buf3: [libc::c_char; 20] = [0; 20];
            sprintf(buf2_0.as_mut_ptr(),
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    get_day(day + 1 as libc::c_int - month_day[i as usize]));
            sprintf(buf3.as_mut_ptr(),
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    get_day(day + 1 as libc::c_int));
            if full != 0 {
                sprintf(buf.as_mut_ptr(),
                        b"%s day of %s (%s day)\x00" as *const u8 as
                            *const libc::c_char, buf2_0.as_mut_ptr(),
                        month_name[i as usize], buf3.as_mut_ptr());
            } else if compact != 0 {
                sprintf(buf.as_mut_ptr(),
                        b"%s day of %s\x00" as *const u8 as
                            *const libc::c_char, buf2_0.as_mut_ptr(),
                        month_name[i as usize]);
            } else {
                sprintf(buf.as_mut_ptr(),
                        b"%s %s\x00" as *const u8 as *const libc::c_char,
                        buf2_0.as_mut_ptr(), month_name[i as usize]);
            }
        }
    }
    return buf.as_mut_ptr() as cptr;
}
#[no_mangle]
pub unsafe extern "C" fn get_day(mut day: libc::c_int) -> cptr {
    static mut buf: [libc::c_char; 20] = [0; 20];
    let mut p: cptr = b"th\x00" as *const u8 as *const libc::c_char;
    if !(day / 10 as libc::c_int == 1 as libc::c_int) {
        if day % 10 as libc::c_int == 1 as libc::c_int {
            p = b"st\x00" as *const u8 as *const libc::c_char
        } else if day % 10 as libc::c_int == 2 as libc::c_int {
            p = b"nd\x00" as *const u8 as *const libc::c_char
        } else if day % 10 as libc::c_int == 3 as libc::c_int {
            p = b"rd\x00" as *const u8 as *const libc::c_char
        }
    }
    sprintf(buf.as_mut_ptr(), b"%d%s\x00" as *const u8 as *const libc::c_char,
            day, p);
    return buf.as_mut_ptr() as cptr;
}
#[no_mangle]
pub unsafe extern "C" fn get_player_race_name(mut pr: libc::c_int,
                                              mut ps: libc::c_int) -> cptr {
    static mut buf: [libc::c_char; 50] = [0; 50];
    if ps != 0 {
        if (*race_mod_info.offset(ps as isize)).place != 0 {
            sprintf(buf.as_mut_ptr(),
                    b"%s %s\x00" as *const u8 as *const libc::c_char,
                    rp_name.offset((*race_info.offset(pr as isize)).title as
                                       isize),
                    rmp_name.offset((*race_mod_info.offset(ps as isize)).title
                                        as isize));
        } else {
            sprintf(buf.as_mut_ptr(),
                    b"%s %s\x00" as *const u8 as *const libc::c_char,
                    rmp_name.offset((*race_mod_info.offset(ps as isize)).title
                                        as isize),
                    rp_name.offset((*race_info.offset(pr as isize)).title as
                                       isize));
        }
    } else {
        sprintf(buf.as_mut_ptr(),
                b"%s\x00" as *const u8 as *const libc::c_char,
                rp_name.offset((*race_info.offset(pr as isize)).title as
                                   isize));
    }
    return buf.as_mut_ptr() as cptr;
}
/*
 * Ask to select an item in a list
 */
#[no_mangle]
pub unsafe extern "C" fn ask_menu(mut ask: cptr,
                                  mut items: *mut *mut libc::c_char,
                                  mut max: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0;
    let mut start: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_char = 0;
    /* Enter "icky" mode */
    character_icky = 1 as libc::c_int as bool_;
    /* Save the screen */
    Term_save();
    loop  {
        /* Display list */
        Term_load();
        Term_save();
        prt(ask, 0 as libc::c_int, 0 as libc::c_int);
        i = start;
        while i < max && i < start + 20 as libc::c_int {
            prt(format(b"%c) %s\x00" as *const u8 as *const libc::c_char,
                       i - start + 'a' as i32, *items.offset(i as isize)) as
                    cptr, i - start + 1 as libc::c_int, 0 as libc::c_int);
            i += 1
        }
        /* Wait for user input */
        c = inkey();
        /* Leave the screen */
        if c as libc::c_int == '\u{1b}' as i32 { break ; }
        /* Scroll */
        if c as libc::c_int == '+' as i32 {
            if (start + 20 as libc::c_int) < max {
                start += 20 as libc::c_int
            }
        } else if c as libc::c_int == '-' as i32 {
            start -= 20 as libc::c_int;
            if start < 0 as libc::c_int { start = 0 as libc::c_int }
        } else {
            /* Scroll */
            /* Good selection */
            c = tolower(c as libc::c_int) as libc::c_char;
            if c as libc::c_int - 'a' as i32 + start >= max {
                bell();
            } else if c as libc::c_int - 'a' as i32 + start < 0 as libc::c_int
             {
                bell();
            } else { ret = c as libc::c_int - 'a' as i32 + start; break ; }
        }
    }
    /* Load the screen */
    Term_load();
    /* Leave "icky" mode */
    character_icky = 0 as libc::c_int as bool_;
    return ret;
}
/*
 * Determine if string "t" is a prefix of string "s"
 */
#[no_mangle]
pub unsafe extern "C" fn prefix(mut s: cptr, mut t: cptr) -> bool_ {
    /* Paranoia */
    if s.is_null() || t.is_null() {
        if alert_failure != 0 {
            message_add(1 as libc::c_int as byte_hack,
                        b"prefix() called with null argument!\x00" as
                            *const u8 as *const libc::c_char,
                        4 as libc::c_int as byte_hack);
        }
        return 0 as libc::c_int as bool_
    }
    /* Scan "t" */
    while *t != 0 {
        /* Compare content and length */
        let fresh80 = t;
        t = t.offset(1);
        let fresh81 = s;
        s = s.offset(1);
        if *fresh80 as libc::c_int != *fresh81 as libc::c_int {
            return 0 as libc::c_int as bool_
        }
    }
    /* Matched, we have a prefix */
    return 1 as libc::c_int as bool_;
}
/*
 * Rescale a value
 */
#[no_mangle]
pub unsafe extern "C" fn value_scale(mut value: libc::c_int,
                                     mut vmax: libc::c_int,
                                     mut max: libc::c_int,
                                     mut min: libc::c_int) -> s32b {
    let mut full_max: s32b = max - min;
    value = value * full_max / vmax;
    value += min;
    return value;
}
/*
 * Displays a box
 */
#[no_mangle]
pub unsafe extern "C" fn draw_box(mut y: libc::c_int, mut x: libc::c_int,
                                  mut h: libc::c_int, mut w: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = x + 1 as libc::c_int;
    while i < x + w {
        j = y + 1 as libc::c_int;
        while j < y + h {
            Term_putch(i, j, 14 as libc::c_int as byte_hack,
                       ' ' as i32 as libc::c_char);
            j += 1
        }
        i += 1
    }
    i = x;
    while i < x + w {
        c_put_str(14 as libc::c_int as byte_hack,
                  b"-\x00" as *const u8 as *const libc::c_char, y, i);
        c_put_str(14 as libc::c_int as byte_hack,
                  b"-\x00" as *const u8 as *const libc::c_char, y + h, i);
        i += 1
    }
    i = y;
    while i < y + h {
        c_put_str(14 as libc::c_int as byte_hack,
                  b"|\x00" as *const u8 as *const libc::c_char, i, x);
        c_put_str(14 as libc::c_int as byte_hack,
                  b"|\x00" as *const u8 as *const libc::c_char, i, x + w);
        i += 1
    }
    Term_putch(x, y, 14 as libc::c_int as byte_hack,
               '/' as i32 as libc::c_char);
    Term_putch(x + w, y, 14 as libc::c_int as byte_hack,
               '\\' as i32 as libc::c_char);
    Term_putch(x, y + h, 14 as libc::c_int as byte_hack,
               '\\' as i32 as libc::c_char);
    Term_putch(x + w, y + h, 14 as libc::c_int as byte_hack,
               '/' as i32 as libc::c_char);
}
/*
 * Displays a scrollable boxed list with a selected item
 */
#[no_mangle]
pub unsafe extern "C" fn display_list(mut y: libc::c_int, mut x: libc::c_int,
                                      mut h: libc::c_int, mut w: libc::c_int,
                                      mut title: cptr, mut list: *mut cptr,
                                      mut max: libc::c_int,
                                      mut begin: libc::c_int,
                                      mut sel: libc::c_int,
                                      mut sel_color: byte_hack) {
    let mut i: libc::c_int = 0;
    draw_box(y, x, h, w);
    c_put_str(14 as libc::c_int as byte_hack, title, y,
              (x as
                   libc::c_ulong).wrapping_add((w as
                                                    libc::c_ulong).wrapping_sub(strlen(title)).wrapping_div(2
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_ulong))
                  as libc::c_int);
    i = 0 as libc::c_int;
    while i < h - 1 as libc::c_int {
        let mut color: byte_hack = 1 as libc::c_int as byte_hack;
        if i + begin >= max { break ; }
        if i + begin == sel { color = sel_color }
        c_put_str(color, *list.offset((i + begin) as isize),
                  y + 1 as libc::c_int + i, x + 1 as libc::c_int);
        i += 1
    };
}
/*
 * Creates an input box
 */
#[no_mangle]
pub unsafe extern "C" fn input_box(mut text: cptr, mut y: libc::c_int,
                                   mut x: libc::c_int,
                                   mut buf: *mut libc::c_char,
                                   mut max: libc::c_int) -> bool_ {
    let mut smax: libc::c_int = strlen(text) as libc::c_int;
    if max > smax { smax = max }
    smax += 1;
    draw_box(y - 1 as libc::c_int, x - smax / 2 as libc::c_int,
             3 as libc::c_int, smax);
    c_put_str(1 as libc::c_int as byte_hack, text, y,
              (x as
                   libc::c_ulong).wrapping_sub(strlen(text).wrapping_div(2 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong))
                  as libc::c_int);
    Term_gotoxy(x - smax / 2 as libc::c_int + 1 as libc::c_int,
                y + 1 as libc::c_int);
    return askfor_aux(buf, max);
}
/*
 * Creates a msg bbox and ask a question
 */
#[no_mangle]
pub unsafe extern "C" fn msg_box(mut text: cptr, mut y: libc::c_int,
                                 mut x: libc::c_int) -> libc::c_char {
    if x == -(1 as libc::c_int) {
        let mut wid: libc::c_int = 0 as libc::c_int;
        let mut hgt: libc::c_int = 0 as libc::c_int;
        Term_get_size(&mut wid, &mut hgt);
        x = wid / 2 as libc::c_int;
        y = hgt / 2 as libc::c_int
    }
    draw_box(y - 1 as libc::c_int,
             (x as
                  libc::c_ulong).wrapping_sub(strlen(text).wrapping_add(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_ulong).wrapping_div(2
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_ulong))
                 as libc::c_int, 2 as libc::c_int,
             strlen(text).wrapping_add(1 as libc::c_int as libc::c_ulong) as
                 libc::c_int);
    c_put_str(1 as libc::c_int as byte_hack, text, y,
              (x as
                   libc::c_ulong).wrapping_sub(strlen(text).wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong).wrapping_div(2
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong)).wrapping_add(1
                                                                                                                                              as
                                                                                                                                              libc::c_int
                                                                                                                                              as
                                                                                                                                              libc::c_ulong)
                  as libc::c_int);
    return inkey();
}
/* Rescale a value */
#[no_mangle]
pub unsafe extern "C" fn rescale(mut x: s32b, mut max: s32b,
                                 mut new_max: s32b) -> s32b {
    return x * new_max / max;
}
/* Nicer wrapper around TERM_XTRA_SCANSUBDIR */
#[no_mangle]
pub unsafe extern "C" fn scansubdir(mut dir: cptr) {
    strnfmt(scansubdir_dir.as_mut_ptr(), 1024 as libc::c_int as uint_hack,
            b"%s\x00" as *const u8 as *const libc::c_char, dir);
    Term_xtra(15 as libc::c_int, 0 as libc::c_int);
}
/*
 * Timers
 */
#[no_mangle]
pub unsafe extern "C" fn new_timer(mut callback: cptr, mut delay: s32b)
 -> *mut timer_type {
    let mut t_ptr: *mut timer_type = 0 as *mut timer_type;
    t_ptr =
        memset(ralloc(::std::mem::size_of::<timer_type>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<timer_type>() as libc::c_ulong) as
            *mut timer_type;
    (*t_ptr).next = gl_timers;
    gl_timers = t_ptr;
    (*t_ptr).callback = string_make(callback);
    (*t_ptr).delay = delay;
    (*t_ptr).countdown = delay;
    (*t_ptr).enabled = 0 as libc::c_int as bool_;
    return t_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn del_timer(mut t_ptr: *mut timer_type) {
    let mut i: *mut timer_type = 0 as *mut timer_type;
    let mut old: *mut timer_type = 0 as *mut timer_type;
    old = 0 as *mut timer_type;
    i = gl_timers;
    while !i.is_null() && i != t_ptr { old = i; i = (*i).next }
    if !i.is_null() {
        if old.is_null() {
            gl_timers = (*t_ptr).next
        } else { (*old).next = (*t_ptr).next }
        string_free((*t_ptr).callback);
        rnfree(t_ptr as vptr,
               ::std::mem::size_of::<timer_type>() as libc::c_ulong);
    } else {
        cmsg_print(10 as libc::c_int as byte_hack,
                   b"Unknown timer!\x00" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_keymap_mode() -> libc::c_int {
    if rogue_like_commands != 0 {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
