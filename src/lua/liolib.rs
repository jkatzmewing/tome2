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
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
     -> *mut libc::c_char;
    #[no_mangle]
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fread(_: *mut libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
             _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    fn fwrite(_: *const libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
              _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn system(__command: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn clock() -> clock_t;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn strftime(__s: *mut libc::c_char, __maxsize: size_t,
                __format: *const libc::c_char, __tp: *const tm) -> size_t;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_type(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_isnumber(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tag(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tonumber(L: *mut lua_State, index: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn lua_strlen(L: *mut lua_State, index: libc::c_int) -> size_t;
    #[no_mangle]
    fn lua_touserdata(L: *mut lua_State, index: libc::c_int)
     -> *mut libc::c_void;
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
    fn lua_getglobals(L: *mut lua_State);
    #[no_mangle]
    fn lua_getref(L: *mut lua_State, ref_0: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_setglobal(L: *mut lua_State, name: *const libc::c_char);
    #[no_mangle]
    fn lua_settagmethod(L: *mut lua_State, tag: libc::c_int,
                        event: *const libc::c_char);
    #[no_mangle]
    fn lua_ref(L: *mut lua_State, lock: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_rawcall(L: *mut lua_State, nargs: libc::c_int,
                   nresults: libc::c_int);
    #[no_mangle]
    fn lua_dostring(L: *mut lua_State, str: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn lua_newtag(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_settag(L: *mut lua_State, tag: libc::c_int);
    #[no_mangle]
    fn lua_error(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    fn lua_newuserdata(L: *mut lua_State, size: size_t) -> *mut libc::c_void;
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
    fn luaL_opt_number(L: *mut lua_State, numArg: libc::c_int,
                       def: libc::c_long) -> libc::c_long;
    #[no_mangle]
    fn luaL_checkstack(L: *mut lua_State, space: libc::c_int,
                       msg: *const libc::c_char);
    #[no_mangle]
    fn luaL_verror(L: *mut lua_State, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn luaL_findstring(name: *const libc::c_char,
                       list: *const *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);
    #[no_mangle]
    fn luaL_prepbuffer(B: *mut luaL_Buffer) -> *mut libc::c_char;
    #[no_mangle]
    fn luaL_addstring(B: *mut luaL_Buffer, s: *const libc::c_char);
    #[no_mangle]
    fn luaL_pushresult(B: *mut luaL_Buffer);
    #[no_mangle]
    fn lua_getstack(L: *mut lua_State, level: libc::c_int, ar: *mut lua_Debug)
     -> libc::c_int;
    #[no_mangle]
    fn lua_getinfo(L: *mut lua_State, what: *const libc::c_char,
                   ar: *mut lua_Debug) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn setlocale(__category: libc::c_int, __locale: *const libc::c_char)
     -> *mut libc::c_char;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
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
pub type clock_t = __clock_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
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
pub struct IOCtrl {
    pub ref_0: [libc::c_int; 2],
    pub iotag: libc::c_int,
    pub closedtag: libc::c_int,
}
static mut filenames: [*const libc::c_char; 2] =
    [b"_INPUT\x00" as *const u8 as *const libc::c_char,
     b"_OUTPUT\x00" as *const u8 as *const libc::c_char];
unsafe extern "C" fn pushresult(mut L: *mut lua_State, mut i: libc::c_int)
 -> libc::c_int {
    if i != 0 {
        lua_pushusertag(L, 0 as *mut libc::c_void, 0 as libc::c_int);
        return 1 as libc::c_int
    } else {
        lua_pushnil(L);
        lua_pushstring(L, strerror(*__errno_location()));
        lua_pushnumber(L, *__errno_location() as libc::c_long);
        return 3 as libc::c_int
    };
}
/*
** {======================================================
** FILE Operations
** =======================================================
*/
unsafe extern "C" fn gethandle(mut L: *mut lua_State, mut ctrl: *mut IOCtrl,
                               mut f: libc::c_int) -> *mut FILE {
    let mut p: *mut libc::c_void = lua_touserdata(L, f);
    if !p.is_null() {
        /* is `f' a userdata ? */
        let mut ftag: libc::c_int = lua_tag(L, f);
        if ftag == (*ctrl).iotag {
            /* does it have the correct tag? */
            return p as *mut FILE
        } else {
            if ftag == (*ctrl).closedtag {
                lua_error(L,
                          b"cannot access a closed file\x00" as *const u8 as
                              *const libc::c_char); /* remove upvalue */
            }
        }
    } /* remove upvalue */
    return 0 as *mut FILE; /* remove upvalue */
}
unsafe extern "C" fn getnonullfile(mut L: *mut lua_State,
                                   mut ctrl: *mut IOCtrl,
                                   mut arg: libc::c_int) -> *mut FILE {
    let mut f: *mut FILE = gethandle(L, ctrl, arg);
    if f.is_null() {
        luaL_argerror(L, arg,
                      b"invalid file handle\x00" as *const u8 as
                          *const libc::c_char);
    }
    return f;
}
unsafe extern "C" fn getfilebyref(mut L: *mut lua_State,
                                  mut ctrl: *mut IOCtrl,
                                  mut inout: libc::c_int) -> *mut FILE {
    let mut f: *mut FILE = 0 as *mut FILE;
    lua_getglobals(L);
    lua_getref(L, (*ctrl).ref_0[inout as usize]);
    lua_rawget(L, -(2 as libc::c_int));
    f = gethandle(L, ctrl, -(1 as libc::c_int));
    if f.is_null() {
        luaL_verror(L,
                    b"global variable `%.10s\' is not a file handle\x00" as
                        *const u8 as *const libc::c_char,
                    filenames[inout as usize]);
    }
    return f;
}
unsafe extern "C" fn setfilebyname(mut L: *mut lua_State,
                                   mut ctrl: *mut IOCtrl, mut f: *mut FILE,
                                   mut name: *const libc::c_char) {
    lua_pushusertag(L, f as *mut libc::c_void, (*ctrl).iotag);
    lua_setglobal(L, name);
}
unsafe extern "C" fn setreturn(mut L: *mut lua_State, mut ctrl: *mut IOCtrl,
                               mut f: *mut FILE, mut inout: libc::c_int)
 -> libc::c_int {
    if f.is_null() {
        return pushresult(L, 0 as libc::c_int)
    } else {
        setfilebyname(L, ctrl, f, filenames[inout as usize]);
        lua_pushusertag(L, f as *mut libc::c_void, (*ctrl).iotag);
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn closefile(mut L: *mut lua_State, mut ctrl: *mut IOCtrl,
                               mut f: *mut FILE) -> libc::c_int {
    if f == stdin || f == stdout || f == stderr {
        return 1 as libc::c_int
    } else {
        lua_pushusertag(L, f as *mut libc::c_void, (*ctrl).iotag);
        lua_settag(L, (*ctrl).closedtag);
        return (fclose(f) == 0 as libc::c_int) as libc::c_int
    };
}
unsafe extern "C" fn io_close(mut L: *mut lua_State) -> libc::c_int {
    let mut ctrl: *mut IOCtrl =
        lua_touserdata(L, -(1 as libc::c_int)) as *mut IOCtrl;
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return pushresult(L,
                      closefile(L, ctrl,
                                getnonullfile(L, ctrl, 1 as libc::c_int)));
}
unsafe extern "C" fn file_collect(mut L: *mut lua_State) -> libc::c_int {
    let mut ctrl: *mut IOCtrl =
        lua_touserdata(L, -(1 as libc::c_int)) as *mut IOCtrl;
    let mut f: *mut FILE = getnonullfile(L, ctrl, 1 as libc::c_int);
    if f != stdin && f != stdout && f != stderr { fclose(f); }
    return 0 as libc::c_int;
}
unsafe extern "C" fn io_open(mut L: *mut lua_State) -> libc::c_int {
    let mut ctrl: *mut IOCtrl =
        lua_touserdata(L, -(1 as libc::c_int)) as *mut IOCtrl;
    let mut f: *mut FILE = 0 as *mut FILE;
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    f =
        fopen(luaL_check_lstr(L, 1 as libc::c_int, 0 as *mut size_t),
              luaL_check_lstr(L, 2 as libc::c_int, 0 as *mut size_t));
    if !f.is_null() {
        lua_pushusertag(L, f as *mut libc::c_void, (*ctrl).iotag);
        return 1 as libc::c_int
    } else { return pushresult(L, 0 as libc::c_int) };
}
unsafe extern "C" fn io_fromto(mut L: *mut lua_State, mut inout: libc::c_int,
                               mut mode: *const libc::c_char) -> libc::c_int {
    let mut ctrl: *mut IOCtrl =
        lua_touserdata(L, -(1 as libc::c_int)) as *mut IOCtrl;
    let mut current: *mut FILE = 0 as *mut FILE;
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    if lua_type(L, 1 as libc::c_int) == -(1 as libc::c_int) {
        closefile(L, ctrl, getfilebyref(L, ctrl, inout));
        current = if inout == 0 as libc::c_int { stdin } else { stdout }
    } else if lua_tag(L, 1 as libc::c_int) == (*ctrl).iotag {
        /* deprecated option */
        current = lua_touserdata(L, 1 as libc::c_int) as *mut FILE
    } else {
        let mut s: *const libc::c_char =
            luaL_check_lstr(L, 1 as libc::c_int,
                            0 as *mut size_t); /* remove upvalue */
        current =
            if *s as libc::c_int == '|' as i32 {
                0 as *mut FILE
            } else { fopen(s, mode) }
    }
    return setreturn(L, ctrl, current, inout);
}
unsafe extern "C" fn io_readfrom(mut L: *mut lua_State) -> libc::c_int {
    return io_fromto(L, 0 as libc::c_int,
                     b"r\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn io_writeto(mut L: *mut lua_State) -> libc::c_int {
    return io_fromto(L, 1 as libc::c_int,
                     b"w\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn io_appendto(mut L: *mut lua_State) -> libc::c_int {
    let mut ctrl: *mut IOCtrl =
        lua_touserdata(L, -(1 as libc::c_int)) as *mut IOCtrl;
    let mut current: *mut FILE = 0 as *mut FILE;
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    current =
        fopen(luaL_check_lstr(L, 1 as libc::c_int, 0 as *mut size_t),
              b"a\x00" as *const u8 as *const libc::c_char);
    return setreturn(L, ctrl, current, 1 as libc::c_int);
}
unsafe extern "C" fn read_number(mut L: *mut lua_State, mut f: *mut FILE)
 -> libc::c_int {
    let mut d: libc::c_long = 0;
    if fscanf(f, b"%ld\x00" as *const u8 as *const libc::c_char,
              &mut d as *mut libc::c_long) == 1 as libc::c_int {
        lua_pushnumber(L, d);
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
    /* read fails */
}
unsafe extern "C" fn read_word(mut L: *mut lua_State, mut f: *mut FILE)
 -> libc::c_int {
    let mut c: libc::c_int = 0; /* skip spaces */
    let mut b: luaL_Buffer =
        luaL_Buffer{p: 0 as *mut libc::c_char,
                    level: 0,
                    L: 0 as *mut lua_State,
                    buffer: [0; 8192],}; /* close buffer */
    luaL_buffinit(L, &mut b); /* do not add the `\n' */
    loop  {
        c = fgetc(f); /* close buffer */
        if !(*(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                 _ISspace as libc::c_int as libc::c_ushort as libc::c_int !=
                 0) {
            break ;
        }
    }
    while c != -(1 as libc::c_int) &&
              *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                  _ISspace as libc::c_int as libc::c_ushort as libc::c_int ==
                  0 {
        (b.p <
             &mut *b.buffer.as_mut_ptr().offset(8192 as libc::c_int as isize)
                 as *mut libc::c_char || !luaL_prepbuffer(&mut b).is_null())
            as libc::c_int;
        let fresh0 = b.p;
        b.p = b.p.offset(1);
        *fresh0 = c as libc::c_char;
        c = fgetc(f)
    }
    ungetc(c, f);
    luaL_pushresult(&mut b);
    return (lua_strlen(L, -(1 as libc::c_int)) >
                0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
unsafe extern "C" fn read_line(mut L: *mut lua_State, mut f: *mut FILE)
 -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut b: luaL_Buffer =
        luaL_Buffer{p: 0 as *mut libc::c_char,
                    level: 0,
                    L: 0 as *mut lua_State,
                    buffer: [0; 8192],};
    luaL_buffinit(L, &mut b);
    loop  {
        let mut p: *mut libc::c_char = luaL_prepbuffer(&mut b);
        if fgets(p, 8192 as libc::c_int, f).is_null() { break ; }
        n = strlen(p) as libc::c_int;
        if *p.offset((n - 1 as libc::c_int) as isize) as libc::c_int !=
               '\n' as i32 {
            b.p = b.p.offset(n as isize)
        } else { b.p = b.p.offset((n - 1 as libc::c_int) as isize); break ; }
    }
    luaL_pushresult(&mut b);
    return (n > 0 as libc::c_int) as libc::c_int;
    /* read something? */
}
unsafe extern "C" fn read_file(mut L: *mut lua_State, mut f: *mut FILE) {
    let mut len: size_t =
        0 as libc::c_int as size_t; /* did not read all it could */
    let mut size: size_t = 8192 as libc::c_int as size_t; /* get _INPUT */
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    loop  {
        let mut newbuffer: *mut libc::c_char =
            if buffer.is_null() {
                malloc(size)
            } else { realloc(buffer as *mut libc::c_void, size) } as
                *mut libc::c_char;
        if newbuffer.is_null() {
            if !buffer.is_null() { free(buffer as *mut libc::c_void); }
            lua_error(L,
                      b"not enough memory to read a file\x00" as *const u8 as
                          *const libc::c_char);
        }
        buffer = newbuffer;
        len =
            (len as
                 libc::c_ulong).wrapping_add(fread(buffer.offset(len as isize)
                                                       as *mut libc::c_void,
                                                   ::std::mem::size_of::<libc::c_char>()
                                                       as libc::c_ulong,
                                                   size.wrapping_sub(len), f))
                as size_t as size_t;
        if len < size { break ; }
        size =
            (size as
                 libc::c_ulong).wrapping_mul(2 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t
    }
    lua_pushlstring(L, buffer, len);
    if !buffer.is_null() { free(buffer as *mut libc::c_void); };
}
unsafe extern "C" fn read_chars(mut L: *mut lua_State, mut f: *mut FILE,
                                mut n: size_t) -> libc::c_int {
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n1: size_t = 0;
    let mut statbuff: [libc::c_char; 8192] = [0; 8192];
    if n <= 8192 as libc::c_int as libc::c_ulong {
        buffer = statbuff.as_mut_ptr()
    } else {
        buffer = malloc(n) as *mut libc::c_char;
        if buffer.is_null() {
            lua_error(L,
                      b"not enough memory to read a file\x00" as *const u8 as
                          *const libc::c_char);
        }
    }
    n1 =
        fread(buffer as *mut libc::c_void,
              ::std::mem::size_of::<libc::c_char>() as libc::c_ulong, n, f);
    lua_pushlstring(L, buffer, n1);
    if buffer != statbuff.as_mut_ptr() {
        if !buffer.is_null() { free(buffer as *mut libc::c_void); }
    }
    return (n1 > 0 as libc::c_int as libc::c_ulong ||
                n == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
unsafe extern "C" fn io_read(mut L: *mut lua_State) -> libc::c_int {
    let mut current_block: u64;
    let mut ctrl: *mut IOCtrl =
        lua_touserdata(L, -(1 as libc::c_int)) as *mut IOCtrl;
    let mut lastarg: libc::c_int = lua_gettop(L) - 1 as libc::c_int;
    let mut firstarg: libc::c_int = 1 as libc::c_int;
    let mut f: *mut FILE = gethandle(L, ctrl, firstarg);
    let mut n: libc::c_int = 0;
    if !f.is_null() {
        firstarg += 1
    } else { f = getfilebyref(L, ctrl, 0 as libc::c_int) }
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    if firstarg > lastarg {
        /* no arguments? */
        lua_settop(L,
                   0 as
                       libc::c_int); /* erase upvalue and other eventual garbage */
        /* push default argument */
        lastarg = 1 as libc::c_int; /* correct indices */
        firstarg = lastarg;
        lua_pushstring(L, b"*l\x00" as *const u8 as *const libc::c_char);
    } else {
        /* ensure stack space for all results and for auxlib's buffer */
        luaL_checkstack(L,
                        lastarg - firstarg + 1 as libc::c_int +
                            20 as libc::c_int,
                        b"too many arguments\x00" as *const u8 as
                            *const libc::c_char); /* deprecated! */
    }
    n = firstarg;
    while n <= lastarg {
        let mut success: libc::c_int = 0;
        if lua_isnumber(L, n) != 0 {
            success = read_chars(L, f, lua_tonumber(L, n) as size_t);
            current_block = 8693738493027456495;
        } else {
            let mut p: *const libc::c_char =
                luaL_check_lstr(L, n, 0 as *mut size_t);
            if *p.offset(0 as libc::c_int as isize) as libc::c_int !=
                   '*' as i32 {
                lua_error(L,
                          b"read patterns are deprecated\x00" as *const u8 as
                              *const libc::c_char);
                success = 0 as libc::c_int;
                current_block = 8693738493027456495;
            } else {
                match *p.offset(1 as libc::c_int as isize) as libc::c_int {
                    110 => {
                        /* number */
                        if read_number(L, f) == 0 { break ; }
                        current_block = 1841672684692190573;
                    }
                    108 => {
                        /* line */
                        success = read_line(L, f);
                        current_block = 8693738493027456495;
                    }
                    97 => {
                        /* file */
                        read_file(L, f); /* always success */
                        success = 1 as libc::c_int;
                        current_block = 8693738493027456495;
                    }
                    119 => {
                        /* word */
                        success = read_word(L, f); /* remove last result */
                        current_block = 8693738493027456495;
                    }
                    _ => {
                        luaL_argerror(L, n,
                                      b"invalid format\x00" as *const u8 as
                                          *const libc::c_char);
                        success = 0 as libc::c_int;
                        current_block = 8693738493027456495;
                    }
                }
            }
        }
        match current_block {
            8693738493027456495 => {
                if success == 0 {
                    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
                    break ;
                    /* read fails */
                }
            }
            _ => { }
        } /* number is already pushed; avoid the "pushstring" */
        n += 1
    }
    return n - firstarg;
}
/* }====================================================== */
unsafe extern "C" fn io_write(mut L: *mut lua_State) -> libc::c_int {
    let mut lastarg: libc::c_int =
        lua_gettop(L) - 1 as libc::c_int; /* get _OUTPUT */
    let mut ctrl: *mut IOCtrl =
        lua_touserdata(L, -(1 as libc::c_int)) as *mut IOCtrl;
    let mut arg: libc::c_int = 1 as libc::c_int;
    let mut status: libc::c_int = 1 as libc::c_int;
    let mut f: *mut FILE = gethandle(L, ctrl, arg);
    if !f.is_null() {
        arg += 1
    } else { f = getfilebyref(L, ctrl, 1 as libc::c_int) }
    while arg <= lastarg {
        if lua_type(L, arg) == 2 as libc::c_int {
            /* LUA_NUMBER */
            /* optimization: could be done exactly as for strings */
            status =
                (status != 0 &&
                     fprintf(f,
                             b"%ld\x00" as *const u8 as *const libc::c_char,
                             lua_tonumber(L, arg)) > 0 as libc::c_int) as
                    libc::c_int
        } else {
            let mut l: size_t = 0; /* remove upvalue */
            let mut s: *const libc::c_char =
                luaL_check_lstr(L, arg, &mut l); /* error */
            status =
                (status != 0 &&
                     fwrite(s as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_char>() as
                                libc::c_ulong, l, f) == l) as libc::c_int
        } /* remove upvalue */
        arg += 1
    }
    pushresult(L, status);
    return 1 as libc::c_int;
}
unsafe extern "C" fn io_seek(mut L: *mut lua_State) -> libc::c_int {
    static mut mode: [libc::c_int; 3] =
        [0 as libc::c_int, 1 as libc::c_int, 2 as libc::c_int];
    static mut modenames: [*const libc::c_char; 4] =
        [b"set\x00" as *const u8 as *const libc::c_char,
         b"cur\x00" as *const u8 as *const libc::c_char,
         b"end\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char];
    let mut ctrl: *mut IOCtrl =
        lua_touserdata(L, -(1 as libc::c_int)) as *mut IOCtrl;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut op: libc::c_int = 0;
    let mut offset: libc::c_long = 0;
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    f = getnonullfile(L, ctrl, 1 as libc::c_int);
    op =
        luaL_findstring(luaL_opt_lstr(L, 2 as libc::c_int,
                                      b"cur\x00" as *const u8 as
                                          *const libc::c_char,
                                      0 as *mut size_t), modenames.as_ptr());
    offset =
        luaL_opt_number(L, 3 as libc::c_int,
                        0 as libc::c_int as libc::c_long);
    if !(op != -(1 as libc::c_int)) {
        luaL_argerror(L, 2 as libc::c_int,
                      b"invalid mode\x00" as *const u8 as
                          *const libc::c_char);
    }
    op = fseek(f, offset, mode[op as usize]);
    if op != 0 {
        return pushresult(L, 0 as libc::c_int)
    } else { lua_pushnumber(L, ftell(f)); return 1 as libc::c_int };
}
unsafe extern "C" fn io_flush(mut L: *mut lua_State) -> libc::c_int {
    let mut ctrl: *mut IOCtrl =
        lua_touserdata(L, -(1 as libc::c_int)) as *mut IOCtrl;
    let mut f: *mut FILE = 0 as *mut FILE;
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    f = gethandle(L, ctrl, 1 as libc::c_int);
    if !(!f.is_null() || lua_type(L, 1 as libc::c_int) == -(1 as libc::c_int))
       {
        luaL_argerror(L, 1 as libc::c_int,
                      b"invalid file handle\x00" as *const u8 as
                          *const libc::c_char);
    }
    return pushresult(L, (fflush(f) == 0 as libc::c_int) as libc::c_int);
}
/* }====================================================== */
/*
** {======================================================
** Other O.S. Operations
** =======================================================
*/
unsafe extern "C" fn io_execute(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L,
                   system(luaL_check_lstr(L, 1 as libc::c_int,
                                          0 as *mut size_t)) as
                       libc::c_long); /* if NULL push nil */
    return 1 as libc::c_int;
}
unsafe extern "C" fn io_remove(mut L: *mut lua_State) -> libc::c_int {
    return pushresult(L,
                      (remove(luaL_check_lstr(L, 1 as libc::c_int,
                                              0 as *mut size_t)) ==
                           0 as libc::c_int) as libc::c_int);
}
unsafe extern "C" fn io_rename(mut L: *mut lua_State) -> libc::c_int {
    return pushresult(L,
                      (rename(luaL_check_lstr(L, 1 as libc::c_int,
                                              0 as *mut size_t),
                              luaL_check_lstr(L, 2 as libc::c_int,
                                              0 as *mut size_t)) ==
                           0 as libc::c_int) as libc::c_int);
}
unsafe extern "C" fn io_getenv(mut L: *mut lua_State) -> libc::c_int {
    lua_pushstring(L,
                   getenv(luaL_check_lstr(L, 1 as libc::c_int,
                                          0 as *mut size_t)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn io_clock(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, clock() / 1000000 as libc::c_int as __clock_t);
    return 1 as libc::c_int;
}
unsafe extern "C" fn io_date(mut L: *mut lua_State) -> libc::c_int {
    let mut b: [libc::c_char; 256] = [0; 256];
    let mut s: *const libc::c_char =
        luaL_opt_lstr(L, 1 as libc::c_int,
                      b"%c\x00" as *const u8 as *const libc::c_char,
                      0 as *mut size_t);
    let mut stm: *mut tm = 0 as *mut tm;
    let mut t: time_t = 0;
    time(&mut t);
    stm = localtime(&mut t);
    if strftime(b.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                s, stm) != 0 {
        lua_pushstring(L, b.as_mut_ptr());
    } else {
        lua_error(L,
                  b"invalid `date\' format\x00" as *const u8 as
                      *const libc::c_char);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn setloc(mut L: *mut lua_State) -> libc::c_int {
    static mut cat: [libc::c_int; 6] =
        [6 as libc::c_int, 3 as libc::c_int, 0 as libc::c_int,
         4 as libc::c_int, 1 as libc::c_int, 2 as libc::c_int];
    static mut catnames: [*const libc::c_char; 7] =
        [b"all\x00" as *const u8 as *const libc::c_char,
         b"collate\x00" as *const u8 as *const libc::c_char,
         b"ctype\x00" as *const u8 as *const libc::c_char,
         b"monetary\x00" as *const u8 as *const libc::c_char,
         b"numeric\x00" as *const u8 as *const libc::c_char,
         b"time\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char];
    let mut op: libc::c_int =
        luaL_findstring(luaL_opt_lstr(L, 2 as libc::c_int,
                                      b"all\x00" as *const u8 as
                                          *const libc::c_char,
                                      0 as *mut size_t), catnames.as_ptr());
    if !(op != -(1 as libc::c_int)) {
        luaL_argerror(L, 2 as libc::c_int,
                      b"invalid option\x00" as *const u8 as
                          *const libc::c_char);
    }
    lua_pushstring(L,
                   setlocale(cat[op as usize],
                             luaL_check_lstr(L, 1 as libc::c_int,
                                             0 as *mut size_t)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn io_exit(mut L: *mut lua_State) -> libc::c_int {
    exit(luaL_opt_number(L, 1 as libc::c_int,
                         0 as libc::c_int as libc::c_long) as libc::c_int);
    /* to avoid warnings */
}
/* }====================================================== */
unsafe extern "C" fn io_debug(mut L: *mut lua_State) -> libc::c_int {
    loop  {
        let mut buffer: [libc::c_char; 250] = [0; 250];
        fprintf(stderr,
                b"lua_debug> \x00" as *const u8 as *const libc::c_char);
        if fgets(buffer.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 250]>() as libc::c_ulong
                     as libc::c_int, stdin).is_null() ||
               strcmp(buffer.as_mut_ptr(),
                      b"cont\n\x00" as *const u8 as *const libc::c_char) ==
                   0 as libc::c_int {
            return 0 as libc::c_int
        }
        lua_dostring(L, buffer.as_mut_ptr());
        lua_settop(L, 0 as libc::c_int);
        /* remove eventual returns */
    };
}
/* size of the second part of the stack */
unsafe extern "C" fn errorfb(mut L: *mut lua_State) -> libc::c_int {
    let mut level: libc::c_int =
        1 as libc::c_int; /* skip level 0 (it's this function) */
    let mut firstpart: libc::c_int =
        1 as libc::c_int; /* still before eventual `...' */
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
                  _func:
                      0 as
                          *mut lua_TObject,}; /* enough to fit following `sprintf's */
    let mut b: luaL_Buffer =
        luaL_Buffer{p: 0 as *mut libc::c_char,
                    level: 0,
                    L: 0 as *mut lua_State,
                    buffer: [0; 8192],};
    luaL_buffinit(L, &mut b);
    luaL_addstring(&mut b,
                   b"error: \x00" as *const u8 as *const libc::c_char);
    luaL_addstring(&mut b,
                   luaL_check_lstr(L, 1 as libc::c_int, 0 as *mut size_t));
    luaL_addstring(&mut b, b"\n\x00" as *const u8 as *const libc::c_char);
    loop  {
        let fresh1 = level;
        level = level + 1;
        if !(lua_getstack(L, fresh1, &mut ar) != 0) { break ; }
        let mut buff: [libc::c_char; 120] = [0; 120];
        if level == 2 as libc::c_int {
            luaL_addstring(&mut b,
                           b"stack traceback:\n\x00" as *const u8 as
                               *const libc::c_char);
        } else if level > 12 as libc::c_int && firstpart != 0 {
            /* no more than `LEVELS2' more levels? */
            if lua_getstack(L, level + 10 as libc::c_int, &mut ar) == 0 {
                level -= 1
            } else { /* keep going */
                luaL_addstring(&mut b,
                               b"       ...\n\x00" as *const u8 as
                                   *const libc::c_char); /* too many levels */
                while lua_getstack(L, level + 10 as libc::c_int, &mut ar) != 0
                      {
                    /* find last levels */
                    level += 1
                }
            }
            firstpart = 0 as libc::c_int;
            continue ;
        }
        sprintf(buff.as_mut_ptr(),
                b"%4d:  \x00" as *const u8 as *const libc::c_char,
                level - 1 as libc::c_int);
        luaL_addstring(&mut b, buff.as_mut_ptr());
        lua_getinfo(L, b"Snl\x00" as *const u8 as *const libc::c_char,
                    &mut ar);
        match *ar.namewhat as libc::c_int {
            103 | 108 => {
                /* global, local */
                sprintf(buff.as_mut_ptr(),
                        b"function `%.50s\'\x00" as *const u8 as
                            *const libc::c_char, ar.name);
            }
            102 => {
                /* field */
                sprintf(buff.as_mut_ptr(),
                        b"method `%.50s\'\x00" as *const u8 as
                            *const libc::c_char, ar.name);
            }
            116 => {
                /* tag method */
                sprintf(buff.as_mut_ptr(),
                        b"`%.50s\' tag method\x00" as *const u8 as
                            *const libc::c_char, ar.name);
            }
            _ => {
                if *ar.what as libc::c_int == 'm' as i32 {
                    /* main? */
                    sprintf(buff.as_mut_ptr(),
                            b"main of %.70s\x00" as *const u8 as
                                *const libc::c_char,
                            ar.short_src.as_mut_ptr());
                } else if *ar.what as libc::c_int == 'C' as i32 {
                    /* C function? */
                    sprintf(buff.as_mut_ptr(),
                            b"%.70s\x00" as *const u8 as *const libc::c_char,
                            ar.short_src.as_mut_ptr());
                } else {
                    sprintf(buff.as_mut_ptr(),
                            b"function <%d:%.70s>\x00" as *const u8 as
                                *const libc::c_char, ar.linedefined,
                            ar.short_src.as_mut_ptr());
                }
                ar.source = 0 as *const libc::c_char
                /* do not print source again */
            }
        }
        luaL_addstring(&mut b, buff.as_mut_ptr());
        if ar.currentline > 0 as libc::c_int {
            sprintf(buff.as_mut_ptr(),
                    b" at line %d\x00" as *const u8 as *const libc::c_char,
                    ar.currentline);
            luaL_addstring(&mut b, buff.as_mut_ptr());
        }
        if !ar.source.is_null() {
            sprintf(buff.as_mut_ptr(),
                    b" [%.70s]\x00" as *const u8 as *const libc::c_char,
                    ar.short_src.as_mut_ptr());
            luaL_addstring(&mut b, buff.as_mut_ptr());
        }
        luaL_addstring(&mut b, b"\n\x00" as *const u8 as *const libc::c_char);
    }
    luaL_pushresult(&mut b);
    lua_getglobal(L, b"_ALERT\x00" as *const u8 as *const libc::c_char);
    if lua_type(L, -(1 as libc::c_int)) == 5 as libc::c_int {
        /* avoid loop if _ALERT is not defined */
        lua_pushvalue(L, -(2 as libc::c_int)); /* error message */
        lua_rawcall(L, 1 as libc::c_int, 0 as libc::c_int);
    }
    return 0 as libc::c_int;
}
static mut iolib: [luaL_reg; 10] =
    unsafe {
        [{
             let mut init =
                 luaL_reg{name:
                              b"_ERRORMESSAGE\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(errorfb as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"clock\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(io_clock as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"date\x00" as *const u8 as *const libc::c_char,
                          func:
                              Some(io_date as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"debug\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(io_debug as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"execute\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(io_execute as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"exit\x00" as *const u8 as *const libc::c_char,
                          func:
                              Some(io_exit as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"getenv\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(io_getenv as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"remove\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(io_remove as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"rename\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(io_rename as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"setlocale\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(setloc as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         }]
    };
static mut iolibtag: [luaL_reg; 9] =
    unsafe {
        [{
             let mut init =
                 luaL_reg{name:
                              b"appendto\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(io_appendto as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"closefile\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(io_close as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"flush\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(io_flush as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"openfile\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(io_open as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"read\x00" as *const u8 as *const libc::c_char,
                          func:
                              Some(io_read as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"readfrom\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(io_readfrom as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"seek\x00" as *const u8 as *const libc::c_char,
                          func:
                              Some(io_seek as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"write\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(io_write as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         },
         {
             let mut init =
                 luaL_reg{name:
                              b"writeto\x00" as *const u8 as
                                  *const libc::c_char,
                          func:
                              Some(io_writeto as
                                       unsafe extern "C" fn(_: *mut lua_State)
                                           -> libc::c_int),};
             init
         }]
    };
unsafe extern "C" fn openwithcontrol(mut L: *mut lua_State) {
    let mut ctrl: *mut IOCtrl =
        lua_newuserdata(L, ::std::mem::size_of::<IOCtrl>() as libc::c_ulong)
            as *mut IOCtrl;
    let mut i: libc::c_uint = 0;
    (*ctrl).iotag = lua_newtag(L);
    (*ctrl).closedtag = lua_newtag(L);
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[luaL_reg; 9]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<luaL_reg>()
                                                   as libc::c_ulong) {
        /* put `ctrl' as upvalue for these functions */
        lua_pushvalue(L, -(1 as libc::c_int));
        lua_pushcclosure(L, iolibtag[i as usize].func, 1 as libc::c_int);
        lua_setglobal(L, iolibtag[i as usize].name);
        i = i.wrapping_add(1)
    }
    /* create references to variable names */
    lua_pushstring(L, filenames[0 as libc::c_int as usize]);
    (*ctrl).ref_0[0 as libc::c_int as usize] = lua_ref(L, 1 as libc::c_int);
    lua_pushstring(L, filenames[1 as libc::c_int as usize]);
    (*ctrl).ref_0[1 as libc::c_int as usize] = lua_ref(L, 1 as libc::c_int);
    /* predefined file handles */
    setfilebyname(L, ctrl, stdin, filenames[0 as libc::c_int as usize]);
    setfilebyname(L, ctrl, stdout, filenames[1 as libc::c_int as usize]);
    setfilebyname(L, ctrl, stdin,
                  b"_STDIN\x00" as *const u8 as *const libc::c_char);
    setfilebyname(L, ctrl, stdout,
                  b"_STDOUT\x00" as *const u8 as *const libc::c_char);
    setfilebyname(L, ctrl, stderr,
                  b"_STDERR\x00" as *const u8 as *const libc::c_char);
    /* close files when collected */
    lua_pushcclosure(L,
                     Some(file_collect as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int),
                     1 as libc::c_int); /* pops `ctrl' from stack */
    lua_settagmethod(L, (*ctrl).iotag,
                     b"gc\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn lua_iolibopen(mut L: *mut lua_State) {
    luaL_openlib(L, iolib.as_ptr(),
                 (::std::mem::size_of::<[luaL_reg; 10]>() as
                      libc::c_ulong).wrapping_div(::std::mem::size_of::<luaL_reg>()
                                                      as libc::c_ulong) as
                     libc::c_int);
    openwithcontrol(L);
}
