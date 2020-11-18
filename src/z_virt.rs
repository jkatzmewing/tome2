use ::libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn core(str: cptr);
}
pub type size_t = libc::c_ulong;
pub type vptr = *mut libc::c_void;
pub type cptr = *const libc::c_char;
pub type errr = libc::c_int;
pub type huge_hack = libc::c_ulong;
/* File: z-virt.c */
/*
 * Copyright (c) 1997 Ben Harrison
 *
 * This software may be copied and distributed for educational, research,
 * and not for profit purposes provided that this copyright and statement
 * are included in all such copies.
 */
/* Purpose: Memory management routines -BEN- */
/*
 * Allow debugging messages to track memory usage.
 */
/*
 * Optional auxiliary "rnfree" function
 */
#[no_mangle]
pub static mut rnfree_aux:
           Option<unsafe extern "C" fn(_: vptr, _: huge_hack) -> vptr> =
    None;
/*
 * Free some memory (allocated by ralloc), return NULL
 */
#[no_mangle]
pub unsafe extern "C" fn rnfree(mut p: vptr, mut len: huge_hack) -> vptr {
    /* Easy to free zero bytes */
    if len == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void
    }
    /* Use the "aux" function */
    if rnfree_aux.is_some() {
        return Some(rnfree_aux.expect("non-null function pointer")).expect("non-null function pointer")(p,
                                                                                                        len)
    }
    /* Use "free" */
    free(p as *mut libc::c_char as *mut libc::c_void);
    /* Done */
    return 0 as *mut libc::c_void;
}
/*
 * Optional auxiliary "rpanic" function
 */
#[no_mangle]
pub static mut rpanic_aux: Option<unsafe extern "C" fn(_: huge_hack) -> vptr>
           =
    None;
/*
 * The system is out of memory, so panic.  If "rpanic_aux" is set,
 * it can be used to free up some memory and do a new "ralloc()",
 * or if not, it can be used to save things, clean up, and exit.
 * By default, this function simply crashes the computer.
 */
#[no_mangle]
pub unsafe extern "C" fn rpanic(mut len: huge_hack) -> vptr {
    /* Hopefully, we have a real "panic" function */
    if rpanic_aux.is_some() {
        return Some(rpanic_aux.expect("non-null function pointer")).expect("non-null function pointer")(len)
    }
    /* Attempt to crash before icky things happen */
    core(b"Out of Memory!\x00" as *const u8 as *const libc::c_char);
    /* Paranoia */
    return 0 as *mut libc::c_void;
}
/*
 * Optional auxiliary "ralloc" function
 */
#[no_mangle]
pub static mut ralloc_aux: Option<unsafe extern "C" fn(_: huge_hack) -> vptr>
           =
    None;
/*
 * Allocate some memory
 */
#[no_mangle]
pub unsafe extern "C" fn ralloc(mut len: huge_hack) -> vptr {
    let mut mem: vptr = 0 as *mut libc::c_void;
    /* Allow allocation of "zero bytes" */
    if len == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void
    }
    /* Use the aux function if set */
    if ralloc_aux.is_some() {
        mem =
            Some(ralloc_aux.expect("non-null function pointer")).expect("non-null function pointer")(len)
    } else {
        /* Use malloc() to allocate some memory */
        mem = malloc(len)
    }
    /* We were able to acquire memory */
    if mem.is_null() { mem = rpanic(len) }
    /* Return the memory, if any */
    return mem;
}
/*
 * Allocate a constant string, containing the same thing as 'str'
 */
#[no_mangle]
pub unsafe extern "C" fn string_make(mut str: cptr) -> cptr {
    let mut len: huge_hack = 0 as libc::c_int as huge_hack;
    let mut t: cptr = str;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Simple sillyness */
    if str.is_null() { return str }
    loop 
         /* Get the number of chars in the string, including terminator */
         {
        let fresh0 = len;
        len = len.wrapping_add(1);
        if !(*str.offset(fresh0 as isize) != 0) { break ; }
    }
    /* Allocate space for the string */
    res = ralloc(len) as *mut libc::c_char;
    s = res;
    loop 
         /* Copy the string (with terminator) */
         {
        let fresh1 = t;
        t = t.offset(1);
        let fresh2 = s;
        s = s.offset(1);
        *fresh2 = *fresh1;
        if !(*fresh2 as libc::c_int != 0 as libc::c_int) { break ; }
    }
    /* Return the allocated, initialized, string */
    return res as cptr;
}
/*
 * Un-allocate a string allocated above.
 * Depends on no changes being made to the string.
 */
#[no_mangle]
pub unsafe extern "C" fn string_free(mut str: cptr) -> errr {
    let mut len: huge_hack = 0 as libc::c_int as huge_hack;
    /* Succeed on non-strings */
    if str.is_null() { return 0 as libc::c_int }
    loop 
         /* Count the number of chars in 'str' plus the terminator */
         {
        let fresh3 = len;
        len = len.wrapping_add(1);
        if !(*str.offset(fresh3 as isize) != 0) { break ; }
    }
    /* Kill the buffer of chars we must have allocated above */
    rnfree(str as vptr, len);
    /* Success */
    return 0 as libc::c_int;
}
