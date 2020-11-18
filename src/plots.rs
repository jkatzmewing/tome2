use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type lua_State;
    #[no_mangle]
    static mut energy_use: s32b;
    #[no_mangle]
    static mut death: bool_;
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
    static mut total_winner: u16b;
    #[no_mangle]
    static mut has_won: u16b;
    #[no_mangle]
    static mut m_max: s16b;
    #[no_mangle]
    static mut quick_messages: bool_;
    #[no_mangle]
    static mut rating: s16b;
    #[no_mangle]
    static mut feat_wall_outer: byte_hack;
    #[no_mangle]
    static mut feat_wall_inner: byte_hack;
    #[no_mangle]
    static mut floor_type: [s16b; 100];
    #[no_mangle]
    static mut fill_type: [s16b; 100];
    #[no_mangle]
    static mut died_from: [libc::c_char; 80];
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut o_list: *mut object_type;
    #[no_mangle]
    static mut m_list: *mut monster_type;
    #[no_mangle]
    static mut town_info: *mut town_type;
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut sp_ptr: *mut player_sex;
    #[no_mangle]
    static mut s_info: *mut skill_type;
    #[no_mangle]
    static mut f_info: *mut feature_type;
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
    static mut get_mon_num_hook:
           Option<unsafe extern "C" fn(_: libc::c_int) -> bool_>;
    #[no_mangle]
    static mut init_flags: libc::c_int;
    #[no_mangle]
    static mut random_artifacts: [random_artifact; 84];
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut m_allow_special: *mut bool_;
    #[no_mangle]
    static mut k_allow_special: *mut bool_;
    #[no_mangle]
    static mut a_allow_special: *mut bool_;
    #[no_mangle]
    static mut random_quests: [random_quest; 99];
    #[no_mangle]
    static mut dungeon_flags1: u32b;
    #[no_mangle]
    static mut dungeon_flags2: u32b;
    #[no_mangle]
    static mut quest: *mut quest_type;
    #[no_mangle]
    fn distance(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int,
                x2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn los(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int, x2: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn lite_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn cave_set_feat(y: libc::c_int, x: libc::c_int, feat: libc::c_int);
    #[no_mangle]
    fn scatter(yp: *mut libc::c_int, xp: *mut libc::c_int, y: libc::c_int,
               x: libc::c_int, d: libc::c_int);
    #[no_mangle]
    fn generate_fracave(y0: libc::c_int, x0: libc::c_int, xsize: libc::c_int,
                        ysize: libc::c_int, cutoff: libc::c_int, light: bool_,
                        room: bool_) -> bool_;
    #[no_mangle]
    fn generate_hmap(y0: libc::c_int, x0: libc::c_int, xsiz: libc::c_int,
                     ysiz: libc::c_int, grd: libc::c_int, roug: libc::c_int,
                     cutoff: libc::c_int);
    #[no_mangle]
    fn room_alloc(x: libc::c_int, y: libc::c_int, crowded: bool_,
                  by0: libc::c_int, bx0: libc::c_int, xx: *mut libc::c_int,
                  yy: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn build_rectangle(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int,
                       x2: libc::c_int, feat: libc::c_int, info: libc::c_int);
    #[no_mangle]
    fn process_dungeon_file(name: cptr, yval: *mut libc::c_int,
                            xval: *mut libc::c_int, ymax: libc::c_int,
                            xmax: libc::c_int, init: bool_, full: bool_)
     -> errr;
    #[no_mangle]
    fn monster_check_experience(m_idx: libc::c_int, silent: bool_);
    #[no_mangle]
    fn delete_monster_idx(i: libc::c_int);
    #[no_mangle]
    fn get_mon_num_prep() -> errr;
    #[no_mangle]
    fn get_mon_num(level: libc::c_int) -> s16b;
    #[no_mangle]
    fn place_monster_one(y: libc::c_int, x: libc::c_int, r_idx: libc::c_int,
                         ego: libc::c_int, slp: bool_, status: libc::c_int)
     -> s16b;
    #[no_mangle]
    fn set_mon_num_hook();
    #[no_mangle]
    fn can_create_companion() -> bool_;
    #[no_mangle]
    fn inc_stack_size_ex(item: libc::c_int, delta: libc::c_int,
                         opt: optimize_flag, desc: describe_flag);
    #[no_mangle]
    fn get_object(item: libc::c_int) -> *mut object_type;
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn inven_item_describe(item: libc::c_int);
    #[no_mangle]
    fn inven_item_optimize(item: libc::c_int) -> bool_;
    #[no_mangle]
    fn inven_carry_okay(o_ptr: *mut object_type) -> bool_;
    #[no_mangle]
    fn inven_carry(o_ptr: *mut object_type, final_0: bool_) -> s16b;
    #[no_mangle]
    fn inven_drop(item: libc::c_int, amt: libc::c_int, dy: libc::c_int,
                  dx: libc::c_int, silent: bool_);
    #[no_mangle]
    fn o_pop() -> s16b;
    #[no_mangle]
    fn object_known(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_aware(o_ptr: *mut object_type);
    #[no_mangle]
    fn lookup_kind(tval: libc::c_int, sval: libc::c_int) -> s16b;
    #[no_mangle]
    fn object_wipe(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_prep(o_ptr: *mut object_type, k_idx: libc::c_int);
    #[no_mangle]
    fn object_copy(o_ptr: *mut object_type, j_ptr: *mut object_type);
    #[no_mangle]
    fn apply_magic(o_ptr: *mut object_type, lev: libc::c_int, okay: bool_,
                   good: bool_, great: bool_);
    #[no_mangle]
    fn make_object(j_ptr: *mut object_type, good: bool_, great: bool_,
                   theme: obj_theme) -> bool_;
    #[no_mangle]
    fn drop_near(o_ptr: *mut object_type, chance: libc::c_int, y: libc::c_int,
                 x: libc::c_int) -> s16b;
    #[no_mangle]
    fn place_trap(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn aggravate_monsters(who: libc::c_int);
    #[no_mangle]
    fn create_artifact(o_ptr: *mut object_type, a_scroll: bool_,
                       get_name: bool_) -> bool_;
    #[no_mangle]
    fn ask_menu(ask: cptr, items: *mut *mut libc::c_char, max: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn bst(what: s32b, t: s32b) -> s32b;
    #[no_mangle]
    fn flush();
    #[no_mangle]
    fn quark_add(str: cptr) -> s16b;
    #[no_mangle]
    fn message_add(type_0: byte_hack, msg: cptr, color: byte_hack);
    #[no_mangle]
    fn cmsg_print(color: byte_hack, msg: cptr);
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn cmsg_format(color: byte_hack, fmt: cptr, _: ...);
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn c_put_str(attr: byte_hack, str: cptr, row: libc::c_int,
                 col: libc::c_int);
    #[no_mangle]
    fn get_check(prompt: cptr) -> bool_;
    #[no_mangle]
    fn test_monster_name(name: cptr) -> libc::c_int;
    #[no_mangle]
    fn test_item_name(name: cptr) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn damroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn ralloc(len: huge_hack) -> vptr;
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn exec_lua(file: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn get_map_size(name: *mut libc::c_char, ysize: *mut libc::c_int,
                    xsize: *mut libc::c_int);
    #[no_mangle]
    fn do_get_new_skill();
    #[no_mangle]
    fn inc_piety(god: libc::c_int, amt: s32b);
    #[no_mangle]
    fn abandon_god(god: libc::c_int);
    #[no_mangle]
    fn lua_gettop(L_0: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L_0: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_isnumber(L_0: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_isstring(L_0: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_getglobal(L_0: *mut lua_State, name: *const libc::c_char);
    #[no_mangle]
    fn lua_call(L_0: *mut lua_State, nargs: libc::c_int,
                nresults: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tolua_tag(L_0: *mut lua_State, type_0: *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn tolua_getnumber(L_0: *mut lua_State, narg: libc::c_int,
                       def: libc::c_long) -> libc::c_long;
    #[no_mangle]
    fn tolua_getstring(L_0: *mut lua_State, narg: libc::c_int,
                       def: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn tolua_getuserdata(L_0: *mut lua_State, narg: libc::c_int,
                         def: *mut libc::c_void) -> *mut libc::c_void;
    #[no_mangle]
    fn tolua_pushnumber(L_0: *mut lua_State, value: libc::c_long);
    #[no_mangle]
    fn tolua_pushstring(L_0: *mut lua_State, value: *const libc::c_char);
    #[no_mangle]
    fn tolua_pushusertype(L_0: *mut lua_State, value: *mut libc::c_void,
                          tag: libc::c_int);
    #[no_mangle]
    fn tolua_istype(L_0: *mut lua_State, narg: libc::c_int, tag: libc::c_int,
                    dim: libc::c_int) -> libc::c_int;
    /* File: plots.c */
    /* Purpose: plots & quests */
    /*
 * Copyright (c) 2001 James E. Wilson, Robert A. Koeneke, DarkGod
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
    #[no_mangle]
    static mut L: *mut lua_State;
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
pub type hook_type
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_char) -> bool_>;
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
pub struct hooks_chain {
    pub hook: hook_type,
    pub name: [libc::c_char; 40],
    pub script: [libc::c_char; 40],
    pub type_0: byte_hack,
    pub next: *mut hooks_chain,
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
pub type optimize_flag = libc::c_uint;
pub const NO_OPTIMIZE: optimize_flag = 1;
pub const OPTIMIZE: optimize_flag = 0;
pub type describe_flag = libc::c_uint;
pub const NO_DESCRIBE: describe_flag = 1;
pub const DESCRIBE: describe_flag = 0;
/* #define DEBUG_HOOK */
/* ******* Hooks stuff *********/
#[no_mangle]
pub static mut hook_file: *mut FILE = 0 as *const FILE as *mut FILE;
static mut hooks_heads: [*mut hooks_chain; 78] =
    [0 as *const hooks_chain as *mut hooks_chain; 78];
/* Wipe hooks and init them with quest hooks */
#[no_mangle]
pub unsafe extern "C" fn wipe_hooks() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 78 as libc::c_int {
        hooks_heads[i as usize] = 0 as *mut hooks_chain;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn init_hooks() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 26 as libc::c_int {
        if (*quest.offset(i as isize)).type_0 as libc::c_int ==
               0 as libc::c_int && (*quest.offset(i as isize)).init.is_some()
           {
            (*quest.offset(i as
                               isize)).init.expect("non-null function pointer")(i);
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn dump_hooks(mut h_idx: libc::c_int) {
    let mut min: libc::c_int = 0 as libc::c_int;
    let mut max: libc::c_int = 78 as libc::c_int;
    let mut i: libc::c_int = 0;
    if h_idx != -(1 as libc::c_int) {
        min = h_idx;
        max = h_idx + 1 as libc::c_int
    }
    i = min;
    while i < max {
        let mut c: *mut hooks_chain = hooks_heads[i as usize];
        /* Find it */
        while !c.is_null() {
            msg_format(b"%s(%s)\x00" as *const u8 as *const libc::c_char,
                       (*c).name.as_mut_ptr(),
                       if (*c).type_0 as libc::c_int == 0 as libc::c_int {
                           b"C\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"Lua\x00" as *const u8 as *const libc::c_char
                       });
            c = (*c).next
        }
        i += 1
    };
}
/* Check a hook */
#[no_mangle]
pub unsafe extern "C" fn check_hook(mut h_idx: libc::c_int) -> bool_ {
    let mut c: *mut hooks_chain = hooks_heads[h_idx as usize];
    return (c != 0 as *mut libc::c_void as *mut hooks_chain) as libc::c_int as
               bool_;
}
/* Add a hook */
#[no_mangle]
pub unsafe extern "C" fn add_hook(mut h_idx: libc::c_int, mut hook: hook_type,
                                  mut name: cptr) -> *mut hooks_chain {
    let mut new_: *mut hooks_chain = 0 as *mut hooks_chain;
    let mut c: *mut hooks_chain = hooks_heads[h_idx as usize];
    /* Find it */
    while !c.is_null() && strcmp((*c).name.as_mut_ptr(), name) != 0 {
        c = (*c).next
    }
    /* If not already in the list, add it */
    if c.is_null() {
        new_ =
            memset(ralloc(::std::mem::size_of::<hooks_chain>() as
                              libc::c_ulong) as *mut libc::c_char as
                       *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<hooks_chain>() as libc::c_ulong) as
                *mut hooks_chain;
        (*new_).hook = hook;
        sprintf((*new_).name.as_mut_ptr(),
                b"%s\x00" as *const u8 as *const libc::c_char, name);
        (*new_).next = hooks_heads[h_idx as usize];
        hooks_heads[h_idx as usize] = new_;
        return new_
    } else { return c };
}
#[no_mangle]
pub unsafe extern "C" fn add_hook_script(mut h_idx: libc::c_int,
                                         mut script: *mut libc::c_char,
                                         mut name: cptr) {
    let mut c: *mut hooks_chain = add_hook(h_idx, None, name);
    sprintf((*c).script.as_mut_ptr(),
            b"%s\x00" as *const u8 as *const libc::c_char, script);
    (*c).type_0 = 1 as libc::c_int as byte_hack;
}
/* Remove a hook */
#[no_mangle]
pub unsafe extern "C" fn del_hook(mut h_idx: libc::c_int,
                                  mut hook: hook_type) {
    let mut c: *mut hooks_chain = hooks_heads[h_idx as usize];
    let mut p: *mut hooks_chain = 0 as *mut hooks_chain;
    /* Find it */
    while !c.is_null() && (*c).hook != hook { p = c; c = (*c).next }
    /* Remove it */
    if !c.is_null() {
        if p.is_null() {
            hooks_heads[h_idx as usize] = (*c).next;
            rnfree(c as vptr,
                   ::std::mem::size_of::<hooks_chain>() as libc::c_ulong);
        } else {
            (*p).next = (*c).next;
            rnfree(c as vptr,
                   ::std::mem::size_of::<hooks_chain>() as libc::c_ulong);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn del_hook_name(mut h_idx: libc::c_int,
                                       mut name: cptr) {
    let mut c: *mut hooks_chain = hooks_heads[h_idx as usize];
    let mut p: *mut hooks_chain = 0 as *mut hooks_chain;
    /* Find it */
    while !c.is_null() && strcmp((*c).name.as_mut_ptr(), name) != 0 {
        p = c;
        c = (*c).next
    }
    /* Remove it */
    if !c.is_null() {
        if p.is_null() {
            hooks_heads[h_idx as usize] = (*c).next;
            rnfree(c as vptr,
                   ::std::mem::size_of::<hooks_chain>() as libc::c_ulong);
        } else {
            (*p).next = (*c).next;
            rnfree(c as vptr,
                   ::std::mem::size_of::<hooks_chain>() as libc::c_ulong);
        }
    };
}
/* get the next argument */
static mut param_pile: [hook_return; 50] = [hook_return{num: 0,}; 50];
static mut get_next_arg_pos: libc::c_int = 0 as libc::c_int;
static mut get_next_arg_pile_pos: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn get_next_arg(mut fmt: *mut libc::c_char) -> s32b {
    loop  {
        let fresh0 = get_next_arg_pos;
        get_next_arg_pos = get_next_arg_pos + 1;
        match *fmt.offset(fresh0 as isize) as libc::c_int {
            100 | 108 => {
                let fresh1 = get_next_arg_pile_pos;
                get_next_arg_pile_pos = get_next_arg_pile_pos + 1;
                return param_pile[fresh1 as usize].num
            }
            41 => { get_next_arg_pos -= 1; return 0 as libc::c_int }
            40 | 44 | _ => { }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_next_arg_str(mut fmt: *mut libc::c_char)
 -> *mut libc::c_char {
    loop  {
        let fresh2 = get_next_arg_pos;
        get_next_arg_pos = get_next_arg_pos + 1;
        match *fmt.offset(fresh2 as isize) as libc::c_int {
            115 => {
                let fresh3 = get_next_arg_pile_pos;
                get_next_arg_pile_pos = get_next_arg_pile_pos + 1;
                return param_pile[fresh3 as usize].str_0 as *mut libc::c_char
            }
            41 => { get_next_arg_pos -= 1; return 0 as *mut libc::c_char }
            40 | 44 | _ => { }
        }
    };
}
/* Actually process the hooks */
#[no_mangle]
pub static mut process_hooks_restart: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut process_hooks_return: [hook_return; 20] =
    [hook_return{num: 0,}; 20];
unsafe extern "C" fn vprocess_hooks_return(mut h_idx: libc::c_int,
                                           mut ret: *mut libc::c_char,
                                           mut fmt: *mut libc::c_char,
                                           mut ap: *mut ::std::ffi::VaList)
 -> bool_ {
    let mut c: *mut hooks_chain = hooks_heads[h_idx as usize];
    let mut real_ap: ::std::ffi::VaListImpl;
    while !c.is_null() {
        if (*c).type_0 as libc::c_int == 0 as libc::c_int {
            let mut i: libc::c_int = 0 as libc::c_int;
            let mut nb: libc::c_int = 0 as libc::c_int;
            /* Push all args in the pile */
            i = 0 as libc::c_int;
            memcpy(&mut real_ap.as_va_list() as *mut ::std::ffi::VaList as
                       *mut libc::c_char as *mut libc::c_void,
                   ap as *mut libc::c_char as *const libc::c_void,
                   ::std::mem::size_of::<::std::ffi::VaList>() as
                       libc::c_ulong);
            while *fmt.offset(i as isize) != 0 {
                match *fmt.offset(i as isize) as libc::c_int {
                    79 => {
                        let fresh4 = nb;
                        nb = nb + 1;
                        param_pile[fresh4 as usize].o_ptr =
                            real_ap.arg::<*mut object_type>()
                    }
                    115 => {
                        let fresh5 = nb;
                        nb = nb + 1;
                        param_pile[fresh5 as usize].str_0 =
                            real_ap.arg::<*mut libc::c_char>() as cptr
                    }
                    100 | 108 => {
                        let fresh6 = nb;
                        nb = nb + 1;
                        param_pile[fresh6 as usize].num =
                            real_ap.arg::<s32b>()
                    }
                    40 | 41 | 44 | _ => { }
                }
                i += 1
            }
            get_next_arg_pos = 0 as libc::c_int;
            get_next_arg_pile_pos = 0 as libc::c_int;
            if (*c).hook.expect("non-null function pointer")(fmt) != 0 {
                return 1 as libc::c_int as bool_
            }
            /* Should we restart ? */
            if process_hooks_restart != 0 {
                c = hooks_heads[h_idx as usize];
                process_hooks_restart = 0 as libc::c_int
            } else { c = (*c).next }
        } else if (*c).type_0 as libc::c_int == 1 as libc::c_int {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            let mut nb_0: libc::c_int = 0 as libc::c_int;
            let mut nbr: libc::c_int = 1 as libc::c_int;
            let mut oldtop: libc::c_int = lua_gettop(L);
            let mut size: libc::c_int = 0;
            /* Push the function */
            lua_getglobal(L, (*c).script.as_mut_ptr());
            /* Push and count the arguments */
            memcpy(&mut real_ap.as_va_list() as *mut ::std::ffi::VaList as
                       *mut libc::c_char as *mut libc::c_void,
                   ap as *mut libc::c_char as *const libc::c_void,
                   ::std::mem::size_of::<::std::ffi::VaList>() as
                       libc::c_ulong);
            while *fmt.offset(i_0 as isize) != 0 {
                let fresh7 = i_0;
                i_0 = i_0 + 1;
                match *fmt.offset(fresh7 as isize) as libc::c_int {
                    100 | 108 => {
                        tolua_pushnumber(L,
                                         real_ap.arg::<s32b>() as
                                             libc::c_long);
                        nb_0 += 1
                    }
                    115 => {
                        tolua_pushstring(L,
                                         real_ap.arg::<*mut libc::c_char>());
                        nb_0 += 1
                    }
                    79 => {
                        tolua_pushusertype(L,
                                           real_ap.arg::<*mut object_type>()
                                               as *mut libc::c_void,
                                           tolua_tag(L,
                                                     b"object_type\x00" as
                                                         *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_char));
                        nb_0 += 1
                    }
                    77 => {
                        tolua_pushusertype(L,
                                           real_ap.arg::<*mut monster_type>()
                                               as *mut libc::c_void,
                                           tolua_tag(L,
                                                     b"monster_type\x00" as
                                                         *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_char));
                        nb_0 += 1
                    }
                    40 | 41 | 44 | _ => { }
                }
            }
            /* Count returns */
            nbr =
                (nbr as libc::c_ulong).wrapping_add(strlen(ret)) as
                    libc::c_int as libc::c_int;
            /* Call the function */
            if lua_call(L, nb_0, nbr) != 0 {
                cmsg_format(10 as libc::c_int as byte_hack,
                            b"ERROR in lua_call while calling \'%s\' lua hook script. Breaking the hook chain now.\x00"
                                as *const u8 as *const libc::c_char,
                            (*c).script.as_mut_ptr());
                return 0 as libc::c_int as bool_
            }
            /* Number of returned values, SHOULD be the same as nbr, but I'm paranoid */
            size = lua_gettop(L) - oldtop;
            /* get the extra returns if needed */
            i_0 = 0 as libc::c_int;
            while i_0 < nbr - 1 as libc::c_int {
                if *ret.offset(i_0 as isize) as libc::c_int == 'd' as i32 ||
                       *ret.offset(i_0 as isize) as libc::c_int == 'l' as i32
                   {
                    if lua_isnumber(L, -size + 1 as libc::c_int + i_0) != 0 {
                        process_hooks_return[i_0 as usize].num =
                            tolua_getnumber(L, -size + 1 as libc::c_int + i_0,
                                            0 as libc::c_int as libc::c_long)
                                as s32b
                    } else {
                        process_hooks_return[i_0 as usize].num =
                            0 as libc::c_int
                    }
                } else if *ret.offset(i_0 as isize) as libc::c_int ==
                              's' as i32 {
                    if lua_isstring(L, -size + 1 as libc::c_int + i_0) != 0 {
                        process_hooks_return[i_0 as usize].str_0 =
                            tolua_getstring(L, -size + 1 as libc::c_int + i_0,
                                            b"\x00" as *const u8 as
                                                *const libc::c_char)
                    } else {
                        process_hooks_return[i_0 as usize].str_0 = 0 as cptr
                    }
                } else if *ret.offset(i_0 as isize) as libc::c_int ==
                              'O' as i32 {
                    if tolua_istype(L, -size + 1 as libc::c_int + i_0,
                                    tolua_tag(L,
                                              b"object_type\x00" as *const u8
                                                  as *const libc::c_char as
                                                  *mut libc::c_char),
                                    0 as libc::c_int) != 0 {
                        process_hooks_return[i_0 as usize].o_ptr =
                            tolua_getuserdata(L,
                                              -size + 1 as libc::c_int + i_0,
                                              0 as *mut libc::c_void) as
                                *mut object_type
                    } else {
                        process_hooks_return[i_0 as usize].o_ptr =
                            0 as *mut object_type
                    }
                } else if *ret.offset(i_0 as isize) as libc::c_int ==
                              'M' as i32 {
                    if tolua_istype(L, -size + 1 as libc::c_int + i_0,
                                    tolua_tag(L,
                                              b"monster_type\x00" as *const u8
                                                  as *const libc::c_char as
                                                  *mut libc::c_char),
                                    0 as libc::c_int) != 0 {
                        process_hooks_return[i_0 as usize].m_ptr =
                            tolua_getuserdata(L,
                                              -size + 1 as libc::c_int + i_0,
                                              0 as *mut libc::c_void) as
                                *mut monster_type
                    } else {
                        process_hooks_return[i_0 as usize].m_ptr =
                            0 as *mut monster_type
                    }
                } else {
                    process_hooks_return[i_0 as usize].num = 0 as libc::c_int
                }
                i_0 += 1
            }
            /* Get the basic return(continue or stop the hook chain) */
            if tolua_getnumber(L, -size, 0 as libc::c_int as libc::c_long) !=
                   0 {
                lua_settop(L, oldtop);
                return 1 as libc::c_int as bool_
            }
            if process_hooks_restart != 0 {
                c = hooks_heads[h_idx as usize];
                process_hooks_restart = 0 as libc::c_int
            } else { c = (*c).next }
            lua_settop(L, oldtop);
        } else {
            msg_format(b"Unkown hook type %d, name %s\x00" as *const u8 as
                           *const libc::c_char, (*c).type_0 as libc::c_int,
                       (*c).name.as_mut_ptr());
        }
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn process_hooks_ret(mut h_idx: libc::c_int,
                                           mut ret: *mut libc::c_char,
                                           mut fmt: *mut libc::c_char,
                                           mut args: ...) -> bool_ {
    let mut ap: ::std::ffi::VaListImpl;
    let mut r: bool_ = 0;
    ap = args.clone();
    r = vprocess_hooks_return(h_idx, ret, fmt, &mut ap.as_va_list());
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn process_hooks(mut h_idx: libc::c_int,
                                       mut fmt: *mut libc::c_char,
                                       mut args: ...) -> bool_ {
    let mut ap: ::std::ffi::VaListImpl;
    let mut ret: bool_ = 0;
    ap = args.clone();
    ret =
        vprocess_hooks_return(h_idx,
                              b"\x00" as *const u8 as *const libc::c_char as
                                  *mut libc::c_char, fmt,
                              &mut ap.as_va_list());
    return ret;
}
/* ******* Plots & Quest stuff ********/
unsafe extern "C" fn quest_describe(mut q_idx: libc::c_int) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 10 as libc::c_int &&
              (*quest.offset(q_idx as
                                 isize)).desc[i as
                                                  usize][0 as libc::c_int as
                                                             usize] as
                  libc::c_int != '\u{0}' as i32 {
        let fresh8 = i;
        i = i + 1;
        cmsg_print(11 as libc::c_int as byte_hack,
                   (*quest.offset(q_idx as
                                      isize)).desc[fresh8 as
                                                       usize].as_mut_ptr() as
                       cptr);
    };
}
/* Catch-all quest hook */
#[no_mangle]
pub unsafe extern "C" fn quest_null_hook(mut q: libc::c_int) -> bool_ {
    /* Do nothing */
    return 0 as libc::c_int as bool_;
}
static mut randquest_hero: [libc::c_int; 9] =
    [20 as libc::c_int, 13 as libc::c_int, 15 as libc::c_int,
     16 as libc::c_int, 9 as libc::c_int, 17 as libc::c_int,
     18 as libc::c_int, 8 as libc::c_int, -(1 as libc::c_int)];
#[no_mangle]
pub unsafe extern "C" fn is_randhero(mut level: libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut result: bool_ = 0 as libc::c_int as bool_;
    i = 0 as libc::c_int;
    while randquest_hero[i as usize] != -(1 as libc::c_int) {
        if random_quests[level as usize].type_0 as libc::c_int ==
               randquest_hero[i as usize] {
            result = 1 as libc::c_int as bool_;
            break ;
        } else { i += 1 }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn do_get_new_obj(mut y: libc::c_int,
                                        mut x: libc::c_int) {
    let mut theme: obj_theme =
        obj_theme{treasure: 0, combat: 0, magic: 0, tools: 0,};
    let mut items: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
    let mut q_ptr: [*mut object_type; 3] = [0 as *mut object_type; 3];
    let mut forge: [object_type; 3] =
        [object_type{k_idx: 0,
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
                     found_aux4: 0,}; 3];
    let mut max: libc::c_int = 0 as libc::c_int;
    let mut res: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    max = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        q_ptr[max as usize] =
            &mut *forge.as_mut_ptr().offset(max as isize) as *mut object_type;
        object_wipe(q_ptr[max as usize]);
        theme.treasure = 100 as libc::c_int as byte_hack;
        theme.combat = 100 as libc::c_int as byte_hack;
        theme.magic = 100 as libc::c_int as byte_hack;
        theme.tools = 100 as libc::c_int as byte_hack;
        make_object(q_ptr[max as usize], 1 as libc::c_int as bool_,
                    1 as libc::c_int as bool_, theme);
        (*q_ptr[max as usize]).found = 6 as libc::c_int as byte_hack;
        items[max as usize] =
            memset(ralloc((100 as libc::c_int as
                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                               as
                                                               libc::c_ulong))
                       as *mut libc::c_char as *mut libc::c_void,
                   0 as libc::c_int,
                   (100 as libc::c_int as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                        as libc::c_ulong)) as
                *mut libc::c_char;
        object_desc(items[max as usize], q_ptr[max as usize],
                    0 as libc::c_int, 0 as libc::c_int);
        max += 1;
        i += 1
    }
    loop  {
        res =
            ask_menu(b"Choose a reward to get(a-c to choose, ESC to cancel)?\x00"
                         as *const u8 as *const libc::c_char,
                     items.as_mut_ptr(), 3 as libc::c_int);
        if !(res > -(1 as libc::c_int)) { continue ; }
        drop_near(q_ptr[res as usize], -(1 as libc::c_int),
                  y + 1 as libc::c_int, x);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"There, Noble Hero. I put it there. Thanks again!\x00" as
                       *const u8 as *const libc::c_char);
        break ;
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        let mut o_ptr: *mut object_type = q_ptr[i as usize];
        if i != res &&
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
                     } else { 0 as libc::c_int }) != 0) {
            if (*o_ptr).tval as libc::c_int == 102 as libc::c_int {
                random_artifacts[(*o_ptr).sval as usize].generated =
                    0 as libc::c_int as byte_hack
            } else if (*k_info.offset((*o_ptr).k_idx as isize)).flags3 as
                          libc::c_long & 0x8000 as libc::c_long != 0 {
                (*k_info.offset((*o_ptr).k_idx as isize)).artifact =
                    0 as libc::c_int as bool_
            } else if (*o_ptr).name1 != 0 {
                (*a_info.offset((*o_ptr).name1 as isize)).cur_num =
                    0 as libc::c_int as byte_hack
            }
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        items[i as usize] =
            rnfree(items[i as usize] as vptr,
                   (100 as libc::c_int as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                        as libc::c_ulong)) as
                *mut libc::c_char;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn princess_death(mut m_idx: s32b, mut r_idx: s32b) {
    let mut r: libc::c_int = 0;
    cmsg_print(11 as libc::c_int as byte_hack,
               b"O Great And Noble Hero, you saved me!\x00" as *const u8 as
                   *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"I am heading home now. I cannot reward you as I should, but please take this.\x00"
                   as *const u8 as *const libc::c_char);
    r = m_max as libc::c_int - 1 as libc::c_int;
    while r >= 1 as libc::c_int {
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(r as isize) as *mut monster_type;
        if !((*m_ptr).r_idx == 0) {
            if (*m_ptr).r_idx as libc::c_int == 969 as libc::c_int {
                let mut x: libc::c_int = (*m_ptr).fx as libc::c_int;
                let mut y: libc::c_int = (*m_ptr).fy as libc::c_int;
                let mut i: libc::c_int = 0;
                let mut j: libc::c_int = 0;
                delete_monster_idx(r);
                i = x - 1 as libc::c_int;
                while i <= x + 1 as libc::c_int {
                    j = y - 1 as libc::c_int;
                    while j <= y + 1 as libc::c_int {
                        if j > 0 as libc::c_int && i > 0 as libc::c_int &&
                               j < cur_hgt as libc::c_int - 1 as libc::c_int
                               &&
                               i < cur_wid as libc::c_int - 1 as libc::c_int {
                            cave_set_feat(j, i, 0x1 as libc::c_int);
                        }
                        j += 1
                    }
                    i += 1
                }
                cave_set_feat(y, x, 0x7 as libc::c_int);
                do_get_new_obj(y, x);
                random_quests[dun_level as usize].done =
                    1 as libc::c_int as bool_;
                break ;
            }
        }
        r -= 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn hero_death(mut m_idx: s32b, mut r_idx: s32b) {
    random_quests[dun_level as usize].done = 1 as libc::c_int as bool_;
    cmsg_print(11 as libc::c_int as byte_hack,
               b"The adventurer steps out of the shadows and picks up his sword:\x00"
                   as *const u8 as *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"\'Ah! My sword! My trusty sword! Thanks.\x00" as *const u8 as
                   *const libc::c_char);
    if can_create_companion() == 0 {
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"I must go on my own way now.\x00" as *const u8 as
                       *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"But before I go, I can help your skills.\'\x00" as
                       *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"He touches your forehead.\x00" as *const u8 as
                       *const libc::c_char);
        do_get_new_skill();
        return
    }
    cmsg_print(11 as libc::c_int as byte_hack,
               b"If you wish, I can help you in your adventures.\'\x00" as
                   *const u8 as *const libc::c_char);
    flush();
    if get_check(b"Do you want him to join you? \x00" as *const u8 as
                     *const libc::c_char) != 0 {
        let mut x: libc::c_int = 0;
        let mut y: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 20 as libc::c_int {
            let mut d: libc::c_int = i / 15 as libc::c_int + 1 as libc::c_int;
            scatter(&mut y, &mut x, (*p_ptr).py as libc::c_int,
                    (*p_ptr).px as libc::c_int, d);
            if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                   isize)).flags1 as libc::c_long &
                   0x10 as libc::c_long != 0 &&
                   (*cave[y as usize].offset(x as isize)).feat as libc::c_int
                       != 0xaf as libc::c_int &&
                   (*cave[y as usize].offset(x as isize)).m_idx == 0 &&
                   !(y == (*p_ptr).py as libc::c_int &&
                         x == (*p_ptr).px as libc::c_int) {
                if !((*cave[y as usize].offset(x as isize)).feat as
                         libc::c_int == 0x3 as libc::c_int) {
                    if !((*cave[y as usize].offset(x as isize)).feat as
                             libc::c_int == 0x40 as libc::c_int) {
                        if !((*cave[y as usize].offset(x as isize)).feat as
                                 libc::c_int == 0xa0 as libc::c_int) {
                            if !((*cave[y as usize].offset(x as isize)).feat
                                     as libc::c_int >= 0x41 as libc::c_int &&
                                     (*cave[y as
                                                usize].offset(x as
                                                                  isize)).feat
                                         as libc::c_int <=
                                         0x49 as libc::c_int) {
                                break ;
                            }
                        }
                    }
                }
            }
            i += 1
        }
        if i < 20 as libc::c_int {
            let mut m_idx_0: libc::c_int = 0;
            *m_allow_special.offset(test_monster_name(b"Adventurer\x00" as
                                                          *const u8 as
                                                          *const libc::c_char)
                                        as isize) = 1 as libc::c_int as bool_;
            m_idx_0 =
                place_monster_one(y, x,
                                  test_monster_name(b"Adventurer\x00" as
                                                        *const u8 as
                                                        *const libc::c_char),
                                  0 as libc::c_int, 0 as libc::c_int as bool_,
                                  4 as libc::c_int) as libc::c_int;
            *m_allow_special.offset(test_monster_name(b"Adventurer\x00" as
                                                          *const u8 as
                                                          *const libc::c_char)
                                        as isize) = 0 as libc::c_int as bool_;
            if m_idx_0 != 0 {
                (*m_list.offset(m_idx_0 as isize)).exp =
                    ((if 1 as libc::c_int +
                             dun_level as libc::c_int * 3 as libc::c_int /
                                 2 as libc::c_int > 150 as libc::c_int {
                          150 as libc::c_int
                      } else {
                          (1 as libc::c_int) +
                              dun_level as libc::c_int * 3 as libc::c_int /
                                  2 as libc::c_int
                      }) *
                         (if 1 as libc::c_int +
                                 dun_level as libc::c_int * 3 as libc::c_int /
                                     2 as libc::c_int > 150 as libc::c_int {
                              150 as libc::c_int
                          } else {
                              (1 as libc::c_int) +
                                  dun_level as libc::c_int * 3 as libc::c_int
                                      / 2 as libc::c_int
                          }) *
                         (if 1 as libc::c_int +
                                 dun_level as libc::c_int * 3 as libc::c_int /
                                     2 as libc::c_int > 150 as libc::c_int {
                              150 as libc::c_int
                          } else {
                              (1 as libc::c_int) +
                                  dun_level as libc::c_int * 3 as libc::c_int
                                      / 2 as libc::c_int
                          }) * 6 as libc::c_int) as u32b;
                (*m_list.offset(m_idx_0 as isize)).status =
                    4 as libc::c_int as s16b;
                monster_check_experience(m_idx_0, 1 as libc::c_int as bool_);
            }
        } else {
            msg_print(b"The adventurer suddenly seems afraid and flees...\x00"
                          as *const u8 as *const libc::c_char);
        }
    } else {
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'As you wish, but I want to do something for you.\'\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"He touches your forehead.\x00" as *const u8 as
                       *const libc::c_char);
        do_get_new_skill();
    };
}
#[no_mangle]
pub unsafe extern "C" fn quest_random_death_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut r_idx: libc::c_int = 0;
    let mut m_idx: s32b = 0;
    m_idx = get_next_arg(fmt);
    r_idx = (*m_list.offset(m_idx as isize)).r_idx as libc::c_int;
    if dungeon_flags1 as libc::c_long & 0x1 as libc::c_long == 0 {
        return 0 as libc::c_int as bool_
    }
    if (dun_level as libc::c_int) < 1 as libc::c_int ||
           dun_level as libc::c_int >= 99 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if random_quests[dun_level as usize].type_0 == 0 {
        return 0 as libc::c_int as bool_
    }
    if random_quests[dun_level as usize].done != 0 {
        return 0 as libc::c_int as bool_
    }
    if (*p_ptr).inside_quest != 0 { return 0 as libc::c_int as bool_ }
    if random_quests[dun_level as usize].r_idx as libc::c_int != r_idx {
        return 0 as libc::c_int as bool_
    }
    if (*m_list.offset(m_idx as isize)).mflag & 0x2 as libc::c_int == 0 {
        return 0 as libc::c_int as bool_
    }
    let ref mut fresh9 =
        (*quest.offset(5 as libc::c_int as
                           isize)).data[0 as libc::c_int as usize];
    *fresh9 += 1;
    if (*quest.offset(5 as libc::c_int as
                          isize)).data[0 as libc::c_int as usize] ==
           random_quests[dun_level as usize].type_0 as libc::c_int {
        if is_randhero(dun_level as libc::c_int) != 0 {
            hero_death(m_idx, r_idx);
        } else { princess_death(m_idx, r_idx); }
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_random_turn_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    (*quest.offset(5 as libc::c_int as isize)).data[0 as libc::c_int as usize]
        = 0 as libc::c_int;
    (*quest.offset(5 as libc::c_int as isize)).data[1 as libc::c_int as usize]
        = 0 as libc::c_int;
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_random_feeling_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    if dungeon_flags1 as libc::c_long & 0x1 as libc::c_long == 0 {
        return 0 as libc::c_int as bool_
    }
    if (dun_level as libc::c_int) < 1 as libc::c_int ||
           dun_level as libc::c_int >= 99 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if random_quests[dun_level as usize].type_0 == 0 {
        return 0 as libc::c_int as bool_
    }
    if random_quests[dun_level as usize].done != 0 {
        return 0 as libc::c_int as bool_
    }
    if (*p_ptr).inside_quest != 0 { return 0 as libc::c_int as bool_ }
    if dun_level == 0 { return 0 as libc::c_int as bool_ }
    if is_randhero(dun_level as libc::c_int) != 0 {
        cmsg_format(11 as libc::c_int as byte_hack,
                    b"A strange man wrapped in a dark cloak steps out of the shadows:\x00"
                        as *const u8 as *const libc::c_char);
        cmsg_format(11 as libc::c_int as byte_hack,
                    b"\'Oh, please help me! A horrible %s stole my sword! I\'m nothing without it.\'\x00"
                        as *const u8 as *const libc::c_char,
                    r_name.offset((*r_info.offset(random_quests[dun_level as
                                                                    usize].r_idx
                                                      as isize)).name as
                                      isize));
    } else {
        cmsg_format(11 as libc::c_int as byte_hack,
                    b"You hear someone shouting: \'Leave me alone, stupid %s\'\x00"
                        as *const u8 as *const libc::c_char,
                    r_name.offset((*r_info.offset(random_quests[dun_level as
                                                                    usize].r_idx
                                                      as isize)).name as
                                      isize));
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_random_gen_hero_hook(mut fmt:
                                                        *mut libc::c_char)
 -> bool_ {
    let mut i: libc::c_int = 0;
    if dungeon_flags1 as libc::c_long & 0x1 as libc::c_long == 0 {
        return 0 as libc::c_int as bool_
    }
    if (dun_level as libc::c_int) < 1 as libc::c_int ||
           dun_level as libc::c_int >= 99 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if random_quests[dun_level as usize].type_0 == 0 {
        return 0 as libc::c_int as bool_
    }
    if random_quests[dun_level as usize].done != 0 {
        return 0 as libc::c_int as bool_
    }
    if (*p_ptr).inside_quest != 0 { return 0 as libc::c_int as bool_ }
    if is_randhero(dun_level as libc::c_int) == 0 {
        return 0 as libc::c_int as bool_
    }
    i = random_quests[dun_level as usize].type_0 as libc::c_int;
    *m_allow_special.offset(random_quests[dun_level as usize].r_idx as isize)
        = 1 as libc::c_int as bool_;
    while i != 0 {
        let mut m_idx: libc::c_int = 0;
        let mut y: libc::c_int =
            1 as libc::c_int +
                Rand_div(1 as libc::c_int +
                             (cur_hgt as libc::c_int - 2 as libc::c_int) -
                             1 as libc::c_int);
        let mut x: libc::c_int =
            1 as libc::c_int +
                Rand_div(1 as libc::c_int +
                             (cur_wid as libc::c_int - 2 as libc::c_int) -
                             1 as libc::c_int);
        m_idx =
            place_monster_one(y, x,
                              random_quests[dun_level as usize].r_idx as
                                  libc::c_int, 0 as libc::c_int,
                              0 as libc::c_int as bool_, -(2 as libc::c_int))
                as libc::c_int;
        if m_idx != 0 {
            let mut m_ptr: *mut monster_type =
                &mut *m_list.offset(m_idx as isize) as *mut monster_type;
            (*m_ptr).mflag |= 0x2 as libc::c_int;
            i -= 1
        }
    }
    *m_allow_special.offset(random_quests[dun_level as usize].r_idx as isize)
        = 0 as libc::c_int as bool_;
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_random_gen_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut x: s32b = 0;
    let mut y: s32b = 0;
    let mut bx0: s32b = 0;
    let mut by0: s32b = 0;
    let mut xstart: libc::c_int = 0;
    let mut ystart: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut yval: libc::c_int = 0;
    let mut xval: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut xsize: libc::c_int = 0;
    let mut ysize: libc::c_int = 0;
    if dungeon_flags1 as libc::c_long & 0x1 as libc::c_long == 0 {
        return 0 as libc::c_int as bool_
    }
    if (dun_level as libc::c_int) < 1 as libc::c_int ||
           dun_level as libc::c_int >= 99 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if random_quests[dun_level as usize].type_0 == 0 {
        return 0 as libc::c_int as bool_
    }
    if random_quests[dun_level as usize].done != 0 {
        return 0 as libc::c_int as bool_
    }
    if (*p_ptr).inside_quest != 0 { return 0 as libc::c_int as bool_ }
    if (*quest.offset(5 as libc::c_int as
                          isize)).data[1 as libc::c_int as usize] != 0 {
        return 0 as libc::c_int as bool_
    }
    if is_randhero(dun_level as libc::c_int) != 0 {
        return 0 as libc::c_int as bool_
    }
    by0 = get_next_arg(fmt);
    bx0 = get_next_arg(fmt);
    get_map_size(format(b"qrand%d.map\x00" as *const u8 as
                            *const libc::c_char,
                        random_quests[dun_level as usize].type_0 as
                            libc::c_int), &mut ysize, &mut xsize);
    if room_alloc(xsize + 2 as libc::c_int, ysize + 2 as libc::c_int,
                  0 as libc::c_int as bool_, by0, bx0, &mut xval, &mut yval)
           == 0 {
        return 0 as libc::c_int as bool_
    }
    y1 = yval - ysize / 2 as libc::c_int;
    x1 = xval - xsize / 2 as libc::c_int;
    y2 = y1 + ysize - 1 as libc::c_int;
    x2 = x1 + xsize - 1 as libc::c_int;
    y = y1;
    while y <= y2 {
        x = x1;
        while x <= x2 {
            cave_set_feat(y, x,
                          floor_type[Rand_div(100 as libc::c_int) as usize] as
                              libc::c_int);
            let ref mut fresh10 = (*cave[y as usize].offset(x as isize)).info;
            *fresh10 =
                (*fresh10 as libc::c_int |
                     (0x8 as libc::c_int | 0x2 as libc::c_int)) as u16b;
            x += 1
        }
        y += 1
    }
    build_rectangle(y1 - 1 as libc::c_int, x1 - 1 as libc::c_int,
                    y2 + 1 as libc::c_int, x2 + 1 as libc::c_int,
                    feat_wall_outer as libc::c_int,
                    0x8 as libc::c_int | 0x2 as libc::c_int);
    set_mon_num_hook();
    get_mon_num_prep();
    xstart = x1;
    ystart = y1;
    init_flags = 0x4 as libc::c_int;
    process_dungeon_file(format(b"qrand%d.map\x00" as *const u8 as
                                    *const libc::c_char,
                                random_quests[dun_level as usize].type_0 as
                                    libc::c_int) as cptr, &mut ystart,
                         &mut xstart, cur_hgt as libc::c_int,
                         cur_wid as libc::c_int, 1 as libc::c_int as bool_,
                         1 as libc::c_int as bool_);
    x = x1;
    while x < xstart {
        y = y1;
        while y < ystart {
            let ref mut fresh11 = (*cave[y as usize].offset(x as isize)).info;
            *fresh11 =
                (*fresh11 as libc::c_int |
                     (0x4 as libc::c_int | 0x8 as libc::c_int)) as u16b;
            if (*cave[y as usize].offset(x as isize)).feat as libc::c_int ==
                   0xac as libc::c_int {
                let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
                let mut i: libc::c_int = 0;
                *m_allow_special.offset(random_quests[dun_level as
                                                          usize].r_idx as
                                            isize) =
                    1 as libc::c_int as bool_;
                i =
                    place_monster_one(y, x,
                                      random_quests[dun_level as usize].r_idx
                                          as libc::c_int, 0 as libc::c_int,
                                      0 as libc::c_int as bool_,
                                      -(2 as libc::c_int)) as libc::c_int;
                *m_allow_special.offset(random_quests[dun_level as
                                                          usize].r_idx as
                                            isize) =
                    0 as libc::c_int as bool_;
                if i != 0 {
                    m_ptr =
                        &mut *m_list.offset(i as isize) as *mut monster_type;
                    (*m_ptr).mflag |= 0x2 as libc::c_int
                }
            }
            y += 1
        }
        x += 1
    }
    (*quest.offset(5 as libc::c_int as isize)).data[1 as libc::c_int as usize]
        = 1 as libc::c_int;
    rating = (rating as libc::c_int + 10 as libc::c_int) as s16b;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_random_dump_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    static mut number: [*mut libc::c_char; 9] =
        [b"two\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
         b"three\x00" as *const u8 as *const libc::c_char as
             *mut libc::c_char,
         b"four\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
         b"five\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
         b"six\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
         b"seven\x00" as *const u8 as *const libc::c_char as
             *mut libc::c_char,
         b"eight\x00" as *const u8 as *const libc::c_char as
             *mut libc::c_char,
         b"nine\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
         b"ten\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
    let mut i: libc::c_int = 0;
    let mut valid: libc::c_int = 0 as libc::c_int;
    let mut lscnt: libc::c_int = 0 as libc::c_int;
    let mut pcnt: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 99 as libc::c_int {
        if random_quests[i as usize].type_0 != 0 {
            valid += 1;
            if random_quests[i as usize].done != 0 {
                if is_randhero(i) != 0 { lscnt += 1 } else { pcnt += 1 }
            }
        }
        i += 1
    }
    if valid != 0 {
        if pcnt > 10 as libc::c_int {
            fprintf(hook_file,
                    b"\n You have completed %d princess quests.\x00" as
                        *const u8 as *const libc::c_char, pcnt);
        } else if pcnt > 1 as libc::c_int {
            fprintf(hook_file,
                    b"\n You have completed %s princess quests.\x00" as
                        *const u8 as *const libc::c_char,
                    number[(pcnt - 2 as libc::c_int) as usize]);
        } else if pcnt == 1 as libc::c_int {
            fprintf(hook_file,
                    b"\n You have completed one princess quest.\x00" as
                        *const u8 as *const libc::c_char);
        } else {
            fprintf(hook_file,
                    b"\n You haven\'t completed a single princess quest.\x00"
                        as *const u8 as *const libc::c_char);
        }
        if lscnt > 10 as libc::c_int {
            fprintf(hook_file,
                    b"\n You have completed %d lost sword quests.\x00" as
                        *const u8 as *const libc::c_char, lscnt);
        } else if lscnt > 1 as libc::c_int {
            fprintf(hook_file,
                    b"\n You have completed %s lost sword quests.\x00" as
                        *const u8 as *const libc::c_char,
                    number[(lscnt - 2 as libc::c_int) as usize]);
        } else if lscnt == 1 as libc::c_int {
            fprintf(hook_file,
                    b"\n You have completed one lost sword quest.\x00" as
                        *const u8 as *const libc::c_char);
        } else {
            fprintf(hook_file,
                    b"\n You haven\'t completed a single lost sword quest.\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_random_describe(mut fff: *mut FILE) -> bool_ {
    if dungeon_flags1 as libc::c_long & 0x1 as libc::c_long == 0 {
        return 0 as libc::c_int as bool_
    }
    if (dun_level as libc::c_int) < 1 as libc::c_int ||
           dun_level as libc::c_int >= 99 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if random_quests[dun_level as usize].type_0 == 0 {
        return 0 as libc::c_int as bool_
    }
    if random_quests[dun_level as usize].done != 0 {
        return 0 as libc::c_int as bool_
    }
    if (*p_ptr).inside_quest != 0 { return 0 as libc::c_int as bool_ }
    if dun_level == 0 { return 0 as libc::c_int as bool_ }
    if is_randhero(dun_level as libc::c_int) == 0 {
        fprintf(fff,
                b"#####yCaptured princess!\n\x00" as *const u8 as
                    *const libc::c_char);
        fprintf(fff,
                b"A princess is being held prisoner and tortured here!\n\x00"
                    as *const u8 as *const libc::c_char);
        fprintf(fff,
                b"Save her from the horrible %s.\n\x00" as *const u8 as
                    *const libc::c_char,
                r_name.offset((*r_info.offset(random_quests[dun_level as
                                                                usize].r_idx
                                                  as isize)).name as isize));
    } else {
        fprintf(fff,
                b"#####yLost sword!\n\x00" as *const u8 as
                    *const libc::c_char);
        fprintf(fff,
                b"An adventurer lost his sword to a bunch of %s!\n\x00" as
                    *const u8 as *const libc::c_char,
                r_name.offset((*r_info.offset(random_quests[dun_level as
                                                                usize].r_idx
                                                  as isize)).name as isize));
        fprintf(fff,
                b"Kill them all to get it back.\n\x00" as *const u8 as
                    *const libc::c_char);
    }
    fprintf(fff,
            b"Number: %d, Killed: %ld.\n\x00" as *const u8 as
                *const libc::c_char,
            random_quests[dun_level as usize].type_0 as libc::c_int,
            (*quest.offset(5 as libc::c_int as
                               isize)).data[0 as libc::c_int as usize] as
                libc::c_long);
    fprintf(fff, b"\n\x00" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_random_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    add_hook(0 as libc::c_int,
             Some(quest_random_death_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"rand_death\x00" as *const u8 as *const libc::c_char);
    add_hook(8 as libc::c_int,
             Some(quest_random_turn_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"rand_new_lvl\x00" as *const u8 as *const libc::c_char);
    add_hook(42 as libc::c_int,
             Some(quest_random_turn_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"rand_regen_lvl\x00" as *const u8 as *const libc::c_char);
    add_hook(43 as libc::c_int,
             Some(quest_random_gen_hero_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"rand_gen_hero\x00" as *const u8 as *const libc::c_char);
    add_hook(7 as libc::c_int,
             Some(quest_random_gen_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"rand_gen\x00" as *const u8 as *const libc::c_char);
    add_hook(4 as libc::c_int,
             Some(quest_random_feeling_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"rand_feel\x00" as *const u8 as *const libc::c_char);
    add_hook(12 as libc::c_int,
             Some(quest_random_dump_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"rand_dump\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_main_monsters_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut r_idx: s32b = 0;
    r_idx = get_next_arg(fmt);
    if r_idx == 860 as libc::c_int {
        if (*r_info.offset(819 as libc::c_int as isize)).max_num != 0 {
            return 1 as libc::c_int as bool_
        }
    } else if r_idx == 862 as libc::c_int {
        if (*r_info.offset(860 as libc::c_int as isize)).max_num != 0 {
            return 1 as libc::c_int as bool_
        }
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_morgoth_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(862 as libc::c_int as isize) as *mut monster_race;
    if (*r_ptr).max_num == 0 {
        total_winner = 1 as libc::c_int as u16b;
        has_won = 1 as libc::c_int as u16b;
        (*quest.offset(3 as libc::c_int as isize)).status =
            5 as libc::c_int as s16b;
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x2 as libc::c_long) as u32b;
        if (*quest.offset(17 as libc::c_int as isize)).status as libc::c_int
               == 5 as libc::c_int {
            cmsg_print(13 as libc::c_int as byte_hack,
                       b"*** CONGRATULATIONS ***\x00" as *const u8 as
                           *const libc::c_char);
            cmsg_print(13 as libc::c_int as byte_hack,
                       b"You have banished Morgoth\'s foul spirit from Ea, and as you watch, a cleansing\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(13 as libc::c_int as byte_hack,
                       b"wind roars through the dungeon, dispersing the nether mists around where the\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(13 as libc::c_int as byte_hack,
                       b"body fell. You feel thanks, and a touch of sorrow, from the Valar\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(13 as libc::c_int as byte_hack,
                       b"for your deed. You will be forever heralded, your deed forever legendary.\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(13 as libc::c_int as byte_hack,
                       b"You may retire (commit suicide) when you are ready.\x00"
                           as *const u8 as *const libc::c_char);
        } else {
            cmsg_print(10 as libc::c_int as byte_hack,
                       b"*** CONGRATULATIONS ***\x00" as *const u8 as
                           *const libc::c_char);
            cmsg_print(10 as libc::c_int as byte_hack,
                       b"You have banished Morgoth from Arda, and made Ea a safer place.\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(10 as libc::c_int as byte_hack,
                       b"As you look down at the dispersing mists around Morgoth, a sudden intuition\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(10 as libc::c_int as byte_hack,
                       b"grasps you. Fingering the One Ring, you gather the nether mists around\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(10 as libc::c_int as byte_hack,
                       b"yourself, and inhale deeply their seductive power.\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(10 as libc::c_int as byte_hack,
                       b"You will be forever feared, your orders forever obeyed.\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(10 as libc::c_int as byte_hack,
                       b"You may retire (commit suicide) when you are ready.\x00"
                           as *const u8 as *const libc::c_char);
        }
        del_hook(0 as libc::c_int,
                 Some(quest_morgoth_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        process_hooks_restart = 1 as libc::c_int;
        if (*quest.offset(17 as libc::c_int as isize)).status as libc::c_int
               == 5 as libc::c_int {
            *(*quest.offset(3 as libc::c_int as isize)).plot =
                20 as libc::c_int as s16b
        } else {
            *(*quest.offset(3 as libc::c_int as isize)).plot =
                21 as libc::c_int as s16b
        }
        (*quest.offset(*(*quest.offset(3 as libc::c_int as isize)).plot as
                           isize)).init.expect("non-null function pointer")(*(*quest.offset(3
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                isize)).plot
                                                                                as
                                                                                libc::c_int);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_morgoth_dump_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    if (*quest.offset(3 as libc::c_int as isize)).status as libc::c_int >=
           2 as libc::c_int {
        if (*quest.offset(17 as libc::c_int as isize)).status as libc::c_int
               == 5 as libc::c_int {
            fprintf(hook_file,
                    b"\n You saved Arda and became a famed %s.\x00" as
                        *const u8 as *const libc::c_char, (*sp_ptr).winner);
        } else {
            fprintf(hook_file,
                    b"\n You became a new force of darkness and enslaved all free people.\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_morgoth_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    if (*quest.offset(3 as libc::c_int as isize)).status as libc::c_int >=
           1 as libc::c_int &&
           ((*quest.offset(3 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(0 as libc::c_int,
                 Some(quest_morgoth_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"morgort_death\x00" as *const u8 as *const libc::c_char);
    }
    add_hook(12 as libc::c_int,
             Some(quest_morgoth_dump_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"morgoth_dump\x00" as *const u8 as *const libc::c_char);
    add_hook(5 as libc::c_int,
             Some(quest_main_monsters_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"main_new_monster\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_sauron_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(860 as libc::c_int as isize) as *mut monster_race;
    if (*r_ptr).max_num == 0 {
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"Well done! You are on the way to slaying Morgoth...\x00"
                       as *const u8 as *const libc::c_char);
        (*quest.offset(2 as libc::c_int as isize)).status =
            5 as libc::c_int as s16b;
        (*quest.offset(3 as libc::c_int as isize)).status =
            1 as libc::c_int as s16b;
        quest_describe(3 as libc::c_int);
        del_hook(0 as libc::c_int,
                 Some(quest_sauron_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        add_hook(0 as libc::c_int,
                 Some(quest_morgoth_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"morgort_death\x00" as *const u8 as *const libc::c_char);
        *(*quest.offset(2 as libc::c_int as isize)).plot =
            3 as libc::c_int as s16b;
        quest_morgoth_init_hook(3 as libc::c_int);
        process_hooks_restart = 1 as libc::c_int
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_sauron_resurect_hook(mut fmt:
                                                        *mut libc::c_char)
 -> bool_ {
    let mut m_idx: s32b = get_next_arg(fmt);
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*m_ptr).r_idx as isize) as *mut monster_race;
    if (*r_ptr).flags7 & 0x80 as libc::c_int as libc::c_uint != 0 &&
           (*r_info.offset(860 as libc::c_int as isize)).max_num as
               libc::c_int != 0 {
        msg_format(b"Somehow you feel %s is not totally destroyed...\x00" as
                       *const u8 as *const libc::c_char,
                   if (*r_ptr).flags1 & 0x8 as libc::c_int as libc::c_uint !=
                          0 {
                       b"she\x00" as *const u8 as *const libc::c_char
                   } else { b"he\x00" as *const u8 as *const libc::c_char });
        (*r_ptr).max_num = 1 as libc::c_int as s16b
    } else if (*m_ptr).r_idx as libc::c_int == 860 as libc::c_int &&
                  ((*quest.offset(17 as libc::c_int as isize)).status as
                       libc::c_int) < 5 as libc::c_int {
        msg_print(b"Sauron will not be permanently defeated until the One Ring is either destroyed or used...\x00"
                      as *const u8 as *const libc::c_char);
        (*r_ptr).max_num = 1 as libc::c_int as s16b
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_sauron_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    if (*quest.offset(2 as libc::c_int as isize)).status as libc::c_int >=
           1 as libc::c_int &&
           ((*quest.offset(2 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(0 as libc::c_int,
                 Some(quest_sauron_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"sauron_death\x00" as *const u8 as *const libc::c_char);
    }
    add_hook(5 as libc::c_int,
             Some(quest_main_monsters_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"main_new_monster\x00" as *const u8 as *const libc::c_char);
    add_hook(0 as libc::c_int,
             Some(quest_sauron_resurect_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"sauron_resurect_death\x00" as *const u8 as
                 *const libc::c_char);
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_necro_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(819 as libc::c_int as isize) as *mut monster_race;
    if (*r_ptr).max_num == 0 {
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"You see the spirit of the necromancer rise and flee...\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"It looks like it was indeed Sauron...\x00" as *const u8
                       as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"You should report that to Galadriel as soon as possible.\x00"
                       as *const u8 as *const libc::c_char);
        (*quest.offset(1 as libc::c_int as isize)).status =
            5 as libc::c_int as s16b;
        *(*quest.offset(1 as libc::c_int as isize)).plot =
            17 as libc::c_int as s16b;
        (*quest.offset(*(*quest.offset(1 as libc::c_int as isize)).plot as
                           isize)).init.expect("non-null function pointer")(*(*quest.offset(1
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                isize)).plot
                                                                                as
                                                                                libc::c_int);
        del_hook(0 as libc::c_int,
                 Some(quest_necro_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        process_hooks_restart = 1 as libc::c_int
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_necro_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    if (*quest.offset(1 as libc::c_int as isize)).status as libc::c_int >=
           1 as libc::c_int &&
           ((*quest.offset(1 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(0 as libc::c_int,
                 Some(quest_necro_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"necro_death\x00" as *const u8 as *const libc::c_char);
    }
    add_hook(5 as libc::c_int,
             Some(quest_main_monsters_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"main_new_monster\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_one_move_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut y: s32b = 0;
    let mut x: s32b = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    y = get_next_arg(fmt);
    x = get_next_arg(fmt);
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    if (*quest.offset(17 as libc::c_int as isize)).status as libc::c_int ==
           0 as libc::c_int {
        if ((*quest.offset(1 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
            return 0 as libc::c_int as bool_
        }
        if (*c_ptr).feat as libc::c_int != 0x4a as libc::c_int ||
               (*c_ptr).special as libc::c_int != 23 as libc::c_int {
            return 0 as libc::c_int as bool_
        }
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"You meet Galadriel; she seems worried.\x00" as *const u8
                       as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'So it was Sauron that lurked in Dol Guldur...\'\x00" as
                       *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'The Enemy is growing in power. Morgoth will be unreachable as long\'\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'as his most powerful servant, Sauron, lives. But the power of Sauron\'\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'lies in the One Ring. Our only hope is that you find it\'\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'and destroy it. I know it will tempt you, but *NEVER* use it\'\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'or it will corrupt you forever.\'\x00" as *const u8 as
                       *const libc::c_char);
        if (*p_ptr).pgod as libc::c_int == 1 as libc::c_int {
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"\'Also, Eru will abandon you if you wear it.\'\x00"
                           as *const u8 as *const libc::c_char);
        }
        if (*p_ptr).pgod as libc::c_int == 2 as libc::c_int {
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"\'Also, Manwe will abandon you if you wear it.\'\x00"
                           as *const u8 as *const libc::c_char);
        }
        if (*p_ptr).pgod as libc::c_int == 3 as libc::c_int {
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"\'Also, Tulkas will abandon you if you wear it.\'\x00"
                           as *const u8 as *const libc::c_char);
        }
        if (*p_ptr).pgod as libc::c_int == 5 as libc::c_int {
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"\'Also, Yavanna will abandon you if you wear it.\'\x00"
                           as *const u8 as *const libc::c_char);
        }
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'Without the destruction of the ring, Sauron\'s death can only be temporary\'\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'When you have it, bring it to Mount Doom, in Mordor,\'\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'to destroy it in the Great Fire where it was forged.\'\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'I do not know where to find it. Seek it through Middle-earth. Maybe there\'\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'are other people that might know.\'\x00" as *const u8
                       as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'Do not forget: the Ring must be cast back into the fires of Mount Doom!\'\x00"
                       as *const u8 as *const libc::c_char);
        if (*p_ptr).pgod as libc::c_int == 4 as libc::c_int {
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"\'Melkor will abandon you when you do, but you must do it anyway!\'\x00"
                           as *const u8 as *const libc::c_char);
        }
        (*quest.offset(17 as libc::c_int as isize)).status =
            1 as libc::c_int as s16b;
        (*quest.offset(17 as libc::c_int as
                           isize)).init.expect("non-null function pointer")(17
                                                                                as
                                                                                libc::c_int);
        return 1 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_one_drop_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut o_idx: s32b = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    o_idx = get_next_arg(fmt);
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(o_idx as isize) as
            *mut object_type;
    if (*quest.offset(17 as libc::c_int as isize)).status as libc::c_int !=
           1 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*o_ptr).name1 as libc::c_int != 13 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat as
           libc::c_int != 0xb2 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    cmsg_print(11 as libc::c_int as byte_hack,
               b"You throw the One Ring into the #RGreat Fire#y; it is rapidly consumed\x00"
                   as *const u8 as *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"by the searing flames.\x00" as *const u8 as
                   *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"You feel the powers of evil weakening.\x00" as *const u8 as
                   *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"Now you can go onto the hunt for Sauron!\x00" as *const u8 as
                   *const libc::c_char);
    inc_stack_size_ex(o_idx, -(99 as libc::c_int), OPTIMIZE, NO_DESCRIBE);
    abandon_god(4 as libc::c_int);
    (*quest.offset(17 as libc::c_int as isize)).status =
        5 as libc::c_int as s16b;
    *(*quest.offset(17 as libc::c_int as isize)).plot =
        2 as libc::c_int as s16b;
    (*quest.offset(*(*quest.offset(17 as libc::c_int as isize)).plot as
                       isize)).status = 1 as libc::c_int as s16b;
    (*quest.offset(*(*quest.offset(17 as libc::c_int as isize)).plot as
                       isize)).init.expect("non-null function pointer")(*(*quest.offset(17
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize)).plot
                                                                            as
                                                                            libc::c_int);
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_one_wield_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut o_idx: s32b = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    o_idx = get_next_arg(fmt);
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(o_idx as isize) as
            *mut object_type;
    if (*quest.offset(17 as libc::c_int as isize)).status as libc::c_int !=
           1 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*o_ptr).name1 as libc::c_int != 13 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    flush();
    if get_check(b"You were warned not to wear it; are you sure?\x00" as
                     *const u8 as *const libc::c_char) == 0 {
        return 1 as libc::c_int as bool_
    }
    flush();
    if get_check(b"You were warned not to wear it; are you *REALLY* sure?\x00"
                     as *const u8 as *const libc::c_char) == 0 {
        return 1 as libc::c_int as bool_
    }
    flush();
    if get_check(b"You were *WARNED* not to wear it; are you *R*E*A*L*L*Y* sure?\x00"
                     as *const u8 as *const libc::c_char) == 0 {
        return 1 as libc::c_int as bool_
    }
    cmsg_print(11 as libc::c_int as byte_hack,
               b"As you put it on your finger you feel #Ddark powers #ysapping your soul.\x00"
                   as *const u8 as *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"The ring firmly binds to your finger!\x00" as *const u8 as
                   *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"You feel you are drawn to the shadow world! Your material form weakens.\x00"
                   as *const u8 as *const libc::c_char);
    abandon_god(1 as libc::c_int);
    abandon_god(2 as libc::c_int);
    abandon_god(3 as libc::c_int);
    abandon_god(5 as libc::c_int);
    (*town_info.offset(1 as libc::c_int as isize)).destroyed =
        1 as libc::c_int as bool_;
    (*town_info.offset(2 as libc::c_int as isize)).destroyed =
        1 as libc::c_int as bool_;
    (*town_info.offset(3 as libc::c_int as isize)).destroyed =
        1 as libc::c_int as bool_;
    (*town_info.offset(4 as libc::c_int as isize)).destroyed =
        1 as libc::c_int as bool_;
    (*town_info.offset(5 as libc::c_int as isize)).destroyed =
        1 as libc::c_int as bool_;
    (*quest.offset(17 as libc::c_int as isize)).status =
        6 as libc::c_int as s16b;
    *(*quest.offset(17 as libc::c_int as isize)).plot =
        2 as libc::c_int as s16b;
    (*quest.offset(*(*quest.offset(17 as libc::c_int as isize)).plot as
                       isize)).status = 1 as libc::c_int as s16b;
    (*quest.offset(*(*quest.offset(17 as libc::c_int as isize)).plot as
                       isize)).init.expect("non-null function pointer")(*(*quest.offset(17
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize)).plot
                                                                            as
                                                                            libc::c_int);
    (*p_ptr).lives = 0 as libc::c_int;
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_one_hp_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    if (*quest.offset(17 as libc::c_int as isize)).status as libc::c_int ==
           6 as libc::c_int {
        let mut mhp: s32b = 0;
        let mut i: libc::c_int = 0;
        mhp = get_next_arg(fmt);
        i = 0 as libc::c_int;
        while i < (*p_ptr).lives + 1 as libc::c_int {
            mhp = mhp * 2 as libc::c_int / 3 as libc::c_int;
            i += 1
        }
        process_hooks_return[0 as libc::c_int as usize].num = mhp;
        return 1 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_one_die_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    if (*quest.offset(17 as libc::c_int as isize)).status as libc::c_int ==
           6 as libc::c_int {
        if (*p_ptr).mhp as libc::c_int > 1 as libc::c_int {
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"You feel the power of the One Ring sustaining your life,\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"but it drags you even more into the shadow world.\x00"
                           as *const u8 as *const libc::c_char);
            return 1 as libc::c_int as bool_
        } else {
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"The One Ring finally drags you totally to the shadow world.\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"Your mortal existence ends there.\x00" as *const u8
                           as *const libc::c_char);
            strcpy(died_from.as_mut_ptr(),
                   b"being drawn to the shadow world\x00" as *const u8 as
                       *const libc::c_char);
        }
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_one_identify_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut item: s32b = 0;
    item = get_next_arg(fmt);
    if (*quest.offset(17 as libc::c_int as isize)).status as libc::c_int ==
           1 as libc::c_int {
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
        o_ptr = get_object(item);
        if (*o_ptr).name1 as libc::c_int == 13 as libc::c_int &&
               !((*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
                     (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as
                         libc::c_int != 0 &&
                         (*k_info.offset((*o_ptr).k_idx as isize)).aware as
                             libc::c_int != 0) {
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"You finally found the One Ring, source of Sauron\'s power, and key to\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"its destruction. Remember, bring it to Mount Doom and destroy it.\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"And *NEVER* use it.\x00" as *const u8 as
                           *const libc::c_char);
        }
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_one_death_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut r_idx: s32b = 0;
    let mut m_idx: s32b = 0;
    let mut ok: bool_ = 0 as libc::c_int as bool_;
    m_idx = get_next_arg(fmt);
    r_idx = (*m_list.offset(m_idx as isize)).r_idx as s32b;
    if (*a_info.offset(13 as libc::c_int as isize)).cur_num != 0 {
        return 0 as libc::c_int as bool_
    }
    if (*quest.offset(17 as libc::c_int as isize)).status as libc::c_int !=
           1 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if Rand_div(100 as libc::c_int) < 30 as libc::c_int &&
           r_idx ==
               test_monster_name(b"Sauron, the Sorcerer\x00" as *const u8 as
                                     *const libc::c_char) {
        ok = 1 as libc::c_int as bool_
    } else if Rand_div(100 as libc::c_int) < 10 as libc::c_int &&
                  r_idx ==
                      test_monster_name(b"Ar-Pharazon the Golden\x00" as
                                            *const u8 as *const libc::c_char)
     {
        ok = 1 as libc::c_int as bool_
    } else if Rand_div(100 as libc::c_int) < 10 as libc::c_int &&
                  r_idx ==
                      test_monster_name(b"Shelob, Spider of Darkness\x00" as
                                            *const u8 as *const libc::c_char)
     {
        ok = 1 as libc::c_int as bool_
    } else if Rand_div(100 as libc::c_int) < 10 as libc::c_int &&
                  r_idx ==
                      test_monster_name(b"The Watcher in the Water\x00" as
                                            *const u8 as *const libc::c_char)
     {
        ok = 1 as libc::c_int as bool_
    } else if Rand_div(100 as libc::c_int) < 10 as libc::c_int &&
                  r_idx ==
                      test_monster_name(b"Glaurung, Father of the Dragons\x00"
                                            as *const u8 as
                                            *const libc::c_char) {
        ok = 1 as libc::c_int as bool_
    } else if Rand_div(100 as libc::c_int) < 10 as libc::c_int &&
                  r_idx ==
                      test_monster_name(b"Feagwath, the Undead Sorcerer\x00"
                                            as *const u8 as
                                            *const libc::c_char) {
        ok = 1 as libc::c_int as bool_
    }
    if ok != 0 {
        let mut i: libc::c_int = 0;
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
        object_prep(q_ptr,
                    lookup_kind(45 as libc::c_int, 37 as libc::c_int) as
                        libc::c_int);
        (*q_ptr).name1 = 13 as libc::c_int as byte_hack;
        apply_magic(q_ptr, -(1 as libc::c_int), 1 as libc::c_int as bool_,
                    1 as libc::c_int as bool_, 1 as libc::c_int as bool_);
        i = 0 as libc::c_int;
        while i < 23 as libc::c_int {
            if (*p_ptr).inventory[i as usize].k_idx == 0 { break ; }
            i += 1
        }
        if i == 23 as libc::c_int {
            let mut o_name: [libc::c_char; 200] = [0; 200];
            object_desc(o_name.as_mut_ptr(),
                        &mut *(*p_ptr).inventory.as_mut_ptr().offset((23 as
                                                                          libc::c_int
                                                                          -
                                                                          1 as
                                                                              libc::c_int)
                                                                         as
                                                                         isize),
                        0 as libc::c_int, 0 as libc::c_int);
            inven_drop(23 as libc::c_int - 1 as libc::c_int,
                       99 as libc::c_int, (*p_ptr).py as libc::c_int,
                       (*p_ptr).px as libc::c_int, 0 as libc::c_int as bool_);
            cmsg_format(10 as libc::c_int as byte_hack,
                        b"You feel the urge to drop your %s to make room in your inventory.\x00"
                            as *const u8 as *const libc::c_char,
                        o_name.as_mut_ptr());
        }
        cmsg_format(10 as libc::c_int as byte_hack,
                    b"You feel the urge to pick up that plain gold ring you see.\x00"
                        as *const u8 as *const libc::c_char);
        inven_carry(q_ptr, 0 as libc::c_int as bool_);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_one_dump_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    if (*quest.offset(17 as libc::c_int as isize)).status as libc::c_int ==
           5 as libc::c_int {
        fprintf(hook_file,
                b"\n You destroyed the One Ring, thus weakening Sauron.\x00"
                    as *const u8 as *const libc::c_char);
    }
    if (*quest.offset(17 as libc::c_int as isize)).status as libc::c_int ==
           6 as libc::c_int {
        fprintf(hook_file,
                b"\n You fell under the evil influence of the One Ring and decided to wear it.\x00"
                    as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_one_gen_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut x: s32b = 0;
    let mut y: s32b = 0;
    let mut tries: s32b = 10000 as libc::c_int;
    if (*quest.offset(17 as libc::c_int as isize)).status as libc::c_int !=
           1 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if dungeon_type as libc::c_int != 3 as libc::c_int ||
           dun_level as libc::c_int != 99 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    while tries != 0 {
        y =
            Rand_div(cur_hgt as libc::c_int - 4 as libc::c_int) +
                1 as libc::c_int + 2 as libc::c_int;
        x =
            Rand_div(cur_wid as libc::c_int - 4 as libc::c_int) +
                1 as libc::c_int + 2 as libc::c_int;
        if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                               isize)).flags1 as libc::c_long &
               0x10 as libc::c_long != 0 &&
               (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
                   0xaf as libc::c_int &&
               (*cave[y as usize].offset(x as isize)).m_idx == 0 &&
               !(y == (*p_ptr).py as libc::c_int &&
                     x == (*p_ptr).px as libc::c_int) {
            break ;
        }
        tries -= 1
    }
    if tries != 0 {
        let mut m_idx: libc::c_int =
            place_monster_one(y, x,
                              test_monster_name(b"Sauron, the Sorcerer\x00" as
                                                    *const u8 as
                                                    *const libc::c_char),
                              0 as libc::c_int, 0 as libc::c_int as bool_,
                              -(2 as libc::c_int)) as libc::c_int;
        if m_idx != 0 {
            let ref mut fresh12 = (*m_list.offset(m_idx as isize)).mflag;
            *fresh12 |= 0x2 as libc::c_int
        }
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_one_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    if (*quest.offset(17 as libc::c_int as isize)).status as libc::c_int >=
           1 as libc::c_int &&
           ((*quest.offset(17 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(43 as libc::c_int,
                 Some(quest_one_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"one_gen\x00" as *const u8 as *const libc::c_char);
        add_hook(0 as libc::c_int,
                 Some(quest_one_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"one_death\x00" as *const u8 as *const libc::c_char);
        add_hook(15 as libc::c_int,
                 Some(quest_one_drop_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"one_drop\x00" as *const u8 as *const libc::c_char);
        add_hook(21 as libc::c_int,
                 Some(quest_one_wield_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"one_wield\x00" as *const u8 as *const libc::c_char);
        add_hook(16 as libc::c_int,
                 Some(quest_one_identify_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"one_id\x00" as *const u8 as *const libc::c_char);
    }
    if (*quest.offset(17 as libc::c_int as isize)).status as libc::c_int ==
           0 as libc::c_int {
        add_hook(17 as libc::c_int,
                 Some(quest_one_move_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"one_move\x00" as *const u8 as *const libc::c_char);
    }
    add_hook(12 as libc::c_int,
             Some(quest_one_dump_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"one_dump\x00" as *const u8 as *const libc::c_char);
    add_hook(57 as libc::c_int,
             Some(quest_one_hp_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"one_hp\x00" as *const u8 as *const libc::c_char);
    add_hook(56 as libc::c_int,
             Some(quest_one_die_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"one_die\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_ultra_good_move_hook(mut fmt:
                                                        *mut libc::c_char)
 -> bool_ {
    let mut y: s32b = 0;
    let mut x: s32b = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    y = get_next_arg(fmt);
    x = get_next_arg(fmt);
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    if (*quest.offset(20 as libc::c_int as isize)).status as libc::c_int ==
           0 as libc::c_int {
        let mut old_quick_messages: bool_ = quick_messages;
        if ((*quest.offset(3 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
            return 0 as libc::c_int as bool_
        }
        if (*c_ptr).feat as libc::c_int != 0x4a as libc::c_int ||
               (*c_ptr).special as libc::c_int != 23 as libc::c_int {
            return 0 as libc::c_int as bool_
        }
        quick_messages = 0 as libc::c_int as bool_;
        cmsg_print(14 as libc::c_int as byte_hack,
                   b"You meet Galadriel.\x00" as *const u8 as
                       *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'I still cannot believe this is all over.\'\x00" as
                       *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'Morgoth\'s reign of terror is over at last!\'\x00" as
                       *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'His spirit has been banished to the Void where he cannot do much harm.\'\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'We can never thank you enough, hero!\'\x00" as
                       *const u8 as *const libc::c_char);
        cmsg_print(14 as libc::c_int as byte_hack,
                   b"Although everything seems all right, Galadriel seems a little subdued.\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'The spirit of Morgoth is not destroyed however -- only banished.\'\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'He can still control his allies left on Arda.\'\x00" as
                       *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'Maybe... maybe there could be a way to remove the threat of evil forever.\'\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'Somebody would have to go into the Void to do it.\'\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'But going there is certain death; we cannot ask it of anyone.\'\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'But you may choose, of your own free will, to attempt it...\'\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(14 as libc::c_int as byte_hack,
                   b"Galadriel plainly presents the choice that now lies before you:\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'You have earned the right to make whatever you wish of your future.\'\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'Become a ruler of Arda if you so desire; reign long, enjoying\'\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'the adulation of all, and have a happy life. Or, you can turn your\'\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'back on safety. Enter the Void, alone, to fight a hopeless battle\'\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'and face certain death.\'\x00" as *const u8 as
                       *const libc::c_char);
        flush();
        if get_check(b"Will you stay on Arda and lead a happy life?\x00" as
                         *const u8 as *const libc::c_char) == 0 {
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"\'So be it. I will open a portal to the Void.\'\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"\'But you must know this: the portal can only lead one way.\'\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"\'It will close once you enter, so as not to permit the horrors\'\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"\'that lurk in the Void to enter Arda. Your only way to come back\'\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"\'is to defeat the spirit of Morgoth, known as Melkor.\'\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"\'You will not be able to recall back either.\'\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"\'You can still choose to retire; it is not too late\'\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"\'to save your life.\'\x00" as *const u8 as
                           *const libc::c_char);
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"\'One last thing: It is quite certain that Melkor will have erected\'\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"\'powerful magical barriers around him. You certainly will\'\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"\'need to find a way to break them to get to him.\'\x00"
                           as *const u8 as *const libc::c_char);
            cave_set_feat(y - 5 as libc::c_int, x, 0x7 as libc::c_int);
            (*cave[(y - 5 as libc::c_int) as
                       usize].offset(x as isize)).special =
                11 as libc::c_int as s16b;
            (*quest.offset(20 as libc::c_int as isize)).status =
                1 as libc::c_int as s16b;
            (*quest.offset(20 as libc::c_int as
                               isize)).init.expect("non-null function pointer")(20
                                                                                    as
                                                                                    libc::c_int);
        }
        quick_messages = old_quick_messages;
        return 1 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_ultra_good_stair_hook(mut fmt:
                                                         *mut libc::c_char)
 -> bool_ {
    let mut dir: cptr = 0 as *const libc::c_char;
    dir = get_next_arg_str(fmt) as cptr;
    if dungeon_type as libc::c_int != 11 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if strcmp(dir, b"up\x00" as *const u8 as *const libc::c_char) == 0 &&
           dun_level as libc::c_int == 128 as libc::c_int {
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"The portal to Arda is now closed.\x00" as *const u8 as
                       *const libc::c_char);
        return 1 as libc::c_int as bool_
    }
    if strcmp(dir, b"up\x00" as *const u8 as *const libc::c_char) == 0 &&
           dun_level as libc::c_int == 150 as libc::c_int {
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"The barrier seems to be impenetrable from this side.\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"You will have to move on.\x00" as *const u8 as
                       *const libc::c_char);
        return 1 as libc::c_int as bool_
    }
    if strcmp(dir, b"down\x00" as *const u8 as *const libc::c_char) == 0 &&
           dun_level as libc::c_int == 149 as libc::c_int {
        let mut i: libc::c_int = 0;
        let mut ultimate: bool_ = 0 as libc::c_int as bool_;
        i = 24 as libc::c_int;
        while i < 52 as libc::c_int {
            let mut f1: u32b = 0;
            let mut f2: u32b = 0;
            let mut f3: u32b = 0;
            let mut f4: u32b = 0;
            let mut f5: u32b = 0;
            let mut esp: u32b = 0;
            let mut o_ptr: *mut object_type = get_object(i);
            if !((*o_ptr).k_idx == 0) {
                object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4,
                             &mut f5, &mut esp);
                if f4 as libc::c_long & 0x1000000 as libc::c_long != 0 {
                    ultimate = 1 as libc::c_int as bool_;
                    break ;
                }
            }
            i += 1
        }
        if ultimate == 0 {
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"It seems the level is protected by an impassable barrier of pure magic.\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"Only the most powerful magic could remove it. You will need to use\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"the Flame Imperishable to pass. The source of Eru Iluvatar\'s own power.\x00"
                           as *const u8 as *const libc::c_char);
            return 1 as libc::c_int as bool_
        } else {
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"The power of the Flame Imperishable shatters the magical barrier.\x00"
                           as *const u8 as *const libc::c_char);
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"The way before you is free.\x00" as *const u8 as
                           *const libc::c_char);
        }
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_ultra_good_recall_hook(mut fmt:
                                                          *mut libc::c_char)
 -> bool_ {
    if dungeon_type as libc::c_int != 11 as libc::c_int &&
           dungeon_type as libc::c_int != 6 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    cmsg_print(11 as libc::c_int as byte_hack,
               b"You cannot recall. The portal to Arda is closed.\x00" as
                   *const u8 as *const libc::c_char);
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_ultra_good_death_hook(mut fmt:
                                                         *mut libc::c_char)
 -> bool_ {
    let mut m_idx: s32b = get_next_arg(fmt);
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    if (*m_ptr).r_idx as libc::c_int == 1044 as libc::c_int {
        total_winner = 2 as libc::c_int as u16b;
        has_won = 2 as libc::c_int as u16b;
        (*quest.offset(20 as libc::c_int as isize)).status =
            5 as libc::c_int as s16b;
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x2 as libc::c_long) as u32b;
        cmsg_print(13 as libc::c_int as byte_hack,
                   b"****** CONGRATULATIONS ******\x00" as *const u8 as
                       *const libc::c_char);
        cmsg_print(13 as libc::c_int as byte_hack,
                   b"You have done more than the impossible. You ended the threat of\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(13 as libc::c_int as byte_hack,
                   b"Melkor forever. Thanks to you, Arda will live in eternal peace.\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(13 as libc::c_int as byte_hack,
                   b"You feel the spirit of Eru touching you. You feel your spirit rising!\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(13 as libc::c_int as byte_hack,
                   b"Before you, a portal to Arda opens. You can now come back to your world\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(13 as libc::c_int as byte_hack,
                   b"and live happily ever after.\x00" as *const u8 as
                       *const libc::c_char);
        cmsg_print(13 as libc::c_int as byte_hack,
                   b"What you do now is up to you, but your deeds shall ever be remembered.\x00"
                       as *const u8 as *const libc::c_char);
        cmsg_print(13 as libc::c_int as byte_hack,
                   b"You may retire (commit suicide) when you are ready.\x00"
                       as *const u8 as *const libc::c_char);
        cave_set_feat((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                      0x7 as libc::c_int);
        del_hook(0 as libc::c_int,
                 Some(quest_ultra_good_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        process_hooks_restart = 1 as libc::c_int;
        *(*quest.offset(20 as libc::c_int as isize)).plot =
            0 as libc::c_int as s16b
    }
    if (*m_ptr).r_idx as libc::c_int == 1032 as libc::c_int {
        let mut i: libc::c_int = 0;
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
        object_prep(q_ptr,
                    lookup_kind(11 as libc::c_int, 255 as libc::c_int) as
                        libc::c_int);
        *k_allow_special.offset(296 as libc::c_int as isize) =
            1 as libc::c_int as bool_;
        apply_magic(q_ptr, -(1 as libc::c_int), 1 as libc::c_int as bool_,
                    1 as libc::c_int as bool_, 1 as libc::c_int as bool_);
        *k_allow_special.offset(296 as libc::c_int as isize) =
            0 as libc::c_int as bool_;
        object_aware(q_ptr);
        object_known(q_ptr);
        (*q_ptr).ident =
            ((*q_ptr).ident as libc::c_int | 0x20 as libc::c_int) as
                byte_hack;
        i = 0 as libc::c_int;
        while i < 23 as libc::c_int {
            if (*p_ptr).inventory[i as usize].k_idx == 0 { break ; }
            i += 1
        }
        if i == 23 as libc::c_int {
            let mut o_name: [libc::c_char; 200] = [0; 200];
            object_desc(o_name.as_mut_ptr(),
                        &mut *(*p_ptr).inventory.as_mut_ptr().offset((23 as
                                                                          libc::c_int
                                                                          -
                                                                          1 as
                                                                              libc::c_int)
                                                                         as
                                                                         isize),
                        0 as libc::c_int, 0 as libc::c_int);
            inven_drop(23 as libc::c_int - 1 as libc::c_int,
                       99 as libc::c_int, (*p_ptr).py as libc::c_int,
                       (*p_ptr).px as libc::c_int, 0 as libc::c_int as bool_);
            cmsg_format(10 as libc::c_int as byte_hack,
                        b"You feel the urge to drop your %s to make room in your inventory.\x00"
                            as *const u8 as *const libc::c_char,
                        o_name.as_mut_ptr());
        }
        cmsg_format(10 as libc::c_int as byte_hack,
                    b"You feel the urge to pick up the Flame Imperishable.\x00"
                        as *const u8 as *const libc::c_char);
        inven_carry(q_ptr, 0 as libc::c_int as bool_);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_ultra_good_dump_hook(mut fmt:
                                                        *mut libc::c_char)
 -> bool_ {
    if (*quest.offset(20 as libc::c_int as isize)).status as libc::c_int >=
           1 as libc::c_int {
        if total_winner as libc::c_int == 2 as libc::c_int {
            fprintf(hook_file,
                    b"\n You destroyed Melkor forever and have been elevated to the status of Vala by Eru Iluvatar.\x00"
                        as *const u8 as *const libc::c_char);
            fprintf(hook_file,
                    b"\n Arda will forever be free.\x00" as *const u8 as
                        *const libc::c_char);
        } else if death != 0 {
            fprintf(hook_file,
                    b"\n You tried to destroy Melkor forever, but died in the attempt.\x00"
                        as *const u8 as *const libc::c_char);
            fprintf(hook_file,
                    b"\n Arda will be quiet, but not free from evil.\x00" as
                        *const u8 as *const libc::c_char);
        }
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_ultra_good_init_hook(mut q: libc::c_int)
 -> bool_ {
    if (*quest.offset(20 as libc::c_int as isize)).status as libc::c_int >=
           1 as libc::c_int &&
           ((*quest.offset(20 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(18 as libc::c_int,
                 Some(quest_ultra_good_stair_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"ultrag_stair\x00" as *const u8 as *const libc::c_char);
        add_hook(62 as libc::c_int,
                 Some(quest_ultra_good_recall_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"ultrag_recall\x00" as *const u8 as *const libc::c_char);
        add_hook(0 as libc::c_int,
                 Some(quest_ultra_good_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"ultrag_death\x00" as *const u8 as *const libc::c_char);
    }
    if (*quest.offset(20 as libc::c_int as isize)).status as libc::c_int ==
           0 as libc::c_int {
        add_hook(17 as libc::c_int,
                 Some(quest_ultra_good_move_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"ultrag_move\x00" as *const u8 as *const libc::c_char);
    }
    add_hook(12 as libc::c_int,
             Some(quest_ultra_good_dump_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"ultrag_dump\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_ultra_evil_init_hook(mut q: libc::c_int)
 -> bool_ {
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_thieves_gen_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut xstart: libc::c_int = 2 as libc::c_int;
    let mut ystart: libc::c_int = 2 as libc::c_int;
    let mut again: bool_ = 1 as libc::c_int as bool_;
    if (*p_ptr).inside_quest as libc::c_int != 4 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*quest.offset(4 as libc::c_int as isize)).status as libc::c_int ==
           0 as libc::c_int {
        (*quest.offset(4 as libc::c_int as isize)).status =
            1 as libc::c_int as s16b
    }
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            cave_set_feat(y, x, 0x3f as libc::c_int);
            x += 1
        }
        y += 1
    }
    dun_level = (*quest.offset((*p_ptr).inside_quest as isize)).level;
    set_mon_num_hook();
    get_mon_num_prep();
    init_flags = 0x4 as libc::c_int;
    process_dungeon_file(b"thieves.map\x00" as *const u8 as
                             *const libc::c_char, &mut ystart, &mut xstart,
                         cur_hgt as libc::c_int, cur_wid as libc::c_int,
                         1 as libc::c_int as bool_,
                         0 as libc::c_int as bool_);
    dungeon_flags2 =
        (dungeon_flags2 as libc::c_long | 0x200 as libc::c_long) as u32b;
    cmsg_print(11 as libc::c_int as byte_hack,
               b"You feel a vicious blow on your head.\x00" as *const u8 as
                   *const libc::c_char);
    while again != 0 {
        again = 0 as libc::c_int as bool_;
        x = 0 as libc::c_int;
        while x < 52 as libc::c_int {
            let mut o_ptr: *mut object_type =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(x as isize) as
                    *mut object_type;
            if !((*o_ptr).k_idx == 0) {
                if !(x >= 24 as libc::c_int &&
                         (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int
                             != 0) {
                    inven_drop(x, 99 as libc::c_int, 4 as libc::c_int,
                               24 as libc::c_int, 1 as libc::c_int as bool_);
                    again = 1 as libc::c_int as bool_;
                    break ;
                }
            }
            x += 1
        }
    }
    del_hook(2 as libc::c_int,
             Some(quest_thieves_gen_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_));
    process_hooks_restart = 1 as libc::c_int;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_thieves_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut mcnt: libc::c_int = 0 as libc::c_int;
    if (*p_ptr).inside_quest as libc::c_int != 4 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*cave[17 as libc::c_int as
                  usize].offset(22 as libc::c_int as isize)).feat as
           libc::c_int == 0x4 as libc::c_int ||
           (*cave[17 as libc::c_int as
                      usize].offset(22 as libc::c_int as isize)).feat as
               libc::c_int == 0x5 as libc::c_int {
        cmsg_print(12 as libc::c_int as byte_hack,
                   b"An alarm rings!\x00" as *const u8 as
                       *const libc::c_char);
        aggravate_monsters(0 as libc::c_int);
        cave_set_feat(14 as libc::c_int, 20 as libc::c_int,
                      0x4 as libc::c_int);
        cave_set_feat(14 as libc::c_int, 16 as libc::c_int,
                      0x4 as libc::c_int);
        cave_set_feat(14 as libc::c_int, 12 as libc::c_int,
                      0x4 as libc::c_int);
        cave_set_feat(14 as libc::c_int, 8 as libc::c_int,
                      0x4 as libc::c_int);
        cave_set_feat(20 as libc::c_int, 20 as libc::c_int,
                      0x4 as libc::c_int);
        cave_set_feat(20 as libc::c_int, 16 as libc::c_int,
                      0x4 as libc::c_int);
        cave_set_feat(20 as libc::c_int, 12 as libc::c_int,
                      0x4 as libc::c_int);
        cave_set_feat(20 as libc::c_int, 8 as libc::c_int,
                      0x4 as libc::c_int);
        msg_print(b"The door explodes.\x00" as *const u8 as
                      *const libc::c_char);
        cave_set_feat(17 as libc::c_int, 22 as libc::c_int,
                      0x1 as libc::c_int);
    }
    i = m_max as libc::c_int - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        if !((*m_ptr).r_idx == 0) {
            if ((*m_ptr).status as libc::c_int) < 2 as libc::c_int {
                mcnt += 1
            }
        }
        i -= 1
    }
    if mcnt == 0 {
        msg_print(b"The magic hiding the stairs is now gone.\x00" as *const u8
                      as *const libc::c_char);
        cave_set_feat(23 as libc::c_int, 4 as libc::c_int,
                      0x6 as libc::c_int);
        (*cave[23 as libc::c_int as
                   usize].offset(4 as libc::c_int as isize)).special =
            0 as libc::c_int as s16b;
        (*quest.offset((*p_ptr).inside_quest as isize)).status =
            2 as libc::c_int as s16b;
        del_hook(3 as libc::c_int,
                 Some(quest_thieves_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        process_hooks_restart = 1 as libc::c_int;
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"You stopped the thieves and saved Bree!\x00" as *const u8
                       as *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_thieves_finish_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut q_idx: s32b = 0;
    q_idx = get_next_arg(fmt);
    if q_idx != 4 as libc::c_int { return 0 as libc::c_int as bool_ }
    c_put_str(11 as libc::c_int as byte_hack,
              b"Thank you for killing the band of thieves!\x00" as *const u8
                  as *const libc::c_char, 8 as libc::c_int, 0 as libc::c_int);
    c_put_str(11 as libc::c_int as byte_hack,
              b"You can use the hideout as your house as a reward.\x00" as
                  *const u8 as *const libc::c_char, 9 as libc::c_int,
              0 as libc::c_int);
    if Rand_div(100 as libc::c_int) < 10 as libc::c_int ||
           (*s_info.offset(16 as libc::c_int as isize)).value ==
               (*s_info.offset(15 as libc::c_int as isize)).value {
        *(*quest.offset(q_idx as isize)).plot =
            if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
                8 as libc::c_int
            } else { 9 as libc::c_int } as s16b
    } else if (*s_info.offset(16 as libc::c_int as isize)).value >
                  (*s_info.offset(15 as libc::c_int as isize)).value {
        *(*quest.offset(q_idx as isize)).plot = 8 as libc::c_int as s16b
    } else {
        *(*quest.offset(q_idx as isize)).plot = 9 as libc::c_int as s16b
    }
    (*quest.offset(*(*quest.offset(q_idx as isize)).plot as
                       isize)).init.expect("non-null function pointer")(*(*quest.offset(q_idx
                                                                                            as
                                                                                            isize)).plot
                                                                            as
                                                                            libc::c_int);
    del_hook(9 as libc::c_int,
             Some(quest_thieves_finish_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_));
    process_hooks_restart = 1 as libc::c_int;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_thieves_feeling_hook(mut fmt:
                                                        *mut libc::c_char)
 -> bool_ {
    if (*p_ptr).inside_quest as libc::c_int != 4 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    msg_print(b"You wake up in a prison cell.\x00" as *const u8 as
                  *const libc::c_char);
    msg_print(b"All your possessions have been stolen!\x00" as *const u8 as
                  *const libc::c_char);
    del_hook(4 as libc::c_int,
             Some(quest_thieves_feeling_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_));
    process_hooks_restart = 1 as libc::c_int;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_thieves_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    if (*quest.offset(4 as libc::c_int as isize)).status as libc::c_int >=
           0 as libc::c_int &&
           ((*quest.offset(4 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(3 as libc::c_int,
                 Some(quest_thieves_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"thieves_end_turn\x00" as *const u8 as *const libc::c_char);
        add_hook(9 as libc::c_int,
                 Some(quest_thieves_finish_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"thieves_finish\x00" as *const u8 as *const libc::c_char);
        add_hook(2 as libc::c_int,
                 Some(quest_thieves_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"thieves_geb\x00" as *const u8 as *const libc::c_char);
        add_hook(4 as libc::c_int,
                 Some(quest_thieves_feeling_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"thieves_feel\x00" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_hobbit_town_gen_hook(mut fmt:
                                                        *mut libc::c_char)
 -> bool_ {
    let mut x: libc::c_int = 1 as libc::c_int;
    let mut y: libc::c_int = 1 as libc::c_int;
    let mut tries: libc::c_int = 10000 as libc::c_int;
    let mut small: s32b = 0;
    small = get_next_arg(fmt);
    if (turn as libc::c_long) <
           (*quest.offset(6 as libc::c_int as
                              isize)).data[1 as libc::c_int as usize] as
               libc::c_long +
               11520 as libc::c_int as libc::c_long * 10 as libc::c_long ||
           (*quest.offset(6 as libc::c_int as isize)).status as libc::c_int >
               2 as libc::c_int || small != 0 ||
           (*p_ptr).town_num as libc::c_int != 1 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    while tries != 0 {
        y =
            Rand_div(20 as libc::c_int) + 1 as libc::c_int +
                cur_hgt as libc::c_int / 2 as libc::c_int - 10 as libc::c_int;
        x =
            Rand_div(20 as libc::c_int) + 1 as libc::c_int +
                cur_wid as libc::c_int / 2 as libc::c_int - 10 as libc::c_int;
        if los((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int, y, x)
               == 0 &&
               ((*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                    isize)).flags1 as libc::c_long &
                    0x10 as libc::c_long != 0 &&
                    (*cave[y as usize].offset(x as isize)).feat as libc::c_int
                        != 0xaf as libc::c_int &&
                    (*cave[y as usize].offset(x as isize)).m_idx == 0 &&
                    !(y == (*p_ptr).py as libc::c_int &&
                          x == (*p_ptr).px as libc::c_int)) &&
               ((*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                    isize)).flags1 as libc::c_long &
                    0x10 as libc::c_long != 0 &&
                    (*f_info.offset((*cave[y as
                                               usize].offset(x as isize)).feat
                                        as isize)).flags1 as libc::c_long &
                        0x100 as libc::c_long == 0) {
            break ;
        }
        tries -= 1
    }
    *m_allow_special.offset(test_monster_name(b"Melinda Proudfoot\x00" as
                                                  *const u8 as
                                                  *const libc::c_char) as
                                isize) = 1 as libc::c_int as bool_;
    place_monster_one(y, x,
                      test_monster_name(b"Melinda Proudfoot\x00" as *const u8
                                            as *const libc::c_char),
                      0 as libc::c_int, 0 as libc::c_int as bool_,
                      -(2 as libc::c_int));
    *m_allow_special.offset(test_monster_name(b"Melinda Proudfoot\x00" as
                                                  *const u8 as
                                                  *const libc::c_char) as
                                isize) = 0 as libc::c_int as bool_;
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_hobbit_gen_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut x: libc::c_int = 1 as libc::c_int;
    let mut y: libc::c_int = 1 as libc::c_int;
    let mut tries: libc::c_int = 10000 as libc::c_int;
    if (*quest.offset(6 as libc::c_int as isize)).status as libc::c_int !=
           1 as libc::c_int ||
           dun_level as libc::c_int !=
               (*quest.offset(6 as libc::c_int as
                                  isize)).data[0 as libc::c_int as usize] ||
           dungeon_type as libc::c_int != 18 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    while tries != 0 {
        y =
            Rand_div(cur_hgt as libc::c_int - 4 as libc::c_int) +
                1 as libc::c_int + 2 as libc::c_int;
        x =
            Rand_div(cur_wid as libc::c_int - 4 as libc::c_int) +
                1 as libc::c_int + 2 as libc::c_int;
        if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                               isize)).flags1 as libc::c_long &
               0x10 as libc::c_long != 0 &&
               (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
                   0xaf as libc::c_int &&
               (*cave[y as usize].offset(x as isize)).m_idx == 0 &&
               !(y == (*p_ptr).py as libc::c_int &&
                     x == (*p_ptr).px as libc::c_int) {
            break ;
        }
        tries -= 1
    }
    *m_allow_special.offset(test_monster_name(b"Merton Proudfoot, the lost hobbit\x00"
                                                  as *const u8 as
                                                  *const libc::c_char) as
                                isize) = 1 as libc::c_int as bool_;
    place_monster_one(y, x,
                      test_monster_name(b"Merton Proudfoot, the lost hobbit\x00"
                                            as *const u8 as
                                            *const libc::c_char),
                      0 as libc::c_int, 0 as libc::c_int as bool_,
                      2 as libc::c_int);
    *m_allow_special.offset(test_monster_name(b"Merton Proudfoot, the lost hobbit\x00"
                                                  as *const u8 as
                                                  *const libc::c_char) as
                                isize) = 0 as libc::c_int as bool_;
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_hobbit_give_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    let mut m_idx: s32b = 0;
    let mut item: s32b = 0;
    m_idx = get_next_arg(fmt);
    item = get_next_arg(fmt);
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(item as isize) as
            *mut object_type;
    m_ptr = &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    if (*m_ptr).r_idx as libc::c_int !=
           test_monster_name(b"Merton Proudfoot, the lost hobbit\x00" as
                                 *const u8 as *const libc::c_char) {
        return 0 as libc::c_int as bool_
    }
    if (*o_ptr).tval as libc::c_int != 70 as libc::c_int ||
           (*o_ptr).sval as libc::c_int != 11 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    msg_print(b"\'Oh, thank you, noble one!\'\x00" as *const u8 as
                  *const libc::c_char);
    msg_print(b"Merton Proudfoot reads the scroll and is recalled to the safety of his home.\x00"
                  as *const u8 as *const libc::c_char);
    delete_monster_idx(m_idx);
    inc_stack_size_ex(item, -(1 as libc::c_int), OPTIMIZE, NO_DESCRIBE);
    (*quest.offset(6 as libc::c_int as isize)).status =
        2 as libc::c_int as s16b;
    del_hook(11 as libc::c_int,
             Some(quest_hobbit_give_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_));
    process_hooks_restart = 1 as libc::c_int;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_hobbit_speak_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut m_idx: s32b = get_next_arg(fmt);
    if (*m_list.offset(m_idx as isize)).r_idx as libc::c_int !=
           test_monster_name(b"Melinda Proudfoot\x00" as *const u8 as
                                 *const libc::c_char) {
        return 0 as libc::c_int as bool_
    }
    if ((*quest.offset(6 as libc::c_int as isize)).status as libc::c_int) <
           2 as libc::c_int {
        let mut m_name: cptr = 0 as *const libc::c_char;
        m_name = get_next_arg_str(fmt) as cptr;
        msg_format(b"%^s begs for your help.\x00" as *const u8 as
                       *const libc::c_char, m_name);
    }
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_hobbit_chat_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    let mut m_idx: s32b = 0;
    m_idx = get_next_arg(fmt);
    m_ptr = &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    if (*m_ptr).r_idx as libc::c_int !=
           test_monster_name(b"Melinda Proudfoot\x00" as *const u8 as
                                 *const libc::c_char) {
        return 0 as libc::c_int as bool_
    }
    if ((*quest.offset(6 as libc::c_int as isize)).status as libc::c_int) <
           2 as libc::c_int {
        msg_print(b"Oh! Oh!\x00" as *const u8 as *const libc::c_char);
        msg_print(b"My poor Merton, where is my poor Merton? He was playing near that dreadful\x00"
                      as *const u8 as *const libc::c_char);
        msg_print(b"maze and never been seen again! Could you find him for me?\x00"
                      as *const u8 as *const libc::c_char);
        (*quest.offset(6 as libc::c_int as isize)).status =
            1 as libc::c_int as s16b;
        (*quest.offset(6 as libc::c_int as
                           isize)).init.expect("non-null function pointer")(6
                                                                                as
                                                                                libc::c_int);
    } else if (*quest.offset(6 as libc::c_int as isize)).status as libc::c_int
                  == 2 as libc::c_int {
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
        msg_print(b"My Merton is back! You saved him, hero.\x00" as *const u8
                      as *const libc::c_char);
        msg_print(b"Take this as a proof of my gratitude.  It was given to my family\x00"
                      as *const u8 as *const libc::c_char);
        msg_print(b"by a famed wizard, but it should serve you better than me.\x00"
                      as *const u8 as *const libc::c_char);
        q_ptr = &mut forge;
        object_prep(q_ptr,
                    lookup_kind(66 as libc::c_int, 3 as libc::c_int) as
                        libc::c_int);
        (*q_ptr).number = 1 as libc::c_int as byte_hack;
        (*q_ptr).found = 6 as libc::c_int as byte_hack;
        object_aware(q_ptr);
        object_known(q_ptr);
        (*q_ptr).ident =
            ((*q_ptr).ident as libc::c_int | 0x10 as libc::c_int) as
                byte_hack;
        inven_carry(q_ptr, 0 as libc::c_int as bool_);
        (*quest.offset(6 as libc::c_int as isize)).status =
            5 as libc::c_int as s16b;
        del_hook(33 as libc::c_int,
                 Some(quest_hobbit_speak_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        process_hooks_restart = 1 as libc::c_int;
        delete_monster_idx(m_idx);
        return 1 as libc::c_int as bool_
    } else {
        msg_print(b"Thanks again.\x00" as *const u8 as *const libc::c_char);
    }
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_hobbit_dump_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    if (*quest.offset(6 as libc::c_int as isize)).status as libc::c_int >=
           2 as libc::c_int {
        fprintf(hook_file,
                b"\n You saved a young hobbit from an horrible fate.\x00" as
                    *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_hobbit_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    if (*quest.offset(6 as libc::c_int as
                          isize)).data[0 as libc::c_int as usize] == 0 {
        (*quest.offset(6 as libc::c_int as
                           isize)).data[0 as libc::c_int as usize] =
            26 as libc::c_int +
                Rand_div(1 as libc::c_int + 34 as libc::c_int -
                             26 as libc::c_int);
        (*quest.offset(6 as libc::c_int as
                           isize)).data[1 as libc::c_int as usize] = turn;
        if wizard != 0 {
            message_add(1 as libc::c_int as byte_hack,
                        format(b"Hobbit level %d\x00" as *const u8 as
                                   *const libc::c_char,
                               (*quest.offset(6 as libc::c_int as
                                                  isize)).data[0 as
                                                                   libc::c_int
                                                                   as usize])
                            as cptr, 6 as libc::c_int as byte_hack);
        }
    }
    if (*quest.offset(6 as libc::c_int as isize)).status as libc::c_int >=
           1 as libc::c_int &&
           ((*quest.offset(6 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(11 as libc::c_int,
                 Some(quest_hobbit_give_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"hobbit_give\x00" as *const u8 as *const libc::c_char);
        add_hook(6 as libc::c_int,
                 Some(quest_hobbit_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"hobbit_gen\x00" as *const u8 as *const libc::c_char);
        add_hook(14 as libc::c_int,
                 Some(quest_hobbit_town_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"hobbit_town_gen\x00" as *const u8 as *const libc::c_char);
        add_hook(32 as libc::c_int,
                 Some(quest_hobbit_chat_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"hobbit_chat\x00" as *const u8 as *const libc::c_char);
        add_hook(33 as libc::c_int,
                 Some(quest_hobbit_speak_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"hobbit_speak\x00" as *const u8 as *const libc::c_char);
    }
    if (*quest.offset(6 as libc::c_int as isize)).status as libc::c_int ==
           0 as libc::c_int {
        add_hook(33 as libc::c_int,
                 Some(quest_hobbit_speak_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"hobbit_speak\x00" as *const u8 as *const libc::c_char);
        add_hook(14 as libc::c_int,
                 Some(quest_hobbit_town_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"hobbit_town_gen\x00" as *const u8 as *const libc::c_char);
        add_hook(32 as libc::c_int,
                 Some(quest_hobbit_chat_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"hobbit_chat\x00" as *const u8 as *const libc::c_char);
    }
    add_hook(12 as libc::c_int,
             Some(quest_hobbit_dump_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"hobbit_dump\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_troll_gen_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut xstart: libc::c_int = 2 as libc::c_int;
    let mut ystart: libc::c_int = 2 as libc::c_int;
    if (*p_ptr).inside_quest as libc::c_int != 8 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            cave_set_feat(y, x, 0x3f as libc::c_int);
            x += 1
        }
        y += 1
    }
    dun_level = (*quest.offset((*p_ptr).inside_quest as isize)).level;
    set_mon_num_hook();
    get_mon_num_prep();
    init_flags = 0x4 as libc::c_int;
    process_dungeon_file(b"trolls.map\x00" as *const u8 as
                             *const libc::c_char, &mut ystart, &mut xstart,
                         cur_hgt as libc::c_int, cur_wid as libc::c_int,
                         1 as libc::c_int as bool_,
                         1 as libc::c_int as bool_);
    x = 3 as libc::c_int;
    while x < xstart {
        y = 3 as libc::c_int;
        while y < ystart {
            if (*cave[y as usize].offset(x as isize)).feat as libc::c_int ==
                   0xac as libc::c_int {
                let mut m_idx: libc::c_int = 0;
                *m_allow_special.offset(test_monster_name(b"Tom the Stone Troll\x00"
                                                              as *const u8 as
                                                              *const libc::c_char)
                                            as isize) =
                    1 as libc::c_int as bool_;
                m_idx =
                    place_monster_one(y, x,
                                      test_monster_name(b"Tom the Stone Troll\x00"
                                                            as *const u8 as
                                                            *const libc::c_char),
                                      0 as libc::c_int,
                                      0 as libc::c_int as bool_,
                                      -(2 as libc::c_int)) as libc::c_int;
                *m_allow_special.offset(test_monster_name(b"Tom the Stone Troll\x00"
                                                              as *const u8 as
                                                              *const libc::c_char)
                                            as isize) =
                    0 as libc::c_int as bool_;
                if m_idx != 0 {
                    let mut o_idx: libc::c_int = 0;
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
                    let ref mut fresh13 =
                        (*m_list.offset(m_idx as isize)).mflag;
                    *fresh13 |= 0x2 as libc::c_int;
                    *a_allow_special.offset(73 as libc::c_int as isize) =
                        1 as libc::c_int as bool_;
                    object_prep(q_ptr,
                                lookup_kind(23 as libc::c_int,
                                            16 as libc::c_int) as
                                    libc::c_int);
                    (*q_ptr).name1 = 73 as libc::c_int as byte_hack;
                    apply_magic(q_ptr, -(1 as libc::c_int),
                                1 as libc::c_int as bool_,
                                1 as libc::c_int as bool_,
                                1 as libc::c_int as bool_);
                    *a_allow_special.offset(73 as libc::c_int as isize) =
                        0 as libc::c_int as bool_;
                    o_idx = o_pop() as libc::c_int;
                    if o_idx != 0 {
                        let mut o_ptr: *mut object_type =
                            &mut *o_list.offset(o_idx as isize) as
                                *mut object_type;
                        object_copy(o_ptr, q_ptr);
                        (*o_ptr).next_o_idx =
                            (*m_list.offset(m_idx as isize)).hold_o_idx;
                        (*o_ptr).held_m_idx = m_idx as s16b;
                        (*o_ptr).ix = 0 as libc::c_int as byte_hack;
                        (*o_ptr).iy = 0 as libc::c_int as byte_hack;
                        (*m_list.offset(m_idx as isize)).hold_o_idx =
                            o_idx as s16b
                    } else {
                        (*a_info.offset((*q_ptr).name1 as isize)).cur_num =
                            0 as libc::c_int as byte_hack
                    }
                }
            }
            y += 1
        }
        x += 1
    }
    (*quest.offset(8 as libc::c_int as isize)).data[0 as libc::c_int as usize]
        = 0 as libc::c_int;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_troll_finish_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut q_idx: s32b = 0;
    q_idx = get_next_arg(fmt);
    if q_idx != 8 as libc::c_int { return 0 as libc::c_int as bool_ }
    c_put_str(11 as libc::c_int as byte_hack,
              b"I heard about your noble deeds.\x00" as *const u8 as
                  *const libc::c_char, 8 as libc::c_int, 0 as libc::c_int);
    c_put_str(11 as libc::c_int as byte_hack,
              b"Keep what you found... may it serve you well.\x00" as
                  *const u8 as *const libc::c_char, 9 as libc::c_int,
              0 as libc::c_int);
    *(*quest.offset(q_idx as isize)).plot = 7 as libc::c_int as s16b;
    (*quest.offset(*(*quest.offset(q_idx as isize)).plot as
                       isize)).init.expect("non-null function pointer")(*(*quest.offset(q_idx
                                                                                            as
                                                                                            isize)).plot
                                                                            as
                                                                            libc::c_int);
    del_hook(9 as libc::c_int,
             Some(quest_troll_finish_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_));
    process_hooks_restart = 1 as libc::c_int;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_troll_death_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut xstart: libc::c_int = 2 as libc::c_int;
    let mut ystart: libc::c_int = 2 as libc::c_int;
    let mut r_idx: s32b = 0;
    let mut m_idx: s32b = 0;
    m_idx = get_next_arg(fmt);
    r_idx = (*m_list.offset(m_idx as isize)).r_idx as s32b;
    if (*p_ptr).inside_quest as libc::c_int != 8 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if r_idx ==
           test_monster_name(b"Tom the Stone Troll\x00" as *const u8 as
                                 *const libc::c_char) {
        cave_set_feat(3 as libc::c_int, 3 as libc::c_int, 0x6 as libc::c_int);
        (*cave[3 as libc::c_int as
                   usize].offset(3 as libc::c_int as isize)).special =
            0 as libc::c_int as s16b;
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"Without Tom, the trolls won\'t be able to do much.\x00"
                       as *const u8 as *const libc::c_char);
        (*quest.offset(8 as libc::c_int as isize)).status =
            2 as libc::c_int as s16b;
        del_hook(0 as libc::c_int,
                 Some(quest_troll_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        process_hooks_restart = 1 as libc::c_int;
        return 0 as libc::c_int as bool_
    }
    init_flags = 0x8 as libc::c_int;
    process_dungeon_file(b"trolls.map\x00" as *const u8 as
                             *const libc::c_char, &mut ystart, &mut xstart,
                         cur_hgt as libc::c_int, cur_wid as libc::c_int,
                         1 as libc::c_int as bool_,
                         1 as libc::c_int as bool_);
    if (*quest.offset(8 as libc::c_int as
                          isize)).data[0 as libc::c_int as usize] != 0 {
        return 0 as libc::c_int as bool_
    }
    (*quest.offset(8 as libc::c_int as isize)).data[0 as libc::c_int as usize]
        = 1 as libc::c_int;
    msg_print(b"Oops, seems like an ambush...\x00" as *const u8 as
                  *const libc::c_char);
    x = 3 as libc::c_int;
    while x < xstart {
        y = 3 as libc::c_int;
        while y < ystart {
            let mut c_ptr: *mut cave_type =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type;
            if (*c_ptr).info as libc::c_int & 0x400 as libc::c_int != 0 {
                let mut r_idx_0: libc::c_int = 0;
                cave_set_feat(y, x, 0x59 as libc::c_int);
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int & !(0x400 as libc::c_int))
                        as u16b;
                r_idx_0 =
                    if Rand_div(2 as libc::c_int) == 0 as libc::c_int {
                        test_monster_name(b"Forest troll\x00" as *const u8 as
                                              *const libc::c_char)
                    } else {
                        test_monster_name(b"Stone troll\x00" as *const u8 as
                                              *const libc::c_char)
                    };
                place_monster_one(y, x, r_idx_0, 0 as libc::c_int,
                                  0 as libc::c_int as bool_,
                                  -(2 as libc::c_int));
            }
            y += 1
        }
        x += 1
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_troll_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    if (*quest.offset(8 as libc::c_int as isize)).status as libc::c_int >=
           1 as libc::c_int &&
           ((*quest.offset(8 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(0 as libc::c_int,
                 Some(quest_troll_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"troll_death\x00" as *const u8 as *const libc::c_char);
        add_hook(2 as libc::c_int,
                 Some(quest_troll_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"troll_gen\x00" as *const u8 as *const libc::c_char);
        add_hook(9 as libc::c_int,
                 Some(quest_troll_finish_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"troll_finish\x00" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_wight_gen_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut xstart: libc::c_int = 2 as libc::c_int;
    let mut ystart: libc::c_int = 2 as libc::c_int;
    if (*p_ptr).inside_quest as libc::c_int != 9 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            cave_set_feat(y, x, 0x3f as libc::c_int);
            x += 1
        }
        y += 1
    }
    dun_level = (*quest.offset((*p_ptr).inside_quest as isize)).level;
    set_mon_num_hook();
    get_mon_num_prep();
    init_flags = 0x4 as libc::c_int;
    process_dungeon_file(b"wights.map\x00" as *const u8 as
                             *const libc::c_char, &mut ystart, &mut xstart,
                         cur_hgt as libc::c_int, cur_wid as libc::c_int,
                         1 as libc::c_int as bool_,
                         1 as libc::c_int as bool_);
    x = 3 as libc::c_int;
    while x < xstart {
        y = 3 as libc::c_int;
        while y < ystart {
            if (*cave[y as usize].offset(x as isize)).feat as libc::c_int ==
                   0xac as libc::c_int {
                let mut m_idx: libc::c_int = 0 as libc::c_int;
                *m_allow_special.offset(test_monster_name(b"The Wight-King of the Barrow-downs\x00"
                                                              as *const u8 as
                                                              *const libc::c_char)
                                            as isize) =
                    1 as libc::c_int as bool_;
                m_idx =
                    place_monster_one(y, x,
                                      test_monster_name(b"The Wight-King of the Barrow-downs\x00"
                                                            as *const u8 as
                                                            *const libc::c_char),
                                      0 as libc::c_int,
                                      0 as libc::c_int as bool_,
                                      -(2 as libc::c_int)) as libc::c_int;
                *m_allow_special.offset(test_monster_name(b"The Wight-King of the Barrow-downs\x00"
                                                              as *const u8 as
                                                              *const libc::c_char)
                                            as isize) =
                    0 as libc::c_int as bool_;
                if m_idx != 0 {
                    let mut o_idx: libc::c_int = 0;
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
                    let ref mut fresh14 =
                        (*m_list.offset(m_idx as isize)).mflag;
                    *fresh14 |= 0x2 as libc::c_int;
                    object_prep(q_ptr,
                                lookup_kind(36 as libc::c_int,
                                            1 as libc::c_int) as libc::c_int);
                    (*q_ptr).art_name =
                        quark_add(b"of the Wight\x00" as *const u8 as
                                      *const libc::c_char) as u16b;
                    (*q_ptr).art_flags1 =
                        ((*q_ptr).art_flags1 as libc::c_long |
                             (0x2 as libc::c_long | 0x200 as libc::c_long)) as
                            u32b;
                    (*q_ptr).art_flags2 =
                        ((*q_ptr).art_flags2 as libc::c_long |
                             (0x1000000 as libc::c_long |
                                  0x1000 as libc::c_long |
                                  0x2000000 as libc::c_long)) as u32b;
                    (*q_ptr).art_flags3 =
                        ((*q_ptr).art_flags3 as libc::c_long |
                             (0x100000 as libc::c_long |
                                  0x200000 as libc::c_long |
                                  0x400000 as libc::c_long |
                                  0x800000 as libc::c_long |
                                  0x4000 as libc::c_long)) as u32b;
                    (*q_ptr).art_flags3 =
                        ((*q_ptr).art_flags3 as libc::c_long |
                             (0x20000000 as libc::c_long |
                                  0x40000000 as libc::c_long)) as u32b;
                    (*q_ptr).ident =
                        ((*q_ptr).ident as libc::c_int | 0x40 as libc::c_int)
                            as byte_hack;
                    if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                        (*q_ptr).art_flags1 =
                            ((*q_ptr).art_flags1 as libc::c_long |
                                 0x80 as libc::c_long) as u32b;
                        (*q_ptr).pval = 6 as libc::c_int
                    } else {
                        (*q_ptr).art_flags1 =
                            ((*q_ptr).art_flags1 as libc::c_long |
                                 0x40 as libc::c_long) as u32b;
                        (*q_ptr).pval = 2 as libc::c_int
                    }
                    o_idx = o_pop() as libc::c_int;
                    if o_idx != 0 {
                        let mut o_ptr: *mut object_type =
                            &mut *o_list.offset(o_idx as isize) as
                                *mut object_type;
                        object_copy(o_ptr, q_ptr);
                        (*o_ptr).next_o_idx =
                            (*m_list.offset(m_idx as isize)).hold_o_idx;
                        (*o_ptr).held_m_idx = m_idx as s16b;
                        (*o_ptr).ix = 0 as libc::c_int as byte_hack;
                        (*o_ptr).iy = 0 as libc::c_int as byte_hack;
                        (*m_list.offset(m_idx as isize)).hold_o_idx =
                            o_idx as s16b
                    }
                }
            }
            y += 1
        }
        x += 1
    }
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_wight_death_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut r_idx: s32b = 0;
    let mut m_idx: s32b = 0;
    m_idx = get_next_arg(fmt);
    r_idx = (*m_list.offset(m_idx as isize)).r_idx as s32b;
    if (*p_ptr).inside_quest as libc::c_int != 9 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if r_idx ==
           test_monster_name(b"The Wight-King of the Barrow-downs\x00" as
                                 *const u8 as *const libc::c_char) {
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"Without their King the wights won\'t be able to do much.\x00"
                       as *const u8 as *const libc::c_char);
        cave_set_feat((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                      0x6 as libc::c_int);
        (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).special =
            0 as libc::c_int as s16b;
        (*quest.offset(9 as libc::c_int as isize)).status =
            2 as libc::c_int as s16b;
        del_hook(0 as libc::c_int,
                 Some(quest_wight_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        process_hooks_restart = 1 as libc::c_int;
        return 0 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_wight_finish_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut q_idx: s32b = 0;
    q_idx = get_next_arg(fmt);
    if q_idx != 9 as libc::c_int { return 0 as libc::c_int as bool_ }
    c_put_str(11 as libc::c_int as byte_hack,
              b"I heard about your noble deeds.\x00" as *const u8 as
                  *const libc::c_char, 8 as libc::c_int, 0 as libc::c_int);
    c_put_str(11 as libc::c_int as byte_hack,
              b"Keep what you found ..  may it serve you well.\x00" as
                  *const u8 as *const libc::c_char, 9 as libc::c_int,
              0 as libc::c_int);
    *(*quest.offset(q_idx as isize)).plot = 7 as libc::c_int as s16b;
    (*quest.offset(*(*quest.offset(q_idx as isize)).plot as
                       isize)).init.expect("non-null function pointer")(*(*quest.offset(q_idx
                                                                                            as
                                                                                            isize)).plot
                                                                            as
                                                                            libc::c_int);
    del_hook(9 as libc::c_int,
             Some(quest_wight_finish_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_));
    process_hooks_restart = 1 as libc::c_int;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_wight_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    if (*quest.offset(9 as libc::c_int as isize)).status as libc::c_int >=
           1 as libc::c_int &&
           ((*quest.offset(9 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(0 as libc::c_int,
                 Some(quest_wight_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"wight_death\x00" as *const u8 as *const libc::c_char);
        add_hook(2 as libc::c_int,
                 Some(quest_wight_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"wight_gen\x00" as *const u8 as *const libc::c_char);
        add_hook(9 as libc::c_int,
                 Some(quest_wight_finish_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"wight_finish\x00" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_nazgul_gen_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut m_idx: libc::c_int = 0;
    let mut x: libc::c_int = 1 as libc::c_int;
    let mut y: libc::c_int = 1 as libc::c_int;
    let mut tries: libc::c_int = 10000 as libc::c_int;
    let mut small: s32b = 0;
    small = get_next_arg(fmt);
    if (*quest.offset(7 as libc::c_int as isize)).status as libc::c_int !=
           1 as libc::c_int || small != 0 ||
           (*p_ptr).town_num as libc::c_int != 1 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    while tries != 0 {
        y =
            Rand_div(cur_hgt as libc::c_int - 4 as libc::c_int) +
                1 as libc::c_int + 2 as libc::c_int;
        x =
            Rand_div(cur_wid as libc::c_int - 4 as libc::c_int) +
                1 as libc::c_int + 2 as libc::c_int;
        if los((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int, y, x)
               == 0 &&
               ((*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                    isize)).flags1 as libc::c_long &
                    0x10 as libc::c_long != 0 &&
                    (*cave[y as usize].offset(x as isize)).feat as libc::c_int
                        != 0xaf as libc::c_int &&
                    (*cave[y as usize].offset(x as isize)).m_idx == 0 &&
                    !(y == (*p_ptr).py as libc::c_int &&
                          x == (*p_ptr).px as libc::c_int)) {
            break ;
        }
        tries -= 1
    }
    *m_allow_special.offset(test_monster_name(b"Uvatha the Horseman\x00" as
                                                  *const u8 as
                                                  *const libc::c_char) as
                                isize) = 1 as libc::c_int as bool_;
    m_idx =
        place_monster_one(y, x,
                          test_monster_name(b"Uvatha the Horseman\x00" as
                                                *const u8 as
                                                *const libc::c_char),
                          0 as libc::c_int, 0 as libc::c_int as bool_,
                          -(2 as libc::c_int)) as libc::c_int;
    if m_idx != 0 {
        let ref mut fresh15 = (*m_list.offset(m_idx as isize)).mflag;
        *fresh15 |= 0x2 as libc::c_int
    }
    *m_allow_special.offset(test_monster_name(b"Uvatha the Horseman\x00" as
                                                  *const u8 as
                                                  *const libc::c_char) as
                                isize) = 0 as libc::c_int as bool_;
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_nazgul_finish_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
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
    let mut q_idx: s32b = 0;
    q_idx = get_next_arg(fmt);
    if q_idx != 7 as libc::c_int { return 0 as libc::c_int as bool_ }
    c_put_str(11 as libc::c_int as byte_hack,
              b"I believe he will not come back! Thank you.\x00" as *const u8
                  as *const libc::c_char, 8 as libc::c_int, 0 as libc::c_int);
    c_put_str(11 as libc::c_int as byte_hack,
              b"Some time ago a ranger gave me this.\x00" as *const u8 as
                  *const libc::c_char, 9 as libc::c_int, 0 as libc::c_int);
    c_put_str(11 as libc::c_int as byte_hack,
              b"I believe it will help you on your quest.\x00" as *const u8 as
                  *const libc::c_char, 10 as libc::c_int, 0 as libc::c_int);
    q_ptr = &mut forge;
    object_prep(q_ptr,
                lookup_kind(80 as libc::c_int, 40 as libc::c_int) as
                    libc::c_int);
    (*q_ptr).found = 6 as libc::c_int as byte_hack;
    (*q_ptr).number = 6 as libc::c_int as byte_hack;
    object_aware(q_ptr);
    object_known(q_ptr);
    (*q_ptr).ident =
        ((*q_ptr).ident as libc::c_int | 0x10 as libc::c_int) as byte_hack;
    inven_carry(q_ptr, 0 as libc::c_int as bool_);
    *(*quest.offset(q_idx as isize)).plot = 0 as libc::c_int as s16b;
    del_hook(9 as libc::c_int,
             Some(quest_nazgul_finish_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_));
    process_hooks_restart = 1 as libc::c_int;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_nazgul_dump_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    if (*quest.offset(7 as libc::c_int as isize)).status as libc::c_int >=
           2 as libc::c_int {
        fprintf(hook_file,
                b"\n You saved Bree from a dreadful Nazgul.\x00" as *const u8
                    as *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_nazgul_forbid_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut q_idx: s32b = 0;
    q_idx = get_next_arg(fmt);
    if q_idx != 7 as libc::c_int { return 0 as libc::c_int as bool_ }
    if ((*p_ptr).lev as libc::c_int) < 30 as libc::c_int {
        c_put_str(1 as libc::c_int as byte_hack,
                  b"I fear you are not ready for the next quest, come back later.\x00"
                      as *const u8 as *const libc::c_char, 8 as libc::c_int,
                  0 as libc::c_int);
        return 1 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_nazgul_death_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut r_idx: s32b = 0;
    let mut m_idx: s32b = 0;
    m_idx = get_next_arg(fmt);
    r_idx = (*m_list.offset(m_idx as isize)).r_idx as s32b;
    if (*quest.offset(7 as libc::c_int as isize)).status as libc::c_int !=
           1 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if r_idx !=
           test_monster_name(b"Uvatha the Horseman\x00" as *const u8 as
                                 *const libc::c_char) {
        return 0 as libc::c_int as bool_
    }
    (*quest.offset(7 as libc::c_int as isize)).status =
        2 as libc::c_int as s16b;
    del_hook(0 as libc::c_int,
             Some(quest_nazgul_death_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_));
    process_hooks_restart = 1 as libc::c_int;
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_nazgul_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    if (*quest.offset(7 as libc::c_int as isize)).status as libc::c_int >=
           1 as libc::c_int &&
           ((*quest.offset(7 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(0 as libc::c_int,
                 Some(quest_nazgul_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"nazgul_death\x00" as *const u8 as *const libc::c_char);
        add_hook(14 as libc::c_int,
                 Some(quest_nazgul_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"nazgul_gen\x00" as *const u8 as *const libc::c_char);
        add_hook(9 as libc::c_int,
                 Some(quest_nazgul_finish_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"nazgul_finish\x00" as *const u8 as *const libc::c_char);
    }
    add_hook(12 as libc::c_int,
             Some(quest_nazgul_dump_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"nazgul_dump\x00" as *const u8 as *const libc::c_char);
    add_hook(13 as libc::c_int,
             Some(quest_nazgul_forbid_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"nazgul_forbid\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_shroom_town_gen_hook(mut fmt:
                                                        *mut libc::c_char)
 -> bool_ {
    let mut m_idx: libc::c_int = 0;
    let mut x: libc::c_int = 1 as libc::c_int;
    let mut y: libc::c_int = 1 as libc::c_int;
    let mut tries: libc::c_int = 10000 as libc::c_int;
    let mut small: s32b = 0;
    small = get_next_arg(fmt);
    if small == 0 && (*p_ptr).wilderness_y == 21 as libc::c_int &&
           (*p_ptr).wilderness_x == 33 as libc::c_int {
        x = cur_wid as libc::c_int / 2 as libc::c_int - 7 as libc::c_int;
        while x <=
                  cur_wid as libc::c_int / 2 as libc::c_int + 7 as libc::c_int
              {
            y = cur_hgt as libc::c_int / 2 as libc::c_int - 5 as libc::c_int;
            while y <=
                      cur_hgt as libc::c_int / 2 as libc::c_int +
                          5 as libc::c_int {
                cave_set_feat(y, x, 181 as libc::c_int);
                y += 1
            }
            x += 1
        }
        x = 0 as libc::c_int;
        while x <
                  (*quest.offset(18 as libc::c_int as
                                     isize)).data[1 as libc::c_int as usize] -
                      (*quest.offset(18 as libc::c_int as
                                         isize)).data[0 as libc::c_int as
                                                          usize] {
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
            object_prep(q_ptr,
                        lookup_kind(80 as libc::c_int,
                                    1 as libc::c_int +
                                        Rand_div(1 as libc::c_int +
                                                     18 as libc::c_int -
                                                     1 as libc::c_int)) as
                            libc::c_int);
            (*q_ptr).number = 1 as libc::c_int as byte_hack;
            (*q_ptr).pval2 = 1 as libc::c_int as s16b;
            drop_near(q_ptr, -(1 as libc::c_int),
                      cur_hgt as libc::c_int / 2 as libc::c_int -
                          5 as libc::c_int +
                          Rand_div(1 as libc::c_int +
                                       (cur_hgt as libc::c_int /
                                            2 as libc::c_int +
                                            5 as libc::c_int) -
                                       (cur_hgt as libc::c_int /
                                            2 as libc::c_int -
                                            5 as libc::c_int)),
                      cur_wid as libc::c_int / 2 as libc::c_int -
                          7 as libc::c_int +
                          Rand_div(1 as libc::c_int +
                                       (cur_wid as libc::c_int /
                                            2 as libc::c_int +
                                            7 as libc::c_int) -
                                       (cur_wid as libc::c_int /
                                            2 as libc::c_int -
                                            7 as libc::c_int)));
            x += 1
        }
        y =
            cur_hgt as libc::c_int / 2 as libc::c_int - 5 as libc::c_int +
                Rand_div(1 as libc::c_int +
                             (cur_hgt as libc::c_int / 2 as libc::c_int +
                                  5 as libc::c_int) -
                             (cur_hgt as libc::c_int / 2 as libc::c_int -
                                  5 as libc::c_int));
        x =
            cur_wid as libc::c_int / 2 as libc::c_int - 7 as libc::c_int +
                Rand_div(1 as libc::c_int +
                             (cur_wid as libc::c_int / 2 as libc::c_int +
                                  7 as libc::c_int) -
                             (cur_wid as libc::c_int / 2 as libc::c_int -
                                  7 as libc::c_int));
        *m_allow_special.offset(test_monster_name(b"Grip, Farmer Maggot\'s dog\x00"
                                                      as *const u8 as
                                                      *const libc::c_char) as
                                    isize) = 1 as libc::c_int as bool_;
        m_idx =
            place_monster_one(y, x,
                              test_monster_name(b"Grip, Farmer Maggot\'s dog\x00"
                                                    as *const u8 as
                                                    *const libc::c_char),
                              0 as libc::c_int, 0 as libc::c_int as bool_,
                              -(2 as libc::c_int)) as libc::c_int;
        if m_idx != 0 {
            let ref mut fresh16 = (*m_list.offset(m_idx as isize)).mflag;
            *fresh16 |= 0x2 as libc::c_int
        }
        *m_allow_special.offset(test_monster_name(b"Grip, Farmer Maggot\'s dog\x00"
                                                      as *const u8 as
                                                      *const libc::c_char) as
                                    isize) = 0 as libc::c_int as bool_;
        y =
            cur_hgt as libc::c_int / 2 as libc::c_int - 5 as libc::c_int +
                Rand_div(1 as libc::c_int +
                             (cur_hgt as libc::c_int / 2 as libc::c_int +
                                  5 as libc::c_int) -
                             (cur_hgt as libc::c_int / 2 as libc::c_int -
                                  5 as libc::c_int));
        x =
            cur_wid as libc::c_int / 2 as libc::c_int - 7 as libc::c_int +
                Rand_div(1 as libc::c_int +
                             (cur_wid as libc::c_int / 2 as libc::c_int +
                                  7 as libc::c_int) -
                             (cur_wid as libc::c_int / 2 as libc::c_int -
                                  7 as libc::c_int));
        *m_allow_special.offset(test_monster_name(b"Wolf, Farmer Maggot\'s dog\x00"
                                                      as *const u8 as
                                                      *const libc::c_char) as
                                    isize) = 1 as libc::c_int as bool_;
        m_idx =
            place_monster_one(y, x,
                              test_monster_name(b"Wolf, Farmer Maggot\'s dog\x00"
                                                    as *const u8 as
                                                    *const libc::c_char),
                              0 as libc::c_int, 0 as libc::c_int as bool_,
                              -(2 as libc::c_int)) as libc::c_int;
        if m_idx != 0 {
            let ref mut fresh17 = (*m_list.offset(m_idx as isize)).mflag;
            *fresh17 |= 0x2 as libc::c_int
        }
        *m_allow_special.offset(test_monster_name(b"Wolf, Farmer Maggot\'s dog\x00"
                                                      as *const u8 as
                                                      *const libc::c_char) as
                                    isize) = 0 as libc::c_int as bool_;
        y =
            cur_hgt as libc::c_int / 2 as libc::c_int - 5 as libc::c_int +
                Rand_div(1 as libc::c_int +
                             (cur_hgt as libc::c_int / 2 as libc::c_int +
                                  5 as libc::c_int) -
                             (cur_hgt as libc::c_int / 2 as libc::c_int -
                                  5 as libc::c_int));
        x =
            cur_wid as libc::c_int / 2 as libc::c_int - 7 as libc::c_int +
                Rand_div(1 as libc::c_int +
                             (cur_wid as libc::c_int / 2 as libc::c_int +
                                  7 as libc::c_int) -
                             (cur_wid as libc::c_int / 2 as libc::c_int -
                                  7 as libc::c_int));
        *m_allow_special.offset(test_monster_name(b"Fang, Farmer Maggot\'s dog\x00"
                                                      as *const u8 as
                                                      *const libc::c_char) as
                                    isize) = 1 as libc::c_int as bool_;
        m_idx =
            place_monster_one(y, x,
                              test_monster_name(b"Fang, Farmer Maggot\'s dog\x00"
                                                    as *const u8 as
                                                    *const libc::c_char),
                              0 as libc::c_int, 0 as libc::c_int as bool_,
                              -(2 as libc::c_int)) as libc::c_int;
        if m_idx != 0 {
            let ref mut fresh18 = (*m_list.offset(m_idx as isize)).mflag;
            *fresh18 |= 0x2 as libc::c_int
        }
        *m_allow_special.offset(test_monster_name(b"Fang, Farmer Maggot\'s dog\x00"
                                                      as *const u8 as
                                                      *const libc::c_char) as
                                    isize) = 0 as libc::c_int as bool_;
        msg_print(b"You hear frenzied barking.\x00" as *const u8 as
                      *const libc::c_char);
    }
    if bst(11520 as libc::c_int / 24 as libc::c_int, turn) < 6 as libc::c_int
           ||
           bst(11520 as libc::c_int / 24 as libc::c_int, turn) >=
               18 as libc::c_int ||
           (*quest.offset(18 as libc::c_int as isize)).status as libc::c_int >
               2 as libc::c_int || small != 0 ||
           (*p_ptr).town_num as libc::c_int != 1 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    while tries != 0 {
        y =
            Rand_div(20 as libc::c_int) + 1 as libc::c_int +
                cur_hgt as libc::c_int / 2 as libc::c_int - 10 as libc::c_int;
        x =
            Rand_div(20 as libc::c_int) + 1 as libc::c_int +
                cur_wid as libc::c_int / 2 as libc::c_int - 10 as libc::c_int;
        if los((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int, y, x)
               == 0 &&
               ((*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                    isize)).flags1 as libc::c_long &
                    0x10 as libc::c_long != 0 &&
                    (*cave[y as usize].offset(x as isize)).feat as libc::c_int
                        != 0xaf as libc::c_int &&
                    (*cave[y as usize].offset(x as isize)).m_idx == 0 &&
                    !(y == (*p_ptr).py as libc::c_int &&
                          x == (*p_ptr).px as libc::c_int)) &&
               ((*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                    isize)).flags1 as libc::c_long &
                    0x10 as libc::c_long != 0 &&
                    (*f_info.offset((*cave[y as
                                               usize].offset(x as isize)).feat
                                        as isize)).flags1 as libc::c_long &
                        0x100 as libc::c_long == 0) {
            break ;
        }
        tries -= 1
    }
    *m_allow_special.offset(test_monster_name(b"Farmer Maggot\x00" as
                                                  *const u8 as
                                                  *const libc::c_char) as
                                isize) = 1 as libc::c_int as bool_;
    place_monster_one(y, x,
                      test_monster_name(b"Farmer Maggot\x00" as *const u8 as
                                            *const libc::c_char),
                      0 as libc::c_int, 0 as libc::c_int as bool_,
                      -(2 as libc::c_int));
    *m_allow_special.offset(test_monster_name(b"Farmer Maggot\x00" as
                                                  *const u8 as
                                                  *const libc::c_char) as
                                isize) = 0 as libc::c_int as bool_;
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_shroom_death_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut r_idx: s32b = 0;
    let mut m_idx: s32b = 0;
    m_idx = get_next_arg(fmt);
    r_idx = (*m_list.offset(m_idx as isize)).r_idx as s32b;
    if (*quest.offset(18 as libc::c_int as isize)).status as libc::c_int >
           2 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if r_idx ==
           test_monster_name(b"Wolf, Farmer Maggot\'s dog\x00" as *const u8 as
                                 *const libc::c_char) ||
           r_idx ==
               test_monster_name(b"Grip, Farmer Maggot\'s dog\x00" as
                                     *const u8 as *const libc::c_char) ||
           r_idx ==
               test_monster_name(b"Fang, Farmer Maggot\'s dog\x00" as
                                     *const u8 as *const libc::c_char) {
        msg_print(b"The dog yells a last time and drops dead on the grass.\x00"
                      as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_shroom_give_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    let mut m_idx: s32b = 0;
    let mut item: s32b = 0;
    m_idx = get_next_arg(fmt);
    item = get_next_arg(fmt);
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(item as isize) as
            *mut object_type;
    m_ptr = &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    if (*m_ptr).r_idx as libc::c_int !=
           test_monster_name(b"Farmer Maggot\x00" as *const u8 as
                                 *const libc::c_char) {
        return 0 as libc::c_int as bool_
    }
    if (*r_info.offset(test_monster_name(b"Grip, Farmer Maggot\'s dog\x00" as
                                             *const u8 as *const libc::c_char)
                           as isize)).max_num as libc::c_int ==
           0 as libc::c_int ||
           (*r_info.offset(test_monster_name(b"Wolf, Farmer Maggot\'s dog\x00"
                                                 as *const u8 as
                                                 *const libc::c_char) as
                               isize)).max_num as libc::c_int ==
               0 as libc::c_int ||
           (*r_info.offset(test_monster_name(b"Fang, Farmer Maggot\'s dog\x00"
                                                 as *const u8 as
                                                 *const libc::c_char) as
                               isize)).max_num as libc::c_int ==
               0 as libc::c_int {
        (*quest.offset(18 as libc::c_int as isize)).status =
            6 as libc::c_int as s16b;
        msg_print(b"My puppy!  My poor, defenceless puppy...\x00" as *const u8
                      as *const libc::c_char);
        msg_print(b"YOU MURDERER!  Out of my sight!\x00" as *const u8 as
                      *const libc::c_char);
        delete_monster_idx(m_idx);
        del_hook(11 as libc::c_int,
                 Some(quest_shroom_give_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        del_hook(32 as libc::c_int,
                 Some(quest_shroom_speak_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        del_hook(14 as libc::c_int,
                 Some(quest_shroom_town_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        process_hooks_restart = 1 as libc::c_int;
        return 1 as libc::c_int as bool_
    }
    if (*o_ptr).tval as libc::c_int != 80 as libc::c_int ||
           (*o_ptr).pval2 as libc::c_int != 1 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    inc_stack_size_ex(item, -(1 as libc::c_int), OPTIMIZE, NO_DESCRIBE);
    let ref mut fresh19 =
        (*quest.offset(18 as libc::c_int as
                           isize)).data[0 as libc::c_int as usize];
    *fresh19 += 1;
    if (*quest.offset(18 as libc::c_int as
                          isize)).data[0 as libc::c_int as usize] ==
           (*quest.offset(18 as libc::c_int as
                              isize)).data[1 as libc::c_int as usize] {
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
        msg_print(b"Oh thank you!\x00" as *const u8 as *const libc::c_char);
        msg_print(b"Take my sling and those mushrooms, may they help you!\x00"
                      as *const u8 as *const libc::c_char);
        msg_print(b"Farmer Maggot heads back to his house.\x00" as *const u8
                      as *const libc::c_char);
        q_ptr = &mut forge;
        object_prep(q_ptr,
                    lookup_kind(80 as libc::c_int, 16 as libc::c_int) as
                        libc::c_int);
        (*q_ptr).found = 6 as libc::c_int as byte_hack;
        (*q_ptr).number =
            (15 as libc::c_int +
                 Rand_div(1 as libc::c_int + 20 as libc::c_int -
                              15 as libc::c_int)) as byte_hack;
        object_aware(q_ptr);
        object_known(q_ptr);
        (*q_ptr).discount = 100 as libc::c_int as byte_hack;
        (*q_ptr).ident =
            ((*q_ptr).ident as libc::c_int | 0x10 as libc::c_int) as
                byte_hack;
        if inven_carry_okay(q_ptr) != 0 {
            inven_carry(q_ptr, 0 as libc::c_int as bool_);
        } else {
            drop_near(q_ptr, 0 as libc::c_int, (*p_ptr).py as libc::c_int,
                      (*p_ptr).px as libc::c_int);
        }
        q_ptr = &mut forge;
        object_prep(q_ptr,
                    lookup_kind(19 as libc::c_int, 2 as libc::c_int) as
                        libc::c_int);
        (*q_ptr).found = 6 as libc::c_int as byte_hack;
        (*q_ptr).number = 1 as libc::c_int as byte_hack;
        (*q_ptr).name1 = 149 as libc::c_int as byte_hack;
        apply_magic(q_ptr, -(1 as libc::c_int), 1 as libc::c_int as bool_,
                    1 as libc::c_int as bool_, 1 as libc::c_int as bool_);
        object_aware(q_ptr);
        object_known(q_ptr);
        (*q_ptr).discount = 100 as libc::c_int as byte_hack;
        (*q_ptr).ident =
            ((*q_ptr).ident as libc::c_int | 0x10 as libc::c_int) as
                byte_hack;
        inven_carry(q_ptr, 0 as libc::c_int as bool_);
        delete_monster_idx(m_idx);
        (*quest.offset(18 as libc::c_int as isize)).status =
            5 as libc::c_int as s16b;
        del_hook(11 as libc::c_int,
                 Some(quest_shroom_give_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        process_hooks_restart = 1 as libc::c_int
    } else {
        msg_format(b"Oh thank you, but you still have %d mushrooms to bring back!\x00"
                       as *const u8 as *const libc::c_char,
                   (*quest.offset(18 as libc::c_int as
                                      isize)).data[1 as libc::c_int as usize]
                       -
                       (*quest.offset(18 as libc::c_int as
                                          isize)).data[0 as libc::c_int as
                                                           usize]);
    }
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_shroom_speak_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut m_idx: s32b = get_next_arg(fmt);
    if (*m_list.offset(m_idx as isize)).r_idx as libc::c_int !=
           test_monster_name(b"Farmer Maggot\x00" as *const u8 as
                                 *const libc::c_char) {
        return 0 as libc::c_int as bool_
    }
    if (*quest.offset(18 as libc::c_int as isize)).status as libc::c_int ==
           0 as libc::c_int {
        let mut m_name: cptr = 0 as *const libc::c_char;
        m_name = get_next_arg_str(fmt) as cptr;
        msg_format(b"%^s asks your help.\x00" as *const u8 as
                       *const libc::c_char, m_name);
        exec_lua(b"ingame_help(\'monster_chat\')\x00" as *const u8 as
                     *const libc::c_char as *mut libc::c_char);
    } else {
        if (*r_info.offset(test_monster_name(b"Grip, Farmer Maggot\'s dog\x00"
                                                 as *const u8 as
                                                 *const libc::c_char) as
                               isize)).max_num as libc::c_int ==
               0 as libc::c_int ||
               (*r_info.offset(test_monster_name(b"Wolf, Farmer Maggot\'s dog\x00"
                                                     as *const u8 as
                                                     *const libc::c_char) as
                                   isize)).max_num as libc::c_int ==
                   0 as libc::c_int ||
               (*r_info.offset(test_monster_name(b"Fang, Farmer Maggot\'s dog\x00"
                                                     as *const u8 as
                                                     *const libc::c_char) as
                                   isize)).max_num as libc::c_int ==
                   0 as libc::c_int {
            (*quest.offset(18 as libc::c_int as isize)).status =
                6 as libc::c_int as s16b;
            msg_print(b"My puppy!  My poor, defenceless puppy...\x00" as
                          *const u8 as *const libc::c_char);
            msg_print(b"YOU MURDERER!  Out of my sight!\x00" as *const u8 as
                          *const libc::c_char);
            delete_monster_idx(m_idx);
            del_hook(11 as libc::c_int,
                     Some(quest_shroom_give_hook as
                              unsafe extern "C" fn(_: *mut libc::c_char)
                                  -> bool_));
            del_hook(32 as libc::c_int,
                     Some(quest_shroom_speak_hook as
                              unsafe extern "C" fn(_: *mut libc::c_char)
                                  -> bool_));
            del_hook(14 as libc::c_int,
                     Some(quest_shroom_town_gen_hook as
                              unsafe extern "C" fn(_: *mut libc::c_char)
                                  -> bool_));
            process_hooks_restart = 1 as libc::c_int;
            return 1 as libc::c_int as bool_
        }
        msg_format(b"You still have %d mushrooms to bring back!\x00" as
                       *const u8 as *const libc::c_char,
                   (*quest.offset(18 as libc::c_int as
                                      isize)).data[1 as libc::c_int as usize]
                       -
                       (*quest.offset(18 as libc::c_int as
                                          isize)).data[0 as libc::c_int as
                                                           usize]);
    }
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_shroom_chat_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    let mut m_idx: s32b = 0;
    m_idx = get_next_arg(fmt);
    m_ptr = &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    if (*m_ptr).r_idx as libc::c_int !=
           test_monster_name(b"Farmer Maggot\x00" as *const u8 as
                                 *const libc::c_char) {
        return 0 as libc::c_int as bool_
    }
    if (*quest.offset(18 as libc::c_int as isize)).status as libc::c_int ==
           0 as libc::c_int {
        msg_print(b"My mushrooms, my mushrooms!\x00" as *const u8 as
                      *const libc::c_char);
        msg_print(b"The rain, a dark horrible rain, began so I had to return to my home.\x00"
                      as *const u8 as *const libc::c_char);
        msg_print(b"But when I came back my dogs were all mad and didn\'t let me near the field.\x00"
                      as *const u8 as *const libc::c_char);
        msg_print(b"Could you please bring me back all the mushrooms that have grown in my field\x00"
                      as *const u8 as *const libc::c_char);
        msg_print(b"to the west of Bree? Please try to not harm my dogs. They are so lovely...\x00"
                      as *const u8 as *const libc::c_char);
        (*quest.offset(18 as libc::c_int as isize)).status =
            1 as libc::c_int as s16b;
        (*quest.offset(18 as libc::c_int as
                           isize)).init.expect("non-null function pointer")(18
                                                                                as
                                                                                libc::c_int);
    } else {
        if (*r_info.offset(test_monster_name(b"Grip, Farmer Maggot\'s dog\x00"
                                                 as *const u8 as
                                                 *const libc::c_char) as
                               isize)).max_num as libc::c_int ==
               0 as libc::c_int ||
               (*r_info.offset(test_monster_name(b"Wolf, Farmer Maggot\'s dog\x00"
                                                     as *const u8 as
                                                     *const libc::c_char) as
                                   isize)).max_num as libc::c_int ==
                   0 as libc::c_int ||
               (*r_info.offset(test_monster_name(b"Fang, Farmer Maggot\'s dog\x00"
                                                     as *const u8 as
                                                     *const libc::c_char) as
                                   isize)).max_num as libc::c_int ==
                   0 as libc::c_int {
            (*quest.offset(18 as libc::c_int as isize)).status =
                6 as libc::c_int as s16b;
            msg_print(b"My puppy!  My poor, defenceless puppy...\x00" as
                          *const u8 as *const libc::c_char);
            msg_print(b"YOU MURDERER!  Out of my sight!\x00" as *const u8 as
                          *const libc::c_char);
            delete_monster_idx(m_idx);
            del_hook(11 as libc::c_int,
                     Some(quest_shroom_give_hook as
                              unsafe extern "C" fn(_: *mut libc::c_char)
                                  -> bool_));
            del_hook(32 as libc::c_int,
                     Some(quest_shroom_speak_hook as
                              unsafe extern "C" fn(_: *mut libc::c_char)
                                  -> bool_));
            del_hook(14 as libc::c_int,
                     Some(quest_shroom_town_gen_hook as
                              unsafe extern "C" fn(_: *mut libc::c_char)
                                  -> bool_));
            process_hooks_restart = 1 as libc::c_int;
            return 1 as libc::c_int as bool_
        }
        msg_format(b"You still have %d mushrooms to bring back!\x00" as
                       *const u8 as *const libc::c_char,
                   (*quest.offset(18 as libc::c_int as
                                      isize)).data[1 as libc::c_int as usize]
                       -
                       (*quest.offset(18 as libc::c_int as
                                          isize)).data[0 as libc::c_int as
                                                           usize]);
    }
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_shroom_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    if (*quest.offset(18 as libc::c_int as
                          isize)).data[1 as libc::c_int as usize] == 0 {
        (*quest.offset(18 as libc::c_int as
                           isize)).data[0 as libc::c_int as usize] =
            0 as libc::c_int;
        (*quest.offset(18 as libc::c_int as
                           isize)).data[1 as libc::c_int as usize] =
            7 as libc::c_int +
                Rand_div(1 as libc::c_int + 14 as libc::c_int -
                             7 as libc::c_int);
        if wizard != 0 {
            message_add(1 as libc::c_int as byte_hack,
                        format(b"Shrooms number %d\x00" as *const u8 as
                                   *const libc::c_char,
                               (*quest.offset(18 as libc::c_int as
                                                  isize)).data[1 as
                                                                   libc::c_int
                                                                   as usize])
                            as cptr, 6 as libc::c_int as byte_hack);
        }
    }
    if (*quest.offset(18 as libc::c_int as isize)).status as libc::c_int >=
           1 as libc::c_int &&
           ((*quest.offset(18 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(0 as libc::c_int,
                 Some(quest_shroom_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"shroom_death\x00" as *const u8 as *const libc::c_char);
        add_hook(11 as libc::c_int,
                 Some(quest_shroom_give_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"shroom_give\x00" as *const u8 as *const libc::c_char);
        add_hook(14 as libc::c_int,
                 Some(quest_shroom_town_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"shroom_town_gen\x00" as *const u8 as *const libc::c_char);
        add_hook(32 as libc::c_int,
                 Some(quest_shroom_chat_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"shroom_chat\x00" as *const u8 as *const libc::c_char);
        add_hook(33 as libc::c_int,
                 Some(quest_shroom_speak_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"shroom_speak\x00" as *const u8 as *const libc::c_char);
    }
    if (*quest.offset(18 as libc::c_int as isize)).status as libc::c_int ==
           0 as libc::c_int {
        add_hook(33 as libc::c_int,
                 Some(quest_shroom_speak_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"shroom_speak\x00" as *const u8 as *const libc::c_char);
        add_hook(14 as libc::c_int,
                 Some(quest_shroom_town_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"shroom_town_gen\x00" as *const u8 as *const libc::c_char);
        add_hook(32 as libc::c_int,
                 Some(quest_shroom_chat_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"shroom_chat\x00" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_wolves_gen_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut xstart: libc::c_int = 2 as libc::c_int;
    let mut ystart: libc::c_int = 2 as libc::c_int;
    if (*p_ptr).inside_quest as libc::c_int != 22 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*quest.offset(22 as libc::c_int as isize)).status as libc::c_int ==
           0 as libc::c_int {
        (*quest.offset(22 as libc::c_int as isize)).status =
            1 as libc::c_int as s16b
    }
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            cave_set_feat(y, x, 0x3f as libc::c_int);
            x += 1
        }
        y += 1
    }
    dun_level = (*quest.offset((*p_ptr).inside_quest as isize)).level;
    set_mon_num_hook();
    get_mon_num_prep();
    init_flags = 0x4 as libc::c_int;
    process_dungeon_file(b"wolves.map\x00" as *const u8 as
                             *const libc::c_char, &mut ystart, &mut xstart,
                         cur_hgt as libc::c_int, cur_wid as libc::c_int,
                         1 as libc::c_int as bool_,
                         0 as libc::c_int as bool_);
    dungeon_flags2 =
        (dungeon_flags2 as libc::c_long | 0x200 as libc::c_long) as u32b;
    i = damroll(4 as libc::c_int as s16b, 4 as libc::c_int as s16b);
    while i > 0 as libc::c_int {
        let mut m_idx: libc::c_int = 0;
        let mut flags: libc::c_int = 0;
        y = Rand_div(21 as libc::c_int) + 3 as libc::c_int;
        x = Rand_div(31 as libc::c_int) + 3 as libc::c_int;
        flags =
            (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                isize)).flags1 as libc::c_int;
        if flags as libc::c_long & 0x40 as libc::c_long == 0 &&
               flags as libc::c_long & 0x10 as libc::c_long != 0 {
            m_idx =
                place_monster_one(y, x, 196 as libc::c_int, 0 as libc::c_int,
                                  (Rand_div(100 as libc::c_int) <
                                       50 as libc::c_int) as libc::c_int as
                                      bool_, -(2 as libc::c_int)) as
                    libc::c_int;
            if m_idx != 0 {
                let ref mut fresh20 = (*m_list.offset(m_idx as isize)).mflag;
                *fresh20 |= 0x2 as libc::c_int
            }
            i -= 1
        }
    }
    i = damroll(4 as libc::c_int as s16b, 4 as libc::c_int as s16b);
    while i > 0 as libc::c_int {
        let mut m_idx_0: libc::c_int = 0;
        let mut flags_0: libc::c_int = 0;
        y = Rand_div(21 as libc::c_int) + 3 as libc::c_int;
        x = Rand_div(31 as libc::c_int) + 3 as libc::c_int;
        flags_0 =
            (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                isize)).flags1 as libc::c_int;
        if flags_0 as libc::c_long & 0x40 as libc::c_long == 0 &&
               flags_0 as libc::c_long & 0x10 as libc::c_long != 0 {
            m_idx_0 =
                place_monster_one(y, x, 257 as libc::c_int, 0 as libc::c_int,
                                  (Rand_div(100 as libc::c_int) <
                                       50 as libc::c_int) as libc::c_int as
                                      bool_, -(2 as libc::c_int)) as
                    libc::c_int;
            if m_idx_0 != 0 {
                let ref mut fresh21 =
                    (*m_list.offset(m_idx_0 as isize)).mflag;
                *fresh21 |= 0x2 as libc::c_int
            }
            i -= 1
        }
    }
    process_hooks_restart = 1 as libc::c_int;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_wolves_death_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut mcnt: libc::c_int = 0 as libc::c_int;
    if (*p_ptr).inside_quest as libc::c_int != 22 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    i = m_max as libc::c_int - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        if !((*m_ptr).r_idx == 0) {
            if (*m_ptr).status as libc::c_int <= -(2 as libc::c_int) {
                mcnt += 1
            }
        }
        i -= 1
    }
    if mcnt <= 1 as libc::c_int {
        (*quest.offset((*p_ptr).inside_quest as isize)).status =
            2 as libc::c_int as s16b;
        del_hook(0 as libc::c_int,
                 Some(quest_wolves_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        del_hook(2 as libc::c_int,
                 Some(quest_wolves_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        process_hooks_restart = 1 as libc::c_int;
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"Lothlorien is safer now.\x00" as *const u8 as
                       *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_wolves_finish_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut q_idx: s32b = 0;
    q_idx = get_next_arg(fmt);
    if q_idx != 22 as libc::c_int { return 0 as libc::c_int as bool_ }
    c_put_str(11 as libc::c_int as byte_hack,
              b"Thank you for killing the pack of wolves!\x00" as *const u8 as
                  *const libc::c_char, 8 as libc::c_int, 0 as libc::c_int);
    c_put_str(11 as libc::c_int as byte_hack,
              b"You can use the hut as your house as a reward.\x00" as
                  *const u8 as *const libc::c_char, 9 as libc::c_int,
              0 as libc::c_int);
    *(*quest.offset(q_idx as isize)).plot = 10 as libc::c_int as s16b;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_wolves_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    if (*quest.offset(22 as libc::c_int as isize)).status as libc::c_int >=
           0 as libc::c_int &&
           ((*quest.offset(22 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(0 as libc::c_int,
                 Some(quest_wolves_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"wolves_monster_death\x00" as *const u8 as
                     *const libc::c_char);
        add_hook(9 as libc::c_int,
                 Some(quest_wolves_finish_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"wolves_finish\x00" as *const u8 as *const libc::c_char);
        add_hook(2 as libc::c_int,
                 Some(quest_wolves_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"wolves_geb\x00" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_spider_gen_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut xstart: libc::c_int = 2 as libc::c_int;
    let mut ystart: libc::c_int = 2 as libc::c_int;
    if (*p_ptr).inside_quest as libc::c_int != 10 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            cave_set_feat(y, x, 0x3f as libc::c_int);
            x += 1
        }
        y += 1
    }
    dun_level = (*quest.offset((*p_ptr).inside_quest as isize)).level;
    set_mon_num_hook();
    get_mon_num_prep();
    init_flags = 0x4 as libc::c_int;
    process_dungeon_file(b"spiders.map\x00" as *const u8 as
                             *const libc::c_char, &mut ystart, &mut xstart,
                         cur_hgt as libc::c_int, cur_wid as libc::c_int,
                         1 as libc::c_int as bool_,
                         1 as libc::c_int as bool_);
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_spider_death_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut mcnt: libc::c_int = 0 as libc::c_int;
    if (*p_ptr).inside_quest as libc::c_int != 10 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    i = m_max as libc::c_int - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        if !((*m_ptr).r_idx == 0) {
            if (*m_ptr).status as libc::c_int <= -(2 as libc::c_int) {
                mcnt += 1
            }
        }
        i -= 1
    }
    if mcnt <= 1 as libc::c_int {
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"The forest is now safer, thanks to you.\x00" as *const u8
                       as *const libc::c_char);
        if (*p_ptr).pgod as libc::c_int == 5 as libc::c_int {
            cmsg_print(13 as libc::c_int as byte_hack,
                       b"You feel the gentle touch of Yavanna, as she smiles at you.\x00"
                           as *const u8 as *const libc::c_char);
            inc_piety(5 as libc::c_int, 6000 as libc::c_int);
        }
        (*quest.offset(10 as libc::c_int as isize)).status =
            2 as libc::c_int as s16b;
        del_hook(0 as libc::c_int,
                 Some(quest_spider_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        process_hooks_restart = 1 as libc::c_int;
        return 0 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_spider_finish_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
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
    let mut q_idx: s32b = 0;
    q_idx = get_next_arg(fmt);
    if q_idx != 10 as libc::c_int { return 0 as libc::c_int as bool_ }
    c_put_str(11 as libc::c_int as byte_hack,
              b"All of us praise your mighty deed in driving back the\x00" as
                  *const u8 as *const libc::c_char, 8 as libc::c_int,
              0 as libc::c_int);
    c_put_str(11 as libc::c_int as byte_hack,
              b"menace. Take this as a reward.\x00" as *const u8 as
                  *const libc::c_char, 9 as libc::c_int, 0 as libc::c_int);
    q_ptr = &mut forge;
    object_prep(q_ptr,
                lookup_kind(71 as libc::c_int, 55 as libc::c_int) as
                    libc::c_int);
    (*q_ptr).number = 1 as libc::c_int as byte_hack;
    (*q_ptr).found = 6 as libc::c_int as byte_hack;
    object_aware(q_ptr);
    object_known(q_ptr);
    (*q_ptr).ident =
        ((*q_ptr).ident as libc::c_int | 0x10 as libc::c_int) as byte_hack;
    inven_carry(q_ptr, 0 as libc::c_int as bool_);
    *(*quest.offset(q_idx as isize)).plot = 11 as libc::c_int as s16b;
    (*quest.offset(*(*quest.offset(q_idx as isize)).plot as
                       isize)).init.expect("non-null function pointer")(*(*quest.offset(q_idx
                                                                                            as
                                                                                            isize)).plot
                                                                            as
                                                                            libc::c_int);
    del_hook(9 as libc::c_int,
             Some(quest_spider_finish_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_));
    process_hooks_restart = 1 as libc::c_int;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_spider_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    if (*quest.offset(10 as libc::c_int as isize)).status as libc::c_int >=
           1 as libc::c_int &&
           ((*quest.offset(10 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(0 as libc::c_int,
                 Some(quest_spider_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"spider_death\x00" as *const u8 as *const libc::c_char);
        add_hook(2 as libc::c_int,
                 Some(quest_spider_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"spider_gen\x00" as *const u8 as *const libc::c_char);
        add_hook(9 as libc::c_int,
                 Some(quest_spider_finish_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"spider_finish\x00" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
static mut wild_locs: [[libc::c_int; 2]; 4] =
    [[32 as libc::c_int, 49 as libc::c_int],
     [32 as libc::c_int, 48 as libc::c_int],
     [33 as libc::c_int, 48 as libc::c_int],
     [34 as libc::c_int, 48 as libc::c_int]];
unsafe extern "C" fn create_molds_hook(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    if (*r_ptr).flags4 & 0x2 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    if (*r_ptr).d_char as libc::c_int == 'm' as i32 {
        return 1 as libc::c_int as bool_
    } else if (*r_ptr).d_char as libc::c_int == ',' as i32 {
        return 1 as libc::c_int as bool_
    } else if (*r_ptr).d_char as libc::c_int == 'e' as i32 {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
#[no_mangle]
pub unsafe extern "C" fn quest_poison_gen_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut cy: libc::c_int = 1 as libc::c_int;
    let mut cx: libc::c_int = 1 as libc::c_int;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut tries: libc::c_int = 10000 as libc::c_int;
    let mut r_idx: libc::c_int = 0;
    let mut old_get_mon_num_hook:
            Option<unsafe extern "C" fn(_: libc::c_int) -> bool_> = None;
    if (*quest.offset(11 as libc::c_int as isize)).status as libc::c_int !=
           1 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*p_ptr).wilderness_y !=
           wild_locs[(*quest.offset(11 as libc::c_int as
                                        isize)).data[0 as libc::c_int as
                                                         usize] as
                         usize][0 as libc::c_int as usize] {
        return 0 as libc::c_int as bool_
    }
    if (*p_ptr).wilderness_x !=
           wild_locs[(*quest.offset(11 as libc::c_int as
                                        isize)).data[0 as libc::c_int as
                                                         usize] as
                         usize][1 as libc::c_int as usize] {
        return 0 as libc::c_int as bool_
    }
    if (*p_ptr).wild_mode != 0 { return 0 as libc::c_int as bool_ }
    while tries != 0 {
        cy =
            Rand_div(cur_hgt as libc::c_int - 24 as libc::c_int) +
                1 as libc::c_int + 22 as libc::c_int;
        cx =
            Rand_div(cur_wid as libc::c_int - 34 as libc::c_int) +
                1 as libc::c_int + 32 as libc::c_int;
        if (*f_info.offset((*cave[cy as usize].offset(cx as isize)).feat as
                               isize)).flags1 as libc::c_long &
               0x10 as libc::c_long != 0 &&
               (*cave[cy as usize].offset(cx as isize)).feat as libc::c_int !=
                   0xaf as libc::c_int &&
               (*cave[cy as usize].offset(cx as isize)).m_idx == 0 &&
               !(cy == (*p_ptr).py as libc::c_int &&
                     cx == (*p_ptr).px as libc::c_int) {
            break ;
        }
        tries -= 1
    }
    old_get_mon_num_hook = get_mon_num_hook;
    get_mon_num_hook =
        Some(create_molds_hook as
                 unsafe extern "C" fn(_: libc::c_int) -> bool_);
    get_mon_num_prep();
    x = cx - 25 as libc::c_int;
    while x <= cx + 25 as libc::c_int {
        y = cy - 25 as libc::c_int;
        while y <= cy + 25 as libc::c_int {
            if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                   y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   x < cur_wid as libc::c_int - 1 as libc::c_int {
                if !(distance(cy, cx, y, x) > 25 as libc::c_int) {
                    if Rand_div(100 as libc::c_int) < 80 as libc::c_int &&
                           ((*cave[y as usize].offset(x as isize)).feat as
                                libc::c_int == 0xbb as libc::c_int ||
                                (*cave[y as usize].offset(x as isize)).feat as
                                    libc::c_int == 0x54 as libc::c_int) {
                        cave_set_feat(y, x, 0xae as libc::c_int);
                    }
                    if !(distance(cy, cx, y, x) > 10 as libc::c_int) {
                        if Rand_div(100 as libc::c_int) < 60 as libc::c_int {
                            let mut m_idx: libc::c_int = 0;
                            r_idx =
                                get_mon_num(30 as libc::c_int) as libc::c_int;
                            m_idx =
                                place_monster_one(y, x, r_idx,
                                                  0 as libc::c_int,
                                                  0 as libc::c_int as bool_,
                                                  -(2 as libc::c_int)) as
                                    libc::c_int;
                            if m_idx != 0 {
                                let ref mut fresh22 =
                                    (*m_list.offset(m_idx as isize)).mflag;
                                *fresh22 |= 0x2 as libc::c_int
                            }
                            if Rand_div(100 as libc::c_int) <
                                   80 as libc::c_int && m_idx != 0 {
                                let mut m_ptr: *mut monster_type =
                                    &mut *m_list.offset(m_idx as isize) as
                                        *mut monster_type;
                                if ((*m_ptr).level as libc::c_int) <
                                       (*p_ptr).lev as libc::c_int {
                                    (*m_ptr).exp =
                                        ((if (*m_ptr).level as libc::c_int +
                                                 (Rand_div((*p_ptr).lev as
                                                               libc::c_int -
                                                               (*m_ptr).level
                                                                   as
                                                                   libc::c_int)
                                                      + 1 as libc::c_int) >
                                                 150 as libc::c_int {
                                              150 as libc::c_int
                                          } else {
                                              ((*m_ptr).level as libc::c_int)
                                                  +
                                                  (Rand_div((*p_ptr).lev as
                                                                libc::c_int -
                                                                (*m_ptr).level
                                                                    as
                                                                    libc::c_int)
                                                       + 1 as libc::c_int)
                                          }) *
                                             (if (*m_ptr).level as libc::c_int
                                                     +
                                                     (Rand_div((*p_ptr).lev as
                                                                   libc::c_int
                                                                   -
                                                                   (*m_ptr).level
                                                                       as
                                                                       libc::c_int)
                                                          + 1 as libc::c_int)
                                                     > 150 as libc::c_int {
                                                  150 as libc::c_int
                                              } else {
                                                  ((*m_ptr).level as
                                                       libc::c_int) +
                                                      (Rand_div((*p_ptr).lev
                                                                    as
                                                                    libc::c_int
                                                                    -
                                                                    (*m_ptr).level
                                                                        as
                                                                        libc::c_int)
                                                           + 1 as libc::c_int)
                                              }) *
                                             (if (*m_ptr).level as libc::c_int
                                                     +
                                                     (Rand_div((*p_ptr).lev as
                                                                   libc::c_int
                                                                   -
                                                                   (*m_ptr).level
                                                                       as
                                                                       libc::c_int)
                                                          + 1 as libc::c_int)
                                                     > 150 as libc::c_int {
                                                  150 as libc::c_int
                                              } else {
                                                  ((*m_ptr).level as
                                                       libc::c_int) +
                                                      (Rand_div((*p_ptr).lev
                                                                    as
                                                                    libc::c_int
                                                                    -
                                                                    (*m_ptr).level
                                                                        as
                                                                        libc::c_int)
                                                           + 1 as libc::c_int)
                                              }) * 6 as libc::c_int) as u32b;
                                    monster_check_experience(m_idx,
                                                             1 as libc::c_int
                                                                 as bool_);
                                }
                            }
                        }
                    }
                }
            }
            y += 1
        }
        x += 1
    }
    get_mon_num_hook = old_get_mon_num_hook;
    get_mon_num_prep();
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_poison_finish_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
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
    let mut q_idx: s32b = 0;
    q_idx = get_next_arg(fmt);
    if q_idx != 11 as libc::c_int { return 0 as libc::c_int as bool_ }
    c_put_str(11 as libc::c_int as byte_hack,
              b"The water is clean again! Thank you so much.\x00" as *const u8
                  as *const libc::c_char, 8 as libc::c_int, 0 as libc::c_int);
    c_put_str(11 as libc::c_int as byte_hack,
              b"The beautiful Mallorns are safe. Take this as a proof of our gratitude.\x00"
                  as *const u8 as *const libc::c_char, 9 as libc::c_int,
              0 as libc::c_int);
    q_ptr = &mut forge;
    object_prep(q_ptr,
                lookup_kind(38 as libc::c_int, 2 as libc::c_int) as
                    libc::c_int);
    (*q_ptr).found = 6 as libc::c_int as byte_hack;
    (*q_ptr).number = 1 as libc::c_int as byte_hack;
    (*q_ptr).name2 = 10 as libc::c_int as s16b;
    apply_magic(q_ptr, 1 as libc::c_int, 0 as libc::c_int as bool_,
                0 as libc::c_int as bool_, 0 as libc::c_int as bool_);
    object_aware(q_ptr);
    object_known(q_ptr);
    (*q_ptr).ident =
        ((*q_ptr).ident as libc::c_int | 0x10 as libc::c_int) as byte_hack;
    inven_carry(q_ptr, 0 as libc::c_int as bool_);
    *(*quest.offset(q_idx as isize)).plot = 0 as libc::c_int as s16b;
    del_hook(9 as libc::c_int,
             Some(quest_poison_finish_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_));
    process_hooks_restart = 1 as libc::c_int;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_poison_dump_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    if (*quest.offset(11 as libc::c_int as isize)).status as libc::c_int >=
           2 as libc::c_int {
        fprintf(hook_file,
                b"\n You saved the beautiful Mallorns of Lothlorien.\x00" as
                    *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_poison_quest_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
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
    let mut q_idx: s32b = 0;
    q_idx = get_next_arg(fmt);
    if q_idx != 11 as libc::c_int { return 0 as libc::c_int as bool_ }
    q_ptr = &mut forge;
    object_prep(q_ptr,
                lookup_kind(72 as libc::c_int, 18 as libc::c_int) as
                    libc::c_int);
    (*q_ptr).number = 99 as libc::c_int as byte_hack;
    object_aware(q_ptr);
    object_known(q_ptr);
    (*q_ptr).ident =
        ((*q_ptr).ident as libc::c_int | 0x10 as libc::c_int) as byte_hack;
    (*q_ptr).note =
        quark_add(b"quest\x00" as *const u8 as *const libc::c_char) as u16b;
    inven_carry(q_ptr, 0 as libc::c_int as bool_);
    del_hook(13 as libc::c_int,
             Some(quest_poison_quest_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_));
    process_hooks_restart = 1 as libc::c_int;
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_poison_drop_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut mcnt: s32b = 0 as libc::c_int;
    let mut i: s32b = 0;
    let mut x: s32b = 0;
    let mut y: s32b = 0;
    let mut o_idx: s32b = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    o_idx = get_next_arg(fmt);
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(o_idx as isize) as
            *mut object_type;
    if (*quest.offset(11 as libc::c_int as isize)).status as libc::c_int !=
           1 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*p_ptr).wilderness_y !=
           wild_locs[(*quest.offset(11 as libc::c_int as
                                        isize)).data[0 as libc::c_int as
                                                         usize] as
                         usize][0 as libc::c_int as usize] {
        return 0 as libc::c_int as bool_
    }
    if (*p_ptr).wilderness_x !=
           wild_locs[(*quest.offset(11 as libc::c_int as
                                        isize)).data[0 as libc::c_int as
                                                         usize] as
                         usize][1 as libc::c_int as usize] {
        return 0 as libc::c_int as bool_
    }
    if (*p_ptr).wild_mode != 0 { return 0 as libc::c_int as bool_ }
    if (*o_ptr).tval as libc::c_int != 72 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*o_ptr).sval as libc::c_int != 18 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    i = m_max as libc::c_int - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        if !((*m_ptr).r_idx == 0) {
            if (*m_ptr).status as libc::c_int <= 0 as libc::c_int {
                mcnt += 1
            }
        }
        i -= 1
    }
    if mcnt < 10 as libc::c_int {
        x = 1 as libc::c_int;
        while x < cur_wid as libc::c_int - 1 as libc::c_int {
            y = 1 as libc::c_int;
            while y < cur_hgt as libc::c_int - 1 as libc::c_int {
                if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                       y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                       x < cur_wid as libc::c_int - 1 as libc::c_int {
                    if (*cave[y as usize].offset(x as isize)).feat as
                           libc::c_int == 0xae as libc::c_int {
                        cave_set_feat(y, x, 0x54 as libc::c_int);
                    }
                }
                y += 1
            }
            x += 1
        }
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"Well done! The water seems to be clean now.\x00" as
                       *const u8 as *const libc::c_char);
        (*quest.offset(11 as libc::c_int as isize)).status =
            2 as libc::c_int as s16b;
        del_hook(15 as libc::c_int,
                 Some(quest_poison_drop_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        process_hooks_restart = 1 as libc::c_int;
        return 0 as libc::c_int as bool_
    } else {
        msg_print(b"There are too many monsters left to cure the water.\x00"
                      as *const u8 as *const libc::c_char);
        return 1 as libc::c_int as bool_
    };
}
#[no_mangle]
pub unsafe extern "C" fn quest_poison_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    if (*quest.offset(11 as libc::c_int as
                          isize)).data[1 as libc::c_int as usize] == 0 {
        (*quest.offset(11 as libc::c_int as
                           isize)).data[1 as libc::c_int as usize] =
            1 as libc::c_int;
        (*quest.offset(11 as libc::c_int as
                           isize)).data[0 as libc::c_int as usize] =
            Rand_div(4 as libc::c_int);
        if wizard != 0 {
            message_add(1 as libc::c_int as byte_hack,
                        format(b"Wilderness poison %d, %d\x00" as *const u8 as
                                   *const libc::c_char,
                               wild_locs[(*quest.offset(11 as libc::c_int as
                                                            isize)).data[0 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]
                                             as
                                             usize][0 as libc::c_int as
                                                        usize],
                               wild_locs[(*quest.offset(11 as libc::c_int as
                                                            isize)).data[0 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]
                                             as
                                             usize][1 as libc::c_int as
                                                        usize]) as cptr,
                        6 as libc::c_int as byte_hack);
        }
    }
    if (*quest.offset(11 as libc::c_int as isize)).status as libc::c_int >=
           1 as libc::c_int &&
           ((*quest.offset(11 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(15 as libc::c_int,
                 Some(quest_poison_drop_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"poison_drop\x00" as *const u8 as *const libc::c_char);
        add_hook(14 as libc::c_int,
                 Some(quest_poison_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"poison_gen\x00" as *const u8 as *const libc::c_char);
        add_hook(9 as libc::c_int,
                 Some(quest_poison_finish_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"poison_finish\x00" as *const u8 as *const libc::c_char);
    }
    if ((*quest.offset(11 as libc::c_int as isize)).status as libc::c_int) <
           2 as libc::c_int {
        add_hook(13 as libc::c_int,
                 Some(quest_poison_quest_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"poison_iquest\x00" as *const u8 as *const libc::c_char);
    }
    add_hook(12 as libc::c_int,
             Some(quest_poison_dump_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"poison_dump\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_dragons_gen_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut xstart: libc::c_int = 2 as libc::c_int;
    let mut ystart: libc::c_int = 2 as libc::c_int;
    if (*p_ptr).inside_quest as libc::c_int != 23 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*quest.offset(23 as libc::c_int as isize)).status as libc::c_int ==
           0 as libc::c_int {
        (*quest.offset(23 as libc::c_int as isize)).status =
            1 as libc::c_int as s16b
    }
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            cave_set_feat(y, x, 0x3f as libc::c_int);
            x += 1
        }
        y += 1
    }
    dun_level = (*quest.offset((*p_ptr).inside_quest as isize)).level;
    set_mon_num_hook();
    get_mon_num_prep();
    init_flags = 0x4 as libc::c_int;
    process_dungeon_file(b"dragons.map\x00" as *const u8 as
                             *const libc::c_char, &mut ystart, &mut xstart,
                         cur_hgt as libc::c_int, cur_wid as libc::c_int,
                         1 as libc::c_int as bool_,
                         0 as libc::c_int as bool_);
    dungeon_flags2 =
        (dungeon_flags2 as libc::c_long | 0x200 as libc::c_long) as u32b;
    i = 35 as libc::c_int;
    while i > 0 as libc::c_int {
        let mut flags: libc::c_int = 0;
        y = Rand_div(21 as libc::c_int) + 3 as libc::c_int;
        x = Rand_div(31 as libc::c_int) + 3 as libc::c_int;
        flags =
            (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                isize)).flags1 as libc::c_int;
        if flags % 2 as libc::c_int == 0 &&
               flags as libc::c_long & 0x40 as libc::c_long == 0 &&
               flags as libc::c_long & 0x10 as libc::c_long != 0 {
            i -= 1;
            cave_set_feat(y, x, 0x61 as libc::c_int);
        }
    }
    i = 25 as libc::c_int;
    while i > 0 as libc::c_int {
        let mut m_idx: libc::c_int = 0;
        let mut flags_0: libc::c_int = 0;
        y = Rand_div(21 as libc::c_int) + 3 as libc::c_int;
        x = Rand_div(31 as libc::c_int) + 3 as libc::c_int;
        flags_0 =
            (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                isize)).flags1 as libc::c_int;
        if flags_0 as libc::c_long & 0x40 as libc::c_long == 0 &&
               flags_0 as libc::c_long & 0x10 as libc::c_long != 0 {
            let mut baby_dragons: [libc::c_int; 8] =
                [163 as libc::c_int, 164 as libc::c_int, 167 as libc::c_int,
                 166 as libc::c_int, 218 as libc::c_int, 219 as libc::c_int,
                 165 as libc::c_int, 204 as libc::c_int];
            let mut young_dragons: [libc::c_int; 8] =
                [459 as libc::c_int, 460 as libc::c_int, 563 as libc::c_int,
                 546 as libc::c_int, 462 as libc::c_int, 559 as libc::c_int,
                 461 as libc::c_int, 556 as libc::c_int];
            let mut mature_dragons: [libc::c_int; 8] =
                [560 as libc::c_int, 549 as libc::c_int, 589 as libc::c_int,
                 592 as libc::c_int, 562 as libc::c_int, 590 as libc::c_int,
                 561 as libc::c_int, 593 as libc::c_int];
            let mut happy_dragons: [libc::c_int; 8] =
                [601 as libc::c_int, 617 as libc::c_int, 644 as libc::c_int,
                 624 as libc::c_int, 602 as libc::c_int, 645 as libc::c_int,
                 618 as libc::c_int, 675 as libc::c_int];
            let mut chance: libc::c_int = 0;
            let mut dragon: libc::c_int = 0;
            let mut color: libc::c_int = 0;
            color = Rand_div(8 as libc::c_int);
            chance = Rand_div(100 as libc::c_int);
            if chance == 0 as libc::c_int {
                dragon = happy_dragons[color as usize]
            } else if chance < 33 as libc::c_int {
                dragon = baby_dragons[color as usize]
            } else if chance < 66 as libc::c_int {
                dragon = young_dragons[color as usize]
            } else { dragon = mature_dragons[color as usize] }
            i -= 1;
            m_idx =
                place_monster_one(y, x, dragon, 0 as libc::c_int,
                                  (Rand_div(100 as libc::c_int) <
                                       33 as libc::c_int) as libc::c_int as
                                      bool_, -(2 as libc::c_int)) as
                    libc::c_int;
            if m_idx != 0 {
                let ref mut fresh23 = (*m_list.offset(m_idx as isize)).mflag;
                *fresh23 |= 0x2 as libc::c_int
            }
        }
    }
    process_hooks_restart = 1 as libc::c_int;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_dragons_death_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut mcnt: libc::c_int = 0 as libc::c_int;
    if (*p_ptr).inside_quest as libc::c_int != 23 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    i = m_max as libc::c_int - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        if !((*m_ptr).r_idx == 0) {
            if (*m_ptr).status as libc::c_int <= -(2 as libc::c_int) {
                mcnt += 1
            }
        }
        i -= 1
    }
    if mcnt <= 1 as libc::c_int {
        (*quest.offset((*p_ptr).inside_quest as isize)).status =
            2 as libc::c_int as s16b;
        del_hook(0 as libc::c_int,
                 Some(quest_dragons_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        del_hook(2 as libc::c_int,
                 Some(quest_dragons_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        process_hooks_restart = 1 as libc::c_int;
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"Gondolin is safer now.\x00" as *const u8 as
                       *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_dragons_finish_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut q_idx: s32b = 0;
    q_idx = get_next_arg(fmt);
    if q_idx != 23 as libc::c_int { return 0 as libc::c_int as bool_ }
    c_put_str(11 as libc::c_int as byte_hack,
              b"Thank you for killing the dragons!\x00" as *const u8 as
                  *const libc::c_char, 8 as libc::c_int, 0 as libc::c_int);
    c_put_str(11 as libc::c_int as byte_hack,
              b"You can use the cave as your house as a reward.\x00" as
                  *const u8 as *const libc::c_char, 9 as libc::c_int,
              0 as libc::c_int);
    *(*quest.offset(q_idx as isize)).plot = 13 as libc::c_int as s16b;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_dragons_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    if (*quest.offset(23 as libc::c_int as isize)).status as libc::c_int >=
           0 as libc::c_int &&
           ((*quest.offset(23 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(0 as libc::c_int,
                 Some(quest_dragons_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"dragons_monster_death\x00" as *const u8 as
                     *const libc::c_char);
        add_hook(9 as libc::c_int,
                 Some(quest_dragons_finish_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"dragons_finish\x00" as *const u8 as *const libc::c_char);
        add_hook(2 as libc::c_int,
                 Some(quest_dragons_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"dragons_geb\x00" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_eol_gen_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut done: bool_ = 0 as libc::c_int as bool_;
    let mut xsize: libc::c_int = 50 as libc::c_int;
    let mut ysize: libc::c_int = 30 as libc::c_int;
    let mut y0: libc::c_int = 0;
    let mut x0: libc::c_int = 0;
    let mut m_idx: libc::c_int = 0 as libc::c_int;
    if (*p_ptr).inside_quest as libc::c_int != 13 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    x0 = 2 as libc::c_int + xsize / 2 as libc::c_int;
    y0 = 2 as libc::c_int + ysize / 2 as libc::c_int;
    feat_wall_outer = 0x3a as libc::c_int as byte_hack;
    feat_wall_inner = 0x39 as libc::c_int as byte_hack;
    y = 0 as libc::c_int;
    while y < 100 as libc::c_int {
        floor_type[y as usize] = 0x1 as libc::c_int as s16b;
        fill_type[y as usize] = 0x3a as libc::c_int as s16b;
        y += 1
    }
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            cave_set_feat(y, x, 0x3f as libc::c_int);
            x += 1
        }
        y += 1
    }
    dun_level = (*quest.offset((*p_ptr).inside_quest as isize)).level;
    while done == 0 {
        let mut grd: libc::c_int = 0;
        let mut roug: libc::c_int = 0;
        let mut cutoff: libc::c_int = 0;
        grd =
            2 as libc::c_int ^ Rand_div(4 as libc::c_int) + 1 as libc::c_int;
        roug =
            (Rand_div(8 as libc::c_int) + 1 as libc::c_int) *
                (Rand_div(4 as libc::c_int) + 1 as libc::c_int);
        cutoff =
            Rand_div(xsize / 4 as libc::c_int) + 1 as libc::c_int +
                (Rand_div(ysize / 4 as libc::c_int) + 1 as libc::c_int) +
                (Rand_div(xsize / 4 as libc::c_int) + 1 as libc::c_int) +
                (Rand_div(ysize / 4 as libc::c_int) + 1 as libc::c_int);
        generate_hmap(y0, x0, xsize, ysize, grd, roug, cutoff);
        done =
            generate_fracave(y0, x0, xsize, ysize, cutoff,
                             0 as libc::c_int as bool_,
                             1 as libc::c_int as bool_)
    }
    x = xsize - 1 as libc::c_int;
    while x >= 2 as libc::c_int {
        y = ysize - 1 as libc::c_int;
        while y >= 2 as libc::c_int {
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
                if m_idx == 0 {
                    *m_allow_special.offset(test_monster_name(b"Eol, the Dark Elf\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char)
                                                as isize) =
                        1 as libc::c_int as bool_;
                    m_idx =
                        place_monster_one(y, x,
                                          test_monster_name(b"Eol, the Dark Elf\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char),
                                          0 as libc::c_int,
                                          0 as libc::c_int as bool_,
                                          -(2 as libc::c_int)) as libc::c_int;
                    *m_allow_special.offset(test_monster_name(b"Eol, the Dark Elf\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char)
                                                as isize) =
                        0 as libc::c_int as bool_;
                    if m_idx != 0 {
                        let ref mut fresh24 =
                            (*m_list.offset(m_idx as isize)).mflag;
                        *fresh24 |= 0x2 as libc::c_int
                    }
                }
                if Rand_div(100 as libc::c_int) < 18 as libc::c_int {
                    place_trap(y, x);
                }
                (*p_ptr).py = y as s16b;
                (*p_ptr).px = x as s16b
            }
            y -= 1
        }
        x -= 1
    }
    cave_set_feat((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                  0x6 as libc::c_int);
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_eol_finish_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
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
    let mut q_idx: s32b = 0;
    q_idx = get_next_arg(fmt);
    if q_idx != 13 as libc::c_int { return 0 as libc::c_int as bool_ }
    c_put_str(11 as libc::c_int as byte_hack,
              b"A tragedy, but the deed needed to be done.\x00" as *const u8
                  as *const libc::c_char, 8 as libc::c_int, 0 as libc::c_int);
    c_put_str(11 as libc::c_int as byte_hack,
              b"Accept my thanks, and that reward.\x00" as *const u8 as
                  *const libc::c_char, 9 as libc::c_int, 0 as libc::c_int);
    q_ptr = &mut forge;
    object_prep(q_ptr,
                lookup_kind(39 as libc::c_int, 3 as libc::c_int) as
                    libc::c_int);
    (*q_ptr).found = 6 as libc::c_int as byte_hack;
    (*q_ptr).name2 = 163 as libc::c_int as s16b;
    apply_magic(q_ptr, 1 as libc::c_int, 0 as libc::c_int as bool_,
                0 as libc::c_int as bool_, 0 as libc::c_int as bool_);
    (*q_ptr).number = 1 as libc::c_int as byte_hack;
    object_aware(q_ptr);
    object_known(q_ptr);
    (*q_ptr).ident =
        ((*q_ptr).ident as libc::c_int | 0x10 as libc::c_int) as byte_hack;
    inven_carry(q_ptr, 0 as libc::c_int as bool_);
    *(*quest.offset(q_idx as isize)).plot = 14 as libc::c_int as s16b;
    (*quest.offset(*(*quest.offset(q_idx as isize)).plot as
                       isize)).init.expect("non-null function pointer")(*(*quest.offset(q_idx
                                                                                            as
                                                                                            isize)).plot
                                                                            as
                                                                            libc::c_int);
    del_hook(9 as libc::c_int,
             Some(quest_eol_finish_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_));
    process_hooks_restart = 1 as libc::c_int;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_eol_fail_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut q_idx: s32b = 0;
    q_idx = get_next_arg(fmt);
    if q_idx != 13 as libc::c_int { return 0 as libc::c_int as bool_ }
    c_put_str(11 as libc::c_int as byte_hack,
              b"You fled ! I did not think you would flee...\x00" as *const u8
                  as *const libc::c_char, 8 as libc::c_int, 0 as libc::c_int);
    *(*quest.offset(q_idx as isize)).plot = 0 as libc::c_int as s16b;
    del_hook(10 as libc::c_int,
             Some(quest_eol_fail_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_));
    process_hooks_restart = 1 as libc::c_int;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_eol_death_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut r_idx: s32b = 0;
    let mut m_idx: s32b = 0;
    m_idx = get_next_arg(fmt);
    r_idx = (*m_list.offset(m_idx as isize)).r_idx as s32b;
    if (*p_ptr).inside_quest as libc::c_int != 13 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if r_idx ==
           test_monster_name(b"Eol, the Dark Elf\x00" as *const u8 as
                                 *const libc::c_char) {
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"Such a sad end...\x00" as *const u8 as
                       *const libc::c_char);
        (*quest.offset(13 as libc::c_int as isize)).status =
            2 as libc::c_int as s16b;
        del_hook(0 as libc::c_int,
                 Some(quest_eol_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        process_hooks_restart = 1 as libc::c_int;
        return 0 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_eol_stair_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((test_monster_name as
                                 unsafe extern "C" fn(_: cptr)
                                     ->
                                         libc::c_int)(b"Eol, the Dark Elf\x00"
                                                          as *const u8 as
                                                          *const libc::c_char)
                                as isize) as *mut monster_race;
    let mut down: cptr = 0 as *const libc::c_char;
    down = get_next_arg_str(fmt) as cptr;
    if (*p_ptr).inside_quest as libc::c_int != 13 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat as
           libc::c_int != 0x6 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    if (*r_ptr).max_num != 0 {
        if strcmp(down, b"up\x00" as *const u8 as *const libc::c_char) == 0 {
            flush();
            if get_check(b"Really abandon the quest?\x00" as *const u8 as
                             *const libc::c_char) == 0 {
                return 1 as libc::c_int as bool_
            }
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"You flee away from Eol...\x00" as *const u8 as
                           *const libc::c_char);
            (*quest.offset(13 as libc::c_int as isize)).status =
                4 as libc::c_int as s16b;
            del_hook(18 as libc::c_int,
                     Some(quest_eol_stair_hook as
                              unsafe extern "C" fn(_: *mut libc::c_char)
                                  -> bool_));
            process_hooks_restart = 1 as libc::c_int;
            return 0 as libc::c_int as bool_
        }
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_eol_init_hook(mut q: libc::c_int) -> bool_ {
    if (*quest.offset(13 as libc::c_int as isize)).status as libc::c_int >=
           1 as libc::c_int &&
           ((*quest.offset(13 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(0 as libc::c_int,
                 Some(quest_eol_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"eol_death\x00" as *const u8 as *const libc::c_char);
        add_hook(2 as libc::c_int,
                 Some(quest_eol_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"eol_gen\x00" as *const u8 as *const libc::c_char);
        add_hook(18 as libc::c_int,
                 Some(quest_eol_stair_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"eol_stair\x00" as *const u8 as *const libc::c_char);
        add_hook(10 as libc::c_int,
                 Some(quest_eol_fail_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"eol_fail\x00" as *const u8 as *const libc::c_char);
        add_hook(9 as libc::c_int,
                 Some(quest_eol_finish_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"eol_finish\x00" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_nirnaeth_gen_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut xstart: libc::c_int = 2 as libc::c_int;
    let mut ystart: libc::c_int = 2 as libc::c_int;
    if (*p_ptr).inside_quest as libc::c_int != 14 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            cave_set_feat(y, x, 0x3f as libc::c_int);
            x += 1
        }
        y += 1
    }
    dun_level = (*quest.offset((*p_ptr).inside_quest as isize)).level;
    set_mon_num_hook();
    get_mon_num_prep();
    init_flags = 0x4 as libc::c_int;
    process_dungeon_file(b"nirnaeth.map\x00" as *const u8 as
                             *const libc::c_char, &mut ystart, &mut xstart,
                         cur_hgt as libc::c_int, cur_wid as libc::c_int,
                         1 as libc::c_int as bool_,
                         1 as libc::c_int as bool_);
    (*quest.offset(14 as libc::c_int as
                       isize)).data[0 as libc::c_int as usize] =
        0 as libc::c_int;
    (*quest.offset(14 as libc::c_int as
                       isize)).data[1 as libc::c_int as usize] =
        0 as libc::c_int;
    x = 2 as libc::c_int;
    while x < xstart {
        y = 2 as libc::c_int;
        while y < ystart {
            if (*cave[y as usize].offset(x as isize)).m_idx != 0 {
                let ref mut fresh25 =
                    (*quest.offset(14 as libc::c_int as
                                       isize)).data[0 as libc::c_int as
                                                        usize];
                *fresh25 += 1
            }
            y += 1
        }
        x += 1
    }
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_nirnaeth_finish_hook(mut fmt:
                                                        *mut libc::c_char)
 -> bool_ {
    let mut q_idx: s32b = 0;
    q_idx = get_next_arg(fmt);
    if q_idx != 14 as libc::c_int { return 0 as libc::c_int as bool_ }
    if (*quest.offset(14 as libc::c_int as
                          isize)).data[1 as libc::c_int as usize] >=
           2 as libc::c_int *
               (*quest.offset(14 as libc::c_int as
                                  isize)).data[0 as libc::c_int as usize] /
               3 as libc::c_int {
        c_put_str(11 as libc::c_int as byte_hack,
                  b"Not only did you found a way out, but you also destroyed a good\x00"
                      as *const u8 as *const libc::c_char, 8 as libc::c_int,
                  0 as libc::c_int);
        c_put_str(11 as libc::c_int as byte_hack,
                  b"number of trolls! Thank you so much. Take this gold please.\x00"
                      as *const u8 as *const libc::c_char, 9 as libc::c_int,
                  0 as libc::c_int);
        c_put_str(11 as libc::c_int as byte_hack,
                  b"I also grant you access to the royal jewelry shop!\x00" as
                      *const u8 as *const libc::c_char, 10 as libc::c_int,
                  0 as libc::c_int);
        (*p_ptr).au += 200000 as libc::c_int;
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x100 as libc::c_long) as u32b;
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b
    } else {
        c_put_str(11 as libc::c_int as byte_hack,
                  b"I thank you for your efforts.\x00" as *const u8 as
                      *const libc::c_char, 8 as libc::c_int,
                  0 as libc::c_int);
        c_put_str(11 as libc::c_int as byte_hack,
                  b"I grant you access to the royal jewelry shop!\x00" as
                      *const u8 as *const libc::c_char, 9 as libc::c_int,
                  0 as libc::c_int);
    }
    *(*quest.offset(q_idx as isize)).plot = 0 as libc::c_int as s16b;
    del_hook(9 as libc::c_int,
             Some(quest_nirnaeth_finish_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_));
    process_hooks_restart = 1 as libc::c_int;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_nirnaeth_death_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    if (*p_ptr).inside_quest as libc::c_int != 14 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    let ref mut fresh26 =
        (*quest.offset(14 as libc::c_int as
                           isize)).data[1 as libc::c_int as usize];
    *fresh26 += 1;
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_nirnaeth_stair_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    if (*p_ptr).inside_quest as libc::c_int != 14 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat as
           libc::c_int != 0x6 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    cmsg_print(11 as libc::c_int as byte_hack,
               b"You found a way out!\x00" as *const u8 as
                   *const libc::c_char);
    (*quest.offset(14 as libc::c_int as isize)).status =
        2 as libc::c_int as s16b;
    del_hook(18 as libc::c_int,
             Some(quest_nirnaeth_stair_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_));
    process_hooks_restart = 1 as libc::c_int;
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_nirnaeth_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    if (*quest.offset(14 as libc::c_int as isize)).status as libc::c_int >=
           1 as libc::c_int &&
           ((*quest.offset(14 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(0 as libc::c_int,
                 Some(quest_nirnaeth_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"nirnaeth_death\x00" as *const u8 as *const libc::c_char);
        add_hook(2 as libc::c_int,
                 Some(quest_nirnaeth_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"nirnaeth_gen\x00" as *const u8 as *const libc::c_char);
        add_hook(18 as libc::c_int,
                 Some(quest_nirnaeth_stair_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"nirnaeth_stair\x00" as *const u8 as *const libc::c_char);
        add_hook(9 as libc::c_int,
                 Some(quest_nirnaeth_finish_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"nirnaeth_finish\x00" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_invasion_gen_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut xstart: libc::c_int = 2 as libc::c_int;
    let mut ystart: libc::c_int = 2 as libc::c_int;
    if (*p_ptr).inside_quest as libc::c_int != 15 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            cave_set_feat(y, x, 0x3f as libc::c_int);
            x += 1
        }
        y += 1
    }
    dun_level = (*quest.offset((*p_ptr).inside_quest as isize)).level;
    set_mon_num_hook();
    get_mon_num_prep();
    init_flags = 0x4 as libc::c_int;
    process_dungeon_file(b"maeglin.map\x00" as *const u8 as
                             *const libc::c_char, &mut ystart, &mut xstart,
                         cur_hgt as libc::c_int, cur_wid as libc::c_int,
                         1 as libc::c_int as bool_,
                         1 as libc::c_int as bool_);
    x = 3 as libc::c_int;
    while x < xstart {
        y = 3 as libc::c_int;
        while y < ystart {
            if (*cave[y as usize].offset(x as isize)).feat as libc::c_int ==
                   0xac as libc::c_int {
                (*quest.offset(15 as libc::c_int as
                                   isize)).data[0 as libc::c_int as usize] =
                    y;
                (*quest.offset(15 as libc::c_int as
                                   isize)).data[1 as libc::c_int as usize] =
                    x;
                (*p_ptr).py = y as s16b;
                (*p_ptr).px = x as s16b;
                cave_set_feat((*p_ptr).py as libc::c_int,
                              (*p_ptr).px as libc::c_int, 0x6 as libc::c_int);
            }
            y += 1
        }
        x += 1
    }
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_invasion_ai_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    let mut m_idx: s32b = 0;
    m_idx = get_next_arg(fmt);
    m_ptr = &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    if (*p_ptr).inside_quest as libc::c_int != 15 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*m_ptr).r_idx as libc::c_int == 825 as libc::c_int {
        if (*m_ptr).fy as libc::c_int ==
               (*quest.offset(15 as libc::c_int as
                                  isize)).data[0 as libc::c_int as usize] &&
               (*m_ptr).fx as libc::c_int ==
                   (*quest.offset(15 as libc::c_int as
                                      isize)).data[1 as libc::c_int as usize]
           {
            delete_monster_idx(m_idx);
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"Maeglin found the way to Gondolin! All hope is lost now!\x00"
                           as *const u8 as *const libc::c_char);
            (*quest.offset(15 as libc::c_int as isize)).status =
                4 as libc::c_int as s16b;
            (*town_info.offset(2 as libc::c_int as isize)).destroyed =
                1 as libc::c_int as bool_;
            return 0 as libc::c_int as bool_
        }
        if distance((*m_ptr).fy as libc::c_int, (*m_ptr).fx as libc::c_int,
                    (*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int) <=
               2 as libc::c_int {
            return 0 as libc::c_int as bool_
        } else {
            process_hooks_return[0 as libc::c_int as usize].num =
                (*quest.offset(15 as libc::c_int as
                                   isize)).data[0 as libc::c_int as usize];
            process_hooks_return[1 as libc::c_int as usize].num =
                (*quest.offset(15 as libc::c_int as
                                   isize)).data[1 as libc::c_int as usize];
            return 1 as libc::c_int as bool_
        }
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_invasion_turn_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut old_quick_messages: bool_ = quick_messages;
    if (*quest.offset(15 as libc::c_int as isize)).status as libc::c_int !=
           0 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if ((*p_ptr).lev as libc::c_int) < 45 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*p_ptr).inside_quest != 0 { return 0 as libc::c_int as bool_ }
    if (*p_ptr).astral != 0 { return 0 as libc::c_int as bool_ }
    quick_messages = 0 as libc::c_int as bool_;
    cmsg_print(11 as libc::c_int as byte_hack,
               b"A Thunderlord appears in front of you and says:\x00" as
                   *const u8 as *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"\'Hello, noble hero. I am Liron, rider of Tolan. Turgon, King of Gondolin sent me.\'\x00"
                   as *const u8 as *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"\'Gondolin is being invaded; he needs your help now or everything will be lost.\'\x00"
                   as *const u8 as *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"\'Please come quickly!\'\x00" as *const u8 as
                   *const libc::c_char);
    (*quest.offset(15 as libc::c_int as isize)).status =
        1 as libc::c_int as s16b;
    quick_messages = old_quick_messages;
    quest_invasion_init_hook(15 as libc::c_int);
    del_hook(3 as libc::c_int,
             Some(quest_invasion_turn_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_));
    process_hooks_restart = 1 as libc::c_int;
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_invasion_dump_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    if (*quest.offset(15 as libc::c_int as isize)).status as libc::c_int ==
           4 as libc::c_int {
        fprintf(hook_file,
                b"\n You abandoned Gondolin when it most needed you, thus causing its destruction.\x00"
                    as *const u8 as *const libc::c_char);
    }
    if (*quest.offset(15 as libc::c_int as isize)).status as libc::c_int ==
           5 as libc::c_int ||
           (*quest.offset(15 as libc::c_int as isize)).status as libc::c_int
               == 3 as libc::c_int ||
           (*quest.offset(15 as libc::c_int as isize)).status as libc::c_int
               == 2 as libc::c_int {
        fprintf(hook_file,
                b"\n You saved Gondolin from destruction.\x00" as *const u8 as
                    *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_invasion_death_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut r_idx: s32b = 0;
    let mut m_idx: s32b = 0;
    m_idx = get_next_arg(fmt);
    r_idx = (*m_list.offset(m_idx as isize)).r_idx as s32b;
    if (*p_ptr).inside_quest as libc::c_int != 15 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if r_idx ==
           test_monster_name(b"Maeglin, the Traitor of Gondolin\x00" as
                                 *const u8 as *const libc::c_char) {
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"You did it! Gondolin will remain hidden.\x00" as
                       *const u8 as *const libc::c_char);
        (*quest.offset(15 as libc::c_int as isize)).status =
            2 as libc::c_int as s16b;
        del_hook(0 as libc::c_int,
                 Some(quest_invasion_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        process_hooks_restart = 1 as libc::c_int;
        return 0 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_invasion_stair_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut down: cptr = 0 as *const libc::c_char;
    down = get_next_arg_str(fmt) as cptr;
    if (*p_ptr).inside_quest as libc::c_int != 15 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat as
           libc::c_int != 0x6 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    if strcmp(down, b"up\x00" as *const u8 as *const libc::c_char) == 0 {
        if (*quest.offset(15 as libc::c_int as isize)).status as libc::c_int
               == 4 as libc::c_int {
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"The armies of Morgoth totally devastated Gondolin, leaving nothing but ruins...\x00"
                           as *const u8 as *const libc::c_char);
        } else if (*quest.offset(15 as libc::c_int as isize)).status as
                      libc::c_int == 2 as libc::c_int {
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"Turgon appears before you and speaks:\x00" as
                           *const u8 as *const libc::c_char);
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"\'I will never be able to thank you enough.\'\x00" as
                           *const u8 as *const libc::c_char);
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"\'My most powerful mages will cast a powerful spell for you, giving you extra life.\'\x00"
                           as *const u8 as *const libc::c_char);
            (*p_ptr).hp_mod =
                ((*p_ptr).hp_mod as libc::c_int + 150 as libc::c_int) as s16b;
            (*p_ptr).update =
                ((*p_ptr).update as libc::c_long | 0x10 as libc::c_long) as
                    u32b;
            (*quest.offset(15 as libc::c_int as isize)).status =
                5 as libc::c_int as s16b
        } else {
            flush();
            if get_check(b"Really abandon the quest?\x00" as *const u8 as
                             *const libc::c_char) == 0 {
                return 1 as libc::c_int as bool_
            }
            cmsg_print(11 as libc::c_int as byte_hack,
                       b"You flee away from Maeglin and his army...\x00" as
                           *const u8 as *const libc::c_char);
            (*quest.offset(15 as libc::c_int as isize)).status =
                4 as libc::c_int as s16b;
            (*town_info.offset(2 as libc::c_int as isize)).destroyed =
                1 as libc::c_int as bool_
        }
        del_hook(18 as libc::c_int,
                 Some(quest_invasion_stair_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        process_hooks_restart = 1 as libc::c_int;
        return 0 as libc::c_int as bool_
    }
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_invasion_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    add_hook(3 as libc::c_int,
             Some(quest_invasion_turn_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"invasion_turn\x00" as *const u8 as *const libc::c_char);
    add_hook(12 as libc::c_int,
             Some(quest_invasion_dump_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"invasion_dump\x00" as *const u8 as *const libc::c_char);
    if (*quest.offset(15 as libc::c_int as isize)).status as libc::c_int >=
           1 as libc::c_int &&
           ((*quest.offset(15 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(19 as libc::c_int,
                 Some(quest_invasion_ai_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"invasion_ai\x00" as *const u8 as *const libc::c_char);
        add_hook(2 as libc::c_int,
                 Some(quest_invasion_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"invasion_gen\x00" as *const u8 as *const libc::c_char);
        add_hook(0 as libc::c_int,
                 Some(quest_invasion_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"invasion_death\x00" as *const u8 as *const libc::c_char);
        add_hook(18 as libc::c_int,
                 Some(quest_invasion_stair_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"invasion_stair\x00" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_haunted_gen_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut m_idx: libc::c_int = 0;
    let mut xstart: libc::c_int = 2 as libc::c_int;
    let mut ystart: libc::c_int = 2 as libc::c_int;
    if (*p_ptr).inside_quest as libc::c_int != 24 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*quest.offset(24 as libc::c_int as isize)).status as libc::c_int ==
           0 as libc::c_int {
        (*quest.offset(24 as libc::c_int as isize)).status =
            1 as libc::c_int as s16b
    }
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            cave_set_feat(y, x, 0x3f as libc::c_int);
            x += 1
        }
        y += 1
    }
    dun_level = (*quest.offset((*p_ptr).inside_quest as isize)).level;
    set_mon_num_hook();
    get_mon_num_prep();
    init_flags = 0x4 as libc::c_int;
    process_dungeon_file(b"haunted.map\x00" as *const u8 as
                             *const libc::c_char, &mut ystart, &mut xstart,
                         cur_hgt as libc::c_int, cur_wid as libc::c_int,
                         1 as libc::c_int as bool_,
                         0 as libc::c_int as bool_);
    dungeon_flags2 =
        (dungeon_flags2 as libc::c_long | 0x200 as libc::c_long) as u32b;
    i = 12 as libc::c_int;
    while i > 0 as libc::c_int {
        let mut flags: libc::c_int = 0;
        y = Rand_div(21 as libc::c_int) + 3 as libc::c_int;
        x = Rand_div(31 as libc::c_int) + 3 as libc::c_int;
        flags =
            (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                isize)).flags1 as libc::c_int;
        if flags as libc::c_long & 0x40 as libc::c_long == 0 &&
               flags as libc::c_long & 0x10 as libc::c_long != 0 {
            m_idx =
                place_monster_one(y, x, 477 as libc::c_int, 0 as libc::c_int,
                                  0 as libc::c_int as bool_,
                                  -(2 as libc::c_int)) as libc::c_int;
            if m_idx != 0 {
                let ref mut fresh27 = (*m_list.offset(m_idx as isize)).mflag;
                *fresh27 |= 0x2 as libc::c_int
            }
            i -= 1
        }
    }
    i = damroll(4 as libc::c_int as s16b, 4 as libc::c_int as s16b);
    while i > 0 as libc::c_int {
        let mut flags_0: libc::c_int = 0;
        y = Rand_div(21 as libc::c_int) + 3 as libc::c_int;
        x = Rand_div(31 as libc::c_int) + 3 as libc::c_int;
        flags_0 =
            (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                isize)).flags1 as libc::c_int;
        if flags_0 as libc::c_long & 0x40 as libc::c_long == 0 &&
               flags_0 as libc::c_long & 0x10 as libc::c_long != 0 {
            let mut monsters: [libc::c_int; 22] =
                [65 as libc::c_int, 100 as libc::c_int, 124 as libc::c_int,
                 125 as libc::c_int, 133 as libc::c_int, 231 as libc::c_int,
                 273 as libc::c_int, 327 as libc::c_int, 365 as libc::c_int,
                 416 as libc::c_int, 418 as libc::c_int, 507 as libc::c_int,
                 508 as libc::c_int, 533 as libc::c_int, 534 as libc::c_int,
                 553 as libc::c_int, 554 as libc::c_int, 555 as libc::c_int,
                 577 as libc::c_int, 607 as libc::c_int, 622 as libc::c_int,
                 665 as libc::c_int];
            let mut monster: libc::c_int =
                monsters[Rand_div(22 as libc::c_int) as usize];
            m_idx =
                place_monster_one(y, x, monster, 0 as libc::c_int,
                                  0 as libc::c_int as bool_,
                                  -(2 as libc::c_int)) as libc::c_int;
            let ref mut fresh28 = (*m_list.offset(m_idx as isize)).mflag;
            *fresh28 |= 0x2 as libc::c_int;
            i -= 1
        }
    }
    i =
        10 as libc::c_int +
            damroll(4 as libc::c_int as s16b, 4 as libc::c_int as s16b);
    while i > 0 as libc::c_int {
        let mut flags_1: libc::c_int = 0;
        y = Rand_div(21 as libc::c_int) + 3 as libc::c_int;
        x = Rand_div(31 as libc::c_int) + 3 as libc::c_int;
        flags_1 =
            (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                isize)).flags1 as libc::c_int;
        if flags_1 as libc::c_long & 0x40 as libc::c_long == 0 &&
               flags_1 as libc::c_long & 0x10 as libc::c_long != 0 {
            i -= 1;
            place_trap(y, x);
        }
    }
    process_hooks_restart = 1 as libc::c_int;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_haunted_death_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut mcnt: libc::c_int = 0 as libc::c_int;
    if (*p_ptr).inside_quest as libc::c_int != 24 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    i = m_max as libc::c_int - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        if !((*m_ptr).r_idx == 0) {
            if (*m_ptr).status as libc::c_int <= -(2 as libc::c_int) {
                mcnt += 1
            }
        }
        i -= 1
    }
    if mcnt <= 1 as libc::c_int {
        (*quest.offset((*p_ptr).inside_quest as isize)).status =
            2 as libc::c_int as s16b;
        del_hook(0 as libc::c_int,
                 Some(quest_haunted_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        del_hook(2 as libc::c_int,
                 Some(quest_haunted_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        process_hooks_restart = 1 as libc::c_int;
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"Minas Anor is safer now.\x00" as *const u8 as
                       *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_haunted_finish_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut q_idx: s32b = 0;
    q_idx = get_next_arg(fmt);
    if q_idx != 24 as libc::c_int { return 0 as libc::c_int as bool_ }
    c_put_str(11 as libc::c_int as byte_hack,
              b"Thank you for saving us!\x00" as *const u8 as
                  *const libc::c_char, 8 as libc::c_int, 0 as libc::c_int);
    c_put_str(11 as libc::c_int as byte_hack,
              b"You can use the building as your house as a reward.\x00" as
                  *const u8 as *const libc::c_char, 9 as libc::c_int,
              0 as libc::c_int);
    *(*quest.offset(q_idx as isize)).plot = 16 as libc::c_int as s16b;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_haunted_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    if (*quest.offset(24 as libc::c_int as isize)).status as libc::c_int >=
           0 as libc::c_int &&
           ((*quest.offset(24 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(0 as libc::c_int,
                 Some(quest_haunted_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"haunted_monster_death\x00" as *const u8 as
                     *const libc::c_char);
        add_hook(9 as libc::c_int,
                 Some(quest_haunted_finish_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"haunted_finish\x00" as *const u8 as *const libc::c_char);
        add_hook(2 as libc::c_int,
                 Some(quest_haunted_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"haunted_geb\x00" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_between_move_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut y: s32b = 0;
    let mut x: s32b = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    y = get_next_arg(fmt);
    x = get_next_arg(fmt);
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    if (*quest.offset(16 as libc::c_int as isize)).status as libc::c_int !=
           1 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*c_ptr).feat as libc::c_int == 0x4a as libc::c_int &&
           (*c_ptr).special as libc::c_int == 27 as libc::c_int {
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"Turgon is there.\x00" as *const u8 as
                       *const libc::c_char);
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"\'Ah, thank you, noble hero! Now please return to Minas Anor to finish the link.\'\x00"
                       as *const u8 as *const libc::c_char);
        (*quest.offset(16 as libc::c_int as isize)).status =
            2 as libc::c_int as s16b;
        return 1 as libc::c_int as bool_
    }
    if (*quest.offset(16 as libc::c_int as
                          isize)).data[0 as libc::c_int as usize] != 0 {
        return 0 as libc::c_int as bool_
    }
    if (*p_ptr).wild_mode == 0 {
        if (*p_ptr).wilderness_y > 19 as libc::c_int {
            return 0 as libc::c_int as bool_
        }
    } else if (*p_ptr).py as libc::c_int > 19 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    (*quest.offset(16 as libc::c_int as
                       isize)).data[0 as libc::c_int as usize] =
        1 as libc::c_int;
    (*p_ptr).wild_mode = 0 as libc::c_int as bool_;
    (*p_ptr).inside_quest = 16 as libc::c_int as s16b;
    (*p_ptr).leaving = 1 as libc::c_int as bool_;
    cmsg_print(11 as libc::c_int as byte_hack,
               b"Looks like a full wing of thunderlords ambushes you!\x00" as
                   *const u8 as *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"Trone steps forth and speaks: \'The secret of the Void Jumpgates\x00"
                   as *const u8 as *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"will not be used by any but the thunderlords!\'\x00" as
                   *const u8 as *const libc::c_char);
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_between_gen_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut xstart: libc::c_int = 2 as libc::c_int;
    let mut ystart: libc::c_int = 2 as libc::c_int;
    if (*p_ptr).inside_quest as libc::c_int != 16 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            cave_set_feat(y, x, 0x3f as libc::c_int);
            x += 1
        }
        y += 1
    }
    dun_level = (*quest.offset((*p_ptr).inside_quest as isize)).level;
    set_mon_num_hook();
    get_mon_num_prep();
    init_flags = 0x4 as libc::c_int;
    process_dungeon_file(b"between.map\x00" as *const u8 as
                             *const libc::c_char, &mut ystart, &mut xstart,
                         cur_hgt as libc::c_int, cur_wid as libc::c_int,
                         1 as libc::c_int as bool_,
                         1 as libc::c_int as bool_);
    energy_use = 0 as libc::c_int;
    dungeon_flags2 =
        (dungeon_flags2 as libc::c_long | 0x200 as libc::c_long) as u32b;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_between_finish_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut q_idx: s32b = 0;
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
    q_idx = get_next_arg(fmt);
    if q_idx != 16 as libc::c_int { return 0 as libc::c_int as bool_ }
    c_put_str(11 as libc::c_int as byte_hack,
              b"Ah you finally arrived, I hope your travel wasn\'t too hard.\x00"
                  as *const u8 as *const libc::c_char, 8 as libc::c_int,
              0 as libc::c_int);
    c_put_str(11 as libc::c_int as byte_hack,
              b"As a reward you can freely use the Void Jumpgates for quick travel.\x00"
                  as *const u8 as *const libc::c_char, 9 as libc::c_int,
              0 as libc::c_int);
    c_put_str(11 as libc::c_int as byte_hack,
              b"Oh and take that horn, it shall serve you well.\x00" as
                  *const u8 as *const libc::c_char, 10 as libc::c_int,
              0 as libc::c_int);
    q_ptr = &mut forge;
    object_prep(q_ptr,
                test_item_name(b"& Golden Horn~ of the Thunderlords\x00" as
                                   *const u8 as *const libc::c_char));
    (*q_ptr).found = 6 as libc::c_int as byte_hack;
    (*q_ptr).number = 1 as libc::c_int as byte_hack;
    *k_allow_special.offset(test_item_name(b"& Golden Horn~ of the Thunderlords\x00"
                                               as *const u8 as
                                               *const libc::c_char) as isize)
        = 1 as libc::c_int as bool_;
    apply_magic(q_ptr, -(1 as libc::c_int), 1 as libc::c_int as bool_,
                1 as libc::c_int as bool_, 1 as libc::c_int as bool_);
    *k_allow_special.offset(test_item_name(b"& Golden Horn~ of the Thunderlords\x00"
                                               as *const u8 as
                                               *const libc::c_char) as isize)
        = 0 as libc::c_int as bool_;
    object_aware(q_ptr);
    object_known(q_ptr);
    (*q_ptr).discount = 100 as libc::c_int as byte_hack;
    (*q_ptr).ident =
        ((*q_ptr).ident as libc::c_int | 0x10 as libc::c_int) as byte_hack;
    inven_carry(q_ptr, 0 as libc::c_int as bool_);
    *(*quest.offset(q_idx as isize)).plot = 0 as libc::c_int as s16b;
    del_hook(9 as libc::c_int,
             Some(quest_between_finish_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_));
    process_hooks_restart = 1 as libc::c_int;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_between_death_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut mcnt: libc::c_int = 0 as libc::c_int;
    if (*p_ptr).inside_quest as libc::c_int != 16 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    i = m_max as libc::c_int - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        if !((*m_ptr).r_idx == 0) {
            if (*m_ptr).status as libc::c_int <= 0 as libc::c_int {
                mcnt += 1
            }
        }
        i -= 1
    }
    if mcnt < 2 as libc::c_int {
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"You can escape now.\x00" as *const u8 as
                       *const libc::c_char);
        cave_set_feat((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                      0x6 as libc::c_int);
        (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).special =
            0 as libc::c_int as s16b;
        return 0 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_between_dump_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    if (*quest.offset(16 as libc::c_int as isize)).status as libc::c_int >=
           2 as libc::c_int {
        fprintf(hook_file,
                b"\n You established a permanent void jumpgates liaison between Minas Anor and Gondolin,\x00"
                    as *const u8 as *const libc::c_char);
        fprintf(hook_file,
                b"\n  thus allowing the last alliance to exist.\x00" as
                    *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_between_forbid_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut q_idx: s32b = 0;
    q_idx = get_next_arg(fmt);
    if q_idx != 16 as libc::c_int { return 0 as libc::c_int as bool_ }
    if ((*p_ptr).lev as libc::c_int) < 45 as libc::c_int {
        c_put_str(1 as libc::c_int as byte_hack,
                  b"I fear you are not ready for the next quest, come back later.\x00"
                      as *const u8 as *const libc::c_char, 8 as libc::c_int,
                  0 as libc::c_int);
        return 1 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_between_init_hook(mut q: libc::c_int)
 -> bool_ {
    if (*quest.offset(16 as libc::c_int as isize)).status as libc::c_int >=
           1 as libc::c_int &&
           ((*quest.offset(16 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(17 as libc::c_int,
                 Some(quest_between_move_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"between_move\x00" as *const u8 as *const libc::c_char);
        add_hook(2 as libc::c_int,
                 Some(quest_between_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"between_gen\x00" as *const u8 as *const libc::c_char);
        add_hook(9 as libc::c_int,
                 Some(quest_between_finish_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"between_finish\x00" as *const u8 as *const libc::c_char);
        add_hook(0 as libc::c_int,
                 Some(quest_between_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"between_death\x00" as *const u8 as *const libc::c_char);
    }
    add_hook(12 as libc::c_int,
             Some(quest_between_dump_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"between_dump\x00" as *const u8 as *const libc::c_char);
    add_hook(13 as libc::c_int,
             Some(quest_between_forbid_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"between_forbid\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_evil_gen_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut xstart: libc::c_int = 2 as libc::c_int;
    let mut ystart: libc::c_int = 2 as libc::c_int;
    if (*p_ptr).inside_quest as libc::c_int != 25 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*quest.offset(25 as libc::c_int as isize)).status as libc::c_int ==
           0 as libc::c_int {
        (*quest.offset(25 as libc::c_int as isize)).status =
            1 as libc::c_int as s16b
    }
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            cave_set_feat(y, x, 0x3f as libc::c_int);
            x += 1
        }
        y += 1
    }
    dun_level = (*quest.offset((*p_ptr).inside_quest as isize)).level;
    set_mon_num_hook();
    get_mon_num_prep();
    init_flags = 0x4 as libc::c_int;
    process_dungeon_file(b"evil.map\x00" as *const u8 as *const libc::c_char,
                         &mut ystart, &mut xstart, cur_hgt as libc::c_int,
                         cur_wid as libc::c_int, 1 as libc::c_int as bool_,
                         0 as libc::c_int as bool_);
    dungeon_flags2 =
        (dungeon_flags2 as libc::c_long | 0x200 as libc::c_long) as u32b;
    i = 6 as libc::c_int;
    while i > 0 as libc::c_int {
        let mut m_idx: libc::c_int = 0;
        let mut flags: libc::c_int = 0;
        y = Rand_div(21 as libc::c_int) + 3 as libc::c_int;
        x = Rand_div(31 as libc::c_int) + 3 as libc::c_int;
        flags =
            (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                isize)).flags1 as libc::c_int;
        if flags as libc::c_long & 0x40 as libc::c_long == 0 &&
               flags as libc::c_long & 0x10 as libc::c_long != 0 {
            m_idx =
                place_monster_one(y, x, 996 as libc::c_int, 0 as libc::c_int,
                                  0 as libc::c_int as bool_,
                                  -(2 as libc::c_int)) as libc::c_int;
            if m_idx != 0 {
                let ref mut fresh29 = (*m_list.offset(m_idx as isize)).mflag;
                *fresh29 |= 0x2 as libc::c_int
            }
            i -= 1
        }
    }
    process_hooks_restart = 1 as libc::c_int;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_evil_death_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut mcnt: libc::c_int = 0 as libc::c_int;
    if (*p_ptr).inside_quest as libc::c_int != 25 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    i = m_max as libc::c_int - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        if !((*m_ptr).r_idx == 0) {
            if (*m_ptr).status as libc::c_int <= -(2 as libc::c_int) {
                mcnt += 1
            }
        }
        i -= 1
    }
    if mcnt <= 1 as libc::c_int {
        (*quest.offset((*p_ptr).inside_quest as isize)).status =
            5 as libc::c_int as s16b;
        *(*quest.offset((*p_ptr).inside_quest as isize)).plot =
            0 as libc::c_int as s16b;
        del_hook(0 as libc::c_int,
                 Some(quest_evil_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        del_hook(2 as libc::c_int,
                 Some(quest_evil_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_));
        process_hooks_restart = 1 as libc::c_int;
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"Khazad-Dum is safer now.\x00" as *const u8 as
                       *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_evil_finish_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut q_idx: s32b = 0;
    q_idx = get_next_arg(fmt);
    if q_idx != 25 as libc::c_int { return 0 as libc::c_int as bool_ }
    c_put_str(11 as libc::c_int as byte_hack,
              b"Thank you for saving us!\x00" as *const u8 as
                  *const libc::c_char, 8 as libc::c_int, 0 as libc::c_int);
    c_put_str(11 as libc::c_int as byte_hack,
              b"You can use the cave as your house as a reward.\x00" as
                  *const u8 as *const libc::c_char, 9 as libc::c_int,
              0 as libc::c_int);
    *(*quest.offset(q_idx as isize)).plot = 0 as libc::c_int as s16b;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_evil_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    if (*quest.offset(25 as libc::c_int as isize)).status as libc::c_int >=
           0 as libc::c_int &&
           ((*quest.offset(25 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(0 as libc::c_int,
                 Some(quest_evil_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"evil_monster_death\x00" as *const u8 as
                     *const libc::c_char);
        add_hook(9 as libc::c_int,
                 Some(quest_evil_finish_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"evil_finish\x00" as *const u8 as *const libc::c_char);
        add_hook(2 as libc::c_int,
                 Some(quest_evil_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"evil_geb\x00" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_narsil_move_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut y: s32b = 0;
    let mut x: s32b = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut i: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    y = get_next_arg(fmt);
    x = get_next_arg(fmt);
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    if (*quest.offset(12 as libc::c_int as isize)).status as libc::c_int !=
           1 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*c_ptr).feat as libc::c_int != 0x4a as libc::c_int ||
           (*c_ptr).special as libc::c_int != 14 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int {
        o_ptr = get_object(i);
        if !((*o_ptr).k_idx == 0) {
            if (*o_ptr).name1 as libc::c_int == 164 as libc::c_int { break ; }
        }
        i += 1
    }
    if i == 52 as libc::c_int { return 0 as libc::c_int as bool_ }
    cmsg_print(11 as libc::c_int as byte_hack,
               b"I heard that the broken sword had been found!\x00" as
                   *const u8 as *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"I thought it was only a rumor... until now.\x00" as *const u8
                   as *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"What you have is really the sword that was broken.\x00" as
                   *const u8 as *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"I will get it reforged...\x00" as *const u8 as
                   *const libc::c_char);
    cmsg_print(14 as libc::c_int as byte_hack,
               b"Aragorn leaves for a few hours then comes back...\x00" as
                   *const u8 as *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"Here it is, Anduril, the sword that was forged and is\x00" as
                   *const u8 as *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"reforged again. Take it; you will surely need it for your quest.\x00"
                   as *const u8 as *const libc::c_char);
    object_prep(o_ptr,
                lookup_kind(23 as libc::c_int, 17 as libc::c_int) as
                    libc::c_int);
    (*o_ptr).name1 = 83 as libc::c_int as byte_hack;
    apply_magic(o_ptr, -(1 as libc::c_int), 1 as libc::c_int as bool_,
                1 as libc::c_int as bool_, 1 as libc::c_int as bool_);
    object_aware(o_ptr);
    object_known(o_ptr);
    inven_item_describe(i);
    inven_item_optimize(i);
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x2 as libc::c_long | 0x8 as libc::c_long |
                  0x1 as libc::c_long)) as u32b;
    (*quest.offset(12 as libc::c_int as isize)).status =
        5 as libc::c_int as s16b;
    del_hook(17 as libc::c_int,
             Some(quest_narsil_move_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_));
    process_hooks_restart = 1 as libc::c_int;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_narsil_dump_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    if (*quest.offset(12 as libc::c_int as isize)).status as libc::c_int >=
           2 as libc::c_int {
        fprintf(hook_file,
                b"\n The sword that was broken is now reforged.\x00" as
                    *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_narsil_identify_hook(mut fmt:
                                                        *mut libc::c_char)
 -> bool_ {
    if (*quest.offset(12 as libc::c_int as isize)).status as libc::c_int ==
           0 as libc::c_int {
        let mut i: libc::c_int = 0;
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
        let mut item: s32b = 0;
        item = get_next_arg(fmt);
        o_ptr = get_object(item);
        if (*o_ptr).name1 as libc::c_int == 164 as libc::c_int {
            (*quest.offset(12 as libc::c_int as isize)).status =
                1 as libc::c_int as s16b;
            i = 0 as libc::c_int;
            while i < 5 as libc::c_int {
                if (*quest.offset(12 as libc::c_int as
                                      isize)).desc[i as
                                                       usize][0 as libc::c_int
                                                                  as usize] as
                       libc::c_int != '\u{0}' as i32 {
                    cmsg_print(11 as libc::c_int as byte_hack,
                               (*quest.offset(12 as libc::c_int as
                                                  isize)).desc[i as
                                                                   usize].as_mut_ptr()
                                   as cptr);
                }
                i += 1
            }
            add_hook(17 as libc::c_int,
                     Some(quest_narsil_move_hook as
                              unsafe extern "C" fn(_: *mut libc::c_char)
                                  -> bool_),
                     b"narsil_move\x00" as *const u8 as *const libc::c_char);
            del_hook(16 as libc::c_int,
                     Some(quest_narsil_identify_hook as
                              unsafe extern "C" fn(_: *mut libc::c_char)
                                  -> bool_));
            process_hooks_restart = 1 as libc::c_int
        }
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_narsil_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    if (*quest.offset(12 as libc::c_int as isize)).status as libc::c_int >=
           1 as libc::c_int &&
           ((*quest.offset(12 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(17 as libc::c_int,
                 Some(quest_narsil_move_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"narsil_move\x00" as *const u8 as *const libc::c_char);
    }
    if (*quest.offset(12 as libc::c_int as isize)).status as libc::c_int ==
           0 as libc::c_int {
        add_hook(16 as libc::c_int,
                 Some(quest_narsil_identify_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"narsil_id\x00" as *const u8 as *const libc::c_char);
    }
    add_hook(12 as libc::c_int,
             Some(quest_narsil_dump_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_),
             b"narsil_dump\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_thrain_death_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut m_idx: s32b = 0;
    let mut r: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    m_idx = get_next_arg(fmt);
    if (*quest.offset(19 as libc::c_int as isize)).status as libc::c_int >=
           5 as libc::c_int ||
           dun_level as libc::c_int !=
               (*quest.offset(19 as libc::c_int as
                                  isize)).data[0 as libc::c_int as usize] ||
           dungeon_type as libc::c_int != 23 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    m_ptr = &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    if (*m_ptr).r_idx as libc::c_int !=
           test_monster_name(b"Dwar, Dog Lord of Waw\x00" as *const u8 as
                                 *const libc::c_char) &&
           (*m_ptr).r_idx as libc::c_int !=
               test_monster_name(b"Hoarmurath of Dir\x00" as *const u8 as
                                     *const libc::c_char) {
        return 0 as libc::c_int as bool_
    }
    let ref mut fresh30 =
        (*quest.offset(19 as libc::c_int as
                           isize)).data[2 as libc::c_int as usize];
    *fresh30 += 1;
    if (*quest.offset(19 as libc::c_int as
                          isize)).data[2 as libc::c_int as usize] <
           2 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    cmsg_print(11 as libc::c_int as byte_hack,
               b"The magic hiding the room dissipates.\x00" as *const u8 as
                   *const libc::c_char);
    x = 0 as libc::c_int;
    while x < cur_wid as libc::c_int {
        y = 0 as libc::c_int;
        while y < cur_hgt as libc::c_int {
            let mut c_ptr: *mut cave_type =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type;
            if !((*c_ptr).mimic as libc::c_int != 61 as libc::c_int) {
                if !((*c_ptr).info as libc::c_int & 0x800 as libc::c_int == 0)
                   {
                    (*c_ptr).mimic = 0 as libc::c_int as byte_hack;
                    lite_spot(y, x);
                }
            }
            y += 1
        }
        x += 1
    }
    (*quest.offset(19 as libc::c_int as isize)).status =
        5 as libc::c_int as s16b;
    cmsg_print(11 as libc::c_int as byte_hack,
               b"Thrain speaks:\x00" as *const u8 as *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"\'Ah, at last you came to me!  But... I fear it is too late for me.\x00"
                   as *const u8 as *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"However your quest continues, you must beware for the Necromancer\x00"
                   as *const u8 as *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"is in fact Sauron, the Dark Lord! He stole the Ring of Durin and tortured\x00"
                   as *const u8 as *const libc::c_char);
    cmsg_print(11 as libc::c_int as byte_hack,
               b"me... arrgh... please make him pay!\'\x00" as *const u8 as
                   *const libc::c_char);
    r = m_max as libc::c_int - 1 as libc::c_int;
    while r >= 1 as libc::c_int {
        let mut m_ptr_0: *mut monster_type =
            &mut *m_list.offset(r as isize) as *mut monster_type;
        if !((*m_ptr_0).r_idx == 0) {
            if (*m_ptr_0).r_idx as libc::c_int ==
                   test_monster_name(b"Thrain, the King Under the Mountain\x00"
                                         as *const u8 as *const libc::c_char)
               {
                let mut x_0: libc::c_int = (*m_ptr_0).fx as libc::c_int;
                let mut y_0: libc::c_int = (*m_ptr_0).fy as libc::c_int;
                let mut i: libc::c_int = 0;
                let mut j: libc::c_int = 0;
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
                delete_monster_idx(r);
                i = x_0 - 1 as libc::c_int;
                while i <= x_0 + 1 as libc::c_int {
                    j = y_0 - 1 as libc::c_int;
                    while j <= y_0 + 1 as libc::c_int {
                        if j > 0 as libc::c_int && i > 0 as libc::c_int &&
                               j < cur_hgt as libc::c_int - 1 as libc::c_int
                               &&
                               i < cur_wid as libc::c_int - 1 as libc::c_int {
                            cave_set_feat(j, i, 0x1 as libc::c_int);
                        }
                        j += 1
                    }
                    i += 1
                }
                q_ptr = &mut forge;
                object_wipe(q_ptr);
                object_prep(q_ptr,
                            lookup_kind(32 as libc::c_int, 7 as libc::c_int)
                                as libc::c_int);
                (*q_ptr).number = 1 as libc::c_int as byte_hack;
                (*q_ptr).found = 6 as libc::c_int as byte_hack;
                create_artifact(q_ptr, 0 as libc::c_int as bool_,
                                1 as libc::c_int as bool_);
                (*q_ptr).art_name =
                    quark_add(b"of Thrain\x00" as *const u8 as
                                  *const libc::c_char) as u16b;
                drop_near(q_ptr, -(1 as libc::c_int), y_0, x_0);
                break ;
            }
        }
        r -= 1
    }
    del_hook(0 as libc::c_int,
             Some(quest_thrain_death_hook as
                      unsafe extern "C" fn(_: *mut libc::c_char) -> bool_));
    process_hooks_restart = 1 as libc::c_int;
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_thrain_gen_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut x: s32b = 0;
    let mut y: s32b = 0;
    let mut bx0: s32b = 0;
    let mut by0: s32b = 0;
    let mut xstart: libc::c_int = 0;
    let mut ystart: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut yval: libc::c_int = 0;
    let mut xval: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut xsize: libc::c_int = 0;
    let mut ysize: libc::c_int = 0;
    if dungeon_type as libc::c_int != 23 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*quest.offset(19 as libc::c_int as
                          isize)).data[0 as libc::c_int as usize] !=
           dun_level as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*quest.offset(19 as libc::c_int as
                          isize)).data[1 as libc::c_int as usize] != 0 {
        return 0 as libc::c_int as bool_
    }
    if ((*quest.offset(19 as libc::c_int as isize)).status as libc::c_int) <
           1 as libc::c_int ||
           (*quest.offset(19 as libc::c_int as isize)).status as libc::c_int
               >= 5 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    by0 = get_next_arg(fmt);
    bx0 = get_next_arg(fmt);
    get_map_size(b"thrain.map\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char, &mut ysize, &mut xsize);
    if room_alloc(xsize + 2 as libc::c_int, ysize + 2 as libc::c_int,
                  0 as libc::c_int as bool_, by0, bx0, &mut xval, &mut yval)
           == 0 {
        return 0 as libc::c_int as bool_
    }
    y1 = yval - ysize / 2 as libc::c_int;
    x1 = xval - xsize / 2 as libc::c_int;
    y2 = y1 + ysize - 1 as libc::c_int;
    x2 = x1 + xsize - 1 as libc::c_int;
    y = y1;
    while y <= y2 {
        x = x1;
        while x <= x2 {
            cave_set_feat(y, x,
                          floor_type[Rand_div(100 as libc::c_int) as usize] as
                              libc::c_int);
            let ref mut fresh31 = (*cave[y as usize].offset(x as isize)).info;
            *fresh31 =
                (*fresh31 as libc::c_int |
                     (0x8 as libc::c_int | 0x2 as libc::c_int)) as u16b;
            x += 1
        }
        y += 1
    }
    build_rectangle(y1 - 1 as libc::c_int, x1 - 1 as libc::c_int,
                    y2 + 1 as libc::c_int, x2 + 1 as libc::c_int,
                    feat_wall_outer as libc::c_int,
                    0x8 as libc::c_int | 0x2 as libc::c_int);
    set_mon_num_hook();
    get_mon_num_prep();
    xstart = x1;
    ystart = y1;
    init_flags = 0x4 as libc::c_int;
    process_dungeon_file(b"thrain.map\x00" as *const u8 as
                             *const libc::c_char, &mut ystart, &mut xstart,
                         cur_hgt as libc::c_int, cur_wid as libc::c_int,
                         1 as libc::c_int as bool_,
                         1 as libc::c_int as bool_);
    x = x1;
    while x < xstart {
        y = y1;
        while y < ystart {
            let ref mut fresh32 = (*cave[y as usize].offset(x as isize)).info;
            *fresh32 =
                (*fresh32 as libc::c_int |
                     (0x4 as libc::c_int | 0x8 as libc::c_int |
                          0x800 as libc::c_int)) as u16b;
            if (*cave[y as usize].offset(x as isize)).feat as libc::c_int ==
                   0xac as libc::c_int {
                let mut i: libc::c_int = 0;
                *m_allow_special.offset(test_monster_name(b"Thrain, the King Under the Mountain\x00"
                                                              as *const u8 as
                                                              *const libc::c_char)
                                            as isize) =
                    1 as libc::c_int as bool_;
                i =
                    place_monster_one(y, x,
                                      test_monster_name(b"Thrain, the King Under the Mountain\x00"
                                                            as *const u8 as
                                                            *const libc::c_char),
                                      0 as libc::c_int,
                                      0 as libc::c_int as bool_,
                                      0 as libc::c_int) as libc::c_int;
                if i != 0 {
                    let ref mut fresh33 = (*m_list.offset(i as isize)).mflag;
                    *fresh33 |= 0x2 as libc::c_int
                }
                *m_allow_special.offset(test_monster_name(b"Thrain, the King Under the Mountain\x00"
                                                              as *const u8 as
                                                              *const libc::c_char)
                                            as isize) =
                    0 as libc::c_int as bool_
            }
            y += 1
        }
        x += 1
    }
    (*quest.offset(19 as libc::c_int as
                       isize)).data[1 as libc::c_int as usize] =
        1 as libc::c_int;
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_thrain_feeling_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    if dungeon_type as libc::c_int != 23 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*quest.offset(19 as libc::c_int as
                          isize)).data[0 as libc::c_int as usize] !=
           dun_level as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*quest.offset(19 as libc::c_int as isize)).status as libc::c_int !=
           0 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    cmsg_format(11 as libc::c_int as byte_hack,
                b"You hear someone shouting under the torture.\x00" as
                    *const u8 as *const libc::c_char);
    (*quest.offset(19 as libc::c_int as isize)).status =
        1 as libc::c_int as s16b;
    (*quest.offset(19 as libc::c_int as
                       isize)).init.expect("non-null function pointer")(19 as
                                                                            libc::c_int);
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_thrain_move_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    let mut y: s32b = 0;
    let mut x: s32b = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    y = get_next_arg(fmt);
    x = get_next_arg(fmt);
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    if dungeon_type as libc::c_int != 23 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*quest.offset(19 as libc::c_int as
                          isize)).data[0 as libc::c_int as usize] !=
           dun_level as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if ((*quest.offset(19 as libc::c_int as isize)).status as libc::c_int) <
           1 as libc::c_int ||
           (*quest.offset(19 as libc::c_int as isize)).status as libc::c_int
               >= 5 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    if (*c_ptr).info as libc::c_int & 0x800 as libc::c_int == 0 {
        return 0 as libc::c_int as bool_
    }
    if (*c_ptr).mimic as libc::c_int != 61 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    cmsg_print(11 as libc::c_int as byte_hack,
               b"The magic hiding the room dissipates.\x00" as *const u8 as
                   *const libc::c_char);
    x = 0 as libc::c_int;
    while x < cur_wid as libc::c_int {
        y = 0 as libc::c_int;
        while y < cur_hgt as libc::c_int {
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type;
            if !((*c_ptr).mimic as libc::c_int != 61 as libc::c_int) {
                if !((*c_ptr).info as libc::c_int & 0x800 as libc::c_int == 0)
                   {
                    (*c_ptr).mimic = 0 as libc::c_int as byte_hack;
                    lite_spot(y, x);
                }
            }
            y += 1
        }
        x += 1
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_thrain_turn_hook(mut fmt: *mut libc::c_char)
 -> bool_ {
    (*quest.offset(19 as libc::c_int as
                       isize)).data[1 as libc::c_int as usize] =
        0 as libc::c_int;
    (*quest.offset(19 as libc::c_int as
                       isize)).data[2 as libc::c_int as usize] =
        0 as libc::c_int;
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn quest_thrain_init_hook(mut q: libc::c_int) -> bool_ {
    if (*quest.offset(19 as libc::c_int as
                          isize)).data[0 as libc::c_int as usize] == 0 {
        (*quest.offset(19 as libc::c_int as
                           isize)).data[0 as libc::c_int as usize] =
            (*d_info.offset(23 as libc::c_int as isize)).mindepth as
                libc::c_int + 1 as libc::c_int +
                Rand_div(1 as libc::c_int +
                             ((*d_info.offset(23 as libc::c_int as
                                                  isize)).maxdepth as
                                  libc::c_int - 1 as libc::c_int) -
                             ((*d_info.offset(23 as libc::c_int as
                                                  isize)).mindepth as
                                  libc::c_int + 1 as libc::c_int));
        if wizard != 0 {
            message_add(1 as libc::c_int as byte_hack,
                        format(b"Thrain lvl %d\x00" as *const u8 as
                                   *const libc::c_char,
                               (*quest.offset(19 as libc::c_int as
                                                  isize)).data[0 as
                                                                   libc::c_int
                                                                   as usize])
                            as cptr, 6 as libc::c_int as byte_hack);
        }
    }
    if (*quest.offset(19 as libc::c_int as isize)).status as libc::c_int >=
           1 as libc::c_int &&
           ((*quest.offset(19 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(17 as libc::c_int,
                 Some(quest_thrain_move_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"thrain_move\x00" as *const u8 as *const libc::c_char);
    }
    if (*quest.offset(19 as libc::c_int as isize)).status as libc::c_int >=
           0 as libc::c_int &&
           ((*quest.offset(19 as libc::c_int as isize)).status as libc::c_int)
               < 5 as libc::c_int {
        add_hook(42 as libc::c_int,
                 Some(quest_thrain_turn_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"thrain_regen_lvl\x00" as *const u8 as *const libc::c_char);
        add_hook(8 as libc::c_int,
                 Some(quest_thrain_turn_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"thrain_new_lvl\x00" as *const u8 as *const libc::c_char);
        add_hook(7 as libc::c_int,
                 Some(quest_thrain_gen_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"thrain_gen\x00" as *const u8 as *const libc::c_char);
        add_hook(4 as libc::c_int,
                 Some(quest_thrain_feeling_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"thrain_feel\x00" as *const u8 as *const libc::c_char);
        add_hook(0 as libc::c_int,
                 Some(quest_thrain_death_hook as
                          unsafe extern "C" fn(_: *mut libc::c_char)
                              -> bool_),
                 b"thrain_death\x00" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int as bool_;
}
/* ************************** Other plot ***************************/
/* ************************ Khazad-dum plot ************************/
/* ************************ Minas Anor plot ************************/
/* ************************* Gondolin plot *************************/
/* ************************** Lorien plot **************************/
/* *************************** Bree plot ***************************/
/* *************************** Main plot ***************************/
/* ************************* Random Quests *************************/
