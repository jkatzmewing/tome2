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
    static mut player_exp: [s32b; 50];
    #[no_mangle]
    static mut stat_names: [cptr; 6];
    #[no_mangle]
    static mut stat_names_reduced: [cptr; 6];
    #[no_mangle]
    static mut window_flag_desc: [cptr; 32];
    #[no_mangle]
    static mut option_info: [option_type; 0];
    #[no_mangle]
    static mut bear_blows: [martial_arts; 8];
    #[no_mangle]
    static mut ma_blows: [martial_arts; 17];
    #[no_mangle]
    static mut move_info: [move_info_type; 9];
    #[no_mangle]
    static mut tactic_info: [tactic_info_type; 9];
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut alive: bool_;
    #[no_mangle]
    static mut death: bool_;
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut turn: s32b;
    #[no_mangle]
    static mut wizard: bool_;
    #[no_mangle]
    static mut total_winner: u16b;
    #[no_mangle]
    static mut noscore: u16b;
    #[no_mangle]
    static mut inven_cnt: s16b;
    #[no_mangle]
    static mut equip_cnt: s16b;
    #[no_mangle]
    static mut is_autosave: bool_;
    #[no_mangle]
    static mut text_out_indent: libc::c_int;
    #[no_mangle]
    static mut highscore_fd: libc::c_int;
    #[no_mangle]
    static mut stupid_monsters: bool_;
    #[no_mangle]
    static mut auto_scum: bool_;
    #[no_mangle]
    static mut smart_learn: bool_;
    #[no_mangle]
    static mut smart_cheat: bool_;
    #[no_mangle]
    static mut cheat_xtra: bool_;
    #[no_mangle]
    static mut small_levels: bool_;
    #[no_mangle]
    static mut empty_levels: bool_;
    #[no_mangle]
    static mut always_small_level: bool_;
    #[no_mangle]
    static mut hitpoint_warn: byte_hack;
    #[no_mangle]
    static mut autosave_l: bool_;
    #[no_mangle]
    static mut player_uid: libc::c_int;
    #[no_mangle]
    static mut player_name: [libc::c_char; 32];
    #[no_mangle]
    static mut player_base: [libc::c_char; 32];
    #[no_mangle]
    static mut died_from: [libc::c_char; 80];
    #[no_mangle]
    static mut history: [[libc::c_char; 60]; 4];
    #[no_mangle]
    static mut savefile: [libc::c_char; 1024];
    #[no_mangle]
    static mut view_n: s16b;
    #[no_mangle]
    static mut view_y: [byte_hack; 1536];
    #[no_mangle]
    static mut view_x: [byte_hack; 1536];
    #[no_mangle]
    static mut macro__buf: *mut libc::c_char;
    #[no_mangle]
    static mut window_flag: [u32b; 8];
    #[no_mangle]
    static mut angband_color_table: [[byte_hack; 4]; 256];
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut max_towns: u16b;
    #[no_mangle]
    static mut town_info: *mut town_type;
    #[no_mangle]
    static mut misc_to_attr: [byte_hack; 256];
    #[no_mangle]
    static mut misc_to_char: [libc::c_char; 256];
    #[no_mangle]
    static mut tval_to_attr: [byte_hack; 128];
    #[no_mangle]
    static mut keymap_act: [[cptr; 256]; 2];
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut sp_ptr: *mut player_sex;
    #[no_mangle]
    static mut rp_ptr: *mut player_race;
    #[no_mangle]
    static mut rmp_ptr: *mut player_race_mod;
    #[no_mangle]
    static mut cp_ptr: *mut player_class;
    #[no_mangle]
    static mut spp_ptr: *mut player_spec;
    #[no_mangle]
    static mut f_info: *mut feature_type;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut r_name: *mut libc::c_char;
    #[no_mangle]
    static mut re_info: *mut monster_ego;
    #[no_mangle]
    static mut d_info: *mut dungeon_info_type;
    #[no_mangle]
    static mut d_name: *mut libc::c_char;
    #[no_mangle]
    static mut class_info: *mut player_class;
    #[no_mangle]
    static mut c_name: *mut libc::c_char;
    #[no_mangle]
    static mut c_text: *mut libc::c_char;
    #[no_mangle]
    static mut race_info: *mut player_race;
    #[no_mangle]
    static mut rp_name: *mut libc::c_char;
    #[no_mangle]
    static mut race_mod_info: *mut player_race_mod;
    #[no_mangle]
    static mut rmp_name: *mut libc::c_char;
    #[no_mangle]
    static mut t_info: *mut trap_type;
    #[no_mangle]
    static mut wf_info: *mut wilderness_type_info;
    #[no_mangle]
    static mut wf_name: *mut libc::c_char;
    #[no_mangle]
    static mut wf_text: *mut libc::c_char;
    #[no_mangle]
    static mut st_info: *mut store_info_type;
    #[no_mangle]
    static mut st_name: *mut libc::c_char;
    #[no_mangle]
    static mut ANGBAND_SYS: cptr;
    #[no_mangle]
    static mut ANGBAND_KEYBOARD: cptr;
    #[no_mangle]
    static mut ANGBAND_GRAF: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_APEX: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_FILE: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_HELP: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_INFO: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_SAVE: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_PREF: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_USER: cptr;
    #[no_mangle]
    static mut item_tester_full: bool_;
    #[no_mangle]
    static mut wild_map: *mut *mut wilderness_map;
    #[no_mangle]
    static mut max_r_idx: u16b;
    #[no_mangle]
    static mut max_re_idx: u16b;
    #[no_mangle]
    static mut max_k_idx: u16b;
    #[no_mangle]
    static mut max_f_idx: u16b;
    #[no_mangle]
    static mut max_d_idx: u16b;
    #[no_mangle]
    static mut max_t_idx: u16b;
    #[no_mangle]
    static mut max_rp_idx: u16b;
    #[no_mangle]
    static mut max_rmp_idx: u16b;
    #[no_mangle]
    static mut max_st_idx: u16b;
    #[no_mangle]
    static mut max_wf_idx: u16b;
    #[no_mangle]
    static mut fates: [fate; 200];
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut max_dlv: *mut s16b;
    #[no_mangle]
    static mut total_bounties: u32b;
    #[no_mangle]
    static mut ironman_rooms: bool_;
    #[no_mangle]
    static mut take_notes: bool_;
    #[no_mangle]
    static mut joke_monsters: bool_;
    #[no_mangle]
    static mut max_q_idx: s16b;
    #[no_mangle]
    static mut quest: *mut quest_type;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn streq(s: cptr, t: cptr) -> bool_;
    #[no_mangle]
    fn prefix(s: cptr, t: cptr) -> bool_;
    #[no_mangle]
    fn quit(str: cptr);
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
    fn quit_fmt(fmt: cptr, _: ...);
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn maxroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn Term_xtra(n: libc::c_int, v: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_fresh() -> errr;
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
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
              _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn strftime(__s: *mut libc::c_char, __maxsize: size_t,
                __format: *const libc::c_char, __tp: *const tm) -> size_t;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    static mut game_module: cptr;
    #[no_mangle]
    static mut VERSION_MAJOR: s32b;
    #[no_mangle]
    static mut VERSION_MINOR: s32b;
    #[no_mangle]
    static mut VERSION_PATCH: s32b;
    #[no_mangle]
    static mut max_plev: s32b;
    #[no_mangle]
    static mut deity_info: *mut deity_type;
    #[no_mangle]
    fn get_version_string() -> *const libc::c_char;
    #[no_mangle]
    static mut hook_file: *mut FILE;
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn distance(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int,
                x2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn signal(__sig: libc::c_int, __handler: __sighandler_t)
     -> __sighandler_t;
    #[no_mangle]
    fn disturb(stop_search: libc::c_int, flush_output: libc::c_int);
    #[no_mangle]
    fn cmovie_clean_line(y: libc::c_int, abuf: *mut libc::c_char,
                         cbuf: *mut libc::c_char);
    #[no_mangle]
    fn cli_add(active: cptr, trigger: cptr, descr: cptr);
    #[no_mangle]
    fn my_fclose(fff: *mut FILE) -> errr;
    #[no_mangle]
    fn color_char_to_attr(c: libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn my_fopen(file: cptr, mode: cptr) -> *mut FILE;
    #[no_mangle]
    fn path_build(buf: *mut libc::c_char, max: libc::c_int, path: cptr,
                  file: cptr) -> errr;
    #[no_mangle]
    fn get_skill(skill: libc::c_int) -> s16b;
    #[no_mangle]
    fn monk_heavy_armor() -> bool_;
    #[no_mangle]
    fn fd_kill(file: cptr) -> errr;
    #[no_mangle]
    fn get_dungeon_save(buf: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn object_flags_known(o_ptr: *mut object_type, f1: *mut u32b,
                          f2: *mut u32b, f3: *mut u32b, f4: *mut u32b,
                          f5: *mut u32b, esp: *mut u32b);
    #[no_mangle]
    fn c_put_str(attr: byte_hack, str: cptr, row: libc::c_int,
                 col: libc::c_int);
    #[no_mangle]
    fn put_str(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn cnv_stat(i: libc::c_int, out_val: *mut libc::c_char);
    #[no_mangle]
    fn bst(what: s32b, t: s32b) -> s32b;
    #[no_mangle]
    fn get_player_race_name(pr: libc::c_int, ps: libc::c_int) -> cptr;
    #[no_mangle]
    fn clear_from(row: libc::c_int);
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn object_out_desc(o_ptr: *mut object_type, fff: *mut FILE,
                       trim_down: bool_, wait_for_it: bool_) -> bool_;
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn index_to_label(i: libc::c_int) -> libc::c_char;
    #[no_mangle]
    fn dump_fates(OutFile: *mut FILE);
    #[no_mangle]
    fn dump_companions(outfile: *mut FILE);
    #[no_mangle]
    fn dump_abilities(fff: *mut FILE);
    #[no_mangle]
    fn dump_skills(fff: *mut FILE);
    #[no_mangle]
    fn dump_corruptions(OutFile: *mut FILE, color: bool_);
    #[no_mangle]
    fn got_corruptions() -> bool_;
    #[no_mangle]
    fn self_knowledge(fff: *mut FILE);
    #[no_mangle]
    fn get_month_name(month: libc::c_int, full: bool_, compact: bool_)
     -> cptr;
    #[no_mangle]
    fn get_day(day: libc::c_int) -> cptr;
    #[no_mangle]
    fn call_lua(function: cptr, args: cptr, ret: cptr, _: ...) -> bool_;
    #[no_mangle]
    fn monster_race_desc(desc: *mut libc::c_char, r_idx: libc::c_int,
                         ego: libc::c_int);
    #[no_mangle]
    fn exec_lua(file: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn get_check(prompt: cptr) -> bool_;
    #[no_mangle]
    fn fd_close(fd: libc::c_int) -> errr;
    #[no_mangle]
    fn fd_open(file: cptr, flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn text_to_ascii(buf: *mut libc::c_char, str: cptr);
    #[no_mangle]
    fn macro_add(pat: cptr, act: cptr) -> errr;
    #[no_mangle]
    fn find_skill_i(name: cptr) -> s16b;
    #[no_mangle]
    fn my_fgets(fff: *mut FILE, buf: *mut libc::c_char, n: huge_hack) -> errr;
    #[no_mangle]
    fn file_exist(buf: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn string_exec_lua(file: *mut libc::c_char) -> cptr;
    #[no_mangle]
    fn tome_dofile_anywhere(dir: cptr, file: *mut libc::c_char,
                            test_exist: bool_) -> bool_;
    #[no_mangle]
    fn askfor_aux(buf: *mut libc::c_char, len: libc::c_int) -> bool_;
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn bell();
    #[no_mangle]
    fn screen_load();
    #[no_mangle]
    fn screen_save();
    #[no_mangle]
    fn move_cursor(row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn flush();
    #[no_mangle]
    fn save_player() -> bool_;
    #[no_mangle]
    fn handle_stuff();
    #[no_mangle]
    fn save_dungeon();
    #[no_mangle]
    fn object_value_real(o_ptr: *mut object_type) -> s32b;
    #[no_mangle]
    fn object_prep(o_ptr: *mut object_type, k_idx: libc::c_int);
    #[no_mangle]
    fn fd_read(fd: libc::c_int, buf: *mut libc::c_char, n: huge_hack) -> errr;
    #[no_mangle]
    fn fd_seek(fd: libc::c_int, n: huge_hack) -> errr;
    #[no_mangle]
    fn add_note_type(note_number: libc::c_int);
    #[no_mangle]
    fn fd_lock(fd: libc::c_int, what: libc::c_int) -> errr;
    #[no_mangle]
    fn fd_write(fd: libc::c_int, buf: cptr, n: huge_hack) -> errr;
    #[no_mangle]
    fn output_note(final_note: *mut libc::c_char);
    #[no_mangle]
    fn show_inven();
    #[no_mangle]
    fn show_equip();
    #[no_mangle]
    fn object_known(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_aware(o_ptr: *mut object_type);
    #[no_mangle]
    fn display_message(x: libc::c_int, y: libc::c_int, split: libc::c_int,
                       color: byte_hack, t: cptr);
    #[no_mangle]
    fn pause_line(row: libc::c_int);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
pub type vptr = *mut libc::c_void;
pub type cptr = *const libc::c_char;
pub type errr = libc::c_int;
pub type byte_hack = libc::c_uchar;
pub type bool_ = libc::c_char;
pub type uint_hack = libc::c_uint;
pub type huge_hack = libc::c_ulong;
pub type s16b = libc::c_short;
pub type u16b = libc::c_ushort;
pub type s32b = libc::c_int;
pub type u32b = libc::c_uint;
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
pub struct store_info_type {
    pub name: u32b,
    pub table: [[s16b; 2]; 56],
    pub table_num: byte_hack,
    pub max_obj: s16b,
    pub owners: [u16b; 4],
    pub actions: [u16b; 6],
    pub d_attr: byte_hack,
    pub d_char: libc::c_char,
    pub x_attr: byte_hack,
    pub x_char: libc::c_char,
    pub flags1: u32b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_sex {
    pub title: cptr,
    pub winner: cptr,
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
pub struct player_spec {
    pub title: s32b,
    pub desc: s32b,
    pub skill_basem: [libc::c_char; 200],
    pub skill_base: [u32b; 200],
    pub skill_modm: [libc::c_char; 200],
    pub skill_mod: [s16b; 200],
    pub skill_ideal: [u32b; 200],
    pub obj_tval: [s16b; 5],
    pub obj_sval: [s16b; 5],
    pub obj_pval: [s16b; 5],
    pub obj_dd: [s16b; 5],
    pub obj_ds: [s16b; 5],
    pub obj_num: s16b,
    pub gods: u32b,
    pub flags1: u32b,
    pub flags2: u32b,
    pub abilities: [C2RustUnnamed_2; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub ability: s16b,
    pub level: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_class {
    pub title: s32b,
    pub desc: s32b,
    pub titles: [s32b; 10],
    pub c_adj: [s16b; 6],
    pub c_dis: s16b,
    pub c_dev: s16b,
    pub c_sav: s16b,
    pub c_stl: s16b,
    pub c_srh: s16b,
    pub c_fos: s16b,
    pub c_thn: s16b,
    pub c_thb: s16b,
    pub x_dis: s16b,
    pub x_dev: s16b,
    pub x_sav: s16b,
    pub x_stl: s16b,
    pub x_srh: s16b,
    pub x_fos: s16b,
    pub x_thn: s16b,
    pub x_thb: s16b,
    pub c_mhp: s16b,
    pub c_exp: s16b,
    pub powers: [s16b; 4],
    pub spell_book: s16b,
    pub spell_stat: s16b,
    pub spell_lev: s16b,
    pub spell_fail: s16b,
    pub spell_mana: s16b,
    pub spell_first: s16b,
    pub spell_weight: s16b,
    pub max_spell_level: byte_hack,
    pub magic_max_spell: byte_hack,
    pub flags1: u32b,
    pub flags2: u32b,
    pub mana: s16b,
    pub blow_num: s16b,
    pub blow_wgt: s16b,
    pub blow_mul: s16b,
    pub extra_blows: s16b,
    pub sense_base: s32b,
    pub sense_pl: s32b,
    pub sense_plus: s32b,
    pub sense_heavy: byte_hack,
    pub sense_heavy_magic: byte_hack,
    pub obj_tval: [s16b; 5],
    pub obj_sval: [s16b; 5],
    pub obj_pval: [s16b; 5],
    pub obj_dd: [s16b; 5],
    pub obj_ds: [s16b; 5],
    pub obj_num: s16b,
    pub body_parts: [libc::c_char; 6],
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
    pub gods: u32b,
    pub spec: [player_spec; 20],
    pub abilities: [C2RustUnnamed_3; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
pub struct martial_arts {
    pub desc: cptr,
    pub min_level: libc::c_int,
    pub chance: libc::c_int,
    pub dd: libc::c_int,
    pub ds: libc::c_int,
    pub effect: s16b,
    pub power: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wilderness_type_info {
    pub name: u32b,
    pub text: u32b,
    pub entrance: u16b,
    pub wild_x: s32b,
    pub wild_y: s32b,
    pub road: byte_hack,
    pub level: libc::c_int,
    pub flags1: u32b,
    pub feat: byte_hack,
    pub terrain_idx: byte_hack,
    pub terrain: [byte_hack; 18],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wilderness_map {
    pub feat: libc::c_int,
    pub seed: u32b,
    pub entrance: u16b,
    pub known: bool_,
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
pub struct deity_type {
    pub name: cptr,
    pub desc: [[libc::c_char; 80]; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tactic_info_type {
    pub to_hit: s16b,
    pub to_dam: s16b,
    pub to_ac: s16b,
    pub to_stealth: s16b,
    pub to_disarm: s16b,
    pub to_saving: s16b,
    pub name: cptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fate {
    pub fate: byte_hack,
    pub level: byte_hack,
    pub serious: byte_hack,
    pub o_idx: s16b,
    pub e_idx: s16b,
    pub a_idx: s16b,
    pub v_idx: s16b,
    pub r_idx: s16b,
    pub count: s16b,
    pub time: s16b,
    pub know: bool_,
    pub icky: bool_,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct move_info_type {
    pub to_speed: s16b,
    pub to_search: s16b,
    pub to_stealth: s16b,
    pub to_percep: s16b,
    pub name: cptr,
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
pub type hyperlink_type = hyperlink;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hyperlink {
    pub path: [libc::c_char; 1024],
    pub rbuf: [libc::c_char; 1024],
    pub finder: [libc::c_char; 81],
    pub shower: [libc::c_char; 81],
    pub caption: [libc::c_char; 128],
    pub link: [[libc::c_char; 32]; 1024],
    pub link_key: [libc::c_char; 1024],
    pub link_x: [libc::c_int; 1024],
    pub link_y: [libc::c_int; 1024],
    pub link_line: [libc::c_int; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct high_score {
    pub what: [libc::c_char; 8],
    pub pts: [libc::c_char; 10],
    pub gold: [libc::c_char; 10],
    pub turns: [libc::c_char; 10],
    pub day: [libc::c_char; 10],
    pub who: [libc::c_char; 16],
    pub uid: [libc::c_char; 8],
    pub sex: [libc::c_char; 2],
    pub p_r: [libc::c_char; 3],
    pub p_s: [libc::c_char; 3],
    pub p_c: [libc::c_char; 3],
    pub p_cs: [libc::c_char; 3],
    pub cur_lev: [libc::c_char; 4],
    pub cur_dun: [libc::c_char; 4],
    pub max_lev: [libc::c_char; 4],
    pub max_dun: [libc::c_char; 4],
    pub arena_number: [libc::c_char; 4],
    pub inside_arena: [libc::c_char; 4],
    pub inside_quest: [libc::c_char; 4],
    pub exit_bldg: [libc::c_char; 4],
    pub how: [libc::c_char; 32],
}
/* File: files.c */
/* Purpose: code dealing with files (and death) */
/*
 * Copyright (c) 1989 James E. Wilson, Robert A. Koeneke
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Extract the first few "tokens" from a buffer
 *
 * This function uses "colon" and "slash" and delim arg as the delimeter characters.
 *
 * We never extract more than "num" tokens.  The "last" token may include
 * "delimeter" characters, allowing the buffer to include a "string" token.
 *
 * We save pointers to the tokens in "tokens", and return the number found.
 *
 * Hack -- Attempt to handle the 'c' character formalism
 *
 * Hack -- An empty buffer, or a final delimeter, yields an "empty" token.
 *
 * Hack -- We will always extract at least one token
 */
#[no_mangle]
pub unsafe extern "C" fn tokenize(mut buf: *mut libc::c_char, mut num: s16b,
                                  mut tokens: *mut *mut libc::c_char,
                                  mut delim1: libc::c_char,
                                  mut delim2: libc::c_char) -> s16b {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut s: *mut libc::c_char = buf;
    /* Process */
    while i < num as libc::c_int - 1 as libc::c_int {
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        /* Scan the string */
        t = s;
        while *t != 0 {
            /* Found a delimiter */
            if *t as libc::c_int == delim1 as libc::c_int ||
                   *t as libc::c_int == delim2 as libc::c_int {
                break ;
            }
            /* Handle single quotes */
            if *t as libc::c_int == '\'' as i32 {
                /* Advance */
                t = t.offset(1);
                /* Handle backslash */
                if *t as libc::c_int == '\\' as i32 { t = t.offset(1) }
                /* Require a character */
                if *t == 0 { break ; }
                /* Advance */
                t = t.offset(1);
                /* Hack -- Require a close quote */
                if *t as libc::c_int != '\'' as i32 {
                    *t = '\'' as i32 as libc::c_char
                }
            }
            /* Handle back-slash */
            if *t as libc::c_int == '\\' as i32 { t = t.offset(1) }
            t = t.offset(1)
        }
        /* Nothing left */
        if *t == 0 { break ; }
        /* Nuke and advance */
        let fresh0 = t;
        t = t.offset(1);
        *fresh0 = '\u{0}' as i32 as libc::c_char;
        /* Save the token */
        let fresh1 = i;
        i = i + 1;
        let ref mut fresh2 = *tokens.offset(fresh1 as isize);
        *fresh2 = s;
        /* Advance */
        s = t
    }
    /* Save the token */
    let fresh3 = i;
    i = i + 1;
    let ref mut fresh4 = *tokens.offset(fresh3 as isize);
    *fresh4 = s;
    /* Number found */
    return i as s16b;
}
/*
 * Parse a sub-file of the "extra info" (format shown below)
 *
 * Each "action" line has an "action symbol" in the first column,
 * followed by a colon, followed by some command specific info,
 * usually in the form of "tokens" separated by colons or slashes.
 *
 * Blank lines, lines starting with white space, and lines starting
 * with pound signs ("#") are ignored (as comments).
 *
 * Note the use of "tokenize()" to allow the use of both colons and
 * slashes as delimeters, while still allowing final tokens which
 * may contain any characters including "delimiters".
 *
 * Note the use of "strtol()" to allow all "integers" to be encoded
 * in decimal, hexidecimal, or octal form.
 *
 * Note that "monster zero" is used for the "player" attr/char, "object
 * zero" will be used for the "stack" attr/char, and "feature zero" is
 * used for the "nothing" attr/char.
 *
 * Parse another file recursively, see below for details
 *   %:<filename>
 *
 * Specify the attr/char values for "monsters" by race index
 *   R:<num>:<a>:<c>
 *
 * Specify the attr/char values for "objects" by kind index
 *   K:<num>:<a>:<c>
 *
 * Specify the attr/char values for "features" by feature index
 *   F:<num>:<a>:<c>
 *
 * Specify the attr/char values for "stores" by store index
 *   B:<num>:<a>:<c>
 *
 * Specify the attr/char values for unaware "objects" by kind tval
 *   U:<tv>:<a>:<c>
 *
 * Specify the attr/char values for inventory "objects" by kind tval
 *   E:<tv>:<a>:<c>
 *
 * Define a macro action, given an encoded macro action
 *   A:<str>
 *
 * Create a normal macro, given an encoded macro trigger
 *   P:<str>
 *
 * Create a command macro, given an encoded macro trigger
 *   C:<str>
 *
 * Create a keyset mapping
 *   S:<key>:<key>:<dir>
 *
 * Turn an option off, given its name
 *   X:<str>
 *
 * Turn an option on, given its name
 *   Y:<str>
 *
 * Specify visual information, given an index, and some data
 *   V:<num>:<kv>:<rv>:<gv>:<bv>
 *
 * Specify squelch settings
 *   Q:<num>:<squelch>
 */
#[no_mangle]
pub unsafe extern "C" fn process_pref_file_aux(mut buf: *mut libc::c_char)
 -> errr {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut zz: [*mut libc::c_char; 16] = [0 as *mut libc::c_char; 16];
    /* Skip "empty" lines */
    if *buf.offset(0 as libc::c_int as isize) == 0 { return 0 as libc::c_int }
    /* Skip "blank" lines */
    if *(*__ctype_b_loc()).offset(*buf.offset(0 as libc::c_int as isize) as
                                      libc::c_int as isize) as libc::c_int &
           _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        return 0 as libc::c_int
    }
    /* Skip comments */
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32 {
        return 0 as libc::c_int
    }
    /* Require "?:*" format */
    if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32 {
        return 1 as libc::c_int
    }
    /* Process "%:<fname>" */
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32 {
        /* Attempt to Process the given file */
        return process_pref_file(buf.offset(2 as libc::c_int as isize) as
                                     cptr)
    }
    /* Process "R:<num>:<a>/<c>" -- attr/char for monster races */
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'R' as i32 {
        if tokenize(buf.offset(2 as libc::c_int as isize),
                    3 as libc::c_int as s16b, zz.as_mut_ptr(),
                    ':' as i32 as libc::c_char, '/' as i32 as libc::c_char) as
               libc::c_int == 3 as libc::c_int {
            let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
            i =
                strtol(zz[0 as libc::c_int as usize],
                       0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    huge_hack as libc::c_int;
            n1 =
                strtol(zz[1 as libc::c_int as usize],
                       0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    libc::c_int;
            n2 =
                strtol(zz[2 as libc::c_int as usize],
                       0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    libc::c_int;
            if i >= max_r_idx as libc::c_int { return 1 as libc::c_int }
            r_ptr = &mut *r_info.offset(i as isize) as *mut monster_race;
            if n1 != 0 { (*r_ptr).x_attr = n1 as byte_hack }
            if n2 != 0 { (*r_ptr).x_char = n2 as libc::c_char }
            return 0 as libc::c_int
        }
    }
    /* Process "G:<type>:<num>:<a>/<c>" -- attr/char for overlay graphics */
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'G' as i32 {
        /* Process "G:M:<num>:<a>/<c>" -- attr/char for ego monsters */
        if *buf.offset(2 as libc::c_int as isize) as libc::c_int == 'M' as i32
           {
            if tokenize(buf.offset(4 as libc::c_int as isize),
                        3 as libc::c_int as s16b, zz.as_mut_ptr(),
                        ':' as i32 as libc::c_char,
                        '/' as i32 as libc::c_char) as libc::c_int ==
                   3 as libc::c_int {
                let mut re_ptr: *mut monster_ego = 0 as *mut monster_ego;
                i =
                    strtol(zz[0 as libc::c_int as usize],
                           0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                        huge_hack as libc::c_int;
                n1 =
                    strtol(zz[1 as libc::c_int as usize],
                           0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                        libc::c_int;
                n2 =
                    strtol(zz[2 as libc::c_int as usize],
                           0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                        libc::c_int;
                if i >= max_re_idx as libc::c_int { return 1 as libc::c_int }
                re_ptr = &mut *re_info.offset(i as isize) as *mut monster_ego;
                if n1 != 0 { (*re_ptr).g_attr = n1 as byte_hack }
                if n2 != 0 { (*re_ptr).g_char = n2 as libc::c_char }
                return 0 as libc::c_int
            }
        }
        /* Process "G:P:<num>:<a>/<c>" -- attr/char for race modifiers */
        if *buf.offset(2 as libc::c_int as isize) as libc::c_int == 'P' as i32
           {
            if tokenize(buf.offset(4 as libc::c_int as isize),
                        3 as libc::c_int as s16b, zz.as_mut_ptr(),
                        ':' as i32 as libc::c_char,
                        '/' as i32 as libc::c_char) as libc::c_int ==
                   3 as libc::c_int {
                let mut rmp_ptr_0: *mut player_race_mod =
                    0 as *mut player_race_mod;
                i =
                    strtol(zz[0 as libc::c_int as usize],
                           0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                        huge_hack as libc::c_int;
                n1 =
                    strtol(zz[1 as libc::c_int as usize],
                           0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                        libc::c_int;
                n2 =
                    strtol(zz[2 as libc::c_int as usize],
                           0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                        libc::c_int;
                if i >= max_rmp_idx as libc::c_int { return 1 as libc::c_int }
                rmp_ptr_0 =
                    &mut *race_mod_info.offset(i as isize) as
                        *mut player_race_mod;
                if n1 != 0 { (*rmp_ptr_0).g_attr = n1 as byte_hack }
                if n2 != 0 { (*rmp_ptr_0).g_char = n2 as libc::c_char }
                return 0 as libc::c_int
            }
        }
        /* Process "G:T:<num>:<a>/<c>" -- attr/char for traps */
        if *buf.offset(2 as libc::c_int as isize) as libc::c_int == 'T' as i32
           {
            if tokenize(buf.offset(4 as libc::c_int as isize),
                        3 as libc::c_int as s16b, zz.as_mut_ptr(),
                        ':' as i32 as libc::c_char,
                        '/' as i32 as libc::c_char) as libc::c_int ==
                   3 as libc::c_int {
                let mut t_ptr: *mut trap_type = 0 as *mut trap_type;
                i =
                    strtol(zz[0 as libc::c_int as usize],
                           0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                        huge_hack as libc::c_int;
                n1 =
                    strtol(zz[1 as libc::c_int as usize],
                           0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                        libc::c_int;
                n2 =
                    strtol(zz[2 as libc::c_int as usize],
                           0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                        libc::c_int;
                if i >= max_t_idx as libc::c_int { return 1 as libc::c_int }
                t_ptr = &mut *t_info.offset(i as isize) as *mut trap_type;
                if n1 != 0 { (*t_ptr).g_attr = n1 as byte_hack }
                if n2 != 0 { (*t_ptr).g_char = n2 as libc::c_char }
                return 0 as libc::c_int
            }
        }
    } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                  'K' as i32 {
        if tokenize(buf.offset(2 as libc::c_int as isize),
                    3 as libc::c_int as s16b, zz.as_mut_ptr(),
                    ':' as i32 as libc::c_char, '/' as i32 as libc::c_char) as
               libc::c_int == 3 as libc::c_int {
            let mut k_ptr: *mut object_kind = 0 as *mut object_kind;
            i =
                strtol(zz[0 as libc::c_int as usize],
                       0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    huge_hack as libc::c_int;
            n1 =
                strtol(zz[1 as libc::c_int as usize],
                       0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    libc::c_int;
            n2 =
                strtol(zz[2 as libc::c_int as usize],
                       0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    libc::c_int;
            if i >= max_k_idx as libc::c_int { return 1 as libc::c_int }
            k_ptr = &mut *k_info.offset(i as isize) as *mut object_kind;
            if n1 != 0 { (*k_ptr).x_attr = n1 as byte_hack }
            if n2 != 0 { (*k_ptr).x_char = n2 as libc::c_char }
            return 0 as libc::c_int
        }
    } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                  'F' as i32 {
        if tokenize(buf.offset(2 as libc::c_int as isize),
                    3 as libc::c_int as s16b, zz.as_mut_ptr(),
                    ':' as i32 as libc::c_char, '/' as i32 as libc::c_char) as
               libc::c_int == 3 as libc::c_int {
            let mut f_ptr: *mut feature_type = 0 as *mut feature_type;
            i =
                strtol(zz[0 as libc::c_int as usize],
                       0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    huge_hack as libc::c_int;
            n1 =
                strtol(zz[1 as libc::c_int as usize],
                       0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    libc::c_int;
            n2 =
                strtol(zz[2 as libc::c_int as usize],
                       0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    libc::c_int;
            if i >= max_f_idx as libc::c_int { return 1 as libc::c_int }
            f_ptr = &mut *f_info.offset(i as isize) as *mut feature_type;
            if n1 != 0 { (*f_ptr).x_attr = n1 as byte_hack }
            if n2 != 0 { (*f_ptr).x_char = n2 as libc::c_char }
            return 0 as libc::c_int
        }
    } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                  'B' as i32 {
        if tokenize(buf.offset(2 as libc::c_int as isize),
                    3 as libc::c_int as s16b, zz.as_mut_ptr(),
                    ':' as i32 as libc::c_char, '/' as i32 as libc::c_char) as
               libc::c_int == 3 as libc::c_int {
            let mut st_ptr: *mut store_info_type = 0 as *mut store_info_type;
            i =
                strtol(zz[0 as libc::c_int as usize],
                       0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    huge_hack as libc::c_int;
            n1 =
                strtol(zz[1 as libc::c_int as usize],
                       0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    libc::c_int;
            n2 =
                strtol(zz[2 as libc::c_int as usize],
                       0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    libc::c_int;
            if i >= max_st_idx as libc::c_int { return 1 as libc::c_int }
            st_ptr = &mut *st_info.offset(i as isize) as *mut store_info_type;
            if n1 != 0 { (*st_ptr).x_attr = n1 as byte_hack }
            if n2 != 0 { (*st_ptr).x_char = n2 as libc::c_char }
            return 0 as libc::c_int
        }
    } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                  'S' as i32 {
        if tokenize(buf.offset(2 as libc::c_int as isize),
                    3 as libc::c_int as s16b, zz.as_mut_ptr(),
                    ':' as i32 as libc::c_char, '/' as i32 as libc::c_char) as
               libc::c_int == 3 as libc::c_int {
            j =
                strtol(zz[0 as libc::c_int as usize],
                       0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    byte_hack as libc::c_int;
            n1 =
                strtol(zz[1 as libc::c_int as usize],
                       0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    libc::c_int;
            n2 =
                strtol(zz[2 as libc::c_int as usize],
                       0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    libc::c_int;
            misc_to_attr[j as usize] = n1 as byte_hack;
            misc_to_char[j as usize] = n2 as libc::c_char;
            return 0 as libc::c_int
        }
    } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                  'U' as i32 {
        if tokenize(buf.offset(2 as libc::c_int as isize),
                    3 as libc::c_int as s16b, zz.as_mut_ptr(),
                    ':' as i32 as libc::c_char, '/' as i32 as libc::c_char) as
               libc::c_int == 3 as libc::c_int {
            j =
                strtol(zz[0 as libc::c_int as usize],
                       0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    huge_hack as libc::c_int;
            n1 =
                strtol(zz[1 as libc::c_int as usize],
                       0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    libc::c_int;
            n2 =
                strtol(zz[2 as libc::c_int as usize],
                       0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    libc::c_int;
            i = 1 as libc::c_int;
            while i < max_k_idx as libc::c_int {
                let mut k_ptr_0: *mut object_kind =
                    &mut *k_info.offset(i as isize) as *mut object_kind;
                if (*k_ptr_0).tval as libc::c_int == j {
                    if n1 != 0 { (*k_ptr_0).d_attr = n1 as byte_hack }
                    if n2 != 0 { (*k_ptr_0).d_char = n2 as libc::c_char }
                }
                i += 1
            }
            return 0 as libc::c_int
        }
    } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                  'E' as i32 {
        if tokenize(buf.offset(2 as libc::c_int as isize),
                    2 as libc::c_int as s16b, zz.as_mut_ptr(),
                    ':' as i32 as libc::c_char, '/' as i32 as libc::c_char) as
               libc::c_int == 2 as libc::c_int {
            j =
                strtol(zz[0 as libc::c_int as usize],
                       0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    byte_hack as libc::c_int % 128 as libc::c_int;
            n1 =
                strtol(zz[1 as libc::c_int as usize],
                       0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                    libc::c_int;
            if n1 != 0 { tval_to_attr[j as usize] = n1 as byte_hack }
            return 0 as libc::c_int
        }
    } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                  'A' as i32 {
        text_to_ascii(macro__buf,
                      buf.offset(2 as libc::c_int as isize) as cptr);
        return 0 as libc::c_int
    } else {
        /* Process "K:<num>:<a>/<c>"  -- attr/char for object kinds */
        /* Process "F:<num>:<a>/<c>" -- attr/char for terrain features */
        /* Process "B:<num>:<a>/<c>" -- attr/char for stores */
        /* Process "S:<num>:<a>/<c>" -- attr/char for special things */
        /* Process "U:<tv>:<a>/<c>" -- attr/char for unaware items */
        /* Process "E:<tv>:<a>" -- attribute for inventory objects */
        /* Process "A:<str>" -- save an "action" for later */
        /* Process "P:<str>" -- normal macro */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'P' as i32
           {
            let mut tmp: [libc::c_char; 1024] = [0; 1024];
            text_to_ascii(tmp.as_mut_ptr(),
                          buf.offset(2 as libc::c_int as isize) as cptr);
            macro_add(tmp.as_mut_ptr() as cptr, macro__buf as cptr);
            return 0 as libc::c_int
        } else {
            /* Process "L:<num>:<trigger>:<descr> -- extended command macro */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   'L' as i32 {
                match tokenize(buf.offset(2 as libc::c_int as isize),
                               3 as libc::c_int as s16b, zz.as_mut_ptr(),
                               ':' as i32 as libc::c_char,
                               0 as libc::c_int as libc::c_char) as
                          libc::c_int {
                    3 => {
                        cli_add(zz[0 as libc::c_int as usize] as cptr,
                                zz[1 as libc::c_int as usize] as cptr,
                                zz[2 as libc::c_int as usize] as cptr);
                        return 0 as libc::c_int
                    }
                    2 => {
                        cli_add(zz[0 as libc::c_int as usize] as cptr,
                                zz[1 as libc::c_int as usize] as cptr,
                                0 as cptr);
                        return 0 as libc::c_int
                    }
                    _ => { return 1 as libc::c_int }
                }
            } else {
                /* Process "C:<str>" -- create keymap */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'C' as i32 {
                    let mut mode: libc::c_int = 0;
                    let mut tmp_0: [libc::c_char; 1024] = [0; 1024];
                    if tokenize(buf.offset(2 as libc::c_int as isize),
                                2 as libc::c_int as s16b, zz.as_mut_ptr(),
                                ':' as i32 as libc::c_char,
                                '/' as i32 as libc::c_char) as libc::c_int !=
                           2 as libc::c_int {
                        return 1 as libc::c_int
                    }
                    mode =
                        strtol(zz[0 as libc::c_int as usize],
                               0 as *mut *mut libc::c_char, 0 as libc::c_int)
                            as libc::c_int;
                    if mode < 0 as libc::c_int || mode >= 2 as libc::c_int {
                        return 1 as libc::c_int
                    }
                    text_to_ascii(tmp_0.as_mut_ptr(),
                                  zz[1 as libc::c_int as usize] as cptr);
                    if tmp_0[0 as libc::c_int as usize] == 0 ||
                           tmp_0[1 as libc::c_int as usize] as libc::c_int !=
                               0 {
                        return 1 as libc::c_int
                    }
                    i =
                        tmp_0[0 as libc::c_int as usize] as byte_hack as
                            libc::c_int;
                    string_free(keymap_act[mode as usize][i as usize]);
                    keymap_act[mode as usize][i as usize] =
                        string_make(macro__buf as cptr);
                    return 0 as libc::c_int
                } else {
                    /* Process "V:<num>:<kv>:<rv>:<gv>:<bv>" -- visual info */
                    if *buf.offset(0 as libc::c_int as isize) as libc::c_int
                           == 'V' as i32 {
                        if tokenize(buf.offset(2 as libc::c_int as isize),
                                    5 as libc::c_int as s16b, zz.as_mut_ptr(),
                                    ':' as i32 as libc::c_char,
                                    '/' as i32 as libc::c_char) as libc::c_int
                               == 5 as libc::c_int {
                            i =
                                strtol(zz[0 as libc::c_int as usize],
                                       0 as *mut *mut libc::c_char,
                                       0 as libc::c_int) as byte_hack as
                                    libc::c_int;
                            angband_color_table[i as
                                                    usize][0 as libc::c_int as
                                                               usize] =
                                strtol(zz[1 as libc::c_int as usize],
                                       0 as *mut *mut libc::c_char,
                                       0 as libc::c_int) as byte_hack;
                            angband_color_table[i as
                                                    usize][1 as libc::c_int as
                                                               usize] =
                                strtol(zz[2 as libc::c_int as usize],
                                       0 as *mut *mut libc::c_char,
                                       0 as libc::c_int) as byte_hack;
                            angband_color_table[i as
                                                    usize][2 as libc::c_int as
                                                               usize] =
                                strtol(zz[3 as libc::c_int as usize],
                                       0 as *mut *mut libc::c_char,
                                       0 as libc::c_int) as byte_hack;
                            angband_color_table[i as
                                                    usize][3 as libc::c_int as
                                                               usize] =
                                strtol(zz[4 as libc::c_int as usize],
                                       0 as *mut *mut libc::c_char,
                                       0 as libc::c_int) as byte_hack;
                            return 0 as libc::c_int
                        }
                    } else if *buf.offset(0 as libc::c_int as isize) as
                                  libc::c_int == 'T' as i32 {
                        let mut len: libc::c_int = 0;
                        let mut tok: libc::c_int = 0;
                        tok =
                            tokenize(buf.offset(2 as libc::c_int as isize),
                                     (2 as libc::c_int + 12 as libc::c_int) as
                                         s16b, zz.as_mut_ptr(),
                                     ':' as i32 as libc::c_char,
                                     '/' as i32 as libc::c_char) as
                                libc::c_int;
                        if tok >= 4 as libc::c_int {
                            let mut i_0: libc::c_int = 0;
                            let mut num: libc::c_int = 0;
                            if !macro_template.is_null() {
                                free(macro_template as *mut libc::c_void);
                                macro_template = 0 as *mut libc::c_char;
                                i_0 = 0 as libc::c_int;
                                while i_0 < max_macrotrigger {
                                    free(macro_trigger_name[i_0 as usize] as
                                             *mut libc::c_void);
                                    i_0 += 1
                                }
                                max_macrotrigger = 0 as libc::c_int
                            }
                            /* set macro trigger names and a template */
	/* Process "T:<trigger>:<keycode>:<shift-keycode>" */
	/* Process "T:<template>:<modifier chr>:<modifier name>:..." */
                            if *zz[0 as libc::c_int as usize] as libc::c_int
                                   == '\u{0}' as i32 {
                                return 0 as libc::c_int
                            } /* clear template */
                            num =
                                strlen(zz[1 as libc::c_int as usize]) as
                                    libc::c_int; /* error */
                            if 2 as libc::c_int + num != tok {
                                return 1 as libc::c_int
                            }
                            len =
                                strlen(zz[0 as libc::c_int as
                                              usize]).wrapping_add(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulong).wrapping_add(num
                                                                                                       as
                                                                                                       libc::c_ulong).wrapping_add(1
                                                                                                                                       as
                                                                                                                                       libc::c_int
                                                                                                                                       as
                                                                                                                                       libc::c_ulong)
                                    as libc::c_int;
                            i_0 = 0 as libc::c_int;
                            while i_0 < num {
                                len =
                                    (len as
                                         libc::c_ulong).wrapping_add(strlen(zz[(2
                                                                                    as
                                                                                    libc::c_int
                                                                                    +
                                                                                    i_0)
                                                                                   as
                                                                                   usize]).wrapping_add(1
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_ulong))
                                        as libc::c_int as libc::c_int;
                                i_0 += 1
                            }
                            macro_template =
                                malloc(len as libc::c_ulong) as
                                    *mut libc::c_char;
                            strcpy(macro_template,
                                   zz[0 as libc::c_int as usize]);
                            macro_modifier_chr =
                                macro_template.offset(strlen(macro_template)
                                                          as
                                                          isize).offset(1 as
                                                                            libc::c_int
                                                                            as
                                                                            isize);
                            strcpy(macro_modifier_chr,
                                   zz[1 as libc::c_int as usize]);
                            macro_modifier_name[0 as libc::c_int as usize] =
                                macro_modifier_chr.offset(strlen(macro_modifier_chr)
                                                              as
                                                              isize).offset(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize);
                            i_0 = 0 as libc::c_int;
                            while i_0 < num {
                                strcpy(macro_modifier_name[i_0 as usize],
                                       zz[(2 as libc::c_int + i_0) as usize]);
                                macro_modifier_name[(i_0 + 1 as libc::c_int)
                                                        as usize] =
                                    macro_modifier_name[i_0 as
                                                            usize].offset(strlen(macro_modifier_name[i_0
                                                                                                         as
                                                                                                         usize])
                                                                              as
                                                                              isize).offset(1
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                isize);
                                i_0 += 1
                            }
                        } else if tok >= 2 as libc::c_int {
                            let mut m: libc::c_int = 0;
                            let mut t: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut s: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            if max_macrotrigger >= 200 as libc::c_int {
                                msg_print(b"Too many macro triggers!\x00" as
                                              *const u8 as
                                              *const libc::c_char);
                                return 1 as libc::c_int
                            }
                            m = max_macrotrigger;
                            max_macrotrigger += 1;
                            len =
                                strlen(zz[0 as libc::c_int as
                                              usize]).wrapping_add(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulong).wrapping_add(strlen(zz[1
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 usize])).wrapping_add(1
                                                                                                                                           as
                                                                                                                                           libc::c_int
                                                                                                                                           as
                                                                                                                                           libc::c_ulong)
                                    as libc::c_int;
                            if tok == 3 as libc::c_int {
                                len =
                                    (len as
                                         libc::c_ulong).wrapping_add(strlen(zz[2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   usize]).wrapping_add(1
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_ulong))
                                        as libc::c_int as libc::c_int
                            }
                            macro_trigger_name[m as usize] =
                                malloc(len as libc::c_ulong) as
                                    *mut libc::c_char;
                            t = macro_trigger_name[m as usize];
                            s = zz[0 as libc::c_int as usize];
                            while *s != 0 {
                                if '\\' as i32 == *s as libc::c_int {
                                    s = s.offset(1)
                                }
                                let fresh5 = s;
                                s = s.offset(1);
                                let fresh6 = t;
                                t = t.offset(1);
                                *fresh6 = *fresh5
                            }
                            *t = '\u{0}' as i32 as libc::c_char;
                            macro_trigger_keycode[0 as libc::c_int as
                                                      usize][m as usize] =
                                macro_trigger_name[m as
                                                       usize].offset(strlen(macro_trigger_name[m
                                                                                                   as
                                                                                                   usize])
                                                                         as
                                                                         isize).offset(1
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize);
                            strcpy(macro_trigger_keycode[0 as libc::c_int as
                                                             usize][m as
                                                                        usize],
                                   zz[1 as libc::c_int as usize]);
                            if tok == 3 as libc::c_int {
                                macro_trigger_keycode[1 as libc::c_int as
                                                          usize][m as usize] =
                                    macro_trigger_keycode[0 as libc::c_int as
                                                              usize][m as
                                                                         usize].offset(strlen(macro_trigger_keycode[0
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        usize][m
                                                                                                                                   as
                                                                                                                                   usize])
                                                                                           as
                                                                                           isize).offset(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             isize);
                                strcpy(macro_trigger_keycode[1 as libc::c_int
                                                                 as
                                                                 usize][m as
                                                                            usize],
                                       zz[2 as libc::c_int as usize]);
                            } else {
                                macro_trigger_keycode[1 as libc::c_int as
                                                          usize][m as usize] =
                                    macro_trigger_keycode[0 as libc::c_int as
                                                              usize][m as
                                                                         usize]
                            }
                        }
                        return 0 as libc::c_int
                    } else {
                        /* Process "X:<str>" -- turn option off */
                        if *buf.offset(0 as libc::c_int as isize) as
                               libc::c_int == 'X' as i32 {
                            i = 0 as libc::c_int;
                            while !(*option_info.as_mut_ptr().offset(i as
                                                                         isize)).o_desc.is_null()
                                  {
                                if !(*option_info.as_mut_ptr().offset(i as
                                                                          isize)).o_var.is_null()
                                       &&
                                       !(*option_info.as_mut_ptr().offset(i as
                                                                              isize)).o_text.is_null()
                                       &&
                                       streq((*option_info.as_mut_ptr().offset(i
                                                                                   as
                                                                                   isize)).o_text,
                                             buf.offset(2 as libc::c_int as
                                                            isize) as cptr) as
                                           libc::c_int != 0 {
                                    *(*option_info.as_mut_ptr().offset(i as
                                                                           isize)).o_var
                                        = 0 as libc::c_int as bool_;
                                    return 0 as libc::c_int
                                }
                                i += 1
                            }
                        } else if *buf.offset(0 as libc::c_int as isize) as
                                      libc::c_int == 'Y' as i32 {
                            i = 0 as libc::c_int;
                            while !(*option_info.as_mut_ptr().offset(i as
                                                                         isize)).o_desc.is_null()
                                  {
                                if !(*option_info.as_mut_ptr().offset(i as
                                                                          isize)).o_var.is_null()
                                       &&
                                       !(*option_info.as_mut_ptr().offset(i as
                                                                              isize)).o_text.is_null()
                                       &&
                                       streq((*option_info.as_mut_ptr().offset(i
                                                                                   as
                                                                                   isize)).o_text,
                                             buf.offset(2 as libc::c_int as
                                                            isize) as cptr) as
                                           libc::c_int != 0 {
                                    *(*option_info.as_mut_ptr().offset(i as
                                                                           isize)).o_var
                                        = 1 as libc::c_int as bool_;
                                    return 0 as libc::c_int
                                }
                                i += 1
                            }
                        } else if *buf.offset(0 as libc::c_int as isize) as
                                      libc::c_int == 'W' as i32 {
                            let mut win: libc::c_int = 0;
                            let mut flag: libc::c_int = 0;
                            let mut value: libc::c_int = 0;
                            if tokenize(buf.offset(2 as libc::c_int as isize),
                                        3 as libc::c_int as s16b,
                                        zz.as_mut_ptr(),
                                        ':' as i32 as libc::c_char,
                                        '/' as i32 as libc::c_char) as
                                   libc::c_int == 3 as libc::c_int {
                                win =
                                    strtol(zz[0 as libc::c_int as usize],
                                           0 as *mut *mut libc::c_char,
                                           0 as libc::c_int) as libc::c_int;
                                flag =
                                    strtol(zz[1 as libc::c_int as usize],
                                           0 as *mut *mut libc::c_char,
                                           0 as libc::c_int) as libc::c_int;
                                value =
                                    strtol(zz[2 as libc::c_int as usize],
                                           0 as *mut *mut libc::c_char,
                                           0 as libc::c_int) as libc::c_int;
                                /* Process "Y:<str>" -- turn option on */
                                /* Process "W:<win>:<flag>:<value>" -- window flags */
                                /* Ignore illegal windows */
			/* Hack -- Ignore the main window */
                                if win <= 0 as libc::c_int ||
                                       win >= 8 as libc::c_int {
                                    return 1 as libc::c_int
                                }
                                /* Ignore illegal flags */
                                if flag < 0 as libc::c_int ||
                                       flag >= 32 as libc::c_int {
                                    return 1 as libc::c_int
                                }
                                /* Require a real flag */
                                if !window_flag_desc[flag as usize].is_null()
                                   {
                                    if value != 0 {
                                        /* Turn flag on */
                                        window_flag[win as usize] =
                                            (window_flag[win as usize] as
                                                 libc::c_long |
                                                 (1 as libc::c_long) << flag)
                                                as u32b
                                    } else {
                                        /* Turn flag off */
                                        window_flag[win as usize] =
                                            (window_flag[win as usize] as
                                                 libc::c_long &
                                                 !((1 as libc::c_long) <<
                                                       flag)) as u32b
                                    }
                                }
                                /* Success */
                                return 0 as libc::c_int
                            }
                        } else if *buf.offset(0 as libc::c_int as isize) as
                                      libc::c_int == 'Q' as i32 {
                            /*  Process "Q:<num>:<squelch>" -- item squelch flags */
                            /* This option isn't used anymore */
                            return 0 as libc::c_int
                        }
                    }
                }
            }
        }
    }
    /* Failure */
    return 1 as libc::c_int;
}
/*
 * Helper function for "process_pref_file()"
 *
 * Input:
 *   v: output buffer array
 *   f: final character
 *
 * Output:
 *   result
 */
unsafe extern "C" fn process_pref_file_expr(mut sp: *mut *mut libc::c_char,
                                            mut fp: *mut libc::c_char)
 -> cptr {
    let mut v: cptr = 0 as *const libc::c_char;
    let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b1: libc::c_char = '[' as i32 as libc::c_char;
    let mut b2: libc::c_char = ']' as i32 as libc::c_char;
    let mut f: libc::c_char = ' ' as i32 as libc::c_char;
    /* Initial */
    s = *sp;
    /* Skip spaces */
    while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as
              libc::c_int &
              _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        s = s.offset(1)
    }
    /* Save start */
    b = s;
    /* Default */
    v = b"?o?o?\x00" as *const u8 as *const libc::c_char;
    /* Analyze */
    if *s as libc::c_int == b1 as libc::c_int {
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        let mut t: *const libc::c_char = 0 as *const libc::c_char;
        /* Skip b1 */
        s = s.offset(1);
        /* First */
        t = process_pref_file_expr(&mut s, &mut f);
        /* Oops */
        if !(*t == 0) {
            /* Function: IOR */
            if streq(t, b"IOR\x00" as *const u8 as *const libc::c_char) != 0 {
                v = b"0\x00" as *const u8 as *const libc::c_char;
                while *s as libc::c_int != 0 &&
                          f as libc::c_int != b2 as libc::c_int {
                    t = process_pref_file_expr(&mut s, &mut f);
                    if *t as libc::c_int != 0 &&
                           streq(t,
                                 b"0\x00" as *const u8 as *const libc::c_char)
                               == 0 {
                        v = b"1\x00" as *const u8 as *const libc::c_char
                    }
                }
            } else if streq(t, b"AND\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                v = b"1\x00" as *const u8 as *const libc::c_char;
                while *s as libc::c_int != 0 &&
                          f as libc::c_int != b2 as libc::c_int {
                    t = process_pref_file_expr(&mut s, &mut f);
                    if *t as libc::c_int != 0 &&
                           streq(t,
                                 b"0\x00" as *const u8 as *const libc::c_char)
                               as libc::c_int != 0 {
                        v = b"0\x00" as *const u8 as *const libc::c_char
                    }
                }
            } else if streq(t, b"NOT\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                v = b"1\x00" as *const u8 as *const libc::c_char;
                while *s as libc::c_int != 0 &&
                          f as libc::c_int != b2 as libc::c_int {
                    t = process_pref_file_expr(&mut s, &mut f);
                    if *t as libc::c_int != 0 &&
                           streq(t,
                                 b"0\x00" as *const u8 as *const libc::c_char)
                               == 0 {
                        v = b"0\x00" as *const u8 as *const libc::c_char
                    }
                }
            } else if streq(t, b"EQU\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                v = b"1\x00" as *const u8 as *const libc::c_char;
                if *s as libc::c_int != 0 &&
                       f as libc::c_int != b2 as libc::c_int {
                    t = process_pref_file_expr(&mut s, &mut f)
                }
                while *s as libc::c_int != 0 &&
                          f as libc::c_int != b2 as libc::c_int {
                    p = t;
                    t = process_pref_file_expr(&mut s, &mut f);
                    if *t as libc::c_int != 0 && streq(p, t) == 0 {
                        v = b"0\x00" as *const u8 as *const libc::c_char
                    }
                }
            } else if streq(t, b"LEQ\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                v = b"1\x00" as *const u8 as *const libc::c_char;
                if *s as libc::c_int != 0 &&
                       f as libc::c_int != b2 as libc::c_int {
                    t = process_pref_file_expr(&mut s, &mut f)
                }
                while *s as libc::c_int != 0 &&
                          f as libc::c_int != b2 as libc::c_int {
                    p = t;
                    t = process_pref_file_expr(&mut s, &mut f);
                    if *t as libc::c_int != 0 &&
                           strcmp(p, t) > 0 as libc::c_int {
                        v = b"0\x00" as *const u8 as *const libc::c_char
                    }
                }
            } else if streq(t, b"GEQ\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                v = b"1\x00" as *const u8 as *const libc::c_char;
                if *s as libc::c_int != 0 &&
                       f as libc::c_int != b2 as libc::c_int {
                    t = process_pref_file_expr(&mut s, &mut f)
                }
                while *s as libc::c_int != 0 &&
                          f as libc::c_int != b2 as libc::c_int {
                    p = t;
                    t = process_pref_file_expr(&mut s, &mut f);
                    if *t as libc::c_int != 0 &&
                           strcmp(p, t) < 0 as libc::c_int {
                        v = b"0\x00" as *const u8 as *const libc::c_char
                    }
                }
            } else if streq(t,
                            b"LEQN\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                let mut n: libc::c_int = 0 as libc::c_int;
                v = b"1\x00" as *const u8 as *const libc::c_char;
                if *s as libc::c_int != 0 &&
                       f as libc::c_int != b2 as libc::c_int {
                    t = process_pref_file_expr(&mut s, &mut f);
                    n = atoi(t)
                }
                while *s as libc::c_int != 0 &&
                          f as libc::c_int != b2 as libc::c_int {
                    p = t;
                    t = process_pref_file_expr(&mut s, &mut f);
                    if *t as libc::c_int != 0 && atoi(t) < n {
                        v = b"0\x00" as *const u8 as *const libc::c_char
                    }
                }
            } else if streq(t,
                            b"GEQN\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                let mut n_0: libc::c_int = 0 as libc::c_int;
                v = b"1\x00" as *const u8 as *const libc::c_char;
                if *s as libc::c_int != 0 &&
                       f as libc::c_int != b2 as libc::c_int {
                    t = process_pref_file_expr(&mut s, &mut f);
                    n_0 = atoi(t)
                }
                while *s as libc::c_int != 0 &&
                          f as libc::c_int != b2 as libc::c_int {
                    p = t;
                    t = process_pref_file_expr(&mut s, &mut f);
                    if *t as libc::c_int != 0 && atoi(t) > n_0 {
                        v = b"0\x00" as *const u8 as *const libc::c_char
                    }
                }
            } else if streq(t,
                            b"SKILL\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                static mut skill_val: [libc::c_char; 17] = [0; 17];
                let mut skill: s16b = -(1 as libc::c_int) as s16b;
                v = b"0\x00" as *const u8 as *const libc::c_char;
                while *s as libc::c_int != 0 &&
                          f as libc::c_int != b2 as libc::c_int {
                    t = process_pref_file_expr(&mut s, &mut f);
                    if *t != 0 { skill = find_skill_i(t) }
                }
                if skill as libc::c_int > 0 as libc::c_int {
                    sprintf(skill_val.as_mut_ptr(),
                            b"%d\x00" as *const u8 as *const libc::c_char,
                            get_skill(skill as libc::c_int) as libc::c_int);
                    v = skill_val.as_mut_ptr() as cptr
                }
            } else {
                /* Function: AND */
                /* Function: NOT */
                /* Function: EQU */
                /* Function: LEQ */
                /* Function: GEQ */
                /* Function: LEQN */
                /* Function: GEQN */
                /* Function SKILL */
                /* Oops */
                while *s as libc::c_int != 0 &&
                          f as libc::c_int != b2 as libc::c_int {
                    t = process_pref_file_expr(&mut s, &mut f)
                }
            }
        }
        /* Verify ending */
        if f as libc::c_int != b2 as libc::c_int {
            v = b"?x?x?\x00" as *const u8 as *const libc::c_char
        }
        /* Extract final and Terminate */
        f = *s;
        if f as libc::c_int != '\u{0}' as i32 {
            let fresh7 = s;
            s = s.offset(1);
            *fresh7 = '\u{0}' as i32 as libc::c_char
        }
    } else {
        /* Other */
        /* Accept all printables except spaces and brackets */
        while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as
                  libc::c_int &
                  _ISprint as libc::c_int as libc::c_ushort as libc::c_int !=
                  0 &&
                  strchr(b" []\x00" as *const u8 as *const libc::c_char,
                         *s as libc::c_int).is_null() {
            s = s.offset(1)
        }
        f = *s;
        if f as libc::c_int != '\u{0}' as i32 {
            let fresh8 = s;
            s = s.offset(1);
            *fresh8 = '\u{0}' as i32 as libc::c_char
        }
        if *b as libc::c_int == '$' as i32 {
            /* Extract final and Terminate */
            /* System */
            if streq(b.offset(1 as libc::c_int as isize) as cptr,
                     b"SYS\x00" as *const u8 as *const libc::c_char) != 0 {
                v = ANGBAND_SYS
            } else if streq(b.offset(1 as libc::c_int as isize) as cptr,
                            b"KEYBOARD\x00" as *const u8 as
                                *const libc::c_char) != 0 {
                v = ANGBAND_KEYBOARD
            }
            /* Graphics */
            if streq(b.offset(1 as libc::c_int as isize) as cptr,
                     b"GRAF\x00" as *const u8 as *const libc::c_char) != 0 {
                v = ANGBAND_GRAF
            } else if streq(b.offset(1 as libc::c_int as isize) as cptr,
                            b"RACE\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                v = rp_name.offset((*rp_ptr).title as isize) as cptr
            } else if streq(b.offset(1 as libc::c_int as isize) as cptr,
                            b"RACEMOD\x00" as *const u8 as
                                *const libc::c_char) != 0 {
                v = rmp_name.offset((*rmp_ptr).title as isize) as cptr
            } else if streq(b.offset(1 as libc::c_int as isize) as cptr,
                            b"CLASS\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                v = c_name.offset((*spp_ptr).title as isize) as cptr
            } else if streq(b.offset(1 as libc::c_int as isize) as cptr,
                            b"PLAYER\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                v = player_base.as_mut_ptr() as cptr
            }
        } else {
            /* Race */
            /* Race */
            /* Class */
            /* Player */
            /* Constant */
            v = b as cptr
        }
    }
    /* Variable */
    /* Save */
    *fp = f;
    /* Save */
    *sp = s;
    /* Result */
    return v;
}
/*
 * Process the "user pref file" with the given name
 *
 * See the function above for a list of legal "commands".
 *
 * We also accept the special "?" and "%" directives, which
 * allow conditional evaluation and filename inclusion.
 */
#[no_mangle]
pub unsafe extern "C" fn process_pref_file(mut name: cptr) -> errr {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut num: libc::c_int = -(1 as libc::c_int);
    let mut err: errr = 0 as libc::c_int;
    let mut bypass: bool_ = 0 as libc::c_int as bool_;
    /* Build the filename -- Allow users to override system pref files */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_USER, name);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* No such file -- Try system pref file */
    if fp.is_null() {
        /* Build the pathname, this time using the system pref directory */
        path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_PREF,
                   name);
        /* Open the file */
        fp =
            my_fopen(buf.as_mut_ptr() as cptr,
                     b"r\x00" as *const u8 as *const libc::c_char);
        /* Failed again */
        if fp.is_null() { return -(1 as libc::c_int) }
    }
    /* Process the file */
    while 0 as libc::c_int ==
              my_fgets(fp, buf.as_mut_ptr(), 1024 as libc::c_int as huge_hack)
          {
        /* Count lines */
        num += 1;
        /* Skip "empty" lines */
        if buf[0 as libc::c_int as usize] == 0 { continue ; }
        /* Skip "blank" lines */
        if *(*__ctype_b_loc()).offset(buf[0 as libc::c_int as usize] as
                                          libc::c_int as isize) as libc::c_int
               & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
           {
            continue ;
        }
        /* Skip comments */
        if buf[0 as libc::c_int as usize] as libc::c_int == '#' as i32 {
            continue ;
        }
        /* Process "?:<expr>" */
        if buf[0 as libc::c_int as usize] as libc::c_int == '?' as i32 &&
               buf[1 as libc::c_int as usize] as libc::c_int == ':' as i32 {
            let mut f: libc::c_char = 0;
            let mut v: cptr = 0 as *const libc::c_char;
            let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
            /* Start */
            s = buf.as_mut_ptr().offset(2 as libc::c_int as isize);
            /* Parse the expr */
            v = process_pref_file_expr(&mut s, &mut f);
            /* Set flag */
            bypass =
                if streq(v, b"0\x00" as *const u8 as *const libc::c_char) as
                       libc::c_int != 0 {
                    1 as libc::c_int
                } else { 0 as libc::c_int } as bool_
        } else {
            /* Apply conditionals */
            if bypass != 0 { continue ; }
            /* Process "%:<file>" */
            if buf[0 as libc::c_int as usize] as libc::c_int == '%' as i32 {
                /* Process that file if allowed */
                process_pref_file(buf.as_mut_ptr().offset(2 as libc::c_int as
                                                              isize) as cptr);
            } else {
                /* Process the line */
                err = process_pref_file_aux(buf.as_mut_ptr());
                /* Oops */
                if err != 0 { break ; }
            }
        }
    }
    /* Error */
    if err != 0 {
        /* Useful error message */
        msg_format(b"Error %d in line %d of file \'%s\'.\x00" as *const u8 as
                       *const libc::c_char, err, num, name);
        msg_format(b"Parsing \'%s\'\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
    }
    /* Close the file */
    my_fclose(fp);
    /* Result */
    return err;
}
/*
 * Print long number with header at given row, column
 * Use the color for the number, not the header
 */
unsafe extern "C" fn prt_lnum(mut header: cptr, mut num: s32b,
                              mut row: libc::c_int, mut col: libc::c_int,
                              mut color: byte_hack) {
    let mut len: libc::c_int = strlen(header) as libc::c_int;
    let mut out_val: [libc::c_char; 32] = [0; 32];
    put_str(header, row, col);
    sprintf(out_val.as_mut_ptr(),
            b"%9ld\x00" as *const u8 as *const libc::c_char,
            num as libc::c_long);
    c_put_str(color, out_val.as_mut_ptr() as cptr, row, col + len);
}
/*
 * Print number with header at given row, column
 */
unsafe extern "C" fn prt_num(mut header: cptr, mut num: libc::c_int,
                             mut row: libc::c_int, mut col: libc::c_int,
                             mut color: byte_hack,
                             mut space: *mut libc::c_char) {
    let mut len: libc::c_int = strlen(header) as libc::c_int;
    let mut out_val: [libc::c_char; 32] = [0; 32];
    put_str(header, row, col);
    put_str(space as cptr, row, col + len);
    sprintf(out_val.as_mut_ptr(),
            b"%6ld\x00" as *const u8 as *const libc::c_char,
            num as libc::c_long);
    c_put_str(color, out_val.as_mut_ptr() as cptr, row,
              ((col + len) as libc::c_ulong).wrapping_add(strlen(space)) as
                  libc::c_int);
}
/*
 * Print str with header at given row, column
 */
unsafe extern "C" fn prt_str(mut header: cptr, mut str: cptr,
                             mut row: libc::c_int, mut col: libc::c_int,
                             mut color: byte_hack) {
    let mut len: libc::c_int = strlen(header) as libc::c_int;
    let mut out_val: [libc::c_char; 32] = [0; 32];
    put_str(header, row, col);
    put_str(b"   \x00" as *const u8 as *const libc::c_char, row, col + len);
    sprintf(out_val.as_mut_ptr(),
            b"%6s\x00" as *const u8 as *const libc::c_char, str);
    c_put_str(color, out_val.as_mut_ptr() as cptr, row,
              col + len + 3 as libc::c_int);
}
/*
 * Prints the following information on the screen.
 *
 * For this to look right, the following should be spaced the
 * same as in the prt_lnum code... -CFT
 */
unsafe extern "C" fn display_player_middle() {
    let mut show_tohit: libc::c_int = (*p_ptr).dis_to_h as libc::c_int;
    let mut show_todam: libc::c_int = (*p_ptr).dis_to_d as libc::c_int;
    let mut o_ptr: *mut object_type =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(24 as libc::c_int as
                                                         isize) as
            *mut object_type;
    let mut num: [libc::c_char; 7] = [0; 7];
    let mut color: byte_hack = 0;
    let mut speed: libc::c_int = 0;
    /* Hack -- add in weapon info if known */
    if (*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
           (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as libc::c_int
               != 0 &&
               (*k_info.offset((*o_ptr).k_idx as isize)).aware as libc::c_int
                   != 0 {
        show_tohit =
            (*p_ptr).dis_to_h as libc::c_int +
                (*p_ptr).to_h_melee as libc::c_int +
                (*o_ptr).to_h as libc::c_int
    } else {
        show_tohit =
            (*p_ptr).dis_to_h as libc::c_int +
                (*p_ptr).to_h_melee as libc::c_int
    }
    if (*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
           (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as libc::c_int
               != 0 &&
               (*k_info.offset((*o_ptr).k_idx as isize)).aware as libc::c_int
                   != 0 {
        show_todam =
            (*p_ptr).dis_to_d as libc::c_int +
                (*p_ptr).to_d_melee as libc::c_int +
                (*o_ptr).to_d as libc::c_int
    } else {
        show_todam =
            (*p_ptr).dis_to_d as libc::c_int +
                (*p_ptr).to_d_melee as libc::c_int
    }
    /* Dump the bonuses to hit/dam */
    prt_num(b"+ To Melee Hit   \x00" as *const u8 as *const libc::c_char,
            show_tohit, 9 as libc::c_int, 1 as libc::c_int,
            14 as libc::c_int as byte_hack,
            b"   \x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char);
    prt_num(b"+ To Melee Damage\x00" as *const u8 as *const libc::c_char,
            show_todam, 10 as libc::c_int, 1 as libc::c_int,
            14 as libc::c_int as byte_hack,
            b"   \x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char);
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(27 as libc::c_int as
                                                         isize) as
            *mut object_type;
    /* Hack -- add in weapon info if known */
    if (*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
           (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as libc::c_int
               != 0 &&
               (*k_info.offset((*o_ptr).k_idx as isize)).aware as libc::c_int
                   != 0 {
        show_tohit =
            (*p_ptr).dis_to_h as libc::c_int +
                (*p_ptr).to_h_ranged as libc::c_int +
                (*o_ptr).to_h as libc::c_int
    } else {
        show_tohit =
            (*p_ptr).dis_to_h as libc::c_int +
                (*p_ptr).to_h_ranged as libc::c_int
    }
    if (*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
           (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as libc::c_int
               != 0 &&
               (*k_info.offset((*o_ptr).k_idx as isize)).aware as libc::c_int
                   != 0 {
        show_todam =
            (*p_ptr).to_d_ranged as libc::c_int + (*o_ptr).to_d as libc::c_int
    } else { show_todam = (*p_ptr).to_d_ranged as libc::c_int }
    prt_num(b"+ To Ranged Hit   \x00" as *const u8 as *const libc::c_char,
            show_tohit, 11 as libc::c_int, 1 as libc::c_int,
            14 as libc::c_int as byte_hack,
            b"  \x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char);
    prt_num(b"+ To Ranged Damage\x00" as *const u8 as *const libc::c_char,
            show_todam, 12 as libc::c_int, 1 as libc::c_int,
            14 as libc::c_int as byte_hack,
            b"  \x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char);
    /* Dump the total armor class */
    prt_str(b"  AC             \x00" as *const u8 as *const libc::c_char,
            format(b"%d+%d\x00" as *const u8 as *const libc::c_char,
                   (*p_ptr).ac as libc::c_int,
                   (*p_ptr).dis_to_a as libc::c_int) as cptr,
            13 as libc::c_int, 1 as libc::c_int,
            14 as libc::c_int as byte_hack);
    prt_num(b"Level      \x00" as *const u8 as *const libc::c_char,
            (*p_ptr).lev as libc::c_int, 9 as libc::c_int, 28 as libc::c_int,
            13 as libc::c_int as byte_hack,
            b"   \x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char);
    if (*p_ptr).exp >= (*p_ptr).max_exp {
        prt_lnum(b"Experience \x00" as *const u8 as *const libc::c_char,
                 (*p_ptr).exp, 10 as libc::c_int, 28 as libc::c_int,
                 13 as libc::c_int as byte_hack);
    } else {
        prt_lnum(b"Experience \x00" as *const u8 as *const libc::c_char,
                 (*p_ptr).exp, 10 as libc::c_int, 28 as libc::c_int,
                 11 as libc::c_int as byte_hack);
    }
    prt_lnum(b"Max Exp    \x00" as *const u8 as *const libc::c_char,
             (*p_ptr).max_exp, 11 as libc::c_int, 28 as libc::c_int,
             13 as libc::c_int as byte_hack);
    if (*p_ptr).lev as libc::c_int >= 50 as libc::c_int ||
           (*p_ptr).lev as libc::c_int >= max_plev {
        put_str(b"Exp to Adv.\x00" as *const u8 as *const libc::c_char,
                12 as libc::c_int, 28 as libc::c_int);
        c_put_str(13 as libc::c_int as byte_hack,
                  b"    *****\x00" as *const u8 as *const libc::c_char,
                  12 as libc::c_int, 28 as libc::c_int + 11 as libc::c_int);
    } else {
        prt_lnum(b"Exp to Adv.\x00" as *const u8 as *const libc::c_char,
                 ((player_exp[((*p_ptr).lev as libc::c_int - 1 as libc::c_int)
                                  as usize] * (*p_ptr).expfact as libc::c_int)
                      as libc::c_long / 100 as libc::c_long) as s32b,
                 12 as libc::c_int, 28 as libc::c_int,
                 13 as libc::c_int as byte_hack);
    }
    prt_lnum(b"Gold       \x00" as *const u8 as *const libc::c_char,
             (*p_ptr).au, 13 as libc::c_int, 28 as libc::c_int,
             13 as libc::c_int as byte_hack);
    if (*p_ptr).necro_extra & 0x8 as libc::c_int as libc::c_uint != 0 {
        put_str(b"Death Points \x00" as *const u8 as *const libc::c_char,
                9 as libc::c_int, 52 as libc::c_int);
        if (*p_ptr).chp as libc::c_int >= (*p_ptr).mhp as libc::c_int {
            color = 14 as libc::c_int as byte_hack
        } else if (*p_ptr).chp as libc::c_int >
                      (*p_ptr).mhp as libc::c_int *
                          hitpoint_warn as libc::c_int / 10 as libc::c_int {
            color = 10 as libc::c_int as byte_hack
        } else { color = 12 as libc::c_int as byte_hack }
        sprintf(num.as_mut_ptr(),
                b"%6ld\x00" as *const u8 as *const libc::c_char,
                (*p_ptr).chp as libc::c_long);
        c_put_str(color, num.as_mut_ptr() as cptr, 9 as libc::c_int,
                  65 as libc::c_int);
        put_str(b"/\x00" as *const u8 as *const libc::c_char,
                9 as libc::c_int, 71 as libc::c_int);
        sprintf(num.as_mut_ptr(),
                b"%6ld\x00" as *const u8 as *const libc::c_char,
                (*p_ptr).mhp as libc::c_long);
        c_put_str(14 as libc::c_int as byte_hack, num.as_mut_ptr() as cptr,
                  9 as libc::c_int, 72 as libc::c_int);
    } else {
        put_str(b"Hit Points   \x00" as *const u8 as *const libc::c_char,
                9 as libc::c_int, 52 as libc::c_int);
        if (*p_ptr).chp as libc::c_int >= (*p_ptr).mhp as libc::c_int {
            color = 13 as libc::c_int as byte_hack
        } else if (*p_ptr).chp as libc::c_int >
                      (*p_ptr).mhp as libc::c_int *
                          hitpoint_warn as libc::c_int / 10 as libc::c_int {
            color = 11 as libc::c_int as byte_hack
        } else { color = 4 as libc::c_int as byte_hack }
        sprintf(num.as_mut_ptr(),
                b"%6ld\x00" as *const u8 as *const libc::c_char,
                (*p_ptr).chp as libc::c_long);
        c_put_str(color, num.as_mut_ptr() as cptr, 9 as libc::c_int,
                  65 as libc::c_int);
        put_str(b"/\x00" as *const u8 as *const libc::c_char,
                9 as libc::c_int, 71 as libc::c_int);
        sprintf(num.as_mut_ptr(),
                b"%6ld\x00" as *const u8 as *const libc::c_char,
                (*p_ptr).mhp as libc::c_long);
        c_put_str(13 as libc::c_int as byte_hack, num.as_mut_ptr() as cptr,
                  9 as libc::c_int, 72 as libc::c_int);
    }
    put_str(b"Spell Points \x00" as *const u8 as *const libc::c_char,
            10 as libc::c_int, 52 as libc::c_int);
    if (*p_ptr).csp as libc::c_int >= (*p_ptr).msp as libc::c_int {
        color = 13 as libc::c_int as byte_hack
    } else if (*p_ptr).csp as libc::c_int >
                  (*p_ptr).msp as libc::c_int * hitpoint_warn as libc::c_int /
                      10 as libc::c_int {
        color = 11 as libc::c_int as byte_hack
    } else { color = 4 as libc::c_int as byte_hack }
    sprintf(num.as_mut_ptr(), b"%6ld\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).csp as libc::c_long);
    c_put_str(color, num.as_mut_ptr() as cptr, 10 as libc::c_int,
              65 as libc::c_int);
    put_str(b"/\x00" as *const u8 as *const libc::c_char, 10 as libc::c_int,
            71 as libc::c_int);
    sprintf(num.as_mut_ptr(), b"%6ld\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).msp as libc::c_long);
    c_put_str(13 as libc::c_int as byte_hack, num.as_mut_ptr() as cptr,
              10 as libc::c_int, 72 as libc::c_int);
    put_str(b"Sanity       \x00" as *const u8 as *const libc::c_char,
            11 as libc::c_int, 52 as libc::c_int);
    if (*p_ptr).csane as libc::c_int >= (*p_ptr).msane as libc::c_int {
        color = 13 as libc::c_int as byte_hack
    } else if (*p_ptr).csane as libc::c_int >
                  (*p_ptr).msane as libc::c_int * hitpoint_warn as libc::c_int
                      / 10 as libc::c_int {
        color = 11 as libc::c_int as byte_hack
    } else { color = 4 as libc::c_int as byte_hack }
    sprintf(num.as_mut_ptr(), b"%6ld\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).csane as libc::c_long);
    c_put_str(color, num.as_mut_ptr() as cptr, 11 as libc::c_int,
              65 as libc::c_int);
    put_str(b"/\x00" as *const u8 as *const libc::c_char, 11 as libc::c_int,
            71 as libc::c_int);
    sprintf(num.as_mut_ptr(), b"%6ld\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).msane as libc::c_long);
    c_put_str(13 as libc::c_int as byte_hack, num.as_mut_ptr() as cptr,
              11 as libc::c_int, 72 as libc::c_int);
    if (*p_ptr).pgod as libc::c_int != 0 as libc::c_int {
        prt_num(b"Piety          \x00" as *const u8 as *const libc::c_char,
                (*p_ptr).grace, 12 as libc::c_int, 52 as libc::c_int,
                13 as libc::c_int as byte_hack,
                b"     \x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char);
    }
    put_str(b"Speed           \x00" as *const u8 as *const libc::c_char,
            13 as libc::c_int, 52 as libc::c_int);
    speed = (*p_ptr).pspeed as libc::c_int;
    /* Hack -- Visually "undo" the Search Mode Slowdown */
    if (*p_ptr).searching != 0 { speed += 10 as libc::c_int }
    if speed > 110 as libc::c_int {
        let mut s: [libc::c_char; 11] = [0; 11];
        sprintf(s.as_mut_ptr(),
                b"Fast (+%d)\x00" as *const u8 as *const libc::c_char,
                speed - 110 as libc::c_int);
        c_put_str(13 as libc::c_int as byte_hack, s.as_mut_ptr() as cptr,
                  13 as libc::c_int,
                  if speed >= 120 as libc::c_int {
                      68 as libc::c_int
                  } else { 69 as libc::c_int });
    } else if speed < 110 as libc::c_int {
        let mut s_0: [libc::c_char; 11] = [0; 11];
        sprintf(s_0.as_mut_ptr(),
                b"Slow (-%d)\x00" as *const u8 as *const libc::c_char,
                110 as libc::c_int - speed);
        c_put_str(15 as libc::c_int as byte_hack, s_0.as_mut_ptr() as cptr,
                  13 as libc::c_int,
                  if speed <= 100 as libc::c_int {
                      68 as libc::c_int
                  } else { 69 as libc::c_int });
    } else {
        put_str(b"Normal\x00" as *const u8 as *const libc::c_char,
                13 as libc::c_int, 72 as libc::c_int);
    };
}
/*
 * Hack -- pass color info around this file
 */
static mut likert_color: byte_hack = 1 as libc::c_int as byte_hack;
/*
 * Returns a "rating" of x depending on y
 */
unsafe extern "C" fn likert(mut x: libc::c_int, mut y: libc::c_int) -> cptr {
    static mut dummy: [libc::c_char; 20] =
        unsafe {
            *::std::mem::transmute::<&[u8; 20],
                                     &mut [libc::c_char; 20]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")
        };
    /* Paranoia */
    if y <= 0 as libc::c_int { y = 1 as libc::c_int }
    /* Negative value */
    if x < 0 as libc::c_int {
        likert_color = 8 as libc::c_int as byte_hack;
        return b"Very Bad\x00" as *const u8 as *const libc::c_char
    }
    /* Analyze the value */
    match x / y {
        0 | 1 => {
            likert_color = 4 as libc::c_int as byte_hack;
            return b"Bad\x00" as *const u8 as *const libc::c_char
        }
        2 => {
            likert_color = 12 as libc::c_int as byte_hack;
            return b"Poor\x00" as *const u8 as *const libc::c_char
        }
        3 | 4 => {
            likert_color = 3 as libc::c_int as byte_hack;
            return b"Fair\x00" as *const u8 as *const libc::c_char
        }
        5 => {
            likert_color = 11 as libc::c_int as byte_hack;
            return b"Good\x00" as *const u8 as *const libc::c_char
        }
        6 => {
            likert_color = 11 as libc::c_int as byte_hack;
            return b"Very Good\x00" as *const u8 as *const libc::c_char
        }
        7 | 8 => {
            likert_color = 13 as libc::c_int as byte_hack;
            return b"Excellent\x00" as *const u8 as *const libc::c_char
        }
        9 | 10 | 11 | 12 | 13 => {
            likert_color = 5 as libc::c_int as byte_hack;
            return b"Superb\x00" as *const u8 as *const libc::c_char
        }
        14 | 15 | 16 | 17 => {
            likert_color = 13 as libc::c_int as byte_hack;
            return b"Heroic\x00" as *const u8 as *const libc::c_char
        }
        _ => {
            likert_color = 13 as libc::c_int as byte_hack;
            sprintf(dummy.as_mut_ptr(),
                    b"Legendary[%d]\x00" as *const u8 as *const libc::c_char,
                    (x / y - 17 as libc::c_int) * 5 as libc::c_int /
                        2 as libc::c_int);
            return dummy.as_mut_ptr() as cptr
        }
    };
}
/*
 * Prints ratings on certain abilities
 *
 * This code is "imitated" elsewhere to "dump" a character sheet.
 */
unsafe extern "C" fn display_player_various() {
    let mut tmp: libc::c_int = 0;
    let mut tmp2: libc::c_int = 0;
    let mut damdice: libc::c_int = 0;
    let mut damsides: libc::c_int = 0;
    let mut dambonus: libc::c_int = 0;
    let mut blows: libc::c_int = 0;
    let mut xthn: libc::c_int = 0;
    let mut xthb: libc::c_int = 0;
    let mut xfos: libc::c_int = 0;
    let mut xsrh: libc::c_int = 0;
    let mut xdis: libc::c_int = 0;
    let mut xdev: libc::c_int = 0;
    let mut xsav: libc::c_int = 0;
    let mut xstl: libc::c_int = 0;
    let mut desc: cptr = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    /* Fighting Skill (with current weapon) */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(24 as libc::c_int as
                                                         isize) as
            *mut object_type;
    tmp =
        (*p_ptr).to_h as libc::c_int + (*o_ptr).to_h as libc::c_int +
            (*p_ptr).to_h_melee as libc::c_int;
    xthn = (*p_ptr).skill_thn as libc::c_int + tmp * 3 as libc::c_int;
    /* Shooting Skill (with current bow and normal missile) */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(27 as libc::c_int as
                                                         isize) as
            *mut object_type;
    tmp =
        (*p_ptr).to_h as libc::c_int + (*o_ptr).to_h as libc::c_int +
            (*p_ptr).to_h_ranged as libc::c_int;
    xthb = (*p_ptr).skill_thb as libc::c_int + tmp * 3 as libc::c_int;
    /* variables for all types of melee damage */
    dambonus = (*p_ptr).dis_to_d as libc::c_int;
    blows = (*p_ptr).num_blow as libc::c_int;
    /* Basic abilities */
    xdis = (*p_ptr).skill_dis as libc::c_int; /* From PsiAngband */
    xdev = (*p_ptr).skill_dev as libc::c_int;
    xsav = (*p_ptr).skill_sav as libc::c_int;
    xstl = (*p_ptr).skill_stl as libc::c_int;
    xsrh = (*p_ptr).skill_srh as libc::c_int;
    xfos = (*p_ptr).skill_fos as libc::c_int;
    put_str(b"Fighting    :\x00" as *const u8 as *const libc::c_char,
            16 as libc::c_int, 1 as libc::c_int);
    desc = likert(xthn, 12 as libc::c_int);
    c_put_str(likert_color, desc, 16 as libc::c_int, 15 as libc::c_int);
    put_str(b"Bows/Throw  :\x00" as *const u8 as *const libc::c_char,
            17 as libc::c_int, 1 as libc::c_int);
    desc = likert(xthb, 12 as libc::c_int);
    c_put_str(likert_color, desc, 17 as libc::c_int, 15 as libc::c_int);
    put_str(b"Saving Throw:\x00" as *const u8 as *const libc::c_char,
            18 as libc::c_int, 1 as libc::c_int);
    desc = likert(xsav, 6 as libc::c_int);
    c_put_str(likert_color, desc, 18 as libc::c_int, 15 as libc::c_int);
    put_str(b"Stealth     :\x00" as *const u8 as *const libc::c_char,
            19 as libc::c_int, 1 as libc::c_int);
    desc = likert(xstl, 1 as libc::c_int);
    c_put_str(likert_color, desc, 19 as libc::c_int, 15 as libc::c_int);
    put_str(b"Perception  :\x00" as *const u8 as *const libc::c_char,
            16 as libc::c_int, 28 as libc::c_int);
    desc = likert(xfos, 6 as libc::c_int);
    c_put_str(likert_color, desc, 16 as libc::c_int, 42 as libc::c_int);
    put_str(b"Searching   :\x00" as *const u8 as *const libc::c_char,
            17 as libc::c_int, 28 as libc::c_int);
    desc = likert(xsrh, 6 as libc::c_int);
    c_put_str(likert_color, desc, 17 as libc::c_int, 42 as libc::c_int);
    put_str(b"Disarming   :\x00" as *const u8 as *const libc::c_char,
            18 as libc::c_int, 28 as libc::c_int);
    desc = likert(xdis, 8 as libc::c_int);
    c_put_str(likert_color, desc, 18 as libc::c_int, 42 as libc::c_int);
    put_str(b"Magic Device:\x00" as *const u8 as *const libc::c_char,
            19 as libc::c_int, 28 as libc::c_int);
    desc = likert(xdev, 6 as libc::c_int);
    c_put_str(likert_color, desc, 19 as libc::c_int, 42 as libc::c_int);
    put_str(b"Blows/Round:\x00" as *const u8 as *const libc::c_char,
            16 as libc::c_int, 55 as libc::c_int);
    put_str(format(b"%d\x00" as *const u8 as *const libc::c_char,
                   (*p_ptr).num_blow as libc::c_int) as cptr,
            16 as libc::c_int, 69 as libc::c_int);
    put_str(b"Shots/Round:\x00" as *const u8 as *const libc::c_char,
            17 as libc::c_int, 55 as libc::c_int);
    put_str(format(b"%d\x00" as *const u8 as *const libc::c_char,
                   (*p_ptr).num_fire as libc::c_int) as cptr,
            17 as libc::c_int, 69 as libc::c_int);
    put_str(b"Mel.dmg/Rnd:\x00" as *const u8 as *const libc::c_char,
            18 as libc::c_int, 55 as libc::c_int);
    if (*p_ptr).melee_style as libc::c_int == 42 as libc::c_int ||
           (*p_ptr).melee_style as libc::c_int == 47 as libc::c_int {
        /* This is all based on py_attack_hand */
        let mut blow_table: *mut martial_arts = 0 as *mut martial_arts;
        let mut min_attack: *mut martial_arts = 0 as *mut martial_arts;
        let mut max_attack: *mut martial_arts = 0 as *mut martial_arts;
        let mut max_blow: libc::c_int = 0;
        let mut plev: libc::c_int = 0;
        let mut i_0: libc::c_int = 0;
        if (*p_ptr).melee_style as libc::c_int == 42 as libc::c_int {
            blow_table = ma_blows.as_mut_ptr();
            max_blow = 17 as libc::c_int;
            plev = get_skill(42 as libc::c_int) as libc::c_int
        } else {
            /* SKILL_BEAR */
            blow_table = bear_blows.as_mut_ptr();
            max_blow = 8 as libc::c_int;
            plev = get_skill(47 as libc::c_int) as libc::c_int
        }
        min_attack = blow_table;
        i_0 = max_blow - 1 as libc::c_int;
        while (*blow_table.offset(i_0 as isize)).min_level > plev &&
                  i_0 != 0 as libc::c_int {
            i_0 -= 1
        }
        max_attack =
            &mut *blow_table.offset(i_0 as isize) as *mut martial_arts;
        dambonus += (*p_ptr).to_d_melee as libc::c_int;
        tmp = (*min_attack).dd + dambonus;
        if tmp < 0 as libc::c_int { tmp = 0 as libc::c_int }
        tmp2 =
            maxroll((*max_attack).dd as s16b, (*max_attack).ds as s16b) +
                dambonus;
        if tmp2 < 0 as libc::c_int { tmp2 = 0 as libc::c_int }
        if tmp == 0 && tmp2 == 0 {
            desc = b"0\x00" as *const u8 as *const libc::c_char
        } else {
            desc =
                format(b"%d-%d\x00" as *const u8 as *const libc::c_char,
                       blows * tmp, blows * tmp2) as cptr
        }
    } else if (*r_info.offset((*p_ptr).body_monster as
                                  isize)).body_parts[0 as libc::c_int as
                                                         usize] == 0 {
        if (*r_info.offset((*p_ptr).body_monster as isize)).flags1 &
               0x10000 as libc::c_int as libc::c_uint != 0 {
            desc = b"nil!\x00" as *const u8 as *const libc::c_char
        } else {
            tmp2 = 0 as libc::c_int;
            tmp = tmp2;
            i = 0 as libc::c_int;
            while i < blows {
                tmp +=
                    (*r_info.offset((*p_ptr).body_monster as
                                        isize)).blow[i as usize].d_dice as
                        libc::c_int;
                tmp2 +=
                    maxroll((*r_info.offset((*p_ptr).body_monster as
                                                isize)).blow[i as
                                                                 usize].d_dice
                                as s16b,
                            (*r_info.offset((*p_ptr).body_monster as
                                                isize)).blow[i as
                                                                 usize].d_side
                                as s16b);
                i += 1
            }
            if dambonus > 0 as libc::c_int {
                tmp += dambonus;
                tmp2 += dambonus
            }
            desc =
                format(b"%d-%d\x00" as *const u8 as *const libc::c_char, tmp,
                       tmp2) as cptr
        }
    } else {
        /* Increase the bonus to damage for weapon combat */
        dambonus += (*p_ptr).to_d_melee as libc::c_int;
        /* Access the first weapon */
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(24 as libc::c_int as
                                                             isize) as
                *mut object_type;
        if (*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
               (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as
                   libc::c_int != 0 &&
                   (*k_info.offset((*o_ptr).k_idx as isize)).aware as
                       libc::c_int != 0 {
            dambonus += (*o_ptr).to_d as libc::c_int
        }
        damdice = (*o_ptr).dd as libc::c_int;
        damsides = (*o_ptr).ds as libc::c_int;
        if damdice == 0 as libc::c_int || damsides == 0 as libc::c_int {
            if dambonus <= 0 as libc::c_int {
                desc = b"nil!\x00" as *const u8 as *const libc::c_char
            } else {
                desc =
                    format(b"%d\x00" as *const u8 as *const libc::c_char,
                           blows * dambonus) as cptr
            }
        } else if dambonus == 0 as libc::c_int {
            desc =
                format(b"%dd%d\x00" as *const u8 as *const libc::c_char,
                       blows * damdice, damsides) as cptr
        } else {
            desc =
                format(b"%dd%d%c%d\x00" as *const u8 as *const libc::c_char,
                       blows * damdice, damsides,
                       if dambonus > 0 as libc::c_int {
                           '+' as i32
                       } else { '\u{0}' as i32 }, blows * dambonus) as cptr
        }
    }
    put_str(desc, 18 as libc::c_int, 69 as libc::c_int);
    put_str(b"Infra-Vision:\x00" as *const u8 as *const libc::c_char,
            19 as libc::c_int, 55 as libc::c_int);
    put_str(format(b"%d feet\x00" as *const u8 as *const libc::c_char,
                   (*p_ptr).see_infra as libc::c_int * 10 as libc::c_int) as
                cptr, 19 as libc::c_int, 69 as libc::c_int);
    /* jk - add tactic */
    put_str(b"Tactic:\x00" as *const u8 as *const libc::c_char,
            20 as libc::c_int, 55 as libc::c_int);
    c_put_str(14 as libc::c_int as byte_hack,
              tactic_info[(*p_ptr).tactic as byte_hack as usize].name,
              20 as libc::c_int, 69 as libc::c_int);
    /* jk - add movement */
    put_str(b"Explor:\x00" as *const u8 as *const libc::c_char,
            21 as libc::c_int, 55 as libc::c_int);
    c_put_str(14 as libc::c_int as byte_hack,
              move_info[(*p_ptr).movement as byte_hack as usize].name,
              21 as libc::c_int, 69 as libc::c_int);
}
/*
 * Obtain the "flags" of the wielded symbiote
 */
#[no_mangle]
pub unsafe extern "C" fn wield_monster_flags(mut f1: *mut u32b,
                                             mut f2: *mut u32b,
                                             mut f3: *mut u32b,
                                             mut f4: *mut u32b,
                                             mut f5: *mut u32b,
                                             mut esp: *mut u32b) {
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
    /* Clear */
    *esp = 0 as libc::c_long as u32b;
    *f5 = *esp;
    *f4 = *f5;
    *f3 = *f4;
    *f2 = *f3;
    *f1 = *f2;
    /* Get the carried monster */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(49 as libc::c_int as
                                                         isize) as
            *mut object_type;
    if (*o_ptr).k_idx != 0 {
        r_ptr =
            &mut *r_info.offset((*o_ptr).pval as isize) as *mut monster_race;
        if (*r_ptr).flags2 & 0x10 as libc::c_int as libc::c_uint != 0 {
            *f2 = (*f2 as libc::c_long | 0x40 as libc::c_long) as u32b
        }
        if (*r_ptr).flags2 & 0x8 as libc::c_int as libc::c_uint != 0 {
            *f2 = (*f2 as libc::c_long | 0x2000 as libc::c_long) as u32b
        }
        if (*r_ptr).flags7 & 0x4 as libc::c_int as libc::c_uint != 0 {
            *f3 = (*f3 as libc::c_long | 0x1000 as libc::c_long) as u32b
        }
        if (*r_ptr).flags7 & 0x1 as libc::c_int as libc::c_uint != 0 {
            *f5 = (*f5 as libc::c_long | 0x8000 as libc::c_long) as u32b
        }
    };
}
/*
 * Obtain the "flags" for the player as if he was an item
 */
#[no_mangle]
pub unsafe extern "C" fn player_flags(mut f1: *mut u32b, mut f2: *mut u32b,
                                      mut f3: *mut u32b, mut f4: *mut u32b,
                                      mut f5: *mut u32b, mut esp: *mut u32b) {
    let mut i: libc::c_int = 0;
    /* Clear */
    *esp = 0 as libc::c_long as u32b;
    *f5 = *esp;
    *f4 = *f5;
    *f3 = *f4;
    *f2 = *f3;
    *f1 = *f2;
    /* Astral chars */
    if (*p_ptr).astral != 0 {
        *f3 = (*f3 as libc::c_long | 0x40 as libc::c_long) as u32b
    }
    /* Skills */
    if get_skill(13 as libc::c_int) as libc::c_int > 20 as libc::c_int {
        *f2 = (*f2 as libc::c_long | 0x2000000 as libc::c_long) as u32b
    }
    if get_skill(13 as libc::c_int) as libc::c_int > 30 as libc::c_int {
        *f2 = (*f2 as libc::c_long | 0x200000 as libc::c_long) as u32b
    }
    if get_skill(29 as libc::c_int) as libc::c_int >= 40 as libc::c_int {
        *esp = (*esp as libc::c_long | 0x80000000 as libc::c_long) as u32b
    }
    if (*p_ptr).melee_style as libc::c_int == 42 as libc::c_int &&
           get_skill(42 as libc::c_int) as libc::c_int > 24 as libc::c_int &&
           monk_heavy_armor() == 0 {
        *f2 = (*f2 as libc::c_long | 0x4000 as libc::c_long) as u32b
    }
    /* Hack - from Lua */
    if get_skill(2 as libc::c_int) as libc::c_int >= 35 as libc::c_int {
        *f1 = (*f1 as libc::c_long | 0x40 as libc::c_long) as u32b
    }
    if get_skill(4 as libc::c_int) as libc::c_int >= 50 as libc::c_int {
        *f5 =
            (*f5 as libc::c_long |
                 (0x4000 as libc::c_long | 0x8000 as libc::c_long)) as u32b
    }
    if get_skill(5 as libc::c_int) as libc::c_int >= 30 as libc::c_int {
        *f5 = (*f5 as libc::c_long | 0x8000 as libc::c_long) as u32b
    }
    /* Gods */
    if (*p_ptr).pgod as libc::c_int == 1 as libc::c_int {
        if (*p_ptr).grace >= 100 as libc::c_int ||
               (*p_ptr).grace <= -(100 as libc::c_int) {
            *f1 = (*f1 as libc::c_long | 0x40 as libc::c_long) as u32b
        }
        if (*p_ptr).grace > 10000 as libc::c_int {
            *f1 = (*f1 as libc::c_long | 0x4 as libc::c_long) as u32b
        }
    }
    if (*p_ptr).pgod as libc::c_int == 4 as libc::c_int {
        *f2 = (*f2 as libc::c_long | 0x40000 as libc::c_long) as u32b;
        if (*p_ptr).melkor_sacrifice as libc::c_int > 0 as libc::c_int {
            *f2 = (*f2 as libc::c_long | 0x80 as libc::c_long) as u32b
        }
        if (*p_ptr).grace > 10000 as libc::c_int {
            *f1 =
                (*f1 as libc::c_long |
                     (0x1 as libc::c_long | 0x10 as libc::c_long |
                          0x2 as libc::c_long | 0x4 as libc::c_long |
                          0x20 as libc::c_long)) as u32b
        }
        if (*p_ptr).pgod as libc::c_int == 4 as libc::c_int &&
               (*p_ptr).praying as libc::c_int != 0 {
            if (*p_ptr).grace > 5000 as libc::c_int {
                *f2 = (*f2 as libc::c_long | 0x40 as libc::c_long) as u32b
            }
            if (*p_ptr).grace > 15000 as libc::c_int {
                *f2 = (*f2 as libc::c_long | 0x400 as libc::c_long) as u32b
            }
        }
    }
    if (*p_ptr).pgod as libc::c_int == 2 as libc::c_int {
        if (*p_ptr).grace >= 2000 as libc::c_int {
            *f3 = (*f3 as libc::c_long | 0x1000 as libc::c_long) as u32b
        }
        if (*p_ptr).pgod as libc::c_int == 2 as libc::c_int &&
               (*p_ptr).praying as libc::c_int != 0 {
            if (*p_ptr).grace >= 7000 as libc::c_int {
                *f2 = (*f2 as libc::c_long | 0x4000 as libc::c_long) as u32b
            }
            if (*p_ptr).grace >= 15000 as libc::c_int {
                *f4 = (*f4 as libc::c_long | 0x10 as libc::c_long) as u32b
            }
            if (*p_ptr).grace >= 5000 as libc::c_int ||
                   (*p_ptr).grace <= -(5000 as libc::c_int) {
                *f1 = (*f1 as libc::c_long | 0x1000 as libc::c_long) as u32b
            }
        }
    }
    if (*p_ptr).pgod as libc::c_int == 3 as libc::c_int {
        if (*p_ptr).grace > 5000 as libc::c_int {
            *f1 = (*f1 as libc::c_long | 0x10 as libc::c_long) as u32b
        }
        if (*p_ptr).grace > 10000 as libc::c_int {
            *f1 = (*f1 as libc::c_long | 0x1 as libc::c_long) as u32b
        }
    }
    /* Classes */
    i = 1 as libc::c_int;
    while i <= (*p_ptr).lev as libc::c_int {
        *f1 |= (*cp_ptr).oflags1[i as usize];
        *f2 |= (*cp_ptr).oflags2[i as usize];
        *f3 |= (*cp_ptr).oflags3[i as usize];
        *f4 |= (*cp_ptr).oflags4[i as usize];
        *f5 |= (*cp_ptr).oflags5[i as usize];
        *esp |= (*cp_ptr).oesp[i as usize];
        i += 1
    }
    /* Races */
    if (*p_ptr).mimic_form == 0 && (*p_ptr).body_monster == 0 {
        i = 1 as libc::c_int;
        while i <= (*p_ptr).lev as libc::c_int {
            *f1 |= (*rp_ptr).oflags1[i as usize];
            *f2 |= (*rp_ptr).oflags2[i as usize];
            *f3 |= (*rp_ptr).oflags3[i as usize];
            *f4 |= (*rp_ptr).oflags4[i as usize];
            *f5 |= (*rp_ptr).oflags5[i as usize];
            *esp |= (*rp_ptr).oesp[i as usize];
            *f1 |= (*rmp_ptr).oflags1[i as usize];
            *f2 |= (*rmp_ptr).oflags2[i as usize];
            *f3 |= (*rmp_ptr).oflags3[i as usize];
            *f4 |= (*rmp_ptr).oflags4[i as usize];
            *f5 |= (*rmp_ptr).oflags5[i as usize];
            *esp |= (*rmp_ptr).oesp[i as usize];
            i += 1
        }
    } else {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset((*p_ptr).body_monster as isize) as
                *mut monster_race;
        if (*r_ptr).flags2 & 0x8 as libc::c_int as libc::c_uint != 0 {
            *f2 = (*f2 as libc::c_long | 0x2000 as libc::c_long) as u32b
        }
        if (*r_ptr).flags2 & 0x200 as libc::c_int as libc::c_uint != 0 {
            *f3 = (*f3 as libc::c_long | 0x20000 as libc::c_long) as u32b
        }
        if (*r_ptr).flags2 & 0x4000 as libc::c_int as libc::c_uint != 0 {
            *f3 = (*f3 as libc::c_long | 0x1 as libc::c_long) as u32b
        }
        if (*r_ptr).flags2 & 0x8000 as libc::c_int as libc::c_uint != 0 {
            *f3 = (*f3 as libc::c_long | 0x2 as libc::c_long) as u32b
        }
        if (*r_ptr).flags2 & 0x40000 as libc::c_int as libc::c_uint != 0 {
            *f3 = (*f3 as libc::c_long | 0x40 as libc::c_long) as u32b
        }
        if (*r_ptr).flags3 & 0x4000 as libc::c_int as libc::c_uint != 0 {
            *f2 = (*f2 as libc::c_long | 0x1000 as libc::c_long) as u32b
        }
        if (*r_ptr).flags3 & 0x10000 as libc::c_int as libc::c_uint != 0 {
            *f2 = (*f2 as libc::c_long | 0x10000 as libc::c_long) as u32b
        }
        if (*r_ptr).flags3 & 0x20000 as libc::c_int as libc::c_uint != 0 {
            *f2 = (*f2 as libc::c_long | 0x20000 as libc::c_long) as u32b
        }
        if (*r_ptr).flags3 & 0x40000 as libc::c_int as libc::c_uint != 0 {
            *f2 = (*f2 as libc::c_long | 0x40000 as libc::c_long) as u32b
        }
        if (*r_ptr).flags3 & 0x100000 as libc::c_int as libc::c_uint != 0 {
            *f2 = (*f2 as libc::c_long | 0x100000 as libc::c_long) as u32b
        }
        if (*r_ptr).flags3 & 0x80000 as libc::c_int as libc::c_uint != 0 {
            *f2 = (*f2 as libc::c_long | 0x80000 as libc::c_long) as u32b
        }
        if (*r_ptr).flags3 & 0x400000 as libc::c_int as libc::c_uint != 0 {
            *f2 = (*f2 as libc::c_long | 0x10000000 as libc::c_long) as u32b
        }
        if (*r_ptr).flags3 & 0x2000000 as libc::c_int as libc::c_uint != 0 {
            *f2 = (*f2 as libc::c_long | 0x20000000 as libc::c_long) as u32b
        }
        if (*r_ptr).flags3 & 0x4000000 as libc::c_int as libc::c_uint != 0 {
            *f2 = (*f2 as libc::c_long | 0x80000000 as libc::c_long) as u32b
        }
        if (*r_ptr).flags3 & 0x10000000 as libc::c_int as libc::c_uint != 0 {
            *f2 = (*f2 as libc::c_long | 0x200000 as libc::c_long) as u32b
        }
        if (*r_ptr).flags3 & 0x80000000 as libc::c_uint != 0 {
            *f2 = (*f2 as libc::c_long | 0x4000 as libc::c_long) as u32b
        }
        if (*r_ptr).flags3 & 0x40000000 as libc::c_int as libc::c_uint != 0 {
            *f2 = (*f2 as libc::c_long | 0x2000000 as libc::c_long) as u32b
        }
        if (*r_ptr).flags7 & 0x4 as libc::c_int as libc::c_uint != 0 {
            *f3 = (*f3 as libc::c_long | 0x1000 as libc::c_long) as u32b
        }
    }
    *f1 |= (*p_ptr).xtra_f1;
    *f2 |= (*p_ptr).xtra_f2;
    *f3 |= (*p_ptr).xtra_f3;
    *f4 |= (*p_ptr).xtra_f4;
    *f5 |= (*p_ptr).xtra_f5;
    *esp |= (*p_ptr).xtra_esp;
    if (*p_ptr).black_breath != 0 {
        *f4 = (*f4 as libc::c_long | 0x4 as libc::c_long) as u32b
    }
    if (*p_ptr).hp_mod as libc::c_int != 0 as libc::c_int {
        *f2 = (*f2 as libc::c_long | 0x80 as libc::c_long) as u32b
    };
}
/*
 * Object flag names
 */
static mut object_flag_names: [cptr; 192] =
    [b"Add Str\x00" as *const u8 as *const libc::c_char,
     b"Add Int\x00" as *const u8 as *const libc::c_char,
     b"Add Wis\x00" as *const u8 as *const libc::c_char,
     b"Add Dex\x00" as *const u8 as *const libc::c_char,
     b"Add Con\x00" as *const u8 as *const libc::c_char,
     b"Add Chr\x00" as *const u8 as *const libc::c_char,
     b"Mul Mana\x00" as *const u8 as *const libc::c_char,
     b"Mul SPower\x00" as *const u8 as *const libc::c_char,
     b"Add Stea.\x00" as *const u8 as *const libc::c_char,
     b"Add Sear.\x00" as *const u8 as *const libc::c_char,
     b"Add Infra\x00" as *const u8 as *const libc::c_char,
     b"Add Tun..\x00" as *const u8 as *const libc::c_char,
     b"Add Speed\x00" as *const u8 as *const libc::c_char,
     b"Add Blows\x00" as *const u8 as *const libc::c_char,
     b"Chaotic\x00" as *const u8 as *const libc::c_char,
     b"Vampiric\x00" as *const u8 as *const libc::c_char,
     b"Slay Anim.\x00" as *const u8 as *const libc::c_char,
     b"Slay Evil\x00" as *const u8 as *const libc::c_char,
     b"Slay Und.\x00" as *const u8 as *const libc::c_char,
     b"Slay Demon\x00" as *const u8 as *const libc::c_char,
     b"Slay Orc\x00" as *const u8 as *const libc::c_char,
     b"Slay Troll\x00" as *const u8 as *const libc::c_char,
     b"Slay Giant\x00" as *const u8 as *const libc::c_char,
     b"Slay Drag.\x00" as *const u8 as *const libc::c_char,
     b"Kill Drag.\x00" as *const u8 as *const libc::c_char,
     b"Sharpness\x00" as *const u8 as *const libc::c_char,
     b"Impact\x00" as *const u8 as *const libc::c_char,
     b"Poison Brd\x00" as *const u8 as *const libc::c_char,
     b"Acid Brand\x00" as *const u8 as *const libc::c_char,
     b"Elec Brand\x00" as *const u8 as *const libc::c_char,
     b"Fire Brand\x00" as *const u8 as *const libc::c_char,
     b"Cold Brand\x00" as *const u8 as *const libc::c_char,
     b"Sust Str\x00" as *const u8 as *const libc::c_char,
     b"Sust Int\x00" as *const u8 as *const libc::c_char,
     b"Sust Wis\x00" as *const u8 as *const libc::c_char,
     b"Sust Dex\x00" as *const u8 as *const libc::c_char,
     b"Sust Con\x00" as *const u8 as *const libc::c_char,
     b"Sust Chr\x00" as *const u8 as *const libc::c_char,
     b"Invisible\x00" as *const u8 as *const libc::c_char,
     b"Mul life\x00" as *const u8 as *const libc::c_char,
     b"Imm Acid\x00" as *const u8 as *const libc::c_char,
     b"Imm Elec\x00" as *const u8 as *const libc::c_char,
     b"Imm Fire\x00" as *const u8 as *const libc::c_char,
     b"Imm Cold\x00" as *const u8 as *const libc::c_char,
     b"Sens Fire\x00" as *const u8 as *const libc::c_char,
     b"Reflect\x00" as *const u8 as *const libc::c_char,
     b"Free Act\x00" as *const u8 as *const libc::c_char,
     b"Hold Life\x00" as *const u8 as *const libc::c_char,
     b"Res Acid\x00" as *const u8 as *const libc::c_char,
     b"Res Elec\x00" as *const u8 as *const libc::c_char,
     b"Res Fire\x00" as *const u8 as *const libc::c_char,
     b"Res Cold\x00" as *const u8 as *const libc::c_char,
     b"Res Pois\x00" as *const u8 as *const libc::c_char,
     b"Res Fear\x00" as *const u8 as *const libc::c_char,
     b"Res Light\x00" as *const u8 as *const libc::c_char,
     b"Res Dark\x00" as *const u8 as *const libc::c_char,
     b"Res Blind\x00" as *const u8 as *const libc::c_char,
     b"Res Conf\x00" as *const u8 as *const libc::c_char,
     b"Res Sound\x00" as *const u8 as *const libc::c_char,
     b"Res Shard\x00" as *const u8 as *const libc::c_char,
     b"Res Neth\x00" as *const u8 as *const libc::c_char,
     b"Res Nexus\x00" as *const u8 as *const libc::c_char,
     b"Res Chaos\x00" as *const u8 as *const libc::c_char,
     b"Res Disen\x00" as *const u8 as *const libc::c_char,
     b"Aura Fire\x00" as *const u8 as *const libc::c_char,
     b"Aura Elec\x00" as *const u8 as *const libc::c_char,
     b"Auto Curse\x00" as *const u8 as *const libc::c_char, 0 as cptr,
     b"NoTeleport\x00" as *const u8 as *const libc::c_char,
     b"AntiMagic\x00" as *const u8 as *const libc::c_char,
     b"WraithForm\x00" as *const u8 as *const libc::c_char,
     b"EvilCurse\x00" as *const u8 as *const libc::c_char, 0 as cptr,
     0 as cptr, 0 as cptr, 0 as cptr,
     b"Levitate\x00" as *const u8 as *const libc::c_char,
     b"Lite\x00" as *const u8 as *const libc::c_char,
     b"See Invis\x00" as *const u8 as *const libc::c_char, 0 as cptr,
     b"Digestion\x00" as *const u8 as *const libc::c_char,
     b"Regen\x00" as *const u8 as *const libc::c_char,
     b"Xtra Might\x00" as *const u8 as *const libc::c_char,
     b"Xtra Shots\x00" as *const u8 as *const libc::c_char, 0 as cptr,
     0 as cptr, 0 as cptr, 0 as cptr,
     b"Activate\x00" as *const u8 as *const libc::c_char,
     b"Drain Exp\x00" as *const u8 as *const libc::c_char,
     b"Teleport\x00" as *const u8 as *const libc::c_char,
     b"Aggravate\x00" as *const u8 as *const libc::c_char,
     b"Blessed\x00" as *const u8 as *const libc::c_char,
     b"Cursed\x00" as *const u8 as *const libc::c_char,
     b"Hvy Curse\x00" as *const u8 as *const libc::c_char,
     b"Prm Curse\x00" as *const u8 as *const libc::c_char,
     b"No blows\x00" as *const u8 as *const libc::c_char,
     b"Precogn.\x00" as *const u8 as *const libc::c_char,
     b"B.Breath\x00" as *const u8 as *const libc::c_char,
     b"Recharge\x00" as *const u8 as *const libc::c_char,
     b"Fly\x00" as *const u8 as *const libc::c_char,
     b"Mrg.Curse\x00" as *const u8 as *const libc::c_char, 0 as cptr,
     0 as cptr, b"Sentient\x00" as *const u8 as *const libc::c_char,
     b"Clone\x00" as *const u8 as *const libc::c_char, 0 as cptr,
     b"Climb\x00" as *const u8 as *const libc::c_char, 0 as cptr, 0 as cptr,
     0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr,
     0 as cptr, 0 as cptr,
     b"Imm Neth\x00" as *const u8 as *const libc::c_char, 0 as cptr,
     0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr,
     0 as cptr, 0 as cptr, b"Orc.ESP\x00" as *const u8 as *const libc::c_char,
     b"Troll.ESP\x00" as *const u8 as *const libc::c_char,
     b"Dragon.ESP\x00" as *const u8 as *const libc::c_char,
     b"Giant.ESP\x00" as *const u8 as *const libc::c_char,
     b"Demon.ESP\x00" as *const u8 as *const libc::c_char,
     b"Undead.ESP\x00" as *const u8 as *const libc::c_char,
     b"Evil.ESP\x00" as *const u8 as *const libc::c_char,
     b"Animal.ESP\x00" as *const u8 as *const libc::c_char,
     b"TLord.ESP\x00" as *const u8 as *const libc::c_char,
     b"Good.ESP\x00" as *const u8 as *const libc::c_char,
     b"Nlive.ESP\x00" as *const u8 as *const libc::c_char,
     b"Unique.ESP\x00" as *const u8 as *const libc::c_char,
     b"Spider ESP\x00" as *const u8 as *const libc::c_char, 0 as cptr,
     0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr,
     0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr,
     0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr, 0 as cptr,
     b"Full ESP\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char];
/*
 * Summarize resistances
 */
unsafe extern "C" fn display_player_ben_one(mut mode: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut dispx: libc::c_int = 0;
    let mut modetemp: libc::c_int = 0;
    let mut xtemp: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut dummy: [libc::c_char; 80] = [0; 80];
    let mut c: libc::c_char = 0;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut b: [[u16b; 10]; 29] = [[0; 10]; 29];
    let mut d: [libc::c_int; 29] = [0; 29];
    let mut got: bool_ = 0;
    let mut a: byte_hack = 0;
    let mut name: cptr = 0 as *const libc::c_char;
    /* Scan equipment */
    i = 24 as libc::c_int;
    while i < 52 as libc::c_int {
        /* Index */
        n = i - 24 as libc::c_int;
        /* Object */
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Known object flags */
        object_flags_known(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                           &mut esp);
        /* Incorporate */
        b[n as usize][0 as libc::c_int as usize] =
            (f1 & 0xffff as libc::c_int as libc::c_uint) as u16b;
        b[n as usize][1 as libc::c_int as usize] =
            (f1 >> 16 as libc::c_int) as u16b;
        b[n as usize][2 as libc::c_int as usize] =
            (f2 & 0xffff as libc::c_int as libc::c_uint) as u16b;
        b[n as usize][3 as libc::c_int as usize] =
            (f2 >> 16 as libc::c_int) as u16b;
        b[n as usize][4 as libc::c_int as usize] =
            (f3 & 0xffff as libc::c_int as libc::c_uint) as u16b;
        b[n as usize][5 as libc::c_int as usize] =
            (f3 >> 16 as libc::c_int) as u16b;
        b[n as usize][6 as libc::c_int as usize] =
            (f4 & 0xffff as libc::c_int as libc::c_uint) as u16b;
        b[n as usize][7 as libc::c_int as usize] =
            (f4 >> 16 as libc::c_int) as u16b;
        b[n as usize][8 as libc::c_int as usize] =
            (esp & 0xffff as libc::c_int as libc::c_uint) as u16b;
        b[n as usize][9 as libc::c_int as usize] =
            (esp >> 16 as libc::c_int) as u16b;
        d[n as usize] = (*o_ptr).pval;
        i += 1
    }
    /* Carried symbiote */
    n = 49 as libc::c_int - 24 as libc::c_int;
    /* Player flags */
    wield_monster_flags(&mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                        &mut esp);
    /* Incorporate */
    b[n as usize][0 as libc::c_int as usize] =
        (f1 & 0xffff as libc::c_int as libc::c_uint) as u16b;
    b[n as usize][1 as libc::c_int as usize] =
        (f1 >> 16 as libc::c_int) as u16b;
    b[n as usize][2 as libc::c_int as usize] =
        (f2 & 0xffff as libc::c_int as libc::c_uint) as u16b;
    b[n as usize][3 as libc::c_int as usize] =
        (f2 >> 16 as libc::c_int) as u16b;
    b[n as usize][4 as libc::c_int as usize] =
        (f3 & 0xffff as libc::c_int as libc::c_uint) as u16b;
    b[n as usize][5 as libc::c_int as usize] =
        (f3 >> 16 as libc::c_int) as u16b;
    b[n as usize][6 as libc::c_int as usize] =
        (f4 & 0xffff as libc::c_int as libc::c_uint) as u16b;
    b[n as usize][7 as libc::c_int as usize] =
        (f4 >> 16 as libc::c_int) as u16b;
    b[n as usize][8 as libc::c_int as usize] =
        (esp & 0xffff as libc::c_int as libc::c_uint) as u16b;
    b[n as usize][9 as libc::c_int as usize] =
        (esp >> 16 as libc::c_int) as u16b;
    /* Index */
    n = 52 as libc::c_int - 24 as libc::c_int;
    /* Player flags */
    player_flags(&mut f1, &mut f2, &mut f3, &mut f4, &mut f5, &mut esp);
    /* Incorporate */
    b[n as usize][0 as libc::c_int as usize] =
        (f1 & 0xffff as libc::c_int as libc::c_uint) as u16b;
    b[n as usize][1 as libc::c_int as usize] =
        (f1 >> 16 as libc::c_int) as u16b;
    b[n as usize][2 as libc::c_int as usize] =
        (f2 & 0xffff as libc::c_int as libc::c_uint) as u16b;
    b[n as usize][3 as libc::c_int as usize] =
        (f2 >> 16 as libc::c_int) as u16b;
    b[n as usize][4 as libc::c_int as usize] =
        (f3 & 0xffff as libc::c_int as libc::c_uint) as u16b;
    b[n as usize][5 as libc::c_int as usize] =
        (f3 >> 16 as libc::c_int) as u16b;
    b[n as usize][6 as libc::c_int as usize] =
        (f4 & 0xffff as libc::c_int as libc::c_uint) as u16b;
    b[n as usize][7 as libc::c_int as usize] =
        (f4 >> 16 as libc::c_int) as u16b;
    b[n as usize][8 as libc::c_int as usize] =
        (esp & 0xffff as libc::c_int as libc::c_uint) as u16b;
    b[n as usize][9 as libc::c_int as usize] =
        (esp >> 16 as libc::c_int) as u16b;
    /* Generate the equip chars */
    sprintf(dummy.as_mut_ptr(), b" \x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int - 24 as libc::c_int {
        /* If you have that body part then show it */
        if (*p_ptr).body_parts[i as usize] != 0 {
            strcat(dummy.as_mut_ptr(),
                   format(b"%c\x00" as *const u8 as *const libc::c_char,
                          i + 'a' as i32));
        }
        i += 1
    }
    strcat(dummy.as_mut_ptr(), b"@\x00" as *const u8 as *const libc::c_char);
    /* Scan cols */
    x = 1 as libc::c_int;
    while x > -(1 as libc::c_int) {
        /* Label */
        Term_putstr(x * 40 as libc::c_int + 11 as libc::c_int,
                    3 as libc::c_int, -(1 as libc::c_int),
                    1 as libc::c_int as byte_hack,
                    dummy.as_mut_ptr() as cptr);
        /* Scan rows */
        y = 0 as libc::c_int;
        while y < 16 as libc::c_int {
            if mode == 3 as libc::c_int && x == 1 as libc::c_int {
                modetemp = 4 as libc::c_int;
                xtemp = 0 as libc::c_int
            } else { modetemp = mode; xtemp = x }
            z = mode;
            while z <= modetemp {
                if mode == 3 as libc::c_int && x == 1 as libc::c_int &&
                       z == modetemp {
                    xtemp = 1 as libc::c_int
                }
                name =
                    object_flag_names[(32 as libc::c_int * modetemp +
                                           16 as libc::c_int * xtemp + y) as
                                          usize];
                got = 0 as libc::c_int as bool_;
                /* No name */
                if !name.is_null() {
                    /* Dump colon */
                    if !(modetemp == 1 as libc::c_int && x == 0 as libc::c_int
                             && y > 7 as libc::c_int && y < 12 as libc::c_int)
                       {
                        Term_putch(x * 40 as libc::c_int + 10 as libc::c_int,
                                   y + 4 as libc::c_int,
                                   1 as libc::c_int as byte_hack,
                                   ':' as i32 as libc::c_char);
                    }
                    /* Check flags */
                    dispx = 0 as libc::c_int;
                    n = 0 as libc::c_int;
                    while n <
                              52 as libc::c_int - 24 as libc::c_int +
                                  1 as libc::c_int {
                        /* Change colour every two columns */
                        let mut is_green: bool_ =
                            (dispx & 0x2 as libc::c_int) as bool_;
                        a =
                            if is_green as libc::c_int != 0 {
                                5 as libc::c_int
                            } else { 2 as libc::c_int } as byte_hack;
                        c = '.' as i32 as libc::c_char;
                        /* If the body part doesn't exists then skip it :) */
                        if !(n < 52 as libc::c_int - 24 as libc::c_int &&
                                 (*p_ptr).body_parts[n as usize] == 0) {
                            /* Increment the drawing coordinates */
                            dispx += 1;
                            /* Check flag */
                            if b[n as
                                     usize][(2 as libc::c_int * modetemp +
                                                 xtemp) as usize] as
                                   libc::c_int & (1 as libc::c_int) << y != 0
                               {
                                a =
                                    if is_green as libc::c_int != 0 {
                                        13 as libc::c_int
                                    } else { 1 as libc::c_int } as byte_hack;
                                if modetemp == 1 as libc::c_int &&
                                       x == 0 as libc::c_int &&
                                       y > 7 as libc::c_int &&
                                       y < 12 as libc::c_int {
                                    c = '*' as i32 as libc::c_char
                                } else if modetemp == 0 as libc::c_int &&
                                              x == 0 as libc::c_int &&
                                              y < 14 as libc::c_int &&
                                              (y < 6 as libc::c_int ||
                                                   y > 7 as libc::c_int) {
                                    if n ==
                                           52 as libc::c_int -
                                               24 as libc::c_int {
                                        c = '+' as i32 as libc::c_char
                                    } else {
                                        c = d[n as usize] as libc::c_char;
                                        if (c as libc::c_int) <
                                               0 as libc::c_int {
                                            c =
                                                -(c as libc::c_int) as
                                                    libc::c_char;
                                            a = 4 as libc::c_int as byte_hack
                                        }
                                        c =
                                            if c as libc::c_int >
                                                   9 as libc::c_int {
                                                '*' as i32
                                            } else {
                                                (c as libc::c_int) +
                                                    '0' as i32
                                            } as libc::c_char
                                    }
                                } else { c = '+' as i32 as libc::c_char }
                                got = 1 as libc::c_int as bool_
                            }
                            /* HACK - Check for nether immunity and
					   apply to Res Neth line */
                            if modetemp == 1 as libc::c_int &&
                                   x == 1 as libc::c_int &&
                                   y == 12 as libc::c_int {
                                if b[n as usize][7 as libc::c_int as usize] as
                                       libc::c_int &
                                       (1 as libc::c_int) << 6 as libc::c_int
                                       != 0 {
                                    a =
                                        if is_green as libc::c_int != 0 {
                                            13 as libc::c_int
                                        } else { 1 as libc::c_int } as
                                            byte_hack;
                                    c = '*' as i32 as libc::c_char;
                                    got = 1 as libc::c_int as bool_
                                }
                            }
                            /* Dump flag */
                            if modetemp == 1 as libc::c_int &&
                                   x == 0 as libc::c_int &&
                                   y > 7 as libc::c_int &&
                                   y < 12 as libc::c_int {
                                if c as libc::c_int == '*' as i32 {
                                    Term_putch(40 as libc::c_int +
                                                   11 as libc::c_int + dispx,
                                               y - 4 as libc::c_int, a, c);
                                }
                            } else {
                                Term_putch(x * 40 as libc::c_int +
                                               11 as libc::c_int + dispx,
                                           y + 4 as libc::c_int, a, c);
                            }
                        }
                        n += 1
                    }
                    a = 1 as libc::c_int as byte_hack;
                    if got != 0 {
                        if modetemp == 1 as libc::c_int &&
                               x == 0 as libc::c_int && y > 7 as libc::c_int
                               && y < 12 as libc::c_int {
                            a = 13 as libc::c_int as byte_hack
                        } else if modetemp != 0 as libc::c_int {
                            a = 5 as libc::c_int as byte_hack
                        }
                    }
                    /* HACK - Check for nether immunity and change "Res Neth" */
                    if modetemp == 1 as libc::c_int && x == 1 as libc::c_int
                           && y == 12 as libc::c_int &&
                           (*p_ptr).immune_neth as libc::c_int != 0 {
                        name =
                            b"Imm Neth\x00" as *const u8 as
                                *const libc::c_char;
                        a = 13 as libc::c_int as byte_hack
                    }
                    /* Dump name */
                    if modetemp == 1 as libc::c_int && x == 0 as libc::c_int
                           && y > 7 as libc::c_int && y < 12 as libc::c_int {
                        if got != 0 {
                            Term_putstr(40 as libc::c_int,
                                        y - 4 as libc::c_int,
                                        -(1 as libc::c_int), a, name);
                        }
                    } else {
                        Term_putstr(x * 40 as libc::c_int,
                                    y + 4 as libc::c_int, -(1 as libc::c_int),
                                    a, name);
                    }
                }
                z += 1
            }
            y += 1
        }
        x -= 1
    };
}
/*
 * Display the character on the screen (various modes)
 *
 * The top two and bottom two lines are left blank.
 *
 * Mode 0 = standard display with skills
 * Mode 1 = standard display with history
 * Mode 2 = current flags (part 1)
 * Mode 3 = current flags (part 2)
 * Mode 4 = current flags (part 3)
 * Mode 5 = current flags (part 4)
 * Mode 6 = current flags (part 5 -- esp)
 */
#[no_mangle]
pub unsafe extern "C" fn display_player(mut mode: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 80] = [0; 80];
    /* Erase screen */
    clear_from(0 as libc::c_int);
    /* Standard */
    if mode == 0 as libc::c_int || mode == 1 as libc::c_int {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset((*p_ptr).body_monster as isize) as
                *mut monster_race;
        /* Name, Sex, Race, Class */
        put_str(b"Name  :\x00" as *const u8 as *const libc::c_char,
                2 as libc::c_int, 1 as libc::c_int);
        put_str(b"Sex   :\x00" as *const u8 as *const libc::c_char,
                3 as libc::c_int, 1 as libc::c_int);
        put_str(b"Race  :\x00" as *const u8 as *const libc::c_char,
                4 as libc::c_int, 1 as libc::c_int);
        put_str(b"Class :\x00" as *const u8 as *const libc::c_char,
                5 as libc::c_int, 1 as libc::c_int);
        put_str(b"Body  :\x00" as *const u8 as *const libc::c_char,
                6 as libc::c_int, 1 as libc::c_int);
        put_str(b"God   :\x00" as *const u8 as *const libc::c_char,
                7 as libc::c_int, 1 as libc::c_int);
        c_put_str(14 as libc::c_int as byte_hack,
                  player_name.as_mut_ptr() as cptr, 2 as libc::c_int,
                  9 as libc::c_int);
        if (*p_ptr).body_monster as libc::c_int != 0 as libc::c_int {
            let mut r_ptr_0: *mut monster_race =
                &mut *r_info.offset((*p_ptr).body_monster as isize) as
                    *mut monster_race;
            let mut tmp: [libc::c_char; 12] = [0; 12];
            if (*r_ptr_0).flags1 & 0x4 as libc::c_int as libc::c_uint !=
                   0 as libc::c_int as libc::c_uint {
                strcpy(tmp.as_mut_ptr(),
                       b"Male\x00" as *const u8 as *const libc::c_char);
            } else if (*r_ptr_0).flags1 & 0x8 as libc::c_int as libc::c_uint
                          != 0 as libc::c_int as libc::c_uint {
                strcpy(tmp.as_mut_ptr(),
                       b"Female\x00" as *const u8 as *const libc::c_char);
            } else {
                strcpy(tmp.as_mut_ptr(),
                       b"Neuter\x00" as *const u8 as *const libc::c_char);
            }
            c_put_str(14 as libc::c_int as byte_hack,
                      tmp.as_mut_ptr() as cptr, 3 as libc::c_int,
                      9 as libc::c_int);
        } else {
            c_put_str(14 as libc::c_int as byte_hack, (*sp_ptr).title,
                      3 as libc::c_int, 9 as libc::c_int);
        }
        sprintf(buf.as_mut_ptr(),
                b"%s\x00" as *const u8 as *const libc::c_char,
                get_player_race_name((*p_ptr).prace as libc::c_int,
                                     (*p_ptr).pracem as libc::c_int));
        c_put_str(14 as libc::c_int as byte_hack, buf.as_mut_ptr() as cptr,
                  4 as libc::c_int, 9 as libc::c_int);
        c_put_str(14 as libc::c_int as byte_hack,
                  c_name.offset((*spp_ptr).title as isize) as cptr,
                  5 as libc::c_int, 9 as libc::c_int);
        c_put_str(14 as libc::c_int as byte_hack,
                  r_name.offset((*r_ptr).name as isize) as cptr,
                  6 as libc::c_int, 9 as libc::c_int);
        c_put_str(14 as libc::c_int as byte_hack,
                  (*deity_info.offset((*p_ptr).pgod as isize)).name,
                  7 as libc::c_int, 9 as libc::c_int);
        /* Age, Height, Weight, Social */
        prt_num(b"Age          \x00" as *const u8 as *const libc::c_char,
                (*p_ptr).age as libc::c_int +
                    bst(11520 as libc::c_int * 365 as libc::c_int,
                        turn -
                            11520 as libc::c_int *
                                (42 as libc::c_int + 127 as libc::c_int) *
                                10 as libc::c_int), 2 as libc::c_int,
                32 as libc::c_int, 14 as libc::c_int as byte_hack,
                b"   \x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char);
        prt_num(b"Height       \x00" as *const u8 as *const libc::c_char,
                (*p_ptr).ht as libc::c_int, 3 as libc::c_int,
                32 as libc::c_int, 14 as libc::c_int as byte_hack,
                b"   \x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char);
        prt_num(b"Weight       \x00" as *const u8 as *const libc::c_char,
                (*p_ptr).wt as libc::c_int, 4 as libc::c_int,
                32 as libc::c_int, 14 as libc::c_int as byte_hack,
                b"   \x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char);
        prt_num(b"Social Class \x00" as *const u8 as *const libc::c_char,
                (*p_ptr).sc as libc::c_int, 5 as libc::c_int,
                32 as libc::c_int, 14 as libc::c_int as byte_hack,
                b"   \x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char);
        /* Display the stats */
        i = 0 as libc::c_int;
        while i < 6 as libc::c_int {
            let mut punctuation: libc::c_char =
                if (*p_ptr).stat_max[i as usize] as libc::c_int ==
                       18 as libc::c_int + 100 as libc::c_int {
                    '!' as i32
                } else { ':' as i32 } as libc::c_char;
            /* Special treatment of "injured" stats */
            if ((*p_ptr).stat_cur[i as usize] as libc::c_int) <
                   (*p_ptr).stat_max[i as usize] as libc::c_int {
                let mut value: libc::c_int = 0;
                let mut colour: libc::c_int = 0;
                if (*p_ptr).stat_cnt[i as usize] != 0 {
                    colour = 3 as libc::c_int
                } else { colour = 11 as libc::c_int }
                /* Use lowercase stat name */
                put_str(format(b"%s%c \x00" as *const u8 as
                                   *const libc::c_char,
                               stat_names_reduced[i as usize],
                               punctuation as libc::c_int) as cptr,
                        2 as libc::c_int + i, 61 as libc::c_int);
                /* Get the current stat */
                value = (*p_ptr).stat_use[i as usize] as libc::c_int;
                /* Obtain the current stat (modified) */
                cnv_stat(value, buf.as_mut_ptr());
                /* Display the current stat (modified) */
                c_put_str(colour as byte_hack, buf.as_mut_ptr() as cptr,
                          2 as libc::c_int + i, 66 as libc::c_int);
                /* Acquire the max stat */
                value = (*p_ptr).stat_top[i as usize] as libc::c_int;
                /* Obtain the maximum stat (modified) */
                cnv_stat(value, buf.as_mut_ptr());
                /* Display the maximum stat (modified) */
                c_put_str(13 as libc::c_int as byte_hack,
                          buf.as_mut_ptr() as cptr, 2 as libc::c_int + i,
                          73 as libc::c_int);
            } else {
                /* Normal treatment of "normal" stats */
                /* Assume uppercase stat name */
                put_str(format(b"%s%c \x00" as *const u8 as
                                   *const libc::c_char,
                               stat_names[i as usize],
                               punctuation as libc::c_int) as cptr,
                        2 as libc::c_int + i, 61 as libc::c_int);
                cnv_stat((*p_ptr).stat_use[i as usize] as libc::c_int,
                         buf.as_mut_ptr());
                c_put_str(13 as libc::c_int as byte_hack,
                          buf.as_mut_ptr() as cptr, 2 as libc::c_int + i,
                          66 as libc::c_int);
            }
            i += 1
        }
        /* Obtain the current stat (modified) */
        /* Display the current stat (modified) */
        /* Extra info */
        display_player_middle();
        /* Display "history" info */
        if mode == 1 as libc::c_int {
            put_str(b"(Character Background)\x00" as *const u8 as
                        *const libc::c_char, 15 as libc::c_int,
                    25 as libc::c_int);
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                put_str(history[i as usize].as_mut_ptr() as cptr,
                        i + 16 as libc::c_int, 10 as libc::c_int);
                i += 1
            }
        } else {
            /* Display "various" info */
            put_str(b"(Miscellaneous Abilities)\x00" as *const u8 as
                        *const libc::c_char, 15 as libc::c_int,
                    25 as libc::c_int);
            display_player_various();
        }
    } else {
        /* Special */
        display_player_ben_one(mode - 2 as libc::c_int);
    };
}
/*
 * Utility function; should probably be in some other file...
 *
 * Describe the player's location -- either by dungeon level, town, or in
 * wilderness with landmark reference.
 */
#[no_mangle]
pub unsafe extern "C" fn describe_player_location() -> cptr {
    let mut i: libc::c_int = 0;
    static mut desc: [libc::c_char; 80] = [0; 80];
    let mut pwx: libc::c_int =
        if (*p_ptr).wild_mode as libc::c_int != 0 {
            (*p_ptr).px as libc::c_int
        } else { (*p_ptr).wilderness_x };
    let mut pwy: libc::c_int =
        if (*p_ptr).wild_mode as libc::c_int != 0 {
            (*p_ptr).py as libc::c_int
        } else { (*p_ptr).wilderness_y };
    let mut feat: libc::c_int =
        (*(*wild_map.offset(pwy as isize)).offset(pwx as isize)).feat;
    if dungeon_type as libc::c_int != 0 as libc::c_int &&
           dun_level as libc::c_int > 0 as libc::c_int {
        sprintf(desc.as_mut_ptr(),
                b"on level %d of %s\x00" as *const u8 as *const libc::c_char,
                dun_level as libc::c_int,
                d_name.offset((*d_info.offset(dungeon_type as isize)).name as
                                  isize));
    } else if (*wf_info.offset(feat as isize)).terrain_idx as libc::c_int ==
                  1 as libc::c_int {
        sprintf(desc.as_mut_ptr(),
                b"in the town of %s\x00" as *const u8 as *const libc::c_char,
                wf_name.offset((*wf_info.offset(feat as isize)).name as
                                   isize));
    } else if (*wf_info.offset(feat as isize)).entrance != 0 {
        sprintf(desc.as_mut_ptr(),
                b"near %s\x00" as *const u8 as *const libc::c_char,
                wf_name.offset((*wf_info.offset(feat as isize)).name as
                                   isize));
    } else {
        /*
		 * The complicated case.  Find the nearest known landmark,
		 * and describe our position relative to that.  Note that
		 * we may not even have any known landmarks (for instance,
		 * a Lost Soul character just after escaping the Halls of
		 * Mandos).
		 */
        let mut landmark: libc::c_int = 0 as libc::c_int;
        let mut lwx: libc::c_int = 0 as libc::c_int;
        let mut lwy: libc::c_int = 0 as libc::c_int;
        let mut l_dist: libc::c_int = -(1 as libc::c_int);
        let mut i_0: libc::c_int = 0;
        i_0 = 0 as libc::c_int;
        while i_0 < max_wf_idx as libc::c_int {
            let mut wx: libc::c_int = (*wf_info.offset(i_0 as isize)).wild_x;
            let mut wy: libc::c_int = (*wf_info.offset(i_0 as isize)).wild_y;
            let mut dist: libc::c_int = 0;
            /* Skip if not a landmark */
            if !((*wf_info.offset(i_0 as isize)).entrance == 0) {
                /* Skip if we haven't seen it */
                if !((*(*wild_map.offset(wy as
                                             isize)).offset(wx as
                                                                isize)).known
                         == 0) {
                    dist = distance(wy, wx, pwy, pwx);
                    if dist < l_dist || l_dist < 0 as libc::c_int {
                        landmark = i_0;
                        l_dist = dist;
                        lwx = wx;
                        lwy = wy
                    }
                }
            }
            i_0 += 1
        }
        if landmark == 0 {
            sprintf(desc.as_mut_ptr(),
                    b"in %s\x00" as *const u8 as *const libc::c_char,
                    wf_text.offset((*wf_info.offset(feat as isize)).text as
                                       isize));
        } else if pwx == lwx && pwy == lwy {
            /* Paranoia; this should have been caught above */
            sprintf(desc.as_mut_ptr(),
                    b"near %s\x00" as *const u8 as *const libc::c_char,
                    wf_name.offset((*wf_info.offset(feat as isize)).name as
                                       isize));
        } else {
            /*
			 * We split the circle into eight equal octants of
			 * size pi/4 radians; the "east" octant, for
			 * instance, is defined as due east plus or minus
			 * pi/8 radians.  Now sin(pi/8) ~= 0.3826 ~= 31/81,
			 * so we check |dx|/|dy| and |dy|/|dx| against that
			 * ratio to determine which octant we're in.
			 */
            let mut dx: libc::c_int = pwx - lwx;
            let mut dy: libc::c_int = pwy - lwy;
            let mut ns: cptr =
                if dy > 0 as libc::c_int {
                    b"south\x00" as *const u8 as *const libc::c_char
                } else { b"north\x00" as *const u8 as *const libc::c_char };
            let mut ew: cptr =
                if dx > 0 as libc::c_int {
                    b"east\x00" as *const u8 as *const libc::c_char
                } else { b"west\x00" as *const u8 as *const libc::c_char };
            dx = if dx < 0 as libc::c_int { -dx } else { dx };
            dy = if dy < 0 as libc::c_int { -dy } else { dy };
            if (dy * 81 as libc::c_int) < dx * 31 as libc::c_int {
                ns = b"\x00" as *const u8 as *const libc::c_char
            }
            if (dx * 81 as libc::c_int) < dy * 31 as libc::c_int {
                ew = b"\x00" as *const u8 as *const libc::c_char
            }
            sprintf(desc.as_mut_ptr(),
                    b"in %s %s%s of %s\x00" as *const u8 as
                        *const libc::c_char,
                    wf_text.offset((*wf_info.offset(feat as isize)).text as
                                       isize), ns, ew,
                    wf_name.offset((*wf_info.offset(landmark as isize)).name
                                       as isize));
        }
    }
    /* strip trailing whitespace */
    i = 0 as libc::c_int;
    while desc[i as usize] != 0 { i += 1 }
    loop  {
        i -= 1;
        if !(desc[i as usize] as libc::c_int == ' ' as i32) { break ; }
        desc[i as usize] = 0 as libc::c_int as libc::c_char
    }
    return desc.as_mut_ptr() as cptr;
}
/*
 * Helper function or file_character_print_grid
 *
 * Figure out if a row on the grid is empty
 */
unsafe extern "C" fn file_character_print_grid_check_row(mut buf:
                                                             *const libc::c_char)
 -> bool_ {
    if !strstr(buf.offset(12 as libc::c_int as isize),
               b"+\x00" as *const u8 as *const libc::c_char).is_null() {
        return 1 as libc::c_int as bool_
    }
    if !strstr(buf.offset(12 as libc::c_int as isize),
               b"*\x00" as *const u8 as *const libc::c_char).is_null() {
        return 1 as libc::c_int as bool_
    }
    if !strstr(buf.offset(12 as libc::c_int as isize),
               b"1\x00" as *const u8 as *const libc::c_char).is_null() {
        return 1 as libc::c_int as bool_
    }
    if !strstr(buf.offset(12 as libc::c_int as isize),
               b"2\x00" as *const u8 as *const libc::c_char).is_null() {
        return 1 as libc::c_int as bool_
    }
    if !strstr(buf.offset(12 as libc::c_int as isize),
               b"3\x00" as *const u8 as *const libc::c_char).is_null() {
        return 1 as libc::c_int as bool_
    }
    if !strstr(buf.offset(12 as libc::c_int as isize),
               b"4\x00" as *const u8 as *const libc::c_char).is_null() {
        return 1 as libc::c_int as bool_
    }
    if !strstr(buf.offset(12 as libc::c_int as isize),
               b"5\x00" as *const u8 as *const libc::c_char).is_null() {
        return 1 as libc::c_int as bool_
    }
    if !strstr(buf.offset(12 as libc::c_int as isize),
               b"6\x00" as *const u8 as *const libc::c_char).is_null() {
        return 1 as libc::c_int as bool_
    }
    if !strstr(buf.offset(12 as libc::c_int as isize),
               b"7\x00" as *const u8 as *const libc::c_char).is_null() {
        return 1 as libc::c_int as bool_
    }
    if !strstr(buf.offset(12 as libc::c_int as isize),
               b"8\x00" as *const u8 as *const libc::c_char).is_null() {
        return 1 as libc::c_int as bool_
    }
    if !strstr(buf.offset(12 as libc::c_int as isize),
               b"9\x00" as *const u8 as *const libc::c_char).is_null() {
        return 1 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
/*
 * Helper function for file_character
 *
 * Prints the big ugly grid
 */
unsafe extern "C" fn file_character_print_grid(mut fff: *mut FILE,
                                               mut show_gaps: bool_,
                                               mut show_legend: bool_) {
    static mut blank_line: cptr =
        b"                                        \x00" as *const u8 as
            *const libc::c_char;
    static mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut a: byte_hack = 0;
    let mut c: libc::c_char = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    y =
        if show_legend as libc::c_int != 0 {
            3 as libc::c_int
        } else { 4 as libc::c_int };
    while y < 23 as libc::c_int {
        x = 0 as libc::c_int;
        while x < 40 as libc::c_int {
            Term_what(x, y, &mut a, &mut c);
            buf[x as usize] = c;
            x += 1
        }
        buf[x as usize] = '\u{0}' as i32 as libc::c_char;
        if strcmp(buf.as_mut_ptr(), blank_line) != 0 &&
               (y == 3 as libc::c_int || show_gaps as libc::c_int != 0 ||
                    file_character_print_grid_check_row(buf.as_mut_ptr()) as
                        libc::c_int != 0) {
            fprintf(fff,
                    b"        %s\n\x00" as *const u8 as *const libc::c_char,
                    buf.as_mut_ptr());
        }
        y += 1
    }
    y = 4 as libc::c_int;
    while y < 23 as libc::c_int {
        x = 40 as libc::c_int;
        while x < 80 as libc::c_int {
            Term_what(x, y, &mut a, &mut c);
            buf[(x - 40 as libc::c_int) as usize] = c;
            x += 1
        }
        buf[x as usize] = '\u{0}' as i32 as libc::c_char;
        if strcmp(buf.as_mut_ptr(), blank_line) != 0 &&
               (show_gaps as libc::c_int != 0 ||
                    file_character_print_grid_check_row(buf.as_mut_ptr()) as
                        libc::c_int != 0) {
            fprintf(fff,
                    b"        %s\n\x00" as *const u8 as *const libc::c_char,
                    buf.as_mut_ptr());
        }
        y += 1
    };
}
/*
 * Helper function for file_character
 *
 * Outputs one item (for Inventory, Equipment, Home, and Mathom-house)
 */
#[no_mangle]
pub unsafe extern "C" fn file_character_print_item(mut fff: *mut FILE,
                                                   mut label: libc::c_char,
                                                   mut obj: *mut object_type,
                                                   mut full: bool_) {
    static mut o_name: [libc::c_char; 80] = [0; 80];
    static mut paren: cptr = b")\x00" as *const u8 as *const libc::c_char;
    object_desc(o_name.as_mut_ptr(), obj, 1 as libc::c_int, 3 as libc::c_int);
    fprintf(fff, b"%c%s %s\n\x00" as *const u8 as *const libc::c_char,
            label as libc::c_int, paren, o_name.as_mut_ptr());
    if ((*obj).tval as libc::c_int == 102 as libc::c_int ||
            (if (*obj).name1 as libc::c_int != 0 {
                 1 as libc::c_int
             } else { 0 as libc::c_int }) != 0 ||
            (if (*obj).art_name as libc::c_int != 0 {
                 1 as libc::c_int
             } else { 0 as libc::c_int }) != 0 ||
            (if (*k_info.offset((*obj).k_idx as isize)).flags3 as libc::c_long
                    & 0x8000 as libc::c_long != 0 {
                 1 as libc::c_int
             } else { 0 as libc::c_int }) != 0 ||
            (if (*obj).name2 as libc::c_int != 0 ||
                    (*obj).name2b as libc::c_int != 0 {
                 1 as libc::c_int
             } else { 0 as libc::c_int }) != 0 ||
            (*obj).tval as libc::c_int == 45 as libc::c_int ||
            (*obj).tval as libc::c_int == 40 as libc::c_int ||
            full as libc::c_int != 0) &&
           (*obj).ident as libc::c_int & 0x20 as libc::c_int != 0 {
        object_out_desc(obj, fff, 1 as libc::c_int as bool_,
                        1 as libc::c_int as bool_);
    };
}
/*
 * Helper function for file_character
 *
 * Prints out one "store" (for Home and Mathom-house)
 */
#[no_mangle]
pub unsafe extern "C" fn file_character_print_store(mut fff: *mut FILE,
                                                    mut place:
                                                        *mut wilderness_type_info,
                                                    mut store: libc::c_int,
                                                    mut full: bool_) {
    let mut i: libc::c_int = 0;
    let mut town: *mut town_type =
        &mut *town_info.offset((*place).entrance as isize) as *mut town_type;
    let mut st_ptr: *mut store_type =
        &mut *(*town).store.offset(store as isize) as *mut store_type;
    if (*st_ptr).stock_num != 0 {
        /* Header with name of the town */
        fprintf(fff,
                b"  [%s Inventory - %s]\n\n\x00" as *const u8 as
                    *const libc::c_char,
                st_name.offset((*st_info.offset(store as isize)).name as
                                   isize),
                wf_name.offset((*place).name as isize));
        /* Dump all available items */
        i = 0 as libc::c_int;
        while i < (*st_ptr).stock_num as libc::c_int {
            file_character_print_item(fff,
                                      (i % 24 as libc::c_int + 'a' as i32) as
                                          libc::c_char,
                                      &mut *(*st_ptr).stock.offset(i as
                                                                       isize),
                                      full);
            i += 1
        }
        /* Add an empty line */
        fprintf(fff, b"\n\n\x00" as *const u8 as *const libc::c_char);
    };
}
/*
 * Helper function for file_character
 *
 * Checks if the store hasn't been added to the list yet, and then adds it if it
 * was not already there.  XXX This is an ugly workaround for the double Gondolin
 * problem.
 *
 * Beware of the ugly pointer gymnastics.
 */
#[no_mangle]
pub unsafe extern "C" fn file_character_check_stores(mut store_list:
                                                         *mut *mut *mut store_type,
                                                     mut store_list_count:
                                                         *mut libc::c_int,
                                                     mut place:
                                                         *mut wilderness_type_info,
                                                     mut store: libc::c_int)
 -> bool_ {
    let mut town: *mut town_type =
        &mut *town_info.offset((*place).entrance as isize) as *mut town_type;
    let mut st_ptr: *mut store_type =
        &mut *(*town).store.offset(store as isize) as *mut store_type;
    let mut head: *mut *mut store_type = *store_list;
    let mut i: libc::c_int = 0;
    /* check the list for this store */
    i = 0 as libc::c_int;
    while i < *store_list_count {
        if *head == st_ptr { return 0 as libc::c_int as bool_ }
        head = head.offset(1);
        i += 1
    }
    /* make room for the new one in the array */
    if !(*store_list).is_null() {
        head = *store_list;
        *store_list =
            ralloc(((*store_list_count + 1 as libc::c_int) as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut store_type>()
                                                        as libc::c_ulong)) as
                *mut *mut store_type;
        memcpy(*store_list as *mut libc::c_char as *mut libc::c_void,
               head as *mut libc::c_char as *const libc::c_void,
               (*store_list_count as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut store_type>()
                                                    as libc::c_ulong));
        rnfree(head as vptr,
               (*store_list_count as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut store_type>()
                                                    as libc::c_ulong));
    } else {
        *store_list =
            ralloc(::std::mem::size_of::<*mut store_type>() as libc::c_ulong)
                as *mut *mut store_type
    }
    /* update data */
    let ref mut fresh9 = *(*store_list).offset(*store_list_count as isize);
    *fresh9 = st_ptr;
    *store_list_count += 1;
    return 1 as libc::c_int as bool_;
}
/*
 * Hack -- Dump a character description file
 *
 * XXX XXX XXX Allow the "full" flag to dump additional info,
 * and trigger its usage from various places in the code.
 */
#[no_mangle]
pub unsafe extern "C" fn file_character(mut name: cptr, mut full: bool_)
 -> errr {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut a: byte_hack = 0;
    let mut c: libc::c_char = 0;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut store_list: *mut *mut store_type = 0 as *mut *mut store_type;
    let mut store_list_count: libc::c_int = 0 as libc::c_int;
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_USER, name);
    /* File type is "TEXT" */
    /* Check for existing file */
    fd = fd_open(buf.as_mut_ptr() as cptr, 0 as libc::c_int);
    /* Existing file */
    if fd >= 0 as libc::c_int {
        let mut out_val: [libc::c_char; 160] = [0; 160];
        /* Close the file */
        fd_close(fd);
        /* Build query */
        sprintf(out_val.as_mut_ptr(),
                b"Replace existing file %s? \x00" as *const u8 as
                    *const libc::c_char, buf.as_mut_ptr());
        /* Ask */
        if get_check(out_val.as_mut_ptr() as cptr) != 0 {
            fd = -(1 as libc::c_int)
        }
    }
    /* Open the non-existing file */
    if fd < 0 as libc::c_int {
        fff =
            my_fopen(buf.as_mut_ptr() as cptr,
                     b"w\x00" as *const u8 as *const libc::c_char)
    }
    /* Invalid file */
    if fff.is_null() {
        /* Message */
        msg_format(b"Character sheet creation failed!\x00" as *const u8 as
                       *const libc::c_char);
        msg_print(0 as cptr);
        /* Error */
        return -(1 as libc::c_int)
    }
    /* Begin dump */
    fprintf(fff,
            b"  [%s Character Sheet]\n\n\x00" as *const u8 as
                *const libc::c_char, get_version_string());
    /* Display player */
    display_player(0 as libc::c_int);
    /* Dump part of the screen */
    y = 2 as libc::c_int;
    while y < 22 as libc::c_int {
        /* Dump each row */
        x = 0 as libc::c_int;
        while x < 79 as libc::c_int {
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
    /* Display history */
    display_player(1 as libc::c_int);
    /* Dump part of the screen */
    y = 15 as libc::c_int;
    while y < 20 as libc::c_int {
        /* Dump each row */
        x = 0 as libc::c_int;
        while x < 79 as libc::c_int {
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
    /* List the patches */
    hook_file = fff;
    exec_lua(b"patchs_list()\x00" as *const u8 as *const libc::c_char as
                 *mut libc::c_char);
    fprintf(fff,
            b"\n\n  [Miscellaneous information]\n\x00" as *const u8 as
                *const libc::c_char);
    if joke_monsters != 0 {
        fprintf(fff,
                b"\n Joke monsters:        ON\x00" as *const u8 as
                    *const libc::c_char);
    } else {
        fprintf(fff,
                b"\n Joke monsters:        OFF\x00" as *const u8 as
                    *const libc::c_char);
    }
    if (*p_ptr).maximize != 0 {
        fprintf(fff,
                b"\n Maximize mode:        ON\x00" as *const u8 as
                    *const libc::c_char);
    } else {
        fprintf(fff,
                b"\n Maximize mode:        OFF\x00" as *const u8 as
                    *const libc::c_char);
    }
    if (*p_ptr).preserve != 0 {
        fprintf(fff,
                b"\n Preserve Mode:        ON\x00" as *const u8 as
                    *const libc::c_char);
    } else {
        fprintf(fff,
                b"\n Preserve Mode:        OFF\x00" as *const u8 as
                    *const libc::c_char);
    }
    if auto_scum != 0 {
        fprintf(fff,
                b"\n Autoscum:             ON\x00" as *const u8 as
                    *const libc::c_char);
    } else {
        fprintf(fff,
                b"\n Autoscum:             OFF\x00" as *const u8 as
                    *const libc::c_char);
    }
    if always_small_level != 0 {
        fprintf(fff,
                b"\n Small Levels:         ALWAYS\x00" as *const u8 as
                    *const libc::c_char);
    } else if small_levels != 0 {
        fprintf(fff,
                b"\n Small Levels:         ON\x00" as *const u8 as
                    *const libc::c_char);
    } else {
        fprintf(fff,
                b"\n Small Levels:         OFF\x00" as *const u8 as
                    *const libc::c_char);
    }
    if empty_levels != 0 {
        fprintf(fff,
                b"\n Arena Levels:         ON\x00" as *const u8 as
                    *const libc::c_char);
    } else {
        fprintf(fff,
                b"\n Arena Levels:         OFF\x00" as *const u8 as
                    *const libc::c_char);
    }
    if ironman_rooms != 0 {
        fprintf(fff,
                b"\n Always unusual rooms: ON\x00" as *const u8 as
                    *const libc::c_char);
    } else {
        fprintf(fff,
                b"\n Always unusual rooms: OFF\x00" as *const u8 as
                    *const libc::c_char);
    }
    fprintf(fff,
            b"\n\n Recall Depth:\x00" as *const u8 as *const libc::c_char);
    y = 1 as libc::c_int;
    while y < max_d_idx as libc::c_int {
        if *max_dlv.offset(y as isize) != 0 {
            fprintf(fff,
                    b"\n        %s: Level %d (%d\')\x00" as *const u8 as
                        *const libc::c_char,
                    d_name.offset((*d_info.offset(y as isize)).name as isize),
                    *max_dlv.offset(y as isize) as libc::c_int,
                    50 as libc::c_int *
                        *max_dlv.offset(y as isize) as libc::c_int);
        }
        y += 1
    }
    fprintf(fff, b"\n\x00" as *const u8 as *const libc::c_char);
    if noscore != 0 {
        fprintf(fff,
                b"\n You have done something illegal.\x00" as *const u8 as
                    *const libc::c_char);
    }
    if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
            (*spp_ptr).flags1) as libc::c_long & 0x1 as libc::c_long != 0 {
        fprintf(fff,
                b"\n You have done something experimental.\x00" as *const u8
                    as *const libc::c_char);
    }
    if stupid_monsters != 0 {
        fprintf(fff,
                b"\n Your opponents are behaving stupidly.\x00" as *const u8
                    as *const libc::c_char);
    }
    let mut desc: [libc::c_char; 80] = [0; 80];
    let mut mimic: cptr = 0 as *const libc::c_char;
    monster_race_desc(desc.as_mut_ptr(), (*p_ptr).body_monster as libc::c_int,
                      0 as libc::c_int);
    fprintf(fff,
            b"\n Your body %s %s.\x00" as *const u8 as *const libc::c_char,
            if death as libc::c_int != 0 {
                b"was\x00" as *const u8 as *const libc::c_char
            } else { b"is\x00" as *const u8 as *const libc::c_char },
            desc.as_mut_ptr());
    if (*p_ptr).tim_mimic != 0 {
        call_lua(b"get_mimic_info\x00" as *const u8 as *const libc::c_char,
                 b"(d,s)\x00" as *const u8 as *const libc::c_char,
                 b"s\x00" as *const u8 as *const libc::c_char,
                 (*p_ptr).mimic_form as libc::c_int,
                 b"name\x00" as *const u8 as *const libc::c_char,
                 &mut mimic as *mut cptr);
        fprintf(fff,
                b"\n You %s disguised as a %s.\x00" as *const u8 as
                    *const libc::c_char,
                if death as libc::c_int != 0 {
                    b"were\x00" as *const u8 as *const libc::c_char
                } else { b"are\x00" as *const u8 as *const libc::c_char },
                mimic);
    }
    /* Where we are, if we're alive */
    if death == 0 {
        fprintf(fff,
                b"\n You are currently %s.\x00" as *const u8 as
                    *const libc::c_char, describe_player_location());
    }
    /* Monsters slain */
    let mut k: libc::c_int = 0;
    let mut Total: s32b = 0 as libc::c_int;
    k = 1 as libc::c_int;
    while k < max_r_idx as libc::c_int {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset(k as isize) as *mut monster_race;
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
        k += 1
    }
    if Total < 1 as libc::c_int {
        fprintf(fff,
                b"\n You have defeated no enemies yet.\x00" as *const u8 as
                    *const libc::c_char);
    } else if Total == 1 as libc::c_int {
        fprintf(fff,
                b"\n You have defeated one enemy.\x00" as *const u8 as
                    *const libc::c_char);
    } else {
        fprintf(fff,
                b"\n You have defeated %ld enemies.\x00" as *const u8 as
                    *const libc::c_char, Total as libc::c_long);
    }
    hook_file = fff;
    process_hooks(12 as libc::c_int,
                  b"()\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char);
    /* Date */
    let mut buf2: [libc::c_char; 20] = [0; 20];
    let mut days: u32b =
        bst(11520 as libc::c_int,
            turn -
                11520 as libc::c_int *
                    (42 as libc::c_int + 127 as libc::c_int) *
                    10 as libc::c_int) as u32b;
    strnfmt(buf2.as_mut_ptr(), 20 as libc::c_int as uint_hack,
            get_day(bst(11520 as libc::c_int * 365 as libc::c_int,
                        11520 as libc::c_int *
                            (42 as libc::c_int + 127 as libc::c_int) *
                            10 as libc::c_int) + 2890 as libc::c_int));
    fprintf(fff,
            b"\n\n You started your adventure the %s of the %s year of the third age.\x00"
                as *const u8 as *const libc::c_char,
            get_month_name(bst(11520 as libc::c_int,
                               11520 as libc::c_int *
                                   (42 as libc::c_int + 127 as libc::c_int) *
                                   10 as libc::c_int), wizard,
                           0 as libc::c_int as bool_), buf2.as_mut_ptr());
    strnfmt(buf2.as_mut_ptr(), 20 as libc::c_int as uint_hack,
            get_day(bst(11520 as libc::c_int * 365 as libc::c_int, turn) +
                        2890 as libc::c_int));
    fprintf(fff,
            b"\n %s the %s of the %s year of the third age.\x00" as *const u8
                as *const libc::c_char,
            if death as libc::c_int != 0 {
                b"You ended your adventure\x00" as *const u8 as
                    *const libc::c_char
            } else {
                b"It is currently\x00" as *const u8 as *const libc::c_char
            },
            get_month_name(bst(11520 as libc::c_int, turn), wizard,
                           0 as libc::c_int as bool_), buf2.as_mut_ptr());
    fprintf(fff,
            if death as libc::c_int != 0 {
                b"\n Your adventure lasted %ld day%s.\x00" as *const u8 as
                    *const libc::c_char
            } else {
                b"\n You have been adventuring for %ld day%s.\x00" as
                    *const u8 as *const libc::c_char
            }, days as libc::c_long,
            if days == 1 as libc::c_int as libc::c_uint {
                b"\x00" as *const u8 as *const libc::c_char
            } else { b"s\x00" as *const u8 as *const libc::c_char });
    fprintf(fff, b"\n\n\x00" as *const u8 as *const libc::c_char);
    /* Emit the self-knowledge lines, even though they duplicate the
	   information in the grids (below), because they contain information
	   that's not in the grids (racial abilities, luck, etc.). */
    if full != 0 {
        self_knowledge(fff);
        fprintf(fff, b"\n\n\x00" as *const u8 as *const libc::c_char);
    }
    /* adds and slays */
    display_player(2 as libc::c_int);
    file_character_print_grid(fff, 0 as libc::c_int as bool_,
                              1 as libc::c_int as bool_);
    /* sustains and resistances */
    display_player(3 as libc::c_int);
    file_character_print_grid(fff, 1 as libc::c_int as bool_,
                              0 as libc::c_int as bool_);
    /* stuff */
    display_player(4 as libc::c_int);
    file_character_print_grid(fff, 0 as libc::c_int as bool_,
                              0 as libc::c_int as bool_);
    /* a little bit of stuff */
    display_player(5 as libc::c_int);
    file_character_print_grid(fff, 0 as libc::c_int as bool_,
                              0 as libc::c_int as bool_);
    /* Dump corruptions */
    if got_corruptions() != 0 {
        fprintf(fff,
                b"\n Corruption list:\n\x00" as *const u8 as
                    *const libc::c_char);
        dump_corruptions(fff, 0 as libc::c_int as bool_);
    }
    /* Dump skills */
    dump_skills(fff);
    dump_abilities(fff);
    /* Dump companions. */
    dump_companions(fff);
    if (*p_ptr).companion_killed != 0 {
        if (*p_ptr).companion_killed as libc::c_int == 1 as libc::c_int {
            fprintf(fff,
                    b"\n One of your companion(s) has been killed.\x00" as
                        *const u8 as *const libc::c_char);
        } else {
            fprintf(fff,
                    b"\n %d of your companions have been killed.\x00" as
                        *const u8 as *const libc::c_char,
                    (*p_ptr).companion_killed as libc::c_int);
        }
    }
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        if fates[i as usize].fate as libc::c_int != 0 &&
               fates[i as usize].know as libc::c_int != 0 {
            fprintf(fff,
                    b"\n\n  [Fates]\n\n\x00" as *const u8 as
                        *const libc::c_char);
            dump_fates(fff);
            break ;
        } else { i += 1 }
    }
    /* Skip some lines */
    fprintf(fff, b"\n\n\x00" as *const u8 as *const libc::c_char);
    /* Dump the equipment */
    text_out_indent = 4 as libc::c_int;
    if equip_cnt != 0 {
        fprintf(fff,
                b"  [Character Equipment]\n\n\x00" as *const u8 as
                    *const libc::c_char);
        i = 24 as libc::c_int;
        while i < 52 as libc::c_int {
            if !((*p_ptr).body_parts[(i - 24 as libc::c_int) as usize] == 0) {
                file_character_print_item(fff, index_to_label(i),
                                          &mut *(*p_ptr).inventory.as_mut_ptr().offset(i
                                                                                           as
                                                                                           isize),
                                          full);
            }
            i += 1
        }
        fprintf(fff, b"\n\n\x00" as *const u8 as *const libc::c_char);
    }
    /* Dump the inventory */
    fprintf(fff,
            b"  [Character Inventory]\n\n\x00" as *const u8 as
                *const libc::c_char);
    i = 0 as libc::c_int;
    while i < 23 as libc::c_int {
        file_character_print_item(fff, index_to_label(i),
                                  &mut *(*p_ptr).inventory.as_mut_ptr().offset(i
                                                                                   as
                                                                                   isize),
                                  full);
        i += 1
    }
    fprintf(fff, b"\n\n\x00" as *const u8 as *const libc::c_char);
    /* Print all homes in the different towns */
    j = 0 as libc::c_int;
    while j < max_wf_idx as libc::c_int {
        if (*wf_info.offset(j as isize)).feat as libc::c_int ==
               0xcb as libc::c_int &&
               file_character_check_stores(&mut store_list,
                                           &mut store_list_count,
                                           &mut *wf_info.offset(j as isize),
                                           7 as libc::c_int) as libc::c_int !=
                   0 {
            file_character_print_store(fff, &mut *wf_info.offset(j as isize),
                                       7 as libc::c_int, full);
        }
        j += 1
    }
    store_list =
        rnfree(store_list as vptr,
               (store_list_count as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut store_type>()
                                                    as libc::c_ulong)) as
            *mut *mut store_type;
    store_list_count = 0 as libc::c_int;
    /* Print all Mathom-houses in the different towns */
    j = 0 as libc::c_int;
    while j < max_wf_idx as libc::c_int {
        if (*wf_info.offset(j as isize)).feat as libc::c_int ==
               0xcb as libc::c_int &&
               file_character_check_stores(&mut store_list,
                                           &mut store_list_count,
                                           &mut *wf_info.offset(j as isize),
                                           57 as libc::c_int) as libc::c_int
                   != 0 {
            file_character_print_store(fff, &mut *wf_info.offset(j as isize),
                                       57 as libc::c_int, full);
        }
        j += 1
    }
    store_list =
        rnfree(store_list as vptr,
               (store_list_count as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut store_type>()
                                                    as libc::c_ulong)) as
            *mut *mut store_type;
    store_list_count = 0 as libc::c_int;
    text_out_indent = 0 as libc::c_int;
    /* Close it */
    my_fclose(fff);
    /* Message */
    msg_print(b"Character sheet creation successful.\x00" as *const u8 as
                  *const libc::c_char);
    msg_print(0 as cptr);
    /* Success */
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn show_file(mut name: cptr, mut what: cptr,
                                   mut line: libc::c_int,
                                   mut mode: libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut link_color: byte_hack = 3 as libc::c_int as byte_hack;
    let mut link_color_sel: byte_hack = 11 as libc::c_int as byte_hack;
    /* Number of "real" lines passed by */
    let mut next: libc::c_int = 0 as libc::c_int;
    /* Number of "real" lines in the file */
    let mut size: libc::c_int = 0 as libc::c_int;
    /* Backup value for "line" */
    let mut back: libc::c_int = 0 as libc::c_int;
    /* Color of the next line */
    let mut color: byte_hack = 1 as libc::c_int as byte_hack;
    /* This screen has sub-screens */
    let mut menu: bool_ = 0 as libc::c_int as bool_;
    /* Current help file */
    let mut fff: *mut FILE = 0 as *mut FILE;
    /* Find this string (if any) */
    let mut find: cptr = 0 as cptr;
    /* Char array type of hyperlink info */
    let mut h_ptr: *mut hyperlink_type = 0 as *mut hyperlink_type;
    /* Pointer to general buffer in the above */
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cur_link: libc::c_int = 0 as libc::c_int;
    let mut max_link: libc::c_int = 0 as libc::c_int;
    /* Read size of screen for big-screen stuff -pav- */
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    /* Allocate hyperlink data */
    h_ptr =
        memset(ralloc(::std::mem::size_of::<hyperlink_type>() as
                          libc::c_ulong) as *mut libc::c_char as
                   *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<hyperlink_type>() as libc::c_ulong) as
            *mut hyperlink_type;
    /* Setup buffer pointer */
    buf = (*h_ptr).rbuf.as_mut_ptr();
    /* Wipe the links */
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        (*h_ptr).link_x[i as usize] = -(1 as libc::c_int);
        i += 1
    }
    /* Hack XXX XXX XXX */
    if !what.is_null() {
        /* h_ptr->caption */
        strcpy((*h_ptr).caption.as_mut_ptr(), what);
        /* Access the "file" */
        strcpy((*h_ptr).path.as_mut_ptr(), name);
        /* Open */
        fff =
            my_fopen((*h_ptr).path.as_mut_ptr() as cptr,
                     b"r\x00" as *const u8 as *const libc::c_char)
    }
    /* Look in "help" */
    if fff.is_null() {
        /* h_ptr->caption */
        sprintf((*h_ptr).caption.as_mut_ptr(),
                b"Help file \'%s\'\x00" as *const u8 as *const libc::c_char,
                name);
        /* Build the filename */
        path_build((*h_ptr).path.as_mut_ptr(), 1024 as libc::c_int,
                   ANGBAND_DIR_HELP, name);
        /* Open the file */
        fff =
            my_fopen((*h_ptr).path.as_mut_ptr() as cptr,
                     b"r\x00" as *const u8 as *const libc::c_char)
    }
    /* Look in "info" */
    if fff.is_null() {
        /* h_ptr->caption */
        sprintf((*h_ptr).caption.as_mut_ptr(),
                b"Info file \'%s\'\x00" as *const u8 as *const libc::c_char,
                name);
        /* Build the filename */
        path_build((*h_ptr).path.as_mut_ptr(), 1024 as libc::c_int,
                   ANGBAND_DIR_INFO, name);
        /* Open the file */
        fff =
            my_fopen((*h_ptr).path.as_mut_ptr() as cptr,
                     b"r\x00" as *const u8 as *const libc::c_char)
    }
    /* Look in "file" */
    if fff.is_null() {
        /* h_ptr->caption */
        sprintf((*h_ptr).caption.as_mut_ptr(),
                b"File \'%s\'\x00" as *const u8 as *const libc::c_char, name);
        /* Build the filename */
        path_build((*h_ptr).path.as_mut_ptr(), 1024 as libc::c_int,
                   ANGBAND_DIR_FILE, name);
        /* Open the file */
        fff =
            my_fopen((*h_ptr).path.as_mut_ptr() as cptr,
                     b"r\x00" as *const u8 as *const libc::c_char)
    }
    /* Oops */
    if fff.is_null() {
        /* Message */
        msg_format(b"Cannot open \'%s\'.\x00" as *const u8 as
                       *const libc::c_char, name);
        msg_print(0 as cptr);
        /* Free hyperlink info */
        h_ptr =
            rnfree(h_ptr as vptr,
                   ::std::mem::size_of::<hyperlink_type>() as libc::c_ulong)
                as *mut hyperlink_type;
        /* Oops */
        return 1 as libc::c_int as bool_
    }
    /* Pre-Parse the file */
    /* Read a line or stop */
    while !(my_fgets(fff, (*h_ptr).rbuf.as_mut_ptr(),
                     1024 as libc::c_int as huge_hack) != 0) {
        /* Get a color */
        if prefix((*h_ptr).rbuf.as_mut_ptr() as cptr,
                  b"#####\x00" as *const u8 as *const libc::c_char) != 0 {
            buf =
                &mut *(*h_ptr).rbuf.as_mut_ptr().offset(6 as libc::c_int as
                                                            isize) as
                    *mut libc::c_char
        } else { buf = (*h_ptr).rbuf.as_mut_ptr() }
        /* Get the link colors */
        if prefix(buf as cptr,
                  b"|||||\x00" as *const u8 as *const libc::c_char) != 0 {
            link_color =
                color_char_to_attr(*buf.offset(5 as libc::c_int as isize)) as
                    byte_hack;
            link_color_sel =
                color_char_to_attr(*buf.offset(6 as libc::c_int as isize)) as
                    byte_hack
        }
        /* Tag ? */
        if prefix(buf as cptr,
                  b"~~~~~\x00" as *const u8 as *const libc::c_char) != 0 {
            if line < 0 as libc::c_int {
                let mut i_0: libc::c_int = 0;
                let mut old_c: libc::c_char = 0;
                i_0 = 5 as libc::c_int;
                while *buf.offset(i_0 as isize) as libc::c_int >= '0' as i32
                          &&
                          *buf.offset(i_0 as isize) as libc::c_int <=
                              '9' as i32 {
                    i_0 += 1
                }
                old_c = *buf.offset(i_0 as isize);
                *buf.offset(i_0 as isize) = '\u{0}' as i32 as libc::c_char;
                if atoi(buf.offset(5 as libc::c_int as isize)) == -line {
                    line = next + 1 as libc::c_int
                }
                *buf.offset(i_0 as isize) = old_c
            }
        }
        x = 0 as libc::c_int;
        while *buf.offset(x as isize) != 0 {
            /* Hyperlink ? */
            if prefix(buf.offset(x as isize) as cptr,
                      b"*****\x00" as *const u8 as *const libc::c_char) != 0 {
                let mut xx: libc::c_int = x + 5 as libc::c_int;
                let mut stmp: libc::c_int = 0;
                let mut xdeb: libc::c_int = x + 5 as libc::c_int;
                let mut z: libc::c_int = 0;
                let mut tmp: [libc::c_char; 20] = [0; 20];
                z = 0 as libc::c_int;
                while z < 20 as libc::c_int {
                    tmp[z as usize] = '\u{0}' as i32 as libc::c_char;
                    z += 1
                }
                (*h_ptr).link_x[max_link as usize] = x;
                (*h_ptr).link_y[max_link as usize] = next;
                if *buf.offset(xx as isize) as libc::c_int == '/' as i32 {
                    xx += 1;
                    (*h_ptr).link_key[max_link as usize] =
                        *buf.offset(xx as isize);
                    xx += 1;
                    xdeb += 2 as libc::c_int
                } else {
                    (*h_ptr).link_key[max_link as usize] =
                        0 as libc::c_int as libc::c_char
                }
                /* Zap the link info */
                while *buf.offset(xx as isize) as libc::c_int != '*' as i32 {
                    (*h_ptr).link[max_link as usize][(xx - xdeb) as usize] =
                        *buf.offset(xx as isize);
                    xx += 1
                }
                (*h_ptr).link[max_link as usize][(xx - xdeb) as usize] =
                    '\u{0}' as i32 as libc::c_char;
                xx += 1;
                stmp = xx;
                while *buf.offset(xx as isize) as libc::c_int != '[' as i32 {
                    tmp[(xx - stmp) as usize] = *buf.offset(xx as isize);
                    xx += 1
                }
                xx += 1;
                tmp[(xx - stmp) as usize] = '\u{0}' as i32 as libc::c_char;
                (*h_ptr).link_line[max_link as usize] =
                    -atoi(tmp.as_mut_ptr());
                max_link += 1
            }
            x += 1
        }
        /* Count the "real" lines */
        next += 1
    }
    /* Save the number of "real" lines */
    size = next;
    loop 
         /* Display the file */
         /* Clear screen */
         {
        Term_clear();
        Term_get_size(&mut wid, &mut hgt);
        /* Restart when necessary */
        if line >= size { line = 0 as libc::c_int }
        /* Re-open the file if needed */
        if next > line {
            /* Close it */
            my_fclose(fff);
            /* Hack -- Re-Open the file */
            fff =
                my_fopen((*h_ptr).path.as_mut_ptr() as cptr,
                         b"r\x00" as *const u8 as *const libc::c_char);
            /* Oops */
            if fff.is_null() {
                /* Free hyperlink info */
                h_ptr =
                    rnfree(h_ptr as vptr,
                           ::std::mem::size_of::<hyperlink_type>() as
                               libc::c_ulong) as *mut hyperlink_type;
                return 0 as libc::c_int as bool_
            }
            /* File has been restarted */
            next = 0 as libc::c_int
        }
        /* Skip lines if needed */
        while next < line {
            /* Skip a line */
            if my_fgets(fff, buf, 1024 as libc::c_int as huge_hack) != 0 {
                break ;
            }
            next += 1
        }
        /* Dump the next 20 (or more in bigscreen) lines of the file */
        i = 0 as libc::c_int;
        while i < hgt - 4 as libc::c_int {
            let mut print_x: libc::c_int = 0;
            /* Hack -- track the "first" line */
            if i == 0 { line = next }
            /* Get a line of the file or stop */
            if my_fgets(fff, (*h_ptr).rbuf.as_mut_ptr(),
                        1024 as libc::c_int as huge_hack) != 0 {
                break ;
            }
            /* Get a color */
            if prefix((*h_ptr).rbuf.as_mut_ptr() as cptr,
                      b"#####\x00" as *const u8 as *const libc::c_char) != 0 {
                color =
                    color_char_to_attr((*h_ptr).rbuf[5 as libc::c_int as
                                                         usize]) as byte_hack;
                buf =
                    &mut *(*h_ptr).rbuf.as_mut_ptr().offset(6 as libc::c_int
                                                                as isize) as
                        *mut libc::c_char
            } else { buf = (*h_ptr).rbuf.as_mut_ptr() }
            /* Count the "real" lines */
            next += 1;
            /* Skip link colors */
            if prefix(buf as cptr,
                      b"|||||\x00" as *const u8 as *const libc::c_char) != 0 {
                continue ;
            }
            /* Skip tags */
            if prefix(buf as cptr,
                      b"~~~~~\x00" as *const u8 as *const libc::c_char) != 0 {
                i += 1
            } else {
                /* Hack -- keep searching */
                if !find.is_null() && i == 0 && strstr(buf, find).is_null() {
                    continue ;
                }
                /* Hack -- stop searching */
                find = 0 as cptr;
                /* Be sure to get a correct cur_link */
                if (*h_ptr).link_y[cur_link as usize] >=
                       line + (hgt - 4 as libc::c_int) {
                    while cur_link > 0 as libc::c_int &&
                              (*h_ptr).link_y[cur_link as usize] >=
                                  line + (hgt - 4 as libc::c_int) {
                        cur_link -= 1
                    }
                }
                if (*h_ptr).link_y[cur_link as usize] < line {
                    while cur_link < max_link &&
                              (*h_ptr).link_y[cur_link as usize] < line {
                        cur_link += 1
                    }
                }
                /* Dump the line */
                print_x = 0 as libc::c_int;
                if prefix(buf as cptr,
                          b"&&&&&\x00" as *const u8 as *const libc::c_char) ==
                       0 {
                    x = 0 as libc::c_int;
                    while *buf.offset(x as isize) != 0 {
                        /* Hyperlink ? */
                        if prefix(buf.offset(x as isize) as cptr,
                                  b"*****\x00" as *const u8 as
                                      *const libc::c_char) != 0 {
                            let mut xx_0: libc::c_int = x + 5 as libc::c_int;
                            /* Zap the link info */
                            while *buf.offset(xx_0 as isize) as libc::c_int !=
                                      '[' as i32 {
                                xx_0 += 1
                            }
                            xx_0 += 1;
                            /* Ok print the link name */
                            while *buf.offset(xx_0 as isize) as libc::c_int !=
                                      ']' as i32 {
                                let mut color_0: byte_hack = link_color;
                                if (*h_ptr).link_x[cur_link as usize] == x &&
                                       (*h_ptr).link_y[cur_link as usize] ==
                                           line + i {
                                    color_0 = link_color_sel
                                }
                                /* Now we treat the next char as printable */
                                if *buf.offset(xx_0 as isize) as libc::c_int
                                       == '\\' as i32 {
                                    xx_0 += 1
                                }
                                Term_putch(print_x, i + 2 as libc::c_int,
                                           color_0,
                                           *buf.offset(xx_0 as isize));
                                xx_0 += 1;
                                print_x += 1
                            }
                            x = xx_0
                        } else if prefix(buf.offset(x as isize) as cptr,
                                         b"[[[[[\x00" as *const u8 as
                                             *const libc::c_char) != 0 {
                            let mut xx_1: libc::c_int = x + 6 as libc::c_int;
                            /* Color ? */
                            /* Ok print the link name */
                            while *buf.offset(xx_1 as isize) as libc::c_int !=
                                      ']' as i32 {
                                /* Now we treat the next char as printable */
                                if *buf.offset(xx_1 as isize) as libc::c_int
                                       == '\\' as i32 {
                                    xx_1 += 1
                                }
                                Term_putch(print_x, i + 2 as libc::c_int,
                                           color_char_to_attr(*buf.offset((x +
                                                                               5
                                                                                   as
                                                                                   libc::c_int)
                                                                              as
                                                                              isize))
                                               as byte_hack,
                                           *buf.offset(xx_1 as isize));
                                xx_1 += 1;
                                print_x += 1
                            }
                            x = xx_1
                        } else if prefix(buf.offset(x as isize) as cptr,
                                         b"{{{{{\x00" as *const u8 as
                                             *const libc::c_char) != 0 {
                            let mut xx_2: libc::c_int = x + 6 as libc::c_int;
                            /* Remove HTML ? */
                            /* Ok remove this section */
                            while *buf.offset(xx_2 as isize) as libc::c_int !=
                                      '}' as i32 {
                                xx_2 += 1
                            }
                            x = xx_2
                        } else {
                            Term_putch(print_x, i + 2 as libc::c_int, color,
                                       *buf.offset(x as isize));
                            print_x += 1
                        }
                        x += 1
                    }
                } else {
                    /* Verbatim mode: i.e: acacacac */
                    x = 5 as libc::c_int;
                    while *buf.offset(x as isize) != 0 {
                        Term_putch(print_x, i + 2 as libc::c_int,
                                   color_char_to_attr(*buf.offset(x as isize))
                                       as byte_hack,
                                   *buf.offset((x + 1 as libc::c_int) as
                                                   isize));
                        print_x += 1;
                        x += 2 as libc::c_int
                    }
                }
                color = 1 as libc::c_int as byte_hack;
                /* Hilite "h_ptr->shower" */
                if (*h_ptr).shower[0 as libc::c_int as usize] != 0 {
                    let mut str: cptr = buf as cptr;
                    loop 
                         /* Display matches */
                         {
                        str =
                            strstr(str, (*h_ptr).shower.as_mut_ptr()) as cptr;
                        if str.is_null() { break ; }
                        let mut len: libc::c_int =
                            strlen((*h_ptr).shower.as_mut_ptr()) as
                                libc::c_int;
                        /* Display the match */
                        Term_putstr(str.wrapping_offset_from(buf) as
                                        libc::c_long as libc::c_int,
                                    i + 2 as libc::c_int, len,
                                    11 as libc::c_int as byte_hack,
                                    (*h_ptr).shower.as_mut_ptr() as cptr);
                        /* Advance */
                        str = str.offset(len as isize)
                    }
                }
                /* Count the printed lines */
                i += 1
            }
        }
        /* Hack -- failed search */
        if !find.is_null() {
            bell();
            line = back;
            find = 0 as cptr
        } else {
            /* Show a general "title" */
            prt(format(b"[%s, %s, Line %d/%d]\x00" as *const u8 as
                           *const libc::c_char, get_version_string(),
                       (*h_ptr).caption.as_mut_ptr(), line, size) as cptr,
                0 as libc::c_int, 0 as libc::c_int);
            /* Prompt -- menu screen */
            if menu != 0 {
                /* Wait for it */
                prt(b"[Press a Number, or ESC to exit.]\x00" as *const u8 as
                        *const libc::c_char, hgt - 1 as libc::c_int,
                    0 as libc::c_int);
            } else if size <= hgt - 4 as libc::c_int {
                /* Prompt -- small files */
                /* Wait for it */
                prt(b"[Press ESC to exit.]\x00" as *const u8 as
                        *const libc::c_char, hgt - 1 as libc::c_int,
                    0 as libc::c_int);
            } else {
                /* Prompt -- large files */
                /* Wait for it */
                prt(b"[Press 2, 8, 4, 6, /, =, #, %, backspace, or ESC to exit.]\x00"
                        as *const u8 as *const libc::c_char,
                    hgt - 1 as libc::c_int, 0 as libc::c_int);
            }
            /* Get a keypress */
            k = inkey() as libc::c_int;
            /* Hack -- return to last screen */
            if k == '?' as i32 || k == 0x7f as libc::c_int ||
                   k == '\u{8}' as i32 {
                break ;
            }
            /* Hack -- try showing */
            if k == '=' as i32 {
                /* Get "h_ptr->shower" */
                prt(b"Show: \x00" as *const u8 as *const libc::c_char,
                    hgt - 1 as libc::c_int, 0 as libc::c_int);
                askfor_aux((*h_ptr).shower.as_mut_ptr(), 80 as libc::c_int);
            }
            /* Hack -- try finding */
            if k == '/' as i32 {
                /* Get "h_ptr->finder" */
                prt(b"Find: \x00" as *const u8 as *const libc::c_char,
                    hgt - 1 as libc::c_int, 0 as libc::c_int);
                if askfor_aux((*h_ptr).finder.as_mut_ptr(), 80 as libc::c_int)
                       != 0 {
                    /* Find it */
                    find = (*h_ptr).finder.as_mut_ptr() as cptr;
                    back = line;
                    line = line + 1 as libc::c_int;
                    /* Show it */
                    strcpy((*h_ptr).shower.as_mut_ptr(),
                           (*h_ptr).finder.as_mut_ptr());
                }
            }
            /* Hack -- go to a specific line */
            if k == '#' as i32 {
                let mut tmp_0: [libc::c_char; 81] = [0; 81];
                prt(b"Goto Line: \x00" as *const u8 as *const libc::c_char,
                    hgt - 1 as libc::c_int, 0 as libc::c_int);
                strcpy(tmp_0.as_mut_ptr(),
                       b"0\x00" as *const u8 as *const libc::c_char);
                if askfor_aux(tmp_0.as_mut_ptr(), 80 as libc::c_int) != 0 {
                    line = atoi(tmp_0.as_mut_ptr())
                }
            }
            /* Hack -- go to a specific file */
            if k == '%' as i32 {
                let mut tmp_1: [libc::c_char; 81] = [0; 81];
                prt(b"Goto File: \x00" as *const u8 as *const libc::c_char,
                    hgt - 1 as libc::c_int, 0 as libc::c_int);
                strcpy(tmp_1.as_mut_ptr(),
                       b"help.hlp\x00" as *const u8 as *const libc::c_char);
                if askfor_aux(tmp_1.as_mut_ptr(), 80 as libc::c_int) != 0 {
                    if show_file(tmp_1.as_mut_ptr() as cptr, 0 as cptr,
                                 0 as libc::c_int, mode) == 0 {
                        k = '\u{1b}' as i32
                    }
                }
            }
            /* Hack -- Allow backing up */
            if k == '-' as i32 {
                line = line - (hgt - 4 as libc::c_int);
                if line < 0 as libc::c_int { line = 0 as libc::c_int }
            }
            if k == '8' as i32 {
                line -= 1;
                if line < 0 as libc::c_int { line = 0 as libc::c_int }
            }
            /* Hack -- Advance a single line */
            if k == '2' as i32 { line = line + 1 as libc::c_int }
            /* Advance one page */
            if k == ' ' as i32 { line = line + (hgt - 4 as libc::c_int) }
            /* Advance one link */
            if k == '6' as i32 || k == '\t' as i32 {
                cur_link += 1;
                if cur_link >= max_link {
                    cur_link = max_link - 1 as libc::c_int
                }
                if (*h_ptr).link_y[cur_link as usize] < line {
                    line = (*h_ptr).link_y[cur_link as usize]
                }
                if (*h_ptr).link_y[cur_link as usize] >=
                       line + (hgt - 4 as libc::c_int) {
                    line =
                        (*h_ptr).link_y[cur_link as usize] -
                            (hgt - 4 as libc::c_int)
                }
            }
            /* Return one link */
            if k == '4' as i32 {
                cur_link -= 1;
                if cur_link < 0 as libc::c_int { cur_link = 0 as libc::c_int }
                if (*h_ptr).link_y[cur_link as usize] < line {
                    line = (*h_ptr).link_y[cur_link as usize]
                }
                if (*h_ptr).link_y[cur_link as usize] >=
                       line + (hgt - 4 as libc::c_int) {
                    line =
                        (*h_ptr).link_y[cur_link as usize] -
                            (hgt - 4 as libc::c_int)
                }
            }
            /* Recurse on numbers */
            if k == '\r' as i32 {
                if (*h_ptr).link_x[cur_link as usize] != -(1 as libc::c_int) {
                    /* Recurse on that file */
                    if show_file((*h_ptr).link[cur_link as usize].as_mut_ptr()
                                     as cptr, 0 as cptr,
                                 (*h_ptr).link_line[cur_link as usize], mode)
                           == 0 {
                        k = '\u{1b}' as i32
                    }
                }
            }
            /* Exit on escape */
            if k == '\u{1b}' as i32 { break ; }
            /* No other key ? lets look for a shortcut */
            i = 0 as libc::c_int;
            while i < max_link {
                if (*h_ptr).link_key[i as usize] as libc::c_int == k {
                    /* Recurse on that file */
                    if show_file((*h_ptr).link[i as usize].as_mut_ptr() as
                                     cptr, 0 as cptr,
                                 (*h_ptr).link_line[i as usize], mode) == 0 {
                        k = '\u{1b}' as i32
                    }
                    break ;
                } else { i += 1 }
            }
        }
    }
    /* Close the file */
    my_fclose(fff);
    /* Free hyperlink buffers */
    h_ptr =
        rnfree(h_ptr as vptr,
               ::std::mem::size_of::<hyperlink_type>() as libc::c_ulong) as
            *mut hyperlink_type;
    /* Escape */
    if k == '\u{1b}' as i32 { return 0 as libc::c_int as bool_ }
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn txt_to_html(mut head: cptr, mut foot: cptr,
                                     mut base: cptr, mut ext: cptr,
                                     mut force: bool_, mut recur: bool_)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    /* Normal return */
    /* Number of "real" lines passed by */
    let mut next: libc::c_int = 0 as libc::c_int;
    let mut buf_name: [libc::c_char; 80] = [0; 80];
    /* Color of the next line */
    let mut color: byte_hack = 1 as libc::c_int as byte_hack;
    /* Current help file */
    let mut fff: *mut FILE = 0 as *mut FILE;
    /* Current aux file */
    let mut aux: *mut FILE = 0 as *mut FILE;
    /* Current html file */
    let mut htm: *mut FILE = 0 as *mut FILE;
    /* Char array type of hyperlink info */
    let mut h_ptr: *mut hyperlink_type = 0 as *mut hyperlink_type;
    let mut file_ext: cptr = 0 as *const libc::c_char;
    let mut link_prefix: cptr = 0 as *const libc::c_char;
    let mut link_suffix: cptr = 0 as *const libc::c_char;
    /* Pointer to general buffer in the above */
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Allocate hyperlink data */
    h_ptr =
        memset(ralloc(::std::mem::size_of::<hyperlink_type>() as
                          libc::c_ulong) as *mut libc::c_char as
                   *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<hyperlink_type>() as libc::c_ulong) as
            *mut hyperlink_type;
    /* Setup buffer pointer */
    buf = (*h_ptr).rbuf.as_mut_ptr();
    /* Wipe the links */
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        (*h_ptr).link_x[i as usize] = -(1 as libc::c_int);
        i += 1
    }
    /* Parse it(yeah lua is neat :) */
    tome_dofile_anywhere(ANGBAND_DIR_HELP,
                         b"def.aux\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char, 1 as libc::c_int as bool_);
    /* Ok now get the parameters */
    file_ext =
        string_exec_lua(b"return file_ext\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char);
    link_prefix =
        string_exec_lua(b"return link_prefix\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char);
    link_suffix =
        string_exec_lua(b"return link_suffix\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char);
    sprintf(buf_name.as_mut_ptr(),
            b"%s.%s\x00" as *const u8 as *const libc::c_char, base, file_ext);
    if force == 0 && file_exist(buf_name.as_mut_ptr()) as libc::c_int != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Build the filename */
    path_build((*h_ptr).path.as_mut_ptr(), 1024 as libc::c_int,
               ANGBAND_DIR_HELP, buf_name.as_mut_ptr() as cptr);
    /* Open the file */
    htm =
        my_fopen((*h_ptr).path.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    sprintf(buf_name.as_mut_ptr(),
            b"%s.%s\x00" as *const u8 as *const libc::c_char, base, ext);
    /* h_ptr->caption */
    sprintf((*h_ptr).caption.as_mut_ptr(),
            b"Help file \'%s\'\x00" as *const u8 as *const libc::c_char,
            buf_name.as_mut_ptr());
    /* Build the filename */
    path_build((*h_ptr).path.as_mut_ptr(), 1024 as libc::c_int,
               ANGBAND_DIR_HELP, buf_name.as_mut_ptr() as cptr);
    /* Open the file */
    fff =
        my_fopen((*h_ptr).path.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Oops */
    if fff.is_null() || htm.is_null() {
        /* Free hyperlink info */
        h_ptr =
            rnfree(h_ptr as vptr,
                   ::std::mem::size_of::<hyperlink_type>() as libc::c_ulong)
                as *mut hyperlink_type;
        my_fclose(fff);
        my_fclose(htm);
        /* Oops */
        return 1 as libc::c_int as bool_
    }
    /* Build the filename */
    path_build((*h_ptr).path.as_mut_ptr(), 1024 as libc::c_int,
               ANGBAND_DIR_HELP, head);
    /* Open the file */
    aux =
        my_fopen((*h_ptr).path.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Copy the header */
    if !aux.is_null() {
        loop  {
            let mut find: *mut libc::c_char = 0 as *mut libc::c_char;
            if my_fgets(aux, (*h_ptr).rbuf.as_mut_ptr(),
                        1024 as libc::c_int as huge_hack) != 0 {
                break ;
            }
            find =
                strstr((*h_ptr).rbuf.as_mut_ptr(),
                       b"%t\x00" as *const u8 as *const libc::c_char);
            if !find.is_null() {
                *find = '\u{0}' as i32 as libc::c_char;
                find = find.offset(2 as libc::c_int as isize);
                fprintf(htm, b"%s\x00" as *const u8 as *const libc::c_char,
                        (*h_ptr).rbuf.as_mut_ptr());
                fprintf(htm, b"%s\x00" as *const u8 as *const libc::c_char,
                        base);
                fprintf(htm, b"%s\n\x00" as *const u8 as *const libc::c_char,
                        find);
            } else {
                fprintf(htm, b"%s\n\x00" as *const u8 as *const libc::c_char,
                        (*h_ptr).rbuf.as_mut_ptr());
            }
        }
        my_fclose(aux);
    }
    loop 
         /* Display the file */
         {
        let mut do_color: bool_ = 0 as libc::c_int as bool_;
        /* Skip a line */
        if my_fgets(fff, (*h_ptr).rbuf.as_mut_ptr(),
                    1024 as libc::c_int as huge_hack) != 0 {
            break ;
        }
        color = 1 as libc::c_int as byte_hack;
        let mut print_x: libc::c_int = 0;
        /* Get a color */
        if prefix((*h_ptr).rbuf.as_mut_ptr() as cptr,
                  b"#####\x00" as *const u8 as *const libc::c_char) != 0 {
            color =
                color_char_to_attr((*h_ptr).rbuf[5 as libc::c_int as usize])
                    as byte_hack;
            do_color = 1 as libc::c_int as bool_;
            fprintf(htm,
                    b"<FONT COLOR=\"#%02X%02X%02X\">\x00" as *const u8 as
                        *const libc::c_char,
                    angband_color_table[color as
                                            usize][1 as libc::c_int as usize]
                        as libc::c_int,
                    angband_color_table[color as
                                            usize][2 as libc::c_int as usize]
                        as libc::c_int,
                    angband_color_table[color as
                                            usize][3 as libc::c_int as usize]
                        as libc::c_int);
            buf =
                &mut *(*h_ptr).rbuf.as_mut_ptr().offset(6 as libc::c_int as
                                                            isize) as
                    *mut libc::c_char
        } else { buf = (*h_ptr).rbuf.as_mut_ptr() }
        /* Count the "real" lines */
        next += 1;
        /* Skip link colors */
        if prefix(buf as cptr,
                  b"|||||\x00" as *const u8 as *const libc::c_char) != 0 {
            continue ;
        }
        /* Skip tags */
        if prefix(buf as cptr,
                  b"~~~~~\x00" as *const u8 as *const libc::c_char) != 0 {
            let mut i_0: libc::c_int = 0;
            i_0 = 5 as libc::c_int;
            while *buf.offset(i_0 as isize) as libc::c_int >= '0' as i32 &&
                      *buf.offset(i_0 as isize) as libc::c_int <= '9' as i32 {
                i_0 += 1
            }
            *buf.offset(i_0 as isize) = '\u{0}' as i32 as libc::c_char;
            fprintf(htm,
                    b"<A NAME=\"%s\"></A>\x00" as *const u8 as
                        *const libc::c_char,
                    buf.offset(5 as libc::c_int as isize));
        } else {
            /* Dump the line */
            print_x = 0 as libc::c_int;
            if prefix(buf as cptr,
                      b"&&&&&\x00" as *const u8 as *const libc::c_char) == 0 {
                x = 0 as libc::c_int;
                while *buf.offset(x as isize) != 0 {
                    /* Hyperlink ? */
                    if prefix(buf.offset(x as isize) as cptr,
                              b"*****\x00" as *const u8 as
                                  *const libc::c_char) != 0 {
                        let mut xx: libc::c_int = x + 5 as libc::c_int;
                        let mut z: libc::c_int = 0 as libc::c_int;
                        let mut buff: [libc::c_char; 80] = [0; 80];
                        let mut link_line: [libc::c_char; 80] = [0; 80];
                        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
                        if *buf.offset(xx as isize) as libc::c_int ==
                               '/' as i32 {
                            xx += 2 as libc::c_int
                        }
                        /* Zap the link info */
                        while *buf.offset(xx as isize) as libc::c_int !=
                                  '*' as i32 {
                            let fresh10 = z;
                            z = z + 1;
                            buff[fresh10 as usize] = *buf.offset(xx as isize);
                            xx += 1
                        }
                        xx += 1;
                        buff[z as usize] = '\u{0}' as i32 as libc::c_char;
                        /* Zap the link info */
                        z = 0 as libc::c_int;
                        while *buf.offset(xx as isize) as libc::c_int !=
                                  '[' as i32 {
                            let fresh11 = z;
                            z = z + 1;
                            link_line[fresh11 as usize] =
                                *buf.offset(xx as isize);
                            xx += 1
                        }
                        xx += 1;
                        link_line[z as usize] =
                            '\u{0}' as i32 as libc::c_char;
                        /* parse it */
                        s = buff.as_mut_ptr();
                        while *s as libc::c_int != '.' as i32 {
                            s = s.offset(1)
                        }
                        *s = '\u{0}' as i32 as libc::c_char;
                        s = s.offset(1);
                        if recur != 0 {
                            txt_to_html(head, foot, buff.as_mut_ptr() as cptr,
                                        s as cptr, 0 as libc::c_int as bool_,
                                        recur);
                        }
                        if atoi(link_line.as_mut_ptr()) != 0 {
                            fprintf(htm,
                                    b"<A HREF=\"%s%s.%s%s#%d\">\x00" as
                                        *const u8 as *const libc::c_char,
                                    link_prefix, buff.as_mut_ptr(), file_ext,
                                    link_suffix,
                                    atoi(link_line.as_mut_ptr()));
                        } else {
                            fprintf(htm,
                                    b"<A HREF=\"%s%s.%s%s\">\x00" as *const u8
                                        as *const libc::c_char, link_prefix,
                                    buff.as_mut_ptr(), file_ext, link_suffix);
                        }
                        /* Ok print the link name */
                        while *buf.offset(xx as isize) as libc::c_int !=
                                  ']' as i32 {
                            /* Now we treat the next char as printable */
                            if *buf.offset(xx as isize) as libc::c_int ==
                                   '\\' as i32 {
                                xx += 1
                            }
                            fprintf(htm,
                                    b"%c\x00" as *const u8 as
                                        *const libc::c_char,
                                    *buf.offset(xx as isize) as libc::c_int);
                            xx += 1;
                            print_x += 1
                        }
                        x = xx;
                        fprintf(htm,
                                b"</A>\x00" as *const u8 as
                                    *const libc::c_char);
                    } else if prefix(buf.offset(x as isize) as cptr,
                                     b"[[[[[\x00" as *const u8 as
                                         *const libc::c_char) != 0 {
                        let mut xx_0: libc::c_int = x + 6 as libc::c_int;
                        color =
                            color_char_to_attr(*buf.offset((x +
                                                                5 as
                                                                    libc::c_int)
                                                               as isize)) as
                                byte_hack;
                        fprintf(htm,
                                b"<FONT COLOR=\"#%02X%02X%02X\">\x00" as
                                    *const u8 as *const libc::c_char,
                                angband_color_table[color as
                                                        usize][1 as
                                                                   libc::c_int
                                                                   as usize]
                                    as libc::c_int,
                                angband_color_table[color as
                                                        usize][2 as
                                                                   libc::c_int
                                                                   as usize]
                                    as libc::c_int,
                                angband_color_table[color as
                                                        usize][3 as
                                                                   libc::c_int
                                                                   as usize]
                                    as libc::c_int);
                        /* Color ? */
                        /* Ok print the link name */
                        while *buf.offset(xx_0 as isize) as libc::c_int !=
                                  ']' as i32 {
                            /* Now we treat the next char as printable */
                            if *buf.offset(xx_0 as isize) as libc::c_int ==
                                   '\\' as i32 {
                                xx_0 += 1
                            }
                            fprintf(htm,
                                    b"%c\x00" as *const u8 as
                                        *const libc::c_char,
                                    *buf.offset(xx_0 as isize) as
                                        libc::c_int);
                            xx_0 += 1;
                            print_x += 1
                        }
                        x += 1;
                        x = xx_0;
                        fprintf(htm,
                                b"</FONT>\x00" as *const u8 as
                                    *const libc::c_char);
                    } else if prefix(buf.offset(x as isize) as cptr,
                                     b"{{{{{\x00" as *const u8 as
                                         *const libc::c_char) != 0 {
                        let mut xx_1: libc::c_int = x + 5 as libc::c_int;
                        /* Hidden HTML tag? */
                        /* Ok output the tag inside */
                        while *buf.offset(xx_1 as isize) as libc::c_int !=
                                  '}' as i32 {
                            fprintf(htm,
                                    b"%c\x00" as *const u8 as
                                        *const libc::c_char,
                                    *buf.offset(xx_1 as isize) as
                                        libc::c_int);
                            xx_1 += 1
                        }
                        x += 1;
                        x = xx_1
                    } else {
                        fprintf(htm,
                                b"%c\x00" as *const u8 as *const libc::c_char,
                                *buf.offset(x as isize) as libc::c_int);
                        print_x += 1
                    }
                    x += 1
                }
            } else {
                /* Verbatim mode: i.e: acacacac */
                let mut old_color: byte_hack = 0;
                x = 5 as libc::c_int;
                old_color =
                    color_char_to_attr(*buf.offset(x as isize)) as byte_hack;
                fprintf(htm,
                        b"<FONT COLOR=\"#%02X%02X%02X\">\x00" as *const u8 as
                            *const libc::c_char,
                        angband_color_table[color as
                                                usize][1 as libc::c_int as
                                                           usize] as
                            libc::c_int,
                        angband_color_table[color as
                                                usize][2 as libc::c_int as
                                                           usize] as
                            libc::c_int,
                        angband_color_table[color as
                                                usize][3 as libc::c_int as
                                                           usize] as
                            libc::c_int);
                while *buf.offset(x as isize) != 0 {
                    color =
                        color_char_to_attr(*buf.offset(x as isize)) as
                            byte_hack;
                    if color as libc::c_int != old_color as libc::c_int {
                        fprintf(htm,
                                b"</FONT><FONT COLOR=\"#%02X%02X%02X\">\x00"
                                    as *const u8 as *const libc::c_char,
                                angband_color_table[color as
                                                        usize][1 as
                                                                   libc::c_int
                                                                   as usize]
                                    as libc::c_int,
                                angband_color_table[color as
                                                        usize][2 as
                                                                   libc::c_int
                                                                   as usize]
                                    as libc::c_int,
                                angband_color_table[color as
                                                        usize][3 as
                                                                   libc::c_int
                                                                   as usize]
                                    as libc::c_int);
                    }
                    fprintf(htm,
                            b"%c\x00" as *const u8 as *const libc::c_char,
                            *buf.offset((x + 1 as libc::c_int) as isize) as
                                libc::c_int);
                    print_x += 1;
                    x += 2 as libc::c_int
                }
                fprintf(htm,
                        b"</FONT>\x00" as *const u8 as *const libc::c_char);
            }
            if do_color != 0 {
                fprintf(htm,
                        b"</FONT>\x00" as *const u8 as *const libc::c_char);
            }
            fprintf(htm, b"\n\x00" as *const u8 as *const libc::c_char);
        }
    }
    /* Build the filename */
    path_build((*h_ptr).path.as_mut_ptr(), 1024 as libc::c_int,
               ANGBAND_DIR_HELP, foot);
    /* Open the file */
    aux =
        my_fopen((*h_ptr).path.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Copy the footer */
    if !aux.is_null() {
        while !(my_fgets(aux, (*h_ptr).rbuf.as_mut_ptr(),
                         1024 as libc::c_int as huge_hack) != 0) {
            fprintf(htm, b"%s\n\x00" as *const u8 as *const libc::c_char,
                    (*h_ptr).rbuf.as_mut_ptr());
        }
        my_fclose(aux);
    }
    /* Close the file */
    my_fclose(htm);
    my_fclose(fff);
    /* Free hyperlink buffers */
    h_ptr =
        rnfree(h_ptr as vptr,
               ::std::mem::size_of::<hyperlink_type>() as libc::c_ulong) as
            *mut hyperlink_type;
    /* Normal return */
    return 1 as libc::c_int as bool_;
}
/* Take an help file screenshot(yes yes I know..) */
#[no_mangle]
pub unsafe extern "C" fn help_file_screenshot(mut name: cptr) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    let mut a: byte_hack = 0 as libc::c_int as byte_hack;
    let mut c: libc::c_char = ' ' as i32 as libc::c_char;
    let mut htm: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* The terms package supports up to 255x255 screen size */
    let mut abuf: [libc::c_char; 256] = [0; 256];
    let mut cbuf: [libc::c_char; 256] = [0; 256];
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_USER, name);
    /* File type is "TEXT" */
    /* Append to the file */
    htm =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    /* Oops */
    if htm.is_null() { return }
    /* Retrieve current screen size */
    Term_get_size(&mut wid, &mut hgt);
    /* Dump the screen */
    y = 0 as libc::c_int;
    while y < hgt {
        cmovie_clean_line(y, abuf.as_mut_ptr(), cbuf.as_mut_ptr());
        /* Dump each row */
        fprintf(htm, b"&&&&&\x00" as *const u8 as *const libc::c_char);
        x = 0 as libc::c_int;
        while x < wid {
            a = abuf[x as usize] as byte_hack;
            c = cbuf[x as usize];
            fprintf(htm, b"%c%c\x00" as *const u8 as *const libc::c_char,
                    a as libc::c_int, c as libc::c_int);
            x += 1
        }
        /* End the row */
        fprintf(htm, b"\n\x00" as *const u8 as *const libc::c_char);
        y += 1
    }
    /* Close it */
    my_fclose(htm);
}
/* Take an html screenshot */
#[no_mangle]
pub unsafe extern "C" fn html_screenshot(mut name: cptr) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    let mut a: byte_hack = 0 as libc::c_int as byte_hack;
    let mut oa: byte_hack = 1 as libc::c_int as byte_hack;
    let mut c: libc::c_char = ' ' as i32 as libc::c_char;
    let mut htm: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* The terms package supports up to 255x255 screen size */
    let mut abuf: [libc::c_char; 256] = [0; 256];
    let mut cbuf: [libc::c_char; 256] = [0; 256];
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_USER, name);
    /* File type is "TEXT" */
    /* Append to the file */
    htm =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    /* Oops */
    if htm.is_null() { return }
    /* Retrieve current screen size */
    Term_get_size(&mut wid, &mut hgt);
    fprintf(htm,
            b"<?xml version=\"1.0\" encoding=\"iso-8859-1\"?>\n<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Strict//EN\" \"DTD/xhtml1-strict.dtd\">\n<html xmlns=\"http://www.w3.org/1999/xhtml\">\n<head>\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(htm,
            b"<meta name=\"GENERATOR\" content=\"%s\"/>\n\x00" as *const u8 as
                *const libc::c_char, get_version_string());
    fprintf(htm,
            b"<title>%s</title>\n\x00" as *const u8 as *const libc::c_char,
            name);
    fprintf(htm,
            b"</head>\n<body>\n<pre style=\"color: #ffffff; background-color: #000000; font-family: monospace\">\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(htm,
            b"<span style=\"color: #%02X%02X%02X\">\n\x00" as *const u8 as
                *const libc::c_char,
            angband_color_table[1 as libc::c_int as
                                    usize][1 as libc::c_int as usize] as
                libc::c_int,
            angband_color_table[1 as libc::c_int as
                                    usize][2 as libc::c_int as usize] as
                libc::c_int,
            angband_color_table[1 as libc::c_int as
                                    usize][3 as libc::c_int as usize] as
                libc::c_int);
    /* Dump the screen */
    y = 0 as libc::c_int;
    while y < hgt {
        cmovie_clean_line(y, abuf.as_mut_ptr(), cbuf.as_mut_ptr());
        /* Dump each row */
        x = 0 as libc::c_int;
        while x < wid {
            a = color_char_to_attr(abuf[x as usize]) as byte_hack;
            c = cbuf[x as usize];
            if oa as libc::c_int != a as libc::c_int {
                fprintf(htm,
                        b"</span><span style=\"color: #%02X%02X%02X\">\x00" as
                            *const u8 as *const libc::c_char,
                        angband_color_table[a as
                                                usize][1 as libc::c_int as
                                                           usize] as
                            libc::c_int,
                        angband_color_table[a as
                                                usize][2 as libc::c_int as
                                                           usize] as
                            libc::c_int,
                        angband_color_table[a as
                                                usize][3 as libc::c_int as
                                                           usize] as
                            libc::c_int);
                oa = a
            }
            if c as libc::c_int == '<' as i32 {
                fprintf(htm, b"&lt;\x00" as *const u8 as *const libc::c_char);
            } else if c as libc::c_int == '>' as i32 {
                fprintf(htm, b"&gt;\x00" as *const u8 as *const libc::c_char);
            } else if c as libc::c_int == '&' as i32 {
                fprintf(htm,
                        b"&amp;\x00" as *const u8 as *const libc::c_char);
            } else {
                fprintf(htm, b"%c\x00" as *const u8 as *const libc::c_char,
                        c as libc::c_int);
            }
            x += 1
        }
        /* End the row */
        fprintf(htm, b"\n\x00" as *const u8 as *const libc::c_char);
        y += 1
    }
    fprintf(htm,
            b"</span>\n</pre>\n</body>\n</html>\n\x00" as *const u8 as
                *const libc::c_char);
    /* Close it */
    my_fclose(htm);
}
/*
 * Peruse the On-Line-Help
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_help() {
    /* Save screen */
    screen_save();
    /* Peruse the main help file */
    show_file(b"help.hlp\x00" as *const u8 as *const libc::c_char, 0 as cptr,
              0 as libc::c_int, 0 as libc::c_int);
    /* Load screen */
    screen_load();
}
/*
 * Process the player name.
 * Extract a clean "base name".
 * Build the savefile name if needed.
 */
#[no_mangle]
pub unsafe extern "C" fn process_player_base() {
    let mut temp: [libc::c_char; 128] = [0; 128];
    /* Rename the savefile, using the player_base */
    sprintf(temp.as_mut_ptr(), b"%s\x00" as *const u8 as *const libc::c_char,
            player_base.as_mut_ptr());
    /* Build the filename */
    path_build(savefile.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_SAVE,
               temp.as_mut_ptr() as cptr);
}
#[no_mangle]
pub unsafe extern "C" fn process_player_name(mut sf: bool_) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut tmp: [libc::c_char; 50] = [0; 50];
    /* Cannot be too long */
    if strlen(player_base.as_mut_ptr()) > 15 as libc::c_int as libc::c_ulong {
        /* Name too long */
        quit_fmt(b"The name \'%s\' is too long!\x00" as *const u8 as
                     *const libc::c_char, player_base.as_mut_ptr());
    }
    /* Cannot contain "icky" characters */
    i = 0 as libc::c_int;
    while player_base[i as usize] != 0 {
        /* No control characters */
        if *(*__ctype_b_loc()).offset(player_base[i as usize] as libc::c_int
                                          as isize) as libc::c_int &
               _IScntrl as libc::c_int as libc::c_ushort as libc::c_int != 0 {
            /* Illegal characters */
            quit_fmt(b"The name \'%s\' contains control chars!\x00" as
                         *const u8 as *const libc::c_char,
                     player_base.as_mut_ptr());
        }
        i += 1
    }
    /* Extract "useful" letters */
    i = 0 as libc::c_int;
    while player_base[i as usize] != 0 {
        let mut c: libc::c_char = player_base[i as usize];
        /* Accept some letters */
        if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as
               libc::c_int &
               _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
               ||
               *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as
                   libc::c_int &
                   _ISdigit as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 {
            let fresh12 = k;
            k = k + 1;
            tmp[fresh12 as usize] = c
        } else if !strchr(b"@. _\x00" as *const u8 as *const libc::c_char,
                          c as libc::c_int).is_null() {
            let fresh13 = k;
            k = k + 1;
            tmp[fresh13 as usize] = '_' as i32 as libc::c_char
        }
        i += 1
    }
    /* Convert space, dot, and underscore to underscore */
    /* Terminate */
    tmp[k as usize] = '\u{0}' as i32 as libc::c_char;
    sprintf(player_base.as_mut_ptr(),
            b"%s\x00" as *const u8 as *const libc::c_char, tmp.as_mut_ptr());
    /* Require a "base" name */
    if player_base[0 as libc::c_int as usize] == 0 {
        strcpy(player_base.as_mut_ptr(),
               b"PLAYER\x00" as *const u8 as *const libc::c_char);
    }
    /* Change the savefile name */
    if sf != 0 { process_player_base(); };
}
/*
 * Gets a name for the character, reacting to name changes.
 *
 * Assumes that "display_player(0)" has just been called
 *
 * Perhaps we should NOT ask for a name (at "birth()") on
 * Unix machines?  XXX XXX
 *
 * What a horrible name for a global function.  XXX XXX XXX
 */
#[no_mangle]
pub unsafe extern "C" fn get_name() {
    let mut tmp: [libc::c_char; 32] = [0; 32];
    /* Clear last line */
    clear_from(22 as libc::c_int);
    /* Prompt and ask */
    prt(b"[Enter your player\'s name above, or hit ESCAPE]\x00" as *const u8
            as *const libc::c_char, 23 as libc::c_int, 2 as libc::c_int);
    /* Ask until happy */
    /* Go to the "name" field */
    move_cursor(2 as libc::c_int, 9 as libc::c_int);
    strcpy(tmp.as_mut_ptr(), player_name.as_mut_ptr());
    if askfor_aux(tmp.as_mut_ptr(), 31 as libc::c_int) != 0 {
        strcpy(player_name.as_mut_ptr(), tmp.as_mut_ptr());
    }
    process_player_name(0 as libc::c_int as bool_);
    /* Save the player name */
    /* Get an input, ignore "Escape" */
    /* Process the player name */
    /* Pad the name (to clear junk) */
    sprintf(tmp.as_mut_ptr(),
            b"%-31.31s\x00" as *const u8 as *const libc::c_char,
            player_name.as_mut_ptr());
    /* Re-Draw the name (in light blue) */
    c_put_str(14 as libc::c_int as byte_hack, tmp.as_mut_ptr() as cptr,
              2 as libc::c_int, 9 as libc::c_int);
    /* Erase the prompt, etc */
    clear_from(22 as libc::c_int);
}
/*
 * Hack -- commit suicide
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_suicide() {
    let mut i: libc::c_int = 0;
    /* Flush input */
    flush();
    /* Verify Retirement */
    if total_winner != 0 {
        /* Verify */
        if get_check(b"Do you want to retire? \x00" as *const u8 as
                         *const libc::c_char) == 0 {
            return
        }
    } else {
        /* Verify Suicide */
        /* Verify */
        if get_check(b"Do you really want to quit? \x00" as *const u8 as
                         *const libc::c_char) == 0 {
            return
        }
        if noscore == 0 {
            /* Special Verification for suicide */
            prt(b"Please verify QUITTING by typing the \'@\' sign: \x00" as
                    *const u8 as *const libc::c_char, 0 as libc::c_int,
                0 as libc::c_int);
            flush();
            i = inkey() as libc::c_int;
            prt(b"\x00" as *const u8 as *const libc::c_char, 0 as libc::c_int,
                0 as libc::c_int);
            if i != '@' as i32 { return }
        }
    }
    /* Stop playing */
    alive = 0 as libc::c_int as bool_;
    /* Kill the player */
    death = 1 as libc::c_int as bool_;
    /* Leaving */
    (*p_ptr).leaving = 1 as libc::c_int as bool_;
    /* Cause of death */
    strcpy(died_from.as_mut_ptr(),
           b"Quitting\x00" as *const u8 as *const libc::c_char);
}
/* HACK - Remove / set the CAVE_VIEW flag, since view_x / view_y
	 * is not saved, and the visible locations are not lighted correctly
	 * when the game is loaded again
	 * Alternatively forget_view() and update_view() can be used
	 */
#[no_mangle]
pub unsafe extern "C" fn remove_cave_view(mut remove: bool_) {
    let mut i: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    if view_n != 0 {
        /* Clear them all */
        i = 0 as libc::c_int;
        while i < view_n as libc::c_int {
            let mut y: libc::c_int = view_y[i as usize] as libc::c_int;
            let mut x: libc::c_int = view_x[i as usize] as libc::c_int;
            /* Access the grid */
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type;
            if remove != 0 {
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int & !(0x20 as libc::c_int)) as
                        u16b
            } else {
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int | 0x20 as libc::c_int) as
                        u16b
            }
            i += 1
        }
    };
}
/*
 * Save the game
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_save_game() {
    remove_cave_view(1 as libc::c_int as bool_);
    /* Save the current level if in a persistent level */
    save_dungeon();
    /* Autosaves do not disturb */
    if is_autosave == 0 {
        /* Disturb the player */
        disturb(1 as libc::c_int, 0 as libc::c_int);
    }
    /* Clear messages */
    msg_print(0 as cptr);
    /* Handle stuff */
    handle_stuff();
    /* Message */
    prt(b"Saving game...\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int, 0 as libc::c_int);
    /* Refresh */
    Term_fresh();
    /* The player is not dead */
    strcpy(died_from.as_mut_ptr(),
           b"(saved)\x00" as *const u8 as *const libc::c_char);
    /* Forbid suspend */
    signals_ignore_tstp();
    /* Save the player */
    if save_player() != 0 {
        prt(b"Saving game... done.\x00" as *const u8 as *const libc::c_char,
            0 as libc::c_int, 0 as libc::c_int);
    } else {
        /* Save failed (oops) */
        prt(b"Saving game... failed!\x00" as *const u8 as *const libc::c_char,
            0 as libc::c_int, 0 as libc::c_int);
    }
    remove_cave_view(0 as libc::c_int as bool_);
    /* Allow suspend again */
    signals_handle_tstp();
    /* Refresh */
    Term_fresh();
    /* Note that the player is not dead */
    strcpy(died_from.as_mut_ptr(),
           b"(alive and well)\x00" as *const u8 as *const libc::c_char);
}
/*
 * Auto-save depending on whether the auto save flag is set.
 */
#[no_mangle]
pub unsafe extern "C" fn autosave_checkpoint() {
    if autosave_l != 0 {
        is_autosave = 1 as libc::c_int as bool_;
        msg_print(b"Autosaving the game...\x00" as *const u8 as
                      *const libc::c_char);
        do_cmd_save_game();
        is_autosave = 0 as libc::c_int as bool_
    };
}
/*
 * Hack -- Calculates the total number of points earned                -JWT-
 */
#[no_mangle]
pub unsafe extern "C" fn total_points() -> libc::c_long {
    let mut max_dl: s16b =
        0 as libc::c_int as
            s16b; /* was 100. Divided values by 5 because of an overflow error */
    let mut i: s16b = 0; /* Penalize preserve, maximize modes */
    let mut k: s16b = 0; /* At least 10% of the original score */
    let mut temp: libc::c_long = 0;
    let mut Total: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut mult: libc::c_long = 20 as libc::c_int as libc::c_long;
    let mut comp_death: libc::c_long =
        ((*p_ptr).companion_killed as libc::c_int * 2 as libc::c_int /
             5 as libc::c_int) as libc::c_long;
    if comp_death == 0 { comp_death = 1 as libc::c_int as libc::c_long }
    if (*p_ptr).preserve != 0 { mult -= 1 as libc::c_int as libc::c_long }
    if (*p_ptr).maximize != 0 { mult -= 1 as libc::c_int as libc::c_long }
    if auto_scum != 0 { mult -= 4 as libc::c_int as libc::c_long }
    if stupid_monsters != 0 { mult -= 10 as libc::c_int as libc::c_long }
    if small_levels != 0 {
        mult +=
            if always_small_level as libc::c_int != 0 {
                4 as libc::c_int
            } else { 10 as libc::c_int } as libc::c_long
    }
    if empty_levels != 0 { mult += 2 as libc::c_int as libc::c_long }
    if smart_learn != 0 { mult += 4 as libc::c_int as libc::c_long }
    if smart_cheat != 0 { mult += 4 as libc::c_int as libc::c_long }
    if mult < 2 as libc::c_int as libc::c_long {
        mult = 2 as libc::c_int as libc::c_long
    }
    /* mult is now between 2 and 40, i.e. 10% and 200% */
    i = 0 as libc::c_int as s16b;
    while (i as libc::c_int) < max_d_idx as libc::c_int {
        if *max_dlv.offset(i as isize) as libc::c_int > max_dl as libc::c_int
           {
            max_dl = *max_dlv.offset(i as isize)
        }
        i += 1
    }
    temp =
        ((*p_ptr).lev as libc::c_int * (*p_ptr).lev as libc::c_int *
             (*p_ptr).lev as libc::c_int * (*p_ptr).lev as libc::c_int +
             100 as libc::c_int * max_dl as libc::c_int) as libc::c_long;
    temp += ((*p_ptr).max_exp / 5 as libc::c_int) as libc::c_long;
    temp = temp * mult / 20 as libc::c_int as libc::c_long;
    /* Gold increases score */
    temp += ((*p_ptr).au / 5 as libc::c_int) as libc::c_long;
    /* Completing quest increase score */
    i = 0 as libc::c_int as s16b;
    while (i as libc::c_int) < max_q_idx as libc::c_int {
        if (*quest.offset(i as isize)).status as libc::c_int >=
               2 as libc::c_int {
            temp += 2000 as libc::c_int as libc::c_long;
            temp +=
                ((*quest.offset(i as isize)).level as libc::c_int *
                     100 as libc::c_int) as libc::c_long
        }
        i += 1
    }
    /* Death of a companion is BAD */
    temp /= comp_death;
    /* The know objects increase the score */
	/* Scan the object kinds */
    k = 1 as libc::c_int as s16b;
    while (k as libc::c_int) < max_k_idx as libc::c_int {
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
                object_prep(i_ptr, k as libc::c_int);
                temp += object_value_real(i_ptr) as libc::c_long
            }
        }
        k += 1
    }
    k = 1 as libc::c_int as s16b;
    while (k as libc::c_int) < max_r_idx as libc::c_int {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset(k as isize) as *mut monster_race;
        if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
            let mut dead: bool_ =
                ((*r_ptr).max_num as libc::c_int == 0 as libc::c_int) as
                    libc::c_int as bool_;
            if dead != 0 {
                /* Uniques are supposed to be harder */
                Total += 50 as libc::c_int as libc::c_long
            }
        } else {
            let mut This: s16b = (*r_ptr).r_pkills;
            if This as libc::c_int > 0 as libc::c_int {
                Total += This as libc::c_long
            }
        }
        k += 1
    }
    temp += Total * 50 as libc::c_int as libc::c_long;
    temp +=
        total_bounties.wrapping_mul(100 as libc::c_int as libc::c_uint) as
            libc::c_long;
    if total_winner != 0 { temp += 1000000 as libc::c_int as libc::c_long }
    return temp;
}
/*
 * Centers a string within a 31 character string                -JWT-
 */
unsafe extern "C" fn center_string(mut buf: *mut libc::c_char,
                                   mut str: cptr) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    /* Total length */
    i = strlen(str) as libc::c_int;
    /* Necessary border */
    j = 15 as libc::c_int - i / 2 as libc::c_int;
    /* Mega-Hack */
    sprintf(buf, b"%*s%s%*s\x00" as *const u8 as *const libc::c_char, j,
            b"\x00" as *const u8 as *const libc::c_char, str,
            31 as libc::c_int - i - j,
            b"\x00" as *const u8 as *const libc::c_char);
}
/*
 * Redefinable "print_tombstone" action
 */
#[no_mangle]
pub static mut tombstone_aux: Option<unsafe extern "C" fn() -> bool_> = None;
/*
 * Display a "tomb-stone"
 */
unsafe extern "C" fn print_tomb() {
    let mut done: bool_ = 0 as libc::c_int as bool_;
    /* Do we use a special tombstone ? */
    if tombstone_aux.is_some() {
        /* Use tombstone hook */
        done =
            Some(tombstone_aux.expect("non-null function pointer")).expect("non-null function pointer")()
    }
    /* Print the text-tombstone */
    if done == 0 {
        let mut p: cptr = 0 as *const libc::c_char;
        let mut tmp: [libc::c_char; 160] = [0; 160];
        let mut buf: [libc::c_char; 1024] = [0; 1024];
        let mut dummy: [libc::c_char; 80] = [0; 80];
        let mut fp: *mut FILE = 0 as *mut FILE;
        let mut ct: time_t = time(0 as *mut time_t);
        /* Clear screen */
        Term_clear();
        /* Build the filename */
        path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_FILE,
                   b"dead.txt\x00" as *const u8 as *const libc::c_char);
        /* Open the News file */
        fp =
            my_fopen(buf.as_mut_ptr() as cptr,
                     b"r\x00" as *const u8 as *const libc::c_char);
        /* Dump */
        if !fp.is_null() {
            let mut i: libc::c_int = 0 as libc::c_int;
            /* Dump the file to the screen */
            while 0 as libc::c_int ==
                      my_fgets(fp, buf.as_mut_ptr(),
                               1024 as libc::c_int as huge_hack) {
                /* Display and advance */
                let fresh14 = i;
                i = i + 1;
                display_message(0 as libc::c_int, fresh14,
                                strlen(buf.as_mut_ptr()) as libc::c_int,
                                1 as libc::c_int as byte_hack,
                                buf.as_mut_ptr() as cptr);
            }
            /* Close */
            my_fclose(fp);
        }
        /* King or Queen */
        if total_winner as libc::c_int != 0 ||
               (*p_ptr).lev as libc::c_int > 50 as libc::c_int {
            p = b"Magnificent\x00" as *const u8 as *const libc::c_char
        } else {
            /* Normal */
            p =
                c_text.offset((*cp_ptr).titles[(((*p_ptr).lev as libc::c_int -
                                                     1 as libc::c_int) /
                                                    5 as libc::c_int) as
                                                   usize] as isize) as cptr
        }
        center_string(buf.as_mut_ptr(), player_name.as_mut_ptr() as cptr);
        put_str(buf.as_mut_ptr() as cptr, 6 as libc::c_int,
                11 as libc::c_int);
        center_string(buf.as_mut_ptr(),
                      b"the\x00" as *const u8 as *const libc::c_char);
        put_str(buf.as_mut_ptr() as cptr, 7 as libc::c_int,
                11 as libc::c_int);
        center_string(buf.as_mut_ptr(), p);
        put_str(buf.as_mut_ptr() as cptr, 8 as libc::c_int,
                11 as libc::c_int);
        center_string(buf.as_mut_ptr(),
                      c_name.offset((*spp_ptr).title as isize) as cptr);
        put_str(buf.as_mut_ptr() as cptr, 10 as libc::c_int,
                11 as libc::c_int);
        sprintf(tmp.as_mut_ptr(),
                b"Level: %d\x00" as *const u8 as *const libc::c_char,
                (*p_ptr).lev as libc::c_int);
        center_string(buf.as_mut_ptr(), tmp.as_mut_ptr() as cptr);
        put_str(buf.as_mut_ptr() as cptr, 11 as libc::c_int,
                11 as libc::c_int);
        sprintf(tmp.as_mut_ptr(),
                b"Exp: %ld\x00" as *const u8 as *const libc::c_char,
                (*p_ptr).exp as libc::c_long);
        center_string(buf.as_mut_ptr(), tmp.as_mut_ptr() as cptr);
        put_str(buf.as_mut_ptr() as cptr, 12 as libc::c_int,
                11 as libc::c_int);
        sprintf(tmp.as_mut_ptr(),
                b"AU: %ld\x00" as *const u8 as *const libc::c_char,
                (*p_ptr).au as libc::c_long);
        center_string(buf.as_mut_ptr(), tmp.as_mut_ptr() as cptr);
        put_str(buf.as_mut_ptr() as cptr, 13 as libc::c_int,
                11 as libc::c_int);
        sprintf(tmp.as_mut_ptr(),
                b"Killed on Level %d\x00" as *const u8 as *const libc::c_char,
                dun_level as libc::c_int);
        center_string(buf.as_mut_ptr(), tmp.as_mut_ptr() as cptr);
        put_str(buf.as_mut_ptr() as cptr, 14 as libc::c_int,
                11 as libc::c_int);
        if strlen(died_from.as_mut_ptr()) > 24 as libc::c_int as libc::c_ulong
           {
            strncpy(dummy.as_mut_ptr(), died_from.as_mut_ptr(),
                    24 as libc::c_int as libc::c_ulong);
            dummy[24 as libc::c_int as usize] =
                '\u{0}' as i32 as libc::c_char;
            sprintf(tmp.as_mut_ptr(),
                    b"by %s.\x00" as *const u8 as *const libc::c_char,
                    dummy.as_mut_ptr());
        } else {
            sprintf(tmp.as_mut_ptr(),
                    b"by %s.\x00" as *const u8 as *const libc::c_char,
                    died_from.as_mut_ptr());
        }
        center_string(buf.as_mut_ptr(), tmp.as_mut_ptr() as cptr);
        put_str(buf.as_mut_ptr() as cptr, 15 as libc::c_int,
                11 as libc::c_int);
        sprintf(tmp.as_mut_ptr(),
                b"%-.24s\x00" as *const u8 as *const libc::c_char,
                ctime(&mut ct));
        center_string(buf.as_mut_ptr(), tmp.as_mut_ptr() as cptr);
        put_str(buf.as_mut_ptr() as cptr, 17 as libc::c_int,
                11 as libc::c_int);
    };
}
/*
 * Display some character info
 */
unsafe extern "C" fn show_info() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut st_ptr: *mut store_type = 0 as *mut store_type;
    /* Hack -- Know everything in the inven/equip */
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int {
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Skip non-objects */
        if !((*o_ptr).k_idx == 0) {
            /* Aware and Known */
            object_aware(o_ptr);
            object_known(o_ptr);
        }
        i += 1
    }
    i = 1 as libc::c_int;
    while i < max_towns as libc::c_int {
        st_ptr =
            &mut *(*town_info.offset(i as
                                         isize)).store.offset(7 as libc::c_int
                                                                  as isize) as
                *mut store_type;
        /* Hack -- Know everything in the home */
        j = 0 as libc::c_int;
        while j < (*st_ptr).stock_num as libc::c_int {
            o_ptr =
                &mut *(*st_ptr).stock.offset(j as isize) as *mut object_type;
            /* Skip non-objects */
            if !((*o_ptr).k_idx == 0) {
                /* Aware and Known */
                object_aware(o_ptr);
                object_known(o_ptr);
            }
            j += 1
        }
        i += 1
    }
    /* Hack -- Recalculate bonuses */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    /* Handle stuff */
    handle_stuff();
    /* Flush all input keys */
    flush();
    /* Flush messages */
    msg_print(0 as cptr);
    /* Describe options */
    prt(b"You may now dump a character record to one or more files.\x00" as
            *const u8 as *const libc::c_char, 21 as libc::c_int,
        0 as libc::c_int);
    prt(b"Then, hit RETURN to see the character, or ESC to abort.\x00" as
            *const u8 as *const libc::c_char, 22 as libc::c_int,
        0 as libc::c_int);
    loop 
         /* Dump character records as requested */
         {
        let mut out_val: [libc::c_char; 160] = [0; 160];
        /* Prompt */
        put_str(b"Filename(you can post it to http://angband.oook.cz/): \x00"
                    as *const u8 as *const libc::c_char, 23 as libc::c_int,
                0 as libc::c_int);
        /* Default */
        strcpy(out_val.as_mut_ptr(),
               b"\x00" as *const u8 as *const libc::c_char);
        /* Ask for filename (or abort) */
        if askfor_aux(out_val.as_mut_ptr(), 60 as libc::c_int) == 0 { return }
        /* Return means "show on screen" */
        if out_val[0 as libc::c_int as usize] == 0 { break ; }
        /* Save screen */
        character_icky = 1 as libc::c_int as bool_;
        Term_save();
        /* Dump a character file */
        file_character(out_val.as_mut_ptr() as cptr,
                       1 as libc::c_int as bool_);
        /* Load screen */
        Term_load();
        character_icky = 0 as libc::c_int as bool_
    }
    /* Display player */
    display_player(0 as libc::c_int);
    /* Prompt for p_ptr->inventory */
    prt(b"Hit any key to see more information (ESC to abort): \x00" as
            *const u8 as *const libc::c_char, 23 as libc::c_int,
        0 as libc::c_int);
    /* Allow abort at this point */
    if inkey() as libc::c_int == '\u{1b}' as i32 { return }
    /* Show equipment and inventory */
    /* Equipment -- if any */
    if equip_cnt != 0 {
        Term_clear();
        item_tester_full = 1 as libc::c_int as bool_;
        show_equip();
        prt(b"You are using: -more-\x00" as *const u8 as *const libc::c_char,
            0 as libc::c_int, 0 as libc::c_int);
        if inkey() as libc::c_int == '\u{1b}' as i32 { return }
    }
    /* Inventory -- if any */
    if inven_cnt != 0 {
        Term_clear();
        item_tester_full = 1 as libc::c_int as bool_;
        show_inven();
        prt(b"You are carrying: -more-\x00" as *const u8 as
                *const libc::c_char, 0 as libc::c_int, 0 as libc::c_int);
        if inkey() as libc::c_int == '\u{1b}' as i32 { return }
    }
    /* Homes in the different towns */
    k = 1 as libc::c_int;
    while k < max_towns as libc::c_int {
        st_ptr =
            &mut *(*town_info.offset(k as
                                         isize)).store.offset(7 as libc::c_int
                                                                  as isize) as
                *mut store_type;
        /* Home -- if anything there */
        if (*st_ptr).stock_num != 0 {
            /* Display contents of the home */
            k = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < (*st_ptr).stock_num as libc::c_int {
                /* Clear screen */
                Term_clear();
                /* Show 12 items */
                j = 0 as libc::c_int;
                while j < 12 as libc::c_int &&
                          i < (*st_ptr).stock_num as libc::c_int {
                    let mut o_name: [libc::c_char; 80] = [0; 80];
                    let mut tmp_val: [libc::c_char; 80] = [0; 80];
                    /* Acquire item */
                    o_ptr =
                        &mut *(*st_ptr).stock.offset(i as isize) as
                            *mut object_type;
                    /* Print header, clear line */
                    sprintf(tmp_val.as_mut_ptr(),
                            b"%c) \x00" as *const u8 as *const libc::c_char,
                            j + 'a' as i32);
                    prt(tmp_val.as_mut_ptr() as cptr, j + 2 as libc::c_int,
                        4 as libc::c_int);
                    /* Display object description */
                    object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                                3 as libc::c_int);
                    c_put_str(tval_to_attr[(*o_ptr).tval as usize],
                              o_name.as_mut_ptr() as cptr,
                              j + 2 as libc::c_int, 7 as libc::c_int);
                    j += 1;
                    i += 1
                }
                /* h_ptr->caption */
                prt(format(b"Your home contains (page %d): -more-\x00" as
                               *const u8 as *const libc::c_char,
                           k + 1 as libc::c_int) as cptr, 0 as libc::c_int,
                    0 as libc::c_int);
                /* Wait for it */
                if inkey() as libc::c_int == '\u{1b}' as i32 { return }
                k += 1
            }
        }
        k += 1
    };
}
/* Method of death (string) */
/*
 * Seek score 'i' in the highscore file
 */
unsafe extern "C" fn highscore_seek(mut i: libc::c_int) -> libc::c_int {
    /* Seek for the requested record */
    return fd_seek(highscore_fd,
                   (i as
                        huge_hack).wrapping_mul(::std::mem::size_of::<high_score>()
                                                    as libc::c_ulong));
}
/*
 * Read one score from the highscore file
 */
unsafe extern "C" fn highscore_read(mut score: *mut high_score) -> errr {
    /* Read the record, note failure */
    return fd_read(highscore_fd, score as *mut libc::c_char,
                   ::std::mem::size_of::<high_score>() as libc::c_ulong);
}
/*
 * Write one score to the highscore file
 */
unsafe extern "C" fn highscore_write(mut score: *mut high_score)
 -> libc::c_int {
    /* Write the record, note failure */
    return fd_write(highscore_fd, score as *mut libc::c_char as cptr,
                    ::std::mem::size_of::<high_score>() as libc::c_ulong);
}
/*
 * Just determine where a new score *would* be placed
 * Return the location (0 is best) or -1 on failure
 */
unsafe extern "C" fn highscore_where(mut score: *mut high_score)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut the_score: high_score =
        high_score{what: [0; 8],
                   pts: [0; 10],
                   gold: [0; 10],
                   turns: [0; 10],
                   day: [0; 10],
                   who: [0; 16],
                   uid: [0; 8],
                   sex: [0; 2],
                   p_r: [0; 3],
                   p_s: [0; 3],
                   p_c: [0; 3],
                   p_cs: [0; 3],
                   cur_lev: [0; 4],
                   cur_dun: [0; 4],
                   max_lev: [0; 4],
                   max_dun: [0; 4],
                   arena_number: [0; 4],
                   inside_arena: [0; 4],
                   inside_quest: [0; 4],
                   exit_bldg: [0; 4],
                   how: [0; 32],};
    /* Paranoia -- it may not have opened */
    if highscore_fd < 0 as libc::c_int { return -(1 as libc::c_int) }
    /* Go to the start of the highscore file */
    if highscore_seek(0 as libc::c_int) != 0 { return -(1 as libc::c_int) }
    /* Read until we get to a higher score */
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        if highscore_read(&mut the_score) != 0 { return i }
        if strcmp(the_score.pts.as_mut_ptr(), (*score).pts.as_mut_ptr()) <
               0 as libc::c_int {
            return i
        }
        i += 1
    }
    /* The "last" entry is always usable */
    return 100 as libc::c_int - 1 as libc::c_int;
}
/*
 * Actually place an entry into the high score file
 * Return the location (0 is best) or -1 on "failure"
 */
unsafe extern "C" fn highscore_add(mut score: *mut high_score)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut slot: libc::c_int = 0;
    let mut done: bool_ = 0 as libc::c_int as bool_;
    let mut the_score: high_score =
        high_score{what: [0; 8],
                   pts: [0; 10],
                   gold: [0; 10],
                   turns: [0; 10],
                   day: [0; 10],
                   who: [0; 16],
                   uid: [0; 8],
                   sex: [0; 2],
                   p_r: [0; 3],
                   p_s: [0; 3],
                   p_c: [0; 3],
                   p_cs: [0; 3],
                   cur_lev: [0; 4],
                   cur_dun: [0; 4],
                   max_lev: [0; 4],
                   max_dun: [0; 4],
                   arena_number: [0; 4],
                   inside_arena: [0; 4],
                   inside_quest: [0; 4],
                   exit_bldg: [0; 4],
                   how: [0; 32],};
    let mut tmpscore: high_score =
        high_score{what: [0; 8],
                   pts: [0; 10],
                   gold: [0; 10],
                   turns: [0; 10],
                   day: [0; 10],
                   who: [0; 16],
                   uid: [0; 8],
                   sex: [0; 2],
                   p_r: [0; 3],
                   p_s: [0; 3],
                   p_c: [0; 3],
                   p_cs: [0; 3],
                   cur_lev: [0; 4],
                   cur_dun: [0; 4],
                   max_lev: [0; 4],
                   max_dun: [0; 4],
                   arena_number: [0; 4],
                   inside_arena: [0; 4],
                   inside_quest: [0; 4],
                   exit_bldg: [0; 4],
                   how: [0; 32],};
    /* Paranoia -- it may not have opened */
    if highscore_fd < 0 as libc::c_int { return -(1 as libc::c_int) }
    /* Determine where the score should go */
    slot = highscore_where(score);
    /* Hack -- Not on the list */
    if slot < 0 as libc::c_int { return -(1 as libc::c_int) }
    /* Hack -- prepare to dump the new score */
    the_score = *score;
    /* Slide all the scores down one */
    i = slot;
    while done == 0 && i < 100 as libc::c_int {
        /* Read the old guy, note errors */
        if highscore_seek(i) != 0 { return -(1 as libc::c_int) }
        if highscore_read(&mut tmpscore) != 0 {
            done = 1 as libc::c_int as bool_
        }
        /* Back up and dump the score we were holding */
        if highscore_seek(i) != 0 { return -(1 as libc::c_int) }
        if highscore_write(&mut the_score) != 0 { return -(1 as libc::c_int) }
        /* Hack -- Save the old score, for the next pass */
        the_score = tmpscore;
        i += 1
    }
    /* Return location used */
    return slot;
}
/*
 * Display the scores in a given range.
 * Assumes the high score list is already open.
 * Only five entries per line, too much info.
 *
 * Mega-Hack -- allow "fake" entry at the given position.
 */
unsafe extern "C" fn display_scores_aux(mut from: libc::c_int,
                                        mut to: libc::c_int,
                                        mut note: libc::c_int,
                                        mut score: *mut high_score) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut place: libc::c_int = 0;
    let mut attr: byte_hack = 0;
    let mut out_val: [libc::c_char; 256] = [0; 256];
    let mut tmp_val: [libc::c_char; 160] = [0; 160];
    let mut the_score: high_score =
        high_score{what: [0; 8],
                   pts: [0; 10],
                   gold: [0; 10],
                   turns: [0; 10],
                   day: [0; 10],
                   who: [0; 16],
                   uid: [0; 8],
                   sex: [0; 2],
                   p_r: [0; 3],
                   p_s: [0; 3],
                   p_c: [0; 3],
                   p_cs: [0; 3],
                   cur_lev: [0; 4],
                   cur_dun: [0; 4],
                   max_lev: [0; 4],
                   max_dun: [0; 4],
                   arena_number: [0; 4],
                   inside_arena: [0; 4],
                   inside_quest: [0; 4],
                   exit_bldg: [0; 4],
                   how: [0; 32],};
    /* Paranoia -- it may not have opened */
    if highscore_fd < 0 as libc::c_int { return }
    /* Assume we will show the first 10 */
    if from < 0 as libc::c_int { from = 0 as libc::c_int }
    if to < 0 as libc::c_int { to = 10 as libc::c_int }
    if to > 100 as libc::c_int { to = 100 as libc::c_int }
    /* Seek to the beginning */
    if highscore_seek(0 as libc::c_int) != 0 { return }
    /* Hack -- Count the high scores */
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        if highscore_read(&mut the_score) != 0 { break ; }
        i += 1
    }
    /* Hack -- allow "fake" entry to be last */
    if note == i && !score.is_null() { i += 1 }
    /* Forget about the last entries */
    if i > to { i = to }
    /* Show 5 per page, until "done" */
    k = from;
    place = k + 1 as libc::c_int;
    while k < i {
        /* Clear screen */
        Term_clear();
        /* Title */
        put_str(format(b"              %s Hall of Fame\x00" as *const u8 as
                           *const libc::c_char, game_module) as cptr,
                0 as libc::c_int, 0 as libc::c_int);
        /* Indicate non-top scores */
        if k > 0 as libc::c_int {
            sprintf(tmp_val.as_mut_ptr(),
                    b"(from position %d)\x00" as *const u8 as
                        *const libc::c_char, k + 1 as libc::c_int);
            put_str(tmp_val.as_mut_ptr() as cptr, 0 as libc::c_int,
                    40 as libc::c_int);
        }
        /* Dump 5 entries */
        j = k;
        n = 0 as libc::c_int;
        while j < i && n < 5 as libc::c_int {
            let mut pcs: libc::c_int = 0;
            let mut pr: libc::c_int = 0;
            let mut ps: libc::c_int = 0;
            let mut pc: libc::c_int = 0;
            let mut clev: libc::c_int = 0;
            let mut mlev: libc::c_int = 0;
            let mut cdun: libc::c_int = 0;
            let mut mdun: libc::c_int = 0;
            let mut user: cptr = 0 as *const libc::c_char;
            let mut gold: cptr = 0 as *const libc::c_char;
            let mut when: cptr = 0 as *const libc::c_char;
            let mut aged: cptr = 0 as *const libc::c_char;
            let mut in_arena: libc::c_int = 0;
            let mut in_quest: libc::c_int = 0;
            /* Hack -- indicate death in yellow */
            attr =
                if j == note { 11 as libc::c_int } else { 1 as libc::c_int }
                    as byte_hack;
            /* Mega-Hack -- insert a "fake" record */
            if note == j && !score.is_null() {
                the_score = *score;
                attr = 13 as libc::c_int as byte_hack;
                score = 0 as *mut high_score;
                note = -(1 as libc::c_int);
                j -= 1
            } else {
                /* Read a normal record */
                /* Read the proper record */
                if highscore_seek(j) != 0 { break ; }
                if highscore_read(&mut the_score) != 0 { break ; }
            }
            /* Extract the race/class */
            pr = atoi(the_score.p_r.as_mut_ptr());
            ps = atoi(the_score.p_s.as_mut_ptr());
            pc = atoi(the_score.p_c.as_mut_ptr());
            pcs = atoi(the_score.p_cs.as_mut_ptr());
            /* Extract the level info */
            clev = atoi(the_score.cur_lev.as_mut_ptr());
            mlev = atoi(the_score.max_lev.as_mut_ptr());
            cdun = atoi(the_score.cur_dun.as_mut_ptr());
            mdun = atoi(the_score.max_dun.as_mut_ptr());
            in_arena = atoi(the_score.inside_arena.as_mut_ptr());
            in_quest = atoi(the_score.inside_quest.as_mut_ptr());
            /* Hack -- extract the gold and such */
            user = the_score.uid.as_mut_ptr() as cptr;
            while *(*__ctype_b_loc()).offset(*user as libc::c_int as isize) as
                      libc::c_int &
                      _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 {
                /* loop */
                user = user.offset(1)
            }
            when = the_score.day.as_mut_ptr() as cptr;
            while *(*__ctype_b_loc()).offset(*when as libc::c_int as isize) as
                      libc::c_int &
                      _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 {
                /* loop */
                when = when.offset(1)
            }
            gold = the_score.gold.as_mut_ptr() as cptr;
            while *(*__ctype_b_loc()).offset(*gold as libc::c_int as isize) as
                      libc::c_int &
                      _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 {
                /* loop */
                gold = gold.offset(1)
            }
            aged = the_score.turns.as_mut_ptr() as cptr;
            while *(*__ctype_b_loc()).offset(*aged as libc::c_int as isize) as
                      libc::c_int &
                      _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 {
                /* loop */
                aged = aged.offset(1)
            }
            /* Dump some info */
            sprintf(out_val.as_mut_ptr(),
                    b"%3d.%9s  %s the %s %s, Level %d\x00" as *const u8 as
                        *const libc::c_char, place,
                    the_score.pts.as_mut_ptr(), the_score.who.as_mut_ptr(),
                    get_player_race_name(pr, ps),
                    c_name.offset((*class_info.offset(pc as
                                                          isize)).spec[pcs as
                                                                           usize].title
                                      as isize), clev);
            /* Append a "maximum level" */
            if mlev > clev {
                strcat(out_val.as_mut_ptr(),
                       format(b" (Max %d)\x00" as *const u8 as
                                  *const libc::c_char, mlev));
            }
            /* Dump the first line */
            c_put_str(attr, out_val.as_mut_ptr() as cptr,
                      n * 4 as libc::c_int + 2 as libc::c_int,
                      0 as libc::c_int);
            /* Another line of info */
            if in_arena != 0 {
                sprintf(out_val.as_mut_ptr(),
                        b"               Killed by %s in the Arena\x00" as
                            *const u8 as *const libc::c_char,
                        the_score.how.as_mut_ptr());
            } else if in_quest != 0 {
                sprintf(out_val.as_mut_ptr(),
                        b"               Killed by %s while questing\x00" as
                            *const u8 as *const libc::c_char,
                        the_score.how.as_mut_ptr());
            } else if cdun == 0 {
                sprintf(out_val.as_mut_ptr(),
                        b"               Killed by %s in the Town\x00" as
                            *const u8 as *const libc::c_char,
                        the_score.how.as_mut_ptr());
            } else {
                sprintf(out_val.as_mut_ptr(),
                        b"               Killed by %s on %s %d\x00" as
                            *const u8 as *const libc::c_char,
                        the_score.how.as_mut_ptr(),
                        b"Dungeon Level\x00" as *const u8 as
                            *const libc::c_char, cdun);
            }
            /* Hack -- some people die in the town */
            /* Append a "maximum level" */
            if mdun > cdun {
                strcat(out_val.as_mut_ptr(),
                       format(b" (Max %d)\x00" as *const u8 as
                                  *const libc::c_char, mdun));
            }
            /* Dump the info */
            c_put_str(attr, out_val.as_mut_ptr() as cptr,
                      n * 4 as libc::c_int + 3 as libc::c_int,
                      0 as libc::c_int);
            /* And still another line of info */
            sprintf(out_val.as_mut_ptr(),
                    b"               (User %s, Date %s, Gold %s, Turn %s).\x00"
                        as *const u8 as *const libc::c_char, user, when, gold,
                    aged);
            c_put_str(attr, out_val.as_mut_ptr() as cptr,
                      n * 4 as libc::c_int + 4 as libc::c_int,
                      0 as libc::c_int);
            place += 1;
            j += 1;
            n += 1
        }
        /* Wait for response */
        prt(b"[Press ESC to quit, any other key to continue.]\x00" as
                *const u8 as *const libc::c_char, 23 as libc::c_int,
            17 as libc::c_int);
        j = inkey() as libc::c_int;
        prt(b"\x00" as *const u8 as *const libc::c_char, 23 as libc::c_int,
            0 as libc::c_int);
        /* Hack -- notice Escape */
        if j == '\u{1b}' as i32 { break ; }
        k += 5 as libc::c_int
    };
}
/*
 * Hack -- Display the scores in a given range and quit.
 *
 * This function is only called from "main.c" when the user asks
 * to see the "high scores".
 */
#[no_mangle]
pub unsafe extern "C" fn display_scores(mut from: libc::c_int,
                                        mut to: libc::c_int) {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_APEX,
               b"scores.raw\x00" as *const u8 as *const libc::c_char);
    /* Open the binary high score file, for reading */
    highscore_fd = fd_open(buf.as_mut_ptr() as cptr, 0 as libc::c_int);
    /* Paranoia -- No score file */
    if highscore_fd < 0 as libc::c_int {
        quit(b"Score file unavailable.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Clear screen */
    Term_clear();
    /* Display the scores */
    display_scores_aux(from, to, -(1 as libc::c_int), 0 as *mut high_score);
    /* Shut the high score file */
    fd_close(highscore_fd);
    /* Forget the high score fd */
    highscore_fd = -(1 as libc::c_int);
    /* Quit */
    quit(0 as cptr);
}
/*
 * show_highclass - selectively list highscores based on class
 * -KMW-
 */
#[no_mangle]
pub unsafe extern "C" fn show_highclass(mut building: libc::c_int) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut m: libc::c_int = 0 as libc::c_int;
    let mut pr: libc::c_int = 0;
    let mut pc: libc::c_int = 0;
    let mut clev: libc::c_int = 0;
    let mut al: libc::c_int = 0;
    let mut the_score: high_score =
        high_score{what: [0; 8],
                   pts: [0; 10],
                   gold: [0; 10],
                   turns: [0; 10],
                   day: [0; 10],
                   who: [0; 16],
                   uid: [0; 8],
                   sex: [0; 2],
                   p_r: [0; 3],
                   p_s: [0; 3],
                   p_c: [0; 3],
                   p_cs: [0; 3],
                   cur_lev: [0; 4],
                   cur_dun: [0; 4],
                   max_lev: [0; 4],
                   max_dun: [0; 4],
                   arena_number: [0; 4],
                   inside_arena: [0; 4],
                   inside_quest: [0; 4],
                   exit_bldg: [0; 4],
                   how: [0; 32],};
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut out_val: [libc::c_char; 256] = [0; 256];
    match building {
        1 => {
            prt(b"               Busts of Greatest Kings\x00" as *const u8 as
                    *const libc::c_char, 5 as libc::c_int, 0 as libc::c_int);
        }
        2 => {
            prt(b"               Plaque - Greatest Arena Champions\x00" as
                    *const u8 as *const libc::c_char, 5 as libc::c_int,
                0 as libc::c_int);
        }
        10 => {
            prt(b"               Plaque - Greatest Fighters\x00" as *const u8
                    as *const libc::c_char, 5 as libc::c_int,
                0 as libc::c_int);
        }
        11 => {
            prt(b"               Spires of the Greatest Magic-Users\x00" as
                    *const u8 as *const libc::c_char, 5 as libc::c_int,
                0 as libc::c_int);
        }
        12 => {
            prt(b"               Busts of Greatest Priests\x00" as *const u8
                    as *const libc::c_char, 5 as libc::c_int,
                0 as libc::c_int);
        }
        13 => {
            prt(b"               Wall Inscriptions - Greatest Thieves\x00" as
                    *const u8 as *const libc::c_char, 5 as libc::c_int,
                0 as libc::c_int);
        }
        14 => {
            prt(b"               Plaque - Greatest Rangers\x00" as *const u8
                    as *const libc::c_char, 5 as libc::c_int,
                0 as libc::c_int);
        }
        15 => {
            prt(b"               Plaque - Greatest Paladins\x00" as *const u8
                    as *const libc::c_char, 5 as libc::c_int,
                0 as libc::c_int);
        }
        16 => {
            prt(b"               Spires of the Greatest Illusionists\x00" as
                    *const u8 as *const libc::c_char, 5 as libc::c_int,
                0 as libc::c_int);
        }
        _ => { bell(); }
    }
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_APEX,
               b"scores.raw\x00" as *const u8 as *const libc::c_char);
    /* Open file */
    highscore_fd = fd_open(buf.as_mut_ptr() as cptr, 0 as libc::c_int);
    if highscore_fd < 0 as libc::c_int {
        msg_print(b"Score file unavailable.\x00" as *const u8 as
                      *const libc::c_char);
        msg_print(0 as cptr);
        return
    }
    if highscore_seek(0 as libc::c_int) != 0 { return }
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        if highscore_read(&mut the_score) != 0 { break ; }
        i += 1
    }
    m = 0 as libc::c_int;
    j = 0 as libc::c_int;
    clev = 0 as libc::c_int;
    while m < 9 as libc::c_int || j < 100 as libc::c_int {
        if highscore_seek(j) != 0 { break ; }
        if highscore_read(&mut the_score) != 0 { break ; }
        pr = atoi(the_score.p_r.as_mut_ptr());
        pc = atoi(the_score.p_c.as_mut_ptr());
        clev = atoi(the_score.cur_lev.as_mut_ptr());
        al = atoi(the_score.arena_number.as_mut_ptr());
        if pc == building - 10 as libc::c_int && building != 1 as libc::c_int
               && building != 2 as libc::c_int ||
               building == 1 as libc::c_int && clev >= 50 as libc::c_int ||
               building == 2 as libc::c_int && al > 29 as libc::c_int {
            sprintf(out_val.as_mut_ptr(),
                    b"%3d) %s the %s (Level %2d)\x00" as *const u8 as
                        *const libc::c_char, m + 1 as libc::c_int,
                    the_score.who.as_mut_ptr(),
                    rp_name.offset((*race_info.offset(pr as isize)).title as
                                       isize), clev);
            prt(out_val.as_mut_ptr() as cptr, m + 7 as libc::c_int,
                0 as libc::c_int);
            m += 1
        }
        j += 1
    }
    /* Now, list the active player if they qualify */
    if building == 1 as libc::c_int &&
           (*p_ptr).lev as libc::c_int >= 50 as libc::c_int {
        sprintf(out_val.as_mut_ptr(),
                b"You) %s the %s (Level %2d)\x00" as *const u8 as
                    *const libc::c_char, player_name.as_mut_ptr(),
                rp_name.offset((*race_info.offset((*p_ptr).prace as
                                                      isize)).title as isize),
                (*p_ptr).lev as libc::c_int);
        prt(out_val.as_mut_ptr() as cptr, m + 8 as libc::c_int,
            0 as libc::c_int);
    } else if building == 2 as libc::c_int &&
                  (*p_ptr).arena_number as libc::c_int > 29 as libc::c_int {
        sprintf(out_val.as_mut_ptr(),
                b"You) %s the %s (Level %2d)\x00" as *const u8 as
                    *const libc::c_char, player_name.as_mut_ptr(),
                rp_name.offset((*race_info.offset((*p_ptr).prace as
                                                      isize)).title as isize),
                (*p_ptr).lev as libc::c_int);
        prt(out_val.as_mut_ptr() as cptr, m + 8 as libc::c_int,
            0 as libc::c_int);
    } else if building != 1 as libc::c_int && building != 2 as libc::c_int {
        if (*p_ptr).lev as libc::c_int > clev &&
               (*p_ptr).pclass as libc::c_int == building - 10 as libc::c_int
           {
            sprintf(out_val.as_mut_ptr(),
                    b"You) %s the %s (Level %2d)\x00" as *const u8 as
                        *const libc::c_char, player_name.as_mut_ptr(),
                    rp_name.offset((*race_info.offset((*p_ptr).prace as
                                                          isize)).title as
                                       isize), (*p_ptr).lev as libc::c_int);
            prt(out_val.as_mut_ptr() as cptr, m + 8 as libc::c_int,
                0 as libc::c_int);
        }
    }
    fd_close(highscore_fd);
    highscore_fd = -(1 as libc::c_int);
    msg_print(b"Hit any key to continue\x00" as *const u8 as
                  *const libc::c_char);
    msg_print(0 as cptr);
    j = 5 as libc::c_int;
    while j < 18 as libc::c_int {
        prt(b"\x00" as *const u8 as *const libc::c_char, j, 0 as libc::c_int);
        j += 1
    };
}
/*
 * Race Legends
 * -KMW-
 */
#[no_mangle]
pub unsafe extern "C" fn race_score(mut race_num: libc::c_int) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut m: libc::c_int = 0 as libc::c_int;
    let mut pr: libc::c_int = 0;
    let mut clev: libc::c_int = 0;
    let mut lastlev: libc::c_int = 0;
    let mut the_score: high_score =
        high_score{what: [0; 8],
                   pts: [0; 10],
                   gold: [0; 10],
                   turns: [0; 10],
                   day: [0; 10],
                   who: [0; 16],
                   uid: [0; 8],
                   sex: [0; 2],
                   p_r: [0; 3],
                   p_s: [0; 3],
                   p_c: [0; 3],
                   p_cs: [0; 3],
                   cur_lev: [0; 4],
                   cur_dun: [0; 4],
                   max_lev: [0; 4],
                   max_dun: [0; 4],
                   arena_number: [0; 4],
                   inside_arena: [0; 4],
                   inside_quest: [0; 4],
                   exit_bldg: [0; 4],
                   how: [0; 32],};
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut out_val: [libc::c_char; 256] = [0; 256];
    let mut tmp_str: [libc::c_char; 80] = [0; 80];
    lastlev = 0 as libc::c_int;
    /* rr9: TODO - pluralize the race */
    sprintf(tmp_str.as_mut_ptr(),
            b"The Greatest of all the %s\x00" as *const u8 as
                *const libc::c_char,
            rp_name.offset((*race_info.offset(race_num as isize)).title as
                               isize));
    prt(tmp_str.as_mut_ptr() as cptr, 5 as libc::c_int, 3 as libc::c_int);
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_APEX,
               b"scores.raw\x00" as *const u8 as *const libc::c_char);
    /* Open the highscore file */
    highscore_fd = fd_open(buf.as_mut_ptr() as cptr, 0 as libc::c_int);
    if highscore_fd < 0 as libc::c_int {
        msg_print(b"Score file unavailable.\x00" as *const u8 as
                      *const libc::c_char);
        msg_print(0 as cptr);
        return
    }
    if highscore_seek(0 as libc::c_int) != 0 { return }
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        if highscore_read(&mut the_score) != 0 { break ; }
        i += 1
    }
    m = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while m < 10 as libc::c_int && j < i {
        if highscore_seek(j) != 0 { break ; }
        if highscore_read(&mut the_score) != 0 { break ; }
        pr = atoi(the_score.p_r.as_mut_ptr());
        clev = atoi(the_score.cur_lev.as_mut_ptr());
        if pr == race_num {
            sprintf(out_val.as_mut_ptr(),
                    b"%3d) %s the %s (Level %3d)\x00" as *const u8 as
                        *const libc::c_char, m + 1 as libc::c_int,
                    the_score.who.as_mut_ptr(),
                    rp_name.offset((*race_info.offset(pr as isize)).title as
                                       isize), clev);
            prt(out_val.as_mut_ptr() as cptr, m + 7 as libc::c_int,
                0 as libc::c_int);
            m += 1;
            lastlev = clev
        }
        j += 1
    }
    /* add player if qualified */
    if (*p_ptr).prace as libc::c_int == race_num &&
           (*p_ptr).lev as libc::c_int >= lastlev {
        sprintf(out_val.as_mut_ptr(),
                b"You) %s the %s (Level %3d)\x00" as *const u8 as
                    *const libc::c_char, player_name.as_mut_ptr(),
                rp_name.offset((*race_info.offset((*p_ptr).prace as
                                                      isize)).title as isize),
                (*p_ptr).lev as libc::c_int);
        prt(out_val.as_mut_ptr() as cptr, m + 8 as libc::c_int,
            0 as libc::c_int);
    }
    fd_close(highscore_fd);
    highscore_fd = -(1 as libc::c_int);
}
/*
 * Race Legends
 * -KMW-
 */
#[no_mangle]
pub unsafe extern "C" fn race_legends() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < max_rp_idx as libc::c_int {
        race_score(i);
        msg_print(b"Hit any key to continue\x00" as *const u8 as
                      *const libc::c_char);
        msg_print(0 as cptr);
        j = 5 as libc::c_int;
        while j < 19 as libc::c_int {
            prt(b"\x00" as *const u8 as *const libc::c_char, j,
                0 as libc::c_int);
            j += 1
        }
        i += 1
    };
}
/*
 * Enters a players name on a hi-score table, if "legal", and in any
 * case, displays some relevant portion of the high score list.
 *
 * Assumes "signals_ignore_tstp()" has been called.
 */
unsafe extern "C" fn top_twenty() -> errr {
    let mut j: libc::c_int = 0;
    let mut the_score: high_score =
        high_score{what: [0; 8],
                   pts: [0; 10],
                   gold: [0; 10],
                   turns: [0; 10],
                   day: [0; 10],
                   who: [0; 16],
                   uid: [0; 8],
                   sex: [0; 2],
                   p_r: [0; 3],
                   p_s: [0; 3],
                   p_c: [0; 3],
                   p_cs: [0; 3],
                   cur_lev: [0; 4],
                   cur_dun: [0; 4],
                   max_lev: [0; 4],
                   max_dun: [0; 4],
                   arena_number: [0; 4],
                   inside_arena: [0; 4],
                   inside_quest: [0; 4],
                   exit_bldg: [0; 4],
                   how: [0; 32],};
    let mut ct: time_t = time(0 as *mut time_t);
    /* Clear screen */
    Term_clear();
    /* No score file */
    if highscore_fd < 0 as libc::c_int {
        msg_print(b"Score file unavailable.\x00" as *const u8 as
                      *const libc::c_char);
        msg_print(0 as cptr);
        return 0 as libc::c_int
    }
    /* Wizard-mode pre-empts scoring */
    if noscore as libc::c_int & 0xf as libc::c_int != 0 {
        msg_print(b"Score not registered for wizards.\x00" as *const u8 as
                      *const libc::c_char);
        msg_print(0 as cptr);
        display_scores_aux(0 as libc::c_int, 10 as libc::c_int,
                           -(1 as libc::c_int), 0 as *mut high_score);
        return 0 as libc::c_int
    }
    /* Borg-mode pre-empts scoring */
    if noscore as libc::c_int & 0xf0 as libc::c_int != 0 {
        msg_print(b"Score not registered for borgs.\x00" as *const u8 as
                      *const libc::c_char);
        msg_print(0 as cptr);
        display_scores_aux(0 as libc::c_int, 10 as libc::c_int,
                           -(1 as libc::c_int), 0 as *mut high_score);
        return 0 as libc::c_int
    }
    /* Cheaters are not scored */
    if noscore as libc::c_int & 0xff00 as libc::c_int != 0 {
        msg_print(b"Score not registered for cheaters.\x00" as *const u8 as
                      *const libc::c_char);
        msg_print(0 as cptr);
        display_scores_aux(0 as libc::c_int, 10 as libc::c_int,
                           -(1 as libc::c_int), 0 as *mut high_score);
        return 0 as libc::c_int
    }
    /* Interupted */
    if total_winner == 0 &&
           streq(died_from.as_mut_ptr() as cptr,
                 b"Interrupting\x00" as *const u8 as *const libc::c_char) as
               libc::c_int != 0 {
        msg_print(b"Score not registered due to interruption.\x00" as
                      *const u8 as *const libc::c_char);
        msg_print(0 as cptr);
        display_scores_aux(0 as libc::c_int, 10 as libc::c_int,
                           -(1 as libc::c_int), 0 as *mut high_score);
        return 0 as libc::c_int
    }
    /* Quitter */
    if total_winner == 0 &&
           streq(died_from.as_mut_ptr() as cptr,
                 b"Quitting\x00" as *const u8 as *const libc::c_char) as
               libc::c_int != 0 {
        msg_print(b"Score not registered due to quitting.\x00" as *const u8 as
                      *const libc::c_char);
        msg_print(0 as cptr);
        display_scores_aux(0 as libc::c_int, 10 as libc::c_int,
                           -(1 as libc::c_int), 0 as *mut high_score);
        return 0 as libc::c_int
    }
    /* Clear the record */
    memset(&mut the_score as *mut high_score as *mut libc::c_char as
               *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<high_score>() as libc::c_ulong);
    /* Save the version */
    sprintf(the_score.what.as_mut_ptr(),
            b"%ld.%ld.%ld\x00" as *const u8 as *const libc::c_char,
            VERSION_MAJOR as libc::c_long, VERSION_MINOR as libc::c_long,
            VERSION_PATCH as libc::c_long);
    /* Calculate and save the points */
    sprintf(the_score.pts.as_mut_ptr(),
            b"%9lu\x00" as *const u8 as *const libc::c_char, total_points());
    the_score.pts[9 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    /* Save the current gold */
    sprintf(the_score.gold.as_mut_ptr(),
            b"%9lu\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).au as libc::c_long);
    the_score.gold[9 as libc::c_int as usize] =
        '\u{0}' as i32 as libc::c_char;
    /* Save the current turn */
    sprintf(the_score.turns.as_mut_ptr(),
            b"%9lu\x00" as *const u8 as *const libc::c_char,
            turn as libc::c_long -
                (11520 as libc::c_int *
                     (42 as libc::c_int + 127 as libc::c_int)) as libc::c_long
                    * 10 as libc::c_long);
    the_score.turns[9 as libc::c_int as usize] =
        '\u{0}' as i32 as libc::c_char;
    /* Save the date in standard form (8 chars) */
    strftime(the_score.day.as_mut_ptr(), 9 as libc::c_int as size_t,
             b"%m/%d/%y\x00" as *const u8 as *const libc::c_char,
             localtime(&mut ct));
    /* Save the player name (15 chars) */
    sprintf(the_score.who.as_mut_ptr(),
            b"%-.15s\x00" as *const u8 as *const libc::c_char,
            player_name.as_mut_ptr());
    /* Save the player info XXX XXX XXX */
    sprintf(the_score.uid.as_mut_ptr(),
            b"%7u\x00" as *const u8 as *const libc::c_char, player_uid);
    sprintf(the_score.sex.as_mut_ptr(),
            b"%c\x00" as *const u8 as *const libc::c_char,
            if (*p_ptr).psex as libc::c_int != 0 {
                'm' as i32
            } else { 'f' as i32 });
    sprintf(the_score.p_r.as_mut_ptr(),
            b"%2d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).prace as libc::c_int);
    sprintf(the_score.p_s.as_mut_ptr(),
            b"%2d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).pracem as libc::c_int);
    sprintf(the_score.p_c.as_mut_ptr(),
            b"%2d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).pclass as libc::c_int);
    sprintf(the_score.p_cs.as_mut_ptr(),
            b"%2d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).pspec as libc::c_int);
    /* Save the level and such */
    sprintf(the_score.cur_lev.as_mut_ptr(),
            b"%3d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).lev as libc::c_int); /* -KMW- */
    sprintf(the_score.cur_dun.as_mut_ptr(),
            b"%3d\x00" as *const u8 as *const libc::c_char,
            dun_level as libc::c_int); /* -KMW- */
    sprintf(the_score.max_lev.as_mut_ptr(),
            b"%3d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).max_plv as libc::c_int);
    sprintf(the_score.max_dun.as_mut_ptr(),
            b"%3d\x00" as *const u8 as *const libc::c_char,
            *max_dlv.offset(dungeon_type as isize) as libc::c_int);
    sprintf(the_score.arena_number.as_mut_ptr(),
            b"%3d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).arena_number as libc::c_int);
    sprintf(the_score.inside_arena.as_mut_ptr(),
            b"%3d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).inside_arena as libc::c_int);
    sprintf(the_score.inside_quest.as_mut_ptr(),
            b"%3d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).inside_quest as libc::c_int);
    sprintf(the_score.exit_bldg.as_mut_ptr(),
            b"%3d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).exit_bldg as libc::c_int);
    /* Save the cause of death (31 chars) */
    sprintf(the_score.how.as_mut_ptr(),
            b"%-.31s\x00" as *const u8 as *const libc::c_char,
            died_from.as_mut_ptr());
    /* Lock (for writing) the highscore file, or fail */
    if fd_lock(highscore_fd, 1 as libc::c_int) != 0 {
        return 1 as libc::c_int
    }
    /* Add a new entry to the score list, see where it went */
    j = highscore_add(&mut the_score);
    /* Unlock the highscore file, or fail */
    if fd_lock(highscore_fd, 2 as libc::c_int) != 0 {
        return 1 as libc::c_int
    }
    /* Hack -- Display the top fifteen scores */
    if j < 10 as libc::c_int {
        display_scores_aux(0 as libc::c_int, 15 as libc::c_int, j,
                           0 as *mut high_score);
    } else {
        /* Display the scores surrounding the player */
        display_scores_aux(0 as libc::c_int, 5 as libc::c_int, j,
                           0 as *mut high_score);
        display_scores_aux(j - 2 as libc::c_int, j + 7 as libc::c_int, j,
                           0 as *mut high_score);
    }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Predict the players location, and display it.
 */
#[no_mangle]
pub unsafe extern "C" fn predict_score() -> errr {
    let mut j: libc::c_int = 0;
    let mut the_score: high_score =
        high_score{what: [0; 8],
                   pts: [0; 10],
                   gold: [0; 10],
                   turns: [0; 10],
                   day: [0; 10],
                   who: [0; 16],
                   uid: [0; 8],
                   sex: [0; 2],
                   p_r: [0; 3],
                   p_s: [0; 3],
                   p_c: [0; 3],
                   p_cs: [0; 3],
                   cur_lev: [0; 4],
                   cur_dun: [0; 4],
                   max_lev: [0; 4],
                   max_dun: [0; 4],
                   arena_number: [0; 4],
                   inside_arena: [0; 4],
                   inside_quest: [0; 4],
                   exit_bldg: [0; 4],
                   how: [0; 32],};
    /* No score file */
    if highscore_fd < 0 as libc::c_int {
        msg_print(b"Score file unavailable.\x00" as *const u8 as
                      *const libc::c_char);
        msg_print(0 as cptr);
        return 0 as libc::c_int
    }
    /* Save the version */
    sprintf(the_score.what.as_mut_ptr(),
            b"%ld.%ld.%ld\x00" as *const u8 as *const libc::c_char,
            VERSION_MAJOR as libc::c_long, VERSION_MINOR as libc::c_long,
            VERSION_PATCH as libc::c_long);
    /* Calculate and save the points */
    sprintf(the_score.pts.as_mut_ptr(),
            b"%9lu\x00" as *const u8 as *const libc::c_char, total_points());
    the_score.pts[9 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    /* Save the current gold */
    sprintf(the_score.gold.as_mut_ptr(),
            b"%9lu\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).au as libc::c_long);
    the_score.gold[9 as libc::c_int as usize] =
        '\u{0}' as i32 as libc::c_char;
    /* Save the current turn */
    sprintf(the_score.turns.as_mut_ptr(),
            b"%9lu\x00" as *const u8 as *const libc::c_char,
            turn as libc::c_long -
                (11520 as libc::c_int *
                     (42 as libc::c_int + 127 as libc::c_int)) as libc::c_long
                    * 10 as libc::c_long);
    the_score.turns[9 as libc::c_int as usize] =
        '\u{0}' as i32 as libc::c_char;
    /* Hack -- no time needed */
    strcpy(the_score.day.as_mut_ptr(),
           b"TODAY\x00" as *const u8 as *const libc::c_char);
    /* Save the player name (15 chars) */
    sprintf(the_score.who.as_mut_ptr(),
            b"%-.15s\x00" as *const u8 as *const libc::c_char,
            player_name.as_mut_ptr());
    /* Save the player info XXX XXX XXX */
    sprintf(the_score.uid.as_mut_ptr(),
            b"%7u\x00" as *const u8 as *const libc::c_char, player_uid);
    sprintf(the_score.sex.as_mut_ptr(),
            b"%c\x00" as *const u8 as *const libc::c_char,
            if (*p_ptr).psex as libc::c_int != 0 {
                'm' as i32
            } else { 'f' as i32 });
    sprintf(the_score.p_r.as_mut_ptr(),
            b"%2d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).prace as libc::c_int);
    sprintf(the_score.p_s.as_mut_ptr(),
            b"%2d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).pracem as libc::c_int);
    sprintf(the_score.p_c.as_mut_ptr(),
            b"%2d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).pclass as libc::c_int);
    sprintf(the_score.p_cs.as_mut_ptr(),
            b"%2d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).pspec as libc::c_int);
    /* Save the level and such */
    sprintf(the_score.cur_lev.as_mut_ptr(),
            b"%3d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).lev as libc::c_int); /* -KMW- */
    sprintf(the_score.cur_dun.as_mut_ptr(),
            b"%3d\x00" as *const u8 as *const libc::c_char,
            dun_level as libc::c_int); /* -KMW- */
    sprintf(the_score.max_lev.as_mut_ptr(),
            b"%3d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).max_plv as libc::c_int);
    sprintf(the_score.max_dun.as_mut_ptr(),
            b"%3d\x00" as *const u8 as *const libc::c_char,
            *max_dlv.offset(dungeon_type as isize) as libc::c_int);
    sprintf(the_score.arena_number.as_mut_ptr(),
            b"%3d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).arena_number as libc::c_int);
    sprintf(the_score.inside_arena.as_mut_ptr(),
            b"%3d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).inside_arena as libc::c_int);
    sprintf(the_score.inside_quest.as_mut_ptr(),
            b"%3d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).inside_quest as libc::c_int);
    sprintf(the_score.exit_bldg.as_mut_ptr(),
            b"%3d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).exit_bldg as libc::c_int);
    /* Hack -- no cause of death */
    strcpy(the_score.how.as_mut_ptr(),
           b"nobody (yet!)\x00" as *const u8 as *const libc::c_char);
    /* See where the entry would be placed */
    j = highscore_where(&mut the_score);
    /* Hack -- Display the top fifteen scores */
    if j < 10 as libc::c_int {
        display_scores_aux(0 as libc::c_int, 15 as libc::c_int, j,
                           &mut the_score);
    } else {
        /* Display some "useful" scores */
        display_scores_aux(0 as libc::c_int, 5 as libc::c_int,
                           -(1 as libc::c_int), 0 as *mut high_score);
        display_scores_aux(j - 2 as libc::c_int, j + 7 as libc::c_int, j,
                           &mut the_score);
    }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Change the player into a King!                        -RAK-
 */
unsafe extern "C" fn kingly() {
    /* Hack -- retire in town */
    dun_level = 0 as libc::c_int as s16b;
    /* Fake death */
    strcpy(died_from.as_mut_ptr(),
           b"Ripe Old Age\x00" as *const u8 as *const libc::c_char);
    /* Restore the experience */
    (*p_ptr).exp = (*p_ptr).max_exp;
    /* Restore the level */
    (*p_ptr).lev = (*p_ptr).max_plv;
    /* Hack -- Instant Gold */
    (*p_ptr).au =
        ((*p_ptr).au as libc::c_long + 10000000 as libc::c_long) as s32b;
    /* Clear screen */
    Term_clear();
    /* Would like to see something more Tolkienian here... */
    /* Display a crown */
    put_str(b"#\x00" as *const u8 as *const libc::c_char, 1 as libc::c_int,
            34 as libc::c_int);
    put_str(b"#####\x00" as *const u8 as *const libc::c_char,
            2 as libc::c_int, 32 as libc::c_int);
    put_str(b"#\x00" as *const u8 as *const libc::c_char, 3 as libc::c_int,
            34 as libc::c_int);
    put_str(b",,,  $$$  ,,,\x00" as *const u8 as *const libc::c_char,
            4 as libc::c_int, 28 as libc::c_int);
    put_str(b",,=$   \"$$$$$\"   $=,,\x00" as *const u8 as
                *const libc::c_char, 5 as libc::c_int, 24 as libc::c_int);
    put_str(b",$$        $$$        $$,\x00" as *const u8 as
                *const libc::c_char, 6 as libc::c_int, 22 as libc::c_int);
    put_str(b"*>         <*>         <*\x00" as *const u8 as
                *const libc::c_char, 7 as libc::c_int, 22 as libc::c_int);
    put_str(b"$$         $$$         $$\x00" as *const u8 as
                *const libc::c_char, 8 as libc::c_int, 22 as libc::c_int);
    put_str(b"\"$$        $$$        $$\"\x00" as *const u8 as
                *const libc::c_char, 9 as libc::c_int, 22 as libc::c_int);
    put_str(b"\"$$       $$$       $$\"\x00" as *const u8 as
                *const libc::c_char, 10 as libc::c_int, 23 as libc::c_int);
    put_str(b"*#########*#########*\x00" as *const u8 as *const libc::c_char,
            11 as libc::c_int, 24 as libc::c_int);
    put_str(b"*#########*#########*\x00" as *const u8 as *const libc::c_char,
            12 as libc::c_int, 24 as libc::c_int);
    /* Display a message */
    put_str(b"Veni, Vidi, Vici!\x00" as *const u8 as *const libc::c_char,
            15 as libc::c_int, 26 as libc::c_int);
    put_str(b"I came, I saw, I conquered!\x00" as *const u8 as
                *const libc::c_char, 16 as libc::c_int, 21 as libc::c_int);
    put_str(format(b"All Hail the Mighty %s!\x00" as *const u8 as
                       *const libc::c_char, (*sp_ptr).winner) as cptr,
            17 as libc::c_int, 22 as libc::c_int);
    /* Flush input */
    flush();
    /* Wait for response */
    pause_line(23 as libc::c_int);
}
/*
 * Wipe the saved levels
 */
#[no_mangle]
pub unsafe extern "C" fn wipe_saved() {
    let mut d: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut od: libc::c_int = dungeon_type as libc::c_int;
    let mut ol: libc::c_int = dun_level as libc::c_int;
    d = 0 as libc::c_int;
    while d < max_d_idx as libc::c_int {
        let mut d_ptr: *mut dungeon_info_type =
            &mut *d_info.offset(d as isize) as *mut dungeon_info_type;
        l = (*d_ptr).mindepth as libc::c_int;
        while l <= (*d_ptr).maxdepth as libc::c_int {
            let mut buf: [libc::c_char; 10] = [0; 10];
            dun_level = l as s16b;
            dungeon_type = d as byte_hack;
            if get_dungeon_save(buf.as_mut_ptr()) != 0 {
                let mut tmp: [libc::c_char; 80] = [0; 80];
                let mut name: [libc::c_char; 1024] = [0; 1024];
                sprintf(tmp.as_mut_ptr(),
                        b"%s.%s\x00" as *const u8 as *const libc::c_char,
                        player_base.as_mut_ptr(), buf.as_mut_ptr());
                path_build(name.as_mut_ptr(), 1024 as libc::c_int,
                           ANGBAND_DIR_SAVE, tmp.as_mut_ptr() as cptr);
                /* Remove the dungeon save file */
                fd_kill(name.as_mut_ptr() as cptr);
            }
            l += 1
        }
        d += 1
    }
    dungeon_type = od as byte_hack;
    dun_level = ol as s16b;
}
/*
 * Close up the current game (player may or may not be dead)
 *
 * This function is called only from "main.c" and "signals.c".
 */
#[no_mangle]
pub unsafe extern "C" fn close_game() {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Handle stuff */
    handle_stuff();
    /* Flush the messages */
    msg_print(0 as cptr);
    /* Flush the input */
    flush();
    /* No suspending now */
    signals_ignore_tstp();
    /* Hack -- Character is now "icky" */
    character_icky = 1 as libc::c_int as bool_;
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_APEX,
               b"scores.raw\x00" as *const u8 as *const libc::c_char);
    /* Open the high score file, for reading/writing */
    highscore_fd = fd_open(buf.as_mut_ptr() as cptr, 0o2 as libc::c_int);
    /* Handle death */
    if death != 0 {
        /* Handle retirement */
        if total_winner != 0 {
            /* Write a note, if that option is on */
            if take_notes != 0 { add_note_type(2 as libc::c_int); }
            kingly();
        }
        /* Wipe the saved levels */
        wipe_saved();
        /* Save memories */
        if save_player() == 0 {
            msg_print(b"Death save failed!\x00" as *const u8 as
                          *const libc::c_char);
        }
        /* You are dead */
        print_tomb();
        /* Show more info */
        show_info();
        /* Write a note */
        if take_notes != 0 {
            let mut long_day: [libc::c_char; 30] = [0; 30];
            let mut buf_0: [libc::c_char; 80] = [0; 80];
            let mut ct: time_t = time(0 as *mut libc::c_void as *mut time_t);
            /* Get the date */
            strftime(long_day.as_mut_ptr(), 30 as libc::c_int as size_t,
                     b"%Y-%m-%d at %H:%M:%S\x00" as *const u8 as
                         *const libc::c_char, localtime(&mut ct));
            /* Create string */
            sprintf(buf_0.as_mut_ptr(),
                    b"\n%s was killed by %s on %s\n\x00" as *const u8 as
                        *const libc::c_char, player_name.as_mut_ptr(),
                    died_from.as_mut_ptr(), long_day.as_mut_ptr());
            /* Output to the notes file */
            output_note(buf_0.as_mut_ptr());
        }
        /* Handle score, show Top scores */
        top_twenty();
    } else {
        /* Still alive */
        is_autosave = 0 as libc::c_int as bool_;
        /* Save the game */
        do_cmd_save_game();
        /* If note-taking enabled, write session end to notes file */
        if take_notes != 0 { add_note_type(3 as libc::c_int); }
        /* Prompt for scores XXX XXX XXX */
        prt(b"Press Return (or Escape).\x00" as *const u8 as
                *const libc::c_char, 0 as libc::c_int, 40 as libc::c_int);
        /* Predict score (or ESCAPE) */
        if inkey() as libc::c_int != '\u{1b}' as i32 { predict_score(); }
    }
    /* Shut the high score file */
    fd_close(highscore_fd);
    /* Forget the high score fd */
    highscore_fd = -(1 as libc::c_int);
    /* Allow suspending now */
    signals_handle_tstp();
}
/*
 * Grab a randomly selected line in lib/file/file_name
 */
#[no_mangle]
pub unsafe extern "C" fn get_rnd_line(mut file_name: *mut libc::c_char,
                                      mut output: *mut libc::c_char) -> errr {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut lines: libc::c_int = 0 as libc::c_int;
    let mut line: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    /* Clear the output buffer */
    strcpy(output, b"\x00" as *const u8 as *const libc::c_char);
    /* test hack */
    if wizard as libc::c_int != 0 && cheat_xtra as libc::c_int != 0 {
        msg_print(file_name as cptr);
    }
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_FILE,
               file_name as cptr);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Failed */
    if fp.is_null() { return -(1 as libc::c_int) }
    /* Read the first line */
    if 0 as libc::c_int !=
           my_fgets(fp, buf.as_mut_ptr(), 80 as libc::c_int as huge_hack) {
        my_fclose(fp);
        return -(1 as libc::c_int)
    }
    /* Retrieve number of valid lines in the file */
    lines = atoi(buf.as_mut_ptr());
    /* Pick a line in the file */
    line = Rand_div(lines) + 1 as libc::c_int;
    /*
	 * Scan through the file XXX XXX XXX
	 * Seemingly wrong use of the counter is justified by the
	 * stupid 'buffer' lines in the random text files.
	 */
    i = 0 as libc::c_int;
    while i <= line {
        if 0 as libc::c_int !=
               my_fgets(fp, buf.as_mut_ptr(), 80 as libc::c_int as huge_hack)
           {
            my_fclose(fp);
            return -(1 as libc::c_int)
        }
        /* Found the line */
        if i == line { break ; }
        i += 1
    }
    /* Copy the line to the output buffer */
    strcpy(output, buf.as_mut_ptr());
    /* Close the file */
    my_fclose(fp);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Read line'th line file the file
 * and return pointer to it, or NULL if it fails.
 *
 * Nuked the static buffer. Caller should provide one. -- pelpel
 *
 * Caution: 'linbuf' should be at least 80 byte long.
 */
#[no_mangle]
pub unsafe extern "C" fn get_line(mut fname: *mut libc::c_char,
                                  mut fdir: cptr,
                                  mut linbuf: *mut libc::c_char,
                                  mut line: libc::c_int)
 -> *mut libc::c_char {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Don't count the first line in the file, which is a comment line */
    line += 1;
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, fdir, fname as cptr);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Failed */
    if fp.is_null() { return 0 as *mut libc::c_char }
    /* Read past specified number of lines */
    i = 0 as libc::c_int;
    while i <= line {
        /* Oops */
        if my_fgets(fp, linbuf, 80 as libc::c_int as huge_hack) !=
               0 as libc::c_int {
            my_fclose(fp);
            return 0 as *mut libc::c_char
        }
        i += 1
    }
    my_fclose(fp);
    return linbuf;
}
/*
 * Return a line for a speaking unique, by Matt G.
 *
 * XXX XXX XXX Opening a file and scanning it through whenever a unique
 * tries to say something? Something like DELAY_LOAD_?_TEXT would be
 * much better -- pelpel
 *
 * XXX XXX XXX I must say the original is an extremely poor and unreliable
 * implementation...  I removed noxious flag -- I'm too stupid to
 * understand such complexities -- and added extra error checkings
 * and made sure fd is always closed -- pelpel
 */
#[no_mangle]
pub unsafe extern "C" fn get_xtra_line(mut file_name: *mut libc::c_char,
                                       mut m_ptr: *mut monster_type,
                                       mut output: *mut libc::c_char)
 -> errr {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut line: libc::c_int = 0;
    let mut num_entries: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut mnum: libc::c_int = 0;
    /* Clear the message buffer */
    strcpy(output, b"\x00" as *const u8 as *const libc::c_char);
    /* test and DEBUG hack */
    if wizard as libc::c_int != 0 && cheat_xtra as libc::c_int != 0 {
        msg_print(file_name as cptr);
    }
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_FILE,
               file_name as cptr);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Failed */
    if fp.is_null() { return -(1 as libc::c_int) }
    /* Monster number we are looking for */
    mnum = (*m_ptr).r_idx as libc::c_int;
    loop 
         /* Find matching N: line */
         {
        let mut n: libc::c_int = 0;
        /* Read a line */
        if my_fgets(fp, buf.as_mut_ptr(), 90 as libc::c_int as huge_hack) !=
               0 as libc::c_int {
            my_fclose(fp);
            return -(1 as libc::c_int)
        }
        /* Not a N: line */
        if buf[0 as libc::c_int as usize] as libc::c_int != 'N' as i32 {
            continue ;
        }
        /* Skip "N:" and parse off a number */
        sscanf(buf.as_mut_ptr().offset(2 as libc::c_int as isize),
               b"%d\x00" as *const u8 as *const libc::c_char,
               &mut n as *mut libc::c_int);
        /* Match found */
        if n == mnum { break ; }
    }
    loop 
         /* Retrieve number of normal messages */
         /* Read next line */
         {
        if my_fgets(fp, buf.as_mut_ptr(), 90 as libc::c_int as huge_hack) !=
               0 as libc::c_int {
            my_fclose(fp);
            return -(1 as libc::c_int)
        }
        /* The first line not beginning with 'N:' holds number of lines */
        if !(buf[0 as libc::c_int as usize] as libc::c_int != 'N' as i32) {
            continue ;
        }
        num_entries = atoi(buf.as_mut_ptr());
        break ;
    }
    /* The monster is afraid */
    if (*m_ptr).monfear != 0 {
        /* Read past normal lines */
        line = 0 as libc::c_int;
        while line < num_entries + 1 as libc::c_int {
            if my_fgets(fp, buf.as_mut_ptr(), 90 as libc::c_int as huge_hack)
                   != 0 {
                my_fclose(fp);
                return -(1 as libc::c_int)
            }
            line += 1
        }
        /* Retrieve number of 'afraid' lines */
        num_entries = atoi(buf.as_mut_ptr())
    }
    /* Pick a random line */
    line = Rand_div(num_entries);
    /* test and DEBUG hack */
    if wizard as libc::c_int != 0 && cheat_xtra as libc::c_int != 0 {
        sprintf(buf.as_mut_ptr(),
                b"Line number %d\x00" as *const u8 as *const libc::c_char,
                line);
        msg_print(buf.as_mut_ptr() as cptr);
    }
    /* Find the selected line */
    i = 0 as libc::c_int;
    while i <= line {
        /* Oops */
        if 0 as libc::c_int !=
               my_fgets(fp, buf.as_mut_ptr(), 90 as libc::c_int as huge_hack)
           {
            my_fclose(fp);
            return -(1 as libc::c_int)
        }
        /* Found it */
        if i == line { break ; }
        i += 1
    }
    /* Copy it to the output buffer */
    strcpy(output, buf.as_mut_ptr());
    /* Close the file */
    my_fclose(fp);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Handle signals -- suspend
 *
 * Actually suspend the game, and then resume cleanly
 */
unsafe extern "C" fn handle_signal_suspend(mut sig: libc::c_int) {
    /* Disable handler */
    signal(sig,
           ::std::mem::transmute::<libc::intptr_t,
                                   __sighandler_t>(1 as libc::c_int as
                                                       libc::intptr_t));
    /* Flush output */
    Term_fresh();
    /* Suspend the "Term" */
    Term_xtra(11 as libc::c_int, 0 as libc::c_int);
    /* Suspend ourself */
    kill(0 as libc::c_int, 19 as libc::c_int);
    /* Resume the "Term" */
    Term_xtra(11 as libc::c_int, 1 as libc::c_int);
    /* Redraw the term */
    Term_redraw();
    /* Flush the term */
    Term_fresh();
    /* Restore handler */
    signal(sig,
           Some(handle_signal_suspend as
                    unsafe extern "C" fn(_: libc::c_int) -> ()));
}
/*
 * Ignore SIGTSTP signals (keyboard suspend)
 */
#[no_mangle]
pub unsafe extern "C" fn signals_ignore_tstp() {
    signal(20 as libc::c_int,
           ::std::mem::transmute::<libc::intptr_t,
                                   __sighandler_t>(1 as libc::c_int as
                                                       libc::intptr_t));
}
/*
 * Handle SIGTSTP signals (keyboard suspend)
 */
#[no_mangle]
pub unsafe extern "C" fn signals_handle_tstp() {
    signal(20 as libc::c_int,
           Some(handle_signal_suspend as
                    unsafe extern "C" fn(_: libc::c_int) -> ()));
}
/*
 * Prepare to handle the relevant signals
 */
#[no_mangle]
pub unsafe extern "C" fn signals_init() {
    signal(1 as libc::c_int,
           ::std::mem::transmute::<libc::intptr_t,
                                   __sighandler_t>(1 as libc::c_int as
                                                       libc::intptr_t));
    signal(20 as libc::c_int,
           Some(handle_signal_suspend as
                    unsafe extern "C" fn(_: libc::c_int) -> ()));
}
/* HANDLE_SIGNALS */
/* HANDLE_SIGNALS */
