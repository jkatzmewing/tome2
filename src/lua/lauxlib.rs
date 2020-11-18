use ::libc;
extern "C" {
    pub type lua_State;
    pub type lua_TObject;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn vsprintf(_: *mut libc::c_char, _: *const libc::c_char,
                _: ::std::ffi::VaList) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_insert(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_stackspace(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_type(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_typename(L: *mut lua_State, t: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn lua_isnumber(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tonumber(L: *mut lua_State, index: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn lua_tostring(L: *mut lua_State, index: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn lua_strlen(L: *mut lua_State, index: libc::c_int) -> size_t;
    #[no_mangle]
    fn lua_pushlstring(L: *mut lua_State, s: *const libc::c_char,
                       len: size_t);
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction,
                        n: libc::c_int);
    #[no_mangle]
    fn lua_setglobal(L: *mut lua_State, name: *const libc::c_char);
    #[no_mangle]
    fn lua_error(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    fn lua_concat(L: *mut lua_State, n: libc::c_int);
    #[no_mangle]
    fn lua_getstack(L: *mut lua_State, level: libc::c_int, ar: *mut lua_Debug)
     -> libc::c_int;
    #[no_mangle]
    fn lua_getinfo(L: *mut lua_State, what: *const libc::c_char,
                   ar: *mut lua_Debug) -> libc::c_int;
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
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
pub type lua_CFunction
    =
    Option<unsafe extern "C" fn(_: *mut lua_State) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_Debug {
    pub event: *const libc::c_char,
    pub currentline: libc::c_int,
    pub name: *const libc::c_char,
    pub namewhat: *const libc::c_char,
    pub nups: libc::c_int,
    pub linedefined: libc::c_int,
    pub what: *const libc::c_char,
    pub source: *const libc::c_char,
    pub short_src: [libc::c_char; 60],
    pub _func: *mut lua_TObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Buffer {
    pub p: *mut libc::c_char,
    pub level: libc::c_int,
    pub L: *mut lua_State,
    pub buffer: [libc::c_char; 8192],
}
/*
** $Id: lauxlib.c,v 1.3 2001/11/26 23:00:23 darkgod Exp $
** Auxiliary functions for building Lua libraries
** See Copyright Notice in lua.h
*/
/* This file uses only the official API of Lua.
** Any function declared here could be written as an application function.
** With care, these functions can be used by other libraries.
*/
#[no_mangle]
pub unsafe extern "C" fn luaL_findstring(mut name: *const libc::c_char,
                                         mut list: *const *const libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(*list.offset(i as isize)).is_null() {
        if strcmp(*list.offset(i as isize), name) == 0 as libc::c_int {
            return i
        }
        i += 1
    }
    return -(1 as libc::c_int);
    /* name not found */
}
#[no_mangle]
pub unsafe extern "C" fn luaL_argerror(mut L: *mut lua_State,
                                       mut narg: libc::c_int,
                                       mut extramsg: *const libc::c_char) {
    let mut ar: lua_Debug =
        lua_Debug{event: 0 as *const libc::c_char,
                  currentline: 0,
                  name: 0 as *const libc::c_char,
                  namewhat: 0 as *const libc::c_char,
                  nups: 0,
                  linedefined: 0,
                  what: 0 as *const libc::c_char,
                  source: 0 as *const libc::c_char,
                  short_src: [0; 60],
                  _func: 0 as *mut lua_TObject,};
    lua_getstack(L, 0 as libc::c_int, &mut ar);
    lua_getinfo(L, b"n\x00" as *const u8 as *const libc::c_char, &mut ar);
    if ar.name.is_null() {
        ar.name = b"?\x00" as *const u8 as *const libc::c_char
    }
    luaL_verror(L,
                b"bad argument #%d to `%.50s\' (%.100s)\x00" as *const u8 as
                    *const libc::c_char, narg, ar.name, extramsg);
}
unsafe extern "C" fn type_error(mut L: *mut lua_State, mut narg: libc::c_int,
                                mut t: libc::c_int) {
    let mut buff: [libc::c_char; 50] = [0; 50];
    sprintf(buff.as_mut_ptr(),
            b"%.8s expected, got %.8s\x00" as *const u8 as
                *const libc::c_char, lua_typename(L, t),
            lua_typename(L, lua_type(L, narg)));
    luaL_argerror(L, narg, buff.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checkstack(mut L: *mut lua_State,
                                         mut space: libc::c_int,
                                         mut mes: *const libc::c_char) {
    if space > lua_stackspace(L) {
        luaL_verror(L,
                    b"stack overflow (%.30s)\x00" as *const u8 as
                        *const libc::c_char, mes);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checktype(mut L: *mut lua_State,
                                        mut narg: libc::c_int,
                                        mut t: libc::c_int) {
    if lua_type(L, narg) != t { type_error(L, narg, t); };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checkany(mut L: *mut lua_State,
                                       mut narg: libc::c_int) {
    if lua_type(L, narg) == -(1 as libc::c_int) {
        luaL_argerror(L, narg,
                      b"value expected\x00" as *const u8 as
                          *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_check_lstr(mut L: *mut lua_State,
                                         mut narg: libc::c_int,
                                         mut len: *mut size_t)
 -> *const libc::c_char {
    let mut s: *const libc::c_char = lua_tostring(L, narg);
    if s.is_null() { type_error(L, narg, 3 as libc::c_int); }
    if !len.is_null() { *len = lua_strlen(L, narg) }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_opt_lstr(mut L: *mut lua_State,
                                       mut narg: libc::c_int,
                                       mut def: *const libc::c_char,
                                       mut len: *mut size_t)
 -> *const libc::c_char {
    if lua_type(L, narg) == -(1 as libc::c_int) {
        if !len.is_null() {
            *len =
                if !def.is_null() {
                    strlen(def)
                } else { 0 as libc::c_int as libc::c_ulong }
        }
        return def
    } else { return luaL_check_lstr(L, narg, len) };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_check_number(mut L: *mut lua_State,
                                           mut narg: libc::c_int)
 -> libc::c_long {
    let mut d: libc::c_long = lua_tonumber(L, narg);
    if d == 0 as libc::c_int as libc::c_long && lua_isnumber(L, narg) == 0 {
        /* avoid extra test when d is not 0 */
        type_error(L, narg, 2 as libc::c_int); /* put nothing on stack */
    } /* number of levels to concat */
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_opt_number(mut L: *mut lua_State,
                                         mut narg: libc::c_int,
                                         mut def: libc::c_long)
 -> libc::c_long {
    if lua_type(L, narg) == -(1 as libc::c_int) {
        return def
    } else { return luaL_check_number(L, narg) };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_openlib(mut L: *mut lua_State,
                                      mut l: *const luaL_reg,
                                      mut n: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        lua_pushcclosure(L, (*l.offset(i as isize)).func, 0 as libc::c_int);
        lua_setglobal(L, (*l.offset(i as isize)).name);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_verror(mut L: *mut lua_State,
                                     mut fmt: *const libc::c_char,
                                     mut args: ...) {
    let mut buff: [libc::c_char; 500] = [0; 500];
    let mut argp: ::std::ffi::VaListImpl;
    argp = args.clone();
    vsprintf(buff.as_mut_ptr(), fmt, argp.as_va_list());
    lua_error(L, buff.as_mut_ptr());
}
unsafe extern "C" fn emptybuffer(mut B: *mut luaL_Buffer) -> libc::c_int {
    let mut l: size_t =
        (*B).p.wrapping_offset_from((*B).buffer.as_mut_ptr()) as libc::c_long
            as size_t;
    if l == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int
    } else {
        lua_pushlstring((*B).L, (*B).buffer.as_mut_ptr(), l);
        (*B).p = (*B).buffer.as_mut_ptr();
        (*B).level += 1;
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn adjuststack(mut B: *mut luaL_Buffer) {
    if (*B).level > 1 as libc::c_int {
        let mut L: *mut lua_State = (*B).L;
        let mut toget: libc::c_int = 1 as libc::c_int;
        let mut toplen: size_t = lua_strlen(L, -(1 as libc::c_int));
        loop  {
            let mut l: size_t = lua_strlen(L, -(toget + 1 as libc::c_int));
            if !((*B).level - toget + 1 as libc::c_int >=
                     20 as libc::c_int / 2 as libc::c_int || toplen > l) {
                break ;
            }
            toplen =
                (toplen as libc::c_ulong).wrapping_add(l) as size_t as size_t;
            toget += 1;
            if !(toget < (*B).level) { break ; }
        }
        if toget >= 2 as libc::c_int {
            lua_concat(L, toget);
            (*B).level = (*B).level - toget + 1 as libc::c_int
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_prepbuffer(mut B: *mut luaL_Buffer)
 -> *mut libc::c_char {
    if emptybuffer(B) != 0 { adjuststack(B); }
    return (*B).buffer.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn luaL_addlstring(mut B: *mut luaL_Buffer,
                                         mut s: *const libc::c_char,
                                         mut l: size_t) {
    loop  {
        let fresh0 = l;
        l = l.wrapping_sub(1);
        if !(fresh0 != 0) { break ; }
        ((*B).p <
             &mut *(*B).buffer.as_mut_ptr().offset(8192 as libc::c_int as
                                                       isize) as
                 *mut libc::c_char || !luaL_prepbuffer(B).is_null()) as
            libc::c_int;
        let fresh1 = s;
        s = s.offset(1);
        let fresh2 = (*B).p;
        (*B).p = (*B).p.offset(1);
        *fresh2 = *fresh1
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_addstring(mut B: *mut luaL_Buffer,
                                        mut s: *const libc::c_char) {
    luaL_addlstring(B, s, strlen(s));
}
#[no_mangle]
pub unsafe extern "C" fn luaL_pushresult(mut B: *mut luaL_Buffer) {
    emptybuffer(B);
    if (*B).level == 0 as libc::c_int {
        lua_pushlstring((*B).L, 0 as *const libc::c_char,
                        0 as libc::c_int as size_t);
    } else if (*B).level > 1 as libc::c_int {
        lua_concat((*B).L, (*B).level);
    }
    (*B).level = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_addvalue(mut B: *mut luaL_Buffer) {
    let mut L: *mut lua_State = (*B).L;
    let mut vl: size_t = lua_strlen(L, -(1 as libc::c_int));
    if vl <=
           (8192 as libc::c_int as libc::c_long -
                (*B).p.wrapping_offset_from((*B).buffer.as_mut_ptr()) as
                    libc::c_long) as size_t {
        /* fit into buffer? */
        memcpy((*B).p as *mut libc::c_void,
               lua_tostring(L, -(1 as libc::c_int)) as *const libc::c_void,
               vl);
        (*B).p = (*B).p.offset(vl as isize);
        lua_settop(L,
                   -(1 as libc::c_int) - 1 as libc::c_int); /* put it there */
        /* remove from stack */
    } else {
        if emptybuffer(B) != 0 {
            lua_insert(L,
                       -(2 as libc::c_int)); /* put buffer before new value */
        } /* add new value into B stack */
        (*B).level += 1;
        adjuststack(B);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_buffinit(mut L: *mut lua_State,
                                       mut B: *mut luaL_Buffer) {
    (*B).L = L;
    (*B).p = (*B).buffer.as_mut_ptr();
    (*B).level = 0 as libc::c_int;
}
/* }====================================================== */
