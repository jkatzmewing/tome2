use ::libc;
extern "C" {
    pub type lua_State;
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_type(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
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
    fn lua_touserdata(L: *mut lua_State, index: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State);
    #[no_mangle]
    fn lua_pushnumber(L: *mut lua_State, n: libc::c_long);
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction,
                        n: libc::c_int);
    #[no_mangle]
    fn lua_getglobal(L: *mut lua_State, name: *const libc::c_char);
    #[no_mangle]
    fn lua_gettable(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_rawget(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_rawset(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_call(L: *mut lua_State, nargs: libc::c_int, nresults: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn lua_setgcthreshold(L: *mut lua_State, newthreshold: libc::c_int);
    #[no_mangle]
    fn lua_newtag(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_settag(L: *mut lua_State, tag: libc::c_int);
    #[no_mangle]
    fn tolua_error(L: *mut lua_State, msg: *mut libc::c_char);
    #[no_mangle]
    fn tolua_pushusertype(L: *mut lua_State, value: *mut libc::c_void,
                          tag: libc::c_int);
    #[no_mangle]
    fn tolua_doclone(L: *mut lua_State, value: *mut libc::c_void,
                     tag: libc::c_int) -> *mut libc::c_void;
    #[no_mangle]
    fn toluaI_setregistry(L: *mut lua_State, field: *mut libc::c_char);
    #[no_mangle]
    fn toluaI_getregistry(L: *mut lua_State, field: *mut libc::c_char);
    #[no_mangle]
    fn toluaI_tm_init(L: *mut lua_State);
    #[no_mangle]
    fn toluaI_tm_linstance(L: *mut lua_State, tag: libc::c_int,
                           lo: libc::c_int);
    #[no_mangle]
    fn toluaI_tm_using(L: *mut lua_State, module: libc::c_int);
    #[no_mangle]
    fn toluaI_tm_setclass(L: *mut lua_State, lo: libc::c_int);
    #[no_mangle]
    fn toluaI_tm_pushmate(L: *mut lua_State, lo: libc::c_int);
    #[no_mangle]
    fn toluaI_tm_pushclass(L: *mut lua_State, lo: libc::c_int);
    #[no_mangle]
    fn toluaI_tt_init(L: *mut lua_State);
    #[no_mangle]
    fn toluaI_tt_isusertype(L: *mut lua_State, lo: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn toluaI_tt_gettag(L: *mut lua_State, type_0: *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn toluaI_tt_getobjtype(L: *mut lua_State, lo: libc::c_int)
     -> *const libc::c_char;
}
pub type lua_CFunction
    =
    Option<unsafe extern "C" fn(_: *mut lua_State) -> libc::c_int>;
/* tolua
** Support code for Lua bindings.
** Written by Waldemar Celes
** TeCGraf/PUC-Rio
** Jul 1998
** $Id: tolua_lb.c,v 1.2 2001/11/26 23:00:27 darkgod Exp $
*/
/* This code is free software; you can redistribute it and/or modify it. 
** The software provided hereunder is on an "as is" basis, and 
** the author has no obligation to provide maintenance, support, updates,
** enhancements, or modifications. 
*/
#[no_mangle]
pub unsafe extern "C" fn tolua_open(mut L: *mut lua_State) -> libc::c_int {
    extern "C" {
        #[link_name = "tolua_tolua_open"]
        fn tolua_tolua_open_0(L_0: *mut lua_State) -> libc::c_int;
    }
    /* check if alread opened */
    toluaI_getregistry(L,
                       b"TOLUA\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char);
    if lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int {
        lua_pushnumber(L, 1 as libc::c_int as libc::c_long);
        toluaI_setregistry(L,
                           b"TOLUA\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char);
        toluaI_tt_init(L);
        toluaI_tm_init(L);
        lua_getglobal(L, b"foreach\x00" as *const u8 as *const libc::c_char);
        toluaI_setregistry(L,
                           b"tolua_orig_foreach\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char);
        tolua_tolua_open_0(L);
        /* opens tolua own binding */
    } /* new tag of instances of that class */
    lua_settop(L,
               -(1 as libc::c_int) - 1 as libc::c_int); /* number and table */
    return 1 as libc::c_int; /* name */
}
#[no_mangle]
pub unsafe extern "C" fn tolua_using(mut L: *mut lua_State,
                                     mut module: libc::c_int) {
    toluaI_tm_using(L, module); /* value */
}
#[no_mangle]
pub unsafe extern "C" fn tolua_class(mut L: *mut lua_State,
                                     mut derived: libc::c_int,
                                     mut base: libc::c_int) {
    let mut tag: libc::c_int = lua_newtag(L); /* function */
    toluaI_tm_setclass(L, derived);
    toluaI_tm_linstance(L, tag, derived);
    lua_pushvalue(L, derived);
    lua_pushstring(L, b".base\x00" as *const u8 as *const libc::c_char);
    lua_pushvalue(L, base);
    lua_rawset(L, -(3 as libc::c_int));
    lua_pushstring(L, b".itag\x00" as *const u8 as *const libc::c_char);
    lua_pushnumber(L, tag as libc::c_long);
    lua_rawset(L, -(3 as libc::c_int));
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tolua_instance(mut L: *mut lua_State,
                                        mut instance: libc::c_int,
                                        mut classobj: libc::c_int) {
    let mut tag: libc::c_int = 0;
    lua_pushvalue(L, classobj);
    lua_pushstring(L, b".itag\x00" as *const u8 as *const libc::c_char);
    lua_gettable(L, -(2 as libc::c_int));
    tag = lua_tonumber(L, -(1 as libc::c_int)) as libc::c_int;
    lua_settop(L, -(2 as libc::c_int) - 1 as libc::c_int);
    if tag == 0 as libc::c_int {
        tolua_error(L,
                    b"unregistered \'classobj\' in function \'tolua_instance\'.\x00"
                        as *const u8 as *const libc::c_char as
                        *mut libc::c_char);
    }
    lua_pushvalue(L, instance);
    lua_settag(L, tag);
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
}
unsafe extern "C" fn filter(mut L: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = 1 as libc::c_int;
    let mut v: libc::c_int = 2 as libc::c_int;
    let mut f: libc::c_int = lua_gettop(L);
    /* do not pass string fields starting with a dot */
    if lua_isstring(L, n) == 0 ||
           *lua_tostring(L, n) as libc::c_int != '.' as i32 {
        lua_pushvalue(L, f); /* no field in mate table */
        lua_pushvalue(L, n);
        lua_pushvalue(L, v);
        lua_call(L, 2 as libc::c_int, 1 as libc::c_int);
    } else { lua_pushnil(L); }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tolua_foreach(mut L: *mut lua_State,
                                       mut lo: libc::c_int,
                                       mut f: libc::c_int) {
    if toluaI_tt_isusertype(L, lo) != 0 {
        toluaI_tm_pushmate(L, lo);
        if lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int {
            return
        } else { lo = lua_gettop(L) }
    }
    toluaI_getregistry(L,
                       b"tolua_orig_foreach\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushvalue(L, lo);
    lua_pushvalue(L, f);
    lua_pushcclosure(L,
                     Some(filter as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 1 as libc::c_int);
    lua_call(L, 2 as libc::c_int, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tolua_type(mut L: *mut lua_State,
                                    mut lo: libc::c_int)
 -> *const libc::c_char {
    return toluaI_tt_getobjtype(L, lo);
}
#[no_mangle]
pub unsafe extern "C" fn tolua_tag(mut L: *mut lua_State,
                                   mut type_0: *mut libc::c_char)
 -> libc::c_int {
    return toluaI_tt_gettag(L, type_0);
}
#[no_mangle]
pub unsafe extern "C" fn tolua_base(mut L: *mut lua_State,
                                    mut lo: libc::c_int) -> libc::c_int {
    if toluaI_tt_isusertype(L, lo) != 0 {
        toluaI_tm_pushclass(L, lo);
        return lua_gettop(L)
    } else if lua_type(L, lo) == 4 as libc::c_int {
        lua_pushvalue(L, lo);
        lua_pushstring(L, b".base\x00" as *const u8 as *const libc::c_char);
        lua_rawget(L, -(2 as libc::c_int));
        return -(1 as libc::c_int)
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn tolua_cast(mut L: *mut lua_State,
                                    mut lo: libc::c_int,
                                    mut type_0: *mut libc::c_char)
 -> libc::c_int {
    if lua_type(L, lo) == 0 as libc::c_int {
        tolua_pushusertype(L, lua_touserdata(L, lo),
                           toluaI_tt_gettag(L, type_0));
        return -(1 as libc::c_int)
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn tolua_takeownership(mut L: *mut lua_State,
                                             mut lo: libc::c_int) {
    if toluaI_tt_isusertype(L, lo) != 0 {
        /* force garbage collection to avoid C to reuse a to-be-collected address */
        lua_setgcthreshold(L, 0 as libc::c_int);
        tolua_doclone(L, lua_touserdata(L, lo), lua_tag(L, lo));
    } else {
        tolua_error(L,
                    b"cannot take ownership of specified obejct.\x00" as
                        *const u8 as *const libc::c_char as
                        *mut libc::c_char);
    };
}
