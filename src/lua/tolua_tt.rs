use ::libc;
extern "C" {
    pub type lua_State;
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_type(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_isnumber(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_isstring(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tag(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tonumber(L: *mut lua_State, index: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn lua_tostring(L: *mut lua_State, index: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushnumber(L: *mut lua_State, n: libc::c_long);
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    fn lua_gettable(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_newtable(L: *mut lua_State);
    #[no_mangle]
    fn lua_settable(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_newtag(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn toluaI_tm_instance(L: *mut lua_State, tag: libc::c_int,
                          lo: libc::c_int);
    #[no_mangle]
    fn toluaI_eh_set(L: *mut lua_State, narg: libc::c_int,
                     provided: *const libc::c_char,
                     expected: *const libc::c_char);
    #[no_mangle]
    fn toluaI_setregistry(L: *mut lua_State, field: *mut libc::c_char);
    #[no_mangle]
    fn toluaI_getregistry(L: *mut lua_State, field: *mut libc::c_char);
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
}
/* tolua: type & tag manipulation.
** Support code for Lua bindings.
** Written by Waldemar Celes
** TeCGraf/PUC-Rio
** Jul 1998
** $Id: tolua_tt.c,v 1.2 2001/11/26 23:00:27 darkgod Exp $
*/
/* This code is free software; you can redistribute it and/or modify it. 
** The software provided hereunder is on an "as is" basis, and 
** the author has no obligation to provide maintenance, support, updates,
** enhancements, or modifications. 
*/
/* Global tables created in Lua registry:
   tolua_tbl_itype: indexed by instance tags, stores the instance types.
   tolua_tbl_itag: indexed by instance types, stores the instance tags.
   tolua_tbl_const: indexed by constant tags, stores the tags.
   tolua_tbl_hierarchy: indexed by instance tags, stores the base tags.
*/
/* exported basic type tags */
#[no_mangle]
pub static mut tolua_tag_nil: libc::c_int = 0;
#[no_mangle]
pub static mut tolua_tag_number: libc::c_int = 0;
#[no_mangle]
pub static mut tolua_tag_string: libc::c_int = 0;
#[no_mangle]
pub static mut tolua_tag_userdata: libc::c_int = 0;
#[no_mangle]
pub static mut tolua_tag_table: libc::c_int = 0;
#[no_mangle]
pub static mut tolua_tag_function: libc::c_int = 0;
unsafe extern "C" fn gettype(mut L: *mut lua_State, mut tag: libc::c_int)
 -> *const libc::c_char {
    let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
    toluaI_getregistry(L,
                       b"tolua_tbl_itype\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushnumber(L, tag as libc::c_long);
    lua_gettable(L, -(2 as libc::c_int));
    type_0 = lua_tostring(L, -(1 as libc::c_int));
    if type_0.is_null() {
        type_0 = b"[undefined]\x00" as *const u8 as *const libc::c_char
    }
    lua_settop(L, -(2 as libc::c_int) - 1 as libc::c_int);
    return type_0;
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_tt_getobjtype(mut L: *mut lua_State,
                                              mut lo: libc::c_int)
 -> *const libc::c_char {
    if lua_gettop(L) < abs(lo) {
        return b"[no object]\x00" as *const u8 as *const libc::c_char
    } else { return gettype(L, lua_tag(L, lo)) };
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_tt_gettag(mut L: *mut lua_State,
                                          mut type_0: *mut libc::c_char)
 -> libc::c_int {
    let mut tag: libc::c_int = 0;
    toluaI_getregistry(L,
                       b"tolua_tbl_itag\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushstring(L, type_0);
    lua_gettable(L, -(2 as libc::c_int));
    tag = lua_tonumber(L, -(1 as libc::c_int)) as libc::c_int;
    lua_settop(L, -(2 as libc::c_int) - 1 as libc::c_int);
    return tag;
}
unsafe extern "C" fn basetag(mut L: *mut lua_State,
                             mut hierarchy: libc::c_int, mut tag: libc::c_int)
 -> libc::c_int {
    let mut btag: libc::c_int = 0;
    lua_pushnumber(L, tag as libc::c_long);
    lua_gettable(L, hierarchy);
    btag = lua_tonumber(L, -(1 as libc::c_int)) as libc::c_int;
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return btag;
}
unsafe extern "C" fn istype(mut L: *mut lua_State, mut lo: libc::c_int,
                            mut tag: libc::c_int) -> libc::c_int {
    let mut otag: libc::c_int = lua_tag(L, lo);
    if tag == otag {
        /* check simplest case */
        return 1 as libc::c_int
    } else if lua_type(L, lo) == 1 as libc::c_int && tag != 2 as libc::c_int
                  && tag != 4 as libc::c_int && tag != 5 as libc::c_int {
        return 1 as libc::c_int
    } else if tag == 3 as libc::c_int && lua_isstring(L, lo) != 0 ||
                  tag == 2 as libc::c_int && lua_isnumber(L, lo) != 0 {
        return 1 as libc::c_int
    } else if tag == 0 as libc::c_int && lua_type(L, lo) == 0 as libc::c_int {
        /* pointer to void* */
        return 1 as libc::c_int
    } else if tag ==
                  toluaI_tt_gettag(L,
                                   b"bool\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char) &&
                  otag == 2 as libc::c_int {
        return 1 as libc::c_int
    } else {
        /* if requested type is const, the non-const is an alternative type */
        let mut tag2: libc::c_int = 0;
        let mut tbl_hierarchy: libc::c_int = 0;
        toluaI_getregistry(L,
                           b"tolua_tbl_const\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char);
        lua_pushnumber(L, tag as libc::c_long);
        lua_gettable(L, -(2 as libc::c_int));
        tag2 = lua_tonumber(L, -(1 as libc::c_int)) as libc::c_int;
        lua_settop(L, -(2 as libc::c_int) - 1 as libc::c_int);
        if tag2 != 0 && tag2 == otag { return 1 as libc::c_int }
        /* check for base classes */
        toluaI_getregistry(L,
                           b"tolua_tbl_hierarchy\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char);
        tbl_hierarchy = lua_gettop(L);
        otag = basetag(L, tbl_hierarchy, otag);
        while otag != 0 {
            if tag == otag || tag2 != 0 && tag2 == otag { break ; }
            otag = basetag(L, tbl_hierarchy, otag)
        }
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        return (otag != 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_tt_init(mut L: *mut lua_State) {
    lua_newtable(L);
    toluaI_setregistry(L,
                       b"tolua_tbl_itype\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_newtable(L);
    toluaI_setregistry(L,
                       b"tolua_tbl_itag\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_newtable(L);
    toluaI_setregistry(L,
                       b"tolua_tbl_const\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_newtable(L);
    toluaI_setregistry(L,
                       b"tolua_tbl_hierarchy\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    /* set and register basic Lua type tag */
    toluaI_tt_register(L, 1 as libc::c_int,
                       b"nil\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char);
    toluaI_tt_register(L, 2 as libc::c_int,
                       b"number\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char);
    toluaI_tt_register(L, 3 as libc::c_int,
                       b"string\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char);
    toluaI_tt_register(L, 0 as libc::c_int,
                       b"userdata\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char);
    toluaI_tt_register(L, 4 as libc::c_int,
                       b"table\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char);
    toluaI_tt_register(L, 5 as libc::c_int,
                       b"function\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char);
    toluaI_tt_register(L, lua_newtag(L),
                       b"bool\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_tt_register(mut L: *mut lua_State,
                                            mut tag: libc::c_int,
                                            mut type_0: *mut libc::c_char) {
    toluaI_getregistry(L,
                       b"tolua_tbl_itype\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushnumber(L, tag as libc::c_long);
    lua_pushstring(L, type_0);
    lua_settable(L, -(3 as libc::c_int));
    toluaI_getregistry(L,
                       b"tolua_tbl_itag\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushstring(L, type_0);
    lua_pushnumber(L, tag as libc::c_long);
    lua_settable(L, -(3 as libc::c_int));
    lua_settop(L, -(2 as libc::c_int) - 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_tt_class(mut L: *mut lua_State,
                                         mut lo: libc::c_int,
                                         mut derived: *mut libc::c_char,
                                         mut base: *mut libc::c_char) {
    let mut cderived: *mut libc::c_char =
        toluaI_tt_concat(b"const \x00" as *const u8 as *const libc::c_char,
                         derived);
    let mut tag: libc::c_int = toluaI_tt_gettag(L, derived);
    let mut ctag: libc::c_int = toluaI_tt_gettag(L, cderived);
    toluaI_tm_instance(L, tag, lo);
    toluaI_tm_instance(L, ctag, lo);
    if *base as libc::c_int != 0 as libc::c_int {
        let mut cbase: *mut libc::c_char =
            toluaI_tt_concat(b"const \x00" as *const u8 as
                                 *const libc::c_char, base);
        let mut btag: libc::c_int = toluaI_tt_gettag(L, base);
        let mut cbtag: libc::c_int = toluaI_tt_gettag(L, cbase);
        toluaI_tt_sethierarchy(L, tag, btag);
        toluaI_tt_sethierarchy(L, ctag, cbtag);
    };
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_tt_sethierarchy(mut L: *mut lua_State,
                                                mut tag: libc::c_int,
                                                mut btag: libc::c_int) {
    toluaI_getregistry(L,
                       b"tolua_tbl_hierarchy\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushnumber(L, tag as libc::c_long);
    lua_pushnumber(L, btag as libc::c_long);
    lua_settable(L, -(3 as libc::c_int));
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_tt_concat(mut s1: *const libc::c_char,
                                          mut s2: *const libc::c_char)
 -> *mut libc::c_char {
    static mut s: [libc::c_char; 8192] = [0; 8192];
    if strlen(s1).wrapping_add(strlen(s2)) <
           8192 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"strlen(s1)+strlen(s2)<BUFSIZ\x00" as *const u8 as
                          *const libc::c_char,
                      b"/home/melpomene/tome2-rs/src/lua/tolua_tt.c\x00" as
                          *const u8 as *const libc::c_char,
                      207 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"char *toluaI_tt_concat(const char *, const char *)\x00")).as_ptr());
    }
    return strcat(strcpy(s.as_mut_ptr(), s1), s2);
}
#[no_mangle]
pub unsafe extern "C" fn tolua_usertype(mut L: *mut lua_State,
                                        mut type_0: *mut libc::c_char) {
    /* check if type is already registered */
    toluaI_getregistry(L,
                       b"tolua_tbl_itag\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushstring(L, type_0);
    lua_gettable(L, -(2 as libc::c_int));
    if lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int {
        let mut ctype: *mut libc::c_char =
            toluaI_tt_concat(b"const \x00" as *const u8 as
                                 *const libc::c_char, type_0);
        let mut tag: libc::c_int = lua_newtag(L);
        let mut ctag: libc::c_int = lua_newtag(L);
        toluaI_tt_register(L, tag, type_0);
        toluaI_tt_register(L, ctag, ctype);
        /* set const table */
        toluaI_getregistry(L,
                           b"tolua_tbl_const\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char);
        lua_pushnumber(L, ctag as libc::c_long);
        lua_pushnumber(L, tag as libc::c_long);
        lua_settable(L, -(3 as libc::c_int));
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    }
    lua_settop(L, -(2 as libc::c_int) - 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_tt_isusertype(mut L: *mut lua_State,
                                              mut lo: libc::c_int)
 -> libc::c_int {
    if lua_type(L, lo) == 0 as libc::c_int &&
           toluaI_tt_gettag(L,
                            b"tolua_tag_userdata\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char) !=
               lua_tag(L, lo) {
        let mut status: libc::c_int = 0;
        toluaI_getregistry(L,
                           b"tolua_tbl_itype\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char);
        lua_pushnumber(L, lua_tag(L, lo) as libc::c_long);
        lua_gettable(L, -(2 as libc::c_int));
        status =
            !(lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int) as
                libc::c_int;
        lua_settop(L, -(2 as libc::c_int) - 1 as libc::c_int);
        return status
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tolua_istype(mut L: *mut lua_State,
                                      mut narg: libc::c_int,
                                      mut tag: libc::c_int,
                                      mut def: libc::c_int) -> libc::c_int {
    if lua_gettop(L) < abs(narg) {
        if def == 0 as libc::c_int {
            toluaI_eh_set(L, narg, toluaI_tt_getobjtype(L, narg),
                          gettype(L, tag));
            return 0 as libc::c_int
        }
    } else if istype(L, narg, tag) == 0 {
        toluaI_eh_set(L, narg, toluaI_tt_getobjtype(L, narg),
                      gettype(L, tag));
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tolua_arrayistype(mut L: *mut lua_State,
                                           mut narg: libc::c_int,
                                           mut tag: libc::c_int,
                                           mut dim: libc::c_int,
                                           mut def: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < dim {
        let mut tf: libc::c_int = 0;
        lua_pushnumber(L, (i + 1 as libc::c_int) as libc::c_long);
        lua_gettable(L, narg);
        tf = lua_gettop(L);
        if istype(L, tf, tag) == 0 &&
               (def == 0 || !(lua_type(L, tf) == 1 as libc::c_int)) {
            static mut t1: [libc::c_char; 8192] = [0; 8192];
            static mut t2: [libc::c_char; 8192] = [0; 8192];
            sprintf(t1.as_mut_ptr(),
                    b"array of %s\x00" as *const u8 as *const libc::c_char,
                    toluaI_tt_getobjtype(L, tf));
            sprintf(t2.as_mut_ptr(),
                    b"array of %s (dimension=%d)\x00" as *const u8 as
                        *const libc::c_char, gettype(L, tag), dim);
            toluaI_eh_set(L, narg, t1.as_mut_ptr(), t2.as_mut_ptr());
            return 0 as libc::c_int
        }
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        i += 1
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tolua_isnoobj(mut L: *mut lua_State,
                                       mut narg: libc::c_int) -> libc::c_int {
    if lua_gettop(L) >= abs(narg) {
        toluaI_eh_set(L, narg, toluaI_tt_getobjtype(L, narg),
                      toluaI_tt_getobjtype(L,
                                           lua_gettop(L) + 1 as libc::c_int));
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
