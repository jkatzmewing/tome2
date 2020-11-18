use ::libc;
extern "C" {
    pub type lua_State;
    #[no_mangle]
    fn tolua_error(L: *mut lua_State, msg: *mut libc::c_char);
    #[no_mangle]
    fn tolua_isnoobj(L: *mut lua_State, narg: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tolua_istype(L: *mut lua_State, narg: libc::c_int, tag: libc::c_int,
                    dim: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tolua_pushvalue(L: *mut lua_State, lo: libc::c_int);
    #[no_mangle]
    fn tolua_pushstring(L: *mut lua_State, value: *const libc::c_char);
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State);
    #[no_mangle]
    fn lua_setglobal(L: *mut lua_State, name: *const libc::c_char);
    #[no_mangle]
    fn tolua_open(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn tolua_using(L: *mut lua_State, module: libc::c_int);
    #[no_mangle]
    fn tolua_class(L: *mut lua_State, derived: libc::c_int,
                   base: libc::c_int);
    #[no_mangle]
    fn tolua_instance(L: *mut lua_State, instance: libc::c_int,
                      classobj: libc::c_int);
    #[no_mangle]
    fn tolua_foreach(L: *mut lua_State, lo: libc::c_int, f: libc::c_int);
    #[no_mangle]
    fn tolua_type(L: *mut lua_State, lo: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn tolua_base(L: *mut lua_State, lo: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tolua_cast(L: *mut lua_State, lo: libc::c_int,
                  type_0: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn tolua_takeownership(L: *mut lua_State, lo: libc::c_int);
    #[no_mangle]
    fn tolua_module(L: *mut lua_State, name: *mut libc::c_char);
    #[no_mangle]
    fn tolua_function(L: *mut lua_State, parent: *mut libc::c_char,
                      name: *mut libc::c_char, func: lua_CFunction);
    #[no_mangle]
    fn tolua_getstring(L: *mut lua_State, narg: libc::c_int,
                       def: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn tolua_getvalue(L: *mut lua_State, narg: libc::c_int, def: libc::c_int)
     -> libc::c_int;
}
pub type lua_CFunction
    =
    Option<unsafe extern "C" fn(_: *mut lua_State) -> libc::c_int>;
/* function to register type */
unsafe extern "C" fn toluaI_reg_types(mut tolua_S: *mut lua_State) { }
/* function: tolua_using */
unsafe extern "C" fn toluaI_tolua_tolua_using00(mut tolua_S: *mut lua_State)
 -> libc::c_int {
    if tolua_isnoobj(tolua_S, 2 as libc::c_int) == 0 {
        tolua_error(tolua_S,
                    b"#ferror in function \'using\'.\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return 0 as libc::c_int
    } else {
        let mut module: libc::c_int =
            tolua_getvalue(tolua_S, 1 as libc::c_int, 0 as libc::c_int);
        tolua_using(tolua_S, module);
        return 0 as libc::c_int
    };
}
/* function: tolua_type */
unsafe extern "C" fn toluaI_tolua_tolua_type00(mut tolua_S: *mut lua_State)
 -> libc::c_int {
    if tolua_isnoobj(tolua_S, 2 as libc::c_int) == 0 {
        tolua_error(tolua_S,
                    b"#ferror in function \'type\'.\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return 0 as libc::c_int
    } else {
        let mut lo: libc::c_int =
            tolua_getvalue(tolua_S, 1 as libc::c_int, 0 as libc::c_int);
        let mut toluaI_ret: *mut libc::c_char =
            tolua_type(tolua_S, lo) as *mut libc::c_char;
        tolua_pushstring(tolua_S, toluaI_ret);
        return 1 as libc::c_int
    };
}
/* function: tolua_foreach */
unsafe extern "C" fn toluaI_tolua_tolua_foreach00(mut tolua_S: *mut lua_State)
 -> libc::c_int {
    if tolua_isnoobj(tolua_S, 3 as libc::c_int) == 0 {
        tolua_error(tolua_S,
                    b"#ferror in function \'foreach\'.\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return 0 as libc::c_int
    } else {
        let mut lo: libc::c_int =
            tolua_getvalue(tolua_S, 1 as libc::c_int, 0 as libc::c_int);
        let mut f: libc::c_int =
            tolua_getvalue(tolua_S, 2 as libc::c_int, 0 as libc::c_int);
        tolua_foreach(tolua_S, lo, f);
        return 0 as libc::c_int
    };
}
/* function: tolua_class */
unsafe extern "C" fn toluaI_tolua_tolua_class00(mut tolua_S: *mut lua_State)
 -> libc::c_int {
    if tolua_isnoobj(tolua_S, 3 as libc::c_int) == 0 {
        tolua_error(tolua_S,
                    b"#ferror in function \'class\'.\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return 0 as libc::c_int
    } else {
        let mut derived: libc::c_int =
            tolua_getvalue(tolua_S, 1 as libc::c_int, 0 as libc::c_int);
        let mut base: libc::c_int =
            tolua_getvalue(tolua_S, 2 as libc::c_int, 0 as libc::c_int);
        tolua_class(tolua_S, derived, base);
        return 0 as libc::c_int
    };
}
/* function: tolua_instance */
unsafe extern "C" fn toluaI_tolua_tolua_instance00(mut tolua_S:
                                                       *mut lua_State)
 -> libc::c_int {
    if tolua_isnoobj(tolua_S, 3 as libc::c_int) == 0 {
        tolua_error(tolua_S,
                    b"#ferror in function \'instance\'.\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return 0 as libc::c_int
    } else {
        let mut instance: libc::c_int =
            tolua_getvalue(tolua_S, 1 as libc::c_int, 0 as libc::c_int);
        let mut classobj: libc::c_int =
            tolua_getvalue(tolua_S, 2 as libc::c_int, 0 as libc::c_int);
        tolua_instance(tolua_S, instance, classobj);
        return 0 as libc::c_int
    };
}
/* function: tolua_base */
unsafe extern "C" fn toluaI_tolua_tolua_base00(mut tolua_S: *mut lua_State)
 -> libc::c_int {
    if tolua_isnoobj(tolua_S, 2 as libc::c_int) == 0 {
        tolua_error(tolua_S,
                    b"#ferror in function \'base\'.\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return 0 as libc::c_int
    } else {
        let mut lo: libc::c_int =
            tolua_getvalue(tolua_S, 1 as libc::c_int, 0 as libc::c_int);
        let mut toluaI_ret: libc::c_int = tolua_base(tolua_S, lo);
        tolua_pushvalue(tolua_S, toluaI_ret);
        return 1 as libc::c_int
    };
}
/* function: tolua_cast */
unsafe extern "C" fn toluaI_tolua_tolua_cast00(mut tolua_S: *mut lua_State)
 -> libc::c_int {
    if tolua_istype(tolua_S, 2 as libc::c_int, 3 as libc::c_int,
                    0 as libc::c_int) == 0 ||
           tolua_isnoobj(tolua_S, 3 as libc::c_int) == 0 {
        tolua_error(tolua_S,
                    b"#ferror in function \'cast\'.\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return 0 as libc::c_int
    } else {
        let mut lo: libc::c_int =
            tolua_getvalue(tolua_S, 1 as libc::c_int, 0 as libc::c_int);
        let mut type_0: *mut libc::c_char =
            tolua_getstring(tolua_S, 2 as libc::c_int,
                            0 as *const libc::c_char) as *mut libc::c_char;
        let mut toluaI_ret: libc::c_int = tolua_cast(tolua_S, lo, type_0);
        tolua_pushvalue(tolua_S, toluaI_ret);
        return 1 as libc::c_int
    };
}
/* function: tolua_takeownership */
unsafe extern "C" fn toluaI_tolua_tolua_takeownership00(mut tolua_S:
                                                            *mut lua_State)
 -> libc::c_int {
    if tolua_isnoobj(tolua_S, 2 as libc::c_int) == 0 {
        tolua_error(tolua_S,
                    b"#ferror in function \'takeownership\'.\x00" as *const u8
                        as *const libc::c_char as *mut libc::c_char);
        return 0 as libc::c_int
    } else {
        let mut lo: libc::c_int =
            tolua_getvalue(tolua_S, 1 as libc::c_int, 0 as libc::c_int);
        tolua_takeownership(tolua_S, lo);
        return 0 as libc::c_int
    };
}
/*
** Lua binding: tolua
** Generated automatically by tolua 4.0b on Tue Nov 14 14:18:50 2000.
*/
/* Exported function */
/* Open function */
#[no_mangle]
pub unsafe extern "C" fn tolua_tolua_open(mut tolua_S: *mut lua_State)
 -> libc::c_int {
    tolua_open(tolua_S);
    toluaI_reg_types(tolua_S);
    tolua_module(tolua_S,
                 b"tolua\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char);
    tolua_function(tolua_S,
                   b"tolua\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
                   b"using\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
                   Some(toluaI_tolua_tolua_using00 as
                            unsafe extern "C" fn(_: *mut lua_State)
                                -> libc::c_int));
    tolua_function(tolua_S,
                   b"tolua\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
                   b"type\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
                   Some(toluaI_tolua_tolua_type00 as
                            unsafe extern "C" fn(_: *mut lua_State)
                                -> libc::c_int));
    tolua_function(tolua_S,
                   b"tolua\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
                   b"foreach\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
                   Some(toluaI_tolua_tolua_foreach00 as
                            unsafe extern "C" fn(_: *mut lua_State)
                                -> libc::c_int));
    tolua_function(tolua_S,
                   b"tolua\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
                   b"class\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
                   Some(toluaI_tolua_tolua_class00 as
                            unsafe extern "C" fn(_: *mut lua_State)
                                -> libc::c_int));
    tolua_function(tolua_S,
                   b"tolua\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
                   b"instance\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
                   Some(toluaI_tolua_tolua_instance00 as
                            unsafe extern "C" fn(_: *mut lua_State)
                                -> libc::c_int));
    tolua_function(tolua_S,
                   b"tolua\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
                   b"base\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
                   Some(toluaI_tolua_tolua_base00 as
                            unsafe extern "C" fn(_: *mut lua_State)
                                -> libc::c_int));
    tolua_function(tolua_S,
                   b"tolua\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
                   b"cast\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
                   Some(toluaI_tolua_tolua_cast00 as
                            unsafe extern "C" fn(_: *mut lua_State)
                                -> libc::c_int));
    tolua_function(tolua_S,
                   b"tolua\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
                   b"takeownership\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
                   Some(toluaI_tolua_tolua_takeownership00 as
                            unsafe extern "C" fn(_: *mut lua_State)
                                -> libc::c_int));
    return 1 as libc::c_int;
}
/* Close function */
#[no_mangle]
pub unsafe extern "C" fn tolua_tolua_close(mut tolua_S: *mut lua_State) {
    lua_pushnil(tolua_S);
    lua_setglobal(tolua_S, b"tolua\x00" as *const u8 as *const libc::c_char);
}
