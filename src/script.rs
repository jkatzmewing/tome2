use ::libc;
extern "C" {
    pub type lua_State;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn suffix(s: cptr, t: cptr) -> bool_;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    static mut ANGBAND_DIR_CORE: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_SCPT: cptr;
    #[no_mangle]
    fn init_corruptions(new_size: s16b);
    #[no_mangle]
    fn init_spells(new_size: s16b);
    #[no_mangle]
    fn init_schools(new_size: s16b);
    #[no_mangle]
    fn file_exist(buf: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn path_build(buf: *mut libc::c_char, max: libc::c_int, path: cptr,
                  file: cptr) -> errr;
    #[no_mangle]
    fn cmsg_print(color: byte_hack, msg: cptr);
    #[no_mangle]
    fn cmsg_format(color: byte_hack, fmt: cptr, _: ...);
    #[no_mangle]
    fn lua_open(stacksize: libc::c_int) -> *mut lua_State;
    #[no_mangle]
    fn lua_gettop(L_0: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L_0: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_isnumber(L_0: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_isstring(L_0: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_pushnil(L_0: *mut lua_State);
    #[no_mangle]
    fn lua_pushnumber(L_0: *mut lua_State, n: libc::c_long);
    #[no_mangle]
    fn lua_getglobal(L_0: *mut lua_State, name: *const libc::c_char);
    #[no_mangle]
    fn lua_call(L_0: *mut lua_State, nargs: libc::c_int,
                nresults: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_dofile(L_0: *mut lua_State, filename: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn lua_dostring(L_0: *mut lua_State, str: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn lua_baselibopen(L_0: *mut lua_State);
    #[no_mangle]
    fn lua_iolibopen(L_0: *mut lua_State);
    #[no_mangle]
    fn lua_strlibopen(L_0: *mut lua_State);
    #[no_mangle]
    fn lua_dblibopen(L_0: *mut lua_State);
    #[no_mangle]
    fn luaL_openlib(L_0: *mut lua_State, l: *const luaL_reg, n: libc::c_int);
    #[no_mangle]
    fn luaL_check_lstr(L_0: *mut lua_State, numArg: libc::c_int,
                       len: *mut size_t) -> *const libc::c_char;
    #[no_mangle]
    fn luaL_check_number(L_0: *mut lua_State, numArg: libc::c_int)
     -> libc::c_long;
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
    /* File: script.c */
    /* Purpose: scripting in lua */
    /*
 * Copyright (c) 2001 Dark God
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
    #[no_mangle]
    fn tolua_monster_open(L_0: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn tolua_player_open(L_0: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn tolua_player_c_open(L_0: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn tolua_util_open(L_0: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn tolua_z_pack_open(L_0: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn tolua_object_open(L_0: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn tolua_spells_open(L_0: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn tolua_quest_open(L_0: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn tolua_dungeon_open(L_0: *mut lua_State) -> libc::c_int;
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
pub type cptr = *const libc::c_char;
pub type errr = libc::c_int;
pub type byte_hack = libc::c_uchar;
pub type bool_ = libc::c_char;
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
pub struct luaL_reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
pub type lua_CFunction
    =
    Option<unsafe extern "C" fn(_: *mut lua_State) -> libc::c_int>;
/*
 * Lua state
 */
#[no_mangle]
pub static mut L: *mut lua_State = 0 as *const lua_State as *mut lua_State;
/* ToME Lua error message handler */
unsafe extern "C" fn tome_errormessage(mut L_0: *mut lua_State)
 -> libc::c_int {
    let mut buf: [libc::c_char; 200] = [0; 200];
    let mut str: cptr =
        luaL_check_lstr(L_0, 1 as libc::c_int, 0 as *mut size_t);
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    while *str.offset(i as isize) != 0 {
        if *str.offset(i as isize) as libc::c_int == '#' as i32 {
            let fresh0 = j;
            j = j + 1;
            buf[fresh0 as usize] = '$' as i32 as libc::c_char
        } else if *str.offset(i as isize) as libc::c_int != '\n' as i32 {
            let fresh1 = j;
            j = j + 1;
            buf[fresh1 as usize] = *str.offset(i as isize)
        } else {
            buf[j as usize] = '\u{0}' as i32 as libc::c_char;
            cmsg_format(10 as libc::c_int as byte_hack,
                        b"LUA: %s\x00" as *const u8 as *const libc::c_char,
                        buf.as_mut_ptr());
            j = 0 as libc::c_int
        }
        i += 1
    }
    buf[j as usize] = '\u{0}' as i32 as libc::c_char;
    cmsg_format(10 as libc::c_int as byte_hack,
                b"LUA: %s\x00" as *const u8 as *const libc::c_char,
                buf.as_mut_ptr());
    return 0 as libc::c_int;
}
static mut tome_iolib: [luaL_reg; 1] =
    unsafe {
        [{
             let mut init =
                 luaL_reg{name:
                              b"_ALERT\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(tome_errormessage as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         }]
    };
/*
 * Monadic bit negation operation
 * MONADIC(not,     ~)
 */
unsafe extern "C" fn int_not(mut L_0: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L_0, !luaL_check_number(L_0, 1 as libc::c_int));
    return 1 as libc::c_int;
}
/*
 * Dyadic integer modulus operation
 * DYADIC(mod,      %)
 */
unsafe extern "C" fn int_mod(mut L_0: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L_0,
                   luaL_check_number(L_0, 1 as libc::c_int) %
                       luaL_check_number(L_0, 2 as libc::c_int));
    return 1 as libc::c_int;
}
/*
 * Variable length bitwise AND operation
 * VARIADIC(and,    &)
 */
unsafe extern "C" fn int_and(mut L_0: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = lua_gettop(L_0);
    let mut i: libc::c_int = 0;
    let mut w: libc::c_long = luaL_check_number(L_0, 1 as libc::c_int);
    i = 2 as libc::c_int;
    while i <= n { w &= luaL_check_number(L_0, i); i += 1 }
    lua_pushnumber(L_0, w);
    return 1 as libc::c_int;
}
/*
 * Variable length bitwise OR operation
 * VARIADIC(or,     |)
 */
unsafe extern "C" fn int_or(mut L_0: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = lua_gettop(L_0);
    let mut i: libc::c_int = 0;
    let mut w: libc::c_long = luaL_check_number(L_0, 1 as libc::c_int);
    i = 2 as libc::c_int;
    while i <= n { w |= luaL_check_number(L_0, i); i += 1 }
    lua_pushnumber(L_0, w);
    return 1 as libc::c_int;
}
/*
 * Variable length bitwise XOR operation
 * VARIADIC(xor,    ^)
 */
unsafe extern "C" fn int_xor(mut L_0: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = lua_gettop(L_0);
    let mut i: libc::c_int = 0;
    let mut w: libc::c_long = luaL_check_number(L_0, 1 as libc::c_int);
    i = 2 as libc::c_int;
    while i <= n { w ^= luaL_check_number(L_0, i); i += 1 }
    lua_pushnumber(L_0, w);
    return 1 as libc::c_int;
}
/*
 * Binary left shift operation
 * TDYADIC(lshift,  <<, , u)
 */
unsafe extern "C" fn int_lshift(mut L_0: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L_0,
                   luaL_check_number(L_0, 1 as libc::c_int) <<
                       luaL_check_number(L_0, 2 as libc::c_int) as
                           libc::c_ulong);
    return 1 as libc::c_int;
}
/*
 * Binary logical right shift operation
 * TDYADIC(rshift,  >>, u, u)
 */
unsafe extern "C" fn int_rshift(mut L_0: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L_0,
                   (luaL_check_number(L_0, 1 as libc::c_int) as libc::c_ulong
                        >>
                        luaL_check_number(L_0, 2 as libc::c_int) as
                            libc::c_ulong) as libc::c_long);
    return 1 as libc::c_int;
}
/*
 * Binary arithmetic right shift operation
 * TDYADIC(arshift, >>, , u)
 */
unsafe extern "C" fn int_arshift(mut L_0: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L_0,
                   luaL_check_number(L_0, 1 as libc::c_int) >>
                       luaL_check_number(L_0, 2 as libc::c_int) as
                           libc::c_ulong);
    return 1 as libc::c_int;
}
static mut bitlib: [luaL_reg; 8] =
    unsafe {
        [{
             let mut init =
                 luaL_reg{name:
                              b"bnot\x00" as *const u8 as *const libc::c_char,
                          func:
                              Some(int_not as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"imod\x00" as *const u8 as *const libc::c_char,
                          func:
                              Some(int_mod as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"band\x00" as *const u8 as *const libc::c_char,
                          func:
                              Some(int_and as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"bor\x00" as *const u8 as *const libc::c_char,
                          func:
                              Some(int_or as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"bxor\x00" as *const u8 as *const libc::c_char,
                          func:
                              Some(int_xor as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"lshift\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(int_lshift as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"rshift\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(int_rshift as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"arshift\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(int_arshift as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         }]
    };
/*
 * Initialize lua scripting
 */
static mut init_lua_done: bool_ = 0 as libc::c_int as bool_;
#[no_mangle]
pub unsafe extern "C" fn init_lua() {
    /* Hack -- Do not initialize more than once */
    if init_lua_done != 0 { return }
    init_lua_done = 1 as libc::c_int as bool_;
    /* Start the interpreter with default stack size */
    L = lua_open(0 as libc::c_int);
    /* Register the Lua base libraries */
    lua_baselibopen(L);
    lua_strlibopen(L);
    lua_iolibopen(L);
    lua_dblibopen(L);
    /* Register tome lua debug library */
    luaL_openlib(L, tome_iolib.as_mut_ptr(),
                 (::std::mem::size_of::<[luaL_reg; 1]>() as
                      libc::c_ulong).wrapping_div(::std::mem::size_of::<luaL_reg>()
                                                      as libc::c_ulong) as
                     libc::c_int);
    /* Register the bitlib */
    luaL_openlib(L, bitlib.as_ptr(),
                 (::std::mem::size_of::<[luaL_reg; 8]>() as
                      libc::c_ulong).wrapping_div(::std::mem::size_of::<luaL_reg>()
                                                      as libc::c_ulong) as
                     libc::c_int);
    /* Register the ToME main APIs */
    tolua_player_open(L);
    tolua_player_c_open(L);
    tolua_util_open(L);
    tolua_z_pack_open(L);
    tolua_object_open(L);
    tolua_monster_open(L);
    tolua_spells_open(L);
    tolua_quest_open(L);
    tolua_dungeon_open(L);
}
#[no_mangle]
pub unsafe extern "C" fn init_lua_init() {
    let mut i: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    /* Load the first lua file */
    tome_dofile_anywhere(ANGBAND_DIR_CORE,
                         b"init.lua\x00" as *const u8 as *const libc::c_char
                             as *mut libc::c_char, 1 as libc::c_int as bool_);
    /* Finish up schools */
    max =
        exec_lua(b"return __schools_num\x00" as *const u8 as
                     *const libc::c_char as *mut libc::c_char);
    init_schools(max as s16b);
    i = 0 as libc::c_int;
    while i < max {
        exec_lua(format(b"finish_school(%d)\x00" as *const u8 as
                            *const libc::c_char, i));
        i += 1
    }
    /* Finish up the spells */
    max =
        exec_lua(b"return __tmp_spells_num\x00" as *const u8 as
                     *const libc::c_char as *mut libc::c_char);
    init_spells(max as s16b);
    i = 0 as libc::c_int;
    while i < max {
        exec_lua(format(b"finish_spell(%d)\x00" as *const u8 as
                            *const libc::c_char, i));
        i += 1
    }
    /* Finish up the corruptions */
    max =
        exec_lua(b"return __corruptions_max\x00" as *const u8 as
                     *const libc::c_char as *mut libc::c_char);
    init_corruptions(max as s16b);
}
#[no_mangle]
pub unsafe extern "C" fn tome_dofile(mut file: *mut libc::c_char) -> bool_ {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut oldtop: libc::c_int = lua_gettop(L);
    /* Build the filename */
    path_build(buf.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int, ANGBAND_DIR_SCPT, file as cptr);
    if file_exist(buf.as_mut_ptr()) == 0 {
        /* No lua source(.lua), maybe a compiled one(.luo) ? */
        if suffix(buf.as_mut_ptr() as cptr,
                  b".lua\x00" as *const u8 as *const libc::c_char) != 0 {
            let mut len: libc::c_int =
                strlen(buf.as_mut_ptr()) as libc::c_int;
            buf[(len - 1 as libc::c_int) as usize] =
                'o' as i32 as libc::c_char;
            if file_exist(buf.as_mut_ptr()) == 0 {
                cmsg_format(10 as libc::c_int as byte_hack,
                            b"tome_dofile(): file %s(%s) doesn\'t exist.\x00"
                                as *const u8 as *const libc::c_char, file,
                            buf.as_mut_ptr());
                return 0 as libc::c_int as bool_
            }
        }
    }
    lua_dofile(L, buf.as_mut_ptr());
    lua_settop(L, oldtop);
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn tome_dofile_anywhere(mut dir: cptr,
                                              mut file: *mut libc::c_char,
                                              mut test_exist: bool_)
 -> bool_ {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut oldtop: libc::c_int = lua_gettop(L);
    /* Build the filename */
    path_build(buf.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int, dir, file as cptr);
    if file_exist(buf.as_mut_ptr()) == 0 {
        /* No lua source(.lua), maybe a compiled one(.luo) ? */
        if suffix(buf.as_mut_ptr() as cptr,
                  b".lua\x00" as *const u8 as *const libc::c_char) != 0 {
            let mut len: libc::c_int =
                strlen(buf.as_mut_ptr()) as libc::c_int;
            buf[(len - 1 as libc::c_int) as usize] =
                'o' as i32 as libc::c_char;
            if file_exist(buf.as_mut_ptr()) == 0 {
                if test_exist != 0 {
                    cmsg_format(10 as libc::c_int as byte_hack,
                                b"tome_dofile_anywhere(): file %s(%s) doesn\'t exist in %s.\x00"
                                    as *const u8 as *const libc::c_char, dir,
                                file, buf.as_mut_ptr());
                }
                return 0 as libc::c_int as bool_
            }
        }
    }
    lua_dofile(L, buf.as_mut_ptr());
    lua_settop(L, oldtop);
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn exec_lua(mut file: *mut libc::c_char)
 -> libc::c_int {
    let mut oldtop: libc::c_int = lua_gettop(L);
    let mut res: libc::c_int = 0;
    if lua_dostring(L, file) == 0 {
        let mut size: libc::c_int = lua_gettop(L) - oldtop;
        res =
            tolua_getnumber(L, -size, 0 as libc::c_int as libc::c_long) as
                libc::c_int
    } else { res = 0 as libc::c_int }
    lua_settop(L, oldtop);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn string_exec_lua(mut file: *mut libc::c_char)
 -> cptr {
    let mut oldtop: libc::c_int = lua_gettop(L);
    let mut res: cptr = 0 as *const libc::c_char;
    if lua_dostring(L, file) == 0 {
        let mut size: libc::c_int = lua_gettop(L) - oldtop;
        res =
            tolua_getstring(L, -size,
                            b"\x00" as *const u8 as *const libc::c_char)
    } else { res = b"\x00" as *const u8 as *const libc::c_char }
    lua_settop(L, oldtop);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn dump_lua_stack(mut min: libc::c_int,
                                        mut max: libc::c_int) {
    let mut i: libc::c_int = 0;
    cmsg_print(11 as libc::c_int as byte_hack,
               b"lua_stack:\x00" as *const u8 as *const libc::c_char);
    i = min;
    while i <= max {
        if lua_isnumber(L, i) != 0 {
            cmsg_format(11 as libc::c_int as byte_hack,
                        b"%d [n] = %d\x00" as *const u8 as
                            *const libc::c_char, i,
                        tolua_getnumber(L, i,
                                        0 as libc::c_int as libc::c_long));
        } else if lua_isstring(L, i) != 0 {
            cmsg_format(11 as libc::c_int as byte_hack,
                        b"%d [s] = \'%s\'\x00" as *const u8 as
                            *const libc::c_char, i,
                        tolua_getstring(L, i, 0 as *const libc::c_char));
        }
        i += 1
    }
    cmsg_print(11 as libc::c_int as byte_hack,
               b"END lua_stack\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn call_lua(mut function: cptr, mut args: cptr,
                                  mut ret: cptr, mut args_0: ...) -> bool_ {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut nb: libc::c_int = 0 as libc::c_int;
    let mut nbr: libc::c_int = 0 as libc::c_int;
    let mut oldtop: libc::c_int = lua_gettop(L);
    let mut size: libc::c_int = 0;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args_0.clone();
    /* Push the function */
    lua_getglobal(L, function);
    /* Push and count the arguments */
    while *args.offset(i as isize) != 0 {
        let fresh2 = i;
        i = i + 1;
        match *args.offset(fresh2 as isize) as libc::c_int {
            100 | 108 => {
                tolua_pushnumber(L, ap.arg::<s32b>() as libc::c_long);
                nb += 1
            }
            115 => {
                tolua_pushstring(L, ap.arg::<*mut libc::c_char>());
                nb += 1
            }
            79 => {
                tolua_pushusertype(L,
                                   ap.arg::<*mut object_type>() as
                                       *mut libc::c_void,
                                   tolua_tag(L,
                                             b"object_type\x00" as *const u8
                                                 as *const libc::c_char as
                                                 *mut libc::c_char));
                nb += 1
            }
            77 => {
                tolua_pushusertype(L,
                                   ap.arg::<*mut monster_type>() as
                                       *mut libc::c_void,
                                   tolua_tag(L,
                                             b"monster_type\x00" as *const u8
                                                 as *const libc::c_char as
                                                 *mut libc::c_char));
                nb += 1
            }
            110 => { lua_pushnil(L); nb += 1 }
            40 | 41 | 44 | _ => { }
        }
    }
    /* Count returns */
    nbr = strlen(ret) as libc::c_int;
    /* Call the function */
    if lua_call(L, nb, nbr) != 0 {
        cmsg_format(10 as libc::c_int as byte_hack,
                    b"ERROR in lua_call while calling \'%s\' from call_lua. Things should start breaking up from now on!\x00"
                        as *const u8 as *const libc::c_char, function);
        return 0 as libc::c_int as bool_
    }
    /* Number of returned values, SHOULD be the same as nbr, but I'm paranoid */
    size = lua_gettop(L) - oldtop;
    /* Get the returns */
    i = 0 as libc::c_int;
    while *ret.offset(i as isize) != 0 {
        match *ret.offset(i as isize) as libc::c_int {
            100 | 108 => {
                let mut tmp: *mut s32b = ap.arg::<*mut s32b>();
                if lua_isnumber(L, -size + i) != 0 {
                    *tmp =
                        tolua_getnumber(L, -size + i,
                                        0 as libc::c_int as libc::c_long) as
                            s32b
                } else { *tmp = 0 as libc::c_int }
            }
            115 => {
                let mut tmp_0: *mut cptr = ap.arg::<*mut cptr>();
                if lua_isstring(L, -size + i) != 0 {
                    *tmp_0 =
                        tolua_getstring(L, -size + i,
                                        b"\x00" as *const u8 as
                                            *const libc::c_char)
                } else { *tmp_0 = 0 as cptr }
            }
            79 => {
                let mut tmp_1: *mut *mut object_type =
                    ap.arg::<*mut *mut object_type>();
                if tolua_istype(L, -size + i,
                                tolua_tag(L,
                                          b"object_type\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char),
                                0 as libc::c_int) != 0 {
                    *tmp_1 =
                        tolua_getuserdata(L, -size + i,
                                          0 as *mut libc::c_void) as
                            *mut object_type
                } else { *tmp_1 = 0 as *mut object_type }
            }
            77 => {
                let mut tmp_2: *mut *mut monster_type =
                    ap.arg::<*mut *mut monster_type>();
                if tolua_istype(L, -size + i,
                                tolua_tag(L,
                                          b"monster_type\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char),
                                0 as libc::c_int) != 0 {
                    *tmp_2 =
                        tolua_getuserdata(L, -size + i,
                                          0 as *mut libc::c_void) as
                            *mut monster_type
                } else { *tmp_2 = 0 as *mut monster_type }
            }
            _ => {
                cmsg_format(10 as libc::c_int as byte_hack,
                            b"ERROR in lua_call while calling \'%s\' from call_lua: Unknown return type \'%c\'\x00"
                                as *const u8 as *const libc::c_char, function,
                            *ret.offset(i as isize) as libc::c_int);
                return 0 as libc::c_int as bool_
            }
        }
        i += 1
    }
    lua_settop(L, oldtop);
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn get_lua_var(mut name: cptr, mut type_0: libc::c_char,
                                     mut arg: *mut libc::c_void) -> bool_ {
    let mut oldtop: libc::c_int = lua_gettop(L);
    let mut size: libc::c_int = 0;
    /* Push the function */
    lua_getglobal(L, name);
    size = lua_gettop(L) - oldtop;
    match type_0 as libc::c_int {
        100 | 108 => {
            let mut tmp: *mut s32b = arg as *mut s32b;
            if lua_isnumber(L, -size) != 0 {
                *tmp =
                    tolua_getnumber(L, -size,
                                    0 as libc::c_int as libc::c_long) as s32b
            } else { *tmp = 0 as libc::c_int }
        }
        115 => {
            let mut tmp_0: *mut cptr = arg as *mut cptr;
            if lua_isstring(L, -size) != 0 {
                *tmp_0 =
                    tolua_getstring(L, -size,
                                    b"\x00" as *const u8 as
                                        *const libc::c_char)
            } else { *tmp_0 = 0 as cptr }
        }
        79 => {
            let mut tmp_1: *mut *mut object_type =
                arg as *mut *mut object_type;
            if tolua_istype(L, -size,
                            tolua_tag(L,
                                      b"object_type\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char),
                            0 as libc::c_int) != 0 {
                *tmp_1 =
                    tolua_getuserdata(L, -size, 0 as *mut libc::c_void) as
                        *mut object_type
            } else { *tmp_1 = 0 as *mut object_type }
        }
        77 => {
            let mut tmp_2: *mut *mut monster_type =
                arg as *mut *mut monster_type;
            if tolua_istype(L, -size,
                            tolua_tag(L,
                                      b"monster_type\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char),
                            0 as libc::c_int) != 0 {
                *tmp_2 =
                    tolua_getuserdata(L, -size, 0 as *mut libc::c_void) as
                        *mut monster_type
            } else { *tmp_2 = 0 as *mut monster_type }
        }
        _ => {
            cmsg_format(10 as libc::c_int as byte_hack,
                        b"ERROR in get_lua_var while calling \'%s\': Unknown return type \'%c\'\x00"
                            as *const u8 as *const libc::c_char, name,
                        type_0 as libc::c_int);
            return 0 as libc::c_int as bool_
        }
    }
    lua_settop(L, oldtop);
    return 1 as libc::c_int as bool_;
}
