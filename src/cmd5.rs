use ::libc;
extern "C" {
    pub type lua_State;
    #[no_mangle]
    static mut ddx: [s16b; 10];
    #[no_mangle]
    static mut ddy: [s16b; 10];
    #[no_mangle]
    static mut adj_str_wgt: [byte_hack; 0];
    #[no_mangle]
    static mut sex_info: [player_sex; 3];
    #[no_mangle]
    static mut monster_powers: [monster_power; 96];
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut energy_use: s32b;
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut summon_kin_type: libc::c_char;
    #[no_mangle]
    static mut flush_failure: bool_;
    #[no_mangle]
    static mut show_choices: bool_;
    #[no_mangle]
    static mut target_col: s16b;
    #[no_mangle]
    static mut target_row: s16b;
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut o_list: *mut object_type;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
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
    fn Term_save() -> errr;
    #[no_mangle]
    fn Term_load() -> errr;
    #[no_mangle]
    fn toupper(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    static mut f_info: *mut feature_type;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut r_name: *mut libc::c_char;
    #[no_mangle]
    static mut race_info: *mut player_race;
    #[no_mangle]
    static mut rp_name: *mut libc::c_char;
    #[no_mangle]
    static mut item_tester_hook:
           Option<unsafe extern "C" fn(_: *mut object_type) -> bool_>;
    #[no_mangle]
    static mut max_rp_idx: u16b;
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut max_dlv: *mut s16b;
    #[no_mangle]
    static mut dungeon_flags2: u32b;
    #[no_mangle]
    fn get_height_weight();
    #[no_mangle]
    fn distance(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int,
                x2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn note_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn lite_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn is_quest(level: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn target_okay() -> bool_;
    #[no_mangle]
    fn window_stuff();
    #[no_mangle]
    fn exec_lua(file: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn call_lua(function: cptr, args: cptr, ret: cptr, _: ...) -> bool_;
    #[no_mangle]
    fn bell();
    #[no_mangle]
    fn get_com(prompt: cptr, command: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn get_object(item: libc::c_int) -> *mut object_type;
    #[no_mangle]
    fn get_item(cp: *mut libc::c_int, pmt: cptr, str: cptr, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn do_cmd_rerate();
    #[no_mangle]
    fn corrupt_player();
    #[no_mangle]
    fn set_cut(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn take_hit(damage: libc::c_int, kb_str: cptr);
    #[no_mangle]
    fn hp_player(num: libc::c_int) -> bool_;
    #[no_mangle]
    fn gain_random_corruption(choose_mut: libc::c_int) -> bool_;
    #[no_mangle]
    fn dec_stat(stat: libc::c_int, amount: libc::c_int, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn handle_stuff();
    #[no_mangle]
    fn check_experience();
    #[no_mangle]
    fn is_a_vowel(ch: libc::c_int) -> bool_;
    #[no_mangle]
    fn lose_corruption(choose_mut: libc::c_int) -> bool_;
    #[no_mangle]
    fn enchant(o_ptr: *mut object_type, n: libc::c_int, eflag: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn apply_magic(o_ptr: *mut object_type, lev: libc::c_int, okay: bool_,
                   good: bool_, great: bool_);
    #[no_mangle]
    fn flush();
    #[no_mangle]
    fn quark_str(num: s16b) -> cptr;
    #[no_mangle]
    fn get_skill(skill: libc::c_int) -> s16b;
    #[no_mangle]
    fn summon_specific_friendly(y1: libc::c_int, x1: libc::c_int,
                                lev: libc::c_int, type_0: libc::c_int,
                                Group_ok: bool_) -> bool_;
    #[no_mangle]
    fn fire_ball(typ: libc::c_int, dir: libc::c_int, dam: libc::c_int,
                 rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn get_aim_dir(dp: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn trap_creation() -> bool_;
    #[no_mangle]
    fn unlite_room(y1: libc::c_int, x1: libc::c_int);
    #[no_mangle]
    fn project(who: libc::c_int, rad: libc::c_int, y: libc::c_int,
               x: libc::c_int, dam: libc::c_int, typ: libc::c_int,
               flg: libc::c_int) -> bool_;
    #[no_mangle]
    fn teleport_player_level();
    #[no_mangle]
    fn fire_beam(typ: libc::c_int, dir: libc::c_int, dam: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn teleport_player_to(ny: libc::c_int, nx: libc::c_int);
    #[no_mangle]
    fn teleport_player(dis: libc::c_int);
    #[no_mangle]
    fn tgt_pt(x: *mut libc::c_int, y: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn fire_bolt(typ: libc::c_int, dir: libc::c_int, dam: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn set_fast(v: libc::c_int, p: libc::c_int) -> bool_;
    #[no_mangle]
    fn fear_monster(dir: libc::c_int, plev: libc::c_int) -> bool_;
    #[no_mangle]
    fn do_cmd_wiz_named_friendly(r_idx: libc::c_int, slp: bool_);
    #[no_mangle]
    fn aggravate_monsters(who: libc::c_int);
    #[no_mangle]
    fn get_check(prompt: cptr) -> bool_;
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn repeat_push(what: libc::c_int);
    #[no_mangle]
    fn repeat_pull(what: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn wield_slot(o_ptr: *mut object_type) -> s16b;
    #[no_mangle]
    fn get_string(prompt: cptr, buf: *mut libc::c_char, len: libc::c_int)
     -> bool_;
    #[no_mangle]
    static mut get_item_extra_hook:
           Option<unsafe extern "C" fn(_: *mut libc::c_int) -> bool_>;
    #[no_mangle]
    fn inven_item_describe(item: libc::c_int);
    #[no_mangle]
    fn cmsg_format(color: byte_hack, fmt: cptr, _: ...);
    #[no_mangle]
    fn summon_specific(y1: libc::c_int, x1: libc::c_int, lev: libc::c_int,
                       type_0: libc::c_int) -> bool_;
    #[no_mangle]
    fn apply_disenchant(mode: libc::c_int) -> bool_;
    #[no_mangle]
    fn lose_all_info() -> bool_;
    #[no_mangle]
    fn wall_stone(y: libc::c_int, x: libc::c_int) -> bool_;
    #[no_mangle]
    fn earthquake(cy: libc::c_int, cx: libc::c_int, r: libc::c_int);
    #[no_mangle]
    fn lite_area(dam: libc::c_int, rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn unlite_area(dam: libc::c_int, rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn door_creation() -> bool_;
    #[no_mangle]
    fn destroy_doors_touch() -> bool_;
    #[no_mangle]
    fn sleep_monsters_touch() -> bool_;
    #[no_mangle]
    fn activate_ty_curse();
    #[no_mangle]
    fn activate_hi_summon();
    #[no_mangle]
    fn summon_cyber();
    #[no_mangle]
    fn wall_breaker();
    #[no_mangle]
    fn lua_gettop(L_0: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L_0: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_getglobal(L_0: *mut lua_State, name: *const libc::c_char);
    #[no_mangle]
    fn lua_call(L_0: *mut lua_State, nargs: libc::c_int,
                nresults: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tolua_getnumber(L_0: *mut lua_State, narg: libc::c_int,
                       def: libc::c_long) -> libc::c_long;
    #[no_mangle]
    fn tolua_pushstring(L_0: *mut lua_State, value: *const libc::c_char);
    /* File: cmd5.c */
    /* Purpose: Class commands */
    /*
 * Copyright (c) 1989 James E. Wilson, Robert A. Koeneke
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
    #[no_mangle]
    static mut L: *mut lua_State;
}
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
pub type cptr = *const libc::c_char;
pub type errr = libc::c_int;
pub type byte_hack = libc::c_uchar;
pub type bool_ = libc::c_char;
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
pub struct monster_power {
    pub power: u32b,
    pub name: cptr,
    pub mana: libc::c_int,
    pub great: bool_,
}
/* Maximum number of tries for teleporting */
#[no_mangle]
pub unsafe extern "C" fn is_school_book(mut o_ptr: *mut object_type)
 -> bool_ {
    if (*o_ptr).tval as libc::c_int == 111 as libc::c_int {
        return 1 as libc::c_int as bool_
    } else if (*o_ptr).tval as libc::c_int == 115 as libc::c_int {
        return 1 as libc::c_int as bool_
    } else if (*o_ptr).tval as libc::c_int == 14 as libc::c_int {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
/* Does it contains a schooled spell ? */
unsafe extern "C" fn hook_school_spellable(mut o_ptr: *mut object_type)
 -> bool_ {
    if is_school_book(o_ptr) != 0 {
        return 1 as libc::c_int as bool_
    } else {
        let mut f1: u32b = 0;
        let mut f2: u32b = 0;
        let mut f3: u32b = 0;
        let mut f4: u32b = 0;
        let mut f5: u32b = 0;
        let mut esp: u32b = 0;
        /* Extract object flags */
        object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                     &mut esp);
        if f5 as libc::c_long & 0x800 as libc::c_long != 0 &&
               (*o_ptr).pval2 as libc::c_int != -(1 as libc::c_int) {
            return 1 as libc::c_int as bool_
        }
    }
    return 0 as libc::c_int as bool_;
}
/* Is it a book */
#[no_mangle]
pub unsafe extern "C" fn item_tester_hook_browsable(mut o_ptr:
                                                        *mut object_type)
 -> bool_ {
    if hook_school_spellable(o_ptr) != 0 { return 1 as libc::c_int as bool_ }
    if (*o_ptr).tval as libc::c_int >= 111 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
/*
 * Are we using a mage staff
 */
#[no_mangle]
pub unsafe extern "C" fn is_magestaff() -> bool_ {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (*p_ptr).body_parts[i as usize] as libc::c_int == 24 as libc::c_int
          {
        let mut o_ptr: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset((24 as libc::c_int +
                                                              i) as isize) as
                *mut object_type;
        /* Wielding a mage staff */
        if (*o_ptr).k_idx as libc::c_int != 0 &&
               (*o_ptr).tval as libc::c_int == 6 as libc::c_int {
            return 1 as libc::c_int as bool_
        }
        /* Next slot */
        i += 1;
        /* Paranoia */
        if i >= 52 as libc::c_int - 24 as libc::c_int { break ; }
    }
    /* Not wielding a mage staff */
    return 0 as libc::c_int as bool_;
}
/*
 * Peruse the spells/prayers in a book
 *
 * Note that *all* spells in the book are listed
 *
 * Note that browsing is allowed while confused or blind,
 * and in the dark, primarily to allow browsing in stores.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_browse_aux(mut o_ptr: *mut object_type) {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    if is_school_book(o_ptr) != 0 {
        browse_school_spell((*o_ptr).sval as libc::c_int, (*o_ptr).pval,
                            o_ptr);
    } else if f5 as libc::c_long & 0x800 as libc::c_long != 0 &&
                  (*o_ptr).pval2 as libc::c_int != -(1 as libc::c_int) {
        browse_school_spell(255 as libc::c_int, (*o_ptr).pval2 as libc::c_int,
                            o_ptr);
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_cmd_browse() {
    let mut item: libc::c_int = 0;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    /* Restrict choices to "useful" books */
    item_tester_hook =
        Some(item_tester_hook_browsable as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Get an item */
    q = b"Browse which book? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have no books that you can read.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s,
                0x2 as libc::c_int | 0x1 as libc::c_int | 0x4 as libc::c_int)
           == 0 {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    do_cmd_browse_aux(o_ptr);
}
#[no_mangle]
pub unsafe extern "C" fn do_poly_wounds() {
    /* Changed to always provide at least _some_ healing */
    let mut wounds: s16b = (*p_ptr).cut;
    let mut hit_p: s16b =
        ((*p_ptr).mhp as libc::c_int - (*p_ptr).chp as libc::c_int) as s16b;
    let mut change: s16b =
        damroll((*p_ptr).lev, 5 as libc::c_int as s16b) as s16b;
    let mut Nasty_effect: bool_ =
        (Rand_div(5 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int) as
            libc::c_int as bool_;
    if !(wounds as libc::c_int != 0 || hit_p as libc::c_int != 0 ||
             Nasty_effect as libc::c_int != 0) {
        return
    }
    msg_print(b"Your wounds are polymorphed into less serious ones.\x00" as
                  *const u8 as *const libc::c_char);
    hp_player(change as libc::c_int);
    if Nasty_effect != 0 {
        msg_print(b"A new wound was created!\x00" as *const u8 as
                      *const libc::c_char);
        take_hit(change as libc::c_int / 2 as libc::c_int,
                 b"a polymorphed wound\x00" as *const u8 as
                     *const libc::c_char);
        set_cut(change as libc::c_int);
    } else {
        set_cut((*p_ptr).cut as libc::c_int -
                    change as libc::c_int / 2 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_poly_self() {
    let mut power: libc::c_int = (*p_ptr).lev as libc::c_int;
    let mut poly_power: libc::c_int = 0;
    msg_print(b"You feel a change coming over you...\x00" as *const u8 as
                  *const libc::c_char);
    if power > Rand_div(20 as libc::c_int) &&
           Rand_div(3 as libc::c_int) == 0 as libc::c_int {
        let mut effect_msg: [libc::c_char; 80] =
            *::std::mem::transmute::<&[u8; 80],
                                     &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
        let mut new_race: libc::c_int = 0;
        let mut expfact: libc::c_int = 0;
        let mut goalexpfact: libc::c_int = 0;
        /* Some form of racial polymorph... */
        power -= 10 as libc::c_int;
        if power > Rand_div(5 as libc::c_int) &&
               Rand_div(4 as libc::c_int) == 0 as libc::c_int {
            /* sex change */
            power -= 2 as libc::c_int;
            if (*p_ptr).psex as libc::c_int == 1 as libc::c_int {
                (*p_ptr).psex = 0 as libc::c_int as byte_hack;
                sp_ptr =
                    &mut *sex_info.as_mut_ptr().offset((*p_ptr).psex as isize)
                        as *mut player_sex;
                strcpy(effect_msg.as_mut_ptr(),
                       b"female\x00" as *const u8 as *const libc::c_char);
            } else {
                (*p_ptr).psex = 1 as libc::c_int as byte_hack;
                sp_ptr =
                    &mut *sex_info.as_mut_ptr().offset((*p_ptr).psex as isize)
                        as *mut player_sex;
                strcpy(effect_msg.as_mut_ptr(),
                       b"male\x00" as *const u8 as *const libc::c_char);
            }
        }
        if power > Rand_div(30 as libc::c_int) &&
               Rand_div(5 as libc::c_int) == 0 as libc::c_int {
            let mut tmp: libc::c_int = 0 as libc::c_int;
            /* Harmful deformity */
            power -= 15 as libc::c_int;
            while tmp < 6 as libc::c_int {
                if Rand_div(2 as libc::c_int) == 0 as libc::c_int {
                    dec_stat(tmp,
                             Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                                 6 as libc::c_int,
                             (Rand_div(3 as libc::c_int) == 0 as libc::c_int)
                                 as libc::c_int);
                    power -= 1 as libc::c_int
                }
                tmp += 1
            }
            /* Deformities are discriminated against! */
            dec_stat(5 as libc::c_int,
                     Rand_div(6 as libc::c_int) + 1 as libc::c_int,
                     1 as libc::c_int);
            if effect_msg[0 as libc::c_int as usize] != 0 {
                let mut tmp_msg: [libc::c_char; 10] = [0; 10];
                strnfmt(tmp_msg.as_mut_ptr(), 10 as libc::c_int as uint_hack,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        effect_msg.as_mut_ptr());
                strnfmt(effect_msg.as_mut_ptr(),
                        80 as libc::c_int as uint_hack,
                        b"deformed %s\x00" as *const u8 as
                            *const libc::c_char, tmp_msg.as_mut_ptr());
            } else {
                strcpy(effect_msg.as_mut_ptr(),
                       b"deformed\x00" as *const u8 as *const libc::c_char);
            }
        }
        while power > Rand_div(20 as libc::c_int) &&
                  Rand_div(10 as libc::c_int) == 0 as libc::c_int {
            /* Polymorph into a less corrupted form */
            power -= 10 as libc::c_int;
            lose_corruption(0 as libc::c_int);
        }
        /*
		 * I'm not sure 'power' is always positive, with *so* many minuses.
		 * Also, passing zero / negative numbers to randint/rand_int can
		 * cause a zero divide exception, IIRC, not to speak of its absurdity
		 * -- pelpel
		 */
        poly_power =
            if power > 1 as libc::c_int { power } else { 1 as libc::c_int };
        /*
		 * Restrict the race choices by exp penalty so weak polymorph
		 * always means weak race
		 */
        goalexpfact =
            100 as libc::c_int + 3 as libc::c_int * Rand_div(poly_power);
        loop 
             /* Roll until an appropriate selection is made */
             {
            new_race = Rand_div(max_rp_idx as s32b);
            expfact =
                (*race_info.offset(new_race as isize)).r_exp as libc::c_int;
            if new_race != (*p_ptr).prace as libc::c_int &&
                   expfact <= goalexpfact {
                break ;
            }
        }
        if effect_msg[0 as libc::c_int as usize] != 0 {
            msg_format(b"You turn into a%s %s!\x00" as *const u8 as
                           *const libc::c_char,
                       if is_a_vowel(*rp_name.offset((*race_info.offset(new_race
                                                                            as
                                                                            isize)).title
                                                         as isize) as
                                         libc::c_int) as libc::c_int != 0 {
                           b"n\x00" as *const u8 as *const libc::c_char
                       } else { b"\x00" as *const u8 as *const libc::c_char },
                       rp_name.offset((*race_info.offset(new_race as
                                                             isize)).title as
                                          isize));
        } else {
            msg_format(b"You turn into a %s %s!\x00" as *const u8 as
                           *const libc::c_char, effect_msg.as_mut_ptr(),
                       (*race_info.offset(new_race as isize)).title);
        }
        (*p_ptr).prace = new_race as byte_hack;
        rp_ptr =
            &mut *race_info.offset((*p_ptr).prace as isize) as
                *mut player_race;
        /* Experience factor */
        (*p_ptr).expfact =
            ((*rp_ptr).r_exp as libc::c_int + (*rmp_ptr).r_exp as libc::c_int
                 + (*cp_ptr).c_exp as libc::c_int) as u16b;
        /* Calculate the height/weight */
        get_height_weight();
        check_experience();
        (*p_ptr).max_plv = (*p_ptr).lev;
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x2000000 as libc::c_long) as
                u32b;
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
        handle_stuff();
        lite_spot((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int);
    }
    if power > Rand_div(30 as libc::c_int) &&
           Rand_div(6 as libc::c_int) == 0 as libc::c_int {
        let mut tmp_0: libc::c_int = 0 as libc::c_int;
        /* Abomination! */
        power -= 20 as libc::c_int;
        msg_print(b"Your internal organs are rearranged!\x00" as *const u8 as
                      *const libc::c_char);
        while tmp_0 < 6 as libc::c_int {
            dec_stat(tmp_0,
                     Rand_div(6 as libc::c_int) + 1 as libc::c_int +
                         6 as libc::c_int,
                     (Rand_div(3 as libc::c_int) == 0 as libc::c_int) as
                         libc::c_int);
            tmp_0 += 1
        }
        if Rand_div(6 as libc::c_int) == 0 as libc::c_int {
            msg_print(b"You find living difficult in your present form!\x00"
                          as *const u8 as *const libc::c_char);
            take_hit(damroll((Rand_div(10 as libc::c_int) + 1 as libc::c_int)
                                 as s16b, (*p_ptr).lev),
                     b"a lethal corruption\x00" as *const u8 as
                         *const libc::c_char);
            power -= 10 as libc::c_int
        }
    }
    if power > Rand_div(20 as libc::c_int) &&
           Rand_div(4 as libc::c_int) == 0 as libc::c_int {
        power -= 10 as libc::c_int;
        do_cmd_rerate();
    }
    while power > Rand_div(15 as libc::c_int) &&
              Rand_div(3 as libc::c_int) == 0 as libc::c_int {
        power -= 7 as libc::c_int;
        gain_random_corruption(0 as libc::c_int);
    }
    if power > Rand_div(5 as libc::c_int) {
        power -= 5 as libc::c_int;
        do_poly_wounds();
    }
    /* Note: earlier deductions may have left power < 0 already. */
    while power > 0 as libc::c_int { corrupt_player(); power -= 1 };
}
/*
 * Brand the current weapon
 */
#[no_mangle]
pub unsafe extern "C" fn brand_weapon(mut brand_type: libc::c_int) {
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut act: cptr = 0 as cptr;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(24 as libc::c_int as
                                                         isize) as
            *mut object_type;
    /*
	 * You can never modify artifacts / ego-items
	 * You can never modify cursed items
	 *
	 * TY: You _can_ modify broken items (if you're silly enough)
	 */
    if (*o_ptr).k_idx == 0 ||
           ((*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
                (if (*o_ptr).name1 as libc::c_int != 0 {
                     1 as libc::c_int
                 } else { 0 as libc::c_int }) != 0 ||
                (if (*o_ptr).art_name as libc::c_int != 0 {
                     1 as libc::c_int
                 } else { 0 as libc::c_int }) != 0 ||
                (if (*k_info.offset((*o_ptr).k_idx as isize)).flags3 as
                        libc::c_long & 0x8000 as libc::c_long != 0 {
                     1 as libc::c_int
                 } else { 0 as libc::c_int }) != 0) ||
           (if (*o_ptr).name2 as libc::c_int != 0 ||
                   (*o_ptr).name2b as libc::c_int != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int }) != 0 ||
           (*o_ptr).art_name as libc::c_int != 0 ||
           (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
        if flush_failure != 0 { flush(); }
        msg_print(b"The Branding failed.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Save the old name */
    object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                0 as libc::c_int);
    match brand_type {
        6 => {
            act =
                b"glows with godly power.\x00" as *const u8 as
                    *const libc::c_char;
            (*o_ptr).name2 = 67 as libc::c_int as s16b;
            (*o_ptr).pval = Rand_div(4 as libc::c_int) + 1 as libc::c_int
        }
        5 => {
            act =
                b"seems very powerful.\x00" as *const u8 as
                    *const libc::c_char;
            (*o_ptr).name2 = 80 as libc::c_int as s16b;
            (*o_ptr).pval = Rand_div(3 as libc::c_int) + 1 as libc::c_int
        }
        4 => {
            act =
                b"seems very unstable now.\x00" as *const u8 as
                    *const libc::c_char;
            (*o_ptr).name2 = 99 as libc::c_int as s16b;
            (*o_ptr).pval = Rand_div(2 as libc::c_int) + 1 as libc::c_int
        }
        3 => {
            act =
                b"thirsts for blood!\x00" as *const u8 as *const libc::c_char;
            (*o_ptr).name2 = 97 as libc::c_int as s16b
        }
        2 => {
            act =
                b"is coated with poison.\x00" as *const u8 as
                    *const libc::c_char;
            (*o_ptr).name2 = 77 as libc::c_int as s16b
        }
        1 => {
            act =
                b"is engulfed in raw chaos!\x00" as *const u8 as
                    *const libc::c_char;
            (*o_ptr).name2 = 78 as libc::c_int as s16b
        }
        _ => {
            if Rand_div(100 as libc::c_int) < 25 as libc::c_int {
                act =
                    b"is covered in a fiery shield!\x00" as *const u8 as
                        *const libc::c_char;
                (*o_ptr).name2 = 75 as libc::c_int as s16b
            } else {
                act =
                    b"glows deep, icy blue!\x00" as *const u8 as
                        *const libc::c_char;
                (*o_ptr).name2 = 76 as libc::c_int as s16b
            }
        }
    }
    /* Apply the ego */
    apply_magic(o_ptr, dun_level as libc::c_int, 0 as libc::c_int as bool_,
                0 as libc::c_int as bool_, 0 as libc::c_int as bool_);
    (*o_ptr).discount = 100 as libc::c_int as byte_hack;
    msg_format(b"Your %s %s\x00" as *const u8 as *const libc::c_char,
               o_name.as_mut_ptr(), act);
    enchant(o_ptr, Rand_div(3 as libc::c_int) + 4 as libc::c_int,
            0x1 as libc::c_int | 0x2 as libc::c_int);
}
/*
 * Fetch an item (teleport it right underneath the caster)
 */
#[no_mangle]
pub unsafe extern "C" fn fetch(mut dir: libc::c_int, mut wgt: libc::c_int,
                               mut require_los: bool_) {
    let mut ty: libc::c_int = 0;
    let mut tx: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    /* Check to see if an object is already there */
    if (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).o_idx != 0 {
        msg_print(b"You can\'t fetch when you\'re already standing on something.\x00"
                      as *const u8 as *const libc::c_char);
        return
    }
    /* Use a target */
    if dir == 5 as libc::c_int && target_okay() as libc::c_int != 0 {
        tx = target_col as libc::c_int;
        ty = target_row as libc::c_int;
        if distance((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                    ty, tx) > 18 as libc::c_int {
            msg_print(b"You can\'t fetch something that far away!\x00" as
                          *const u8 as *const libc::c_char);
            return
        }
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(ty as isize)).offset(tx as isize)
                as *mut cave_type;
        if (*c_ptr).o_idx == 0 {
            msg_print(b"There is no object at this place.\x00" as *const u8 as
                          *const libc::c_char);
            return
        }
        if require_los as libc::c_int != 0 &&
               !((*cave[ty as usize].offset(tx as isize)).info as libc::c_int
                     & 0x20 as libc::c_int != 0 as libc::c_int) {
            msg_print(b"You have no direct line of sight to that location.\x00"
                          as *const u8 as *const libc::c_char);
            return
        }
    } else {
        /* Use a direction */
        ty = (*p_ptr).py as libc::c_int; /* Where to drop the item */
        tx = (*p_ptr).px as libc::c_int;
        loop  {
            ty += ddy[dir as usize] as libc::c_int;
            tx += ddx[dir as usize] as libc::c_int;
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset(ty as
                                                     isize)).offset(tx as
                                                                        isize)
                    as *mut cave_type;
            if distance((*p_ptr).py as libc::c_int,
                        (*p_ptr).px as libc::c_int, ty, tx) >
                   18 as libc::c_int ||
                   !((*f_info.offset((*cave[ty as
                                                usize].offset(tx as
                                                                  isize)).feat
                                         as isize)).flags1 as libc::c_long &
                         0x10 as libc::c_long != 0 &&
                         (*cave[ty as usize].offset(tx as isize)).feat as
                             libc::c_int != 0xaf as libc::c_int) {
                return
            }
            if (*c_ptr).o_idx != 0 { break ; }
        }
    }
    o_ptr = &mut *o_list.offset((*c_ptr).o_idx as isize) as *mut object_type;
    if (*o_ptr).weight > wgt {
        /* Too heavy to 'fetch' */
        msg_print(b"The object is too heavy.\x00" as *const u8 as
                      *const libc::c_char); /* 'move' it */
        return
    }
    i = (*c_ptr).o_idx as libc::c_int;
    (*c_ptr).o_idx = (*o_ptr).next_o_idx;
    (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).o_idx =
        i as s16b;
    (*o_ptr).next_o_idx = 0 as libc::c_int as s16b;
    (*o_ptr).iy = (*p_ptr).py as byte_hack;
    (*o_ptr).ix = (*p_ptr).px as byte_hack;
    object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                0 as libc::c_int);
    msg_format(b"%^s flies through the air to your feet.\x00" as *const u8 as
                   *const libc::c_char, o_name.as_mut_ptr());
    note_spot((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int);
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as u32b;
}
/*
 * Handle random effects of player shrieking
 */
#[no_mangle]
pub unsafe extern "C" fn shriek_effect() {
    match Rand_div(9 as libc::c_int) + 1 as libc::c_int {
        1 | 5 | 8 | 9 => {
            msg_print(b"You make a high-pitched shriek!\x00" as *const u8 as
                          *const libc::c_char);
            aggravate_monsters(1 as libc::c_int);
        }
        2 | 6 => {
            msg_print(b"Oops! You call a monster.\x00" as *const u8 as
                          *const libc::c_char);
            summon_specific((*p_ptr).py as libc::c_int,
                            (*p_ptr).px as libc::c_int,
                            *max_dlv.offset(dungeon_type as isize) as
                                libc::c_int, 0 as libc::c_int);
        }
        3 | 7 => {
            msg_print(b"The dungeon collapses!\x00" as *const u8 as
                          *const libc::c_char);
            earthquake((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                       5 as libc::c_int);
        }
        4 => {
            msg_print(b"Your shriek is so horrible that you damage your health!\x00"
                          as *const u8 as *const libc::c_char);
            take_hit(damroll(((*p_ptr).lev as libc::c_int / 5 as libc::c_int)
                                 as s16b, 8 as libc::c_int as s16b),
                     b"inner hemorrhaging\x00" as *const u8 as
                         *const libc::c_char);
        }
        _ => { }
    };
}
/*
 * Like all the random effect codes, this is *ugly*,
 * and there is not a single line of comment, so I can't tell
 * some fall throughs are really intended. Well, I know it's
 * intended to be bizarre :) -- pelpel
 */
#[no_mangle]
pub unsafe extern "C" fn wild_magic(mut spell: libc::c_int) {
    let mut counter: libc::c_int = 0 as libc::c_int;
    let mut type_0: libc::c_int =
        33 as libc::c_int - 1 as libc::c_int +
            (Rand_div(6 as libc::c_int) + 1 as libc::c_int);
    if type_0 < 33 as libc::c_int {
        type_0 = 33 as libc::c_int
    } else if type_0 > 38 as libc::c_int { type_0 = 38 as libc::c_int }
    match Rand_div(spell) + 1 as libc::c_int +
              (Rand_div(8 as libc::c_int) + 1 as libc::c_int) +
              1 as libc::c_int {
        1 | 2 | 3 => { teleport_player(10 as libc::c_int); }
        4 | 5 | 6 => { teleport_player(100 as libc::c_int); }
        7 | 8 => { teleport_player(200 as libc::c_int); }
        9 | 10 | 11 => { unlite_area(10 as libc::c_int, 3 as libc::c_int); }
        12 | 13 | 14 => {
            lite_area(damroll(2 as libc::c_int as s16b,
                              3 as libc::c_int as s16b), 2 as libc::c_int);
        }
        15 => { destroy_doors_touch(); }
        16 | 17 => { wall_breaker(); }
        18 => { sleep_monsters_touch(); }
        19 | 20 => { trap_creation(); }
        21 | 22 => { door_creation(); }
        23 | 24 | 25 => { aggravate_monsters(1 as libc::c_int); }
        26 => {
            /* Prevent destruction of quest levels and town */
            if is_quest(dun_level as libc::c_int) == 0 &&
                   dun_level as libc::c_int != 0 {
                earthquake((*p_ptr).py as libc::c_int,
                           (*p_ptr).px as libc::c_int, 5 as libc::c_int);
            }
        }
        27 | 28 => { }
        29 | 30 => { apply_disenchant(0 as libc::c_int); }
        31 => { lose_all_info(); }
        32 => {
            fire_ball(30 as libc::c_int, 0 as libc::c_int,
                      spell + 5 as libc::c_int,
                      1 as libc::c_int + spell / 10 as libc::c_int);
        }
        33 => {
            wall_stone((*p_ptr).py as libc::c_int,
                       (*p_ptr).px as libc::c_int);
        }
        34 | 35 => {
            loop  {
                let fresh0 = counter;
                counter = counter + 1;
                if !(fresh0 < 8 as libc::c_int) { break ; }
                summon_specific((*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                dun_level as libc::c_int * 3 as libc::c_int /
                                    2 as libc::c_int, type_0);
            }
        }
        36 | 37 => { activate_hi_summon(); }
        38 => { summon_cyber(); }
        _ => { activate_ty_curse(); }
    };
}
/*
 * Hack -- Determine if the player is wearing an artefact ring
 * specified by art_type, that should be an index into a_info
 */
#[no_mangle]
pub unsafe extern "C" fn check_ring(mut art_type: libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0;
    /* We are only interested in ring slots */
    i = 28 as libc::c_int;
    /* Scan the list of rings until we reach the end */
    while (*p_ptr).body_parts[(i - 24 as libc::c_int) as usize] as libc::c_int
              == 28 as libc::c_int {
        /* Found the ring we were looking for */
        if (*p_ptr).inventory[i as usize].k_idx as libc::c_int != 0 &&
               (*p_ptr).inventory[i as usize].name1 as libc::c_int == art_type
           {
            return 1 as libc::c_int as bool_
        }
        /* Next item */
        i += 1
    }
    /* Found nothing */
    return 0 as libc::c_int as bool_;
}
/*
 * Return the symbiote's name or description.
 */
#[no_mangle]
pub unsafe extern "C" fn symbiote_name(mut capitalize: bool_) -> cptr {
    let mut o_ptr: *mut object_type =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(49 as libc::c_int as
                                                         isize) as
            *mut object_type;
    static mut buf: [libc::c_char; 80] = [0; 80];
    /* Make sure there actually is a symbiote there... */
    if (*o_ptr).k_idx == 0 {
        strcpy(buf.as_mut_ptr(),
               b"A non-existent symbiote\x00" as *const u8 as
                   *const libc::c_char);
    } else {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset((*o_ptr).pval as isize) as *mut monster_race;
        let mut s: cptr = 0 as cptr;
        if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
            /* Unique monster; no preceding "your", and ignore our name. */
            strncpy(buf.as_mut_ptr(), r_name.offset((*r_ptr).name as isize),
                    ::std::mem::size_of::<[libc::c_char; 80]>() as
                        libc::c_ulong);
        } else if (*o_ptr).note as libc::c_int != 0 &&
                      {
                          s =
                              strstr(quark_str((*o_ptr).note as s16b),
                                     b"#named \x00" as *const u8 as
                                         *const libc::c_char) as cptr;
                          !s.is_null()
                      } {
            /* We've named it. */
            strncpy(buf.as_mut_ptr(), s.offset(7 as libc::c_int as isize),
                    ::std::mem::size_of::<[libc::c_char; 80]>() as
                        libc::c_ulong);
        } else {
            /* No special cases, just return "Your <monster type>". */
            strcpy(buf.as_mut_ptr(),
                   b"your \x00" as *const u8 as *const libc::c_char);
            strncpy(buf.as_mut_ptr().offset(5 as libc::c_int as isize),
                    r_name.offset((*r_ptr).name as isize),
                    (::std::mem::size_of::<[libc::c_char; 80]>() as
                         libc::c_ulong).wrapping_sub(5 as libc::c_int as
                                                         libc::c_ulong));
        }
    }
    /* Just in case... */
    buf[(::std::mem::size_of::<[libc::c_char; 80]>() as
             libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong) as
            usize] = '\u{0}' as i32 as libc::c_char;
    if capitalize != 0 {
        buf[0 as libc::c_int as usize] =
            toupper(buf[0 as libc::c_int as usize] as libc::c_int) as
                libc::c_char
    }
    return buf.as_mut_ptr() as cptr;
}
/*
 * Use a power of the monster in symbiosis
 */
#[no_mangle]
pub unsafe extern "C" fn use_symbiotic_power(mut r_idx: libc::c_int,
                                             mut great: bool_,
                                             mut only_number: bool_,
                                             mut no_cost: bool_)
 -> libc::c_int {
    let mut power: libc::c_int = -(1 as libc::c_int);
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut dir: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut powers: [libc::c_int; 96] = [0; 96];
    let mut flag: bool_ = 0;
    let mut redraw: bool_ = 0;
    let mut ask: libc::c_int = 0;
    let mut plev: libc::c_int = (*p_ptr).lev as libc::c_int;
    let mut choice: libc::c_char = 0;
    let mut out_val: [libc::c_char; 160] = [0; 160];
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    let mut rlev: libc::c_int =
        if (*r_ptr).level as libc::c_int >= 1 as libc::c_int {
            (*r_ptr).level as libc::c_int
        } else { 1 as libc::c_int };
    let mut x: libc::c_int = (*p_ptr).px as libc::c_int;
    let mut y: libc::c_int = (*p_ptr).py as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut rad: libc::c_int = 0;
    let mut label: libc::c_int = 0;
    /* List the monster powers -- RF4_* */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if (*r_ptr).flags4 as libc::c_long & (1 as libc::c_long) << i != 0 {
            if !(monster_powers[i as usize].great as libc::c_int != 0 &&
                     great == 0) {
                if !(monster_powers[i as usize].power == 0) {
                    let fresh1 = num;
                    num = num + 1;
                    powers[fresh1 as usize] = i
                }
            }
        }
        i += 1
    }
    /* List the monster powers -- RF5_* */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if (*r_ptr).flags5 as libc::c_long & (1 as libc::c_long) << i != 0 {
            if !(monster_powers[(i + 32 as libc::c_int) as usize].great as
                     libc::c_int != 0 && great == 0) {
                if !(monster_powers[(i + 32 as libc::c_int) as usize].power ==
                         0) {
                    let fresh2 = num;
                    num = num + 1;
                    powers[fresh2 as usize] = i + 32 as libc::c_int
                }
            }
        }
        i += 1
    }
    /* List the monster powers -- RF6_* */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if (*r_ptr).flags6 as libc::c_long & (1 as libc::c_long) << i != 0 {
            if !(monster_powers[(i + 64 as libc::c_int) as usize].great as
                     libc::c_int != 0 && great == 0) {
                if !(monster_powers[(i + 64 as libc::c_int) as usize].power ==
                         0) {
                    let fresh3 = num;
                    num = num + 1;
                    powers[fresh3 as usize] = i + 64 as libc::c_int
                }
            }
        }
        i += 1
    }
    if num == 0 {
        msg_print(b"You have no powers you can use.\x00" as *const u8 as
                      *const libc::c_char);
        return 0 as libc::c_int
    }
    if only_number != 0 { return num }
    /* Nothing chosen yet */
    flag = 0 as libc::c_int as bool_;
    /* No redraw yet */
    redraw = 0 as libc::c_int as bool_;
    /* Get the last label */
    label =
        if num <= 26 as libc::c_int {
            (num - 1 as libc::c_int) + 'a' as i32
        } else { (num - 1 as libc::c_int - 26 as libc::c_int) + '0' as i32 };
    /* Build a prompt (accept all spells) */
	/* Mega Hack -- if no_cost is false, we're actually a Possessor -dsb */
    strnfmt(out_val.as_mut_ptr(), 78 as libc::c_int as uint_hack,
            b"(Powers a-%c, *=List, ESC=exit) Use which power of your %s? \x00"
                as *const u8 as *const libc::c_char, label,
            if no_cost as libc::c_int != 0 {
                b"symbiote\x00" as *const u8 as *const libc::c_char
            } else { b"body\x00" as *const u8 as *const libc::c_char });
    /* Get a spell from the user */
    while flag == 0 &&
              get_com(out_val.as_mut_ptr() as cptr, &mut choice) as
                  libc::c_int != 0 {
        /* Request redraw */
        if choice as libc::c_int == ' ' as i32 ||
               choice as libc::c_int == '*' as i32 ||
               choice as libc::c_int == '?' as i32 {
            /* Show the list */
            if redraw == 0 {
                let mut y_0: byte_hack = 1 as libc::c_int as byte_hack;
                let mut x_0: byte_hack = 0 as libc::c_int as byte_hack;
                let mut ctr: libc::c_int = 0 as libc::c_int;
                let mut dummy: [libc::c_char; 80] = [0; 80];
                strcpy(dummy.as_mut_ptr(),
                       b"\x00" as *const u8 as *const libc::c_char);
                /* Show list */
                redraw = 1 as libc::c_int as bool_;
                /* Save the screen */
                character_icky = 1 as libc::c_int as bool_;
                Term_save();
                let fresh4 = y_0;
                y_0 = y_0.wrapping_add(1);
                prt(b"\x00" as *const u8 as *const libc::c_char,
                    fresh4 as libc::c_int, x_0 as libc::c_int);
                while ctr < num {
                    let mut mp_ptr: *mut monster_power =
                        &mut *monster_powers.as_mut_ptr().offset(*powers.as_mut_ptr().offset(ctr
                                                                                                 as
                                                                                                 isize)
                                                                     as isize)
                            as *mut monster_power;
                    let mut mana: libc::c_int =
                        (*mp_ptr).mana / 10 as libc::c_int;
                    if mana > (*p_ptr).msp as libc::c_int {
                        mana = (*p_ptr).msp as libc::c_int
                    }
                    if mana == 0 { mana = 1 as libc::c_int }
                    label =
                        if ctr < 26 as libc::c_int {
                            (ctr) + 'a' as i32
                        } else { (ctr - 26 as libc::c_int) + '0' as i32 };
                    if no_cost == 0 {
                        strnfmt(dummy.as_mut_ptr(),
                                80 as libc::c_int as uint_hack,
                                b" %c) %2d %s\x00" as *const u8 as
                                    *const libc::c_char, label, mana,
                                (*mp_ptr).name);
                    } else {
                        strnfmt(dummy.as_mut_ptr(),
                                80 as libc::c_int as uint_hack,
                                b" %c) %s\x00" as *const u8 as
                                    *const libc::c_char, label,
                                (*mp_ptr).name);
                    }
                    if ctr < 17 as libc::c_int {
                        prt(dummy.as_mut_ptr() as cptr,
                            y_0 as libc::c_int + ctr, x_0 as libc::c_int);
                    } else {
                        prt(dummy.as_mut_ptr() as cptr,
                            y_0 as libc::c_int + ctr - 17 as libc::c_int,
                            x_0 as libc::c_int + 40 as libc::c_int);
                    }
                    ctr += 1
                }
                if ctr < 17 as libc::c_int {
                    prt(b"\x00" as *const u8 as *const libc::c_char,
                        y_0 as libc::c_int + ctr, x_0 as libc::c_int);
                } else {
                    prt(b"\x00" as *const u8 as *const libc::c_char,
                        y_0 as libc::c_int + 17 as libc::c_int,
                        x_0 as libc::c_int);
                }
            } else {
                /* Hide the list */
                /* Hide list */
                redraw = 0 as libc::c_int as bool_;
                Term_load();
                character_icky = 0 as libc::c_int as bool_
            }
        } else {
            if choice as libc::c_int == '\r' as i32 && num == 1 as libc::c_int
               {
                choice = 'a' as i32 as libc::c_char
            }
            if *(*__ctype_b_loc()).offset(choice as libc::c_int as isize) as
                   libc::c_int &
                   _ISalpha as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 {
                /* Restore the screen */
                /* Note verify */
                ask =
                    *(*__ctype_b_loc()).offset(choice as libc::c_int as isize)
                        as libc::c_int &
                        _ISupper as libc::c_int as libc::c_ushort as
                            libc::c_int;
                /* Lowercase */
                if ask != 0 {
                    choice = tolower(choice as libc::c_int) as libc::c_char
                }
                /* Extract request */
                i =
                    if *(*__ctype_b_loc()).offset(choice as libc::c_int as
                                                      isize) as libc::c_int &
                           _ISlower as libc::c_int as libc::c_ushort as
                               libc::c_int != 0 {
                        (choice as libc::c_int) - 'a' as i32
                    } else { -(1 as libc::c_int) }
            } else {
                /* Can't uppercase digits XXX XXX XXX */
                ask = 0 as libc::c_int;
                i = choice as libc::c_int - '0' as i32 + 26 as libc::c_int
            }
            /* Totally Illegal */
            if i < 0 as libc::c_int || i >= num {
                bell();
            } else {
                /* Save the spell index */
                power = powers[i as usize];
                /* Verify it */
                if ask != 0 {
                    let mut tmp_val: [libc::c_char; 160] = [0; 160];
                    /* Prompt */
                    strnfmt(tmp_val.as_mut_ptr(),
                            78 as libc::c_int as uint_hack,
                            b"Use %s? \x00" as *const u8 as
                                *const libc::c_char,
                            monster_powers[power as usize].name);
                    /* Belay that order */
                    if get_check(tmp_val.as_mut_ptr() as cptr) == 0 {
                        continue ;
                    }
                }
                /* Stop the loop */
                flag = 1 as libc::c_int as bool_
            }
        }
    }
    /* Restore the screen */
    if redraw != 0 { Term_load(); character_icky = 0 as libc::c_int as bool_ }
    /* Abort if needed */
    if flag == 0 { energy_use = 0 as libc::c_int; return -(1 as libc::c_int) }
    /* 'Powerful' monsters have wider radii */
    if (*r_ptr).flags2 & 0x1000 as libc::c_int as libc::c_uint != 0 {
        rad =
            1 as libc::c_int + (*p_ptr).lev as libc::c_int / 15 as libc::c_int
    } else {
        rad =
            1 as libc::c_int + (*p_ptr).lev as libc::c_int / 20 as libc::c_int
    }
    /* Analyse power */
    match power {
        0 => {
            aggravate_monsters(-(1 as libc::c_int));
            /* 95 S_UNIQUE -- Not available */
        }
        1 => {
            /* MULTIPLY */
            do_cmd_wiz_named_friendly((*p_ptr).body_monster as libc::c_int,
                                      0 as libc::c_int as bool_);
        }
        2 => {
            /* S_ANIMAL */
            summon_specific_friendly(y, x, rlev, 42 as libc::c_int,
                                     1 as libc::c_int as bool_);
        }
        3 => {
            /* ROCKET */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(72 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 12 as libc::c_int,
                          1 as libc::c_int +
                              (*p_ptr).lev as libc::c_int /
                                  20 as libc::c_int);
            }
        }
        4 => {
            /* ARROW_1 */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(11 as libc::c_int, dir,
                          damroll(1 as libc::c_int as s16b,
                                  6 as libc::c_int as s16b));
            }
        }
        5 => {
            /* ARROW_2 */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(11 as libc::c_int, dir,
                          damroll(3 as libc::c_int as s16b,
                                  6 as libc::c_int as s16b));
            }
        }
        6 => {
            /* ARROW_3 */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(11 as libc::c_int, dir,
                          damroll(5 as libc::c_int as s16b,
                                  6 as libc::c_int as s16b));
            }
        }
        7 => {
            /* ARROW_4 */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(11 as libc::c_int, dir,
                          damroll(7 as libc::c_int as s16b,
                                  6 as libc::c_int as s16b));
            }
        }
        8 => {
            /* BR_ACID */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(3 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 5 as libc::c_int,
                          rad);
            }
        }
        9 => {
            /* BR_ELEC */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(1 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 5 as libc::c_int,
                          rad);
            }
        }
        10 => {
            /* BR_FIRE */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(5 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 5 as libc::c_int,
                          rad);
            }
        }
        11 => {
            /* BR_COLD */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(4 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 5 as libc::c_int,
                          rad);
            }
        }
        12 => {
            /* BR_POIS */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(2 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 5 as libc::c_int,
                          rad);
            }
        }
        13 => {
            /* BR_NETH */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(31 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 5 as libc::c_int,
                          rad);
            }
        }
        14 => {
            /* BR_LITE */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(15 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 8 as libc::c_int,
                          rad);
            }
        }
        15 => {
            /* BR_DARK */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(16 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 8 as libc::c_int,
                          rad);
            }
        }
        16 => {
            /* BR_CONF */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(22 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 8 as libc::c_int,
                          rad);
            }
        }
        17 => {
            /* BR_SOUN */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(21 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 8 as libc::c_int,
                          rad);
            }
        }
        18 => {
            /* BR_CHAO */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(30 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 7 as libc::c_int,
                          rad);
            }
        }
        19 => {
            /* BR_DISE */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(32 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 7 as libc::c_int,
                          rad);
            }
        }
        20 => {
            /* BR_NEXU */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(33 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 5 as libc::c_int,
                          rad);
            }
        }
        21 => {
            /* BR_TIME */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(34 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 3 as libc::c_int,
                          rad);
            }
        }
        22 => {
            /* BR_INER */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(24 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 4 as libc::c_int,
                          rad);
            }
        }
        23 => {
            /* BR_GRAV */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(35 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 4 as libc::c_int,
                          rad);
            }
        }
        24 => {
            /* BR_SHAR */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(20 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 8 as libc::c_int,
                          rad);
            }
        }
        25 => {
            /* BR_PLAS */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(12 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 3 as libc::c_int,
                          rad);
            }
        }
        26 => {
            /* BR_WALL */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(23 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 4 as libc::c_int,
                          rad);
            }
        }
        27 => {
            /* BR_MANA */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(26 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 5 as libc::c_int,
                          rad);
            }
        }
        28 => {
            /* BA_NUKE */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(73 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 8 as libc::c_int,
                          1 as libc::c_int +
                              (*p_ptr).lev as libc::c_int /
                                  20 as libc::c_int);
            }
        }
        29 => {
            /* BR_NUKE */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(73 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 8 as libc::c_int,
                          1 as libc::c_int +
                              (*p_ptr).lev as libc::c_int /
                                  20 as libc::c_int);
            }
        }
        30 => {
            /* BA_CHAO */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(30 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 4 as libc::c_int,
                          2 as libc::c_int);
            }
        }
        31 => {
            /* BR_DISI */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(81 as libc::c_int, dir,
                          (*p_ptr).lev as libc::c_int * 5 as libc::c_int,
                          1 as libc::c_int +
                              (*p_ptr).lev as libc::c_int /
                                  20 as libc::c_int);
            }
        }
        32 => {
            /* *** RF5 (bit position + 32) ****/
            /* BA_ACID */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(3 as libc::c_int, dir,
                          Rand_div((*p_ptr).lev as libc::c_int *
                                       6 as libc::c_int) + 1 as libc::c_int +
                              20 as libc::c_int, 2 as libc::c_int);
            }
        }
        33 => {
            /* BA_ELEC */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(1 as libc::c_int, dir,
                          Rand_div((*p_ptr).lev as libc::c_int *
                                       3 as libc::c_int) + 1 as libc::c_int +
                              20 as libc::c_int, 2 as libc::c_int);
            }
        }
        34 => {
            /* BA_FIRE */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(5 as libc::c_int, dir,
                          Rand_div((*p_ptr).lev as libc::c_int *
                                       7 as libc::c_int) + 1 as libc::c_int +
                              20 as libc::c_int, 2 as libc::c_int);
            }
        }
        35 => {
            /* BA_COLD */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(4 as libc::c_int, dir,
                          Rand_div((*p_ptr).lev as libc::c_int *
                                       3 as libc::c_int) + 1 as libc::c_int +
                              20 as libc::c_int, 2 as libc::c_int);
            }
        }
        36 => {
            /* BA_POIS */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(2 as libc::c_int, dir,
                          damroll(12 as libc::c_int as s16b,
                                  2 as libc::c_int as s16b),
                          2 as libc::c_int);
            }
        }
        37 => {
            /* BA_NETH */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(31 as libc::c_int, dir,
                          Rand_div((*p_ptr).lev as libc::c_int *
                                       4 as libc::c_int) + 1 as libc::c_int +
                              20 as libc::c_int, 2 as libc::c_int);
            }
        }
        38 => {
            /* BA_WATE */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(14 as libc::c_int, dir,
                          Rand_div((*p_ptr).lev as libc::c_int *
                                       4 as libc::c_int) + 1 as libc::c_int +
                              20 as libc::c_int, 2 as libc::c_int);
            }
        }
        39 => {
            /* BA_MANA */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(26 as libc::c_int, dir,
                          Rand_div((*p_ptr).lev as libc::c_int *
                                       3 as libc::c_int) + 1 as libc::c_int +
                              20 as libc::c_int, 2 as libc::c_int);
            }
        }
        40 => {
            /* BA_DARK */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(16 as libc::c_int, dir,
                          Rand_div((*p_ptr).lev as libc::c_int *
                                       3 as libc::c_int) + 1 as libc::c_int +
                              20 as libc::c_int, 2 as libc::c_int);
            }
        }
        44 => {
            /* 41 DRAIN_MANA -- Not available */
            /* 42 MIND_BLAST -- Not available */
            /* 43 BRAIN_SMASH -- Not available */
            /* CAUSE_1 */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(26 as libc::c_int, dir,
                          damroll(3 as libc::c_int as s16b,
                                  8 as libc::c_int as s16b));
            }
        }
        45 => {
            /* CAUSE_2 */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(26 as libc::c_int, dir,
                          damroll(8 as libc::c_int as s16b,
                                  8 as libc::c_int as s16b));
            }
        }
        46 => {
            /* CAUSE_3 */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(26 as libc::c_int, dir,
                          damroll(10 as libc::c_int as s16b,
                                  15 as libc::c_int as s16b));
            }
        }
        47 => {
            /* CAUSE_4 */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(26 as libc::c_int, dir,
                          damroll(15 as libc::c_int as s16b,
                                  15 as libc::c_int as s16b));
            }
        }
        48 => {
            /* BO_ACID */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(3 as libc::c_int, dir,
                          damroll(7 as libc::c_int as s16b,
                                  8 as libc::c_int as s16b) +
                              (*p_ptr).lev as libc::c_int / 3 as libc::c_int);
            }
        }
        49 => {
            /* BO_ELEC */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(1 as libc::c_int, dir,
                          damroll(4 as libc::c_int as s16b,
                                  8 as libc::c_int as s16b) +
                              (*p_ptr).lev as libc::c_int / 3 as libc::c_int);
            }
        }
        50 => {
            /* BO_FIRE */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(5 as libc::c_int, dir,
                          damroll(9 as libc::c_int as s16b,
                                  8 as libc::c_int as s16b) +
                              (*p_ptr).lev as libc::c_int / 3 as libc::c_int);
            }
        }
        51 => {
            /* BO_COLD */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(4 as libc::c_int, dir,
                          damroll(6 as libc::c_int as s16b,
                                  8 as libc::c_int as s16b) +
                              (*p_ptr).lev as libc::c_int / 3 as libc::c_int);
            }
        }
        52 => {
            /* BO_POIS */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(2 as libc::c_int, dir,
                          damroll(7 as libc::c_int as s16b,
                                  8 as libc::c_int as s16b) +
                              (*p_ptr).lev as libc::c_int / 3 as libc::c_int);
            }
        }
        53 => {
            /* BO_NETH */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(31 as libc::c_int, dir,
                          damroll(5 as libc::c_int as s16b,
                                  5 as libc::c_int as s16b) +
                              (*p_ptr).lev as libc::c_int / 3 as libc::c_int);
            }
        }
        54 => {
            /* BO_WATE */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(14 as libc::c_int, dir,
                          damroll(10 as libc::c_int as s16b,
                                  10 as libc::c_int as s16b) +
                              (*p_ptr).lev as libc::c_int / 3 as libc::c_int);
            }
        }
        55 => {
            /* BO_MANA */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(26 as libc::c_int, dir,
                          damroll(3 as libc::c_int as s16b,
                                  8 as libc::c_int as s16b) +
                              (*p_ptr).lev as libc::c_int / 3 as libc::c_int);
            }
        }
        56 => {
            /* BO_PLAS */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(12 as libc::c_int, dir,
                          damroll(8 as libc::c_int as s16b,
                                  8 as libc::c_int as s16b) +
                              (*p_ptr).lev as libc::c_int / 3 as libc::c_int);
            }
        }
        57 => {
            /* BO_ICEE */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(28 as libc::c_int, dir,
                          damroll(6 as libc::c_int as s16b,
                                  6 as libc::c_int as s16b) +
                              (*p_ptr).lev as libc::c_int / 3 as libc::c_int);
            }
        }
        58 => {
            /* MISSILE */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(10 as libc::c_int, dir,
                          damroll(2 as libc::c_int as s16b,
                                  6 as libc::c_int as s16b) +
                              (*p_ptr).lev as libc::c_int / 3 as libc::c_int);
            }
        }
        59 => {
            /* SCARE */
            if !(get_aim_dir(&mut dir) == 0) { fear_monster(dir, plev); }
        }
        60 => {
            /* BLIND */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(22 as libc::c_int, dir,
                          damroll(1 as libc::c_int as s16b,
                                  8 as libc::c_int as s16b) +
                              (*p_ptr).lev as libc::c_int / 3 as libc::c_int);
            }
        }
        61 => {
            /* CONF */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(22 as libc::c_int, dir,
                          damroll(7 as libc::c_int as s16b,
                                  8 as libc::c_int as s16b) +
                              (*p_ptr).lev as libc::c_int / 3 as libc::c_int);
            }
        }
        62 => {
            /* SLOW */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(55 as libc::c_int, dir,
                          damroll(6 as libc::c_int as s16b,
                                  8 as libc::c_int as s16b) +
                              (*p_ptr).lev as libc::c_int / 3 as libc::c_int);
            }
        }
        63 => {
            /* HOLD */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(57 as libc::c_int, dir,
                          damroll(5 as libc::c_int as s16b,
                                  8 as libc::c_int as s16b) +
                              (*p_ptr).lev as libc::c_int / 3 as libc::c_int);
            }
        }
        64 => {
            /* *** RF6 (bit position + 64) ****/
            /* HASTE */
            if (*p_ptr).fast == 0 {
                set_fast(Rand_div(20 as libc::c_int + plev) + 1 as libc::c_int
                             + plev, 10 as libc::c_int);
            } else {
                set_fast((*p_ptr).fast as libc::c_int +
                             (Rand_div(5 as libc::c_int) + 1 as libc::c_int),
                         10 as libc::c_int);
            }
        }
        65 => {
            /* HAND_DOOM */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_bolt(26 as libc::c_int, dir,
                          damroll(10 as libc::c_int as s16b,
                                  8 as libc::c_int as s16b) +
                              (*p_ptr).lev as libc::c_int);
            }
        }
        66 => {
            /* HEAL */
            hp_player(damroll(8 as libc::c_int as s16b,
                              5 as libc::c_int as s16b));
        }
        67 => {
            /* S_ANIMALS */
            k = 0 as libc::c_int;
            while k < 4 as libc::c_int {
                summon_specific_friendly(y, x, rlev, 42 as libc::c_int,
                                         1 as libc::c_int as bool_);
                k += 1
            }
        }
        68 => {
            /* BLINK */
            if dungeon_flags2 as libc::c_long & 0x8 as libc::c_long != 0 {
                msg_print(b"No teleport on special levels...\x00" as *const u8
                              as *const libc::c_char);
            } else { teleport_player(10 as libc::c_int); }
        }
        69 => {
            /* TPORT */
            if dungeon_flags2 as libc::c_long & 0x8 as libc::c_long != 0 {
                msg_print(b"No teleport on special levels...\x00" as *const u8
                              as *const libc::c_char);
            } else { teleport_player(plev * 5 as libc::c_int); }
        }
        70 => {
            /* TELE_TO */
            let mut ii: libc::c_int = 0;
            let mut ij: libc::c_int = 0;
            if dungeon_flags2 as libc::c_long & 0x8 as libc::c_long != 0 {
                msg_print(b"No teleport on special levels...\x00" as *const u8
                              as *const libc::c_char);
            } else {
                msg_print(b"You go between.\x00" as *const u8 as
                              *const libc::c_char);
                if !(tgt_pt(&mut ii, &mut ij) == 0) {
                    (*p_ptr).energy -= 60 as libc::c_int - plev;
                    if !((*f_info.offset((*cave[ij as
                                                    usize].offset(ii as
                                                                      isize)).feat
                                             as isize)).flags1 as libc::c_long
                             & 0x10 as libc::c_long != 0 &&
                             (*cave[ij as usize].offset(ii as isize)).feat as
                                 libc::c_int != 0xaf as libc::c_int &&
                             (*cave[ij as usize].offset(ii as isize)).m_idx ==
                                 0 &&
                             !(ij == (*p_ptr).py as libc::c_int &&
                                   ii == (*p_ptr).px as libc::c_int)) ||
                           (*cave[ij as usize].offset(ii as isize)).info as
                               libc::c_int & 0x4 as libc::c_int != 0 ||
                           distance(ij, ii, (*p_ptr).py as libc::c_int,
                                    (*p_ptr).px as libc::c_int) >
                               plev * 20 as libc::c_int + 2 as libc::c_int {
                        msg_print(b"You fail to show the destination correctly!\x00"
                                      as *const u8 as *const libc::c_char);
                        (*p_ptr).energy -= 100 as libc::c_int;
                        teleport_player(10 as libc::c_int);
                    } else { teleport_player_to(ij, ii); }
                }
            }
        }
        71 => {
            /* TELE_AWAY */
            if dungeon_flags2 as libc::c_long & 0x8 as libc::c_long != 0 {
                msg_print(b"No teleport on special levels...\x00" as *const u8
                              as *const libc::c_char);
            } else if !(get_aim_dir(&mut dir) == 0) {
                fire_beam(63 as libc::c_int, dir, plev);
            }
        }
        72 => {
            /* TELE_LEVEL */
            if dungeon_flags2 as libc::c_long & 0x8 as libc::c_long != 0 {
                msg_print(b"No teleport on special levels...\x00" as *const u8
                              as *const libc::c_char);
            } else { teleport_player_level(); }
        }
        73 => {
            /* DARKNESS */
            project(-(1 as libc::c_int), 3 as libc::c_int,
                    (*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                    0 as libc::c_int, 18 as libc::c_int,
                    0x10 as libc::c_int | 0x40 as libc::c_int);
            /* Unlite the room */
            unlite_room((*p_ptr).py as libc::c_int,
                        (*p_ptr).px as libc::c_int);
        }
        74 => {
            /* TRAPS */
            trap_creation();
        }
        76 => {
            /* 75 FORGET -- Not available */
            /* ANIM_DEAD -- Use the same code as the nether spell */
            if !(get_aim_dir(&mut dir) == 0) {
                fire_ball(92 as libc::c_int, dir, 1 as libc::c_int,
                          0 as libc::c_int);
            }
        }
        79 => {
            /* 77 S_BUG -- Not available, well we do that anyway ;) */
            /* 78 S_RNG -- Not available, who dares? */
            /* S_THUNDERLORD */
            k = 0 as libc::c_int;
            while k < 1 as libc::c_int {
                summon_specific_friendly(y, x, rlev, 49 as libc::c_int,
                                         1 as libc::c_int as bool_);
                k += 1
            }
        }
        80 => {
            /* S_KIN -- Summon Kin, because we code bugs :) */
            /* Big hack */
            summon_kin_type = (*r_ptr).d_char;
            k = 0 as libc::c_int;
            while k < 6 as libc::c_int {
                summon_specific_friendly(y, x, rlev, 40 as libc::c_int,
                                         1 as libc::c_int as bool_);
                k += 1
            }
        }
        81 => {
            /* S_HI_DEMON */
            k = 0 as libc::c_int;
            while k < 1 as libc::c_int {
                summon_specific_friendly(y, x, rlev, 39 as libc::c_int,
                                         1 as libc::c_int as bool_);
                k += 1
            }
        }
        82 => {
            /* S_MONSTER */
            k = 0 as libc::c_int;
            while k < 1 as libc::c_int {
                summon_specific_friendly(y, x, rlev, 0 as libc::c_int,
                                         1 as libc::c_int as bool_);
                k += 1
            }
        }
        83 => {
            /* S_MONSTERS */
            k = 0 as libc::c_int;
            while k < 6 as libc::c_int {
                summon_specific_friendly(y, x, rlev, 0 as libc::c_int,
                                         1 as libc::c_int as bool_);
                k += 1
            }
        }
        84 => {
            /* S_ANT */
            k = 0 as libc::c_int;
            while k < 6 as libc::c_int {
                summon_specific_friendly(y, x, rlev, 11 as libc::c_int,
                                         1 as libc::c_int as bool_);
                k += 1
            }
        }
        85 => {
            /* S_SPIDER */
            k = 0 as libc::c_int;
            while k < 6 as libc::c_int {
                summon_specific_friendly(y, x, rlev, 12 as libc::c_int,
                                         1 as libc::c_int as bool_);
                k += 1
            }
        }
        86 => {
            /* S_HOUND */
            k = 0 as libc::c_int;
            while k < 6 as libc::c_int {
                summon_specific_friendly(y, x, rlev, 13 as libc::c_int,
                                         1 as libc::c_int as bool_);
                k += 1
            }
        }
        87 => {
            /* S_HYDRA */
            k = 0 as libc::c_int;
            while k < 6 as libc::c_int {
                summon_specific_friendly(y, x, rlev, 14 as libc::c_int,
                                         1 as libc::c_int as bool_);
                k += 1
            }
        }
        88 => {
            /* S_ANGEL */
            k = 0 as libc::c_int;
            while k < 1 as libc::c_int {
                summon_specific_friendly(y, x, rlev, 15 as libc::c_int,
                                         1 as libc::c_int as bool_);
                k += 1
            }
        }
        89 => {
            /* S_DEMON */
            k = 0 as libc::c_int;
            while k < 1 as libc::c_int {
                summon_specific_friendly(y, x, rlev, 16 as libc::c_int,
                                         1 as libc::c_int as bool_);
                k += 1
            }
        }
        90 => {
            /* S_UNDEAD */
            k = 0 as libc::c_int;
            while k < 1 as libc::c_int {
                summon_specific_friendly(y, x, rlev, 17 as libc::c_int,
                                         1 as libc::c_int as bool_);
                k += 1
            }
        }
        91 => {
            /* S_DRAGON */
            k = 0 as libc::c_int;
            while k < 1 as libc::c_int {
                summon_specific_friendly(y, x, rlev, 18 as libc::c_int,
                                         1 as libc::c_int as bool_);
                k += 1
            }
        }
        92 => {
            /* S_HI_UNDEAD */
            k = 0 as libc::c_int;
            while k < 8 as libc::c_int {
                summon_specific_friendly(y, x, rlev, 44 as libc::c_int,
                                         1 as libc::c_int as bool_);
                k += 1
            }
        }
        93 => {
            /* S_HI_DRAGON */
            k = 0 as libc::c_int;
            while k < 8 as libc::c_int {
                summon_specific_friendly(y, x, rlev, 45 as libc::c_int,
                                         1 as libc::c_int as bool_);
                k += 1
            }
        }
        94 => {
            /* S_WRAITH */
            k = 0 as libc::c_int;
            while k < 8 as libc::c_int {
                summon_specific_friendly(y, x, rlev, 31 as libc::c_int,
                                         1 as libc::c_int as bool_);
                k += 1
            }
        }
        _ => { }
    }
    /* Take some SP */
    if no_cost == 0 {
        let mut chance: libc::c_int = 0;
        let mut pchance: libc::c_int = 0;
        chance =
            monster_powers[power as usize].mana +
                (*r_ptr).level as libc::c_int;
        pchance =
            *adj_str_wgt.as_mut_ptr().offset((*p_ptr).stat_ind[2 as
                                                                   libc::c_int
                                                                   as usize]
                                                 as isize) as libc::c_int /
                2 as libc::c_int +
                get_skill(50 as libc::c_int) as libc::c_int;
        if Rand_div(chance) >= pchance {
            let mut m: libc::c_int =
                monster_powers[power as usize].mana / 10 as libc::c_int;
            if m > (*p_ptr).msp as libc::c_int {
                m = (*p_ptr).msp as libc::c_int
            }
            if m == 0 { m = 1 as libc::c_int }
            (*p_ptr).csp = ((*p_ptr).csp as libc::c_int - m) as s16b
        }
    }
    /* Redraw mana */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x80 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
    return num;
}
/*
 * Schooled magic
 */
/*
 * Find a spell in any books/objects
 */
static mut hack_force_spell: libc::c_int = -(1 as libc::c_int);
static mut hack_force_spell_obj: *mut object_type =
    0 as *const object_type as *mut object_type;
#[no_mangle]
pub unsafe extern "C" fn get_item_hook_find_spell(mut item: *mut libc::c_int)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut spell: libc::c_int = 0;
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut buf2: [libc::c_char; 100] = [0; 100];
    strcpy(buf.as_mut_ptr(),
           b"Manathrust\x00" as *const u8 as *const libc::c_char);
    if get_string(b"Spell name? \x00" as *const u8 as *const libc::c_char,
                  buf.as_mut_ptr(), 79 as libc::c_int) == 0 {
        return 0 as libc::c_int as bool_
    }
    sprintf(buf2.as_mut_ptr(),
            b"return find_spell(\"%s\")\x00" as *const u8 as
                *const libc::c_char, buf.as_mut_ptr());
    spell = exec_lua(buf2.as_mut_ptr());
    if spell == -(1 as libc::c_int) { return 0 as libc::c_int as bool_ }
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        let mut f1: u32b = 0;
        let mut f2: u32b = 0;
        let mut f3: u32b = 0;
        let mut f4: u32b = 0;
        let mut f5: u32b = 0;
        let mut esp: u32b = 0;
        /* Must we wield it ? */
        object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                     &mut esp);
        if !(wield_slot(o_ptr) as libc::c_int != -(1 as libc::c_int) &&
                 i < 24 as libc::c_int &&
                 f5 as libc::c_long & 0x10000 as libc::c_long != 0) {
            /* Is it a non-book? */
            if is_school_book(o_ptr) == 0 {
                let mut f1_0: u32b = 0;
                let mut f2_0: u32b = 0;
                let mut f3_0: u32b = 0;
                let mut f4_0: u32b = 0;
                let mut f5_0: u32b = 0;
                let mut esp_0: u32b = 0;
                /* Extract object flags */
                object_flags(o_ptr, &mut f1_0, &mut f2_0, &mut f3_0,
                             &mut f4_0, &mut f5_0, &mut esp_0);
                if f5_0 as libc::c_long & 0x800 as libc::c_long != 0 &&
                       (*o_ptr).pval2 as libc::c_int == spell {
                    *item = i;
                    hack_force_spell = spell;
                    hack_force_spell_obj = o_ptr;
                    return 1 as libc::c_int as bool_
                }
            } else if (*o_ptr).sval as libc::c_int == 255 as libc::c_int &&
                          (*o_ptr).pval == spell {
                *item = i;
                hack_force_spell = spell;
                hack_force_spell_obj = o_ptr;
                return 1 as libc::c_int as bool_
            } else {
                /* A random book ? */
                /* A normal book */
                if (*o_ptr).sval as libc::c_int != 255 as libc::c_int {
                    sprintf(buf2.as_mut_ptr(),
                            b"return spell_in_book(%d, %d)\x00" as *const u8
                                as *const libc::c_char,
                            (*o_ptr).sval as libc::c_int, spell);
                    if exec_lua(buf2.as_mut_ptr()) != 0 {
                        *item = i;
                        hack_force_spell = spell;
                        hack_force_spell_obj = o_ptr;
                        return 1 as libc::c_int as bool_
                    }
                }
            }
        }
        i += 1
    }
    return 0 as libc::c_int as bool_;
}
/*
 * Get a spell from a book
 */
#[no_mangle]
pub unsafe extern "C" fn get_school_spell(mut do_what: cptr,
                                          mut check_fct: cptr,
                                          mut force_book: s16b) -> s32b {
    let mut i: libc::c_int = 0;
    let mut item: libc::c_int = 0;
    let mut spell: s32b = -(1 as libc::c_int);
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut where_0: s32b = 1 as libc::c_int;
    let mut ask: libc::c_int = 0;
    let mut flag: bool_ = 0;
    let mut redraw: bool_ = 0;
    let mut choice: libc::c_char = 0;
    let mut out_val: [libc::c_char; 160] = [0; 160];
    let mut buf2: [libc::c_char; 40] = [0; 40];
    let mut buf3: [libc::c_char; 40] = [0; 40];
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
    let mut tmp: libc::c_int = 0;
    let mut sval: libc::c_int = 0;
    let mut pval: libc::c_int = 0;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    hack_force_spell = -(1 as libc::c_int);
    hack_force_spell_obj = 0 as *mut object_type;
    /* Ok do we need to ask for a book ? */
    if force_book == 0 {
        get_item_extra_hook =
            Some(get_item_hook_find_spell as
                     unsafe extern "C" fn(_: *mut libc::c_int) -> bool_);
        item_tester_hook =
            Some(hook_school_spellable as
                     unsafe extern "C" fn(_: *mut object_type) -> bool_);
        sprintf(buf2.as_mut_ptr(),
                b"You have no book to %s from\x00" as *const u8 as
                    *const libc::c_char, do_what);
        sprintf(buf3.as_mut_ptr(),
                b"%s from which book?\x00" as *const u8 as
                    *const libc::c_char, do_what);
        if get_item(&mut item, buf3.as_mut_ptr() as cptr,
                    buf2.as_mut_ptr() as cptr,
                    0x2 as libc::c_int | 0x1 as libc::c_int |
                        0x8 as libc::c_int) == 0 {
            return -(1 as libc::c_int)
        }
        /* Get the item */
        o_ptr = get_object(item);
        object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                     &mut esp);
        /* If it can be wielded, it must */
        if wield_slot(o_ptr) as libc::c_int != -(1 as libc::c_int) &&
               item < 24 as libc::c_int &&
               f5 as libc::c_long & 0x10000 as libc::c_long != 0 {
            msg_format(b"You cannot %s from that object; it must be wielded first.\x00"
                           as *const u8 as *const libc::c_char, do_what);
            return -(1 as libc::c_int)
        }
    } else {
        o_ptr = &mut forge;
        (*o_ptr).tval = 111 as libc::c_int as byte_hack;
        (*o_ptr).sval = force_book as byte_hack;
        (*o_ptr).pval = 0 as libc::c_int
    }
    if repeat_pull(&mut tmp) != 0 { return tmp }
    /* Nothing chosen yet */
    flag = 0 as libc::c_int as bool_;
    /* No redraw yet */
    redraw = 0 as libc::c_int as bool_;
    /* Show choices */
    if show_choices != 0 {
        /* Window stuff */
        window_stuff();
    }
    /* No spell to cast by default */
    spell = -(1 as libc::c_int);
    /* Is it a random book, or something else ? */
    if is_school_book(o_ptr) != 0 {
        sval = (*o_ptr).sval as libc::c_int;
        pval = (*o_ptr).pval
    } else { sval = 255 as libc::c_int; pval = (*o_ptr).pval2 as libc::c_int }
    if hack_force_spell == -(1 as libc::c_int) {
        num =
            exec_lua(format(b"return book_spells_num(%d)\x00" as *const u8 as
                                *const libc::c_char, sval));
        /* Build a prompt (accept all spells) */
        strnfmt(out_val.as_mut_ptr(), 78 as libc::c_int as uint_hack,
                b"(Spells %c-%c, Descs %c-%c, *=List, ESC=exit) %^s which spell? \x00"
                    as *const u8 as *const libc::c_char,
                0 as libc::c_int + 'a' as i32,
                num - 1 as libc::c_int + 'a' as i32,
                0 as libc::c_int + 'a' as i32 - 'a' as i32 + 'A' as i32,
                num - 1 as libc::c_int + 'a' as i32 - 'a' as i32 + 'A' as i32,
                do_what);
        /* Get a spell from the user */
        while flag == 0 &&
                  get_com(out_val.as_mut_ptr() as cptr, &mut choice) as
                      libc::c_int != 0 {
            /* Request redraw */
            if choice as libc::c_int == ' ' as i32 ||
                   choice as libc::c_int == '*' as i32 ||
                   choice as libc::c_int == '?' as i32 {
                /* Show the list */
                if redraw == 0 {
                    /* Show list */
                    redraw = 1 as libc::c_int as bool_;
                    /* Save the screen */
                    character_icky = 1 as libc::c_int as bool_;
                    Term_save();
                    /* Display a list of spells */
                    call_lua(b"print_book\x00" as *const u8 as
                                 *const libc::c_char,
                             b"(d,d,O)\x00" as *const u8 as
                                 *const libc::c_char,
                             b"d\x00" as *const u8 as *const libc::c_char,
                             sval, pval, o_ptr, &mut where_0 as *mut s32b);
                } else {
                    /* Hide the list */
                    /* Hide list */
                    redraw = 0 as libc::c_int as bool_;
                    where_0 = 1 as libc::c_int;
                    Term_load();
                    character_icky = 0 as libc::c_int as bool_
                }
            } else {
                /* Restore the screen */
                /* Note verify */
                ask =
                    *(*__ctype_b_loc()).offset(choice as libc::c_int as isize)
                        as libc::c_int &
                        _ISupper as libc::c_int as libc::c_ushort as
                            libc::c_int;
                /* Lowercase */
                if ask != 0 {
                    choice = tolower(choice as libc::c_int) as libc::c_char
                }
                /* Extract request */
                i =
                    if *(*__ctype_b_loc()).offset(choice as libc::c_int as
                                                      isize) as libc::c_int &
                           _ISlower as libc::c_int as libc::c_ushort as
                               libc::c_int != 0 {
                        (choice as libc::c_int) - 'a' as i32
                    } else { -(1 as libc::c_int) };
                /* Totally Illegal */
                if i < 0 as libc::c_int || i >= num {
                    bell();
                } else if ask != 0 {
                    /* Verify it */
                    /* Show the list */
                    if redraw == 0 {
                        /* Show list */
                        redraw = 1 as libc::c_int as bool_;
                        /* Save the screen */
                        character_icky = 1 as libc::c_int as bool_;
                        Term_load();
                        Term_save();
                    } else {
                        /* Rstore the screen */
                        /* Restore the screen */
                        Term_load();
                    }
                    /* Display a list of spells */
                    call_lua(b"print_book\x00" as *const u8 as
                                 *const libc::c_char,
                             b"(d,d,O)\x00" as *const u8 as
                                 *const libc::c_char,
                             b"d\x00" as *const u8 as *const libc::c_char,
                             sval, pval, o_ptr, &mut where_0 as *mut s32b);
                    exec_lua(format(b"print_spell_desc(spell_x(%d, %d, %d), %d)\x00"
                                        as *const u8 as *const libc::c_char,
                                    sval, pval, i, where_0));
                } else {
                    let mut ok: s32b = 0;
                    /* Save the spell index */
                    spell =
                        exec_lua(format(b"return spell_x(%d, %d, %d)\x00" as
                                            *const u8 as *const libc::c_char,
                                        sval, pval, i));
                    /* Do we need to do some pre test */
                    call_lua(check_fct,
                             b"(d,O)\x00" as *const u8 as *const libc::c_char,
                             b"d\x00" as *const u8 as *const libc::c_char,
                             spell, o_ptr, &mut ok as *mut s32b);
                    /* Require "okay" spells */
                    if ok == 0 {
                        bell();
                        msg_format(b"You may not %s that spell.\x00" as
                                       *const u8 as *const libc::c_char,
                                   do_what);
                        spell = -(1 as libc::c_int)
                    } else {
                        /* Stop the loop */
                        flag = 1 as libc::c_int as bool_
                    }
                }
            }
        }
    } else {
        let mut ok_0: s32b = 0;
        /* Require "okay" spells */
        call_lua(check_fct, b"(d, O)\x00" as *const u8 as *const libc::c_char,
                 b"d\x00" as *const u8 as *const libc::c_char,
                 hack_force_spell, hack_force_spell_obj,
                 &mut ok_0 as *mut s32b);
        if ok_0 != 0 {
            flag = 1 as libc::c_int as bool_;
            spell = hack_force_spell
        } else {
            bell();
            msg_format(b"You may not %s that spell.\x00" as *const u8 as
                           *const libc::c_char, do_what);
            spell = -(1 as libc::c_int)
        }
    }
    /* Restore the screen */
    if redraw != 0 { Term_load(); character_icky = 0 as libc::c_int as bool_ }
    /* Show choices */
    if show_choices != 0 {
        /* Window stuff */
        window_stuff();
    }
    /* Abort if needed */
    if flag == 0 { return -(1 as libc::c_int) }
    tmp = spell;
    repeat_push(tmp);
    return spell;
}
#[no_mangle]
pub unsafe extern "C" fn cast_school_spell() {
    let mut spell: libc::c_int = 0;
    /* No magic */
    if (*p_ptr).antimagic != 0 {
        msg_print(b"Your anti-magic field disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    /* No magic */
    if (*p_ptr).anti_magic != 0 {
        msg_print(b"Your anti-magic shell disrupts any magic attempts.\x00" as
                      *const u8 as *const libc::c_char);
        return
    }
    spell =
        get_school_spell(b"cast\x00" as *const u8 as *const libc::c_char,
                         b"is_ok_spell\x00" as *const u8 as
                             *const libc::c_char, 0 as libc::c_int as s16b);
    /* Actualy cast the choice */
    if spell != -(1 as libc::c_int) {
        exec_lua(format(b"cast_school_spell(%d, spell(%d))\x00" as *const u8
                            as *const libc::c_char, spell, spell));
    };
}
#[no_mangle]
pub unsafe extern "C" fn browse_school_spell(mut book: libc::c_int,
                                             mut pval: libc::c_int,
                                             mut o_ptr: *mut object_type) {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut where_0: libc::c_int = 1 as libc::c_int;
    let mut ask: libc::c_int = 0;
    let mut choice: libc::c_char = 0;
    let mut out_val: [libc::c_char; 160] = [0; 160];
    /* Show choices */
    if show_choices != 0 {
        /* Window stuff */
        window_stuff();
    }
    num =
        exec_lua(format(b"return book_spells_num(%d)\x00" as *const u8 as
                            *const libc::c_char, book));
    /* Build a prompt (accept all spells) */
    strnfmt(out_val.as_mut_ptr(), 78 as libc::c_int as uint_hack,
            b"(Spells %c-%c, ESC=exit) cast which spell? \x00" as *const u8 as
                *const libc::c_char, 0 as libc::c_int + 'a' as i32,
            num - 1 as libc::c_int + 'a' as i32);
    /* Save the screen */
    character_icky = 1 as libc::c_int as bool_;
    Term_save();
    /* Display a list of spells */
    call_lua(b"print_book\x00" as *const u8 as *const libc::c_char,
             b"(d,d,O)\x00" as *const u8 as *const libc::c_char,
             b"d\x00" as *const u8 as *const libc::c_char, book, pval, o_ptr,
             &mut where_0 as *mut libc::c_int);
    /* Get a spell from the user */
    while get_com(out_val.as_mut_ptr() as cptr, &mut choice) != 0 {
        /* Display a list of spells */
        call_lua(b"print_book\x00" as *const u8 as *const libc::c_char,
                 b"(d,d,O)\x00" as *const u8 as *const libc::c_char,
                 b"d\x00" as *const u8 as *const libc::c_char, book, pval,
                 o_ptr, &mut where_0 as *mut libc::c_int);
        /* Note verify */
        ask =
            *(*__ctype_b_loc()).offset(choice as libc::c_int as isize) as
                libc::c_int &
                _ISupper as libc::c_int as libc::c_ushort as libc::c_int;
        /* Lowercase */
        if ask != 0 {
            choice = tolower(choice as libc::c_int) as libc::c_char
        }
        /* Extract request */
        i =
            if *(*__ctype_b_loc()).offset(choice as libc::c_int as isize) as
                   libc::c_int &
                   _ISlower as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 {
                (choice as libc::c_int) - 'a' as i32
            } else { -(1 as libc::c_int) };
        /* Totally Illegal */
        if i < 0 as libc::c_int || i >= num {
            bell();
        } else {
            /* Restore the screen */
            Term_load();
            /* Display a list of spells */
            call_lua(b"print_book\x00" as *const u8 as *const libc::c_char,
                     b"(d,d,O)\x00" as *const u8 as *const libc::c_char,
                     b"d\x00" as *const u8 as *const libc::c_char, book, pval,
                     o_ptr, &mut where_0 as *mut libc::c_int);
            exec_lua(format(b"print_spell_desc(spell_x(%d, %d, %d), %d)\x00"
                                as *const u8 as *const libc::c_char, book,
                            pval, i, where_0));
        }
    }
    /* Restore the screen */
    Term_load();
    character_icky = 0 as libc::c_int as bool_;
    /* Show choices */
    if show_choices != 0 {
        /* Window stuff */
        window_stuff();
    };
}
/* Can it contains a schooled spell ? */
unsafe extern "C" fn hook_school_can_spellable(mut o_ptr: *mut object_type)
 -> bool_ {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    /* Extract object flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    if f5 as libc::c_long & 0x800 as libc::c_long != 0 &&
           (*o_ptr).pval2 as libc::c_int == -(1 as libc::c_int) {
        return 1 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
/*
 * Copy a spell from a bok to an object
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_copy_spell() {
    let mut spell: libc::c_int =
        get_school_spell(b"copy\x00" as *const u8 as *const libc::c_char,
                         b"is_ok_spell\x00" as *const u8 as
                             *const libc::c_char, 0 as libc::c_int as s16b);
    let mut item: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    if spell == -(1 as libc::c_int) { return }
    /* Spells that cannot be randomly created cannot be copied */
    if exec_lua(format(b"return can_spell_random(%d)\x00" as *const u8 as
                           *const libc::c_char, spell)) == 0 as libc::c_int {
        msg_print(b"This spell cannot be copied.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    item_tester_hook =
        Some(hook_school_can_spellable as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    if get_item(&mut item,
                b"Copy to which object? \x00" as *const u8 as
                    *const libc::c_char,
                b"You have no object to copy to.\x00" as *const u8 as
                    *const libc::c_char,
                0x2 as libc::c_int | 0x1 as libc::c_int) == 0 {
        return
    }
    o_ptr = get_object(item);
    msg_print(b"You copy the spell!\x00" as *const u8 as *const libc::c_char);
    (*o_ptr).pval2 = spell as s16b;
    inven_item_describe(item);
}
/*
 * Finds a spell by name, optimized for speed
 */
#[no_mangle]
pub unsafe extern "C" fn find_spell(mut name: *mut libc::c_char)
 -> libc::c_int {
    let mut oldtop: libc::c_int = 0;
    let mut spell: libc::c_int = 0;
    oldtop = lua_gettop(L);
    lua_getglobal(L, b"find_spell\x00" as *const u8 as *const libc::c_char);
    tolua_pushstring(L, name);
    /* Call the function */
    if lua_call(L, 1 as libc::c_int, 1 as libc::c_int) != 0 {
        cmsg_format(10 as libc::c_int as byte_hack,
                    b"ERROR in lua_call while calling \'find_spell\'.\x00" as
                        *const u8 as *const libc::c_char);
        lua_settop(L, oldtop);
        return -(1 as libc::c_int)
    }
    spell =
        tolua_getnumber(L, -(lua_gettop(L) - oldtop),
                        -(1 as libc::c_int) as libc::c_long) as libc::c_int;
    lua_settop(L, oldtop);
    return spell;
}
