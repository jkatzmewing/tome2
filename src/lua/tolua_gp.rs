use ::libc;
extern "C" {
    pub type lua_State;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State);
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_type(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tonumber(L: *mut lua_State, index: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn lua_tostring(L: *mut lua_State, index: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn lua_touserdata(L: *mut lua_State, index: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn lua_pushnumber(L: *mut lua_State, n: libc::c_long);
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    fn lua_pushusertag(L: *mut lua_State, u: *mut libc::c_void,
                       tag: libc::c_int);
    #[no_mangle]
    fn lua_gettable(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_settable(L: *mut lua_State, index: libc::c_int);
}
/* tolua: get & push functions.
** Support code for Lua bindings.
** Written by Waldemar Celes
** TeCGraf/PUC-Rio
** Jul 1998
** $Id: tolua_gp.c,v 1.2 2001/11/26 23:00:27 darkgod Exp $
*/
/* This code is free software; you can redistribute it and/or modify it. 
** The software provided hereunder is on an "as is" basis, and 
** the author has no obligation to provide maintenance, support, updates,
** enhancements, or modifications. 
*/
#[no_mangle]
pub unsafe extern "C" fn tolua_getnumber(mut L: *mut lua_State,
                                         mut narg: libc::c_int,
                                         mut def: libc::c_long)
 -> libc::c_long {
    return if lua_gettop(L) < abs(narg) {
               def
           } else { lua_tonumber(L, narg) };
}
#[no_mangle]
pub unsafe extern "C" fn tolua_getstring(mut L: *mut lua_State,
                                         mut narg: libc::c_int,
                                         mut def: *const libc::c_char)
 -> *const libc::c_char {
    return if lua_gettop(L) < abs(narg) {
               def
           } else { lua_tostring(L, narg) };
}
#[no_mangle]
pub unsafe extern "C" fn tolua_getuserdata(mut L: *mut lua_State,
                                           mut narg: libc::c_int,
                                           mut def: *mut libc::c_void)
 -> *mut libc::c_void {
    return if lua_gettop(L) < abs(narg) {
               def
           } else { lua_touserdata(L, narg) };
}
#[no_mangle]
pub unsafe extern "C" fn tolua_getusertype(mut L: *mut lua_State,
                                           mut narg: libc::c_int,
                                           mut def: *mut libc::c_void)
 -> *mut libc::c_void {
    return if lua_gettop(L) < abs(narg) {
               def
           } else { lua_touserdata(L, narg) };
}
#[no_mangle]
pub unsafe extern "C" fn tolua_getvalue(mut L: *mut lua_State,
                                        mut narg: libc::c_int,
                                        mut def: libc::c_int) -> libc::c_int {
    return if lua_gettop(L) < abs(narg) { def } else { narg };
}
#[no_mangle]
pub unsafe extern "C" fn tolua_getbool(mut L: *mut lua_State,
                                       mut narg: libc::c_int,
                                       mut def: libc::c_int) -> libc::c_int {
    return if lua_gettop(L) < abs(narg) {
               def
           } else if lua_type(L, narg) == 1 as libc::c_int {
               0 as libc::c_int
           } else {
               (lua_tonumber(L, narg) != 0 as libc::c_int as libc::c_long) as
                   libc::c_int
           };
}
#[no_mangle]
pub unsafe extern "C" fn tolua_getfieldnumber(mut L: *mut lua_State,
                                              mut lo: libc::c_int,
                                              mut index: libc::c_int,
                                              mut def: libc::c_long)
 -> libc::c_long {
    let mut v: libc::c_long = 0;
    lua_pushnumber(L, index as libc::c_long);
    lua_gettable(L, lo);
    v =
        if lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int {
            def
        } else { lua_tonumber(L, -(1 as libc::c_int)) };
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn tolua_getfieldstring(mut L: *mut lua_State,
                                              mut lo: libc::c_int,
                                              mut index: libc::c_int,
                                              mut def: *const libc::c_char)
 -> *const libc::c_char {
    let mut v: *const libc::c_char = 0 as *const libc::c_char;
    lua_pushnumber(L, index as libc::c_long);
    lua_gettable(L, lo);
    v =
        if lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int {
            def
        } else { lua_tostring(L, -(1 as libc::c_int)) };
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn tolua_getfielduserdata(mut L: *mut lua_State,
                                                mut lo: libc::c_int,
                                                mut index: libc::c_int,
                                                mut def: *mut libc::c_void)
 -> *mut libc::c_void {
    let mut v: *mut libc::c_void = 0 as *mut libc::c_void;
    lua_pushnumber(L, index as libc::c_long);
    lua_gettable(L, lo);
    v =
        if lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int {
            def
        } else { lua_touserdata(L, -(1 as libc::c_int)) };
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn tolua_getfieldusertype(mut L: *mut lua_State,
                                                mut lo: libc::c_int,
                                                mut index: libc::c_int,
                                                mut def: *mut libc::c_void)
 -> *mut libc::c_void {
    let mut v: *mut libc::c_void = 0 as *mut libc::c_void;
    lua_pushnumber(L, index as libc::c_long);
    lua_gettable(L, lo);
    v =
        if lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int {
            def
        } else { lua_touserdata(L, -(1 as libc::c_int)) };
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn tolua_getfieldvalue(mut L: *mut lua_State,
                                             mut lo: libc::c_int,
                                             mut index: libc::c_int,
                                             mut def: libc::c_int)
 -> libc::c_int {
    let mut v: libc::c_int = 0;
    lua_pushnumber(L, index as libc::c_long);
    lua_gettable(L, lo);
    v =
        if lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int {
            def
        } else { lo };
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn tolua_getfieldbool(mut L: *mut lua_State,
                                            mut lo: libc::c_int,
                                            mut index: libc::c_int,
                                            mut def: libc::c_int)
 -> libc::c_int {
    let mut v: libc::c_int = 0;
    lua_pushnumber(L, index as libc::c_long);
    lua_gettable(L, lo);
    v =
        if lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int {
            0 as libc::c_int
        } else {
            (lua_tonumber(L, -(1 as libc::c_int)) !=
                 0 as libc::c_int as libc::c_long) as libc::c_int
        };
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn tolua_pushnumber(mut L: *mut lua_State,
                                          mut value: libc::c_long) {
    lua_pushnumber(L, value);
}
#[no_mangle]
pub unsafe extern "C" fn tolua_pushstring(mut L: *mut lua_State,
                                          mut value: *const libc::c_char) {
    if value.is_null() { lua_pushnil(L); } else { lua_pushstring(L, value); };
}
#[no_mangle]
pub unsafe extern "C" fn tolua_pushuserdata(mut L: *mut lua_State,
                                            mut value: *mut libc::c_void) {
    if value.is_null() {
        lua_pushnil(L);
    } else { lua_pushusertag(L, value, 0 as libc::c_int); };
}
#[no_mangle]
pub unsafe extern "C" fn tolua_pushusertype(mut L: *mut lua_State,
                                            mut value: *mut libc::c_void,
                                            mut tag: libc::c_int) {
    if value.is_null() {
        lua_pushnil(L);
    } else { lua_pushusertag(L, value, tag); };
}
#[no_mangle]
pub unsafe extern "C" fn tolua_pushvalue(mut L: *mut lua_State,
                                         mut lo: libc::c_int) {
    lua_pushvalue(L, lo);
}
#[no_mangle]
pub unsafe extern "C" fn tolua_pushbool(mut L: *mut lua_State,
                                        mut value: libc::c_int) {
    if value != 0 {
        lua_pushnumber(L, value as libc::c_long);
    } else { lua_pushnil(L); };
}
#[no_mangle]
pub unsafe extern "C" fn tolua_pushfieldnumber(mut L: *mut lua_State,
                                               mut lo: libc::c_int,
                                               mut index: libc::c_int,
                                               mut v: libc::c_long) {
    lua_pushnumber(L, index as libc::c_long);
    tolua_pushnumber(L, v);
    lua_settable(L, lo);
}
#[no_mangle]
pub unsafe extern "C" fn tolua_pushfieldstring(mut L: *mut lua_State,
                                               mut lo: libc::c_int,
                                               mut index: libc::c_int,
                                               mut v: *mut libc::c_char) {
    lua_pushnumber(L, index as libc::c_long);
    tolua_pushstring(L, v);
    lua_settable(L, lo);
}
#[no_mangle]
pub unsafe extern "C" fn tolua_pushfielduserdata(mut L: *mut lua_State,
                                                 mut lo: libc::c_int,
                                                 mut index: libc::c_int,
                                                 mut v: *mut libc::c_void) {
    lua_pushnumber(L, index as libc::c_long);
    tolua_pushuserdata(L, v);
    lua_settable(L, lo);
}
#[no_mangle]
pub unsafe extern "C" fn tolua_pushfieldusertype(mut L: *mut lua_State,
                                                 mut lo: libc::c_int,
                                                 mut index: libc::c_int,
                                                 mut v: *mut libc::c_void,
                                                 mut tag: libc::c_int) {
    lua_pushnumber(L, index as libc::c_long);
    tolua_pushusertype(L, v, tag);
    lua_settable(L, lo);
}
#[no_mangle]
pub unsafe extern "C" fn tolua_pushfieldvalue(mut L: *mut lua_State,
                                              mut lo: libc::c_int,
                                              mut index: libc::c_int,
                                              mut v: libc::c_int) {
    lua_pushnumber(L, index as libc::c_long);
    lua_pushvalue(L, v);
    lua_settable(L, lo);
}
#[no_mangle]
pub unsafe extern "C" fn tolua_pushfieldbool(mut L: *mut lua_State,
                                             mut lo: libc::c_int,
                                             mut index: libc::c_int,
                                             mut v: libc::c_int) {
    lua_pushnumber(L, index as libc::c_long);
    tolua_pushbool(L, v);
    lua_settable(L, lo);
}
