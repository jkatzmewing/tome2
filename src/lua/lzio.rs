use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn fread(_: *mut libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
             _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    fn feof(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zio {
    pub n: size_t,
    pub p: *const libc::c_uchar,
    pub filbuf: Option<unsafe extern "C" fn(_: *mut ZIO) -> libc::c_int>,
    pub u: *mut libc::c_void,
    pub name: *const libc::c_char,
    pub buffer: [libc::c_uchar; 256],
}
pub type ZIO = zio;
/*
** $Id: lzio.c,v 1.5 2004/06/04 13:42:10 neil Exp $
** a generic input stream interface
** See Copyright Notice in lua.h
*/
/* ----------------------------------------------------- memory buffers --- */
unsafe extern "C" fn zmfilbuf(mut z: *mut ZIO) -> libc::c_int {
    /* to avoid warnings */
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn luaZ_mopen(mut z: *mut ZIO,
                                    mut b: *const libc::c_char,
                                    mut size: size_t,
                                    mut name: *const libc::c_char)
 -> *mut ZIO {
    if b.is_null() { return 0 as *mut ZIO }
    (*z).n = size;
    (*z).p = b as *const libc::c_uchar;
    (*z).filbuf =
        Some(zmfilbuf as unsafe extern "C" fn(_: *mut ZIO) -> libc::c_int);
    (*z).u = 0 as *mut libc::c_void;
    (*z).name = name;
    return z;
}
/* ------------------------------------------------------------ strings --- */
#[no_mangle]
pub unsafe extern "C" fn luaZ_sopen(mut z: *mut ZIO,
                                    mut s: *const libc::c_char,
                                    mut name: *const libc::c_char)
 -> *mut ZIO {
    if s.is_null() { return 0 as *mut ZIO }
    return luaZ_mopen(z, s, strlen(s), name);
}
/* -------------------------------------------------------------- FILEs --- */
unsafe extern "C" fn zffilbuf(mut z: *mut ZIO) -> libc::c_int {
    let mut n: size_t = 0;
    if feof((*z).u as *mut FILE) != 0 { return -(1 as libc::c_int) }
    n =
        fread((*z).buffer.as_mut_ptr() as *mut libc::c_void,
              1 as libc::c_int as libc::c_ulong,
              256 as libc::c_int as libc::c_ulong, (*z).u as *mut FILE);
    if n == 0 as libc::c_int as libc::c_ulong { return -(1 as libc::c_int) }
    (*z).n = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    (*z).p = (*z).buffer.as_mut_ptr();
    let fresh0 = (*z).p;
    (*z).p = (*z).p.offset(1);
    return *fresh0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaZ_Fopen(mut z: *mut ZIO, mut f: *mut FILE,
                                    mut name: *const libc::c_char)
 -> *mut ZIO {
    if f.is_null() { return 0 as *mut ZIO }
    (*z).n = 0 as libc::c_int as size_t;
    (*z).p = (*z).buffer.as_mut_ptr();
    (*z).filbuf =
        Some(zffilbuf as unsafe extern "C" fn(_: *mut ZIO) -> libc::c_int);
    (*z).u = f as *mut libc::c_void;
    (*z).name = name;
    return z;
}
/* --------------------------------------------------------------- read --- */
#[no_mangle]
pub unsafe extern "C" fn luaZ_read(mut z: *mut ZIO, mut b: *mut libc::c_void,
                                   mut n: size_t) -> size_t {
    while n != 0 {
        let mut m: size_t = 0; /* return number of missing bytes */
        if (*z).n == 0 as libc::c_int as libc::c_ulong {
            if (*z).filbuf.expect("non-null function pointer")(z) ==
                   -(1 as libc::c_int) {
                return n
            }
            (*z).n = (*z).n.wrapping_add(1);
            (*z).p = (*z).p.offset(-1)
            /* put result from `filbuf' in the buffer */
        } /* min. between n and z->n */
        m = if n <= (*z).n { n } else { (*z).n };
        memcpy(b, (*z).p as *const libc::c_void, m);
        (*z).n =
            ((*z).n as libc::c_ulong).wrapping_sub(m) as size_t as size_t;
        (*z).p = (*z).p.offset(m as isize);
        b = (b as *mut libc::c_char).offset(m as isize) as *mut libc::c_void;
        n = (n as libc::c_ulong).wrapping_sub(m) as size_t as size_t
    }
    return 0 as libc::c_int as size_t;
}
