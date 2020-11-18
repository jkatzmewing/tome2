use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut sex_info: [player_sex; 3];
    #[no_mangle]
    static mut stat_names: [cptr; 6];
    #[no_mangle]
    static mut inscription_info: [inscription_info_type; 8];
    #[no_mangle]
    static mut death: bool_;
    #[no_mangle]
    static mut wizard: bool_;
    #[no_mangle]
    static mut total_winner: u16b;
    #[no_mangle]
    static mut has_won: u16b;
    #[no_mangle]
    static mut noscore: u16b;
    #[no_mangle]
    static mut inkey_scan: bool_;
    #[no_mangle]
    static mut inven_cnt: s16b;
    #[no_mangle]
    static mut equip_cnt: s16b;
    #[no_mangle]
    static mut hack_corruption: bool_;
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
    static mut player_name: [libc::c_char; 32];
    #[no_mangle]
    static mut player_base: [libc::c_char; 32];
    #[no_mangle]
    static mut history: [[libc::c_char; 60]; 4];
    #[no_mangle]
    static mut savefile: [libc::c_char; 1024];
    #[no_mangle]
    static mut max_ab_idx: u16b;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn quit(str: cptr);
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn ralloc(len: huge_hack) -> vptr;
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
    fn randnor(mean: libc::c_int, stand: libc::c_int) -> s16b;
    #[no_mangle]
    fn damroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn Term_fresh() -> errr;
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
    fn Term_clear() -> errr;
    #[no_mangle]
    static mut wild_map: *mut *mut wilderness_map;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn toupper(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut max_real_towns: u16b;
    #[no_mangle]
    static mut max_towns: u16b;
    #[no_mangle]
    static mut town_info: *mut town_type;
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
    static mut alchemist_known_egos: [u32b; 32];
    #[no_mangle]
    static mut alchemist_known_artifacts: [u32b; 6];
    #[no_mangle]
    static mut alchemist_gained: u32b;
    #[no_mangle]
    static mut player_hp: [s16b; 50];
    #[no_mangle]
    static mut ab_info: *mut ability_type;
    #[no_mangle]
    static mut s_info: *mut skill_type;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut a_info: *mut artifact_type;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut d_info: *mut dungeon_info_type;
    #[no_mangle]
    static mut c_head: *mut header;
    #[no_mangle]
    static mut class_info: *mut player_class;
    #[no_mangle]
    static mut c_name: *mut libc::c_char;
    #[no_mangle]
    static mut c_text: *mut libc::c_char;
    #[no_mangle]
    static mut meta_class_info: *mut meta_class_type;
    #[no_mangle]
    static mut rp_head: *mut header;
    #[no_mangle]
    static mut race_info: *mut player_race;
    #[no_mangle]
    static mut rp_name: *mut libc::c_char;
    #[no_mangle]
    static mut rp_text: *mut libc::c_char;
    #[no_mangle]
    static mut rmp_head: *mut header;
    #[no_mangle]
    static mut race_mod_info: *mut player_race_mod;
    #[no_mangle]
    static mut rmp_name: *mut libc::c_char;
    #[no_mangle]
    static mut rmp_text: *mut libc::c_char;
    #[no_mangle]
    static mut t_info: *mut trap_type;
    #[no_mangle]
    static mut ANGBAND_DIR_FILE: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_SAVE: cptr;
    #[no_mangle]
    static mut get_mon_num_hook:
           Option<unsafe extern "C" fn(_: libc::c_int) -> bool_>;
    #[no_mangle]
    static mut max_wild_x: u16b;
    #[no_mangle]
    static mut max_wild_y: u16b;
    #[no_mangle]
    static mut max_s_idx: u16b;
    #[no_mangle]
    static mut max_r_idx: u16b;
    #[no_mangle]
    static mut max_k_idx: u16b;
    #[no_mangle]
    static mut max_a_idx: u16b;
    #[no_mangle]
    static mut max_d_idx: u16b;
    #[no_mangle]
    static mut max_t_idx: u16b;
    #[no_mangle]
    static mut max_rp_idx: u16b;
    #[no_mangle]
    static mut max_mc_idx: u16b;
    #[no_mangle]
    static mut max_rmp_idx: u16b;
    #[no_mangle]
    static mut max_st_idx: u16b;
    #[no_mangle]
    static mut random_artifacts: [random_artifact; 84];
    #[no_mangle]
    static mut spell_num: s16b;
    #[no_mangle]
    static mut rune_spells: [rune_spell; 100];
    #[no_mangle]
    static mut rune_num: s16b;
    #[no_mangle]
    static mut fates: [fate; 200];
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut max_dlv: *mut s16b;
    #[no_mangle]
    static mut total_bounties: u32b;
    #[no_mangle]
    static mut doppleganger: s16b;
    #[no_mangle]
    static mut autoroll: bool_;
    #[no_mangle]
    static mut point_based: bool_;
    #[no_mangle]
    static mut maximize: bool_;
    #[no_mangle]
    static mut preserve: bool_;
    #[no_mangle]
    static mut special_lvls: bool_;
    #[no_mangle]
    static mut ironman_rooms: bool_;
    #[no_mangle]
    static mut take_notes: bool_;
    #[no_mangle]
    static mut plots: [s16b; 7];
    #[no_mangle]
    static mut random_quests: [random_quest; 99];
    #[no_mangle]
    static mut special_lvl: [*mut bool_; 128];
    #[no_mangle]
    static mut previous_char: birther;
    #[no_mangle]
    static mut bg: *mut hist_type;
    #[no_mangle]
    static mut max_bg_idx: libc::c_int;
    #[no_mangle]
    static mut quest: *mut quest_type;
    #[no_mangle]
    static mut linear_stats: bool_;
    #[no_mangle]
    static mut max_corruptions: s16b;
    #[no_mangle]
    static mut game_module: cptr;
    #[no_mangle]
    static mut deity_info: *mut deity_type;
    #[no_mangle]
    static mut max_gods: s32b;
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn my_fclose(fff: *mut FILE) -> errr;
    #[no_mangle]
    fn process_player_name(sf: bool_);
    #[no_mangle]
    fn fd_close(fd: libc::c_int) -> errr;
    #[no_mangle]
    fn fd_open(file: cptr, flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn my_fgets(fff: *mut FILE, buf: *mut libc::c_char, n: huge_hack) -> errr;
    #[no_mangle]
    fn my_fopen(file: cptr, mode: cptr) -> *mut FILE;
    #[no_mangle]
    fn path_build(buf: *mut libc::c_char, max: libc::c_int, path: cptr,
                  file: cptr) -> errr;
    #[no_mangle]
    fn get_player_race_name(pr: libc::c_int, ps: libc::c_int) -> cptr;
    #[no_mangle]
    fn askfor_aux(buf: *mut libc::c_char, len: libc::c_int) -> bool_;
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn fd_kill(file: cptr) -> errr;
    #[no_mangle]
    fn get_check(prompt: cptr) -> bool_;
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn put_str(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn c_put_str(attr: byte_hack, str: cptr, row: libc::c_int,
                 col: libc::c_int);
    #[no_mangle]
    fn call_lua(function: cptr, args: cptr, ret: cptr, _: ...) -> bool_;
    #[no_mangle]
    fn get_line(fname: *mut libc::c_char, fdir: cptr,
                linbuf: *mut libc::c_char, line: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn select_bounties();
    #[no_mangle]
    fn store_init(town_num: libc::c_int, store_num: libc::c_int);
    #[no_mangle]
    fn create_stores_stock(t: libc::c_int);
    #[no_mangle]
    fn message_add(type_0: byte_hack, msg: cptr, color: byte_hack);
    #[no_mangle]
    fn inven_carry(o_ptr: *mut object_type, final_0: bool_) -> s16b;
    #[no_mangle]
    fn object_known(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_aware(o_ptr: *mut object_type);
    #[no_mangle]
    fn lookup_kind(tval: libc::c_int, sval: libc::c_int) -> s16b;
    #[no_mangle]
    fn object_prep(o_ptr: *mut object_type, k_idx: libc::c_int);
    #[no_mangle]
    fn has_ability(ab: libc::c_int) -> bool_;
    #[no_mangle]
    fn add_note_type(note_number: libc::c_int);
    #[no_mangle]
    fn select_default_melee();
    #[no_mangle]
    fn follow_god(god: libc::c_int, silent: bool_);
    #[no_mangle]
    fn apply_level_abilities(level: libc::c_int);
    #[no_mangle]
    fn recalc_skills(init: bool_);
    #[no_mangle]
    fn get_name();
    #[no_mangle]
    fn display_player(mode: libc::c_int);
    #[no_mangle]
    fn clear_from(row: libc::c_int);
    #[no_mangle]
    fn bell();
    #[no_mangle]
    fn do_cmd_help();
    #[no_mangle]
    fn update_stuff();
    #[no_mangle]
    fn flush();
    #[no_mangle]
    fn cnv_stat(i: libc::c_int, out_val: *mut libc::c_char);
    #[no_mangle]
    fn modify_stat_value(value: libc::c_int, amount: libc::c_int) -> s16b;
    #[no_mangle]
    fn init_skill(value: s32b, mod_0: s32b, i: libc::c_int);
    #[no_mangle]
    fn compute_skills(v: *mut s32b, m: *mut s32b, i: libc::c_int);
    #[no_mangle]
    fn quest_random_init_hook(q_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn get_mon_num(level: libc::c_int) -> s16b;
    #[no_mangle]
    fn get_mon_num_prep() -> errr;
    #[no_mangle]
    fn monster_quest(r_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn screen_load();
    #[no_mangle]
    fn do_cmd_options_aux(page: libc::c_int, info: cptr, read_only: bool_);
    #[no_mangle]
    fn screen_save();
    #[no_mangle]
    fn set_grace(v: s32b);
    #[no_mangle]
    fn exec_lua(file: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn wipe_m_list();
    #[no_mangle]
    fn object_wipe(o_ptr: *mut object_type);
    #[no_mangle]
    fn wipe_saved();
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
pub type uint_hack = libc::c_uint;
pub type huge_hack = libc::c_ulong;
pub type s16b = libc::c_short;
pub type u16b = libc::c_ushort;
pub type s32b = libc::c_int;
pub type u32b = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct header {
    pub info_num: u16b,
    pub name_size: u32b,
    pub text_size: u32b,
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
pub struct meta_class_type {
    pub name: [libc::c_char; 80],
    pub color: byte_hack,
    pub classes: *mut s16b,
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
pub struct random_artifact {
    pub name_full: [libc::c_char; 80],
    pub name_short: [libc::c_char; 80],
    pub level: byte_hack,
    pub attr: byte_hack,
    pub cost: u32b,
    pub activation: byte_hack,
    pub timeout: s16b,
    pub generated: byte_hack,
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
pub struct inscription_info_type {
    pub text: [libc::c_char; 40],
    pub when: byte_hack,
    pub know: bool_,
    pub mana: byte_hack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rune_spell {
    pub name: [libc::c_char; 30],
    pub type_0: s16b,
    pub rune2: s16b,
    pub mana: s16b,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct random_quest {
    pub type_0: byte_hack,
    pub r_idx: s16b,
    pub done: bool_,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct birther {
    pub sex: s16b,
    pub race: s16b,
    pub rmod: s16b,
    pub pclass: s16b,
    pub spec: s16b,
    pub quests: byte_hack,
    pub god: byte_hack,
    pub grace: s32b,
    pub god_favor: s32b,
    pub age: s16b,
    pub wt: s16b,
    pub ht: s16b,
    pub sc: s16b,
    pub au: s32b,
    pub stat: [s16b; 6],
    pub luck: s16b,
    pub chaos_patron: s16b,
    pub weapon: u32b,
    pub history: [[libc::c_char; 60]; 4],
    pub quick_ok: bool_,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hist_type {
    pub info: s32b,
    pub roll: byte_hack,
    pub chart: s16b,
    pub next: s16b,
    pub bonus: byte_hack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct skill_type {
    pub name: u32b,
    pub desc: u32b,
    pub action_desc: u32b,
    pub action_mkey: s16b,
    pub i_value: s32b,
    pub i_mod: s32b,
    pub value: s32b,
    pub mod_0: s32b,
    pub rate: s16b,
    pub uses: u32b,
    pub action: [s16b; 200],
    pub father: s16b,
    pub dev: bool_,
    pub order: s16b,
    pub hidden: bool_,
    pub random_gain_chance: byte_hack,
    pub flags1: u32b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ability_type {
    pub name: u32b,
    pub desc: u32b,
    pub action_desc: u32b,
    pub action_mkey: s16b,
    pub cost: s16b,
    pub acquired: bool_,
    pub skills: [s16b; 10],
    pub skill_levels: [s16b; 10],
    pub stat: [s16b; 6],
    pub need_abilities: [s16b; 10],
    pub forbid_abilities: [s16b; 10],
}
/*
 * Maximum number of tries for selection of a proper quest monster
 */
/* Max quests */
static mut max_quests: byte_hack = 0 as libc::c_int as byte_hack;
/*
 * Current stats
 */
static mut stat_use: [s16b; 6] = [0; 6];
/*
 * Autoroll limit
 */
static mut stat_limit: [s16b; 6] = [0; 6];
/*
 * Autoroll matches
 */
static mut stat_match: [s32b; 6] = [0; 6];
/*
 * Autoroll round
 */
static mut auto_round: s32b = 0;
/*
 * Last round
 */
static mut last_round: s32b = 0;
/* Human */
static mut human_syllable1: [*mut libc::c_char; 65] =
    [b"Ab\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Ac\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Ad\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Af\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Agr\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Ast\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"As\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Al\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Adw\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Adr\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Ar\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"B\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Br\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"C\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Cr\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Ch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Cad\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"D\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Dr\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Dw\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Ed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Eth\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Et\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Er\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"El\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Eow\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"F\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Fr\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"G\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Gr\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Gw\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Gal\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Gl\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"H\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Ha\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Ib\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Jer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"K\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Ka\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Ked\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"L\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Loth\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Lar\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Leg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"M\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Mir\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"N\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Nyd\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Ol\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Oc\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"On\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"P\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Pr\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"R\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Rh\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"S\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Sev\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"T\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Tr\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Th\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"V\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Y\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Z\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"W\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"Wic\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
static mut human_syllable2: [*mut libc::c_char; 34] =
    [b"a\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ae\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"au\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ao\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"are\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ale\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ali\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ay\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ardo\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"e\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ei\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ea\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"eri\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"era\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ela\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"eli\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"enda\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"erra\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ia\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ie\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ire\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ira\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ila\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ili\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ira\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"igo\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"o\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"oa\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"oi\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"oe\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ore\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"u\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"y\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
static mut human_syllable3: [*mut libc::c_char; 70] =
    [b"a\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"and\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"b\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"bwyn\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"baen\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"bard\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"c\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ctred\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"cred\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"can\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"dan\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"don\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"der\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"dric\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"dfrid\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"dus\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"f\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"g\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"gord\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"gan\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"l\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"li\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"lgrin\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"lin\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"lith\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"lath\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"loth\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ld\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ldric\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ldan\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"m\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"mas\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"mos\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"mar\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"mond\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"nydd\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"nidd\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"nnon\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"nwan\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"nyth\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"nad\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"nn\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"nnor\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"nd\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"p\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"r\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"ron\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"rd\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"sh\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"seth\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"sean\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"t\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"th\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"tha\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"tlan\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"trem\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"tram\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"v\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"vudd\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"w\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"wan\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"win\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"wyn\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"wyr\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"wyr\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"wyth\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
/*
 * Random Name Generator
 * based on a Javascript by Michael Hensley
 * "http://geocities.com/timessquare/castle/6274/"
 */
unsafe extern "C" fn create_random_name(mut race: libc::c_int,
                                        mut name: *mut libc::c_char) {
    let mut syl1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut syl2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut syl3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idx: libc::c_int = 0;
    /* Paranoia */
    if name.is_null() { return }
    /* Select the monster type */
    match race { _ => { } }
    /* Create the monster name */
    /* Use human ones */
    idx =
        Rand_div((::std::mem::size_of::<[*mut libc::c_char; 65]>() as
                      libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut libc::c_char>()
                                                      as libc::c_ulong) as
                     s32b);
    syl1 = human_syllable1[idx as usize];
    idx =
        Rand_div((::std::mem::size_of::<[*mut libc::c_char; 34]>() as
                      libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut libc::c_char>()
                                                      as libc::c_ulong) as
                     s32b);
    syl2 = human_syllable2[idx as usize];
    idx =
        Rand_div((::std::mem::size_of::<[*mut libc::c_char; 70]>() as
                      libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut libc::c_char>()
                                                      as libc::c_ulong) as
                     s32b);
    syl3 = human_syllable3[idx as usize];
    /* Concatenate selected syllables */
    strnfmt(name, 32 as libc::c_int as uint_hack,
            b"%s%s%s\x00" as *const u8 as *const libc::c_char, syl1, syl2,
            syl3);
}
#[no_mangle]
pub unsafe extern "C" fn print_desc_aux(mut txt: cptr, mut y: libc::c_int,
                                        mut xx: libc::c_int) {
    let mut i: libc::c_int = -(1 as libc::c_int);
    let mut x: libc::c_int = xx;
    loop  {
        i += 1;
        if !(*txt.offset(i as isize) as libc::c_int != 0 as libc::c_int) {
            break ;
        }
        if *txt.offset(i as isize) as libc::c_int == '\n' as i32 {
            x = xx;
            y += 1
        } else {
            let fresh0 = x;
            x = x + 1;
            Term_putch(fresh0, y, 11 as libc::c_int as byte_hack,
                       *txt.offset(i as isize));
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn print_desc(mut txt: cptr) {
    print_desc_aux(txt, 12 as libc::c_int, 1 as libc::c_int);
}
/*
 * Save the current data for later
 */
unsafe extern "C" fn save_prev_data() {
    let mut i: libc::c_int = 0;
    /* ** Save the current data ***/
    /* Save the data */
    previous_char.sex = (*p_ptr).psex as s16b;
    previous_char.race = (*p_ptr).prace as s16b;
    previous_char.rmod = (*p_ptr).pracem as s16b;
    previous_char.pclass = (*p_ptr).pclass as s16b;
    previous_char.spec = (*p_ptr).pspec as s16b;
    previous_char.quests = max_quests;
    previous_char.god = (*p_ptr).pgod;
    previous_char.grace = (*p_ptr).grace;
    previous_char.age = (*p_ptr).age;
    previous_char.wt = (*p_ptr).wt;
    previous_char.ht = (*p_ptr).ht;
    previous_char.sc = (*p_ptr).sc;
    previous_char.au = (*p_ptr).au;
    /* Save the stats */
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        previous_char.stat[i as usize] = (*p_ptr).stat_max[i as usize];
        i += 1
    }
    previous_char.luck = (*p_ptr).luck_base;
    /* Save the chaos patron */
    previous_char.chaos_patron = (*p_ptr).chaos_patron;
    /* Save the weapon specialty */
    previous_char.weapon = 0 as libc::c_int as u32b;
    /* Save the history */
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        strcpy(previous_char.history[i as usize].as_mut_ptr(),
               history[i as usize].as_mut_ptr());
        i += 1
    };
}
/*
 * Load the previous data
 */
unsafe extern "C" fn load_prev_data(mut save: bool_) {
    let mut i: libc::c_int = 0;
    let mut temp: birther =
        birther{sex: 0,
                race: 0,
                rmod: 0,
                pclass: 0,
                spec: 0,
                quests: 0,
                god: 0,
                grace: 0,
                god_favor: 0,
                age: 0,
                wt: 0,
                ht: 0,
                sc: 0,
                au: 0,
                stat: [0; 6],
                luck: 0,
                chaos_patron: 0,
                weapon: 0,
                history: [[0; 60]; 4],
                quick_ok: 0,};
    /* ** Save the current data ***/
    /* Save the data */
    temp.age = (*p_ptr).age;
    temp.wt = (*p_ptr).wt;
    temp.ht = (*p_ptr).ht;
    temp.sc = (*p_ptr).sc;
    temp.au = (*p_ptr).au;
    /* Save the stats */
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        temp.stat[i as usize] = (*p_ptr).stat_max[i as usize];
        i += 1
    }
    temp.luck = (*p_ptr).luck_base;
    /* Save the chaos patron */
    temp.chaos_patron = (*p_ptr).chaos_patron;
    /* Save the weapon specialty */
    temp.weapon = 0 as libc::c_int as u32b;
    /* Save the history */
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        strcpy(temp.history[i as usize].as_mut_ptr(),
               history[i as usize].as_mut_ptr());
        i += 1
    }
    /* ** Load the previous data ***/
    /* Load the data */
    (*p_ptr).age = previous_char.age;
    (*p_ptr).wt = previous_char.wt;
    (*p_ptr).ht = previous_char.ht;
    (*p_ptr).sc = previous_char.sc;
    (*p_ptr).au = previous_char.au;
    /* Load the stats */
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        (*p_ptr).stat_max[i as usize] = previous_char.stat[i as usize];
        (*p_ptr).stat_cur[i as usize] = previous_char.stat[i as usize];
        i += 1
    }
    (*p_ptr).luck_base = previous_char.luck;
    (*p_ptr).luck_max = previous_char.luck;
    /* Load the chaos patron */
    (*p_ptr).chaos_patron = previous_char.chaos_patron;
    /* Load the history */
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        strcpy(history[i as usize].as_mut_ptr(),
               previous_char.history[i as usize].as_mut_ptr());
        i += 1
    }
    /* ** Save the current data ***/
    if save == 0 { return }
    /* Save the data */
    previous_char.age = temp.age;
    previous_char.wt = temp.wt;
    previous_char.ht = temp.ht;
    previous_char.sc = temp.sc;
    previous_char.au = temp.au;
    /* Save the stats */
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        previous_char.stat[i as usize] = temp.stat[i as usize];
        i += 1
    }
    previous_char.luck = temp.luck;
    /* Save the chaos patron */
    previous_char.chaos_patron = temp.chaos_patron;
    /* Save the weapon specialty */
    previous_char.weapon = temp.weapon;
    /* Save the history */
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        strcpy(previous_char.history[i as usize].as_mut_ptr(),
               temp.history[i as usize].as_mut_ptr());
        i += 1
    };
}
/*
 * Returns adjusted stat -JK-  Algorithm by -JWT-
 *
 * auto_roll is boolean and states maximum changes should be used rather
 * than random ones to allow specification of higher values to wait for
 *
 * The "p_ptr->maximize" code is important        -BEN-
 */
unsafe extern "C" fn adjust_stat(mut value: libc::c_int,
                                 mut amount: libc::c_int,
                                 mut auto_roll: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    /* Negative amounts */
    if amount < 0 as libc::c_int {
        /* Apply penalty */
        i = 0 as libc::c_int;
        while i < 0 as libc::c_int - amount {
            if value >= 18 as libc::c_int + 10 as libc::c_int {
                value -= 10 as libc::c_int
            } else if value > 18 as libc::c_int {
                value = 18 as libc::c_int
            } else if value > 3 as libc::c_int { value -= 1 }
            i += 1
        }
    } else if amount > 0 as libc::c_int {
        /* Positive amounts */
        /* Apply reward */
        i = 0 as libc::c_int;
        while i < amount {
            if value < 18 as libc::c_int {
                value += 1
            } else if (*p_ptr).maximize != 0 {
                value += 10 as libc::c_int
            } else if value < 18 as libc::c_int + 70 as libc::c_int {
                value +=
                    (if auto_roll != 0 {
                         15 as libc::c_int
                     } else {
                         (Rand_div(15 as libc::c_int)) + 1 as libc::c_int
                     }) + 5 as libc::c_int
            } else if value < 18 as libc::c_int + 90 as libc::c_int {
                value +=
                    (if auto_roll != 0 {
                         6 as libc::c_int
                     } else {
                         (Rand_div(6 as libc::c_int)) + 1 as libc::c_int
                     }) + 2 as libc::c_int
            } else if value < 18 as libc::c_int + 100 as libc::c_int {
                value += 1
            }
            i += 1
        }
    }
    /* Return the result */
    return value;
}
/*
 * Roll for a characters stats
 *
 * For efficiency, we include a chunk of "calc_bonuses()".
 */
unsafe extern "C" fn get_stats() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut bonus: libc::c_int = 0;
    let mut dice: [libc::c_int; 18] = [0; 18];
    loop 
         /* Roll and verify some stats */
         /* Roll some dice */
         {
        i = 0 as libc::c_int;
        j = i;
        while i < 18 as libc::c_int {
            /* Roll the dice */
            dice[i as usize] =
                Rand_div(3 as libc::c_int + i % 3 as libc::c_int) +
                    1 as libc::c_int;
            /* Collect the maximum */
            j += dice[i as usize];
            i += 1
        }
        /*
		 * Verify totals
		 *
		 * 57 was 54... I hate 'magic numbers' :< TY
		 *
		 * (d3 + d4 + d5)         ~= 7.5 (+- 4.5)
		 * with 5 makes avg. stat value of 12.5 (min 8, max 17)
		 *
		 * (d3 + d4 + d5) x 6 ~= 45 (+- 18)
		 *
		 * So the original value (still used by Vanilla as of 2.9.3)
		 * allows (avg - 2)..(avg + 8), while this Z version
		 * (avg - 2)..(avg + 11). I don't understand what TY meant
		 * by "magic numbers", but I like big stats :) -- pelpel
		 *
		 */
        if j > 42 as libc::c_int && j < 57 as libc::c_int { break ; }
    }
    /* Acquire the stats */
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        /* Extract 5 + 1d3 + 1d4 + 1d5 */
        j =
            5 as libc::c_int + dice[(3 as libc::c_int * i) as usize] +
                dice[(3 as libc::c_int * i + 1 as libc::c_int) as usize] +
                dice[(3 as libc::c_int * i + 2 as libc::c_int) as usize];
        /* Save that value */
        (*p_ptr).stat_max[i as usize] = j as s16b;
        /* Obtain a "bonus" for "race" and "class" */
        bonus =
            (*rp_ptr).r_adj[i as usize] as libc::c_int +
                (*rmp_ptr).r_adj[i as usize] as libc::c_int +
                (*cp_ptr).c_adj[i as usize] as libc::c_int;
        /* Variable stat maxes */
        if (*p_ptr).maximize != 0 {
            /* Start fully healed */
            (*p_ptr).stat_cur[i as usize] = (*p_ptr).stat_max[i as usize];
            /* Efficiency -- Apply the racial/class bonuses */
            stat_use[i as usize] =
                modify_stat_value((*p_ptr).stat_max[i as usize] as
                                      libc::c_int, bonus)
        } else {
            /* Fixed stat maxes */
            /* Apply the bonus to the stat (somewhat randomly) */
            stat_use[i as usize] =
                adjust_stat((*p_ptr).stat_max[i as usize] as libc::c_int,
                            bonus, 0 as libc::c_int) as s16b;
            (*p_ptr).stat_max[i as usize] = stat_use[i as usize];
            (*p_ptr).stat_cur[i as usize] = (*p_ptr).stat_max[i as usize]
        }
        /* Save the resulting stat maximum */
        /* No temporary drain (yet...) */
        (*p_ptr).stat_cnt[i as usize] = 0 as libc::c_int as s16b;
        (*p_ptr).stat_los[i as usize] = 0 as libc::c_int as s16b;
        i += 1
    }
    /* Get luck */
    (*p_ptr).luck_base =
        ((*rp_ptr).luck as libc::c_int + (*rmp_ptr).luck as libc::c_int +
             (-(5 as libc::c_int) +
                  Rand_div(1 as libc::c_int + 5 as libc::c_int -
                               -(5 as libc::c_int)))) as s16b;
    (*p_ptr).luck_max = (*p_ptr).luck_base;
}
/*
 * Roll for some info that the auto-roller ignores
 */
unsafe extern "C" fn get_extra() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut min_value: libc::c_int = 0;
    let mut max_value: libc::c_int = 0;
    /* Level one */
    (*p_ptr).lev = 1 as libc::c_int as s16b;
    (*p_ptr).max_plv = (*p_ptr).lev;
    /* Experience factor */
    (*p_ptr).expfact =
        ((*rp_ptr).r_exp as libc::c_int + (*rmp_ptr).r_exp as libc::c_int +
             (*cp_ptr).c_exp as libc::c_int) as u16b;
    /* Initialize arena and rewards information -KMW- */
    (*p_ptr).arena_number =
        0 as libc::c_int as s16b; /* only used for arena now -KMW- */
    (*p_ptr).inside_arena = 0 as libc::c_int as s16b;
    (*p_ptr).inside_quest = 0 as libc::c_int as s16b;
    (*p_ptr).exit_bldg = 1 as libc::c_int as bool_;
    /* Hitdice */
    (*p_ptr).hitdie =
        ((*rp_ptr).r_mhp as libc::c_int + (*rmp_ptr).r_mhp as libc::c_int +
             (*cp_ptr).c_mhp as libc::c_int) as byte_hack;
    /* Initial hitpoints */
    (*p_ptr).mhp = (*p_ptr).hitdie as s16b;
    /* Minimum hitpoints at highest level */
    min_value =
        50 as libc::c_int *
            ((*p_ptr).hitdie as libc::c_int - 1 as libc::c_int) *
            3 as libc::c_int / 8 as libc::c_int;
    min_value += 50 as libc::c_int;
    /* Maximum hitpoints at highest level */
    max_value =
        50 as libc::c_int *
            ((*p_ptr).hitdie as libc::c_int - 1 as libc::c_int) *
            5 as libc::c_int / 8 as libc::c_int;
    max_value += 50 as libc::c_int;
    /* Pre-calculate level 1 hitdice */
    player_hp[0 as libc::c_int as usize] = (*p_ptr).hitdie as s16b;
    loop 
         /* Roll out the hitpoints */
         /* Roll the hitpoint values */
         {
        i = 1 as libc::c_int;
        while i < 50 as libc::c_int {
            j = Rand_div((*p_ptr).hitdie as s32b) + 1 as libc::c_int;
            player_hp[i as usize] =
                (player_hp[(i - 1 as libc::c_int) as usize] as libc::c_int +
                     j) as s16b;
            i += 1
        }
        /* XXX Could also require acceptable "mid-level" hitpoints */
        /* Require "valid" hitpoints at highest level */
        if (player_hp[(50 as libc::c_int - 1 as libc::c_int) as usize] as
                libc::c_int) < min_value {
            continue ;
        }
        if !(player_hp[(50 as libc::c_int - 1 as libc::c_int) as usize] as
                 libc::c_int > max_value) {
            break ;
        }
    }
    (*p_ptr).tactic = 4 as libc::c_int as libc::c_char;
    (*p_ptr).movement = 4 as libc::c_int as libc::c_char;
    /* SHOW_LIFE_RATE */
}
/*
 * Get the racial history, and social class, using the "history charts".
 */
unsafe extern "C" fn get_history() {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut chart: libc::c_int = 0;
    let mut roll: libc::c_int = 0;
    let mut social_class: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 240] = [0; 240];
    /* Clear the previous history strings */
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        history[i as usize][0 as libc::c_int as usize] =
            '\u{0}' as i32 as libc::c_char;
        i += 1
    }
    /* Clear the history text */
    buf[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    /* Initial social class */
    social_class = Rand_div(4 as libc::c_int) + 1 as libc::c_int;
    /* Starting place */
    chart = (*rp_ptr).chart as libc::c_int;
    /* Process the history */
    while chart != 0 {
        /* Start over */
        i = 0 as libc::c_int;
        /* Roll for nobility */
        roll = Rand_div(100 as libc::c_int) + 1 as libc::c_int;
        /* Access the proper entry in the table */
        while chart != (*bg.offset(i as isize)).chart as libc::c_int ||
                  roll > (*bg.offset(i as isize)).roll as libc::c_int {
            i += 1
        }
        /* Acquire the textual history */
        strcat(buf.as_mut_ptr(),
               rp_text.offset((*bg.offset(i as isize)).info as isize));
        /* Add in the social class */
        social_class +=
            (*bg.offset(i as isize)).bonus as libc::c_int - 50 as libc::c_int;
        /* Enter the next chart */
        chart = (*bg.offset(i as isize)).next as libc::c_int
    }
    /* Verify social class */
    if social_class > 100 as libc::c_int {
        social_class = 100 as libc::c_int
    } else if social_class < 1 as libc::c_int {
        social_class = 1 as libc::c_int
    }
    /* Save the social class */
    (*p_ptr).sc = social_class as s16b;
    /* Skip leading spaces */
    s = buf.as_mut_ptr();
    while *s as libc::c_int == ' ' as i32 {
        /* loop */
        s = s.offset(1)
    }
    /* Get apparent length */
    n = strlen(s) as libc::c_int;
    /* Kill trailing spaces */
    while n > 0 as libc::c_int &&
              *s.offset((n - 1 as libc::c_int) as isize) as libc::c_int ==
                  ' ' as i32 {
        n -= 1;
        *s.offset(n as isize) = '\u{0}' as i32 as libc::c_char
    }
    /* Start at first line */
    i = 0 as libc::c_int;
    loop 
         /* Collect the history */
         /* Extract remaining length */
         {
        n = strlen(s) as libc::c_int;
        /* All done */
        if n < 60 as libc::c_int {
            /* Save one line of history */
            let fresh1 = i;
            i = i + 1;
            strcpy(history[fresh1 as usize].as_mut_ptr(), s);
            break ;
        } else {
            /* Find a reasonable break-point */
            n = 60 as libc::c_int;
            while n > 0 as libc::c_int &&
                      *s.offset((n - 1 as libc::c_int) as isize) as
                          libc::c_int != ' ' as i32 {
                /* loop */
                n -= 1
            }
            /* Save next location */
            t = s.offset(n as isize);
            /* Wipe trailing spaces */
            while n > 0 as libc::c_int &&
                      *s.offset((n - 1 as libc::c_int) as isize) as
                          libc::c_int == ' ' as i32 {
                n -= 1;
                *s.offset(n as isize) = '\u{0}' as i32 as libc::c_char
            }
            /* Save one line of history */
            let fresh2 = i;
            i = i + 1;
            strcpy(history[fresh2 as usize].as_mut_ptr(), s);
            /* Start next line */
            s = t;
            while *s as libc::c_int == ' ' as i32 {
                /* loop */
                s = s.offset(1)
            }
        }
    };
}
/*
 * Fill the random_artifacts array with relevant info.
 */
#[no_mangle]
pub unsafe extern "C" fn init_randart() -> errr {
    let mut i: libc::c_int = 0;
    let mut cost: libc::c_long = 0;
    let mut ra_ptr: *mut random_artifact = 0 as *mut random_artifact;
    let mut buf: [libc::c_char; 80] = [0; 80];
    i = 0 as libc::c_int;
    while i < 84 as libc::c_int {
        ra_ptr =
            &mut *random_artifacts.as_mut_ptr().offset(i as isize) as
                *mut random_artifact;
        strcpy((*ra_ptr).name_short.as_mut_ptr(),
               get_line(b"rart_s.txt\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char, ANGBAND_DIR_FILE,
                        buf.as_mut_ptr(), i));
        strcpy((*ra_ptr).name_full.as_mut_ptr(),
               get_line(b"rart_f.txt\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char, ANGBAND_DIR_FILE,
                        buf.as_mut_ptr(), i));
        (*ra_ptr).attr =
            (Rand_div(15 as libc::c_int) + 1 as libc::c_int) as byte_hack;
        (*ra_ptr).activation = Rand_div(51 as libc::c_int) as byte_hack;
        (*ra_ptr).generated = 0 as libc::c_int as byte_hack;
        cost = randnor(0 as libc::c_int, 250 as libc::c_int) as libc::c_long;
        if cost < 0 as libc::c_int as libc::c_long {
            cost = 0 as libc::c_int as libc::c_long
        }
        (*ra_ptr).cost = cost as u32b;
        i += 1
    }
    return 0 as libc::c_int;
}
/*
 * A helper function for get_ahw(), also called by polymorph code
 */
#[no_mangle]
pub unsafe extern "C" fn get_height_weight() {
    let mut h_mean: libc::c_int = 0;
    let mut h_stddev: libc::c_int = 0;
    let mut w_mean: libc::c_int = 0;
    let mut w_stddev: libc::c_int = 0;
    /* Extract mean and standard deviation -- Male */
    if (*p_ptr).psex as libc::c_int == 1 as libc::c_int {
        h_mean =
            (*rp_ptr).m_b_ht as libc::c_int +
                (*rmp_ptr).m_b_ht as libc::c_int;
        h_stddev =
            (*rp_ptr).m_m_ht as libc::c_int +
                (*rmp_ptr).m_m_ht as libc::c_int;
        w_mean =
            (*rp_ptr).m_b_wt as libc::c_int +
                (*rmp_ptr).m_b_wt as libc::c_int;
        w_stddev =
            (*rp_ptr).m_m_wt as libc::c_int + (*rmp_ptr).m_m_wt as libc::c_int
    } else if (*p_ptr).psex as libc::c_int == 0 as libc::c_int {
        h_mean =
            (*rp_ptr).f_b_ht as libc::c_int +
                (*rmp_ptr).f_b_ht as libc::c_int;
        h_stddev =
            (*rp_ptr).f_m_ht as libc::c_int +
                (*rmp_ptr).f_m_ht as libc::c_int;
        w_mean =
            (*rp_ptr).f_b_wt as libc::c_int +
                (*rmp_ptr).f_b_wt as libc::c_int;
        w_stddev =
            (*rp_ptr).f_m_wt as libc::c_int + (*rmp_ptr).f_m_wt as libc::c_int
    } else {
        /* Female */
        /* Neuter XXX */
        h_mean =
            ((*rp_ptr).m_b_ht as libc::c_int +
                 (*rmp_ptr).m_b_ht as libc::c_int +
                 (*rp_ptr).f_b_ht as libc::c_int +
                 (*rmp_ptr).f_b_ht as libc::c_int) / 2 as libc::c_int;
        h_stddev =
            ((*rp_ptr).m_m_ht as libc::c_int +
                 (*rmp_ptr).m_m_ht as libc::c_int +
                 (*rp_ptr).f_m_ht as libc::c_int +
                 (*rmp_ptr).f_m_ht as libc::c_int) / 2 as libc::c_int;
        w_mean =
            ((*rp_ptr).m_b_wt as libc::c_int +
                 (*rmp_ptr).m_b_wt as libc::c_int +
                 (*rp_ptr).f_b_wt as libc::c_int +
                 (*rmp_ptr).f_b_wt as libc::c_int) / 2 as libc::c_int;
        w_stddev =
            ((*rp_ptr).m_m_wt as libc::c_int +
                 (*rmp_ptr).m_m_wt as libc::c_int +
                 (*rp_ptr).f_m_wt as libc::c_int +
                 (*rmp_ptr).f_m_wt as libc::c_int) / 2 as libc::c_int
    }
    /* Calculate height/weight */
    (*p_ptr).ht = randnor(h_mean, h_stddev);
    (*p_ptr).wt = randnor(w_mean, w_stddev);
    /* Weight/height shouldn't be negative */
    if ((*p_ptr).ht as libc::c_int) < 1 as libc::c_int {
        (*p_ptr).ht = 1 as libc::c_int as s16b
    }
    if ((*p_ptr).wt as libc::c_int) < 1 as libc::c_int {
        (*p_ptr).wt = 1 as libc::c_int as s16b
    };
}
/*
 * Computes character's age, height, and weight
 */
unsafe extern "C" fn get_ahw() {
    /* Calculate the age */
    (*p_ptr).age =
        ((*rp_ptr).b_age as libc::c_int + (*rmp_ptr).b_age as libc::c_int +
             (Rand_div((*rp_ptr).m_age as libc::c_int +
                           (*rmp_ptr).m_age as libc::c_int) +
                  1 as libc::c_int)) as s16b;
    /* Calculate the height/weight */
    get_height_weight();
}
/*
 * Get the player's starting money
 */
unsafe extern "C" fn get_money() {
    let mut i: libc::c_int = 0;
    let mut gold: libc::c_int = 0;
    /* Social Class determines starting gold */
    gold =
        (*p_ptr).sc as libc::c_int * 6 as libc::c_int +
            (Rand_div(100 as libc::c_int) + 1 as libc::c_int) +
            300 as libc::c_int;
    /* Process the stats */
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        /* Mega-Hack -- reduce gold for high stats */
        if stat_use[i as usize] as libc::c_int >=
               18 as libc::c_int + 50 as libc::c_int {
            gold -= 300 as libc::c_int
        } else if stat_use[i as usize] as libc::c_int >=
                      18 as libc::c_int + 20 as libc::c_int {
            gold -= 200 as libc::c_int
        } else if stat_use[i as usize] as libc::c_int > 18 as libc::c_int {
            gold -= 150 as libc::c_int
        } else {
            gold -=
                (stat_use[i as usize] as libc::c_int - 8 as libc::c_int) *
                    10 as libc::c_int
        }
        i += 1
    }
    /* Minimum 100 gold */
    if gold < 100 as libc::c_int { gold = 100 as libc::c_int }
    /* Save the gold */
    (*p_ptr).au = gold;
}
/*
 * Display stat values, subset of "put_stats()"
 *
 * See 'display_player()' for basic method.
 */
unsafe extern "C" fn birth_put_stats() {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut attr: byte_hack = 0;
    let mut buf: [libc::c_char; 80] = [0; 80];
    /* Put the stats (and percents) */
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        /* Put the stat */
        cnv_stat((*p_ptr).stat_use[i as usize] as libc::c_int,
                 buf.as_mut_ptr());
        c_put_str(13 as libc::c_int as byte_hack, buf.as_mut_ptr() as cptr,
                  2 as libc::c_int + i, 66 as libc::c_int);
        /* Put the percent */
        if stat_match[i as usize] != 0 {
            p =
                (1000 as libc::c_long * stat_match[i as usize] as libc::c_long
                     / auto_round as libc::c_long) as libc::c_int;
            attr =
                if p < 100 as libc::c_int {
                    11 as libc::c_int
                } else { 13 as libc::c_int } as byte_hack;
            strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                    b"%3d.%d%%\x00" as *const u8 as *const libc::c_char,
                    p / 10 as libc::c_int, p % 10 as libc::c_int);
            c_put_str(attr, buf.as_mut_ptr() as cptr, 2 as libc::c_int + i,
                      73 as libc::c_int);
        } else {
            /* Never happened */
            c_put_str(4 as libc::c_int as byte_hack,
                      b"(NONE)\x00" as *const u8 as *const libc::c_char,
                      2 as libc::c_int + i, 73 as libc::c_int);
        }
        i += 1
    };
}
/*
 * Clear all the global "character" data
 */
unsafe extern "C" fn player_wipe() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut powers: *mut bool_ = 0 as *mut bool_;
    let mut corruptions: *mut bool_ = 0 as *mut bool_;
    /* Wipe special levels */
    wipe_saved();
    /* Save the powers & corruptions */
    powers = (*p_ptr).powers;
    corruptions = (*p_ptr).corruptions;
    /* Hack -- zero the struct */
    p_ptr =
        memset(p_ptr as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<player_type>() as libc::c_ulong) as
            *mut player_type;
    /* Restore the powers & corruptions */
    (*p_ptr).powers = powers;
    (*p_ptr).corruptions = corruptions;
    /* Not dead yet */
    (*p_ptr).lives = 0 as libc::c_int;
    /* Wipe the corruptions */
    memset((*p_ptr).corruptions as *mut libc::c_char as *mut libc::c_void,
           0 as libc::c_int,
           (max_corruptions as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>() as
                                                libc::c_ulong));
    /* Wipe the history */
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 60 as libc::c_int {
            if j < 59 as libc::c_int {
                history[i as usize][j as usize] = ' ' as i32 as libc::c_char
            } else {
                history[i as usize][j as usize] =
                    '\u{0}' as i32 as libc::c_char
            }
            j += 1
        }
        i += 1
    }
    /* Wipe the towns */
    i = 0 as libc::c_int;
    while i < max_d_idx as libc::c_int {
        j = 0 as libc::c_int;
        while j < 128 as libc::c_int {
            *special_lvl[j as usize].offset(i as isize) =
                0 as libc::c_int as bool_;
            j += 1
        }
        i += 1
    }
    /* Wipe the towns */
    i = max_real_towns as libc::c_int + 1 as libc::c_int;
    while i < max_towns as libc::c_int {
        (*town_info.offset(i as isize)).flags = 0 as libc::c_int as byte_hack;
        i += 1
    }
    /* Wipe the quests */
    i = 0 as libc::c_int;
    while i < 26 as libc::c_int {
        (*quest.offset(i as isize)).status = 0 as libc::c_int as s16b;
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            (*quest.offset(i as isize)).data[j as usize] = 0 as libc::c_int;
            j += 1
        }
        i += 1
    }
    /* Wipe the rune spells */
    rune_num = 0 as libc::c_int as s16b;
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        strcpy(rune_spells[i as usize].name.as_mut_ptr(),
               b"\x00" as *const u8 as *const libc::c_char);
        rune_spells[i as usize].type_0 = 0 as libc::c_int as s16b;
        rune_spells[i as usize].rune2 = 0 as libc::c_int as s16b;
        rune_spells[i as usize].mana = 0 as libc::c_int as s16b;
        i += 1
    }
    /* No items */
    inven_cnt = 0 as libc::c_int as s16b;
    equip_cnt = 0 as libc::c_int as s16b;
    /* Clear the inventory */
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int {
        object_wipe(&mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize));
        i += 1
    }
    /* Generate random artifacts */
    init_randart();
    /* Start with no artifacts made yet */
    i = 0 as libc::c_int;
    while i < max_a_idx as libc::c_int {
        let mut a_ptr: *mut artifact_type =
            &mut *a_info.offset(i as isize) as *mut artifact_type;
        (*a_ptr).cur_num = 0 as libc::c_int as byte_hack;
        i += 1
    }
    /* Reset the "objects" */
    i = 1 as libc::c_int;
    while i < max_k_idx as libc::c_int {
        let mut k_ptr: *mut object_kind =
            &mut *k_info.offset(i as isize) as *mut object_kind;
        /* Reset "tried" */
        (*k_ptr).tried = 0 as libc::c_int as bool_;
        /* Reset "aware" */
        (*k_ptr).aware = 0 as libc::c_int as bool_;
        /* Reset "know" */
        (*k_ptr).know = 0 as libc::c_int as bool_;
        /* Reset "artifact" */
        (*k_ptr).artifact = 0 as libc::c_int as bool_;
        i += 1
    }
    /* Reset the "monsters" */
    i = 1 as libc::c_int;
    while i < max_r_idx as libc::c_int {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset(i as isize) as *mut monster_race;
        /* Hack -- Reset the counter */
        (*r_ptr).cur_num = 0 as libc::c_int as byte_hack;
        /* Hack -- Reset the max counter */
        (*r_ptr).max_num = 100 as libc::c_int as s16b;
        /* Hack -- Reset the max counter */
        if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
            (*r_ptr).max_num = 1 as libc::c_int as s16b
        }
        if (*r_ptr).flags3 & 0x8000000 as libc::c_int as libc::c_uint != 0 {
            (*r_ptr).max_num = 4 as libc::c_int as s16b
        }
        /* Clear player kills */
        (*r_ptr).r_pkills = 0 as libc::c_int as s16b;
        /* Clear saved flag */
        (*r_ptr).on_saved = 0 as libc::c_int as bool_;
        i += 1
    }
    /* Hack -- Well fed player */
    (*p_ptr).food = (10000 as libc::c_int - 1 as libc::c_int) as s16b;
    /* Wipe the alchemists' recipes */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        alchemist_known_egos[i as usize] = 0 as libc::c_int as u32b;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        alchemist_known_artifacts[i as usize] = 0 as libc::c_int as u32b;
        i += 1
    }
    alchemist_gained = 0 as libc::c_int as u32b;
    /* Clear "cheat" options */
    cheat_peek = 0 as libc::c_int as bool_;
    cheat_hear = 0 as libc::c_int as bool_;
    cheat_room = 0 as libc::c_int as bool_;
    cheat_xtra = 0 as libc::c_int as bool_;
    cheat_know = 0 as libc::c_int as bool_;
    cheat_live = 0 as libc::c_int as bool_;
    /* Assume no winning game */
    total_winner = 0 as libc::c_int as u16b;
    has_won = 0 as libc::c_int as u16b;
    /* Assume no cheating */
    noscore = 0 as libc::c_int as u16b;
    wizard = 0 as libc::c_int as bool_;
    /* Assume no innate spells */
    spell_num = 0 as libc::c_int as s16b;
    /* Clear the fate */
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        fates[i as usize].fate = 0 as libc::c_int as byte_hack;
        i += 1
    }
    (*p_ptr).no_mortal = 0 as libc::c_int as bool_;
    /* Player don't have the black breath from the beginning !*/
    (*p_ptr).black_breath = 0 as libc::c_int as bool_;
    /* Default pet command settings */
    (*p_ptr).pet_follow_distance = 6 as libc::c_int as byte_hack;
    (*p_ptr).pet_open_doors = 0 as libc::c_int as byte_hack;
    (*p_ptr).pet_pickup_items = 0 as libc::c_int as byte_hack;
    /* Body changing initialisation */
    (*p_ptr).body_monster = 0 as libc::c_int as u16b;
    (*p_ptr).disembodied = 0 as libc::c_int as bool_;
    /* Wipe the bounties */
    total_bounties = 0 as libc::c_int as u32b;
    /* Wipe spells */
    (*p_ptr).xtra_spells = 0 as libc::c_int as s16b;
    /* Wipe xtra hp */
    (*p_ptr).hp_mod = 0 as libc::c_int as s16b;
    /* Wipe the monsters */
    wipe_m_list();
    /* Wipe the doppleganger */
    doppleganger = 0 as libc::c_int as s16b;
    /* Wipe the recall depths */
    i = 0 as libc::c_int;
    while i < max_d_idx as libc::c_int {
        *max_dlv.offset(i as isize) = 0 as libc::c_int as s16b;
        i += 1
    }
    /* Wipe the known inscription list */
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        inscription_info[i as usize].know = 0 as libc::c_int as bool_;
        i += 1
    }
    /* Wipe the known traps list */
    i = 0 as libc::c_int;
    while i < max_t_idx as libc::c_int {
        (*t_info.offset(i as isize)).known = 0 as libc::c_int as s16b;
        (*t_info.offset(i as isize)).ident = 0 as libc::c_int as bool_;
        i += 1
    }
    /* Reset wild_mode to FALSE */
    (*p_ptr).wild_mode = 0 as libc::c_int as bool_;
    (*p_ptr).old_wild_mode = 0 as libc::c_int as bool_;
    /* Initialize allow_one_death */
    (*p_ptr).allow_one_death = 0 as libc::c_int as byte_hack;
    (*p_ptr).loan_time = 0 as libc::c_int;
    (*p_ptr).loan = (*p_ptr).loan_time;
    /* Wipe the power list */
    i = 0 as libc::c_int;
    while i < 62 as libc::c_int {
        (*p_ptr).powers_mod[i as usize] = 0 as libc::c_int as bool_;
        i += 1
    }
    /* No companions killed */
    (*p_ptr).companion_killed = 0 as libc::c_int as s16b;
}
/* Create an object */
#[no_mangle]
pub unsafe extern "C" fn outfit_obj(mut tv: libc::c_int, mut sv: libc::c_int,
                                    mut pval: libc::c_int,
                                    mut dd: libc::c_int,
                                    mut ds: libc::c_int) {
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
    /* Get local object */
    q_ptr = &mut forge;
    (*q_ptr).pval = 0 as libc::c_int;
    (*q_ptr).pval2 = 0 as libc::c_int as s16b;
    /* Hack -- Give the player an object */
    object_prep(q_ptr, lookup_kind(tv, sv) as libc::c_int);
    if pval != 0 { (*q_ptr).pval = pval }
    /* These objects are "storebought" */
    (*q_ptr).ident =
        ((*q_ptr).ident as libc::c_int | 0x20 as libc::c_int) as byte_hack;
    (*q_ptr).number = damroll(dd as s16b, ds as s16b) as byte_hack;
    object_aware(q_ptr);
    object_known(q_ptr);
    inven_carry(q_ptr, 0 as libc::c_int as bool_);
}
/*
 * Init players with some belongings
 *
 * Having an item makes the player "aware" of its purpose.
 */
unsafe extern "C" fn player_outfit() {
    let mut i: libc::c_int = 0;
    /*
	 * Get an adventurer guide describing a bit of the
	 * wilderness.
	 */
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
    let mut q_ptr: *mut object_type = &mut forge;
    /* Hack -- Give the player an adventurer guide */
    object_prep(q_ptr,
                lookup_kind(8 as libc::c_int, 20 as libc::c_int) as
                    libc::c_int);
    (*q_ptr).number = 1 as libc::c_int as byte_hack;
    object_aware(q_ptr);
    object_known(q_ptr);
    inven_carry(q_ptr, 0 as libc::c_int as bool_);
    process_hooks(35 as libc::c_int,
                  b"()\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char);
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
    let mut q_ptr_0: *mut object_type = &mut forge_0;
    /* Hack -- Give the player some food */
    object_prep(q_ptr_0,
                lookup_kind(80 as libc::c_int, 35 as libc::c_int) as
                    libc::c_int);
    (*q_ptr_0).number =
        (3 as libc::c_int +
             Rand_div(1 as libc::c_int + 7 as libc::c_int - 3 as libc::c_int))
            as byte_hack;
    object_aware(q_ptr_0);
    object_known(q_ptr_0);
    inven_carry(q_ptr_0, 0 as libc::c_int as bool_);
    let mut forge_1: object_type =
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
    let mut q_ptr_1: *mut object_type = &mut forge_1;
    /* Hack -- Give the player some torches */
    object_prep(q_ptr_1,
                lookup_kind(39 as libc::c_int, 0 as libc::c_int) as
                    libc::c_int);
    (*q_ptr_1).number =
        (3 as libc::c_int +
             Rand_div(1 as libc::c_int + 7 as libc::c_int - 3 as libc::c_int))
            as byte_hack;
    (*q_ptr_1).timeout =
        ((3 as libc::c_int +
              Rand_div(1 as libc::c_int + 7 as libc::c_int -
                           3 as libc::c_int)) * 500 as libc::c_int) as s16b;
    object_aware(q_ptr_1);
    object_known(q_ptr_1);
    inven_carry(q_ptr_1, 0 as libc::c_int as bool_);
    /* Rogues have a better knowledge of traps */
    if has_ability(9 as libc::c_int) != 0 {
        (*t_info.offset(118 as libc::c_int as isize)).known =
            (Rand_div(50 as libc::c_int) + 1 as libc::c_int +
                 50 as libc::c_int) as s16b;
        (*t_info.offset(23 as libc::c_int as isize)).known =
            (Rand_div(50 as libc::c_int) + 1 as libc::c_int +
                 50 as libc::c_int) as s16b;
        (*t_info.offset(64 as libc::c_int as isize)).known =
            (Rand_div(50 as libc::c_int) + 1 as libc::c_int +
                 50 as libc::c_int) as s16b;
        (*t_info.offset(118 as libc::c_int as isize)).ident =
            1 as libc::c_int as bool_;
        (*t_info.offset(23 as libc::c_int as isize)).ident =
            1 as libc::c_int as bool_;
        (*t_info.offset(64 as libc::c_int as isize)).ident =
            1 as libc::c_int as bool_;
        /* Hack -- Give the player a some ammo for the traps */
        let mut forge_2: object_type =
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
        let mut q_ptr_2: *mut object_type = &mut forge_2;
        object_prep(q_ptr_2,
                    lookup_kind(16 as libc::c_int, 1 as libc::c_int) as
                        libc::c_int);
        (*q_ptr_2).number =
            (5 as libc::c_int +
                 Rand_div(1 as libc::c_int + 15 as libc::c_int -
                              5 as libc::c_int)) as byte_hack;
        object_aware(q_ptr_2);
        object_known(q_ptr_2);
        /* These objects are "storebought" */
        (*q_ptr_2).ident =
            ((*q_ptr_2).ident as libc::c_int | 0x20 as libc::c_int) as
                byte_hack;
        inven_carry(q_ptr_2, 0 as libc::c_int as bool_);
    }
    /* Hack -- Give the player some useful objects */
    i = 0 as libc::c_int;
    while i < (*rp_ptr).obj_num as libc::c_int {
        outfit_obj((*rp_ptr).obj_tval[i as usize] as libc::c_int,
                   (*rp_ptr).obj_sval[i as usize] as libc::c_int,
                   (*rp_ptr).obj_pval[i as usize] as libc::c_int,
                   (*rp_ptr).obj_dd[i as usize] as libc::c_int,
                   (*rp_ptr).obj_ds[i as usize] as libc::c_int);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*rmp_ptr).obj_num as libc::c_int {
        outfit_obj((*rmp_ptr).obj_tval[i as usize] as libc::c_int,
                   (*rmp_ptr).obj_sval[i as usize] as libc::c_int,
                   (*rmp_ptr).obj_pval[i as usize] as libc::c_int,
                   (*rmp_ptr).obj_dd[i as usize] as libc::c_int,
                   (*rmp_ptr).obj_ds[i as usize] as libc::c_int);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*cp_ptr).obj_num as libc::c_int {
        outfit_obj((*cp_ptr).obj_tval[i as usize] as libc::c_int,
                   (*cp_ptr).obj_sval[i as usize] as libc::c_int,
                   (*cp_ptr).obj_pval[i as usize] as libc::c_int,
                   (*cp_ptr).obj_dd[i as usize] as libc::c_int,
                   (*cp_ptr).obj_ds[i as usize] as libc::c_int);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*cp_ptr).spec[(*p_ptr).pspec as usize].obj_num as libc::c_int {
        outfit_obj((*cp_ptr).spec[(*p_ptr).pspec as
                                      usize].obj_tval[i as usize] as
                       libc::c_int,
                   (*cp_ptr).spec[(*p_ptr).pspec as
                                      usize].obj_sval[i as usize] as
                       libc::c_int,
                   (*cp_ptr).spec[(*p_ptr).pspec as
                                      usize].obj_pval[i as usize] as
                       libc::c_int,
                   (*cp_ptr).spec[(*p_ptr).pspec as usize].obj_dd[i as usize]
                       as libc::c_int,
                   (*cp_ptr).spec[(*p_ptr).pspec as usize].obj_ds[i as usize]
                       as libc::c_int);
        i += 1
    };
}
/* Possible number(and layout) or random quests */
#[no_mangle]
pub static mut random_quests_types: [libc::c_int; 32] =
    [1 as libc::c_int, 5 as libc::c_int, 6 as libc::c_int, 7 as libc::c_int,
     10 as libc::c_int, 11 as libc::c_int, 12 as libc::c_int,
     14 as libc::c_int, 1 as libc::c_int, 5 as libc::c_int, 6 as libc::c_int,
     7 as libc::c_int, 10 as libc::c_int, 11 as libc::c_int,
     12 as libc::c_int, 14 as libc::c_int, 1 as libc::c_int, 5 as libc::c_int,
     6 as libc::c_int, 7 as libc::c_int, 10 as libc::c_int, 11 as libc::c_int,
     12 as libc::c_int, 14 as libc::c_int, 20 as libc::c_int,
     13 as libc::c_int, 15 as libc::c_int, 16 as libc::c_int,
     9 as libc::c_int, 17 as libc::c_int, 18 as libc::c_int,
     8 as libc::c_int];
unsafe extern "C" fn gen_random_quests(mut n: libc::c_int) {
    let mut step: libc::c_int = 0;
    let mut lvl: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut old_type: libc::c_int = dungeon_type as libc::c_int;
    /* Factor dlev value by 1000 to keep precision */
    step = 98 as libc::c_int * 1000 as libc::c_int / n;
    lvl = step / 2 as libc::c_int;
    (*quest.offset(5 as libc::c_int as isize)).status =
        1 as libc::c_int as s16b;
    i = 0 as libc::c_int;
    while i < n {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset(2 as libc::c_int as isize) as
                *mut monster_race;
        let mut rl: libc::c_int =
            lvl / 1000 as libc::c_int + 1 as libc::c_int;
        let mut min_level: libc::c_int = 0;
        let mut tries: libc::c_int = 5000 as libc::c_int;
        let mut q_ptr: *mut random_quest =
            &mut *random_quests.as_mut_ptr().offset(rl as isize) as
                *mut random_quest;
        let mut j: libc::c_int = 0;
        /* Find the appropriate dungeon */
        j = 0 as libc::c_int;
        while j < max_d_idx as libc::c_int {
            let mut d_ptr: *mut dungeon_info_type =
                &mut *d_info.offset(j as isize) as *mut dungeon_info_type;
            if !((*d_ptr).flags1 as libc::c_long & 0x1 as libc::c_long == 0) {
                if (*d_ptr).mindepth as libc::c_int <= rl &&
                       rl <= (*d_ptr).maxdepth as libc::c_int {
                    dungeon_type = j as byte_hack;
                    break ;
                }
            }
            j += 1
        }
        (*q_ptr).type_0 =
            random_quests_types[Rand_div(8 as libc::c_int * 3 as libc::c_int +
                                             8 as libc::c_int *
                                                 1 as libc::c_int) as usize]
                as byte_hack;
        /* XXX XXX XXX Try until valid choice is found */
        while tries != 0 {
            let mut ok: bool_ = 0;
            tries -= 1;
            /* Random monster 5 - 10 levels out of depth */
            (*q_ptr).r_idx =
                get_mon_num(rl + 4 as libc::c_int +
                                (Rand_div(6 as libc::c_int) +
                                     1 as libc::c_int));
            if (*q_ptr).r_idx == 0 { continue ; }
            r_ptr =
                &mut *r_info.offset((*q_ptr).r_idx as isize) as
                    *mut monster_race;
            /* Accept only monsters that can be generated */
            if (*r_ptr).flags9 & 0x2000 as libc::c_int as libc::c_uint != 0 {
                continue ;
            }
            if (*r_ptr).flags9 & 0x4000 as libc::c_int as libc::c_uint != 0 {
                continue ;
            }
            /* Accept only monsters that are not breeders */
            if (*r_ptr).flags4 & 0x2 as libc::c_int as libc::c_uint != 0 {
                continue ;
            }
            /* Forbid joke monsters */
            if (*r_ptr).flags8 & 0x8000 as libc::c_int as libc::c_uint != 0 {
                continue ;
            }
            /* Accept only monsters that are not friends */
            if (*r_ptr).flags7 & 0x10 as libc::c_int as libc::c_uint != 0 {
                continue ;
            }
            /* Refuse nazguls */
            if (*r_ptr).flags7 & 0x80 as libc::c_int as libc::c_uint != 0 {
                continue ;
            }
            /* Accept only monsters that are not good */
            if (*r_ptr).flags3 & 0x200 as libc::c_int as libc::c_uint != 0 {
                continue ;
            }
            /* Assume no explosion attacks */
            ok = 1 as libc::c_int as bool_;
            /* Reject monsters with exploding attacks */
            k = 0 as libc::c_int;
            while k < 4 as libc::c_int {
                if (*r_ptr).blow[k as usize].method as libc::c_int ==
                       16 as libc::c_int {
                    ok = 0 as libc::c_int as bool_
                }
                k += 1
            }
            if ok == 0 { continue ; }
            /* No mutliple uniques */
            if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 &&
                   ((*q_ptr).type_0 as libc::c_int != 1 as libc::c_int ||
                        (*r_ptr).max_num as libc::c_int ==
                            -(1 as libc::c_int)) {
                continue ;
            }
            /* No single non uniques */
            if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint == 0 &&
                   (*q_ptr).type_0 as libc::c_int == 1 as libc::c_int {
                continue ;
            }
            /* Level restriction */
            min_level =
                if rl > 49 as libc::c_int { 49 as libc::c_int } else { rl };
            /* Accept monsters matching the level restriction */
            if (*r_ptr).level as libc::c_int > min_level { break ; }
        }
        /* Arg could not find anything ??? */
        if tries == 0 {
            if wizard != 0 {
                message_add(1 as libc::c_int as byte_hack,
                            format(b"Could not find quest monster on lvl %d\x00"
                                       as *const u8 as *const libc::c_char,
                                   rl) as cptr,
                            4 as libc::c_int as byte_hack);
            }
            (*q_ptr).type_0 = 0 as libc::c_int as byte_hack
        } else {
            if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
                (*r_ptr).max_num = -(1 as libc::c_int) as s16b
            }
            (*q_ptr).done = 0 as libc::c_int as bool_;
            if wizard != 0 {
                message_add(1 as libc::c_int as byte_hack,
                            format(b"Quest for %d on lvl %d\x00" as *const u8
                                       as *const libc::c_char,
                                   (*q_ptr).r_idx as libc::c_int, rl) as cptr,
                            4 as libc::c_int as byte_hack);
            }
        }
        lvl += step;
        i += 1
    }
    dungeon_type = old_type as byte_hack;
}
#[no_mangle]
pub unsafe extern "C" fn dump_classes(mut classes: *mut s16b,
                                      mut sel: libc::c_int,
                                      mut restrictions: *mut u32b)
 -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut desc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str: cptr = 0 as *const libc::c_char;
    desc =
        memset(ralloc(((*c_head).text_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*c_head).text_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* Clean up */
    clear_from(12 as libc::c_int);
    while *classes.offset(n as isize) as libc::c_int != -(1 as libc::c_int) {
        let mut mod_0: cptr = b"\x00" as *const u8 as *const libc::c_char;
        let mut p2: libc::c_char = ')' as i32 as libc::c_char;
        let mut p1: libc::c_char = ' ' as i32 as libc::c_char;
        /* Analyze */
        (*p_ptr).pclass = *classes.offset(n as isize) as byte_hack;
        cp_ptr =
            &mut *class_info.offset((*p_ptr).pclass as isize) as
                *mut player_class;
        str = c_name.offset((*cp_ptr).title as isize) as cptr;
        if sel == n {
            p1 = '[' as i32 as libc::c_char;
            p2 = ']' as i32 as libc::c_char
        }
        /* Display */
        strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"%c%c%c %s%s\x00" as *const u8 as *const libc::c_char,
                p1 as libc::c_int,
                if n <= 25 as libc::c_int {
                    (n) + 'a' as i32
                } else { (n - 26 as libc::c_int) + '0' as i32 },
                p2 as libc::c_int, str, mod_0);
        /* Print some more info */
        if sel == n {
            strnfmt(desc, (*c_head).text_size,
                    b"%s%s\x00" as *const u8 as *const libc::c_char,
                    c_text.offset((*cp_ptr).desc as isize),
                    if (*cp_ptr).flags1 as libc::c_long & 0x1 as libc::c_long
                           != 0 {
                        b"\nEXPERIMENTAL\x00" as *const u8 as
                            *const libc::c_char
                    } else { b"\x00" as *const u8 as *const libc::c_char });
            print_desc(desc as cptr);
            if *restrictions.offset((*classes.offset(n as isize) as
                                         libc::c_int / 32 as libc::c_int) as
                                        isize) as libc::c_long &
                   (1 as libc::c_long) <<
                       *classes.offset(n as isize) as libc::c_int == 0 ||
                   (*cp_ptr).flags1 as libc::c_long & 0x1 as libc::c_long != 0
               {
                c_put_str(6 as libc::c_int as byte_hack,
                          buf.as_mut_ptr() as cptr,
                          18 as libc::c_int + n / 4 as libc::c_int,
                          1 as libc::c_int +
                              20 as libc::c_int * (n % 4 as libc::c_int));
            } else {
                c_put_str(14 as libc::c_int as byte_hack,
                          buf.as_mut_ptr() as cptr,
                          18 as libc::c_int + n / 4 as libc::c_int,
                          1 as libc::c_int +
                              20 as libc::c_int * (n % 4 as libc::c_int));
            }
        } else if *restrictions.offset((*classes.offset(n as isize) as
                                            libc::c_int / 32 as libc::c_int)
                                           as isize) as libc::c_long &
                      (1 as libc::c_long) <<
                          *classes.offset(n as isize) as libc::c_int == 0 ||
                      (*cp_ptr).flags1 as libc::c_long & 0x1 as libc::c_long
                          != 0 {
            c_put_str(2 as libc::c_int as byte_hack, buf.as_mut_ptr() as cptr,
                      18 as libc::c_int + n / 4 as libc::c_int,
                      1 as libc::c_int +
                          20 as libc::c_int * (n % 4 as libc::c_int));
        } else {
            put_str(buf.as_mut_ptr() as cptr,
                    18 as libc::c_int + n / 4 as libc::c_int,
                    1 as libc::c_int +
                        20 as libc::c_int * (n % 4 as libc::c_int));
        }
        n += 1
    }
    rnfree(desc as vptr,
           ((*c_head).text_size as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                as libc::c_ulong));
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn dump_specs(mut sel: libc::c_int) -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut desc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str: cptr = 0 as *const libc::c_char;
    desc =
        memset(ralloc(((*c_head).text_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*c_head).text_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* Clean up */
    clear_from(12 as libc::c_int);
    n = 0 as libc::c_int;
    while n < 20 as libc::c_int {
        let mut p2: libc::c_char = ')' as i32 as libc::c_char;
        let mut p1: libc::c_char = ' ' as i32 as libc::c_char;
        /* Found the last one ? */
        if (*class_info.offset((*p_ptr).pclass as
                                   isize)).spec[n as usize].title == 0 {
            break ;
        }
        /* Analyze */
        (*p_ptr).pspec = n as byte_hack;
        spp_ptr =
            &mut *(*class_info.offset((*p_ptr).pclass as
                                          isize)).spec.as_mut_ptr().offset((*p_ptr).pspec
                                                                               as
                                                                               isize)
                as *mut player_spec;
        str = c_name.offset((*spp_ptr).title as isize) as cptr;
        if sel == n {
            p1 = '[' as i32 as libc::c_char;
            p2 = ']' as i32 as libc::c_char
        }
        /* Display */
        strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"%c%c%c %s\x00" as *const u8 as *const libc::c_char,
                p1 as libc::c_int, n + 'a' as i32, p2 as libc::c_int, str);
        /* Print some more info */
        if sel == n {
            strnfmt(desc, (*c_head).text_size,
                    b"%s%s\x00" as *const u8 as *const libc::c_char,
                    c_text.offset((*spp_ptr).desc as isize),
                    if (*spp_ptr).flags1 as libc::c_long & 0x1 as libc::c_long
                           != 0 {
                        b"\nEXPERIMENTAL\x00" as *const u8 as
                            *const libc::c_char
                    } else { b"\x00" as *const u8 as *const libc::c_char });
            print_desc(desc as cptr);
            if (*spp_ptr).flags1 as libc::c_long & 0x1 as libc::c_long != 0 {
                c_put_str(6 as libc::c_int as byte_hack,
                          buf.as_mut_ptr() as cptr,
                          18 as libc::c_int + n / 4 as libc::c_int,
                          1 as libc::c_int +
                              20 as libc::c_int * (n % 4 as libc::c_int));
            } else {
                c_put_str(14 as libc::c_int as byte_hack,
                          buf.as_mut_ptr() as cptr,
                          18 as libc::c_int + n / 4 as libc::c_int,
                          1 as libc::c_int +
                              20 as libc::c_int * (n % 4 as libc::c_int));
            }
        } else if (*spp_ptr).flags1 as libc::c_long & 0x1 as libc::c_long != 0
         {
            c_put_str(2 as libc::c_int as byte_hack, buf.as_mut_ptr() as cptr,
                      18 as libc::c_int + n / 4 as libc::c_int,
                      1 as libc::c_int +
                          20 as libc::c_int * (n % 4 as libc::c_int));
        } else {
            put_str(buf.as_mut_ptr() as cptr,
                    18 as libc::c_int + n / 4 as libc::c_int,
                    1 as libc::c_int +
                        20 as libc::c_int * (n % 4 as libc::c_int));
        }
        n += 1
    }
    rnfree(desc as vptr,
           ((*c_head).text_size as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                as libc::c_ulong));
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn dump_races(mut sel: libc::c_int) -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut desc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str: cptr = 0 as *const libc::c_char;
    desc =
        memset(ralloc(((*rp_head).text_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*rp_head).text_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* Clean up */
    clear_from(12 as libc::c_int);
    n = 0 as libc::c_int;
    while n < max_rp_idx as libc::c_int {
        let mut p2: libc::c_char = ')' as i32 as libc::c_char;
        let mut p1: libc::c_char = ' ' as i32 as libc::c_char;
        /* Analyze */
        (*p_ptr).prace = n as byte_hack;
        rp_ptr =
            &mut *race_info.offset((*p_ptr).prace as isize) as
                *mut player_race;
        str = rp_name.offset((*rp_ptr).title as isize) as cptr;
        if sel == n {
            p1 = '[' as i32 as libc::c_char;
            p2 = ']' as i32 as libc::c_char
        }
        /* Display */
        strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"%c%c%c %s\x00" as *const u8 as *const libc::c_char,
                p1 as libc::c_int, n + 'a' as i32, p2 as libc::c_int, str);
        /* Print some more info */
        if sel == n {
            strnfmt(desc, (*rp_head).text_size,
                    b"%s%s\x00" as *const u8 as *const libc::c_char,
                    rp_text.offset((*rp_ptr).desc as isize),
                    if (*rp_ptr).flags1 as libc::c_long & 0x1 as libc::c_long
                           != 0 {
                        b"\nEXPERIMENTAL\x00" as *const u8 as
                            *const libc::c_char
                    } else { b"\x00" as *const u8 as *const libc::c_char });
            print_desc(desc as cptr);
            if (*rp_ptr).flags1 as libc::c_long & 0x1 as libc::c_long != 0 {
                c_put_str(6 as libc::c_int as byte_hack,
                          buf.as_mut_ptr() as cptr,
                          18 as libc::c_int + n / 5 as libc::c_int,
                          1 as libc::c_int +
                              15 as libc::c_int * (n % 5 as libc::c_int));
            } else {
                c_put_str(14 as libc::c_int as byte_hack,
                          buf.as_mut_ptr() as cptr,
                          18 as libc::c_int + n / 5 as libc::c_int,
                          1 as libc::c_int +
                              15 as libc::c_int * (n % 5 as libc::c_int));
            }
        } else if (*rp_ptr).flags1 as libc::c_long & 0x1 as libc::c_long != 0
         {
            c_put_str(2 as libc::c_int as byte_hack, buf.as_mut_ptr() as cptr,
                      18 as libc::c_int + n / 5 as libc::c_int,
                      1 as libc::c_int +
                          15 as libc::c_int * (n % 5 as libc::c_int));
        } else {
            put_str(buf.as_mut_ptr() as cptr,
                    18 as libc::c_int + n / 5 as libc::c_int,
                    1 as libc::c_int +
                        15 as libc::c_int * (n % 5 as libc::c_int));
        }
        n += 1
    }
    rnfree(desc as vptr,
           ((*rp_head).text_size as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                as libc::c_ulong));
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn dump_rmods(mut sel: libc::c_int,
                                    mut racem: *mut libc::c_int,
                                    mut max: libc::c_int) -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut desc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str: cptr = 0 as *const libc::c_char;
    desc =
        memset(ralloc(((*rmp_head).text_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*rmp_head).text_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* Clean up */
    clear_from(12 as libc::c_int);
    /* Dump races */
    n = 0 as libc::c_int;
    while n < max {
        let mut p2: libc::c_char = ')' as i32 as libc::c_char;
        let mut p1: libc::c_char = ' ' as i32 as libc::c_char;
        /* Analyze */
        (*p_ptr).pracem = *racem.offset(n as isize) as byte_hack;
        rmp_ptr =
            &mut *race_mod_info.offset((*p_ptr).pracem as isize) as
                *mut player_race_mod;
        str = rmp_name.offset((*rmp_ptr).title as isize) as cptr;
        if sel == n {
            p1 = '[' as i32 as libc::c_char;
            p2 = ']' as i32 as libc::c_char
        }
        /* Display */
        if *racem.offset(n as isize) != 0 {
            strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                    b"%c%c%c %s\x00" as *const u8 as *const libc::c_char,
                    p1 as libc::c_int, n + 'a' as i32, p2 as libc::c_int,
                    str);
        } else {
            strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                    b"%c%c%c Classical\x00" as *const u8 as
                        *const libc::c_char, p1 as libc::c_int,
                    n + 'a' as i32, p2 as libc::c_int);
        }
        /* Print some more info */
        if sel == n {
            strnfmt(desc, (*rmp_head).text_size,
                    b"%s%s\x00" as *const u8 as *const libc::c_char,
                    rmp_text.offset((*rmp_ptr).desc as isize),
                    if (*rmp_ptr).flags1 as libc::c_long & 0x1 as libc::c_long
                           != 0 {
                        b"\nEXPERIMENTAL\x00" as *const u8 as
                            *const libc::c_char
                    } else { b"\x00" as *const u8 as *const libc::c_char });
            print_desc(desc as cptr);
            if (*rmp_ptr).flags1 as libc::c_long & 0x1 as libc::c_long != 0 {
                c_put_str(6 as libc::c_int as byte_hack,
                          buf.as_mut_ptr() as cptr,
                          18 as libc::c_int + n / 5 as libc::c_int,
                          1 as libc::c_int +
                              15 as libc::c_int * (n % 5 as libc::c_int));
            } else {
                c_put_str(14 as libc::c_int as byte_hack,
                          buf.as_mut_ptr() as cptr,
                          18 as libc::c_int + n / 5 as libc::c_int,
                          1 as libc::c_int +
                              15 as libc::c_int * (n % 5 as libc::c_int));
            }
        } else if (*rmp_ptr).flags1 as libc::c_long & 0x1 as libc::c_long != 0
         {
            c_put_str(2 as libc::c_int as byte_hack, buf.as_mut_ptr() as cptr,
                      18 as libc::c_int + n / 5 as libc::c_int,
                      1 as libc::c_int +
                          15 as libc::c_int * (n % 5 as libc::c_int));
        } else {
            put_str(buf.as_mut_ptr() as cptr,
                    18 as libc::c_int + n / 5 as libc::c_int,
                    1 as libc::c_int +
                        15 as libc::c_int * (n % 5 as libc::c_int));
        }
        n += 1
    }
    rnfree(desc as vptr,
           ((*rmp_head).text_size as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                as libc::c_ulong));
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn dump_gods(mut sel: libc::c_int,
                                   mut choice: *mut libc::c_int,
                                   mut max: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut str: cptr = 0 as *const libc::c_char;
    /* Clean up */
    clear_from(12 as libc::c_int);
    Term_putstr(5 as libc::c_int, 17 as libc::c_int, -(1 as libc::c_int),
                1 as libc::c_int as byte_hack,
                b"You can choose to worship a god, some class must start with a god.\x00"
                    as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < max {
        let mut p2: libc::c_char = ')' as i32 as libc::c_char;
        let mut p1: libc::c_char = ' ' as i32 as libc::c_char;
        let mut n: libc::c_int = *choice.offset(i as isize);
        let mut g_ptr: *mut deity_type =
            &mut *deity_info.offset(0 as libc::c_int as isize) as
                *mut deity_type;
        if n == 0 {
            str = b"No God\x00" as *const u8 as *const libc::c_char
        } else {
            g_ptr = &mut *deity_info.offset(n as isize) as *mut deity_type;
            str = (*g_ptr).name
        }
        if sel == i {
            p1 = '[' as i32 as libc::c_char;
            p2 = ']' as i32 as libc::c_char
        }
        /* Display */
        strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"%c%c%c %s\x00" as *const u8 as *const libc::c_char,
                p1 as libc::c_int, i + 'a' as i32, p2 as libc::c_int, str);
        /* Print some more info */
        if sel == i {
            if n != 0 {
                /* Display the first four lines of the god description */
                j = 0 as libc::c_int;
                while j < 4 as libc::c_int {
                    if strcmp((*g_ptr).desc[j as usize].as_mut_ptr(),
                              b"\x00" as *const u8 as *const libc::c_char) !=
                           0 {
                        print_desc_aux((*g_ptr).desc[j as usize].as_mut_ptr()
                                           as cptr, 12 as libc::c_int + j,
                                       1 as libc::c_int);
                    }
                    j += 1
                }
            } else {
                print_desc(b"You can begin as an atheist and still convert to a god later.\x00"
                               as *const u8 as *const libc::c_char);
            }
            c_put_str(14 as libc::c_int as byte_hack,
                      buf.as_mut_ptr() as cptr,
                      20 as libc::c_int + i / 4 as libc::c_int,
                      1 as libc::c_int +
                          20 as libc::c_int * (i % 4 as libc::c_int));
        } else {
            put_str(buf.as_mut_ptr() as cptr,
                    20 as libc::c_int + i / 4 as libc::c_int,
                    1 as libc::c_int +
                        20 as libc::c_int * (i % 4 as libc::c_int));
        }
        i += 1
    }
    return max;
}
/* Ask questions */
static mut do_quick_start: bool_ = 0 as libc::c_int as bool_;
unsafe extern "C" fn player_birth_aux_ask() -> bool_ {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut sel: libc::c_int = 0;
    let mut tmp: s32b = 0;
    let mut racem: [libc::c_int; 100] = [0; 100];
    let mut max_racem: libc::c_int = 0 as libc::c_int;
    let mut restrictions: [u32b; 2] = [0; 2];
    let mut str: cptr = 0 as *const libc::c_char;
    let mut c: libc::c_char = 0;
    let mut p2: libc::c_char = ')' as i32 as libc::c_char;
    let mut buf: [libc::c_char; 200] = [0; 200];
    let mut inp: [libc::c_char; 200] = [0; 200];
    let mut class_types: *mut s16b = 0 as *mut s16b;
    let mut allow_quest: s32b = 0;
    /* ** Intro ***/
    /* Clear screen */
    Term_clear();
    /* Title everything */
    put_str(b"Name  :\x00" as *const u8 as *const libc::c_char,
            2 as libc::c_int, 1 as libc::c_int);
    put_str(b"Sex   :\x00" as *const u8 as *const libc::c_char,
            3 as libc::c_int, 1 as libc::c_int);
    put_str(b"Race  :\x00" as *const u8 as *const libc::c_char,
            4 as libc::c_int, 1 as libc::c_int);
    put_str(b"Class :\x00" as *const u8 as *const libc::c_char,
            5 as libc::c_int, 1 as libc::c_int);
    /* Dump the default name */
    c_put_str(14 as libc::c_int as byte_hack,
              player_name.as_mut_ptr() as cptr, 2 as libc::c_int,
              9 as libc::c_int);
    /* ** Instructions ***/
    /* Display some helpful information */
    Term_putstr(5 as libc::c_int, 8 as libc::c_int, -(1 as libc::c_int),
                1 as libc::c_int as byte_hack,
                b"Please answer the following questions.  Most of the questions\x00"
                    as *const u8 as *const libc::c_char);
    Term_putstr(5 as libc::c_int, 9 as libc::c_int, -(1 as libc::c_int),
                1 as libc::c_int as byte_hack,
                b"display a set of standard answers, and many will also accept\x00"
                    as *const u8 as *const libc::c_char);
    Term_putstr(5 as libc::c_int, 10 as libc::c_int, -(1 as libc::c_int),
                1 as libc::c_int as byte_hack,
                b"some special responses, including \'Q\' to quit, \'S\' to restart,\x00"
                    as *const u8 as *const libc::c_char);
    Term_putstr(5 as libc::c_int, 11 as libc::c_int, -(1 as libc::c_int),
                1 as libc::c_int as byte_hack,
                b"and \'?\' for help.  Note that \'Q\' and \'S\' must be capitalized.\x00"
                    as *const u8 as *const libc::c_char);
    /* ** Quick Start ***/
    if previous_char.quick_ok != 0 {
        /* Extra info */
        Term_putstr(1 as libc::c_int, 15 as libc::c_int, -(1 as libc::c_int),
                    1 as libc::c_int as byte_hack,
                    b"Do you want to use the quick start function(same character as your last one).\x00"
                        as *const u8 as *const libc::c_char);
        loop 
             /* Choose */
             {
            put_str(b"Use quick start (y/n)?\x00" as *const u8 as
                        *const libc::c_char, 20 as libc::c_int,
                    2 as libc::c_int);
            c = inkey();
            if c as libc::c_int == 'Q' as i32 {
                quit(0 as cptr);
            } else if c as libc::c_int == 'S' as i32 {
                return 0 as libc::c_int as bool_
            } else if c as libc::c_int == 'y' as i32 ||
                          c as libc::c_int == 'Y' as i32 {
                do_quick_start = 1 as libc::c_int as bool_;
                break ;
            } else { do_quick_start = 0 as libc::c_int as bool_; break ; }
        }
    }
    /* Clean up */
    clear_from(15 as libc::c_int);
    /* ** Player sex ***/
    if do_quick_start != 0 {
        k = previous_char.sex as libc::c_int
    } else {
        /* Extra info */
        Term_putstr(5 as libc::c_int, 15 as libc::c_int, -(1 as libc::c_int),
                    1 as libc::c_int as byte_hack,
                    b"Your \'sex\' does not have any significant gameplay effects.\x00"
                        as *const u8 as *const libc::c_char);
        /* Prompt for "Sex" */
        n = 0 as libc::c_int;
        while n < 3 as libc::c_int {
            /* Analyze */
            (*p_ptr).psex = n as byte_hack;
            sp_ptr =
                &mut *sex_info.as_mut_ptr().offset((*p_ptr).psex as isize) as
                    *mut player_sex;
            str = (*sp_ptr).title;
            /* Display */
            strnfmt(buf.as_mut_ptr(), 200 as libc::c_int as uint_hack,
                    b"%c%c %s\x00" as *const u8 as *const libc::c_char,
                    n + 'a' as i32, p2 as libc::c_int, str);
            put_str(buf.as_mut_ptr() as cptr,
                    21 as libc::c_int + n / 5 as libc::c_int,
                    2 as libc::c_int +
                        15 as libc::c_int * (n % 5 as libc::c_int));
            n += 1
        }
        loop 
             /* Choose */
             {
            strnfmt(buf.as_mut_ptr(), 200 as libc::c_int as uint_hack,
                    b"Choose a sex (%c-%c), * for random, = for options: \x00"
                        as *const u8 as *const libc::c_char,
                    0 as libc::c_int + 'a' as i32,
                    n - 1 as libc::c_int + 'a' as i32);
            put_str(buf.as_mut_ptr() as cptr, 20 as libc::c_int,
                    2 as libc::c_int);
            c = inkey();
            if c as libc::c_int == 'Q' as i32 { quit(0 as cptr); }
            if c as libc::c_int == 'S' as i32 {
                return 0 as libc::c_int as bool_
            }
            if c as libc::c_int == '*' as i32 {
                k = Rand_div(3 as libc::c_int);
                break ;
            } else {
                k =
                    if *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                           as libc::c_int &
                           _ISlower as libc::c_int as libc::c_ushort as
                               libc::c_int != 0 {
                        (c as libc::c_int) - 'a' as i32
                    } else { -(1 as libc::c_int) };
                if k >= 0 as libc::c_int && k < n { break ; }
                if c as libc::c_int == '?' as i32 {
                    do_cmd_help();
                } else if c as libc::c_int == '=' as i32 {
                    screen_save();
                    do_cmd_options_aux(6 as libc::c_int,
                                       b"Startup Options\x00" as *const u8 as
                                           *const libc::c_char,
                                       0 as libc::c_int as bool_);
                    screen_load();
                } else { bell(); }
            }
        }
    }
    /* Set sex */
    (*p_ptr).psex = k as byte_hack;
    sp_ptr =
        &mut *sex_info.as_mut_ptr().offset((*p_ptr).psex as isize) as
            *mut player_sex;
    str = (*sp_ptr).title;
    /* Display */
    c_put_str(14 as libc::c_int as byte_hack, str, 3 as libc::c_int,
              9 as libc::c_int);
    /* Clean up */
    clear_from(15 as libc::c_int);
    /* ** Player race ***/
    if do_quick_start != 0 {
        k = previous_char.race as libc::c_int
    } else if max_rp_idx as libc::c_int == 1 as libc::c_int {
        k = 0 as libc::c_int
    } else {
        /* Only one choice = instant choice */
        /* Extra info */
        Term_putstr(5 as libc::c_int, 16 as libc::c_int, -(1 as libc::c_int),
                    1 as libc::c_int as byte_hack,
                    b"Your \'race\' determines various intrinsic factors and bonuses.\x00"
                        as *const u8 as *const libc::c_char);
        hack_corruption = 0 as libc::c_int as bool_;
        /* Dump races */
        sel = 0 as libc::c_int;
        n = dump_races(sel);
        loop 
             /* Choose */
             {
            strnfmt(buf.as_mut_ptr(), 200 as libc::c_int as uint_hack,
                    b"Choose a race (%c-%c), * for a random choice, = for options, 8/2/4/6 for movement: \x00"
                        as *const u8 as *const libc::c_char,
                    0 as libc::c_int + 'a' as i32,
                    max_rp_idx as libc::c_int - 1 as libc::c_int +
                        'a' as i32);
            put_str(buf.as_mut_ptr() as cptr, 17 as libc::c_int,
                    2 as libc::c_int);
            c = inkey();
            if c as libc::c_int == 'Q' as i32 { quit(0 as cptr); }
            if c as libc::c_int == 'S' as i32 {
                return 0 as libc::c_int as bool_
            }
            if c as libc::c_int == '*' as i32 {
                k = Rand_div(max_rp_idx as s32b);
                break ;
            } else {
                k =
                    if *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                           as libc::c_int &
                           _ISlower as libc::c_int as libc::c_ushort as
                               libc::c_int != 0 {
                        (c as libc::c_int) - 'a' as i32
                    } else { -(1 as libc::c_int) };
                if k >= 0 as libc::c_int && k < n { break ; }
                if c as libc::c_int == '?' as i32 {
                    exec_lua(format(b"ingame_help(\'select_context\', \'race\', \'%s\')\x00"
                                        as *const u8 as *const libc::c_char,
                                    rp_name.offset((*race_info.offset(sel as
                                                                          isize)).title
                                                       as isize)));
                } else if c as libc::c_int == '=' as i32 {
                    screen_save();
                    do_cmd_options_aux(6 as libc::c_int,
                                       b"Startup Options\x00" as *const u8 as
                                           *const libc::c_char,
                                       0 as libc::c_int as bool_);
                    screen_load();
                } else if c as libc::c_int == '2' as i32 {
                    sel += 5 as libc::c_int;
                    if sel >= n { sel %= 5 as libc::c_int }
                    dump_races(sel);
                } else if c as libc::c_int == '8' as i32 {
                    sel -= 5 as libc::c_int;
                    if sel < 0 as libc::c_int {
                        sel = n - 1 as libc::c_int - -sel % 5 as libc::c_int
                    }
                    /* C's modulus operator does not have defined
					 results for negative first values. Damn. */
                    dump_races(sel);
                } else if c as libc::c_int == '6' as i32 {
                    sel += 1;
                    if sel >= n { sel = 0 as libc::c_int }
                    dump_races(sel);
                } else if c as libc::c_int == '4' as i32 {
                    sel -= 1;
                    if sel < 0 as libc::c_int { sel = n - 1 as libc::c_int }
                    dump_races(sel);
                } else if c as libc::c_int == '\r' as i32 {
                    k = sel;
                    break ;
                } else { bell(); }
            }
        }
    }
    /* Set race */
    (*p_ptr).prace = k as byte_hack;
    rp_ptr =
        &mut *race_info.offset((*p_ptr).prace as isize) as *mut player_race;
    str = rp_name.offset((*rp_ptr).title as isize) as cptr;
    /* Display */
    c_put_str(14 as libc::c_int as byte_hack, str, 4 as libc::c_int,
              9 as libc::c_int);
    /* Get a random name */
    if do_quick_start == 0 {
        create_random_name((*p_ptr).prace as libc::c_int,
                           player_name.as_mut_ptr());
    }
    /* Display */
    c_put_str(14 as libc::c_int as byte_hack,
              player_name.as_mut_ptr() as cptr, 2 as libc::c_int,
              9 as libc::c_int);
    /* Clean up */
    clear_from(12 as libc::c_int);
    /* ** Player race mod ***/
    if do_quick_start != 0 {
        k = previous_char.rmod as libc::c_int;
        (*p_ptr).pracem = k as byte_hack;
        rmp_ptr =
            &mut *race_mod_info.offset((*p_ptr).pracem as isize) as
                *mut player_race_mod
    } else if max_rmp_idx as libc::c_int == 1 as libc::c_int {
        k = 0 as libc::c_int
    } else {
        n = 0 as libc::c_int;
        while n < 100 as libc::c_int {
            racem[n as usize] = 0 as libc::c_int;
            n += 1
        }
        max_racem = 0 as libc::c_int;
        n = 0 as libc::c_int;
        while n < max_rmp_idx as libc::c_int {
            /* Only one choice = instant choice */
            /* Analyze */
            (*p_ptr).pracem = n as byte_hack;
            rmp_ptr =
                &mut *race_mod_info.offset((*p_ptr).pracem as isize) as
                    *mut player_race_mod;
            /* Must be an ok choice */
            if !((1 as libc::c_long) << (*p_ptr).prace as libc::c_int &
                     (*rmp_ptr).choice[((*p_ptr).prace as libc::c_int /
                                            32 as libc::c_int) as usize] as
                         libc::c_long == 0) {
                /* Ok thats a possibility */
                let fresh3 = max_racem;
                max_racem = max_racem + 1;
                racem[fresh3 as usize] = n
            }
            n += 1
        }
        /* Ah ! nothing found, lets use the default */
        if max_racem == 0 {
            (*p_ptr).pracem = 0 as libc::c_int as byte_hack
        } else if max_racem == 1 as libc::c_int {
            (*p_ptr).pracem = racem[0 as libc::c_int as usize] as byte_hack
        } else {
            /* Only one ? use it */
            /* We got to ask the player */
            /* Extra info */
            Term_putstr(5 as libc::c_int, 15 as libc::c_int,
                        -(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                        b"Your \'race modifier\' determines various intrinsic factors and bonuses.\x00"
                            as *const u8 as *const libc::c_char);
            sel = 0 as libc::c_int;
            n = dump_rmods(sel, racem.as_mut_ptr(), max_racem);
            loop 
                 /* Dump races */
                 /* Choose */
                 {
                strnfmt(buf.as_mut_ptr(), 200 as libc::c_int as uint_hack,
                        b"Choose a race modifier (%c-%c), * for a random choice, = for options: \x00"
                            as *const u8 as *const libc::c_char,
                        0 as libc::c_int + 'a' as i32,
                        max_racem - 1 as libc::c_int + 'a' as i32);
                put_str(buf.as_mut_ptr() as cptr, 17 as libc::c_int,
                        2 as libc::c_int);
                c = inkey();
                if c as libc::c_int == 'Q' as i32 { quit(0 as cptr); }
                if c as libc::c_int == 'S' as i32 {
                    return 0 as libc::c_int as bool_
                }
                if c as libc::c_int == '*' as i32 {
                    loop  {
                        k = Rand_div(max_racem);
                        if !((1 as libc::c_long) << racem[k as usize] &
                                 (*rmp_ptr).choice[(racem[k as usize] /
                                                        32 as libc::c_int) as
                                                       usize] as libc::c_long
                                 == 0) {
                            break ;
                        }
                    }
                    break ;
                } else {
                    if c as libc::c_int == '?' as i32 {
                        exec_lua(format(b"ingame_help(\'select_context\', \'subrace\', \'%s\')\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        rmp_name.offset((*race_mod_info.offset(racem[sel
                                                                                         as
                                                                                         usize]
                                                                                   as
                                                                                   isize)).title
                                                            as isize)));
                    }
                    k =
                        if *(*__ctype_b_loc()).offset(c as libc::c_int as
                                                          isize) as
                               libc::c_int &
                               _ISlower as libc::c_int as libc::c_ushort as
                                   libc::c_int != 0 {
                            (c as libc::c_int) - 'a' as i32
                        } else { -(1 as libc::c_int) };
                    if k >= 0 as libc::c_int && k < max_racem &&
                           (1 as libc::c_long) <<
                               (*p_ptr).prace as libc::c_int &
                               (*race_mod_info.offset(racem[k as usize] as
                                                          isize)).choice[((*p_ptr).prace
                                                                              as
                                                                              libc::c_int
                                                                              /
                                                                              32
                                                                                  as
                                                                                  libc::c_int)
                                                                             as
                                                                             usize]
                                   as libc::c_long != 0 {
                        break ;
                    }
                    if c as libc::c_int == '=' as i32 {
                        screen_save();
                        do_cmd_options_aux(6 as libc::c_int,
                                           b"Startup Options\x00" as *const u8
                                               as *const libc::c_char,
                                           0 as libc::c_int as bool_);
                        screen_load();
                    } else if c as libc::c_int == '2' as i32 {
                        sel += 5 as libc::c_int;
                        if sel >= n { sel = sel - n + 1 as libc::c_int }
                        dump_rmods(sel, racem.as_mut_ptr(), max_racem);
                    } else if c as libc::c_int == '8' as i32 {
                        sel -= 5 as libc::c_int;
                        if sel < 0 as libc::c_int {
                            sel = n - 1 as libc::c_int + sel
                        }
                        dump_rmods(sel, racem.as_mut_ptr(), max_racem);
                    } else if c as libc::c_int == '6' as i32 {
                        sel += 1;
                        if sel >= n { sel = 0 as libc::c_int }
                        dump_rmods(sel, racem.as_mut_ptr(), max_racem);
                    } else if c as libc::c_int == '4' as i32 {
                        sel -= 1;
                        if sel < 0 as libc::c_int {
                            sel = n - 1 as libc::c_int
                        }
                        dump_rmods(sel, racem.as_mut_ptr(), max_racem);
                    } else if c as libc::c_int == '\r' as i32 {
                        k = sel;
                        break ;
                    } else { bell(); }
                }
            }
            (*p_ptr).pracem = racem[k as usize] as byte_hack
        }
        rmp_ptr =
            &mut *race_mod_info.offset((*p_ptr).pracem as isize) as
                *mut player_race_mod;
        /* Set race */
        /* Display */
        c_put_str(14 as libc::c_int as byte_hack,
                  get_player_race_name((*p_ptr).prace as libc::c_int,
                                       (*p_ptr).pracem as libc::c_int),
                  4 as libc::c_int, 9 as libc::c_int);
    }
    /* Clean up */
    clear_from(12 as libc::c_int);
    /* ** Player class ***/
    if do_quick_start != 0 {
        k = previous_char.pclass as libc::c_int;
        (*p_ptr).pclass = k as byte_hack;
        cp_ptr =
            &mut *class_info.offset((*p_ptr).pclass as isize) as
                *mut player_class;
        k = previous_char.spec as libc::c_int;
        (*p_ptr).pspec = k as byte_hack;
        spp_ptr =
            &mut *(*class_info.offset((*p_ptr).pclass as
                                          isize)).spec.as_mut_ptr().offset((*p_ptr).pspec
                                                                               as
                                                                               isize)
                as *mut player_spec
    } else {
        let mut z: libc::c_int = 0;
        z = 0 as libc::c_int;
        while z < 2 as libc::c_int {
            restrictions[z as usize] =
                ((*rp_ptr).choice[z as usize] | (*rmp_ptr).pclass[z as usize])
                    & !(*rmp_ptr).mclass[z as usize];
            z += 1
        }
        if max_mc_idx as libc::c_int > 1 as libc::c_int {
            /* Extra info */
            Term_putstr(5 as libc::c_int, 13 as libc::c_int,
                        -(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                        b"Your \'class\' determines various intrinsic abilities and bonuses.\x00"
                            as *const u8 as *const libc::c_char);
            /* Get a class type */
            i = 0 as libc::c_int;
            while i < max_mc_idx as libc::c_int {
                c_put_str((*meta_class_info.offset(i as isize)).color,
                          format(b"%c) %s\x00" as *const u8 as
                                     *const libc::c_char, i + 'a' as i32,
                                 (*meta_class_info.offset(i as
                                                              isize)).name.as_mut_ptr())
                              as cptr, 16 as libc::c_int + i,
                          2 as libc::c_int);
                i += 1
            }
            loop  {
                strnfmt(buf.as_mut_ptr(), 200 as libc::c_int as uint_hack,
                        b"Choose a class type (a-%c), * for random, = for options: \x00"
                            as *const u8 as *const libc::c_char,
                        max_mc_idx as libc::c_int - 1 as libc::c_int +
                            'a' as i32);
                put_str(buf.as_mut_ptr() as cptr, 15 as libc::c_int,
                        2 as libc::c_int);
                c = inkey();
                if c as libc::c_int == 'Q' as i32 { quit(0 as cptr); }
                if c as libc::c_int == 'S' as i32 {
                    return 0 as libc::c_int as bool_
                }
                if c as libc::c_int == '*' as i32 {
                    k = Rand_div(max_mc_idx as s32b);
                    break ;
                } else {
                    k =
                        if *(*__ctype_b_loc()).offset(c as libc::c_int as
                                                          isize) as
                               libc::c_int &
                               _ISlower as libc::c_int as libc::c_ushort as
                                   libc::c_int != 0 {
                            (c as libc::c_int) - 'a' as i32
                        } else {
                            (c as libc::c_int - '0' as i32) +
                                26 as libc::c_int
                        };
                    if k >= 0 as libc::c_int && k < max_mc_idx as libc::c_int
                       {
                        break ;
                    }
                    if c as libc::c_int == '?' as i32 {
                        do_cmd_help();
                    } else if c as libc::c_int == '=' as i32 {
                        screen_save();
                        do_cmd_options_aux(6 as libc::c_int,
                                           b"Startup Options\x00" as *const u8
                                               as *const libc::c_char,
                                           0 as libc::c_int as bool_);
                        screen_load();
                    } else { bell(); }
                }
            }
        } else { k = 0 as libc::c_int }
        class_types = (*meta_class_info.offset(k as isize)).classes;
        clear_from(15 as libc::c_int);
        /* Count classes */
        n = 0 as libc::c_int;
        while *class_types.offset(n as isize) as libc::c_int !=
                  -(1 as libc::c_int) {
            n += 1
        }
        /* Only one choice = instant choice */
        if n == 1 as libc::c_int {
            k = 0 as libc::c_int
        } else {
            /* Dump classes */
            sel = 0 as libc::c_int;
            n = dump_classes(class_types, sel, restrictions.as_mut_ptr());
            loop 
                 /* Get a class */
                 {
                strnfmt(buf.as_mut_ptr(), 200 as libc::c_int as uint_hack,
                        b"Choose a class (%c-%c), * for random, = for options, 8/2/4 for up/down/back: \x00"
                            as *const u8 as *const libc::c_char,
                        0 as libc::c_int + 'a' as i32,
                        if n <= 25 as libc::c_int {
                            (n - 1 as libc::c_int) + 'a' as i32
                        } else {
                            (n - 26 as libc::c_int - 1 as libc::c_int) +
                                '0' as i32
                        });
                put_str(buf.as_mut_ptr() as cptr, 15 as libc::c_int,
                        2 as libc::c_int);
                c = inkey();
                if c as libc::c_int == 'Q' as i32 { quit(0 as cptr); }
                if c as libc::c_int == 'S' as i32 {
                    return 0 as libc::c_int as bool_
                }
                if c as libc::c_int == '*' as i32 {
                    k = Rand_div(n) + 1 as libc::c_int - 1 as libc::c_int;
                    break ;
                } else {
                    k =
                        if *(*__ctype_b_loc()).offset(c as libc::c_int as
                                                          isize) as
                               libc::c_int &
                               _ISlower as libc::c_int as libc::c_ushort as
                                   libc::c_int != 0 {
                            (c as libc::c_int) - 'a' as i32
                        } else {
                            (c as libc::c_int - '0' as i32) +
                                26 as libc::c_int
                        };
                    if k >= 0 as libc::c_int && k < n { break ; }
                    if c as libc::c_int == '?' as i32 {
                        exec_lua(format(b"ingame_help(\'select_context\', \'class\', \'%s\')\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        c_name.offset((*class_info.offset(*class_types.offset(sel
                                                                                                  as
                                                                                                  isize)
                                                                              as
                                                                              isize)).title
                                                          as isize)));
                    } else if c as libc::c_int == '=' as i32 {
                        screen_save();
                        do_cmd_options_aux(6 as libc::c_int,
                                           b"Startup Options\x00" as *const u8
                                               as *const libc::c_char,
                                           0 as libc::c_int as bool_);
                        screen_load();
                    } else if c as libc::c_int == '2' as i32 {
                        sel += 4 as libc::c_int;
                        if sel >= n { sel %= 4 as libc::c_int }
                        dump_classes(class_types, sel,
                                     restrictions.as_mut_ptr());
                    } else if c as libc::c_int == '8' as i32 {
                        sel -= 4 as libc::c_int;
                        if sel < 0 as libc::c_int {
                            sel =
                                n - 1 as libc::c_int - -sel % 4 as libc::c_int
                        }
                        /* C's modulus operator does not have defined
					 results for negative first values. Damn. */
                        dump_classes(class_types, sel,
                                     restrictions.as_mut_ptr());
                    } else if c as libc::c_int == '6' as i32 {
                        sel += 1;
                        if sel >= n { sel = 0 as libc::c_int }
                        dump_classes(class_types, sel,
                                     restrictions.as_mut_ptr());
                    } else if c as libc::c_int == '4' as i32 {
                        sel -= 1;
                        if sel < 0 as libc::c_int {
                            sel = n - 1 as libc::c_int
                        }
                        dump_classes(class_types, sel,
                                     restrictions.as_mut_ptr());
                    } else if c as libc::c_int == '\r' as i32 {
                        k = sel;
                        break ;
                    } else { bell(); }
                }
            }
        }
        /* Set class */
        (*p_ptr).pclass = *class_types.offset(k as isize) as byte_hack;
        /* Choose class spec */
        clear_from(15 as libc::c_int);
        /* Count choices */
        n = 0 as libc::c_int;
        while n < 20 as libc::c_int {
            /* Found the last one ? */
            if (*class_info.offset((*p_ptr).pclass as
                                       isize)).spec[n as usize].title == 0 {
                break ;
            }
            n += 1
        }
        /* Only one choice = auto choice */
        if n == 1 as libc::c_int {
            k = 0 as libc::c_int
        } else {
            /* Dump classes spec */
            sel = 0 as libc::c_int;
            n = dump_specs(sel);
            loop 
                 /* Get a class */
                 {
                strnfmt(buf.as_mut_ptr(), 200 as libc::c_int as uint_hack,
                        b"Choose a class specialisation (%c-%c), * for random, = for options, 8/2/4/6 for up/down/left/right: \x00"
                            as *const u8 as *const libc::c_char,
                        0 as libc::c_int + 'a' as i32,
                        if n <= 25 as libc::c_int {
                            (n - 1 as libc::c_int) + 'a' as i32
                        } else {
                            (n - 26 as libc::c_int - 1 as libc::c_int) +
                                '0' as i32
                        });
                put_str(buf.as_mut_ptr() as cptr, 15 as libc::c_int,
                        2 as libc::c_int);
                c = inkey();
                if c as libc::c_int == 'Q' as i32 { quit(0 as cptr); }
                if c as libc::c_int == 'S' as i32 {
                    return 0 as libc::c_int as bool_
                }
                if c as libc::c_int == '*' as i32 {
                    k = Rand_div(n) + 1 as libc::c_int - 1 as libc::c_int;
                    break ;
                } else {
                    k =
                        if *(*__ctype_b_loc()).offset(c as libc::c_int as
                                                          isize) as
                               libc::c_int &
                               _ISlower as libc::c_int as libc::c_ushort as
                                   libc::c_int != 0 {
                            (c as libc::c_int) - 'a' as i32
                        } else {
                            (c as libc::c_int - '0' as i32) +
                                26 as libc::c_int
                        };
                    if k >= 0 as libc::c_int && k < n { break ; }
                    if c as libc::c_int == '?' as i32 {
                        exec_lua(format(b"ingame_help(\'select_context\', \'class\', \'%s\')\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        c_name.offset((*class_info.offset((*p_ptr).pclass
                                                                              as
                                                                              isize)).spec[sel
                                                                                               as
                                                                                               usize].title
                                                          as isize)));
                    } else if c as libc::c_int == '=' as i32 {
                        screen_save();
                        do_cmd_options_aux(6 as libc::c_int,
                                           b"Startup Options\x00" as *const u8
                                               as *const libc::c_char,
                                           0 as libc::c_int as bool_);
                        screen_load();
                    } else if c as libc::c_int == '2' as i32 {
                        sel += 4 as libc::c_int;
                        if sel >= n { sel = sel - n + 1 as libc::c_int }
                        dump_specs(sel);
                    } else if c as libc::c_int == '8' as i32 {
                        sel -= 4 as libc::c_int;
                        if sel < 0 as libc::c_int {
                            sel = n - 1 as libc::c_int + sel
                        }
                        dump_specs(sel);
                    } else if c as libc::c_int == '6' as i32 {
                        sel += 1;
                        if sel >= n { sel = 0 as libc::c_int }
                        dump_specs(sel);
                    } else if c as libc::c_int == '4' as i32 {
                        sel -= 1;
                        if sel < 0 as libc::c_int {
                            sel = n - 1 as libc::c_int
                        }
                        dump_specs(sel);
                    } else if c as libc::c_int == '\r' as i32 {
                        k = sel;
                        break ;
                    } else { bell(); }
                }
            }
        }
        /* Set class spec */
        (*p_ptr).pspec = k as byte_hack
    }
    cp_ptr =
        &mut *class_info.offset((*p_ptr).pclass as isize) as
            *mut player_class;
    spp_ptr =
        &mut *(*class_info.offset((*p_ptr).pclass as
                                      isize)).spec.as_mut_ptr().offset((*p_ptr).pspec
                                                                           as
                                                                           isize)
            as *mut player_spec;
    str = c_name.offset((*spp_ptr).title as isize) as cptr;
    /* Display */
    c_put_str(14 as libc::c_int as byte_hack, str, 5 as libc::c_int,
              9 as libc::c_int);
    /* Clean up */
    clear_from(15 as libc::c_int);
    /* ** Player god ***/
    if do_quick_start != 0 {
        k = previous_char.god as libc::c_int;
        (*p_ptr).pgod = k as byte_hack;
        set_grace(previous_char.grace);
    } else if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
                   (*spp_ptr).flags1) as libc::c_long & 0x4000 as libc::c_long
                  != 0 {
        (*p_ptr).pgod = 0 as libc::c_int as byte_hack
    } else {
        let mut choice: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut max: libc::c_int = 0 as libc::c_int;
        choice =
            memset(ralloc((max_gods as
                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                               as
                                                               libc::c_ulong))
                       as *mut libc::c_char as *mut libc::c_void,
                   0 as libc::c_int,
                   (max_gods as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                        as libc::c_ulong)) as
                *mut libc::c_int;
        /* Get the list of possible gods */
        n = 0 as libc::c_int;
        while n < max_gods {
            if ((*cp_ptr).gods | (*spp_ptr).gods) as libc::c_long &
                   (1 as libc::c_long) << n != 0 {
                let fresh4 = max;
                max = max + 1;
                *choice.offset(fresh4 as isize) = n
            }
            n += 1
        }
        if max == 0 {
            (*p_ptr).pgod = 0 as libc::c_int as byte_hack
        } else if max == 1 as libc::c_int {
            (*p_ptr).pgod =
                *choice.offset(0 as libc::c_int as isize) as byte_hack
        } else if max > 1 as libc::c_int {
            sel = 0 as libc::c_int;
            n = dump_gods(sel, choice, max);
            loop 
                 /* Choose */
                 {
                strnfmt(buf.as_mut_ptr(), 200 as libc::c_int as uint_hack,
                        b"Choose a god (%c-%c), * for a random choice, = for options, 8/2/4/6 for movement: \x00"
                            as *const u8 as *const libc::c_char,
                        0 as libc::c_int + 'a' as i32,
                        max - 1 as libc::c_int + 'a' as i32);
                put_str(buf.as_mut_ptr() as cptr, 19 as libc::c_int,
                        2 as libc::c_int);
                c = inkey();
                if c as libc::c_int == 'Q' as i32 { quit(0 as cptr); }
                if c as libc::c_int == 'S' as i32 {
                    rnfree(choice as vptr,
                           (max_gods as
                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                                as
                                                                libc::c_ulong));
                    return 0 as libc::c_int as bool_
                }
                if c as libc::c_int == '*' as i32 {
                    k =
                        *choice.offset((Rand_div(max) + 1 as libc::c_int -
                                            1 as libc::c_int) as isize);
                    break ;
                } else {
                    k =
                        if *(*__ctype_b_loc()).offset(c as libc::c_int as
                                                          isize) as
                               libc::c_int &
                               _ISlower as libc::c_int as libc::c_ushort as
                                   libc::c_int != 0 {
                            (c as libc::c_int) - 'a' as i32
                        } else { -(1 as libc::c_int) };
                    if k >= 0 as libc::c_int && k < max {
                        k = *choice.offset(k as isize);
                        break ;
                    } else if c as libc::c_int == '?' as i32 {
                        exec_lua(format(b"ingame_help(\'select_context\', \'god\', \'%s\')\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        (*deity_info.offset(*choice.offset(sel
                                                                               as
                                                                               isize)
                                                                as
                                                                isize)).name));
                    } else if c as libc::c_int == '=' as i32 {
                        screen_save();
                        do_cmd_options_aux(6 as libc::c_int,
                                           b"Startup Options\x00" as *const u8
                                               as *const libc::c_char,
                                           0 as libc::c_int as bool_);
                        screen_load();
                    } else if c as libc::c_int == '2' as i32 {
                        sel += 4 as libc::c_int;
                        if sel >= n { sel %= 4 as libc::c_int }
                        dump_gods(sel, choice, max);
                    } else if c as libc::c_int == '8' as i32 {
                        sel -= 4 as libc::c_int;
                        /* C's modulus operator does not have defined
					   results for negative first values. Damn. */
                        if sel < 0 as libc::c_int {
                            sel =
                                n - 1 as libc::c_int - -sel % 4 as libc::c_int
                        }
                        dump_gods(sel, choice, max);
                    } else if c as libc::c_int == '6' as i32 {
                        sel += 1;
                        if sel >= n { sel = 0 as libc::c_int }
                        dump_gods(sel, choice, max);
                    } else if c as libc::c_int == '4' as i32 {
                        sel -= 1;
                        if sel < 0 as libc::c_int {
                            sel = n - 1 as libc::c_int
                        }
                        dump_gods(sel, choice, max);
                    } else if c as libc::c_int == '\r' as i32 {
                        k = *choice.offset(sel as isize);
                        break ;
                    } else { bell(); }
                }
            }
            rnfree(choice as vptr,
                   (max_gods as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                        as libc::c_ulong));
            /* Set god */
            (*p_ptr).pgod = k as byte_hack;
            (*p_ptr).grace = 0 as libc::c_int
        }
        /* A god that like us ? more grace ! */
        if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
                (*spp_ptr).flags1) as libc::c_long & 0x400000 as libc::c_long
               != 0 {
            set_grace(200 as libc::c_int);
        } else { set_grace(100 as libc::c_int); }
    }
    /* Clean up */
    clear_from(12 as libc::c_int);
    if do_quick_start == 0 {
        /* Clear */
        clear_from(15 as libc::c_int);
        /*  */
        if get_check(b"Do you want to modify the options\x00" as *const u8 as
                         *const libc::c_char) != 0 {
            screen_save();
            do_cmd_options_aux(6 as libc::c_int,
                               b"Startup Options\x00" as *const u8 as
                                   *const libc::c_char,
                               0 as libc::c_int as bool_);
            screen_load();
        }
    }
    /* Set birth options: maximize, preserve, sepcial levels and astral */
    (*p_ptr).maximize = maximize as byte_hack;
    (*p_ptr).preserve = preserve as byte_hack;
    (*p_ptr).special = special_lvls as byte_hack;
    (*p_ptr).astral =
        if ((*rp_ptr).flags2 | (*rmp_ptr).flags2 | (*cp_ptr).flags2 |
                (*spp_ptr).flags2) as libc::c_long & 0x2 as libc::c_long != 0
           {
            1 as libc::c_int
        } else { 0 as libc::c_int } as bool_;
    /*
	 * A note by pelpel. (remove this please)
	 * Be it the new Vanilla way (adult vs. birth options) or
	 * the old one (player_type members), it would be less confusing
	 * to handle birth-only options in a uniform fashion,the above and
	 * the following:
	 * ironman_rooms,
	 * joke_monsters,
	 * always_small_level, and
	 * fate_option
	 */
    /* Set the recall dungeon accordingly */
    call_lua(b"get_module_info\x00" as *const u8 as *const libc::c_char,
             b"(s)\x00" as *const u8 as *const libc::c_char,
             b"d\x00" as *const u8 as *const libc::c_char,
             b"base_dungeon\x00" as *const u8 as *const libc::c_char,
             &mut tmp as *mut s32b);
    dungeon_type = tmp as byte_hack;
    (*p_ptr).recall_dungeon = dungeon_type as s16b;
    *max_dlv.offset(dungeon_type as isize) =
        (*d_info.offset(dungeon_type as isize)).mindepth;
    if (*p_ptr).astral != 0 {
        let mut x: s32b = 0;
        let mut y: s32b = 0;
        let mut astral_dun: s32b = 0;
        call_lua(b"get_module_info\x00" as *const u8 as *const libc::c_char,
                 b"(s)\x00" as *const u8 as *const libc::c_char,
                 b"d\x00" as *const u8 as *const libc::c_char,
                 b"astral_dungeon\x00" as *const u8 as *const libc::c_char,
                 &mut astral_dun as *mut s32b);
        dungeon_type = astral_dun as byte_hack;
        /* Somewhere in the misty mountains */
        call_lua(b"get_module_info\x00" as *const u8 as *const libc::c_char,
                 b"(s)\x00" as *const u8 as *const libc::c_char,
                 b"d\x00" as *const u8 as *const libc::c_char,
                 b"astral_wild_x\x00" as *const u8 as *const libc::c_char,
                 &mut x as *mut s32b);
        call_lua(b"get_module_info\x00" as *const u8 as *const libc::c_char,
                 b"(s)\x00" as *const u8 as *const libc::c_char,
                 b"d\x00" as *const u8 as *const libc::c_char,
                 b"astral_wild_y\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut s32b);
        (*p_ptr).wilderness_x = x;
        (*p_ptr).wilderness_y = y
    }
    /* Clean up */
    clear_from(10 as libc::c_int);
    /* ** User enters number of quests ***/
	/* Heino Vander Sanden and Jimmy De Laet */
    call_lua(b"get_module_info\x00" as *const u8 as *const libc::c_char,
             b"(s)\x00" as *const u8 as *const libc::c_char,
             b"d\x00" as *const u8 as *const libc::c_char,
             b"rand_quest\x00" as *const u8 as *const libc::c_char,
             &mut allow_quest as *mut s32b);
    if ironman_rooms == 0 && allow_quest != 0 {
        if do_quick_start != 0 {
            v = previous_char.quests as libc::c_int
        } else {
            /* Extra info */
            Term_putstr(5 as libc::c_int, 15 as libc::c_int,
                        -(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                        b"Select the number of optional random quests you\'d like to receive.\x00"
                            as *const u8 as *const libc::c_char);
            Term_putstr(5 as libc::c_int, 16 as libc::c_int,
                        -(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                        b"If you do not want any optional quests, enter 0.\x00"
                            as *const u8 as *const libc::c_char);
            /* Ask the number of additional quests */
            put_str(format(b"Number of quests? (0-%u) \x00" as *const u8 as
                               *const libc::c_char,
                           99 as libc::c_int - 1 as libc::c_int) as cptr,
                    20 as libc::c_int, 2 as libc::c_int);
            loop 
                 /* Get a the number of additional quest */
                 /* Move the cursor */
                 {
                put_str(b"\x00" as *const u8 as *const libc::c_char,
                        20 as libc::c_int, 27 as libc::c_int);
                /* Default */
                strcpy(inp.as_mut_ptr(),
                       b"20\x00" as *const u8 as *const libc::c_char);
                /* Get a response (or escape) */
                if askfor_aux(inp.as_mut_ptr(), 2 as libc::c_int) == 0 {
                    inp[0 as libc::c_int as usize] =
                        '\u{0}' as i32 as libc::c_char
                }
                if inp[0 as libc::c_int as usize] as libc::c_int == '*' as i32
                   {
                    v = Rand_div(99 as libc::c_int)
                } else { v = atoi(inp.as_mut_ptr()) }
                /* Break on valid input */
                if v < 99 as libc::c_int && v >= 0 as libc::c_int { break ; }
            }
            /* Clear */
            clear_from(15 as libc::c_int);
        }
    } else {
        /* NO quests for ironman rooms or persistent levels, since they
		   don't work */
        v = 0 as libc::c_int
    }
    /* Set the quest monster hook */
    get_mon_num_hook =
        Some(monster_quest as unsafe extern "C" fn(_: libc::c_int) -> bool_);
    /* Prepare allocation table */
    get_mon_num_prep();
    /* Generate quests */
    i = 0 as libc::c_int;
    while i < 99 as libc::c_int {
        random_quests[i as usize].type_0 = 0 as libc::c_int as byte_hack;
        i += 1
    }
    if v != 0 { gen_random_quests(v); }
    max_quests = v as byte_hack;
    (*p_ptr).inside_quest = 0 as libc::c_int as s16b;
    /* Init the plots */
    call_lua(b"get_module_info\x00" as *const u8 as *const libc::c_char,
             b"(s)\x00" as *const u8 as *const libc::c_char,
             b"d\x00" as *const u8 as *const libc::c_char,
             b"C_quest\x00" as *const u8 as *const libc::c_char,
             &mut allow_quest as *mut s32b);
    if allow_quest != 0 {
        plots[0 as libc::c_int as usize] = 1 as libc::c_int as s16b;
        (*quest.offset(plots[0 as libc::c_int as usize] as isize)).status =
            1 as libc::c_int as s16b;
        plots[1 as libc::c_int as usize] = 4 as libc::c_int as s16b;
        (*quest.offset(plots[1 as libc::c_int as usize] as isize)).status =
            0 as libc::c_int as s16b;
        plots[2 as libc::c_int as usize] = 22 as libc::c_int as s16b;
        (*quest.offset(plots[2 as libc::c_int as usize] as isize)).status =
            0 as libc::c_int as s16b;
        plots[4 as libc::c_int as usize] = 23 as libc::c_int as s16b;
        (*quest.offset(plots[4 as libc::c_int as usize] as isize)).status =
            0 as libc::c_int as s16b;
        plots[5 as libc::c_int as usize] = 24 as libc::c_int as s16b;
        (*quest.offset(plots[5 as libc::c_int as usize] as isize)).status =
            0 as libc::c_int as s16b;
        plots[6 as libc::c_int as usize] = 25 as libc::c_int as s16b;
        (*quest.offset(plots[6 as libc::c_int as usize] as isize)).status =
            0 as libc::c_int as s16b;
        plots[3 as libc::c_int as usize] = 0 as libc::c_int as s16b
    }
    quest_random_init_hook(5 as libc::c_int);
    /* Ok */
    return 1 as libc::c_int as bool_;
}
/*
 * Initial stat costs (initial stats always range from 10 to 18 inclusive).
 */
static mut birth_stat_costs: [libc::c_int; 9] =
    [0 as libc::c_int, 1 as libc::c_int, 2 as libc::c_int, 4 as libc::c_int,
     7 as libc::c_int, 11 as libc::c_int, 16 as libc::c_int,
     22 as libc::c_int, 30 as libc::c_int];
/*
 * Helper function for 'player_birth()'.
 *
 * This function handles "point-based" character creation.
 *
 * The player selects, for each stat, a value from 10 to 18 (inclusive),
 * each costing a certain amount of points (as above), from a pool of 48
 * available points, to which race/class modifiers are then applied.
 *
 * Each unused point is converted into 100 gold pieces, with a maximum of
 * 600 gp at birth.
 *
 * Taken from V 2.9.0
 */
unsafe extern "C" fn player_birth_aux_point() -> bool_ {
    let mut i: libc::c_int = 0;
    let mut row: libc::c_int = 3 as libc::c_int;
    let mut col: libc::c_int = 42 as libc::c_int;
    let mut stat: libc::c_int = 0 as libc::c_int;
    let mut stats: [libc::c_int; 6] = [0; 6];
    let mut cost: libc::c_int = 0;
    let mut ch: libc::c_char = 0;
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut mode: libc::c_int = 0 as libc::c_int;
    /* Initialize stats */
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        /* Initial stats */
        stats[i as usize] = 10 as libc::c_int;
        i += 1
    }
    /* Roll for base hitpoints */
    get_extra();
    /* Roll for age/height/weight */
    get_ahw();
    /* Roll for social class */
    get_history();
    /* ** Generate ***/
    process_hooks(68 as libc::c_int,
                  b"()\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char);
    /* Hack -- get a chaos patron even if you are not a chaos warrior */
    (*p_ptr).chaos_patron =
        (Rand_div(16 as libc::c_int) + 1 as libc::c_int - 1 as libc::c_int) as
            s16b;
    /* Get luck */
    (*p_ptr).luck_base =
        ((*rp_ptr).luck as libc::c_int + (*rmp_ptr).luck as libc::c_int +
             (-(5 as libc::c_int) +
                  Rand_div(1 as libc::c_int + 5 as libc::c_int -
                               -(5 as libc::c_int)))) as s16b;
    (*p_ptr).luck_max = (*p_ptr).luck_base;
    loop 
         /* Interact */
         /* Reset cost */
         {
        cost = 0 as libc::c_int;
        /* Process stats */
        i = 0 as libc::c_int;
        while i < 6 as libc::c_int {
            /* Variable stat maxes */
            if (*p_ptr).maximize != 0 {
                /* Reset stats */
                (*p_ptr).stat_max[i as usize] = stats[i as usize] as s16b;
                (*p_ptr).stat_cur[i as usize] = (*p_ptr).stat_max[i as usize]
            } else {
                /* Fixed stat maxes */
                /* Obtain a "bonus" for "race" and "class" */
                let mut bonus: libc::c_int =
                    (*rp_ptr).r_adj[i as usize] as libc::c_int +
                        (*cp_ptr).c_adj[i as usize] as libc::c_int;
                (*p_ptr).stat_max[i as usize] =
                    modify_stat_value(stats[i as usize], bonus);
                (*p_ptr).stat_cur[i as usize] = (*p_ptr).stat_max[i as usize]
            }
            /* Apply the racial/class bonuses */
            /* Total cost */
            cost +=
                birth_stat_costs[(stats[i as usize] - 10 as libc::c_int) as
                                     usize];
            i += 1
        }
        /* Restrict cost */
        if cost > 48 as libc::c_int {
            /* Warning */
            bell();
            /* Reduce stat */
            stats[stat as usize] -= 1
        } else {
            /* Gold is inversely proportional to cost */
            (*p_ptr).au =
                100 as libc::c_int * (48 as libc::c_int - cost) +
                    100 as libc::c_int;
            /* Maximum of 600 gold */
            if (*p_ptr).au > 600 as libc::c_int {
                (*p_ptr).au = 600 as libc::c_int
            }
            /* Calculate the bonuses and hitpoints */
            (*p_ptr).update =
                ((*p_ptr).update as libc::c_long |
                     (0x1 as libc::c_long | 0x10 as libc::c_long)) as u32b;
            /* Update stuff */
            update_stuff();
            /* Fully healed */
            (*p_ptr).chp = (*p_ptr).mhp;
            /* Fully rested */
            (*p_ptr).csp = (*p_ptr).msp;
            /* Display the player */
            display_player(mode);
            /* Display the costs header */
            put_str(b"Cost\x00" as *const u8 as *const libc::c_char,
                    row - 2 as libc::c_int, col + 32 as libc::c_int);
            /* Display the costs */
            i = 0 as libc::c_int;
            while i < 6 as libc::c_int {
                /* Display cost */
                strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                        b"%4d\x00" as *const u8 as *const libc::c_char,
                        birth_stat_costs[(stats[i as usize] -
                                              10 as libc::c_int) as usize]);
                put_str(buf.as_mut_ptr() as cptr,
                        row + (i - 1 as libc::c_int),
                        col + 32 as libc::c_int);
                i += 1
            }
            /* Prompt XXX XXX XXX */
            strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                    b"Total Cost %2d/48.  Use 2/8 to move, 4/6 to modify, ESC to accept.\x00"
                        as *const u8 as *const libc::c_char, cost);
            prt(buf.as_mut_ptr() as cptr, 0 as libc::c_int, 0 as libc::c_int);
            /* Place cursor just after cost of current stat */
            Term_gotoxy(col + 36 as libc::c_int,
                        row + stat - 1 as libc::c_int);
            /* Get key */
            ch = inkey();
            /* Quit */
            if ch as libc::c_int == 'Q' as i32 { quit(0 as cptr); }
            /* Start over */
            if ch as libc::c_int == 'S' as i32 {
                return 0 as libc::c_int as bool_
            }
            /* Done */
            if ch as libc::c_int == '\u{1b}' as i32 { break ; }
            /* Prev stat */
            if ch as libc::c_int == '8' as i32 {
                stat =
                    (stat + 6 as libc::c_int - 1 as libc::c_int) %
                        6 as libc::c_int
            }
            /* Next stat */
            if ch as libc::c_int == '2' as i32 {
                stat = (stat + 1 as libc::c_int) % 6 as libc::c_int
            }
            /* Decrease stat */
            if ch as libc::c_int == '4' as i32 &&
                   stats[stat as usize] > 10 as libc::c_int {
                stats[stat as usize] -= 1
            }
            /* Increase stat */
            if ch as libc::c_int == '6' as i32 &&
                   stats[stat as usize] < 18 as libc::c_int {
                stats[stat as usize] += 1
            }
        }
    }
    /* Done */
    return 1 as libc::c_int as bool_;
}
/*
 * Use the autoroller or not to generate a char
 */
unsafe extern "C" fn player_birth_aux_auto() -> bool_ {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut mode: libc::c_int = 0 as libc::c_int;
    let mut flag: bool_ = 0 as libc::c_int as bool_;
    let mut prev: bool_ = 0 as libc::c_int as bool_;
    let mut c: libc::c_char = 0;
    let mut b1: libc::c_char = '[' as i32 as libc::c_char;
    let mut b2: libc::c_char = ']' as i32 as libc::c_char;
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut inp: [libc::c_char; 80] = [0; 80];
    /* Initialize */
    if autoroll != 0 {
        let mut mval: [libc::c_int; 6] = [0; 6];
        /* Clear fields */
        auto_round = 0 as libc::c_long as s32b;
        last_round = 0 as libc::c_long as s32b;
        /* Clean up */
        clear_from(10 as libc::c_int);
        /* Prompt for the minimum stats */
        put_str(b"Enter minimum attribute for: \x00" as *const u8 as
                    *const libc::c_char, 15 as libc::c_int, 2 as libc::c_int);
        /* Output the maximum stats */
        i = 0 as libc::c_int;
        while i < 6 as libc::c_int {
            let mut stat_buf: [libc::c_char; 15] = [0; 15];
            /* Reset the "success" counter */
            stat_match[i as usize] = 0 as libc::c_int;
            /* Race/Class bonus */
            j =
                (*rp_ptr).r_adj[i as usize] as libc::c_int +
                    (*rmp_ptr).r_adj[i as usize] as libc::c_int +
                    (*cp_ptr).c_adj[i as usize] as libc::c_int;
            /* Obtain the "maximal" stat */
            m = adjust_stat(17 as libc::c_int, j, 1 as libc::c_int);
            /* Save the maximum */
            mval[i as usize] = m;
            /* Extract a textual format */
            cnv_stat(m, stat_buf.as_mut_ptr());
            strnfmt(inp.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                    b"(Max of %s):\x00" as *const u8 as *const libc::c_char,
                    stat_buf.as_mut_ptr());
            /* Prepare a prompt */
            strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                    b"%-5s: %-20s\x00" as *const u8 as *const libc::c_char,
                    stat_names[i as usize], inp.as_mut_ptr());
            /* Dump the prompt */
            put_str(buf.as_mut_ptr() as cptr, 16 as libc::c_int + i,
                    5 as libc::c_int);
            i += 1
        }
        /* Input the minimum stats */
        i = 0 as libc::c_int;
        while i < 6 as libc::c_int {
            loop 
                 /* Get a minimum stat */
                 {
                let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
                /* Move the cursor */
                put_str(b"\x00" as *const u8 as *const libc::c_char,
                        16 as libc::c_int + i, 30 as libc::c_int);
                /* Default */
                strcpy(inp.as_mut_ptr(),
                       b"\x00" as *const u8 as *const libc::c_char);
                /* Get a response (or escape) */
                if askfor_aux(inp.as_mut_ptr(), 8 as libc::c_int) == 0 {
                    inp[0 as libc::c_int as usize] =
                        '\u{0}' as i32 as libc::c_char
                }
                /* Weirdos stat display .. erm .. I mean, original stat display */
                if linear_stats == 0 {
                    /* Hack -- add a fake slash */
                    strcat(inp.as_mut_ptr(),
                           b"/\x00" as *const u8 as *const libc::c_char);
                    /* Hack -- look for the "slash" */
                    s = strchr(inp.as_mut_ptr(), '/' as i32);
                    /* Hack -- Nuke the slash */
                    let fresh5 = s;
                    s = s.offset(1);
                    *fresh5 = '\u{0}' as i32 as libc::c_char;
                    /* Hack -- Extract an input */
                    v = atoi(inp.as_mut_ptr()) + atoi(s)
                } else {
                    let mut z: libc::c_int = atoi(inp.as_mut_ptr());
                    if z <= 18 as libc::c_int {
                        v = z
                    } else {
                        let mut extra: libc::c_int = z - 18 as libc::c_int;
                        v = 18 as libc::c_int + extra * 10 as libc::c_int
                    }
                }
                /* Break on valid input */
                if v <= mval[i as usize] { break ; }
            }
            /* Save the minimum stat */
            stat_limit[i as usize] =
                if v > 0 as libc::c_int { v } else { 0 as libc::c_int } as
                    s16b;
            i += 1
        }
    }
    loop 
         /* ALLOW_AUTOROLLER */
         /* Roll */
         /* Feedback */
         {
        if autoroll != 0 {
            Term_clear();
            put_str(b"Name :\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int, 1 as libc::c_int);
            put_str(b"Sex  :\x00" as *const u8 as *const libc::c_char,
                    3 as libc::c_int, 1 as libc::c_int);
            put_str(b"Race :\x00" as *const u8 as *const libc::c_char,
                    4 as libc::c_int, 1 as libc::c_int);
            put_str(b"Class:\x00" as *const u8 as *const libc::c_char,
                    5 as libc::c_int, 1 as libc::c_int);
            c_put_str(14 as libc::c_int as byte_hack,
                      player_name.as_mut_ptr() as cptr, 2 as libc::c_int,
                      9 as libc::c_int);
            c_put_str(14 as libc::c_int as byte_hack, (*sp_ptr).title,
                      3 as libc::c_int, 9 as libc::c_int);
            strnfmt(buf.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    get_player_race_name((*p_ptr).prace as libc::c_int,
                                         (*p_ptr).pracem as libc::c_int));
            c_put_str(14 as libc::c_int as byte_hack,
                      buf.as_mut_ptr() as cptr, 4 as libc::c_int,
                      9 as libc::c_int);
            c_put_str(14 as libc::c_int as byte_hack,
                      c_name.offset((*spp_ptr).title as isize) as cptr,
                      5 as libc::c_int, 9 as libc::c_int);
            /* Label stats */
            put_str(b"STR:\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int + 0 as libc::c_int, 61 as libc::c_int);
            put_str(b"INT:\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int + 1 as libc::c_int, 61 as libc::c_int);
            put_str(b"WIS:\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int + 2 as libc::c_int, 61 as libc::c_int);
            put_str(b"DEX:\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int + 3 as libc::c_int, 61 as libc::c_int);
            put_str(b"CON:\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int + 4 as libc::c_int, 61 as libc::c_int);
            put_str(b"CHR:\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int + 5 as libc::c_int, 61 as libc::c_int);
            /* Note when we started */
            last_round = auto_round;
            /* Indicate the state */
            put_str(b"(Hit ESC to abort)\x00" as *const u8 as
                        *const libc::c_char, 11 as libc::c_int,
                    61 as libc::c_int);
            /* Label count */
            put_str(b"Round:\x00" as *const u8 as *const libc::c_char,
                    9 as libc::c_int, 61 as libc::c_int);
        } else {
            /* Otherwise just get a character */
            /* Get a new character */
            get_stats();
        }
        /* Auto-roll */
        while autoroll != 0 {
            let mut accept: bool_ = 1 as libc::c_int as bool_;
            /* Get a new character */
            get_stats();
            /* Advance the round */
            auto_round += 1;
            /* Hack -- Prevent overflow */
            if auto_round as libc::c_long >= 1000000 as libc::c_long {
                break ;
            }
            /* Check and count acceptable stats */
            i = 0 as libc::c_int;
            while i < 6 as libc::c_int {
                /* This stat is okay */
                if stat_use[i as usize] as libc::c_int >=
                       stat_limit[i as usize] as libc::c_int {
                    stat_match[i as usize] += 1
                } else {
                    /* This stat is not okay */
                    accept = 0 as libc::c_int as bool_
                }
                i += 1
            }
            /* Break if "happy" */
            if accept != 0 { break ; }
            /* Take note every 25 rolls */
            flag =
                (auto_round as libc::c_long % 25 as libc::c_long == 0) as
                    libc::c_int as bool_;
            /* Update display occasionally */
            if !(flag as libc::c_int != 0 ||
                     auto_round < last_round + 100 as libc::c_int) {
                continue ;
            }
            /* Dump data */
            birth_put_stats();
            /* Dump round */
            put_str(format(b"%6ld\x00" as *const u8 as *const libc::c_char,
                           auto_round) as cptr, 9 as libc::c_int,
                    73 as libc::c_int);
            /* Make sure they see everything */
            Term_fresh();
            /* Do not wait for a key */
            inkey_scan = 1 as libc::c_int as bool_;
            /* Check for a keypress */
            if inkey() != 0 { break ; }
        }
        /* Flush input */
        flush();
        /* ** Display ***/
        /* Mode */
        mode = 0 as libc::c_int;
        /* Roll for base hitpoints */
        get_extra();
        /* Roll for age/height/weight */
        get_ahw();
        /* Roll for social class */
        get_history();
        /* Roll for gold */
        get_money();
        /* ** Generate ***/
        process_hooks(68 as libc::c_int,
                      b"()\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char);
        /* Hack -- get a chaos patron even if you are not a chaos warrior */
        (*p_ptr).chaos_patron =
            (Rand_div(16 as libc::c_int) + 1 as libc::c_int -
                 1 as libc::c_int) as s16b;
        loop 
             /* Input loop */
             /* Calculate the bonuses and hitpoints */
             {
            (*p_ptr).update =
                ((*p_ptr).update as libc::c_long |
                     (0x1 as libc::c_long | 0x10 as libc::c_long |
                          0x20 as libc::c_long | 0x4 as libc::c_long)) as
                    u32b;
            /* Update stuff */
            update_stuff();
            /* Fully healed */
            (*p_ptr).chp = (*p_ptr).mhp;
            /* Fully rested */
            (*p_ptr).csp = (*p_ptr).msp;
            /* Display the player */
            display_player(mode);
            /* Prepare a prompt (must squeeze everything in) */
            Term_gotoxy(2 as libc::c_int, 23 as libc::c_int);
            Term_addch(1 as libc::c_int as byte_hack, b1);
            Term_addstr(-(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                        b"\'r\' to reroll\x00" as *const u8 as
                            *const libc::c_char);
            if prev != 0 {
                Term_addstr(-(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            b", \'p\' for prev\x00" as *const u8 as
                                *const libc::c_char);
            }
            if mode != 0 {
                Term_addstr(-(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            b", \'h\' for Misc.\x00" as *const u8 as
                                *const libc::c_char);
            } else {
                Term_addstr(-(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            b", \'h\' for History\x00" as *const u8 as
                                *const libc::c_char);
            }
            Term_addstr(-(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                        b", or ESC to accept\x00" as *const u8 as
                            *const libc::c_char);
            Term_addch(1 as libc::c_int as byte_hack, b2);
            /* Prompt and get a command */
            c = inkey();
            /* Quit */
            if c as libc::c_int == 'Q' as i32 { quit(0 as cptr); }
            /* Start over */
            if c as libc::c_int == 'S' as i32 {
                return 0 as libc::c_int as bool_
            }
            /* Escape accepts the roll */
            if c as libc::c_int == '\u{1b}' as i32 { break ; }
            /* Reroll this character */
            if c as libc::c_int == ' ' as i32 ||
                   c as libc::c_int == 'r' as i32 {
                break ;
            }
            /* Previous character */
            if prev as libc::c_int != 0 && c as libc::c_int == 'p' as i32 {
                load_prev_data(1 as libc::c_int as bool_);
            } else if c as libc::c_int == 'H' as i32 ||
                          c as libc::c_int == 'h' as i32 {
                mode =
                    if mode != 0 as libc::c_int {
                        0 as libc::c_int
                    } else { 1 as libc::c_int }
            } else if c as libc::c_int == '?' as i32 {
                do_cmd_help();
            } else {
                /* Toggle the display */
                /* Help */
                /* Warning */
                bell();
            }
        }
        /* Are we done? */
        if c as libc::c_int == '\u{1b}' as i32 { break ; }
        /* Save this for the "previous" character */
        save_prev_data();
        /* Note that a previous roll exists */
        prev = 1 as libc::c_int as bool_
    }
    /* Clear prompt */
    clear_from(23 as libc::c_int);
    return 1 as libc::c_int as bool_;
}
/*
 * Helper function for 'player_birth()'
 *
 * The delay may be reduced, but is recommended to keep players
 * from continuously rolling up characters, which can be VERY
 * expensive CPU wise.  And it cuts down on player stupidity.
 */
unsafe extern "C" fn player_birth_aux() -> bool_ {
    let mut c: libc::c_char = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut old_history: [[libc::c_char; 60]; 4] = [[0; 60]; 4];
    /* Ask */
    if player_birth_aux_ask() == 0 { return 0 as libc::c_int as bool_ }
    i = 1 as libc::c_int;
    while i < max_s_idx as libc::c_int {
        (*s_info.offset(i as isize)).dev = 0 as libc::c_int as bool_;
        i += 1
    }
    i = 1 as libc::c_int;
    while i < max_s_idx as libc::c_int {
        let mut value: s32b = 0 as libc::c_int;
        let mut mod_0: s32b = 0 as libc::c_int;
        compute_skills(&mut value, &mut mod_0, i);
        init_skill(value, mod_0, i);
        /* Develop only revelant branches */
        if (*s_info.offset(i as isize)).value != 0 ||
               (*s_info.offset(i as isize)).mod_0 != 0 {
            let mut z: libc::c_int =
                (*s_info.offset(i as isize)).father as libc::c_int;
            while z != -(1 as libc::c_int) {
                (*s_info.offset(z as isize)).dev = 1 as libc::c_int as bool_;
                z = (*s_info.offset(z as isize)).father as libc::c_int;
                if z == 0 as libc::c_int { break ; }
            }
        }
        i += 1
    }
    if do_quick_start != 0 {
        load_prev_data(0 as libc::c_int as bool_);
        /* Roll for base hitpoints */
        get_extra();
        /* Calculate the bonuses and hitpoints */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x1 as libc::c_long | 0x10 as libc::c_long |
                      0x20 as libc::c_long | 0x4 as libc::c_long)) as u32b;
        /* Update stuff */
        update_stuff();
        /* Fully healed */
        (*p_ptr).chp = (*p_ptr).mhp;
        /* Fully rested */
        (*p_ptr).csp = (*p_ptr).msp
    } else {
        /* Point based */
        if point_based != 0 {
            if player_birth_aux_point() == 0 {
                return 0 as libc::c_int as bool_
            }
        } else if player_birth_aux_auto() == 0 {
            return 0 as libc::c_int as bool_
        }
        /* Auto-roll */
        /* Edit character background */
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            strnfmt(old_history[i as usize].as_mut_ptr(),
                    60 as libc::c_int as uint_hack,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    history[i as usize].as_mut_ptr());
            i += 1
        }
        /* Turn 0 to space */
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            j = 0 as libc::c_int;
            while history[i as usize][j as usize] != 0 {
                /* loop */
                j += 1
            }
            while j < 59 as libc::c_int {
                history[i as usize][j as usize] = ' ' as i32 as libc::c_char;
                j += 1
            }
            i += 1
        }
        display_player(1 as libc::c_int);
        c_put_str(13 as libc::c_int as byte_hack,
                  b"(Character Background - Edit Mode)\x00" as *const u8 as
                      *const libc::c_char, 15 as libc::c_int,
                  20 as libc::c_int);
        loop  {
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                put_str(history[i as usize].as_mut_ptr() as cptr,
                        i + 16 as libc::c_int, 10 as libc::c_int);
                i += 1
            }
            c_put_str(14 as libc::c_int as byte_hack,
                      format(b"%c\x00" as *const u8 as *const libc::c_char,
                             history[y as usize][x as usize] as libc::c_int)
                          as cptr, y + 16 as libc::c_int,
                      x + 10 as libc::c_int);
            /* Place cursor just after cost of current stat */
            Term_gotoxy(x + 10 as libc::c_int, y + 16 as libc::c_int);
            c = inkey();
            if c as libc::c_int == '8' as i32 {
                y -= 1;
                if y < 0 as libc::c_int { y = 3 as libc::c_int }
            } else if c as libc::c_int == '2' as i32 {
                y += 1;
                if y > 3 as libc::c_int { y = 0 as libc::c_int }
            } else if c as libc::c_int == '6' as i32 {
                x += 1;
                if x > 59 as libc::c_int { x = 0 as libc::c_int }
            } else if c as libc::c_int == '4' as i32 {
                x -= 1;
                if x < 0 as libc::c_int { x = 59 as libc::c_int }
            } else {
                if c as libc::c_int == '\r' as i32 { break ; }
                if c as libc::c_int == '\u{1b}' as i32 {
                    i = 0 as libc::c_int;
                    while i < 4 as libc::c_int {
                        strnfmt(history[i as usize].as_mut_ptr(),
                                60 as libc::c_int as uint_hack,
                                b"%s\x00" as *const u8 as *const libc::c_char,
                                old_history[i as usize].as_mut_ptr());
                        put_str(history[i as usize].as_mut_ptr() as cptr,
                                i + 16 as libc::c_int, 10 as libc::c_int);
                        i += 1
                    }
                    break ;
                } else {
                    let fresh6 = x;
                    x = x + 1;
                    history[y as usize][fresh6 as usize] = c;
                    if x > 58 as libc::c_int {
                        x = 0 as libc::c_int;
                        y += 1;
                        if y > 3 as libc::c_int { y = 0 as libc::c_int }
                    }
                }
            }
        }
        /* ** Finish up ***/
        /* Get a name, recolor it, prepare savefile */
        get_name();
        /* Prompt for it */
        prt(b"[\'Q\' to suicide, \'S\' to start over, or ESC to continue]\x00"
                as *const u8 as *const libc::c_char, 23 as libc::c_int,
            10 as libc::c_int);
        /* Get a key */
        c = inkey();
        /* Quit */
        if c as libc::c_int == 'Q' as i32 { quit(0 as cptr); }
        /* Start over */
        if c as libc::c_int == 'S' as i32 { return 0 as libc::c_int as bool_ }
    }
    /* Save this for the next character */
    previous_char.quick_ok = 1 as libc::c_int as bool_;
    save_prev_data();
    /* Accept */
    return 1 as libc::c_int as bool_;
}
/*
 * Helper function for validate_bg().
 */
unsafe extern "C" fn validate_bg_aux(mut chart: libc::c_int,
                                     mut chart_checked: *mut bool_,
                                     mut buf: *mut libc::c_char) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    /* Assume the chart does not exist */
    let mut chart_exists: bool_ = 0 as libc::c_int as bool_;
    /* Assume the chart is not complete */
    let mut chart_complete: bool_ = 0 as libc::c_int as bool_;
    let mut bg_max: libc::c_int = max_bg_idx;
    /* No chart */
    if chart == 0 { return }
    /* Already saw this chart */
    if *chart_checked.offset(chart as isize) != 0 { return }
    /* Build a debug message */
    s = buf.offset(strlen(buf) as isize);
    /* XXX XXX XXX */
    strnfmt(s, -(1 as libc::c_int) as uint_hack,
            b"%d --> \x00" as *const u8 as *const libc::c_char, chart);
    /* Check each chart */
    i = 0 as libc::c_int;
    while i < bg_max {
        /* Require same chart */
        if !((*bg.offset(i as isize)).chart as libc::c_int != chart) {
            /* The chart exists */
            chart_exists = 1 as libc::c_int as bool_;
            /* Validate the "next" chart recursively */
            validate_bg_aux((*bg.offset(i as isize)).next as libc::c_int,
                            chart_checked, buf);
            /* Require a terminator */
            if !((*bg.offset(i as isize)).roll as libc::c_int !=
                     100 as libc::c_int) {
                /* The chart is complete */
                chart_complete = 1 as libc::c_int as bool_
            }
        }
        i += 1
    }
    /* Failed: The chart does not exist */
    if chart_exists == 0 {
        quit_fmt(b"birth.c: bg[] chart %d does not exist\n%s\x00" as *const u8
                     as *const libc::c_char, chart, buf);
    }
    /* Failed: The chart is not complete */
    if chart_complete == 0 {
        quit_fmt(b"birth.c: bg[] chart %d is not complete\x00" as *const u8 as
                     *const libc::c_char, chart);
    }
    /* Remember we saw this chart */
    *chart_checked.offset(chart as isize) = 1 as libc::c_int as bool_;
    /* Build a debug message */
    *s = 0 as libc::c_int as libc::c_char;
}
/*
 * Verify that the bg[] table is valid.
 */
unsafe extern "C" fn validate_bg() {
    let mut i: libc::c_int = 0;
    let mut race: libc::c_int = 0;
    let mut chart_checked: [bool_; 512] = [0; 512];
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    i = 0 as libc::c_int;
    while i < 512 as libc::c_int {
        chart_checked[i as usize] = 0 as libc::c_int as bool_;
        i += 1
    }
    /* Check each race */
    race = 0 as libc::c_int;
    while race < max_rp_idx as libc::c_int {
        /* Get the first chart for this race */
        let mut chart: libc::c_int =
            (*race_info.offset(race as isize)).chart as libc::c_int;
        strcpy(buf.as_mut_ptr(), b"\x00" as *const u8 as *const libc::c_char);
        /* Validate the chart recursively */
        validate_bg_aux(chart, chart_checked.as_mut_ptr(), buf.as_mut_ptr());
        race += 1
    };
}
/*
 * Initialize a random town
 */
#[no_mangle]
pub unsafe extern "C" fn init_town(mut t_idx: libc::c_int,
                                   mut level: libc::c_int) {
    let mut t_ptr: *mut town_type =
        &mut *town_info.offset(t_idx as isize) as *mut town_type;
    /* Mark it as existent */
    (*t_ptr).flags =
        ((*t_ptr).flags as libc::c_int | 0x1 as libc::c_int) as byte_hack;
    /* Mark it as not found */
    (*t_ptr).flags =
        ((*t_ptr).flags as libc::c_int & !(0x2 as libc::c_int)) as byte_hack;
    /* Generation seed for the town */
    (*t_ptr).seed =
        (Rand_div(0x10000000 as libc::c_int) + 1 as libc::c_int) as u32b;
    /* Total hack and not even used */
    (*t_ptr).numstores = 8 as libc::c_int as byte_hack;
}
/*
 * Create a new character.
 *
 * Note that we may be called with "junk" leftover in the various
 * fields, so we must be sure to clear them first.
 */
#[no_mangle]
pub unsafe extern "C" fn player_birth() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut rtown: libc::c_int = 20 as libc::c_int;
    /* Validate the bg[] table */
    validate_bg();
    loop 
         /* Create a new character */
         /* Wipe the player */
         {
        player_wipe();
        /* Roll up a new character */
        if player_birth_aux() != 0 { break ; }
    }
    /* Finish skills */
    (*p_ptr).skill_points = 0 as libc::c_int as s16b;
    (*p_ptr).skill_last_level = 1 as libc::c_int as s16b;
    recalc_skills(0 as libc::c_int as bool_);
    /* grab level 1 abilities */
    i = 0 as libc::c_int;
    while i < max_ab_idx as libc::c_int {
        (*ab_info.offset(i as isize)).acquired = 0 as libc::c_int as bool_;
        i += 1
    }
    apply_level_abilities(1 as libc::c_int);
    /* Complete the god */
    i = (*p_ptr).pgod as libc::c_int;
    (*p_ptr).pgod = 0 as libc::c_int as byte_hack;
    follow_god(i, 1 as libc::c_int as bool_);
    /* Select the default melee type */
    select_default_melee();
    /* Make a note file if that option is set */
    if take_notes != 0 { add_note_type(1 as libc::c_int); }
    /* Note player birth in the message recall */
    message_add(1 as libc::c_int as byte_hack,
                b" \x00" as *const u8 as *const libc::c_char,
                14 as libc::c_int as byte_hack);
    message_add(1 as libc::c_int as byte_hack,
                b"  \x00" as *const u8 as *const libc::c_char,
                14 as libc::c_int as byte_hack);
    message_add(1 as libc::c_int as byte_hack,
                b"====================\x00" as *const u8 as
                    *const libc::c_char, 14 as libc::c_int as byte_hack);
    message_add(1 as libc::c_int as byte_hack,
                b"  \x00" as *const u8 as *const libc::c_char,
                14 as libc::c_int as byte_hack);
    message_add(1 as libc::c_int as byte_hack,
                b" \x00" as *const u8 as *const libc::c_char,
                14 as libc::c_int as byte_hack);
    /* Hack -- outfit the player */
    player_outfit();
    /* Initialize random towns in the dungeons */
    i = 0 as libc::c_int;
    while i < max_d_idx as libc::c_int {
        let mut d_ptr: *mut dungeon_info_type =
            &mut *d_info.offset(i as isize) as *mut dungeon_info_type;
        let mut num: libc::c_int = 0 as libc::c_int;
        let mut z: libc::c_int = 0;
        (*d_ptr).t_num = 0 as libc::c_int as s16b;
        z = 0 as libc::c_int;
        while z < 4 as libc::c_int {
            (*d_ptr).t_idx[z as usize] = 0 as libc::c_int as s16b;
            (*d_ptr).t_level[z as usize] = 0 as libc::c_int as s16b;
            z += 1
        }
        if !((*d_ptr).flags1 as libc::c_long & 0x1000000 as libc::c_long == 0)
           {
            /* Can we add a town ? */
            while Rand_div(100 as libc::c_int) <
                      50 as libc::c_int - num * 10 as libc::c_int {
                let mut lev: libc::c_int = 0;
                (*d_ptr).t_idx[num as usize] = rtown as s16b;
                rtown += 1;
                loop  {
                    let mut j_0: libc::c_int = 0;
                    let mut ok: bool_ = 1 as libc::c_int as bool_;
                    lev =
                        (*d_ptr).mindepth as libc::c_int +
                            Rand_div(1 as libc::c_int +
                                         ((*d_ptr).maxdepth as libc::c_int -
                                              1 as libc::c_int) -
                                         (*d_ptr).mindepth as libc::c_int);
                    /* Be sure it wasnt already used */
                    j_0 = 0 as libc::c_int;
                    while j_0 < num {
                        if (*d_ptr).t_level[j_0 as usize] as libc::c_int ==
                               lev {
                            ok = 0 as libc::c_int as bool_
                        }
                        j_0 += 1
                    }
                    /* Ok found one */
                    if ok != 0 { break ; }
                }
                (*d_ptr).t_level[num as usize] = lev as s16b;
                if wizard != 0 {
                    message_add(1 as libc::c_int as byte_hack,
                                format(b"Random dungeon town: d_idx:%d, lev:%d\x00"
                                           as *const u8 as
                                           *const libc::c_char, i, lev) as
                                    cptr, 1 as libc::c_int as byte_hack);
                }
                /* Create the town */
                init_town((*d_ptr).t_idx[num as usize] as libc::c_int,
                          (*d_ptr).t_level[num as usize] as libc::c_int);
                num += 1;
                /* No free slots left */
                if num >= 4 as libc::c_int { break ; }
            }
            (*d_ptr).t_num = num as s16b
        }
        i += 1
    }
    /* Init the towns */
    i = 1 as libc::c_int;
    while i < max_towns as libc::c_int {
        /* Not destroyed ! yet .. ;) */
        (*town_info.offset(i as isize)).destroyed = 0 as libc::c_int as bool_;
        /* Ignore non-existent towns */
        if !((*town_info.offset(i as isize)).flags as libc::c_int &
                 0x1 as libc::c_int == 0) {
            create_stores_stock(i);
            /* Init the stores */
            j = 0 as libc::c_int;
            while j < max_st_idx as libc::c_int {
                /* Initialize */
                store_init(i, j);
                j += 1
            }
        }
        i += 1
    }
    /* Init wilderness seeds */
    i = 0 as libc::c_int;
    while i < max_wild_x as libc::c_int {
        j = 0 as libc::c_int;
        while j < max_wild_y as libc::c_int {
            (*(*wild_map.offset(j as isize)).offset(i as isize)).seed =
                Rand_div(0x10000000 as libc::c_int) as u32b;
            (*(*wild_map.offset(j as isize)).offset(i as isize)).entrance =
                0 as libc::c_int as u16b;
            (*(*wild_map.offset(j as isize)).offset(i as isize)).known =
                0 as libc::c_int as bool_;
            j += 1
        }
        i += 1
    }
    /* Select bounty monsters. */
    select_bounties();
}
#[no_mangle]
pub static mut savefile_module: [[libc::c_char; 80]; 46] = [[0; 80]; 46];
#[no_mangle]
pub static mut savefile_names: [[libc::c_char; 30]; 46] = [[0; 30]; 46];
#[no_mangle]
pub static mut savefile_desc: [[libc::c_char; 80]; 46] = [[0; 80]; 46];
#[no_mangle]
pub static mut savefile_alive: [bool_; 46] = [0; 46];
#[no_mangle]
pub static mut savefile_idx: [libc::c_int; 46] = [0; 46];
/*
 * Grab all the names from an index
 */
#[no_mangle]
pub unsafe extern "C" fn load_savefile_names() -> libc::c_int {
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: [libc::c_char; 50] = [0; 50];
    let mut player_base_save: [libc::c_char; 32] = [0; 32];
    let mut max: libc::c_int = 0 as libc::c_int;
    let mut fd: libc::c_int = 0;
    /* Build the filename */
    strcpy(tmp.as_mut_ptr(),
           b"global.svg\x00" as *const u8 as *const libc::c_char);
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_SAVE,
               tmp.as_mut_ptr() as cptr);
    /* File type is "TEXT" */
    /* Read the file */
    fff =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Failure */
    if fff.is_null() { return 0 as libc::c_int }
    /* Save the current 'player_base' */
    strncpy(player_base_save.as_mut_ptr(), player_base.as_mut_ptr(),
            32 as libc::c_int as libc::c_ulong);
    /*
	 * Parse, use '@' intead of ':' as a separator because it cannot exists
	 * in savefiles
	 */
    while 0 as libc::c_int ==
              my_fgets(fff, buf.as_mut_ptr(),
                       1024 as libc::c_int as huge_hack) {
        let mut i: libc::c_int = 0 as libc::c_int;
        let mut start: libc::c_int = 0;
        let mut count: libc::c_int = 0;
        /* Check for pre-ToME 2.1.2 file */
        count = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while buf[i as usize] as libc::c_int != 0 &&
                  buf[i as usize] as libc::c_int != '\n' as i32 {
            if buf[i as usize] as libc::c_int == '@' as i32 { count += 1 }
            i += 1
        }
        /* Check module if a current svg file */
        start = 0 as libc::c_int;
        i = 0 as libc::c_int;
        if count > 1 as libc::c_int {
            while buf[i as usize] as libc::c_int != '@' as i32 {
                savefile_module[max as usize][(i - start) as usize] =
                    buf[i as usize];
                i += 1
            }
            savefile_module[max as usize][i as usize] =
                '\u{0}' as i32 as libc::c_char;
            i += 1
        } else {
            /* Default to ToME for old files */
            savefile_module[max as usize][0 as libc::c_int as usize] =
                'T' as i32 as libc::c_char;
            savefile_module[max as usize][1 as libc::c_int as usize] =
                'o' as i32 as libc::c_char;
            savefile_module[max as usize][2 as libc::c_int as usize] =
                'M' as i32 as libc::c_char;
            savefile_module[max as usize][3 as libc::c_int as usize] =
                'E' as i32 as libc::c_char;
            savefile_module[max as usize][4 as libc::c_int as usize] =
                '\u{0}' as i32 as libc::c_char
        }
        if buf[i as usize] as libc::c_int == '0' as i32 {
            savefile_alive[max as usize] = 0 as libc::c_int as bool_
        } else if buf[i as usize] as libc::c_int == '1' as i32 {
            savefile_alive[max as usize] = 1 as libc::c_int as bool_
        }
        i += 1;
        start = i;
        while buf[i as usize] as libc::c_int != '@' as i32 {
            savefile_names[max as usize][(i - start) as usize] =
                buf[i as usize];
            i += 1
        }
        savefile_names[max as usize][(i - start) as usize] =
            '\u{0}' as i32 as libc::c_char;
        i += 1;
        strcpy(savefile_desc[max as usize].as_mut_ptr(),
               buf.as_mut_ptr().offset(i as isize));
        /* Build platform-dependent savefile name */
        strncpy(player_base.as_mut_ptr(),
                savefile_names[max as usize].as_mut_ptr(),
                32 as libc::c_int as libc::c_ulong);
        process_player_name(1 as libc::c_int as bool_);
        /* File type is 'SAVE' */
        /* Try to open the savefile */
        fd = fd_open(savefile.as_mut_ptr() as cptr, 0 as libc::c_int);
        if fd >= 0 as libc::c_int { fd_close(fd); max += 1 }
    }
    my_fclose(fff);
    /* Still existing ? */
    /* Restore the values of 'player_base' and 'savefile' */
    strncpy(player_base.as_mut_ptr(), player_base_save.as_mut_ptr(),
            32 as libc::c_int as libc::c_ulong);
    process_player_name(1 as libc::c_int as bool_);
    return max;
}
/*
 * Save all the names from an index
 */
#[no_mangle]
pub unsafe extern "C" fn save_savefile_names() {
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: [libc::c_char; 50] = [0; 50];
    let mut max: libc::c_int = load_savefile_names();
    let mut i: libc::c_int = 0;
    /* Build the filename */
    strcpy(tmp.as_mut_ptr(),
           b"global.svg\x00" as *const u8 as *const libc::c_char);
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_SAVE,
               tmp.as_mut_ptr() as cptr);
    /* File type is "TEXT" */
    /* Read the file */
    fff =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    /* Failure */
    if fff.is_null() { return }
    /*
	 * Save, use '@' intead of ':' as a separator because it cannot exists
	 * in savefiles
	 */
    fprintf(fff,
            b"%s@%c%s@%s, the %s %s is %s\n\x00" as *const u8 as
                *const libc::c_char, game_module,
            if death as libc::c_int != 0 { '0' as i32 } else { '1' as i32 },
            player_base.as_mut_ptr(), player_name.as_mut_ptr(),
            get_player_race_name((*p_ptr).prace as libc::c_int,
                                 (*p_ptr).pracem as libc::c_int),
            c_name.offset((*spp_ptr).title as isize),
            if death == 0 {
                b"alive\x00" as *const u8 as *const libc::c_char
            } else { b"dead\x00" as *const u8 as *const libc::c_char });
    i = 0 as libc::c_int;
    while i < max {
        if !(strcmp(savefile_names[i as usize].as_mut_ptr(),
                    player_base.as_mut_ptr()) == 0) {
            fprintf(fff,
                    b"%s@%c%s@%s\n\x00" as *const u8 as *const libc::c_char,
                    savefile_module[i as usize].as_mut_ptr(),
                    if savefile_alive[i as usize] as libc::c_int != 0 {
                        '1' as i32
                    } else { '0' as i32 },
                    savefile_names[i as usize].as_mut_ptr(),
                    savefile_desc[i as usize].as_mut_ptr());
        }
        i += 1
    }
    my_fclose(fff);
}
unsafe extern "C" fn dump_savefiles(mut sel: libc::c_int,
                                    mut max: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 40] = [0; 40];
    let mut pre: libc::c_char = ' ' as i32 as libc::c_char;
    let mut post: libc::c_char = ')' as i32 as libc::c_char;
    let mut ind: libc::c_char = 0;
    i = 0 as libc::c_int;
    while i < max {
        ind = (i % 26 as libc::c_int + 'a' as i32) as libc::c_char;
        if i >= 26 as libc::c_int {
            ind = toupper(ind as libc::c_int) as libc::c_char
        }
        if sel == i {
            pre = '[' as i32 as libc::c_char;
            post = ']' as i32 as libc::c_char
        } else {
            pre = ' ' as i32 as libc::c_char;
            post = ')' as i32 as libc::c_char
        }
        if i == 0 as libc::c_int {
            strnfmt(buf.as_mut_ptr(), 40 as libc::c_int as uint_hack,
                    b"%c%c%c New Character\x00" as *const u8 as
                        *const libc::c_char, pre as libc::c_int,
                    ind as libc::c_int, post as libc::c_int);
        } else if i == 1 as libc::c_int {
            strnfmt(buf.as_mut_ptr(), 40 as libc::c_int as uint_hack,
                    b"%c%c%c Load Savefile\x00" as *const u8 as
                        *const libc::c_char, pre as libc::c_int,
                    ind as libc::c_int, post as libc::c_int);
        } else {
            strnfmt(buf.as_mut_ptr(), 40 as libc::c_int as uint_hack,
                    b"%c%c%c %s\x00" as *const u8 as *const libc::c_char,
                    pre as libc::c_int, ind as libc::c_int,
                    post as libc::c_int,
                    savefile_names[savefile_idx[(i - 2 as libc::c_int) as
                                                    usize] as
                                       usize].as_mut_ptr());
        }
        if sel == i {
            if i >= 2 as libc::c_int {
                if savefile_alive[(i - 2 as libc::c_int) as usize] != 0 {
                    c_put_str(13 as libc::c_int as byte_hack,
                              savefile_desc[savefile_idx[(i -
                                                              2 as
                                                                  libc::c_int)
                                                             as usize] as
                                                usize].as_mut_ptr() as cptr,
                              5 as libc::c_int, 0 as libc::c_int);
                } else {
                    c_put_str(12 as libc::c_int as byte_hack,
                              savefile_desc[savefile_idx[(i -
                                                              2 as
                                                                  libc::c_int)
                                                             as usize] as
                                                usize].as_mut_ptr() as cptr,
                              5 as libc::c_int, 0 as libc::c_int);
                }
            } else if i == 1 as libc::c_int {
                c_put_str(11 as libc::c_int as byte_hack,
                          b"Load an existing savefile that is not in the list\x00"
                              as *const u8 as *const libc::c_char,
                          5 as libc::c_int, 0 as libc::c_int);
            } else {
                c_put_str(11 as libc::c_int as byte_hack,
                          b"Create a new character\x00" as *const u8 as
                              *const libc::c_char, 5 as libc::c_int,
                          0 as libc::c_int);
            }
            c_put_str(14 as libc::c_int as byte_hack,
                      buf.as_mut_ptr() as cptr,
                      6 as libc::c_int + i / 4 as libc::c_int,
                      20 as libc::c_int * (i % 4 as libc::c_int));
        } else {
            put_str(buf.as_mut_ptr() as cptr,
                    6 as libc::c_int + i / 4 as libc::c_int,
                    20 as libc::c_int * (i % 4 as libc::c_int));
        }
        i += 1
    };
}
/* Asks for new game or load game */
#[no_mangle]
pub static mut no_begin_screen: bool_ = 0 as libc::c_int as bool_;
#[no_mangle]
pub unsafe extern "C" fn begin_screen() -> bool_ {
    let mut m: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut sel: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    loop 
         /* Reload, gods I hate using goto .. */
         {
        sel = 0 as libc::c_int;
        /* Grab the savefiles */
        max = load_savefile_names();
        /* Get only the usable savefiles */
        k = 0 as libc::c_int;
        m = 0 as libc::c_int;
        while k < max {
            let mut can_use: s32b = 0;
            call_lua(b"module_savefile_loadable\x00" as *const u8 as
                         *const libc::c_char,
                     b"(s)\x00" as *const u8 as *const libc::c_char,
                     b"d\x00" as *const u8 as *const libc::c_char,
                     savefile_module[k as usize].as_mut_ptr(),
                     &mut can_use as *mut s32b);
            if can_use != 0 {
                let fresh7 = m;
                m = m + 1;
                savefile_idx[fresh7 as usize] = k
            }
            k += 1
        }
        max = m + 2 as libc::c_int;
        if max > 2 as libc::c_int { sel = 2 as libc::c_int }
        loop  {
            /* Clear screen */
            Term_clear();
            /* Let the user choose */
            c_put_str(11 as libc::c_int as byte_hack,
                      format(b"Welcome to %s!  To play you will need a character.\x00"
                                 as *const u8 as *const libc::c_char,
                             game_module) as cptr, 1 as libc::c_int,
                      10 as libc::c_int);
            put_str(b"Press 8/2/4/6 to move, Return to select, Backspace to delete a savefile.\x00"
                        as *const u8 as *const libc::c_char, 3 as libc::c_int,
                    3 as libc::c_int);
            put_str(b"and Esc to quit.\x00" as *const u8 as
                        *const libc::c_char, 4 as libc::c_int,
                    32 as libc::c_int);
            dump_savefiles(sel, max);
            k = inkey() as libc::c_int;
            if k == '\u{1b}' as i32 { quit(0 as cptr); }
            if k == '6' as i32 {
                sel += 1;
                if sel >= max { sel = 0 as libc::c_int }
            } else if k == '4' as i32 {
                sel -= 1;
                if sel < 0 as libc::c_int { sel = max - 1 as libc::c_int }
            } else if k == '2' as i32 {
                sel += 4 as libc::c_int;
                if sel >= max { sel = sel % max }
            } else if k == '8' as i32 {
                sel -= 4 as libc::c_int;
                if sel < 0 as libc::c_int {
                    sel = (sel + max - 1 as libc::c_int) % max
                }
            } else {
                if k == '\r' as i32 {
                    if sel < 26 as libc::c_int {
                        k = sel + 'a' as i32
                    } else { k = toupper(sel + 'a' as i32) }
                } else if (k == 0x7f as libc::c_int || k == '\u{8}' as i32) &&
                              sel >= 2 as libc::c_int {
                    let mut player_base_save: [libc::c_char; 32] = [0; 32];
                    if get_check(format(b"Really delete \'%s\'?\x00" as
                                            *const u8 as *const libc::c_char,
                                        savefile_names[savefile_idx[(sel -
                                                                         2 as
                                                                             libc::c_int)
                                                                        as
                                                                        usize]
                                                           as
                                                           usize].as_mut_ptr())
                                     as cptr) == 0 {
                        continue ;
                    }
                    /* Save current 'player_base' */
                    strncpy(player_base_save.as_mut_ptr(),
                            player_base.as_mut_ptr(),
                            32 as libc::c_int as libc::c_ulong);
                    /* Build platform-dependent save file name */
                    strncpy(player_base.as_mut_ptr(),
                            savefile_names[savefile_idx[(sel -
                                                             2 as libc::c_int)
                                                            as usize] as
                                               usize].as_mut_ptr(),
                            32 as libc::c_int as libc::c_ulong);
                    process_player_name(1 as libc::c_int as bool_);
                    /* Remove the savefile */
                    fd_kill(savefile.as_mut_ptr() as cptr);
                    /* Restore 'player_base' and 'savefile' */
                    strncpy(player_base.as_mut_ptr(),
                            player_base_save.as_mut_ptr(),
                            32 as libc::c_int as libc::c_ulong);
                    process_player_name(1 as libc::c_int as bool_);
                    break ;
                }
                if k == 'a' as i32 {
                    /* Display prompt */
                    prt(b"Enter the name of the savefile that will hold this character: \x00"
                            as *const u8 as *const libc::c_char,
                        23 as libc::c_int, 0 as libc::c_int);
                    /* Ask the user for a string */
                    if askfor_aux(player_base.as_mut_ptr(), 15 as libc::c_int)
                           == 0 {
                        continue ;
                    }
                    /* Process the player name */
                    process_player_name(1 as libc::c_int as bool_);
                    return 1 as libc::c_int as bool_
                } else if k == 'b' as i32 {
                    /* Display prompt */
                    prt(b"Enter the name of a savefile: \x00" as *const u8 as
                            *const libc::c_char, 23 as libc::c_int,
                        0 as libc::c_int);
                    /* Ask the user for a string */
                    if askfor_aux(player_base.as_mut_ptr(), 15 as libc::c_int)
                           == 0 {
                        continue ;
                    }
                    /* Process the player name */
                    process_player_name(1 as libc::c_int as bool_);
                    return 0 as libc::c_int as bool_
                } else {
                    let mut x: libc::c_int = 0;
                    if *(*__ctype_b_loc()).offset(k as isize) as libc::c_int &
                           _ISlower as libc::c_int as libc::c_ushort as
                               libc::c_int != 0 {
                        x = k - 'a' as i32
                    } else { x = tolower(k) - 'a' as i32 + 26 as libc::c_int }
                    if x < 2 as libc::c_int || x >= max { continue ; }
                    strnfmt(player_base.as_mut_ptr(),
                            32 as libc::c_int as uint_hack,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            savefile_names[savefile_idx[(x - 2 as libc::c_int)
                                                            as usize] as
                                               usize].as_mut_ptr());
                    /* Process the player name */
                    process_player_name(1 as libc::c_int as bool_);
                    return 0 as libc::c_int as bool_
                }
            }
        }
    };
}
