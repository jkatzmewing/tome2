use ::libc;
extern "C" {
    pub type lua_State;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_insert(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_type(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tag(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tostring(L: *mut lua_State, index: libc::c_int)
     -> *const libc::c_char;
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
    fn lua_getref(L: *mut lua_State, ref_0: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_newtable(L: *mut lua_State);
    #[no_mangle]
    fn lua_setglobal(L: *mut lua_State, name: *const libc::c_char);
    #[no_mangle]
    fn lua_settable(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_rawset(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_settagmethod(L: *mut lua_State, tag: libc::c_int,
                        event: *const libc::c_char);
    #[no_mangle]
    fn lua_newtag(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_settag(L: *mut lua_State, tag: libc::c_int);
    #[no_mangle]
    fn lua_error(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    fn tolua_error(L: *mut lua_State, msg: *mut libc::c_char);
    #[no_mangle]
    fn tolua_getstring(L: *mut lua_State, narg: libc::c_int,
                       def: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn tolua_getuserdata(L: *mut lua_State, narg: libc::c_int,
                         def: *mut libc::c_void) -> *mut libc::c_void;
    #[no_mangle]
    fn toluaI_tm_global(L: *mut lua_State, lo: libc::c_int);
    #[no_mangle]
    fn toluaI_tm_module(L: *mut lua_State, lo: libc::c_int);
    #[no_mangle]
    fn toluaI_tm_class(L: *mut lua_State, lo: libc::c_int,
                       name: *mut libc::c_char);
    #[no_mangle]
    fn toluaI_tt_class(L: *mut lua_State, lo: libc::c_int,
                       derived: *mut libc::c_char, base: *mut libc::c_char);
}
pub type lua_CFunction
    =
    Option<unsafe extern "C" fn(_: *mut lua_State) -> libc::c_int>;
/* tolua: register functions
** Support code for Lua bindings.
** Written by Waldemar Celes
** TeCGraf/PUC-Rio
** Jul 1998
** $Id: tolua_rg.c,v 1.2 2001/11/26 23:00:27 darkgod Exp $
*/
/* This code is free software; you can redistribute it and/or modify it. 
** The software provided hereunder is on an "as is" basis, and 
** the author has no obligation to provide maintenance, support, updates,
** enhancements, or modifications. 
*/
#[no_mangle]
pub unsafe extern "C" fn tolua_globalvar(mut L: *mut lua_State,
                                         mut name: *mut libc::c_char,
                                         mut get: lua_CFunction,
                                         mut set: lua_CFunction) {
    lua_newtable(L); /* duplicate top */
    lua_pushstring(L,
                   b".get\x00" as *const u8 as
                       *const libc::c_char); /* field */
    lua_pushcclosure(L, get, 0 as libc::c_int); /* self */
    lua_settable(L, -(3 as libc::c_int)); /* duplicate top */
    if set.is_some() {
        lua_pushstring(L,
                       b".set\x00" as *const u8 as
                           *const libc::c_char); /* duplicate top */
        lua_pushcclosure(L, set, 0 as libc::c_int);
        lua_settable(L, -(3 as libc::c_int));
    }
    lua_pushvalue(L, -(1 as libc::c_int));
    lua_setglobal(L, name);
    toluaI_tm_global(L, lua_gettop(L));
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
}
unsafe extern "C" fn toluaI_const_global_array(mut L: *mut lua_State)
 -> libc::c_int {
    lua_error(L,
              b"value of const array cannot be changed\x00" as *const u8 as
                  *const libc::c_char);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tolua_globalarray(mut L: *mut lua_State,
                                           mut name: *mut libc::c_char,
                                           mut get: lua_CFunction,
                                           mut set: lua_CFunction) {
    let mut tag: libc::c_int = lua_newtag(L);
    lua_newtable(L);
    lua_settag(L, tag);
    lua_setglobal(L, name);
    lua_pushcclosure(L, get, 0 as libc::c_int);
    lua_settagmethod(L, tag,
                     b"gettable\x00" as *const u8 as *const libc::c_char);
    if set.is_some() {
        lua_pushcclosure(L, set, 0 as libc::c_int);
    } else {
        lua_pushcclosure(L,
                         Some(toluaI_const_global_array as
                                  unsafe extern "C" fn(_: *mut lua_State)
                                      -> libc::c_int), 0 as libc::c_int);
    }
    lua_settagmethod(L, tag,
                     b"settable\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn tolua_tablevar(mut L: *mut lua_State,
                                        mut table: *mut libc::c_char,
                                        mut name: *mut libc::c_char,
                                        mut get: lua_CFunction,
                                        mut set: lua_CFunction) {
    lua_getglobal(L, table);
    lua_pushstring(L, b".get\x00" as *const u8 as *const libc::c_char);
    lua_gettable(L, -(2 as libc::c_int));
    lua_pushstring(L, name);
    lua_pushcclosure(L, get, 0 as libc::c_int);
    lua_settable(L, -(3 as libc::c_int));
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    if set.is_some() {
        lua_pushstring(L, b".set\x00" as *const u8 as *const libc::c_char);
        lua_gettable(L, -(2 as libc::c_int));
        lua_pushstring(L, name);
        lua_pushcclosure(L, set, 0 as libc::c_int);
        lua_settable(L, -(3 as libc::c_int));
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    }
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
}
unsafe extern "C" fn toluaI_get_array(mut L: *mut lua_State) -> libc::c_int {
    let mut self_0: *mut libc::c_void =
        tolua_getuserdata(L, 1 as libc::c_int, 0 as *mut libc::c_void);
    let mut field: *const libc::c_char =
        tolua_getstring(L, 2 as libc::c_int, 0 as *const libc::c_char);
    if field.is_null() {
        tolua_error(L,
                    b"invalid \'field\' in accessing array\x00" as *const u8
                        as *const libc::c_char as *mut libc::c_char);
    }
    if self_0.is_null() {
        static mut msg: [libc::c_char; 8192] = [0; 8192];
        sprintf(msg.as_mut_ptr(),
                b"invalid \'self\' in accessing array \'%s\'\x00" as *const u8
                    as *const libc::c_char, field);
        tolua_error(L, msg.as_mut_ptr());
    }
    toluaI_getregistry(L,
                       b"tolua_tbl_itype\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushnumber(L, lua_tag(L, 1 as libc::c_int) as libc::c_long);
    lua_gettable(L, -(2 as libc::c_int));
    lua_getglobal(L, lua_tostring(L, -(1 as libc::c_int)));
    lua_pushstring(L, b".array\x00" as *const u8 as *const libc::c_char);
    lua_gettable(L, -(2 as libc::c_int));
    lua_pushvalue(L, 2 as libc::c_int);
    lua_gettable(L, -(2 as libc::c_int));
    lua_pushstring(L, b".self\x00" as *const u8 as *const libc::c_char);
    lua_pushvalue(L, 1 as libc::c_int);
    lua_rawset(L, -(3 as libc::c_int));
    return 1 as libc::c_int;
}
unsafe extern "C" fn toluaI_const_array(mut L: *mut lua_State)
 -> libc::c_int {
    lua_error(L,
              b"value of const field cannot be changed\x00" as *const u8 as
                  *const libc::c_char);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tolua_tablearray(mut L: *mut lua_State,
                                          mut table: *mut libc::c_char,
                                          mut name: *mut libc::c_char,
                                          mut get: lua_CFunction,
                                          mut set: lua_CFunction) {
    let mut tag: libc::c_int = lua_newtag(L);
    lua_getglobal(L, table);
    lua_pushstring(L, b".array\x00" as *const u8 as *const libc::c_char);
    lua_rawget(L, -(2 as libc::c_int));
    lua_pushstring(L, name);
    lua_newtable(L);
    lua_settag(L, tag);
    lua_settable(L, -(3 as libc::c_int));
    lua_settop(L, -(2 as libc::c_int) - 1 as libc::c_int);
    lua_pushcclosure(L, get, 0 as libc::c_int);
    lua_settagmethod(L, tag,
                     b"gettable\x00" as *const u8 as *const libc::c_char);
    if set.is_some() {
        lua_pushcclosure(L, set, 0 as libc::c_int);
    } else {
        lua_pushcclosure(L,
                         Some(toluaI_const_array as
                                  unsafe extern "C" fn(_: *mut lua_State)
                                      -> libc::c_int), 0 as libc::c_int);
    }
    lua_settagmethod(L, tag,
                     b"settable\x00" as *const u8 as *const libc::c_char);
    tolua_tablevar(L, table, name,
                   Some(toluaI_get_array as
                            unsafe extern "C" fn(_: *mut lua_State)
                                -> libc::c_int), None);
}
#[no_mangle]
pub unsafe extern "C" fn tolua_module(mut L: *mut lua_State,
                                      mut name: *mut libc::c_char) {
    lua_getglobal(L, name);
    if !(lua_type(L, -(1 as libc::c_int)) == 4 as libc::c_int) {
        lua_newtable(L);
        lua_pushstring(L, b".get\x00" as *const u8 as *const libc::c_char);
        lua_newtable(L);
        lua_settable(L, -(3 as libc::c_int));
        lua_pushstring(L, b".set\x00" as *const u8 as *const libc::c_char);
        lua_newtable(L);
        lua_settable(L, -(3 as libc::c_int));
        lua_pushvalue(L, -(1 as libc::c_int));
        lua_setglobal(L, name);
        toluaI_tm_module(L, lua_gettop(L));
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    }
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tolua_cclass(mut L: *mut lua_State,
                                      mut name: *mut libc::c_char,
                                      mut base: *mut libc::c_char) {
    let mut t: libc::c_int = 0;
    lua_newtable(L);
    lua_pushstring(L, b".get\x00" as *const u8 as *const libc::c_char);
    lua_newtable(L);
    lua_settable(L, -(3 as libc::c_int));
    lua_pushstring(L, b".set\x00" as *const u8 as *const libc::c_char);
    lua_newtable(L);
    lua_settable(L, -(3 as libc::c_int));
    lua_pushstring(L, b".array\x00" as *const u8 as *const libc::c_char);
    lua_newtable(L);
    lua_settable(L, -(3 as libc::c_int));
    if *base as libc::c_int != 0 as libc::c_int {
        lua_pushstring(L, b".base\x00" as *const u8 as *const libc::c_char);
        lua_getglobal(L, base);
        lua_rawset(L, -(3 as libc::c_int));
    }
    lua_pushvalue(L, -(1 as libc::c_int));
    lua_setglobal(L, name);
    t = lua_gettop(L);
    toluaI_tm_class(L, t, name);
    toluaI_tt_class(L, t, name, base);
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tolua_function(mut L: *mut lua_State,
                                        mut parent: *mut libc::c_char,
                                        mut name: *mut libc::c_char,
                                        mut func: lua_CFunction) {
    if parent.is_null() {
        lua_pushcclosure(L, func, 0 as libc::c_int);
        lua_setglobal(L, name);
    } else {
        lua_getglobal(L, parent);
        lua_pushstring(L, name);
        lua_pushcclosure(L, func, 0 as libc::c_int);
        lua_settable(L, -(3 as libc::c_int));
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tolua_constant(mut L: *mut lua_State,
                                        mut parent: *mut libc::c_char,
                                        mut name: *mut libc::c_char,
                                        mut value: libc::c_long) {
    if parent.is_null() {
        lua_pushnumber(L, value);
        lua_setglobal(L, name);
    } else {
        lua_getglobal(L, parent);
        lua_pushstring(L, name);
        lua_pushnumber(L, value);
        lua_settable(L, -(3 as libc::c_int));
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_setregistry(mut L: *mut lua_State,
                                            mut field: *mut libc::c_char) {
    lua_getref(L, 0 as libc::c_int);
    lua_insert(L, -(2 as libc::c_int));
    lua_pushstring(L, field);
    lua_insert(L, -(2 as libc::c_int));
    lua_settable(L, -(3 as libc::c_int));
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn toluaI_getregistry(mut L: *mut lua_State,
                                            mut field: *mut libc::c_char) {
    lua_getref(L, 0 as libc::c_int);
    lua_pushstring(L, field);
    lua_gettable(L, -(2 as libc::c_int));
    lua_insert(L, -(2 as libc::c_int));
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
}
