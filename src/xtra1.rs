use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut adj_mag_mana: [byte_hack; 0];
    #[no_mangle]
    static mut adj_wis_sav: [byte_hack; 0];
    #[no_mangle]
    static mut adj_dex_dis: [byte_hack; 0];
    #[no_mangle]
    static mut adj_int_dis: [byte_hack; 0];
    #[no_mangle]
    static mut adj_dex_ta: [byte_hack; 0];
    #[no_mangle]
    static mut adj_str_td: [byte_hack; 0];
    #[no_mangle]
    static mut adj_dex_th: [byte_hack; 0];
    #[no_mangle]
    static mut adj_str_th: [byte_hack; 0];
    #[no_mangle]
    static mut adj_str_wgt: [byte_hack; 0];
    #[no_mangle]
    static mut adj_str_hold: [byte_hack; 0];
    #[no_mangle]
    static mut adj_str_dig: [byte_hack; 0];
    #[no_mangle]
    static mut adj_str_blow: [byte_hack; 0];
    #[no_mangle]
    static mut adj_dex_blow: [byte_hack; 0];
    #[no_mangle]
    static mut adj_con_mhp: [byte_hack; 0];
    #[no_mangle]
    static mut blows_table: [[byte_hack; 12]; 12];
    #[no_mangle]
    static mut player_exp: [s32b; 50];
    #[no_mangle]
    static mut stat_names: [cptr; 6];
    #[no_mangle]
    static mut stat_names_reduced: [cptr; 6];
    #[no_mangle]
    static mut move_info: [move_info_type; 9];
    #[no_mangle]
    static mut tactic_info: [tactic_info_type; 9];
    #[no_mangle]
    static mut max_body_part: [libc::c_int; 6];
    #[no_mangle]
    static mut character_generated: bool_;
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut character_xtra: bool_;
    #[no_mangle]
    static mut command_rep: s16b;
    #[no_mangle]
    static mut running: s16b;
    #[no_mangle]
    static mut resting: s16b;
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut wizard: bool_;
    #[no_mangle]
    static mut total_winner: u16b;
    #[no_mangle]
    static mut m_max: s16b;
    #[no_mangle]
    static mut depth_in_feet: bool_;
    #[no_mangle]
    static mut monster_lite: bool_;
    #[no_mangle]
    static mut view_reduce_lite: bool_;
    #[no_mangle]
    static mut hitpoint_warn: byte_hack;
    #[no_mangle]
    static mut health_who: s16b;
    #[no_mangle]
    static mut monster_race_idx: s16b;
    #[no_mangle]
    static mut monster_ego_idx: s16b;
    #[no_mangle]
    static mut tracked_object: *mut object_type;
    #[no_mangle]
    static mut window_flag: [u32b; 8];
    #[no_mangle]
    static mut angband_term: [*mut term; 8];
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut o_list: *mut object_type;
    #[no_mangle]
    static mut m_list: *mut monster_type;
    #[no_mangle]
    static mut alloc_kind_table_valid: bool_;
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut rp_ptr: *mut player_race;
    #[no_mangle]
    static mut rmp_ptr: *mut player_race_mod;
    #[no_mangle]
    static mut cp_ptr: *mut player_class;
    #[no_mangle]
    static mut spp_ptr: *mut player_spec;
    #[no_mangle]
    static mut player_hp: [s16b; 50];
    #[no_mangle]
    static mut s_info: *mut skill_type;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut a_info: *mut artifact_type;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut r_name: *mut libc::c_char;
    #[no_mangle]
    static mut d_info: *mut dungeon_info_type;
    #[no_mangle]
    static mut c_name: *mut libc::c_char;
    #[no_mangle]
    static mut c_text: *mut libc::c_char;
    #[no_mangle]
    static mut rp_name: *mut libc::c_char;
    #[no_mangle]
    static mut wf_info: *mut wilderness_type_info;
    #[no_mangle]
    static mut wf_name: *mut libc::c_char;
    #[no_mangle]
    static mut get_obj_num_hook:
           Option<unsafe extern "C" fn(_: libc::c_int) -> bool_>;
    #[no_mangle]
    static mut monk_armour_aux: bool_;
    #[no_mangle]
    static mut monk_notify_aux: bool_;
    #[no_mangle]
    static mut wild_map: *mut *mut wilderness_map;
    #[no_mangle]
    static mut max_s_idx: u16b;
    #[no_mangle]
    static mut max_r_idx: u16b;
    #[no_mangle]
    static mut max_a_idx: u16b;
    #[no_mangle]
    static mut fates: [fate; 200];
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut max_dlv: *mut s16b;
    #[no_mangle]
    static mut munchkin_multipliers: bool_;
    #[no_mangle]
    static mut exp_need: bool_;
    #[no_mangle]
    static mut dungeon_flags1: u32b;
    #[no_mangle]
    static mut dungeon_flags2: u32b;
    #[no_mangle]
    static mut powers_type: *mut power_type;
    #[no_mangle]
    static mut power_max: s16b;
    #[no_mangle]
    static mut player_char_health: bool_;
    #[no_mangle]
    static mut linear_stats: bool_;
    #[no_mangle]
    static mut max_plev: s32b;
    #[no_mangle]
    static mut process_hooks_return: [hook_return; 20];
    #[no_mangle]
    fn process_hooks_ret(h_idx: libc::c_int, ret: *mut libc::c_char,
                         fmt: *mut libc::c_char, _: ...) -> bool_;
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn lite_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn prt_map();
    #[no_mangle]
    fn display_map(cy: *mut libc::c_int, cx: *mut libc::c_int);
    #[no_mangle]
    fn forget_view();
    #[no_mangle]
    fn update_view();
    #[no_mangle]
    fn update_mon_lite();
    #[no_mangle]
    fn update_flow();
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
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn maxroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    static mut Term: *mut term;
    #[no_mangle]
    fn Term_fresh() -> errr;
    #[no_mangle]
    fn Term_putstr(x: libc::c_int, y: libc::c_int, n: libc::c_int,
                   a: byte_hack, s: cptr) -> errr;
    #[no_mangle]
    fn Term_erase(x: libc::c_int, y: libc::c_int, n: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_clear() -> errr;
    #[no_mangle]
    fn Term_get_size(w: *mut libc::c_int, h: *mut libc::c_int) -> errr;
    #[no_mangle]
    fn Term_locate(x: *mut libc::c_int, y: *mut libc::c_int) -> errr;
    #[no_mangle]
    fn Term_activate(t: *mut term) -> errr;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sroot(n: s32b) -> s32b;
    #[no_mangle]
    fn display_player(mode: libc::c_int);
    #[no_mangle]
    fn display_roff(r_idx: libc::c_int, ego: libc::c_int);
    #[no_mangle]
    fn race_info_idx(r_idx: libc::c_int, ego: libc::c_int)
     -> *mut monster_race;
    #[no_mangle]
    fn get_mon_num_prep() -> errr;
    #[no_mangle]
    fn get_mon_num(level: libc::c_int) -> s16b;
    #[no_mangle]
    fn monster_race_desc(desc: *mut libc::c_char, r_idx: libc::c_int,
                         ego: libc::c_int);
    #[no_mangle]
    fn update_monsters(full: bool_);
    #[no_mangle]
    fn apply_set(a_idx: s16b, set_idx: s16b) -> bool_;
    #[no_mangle]
    fn calc_total_weight() -> s32b;
    #[no_mangle]
    fn init_match_theme(theme: obj_theme);
    #[no_mangle]
    fn kind_is_legal(k_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn object_power(o_ptr: *mut object_type) -> libc::c_int;
    #[no_mangle]
    static mut object_flags_no_set: bool_;
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn object_desc_store(buf: *mut libc::c_char, o_ptr: *mut object_type,
                         pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn object_out_desc(o_ptr: *mut object_type, fff: *mut FILE,
                       trim_down: bool_, wait_for_it: bool_) -> bool_;
    #[no_mangle]
    fn inven_takeoff(item: libc::c_int, amt: libc::c_int, force_drop: bool_)
     -> s16b;
    #[no_mangle]
    fn display_inven();
    #[no_mangle]
    fn display_equip();
    #[no_mangle]
    fn get_obj_num_prep() -> errr;
    #[no_mangle]
    fn get_obj_num(level: libc::c_int) -> s16b;
    #[no_mangle]
    fn lookup_kind(tval: libc::c_int, sval: libc::c_int) -> s16b;
    #[no_mangle]
    fn object_wipe(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_prep(o_ptr: *mut object_type, k_idx: libc::c_int);
    #[no_mangle]
    fn combine_pack();
    #[no_mangle]
    fn reorder_pack();
    #[no_mangle]
    fn random_artifact_resistance(o_ptr: *mut object_type);
    #[no_mangle]
    fn message_str(age: libc::c_int) -> cptr;
    #[no_mangle]
    fn message_color(age: libc::c_int) -> byte_hack;
    #[no_mangle]
    fn display_message(x: libc::c_int, y: libc::c_int, split: libc::c_int,
                       color: byte_hack, t: cptr);
    #[no_mangle]
    fn cmsg_print(color: byte_hack, msg: cptr);
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn c_put_str(attr: byte_hack, str: cptr, row: libc::c_int,
                 col: libc::c_int);
    #[no_mangle]
    fn put_str(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn c_prt(attr: byte_hack, str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn text_out(str: cptr);
    #[no_mangle]
    fn get_skill_scale(skill: libc::c_int, scale: u32b) -> s16b;
    #[no_mangle]
    fn get_skill(skill: libc::c_int) -> s16b;
    #[no_mangle]
    fn call_lua(function: cptr, args: cptr, ret: cptr, _: ...) -> bool_;
    #[no_mangle]
    fn forbid_gloves() -> bool_;
    #[no_mangle]
    fn forbid_non_blessed() -> bool_;
    #[no_mangle]
    fn resolve_mimic_name(name: cptr) -> libc::c_int;
    #[no_mangle]
    fn has_ability(ab: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_stun(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn wisdom_scale(max: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn get_dungeon_name(buf: *mut libc::c_char) -> bool_;
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
    pub abilities: [C2RustUnnamed; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
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
pub struct power_type {
    pub name: *mut libc::c_char,
    pub desc_text: *mut libc::c_char,
    pub gain_text: *mut libc::c_char,
    pub lose_text: *mut libc::c_char,
    pub level: byte_hack,
    pub cost: byte_hack,
    pub stat: byte_hack,
    pub diff: byte_hack,
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
/* File: misc.c */
/* Purpose: misc code */
/*
 * Copyright (c) 1989 James E. Wilson, Robert A. Koeneke
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Converts stat num into a six-char (right justified) string
 */
#[no_mangle]
pub unsafe extern "C" fn cnv_stat(mut val: libc::c_int,
                                  mut out_val: *mut libc::c_char) {
    if linear_stats == 0 {
        /* Above 18 */
        if val > 18 as libc::c_int {
            let mut bonus: libc::c_int = val - 18 as libc::c_int;
            if bonus >= 220 as libc::c_int {
                sprintf(out_val,
                        b"18/%3s\x00" as *const u8 as *const libc::c_char,
                        b"***\x00" as *const u8 as *const libc::c_char);
            } else if bonus >= 100 as libc::c_int {
                sprintf(out_val,
                        b"18/%03d\x00" as *const u8 as *const libc::c_char,
                        bonus);
            } else {
                sprintf(out_val,
                        b" 18/%02d\x00" as *const u8 as *const libc::c_char,
                        bonus);
            }
        } else {
            /* From 3 to 18 */
            sprintf(out_val,
                    b"    %2d\x00" as *const u8 as *const libc::c_char, val);
        }
    } else if val > 18 as libc::c_int {
        let mut bonus_0: libc::c_int = val - 18 as libc::c_int;
        if bonus_0 >= 220 as libc::c_int {
            sprintf(out_val,
                    b"    40\x00" as *const u8 as *const libc::c_char);
        } else {
            sprintf(out_val,
                    b"    %2d\x00" as *const u8 as *const libc::c_char,
                    18 as libc::c_int + bonus_0 / 10 as libc::c_int);
        }
    } else {
        /* Above 18 */
        /* From 3 to 18 */
        sprintf(out_val, b"    %2d\x00" as *const u8 as *const libc::c_char,
                val);
    };
}
/*
 * Modify a stat value by a "modifier", return new value
 *
 * Stats go up: 3,4,...,17,18,18/10,18/20,...,18/220
 * Or even: 18/13, 18/23, 18/33, ..., 18/220
 *
 * Stats go down: 18/220, 18/210,..., 18/10, 18, 17, ..., 3
 * Or even: 18/13, 18/03, 18, 17, ..., 3
 */
#[no_mangle]
pub unsafe extern "C" fn modify_stat_value(mut value: libc::c_int,
                                           mut amount: libc::c_int) -> s16b {
    let mut i: libc::c_int = 0;
    /* Reward */
    if amount > 0 as libc::c_int {
        /* Apply each point */
        i = 0 as libc::c_int;
        while i < amount {
            /* One point at a time */
            if value < 18 as libc::c_int {
                value += 1
            } else {
                /* Ten "points" at a time */
                value += 10 as libc::c_int
            }
            i += 1
        }
    } else if amount < 0 as libc::c_int {
        /* Penalty */
        /* Apply each point */
        i = 0 as libc::c_int;
        while i < 0 as libc::c_int - amount {
            /* Ten points at a time */
            if value >= 18 as libc::c_int + 10 as libc::c_int {
                value -= 10 as libc::c_int
            } else if value > 18 as libc::c_int {
                value = 18 as libc::c_int
            } else if value > 3 as libc::c_int { value -= 1 }
            i += 1
        }
    }
    /* Hack -- prevent weirdness */
    /* One point at a time */
    /* Return new value */
    return value as s16b;
}
/*
 * Print character info at given row, column in a 13 char field
 */
unsafe extern "C" fn prt_field(mut info: cptr, mut row: libc::c_int,
                               mut col: libc::c_int) {
    /* Dump 13 spaces to clear */
    c_put_str(1 as libc::c_int as byte_hack,
              b"             \x00" as *const u8 as *const libc::c_char, row,
              col);
    /* Dump the info itself */
    c_put_str(14 as libc::c_int as byte_hack, info, row, col);
}
/*
 * Prints players max/cur piety
 */
unsafe extern "C" fn prt_piety() {
    let mut tmp: [libc::c_char; 32] = [0; 32];
    /* Do not show piety unless it matters */
    if (*p_ptr).pgod == 0 { return }
    c_put_str(9 as libc::c_int as byte_hack,
              b"Pt \x00" as *const u8 as *const libc::c_char,
              18 as libc::c_int, 0 as libc::c_int);
    sprintf(tmp.as_mut_ptr(), b"%9ld\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).grace as libc::c_long);
    c_put_str(if (*p_ptr).praying as libc::c_int != 0 {
                  14 as libc::c_int
              } else { 5 as libc::c_int } as byte_hack,
              tmp.as_mut_ptr() as cptr, 18 as libc::c_int,
              0 as libc::c_int + 3 as libc::c_int);
}
/*
 * Prints the player's current sanity.
 */
unsafe extern "C" fn prt_sane() {
    let mut tmp: [libc::c_char; 32] = [0; 32];
    let mut color: byte_hack = 0;
    let mut perc: libc::c_int = 0;
    if (*p_ptr).msane as libc::c_int == 0 as libc::c_int {
        perc = 100 as libc::c_int
    } else {
        perc =
            100 as libc::c_int * (*p_ptr).csane as libc::c_int /
                (*p_ptr).msane as libc::c_int
    }
    c_put_str(3 as libc::c_int as byte_hack,
              b"SN \x00" as *const u8 as *const libc::c_char,
              16 as libc::c_int, 0 as libc::c_int);
    sprintf(tmp.as_mut_ptr(),
            b"%4d/%4d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).csane as libc::c_int, (*p_ptr).msane as libc::c_int);
    if perc >= 100 as libc::c_int {
        color = 13 as libc::c_int as byte_hack
    } else if perc > 10 as libc::c_int * hitpoint_warn as libc::c_int {
        color = 11 as libc::c_int as byte_hack
    } else { color = 4 as libc::c_int as byte_hack }
    c_put_str(color, tmp.as_mut_ptr() as cptr, 16 as libc::c_int,
              0 as libc::c_int + 3 as libc::c_int);
}
/*
 * Print character stat in given row, column
 */
unsafe extern "C" fn prt_stat(mut stat: libc::c_int) {
    let mut tmp: [libc::c_char; 32] = [0; 32];
    cnv_stat((*p_ptr).stat_use[stat as usize] as libc::c_int,
             tmp.as_mut_ptr());
    /* Display "injured" stat */
    if ((*p_ptr).stat_cur[stat as usize] as libc::c_int) <
           (*p_ptr).stat_max[stat as usize] as libc::c_int {
        let mut colour: libc::c_int = 0;
        if (*p_ptr).stat_cnt[stat as usize] != 0 {
            colour = 3 as libc::c_int
        } else { colour = 11 as libc::c_int }
        put_str(format(b"%s: \x00" as *const u8 as *const libc::c_char,
                       stat_names_reduced[stat as usize]) as cptr,
                8 as libc::c_int + stat, 0 as libc::c_int);
        c_put_str(colour as byte_hack, tmp.as_mut_ptr() as cptr,
                  8 as libc::c_int + stat,
                  0 as libc::c_int + 6 as libc::c_int);
    } else {
        /* Display "healthy" stat */
        put_str(format(b"%s: \x00" as *const u8 as *const libc::c_char,
                       stat_names[stat as usize]) as cptr,
                8 as libc::c_int + stat, 0 as libc::c_int);
        c_put_str(13 as libc::c_int as byte_hack, tmp.as_mut_ptr() as cptr,
                  8 as libc::c_int + stat,
                  0 as libc::c_int + 6 as libc::c_int);
    }
    /* Display "boosted" stat */
    if (*p_ptr).stat_cur[stat as usize] as libc::c_int >
           (*p_ptr).stat_max[stat as usize] as libc::c_int {
        put_str(format(b"%s: \x00" as *const u8 as *const libc::c_char,
                       stat_names[stat as usize]) as cptr,
                8 as libc::c_int + stat, 0 as libc::c_int);
        c_put_str(10 as libc::c_int as byte_hack, tmp.as_mut_ptr() as cptr,
                  8 as libc::c_int + stat,
                  0 as libc::c_int + 6 as libc::c_int);
    }
    /* Indicate natural maximum */
    if (*p_ptr).stat_max[stat as usize] as libc::c_int ==
           18 as libc::c_int + 100 as libc::c_int {
        put_str(b"!\x00" as *const u8 as *const libc::c_char,
                8 as libc::c_int + stat, 3 as libc::c_int);
    };
}
/*
 * Prints "title", including "wizard" or "winner" as needed.
 */
unsafe extern "C" fn prt_title() {
    let mut p: cptr = b"\x00" as *const u8 as *const libc::c_char;
    /* Mimic shape */
    if (*p_ptr).mimic_form != 0 {
        call_lua(b"get_mimic_info\x00" as *const u8 as *const libc::c_char,
                 b"(d,s)\x00" as *const u8 as *const libc::c_char,
                 b"s\x00" as *const u8 as *const libc::c_char,
                 (*p_ptr).mimic_form as libc::c_int,
                 b"show_name\x00" as *const u8 as *const libc::c_char,
                 &mut p as *mut cptr);
    } else if wizard != 0 {
        p = b"[=-WIZARD-=]\x00" as *const u8 as *const libc::c_char
    } else if total_winner as libc::c_int == 1 as libc::c_int {
        p = b"***WINNER***\x00" as *const u8 as *const libc::c_char
    } else if total_winner as libc::c_int == 2 as libc::c_int {
        p = b"***GOD***\x00" as *const u8 as *const libc::c_char
    } else {
        /* Wizard */
        /* Winner */
        /* Ultra Winner */
        /* Normal */
        p =
            c_text.offset((*cp_ptr).titles[(((*p_ptr).lev as libc::c_int -
                                                 1 as libc::c_int) /
                                                5 as libc::c_int) as usize] as
                              isize) as cptr
    }
    prt_field(p, 3 as libc::c_int, 0 as libc::c_int);
}
/*
 * Prints level
 */
unsafe extern "C" fn prt_level() {
    let mut tmp: [libc::c_char; 32] = [0; 32];
    sprintf(tmp.as_mut_ptr(), b"%6d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).lev as libc::c_int);
    if (*p_ptr).lev as libc::c_int >= (*p_ptr).max_plv as libc::c_int {
        put_str(b"LEVEL \x00" as *const u8 as *const libc::c_char,
                4 as libc::c_int, 0 as libc::c_int);
        c_put_str(13 as libc::c_int as byte_hack, tmp.as_mut_ptr() as cptr,
                  4 as libc::c_int, 0 as libc::c_int + 6 as libc::c_int);
    } else {
        put_str(b"Level \x00" as *const u8 as *const libc::c_char,
                4 as libc::c_int, 0 as libc::c_int);
        c_put_str(11 as libc::c_int as byte_hack, tmp.as_mut_ptr() as cptr,
                  4 as libc::c_int, 0 as libc::c_int + 6 as libc::c_int);
    };
}
/*
 * Display the experience
 */
unsafe extern "C" fn prt_exp() {
    let mut out_val: [libc::c_char; 32] = [0; 32];
    if exp_need == 0 {
        sprintf(out_val.as_mut_ptr(),
                b"%8ld\x00" as *const u8 as *const libc::c_char,
                (*p_ptr).exp as libc::c_long);
    } else if (*p_ptr).lev as libc::c_int >= 50 as libc::c_int ||
                  (*p_ptr).lev as libc::c_int >= max_plev {
        sprintf(out_val.as_mut_ptr(),
                b"********\x00" as *const u8 as *const libc::c_char);
    } else {
        sprintf(out_val.as_mut_ptr(),
                b"%8ld\x00" as *const u8 as *const libc::c_char,
                (player_exp[((*p_ptr).lev as libc::c_int - 1 as libc::c_int)
                                as usize] * (*p_ptr).expfact as libc::c_int)
                    as libc::c_long / 100 as libc::c_long -
                    (*p_ptr).exp as libc::c_long);
    }
    if (*p_ptr).exp >= (*p_ptr).max_exp {
        put_str(b"EXP \x00" as *const u8 as *const libc::c_char,
                5 as libc::c_int, 0 as libc::c_int);
        c_put_str(13 as libc::c_int as byte_hack,
                  out_val.as_mut_ptr() as cptr, 5 as libc::c_int,
                  0 as libc::c_int + 4 as libc::c_int);
    } else {
        put_str(b"Exp \x00" as *const u8 as *const libc::c_char,
                5 as libc::c_int, 0 as libc::c_int);
        c_put_str(11 as libc::c_int as byte_hack,
                  out_val.as_mut_ptr() as cptr, 5 as libc::c_int,
                  0 as libc::c_int + 4 as libc::c_int);
    };
}
/*
 * Prints current gold
 */
unsafe extern "C" fn prt_gold() {
    let mut tmp: [libc::c_char; 32] = [0; 32];
    put_str(b"AU \x00" as *const u8 as *const libc::c_char, 6 as libc::c_int,
            0 as libc::c_int);
    sprintf(tmp.as_mut_ptr(), b"%9ld\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).au as libc::c_long);
    c_put_str(13 as libc::c_int as byte_hack, tmp.as_mut_ptr() as cptr,
              6 as libc::c_int, 0 as libc::c_int + 3 as libc::c_int);
}
/*
 * Prints current AC
 */
unsafe extern "C" fn prt_ac() {
    let mut tmp: [libc::c_char; 32] = [0; 32];
    put_str(b"Cur AC \x00" as *const u8 as *const libc::c_char,
            14 as libc::c_int, 0 as libc::c_int);
    sprintf(tmp.as_mut_ptr(), b"%5d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).dis_ac as libc::c_int +
                (*p_ptr).dis_to_a as libc::c_int);
    c_put_str(13 as libc::c_int as byte_hack, tmp.as_mut_ptr() as cptr,
              14 as libc::c_int, 0 as libc::c_int + 7 as libc::c_int);
}
/*
 * Prints Cur/Max hit points
 */
unsafe extern "C" fn prt_hp() {
    let mut tmp: [libc::c_char; 32] = [0; 32];
    let mut color: byte_hack = 0;
    if player_char_health != 0 {
        lite_spot((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int);
    }
    if (*p_ptr).necro_extra & 0x8 as libc::c_int as libc::c_uint != 0 {
        c_put_str(8 as libc::c_int as byte_hack,
                  b"DP \x00" as *const u8 as *const libc::c_char,
                  15 as libc::c_int, 0 as libc::c_int);
        sprintf(tmp.as_mut_ptr(),
                b"%4d/%4d\x00" as *const u8 as *const libc::c_char,
                (*p_ptr).chp as libc::c_int, (*p_ptr).mhp as libc::c_int);
        if (*p_ptr).chp as libc::c_int >= (*p_ptr).mhp as libc::c_int {
            color = 14 as libc::c_int as byte_hack
        } else if (*p_ptr).chp as libc::c_int >
                      (*p_ptr).mhp as libc::c_int *
                          hitpoint_warn as libc::c_int / 10 as libc::c_int {
            color = 10 as libc::c_int as byte_hack
        } else { color = 12 as libc::c_int as byte_hack }
        c_put_str(color, tmp.as_mut_ptr() as cptr, 15 as libc::c_int,
                  0 as libc::c_int + 3 as libc::c_int);
    } else {
        c_put_str(4 as libc::c_int as byte_hack,
                  b"HP \x00" as *const u8 as *const libc::c_char,
                  15 as libc::c_int, 0 as libc::c_int);
        sprintf(tmp.as_mut_ptr(),
                b"%4d/%4d\x00" as *const u8 as *const libc::c_char,
                (*p_ptr).chp as libc::c_int, (*p_ptr).mhp as libc::c_int);
        if (*p_ptr).chp as libc::c_int >= (*p_ptr).mhp as libc::c_int {
            color = 13 as libc::c_int as byte_hack
        } else if (*p_ptr).chp as libc::c_int >
                      (*p_ptr).mhp as libc::c_int *
                          hitpoint_warn as libc::c_int / 10 as libc::c_int {
            color = 11 as libc::c_int as byte_hack
        } else { color = 4 as libc::c_int as byte_hack }
        c_put_str(color, tmp.as_mut_ptr() as cptr, 15 as libc::c_int,
                  0 as libc::c_int + 3 as libc::c_int);
    };
}
/*
 * Prints Cur/Max monster hit points
 */
unsafe extern "C" fn prt_mh() {
    let mut tmp: [libc::c_char; 32] = [0; 32];
    let mut color: byte_hack = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    /* Get the carried monster */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(49 as libc::c_int as
                                                         isize) as
            *mut object_type;
    if (*o_ptr).pval2 == 0 {
        put_str(b"             \x00" as *const u8 as *const libc::c_char,
                19 as libc::c_int, 0 as libc::c_int);
        return
    }
    put_str(b"MH \x00" as *const u8 as *const libc::c_char, 19 as libc::c_int,
            0 as libc::c_int);
    sprintf(tmp.as_mut_ptr(),
            b"%4d/%4d\x00" as *const u8 as *const libc::c_char,
            (*o_ptr).pval2 as libc::c_int, (*o_ptr).pval3);
    if (*o_ptr).pval2 as libc::c_int >= (*o_ptr).pval3 {
        color = 13 as libc::c_int as byte_hack
    } else if (*o_ptr).pval2 as libc::c_int >
                  (*o_ptr).pval3 * hitpoint_warn as libc::c_int /
                      10 as libc::c_int {
        color = 11 as libc::c_int as byte_hack
    } else { color = 4 as libc::c_int as byte_hack }
    c_put_str(color, tmp.as_mut_ptr() as cptr, 19 as libc::c_int,
              0 as libc::c_int + 3 as libc::c_int);
}
/*
 * Prints players max/cur spell points
 */
unsafe extern "C" fn prt_sp() {
    let mut tmp: [libc::c_char; 32] = [0; 32];
    let mut color: byte_hack = 0;
    c_put_str(13 as libc::c_int as byte_hack,
              b"SP \x00" as *const u8 as *const libc::c_char,
              17 as libc::c_int, 0 as libc::c_int);
    sprintf(tmp.as_mut_ptr(),
            b"%4d/%4d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).csp as libc::c_int, (*p_ptr).msp as libc::c_int);
    if (*p_ptr).csp as libc::c_int >= (*p_ptr).msp as libc::c_int {
        color = 13 as libc::c_int as byte_hack
    } else if (*p_ptr).csp as libc::c_int >
                  (*p_ptr).msp as libc::c_int * hitpoint_warn as libc::c_int /
                      10 as libc::c_int {
        color = 11 as libc::c_int as byte_hack
    } else { color = 4 as libc::c_int as byte_hack }
    c_put_str(color, tmp.as_mut_ptr() as cptr, 17 as libc::c_int,
              0 as libc::c_int + 3 as libc::c_int);
}
/*
 * Prints depth in stat area
 */
unsafe extern "C" fn prt_depth() {
    let mut depths: [libc::c_char; 32] = [0; 32];
    let mut d_ptr: *mut dungeon_info_type =
        &mut *d_info.offset(dungeon_type as isize) as *mut dungeon_info_type;
    if (*p_ptr).wild_mode != 0 {
        strcpy(depths.as_mut_ptr(),
               b"             \x00" as *const u8 as *const libc::c_char);
    } else if (*p_ptr).inside_arena != 0 {
        strcpy(depths.as_mut_ptr(),
               b"Arena\x00" as *const u8 as *const libc::c_char);
    } else if !(get_dungeon_name(depths.as_mut_ptr()) != 0) {
        if dungeon_flags2 as libc::c_long & 0x40 as libc::c_long != 0 {
            strcpy(depths.as_mut_ptr(),
                   b"Special\x00" as *const u8 as *const libc::c_char);
        } else if (*p_ptr).inside_quest != 0 {
            strcpy(depths.as_mut_ptr(),
                   b"Quest\x00" as *const u8 as *const libc::c_char);
        } else if dun_level == 0 {
            if !wf_name.offset((*wf_info.offset((*(*wild_map.offset((*p_ptr).wilderness_y
                                                                        as
                                                                        isize)).offset((*p_ptr).wilderness_x
                                                                                           as
                                                                                           isize)).feat
                                                    as isize)).name as
                                   isize).is_null() {
                strcpy(depths.as_mut_ptr(),
                       wf_name.offset((*wf_info.offset((*(*wild_map.offset((*p_ptr).wilderness_y
                                                                               as
                                                                               isize)).offset((*p_ptr).wilderness_x
                                                                                                  as
                                                                                                  isize)).feat
                                                           as isize)).name as
                                          isize));
            } else {
                strcpy(depths.as_mut_ptr(),
                       b"Town/Wild\x00" as *const u8 as *const libc::c_char);
            }
        } else if depth_in_feet != 0 {
            if dungeon_flags1 as libc::c_long & 0x800000 as libc::c_long != 0
               {
                strnfmt(depths.as_mut_ptr(), 32 as libc::c_int as uint_hack,
                        b"%c%c%c -%d ft\x00" as *const u8 as
                            *const libc::c_char,
                        (*d_ptr).short_name[0 as libc::c_int as usize] as
                            libc::c_int,
                        (*d_ptr).short_name[1 as libc::c_int as usize] as
                            libc::c_int,
                        (*d_ptr).short_name[2 as libc::c_int as usize] as
                            libc::c_int,
                        dun_level as libc::c_int * 50 as libc::c_int);
            } else {
                strnfmt(depths.as_mut_ptr(), 32 as libc::c_int as uint_hack,
                        b"%c%c%c %d ft\x00" as *const u8 as
                            *const libc::c_char,
                        (*d_ptr).short_name[0 as libc::c_int as usize] as
                            libc::c_int,
                        (*d_ptr).short_name[1 as libc::c_int as usize] as
                            libc::c_int,
                        (*d_ptr).short_name[2 as libc::c_int as usize] as
                            libc::c_int,
                        dun_level as libc::c_int * 50 as libc::c_int);
            }
        } else if dungeon_flags1 as libc::c_long & 0x800000 as libc::c_long !=
                      0 {
            strnfmt(depths.as_mut_ptr(), 32 as libc::c_int as uint_hack,
                    b"%c%c%c -%d\x00" as *const u8 as *const libc::c_char,
                    (*d_ptr).short_name[0 as libc::c_int as usize] as
                        libc::c_int,
                    (*d_ptr).short_name[1 as libc::c_int as usize] as
                        libc::c_int,
                    (*d_ptr).short_name[2 as libc::c_int as usize] as
                        libc::c_int, dun_level as libc::c_int);
        } else {
            strnfmt(depths.as_mut_ptr(), 32 as libc::c_int as uint_hack,
                    b"%c%c%c %d\x00" as *const u8 as *const libc::c_char,
                    (*d_ptr).short_name[0 as libc::c_int as usize] as
                        libc::c_int,
                    (*d_ptr).short_name[1 as libc::c_int as usize] as
                        libc::c_int,
                    (*d_ptr).short_name[2 as libc::c_int as usize] as
                        libc::c_int, dun_level as libc::c_int);
        }
    }
    /* Right-Adjust the "depth", and clear old values */
    if (*p_ptr).word_recall != 0 {
        c_prt(3 as libc::c_int as byte_hack,
              format(b"%13s\x00" as *const u8 as *const libc::c_char,
                     depths.as_mut_ptr()) as cptr,
              (*Term).hgt as libc::c_int - 1 as libc::c_int,
              (*Term).wid as libc::c_int - 14 as libc::c_int);
    } else {
        prt(format(b"%13s\x00" as *const u8 as *const libc::c_char,
                   depths.as_mut_ptr()) as cptr,
            (*Term).hgt as libc::c_int - 1 as libc::c_int,
            (*Term).wid as libc::c_int - 14 as libc::c_int);
    };
}
/*
 * Prints status of hunger
 */
unsafe extern "C" fn prt_hunger() {
    /* Fainting / Starving */
    if ((*p_ptr).food as libc::c_int) < 500 as libc::c_int {
        c_put_str(4 as libc::c_int as byte_hack,
                  b"Weak  \x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 1 as libc::c_int,
                  0 as libc::c_int);
    } else if ((*p_ptr).food as libc::c_int) < 1000 as libc::c_int {
        c_put_str(3 as libc::c_int as byte_hack,
                  b"Weak  \x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 1 as libc::c_int,
                  0 as libc::c_int);
    } else if ((*p_ptr).food as libc::c_int) < 2000 as libc::c_int {
        c_put_str(11 as libc::c_int as byte_hack,
                  b"Hungry\x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 1 as libc::c_int,
                  0 as libc::c_int);
    } else if ((*p_ptr).food as libc::c_int) < 10000 as libc::c_int {
        c_put_str(13 as libc::c_int as byte_hack,
                  b"      \x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 1 as libc::c_int,
                  0 as libc::c_int);
    } else if ((*p_ptr).food as libc::c_int) < 15000 as libc::c_int {
        c_put_str(13 as libc::c_int as byte_hack,
                  b"Full  \x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 1 as libc::c_int,
                  0 as libc::c_int);
    } else {
        /* Weak */
        /* Hungry */
        /* Normal */
        /* Full */
        /* Gorged */
        c_put_str(5 as libc::c_int as byte_hack,
                  b"Gorged\x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 1 as libc::c_int,
                  0 as libc::c_int);
    };
}
/*
 * Prints Blind status
 */
unsafe extern "C" fn prt_blind() {
    if (*p_ptr).blind != 0 {
        c_put_str(3 as libc::c_int as byte_hack,
                  b"Blind\x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 1 as libc::c_int,
                  7 as libc::c_int);
    } else {
        put_str(b"     \x00" as *const u8 as *const libc::c_char,
                (*Term).hgt as libc::c_int - 1 as libc::c_int,
                7 as libc::c_int);
    };
}
/*
 * Prints Confusion status
 */
unsafe extern "C" fn prt_confused() {
    if (*p_ptr).confused != 0 {
        c_put_str(3 as libc::c_int as byte_hack,
                  b"Conf\x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 1 as libc::c_int,
                  13 as libc::c_int);
    } else {
        put_str(b"    \x00" as *const u8 as *const libc::c_char,
                (*Term).hgt as libc::c_int - 1 as libc::c_int,
                13 as libc::c_int);
    };
}
/*
 * Prints Fear status
 */
unsafe extern "C" fn prt_afraid() {
    if (*p_ptr).afraid != 0 {
        c_put_str(3 as libc::c_int as byte_hack,
                  b"Afraid\x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 1 as libc::c_int,
                  18 as libc::c_int);
    } else {
        put_str(b"      \x00" as *const u8 as *const libc::c_char,
                (*Term).hgt as libc::c_int - 1 as libc::c_int,
                18 as libc::c_int);
    };
}
/*
 * Prints Poisoned status
 */
unsafe extern "C" fn prt_poisoned() {
    if (*p_ptr).poisoned != 0 {
        c_put_str(3 as libc::c_int as byte_hack,
                  b"Poison\x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 1 as libc::c_int,
                  25 as libc::c_int);
    } else {
        put_str(b"      \x00" as *const u8 as *const libc::c_char,
                (*Term).hgt as libc::c_int - 1 as libc::c_int,
                25 as libc::c_int);
    };
}
/*
 * Prints trap detection status
 */
unsafe extern "C" fn prt_dtrap() {
    if (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).info as
           libc::c_int & 0x1000 as libc::c_int != 0 {
        c_put_str(13 as libc::c_int as byte_hack,
                  b"DTrap\x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 1 as libc::c_int,
                  32 as libc::c_int);
    } else {
        put_str(b"     \x00" as *const u8 as *const libc::c_char,
                (*Term).hgt as libc::c_int - 1 as libc::c_int,
                32 as libc::c_int);
    };
}
/*
 * Prints Searching, Resting, Paralysis, or 'count' status
 * Display is always exactly 10 characters wide (see below)
 *
 * This function was a major bottleneck when resting, so a lot of
 * the text formatting code was optimized in place below.
 */
unsafe extern "C" fn prt_state() {
    let mut attr: byte_hack = 1 as libc::c_int as byte_hack;
    let mut text: [libc::c_char; 16] = [0; 16];
    /* Paralysis */
    if (*p_ptr).paralyzed != 0 {
        attr = 4 as libc::c_int as byte_hack;
        strcpy(text.as_mut_ptr(),
               b"Paralyzed!\x00" as *const u8 as *const libc::c_char);
    } else if resting != 0 {
        let mut i: libc::c_int = 0;
        /* Resting */
        /* Start with "Rest" */
        strcpy(text.as_mut_ptr(),
               b"Rest      \x00" as *const u8 as *const libc::c_char);
        /* Extensive (timed) rest */
        if resting as libc::c_int >= 1000 as libc::c_int {
            i = resting as libc::c_int / 100 as libc::c_int;
            text[9 as libc::c_int as usize] = '0' as i32 as libc::c_char;
            text[8 as libc::c_int as usize] = '0' as i32 as libc::c_char;
            text[7 as libc::c_int as usize] =
                ('0' as i32 + i % 10 as libc::c_int) as libc::c_char;
            if i >= 10 as libc::c_int {
                i = i / 10 as libc::c_int;
                text[6 as libc::c_int as usize] =
                    ('0' as i32 + i % 10 as libc::c_int) as libc::c_char;
                if i >= 10 as libc::c_int {
                    text[5 as libc::c_int as usize] =
                        ('0' as i32 + i / 10 as libc::c_int) as libc::c_char
                }
            }
        } else if resting as libc::c_int >= 100 as libc::c_int {
            i = resting as libc::c_int;
            text[9 as libc::c_int as usize] =
                ('0' as i32 + i % 10 as libc::c_int) as libc::c_char;
            i = i / 10 as libc::c_int;
            text[8 as libc::c_int as usize] =
                ('0' as i32 + i % 10 as libc::c_int) as libc::c_char;
            text[7 as libc::c_int as usize] =
                ('0' as i32 + i / 10 as libc::c_int) as libc::c_char
        } else if resting as libc::c_int >= 10 as libc::c_int {
            i = resting as libc::c_int;
            text[9 as libc::c_int as usize] =
                ('0' as i32 + i % 10 as libc::c_int) as libc::c_char;
            text[8 as libc::c_int as usize] =
                ('0' as i32 + i / 10 as libc::c_int) as libc::c_char
        } else if resting as libc::c_int > 0 as libc::c_int {
            i = resting as libc::c_int;
            text[9 as libc::c_int as usize] = ('0' as i32 + i) as libc::c_char
        } else if resting as libc::c_int == -(1 as libc::c_int) {
            text[9 as libc::c_int as usize] = '*' as i32 as libc::c_char;
            text[8 as libc::c_int as usize] = text[9 as libc::c_int as usize];
            text[7 as libc::c_int as usize] = text[8 as libc::c_int as usize];
            text[6 as libc::c_int as usize] = text[7 as libc::c_int as usize];
            text[5 as libc::c_int as usize] = text[6 as libc::c_int as usize]
        } else if resting as libc::c_int == -(2 as libc::c_int) {
            text[9 as libc::c_int as usize] = '&' as i32 as libc::c_char;
            text[8 as libc::c_int as usize] = text[9 as libc::c_int as usize];
            text[7 as libc::c_int as usize] = text[8 as libc::c_int as usize];
            text[6 as libc::c_int as usize] = text[7 as libc::c_int as usize];
            text[5 as libc::c_int as usize] = text[6 as libc::c_int as usize]
        }
    } else if command_rep != 0 {
        if command_rep as libc::c_int > 999 as libc::c_int {
            sprintf(text.as_mut_ptr(),
                    b"Rep. %3d00\x00" as *const u8 as *const libc::c_char,
                    command_rep as libc::c_int / 100 as libc::c_int);
        } else {
            sprintf(text.as_mut_ptr(),
                    b"Repeat %3d\x00" as *const u8 as *const libc::c_char,
                    command_rep as libc::c_int);
        }
    } else if (*p_ptr).searching != 0 {
        strcpy(text.as_mut_ptr(),
               b"Searching \x00" as *const u8 as *const libc::c_char);
    } else {
        /* Long (timed) rest */
        /* Medium (timed) rest */
        /* Short (timed) rest */
        /* Rest until healed */
        /* Rest until done */
        /* Repeating */
        /* Searching */
        /* Nothing interesting */
        strcpy(text.as_mut_ptr(),
               b"          \x00" as *const u8 as *const libc::c_char);
    }
    /* Display the info (or blanks) */
    c_put_str(attr, text.as_mut_ptr() as cptr,
              (*Term).hgt as libc::c_int - 1 as libc::c_int,
              38 as libc::c_int);
}
/*
 * Prints the speed of a character.			-CJS-
 */
unsafe extern "C" fn prt_speed() {
    let mut i: libc::c_int = (*p_ptr).pspeed as libc::c_int;
    let mut attr: byte_hack = 1 as libc::c_int as byte_hack;
    let mut buf: [libc::c_char; 32] =
        *::std::mem::transmute::<&[u8; 32],
                                 &mut [libc::c_char; 32]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    /* Hack -- Visually "undo" the Search Mode Slowdown */
    if (*p_ptr).searching != 0 { i += 10 as libc::c_int }
    /* Fast */
    if i > 110 as libc::c_int {
        attr = 13 as libc::c_int as byte_hack;
        sprintf(buf.as_mut_ptr(),
                b"Fast (+%d)\x00" as *const u8 as *const libc::c_char,
                i - 110 as libc::c_int);
    } else if i < 110 as libc::c_int {
        attr = 15 as libc::c_int as byte_hack;
        sprintf(buf.as_mut_ptr(),
                b"Slow (-%d)\x00" as *const u8 as *const libc::c_char,
                110 as libc::c_int - i);
    }
    /* Slow */
    /* Display the speed */
    c_put_str(attr,
              format(b"%-10s\x00" as *const u8 as *const libc::c_char,
                     buf.as_mut_ptr()) as cptr,
              (*Term).hgt as libc::c_int - 1 as libc::c_int,
              49 as libc::c_int);
}
unsafe extern "C" fn prt_study() {
    if (*p_ptr).skill_points != 0 {
        put_str(b"Skill\x00" as *const u8 as *const libc::c_char,
                (*Term).hgt as libc::c_int - 1 as libc::c_int,
                60 as libc::c_int);
    } else {
        put_str(b"     \x00" as *const u8 as *const libc::c_char,
                (*Term).hgt as libc::c_int - 1 as libc::c_int,
                60 as libc::c_int);
    };
}
unsafe extern "C" fn prt_cut() {
    let mut c: libc::c_int = (*p_ptr).cut as libc::c_int;
    if c > 1000 as libc::c_int {
        c_put_str(12 as libc::c_int as byte_hack,
                  b"Mortal wound\x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 3 as libc::c_int,
                  0 as libc::c_int);
    } else if c > 200 as libc::c_int {
        c_put_str(4 as libc::c_int as byte_hack,
                  b"Deep gash   \x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 3 as libc::c_int,
                  0 as libc::c_int);
    } else if c > 100 as libc::c_int {
        c_put_str(4 as libc::c_int as byte_hack,
                  b"Severe cut  \x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 3 as libc::c_int,
                  0 as libc::c_int);
    } else if c > 50 as libc::c_int {
        c_put_str(3 as libc::c_int as byte_hack,
                  b"Nasty cut   \x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 3 as libc::c_int,
                  0 as libc::c_int);
    } else if c > 25 as libc::c_int {
        c_put_str(3 as libc::c_int as byte_hack,
                  b"Bad cut     \x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 3 as libc::c_int,
                  0 as libc::c_int);
    } else if c > 10 as libc::c_int {
        c_put_str(11 as libc::c_int as byte_hack,
                  b"Light cut   \x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 3 as libc::c_int,
                  0 as libc::c_int);
    } else if c != 0 {
        c_put_str(11 as libc::c_int as byte_hack,
                  b"Graze       \x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 3 as libc::c_int,
                  0 as libc::c_int);
    } else {
        put_str(b"            \x00" as *const u8 as *const libc::c_char,
                (*Term).hgt as libc::c_int - 3 as libc::c_int,
                0 as libc::c_int);
    };
}
unsafe extern "C" fn prt_stun() {
    let mut s: libc::c_int = (*p_ptr).stun as libc::c_int;
    if s > 100 as libc::c_int {
        c_put_str(4 as libc::c_int as byte_hack,
                  b"Knocked out \x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 2 as libc::c_int,
                  0 as libc::c_int);
    } else if s > 50 as libc::c_int {
        c_put_str(3 as libc::c_int as byte_hack,
                  b"Heavy stun  \x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 2 as libc::c_int,
                  0 as libc::c_int);
    } else if s != 0 {
        c_put_str(3 as libc::c_int as byte_hack,
                  b"Stun        \x00" as *const u8 as *const libc::c_char,
                  (*Term).hgt as libc::c_int - 2 as libc::c_int,
                  0 as libc::c_int);
    } else {
        put_str(b"            \x00" as *const u8 as *const libc::c_char,
                (*Term).hgt as libc::c_int - 2 as libc::c_int,
                0 as libc::c_int);
    };
}
/*
 * Redraw the "monster health bar"	-DRS-
 * Rather extensive modifications by	-BEN-
 *
 * The "monster health bar" provides visual feedback on the "health"
 * of the monster currently being "tracked".  There are several ways
 * to "track" a monster, including targetting it, attacking it, and
 * affecting it (and nobody else) with a ranged attack.
 *
 * Display the monster health bar (affectionately known as the
 * "health-o-meter").  Clear health bar if nothing is being tracked.
 * Auto-track current target monster when bored.  Note that the
 * health-bar stops tracking any monster that "disappears".
 */
unsafe extern "C" fn health_redraw() {
    /* Not tracking */
    if health_who == 0 {
        /* Erase the health bar */
        Term_erase(0 as libc::c_int,
                   (*Term).hgt as libc::c_int - 4 as libc::c_int,
                   12 as libc::c_int);
    } else if (*m_list.offset(health_who as isize)).ml == 0 {
        /* Tracking an unseen monster */
        /* Indicate that the monster health is "unknown" */
        Term_putstr(0 as libc::c_int,
                    (*Term).hgt as libc::c_int - 4 as libc::c_int,
                    12 as libc::c_int, 1 as libc::c_int as byte_hack,
                    b"[----------]\x00" as *const u8 as *const libc::c_char);
    } else if (*p_ptr).image != 0 {
        /* Tracking a hallucinatory monster */
        /* Indicate that the monster health is "unknown" */
        Term_putstr(0 as libc::c_int,
                    (*Term).hgt as libc::c_int - 4 as libc::c_int,
                    12 as libc::c_int, 1 as libc::c_int as byte_hack,
                    b"[----------]\x00" as *const u8 as *const libc::c_char);
    } else if (((*m_list.offset(health_who as isize)).hp == 0) as libc::c_int)
                  < 0 as libc::c_int {
        /* Tracking a dead monster (???) */
        /* Indicate that the monster health is "unknown" */
        Term_putstr(0 as libc::c_int,
                    (*Term).hgt as libc::c_int - 4 as libc::c_int,
                    12 as libc::c_int, 1 as libc::c_int as byte_hack,
                    b"[----------]\x00" as *const u8 as *const libc::c_char);
    } else {
        /* Tracking a visible monster */
        let mut pct: libc::c_int = 0;
        let mut len: libc::c_int = 0;
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(health_who as isize) as *mut monster_type;
        /* Default to almost dead */
        let mut attr: byte_hack = 4 as libc::c_int as byte_hack;
        /* Extract the "percent" of health */
        pct =
            (100 as libc::c_long * (*m_ptr).hp as libc::c_long /
                 (*m_ptr).maxhp as libc::c_long) as libc::c_int;
        /* Badly wounded */
        if pct >= 10 as libc::c_int { attr = 12 as libc::c_int as byte_hack }
        /* Wounded */
        if pct >= 25 as libc::c_int { attr = 3 as libc::c_int as byte_hack }
        /* Somewhat Wounded */
        if pct >= 60 as libc::c_int { attr = 11 as libc::c_int as byte_hack }
        /* Healthy */
        if pct >= 100 as libc::c_int { attr = 13 as libc::c_int as byte_hack }
        /* Afraid */
        if (*m_ptr).monfear != 0 { attr = 10 as libc::c_int as byte_hack }
        /* Asleep */
        if (*m_ptr).csleep != 0 { attr = 6 as libc::c_int as byte_hack }
        /* Poisoned */
        if (*m_ptr).poisoned != 0 { attr = 5 as libc::c_int as byte_hack }
        /* Bleeding */
        if (*m_ptr).bleeding != 0 { attr = 4 as libc::c_int as byte_hack }
        /* Convert percent into "health" */
        len =
            if pct < 10 as libc::c_int {
                1 as libc::c_int
            } else if pct < 90 as libc::c_int {
                (pct / 10 as libc::c_int) + 1 as libc::c_int
            } else { 10 as libc::c_int };
        /* Default to "unknown" */
        Term_putstr(0 as libc::c_int,
                    (*Term).hgt as libc::c_int - 4 as libc::c_int,
                    12 as libc::c_int, 1 as libc::c_int as byte_hack,
                    b"[----------]\x00" as *const u8 as *const libc::c_char);
        /* Dump the current "health" (use '*' symbols) */
        Term_putstr(0 as libc::c_int + 1 as libc::c_int,
                    (*Term).hgt as libc::c_int - 4 as libc::c_int, len, attr,
                    b"**********\x00" as *const u8 as *const libc::c_char);
    };
}
/*
 * Display basic info (mostly left of map)
 */
unsafe extern "C" fn prt_frame_basic() {
    let mut i: libc::c_int = 0;
    /* Race and Class */
    prt_field(rp_name.offset((*rp_ptr).title as isize) as cptr,
              1 as libc::c_int, 0 as libc::c_int);
    prt_field(c_name.offset((*spp_ptr).title as isize) as cptr,
              2 as libc::c_int, 0 as libc::c_int);
    /* Title */
    prt_title();
    /* Level/Experience */
    prt_level();
    prt_exp();
    /* All Stats */
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int { prt_stat(i); i += 1 }
    /* Armor */
    prt_ac();
    /* Hitpoints */
    prt_hp();
    /* Current sanity */
    prt_sane();
    /* Spellpoints */
    prt_sp();
    /* Piety */
    prt_piety();
    /* Monster hitpoints */
    prt_mh();
    /* Gold */
    prt_gold();
    /* Current depth */
    prt_depth();
    /* Special */
    health_redraw();
}
/*
 * Display extra info (mostly below map)
 */
unsafe extern "C" fn prt_frame_extra() {
    /* Cut/Stun */
    prt_cut();
    prt_stun();
    /* Food */
    prt_hunger();
    /* Various */
    prt_blind();
    prt_confused();
    prt_afraid();
    prt_poisoned();
    prt_dtrap();
    /* State */
    prt_state();
    /* Speed */
    prt_speed();
    /* Study spells */
    prt_study();
}
/*
 * Hack -- display inventory in sub-windows
 */
unsafe extern "C" fn fix_inven() {
    let mut j: libc::c_int = 0;
    /* Scan windows */
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        let mut old: *mut term = Term;
        /* No window */
        if !angband_term[j as usize].is_null() {
            /* No relevant flags */
            if !(window_flag[j as usize] as libc::c_long & 0x1 as libc::c_long
                     == 0) {
                /* Activate */
                Term_activate(angband_term[j as usize]);
                /* Display inventory */
                display_inven();
                /* Fresh */
                Term_fresh();
                /* Restore */
                Term_activate(old);
            }
        }
        j += 1
    };
}
/*
 * Hack -- display equipment in sub-windows
 */
unsafe extern "C" fn fix_equip() {
    let mut j: libc::c_int = 0;
    /* Scan windows */
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        let mut old: *mut term = Term;
        /* No window */
        if !angband_term[j as usize].is_null() {
            /* No relevant flags */
            if !(window_flag[j as usize] as libc::c_long & 0x2 as libc::c_long
                     == 0) {
                /* Activate */
                Term_activate(angband_term[j as usize]);
                /* Display equipment */
                display_equip();
                /* Fresh */
                Term_fresh();
                /* Restore */
                Term_activate(old);
            }
        }
        j += 1
    };
}
/*
 * Hack -- display character in sub-windows
 */
unsafe extern "C" fn fix_player() {
    let mut j: libc::c_int = 0;
    /* Scan windows */
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        let mut old: *mut term = Term;
        /* No window */
        if !angband_term[j as usize].is_null() {
            /* No relevant flags */
            if !(window_flag[j as usize] as libc::c_long & 0x8 as libc::c_long
                     == 0) {
                /* Activate */
                Term_activate(angband_term[j as usize]);
                /* Display player */
                display_player(0 as libc::c_int);
                /* Fresh */
                Term_fresh();
                /* Restore */
                Term_activate(old);
            }
        }
        j += 1
    };
}
/*
 * Hack -- display recent messages in sub-windows
 *
 * XXX XXX XXX Adjust for width and split messages
 */
#[no_mangle]
pub unsafe extern "C" fn fix_message() {
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    /* Scan windows */
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        let mut old: *mut term = Term;
        /* No window */
        if !angband_term[j as usize].is_null() {
            /* No relevant flags */
            if !(window_flag[j as usize] as libc::c_long &
                     0x40 as libc::c_long == 0) {
                /* Activate */
                Term_activate(angband_term[j as usize]);
                /* Get size */
                Term_get_size(&mut w, &mut h);
                /* Dump messages */
                i = 0 as libc::c_int;
                while i < h {
                    /* Dump the message on the appropriate line */
                    display_message(0 as libc::c_int,
                                    h - 1 as libc::c_int - i,
                                    strlen(message_str(i as s16b as
                                                           libc::c_int)) as
                                        libc::c_int,
                                    message_color(i as s16b as libc::c_int),
                                    message_str(i as s16b as libc::c_int));
                    /* Cursor */
                    Term_locate(&mut x, &mut y);
                    /* Clear to end of line */
                    Term_erase(x, y, 255 as libc::c_int);
                    i += 1
                }
                /* Fresh */
                Term_fresh();
                /* Restore */
                Term_activate(old);
            }
        }
        j += 1
    };
}
/*
 * Hack -- display overhead view in sub-windows
 *
 * Note that the "player" symbol does NOT appear on the map.
 */
unsafe extern "C" fn fix_overhead() {
    let mut j: libc::c_int = 0;
    let mut cy: libc::c_int = 0;
    let mut cx: libc::c_int = 0;
    /* Scan windows */
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        let mut old: *mut term = Term;
        /* No window */
        if !angband_term[j as usize].is_null() {
            /* No relevant flags */
            if !(window_flag[j as usize] as libc::c_long &
                     0x80 as libc::c_long == 0) {
                /* Activate */
                Term_activate(angband_term[j as usize]);
                /* Redraw map */
                display_map(&mut cy, &mut cx);
                /* Fresh */
                Term_fresh();
                /* Restore */
                Term_activate(old);
            }
        }
        j += 1
    };
}
/*
 * Hack -- display monster recall in sub-windows
 */
unsafe extern "C" fn fix_monster() {
    let mut j: libc::c_int = 0;
    /* Scan windows */
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        let mut old: *mut term = Term;
        /* No window */
        if !angband_term[j as usize].is_null() {
            /* No relevant flags */
            if !(window_flag[j as usize] as libc::c_long &
                     0x100 as libc::c_long == 0) {
                /* Activate */
                Term_activate(angband_term[j as usize]);
                /* Display monster race info */
                if monster_race_idx != 0 {
                    display_roff(monster_race_idx as libc::c_int,
                                 monster_ego_idx as libc::c_int);
                }
                /* Fresh */
                Term_fresh();
                /* Restore */
                Term_activate(old);
            }
        }
        j += 1
    };
}
/*
 * Hack -- display object recall in sub-windows
 */
unsafe extern "C" fn fix_object() {
    let mut j: libc::c_int = 0;
    /* Scan windows */
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        let mut old: *mut term = Term;
        /* No window */
        if !angband_term[j as usize].is_null() {
            /* No relevant flags */
            if !(window_flag[j as usize] as libc::c_long &
                     0x200 as libc::c_long == 0) {
                /* Activate */
                Term_activate(angband_term[j as usize]);
                /* Clear */
                Term_clear();
                /* Display object info */
                if !tracked_object.is_null() {
                    if object_out_desc(tracked_object, 0 as *mut FILE,
                                       0 as libc::c_int as bool_,
                                       0 as libc::c_int as bool_) == 0 {
                        text_out(b"You see nothing special.\x00" as *const u8
                                     as *const libc::c_char);
                    }
                }
                /* Fresh */
                Term_fresh();
                /* Restore */
                Term_activate(old);
            }
        }
        j += 1
    };
}
/* Show the monster list in a window */
unsafe extern "C" fn fix_m_list() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    /* Scan windows */
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        let mut old: *mut term = Term;
        let mut c: libc::c_int = 0 as libc::c_int;
        /* No window */
        if !angband_term[j as usize].is_null() {
            /* No relevant flags */
            if !(window_flag[j as usize] as libc::c_long &
                     0x10 as libc::c_long == 0) {
                /* Activate */
                Term_activate(angband_term[j as usize]);
                /* Clear */
                Term_clear();
                /* Hallucination */
                if (*p_ptr).image != 0 {
                    c_prt(1 as libc::c_int as byte_hack,
                          b"You can not see clearly\x00" as *const u8 as
                              *const libc::c_char, 0 as libc::c_int,
                          0 as libc::c_int);
                    /* Fresh */
                    Term_fresh();
                    /* Restore */
                    Term_activate(old);
                    return
                }
                /* reset visible count */
                i = 1 as libc::c_int;
                while i < max_r_idx as libc::c_int {
                    let mut r_ptr: *mut monster_race =
                        &mut *r_info.offset(i as isize) as *mut monster_race;
                    (*r_ptr).total_visible = 0 as libc::c_int as byte_hack;
                    i += 1
                }
                let mut current_block_16: u64;
                /* Count up the number visible in each race */
                i = 1 as libc::c_int;
                while i < m_max as libc::c_int {
                    let mut m_ptr: *mut monster_type =
                        &mut *m_list.offset(i as isize) as *mut monster_type;
                    let mut r_ptr_0: *mut monster_race =
                        &mut *r_info.offset((*m_ptr).r_idx as isize) as
                            *mut monster_race;
                    /* Skip dead monsters */
                    if !((*m_ptr).hp < 0 as libc::c_int) {
                        /* Skip unseen monsters */
                        if (*r_ptr_0).flags9 &
                               0x8 as libc::c_int as libc::c_uint != 0 {
                            let mut o_ptr: *mut object_type =
                                0 as *mut object_type;
                            /* Acquire object */
                            o_ptr =
                                &mut *o_list.offset((*m_ptr).hold_o_idx as
                                                        isize) as
                                    *mut object_type;
                            /* Memorized objects */
                            if (*o_ptr).marked == 0 {
                                current_block_16 = 6009453772311597924;
                            } else {
                                current_block_16 = 15925075030174552612;
                            }
                        } else if (*m_ptr).ml == 0 {
                            current_block_16 = 6009453772311597924;
                        } else { current_block_16 = 15925075030174552612; }
                        match current_block_16 {
                            6009453772311597924 => { }
                            _ => {
                                /* Increase for this race */
                                (*r_ptr_0).total_visible =
                                    (*r_ptr_0).total_visible.wrapping_add(1);
                                /* Increase total Count */
                                c += 1
                            }
                        }
                    }
                    i += 1
                }
                /* Are monsters visible? */
                if c != 0 {
                    let mut w: libc::c_int = 0;
                    let mut h: libc::c_int = 0;
                    let mut num: libc::c_int = 0 as libc::c_int;
                    Term_get_size(&mut w, &mut h);
                    c_prt(1 as libc::c_int as byte_hack,
                          format(b"You can see %d monster%s\x00" as *const u8
                                     as *const libc::c_char, c,
                                 if c > 1 as libc::c_int {
                                     b"s:\x00" as *const u8 as
                                         *const libc::c_char
                                 } else {
                                     b":\x00" as *const u8 as
                                         *const libc::c_char
                                 }) as cptr, 0 as libc::c_int,
                          0 as libc::c_int);
                    i = 1 as libc::c_int;
                    while i < max_r_idx as libc::c_int {
                        let mut r_ptr_1: *mut monster_race =
                            &mut *r_info.offset(i as isize) as
                                *mut monster_race;
                        /* Default Colour */
                        let mut attr: byte_hack =
                            2 as libc::c_int as byte_hack;
                        /* Only visible monsters */
                        if !((*r_ptr_1).total_visible == 0) {
                            /* Uniques */
                            if (*r_ptr_1).flags1 &
                                   0x1 as libc::c_int as libc::c_uint != 0 {
                                attr = 14 as libc::c_int as byte_hack
                            }
                            /* Have we ever killed one? */
                            if (*r_ptr_1).r_tkills != 0 {
                                if (*r_ptr_1).level as libc::c_int >
                                       dun_level as libc::c_int {
                                    attr = 10 as libc::c_int as byte_hack;
                                    if (*r_ptr_1).flags1 &
                                           0x1 as libc::c_int as libc::c_uint
                                           != 0 {
                                        attr = 4 as libc::c_int as byte_hack
                                    }
                                }
                            } else if (*r_ptr_1).flags1 &
                                          0x1 as libc::c_int as libc::c_uint
                                          == 0 {
                                attr = 5 as libc::c_int as byte_hack
                            }
                            /* Dump the monster name */
                            if (*r_ptr_1).total_visible as libc::c_int ==
                                   1 as libc::c_int {
                                c_prt(attr,
                                      r_name.offset((*r_ptr_1).name as isize)
                                          as cptr,
                                      num % (h - 1 as libc::c_int) +
                                          1 as libc::c_int,
                                      num / (h - 1 as libc::c_int) *
                                          26 as libc::c_int);
                            } else {
                                c_prt(attr,
                                      format(b"%s (x%d)\x00" as *const u8 as
                                                 *const libc::c_char,
                                             r_name.offset((*r_ptr_1).name as
                                                               isize),
                                             (*r_ptr_1).total_visible as
                                                 libc::c_int) as cptr,
                                      num % (h - 1 as libc::c_int) +
                                          1 as libc::c_int,
                                      num / (h - 1 as libc::c_int) *
                                          26 as libc::c_int);
                            }
                            num += 1
                        }
                        i += 1
                    }
                } else {
                    c_prt(1 as libc::c_int as byte_hack,
                          b"You see no monsters.\x00" as *const u8 as
                              *const libc::c_char, 0 as libc::c_int,
                          0 as libc::c_int);
                }
                /* Fresh */
                Term_fresh();
                /* Restore */
                Term_activate(old);
            }
        }
        j += 1
    };
}
/*
 * Calculate number of spells player should have, and forget,
 * or remember, spells until that number is properly reflected.
 *
 * Note that this function induces various "status" messages,
 * which must be bypasses until the character is created.
 */
unsafe extern "C" fn calc_spells() {
    (*p_ptr).new_spells = 0 as libc::c_int as s16b;
}
/* Ugly hack */
#[no_mangle]
pub static mut calc_powers_silent: bool_ = 0 as libc::c_int as bool_;
/* Calc the player powers */
unsafe extern "C" fn calc_powers() {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 0 as libc::c_int;
    let mut old_powers: *mut bool_ = 0 as *mut bool_;
    /* Hack -- wait for creation */
    if character_generated == 0 { return }
    /* Hack -- handle "xtra" mode */
    if character_xtra != 0 { return }
    old_powers =
        memset(ralloc((power_max as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (power_max as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                    as libc::c_ulong)) as
            *mut bool_;
    /* Save old powers */
    i = 0 as libc::c_int;
    while i < power_max as libc::c_int {
        *old_powers.offset(i as isize) = *(*p_ptr).powers.offset(i as isize);
        i += 1
    }
    /* Get intrinsincs */
    i = 0 as libc::c_int;
    while i < 62 as libc::c_int {
        *(*p_ptr).powers.offset(i as isize) = (*p_ptr).powers_mod[i as usize];
        i += 1
    }
    while i < power_max as libc::c_int {
        *(*p_ptr).powers.offset(i as isize) = 0 as libc::c_int as bool_;
        i += 1
    }
    /* Hooked powers */
    process_hooks(30 as libc::c_int,
                  b"()\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char);
    /* Add objects powers */
    i = 24 as libc::c_int;
    while i < 52 as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        if !((*o_ptr).k_idx == 0) {
            p = object_power(o_ptr);
            if p != -(1 as libc::c_int) {
                *(*p_ptr).powers.offset(p as isize) =
                    1 as libc::c_int as bool_
            }
        }
        i += 1
    }
    if (*p_ptr).tim_mimic == 0 && (*p_ptr).body_monster == 0 {
        /* Add in racial and subracial powers */
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            p = (*rp_ptr).powers[i as usize] as libc::c_int;
            if p != -(1 as libc::c_int) {
                *(*p_ptr).powers.offset(p as isize) =
                    1 as libc::c_int as bool_
            }
            p = (*rmp_ptr).powers[i as usize] as libc::c_int;
            if p != -(1 as libc::c_int) {
                *(*p_ptr).powers.offset(p as isize) =
                    1 as libc::c_int as bool_
            }
            i += 1
        }
    } else if (*p_ptr).mimic_form != 0 {
        call_lua(b"calc_mimic_power\x00" as *const u8 as *const libc::c_char,
                 b"(d)\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char,
                 (*p_ptr).mimic_form as libc::c_int);
    }
    /* Add in class powers */
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        p = (*cp_ptr).powers[i as usize] as libc::c_int;
        if p != -(1 as libc::c_int) {
            *(*p_ptr).powers.offset(p as isize) = 1 as libc::c_int as bool_
        }
        i += 1
    }
    if (*p_ptr).disembodied != 0 {
        p = 54 as libc::c_int;
        *(*p_ptr).powers.offset(p as isize) = 1 as libc::c_int as bool_
    }
    /* Now lets warn the player */
    i = 0 as libc::c_int;
    while i < power_max as libc::c_int {
        let mut old: s32b = *old_powers.offset(i as isize) as s32b;
        let mut new_: s32b = *(*p_ptr).powers.offset(i as isize) as s32b;
        if new_ > old {
            if calc_powers_silent == 0 {
                cmsg_print(5 as libc::c_int as byte_hack,
                           (*powers_type.offset(i as isize)).gain_text as
                               cptr);
            }
        } else if new_ < old {
            if calc_powers_silent == 0 {
                cmsg_print(4 as libc::c_int as byte_hack,
                           (*powers_type.offset(i as isize)).lose_text as
                               cptr);
            }
        }
        i += 1
    }
    calc_powers_silent = 0 as libc::c_int as bool_;
    rnfree(old_powers as vptr,
           (power_max as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>() as
                                                libc::c_ulong));
}
/*
 * Calculate the player's sanity
 */
#[no_mangle]
pub unsafe extern "C" fn calc_sanity() {
    let mut bonus: libc::c_int = 0;
    let mut msane: libc::c_int = 0;
    /* Hack -- use the con/hp table for sanity/wis */
    bonus =
        *adj_con_mhp.as_mut_ptr().offset((*p_ptr).stat_ind[2 as libc::c_int as
                                                               usize] as
                                             isize) as libc::c_int -
            128 as libc::c_int;
    /* Hack -- assume 5 sanity points per level. */
    msane =
        5 as libc::c_int * ((*p_ptr).lev as libc::c_int + 1 as libc::c_int) +
            bonus * (*p_ptr).lev as libc::c_int / 2 as libc::c_int;
    if msane < (*p_ptr).lev as libc::c_int + 1 as libc::c_int {
        msane = (*p_ptr).lev as libc::c_int + 1 as libc::c_int
    }
    if (*p_ptr).msane as libc::c_int != msane {
        /* Sanity carries over between levels. */
        (*p_ptr).csane =
            ((*p_ptr).csane as libc::c_int +
                 (msane - (*p_ptr).msane as libc::c_int)) as s16b;
        (*p_ptr).msane = msane as s16b;
        if (*p_ptr).csane as libc::c_int >= msane {
            (*p_ptr).csane = msane as s16b;
            (*p_ptr).csane_frac = 0 as libc::c_int as u16b
        }
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x800000 as libc::c_long) as
                u32b;
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b
    };
}
/*
 * Calculate maximum mana.  You do not need to know any spells.
 * Note that mana is lowered by heavy (or inappropriate) armor.
 *
 * This function induces status messages.
 */
unsafe extern "C" fn calc_mana() {
    let mut msp: libc::c_int = 0;
    let mut levels: libc::c_int = 0;
    let mut cur_wgt: libc::c_int = 0;
    let mut max_wgt: libc::c_int = 0;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    levels = (*p_ptr).lev as libc::c_int;
    /* Hack -- no negative mana */
    if levels < 0 as libc::c_int { levels = 0 as libc::c_int }
    /* Extract total mana */
    msp =
        get_skill_scale(15 as libc::c_int, 200 as libc::c_int as u32b) as
            libc::c_int +
            *adj_mag_mana.as_mut_ptr().offset((if (*p_ptr).stat_ind[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                                      as libc::c_int >
                                                      (*p_ptr).stat_ind[2 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                          as libc::c_int {
                                                   (*p_ptr).stat_ind[1 as
                                                                         libc::c_int
                                                                         as
                                                                         usize]
                                                       as libc::c_int
                                               } else {
                                                   (*p_ptr).stat_ind[2 as
                                                                         libc::c_int
                                                                         as
                                                                         usize]
                                                       as libc::c_int
                                               }) as isize) as libc::c_int *
                levels / 4 as libc::c_int;
    /* Hack -- usually add one mana */
    if msp != 0 { msp += 1 }
    /* Possessors mana is different */
    if (*p_ptr).body_monster as libc::c_int != 0 && (*p_ptr).disembodied == 0
       {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset((*p_ptr).body_monster as isize) as
                *mut monster_race;
        let mut f: libc::c_int =
            100 as libc::c_int /
                (if (*r_ptr).freq_spell as libc::c_int != 0 {
                     (*r_ptr).freq_spell as libc::c_int
                 } else { 1 as libc::c_int });
        msp = 21 as libc::c_int - f;
        if msp < 1 as libc::c_int { msp = 1 as libc::c_int }
    }
    /* Apply race mod mana */
    msp = msp * (*rmp_ptr).mana as libc::c_int / 100 as libc::c_int;
    /* Apply class mana */
    msp += msp * (*cp_ptr).mana as libc::c_int / 100 as libc::c_int;
    /* Apply Eru mana */
    if (*p_ptr).pgod as libc::c_int == 1 as libc::c_int {
        let mut tmp: s32b = (*p_ptr).grace;
        if tmp >= 35000 as libc::c_int { tmp = 35000 as libc::c_int }
        tmp /= 100 as libc::c_int;
        msp += msp * tmp / 1000 as libc::c_int
    }
    /* Only mages are affected */
    if forbid_gloves() != 0 {
        /* Assume player is not encumbered by gloves */
        (*p_ptr).cumber_glove = 0 as libc::c_int as bool_;
        /* Get the gloves */
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(44 as libc::c_int as
                                                             isize) as
                *mut object_type;
        /* Examine the gloves */
        object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                     &mut esp);
        /* Normal gloves hurt mage-type spells */
        if (*o_ptr).k_idx as libc::c_int != 0 &&
               f2 as libc::c_long & 0x4000 as libc::c_long == 0 &&
               !(f1 as libc::c_long & 0x8 as libc::c_long != 0 &&
                     (*o_ptr).pval > 0 as libc::c_int) &&
               f5 as libc::c_long & 0x800 as libc::c_long == 0 {
            /* Encumbered */
            (*p_ptr).cumber_glove = 1 as libc::c_int as bool_;
            /* Reduce mana */
            msp = 3 as libc::c_int * msp / 4 as libc::c_int
        }
    }
    /* Augment mana */
    if munchkin_multipliers != 0 {
        if (*p_ptr).to_m != 0 {
            msp += msp * (*p_ptr).to_m as libc::c_int / 5 as libc::c_int
        }
    } else if (*p_ptr).to_m != 0 {
        msp += msp * (*p_ptr).to_m as libc::c_int / 10 as libc::c_int
    }
    /* Assume player not encumbered by armor */
    (*p_ptr).cumber_armor = 0 as libc::c_int as bool_;
    /* Weigh the armor */
    cur_wgt = 0 as libc::c_int;
    cur_wgt += (*p_ptr).inventory[37 as libc::c_int as usize].weight;
    cur_wgt += (*p_ptr).inventory[42 as libc::c_int as usize].weight;
    cur_wgt += (*p_ptr).inventory[39 as libc::c_int as usize].weight;
    cur_wgt += (*p_ptr).inventory[38 as libc::c_int as usize].weight;
    cur_wgt += (*p_ptr).inventory[44 as libc::c_int as usize].weight;
    cur_wgt += (*p_ptr).inventory[47 as libc::c_int as usize].weight;
    /* Determine the weight allowance */
    max_wgt =
        200 as libc::c_int +
            get_skill_scale(16 as libc::c_int, 500 as libc::c_int as u32b) as
                libc::c_int;
    /* Heavy armor penalizes mana */
    if (cur_wgt - max_wgt) / 10 as libc::c_int > 0 as libc::c_int {
        /* Encumbered */
        (*p_ptr).cumber_armor = 1 as libc::c_int as bool_;
        /* Reduce mana */
        msp -= (cur_wgt - max_wgt) / 10 as libc::c_int
    }
    /* When meditating your mana is increased ! */
    if (*p_ptr).meditation != 0 {
        msp += 50 as libc::c_int;
        (*p_ptr).csp =
            ((*p_ptr).csp as libc::c_int + 50 as libc::c_int) as s16b
    }
    /* Sp mods? */
    if process_hooks_ret(60 as libc::c_int,
                         b"d\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char,
                         b"(d)\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char, msp) != 0 {
        msp = process_hooks_return[0 as libc::c_int as usize].num
    }
    /* Mana can never be negative */
    if msp < 0 as libc::c_int { msp = 0 as libc::c_int }
    /* Maximum mana has changed */
    if (*p_ptr).msp as libc::c_int != msp {
        /* Save new limit */
        (*p_ptr).msp = msp as s16b;
        /* Enforce new limit */
        if (*p_ptr).csp as libc::c_int >= msp {
            (*p_ptr).csp = msp as s16b;
            (*p_ptr).csp_frac = 0 as libc::c_int as u16b
        }
        /* Display mana later */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b
    }
    /* Hack -- handle "xtra" mode */
    if character_xtra != 0 { return }
    /* Take note when "glove state" changes */
    if (*p_ptr).old_cumber_glove as libc::c_int !=
           (*p_ptr).cumber_glove as libc::c_int {
        /* Message */
        if (*p_ptr).cumber_glove != 0 {
            msg_print(b"Your covered hands feel unsuitable for spellcasting.\x00"
                          as *const u8 as *const libc::c_char);
        } else {
            msg_print(b"Your hands feel more suitable for spellcasting.\x00"
                          as *const u8 as *const libc::c_char);
        }
        /* Save it */
        (*p_ptr).old_cumber_glove = (*p_ptr).cumber_glove
    }
    /* Take note when "armor state" changes */
    if (*p_ptr).old_cumber_armor as libc::c_int !=
           (*p_ptr).cumber_armor as libc::c_int {
        /* Message */
        if (*p_ptr).cumber_armor != 0 {
            msg_print(b"The weight of your armor encumbers your movement.\x00"
                          as *const u8 as *const libc::c_char);
        } else {
            msg_print(b"You feel able to move more freely.\x00" as *const u8
                          as *const libc::c_char);
        }
        /* Save it */
        (*p_ptr).old_cumber_armor = (*p_ptr).cumber_armor
    };
}
/*
 * Calculate the players (maximal) hit points
 * Adjust current hitpoints if necessary
 */
#[no_mangle]
pub unsafe extern "C" fn calc_hitpoints() {
    let mut bonus: libc::c_int = 0;
    let mut mhp: libc::c_int = 0;
    /* Un-inflate "half-hitpoint bonus per level" value */
    bonus =
        *adj_con_mhp.as_mut_ptr().offset((*p_ptr).stat_ind[4 as libc::c_int as
                                                               usize] as
                                             isize) as libc::c_int -
            128 as libc::c_int;
    /* Calculate hitpoints */
    mhp =
        player_hp[((*p_ptr).lev as libc::c_int - 1 as libc::c_int) as usize]
            as libc::c_int +
            bonus * (*p_ptr).lev as libc::c_int / 2 as libc::c_int;
    /* Always have at least one hitpoint per level */
    if mhp < (*p_ptr).lev as libc::c_int + 1 as libc::c_int {
        mhp = (*p_ptr).lev as libc::c_int + 1 as libc::c_int
    }
    /* Factor in the pernament hp modifications */
    mhp += (*p_ptr).hp_mod as libc::c_int;
    if mhp < 1 as libc::c_int { mhp = 1 as libc::c_int }
    /* Hack: Sorcery impose a hp penality */
    if mhp != 0 && get_skill(41 as libc::c_int) as libc::c_int != 0 {
        mhp -=
            mhp *
                get_skill_scale(41 as libc::c_int, 50 as libc::c_int as u32b)
                    as libc::c_int / 100 as libc::c_int;
        if mhp < 1 as libc::c_int { mhp = 1 as libc::c_int }
    }
    /* Factor in the melkor hp modifications */
    if (*p_ptr).pgod as libc::c_int == 4 as libc::c_int {
        mhp -= (*p_ptr).melkor_sacrifice as libc::c_int * 10 as libc::c_int;
        if mhp < 1 as libc::c_int { mhp = 1 as libc::c_int }
    }
    /* Factor in the hero / superhero settings */
    if (*p_ptr).hero != 0 { mhp += 10 as libc::c_int }
    if (*p_ptr).shero != 0 { mhp += 30 as libc::c_int }
    /* Augment Hitpoint */
    if munchkin_multipliers != 0 {
        mhp += mhp * (*p_ptr).to_l as libc::c_int / 5 as libc::c_int
    } else { mhp += mhp * (*p_ptr).to_l as libc::c_int / 10 as libc::c_int }
    if mhp < 1 as libc::c_int { mhp = 1 as libc::c_int }
    if (*p_ptr).body_monster != 0 {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset((*p_ptr).body_monster as isize) as
                *mut monster_race;
        let mut rhp: u32b =
            maxroll((*r_ptr).hdice as s16b, (*r_ptr).hside as s16b) as u32b;
        /* Adjust the hp with the possession skill */
        rhp =
            rhp.wrapping_mul((20 as libc::c_int +
                                  get_skill_scale(50 as libc::c_int,
                                                  80 as libc::c_int as u32b)
                                      as libc::c_int) as
                                 libc::c_uint).wrapping_div(100 as libc::c_int
                                                                as
                                                                libc::c_uint);
        mhp =
            rhp.wrapping_add(sroot(rhp as s32b) as
                                 libc::c_uint).wrapping_add(mhp as
                                                                libc::c_uint).wrapping_div(3
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint)
                as libc::c_int
    }
    if (*p_ptr).disembodied != 0 { mhp = 1 as libc::c_int }
    /* HACK - being undead means less DP */
    if (*p_ptr).necro_extra & 0x8 as libc::c_int as libc::c_uint != 0 {
        let mut divisor: libc::c_int =
            (*p_ptr).lev as libc::c_int / 4 as libc::c_int;
        /* Beware of the horrible division by zero ! :) */
        if divisor == 0 as libc::c_int { divisor = 1 as libc::c_int }
        /* Actually decrease the max hp */
        mhp /= divisor;
        /* Never less than 1 */
        if mhp < 1 as libc::c_int { mhp = 1 as libc::c_int }
    }
    /* Hp mods? */
    if process_hooks_ret(57 as libc::c_int,
                         b"d\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char,
                         b"(d)\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char, mhp) != 0 {
        mhp = process_hooks_return[0 as libc::c_int as usize].num
    }
    /* Never less than 1 */
    if mhp < 1 as libc::c_int { mhp = 1 as libc::c_int }
    /* New maximum hitpoints */
    if (*p_ptr).mhp as libc::c_int != mhp {
        /* XXX XXX XXX New hitpoint maintenance */
        /* Enforce maximum */
        if (*p_ptr).chp as libc::c_int >= mhp {
            (*p_ptr).chp = mhp as s16b;
            (*p_ptr).chp_frac = 0 as libc::c_int as u16b
        }
        /* Save the new max-hitpoints */
        (*p_ptr).mhp = mhp as s16b;
        /* Display hitpoints (later) */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x40 as libc::c_long) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b
    };
}
/*
 * Extract and set the current "lite radius"
 *
 * SWD: Experimental modification: multiple light sources have additive effect.
 *
 */
unsafe extern "C" fn calc_torch() {
    let mut i: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    /* Assume no light */
    (*p_ptr).cur_lite = 0 as libc::c_int as s16b;
    /* Loop through all wielded items */
    i = 24 as libc::c_int;
    while i < 52 as libc::c_int {
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Skip empty slots */
        if !((*o_ptr).k_idx == 0) {
            /* Extract the flags */
            object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                         &mut esp);
            /* does this item glow? */
            if f4 as libc::c_long & 0x10000000 as libc::c_long != 0 &&
                   (*o_ptr).timeout as libc::c_int > 0 as libc::c_int ||
                   f4 as libc::c_long & 0x10000000 as libc::c_long == 0 {
                if f3 as libc::c_long & 0x2000 as libc::c_long != 0 {
                    (*p_ptr).cur_lite += 1
                }
                if f4 as libc::c_long & 0x4000000 as libc::c_long != 0 {
                    (*p_ptr).cur_lite =
                        ((*p_ptr).cur_lite as libc::c_int + 2 as libc::c_int)
                            as s16b
                }
                if f4 as libc::c_long & 0x8000000 as libc::c_long != 0 {
                    (*p_ptr).cur_lite =
                        ((*p_ptr).cur_lite as libc::c_int + 3 as libc::c_int)
                            as s16b
                }
            }
        }
        i += 1
    }
    if (*p_ptr).tim_lite != 0 {
        (*p_ptr).cur_lite =
            ((*p_ptr).cur_lite as libc::c_int + 2 as libc::c_int) as s16b
    }
    if (*p_ptr).holy != 0 {
        (*p_ptr).cur_lite =
            ((*p_ptr).cur_lite as libc::c_int + 1 as libc::c_int) as s16b
    }
    /* max radius is 5 without rewriting other code -- */
	/* see cave.c:update_lite() and defines.h:LITE_MAX */
    if (*p_ptr).cur_lite as libc::c_int > 5 as libc::c_int {
        (*p_ptr).cur_lite = 5 as libc::c_int as s16b
    }
    /* check if the player doesn't have a lite source, */
	/* but does glow as an intrinsic.                  */
    if (*p_ptr).cur_lite as libc::c_int == 0 as libc::c_int &&
           (*p_ptr).lite as libc::c_int != 0 {
        (*p_ptr).cur_lite = 1 as libc::c_int as s16b
    }
    /* Hooked powers */
    process_hooks(69 as libc::c_int,
                  b"()\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char);
    /* end experimental mods */
    /* Reduce lite in the small-scale wilderness map */
    if (*p_ptr).wild_mode != 0 {
        /* Reduce the lite radius if needed */
        if (*p_ptr).cur_lite as libc::c_int > 3 as libc::c_int {
            (*p_ptr).cur_lite = 3 as libc::c_int as s16b
        }
    }
    /* Reduce lite when running if requested */
    if running as libc::c_int != 0 && view_reduce_lite as libc::c_int != 0 {
        /* Reduce the lite radius if needed */
        if (*p_ptr).cur_lite as libc::c_int > 1 as libc::c_int {
            (*p_ptr).cur_lite = 1 as libc::c_int as s16b
        }
    }
    /* Notice changes in the "lite radius" */
    if (*p_ptr).old_lite as libc::c_int != (*p_ptr).cur_lite as libc::c_int {
        /* Update the view */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x100000 as libc::c_long) as
                u32b;
        /* Update the monsters */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long) as
                u32b;
        /* Remember the old lite */
        (*p_ptr).old_lite = (*p_ptr).cur_lite
    };
}
/*
 * Computes current weight limit.
 */
#[no_mangle]
pub unsafe extern "C" fn weight_limit() -> libc::c_int {
    let mut i: libc::c_int = 0;
    /* Weight limit based only on strength */
    i =
        *adj_str_wgt.as_mut_ptr().offset((*p_ptr).stat_ind[0 as libc::c_int as
                                                               usize] as
                                             isize) as libc::c_int *
            100 as libc::c_int;
    if process_hooks_ret(74 as libc::c_int,
                         b"d\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char,
                         b"(d)\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char, i) != 0 {
        i = process_hooks_return[0 as libc::c_int as usize].num
    }
    /* Return the result */
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn calc_wield_monster() {
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
    /* Get the carried monster */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(49 as libc::c_int as
                                                         isize) as
            *mut object_type;
    if (*o_ptr).k_idx != 0 {
        r_ptr =
            &mut *r_info.offset((*o_ptr).pval as isize) as *mut monster_race;
        if (*r_ptr).flags2 & 0x10 as libc::c_int as libc::c_uint != 0 {
            (*p_ptr).invis =
                ((*p_ptr).invis as libc::c_int + 20 as libc::c_int) as s16b
        }
        if (*r_ptr).flags2 & 0x8 as libc::c_int as libc::c_uint != 0 {
            (*p_ptr).reflect = 1 as libc::c_int as bool_
        }
        if (*r_ptr).flags7 & 0x4 as libc::c_int as libc::c_uint != 0 {
            (*p_ptr).ffall = 1 as libc::c_int as bool_
        }
        if (*r_ptr).flags7 & 0x1 as libc::c_int as libc::c_uint != 0 {
            (*p_ptr).water_breath = 1 as libc::c_int as bool_
        }
    };
}
/*
 * Calc which body parts the player have, based on the
 * monster he incarnate, note that that's bnot a hack
 * since body parts of the player when in it's own body
 * are also defined in r_info(monster 0)
 */
#[no_mangle]
pub unsafe extern "C" fn calc_body() {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*p_ptr).body_monster as isize) as
            *mut monster_race;
    let mut i: libc::c_int = 0;
    let mut b_weapon: libc::c_int = 0;
    let mut b_legs: libc::c_int = 0;
    let mut b_arms: libc::c_int = 0;
    let mut body_parts: *mut byte_hack = 0 as *mut byte_hack;
    let mut bp: [byte_hack; 6] = [0; 6];
    if (*p_ptr).body_monster == 0 {
        body_parts = bp.as_mut_ptr();
        i = 0 as libc::c_int;
        while i < 6 as libc::c_int {
            let mut b: libc::c_int = 0;
            b =
                (*rp_ptr).body_parts[i as usize] as libc::c_int +
                    (*rmp_ptr).body_parts[i as usize] as libc::c_int;
            if b < 0 as libc::c_int { b = 0 as libc::c_int }
            if (*p_ptr).mimic_form as libc::c_int ==
                   resolve_mimic_name(b"Bear\x00" as *const u8 as
                                          *const libc::c_char) {
                if i == 2 as libc::c_int {
                    b = 0 as libc::c_int
                } else if i == 5 as libc::c_int { b = 0 as libc::c_int }
            }
            bp[i as usize] = b as byte_hack;
            i += 1
        }
    } else {
        body_parts = bp.as_mut_ptr();
        i = 0 as libc::c_int;
        while i < 6 as libc::c_int {
            let mut b_0: libc::c_int = 0;
            b_0 = (*r_ptr).body_parts[i as usize] as libc::c_int;
            if b_0 < 0 as libc::c_int { b_0 = 0 as libc::c_int }
            bp[i as usize] = b_0 as byte_hack;
            i += 1
        }
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        let mut b_1: libc::c_int = 0;
        b_1 =
            bp[i as usize] as libc::c_int +
                (*cp_ptr).body_parts[i as usize] as libc::c_int;
        if b_1 < 0 as libc::c_int { b_1 = 0 as libc::c_int }
        if b_1 > max_body_part[i as usize] { b_1 = max_body_part[i as usize] }
        bp[i as usize] = b_1 as byte_hack;
        i += 1
    }
    b_weapon = *body_parts.offset(0 as libc::c_int as isize) as libc::c_int;
    b_arms = *body_parts.offset(2 as libc::c_int as isize) as libc::c_int;
    b_legs = *body_parts.offset(5 as libc::c_int as isize) as libc::c_int;
    if (*p_ptr).mimic_extra & 0x40 as libc::c_int as libc::c_uint != 0 {
        b_weapon += 1;
        b_arms += 1;
        if b_weapon > 3 as libc::c_int { b_weapon = 3 as libc::c_int }
        if b_arms > 3 as libc::c_int { b_arms = 3 as libc::c_int }
    }
    if (*p_ptr).mimic_extra & 0x20 as libc::c_int as libc::c_uint != 0 {
        b_legs += 1;
        if b_legs > 2 as libc::c_int { b_legs = 2 as libc::c_int }
    }
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int - 24 as libc::c_int {
        (*p_ptr).body_parts[i as usize] = 0 as libc::c_int as byte_hack;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < b_weapon {
        (*p_ptr).body_parts[(24 as libc::c_int - 24 as libc::c_int + i) as
                                usize] = 24 as libc::c_int as byte_hack;
        i += 1
    }
    if *body_parts.offset(0 as libc::c_int as isize) != 0 {
        (*p_ptr).body_parts[(27 as libc::c_int - 24 as libc::c_int) as usize]
            = 27 as libc::c_int as byte_hack
    }
    i = 0 as libc::c_int;
    while i < *body_parts.offset(1 as libc::c_int as isize) as libc::c_int {
        (*p_ptr).body_parts[(37 as libc::c_int - 24 as libc::c_int + i) as
                                usize] = 37 as libc::c_int as byte_hack;
        (*p_ptr).body_parts[(38 as libc::c_int - 24 as libc::c_int + i) as
                                usize] = 38 as libc::c_int as byte_hack;
        (*p_ptr).body_parts[(36 as libc::c_int - 24 as libc::c_int + i) as
                                usize] = 36 as libc::c_int as byte_hack;
        (*p_ptr).body_parts[(50 as libc::c_int - 24 as libc::c_int + i) as
                                usize] = 50 as libc::c_int as byte_hack;
        (*p_ptr).body_parts[(49 as libc::c_int - 24 as libc::c_int + i) as
                                usize] = 49 as libc::c_int as byte_hack;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < *body_parts.offset(3 as libc::c_int as isize) as libc::c_int {
        (*p_ptr).body_parts[(28 as libc::c_int - 24 as libc::c_int + i) as
                                usize] = 28 as libc::c_int as byte_hack;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < *body_parts.offset(4 as libc::c_int as isize) as libc::c_int {
        (*p_ptr).body_parts[(42 as libc::c_int - 24 as libc::c_int + i) as
                                usize] = 42 as libc::c_int as byte_hack;
        (*p_ptr).body_parts[(34 as libc::c_int - 24 as libc::c_int + i) as
                                usize] = 34 as libc::c_int as byte_hack;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < b_arms {
        (*p_ptr).body_parts[(39 as libc::c_int - 24 as libc::c_int + i) as
                                usize] = 39 as libc::c_int as byte_hack;
        (*p_ptr).body_parts[(44 as libc::c_int - 24 as libc::c_int + i) as
                                usize] = 44 as libc::c_int as byte_hack;
        i += 1
    }
    if *body_parts.offset(2 as libc::c_int as isize) != 0 {
        (*p_ptr).body_parts[(51 as libc::c_int - 24 as libc::c_int) as usize]
            = 51 as libc::c_int as byte_hack
    }
    i = 0 as libc::c_int;
    while i < b_legs {
        (*p_ptr).body_parts[(47 as libc::c_int - 24 as libc::c_int + i) as
                                usize] = 47 as libc::c_int as byte_hack;
        i += 1
    }
    /* Ok now if the player lost a body part, he must drop the object he had on it */
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int - 24 as libc::c_int {
        if (*p_ptr).body_parts[i as usize] == 0 &&
               (*p_ptr).inventory[(i + 24 as libc::c_int) as usize].k_idx as
                   libc::c_int != 0 {
            /* Drop it NOW ! */
            inven_takeoff(i + 24 as libc::c_int, 255 as libc::c_int,
                          1 as libc::c_int as bool_);
        }
        i += 1
    };
}
/* Should be called by every calc_bonus call */
#[no_mangle]
pub unsafe extern "C" fn calc_body_bonus() {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*p_ptr).body_monster as isize) as
            *mut monster_race;
    /* If in the player body nothing have to be done */
    if (*p_ptr).body_monster == 0 { return }
    if (*p_ptr).disembodied != 0 {
        (*p_ptr).wraith_form = 1 as libc::c_int as bool_;
        return
    }
    (*p_ptr).ac =
        ((*p_ptr).ac as libc::c_int + (*r_ptr).ac as libc::c_int) as s16b;
    (*p_ptr).pspeed = (*r_ptr).speed as s16b;
    if (*r_ptr).flags1 & 0x20000 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).immovable = 1 as libc::c_int as bool_
    }
    if (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).stat_add[1 as libc::c_int as usize] =
            ((*p_ptr).stat_add[1 as libc::c_int as usize] as libc::c_int -
                 1 as libc::c_int) as s16b
    }
    if (*r_ptr).flags2 & 0x2 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).stat_add[1 as libc::c_int as usize] =
            ((*p_ptr).stat_add[1 as libc::c_int as usize] as libc::c_int +
                 1 as libc::c_int) as s16b
    }
    if (*r_ptr).flags2 & 0x8 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).reflect = 1 as libc::c_int as bool_
    }
    if (*r_ptr).flags2 & 0x10 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).invis =
            ((*p_ptr).invis as libc::c_int + 20 as libc::c_int) as s16b
    }
    if (*r_ptr).flags2 & 0x200 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).regenerate = 1 as libc::c_int as bool_
    }
    if (*r_ptr).flags2 & 0x4000 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).sh_fire = 1 as libc::c_int as bool_
    }
    if (*r_ptr).flags2 & 0x8000 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).sh_elec = 1 as libc::c_int as bool_
    }
    if (*r_ptr).flags2 & 0x40000 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).wraith_form = 1 as libc::c_int as bool_
    }
    if (*r_ptr).flags3 & 0x4000 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).sensible_fire = 1 as libc::c_int as bool_
    }
    if (*r_ptr).flags3 & 0x10000 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).resist_acid = 1 as libc::c_int as bool_
    }
    if (*r_ptr).flags3 & 0x20000 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).resist_elec = 1 as libc::c_int as bool_
    }
    if (*r_ptr).flags3 & 0x40000 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).resist_fire = 1 as libc::c_int as bool_
    }
    if (*r_ptr).flags3 & 0x100000 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).resist_pois = 1 as libc::c_int as bool_
    }
    if (*r_ptr).flags3 & 0x80000 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).resist_cold = 1 as libc::c_int as bool_
    }
    if (*r_ptr).flags3 & 0x400000 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).resist_neth = 1 as libc::c_int as bool_
    }
    if (*r_ptr).flags3 & 0x2000000 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).resist_nexus = 1 as libc::c_int as bool_
    }
    if (*r_ptr).flags3 & 0x4000000 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).resist_disen = 1 as libc::c_int as bool_
    }
    if (*r_ptr).flags3 & 0x10000000 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).resist_fear = 1 as libc::c_int as bool_
    }
    if (*r_ptr).flags3 & 0x80000000 as libc::c_uint != 0 {
        (*p_ptr).free_act = 1 as libc::c_int as bool_
    }
    if (*r_ptr).flags3 & 0x40000000 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).resist_conf = 1 as libc::c_int as bool_
    }
    if (*r_ptr).flags7 & 0x4 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).ffall = 1 as libc::c_int as bool_
    }
    if (*r_ptr).flags7 & 0x1 as libc::c_int as libc::c_uint != 0 {
        (*p_ptr).water_breath = 1 as libc::c_int as bool_
    };
}
#[no_mangle]
pub unsafe extern "C" fn calc_mimic() -> byte_hack {
    let mut blow: s32b = 0 as libc::c_int;
    call_lua(b"calc_mimic\x00" as *const u8 as *const libc::c_char,
             b"(d)\x00" as *const u8 as *const libc::c_char,
             b"d\x00" as *const u8 as *const libc::c_char,
             (*p_ptr).mimic_form as libc::c_int, &mut blow as *mut s32b);
    return blow as byte_hack;
}
/* Returns the number of extra blows based on abilities. */
unsafe extern "C" fn get_extra_blows_ability() -> libc::c_int {
    /* Count bonus abilities */
    let mut num: libc::c_int = 0 as libc::c_int;
    if has_ability(3 as libc::c_int) != 0 { num += 1 }
    if has_ability(4 as libc::c_int) != 0 { num += 1 }
    return num;
}
/* Returns the blow information based on class */
#[no_mangle]
pub unsafe extern "C" fn analyze_blow(mut num: *mut libc::c_int,
                                      mut wgt: *mut libc::c_int,
                                      mut mul: *mut libc::c_int) {
    *num = (*cp_ptr).blow_num as libc::c_int;
    *wgt = (*cp_ptr).blow_wgt as libc::c_int;
    *mul = (*cp_ptr).blow_mul as libc::c_int;
    /* Count bonus abilities */
    *num += get_extra_blows_ability();
}
/* Are all the weapons wielded of the right type ? */
#[no_mangle]
pub unsafe extern "C" fn get_weaponmastery_skill() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut skill: libc::c_int = 0 as libc::c_int;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    i = 0 as libc::c_int;
    /* All weapons must be of the same type */
    while (*p_ptr).body_parts[i as usize] as libc::c_int == 24 as libc::c_int
              && i < 52 as libc::c_int {
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset((24 as libc::c_int +
                                                              i) as isize) as
                *mut object_type;
        if (*o_ptr).k_idx == 0 {
            i += 1
        } else {
            match (*o_ptr).tval as libc::c_int {
                115 | 23 => {
                    if skill == 0 || skill == 18 as libc::c_int {
                        skill = 18 as libc::c_int
                    } else { skill = -(1 as libc::c_int) }
                }
                24 => {
                    if skill == 0 || skill == 19 as libc::c_int {
                        skill = 19 as libc::c_int
                    } else { skill = -(1 as libc::c_int) }
                }
                21 => {
                    if skill == 0 || skill == 21 as libc::c_int {
                        skill = 21 as libc::c_int
                    } else { skill = -(1 as libc::c_int) }
                }
                22 => {
                    if skill == 0 || skill == 20 as libc::c_int {
                        skill = 20 as libc::c_int
                    } else { skill = -(1 as libc::c_int) }
                }
                _ => { }
            }
            i += 1
        }
    }
    /* Everything is ok */
    return skill;
}
/* Are all the ranged weapons wielded of the right type ? */
#[no_mangle]
pub unsafe extern "C" fn get_archery_skill() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut skill: libc::c_int = 0 as libc::c_int;
    i = 27 as libc::c_int - 24 as libc::c_int;
    /* All weapons must be of the same type */
    while (*p_ptr).body_parts[i as usize] as libc::c_int == 27 as libc::c_int
          {
        if (*p_ptr).inventory[(24 as libc::c_int + i) as usize].tval as
               libc::c_int == 19 as libc::c_int {
            match (*p_ptr).inventory[(24 as libc::c_int + i) as usize].sval as
                      libc::c_int / 10 as libc::c_int {
                0 => {
                    if skill == 0 || skill == 24 as libc::c_int {
                        skill = 24 as libc::c_int
                    } else { skill = -(1 as libc::c_int) }
                }
                1 => {
                    if skill == 0 || skill == 25 as libc::c_int {
                        skill = 25 as libc::c_int
                    } else { skill = -(1 as libc::c_int) }
                }
                2 => {
                    if skill == 0 || skill == 26 as libc::c_int {
                        skill = 26 as libc::c_int
                    } else { skill = -(1 as libc::c_int) }
                }
                _ => { }
            }
        } else if skill == 0 || skill == 27 as libc::c_int {
            skill = 27 as libc::c_int
        } else { skill = -(1 as libc::c_int) }
        i += 1
    }
    /* Everything is ok */
    return skill;
}
/* Apply gods */
#[no_mangle]
pub unsafe extern "C" fn calc_gods() {
    /* Boost WIS if the player follows Eru */
    if (*p_ptr).pgod as libc::c_int == 1 as libc::c_int {
        if (*p_ptr).grace > 10000 as libc::c_int {
            (*p_ptr).stat_add[2 as libc::c_int as usize] =
                ((*p_ptr).stat_add[2 as libc::c_int as usize] as libc::c_int +
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 20000 as libc::c_int {
            (*p_ptr).stat_add[2 as libc::c_int as usize] =
                ((*p_ptr).stat_add[2 as libc::c_int as usize] as libc::c_int +
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 30000 as libc::c_int {
            (*p_ptr).stat_add[2 as libc::c_int as usize] =
                ((*p_ptr).stat_add[2 as libc::c_int as usize] as libc::c_int +
                     1 as libc::c_int) as s16b
        }
    }
    /* Boost str, con, chr and reduce int, wis if the player follows Melkor */
    if (*p_ptr).pgod as libc::c_int == 4 as libc::c_int {
        if (*p_ptr).grace > 10000 as libc::c_int {
            (*p_ptr).stat_add[0 as libc::c_int as usize] =
                ((*p_ptr).stat_add[0 as libc::c_int as usize] as libc::c_int +
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 20000 as libc::c_int {
            (*p_ptr).stat_add[0 as libc::c_int as usize] =
                ((*p_ptr).stat_add[0 as libc::c_int as usize] as libc::c_int +
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 30000 as libc::c_int {
            (*p_ptr).stat_add[0 as libc::c_int as usize] =
                ((*p_ptr).stat_add[0 as libc::c_int as usize] as libc::c_int +
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 10000 as libc::c_int {
            (*p_ptr).stat_add[4 as libc::c_int as usize] =
                ((*p_ptr).stat_add[4 as libc::c_int as usize] as libc::c_int +
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 20000 as libc::c_int {
            (*p_ptr).stat_add[4 as libc::c_int as usize] =
                ((*p_ptr).stat_add[4 as libc::c_int as usize] as libc::c_int +
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 30000 as libc::c_int {
            (*p_ptr).stat_add[4 as libc::c_int as usize] =
                ((*p_ptr).stat_add[4 as libc::c_int as usize] as libc::c_int +
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 10000 as libc::c_int {
            (*p_ptr).stat_add[5 as libc::c_int as usize] =
                ((*p_ptr).stat_add[5 as libc::c_int as usize] as libc::c_int +
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 20000 as libc::c_int {
            (*p_ptr).stat_add[5 as libc::c_int as usize] =
                ((*p_ptr).stat_add[5 as libc::c_int as usize] as libc::c_int +
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 30000 as libc::c_int {
            (*p_ptr).stat_add[5 as libc::c_int as usize] =
                ((*p_ptr).stat_add[5 as libc::c_int as usize] as libc::c_int +
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 10000 as libc::c_int {
            (*p_ptr).stat_add[1 as libc::c_int as usize] =
                ((*p_ptr).stat_add[1 as libc::c_int as usize] as libc::c_int -
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 20000 as libc::c_int {
            (*p_ptr).stat_add[1 as libc::c_int as usize] =
                ((*p_ptr).stat_add[1 as libc::c_int as usize] as libc::c_int -
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 30000 as libc::c_int {
            (*p_ptr).stat_add[1 as libc::c_int as usize] =
                ((*p_ptr).stat_add[1 as libc::c_int as usize] as libc::c_int -
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 10000 as libc::c_int {
            (*p_ptr).stat_add[2 as libc::c_int as usize] =
                ((*p_ptr).stat_add[2 as libc::c_int as usize] as libc::c_int -
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 20000 as libc::c_int {
            (*p_ptr).stat_add[2 as libc::c_int as usize] =
                ((*p_ptr).stat_add[2 as libc::c_int as usize] as libc::c_int -
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 30000 as libc::c_int {
            (*p_ptr).stat_add[2 as libc::c_int as usize] =
                ((*p_ptr).stat_add[2 as libc::c_int as usize] as libc::c_int -
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).pgod as libc::c_int == 4 as libc::c_int &&
               (*p_ptr).praying as libc::c_int != 0 {
            if (*p_ptr).grace > 5000 as libc::c_int {
                (*p_ptr).invis =
                    ((*p_ptr).invis as libc::c_int + 30 as libc::c_int) as
                        s16b
            }
            if (*p_ptr).grace > 15000 as libc::c_int {
                (*p_ptr).immune_fire = 1 as libc::c_int as bool_
            }
        }
        (*p_ptr).resist_fire = 1 as libc::c_int as bool_
    }
    /* Gifts of Manwe if the player is praying to Manwe */
    if (*p_ptr).pgod as libc::c_int == 2 as libc::c_int &&
           (*p_ptr).praying as libc::c_int != 0 {
        let mut add: s32b = (*p_ptr).grace;
        /* provides speed every 5000 grace */
        if add > 35000 as libc::c_int { add = 35000 as libc::c_int }
        add /= 5000 as libc::c_int;
        (*p_ptr).pspeed = ((*p_ptr).pspeed as libc::c_int + add) as s16b;
        /* Provides fly & FA */
        if (*p_ptr).grace >= 7000 as libc::c_int {
            (*p_ptr).free_act = 1 as libc::c_int as bool_
        }
        if (*p_ptr).grace >= 15000 as libc::c_int {
            (*p_ptr).fly = 1 as libc::c_int as bool_
        }
    }
    /* Manwe bonus not requiring the praying status */
    if (*p_ptr).pgod as libc::c_int == 2 as libc::c_int {
        if (*p_ptr).grace >= 2000 as libc::c_int {
            (*p_ptr).ffall = 1 as libc::c_int as bool_
        }
    }
    /* Boost Str and Con if the player is following Tulkas */
    if (*p_ptr).pgod as libc::c_int == 3 as libc::c_int {
        if (*p_ptr).grace > 5000 as libc::c_int {
            (*p_ptr).stat_add[4 as libc::c_int as usize] =
                ((*p_ptr).stat_add[4 as libc::c_int as usize] as libc::c_int +
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 10000 as libc::c_int {
            (*p_ptr).stat_add[4 as libc::c_int as usize] =
                ((*p_ptr).stat_add[4 as libc::c_int as usize] as libc::c_int +
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 15000 as libc::c_int {
            (*p_ptr).stat_add[4 as libc::c_int as usize] =
                ((*p_ptr).stat_add[4 as libc::c_int as usize] as libc::c_int +
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 10000 as libc::c_int {
            (*p_ptr).stat_add[0 as libc::c_int as usize] =
                ((*p_ptr).stat_add[0 as libc::c_int as usize] as libc::c_int +
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 15000 as libc::c_int {
            (*p_ptr).stat_add[0 as libc::c_int as usize] =
                ((*p_ptr).stat_add[0 as libc::c_int as usize] as libc::c_int +
                     1 as libc::c_int) as s16b
        }
        if (*p_ptr).grace > 20000 as libc::c_int {
            (*p_ptr).stat_add[0 as libc::c_int as usize] =
                ((*p_ptr).stat_add[0 as libc::c_int as usize] as libc::c_int +
                     1 as libc::c_int) as s16b
        }
    };
}
/* Apply flags */
static mut extra_blows: libc::c_int = 0;
static mut extra_shots: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn apply_flags(mut f1: u32b, mut f2: u32b, mut f3: u32b,
                                     mut f4: u32b, mut f5: u32b,
                                     mut esp: u32b, mut pval: s16b,
                                     mut tval: s16b, mut to_h: s16b,
                                     mut to_d: s16b, mut to_a: s16b) {
    let mut antimagic_mod: s16b = 0;
    /* Affect stats */
    if f1 as libc::c_long & 0x1 as libc::c_long != 0 {
        (*p_ptr).stat_add[0 as libc::c_int as usize] =
            ((*p_ptr).stat_add[0 as libc::c_int as usize] as libc::c_int +
                 pval as libc::c_int) as s16b
    }
    if f1 as libc::c_long & 0x2 as libc::c_long != 0 {
        (*p_ptr).stat_add[1 as libc::c_int as usize] =
            ((*p_ptr).stat_add[1 as libc::c_int as usize] as libc::c_int +
                 pval as libc::c_int) as s16b
    }
    if f1 as libc::c_long & 0x4 as libc::c_long != 0 {
        (*p_ptr).stat_add[2 as libc::c_int as usize] =
            ((*p_ptr).stat_add[2 as libc::c_int as usize] as libc::c_int +
                 pval as libc::c_int) as s16b
    }
    if f1 as libc::c_long & 0x8 as libc::c_long != 0 {
        (*p_ptr).stat_add[3 as libc::c_int as usize] =
            ((*p_ptr).stat_add[3 as libc::c_int as usize] as libc::c_int +
                 pval as libc::c_int) as s16b
    }
    if f1 as libc::c_long & 0x10 as libc::c_long != 0 {
        (*p_ptr).stat_add[4 as libc::c_int as usize] =
            ((*p_ptr).stat_add[4 as libc::c_int as usize] as libc::c_int +
                 pval as libc::c_int) as s16b
    }
    if f1 as libc::c_long & 0x20 as libc::c_long != 0 {
        (*p_ptr).stat_add[5 as libc::c_int as usize] =
            ((*p_ptr).stat_add[5 as libc::c_int as usize] as libc::c_int +
                 pval as libc::c_int) as s16b
    }
    if f5 as libc::c_long & 0x200 as libc::c_long != 0 {
        (*p_ptr).luck_cur =
            ((*p_ptr).luck_cur as libc::c_int + pval as libc::c_int) as s16b
    }
    /* Affect spell power */
    if f1 as libc::c_long & 0x80 as libc::c_long != 0 {
        (*p_ptr).to_s =
            ((*p_ptr).to_s as libc::c_int + pval as libc::c_int) as s16b
    }
    /* Affect mana capacity */
    if f1 as libc::c_long & 0x40 as libc::c_long != 0 {
        (*p_ptr).to_m =
            ((*p_ptr).to_m as libc::c_int + pval as libc::c_int) as s16b
    }
    /* Affect life capacity */
    if f2 as libc::c_long & 0x80 as libc::c_long != 0 {
        (*p_ptr).to_l =
            ((*p_ptr).to_l as libc::c_int + pval as libc::c_int) as s16b
    }
    /* Affect stealth */
    if f1 as libc::c_long & 0x100 as libc::c_long != 0 {
        (*p_ptr).skill_stl =
            ((*p_ptr).skill_stl as libc::c_int + pval as libc::c_int) as s16b
    }
    /* Affect searching ability (factor of five) */
    if f1 as libc::c_long & 0x200 as libc::c_long != 0 {
        (*p_ptr).skill_srh =
            ((*p_ptr).skill_srh as libc::c_int +
                 pval as libc::c_int * 5 as libc::c_int) as s16b
    }
    /* Affect searching frequency (factor of five) */
    if f1 as libc::c_long & 0x200 as libc::c_long != 0 {
        (*p_ptr).skill_fos =
            ((*p_ptr).skill_fos as libc::c_int +
                 pval as libc::c_int * 5 as libc::c_int) as s16b
    }
    /* Affect infravision */
    if f1 as libc::c_long & 0x400 as libc::c_long != 0 {
        (*p_ptr).see_infra =
            ((*p_ptr).see_infra as libc::c_int + pval as libc::c_int) as s16b
    }
    /* Affect digging (factor of 20) */
    if f1 as libc::c_long & 0x800 as libc::c_long != 0 {
        (*p_ptr).skill_dig =
            ((*p_ptr).skill_dig as libc::c_int +
                 pval as libc::c_int * 20 as libc::c_int) as s16b
    }
    /* Affect speed */
    if f1 as libc::c_long & 0x1000 as libc::c_long != 0 {
        (*p_ptr).pspeed =
            ((*p_ptr).pspeed as libc::c_int + pval as libc::c_int) as s16b
    }
    /* Affect blows */
    if f1 as libc::c_long & 0x2000 as libc::c_long != 0 {
        extra_blows += pval as libc::c_int
    }
    if f5 as libc::c_long & 0x20 as libc::c_long != 0 {
        (*p_ptr).xtra_crit =
            ((*p_ptr).xtra_crit as libc::c_int + pval as libc::c_int) as s16b
    }
    /* Hack -- Sensible fire */
    if f2 as libc::c_long & 0x1000 as libc::c_long != 0 {
        (*p_ptr).sensible_fire = 1 as libc::c_int as bool_
    }
    /* Hack -- cause earthquakes */
    if f1 as libc::c_long & 0x4000000 as libc::c_long != 0 {
        (*p_ptr).impact = 1 as libc::c_int as bool_
    }
    /* Affect invisibility */
    if f2 as libc::c_long & 0x40 as libc::c_long != 0 {
        (*p_ptr).invis =
            ((*p_ptr).invis as libc::c_int +
                 pval as libc::c_int * 10 as libc::c_int) as s16b
    }
    /* Boost shots */
    if f3 as libc::c_long & 0x80000 as libc::c_long != 0 { extra_shots += 1 }
    /* Various flags */
    if f3 as libc::c_long & 0x8000000 as libc::c_long != 0 {
        (*p_ptr).aggravate = 1 as libc::c_int as bool_
    }
    if f3 as libc::c_long & 0x4000000 as libc::c_long != 0 {
        (*p_ptr).teleport = 1 as libc::c_int as bool_
    }
    if f5 as libc::c_long & 0x2 as libc::c_long != 0 {
        (*p_ptr).drain_mana = (*p_ptr).drain_mana.wrapping_add(1)
    }
    if f5 as libc::c_long & 0x4 as libc::c_long != 0 {
        (*p_ptr).drain_life = (*p_ptr).drain_life.wrapping_add(1)
    }
    if f3 as libc::c_long & 0x2000000 as libc::c_long != 0 {
        (*p_ptr).exp_drain = 1 as libc::c_int as bool_
    }
    if f3 as libc::c_long & 0x10000000 as libc::c_long != 0 {
        (*p_ptr).bless_blade = 1 as libc::c_int as bool_
    }
    if f3 as libc::c_long & 0x40000 as libc::c_long != 0 {
        (*p_ptr).xtra_might =
            ((*p_ptr).xtra_might as libc::c_int + pval as libc::c_int) as
                byte_hack
    }
    if f3 as libc::c_long & 0x10000 as libc::c_long != 0 {
        (*p_ptr).slow_digest = 1 as libc::c_int as bool_
    }
    if f3 as libc::c_long & 0x20000 as libc::c_long != 0 {
        (*p_ptr).regenerate = 1 as libc::c_int as bool_
    }
    if esp != 0 { (*p_ptr).telepathy |= esp }
    if tval as libc::c_int != 39 as libc::c_int &&
           f3 as libc::c_long & 0x2000 as libc::c_long != 0 {
        (*p_ptr).lite = 1 as libc::c_int as bool_
    }
    if tval as libc::c_int != 39 as libc::c_int &&
           f4 as libc::c_long & 0x4000000 as libc::c_long != 0 {
        (*p_ptr).lite = 1 as libc::c_int as bool_
    }
    if tval as libc::c_int != 39 as libc::c_int &&
           f4 as libc::c_long & 0x8000000 as libc::c_long != 0 {
        (*p_ptr).lite = 1 as libc::c_int as bool_
    }
    if f3 as libc::c_long & 0x4000 as libc::c_long != 0 {
        (*p_ptr).see_inv = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x4000 as libc::c_long != 0 {
        (*p_ptr).free_act = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x8000 as libc::c_long != 0 {
        (*p_ptr).hold_life = 1 as libc::c_int as bool_
    }
    if f3 as libc::c_long & 0x40 as libc::c_long != 0 {
        (*p_ptr).wraith_form = 1 as libc::c_int as bool_
    }
    if f3 as libc::c_long & 0x1000 as libc::c_long != 0 {
        (*p_ptr).ffall = 1 as libc::c_int as bool_
    }
    if f4 as libc::c_long & 0x10 as libc::c_long != 0 {
        (*p_ptr).fly = 1 as libc::c_int as bool_
    }
    if f4 as libc::c_long & 0x800 as libc::c_long != 0 {
        (*p_ptr).climb = 1 as libc::c_int as bool_
    }
    /* Immunity flags */
    if f2 as libc::c_long & 0x400 as libc::c_long != 0 {
        (*p_ptr).immune_fire = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x100 as libc::c_long != 0 {
        (*p_ptr).immune_acid = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x800 as libc::c_long != 0 {
        (*p_ptr).immune_cold = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x200 as libc::c_long != 0 {
        (*p_ptr).immune_elec = 1 as libc::c_int as bool_
    }
    /* Resistance flags */
    if f2 as libc::c_long & 0x10000 as libc::c_long != 0 {
        (*p_ptr).resist_acid = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x20000 as libc::c_long != 0 {
        (*p_ptr).resist_elec = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x40000 as libc::c_long != 0 {
        (*p_ptr).resist_fire = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x80000 as libc::c_long != 0 {
        (*p_ptr).resist_cold = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x100000 as libc::c_long != 0 {
        (*p_ptr).resist_pois = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x200000 as libc::c_long != 0 {
        (*p_ptr).resist_fear = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x2000000 as libc::c_long != 0 {
        (*p_ptr).resist_conf = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x4000000 as libc::c_long != 0 {
        (*p_ptr).resist_sound = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x400000 as libc::c_long != 0 {
        (*p_ptr).resist_lite = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x800000 as libc::c_long != 0 {
        (*p_ptr).resist_dark = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x40000000 as libc::c_long != 0 {
        (*p_ptr).resist_chaos = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x80000000 as libc::c_long != 0 {
        (*p_ptr).resist_disen = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x8000000 as libc::c_long != 0 {
        (*p_ptr).resist_shard = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x20000000 as libc::c_long != 0 {
        (*p_ptr).resist_nexus = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x1000000 as libc::c_long != 0 {
        (*p_ptr).resist_blind = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x10000000 as libc::c_long != 0 {
        (*p_ptr).resist_neth = 1 as libc::c_int as bool_
    }
    if f4 as libc::c_long & 0x400000 as libc::c_long != 0 {
        (*p_ptr).immune_neth = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x2000 as libc::c_long != 0 {
        (*p_ptr).reflect = 1 as libc::c_int as bool_
    }
    if f3 as libc::c_long & 0x1 as libc::c_long != 0 {
        (*p_ptr).sh_fire = 1 as libc::c_int as bool_
    }
    if f3 as libc::c_long & 0x2 as libc::c_long != 0 {
        (*p_ptr).sh_elec = 1 as libc::c_int as bool_
    }
    if f3 as libc::c_long & 0x20 as libc::c_long != 0 {
        (*p_ptr).anti_magic = 1 as libc::c_int as bool_
    }
    if f3 as libc::c_long & 0x10 as libc::c_long != 0 {
        (*p_ptr).anti_tele = 1 as libc::c_int as bool_
    }
    /* Sustain flags */
    if f2 as libc::c_long & 0x1 as libc::c_long != 0 {
        (*p_ptr).sustain_str = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x2 as libc::c_long != 0 {
        (*p_ptr).sustain_int = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x4 as libc::c_long != 0 {
        (*p_ptr).sustain_wis = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x8 as libc::c_long != 0 {
        (*p_ptr).sustain_dex = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x10 as libc::c_long != 0 {
        (*p_ptr).sustain_con = 1 as libc::c_int as bool_
    }
    if f2 as libc::c_long & 0x20 as libc::c_long != 0 {
        (*p_ptr).sustain_chr = 1 as libc::c_int as bool_
    }
    if f4 as libc::c_long & 0x2 as libc::c_long != 0 {
        (*p_ptr).precognition = 1 as libc::c_int as bool_
    }
    antimagic_mod =
        (to_h as libc::c_int + to_d as libc::c_int + to_a as libc::c_int) as
            s16b;
    if f4 as libc::c_long & 0x20000 as libc::c_long != 0 {
        let mut tmp: s32b = 0;
        tmp =
            10 as libc::c_int +
                get_skill_scale(33 as libc::c_int, 40 as libc::c_int as u32b)
                    as libc::c_int - antimagic_mod as libc::c_int;
        if tmp > 0 as libc::c_int {
            (*p_ptr).antimagic =
                ((*p_ptr).antimagic as libc::c_int + tmp) as byte_hack
        }
        tmp =
            1 as libc::c_int +
                get_skill_scale(33 as libc::c_int, 4 as libc::c_int as u32b)
                    as libc::c_int -
                antimagic_mod as libc::c_int / 15 as libc::c_int;
        if tmp > 0 as libc::c_int {
            (*p_ptr).antimagic_dis =
                ((*p_ptr).antimagic_dis as libc::c_int + tmp) as byte_hack
        }
    }
    if f4 as libc::c_long & 0x40000 as libc::c_long != 0 {
        let mut tmp_0: s32b = 0;
        tmp_0 =
            7 as libc::c_int +
                get_skill_scale(33 as libc::c_int, 33 as libc::c_int as u32b)
                    as libc::c_int - antimagic_mod as libc::c_int;
        if tmp_0 > 0 as libc::c_int {
            (*p_ptr).antimagic =
                ((*p_ptr).antimagic as libc::c_int + tmp_0) as byte_hack
        }
        tmp_0 =
            1 as libc::c_int +
                get_skill_scale(33 as libc::c_int, 2 as libc::c_int as u32b)
                    as libc::c_int -
                antimagic_mod as libc::c_int / 15 as libc::c_int;
        if tmp_0 > 0 as libc::c_int {
            (*p_ptr).antimagic_dis =
                ((*p_ptr).antimagic_dis as libc::c_int + tmp_0) as byte_hack
        }
    }
    if f4 as libc::c_long & 0x80000 as libc::c_long != 0 {
        let mut tmp_1: s32b = 0;
        tmp_1 =
            5 as libc::c_int +
                get_skill_scale(33 as libc::c_int, 15 as libc::c_int as u32b)
                    as libc::c_int - antimagic_mod as libc::c_int;
        if tmp_1 > 0 as libc::c_int {
            (*p_ptr).antimagic =
                ((*p_ptr).antimagic as libc::c_int + tmp_1) as byte_hack
        }
        (*p_ptr).antimagic_dis =
            ((*p_ptr).antimagic_dis as libc::c_int + 2 as libc::c_int) as
                byte_hack
    }
    if f4 as libc::c_long & 0x100000 as libc::c_long != 0 {
        let mut tmp_2: s32b = 0;
        tmp_2 =
            1 as libc::c_int +
                get_skill_scale(33 as libc::c_int, 9 as libc::c_int as u32b)
                    as libc::c_int - antimagic_mod as libc::c_int;
        if tmp_2 > 0 as libc::c_int {
            (*p_ptr).antimagic =
                ((*p_ptr).antimagic as libc::c_int + tmp_2) as byte_hack
        }
        (*p_ptr).antimagic_dis =
            ((*p_ptr).antimagic_dis as libc::c_int + 1 as libc::c_int) as
                byte_hack
    }
    if f4 as libc::c_long & 0x2000000 as libc::c_long != 0 {
        (*p_ptr).auto_id = 1 as libc::c_int as bool_
    }
    /* The new code implementing Tolkien's concept of "Black Breath"
	 * takes advantage of the existing drain_exp character flag, renamed
	 * "black_breath". This flag can also be set by a unlucky blow from
	 * an undead.  -LM-
	 */
    if f4 as libc::c_long & 0x4 as libc::c_long != 0 {
        (*p_ptr).black_breath = 1 as libc::c_int as bool_
    }
    if f5 as libc::c_long & 0x400 as libc::c_long != 0 {
        (*p_ptr).immovable = 1 as libc::c_int as bool_
    }
    /* Breaths */
    if f5 as libc::c_long & 0x8000 as libc::c_long != 0 {
        (*p_ptr).water_breath = 1 as libc::c_int as bool_
    }
    if f5 as libc::c_long & 0x4000 as libc::c_long != 0 {
        (*p_ptr).magical_breath = 1 as libc::c_int as bool_;
        (*p_ptr).water_breath = 1 as libc::c_int as bool_
    };
}
/*
 * Calculate the players current "state", taking into account
 * not only race/class intrinsics, but also objects being worn
 * and temporary spell effects.
 *
 * See also calc_mana() and calc_hitpoints().
 *
 * Take note of the new "speed code", in particular, a very strong
 * player will start slowing down as soon as he reaches 150 pounds,
 * but not until he reaches 450 pounds will he be half as fast as
 * a normal kobold.  This both hurts and helps the player, hurts
 * because in the old days a player could just avoid 300 pounds,
 * and helps because now carrying 300 pounds is not very painful.
 *
 * The "weapon" and "bow" do *not* add to the bonuses to hit or to
 * damage, since that would affect non-combat things.  These values
 * are actually added in later, at the appropriate place.
 *
 * This function induces various "status" messages, unless silent is
 * TRUE.
 */
#[no_mangle]
pub unsafe extern "C" fn calc_bonuses(mut silent: bool_) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut hold: libc::c_int = 0;
    let mut old_speed: libc::c_int = 0;
    let mut old_telepathy: u32b = 0;
    let mut old_see_inv: libc::c_int = 0;
    let mut old_dis_ac: libc::c_int = 0;
    let mut old_dis_to_a: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    /* Save the old speed */
    old_speed = (*p_ptr).pspeed as libc::c_int;
    /* Save the old vision stuff */
    old_telepathy = (*p_ptr).telepathy;
    old_see_inv = (*p_ptr).see_inv as libc::c_int;
    /* Save the old armor class */
    old_dis_ac = (*p_ptr).dis_ac as libc::c_int;
    old_dis_to_a = (*p_ptr).dis_to_a as libc::c_int;
    /* Clear extra blows/shots */
    extra_shots = 0 as libc::c_int;
    extra_blows = extra_shots;
    /* Clear the stat modifiers */
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        (*p_ptr).stat_add[i as usize] = 0 as libc::c_int as s16b;
        i += 1
    }
    /* Mana multiplier */
    (*p_ptr).to_m = 0 as libc::c_int as s16b;
    /* Life multiplier */
    (*p_ptr).to_l = 0 as libc::c_int as s16b;
    /* Spell power */
    (*p_ptr).to_s = 0 as libc::c_int as s16b;
    /* Clear the Displayed/Real armor class */
    (*p_ptr).ac = 0 as libc::c_int as s16b;
    (*p_ptr).dis_ac = (*p_ptr).ac;
    /* Clear the Displayed/Real Bonuses */
    (*p_ptr).to_h_ranged = 0 as libc::c_int as s16b;
    (*p_ptr).to_h_melee = (*p_ptr).to_h_ranged;
    (*p_ptr).to_h = (*p_ptr).to_h_melee;
    (*p_ptr).dis_to_h = (*p_ptr).to_h;
    (*p_ptr).to_d_ranged = 0 as libc::c_int as s16b;
    (*p_ptr).to_d_melee = (*p_ptr).to_d_ranged;
    (*p_ptr).to_d = (*p_ptr).to_d_melee;
    (*p_ptr).dis_to_d = (*p_ptr).to_d;
    (*p_ptr).to_a = 0 as libc::c_int as s16b;
    (*p_ptr).dis_to_a = (*p_ptr).to_a;
    /* Start with "normal" speed */
    (*p_ptr).pspeed = 110 as libc::c_int as s16b;
    /* Start with 0% additionnal crits */
    (*p_ptr).xtra_crit = 0 as libc::c_int as s16b;
    /* Start with a single blow per turn */
    (*p_ptr).num_blow = 1 as libc::c_int as s16b;
    /* Start with a single shot per turn */
    (*p_ptr).num_fire = 1 as libc::c_int as s16b;
    /* Starts with single throwing damage */
    (*p_ptr).throw_mult = 1 as libc::c_int as byte_hack;
    /* Reset the "xtra" tval */
    (*p_ptr).tval_xtra = 0 as libc::c_int as byte_hack;
    /* Reset the "ammo" tval */
    (*p_ptr).tval_ammo = 0 as libc::c_int as byte_hack;
    /* Clear all the flags */
    (*p_ptr).invis = 0 as libc::c_int as s16b;
    (*p_ptr).immovable = 0 as libc::c_int as bool_;
    (*p_ptr).aggravate = 0 as libc::c_int as bool_;
    (*p_ptr).teleport = 0 as libc::c_int as bool_;
    (*p_ptr).exp_drain = 0 as libc::c_int as bool_;
    (*p_ptr).drain_mana = 0 as libc::c_int as byte_hack;
    (*p_ptr).drain_life = 0 as libc::c_int as byte_hack;
    (*p_ptr).bless_blade = 0 as libc::c_int as bool_;
    (*p_ptr).xtra_might = 0 as libc::c_int as byte_hack;
    (*p_ptr).auto_id = 0 as libc::c_int as bool_;
    (*p_ptr).impact = 0 as libc::c_int as bool_;
    (*p_ptr).see_inv = 0 as libc::c_int as bool_;
    (*p_ptr).free_act = 0 as libc::c_int as bool_;
    (*p_ptr).slow_digest = 0 as libc::c_int as bool_;
    (*p_ptr).regenerate = 0 as libc::c_int as bool_;
    (*p_ptr).fly = 0 as libc::c_int as bool_;
    (*p_ptr).climb = 0 as libc::c_int as bool_;
    (*p_ptr).ffall = 0 as libc::c_int as bool_;
    (*p_ptr).hold_life = 0 as libc::c_int as bool_;
    (*p_ptr).telepathy = 0 as libc::c_int as u32b;
    (*p_ptr).lite = 0 as libc::c_int as bool_;
    (*p_ptr).sustain_str = 0 as libc::c_int as bool_;
    (*p_ptr).sustain_int = 0 as libc::c_int as bool_;
    (*p_ptr).sustain_wis = 0 as libc::c_int as bool_;
    (*p_ptr).sustain_con = 0 as libc::c_int as bool_;
    (*p_ptr).sustain_dex = 0 as libc::c_int as bool_;
    (*p_ptr).sustain_chr = 0 as libc::c_int as bool_;
    (*p_ptr).resist_acid = 0 as libc::c_int as bool_;
    (*p_ptr).resist_elec = 0 as libc::c_int as bool_;
    (*p_ptr).resist_fire = 0 as libc::c_int as bool_;
    (*p_ptr).resist_cold = 0 as libc::c_int as bool_;
    (*p_ptr).resist_pois = 0 as libc::c_int as bool_;
    (*p_ptr).resist_conf = 0 as libc::c_int as bool_;
    (*p_ptr).resist_sound = 0 as libc::c_int as bool_;
    (*p_ptr).resist_lite = 0 as libc::c_int as bool_;
    (*p_ptr).resist_dark = 0 as libc::c_int as bool_;
    (*p_ptr).resist_chaos = 0 as libc::c_int as bool_;
    (*p_ptr).resist_disen = 0 as libc::c_int as bool_;
    (*p_ptr).resist_shard = 0 as libc::c_int as bool_;
    (*p_ptr).resist_nexus = 0 as libc::c_int as bool_;
    (*p_ptr).resist_blind = 0 as libc::c_int as bool_;
    (*p_ptr).resist_neth = 0 as libc::c_int as bool_;
    (*p_ptr).immune_neth = 0 as libc::c_int as bool_;
    (*p_ptr).resist_fear = 0 as libc::c_int as bool_;
    (*p_ptr).resist_continuum = 0 as libc::c_int as bool_;
    (*p_ptr).reflect = 0 as libc::c_int as bool_;
    (*p_ptr).sh_fire = 0 as libc::c_int as bool_;
    (*p_ptr).sh_elec = 0 as libc::c_int as bool_;
    (*p_ptr).anti_magic = 0 as libc::c_int as bool_;
    (*p_ptr).anti_tele = 0 as libc::c_int as bool_;
    (*p_ptr).water_breath = 0 as libc::c_int as bool_;
    (*p_ptr).magical_breath = 0 as libc::c_int as bool_;
    (*p_ptr).sensible_fire = 0 as libc::c_int as bool_;
    (*p_ptr).sensible_lite = 0 as libc::c_int as bool_;
    (*p_ptr).immune_acid = 0 as libc::c_int as bool_;
    (*p_ptr).immune_elec = 0 as libc::c_int as bool_;
    (*p_ptr).immune_fire = 0 as libc::c_int as bool_;
    (*p_ptr).immune_cold = 0 as libc::c_int as bool_;
    (*p_ptr).precognition = 0 as libc::c_int as bool_;
    (*p_ptr).wraith_form = 0 as libc::c_int as bool_;
    /* The anti magic field surrounding the player */
    (*p_ptr).antimagic = 0 as libc::c_int as byte_hack;
    (*p_ptr).antimagic_dis = 0 as libc::c_int as byte_hack;
    /* Base infravision (purely racial) */
    (*p_ptr).see_infra =
        ((*rp_ptr).infra as libc::c_int + (*rmp_ptr).infra as libc::c_int) as
            s16b;
    /* Base skill -- disarming */
    (*p_ptr).skill_dis = 0 as libc::c_int as s16b;
    /* Base skill -- magic devices */
    (*p_ptr).skill_dev = 0 as libc::c_int as s16b;
    /* Base skill -- saving throw */
    (*p_ptr).skill_sav = 0 as libc::c_int as s16b;
    /* Base skill -- stealth */
    (*p_ptr).skill_stl = 0 as libc::c_int as s16b;
    /* Base skill -- searching ability */
    (*p_ptr).skill_srh = 0 as libc::c_int as s16b;
    /* Base skill -- searching frequency */
    (*p_ptr).skill_fos = 0 as libc::c_int as s16b;
    /* Base skill -- combat (normal) */
    (*p_ptr).skill_thn = 0 as libc::c_int as s16b;
    /* Base skill -- combat (shooting) */
    (*p_ptr).skill_thb = 0 as libc::c_int as s16b;
    /* Base skill -- combat (throwing) */
    (*p_ptr).skill_tht = 0 as libc::c_int as s16b;
    /* Base skill -- digging */
    (*p_ptr).skill_dig = 0 as libc::c_int as s16b;
    /* Xtra player flags */
    (*p_ptr).xtra_f1 = 0 as libc::c_int as u32b;
    (*p_ptr).xtra_f2 = 0 as libc::c_int as u32b;
    (*p_ptr).xtra_f3 = 0 as libc::c_int as u32b;
    (*p_ptr).xtra_f4 = 0 as libc::c_int as u32b;
    (*p_ptr).xtra_f5 = 0 as libc::c_int as u32b;
    (*p_ptr).xtra_esp = 0 as libc::c_int as u32b;
    /* Hide the skills that should auto hide */
    i = 0 as libc::c_int;
    while i < max_s_idx as libc::c_int {
        if (*s_info.offset(i as isize)).flags1 &
               0x2 as libc::c_int as libc::c_uint != 0 {
            (*s_info.offset(i as isize)).hidden = 1 as libc::c_int as bool_
        }
        i += 1
    }
    /* Base Luck */
    (*p_ptr).luck_cur = (*p_ptr).luck_base;
    /* Mimic override body's bonuses */
    if (*p_ptr).mimic_form != 0 {
        extra_blows += calc_mimic() as libc::c_int
    } else { calc_body_bonus(); }
    /* Let the scripts do what they need */
    process_hooks(29 as libc::c_int,
                  b"()\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char);
    /* The powers gived by the wielded monster */
    calc_wield_monster();
    i = 1 as libc::c_int;
    while i <= (*p_ptr).lev as libc::c_int {
        apply_flags((*cp_ptr).oflags1[i as usize],
                    (*cp_ptr).oflags2[i as usize],
                    (*cp_ptr).oflags3[i as usize],
                    (*cp_ptr).oflags4[i as usize],
                    (*cp_ptr).oflags5[i as usize], (*cp_ptr).oesp[i as usize],
                    (*cp_ptr).opval[i as usize], 0 as libc::c_int as s16b,
                    0 as libc::c_int as s16b, 0 as libc::c_int as s16b,
                    0 as libc::c_int as s16b);
        i += 1
    }
    if (*p_ptr).melee_style as libc::c_int == 42 as libc::c_int {
        /* Unencumbered Monks become faster every 10 levels */
        if monk_heavy_armor() == 0 {
            (*p_ptr).pspeed =
                ((*p_ptr).pspeed as libc::c_int +
                     get_skill_scale(42 as libc::c_int,
                                     5 as libc::c_int as u32b) as libc::c_int)
                    as s16b
        }
        /* Free action if unencumbered at level 25 */
        if get_skill(42 as libc::c_int) as libc::c_int > 24 as libc::c_int &&
               monk_heavy_armor() == 0 {
            (*p_ptr).free_act = 1 as libc::c_int as bool_
        }
    }
    if get_skill(33 as libc::c_int) != 0 {
        (*p_ptr).antimagic =
            ((*p_ptr).antimagic as libc::c_int +
                 get_skill(33 as libc::c_int) as libc::c_int) as byte_hack;
        (*p_ptr).antimagic_dis =
            ((*p_ptr).antimagic_dis as libc::c_int +
                 (get_skill_scale(33 as libc::c_int,
                                  10 as libc::c_int as u32b) as libc::c_int +
                      1 as libc::c_int)) as byte_hack;
        if (*p_ptr).antimagic_extra & 0x10 as libc::c_int as libc::c_uint != 0
           {
            (*p_ptr).anti_tele = 1 as libc::c_int as bool_;
            (*p_ptr).resist_continuum = 1 as libc::c_int as bool_
        }
    }
    if get_skill(13 as libc::c_int) as libc::c_int > 20 as libc::c_int {
        (*p_ptr).resist_conf = 1 as libc::c_int as bool_
    }
    if get_skill(13 as libc::c_int) as libc::c_int > 30 as libc::c_int {
        (*p_ptr).resist_fear = 1 as libc::c_int as bool_
    }
    if get_skill(29 as libc::c_int) as libc::c_int >= 40 as libc::c_int {
        (*p_ptr).telepathy = 0x80000000 as libc::c_long as u32b
    }
    if (*p_ptr).astral != 0 {
        (*p_ptr).wraith_form = 1 as libc::c_int as bool_
    }
    /* **** Races ****/
    if (*p_ptr).mimic_form == 0 && (*p_ptr).body_monster == 0 {
        let mut i_0: libc::c_int = 0;
        i_0 = 1 as libc::c_int;
        while i_0 <= (*p_ptr).lev as libc::c_int {
            apply_flags((*rp_ptr).oflags1[i_0 as usize],
                        (*rp_ptr).oflags2[i_0 as usize],
                        (*rp_ptr).oflags3[i_0 as usize],
                        (*rp_ptr).oflags4[i_0 as usize],
                        (*rp_ptr).oflags5[i_0 as usize],
                        (*rp_ptr).oesp[i_0 as usize],
                        (*rp_ptr).opval[i_0 as usize],
                        0 as libc::c_int as s16b, 0 as libc::c_int as s16b,
                        0 as libc::c_int as s16b, 0 as libc::c_int as s16b);
            apply_flags((*rmp_ptr).oflags1[i_0 as usize],
                        (*rmp_ptr).oflags2[i_0 as usize],
                        (*rmp_ptr).oflags3[i_0 as usize],
                        (*rmp_ptr).oflags4[i_0 as usize],
                        (*rmp_ptr).oflags5[i_0 as usize],
                        (*rmp_ptr).oesp[i_0 as usize],
                        (*rmp_ptr).opval[i_0 as usize],
                        0 as libc::c_int as s16b, 0 as libc::c_int as s16b,
                        0 as libc::c_int as s16b, 0 as libc::c_int as s16b);
            i_0 += 1
        }
        if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
                (*spp_ptr).flags1) as libc::c_long & 0x100 as libc::c_long !=
               0 {
            (*p_ptr).sensible_lite = 1 as libc::c_int as bool_
        }
    }
    /* The extra flags */
    apply_flags((*p_ptr).xtra_f1, (*p_ptr).xtra_f2, (*p_ptr).xtra_f3,
                (*p_ptr).xtra_f4, (*p_ptr).xtra_f5, (*p_ptr).xtra_esp,
                0 as libc::c_int as s16b, 0 as libc::c_int as s16b,
                0 as libc::c_int as s16b, 0 as libc::c_int as s16b,
                0 as libc::c_int as s16b);
    /* Hack -- apply racial/class stat maxes */
    if (*p_ptr).maximize != 0 {
        /* Apply the racial modifiers */
        i = 0 as libc::c_int;
        while i < 6 as libc::c_int {
            /* Modify the stats for "race" */
            (*p_ptr).stat_add[i as usize] =
                ((*p_ptr).stat_add[i as usize] as libc::c_int +
                     ((*rp_ptr).r_adj[i as usize] as libc::c_int +
                          (*rmp_ptr).r_adj[i as usize] as libc::c_int +
                          (*cp_ptr).c_adj[i as usize] as libc::c_int)) as
                    s16b;
            i += 1
        }
    }
    /* Scan the usable inventory */
    i = 24 as libc::c_int;
    while i < 52 as libc::c_int {
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Skip non-objects */
        if !((*o_ptr).k_idx == 0) {
            /* Extract the item flags */
            object_flags_no_set = 1 as libc::c_int as bool_;
            object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                         &mut esp);
            object_flags_no_set = 0 as libc::c_int as bool_;
            /* MEGA ugly hack -- set spacetime distortion resistance */
            if (*o_ptr).name1 as libc::c_int == 14 as libc::c_int {
                (*p_ptr).resist_continuum = 1 as libc::c_int as bool_
            }
            /* Hack - don't give the Black Breath when merely inspecting a weapon */
            if silent != 0 {
                f4 = (f4 as libc::c_long & !(0x4 as libc::c_long)) as u32b
            }
            apply_flags(f1, f2, f3, f4, f5, esp, (*o_ptr).pval as s16b,
                        (*o_ptr).tval as s16b, (*o_ptr).to_h, (*o_ptr).to_d,
                        (*o_ptr).to_a);
            if (*o_ptr).name1 != 0 {
                apply_set((*o_ptr).name1 as s16b,
                          (*a_info.offset((*o_ptr).name1 as isize)).set);
            }
            /* Modify the base armor class */
            (*p_ptr).ac =
                ((*p_ptr).ac as libc::c_int + (*o_ptr).ac as libc::c_int) as
                    s16b;
            /* The base armor class is always known */
            (*p_ptr).dis_ac =
                ((*p_ptr).dis_ac as libc::c_int + (*o_ptr).ac as libc::c_int)
                    as s16b;
            /* Apply the bonuses to armor class */
            (*p_ptr).to_a =
                ((*p_ptr).to_a as libc::c_int + (*o_ptr).to_a as libc::c_int)
                    as s16b;
            /* Apply the mental bonuses to armor class, if known */
            if (*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
                   (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as
                       libc::c_int != 0 &&
                       (*k_info.offset((*o_ptr).k_idx as isize)).aware as
                           libc::c_int != 0 {
                (*p_ptr).dis_to_a =
                    ((*p_ptr).dis_to_a as libc::c_int +
                         (*o_ptr).to_a as libc::c_int) as s16b
            }
            /* Hack -- do not apply "weapon" bonuses */
            if !((*p_ptr).body_parts[(i - 24 as libc::c_int) as usize] as
                     libc::c_int == 24 as libc::c_int) {
                /* Hack -- do not apply "bow" bonuses */
                if !((*p_ptr).body_parts[(i - 24 as libc::c_int) as usize] as
                         libc::c_int == 27 as libc::c_int) {
                    /* Hack -- do not apply "ammo" bonuses */
                    if !((*p_ptr).body_parts[(i - 24 as libc::c_int) as usize]
                             as libc::c_int == 50 as libc::c_int) {
                        /* Hack -- do not apply "tool" bonuses */
                        if !((*p_ptr).body_parts[(i - 24 as libc::c_int) as
                                                     usize] as libc::c_int ==
                                 51 as libc::c_int) {
                            /* Apply the bonuses to hit/damage */
                            (*p_ptr).to_h =
                                ((*p_ptr).to_h as libc::c_int +
                                     (*o_ptr).to_h as libc::c_int) as s16b;
                            (*p_ptr).to_d =
                                ((*p_ptr).to_d as libc::c_int +
                                     (*o_ptr).to_d as libc::c_int) as s16b;
                            /* Apply the mental bonuses tp hit/damage, if known */
                            if (*o_ptr).ident as libc::c_int &
                                   0x8 as libc::c_int != 0 ||
                                   (*k_info.offset((*o_ptr).k_idx as
                                                       isize)).easy_know as
                                       libc::c_int != 0 &&
                                       (*k_info.offset((*o_ptr).k_idx as
                                                           isize)).aware as
                                           libc::c_int != 0 {
                                (*p_ptr).dis_to_h =
                                    ((*p_ptr).dis_to_h as libc::c_int +
                                         (*o_ptr).to_h as libc::c_int) as s16b
                            }
                            if (*o_ptr).ident as libc::c_int &
                                   0x8 as libc::c_int != 0 ||
                                   (*k_info.offset((*o_ptr).k_idx as
                                                       isize)).easy_know as
                                       libc::c_int != 0 &&
                                       (*k_info.offset((*o_ptr).k_idx as
                                                           isize)).aware as
                                           libc::c_int != 0 {
                                (*p_ptr).dis_to_d =
                                    ((*p_ptr).dis_to_d as libc::c_int +
                                         (*o_ptr).to_d as libc::c_int) as s16b
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    /* Monks get extra ac for armour _not worn_ */
    if (*p_ptr).melee_style as libc::c_int == 42 as libc::c_int &&
           monk_heavy_armor() == 0 {
        if (*p_ptr).inventory[37 as libc::c_int as usize].k_idx == 0 {
            (*p_ptr).to_a =
                ((*p_ptr).to_a as libc::c_int +
                     get_skill_scale(42 as libc::c_int,
                                     75 as libc::c_int as u32b) as
                         libc::c_int) as s16b;
            (*p_ptr).dis_to_a =
                ((*p_ptr).dis_to_a as libc::c_int +
                     get_skill_scale(42 as libc::c_int,
                                     75 as libc::c_int as u32b) as
                         libc::c_int) as s16b
        }
        if (*p_ptr).inventory[38 as libc::c_int as usize].k_idx == 0 &&
               get_skill(42 as libc::c_int) as libc::c_int > 15 as libc::c_int
           {
            (*p_ptr).to_a =
                ((*p_ptr).to_a as libc::c_int +
                     (get_skill(42 as libc::c_int) as libc::c_int -
                          13 as libc::c_int) / 3 as libc::c_int) as s16b;
            (*p_ptr).dis_to_a =
                ((*p_ptr).dis_to_a as libc::c_int +
                     (get_skill(42 as libc::c_int) as libc::c_int -
                          13 as libc::c_int) / 3 as libc::c_int) as s16b
        }
        if (*p_ptr).inventory[39 as libc::c_int as usize].k_idx == 0 &&
               get_skill(42 as libc::c_int) as libc::c_int > 10 as libc::c_int
           {
            (*p_ptr).to_a =
                ((*p_ptr).to_a as libc::c_int +
                     (get_skill(42 as libc::c_int) as libc::c_int -
                          8 as libc::c_int) / 3 as libc::c_int) as s16b;
            (*p_ptr).dis_to_a =
                ((*p_ptr).dis_to_a as libc::c_int +
                     (get_skill(42 as libc::c_int) as libc::c_int -
                          8 as libc::c_int) / 3 as libc::c_int) as s16b
        }
        if (*p_ptr).inventory[42 as libc::c_int as usize].k_idx == 0 &&
               get_skill(42 as libc::c_int) as libc::c_int > 4 as libc::c_int
           {
            (*p_ptr).to_a =
                ((*p_ptr).to_a as libc::c_int +
                     (get_skill(42 as libc::c_int) as libc::c_int -
                          2 as libc::c_int) / 3 as libc::c_int) as s16b;
            (*p_ptr).dis_to_a =
                ((*p_ptr).dis_to_a as libc::c_int +
                     (get_skill(42 as libc::c_int) as libc::c_int -
                          2 as libc::c_int) / 3 as libc::c_int) as s16b
        }
        if (*p_ptr).inventory[44 as libc::c_int as usize].k_idx == 0 {
            (*p_ptr).to_a =
                ((*p_ptr).to_a as libc::c_int +
                     get_skill(42 as libc::c_int) as libc::c_int /
                         2 as libc::c_int) as s16b;
            (*p_ptr).dis_to_a =
                ((*p_ptr).dis_to_a as libc::c_int +
                     get_skill(42 as libc::c_int) as libc::c_int /
                         2 as libc::c_int) as s16b
        }
        if (*p_ptr).inventory[47 as libc::c_int as usize].k_idx == 0 {
            (*p_ptr).to_a =
                ((*p_ptr).to_a as libc::c_int +
                     get_skill(42 as libc::c_int) as libc::c_int /
                         3 as libc::c_int) as s16b;
            (*p_ptr).dis_to_a =
                ((*p_ptr).dis_to_a as libc::c_int +
                     get_skill(42 as libc::c_int) as libc::c_int /
                         3 as libc::c_int) as s16b
        }
    }
    /* Hack -- aura of fire also provides light */
    if (*p_ptr).sh_fire != 0 { (*p_ptr).lite = 1 as libc::c_int as bool_ }
    if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
            (*spp_ptr).flags1) as libc::c_long & 0x80 as libc::c_long != 0 {
        (*p_ptr).to_a =
            ((*p_ptr).to_a as libc::c_int +
                 (20 as libc::c_int +
                      (*p_ptr).lev as libc::c_int / 5 as libc::c_int)) as
                s16b;
        (*p_ptr).dis_to_a =
            ((*p_ptr).dis_to_a as libc::c_int +
                 (20 as libc::c_int +
                      (*p_ptr).lev as libc::c_int / 5 as libc::c_int)) as s16b
    }
    /* Take care of gods */
    calc_gods();
    /* Calculate stats */
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        let mut top: libc::c_int = 0;
        let mut use_0: libc::c_int = 0;
        let mut ind: libc::c_int = 0;
        /* Extract the new "stat_use" value for the stat */
        top =
            modify_stat_value((*p_ptr).stat_max[i as usize] as libc::c_int,
                              (*p_ptr).stat_add[i as usize] as libc::c_int) as
                libc::c_int;
        /* Notice changes */
        if (*p_ptr).stat_top[i as usize] as libc::c_int != top {
            /* Save the new value */
            (*p_ptr).stat_top[i as usize] = top as s16b;
            /* Redisplay the stats later */
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x10 as libc::c_long) as
                    u32b;
            /* Window stuff */
            (*p_ptr).window =
                ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as
                    u32b
        }
        /* Extract the new "stat_use" value for the stat */
        use_0 =
            modify_stat_value((*p_ptr).stat_cur[i as usize] as libc::c_int,
                              (*p_ptr).stat_add[i as usize] as libc::c_int) as
                libc::c_int;
        /* Notice changes */
        if (*p_ptr).stat_use[i as usize] as libc::c_int != use_0 {
            /* Save the new value */
            (*p_ptr).stat_use[i as usize] = use_0 as s16b;
            /* Redisplay the stats later */
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x10 as libc::c_long) as
                    u32b;
            /* Window stuff */
            (*p_ptr).window =
                ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as
                    u32b
        }
        /* Values: 3, 4, ..., 17 */
        if use_0 <= 18 as libc::c_int {
            ind = use_0 - 3 as libc::c_int
        } else if use_0 <= 18 as libc::c_int + 219 as libc::c_int {
            ind =
                15 as libc::c_int +
                    (use_0 - 18 as libc::c_int) / 10 as libc::c_int
        } else {
            /* Ranges: 18/00-18/09, ..., 18/210-18/219 */
            /* Range: 18/220+ */
            ind = 37 as libc::c_int
        }
        /* Notice changes */
        if (*p_ptr).stat_ind[i as usize] as libc::c_int != ind {
            /* Save the new index */
            (*p_ptr).stat_ind[i as usize] = ind as s16b;
            /* Change in CON affects Hitpoints */
            if i == 4 as libc::c_int {
                (*p_ptr).update =
                    ((*p_ptr).update as libc::c_long | 0x10 as libc::c_long)
                        as u32b
            } else if i == 2 as libc::c_int {
                (*p_ptr).update =
                    ((*p_ptr).update as libc::c_long |
                         (0x20 as libc::c_long | 0x8 as libc::c_long)) as u32b
            }
            /* Change in WIS affects Sanity Points */
            /* Change in spell stat affects Mana/Spells */
            if i == 1 as libc::c_int {
                (*p_ptr).update =
                    ((*p_ptr).update as libc::c_long |
                         (0x20 as libc::c_long | 0x40 as libc::c_long)) as
                        u32b
            }
            /* Window stuff */
            (*p_ptr).window =
                ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as
                    u32b
        }
        i += 1
    }
    /* Provide the damage we get from sacrifice */
    if (*p_ptr).pgod as libc::c_int == 4 as libc::c_int {
        let mut x: libc::c_int = wisdom_scale(4 as libc::c_int);
        if x < 1 as libc::c_int { x = 1 as libc::c_int }
        (*p_ptr).dis_to_d =
            ((*p_ptr).dis_to_d as libc::c_int +
                 x * (*p_ptr).melkor_sacrifice as libc::c_int) as s16b;
        (*p_ptr).to_d =
            ((*p_ptr).to_d as libc::c_int +
                 x * (*p_ptr).melkor_sacrifice as libc::c_int) as s16b
    }
    /* jk - add in the tactics */
    (*p_ptr).dis_to_h =
        ((*p_ptr).dis_to_h as libc::c_int +
             tactic_info[(*p_ptr).tactic as byte_hack as usize].to_hit as
                 libc::c_int) as s16b;
    (*p_ptr).to_h =
        ((*p_ptr).to_h as libc::c_int +
             tactic_info[(*p_ptr).tactic as byte_hack as usize].to_hit as
                 libc::c_int) as s16b;
    (*p_ptr).dis_to_d =
        ((*p_ptr).dis_to_d as libc::c_int +
             tactic_info[(*p_ptr).tactic as byte_hack as usize].to_dam as
                 libc::c_int) as s16b;
    (*p_ptr).to_d =
        ((*p_ptr).to_d as libc::c_int +
             tactic_info[(*p_ptr).tactic as byte_hack as usize].to_dam as
                 libc::c_int) as s16b;
    (*p_ptr).dis_to_a =
        ((*p_ptr).dis_to_a as libc::c_int +
             tactic_info[(*p_ptr).tactic as byte_hack as usize].to_ac as
                 libc::c_int) as s16b;
    (*p_ptr).to_a =
        ((*p_ptr).to_a as libc::c_int +
             tactic_info[(*p_ptr).tactic as byte_hack as usize].to_ac as
                 libc::c_int) as s16b;
    (*p_ptr).skill_stl =
        ((*p_ptr).skill_stl as libc::c_int +
             tactic_info[(*p_ptr).tactic as byte_hack as usize].to_stealth as
                 libc::c_int) as s16b;
    (*p_ptr).skill_dis =
        ((*p_ptr).skill_dis as libc::c_int +
             tactic_info[(*p_ptr).tactic as byte_hack as usize].to_disarm as
                 libc::c_int) as s16b;
    (*p_ptr).skill_sav =
        ((*p_ptr).skill_sav as libc::c_int +
             tactic_info[(*p_ptr).tactic as byte_hack as usize].to_saving as
                 libc::c_int) as s16b;
    (*p_ptr).pspeed =
        ((*p_ptr).pspeed as libc::c_int +
             move_info[(*p_ptr).movement as byte_hack as usize].to_speed as
                 libc::c_int) as s16b;
    (*p_ptr).skill_srh =
        ((*p_ptr).skill_srh as libc::c_int +
             move_info[(*p_ptr).movement as byte_hack as usize].to_search as
                 libc::c_int) as s16b;
    (*p_ptr).skill_fos =
        ((*p_ptr).skill_fos as libc::c_int +
             move_info[(*p_ptr).movement as byte_hack as usize].to_percep as
                 libc::c_int) as s16b;
    (*p_ptr).skill_stl =
        ((*p_ptr).skill_stl as libc::c_int +
             move_info[(*p_ptr).movement as byte_hack as usize].to_stealth as
                 libc::c_int) as s16b;
    /* Apply temporary "stun" */
    if (*p_ptr).stun as libc::c_int > 50 as libc::c_int {
        (*p_ptr).to_h =
            ((*p_ptr).to_h as libc::c_int - 20 as libc::c_int) as s16b;
        (*p_ptr).dis_to_h =
            ((*p_ptr).dis_to_h as libc::c_int - 20 as libc::c_int) as s16b;
        (*p_ptr).to_d =
            ((*p_ptr).to_d as libc::c_int - 20 as libc::c_int) as s16b;
        (*p_ptr).dis_to_d =
            ((*p_ptr).dis_to_d as libc::c_int - 20 as libc::c_int) as s16b
    } else if (*p_ptr).stun != 0 {
        (*p_ptr).to_h =
            ((*p_ptr).to_h as libc::c_int - 5 as libc::c_int) as s16b;
        (*p_ptr).dis_to_h =
            ((*p_ptr).dis_to_h as libc::c_int - 5 as libc::c_int) as s16b;
        (*p_ptr).to_d =
            ((*p_ptr).to_d as libc::c_int - 5 as libc::c_int) as s16b;
        (*p_ptr).dis_to_d =
            ((*p_ptr).dis_to_d as libc::c_int - 5 as libc::c_int) as s16b
    }
    /* Invulnerability */
    if (*p_ptr).invuln != 0 {
        (*p_ptr).to_a =
            ((*p_ptr).to_a as libc::c_int + 100 as libc::c_int) as s16b;
        (*p_ptr).dis_to_a =
            ((*p_ptr).dis_to_a as libc::c_int + 100 as libc::c_int) as s16b
    }
    /* Breath */
    if (*p_ptr).tim_water_breath != 0 {
        (*p_ptr).water_breath = 1 as libc::c_int as bool_
    }
    if (*p_ptr).tim_magic_breath != 0 {
        (*p_ptr).magical_breath = 1 as libc::c_int as bool_
    }
    /* wraith_form */
    if (*p_ptr).tim_wraith != 0 {
        if (*p_ptr).disembodied != 0 {
            (*p_ptr).to_a =
                ((*p_ptr).to_a as libc::c_int + 10 as libc::c_int) as s16b;
            (*p_ptr).dis_to_a =
                ((*p_ptr).dis_to_a as libc::c_int + 10 as libc::c_int) as s16b
        } else {
            (*p_ptr).to_a =
                ((*p_ptr).to_a as libc::c_int + 50 as libc::c_int) as s16b;
            (*p_ptr).dis_to_a =
                ((*p_ptr).dis_to_a as libc::c_int + 50 as libc::c_int) as
                    s16b;
            (*p_ptr).reflect = 1 as libc::c_int as bool_
        }
        (*p_ptr).wraith_form = 1 as libc::c_int as bool_
    }
    /* Temporary holy aura */
    if (*p_ptr).holy != 0 {
        (*p_ptr).hold_life = 1 as libc::c_int as bool_;
        (*p_ptr).luck_cur =
            ((*p_ptr).luck_cur as libc::c_int + 5 as libc::c_int) as s16b
    }
    /* Temporary blessing */
    if (*p_ptr).blessed != 0 {
        (*p_ptr).to_a =
            ((*p_ptr).to_a as libc::c_int + 5 as libc::c_int) as s16b;
        (*p_ptr).dis_to_a =
            ((*p_ptr).dis_to_a as libc::c_int + 5 as libc::c_int) as s16b;
        (*p_ptr).to_h =
            ((*p_ptr).to_h as libc::c_int + 10 as libc::c_int) as s16b;
        (*p_ptr).dis_to_h =
            ((*p_ptr).dis_to_h as libc::c_int + 10 as libc::c_int) as s16b
    }
    /* Temporary invisibility */
    if (*p_ptr).tim_invisible != 0 {
        (*p_ptr).invis =
            ((*p_ptr).invis as libc::c_int +
                 (*p_ptr).tim_inv_pow as libc::c_int) as s16b
    }
    /* Temporary shield */
    if (*p_ptr).shield != 0 {
        (*p_ptr).to_a =
            ((*p_ptr).to_a as libc::c_int +
                 (*p_ptr).shield_power as libc::c_int) as s16b;
        (*p_ptr).dis_to_a =
            ((*p_ptr).dis_to_a as libc::c_int +
                 (*p_ptr).shield_power as libc::c_int) as s16b
    }
    /* Temporary "Hero" */
    if (*p_ptr).hero != 0 {
        (*p_ptr).to_h =
            ((*p_ptr).to_h as libc::c_int + 12 as libc::c_int) as s16b;
        (*p_ptr).dis_to_h =
            ((*p_ptr).dis_to_h as libc::c_int + 12 as libc::c_int) as s16b
    }
    /* Temporary "roots" */
    if (*p_ptr).tim_roots != 0 {
        set_stun(0 as libc::c_int);
        (*p_ptr).to_d_melee =
            ((*p_ptr).to_d_melee as libc::c_int +
                 (*p_ptr).tim_roots_dam as libc::c_int) as s16b;
        (*p_ptr).to_a =
            ((*p_ptr).to_a as libc::c_int +
                 (*p_ptr).tim_roots_ac as libc::c_int) as s16b;
        (*p_ptr).dis_to_a =
            ((*p_ptr).dis_to_a as libc::c_int +
                 (*p_ptr).tim_roots_ac as libc::c_int) as s16b
    }
    /* Temporary "Beserk" */
    if (*p_ptr).shero != 0 {
        (*p_ptr).to_h =
            ((*p_ptr).to_h as libc::c_int + 24 as libc::c_int) as s16b;
        (*p_ptr).dis_to_h =
            ((*p_ptr).dis_to_h as libc::c_int + 24 as libc::c_int) as s16b;
        (*p_ptr).to_a =
            ((*p_ptr).to_a as libc::c_int - 10 as libc::c_int) as s16b;
        (*p_ptr).dis_to_a =
            ((*p_ptr).dis_to_a as libc::c_int - 10 as libc::c_int) as s16b
    }
    /* Temporary "Accurancy" */
    if (*p_ptr).strike != 0 {
        (*p_ptr).to_d =
            ((*p_ptr).to_d as libc::c_int + 15 as libc::c_int) as s16b;
        (*p_ptr).dis_to_d =
            ((*p_ptr).dis_to_d as libc::c_int + 15 as libc::c_int) as s16b;
        (*p_ptr).to_h =
            ((*p_ptr).to_h as libc::c_int + 15 as libc::c_int) as s16b;
        (*p_ptr).dis_to_h =
            ((*p_ptr).dis_to_h as libc::c_int + 15 as libc::c_int) as s16b
    }
    /* Temporary "Meditation" */
    if (*p_ptr).meditation != 0 {
        (*p_ptr).to_d =
            ((*p_ptr).to_d as libc::c_int - 25 as libc::c_int) as s16b;
        (*p_ptr).dis_to_d =
            ((*p_ptr).dis_to_d as libc::c_int - 25 as libc::c_int) as s16b;
        (*p_ptr).to_h =
            ((*p_ptr).to_h as libc::c_int - 25 as libc::c_int) as s16b;
        (*p_ptr).dis_to_h =
            ((*p_ptr).dis_to_h as libc::c_int - 25 as libc::c_int) as s16b
    }
    /* Temporary "Reflection" */
    if (*p_ptr).tim_reflect != 0 {
        (*p_ptr).reflect = 1 as libc::c_int as bool_
    }
    /* Temporary "Time Resistance" */
    if (*p_ptr).tim_res_time != 0 {
        (*p_ptr).resist_continuum = 1 as libc::c_int as bool_
    }
    /* Temporary "Levitation" and "Flying" */
    if (*p_ptr).tim_ffall != 0 { (*p_ptr).ffall = 1 as libc::c_int as bool_ }
    if (*p_ptr).tim_fly != 0 { (*p_ptr).fly = 1 as libc::c_int as bool_ }
    /* Temporary "Fire Aura" */
    if (*p_ptr).tim_fire_aura != 0 {
        (*p_ptr).sh_fire = 1 as libc::c_int as bool_
    }
    /* Oppose Light & Dark */
    if (*p_ptr).oppose_ld != 0 {
        (*p_ptr).resist_lite = 1 as libc::c_int as bool_;
        (*p_ptr).resist_dark = 1 as libc::c_int as bool_
    }
    /* Oppose Chaos & Confusion */
    if (*p_ptr).oppose_cc != 0 {
        (*p_ptr).resist_chaos = 1 as libc::c_int as bool_;
        (*p_ptr).resist_conf = 1 as libc::c_int as bool_
    }
    /* Oppose Sound & Shards */
    if (*p_ptr).oppose_ss != 0 {
        (*p_ptr).resist_sound = 1 as libc::c_int as bool_;
        (*p_ptr).resist_shard = 1 as libc::c_int as bool_
    }
    /* Oppose Nexus */
    if (*p_ptr).oppose_nex != 0 {
        (*p_ptr).resist_nexus = 1 as libc::c_int as bool_
    }
    /* Mental barrier */
    if (*p_ptr).tim_mental_barrier != 0 {
        (*p_ptr).sustain_int = 1 as libc::c_int as bool_;
        (*p_ptr).sustain_wis = 1 as libc::c_int as bool_
    }
    /* Temporary "fast" */
    if (*p_ptr).fast != 0 {
        (*p_ptr).pspeed =
            ((*p_ptr).pspeed as libc::c_int +
                 (*p_ptr).speed_factor as libc::c_int) as s16b
    }
    /* Temporary "light speed" */
    if (*p_ptr).lightspeed != 0 {
        (*p_ptr).pspeed =
            ((*p_ptr).pspeed as libc::c_int + 50 as libc::c_int) as s16b
    }
    /* Temporary "slow" */
    if (*p_ptr).slow != 0 {
        (*p_ptr).pspeed =
            ((*p_ptr).pspeed as libc::c_int - 10 as libc::c_int) as s16b
    }
    if (*p_ptr).tim_esp != 0 {
        (*p_ptr).telepathy =
            ((*p_ptr).telepathy as libc::c_long | 0x80000000 as libc::c_long)
                as u32b
    }
    /* Temporary see invisible */
    if (*p_ptr).tim_invis != 0 {
        (*p_ptr).see_inv = 1 as libc::c_int as bool_
    }
    /* Temporary infravision boost */
    if (*p_ptr).tim_infra != 0 { (*p_ptr).see_infra += 1 }
    /* Hack -- Magic breath -> Water breath */
    if (*p_ptr).magical_breath != 0 {
        (*p_ptr).water_breath = 1 as libc::c_int as bool_
    }
    /* Hack -- Can Fly -> Can Levitate */
    if (*p_ptr).fly != 0 { (*p_ptr).ffall = 1 as libc::c_int as bool_ }
    /* Hack -- Res Chaos -> Res Conf */
    if (*p_ptr).resist_chaos != 0 {
        (*p_ptr).resist_conf = 1 as libc::c_int as bool_
    }
    /* Hack -- Hero/Shero -> Res fear */
    if (*p_ptr).hero as libc::c_int != 0 || (*p_ptr).shero as libc::c_int != 0
       {
        (*p_ptr).resist_fear = 1 as libc::c_int as bool_
    }
    /* Hack -- Telepathy Change */
    if (*p_ptr).telepathy != old_telepathy {
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long) as
                u32b
    }
    /* Hack -- See Invis Change */
    if (*p_ptr).see_inv as libc::c_int != old_see_inv {
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long) as
                u32b
    }
    /* Extract the current weight (in tenth pounds) */
    j = calc_total_weight();
    /* Extract the "weight limit" (in tenth pounds) */
    i = weight_limit();
    /* XXX XXX XXX Apply "encumbrance" from weight */
    if j > i / 2 as libc::c_int {
        (*p_ptr).pspeed =
            ((*p_ptr).pspeed as libc::c_int -
                 (j - i / 2 as libc::c_int) / (i / 10 as libc::c_int)) as s16b
    }
    /* Bloating slows the player down (a little) */
    if (*p_ptr).food as libc::c_int >= 15000 as libc::c_int {
        (*p_ptr).pspeed =
            ((*p_ptr).pspeed as libc::c_int - 10 as libc::c_int) as s16b
    }
    /* Searching slows the player down */
    if (*p_ptr).searching != 0 {
        (*p_ptr).pspeed =
            ((*p_ptr).pspeed as libc::c_int - 10 as libc::c_int) as s16b
    }
    /* In order to get a "nice" mana path druids need to ahve a 0 speed */
    if (*p_ptr).druid_extra2 == 1 as libc::c_int as libc::c_uint &&
           (*p_ptr).pspeed as libc::c_int > 110 as libc::c_int {
        (*p_ptr).pspeed = 110 as libc::c_int as s16b
    }
    /* Display the speed (if needed) */
    if (*p_ptr).pspeed as libc::c_int != old_speed {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x200000 as libc::c_long) as
                u32b
    }
    /* Actual Modifier Bonuses (Un-inflate stat bonuses) */
    (*p_ptr).to_a =
        ((*p_ptr).to_a as libc::c_int +
             (*adj_dex_ta.as_mut_ptr().offset((*p_ptr).stat_ind[3 as
                                                                    libc::c_int
                                                                    as usize]
                                                  as isize) as libc::c_int -
                  128 as libc::c_int)) as s16b;
    (*p_ptr).to_d =
        ((*p_ptr).to_d as libc::c_int +
             (*adj_str_td.as_mut_ptr().offset((*p_ptr).stat_ind[0 as
                                                                    libc::c_int
                                                                    as usize]
                                                  as isize) as libc::c_int -
                  128 as libc::c_int)) as s16b;
    (*p_ptr).to_h =
        ((*p_ptr).to_h as libc::c_int +
             (*adj_dex_th.as_mut_ptr().offset((*p_ptr).stat_ind[3 as
                                                                    libc::c_int
                                                                    as usize]
                                                  as isize) as libc::c_int -
                  128 as libc::c_int)) as s16b;
    (*p_ptr).to_h =
        ((*p_ptr).to_h as libc::c_int +
             (*adj_str_th.as_mut_ptr().offset((*p_ptr).stat_ind[0 as
                                                                    libc::c_int
                                                                    as usize]
                                                  as isize) as libc::c_int -
                  128 as libc::c_int)) as s16b;
    /* Displayed Modifier Bonuses (Un-inflate stat bonuses) */
    (*p_ptr).dis_to_a =
        ((*p_ptr).dis_to_a as libc::c_int +
             (*adj_dex_ta.as_mut_ptr().offset((*p_ptr).stat_ind[3 as
                                                                    libc::c_int
                                                                    as usize]
                                                  as isize) as libc::c_int -
                  128 as libc::c_int)) as s16b;
    (*p_ptr).dis_to_d =
        ((*p_ptr).dis_to_d as libc::c_int +
             (*adj_str_td.as_mut_ptr().offset((*p_ptr).stat_ind[0 as
                                                                    libc::c_int
                                                                    as usize]
                                                  as isize) as libc::c_int -
                  128 as libc::c_int)) as s16b;
    (*p_ptr).dis_to_h =
        ((*p_ptr).dis_to_h as libc::c_int +
             (*adj_dex_th.as_mut_ptr().offset((*p_ptr).stat_ind[3 as
                                                                    libc::c_int
                                                                    as usize]
                                                  as isize) as libc::c_int -
                  128 as libc::c_int)) as s16b;
    (*p_ptr).dis_to_h =
        ((*p_ptr).dis_to_h as libc::c_int +
             (*adj_str_th.as_mut_ptr().offset((*p_ptr).stat_ind[0 as
                                                                    libc::c_int
                                                                    as usize]
                                                  as isize) as libc::c_int -
                  128 as libc::c_int)) as s16b;
    /* Redraw armor (if needed) */
    if (*p_ptr).dis_ac as libc::c_int != old_dis_ac ||
           (*p_ptr).dis_to_a as libc::c_int != old_dis_to_a {
        /* Redraw */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x20 as libc::c_long) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b
    }
    /* Obtain the "hold" value */
    hold =
        *adj_str_hold.as_mut_ptr().offset((*p_ptr).stat_ind[0 as libc::c_int
                                                                as usize] as
                                              isize) as libc::c_int;
    /* Examine the "current bow" */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(27 as libc::c_int as
                                                         isize) as
            *mut object_type;
    /* Assume not heavy */
    (*p_ptr).heavy_shoot = 0 as libc::c_int as bool_;
    /* It is hard to carholdry a heavy bow */
    if hold < (*o_ptr).weight / 10 as libc::c_int {
        /* Hard to wield a heavy bow */
        (*p_ptr).to_h =
            ((*p_ptr).to_h as libc::c_int +
                 2 as libc::c_int *
                     (hold - (*o_ptr).weight / 10 as libc::c_int)) as s16b;
        (*p_ptr).dis_to_h =
            ((*p_ptr).dis_to_h as libc::c_int +
                 2 as libc::c_int *
                     (hold - (*o_ptr).weight / 10 as libc::c_int)) as s16b;
        /* Heavy Bow */
        (*p_ptr).heavy_shoot = 1 as libc::c_int as bool_
    }
    /* Take note of required "tval" for missiles */
    match (*o_ptr).sval as libc::c_int {
        2 => { (*p_ptr).tval_ammo = 16 as libc::c_int as byte_hack }
        12 | 13 => { (*p_ptr).tval_ammo = 17 as libc::c_int as byte_hack }
        23 | 24 => { (*p_ptr).tval_ammo = 18 as libc::c_int as byte_hack }
        _ => { }
    }
    /* Compute "extra shots" if needed */
    if (*o_ptr).k_idx as libc::c_int != 0 && (*p_ptr).heavy_shoot == 0 {
        let mut archery: libc::c_int = get_archery_skill();
        if archery != -(1 as libc::c_int) {
            (*p_ptr).to_h_ranged =
                ((*p_ptr).to_h_ranged as libc::c_int +
                     get_skill_scale(archery, 25 as libc::c_int as u32b) as
                         libc::c_int) as s16b;
            (*p_ptr).num_fire =
                ((*p_ptr).num_fire as libc::c_int +
                     get_skill(archery) as libc::c_int / 16 as libc::c_int) as
                    s16b;
            (*p_ptr).xtra_might =
                ((*p_ptr).xtra_might as libc::c_int +
                     get_skill(archery) as libc::c_int / 25 as libc::c_int) as
                    byte_hack;
            match archery {
                24 => {
                    if (*p_ptr).tval_ammo as libc::c_int == 16 as libc::c_int
                       {
                        (*p_ptr).xtra_might =
                            ((*p_ptr).xtra_might as libc::c_int +
                                 get_skill(archery) as libc::c_int /
                                     30 as libc::c_int) as byte_hack
                    }
                }
                25 => {
                    if (*p_ptr).tval_ammo as libc::c_int == 17 as libc::c_int
                       {
                        (*p_ptr).xtra_might =
                            ((*p_ptr).xtra_might as libc::c_int +
                                 get_skill(archery) as libc::c_int /
                                     30 as libc::c_int) as byte_hack
                    }
                }
                26 => {
                    if (*p_ptr).tval_ammo as libc::c_int == 18 as libc::c_int
                       {
                        (*p_ptr).xtra_might =
                            ((*p_ptr).xtra_might as libc::c_int +
                                 get_skill(archery) as libc::c_int /
                                     30 as libc::c_int) as byte_hack
                    }
                }
                _ => { }
            }
        }
        /* Add in the "bonus shots" */
        (*p_ptr).num_fire =
            ((*p_ptr).num_fire as libc::c_int + extra_shots) as s16b;
        /* Require at least one shot */
        if ((*p_ptr).num_fire as libc::c_int) < 1 as libc::c_int {
            (*p_ptr).num_fire = 1 as libc::c_int as s16b
        }
    }
    if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
            (*spp_ptr).flags1) as libc::c_long & 0x10 as libc::c_long != 0 &&
           (*p_ptr).tval_ammo as libc::c_int == 17 as libc::c_int {
        (*p_ptr).xtra_might =
            ((*p_ptr).xtra_might as libc::c_int + 1 as libc::c_int) as
                byte_hack
    }
    if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
            (*spp_ptr).flags1) as libc::c_long & 0x40 as libc::c_long != 0 &&
           (*p_ptr).tval_ammo as libc::c_int == 16 as libc::c_int {
        (*p_ptr).xtra_might =
            ((*p_ptr).xtra_might as libc::c_int + 1 as libc::c_int) as
                byte_hack
    }
    if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
            (*spp_ptr).flags1) as libc::c_long & 0x20 as libc::c_long != 0 &&
           (*p_ptr).tval_ammo as libc::c_int == 18 as libc::c_int {
        (*p_ptr).xtra_might =
            ((*p_ptr).xtra_might as libc::c_int + 1 as libc::c_int) as
                byte_hack
    }
    /* Examine the "current tool" */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(51 as libc::c_int as
                                                         isize) as
            *mut object_type;
    /* Boost digging skill by tool weight */
    if (*o_ptr).k_idx as libc::c_int != 0 &&
           (*o_ptr).tval as libc::c_int == 20 as libc::c_int {
        (*p_ptr).skill_dig =
            ((*p_ptr).skill_dig as libc::c_int +
                 (*o_ptr).weight / 10 as libc::c_int) as s16b
    }
    /* Examine the main weapon */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(24 as libc::c_int as
                                                         isize) as
            *mut object_type;
    /* Assume not heavy */
    (*p_ptr).heavy_wield = 0 as libc::c_int as bool_;
    /* Normal weapons */
    if (*o_ptr).k_idx as libc::c_int != 0 && (*p_ptr).heavy_wield == 0 {
        let mut str_index: libc::c_int = 0;
        let mut dex_index: libc::c_int = 0;
        let mut num: libc::c_int = 0 as libc::c_int;
        let mut wgt: libc::c_int = 0 as libc::c_int;
        let mut mul: libc::c_int = 0 as libc::c_int;
        let mut div: libc::c_int = 0 as libc::c_int;
        analyze_blow(&mut num, &mut wgt, &mut mul);
        /* Enforce a minimum "weight" (tenth pounds) */
        div = if (*o_ptr).weight < wgt { wgt } else { (*o_ptr).weight };
        /* Access the strength vs weight */
        str_index =
            *adj_str_blow.as_mut_ptr().offset((*p_ptr).stat_ind[0 as
                                                                    libc::c_int
                                                                    as usize]
                                                  as isize) as libc::c_int *
                mul / div;
        /* Maximal value */
        if str_index > 11 as libc::c_int { str_index = 11 as libc::c_int }
        /* Index by dexterity */
        dex_index =
            *adj_dex_blow.as_mut_ptr().offset((*p_ptr).stat_ind[3 as
                                                                    libc::c_int
                                                                    as usize]
                                                  as isize) as libc::c_int;
        /* Maximal value */
        if dex_index > 11 as libc::c_int { dex_index = 11 as libc::c_int }
        /* Use the blows table */
        (*p_ptr).num_blow =
            blows_table[str_index as usize][dex_index as usize] as s16b;
        /* Maximal value */
        if (*p_ptr).num_blow as libc::c_int > num {
            (*p_ptr).num_blow = num as s16b
        }
        /* Add in the "bonus blows" */
        (*p_ptr).num_blow =
            ((*p_ptr).num_blow as libc::c_int + extra_blows) as s16b;
        /* Special class bonus blows */
        (*p_ptr).num_blow =
            ((*p_ptr).num_blow as libc::c_int +
                 (*p_ptr).lev as libc::c_int *
                     (*cp_ptr).extra_blows as libc::c_int / 50 as libc::c_int)
                as s16b;
        /* Weapon specialization bonus blows */
        if get_weaponmastery_skill() != -(1 as libc::c_int) {
            (*p_ptr).num_blow =
                ((*p_ptr).num_blow as libc::c_int +
                     get_skill_scale(get_weaponmastery_skill(),
                                     2 as libc::c_int as u32b) as libc::c_int)
                    as s16b
        }
        /* Bonus blows for plain weaponmastery skill */
        (*p_ptr).num_blow =
            ((*p_ptr).num_blow as libc::c_int +
                 get_skill_scale(17 as libc::c_int, 3 as libc::c_int as u32b)
                     as libc::c_int) as s16b;
        /* Require at least one blow */
        if ((*p_ptr).num_blow as libc::c_int) < 1 as libc::c_int {
            (*p_ptr).num_blow = 1 as libc::c_int as s16b
        }
    } else if (*p_ptr).melee_style as libc::c_int == 42 as libc::c_int &&
                  monk_empty_hands() as libc::c_int != 0 {
        let mut plev: libc::c_int =
            get_skill(42 as libc::c_int) as libc::c_int;
        (*p_ptr).num_blow = get_extra_blows_ability() as s16b;
        if plev > 9 as libc::c_int { (*p_ptr).num_blow += 1 }
        if plev > 19 as libc::c_int { (*p_ptr).num_blow += 1 }
        if plev > 29 as libc::c_int { (*p_ptr).num_blow += 1 }
        if plev > 34 as libc::c_int { (*p_ptr).num_blow += 1 }
        if plev > 39 as libc::c_int { (*p_ptr).num_blow += 1 }
        if plev > 44 as libc::c_int { (*p_ptr).num_blow += 1 }
        if plev > 49 as libc::c_int { (*p_ptr).num_blow += 1 }
        if monk_heavy_armor() != 0 {
            (*p_ptr).num_blow =
                ((*p_ptr).num_blow as libc::c_int / 2 as libc::c_int) as s16b
        }
        (*p_ptr).num_blow =
            ((*p_ptr).num_blow as libc::c_int +
                 (1 as libc::c_int + extra_blows)) as s16b;
        if monk_heavy_armor() == 0 {
            (*p_ptr).to_h =
                ((*p_ptr).to_h as libc::c_int + plev / 3 as libc::c_int) as
                    s16b;
            (*p_ptr).to_d =
                ((*p_ptr).to_d as libc::c_int + plev / 3 as libc::c_int) as
                    s16b;
            (*p_ptr).dis_to_h =
                ((*p_ptr).dis_to_h as libc::c_int + plev / 3 as libc::c_int)
                    as s16b;
            (*p_ptr).dis_to_d =
                ((*p_ptr).dis_to_d as libc::c_int + plev / 3 as libc::c_int)
                    as s16b
        }
    } else if (*r_info.offset((*p_ptr).body_monster as
                                  isize)).body_parts[0 as libc::c_int as
                                                         usize] == 0 {
        let mut str_index_0: libc::c_int = 0;
        let mut dex_index_0: libc::c_int = 0;
        let mut num_0: libc::c_int = 0 as libc::c_int;
        let mut wgt_0: libc::c_int = 0 as libc::c_int;
        let mut mul_0: libc::c_int = 0 as libc::c_int;
        let mut r_ptr: *mut monster_race =
            race_info_idx((*p_ptr).body_monster as libc::c_int,
                          0 as libc::c_int);
        analyze_blow(&mut num_0, &mut wgt_0, &mut mul_0);
        /* Different calculation for bear form with empty hands */
        /* Monsters that only have their "natural" attacks */
        /* Access the strength vs weight */
        str_index_0 =
            *adj_str_blow.as_mut_ptr().offset((*p_ptr).stat_ind[0 as
                                                                    libc::c_int
                                                                    as usize]
                                                  as isize) as libc::c_int *
                mul_0 / 3 as libc::c_int;
        /* Maximal value */
        if str_index_0 > 11 as libc::c_int { str_index_0 = 11 as libc::c_int }
        /* Index by dexterity */
        dex_index_0 =
            *adj_dex_blow.as_mut_ptr().offset((*p_ptr).stat_ind[3 as
                                                                    libc::c_int
                                                                    as usize]
                                                  as isize) as libc::c_int;
        /* Maximal value */
        if dex_index_0 > 11 as libc::c_int { dex_index_0 = 11 as libc::c_int }
        /* Use the blows table */
        (*p_ptr).num_blow =
            blows_table[str_index_0 as usize][dex_index_0 as usize] as s16b;
        /* Add in the "bonus blows" */
        (*p_ptr).num_blow =
            ((*p_ptr).num_blow as libc::c_int + extra_blows) as s16b;
        /* Maximal value */
        if (*p_ptr).num_blow as libc::c_int > 4 as libc::c_int {
            (*p_ptr).num_blow = 4 as libc::c_int as s16b
        }
        /* Require at least one blow */
        if ((*p_ptr).num_blow as libc::c_int) < 1 as libc::c_int {
            (*p_ptr).num_blow = 1 as libc::c_int as s16b
        }
        /* Limit as defined by monster body */
        num_0 = 0 as libc::c_int;
        while num_0 < (*p_ptr).num_blow as libc::c_int {
            if (*r_ptr).blow[num_0 as usize].effect == 0 { break ; }
            num_0 += 1
        }
        (*p_ptr).num_blow = num_0 as s16b
    } else if (*p_ptr).body_monster == 0 &&
                  (*p_ptr).mimic_form as libc::c_int ==
                      resolve_mimic_name(b"Bear\x00" as *const u8 as
                                             *const libc::c_char) &&
                  (*p_ptr).melee_style as libc::c_int == 47 as libc::c_int {
        let mut plev_0: libc::c_int =
            get_skill(47 as libc::c_int) as libc::c_int;
        (*p_ptr).num_blow = 0 as libc::c_int as s16b;
        (*p_ptr).num_blow =
            ((*p_ptr).num_blow as libc::c_int +
                 (2 as libc::c_int + plev_0 / 5 as libc::c_int + extra_blows))
                as s16b;
        (*p_ptr).to_h =
            ((*p_ptr).to_h as libc::c_int - plev_0 / 5 as libc::c_int) as
                s16b;
        (*p_ptr).dis_to_h =
            ((*p_ptr).dis_to_h as libc::c_int - plev_0 / 5 as libc::c_int) as
                s16b;
        (*p_ptr).to_d =
            ((*p_ptr).to_d as libc::c_int + plev_0 / 2 as libc::c_int) as
                s16b;
        (*p_ptr).dis_to_d =
            ((*p_ptr).dis_to_d as libc::c_int + plev_0 / 2 as libc::c_int) as
                s16b
    }
    /* Different calculation for monks with empty hands */
    /* Assume okay */
    (*p_ptr).icky_wield = 0 as libc::c_int as bool_;
    monk_armour_aux = 0 as libc::c_int as bool_;
    if get_weaponmastery_skill() != -(1 as libc::c_int) {
        let mut lev: libc::c_int =
            get_skill(get_weaponmastery_skill()) as libc::c_int;
        (*p_ptr).to_h_melee =
            ((*p_ptr).to_h_melee as libc::c_int + lev) as s16b;
        (*p_ptr).to_d_melee =
            ((*p_ptr).to_d_melee as libc::c_int + lev / 2 as libc::c_int) as
                s16b
    }
    if get_skill(16 as libc::c_int) != 0 {
        let mut lev_0: libc::c_int =
            get_skill_scale(16 as libc::c_int, 10 as libc::c_int as u32b) as
                libc::c_int;
        (*p_ptr).to_d = ((*p_ptr).to_d as libc::c_int + lev_0) as s16b;
        (*p_ptr).dis_to_d = ((*p_ptr).dis_to_d as libc::c_int + lev_0) as s16b
    }
    if get_skill(46 as libc::c_int) != 0 {
        /* Get the armor weight */
        let mut cur_wgt: libc::c_int = 0 as libc::c_int;
        cur_wgt += (*p_ptr).inventory[37 as libc::c_int as usize].weight;
        cur_wgt += (*p_ptr).inventory[42 as libc::c_int as usize].weight;
        cur_wgt += (*p_ptr).inventory[39 as libc::c_int as usize].weight;
        cur_wgt += (*p_ptr).inventory[38 as libc::c_int as usize].weight;
        cur_wgt += (*p_ptr).inventory[44 as libc::c_int as usize].weight;
        cur_wgt += (*p_ptr).inventory[47 as libc::c_int as usize].weight;
        /* Base dodge chance */
        (*p_ptr).dodge_chance =
            (get_skill_scale(46 as libc::c_int, 150 as libc::c_int as u32b) as
                 libc::c_int + get_skill(42 as libc::c_int) as libc::c_int) as
                s16b;
        /* Armor weight bonus/penalty */
        (*p_ptr).dodge_chance =
            ((*p_ptr).dodge_chance as libc::c_int -
                 cur_wgt * 2 as libc::c_int) as s16b;
        /* Encumberance bonus/penalty */
        (*p_ptr).dodge_chance =
            ((*p_ptr).dodge_chance as libc::c_int -
                 calc_total_weight() / 100 as libc::c_int) as s16b;
        /* Never below 0 */
        if ((*p_ptr).dodge_chance as libc::c_int) < 0 as libc::c_int {
            (*p_ptr).dodge_chance = 0 as libc::c_int as s16b
        }
    } else { (*p_ptr).dodge_chance = 0 as libc::c_int as s16b }
    /* Parse all the weapons */
    i = 0 as libc::c_int;
    while (*p_ptr).body_parts[i as usize] as libc::c_int == 24 as libc::c_int
          {
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset((24 as libc::c_int +
                                                              i) as isize) as
                *mut object_type;
        /* 2handed weapon and shield = less damage */
        if (*p_ptr).inventory[(24 as libc::c_int + i) as usize].k_idx as
               libc::c_int != 0 &&
               (*p_ptr).inventory[(39 as libc::c_int + i) as usize].k_idx as
                   libc::c_int != 0 {
            /* Extract the item flags */
            object_flags(&mut *(*p_ptr).inventory.as_mut_ptr().offset((24 as
                                                                           libc::c_int
                                                                           +
                                                                           i)
                                                                          as
                                                                          isize),
                         &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                         &mut esp);
            if f4 as libc::c_long & 0x40 as libc::c_long != 0 {
                let mut tmp: libc::c_int = 0;
                /* Reduce the bonuses */
                tmp = (*o_ptr).to_h as libc::c_int / 2 as libc::c_int;
                if tmp < 0 as libc::c_int { tmp = -tmp }
                (*p_ptr).to_h_melee =
                    ((*p_ptr).to_h_melee as libc::c_int - tmp) as s16b;
                tmp = (*o_ptr).to_d as libc::c_int / 2 as libc::c_int;
                if tmp < 0 as libc::c_int { tmp = -tmp }
                tmp +=
                    (*o_ptr).dd as libc::c_int * (*o_ptr).ds as libc::c_int /
                        2 as libc::c_int;
                (*p_ptr).to_d_melee =
                    ((*p_ptr).to_d_melee as libc::c_int - tmp) as s16b
            }
        }
        /* Priest weapon penalty for non-blessed edged weapons */
        if forbid_non_blessed() as libc::c_int != 0 &&
               (*p_ptr).bless_blade == 0 &&
               ((*o_ptr).tval as libc::c_int == 24 as libc::c_int ||
                    (*o_ptr).tval as libc::c_int == 23 as libc::c_int ||
                    (*o_ptr).tval as libc::c_int == 22 as libc::c_int) &&
               (*o_ptr).k_idx as libc::c_int != 0 {
            /* Reduce the real bonuses */
            (*p_ptr).to_h =
                ((*p_ptr).to_h as libc::c_int - 15 as libc::c_int) as s16b;
            (*p_ptr).to_d =
                ((*p_ptr).to_d as libc::c_int - 15 as libc::c_int) as s16b;
            /* Reduce the mental bonuses */
            (*p_ptr).dis_to_h =
                ((*p_ptr).dis_to_h as libc::c_int - 15 as libc::c_int) as
                    s16b;
            (*p_ptr).dis_to_d =
                ((*p_ptr).dis_to_d as libc::c_int - 15 as libc::c_int) as
                    s16b;
            /* Icky weapon */
            (*p_ptr).icky_wield = 1 as libc::c_int as bool_
        }
        /* Sorcerer can't wield a weapon unless it's a mage staff */
        if get_skill(41 as libc::c_int) != 0 {
            let mut malus: libc::c_int =
                get_skill_scale(41 as libc::c_int, 100 as libc::c_int as u32b)
                    as libc::c_int;
            if (*o_ptr).tval as libc::c_int != 6 as libc::c_int &&
                   (*o_ptr).k_idx as libc::c_int != 0 {
                /* Reduce the real bonuses */
                (*p_ptr).to_h =
                    ((*p_ptr).to_h as libc::c_int - malus) as s16b;
                (*p_ptr).to_d =
                    ((*p_ptr).to_d as libc::c_int - malus) as s16b;
                /* Reduce the mental bonuses */
                (*p_ptr).dis_to_h =
                    ((*p_ptr).dis_to_h as libc::c_int - malus) as s16b;
                (*p_ptr).dis_to_d =
                    ((*p_ptr).dis_to_d as libc::c_int - malus) as s16b;
                /* Icky weapon */
                (*p_ptr).icky_wield = 1 as libc::c_int as bool_
            } else {
                /* Reduce the real bonuses */
                (*p_ptr).to_h =
                    ((*p_ptr).to_h as libc::c_int - malus / 10 as libc::c_int)
                        as s16b;
                (*p_ptr).to_d =
                    ((*p_ptr).to_d as libc::c_int - malus / 10 as libc::c_int)
                        as s16b;
                /* Reduce the mental bonuses */
                (*p_ptr).dis_to_h =
                    ((*p_ptr).dis_to_h as libc::c_int -
                         malus / 10 as libc::c_int) as s16b;
                (*p_ptr).dis_to_d =
                    ((*p_ptr).dis_to_d as libc::c_int -
                         malus / 10 as libc::c_int) as s16b
            }
        }
        /* Check next weapon */
        i += 1
    }
    if monk_heavy_armor() != 0 { monk_armour_aux = 1 as libc::c_int as bool_ }
    /* Affect Skill -- stealth (bonus one) */
    (*p_ptr).skill_stl =
        ((*p_ptr).skill_stl as libc::c_int + 1 as libc::c_int) as s16b;
    /* Affect Skill -- disarming (DEX and INT) */
    (*p_ptr).skill_dis =
        ((*p_ptr).skill_dis as libc::c_int +
             *adj_dex_dis.as_mut_ptr().offset((*p_ptr).stat_ind[3 as
                                                                    libc::c_int
                                                                    as usize]
                                                  as isize) as libc::c_int) as
            s16b;
    (*p_ptr).skill_dis =
        ((*p_ptr).skill_dis as libc::c_int +
             *adj_int_dis.as_mut_ptr().offset((*p_ptr).stat_ind[1 as
                                                                    libc::c_int
                                                                    as usize]
                                                  as isize) as libc::c_int) as
            s16b;
    /* Affect Skill -- magic devices (INT) */
    (*p_ptr).skill_dev =
        ((*p_ptr).skill_dev as libc::c_int +
             get_skill_scale(56 as libc::c_int, 20 as libc::c_int as u32b) as
                 libc::c_int) as s16b;
    /* Affect Skill -- saving throw (WIS) */
    (*p_ptr).skill_sav =
        ((*p_ptr).skill_sav as libc::c_int +
             *adj_wis_sav.as_mut_ptr().offset((*p_ptr).stat_ind[2 as
                                                                    libc::c_int
                                                                    as usize]
                                                  as isize) as libc::c_int) as
            s16b;
    /* Affect Skill -- digging (STR) */
    (*p_ptr).skill_dig =
        ((*p_ptr).skill_dig as libc::c_int +
             *adj_str_dig.as_mut_ptr().offset((*p_ptr).stat_ind[0 as
                                                                    libc::c_int
                                                                    as usize]
                                                  as isize) as libc::c_int) as
            s16b;
    /* Affect Skill -- disarming (skill) */
    (*p_ptr).skill_dis =
        ((*p_ptr).skill_dis as libc::c_int +
             get_skill_scale(37 as libc::c_int, 75 as libc::c_int as u32b) as
                 libc::c_int) as s16b;
    /* Affect Skill -- magic devices (skill) */
    (*p_ptr).skill_dev =
        ((*p_ptr).skill_dev as libc::c_int +
             get_skill_scale(56 as libc::c_int, 150 as libc::c_int as u32b) as
                 libc::c_int) as s16b;
    /* Affect Skill -- saving throw (skill and level) */
    (*p_ptr).skill_sav =
        ((*p_ptr).skill_sav as libc::c_int +
             get_skill_scale(28 as libc::c_int, 75 as libc::c_int as u32b) as
                 libc::c_int) as s16b;
    /* Affect Skill -- stealth (skill) */
    (*p_ptr).skill_stl =
        ((*p_ptr).skill_stl as libc::c_int +
             get_skill_scale(36 as libc::c_int, 25 as libc::c_int as u32b) as
                 libc::c_int) as s16b;
    /* Affect Skill -- search ability (Sneakiness skill) */
    (*p_ptr).skill_srh =
        ((*p_ptr).skill_srh as libc::c_int +
             get_skill_scale(35 as libc::c_int, 35 as libc::c_int as u32b) as
                 libc::c_int) as s16b;
    /* Affect Skill -- search frequency (Sneakiness skill) */
    (*p_ptr).skill_fos =
        ((*p_ptr).skill_fos as libc::c_int +
             get_skill_scale(35 as libc::c_int, 25 as libc::c_int as u32b) as
                 libc::c_int) as s16b;
    /* Affect Skill -- combat (Combat skill + mastery) */
    (*p_ptr).skill_thn =
        ((*p_ptr).skill_thn as libc::c_int +
             50 as libc::c_int *
                 ((7 as libc::c_int *
                       get_skill((*p_ptr).melee_style as libc::c_int) as
                           libc::c_int +
                       3 as libc::c_int *
                           get_skill(16 as libc::c_int) as libc::c_int) /
                      10 as libc::c_int) / 10 as libc::c_int) as s16b;
    /* Affect Skill -- combat (shooting) (Level, by Class) */
    (*p_ptr).skill_thb =
        ((*p_ptr).skill_thb as libc::c_int +
             50 as libc::c_int *
                 ((7 as libc::c_int *
                       get_skill(23 as libc::c_int) as libc::c_int +
                       3 as libc::c_int *
                           get_skill(16 as libc::c_int) as libc::c_int) /
                      10 as libc::c_int) / 10 as libc::c_int) as s16b;
    /* Affect Skill -- combat (throwing) (Level) */
    (*p_ptr).skill_tht =
        ((*p_ptr).skill_tht as libc::c_int +
             50 as libc::c_int * (*p_ptr).lev as libc::c_int /
                 10 as libc::c_int) as s16b;
    /* Limit Skill -- stealth from 0 to 30 */
    if (*p_ptr).skill_stl as libc::c_int > 30 as libc::c_int {
        (*p_ptr).skill_stl = 30 as libc::c_int as s16b
    }
    if ((*p_ptr).skill_stl as libc::c_int) < 0 as libc::c_int {
        (*p_ptr).skill_stl = 0 as libc::c_int as s16b
    }
    /* Limit Skill -- digging from 1 up */
    if ((*p_ptr).skill_dig as libc::c_int) < 1 as libc::c_int {
        (*p_ptr).skill_dig = 1 as libc::c_int as s16b
    }
    if (*p_ptr).anti_magic as libc::c_int != 0 &&
           ((*p_ptr).skill_sav as libc::c_int) < 95 as libc::c_int {
        (*p_ptr).skill_sav = 95 as libc::c_int as s16b
    }
    /* Hack -- handle "xtra" mode */
    if character_xtra != 0 { return }
    /* Take note when "heavy bow" changes */
    if (*p_ptr).old_heavy_shoot as libc::c_int !=
           (*p_ptr).heavy_shoot as libc::c_int {
        if !(silent != 0) {
            /* Message */
            if (*p_ptr).heavy_shoot != 0 {
                msg_print(b"You have trouble wielding such a heavy bow.\x00"
                              as *const u8 as *const libc::c_char);
            } else if (*p_ptr).inventory[27 as libc::c_int as usize].k_idx !=
                          0 {
                msg_print(b"You have no trouble wielding your bow.\x00" as
                              *const u8 as *const libc::c_char);
            } else {
                msg_print(b"You feel relieved to put down your heavy bow.\x00"
                              as *const u8 as *const libc::c_char);
            }
        }
        /* Save it */
        (*p_ptr).old_heavy_shoot = (*p_ptr).heavy_shoot
    }
    /* Take note when "heavy weapon" changes */
    if (*p_ptr).old_heavy_wield as libc::c_int !=
           (*p_ptr).heavy_wield as libc::c_int {
        if !(silent != 0) {
            /* Message */
            if (*p_ptr).heavy_wield != 0 {
                msg_print(b"You have trouble wielding such a heavy weapon.\x00"
                              as *const u8 as *const libc::c_char);
            } else if (*p_ptr).inventory[24 as libc::c_int as usize].k_idx !=
                          0 {
                msg_print(b"You have no trouble wielding your weapon.\x00" as
                              *const u8 as *const libc::c_char);
            } else {
                msg_print(b"You feel relieved to put down your heavy weapon.\x00"
                              as *const u8 as *const libc::c_char);
            }
        }
        /* Save it */
        (*p_ptr).old_heavy_wield = (*p_ptr).heavy_wield
    }
    /* Take note when "illegal weapon" changes */
    if (*p_ptr).old_icky_wield as libc::c_int !=
           (*p_ptr).icky_wield as libc::c_int {
        if !(silent != 0) {
            /* Message */
            if (*p_ptr).icky_wield != 0 {
                msg_print(b"You do not feel comfortable with your weapon.\x00"
                              as *const u8 as *const libc::c_char);
            } else if (*p_ptr).inventory[24 as libc::c_int as usize].k_idx !=
                          0 {
                msg_print(b"You feel comfortable with your weapon.\x00" as
                              *const u8 as *const libc::c_char);
            } else {
                msg_print(b"You feel more comfortable after removing your weapon.\x00"
                              as *const u8 as *const libc::c_char);
            }
        }
        /* Save it */
        (*p_ptr).old_icky_wield = (*p_ptr).icky_wield
    }
    if monk_armour_aux as libc::c_int != monk_notify_aux as libc::c_int {
        if !((*p_ptr).melee_style as libc::c_int != 42 as libc::c_int ||
                 silent as libc::c_int != 0) {
            if monk_heavy_armor() != 0 {
                msg_print(b"The weight of your armor disrupts your balance.\x00"
                              as *const u8 as *const libc::c_char);
            } else {
                msg_print(b"You regain your balance.\x00" as *const u8 as
                              *const libc::c_char);
            }
        }
        monk_notify_aux = monk_armour_aux
    }
    /* Resist lite & senseible lite negates one an other */
    if (*p_ptr).resist_lite as libc::c_int != 0 &&
           (*p_ptr).sensible_lite as libc::c_int != 0 {
        (*p_ptr).sensible_lite = 0 as libc::c_int as bool_;
        (*p_ptr).resist_lite = (*p_ptr).sensible_lite
    }
    /* resistance to fire cancel sensibility to fire */
    if (*p_ptr).resist_fire as libc::c_int != 0 ||
           (*p_ptr).oppose_fire as libc::c_int != 0 ||
           (*p_ptr).immune_fire as libc::c_int != 0 {
        (*p_ptr).sensible_fire = 0 as libc::c_int as bool_
    }
    /* Minimum saving throw */
    if (*p_ptr).skill_sav as libc::c_int <= 10 as libc::c_int {
        (*p_ptr).skill_sav = 10 as libc::c_int as s16b
    } else {
        (*p_ptr).skill_sav =
            ((*p_ptr).skill_sav as libc::c_int + 10 as libc::c_int) as s16b
    }
    /* Let the scripts do what they need */
    process_hooks(77 as libc::c_int,
                  b"(d)\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, silent as libc::c_int);
}
/*
 * Handle "p_ptr->notice"
 */
#[no_mangle]
pub unsafe extern "C" fn notice_stuff() {
    /* Notice stuff */
    if (*p_ptr).notice == 0 { return }
    /* Combine the pack */
    if (*p_ptr).notice as libc::c_long & 0x1 as libc::c_long != 0 {
        (*p_ptr).notice =
            ((*p_ptr).notice as libc::c_long & !(0x1 as libc::c_long)) as
                u32b;
        combine_pack();
    }
    /* Reorder the pack */
    if (*p_ptr).notice as libc::c_long & 0x2 as libc::c_long != 0 {
        (*p_ptr).notice =
            ((*p_ptr).notice as libc::c_long & !(0x2 as libc::c_long)) as
                u32b;
        reorder_pack();
    };
}
/*
 * Handle "p_ptr->update"
 */
#[no_mangle]
pub unsafe extern "C" fn update_stuff() {
    /* Update stuff */
    if (*p_ptr).update == 0 { return }
    if (*p_ptr).update as libc::c_long & 0x4 as libc::c_long != 0 {
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long & !(0x4 as libc::c_long)) as
                u32b;
        calc_body();
    }
    if (*p_ptr).update as libc::c_long & 0x1 as libc::c_long != 0 {
        /* Ok now THAT is an ugly hack */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long & !(0x80 as libc::c_long)) as
                u32b;
        calc_powers();
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long & !(0x1 as libc::c_long)) as
                u32b;
        calc_bonuses(0 as libc::c_int as bool_);
    }
    if (*p_ptr).update as libc::c_long & 0x2 as libc::c_long != 0 {
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long & !(0x2 as libc::c_long)) as
                u32b;
        calc_torch();
    }
    if (*p_ptr).update as libc::c_long & 0x10 as libc::c_long != 0 {
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long & !(0x10 as libc::c_long)) as
                u32b;
        calc_hitpoints();
    }
    if (*p_ptr).update as libc::c_long & 0x8 as libc::c_long != 0 {
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long & !(0x8 as libc::c_long)) as
                u32b;
        calc_sanity();
    }
    if (*p_ptr).update as libc::c_long & 0x20 as libc::c_long != 0 {
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long & !(0x20 as libc::c_long)) as
                u32b;
        calc_mana();
    }
    if (*p_ptr).update as libc::c_long & 0x40 as libc::c_long != 0 {
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long & !(0x40 as libc::c_long)) as
                u32b;
        calc_spells();
    }
    if (*p_ptr).update as libc::c_long & 0x80 as libc::c_long != 0 {
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long & !(0x80 as libc::c_long)) as
                u32b;
        calc_powers();
    }
    /* Character is not ready yet, no screen updates */
    if character_generated == 0 { return }
    /* Character is in "icky" mode, no screen updates */
    if character_icky != 0 { return }
    if (*p_ptr).update as libc::c_long & 0x10000 as libc::c_long != 0 {
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long & !(0x10000 as libc::c_long)) as
                u32b;
        forget_view();
    }
    if (*p_ptr).update as libc::c_long & 0x100000 as libc::c_long != 0 {
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long & !(0x100000 as libc::c_long)) as
                u32b;
        update_view();
    }
    if (*p_ptr).update as libc::c_long & 0x10000000 as libc::c_long != 0 {
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long & !(0x10000000 as libc::c_long))
                as u32b;
        update_flow();
    }
    if (*p_ptr).update as libc::c_long & 0x2000000 as libc::c_long != 0 {
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long & !(0x2000000 as libc::c_long))
                as u32b;
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long & !(0x1000000 as libc::c_long))
                as u32b;
        update_monsters(1 as libc::c_int as bool_);
    }
    if (*p_ptr).update as libc::c_long & 0x1000000 as libc::c_long != 0 {
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long & !(0x1000000 as libc::c_long))
                as u32b;
        update_monsters(0 as libc::c_int as bool_);
    }
    if (*p_ptr).update as libc::c_long & 0x200000 as libc::c_long != 0 {
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long & !(0x200000 as libc::c_long)) as
                u32b;
        if monster_lite != 0 { update_mon_lite(); }
    };
}
/*
 * Handle "p_ptr->redraw"
 */
#[no_mangle]
pub unsafe extern "C" fn redraw_stuff() {
    /* Redraw stuff */
    if (*p_ptr).redraw == 0 { return }
    /* Character is not ready yet, no screen updates */
    if character_generated == 0 { return }
    /* Character is in "icky" mode, no screen updates */
    if character_icky != 0 { return }
    /* Should we tell lua to redisplay too ? */
    process_hooks(51 as libc::c_int,
                  b"()\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char);
    /* Hack -- clear the screen */
    if (*p_ptr).redraw as libc::c_long & 0x8000000 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x8000000 as libc::c_long))
                as u32b;
        msg_print(0 as cptr);
        Term_clear();
    }
    if (*p_ptr).redraw as libc::c_long & 0x4000000 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x4000000 as libc::c_long))
                as u32b;
        prt_map();
    }
    if (*p_ptr).redraw as libc::c_long & 0x2000000 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x2000000 as libc::c_long))
                as u32b;
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long &
                 !(0x1 as libc::c_long | 0x2 as libc::c_long |
                       0x10 as libc::c_long)) as u32b;
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long &
                 !(0x4 as libc::c_long | 0x8 as libc::c_long |
                       0x100 as libc::c_long)) as u32b;
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long &
                 !(0x20 as libc::c_long | 0x40 as libc::c_long |
                       0x80 as libc::c_long | 0x8000 as libc::c_long |
                       0x10000000 as libc::c_long)) as u32b;
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long &
                 !(0x200 as libc::c_long | 0x800 as libc::c_long)) as u32b;
        prt_frame_basic();
    }
    if (*p_ptr).redraw as libc::c_long & 0x1 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x1 as libc::c_long)) as
                u32b;
        prt_field(rp_name.offset((*rp_ptr).title as isize) as cptr,
                  1 as libc::c_int, 0 as libc::c_int);
        prt_field(c_name.offset((*spp_ptr).title as isize) as cptr,
                  2 as libc::c_int, 0 as libc::c_int);
    }
    if (*p_ptr).redraw as libc::c_long & 0x2 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x2 as libc::c_long)) as
                u32b;
        prt_title();
    }
    if (*p_ptr).redraw as libc::c_long & 0x4 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x4 as libc::c_long)) as
                u32b;
        prt_level();
    }
    if (*p_ptr).redraw as libc::c_long & 0x8 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x8 as libc::c_long)) as
                u32b;
        prt_exp();
    }
    if (*p_ptr).redraw as libc::c_long & 0x10 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x10 as libc::c_long)) as
                u32b;
        prt_stat(0 as libc::c_int);
        prt_stat(1 as libc::c_int);
        prt_stat(2 as libc::c_int);
        prt_stat(3 as libc::c_int);
        prt_stat(4 as libc::c_int);
        prt_stat(5 as libc::c_int);
    }
    if (*p_ptr).redraw as libc::c_long & 0x20 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x20 as libc::c_long)) as
                u32b;
        prt_ac();
    }
    if (*p_ptr).redraw as libc::c_long & 0x40 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x40 as libc::c_long)) as
                u32b;
        prt_hp();
    }
    if (*p_ptr).redraw as libc::c_long & 0x80 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x80 as libc::c_long)) as
                u32b;
        prt_sp();
    }
    if (*p_ptr).redraw as libc::c_long & 0x8000 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x8000 as libc::c_long)) as
                u32b;
        prt_piety();
    }
    if (*p_ptr).redraw as libc::c_long & 0x10000000 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x10000000 as libc::c_long))
                as u32b;
        prt_mh();
    }
    if (*p_ptr).redraw as libc::c_long & 0x100 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x100 as libc::c_long)) as
                u32b;
        prt_gold();
    }
    if (*p_ptr).redraw as libc::c_long & 0x200 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x200 as libc::c_long)) as
                u32b;
        prt_depth();
    }
    if (*p_ptr).redraw as libc::c_long & 0x800 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x800 as libc::c_long)) as
                u32b;
        health_redraw();
    }
    if (*p_ptr).redraw as libc::c_long & 0x1000000 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x1000000 as libc::c_long))
                as u32b;
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long &
                 !(0x1000 as libc::c_long | 0x2000 as libc::c_long)) as u32b;
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x4000 as libc::c_long)) as
                u32b;
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long &
                 !(0x10000 as libc::c_long | 0x20000 as libc::c_long)) as
                u32b;
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long &
                 !(0x40000 as libc::c_long | 0x80000 as libc::c_long)) as
                u32b;
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long &
                 !(0x100000 as libc::c_long | 0x200000 as libc::c_long |
                       0x400000 as libc::c_long | 0x800000 as libc::c_long))
                as u32b;
        prt_frame_extra();
    }
    if (*p_ptr).redraw as libc::c_long & 0x1000 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x1000 as libc::c_long)) as
                u32b;
        prt_cut();
    }
    if (*p_ptr).redraw as libc::c_long & 0x2000 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x2000 as libc::c_long)) as
                u32b;
        prt_stun();
    }
    if (*p_ptr).redraw as libc::c_long & 0x4000 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x4000 as libc::c_long)) as
                u32b;
        prt_hunger();
    }
    if (*p_ptr).redraw as libc::c_long & 0x10000 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x10000 as libc::c_long)) as
                u32b;
        prt_blind();
    }
    if (*p_ptr).redraw as libc::c_long & 0x20000 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x20000 as libc::c_long)) as
                u32b;
        prt_confused();
    }
    if (*p_ptr).redraw as libc::c_long & 0x40000 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x40000 as libc::c_long)) as
                u32b;
        prt_afraid();
    }
    if (*p_ptr).redraw as libc::c_long & 0x80000 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x80000 as libc::c_long)) as
                u32b;
        prt_poisoned();
    }
    if (*p_ptr).redraw as libc::c_long & 0x20000000 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x20000000 as libc::c_long))
                as u32b;
        prt_dtrap();
    }
    if (*p_ptr).redraw as libc::c_long & 0x100000 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x100000 as libc::c_long)) as
                u32b;
        prt_state();
    }
    if (*p_ptr).redraw as libc::c_long & 0x200000 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x200000 as libc::c_long)) as
                u32b;
        prt_speed();
    }
    if (*p_ptr).redraw as libc::c_long & 0x400000 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x400000 as libc::c_long)) as
                u32b;
        prt_study();
    }
    if (*p_ptr).redraw as libc::c_long & 0x800000 as libc::c_long != 0 {
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long & !(0x800000 as libc::c_long)) as
                u32b;
        prt_sane();
    };
}
/*
 * Handle "p_ptr->window"
 */
#[no_mangle]
pub unsafe extern "C" fn window_stuff() {
    let mut j: libc::c_int = 0;
    let mut mask: u32b = 0 as libc::c_long as u32b;
    /* Nothing to do */
    if (*p_ptr).window == 0 { return }
    /* Scan windows */
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        /* Save usable flags */
        if !angband_term[j as usize].is_null() {
            mask |= window_flag[j as usize]
        }
        j += 1
    }
    /* Apply usable flags */
    (*p_ptr).window &= mask;
    /* Nothing to do */
    if (*p_ptr).window == 0 { return }
    /* Display p_ptr->inventory */
    if (*p_ptr).window as libc::c_long & 0x1 as libc::c_long != 0 {
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long & !(0x1 as libc::c_long)) as
                u32b;
        fix_inven();
    }
    /* Display equipment */
    if (*p_ptr).window as libc::c_long & 0x2 as libc::c_long != 0 {
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long & !(0x2 as libc::c_long)) as
                u32b;
        fix_equip();
    }
    /* Display player */
    if (*p_ptr).window as libc::c_long & 0x8 as libc::c_long != 0 {
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long & !(0x8 as libc::c_long)) as
                u32b;
        fix_player();
    }
    /* Display monster list */
    if (*p_ptr).window as libc::c_long & 0x10 as libc::c_long != 0 {
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long & !(0x10 as libc::c_long)) as
                u32b;
        fix_m_list();
    }
    /* Display overhead view */
    if (*p_ptr).window as libc::c_long & 0x40 as libc::c_long != 0 {
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long & !(0x40 as libc::c_long)) as
                u32b;
        fix_message();
    }
    /* Display overhead view */
    if (*p_ptr).window as libc::c_long & 0x80 as libc::c_long != 0 {
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long & !(0x80 as libc::c_long)) as
                u32b;
        fix_overhead();
    }
    /* Display monster recall */
    if (*p_ptr).window as libc::c_long & 0x100 as libc::c_long != 0 {
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long & !(0x100 as libc::c_long)) as
                u32b;
        fix_monster();
    }
    /* Display object recall */
    if (*p_ptr).window as libc::c_long & 0x200 as libc::c_long != 0 {
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long & !(0x200 as libc::c_long)) as
                u32b;
        fix_object();
    };
}
/*
 * Handle "p_ptr->update" and "p_ptr->redraw" and "p_ptr->window"
 */
#[no_mangle]
pub unsafe extern "C" fn handle_stuff() {
    /* Update stuff */
    if (*p_ptr).update != 0 { update_stuff(); }
    /* Redraw stuff */
    if (*p_ptr).redraw != 0 { redraw_stuff(); }
    /* Window stuff */
    if (*p_ptr).window != 0 { window_stuff(); };
}
#[no_mangle]
pub unsafe extern "C" fn monk_empty_hands() -> bool_ {
    let mut i: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    if (*p_ptr).melee_style as libc::c_int != 42 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    i = 0 as libc::c_int;
    while (*p_ptr).body_parts[i as usize] as libc::c_int == 24 as libc::c_int
          {
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset((24 as libc::c_int +
                                                              i) as isize) as
                *mut object_type;
        if (*o_ptr).k_idx != 0 { return 0 as libc::c_int as bool_ }
        i += 1
    }
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn monk_heavy_armor() -> bool_ {
    let mut monk_arm_wgt: u16b = 0 as libc::c_int as u16b;
    if (*p_ptr).melee_style as libc::c_int != 42 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Weight the armor */
    monk_arm_wgt =
        (monk_arm_wgt as libc::c_int +
             (*p_ptr).inventory[37 as libc::c_int as usize].weight) as u16b;
    monk_arm_wgt =
        (monk_arm_wgt as libc::c_int +
             (*p_ptr).inventory[42 as libc::c_int as usize].weight) as u16b;
    monk_arm_wgt =
        (monk_arm_wgt as libc::c_int +
             (*p_ptr).inventory[39 as libc::c_int as usize].weight) as u16b;
    monk_arm_wgt =
        (monk_arm_wgt as libc::c_int +
             (*p_ptr).inventory[38 as libc::c_int as usize].weight) as u16b;
    monk_arm_wgt =
        (monk_arm_wgt as libc::c_int +
             (*p_ptr).inventory[44 as libc::c_int as usize].weight) as u16b;
    monk_arm_wgt =
        (monk_arm_wgt as libc::c_int +
             (*p_ptr).inventory[47 as libc::c_int as usize].weight) as u16b;
    return (monk_arm_wgt as libc::c_int >
                100 as libc::c_int +
                    get_skill(42 as libc::c_int) as libc::c_int *
                        4 as libc::c_int) as libc::c_int as bool_;
}
unsafe extern "C" fn get_artifact_idx(mut level: libc::c_int) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    while count < 1000 as libc::c_int {
        let mut a_ptr: *mut artifact_type = 0 as *mut artifact_type;
        count += 1;
        i =
            Rand_div(max_a_idx as libc::c_int - 1 as libc::c_int) +
                1 as libc::c_int;
        a_ptr = &mut *a_info.offset(i as isize) as *mut artifact_type;
        if (*a_ptr).tval == 0 { continue ; }
        /* It is found/lost */
        if (*a_ptr).cur_num != 0 { continue ; }
        /* OoD */
        if (*a_ptr).level as libc::c_int > level { continue ; }
        /* Avoid granting SPECIAL_GENE artifacts */
        if (*a_ptr).flags4 as libc::c_long & 0x400 as libc::c_long != 0 {
            continue ;
        }
        return i
    }
    /* No matches found */
	/* Grant a randart */
    return 0 as libc::c_int;
}
/* Chose a fate */
#[no_mangle]
pub unsafe extern "C" fn gain_fate(mut fate: byte_hack) {
    let mut i: libc::c_int = 0;
    let mut level: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        if fates[i as usize].fate == 0 {
            fates[i as usize].level = 0 as libc::c_int as byte_hack;
            cmsg_print(10 as libc::c_int as byte_hack,
                       b"More of your prophecy has been unearthed!\x00" as
                           *const u8 as *const libc::c_char);
            cmsg_print(10 as libc::c_int as byte_hack,
                       b"You should see a soothsayer quickly.\x00" as
                           *const u8 as *const libc::c_char);
            if fate != 0 {
                fates[i as usize].fate = fate
            } else {
                /* If lucky (current luck > 0) avoid death fate */
                match Rand_div(if (*p_ptr).luck_cur as libc::c_int >
                                      0 as libc::c_int {
                                   17 as libc::c_int
                               } else { 18 as libc::c_int }) {
                    6 | 2 | 3 | 7 | 8 | 9 | 13 => {
                        fates[i as usize].fate = 1 as libc::c_int as byte_hack
                    }
                    1 | 4 | 5 | 10 | 11 | 12 | 14 => {
                        fates[i as usize].fate = 4 as libc::c_int as byte_hack
                    }
                    15 | 16 => {
                        fates[i as usize].fate = 3 as libc::c_int as byte_hack
                    }
                    17 => {
                        fates[i as usize].fate = 6 as libc::c_int as byte_hack
                    }
                    0 => {
                        /* The deepest the better */
                        let mut chance: libc::c_int =
                            dun_level as libc::c_int / 4 as libc::c_int;
                        /* No more than 1/2 chances */
                        if chance > 50 as libc::c_int {
                            chance = 50 as libc::c_int
                        }
                        /* It's HARD to get now */
                        if Rand_div(100 as libc::c_int) < chance {
                            fates[i as usize].fate =
                                2 as libc::c_int as byte_hack
                        } else {
                            fates[i as usize].fate =
                                1 as libc::c_int as byte_hack
                        }
                    }
                    _ => { }
                }
            }
            match fates[i as usize].fate as libc::c_int {
                1 => {
                    loop  {
                        let mut k_ptr: *mut object_kind =
                            0 as *mut object_kind;
                        let mut theme: obj_theme =
                            obj_theme{treasure: 0,
                                      combat: 0,
                                      magic: 0,
                                      tools: 0,};
                        /* No themes */
                        theme.treasure = 100 as libc::c_int as byte_hack;
                        theme.combat = 100 as libc::c_int as byte_hack;
                        theme.magic = 100 as libc::c_int as byte_hack;
                        theme.tools = 100 as libc::c_int as byte_hack;
                        init_match_theme(theme);
                        /* Apply restriction */
                        get_obj_num_hook =
                            Some(kind_is_legal as
                                     unsafe extern "C" fn(_: libc::c_int)
                                         -> bool_);
                        /* Rebuild allocation table */
                        get_obj_num_prep();
                        fates[i as usize].o_idx =
                            get_obj_num(*max_dlv.offset(dungeon_type as isize)
                                            as libc::c_int +
                                            (Rand_div(10 as libc::c_int) +
                                                 1 as libc::c_int));
                        /* Invalidate the cached allocation table */
                        alloc_kind_table_valid = 0 as libc::c_int as bool_;
                        k_ptr =
                            &mut *k_info.offset((*fates.as_mut_ptr().offset(i
                                                                                as
                                                                                isize)).o_idx
                                                    as isize) as
                                *mut object_kind;
                        if (*k_ptr).flags3 as libc::c_long &
                               0x800 as libc::c_long == 0 &&
                               (*k_ptr).flags3 as libc::c_long &
                                   0x8000 as libc::c_long == 0 {
                            break ;
                        }
                    }
                    level =
                        *max_dlv.offset(dungeon_type as isize) as libc::c_int
                            - 20 as libc::c_int +
                            Rand_div(1 as libc::c_int +
                                         (*max_dlv.offset(dungeon_type as
                                                              isize) as
                                              libc::c_int + 20 as libc::c_int)
                                         -
                                         (*max_dlv.offset(dungeon_type as
                                                              isize) as
                                              libc::c_int -
                                              20 as libc::c_int));
                    fates[i as usize].level =
                        if level < 1 as libc::c_int {
                            1 as libc::c_int
                        } else if level > 98 as libc::c_int {
                            98 as libc::c_int
                        } else { level } as byte_hack;
                    fates[i as usize].serious =
                        Rand_div(2 as libc::c_int) as byte_hack;
                    fates[i as usize].know = 0 as libc::c_int as bool_;
                    if wizard != 0 {
                        msg_format(b"New fate : Find object %d on level %d\x00"
                                       as *const u8 as *const libc::c_char,
                                   fates[i as usize].o_idx as libc::c_int,
                                   fates[i as usize].level as libc::c_int);
                    }
                }
                4 => {
                    /* Prepare allocation table */
                    get_mon_num_prep();
                    fates[i as usize].r_idx =
                        get_mon_num(*max_dlv.offset(dungeon_type as isize) as
                                        libc::c_int +
                                        (Rand_div(10 as libc::c_int) +
                                             1 as libc::c_int));
                    level =
                        *max_dlv.offset(dungeon_type as isize) as libc::c_int
                            - 20 as libc::c_int +
                            Rand_div(1 as libc::c_int +
                                         (*max_dlv.offset(dungeon_type as
                                                              isize) as
                                              libc::c_int + 20 as libc::c_int)
                                         -
                                         (*max_dlv.offset(dungeon_type as
                                                              isize) as
                                              libc::c_int -
                                              20 as libc::c_int));
                    fates[i as usize].level =
                        if level < 1 as libc::c_int {
                            1 as libc::c_int
                        } else if level > 98 as libc::c_int {
                            98 as libc::c_int
                        } else { level } as byte_hack;
                    fates[i as usize].serious =
                        Rand_div(2 as libc::c_int) as byte_hack;
                    fates[i as usize].know = 0 as libc::c_int as bool_;
                    if wizard != 0 {
                        msg_format(b"New fate : Meet monster %d on level %d\x00"
                                       as *const u8 as *const libc::c_char,
                                   fates[i as usize].r_idx as libc::c_int,
                                   fates[i as usize].level as libc::c_int);
                    }
                }
                3 => {
                    fates[i as usize].a_idx =
                        get_artifact_idx(*max_dlv.offset(dungeon_type as
                                                             isize) as
                                             libc::c_int +
                                             (Rand_div(10 as libc::c_int) +
                                                  1 as libc::c_int)) as s16b;
                    level =
                        *max_dlv.offset(dungeon_type as isize) as libc::c_int
                            - 20 as libc::c_int +
                            Rand_div(1 as libc::c_int +
                                         (*max_dlv.offset(dungeon_type as
                                                              isize) as
                                              libc::c_int + 20 as libc::c_int)
                                         -
                                         (*max_dlv.offset(dungeon_type as
                                                              isize) as
                                              libc::c_int -
                                              20 as libc::c_int));
                    fates[i as usize].level =
                        if level < 1 as libc::c_int {
                            1 as libc::c_int
                        } else if level > 98 as libc::c_int {
                            98 as libc::c_int
                        } else { level } as byte_hack;
                    fates[i as usize].serious = 1 as libc::c_int as byte_hack;
                    fates[i as usize].know = 0 as libc::c_int as bool_;
                    if wizard != 0 {
                        msg_format(b"New fate : Find artifact %d on level %d\x00"
                                       as *const u8 as *const libc::c_char,
                                   fates[i as usize].a_idx as libc::c_int,
                                   fates[i as usize].level as libc::c_int);
                    }
                }
                6 => {
                    level =
                        *max_dlv.offset(dungeon_type as isize) as libc::c_int
                            - 20 as libc::c_int +
                            Rand_div(1 as libc::c_int +
                                         (*max_dlv.offset(dungeon_type as
                                                              isize) as
                                              libc::c_int + 20 as libc::c_int)
                                         -
                                         (*max_dlv.offset(dungeon_type as
                                                              isize) as
                                              libc::c_int -
                                              20 as libc::c_int));
                    fates[i as usize].level =
                        if level < 1 as libc::c_int {
                            1 as libc::c_int
                        } else if level > 98 as libc::c_int {
                            98 as libc::c_int
                        } else { level } as byte_hack;
                    fates[i as usize].serious = 1 as libc::c_int as byte_hack;
                    fates[i as usize].know = 0 as libc::c_int as bool_;
                    if wizard as libc::c_int != 0 ||
                           (*p_ptr).precognition as libc::c_int != 0 {
                        msg_format(b"New fate : Death on level %d\x00" as
                                       *const u8 as *const libc::c_char,
                                   fates[i as usize].level as libc::c_int);
                    }
                }
                2 => {
                    fates[i as usize].serious = 1 as libc::c_int as byte_hack;
                    (*p_ptr).no_mortal = 1 as libc::c_int as bool_;
                    if wizard as libc::c_int != 0 ||
                           (*p_ptr).precognition as libc::c_int != 0 {
                        msg_format(b"New fate : Never to die by the hand of a mortal being.\x00"
                                       as *const u8 as *const libc::c_char);
                    }
                }
                _ => { }
            }
            break ;
        } else { i += 1 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn fate_desc(mut desc: *mut libc::c_char,
                                   mut fate: libc::c_int) {
    let mut buf: [libc::c_char; 120] = [0; 120];
    if fates[fate as usize].serious != 0 {
        strcpy(desc,
               b"You are fated to \x00" as *const u8 as *const libc::c_char);
    } else {
        strcpy(desc, b"You may \x00" as *const u8 as *const libc::c_char);
    }
    match fates[fate as usize].fate as libc::c_int {
        1 => {
            let mut o_ptr: *mut object_type = 0 as *mut object_type;
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
            let mut o_name: [libc::c_char; 80] = [0; 80];
            o_ptr = &mut forge;
            object_prep(o_ptr, fates[fate as usize].o_idx as libc::c_int);
            object_desc_store(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                              0 as libc::c_int);
            sprintf(buf.as_mut_ptr(),
                    b"find %s on level %d.\x00" as *const u8 as
                        *const libc::c_char, o_name.as_mut_ptr(),
                    fates[fate as usize].level as libc::c_int);
            strcat(desc, buf.as_mut_ptr());
        }
        3 => {
            let mut q_ptr: *mut object_type = 0 as *mut object_type;
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
            let mut o_name_0: [libc::c_char; 80] = [0; 80];
            let mut a_ptr: *mut artifact_type =
                &mut *a_info.offset((*fates.as_mut_ptr().offset(fate as
                                                                    isize)).a_idx
                                        as isize) as *mut artifact_type;
            let mut I_kind: libc::c_int = 0;
            /* Failed artefact allocation XXX XXX XXX */
            if fates[fate as usize].a_idx as libc::c_int == 0 as libc::c_int {
                strcpy(o_name_0.as_mut_ptr(),
                       b"something special\x00" as *const u8 as
                           *const libc::c_char);
            } else {
                /* Legal artefacts */
                /* Get local object */
                q_ptr = &mut forge_0;
                object_wipe(q_ptr);
                I_kind =
                    lookup_kind((*a_ptr).tval as libc::c_int,
                                (*a_ptr).sval as libc::c_int) as libc::c_int;
                object_prep(q_ptr, I_kind);
                (*q_ptr).name1 = fates[fate as usize].a_idx as byte_hack;
                (*q_ptr).pval = (*a_ptr).pval as s32b;
                (*q_ptr).ac = (*a_ptr).ac;
                (*q_ptr).dd = (*a_ptr).dd;
                (*q_ptr).ds = (*a_ptr).ds;
                (*q_ptr).to_a = (*a_ptr).to_a;
                (*q_ptr).to_h = (*a_ptr).to_h;
                (*q_ptr).to_d = (*a_ptr).to_d;
                (*q_ptr).weight = (*a_ptr).weight as s32b;
                if (*a_ptr).flags3 as libc::c_long &
                       0x20000000 as libc::c_long != 0 {
                    (*q_ptr).ident =
                        ((*q_ptr).ident as libc::c_int | 0x40 as libc::c_int)
                            as byte_hack
                }
                random_artifact_resistance(q_ptr);
                object_desc_store(o_name_0.as_mut_ptr(), q_ptr,
                                  1 as libc::c_int, 0 as libc::c_int);
            }
            sprintf(buf.as_mut_ptr(),
                    b"find %s on level %d.\x00" as *const u8 as
                        *const libc::c_char, o_name_0.as_mut_ptr(),
                    fates[fate as usize].level as libc::c_int);
            strcat(desc, buf.as_mut_ptr());
        }
        4 => {
            let mut m_name: [libc::c_char; 80] = [0; 80];
            monster_race_desc(m_name.as_mut_ptr(),
                              fates[fate as usize].r_idx as libc::c_int,
                              0 as libc::c_int);
            sprintf(buf.as_mut_ptr(),
                    b"meet %s on level %d.\x00" as *const u8 as
                        *const libc::c_char, m_name.as_mut_ptr(),
                    fates[fate as usize].level as libc::c_int);
            strcat(desc, buf.as_mut_ptr());
        }
        6 => {
            sprintf(buf.as_mut_ptr(),
                    b"die on level %d.\x00" as *const u8 as
                        *const libc::c_char,
                    fates[fate as usize].level as libc::c_int);
            strcat(desc, buf.as_mut_ptr());
        }
        2 => {
            strcat(desc,
                   b"never to die by the hand of a mortal being.\x00" as
                       *const u8 as *const libc::c_char);
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dump_fates(mut outfile: *mut FILE) {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 120] = [0; 120];
    let mut pending: bool_ = 0 as libc::c_int as bool_;
    if outfile.is_null() { return }
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        if fates[i as usize].fate as libc::c_int != 0 &&
               fates[i as usize].know as libc::c_int != 0 {
            fate_desc(buf.as_mut_ptr(), i);
            fprintf(outfile, b"%s\n\x00" as *const u8 as *const libc::c_char,
                    buf.as_mut_ptr());
        }
        if fates[i as usize].fate as libc::c_int != 0 &&
               fates[i as usize].know == 0 {
            pending = 1 as libc::c_int as bool_
        }
        i += 1
    }
    if pending != 0 {
        fprintf(outfile,
                b"You do not know all of your fate.\n\x00" as *const u8 as
                    *const libc::c_char);
    };
}
/* Wipe the object */
/* Acquire the "kind" index */
/* Create the artifact */
/* Save the name */
/* Extract the fields */
/* Hack -- acquire "cursed" flag */
/*
 * Return a luck number between a certain range
 */
#[no_mangle]
pub unsafe extern "C" fn luck(mut min: libc::c_int, mut max: libc::c_int)
 -> libc::c_int {
    let mut luck_0: libc::c_int = (*p_ptr).luck_cur as libc::c_int;
    let mut range: libc::c_int = max - min;
    if luck_0 < -(30 as libc::c_int) { luck_0 = -(30 as libc::c_int) }
    if luck_0 > 30 as libc::c_int { luck_0 = 30 as libc::c_int }
    luck_0 += 30 as libc::c_int;
    luck_0 *= range;
    luck_0 /= 60 as libc::c_int;
    return luck_0 + min;
}
