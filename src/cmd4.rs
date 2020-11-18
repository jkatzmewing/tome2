use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut ddx: [s16b; 10];
    #[no_mangle]
    static mut ddy: [s16b; 10];
    #[no_mangle]
    static mut color_names: [cptr; 16];
    #[no_mangle]
    static mut window_flag_desc: [cptr; 32];
    #[no_mangle]
    static mut option_info: [option_type; 0];
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut cur_hgt: s16b;
    #[no_mangle]
    static mut cur_wid: s16b;
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut turn: s32b;
    #[no_mangle]
    static mut wizard: bool_;
    #[no_mangle]
    static mut use_graphics: bool_;
    #[no_mangle]
    static mut use_bigtile: bool_;
    #[no_mangle]
    static mut noscore: u16b;
    #[no_mangle]
    static mut inkey_base: bool_;
    #[no_mangle]
    static mut inkey_scan: bool_;
    #[no_mangle]
    static mut m_max: s16b;
    #[no_mangle]
    static mut cheat_peek: bool_;
    #[no_mangle]
    static mut cheat_hear: bool_;
    #[no_mangle]
    static mut cheat_room: bool_;
    #[no_mangle]
    static mut cheat_xtra: bool_;
    #[no_mangle]
    static mut cheat_know: bool_;
    #[no_mangle]
    static mut cheat_live: bool_;
    #[no_mangle]
    static mut hitpoint_warn: byte_hack;
    #[no_mangle]
    static mut delay_factor: byte_hack;
    #[no_mangle]
    static mut autosave_freq: s16b;
    #[no_mangle]
    static mut autosave_t: bool_;
    #[no_mangle]
    static mut autosave_l: bool_;
    #[no_mangle]
    static mut feeling: s16b;
    #[no_mangle]
    static mut player_name: [libc::c_char; 32];
    #[no_mangle]
    static mut player_base: [libc::c_char; 32];
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn streq(s: cptr, t: cptr) -> bool_;
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn ralloc(len: huge_hack) -> vptr;
    #[no_mangle]
    fn string_make(str: cptr) -> cptr;
    #[no_mangle]
    fn string_free(str: cptr) -> errr;
    #[no_mangle]
    fn strnfmt(buf: *mut libc::c_char, max: uint_hack, fmt: cptr, _: ...)
     -> uint_hack;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    static mut Term: *mut term;
    #[no_mangle]
    fn Term_xtra(n: libc::c_int, v: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_fresh() -> errr;
    #[no_mangle]
    fn Term_gotoxy(x: libc::c_int, y: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_draw(x: libc::c_int, y: libc::c_int, a: byte_hack,
                 c: libc::c_char) -> errr;
    #[no_mangle]
    fn Term_addstr(n: libc::c_int, a: byte_hack, s: cptr) -> errr;
    #[no_mangle]
    fn Term_putch(x: libc::c_int, y: libc::c_int, a: byte_hack,
                  c: libc::c_char) -> errr;
    #[no_mangle]
    fn Term_putstr(x: libc::c_int, y: libc::c_int, n: libc::c_int,
                   a: byte_hack, s: cptr) -> errr;
    #[no_mangle]
    fn Term_clear() -> errr;
    #[no_mangle]
    fn Term_redraw() -> errr;
    #[no_mangle]
    fn Term_get_size(w: *mut libc::c_int, h: *mut libc::c_int) -> errr;
    #[no_mangle]
    fn Term_what(x: libc::c_int, y: libc::c_int, a: *mut byte_hack,
                 c: *mut libc::c_char) -> errr;
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
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut macro__num: s16b;
    #[no_mangle]
    static mut macro__pat: *mut cptr;
    #[no_mangle]
    static mut macro__act: *mut cptr;
    #[no_mangle]
    static mut macro__buf: *mut libc::c_char;
    #[no_mangle]
    static mut window_flag: [u32b; 8];
    #[no_mangle]
    static mut angband_term: [*mut term; 8];
    #[no_mangle]
    static mut angband_term_name: [[libc::c_char; 80]; 8];
    #[no_mangle]
    static mut angband_color_table: [[byte_hack; 4]; 256];
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut o_list: *mut object_type;
    #[no_mangle]
    static mut m_list: *mut monster_type;
    #[no_mangle]
    static mut town_info: *mut town_type;
    #[no_mangle]
    static mut keymap_act: [[cptr; 256]; 2];
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut f_info: *mut feature_type;
    #[no_mangle]
    static mut f_name: *mut libc::c_char;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut k_name: *mut libc::c_char;
    #[no_mangle]
    static mut a_info: *mut artifact_type;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut r_name: *mut libc::c_char;
    #[no_mangle]
    static mut d_info: *mut dungeon_info_type;
    #[no_mangle]
    static mut d_name: *mut libc::c_char;
    #[no_mangle]
    static mut t_info: *mut trap_type;
    #[no_mangle]
    static mut t_name: *mut libc::c_char;
    #[no_mangle]
    static mut ANGBAND_SYS: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_FILE: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_USER: cptr;
    #[no_mangle]
    static mut max_r_idx: u16b;
    #[no_mangle]
    static mut max_k_idx: u16b;
    #[no_mangle]
    static mut max_f_idx: u16b;
    #[no_mangle]
    static mut max_a_idx: u16b;
    #[no_mangle]
    static mut max_d_idx: u16b;
    #[no_mangle]
    static mut max_t_idx: u16b;
    #[no_mangle]
    static mut fate_flag: bool_;
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut max_dlv: *mut s16b;
    #[no_mangle]
    static mut take_notes: bool_;
    #[no_mangle]
    static mut generate_special_feeling: bool_;
    #[no_mangle]
    static mut dungeon_flags2: u32b;
    #[no_mangle]
    static mut max_q_idx: s16b;
    #[no_mangle]
    static mut quest: *mut quest_type;
    #[no_mangle]
    fn get_version_string() -> *const libc::c_char;
    #[no_mangle]
    static mut hook_file: *mut FILE;
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn ingame_help(enable: bool_);
    #[no_mangle]
    fn is_quest(level: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cmsg_format(color: byte_hack, fmt: cptr, _: ...);
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn ascii_to_text(buf: *mut libc::c_char, str: cptr);
    #[no_mangle]
    fn macro_add(pat: cptr, act: cptr) -> errr;
    #[no_mangle]
    fn flush();
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn get_check(prompt: cptr) -> bool_;
    #[no_mangle]
    fn handle_stuff();
    #[no_mangle]
    fn bell();
    #[no_mangle]
    fn update_stuff();
    #[no_mangle]
    fn file_character(name: cptr, full: bool_) -> errr;
    #[no_mangle]
    fn get_string(prompt: cptr, buf: *mut libc::c_char, len: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn get_name();
    #[no_mangle]
    fn display_player(mode: libc::c_int);
    #[no_mangle]
    fn message_str(age: libc::c_int) -> cptr;
    #[no_mangle]
    fn message_color(age: libc::c_int) -> byte_hack;
    #[no_mangle]
    fn display_message(x: libc::c_int, y: libc::c_int, split: libc::c_int,
                       color: byte_hack, t: cptr);
    #[no_mangle]
    fn askfor_aux(buf: *mut libc::c_char, len: libc::c_int) -> bool_;
    #[no_mangle]
    fn message_num() -> s16b;
    #[no_mangle]
    fn screen_load();
    #[no_mangle]
    fn get_keymap_dir(ch: libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn move_cursor(row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn c_prt(attr: byte_hack, str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn do_cmd_automatizer();
    #[no_mangle]
    fn my_fclose(fff: *mut FILE) -> errr;
    #[no_mangle]
    fn my_fopen(file: cptr, mode: cptr) -> *mut FILE;
    #[no_mangle]
    fn path_build(buf: *mut libc::c_char, max: libc::c_int, path: cptr,
                  file: cptr) -> errr;
    #[no_mangle]
    fn process_pref_file(name: cptr) -> errr;
    #[no_mangle]
    fn screen_save();
    #[no_mangle]
    fn process_pref_file_aux(buf: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn text_to_ascii(buf: *mut libc::c_char, str: cptr);
    #[no_mangle]
    fn clear_from(row: libc::c_int);
    #[no_mangle]
    fn get_keymap_mode() -> libc::c_int;
    #[no_mangle]
    fn macro_find_exact(pat: cptr) -> sint;
    #[no_mangle]
    fn reset_visuals();
    #[no_mangle]
    fn add_note(note: *mut libc::c_char, code: libc::c_char);
    #[no_mangle]
    fn call_lua(function: cptr, args: cptr, ret: cptr, _: ...) -> bool_;
    #[no_mangle]
    fn get_level_desc(buf: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn get_dungeon_save(buf: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn my_fgets(fff: *mut FILE, buf: *mut libc::c_char, n: huge_hack) -> errr;
    #[no_mangle]
    fn show_notes_file();
    #[no_mangle]
    fn fd_kill(file: cptr) -> errr;
    #[no_mangle]
    fn show_file(name: cptr, what: cptr, line: libc::c_int, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn path_temp(buf: *mut libc::c_char, max: libc::c_int) -> errr;
    #[no_mangle]
    fn dump_fates(OutFile: *mut FILE);
    #[no_mangle]
    fn exec_lua(file: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn race_info_idx(r_idx: libc::c_int, ego: libc::c_int)
     -> *mut monster_race;
    #[no_mangle]
    fn monster_desc(desc: *mut libc::c_char, m_ptr: *mut monster_type,
                    mode: libc::c_int);
    #[no_mangle]
    fn has_ability(ab: libc::c_int) -> bool_;
    #[no_mangle]
    fn dump_corruptions(OutFile: *mut FILE, color: bool_);
    #[no_mangle]
    fn object_desc_store(buf: *mut libc::c_char, o_ptr: *mut object_type,
                         pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn object_prep(o_ptr: *mut object_type, k_idx: libc::c_int);
    #[no_mangle]
    static mut conv_color: [byte_hack; 16];
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn lookup_kind(tval: libc::c_int, sval: libc::c_int) -> s16b;
    #[no_mangle]
    fn bst(what: s32b, t: s32b) -> s32b;
    #[no_mangle]
    fn get_month_name(month: libc::c_int, full: bool_, compact: bool_)
     -> cptr;
    #[no_mangle]
    fn get_day(day: libc::c_int) -> cptr;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub struct feature_type {
    pub name: u32b,
    pub text: u32b,
    pub tunnel: u32b,
    pub block: u32b,
    pub mimic: byte_hack,
    pub flags1: u32b,
    pub extra: byte_hack,
    pub unused: s16b,
    pub d_attr: byte_hack,
    pub d_char: libc::c_char,
    pub x_attr: byte_hack,
    pub x_char: libc::c_char,
    pub shimmer: [byte_hack; 7],
    pub d_dice: [libc::c_int; 4],
    pub d_side: [libc::c_int; 4],
    pub d_frequency: [libc::c_int; 4],
    pub d_type: [libc::c_int; 4],
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
pub struct artifact_type {
    pub name: u32b,
    pub text: u32b,
    pub tval: byte_hack,
    pub sval: byte_hack,
    pub pval: s16b,
    pub to_h: s16b,
    pub to_d: s16b,
    pub to_a: s16b,
    pub activate: s16b,
    pub ac: s16b,
    pub dd: byte_hack,
    pub ds: byte_hack,
    pub weight: s16b,
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
    pub level: byte_hack,
    pub rarity: byte_hack,
    pub cur_num: byte_hack,
    pub max_num: byte_hack,
    pub esp: u32b,
    pub oesp: u32b,
    pub power: s16b,
    pub set: s16b,
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
pub struct trap_type {
    pub probability: s16b,
    pub another: s16b,
    pub p1valinc: s16b,
    pub difficulty: byte_hack,
    pub minlevel: byte_hack,
    pub color: byte_hack,
    pub flags: u32b,
    pub ident: bool_,
    pub known: s16b,
    pub name: s16b,
    pub dd: s16b,
    pub ds: s16b,
    pub text: s16b,
    pub g_attr: byte_hack,
    pub g_char: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cave_type {
    pub info: u16b,
    pub feat: byte_hack,
    pub o_idx: s16b,
    pub m_idx: s16b,
    pub t_idx: s16b,
    pub special: s16b,
    pub special2: s16b,
    pub inscription: s16b,
    pub mana: byte_hack,
    pub mimic: byte_hack,
    pub cost: byte_hack,
    pub when: byte_hack,
    pub effect: s16b,
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
pub struct monster_mind {
    pub dummy: byte_hack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_type {
    pub r_idx: s16b,
    pub ego: u16b,
    pub fy: byte_hack,
    pub fx: byte_hack,
    pub hp: s32b,
    pub maxhp: s32b,
    pub blow: [monster_blow; 4],
    pub speed: byte_hack,
    pub level: byte_hack,
    pub ac: s16b,
    pub exp: u32b,
    pub csleep: s16b,
    pub mspeed: byte_hack,
    pub energy: byte_hack,
    pub stunned: byte_hack,
    pub confused: byte_hack,
    pub monfear: byte_hack,
    pub bleeding: s16b,
    pub poisoned: s16b,
    pub cdis: byte_hack,
    pub mflag: s32b,
    pub ml: bool_,
    pub hold_o_idx: s16b,
    pub smart: u32b,
    pub status: s16b,
    pub target: s16b,
    pub possessor: s16b,
    pub sr_ptr: *mut monster_race,
    pub mind: *mut monster_mind,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option_type {
    pub o_var: *mut bool_,
    pub o_norm: byte_hack,
    pub o_page: byte_hack,
    pub o_bit: byte_hack,
    pub o_text: cptr,
    pub o_desc: cptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct store_type {
    pub st_idx: u16b,
    pub owner: u16b,
    pub insult_cur: s16b,
    pub good_buy: s16b,
    pub bad_buy: s16b,
    pub store_open: s32b,
    pub last_visit: s32b,
    pub stock_num: byte_hack,
    pub stock_size: s16b,
    pub stock: *mut object_type,
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
pub struct town_type {
    pub name: cptr,
    pub seed: u32b,
    pub store: *mut store_type,
    pub numstores: byte_hack,
    pub flags: byte_hack,
    pub stocked: bool_,
    pub destroyed: bool_,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rule_type {
    pub mode: byte_hack,
    pub percent: byte_hack,
    pub mflags1: u32b,
    pub mflags2: u32b,
    pub mflags3: u32b,
    pub mflags4: u32b,
    pub mflags5: u32b,
    pub mflags6: u32b,
    pub mflags7: u32b,
    pub mflags8: u32b,
    pub mflags9: u32b,
    pub r_char: [libc::c_char; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dungeon_info_type {
    pub name: u32b,
    pub text: u32b,
    pub short_name: [libc::c_char; 3],
    pub generator: [libc::c_char; 30],
    pub floor1: s16b,
    pub floor_percent1: [byte_hack; 2],
    pub floor2: s16b,
    pub floor_percent2: [byte_hack; 2],
    pub floor3: s16b,
    pub floor_percent3: [byte_hack; 2],
    pub outer_wall: s16b,
    pub inner_wall: s16b,
    pub fill_type1: s16b,
    pub fill_percent1: [byte_hack; 2],
    pub fill_type2: s16b,
    pub fill_percent2: [byte_hack; 2],
    pub fill_type3: s16b,
    pub fill_percent3: [byte_hack; 2],
    pub fill_method: byte_hack,
    pub mindepth: s16b,
    pub maxdepth: s16b,
    pub principal: bool_,
    pub next: byte_hack,
    pub min_plev: byte_hack,
    pub min_m_alloc_level: libc::c_int,
    pub max_m_alloc_chance: libc::c_int,
    pub flags1: u32b,
    pub flags2: u32b,
    pub size_x: libc::c_int,
    pub size_y: libc::c_int,
    pub rule_percents: [byte_hack; 100],
    pub rules: [rule_type; 5],
    pub final_object: libc::c_int,
    pub final_artifact: libc::c_int,
    pub final_guardian: libc::c_int,
    pub ix: libc::c_int,
    pub iy: libc::c_int,
    pub ox: libc::c_int,
    pub oy: libc::c_int,
    pub objs: obj_theme,
    pub d_dice: [libc::c_int; 4],
    pub d_side: [libc::c_int; 4],
    pub d_frequency: [libc::c_int; 4],
    pub d_type: [libc::c_int; 4],
    pub t_idx: [s16b; 4],
    pub t_level: [s16b; 4],
    pub t_num: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quest_type {
    pub silent: bool_,
    pub dynamic_desc: bool_,
    pub name: [libc::c_char; 40],
    pub desc: [[libc::c_char; 80]; 10],
    pub status: s16b,
    pub level: s16b,
    pub plot: *mut s16b,
    pub type_0: byte_hack,
    pub init: Option<unsafe extern "C" fn(_: libc::c_int) -> bool_>,
    pub data: [s32b; 4],
    pub gen_desc: Option<unsafe extern "C" fn(_: *mut FILE) -> bool_>,
}
/* File: cmd4.c */
/* Purpose: Interface commands */
/*
 * Copyright (c) 1989 James E. Wilson, Robert A. Koeneke
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Hack -- redraw the screen
 *
 * This command performs various low level updates, clears all the "extra"
 * windows, does a total redraw of the main window, and requests all of the
 * interesting updates and redraws that I can think of.
 *
 * This command is also used to "instantiate" the results of the user
 * selecting various things, such as graphics mode, so it must call
 * the "TERM_XTRA_REACT" hook before redrawing the windows.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_redraw() {
    let mut j: libc::c_int = 0;
    let mut old: *mut term = Term;
    /* Hack -- react to changes */
    Term_xtra(10 as libc::c_int, 0 as libc::c_int);
    /* Combine and Reorder the pack (later) */
    (*p_ptr).notice =
        ((*p_ptr).notice as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
    /* Update torch */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x2 as libc::c_long) as u32b;
    /* Update stuff */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x1 as libc::c_long | 0x10 as libc::c_long |
                  0x20 as libc::c_long | 0x40 as libc::c_long |
                  0x80 as libc::c_long | 0x8 as libc::c_long |
                  0x4 as libc::c_long)) as u32b;
    /* Forget view */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x10000 as libc::c_long) as u32b;
    /* Update view */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x100000 as libc::c_long) as u32b;
    /* Update monster light */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x200000 as libc::c_long) as u32b;
    /* Update monsters */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long) as u32b;
    /* Redraw everything */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long |
             (0x8000000 as libc::c_long | 0x2000000 as libc::c_long |
                  0x1000000 as libc::c_long | 0x4000000 as libc::c_long)) as
            u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long | 0x8 as libc::c_long
                  | 0x10 as libc::c_long)) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x40 as libc::c_long | 0x80 as libc::c_long |
                  0x100 as libc::c_long | 0x200 as libc::c_long)) as u32b;
    /* Hack -- update */
    handle_stuff();
    /* Redraw every window */
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        /* Dead window */
        if !angband_term[j as usize].is_null() {
            /* Activate */
            Term_activate(angband_term[j as usize]);
            /* Redraw */
            Term_redraw();
            /* Refresh */
            Term_fresh();
            /* Restore */
            Term_activate(old);
        }
        j += 1
    };
}
/*
 * Hack -- change name
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_change_name() {
    let mut c: libc::c_char = 0;
    let mut mode: libc::c_int = 0 as libc::c_int;
    let mut tmp: [libc::c_char; 160] = [0; 160];
    /* Enter "icky" mode */
    character_icky = 1 as libc::c_int as bool_;
    /* Save the screen */
    Term_save();
    loop 
         /* Forever */
         /* keep mode below 7 */
         {
        mode = (mode + 6 as libc::c_int) % 6 as libc::c_int;
        /* Display the player */
        display_player(mode);
        /* Prompt */
        if mode == 0 as libc::c_int {
            Term_putstr(14 as libc::c_int, 22 as libc::c_int,
                        -(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                        b"[\'t/T\' to change tactics, \'e/E\' to change movement]\x00"
                            as *const u8 as *const libc::c_char);
        }
        Term_putstr(4 as libc::c_int, 23 as libc::c_int, -(1 as libc::c_int),
                    1 as libc::c_int as byte_hack,
                    b"[\'c\' to change name, \'f\' to file, \'p\' for previous, \'n\' for next, or ESC]\x00"
                        as *const u8 as *const libc::c_char);
        /* Query */
        c = inkey();
        /* Exit */
        if c as libc::c_int == '\u{1b}' as i32 { break ; }
        /* Change name */
        if c as libc::c_int == 'c' as i32 {
            get_name();
        } else if c as libc::c_int == 'f' as i32 {
            strnfmt(tmp.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                    b"%s.txt\x00" as *const u8 as *const libc::c_char,
                    player_name.as_mut_ptr());
            if get_string(b"Filename(you can post it to http://angband.oook.cz/): \x00"
                              as *const u8 as *const libc::c_char,
                          tmp.as_mut_ptr(), 80 as libc::c_int) != 0 {
                if tmp[0 as libc::c_int as usize] as libc::c_int != 0 &&
                       tmp[0 as libc::c_int as usize] as libc::c_int !=
                           ' ' as i32 {
                    file_character(tmp.as_mut_ptr() as cptr,
                                   0 as libc::c_int as bool_);
                }
            }
        } else if c as libc::c_int == 'n' as i32 {
            mode += 1
        } else if c as libc::c_int == 'p' as i32 {
            mode -= 1
        } else if mode == 0 as libc::c_int {
            /* File dump */
            /* Toggle mode */
            /* Change tactic */
            if c as libc::c_int == 't' as i32 {
                do_cmd_change_tactic(-(1 as libc::c_int));
            } else if c as libc::c_int == 'T' as i32 {
                do_cmd_change_tactic(1 as libc::c_int);
            } else if c as libc::c_int == 'e' as i32 {
                do_cmd_change_movement(-(1 as libc::c_int));
            } else if c as libc::c_int == 'E' as i32 {
                do_cmd_change_movement(1 as libc::c_int);
            } else { bell(); }
        } else {
            /* Change movement */
            /* Oops */
            bell();
        }
        /* Flush messages */
        msg_print(0 as cptr);
    }
    /* Restore the screen */
    Term_load();
    /* Leave "icky" mode */
    character_icky = 0 as libc::c_int as bool_;
    /* Redraw everything */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long |
             (0x8000000 as libc::c_long | 0x2000000 as libc::c_long |
                  0x1000000 as libc::c_long | 0x4000000 as libc::c_long)) as
            u32b;
    handle_stuff();
}
/*
 * Recall the most recent message
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_message_one() {
    let mut msg: cptr =
        format(b"> %s\x00" as *const u8 as *const libc::c_char,
               message_str(0 as libc::c_int)) as cptr;
    /* Recall one message XXX XXX XXX */
    display_message(0 as libc::c_int, 0 as libc::c_int,
                    strlen(msg) as libc::c_int,
                    message_color(0 as libc::c_int), msg);
}
/*
 * Show previous messages to the user	-BEN-
 *
 * The screen format uses line 0 and (Term->hgt - 1) for headers and prompts,
 * skips line 1 and (Term->hgt - 2), and uses line 2 thru (Term->hgt - 3) for
 * old messages.
 *
 * This command shows you which commands you are viewing, and allows
 * you to "search" for strings in the recall.
 *
 * Note that messages may be longer than 80 characters, but they are
 * displayed using "infinite" length, with a special sub-command to
 * "slide" the virtual display to the left or right.
 *
 * Attempt to only hilite the matching portions of the string.
 *
 * Now taking advantages of big-screen. -pav-
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_messages() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut q: u32b = 0;
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    let mut shower: [libc::c_char; 80] = [0; 80];
    let mut finder: [libc::c_char; 80] = [0; 80];
    /* Wipe finder */
    strcpy(finder.as_mut_ptr(), b"\x00" as *const u8 as *const libc::c_char);
    /* Wipe shower */
    strcpy(shower.as_mut_ptr(), b"\x00" as *const u8 as *const libc::c_char);
    /* Total messages */
    n = message_num() as libc::c_int;
    /* Start on first message */
    i = 0 as libc::c_int;
    /* Start at leftmost edge */
    q = 0 as libc::c_int as u32b;
    /* Enter "icky" mode */
    character_icky = 1 as libc::c_int as bool_;
    /* Save the screen */
    Term_save();
    loop 
         /* Process requests until done */
         /* Clear screen */
         {
        Term_clear();
        /* Retrieve current screen size */
        Term_get_size(&mut wid, &mut hgt);
        /* Dump up to 20 (or more in bigscreen) lines of messages */
        j = 0 as libc::c_int;
        while j < hgt - 4 as libc::c_int && i + j < n {
            let mut msg: cptr = message_str(i + j);
            let mut color: byte_hack = message_color(i + j);
            /* Apply horizontal scroll */
            msg =
                if strlen(msg) >= q as libc::c_ulong {
                    msg.offset(q as isize)
                } else { b"\x00" as *const u8 as *const libc::c_char };
            /* Dump the messages, bottom to top */
            display_message(0 as libc::c_int, hgt - 3 as libc::c_int - j,
                            strlen(msg) as libc::c_int, color, msg);
            /* Hilite "shower" */
            if shower[0 as libc::c_int as usize] != 0 {
                let mut str: cptr = msg;
                loop 
                     /* Display matches */
                     {
                    str = strstr(str, shower.as_mut_ptr()) as cptr;
                    if str.is_null() { break ; }
                    let mut len: libc::c_int =
                        strlen(shower.as_mut_ptr()) as libc::c_int;
                    /* Display the match */
                    Term_putstr(str.wrapping_offset_from(msg) as libc::c_long
                                    as libc::c_int,
                                hgt - 3 as libc::c_int - j, len,
                                11 as libc::c_int as byte_hack,
                                shower.as_mut_ptr() as cptr);
                    /* Advance */
                    str = str.offset(len as isize)
                }
            }
            j += 1
        }
        /* Display header XXX XXX XXX */
        prt(format(b"Message Recall (%d-%d of %d), Offset %d\x00" as *const u8
                       as *const libc::c_char, i, i + j - 1 as libc::c_int, n,
                   q) as cptr, 0 as libc::c_int, 0 as libc::c_int);
        /* Display prompt (not very informative) */
        prt(b"[Press \'p\' for older, \'n\' for newer, ..., or ESCAPE]\x00" as
                *const u8 as *const libc::c_char, hgt - 1 as libc::c_int,
            0 as libc::c_int);
        /* Get a command */
        k = inkey() as libc::c_int;
        /* Exit on Escape */
        if k == '\u{1b}' as i32 { break ; }
        /* Hack -- Save the old index */
        j = i;
        /* Horizontal scroll */
        if k == '4' as i32 {
            /* Scroll left */
            q =
                if q >=
                       (wid as
                            u32b).wrapping_div(2 as libc::c_int as
                                                   libc::c_uint) {
                    q.wrapping_sub((wid / 2 as libc::c_int) as libc::c_uint)
                } else { 0 as libc::c_int as libc::c_uint }
        } else if k == '6' as i32 {
            /* Horizontal scroll */
            /* Scroll right */
            q = q.wrapping_add((wid / 2 as libc::c_int) as libc::c_uint)
        } else if k == '=' as i32 {
            /* Hack -- handle show */
            /* Prompt */
            prt(b"Show: \x00" as *const u8 as *const libc::c_char,
                hgt - 1 as libc::c_int, 0 as libc::c_int);
            /* Get a "shower" string, or continue */
            if askfor_aux(shower.as_mut_ptr(), 80 as libc::c_int) == 0 {
                continue ;
            }
        } else {
            /* Hack -- handle find */
            if k == '/' as i32 {
                let mut z: s16b = 0;
                /* Prompt */
                prt(b"Find: \x00" as *const u8 as *const libc::c_char,
                    hgt - 1 as libc::c_int, 0 as libc::c_int);
                /* Get a "finder" string, or continue */
                if askfor_aux(finder.as_mut_ptr(), 80 as libc::c_int) == 0 {
                    continue ;
                }
                /* Show it */
                strcpy(shower.as_mut_ptr(), finder.as_mut_ptr());
                /* Scan messages */
                z = (i + 1 as libc::c_int) as s16b;
                while (z as libc::c_int) < n {
                    let mut msg_0: cptr = message_str(z as libc::c_int);
                    /* Search for it */
                    if !strstr(msg_0, finder.as_mut_ptr()).is_null() {
                        /* New location */
                        i = z as libc::c_int;
                        break ;
                    } else { z += 1 }
                }
            }
            /* Recall 1 older message */
            if k == '8' as i32 || k == '\n' as i32 || k == '\r' as i32 {
                /* Go newer if legal */
                if (i + 1 as libc::c_int) < n { i += 1 as libc::c_int }
            }
            /* Recall 10 older messages */
            if k == '+' as i32 {
                /* Go older if legal */
                if (i + 10 as libc::c_int) < n { i += 10 as libc::c_int }
            }
            /* Recall one screen of older messages */
            if k == 'p' as i32 || k == 'P' as i32 & 0x1f as libc::c_int ||
                   k == ' ' as i32 {
                /* Go older if legal */
                if (i + (hgt - 4 as libc::c_int)) < n {
                    i += hgt - 4 as libc::c_int
                }
            }
            /* Recall one screen of newer messages */
            if k == 'n' as i32 || k == 'N' as i32 & 0x1f as libc::c_int {
                /* Go newer (if able) */
                i =
                    if i >= hgt - 4 as libc::c_int {
                        (i) - (hgt - 4 as libc::c_int)
                    } else { 0 as libc::c_int }
            }
            /* Recall 10 newer messages */
            if k == '-' as i32 {
                /* Go newer (if able) */
                i =
                    if i >= 10 as libc::c_int {
                        (i) - 10 as libc::c_int
                    } else { 0 as libc::c_int }
            }
            /* Recall 1 newer messages */
            if k == '2' as i32 {
                /* Go newer (if able) */
                i =
                    if i >= 1 as libc::c_int {
                        (i) - 1 as libc::c_int
                    } else { 0 as libc::c_int }
            }
            /* Hack -- Error of some kind */
            if i == j { bell(); }
        }
    }
    /* Restore the screen */
    Term_load();
    /* Leave "icky" mode */
    character_icky = 0 as libc::c_int as bool_;
}
/*
 * Cheating options
 */
static mut cheat_info: [option_type; 6] =
    unsafe {
        [{
             let mut init =
                 option_type{o_var: &cheat_peek as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 0 as libc::c_int as byte_hack,
                             o_bit: 0 as libc::c_int as byte_hack,
                             o_text:
                                 b"cheat_peek\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Peek into object creation\x00" as *const u8
                                     as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var: &cheat_hear as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 0 as libc::c_int as byte_hack,
                             o_bit: 1 as libc::c_int as byte_hack,
                             o_text:
                                 b"cheat_hear\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Peek into monster creation\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var: &cheat_room as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 0 as libc::c_int as byte_hack,
                             o_bit: 2 as libc::c_int as byte_hack,
                             o_text:
                                 b"cheat_room\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Peek into dungeon creation\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var: &cheat_xtra as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 0 as libc::c_int as byte_hack,
                             o_bit: 3 as libc::c_int as byte_hack,
                             o_text:
                                 b"cheat_xtra\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Peek into something else\x00" as *const u8
                                     as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var: &cheat_know as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 0 as libc::c_int as byte_hack,
                             o_bit: 4 as libc::c_int as byte_hack,
                             o_text:
                                 b"cheat_know\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Know complete monster info\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var: &cheat_live as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 0 as libc::c_int as byte_hack,
                             o_bit: 5 as libc::c_int as byte_hack,
                             o_text:
                                 b"cheat_live\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Allow player to avoid death\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         }]
    };
/*
 * Interact with some options for cheating
 */
unsafe extern "C" fn do_cmd_options_cheat(mut info: cptr) {
    let mut ch: libc::c_char = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 6 as libc::c_int;
    let mut dir: libc::c_int = 0;
    let mut buf: [libc::c_char; 80] = [0; 80];
    /* Clear screen */
    Term_clear();
    loop 
         /* Interact with the player */
         /* Prompt XXX XXX XXX */
         {
        strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"%s (RET to advance, y/n to set, ESC to accept) \x00" as
                    *const u8 as *const libc::c_char, info);
        prt(buf.as_mut_ptr() as cptr, 0 as libc::c_int, 0 as libc::c_int);
        /* Display the options */
        i = 0 as libc::c_int;
        while i < n {
            let mut a: byte_hack = 1 as libc::c_int as byte_hack;
            /* Color current option */
            if i == k { a = 14 as libc::c_int as byte_hack }
            /* Display the option text */
            strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                    b"%-48s: %s  (%s)\x00" as *const u8 as
                        *const libc::c_char, cheat_info[i as usize].o_desc,
                    if *cheat_info[i as usize].o_var as libc::c_int != 0 {
                        b"yes\x00" as *const u8 as *const libc::c_char
                    } else { b"no \x00" as *const u8 as *const libc::c_char },
                    cheat_info[i as usize].o_text);
            c_prt(a, buf.as_mut_ptr() as cptr, i + 2 as libc::c_int,
                  0 as libc::c_int);
            i += 1
        }
        /* Hilite current option */
        move_cursor(k + 2 as libc::c_int, 50 as libc::c_int);
        /* Get a key */
        ch = inkey();
        /*
		 * Hack -- Try to translate the key into a direction
		 * to allow the use of roguelike keys for navigation
		 */
        dir = get_keymap_dir(ch);
        if dir == 2 as libc::c_int || dir == 4 as libc::c_int ||
               dir == 6 as libc::c_int || dir == 8 as libc::c_int {
            ch = (dir + '0' as i32) as libc::c_char
        }
        /* Analyze */
        match ch as libc::c_int {
            27 => { return }
            45 | 56 => { k = (n + k - 1 as libc::c_int) % n }
            32 | 10 | 13 | 50 => { k = (k + 1 as libc::c_int) % n }
            121 | 89 | 54 => {
                noscore =
                    (noscore as libc::c_int |
                         cheat_info[k as usize].o_page as libc::c_int *
                             256 as libc::c_int +
                             cheat_info[k as usize].o_bit as libc::c_int) as
                        u16b;
                *cheat_info[k as usize].o_var = 1 as libc::c_int as bool_;
                k = (k + 1 as libc::c_int) % n
            }
            110 | 78 | 52 => {
                *cheat_info[k as usize].o_var = 0 as libc::c_int as bool_;
                k = (k + 1 as libc::c_int) % n
            }
            _ => { bell(); }
        }
    };
}
static mut autosave_info: [option_type; 2] =
    unsafe {
        [{
             let mut init =
                 option_type{o_var: &autosave_l as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 0 as libc::c_int as byte_hack,
                             o_bit: 6 as libc::c_int as byte_hack,
                             o_text:
                                 b"autosave_l\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Autosave when entering new levels\x00" as
                                     *const u8 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 option_type{o_var: &autosave_t as *const bool_ as *mut bool_,
                             o_norm: 0 as libc::c_int as byte_hack,
                             o_page: 0 as libc::c_int as byte_hack,
                             o_bit: 7 as libc::c_int as byte_hack,
                             o_text:
                                 b"autosave_t\x00" as *const u8 as
                                     *const libc::c_char,
                             o_desc:
                                 b"Timed autosave\x00" as *const u8 as
                                     *const libc::c_char,};
             init
         }]
    };
#[no_mangle]
pub unsafe extern "C" fn toggle_frequency(mut current: s16b) -> s16b {
    if current as libc::c_int == 0 as libc::c_int {
        return 50 as libc::c_int as s16b
    }
    if current as libc::c_int == 50 as libc::c_int {
        return 100 as libc::c_int as s16b
    }
    if current as libc::c_int == 100 as libc::c_int {
        return 250 as libc::c_int as s16b
    }
    if current as libc::c_int == 250 as libc::c_int {
        return 500 as libc::c_int as s16b
    }
    if current as libc::c_int == 500 as libc::c_int {
        return 1000 as libc::c_int as s16b
    }
    if current as libc::c_int == 1000 as libc::c_int {
        return 2500 as libc::c_int as s16b
    }
    if current as libc::c_int == 2500 as libc::c_int {
        return 5000 as libc::c_int as s16b
    }
    if current as libc::c_int == 5000 as libc::c_int {
        return 10000 as libc::c_int as s16b
    }
    if current as libc::c_int == 10000 as libc::c_int {
        return 25000 as libc::c_int as s16b
    }
    return 0 as libc::c_int as s16b;
}
/*
 * Interact with some options for cheating
 */
unsafe extern "C" fn do_cmd_options_autosave(mut info: cptr) {
    let mut ch: libc::c_char = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 2 as libc::c_int;
    let mut dir: libc::c_int = 0;
    let mut buf: [libc::c_char; 80] = [0; 80];
    /* Clear screen */
    Term_clear();
    loop 
         /* Interact with the player */
         /* Prompt XXX XXX XXX */
         {
        strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"%s (RET to advance, y/n to set, \'F\' for frequency, ESC to accept) \x00"
                    as *const u8 as *const libc::c_char, info);
        prt(buf.as_mut_ptr() as cptr, 0 as libc::c_int, 0 as libc::c_int);
        /* Display the options */
        i = 0 as libc::c_int;
        while i < n {
            let mut a: byte_hack = 1 as libc::c_int as byte_hack;
            /* Color current option */
            if i == k { a = 14 as libc::c_int as byte_hack }
            /* Display the option text */
            strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                    b"%-48s: %s  (%s)\x00" as *const u8 as
                        *const libc::c_char, autosave_info[i as usize].o_desc,
                    if *autosave_info[i as usize].o_var as libc::c_int != 0 {
                        b"yes\x00" as *const u8 as *const libc::c_char
                    } else { b"no \x00" as *const u8 as *const libc::c_char },
                    autosave_info[i as usize].o_text);
            c_prt(a, buf.as_mut_ptr() as cptr, i + 2 as libc::c_int,
                  0 as libc::c_int);
            i += 1
        }
        prt(format(b"Timed autosave frequency: every %d turns\x00" as
                       *const u8 as *const libc::c_char,
                   autosave_freq as libc::c_int) as cptr, 5 as libc::c_int,
            0 as libc::c_int);
        /* Hilite current option */
        move_cursor(k + 2 as libc::c_int, 50 as libc::c_int);
        /* Get a key */
        ch = inkey();
        /*
		 * Hack -- Try to translate the key into a direction
		 * to allow the use of roguelike keys for navigation
		 */
        dir = get_keymap_dir(ch);
        if dir == 2 as libc::c_int || dir == 4 as libc::c_int ||
               dir == 6 as libc::c_int || dir == 8 as libc::c_int {
            ch = (dir + '0' as i32) as libc::c_char
        }
        /* Analyze */
        match ch as libc::c_int {
            27 => { return }
            45 | 56 => { k = (n + k - 1 as libc::c_int) % n }
            32 | 10 | 13 | 50 => { k = (k + 1 as libc::c_int) % n }
            121 | 89 | 54 => {
                *autosave_info[k as usize].o_var = 1 as libc::c_int as bool_;
                k = (k + 1 as libc::c_int) % n
            }
            110 | 78 | 52 => {
                *autosave_info[k as usize].o_var = 0 as libc::c_int as bool_;
                k = (k + 1 as libc::c_int) % n
            }
            102 | 70 => {
                autosave_freq = toggle_frequency(autosave_freq);
                prt(format(b"Timed autosave frequency: every %d turns\x00" as
                               *const u8 as *const libc::c_char,
                           autosave_freq as libc::c_int) as cptr,
                    5 as libc::c_int, 0 as libc::c_int);
            }
            _ => { bell(); }
        }
    };
}
/* Switch an option by only knowing its name */
#[no_mangle]
pub unsafe extern "C" fn change_option(mut name: cptr, mut value: bool_)
 -> bool_ {
    let mut i: libc::c_int = 0;
    /* Scan the options */
    i = 0 as libc::c_int;
    while !(*option_info.as_mut_ptr().offset(i as isize)).o_desc.is_null() {
        if strcmp((*option_info.as_mut_ptr().offset(i as isize)).o_text, name)
               == 0 {
            let mut old: bool_ =
                *(*option_info.as_mut_ptr().offset(i as isize)).o_var;
            *(*option_info.as_mut_ptr().offset(i as isize)).o_var = value;
            return old
        }
        i += 1
    }
    cmsg_format(10 as libc::c_int as byte_hack,
                b"Warning, change_option couldn\'t find option \'%s\'.\x00" as
                    *const u8 as *const libc::c_char, name);
    return 0 as libc::c_int as bool_;
}
/*
 * Interact with some options
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_options_aux(mut page: libc::c_int,
                                            mut info: cptr,
                                            mut read_only: bool_) {
    let mut ch: libc::c_char = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut dir: libc::c_int = 0;
    let mut opt: [libc::c_int; 24] = [0; 24];
    let mut buf: [libc::c_char; 80] = [0; 80];
    /* Lookup the options */
    i = 0 as libc::c_int;
    while i < 24 as libc::c_int { opt[i as usize] = 0 as libc::c_int; i += 1 }
    /* Scan the options */
    i = 0 as libc::c_int;
    while !(*option_info.as_mut_ptr().offset(i as isize)).o_desc.is_null() {
        /* Notice options on this "page" */
        if (*option_info.as_mut_ptr().offset(i as isize)).o_page as
               libc::c_int == page {
            let fresh0 = n;
            n = n + 1;
            opt[fresh0 as usize] = i
        }
        i += 1
    }
    /* Clear screen */
    Term_clear();
    loop 
         /* Interact with the player */
         /* Prompt XXX XXX XXX */
         {
        strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"%s (RET to advance, y/n to set, ESC to accept) \x00" as
                    *const u8 as *const libc::c_char, info);
        prt(buf.as_mut_ptr() as cptr, 0 as libc::c_int, 0 as libc::c_int);
        /* Display the options */
        i = 0 as libc::c_int;
        while i < n {
            let mut a: byte_hack = 1 as libc::c_int as byte_hack;
            /* Color current option */
            if i == k { a = 14 as libc::c_int as byte_hack }
            /* Display the option text */
            strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                    b"%-48s: %s  (%s)\x00" as *const u8 as
                        *const libc::c_char,
                    (*option_info.as_mut_ptr().offset(opt[i as usize] as
                                                          isize)).o_desc,
                    if *(*option_info.as_mut_ptr().offset(opt[i as usize] as
                                                              isize)).o_var as
                           libc::c_int != 0 {
                        b"yes\x00" as *const u8 as *const libc::c_char
                    } else { b"no \x00" as *const u8 as *const libc::c_char },
                    (*option_info.as_mut_ptr().offset(opt[i as usize] as
                                                          isize)).o_text);
            c_prt(a, buf.as_mut_ptr() as cptr, i + 2 as libc::c_int,
                  0 as libc::c_int);
            i += 1
        }
        /* Hilite current option */
        move_cursor(k + 2 as libc::c_int, 50 as libc::c_int);
        /* Get a key */
        ch = inkey();
        /*
		 * Hack -- Try to translate the key into a direction
		 * to allow the use of roguelike keys for navigation
		 */
        dir = get_keymap_dir(ch);
        if dir == 2 as libc::c_int || dir == 4 as libc::c_int ||
               dir == 6 as libc::c_int || dir == 8 as libc::c_int {
            ch = (dir + '0' as i32) as libc::c_char
        }
        /* Analyze */
        match ch as libc::c_int {
            27 => { return }
            45 | 56 => { k = (n + k - 1 as libc::c_int) % n }
            32 | 10 | 13 | 50 => { k = (k + 1 as libc::c_int) % n }
            121 | 89 | 54 => {
                if !(read_only != 0) {
                    *(*option_info.as_mut_ptr().offset(opt[k as usize] as
                                                           isize)).o_var =
                        1 as libc::c_int as bool_;
                    k = (k + 1 as libc::c_int) % n
                }
            }
            110 | 78 | 52 => {
                if !(read_only != 0) {
                    *(*option_info.as_mut_ptr().offset(opt[k as usize] as
                                                           isize)).o_var =
                        0 as libc::c_int as bool_;
                    k = (k + 1 as libc::c_int) % n
                }
            }
            _ => { bell(); }
        }
    };
}
/*
 * Modify the "window" options
 */
unsafe extern "C" fn do_cmd_options_win() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut ch: libc::c_char = 0;
    let mut go: bool_ = 1 as libc::c_int as bool_;
    let mut old_flag: [u32b; 8] = [0; 8];
    /* Memorize old flags */
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        /* Acquire current flags */
        old_flag[j as usize] = window_flag[j as usize];
        j += 1
    }
    /* Clear screen */
    Term_clear();
    /* Interact */
    while go != 0 {
        /* Prompt XXX XXX XXX */
        prt(b"Window Flags (<dir>, t, y, n, ESC) \x00" as *const u8 as
                *const libc::c_char, 0 as libc::c_int, 0 as libc::c_int);
        /* Display the windows */
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            let mut a: byte_hack = 1 as libc::c_int as byte_hack;
            let mut s: cptr =
                angband_term_name[j as usize].as_mut_ptr() as cptr;
            /* Use color */
            if j == x { a = 14 as libc::c_int as byte_hack }
            /* Window name, staggered, centered */
            Term_putstr(((35 as libc::c_int + j * 5 as libc::c_int) as
                             libc::c_ulong).wrapping_sub(strlen(s).wrapping_div(2
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong))
                            as libc::c_int,
                        2 as libc::c_int + j % 2 as libc::c_int,
                        -(1 as libc::c_int), a, s);
            j += 1
        }
        /* Display the options */
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            let mut a_0: byte_hack = 1 as libc::c_int as byte_hack;
            let mut str: cptr = window_flag_desc[i as usize];
            /* Use color */
            if i == y { a_0 = 14 as libc::c_int as byte_hack }
            /* Unused option */
            if str.is_null() {
                str =
                    b"(Unused option)\x00" as *const u8 as *const libc::c_char
            }
            /* Flag name */
            Term_putstr(0 as libc::c_int, i + 5 as libc::c_int,
                        -(1 as libc::c_int), a_0, str);
            /* Display the windows */
            j = 0 as libc::c_int;
            while j < 8 as libc::c_int {
                let mut a_1: byte_hack = 1 as libc::c_int as byte_hack;
                let mut c: libc::c_char = '.' as i32 as libc::c_char;
                /* Use color */
                if i == y && j == x { a_1 = 14 as libc::c_int as byte_hack }
                /* Active flag */
                if window_flag[j as usize] as libc::c_long &
                       (1 as libc::c_long) << i != 0 {
                    c = 'X' as i32 as libc::c_char
                }
                /* Flag value */
                Term_putch(35 as libc::c_int + j * 5 as libc::c_int,
                           i + 5 as libc::c_int, a_1, c);
                j += 1
            }
            i += 1
        }
        /* Place Cursor */
        Term_gotoxy(35 as libc::c_int + x * 5 as libc::c_int,
                    y + 5 as libc::c_int);
        /* Get key */
        ch = inkey();
        let mut current_block_46: u64;
        /* Analyze */
        match ch as libc::c_int {
            27 => {
                go = 0 as libc::c_int as bool_;
                current_block_46 = 13303144130133872306;
            }
            84 | 116 => {
                /* Clear windows */
                j = 0 as libc::c_int;
                while j < 8 as libc::c_int {
                    window_flag[j as usize] =
                        (window_flag[j as usize] as libc::c_long &
                             !((1 as libc::c_long) << y)) as u32b;
                    j += 1
                }
                /* Fall through */
                i = 0 as libc::c_int;
                while i < 16 as libc::c_int {
                    window_flag[x as usize] =
                        (window_flag[x as usize] as libc::c_long &
                             !((1 as libc::c_long) << i)) as u32b;
                    i += 1
                }
                current_block_46 = 13460095289871124136;
            }
            121 | 89 => { current_block_46 = 13460095289871124136; }
            110 | 78 => {
                /* Clear flags */
                /* Clear flag */
                window_flag[x as usize] =
                    (window_flag[x as usize] as libc::c_long &
                         !((1 as libc::c_long) << y)) as u32b;
                current_block_46 = 13303144130133872306;
            }
            _ => {
                d = get_keymap_dir(ch);
                x =
                    (x + ddx[d as usize] as libc::c_int + 8 as libc::c_int) %
                        8 as libc::c_int;
                y =
                    (y + ddy[d as usize] as libc::c_int + 16 as libc::c_int) %
                        16 as libc::c_int;
                if d == 0 { bell(); }
                current_block_46 = 13303144130133872306;
            }
        }
        match current_block_46 {
            13460095289871124136 =>
            /* Ignore screen */
            {
                if !(x == 0 as libc::c_int) {
                    /* Set flag */
                    window_flag[x as usize] =
                        (window_flag[x as usize] as libc::c_long |
                             (1 as libc::c_long) << y) as u32b
                }
            }
            _ => { }
        }
    }
    /* Notice changes */
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        let mut old: *mut term = Term;
        /* Dead window */
        if !angband_term[j as usize].is_null() {
            /* Ignore non-changes */
            if !(window_flag[j as usize] == old_flag[j as usize]) {
                /* Activate */
                Term_activate(angband_term[j as usize]);
                /* Erase */
                Term_clear();
                /* Refresh */
                Term_fresh();
                /* Restore */
                Term_activate(old);
            }
        }
        j += 1
    };
}
/*
 * Write all current options to the given preference file in the
 * lib/user directory. Modified from KAmband 1.8.
 */
unsafe extern "C" fn option_dump(mut fname: cptr) -> errr {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_USER,
               fname);
    /* File type is "TEXT" */
    /* Append to the file */
    fff =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"a\x00" as *const u8 as *const libc::c_char);
    /* Failure */
    if fff.is_null() { return -(1 as libc::c_int) }
    /* Skip some lines */
    fprintf(fff, b"\n\n\x00" as *const u8 as *const libc::c_char);
    /* Start dumping */
    fprintf(fff,
            b"# Automatic option dump\n\n\x00" as *const u8 as
                *const libc::c_char);
    /* Dump options (skip cheat, adult, score) */
    i = 0 as libc::c_int;
    while !(*option_info.as_mut_ptr().offset(i as isize)).o_var.is_null() {
        /* Require a real option */
        if !(*option_info.as_mut_ptr().offset(i as isize)).o_text.is_null() {
            /* No birth options */
            if !((*option_info.as_mut_ptr().offset(i as isize)).o_page as
                     libc::c_int == 6 as libc::c_int) {
                /* Comment */
                fprintf(fff,
                        b"# Option \'%s\'\n\x00" as *const u8 as
                            *const libc::c_char,
                        (*option_info.as_mut_ptr().offset(i as
                                                              isize)).o_desc);
                /* Dump the option */
                if *(*option_info.as_mut_ptr().offset(i as isize)).o_var != 0
                   {
                    fprintf(fff,
                            b"Y:%s\n\x00" as *const u8 as *const libc::c_char,
                            (*option_info.as_mut_ptr().offset(i as
                                                                  isize)).o_text);
                } else {
                    fprintf(fff,
                            b"X:%s\n\x00" as *const u8 as *const libc::c_char,
                            (*option_info.as_mut_ptr().offset(i as
                                                                  isize)).o_text);
                }
                /* Skip a line */
                fprintf(fff, b"\n\x00" as *const u8 as *const libc::c_char);
            }
        }
        i += 1
    }
    /* Dump window flags */
    i = 1 as libc::c_int;
    while i < 8 as libc::c_int {
        /* Require a real window */
        if !angband_term[i as usize].is_null() {
            /* Check each flag */
            j = 0 as libc::c_int;
            while j < 32 as libc::c_int {
                /* Require a real flag */
                if !window_flag_desc[j as usize].is_null() {
                    /* Comment */
                    fprintf(fff,
                            b"# Window \'%s\', Flag \'%s\'\n\x00" as *const u8
                                as *const libc::c_char,
                            angband_term_name[i as usize].as_mut_ptr(),
                            window_flag_desc[j as usize]);
                    /* Dump the flag */
                    if window_flag[i as usize] as libc::c_long &
                           (1 as libc::c_long) << j != 0 {
                        fprintf(fff,
                                b"W:%d:%d:1\n\x00" as *const u8 as
                                    *const libc::c_char, i, j);
                    } else {
                        fprintf(fff,
                                b"W:%d:%d:0\n\x00" as *const u8 as
                                    *const libc::c_char, i, j);
                    }
                    /* Skip a line */
                    fprintf(fff,
                            b"\n\x00" as *const u8 as *const libc::c_char);
                }
                j += 1
            }
        }
        i += 1
    }
    /* Close */
    my_fclose(fff);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Ask for a "user pref file" and process it.
 *
 * This function should only be used by standard interaction commands,
 * in which a standard "Command:" prompt is present on the given row.
 *
 * Allow absolute file names?  XXX XXX XXX
 */
unsafe extern "C" fn do_cmd_pref_file_hack(mut row: libc::c_int) {
    let mut ftmp: [libc::c_char; 80] = [0; 80];
    /* Prompt */
    prt(b"Command: Load a user pref file\x00" as *const u8 as
            *const libc::c_char, row, 0 as libc::c_int);
    /* Prompt */
    prt(b"File: \x00" as *const u8 as *const libc::c_char,
        row + 2 as libc::c_int, 0 as libc::c_int);
    /* Default filename */
    strnfmt(ftmp.as_mut_ptr(), 80 as libc::c_int as uint_hack,
            b"%s.prf\x00" as *const u8 as *const libc::c_char,
            player_base.as_mut_ptr());
    /* Ask for a file (or cancel) */
    if askfor_aux(ftmp.as_mut_ptr(), 80 as libc::c_int) == 0 { return }
    /* Process the given filename */
    if process_pref_file(ftmp.as_mut_ptr() as cptr) != 0 {
        /* Mention failure */
        msg_format(b"Failed to load \'%s\'!\x00" as *const u8 as
                       *const libc::c_char, ftmp.as_mut_ptr());
    } else {
        /* Mention success */
        msg_format(b"Loaded \'%s\'.\x00" as *const u8 as *const libc::c_char,
                   ftmp.as_mut_ptr());
    };
}
/*
 * Set or unset various options.
 *
 * The user must use the "Ctrl-R" command to "adapt" to changes
 * in any options which control "visual" aspects of the game.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_options() {
    let mut k: libc::c_int = 0;
    /* Save the screen */
    screen_save();
    loop 
         /* Interact */
         /* Clear screen */
         {
        Term_clear();
        /* Why are we here */
        prt(b"Options\x00" as *const u8 as *const libc::c_char,
            2 as libc::c_int, 0 as libc::c_int);
        /* Give some choices */
        prt(b"(1) User Interface Options\x00" as *const u8 as
                *const libc::c_char, 4 as libc::c_int, 5 as libc::c_int);
        prt(b"(2) Disturbance Options\x00" as *const u8 as
                *const libc::c_char, 5 as libc::c_int, 5 as libc::c_int);
        prt(b"(3) Game-Play Options\x00" as *const u8 as *const libc::c_char,
            6 as libc::c_int, 5 as libc::c_int);
        prt(b"(4) Efficiency Options\x00" as *const u8 as *const libc::c_char,
            7 as libc::c_int, 5 as libc::c_int);
        prt(b"(5) ToME Options\x00" as *const u8 as *const libc::c_char,
            8 as libc::c_int, 5 as libc::c_int);
        prt(b"(6) Birth Options(read only)\x00" as *const u8 as
                *const libc::c_char, 9 as libc::c_int, 5 as libc::c_int);
        /* Special choices */
        prt(b"(D) Base Delay Factor\x00" as *const u8 as *const libc::c_char,
            10 as libc::c_int, 5 as libc::c_int);
        prt(b"(H) Hitpoint Warning\x00" as *const u8 as *const libc::c_char,
            11 as libc::c_int, 5 as libc::c_int);
        prt(b"(A) Autosave Options\x00" as *const u8 as *const libc::c_char,
            12 as libc::c_int, 5 as libc::c_int);
        /* Automatizer */
        prt(b"(T) Automatizer\x00" as *const u8 as *const libc::c_char,
            14 as libc::c_int, 5 as libc::c_int);
        /* Window flags */
        prt(b"(W) Window Flags\x00" as *const u8 as *const libc::c_char,
            16 as libc::c_int, 5 as libc::c_int);
        /* Cheating */
        prt(b"(C) Cheating Options\x00" as *const u8 as *const libc::c_char,
            18 as libc::c_int, 5 as libc::c_int);
        /* Dump */
        prt(b"(U) Dump Options setting\x00" as *const u8 as
                *const libc::c_char, 20 as libc::c_int, 5 as libc::c_int);
        prt(b"(O) Load Options setting\x00" as *const u8 as
                *const libc::c_char, 21 as libc::c_int, 5 as libc::c_int);
        /* Prompt */
        prt(b"Command: \x00" as *const u8 as *const libc::c_char,
            22 as libc::c_int, 0 as libc::c_int);
        /* Get command */
        k = inkey() as libc::c_int;
        /* Exit */
        if k == '\u{1b}' as i32 { break ; }
        /* Analyze */
        match k {
            111 | 79 => {
                /* Load a user pref file */
                /* Ask for and load a user pref file */
                do_cmd_pref_file_hack(21 as libc::c_int);
            }
            117 | 85 => {
                /* Append options to a file */
                let mut ftmp: [libc::c_char; 80] = [0; 80];
                /* Prompt */
                prt(b"Command: Append options to a file\x00" as *const u8 as
                        *const libc::c_char, 21 as libc::c_int,
                    0 as libc::c_int);
                /* Prompt */
                prt(b"File: \x00" as *const u8 as *const libc::c_char,
                    21 as libc::c_int, 0 as libc::c_int);
                /* Default filename */
                strnfmt(ftmp.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                        b"%s.prf\x00" as *const u8 as *const libc::c_char,
                        player_base.as_mut_ptr());
                /* Ask for a file */
                if askfor_aux(ftmp.as_mut_ptr(), 80 as libc::c_int) == 0 {
                    continue ;
                }
                /* Dump the options */
                if option_dump(ftmp.as_mut_ptr() as cptr) != 0 {
                    /* Failure */
                    msg_print(b"Failed!\x00" as *const u8 as
                                  *const libc::c_char);
                } else {
                    /* Success */
                    msg_print(b"Done.\x00" as *const u8 as
                                  *const libc::c_char);
                }
            }
            49 => {
                /* General Options */
                /* Process the general options */
                do_cmd_options_aux(1 as libc::c_int,
                                   b"User Interface Options\x00" as *const u8
                                       as *const libc::c_char,
                                   0 as libc::c_int as bool_);
            }
            50 => {
                /* Disturbance Options */
                /* Spawn */
                do_cmd_options_aux(2 as libc::c_int,
                                   b"Disturbance Options\x00" as *const u8 as
                                       *const libc::c_char,
                                   0 as libc::c_int as bool_);
            }
            51 => {
                /* Inventory Options */
                /* Spawn */
                do_cmd_options_aux(3 as libc::c_int,
                                   b"Game-Play Options\x00" as *const u8 as
                                       *const libc::c_char,
                                   0 as libc::c_int as bool_);
            }
            52 => {
                /* Efficiency Options */
                /* Spawn */
                do_cmd_options_aux(4 as libc::c_int,
                                   b"Efficiency Options\x00" as *const u8 as
                                       *const libc::c_char,
                                   0 as libc::c_int as bool_);
            }
            53 => {
                /* ToME Options */
                do_cmd_options_aux(5 as libc::c_int,
                                   b"ToME Options\x00" as *const u8 as
                                       *const libc::c_char,
                                   0 as libc::c_int as bool_);
            }
            54 => {
                /* Birth Options - read only */
                do_cmd_options_aux(6 as libc::c_int,
                                   b"Birth Options(read only)\x00" as
                                       *const u8 as *const libc::c_char,
                                   1 as libc::c_int as bool_);
            }
            67 => {
                /* Cheating Options */
                /* Spawn */
                do_cmd_options_cheat(b"Cheaters never win\x00" as *const u8 as
                                         *const libc::c_char);
            }
            116 | 84 => { do_cmd_automatizer(); }
            97 | 65 => {
                do_cmd_options_autosave(b"Autosave\x00" as *const u8 as
                                            *const libc::c_char);
            }
            87 | 119 => {
                /* Window flags */
                /* Spawn */
                do_cmd_options_win();
            }
            68 | 100 => {
                /* Hack -- Delay Speed */
                /* Prompt */
                prt(b"Command: Base Delay Factor\x00" as *const u8 as
                        *const libc::c_char, 21 as libc::c_int,
                    0 as libc::c_int);
                loop 
                     /* Get a new value */
                     {
                    let mut msec: libc::c_int =
                        delay_factor as libc::c_int *
                            delay_factor as libc::c_int *
                            delay_factor as libc::c_int;
                    prt(format(b"Current base delay factor: %d (%d msec)\x00"
                                   as *const u8 as *const libc::c_char,
                               delay_factor as libc::c_int, msec) as cptr,
                        22 as libc::c_int, 0 as libc::c_int);
                    prt(b"Delay Factor (0-9 or ESC to accept): \x00" as
                            *const u8 as *const libc::c_char,
                        23 as libc::c_int, 0 as libc::c_int);
                    k = inkey() as libc::c_int;
                    if k == '\u{1b}' as i32 { break ; }
                    if *(*__ctype_b_loc()).offset(k as isize) as libc::c_int &
                           _ISdigit as libc::c_int as libc::c_ushort as
                               libc::c_int != 0 {
                        delay_factor = (k - '0' as i32) as byte_hack
                    } else { bell(); }
                }
            }
            72 | 104 => {
                /* Hack -- hitpoint warning factor */
                /* Prompt */
                prt(b"Command: Hitpoint Warning\x00" as *const u8 as
                        *const libc::c_char, 18 as libc::c_int,
                    0 as libc::c_int);
                loop 
                     /* Get a new value */
                     {
                    prt(format(b"Current hitpoint warning: %d0%%\x00" as
                                   *const u8 as *const libc::c_char,
                               hitpoint_warn as libc::c_int) as cptr,
                        22 as libc::c_int, 0 as libc::c_int);
                    prt(b"Hitpoint Warning (0-9 or ESC to accept): \x00" as
                            *const u8 as *const libc::c_char,
                        20 as libc::c_int, 0 as libc::c_int);
                    k = inkey() as libc::c_int;
                    if k == '\u{1b}' as i32 { break ; }
                    if *(*__ctype_b_loc()).offset(k as isize) as libc::c_int &
                           _ISdigit as libc::c_int as libc::c_ushort as
                               libc::c_int != 0 {
                        hitpoint_warn = (k - '0' as i32) as byte_hack
                    } else { bell(); }
                }
            }
            _ => {
                /* Unknown option */
                /* Oops */
                bell();
            }
        }
        /* Flush messages */
        msg_print(0 as cptr);
    }
    /* Restore the screen */
    screen_load();
    /* Set the ingame help */
    ingame_help((*p_ptr).help.enabled);
}
/*
 * Ask for a "user pref line" and process it
 *
 * XXX XXX XXX Allow absolute file names?
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_pref() {
    let mut buf: [libc::c_char; 80] = [0; 80];
    /* Default */
    strcpy(buf.as_mut_ptr(), b"\x00" as *const u8 as *const libc::c_char);
    /* Ask for a "user pref command" */
    if get_string(b"Pref: \x00" as *const u8 as *const libc::c_char,
                  buf.as_mut_ptr(), 80 as libc::c_int) == 0 {
        return
    }
    /* Process that pref command */
    process_pref_file_aux(buf.as_mut_ptr());
}
/*
 * Hack -- append all current macros to the given file
 */
unsafe extern "C" fn macro_dump(mut fname: cptr) -> errr {
    let mut i: libc::c_int = 0;
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_USER,
               fname);
    /* File type is "TEXT" */
    /* Append to the file */
    fff =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"a\x00" as *const u8 as *const libc::c_char);
    /* Failure */
    if fff.is_null() { return -(1 as libc::c_int) }
    /* Skip space */
    fprintf(fff, b"\n\n\x00" as *const u8 as *const libc::c_char);
    /* Start dumping */
    fprintf(fff,
            b"# Automatic macro dump\n\n\x00" as *const u8 as
                *const libc::c_char);
    /* Dump them */
    i = 0 as libc::c_int;
    while i < macro__num as libc::c_int {
        /* Start the macro */
        fprintf(fff,
                b"# Macro \'%d\'\n\n\x00" as *const u8 as *const libc::c_char,
                i);
        /* Extract the action */
        ascii_to_text(buf.as_mut_ptr(), *macro__act.offset(i as isize));
        /* Dump the macro */
        fprintf(fff, b"A:%s\n\x00" as *const u8 as *const libc::c_char,
                buf.as_mut_ptr());
        /* Extract the action */
        ascii_to_text(buf.as_mut_ptr(), *macro__pat.offset(i as isize));
        /* Dump normal macros */
        fprintf(fff, b"P:%s\n\x00" as *const u8 as *const libc::c_char,
                buf.as_mut_ptr());
        /* End the macro */
        fprintf(fff, b"\n\n\x00" as *const u8 as *const libc::c_char);
        i += 1
    }
    /* Start dumping */
    fprintf(fff, b"\n\n\n\n\x00" as *const u8 as *const libc::c_char);
    /* Close */
    my_fclose(fff);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Hack -- ask for a "trigger" (see below)
 *
 * Note the complex use of the "inkey()" function from "util.c".
 *
 * Note that both "flush()" calls are extremely important.
 */
unsafe extern "C" fn do_cmd_macro_aux(mut buf: *mut libc::c_char,
                                      mut macro_screen: bool_) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut tmp: [libc::c_char; 1024] = [0; 1024];
    /* Flush */
    flush();
    /* Do not process macros */
    inkey_base = 1 as libc::c_int as bool_;
    /* First key */
    i = inkey() as libc::c_int;
    /* Read the pattern */
    while i != 0 {
        /* Save the key */
        let fresh1 = n;
        n = n + 1;
        *buf.offset(fresh1 as isize) = i as libc::c_char;
        /* Do not process macros */
        inkey_base = 1 as libc::c_int as bool_;
        /* Do not wait for keys */
        inkey_scan = 1 as libc::c_int as bool_;
        /* Attempt to read a key */
        i = inkey() as libc::c_int
    }
    /* Terminate */
    *buf.offset(n as isize) = '\u{0}' as i32 as libc::c_char;
    /* Flush */
    flush();
    if macro_screen != 0 {
        /* Convert the trigger */
        ascii_to_text(tmp.as_mut_ptr(), buf as cptr);
        /* Hack -- display the trigger */
        Term_addstr(-(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                    tmp.as_mut_ptr() as cptr);
    };
}
/*
 * Hack -- ask for a keymap "trigger" (see below)
 *
 * Note that both "flush()" calls are extremely important.  This may
 * no longer be true, since "util.c" is much simpler now.  XXX XXX XXX
 */
unsafe extern "C" fn do_cmd_macro_aux_keymap(mut buf: *mut libc::c_char) {
    let mut tmp: [libc::c_char; 1024] = [0; 1024];
    /* Flush */
    flush();
    /* Get a key */
    *buf.offset(0 as libc::c_int as isize) = inkey();
    *buf.offset(1 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    /* Convert to ascii */
    ascii_to_text(tmp.as_mut_ptr(), buf as cptr);
    /* Hack -- display the trigger */
    Term_addstr(-(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                tmp.as_mut_ptr() as cptr);
    /* Flush */
    flush();
}
/*
 * Hack -- append all keymaps to the given file
 */
unsafe extern "C" fn keymap_dump(mut fname: cptr) -> errr {
    let mut i: libc::c_int = 0;
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut key: [libc::c_char; 1024] = [0; 1024];
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut mode: libc::c_int = 0;
    /* Keymap mode */
    mode = get_keymap_mode();
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_USER,
               fname);
    /* File type is "TEXT" */
    /* Append to the file */
    fff =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"a\x00" as *const u8 as *const libc::c_char);
    /* Failure */
    if fff.is_null() { return -(1 as libc::c_int) }
    /* Skip space */
    fprintf(fff, b"\n\n\x00" as *const u8 as *const libc::c_char);
    /* Start dumping */
    fprintf(fff,
            b"# Automatic keymap dump\n\n\x00" as *const u8 as
                *const libc::c_char);
    /* Dump them */
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        let mut act: cptr = 0 as *const libc::c_char;
        /* Loop up the keymap */
        act = keymap_act[mode as usize][i as usize];
        /* Skip empty keymaps */
        if !act.is_null() {
            /* Encode the key */
            buf[0 as libc::c_int as usize] = i as libc::c_char;
            buf[1 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
            ascii_to_text(key.as_mut_ptr(), buf.as_mut_ptr() as cptr);
            /* Encode the action */
            ascii_to_text(buf.as_mut_ptr(), act);
            /* Dump the macro */
            fprintf(fff, b"A:%s\n\x00" as *const u8 as *const libc::c_char,
                    buf.as_mut_ptr());
            fprintf(fff, b"C:%d:%s\n\x00" as *const u8 as *const libc::c_char,
                    mode, key.as_mut_ptr());
        }
        i += 1
    }
    /* Start dumping */
    fprintf(fff, b"\n\n\n\x00" as *const u8 as *const libc::c_char);
    /* Close */
    my_fclose(fff);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Interact with "macros"
 *
 * Note that the macro "action" must be defined before the trigger.
 *
 * Could use some helpful instructions on this page.  XXX XXX XXX
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_macros() {
    let mut i: libc::c_int = 0;
    let mut tmp: [libc::c_char; 1024] = [0; 1024];
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut mode: libc::c_int = 0;
    /* Keymap mode */
    mode = get_keymap_mode();
    /* File type is "TEXT" */
    /* Enter "icky" mode */
    character_icky = 1 as libc::c_int as bool_;
    /* Save screen */
    Term_save();
    loop 
         /* Process requests until done */
         /* Clear screen */
         {
        Term_clear();
        /* Describe */
        prt(b"Interact with Macros\x00" as *const u8 as *const libc::c_char,
            2 as libc::c_int, 0 as libc::c_int);
        /* Describe that action */
        prt(b"Current action (if any) shown below:\x00" as *const u8 as
                *const libc::c_char, 20 as libc::c_int, 0 as libc::c_int);
        /* Analyze the current action */
        ascii_to_text(buf.as_mut_ptr(), macro__buf as cptr);
        /* Display the current action */
        prt(buf.as_mut_ptr() as cptr, 22 as libc::c_int, 0 as libc::c_int);
        /* Selections */
        prt(b"(1) Load a user pref file\x00" as *const u8 as
                *const libc::c_char, 4 as libc::c_int, 5 as libc::c_int);
        prt(b"(2) Append macros to a file\x00" as *const u8 as
                *const libc::c_char, 5 as libc::c_int, 5 as libc::c_int);
        prt(b"(3) Query a macro\x00" as *const u8 as *const libc::c_char,
            6 as libc::c_int, 5 as libc::c_int);
        prt(b"(4) Create a macro\x00" as *const u8 as *const libc::c_char,
            7 as libc::c_int, 5 as libc::c_int);
        prt(b"(5) Remove a macro\x00" as *const u8 as *const libc::c_char,
            8 as libc::c_int, 5 as libc::c_int);
        prt(b"(6) Append keymaps to a file\x00" as *const u8 as
                *const libc::c_char, 9 as libc::c_int, 5 as libc::c_int);
        prt(b"(7) Query a keymap\x00" as *const u8 as *const libc::c_char,
            10 as libc::c_int, 5 as libc::c_int);
        prt(b"(8) Create a keymap\x00" as *const u8 as *const libc::c_char,
            11 as libc::c_int, 5 as libc::c_int);
        prt(b"(9) Remove a keymap\x00" as *const u8 as *const libc::c_char,
            12 as libc::c_int, 5 as libc::c_int);
        prt(b"(0) Enter a new action\x00" as *const u8 as *const libc::c_char,
            13 as libc::c_int, 5 as libc::c_int);
        /* Prompt */
        prt(b"Command: \x00" as *const u8 as *const libc::c_char,
            16 as libc::c_int, 0 as libc::c_int);
        /* Get a command */
        i = inkey() as libc::c_int;
        /* Leave */
        if i == '\u{1b}' as i32 { break ; }
        /* Load a 'macro' file */
        if i == '1' as i32 {
            /* Prompt */
            prt(b"Command: Load a user pref file\x00" as *const u8 as
                    *const libc::c_char, 16 as libc::c_int, 0 as libc::c_int);
            /* Prompt */
            prt(b"File: \x00" as *const u8 as *const libc::c_char,
                18 as libc::c_int, 0 as libc::c_int);
            /* Default filename */
            strnfmt(tmp.as_mut_ptr(), 1024 as libc::c_int as uint_hack,
                    b"%s.prf\x00" as *const u8 as *const libc::c_char,
                    player_name.as_mut_ptr());
            /* Ask for a file */
            if askfor_aux(tmp.as_mut_ptr(), 80 as libc::c_int) == 0 {
                continue ;
            }
            /* Process the given filename */
            if 0 as libc::c_int != process_pref_file(tmp.as_mut_ptr() as cptr)
               {
                /* Prompt */
                msg_print(b"Could not load file!\x00" as *const u8 as
                              *const libc::c_char);
            }
        } else if i == '2' as i32 {
            /* Save macros */
            /* Prompt */
            prt(b"Command: Append macros to a file\x00" as *const u8 as
                    *const libc::c_char, 16 as libc::c_int, 0 as libc::c_int);
            /* Prompt */
            prt(b"File: \x00" as *const u8 as *const libc::c_char,
                18 as libc::c_int, 0 as libc::c_int);
            /* Default filename */
            strnfmt(tmp.as_mut_ptr(), 1024 as libc::c_int as uint_hack,
                    b"%s.prf\x00" as *const u8 as *const libc::c_char,
                    player_name.as_mut_ptr());
            /* Ask for a file */
            if askfor_aux(tmp.as_mut_ptr(), 80 as libc::c_int) == 0 {
                continue ;
            }
            /* Dump the macros */
            macro_dump(tmp.as_mut_ptr() as cptr);
            /* Prompt */
            msg_print(b"Appended macros.\x00" as *const u8 as
                          *const libc::c_char);
        } else if i == '3' as i32 {
            let mut k: libc::c_int = 0;
            /* Query a macro */
            /* Prompt */
            prt(b"Command: Query a macro\x00" as *const u8 as
                    *const libc::c_char, 16 as libc::c_int, 0 as libc::c_int);
            /* Prompt */
            prt(b"Trigger: \x00" as *const u8 as *const libc::c_char,
                18 as libc::c_int, 0 as libc::c_int);
            /* Get a macro trigger */
            do_cmd_macro_aux(buf.as_mut_ptr(), 1 as libc::c_int as bool_);
            /* Acquire action */
            k = macro_find_exact(buf.as_mut_ptr() as cptr);
            /* Nothing found */
            if k < 0 as libc::c_int {
                /* Prompt */
                msg_print(b"Found no macro.\x00" as *const u8 as
                              *const libc::c_char);
            } else {
                /* Found one */
                /* Obtain the action */
                strcpy(macro__buf, *macro__act.offset(k as isize));
                ascii_to_text(buf.as_mut_ptr(), macro__buf as cptr);
                prt(buf.as_mut_ptr() as cptr, 22 as libc::c_int,
                    0 as libc::c_int);
                msg_print(b"Found a macro.\x00" as *const u8 as
                              *const libc::c_char);
            }
        } else if i == '4' as i32 {
            /* Analyze the current action */
            /* Display the current action */
            /* Prompt */
            /* Create a macro */
            /* Prompt */
            prt(b"Command: Create a macro\x00" as *const u8 as
                    *const libc::c_char, 16 as libc::c_int, 0 as libc::c_int);
            /* Prompt */
            prt(b"Trigger: \x00" as *const u8 as *const libc::c_char,
                18 as libc::c_int, 0 as libc::c_int);
            /* Get a macro trigger */
            do_cmd_macro_aux(buf.as_mut_ptr(), 1 as libc::c_int as bool_);
            /* Clear */
            clear_from(20 as libc::c_int);
            /* Prompt */
            prt(b"Action: \x00" as *const u8 as *const libc::c_char,
                20 as libc::c_int, 0 as libc::c_int);
            /* Convert to text */
            ascii_to_text(tmp.as_mut_ptr(), macro__buf as cptr);
            /* Get an encoded action */
            if askfor_aux(tmp.as_mut_ptr(), 80 as libc::c_int) != 0 {
                /* Convert to ascii */
                text_to_ascii(macro__buf, tmp.as_mut_ptr() as cptr);
                /* Link the macro */
                macro_add(buf.as_mut_ptr() as cptr, macro__buf as cptr);
                /* Prompt */
                msg_print(b"Added a macro.\x00" as *const u8 as
                              *const libc::c_char);
            }
        } else if i == '5' as i32 {
            /* Remove a macro */
            /* Prompt */
            prt(b"Command: Remove a macro\x00" as *const u8 as
                    *const libc::c_char, 16 as libc::c_int, 0 as libc::c_int);
            /* Prompt */
            prt(b"Trigger: \x00" as *const u8 as *const libc::c_char,
                18 as libc::c_int, 0 as libc::c_int);
            /* Get a macro trigger */
            do_cmd_macro_aux(buf.as_mut_ptr(), 1 as libc::c_int as bool_);
            /* Link the macro */
            macro_add(buf.as_mut_ptr() as cptr, buf.as_mut_ptr() as cptr);
            /* Prompt */
            msg_print(b"Removed a macro.\x00" as *const u8 as
                          *const libc::c_char);
        } else if i == '6' as i32 {
            /* Save keymaps */
            /* Prompt */
            prt(b"Command: Append keymaps to a file\x00" as *const u8 as
                    *const libc::c_char, 16 as libc::c_int, 0 as libc::c_int);
            /* Prompt */
            prt(b"File: \x00" as *const u8 as *const libc::c_char,
                18 as libc::c_int, 0 as libc::c_int);
            /* Default filename */
            strnfmt(tmp.as_mut_ptr(), 1024 as libc::c_int as uint_hack,
                    b"%s.prf\x00" as *const u8 as *const libc::c_char,
                    player_name.as_mut_ptr());
            /* Ask for a file */
            if askfor_aux(tmp.as_mut_ptr(), 80 as libc::c_int) == 0 {
                continue ;
            }
            /* Dump the macros */
            keymap_dump(tmp.as_mut_ptr() as cptr);
            /* Prompt */
            msg_print(b"Appended keymaps.\x00" as *const u8 as
                          *const libc::c_char);
        } else if i == '7' as i32 {
            let mut act: cptr = 0 as *const libc::c_char;
            /* Query a keymap */
            /* Prompt */
            prt(b"Command: Query a keymap\x00" as *const u8 as
                    *const libc::c_char, 16 as libc::c_int, 0 as libc::c_int);
            /* Prompt */
            prt(b"Keypress: \x00" as *const u8 as *const libc::c_char,
                18 as libc::c_int, 0 as libc::c_int);
            /* Get a keymap trigger */
            do_cmd_macro_aux_keymap(buf.as_mut_ptr());
            /* Look up the keymap */
            act =
                keymap_act[mode as
                               usize][buf[0 as libc::c_int as usize] as
                                          byte_hack as usize];
            /* Nothing found */
            if act.is_null() {
                /* Prompt */
                msg_print(b"Found no keymap.\x00" as *const u8 as
                              *const libc::c_char);
            } else {
                /* Found one */
                /* Obtain the action */
                strcpy(macro__buf, act);
                ascii_to_text(buf.as_mut_ptr(), macro__buf as cptr);
                prt(buf.as_mut_ptr() as cptr, 22 as libc::c_int,
                    0 as libc::c_int);
                msg_print(b"Found a keymap.\x00" as *const u8 as
                              *const libc::c_char);
            }
        } else if i == '8' as i32 {
            /* Analyze the current action */
            /* Display the current action */
            /* Prompt */
            /* Create a keymap */
            /* Prompt */
            prt(b"Command: Create a keymap\x00" as *const u8 as
                    *const libc::c_char, 16 as libc::c_int, 0 as libc::c_int);
            /* Prompt */
            prt(b"Keypress: \x00" as *const u8 as *const libc::c_char,
                18 as libc::c_int, 0 as libc::c_int);
            /* Get a keymap trigger */
            do_cmd_macro_aux_keymap(buf.as_mut_ptr());
            /* Clear */
            clear_from(20 as libc::c_int);
            /* Prompt */
            prt(b"Action: \x00" as *const u8 as *const libc::c_char,
                20 as libc::c_int, 0 as libc::c_int);
            /* Convert to text */
            ascii_to_text(tmp.as_mut_ptr(), macro__buf as cptr);
            /* Get an encoded action */
            if askfor_aux(tmp.as_mut_ptr(), 80 as libc::c_int) != 0 {
                /* Convert to ascii */
                text_to_ascii(macro__buf, tmp.as_mut_ptr() as cptr);
                /* Free old keymap */
                string_free(keymap_act[mode as
                                           usize][buf[0 as libc::c_int as
                                                          usize] as byte_hack
                                                      as usize]);
                /* Make new keymap */
                keymap_act[mode as
                               usize][buf[0 as libc::c_int as usize] as
                                          byte_hack as usize] =
                    string_make(macro__buf as cptr);
                /* Prompt */
                msg_print(b"Added a keymap.\x00" as *const u8 as
                              *const libc::c_char);
            }
        } else if i == '9' as i32 {
            /* Remove a keymap */
            /* Prompt */
            prt(b"Command: Remove a keymap\x00" as *const u8 as
                    *const libc::c_char, 16 as libc::c_int, 0 as libc::c_int);
            /* Prompt */
            prt(b"Keypress: \x00" as *const u8 as *const libc::c_char,
                18 as libc::c_int, 0 as libc::c_int);
            /* Get a keymap trigger */
            do_cmd_macro_aux_keymap(buf.as_mut_ptr());
            /* Free old keymap */
            string_free(keymap_act[mode as
                                       usize][buf[0 as libc::c_int as usize]
                                                  as byte_hack as usize]);
            /* Make new keymap */
            keymap_act[mode as
                           usize][buf[0 as libc::c_int as usize] as byte_hack
                                      as usize] = 0 as cptr;
            /* Prompt */
            msg_print(b"Removed a keymap.\x00" as *const u8 as
                          *const libc::c_char);
        } else if i == '0' as i32 {
            /* Enter a new action */
            /* Prompt */
            prt(b"Command: Enter a new action\x00" as *const u8 as
                    *const libc::c_char, 16 as libc::c_int, 0 as libc::c_int);
            /* Go to the correct location */
            Term_gotoxy(0 as libc::c_int, 22 as libc::c_int);
            /* Hack -- limit the value */
            tmp[80 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
            /* Get an encoded action */
            if askfor_aux(buf.as_mut_ptr(), 80 as libc::c_int) == 0 {
                continue ;
            }
            /* Extract an action */
            text_to_ascii(macro__buf, buf.as_mut_ptr() as cptr);
        } else {
            /* Oops */
            /* Oops */
            bell();
        }
        /* Flush messages */
        msg_print(0 as cptr);
    }
    /* Load screen */
    Term_load();
    /* Leave "icky" mode */
    character_icky = 0 as libc::c_int as bool_;
}
/*
 * Interact with "visuals"
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_visuals() {
    let mut i: libc::c_int = 0;
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut tmp: [libc::c_char; 160] = [0; 160];
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* File type is "TEXT" */
    /* Enter "icky" mode */
    character_icky = 1 as libc::c_int as bool_;
    /* Save the screen */
    Term_save();
    loop 
         /* Interact until done */
         /* Clear screen */
         {
        Term_clear();
        /* Ask for a choice */
        prt(b"Interact with Visuals\x00" as *const u8 as *const libc::c_char,
            2 as libc::c_int, 0 as libc::c_int);
        /* Give some choices */
        prt(b"(1) Load a user pref file\x00" as *const u8 as
                *const libc::c_char, 4 as libc::c_int, 5 as libc::c_int);
        prt(b"(2) Dump monster attr/chars\x00" as *const u8 as
                *const libc::c_char, 5 as libc::c_int, 5 as libc::c_int);
        prt(b"(3) Dump object attr/chars\x00" as *const u8 as
                *const libc::c_char, 6 as libc::c_int, 5 as libc::c_int);
        prt(b"(4) Dump feature attr/chars\x00" as *const u8 as
                *const libc::c_char, 7 as libc::c_int, 5 as libc::c_int);
        prt(b"(5) (unused)\x00" as *const u8 as *const libc::c_char,
            8 as libc::c_int, 5 as libc::c_int);
        prt(b"(6) Change monster attr/chars\x00" as *const u8 as
                *const libc::c_char, 9 as libc::c_int, 5 as libc::c_int);
        prt(b"(7) Change object attr/chars\x00" as *const u8 as
                *const libc::c_char, 10 as libc::c_int, 5 as libc::c_int);
        prt(b"(8) Change feature attr/chars\x00" as *const u8 as
                *const libc::c_char, 11 as libc::c_int, 5 as libc::c_int);
        prt(b"(9) (unused)\x00" as *const u8 as *const libc::c_char,
            12 as libc::c_int, 5 as libc::c_int);
        prt(b"(0) Reset visuals\x00" as *const u8 as *const libc::c_char,
            13 as libc::c_int, 5 as libc::c_int);
        /* Prompt */
        prt(b"Command: \x00" as *const u8 as *const libc::c_char,
            15 as libc::c_int, 0 as libc::c_int);
        /* Prompt */
        i = inkey() as libc::c_int;
        /* Done */
        if i == '\u{1b}' as i32 { break ; }
        /* Load a 'pref' file */
        if i == '1' as i32 {
            /* Prompt */
            prt(b"Command: Load a user pref file\x00" as *const u8 as
                    *const libc::c_char, 15 as libc::c_int, 0 as libc::c_int);
            /* Prompt */
            prt(b"File: \x00" as *const u8 as *const libc::c_char,
                17 as libc::c_int, 0 as libc::c_int);
            /* Default filename */
            strnfmt(tmp.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                    b"user-%s.prf\x00" as *const u8 as *const libc::c_char,
                    ANGBAND_SYS);
            /* Query */
            if askfor_aux(tmp.as_mut_ptr(), 70 as libc::c_int) == 0 {
                continue ;
            }
            /* Process the given filename */
            process_pref_file(tmp.as_mut_ptr() as cptr);
        } else if i == '2' as i32 {
            /* Dump monster attr/chars */
            /* Prompt */
            prt(b"Command: Dump monster attr/chars\x00" as *const u8 as
                    *const libc::c_char, 15 as libc::c_int, 0 as libc::c_int);
            /* Prompt */
            prt(b"File: \x00" as *const u8 as *const libc::c_char,
                17 as libc::c_int, 0 as libc::c_int);
            /* Default filename */
            strnfmt(tmp.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                    b"user-%s.prf\x00" as *const u8 as *const libc::c_char,
                    ANGBAND_SYS);
            /* Get a filename */
            if askfor_aux(tmp.as_mut_ptr(), 70 as libc::c_int) == 0 {
                continue ;
            }
            /* Build the filename */
            path_build(buf.as_mut_ptr(), 1024 as libc::c_int,
                       ANGBAND_DIR_USER, tmp.as_mut_ptr() as cptr);
            /* Append to the file */
            fff =
                my_fopen(buf.as_mut_ptr() as cptr,
                         b"a\x00" as *const u8 as *const libc::c_char);
            /* Failure */
            if fff.is_null() { continue ; }
            /* Start dumping */
            fprintf(fff, b"\n\n\x00" as *const u8 as *const libc::c_char);
            fprintf(fff,
                    b"# Monster attr/char definitions\n\n\x00" as *const u8 as
                        *const libc::c_char);
            /* Dump monsters */
            i = 0 as libc::c_int;
            while i < max_r_idx as libc::c_int {
                let mut r_ptr: *mut monster_race =
                    &mut *r_info.offset(i as isize) as *mut monster_race;
                /* Skip non-entries */
                if !((*r_ptr).name == 0) {
                    /* Dump a comment */
                    fprintf(fff,
                            b"# %s\n\x00" as *const u8 as *const libc::c_char,
                            r_name.offset((*r_ptr).name as isize));
                    /* Dump the monster attr/char info */
                    fprintf(fff,
                            b"R:%d:0x%02X:0x%02X\n\n\x00" as *const u8 as
                                *const libc::c_char, i,
                            (*r_ptr).x_attr as libc::c_int,
                            (*r_ptr).x_char as byte_hack as libc::c_int);
                }
                i += 1
            }
            /* All done */
            fprintf(fff, b"\n\n\n\n\x00" as *const u8 as *const libc::c_char);
            /* Close */
            my_fclose(fff);
            /* Message */
            msg_print(b"Dumped monster attr/chars.\x00" as *const u8 as
                          *const libc::c_char);
        } else if i == '3' as i32 {
            /* Dump object attr/chars */
            /* Prompt */
            prt(b"Command: Dump object attr/chars\x00" as *const u8 as
                    *const libc::c_char, 15 as libc::c_int, 0 as libc::c_int);
            /* Prompt */
            prt(b"File: \x00" as *const u8 as *const libc::c_char,
                17 as libc::c_int, 0 as libc::c_int);
            /* Default filename */
            strnfmt(tmp.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                    b"user-%s.prf\x00" as *const u8 as *const libc::c_char,
                    ANGBAND_SYS);
            /* Get a filename */
            if askfor_aux(tmp.as_mut_ptr(), 70 as libc::c_int) == 0 {
                continue ;
            }
            /* Build the filename */
            path_build(buf.as_mut_ptr(), 1024 as libc::c_int,
                       ANGBAND_DIR_USER, tmp.as_mut_ptr() as cptr);
            /* Append to the file */
            fff =
                my_fopen(buf.as_mut_ptr() as cptr,
                         b"a\x00" as *const u8 as *const libc::c_char);
            /* Failure */
            if fff.is_null() { continue ; }
            /* Start dumping */
            fprintf(fff, b"\n\n\x00" as *const u8 as *const libc::c_char);
            fprintf(fff,
                    b"# Object attr/char definitions\n\n\x00" as *const u8 as
                        *const libc::c_char);
            /* Dump objects */
            i = 0 as libc::c_int;
            while i < max_k_idx as libc::c_int {
                let mut k_ptr: *mut object_kind =
                    &mut *k_info.offset(i as isize) as *mut object_kind;
                /* Skip non-entries */
                if !((*k_ptr).name == 0) {
                    /* Dump a comment */
                    fprintf(fff,
                            b"# %s\n\x00" as *const u8 as *const libc::c_char,
                            k_name.offset((*k_ptr).name as isize));
                    /* Dump the object attr/char info */
                    fprintf(fff,
                            b"K:%d:0x%02X:0x%02X\n\n\x00" as *const u8 as
                                *const libc::c_char, i,
                            (*k_ptr).x_attr as libc::c_int,
                            (*k_ptr).x_char as byte_hack as libc::c_int);
                }
                i += 1
            }
            /* All done */
            fprintf(fff, b"\n\n\n\n\x00" as *const u8 as *const libc::c_char);
            /* Close */
            my_fclose(fff);
            /* Message */
            msg_print(b"Dumped object attr/chars.\x00" as *const u8 as
                          *const libc::c_char);
        } else if i == '4' as i32 {
            /* Dump feature attr/chars */
            /* Prompt */
            prt(b"Command: Dump feature attr/chars\x00" as *const u8 as
                    *const libc::c_char, 15 as libc::c_int, 0 as libc::c_int);
            /* Prompt */
            prt(b"File: \x00" as *const u8 as *const libc::c_char,
                17 as libc::c_int, 0 as libc::c_int);
            /* Default filename */
            strnfmt(tmp.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                    b"user-%s.prf\x00" as *const u8 as *const libc::c_char,
                    ANGBAND_SYS);
            /* Get a filename */
            if askfor_aux(tmp.as_mut_ptr(), 70 as libc::c_int) == 0 {
                continue ;
            }
            /* Build the filename */
            path_build(buf.as_mut_ptr(), 1024 as libc::c_int,
                       ANGBAND_DIR_USER, tmp.as_mut_ptr() as cptr);
            /* Append to the file */
            fff =
                my_fopen(buf.as_mut_ptr() as cptr,
                         b"a\x00" as *const u8 as *const libc::c_char);
            /* Failure */
            if fff.is_null() { continue ; }
            /* Start dumping */
            fprintf(fff, b"\n\n\x00" as *const u8 as *const libc::c_char);
            fprintf(fff,
                    b"# Feature attr/char definitions\n\n\x00" as *const u8 as
                        *const libc::c_char);
            /* Dump features */
            i = 0 as libc::c_int;
            while i < max_f_idx as libc::c_int {
                let mut f_ptr: *mut feature_type =
                    &mut *f_info.offset(i as isize) as *mut feature_type;
                /* Skip non-entries */
                if !((*f_ptr).name == 0) {
                    /* Dump a comment */
                    fprintf(fff,
                            b"# %s\n\x00" as *const u8 as *const libc::c_char,
                            f_name.offset((*f_ptr).name as isize));
                    /* Dump the feature attr/char info */
                    fprintf(fff,
                            b"F:%d:0x%02X:0x%02X\n\n\x00" as *const u8 as
                                *const libc::c_char, i,
                            (*f_ptr).x_attr as libc::c_int,
                            (*f_ptr).x_char as byte_hack as libc::c_int);
                }
                i += 1
            }
            /* All done */
            fprintf(fff, b"\n\n\n\n\x00" as *const u8 as *const libc::c_char);
            /* Close */
            my_fclose(fff);
            /* Message */
            msg_print(b"Dumped feature attr/chars.\x00" as *const u8 as
                          *const libc::c_char);
        } else if i == '6' as i32 {
            static mut r: libc::c_int = 0 as libc::c_int;
            /* Modify monster attr/chars */
            /* Prompt */
            prt(b"Command: Change monster attr/chars\x00" as *const u8 as
                    *const libc::c_char, 15 as libc::c_int, 0 as libc::c_int);
            loop 
                 /* Hack -- query until done */
                 {
                let mut r_ptr_0: *mut monster_race =
                    &mut *r_info.offset(r as isize) as *mut monster_race;
                let mut da: byte_hack = (*r_ptr_0).d_attr;
                let mut dc: libc::c_char = (*r_ptr_0).d_char;
                let mut ca: byte_hack = (*r_ptr_0).x_attr;
                let mut cc: libc::c_char = (*r_ptr_0).x_char;
                /* Label the object */
                Term_putstr(5 as libc::c_int, 17 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            format(b"Monster = %d, Name = %-40.40s\x00" as
                                       *const u8 as *const libc::c_char, r,
                                   r_name.offset((*r_ptr_0).name as isize)) as
                                cptr);
                /* Label the Default values */
                Term_putstr(10 as libc::c_int, 19 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            format(b"Default attr/char = %3u / %3u\x00" as
                                       *const u8 as *const libc::c_char,
                                   da as libc::c_int,
                                   dc as libc::c_int & 0xff as libc::c_int) as
                                cptr);
                Term_putstr(40 as libc::c_int, 19 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            b"<< ? >>\x00" as *const u8 as
                                *const libc::c_char);
                Term_putch(43 as libc::c_int, 19 as libc::c_int, da, dc);
                if use_bigtile != 0 {
                    if da as libc::c_int & 0x80 as libc::c_int != 0 {
                        Term_putch(44 as libc::c_int, 19 as libc::c_int,
                                   255 as libc::c_int as byte_hack,
                                   255 as libc::c_int as libc::c_char);
                    } else {
                        Term_putch(44 as libc::c_int, 19 as libc::c_int,
                                   0 as libc::c_int as byte_hack,
                                   ' ' as i32 as libc::c_char);
                    }
                }
                /* Label the Current values */
                Term_putstr(10 as libc::c_int, 20 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            format(b"Current attr/char = %3u / %3u\x00" as
                                       *const u8 as *const libc::c_char,
                                   ca as libc::c_int,
                                   cc as libc::c_int & 0xff as libc::c_int) as
                                cptr);
                Term_putstr(40 as libc::c_int, 20 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            b"<< ? >>\x00" as *const u8 as
                                *const libc::c_char);
                Term_putch(43 as libc::c_int, 20 as libc::c_int, ca, cc);
                if use_bigtile != 0 {
                    if ca as libc::c_int & 0x80 as libc::c_int != 0 {
                        Term_putch(44 as libc::c_int, 20 as libc::c_int,
                                   255 as libc::c_int as byte_hack,
                                   255 as libc::c_int as libc::c_char);
                    } else {
                        Term_putch(44 as libc::c_int, 20 as libc::c_int,
                                   0 as libc::c_int as byte_hack,
                                   ' ' as i32 as libc::c_char);
                    }
                }
                /* Prompt */
                Term_putstr(0 as libc::c_int, 22 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            b"Command (n/N/a/A/c/C): \x00" as *const u8 as
                                *const libc::c_char);
                /* Get a command */
                i = inkey() as libc::c_int;
                /* All done */
                if i == '\u{1b}' as i32 { break ; }
                /* Analyze */
                if i == 'n' as i32 {
                    r =
                        (r + max_r_idx as libc::c_int + 1 as libc::c_int) %
                            max_r_idx as libc::c_int
                }
                if i == 'N' as i32 {
                    r =
                        (r + max_r_idx as libc::c_int - 1 as libc::c_int) %
                            max_r_idx as libc::c_int
                }
                if i == 'a' as i32 {
                    (*r_ptr_0).x_attr =
                        (ca as libc::c_int + 1 as libc::c_int) as byte_hack
                }
                if i == 'A' as i32 {
                    (*r_ptr_0).x_attr =
                        (ca as libc::c_int - 1 as libc::c_int) as byte_hack
                }
                if i == 'c' as i32 {
                    (*r_ptr_0).x_char =
                        (cc as libc::c_int + 1 as libc::c_int) as byte_hack as
                            libc::c_char
                }
                if i == 'C' as i32 {
                    (*r_ptr_0).x_char =
                        (cc as libc::c_int - 1 as libc::c_int) as byte_hack as
                            libc::c_char
                }
            }
        } else if i == '7' as i32 {
            static mut k: libc::c_int = 0 as libc::c_int;
            /* Modify object attr/chars */
            /* Prompt */
            prt(b"Command: Change object attr/chars\x00" as *const u8 as
                    *const libc::c_char, 15 as libc::c_int, 0 as libc::c_int);
            loop 
                 /* Hack -- query until done */
                 {
                let mut k_ptr_0: *mut object_kind =
                    &mut *k_info.offset(k as isize) as *mut object_kind;
                let mut da_0: byte_hack = (*k_ptr_0).d_attr;
                let mut dc_0: libc::c_char =
                    (*k_ptr_0).d_char as byte_hack as libc::c_char;
                let mut ca_0: byte_hack = (*k_ptr_0).x_attr;
                let mut cc_0: libc::c_char =
                    (*k_ptr_0).x_char as byte_hack as libc::c_char;
                /* Label the object */
                Term_putstr(5 as libc::c_int, 17 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            format(b"Object = %d, Name = %-40.40s\x00" as
                                       *const u8 as *const libc::c_char, k,
                                   k_name.offset((*k_ptr_0).name as isize)) as
                                cptr);
                /* Label the Default values */
                Term_putstr(10 as libc::c_int, 19 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            format(b"Default attr/char = %3u / %3u\x00" as
                                       *const u8 as *const libc::c_char,
                                   da_0 as libc::c_int,
                                   dc_0 as libc::c_int & 0xff as libc::c_int)
                                as cptr);
                Term_putstr(40 as libc::c_int, 19 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            b"<< ? >>\x00" as *const u8 as
                                *const libc::c_char);
                Term_putch(43 as libc::c_int, 19 as libc::c_int, da_0, dc_0);
                if use_bigtile != 0 {
                    if da_0 as libc::c_int & 0x80 as libc::c_int != 0 {
                        Term_putch(44 as libc::c_int, 19 as libc::c_int,
                                   255 as libc::c_int as byte_hack,
                                   255 as libc::c_int as libc::c_char);
                    } else {
                        Term_putch(44 as libc::c_int, 19 as libc::c_int,
                                   0 as libc::c_int as byte_hack,
                                   ' ' as i32 as libc::c_char);
                    }
                }
                /* Label the Current values */
                Term_putstr(10 as libc::c_int, 20 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            format(b"Current attr/char = %3u / %3u\x00" as
                                       *const u8 as *const libc::c_char,
                                   ca_0 as libc::c_int,
                                   cc_0 as libc::c_int & 0xff as libc::c_int)
                                as cptr);
                Term_putstr(40 as libc::c_int, 20 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            b"<< ? >>\x00" as *const u8 as
                                *const libc::c_char);
                Term_putch(43 as libc::c_int, 20 as libc::c_int, ca_0, cc_0);
                if use_bigtile != 0 {
                    if ca_0 as libc::c_int & 0x80 as libc::c_int != 0 {
                        Term_putch(44 as libc::c_int, 20 as libc::c_int,
                                   255 as libc::c_int as byte_hack,
                                   255 as libc::c_int as libc::c_char);
                    } else {
                        Term_putch(44 as libc::c_int, 20 as libc::c_int,
                                   0 as libc::c_int as byte_hack,
                                   ' ' as i32 as libc::c_char);
                    }
                }
                /* Prompt */
                Term_putstr(0 as libc::c_int, 22 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            b"Command (n/N/a/A/c/C): \x00" as *const u8 as
                                *const libc::c_char);
                /* Get a command */
                i = inkey() as libc::c_int;
                /* All done */
                if i == '\u{1b}' as i32 { break ; }
                /* Analyze */
                if i == 'n' as i32 {
                    k =
                        (k + max_k_idx as libc::c_int + 1 as libc::c_int) %
                            max_k_idx as libc::c_int
                }
                if i == 'N' as i32 {
                    k =
                        (k + max_k_idx as libc::c_int - 1 as libc::c_int) %
                            max_k_idx as libc::c_int
                }
                if i == 'a' as i32 {
                    (*k_info.offset(k as isize)).x_attr =
                        (ca_0 as libc::c_int + 1 as libc::c_int) as byte_hack
                }
                if i == 'A' as i32 {
                    (*k_info.offset(k as isize)).x_attr =
                        (ca_0 as libc::c_int - 1 as libc::c_int) as byte_hack
                }
                if i == 'c' as i32 {
                    (*k_info.offset(k as isize)).x_char =
                        (cc_0 as libc::c_int + 1 as libc::c_int) as byte_hack
                            as libc::c_char
                }
                if i == 'C' as i32 {
                    (*k_info.offset(k as isize)).x_char =
                        (cc_0 as libc::c_int - 1 as libc::c_int) as byte_hack
                            as libc::c_char
                }
            }
        } else if i == '8' as i32 {
            static mut f: libc::c_int = 0 as libc::c_int;
            /* Modify feature attr/chars */
            /* Prompt */
            prt(b"Command: Change feature attr/chars\x00" as *const u8 as
                    *const libc::c_char, 15 as libc::c_int, 0 as libc::c_int);
            loop 
                 /* Hack -- query until done */
                 {
                let mut f_ptr_0: *mut feature_type =
                    &mut *f_info.offset(f as isize) as *mut feature_type;
                let mut da_1: byte_hack = (*f_ptr_0).d_attr;
                let mut dc_1: libc::c_char =
                    (*f_ptr_0).d_char as byte_hack as libc::c_char;
                let mut ca_1: byte_hack = (*f_ptr_0).x_attr;
                let mut cc_1: libc::c_char =
                    (*f_ptr_0).x_char as byte_hack as libc::c_char;
                /* Label the object */
                Term_putstr(5 as libc::c_int, 17 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            format(b"Terrain = %d, Name = %-40.40s\x00" as
                                       *const u8 as *const libc::c_char, f,
                                   f_name.offset((*f_ptr_0).name as isize)) as
                                cptr);
                /* Label the Default values */
                Term_putstr(10 as libc::c_int, 19 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            format(b"Default attr/char = %3u / %3u\x00" as
                                       *const u8 as *const libc::c_char,
                                   da_1 as libc::c_int,
                                   dc_1 as libc::c_int & 0xff as libc::c_int)
                                as cptr);
                Term_putstr(40 as libc::c_int, 19 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            b"<< ? >>\x00" as *const u8 as
                                *const libc::c_char);
                Term_putch(43 as libc::c_int, 19 as libc::c_int, da_1, dc_1);
                if use_bigtile != 0 {
                    if da_1 as libc::c_int & 0x80 as libc::c_int != 0 {
                        Term_putch(44 as libc::c_int, 19 as libc::c_int,
                                   255 as libc::c_int as byte_hack,
                                   255 as libc::c_int as libc::c_char);
                    } else {
                        Term_putch(44 as libc::c_int, 19 as libc::c_int,
                                   0 as libc::c_int as byte_hack,
                                   ' ' as i32 as libc::c_char);
                    }
                }
                /* Label the Current values */
                Term_putstr(10 as libc::c_int, 20 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            format(b"Current attr/char = %3u / %3u\x00" as
                                       *const u8 as *const libc::c_char,
                                   ca_1 as libc::c_int,
                                   cc_1 as libc::c_int & 0xff as libc::c_int)
                                as cptr);
                Term_putstr(40 as libc::c_int, 20 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            b"<< ? >>\x00" as *const u8 as
                                *const libc::c_char);
                Term_putch(43 as libc::c_int, 20 as libc::c_int, ca_1, cc_1);
                if use_bigtile != 0 {
                    if ca_1 as libc::c_int & 0x80 as libc::c_int != 0 {
                        Term_putch(44 as libc::c_int, 20 as libc::c_int,
                                   255 as libc::c_int as byte_hack,
                                   255 as libc::c_int as libc::c_char);
                    } else {
                        Term_putch(44 as libc::c_int, 20 as libc::c_int,
                                   0 as libc::c_int as byte_hack,
                                   ' ' as i32 as libc::c_char);
                    }
                }
                /* Prompt */
                Term_putstr(0 as libc::c_int, 22 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            b"Command (n/N/a/A/c/C/d): \x00" as *const u8 as
                                *const libc::c_char);
                /* Get a command */
                i = inkey() as libc::c_int;
                /* All done */
                if i == '\u{1b}' as i32 { break ; }
                /* Analyze */
                if i == 'n' as i32 {
                    f =
                        (f + max_f_idx as libc::c_int + 1 as libc::c_int) %
                            max_f_idx as libc::c_int
                }
                if i == 'N' as i32 {
                    f =
                        (f + max_f_idx as libc::c_int - 1 as libc::c_int) %
                            max_f_idx as libc::c_int
                }
                if i == 'a' as i32 {
                    (*f_info.offset(f as isize)).x_attr =
                        (ca_1 as libc::c_int + 1 as libc::c_int) as byte_hack
                }
                if i == 'A' as i32 {
                    (*f_info.offset(f as isize)).x_attr =
                        (ca_1 as libc::c_int - 1 as libc::c_int) as byte_hack
                }
                if i == 'c' as i32 {
                    (*f_info.offset(f as isize)).x_char =
                        (cc_1 as libc::c_int + 1 as libc::c_int) as byte_hack
                            as libc::c_char
                }
                if i == 'C' as i32 {
                    (*f_info.offset(f as isize)).x_char =
                        (cc_1 as libc::c_int - 1 as libc::c_int) as byte_hack
                            as libc::c_char
                }
                if i == 'd' as i32 {
                    (*f_info.offset(f as isize)).x_char = (*f_ptr_0).d_char;
                    (*f_info.offset(f as isize)).x_attr = (*f_ptr_0).d_attr
                }
            }
        } else if i == '0' as i32 {
            /* Reset visuals */
            /* Reset */
            reset_visuals();
            /* Message */
            msg_print(b"Visual attr/char tables reset.\x00" as *const u8 as
                          *const libc::c_char);
        } else {
            /* Unknown option */
            bell();
        }
        /* Flush messages */
        msg_print(0 as cptr);
    }
    /* Restore the screen */
    Term_load();
    /* Leave "icky" mode */
    character_icky = 0 as libc::c_int as bool_;
}
/*
 * Interact with "colors"
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_colors() {
    let mut i: libc::c_int = 0;
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut tmp: [libc::c_char; 160] = [0; 160];
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* File type is "TEXT" */
    /* Enter "icky" mode */
    character_icky = 1 as libc::c_int as bool_;
    /* Save the screen */
    Term_save();
    loop 
         /* Interact until done */
         /* Clear screen */
         {
        Term_clear();
        /* Ask for a choice */
        prt(b"Interact with Colors\x00" as *const u8 as *const libc::c_char,
            2 as libc::c_int, 0 as libc::c_int);
        /* Give some choices */
        prt(b"(1) Load a user pref file\x00" as *const u8 as
                *const libc::c_char, 4 as libc::c_int, 5 as libc::c_int);
        prt(b"(2) Dump colors\x00" as *const u8 as *const libc::c_char,
            5 as libc::c_int, 5 as libc::c_int);
        prt(b"(3) Modify colors\x00" as *const u8 as *const libc::c_char,
            6 as libc::c_int, 5 as libc::c_int);
        /* Prompt */
        prt(b"Command: \x00" as *const u8 as *const libc::c_char,
            8 as libc::c_int, 0 as libc::c_int);
        /* Prompt */
        i = inkey() as libc::c_int;
        /* Done */
        if i == '\u{1b}' as i32 { break ; }
        /* Load a 'pref' file */
        if i == '1' as i32 {
            /* Prompt */
            prt(b"Command: Load a user pref file\x00" as *const u8 as
                    *const libc::c_char, 8 as libc::c_int, 0 as libc::c_int);
            /* Prompt */
            prt(b"File: \x00" as *const u8 as *const libc::c_char,
                10 as libc::c_int, 0 as libc::c_int);
            /* Default file */
            strnfmt(tmp.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                    b"user-%s.prf\x00" as *const u8 as *const libc::c_char,
                    ANGBAND_SYS);
            /* Query */
            if askfor_aux(tmp.as_mut_ptr(), 70 as libc::c_int) == 0 {
                continue ;
            }
            /* Process the given filename */
            process_pref_file(tmp.as_mut_ptr() as cptr);
            /* Mega-Hack -- react to changes */
            Term_xtra(10 as libc::c_int, 0 as libc::c_int);
            /* Mega-Hack -- redraw */
            Term_redraw();
        } else if i == '2' as i32 {
            /* Dump colors */
            /* Prompt */
            prt(b"Command: Dump colors\x00" as *const u8 as
                    *const libc::c_char, 8 as libc::c_int, 0 as libc::c_int);
            /* Prompt */
            prt(b"File: \x00" as *const u8 as *const libc::c_char,
                10 as libc::c_int, 0 as libc::c_int);
            /* Default filename */
            strnfmt(tmp.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                    b"user-%s.prf\x00" as *const u8 as *const libc::c_char,
                    ANGBAND_SYS);
            /* Get a filename */
            if askfor_aux(tmp.as_mut_ptr(), 70 as libc::c_int) == 0 {
                continue ;
            }
            /* Build the filename */
            path_build(buf.as_mut_ptr(), 1024 as libc::c_int,
                       ANGBAND_DIR_USER, tmp.as_mut_ptr() as cptr);
            /* Append to the file */
            fff =
                my_fopen(buf.as_mut_ptr() as cptr,
                         b"a\x00" as *const u8 as *const libc::c_char);
            /* Failure */
            if fff.is_null() { continue ; }
            /* Start dumping */
            fprintf(fff, b"\n\n\x00" as *const u8 as *const libc::c_char);
            fprintf(fff,
                    b"# Color redefinitions\n\n\x00" as *const u8 as
                        *const libc::c_char);
            /* Dump colors */
            i = 0 as libc::c_int;
            while i < 256 as libc::c_int {
                let mut kv: libc::c_int =
                    angband_color_table[i as usize][0 as libc::c_int as usize]
                        as libc::c_int;
                let mut rv: libc::c_int =
                    angband_color_table[i as usize][1 as libc::c_int as usize]
                        as libc::c_int;
                let mut gv: libc::c_int =
                    angband_color_table[i as usize][2 as libc::c_int as usize]
                        as libc::c_int;
                let mut bv: libc::c_int =
                    angband_color_table[i as usize][3 as libc::c_int as usize]
                        as libc::c_int;
                let mut name: cptr =
                    b"unknown\x00" as *const u8 as *const libc::c_char;
                /* Skip non-entries */
                if !(kv == 0 && rv == 0 && gv == 0 && bv == 0) {
                    /* Extract the color name */
                    if i < 16 as libc::c_int {
                        name = color_names[i as usize]
                    }
                    /* Dump a comment */
                    fprintf(fff,
                            b"# Color \'%s\'\n\x00" as *const u8 as
                                *const libc::c_char, name);
                    /* Dump the monster attr/char info */
                    fprintf(fff,
                            b"V:%d:0x%02X:0x%02X:0x%02X:0x%02X\n\n\x00" as
                                *const u8 as *const libc::c_char, i, kv, rv,
                            gv, bv);
                }
                i += 1
            }
            /* All done */
            fprintf(fff, b"\n\n\n\n\x00" as *const u8 as *const libc::c_char);
            /* Close */
            my_fclose(fff);
            /* Message */
            msg_print(b"Dumped color redefinitions.\x00" as *const u8 as
                          *const libc::c_char);
        } else if i == '3' as i32 {
            static mut a: byte_hack = 0 as libc::c_int as byte_hack;
            /* Edit colors */
            /* Prompt */
            prt(b"Command: Modify colors\x00" as *const u8 as
                    *const libc::c_char, 8 as libc::c_int, 0 as libc::c_int);
            loop 
                 /* Hack -- query until done */
                 {
                let mut name_0: cptr = 0 as *const libc::c_char;
                /* Clear */
                clear_from(10 as libc::c_int);
                /* Exhibit the normal colors */
                i = 0 as libc::c_int;
                while i < 16 as libc::c_int {
                    /* Exhibit this color */
                    Term_putstr(i * 4 as libc::c_int, 20 as libc::c_int,
                                -(1 as libc::c_int), a,
                                b"###\x00" as *const u8 as
                                    *const libc::c_char);
                    /* Exhibit all colors */
                    Term_putstr(i * 4 as libc::c_int, 22 as libc::c_int,
                                -(1 as libc::c_int), i as byte_hack,
                                format(b"%3d\x00" as *const u8 as
                                           *const libc::c_char, i) as cptr);
                    i += 1
                }
                /* Describe the color */
                name_0 =
                    if (a as libc::c_int) < 16 as libc::c_int {
                        color_names[a as usize]
                    } else {
                        b"undefined\x00" as *const u8 as *const libc::c_char
                    };
                /* Describe the color */
                Term_putstr(5 as libc::c_int, 10 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            format(b"Color = %d, Name = %s\x00" as *const u8
                                       as *const libc::c_char,
                                   a as libc::c_int, name_0) as cptr);
                /* Label the Current values */
                Term_putstr(5 as libc::c_int, 12 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            format(b"K = 0x%02x / R,G,B = 0x%02x,0x%02x,0x%02x\x00"
                                       as *const u8 as *const libc::c_char,
                                   angband_color_table[a as
                                                           usize][0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]
                                       as libc::c_int,
                                   angband_color_table[a as
                                                           usize][1 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]
                                       as libc::c_int,
                                   angband_color_table[a as
                                                           usize][2 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]
                                       as libc::c_int,
                                   angband_color_table[a as
                                                           usize][3 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]
                                       as libc::c_int) as cptr);
                /* Prompt */
                Term_putstr(0 as libc::c_int, 14 as libc::c_int,
                            -(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            b"Command (n/N/k/K/r/R/g/G/b/B): \x00" as
                                *const u8 as *const libc::c_char);
                /* Get a command */
                i = inkey() as libc::c_int;
                /* All done */
                if i == '\u{1b}' as i32 { break ; }
                /* Analyze */
                if i == 'n' as i32 {
                    a = (a as libc::c_int + 1 as libc::c_int) as byte_hack
                }
                if i == 'N' as i32 {
                    a = (a as libc::c_int - 1 as libc::c_int) as byte_hack
                }
                if i == 'k' as i32 {
                    angband_color_table[a as usize][0 as libc::c_int as usize]
                        =
                        (angband_color_table[a as
                                                 usize][0 as libc::c_int as
                                                            usize] as
                             libc::c_int + 1 as libc::c_int) as byte_hack
                }
                if i == 'K' as i32 {
                    angband_color_table[a as usize][0 as libc::c_int as usize]
                        =
                        (angband_color_table[a as
                                                 usize][0 as libc::c_int as
                                                            usize] as
                             libc::c_int - 1 as libc::c_int) as byte_hack
                }
                if i == 'r' as i32 {
                    angband_color_table[a as usize][1 as libc::c_int as usize]
                        =
                        (angband_color_table[a as
                                                 usize][1 as libc::c_int as
                                                            usize] as
                             libc::c_int + 1 as libc::c_int) as byte_hack
                }
                if i == 'R' as i32 {
                    angband_color_table[a as usize][1 as libc::c_int as usize]
                        =
                        (angband_color_table[a as
                                                 usize][1 as libc::c_int as
                                                            usize] as
                             libc::c_int - 1 as libc::c_int) as byte_hack
                }
                if i == 'g' as i32 {
                    angband_color_table[a as usize][2 as libc::c_int as usize]
                        =
                        (angband_color_table[a as
                                                 usize][2 as libc::c_int as
                                                            usize] as
                             libc::c_int + 1 as libc::c_int) as byte_hack
                }
                if i == 'G' as i32 {
                    angband_color_table[a as usize][2 as libc::c_int as usize]
                        =
                        (angband_color_table[a as
                                                 usize][2 as libc::c_int as
                                                            usize] as
                             libc::c_int - 1 as libc::c_int) as byte_hack
                }
                if i == 'b' as i32 {
                    angband_color_table[a as usize][3 as libc::c_int as usize]
                        =
                        (angband_color_table[a as
                                                 usize][3 as libc::c_int as
                                                            usize] as
                             libc::c_int + 1 as libc::c_int) as byte_hack
                }
                if i == 'B' as i32 {
                    angband_color_table[a as usize][3 as libc::c_int as usize]
                        =
                        (angband_color_table[a as
                                                 usize][3 as libc::c_int as
                                                            usize] as
                             libc::c_int - 1 as libc::c_int) as byte_hack
                }
                /* Hack -- react to changes */
                Term_xtra(10 as libc::c_int, 0 as libc::c_int);
                /* Hack -- redraw */
                Term_redraw();
            }
        } else {
            /* Unknown option */
            bell();
        }
        /* Flush messages */
        msg_print(0 as cptr);
    }
    /* Restore the screen */
    Term_load();
    /* Leave "icky" mode */
    character_icky = 0 as libc::c_int as bool_;
}
/*
 * Take notes.  There are two ways this can happen, either in the message
 * recall or a file.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_note() {
    let mut buf: [libc::c_char; 80] = [0; 80];
    /* Default */
    strcpy(buf.as_mut_ptr(), b"\x00" as *const u8 as *const libc::c_char);
    if get_string(b"Note: \x00" as *const u8 as *const libc::c_char,
                  buf.as_mut_ptr(), 60 as libc::c_int) == 0 {
        return
    }
    /* Ignore empty notes */
    if buf[0 as libc::c_int as usize] == 0 ||
           buf[0 as libc::c_int as usize] as libc::c_int == ' ' as i32 {
        return
    }
    if take_notes != 0 {
        /* Add note to file */
        add_note(buf.as_mut_ptr(), ' ' as i32 as libc::c_char);
    } else {
        /* Add note to message recall */
        msg_format(b"Note: %s\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
    };
}
/*
 * Mention the current version
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_version() {
    let mut author: cptr = 0 as *const libc::c_char;
    let mut email: cptr = 0 as *const libc::c_char;
    call_lua(b"get_module_info\x00" as *const u8 as *const libc::c_char,
             b"(s,d)\x00" as *const u8 as *const libc::c_char,
             b"s\x00" as *const u8 as *const libc::c_char,
             b"author\x00" as *const u8 as *const libc::c_char,
             1 as libc::c_int, &mut author as *mut cptr);
    call_lua(b"get_module_info\x00" as *const u8 as *const libc::c_char,
             b"(s,d)\x00" as *const u8 as *const libc::c_char,
             b"s\x00" as *const u8 as *const libc::c_char,
             b"author\x00" as *const u8 as *const libc::c_char,
             2 as libc::c_int, &mut email as *mut cptr);
    /* Silly message */
    msg_format(b"You are playing %s made by %s (%s).\x00" as *const u8 as
                   *const libc::c_char, get_version_string(), author, email);
    call_lua(b"patchs_display\x00" as *const u8 as *const libc::c_char,
             b"()\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char);
}
/*
 * Array of feeling strings
 */
static mut do_cmd_feeling_text: [cptr; 11] =
    [b"Looks like any other level.\x00" as *const u8 as *const libc::c_char,
     b"You feel there is something special about this level.\x00" as *const u8
         as *const libc::c_char,
     b"You have a superb feeling about this level.\x00" as *const u8 as
         *const libc::c_char,
     b"You have an excellent feeling...\x00" as *const u8 as
         *const libc::c_char,
     b"You have a very good feeling...\x00" as *const u8 as
         *const libc::c_char,
     b"You have a good feeling...\x00" as *const u8 as *const libc::c_char,
     b"You feel strangely lucky...\x00" as *const u8 as *const libc::c_char,
     b"You feel your luck is turning...\x00" as *const u8 as
         *const libc::c_char,
     b"You like the look of this place...\x00" as *const u8 as
         *const libc::c_char,
     b"This level can\'t be all bad...\x00" as *const u8 as
         *const libc::c_char,
     b"What a boring place...\x00" as *const u8 as *const libc::c_char];
/*
 * Note that "feeling" is set to zero unless some time has passed.
 * Note that this is done when the level is GENERATED, not entered.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_feeling() {
    /* Verify the feeling */
    if (feeling as libc::c_int) < 0 as libc::c_int {
        feeling = 0 as libc::c_int as s16b
    }
    if feeling as libc::c_int > 10 as libc::c_int {
        feeling = 10 as libc::c_int as s16b
    }
    /* Feeling of the fate */
    if fate_flag as libc::c_int != 0 &&
           dungeon_flags2 as libc::c_long & 0x40 as libc::c_long == 0 &&
           (*p_ptr).inside_quest == 0 {
        msg_print(b"You feel that you will meet your fate here.\x00" as
                      *const u8 as *const libc::c_char);
    }
    /* Hooked feelings ? */
    if process_hooks(4 as libc::c_int,
                     b"(d)\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                     is_quest(dun_level as libc::c_int)) != 0 {
        return
    }
    /* No useful feeling in special levels */
    if dungeon_flags2 as libc::c_long & 0x100 as libc::c_long != 0 {
        let mut buf: [libc::c_char; 1024] = [0; 1024];
        if get_dungeon_save(buf.as_mut_ptr()) as libc::c_int != 0 ||
               generate_special_feeling as libc::c_int != 0 ||
               dungeon_flags2 as libc::c_long & 0x10000 as libc::c_long != 0 {
            if get_level_desc(buf.as_mut_ptr()) == 0 {
                msg_print(b"Someone forgot to describe this level!\x00" as
                              *const u8 as *const libc::c_char);
            } else { msg_print(buf.as_mut_ptr() as cptr); }
            return
        }
    }
    /* No useful feeling in quests */
    if (*p_ptr).inside_quest != 0 {
        msg_print(b"Looks like a typical quest level.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Display feelings in the dungeon, nothing on the surface */
    if dun_level != 0 {
        /* This could be simplified with a correct p_ptr->town_num */
        let mut i: libc::c_int = 0;
        let mut town_level: libc::c_int = 0 as libc::c_int;
        let mut d_ptr: *mut dungeon_info_type =
            &mut *d_info.offset(dungeon_type as isize) as
                *mut dungeon_info_type;
        /* Is it a town level ? */
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            if (*d_ptr).t_level[i as usize] as libc::c_int ==
                   dun_level as libc::c_int {
                town_level = (*d_ptr).t_idx[i as usize] as libc::c_int
            }
            i += 1
        }
        if town_level != 0 {
            msg_print(b"You hear the sound of a market.\x00" as *const u8 as
                          *const libc::c_char);
        } else { msg_print(do_cmd_feeling_text[feeling as usize]); }
    };
}
/*
 * Encode the screen colors
 */
static mut hack: [libc::c_char; 17] =
    unsafe {
        *::std::mem::transmute::<&[u8; 17],
                                 &mut [libc::c_char; 17]>(b"dwsorgbuDWvyRGBU\x00")
    };
/*
 * Hack -- load a screen dump from a file
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_load_screen() {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut a: byte_hack = 0 as libc::c_int as byte_hack;
    let mut c: libc::c_char = ' ' as i32 as libc::c_char;
    let mut okay: bool_ = 1 as libc::c_int as bool_;
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_USER,
               b"dump.txt\x00" as *const u8 as *const libc::c_char);
    /* Append to the file */
    fff =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Oops */
    if fff.is_null() { return }
    /* Retrieve the current screen size */
    Term_get_size(&mut wid, &mut hgt);
    /* Enter "icky" mode */
    character_icky = 1 as libc::c_int as bool_;
    /* Save the screen */
    Term_save();
    /* Clear the screen */
    Term_clear();
    /* Load the screen */
    y = 0 as libc::c_int;
    while okay != 0 {
        /* Get a line of data */
        if my_fgets(fff, buf.as_mut_ptr(), 1024 as libc::c_int as huge_hack)
               != 0 {
            okay = 0 as libc::c_int as bool_
        }
        /* Stop on blank line */
        if buf[0 as libc::c_int as usize] == 0 { break ; }
        /* Ignore off screen lines */
        if !(y >= hgt) {
            /* Get width */
            len = strlen(buf.as_mut_ptr()) as libc::c_int;
            /* Truncate if it's longer than current screen width */
            if len > wid { len = wid }
            /* Show each row */
            x = 0 as libc::c_int;
            while x < len {
                /* Put the attr/char */
                Term_draw(x, y, 1 as libc::c_int as byte_hack,
                          buf[x as usize]);
                x += 1
            }
        }
        y += 1
    }
    /* Dump the screen */
    y = 0 as libc::c_int;
    while okay != 0 {
        /* Get a line of data */
        if my_fgets(fff, buf.as_mut_ptr(), 1024 as libc::c_int as huge_hack)
               != 0 {
            okay = 0 as libc::c_int as bool_
        }
        /* Stop on blank line */
        if buf[0 as libc::c_int as usize] == 0 { break ; }
        /* Ignore off screen lines */
        if !(y >= hgt) {
            /* Get width */
            len = strlen(buf.as_mut_ptr()) as libc::c_int;
            /* Truncate if it's longer than current screen width */
            if len > wid { len = wid }
            /* Dump each row */
            x = 0 as libc::c_int;
            while x < len {
                /* Get the attr/char */
                Term_what(x, y, &mut a, &mut c);
                /* Look up the attr */
                i = 0 as libc::c_int;
                while i < 16 as libc::c_int {
                    /* Use attr matches */
                    if hack[i as usize] as libc::c_int ==
                           buf[x as usize] as libc::c_int {
                        a = i as byte_hack
                    }
                    i += 1
                }
                /* Put the attr/char */
                Term_draw(x, y, a, c);
                x += 1
            }
        }
        y += 1
    }
    /* Close it */
    my_fclose(fff);
    /* Message */
    msg_print(b"Screen dump loaded.\x00" as *const u8 as *const libc::c_char);
    msg_print(0 as cptr);
    /* Restore the screen */
    Term_load();
    /* Leave "icky" mode */
    character_icky = 0 as libc::c_int as bool_;
}
/*
 * Redefinable "save_screen" action
 */
#[no_mangle]
pub static mut screendump_aux: Option<unsafe extern "C" fn() -> ()> = None;
/*
 * Hack -- save a screen dump to a file
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_save_screen() {
    /* Do we use a special screendump function ? */
    if screendump_aux.is_some() {
        /* Dump the screen to a graphics file */
        Some(screendump_aux.expect("non-null function pointer")).expect("non-null function pointer")();
    } else {
        /* Dump the screen as text */
        let mut y: libc::c_int = 0;
        let mut x: libc::c_int = 0;
        let mut wid: libc::c_int = 0;
        let mut hgt: libc::c_int = 0;
        let mut a: byte_hack = 0 as libc::c_int as byte_hack;
        let mut c: libc::c_char = ' ' as i32 as libc::c_char;
        let mut fff: *mut FILE = 0 as *mut FILE;
        let mut buf: [libc::c_char; 1024] = [0; 1024];
        /* Build the filename */
        path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_USER,
                   b"dump.txt\x00" as *const u8 as *const libc::c_char);
        /* File type is "TEXT" */
        /* Append to the file */
        fff =
            my_fopen(buf.as_mut_ptr() as cptr,
                     b"w\x00" as *const u8 as *const libc::c_char);
        if fff.is_null() { return }
        Term_get_size(&mut wid, &mut hgt);
        character_icky = 1 as libc::c_int as bool_;
        Term_save();
        y = 0 as libc::c_int;
        while y < hgt {
            /* Oops */
            /* Retrieve the current screen size */
            /* Enter "icky" mode */
            /* Save the screen */
            /* Dump the screen */
            /* Dump each row */
            x = 0 as libc::c_int;
            while x < wid {
                /* Get the attr/char */
                Term_what(x, y, &mut a, &mut c);
                /* Dump it */
                buf[x as usize] = c;
                x += 1
            }
            /* Terminate */
            buf[x as usize] = '\u{0}' as i32 as libc::c_char;
            /* End the row */
            fprintf(fff, b"%s\n\x00" as *const u8 as *const libc::c_char,
                    buf.as_mut_ptr());
            y += 1
        }
        fprintf(fff, b"\n\x00" as *const u8 as *const libc::c_char);
        y = 0 as libc::c_int;
        while y < hgt {
            /* Skip a line */
            /* Dump the screen */
            /* Dump each row */
            x = 0 as libc::c_int;
            while x < wid {
                /* Get the attr/char */
                Term_what(x, y, &mut a, &mut c);
                /* Dump it */
                buf[x as usize] =
                    hack[(a as libc::c_int & 0xf as libc::c_int) as usize];
                x += 1
            }
            /* Terminate */
            buf[x as usize] = '\u{0}' as i32 as libc::c_char;
            /* End the row */
            fprintf(fff, b"%s\n\x00" as *const u8 as *const libc::c_char,
                    buf.as_mut_ptr());
            y += 1
        }
        fprintf(fff, b"\n\x00" as *const u8 as *const libc::c_char);
        my_fclose(fff);
        msg_print(b"Screen dump saved.\x00" as *const u8 as
                      *const libc::c_char);
        msg_print(0 as cptr);
        Term_load();
        character_icky = 0 as libc::c_int as bool_
    };
}
/* Skip a line */
/* Close it */
/* Message */
/* Restore the screen */
/* Leave "icky" mode */
/*
 * Check the status of "artifacts"
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_knowledge_artifacts() {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut file_name: [libc::c_char; 1024] = [0; 1024];
    let mut base_name: [libc::c_char; 80] = [0; 80];
    let mut okay: *mut bool_ = 0 as *mut bool_;
    let mut okayk: *mut bool_ = 0 as *mut bool_;
    /* Allocate the "okay" array */
    okay =
        memset(ralloc((max_a_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_a_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                    as libc::c_ulong)) as
            *mut bool_;
    okayk =
        memset(ralloc((max_k_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_k_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                    as libc::c_ulong)) as
            *mut bool_;
    /* Temporary file */
    if path_temp(file_name.as_mut_ptr(), 1024 as libc::c_int) != 0 { return }
    /* Open a new file */
    fff =
        my_fopen(file_name.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    /* Scan the artifacts */
    k = 0 as libc::c_int;
    while k < max_a_idx as libc::c_int {
        let mut a_ptr: *mut artifact_type =
            &mut *a_info.offset(k as isize) as *mut artifact_type;
        /* Default */
        *okay.offset(k as isize) = 0 as libc::c_int as bool_;
        /* Skip "empty" artifacts */
        if !((*a_ptr).name == 0) {
            /* Skip "uncreated" artifacts */
            if !((*a_ptr).cur_num == 0) {
                /* Assume okay */
                *okay.offset(k as isize) = 1 as libc::c_int as bool_
            }
        }
        k += 1
    }
    k = 0 as libc::c_int;
    while k < max_k_idx as libc::c_int {
        let mut k_ptr: *mut object_kind =
            &mut *k_info.offset(k as isize) as *mut object_kind;
        /* Default */
        *okayk.offset(k as isize) = 0 as libc::c_int as bool_;
        /* Skip "empty" artifacts */
        if !((*k_ptr).flags3 as libc::c_long & 0x8000 as libc::c_long == 0) {
            /* Skip "uncreated" artifacts */
            if !((*k_ptr).artifact == 0) {
                /* Assume okay */
                *okayk.offset(k as isize) = 1 as libc::c_int as bool_
            }
        }
        k += 1
    }
    /* Check the dungeon */
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            let mut c_ptr: *mut cave_type =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type;
            let mut this_o_idx: s16b = 0;
            let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
            /* Scan all objects in the grid */
            this_o_idx = (*c_ptr).o_idx;
            while this_o_idx != 0 {
                let mut o_ptr: *mut object_type = 0 as *mut object_type;
                /* Acquire object */
                o_ptr =
                    &mut *o_list.offset(this_o_idx as isize) as
                        *mut object_type;
                /* Acquire next object */
                next_o_idx = (*o_ptr).next_o_idx;
                /* Ignore random artifacts */
                if !((*o_ptr).tval as libc::c_int == 102 as libc::c_int) {
                    /* Ignore non-artifacts */
                    if (*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
                           (if (*o_ptr).name1 as libc::c_int != 0 {
                                1 as libc::c_int
                            } else { 0 as libc::c_int }) != 0 ||
                           (if (*o_ptr).art_name as libc::c_int != 0 {
                                1 as libc::c_int
                            } else { 0 as libc::c_int }) != 0 ||
                           (if (*k_info.offset((*o_ptr).k_idx as
                                                   isize)).flags3 as
                                   libc::c_long & 0x8000 as libc::c_long != 0
                               {
                                1 as libc::c_int
                            } else { 0 as libc::c_int }) != 0 {
                        /* Ignore known items */
                        if !((*o_ptr).ident as libc::c_int &
                                 0x8 as libc::c_int != 0 ||
                                 (*k_info.offset((*o_ptr).k_idx as
                                                     isize)).easy_know as
                                     libc::c_int != 0 &&
                                     (*k_info.offset((*o_ptr).k_idx as
                                                         isize)).aware as
                                         libc::c_int != 0) {
                            /* Note the artifact */
                            if (*k_info.offset((*o_ptr).k_idx as
                                                   isize)).flags3 as
                                   libc::c_long & 0x8000 as libc::c_long != 0
                               {
                                *okayk.offset((*o_ptr).k_idx as isize) =
                                    0 as libc::c_int as bool_
                            } else {
                                *okay.offset((*o_ptr).name1 as isize) =
                                    0 as libc::c_int as bool_
                            }
                        }
                    }
                }
                this_o_idx = next_o_idx
            }
            x += 1
        }
        y += 1
    }
    /* Check monsters in the dungeon */
    i = 0 as libc::c_int;
    while i < m_max as libc::c_int {
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        let mut this_o_idx_0: s16b = 0;
        let mut next_o_idx_0: s16b = 0 as libc::c_int as s16b;
        /* Scan all objects the monster carries */
        this_o_idx_0 = (*m_ptr).hold_o_idx;
        while this_o_idx_0 != 0 {
            let mut o_ptr_0: *mut object_type = 0 as *mut object_type;
            /* Acquire object */
            o_ptr_0 =
                &mut *o_list.offset(this_o_idx_0 as isize) as
                    *mut object_type;
            /* Acquire next object */
            next_o_idx_0 = (*o_ptr_0).next_o_idx;
            /* Ignore random artifacts */
            if !((*o_ptr_0).tval as libc::c_int == 102 as libc::c_int) {
                /* Ignore non-artifacts */
                if (*o_ptr_0).tval as libc::c_int == 102 as libc::c_int ||
                       (if (*o_ptr_0).name1 as libc::c_int != 0 {
                            1 as libc::c_int
                        } else { 0 as libc::c_int }) != 0 ||
                       (if (*o_ptr_0).art_name as libc::c_int != 0 {
                            1 as libc::c_int
                        } else { 0 as libc::c_int }) != 0 ||
                       (if (*k_info.offset((*o_ptr_0).k_idx as isize)).flags3
                               as libc::c_long & 0x8000 as libc::c_long != 0 {
                            1 as libc::c_int
                        } else { 0 as libc::c_int }) != 0 {
                    /* Ignore known items */
                    if !((*o_ptr_0).ident as libc::c_int & 0x8 as libc::c_int
                             != 0 ||
                             (*k_info.offset((*o_ptr_0).k_idx as
                                                 isize)).easy_know as
                                 libc::c_int != 0 &&
                                 (*k_info.offset((*o_ptr_0).k_idx as
                                                     isize)).aware as
                                     libc::c_int != 0) {
                        /* Note the artifact */
                        if (*k_info.offset((*o_ptr_0).k_idx as isize)).flags3
                               as libc::c_long & 0x8000 as libc::c_long != 0 {
                            *okayk.offset((*o_ptr_0).k_idx as isize) =
                                0 as libc::c_int as bool_
                        } else {
                            *okay.offset((*o_ptr_0).name1 as isize) =
                                0 as libc::c_int as bool_
                        }
                    }
                }
            }
            this_o_idx_0 = next_o_idx_0
        }
        i += 1
    }
    /* Check the p_ptr->inventory and equipment */
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int {
        let mut o_ptr_1: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Ignore non-objects */
        if !((*o_ptr_1).k_idx == 0) {
            /* Ignore random artifacts */
            if !((*o_ptr_1).tval as libc::c_int == 102 as libc::c_int) {
                /* Ignore non-artifacts */
                if (*o_ptr_1).tval as libc::c_int == 102 as libc::c_int ||
                       (if (*o_ptr_1).name1 as libc::c_int != 0 {
                            1 as libc::c_int
                        } else { 0 as libc::c_int }) != 0 ||
                       (if (*o_ptr_1).art_name as libc::c_int != 0 {
                            1 as libc::c_int
                        } else { 0 as libc::c_int }) != 0 ||
                       (if (*k_info.offset((*o_ptr_1).k_idx as isize)).flags3
                               as libc::c_long & 0x8000 as libc::c_long != 0 {
                            1 as libc::c_int
                        } else { 0 as libc::c_int }) != 0 {
                    /* Ignore known items */
                    if !((*o_ptr_1).ident as libc::c_int & 0x8 as libc::c_int
                             != 0 ||
                             (*k_info.offset((*o_ptr_1).k_idx as
                                                 isize)).easy_know as
                                 libc::c_int != 0 &&
                                 (*k_info.offset((*o_ptr_1).k_idx as
                                                     isize)).aware as
                                     libc::c_int != 0) {
                        /* Note the artifact */
                        if (*k_info.offset((*o_ptr_1).k_idx as isize)).flags3
                               as libc::c_long & 0x8000 as libc::c_long != 0 {
                            *okayk.offset((*o_ptr_1).k_idx as isize) =
                                0 as libc::c_int as bool_
                        } else {
                            *okay.offset((*o_ptr_1).name1 as isize) =
                                0 as libc::c_int as bool_
                        }
                    }
                }
            }
        }
        i += 1
    }
    /* Scan the artifacts */
    k = 0 as libc::c_int;
    while k < max_a_idx as libc::c_int {
        let mut a_ptr_0: *mut artifact_type =
            &mut *a_info.offset(k as isize) as *mut artifact_type;
        /* List "dead" ones */
        if !(*okay.offset(k as isize) == 0) {
            /* Paranoia */
            strcpy(base_name.as_mut_ptr(),
                   b"Unknown Artifact\x00" as *const u8 as
                       *const libc::c_char);
            /* Obtain the base object type */
            z =
                lookup_kind((*a_ptr_0).tval as libc::c_int,
                            (*a_ptr_0).sval as libc::c_int) as libc::c_int;
            /* Real object */
            if z != 0 {
                let mut forge: object_type =
                    object_type{k_idx: 0,
                                iy: 0,
                                ix: 0,
                                tval: 0,
                                sval: 0,
                                pval: 0,
                                pval2: 0,
                                pval3: 0,
                                discount: 0,
                                number: 0,
                                weight: 0,
                                elevel: 0,
                                exp: 0,
                                name1: 0,
                                name2: 0,
                                name2b: 0,
                                xtra1: 0,
                                xtra2: 0,
                                to_h: 0,
                                to_d: 0,
                                to_a: 0,
                                ac: 0,
                                dd: 0,
                                ds: 0,
                                timeout: 0,
                                ident: 0,
                                marked: 0,
                                note: 0,
                                art_name: 0,
                                art_flags1: 0,
                                art_flags2: 0,
                                art_flags3: 0,
                                art_flags4: 0,
                                art_flags5: 0,
                                art_esp: 0,
                                art_oflags1: 0,
                                art_oflags2: 0,
                                art_oflags3: 0,
                                art_oflags4: 0,
                                art_oflags5: 0,
                                art_oesp: 0,
                                next_o_idx: 0,
                                held_m_idx: 0,
                                sense: 0,
                                found: 0,
                                found_aux1: 0,
                                found_aux2: 0,
                                found_aux3: 0,
                                found_aux4: 0,};
                let mut q_ptr: *mut object_type = 0 as *mut object_type;
                let mut f1: u32b = 0;
                let mut f2: u32b = 0;
                let mut f3: u32b = 0;
                let mut f4: u32b = 0;
                let mut f5: u32b = 0;
                let mut esp: u32b = 0;
                /* Get local object */
                q_ptr = &mut forge;
                /* Create fake object */
                object_prep(q_ptr, z);
                /* Make it an artifact */
                (*q_ptr).name1 = k as byte_hack;
                /* Spell in it ? no ! */
                object_flags(q_ptr, &mut f1, &mut f2, &mut f3, &mut f4,
                             &mut f5, &mut esp);
                if f5 as libc::c_long & 0x800 as libc::c_long != 0 {
                    (*q_ptr).pval2 = -(1 as libc::c_int) as s16b
                }
                /* Describe the artifact */
                object_desc_store(base_name.as_mut_ptr(), q_ptr,
                                  0 as libc::c_int, 0 as libc::c_int);
            }
            /* Hack -- Build the artifact name */
            fprintf(fff,
                    b"     The %s\n\x00" as *const u8 as *const libc::c_char,
                    base_name.as_mut_ptr());
        }
        k += 1
    }
    k = 0 as libc::c_int;
    while k < max_k_idx as libc::c_int {
        /* List "dead" ones */
        if !(*okayk.offset(k as isize) == 0) {
            /* Paranoia */
            strcpy(base_name.as_mut_ptr(),
                   b"Unknown Artifact\x00" as *const u8 as
                       *const libc::c_char);
            /* Real object */
            if k != 0 {
                let mut forge_0: object_type =
                    object_type{k_idx: 0,
                                iy: 0,
                                ix: 0,
                                tval: 0,
                                sval: 0,
                                pval: 0,
                                pval2: 0,
                                pval3: 0,
                                discount: 0,
                                number: 0,
                                weight: 0,
                                elevel: 0,
                                exp: 0,
                                name1: 0,
                                name2: 0,
                                name2b: 0,
                                xtra1: 0,
                                xtra2: 0,
                                to_h: 0,
                                to_d: 0,
                                to_a: 0,
                                ac: 0,
                                dd: 0,
                                ds: 0,
                                timeout: 0,
                                ident: 0,
                                marked: 0,
                                note: 0,
                                art_name: 0,
                                art_flags1: 0,
                                art_flags2: 0,
                                art_flags3: 0,
                                art_flags4: 0,
                                art_flags5: 0,
                                art_esp: 0,
                                art_oflags1: 0,
                                art_oflags2: 0,
                                art_oflags3: 0,
                                art_oflags4: 0,
                                art_oflags5: 0,
                                art_oesp: 0,
                                next_o_idx: 0,
                                held_m_idx: 0,
                                sense: 0,
                                found: 0,
                                found_aux1: 0,
                                found_aux2: 0,
                                found_aux3: 0,
                                found_aux4: 0,};
                let mut q_ptr_0: *mut object_type = 0 as *mut object_type;
                /* Get local object */
                q_ptr_0 = &mut forge_0;
                /* Create fake object */
                object_prep(q_ptr_0, k);
                /* Describe the artifact */
                object_desc_store(base_name.as_mut_ptr(), q_ptr_0,
                                  0 as libc::c_int, 0 as libc::c_int);
            }
            /* Hack -- Build the artifact name */
            fprintf(fff,
                    b"     The %s\n\x00" as *const u8 as *const libc::c_char,
                    base_name.as_mut_ptr());
        }
        k += 1
    }
    /* Close the file */
    my_fclose(fff);
    /* Display the file contents */
    show_file(file_name.as_mut_ptr() as cptr,
              b"Artifacts Seen\x00" as *const u8 as *const libc::c_char,
              0 as libc::c_int, 0 as libc::c_int);
    /* Remove the file */
    fd_kill(file_name.as_mut_ptr() as cptr);
    rnfree(okay as vptr,
           (max_a_idx as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>() as
                                                libc::c_ulong));
    rnfree(okayk as vptr,
           (max_k_idx as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>() as
                                                libc::c_ulong));
}
/*
 * Check the status of traps
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_knowledge_traps() {
    let mut k: libc::c_int = 0;
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut t_ptr: *mut trap_type = 0 as *mut trap_type;
    let mut file_name: [libc::c_char; 1024] = [0; 1024];
    /* Temporary file */
    if path_temp(file_name.as_mut_ptr(), 1024 as libc::c_int) != 0 { return }
    /* Open a new file */
    fff =
        my_fopen(file_name.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    /* Scan the traps */
    k = 0 as libc::c_int;
    while k < max_t_idx as libc::c_int {
        /* Get the trap */
        t_ptr = &mut *t_info.offset(k as isize) as *mut trap_type;
        /* Skip "empty" traps */
        if !((*t_ptr).name == 0) {
            /* Skip unidentified traps */
            if !((*t_ptr).ident == 0) {
                /* Hack -- Build the trap name */
                fprintf(fff,
                        b"     %s\n\x00" as *const u8 as *const libc::c_char,
                        t_name.offset((*t_ptr).name as libc::c_int as isize));
            }
        }
        k += 1
    }
    /* Close the file */
    my_fclose(fff);
    /* Display the file contents */
    show_file(file_name.as_mut_ptr() as cptr,
              b"Traps known\x00" as *const u8 as *const libc::c_char,
              0 as libc::c_int, 0 as libc::c_int);
    /* Remove the file */
    fd_kill(file_name.as_mut_ptr() as cptr);
}
/*
 * Display known uniques
 *
 * Note that the player ghosts are ignored.  XXX XXX XXX
 */
unsafe extern "C" fn insert_sort_unique(mut sort_uniques: *mut libc::c_int,
                                        mut num: *mut libc::c_int,
                                        mut r_idx: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    let mut level: libc::c_int = (*r_ptr).level as libc::c_int;
    /* Hack -- Morgoth is always at the bottom of the list */
    if r_idx == 862 as libc::c_int { level = 20000 as libc::c_int }
    /* Find the place */
    i = 0 as libc::c_int;
    while i < *num {
        let mut r2_ptr: *mut monster_race =
            &mut *r_info.offset(*sort_uniques.offset(i as isize) as isize) as
                *mut monster_race;
        let mut level2: libc::c_int = (*r2_ptr).level as libc::c_int;
        if *sort_uniques.offset(i as isize) == 862 as libc::c_int {
            level2 = 20000 as libc::c_int
        }
        if level < level2 { break ; }
        i += 1
    }
    /* Move the remaining items */
    j = *num - 1 as libc::c_int;
    while j >= i {
        *sort_uniques.offset((j + 1 as libc::c_int) as isize) =
            *sort_uniques.offset(j as isize);
        j -= 1
    }
    /* Insert it */
    *sort_uniques.offset(i as isize) = r_idx;
    *num += 1;
}
unsafe extern "C" fn do_cmd_knowledge_uniques() {
    let mut k: libc::c_int = 0;
    let mut sort_uniques: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut file_name: [libc::c_char; 1024] = [0; 1024];
    /* Temporary file */
    if path_temp(file_name.as_mut_ptr(), 1024 as libc::c_int) != 0 { return }
    /* Open a new file */
    fff =
        my_fopen(file_name.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    sort_uniques =
        memset(ralloc((max_r_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_r_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong)) as
            *mut libc::c_int;
    /* Sort the monster races */
    k = 1 as libc::c_int;
    while k < max_r_idx as libc::c_int {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset(k as isize) as *mut monster_race;
        /* Only print Uniques */
        if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 &&
               (*r_ptr).flags7 & 0x10 as libc::c_int as libc::c_uint == 0 &&
               (*r_ptr).flags7 & 0x4000 as libc::c_int as libc::c_uint == 0 {
            insert_sort_unique(sort_uniques, &mut num, k);
        }
        k += 1
    }
    /* Scan the monster races -- sorted */
    k = 0 as libc::c_int;
    while k < num {
        let mut r_ptr_0: *mut monster_race =
            &mut *r_info.offset(*sort_uniques.offset(k as isize) as isize) as
                *mut monster_race;
        /* Only print Uniques */
        if (*r_ptr_0).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
            let mut dead: bool_ =
                ((*r_ptr_0).max_num as libc::c_int == 0 as libc::c_int) as
                    libc::c_int as bool_;
            /* Only display "known" uniques */
            if dead as libc::c_int != 0 || cheat_know as libc::c_int != 0 ||
                   (*r_ptr_0).r_sights as libc::c_int != 0 {
                /* Print a message */
                if dead != 0 {
                    /* Don't print the unique's ASCII symbol
					 * if use_graphics is on. */
                    if use_graphics != 0 {
                        fprintf(fff,
                                b"[[[[[R%-70s is dead]\n\x00" as *const u8 as
                                    *const libc::c_char,
                                r_name.offset((*r_ptr_0).name as isize));
                    } else {
                        fprintf(fff,
                                b"[[[[[%c%c] [[[[[R%-68s is dead]\n\x00" as
                                    *const u8 as *const libc::c_char,
                                conv_color[(*r_ptr_0).d_attr as usize] as
                                    libc::c_int,
                                (*r_ptr_0).d_char as libc::c_int,
                                r_name.offset((*r_ptr_0).name as isize));
                    }
                } else if use_graphics != 0 {
                    fprintf(fff,
                            b"[[[[[w%-70s is alive]\n\x00" as *const u8 as
                                *const libc::c_char,
                            r_name.offset((*r_ptr_0).name as isize));
                } else {
                    fprintf(fff,
                            b"[[[[[%c%c] [[[[[w%-68s is alive]\n\x00" as
                                *const u8 as *const libc::c_char,
                            conv_color[(*r_ptr_0).d_attr as usize] as
                                libc::c_int, (*r_ptr_0).d_char as libc::c_int,
                            r_name.offset((*r_ptr_0).name as isize));
                }
            }
        }
        k += 1
    }
    rnfree(sort_uniques as vptr,
           (max_r_idx as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong));
    /* Don't print the unique's ASCII symbol
					 * if use_graphics is on. */
    /* Close the file */
    my_fclose(fff);
    /* Display the file contents */
    show_file(file_name.as_mut_ptr() as cptr,
              b"Known Uniques\x00" as *const u8 as *const libc::c_char,
              0 as libc::c_int, 0 as libc::c_int);
    /* Remove the file */
    fd_kill(file_name.as_mut_ptr() as cptr);
}
#[no_mangle]
pub unsafe extern "C" fn plural_aux(mut name: *mut libc::c_char) {
    let mut name_len: libc::c_int = strlen(name) as libc::c_int;
    /* Hack -- Precedent must be pluralised for this one */
    if !strstr(name,
               b"Disembodied hand\x00" as *const u8 as
                   *const libc::c_char).is_null() {
        strcpy(name,
               b"Disembodied hands that strangled people\x00" as *const u8 as
                   *const libc::c_char);
    } else if !strstr(name,
                      b" of \x00" as *const u8 as
                          *const libc::c_char).is_null() {
        let mut aider: cptr =
            strstr(name, b" of \x00" as *const u8 as *const libc::c_char) as
                cptr;
        let mut dummy: [libc::c_char; 80] = [0; 80];
        let mut i: libc::c_int = 0 as libc::c_int;
        let mut ctr: cptr = name as cptr;
        while ctr < aider {
            dummy[i as usize] = *ctr;
            ctr = ctr.offset(1);
            i += 1
        }
        if dummy[(i - 1 as libc::c_int) as usize] as libc::c_int == 's' as i32
           {
            strcpy(&mut *dummy.as_mut_ptr().offset(i as isize),
                   b"es\x00" as *const u8 as *const libc::c_char);
            i += 1
        } else {
            strcpy(&mut *dummy.as_mut_ptr().offset(i as isize),
                   b"s\x00" as *const u8 as *const libc::c_char);
        }
        strcpy(&mut *dummy.as_mut_ptr().offset((i + 1 as libc::c_int) as
                                                   isize), aider);
        strcpy(name, dummy.as_mut_ptr());
    } else if !strstr(name,
                      b"coins\x00" as *const u8 as
                          *const libc::c_char).is_null() {
        let mut dummy_0: [libc::c_char; 80] = [0; 80];
        strcpy(dummy_0.as_mut_ptr(),
               b"piles of \x00" as *const u8 as *const libc::c_char);
        strcat(dummy_0.as_mut_ptr(), name);
        strcpy(name, dummy_0.as_mut_ptr());
        return
    } else {
        /* "someone of something" */
        /* Creeping coins */
        /* Manes stay manes */
        if !strstr(name,
                   b"Manes\x00" as *const u8 as *const libc::c_char).is_null()
           {
            return
        } else {
            /* Broken plurals are, well, broken */
            if *name.offset((name_len - 1 as libc::c_int) as isize) as
                   libc::c_int == 'y' as i32 {
                strcpy(&mut *name.offset((name_len - 1 as libc::c_int) as
                                             isize),
                       b"ies\x00" as *const u8 as *const libc::c_char);
            } else if streq(&mut *name.offset((name_len - 4 as libc::c_int) as
                                                  isize) as *mut libc::c_char
                                as cptr,
                            b"ouse\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                strcpy(&mut *name.offset((name_len - 4 as libc::c_int) as
                                             isize),
                       b"ice\x00" as *const u8 as *const libc::c_char);
            } else if streq(&mut *name.offset((name_len - 6 as libc::c_int) as
                                                  isize) as *mut libc::c_char
                                as cptr,
                            b"kelman\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                strcpy(&mut *name.offset((name_len - 6 as libc::c_int) as
                                             isize),
                       b"kelmen\x00" as *const u8 as *const libc::c_char);
            } else if streq(&mut *name.offset((name_len - 2 as libc::c_int) as
                                                  isize) as *mut libc::c_char
                                as cptr,
                            b"ex\x00" as *const u8 as *const libc::c_char) !=
                          0 {
                strcpy(&mut *name.offset((name_len - 2 as libc::c_int) as
                                             isize),
                       b"ices\x00" as *const u8 as *const libc::c_char);
            } else if streq(&mut *name.offset((name_len - 3 as libc::c_int) as
                                                  isize) as *mut libc::c_char
                                as cptr,
                            b"olf\x00" as *const u8 as *const libc::c_char) !=
                          0 {
                strcpy(&mut *name.offset((name_len - 3 as libc::c_int) as
                                             isize),
                       b"olves\x00" as *const u8 as *const libc::c_char);
            } else if streq(&mut *name.offset((name_len - 2 as libc::c_int) as
                                                  isize) as *mut libc::c_char
                                as cptr,
                            b"ch\x00" as *const u8 as *const libc::c_char) as
                          libc::c_int != 0 ||
                          *name.offset((name_len - 1 as libc::c_int) as isize)
                              as libc::c_int == 's' as i32 {
                strcpy(&mut *name.offset(name_len as isize),
                       b"es\x00" as *const u8 as *const libc::c_char);
            } else {
                strcpy(&mut *name.offset(name_len as isize),
                       b"s\x00" as *const u8 as *const libc::c_char);
            }
        }
    };
}
/* Now begins sane cases */
/*
 * Display current pets
 */
unsafe extern "C" fn do_cmd_knowledge_pets() {
    let mut i: libc::c_int = 0;
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    let mut t_friends: libc::c_int = 0 as libc::c_int;
    let mut t_levels: libc::c_int = 0 as libc::c_int;
    let mut show_upkeep: libc::c_int = 0 as libc::c_int;
    let mut upkeep_divider: libc::c_int = 20 as libc::c_int;
    let mut file_name: [libc::c_char; 1024] = [0; 1024];
    /* Temporary file */
    if path_temp(file_name.as_mut_ptr(), 1024 as libc::c_int) != 0 { return }
    /* Open a new file */
    fff =
        my_fopen(file_name.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    if has_ability(2 as libc::c_int) != 0 {
        upkeep_divider = 15 as libc::c_int
    }
    /* Process the monsters (backwards) */
    i = m_max as libc::c_int - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        /* Access the monster */
        m_ptr = &mut *m_list.offset(i as isize) as *mut monster_type;
        /* Ignore "dead" monsters */
        if !((*m_ptr).r_idx == 0) {
            /* Calculate "upkeep" for friendly monsters */
            if (*m_ptr).status as libc::c_int >= 3 as libc::c_int {
                let mut pet_name: [libc::c_char; 80] = [0; 80];
                let mut r_ptr: *mut monster_race =
                    if !(*m_ptr).sr_ptr.is_null() {
                        (*m_ptr).sr_ptr
                    } else {
                        race_info_idx((*m_ptr).r_idx as libc::c_int,
                                      (*m_ptr).ego as libc::c_int)
                    };
                t_friends += 1;
                t_levels += (*m_ptr).level as libc::c_int;
                monster_desc(pet_name.as_mut_ptr(), m_ptr,
                             0x88 as libc::c_int);
                fprintf(fff,
                        b"%s%s (%s)\n\x00" as *const u8 as
                            *const libc::c_char,
                        if (*r_ptr).flags1 &
                               0x1 as libc::c_int as libc::c_uint != 0 {
                            b"#####G\x00" as *const u8 as *const libc::c_char
                        } else {
                            b"\x00" as *const u8 as *const libc::c_char
                        }, pet_name.as_mut_ptr(),
                        if ((*m_ptr).status as libc::c_int) < 4 as libc::c_int
                           {
                            b"pet\x00" as *const u8 as *const libc::c_char
                        } else {
                            b"companion\x00" as *const u8 as
                                *const libc::c_char
                        });
            }
        }
        i -= 1
    }
    if t_friends >
           1 as libc::c_int + (*p_ptr).lev as libc::c_int / upkeep_divider {
        show_upkeep = t_levels;
        if show_upkeep > 100 as libc::c_int {
            show_upkeep = 100 as libc::c_int
        } else if show_upkeep < 10 as libc::c_int {
            show_upkeep = 10 as libc::c_int
        }
    }
    fprintf(fff,
            b"----------------------------------------------\n\x00" as
                *const u8 as *const libc::c_char);
    fprintf(fff,
            b"   Total: %d pet%s.\n\x00" as *const u8 as *const libc::c_char,
            t_friends,
            if t_friends == 1 as libc::c_int {
                b"\x00" as *const u8 as *const libc::c_char
            } else { b"s\x00" as *const u8 as *const libc::c_char });
    fprintf(fff,
            b"   Upkeep: %d%% mana.\n\x00" as *const u8 as
                *const libc::c_char, show_upkeep);
    /* Close the file */
    my_fclose(fff);
    /* Display the file contents */
    show_file(file_name.as_mut_ptr() as cptr,
              b"Current Pets\x00" as *const u8 as *const libc::c_char,
              0 as libc::c_int, 0 as libc::c_int);
    /* Remove the file */
    fd_kill(file_name.as_mut_ptr() as cptr);
}
/*
 * Total kill count
 *
 * Note that the player ghosts are ignored.  XXX XXX XXX
 */
unsafe extern "C" fn do_cmd_knowledge_kill_count() {
    let mut k: libc::c_int = 0;
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut file_name: [libc::c_char; 1024] = [0; 1024];
    let mut Total: s32b = 0 as libc::c_int;
    /* Temporary file */
    if path_temp(file_name.as_mut_ptr(), 1024 as libc::c_int) != 0 { return }
    /* Open a new file */
    fff =
        my_fopen(file_name.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    /* Monsters slain */
    let mut kk: libc::c_int = 0;
    /* For all monsters */
    kk = 1 as libc::c_int;
    while kk < max_r_idx as libc::c_int {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset(kk as isize) as *mut monster_race;
        if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
            let mut dead: bool_ =
                ((*r_ptr).max_num as libc::c_int == 0 as libc::c_int) as
                    libc::c_int as bool_;
            if dead != 0 { Total += 1 }
        } else {
            let mut This: s16b = (*r_ptr).r_pkills;
            if This as libc::c_int > 0 as libc::c_int {
                Total += This as libc::c_int
            }
        }
        kk += 1
    }
    if Total < 1 as libc::c_int {
        fprintf(fff,
                b"You have defeated no enemies yet.\n\n\x00" as *const u8 as
                    *const libc::c_char);
    } else if Total == 1 as libc::c_int {
        fprintf(fff,
                b"You have defeated one enemy.\n\n\x00" as *const u8 as
                    *const libc::c_char);
    } else {
        fprintf(fff,
                b"You have defeated %ld enemies.\n\n\x00" as *const u8 as
                    *const libc::c_char, Total as libc::c_long);
    }
    Total = 0 as libc::c_int;
    /* Scan the monster races */
    k = 0 as libc::c_int;
    while k < max_r_idx as libc::c_int {
        let mut r_ptr_0: *mut monster_race =
            &mut *r_info.offset(k as isize) as *mut monster_race;
        if (*r_ptr_0).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
            let mut dead_0: bool_ =
                ((*r_ptr_0).max_num as libc::c_int == 0 as libc::c_int) as
                    libc::c_int as bool_;
            if dead_0 != 0 {
                /* Print a message */
                fprintf(fff,
                        b"     %s\n\x00" as *const u8 as *const libc::c_char,
                        r_name.offset((*r_ptr_0).name as isize));
                Total += 1
            }
        } else {
            let mut This_0: s16b = (*r_ptr_0).r_pkills;
            if This_0 as libc::c_int > 0 as libc::c_int {
                if (This_0 as libc::c_int) < 2 as libc::c_int {
                    if !strstr(r_name.offset((*r_ptr_0).name as isize),
                               b"coins\x00" as *const u8 as
                                   *const libc::c_char).is_null() {
                        fprintf(fff,
                                b"     1 pile of %s\n\x00" as *const u8 as
                                    *const libc::c_char,
                                r_name.offset((*r_ptr_0).name as isize));
                    } else {
                        fprintf(fff,
                                b"     1 %s\n\x00" as *const u8 as
                                    *const libc::c_char,
                                r_name.offset((*r_ptr_0).name as isize));
                    }
                } else {
                    let mut to_plural: [libc::c_char; 80] = [0; 80];
                    strcpy(to_plural.as_mut_ptr(),
                           r_name.offset((*r_ptr_0).name as isize));
                    plural_aux(to_plural.as_mut_ptr());
                    fprintf(fff,
                            b"     %d %s\n\x00" as *const u8 as
                                *const libc::c_char, This_0 as libc::c_int,
                            to_plural.as_mut_ptr());
                }
                Total += This_0 as libc::c_int
            }
        }
        k += 1
    }
    fprintf(fff,
            b"----------------------------------------------\n\x00" as
                *const u8 as *const libc::c_char);
    fprintf(fff,
            b"   Total: %ld creature%s killed.\n\x00" as *const u8 as
                *const libc::c_char, Total as libc::c_long,
            if Total == 1 as libc::c_int {
                b"\x00" as *const u8 as *const libc::c_char
            } else { b"s\x00" as *const u8 as *const libc::c_char });
    /* Close the file */
    my_fclose(fff);
    /* Display the file contents */
    show_file(file_name.as_mut_ptr() as cptr,
              b"Kill Count\x00" as *const u8 as *const libc::c_char,
              0 as libc::c_int, 0 as libc::c_int);
    /* Remove the file */
    fd_kill(file_name.as_mut_ptr() as cptr);
}
/*
 * Display known objects
 */
unsafe extern "C" fn do_cmd_knowledge_objects() {
    let mut k: libc::c_int = 0;
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut file_name: [libc::c_char; 1024] = [0; 1024];
    /* Temporary file */
    if path_temp(file_name.as_mut_ptr(), 1024 as libc::c_int) != 0 { return }
    /* Open a new file */
    fff =
        my_fopen(file_name.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    /* Scan the object kinds */
    k = 1 as libc::c_int;
    while k < max_k_idx as libc::c_int {
        let mut k_ptr: *mut object_kind =
            &mut *k_info.offset(k as isize) as *mut object_kind;
        /* Hack -- skip artifacts */
        if !((*k_ptr).flags3 as libc::c_long & 0x800 as libc::c_long != 0) {
            /* List known flavored objects */
            if (*k_ptr).flavor as libc::c_int != 0 &&
                   (*k_ptr).aware as libc::c_int != 0 {
                let mut i_ptr: *mut object_type = 0 as *mut object_type;
                let mut object_type_body: object_type =
                    object_type{k_idx: 0,
                                iy: 0,
                                ix: 0,
                                tval: 0,
                                sval: 0,
                                pval: 0,
                                pval2: 0,
                                pval3: 0,
                                discount: 0,
                                number: 0,
                                weight: 0,
                                elevel: 0,
                                exp: 0,
                                name1: 0,
                                name2: 0,
                                name2b: 0,
                                xtra1: 0,
                                xtra2: 0,
                                to_h: 0,
                                to_d: 0,
                                to_a: 0,
                                ac: 0,
                                dd: 0,
                                ds: 0,
                                timeout: 0,
                                ident: 0,
                                marked: 0,
                                note: 0,
                                art_name: 0,
                                art_flags1: 0,
                                art_flags2: 0,
                                art_flags3: 0,
                                art_flags4: 0,
                                art_flags5: 0,
                                art_esp: 0,
                                art_oflags1: 0,
                                art_oflags2: 0,
                                art_oflags3: 0,
                                art_oflags4: 0,
                                art_oflags5: 0,
                                art_oesp: 0,
                                next_o_idx: 0,
                                held_m_idx: 0,
                                sense: 0,
                                found: 0,
                                found_aux1: 0,
                                found_aux2: 0,
                                found_aux3: 0,
                                found_aux4: 0,};
                /* Get local object */
                i_ptr = &mut object_type_body;
                /* Create fake object */
                object_prep(i_ptr, k);
                /* Describe the object */
                object_desc_store(o_name.as_mut_ptr(), i_ptr,
                                  0 as libc::c_int, 0 as libc::c_int);
                /* Print a message */
                fprintf(fff,
                        b"     %s\n\x00" as *const u8 as *const libc::c_char,
                        o_name.as_mut_ptr());
            }
        }
        k += 1
    }
    /* Close the file */
    my_fclose(fff);
    /* Display the file contents */
    show_file(file_name.as_mut_ptr() as cptr,
              b"Known Objects\x00" as *const u8 as *const libc::c_char,
              0 as libc::c_int, 0 as libc::c_int);
    /* Remove the file */
    fd_kill(file_name.as_mut_ptr() as cptr);
}
/*
 * List recall depths
 */
unsafe extern "C" fn do_cmd_knowledge_dungeons() {
    let mut y: libc::c_int = 0;
    let mut file_name: [libc::c_char; 1024] = [0; 1024];
    let mut fff: *mut FILE = 0 as *mut FILE;
    /* Temporary file */
    if path_temp(file_name.as_mut_ptr(), 1024 as libc::c_int) != 0 { return }
    /* Open a new file */
    fff =
        my_fopen(file_name.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    /* Oops */
    if fff.is_null() { return }
    /* Scan all dungeons */
    y = 1 as libc::c_int;
    while y < max_d_idx as libc::c_int {
        /* The dungeon has a valid recall depth set */
        if *max_dlv.offset(y as isize) != 0 {
            /* Describe the recall depth */
            fprintf(fff,
                    b"       %c%s: Level %d (%d\')\n\x00" as *const u8 as
                        *const libc::c_char,
                    if (*p_ptr).recall_dungeon as libc::c_int == y {
                        '*' as i32
                    } else { ' ' as i32 },
                    d_name.offset((*d_info.offset(y as isize)).name as isize),
                    *max_dlv.offset(y as isize) as libc::c_int,
                    50 as libc::c_int *
                        *max_dlv.offset(y as isize) as libc::c_int);
        }
        y += 1
    }
    /* Close the file */
    my_fclose(fff);
    /* Display the file contents */
    show_file(file_name.as_mut_ptr() as cptr,
              b"Recall Depths\x00" as *const u8 as *const libc::c_char,
              0 as libc::c_int, 0 as libc::c_int);
    /* Remove the file */
    fd_kill(file_name.as_mut_ptr() as cptr);
}
/*
 * List known towns
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_knowledge_towns() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut file_name: [libc::c_char; 1024] = [0; 1024];
    let mut fff: *mut FILE = 0 as *mut FILE;
    /* Temporary file */
    if path_temp(file_name.as_mut_ptr(), 1024 as libc::c_int) != 0 { return }
    /* Open a new file */
    fff =
        my_fopen(file_name.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    /* Oops */
    if fff.is_null() { return }
    /* Scan all dungeons */
    i = 0 as libc::c_int;
    while i < max_d_idx as libc::c_int {
        let mut d_ptr: *mut dungeon_info_type =
            &mut *d_info.offset(i as isize) as *mut dungeon_info_type;
        /* Scan all dungeon town slots */
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            let mut town_idx: libc::c_int =
                (*d_ptr).t_idx[j as usize] as libc::c_int;
            /* Ignore non-existent towns */
            if !((*town_info.offset(town_idx as isize)).flags as libc::c_int &
                     0x1 as libc::c_int == 0) {
                /* Ignore unknown towns */
                if !((*town_info.offset(town_idx as isize)).flags as
                         libc::c_int & 0x2 as libc::c_int == 0) {
                    /* Describe the dungeon town */
                    fprintf(fff,
                            b"        %s: Level %d (%d\')\n\x00" as *const u8
                                as *const libc::c_char,
                            d_name.offset((*d_ptr).name as isize),
                            (*d_ptr).t_level[j as usize] as libc::c_int,
                            50 as libc::c_int *
                                (*d_ptr).t_level[j as usize] as libc::c_int);
                }
            }
            j += 1
        }
        i += 1
    }
    /* Close the file */
    my_fclose(fff);
    /* Display the file contents */
    show_file(file_name.as_mut_ptr() as cptr,
              b"Dungeon Towns\x00" as *const u8 as *const libc::c_char,
              0 as libc::c_int, 0 as libc::c_int);
    /* Remove the file */
    fd_kill(file_name.as_mut_ptr() as cptr);
}
/*
 * List corruptions
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_knowledge_corruptions() {
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut file_name: [libc::c_char; 1024] = [0; 1024];
    /* Temporary file */
    if path_temp(file_name.as_mut_ptr(), 1024 as libc::c_int) != 0 { return }
    /* Open a new file */
    fff =
        my_fopen(file_name.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    /* Dump the corruptions to file */
    if !fff.is_null() { dump_corruptions(fff, 1 as libc::c_int as bool_); }
    /* Close the file */
    my_fclose(fff);
    /* Display the file contents */
    show_file(file_name.as_mut_ptr() as cptr,
              b"Corruptions\x00" as *const u8 as *const libc::c_char,
              0 as libc::c_int, 0 as libc::c_int);
    /* Remove the file */
    fd_kill(file_name.as_mut_ptr() as cptr);
}
/*
 * Helper function for do_cmd_knowledge_quests
 */
unsafe extern "C" fn insert_sort_quest(mut order: *mut libc::c_int,
                                       mut num: *mut libc::c_int,
                                       mut q_idx: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut q_ptr: *mut quest_type =
        &mut *quest.offset(q_idx as isize) as *mut quest_type;
    let mut level: libc::c_int = (*q_ptr).level as libc::c_int;
    /* Find the place */
    i = 0 as libc::c_int;
    while i < *num {
        let mut q2_ptr: *mut quest_type =
            &mut *quest.offset(*order.offset(i as isize) as isize) as
                *mut quest_type;
        let mut level2: libc::c_int = (*q2_ptr).level as libc::c_int;
        if level < level2 { break ; }
        i += 1
    }
    /* Move the remaining items */
    j = *num - 1 as libc::c_int;
    while j >= i {
        *order.offset((j + 1 as libc::c_int) as isize) =
            *order.offset(j as isize);
        j -= 1
    }
    /* Insert it */
    *order.offset(i as isize) = q_idx;
    *num += 1;
}
/*
 * Print quest status of all active quests
 */
unsafe extern "C" fn do_cmd_knowledge_quests() {
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut file_name: [libc::c_char; 1024] = [0; 1024];
    let mut order: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    /* Temporary file */
    if path_temp(file_name.as_mut_ptr(), 1024 as libc::c_int) != 0 { return }
    /* Open a new file */
    fff =
        my_fopen(file_name.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    order =
        memset(ralloc((max_q_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_q_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong)) as
            *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < max_q_idx as libc::c_int {
        insert_sort_quest(order, &mut num, i);
        i += 1
    }
    z = 0 as libc::c_int;
    while z < max_q_idx as libc::c_int {
        i = *order.offset(z as isize);
        /* Dynamic quests */
        if (*quest.offset(i as isize)).dynamic_desc != 0 {
            /* C type quests */
            if (*quest.offset(i as isize)).type_0 as libc::c_int ==
                   0 as libc::c_int {
                ((*quest.offset(i as
                                    isize)).gen_desc.expect("non-null function pointer")(fff))
                    == 0;
            } else {
                /* MUST be a lua quest */
                hook_file = fff;
                exec_lua(format(b"__quest_dynamic_desc[%d]()\x00" as *const u8
                                    as *const libc::c_char, i));
            }
        } else if (*quest.offset(i as isize)).silent == 0 {
            if (*quest.offset(i as isize)).status as libc::c_int ==
                   1 as libc::c_int {
                /* Fixed quests (only known ones) */
                /* Print the quest info */
                fprintf(fff,
                        b"#####y%s (Danger level: %d)\n\x00" as *const u8 as
                            *const libc::c_char,
                        (*quest.offset(i as isize)).name.as_mut_ptr(),
                        (*quest.offset(i as isize)).level as libc::c_int);
                j = 0 as libc::c_int;
                while j < 10 as libc::c_int &&
                          (*quest.offset(i as
                                             isize)).desc[j as
                                                              usize][0 as
                                                                         libc::c_int
                                                                         as
                                                                         usize]
                              as libc::c_int != '\u{0}' as i32 {
                    let fresh2 = j;
                    j = j + 1;
                    fprintf(fff,
                            b"%s\n\x00" as *const u8 as *const libc::c_char,
                            (*quest.offset(i as
                                               isize)).desc[fresh2 as
                                                                usize].as_mut_ptr());
                }
                fprintf(fff, b"\n\x00" as *const u8 as *const libc::c_char);
            } else if (*quest.offset(i as isize)).status as libc::c_int ==
                          2 as libc::c_int {
                fprintf(fff,
                        b"#####G%s Completed - Unrewarded\n\x00" as *const u8
                            as *const libc::c_char,
                        (*quest.offset(i as isize)).name.as_mut_ptr());
                fprintf(fff, b"\n\x00" as *const u8 as *const libc::c_char);
            }
        }
        z += 1
    }
    rnfree(order as vptr,
           (max_q_idx as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong));
    /* Close the file */
    my_fclose(fff);
    /* Display the file contents */
    show_file(file_name.as_mut_ptr() as cptr,
              b"Quest status\x00" as *const u8 as *const libc::c_char,
              0 as libc::c_int, 0 as libc::c_int);
    /* Remove the file */
    fd_kill(file_name.as_mut_ptr() as cptr);
}
/*
 * Print fate status
 */
unsafe extern "C" fn do_cmd_knowledge_fates() {
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut file_name: [libc::c_char; 1024] = [0; 1024];
    /* Temporary file */
    if path_temp(file_name.as_mut_ptr(), 1024 as libc::c_int) != 0 { return }
    /* Open a new file */
    fff =
        my_fopen(file_name.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    dump_fates(fff);
    /* Close the file */
    my_fclose(fff);
    /* Display the file contents */
    show_file(file_name.as_mut_ptr() as cptr,
              b"Fate status\x00" as *const u8 as *const libc::c_char,
              0 as libc::c_int, 0 as libc::c_int);
    /* Remove the file */
    fd_kill(file_name.as_mut_ptr() as cptr);
}
/*
 * Print the note file
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_knowledge_notes() {
    /* Spawn */
    show_notes_file();
}
/*
 * Interact with "knowledge"
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_knowledge() {
    let mut i: libc::c_int = 0;
    /* File type is "TEXT" */
    /* Enter "icky" mode */
    character_icky = 1 as libc::c_int as bool_;
    /* Save the screen */
    Term_save();
    loop 
         /* Interact until done */
         /* Clear screen */
         {
        Term_clear();
        /* Ask for a choice */
        prt(b"Display current knowledge\x00" as *const u8 as
                *const libc::c_char, 2 as libc::c_int, 0 as libc::c_int);
        /* Give some choices */
        prt(b"(1) Display known artifacts\x00" as *const u8 as
                *const libc::c_char, 4 as libc::c_int, 5 as libc::c_int);
        prt(b"(2) Display known uniques\x00" as *const u8 as
                *const libc::c_char, 5 as libc::c_int, 5 as libc::c_int);
        prt(b"(3) Display known objects\x00" as *const u8 as
                *const libc::c_char, 6 as libc::c_int, 5 as libc::c_int);
        prt(b"(4) Display kill count\x00" as *const u8 as *const libc::c_char,
            7 as libc::c_int, 5 as libc::c_int);
        prt(b"(5) Display recall depths\x00" as *const u8 as
                *const libc::c_char, 8 as libc::c_int, 5 as libc::c_int);
        prt(b"(6) Display corruptions\x00" as *const u8 as
                *const libc::c_char, 9 as libc::c_int, 5 as libc::c_int);
        prt(b"(7) Display current pets\x00" as *const u8 as
                *const libc::c_char, 10 as libc::c_int, 5 as libc::c_int);
        prt(b"(8) Display current quests\x00" as *const u8 as
                *const libc::c_char, 11 as libc::c_int, 5 as libc::c_int);
        prt(b"(9) Display current fates\x00" as *const u8 as
                *const libc::c_char, 12 as libc::c_int, 5 as libc::c_int);
        prt(b"(0) Display known traps\x00" as *const u8 as
                *const libc::c_char, 13 as libc::c_int, 5 as libc::c_int);
        prt(b"(A) Display known dungeon towns\x00" as *const u8 as
                *const libc::c_char, 14 as libc::c_int, 5 as libc::c_int);
        if take_notes != 0 {
            prt(b"(B) Display notes\x00" as *const u8 as *const libc::c_char,
                15 as libc::c_int, 5 as libc::c_int);
        }
        /* Prompt */
        prt(b"Command: \x00" as *const u8 as *const libc::c_char,
            17 as libc::c_int, 0 as libc::c_int);
        /* Prompt */
        i = inkey() as libc::c_int;
        /* Done */
        if i == '\u{1b}' as i32 { break ; }
        match i {
            49 => {
                /* Artifacts */
                do_cmd_knowledge_artifacts();
            }
            50 => {
                /* Uniques */
                do_cmd_knowledge_uniques();
            }
            51 => {
                /* Objects */
                do_cmd_knowledge_objects();
            }
            52 => {
                /* Kill count  */
                do_cmd_knowledge_kill_count();
            }
            53 => {
                /* Recall depths */
                do_cmd_knowledge_dungeons();
            }
            54 => {
                /* corruptions */
                do_cmd_knowledge_corruptions();
            }
            55 => {
                /* Pets */
                do_cmd_knowledge_pets();
            }
            56 => {
                /* Quests */
                do_cmd_knowledge_quests();
            }
            57 => {
                /* Fates */
                do_cmd_knowledge_fates();
            }
            48 => {
                /* Traps */
                do_cmd_knowledge_traps();
            }
            65 | 97 => {
                /* Dungeon towns */
                do_cmd_knowledge_towns();
            }
            66 | 98 => {
                /* Notes */
                if take_notes != 0 {
                    do_cmd_knowledge_notes();
                } else { bell(); }
            }
            _ => {
                /* Unknown option */
                bell();
            }
        }
        /* Flush messages */
        msg_print(0 as cptr);
    }
    /* Restore the screen */
    Term_load();
    /* Leave "icky" mode */
    character_icky = 0 as libc::c_int as bool_;
}
/*
 * Check on the status of an active quest -KMW-
 * TODO: Spill out status when not a simple kill # monster.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_checkquest() {
    /* File type is "TEXT" */
    /* Enter "icky" mode */
    character_icky = 1 as libc::c_int as bool_;
    /* Save the screen */
    Term_save();
    /* Quest info */
    do_cmd_knowledge_quests();
    /* Restore the screen */
    Term_load();
    /* Leave "icky" mode */
    character_icky = 0 as libc::c_int as bool_;
}
/*
 * Change player's "tactic" setting
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_change_tactic(mut i: libc::c_int) {
    (*p_ptr).tactic = ((*p_ptr).tactic as libc::c_int + i) as libc::c_char;
    if (*p_ptr).tactic as libc::c_int > 8 as libc::c_int {
        (*p_ptr).tactic = 0 as libc::c_int as libc::c_char
    }
    if ((*p_ptr).tactic as libc::c_int) < 0 as libc::c_int {
        (*p_ptr).tactic = 8 as libc::c_int as libc::c_char
    }
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    update_stuff();
    prt(b"\x00" as *const u8 as *const libc::c_char, 0 as libc::c_int,
        0 as libc::c_int);
}
/*
 * Change player's "movement" setting
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_change_movement(mut i: libc::c_int) {
    (*p_ptr).movement =
        ((*p_ptr).movement as libc::c_int + i) as libc::c_char;
    if (*p_ptr).movement as libc::c_int > 8 as libc::c_int {
        (*p_ptr).movement = 0 as libc::c_int as libc::c_char
    }
    if ((*p_ptr).movement as libc::c_int) < 0 as libc::c_int {
        (*p_ptr).movement = 8 as libc::c_int as libc::c_char
    }
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    update_stuff();
    prt(b"\x00" as *const u8 as *const libc::c_char, 0 as libc::c_int,
        0 as libc::c_int);
}
/*
 * Display the time and date
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_time() {
    let mut day: libc::c_int = bst(11520 as libc::c_int, turn);
    let mut hour: libc::c_int =
        bst(11520 as libc::c_int / 24 as libc::c_int, turn);
    let mut min: libc::c_int =
        bst(11520 as libc::c_int / 24 as libc::c_int / 60 as libc::c_int,
            turn);
    let mut full: libc::c_int = hour * 100 as libc::c_int + min;
    let mut buf2: [libc::c_char; 20] = [0; 20];
    let mut start: libc::c_int = 9999 as libc::c_int;
    let mut end: libc::c_int = -(9999 as libc::c_int);
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut desc: [libc::c_char; 1024] = [0; 1024];
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut fff: *mut FILE = 0 as *mut FILE;
    /* Note */
    strcpy(desc.as_mut_ptr(),
           b"It is a strange time.\x00" as *const u8 as *const libc::c_char);
    /* Format time of the day */
    strnfmt(buf2.as_mut_ptr(), 20 as libc::c_int as uint_hack,
            get_day(bst(11520 as libc::c_int * 365 as libc::c_int, turn) +
                        2890 as libc::c_int));
    /* Display current date in the Elvish calendar */
    msg_format(b"This is %s of the %s year of the third age.\x00" as *const u8
                   as *const libc::c_char,
               get_month_name(day, wizard, 0 as libc::c_int as bool_),
               buf2.as_mut_ptr());
    /* Message */
    msg_format(b"The time is %d:%02d %s.\x00" as *const u8 as
                   *const libc::c_char,
               if hour % 12 as libc::c_int == 0 as libc::c_int {
                   12 as libc::c_int
               } else { (hour) % 12 as libc::c_int }, min,
               if hour < 12 as libc::c_int {
                   b"AM\x00" as *const u8 as *const libc::c_char
               } else { b"PM\x00" as *const u8 as *const libc::c_char });
    /* Find the path */
    if Rand_div(10 as libc::c_int) == 0 || (*p_ptr).image as libc::c_int != 0
       {
        path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_FILE,
                   b"timefun.txt\x00" as *const u8 as *const libc::c_char);
    } else {
        path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_FILE,
                   b"timenorm.txt\x00" as *const u8 as *const libc::c_char);
    }
    /* Open this file */
    fff =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"rt\x00" as *const u8 as *const libc::c_char);
    /* Oops */
    if fff.is_null() { return }
    /* Find this time */
    while my_fgets(fff, buf.as_mut_ptr(), 1024 as libc::c_int as huge_hack) ==
              0 {
        /* Ignore comments */
        if buf[0 as libc::c_int as usize] == 0 ||
               buf[0 as libc::c_int as usize] as libc::c_int == '#' as i32 {
            continue ;
        }
        /* Ignore invalid lines */
        if buf[1 as libc::c_int as usize] as libc::c_int != ':' as i32 {
            continue ;
        }
        /* Process 'Start' */
        if buf[0 as libc::c_int as usize] as libc::c_int == 'S' as i32 {
            /* Extract the starting time */
            start = atoi(buf.as_mut_ptr().offset(2 as libc::c_int as isize));
            /* Assume valid for an hour */
            end = start + 59 as libc::c_int
        } else if buf[0 as libc::c_int as usize] as libc::c_int == 'E' as i32
         {
            /* Process 'End' */
            /* Extract the ending time */
            end = atoi(buf.as_mut_ptr().offset(2 as libc::c_int as isize))
        } else {
            /* Ignore incorrect range */
            if start > full || full > end { continue ; }
            /* Process 'Description' */
            if !(buf[0 as libc::c_int as usize] as libc::c_int == 'D' as i32)
               {
                continue ;
            }
            num += 1;
            /* Apply the randomizer */
            if Rand_div(num) == 0 {
                strcpy(desc.as_mut_ptr(),
                       buf.as_mut_ptr().offset(2 as libc::c_int as isize));
            }
        }
    }
    /* Message */
    msg_print(desc.as_mut_ptr() as cptr);
    /* Close the file */
    my_fclose(fff);
}
/*
 * Macro recorder!
 * It records all keypresses and then put them in a macro
 * Not as powerful as the macro screen, but much easier for newbies
 */
#[no_mangle]
pub static mut macro_recorder_current: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn macro_recorder_start() {
    msg_print(b"Starting macro recording, press this key again to stop. Note that if the action you want to record accepts the @ key, use it; it will remove your the need to inscribe stuff.\x00"
                  as *const u8 as *const libc::c_char);
    macro_recorder_current =
        memset(ralloc((1 as libc::c_int as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (1 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    *macro_recorder_current.offset(0 as libc::c_int as isize) =
        '\u{0}' as i32 as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn macro_recorder_add(mut c: libc::c_char) {
    let mut old_macro_recorder_current: *mut libc::c_char =
        macro_recorder_current;
    if macro_recorder_current.is_null() { return }
    macro_recorder_current =
        memset(ralloc(strlen(macro_recorder_current).wrapping_add(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong).wrapping_add(1
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                                                                                      as
                                                                                                                                      libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               strlen(macro_recorder_current).wrapping_add(1 as libc::c_int as
                                                               libc::c_ulong).wrapping_add(1
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                                                                               as
                                                                                                                               libc::c_ulong))
            as *mut libc::c_char;
    sprintf(macro_recorder_current,
            b"%s%c\x00" as *const u8 as *const libc::c_char,
            old_macro_recorder_current, c as libc::c_int);
    rnfree(old_macro_recorder_current as vptr,
           strlen(old_macro_recorder_current).wrapping_add(1 as libc::c_int as
                                                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                                               as
                                                                                               libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn macro_recorder_stop() {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut macro_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Ok we remove the last key, because it is the key to stop recording */
    *macro_recorder_current.offset(strlen(macro_recorder_current).wrapping_sub(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_ulong)
                                       as isize) =
        '\u{0}' as i32 as libc::c_char;
    /* Stop the recording */
    macro_0 = macro_recorder_current;
    macro_recorder_current = 0 as *mut libc::c_char;
    /* Add it */
    if get_check(b"Are you satisfied and want to create the macro? \x00" as
                     *const u8 as *const libc::c_char) != 0 {
        prt(b"Trigger: \x00" as *const u8 as *const libc::c_char,
            0 as libc::c_int, 0 as libc::c_int);
        /* Get a macro trigger */
        do_cmd_macro_aux(buf.as_mut_ptr(), 0 as libc::c_int as bool_);
        /* Link the macro */
        macro_add(buf.as_mut_ptr() as cptr, macro_0 as cptr);
        /* Prompt */
        str =
            memset(ralloc(strlen(macro_0).wrapping_add(1 as libc::c_int as
                                                           libc::c_ulong).wrapping_mul(3
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                                                                           as
                                                                                                                           libc::c_ulong))
                       as *mut libc::c_char as *mut libc::c_void,
                   0 as libc::c_int,
                   strlen(macro_0).wrapping_add(1 as libc::c_int as
                                                    libc::c_ulong).wrapping_mul(3
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                                                                    as
                                                                                                                    libc::c_ulong))
                as *mut libc::c_char;
        ascii_to_text(str, macro_0 as cptr);
        msg_format(b"Added a macro \'%s\'. If you want it to stay permanently, press @ now and dump macros to a file.\x00"
                       as *const u8 as *const libc::c_char, str);
        rnfree(str as vptr,
               strlen(macro_0).wrapping_add(1 as libc::c_int as
                                                libc::c_ulong).wrapping_mul(3
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                                                                as
                                                                                                                libc::c_ulong));
    }
    /* Ok now rid of useless stuff */
    rnfree(macro_0 as vptr,
           strlen(macro_0).wrapping_add(1 as libc::c_int as
                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                            as
                                                                            libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn do_cmd_macro_recorder() {
    if macro_recorder_current.is_null() {
        macro_recorder_start();
    } else { macro_recorder_stop(); };
}
