use ::libc;
extern "C" {
    pub type lua_State;
    pub type lua_TObject;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_type(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_isnumber(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tonumber(L: *mut lua_State, index: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State);
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
    fn lua_getref(L: *mut lua_State, ref_0: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_newtable(L: *mut lua_State);
    #[no_mangle]
    fn lua_settable(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_rawcall(L: *mut lua_State, nargs: libc::c_int,
                   nresults: libc::c_int);
    #[no_mangle]
    fn luaL_openlib(L: *mut lua_State, l: *const luaL_reg, n: libc::c_int);
    #[no_mangle]
    fn luaL_argerror(L: *mut lua_State, numarg: libc::c_int,
                     extramsg: *const libc::c_char);
    #[no_mangle]
    fn luaL_opt_lstr(L: *mut lua_State, numArg: libc::c_int,
                     def: *const libc::c_char, len: *mut size_t)
     -> *const libc::c_char;
    #[no_mangle]
    fn luaL_check_number(L: *mut lua_State, numArg: libc::c_int)
     -> libc::c_long;
    #[no_mangle]
    fn luaL_checkany(L: *mut lua_State, narg: libc::c_int);
    #[no_mangle]
    fn lua_getstack(L: *mut lua_State, level: libc::c_int, ar: *mut lua_Debug)
     -> libc::c_int;
    #[no_mangle]
    fn lua_getinfo(L: *mut lua_State, what: *const libc::c_char,
                   ar: *mut lua_Debug) -> libc::c_int;
    #[no_mangle]
    fn lua_getlocal(L: *mut lua_State, ar: *const lua_Debug, n: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn lua_setlocal(L: *mut lua_State, ar: *const lua_Debug, n: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn lua_setcallhook(L: *mut lua_State, func: lua_Hook) -> lua_Hook;
    #[no_mangle]
    fn lua_setlinehook(L: *mut lua_State, func: lua_Hook) -> lua_Hook;
}
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
pub type lua_Hook
    =
    Option<unsafe extern "C" fn(_: *mut lua_State, _: *mut lua_Debug) -> ()>;
/*
** $Id: ldblib.c,v 1.2 2001/11/26 23:00:23 darkgod Exp $
** Interface from Lua to its debug API
** See Copyright Notice in lua.h
*/
unsafe extern "C" fn settabss(mut L: *mut lua_State,
                              mut i: *const libc::c_char,
                              mut v: *const libc::c_char) {
    lua_pushstring(L, i); /* level out of range */
    lua_pushstring(L, v);
    lua_settable(L, -(3 as libc::c_int));
}
unsafe extern "C" fn settabsi(mut L: *mut lua_State,
                              mut i: *const libc::c_char,
                              mut v: libc::c_int) {
    lua_pushstring(L, i);
    lua_pushnumber(L, v as libc::c_long);
    lua_settable(L, -(3 as libc::c_int));
}
unsafe extern "C" fn getinfo(mut L: *mut lua_State) -> libc::c_int {
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
    let mut options: *const libc::c_char =
        luaL_opt_lstr(L, 2 as libc::c_int,
                      b"flnSu\x00" as *const u8 as *const libc::c_char,
                      0 as *mut size_t);
    let mut buff: [libc::c_char; 20] = [0; 20];
    if lua_isnumber(L, 1 as libc::c_int) != 0 {
        if lua_getstack(L, lua_tonumber(L, 1 as libc::c_int) as libc::c_int,
                        &mut ar) == 0 {
            lua_pushnil(L);
            return 1 as libc::c_int
        }
    } else if lua_type(L, 1 as libc::c_int) == 5 as libc::c_int {
        lua_pushvalue(L, 1 as libc::c_int);
        sprintf(buff.as_mut_ptr(),
                b">%.10s\x00" as *const u8 as *const libc::c_char, options);
        options = buff.as_mut_ptr()
    } else {
        luaL_argerror(L, 1 as libc::c_int,
                      b"function or level expected\x00" as *const u8 as
                          *const libc::c_char);
    }
    if lua_getinfo(L, options, &mut ar) == 0 {
        luaL_argerror(L, 2 as libc::c_int,
                      b"invalid option\x00" as *const u8 as
                          *const libc::c_char);
    }
    lua_newtable(L);
    while *options != 0 {
        match *options as libc::c_int {
            83 => {
                settabss(L, b"source\x00" as *const u8 as *const libc::c_char,
                         ar.source);
                if !ar.source.is_null() {
                    settabss(L,
                             b"short_src\x00" as *const u8 as
                                 *const libc::c_char,
                             ar.short_src.as_mut_ptr());
                }
                settabsi(L,
                         b"linedefined\x00" as *const u8 as
                             *const libc::c_char, ar.linedefined);
                settabss(L, b"what\x00" as *const u8 as *const libc::c_char,
                         ar.what);
            }
            108 => {
                settabsi(L,
                         b"currentline\x00" as *const u8 as
                             *const libc::c_char, ar.currentline);
            }
            117 => {
                settabsi(L, b"nups\x00" as *const u8 as *const libc::c_char,
                         ar.nups);
            }
            110 => {
                settabss(L, b"name\x00" as *const u8 as *const libc::c_char,
                         ar.name);
                settabss(L,
                         b"namewhat\x00" as *const u8 as *const libc::c_char,
                         ar.namewhat);
            }
            102 => {
                lua_pushstring(L,
                               b"func\x00" as *const u8 as
                                   *const libc::c_char);
                lua_pushvalue(L, -(3 as libc::c_int));
                lua_settable(L, -(3 as libc::c_int));
            }
            _ => { }
        }
        options = options.offset(1)
    }
    return 1 as libc::c_int;
    /* return table */
}
unsafe extern "C" fn getlocal(mut L: *mut lua_State) -> libc::c_int {
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
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if lua_getstack(L, luaL_check_number(L, 1 as libc::c_int) as libc::c_int,
                    &mut ar) == 0 {
        /* level out of range? */
        luaL_argerror(L, 1 as libc::c_int,
                      b"level out of range\x00" as *const u8 as
                          *const libc::c_char);
    }
    name =
        lua_getlocal(L, &mut ar,
                     luaL_check_number(L, 2 as libc::c_int) as libc::c_int);
    if !name.is_null() {
        lua_pushstring(L, name);
        lua_pushvalue(L, -(2 as libc::c_int));
        return 2 as libc::c_int
    } else { lua_pushnil(L); return 1 as libc::c_int };
}
unsafe extern "C" fn setlocal(mut L: *mut lua_State) -> libc::c_int {
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
    if lua_getstack(L, luaL_check_number(L, 1 as libc::c_int) as libc::c_int,
                    &mut ar) == 0 {
        /* level out of range? */
        luaL_argerror(L, 1 as libc::c_int,
                      b"level out of range\x00" as *const u8 as
                          *const libc::c_char);
    }
    luaL_checkany(L, 3 as libc::c_int);
    lua_pushstring(L,
                   lua_setlocal(L, &mut ar,
                                luaL_check_number(L, 2 as libc::c_int) as
                                    libc::c_int));
    return 1 as libc::c_int;
}
/* dummy variables (to define unique addresses) */
static mut key1: libc::c_char = 0;
static mut key2: libc::c_char = 0;
unsafe extern "C" fn hookf(mut L: *mut lua_State,
                           mut key: *mut libc::c_void) {
    lua_getref(L, 0 as libc::c_int); /* pop result from gettable */
    lua_pushusertag(L, key, 0 as libc::c_int);
    lua_gettable(L, -(2 as libc::c_int));
    if lua_type(L, -(1 as libc::c_int)) == 5 as libc::c_int {
        lua_pushvalue(L, 1 as libc::c_int);
        lua_rawcall(L, 1 as libc::c_int, 0 as libc::c_int);
    } else { lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int); }
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    /* pop table */
}
unsafe extern "C" fn callf(mut L: *mut lua_State, mut ar: *mut lua_Debug) {
    lua_pushstring(L, (*ar).event); /* dup key */
    hookf(L,
          &mut key1 as *mut libc::c_char as
              *mut libc::c_void); /* get old value */
}
unsafe extern "C" fn linef(mut L: *mut lua_State, mut ar: *mut lua_Debug) {
    lua_pushnumber(L, (*ar).currentline as libc::c_long); /* key (again) */
    hookf(L, &mut key2 as *mut libc::c_char as *mut libc::c_void);
}
unsafe extern "C" fn sethook(mut L: *mut lua_State,
                             mut key: *mut libc::c_void, mut hook: lua_Hook,
                             mut sethookf:
                                 Option<unsafe extern "C" fn(_:
                                                                 *mut lua_State,
                                                             _: lua_Hook)
                                            -> lua_Hook>) {
    lua_settop(L, 1 as libc::c_int);
    if lua_type(L, 1 as libc::c_int) == 1 as libc::c_int {
        Some(sethookf.expect("non-null function pointer")).expect("non-null function pointer")(L,
                                                                                               None);
    } else if lua_type(L, 1 as libc::c_int) == 5 as libc::c_int {
        Some(sethookf.expect("non-null function pointer")).expect("non-null function pointer")(L,
                                                                                               hook);
    } else {
        luaL_argerror(L, 1 as libc::c_int,
                      b"function expected\x00" as *const u8 as
                          *const libc::c_char);
    }
    lua_getref(L, 0 as libc::c_int);
    lua_pushusertag(L, key, 0 as libc::c_int);
    lua_pushvalue(L, -(1 as libc::c_int));
    lua_gettable(L, -(3 as libc::c_int));
    lua_pushvalue(L, -(2 as libc::c_int));
    lua_pushvalue(L, 1 as libc::c_int);
    lua_settable(L, -(5 as libc::c_int));
    /* set new value */
}
unsafe extern "C" fn setcallhook(mut L: *mut lua_State) -> libc::c_int {
    sethook(L, &mut key1 as *mut libc::c_char as *mut libc::c_void,
            Some(callf as
                     unsafe extern "C" fn(_: *mut lua_State,
                                          _: *mut lua_Debug) -> ()),
            Some(lua_setcallhook as
                     unsafe extern "C" fn(_: *mut lua_State, _: lua_Hook)
                         -> lua_Hook));
    return 1 as libc::c_int;
}
unsafe extern "C" fn setlinehook(mut L: *mut lua_State) -> libc::c_int {
    sethook(L, &mut key2 as *mut libc::c_char as *mut libc::c_void,
            Some(linef as
                     unsafe extern "C" fn(_: *mut lua_State,
                                          _: *mut lua_Debug) -> ()),
            Some(lua_setlinehook as
                     unsafe extern "C" fn(_: *mut lua_State, _: lua_Hook)
                         -> lua_Hook));
    return 1 as libc::c_int;
}
static mut dblib: [luaL_reg; 5] =
    unsafe {
        [{
             let mut init =
                 luaL_reg{name:
                              b"getlocal\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(getlocal as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"getinfo\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(getinfo as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"setcallhook\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(setcallhook as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"setlinehook\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(setlinehook as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"setlocal\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(setlocal as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         }]
    };
#[no_mangle]
pub unsafe extern "C" fn lua_dblibopen(mut L: *mut lua_State) {
    luaL_openlib(L, dblib.as_ptr(),
                 (::std::mem::size_of::<[luaL_reg; 5]>() as
                      libc::c_ulong).wrapping_div(::std::mem::size_of::<luaL_reg>()
                                                      as libc::c_ulong) as
                     libc::c_int);
}
