use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
pub type vptr = *mut libc::c_void;
pub type cptr = *const libc::c_char;
pub type errr = libc::c_int;
pub type byte_hack = libc::c_uchar;
pub type bool_ = libc::c_char;
pub type sint = libc::c_int;
pub type uint_hack = libc::c_uint;
pub type huge_hack = libc::c_ulong;
/* File: z-util.c */
/* Purpose: Low level utilities -BEN- */
/*
 * Global variables for temporary use
 */
#[no_mangle]
pub static mut char_tmp: libc::c_char = 0 as libc::c_int as libc::c_char;
#[no_mangle]
pub static mut byte_tmp: byte_hack = 0 as libc::c_int as byte_hack;
#[no_mangle]
pub static mut sint_tmp: sint = 0 as libc::c_int;
#[no_mangle]
pub static mut uint_tmp: uint_hack = 0 as libc::c_int as uint_hack;
#[no_mangle]
pub static mut long_tmp: libc::c_long = 0 as libc::c_int as libc::c_long;
#[no_mangle]
pub static mut huge_tmp: huge_hack = 0 as libc::c_int as huge_hack;
#[no_mangle]
pub static mut errr_tmp: errr = 0 as libc::c_int;
/*
 * Global pointers for temporary use
 */
#[no_mangle]
pub static mut cptr_tmp: cptr = 0 as cptr;
#[no_mangle]
pub static mut vptr_tmp: vptr = 0 as *const libc::c_void as *mut libc::c_void;
/*
 * Constant bool meaning true
 */
#[no_mangle]
pub static mut bool_true: bool_ = 1 as libc::c_int as bool_;
/*
 * Constant bool meaning false
 */
#[no_mangle]
pub static mut bool_false: bool_ = 0 as libc::c_int as bool_;
/*
 * Global NULL cptr
 */
#[no_mangle]
pub static mut cptr_null: cptr = 0 as cptr;
/*
 * Global NULL vptr
 */
#[no_mangle]
pub static mut vptr_null: vptr =
    0 as *const libc::c_void as *mut libc::c_void;
/*
 * Global SELF vptr
 */
#[no_mangle]
pub static mut vptr_self: vptr =
    unsafe { &vptr_self as *const vptr as *mut vptr as vptr };
/*
 * Convenient storage of the program name
 */
#[no_mangle]
pub static mut argv0: cptr = 0 as cptr;
/*
 * A routine that does nothing
 */
#[no_mangle]
pub unsafe extern "C" fn func_nothing() {
    /* Do nothing */
}
/*
 * A routine that always returns "success"
 */
#[no_mangle]
pub unsafe extern "C" fn func_success() -> errr { return 0 as libc::c_int; }
/*
 * A routine that always returns a simple "problem code"
 */
#[no_mangle]
pub unsafe extern "C" fn func_problem() -> errr { return 1 as libc::c_int; }
/*
 * A routine that always returns a simple "failure code"
 */
#[no_mangle]
pub unsafe extern "C" fn func_failure() -> errr {
    return -(1 as libc::c_int);
}
/*
 * A routine that always returns "true"
 */
#[no_mangle]
pub unsafe extern "C" fn func_true() -> bool_ {
    return 1 as libc::c_int as bool_;
}
/*
 * A routine that always returns "false"
 */
#[no_mangle]
pub unsafe extern "C" fn func_false() -> bool_ {
    return 0 as libc::c_int as bool_;
}
/*
 * Determine if string "t" is equal to string "t"
 */
#[no_mangle]
pub unsafe extern "C" fn streq(mut a: cptr, mut b: cptr) -> bool_ {
    if a.is_null() && b.is_null() { return 1 as libc::c_int as bool_ }
    if a.is_null() { return 0 as libc::c_int as bool_ }
    if b.is_null() { return 0 as libc::c_int as bool_ }
    return (strcmp(a, b) == 0) as libc::c_int as bool_;
}
/*
 * Determine if string "t" is a suffix of string "s"
 */
#[no_mangle]
pub unsafe extern "C" fn suffix(mut s: cptr, mut t: cptr) -> bool_ {
    let mut tlen: libc::c_int = strlen(t) as libc::c_int;
    let mut slen: libc::c_int = strlen(s) as libc::c_int;
    /* Check for incompatible lengths */
    if tlen > slen { return 0 as libc::c_int as bool_ }
    /* Compare "t" to the end of "s" */
    return (strcmp(s.offset(slen as isize).offset(-(tlen as isize)), t) == 0)
               as libc::c_int as bool_;
}
/*
 * Redefinable "plog" action
 */
#[no_mangle]
pub static mut plog_aux: Option<unsafe extern "C" fn(_: cptr) -> ()> = None;
/*
 * Print (or log) a "warning" message (ala "perror()")
 * Note the use of the (optional) "plog_aux" hook.
 */
#[no_mangle]
pub unsafe extern "C" fn plog(mut str: cptr) {
    /* Use the "alternative" function if possible */
    if plog_aux.is_some() {
        Some(plog_aux.expect("non-null function pointer")).expect("non-null function pointer")(str);
    } else {
        /* Just do a labeled fprintf to stderr */
        fprintf(stderr, b"%s: %s\n\x00" as *const u8 as *const libc::c_char,
                if !argv0.is_null() {
                    argv0
                } else { b"???\x00" as *const u8 as *const libc::c_char },
                str);
    };
}
/*
 * Redefinable "quit" action
 */
#[no_mangle]
pub static mut quit_aux: Option<unsafe extern "C" fn(_: cptr) -> ()> = None;
/*
 * Exit (ala "exit()").  If 'str' is NULL, do "exit(0)".
 * If 'str' begins with "+" or "-", do "exit(atoi(str))".
 * Otherwise, plog() 'str' and exit with an error code of -1.
 * But always use 'quit_aux', if set, before anything else.
 */
#[no_mangle]
pub unsafe extern "C" fn quit(mut str: cptr) {
    /* Attempt to use the aux function */
    if quit_aux.is_some() {
        Some(quit_aux.expect("non-null function pointer")).expect("non-null function pointer")(str);
    }
    /* Success */
    if str.is_null() { exit(0 as libc::c_int); }
    /* Extract a "special error code" */
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 ||
           *str.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
       {
        exit(atoi(str));
    }
    /* Send the string to plog() */
    plog(str);
    /* Failure */
    exit(-(1 as libc::c_int));
}
/*
 * Redefinable "core" action
 */
#[no_mangle]
pub static mut core_aux: Option<unsafe extern "C" fn(_: cptr) -> ()> = None;
/*
 * Dump a core file, after printing a warning message
 * As with "quit()", try to use the "core_aux()" hook first.
 */
#[no_mangle]
pub unsafe extern "C" fn core(mut str: cptr) {
    let mut crash: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Use the aux function */
    if core_aux.is_some() {
        Some(core_aux.expect("non-null function pointer")).expect("non-null function pointer")(str);
    }
    /* Dump the warning string */
    if !str.is_null() { plog(str); }
    /* Attempt to Crash */
    *crash = *crash;
    /* Be sure we exited */
    quit(b"core() failed\x00" as *const u8 as *const libc::c_char);
}
