use ::libc;
extern "C" {
    pub type lua_State;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, index: libc::c_int);
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
    fn lua_error(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    fn toluaI_setregistry(L: *mut lua_State, field: *mut libc::c_char);
    #[no_mangle]
    fn toluaI_getregistry(L: *mut lua_State, field: *mut libc::c_char);
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
}
/* tolua: error handling
** Support code for Lua bindings.
** Written by Waldemar Celes
** TeCGraf/PUC-Rio
** Jul 1998
** $Id: tolua_eh.c,v 1.2 2001/11/26 23:00:27 darkgod Exp $
*/
/* This code is free software; you can redistribute it and/or modify it. 
** The software provided hereunder is on an "as is" basis, and 
** the author has no obligation to provide maintenance, support, updates,
** enhancements, or modifications. 
*/
/* registry fiels used to hold current error info 
   - tolua_err_narg: number of wrong argument
   - tolua_err_provided: provided type
   - tolua_err_expected: expected type
*/
#[no_mangle]
pub unsafe extern "C" fn toluaI_eh_set(mut L: *mut lua_State,
                                       mut narg: libc::c_int,
                                       mut provided: *const libc::c_char,
                                       mut expected: *const libc::c_char) {
    lua_pushnumber(L, narg as libc::c_long);
    toluaI_setregistry(L,
                       b"tolua_err_narg\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushstring(L, provided);
    toluaI_setregistry(L,
                       b"tolua_err_provided\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
    lua_pushstring(L, expected);
    toluaI_setregistry(L,
                       b"tolua_err_expected\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn tolua_error(mut L: *mut lua_State,
                                     mut msg: *mut libc::c_char) {
    if *msg.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32 {
        static mut buffer: [libc::c_char; 8192] = [0; 8192];
        let mut err_provided: *const libc::c_char = 0 as *const libc::c_char;
        let mut err_expected: *const libc::c_char = 0 as *const libc::c_char;
        toluaI_getregistry(L,
                           b"tolua_err_provided\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char);
        err_provided = lua_tostring(L, -(1 as libc::c_int));
        toluaI_getregistry(L,
                           b"tolua_err_expected\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char);
        err_expected = lua_tostring(L, -(1 as libc::c_int));
        lua_settop(L, -(2 as libc::c_int) - 1 as libc::c_int);
        if *msg.offset(1 as libc::c_int as isize) as libc::c_int == 'f' as i32
           {
            let mut err_narg: libc::c_int = 0;
            toluaI_getregistry(L,
                               b"tolua_err_narg\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char);
            err_narg = lua_tonumber(L, -(1 as libc::c_int)) as libc::c_int;
            lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
            sprintf(buffer.as_mut_ptr(),
                    b"%s\n     argument #%d is \'%s\'; \'%s\' expected.\n\x00"
                        as *const u8 as *const libc::c_char,
                    msg.offset(2 as libc::c_int as isize), err_narg,
                    err_provided, err_expected);
        } else if *msg.offset(1 as libc::c_int as isize) as libc::c_int ==
                      'v' as i32 {
            sprintf(buffer.as_mut_ptr(),
                    b"%s\n     value is \'%s\'; \'%s\' expected.\n\x00" as
                        *const u8 as *const libc::c_char,
                    msg.offset(2 as libc::c_int as isize), err_provided,
                    err_expected);
        }
        lua_error(L, buffer.as_mut_ptr());
    } else { lua_error(L, msg); };
}
