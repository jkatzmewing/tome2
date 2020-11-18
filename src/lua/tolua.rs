use ::libc;
extern "C" {
    pub type lua_State;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn lua_open(stacksize: libc::c_int) -> *mut lua_State;
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn lua_newtable(L: *mut lua_State);
    #[no_mangle]
    fn lua_setglobal(L: *mut lua_State, name: *const libc::c_char);
    #[no_mangle]
    fn lua_settable(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_baselibopen(L: *mut lua_State);
    #[no_mangle]
    fn lua_iolibopen(L: *mut lua_State);
    #[no_mangle]
    fn lua_strlibopen(L: *mut lua_State);
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
/* tolua
** Support code for Lua bindings.
** Written by Waldemar Celes
** TeCGraf/PUC-Rio
** Jul 1998
** $Id: tolua.c,v 1.4 2004/06/04 13:42:10 neil Exp $
*/
/* This code is free software; you can redistribute it and/or modify it. 
** The software provided hereunder is on an "as is" basis, and 
** the author has no obligation to provide maintenance, support, updates,
** enhancements, or modifications. 
*/
unsafe extern "C" fn help() {
    fprintf(stderr,
            b"\nusage: tolua [options] input_file\n\nCommand line options are:\n  -v       : print version information.\n  -o  file : set output file; default is stdout.\n  -H  file : create include file.\n  -n  name : set package name; default is input file root name.\n  -p       : parse only.\n  -P       : parse and print structure information (for debug).\n  -h       : print this message.\nShould the input file be omitted, stdin is assumed;\nin that case, the package name must be explicitly set.\n\n\x00"
                as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn version() {
    fprintf(stderr,
            b"%s (written by W. Celes)\n\x00" as *const u8 as
                *const libc::c_char,
            b"tolua 4.0a - angband\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn setfield(mut L: *mut lua_State, mut table: libc::c_int,
                              mut f: *mut libc::c_char,
                              mut v: *mut libc::c_char) {
    lua_pushstring(L, f);
    lua_pushstring(L, v);
    lua_settable(L, table);
}
unsafe extern "C" fn error(mut o: *mut libc::c_char) {
    fprintf(stderr,
            b"tolua: unknown option \'%s\'\n\x00" as *const u8 as
                *const libc::c_char, o);
    help();
    exit(1 as libc::c_int);
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut L: *mut lua_State = lua_open(0 as libc::c_int);
    lua_baselibopen(L);
    lua_iolibopen(L);
    lua_strlibopen(L);
    lua_pushstring(L,
                   b"tolua 4.0a - angband\x00" as *const u8 as
                       *const libc::c_char);
    lua_setglobal(L,
                  b"TOLUA_VERSION\x00" as *const u8 as *const libc::c_char);
    if argc == 1 as libc::c_int {
        help();
        return 0 as libc::c_int
    } else {
        let mut i: libc::c_int = 0;
        let mut t: libc::c_int = 0;
        lua_newtable(L);
        lua_pushvalue(L, -(1 as libc::c_int));
        lua_setglobal(L, b"flags\x00" as *const u8 as *const libc::c_char);
        t = lua_gettop(L);
        i = 1 as libc::c_int;
        while i < argc {
            if **argv.offset(i as isize) as libc::c_int == '-' as i32 {
                match *(*argv.offset(i as
                                         isize)).offset(1 as libc::c_int as
                                                            isize) as
                          libc::c_int {
                    118 => { version(); return 0 as libc::c_int }
                    104 => { help(); return 0 as libc::c_int }
                    112 => {
                        setfield(L, t,
                                 b"p\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char,
                                 b"\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char);
                    }
                    80 => {
                        setfield(L, t,
                                 b"P\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char,
                                 b"\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char);
                    }
                    111 => {
                        i += 1;
                        setfield(L, t,
                                 b"o\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char,
                                 *argv.offset(i as isize));
                    }
                    110 => {
                        i += 1;
                        setfield(L, t,
                                 b"n\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char,
                                 *argv.offset(i as isize));
                    }
                    72 => {
                        i += 1;
                        setfield(L, t,
                                 b"H\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char,
                                 *argv.offset(i as isize));
                    }
                    _ => { error(*argv.offset(i as isize)); }
                }
                i += 1
            } else {
                setfield(L, t,
                         b"f\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char, *argv.offset(i as isize));
                break ;
            }
        }
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    }
    extern "C" {
        #[link_name = "tolua_tolualua_open"]
        fn tolua_tolualua_open_0(L_0: *mut lua_State) -> libc::c_int;
    }
    tolua_tolualua_open_0(L);
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
