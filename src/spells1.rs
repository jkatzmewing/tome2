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
    static mut adj_con_fix: [byte_hack; 0];
    #[no_mangle]
    static mut command_new: s16b;
    #[no_mangle]
    static mut death: bool_;
    #[no_mangle]
    static mut cur_hgt: s16b;
    #[no_mangle]
    static mut cur_wid: s16b;
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut total_winner: u16b;
    #[no_mangle]
    static mut fresh_before: bool_;
    #[no_mangle]
    static mut alert_hitpoint: bool_;
    #[no_mangle]
    static mut last_words: bool_;
    #[no_mangle]
    static mut hitpoint_warn: byte_hack;
    #[no_mangle]
    static mut delay_factor: byte_hack;
    #[no_mangle]
    static mut panel_row_min: s16b;
    #[no_mangle]
    static mut panel_row_max: s16b;
    #[no_mangle]
    static mut panel_col_min: s16b;
    #[no_mangle]
    static mut panel_col_max: s16b;
    #[no_mangle]
    static mut health_who: s16b;
    #[no_mangle]
    static mut monster_race_idx: s16b;
    #[no_mangle]
    static mut died_from: [libc::c_char; 80];
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut o_list: *mut object_type;
    #[no_mangle]
    static mut m_list: *mut monster_type;
    #[no_mangle]
    static mut misc_to_attr: [byte_hack; 256];
    #[no_mangle]
    static mut misc_to_char: [libc::c_char; 256];
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut f_info: *mut feature_type;
    #[no_mangle]
    static mut f_name: *mut libc::c_char;
    #[no_mangle]
    static mut k_info: *mut object_kind;
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
    static mut ANGBAND_GRAF: cptr;
    #[no_mangle]
    static mut carried_monster_hit: bool_;
    #[no_mangle]
    static mut random_spells: [random_spell; 100];
    #[no_mangle]
    static mut spell_num: s16b;
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut max_dlv: *mut s16b;
    #[no_mangle]
    static mut dungeon_flags2: u32b;
    #[no_mangle]
    static mut player_char_health: bool_;
    #[no_mangle]
    static mut school_spells: *mut spell_type;
    #[no_mangle]
    static mut project_time: libc::c_int;
    #[no_mangle]
    static mut project_time_effect: s32b;
    #[no_mangle]
    static mut last_teleportation_y: s16b;
    #[no_mangle]
    static mut last_teleportation_x: s16b;
    #[no_mangle]
    static mut DUNGEON_DEATH: s32b;
    #[no_mangle]
    static mut deity_info: *mut deity_type;
    #[no_mangle]
    static mut process_hooks_return: [hook_return; 20];
    #[no_mangle]
    fn process_hooks_ret(h_idx: libc::c_int, ret: *mut libc::c_char,
                         fmt: *mut libc::c_char, _: ...) -> bool_;
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn distance(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int,
                x2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn los(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int, x2: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn cave_valid_bold(y: libc::c_int, x: libc::c_int) -> bool_;
    #[no_mangle]
    fn move_cursor_relative(row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn print_rel(c: libc::c_char, a: byte_hack, y: libc::c_int,
                 x: libc::c_int);
    #[no_mangle]
    fn note_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn lite_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn cave_set_feat(y: libc::c_int, x: libc::c_int, feat: libc::c_int);
    #[no_mangle]
    fn place_floor_convert_glass(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn scatter(yp: *mut libc::c_int, xp: *mut libc::c_int, y: libc::c_int,
               x: libc::c_int, d: libc::c_int);
    #[no_mangle]
    fn health_track(m_idx: libc::c_int);
    #[no_mangle]
    fn monster_race_track(r_idx: libc::c_int, ego: libc::c_int);
    #[no_mangle]
    fn disturb(stop_search: libc::c_int, flush_output: libc::c_int);
    #[no_mangle]
    fn is_quest(level: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn new_effect(type_0: libc::c_int, dam: libc::c_int, time: libc::c_int,
                  cy: libc::c_int, cx: libc::c_int, rad: libc::c_int,
                  flags: s32b) -> libc::c_int;
    #[no_mangle]
    fn py_attack(y: libc::c_int, x: libc::c_int, max_blow: libc::c_int);
    #[no_mangle]
    fn do_cmd_html_dump();
    #[no_mangle]
    fn streq(s: cptr, t: cptr) -> bool_;
    #[no_mangle]
    fn prefix(s: cptr, t: cptr) -> bool_;
    #[no_mangle]
    fn strnfmt(buf: *mut libc::c_char, max: uint_hack, fmt: cptr, _: ...)
     -> uint_hack;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn damroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn maxroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn Term_xtra(n: libc::c_int, v: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_fresh() -> errr;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn do_poly_self();
    #[no_mangle]
    fn symbiote_name(capitalize: bool_) -> cptr;
    #[no_mangle]
    fn autosave_checkpoint();
    #[no_mangle]
    fn get_rnd_line(file_name: *mut libc::c_char, output: *mut libc::c_char)
     -> errr;
    #[no_mangle]
    fn mon_take_hit_mon(s_idx: libc::c_int, m_idx: libc::c_int,
                        dam: libc::c_int, fear: *mut bool_, note: cptr)
     -> bool_;
    #[no_mangle]
    fn race_info_idx(r_idx: libc::c_int, ego: libc::c_int)
     -> *mut monster_race;
    #[no_mangle]
    fn delete_monster_idx(i: libc::c_int);
    #[no_mangle]
    fn delete_monster(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn get_mon_num(level: libc::c_int) -> s16b;
    #[no_mangle]
    fn monster_desc(desc: *mut libc::c_char, m_ptr: *mut monster_type,
                    mode: libc::c_int);
    #[no_mangle]
    fn update_mon(m_idx: libc::c_int, full: bool_);
    #[no_mangle]
    fn place_monster_aux(y: libc::c_int, x: libc::c_int, r_idx: libc::c_int,
                         slp: bool_, grp: bool_, status: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn monster_swap(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int,
                    x2: libc::c_int);
    #[no_mangle]
    fn multiply_monster(m_idx: libc::c_int, charm: bool_, clone: bool_)
     -> bool_;
    #[no_mangle]
    fn message_pain(m_idx: libc::c_int, dam: libc::c_int);
    #[no_mangle]
    fn summon_specific_friendly(y1: libc::c_int, x1: libc::c_int,
                                lev: libc::c_int, type_0: libc::c_int,
                                Group_ok: bool_) -> bool_;
    #[no_mangle]
    fn place_monster_one(y: libc::c_int, x: libc::c_int, r_idx: libc::c_int,
                         ego: libc::c_int, slp: bool_, status: libc::c_int)
     -> s16b;
    #[no_mangle]
    fn can_create_companion() -> bool_;
    #[no_mangle]
    fn change_side(m_ptr: *mut monster_type) -> bool_;
    #[no_mangle]
    fn is_friend(m_ptr: *mut monster_type) -> libc::c_int;
    #[no_mangle]
    fn inc_stack_size_ex(item: libc::c_int, delta: libc::c_int,
                         opt: optimize_flag, desc: describe_flag);
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn index_to_label(i: libc::c_int) -> libc::c_char;
    #[no_mangle]
    fn delete_object_idx(o_idx: libc::c_int);
    #[no_mangle]
    fn delete_object(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn object_known(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_aware(o_ptr: *mut object_type);
    #[no_mangle]
    fn lookup_kind(tval: libc::c_int, sval: libc::c_int) -> s16b;
    #[no_mangle]
    fn object_prep(o_ptr: *mut object_type, k_idx: libc::c_int);
    #[no_mangle]
    fn place_object(y: libc::c_int, x: libc::c_int, good: bool_, great: bool_,
                    where_0: libc::c_int);
    #[no_mangle]
    fn place_gold(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn drop_near(o_ptr: *mut object_type, chance: libc::c_int, y: libc::c_int,
                 x: libc::c_int) -> s16b;
    #[no_mangle]
    fn place_trap(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn sound(num: libc::c_int);
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn handle_stuff();
    #[no_mangle]
    fn verify_panel();
    #[no_mangle]
    fn get_check(prompt: cptr) -> bool_;
    #[no_mangle]
    fn exec_lua(file: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn cmsg_print(color: byte_hack, msg: cptr);
    #[no_mangle]
    fn bell();
    #[no_mangle]
    fn do_cmd_wiz_cure_all();
    #[no_mangle]
    fn calc_hitpoints();
    #[no_mangle]
    fn cmsg_format(color: byte_hack, fmt: cptr, _: ...);
    #[no_mangle]
    fn has_ability(ab: libc::c_int) -> bool_;
    #[no_mangle]
    fn get_skill(skill: libc::c_int) -> s16b;
    #[no_mangle]
    fn swap_position(lty: libc::c_int, ltx: libc::c_int);
    #[no_mangle]
    fn set_shield(v: libc::c_int, p: libc::c_int, o: s16b, d1: s16b, d2: s16b)
     -> bool_;
    #[no_mangle]
    fn restore_level() -> bool_;
    #[no_mangle]
    fn set_paralyzed(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_tim_esp(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn self_knowledge(fff: *mut FILE);
    #[no_mangle]
    fn set_stun(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_cut(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn do_dec_stat(stat: libc::c_int, mode: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_slow(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_fast(v: libc::c_int, p: libc::c_int) -> bool_;
    #[no_mangle]
    fn hp_player(num: libc::c_int) -> bool_;
    #[no_mangle]
    fn lose_exp(amount: s32b);
    #[no_mangle]
    fn set_blind(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_confused(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_image(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_poisoned(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn mon_take_hit(m_idx: libc::c_int, dam: libc::c_int, fear: *mut bool_,
                    note: cptr) -> bool_;
    #[no_mangle]
    fn inc_piety(god: libc::c_int, amt: s32b);
    #[no_mangle]
    fn set_afraid(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn do_probe(m_idx: libc::c_int);
    #[no_mangle]
    fn test_monster_name(name: cptr) -> libc::c_int;
    #[no_mangle]
    fn squeltch_grid();
    #[no_mangle]
    fn update_stuff();
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
pub type cptr = *const libc::c_char;
pub type errr = libc::c_int;
pub type byte_hack = libc::c_uchar;
pub type bool_ = libc::c_char;
pub type sint = libc::c_int;
pub type uint_hack = libc::c_uint;
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
pub struct deity_type {
    pub name: cptr,
    pub desc: [[libc::c_char; 80]; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct random_spell {
    pub desc: [libc::c_char; 30],
    pub name: [libc::c_char; 30],
    pub mana: s16b,
    pub fail: s16b,
    pub proj_flags: u32b,
    pub GF: byte_hack,
    pub radius: byte_hack,
    pub dam_sides: byte_hack,
    pub dam_dice: byte_hack,
    pub level: byte_hack,
    pub untried: bool_,
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
pub union hook_return {
    pub num: s32b,
    pub str_0: cptr,
    pub o_ptr: *mut object_type,
    pub m_ptr: *mut monster_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spell_type {
    pub name: cptr,
    pub skill_level: byte_hack,
    pub mana: byte_hack,
    pub mana_max: byte_hack,
    pub fail: s16b,
    pub level: s16b,
}
pub type optimize_flag = libc::c_uint;
pub const NO_OPTIMIZE: optimize_flag = 1;
pub const OPTIMIZE: optimize_flag = 0;
pub type describe_flag = libc::c_uint;
pub const NO_DESCRIBE: describe_flag = 1;
pub const DESCRIBE: describe_flag = 0;
/*
 * This seems like a pretty standard "typedef"
 */
pub type inven_func
    =
    Option<unsafe extern "C" fn(_: *mut object_type) -> libc::c_int>;
/*
 * Helper function -- return a "nearby" race for polymorphing
 *
 * Note that this function is one of the more "dangerous" ones...
 */
#[no_mangle]
pub unsafe extern "C" fn poly_r_idx(mut r_idx: libc::c_int) -> s16b {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    /* Hack -- Uniques never polymorph */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        return r_idx as s16b
    }
    /* Pick a (possibly new) non-unique race */
    i = 0 as libc::c_int;
    while i < 1000 as libc::c_int {
        /* Pick a new race, using a level calculation */
        r =
            get_mon_num((dun_level as libc::c_int +
                             (*r_ptr).level as libc::c_int) / 2 as libc::c_int
                            + 5 as libc::c_int) as libc::c_int;
        /* Handle failure */
        if r == 0 { break ; }
        /* Obtain race */
        r_ptr = &mut *r_info.offset(r as isize) as *mut monster_race;
        /* Ignore unique monsters */
        if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
            i += 1
        } else {
            /* Use that index */
            r_idx = r;
            break ;
        }
    }
    /* Result */
    return r_idx as s16b;
}
/*
 * Teleport player, using a distance and a direction as a rough guide.
 *
 * This function is not at all obsessive about correctness.
 * This function allows teleporting into vaults (!)
 */
#[no_mangle]
pub unsafe extern "C" fn teleport_player_directed(mut rad: libc::c_int,
                                                  mut dir: libc::c_int) {
    let mut y: libc::c_int = (*p_ptr).py as libc::c_int;
    let mut x: libc::c_int = (*p_ptr).px as libc::c_int;
    let mut yfoo: libc::c_int = ddy[dir as usize] as libc::c_int;
    let mut xfoo: libc::c_int = ddx[dir as usize] as libc::c_int;
    let mut min: libc::c_int = rad / 4 as libc::c_int;
    let mut dis: libc::c_int = rad;
    let mut i: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut look: bool_ = 1 as libc::c_int as bool_;
    let mut y_major: bool_ = 0 as libc::c_int as bool_;
    let mut x_major: bool_ = 0 as libc::c_int as bool_;
    let mut y_neg: libc::c_int = 1 as libc::c_int;
    let mut x_neg: libc::c_int = 1 as libc::c_int;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    if xfoo == 0 as libc::c_int && yfoo == 0 as libc::c_int {
        teleport_player(rad);
        return
    }
    /* Rooted means no move */
    if (*p_ptr).tim_roots != 0 { return }
    if yfoo == 0 as libc::c_int { x_major = 1 as libc::c_int as bool_ }
    if xfoo == 0 as libc::c_int { y_major = 1 as libc::c_int as bool_ }
    if yfoo < 0 as libc::c_int { y_neg = -(1 as libc::c_int) }
    if xfoo < 0 as libc::c_int { x_neg = -(1 as libc::c_int) }
    /* Look until done */
    while look != 0 {
        /* Verify max distance */
        if dis > 200 as libc::c_int { teleport_player(rad); return }
        /* Try several locations */
        i = 0 as libc::c_int;
        while i < 500 as libc::c_int {
            loop 
                 /* Pick a (possibly illegal) location */
                 {
                if y_major != 0 {
                    y =
                        (*p_ptr).py as libc::c_int +
                            y_neg * dis / 2 as libc::c_int +
                            Rand_div(1 as libc::c_int + dis / 2 as libc::c_int
                                         + dis / 2 as libc::c_int) -
                            dis / 2 as libc::c_int
                } else {
                    y =
                        (*p_ptr).py as libc::c_int +
                            Rand_div(1 as libc::c_int + dis / 3 as libc::c_int
                                         + dis / 3 as libc::c_int) -
                            dis / 3 as libc::c_int
                }
                if x_major != 0 {
                    x =
                        (*p_ptr).px as libc::c_int +
                            x_neg * dis / 2 as libc::c_int +
                            Rand_div(1 as libc::c_int + dis / 2 as libc::c_int
                                         + dis / 2 as libc::c_int) -
                            dis / 2 as libc::c_int
                } else {
                    x =
                        (*p_ptr).px as libc::c_int +
                            Rand_div(1 as libc::c_int + dis / 3 as libc::c_int
                                         + dis / 3 as libc::c_int) -
                            dis / 3 as libc::c_int
                }
                d =
                    distance((*p_ptr).py as libc::c_int,
                             (*p_ptr).px as libc::c_int, y, x);
                if d >= min && d <= dis { break ; }
            }
            /* Ignore illegal locations */
            if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                   y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   x < cur_wid as libc::c_int - 1 as libc::c_int {
                /* Require "naked" floor space */
                if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x10 as libc::c_long != 0 &&
                       (*cave[y as usize].offset(x as isize)).feat as
                           libc::c_int != 0xaf as libc::c_int &&
                       (*cave[y as usize].offset(x as isize)).m_idx == 0 &&
                       !(y == (*p_ptr).py as libc::c_int &&
                             x == (*p_ptr).px as libc::c_int) {
                    /* This grid looks good */
                    look = 0 as libc::c_int as bool_;
                    break ;
                }
            }
            i += 1
        }
        /* Increase the maximum distance */
        dis = dis * 2 as libc::c_int;
        /* Decrease the minimum distance */
        min = min / 2 as libc::c_int
    }
    /* Sound */
    sound(9 as libc::c_int);
    /* Move player */
    teleport_player_to(y, x);
    /* Handle stuff XXX XXX XXX */
    handle_stuff();
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Hack -- enter a store if we are on one */
    if (*c_ptr).feat as libc::c_int == 0x4a as libc::c_int {
        /* Disturb */
        disturb(0 as libc::c_int, 0 as libc::c_int);
        /* Hack -- enter store */
        command_new = '_' as i32 as s16b
    };
}
/*
 * Teleport a monster, normally up to "dis" grids away.
 *
 * Attempt to move the monster at least "dis/2" grids away.
 *
 * But allow variation to prevent infinite loops.
 */
#[no_mangle]
pub unsafe extern "C" fn teleport_away(mut m_idx: libc::c_int,
                                       mut dis: libc::c_int) {
    let mut ny: libc::c_int = 0 as libc::c_int;
    let mut nx: libc::c_int = 0 as libc::c_int;
    let mut oy: libc::c_int = 0;
    let mut ox: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut min: libc::c_int = 0;
    let mut tries: libc::c_int = 0 as libc::c_int;
    let mut look: bool_ = 1 as libc::c_int as bool_;
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    if (*p_ptr).resist_continuum != 0 {
        msg_print(b"The space-time continuum can\'t be disrupted.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* Paranoia */
    if (*m_ptr).r_idx == 0 { return }
    /* Save the old location */
    oy = (*m_ptr).fy as libc::c_int;
    ox = (*m_ptr).fx as libc::c_int;
    /* Minimum distance */
    min = dis / 2 as libc::c_int;
    /* Look until done */
    while look != 0 {
        tries += 1;
        /* Verify max distance */
        if dis > 200 as libc::c_int { dis = 200 as libc::c_int }
        let mut current_block_17: u64;
        /* Try several locations */
        i = 0 as libc::c_int;
        while i < 500 as libc::c_int {
            loop 
                 /* Pick a (possibly illegal) location */
                 {
                ny = oy + Rand_div(1 as libc::c_int + dis + dis) - dis;
                nx = ox + Rand_div(1 as libc::c_int + dis + dis) - dis;
                d = distance(oy, ox, ny, nx);
                if d >= min && d <= dis { break ; }
            }
            /* Ignore illegal locations */
            if ny > 0 as libc::c_int && nx > 0 as libc::c_int &&
                   ny < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   nx < cur_wid as libc::c_int - 1 as libc::c_int {
                /* Require "empty" floor space */
                if (*f_info.offset((*cave[ny as
                                              usize].offset(nx as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x10 as libc::c_long != 0 &&
                       (*cave[ny as usize].offset(nx as isize)).feat as
                           libc::c_int != 0xaf as libc::c_int &&
                       (*cave[ny as usize].offset(nx as isize)).m_idx == 0 &&
                       !(ny == (*p_ptr).py as libc::c_int &&
                             nx == (*p_ptr).px as libc::c_int) {
                    /* Hack -- no teleport onto glyph of warding */
                    if !((*cave[ny as usize].offset(nx as isize)).feat as
                             libc::c_int == 0x3 as libc::c_int) {
                        if !((*cave[ny as usize].offset(nx as isize)).feat as
                                 libc::c_int == 0x40 as libc::c_int) {
                            /* ...nor onto the Pattern */
                            if !((*cave[ny as usize].offset(nx as isize)).feat
                                     as libc::c_int >= 0x41 as libc::c_int &&
                                     (*cave[ny as
                                                usize].offset(nx as
                                                                  isize)).feat
                                         as libc::c_int <=
                                         0x49 as libc::c_int) {
                                /* No teleporting into vaults and such */
                                if !((*p_ptr).inside_quest as libc::c_int != 0
                                         ||
                                         (*p_ptr).inside_arena as libc::c_int
                                             != 0) {
                                    if (*cave[ny as
                                                  usize].offset(nx as
                                                                    isize)).info
                                           as libc::c_int & 0x4 as libc::c_int
                                           != 0 {
                                        current_block_17 =
                                            10048703153582371463;
                                    } else {
                                        current_block_17 =
                                            4775909272756257391;
                                    }
                                } else {
                                    current_block_17 = 4775909272756257391;
                                }
                                match current_block_17 {
                                    10048703153582371463 => { }
                                    _ => {
                                        /* This grid looks good */
                                        look = 0 as libc::c_int as bool_;
                                        break ;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            i += 1
        }
        /* Increase the maximum distance */
        dis = dis * 2 as libc::c_int;
        /* Decrease the minimum distance */
        min = min / 2 as libc::c_int;
        /* Stop after MAX_TRIES tries */
        if tries > 100 as libc::c_int { return }
    }
    /* Sound */
    sound(14 as libc::c_int);
    /* Update the new location */
    (*cave[ny as usize].offset(nx as isize)).m_idx = m_idx as s16b;
    last_teleportation_y = ny as s16b;
    last_teleportation_x = nx as s16b;
    /* Update the old location */
    (*cave[oy as usize].offset(ox as isize)).m_idx = 0 as libc::c_int as s16b;
    /* Move the monster */
    (*m_ptr).fy = ny as byte_hack;
    (*m_ptr).fx = nx as byte_hack;
    /* Update the monster (new location) */
    update_mon(m_idx, 1 as libc::c_int as bool_);
    /* Redraw the old grid */
    lite_spot(oy, ox);
    /* Redraw the new grid */
    lite_spot(ny, nx);
    /* Update monster light */
    if (*r_ptr).flags9 & 0x4 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x200000 as libc::c_long) as
                u32b
    };
}
/*
 * Teleport monster next to the player
 */
#[no_mangle]
pub unsafe extern "C" fn teleport_to_player(mut m_idx: libc::c_int) {
    let mut ny: libc::c_int = 0 as libc::c_int;
    let mut nx: libc::c_int = 0 as libc::c_int;
    let mut oy: libc::c_int = 0;
    let mut ox: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut min: libc::c_int = 0;
    let mut dis: libc::c_int = 2 as libc::c_int;
    let mut look: bool_ = 1 as libc::c_int as bool_;
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    let mut attempts: libc::c_int = 500 as libc::c_int;
    if (*p_ptr).resist_continuum != 0 {
        msg_print(b"The space-time continuum can\'t be disrupted.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* Paranoia */
    if (*m_ptr).r_idx == 0 { return }
    /* "Skill" test */
    if Rand_div(100 as libc::c_int) + 1 as libc::c_int >
           (*m_ptr).level as libc::c_int {
        return
    }
    /* Save the old location */
    oy = (*m_ptr).fy as libc::c_int;
    ox = (*m_ptr).fx as libc::c_int;
    /* Minimum distance */
    min = dis / 2 as libc::c_int;
    /* Look until done */
    while look as libc::c_int != 0 && { attempts -= 1; (attempts) != 0 } {
        /* Verify max distance */
        if dis > 200 as libc::c_int { dis = 200 as libc::c_int }
        /* Try several locations */
        i = 0 as libc::c_int;
        while i < 500 as libc::c_int {
            loop 
                 /* Pick a (possibly illegal) location */
                 {
                ny =
                    (*p_ptr).py as libc::c_int +
                        Rand_div(1 as libc::c_int + dis + dis) - dis;
                nx =
                    (*p_ptr).px as libc::c_int +
                        Rand_div(1 as libc::c_int + dis + dis) - dis;
                d =
                    distance((*p_ptr).py as libc::c_int,
                             (*p_ptr).px as libc::c_int, ny, nx);
                if d >= min && d <= dis { break ; }
            }
            /* Ignore illegal locations */
            if ny > 0 as libc::c_int && nx > 0 as libc::c_int &&
                   ny < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   nx < cur_wid as libc::c_int - 1 as libc::c_int {
                /* Require "empty" floor space */
                if (*f_info.offset((*cave[ny as
                                              usize].offset(nx as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x10 as libc::c_long != 0 &&
                       (*cave[ny as usize].offset(nx as isize)).feat as
                           libc::c_int != 0xaf as libc::c_int &&
                       (*cave[ny as usize].offset(nx as isize)).m_idx == 0 &&
                       !(ny == (*p_ptr).py as libc::c_int &&
                             nx == (*p_ptr).px as libc::c_int) {
                    /* Hack -- no teleport onto glyph of warding */
                    if !((*cave[ny as usize].offset(nx as isize)).feat as
                             libc::c_int == 0x3 as libc::c_int) {
                        if !((*cave[ny as usize].offset(nx as isize)).feat as
                                 libc::c_int == 0x40 as libc::c_int) {
                            /* ...nor onto the Pattern */
                            if !((*cave[ny as usize].offset(nx as isize)).feat
                                     as libc::c_int >= 0x41 as libc::c_int &&
                                     (*cave[ny as
                                                usize].offset(nx as
                                                                  isize)).feat
                                         as libc::c_int <=
                                         0x49 as libc::c_int) {
                                /* No teleporting into vaults and such */
			/* if (cave[ny][nx].info & (CAVE_ICKY)) continue; */
                                /* This grid looks good */
                                look = 0 as libc::c_int as bool_;
                                break ;
                            }
                        }
                    }
                }
            }
            i += 1
        }
        /* Increase the maximum distance */
        dis = dis * 2 as libc::c_int;
        /* Decrease the minimum distance */
        min = min / 2 as libc::c_int
    }
    if attempts < 1 as libc::c_int { return }
    /* Sound */
    sound(14 as libc::c_int);
    /* Update the new location */
    (*cave[ny as usize].offset(nx as isize)).m_idx = m_idx as s16b;
    last_teleportation_y = ny as s16b;
    last_teleportation_x = nx as s16b;
    /* Update the old location */
    (*cave[oy as usize].offset(ox as isize)).m_idx = 0 as libc::c_int as s16b;
    /* Move the monster */
    (*m_ptr).fy = ny as byte_hack;
    (*m_ptr).fx = nx as byte_hack;
    /* Update the monster (new location) */
    update_mon(m_idx, 1 as libc::c_int as bool_);
    /* Redraw the old grid */
    lite_spot(oy, ox);
    /* Redraw the new grid */
    lite_spot(ny, nx);
    /* Update monster light */
    if (*r_ptr).flags9 & 0x4 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x200000 as libc::c_long) as
                u32b
    };
}
/*
 * Teleport the player to a location up to "dis" grids away.
 *
 * If no such spaces are readily available, the distance may increase.
 * Try very hard to move the player at least a quarter that distance.
 */
/* It'd be better if this was made an argument ... */
#[no_mangle]
pub static mut teleport_player_bypass: bool_ = 0 as libc::c_int as bool_;
#[no_mangle]
pub unsafe extern "C" fn teleport_player(mut dis: libc::c_int) {
    let mut d: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut min: libc::c_int = 0;
    let mut ox: libc::c_int = 0;
    let mut oy: libc::c_int = 0;
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut tries: libc::c_int = 0 as libc::c_int;
    let mut xx: libc::c_int = -(1 as libc::c_int);
    let mut yy: libc::c_int = -(1 as libc::c_int);
    let mut look: bool_ = 1 as libc::c_int as bool_;
    if (*p_ptr).resist_continuum as libc::c_int != 0 &&
           teleport_player_bypass == 0 {
        msg_print(b"The space-time continuum can\'t be disrupted.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    if (*p_ptr).wild_mode != 0 { return }
    /* Rooted means no move */
    if (*p_ptr).tim_roots != 0 { return } /* To be on the safe side... */
    if (*p_ptr).anti_tele as libc::c_int != 0 && teleport_player_bypass == 0 {
        msg_print(b"A mysterious force prevents you from teleporting!\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    if dungeon_flags2 as libc::c_long & 0x8 as libc::c_long != 0 &&
           teleport_player_bypass == 0 {
        msg_print(b"No teleport on special levels...\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    if dis > 200 as libc::c_int { dis = 200 as libc::c_int }
    /* Minimum distance */
    min = dis / 2 as libc::c_int;
    /* Look until done */
    while look != 0 {
        tries += 1;
        /* Verify max distance */
        if dis > 200 as libc::c_int { dis = 200 as libc::c_int }
        /* Try several locations */
        i = 0 as libc::c_int;
        while i < 500 as libc::c_int {
            loop 
                 /* Pick a (possibly illegal) location */
                 {
                y =
                    (*p_ptr).py as libc::c_int +
                        Rand_div(1 as libc::c_int + dis + dis) - dis;
                x =
                    (*p_ptr).px as libc::c_int +
                        Rand_div(1 as libc::c_int + dis + dis) - dis;
                d =
                    distance((*p_ptr).py as libc::c_int,
                             (*p_ptr).px as libc::c_int, y, x);
                if d >= min && d <= dis { break ; }
            }
            /* Ignore illegal locations */
            if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                   y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   x < cur_wid as libc::c_int - 1 as libc::c_int {
                /* Require "naked" floor space */
                if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x10 as libc::c_long != 0 &&
                       (*cave[y as usize].offset(x as isize)).feat as
                           libc::c_int != 0xaf as libc::c_int &&
                       (*f_info.offset((*cave[y as
                                                  usize].offset(x as
                                                                    isize)).feat
                                           as isize)).flags1 as libc::c_long &
                           0x40 as libc::c_long == 0 &&
                       (*cave[y as usize].offset(x as isize)).o_idx as
                           libc::c_int == 0 as libc::c_int &&
                       (*cave[y as usize].offset(x as isize)).m_idx as
                           libc::c_int == 0 as libc::c_int {
                    /* No teleporting into vaults and such */
                    if !((*cave[y as usize].offset(x as isize)).info as
                             libc::c_int & 0x4 as libc::c_int != 0) {
                        /* This grid looks good */
                        look = 0 as libc::c_int as bool_;
                        break ;
                    }
                }
            }
            i += 1
        }
        /* Increase the maximum distance */
        dis = dis * 2 as libc::c_int;
        /* Decrease the minimum distance */
        min = min / 2 as libc::c_int;
        /* Stop after MAX_TRIES tries */
        if tries > 100 as libc::c_int { return }
    }
    /* Sound */
    sound(9 as libc::c_int);
    /* Save the old location */
    oy = (*p_ptr).py as libc::c_int;
    ox = (*p_ptr).px as libc::c_int;
    /* Move the player */
    (*p_ptr).py = y as s16b;
    (*p_ptr).px = x as s16b;
    last_teleportation_y = y as s16b;
    last_teleportation_x = x as s16b;
    /* Redraw the old spot */
    lite_spot(oy, ox);
    while xx < 2 as libc::c_int {
        yy = -(1 as libc::c_int);
        while yy < 2 as libc::c_int {
            if !(xx == 0 as libc::c_int && yy == 0 as libc::c_int) {
                if (*cave[(oy + yy) as
                              usize].offset((ox + xx) as isize)).m_idx != 0 {
                    let mut r_ptr: *mut monster_race =
                        if !(*m_list.offset((*cave[(oy + yy) as
                                                       usize].offset((ox + xx)
                                                                         as
                                                                         isize)).m_idx
                                                as isize)).sr_ptr.is_null() {
                            (*m_list.offset((*cave[(oy + yy) as
                                                       usize].offset((ox + xx)
                                                                         as
                                                                         isize)).m_idx
                                                as isize)).sr_ptr
                        } else {
                            race_info_idx((*m_list.offset((*cave[(oy + yy) as
                                                                     usize].offset((ox
                                                                                        +
                                                                                        xx)
                                                                                       as
                                                                                       isize)).m_idx
                                                              as isize)).r_idx
                                              as libc::c_int,
                                          (*m_list.offset((*cave[(oy + yy) as
                                                                     usize].offset((ox
                                                                                        +
                                                                                        xx)
                                                                                       as
                                                                                       isize)).m_idx
                                                              as isize)).ego
                                              as libc::c_int)
                        };
                    if (*r_ptr).flags6 & 0x20 as libc::c_int as libc::c_uint
                           != 0 &&
                           (*r_ptr).flags3 &
                               0x200000 as libc::c_int as libc::c_uint == 0 {
                        /*
						 * The latter limitation is to avoid
						 * totally unkillable suckers...
						 */
                        if (*m_list.offset((*cave[(oy + yy) as
                                                      usize].offset((ox + xx)
                                                                        as
                                                                        isize)).m_idx
                                               as isize)).csleep == 0 {
                            teleport_to_player((*cave[(oy + yy) as
                                                          usize].offset((ox +
                                                                             xx)
                                                                            as
                                                                            isize)).m_idx
                                                   as libc::c_int);
                        }
                    }
                }
            }
            yy += 1
        }
        xx += 1
    }
    /* Redraw the new spot */
    lite_spot((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int);
    /* Check for new panel (redraw map) */
    verify_panel();
    /* Update stuff */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x100000 as libc::c_long | 0x10000000 as libc::c_long |
                  0x200000 as libc::c_long)) as u32b;
    /* Update the monsters */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x2000000 as libc::c_long) as u32b;
    /* Redraw trap detection status */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x20000000 as libc::c_long) as
            u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
    /* Handle stuff XXX XXX XXX */
    handle_stuff();
}
/*
 * get a grid near the given location
 *
 * This function is slightly obsessive about correctness.
 */
#[no_mangle]
pub unsafe extern "C" fn get_pos_player(mut dis: libc::c_int,
                                        mut ny: *mut libc::c_int,
                                        mut nx: *mut libc::c_int) {
    let mut d: libc::c_int = 0; /* To be on the safe side... */
    let mut i: libc::c_int = 0;
    let mut min: libc::c_int = 0;
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut tries: libc::c_int = 0 as libc::c_int;
    let mut look: bool_ = 1 as libc::c_int as bool_;
    if dis > 200 as libc::c_int { dis = 200 as libc::c_int }
    /* Minimum distance */
    min = dis / 2 as libc::c_int;
    /* Look until done */
    while look != 0 {
        tries += 1;
        /* Verify max distance */
        if dis > 200 as libc::c_int { dis = 200 as libc::c_int }
        /* Try several locations */
        i = 0 as libc::c_int;
        while i < 500 as libc::c_int {
            loop 
                 /* Pick a (possibly illegal) location */
                 {
                y =
                    (*p_ptr).py as libc::c_int +
                        Rand_div(1 as libc::c_int + dis + dis) - dis;
                x =
                    (*p_ptr).px as libc::c_int +
                        Rand_div(1 as libc::c_int + dis + dis) - dis;
                d =
                    distance((*p_ptr).py as libc::c_int,
                             (*p_ptr).px as libc::c_int, y, x);
                if d >= min && d <= dis { break ; }
            }
            /* Ignore illegal locations */
            if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                   y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   x < cur_wid as libc::c_int - 1 as libc::c_int {
                /* Require "naked" floor space */
                if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x10 as libc::c_long != 0 &&
                       (*cave[y as usize].offset(x as isize)).feat as
                           libc::c_int != 0xaf as libc::c_int &&
                       (*f_info.offset((*cave[y as
                                                  usize].offset(x as
                                                                    isize)).feat
                                           as isize)).flags1 as libc::c_long &
                           0x40 as libc::c_long == 0 &&
                       (*cave[y as usize].offset(x as isize)).o_idx as
                           libc::c_int == 0 as libc::c_int &&
                       (*cave[y as usize].offset(x as isize)).m_idx as
                           libc::c_int == 0 as libc::c_int {
                    /* No teleporting into vaults and such */
                    if !((*cave[y as usize].offset(x as isize)).info as
                             libc::c_int & 0x4 as libc::c_int != 0) {
                        /* This grid looks good */
                        look = 0 as libc::c_int as bool_;
                        break ;
                    }
                }
            }
            i += 1
        }
        /* Increase the maximum distance */
        dis = dis * 2 as libc::c_int;
        /* Decrease the minimum distance */
        min = min / 2 as libc::c_int;
        /* Stop after MAX_TRIES tries */
        if tries > 100 as libc::c_int { return }
    }
    *ny = y;
    *nx = x;
}
/*
 * Teleport a monster to a grid near the given location
 *
 * This function is slightly obsessive about correctness.
 */
#[no_mangle]
pub unsafe extern "C" fn teleport_monster_to(mut m_idx: libc::c_int,
                                             mut ny: libc::c_int,
                                             mut nx: libc::c_int) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut oy: libc::c_int = 0;
    let mut ox: libc::c_int = 0;
    let mut dis: libc::c_int = 0 as libc::c_int;
    let mut ctr: libc::c_int = 0 as libc::c_int;
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    if (*p_ptr).resist_continuum != 0 {
        msg_print(b"The space-time continuum can\'t be disrupted.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    if (*p_ptr).anti_tele != 0 {
        msg_print(b"A mysterious force prevents you from teleporting!\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    loop 
         /* Find a usable location */
         {
        loop 
             /* Pick a nearby legal location */
             {
            y = ny + Rand_div(1 as libc::c_int + dis + dis) - dis;
            x = nx + Rand_div(1 as libc::c_int + dis + dis) - dis;
            if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                   y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   x < cur_wid as libc::c_int - 1 as libc::c_int {
                break ;
            }
        }
        /* Not on the player's grid */
		/* Accept "naked" floor grids */
        if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                               isize)).flags1 as libc::c_long &
               0x10 as libc::c_long != 0 &&
               (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
                   0xaf as libc::c_int &&
               (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                   isize)).flags1 as libc::c_long &
                   0x40 as libc::c_long == 0 &&
               (*cave[y as usize].offset(x as isize)).o_idx as libc::c_int ==
                   0 as libc::c_int &&
               (*cave[y as usize].offset(x as isize)).m_idx as libc::c_int ==
                   0 as libc::c_int && y != (*p_ptr).py as libc::c_int &&
               x != (*p_ptr).px as libc::c_int {
            break ;
        }
        /* Occasionally advance the distance */
        ctr += 1;
        if ctr >
               4 as libc::c_int * dis * dis + 4 as libc::c_int * dis +
                   1 as libc::c_int {
            ctr = 0 as libc::c_int;
            dis += 1
        }
    }
    /* Sound */
    sound(14 as libc::c_int);
    /* Save the old position */
    oy = (*m_ptr).fy as libc::c_int;
    ox = (*m_ptr).fx as libc::c_int;
    (*cave[oy as usize].offset(ox as isize)).m_idx = 0 as libc::c_int as s16b;
    /* Move the monster */
    (*m_ptr).fy = y as byte_hack;
    (*m_ptr).fx = x as byte_hack;
    (*cave[y as usize].offset(x as isize)).m_idx = m_idx as s16b;
    last_teleportation_y = y as s16b;
    last_teleportation_x = x as s16b;
    /* Update the monster (new location) */
    update_mon(m_idx, 1 as libc::c_int as bool_);
    /* Redraw the old spot */
    lite_spot(oy, ox);
    /* Redraw the new spot */
    lite_spot((*m_ptr).fy as libc::c_int, (*m_ptr).fx as libc::c_int);
}
/*
 * Teleport player to a grid near the given location
 *
 * This function is slightly obsessive about correctness.
 * This function allows teleporting into vaults (!)
 */
#[no_mangle]
pub unsafe extern "C" fn teleport_player_to(mut ny: libc::c_int,
                                            mut nx: libc::c_int) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut oy: libc::c_int = 0;
    let mut ox: libc::c_int = 0;
    let mut dis: libc::c_int = 0 as libc::c_int;
    let mut ctr: libc::c_int = 0 as libc::c_int;
    if (*p_ptr).resist_continuum != 0 {
        msg_print(b"The space-time continuum can\'t be disrupted.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    if (*p_ptr).anti_tele != 0 {
        msg_print(b"A mysterious force prevents you from teleporting!\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    if dungeon_flags2 as libc::c_long & 0x8 as libc::c_long != 0 {
        msg_print(b"No teleport on special levels...\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Rooted means no move */
    if (*p_ptr).tim_roots != 0 { return }
    loop 
         /* Find a usable location */
         {
        loop 
             /* Pick a nearby legal location */
             {
            y = ny + Rand_div(1 as libc::c_int + dis + dis) - dis;
            x = nx + Rand_div(1 as libc::c_int + dis + dis) - dis;
            if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                   y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   x < cur_wid as libc::c_int - 1 as libc::c_int {
                break ;
            }
        }
        /* Accept "naked" floor grids */
        if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                               isize)).flags1 as libc::c_long &
               0x10 as libc::c_long != 0 &&
               (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
                   0xaf as libc::c_int &&
               (*cave[y as usize].offset(x as isize)).o_idx as libc::c_int ==
                   0 as libc::c_int &&
               (*cave[y as usize].offset(x as isize)).m_idx as libc::c_int ==
                   0 as libc::c_int {
            break ;
        }
        /* Occasionally advance the distance */
        ctr += 1;
        if ctr >
               4 as libc::c_int * dis * dis + 4 as libc::c_int * dis +
                   1 as libc::c_int {
            ctr = 0 as libc::c_int;
            dis += 1
        }
    }
    /* Sound */
    sound(9 as libc::c_int);
    /* Save the old location */
    oy = (*p_ptr).py as libc::c_int;
    ox = (*p_ptr).px as libc::c_int;
    /* Move the player */
    (*p_ptr).py = y as s16b;
    (*p_ptr).px = x as s16b;
    last_teleportation_y = y as s16b;
    last_teleportation_x = x as s16b;
    /* Redraw the old spot */
    lite_spot(oy, ox);
    /* Redraw the new spot */
    lite_spot((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int);
    /* Check for new panel (redraw map) */
    verify_panel();
    /* Update stuff */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x100000 as libc::c_long | 0x10000000 as libc::c_long |
                  0x200000 as libc::c_long)) as u32b;
    /* Update the monsters */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x2000000 as libc::c_long) as u32b;
    /* Redraw trap detection status */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x20000000 as libc::c_long) as
            u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
    /* Handle stuff XXX XXX XXX */
    handle_stuff();
}
/*
 * Teleport the player one level up or down (random when legal)
 */
#[no_mangle]
pub unsafe extern "C" fn teleport_player_level() {
    /* No effect in arena or quest */
    if (*p_ptr).inside_arena as libc::c_int != 0 ||
           (*p_ptr).inside_quest as libc::c_int != 0 {
        msg_print(b"There is no effect.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    if dungeon_flags2 as libc::c_long & 0x8 as libc::c_long != 0 {
        msg_print(b"No teleport on special levels...\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    if dungeon_flags2 as libc::c_long & 0x4000 as libc::c_long != 0 {
        msg_print(b"Some powerfull force prevents your from teleporting.\x00"
                      as *const u8 as *const libc::c_char);
        return
    }
    if (*p_ptr).resist_continuum != 0 {
        msg_print(b"The space-time continuum can\'t be disrupted.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* Hack -- when you are fated to die, you cant cheat :) */
    if dungeon_type as libc::c_int == DUNGEON_DEATH {
        msg_print(b"A mysterious force prevents you from teleporting!\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    if (*p_ptr).anti_tele != 0 {
        msg_print(b"A mysterious force prevents you from teleporting!\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* Rooted means no move */
    if (*p_ptr).tim_roots != 0 { return }
    if dun_level == 0 {
        msg_print(b"You sink through the floor.\x00" as *const u8 as
                      *const libc::c_char);
        autosave_checkpoint();
        dun_level += 1;
        /* Leaving */
        (*p_ptr).leaving = 1 as libc::c_int as bool_
    } else if is_quest(dun_level as libc::c_int) != 0 ||
                  dun_level as libc::c_int >=
                      128 as libc::c_int - 1 as libc::c_int {
        msg_print(b"You rise up through the ceiling.\x00" as *const u8 as
                      *const libc::c_char);
        autosave_checkpoint();
        dun_level -= 1;
        /* Leaving */
        (*p_ptr).leaving = 1 as libc::c_int as bool_
    } else if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
        msg_print(b"You rise up through the ceiling.\x00" as *const u8 as
                      *const libc::c_char);
        autosave_checkpoint();
        dun_level -= 1;
        /* Leaving */
        (*p_ptr).leaving = 1 as libc::c_int as bool_
    } else {
        msg_print(b"You sink through the floor.\x00" as *const u8 as
                      *const libc::c_char);
        autosave_checkpoint();
        dun_level += 1;
        /* Leaving */
        (*p_ptr).leaving = 1 as libc::c_int as bool_
    }
    /* Sound */
    sound(24 as libc::c_int);
}
/*
 * Recall the player to town or dungeon
 */
#[no_mangle]
pub unsafe extern "C" fn recall_player(mut d: libc::c_int,
                                       mut f: libc::c_int) {
    /* Rooted means no move */
    if (*p_ptr).tim_roots != 0 {
        msg_print(b"Your roots prevent the recall.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    if dun_level as libc::c_int != 0 &&
           *max_dlv.offset(dungeon_type as isize) as libc::c_int >
               dun_level as libc::c_int && (*p_ptr).inside_quest == 0 {
        if get_check(b"Reset recall depth? \x00" as *const u8 as
                         *const libc::c_char) != 0 {
            *max_dlv.offset(dungeon_type as isize) = dun_level
        }
    }
    if (*p_ptr).word_recall == 0 {
        (*p_ptr).word_recall = (Rand_div(d) + f) as s16b;
        msg_print(b"The air about you becomes charged...\x00" as *const u8 as
                      *const libc::c_char);
    } else {
        (*p_ptr).word_recall = 0 as libc::c_int as s16b;
        msg_print(b"A tension leaves the air around you...\x00" as *const u8
                      as *const libc::c_char);
    }
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x200 as libc::c_long) as u32b;
}
/*
 * Get a legal "multi-hued" color for drawing "spells"
 */
unsafe extern "C" fn mh_attr(mut max: libc::c_int) -> byte_hack {
    match Rand_div(max) + 1 as libc::c_int {
        1 => { return 4 as libc::c_int as byte_hack }
        2 => { return 5 as libc::c_int as byte_hack }
        3 => { return 6 as libc::c_int as byte_hack }
        4 => { return 11 as libc::c_int as byte_hack }
        5 => { return 3 as libc::c_int as byte_hack }
        6 => { return 10 as libc::c_int as byte_hack }
        7 => { return 12 as libc::c_int as byte_hack }
        8 => { return 13 as libc::c_int as byte_hack }
        9 => { return 14 as libc::c_int as byte_hack }
        10 => { return 7 as libc::c_int as byte_hack }
        11 => { return 15 as libc::c_int as byte_hack }
        12 => { return 2 as libc::c_int as byte_hack }
        13 => { return 1 as libc::c_int as byte_hack }
        14 => { return 9 as libc::c_int as byte_hack }
        15 => { return 8 as libc::c_int as byte_hack }
        _ => { }
    }
    return 1 as libc::c_int as byte_hack;
}
/*
 * Return a color to use for the bolt/ball spells
 */
#[no_mangle]
pub unsafe extern "C" fn spell_color(mut type_0: libc::c_int) -> byte_hack {
    /* Hooks! */
    if process_hooks_ret(58 as libc::c_int,
                         b"d\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char,
                         b"(d,d)\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char, type_0,
                         streq(ANGBAND_GRAF,
                               b"new\x00" as *const u8 as *const libc::c_char)
                             as libc::c_int) != 0 {
        return process_hooks_return[0 as libc::c_int as usize].num as
                   byte_hack
    }
    /* Check if A.B.'s new graphics should be used (rr9) */
    if streq(ANGBAND_GRAF, b"new\x00" as *const u8 as *const libc::c_char) !=
           0 {
        /* Analyze */
        match type_0 {
            10 => { return 0xf as libc::c_int as byte_hack }
            3 => { return 0x4 as libc::c_int as byte_hack }
            1 => { return 0x2 as libc::c_int as byte_hack }
            5 => { return 0 as libc::c_int as byte_hack }
            4 => { return 0x1 as libc::c_int as byte_hack }
            2 => { return 0x3 as libc::c_int as byte_hack }
            6 => { return 0x3 as libc::c_int as byte_hack }
            79 => { return 0 as libc::c_int as byte_hack }
            80 => { return 0 as libc::c_int as byte_hack }
            26 => { return 0xe as libc::c_int as byte_hack }
            11 => { return 0xf as libc::c_int as byte_hack }
            14 => { return 0x4 as libc::c_int as byte_hack }
            13 => { return 0x4 as libc::c_int as byte_hack }
            31 => { return 0x7 as libc::c_int as byte_hack }
            30 => { return mh_attr(15 as libc::c_int) }
            32 => { return 0x5 as libc::c_int as byte_hack }
            33 => { return 0xc as libc::c_int as byte_hack }
            22 => { return mh_attr(4 as libc::c_int) }
            21 => { return 0x9 as libc::c_int as byte_hack }
            20 => { return 0x8 as libc::c_int as byte_hack }
            23 => { return 0x9 as libc::c_int as byte_hack }
            24 => { return 0x9 as libc::c_int as byte_hack }
            35 => { return 0x9 as libc::c_int as byte_hack }
            34 => { return 0x9 as libc::c_int as byte_hack }
            17 => { return 0x6 as libc::c_int as byte_hack }
            15 => { return 0x6 as libc::c_int as byte_hack }
            18 => { return 0x7 as libc::c_int as byte_hack }
            16 => { return 0x7 as libc::c_int as byte_hack }
            12 => { return 0xb as libc::c_int as byte_hack }
            27 => { return 0 as libc::c_int as byte_hack }
            28 => { return 0x1 as libc::c_int as byte_hack }
            72 => { return 0xf as libc::c_int as byte_hack }
            77 => { return 0x7 as libc::c_int as byte_hack }
            73 => { return mh_attr(2 as libc::c_int) }
            81 => { return 0x5 as libc::c_int as byte_hack }
            85 | 86 | 87 | 89 => { return 0x9 as libc::c_int as byte_hack }
            _ => { }
        }
    } else {
        /* Normal tiles or ASCII */
        /* Analyze */
        match type_0 {
            10 => { return 2 as libc::c_int as byte_hack }
            3 => {
                return if (Rand_div(5 as libc::c_int) + 1 as libc::c_int) <
                              3 as libc::c_int {
                           11 as libc::c_int
                       } else { 13 as libc::c_int } as byte_hack
            }
            1 => {
                return if (Rand_div(7 as libc::c_int) + 1 as libc::c_int) <
                              6 as libc::c_int {
                           1 as libc::c_int
                       } else if Rand_div(4 as libc::c_int) + 1 as libc::c_int
                                     == 1 as libc::c_int {
                           6 as libc::c_int
                       } else { 14 as libc::c_int } as byte_hack
            }
            5 => {
                return if (Rand_div(6 as libc::c_int) + 1 as libc::c_int) <
                              4 as libc::c_int {
                           11 as libc::c_int
                       } else if Rand_div(4 as libc::c_int) + 1 as libc::c_int
                                     == 1 as libc::c_int {
                           4 as libc::c_int
                       } else { 12 as libc::c_int } as byte_hack
            }
            4 => {
                return if (Rand_div(6 as libc::c_int) + 1 as libc::c_int) <
                              4 as libc::c_int {
                           1 as libc::c_int
                       } else { 9 as libc::c_int } as byte_hack
            }
            2 => {
                return if (Rand_div(5 as libc::c_int) + 1 as libc::c_int) <
                              3 as libc::c_int {
                           13 as libc::c_int
                       } else { 5 as libc::c_int } as byte_hack
            }
            6 => {
                return if (Rand_div(7 as libc::c_int) + 1 as libc::c_int) <
                              3 as libc::c_int {
                           13 as libc::c_int
                       } else { 5 as libc::c_int } as byte_hack
            }
            79 => {
                return if Rand_div(5 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                           3 as libc::c_int
                       } else { 1 as libc::c_int } as byte_hack
            }
            80 => {
                return if Rand_div(6 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                           4 as libc::c_int
                       } else { 8 as libc::c_int } as byte_hack
            }
            26 => {
                return if Rand_div(5 as libc::c_int) + 1 as libc::c_int !=
                              1 as libc::c_int {
                           10 as libc::c_int
                       } else { 14 as libc::c_int } as byte_hack
            }
            11 => { return 15 as libc::c_int as byte_hack }
            14 => {
                return if Rand_div(4 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                           14 as libc::c_int
                       } else { 6 as libc::c_int } as byte_hack
            }
            13 => {
                return if Rand_div(4 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                           14 as libc::c_int
                       } else { 6 as libc::c_int } as byte_hack
            }
            31 => {
                return if Rand_div(4 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                           2 as libc::c_int
                       } else { 8 as libc::c_int } as byte_hack
            }
            30 => { return mh_attr(15 as libc::c_int) }
            32 => {
                return if Rand_div(5 as libc::c_int) + 1 as libc::c_int !=
                              1 as libc::c_int {
                           14 as libc::c_int
                       } else { 10 as libc::c_int } as byte_hack
            }
            33 => {
                return if (Rand_div(5 as libc::c_int) + 1 as libc::c_int) <
                              3 as libc::c_int {
                           12 as libc::c_int
                       } else { 10 as libc::c_int } as byte_hack
            }
            22 => { return mh_attr(4 as libc::c_int) }
            21 => {
                return if Rand_div(4 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                           10 as libc::c_int
                       } else { 1 as libc::c_int } as byte_hack
            }
            20 => {
                return if (Rand_div(5 as libc::c_int) + 1 as libc::c_int) <
                              3 as libc::c_int {
                           7 as libc::c_int
                       } else { 2 as libc::c_int } as byte_hack
            }
            23 => {
                return if (Rand_div(5 as libc::c_int) + 1 as libc::c_int) <
                              3 as libc::c_int {
                           9 as libc::c_int
                       } else { 3 as libc::c_int } as byte_hack
            }
            24 => {
                return if (Rand_div(5 as libc::c_int) + 1 as libc::c_int) <
                              3 as libc::c_int {
                           2 as libc::c_int
                       } else { 9 as libc::c_int } as byte_hack
            }
            35 => {
                return if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                           15 as libc::c_int
                       } else { 7 as libc::c_int } as byte_hack
            }
            34 => {
                return if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                           1 as libc::c_int
                       } else { 8 as libc::c_int } as byte_hack
            }
            17 => {
                return if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                           3 as libc::c_int
                       } else { 11 as libc::c_int } as byte_hack
            }
            15 => {
                return if Rand_div(4 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                           3 as libc::c_int
                       } else { 11 as libc::c_int } as byte_hack
            }
            18 => {
                return if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                           0 as libc::c_int
                       } else { 8 as libc::c_int } as byte_hack
            }
            16 => {
                return if Rand_div(4 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                           0 as libc::c_int
                       } else { 8 as libc::c_int } as byte_hack
            }
            12 => {
                return if Rand_div(5 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                           4 as libc::c_int
                       } else { 12 as libc::c_int } as byte_hack
            }
            27 => {
                return if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                           4 as libc::c_int
                       } else { 7 as libc::c_int } as byte_hack
            }
            28 => {
                return if Rand_div(4 as libc::c_int) + 1 as libc::c_int ==
                              1 as libc::c_int {
                           14 as libc::c_int
                       } else { 1 as libc::c_int } as byte_hack
            }
            72 => {
                return if (Rand_div(6 as libc::c_int) + 1 as libc::c_int) <
                              4 as libc::c_int {
                           12 as libc::c_int
                       } else if Rand_div(4 as libc::c_int) + 1 as libc::c_int
                                     == 1 as libc::c_int {
                           4 as libc::c_int
                       } else { 15 as libc::c_int } as byte_hack
            }
            105 | 77 => { return 8 as libc::c_int as byte_hack }
            73 => { return mh_attr(2 as libc::c_int) }
            81 => {
                return if Rand_div(3 as libc::c_int) + 1 as libc::c_int !=
                              1 as libc::c_int {
                           8 as libc::c_int
                       } else if Rand_div(2 as libc::c_int) + 1 as libc::c_int
                                     == 1 as libc::c_int {
                           3 as libc::c_int
                       } else { 15 as libc::c_int } as byte_hack
            }
            85 | 86 | 87 | 89 => {
                return if Rand_div(3 as libc::c_int) + 1 as libc::c_int !=
                              1 as libc::c_int {
                           14 as libc::c_int
                       } else { 1 as libc::c_int } as byte_hack
            }
            _ => { }
        }
    }
    /* Standard "color" */
    return 1 as libc::c_int as byte_hack;
}
/*
 * Find the attr/char pair to use for a spell effect
 *
 * It is moving (or has moved) from (x,y) to (nx,ny).
 *
 * If the distance is not "one", we (may) return "*".
 */
unsafe extern "C" fn bolt_pict(mut y: libc::c_int, mut x: libc::c_int,
                               mut ny: libc::c_int, mut nx: libc::c_int,
                               mut typ: libc::c_int) -> u16b {
    let mut base: libc::c_int = 0;
    let mut k: byte_hack = 0;
    let mut a: byte_hack = 0;
    let mut c: libc::c_char = 0;
    /* No motion (*) */
    if ny == y && nx == x {
        base = 0x30 as libc::c_int
    } else if nx == x {
        base = 0x40 as libc::c_int
    } else if ny == y {
        base = 0x50 as libc::c_int
    } else if ny - y == x - nx {
        base = 0x60 as libc::c_int
    } else if ny - y == nx - x {
        base = 0x70 as libc::c_int
    } else {
        /* Vertical (|) */
        /* Horizontal (-) */
        /* Diagonal (/) */
        /* Diagonal (\) */
        /* Weird (*) */
        base = 0x30 as libc::c_int
    }
    /* Basic spell color */
    k = spell_color(typ);
    /* Obtain attr/char */
    a = misc_to_attr[(base + k as libc::c_int) as usize];
    c = misc_to_char[(base + k as libc::c_int) as usize];
    /* Create pict */
    return ((a as u16b as libc::c_int) << 8 as libc::c_int |
                c as byte_hack as libc::c_int) as u16b;
}
/*
 * Cast the spelbound spells
 */
#[no_mangle]
pub unsafe extern "C" fn spellbinder_trigger() {
    let mut i: libc::c_int = 0;
    cmsg_print(13 as libc::c_int as byte_hack,
               b"The spellbinder is triggered!\x00" as *const u8 as
                   *const libc::c_char);
    i = 0 as libc::c_int;
    while i < (*p_ptr).spellbinder_num as libc::c_int {
        msg_format(b"Triggering spell %s.\x00" as *const u8 as
                       *const libc::c_char,
                   (*school_spells.offset((*p_ptr).spellbinder[i as usize] as
                                              isize)).name);
        exec_lua(format(b"cast_school_spell(%d, spell(%d), TRUE)\x00" as
                            *const u8 as *const libc::c_char,
                        (*p_ptr).spellbinder[i as usize],
                        (*p_ptr).spellbinder[i as usize]));
        i += 1
    }
    (*p_ptr).spellbinder_num = 0 as libc::c_int as byte_hack;
    (*p_ptr).spellbinder_trigger = 0 as libc::c_int as byte_hack;
}
/*
 * Decreases players hit points and sets death flag if necessary
 *
 * XXX XXX XXX Invulnerability needs to be changed into a "shield"
 *
 * XXX XXX XXX Hack -- this function allows the user to save (or quit)
 * the game when he dies, since the "You die." message is shown before
 * setting the player to "dead".
 */
#[no_mangle]
pub unsafe extern "C" fn take_hit(mut damage: libc::c_int,
                                  mut hit_from: cptr) {
    let mut o_ptr: *mut object_type =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(49 as libc::c_int as
                                                         isize) as
            *mut object_type;
    let mut old_chp: libc::c_int = (*p_ptr).chp as libc::c_int;
    let mut pen_invuln: bool_ = 0 as libc::c_int as bool_;
    let mut monster_take: bool_ = 0 as libc::c_int as bool_;
    let mut death_message: [libc::c_char; 80] = [0; 80];
    let mut warning: libc::c_int =
        (*p_ptr).mhp as libc::c_int * hitpoint_warn as libc::c_int /
            10 as libc::c_int;
    let mut percent: libc::c_int = 0;
    /* Paranoia */
    if death != 0 { return }
    /* Disturb */
    disturb(1 as libc::c_int, 0 as libc::c_int);
    /* Apply "invulnerability" */
    if (*p_ptr).invuln as libc::c_int != 0 && damage < 9000 as libc::c_int {
        if Rand_div(13 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int
           {
            pen_invuln = 1 as libc::c_int as bool_
        } else { return }
    }
    /* Apply disruption shield */
    if (*p_ptr).disrupt_shield != 0 {
        if (*p_ptr).csp as libc::c_int > damage * 2 as libc::c_int {
            (*p_ptr).csp =
                ((*p_ptr).csp as libc::c_int - damage * 2 as libc::c_int) as
                    s16b;
            damage = 0 as libc::c_int
        } else {
            damage -= (*p_ptr).csp as libc::c_int / 2 as libc::c_int;
            (*p_ptr).csp = 0 as libc::c_int as s16b;
            (*p_ptr).csp_frac = 0 as libc::c_int as u16b
        }
        /* Display the mana */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b
    }
    /* Hurt the wielded monster if any */
    if (*o_ptr).k_idx as libc::c_int != 0 &&
           Rand_div(100 as libc::c_int) <
               5 as libc::c_int + get_skill(8 as libc::c_int) as libc::c_int
           && carried_monster_hit == 0 {
        let mut sym_name: cptr = symbiote_name(1 as libc::c_int as bool_);
        if (*o_ptr).pval2 as libc::c_int - damage <= 0 as libc::c_int {
            cmsg_format(12 as libc::c_int as byte_hack,
                        b"%s dies from protecting you, you feel very sad...\x00"
                            as *const u8 as *const libc::c_char, sym_name);
            inc_stack_size_ex(49 as libc::c_int, -(1 as libc::c_int),
                              OPTIMIZE, NO_DESCRIBE);
            damage -= (*o_ptr).pval2 as libc::c_int;
            (*o_ptr).pval2 = 0 as libc::c_int as s16b;
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x10000000 as libc::c_long)
                    as u32b
        } else {
            msg_format(b"%s takes the damage instead of you.\x00" as *const u8
                           as *const libc::c_char, sym_name);
            (*o_ptr).pval2 = ((*o_ptr).pval2 as libc::c_int - damage) as s16b;
            monster_take = 1 as libc::c_int as bool_
        }
        carried_monster_hit = 0 as libc::c_int as bool_;
        /* Display the monster hitpoints */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x10000000 as libc::c_long) as
                u32b
    }
    /* Hurt the player */
    if monster_take == 0 {
        (*p_ptr).chp = ((*p_ptr).chp as libc::c_int - damage) as s16b
    }
    /* Display the hitpoints */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x40 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
    if pen_invuln != 0 {
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"The attack penetrates your shield of invulnerability!\x00"
                       as *const u8 as *const libc::c_char);
    }
    /* Dead player */
    if ((*p_ptr).chp as libc::c_int) < 0 as libc::c_int {
        /* Necromancers get a special treatment */
        if has_ability(10 as libc::c_int) == 0 ||
               (*p_ptr).necro_extra & 0x8 as libc::c_int as libc::c_uint != 0
           {
            /* Sound */
            sound(7 as libc::c_int);
            /* Hack -- Note death */
            if last_words == 0 {
                cmsg_print(4 as libc::c_int as byte_hack,
                           b"You die.\x00" as *const u8 as
                               *const libc::c_char);
                msg_print(0 as cptr);
            } else {
                get_rnd_line(b"death.txt\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             death_message.as_mut_ptr());
                cmsg_print(4 as libc::c_int as byte_hack,
                           death_message.as_mut_ptr() as cptr);
            }
            /* Note cause of death */
            strcpy(died_from.as_mut_ptr(), hit_from);
            if (*p_ptr).image != 0 {
                strcat(died_from.as_mut_ptr(),
                       b"(?)\x00" as *const u8 as *const libc::c_char);
            }
            /* Leaving */
            (*p_ptr).leaving = 1 as libc::c_int as bool_;
            /* No longer a winner */
            total_winner = 0 as libc::c_int as u16b;
            /* Note death */
            death = 1 as libc::c_int as bool_;
            if get_check(b"Dump the screen? \x00" as *const u8 as
                             *const libc::c_char) != 0 {
                do_cmd_html_dump();
            }
            /* Dead */
            return
        } else {
            /* Just turn the necromancer into an undead */
            (*p_ptr).necro_extra |= 0x8 as libc::c_int as libc::c_uint;
            (*p_ptr).necro_extra2 =
                ((*p_ptr).lev as libc::c_int +
                     (Rand_div((*p_ptr).lev as libc::c_int / 2 as libc::c_int)
                          - (*p_ptr).lev as libc::c_int / 4 as libc::c_int))
                    as u32b;
            if (*p_ptr).necro_extra2 < 1 as libc::c_int as libc::c_uint {
                (*p_ptr).necro_extra2 = 1 as libc::c_int as u32b
            }
            cmsg_format(8 as libc::c_int as byte_hack,
                        b"You have to kill %d monster%s to be brought back to life.\x00"
                            as *const u8 as *const libc::c_char,
                        (*p_ptr).necro_extra2,
                        if (*p_ptr).necro_extra2 ==
                               1 as libc::c_int as libc::c_uint {
                            b"\x00" as *const u8 as *const libc::c_char
                        } else {
                            b"s\x00" as *const u8 as *const libc::c_char
                        });
            /* MEGA-HACK !!! */
            calc_hitpoints();
            /* Enforce maximum */
            (*p_ptr).chp = (*p_ptr).mhp;
            (*p_ptr).chp_frac = 0 as libc::c_int as u16b;
            do_cmd_wiz_cure_all();
            /* Display the hitpoints */
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x40 as libc::c_long) as
                    u32b;
            /* Window stuff */
            (*p_ptr).window =
                ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as
                    u32b
        }
    }
    /* Hitpoint warning */
    if ((*p_ptr).chp as libc::c_int) < warning {
        /* Hack -- bell on first notice */
        if alert_hitpoint as libc::c_int != 0 && old_chp > warning { bell(); }
        sound(28 as libc::c_int);
        /* Message */
        if (*p_ptr).necro_extra & 0x8 as libc::c_int as libc::c_uint != 0 {
            cmsg_print(4 as libc::c_int as byte_hack,
                       b"*** LOW DEATHPOINT WARNING! ***\x00" as *const u8 as
                           *const libc::c_char);
        } else {
            cmsg_print(4 as libc::c_int as byte_hack,
                       b"*** LOW HITPOINT WARNING! ***\x00" as *const u8 as
                           *const libc::c_char);
        }
        msg_print(0 as cptr);
    }
    /* How much life is left ? */
    percent =
        (*p_ptr).chp as libc::c_int * 100 as libc::c_int /
            (*p_ptr).mhp as libc::c_int;
    /* Check the spellbinder trigger */
    if (*p_ptr).spellbinder_trigger as libc::c_int == 1 as libc::c_int {
        /* Trigger ?! */
        if percent <= 75 as libc::c_int { spellbinder_trigger(); }
    } else if (*p_ptr).spellbinder_trigger as libc::c_int == 2 as libc::c_int
     {
        /* Trigger ?! */
        if percent <= 50 as libc::c_int { spellbinder_trigger(); }
    } else if (*p_ptr).spellbinder_trigger as libc::c_int == 3 as libc::c_int
     {
        /* Trigger ?! */
        if percent <= 25 as libc::c_int { spellbinder_trigger(); }
    }
    /* Melkor acn summon to help you */
    if percent < 25 as libc::c_int {
        if (*p_ptr).pgod as libc::c_int == 4 as libc::c_int &&
               (*p_ptr).praying as libc::c_int != 0 {
            let mut chance: libc::c_int =
                (*p_ptr).grace / 500 as libc::c_int; /*  * 100 / 50000; */
            if Rand_div(100 as libc::c_int) < chance - 10 as libc::c_int {
                let mut i: libc::c_int = 0;
                let mut type_0: libc::c_int = 16 as libc::c_int;
                if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
                    type_0 = 17 as libc::c_int
                }
                if (*p_ptr).grace > 10000 as libc::c_int {
                    if type_0 == 16 as libc::c_int {
                        type_0 = 39 as libc::c_int
                    } else { type_0 = 21 as libc::c_int }
                }
                chance /= 10 as libc::c_int;
                if chance < 1 as libc::c_int { chance = 1 as libc::c_int }
                i = 0 as libc::c_int;
                while i < chance {
                    summon_specific_friendly((*p_ptr).py as libc::c_int,
                                             (*p_ptr).px as libc::c_int,
                                             dun_level as libc::c_int /
                                                 2 as libc::c_int, type_0,
                                             0 as libc::c_int as bool_);
                    i += 1
                }
                msg_print(b"Melkor summons monsters to help you!\x00" as
                              *const u8 as *const libc::c_char);
            }
        }
    }
    if player_char_health != 0 {
        lite_spot((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int);
    };
}
/* Decrease player's sanity. This is a copy of the function above. */
#[no_mangle]
pub unsafe extern "C" fn take_sanity_hit(mut damage: libc::c_int,
                                         mut hit_from: cptr) {
    let mut old_csane: libc::c_int = (*p_ptr).csane as libc::c_int;
    let mut death_message: [libc::c_char; 80] = [0; 80];
    let mut warning: libc::c_int =
        (*p_ptr).msane as libc::c_int * hitpoint_warn as libc::c_int /
            10 as libc::c_int;
    /* Paranoia */
    if death != 0 { return }
    /* Disturb */
    disturb(1 as libc::c_int, 0 as libc::c_int);
    /* Hurt the player */
    (*p_ptr).csane = ((*p_ptr).csane as libc::c_int - damage) as s16b;
    /* Display the hitpoints */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x800000 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
    /* Dead player */
    if ((*p_ptr).csane as libc::c_int) < 0 as libc::c_int {
        /* Sound */
        sound(7 as libc::c_int);
        /* Hack -- Note death */
        cmsg_print(10 as libc::c_int as byte_hack,
                   b"You turn into an unthinking vegetable.\x00" as *const u8
                       as *const libc::c_char);
        if last_words == 0 {
            cmsg_print(4 as libc::c_int as byte_hack,
                       b"You die.\x00" as *const u8 as *const libc::c_char);
            msg_print(0 as cptr);
        } else {
            get_rnd_line(b"death.txt\x00" as *const u8 as *const libc::c_char
                             as *mut libc::c_char,
                         death_message.as_mut_ptr());
            cmsg_print(4 as libc::c_int as byte_hack,
                       death_message.as_mut_ptr() as cptr);
        }
        /* Note cause of death */
        strcpy(died_from.as_mut_ptr(), hit_from);
        if (*p_ptr).image != 0 {
            strcat(died_from.as_mut_ptr(),
                   b"(?)\x00" as *const u8 as *const libc::c_char);
        }
        /* Leaving */
        (*p_ptr).leaving = 1 as libc::c_int as bool_;
        /* Note death */
        death = 1 as libc::c_int as bool_;
        if get_check(b"Dump the screen? \x00" as *const u8 as
                         *const libc::c_char) != 0 {
            do_cmd_html_dump();
        }
        /* Dead */
        return
    }
    /* Hitpoint warning */
    if ((*p_ptr).csane as libc::c_int) < warning {
        /* Hack -- bell on first notice */
        if alert_hitpoint as libc::c_int != 0 && old_csane > warning {
            bell();
        }
        sound(28 as libc::c_int);
        /* Message */
        cmsg_print(4 as libc::c_int as byte_hack,
                   b"*** LOW SANITY WARNING! ***\x00" as *const u8 as
                       *const libc::c_char);
        msg_print(0 as cptr);
    };
}
/*
 * Note that amulets, rods, and high-level spell books are immune
 * to "inventory damage" of any kind.  Also sling ammo and shovels.
 */
/*
 * Does a given class of objects (usually) hate acid?
 * Note that acid can either melt or corrode something.
 */
unsafe extern "C" fn hates_acid(mut o_ptr: *mut object_type) -> bool_ {
    /* Analyze the type */
    match (*o_ptr).tval as libc::c_int {
        17 | 18 | 19 | 23 | 24 | 21 | 22 | 32 | 33 | 34 | 30 | 31 | 35 | 36 |
        37 | 38 => {
            /* Wearable items */
            return 1 as libc::c_int as bool_
        }
        55 | 70 => {
            /* Staffs/Scrolls are wood/paper */
            return 1 as libc::c_int as bool_
        }
        7 => {
            /* Ouch */
            return 1 as libc::c_int as bool_
        }
        1 | 2 | 10 => {
            /* Junk is useless */
            return 1 as libc::c_int as bool_
        }
        _ => { }
    }
    return 0 as libc::c_int as bool_;
}
/*
 * Does a given object (usually) hate electricity?
 */
unsafe extern "C" fn hates_elec(mut o_ptr: *mut object_type) -> bool_ {
    match (*o_ptr).tval as libc::c_int {
        45 | 65 | 10 => { return 1 as libc::c_int as bool_ }
        _ => { }
    }
    return 0 as libc::c_int as bool_;
}
/*
 * Does a given object (usually) hate fire?
 * Hafted/Polearm weapons have wooden shafts.
 * Arrows/Bows are mostly wooden.
 */
unsafe extern "C" fn hates_fire(mut o_ptr: *mut object_type) -> bool_ {
    /* Analyze the type */
    match (*o_ptr).tval as libc::c_int {
        17 => {
            /* Special case for archers */
            return 1 as libc::c_int as bool_
        }
        39 | 19 | 21 | 22 | 30 | 31 | 35 | 36 => {
            /* Wearable */
            return 1 as libc::c_int as bool_
        }
        111 | 112 | 113 => {
            /* Books */
            return 1 as libc::c_int as bool_
        }
        7 => {
            /* Chests */
            return 1 as libc::c_int as bool_
        }
        55 | 70 | 10 => {
            /* Staffs/Scrolls burn */
            return 1 as libc::c_int as bool_
        }
        _ => { }
    }
    return 0 as libc::c_int as bool_;
}
/*
 * Does a given object (usually) hate cold?
 */
unsafe extern "C" fn hates_cold(mut o_ptr: *mut object_type) -> bool_ {
    match (*o_ptr).tval as libc::c_int {
        72 | 71 | 77 | 2 | 10 => { return 1 as libc::c_int as bool_ }
        _ => { }
    }
    return 0 as libc::c_int as bool_;
}
/*
 * Melt something
 */
unsafe extern "C" fn set_acid_destroy(mut o_ptr: *mut object_type)
 -> libc::c_int {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    if hates_acid(o_ptr) == 0 { return 0 as libc::c_int }
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    if f3 as libc::c_long & 0x100000 as libc::c_long != 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/*
 * Electrical damage
 */
unsafe extern "C" fn set_elec_destroy(mut o_ptr: *mut object_type)
 -> libc::c_int {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    if hates_elec(o_ptr) == 0 { return 0 as libc::c_int }
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    if f3 as libc::c_long & 0x200000 as libc::c_long != 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/*
 * Burn something
 */
unsafe extern "C" fn set_fire_destroy(mut o_ptr: *mut object_type)
 -> libc::c_int {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    if hates_fire(o_ptr) == 0 { return 0 as libc::c_int }
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    if f3 as libc::c_long & 0x400000 as libc::c_long != 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/*
 * Freeze things
 */
unsafe extern "C" fn set_cold_destroy(mut o_ptr: *mut object_type)
 -> libc::c_int {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    if hates_cold(o_ptr) == 0 { return 0 as libc::c_int }
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    if f3 as libc::c_long & 0x800000 as libc::c_long != 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/*
 * Destroys a type of item on a given percent chance
 * Note that missiles are no longer necessarily all destroyed
 * Destruction taken from "melee.c" code for "stealing".
 * Returns number of items destroyed.
 */
unsafe extern "C" fn inven_damage(mut typ: inven_func, mut perc: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut amt: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    /* Count the casualties */
    k = 0 as libc::c_int;
    /* Scan through the slots backwards */
    i = 0 as libc::c_int;
    while i < 23 as libc::c_int {
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Skip non-objects */
        if !((*o_ptr).k_idx == 0) {
            /* Hack -- for now, skip artifacts */
            if !((*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
                     (if (*o_ptr).name1 as libc::c_int != 0 {
                          1 as libc::c_int
                      } else { 0 as libc::c_int }) != 0 ||
                     (if (*o_ptr).art_name as libc::c_int != 0 {
                          1 as libc::c_int
                      } else { 0 as libc::c_int }) != 0 ||
                     (if (*k_info.offset((*o_ptr).k_idx as isize)).flags3 as
                             libc::c_long & 0x8000 as libc::c_long != 0 {
                          1 as libc::c_int
                      } else { 0 as libc::c_int }) != 0 ||
                     (*o_ptr).art_name as libc::c_int != 0) {
                /* Give this item slot a shot at death */
                if Some(typ.expect("non-null function pointer")).expect("non-null function pointer")(o_ptr)
                       != 0 {
                    /* Count the casualties */
                    j = 0 as libc::c_int;
                    amt = j;
                    while j < (*o_ptr).number as libc::c_int {
                        if Rand_div(100 as libc::c_int) < perc { amt += 1 }
                        j += 1
                    }
                    /* Some casualities */
                    if amt != 0 {
                        /* Get a description */
                        object_desc(o_name.as_mut_ptr(), o_ptr,
                                    0 as libc::c_int, 3 as libc::c_int);
                        /* Message */
                        msg_format(b"%sour %s (%c) %s destroyed!\x00" as
                                       *const u8 as *const libc::c_char,
                                   if (*o_ptr).number as libc::c_int >
                                          1 as libc::c_int {
                                       if amt ==
                                              (*o_ptr).number as libc::c_int {
                                           b"All of y\x00" as *const u8 as
                                               *const libc::c_char
                                       } else if amt > 1 as libc::c_int {
                                           b"Some of y\x00" as *const u8 as
                                               *const libc::c_char
                                       } else {
                                           b"One of y\x00" as *const u8 as
                                               *const libc::c_char
                                       }
                                   } else {
                                       b"Y\x00" as *const u8 as
                                           *const libc::c_char
                                   }, o_name.as_mut_ptr(),
                                   index_to_label(i) as libc::c_int,
                                   if amt > 1 as libc::c_int {
                                       b"were\x00" as *const u8 as
                                           *const libc::c_char
                                   } else {
                                       b"was\x00" as *const u8 as
                                           *const libc::c_char
                                   });
                        /* Potions smash open */
                        if (*k_info.offset((*o_ptr).k_idx as isize)).tval as
                               libc::c_int == 71 as libc::c_int {
                            potion_smash_effect(0 as libc::c_int,
                                                (*p_ptr).py as libc::c_int,
                                                (*p_ptr).px as libc::c_int,
                                                (*o_ptr).sval as libc::c_int);
                        }
                        /*
				 * Hack -- If rods or wand are destroyed, the total maximum 
				 * timeout or charges of the stack needs to be reduced, 
				 * unless all the items are being destroyed. -LM-
				 */
                        if (*o_ptr).tval as libc::c_int == 65 as libc::c_int
                               && amt < (*o_ptr).number as libc::c_int {
                            (*o_ptr).pval -=
                                (*o_ptr).pval * amt /
                                    (*o_ptr).number as libc::c_int
                        }
                        /* Destroy "amt" items */
                        inc_stack_size_ex(i, -amt, OPTIMIZE, NO_DESCRIBE);
                        /* Count the casualties */
                        k += amt
                    }
                }
            }
        }
        i += 1
    }
    /* Return the casualty count */
    return k;
}
/*
 * Acid has hit the player, attempt to affect some armor.
 *
 * Note that the "base armor" of an object never changes.
 *
 * If any armor is damaged (or resists), the player takes less damage.
 */
unsafe extern "C" fn minus_ac() -> libc::c_int {
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    /* Pick a (possibly empty) inventory slot */
    match Rand_div(6 as libc::c_int) + 1 as libc::c_int {
        1 => {
            o_ptr =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(37 as libc::c_int
                                                                 as isize) as
                    *mut object_type
        }
        2 => {
            o_ptr =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(39 as libc::c_int
                                                                 as isize) as
                    *mut object_type
        }
        3 => {
            o_ptr =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(38 as libc::c_int
                                                                 as isize) as
                    *mut object_type
        }
        4 => {
            o_ptr =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(44 as libc::c_int
                                                                 as isize) as
                    *mut object_type
        }
        5 => {
            o_ptr =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(42 as libc::c_int
                                                                 as isize) as
                    *mut object_type
        }
        6 => {
            o_ptr =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(47 as libc::c_int
                                                                 as isize) as
                    *mut object_type
        }
        _ => { }
    }
    /* Nothing to damage */
    if (*o_ptr).k_idx == 0 { return 0 as libc::c_int }
    /* No damage left to be done */
    if (*o_ptr).ac as libc::c_int + (*o_ptr).to_a as libc::c_int <=
           0 as libc::c_int {
        return 0 as libc::c_int
    }
    /* Describe */
    object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                0 as libc::c_int);
    /* Extract the flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Object resists */
    if f3 as libc::c_long & 0x100000 as libc::c_long != 0 {
        msg_format(b"Your %s is unaffected!\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr());
        return 1 as libc::c_int
    }
    /* Message */
    msg_format(b"Your %s is damaged!\x00" as *const u8 as *const libc::c_char,
               o_name.as_mut_ptr());
    /* Damage the item */
    (*o_ptr).to_a -= 1;
    /* Calculate bonuses */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x2 as libc::c_long | 0x8 as libc::c_long)) as u32b;
    /* Item was damaged */
    return 1 as libc::c_int;
}
/*
 * Hurt the player with Acid
 */
#[no_mangle]
pub unsafe extern "C" fn acid_dam(mut dam: libc::c_int, mut kb_str: cptr) {
    let mut inv: libc::c_int =
        if dam < 30 as libc::c_int {
            1 as libc::c_int
        } else if dam < 60 as libc::c_int {
            2 as libc::c_int
        } else { 3 as libc::c_int };
    /* Total Immunity */
    if (*p_ptr).immune_acid as libc::c_int != 0 || dam <= 0 as libc::c_int {
        return
    }
    /* Resist the damage */
    if (*p_ptr).resist_acid != 0 {
        dam = (dam + 2 as libc::c_int) / 3 as libc::c_int
    }
    if (*p_ptr).oppose_acid != 0 {
        dam = (dam + 2 as libc::c_int) / 3 as libc::c_int
    }
    if !((*p_ptr).oppose_acid as libc::c_int != 0 ||
             (*p_ptr).resist_acid as libc::c_int != 0) &&
           Rand_div(32 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int
       {
        do_dec_stat(5 as libc::c_int, 2 as libc::c_int);
    }
    /* If any armor gets hit, defend the player */
    if minus_ac() != 0 { dam = (dam + 1 as libc::c_int) / 2 as libc::c_int }
    /* Take damage */
    take_hit(dam, kb_str);
    /* Inventory damage */
    if !((*p_ptr).oppose_acid as libc::c_int != 0 &&
             (*p_ptr).resist_acid as libc::c_int != 0) {
        inven_damage(Some(set_acid_destroy as
                              unsafe extern "C" fn(_: *mut object_type)
                                  -> libc::c_int), inv);
    };
}
/*
 * Hurt the player with electricity
 */
#[no_mangle]
pub unsafe extern "C" fn elec_dam(mut dam: libc::c_int, mut kb_str: cptr) {
    let mut inv: libc::c_int =
        if dam < 30 as libc::c_int {
            1 as libc::c_int
        } else if dam < 60 as libc::c_int {
            2 as libc::c_int
        } else { 3 as libc::c_int };
    /* Total immunity */
    if (*p_ptr).immune_elec as libc::c_int != 0 || dam <= 0 as libc::c_int {
        return
    }
    /* Resist the damage */
    if (*p_ptr).oppose_elec != 0 {
        dam = (dam + 2 as libc::c_int) / 3 as libc::c_int
    }
    if (*p_ptr).resist_elec != 0 {
        dam = (dam + 2 as libc::c_int) / 3 as libc::c_int
    }
    if !((*p_ptr).oppose_elec as libc::c_int != 0 ||
             (*p_ptr).resist_elec as libc::c_int != 0) &&
           Rand_div(32 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int
       {
        do_dec_stat(3 as libc::c_int, 2 as libc::c_int);
    }
    /* Take damage */
    take_hit(dam, kb_str);
    /* Inventory damage */
    if !((*p_ptr).oppose_elec as libc::c_int != 0 &&
             (*p_ptr).resist_elec as libc::c_int != 0) {
        inven_damage(Some(set_elec_destroy as
                              unsafe extern "C" fn(_: *mut object_type)
                                  -> libc::c_int), inv);
    };
}
/*
 * Hurt the player with Fire
 */
#[no_mangle]
pub unsafe extern "C" fn fire_dam(mut dam: libc::c_int, mut kb_str: cptr) {
    let mut inv: libc::c_int =
        if dam < 30 as libc::c_int {
            1 as libc::c_int
        } else if dam < 60 as libc::c_int {
            2 as libc::c_int
        } else { 3 as libc::c_int };
    /* Totally immune */
    if (*p_ptr).immune_fire as libc::c_int != 0 || dam <= 0 as libc::c_int {
        return
    }
    /* Resist the damage */
    if (*p_ptr).sensible_fire != 0 {
        dam = (dam + 2 as libc::c_int) * 2 as libc::c_int
    }
    if (*p_ptr).resist_fire != 0 {
        dam = (dam + 2 as libc::c_int) / 3 as libc::c_int
    }
    if (*p_ptr).oppose_fire != 0 {
        dam = (dam + 2 as libc::c_int) / 3 as libc::c_int
    }
    if !((*p_ptr).oppose_fire as libc::c_int != 0 ||
             (*p_ptr).resist_fire as libc::c_int != 0) &&
           Rand_div(32 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int
       {
        do_dec_stat(0 as libc::c_int, 2 as libc::c_int);
    }
    /* Take damage */
    take_hit(dam, kb_str);
    /* Inventory damage */
    if !((*p_ptr).resist_fire as libc::c_int != 0 &&
             (*p_ptr).oppose_fire as libc::c_int != 0) {
        inven_damage(Some(set_fire_destroy as
                              unsafe extern "C" fn(_: *mut object_type)
                                  -> libc::c_int), inv);
    };
}
/*
 * Hurt the player with Cold
 */
#[no_mangle]
pub unsafe extern "C" fn cold_dam(mut dam: libc::c_int, mut kb_str: cptr) {
    let mut inv: libc::c_int =
        if dam < 30 as libc::c_int {
            1 as libc::c_int
        } else if dam < 60 as libc::c_int {
            2 as libc::c_int
        } else { 3 as libc::c_int };
    /* Total immunity */
    if (*p_ptr).immune_cold as libc::c_int != 0 || dam <= 0 as libc::c_int {
        return
    }
    /* Resist the damage */
    if (*p_ptr).resist_cold != 0 {
        dam = (dam + 2 as libc::c_int) / 3 as libc::c_int
    }
    if (*p_ptr).oppose_cold != 0 {
        dam = (dam + 2 as libc::c_int) / 3 as libc::c_int
    }
    if !((*p_ptr).oppose_cold as libc::c_int != 0 ||
             (*p_ptr).resist_cold as libc::c_int != 0) &&
           Rand_div(32 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int
       {
        do_dec_stat(0 as libc::c_int, 2 as libc::c_int);
    }
    /* Take damage */
    take_hit(dam, kb_str);
    /* Inventory damage */
    if !((*p_ptr).resist_cold as libc::c_int != 0 &&
             (*p_ptr).oppose_cold as libc::c_int != 0) {
        inven_damage(Some(set_cold_destroy as
                              unsafe extern "C" fn(_: *mut object_type)
                                  -> libc::c_int), inv);
    };
}
/*
 * Increases a stat by one randomized level             -RAK-
 *
 * Note that this function (used by stat potions) now restores
 * the stat BEFORE increasing it.
 */
#[no_mangle]
pub unsafe extern "C" fn inc_stat(mut stat: libc::c_int) -> bool_ {
    let mut value: libc::c_int = 0;
    let mut gain: libc::c_int = 0;
    /* Then augment the current/max stat */
    value = (*p_ptr).stat_cur[stat as usize] as libc::c_int;
    /* Cannot go above 18/100 */
    if value < 18 as libc::c_int + 100 as libc::c_int {
        /* Gain one (sometimes two) points */
        if value < 18 as libc::c_int {
            gain =
                if Rand_div(100 as libc::c_int) < 75 as libc::c_int {
                    1 as libc::c_int
                } else { 2 as libc::c_int };
            value += gain
        } else if value < 18 as libc::c_int + 98 as libc::c_int {
            /* Gain 1/6 to 1/3 of distance to 18/100 */
            /* Approximate gain value */
            gain =
                ((18 as libc::c_int + 100 as libc::c_int - value) /
                     2 as libc::c_int + 3 as libc::c_int) / 2 as libc::c_int;
            /* Paranoia */
            if gain < 1 as libc::c_int { gain = 1 as libc::c_int }
            /* Apply the bonus */
            value +=
                Rand_div(gain) + 1 as libc::c_int + gain / 2 as libc::c_int;
            /* Maximal value */
            if value > 18 as libc::c_int + 99 as libc::c_int {
                value = 18 as libc::c_int + 99 as libc::c_int
            }
        } else {
            /* Gain one point at a time */
            value += 1
        }
        /* Save the new value */
        (*p_ptr).stat_cur[stat as usize] = value as s16b;
        /* Bring up the maximum too */
        if value > (*p_ptr).stat_max[stat as usize] as libc::c_int {
            (*p_ptr).stat_max[stat as usize] = value as s16b
        }
        /* Recalculate bonuses */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
        /* Success */
        return 1 as libc::c_int as bool_
    }
    /* Nothing to gain */
    return 0 as libc::c_int as bool_;
}
/*
 * Decreases a stat by an amount indended to vary from 0 to 100 percent.
 *
 * Amount could be a little higher in extreme cases to mangle very high
 * stats from massive assaults.  -CWS
 *
 * Note that "permanent" means that the *given* amount is permanent,
 * not that the new value becomes permanent.  This may not work exactly
 * as expected, due to "weirdness" in the algorithm, but in general,
 * if your stat is already drained, the "max" value will not drop all
 * the way down to the "cur" value.
 */
#[no_mangle]
pub unsafe extern "C" fn dec_stat(mut stat: libc::c_int,
                                  mut amount: libc::c_int,
                                  mut mode: libc::c_int) -> bool_ {
    let mut cur: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut loss: libc::c_int = 0 as libc::c_int;
    let mut same: libc::c_int = 0;
    let mut res: libc::c_int = 0 as libc::c_int;
    /* Acquire current value */
    cur = (*p_ptr).stat_cur[stat as usize] as libc::c_int;
    max = (*p_ptr).stat_max[stat as usize] as libc::c_int;
    /* Note when the values are identical */
    same = (cur == max) as libc::c_int;
    /* Damage "current" value */
    if cur > 3 as libc::c_int {
        /* Handle "low" values */
        if cur <= 18 as libc::c_int {
            if amount > 90 as libc::c_int { cur -= 1 }
            if amount > 50 as libc::c_int { cur -= 1 }
            if amount > 20 as libc::c_int { cur -= 1 }
            cur -= 1
        } else {
            /* Handle "high" values */
            /* Hack -- Decrement by a random amount between one-quarter */
			/* and one-half of the stat bonus times the percentage, with a */
			/* minimum damage of half the percentage. -CWS */
            loss =
                ((cur - 18 as libc::c_int) / 2 as libc::c_int +
                     1 as libc::c_int) / 2 as libc::c_int + 1 as libc::c_int;
            if loss < 1 as libc::c_int { loss = 1 as libc::c_int }
            loss =
                (Rand_div(loss) + 1 as libc::c_int + loss) * amount /
                    100 as libc::c_int;
            if loss < amount / 2 as libc::c_int {
                loss = amount / 2 as libc::c_int
            }
            cur = cur - loss;
            if cur < 18 as libc::c_int {
                cur =
                    if amount <= 20 as libc::c_int {
                        18 as libc::c_int
                    } else { 17 as libc::c_int }
            }
        }
        /* Paranoia */
        /* Randomize the loss */
        /* Maximal loss */
        /* Lose some points */
        /* Hack -- Only reduce stat to 17 sometimes */
        /* Prevent illegal values */
        if cur < 3 as libc::c_int { cur = 3 as libc::c_int }
        /* Something happened */
        if cur != (*p_ptr).stat_cur[stat as usize] as libc::c_int {
            res = 1 as libc::c_int
        }
    }
    /* Damage "max" value */
    if mode == 3 as libc::c_int && max > 3 as libc::c_int {
        /* Handle "low" values */
        if max <= 18 as libc::c_int {
            if amount > 90 as libc::c_int { max -= 1 }
            if amount > 50 as libc::c_int { max -= 1 }
            if amount > 20 as libc::c_int { max -= 1 }
            max -= 1
        } else {
            /* Handle "high" values */
            /* Hack -- Decrement by a random amount between one-quarter */
			/* and one-half of the stat bonus times the percentage, with a */
			/* minimum damage of half the percentage. -CWS */
            loss =
                ((max - 18 as libc::c_int) / 2 as libc::c_int +
                     1 as libc::c_int) / 2 as libc::c_int + 1 as libc::c_int;
            loss =
                (Rand_div(loss) + 1 as libc::c_int + loss) * amount /
                    100 as libc::c_int;
            if loss < amount / 2 as libc::c_int {
                loss = amount / 2 as libc::c_int
            }
            max = max - loss;
            if max < 18 as libc::c_int {
                max =
                    if amount <= 20 as libc::c_int {
                        18 as libc::c_int
                    } else { 17 as libc::c_int }
            }
        }
        /* Lose some points */
        /* Hack -- Only reduce stat to 17 sometimes */
        /* Hack -- keep it clean */
        if same != 0 || max < cur { max = cur }
        /* Something happened */
        if max != (*p_ptr).stat_max[stat as usize] as libc::c_int {
            res = 1 as libc::c_int
        }
    }
    /* Apply changes */
    if res != 0 {
        if mode == 1 as libc::c_int {
            let mut dectime: u16b = 0;
            /* a little crude, perhaps */
            dectime =
                (Rand_div(*max_dlv.offset(dungeon_type as isize) as
                              libc::c_int * 50 as libc::c_int) +
                     50 as libc::c_int) as u16b;
            /* Calculate loss */
            loss = (*p_ptr).stat_cur[stat as usize] as libc::c_int - cur;
            /* prevent overflow, stat_cnt = u16b */
			/* or add another temporary drain... */
            if ((*p_ptr).stat_cnt[stat as usize] as libc::c_int +
                    dectime as libc::c_int) <
                   (*p_ptr).stat_cnt[stat as usize] as libc::c_int ||
                   (*p_ptr).stat_los[stat as usize] as libc::c_int >
                       0 as libc::c_int {
                (*p_ptr).stat_cnt[stat as usize] =
                    ((*p_ptr).stat_cnt[stat as usize] as libc::c_int +
                         dectime as libc::c_int) as s16b;
                (*p_ptr).stat_los[stat as usize] =
                    ((*p_ptr).stat_los[stat as usize] as libc::c_int + loss)
                        as s16b
            } else {
                (*p_ptr).stat_cnt[stat as usize] = dectime as s16b;
                (*p_ptr).stat_los[stat as usize] = loss as s16b
            }
        }
        /* Actually set the stat to its new value. */
        (*p_ptr).stat_cur[stat as usize] = cur as s16b;
        (*p_ptr).stat_max[stat as usize] = max as s16b;
        /* Recalculate bonuses */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b
    }
    /* Done */
    return res as bool_;
}
/*
 * Restore a stat.  Return TRUE only if this actually makes a difference.
 */
#[no_mangle]
pub unsafe extern "C" fn res_stat(mut stat: libc::c_int, mut full: bool_)
 -> bool_ {
    /* Fully restore */
    if full != 0 {
        /* Restore if needed */
        if (*p_ptr).stat_cur[stat as usize] as libc::c_int !=
               (*p_ptr).stat_max[stat as usize] as libc::c_int {
            /* Restore */
            (*p_ptr).stat_cur[stat as usize] =
                (*p_ptr).stat_max[stat as usize];
            /* Remove temporary drain */
            (*p_ptr).stat_cnt[stat as usize] = 0 as libc::c_int as s16b;
            (*p_ptr).stat_los[stat as usize] = 0 as libc::c_int as s16b;
            /* Recalculate bonuses */
            (*p_ptr).update =
                ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as
                    u32b;
            /* Something happened */
            return 1 as libc::c_int as bool_
        }
    } else if (*p_ptr).stat_los[stat as usize] != 0 {
        /* Restore temporary drained stat */
        /* Restore if needed */
        /* Restore */
        (*p_ptr).stat_cur[stat as usize] =
            ((*p_ptr).stat_cur[stat as usize] as libc::c_int +
                 (*p_ptr).stat_los[stat as usize] as libc::c_int) as s16b;
        /* Remove temporary drain */
        (*p_ptr).stat_cnt[stat as usize] = 0 as libc::c_int as s16b;
        (*p_ptr).stat_los[stat as usize] = 0 as libc::c_int as s16b;
        /* Recalculate bonuses */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
        /* Something happened */
        return 1 as libc::c_int as bool_
    }
    /* Nothing to restore */
    return 0 as libc::c_int as bool_;
}
/*
 * Apply disenchantment to the player's stuff
 *
 * XXX XXX XXX This function is also called from the "melee" code
 *
 * If "mode is set to 0 then a random slot will be used, if not the "mode"
 * slot will be used.
 *
 * Return "TRUE" if the player notices anything
 */
#[no_mangle]
pub unsafe extern "C" fn apply_disenchant(mut mode: libc::c_int) -> bool_ {
    let mut t: libc::c_int = mode;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    if mode == 0 {
        /* Pick a random slot */
        match Rand_div(8 as libc::c_int) + 1 as libc::c_int {
            1 => { t = 24 as libc::c_int }
            2 => { t = 27 as libc::c_int }
            3 => { t = 37 as libc::c_int }
            4 => { t = 38 as libc::c_int }
            5 => { t = 39 as libc::c_int }
            6 => { t = 42 as libc::c_int }
            7 => { t = 44 as libc::c_int }
            8 => { t = 47 as libc::c_int }
            _ => { }
        }
    }
    /* Get the item */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(t as isize) as
            *mut object_type;
    /* No item, nothing happens */
    if (*o_ptr).k_idx == 0 { return 0 as libc::c_int as bool_ }
    /* Nothing to disenchant */
    if (*o_ptr).to_h as libc::c_int <= 0 as libc::c_int &&
           (*o_ptr).to_d as libc::c_int <= 0 as libc::c_int &&
           (*o_ptr).to_a as libc::c_int <= 0 as libc::c_int {
        /* Nothing to notice */
        return 0 as libc::c_int as bool_
    }
    /* Describe the object */
    object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                0 as libc::c_int);
    /* Artifacts have 71% chance to resist */
    if ((*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
            (if (*o_ptr).name1 as libc::c_int != 0 {
                 1 as libc::c_int
             } else { 0 as libc::c_int }) != 0 ||
            (if (*o_ptr).art_name as libc::c_int != 0 {
                 1 as libc::c_int
             } else { 0 as libc::c_int }) != 0 ||
            (if (*k_info.offset((*o_ptr).k_idx as isize)).flags3 as
                    libc::c_long & 0x8000 as libc::c_long != 0 {
                 1 as libc::c_int
             } else { 0 as libc::c_int }) != 0 ||
            (*o_ptr).art_name as libc::c_int != 0) &&
           Rand_div(100 as libc::c_int) < 71 as libc::c_int {
        /* Message */
        msg_format(b"Your %s (%c) resist%s disenchantment!\x00" as *const u8
                       as *const libc::c_char, o_name.as_mut_ptr(),
                   index_to_label(t) as libc::c_int,
                   if (*o_ptr).number as libc::c_int != 1 as libc::c_int {
                       b"\x00" as *const u8 as *const libc::c_char
                   } else { b"s\x00" as *const u8 as *const libc::c_char });
        /* Notice */
        return 1 as libc::c_int as bool_
    }
    /* Disenchant tohit */
    if (*o_ptr).to_h as libc::c_int > 0 as libc::c_int { (*o_ptr).to_h -= 1 }
    if (*o_ptr).to_h as libc::c_int > 5 as libc::c_int &&
           Rand_div(100 as libc::c_int) < 20 as libc::c_int {
        (*o_ptr).to_h -= 1
    }
    /* Disenchant todam */
    if (*o_ptr).to_d as libc::c_int > 0 as libc::c_int { (*o_ptr).to_d -= 1 }
    if (*o_ptr).to_d as libc::c_int > 5 as libc::c_int &&
           Rand_div(100 as libc::c_int) < 20 as libc::c_int {
        (*o_ptr).to_d -= 1
    }
    /* Disenchant toac */
    if (*o_ptr).to_a as libc::c_int > 0 as libc::c_int { (*o_ptr).to_a -= 1 }
    if (*o_ptr).to_a as libc::c_int > 5 as libc::c_int &&
           Rand_div(100 as libc::c_int) < 20 as libc::c_int {
        (*o_ptr).to_a -= 1
    }
    /* Message */
    msg_format(b"Your %s (%c) %s disenchanted!\x00" as *const u8 as
                   *const libc::c_char, o_name.as_mut_ptr(),
               index_to_label(t) as libc::c_int,
               if (*o_ptr).number as libc::c_int != 1 as libc::c_int {
                   b"were\x00" as *const u8 as *const libc::c_char
               } else { b"was\x00" as *const u8 as *const libc::c_char });
    /* Recalculate bonuses */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x2 as libc::c_long | 0x8 as libc::c_long)) as u32b;
    /* Notice */
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn corrupt_player() {
    let mut max1: libc::c_int = 0;
    let mut cur1: libc::c_int = 0;
    let mut max2: libc::c_int = 0;
    let mut cur2: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    /* Pick a pair of stats */
    ii = Rand_div(6 as libc::c_int);
    jj = ii;
    while jj == ii {
        /* loop */
        jj = Rand_div(6 as libc::c_int)
    }
    max1 = (*p_ptr).stat_max[ii as usize] as libc::c_int;
    cur1 = (*p_ptr).stat_cur[ii as usize] as libc::c_int;
    max2 = (*p_ptr).stat_max[jj as usize] as libc::c_int;
    cur2 = (*p_ptr).stat_cur[jj as usize] as libc::c_int;
    (*p_ptr).stat_max[ii as usize] = max2 as s16b;
    (*p_ptr).stat_cur[ii as usize] = cur2 as s16b;
    (*p_ptr).stat_max[jj as usize] = max1 as s16b;
    (*p_ptr).stat_cur[jj as usize] = cur1 as s16b;
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
}
/*
 * Apply Nexus
 */
unsafe extern "C" fn apply_nexus(mut m_ptr: *mut monster_type) {
    if m_ptr.is_null() { return }
    if dungeon_flags2 as libc::c_long & 0x8 as libc::c_long == 0 {
        match Rand_div(7 as libc::c_int) + 1 as libc::c_int {
            1 | 2 | 3 => { teleport_player(200 as libc::c_int); }
            4 | 5 => {
                teleport_player_to((*m_ptr).fy as libc::c_int,
                                   (*m_ptr).fx as libc::c_int);
            }
            6 => {
                if Rand_div(100 as libc::c_int) <
                       (*p_ptr).skill_sav as libc::c_int {
                    msg_print(b"You resist the effects!\x00" as *const u8 as
                                  *const libc::c_char);
                } else {
                    /* Teleport Level */
                    teleport_player_level();
                }
            }
            7 => {
                if Rand_div(100 as libc::c_int) <
                       (*p_ptr).skill_sav as libc::c_int {
                    msg_print(b"You resist the effects!\x00" as *const u8 as
                                  *const libc::c_char);
                } else {
                    msg_print(b"Your body starts to scramble...\x00" as
                                  *const u8 as *const libc::c_char);
                    corrupt_player();
                }
            }
            _ => { }
        }
    };
}
/*
 * Convert 2 couples of coordonates to a direction
 */
#[no_mangle]
pub unsafe extern "C" fn yx_to_dir(mut y2: libc::c_int, mut x2: libc::c_int,
                                   mut y1: libc::c_int, mut x1: libc::c_int)
 -> libc::c_int {
    let mut y: libc::c_int = y2 - y1;
    let mut x: libc::c_int = x2 - x1;
    if y == 0 as libc::c_int && x == 1 as libc::c_int {
        return 6 as libc::c_int
    }
    if y == 0 as libc::c_int && x == -(1 as libc::c_int) {
        return 4 as libc::c_int
    }
    if y == -(1 as libc::c_int) && x == 0 as libc::c_int {
        return 8 as libc::c_int
    }
    if y == 1 as libc::c_int && x == 0 as libc::c_int {
        return 2 as libc::c_int
    }
    if y == -(1 as libc::c_int) && x == -(1 as libc::c_int) {
        return 7 as libc::c_int
    }
    if y == -(1 as libc::c_int) && x == 1 as libc::c_int {
        return 9 as libc::c_int
    }
    if y == 1 as libc::c_int && x == 1 as libc::c_int {
        return 3 as libc::c_int
    }
    if y == 1 as libc::c_int && x == -(1 as libc::c_int) {
        return 1 as libc::c_int
    }
    return 5 as libc::c_int;
}
/*
 * Give the opposate direction of the given one
 */
#[no_mangle]
pub unsafe extern "C" fn invert_dir(mut dir: libc::c_int) -> libc::c_int {
    if dir == 4 as libc::c_int { return 6 as libc::c_int }
    if dir == 6 as libc::c_int { return 4 as libc::c_int }
    if dir == 8 as libc::c_int { return 2 as libc::c_int }
    if dir == 2 as libc::c_int { return 8 as libc::c_int }
    if dir == 7 as libc::c_int { return 3 as libc::c_int }
    if dir == 9 as libc::c_int { return 1 as libc::c_int }
    if dir == 1 as libc::c_int { return 9 as libc::c_int }
    if dir == 3 as libc::c_int { return 7 as libc::c_int }
    return 5 as libc::c_int;
}
/*
 * Determine which way the mana path follow
 */
#[no_mangle]
pub unsafe extern "C" fn get_mana_path_dir(mut y: libc::c_int,
                                           mut x: libc::c_int,
                                           mut oy: libc::c_int,
                                           mut ox: libc::c_int,
                                           mut pdir: libc::c_int,
                                           mut mana: libc::c_int)
 -> libc::c_int {
    let mut dir: [libc::c_int; 8] =
        [5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int];
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0 as libc::c_int;
    /* Check which case are allowed */
    if (*cave[(y - 1 as libc::c_int) as usize].offset(x as isize)).mana as
           libc::c_int == mana {
        let fresh0 = n;
        n = n + 1;
        dir[fresh0 as usize] = 8 as libc::c_int
    }
    if (*cave[(y + 1 as libc::c_int) as usize].offset(x as isize)).mana as
           libc::c_int == mana {
        let fresh1 = n;
        n = n + 1;
        dir[fresh1 as usize] = 2 as libc::c_int
    }
    if (*cave[y as usize].offset((x - 1 as libc::c_int) as isize)).mana as
           libc::c_int == mana {
        let fresh2 = n;
        n = n + 1;
        dir[fresh2 as usize] = 4 as libc::c_int
    }
    if (*cave[y as usize].offset((x + 1 as libc::c_int) as isize)).mana as
           libc::c_int == mana {
        let fresh3 = n;
        n = n + 1;
        dir[fresh3 as usize] = 6 as libc::c_int
    }
    /* If only 2 possibilities select the only good one */
    if n == 2 as libc::c_int {
        if invert_dir(yx_to_dir(y, x, oy, ox)) !=
               dir[0 as libc::c_int as usize] {
            return dir[0 as libc::c_int as usize]
        }
        if invert_dir(yx_to_dir(y, x, oy, ox)) !=
               dir[1 as libc::c_int as usize] {
            return dir[1 as libc::c_int as usize]
        }
        /* Should never happen */
        return 5 as libc::c_int
    }
    /* Check if it's not your last place */
    i = 0 as libc::c_int;
    while i < n {
        if oy == y + ddy[dir[i as usize] as usize] as libc::c_int &&
               ox == x + ddx[dir[i as usize] as usize] as libc::c_int {
            if dir[i as usize] == 8 as libc::c_int {
                dir[i as usize] = 2 as libc::c_int
            } else if dir[i as usize] == 2 as libc::c_int {
                dir[i as usize] = 8 as libc::c_int
            } else if dir[i as usize] == 6 as libc::c_int {
                dir[i as usize] = 4 as libc::c_int
            } else if dir[i as usize] == 4 as libc::c_int {
                dir[i as usize] = 6 as libc::c_int
            }
        }
        i += 1
    }
    /* Select the desired one if possible */
    i = 0 as libc::c_int;
    while i < n {
        if dir[i as usize] == pdir &&
               (*cave[(y + ddy[dir[i as usize] as usize] as libc::c_int) as
                          usize].offset((x +
                                             ddx[dir[i as usize] as usize] as
                                                 libc::c_int) as isize)).mana
                   as libc::c_int == mana {
            return dir[i as usize]
        }
        i += 1
    }
    /* If not select a random one */
    if n > 2 as libc::c_int {
        let mut nb: byte_hack = 200 as libc::c_int as byte_hack;
        while nb != 0 {
            nb = nb.wrapping_sub(1);
            r = Rand_div(n);
            if dir[r as usize] != 5 as libc::c_int &&
                   yx_to_dir(y, x, oy, ox) != dir[r as usize] {
                break ;
            }
        }
        return dir[r as usize]
    } else {
        /* If nothing is found return 5 */
        return 5 as libc::c_int
    };
}
/*
 * Determine the path taken by a projection.
 *
 * The projection will always start from the grid (y1,x1), and will travel
 * towards the grid (y2,x2), touching one grid per unit of distance along
 * the major axis, and stopping when it enters the destination grid or a
 * wall grid, or has travelled the maximum legal distance of "range".
 *
 * Note that "distance" in this function (as in the "update_view()" code)
 * is defined as "MAX(dy,dx) + MIN(dy,dx)/2", which means that the player
 * actually has an "octagon of projection" not a "circle of projection".
 *
 * The path grids are saved into the grid array pointed to by "gp", and
 * there should be room for at least "range" grids in "gp".  Note that
 * due to the way in which distance is calculated, this function normally
 * uses fewer than "range" grids for the projection path, so the result
 * of this function should never be compared directly to "range".  Note
 * that the initial grid (y1,x1) is never saved into the grid array, not
 * even if the initial grid is also the final grid.  XXX XXX XXX
 *
 * The "flg" flags can be used to modify the behavior of this function.
 *
 * In particular, the "PROJECT_STOP" and "PROJECT_THRU" flags have the same
 * semantics as they do for the "project" function, namely, that the path
 * will stop as soon as it hits a monster, or that the path will continue
 * through the destination grid, respectively.
 *
 * The "PROJECT_JUMP" flag, which for the "project()" function means to
 * start at a special grid (which makes no sense in this function), means
 * that the path should be "angled" slightly if needed to avoid any wall
 * grids, allowing the player to "target" any grid which is in "view".
 * This flag is non-trivial and has not yet been implemented, but could
 * perhaps make use of the "vinfo" array (above).  XXX XXX XXX
 *
 * This function returns the number of grids (if any) in the path.  This
 * function will return zero if and only if (y1,x1) and (y2,x2) are equal.
 *
 * This algorithm is similar to, but slightly different from, the one used
 * by "update_view_los()", and very different from the one used by "los()".
 */
#[no_mangle]
pub unsafe extern "C" fn project_path(mut gp: *mut u16b,
                                      mut range: libc::c_int,
                                      mut y1: libc::c_int,
                                      mut x1: libc::c_int,
                                      mut y2: libc::c_int,
                                      mut x2: libc::c_int,
                                      mut flg: libc::c_int) -> sint {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut mana: libc::c_int = 0 as libc::c_int;
    let mut dir: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0 as libc::c_int;
    /* Absolute */
    let mut ay: libc::c_int = 0;
    let mut ax: libc::c_int = 0;
    /* Offsets */
    let mut sy: libc::c_int = 0;
    let mut sx: libc::c_int = 0;
    /* Fractions */
    let mut frac: libc::c_int = 0;
    /* Scale factors */
    let mut full: libc::c_int = 0;
    let mut half: libc::c_int = 0;
    /* Slope */
    let mut m: libc::c_int = 0;
    /* No path necessary (or allowed) */
    if x1 == x2 && y1 == y2 { return 0 as libc::c_int }
    /* Hack -- to make a bolt/beam/ball follow a mana path */
    if flg & 0x4000 as libc::c_int != 0 {
        let mut oy: libc::c_int = y1;
        let mut ox: libc::c_int = x1;
        let mut pdir: libc::c_int = yx_to_dir(y2, x2, y1, x1);
        /* Get the mana path level to follow */
        mana = (*cave[y1 as usize].offset(x1 as isize)).mana as libc::c_int;
        /* Start */
        dir = get_mana_path_dir(y1, x1, y1, x1, pdir, mana);
        y = y1 + ddy[dir as usize] as libc::c_int;
        x = x1 + ddx[dir as usize] as libc::c_int;
        loop 
             /* Create the projection path */
             /* Save grid */
             {
            let fresh4 = n;
            n = n + 1;
            *gp.offset(fresh4 as isize) =
                (256 as libc::c_int * y + x) as u16b;
            /* Hack -- Check maximum range */
            if n >= range + 10 as libc::c_int { return n }
            /* Always stop at non-initial wall grids */
            if n > 0 as libc::c_int &&
                   ((*f_info.offset((*cave[y as
                                               usize].offset(x as isize)).feat
                                        as isize)).flags1 as libc::c_long &
                        0x2 as libc::c_long != 0 ||
                        !((*f_info.offset((*cave[y as
                                                     usize].offset(x as
                                                                       isize)).feat
                                              as isize)).flags1 as
                              libc::c_long & 0x10 as libc::c_long != 0 &&
                              (*cave[y as usize].offset(x as isize)).feat as
                                  libc::c_int != 0xaf as libc::c_int)) {
                return n
            }
            /* Sometimes stop at non-initial monsters/players */
            if flg & 0x8 as libc::c_int != 0 {
                if n > 0 as libc::c_int &&
                       (*cave[y as usize].offset(x as isize)).m_idx as
                           libc::c_int != 0 as libc::c_int {
                    return n
                }
            }
            /* Get the new direction */
            dir = get_mana_path_dir(y, x, oy, ox, pdir, mana);
            if dir == 5 as libc::c_int { return n }
            oy = y;
            ox = x;
            y += ddy[dir as usize] as libc::c_int;
            x += ddx[dir as usize] as libc::c_int
        }
    }
    /* Analyze "dy" */
    if y2 < y1 {
        ay = y1 - y2;
        sy = -(1 as libc::c_int)
    } else { ay = y2 - y1; sy = 1 as libc::c_int }
    /* Analyze "dx" */
    if x2 < x1 {
        ax = x1 - x2;
        sx = -(1 as libc::c_int)
    } else { ax = x2 - x1; sx = 1 as libc::c_int }
    /* Number of "units" in one "half" grid */
    half = ay * ax;
    /* Number of "units" in one "full" grid */
    full = half << 1 as libc::c_int;
    /* Vertical */
    if ay > ax {
        /* Start at tile edge */
        frac = ax * ax;
        /* Let m = ((dx/dy) * full) = (dx * dx * 2) = (frac * 2) */
        m = frac << 1 as libc::c_int;
        /* Start */
        y = y1 + sy;
        x = x1;
        loop 
             /* Create the projection path */
             /* Save grid */
             {
            let fresh5 = n;
            n = n + 1;
            *gp.offset(fresh5 as isize) =
                (256 as libc::c_int * y + x) as u16b;
            /* Hack -- Check maximum range */
            if n + (k >> 1 as libc::c_int) >= range { break ; }
            /* Sometimes stop at destination grid */
            if flg & 0x4 as libc::c_int == 0 {
                if x == x2 && y == y2 { break ; }
            }
            /* Always stop at non-initial wall grids */
            if n > 0 as libc::c_int &&
                   ((*f_info.offset((*cave[y as
                                               usize].offset(x as isize)).feat
                                        as isize)).flags1 as libc::c_long &
                        0x2 as libc::c_long != 0 ||
                        !((*f_info.offset((*cave[y as
                                                     usize].offset(x as
                                                                       isize)).feat
                                              as isize)).flags1 as
                              libc::c_long & 0x10 as libc::c_long != 0 &&
                              (*cave[y as usize].offset(x as isize)).feat as
                                  libc::c_int != 0xaf as libc::c_int)) &&
                   flg & 0x2000 as libc::c_int == 0 {
                break ;
            }
            /* Sometimes stop at non-initial monsters/players */
            if flg & 0x8 as libc::c_int != 0 {
                if n > 0 as libc::c_int &&
                       (*cave[y as usize].offset(x as isize)).m_idx as
                           libc::c_int != 0 as libc::c_int {
                    break ;
                }
            }
            /* Slant */
            if m != 0 {
                /* Advance (X) part 1 */
                frac += m;
                /* Horizontal change */
                if frac >= half {
                    /* Advance (X) part 2 */
                    x += sx;
                    /* Advance (X) part 3 */
                    frac -= full;
                    /* Track distance */
                    k += 1
                }
            }
            /* Advance (Y) */
            y += sy
        }
    } else if ax > ay {
        /* Horizontal */
        /* Start at tile edge */
        frac = ay * ay;
        /* Let m = ((dy/dx) * full) = (dy * dy * 2) = (frac * 2) */
        m = frac << 1 as libc::c_int;
        /* Start */
        y = y1;
        x = x1 + sx;
        loop 
             /* Create the projection path */
             /* Save grid */
             {
            let fresh6 = n;
            n = n + 1;
            *gp.offset(fresh6 as isize) =
                (256 as libc::c_int * y + x) as u16b;
            /* Hack -- Check maximum range */
            if n + (k >> 1 as libc::c_int) >= range { break ; }
            /* Sometimes stop at destination grid */
            if flg & 0x4 as libc::c_int == 0 {
                if x == x2 && y == y2 { break ; }
            }
            /* Always stop at non-initial wall grids */
            if n > 0 as libc::c_int &&
                   ((*f_info.offset((*cave[y as
                                               usize].offset(x as isize)).feat
                                        as isize)).flags1 as libc::c_long &
                        0x2 as libc::c_long != 0 ||
                        !((*f_info.offset((*cave[y as
                                                     usize].offset(x as
                                                                       isize)).feat
                                              as isize)).flags1 as
                              libc::c_long & 0x10 as libc::c_long != 0 &&
                              (*cave[y as usize].offset(x as isize)).feat as
                                  libc::c_int != 0xaf as libc::c_int)) &&
                   flg & 0x2000 as libc::c_int == 0 {
                break ;
            }
            /* Sometimes stop at non-initial monsters/players */
            if flg & 0x8 as libc::c_int != 0 {
                if n > 0 as libc::c_int &&
                       (*cave[y as usize].offset(x as isize)).m_idx as
                           libc::c_int != 0 as libc::c_int {
                    break ;
                }
            }
            /* Slant */
            if m != 0 {
                /* Advance (Y) part 1 */
                frac += m;
                /* Vertical change */
                if frac >= half {
                    /* Advance (Y) part 2 */
                    y += sy;
                    /* Advance (Y) part 3 */
                    frac -= full;
                    /* Track distance */
                    k += 1
                }
            }
            /* Advance (X) */
            x += sx
        }
    } else {
        /* Diagonal */
        /* Start */
        y = y1 + sy;
        x = x1 + sx;
        loop 
             /* Create the projection path */
             /* Save grid */
             {
            let fresh7 = n;
            n = n + 1;
            *gp.offset(fresh7 as isize) =
                (256 as libc::c_int * y + x) as u16b;
            /* Hack -- Check maximum range */
            if n + (n >> 1 as libc::c_int) >= range { break ; }
            /* Sometimes stop at destination grid */
            if flg & 0x4 as libc::c_int == 0 {
                if x == x2 && y == y2 { break ; }
            }
            /* Always stop at non-initial wall grids */
            if n > 0 as libc::c_int &&
                   ((*f_info.offset((*cave[y as
                                               usize].offset(x as isize)).feat
                                        as isize)).flags1 as libc::c_long &
                        0x2 as libc::c_long != 0 ||
                        !((*f_info.offset((*cave[y as
                                                     usize].offset(x as
                                                                       isize)).feat
                                              as isize)).flags1 as
                              libc::c_long & 0x10 as libc::c_long != 0 &&
                              (*cave[y as usize].offset(x as isize)).feat as
                                  libc::c_int != 0xaf as libc::c_int)) &&
                   flg & 0x2000 as libc::c_int == 0 {
                break ;
            }
            /* Sometimes stop at non-initial monsters/players */
            if flg & 0x8 as libc::c_int != 0 {
                if n > 0 as libc::c_int &&
                       (*cave[y as usize].offset(x as isize)).m_idx as
                           libc::c_int != 0 as libc::c_int {
                    break ;
                }
            }
            /* Advance (Y) */
            y += sy;
            /* Advance (X) */
            x += sx
        }
    }
    /* Length */
    return n;
}
/*
 * Mega-Hack -- track "affected" monsters (see "project()" comments)
 */
static mut project_m_n: libc::c_int = 0;
static mut project_m_x: libc::c_int = 0;
static mut project_m_y: libc::c_int = 0;
/*
 * We are called from "project()" to "damage" terrain features
 *
 * We are called both for "beam" effects and "ball" effects.
 *
 * The "r" parameter is the "distance from ground zero".
 *
 * Note that we determine if the player can "see" anything that happens
 * by taking into account: blindness, line-of-sight, and illumination.
 *
 * We return "TRUE" if the effect of the projection is "obvious".
 *
 * XXX XXX XXX We also "see" grids which are "memorized", probably a hack
 *
 * XXX XXX XXX Perhaps we should affect doors?
 */
unsafe extern "C" fn project_f(mut who: libc::c_int, mut r: libc::c_int,
                               mut y: libc::c_int, mut x: libc::c_int,
                               mut dam: libc::c_int, mut typ: libc::c_int)
 -> bool_ {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    let mut obvious: bool_ = 0 as libc::c_int as bool_;
    let mut flag: bool_ = 0 as libc::c_int as bool_;
    let mut seen: bool_ = 0;
    /* XXX XXX XXX */
    who = if who != 0 { who } else { 0 as libc::c_int };
    /* Reduce damage by distance */
    dam = (dam + r) / (r + 1 as libc::c_int);
    /* Remember if the grid is with the LoS of player */
    seen =
        ((*cave[y as usize].offset(x as isize)).info as libc::c_int &
             0x10 as libc::c_int != 0 as libc::c_int) as libc::c_int as bool_;
    /* Analyze the type */
    match typ {
        1 | 21 | 26 | 85 | 86 | 87 | 89 => { }
        4 | 28 => {
            let mut percent: libc::c_int =
                if (*c_ptr).feat as libc::c_int == 4 as libc::c_int {
                    20 as libc::c_int
                } else { 50 as libc::c_int };
            /* Only affects "boring" grids */
            if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                   isize)).flags1 as libc::c_long &
                   0x10 as libc::c_long != 0 &&
                   (*f_info.offset((*cave[y as usize].offset(x as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x100 as libc::c_long == 0 {
                if Rand_div(100 as libc::c_int) < percent {
                    cave_set_feat(y, x, 0x5a as libc::c_int);
                    if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                }
            }
        }
        103 => {
            let mut y1: libc::c_int =
                Rand_div(cur_hgt as s32b) + 1 as libc::c_int -
                    1 as libc::c_int;
            let mut x1: libc::c_int =
                Rand_div(cur_wid as s32b) + 1 as libc::c_int -
                    1 as libc::c_int;
            let mut y2: libc::c_int = y1;
            let mut x2: libc::c_int = x1;
            let mut tries: libc::c_int = 1000 as libc::c_int;
            /*
			 * Avoid "interesting" and/or permanent features
			 *
			 * If we can make sure that all the "permanent" features
			 * have the remember flag set as well, we can simplify
			 * the conditional... -- pelpel
			 */
            if !(!((*f_info.offset((*cave[y as usize].offset(x as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x10 as libc::c_long != 0 &&
                       (*f_info.offset((*cave[y as
                                                  usize].offset(x as
                                                                    isize)).feat
                                           as isize)).flags1 as libc::c_long &
                           0x100 as libc::c_long == 0) ||
                     (*f_info.offset((*cave[y as
                                                usize].offset(x as
                                                                  isize)).feat
                                         as isize)).flags1 as libc::c_long &
                         0x40 as libc::c_long != 0) {
                /* Destination shouldn't be "interesting" either */
                while tries != 0 &&
                          (!((*f_info.offset((*cave[y2 as
                                                        usize].offset(x2 as
                                                                          isize)).feat
                                                 as isize)).flags1 as
                                 libc::c_long & 0x10 as libc::c_long != 0 &&
                                 (*f_info.offset((*cave[y2 as
                                                            usize].offset(x2
                                                                              as
                                                                              isize)).feat
                                                     as isize)).flags1 as
                                     libc::c_long & 0x100 as libc::c_long ==
                                     0) ||
                               (*f_info.offset((*cave[y2 as
                                                          usize].offset(x2 as
                                                                            isize)).feat
                                                   as isize)).flags1 as
                                   libc::c_long & 0x40 as libc::c_long != 0) {
                    y1 =
                        Rand_div(cur_hgt as s32b) + 1 as libc::c_int -
                            1 as libc::c_int;
                    y2 = y1;
                    x1 =
                        Rand_div(cur_wid as s32b) + 1 as libc::c_int -
                            1 as libc::c_int;
                    x2 = x1;
                    scatter(&mut y2, &mut x2, y1, x1, 20 as libc::c_int);
                    tries -= 1
                }
                /* No boarding grids found */
                if !(tries == 0) {
                    /* Place a pair of between gates */
                    cave_set_feat(y, x, 0xa0 as libc::c_int);
                    (*cave[y as usize].offset(x as isize)).special =
                        (x2 + (y2 << 8 as libc::c_int)) as s16b;
                    cave_set_feat(y2, x2, 0xa0 as libc::c_int);
                    (*cave[y2 as usize].offset(x2 as isize)).special =
                        (x + (y << 8 as libc::c_int)) as s16b;
                    if seen != 0 {
                        obvious = 1 as libc::c_int as bool_;
                        note_spot(y, x);
                    }
                    if (*cave[y2 as usize].offset(x2 as isize)).info as
                           libc::c_int & 0x10 as libc::c_int !=
                           0 as libc::c_int {
                        obvious = 1 as libc::c_int as bool_;
                        note_spot(y2, x2);
                    }
                }
            }
        }
        5 | 27 | 12 | 79 | 80 => {
            /* "Permanent" features will stay */
            if !((*f_info.offset((*c_ptr).feat as isize)).flags1 as
                     libc::c_long & 0x40 as libc::c_long != 0) {
                /* Trees *will* burn */
                if (*c_ptr).feat as libc::c_int == 0x60 as libc::c_int {
                    cave_set_feat(y, x, 0x5c as libc::c_int);
                    /* Silly thing to destroy trees when a yavanna worshipper */
                    inc_piety(5 as libc::c_int, -(50 as libc::c_int));
                    if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                }
                /* Trees *will* burn */
                if (*c_ptr).feat as libc::c_int == 0xca as libc::c_int {
                    cave_set_feat(y, x, 212 as libc::c_int);
                    /* Silly thing to destroy trees when a yavanna worshipper */
                    inc_piety(5 as libc::c_int, -(60 as libc::c_int));
                    if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                } else if (*c_ptr).feat as libc::c_int == 0x5a as libc::c_int
                 {
                    let mut k: libc::c_int = Rand_div(100 as libc::c_int);
                    if !(k >= 30 as libc::c_int) {
                        /* Ice can melt (chance == 30%) */
                        /* Melt ice */
                        if k < 10 as libc::c_int {
                            cave_set_feat(y, x, 0x58 as libc::c_int);
                        } else if k < 30 as libc::c_int {
                            cave_set_feat(y, x, 0x54 as libc::c_int);
                        }
                        if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                    }
                } else if (*f_info.offset((*c_ptr).feat as isize)).flags1 as
                              libc::c_long & 0x10 as libc::c_long != 0 {
                    let mut k_0: libc::c_int = Rand_div(100 as libc::c_int);
                    if !(k_0 >= 25 as libc::c_int) {
                        /* Floors can become ash or lava (chance == 25%) */
                        /* Burn floor */
                        if k_0 < 10 as libc::c_int {
                            cave_set_feat(y, x, 0x56 as libc::c_int);
                        } else if k_0 < 25 as libc::c_int {
                            cave_set_feat(y, x, 0x5d as libc::c_int);
                        }
                        if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                    }
                } else if (*c_ptr).feat as libc::c_int == 0x62 as libc::c_int
                              ||
                              (*c_ptr).feat as libc::c_int ==
                                  0x63 as libc::c_int ||
                              (*c_ptr).feat as libc::c_int ==
                                  0x64 as libc::c_int {
                    let mut k_1: libc::c_int = Rand_div(100 as libc::c_int);
                    /* Sandwall can be turned into glass (chance == 30%) */
                    /* Glass it */
                    if k_1 < 30 as libc::c_int {
                        cave_set_feat(y, x, 0xbc as libc::c_int);
                        /* Visibility change */
                        (*p_ptr).update =
                            ((*p_ptr).update as libc::c_long |
                                 (0x100000 as libc::c_long |
                                      0x1000000 as libc::c_long |
                                      0x200000 as libc::c_long)) as u32b;
                        if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                    }
                }
            }
        }
        13 | 14 => {
            let mut p1: libc::c_int = 0 as libc::c_int;
            let mut p2: libc::c_int = 0 as libc::c_int;
            let mut f1: libc::c_int = 0 as libc::c_int;
            let mut f2: libc::c_int = 0 as libc::c_int;
            let mut f: libc::c_int = 0 as libc::c_int;
            let mut k_2: libc::c_int = 0;
            /* "Permanent" features will stay */
            if !((*f_info.offset((*c_ptr).feat as isize)).flags1 as
                     libc::c_long & 0x40 as libc::c_long != 0) {
                /* Needs more than 30 damage */
                if !(dam < 30 as libc::c_int) {
                    if (*c_ptr).feat as libc::c_int == 0x1 as libc::c_int ||
                           (*c_ptr).feat as libc::c_int == 0x58 as libc::c_int
                           ||
                           (*c_ptr).feat as libc::c_int == 0x59 as libc::c_int
                       {
                        /* 35% chance to create shallow water */
                        p1 = 35 as libc::c_int;
                        f1 = 0x54 as libc::c_int;
                        /* 5% chance to create deep water */
                        p2 = 40 as libc::c_int;
                        f2 = 0xbb as libc::c_int
                    } else if (*c_ptr).feat as libc::c_int ==
                                  0x32 as libc::c_int ||
                                  (*c_ptr).feat as libc::c_int ==
                                      0x34 as libc::c_int ||
                                  (*c_ptr).feat as libc::c_int ==
                                      0x36 as libc::c_int ||
                                  (*c_ptr).feat as libc::c_int ==
                                      0x56 as libc::c_int {
                        /* 15% chance to convert it to normal floor */
                        p1 = 15 as libc::c_int;
                        f1 = 0x1 as libc::c_int
                    } else if (*c_ptr).feat as libc::c_int ==
                                  0x55 as libc::c_int {
                        /* 10% chance to convert it to shallow lava */
                        p1 = 10 as libc::c_int;
                        f1 = 0x56 as libc::c_int;
                        /* 5% chance to convert it to normal floor */
                        p2 = 15 as libc::c_int;
                        f2 = 0x1 as libc::c_int
                    } else if (*c_ptr).feat as libc::c_int ==
                                  0x54 as libc::c_int ||
                                  (*c_ptr).feat as libc::c_int ==
                                      0x57 as libc::c_int {
                        /* 10% chance to convert it to deep water */
                        p1 = 10 as libc::c_int;
                        f1 = 0xbb as libc::c_int
                    }
                    k_2 = Rand_div(100 as libc::c_int);
                    if k_2 < p1 { f = f1 } else if k_2 < p2 { f = f2 }
                    if f != 0 {
                        if f == 0x1 as libc::c_int {
                            place_floor_convert_glass(y, x);
                        } else { cave_set_feat(y, x, f); }
                        if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                    }
                }
            }
        }
        31 | 33 | 3 | 20 | 34 | 23 | 73 => {
            /* "Permanent" features will stay */
            if !((*f_info.offset((*c_ptr).feat as isize)).flags1 as
                     libc::c_long & 0x40 as libc::c_long != 0) {
                if (*c_ptr).feat as libc::c_int == 0x60 as libc::c_int ||
                       (*c_ptr).feat as libc::c_int == 0xca as libc::c_int {
                    /* Destroy the grid */
                    cave_set_feat(y, x, 0x5c as libc::c_int);
                    /* Silly thing to destroy trees when a yavanna worshipper */
                    inc_piety(5 as libc::c_int, -(50 as libc::c_int));
                    if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                }
            }
        }
        81 => {
            /* "Permanent" features will stay */
            if !((*f_info.offset((*c_ptr).feat as isize)).flags1 as
                     libc::c_long & 0x40 as libc::c_long != 0) {
                if ((*c_ptr).feat as libc::c_int == 0x60 as libc::c_int ||
                        (*c_ptr).feat as libc::c_int == 0xca as libc::c_int ||
                        (*f_info.offset((*c_ptr).feat as isize)).flags1 as
                            libc::c_long & 0x10 as libc::c_long != 0) &&
                       Rand_div(100 as libc::c_int) < 30 as libc::c_int {
                    /* Flow change */
                    if (*c_ptr).feat as libc::c_int == 0x60 as libc::c_int {
                        (*p_ptr).update =
                            ((*p_ptr).update as libc::c_long |
                                 0x10000000 as libc::c_long) as u32b
                    }
                    cave_set_feat(y, x, 0x5d as libc::c_int);
                    /* Silly thing to destroy trees when a yavanna worshipper */
                    if (*c_ptr).feat as libc::c_int == 0x60 as libc::c_int ||
                           (*c_ptr).feat as libc::c_int == 0xca as libc::c_int
                       {
                        inc_piety(5 as libc::c_int, -(50 as libc::c_int));
                    }
                    /* Visibility change */
                    (*p_ptr).update =
                        ((*p_ptr).update as libc::c_long |
                             (0x100000 as libc::c_long |
                                  0x1000000 as libc::c_long |
                                  0x200000 as libc::c_long)) as u32b;
                    if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                }
            }
        }
        42 => {
            /* Destroy Traps (and Locks) */
            /* Destroy normal traps and disarm monster traps */
            if (*c_ptr).t_idx as libc::c_int != 0 as libc::c_int ||
                   (*c_ptr).feat as libc::c_int == 0xaf as libc::c_int {
                /* Check line of sight */
                if (*cave[y as usize].offset(x as isize)).info as libc::c_int
                       & 0x20 as libc::c_int != 0 as libc::c_int {
                    msg_print(b"There is a bright flash of light!\x00" as
                                  *const u8 as *const libc::c_char);
                    obvious = 1 as libc::c_int as bool_
                }
                /* Forget the trap */
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int &
                         !(0x1 as libc::c_int | 0x100 as libc::c_int)) as
                        u16b;
                /* Destroy normal traps */
                (*c_ptr).t_idx = 0 as libc::c_int as s16b;
                /* Disarm monster traps */
                if (*c_ptr).feat as libc::c_int == 0xaf as libc::c_int {
                    (*c_ptr).special2 = 0 as libc::c_int as s16b;
                    (*c_ptr).special = (*c_ptr).special2;
                    /* Remove the feature */
                    if (*f_info.offset((*c_ptr).feat as isize)).flags1 as
                           libc::c_long & 0x40 as libc::c_long == 0 {
                        place_floor_convert_glass(y, x);
                    }
                }
                /* Hack -- Force redraw */
                note_spot(y, x);
                lite_spot(y, x);
            } else if (*c_ptr).feat as libc::c_int == 0x30 as libc::c_int ||
                          (*c_ptr).feat as libc::c_int >=
                              0x20 as libc::c_int + 0x1 as libc::c_int &&
                              (*c_ptr).feat as libc::c_int <=
                                  0x20 as libc::c_int + 0x7 as libc::c_int {
                /* Secret / Locked doors are found and unlocked */
                /* Check line of sound */
                if (*cave[y as usize].offset(x as isize)).info as libc::c_int
                       & 0x20 as libc::c_int != 0 as libc::c_int {
                    msg_print(b"Click!\x00" as *const u8 as
                                  *const libc::c_char);
                    obvious = 1 as libc::c_int as bool_
                }
                /* Remove feature mimic */
                (*cave[y as usize].offset(x as isize)).mimic =
                    0 as libc::c_int as byte_hack;
                /* Unlock the door */
                cave_set_feat(y, x, 0x20 as libc::c_int + 0 as libc::c_int);
            }
        }
        41 => {
            /* Destroy Doors (and traps) */
            /* Destroy all doors and traps, and disarm monster traps */
            if (*c_ptr).feat as libc::c_int == 0x4 as libc::c_int ||
                   (*c_ptr).feat as libc::c_int == 0x5 as libc::c_int ||
                   (*c_ptr).t_idx as libc::c_int != 0 as libc::c_int ||
                   (*c_ptr).feat as libc::c_int == 0xaf as libc::c_int ||
                   (*c_ptr).feat as libc::c_int >= 0x20 as libc::c_int &&
                       (*c_ptr).feat as libc::c_int <= 0x2f as libc::c_int {
                /* Check line of sight */
                if (*cave[y as usize].offset(x as isize)).info as libc::c_int
                       & 0x20 as libc::c_int != 0 as libc::c_int {
                    /* Message */
                    msg_print(b"There is a bright flash of light!\x00" as
                                  *const u8 as *const libc::c_char);
                    obvious = 1 as libc::c_int as bool_;
                    /* Visibility change */
                    if (*c_ptr).feat as libc::c_int >= 0x20 as libc::c_int &&
                           (*c_ptr).feat as libc::c_int <= 0x2f as libc::c_int
                       {
                        /* Update some things */
                        (*p_ptr).update =
                            ((*p_ptr).update as libc::c_long |
                                 (0x100000 as libc::c_long |
                                      0x1000000 as libc::c_long |
                                      0x200000 as libc::c_long)) as u32b
                    }
                }
                /* Forget the door */
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int &
                         !(0x1 as libc::c_int | 0x100 as libc::c_int)) as
                        u16b;
                /* Remove normal traps */
                (*c_ptr).t_idx = 0 as libc::c_int as s16b;
                /* Disarm monster traps */
                if (*c_ptr).feat as libc::c_int == 0xaf as libc::c_int {
                    (*c_ptr).special2 = 0 as libc::c_int as s16b;
                    (*c_ptr).special = (*c_ptr).special2
                }
                /* Remove the feature */
                if (*f_info.offset((*c_ptr).feat as isize)).flags1 as
                       libc::c_long & 0x40 as libc::c_long == 0 {
                    place_floor_convert_glass(y, x);
                }
                /* Hack -- Force redraw */
                note_spot(y, x);
                lite_spot(y, x);
            }
        }
        88 => {
            /* Jams a door (as if with a spike) */
            if (*c_ptr).feat as libc::c_int >= 0x20 as libc::c_int &&
                   (*c_ptr).feat as libc::c_int <= 0x2f as libc::c_int {
                /* Convert "locked" to "stuck" XXX XXX XXX */
                if ((*c_ptr).feat as libc::c_int) <
                       0x20 as libc::c_int + 0x8 as libc::c_int {
                    (*c_ptr).feat =
                        ((*c_ptr).feat as libc::c_int + 0x8 as libc::c_int) as
                            byte_hack
                }
                /* Add one spike to the door */
                if ((*c_ptr).feat as libc::c_int) < 0x2f as libc::c_int {
                    (*c_ptr).feat = (*c_ptr).feat.wrapping_add(1)
                }
                /* Check line of sight */
                if (*cave[y as usize].offset(x as isize)).info as libc::c_int
                       & 0x20 as libc::c_int != 0 as libc::c_int {
                    /* Message */
                    msg_print(b"The door seems stuck.\x00" as *const u8 as
                                  *const libc::c_char);
                    obvious = 1 as libc::c_int as bool_
                }
            }
        }
        40 => {
            /* Non-walls (etc) */
            if !((*f_info.offset((*cave[y as usize].offset(x as isize)).feat
                                     as isize)).flags1 as libc::c_long &
                     0x10 as libc::c_long != 0 &&
                     (*cave[y as usize].offset(x as isize)).feat as
                         libc::c_int != 0xaf as libc::c_int) {
                /* "Permanent" features will stay */
                if !((*f_info.offset((*c_ptr).feat as isize)).flags1 as
                         libc::c_long & 0x40 as libc::c_long != 0) {
                    /* Granite -- How about other wall types? */
                    if (*c_ptr).feat as libc::c_int >= 0x38 as libc::c_int &&
                           (*c_ptr).feat as libc::c_int <= 0x3b as libc::c_int
                       {
                        /* Message */
                        if (*c_ptr).info as libc::c_int & 0x1 as libc::c_int
                               != 0 {
                            msg_print(b"The wall turns into mud!\x00" as
                                          *const u8 as *const libc::c_char);
                            obvious = 1 as libc::c_int as bool_
                        }
                        /* Forget the wall */
                        (*c_ptr).info =
                            ((*c_ptr).info as libc::c_int &
                                 !(0x1 as libc::c_int)) as u16b;
                        /* Destroy the wall */
                        cave_set_feat(y, x, 0x1 as libc::c_int);
                    } else if (*c_ptr).feat as libc::c_int >=
                                  0x34 as libc::c_int &&
                                  (*c_ptr).feat as libc::c_int <=
                                      0x37 as libc::c_int ||
                                  (*c_ptr).feat as libc::c_int ==
                                      0x64 as libc::c_int {
                        /* Quartz / Magma / Sand with treasure */
                        /* Message */
                        if (*c_ptr).info as libc::c_int & 0x1 as libc::c_int
                               != 0 {
                            msg_print(b"The vein turns into mud!\x00" as
                                          *const u8 as *const libc::c_char);
                            msg_print(b"You have found something!\x00" as
                                          *const u8 as *const libc::c_char);
                            obvious = 1 as libc::c_int as bool_
                        }
                        /* Forget the wall */
                        (*c_ptr).info =
                            ((*c_ptr).info as libc::c_int &
                                 !(0x1 as libc::c_int)) as u16b;
                        /* Destroy the wall */
                        cave_set_feat(y, x, 0x1 as libc::c_int);
                        /* Place some gold */
                        place_gold(y, x);
                    } else if (*c_ptr).feat as libc::c_int ==
                                  0x32 as libc::c_int ||
                                  (*c_ptr).feat as libc::c_int ==
                                      0x33 as libc::c_int ||
                                  (*c_ptr).feat as libc::c_int ==
                                      0x62 as libc::c_int ||
                                  (*c_ptr).feat as libc::c_int ==
                                      0x63 as libc::c_int {
                        /* Quartz / Magma / Sand */
                        /* Message */
                        if (*c_ptr).info as libc::c_int & 0x1 as libc::c_int
                               != 0 {
                            msg_print(b"The vein turns into mud!\x00" as
                                          *const u8 as *const libc::c_char);
                            obvious = 1 as libc::c_int as bool_
                        }
                        /* Forget the wall */
                        (*c_ptr).info =
                            ((*c_ptr).info as libc::c_int &
                                 !(0x1 as libc::c_int)) as u16b;
                        /* Destroy the wall */
                        cave_set_feat(y, x, 0x1 as libc::c_int);
                    } else if (*c_ptr).feat as libc::c_int ==
                                  0x31 as libc::c_int {
                        /* Rubble */
                        /* Message */
                        if (*c_ptr).info as libc::c_int & 0x1 as libc::c_int
                               != 0 {
                            msg_print(b"The rubble turns into mud!\x00" as
                                          *const u8 as *const libc::c_char);
                            obvious = 1 as libc::c_int as bool_
                        }
                        /* Forget the wall */
                        (*c_ptr).info =
                            ((*c_ptr).info as libc::c_int &
                                 !(0x1 as libc::c_int)) as u16b;
                        /* Destroy the rubble */
                        cave_set_feat(y, x, 0x1 as libc::c_int);
                        /* Hack -- place an object */
                        if Rand_div(100 as libc::c_int) < 10 as libc::c_int {
                            /* Found something */
                            if seen != 0 {
                                msg_print(b"There was something buried in the rubble!\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                obvious = 1 as libc::c_int as bool_
                            }
                            /* Place gold */
                            place_object(y, x, 0 as libc::c_int as bool_,
                                         0 as libc::c_int as bool_,
                                         5 as libc::c_int);
                        }
                    } else if (*c_ptr).feat as libc::c_int >=
                                  0x20 as libc::c_int &&
                                  (*c_ptr).feat as libc::c_int <=
                                      0x2f as libc::c_int ||
                                  (*c_ptr).feat as libc::c_int ==
                                      0x30 as libc::c_int {
                        /* Destroy doors (and secret doors) */
                        /* Hack -- special message */
                        if (*c_ptr).info as libc::c_int & 0x1 as libc::c_int
                               != 0 {
                            msg_print(b"The door turns into mud!\x00" as
                                          *const u8 as *const libc::c_char);
                            obvious = 1 as libc::c_int as bool_
                        }
                        /* Forget the wall */
                        (*c_ptr).info =
                            ((*c_ptr).info as libc::c_int &
                                 !(0x1 as libc::c_int)) as u16b;
                        /* Remove mimic */
                        (*c_ptr).mimic = 0 as libc::c_int as byte_hack;
                        /* Destroy the feature */
                        cave_set_feat(y, x, 0x1 as libc::c_int);
                    }
                    /* Update some things */
                    (*p_ptr).update =
                        ((*p_ptr).update as libc::c_long |
                             (0x100000 as libc::c_long |
                                  0x10000000 as libc::c_long |
                                  0x1000000 as libc::c_long |
                                  0x200000 as libc::c_long)) as u32b
                }
            }
        }
        46 => {
            /* Require a "naked" floor grid */
            if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                   isize)).flags1 as libc::c_long &
                   0x10 as libc::c_long != 0 &&
                   (*cave[y as usize].offset(x as isize)).feat as libc::c_int
                       != 0xaf as libc::c_int &&
                   (*cave[y as usize].offset(x as isize)).o_idx as libc::c_int
                       == 0 as libc::c_int &&
                   (*f_info.offset((*cave[y as usize].offset(x as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x40 as libc::c_long == 0 {
                if !((*f_info.offset((*c_ptr).feat as isize)).flags1 as
                         libc::c_long & 0x40 as libc::c_long != 0) {
                    /* Create a closed door */
                    cave_set_feat(y, x,
                                  0x20 as libc::c_int + 0 as libc::c_int);
                    /* Observe */
                    if (*c_ptr).info as libc::c_int & 0x1 as libc::c_int != 0
                       {
                        obvious = 1 as libc::c_int as bool_
                    }
                    /* Update some things */
                    (*p_ptr).update =
                        ((*p_ptr).update as libc::c_long |
                             (0x100000 as libc::c_long |
                                  0x1000000 as libc::c_long |
                                  0x200000 as libc::c_long)) as u32b
                }
            }
        }
        47 => {
            /* Require a "naked" floor grid */
            if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                   isize)).flags1 as libc::c_long &
                   0x10 as libc::c_long != 0 &&
                   (*cave[y as usize].offset(x as isize)).feat as libc::c_int
                       != 0xaf as libc::c_int &&
                   (*cave[y as usize].offset(x as isize)).o_idx as libc::c_int
                       == 0 as libc::c_int &&
                   (*f_info.offset((*cave[y as usize].offset(x as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x40 as libc::c_long == 0 {
                /* Place a trap */
                place_trap(y, x);
            }
        }
        74 => {
            /* Require a "naked" floor grid */
            if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                   isize)).flags1 as libc::c_long &
                   0x10 as libc::c_long != 0 &&
                   (*cave[y as usize].offset(x as isize)).feat as libc::c_int
                       != 0xaf as libc::c_int &&
                   (*cave[y as usize].offset(x as isize)).o_idx as libc::c_int
                       == 0 as libc::c_int &&
                   (*f_info.offset((*cave[y as usize].offset(x as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x40 as libc::c_long == 0 {
                if !((*f_info.offset((*c_ptr).feat as isize)).flags1 as
                         libc::c_long & 0x40 as libc::c_long != 0) {
                    cave_set_feat(y, x, 0x3 as libc::c_int);
                    if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                }
            }
        }
        76 => {
            /* Require a "naked" floor grid */
            if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                   isize)).flags1 as libc::c_long &
                   0x10 as libc::c_long != 0 &&
                   (*cave[y as usize].offset(x as isize)).feat as libc::c_int
                       != 0xaf as libc::c_int &&
                   (*cave[y as usize].offset(x as isize)).o_idx as libc::c_int
                       == 0 as libc::c_int &&
                   (*f_info.offset((*cave[y as usize].offset(x as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x40 as libc::c_long == 0 {
                if !((*f_info.offset((*c_ptr).feat as isize)).flags1 as
                         libc::c_long & 0x40 as libc::c_long != 0) {
                    if !((*f_info.offset((*c_ptr).feat as isize)).flags1 as
                             libc::c_long & 0x10 as libc::c_long == 0) {
                        /* Place a wall */
                        cave_set_feat(y, x, 0x38 as libc::c_int);
                        if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                        /* Update some things */
                        (*p_ptr).update =
                            ((*p_ptr).update as libc::c_long |
                                 (0x100000 as libc::c_long |
                                      0x1000000 as libc::c_long |
                                      0x200000 as libc::c_long)) as u32b
                    }
                }
            }
        }
        104 => {
            if dam >= 256 as libc::c_int {
                /* With erase mana */
                /* Absorb some of the mana of the grid */
                (*p_ptr).csp =
                    ((*p_ptr).csp as libc::c_int +
                         (*cave[y as usize].offset(x as isize)).mana as
                             libc::c_int / 80 as libc::c_int) as s16b;
                if (*p_ptr).csp as libc::c_int > (*p_ptr).msp as libc::c_int {
                    (*p_ptr).csp = (*p_ptr).msp
                }
                /* Set the new amount */
                (*cave[y as usize].offset(x as isize)).mana =
                    (dam - 256 as libc::c_int) as byte_hack
            } else {
                /* Without erase mana */
                let mut amt: libc::c_int =
                    (*cave[y as usize].offset(x as isize)).mana as libc::c_int
                        + dam;
                /* Check if not overflow */
                if amt > 255 as libc::c_int { amt = 255 as libc::c_int }
                /* Set the new amount */
                (*cave[y as usize].offset(x as isize)).mana = amt as byte_hack
            }
        }
        101 => {
            if !((*f_info.offset((*c_ptr).feat as isize)).flags1 as
                     libc::c_long & 0x40 as libc::c_long != 0) {
                /* Shallow Lava */
                if dam == 1 as libc::c_int {
                    /* Require a "naked" floor grid */
                    if (*f_info.offset((*cave[y as
                                                  usize].offset(x as
                                                                    isize)).feat
                                           as isize)).flags1 as libc::c_long &
                           0x10 as libc::c_long != 0 &&
                           (*cave[y as usize].offset(x as isize)).feat as
                               libc::c_int != 0xaf as libc::c_int &&
                           (*f_info.offset((*cave[y as
                                                      usize].offset(x as
                                                                        isize)).feat
                                               as isize)).flags1 as
                               libc::c_long & 0x40 as libc::c_long == 0 &&
                           (*cave[y as usize].offset(x as isize)).o_idx as
                               libc::c_int == 0 as libc::c_int &&
                           (*cave[y as usize].offset(x as isize)).m_idx as
                               libc::c_int == 0 as libc::c_int {
                        /* Place a shallow lava */
                        cave_set_feat(y, x, 0x56 as libc::c_int);
                        if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                    }
                } else if !((*f_info.offset((*cave[y as
                                                       usize].offset(x as
                                                                         isize)).feat
                                                as isize)).flags1 as
                                libc::c_long & 0x40 as libc::c_long != 0 ||
                                dam == 0) {
                    /* Deep Lava */
                    /* Require a "naked" floor grid */
                    /* Place a deep lava */
                    cave_set_feat(y, x, 0x55 as libc::c_int);
                    if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                    /* Dam is used as a counter for the number of grid to convert */
                    dam -= 1
                }
            }
        }
        17 | 15 => {
            /* Lite up the grid */
            /* Turn on the light */
            (*c_ptr).info =
                ((*c_ptr).info as libc::c_int | 0x2 as libc::c_int) as u16b;
            /* Notice */
            note_spot(y, x);
            /* Redraw */
            lite_spot(y, x);
            /* Observe */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /*
			 * Mega-Hack -- Update the monster in the affected grid
			 * This allows "spear of light" (etc) to work "correctly"
			 */
            if (*c_ptr).m_idx != 0 {
                update_mon((*c_ptr).m_idx as libc::c_int,
                           0 as libc::c_int as bool_);
            }
        }
        18 | 16 => {
            /* Darken the grid */
            /* Notice */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Turn off the light. */
            (*c_ptr).info =
                ((*c_ptr).info as libc::c_int & !(0x2 as libc::c_int)) as
                    u16b;
            /* Hack -- Forget "boring" grids */
            if (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long
                   & 0x10 as libc::c_long != 0 &&
                   (*f_info.offset((*c_ptr).feat as isize)).flags1 as
                       libc::c_long & 0x100 as libc::c_long == 0 {
                /* Forget */
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int & !(0x1 as libc::c_int)) as
                        u16b;
                /* Notice */
                note_spot(y, x);
            }
            /* Redraw */
            lite_spot(y, x);
            /*
			 * Mega-Hack -- Update the monster in the affected grid
			 * This allows "spear of light" (etc) to work "correctly"
			 */
            if (*c_ptr).m_idx != 0 {
                update_mon((*c_ptr).m_idx as libc::c_int,
                           0 as libc::c_int as bool_);
            }
        }
        94 => {
            let mut t: libc::c_int = 0;
            /* Lose room and vault */
            (*c_ptr).info =
                ((*c_ptr).info as libc::c_int &
                     !(0x8 as libc::c_int | 0x4 as libc::c_int)) as u16b;
            /* Lose light and knowledge */
            (*c_ptr).info =
                ((*c_ptr).info as libc::c_int &
                     !(0x1 as libc::c_int | 0x2 as libc::c_int)) as u16b;
            /* Hack -- Notice player affect */
            if x == (*p_ptr).px as libc::c_int &&
                   y == (*p_ptr).py as libc::c_int {
                /* Hurt the player later */
                flag = 1 as libc::c_int as bool_
            } else {
                /* Delete the monster (if any) */
                delete_monster(y, x);
                if !((*f_info.offset((*c_ptr).feat as isize)).flags1 as
                         libc::c_long & 0x40 as libc::c_long != 0) {
                    /* Destroy "valid" grids */
                    if cave_valid_bold(y, x) != 0 {
                        /* Delete objects */
                        delete_object(y, x);
                        /* Wall (or floor) type */
                        t = Rand_div(200 as libc::c_int);
                        /* Granite */
                        if t < 20 as libc::c_int {
                            /* Create granite wall */
                            cave_set_feat(y, x, 0x38 as libc::c_int);
                        } else if t < 60 as libc::c_int {
                            /* Quartz */
                            /* Create quartz vein */
                            cave_set_feat(y, x, 0x33 as libc::c_int);
                        } else if t < 90 as libc::c_int {
                            /* Magma */
                            /* Create magma vein */
                            cave_set_feat(y, x, 0x32 as libc::c_int);
                        } else if t < 110 as libc::c_int {
                            /* Sand */
                            /* Create sand vein */
                            cave_set_feat(y, x, 0x62 as libc::c_int);
                        } else {
                            /* Floor */
                            /* Create floor */
                            cave_set_feat(y, x, 0x1 as libc::c_int);
                        }
                        /* Visibility and flow changes */
                        (*p_ptr).update =
                            ((*p_ptr).update as libc::c_long |
                                 (0x100000 as libc::c_long |
                                      0x10000000 as libc::c_long |
                                      0x1000000 as libc::c_long |
                                      0x200000 as libc::c_long)) as u32b
                    }
                    obvious = 1 as libc::c_int as bool_
                }
            }
        }
        _ => {
            /* Hooks! */
            if process_hooks_ret(59 as libc::c_int,
                                 b"dd\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char,
                                 b"(s,d,d,d,d,d,d)\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 b"grid\x00" as *const u8 as
                                     *const libc::c_char, who, typ, dam, r, y,
                                 x) != 0 {
                obvious =
                    process_hooks_return[0 as libc::c_int as usize].num as
                        bool_;
                flag =
                    process_hooks_return[1 as libc::c_int as usize].num as
                        bool_
            }
        }
    }
    /* Hack -- Affect player */
    if flag != 0 {
        /* Message */
        msg_print(b"There is a searing blast of light!\x00" as *const u8 as
                      *const libc::c_char);
        /* Blind the player */
        if (*p_ptr).resist_blind == 0 && (*p_ptr).resist_lite == 0 {
            /* Become blind */
            set_blind((*p_ptr).blind as libc::c_int + 10 as libc::c_int +
                          (Rand_div(10 as libc::c_int) + 1 as libc::c_int));
        }
    }
    /* Return "Anything seen?" */
    return obvious;
}
/* Array of raisable ego monster */
static mut raise_ego: [libc::c_int; 10] =
    [1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 4 as libc::c_int,
     4 as libc::c_int, 3 as libc::c_int];
/*
 * We are called from "project()" to "damage" objects
 *
 * We are called both for "beam" effects and "ball" effects.
 *
 * Perhaps we should only SOMETIMES damage things on the ground.
 *
 * The "r" parameter is the "distance from ground zero".
 *
 * Note that we determine if the player can "see" anything that happens
 * by taking into account: blindness, line-of-sight, and illumination.
 *
 * XXX XXX XXX We also "see" grids which are "memorized", probably a hack
 *
 * We return "TRUE" if the effect of the projection is "obvious".
 */
unsafe extern "C" fn project_o(mut who: libc::c_int, mut r: libc::c_int,
                               mut y: libc::c_int, mut x: libc::c_int,
                               mut dam: libc::c_int, mut typ: libc::c_int)
 -> bool_ {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    let mut this_o_idx: s16b = 0;
    let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
    let mut obvious: bool_ = 0 as libc::c_int as bool_;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut o_sval: libc::c_int = 0 as libc::c_int;
    let mut is_potion: bool_ = 0 as libc::c_int as bool_;
    /* XXX XXX XXX */
    who = if who != 0 { who } else { 0 as libc::c_int };
    /* Reduce damage by distance */
    dam = (dam + r) / (r + 1 as libc::c_int);
    /* Scan all objects in the grid */
    this_o_idx = (*c_ptr).o_idx;
    while this_o_idx != 0 {
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
        let mut is_art: bool_ = 0 as libc::c_int as bool_;
        let mut ignore: bool_ = 0 as libc::c_int as bool_;
        let mut plural: bool_ = 0 as libc::c_int as bool_;
        let mut do_kill: bool_ = 0 as libc::c_int as bool_;
        let mut note_kill: cptr = 0 as cptr;
        /* Acquire object */
        o_ptr = &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
        /* Acquire next object */
        next_o_idx = (*o_ptr).next_o_idx;
        /* Extract the flags */
        object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                     &mut esp);
        /* Get the "plural"-ness */
        if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
            plural = 1 as libc::c_int as bool_
        }
        /* Check for artifact */
        if (*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
               (if (*o_ptr).name1 as libc::c_int != 0 {
                    1 as libc::c_int
                } else { 0 as libc::c_int }) != 0 ||
               (if (*o_ptr).art_name as libc::c_int != 0 {
                    1 as libc::c_int
                } else { 0 as libc::c_int }) != 0 ||
               (if (*k_info.offset((*o_ptr).k_idx as isize)).flags3 as
                       libc::c_long & 0x8000 as libc::c_long != 0 {
                    1 as libc::c_int
                } else { 0 as libc::c_int }) != 0 ||
               (*o_ptr).art_name as libc::c_int != 0 {
            is_art = 1 as libc::c_int as bool_
        }
        /* Analyze the type */
        match typ {
            7 => {
                /* makes corpses explode */
                if (*o_ptr).tval as libc::c_int == 9 as libc::c_int {
                    let mut r_ptr: *mut monster_race =
                        &mut *r_info.offset((*o_ptr).pval2 as isize) as
                            *mut monster_race;
                    let mut dama: s32b = 0;
                    let mut radius: s32b = 7 as libc::c_int;
                    if (*r_ptr).flags1 & 0x200 as libc::c_int as libc::c_uint
                           != 0 {
                        dama =
                            maxroll((*r_ptr).hdice as s16b,
                                    (*r_ptr).hside as s16b)
                    } else {
                        dama =
                            damroll((*r_ptr).hdice as s16b,
                                    (*r_ptr).hside as s16b)
                    }
                    /* Adjust the damage */
                    dama = dama * dam / 100 as libc::c_int;
                    /* Adjust the radius */
                    radius = radius * dam / 100 as libc::c_int;
                    do_kill = 1 as libc::c_int as bool_;
                    note_kill =
                        if plural as libc::c_int != 0 {
                            b" explode!\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b" explodes!\x00" as *const u8 as
                                *const libc::c_char
                        };
                    project(who, radius, y, x, dama, 20 as libc::c_int,
                            0x8 as libc::c_int | 0x10 as libc::c_int |
                                0x20 as libc::c_int | 0x40 as libc::c_int);
                }
            }
            3 => {
                /* Acid -- Lots of things */
                if hates_acid(o_ptr) != 0 {
                    do_kill = 1 as libc::c_int as bool_;
                    note_kill =
                        if plural as libc::c_int != 0 {
                            b" melt!\x00" as *const u8 as *const libc::c_char
                        } else {
                            b" melts!\x00" as *const u8 as *const libc::c_char
                        };
                    if f3 as libc::c_long & 0x100000 as libc::c_long != 0 {
                        ignore = 1 as libc::c_int as bool_
                    }
                }
            }
            1 => {
                /* Elec -- Rings and Wands */
                if hates_elec(o_ptr) != 0 {
                    do_kill = 1 as libc::c_int as bool_;
                    note_kill =
                        if plural as libc::c_int != 0 {
                            b" are destroyed!\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b" is destroyed!\x00" as *const u8 as
                                *const libc::c_char
                        };
                    if f3 as libc::c_long & 0x200000 as libc::c_long != 0 {
                        ignore = 1 as libc::c_int as bool_
                    }
                }
            }
            5 => {
                /* Fire -- Flammable objects */
                if hates_fire(o_ptr) != 0 {
                    do_kill = 1 as libc::c_int as bool_;
                    note_kill =
                        if plural as libc::c_int != 0 {
                            b" burn up!\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b" burns up!\x00" as *const u8 as
                                *const libc::c_char
                        };
                    if f3 as libc::c_long & 0x400000 as libc::c_long != 0 {
                        ignore = 1 as libc::c_int as bool_
                    }
                }
            }
            4 => {
                /* Cold -- potions and flasks */
                if hates_cold(o_ptr) != 0 {
                    note_kill =
                        if plural as libc::c_int != 0 {
                            b" shatter!\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b" shatters!\x00" as *const u8 as
                                *const libc::c_char
                        };
                    do_kill = 1 as libc::c_int as bool_;
                    if f3 as libc::c_long & 0x800000 as libc::c_long != 0 {
                        ignore = 1 as libc::c_int as bool_
                    }
                }
            }
            12 => {
                /* Fire + Elec */
                if hates_fire(o_ptr) != 0 {
                    do_kill = 1 as libc::c_int as bool_;
                    note_kill =
                        if plural as libc::c_int != 0 {
                            b" burn up!\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b" burns up!\x00" as *const u8 as
                                *const libc::c_char
                        };
                    if f3 as libc::c_long & 0x400000 as libc::c_long != 0 {
                        ignore = 1 as libc::c_int as bool_
                    }
                }
                if hates_elec(o_ptr) != 0 {
                    ignore = 0 as libc::c_int as bool_;
                    do_kill = 1 as libc::c_int as bool_;
                    note_kill =
                        if plural as libc::c_int != 0 {
                            b" are destroyed!\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b" is destroyed!\x00" as *const u8 as
                                *const libc::c_char
                        };
                    if f3 as libc::c_long & 0x200000 as libc::c_long != 0 {
                        ignore = 1 as libc::c_int as bool_
                    }
                }
            }
            27 => {
                /* Fire + Cold */
                if hates_fire(o_ptr) != 0 {
                    do_kill = 1 as libc::c_int as bool_;
                    note_kill =
                        if plural as libc::c_int != 0 {
                            b" burn up!\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b" burns up!\x00" as *const u8 as
                                *const libc::c_char
                        };
                    if f3 as libc::c_long & 0x400000 as libc::c_long != 0 {
                        ignore = 1 as libc::c_int as bool_
                    }
                }
                if hates_cold(o_ptr) != 0 {
                    ignore = 0 as libc::c_int as bool_;
                    do_kill = 1 as libc::c_int as bool_;
                    note_kill =
                        if plural as libc::c_int != 0 {
                            b" shatter!\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b" shatters!\x00" as *const u8 as
                                *const libc::c_char
                        };
                    if f3 as libc::c_long & 0x800000 as libc::c_long != 0 {
                        ignore = 1 as libc::c_int as bool_
                    }
                }
            }
            28 | 20 | 23 | 21 => {
                /* Hack -- break potions and such */
                if hates_cold(o_ptr) != 0 {
                    note_kill =
                        if plural as libc::c_int != 0 {
                            b" shatter!\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b" shatters!\x00" as *const u8 as
                                *const libc::c_char
                        };
                    do_kill = 1 as libc::c_int as bool_
                }
            }
            26 => {
                /* Mana and Chaos -- destroy everything */
                do_kill = 1 as libc::c_int as bool_;
                note_kill =
                    if plural as libc::c_int != 0 {
                        b" are destroyed!\x00" as *const u8 as
                            *const libc::c_char
                    } else {
                        b" is destroyed!\x00" as *const u8 as
                            *const libc::c_char
                    }
            }
            81 => {
                do_kill = 1 as libc::c_int as bool_;
                note_kill =
                    if plural as libc::c_int != 0 {
                        b" evaporate!\x00" as *const u8 as *const libc::c_char
                    } else {
                        b" evaporates!\x00" as *const u8 as
                            *const libc::c_char
                    }
            }
            30 => {
                do_kill = 1 as libc::c_int as bool_;
                note_kill =
                    if plural as libc::c_int != 0 {
                        b" are destroyed!\x00" as *const u8 as
                            *const libc::c_char
                    } else {
                        b" is destroyed!\x00" as *const u8 as
                            *const libc::c_char
                    };
                if f2 as libc::c_long & 0x40000000 as libc::c_long != 0 {
                    ignore = 1 as libc::c_int as bool_
                }
            }
            79 | 80 => {
                /* Holy Fire and Hell Fire -- destroys cursed non-artifacts */
                if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
                    do_kill = 1 as libc::c_int as bool_;
                    note_kill =
                        if plural as libc::c_int != 0 {
                            b" are destroyed!\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b" is destroyed!\x00" as *const u8 as
                                *const libc::c_char
                        }
                }
            }
            42 | 41 => {
                /* Unlock chests */
                /* Chests are noticed only if trapped or locked */
                if (*o_ptr).tval as libc::c_int == 7 as libc::c_int {
                    /* Disarm/Unlock traps */
                    if (*o_ptr).pval > 0 as libc::c_int {
                        /* Disarm or Unlock */
                        (*o_ptr).pval = 0 as libc::c_int - (*o_ptr).pval;
                        /* Identify */
                        object_known(o_ptr);
                        /* Notice */
                        if (*o_ptr).marked != 0 {
                            msg_print(b"Click!\x00" as *const u8 as
                                          *const libc::c_char);
                            obvious = 1 as libc::c_int as bool_
                        }
                    }
                }
            }
            93 => {
                /* Identify it fully */
                object_aware(o_ptr);
                object_known(o_ptr);
                /* Mark the item as fully known */
                (*o_ptr).ident =
                    ((*o_ptr).ident as libc::c_int | 0x20 as libc::c_int) as
                        byte_hack;
                /* Process the appropriate hooks */
                process_hooks(16 as libc::c_int,
                              b"(d,s)\x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char,
                              0 as libc::c_int - this_o_idx as libc::c_int,
                              b"full\x00" as *const u8 as
                                  *const libc::c_char);
                /* Squelch ! */
                squeltch_grid();
            }
            91 => {
                object_aware(o_ptr);
                object_known(o_ptr);
                /* Process the appropriate hooks */
                process_hooks(16 as libc::c_int,
                              b"(d,s)\x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char,
                              0 as libc::c_int - this_o_idx as libc::c_int,
                              b"normal\x00" as *const u8 as
                                  *const libc::c_char);
                /* Squelch ! */
                squeltch_grid();
            }
            92 => {
                get_pos_player(7 as libc::c_int, &mut y, &mut x);
                /* Only corpses can be raised */
                if (*o_ptr).tval as libc::c_int == 9 as libc::c_int {
                    let mut ego: libc::c_int =
                        raise_ego[Rand_div(10 as libc::c_int) as usize];
                    if place_monster_one(y, x, (*o_ptr).pval2 as libc::c_int,
                                         ego, 0 as libc::c_int as bool_,
                                         if who == 0 {
                                             3 as libc::c_int
                                         } else { -(2 as libc::c_int) }) != 0
                       {
                        msg_print(b"A monster rises from the grave!\x00" as
                                      *const u8 as *const libc::c_char);
                    }
                    do_kill = 1 as libc::c_int as bool_
                }
            }
            107 => {
                let mut r_ptr_0: *mut monster_race =
                    &mut *r_info.offset((*o_ptr).pval2 as isize) as
                        *mut monster_race;
                let mut name: cptr = 0 as *const libc::c_char;
                if !((*o_ptr).tval as libc::c_int != 9 as libc::c_int) {
                    if Rand_div(100 as libc::c_int) + 1 as libc::c_int >
                           (*r_ptr_0).level as libc::c_int -
                               (*p_ptr).lev as libc::c_int {
                        if ((*r_ptr_0).level as libc::c_int) <
                               10 as libc::c_int {
                            name =
                                b"Manes\x00" as *const u8 as
                                    *const libc::c_char
                        } else if ((*r_ptr_0).level as libc::c_int) <
                                      18 as libc::c_int {
                            name =
                                b"Tengu\x00" as *const u8 as
                                    *const libc::c_char
                        } else if ((*r_ptr_0).level as libc::c_int) <
                                      26 as libc::c_int {
                            name =
                                b"Imp\x00" as *const u8 as *const libc::c_char
                        } else if ((*r_ptr_0).level as libc::c_int) <
                                      34 as libc::c_int {
                            name =
                                b"Arch-vile\x00" as *const u8 as
                                    *const libc::c_char
                        } else if ((*r_ptr_0).level as libc::c_int) <
                                      42 as libc::c_int {
                            name =
                                b"Bodak\x00" as *const u8 as
                                    *const libc::c_char
                        } else if ((*r_ptr_0).level as libc::c_int) <
                                      50 as libc::c_int {
                            name =
                                b"Erynies\x00" as *const u8 as
                                    *const libc::c_char
                        } else if ((*r_ptr_0).level as libc::c_int) <
                                      58 as libc::c_int {
                            name =
                                b"Vrock\x00" as *const u8 as
                                    *const libc::c_char
                        } else if ((*r_ptr_0).level as libc::c_int) <
                                      66 as libc::c_int {
                            name =
                                b"Hezrou\x00" as *const u8 as
                                    *const libc::c_char
                        } else if ((*r_ptr_0).level as libc::c_int) <
                                      74 as libc::c_int {
                            name =
                                b"Glabrezu\x00" as *const u8 as
                                    *const libc::c_char
                        } else if ((*r_ptr_0).level as libc::c_int) <
                                      82 as libc::c_int {
                            name =
                                b"Nalfeshnee\x00" as *const u8 as
                                    *const libc::c_char
                        } else if ((*r_ptr_0).level as libc::c_int) <
                                      90 as libc::c_int {
                            name =
                                b"Marilith\x00" as *const u8 as
                                    *const libc::c_char
                        } else {
                            name =
                                b"Nycadaemon\x00" as *const u8 as
                                    *const libc::c_char
                        }
                        if place_monster_one(y, x, test_monster_name(name),
                                             0 as libc::c_int,
                                             0 as libc::c_int as bool_,
                                             if who == 0 {
                                                 3 as libc::c_int
                                             } else { -(2 as libc::c_int) })
                               != 0 {
                            msg_print(b"A demon emerges from Hell!\x00" as
                                          *const u8 as *const libc::c_char);
                        }
                    }
                    do_kill = 1 as libc::c_int as bool_
                }
            }
            _ => {
                /* Hooks! */
                if process_hooks_ret(59 as libc::c_int,
                                     b"dd\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char,
                                     b"(s,d,d,d,d,d,d,O)\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char,
                                     b"object\x00" as *const u8 as
                                         *const libc::c_char, who, typ, dam,
                                     r, y, x, o_ptr) != 0 {
                    obvious =
                        process_hooks_return[0 as libc::c_int as usize].num as
                            bool_;
                    do_kill =
                        process_hooks_return[1 as libc::c_int as usize].num as
                            bool_
                }
            }
        }
        /* Attempt to destroy the object */
        if do_kill != 0 {
            /* Effect "observed" */
            if (*o_ptr).marked != 0 {
                obvious = 1 as libc::c_int as bool_;
                object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                            0 as libc::c_int);
            }
            /* Artifacts, and other objects, get to resist */
            if is_art as libc::c_int != 0 || ignore as libc::c_int != 0 {
                /* Observe the resist */
                if (*o_ptr).marked != 0 {
                    msg_format(b"The %s %s unaffected!\x00" as *const u8 as
                                   *const libc::c_char, o_name.as_mut_ptr(),
                               if plural as libc::c_int != 0 {
                                   b"are\x00" as *const u8 as
                                       *const libc::c_char
                               } else {
                                   b"is\x00" as *const u8 as
                                       *const libc::c_char
                               });
                }
            } else {
                /* Kill it */
                /* Describe if needed */
                if (*o_ptr).marked as libc::c_int != 0 && !note_kill.is_null()
                   {
                    msg_format(b"The %s%s\x00" as *const u8 as
                                   *const libc::c_char, o_name.as_mut_ptr(),
                               note_kill);
                }
                o_sval = (*o_ptr).sval as libc::c_int;
                is_potion =
                    ((*k_info.offset((*o_ptr).k_idx as isize)).tval as
                         libc::c_int == 71 as libc::c_int ||
                         (*k_info.offset((*o_ptr).k_idx as isize)).tval as
                             libc::c_int == 72 as libc::c_int) as libc::c_int
                        as bool_;
                delete_object_idx(this_o_idx as libc::c_int);
                if is_potion != 0 { potion_smash_effect(who, y, x, o_sval); }
                lite_spot(y, x);
            }
        }
        this_o_idx = next_o_idx
    }
    /* Delete the object */
    /* Potions produce effects when 'shattered' */
    /* Redraw */
    /* Return "Anything seen?" */
    return obvious;
}
/* Can the monster be hurt ? */
#[no_mangle]
pub unsafe extern "C" fn hurt_monster(mut m_ptr: *mut monster_type) -> bool_ {
    if (*m_ptr).status as libc::c_int == 4 as libc::c_int {
        return 0 as libc::c_int as bool_
    } else { return 1 as libc::c_int as bool_ };
}
/*
 * Helper function for "project()" below.
 *
 * Handle a beam/bolt/ball causing damage to a monster.
 *
 * This routine takes a "source monster" (by index) which is mostly used to
 * determine if the player is causing the damage, and a "radius" (see below),
 * which is used to decrease the power of explosions with distance, and a
 * location, via integers which are modified by certain types of attacks
 * (polymorph and teleport being the obvious ones), a default damage, which
 * is modified as needed based on various properties, and finally a "damage
 * type" (see below).
 *
 * Note that this routine can handle "no damage" attacks (like teleport) by
 * taking a "zero" damage, and can even take "parameters" to attacks (like
 * confuse) by accepting a "damage", using it to calculate the effect, and
 * then setting the damage to zero.  Note that the "damage" parameter is
 * divided by the radius, so monsters not at the "epicenter" will not take
 * as much damage (or whatever)...
 *
 * Note that "polymorph" is dangerous, since a failure in "place_monster()"'
 * may result in a dereference of an invalid pointer.  XXX XXX XXX
 *
 * Various messages are produced, and damage is applied.
 *
 * Just "casting" a substance (i.e. plasma) does not make you immune, you must
 * actually be "made" of that substance, or "breathe" big balls of it.
 *
 * We assume that "Plasma" monsters, and "Plasma" breathers, are immune
 * to plasma.
 *
 * We assume "Nether" is an evil, necromantic force, so it doesn't hurt undead,
 * and hurts evil less.  If can breath nether, then it resists it as well.
 *
 * Damage reductions use the following formulas:
 *   Note that "dam = dam * 6 / (randint(6) + 6);"
 *     gives avg damage of .655, ranging from .858 to .500
 *   Note that "dam = dam * 5 / (randint(6) + 6);"
 *     gives avg damage of .544, ranging from .714 to .417
 *   Note that "dam = dam * 4 / (randint(6) + 6);"
 *     gives avg damage of .444, ranging from .556 to .333
 *   Note that "dam = dam * 3 / (randint(6) + 6);"
 *     gives avg damage of .327, ranging from .427 to .250
 *   Note that "dam = dam * 2 / (randint(6) + 6);"
 *     gives something simple.
 *
 * In this function, "result" messages are postponed until the end, where
 * the "note" string is appended to the monster name, if not NULL.  So,
 * to make a spell have "no effect" just set "note" to NULL.  You should
 * also set "notice" to FALSE, or the player will learn what the spell does.
 *
 * We attempt to return "TRUE" if the player saw anything "useful" happen.
 */
#[no_mangle]
pub unsafe extern "C" fn project_m(mut who: libc::c_int, mut r: libc::c_int,
                                   mut y: libc::c_int, mut x: libc::c_int,
                                   mut dam: libc::c_int, mut typ: libc::c_int)
 -> bool_ {
    let mut tmp: libc::c_int = 0;
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset((*c_ptr).m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    let mut killer: [libc::c_char; 80] = [0; 80];
    let mut name: cptr = r_name.offset((*r_ptr).name as isize) as cptr;
    /* Is the monster "seen"? */
    let mut seen: bool_ = 0;
    /* Were the effects "obvious" (if seen)? */
    let mut obvious: bool_ = 0 as libc::c_int as bool_;
    /* Were the effects "irrelevant"? */
    let mut skipped: bool_ = 0 as libc::c_int as bool_;
    /* Move setting */
    let mut x1: libc::c_int = 0 as libc::c_int;
    let mut y1: libc::c_int = 0 as libc::c_int;
    let mut a: libc::c_int = 0 as libc::c_int;
    let mut b: libc::c_int = 0 as libc::c_int;
    let mut do_move: libc::c_int = 0 as libc::c_int;
    /* Polymorph setting (true or false) */
    let mut do_poly: libc::c_int = 0 as libc::c_int;
    /* Teleport setting (max distance) */
    let mut do_dist: libc::c_int = 0 as libc::c_int;
    /* Confusion setting (amount to confuse) */
    let mut do_conf: libc::c_int = 0 as libc::c_int;
    /* Stunning setting (amount to stun) */
    let mut do_stun: libc::c_int = 0 as libc::c_int;
    /* Bleeding amount */
    let mut do_cut: libc::c_int = 0 as libc::c_int;
    /* Poison amount */
    let mut do_pois: libc::c_int = 0 as libc::c_int;
    /* Sleep amount (amount to sleep) */
    let mut do_sleep: libc::c_int = 0 as libc::c_int;
    /* Fear amount (amount to fear) */
    let mut do_fear: libc::c_int = 0 as libc::c_int;
    /* Hold the monster name */
    let mut m_name: [libc::c_char; 80] = [0; 80];
    /* Assume no note */
    let mut note: cptr = 0 as cptr;
    /* Assume a default death */
    let mut note_dies: cptr =
        b" dies.\x00" as *const u8 as *const libc::c_char;
    /* Nobody here */
    if (*c_ptr).m_idx == 0 { return 0 as libc::c_int as bool_ }
    /* Never affect projector */
    if who != 0 && (*c_ptr).m_idx as libc::c_int == who {
        return 0 as libc::c_int as bool_
    }
    /*
	 * Don't affect already dead monsters
	 * Prevents problems with chain reactions of exploding monsters
	 */
    if (*m_ptr).hp < 0 as libc::c_int { return 0 as libc::c_int as bool_ }
    /* Remember if the monster is within player's line of sight */
    seen =
        if (*m_ptr).ml as libc::c_int != 0 &&
               (who != -(101 as libc::c_int) && who != -(100 as libc::c_int))
           {
            1 as libc::c_int
        } else { 0 as libc::c_int } as bool_;
    /* Reduce damage by distance */
    dam = (dam + r) / (r + 1 as libc::c_int);
    /* Get the monster name (BEFORE polymorphing) */
    monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
    /* Mega Gachk */
    if (*r_ptr).flags2 & 0x100 as libc::c_int as libc::c_uint != 0 {
        msg_format(b"%^s is immune to magic.\x00" as *const u8 as
                       *const libc::c_char, m_name.as_mut_ptr());
        return seen
    }
    /* Some monsters get "destroyed" */
    if (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0 ||
           (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0 ||
           (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint != 0 ||
           (*r_ptr).flags3 & 0x800 as libc::c_int as libc::c_uint != 0 ||
           !strchr(b"Evg\x00" as *const u8 as *const libc::c_char,
                   (*r_ptr).d_char as libc::c_int).is_null() {
        /* Special note at death */
        note_dies = b" is destroyed.\x00" as *const u8 as *const libc::c_char
    }
    if who == 0 && is_friend(m_ptr) >= 0 as libc::c_int {
        let mut get_angry: bool_ = 0 as libc::c_int as bool_;
        /* Grrr? */
        match typ {
            61 | 62 | 63 | 82 | 110 | 99 | 83 | 84 | 106 | 53 | 54 | 18 | 88 |
            92 | 107 | 91 => {
            }
            108 => {
                if (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0
                   {
                    get_angry = 1 as libc::c_int as bool_
                }
            }
            40 => {
                if (*r_ptr).flags3 & 0x2000 as libc::c_int as libc::c_uint !=
                       0 {
                    get_angry = 1 as libc::c_int as bool_
                }
            }
            79 => {
                if (*r_ptr).flags3 & 0x200 as libc::c_int as libc::c_uint == 0
                   {
                    get_angry = 1 as libc::c_int as bool_
                }
            }
            64 | 67 => {
                if (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0
                   {
                    get_angry = 1 as libc::c_int as bool_
                }
            }
            65 | 68 => {
                if (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint != 0
                   {
                    get_angry = 1 as libc::c_int as bool_
                }
            }
            90 => {
                if (*r_ptr).flags3 & 0x200 as libc::c_int as libc::c_uint != 0
                   {
                    get_angry = 1 as libc::c_int as bool_
                }
            }
            70 => {
                if (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0
                   {
                    get_angry = 1 as libc::c_int as bool_
                }
            }
            71 | 6 => {
                if (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint == 0
                       &&
                       (*r_ptr).flags3 & 0x800 as libc::c_int as libc::c_uint
                           == 0 {
                    get_angry = 1 as libc::c_int as bool_
                }
            }
            85 | 86 => {
                if (*r_ptr).flags2 & 0x40 as libc::c_int as libc::c_uint == 0
                   {
                    get_angry = 1 as libc::c_int as bool_
                }
            }
            89 => {
                if (*r_ptr).flags3 & 0x40000000 as libc::c_int as libc::c_uint
                       == 0 {
                    get_angry = 1 as libc::c_int as bool_
                }
            }
            52 | 51 => {
                if Rand_div(8 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                    get_angry = 1 as libc::c_int as bool_
                }
            }
            15 | 17 => {
                if (*r_ptr).flags3 & 0x1000 as libc::c_int as libc::c_uint !=
                       0 {
                    get_angry = 1 as libc::c_int as bool_
                }
            }
            _ => {
                /* Hooks! */
                if process_hooks_ret(59 as libc::c_int,
                                     b"d\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char,
                                     b"(s,d,d,d,d,d,d,M)\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char,
                                     b"angry\x00" as *const u8 as
                                         *const libc::c_char, who, typ, dam,
                                     r, y, x, m_ptr) != 0 {
                    get_angry =
                        process_hooks_return[0 as libc::c_int as usize].num as
                            bool_
                } else { get_angry = 1 as libc::c_int as bool_ }
            }
        }
        /* Now anger it if appropriate */
        if get_angry as libc::c_int == 1 as libc::c_int && who == 0 {
            match is_friend(m_ptr) {
                1 => {
                    if change_side(m_ptr) != 0 {
                        msg_format(b"%^s gets angry!\x00" as *const u8 as
                                       *const libc::c_char,
                                   m_name.as_mut_ptr());
                    }
                }
                0 => {
                    msg_format(b"%^s gets angry!\x00" as *const u8 as
                                   *const libc::c_char, m_name.as_mut_ptr());
                    (*m_ptr).status = -(1 as libc::c_int) as s16b
                }
                _ => { }
            }
        }
    }
    /* Analyze the damage type */
    match typ {
        109 => {
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            py_attack(y, x, dam);
            skipped = 1 as libc::c_int as bool_;
            dam = 0 as libc::c_int
        }
        91 => {
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Probe */
            do_probe((*c_ptr).m_idx as libc::c_int);
            dam = 0 as libc::c_int
        }
        105 => {
            /* Death -- instant death  */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).r_flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                dam = 0 as libc::c_int
            } else {
                /* It KILLS */
                dam = 32535 as libc::c_int
            }
        }
        10 => {
            /* Magic Missile -- pure damage */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
        }
        3 => {
            /* Acid */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).flags9 & 0x40 as libc::c_int as libc::c_uint != 0 {
                note =
                    b" is hit hard.\x00" as *const u8 as *const libc::c_char;
                dam *= 3 as libc::c_int;
                if seen != 0 {
                    (*r_ptr).r_flags9 |= 0x40 as libc::c_int as libc::c_uint
                }
            }
            if (*r_ptr).flags3 & 0x10000 as libc::c_int as libc::c_uint != 0 {
                note =
                    b" resists a lot.\x00" as *const u8 as
                        *const libc::c_char;
                dam /= 9 as libc::c_int;
                if seen != 0 {
                    (*r_ptr).r_flags3 |=
                        0x10000 as libc::c_int as libc::c_uint
                }
            }
        }
        1 => {
            /* Electricity */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).flags9 & 0x80 as libc::c_int as libc::c_uint != 0 {
                note =
                    b" is hit hard.\x00" as *const u8 as *const libc::c_char;
                dam *= 3 as libc::c_int;
                if seen != 0 {
                    (*r_ptr).r_flags9 |= 0x80 as libc::c_int as libc::c_uint
                }
            }
            if (*r_ptr).flags3 & 0x20000 as libc::c_int as libc::c_uint != 0 {
                note =
                    b" resists a lot.\x00" as *const u8 as
                        *const libc::c_char;
                dam /= 9 as libc::c_int;
                if seen != 0 {
                    (*r_ptr).r_flags3 |=
                        0x20000 as libc::c_int as libc::c_uint
                }
            }
        }
        5 => {
            /* Fire damage */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).flags3 & 0x4000 as libc::c_int as libc::c_uint != 0 {
                note =
                    b" is hit hard.\x00" as *const u8 as *const libc::c_char;
                dam *= 3 as libc::c_int;
                if seen != 0 {
                    (*r_ptr).r_flags3 |= 0x4000 as libc::c_int as libc::c_uint
                }
            }
            if (*r_ptr).flags3 & 0x40000 as libc::c_int as libc::c_uint != 0 {
                note =
                    b" resists a lot.\x00" as *const u8 as
                        *const libc::c_char;
                dam /= 9 as libc::c_int;
                if seen != 0 {
                    (*r_ptr).r_flags3 |=
                        0x40000 as libc::c_int as libc::c_uint
                }
            }
        }
        4 => {
            /* Cold */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).flags3 & 0x8000 as libc::c_int as libc::c_uint != 0 {
                note =
                    b" is hit hard.\x00" as *const u8 as *const libc::c_char;
                dam *= 3 as libc::c_int;
                if seen != 0 {
                    (*r_ptr).r_flags3 |= 0x8000 as libc::c_int as libc::c_uint
                }
            }
            if (*r_ptr).flags3 & 0x80000 as libc::c_int as libc::c_uint != 0 {
                note =
                    b" resists a lot.\x00" as *const u8 as
                        *const libc::c_char;
                dam /= 9 as libc::c_int;
                if seen != 0 {
                    (*r_ptr).r_flags3 |=
                        0x80000 as libc::c_int as libc::c_uint
                }
            }
        }
        2 => {
            /* Poison */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if Rand_div(100 as libc::c_int) < 25 as libc::c_int {
                do_pois =
                    (10 as libc::c_int +
                         (Rand_div(11 as libc::c_int) + 1 as libc::c_int) + r)
                        / (r + 1 as libc::c_int)
            }
            if (*r_ptr).flags9 & 0x100 as libc::c_int as libc::c_uint != 0 {
                note =
                    b" is hit hard.\x00" as *const u8 as *const libc::c_char;
                dam *= 3 as libc::c_int;
                do_pois *= 2 as libc::c_int;
                if seen != 0 {
                    (*r_ptr).r_flags9 |= 0x100 as libc::c_int as libc::c_uint
                }
            }
            if (*r_ptr).flags3 & 0x100000 as libc::c_int as libc::c_uint != 0
               {
                note =
                    b" resists a lot.\x00" as *const u8 as
                        *const libc::c_char;
                dam /= 9 as libc::c_int;
                do_pois = 0 as libc::c_int;
                if seen != 0 {
                    (*r_ptr).r_flags3 |=
                        0x100000 as libc::c_int as libc::c_uint
                }
            }
        }
        6 => {
            /* Thick Poison */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if Rand_div(100 as libc::c_int) < 15 as libc::c_int {
                do_pois =
                    (10 as libc::c_int +
                         (Rand_div(11 as libc::c_int) + 1 as libc::c_int) + r)
                        / (r + 1 as libc::c_int)
            }
            if (*r_ptr).flags3 & 0x800 as libc::c_int as libc::c_uint != 0 ||
                   (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0
               {
                note = b" is immune.\x00" as *const u8 as *const libc::c_char;
                dam = 0 as libc::c_int;
                do_pois = 0 as libc::c_int
            }
        }
        73 => {
            /* Nuclear waste */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).flags3 & 0x100000 as libc::c_int as libc::c_uint != 0
               {
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                dam *= 3 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int;
                if seen != 0 {
                    (*r_ptr).r_flags3 |=
                        0x100000 as libc::c_int as libc::c_uint
                }
            } else if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                          1 as libc::c_int {
                do_poly = 1 as libc::c_int
            }
        }
        80 => {
            /* Holy Orb -- hurts Evil (replaced with Hellfire) */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint != 0 {
                dam *= 2 as libc::c_int;
                note =
                    b" is hit hard.\x00" as *const u8 as *const libc::c_char;
                if seen != 0 {
                    (*r_ptr).r_flags3 |= 0x40 as libc::c_int as libc::c_uint
                }
            }
        }
        79 => {
            /* Holy Fire -- hurts Evil, Good are immune, others _resist_ */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).flags3 & 0x200 as libc::c_int as libc::c_uint != 0 {
                dam = 0 as libc::c_int;
                note = b" is immune.\x00" as *const u8 as *const libc::c_char;
                if seen != 0 {
                    (*r_ptr).r_flags3 |= 0x200 as libc::c_int as libc::c_uint
                }
            } else if (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint !=
                          0 {
                dam *= 2 as libc::c_int;
                note =
                    b" is hit hard.\x00" as *const u8 as *const libc::c_char;
                if seen != 0 {
                    (*r_ptr).r_flags3 |= 0x40 as libc::c_int as libc::c_uint
                }
            } else {
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                dam *= 3 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int
            }
        }
        11 => {
            /* Arrow -- XXX no defense */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
        }
        12 => {
            /* Plasma -- XXX perhaps check ELEC or FIRE */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).flags3 & 0x1000000 as libc::c_int as libc::c_uint != 0
               {
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                dam *= 3 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int;
                if seen != 0 {
                    (*r_ptr).r_flags3 |=
                        0x1000000 as libc::c_int as libc::c_uint
                }
            }
        }
        31 => {
            /* Nether -- see above */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0 {
                note = b" is immune.\x00" as *const u8 as *const libc::c_char;
                dam = 0 as libc::c_int;
                if seen != 0 {
                    (*r_ptr).r_flags3 |= 0x20 as libc::c_int as libc::c_uint
                }
            } else if (*r_ptr).flags3 &
                          0x400000 as libc::c_int as libc::c_uint != 0 {
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                dam *= 3 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int;
                if seen != 0 {
                    (*r_ptr).r_flags3 |=
                        0x400000 as libc::c_int as libc::c_uint
                }
            } else if (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint !=
                          0 {
                dam /= 2 as libc::c_int;
                note =
                    b" resists somewhat.\x00" as *const u8 as
                        *const libc::c_char;
                if seen != 0 {
                    (*r_ptr).r_flags3 |= 0x40 as libc::c_int as libc::c_uint
                }
            }
        }
        14 => {
            /* Water (acid) damage -- Water spirits/elementals are immune */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).d_char as libc::c_int == 'E' as i32 &&
                   (prefix(name, b"W\x00" as *const u8 as *const libc::c_char)
                        as libc::c_int != 0 ||
                        !strstr(r_name.offset((*r_ptr).name as isize),
                                b"Unmaker\x00" as *const u8 as
                                    *const libc::c_char).is_null()) {
                note = b" is immune.\x00" as *const u8 as *const libc::c_char;
                dam = 0 as libc::c_int
            } else if (*r_ptr).flags3 &
                          0x800000 as libc::c_int as libc::c_uint != 0 {
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                dam *= 3 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int;
                if seen != 0 {
                    (*r_ptr).r_flags3 |=
                        0x800000 as libc::c_int as libc::c_uint
                }
            }
        }
        13 => {
            /* Wave = Water + Force */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).d_char as libc::c_int == 'E' as i32 &&
                   (prefix(name, b"W\x00" as *const u8 as *const libc::c_char)
                        as libc::c_int != 0 ||
                        !strstr(r_name.offset((*r_ptr).name as isize),
                                b"Unmaker\x00" as *const u8 as
                                    *const libc::c_char).is_null()) {
                note = b" is immune.\x00" as *const u8 as *const libc::c_char;
                dam = 0 as libc::c_int
            } else if (*r_ptr).flags3 &
                          0x800000 as libc::c_int as libc::c_uint != 0 {
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                dam *= 3 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int;
                if seen != 0 {
                    (*r_ptr).r_flags3 |=
                        0x800000 as libc::c_int as libc::c_uint
                }
            }
            if who == 0 as libc::c_int {
                a = 0 as libc::c_int;
                b = 0 as libc::c_int;
                /* Get vector from firer to target */
                x1 =
                    ((*m_ptr).fx as libc::c_int - (*p_ptr).px as libc::c_int)
                        * 10 as libc::c_int;
                y1 =
                    ((*m_ptr).fy as libc::c_int - (*p_ptr).py as libc::c_int)
                        * 10 as libc::c_int;
                /* Make sure no zero divides */
                if x1 == 0 as libc::c_int { x1 = 1 as libc::c_int }
                if y1 == 0 as libc::c_int { y1 = 1 as libc::c_int }
                /* Select direction monster is being pushed */
                /* Roughly horizontally */
                if 2 as libc::c_int * y1 / x1 == 0 as libc::c_int {
                    if x1 > 0 as libc::c_int {
                        a = 1 as libc::c_int;
                        b = 0 as libc::c_int
                    } else { a = -(1 as libc::c_int); b = 0 as libc::c_int }
                } else if 2 as libc::c_int * x1 / y1 == 0 as libc::c_int {
                    if y1 > 0 as libc::c_int {
                        a = 0 as libc::c_int;
                        b = 1 as libc::c_int
                    } else { a = 0 as libc::c_int; b = -(1 as libc::c_int) }
                } else {
                    /* Roughly vertically */
                    /* Take diagonals */
                    if y1 > 0 as libc::c_int {
                        b = 1 as libc::c_int
                    } else { b = -(1 as libc::c_int) }
                    if x1 > 0 as libc::c_int {
                        a = 1 as libc::c_int
                    } else { a = -(1 as libc::c_int) }
                }
                /* Move monster 2 offsets back */
                do_move = 2 as libc::c_int;
                /* Old monster coords in x,y */
                y1 = (*m_ptr).fy as libc::c_int;
                x1 = (*m_ptr).fx as libc::c_int;
                /* Monster move offsets in a,b */
                note =
                    b" is thrown away.\x00" as *const u8 as
                        *const libc::c_char
            }
        }
        30 => {
            /* Chaos -- Chaos breathers resist */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            do_poly = 1 as libc::c_int;
            do_conf =
                (5 as libc::c_int +
                     (Rand_div(11 as libc::c_int) + 1 as libc::c_int) + r) /
                    (r + 1 as libc::c_int);
            if (*r_ptr).flags4 & 0x40000 as libc::c_int as libc::c_uint != 0
                   ||
                   (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0
                       &&
                       Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                dam *= 3 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int;
                do_poly = 0 as libc::c_int
            }
        }
        20 => {
            /* Shards -- Shard breathers resist */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if Rand_div(100 as libc::c_int) < 33 as libc::c_int {
                do_cut =
                    (10 as libc::c_int +
                         (Rand_div(15 as libc::c_int) + 1 as libc::c_int) + r)
                        / (r + 1 as libc::c_int)
            }
            if (*r_ptr).flags4 & 0x1000000 as libc::c_int as libc::c_uint != 0
               {
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                dam *= 3 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int;
                do_cut = 0 as libc::c_int
            }
        }
        72 => {
            /* Rocket: Shard resistance helps */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if Rand_div(100 as libc::c_int) < 12 as libc::c_int {
                do_cut =
                    (10 as libc::c_int +
                         (Rand_div(15 as libc::c_int) + 1 as libc::c_int) + r)
                        / (r + 1 as libc::c_int)
            }
            if (*r_ptr).flags4 & 0x1000000 as libc::c_int as libc::c_uint != 0
               {
                note =
                    b" resists somewhat.\x00" as *const u8 as
                        *const libc::c_char;
                dam /= 2 as libc::c_int;
                do_cut = 0 as libc::c_int
            }
        }
        21 => {
            /* Sound -- Sound breathers resist */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if who <= 0 as libc::c_int {
                if Rand_div(100 as libc::c_int - (*p_ptr).lev as libc::c_int)
                       < 50 as libc::c_int {
                    do_stun =
                        (10 as libc::c_int +
                             (Rand_div(15 as libc::c_int) + 1 as libc::c_int)
                             + r) / (r + 1 as libc::c_int)
                }
            } else {
                do_stun =
                    (10 as libc::c_int +
                         (Rand_div(15 as libc::c_int) + 1 as libc::c_int) + r)
                        / (r + 1 as libc::c_int)
            }
            if (*r_ptr).flags4 & 0x20000 as libc::c_int as libc::c_uint != 0 {
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                dam *= 2 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int
            }
        }
        22 => {
            /* Confusion */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            do_conf =
                (10 as libc::c_int +
                     (Rand_div(15 as libc::c_int) + 1 as libc::c_int) + r) /
                    (r + 1 as libc::c_int);
            if (*r_ptr).flags4 & 0x10000 as libc::c_int as libc::c_uint != 0 {
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                dam *= 2 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int
            } else if (*r_ptr).flags3 &
                          0x40000000 as libc::c_int as libc::c_uint != 0 {
                note =
                    b" resists somewhat.\x00" as *const u8 as
                        *const libc::c_char;
                dam /= 2 as libc::c_int
            }
        }
        32 => {
            /* Disenchantment -- Breathers and Disenchanters resist */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).flags3 & 0x4000000 as libc::c_int as libc::c_uint != 0
               {
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                dam *= 3 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int;
                if seen != 0 {
                    (*r_ptr).r_flags3 |=
                        0x4000000 as libc::c_int as libc::c_uint
                }
            }
        }
        33 => {
            /* Nexus -- Breathers and Existers resist */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).flags3 & 0x2000000 as libc::c_int as libc::c_uint != 0
               {
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                dam *= 3 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int;
                if seen != 0 {
                    (*r_ptr).r_flags3 |=
                        0x2000000 as libc::c_int as libc::c_uint
                }
            }
        }
        23 => {
            /* Force */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /*
			 * If fired by player, try pushing monster.
			 * First get vector from player to monster.
			 * x10 so we can use pseudo-fixed point maths.
			 *
			 * Really should use get_angle_to_grid (util.c)
			 */
            if who == 0 as libc::c_int {
                a = 0 as libc::c_int;
                b = 0 as libc::c_int;
                /* Get vector from firer to target */
                x1 =
                    ((*m_ptr).fx as libc::c_int - (*p_ptr).px as libc::c_int)
                        * 10 as libc::c_int;
                y1 =
                    ((*m_ptr).fy as libc::c_int - (*p_ptr).py as libc::c_int)
                        * 10 as libc::c_int;
                /* Make sure no zero divides */
                if x1 == 0 as libc::c_int { x1 = 1 as libc::c_int }
                if y1 == 0 as libc::c_int { y1 = 1 as libc::c_int }
                /* Select direction monster is being pushed */
                /* Roughly horizontally */
                if 2 as libc::c_int * y1 / x1 == 0 as libc::c_int {
                    if x1 > 0 as libc::c_int {
                        a = 1 as libc::c_int;
                        b = 0 as libc::c_int
                    } else { a = -(1 as libc::c_int); b = 0 as libc::c_int }
                } else if 2 as libc::c_int * x1 / y1 == 0 as libc::c_int {
                    if y1 > 0 as libc::c_int {
                        a = 0 as libc::c_int;
                        b = 1 as libc::c_int
                    } else { a = 0 as libc::c_int; b = -(1 as libc::c_int) }
                } else {
                    /* Roughly vertically */
                    /* Take diagonals */
                    if y1 > 0 as libc::c_int {
                        b = 1 as libc::c_int
                    } else { b = -(1 as libc::c_int) }
                    if x1 > 0 as libc::c_int {
                        a = 1 as libc::c_int
                    } else { a = -(1 as libc::c_int) }
                }
                /* Move monster 2 offsets back */
                do_move = 2 as libc::c_int;
                /* Old monster coords in x,y */
                y1 = (*m_ptr).fy as libc::c_int;
                x1 = (*m_ptr).fx as libc::c_int;
                /* Monster move offsets in a,b */
                note =
                    b" is thrown away.\x00" as *const u8 as
                        *const libc::c_char
            } else {
                /* --hack-- Only stun if a monster fired it */
                do_stun =
                    (Rand_div(15 as libc::c_int) + 1 as libc::c_int + r) /
                        (r + 1 as libc::c_int)
            }
            if (*r_ptr).flags4 & 0x4000000 as libc::c_int as libc::c_uint != 0
               {
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                dam *= 3 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int
            }
        }
        24 => {
            /* Inertia -- breathers resist */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).flags4 & 0x400000 as libc::c_int as libc::c_uint != 0
               {
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                dam *= 3 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int
            } else if (*m_ptr).level as libc::c_int >
                          Rand_div((if (dam - 10 as libc::c_int) <
                                           1 as libc::c_int {
                                        1 as libc::c_int
                                    } else { (dam) - 10 as libc::c_int })) +
                              1 as libc::c_int + 10 as libc::c_int {
                obvious = 0 as libc::c_int as bool_
            } else {
                /* Powerful monsters can resist */
                /* Normal monsters slow down */
                if (*m_ptr).mspeed as libc::c_int > 60 as libc::c_int {
                    (*m_ptr).mspeed =
                        ((*m_ptr).mspeed as libc::c_int - 10 as libc::c_int)
                            as byte_hack
                }
                note =
                    b" starts moving slower.\x00" as *const u8 as
                        *const libc::c_char
            }
        }
        34 => {
            /* Time -- breathers resist */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).flags4 & 0x200000 as libc::c_int as libc::c_uint != 0
               {
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                dam *= 3 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int
            }
        }
        35 => {
            /* Gravity -- breathers resist */
            let mut resist_tele: bool_ = 0 as libc::c_int as bool_;
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).flags3 & 0x200000 as libc::c_int as libc::c_uint != 0
               {
                if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
                    if seen != 0 {
                        (*r_ptr).r_flags3 |=
                            0x200000 as libc::c_int as libc::c_uint
                    }
                    note =
                        b" is unaffected!\x00" as *const u8 as
                            *const libc::c_char;
                    resist_tele = 1 as libc::c_int as bool_
                } else if (*m_ptr).level as libc::c_int >
                              Rand_div(100 as libc::c_int) + 1 as libc::c_int
                 {
                    if seen != 0 {
                        (*r_ptr).r_flags3 |=
                            0x200000 as libc::c_int as libc::c_uint
                    }
                    note =
                        b" resists!\x00" as *const u8 as *const libc::c_char;
                    resist_tele = 1 as libc::c_int as bool_
                }
            }
            if resist_tele == 0 {
                do_dist = 10 as libc::c_int
            } else { do_dist = 0 as libc::c_int }
            if (*r_ptr).flags4 & 0x800000 as libc::c_int as libc::c_uint != 0
               {
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                dam *= 3 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int;
                do_dist = 0 as libc::c_int
            } else {
                /* 1. slowness */
				/* Powerful monsters can resist */
                if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0
                       ||
                       (*m_ptr).level as libc::c_int >
                           Rand_div((if (dam - 10 as libc::c_int) <
                                            1 as libc::c_int {
                                         1 as libc::c_int
                                     } else { (dam) - 10 as libc::c_int })) +
                               1 as libc::c_int + 10 as libc::c_int {
                    obvious = 0 as libc::c_int as bool_
                } else {
                    /* Normal monsters slow down */
                    if (*m_ptr).mspeed as libc::c_int > 60 as libc::c_int {
                        (*m_ptr).mspeed =
                            ((*m_ptr).mspeed as libc::c_int -
                                 10 as libc::c_int) as byte_hack
                    }
                    note =
                        b" starts moving slower.\x00" as *const u8 as
                            *const libc::c_char
                }
                /* 2. stun */
                do_stun =
                    damroll(((*p_ptr).lev as libc::c_int / 10 as libc::c_int +
                                 3 as libc::c_int) as s16b, dam as s16b) +
                        1 as libc::c_int;
                /* Attempt a saving throw */
                if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0
                       ||
                       (*m_ptr).level as libc::c_int >
                           Rand_div((if (dam - 10 as libc::c_int) <
                                            1 as libc::c_int {
                                         1 as libc::c_int
                                     } else { (dam) - 10 as libc::c_int })) +
                               1 as libc::c_int + 10 as libc::c_int {
                    /* Resist */
                    do_stun = 0 as libc::c_int;
                    /* No obvious effect */
                    note =
                        b" is unaffected!\x00" as *const u8 as
                            *const libc::c_char;
                    obvious = 0 as libc::c_int as bool_
                }
            }
        }
        26 => {
            /* Pure damage */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
        }
        81 => {
            /* Pure damage */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).flags3 & 0x2000 as libc::c_int as libc::c_uint != 0 {
                if seen != 0 {
                    (*r_ptr).r_flags3 |= 0x2000 as libc::c_int as libc::c_uint
                }
                note =
                    b" loses some skin!\x00" as *const u8 as
                        *const libc::c_char;
                note_dies =
                    b" evaporates!\x00" as *const u8 as *const libc::c_char;
                dam *= 2 as libc::c_int
            }
            if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
                if Rand_div((*m_ptr).level as libc::c_int + 10 as libc::c_int)
                       > Rand_div((*p_ptr).lev as s32b) {
                    note =
                        b" resists.\x00" as *const u8 as *const libc::c_char;
                    dam >>= 3 as libc::c_int
                }
            }
        }
        102 => {
            if (*r_ptr).flags3 & 0x10000000 as libc::c_int as libc::c_uint !=
                   0 {
                note =
                    b" is unaffected.\x00" as *const u8 as *const libc::c_char
            } else {
                set_afraid((*p_ptr).afraid as libc::c_int +
                               dam / 2 as libc::c_int +
                               (Rand_div(dam / 2 as libc::c_int) +
                                    1 as libc::c_int));
            }
            /* No damage */
            dam = 0 as libc::c_int
        }
        85 => {
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).flags2 & 0x40 as libc::c_int as libc::c_uint != 0 {
                dam = 0 as libc::c_int;
                note = b" is immune!\x00" as *const u8 as *const libc::c_char
            } else if (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint !=
                          0 ||
                          (*r_ptr).flags2 &
                              0x80 as libc::c_int as libc::c_uint != 0 ||
                          (*r_ptr).flags3 &
                              0x80 as libc::c_int as libc::c_uint != 0 ||
                          (*m_ptr).level as libc::c_int >
                              Rand_div(3 as libc::c_int * dam) +
                                  1 as libc::c_int {
                dam /= 3 as libc::c_int;
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                /* Powerful demons & undead can turn a mindcrafter's
				* attacks back on them */
                if ((*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0
                        ||
                        (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint
                            != 0) &&
                       (*m_ptr).level as libc::c_int >
                           (*p_ptr).lev as libc::c_int / 2 as libc::c_int &&
                       Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                    note = 0 as cptr;
                    msg_format(b"%^s%s corrupted mind backlashes your attack!\x00"
                                   as *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr(),
                               if seen as libc::c_int != 0 {
                                   b"\'s\x00" as *const u8 as
                                       *const libc::c_char
                               } else {
                                   b"s\x00" as *const u8 as
                                       *const libc::c_char
                               });
                    /* Saving throw */
                    if Rand_div(100 as libc::c_int) <
                           (*p_ptr).skill_sav as libc::c_int {
                        msg_print(b"You resist the effects!\x00" as *const u8
                                      as *const libc::c_char);
                    } else {
                        /* Injure +/- confusion */
                        monster_desc(killer.as_mut_ptr(), m_ptr,
                                     0x88 as
                                         libc::c_int); /* has already been /3 */
                        take_hit(dam, killer.as_mut_ptr() as cptr);
                        if Rand_div(4 as libc::c_int) + 1 as libc::c_int ==
                               1 as libc::c_int {
                            match Rand_div(4 as libc::c_int) +
                                      1 as libc::c_int {
                                1 => {
                                    set_confused((*p_ptr).confused as
                                                     libc::c_int +
                                                     3 as libc::c_int +
                                                     (Rand_div(dam) +
                                                          1 as libc::c_int));
                                }
                                2 => {
                                    set_stun((*p_ptr).stun as libc::c_int +
                                                 (Rand_div(dam) +
                                                      1 as libc::c_int));
                                }
                                3 => {
                                    if (*r_ptr).flags3 &
                                           0x10000000 as libc::c_int as
                                               libc::c_uint != 0 {
                                        note =
                                            b" is unaffected.\x00" as
                                                *const u8 as
                                                *const libc::c_char
                                    } else {
                                        set_afraid((*p_ptr).afraid as
                                                       libc::c_int +
                                                       3 as libc::c_int +
                                                       (Rand_div(dam) +
                                                            1 as
                                                                libc::c_int));
                                    }
                                }
                                _ => {
                                    if (*p_ptr).free_act == 0 {
                                        set_paralyzed((*p_ptr).paralyzed as
                                                          libc::c_int +
                                                          (Rand_div(dam) +
                                                               1 as
                                                                   libc::c_int));
                                    }
                                }
                            }
                        }
                    }
                    dam = 0 as libc::c_int
                }
            }
            if dam > 0 as libc::c_int &&
                   Rand_div(4 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                match Rand_div(4 as libc::c_int) + 1 as libc::c_int {
                    1 => {
                        do_conf =
                            3 as libc::c_int +
                                (Rand_div(dam) + 1 as libc::c_int)
                    }
                    2 => {
                        do_stun =
                            3 as libc::c_int +
                                (Rand_div(dam) + 1 as libc::c_int)
                    }
                    3 => {
                        do_fear =
                            3 as libc::c_int +
                                (Rand_div(dam) + 1 as libc::c_int)
                    }
                    _ => {
                        do_sleep =
                            3 as libc::c_int +
                                (Rand_div(dam) + 1 as libc::c_int)
                    }
                }
            }
            note_dies =
                b" collapses, a mindless husk.\x00" as *const u8 as
                    *const libc::c_char
        }
        86 => {
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).flags2 & 0x40 as libc::c_int as libc::c_uint != 0 {
                dam = 0 as libc::c_int;
                note = b" is immune!\x00" as *const u8 as *const libc::c_char
            } else if (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint !=
                          0 ||
                          (*r_ptr).flags2 &
                              0x80 as libc::c_int as libc::c_uint != 0 ||
                          (*r_ptr).flags3 &
                              0x80 as libc::c_int as libc::c_uint != 0 ||
                          (*m_ptr).level as libc::c_int >
                              Rand_div(3 as libc::c_int * dam) +
                                  1 as libc::c_int {
                dam /= 3 as libc::c_int;
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                /*
				 * Powerful demons & undead can turn a mindcrafter's
				 * attacks back on them
				 */
                if ((*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0
                        ||
                        (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint
                            != 0) &&
                       (*m_ptr).level as libc::c_int >
                           (*p_ptr).lev as libc::c_int / 2 as libc::c_int &&
                       Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                    note = 0 as cptr;
                    msg_format(b"%^s%s corrupted mind backlashes your attack!\x00"
                                   as *const u8 as *const libc::c_char,
                               m_name.as_mut_ptr(),
                               if seen as libc::c_int != 0 {
                                   b"\'s\x00" as *const u8 as
                                       *const libc::c_char
                               } else {
                                   b"s\x00" as *const u8 as
                                       *const libc::c_char
                               });
                    /* Saving throw */
                    if Rand_div(100 as libc::c_int) <
                           (*p_ptr).skill_sav as libc::c_int {
                        msg_print(b"You resist the effects!\x00" as *const u8
                                      as *const libc::c_char);
                    } else {
                        /* Injure + mana drain */
                        monster_desc(killer.as_mut_ptr(), m_ptr,
                                     0x88 as libc::c_int);
                        msg_print(b"Your psychic energy is drained!\x00" as
                                      *const u8 as *const libc::c_char);
                        (*p_ptr).csp =
                            if (0 as libc::c_int) <
                                   (*p_ptr).csp as libc::c_int -
                                       damroll(5 as libc::c_int as s16b,
                                               dam as s16b) / 2 as libc::c_int
                               {
                                ((*p_ptr).csp as libc::c_int) -
                                    damroll(5 as libc::c_int as s16b,
                                            dam as s16b) / 2 as libc::c_int
                            } else { 0 as libc::c_int } as s16b;
                        (*p_ptr).redraw =
                            ((*p_ptr).redraw as libc::c_long |
                                 0x80 as libc::c_long) as u32b;
                        take_hit(dam, killer.as_mut_ptr() as cptr);
                        /* has already been /3 */
                    }
                    dam = 0 as libc::c_int
                }
            } else if dam > 0 as libc::c_int {
                let mut b_0: libc::c_int =
                    damroll(5 as libc::c_int as s16b, dam as s16b) /
                        4 as libc::c_int;
                msg_format(b"You convert %s%s pain into psychic energy!\x00"
                               as *const u8 as *const libc::c_char,
                           m_name.as_mut_ptr(),
                           if seen as libc::c_int != 0 {
                               b"\'s\x00" as *const u8 as *const libc::c_char
                           } else {
                               b"s\x00" as *const u8 as *const libc::c_char
                           });
                b_0 =
                    if (*p_ptr).msp as libc::c_int >
                           (*p_ptr).csp as libc::c_int + b_0 {
                        ((*p_ptr).csp as libc::c_int) + b_0
                    } else { (*p_ptr).msp as libc::c_int };
                (*p_ptr).csp = b_0 as s16b;
                (*p_ptr).redraw =
                    ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long)
                        as u32b
            }
            note_dies =
                b" collapses, a mindless husk.\x00" as *const u8 as
                    *const libc::c_char
        }
        87 => {
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            do_dist = 7 as libc::c_int;
            /* 1. stun */
            do_stun =
                damroll(((*p_ptr).lev as libc::c_int / 10 as libc::c_int +
                             3 as libc::c_int) as s16b, dam as s16b) +
                    1 as libc::c_int;
            /* Attempt a saving throw */
            if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 ||
                   (*m_ptr).level as libc::c_int >
                       5 as libc::c_int + (Rand_div(dam) + 1 as libc::c_int) {
                /* Resist */
                do_stun = 0 as libc::c_int;
                /* No obvious effect */
                obvious = 0 as libc::c_int as bool_
            }
        }
        27 => {
            /* Meteor -- powerful magic missile */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
        }
        89 => {
            if !(is_friend(m_ptr) > 0 as libc::c_int) {
                if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                /* Attempt a saving throw */
                if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0
                       ||
                       (*r_ptr).flags3 &
                           0x40000000 as libc::c_int as libc::c_uint != 0 ||
                       (*m_ptr).level as libc::c_int >
                           Rand_div((if (dam - 10 as libc::c_int) <
                                            1 as libc::c_int {
                                         1 as libc::c_int
                                     } else { (dam) - 10 as libc::c_int })) +
                               1 as libc::c_int + 10 as libc::c_int {
                    /* Memorize a flag */
                    if (*r_ptr).flags3 &
                           0x40000000 as libc::c_int as libc::c_uint != 0 {
                        if seen != 0 {
                            (*r_ptr).r_flags3 |=
                                0x40000000 as libc::c_int as libc::c_uint
                        }
                    }
                    /* Resist */
                    do_conf = 0 as libc::c_int;
                    /*
				 * Powerful demons & undead can turn a mindcrafter's
				 * attacks back on them
				 */
                    if ((*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint
                            != 0 ||
                            (*r_ptr).flags3 &
                                0x10 as libc::c_int as libc::c_uint != 0) &&
                           (*m_ptr).level as libc::c_int >
                               (*p_ptr).lev as libc::c_int / 2 as libc::c_int
                           &&
                           Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                               1 as libc::c_int {
                        note = 0 as cptr;
                        msg_format(b"%^s%s corrupted mind backlashes your attack!\x00"
                                       as *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr(),
                                   if seen as libc::c_int != 0 {
                                       b"\'s\x00" as *const u8 as
                                           *const libc::c_char
                                   } else {
                                       b"s\x00" as *const u8 as
                                           *const libc::c_char
                                   });
                        /* Saving throw */
                        if Rand_div(100 as libc::c_int) <
                               (*p_ptr).skill_sav as libc::c_int {
                            msg_print(b"You resist the effects!\x00" as
                                          *const u8 as *const libc::c_char);
                        } else {
                            /* Confuse, stun, terrify */
                            match Rand_div(4 as libc::c_int) +
                                      1 as libc::c_int {
                                1 => {
                                    set_stun((*p_ptr).stun as libc::c_int +
                                                 dam / 2 as libc::c_int);
                                }
                                2 => {
                                    set_confused((*p_ptr).confused as
                                                     libc::c_int +
                                                     dam / 2 as libc::c_int);
                                }
                                _ => {
                                    if (*r_ptr).flags3 &
                                           0x10000000 as libc::c_int as
                                               libc::c_uint != 0 {
                                        note =
                                            b" is unaffected.\x00" as
                                                *const u8 as
                                                *const libc::c_char
                                    } else {
                                        set_afraid((*p_ptr).afraid as
                                                       libc::c_int + dam);
                                    }
                                }
                            }
                        }
                    } else {
                        /* No obvious effect */
                        note =
                            b" is unaffected!\x00" as *const u8 as
                                *const libc::c_char;
                        obvious = 0 as libc::c_int as bool_
                    }
                } else if dam > 29 as libc::c_int &&
                              (Rand_div(100 as libc::c_int) +
                                   1 as libc::c_int) < dam {
                    note =
                        b" is in your thrall!\x00" as *const u8 as
                            *const libc::c_char;
                    (*m_ptr).status = 3 as libc::c_int as s16b;
                    if (*r_ptr).flags3 & 0x80 as libc::c_int as libc::c_uint
                           != 0 &&
                           (*r_ptr).flags3 &
                               0x40 as libc::c_int as libc::c_uint == 0 {
                        inc_piety(5 as libc::c_int,
                                  (*m_ptr).level as libc::c_int *
                                      2 as libc::c_int);
                    }
                } else {
                    match Rand_div(4 as libc::c_int) + 1 as libc::c_int {
                        1 => { do_stun = dam / 2 as libc::c_int }
                        2 => { do_conf = dam / 2 as libc::c_int }
                        _ => { do_fear = dam }
                    }
                }
                /* No "real" damage */
                dam = 0 as libc::c_int
            }
        }
        28 => {
            /* Ice -- Cold + Cuts + Stun */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            do_stun =
                (Rand_div(15 as libc::c_int) + 1 as libc::c_int +
                     1 as libc::c_int) / (r + 1 as libc::c_int);
            if Rand_div(100 as libc::c_int) < 33 as libc::c_int {
                do_cut =
                    (10 as libc::c_int +
                         (Rand_div(15 as libc::c_int) + 1 as libc::c_int) + r)
                        / (r + 1 as libc::c_int)
            }
            if (*r_ptr).flags3 & 0x8000 as libc::c_int as libc::c_uint != 0 {
                note =
                    b" is hit hard.\x00" as *const u8 as *const libc::c_char;
                dam *= 3 as libc::c_int;
                do_cut *= 2 as libc::c_int;
                if seen != 0 {
                    (*r_ptr).r_flags3 |= 0x8000 as libc::c_int as libc::c_uint
                }
            }
            if (*r_ptr).flags3 & 0x80000 as libc::c_int as libc::c_uint != 0 {
                note =
                    b" resists a lot.\x00" as *const u8 as
                        *const libc::c_char;
                dam /= 9 as libc::c_int;
                do_cut = 0 as libc::c_int;
                if seen != 0 {
                    (*r_ptr).r_flags3 |=
                        0x80000 as libc::c_int as libc::c_uint
                }
            }
        }
        58 => {
            /* Drain Life */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0 ||
                   (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0
                   ||
                   (*r_ptr).flags3 & 0x800 as libc::c_int as libc::c_uint != 0
                   ||
                   !strchr(b"Egv\x00" as *const u8 as *const libc::c_char,
                           (*r_ptr).d_char as libc::c_int).is_null() {
                if (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0
                   {
                    if seen != 0 {
                        (*r_ptr).r_flags3 |=
                            0x20 as libc::c_int as libc::c_uint
                    }
                }
                if (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0
                   {
                    if seen != 0 {
                        (*r_ptr).r_flags3 |=
                            0x10 as libc::c_int as libc::c_uint
                    }
                }
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                obvious = 0 as libc::c_int as bool_;
                dam = 0 as libc::c_int
            }
        }
        77 => {
            /* Death Ray */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0 ||
                   (*r_ptr).flags3 & 0x800 as libc::c_int as libc::c_uint != 0
               {
                if (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0
                   {
                    if seen != 0 {
                        (*r_ptr).r_flags3 |=
                            0x20 as libc::c_int as libc::c_uint
                    }
                }
                note = b" is immune.\x00" as *const u8 as *const libc::c_char;
                obvious = 0 as libc::c_int as bool_;
                dam = 0 as libc::c_int
            } else if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint !=
                          0 &&
                          Rand_div(888 as libc::c_int) + 1 as libc::c_int !=
                              666 as libc::c_int ||
                          (*m_ptr).level as libc::c_int +
                              (Rand_div(20 as libc::c_int) + 1 as libc::c_int)
                              >
                              Rand_div(dam +
                                           (Rand_div(10 as libc::c_int) +
                                                1 as libc::c_int)) +
                                  1 as libc::c_int &&
                              Rand_div(100 as libc::c_int) + 1 as libc::c_int
                                  != 66 as libc::c_int {
                note = b" resists!\x00" as *const u8 as *const libc::c_char;
                obvious = 0 as libc::c_int as bool_;
                dam = 0 as libc::c_int
            } else { dam = (*p_ptr).lev as libc::c_int * 200 as libc::c_int }
        }
        52 => {
            /* Polymorph monster (Use "dam" as "power") */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Attempt to polymorph (see below) */
            do_poly = 1 as libc::c_int;
            /* Powerful monsters can resist */
            if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 ||
                   (*m_ptr).mflag & 0x2 as libc::c_int != 0 ||
                   (*m_ptr).level as libc::c_int >
                       Rand_div((if (dam - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (dam) - 10 as libc::c_int })) +
                           1 as libc::c_int + 10 as libc::c_int {
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                do_poly = 0 as libc::c_int;
                obvious = 0 as libc::c_int as bool_
            }
            /* No "real" damage */
            dam = 0 as libc::c_int
        }
        51 => {
            /* Clone monsters (Ignore "dam") */
            let mut is_frien: bool_ = 0 as libc::c_int as bool_;
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if is_friend(m_ptr) > 0 as libc::c_int &&
                   Rand_div(3 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                is_frien = 1 as libc::c_int as bool_
            }
            /* Heal fully */
            (*m_ptr).hp = (*m_ptr).maxhp;
            /* Speed up */
            if ((*m_ptr).mspeed as libc::c_int) < 150 as libc::c_int {
                (*m_ptr).mspeed =
                    ((*m_ptr).mspeed as libc::c_int + 10 as libc::c_int) as
                        byte_hack
            }
            /* Attempt to clone. */
            if multiply_monster((*c_ptr).m_idx as libc::c_int, is_frien,
                                1 as libc::c_int as bool_) != 0 {
                note = b" spawns!\x00" as *const u8 as *const libc::c_char
            }
            /* No "real" damage */
            dam = 0 as libc::c_int
        }
        53 => {
            /* Heal Monster (use "dam" as amount of healing) */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Wake up */
            (*m_ptr).csleep = 0 as libc::c_int as s16b;
            /* Heal */
            (*m_ptr).hp += dam;
            /* No overflow */
            if (*m_ptr).hp > (*m_ptr).maxhp { (*m_ptr).hp = (*m_ptr).maxhp }
            /* Redraw (later) if needed */
            if health_who as libc::c_int == (*c_ptr).m_idx as libc::c_int {
                (*p_ptr).redraw =
                    ((*p_ptr).redraw as libc::c_long | 0x800 as libc::c_long)
                        as u32b
            }
            /* Message */
            note =
                b" looks healthier.\x00" as *const u8 as *const libc::c_char;
            /* No "real" damage */
            dam = 0 as libc::c_int
        }
        54 => {
            /* Speed Monster (Ignore "dam") */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Speed up */
            if ((*m_ptr).mspeed as libc::c_int) <
                   (*m_ptr).speed as libc::c_int + 15 as libc::c_int {
                (*m_ptr).mspeed =
                    ((*m_ptr).mspeed as libc::c_int + 10 as libc::c_int) as
                        byte_hack
            }
            note =
                b" starts moving faster.\x00" as *const u8 as
                    *const libc::c_char;
            /* No "real" damage */
            dam = 0 as libc::c_int
        }
        55 => {
            /* Slow Monster (Use "dam" as "power") */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Powerful monsters can resist */
            if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 ||
                   (*m_ptr).level as libc::c_int >
                       Rand_div((if (dam - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (dam) - 10 as libc::c_int })) +
                           1 as libc::c_int + 10 as libc::c_int {
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                obvious = 0 as libc::c_int as bool_
            } else {
                /* Normal monsters slow down */
                if (*m_ptr).mspeed as libc::c_int > 60 as libc::c_int {
                    (*m_ptr).mspeed =
                        ((*m_ptr).mspeed as libc::c_int - 10 as libc::c_int)
                            as byte_hack
                }
                note =
                    b" starts moving slower.\x00" as *const u8 as
                        *const libc::c_char
            }
            /* No "real" damage */
            dam = 0 as libc::c_int
        }
        57 => {
            /* Sleep (Use "dam" as "power") */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Attempt a saving throw */
            if (*r_ptr).flags3 & 0x80000000 as libc::c_uint != 0 ||
                   (*m_ptr).level as libc::c_int >
                       Rand_div((if (dam - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (dam) - 10 as libc::c_int })) +
                           1 as libc::c_int + 10 as libc::c_int {
                /* Memorize a flag */
                if (*r_ptr).flags3 & 0x80000000 as libc::c_uint != 0 {
                    if seen != 0 {
                        (*r_ptr).r_flags3 |= 0x80000000 as libc::c_uint
                    }
                }
                /* No obvious effect */
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                obvious = 0 as libc::c_int as bool_
            } else {
                /* Go to sleep (much) later */
                note =
                    b" falls asleep!\x00" as *const u8 as *const libc::c_char;
                do_sleep = 500 as libc::c_int
            }
            /* No "real" damage */
            dam = 0 as libc::c_int
        }
        75 => {
            /* Sleep (Use "dam" as "power") */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Attempt a saving throw */
            if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 ||
                   (*m_ptr).level as libc::c_int >
                       Rand_div((if (dam - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (dam) - 10 as libc::c_int })) +
                           1 as libc::c_int + 10 as libc::c_int {
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                obvious = 0 as libc::c_int as bool_
            } else {
                /* Go to sleep (much) later */
                note =
                    b" is suspended!\x00" as *const u8 as *const libc::c_char;
                do_sleep = 500 as libc::c_int
            }
            /* No "real" damage */
            dam = 0 as libc::c_int
        }
        82 => {
            /* Charm monster */
            dam +=
                *adj_con_fix.as_mut_ptr().offset((*p_ptr).stat_ind[5 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                                     as isize) as libc::c_int
                    - 1 as libc::c_int;
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Attempt a saving throw */
            if (*m_ptr).mflag & 0x2 as libc::c_int != 0 ||
                   (*r_ptr).flags3 & 0x40000000 as libc::c_int as libc::c_uint
                       != 0 ||
                   (*m_ptr).level as libc::c_int >
                       Rand_div((if (dam - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (dam) - 10 as libc::c_int })) +
                           1 as libc::c_int + 5 as libc::c_int {
                /* Memorize a flag */
                if (*r_ptr).flags3 & 0x40000000 as libc::c_int as libc::c_uint
                       != 0 {
                    if seen != 0 {
                        (*r_ptr).r_flags3 |=
                            0x40000000 as libc::c_int as libc::c_uint
                    }
                }
                /* Resist */
				/* No obvious effect */
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                obvious = 0 as libc::c_int as bool_
            } else if (*p_ptr).aggravate != 0 {
                note =
                    b" hates you too much!\x00" as *const u8 as
                        *const libc::c_char
            } else if is_friend(m_ptr) < 0 as libc::c_int {
                note =
                    b" suddenly seems friendly!\x00" as *const u8 as
                        *const libc::c_char;
                (*m_ptr).status = 2 as libc::c_int as s16b;
                if (*r_ptr).flags3 & 0x80 as libc::c_int as libc::c_uint != 0
                       &&
                       (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint
                           == 0 {
                    inc_piety(5 as libc::c_int,
                              (*m_ptr).level as libc::c_int *
                                  2 as libc::c_int);
                }
            }
            /* No "real" damage */
            dam = 0 as libc::c_int
        }
        99 => {
            /* *Charm* monster */
            dam +=
                *adj_con_fix.as_mut_ptr().offset((*p_ptr).stat_ind[5 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                                     as isize) as libc::c_int
                    - 1 as libc::c_int;
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Attempt a saving throw */
            if (*m_ptr).mflag & 0x2 as libc::c_int != 0 ||
                   (*r_ptr).flags3 & 0x40000000 as libc::c_int as libc::c_uint
                       != 0 ||
                   (*m_ptr).level as libc::c_int >
                       Rand_div((if (dam - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (dam) - 10 as libc::c_int })) +
                           1 as libc::c_int + 5 as libc::c_int {
                /* Memorize a flag */
                if (*r_ptr).flags3 & 0x40000000 as libc::c_int as libc::c_uint
                       != 0 {
                    if seen != 0 {
                        (*r_ptr).r_flags3 |=
                            0x40000000 as libc::c_int as libc::c_uint
                    }
                }
                /* Resist */
				/* No obvious effect */
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                obvious = 0 as libc::c_int as bool_
            } else if (*p_ptr).aggravate != 0 {
                note =
                    b" hates you too much!\x00" as *const u8 as
                        *const libc::c_char
            } else if is_friend(m_ptr) < 0 as libc::c_int {
                note =
                    b" suddenly seems friendly!\x00" as *const u8 as
                        *const libc::c_char;
                if can_create_companion() != 0 {
                    (*m_ptr).status = 4 as libc::c_int as s16b
                } else { (*m_ptr).status = 3 as libc::c_int as s16b }
                if (*r_ptr).flags3 & 0x80 as libc::c_int as libc::c_uint != 0
                       &&
                       (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint
                           == 0 {
                    inc_piety(5 as libc::c_int,
                              (*m_ptr).level as libc::c_int *
                                  2 as libc::c_int);
                }
            }
            /* No "real" damage */
            dam = 0 as libc::c_int
        }
        83 => {
            /* Control undead */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Attempt a saving throw */
            if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 ||
                   (*m_ptr).mflag & 0x2 as libc::c_int != 0 ||
                   (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint == 0
                   ||
                   (*m_ptr).level as libc::c_int >
                       Rand_div((if (dam - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (dam) - 10 as libc::c_int })) +
                           1 as libc::c_int + 10 as libc::c_int {
                /* Resist */
				/* No obvious effect */
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                obvious = 0 as libc::c_int as bool_
            } else if (*p_ptr).aggravate != 0 {
                note =
                    b" hates you too much!\x00" as *const u8 as
                        *const libc::c_char
            } else {
                note =
                    b" is in your thrall!\x00" as *const u8 as
                        *const libc::c_char;
                (*m_ptr).status = 3 as libc::c_int as s16b
            }
            /* No "real" damage */
            dam = 0 as libc::c_int
        }
        110 => {
            /* Control never-moving */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Attempt a saving throw */
            if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 ||
                   (*m_ptr).mflag & 0x2 as libc::c_int != 0 ||
                   (*r_ptr).flags1 & 0x20000 as libc::c_int as libc::c_uint ==
                       0 ||
                   (*m_ptr).level as libc::c_int >
                       Rand_div((if (dam - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (dam) - 10 as libc::c_int })) +
                           1 as libc::c_int + 10 as libc::c_int {
                /* Resist */
				/* No obvious effect */
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                obvious = 0 as libc::c_int as bool_
            } else if (*p_ptr).aggravate != 0 {
                note =
                    b" hates you too much!\x00" as *const u8 as
                        *const libc::c_char
            } else {
                note =
                    b" is in your thrall!\x00" as *const u8 as
                        *const libc::c_char;
                (*m_ptr).status = 3 as libc::c_int as s16b
            }
            /* No "real" damage */
            dam = 0 as libc::c_int
        }
        84 => {
            /* Tame animal */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Attempt a saving throw */
            if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 ||
                   (*m_ptr).mflag & 0x2 as libc::c_int != 0 ||
                   (*r_ptr).flags3 & 0x80 as libc::c_int as libc::c_uint == 0
                   ||
                   (*r_ptr).flags3 & 0x40000000 as libc::c_int as libc::c_uint
                       != 0 ||
                   (*m_ptr).level as libc::c_int >
                       Rand_div((if (dam - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (dam) - 10 as libc::c_int })) +
                           1 as libc::c_int + 10 as libc::c_int {
                /* Memorize a flag */
                if (*r_ptr).flags3 & 0x40000000 as libc::c_int as libc::c_uint
                       != 0 {
                    if seen != 0 {
                        (*r_ptr).r_flags3 |=
                            0x40000000 as libc::c_int as libc::c_uint
                    }
                }
                /* Resist */
				/* No obvious effect */
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                obvious = 0 as libc::c_int as bool_
            } else if (*p_ptr).aggravate != 0 {
                note =
                    b" hates you too much!\x00" as *const u8 as
                        *const libc::c_char
            } else {
                note = b" is tamed!\x00" as *const u8 as *const libc::c_char;
                (*m_ptr).status = 3 as libc::c_int as s16b;
                inc_piety(5 as libc::c_int,
                          (*m_ptr).level as libc::c_int * 2 as libc::c_int);
            }
            /* No "real" damage */
            dam = 0 as libc::c_int
        }
        106 => {
            /* Control demon */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Attempt a saving throw */
            if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 ||
                   (*m_ptr).mflag & 0x2 as libc::c_int != 0 ||
                   (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint == 0
                   ||
                   (*m_ptr).level as libc::c_int >
                       Rand_div((if (dam - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (dam) - 10 as libc::c_int })) +
                           1 as libc::c_int + 10 as libc::c_int {
                /* Memorize a flag */
                if (*r_ptr).flags3 & 0x40000000 as libc::c_int as libc::c_uint
                       != 0 {
                    if seen != 0 {
                        (*r_ptr).r_flags3 |=
                            0x40000000 as libc::c_int as libc::c_uint
                    }
                }
                /* Resist */
				/* No obvious effect */
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                obvious = 0 as libc::c_int as bool_
            } else if (*p_ptr).aggravate != 0 {
                note =
                    b" hates you too much!\x00" as *const u8 as
                        *const libc::c_char
            } else {
                note =
                    b" obeys your commands!\x00" as *const u8 as
                        *const libc::c_char;
                (*m_ptr).status = 3 as libc::c_int as s16b
            }
            /* No "real" damage */
            dam = 0 as libc::c_int
        }
        56 => {
            /* Confusion (Use "dam" as "power") */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Get confused later */
            do_conf =
                damroll(3 as libc::c_int as s16b,
                        (dam / 2 as libc::c_int) as s16b) + 1 as libc::c_int;
            /* Attempt a saving throw */
            if (*r_ptr).flags3 & 0x40000000 as libc::c_int as libc::c_uint !=
                   0 ||
                   (*m_ptr).level as libc::c_int >
                       Rand_div((if (dam - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (dam) - 10 as libc::c_int })) +
                           1 as libc::c_int + 10 as libc::c_int {
                /* Memorize a flag */
                if (*r_ptr).flags3 & 0x40000000 as libc::c_int as libc::c_uint
                       != 0 {
                    if seen != 0 {
                        (*r_ptr).r_flags3 |=
                            0x40000000 as libc::c_int as libc::c_uint
                    }
                }
                /* Resist */
                do_conf = 0 as libc::c_int;
                /* No obvious effect */
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                obvious = 0 as libc::c_int as bool_
            }
            /* No "real" damage */
            dam = 0 as libc::c_int
        }
        78 => {
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            do_stun =
                damroll(((*p_ptr).lev as libc::c_int / 10 as libc::c_int +
                             3 as libc::c_int) as s16b, dam as s16b) +
                    1 as libc::c_int;
            /* Attempt a saving throw */
            if (*m_ptr).level as libc::c_int >
                   Rand_div((if (dam - 10 as libc::c_int) < 1 as libc::c_int {
                                 1 as libc::c_int
                             } else { (dam) - 10 as libc::c_int })) +
                       1 as libc::c_int + 10 as libc::c_int {
                /* Resist */
                do_stun = 0 as libc::c_int;
                /* No obvious effect */
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                obvious = 0 as libc::c_int as bool_
            }
            /* No "real" damage */
            dam = 0 as libc::c_int
        }
        98 => {
            /* Confusion (Use "dam" as "power") */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Get confused later */
            do_conf =
                damroll(3 as libc::c_int as s16b,
                        (dam / 2 as libc::c_int) as s16b) + 1 as libc::c_int;
            /* Attempt a saving throw */
            if (*r_ptr).flags3 & 0x40000000 as libc::c_int as libc::c_uint !=
                   0 ||
                   (*m_ptr).level as libc::c_int >
                       Rand_div((if (dam - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (dam) - 10 as libc::c_int })) +
                           1 as libc::c_int + 10 as libc::c_int {
                /* Memorize a flag */
                if (*r_ptr).flags3 & 0x40000000 as libc::c_int as libc::c_uint
                       != 0 {
                    if seen != 0 {
                        (*r_ptr).r_flags3 |=
                            0x40000000 as libc::c_int as libc::c_uint
                    }
                }
                /* Resist */
                do_conf = 0 as libc::c_int;
                /* No obvious effect */
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                obvious = 0 as libc::c_int as bool_
            }
        }
        96 => {
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            do_stun =
                damroll(((*p_ptr).lev as libc::c_int / 10 as libc::c_int +
                             3 as libc::c_int) as s16b, dam as s16b) +
                    1 as libc::c_int;
            /* Attempt a saving throw */
            if (*m_ptr).level as libc::c_int >
                   Rand_div((if (dam - 10 as libc::c_int) < 1 as libc::c_int {
                                 1 as libc::c_int
                             } else { (dam) - 10 as libc::c_int })) +
                       1 as libc::c_int + 10 as libc::c_int {
                /* Resist */
                do_stun = 0 as libc::c_int;
                /* No obvious effect */
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                obvious = 0 as libc::c_int as bool_
            }
        }
        100 => {
            /* Implosion is the same than Stun_dam but only affect the living */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            do_stun =
                damroll(((*p_ptr).lev as libc::c_int / 10 as libc::c_int +
                             3 as libc::c_int) as s16b, dam as s16b) +
                    1 as libc::c_int;
            /* Attempt a saving throw */
            if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 ||
                   (*m_ptr).level as libc::c_int >
                       Rand_div((if (dam - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (dam) - 10 as libc::c_int })) +
                           1 as libc::c_int + 10 as libc::c_int {
                /* Resist */
                do_stun = 0 as libc::c_int;
                /* No obvious effect */
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                obvious = 0 as libc::c_int as bool_
            }
            /* Non_living resists */
            if (*r_ptr).flags3 & 0x800 as libc::c_int as libc::c_uint != 0 {
                /* Resist */
                do_stun = 0 as libc::c_int;
                dam = 0 as libc::c_int;
                /* No obvious effect */
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                obvious = 0 as libc::c_int as bool_
            }
        }
        95 => {
            /* Confusion & Stunning (Use "dam" as "power") */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Get confused later */
            do_conf =
                damroll(3 as libc::c_int as s16b,
                        (dam / 2 as libc::c_int) as s16b) + 1 as libc::c_int;
            /* Attempt a saving throw */
            if (*r_ptr).flags3 & 0x40000000 as libc::c_int as libc::c_uint !=
                   0 ||
                   (*m_ptr).level as libc::c_int >
                       Rand_div((if (dam - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (dam) - 10 as libc::c_int })) +
                           1 as libc::c_int + 10 as libc::c_int {
                /* Memorize a flag */
                if (*r_ptr).flags3 & 0x40000000 as libc::c_int as libc::c_uint
                       != 0 {
                    if seen != 0 {
                        (*r_ptr).r_flags3 |=
                            0x40000000 as libc::c_int as libc::c_uint
                    }
                }
                /* Resist */
                do_conf = 0 as libc::c_int;
                /* No obvious effect */
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                obvious = 0 as libc::c_int as bool_
            }
            do_stun =
                damroll(((*p_ptr).lev as libc::c_int / 10 as libc::c_int +
                             3 as libc::c_int) as s16b, dam as s16b) +
                    1 as libc::c_int;
            /* Attempt a saving throw */
            if (*m_ptr).level as libc::c_int >
                   Rand_div((if (dam - 10 as libc::c_int) < 1 as libc::c_int {
                                 1 as libc::c_int
                             } else { (dam) - 10 as libc::c_int })) +
                       1 as libc::c_int + 10 as libc::c_int {
                /* Resist */
                do_stun = 0 as libc::c_int;
                /* No obvious effect */
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                obvious = 0 as libc::c_int as bool_
            }
        }
        17 => {
            /* Lite, but only hurts susceptible creatures */
            /* Hurt by light */
            if (*r_ptr).flags3 & 0x1000 as libc::c_int as libc::c_uint != 0 {
                /* Obvious effect */
                if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                /* Memorize the effects */
                if seen != 0 {
                    (*r_ptr).r_flags3 |= 0x1000 as libc::c_int as libc::c_uint
                }
                /* Special effect */
                note =
                    b" cringes from the light!\x00" as *const u8 as
                        *const libc::c_char;
                note_dies =
                    b" shrivels away in the light!\x00" as *const u8 as
                        *const libc::c_char
            } else {
                /* Normally no damage */
                /* No damage */
                dam = 0 as libc::c_int
            }
        }
        15 => {
            /* Lite -- opposite of Dark */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            if (*r_ptr).flags4 & 0x4000 as libc::c_int as libc::c_uint != 0 {
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                dam *= 2 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int
            } else if (*r_ptr).flags3 & 0x1000 as libc::c_int as libc::c_uint
                          != 0 {
                if seen != 0 {
                    (*r_ptr).r_flags3 |= 0x1000 as libc::c_int as libc::c_uint
                }
                note =
                    b" cringes from the light!\x00" as *const u8 as
                        *const libc::c_char;
                note_dies =
                    b" shrivels away in the light!\x00" as *const u8 as
                        *const libc::c_char;
                dam *= 2 as libc::c_int
            }
        }
        16 => {
            /* Dark -- opposite of Lite */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Likes darkness... */
            if (*r_ptr).flags4 & 0x8000 as libc::c_int as libc::c_uint != 0 ||
                   (*r_ptr).flags3 & 0x1 as libc::c_int as libc::c_uint != 0
                   ||
                   (*r_ptr).flags3 & 0x1000 as libc::c_int as libc::c_uint !=
                       0 {
                note = b" resists.\x00" as *const u8 as *const libc::c_char;
                dam *= 2 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int
            }
        }
        40 => {
            /* Stone to Mud */
            /* Hurt by rock remover */
            if (*r_ptr).flags3 & 0x2000 as libc::c_int as libc::c_uint != 0 {
                /* Notice effect */
                if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                /* Memorize the effects */
                if seen != 0 {
                    (*r_ptr).r_flags3 |= 0x2000 as libc::c_int as libc::c_uint
                }
                /* Cute little message */
                note =
                    b" loses some skin!\x00" as *const u8 as
                        *const libc::c_char;
                note_dies =
                    b" dissolves!\x00" as *const u8 as *const libc::c_char
            } else {
                /* Usually, ignore the effects */
                /* No damage */
                dam = 0 as libc::c_int
            }
        }
        61 => {
            if !(dungeon_flags2 as libc::c_long & 0x8 as libc::c_long != 0)
               { /* No teleport on special levels */
                /* Only affect undead */
                if (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0
                   {
                    let mut resists_tele: bool_ = 0 as libc::c_int as bool_;
                    if (*r_ptr).flags3 &
                           0x200000 as libc::c_int as libc::c_uint != 0 {
                        if (*r_ptr).flags1 &
                               0x1 as libc::c_int as libc::c_uint != 0 {
                            if seen != 0 {
                                (*r_ptr).r_flags3 |=
                                    0x200000 as libc::c_int as libc::c_uint
                            }
                            note =
                                b" is unaffected!\x00" as *const u8 as
                                    *const libc::c_char;
                            resists_tele = 1 as libc::c_int as bool_
                        } else if (*m_ptr).level as libc::c_int >
                                      Rand_div(100 as libc::c_int) +
                                          1 as libc::c_int {
                            if seen != 0 {
                                (*r_ptr).r_flags3 |=
                                    0x200000 as libc::c_int as libc::c_uint
                            }
                            note =
                                b" resists!\x00" as *const u8 as
                                    *const libc::c_char;
                            resists_tele = 1 as libc::c_int as bool_
                        }
                    }
                    if resists_tele == 0 {
                        if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                        if seen != 0 {
                            (*r_ptr).r_flags3 |=
                                0x20 as libc::c_int as libc::c_uint
                        }
                        do_dist = dam
                    }
                } else {
                    /* Others ignore */
                    /* Irrelevant */
                    skipped = 1 as libc::c_int as bool_
                }
                /* No "real" damage */
                dam = 0 as libc::c_int
            }
        }
        62 => {
            if !(dungeon_flags2 as libc::c_long & 0x8 as libc::c_long != 0)
               { /* No teleport on special levels */
                /* Only affect evil */
                if (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint != 0
                   {
                    let mut resists_tele_0: bool_ = 0 as libc::c_int as bool_;
                    if (*r_ptr).flags3 &
                           0x200000 as libc::c_int as libc::c_uint != 0 {
                        if (*r_ptr).flags1 &
                               0x1 as libc::c_int as libc::c_uint != 0 {
                            if seen != 0 {
                                (*r_ptr).r_flags3 |=
                                    0x200000 as libc::c_int as libc::c_uint
                            }
                            note =
                                b" is unaffected!\x00" as *const u8 as
                                    *const libc::c_char;
                            resists_tele_0 = 1 as libc::c_int as bool_
                        } else if (*m_ptr).level as libc::c_int >
                                      Rand_div(100 as libc::c_int) +
                                          1 as libc::c_int {
                            if seen != 0 {
                                (*r_ptr).r_flags3 |=
                                    0x200000 as libc::c_int as libc::c_uint
                            }
                            note =
                                b" resists!\x00" as *const u8 as
                                    *const libc::c_char;
                            resists_tele_0 = 1 as libc::c_int as bool_
                        }
                    }
                    if resists_tele_0 == 0 {
                        if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                        if seen != 0 {
                            (*r_ptr).r_flags3 |=
                                0x40 as libc::c_int as libc::c_uint
                        }
                        do_dist = dam
                    }
                } else {
                    /* Others ignore */
                    /* Irrelevant */
                    skipped = 1 as libc::c_int as bool_
                }
                /* No "real" damage */
                dam = 0 as libc::c_int
            }
        }
        63 => {
            /* Teleport monster (Use "dam" as "power") */
            let mut resists_tele_1: bool_ =
                0 as libc::c_int as bool_; /* No teleport on special levels */
            if !(dungeon_flags2 as libc::c_long & 0x8 as libc::c_long != 0) {
                if (*r_ptr).flags3 & 0x200000 as libc::c_int as libc::c_uint
                       != 0 {
                    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint !=
                           0 {
                        if seen != 0 {
                            (*r_ptr).r_flags3 |=
                                0x200000 as libc::c_int as libc::c_uint
                        }
                        note =
                            b" is unaffected!\x00" as *const u8 as
                                *const libc::c_char;
                        resists_tele_1 = 1 as libc::c_int as bool_
                    } else if (*m_ptr).level as libc::c_int >
                                  Rand_div(100 as libc::c_int) +
                                      1 as libc::c_int {
                        if seen != 0 {
                            (*r_ptr).r_flags3 |=
                                0x200000 as libc::c_int as libc::c_uint
                        }
                        note =
                            b" resists!\x00" as *const u8 as
                                *const libc::c_char;
                        resists_tele_1 = 1 as libc::c_int as bool_
                    }
                }
                if resists_tele_1 == 0 {
                    /* Obvious */
                    if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                    /* Prepare to teleport */
                    do_dist = dam
                }
                /* No "real" damage */
                dam = 0 as libc::c_int
            }
        }
        64 => {
            /* Turn undead (Use "dam" as "power") */
            /* Only affect undead */
            if (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0 {
                /* Learn about type */
                if seen != 0 {
                    (*r_ptr).r_flags3 |= 0x20 as libc::c_int as libc::c_uint
                }
                /* Obvious */
                if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                /* Apply some fear */
                do_fear =
                    damroll(3 as libc::c_int as s16b,
                            (dam / 2 as libc::c_int) as s16b) +
                        1 as libc::c_int;
                /* Attempt a saving throw */
                if (*m_ptr).level as libc::c_int >
                       Rand_div((if (dam - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (dam) - 10 as libc::c_int })) +
                           1 as libc::c_int + 10 as libc::c_int {
                    /* No obvious effect */
                    note =
                        b" is unaffected!\x00" as *const u8 as
                            *const libc::c_char;
                    obvious = 0 as libc::c_int as bool_;
                    do_fear = 0 as libc::c_int
                }
            } else {
                /* Others ignore */
                /* Irrelevant */
                skipped = 1 as libc::c_int as bool_
            }
            /* No "real" damage */
            dam = 0 as libc::c_int
        }
        65 => {
            /* Turn evil (Use "dam" as "power") */
            /* Only affect evil */
            if (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint != 0 {
                /* Learn about type */
                if seen != 0 {
                    (*r_ptr).r_flags3 |= 0x40 as libc::c_int as libc::c_uint
                }
                /* Obvious */
                if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                /* Apply some fear */
                do_fear =
                    damroll(3 as libc::c_int as s16b,
                            (dam / 2 as libc::c_int) as s16b) +
                        1 as libc::c_int;
                /* Attempt a saving throw */
                if (*m_ptr).level as libc::c_int >
                       Rand_div((if (dam - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (dam) - 10 as libc::c_int })) +
                           1 as libc::c_int + 10 as libc::c_int {
                    /* No obvious effect */
                    note =
                        b" is unaffected!\x00" as *const u8 as
                            *const libc::c_char;
                    obvious = 0 as libc::c_int as bool_;
                    do_fear = 0 as libc::c_int
                }
            } else {
                /* Others ignore */
                /* Irrelevant */
                skipped = 1 as libc::c_int as bool_
            }
            /* No "real" damage */
            dam = 0 as libc::c_int
        }
        66 => {
            /* Turn monster (Use "dam" as "power") */
            /* Obvious */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Apply some fear */
            do_fear =
                damroll(3 as libc::c_int as s16b,
                        (dam / 2 as libc::c_int) as s16b) + 1 as libc::c_int;
            /* Attempt a saving throw */
            if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 ||
                   (*r_ptr).flags3 & 0x10000000 as libc::c_int as libc::c_uint
                       != 0 ||
                   (*m_ptr).level as libc::c_int >
                       Rand_div((if (dam - 10 as libc::c_int) <
                                        1 as libc::c_int {
                                     1 as libc::c_int
                                 } else { (dam) - 10 as libc::c_int })) +
                           1 as libc::c_int + 10 as libc::c_int {
                /* No obvious effect */
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                obvious = 0 as libc::c_int as bool_;
                do_fear = 0 as libc::c_int
            }
            /* No "real" damage */
            dam = 0 as libc::c_int
        }
        67 => {
            /* Dispel undead */
            /* Only affect undead */
            if (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0 {
                /* Learn about type */
                if seen != 0 {
                    (*r_ptr).r_flags3 |= 0x20 as libc::c_int as libc::c_uint
                }
                /* Obvious */
                if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                /* Message */
                note = b" shudders.\x00" as *const u8 as *const libc::c_char;
                note_dies =
                    b" dissolves!\x00" as *const u8 as *const libc::c_char
            } else {
                /* Others ignore */
                /* Irrelevant */
                skipped = 1 as libc::c_int as bool_;
                dam = 0 as libc::c_int
            }
        }
        68 => {
            /* No damage */
            /* Dispel evil */
            /* Only affect evil */
            if (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint != 0 {
                /* Learn about type */
                if seen != 0 {
                    (*r_ptr).r_flags3 |= 0x40 as libc::c_int as libc::c_uint
                }
                /* Obvious */
                if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                /* Message */
                note = b" shudders.\x00" as *const u8 as *const libc::c_char;
                note_dies =
                    b" dissolves!\x00" as *const u8 as *const libc::c_char
            } else {
                /* Others ignore */
                /* Irrelevant */
                skipped = 1 as libc::c_int as bool_;
                dam = 0 as libc::c_int
            }
        }
        90 => {
            /* No damage */
            /* Dispel good */
            /* Only affect good */
            if (*r_ptr).flags3 & 0x200 as libc::c_int as libc::c_uint != 0 {
                /* Learn about type */
                if seen != 0 {
                    (*r_ptr).r_flags3 |= 0x200 as libc::c_int as libc::c_uint
                }
                /* Obvious */
                if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                /* Message */
                note = b" shudders.\x00" as *const u8 as *const libc::c_char;
                note_dies =
                    b" dissolves!\x00" as *const u8 as *const libc::c_char
            } else {
                /* Others ignore */
                /* Irrelevant */
                skipped = 1 as libc::c_int as bool_;
                dam = 0 as libc::c_int
            }
        }
        71 => {
            /* No damage */
            /* Dispel living */
            /* Only affect non-undead */
            if (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint == 0 &&
                   (*r_ptr).flags3 & 0x800 as libc::c_int as libc::c_uint == 0
               {
                /* Obvious */
                if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                /* Message */
                note = b" shudders.\x00" as *const u8 as *const libc::c_char;
                note_dies =
                    b" dissolves!\x00" as *const u8 as *const libc::c_char
            } else {
                /* Others ignore */
                /* Irrelevant */
                skipped = 1 as libc::c_int as bool_;
                dam = 0 as libc::c_int
            }
        }
        70 => {
            /* No damage */
            /* Dispel demons */
            /* Only affect demons */
            if (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0 {
                /* Learn about type */
                if seen != 0 {
                    (*r_ptr).r_flags3 |= 0x10 as libc::c_int as libc::c_uint
                }
                /* Obvious */
                if seen != 0 { obvious = 1 as libc::c_int as bool_ }
                /* Message */
                note = b" shudders.\x00" as *const u8 as *const libc::c_char;
                note_dies =
                    b" dissolves!\x00" as *const u8 as *const libc::c_char
            } else {
                /* Others ignore */
                /* Irrelevant */
                skipped = 1 as libc::c_int as bool_;
                dam = 0 as libc::c_int
            }
        }
        69 => {
            /* No damage */
            /* Dispel monster */
            /* Obvious */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Message */
            note = b" shudders.\x00" as *const u8 as *const libc::c_char;
            note_dies = b" dissolves!\x00" as *const u8 as *const libc::c_char
        }
        92 => {
            /* Raise Death -- Heal monster */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Wake up */
            (*m_ptr).csleep = 0 as libc::c_int as s16b;
            /* Heal */
            (*m_ptr).hp += dam;
            /* No overflow */
            if (*m_ptr).hp > (*m_ptr).maxhp { (*m_ptr).hp = (*m_ptr).maxhp }
            /* Redraw (later) if needed */
            if health_who as libc::c_int == (*c_ptr).m_idx as libc::c_int {
                (*p_ptr).redraw =
                    ((*p_ptr).redraw as libc::c_long | 0x800 as libc::c_long)
                        as u32b
            }
            /* Message */
            note =
                b" looks healthier.\x00" as *const u8 as *const libc::c_char;
            /* No "real" damage */
            dam = 0 as libc::c_int
        }
        108 => {
            /* Trap the soul of a demon and leave body */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Check race */
            if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 ||
                   (*m_ptr).mflag & 0x2 as libc::c_int != 0 ||
                   (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint == 0
               {
                /* No obvious effect */
                note =
                    b" is unaffected!\x00" as *const u8 as
                        *const libc::c_char;
                obvious = 0 as libc::c_int as bool_;
                dam = 0 as libc::c_int
            } else if dam > (*m_ptr).hp {
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
                let mut i_ptr: *mut object_type = &mut forge;
                /* Hack : drop corpse if the demon is killed by this
			 * spell */
                /* Wipe the object */
                object_prep(i_ptr,
                            lookup_kind(9 as libc::c_int, 1 as libc::c_int) as
                                libc::c_int);
                /* Unique corpses are unique */
                if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
                    object_aware(i_ptr);
                    (*i_ptr).name1 = 201 as libc::c_int as byte_hack
                }
                /* Length of decay - very long time */
                (*i_ptr).pval = 100000 as libc::c_int;
                /* Set weight */
                (*i_ptr).weight =
                    (*r_ptr).weight +
                        Rand_div((*r_ptr).weight) / 10 as libc::c_int +
                        1 as libc::c_int;
                /* Remember what we are */
                (*i_ptr).pval2 = (*m_ptr).r_idx;
                /* Give HP */
                (*i_ptr).pval3 =
                    maxroll((*r_ptr).hdice as s16b, (*r_ptr).hside as s16b);
                /* Drop it */
                drop_near(i_ptr, -(1 as libc::c_int), y, x);
            }
        }
        _ => {
            /* Default */
            /* Hooks! */
            if process_hooks_ret(59 as libc::c_int,
                                 b"dddddddddss\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 b"(s,d,d,d,d,d,d,M)\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 b"monster\x00" as *const u8 as
                                     *const libc::c_char, who, typ, dam, r, y,
                                 x, m_ptr) != 0 {
                obvious =
                    process_hooks_return[0 as libc::c_int as usize].num as
                        bool_;
                dam = process_hooks_return[1 as libc::c_int as usize].num;
                do_stun = process_hooks_return[2 as libc::c_int as usize].num;
                do_fear = process_hooks_return[3 as libc::c_int as usize].num;
                do_conf = process_hooks_return[4 as libc::c_int as usize].num;
                do_dist = process_hooks_return[5 as libc::c_int as usize].num;
                do_pois = process_hooks_return[6 as libc::c_int as usize].num;
                do_cut = process_hooks_return[7 as libc::c_int as usize].num;
                do_poly = process_hooks_return[8 as libc::c_int as usize].num;
                if !process_hooks_return[9 as libc::c_int as
                                             usize].str_0.is_null() {
                    note =
                        process_hooks_return[9 as libc::c_int as usize].str_0
                }
                if !process_hooks_return[10 as libc::c_int as
                                             usize].str_0.is_null() {
                    note_dies =
                        process_hooks_return[10 as libc::c_int as usize].str_0
                }
            } else {
                /* Irrelevant */
                skipped = 1 as libc::c_int as bool_;
                /* No damage */
                dam = 0 as libc::c_int
            }
        }
    }
    /* Absolutely no effect */
    if skipped != 0 { return 0 as libc::c_int as bool_ }
    /* "Unique" monsters cannot be polymorphed */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        do_poly = 0 as libc::c_int
    }
    /*
	 * "Quest" monsters cannot be polymorphed
	 */
    if (*m_ptr).mflag & 0x2 as libc::c_int != 0 { do_poly = 0 as libc::c_int }
    /* "Unique" monsters can only be "killed" by the player unless they are player's friends */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 &&
           (*m_ptr).status as libc::c_int <= 1 as libc::c_int {
        /* Uniques may only be killed by the player */
        if who != 0 && who != -(2 as libc::c_int) && dam > (*m_ptr).hp {
            dam = (*m_ptr).hp
        }
    }
    /*
	 * "Quest" monsters can only be "killed" by the player
	 */
    if (*m_ptr).mflag & 0x2 as libc::c_int != 0 {
        if who > 0 as libc::c_int && dam > (*m_ptr).hp { dam = (*m_ptr).hp }
    }
    if do_pois != 0 &&
           (*r_ptr).flags3 & 0x100000 as libc::c_int as libc::c_uint == 0 &&
           (*r_ptr).flags3 & 0x1000 as libc::c_int as libc::c_uint == 0 &&
           hurt_monster(m_ptr) as libc::c_int != 0 {
        if (*m_ptr).poisoned != 0 {
            note =
                b" is more poisoned.\x00" as *const u8 as *const libc::c_char
        } else {
            note = b" is poisoned.\x00" as *const u8 as *const libc::c_char
        }
        (*m_ptr).poisoned =
            ((*m_ptr).poisoned as libc::c_int + do_pois) as s16b
    }
    if do_cut != 0 &&
           (*r_ptr).flags4 & 0x4000000 as libc::c_int as libc::c_uint == 0 &&
           hurt_monster(m_ptr) as libc::c_int != 0 {
        if (*m_ptr).bleeding != 0 {
            note =
                b" bleeds more strongly.\x00" as *const u8 as
                    *const libc::c_char
        } else {
            note =
                b" starts bleeding.\x00" as *const u8 as *const libc::c_char
        }
        (*m_ptr).bleeding =
            ((*m_ptr).bleeding as libc::c_int + do_cut) as s16b
    }
    /* Check for death */
    if dam > (*m_ptr).hp && hurt_monster(m_ptr) as libc::c_int != 0 {
        /* Extract method of death */
        note = note_dies
    } else if do_poly != 0 &&
                  ((*f_info.offset((*cave[y as usize].offset(x as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x10 as libc::c_long != 0 &&
                       (*cave[y as usize].offset(x as isize)).feat as
                           libc::c_int != 0xaf as libc::c_int) &&
                  Rand_div(90 as libc::c_int) + 1 as libc::c_int >
                      (*m_ptr).level as libc::c_int {
        /* Mega-Hack -- Handle "polymorph" -- monsters get a saving throw */
        /* Default -- assume no polymorph */
        note = b" is unaffected!\x00" as *const u8 as *const libc::c_char;
        /* Handle polymorph */
        if do_poly_monster(y, x) != 0 {
            /* Obvious */
            if seen != 0 { obvious = 1 as libc::c_int as bool_ }
            /* Monster polymorphs */
            note = b" changes!\x00" as *const u8 as *const libc::c_char;
            /* Turn off the damage */
            dam = 0 as libc::c_int;
            /* Hack -- Get new monster */
            m_ptr =
                &mut *m_list.offset((*c_ptr).m_idx as isize) as
                    *mut monster_type;
            /* Hack -- Get new race */
            r_ptr =
                if !(*m_ptr).sr_ptr.is_null() {
                    (*m_ptr).sr_ptr
                } else {
                    race_info_idx((*m_ptr).r_idx as libc::c_int,
                                  (*m_ptr).ego as libc::c_int)
                }
        }
    } else if do_move != 0 && hurt_monster(m_ptr) as libc::c_int != 0 {
        let mut back: libc::c_int = 0 as libc::c_int;
        /* Handle moving the monster.
	 *
	 * Note: This is a effect of force, but only when used
	 * by the player. (For the moment). The usual stun effect
	 * is not applied.
	 */
        /* Obvious */
        if seen != 0 {
            obvious = 1 as libc::c_int as bool_
        } /* Default of no movement */
        back = 0 as libc::c_int;
        /* How far can we push the monster? */
        do_move = 1 as libc::c_int;
        while do_move < 3 as libc::c_int {
            /* Get monster coords */
			/* And offset position */
            y1 = (*m_ptr).fy as libc::c_int + b * do_move;
            x1 = (*m_ptr).fx as libc::c_int + a * do_move;
            if y1 > 0 as libc::c_int && x1 > 0 as libc::c_int &&
                   y1 < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   x1 < cur_wid as libc::c_int - 1 as libc::c_int {
                /* Require "empty" floor space */
                if !(!(y1 > 0 as libc::c_int && x1 > 0 as libc::c_int &&
                           y1 < cur_hgt as libc::c_int - 1 as libc::c_int &&
                           x1 < cur_wid as libc::c_int - 1 as libc::c_int) ||
                         !((*f_info.offset((*cave[y1 as
                                                      usize].offset(x1 as
                                                                        isize)).feat
                                               as isize)).flags1 as
                               libc::c_long & 0x10 as libc::c_long != 0 &&
                               (*cave[y1 as usize].offset(x1 as isize)).feat
                                   as libc::c_int != 0xaf as libc::c_int &&
                               (*cave[y1 as usize].offset(x1 as isize)).m_idx
                                   == 0 &&
                               !(y1 == (*p_ptr).py as libc::c_int &&
                                     x1 == (*p_ptr).px as libc::c_int))) {
                    /* Hack -- no teleport onto glyph of warding */
                    if !((*cave[y1 as usize].offset(x1 as isize)).feat as
                             libc::c_int == 0x3 as libc::c_int) {
                        /* amount moved */
                        back = do_move
                    }
                }
            }
            do_move += 1
        }
        /* Move the monster */
        if back != 0 {
            y1 = (*m_ptr).fy as libc::c_int + b * back;
            x1 = (*m_ptr).fx as libc::c_int + a * back;
            monster_swap((*m_ptr).fy as libc::c_int,
                         (*m_ptr).fx as libc::c_int, y1, x1);
            if back == 2 as libc::c_int {
                note =
                    b" is knocked back!\x00" as *const u8 as
                        *const libc::c_char
            }
            if back == 1 as libc::c_int {
                note =
                    b" is knocked back and crushed!\x00" as *const u8 as
                        *const libc::c_char;
                /* was kept from being pushed all the way, do extra dam */
                dam = dam * 13 as libc::c_int / 10 as libc::c_int
            }
            /* Get new position */
            y = y1;
            x = x1;
            /* Hack -- get new grid */
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type
        } else {
            /* could not move the monster */
            note =
                b" is severely crushed!\x00" as *const u8 as
                    *const libc::c_char;
            /* Do extra damage (1/3)*/
            dam = dam * 15 as libc::c_int / 10 as libc::c_int
        }
    } else if do_dist != 0 {
        /* Handle "teleport" */
        /* Obvious */
        if seen != 0 { obvious = 1 as libc::c_int as bool_ }
        /* Message */
        note = b" disappears!\x00" as *const u8 as *const libc::c_char;
        /* Teleport */
        teleport_away((*c_ptr).m_idx as libc::c_int, do_dist);
        /* Hack -- get new location */
        y = (*m_ptr).fy as libc::c_int;
        x = (*m_ptr).fx as libc::c_int;
        /* Hack -- get new grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type
    } else if do_stun != 0 &&
                  (*r_ptr).flags4 & 0x20000 as libc::c_int as libc::c_uint ==
                      0 &&
                  (*r_ptr).flags4 & 0x4000000 as libc::c_int as libc::c_uint
                      == 0 && hurt_monster(m_ptr) as libc::c_int != 0 {
        /* Sound and Impact breathers never stun */
        /* Obvious */
        if seen != 0 { obvious = 1 as libc::c_int as bool_ }
        /* Get confused */
        if (*m_ptr).stunned != 0 {
            note = b" is more dazed.\x00" as *const u8 as *const libc::c_char;
            tmp = (*m_ptr).stunned as libc::c_int + do_stun / 2 as libc::c_int
        } else {
            note = b" is dazed.\x00" as *const u8 as *const libc::c_char;
            tmp = do_stun
        }
        /* Apply stun */
        (*m_ptr).stunned =
            if tmp < 200 as libc::c_int { tmp } else { 200 as libc::c_int } as
                byte_hack
    } else if do_conf != 0 &&
                  (*r_ptr).flags3 & 0x40000000 as libc::c_int as libc::c_uint
                      == 0 &&
                  (*r_ptr).flags4 & 0x10000 as libc::c_int as libc::c_uint ==
                      0 &&
                  (*r_ptr).flags4 & 0x40000 as libc::c_int as libc::c_uint ==
                      0 && hurt_monster(m_ptr) as libc::c_int != 0 {
        /* Confusion and Chaos breathers (and sleepers) never confuse */
        /* Obvious */
        if seen != 0 { obvious = 1 as libc::c_int as bool_ }
        /* Already partially confused */
        if (*m_ptr).confused != 0 {
            note =
                b" looks more confused.\x00" as *const u8 as
                    *const libc::c_char;
            tmp =
                (*m_ptr).confused as libc::c_int + do_conf / 2 as libc::c_int
        } else {
            /* Was not confused */
            note =
                b" looks confused.\x00" as *const u8 as *const libc::c_char;
            tmp = do_conf
        }
        /* Apply confusion */
        (*m_ptr).confused =
            if tmp < 200 as libc::c_int { tmp } else { 200 as libc::c_int } as
                byte_hack
    }
    /* Fear */
    if do_fear != 0 && hurt_monster(m_ptr) as libc::c_int != 0 {
        /* Increase fear */
        tmp = (*m_ptr).monfear as libc::c_int + do_fear;
        /* Set fear */
        (*m_ptr).monfear =
            if tmp < 200 as libc::c_int { tmp } else { 200 as libc::c_int } as
                byte_hack
    }
    /* If another monster did the damage, hurt the monster by hand */
    if who > 0 as libc::c_int {
        let mut fear: bool_ = 0 as libc::c_int as bool_;
        /* Dead monster */
        if !(mon_take_hit_mon(who, (*c_ptr).m_idx as libc::c_int, dam,
                              &mut fear, note_dies) != 0) {
            /* Damaged monster */
            /* Give detailed messages if visible or destroyed */
            if !note.is_null() && seen as libc::c_int != 0 {
                msg_format(b"%^s%s\x00" as *const u8 as *const libc::c_char,
                           m_name.as_mut_ptr(), note);
            } else if dam > 0 as libc::c_int {
                message_pain((*c_ptr).m_idx as libc::c_int, dam);
            }
            if do_sleep != 0 { (*m_ptr).csleep = do_sleep as s16b }
        }
    } else if hurt_monster(m_ptr) != 0 {
        let mut fear_0: bool_ = 0 as libc::c_int as bool_;
        /* Hack -- Pain message */
        /* Hack -- handle sleep */
        /* If the player did it, give him experience, check fear */
        /* Hurt the monster, check for fear and death */
        if !(mon_take_hit((*c_ptr).m_idx as libc::c_int, dam, &mut fear_0,
                          note_dies) != 0) {
            /* Damaged monster */
            /* Give detailed messages if visible or destroyed */
            if !note.is_null() && seen as libc::c_int != 0 {
                msg_format(b"%^s%s\x00" as *const u8 as *const libc::c_char,
                           m_name.as_mut_ptr(), note);
            } else if dam > 0 as libc::c_int {
                message_pain((*c_ptr).m_idx as libc::c_int, dam);
            }
            if (fear_0 as libc::c_int != 0 || do_fear != 0) &&
                   (*m_ptr).ml as libc::c_int != 0 {
                /* Hack -- Pain message */
                /* Sound */
                sound(3 as libc::c_int);
                /* Message */
                msg_format(b"%^s flees in terror!\x00" as *const u8 as
                               *const libc::c_char, m_name.as_mut_ptr());
            }
            if do_sleep != 0 { (*m_ptr).csleep = do_sleep as s16b }
        }
    }
    /* Take note */
    /* Hack -- handle sleep */
    /* XXX XXX XXX Verify this code */
    /* Update the monster */
    update_mon((*c_ptr).m_idx as libc::c_int, 0 as libc::c_int as bool_);
    /* Redraw the monster grid */
    lite_spot(y, x);
    /* Update monster recall window */
    if monster_race_idx as libc::c_int == (*m_ptr).r_idx as libc::c_int {
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x100 as libc::c_long) as u32b
    }
    /* Track it */
    project_m_n += 1;
    project_m_x = x;
    project_m_y = y;
    /* Return "Anything seen?" */
    return obvious;
}
/* Is the spell unsafe for the player ? */
#[export_name = "unsafe"]
pub static mut unsafe_0: bool_ = 0 as libc::c_int as bool_;
/*
 * Helper function for "project()" below.
 *
 * Handle a beam/bolt/ball causing damage to the player.
 *
 * This routine takes a "source monster" (by index), a "distance", a default
 * "damage", and a "damage type".  See "project_m()" above.
 *
 * If "rad" is non-zero, then the blast was centered elsewhere, and the damage
 * is reduced (see "project_m()" above).  This can happen if a monster breathes
 * at the player and hits a wall instead.
 *
 * NOTE (Zangband): 'Bolt' attacks can be reflected back, so we need to know
 * if this is actually a ball or a bolt spell
 *
 *
 * We return "TRUE" if any "obvious" effects were observed.  XXX XXX Actually,
 * we just assume that the effects were obvious, for historical reasons.
 */
unsafe extern "C" fn project_p(mut who: libc::c_int, mut r: libc::c_int,
                               mut y: libc::c_int, mut x: libc::c_int,
                               mut dam: libc::c_int, mut typ: libc::c_int,
                               mut a_rad: libc::c_int) -> bool_ {
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut do_move: libc::c_int = 0 as libc::c_int;
    let mut a: libc::c_int = 0 as libc::c_int;
    let mut b: libc::c_int = 0 as libc::c_int;
    let mut x1: libc::c_int = 0 as libc::c_int;
    let mut y1: libc::c_int = 0 as libc::c_int;
    /* Hack -- assume obvious */
    let mut obvious: bool_ = 1 as libc::c_int as bool_;
    /* Player blind-ness */
    let mut blind: bool_ =
        if (*p_ptr).blind as libc::c_int != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int } as bool_;
    /* Player needs a "description" (he is blind) */
    let mut fuzzy: bool_ = 0 as libc::c_int as bool_;
    /* Source monster */
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    /* Monster name (for attacks) */
    let mut m_name: [libc::c_char; 80] = [0; 80];
    /* Monster name (for damage) */
    let mut killer: [libc::c_char; 80] = [0; 80];
    /* Hack -- messages */
    let mut act: cptr = 0 as cptr;
    /* Player is not here */
    if x != (*p_ptr).px as libc::c_int || y != (*p_ptr).py as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Player cannot hurt himself */
    if who == 0 && unsafe_0 == 0 { return 0 as libc::c_int as bool_ }
    /* Bolt attack from a monster */
    if a_rad == 0 && get_skill(46 as libc::c_int) as libc::c_int != 0 &&
           who > 0 as libc::c_int {
        let mut chance: libc::c_int =
            ((*p_ptr).dodge_chance as libc::c_int -
                 (*r_info.offset(who as isize)).level as libc::c_int *
                     5 as libc::c_int / 6 as libc::c_int) / 3 as libc::c_int;
        if chance > 0 as libc::c_int && Rand_div(100 as libc::c_int) < chance
           {
            msg_print(b"You dodge a magical attack!\x00" as *const u8 as
                          *const libc::c_char);
            return 1 as libc::c_int as bool_
        }
    }
    /* Effects done by the plane cannot bounce */
    if (*p_ptr).reflect as libc::c_int != 0 && a_rad == 0 &&
           !(Rand_div(10 as libc::c_int) + 1 as libc::c_int ==
                 1 as libc::c_int) &&
           (who != -(101 as libc::c_int) && who != -(100 as libc::c_int)) {
        let mut t_y: libc::c_int = 0;
        let mut t_x: libc::c_int = 0;
        let mut max_attempts: libc::c_int = 10 as libc::c_int;
        if blind != 0 {
            msg_print(b"Something bounces!\x00" as *const u8 as
                          *const libc::c_char);
        } else {
            msg_print(b"The attack bounces!\x00" as *const u8 as
                          *const libc::c_char);
        }
        loop 
             /* Choose 'new' target */
             {
            t_y =
                (*m_list.offset(who as isize)).fy as libc::c_int -
                    1 as libc::c_int +
                    (Rand_div(3 as libc::c_int) + 1 as libc::c_int);
            t_x =
                (*m_list.offset(who as isize)).fx as libc::c_int -
                    1 as libc::c_int +
                    (Rand_div(3 as libc::c_int) + 1 as libc::c_int);
            max_attempts -= 1;
            if !(max_attempts != 0 &&
                     (t_y >= 0 as libc::c_int && t_x >= 0 as libc::c_int &&
                          t_y < cur_hgt as libc::c_int &&
                          t_x < cur_wid as libc::c_int) &&
                     !((*cave[t_y as usize].offset(t_x as isize)).info as
                           libc::c_int & 0x20 as libc::c_int !=
                           0 as libc::c_int)) {
                break ;
            }
        }
        if max_attempts < 1 as libc::c_int {
            t_y = (*m_list.offset(who as isize)).fy as libc::c_int;
            t_x = (*m_list.offset(who as isize)).fx as libc::c_int
        }
        project(0 as libc::c_int, 0 as libc::c_int, t_y, t_x, dam, typ,
                0x8 as libc::c_int | 0x40 as libc::c_int);
        disturb(1 as libc::c_int, 0 as libc::c_int);
        return 1 as libc::c_int as bool_
    }
    /* XXX XXX XXX */
	/* Limit maximum damage */
    if dam > 1600 as libc::c_int { dam = 1600 as libc::c_int }
    /* Reduce damage by distance */
    dam = (dam + r) / (r + 1 as libc::c_int);
    /* If the player is blind, be more descriptive */
    if blind != 0 { fuzzy = 1 as libc::c_int as bool_ }
    /* If the player is hit by a trap, be more descritive */
    if who == -(2 as libc::c_int) { fuzzy = 1 as libc::c_int as bool_ }
    /* Did ``God'' do it? */
    if who == -(99 as libc::c_int) {
        if (*p_ptr).pgod != 0 {
            /* Find out the name of player's god. */
            sprintf(killer.as_mut_ptr(),
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    (*deity_info.offset((*p_ptr).pgod as isize)).name);
        } else {
            strcpy(killer.as_mut_ptr(),
                   b"Divine Wrath\x00" as *const u8 as *const libc::c_char);
        }
    }
    /* Did the dungeon do it? */
    if who == -(100 as libc::c_int) {
        sprintf(killer.as_mut_ptr(),
                b"%s\x00" as *const u8 as *const libc::c_char,
                d_name.offset((*d_info.offset(dungeon_type as isize)).name as
                                  isize));
    }
    if who == -(101 as libc::c_int) {
        sprintf(killer.as_mut_ptr(),
                b"%s\x00" as *const u8 as *const libc::c_char,
                f_name.offset((*f_info.offset((*cave[(*p_ptr).py as
                                                         usize].offset((*p_ptr).px
                                                                           as
                                                                           isize)).feat
                                                  as isize)).name as isize));
    }
    if who >= -(1 as libc::c_int) {
        /* Get the source monster */
        m_ptr = &mut *m_list.offset(who as isize) as *mut monster_type;
        /* Get the monster name */
        monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
        /* Get the monster's real name */
        monster_desc(killer.as_mut_ptr(), m_ptr, 0x88 as libc::c_int);
    }
    if who == -(2 as libc::c_int) {
        sprintf(killer.as_mut_ptr(),
                b"%s\x00" as *const u8 as *const libc::c_char,
                t_name.offset((*t_info.offset((*cave[(*p_ptr).py as
                                                         usize].offset((*p_ptr).px
                                                                           as
                                                                           isize)).t_idx
                                                  as isize)).name as
                                  libc::c_int as isize));
    }
    /* Analyze the damage */
    match typ {
        77 => {
            if fuzzy != 0 {
                msg_print(b"You are hit by pure death!\x00" as *const u8 as
                              *const libc::c_char);
            }
            take_hit(32000 as libc::c_int, killer.as_mut_ptr() as cptr);
        }
        3 => {
            /* Standard damage -- hurts inventory too */
            if fuzzy != 0 {
                msg_print(b"You are hit by acid!\x00" as *const u8 as
                              *const libc::c_char);
            }
            acid_dam(dam, killer.as_mut_ptr() as cptr);
        }
        5 => {
            /* Standard damage -- hurts inventory too */
            if fuzzy != 0 {
                msg_print(b"You are hit by fire!\x00" as *const u8 as
                              *const libc::c_char);
            }
            fire_dam(dam, killer.as_mut_ptr() as cptr);
        }
        4 => {
            /* Standard damage -- hurts inventory too */
            if fuzzy != 0 {
                msg_print(b"You are hit by cold!\x00" as *const u8 as
                              *const libc::c_char);
            }
            cold_dam(dam, killer.as_mut_ptr() as cptr);
        }
        1 => {
            /* Standard damage -- hurts inventory too */
            if fuzzy != 0 {
                msg_print(b"You are hit by lightning!\x00" as *const u8 as
                              *const libc::c_char);
            }
            elec_dam(dam, killer.as_mut_ptr() as cptr);
        }
        2 => {
            /* Standard damage -- also poisons player */
            if fuzzy != 0 {
                msg_print(b"You are hit by poison!\x00" as *const u8 as
                              *const libc::c_char);
            }
            if (*p_ptr).resist_pois != 0 {
                dam = (dam + 2 as libc::c_int) / 3 as libc::c_int
            }
            if (*p_ptr).oppose_pois != 0 {
                dam = (dam + 2 as libc::c_int) / 3 as libc::c_int
            }
            if !((*p_ptr).oppose_pois as libc::c_int != 0 ||
                     (*p_ptr).resist_pois as libc::c_int != 0) &&
                   Rand_div(32 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                do_dec_stat(4 as libc::c_int, 2 as libc::c_int);
            }
            take_hit(dam, killer.as_mut_ptr() as cptr);
            if !((*p_ptr).resist_pois as libc::c_int != 0 ||
                     (*p_ptr).oppose_pois as libc::c_int != 0) {
                set_poisoned((*p_ptr).poisoned as libc::c_int + Rand_div(dam)
                                 + 10 as libc::c_int);
            }
        }
        73 => {
            /* Standard damage -- also poisons / mutates player */
            if fuzzy != 0 {
                msg_print(b"You are hit by radiation!\x00" as *const u8 as
                              *const libc::c_char);
            }
            if (*p_ptr).resist_pois != 0 {
                dam =
                    (2 as libc::c_int * dam + 2 as libc::c_int) /
                        5 as libc::c_int
            }
            if (*p_ptr).oppose_pois != 0 {
                dam =
                    (2 as libc::c_int * dam + 2 as libc::c_int) /
                        5 as libc::c_int
            }
            take_hit(dam, killer.as_mut_ptr() as cptr);
            if !((*p_ptr).resist_pois as libc::c_int != 0 ||
                     (*p_ptr).oppose_pois as libc::c_int != 0) {
                set_poisoned((*p_ptr).poisoned as libc::c_int + Rand_div(dam)
                                 + 10 as libc::c_int);
                if Rand_div(5 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                    /* 6 */
                    msg_print(b"You undergo a freakish metamorphosis!\x00" as
                                  *const u8 as *const libc::c_char);
                    if Rand_div(4 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                        /* 4 */
                        do_poly_self();
                    } else { corrupt_player(); }
                }
                if Rand_div(6 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                    inven_damage(Some(set_acid_destroy as
                                          unsafe extern "C" fn(_:
                                                                   *mut object_type)
                                              -> libc::c_int),
                                 2 as libc::c_int);
                }
            }
        }
        10 => {
            /* Standard damage */
            if fuzzy != 0 {
                msg_print(b"You are hit by something!\x00" as *const u8 as
                              *const libc::c_char);
            }
            take_hit(dam, killer.as_mut_ptr() as cptr);
        }
        79 => {
            /* Holy Orb -- Player only takes partial damage */
            if fuzzy != 0 {
                msg_print(b"You are hit by something!\x00" as *const u8 as
                              *const libc::c_char);
            }
            take_hit(dam, killer.as_mut_ptr() as cptr);
        }
        80 => {
            if fuzzy != 0 {
                msg_print(b"You are hit by something!\x00" as *const u8 as
                              *const libc::c_char);
            }
            take_hit(dam, killer.as_mut_ptr() as cptr);
        }
        11 => {
            /* Arrow -- XXX no dodging */
            if fuzzy != 0 {
                msg_print(b"You are hit by something sharp!\x00" as *const u8
                              as *const libc::c_char);
            }
            take_hit(dam, killer.as_mut_ptr() as cptr);
        }
        12 => {
            /* Plasma -- XXX No resist */
            if fuzzy != 0 {
                msg_print(b"You are hit by something *HOT*!\x00" as *const u8
                              as *const libc::c_char);
            }
            take_hit(dam, killer.as_mut_ptr() as cptr);
            if (*p_ptr).resist_sound == 0 {
                let mut k_0: libc::c_int =
                    Rand_div((if dam > 40 as libc::c_int {
                                  35 as libc::c_int
                              } else {
                                  (dam * 3 as libc::c_int / 4 as libc::c_int)
                                      + 5 as libc::c_int
                              })) + 1 as libc::c_int;
                set_stun((*p_ptr).stun as libc::c_int + k_0);
            }
            if !((*p_ptr).resist_fire as libc::c_int != 0 ||
                     (*p_ptr).oppose_fire as libc::c_int != 0 ||
                     (*p_ptr).immune_fire as libc::c_int != 0) {
                inven_damage(Some(set_acid_destroy as
                                      unsafe extern "C" fn(_:
                                                               *mut object_type)
                                          -> libc::c_int), 3 as libc::c_int);
            }
        }
        31 => {
            /* Nether -- drain experience */
            if fuzzy != 0 {
                msg_print(b"You are hit by nether forces!\x00" as *const u8 as
                              *const libc::c_char);
            }
            if (*p_ptr).immune_neth != 0 {
                dam = 0 as libc::c_int
            } else if (*p_ptr).resist_neth != 0 {
                dam *= 6 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int
            } else if (*p_ptr).hold_life as libc::c_int != 0 &&
                          Rand_div(100 as libc::c_int) < 75 as libc::c_int {
                msg_print(b"You keep hold of your life force!\x00" as
                              *const u8 as *const libc::c_char);
            } else if (*p_ptr).hold_life != 0 {
                msg_print(b"You feel your life slipping away!\x00" as
                              *const u8 as *const libc::c_char);
                lose_exp(200 as libc::c_int +
                             (*p_ptr).exp / 1000 as libc::c_int *
                                 2 as libc::c_int);
            } else {
                msg_print(b"You feel your life draining away!\x00" as
                              *const u8 as *const libc::c_char);
                lose_exp(200 as libc::c_int +
                             (*p_ptr).exp / 100 as libc::c_int *
                                 2 as libc::c_int);
            }
            take_hit(dam, killer.as_mut_ptr() as cptr);
        }
        13 | 14 => {
            /* Water -- stun/confuse */
            if fuzzy != 0 {
                msg_print(b"You are hit by something wet!\x00" as *const u8 as
                              *const libc::c_char);
            }
            if (*p_ptr).resist_sound == 0 {
                set_stun((*p_ptr).stun as libc::c_int +
                             (Rand_div(40 as libc::c_int) +
                                  1 as libc::c_int));
            }
            if (*p_ptr).resist_conf == 0 {
                set_confused((*p_ptr).confused as libc::c_int +
                                 (Rand_div(5 as libc::c_int) +
                                      1 as libc::c_int) + 5 as libc::c_int);
            }
            if Rand_div(5 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
                inven_damage(Some(set_cold_destroy as
                                      unsafe extern "C" fn(_:
                                                               *mut object_type)
                                          -> libc::c_int), 3 as libc::c_int);
            }
            take_hit(dam, killer.as_mut_ptr() as cptr);
        }
        30 => {
            /* Chaos -- many effects */
            if fuzzy != 0 {
                msg_print(b"You are hit by a wave of anarchy!\x00" as
                              *const u8 as *const libc::c_char);
            }
            if (*p_ptr).resist_chaos != 0 {
                dam *= 6 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int
            }
            if (*p_ptr).resist_conf == 0 {
                set_confused((*p_ptr).confused as libc::c_int +
                                 Rand_div(20 as libc::c_int) +
                                 10 as libc::c_int);
            }
            if (*p_ptr).resist_chaos == 0 {
                set_image((*p_ptr).image as libc::c_int +
                              (Rand_div(10 as libc::c_int) +
                                   1 as libc::c_int));
            }
            if (*p_ptr).resist_neth == 0 && (*p_ptr).resist_chaos == 0 {
                if (*p_ptr).hold_life as libc::c_int != 0 &&
                       Rand_div(100 as libc::c_int) < 75 as libc::c_int {
                    msg_print(b"You keep hold of your life force!\x00" as
                                  *const u8 as *const libc::c_char);
                } else if (*p_ptr).hold_life != 0 {
                    msg_print(b"You feel your life slipping away!\x00" as
                                  *const u8 as *const libc::c_char);
                    lose_exp(500 as libc::c_int +
                                 (*p_ptr).exp / 1000 as libc::c_int *
                                     2 as libc::c_int);
                } else {
                    msg_print(b"You feel your life draining away!\x00" as
                                  *const u8 as *const libc::c_char);
                    lose_exp(5000 as libc::c_int +
                                 (*p_ptr).exp / 100 as libc::c_int *
                                     2 as libc::c_int);
                }
            }
            if (*p_ptr).resist_chaos == 0 ||
                   Rand_div(9 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                inven_damage(Some(set_elec_destroy as
                                      unsafe extern "C" fn(_:
                                                               *mut object_type)
                                          -> libc::c_int), 2 as libc::c_int);
                inven_damage(Some(set_fire_destroy as
                                      unsafe extern "C" fn(_:
                                                               *mut object_type)
                                          -> libc::c_int), 2 as libc::c_int);
            }
            take_hit(dam, killer.as_mut_ptr() as cptr);
        }
        20 => {
            /* Shards -- mostly cutting */
            if fuzzy != 0 {
                msg_print(b"You are hit by something sharp!\x00" as *const u8
                              as *const libc::c_char);
            }
            if (*p_ptr).resist_shard != 0 {
                dam *= 6 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int
            } else { set_cut((*p_ptr).cut as libc::c_int + dam); }
            if (*p_ptr).resist_shard == 0 ||
                   Rand_div(13 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                inven_damage(Some(set_cold_destroy as
                                      unsafe extern "C" fn(_:
                                                               *mut object_type)
                                          -> libc::c_int), 2 as libc::c_int);
            }
            take_hit(dam, killer.as_mut_ptr() as cptr);
        }
        21 => {
            /* Sound -- mostly stunning */
            if fuzzy != 0 {
                msg_print(b"You are hit by a loud noise!\x00" as *const u8 as
                              *const libc::c_char);
            }
            if (*p_ptr).resist_sound != 0 {
                dam *= 5 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int
            } else {
                let mut k_1: libc::c_int =
                    Rand_div((if dam > 90 as libc::c_int {
                                  35 as libc::c_int
                              } else {
                                  (dam / 3 as libc::c_int) + 5 as libc::c_int
                              })) + 1 as libc::c_int;
                set_stun((*p_ptr).stun as libc::c_int + k_1);
            }
            if (*p_ptr).resist_sound == 0 ||
                   Rand_div(13 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                inven_damage(Some(set_cold_destroy as
                                      unsafe extern "C" fn(_:
                                                               *mut object_type)
                                          -> libc::c_int), 2 as libc::c_int);
            }
            take_hit(dam, killer.as_mut_ptr() as cptr);
        }
        22 => {
            /* Pure confusion */
            if fuzzy != 0 {
                msg_print(b"You are hit by something puzzling!\x00" as
                              *const u8 as *const libc::c_char);
            }
            if (*p_ptr).resist_conf != 0 {
                dam *= 5 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int
            }
            if (*p_ptr).resist_conf == 0 {
                set_confused((*p_ptr).confused as libc::c_int +
                                 (Rand_div(20 as libc::c_int) +
                                      1 as libc::c_int) + 10 as libc::c_int);
            }
            take_hit(dam, killer.as_mut_ptr() as cptr);
        }
        32 => {
            /* Disenchantment -- see above */
            if fuzzy != 0 {
                msg_print(b"You are hit by something static!\x00" as *const u8
                              as *const libc::c_char);
            }
            if (*p_ptr).resist_disen != 0 {
                dam *= 6 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int
            } else { apply_disenchant(0 as libc::c_int); }
            take_hit(dam, killer.as_mut_ptr() as cptr);
        }
        33 => {
            /* Nexus -- see above */
            if fuzzy != 0 {
                msg_print(b"You are hit by something strange!\x00" as
                              *const u8 as *const libc::c_char);
            }
            if (*p_ptr).resist_nexus != 0 {
                dam *= 6 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int
            } else { apply_nexus(m_ptr); }
            take_hit(dam, killer.as_mut_ptr() as cptr);
        }
        23 => {
            /* Force -- mostly stun */
            if fuzzy != 0 {
                msg_print(b"You are hit by kinetic force!\x00" as *const u8 as
                              *const libc::c_char);
            }
            if (*p_ptr).resist_sound == 0 {
                set_stun((*p_ptr).stun as libc::c_int +
                             (Rand_div(20 as libc::c_int) +
                                  1 as libc::c_int));
                /*
				 * If fired by player, try pushing monster.
				 * First get vector from player to monster.
				 * x10 so we can use pseudo-fixed point maths.
				 *
				 * Really should use get_angle_to_grid (util.c)
				 */
                if who > 0 as libc::c_int {
                    a = 0 as libc::c_int;
                    b = 0 as libc::c_int;
                    /* Get vector from firer to target */
                    x1 =
                        ((*p_ptr).px as libc::c_int -
                             (*m_ptr).fx as libc::c_int) * 10 as libc::c_int;
                    y1 =
                        ((*p_ptr).py as libc::c_int -
                             (*m_ptr).fy as libc::c_int) * 10 as libc::c_int;
                    /* Make sure no zero divides */
                    if x1 == 0 as libc::c_int { x1 = 1 as libc::c_int }
                    if y1 == 0 as libc::c_int { y1 = 1 as libc::c_int }
                    /* Select direction player is being pushed */
                    /* Roughly horizontally */
                    if 2 as libc::c_int * y1 / x1 == 0 as libc::c_int {
                        if x1 > 0 as libc::c_int {
                            a = 1 as libc::c_int;
                            b = 0 as libc::c_int
                        } else {
                            a = -(1 as libc::c_int);
                            b = 0 as libc::c_int
                        }
                    } else if 2 as libc::c_int * x1 / y1 == 0 as libc::c_int {
                        if y1 > 0 as libc::c_int {
                            a = 0 as libc::c_int;
                            b = 1 as libc::c_int
                        } else {
                            a = 0 as libc::c_int;
                            b = -(1 as libc::c_int)
                        }
                    } else {
                        /* Roughly vertically */
                        /* Take diagonals */
                        if y1 > 0 as libc::c_int {
                            b = 1 as libc::c_int
                        } else { b = -(1 as libc::c_int) }
                        if x1 > 0 as libc::c_int {
                            a = 1 as libc::c_int
                        } else { a = -(1 as libc::c_int) }
                    }
                    /* Move monster 2 offsets back */
                    do_move = 2 as libc::c_int;
                    /* Old monster coords in x,y */
                    y1 = (*p_ptr).py as libc::c_int;
                    x1 = (*p_ptr).px as libc::c_int
                }
            } else { take_hit(dam, killer.as_mut_ptr() as cptr); }
        }
        72 => {
            /* Rocket -- stun, cut */
            if fuzzy != 0 {
                msg_print(b"There is an explosion!\x00" as *const u8 as
                              *const libc::c_char);
            }
            if (*p_ptr).resist_sound == 0 {
                set_stun((*p_ptr).stun as libc::c_int +
                             (Rand_div(20 as libc::c_int) +
                                  1 as libc::c_int));
            }
            if (*p_ptr).resist_shard != 0 {
                dam /= 2 as libc::c_int
            } else {
                set_cut((*p_ptr).cut as libc::c_int + dam / 2 as libc::c_int);
            }
            if (*p_ptr).resist_shard == 0 ||
                   Rand_div(12 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                inven_damage(Some(set_cold_destroy as
                                      unsafe extern "C" fn(_:
                                                               *mut object_type)
                                          -> libc::c_int), 3 as libc::c_int);
            }
            take_hit(dam, killer.as_mut_ptr() as cptr);
        }
        24 => {
            /* Inertia -- slowness */
            if fuzzy != 0 {
                msg_print(b"You are hit by something slow!\x00" as *const u8
                              as *const libc::c_char);
            }
            set_slow((*p_ptr).slow as libc::c_int + Rand_div(4 as libc::c_int)
                         + 4 as libc::c_int);
            take_hit(dam, killer.as_mut_ptr() as cptr);
        }
        15 => {
            /* Lite -- blinding */
            if fuzzy != 0 {
                msg_print(b"You are hit by something!\x00" as *const u8 as
                              *const libc::c_char);
            }
            if (*p_ptr).resist_lite != 0 {
                dam *= 4 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int
            } else if blind == 0 && (*p_ptr).resist_blind == 0 {
                set_blind((*p_ptr).blind as libc::c_int +
                              (Rand_div(5 as libc::c_int) + 1 as libc::c_int)
                              + 2 as libc::c_int);
            }
            if (*p_ptr).sensible_lite != 0 {
                msg_print(b"The light scorches your flesh!\x00" as *const u8
                              as *const libc::c_char);
                dam *= 3 as libc::c_int
            }
            take_hit(dam, killer.as_mut_ptr() as cptr);
            if (*p_ptr).tim_wraith != 0 {
                (*p_ptr).tim_wraith = 0 as libc::c_int as s16b;
                msg_print(b"The light forces you out of your incorporeal shadow form.\x00"
                              as *const u8 as *const libc::c_char);
                (*p_ptr).redraw =
                    ((*p_ptr).redraw as libc::c_long |
                         0x4000000 as libc::c_long) as u32b;
                /* Update monsters */
                (*p_ptr).update =
                    ((*p_ptr).update as libc::c_long |
                         0x1000000 as libc::c_long) as u32b;
                /* Window stuff */
                (*p_ptr).window =
                    ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long)
                        as u32b
            }
        }
        16 => {
            /* Dark -- blinding */
            if fuzzy != 0 {
                msg_print(b"You are hit by something!\x00" as *const u8 as
                              *const libc::c_char);
            }
            if (*p_ptr).resist_dark != 0 {
                dam *= 4 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int
            } else if blind == 0 && (*p_ptr).resist_blind == 0 {
                set_blind((*p_ptr).blind as libc::c_int +
                              (Rand_div(5 as libc::c_int) + 1 as libc::c_int)
                              + 2 as libc::c_int);
            }
            if (*p_ptr).wraith_form != 0 {
                hp_player(dam);
            } else { take_hit(dam, killer.as_mut_ptr() as cptr); }
        }
        34 => {
            /* Time -- bolt fewer effects XXX */
            if fuzzy != 0 {
                msg_print(b"You are hit by a blast from the past!\x00" as
                              *const u8 as
                              *const libc::c_char); /* No teleport on special levels */
            }
            if (*p_ptr).resist_continuum != 0 {
                dam *= 4 as libc::c_int;
                dam /=
                    Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                        6 as libc::c_int;
                msg_print(b"You feel as if time is passing you by.\x00" as
                              *const u8 as *const libc::c_char);
            } else {
                match Rand_div(10 as libc::c_int) + 1 as libc::c_int {
                    1 | 2 | 3 | 4 | 5 => {
                        msg_print(b"You feel life has clocked back.\x00" as
                                      *const u8 as *const libc::c_char);
                        lose_exp(100 as libc::c_int +
                                     (*p_ptr).exp / 100 as libc::c_int *
                                         2 as libc::c_int);
                    }
                    6 | 7 | 8 | 9 => {
                        match Rand_div(6 as libc::c_int) + 1 as libc::c_int {
                            1 => {
                                k = 0 as libc::c_int;
                                act =
                                    b"strong\x00" as *const u8 as
                                        *const libc::c_char
                            }
                            2 => {
                                k = 1 as libc::c_int;
                                act =
                                    b"bright\x00" as *const u8 as
                                        *const libc::c_char
                            }
                            3 => {
                                k = 2 as libc::c_int;
                                act =
                                    b"wise\x00" as *const u8 as
                                        *const libc::c_char
                            }
                            4 => {
                                k = 3 as libc::c_int;
                                act =
                                    b"agile\x00" as *const u8 as
                                        *const libc::c_char
                            }
                            5 => {
                                k = 4 as libc::c_int;
                                act =
                                    b"hardy\x00" as *const u8 as
                                        *const libc::c_char
                            }
                            6 => {
                                k = 5 as libc::c_int;
                                act =
                                    b"beautiful\x00" as *const u8 as
                                        *const libc::c_char
                            }
                            _ => { }
                        }
                        msg_format(b"You\'re not as %s as you used to be...\x00"
                                       as *const u8 as *const libc::c_char,
                                   act);
                        (*p_ptr).stat_cur[k as usize] =
                            ((*p_ptr).stat_cur[k as usize] as libc::c_int *
                                 3 as libc::c_int / 4 as libc::c_int) as s16b;
                        if ((*p_ptr).stat_cur[k as usize] as libc::c_int) <
                               3 as libc::c_int {
                            (*p_ptr).stat_cur[k as usize] =
                                3 as libc::c_int as s16b
                        }
                        (*p_ptr).update =
                            ((*p_ptr).update as libc::c_long |
                                 0x1 as libc::c_long) as u32b
                    }
                    10 => {
                        msg_print(b"You\'re not as powerful as you used to be...\x00"
                                      as *const u8 as *const libc::c_char);
                        k = 0 as libc::c_int;
                        while k < 6 as libc::c_int {
                            (*p_ptr).stat_cur[k as usize] =
                                ((*p_ptr).stat_cur[k as usize] as libc::c_int
                                     * 3 as libc::c_int / 4 as libc::c_int) as
                                    s16b;
                            if ((*p_ptr).stat_cur[k as usize] as libc::c_int)
                                   < 3 as libc::c_int {
                                (*p_ptr).stat_cur[k as usize] =
                                    3 as libc::c_int as s16b
                            }
                            k += 1
                        }
                        (*p_ptr).update =
                            ((*p_ptr).update as libc::c_long |
                                 0x1 as libc::c_long) as u32b
                    }
                    _ => { }
                }
            }
            take_hit(dam, killer.as_mut_ptr() as cptr);
        }
        35 => {
            if !(dungeon_flags2 as libc::c_long & 0x8 as libc::c_long != 0) {
                if fuzzy != 0 {
                    msg_print(b"You are hit by something heavy!\x00" as
                                  *const u8 as *const libc::c_char);
                }
                msg_print(b"Gravity warps around you.\x00" as *const u8 as
                              *const libc::c_char);
                if unsafe_0 == 0 {
                    teleport_player(5 as libc::c_int);
                    if (*p_ptr).ffall == 0 {
                        set_slow((*p_ptr).slow as libc::c_int +
                                     Rand_div(4 as libc::c_int) +
                                     4 as libc::c_int);
                    }
                    if !((*p_ptr).resist_sound as libc::c_int != 0 ||
                             (*p_ptr).ffall as libc::c_int != 0) {
                        let mut k_2: libc::c_int =
                            Rand_div((if dam > 90 as libc::c_int {
                                          35 as libc::c_int
                                      } else {
                                          (dam / 3 as libc::c_int) +
                                              5 as libc::c_int
                                      })) + 1 as libc::c_int;
                        set_stun((*p_ptr).stun as libc::c_int + k_2);
                    }
                    if (*p_ptr).ffall != 0 {
                        dam = dam * 2 as libc::c_int / 3 as libc::c_int
                    }
                    if (*p_ptr).ffall == 0 ||
                           Rand_div(13 as libc::c_int) + 1 as libc::c_int ==
                               1 as libc::c_int {
                        inven_damage(Some(set_cold_destroy as
                                              unsafe extern "C" fn(_:
                                                                       *mut object_type)
                                                  -> libc::c_int),
                                     2 as libc::c_int);
                    }
                    take_hit(dam, killer.as_mut_ptr() as cptr);
                } else { teleport_player(dam); }
            }
        }
        81 => {
            /* Standard damage */
            if fuzzy != 0 {
                msg_print(b"You are hit by pure energy!\x00" as *const u8 as
                              *const libc::c_char);
            }
            take_hit(dam, killer.as_mut_ptr() as cptr);
        }
        53 => {
            if fuzzy != 0 {
                msg_print(b"You are hit by something invigorating!\x00" as
                              *const u8 as *const libc::c_char);
            }
            hp_player(dam);
            dam = 0 as libc::c_int
        }
        54 => {
            if fuzzy != 0 {
                msg_print(b"You are hit by something!\x00" as *const u8 as
                              *const libc::c_char);
            }
            set_fast((*p_ptr).fast as libc::c_int +
                         (Rand_div(5 as libc::c_int) + 1 as libc::c_int),
                     10 as libc::c_int);
            dam = 0 as libc::c_int
        }
        55 => {
            if fuzzy != 0 {
                msg_print(b"You are hit by something slow!\x00" as *const u8
                              as *const libc::c_char);
            }
            set_slow((*p_ptr).slow as libc::c_int + Rand_div(4 as libc::c_int)
                         + 4 as libc::c_int);
        }
        57 => {
            if !((*p_ptr).free_act != 0) {
                if fuzzy != 0 {
                    msg_print(b"You fall asleep!\x00" as *const u8 as
                                  *const libc::c_char);
                }
                set_paralyzed((*p_ptr).paralyzed as libc::c_int + dam);
                dam = 0 as libc::c_int
            }
        }
        26 => {
            /* Pure damage */
            if fuzzy != 0 {
                msg_print(b"You are hit by an aura of magic!\x00" as *const u8
                              as *const libc::c_char);
            }
            take_hit(dam, killer.as_mut_ptr() as cptr);
        }
        27 => {
            /* Pure damage */
            if fuzzy != 0 {
                msg_print(b"Something falls from the sky on you!\x00" as
                              *const u8 as *const libc::c_char);
            }
            take_hit(dam, killer.as_mut_ptr() as cptr);
            if (*p_ptr).resist_shard == 0 ||
                   Rand_div(13 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                if (*p_ptr).immune_fire == 0 {
                    inven_damage(Some(set_fire_destroy as
                                          unsafe extern "C" fn(_:
                                                                   *mut object_type)
                                              -> libc::c_int),
                                 2 as libc::c_int);
                }
                inven_damage(Some(set_cold_destroy as
                                      unsafe extern "C" fn(_:
                                                               *mut object_type)
                                          -> libc::c_int), 2 as libc::c_int);
            }
        }
        28 => {
            /* Ice -- cold plus stun plus cuts */
            if fuzzy != 0 {
                msg_print(b"You are hit by something sharp and cold!\x00" as
                              *const u8 as *const libc::c_char);
            }
            cold_dam(dam, killer.as_mut_ptr() as cptr);
            if (*p_ptr).resist_shard == 0 {
                set_cut((*p_ptr).cut as libc::c_int +
                            damroll(5 as libc::c_int as s16b,
                                    8 as libc::c_int as s16b));
            }
            if (*p_ptr).resist_sound == 0 {
                set_stun((*p_ptr).stun as libc::c_int +
                             (Rand_div(15 as libc::c_int) +
                                  1 as libc::c_int));
            }
            if !((*p_ptr).resist_cold as libc::c_int != 0 ||
                     (*p_ptr).oppose_cold as libc::c_int != 0) ||
                   Rand_div(12 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                if (*p_ptr).immune_cold == 0 {
                    inven_damage(Some(set_cold_destroy as
                                          unsafe extern "C" fn(_:
                                                                   *mut object_type)
                                              -> libc::c_int),
                                 3 as libc::c_int);
                }
            }
        }
        91 => {
            /* Knowledge */
            if fuzzy != 0 {
                msg_print(b"You are hit by pure knowledge!\x00" as *const u8
                              as *const libc::c_char);
            }
            self_knowledge(0 as *mut FILE);
        }
        85 => {
            /* Psi -- ESP */
            if fuzzy != 0 {
                msg_print(b"You are hit by pure psionic energy!\x00" as
                              *const u8 as *const libc::c_char);
            }
            set_tim_esp((*p_ptr).tim_esp as libc::c_int + dam);
        }
        75 => {
            /* Statis -- paralyse */
            if fuzzy != 0 {
                msg_print(b"You are hit by something paralyzing!\x00" as
                              *const u8 as *const libc::c_char);
            }
            set_paralyzed((*p_ptr).paralyzed as libc::c_int + dam);
        }
        92 => {
            /* Raise Death -- restore life */
            if fuzzy != 0 {
                msg_print(b"You are hit by pure anti-death energy!\x00" as
                              *const u8 as *const libc::c_char);
            }
            restore_level();
        }
        74 => {
            /* Make Glyph -- Shield */
            if fuzzy != 0 {
                msg_print(b"You are hit by pure protection!\x00" as *const u8
                              as *const libc::c_char);
            }
            set_shield((*p_ptr).shield as libc::c_int + dam,
                       50 as libc::c_int, 0 as libc::c_int as s16b,
                       0 as libc::c_int as s16b, 0 as libc::c_int as s16b);
        }
        _ => {
            /* Default */
            /* Hooks! */
            if process_hooks_ret(59 as libc::c_int,
                                 b"dd\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char,
                                 b"(s,d,d,d,d,d,d)\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 b"player\x00" as *const u8 as
                                     *const libc::c_char, who, typ, dam, r, y,
                                 x) != 0 {
                obvious =
                    process_hooks_return[0 as libc::c_int as usize].num as
                        bool_;
                dam = process_hooks_return[1 as libc::c_int as usize].num
            } else {
                /* No damage */
                dam = 0 as libc::c_int
            }
        }
    }
    /* Handle moving the player.
	 *
	 * Note: This is a effect of force
	 */
    if do_move != 0 {
        let mut back: libc::c_int =
            0 as libc::c_int; /* Default of no movement */
        back = 0 as libc::c_int;
        /* How far can we push the monster? */
        do_move = 1 as libc::c_int;
        while do_move < 3 as libc::c_int {
            /* Get monster coords */
			/* And offset position */
            y1 = (*p_ptr).py as libc::c_int + b * do_move;
            x1 = (*p_ptr).px as libc::c_int + a * do_move;
            if y1 > 0 as libc::c_int && x1 > 0 as libc::c_int &&
                   y1 < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   x1 < cur_wid as libc::c_int - 1 as libc::c_int {
                /* Require "empty" floor space */
                if !(!(y1 > 0 as libc::c_int && x1 > 0 as libc::c_int &&
                           y1 < cur_hgt as libc::c_int - 1 as libc::c_int &&
                           x1 < cur_wid as libc::c_int - 1 as libc::c_int) ||
                         !((*f_info.offset((*cave[y1 as
                                                      usize].offset(x1 as
                                                                        isize)).feat
                                               as isize)).flags1 as
                               libc::c_long & 0x10 as libc::c_long != 0 &&
                               (*cave[y1 as usize].offset(x1 as isize)).feat
                                   as libc::c_int != 0xaf as libc::c_int &&
                               (*cave[y1 as usize].offset(x1 as isize)).m_idx
                                   == 0 &&
                               !(y1 == (*p_ptr).py as libc::c_int &&
                                     x1 == (*p_ptr).px as libc::c_int))) {
                    /* amount moved */
                    back = do_move
                }
            }
            do_move += 1
        }
        /* Move the monster */
        if back != 0 {
            y1 = (*p_ptr).py as libc::c_int + b * back;
            x1 = (*p_ptr).px as libc::c_int + a * back;
            swap_position(y1, x1);
            if back == 2 as libc::c_int {
                msg_print(b"You are knocked back!\x00" as *const u8 as
                              *const libc::c_char);
            }
            if back == 1 as libc::c_int {
                msg_print(b"You are knocked back and crushed!\x00" as
                              *const u8 as *const libc::c_char);
                /* was kept from being pushed all the way, do extra dam */
                dam = dam * 13 as libc::c_int / 10 as libc::c_int
            }
            /* Get new position */
            y = y1;
            x = x1
        } else {
            /* could not move the monster */
            msg_print(b"You are severely crushed!\x00" as *const u8 as
                          *const libc::c_char);
            /* Do extra damage (1/3)*/
            dam = dam * 15 as libc::c_int / 10 as libc::c_int
        }
        take_hit(dam, killer.as_mut_ptr() as cptr);
    }
    /* Disturb */
    disturb(1 as libc::c_int, 0 as libc::c_int);
    /* Return "Anything seen?" */
    return obvious;
}
/*
 * Generic "beam"/"bolt"/"ball" projection routine.
 *
 * Input:
 *   who: Index of "source" monster (negative for "player")
 *        jk -- -2 for traps, only used with project_jump
 *   rad: Radius of explosion (0 = beam/bolt, 1 to 9 = ball)
 *   y,x: Target location (or location to travel "towards")
 *   dam: Base damage roll to apply to affected monsters (or player)
 *   typ: Type of damage to apply to monsters (and objects)
 *   flg: Extra bit flags (see PROJECT_xxxx in "defines.h")
 *
 * Return:
 *   TRUE if any "effects" of the projection were observed, else FALSE
 *
 * Allows a monster (or player) to project a beam/bolt/ball of a given kind
 * towards a given location (optionally passing over the heads of interposing
 * monsters), and have it do a given amount of damage to the monsters (and
 * optionally objects) within the given radius of the final location.
 *
 * A "bolt" travels from source to target and affects only the target grid.
 * A "beam" travels from source to target, affecting all grids passed through.
 * A "ball" travels from source to the target, exploding at the target, and
 *   affecting everything within the given radius of the target location.
 *
 * Traditionally, a "bolt" does not affect anything on the ground, and does
 * not pass over the heads of interposing monsters, much like a traditional
 * missile, and will "stop" abruptly at the "target" even if no monster is
 * positioned there, while a "ball", on the other hand, passes over the heads
 * of monsters between the source and target, and affects everything except
 * the source monster which lies within the final radius, while a "beam"
 * affects every monster between the source and target, except for the casting
 * monster (or player), and rarely affects things on the ground.
 *
 * Two special flags allow us to use this function in special ways, the
 * "PROJECT_HIDE" flag allows us to perform "invisible" projections, while
 * the "PROJECT_JUMP" flag allows us to affect a specific grid, without
 * actually projecting from the source monster (or player).
 *
 * The player will only get "experience" for monsters killed by himself
 * Unique monsters can only be destroyed by attacks from the player
 *
 * Only 256 grids can be affected per projection, limiting the effective
 * "radius" of standard ball attacks to nine units (diameter nineteen).
 *
 * One can project in a given "direction" by combining PROJECT_THRU with small
 * offsets to the initial location (see "line_spell()"), or by calculating
 * "virtual targets" far away from the player.
 *
 * One can also use PROJECT_THRU to send a beam/bolt along an angled path,
 * continuing until it actually hits something (useful for "stone to mud").
 *
 * Bolts and Beams explode INSIDE walls, so that they can destroy doors.
 *
 * Balls must explode BEFORE hitting walls, or they would affect monsters
 * on both sides of a wall.  Some bug reports indicate that this is still
 * happening in 2.7.8 for Windows, though it appears to be impossible.
 *
 * We "pre-calculate" the blast area only in part for efficiency.
 * More importantly, this lets us do "explosions" from the "inside" out.
 * This results in a more logical distribution of "blast" treasure.
 * It also produces a better (in my opinion) animation of the explosion.
 * It could be (but is not) used to have the treasure dropped by monsters
 * in the middle of the explosion fall "outwards", and then be damaged by
 * the blast as it spreads outwards towards the treasure drop location.
 *
 * Walls and doors are included in the blast area, so that they can be
 * "burned" or "melted" in later versions.
 *
 * This algorithm is intended to maximize simplicity, not necessarily
 * efficiency, since this function is not a bottleneck in the code.
 *
 * We apply the blast effect from ground zero outwards, in several passes,
 * first affecting features, then objects, then monsters, then the player.
 * This allows walls to be removed before checking the object or monster
 * in the wall, and protects objects which are dropped by monsters killed
 * in the blast, and allows the player to see all affects before he is
 * killed or teleported away.  The semantics of this method are open to
 * various interpretations, but they seem to work well in practice.
 *
 * We process the blast area from ground-zero outwards to allow for better
 * distribution of treasure dropped by monsters, and because it provides a
 * pleasing visual effect at low cost.
 *
 * Note that the damage done by "ball" explosions decreases with distance.
 * This decrease is rapid, grids at radius "dist" take "1/dist" damage.
 *
 * Notice the "napalm" effect of "beam" weapons.  First they "project" to
 * the target, and then the damage "flows" along this beam of destruction.
 * The damage at every grid is the same as at the "center" of a "ball"
 * explosion, since the "beam" grids are treated as if they ARE at the
 * center of a "ball" explosion.
 *
 * Currently, specifying "beam" plus "ball" means that locations which are
 * covered by the initial "beam", and also covered by the final "ball", except
 * for the final grid (the epicenter of the ball), will be "hit twice", once
 * by the initial beam, and once by the exploding ball.  For the grid right
 * next to the epicenter, this results in 150% damage being done.  The center
 * does not have this problem, for the same reason the final grid in a "beam"
 * plus "bolt" does not -- it is explicitly removed.  Simply removing "beam"
 * grids which are covered by the "ball" will NOT work, as then they will
 * receive LESS damage than they should.  Do not combine "beam" with "ball".
 *
 * The array "gy[],gx[]" with current size "grids" is used to hold the
 * collected locations of all grids in the "blast area" plus "beam path".
 *
 * Note the rather complex usage of the "gm[]" array.  First, gm[0] is always
 * zero.  Second, for N>1, gm[N] is always the index (in gy[],gx[]) of the
 * first blast grid (see above) with radius "N" from the blast center.  Note
 * that only the first gm[1] grids in the blast area thus take full damage.
 * Also, note that gm[rad+1] is always equal to "grids", which is the total
 * number of blast grids.
 *
 * Note that once the projection is complete, (y2,x2) holds the final location
 * of bolts/beams, and the "epicenter" of balls.
 *
 * Note also that "rad" specifies the "inclusive" radius of projection blast,
 * so that a "rad" of "one" actually covers 5 or 9 grids, depending on the
 * implementation of the "distance" function.  Also, a bolt can be properly
 * viewed as a "ball" with a "rad" of "zero".
 *
 * Note that if no "target" is reached before the beam/bolt/ball travels the
 * maximum distance allowed (MAX_RANGE), no "blast" will be induced.  This
 * may be relevant even for bolts, since they have a "1x1" mini-blast.
 *
 * Note that for consistency, we "pretend" that the bolt actually takes "time"
 * to move from point A to point B, even if the player cannot see part of the
 * projection path.  Note that in general, the player will *always* see part
 * of the path, since it either starts at the player or ends on the player.
 *
 * Hack -- we assume that every "projection" is "self-illuminating".
 *
 * Hack -- when only a single monster is affected, we automatically track
 * (and recall) that monster, unless "PROJECT_JUMP" is used.
 *
 * Note that all projections now "explode" at their final destination, even
 * if they were being projected at a more distant destination.  This means
 * that "ball" spells will *always* explode.
 *
 * Note that we must call "handle_stuff()" after affecting terrain features
 * in the blast radius, in case the "illumination" of the grid was changed,
 * and "update_view()" and "update_monsters()" need to be called.
 */
#[no_mangle]
pub unsafe extern "C" fn project(mut who: libc::c_int, mut rad: libc::c_int,
                                 mut y: libc::c_int, mut x: libc::c_int,
                                 mut dam: libc::c_int, mut typ: libc::c_int,
                                 mut flg: libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0; /* For reflecting monsters */
    let mut t: libc::c_int = 0;
    let mut dist: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut dist_hack: libc::c_int = 0 as libc::c_int;
    let mut y_saver: libc::c_int = 0;
    let mut x_saver: libc::c_int = 0;
    let mut msec: libc::c_int =
        delay_factor as libc::c_int * delay_factor as libc::c_int *
            delay_factor as libc::c_int;
    /* Assume the player sees nothing */
    let mut notice: bool_ = 0 as libc::c_int as bool_;
    /* Assume the player has seen nothing */
    let mut visual: bool_ = 0 as libc::c_int as bool_;
    /* Assume the player has seen no blast grids */
    let mut drawn: bool_ = 0 as libc::c_int as bool_;
    /* Is the player blind? */
    let mut blind: bool_ =
        if (*p_ptr).blind as libc::c_int != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int } as bool_;
    /* Number of grids in the "path" */
    let mut path_n: libc::c_int = 0 as libc::c_int;
    /* Actual grids in the "path" */
    let mut path_g: [u16b; 1024] = [0; 1024];
    /* Number of grids in the "blast area" (including the "beam" path) */
    let mut grids: libc::c_int = 0 as libc::c_int;
    let mut effect: libc::c_int = 0 as libc::c_int;
    /* Coordinates of the affected grids */
    let mut gx: [byte_hack; 1024] = [0; 1024];
    let mut gy: [byte_hack; 1024] = [0; 1024];
    /* Encoded "radius" info (see above) */
    let mut gm: [byte_hack; 64] = [0; 64];
    /* Hack -- Jump to target */
    if flg & 0x1 as libc::c_int != 0 {
        x1 = x;
        y1 = y;
        /* Clear the flag */
        flg &= !(0x1 as libc::c_int)
    } else if who <= 0 as libc::c_int &&
                  (who != -(101 as libc::c_int) &&
                       who != -(100 as libc::c_int)) {
        x1 = (*p_ptr).px as libc::c_int;
        y1 = (*p_ptr).py as libc::c_int
    } else if who > 0 as libc::c_int {
        x1 = (*m_list.offset(who as isize)).fx as libc::c_int;
        y1 = (*m_list.offset(who as isize)).fy as libc::c_int
    } else {
        /* Start at player */
        /* Start at monster */
        /* Oops */
        x1 = x;
        y1 = y
    }
    y_saver = y1;
    x_saver = x1;
    /* Default "destination" */
    y2 = y;
    x2 = x;
    /* Hack -- verify stuff */
    if flg & 0x4 as libc::c_int != 0 {
        if x1 == x2 && y1 == y2 { flg &= !(0x4 as libc::c_int) }
    }
    /* Hack -- Assume there will be no blast (max radius 16) */
    dist = 0 as libc::c_int;
    while dist < 64 as libc::c_int {
        gm[dist as usize] = 0 as libc::c_int as byte_hack;
        dist += 1
    }
    /* Initial grid */
    y = y1;
    x = x1;
    dist = 0 as libc::c_int;
    /* Collect beam grids */
    if flg & 0x2 as libc::c_int != 0 {
        gy[grids as usize] = y as byte_hack;
        gx[grids as usize] = x as byte_hack;
        grids += 1
    }
    /* Calculate the projection path */
    if who == -(101 as libc::c_int) || who == -(100 as libc::c_int) {
        path_n = 0 as libc::c_int
    } else {
        path_n =
            project_path(path_g.as_mut_ptr(), 18 as libc::c_int, y1, x1, y2,
                         x2, flg)
    }
    /* Hack -- Handle stuff */
    handle_stuff();
    /* Project along the path */
    i = 0 as libc::c_int;
    while i < path_n {
        let mut oy: libc::c_int = y;
        let mut ox: libc::c_int = x;
        let mut ny: libc::c_int =
            (path_g[i as usize] as
                 libc::c_uint).wrapping_div(256 as libc::c_uint) as
                libc::c_int;
        let mut nx: libc::c_int =
            (path_g[i as usize] as
                 libc::c_uint).wrapping_rem(256 as libc::c_uint) as
                libc::c_int;
        /* Hack -- Balls explode before reaching walls */
        if !((*f_info.offset((*cave[ny as usize].offset(nx as isize)).feat as
                                 isize)).flags1 as libc::c_long &
                 0x10 as libc::c_long != 0 &&
                 (*cave[ny as usize].offset(nx as isize)).feat as libc::c_int
                     != 0xaf as libc::c_int) && rad > 0 as libc::c_int {
            break ;
        }
        /* Advance */
        y = ny;
        x = nx;
        /* Collect beam grids */
        if flg & 0x2 as libc::c_int != 0 {
            gy[grids as usize] = y as byte_hack;
            gx[grids as usize] = x as byte_hack;
            grids += 1
        }
        /* Only do visuals if requested */
        if blind == 0 && flg & 0x80 as libc::c_int == 0 {
            /* Only do visuals if the player can "see" the bolt */
            if y >= panel_row_min as libc::c_int &&
                   y <= panel_row_max as libc::c_int &&
                   x >= panel_col_min as libc::c_int &&
                   x <= panel_col_max as libc::c_int &&
                   (*cave[y as usize].offset(x as isize)).info as libc::c_int
                       & 0x20 as libc::c_int != 0 as libc::c_int {
                let mut p: u16b = 0;
                let mut a: byte_hack = 0;
                let mut c: libc::c_char = 0;
                /* Obtain the bolt pict */
                p = bolt_pict(oy, ox, y, x, typ);
                /* Extract attr/char */
                a = (p as libc::c_int >> 8 as libc::c_int) as byte_hack;
                c = p as byte_hack as libc::c_char;
                /* Visual effects */
                print_rel(c, a, y, x);
                move_cursor_relative(y, x);
                if fresh_before != 0 { Term_fresh(); }
                Term_xtra(13 as libc::c_int, msec);
                lite_spot(y, x);
                if fresh_before != 0 { Term_fresh(); }
                /* Display "beam" grids */
                if flg & 0x2 as libc::c_int != 0 {
                    /* Obtain the explosion pict */
                    p = bolt_pict(y, x, y, x, typ);
                    /* Extract attr/char */
                    a = (p as libc::c_int >> 8 as libc::c_int) as byte_hack;
                    c = p as byte_hack as libc::c_char;
                    /* Visual effects */
                    print_rel(c, a, y, x);
                }
                /* Hack -- Activate delay */
                visual = 1 as libc::c_int as bool_
            } else if visual != 0 {
                /* Hack -- delay anyway for consistency */
                /* Delay for consistency */
                Term_xtra(13 as libc::c_int, msec);
            }
        }
        i += 1
    }
    /* Save the "blast epicenter" */
    y2 = y;
    x2 = x;
    /* Start the "explosion" */
    gm[0 as libc::c_int as usize] = 0 as libc::c_int as byte_hack;
    /* Hack -- make sure beams get to "explode" */
    gm[1 as libc::c_int as usize] = grids as byte_hack;
    dist_hack = dist;
    /* Explode */
    /* Hack -- remove final beam grid */
    if flg & 0x2 as libc::c_int != 0 { grids -= 1 }
    dist = 0 as libc::c_int;
    while dist <= rad {
        /* Determine the blast area, work from the inside out */
        /* Scan the maximal blast area of radius "dist" */
        y = y2 - dist;
        while y <= y2 + dist {
            let mut current_block_91: u64;
            x = x2 - dist;
            while x <= x2 + dist {
                /* Ignore "illegal" locations */
                if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                       y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                       x < cur_wid as libc::c_int - 1 as libc::c_int {
                    /* Enforce a "circular" explosion */
                    if !(distance(y2, x2, y, x) != dist) {
                        /* Ball explosions are stopped by walls */
                        if typ == 81 as libc::c_int {
                            if cave_valid_bold(y, x) as libc::c_int != 0 &&
                                   (((*cave[y as
                                                usize].offset(x as
                                                                  isize)).feat
                                         as libc::c_int) < 0x41 as libc::c_int
                                        ||
                                        (*cave[y as
                                                   usize].offset(x as
                                                                     isize)).feat
                                            as libc::c_int >
                                            0x49 as libc::c_int) {
                                cave_set_feat(y, x, 0x1 as libc::c_int);
                            }
                            /* Update some things -- similar to GF_KILL_WALL */
                            (*p_ptr).update =
                                ((*p_ptr).update as libc::c_long |
                                     (0x100000 as libc::c_long |
                                          0x10000000 as libc::c_long |
                                          0x1000000 as libc::c_long |
                                          0x200000 as libc::c_long)) as u32b;
                            current_block_91 = 562309032768341766;
                        } else if los(y2, x2, y, x) == 0 {
                            current_block_91 = 5023038348526654800;
                        } else { current_block_91 = 562309032768341766; }
                        match current_block_91 {
                            5023038348526654800 => { }
                            _ => {
                                /* Save this grid */
                                gy[grids as usize] = y as byte_hack;
                                gx[grids as usize] = x as byte_hack;
                                grids += 1
                            }
                        }
                    }
                }
                x += 1
            }
            y += 1
        }
        /* Encode some more "radius" info */
        gm[(dist + 1 as libc::c_int) as usize] = grids as byte_hack;
        dist += 1
    }
    /* Speed -- ignore "non-explosions" */
    if grids == 0 { return 0 as libc::c_int as bool_ }
    /* Display the "blast area" if requested */
    if blind == 0 && flg & 0x80 as libc::c_int == 0 {
        /* Then do the "blast", from inside out */
        t = 0 as libc::c_int;
        while t <= rad {
            /* Dump everything with this radius */
            i = gm[t as usize] as libc::c_int;
            while i < gm[(t + 1 as libc::c_int) as usize] as libc::c_int {
                /* Extract the location */
                y = gy[i as usize] as libc::c_int;
                x = gx[i as usize] as libc::c_int;
                /* Only do visuals if the player can "see" the blast */
                if y >= panel_row_min as libc::c_int &&
                       y <= panel_row_max as libc::c_int &&
                       x >= panel_col_min as libc::c_int &&
                       x <= panel_col_max as libc::c_int &&
                       (*cave[y as usize].offset(x as isize)).info as
                           libc::c_int & 0x20 as libc::c_int !=
                           0 as libc::c_int {
                    let mut p_0: u16b = 0;
                    let mut a_0: byte_hack = 0;
                    let mut c_0: libc::c_char = 0;
                    drawn = 1 as libc::c_int as bool_;
                    /* Obtain the explosion pict */
                    p_0 = bolt_pict(y, x, y, x, typ);
                    /* Extract attr/char */
                    a_0 =
                        (p_0 as libc::c_int >> 8 as libc::c_int) as byte_hack;
                    c_0 = p_0 as byte_hack as libc::c_char;
                    /* Visual effects -- Display */
                    print_rel(c_0, a_0, y, x);
                }
                i += 1
            }
            /* Hack -- center the cursor */
            move_cursor_relative(y2, x2);
            /* Flush each "radius" seperately */
            if fresh_before != 0 { Term_fresh(); }
            /* Delay (efficiently) */
            if visual as libc::c_int != 0 || drawn as libc::c_int != 0 {
                Term_xtra(13 as libc::c_int, msec);
            }
            t += 1
        }
        /* Flush the erasing */
        if drawn != 0 {
            /* Erase the explosion drawn above */
            i = 0 as libc::c_int;
            while i < grids {
                /* Extract the location */
                y = gy[i as usize] as libc::c_int;
                x = gx[i as usize] as libc::c_int;
                /* Hack -- Erase if needed */
                if y >= panel_row_min as libc::c_int &&
                       y <= panel_row_max as libc::c_int &&
                       x >= panel_col_min as libc::c_int &&
                       x <= panel_col_max as libc::c_int &&
                       (*cave[y as usize].offset(x as isize)).info as
                           libc::c_int & 0x20 as libc::c_int !=
                           0 as libc::c_int {
                    lite_spot(y, x);
                }
                i += 1
            }
            /* Hack -- center the cursor */
            move_cursor_relative(y2, x2);
            /* Flush the explosion */
            if fresh_before != 0 { Term_fresh(); }
        }
    }
    /* Check features */
    if flg & 0x10 as libc::c_int != 0 {
        /* Start with "dist" of zero */
        dist = 0 as libc::c_int;
        /* Effect ? */
        if flg & 0x10000 as libc::c_int != 0 {
            effect =
                new_effect(typ, dam, project_time, (*p_ptr).py as libc::c_int,
                           (*p_ptr).px as libc::c_int, rad,
                           project_time_effect);
            project_time = 0 as libc::c_int;
            project_time_effect = 0 as libc::c_int
        }
        /* Scan for features */
        i = 0 as libc::c_int;
        while i < grids {
            /* Hack -- Notice new "dist" values */
            if gm[(dist + 1 as libc::c_int) as usize] as libc::c_int == i {
                dist += 1
            }
            /* Get the grid location */
            y = gy[i as usize] as libc::c_int;
            x = gx[i as usize] as libc::c_int;
            /* Affect the feature in that grid */
            if project_f(who, dist, y, x, dam, typ) != 0 {
                notice = 1 as libc::c_int as bool_
            }
            /* Effect ? */
            if flg & 0x10000 as libc::c_int != 0 {
                (*cave[y as usize].offset(x as isize)).effect =
                    effect as s16b;
                lite_spot(y, x);
            }
            i += 1
        }
    }
    /* Update stuff if needed */
    if (*p_ptr).update != 0 { update_stuff(); }
    /* Check objects */
    if flg & 0x20 as libc::c_int != 0 {
        /* Start with "dist" of zero */
        dist = 0 as libc::c_int;
        /* Scan for objects */
        i = 0 as libc::c_int;
        while i < grids {
            /* Hack -- Notice new "dist" values */
            if gm[(dist + 1 as libc::c_int) as usize] as libc::c_int == i {
                dist += 1
            }
            /* Get the grid location */
            y = gy[i as usize] as libc::c_int;
            x = gx[i as usize] as libc::c_int;
            /* Affect the object in the grid */
            if project_o(who, dist, y, x, dam, typ) != 0 {
                notice = 1 as libc::c_int as bool_
            }
            i += 1
        }
    }
    /* Check monsters */
    if flg & 0x40 as libc::c_int != 0 {
        /* Mega-Hack */
        project_m_n = 0 as libc::c_int;
        project_m_x = 0 as libc::c_int;
        project_m_y = 0 as libc::c_int;
        /* Start with "dist" of zero */
        dist = 0 as libc::c_int;
        /* Scan for monsters */
        i = 0 as libc::c_int;
        while i < grids {
            /* Hack -- Notice new "dist" values */
            if gm[(dist + 1 as libc::c_int) as usize] as libc::c_int == i {
                dist += 1
            }
            /* Get the grid location */
            y = gy[i as usize] as libc::c_int;
            x = gx[i as usize] as libc::c_int;
            if grids > 1 as libc::c_int {
                /* Affect the monster in the grid */
                if project_m(who, dist, y, x, dam, typ) != 0 {
                    notice = 1 as libc::c_int as bool_
                }
            } else {
                let mut ref_ptr: *mut monster_race =
                    if !(*m_list.offset((*cave[y as
                                                   usize].offset(x as
                                                                     isize)).m_idx
                                            as isize)).sr_ptr.is_null() {
                        (*m_list.offset((*cave[y as
                                                   usize].offset(x as
                                                                     isize)).m_idx
                                            as isize)).sr_ptr
                    } else {
                        race_info_idx((*m_list.offset((*cave[y as
                                                                 usize].offset(x
                                                                                   as
                                                                                   isize)).m_idx
                                                          as isize)).r_idx as
                                          libc::c_int,
                                      (*m_list.offset((*cave[y as
                                                                 usize].offset(x
                                                                                   as
                                                                                   isize)).m_idx
                                                          as isize)).ego as
                                          libc::c_int)
                    };
                if (*ref_ptr).flags2 & 0x8 as libc::c_int as libc::c_uint != 0
                       &&
                       Rand_div(10 as libc::c_int) + 1 as libc::c_int !=
                           1 as libc::c_int && dist_hack > 1 as libc::c_int {
                    let mut t_y: libc::c_int = 0;
                    let mut t_x: libc::c_int = 0;
                    let mut max_attempts: libc::c_int = 10 as libc::c_int;
                    loop 
                         /* Choose 'new' target */
                         {
                        t_y =
                            y_saver - 1 as libc::c_int +
                                (Rand_div(3 as libc::c_int) +
                                     1 as libc::c_int);
                        t_x =
                            x_saver - 1 as libc::c_int +
                                (Rand_div(3 as libc::c_int) +
                                     1 as libc::c_int);
                        max_attempts -= 1;
                        if !(max_attempts != 0 &&
                                 (t_y >= 0 as libc::c_int &&
                                      t_x >= 0 as libc::c_int &&
                                      t_y < cur_hgt as libc::c_int &&
                                      t_x < cur_wid as libc::c_int) &&
                                 los(y, x, t_y, t_x) == 0) {
                            break ;
                        }
                    }
                    if max_attempts < 1 as libc::c_int {
                        t_y = y_saver;
                        t_x = x_saver
                    }
                    if (*m_list.offset((*cave[y as
                                                  usize].offset(x as
                                                                    isize)).m_idx
                                           as isize)).ml != 0 {
                        msg_print(b"The attack bounces!\x00" as *const u8 as
                                      *const libc::c_char);
                        (*ref_ptr).r_flags2 |=
                            0x8 as libc::c_int as libc::c_uint
                    }
                    project((*cave[y as usize].offset(x as isize)).m_idx as
                                libc::c_int, 0 as libc::c_int, t_y, t_x, dam,
                            typ, flg);
                } else if project_m(who, dist, y, x, dam, typ) != 0 {
                    notice = 1 as libc::c_int as bool_
                }
            }
            i += 1
        }
        /* Player affected one monster (without "jumping") */
        if who < 0 as libc::c_int && project_m_n == 1 as libc::c_int &&
               flg & 0x1 as libc::c_int == 0 {
            /* Location */
            x = project_m_x;
            y = project_m_y;
            /* Track if possible */
            if (*cave[y as usize].offset(x as isize)).m_idx as libc::c_int >
                   0 as libc::c_int {
                let mut m_ptr: *mut monster_type =
                    &mut *m_list.offset((*(*cave.as_mut_ptr().offset(y as
                                                                         isize)).offset(x
                                                                                            as
                                                                                            isize)).m_idx
                                            as isize) as *mut monster_type;
                /* Hack -- auto-recall */
                if (*m_ptr).ml != 0 {
                    monster_race_track((*m_ptr).r_idx as libc::c_int,
                                       (*m_ptr).ego as libc::c_int);
                }
                /* Hack - auto-track */
                if (*m_ptr).ml != 0 {
                    health_track((*cave[y as usize].offset(x as isize)).m_idx
                                     as libc::c_int);
                }
            }
        }
    }
    /* Check player */
    if flg & 0x40 as libc::c_int != 0 {
        /* Start with "dist" of zero */
        dist = 0 as libc::c_int;
        /* Scan for player */
        i = 0 as libc::c_int;
        while i < grids {
            /* Hack -- Notice new "dist" values */
            if gm[(dist + 1 as libc::c_int) as usize] as libc::c_int == i {
                dist += 1
            }
            /* Get the grid location */
            y = gy[i as usize] as libc::c_int;
            x = gx[i as usize] as libc::c_int;
            /* Affect the player */
            if project_p(who, dist, y, x, dam, typ, rad) != 0 {
                notice = 1 as libc::c_int as bool_
            }
            i += 1
        }
    }
    return notice;
}
/* Return "something was noticed" */
/*
 * Potions "smash open" and cause an area effect when
 * (1) they are shattered while in the player's inventory,
 * due to cold (etc) attacks;
 * (2) they are thrown at a monster, or obstacle;
 * (3) they are shattered by a "cold ball" or other such spell
 * while lying on the floor.
 *
 * Arguments:
 *    who   ---  who caused the potion to shatter (0=player)
 *          potions that smash on the floor are assumed to
 *          be caused by no-one (who = 1), as are those that
 *          shatter inside the player inventory.
 *          (Not anymore -- I changed this; TY)
 *    y, x  --- coordinates of the potion (or player if
 *          the potion was in her inventory);
 *    o_ptr --- pointer to the potion object.
 */
#[no_mangle]
pub unsafe extern "C" fn potion_smash_effect(mut who: libc::c_int,
                                             mut y: libc::c_int,
                                             mut x: libc::c_int,
                                             mut o_sval: libc::c_int)
 -> bool_ {
    let mut radius: libc::c_int = 2 as libc::c_int;
    let mut dt: libc::c_int = 0 as libc::c_int;
    let mut dam: libc::c_int = 0 as libc::c_int;
    let mut ident: bool_ = 0 as libc::c_int as bool_;
    let mut angry: bool_ = 0 as libc::c_int as bool_;
    match o_sval {
        5 | 2 | 13 | 16 | 17 | 18 | 19 | 20 | 21 | 0 | 1 => {
            /* perhaps a 'water' attack? */
            return 1 as libc::c_int as bool_
        }
        24 | 25 | 26 | 27 | 28 | 30 | 31 | 32 | 33 | 41 | 42 | 43 | 44 | 45 |
        46 | 47 | 48 | 49 | 50 | 51 | 52 | 53 | 55 | 56 | 57 | 58 | 59 | 60 |
        62 | 63 => {
            /* All of the above potions have no effect when shattered */
            return 0 as libc::c_int as bool_
        }
        4 => {
            dt = 55 as libc::c_int;
            dam = 5 as libc::c_int;
            ident = 1 as libc::c_int as bool_;
            angry = 1 as libc::c_int as bool_
        }
        6 => {
            dt = 2 as libc::c_int;
            dam = 3 as libc::c_int;
            ident = 1 as libc::c_int as bool_;
            angry = 1 as libc::c_int as bool_
        }
        7 => {
            dt = 16 as libc::c_int;
            ident = 1 as libc::c_int as bool_;
            angry = 1 as libc::c_int as bool_
        }
        9 => {
            /* Booze */
            dt = 56 as libc::c_int; /* !! */
            ident = 1 as libc::c_int as bool_;
            angry = 1 as libc::c_int as bool_
        }
        11 => {
            dt = 57 as libc::c_int;
            angry = 1 as libc::c_int as bool_;
            ident = 1 as libc::c_int as bool_
        }
        15 | 22 => {
            dt = 20 as libc::c_int;
            dam =
                damroll(25 as libc::c_int as s16b, 25 as libc::c_int as s16b);
            angry = 1 as libc::c_int as bool_;
            ident = 1 as libc::c_int as bool_
        }
        23 => {
            dt = 26 as libc::c_int;
            dam =
                damroll(10 as libc::c_int as s16b, 10 as libc::c_int as s16b);
            angry = 1 as libc::c_int as bool_;
            radius = 1 as libc::c_int;
            ident = 1 as libc::c_int as bool_
        }
        29 => { dt = 54 as libc::c_int; ident = 1 as libc::c_int as bool_ }
        34 => {
            dt = 53 as libc::c_int;
            dam = damroll(2 as libc::c_int as s16b, 3 as libc::c_int as s16b);
            ident = 1 as libc::c_int as bool_
        }
        35 => {
            dt = 53 as libc::c_int;
            dam = damroll(4 as libc::c_int as s16b, 3 as libc::c_int as s16b);
            ident = 1 as libc::c_int as bool_
        }
        36 | 61 => {
            dt = 53 as libc::c_int;
            dam = damroll(6 as libc::c_int as s16b, 3 as libc::c_int as s16b);
            ident = 1 as libc::c_int as bool_
        }
        37 => {
            dt = 53 as libc::c_int;
            dam =
                damroll(10 as libc::c_int as s16b, 10 as libc::c_int as s16b);
            ident = 1 as libc::c_int as bool_
        }
        38 | 39 => {
            dt = 53 as libc::c_int;
            dam =
                damroll(50 as libc::c_int as s16b, 50 as libc::c_int as s16b);
            radius = 1 as libc::c_int;
            ident = 1 as libc::c_int as bool_
        }
        40 => {
            /* MANA */
            dt = 26 as libc::c_int;
            dam =
                damroll(10 as libc::c_int as s16b, 10 as libc::c_int as s16b);
            radius = 1 as libc::c_int;
            ident = 1 as libc::c_int as bool_
        }
        _ => { }
    }
    project(who, radius, y, x, dam, dt,
            0x1 as libc::c_int | 0x20 as libc::c_int | 0x40 as libc::c_int);
    /* XXX	those potions that explode need to become "known" */
    return angry;
}
/* This is for Thaumaturgy */
static mut destructive_attack_types: [libc::c_int; 10] =
    [40 as libc::c_int, 40 as libc::c_int, 40 as libc::c_int,
     76 as libc::c_int, 76 as libc::c_int, 76 as libc::c_int,
     94 as libc::c_int, 94 as libc::c_int, 94 as libc::c_int,
     94 as libc::c_int];
/* Also for Power-mages */
static mut attack_types: [libc::c_int; 25] =
    [11 as libc::c_int, 10 as libc::c_int, 26 as libc::c_int,
     14 as libc::c_int, 12 as libc::c_int, 27 as libc::c_int,
     28 as libc::c_int, 35 as libc::c_int, 24 as libc::c_int,
     23 as libc::c_int, 34 as libc::c_int, 3 as libc::c_int, 1 as libc::c_int,
     5 as libc::c_int, 4 as libc::c_int, 2 as libc::c_int, 15 as libc::c_int,
     16 as libc::c_int, 22 as libc::c_int, 21 as libc::c_int,
     20 as libc::c_int, 33 as libc::c_int, 31 as libc::c_int,
     30 as libc::c_int, 32 as libc::c_int];
/*
 * Describe the attack using normal names.
 */
#[no_mangle]
pub unsafe extern "C" fn describe_attack_fully(mut type_0: libc::c_int,
                                               mut r: *mut libc::c_char) {
    match type_0 {
        11 => {
            strcpy(r, b"arrows\x00" as *const u8 as *const libc::c_char);
        }
        10 => {
            strcpy(r,
                   b"magic missiles\x00" as *const u8 as *const libc::c_char);
        }
        26 => { strcpy(r, b"mana\x00" as *const u8 as *const libc::c_char); }
        17 => { strcpy(r, b"light\x00" as *const u8 as *const libc::c_char); }
        18 => { strcpy(r, b"dark\x00" as *const u8 as *const libc::c_char); }
        14 => { strcpy(r, b"water\x00" as *const u8 as *const libc::c_char); }
        12 => {
            strcpy(r, b"plasma\x00" as *const u8 as *const libc::c_char);
        }
        27 => {
            strcpy(r, b"meteors\x00" as *const u8 as *const libc::c_char);
        }
        28 => { strcpy(r, b"ice\x00" as *const u8 as *const libc::c_char); }
        35 => {
            strcpy(r, b"gravity\x00" as *const u8 as *const libc::c_char);
        }
        24 => {
            strcpy(r, b"inertia\x00" as *const u8 as *const libc::c_char);
        }
        23 => { strcpy(r, b"force\x00" as *const u8 as *const libc::c_char); }
        34 => {
            strcpy(r, b"pure time\x00" as *const u8 as *const libc::c_char);
        }
        3 => { strcpy(r, b"acid\x00" as *const u8 as *const libc::c_char); }
        1 => {
            strcpy(r, b"lightning\x00" as *const u8 as *const libc::c_char);
        }
        5 => { strcpy(r, b"flames\x00" as *const u8 as *const libc::c_char); }
        4 => { strcpy(r, b"cold\x00" as *const u8 as *const libc::c_char); }
        2 => { strcpy(r, b"poison\x00" as *const u8 as *const libc::c_char); }
        15 => {
            strcpy(r, b"pure light\x00" as *const u8 as *const libc::c_char);
        }
        16 => {
            strcpy(r, b"pure dark\x00" as *const u8 as *const libc::c_char);
        }
        22 => {
            strcpy(r, b"confusion\x00" as *const u8 as *const libc::c_char);
        }
        21 => { strcpy(r, b"sound\x00" as *const u8 as *const libc::c_char); }
        20 => {
            strcpy(r, b"shards\x00" as *const u8 as *const libc::c_char);
        }
        33 => { strcpy(r, b"nexus\x00" as *const u8 as *const libc::c_char); }
        31 => {
            strcpy(r, b"nether\x00" as *const u8 as *const libc::c_char);
        }
        30 => { strcpy(r, b"chaos\x00" as *const u8 as *const libc::c_char); }
        32 => {
            strcpy(r,
                   b"disenchantment\x00" as *const u8 as *const libc::c_char);
        }
        40 => {
            strcpy(r,
                   b"wall destruction\x00" as *const u8 as
                       *const libc::c_char);
        }
        41 => {
            strcpy(r,
                   b"door destruction\x00" as *const u8 as
                       *const libc::c_char);
        }
        42 => {
            strcpy(r,
                   b"trap destruction\x00" as *const u8 as
                       *const libc::c_char);
        }
        76 => {
            strcpy(r,
                   b"wall creation\x00" as *const u8 as *const libc::c_char);
        }
        46 => {
            strcpy(r,
                   b"door creation\x00" as *const u8 as *const libc::c_char);
        }
        47 => {
            strcpy(r,
                   b"trap creation\x00" as *const u8 as *const libc::c_char);
        }
        94 => {
            strcpy(r, b"destruction\x00" as *const u8 as *const libc::c_char);
        }
        _ => {
            strcpy(r,
                   b"something unknown\x00" as *const u8 as
                       *const libc::c_char);
        }
    };
}
/*
 * Give a randomly-generated spell a name.
 * Note that it only describes the first effect!
 */
unsafe extern "C" fn name_spell(mut s_ptr: *mut random_spell) {
    let mut buff: [libc::c_char; 30] = [0; 30];
    let mut buff2: cptr = b"???\x00" as *const u8 as *const libc::c_char;
    if (*s_ptr).proj_flags & 0x8 as libc::c_int as libc::c_uint != 0 &&
           (*s_ptr).radius as libc::c_int == 0 as libc::c_int {
        buff2 = b"Bolt\x00" as *const u8 as *const libc::c_char
    } else if (*s_ptr).proj_flags & 0x2 as libc::c_int as libc::c_uint != 0 {
        buff2 = b"Beam\x00" as *const u8 as *const libc::c_char
    } else if (*s_ptr).proj_flags & 0x8 as libc::c_int as libc::c_uint != 0 &&
                  (*s_ptr).radius as libc::c_int > 0 as libc::c_int {
        buff2 = b"Ball\x00" as *const u8 as *const libc::c_char
    } else if (*s_ptr).proj_flags & 0x400 as libc::c_int as libc::c_uint != 0
     {
        buff2 = b"Blast\x00" as *const u8 as *const libc::c_char
    } else if (*s_ptr).proj_flags & 0x200 as libc::c_int as libc::c_uint != 0
     {
        buff2 = b"Area\x00" as *const u8 as *const libc::c_char
    } else if (*s_ptr).proj_flags & 0x100 as libc::c_int as libc::c_uint != 0
     {
        buff2 = b"View\x00" as *const u8 as *const libc::c_char
    }
    describe_attack_fully((*s_ptr).GF as libc::c_int, buff.as_mut_ptr());
    strnfmt((*s_ptr).name.as_mut_ptr(), 30 as libc::c_int as uint_hack,
            b"%s - %s\x00" as *const u8 as *const libc::c_char, buff2,
            buff.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn generate_spell(mut plev: libc::c_int) {
    let mut rspell: *mut random_spell = 0 as *mut random_spell;
    let mut dice: libc::c_int = 0;
    let mut sides: libc::c_int = 0;
    let mut chance: libc::c_int = 0;
    let mut mana: libc::c_int = 0;
    let mut power: libc::c_int = 0;
    let mut destruc_gen: bool_ = 0 as libc::c_int as bool_;
    let mut simple_gen: bool_ = 1 as libc::c_int as bool_;
    let mut ball_desc: bool_ = 0 as libc::c_int as bool_;
    if spell_num as libc::c_int == 100 as libc::c_int { return }
    rspell =
        &mut *random_spells.as_mut_ptr().offset(spell_num as isize) as
            *mut random_spell;
    power = Rand_div(5 as libc::c_int);
    dice = plev / 2 as libc::c_int;
    sides = plev;
    mana = plev;
    /* Make the spell more or less powerful. */
    dice += power;
    sides += power;
    mana += plev * power / 15 as libc::c_int;
    /* Stay within reasonable bounds. */
    if dice < 1 as libc::c_int { dice = 1 as libc::c_int }
    if sides < 5 as libc::c_int { sides = 5 as libc::c_int }
    if mana < 1 as libc::c_int { mana = 1 as libc::c_int }
    (*rspell).level = plev as byte_hack;
    (*rspell).mana = mana as s16b;
    (*rspell).untried = 1 as libc::c_int as bool_;
    /* Spells are always maximally destructive. */
    (*rspell).proj_flags =
        (0x40 as libc::c_int | 0x20 as libc::c_int | 0x10 as libc::c_int) as
            u32b;
    chance = Rand_div(100 as libc::c_int) + 1 as libc::c_int;
    /* Hack -- Always start with Magic Missile or derivative at lev. 1 */
    if plev == 1 as libc::c_int || chance < 25 as libc::c_int {
        (*rspell).proj_flags |= 0x8 as libc::c_int as libc::c_uint;
        /* Swap dice and sides for better damage */
        (*rspell).dam_dice = sides as byte_hack;
        (*rspell).dam_sides = dice as byte_hack;
        (*rspell).radius = 0 as libc::c_int as byte_hack
    } else if chance < 50 as libc::c_int {
        (*rspell).proj_flags |= 0x2 as libc::c_int as libc::c_uint;
        (*rspell).dam_dice = dice as byte_hack;
        (*rspell).dam_sides = sides as byte_hack;
        (*rspell).radius = 0 as libc::c_int as byte_hack
    } else if chance < 76 as libc::c_int {
        (*rspell).proj_flags |= 0x8 as libc::c_int as libc::c_uint;
        (*rspell).radius = (dice / 3 as libc::c_int) as byte_hack;
        (*rspell).dam_dice = dice as byte_hack;
        (*rspell).dam_sides = sides as byte_hack;
        ball_desc = 1 as libc::c_int as bool_
    } else if chance < 83 as libc::c_int {
        (*rspell).proj_flags |= 0x400 as libc::c_int as libc::c_uint;
        (*rspell).radius = (sides / 3 as libc::c_int) as byte_hack;
        (*rspell).dam_dice = dice as byte_hack;
        (*rspell).dam_sides = sides as byte_hack;
        destruc_gen = 1 as libc::c_int as bool_;
        simple_gen = 0 as libc::c_int as bool_
    } else if chance < 90 as libc::c_int {
        (*rspell).proj_flags |= 0x200 as libc::c_int as libc::c_uint;
        /* Area effect spells do way less damage "per shot" */
        (*rspell).dam_dice = (dice / 5 as libc::c_int) as byte_hack;
        (*rspell).dam_sides = (sides / 5 as libc::c_int) as byte_hack;
        (*rspell).radius = (sides / 3 as libc::c_int) as byte_hack;
        if ((*rspell).radius as libc::c_int) < 4 as libc::c_int {
            (*rspell).radius = 4 as libc::c_int as byte_hack
        }
        destruc_gen = 1 as libc::c_int as bool_
    } else {
        (*rspell).proj_flags |= 0x100 as libc::c_int as libc::c_uint;
        /* View spells do less damage */
        (*rspell).dam_dice = dice as byte_hack;
        (*rspell).dam_sides = (sides / 2 as libc::c_int) as byte_hack
    }
    /* Both a destructive and a simple spell requested --
	 * pick one or the other. */
    if destruc_gen as libc::c_int != 0 && simple_gen as libc::c_int != 0 {
        if Rand_div(100 as libc::c_int) < 25 as libc::c_int {
            simple_gen = 0 as libc::c_int as bool_
        } else { destruc_gen = 0 as libc::c_int as bool_ }
    }
    /* Pick a simple spell */
    if simple_gen != 0 {
        (*rspell).GF =
            attack_types[Rand_div(25 as libc::c_int) as usize] as byte_hack
    } else {
        /* Pick a destructive spell */
        (*rspell).GF =
            destructive_attack_types[Rand_div(10 as libc::c_int) as usize] as
                byte_hack
    }
    /* Give the spell a name. */
    name_spell(rspell);
    if ball_desc != 0 {
        /* 30 character limit on the string! */
        sprintf((*rspell).desc.as_mut_ptr(),
                b"Dam: %d, Rad: %d, Pow: %d\x00" as *const u8 as
                    *const libc::c_char, sides, dice, power);
    } else {
        sprintf((*rspell).desc.as_mut_ptr(),
                b"Damage: %dd%d, Power: %d\x00" as *const u8 as
                    *const libc::c_char, dice, sides, power);
    }
    spell_num += 1;
}
/*
 * Polymorph a monster at given location.
 */
#[no_mangle]
pub unsafe extern "C" fn do_poly_monster(mut y: libc::c_int,
                                         mut x: libc::c_int) -> s16b {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    let mut hack_m_idx: s16b = 0;
    let mut old_m_idx: s16b = 0;
    let mut new_m_idx: s16b = 0 as libc::c_int as s16b;
    let mut new_r_idx: s16b = 0;
    /* Get a "old" monster */
    old_m_idx = (*c_ptr).m_idx;
    /* Giga-Hack -- Remember monster */
    hack_m_idx = old_m_idx;
    /* Get a monster */
    m_ptr = &mut *m_list.offset((*c_ptr).m_idx as isize) as *mut monster_type;
    /* Pick a "new" monster race */
    new_r_idx = poly_r_idx((*m_ptr).r_idx as libc::c_int);
    /* No polymorph happend */
    if new_r_idx as libc::c_int == (*m_ptr).r_idx as libc::c_int {
        return 0 as libc::c_int as s16b
    }
    /* Giga-Hack -- Removes the moster XXX XXX XXX XXX */
    (*c_ptr).m_idx = 0 as libc::c_int as s16b;
    /*
	 * Handle polymorph --
	 * Create a new monster (no groups)
	 */
    if place_monster_aux(y, x, new_r_idx as libc::c_int,
                         0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
                         (*m_ptr).status as libc::c_int) != 0 {
        /* Get a "new" monster */
        new_m_idx = (*c_ptr).m_idx;
        /* Giga-Hack -- Remember "new" monster */
        hack_m_idx = new_m_idx;
        /* "Kill" the "old" monster */
        delete_monster_idx(old_m_idx as libc::c_int);
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as
                u32b
    }
    /* Giga-Hack -- restore saved monster XXX XXX XXX */
    (*c_ptr).m_idx = hack_m_idx;
    return new_m_idx;
}
