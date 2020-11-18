use ::libc;
extern "C" {
    pub type lua_State;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn toupper(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn lua_tostring(L: *mut lua_State, index: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn lua_strlen(L: *mut lua_State, index: libc::c_int) -> size_t;
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State);
    #[no_mangle]
    fn lua_pushnumber(L: *mut lua_State, n: libc::c_long);
    #[no_mangle]
    fn lua_pushlstring(L: *mut lua_State, s: *const libc::c_char,
                       len: size_t);
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    fn lua_rawcall(L: *mut lua_State, nargs: libc::c_int,
                   nresults: libc::c_int);
    #[no_mangle]
    fn lua_error(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    fn luaL_openlib(L: *mut lua_State, l: *const luaL_reg, n: libc::c_int);
    #[no_mangle]
    fn luaL_argerror(L: *mut lua_State, numarg: libc::c_int,
                     extramsg: *const libc::c_char);
    #[no_mangle]
    fn luaL_check_lstr(L: *mut lua_State, numArg: libc::c_int,
                       len: *mut size_t) -> *const libc::c_char;
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
    fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);
    #[no_mangle]
    fn luaL_prepbuffer(B: *mut luaL_Buffer) -> *mut libc::c_char;
    #[no_mangle]
    fn luaL_addlstring(B: *mut luaL_Buffer, s: *const libc::c_char,
                       l: size_t);
    #[no_mangle]
    fn luaL_addvalue(B: *mut luaL_Buffer);
    #[no_mangle]
    fn luaL_pushresult(B: *mut luaL_Buffer);
}
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
pub struct luaL_Buffer {
    pub p: *mut libc::c_char,
    pub level: libc::c_int,
    pub L: *mut lua_State,
    pub buffer: [libc::c_char; 8192],
}
/* arbitrary limit */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Capture {
    pub src_end: *const libc::c_char,
    pub level: libc::c_int,
    pub capture: [C2RustUnnamed_0; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub init: *const libc::c_char,
    pub len: libc::c_long,
}
/*
** $Id: lstrlib.c,v 1.3 2001/11/26 23:00:26 darkgod Exp $
** Standard library for string operations and pattern-matching
** See Copyright Notice in lua.h
*/
unsafe extern "C" fn str_len(mut L: *mut lua_State) -> libc::c_int {
    let mut l: size_t = 0;
    luaL_check_lstr(L, 1 as libc::c_int, &mut l);
    lua_pushnumber(L, l as libc::c_long);
    return 1 as libc::c_int;
}
unsafe extern "C" fn posrelat(mut pos: libc::c_long, mut len: size_t)
 -> libc::c_long {
    /* relative string position: negative means back from end */
    return if pos >= 0 as libc::c_int as libc::c_long {
               pos
           } else {
               (len as libc::c_long + pos) + 1 as libc::c_int as libc::c_long
           }; /* number of arguments */
}
unsafe extern "C" fn str_sub(mut L: *mut lua_State) -> libc::c_int {
    let mut l: size_t = 0;
    let mut s: *const libc::c_char =
        luaL_check_lstr(L, 1 as libc::c_int, &mut l);
    let mut start: libc::c_long =
        posrelat(luaL_check_number(L, 2 as libc::c_int), l);
    let mut end: libc::c_long =
        posrelat(luaL_opt_number(L, 3 as libc::c_int,
                                 -(1 as libc::c_int) as libc::c_long), l);
    if start < 1 as libc::c_int as libc::c_long {
        start = 1 as libc::c_int as libc::c_long
    }
    if end > l as libc::c_long { end = l as libc::c_long }
    if start <= end {
        lua_pushlstring(L,
                        s.offset(start as
                                     isize).offset(-(1 as libc::c_int as
                                                         isize)),
                        (end - start + 1 as libc::c_int as libc::c_long) as
                            size_t);
    } else { lua_pushstring(L, b"\x00" as *const u8 as *const libc::c_char); }
    return 1 as libc::c_int;
}
unsafe extern "C" fn str_lower(mut L: *mut lua_State) -> libc::c_int {
    let mut l: size_t = 0;
    let mut i: size_t = 0;
    let mut b: luaL_Buffer =
        luaL_Buffer{p: 0 as *mut libc::c_char,
                    level: 0,
                    L: 0 as *mut lua_State,
                    buffer: [0; 8192],};
    let mut s: *const libc::c_char =
        luaL_check_lstr(L, 1 as libc::c_int, &mut l);
    luaL_buffinit(L, &mut b);
    i = 0 as libc::c_int as size_t;
    while i < l {
        (b.p <
             &mut *b.buffer.as_mut_ptr().offset(8192 as libc::c_int as isize)
                 as *mut libc::c_char || !luaL_prepbuffer(&mut b).is_null())
            as libc::c_int;
        let fresh0 = b.p;
        b.p = b.p.offset(1);
        *fresh0 =
            tolower(*s.offset(i as isize) as libc::c_uchar as libc::c_int) as
                libc::c_char;
        i = i.wrapping_add(1)
    }
    luaL_pushresult(&mut b);
    return 1 as libc::c_int;
}
unsafe extern "C" fn str_upper(mut L: *mut lua_State) -> libc::c_int {
    let mut l: size_t = 0;
    let mut i: size_t = 0;
    let mut b: luaL_Buffer =
        luaL_Buffer{p: 0 as *mut libc::c_char,
                    level: 0,
                    L: 0 as *mut lua_State,
                    buffer: [0; 8192],};
    let mut s: *const libc::c_char =
        luaL_check_lstr(L, 1 as libc::c_int, &mut l);
    luaL_buffinit(L, &mut b);
    i = 0 as libc::c_int as size_t;
    while i < l {
        (b.p <
             &mut *b.buffer.as_mut_ptr().offset(8192 as libc::c_int as isize)
                 as *mut libc::c_char || !luaL_prepbuffer(&mut b).is_null())
            as libc::c_int;
        let fresh1 = b.p;
        b.p = b.p.offset(1);
        *fresh1 =
            toupper(*s.offset(i as isize) as libc::c_uchar as libc::c_int) as
                libc::c_char;
        i = i.wrapping_add(1)
    }
    luaL_pushresult(&mut b);
    return 1 as libc::c_int;
}
unsafe extern "C" fn str_rep(mut L: *mut lua_State) -> libc::c_int {
    let mut l: size_t = 0;
    let mut b: luaL_Buffer =
        luaL_Buffer{p: 0 as *mut libc::c_char,
                    level: 0,
                    L: 0 as *mut lua_State,
                    buffer: [0; 8192],};
    let mut s: *const libc::c_char =
        luaL_check_lstr(L, 1 as libc::c_int, &mut l);
    let mut n: libc::c_int =
        luaL_check_number(L, 2 as libc::c_int) as libc::c_int;
    luaL_buffinit(L, &mut b);
    loop  {
        let fresh2 = n;
        n = n - 1;
        if !(fresh2 > 0 as libc::c_int) { break ; }
        luaL_addlstring(&mut b, s, l);
    }
    luaL_pushresult(&mut b);
    return 1 as libc::c_int;
}
unsafe extern "C" fn str_byte(mut L: *mut lua_State) -> libc::c_int {
    let mut l: size_t = 0;
    let mut s: *const libc::c_char =
        luaL_check_lstr(L, 1 as libc::c_int, &mut l);
    let mut pos: libc::c_long =
        posrelat(luaL_opt_number(L, 2 as libc::c_int,
                                 1 as libc::c_int as libc::c_long), l);
    if !((0 as libc::c_int as libc::c_long) < pos && pos as size_t <= l) {
        luaL_argerror(L, 2 as libc::c_int,
                      b"out of range\x00" as *const u8 as
                          *const libc::c_char);
    }
    lua_pushnumber(L,
                   *s.offset((pos - 1 as libc::c_int as libc::c_long) as
                                 isize) as libc::c_uchar as libc::c_long);
    return 1 as libc::c_int;
}
unsafe extern "C" fn str_char(mut L: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = lua_gettop(L);
    let mut i: libc::c_int = 0;
    let mut b: luaL_Buffer =
        luaL_Buffer{p: 0 as *mut libc::c_char,
                    level: 0,
                    L: 0 as *mut lua_State,
                    buffer: [0; 8192],};
    luaL_buffinit(L, &mut b);
    i = 1 as libc::c_int;
    while i <= n {
        let mut c: libc::c_int = luaL_check_number(L, i) as libc::c_int;
        if !(c as libc::c_uchar as libc::c_int == c) {
            luaL_argerror(L, i,
                          b"invalid value\x00" as *const u8 as
                              *const libc::c_char);
        }
        (b.p <
             &mut *b.buffer.as_mut_ptr().offset(8192 as libc::c_int as isize)
                 as *mut libc::c_char || !luaL_prepbuffer(&mut b).is_null())
            as libc::c_int;
        let fresh3 = b.p;
        b.p = b.p.offset(1);
        *fresh3 = c as libc::c_uchar as libc::c_char;
        i += 1
    }
    luaL_pushresult(&mut b);
    return 1 as libc::c_int;
}
unsafe extern "C" fn check_capture(mut L: *mut lua_State, mut l: libc::c_int,
                                   mut cap: *mut Capture) -> libc::c_int {
    l -= '1' as i32;
    if !(0 as libc::c_int <= l && l < (*cap).level &&
             (*cap).capture[l as usize].len !=
                 -(1 as libc::c_int) as libc::c_long) {
        lua_error(L,
                  b"invalid capture index\x00" as *const u8 as
                      *const libc::c_char);
    }
    return l;
}
unsafe extern "C" fn capture_to_close(mut L: *mut lua_State,
                                      mut cap: *mut Capture) -> libc::c_int {
    let mut level: libc::c_int = (*cap).level;
    level -= 1;
    while level >= 0 as libc::c_int {
        if (*cap).capture[level as usize].len ==
               -(1 as libc::c_int) as libc::c_long {
            return level
        }
        level -= 1
    }
    lua_error(L,
              b"invalid pattern capture\x00" as *const u8 as
                  *const libc::c_char);
    return 0 as libc::c_int;
    /* to avoid warnings */
}
#[no_mangle]
pub unsafe extern "C" fn luaI_classend(mut L: *mut lua_State,
                                       mut p: *const libc::c_char)
 -> *const libc::c_char {
    let fresh4 = p;
    p = p.offset(1);
    match *fresh4 as libc::c_int {
        37 => {
            if *p as libc::c_int == '\u{0}' as i32 {
                lua_error(L,
                          b"malformed pattern (ends with `%\')\x00" as
                              *const u8 as *const libc::c_char);
            }
            return p.offset(1 as libc::c_int as isize)
        }
        91 => {
            if *p as libc::c_int == '^' as i32 { p = p.offset(1) }
            loop  {
                /* look for a ']' */
                if *p as libc::c_int == '\u{0}' as i32 {
                    lua_error(L,
                              b"malformed pattern (missing `]\')\x00" as
                                  *const u8 as *const libc::c_char);
                }
                let fresh5 = p;
                p = p.offset(1);
                if *fresh5 as libc::c_int == '%' as i32 &&
                       *p as libc::c_int != '\u{0}' as i32 {
                    p = p.offset(1)
                }
                if !(*p as libc::c_int != ']' as i32) { break ; }
                /* skip escapes (e.g. '%]') */
            }
            return p.offset(1 as libc::c_int as isize)
        }
        _ => { return p }
    };
}
unsafe extern "C" fn match_class(mut c: libc::c_int, mut cl: libc::c_int)
 -> libc::c_int {
    let mut res: libc::c_int = 0;
    match tolower(cl) {
        97 => {
            res =
                *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                    _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
        }
        99 => {
            res =
                *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                    _IScntrl as libc::c_int as libc::c_ushort as libc::c_int
        }
        100 => {
            res =
                *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                    _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        }
        108 => {
            res =
                *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                    _ISlower as libc::c_int as libc::c_ushort as libc::c_int
        }
        112 => {
            res =
                *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                    _ISpunct as libc::c_int as libc::c_ushort as libc::c_int
        }
        115 => {
            res =
                *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                    _ISspace as libc::c_int as libc::c_ushort as libc::c_int
        }
        117 => {
            res =
                *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                    _ISupper as libc::c_int as libc::c_ushort as libc::c_int
        }
        119 => {
            res =
                *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                    _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
        }
        120 => {
            res =
                *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                    _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
        }
        122 => { res = (c == '\u{0}' as i32) as libc::c_int }
        _ => { return (cl == c) as libc::c_int }
    }
    return if *(*__ctype_b_loc()).offset(cl as isize) as libc::c_int &
                  _ISlower as libc::c_int as libc::c_ushort as libc::c_int !=
                  0 {
               res
           } else { (res == 0) as libc::c_int };
}
unsafe extern "C" fn matchbracketclass(mut c: libc::c_int,
                                       mut p: *const libc::c_char,
                                       mut endclass: *const libc::c_char)
 -> libc::c_int {
    let mut sig: libc::c_int = 1 as libc::c_int;
    if *p.offset(1 as libc::c_int as isize) as libc::c_int == '^' as i32 {
        sig = 0 as libc::c_int;
        p = p.offset(1)
        /* skip the '^' */
    }
    loop  {
        p = p.offset(1);
        if !(p < endclass) { break ; }
        if *p as libc::c_int == '%' as i32 {
            p = p.offset(1);
            if match_class(c, *p as libc::c_uchar as libc::c_int) != 0 {
                return sig
            }
        } else if *p.offset(1 as libc::c_int as isize) as libc::c_int ==
                      '-' as i32 &&
                      p.offset(2 as libc::c_int as isize) < endclass {
            p = p.offset(2 as libc::c_int as isize);
            if *p.offset(-(2 as libc::c_int as isize)) as libc::c_uchar as
                   libc::c_int <= c && c <= *p as libc::c_uchar as libc::c_int
               {
                return sig
            }
        } else if *p as libc::c_uchar as libc::c_int == c { return sig }
    }
    return (sig == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaI_singlematch(mut c: libc::c_int,
                                          mut p: *const libc::c_char,
                                          mut ep: *const libc::c_char)
 -> libc::c_int {
    match *p as libc::c_int {
        46 => {
            /* matches any char */
            return 1 as libc::c_int
        }
        37 => {
            return match_class(c,
                               *p.offset(1 as libc::c_int as isize) as
                                   libc::c_uchar as libc::c_int)
        }
        91 => {
            return matchbracketclass(c, p,
                                     ep.offset(-(1 as libc::c_int as isize)))
        }
        _ => {
            return (*p as libc::c_uchar as libc::c_int == c) as libc::c_int
        }
    };
}
unsafe extern "C" fn matchbalance(mut L: *mut lua_State,
                                  mut s: *const libc::c_char,
                                  mut p: *const libc::c_char,
                                  mut cap: *mut Capture)
 -> *const libc::c_char {
    if *p as libc::c_int == 0 as libc::c_int ||
           *p.offset(1 as libc::c_int as isize) as libc::c_int ==
               0 as libc::c_int {
        lua_error(L,
                  b"unbalanced pattern\x00" as *const u8 as
                      *const libc::c_char);
    }
    if *s as libc::c_int != *p as libc::c_int {
        return 0 as *const libc::c_char
    } else {
        let mut b: libc::c_int = *p as libc::c_int;
        let mut e: libc::c_int =
            *p.offset(1 as libc::c_int as isize) as libc::c_int;
        let mut cont: libc::c_int = 1 as libc::c_int;
        loop  {
            s = s.offset(1);
            if !(s < (*cap).src_end) { break ; }
            if *s as libc::c_int == e {
                cont -= 1;
                if cont == 0 as libc::c_int {
                    return s.offset(1 as libc::c_int as isize)
                }
            } else if *s as libc::c_int == b { cont += 1 }
        }
    }
    return 0 as *const libc::c_char;
    /* string ends out of balance */
}
unsafe extern "C" fn max_expand(mut L: *mut lua_State,
                                mut s: *const libc::c_char,
                                mut p: *const libc::c_char,
                                mut ep: *const libc::c_char,
                                mut cap: *mut Capture)
 -> *const libc::c_char {
    let mut i: libc::c_long =
        0 as libc::c_int as libc::c_long; /* counts maximum expand for item */
    while s.offset(i as isize) < (*cap).src_end &&
              luaI_singlematch(*s.offset(i as isize) as libc::c_uchar as
                                   libc::c_int, p, ep) != 0 {
        i += 1
    }
    /* keeps trying to match with the maximum repetitions */
    while i >= 0 as libc::c_int as libc::c_long {
        let mut res: *const libc::c_char =
            match_0(L, s.offset(i as isize),
                    ep.offset(1 as libc::c_int as isize), cap);
        if !res.is_null() { return res }
        i -= 1
        /* else didn't match; reduce 1 repetition to try again */
    } /* try with one more repetition */
    return 0 as *const libc::c_char; /* undo capture */
}
unsafe extern "C" fn min_expand(mut L: *mut lua_State,
                                mut s: *const libc::c_char,
                                mut p: *const libc::c_char,
                                mut ep: *const libc::c_char,
                                mut cap: *mut Capture)
 -> *const libc::c_char {
    loop  {
        let mut res: *const libc::c_char =
            match_0(L, s, ep.offset(1 as libc::c_int as isize), cap);
        if !res.is_null() {
            return res
        } else {
            if s < (*cap).src_end &&
                   luaI_singlematch(*s as libc::c_uchar as libc::c_int, p, ep)
                       != 0 {
                s = s.offset(1)
            } else { return 0 as *const libc::c_char }
        }
    };
}
unsafe extern "C" fn start_capture(mut L: *mut lua_State,
                                   mut s: *const libc::c_char,
                                   mut p: *const libc::c_char,
                                   mut cap: *mut Capture)
 -> *const libc::c_char {
    let mut res: *const libc::c_char = 0 as *const libc::c_char;
    let mut level: libc::c_int = (*cap).level;
    if level >= 32 as libc::c_int {
        lua_error(L,
                  b"too many captures\x00" as *const u8 as
                      *const libc::c_char);
    }
    (*cap).capture[level as usize].init = s;
    (*cap).capture[level as usize].len = -(1 as libc::c_int) as libc::c_long;
    (*cap).level = level + 1 as libc::c_int;
    res = match_0(L, s, p.offset(1 as libc::c_int as isize), cap);
    if res.is_null() {
        /* match failed? */
        (*cap).level -= 1
    } /* close capture */
    return res; /* undo capture */
}
unsafe extern "C" fn end_capture(mut L: *mut lua_State,
                                 mut s: *const libc::c_char,
                                 mut p: *const libc::c_char,
                                 mut cap: *mut Capture)
 -> *const libc::c_char {
    let mut l: libc::c_int = capture_to_close(L, cap);
    let mut res: *const libc::c_char = 0 as *const libc::c_char;
    (*cap).capture[l as usize].len =
        s.wrapping_offset_from((*cap).capture[l as usize].init) as
            libc::c_long;
    res = match_0(L, s, p.offset(1 as libc::c_int as isize), cap);
    if res.is_null() {
        /* match failed? */
        (*cap).capture[l as usize].len = -(1 as libc::c_int) as libc::c_long
    }
    return res;
}
unsafe extern "C" fn match_capture(mut L: *mut lua_State,
                                   mut s: *const libc::c_char,
                                   mut level: libc::c_int,
                                   mut cap: *mut Capture)
 -> *const libc::c_char {
    let mut l: libc::c_int = check_capture(L, level, cap);
    let mut len: size_t = (*cap).capture[l as usize].len as size_t;
    if (*cap).src_end.wrapping_offset_from(s) as libc::c_long as size_t >= len
           &&
           memcmp((*cap).capture[l as usize].init as *const libc::c_void,
                  s as *const libc::c_void, len) == 0 as libc::c_int {
        return s.offset(len as isize)
    } else { return 0 as *const libc::c_char };
}
unsafe extern "C" fn match_0(mut L: *mut lua_State,
                             mut s: *const libc::c_char,
                             mut p: *const libc::c_char,
                             mut cap: *mut Capture) -> *const libc::c_char {
    loop 
         /* using goto's to optimize tail recursion */
         {
        match *p as libc::c_int {
            40 => {
                /* start capture */
                return start_capture(L, s, p, cap)
            }
            41 => {
                /* end capture */
                return end_capture(L, s, p, cap)
            }
            37 => {
                /* may be %[0-9] or %b */
                if *(*__ctype_b_loc()).offset(*p.offset(1 as libc::c_int as
                                                            isize) as
                                                  libc::c_uchar as libc::c_int
                                                  as isize) as libc::c_int &
                       _ISdigit as libc::c_int as libc::c_ushort as
                           libc::c_int != 0 {
                    /* capture? */
                    s =
                        match_capture(L, s,
                                      *p.offset(1 as libc::c_int as isize) as
                                          libc::c_int, cap);
                    if s.is_null() { return 0 as *const libc::c_char }
                    p = p.offset(2 as libc::c_int as isize);
                    continue ;
                    /* else return match(L, s, p+2, cap) */
                } else if *p.offset(1 as libc::c_int as isize) as libc::c_int
                              == 'b' as i32 {
                    /* balanced string? */
                    s =
                        matchbalance(L, s,
                                     p.offset(2 as libc::c_int as isize),
                                     cap);
                    if s.is_null() { return 0 as *const libc::c_char }
                    p = p.offset(4 as libc::c_int as isize);
                    continue ;
                    /* else return match(L, s, p+4, cap); */
                }
            }
            0 => { return s }
            36 => {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int ==
                       '\u{0}' as i32 { /* match succeeded */
                    /* end of pattern */
                    /* is the '$' the last char in pattern? */
                    return if s == (*cap).src_end {
                               s
                           } else { 0 as *const libc::c_char }
                }
            }
            _ => { }
        } /* check end of string */
        /* it is a pattern item */
        let mut ep: *const libc::c_char =
            luaI_classend(L, p); /* points to what is next */
        let mut m: libc::c_int =
            (s < (*cap).src_end &&
                 luaI_singlematch(*s as libc::c_uchar as libc::c_int, p, ep)
                     != 0) as libc::c_int;
        match *ep as libc::c_int {
            63 => {
                /* optional */
                let mut res: *const libc::c_char = 0 as *const libc::c_char;
                if m != 0 &&
                       {
                           res =
                               match_0(L, s.offset(1 as libc::c_int as isize),
                                       ep.offset(1 as libc::c_int as isize),
                                       cap);
                           !res.is_null()
                       } {
                    return res
                }
                p = ep.offset(1 as libc::c_int as isize)
                /* else return match(L, s, ep+1, cap); */
            }
            42 => {
                /* 0 or more repetitions */
                return max_expand(L, s, p, ep, cap)
            }
            43 => {
                /* 1 or more repetitions */
                return if m != 0 {
                           max_expand(L, s.offset(1 as libc::c_int as isize),
                                      p, ep, cap)
                       } else { 0 as *const libc::c_char }
            }
            45 => {
                /* 0 or more repetitions (minimum) */
                return min_expand(L, s, p, ep, cap)
            }
            _ => {
                if m == 0 {
                    return 0 as *const libc::c_char
                } /* empty strings are everywhere */
                s = s.offset(1); /* avoids a negative `l1' */
                p = ep
            }
        }
    }; /* to search for a `*s2' inside `s1' */
}
unsafe extern "C" fn lmemfind(mut s1: *const libc::c_char, mut l1: size_t,
                              mut s2: *const libc::c_char, mut l2: size_t)
 -> *const libc::c_char {
    if l2 == 0 as libc::c_int as libc::c_ulong {
        return s1
    } else if l2 > l1 {
        return 0 as *const libc::c_char
    } else {
        let mut init: *const libc::c_char =
            0 as
                *const libc::c_char; /* 1st char will be checked by `memchr' */
        l2 = l2.wrapping_sub(1); /* `s2' cannot be found after that */
        l1 = l1.wrapping_sub(l2); /* 1st char is already checked */
        while l1 > 0 as libc::c_int as libc::c_ulong &&
                  {
                      init =
                          memchr(s1 as *const libc::c_void,
                                 *s2 as libc::c_int, l1) as
                              *const libc::c_char;
                      !init.is_null()
                  } {
            init = init.offset(1);
            if memcmp(init as *const libc::c_void,
                      s2.offset(1 as libc::c_int as isize) as
                          *const libc::c_void, l2) == 0 as libc::c_int {
                return init.offset(-(1 as libc::c_int as isize))
            } else {
                /* correct `l1' and `s1' to try again */
                l1 =
                    (l1 as
                         libc::c_ulong).wrapping_sub(init.wrapping_offset_from(s1)
                                                         as libc::c_long as
                                                         libc::c_ulong) as
                        size_t as size_t;
                s1 = init
            }
        }
        return 0 as *const libc::c_char
        /* not found */
    };
}
unsafe extern "C" fn push_captures(mut L: *mut lua_State,
                                   mut cap: *mut Capture) -> libc::c_int {
    let mut i: libc::c_int = 0;
    luaL_checkstack(L, (*cap).level,
                    b"too many captures\x00" as *const u8 as
                        *const libc::c_char);
    i = 0 as libc::c_int;
    while i < (*cap).level {
        let mut l: libc::c_int =
            (*cap).capture[i as usize].len as libc::c_int;
        if l == -(1 as libc::c_int) {
            lua_error(L,
                      b"unfinished capture\x00" as *const u8 as
                          *const libc::c_char);
        }
        lua_pushlstring(L, (*cap).capture[i as usize].init, l as size_t);
        i += 1
    }
    return (*cap).level;
    /* number of strings pushed */
}
unsafe extern "C" fn str_find(mut L: *mut lua_State) -> libc::c_int {
    let mut l1: size_t = 0;
    let mut l2: size_t = 0;
    let mut s: *const libc::c_char =
        luaL_check_lstr(L, 1 as libc::c_int, &mut l1);
    let mut p: *const libc::c_char =
        luaL_check_lstr(L, 2 as libc::c_int, &mut l2);
    let mut init: libc::c_long =
        posrelat(luaL_opt_number(L, 3 as libc::c_int,
                                 1 as libc::c_int as libc::c_long), l1) -
            1 as libc::c_int as libc::c_long;
    let mut cap: Capture =
        Capture{src_end: 0 as *const libc::c_char,
                level: 0,
                capture:
                    [C2RustUnnamed_0{init: 0 as *const libc::c_char, len: 0,};
                        32],};
    if !(0 as libc::c_int as libc::c_long <= init && init as size_t <= l1) {
        luaL_argerror(L, 3 as libc::c_int,
                      b"out of range\x00" as *const u8 as
                          *const libc::c_char);
    }
    if lua_gettop(L) > 3 as libc::c_int ||
           strpbrk(p,
                   b"^$*+?.([%-\x00" as *const u8 as
                       *const libc::c_char).is_null() {
        /* or no special characters? */
        let mut s2: *const libc::c_char =
            lmemfind(s.offset(init as isize),
                     l1.wrapping_sub(init as libc::c_ulong), p,
                     l2); /* start */
        if !s2.is_null() {
            lua_pushnumber(L,
                           s2.wrapping_offset_from(s) as libc::c_long +
                               1 as libc::c_int as libc::c_long); /* end */
            lua_pushnumber(L,
                           (s2.wrapping_offset_from(s) as libc::c_long as
                                libc::c_ulong).wrapping_add(l2) as
                               libc::c_long); /* not found */
            return 2 as libc::c_int
        }
    } else {
        let mut anchor: libc::c_int =
            if *p as libc::c_int == '^' as i32 {
                p = p.offset(1); /* skip ESC */
                1 as libc::c_int
            } else { 0 as libc::c_int };
        let mut s1: *const libc::c_char = s.offset(init as isize);
        cap.src_end = s.offset(l1 as isize);
        loop  {
            let mut res: *const libc::c_char = 0 as *const libc::c_char;
            cap.level = 0 as libc::c_int;
            res = match_0(L, s1, p, &mut cap);
            if !res.is_null() {
                lua_pushnumber(L,
                               s1.wrapping_offset_from(s) as libc::c_long +
                                   1 as libc::c_int as libc::c_long);
                lua_pushnumber(L,
                               res.wrapping_offset_from(s) as libc::c_long);
                return push_captures(L, &mut cap) + 2 as libc::c_int
            }
            let fresh6 = s1;
            s1 = s1.offset(1);
            if !(fresh6 < cap.src_end && anchor == 0) { break ; }
        }
    }
    lua_pushnil(L);
    return 1 as libc::c_int;
}
unsafe extern "C" fn add_s(mut L: *mut lua_State, mut b: *mut luaL_Buffer,
                           mut cap: *mut Capture) {
    if lua_isstring(L, 3 as libc::c_int) != 0 {
        let mut news: *const libc::c_char = lua_tostring(L, 3 as libc::c_int);
        let mut l: size_t = lua_strlen(L, 3 as libc::c_int);
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < l {
            if *news.offset(i as isize) as libc::c_int != '%' as i32 {
                ((*b).p <
                     &mut *(*b).buffer.as_mut_ptr().offset(8192 as libc::c_int
                                                               as isize) as
                         *mut libc::c_char || !luaL_prepbuffer(b).is_null())
                    as libc::c_int;
                let fresh7 = (*b).p;
                (*b).p = (*b).p.offset(1);
                *fresh7 = *news.offset(i as isize)
            } else {
                i = i.wrapping_add(1);
                if *(*__ctype_b_loc()).offset(*news.offset(i as isize) as
                                                  libc::c_uchar as libc::c_int
                                                  as isize) as libc::c_int &
                       _ISdigit as libc::c_int as libc::c_ushort as
                           libc::c_int == 0 {
                    ((*b).p <
                         &mut *(*b).buffer.as_mut_ptr().offset(8192 as
                                                                   libc::c_int
                                                                   as isize)
                             as *mut libc::c_char ||
                         !luaL_prepbuffer(b).is_null()) as libc::c_int;
                    let fresh8 = (*b).p;
                    (*b).p = (*b).p.offset(1);
                    *fresh8 = *news.offset(i as isize)
                } else {
                    let mut level: libc::c_int =
                        check_capture(L,
                                      *news.offset(i as isize) as libc::c_int,
                                      cap);
                    luaL_addlstring(b, (*cap).capture[level as usize].init,
                                    (*cap).capture[level as usize].len as
                                        size_t);
                }
            }
            i = i.wrapping_add(1)
        }
    } else {
        /* is a function */
        let mut n: libc::c_int = 0; /* add return to accumulated result */
        lua_pushvalue(L, 3 as libc::c_int);
        n = push_captures(L, cap);
        lua_rawcall(L, n, 1 as libc::c_int);
        if lua_isstring(L, -(1 as libc::c_int)) != 0 {
            luaL_addvalue(b);
        } else { lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int); }
    };
}
unsafe extern "C" fn str_gsub(mut L: *mut lua_State) -> libc::c_int {
    let mut srcl: size_t = 0;
    let mut src: *const libc::c_char =
        luaL_check_lstr(L, 1 as libc::c_int, &mut srcl);
    let mut p: *const libc::c_char =
        luaL_check_lstr(L, 2 as libc::c_int, 0 as *mut size_t);
    let mut max_s: libc::c_int =
        luaL_opt_number(L, 4 as libc::c_int,
                        srcl.wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as libc::c_long) as libc::c_int;
    let mut anchor: libc::c_int =
        if *p as libc::c_int == '^' as i32 {
            p = p.offset(1);
            1 as libc::c_int
        } else { 0 as libc::c_int };
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut cap: Capture =
        Capture{src_end: 0 as *const libc::c_char,
                level: 0,
                capture:
                    [C2RustUnnamed_0{init: 0 as *const libc::c_char, len: 0,};
                        32],};
    let mut b: luaL_Buffer =
        luaL_Buffer{p: 0 as *mut libc::c_char,
                    level: 0,
                    L: 0 as *mut lua_State,
                    buffer: [0; 8192],};
    if !(lua_gettop(L) >= 3 as libc::c_int &&
             (lua_isstring(L, 3 as libc::c_int) != 0 ||
                  lua_type(L, 3 as libc::c_int) == 5 as libc::c_int)) {
        luaL_argerror(L, 3 as libc::c_int,
                      b"string or function expected\x00" as *const u8 as
                          *const libc::c_char);
    }
    luaL_buffinit(L, &mut b);
    cap.src_end = src.offset(srcl as isize);
    while n < max_s {
        let mut e: *const libc::c_char = 0 as *const libc::c_char;
        cap.level = 0 as libc::c_int;
        e = match_0(L, src, p, &mut cap);
        if !e.is_null() { n += 1; add_s(L, &mut b, &mut cap); }
        if !e.is_null() && e > src {
            /* non empty match? */
            src = e
        } else {
            if !(src < cap.src_end) {
                break ; /* skip it */
            } /* number of substitutions */
            (b.p <
                 &mut *b.buffer.as_mut_ptr().offset(8192 as libc::c_int as
                                                        isize) as
                     *mut libc::c_char || !luaL_prepbuffer(&mut b).is_null())
                as libc::c_int;
            let fresh9 = src;
            src = src.offset(1);
            let fresh10 = b.p;
            b.p = b.p.offset(1);
            *fresh10 = *fresh9
        }
        if anchor != 0 { break ; }
    }
    luaL_addlstring(&mut b, src,
                    cap.src_end.wrapping_offset_from(src) as libc::c_long as
                        size_t);
    luaL_pushresult(&mut b);
    lua_pushnumber(L, n as libc::c_long);
    return 2 as libc::c_int;
}
/* }====================================================== */
unsafe extern "C" fn luaI_addquoted(mut L: *mut lua_State,
                                    mut b: *mut luaL_Buffer,
                                    mut arg: libc::c_int) {
    let mut l: size_t = 0; /* skip the "addsize" at the end */
    let mut s: *const libc::c_char = luaL_check_lstr(L, arg, &mut l); /* %% */
    ((*b).p <
         &mut *(*b).buffer.as_mut_ptr().offset(8192 as libc::c_int as isize)
             as *mut libc::c_char || !luaL_prepbuffer(b).is_null()) as
        libc::c_int;
    let fresh11 = (*b).p;
    (*b).p = (*b).p.offset(1);
    *fresh11 = '\"' as i32 as libc::c_char;
    loop  {
        let fresh12 = l;
        l = l.wrapping_sub(1);
        if !(fresh12 != 0) { break ; }
        match *s as libc::c_int {
            34 | 92 | 10 => {
                ((*b).p <
                     &mut *(*b).buffer.as_mut_ptr().offset(8192 as libc::c_int
                                                               as isize) as
                         *mut libc::c_char || !luaL_prepbuffer(b).is_null())
                    as libc::c_int;
                let fresh13 = (*b).p;
                (*b).p = (*b).p.offset(1);
                *fresh13 = '\\' as i32 as libc::c_char;
                ((*b).p <
                     &mut *(*b).buffer.as_mut_ptr().offset(8192 as libc::c_int
                                                               as isize) as
                         *mut libc::c_char || !luaL_prepbuffer(b).is_null())
                    as libc::c_int;
                let fresh14 = (*b).p;
                (*b).p = (*b).p.offset(1);
                *fresh14 = *s
            }
            0 => {
                luaL_addlstring(b,
                                b"\\000\x00" as *const u8 as
                                    *const libc::c_char,
                                4 as libc::c_int as size_t);
            }
            _ => {
                ((*b).p <
                     &mut *(*b).buffer.as_mut_ptr().offset(8192 as libc::c_int
                                                               as isize) as
                         *mut libc::c_char || !luaL_prepbuffer(b).is_null())
                    as libc::c_int;
                let fresh15 = (*b).p;
                (*b).p = (*b).p.offset(1);
                *fresh15 = *s
            }
        }
        s = s.offset(1)
    }
    ((*b).p <
         &mut *(*b).buffer.as_mut_ptr().offset(8192 as libc::c_int as isize)
             as *mut libc::c_char || !luaL_prepbuffer(b).is_null()) as
        libc::c_int;
    let fresh16 = (*b).p;
    (*b).p = (*b).p.offset(1);
    *fresh16 = '\"' as i32 as libc::c_char;
}
unsafe extern "C" fn str_format(mut L: *mut lua_State) -> libc::c_int {
    let mut arg: libc::c_int = 1 as libc::c_int;
    let mut strfrmt: *const libc::c_char =
        luaL_check_lstr(L, arg, 0 as *mut size_t);
    let mut b: luaL_Buffer =
        luaL_Buffer{p: 0 as *mut libc::c_char,
                    level: 0,
                    L: 0 as *mut lua_State,
                    buffer: [0; 8192],};
    luaL_buffinit(L, &mut b);
    while *strfrmt != 0 {
        if *strfrmt as libc::c_int != '%' as i32 {
            (b.p <
                 &mut *b.buffer.as_mut_ptr().offset(8192 as libc::c_int as
                                                        isize) as
                     *mut libc::c_char || !luaL_prepbuffer(&mut b).is_null())
                as libc::c_int;
            let fresh17 = strfrmt;
            strfrmt = strfrmt.offset(1);
            let fresh18 = b.p;
            b.p = b.p.offset(1);
            *fresh18 = *fresh17
        } else {
            strfrmt = strfrmt.offset(1);
            if *strfrmt as libc::c_int == '%' as i32 {
                (b.p <
                     &mut *b.buffer.as_mut_ptr().offset(8192 as libc::c_int as
                                                            isize) as
                         *mut libc::c_char ||
                     !luaL_prepbuffer(&mut b).is_null()) as libc::c_int;
                let fresh19 = strfrmt;
                strfrmt = strfrmt.offset(1);
                let fresh20 = b.p;
                b.p = b.p.offset(1);
                *fresh20 = *fresh19
            } else {
                /* format item */
                let mut cap: Capture =
                    Capture{src_end: 0 as *const libc::c_char,
                            level: 0,
                            capture:
                                [C2RustUnnamed_0{init:
                                                     0 as *const libc::c_char,
                                                 len: 0,};
                                    32],}; /* to store the format ('%...') */
                let mut form: [libc::c_char; 20] =
                    [0; 20]; /* to store the formatted item */
                let mut buff: [libc::c_char; 512] = [0; 512];
                let mut initf: *const libc::c_char = strfrmt;
                form[0 as libc::c_int as usize] = '%' as i32 as libc::c_char;
                if *(*__ctype_b_loc()).offset(*initf as libc::c_uchar as
                                                  libc::c_int as isize) as
                       libc::c_int &
                       _ISdigit as libc::c_int as libc::c_ushort as
                           libc::c_int != 0 &&
                       *initf.offset(1 as libc::c_int as isize) as libc::c_int
                           == '$' as i32 {
                    arg = *initf as libc::c_int - '0' as i32;
                    initf = initf.offset(2 as libc::c_int as isize)
                    /* skip the 'n$' */
                } /* +1 to include conversion */
                arg += 1;
                cap.src_end =
                    strfrmt.offset(strlen(strfrmt) as
                                       isize).offset(1 as libc::c_int as
                                                         isize);
                cap.level = 0 as libc::c_int;
                strfrmt =
                    match_0(L, initf,
                            b"[-+ #0]*(%d*)%.?(%d*)\x00" as *const u8 as
                                *const libc::c_char, &mut cap);
                if cap.capture[0 as libc::c_int as usize].len >
                       2 as libc::c_int as libc::c_long ||
                       cap.capture[1 as libc::c_int as usize].len >
                           2 as libc::c_int as libc::c_long ||
                       strfrmt.wrapping_offset_from(initf) as libc::c_long >
                           (20 as libc::c_int - 2 as libc::c_int) as
                               libc::c_long {
                    lua_error(L,
                              b"invalid format (width or precision too long)\x00"
                                  as *const u8 as *const libc::c_char);
                }
                strncpy(form.as_mut_ptr().offset(1 as libc::c_int as isize),
                        initf,
                        (strfrmt.wrapping_offset_from(initf) as libc::c_long +
                             1 as libc::c_int as libc::c_long) as
                            libc::c_ulong);
                form[(strfrmt.wrapping_offset_from(initf) as libc::c_long +
                          2 as libc::c_int as libc::c_long) as usize] =
                    0 as libc::c_int as libc::c_char;
                let fresh21 = strfrmt;
                strfrmt = strfrmt.offset(1);
                match *fresh21 as libc::c_int {
                    99 | 100 | 105 => {
                        sprintf(buff.as_mut_ptr(), form.as_mut_ptr(),
                                luaL_check_number(L, arg) as libc::c_int);
                    }
                    111 | 117 | 120 | 88 => {
                        sprintf(buff.as_mut_ptr(), form.as_mut_ptr(),
                                luaL_check_number(L, arg) as libc::c_uint);
                    }
                    101 | 69 | 102 | 103 | 71 => {
                        sprintf(buff.as_mut_ptr(), form.as_mut_ptr(),
                                luaL_check_number(L, arg));
                    }
                    113 => { luaI_addquoted(L, &mut b, arg); continue ; }
                    115 => {
                        let mut l: size_t = 0;
                        let mut s: *const libc::c_char =
                            luaL_check_lstr(L, arg, &mut l);
                        if cap.capture[1 as libc::c_int as usize].len ==
                               0 as libc::c_int as libc::c_long &&
                               l >= 100 as libc::c_int as libc::c_ulong {
                            /* no precision and string is too long to be formatted;
               keep original string */
                            lua_pushvalue(L, arg);
                            luaL_addvalue(&mut b);
                            continue ;
                            /* skip the "addsize" at the end */
                        } else {
                            sprintf(buff.as_mut_ptr(), form.as_mut_ptr(), s);
                        }
                    }
                    _ => {
                        /* also treat cases 'pnLlh' */
                        lua_error(L,
                                  b"invalid option in `format\'\x00" as
                                      *const u8 as *const libc::c_char);
                    }
                }
                luaL_addlstring(&mut b, buff.as_mut_ptr(),
                                strlen(buff.as_mut_ptr()));
            }
        }
    }
    luaL_pushresult(&mut b);
    return 1 as libc::c_int;
}
static mut strlib: [luaL_reg; 11] =
    unsafe {
        [{
             let mut init =
                 luaL_reg{name:
                              b"strlen\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(str_len as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"strsub\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(str_sub as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"strlower\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(str_lower as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"strupper\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(str_upper as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"strchar\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(str_char as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"strrep\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(str_rep as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"ascii\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(str_byte as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"strbyte\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(str_byte as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"format\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(str_format as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"strfind\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(str_find as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"gsub\x00" as *const u8 as *const libc::c_char,
                          func:
                              Some(str_gsub as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         }]
    };
/*
** Open string library
*/
#[no_mangle]
pub unsafe extern "C" fn lua_strlibopen(mut L: *mut lua_State) {
    luaL_openlib(L, strlib.as_ptr(),
                 (::std::mem::size_of::<[luaL_reg; 11]>() as
                      libc::c_ulong).wrapping_div(::std::mem::size_of::<luaL_reg>()
                                                      as libc::c_ulong) as
                     libc::c_int);
}
