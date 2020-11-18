use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type lua_State;
    pub type lua_TObject;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn strtoul(_: *const libc::c_char, _: *mut *mut libc::c_char,
               _: libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_type(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_typename(L: *mut lua_State, t: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn lua_isnumber(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tag(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_lessthan(L: *mut lua_State, index1: libc::c_int,
                    index2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tonumber(L: *mut lua_State, index: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn lua_tostring(L: *mut lua_State, index: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn lua_touserdata(L: *mut lua_State, index: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn lua_topointer(L: *mut lua_State, index: libc::c_int)
     -> *const libc::c_void;
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
    fn lua_pushusertag(L: *mut lua_State, u: *mut libc::c_void,
                       tag: libc::c_int);
    #[no_mangle]
    fn lua_getglobal(L: *mut lua_State, name: *const libc::c_char);
    #[no_mangle]
    fn lua_rawget(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_rawgeti(L: *mut lua_State, index: libc::c_int, n: libc::c_int);
    #[no_mangle]
    fn lua_getglobals(L: *mut lua_State);
    #[no_mangle]
    fn lua_gettagmethod(L: *mut lua_State, tag: libc::c_int,
                        event: *const libc::c_char);
    #[no_mangle]
    fn lua_setglobal(L: *mut lua_State, name: *const libc::c_char);
    #[no_mangle]
    fn lua_rawset(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_rawseti(L: *mut lua_State, index: libc::c_int, n: libc::c_int);
    #[no_mangle]
    fn lua_setglobals(L: *mut lua_State);
    #[no_mangle]
    fn lua_settagmethod(L: *mut lua_State, tag: libc::c_int,
                        event: *const libc::c_char);
    #[no_mangle]
    fn lua_call(L: *mut lua_State, nargs: libc::c_int, nresults: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn lua_rawcall(L: *mut lua_State, nargs: libc::c_int,
                   nresults: libc::c_int);
    #[no_mangle]
    fn lua_dofile(L: *mut lua_State, filename: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn lua_dobuffer(L: *mut lua_State, buff: *const libc::c_char,
                    size: size_t, name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn lua_getgcthreshold(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_getgccount(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_setgcthreshold(L: *mut lua_State, newthreshold: libc::c_int);
    #[no_mangle]
    fn lua_newtag(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_next(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_copytagmethods(L: *mut lua_State, tagto: libc::c_int,
                          tagfrom: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_settag(L: *mut lua_State, tag: libc::c_int);
    #[no_mangle]
    fn lua_error(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    fn lua_getn(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_concat(L: *mut lua_State, n: libc::c_int);
    #[no_mangle]
    fn luaL_openlib(L: *mut lua_State, l: *const luaL_reg, n: libc::c_int);
    #[no_mangle]
    fn luaL_argerror(L: *mut lua_State, numarg: libc::c_int,
                     extramsg: *const libc::c_char);
    #[no_mangle]
    fn luaL_check_lstr(L: *mut lua_State, numArg: libc::c_int,
                       len: *mut size_t) -> *const libc::c_char;
    #[no_mangle]
    fn luaL_opt_lstr(L: *mut lua_State, numArg: libc::c_int,
                     def: *const libc::c_char, len: *mut size_t)
     -> *const libc::c_char;
    #[no_mangle]
    fn luaL_check_number(L: *mut lua_State, numArg: libc::c_int)
     -> libc::c_long;
    #[no_mangle]
    fn luaL_opt_number(L: *mut lua_State, numArg: libc::c_int,
                       def: libc::c_long) -> libc::c_long;
    #[no_mangle]
    fn luaL_checkstack(L: *mut lua_State, space: libc::c_int,
                       msg: *const libc::c_char);
    #[no_mangle]
    fn luaL_checktype(L: *mut lua_State, narg: libc::c_int, t: libc::c_int);
    #[no_mangle]
    fn luaL_checkany(L: *mut lua_State, narg: libc::c_int);
    #[no_mangle]
    fn luaL_verror(L: *mut lua_State, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn lua_getstack(L: *mut lua_State, level: libc::c_int, ar: *mut lua_Debug)
     -> libc::c_int;
    #[no_mangle]
    fn lua_getinfo(L: *mut lua_State, what: *const libc::c_char,
                   ar: *mut lua_Debug) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type size_t = libc::c_ulong;
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
/*
** $Id: lbaselib.c,v 1.2 2001/11/26 23:00:23 darkgod Exp $
** Basic library
** See Copyright Notice in lua.h
*/
/*
** If your system does not support `stderr', redefine this function, or
** redefine _ERRORMESSAGE so that it won't need _ALERT.
*/
unsafe extern "C" fn luaB__ALERT(mut L: *mut lua_State) -> libc::c_int {
    fputs(luaL_check_lstr(L, 1 as libc::c_int, 0 as *mut size_t), stderr);
    return 0 as libc::c_int;
}
/*
** Basic implementation of _ERRORMESSAGE.
** The library `liolib' redefines _ERRORMESSAGE for better error information.
*/
unsafe extern "C" fn luaB__ERRORMESSAGE(mut L: *mut lua_State)
 -> libc::c_int {
    luaL_checktype(L, 1 as libc::c_int, 3 as libc::c_int);
    lua_getglobal(L, b"_ALERT\x00" as *const u8 as *const libc::c_char);
    if lua_type(L, -(1 as libc::c_int)) == 5 as libc::c_int {
        /* avoid error loop if _ALERT is not defined */
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
        lua_pushstring(L, b"error: \x00" as *const u8 as *const libc::c_char);
        lua_pushvalue(L, 1 as libc::c_int);
        if lua_getstack(L, 1 as libc::c_int, &mut ar) != 0 {
            lua_getinfo(L, b"Sl\x00" as *const u8 as *const libc::c_char,
                        &mut ar);
            if !ar.source.is_null() && ar.currentline > 0 as libc::c_int {
                let mut buff: [libc::c_char; 100] = [0; 100];
                sprintf(buff.as_mut_ptr(),
                        b"\n  <%.70s: line %d>\x00" as *const u8 as
                            *const libc::c_char, ar.short_src.as_mut_ptr(),
                        ar.currentline);
                lua_pushstring(L, buff.as_mut_ptr());
                lua_concat(L, 2 as libc::c_int);
            }
        }
        lua_pushstring(L, b"\n\x00" as *const u8 as *const libc::c_char);
        lua_concat(L, 3 as libc::c_int);
        lua_rawcall(L, 1 as libc::c_int, 0 as libc::c_int);
    }
    return 0 as libc::c_int;
}
/*
** If your system does not support `stdout', you can just remove this function.
** If you need, you can define your own `print' function, following this
** model but changing `fputs' to put the strings at a proper place
** (a console window or a log file, for instance).
*/
unsafe extern "C" fn luaB_print(mut L: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = lua_gettop(L); /* number of arguments */
    let mut i: libc::c_int = 0;
    lua_getglobal(L, b"tostring\x00" as *const u8 as *const libc::c_char);
    i = 1 as libc::c_int;
    while i <= n {
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        /* pop result */
        lua_pushvalue(L, -(1 as libc::c_int)); /* function to be called */
        lua_pushvalue(L, i); /* value to print */
        lua_rawcall(L, 1 as libc::c_int, 1 as libc::c_int); /* get result */
        s = lua_tostring(L, -(1 as libc::c_int));
        if s.is_null() {
            lua_error(L,
                      b"`tostring\' must return a string to `print\'\x00" as
                          *const u8 as *const libc::c_char);
        }
        if i > 1 as libc::c_int {
            fputs(b"\t\x00" as *const u8 as *const libc::c_char, stdout);
        }
        fputs(s, stdout);
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        i += 1
    }
    fputs(b"\n\x00" as *const u8 as *const libc::c_char, stdout);
    return 0 as libc::c_int;
}
unsafe extern "C" fn luaB_tonumber(mut L: *mut lua_State) -> libc::c_int {
    let mut base: libc::c_int =
        luaL_opt_number(L, 2 as libc::c_int,
                        10 as libc::c_int as libc::c_long) as libc::c_int;
    if base == 10 as libc::c_int {
        /* standard conversion */
        luaL_checkany(L, 1 as libc::c_int);
        if lua_isnumber(L, 1 as libc::c_int) != 0 {
            lua_pushnumber(L, lua_tonumber(L, 1 as libc::c_int));
            return 1 as libc::c_int
        }
    } else {
        let mut s1: *const libc::c_char =
            luaL_check_lstr(L, 1 as libc::c_int, 0 as *mut size_t);
        let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut n: libc::c_ulong = 0;
        if !(2 as libc::c_int <= base && base <= 36 as libc::c_int) {
            luaL_argerror(L, 2 as libc::c_int,
                          b"base out of range\x00" as *const u8 as
                              *const libc::c_char);
        }
        n = strtoul(s1, &mut s2, base);
        if s1 != s2 as *const libc::c_char {
            /* at least one valid digit? */
            while *(*__ctype_b_loc()).offset(*s2 as libc::c_uchar as
                                                 libc::c_int as isize) as
                      libc::c_int &
                      _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 {
                s2 = s2.offset(1)
            } /* skip trailing spaces */
            if *s2 as libc::c_int == '\u{0}' as i32 {
                /* no invalid trailing characters? */
                lua_pushnumber(L, n as libc::c_long); /* else not a number */
                return 1 as libc::c_int
            }
        }
    }
    lua_pushnil(L);
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_error(mut L: *mut lua_State) -> libc::c_int {
    lua_error(L,
              luaL_opt_lstr(L, 1 as libc::c_int, 0 as *const libc::c_char,
                            0 as *mut size_t));
    return 0 as libc::c_int;
    /* to avoid warnings */
}
unsafe extern "C" fn luaB_setglobal(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 2 as libc::c_int); /* push table */
    lua_setglobal(L, luaL_check_lstr(L, 1 as libc::c_int, 0 as *mut size_t));
    return 0 as libc::c_int;
}
unsafe extern "C" fn luaB_getglobal(mut L: *mut lua_State) -> libc::c_int {
    lua_getglobal(L, luaL_check_lstr(L, 1 as libc::c_int, 0 as *mut size_t));
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_tag(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 1 as libc::c_int);
    lua_pushnumber(L, lua_tag(L, 1 as libc::c_int) as libc::c_long);
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_settag(mut L: *mut lua_State) -> libc::c_int {
    luaL_checktype(L, 1 as libc::c_int, 4 as libc::c_int);
    lua_pushvalue(L, 1 as libc::c_int);
    lua_settag(L, luaL_check_number(L, 2 as libc::c_int) as libc::c_int);
    return 1 as libc::c_int;
    /* return table */
}
unsafe extern "C" fn luaB_newtag(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L,
                   lua_newtag(L) as libc::c_long); /* value to be returned */
    return 1 as libc::c_int; /* new table of globals */
}
unsafe extern "C" fn luaB_copytagmethods(mut L: *mut lua_State)
 -> libc::c_int {
    lua_pushnumber(L,
                   lua_copytagmethods(L,
                                      luaL_check_number(L, 1 as libc::c_int)
                                          as libc::c_int,
                                      luaL_check_number(L, 2 as libc::c_int)
                                          as libc::c_int) as
                       libc::c_long); /* create a 2nd argument if there isn't one */
    return 1 as libc::c_int; /* results are already on the stack */
}
unsafe extern "C" fn luaB_globals(mut L: *mut lua_State) -> libc::c_int {
    lua_getglobals(L); /* at least one result to signal no errors */
    if !(lua_type(L, 1 as libc::c_int) == -(1 as libc::c_int)) {
        luaL_checktype(L, 1 as libc::c_int, 4 as libc::c_int);
        lua_pushvalue(L, 1 as libc::c_int);
        lua_setglobals(L);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_rawget(mut L: *mut lua_State) -> libc::c_int {
    luaL_checktype(L, 1 as libc::c_int, 4 as libc::c_int);
    luaL_checkany(L, 2 as libc::c_int);
    lua_rawget(L, -(2 as libc::c_int));
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_rawset(mut L: *mut lua_State) -> libc::c_int {
    luaL_checktype(L, 1 as libc::c_int, 4 as libc::c_int);
    luaL_checkany(L, 2 as libc::c_int);
    luaL_checkany(L, 3 as libc::c_int);
    lua_rawset(L, -(3 as libc::c_int));
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_settagmethod(mut L: *mut lua_State) -> libc::c_int {
    let mut tag: libc::c_int =
        luaL_check_number(L, 1 as libc::c_int) as libc::c_int;
    let mut event: *const libc::c_char =
        luaL_check_lstr(L, 2 as libc::c_int, 0 as *mut size_t);
    if !(lua_type(L, 3 as libc::c_int) == 5 as libc::c_int ||
             lua_type(L, 3 as libc::c_int) == 1 as libc::c_int) {
        luaL_argerror(L, 3 as libc::c_int,
                      b"function or nil expected\x00" as *const u8 as
                          *const libc::c_char);
    }
    if strcmp(event, b"gc\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        lua_error(L,
                  b"deprecated use: cannot set the `gc\' tag method from Lua\x00"
                      as *const u8 as *const libc::c_char);
    }
    lua_gettagmethod(L, tag, event);
    lua_pushvalue(L, 3 as libc::c_int);
    lua_settagmethod(L, tag, event);
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_gettagmethod(mut L: *mut lua_State) -> libc::c_int {
    let mut tag: libc::c_int =
        luaL_check_number(L, 1 as libc::c_int) as libc::c_int;
    let mut event: *const libc::c_char =
        luaL_check_lstr(L, 2 as libc::c_int, 0 as *mut size_t);
    if strcmp(event, b"gc\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        lua_error(L,
                  b"deprecated use: cannot get the `gc\' tag method from Lua\x00"
                      as *const u8 as *const libc::c_char);
    }
    lua_gettagmethod(L, tag, event);
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_gcinfo(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, lua_getgccount(L) as libc::c_long);
    lua_pushnumber(L, lua_getgcthreshold(L) as libc::c_long);
    return 2 as libc::c_int;
}
unsafe extern "C" fn luaB_collectgarbage(mut L: *mut lua_State)
 -> libc::c_int {
    lua_setgcthreshold(L,
                       luaL_opt_number(L, 1 as libc::c_int,
                                       0 as libc::c_int as libc::c_long) as
                           libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn luaB_type(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 1 as libc::c_int);
    lua_pushstring(L, lua_typename(L, lua_type(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_next(mut L: *mut lua_State) -> libc::c_int {
    luaL_checktype(L, 1 as libc::c_int, 4 as libc::c_int);
    lua_settop(L, 2 as libc::c_int);
    if lua_next(L, 1 as libc::c_int) != 0 {
        return 2 as libc::c_int
    } else { lua_pushnil(L); return 1 as libc::c_int };
}
unsafe extern "C" fn passresults(mut L: *mut lua_State,
                                 mut status: libc::c_int,
                                 mut oldtop: libc::c_int) -> libc::c_int {
    static mut errornames: [*const libc::c_char; 6] =
        [b"ok\x00" as *const u8 as *const libc::c_char,
         b"run-time error\x00" as *const u8 as *const libc::c_char,
         b"file error\x00" as *const u8 as *const libc::c_char,
         b"syntax error\x00" as *const u8 as *const libc::c_char,
         b"memory error\x00" as *const u8 as *const libc::c_char,
         b"error in error handling\x00" as *const u8 as *const libc::c_char];
    if status == 0 as libc::c_int {
        let mut nresults: libc::c_int = lua_gettop(L) - oldtop;
        if nresults > 0 as libc::c_int {
            return nresults
        } else {
            lua_pushusertag(L, 0 as *mut libc::c_void, 0 as libc::c_int);
            return 1 as libc::c_int
        }
    } else {
        /* error */
        lua_pushnil(L); /* error code */
        lua_pushstring(L, errornames[status as usize]);
        return 2 as libc::c_int
    };
}
unsafe extern "C" fn luaB_dostring(mut L: *mut lua_State) -> libc::c_int {
    let mut oldtop: libc::c_int = lua_gettop(L);
    let mut l: size_t = 0;
    let mut s: *const libc::c_char =
        luaL_check_lstr(L, 1 as libc::c_int, &mut l);
    if *s as libc::c_int == '\u{17}' as i32 {
        /* binary files start with ESC... */
        lua_error(L,
                  b"`dostring\' cannot run pre-compiled code\x00" as *const u8
                      as *const libc::c_char); /* index of old error method */
    }
    return passresults(L,
                       lua_dobuffer(L, s, l,
                                    luaL_opt_lstr(L, 2 as libc::c_int, s,
                                                  0 as *mut size_t)), oldtop);
}
unsafe extern "C" fn luaB_dofile(mut L: *mut lua_State) -> libc::c_int {
    let mut oldtop: libc::c_int = lua_gettop(L);
    let mut fname: *const libc::c_char =
        luaL_opt_lstr(L, 1 as libc::c_int, 0 as *const libc::c_char,
                      0 as *mut size_t);
    return passresults(L, lua_dofile(L, fname), oldtop);
}
unsafe extern "C" fn luaB_call(mut L: *mut lua_State) -> libc::c_int {
    let mut oldtop: libc::c_int = 0;
    let mut options: *const libc::c_char =
        luaL_opt_lstr(L, 3 as libc::c_int,
                      b"\x00" as *const u8 as *const libc::c_char,
                      0 as *mut size_t);
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    luaL_checktype(L, 2 as libc::c_int, 4 as libc::c_int);
    n = lua_getn(L, 2 as libc::c_int);
    if !(lua_type(L, 4 as libc::c_int) == -(1 as libc::c_int)) {
        /* set new error method */
        lua_getglobal(L,
                      b"_ERRORMESSAGE\x00" as *const u8 as
                          *const libc::c_char); /* get index */
        err = lua_gettop(L); /* top before function-call preparation */
        lua_pushvalue(L, 4 as libc::c_int);
        lua_setglobal(L,
                      b"_ERRORMESSAGE\x00" as *const u8 as
                          *const libc::c_char);
    }
    oldtop = lua_gettop(L);
    /* push function */
    lua_pushvalue(L, 1 as libc::c_int);
    luaL_checkstack(L, n,
                    b"too many arguments\x00" as *const u8 as
                        *const libc::c_char);
    i = 0 as libc::c_int;
    while i < n {
        /* push arg[1...n] */
        lua_rawgeti(L, 2 as libc::c_int, i + 1 as libc::c_int);
        i += 1
    }
    status = lua_call(L, n, -(1 as libc::c_int));
    if err != 0 as libc::c_int {
        /* restore old error method */
        lua_pushvalue(L, err);
        lua_setglobal(L,
                      b"_ERRORMESSAGE\x00" as *const u8 as
                          *const libc::c_char);
    }
    if status != 0 as libc::c_int {
        /* error in call? */
        if !strchr(options, 'x' as i32).is_null()
           { /* propagate error without additional messages */
            lua_pushnil(L); /* return nil to signal the error */
        } else { lua_error(L, 0 as *const libc::c_char); }
        return 1 as libc::c_int
    }
    if !strchr(options, 'p' as i32).is_null() {
        /* pack results? */
        lua_error(L,
                  b"deprecated option `p\' in `call\'\x00" as *const u8 as
                      *const libc::c_char);
    }
    return lua_gettop(L) - oldtop;
    /* results are already on the stack */
}
unsafe extern "C" fn luaB_tostring(mut L: *mut lua_State) -> libc::c_int {
    let mut buff: [libc::c_char; 64] = [0; 64]; /* function */
    match lua_type(L, 1 as libc::c_int) {
        2 => {
            lua_pushstring(L, lua_tostring(L, 1 as libc::c_int));
            return 1 as libc::c_int
        }
        3 => { lua_pushvalue(L, 1 as libc::c_int); return 1 as libc::c_int }
        4 => {
            sprintf(buff.as_mut_ptr(),
                    b"table: %p\x00" as *const u8 as *const libc::c_char,
                    lua_topointer(L, 1 as libc::c_int));
        }
        5 => {
            sprintf(buff.as_mut_ptr(),
                    b"function: %p\x00" as *const u8 as *const libc::c_char,
                    lua_topointer(L, 1 as libc::c_int));
        }
        0 => {
            sprintf(buff.as_mut_ptr(),
                    b"userdata(%d): %p\x00" as *const u8 as
                        *const libc::c_char, lua_tag(L, 1 as libc::c_int),
                    lua_touserdata(L, 1 as libc::c_int));
        }
        1 => {
            lua_pushstring(L, b"nil\x00" as *const u8 as *const libc::c_char);
            return 1 as libc::c_int
        }
        _ => {
            luaL_argerror(L, 1 as libc::c_int,
                          b"value expected\x00" as *const u8 as
                              *const libc::c_char);
        }
    }
    lua_pushstring(L, buff.as_mut_ptr());
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_foreachi(mut L: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    luaL_checktype(L, 1 as libc::c_int, 4 as libc::c_int);
    luaL_checktype(L, 2 as libc::c_int, 5 as libc::c_int);
    n = lua_getn(L, 1 as libc::c_int);
    i = 1 as libc::c_int;
    while i <= n {
        lua_pushvalue(L, 2 as libc::c_int);
        /* remove nil result */
        lua_pushnumber(L, i as libc::c_long); /* 1st argument */
        lua_rawgeti(L, 1 as libc::c_int, i); /* 2nd argument */
        lua_rawcall(L, 2 as libc::c_int, 1 as libc::c_int); /* first index */
        if !(lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int) {
            return 1 as libc::c_int
        }
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        i += 1
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn luaB_foreach(mut L: *mut lua_State) -> libc::c_int {
    luaL_checktype(L, 1 as libc::c_int, 4 as libc::c_int);
    luaL_checktype(L, 2 as libc::c_int, 5 as libc::c_int);
    lua_pushnil(L);
    loop  {
        if lua_next(L, 1 as libc::c_int) == 0 as libc::c_int {
            return 0 as libc::c_int
        }
        /* remove value and result */
        lua_pushvalue(L, 2 as libc::c_int); /* function */
        lua_pushvalue(L, -(3 as libc::c_int)); /* key */
        lua_pushvalue(L, -(3 as libc::c_int)); /* value */
        lua_rawcall(L, 2 as libc::c_int,
                    1 as libc::c_int); /* last argument: to be inserted */
        if !(lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int) {
            return 1 as libc::c_int
        } /* 2nd argument is the position */
        lua_settop(L, -(2 as libc::c_int) - 1 as libc::c_int);
    };
}
unsafe extern "C" fn luaB_assert(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 1 as libc::c_int);
    if lua_type(L, 1 as libc::c_int) == 1 as libc::c_int {
        luaL_verror(L,
                    b"assertion failed!  %.90s\x00" as *const u8 as
                        *const libc::c_char,
                    luaL_opt_lstr(L, 2 as libc::c_int,
                                  b"\x00" as *const u8 as *const libc::c_char,
                                  0 as *mut size_t));
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn luaB_getn(mut L: *mut lua_State) -> libc::c_int {
    luaL_checktype(L, 1 as libc::c_int, 4 as libc::c_int);
    lua_pushnumber(L, lua_getn(L, 1 as libc::c_int) as libc::c_long);
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_tinsert(mut L: *mut lua_State) -> libc::c_int {
    let mut v: libc::c_int = lua_gettop(L);
    let mut n: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    luaL_checktype(L, 1 as libc::c_int, 4 as libc::c_int);
    n = lua_getn(L, 1 as libc::c_int);
    if v == 2 as libc::c_int {
        /* called with only 2 arguments */
        pos = n + 1 as libc::c_int
    } else {
        pos = luaL_check_number(L, 2 as libc::c_int) as libc::c_int
    } /* t.n = n+1 */
    lua_pushstring(L, b"n\x00" as *const u8 as *const libc::c_char);
    lua_pushnumber(L, (n + 1 as libc::c_int) as libc::c_long);
    lua_rawset(L, 1 as libc::c_int);
    while n >= pos {
        lua_rawgeti(L, 1 as libc::c_int, n);
        lua_rawseti(L, 1 as libc::c_int, n + 1 as libc::c_int);
        n -= 1
        /* t[n+1] = t[n] */
    } /* t[pos] = v */
    lua_pushvalue(L, v); /* table is "empty" */
    lua_rawseti(L, 1 as libc::c_int, pos); /* result = t[pos] */
    return 0 as libc::c_int;
}
unsafe extern "C" fn luaB_tremove(mut L: *mut lua_State) -> libc::c_int {
    let mut pos: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    luaL_checktype(L, 1 as libc::c_int, 4 as libc::c_int);
    n = lua_getn(L, 1 as libc::c_int);
    pos =
        luaL_opt_number(L, 2 as libc::c_int, n as libc::c_long) as
            libc::c_int;
    if n <= 0 as libc::c_int { return 0 as libc::c_int }
    lua_rawgeti(L, 1 as libc::c_int, pos);
    while pos < n {
        lua_rawgeti(L, 1 as libc::c_int, pos + 1 as libc::c_int);
        lua_rawseti(L, 1 as libc::c_int, pos);
        pos += 1
        /* a[pos] = a[pos+1] */
    } /* t.n = n-1 */
    lua_pushstring(L,
                   b"n\x00" as *const u8 as
                       *const libc::c_char); /* t[n] = nil */
    lua_pushnumber(L, (n - 1 as libc::c_int) as libc::c_long);
    lua_rawset(L, 1 as libc::c_int);
    lua_pushnil(L);
    lua_rawseti(L, 1 as libc::c_int, n);
    return 1 as libc::c_int;
}
/*
** {======================================================
** Quicksort
** (based on `Algorithms in MODULA-3', Robert Sedgewick;
**  Addison-Wesley, 1993.)
*/
unsafe extern "C" fn set2(mut L: *mut lua_State, mut i: libc::c_int,
                          mut j: libc::c_int) {
    lua_rawseti(L, 1 as libc::c_int, i);
    lua_rawseti(L, 1 as libc::c_int, j);
}
unsafe extern "C" fn sort_comp(mut L: *mut lua_State, mut a: libc::c_int,
                               mut b: libc::c_int) -> libc::c_int {
    /* WARNING: the caller (auxsort) must ensure stack space */
    if !(lua_type(L, 2 as libc::c_int) == 1 as libc::c_int) {
        /* function? */
        let mut res: libc::c_int = 0; /* -1 to compensate function */
        lua_pushvalue(L,
                      2 as
                          libc::c_int); /* -2 to compensate function and `a' */
        lua_pushvalue(L, a - 1 as libc::c_int);
        lua_pushvalue(L, b - 2 as libc::c_int);
        lua_rawcall(L, 2 as libc::c_int, 1 as libc::c_int);
        res =
            !(lua_type(L, -(1 as libc::c_int)) == 1 as libc::c_int) as
                libc::c_int;
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        return res
    } else {
        /* a < b? */
        return lua_lessthan(L, a, b)
    };
}
unsafe extern "C" fn auxsort(mut L: *mut lua_State, mut l: libc::c_int,
                             mut u: libc::c_int) {
    while l < u {
        /* for tail recursion */
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        /* sort elements a[l], a[(l+u)/2] and a[u] */
        lua_rawgeti(L, 1 as libc::c_int, l);
        lua_rawgeti(L, 1 as libc::c_int, u);
        if sort_comp(L, -(1 as libc::c_int), -(2 as libc::c_int)) != 0 {
            /* a[u] < a[l]? */
            set2(L, l, u); /* swap a[l] - a[u] */
        } else {
            lua_settop(L,
                       -(2 as libc::c_int) -
                           1 as libc::c_int); /* only 2 elements */
        }
        if u - l == 1 as libc::c_int { break ; }
        i = (l + u) / 2 as libc::c_int;
        lua_rawgeti(L, 1 as libc::c_int, i);
        lua_rawgeti(L, 1 as libc::c_int, l);
        if sort_comp(L, -(2 as libc::c_int), -(1 as libc::c_int)) != 0 {
            /* a[i]<a[l]? */
            set2(L, i, l); /* remove a[l] */
        } else {
            lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
            lua_rawgeti(L, 1 as libc::c_int, u);
            if sort_comp(L, -(1 as libc::c_int), -(2 as libc::c_int)) != 0 {
                /* a[u]<a[i]? */
                set2(L, i, u); /* only 3 elements */
            } else {
                lua_settop(L,
                           -(2 as libc::c_int) -
                               1 as libc::c_int); /* Pivot */
            }
        }
        if u - l == 2 as libc::c_int { break ; }
        lua_rawgeti(L, 1 as libc::c_int, i);
        lua_pushvalue(L, -(1 as libc::c_int));
        lua_rawgeti(L, 1 as libc::c_int, u - 1 as libc::c_int);
        set2(L, i, u - 1 as libc::c_int);
        /* a[l] <= P == a[u-1] <= a[u], only need to sort from l+1 to u-2 */
        i = l;
        j = u - 1 as libc::c_int;
        loop  {
            loop 
                 /* invariant: a[l..i] <= P <= a[j..u] */
                 /* repeat ++i until a[i] >= P */
                 {
                i += 1;
                lua_rawgeti(L, 1 as libc::c_int, i);
                if !(sort_comp(L, -(1 as libc::c_int), -(2 as libc::c_int)) !=
                         0) {
                    break ;
                }
                if i > u {
                    lua_error(L,
                              b"invalid order function for sorting\x00" as
                                  *const u8 as *const libc::c_char);
                }
                lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
                /* remove a[i] */
            }
            loop 
                 /* repeat --j until a[j] <= P */
                 {
                j -= 1;
                lua_rawgeti(L, 1 as libc::c_int, j);
                if !(sort_comp(L, -(3 as libc::c_int), -(1 as libc::c_int)) !=
                         0) {
                    break ;
                }
                if j < l {
                    lua_error(L,
                              b"invalid order function for sorting\x00" as
                                  *const u8 as *const libc::c_char);
                }
                lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
                /* remove a[j] */
            } /* pop pivot, a[i], a[j] */
            if j < i {
                lua_settop(L,
                           -(3 as libc::c_int) -
                               1 as
                                   libc::c_int); /* swap pivot (a[u-1]) with a[i] */
                break ;
            } else { set2(L, i, j); }
        }
        lua_rawgeti(L, 1 as libc::c_int, u - 1 as libc::c_int);
        lua_rawgeti(L, 1 as libc::c_int, i);
        set2(L, u - 1 as libc::c_int, i);
        /* a[l..i-1] <= a[i] == P <= a[i+1..u] */
    /* adjust so that smaller "half" is in [j..i] and larger one in [l..u] */
        if i - l < u - i {
            j = l;
            i = i - 1 as libc::c_int;
            l = i + 2 as libc::c_int
        } else { j = i + 1 as libc::c_int; i = u; u = j - 2 as libc::c_int }
        auxsort(L, j, i);
        /* call recursively the smaller one */
    };
    /* repeat the routine for the larger one */
}
unsafe extern "C" fn luaB_sort(mut L: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = 0;
    luaL_checktype(L, 1 as libc::c_int, 4 as libc::c_int);
    n = lua_getn(L, 1 as libc::c_int);
    if !(lua_type(L, 2 as libc::c_int) == -(1 as libc::c_int)) {
        /* is there a 2nd argument? */
        luaL_checktype(L, 2 as libc::c_int,
                       5 as
                           libc::c_int); /* make sure there is two arguments */
    }
    lua_settop(L, 2 as libc::c_int);
    auxsort(L, 1 as libc::c_int, n);
    return 0 as libc::c_int;
}
static mut deprecated_names: [luaL_reg; 4] =
    unsafe {
        [{
             let mut init =
                 luaL_reg{name:
                              b"foreachvar\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_foreach as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"nextvar\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_next as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"rawgetglobal\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_rawget as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"rawsetglobal\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_rawset as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         }]
    };
/*
** gives an explicit error in any attempt to call a deprecated function
*/
unsafe extern "C" fn deprecated_func(mut L: *mut lua_State) -> libc::c_int {
    luaL_verror(L,
                b"function `%.20s\' is deprecated\x00" as *const u8 as
                    *const libc::c_char,
                lua_tostring(L, -(1 as libc::c_int)));
    return 0 as libc::c_int;
    /* to avoid warnings */
}
unsafe extern "C" fn deprecated_funcs(mut L: *mut lua_State) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        lua_pushstring(L, deprecated_names[i as usize].name);
        lua_pushcclosure(L,
                         Some(deprecated_func as
                                  unsafe extern "C" fn(_: *mut lua_State)
                                      -> libc::c_int), 1 as libc::c_int);
        lua_setglobal(L, deprecated_names[i as usize].name);
        i += 1
    };
}
/* }====================================================== */
static mut base_funcs: [luaL_reg; 33] =
    unsafe {
        [{
             let mut init =
                 luaL_reg{name:
                              b"_ALERT\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB__ALERT as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"_ERRORMESSAGE\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB__ERRORMESSAGE as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"call\x00" as *const u8 as *const libc::c_char,
                          func:
                              Some(luaB_call as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"collectgarbage\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_collectgarbage as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"copytagmethods\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_copytagmethods as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"dofile\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_dofile as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"dostring\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_dostring as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"error\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_error as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"foreach\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_foreach as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"foreachi\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_foreachi as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"gcinfo\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_gcinfo as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"getglobal\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_getglobal as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"gettagmethod\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_gettagmethod as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"globals\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_globals as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"newtag\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_newtag as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"next\x00" as *const u8 as *const libc::c_char,
                          func:
                              Some(luaB_next as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"print\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_print as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"rawget\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_rawget as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"rawset\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_rawset as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"rawgettable\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_rawget as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"rawsettable\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_rawset as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"setglobal\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_setglobal as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"settag\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_settag as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"settagmethod\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_settagmethod as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"tag\x00" as *const u8 as *const libc::c_char,
                          func:
                              Some(luaB_tag as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"tonumber\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_tonumber as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"tostring\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_tostring as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"type\x00" as *const u8 as *const libc::c_char,
                          func:
                              Some(luaB_type as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"assert\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_assert as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"getn\x00" as *const u8 as *const libc::c_char,
                          func:
                              Some(luaB_getn as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"sort\x00" as *const u8 as *const libc::c_char,
                          func:
                              Some(luaB_sort as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"tinsert\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_tinsert as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"tremove\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(luaB_tremove as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         }]
    };
#[no_mangle]
pub unsafe extern "C" fn lua_baselibopen(mut L: *mut lua_State) {
    luaL_openlib(L, base_funcs.as_ptr(),
                 (::std::mem::size_of::<[luaL_reg; 33]>() as
                      libc::c_ulong).wrapping_div(::std::mem::size_of::<luaL_reg>()
                                                      as libc::c_ulong) as
                     libc::c_int);
    lua_pushstring(L, b"Lua 4.0\x00" as *const u8 as *const libc::c_char);
    lua_setglobal(L, b"_VERSION\x00" as *const u8 as *const libc::c_char);
    deprecated_funcs(L);
}
