use ::libc;
extern "C" {
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn toupper(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn plog(str: cptr);
    #[no_mangle]
    fn quit(str: cptr);
    #[no_mangle]
    fn core(str: cptr);
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn ralloc(len: huge_hack) -> vptr;
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
pub type vptr = *mut libc::c_void;
pub type cptr = *const libc::c_char;
pub type bool_ = libc::c_char;
pub type uint_hack = libc::c_uint;
pub type huge_hack = libc::c_ulong;
/* File: z-form.c */
/* Purpose: Low level text formatting -BEN- */
/*
 * Here is some information about the routines in this file.
 *
 * In general, the following routines take a "buffer", a "max length",
 * a "format string", and some "arguments", and use the format string
 * and the arguments to create a (terminated) string in the buffer
 * (using only the first "max length" bytes), and return the "length"
 * of the resulting string, not including the (mandatory) terminator.
 *
 * The format strings allow the basic "sprintf()" format sequences, though
 * some of them are processed slightly more carefully or portably, as well
 * as a few "special" sequences, including the "%r" and "%v" sequences, and
 * the "capilitization" sequences of "%C", "%S", and "%V".
 *
 * Note that some "limitations" are enforced by the current implementation,
 * for example, no "format sequence" can exceed 100 characters, including any
 * "length" restrictions, and the result of combining and "format sequence"
 * with the relevent "arguments" must not exceed 1000 characters.
 *
 * These limitations could be fixed by stealing some of the code from,
 * say, "vsprintf()" and placing it into my "vstrnfmt()" function.
 *
 * Note that a "^" inside a "format sequence" causes the first non-space
 * character in the string resulting from the combination of the format
 * sequence and the argument(s) to be "capitalized" if possible.  Note
 * that the "^" character is removed before the "standard" formatting
 * routines are called.  Likewise, a "*" inside a "format sequence" is
 * removed from the "format sequence", and replaced by the textual form
 * of the next argument in the argument list.  See examples below.
 *
 * Legal format characters: %,n,p,c,s,d,i,o,u,X,x,E,e,F,f,G,g,r,v.
 *
 * Format("%%")
 *   Append the literal "%".
 *   No legal modifiers.
 *
 * Format("%n", int *np)
 *   Save the current length into (*np).
 *   No legal modifiers.
 *
 * Format("%p", vptr v)
 *   Append the pointer "v" (implementation varies).
 *   No legal modifiers.
 *
 * Format("%E", double r)
 * Format("%F", double r)
 * Format("%G", double r)
 * Format("%e", double r)
 * Format("%f", double r)
 * Format("%g", double r)
 *   Append the double "r", in various formats.
 *
 * Format("%ld", long int i)
 *   Append the long integer "i".
 *
 * Format("%d", int i)
 *   Append the integer "i".
 *
 * Format("%lu", unsigned long int i)
 *   Append the unsigned long integer "i".
 *
 * Format("%u", unsigned int i)
 *   Append the unsigned integer "i".
 *
 * Format("%lo", unsigned long int i)
 *   Append the unsigned long integer "i", in octal.
 *
 * Format("%o", unsigned int i)
 *   Append the unsigned integer "i", in octal.
 *
 * Format("%lX", unsigned long int i)
 *   Note -- use all capital letters
 * Format("%lx", unsigned long int i)
 *   Append the unsigned long integer "i", in hexidecimal.
 *
 * Format("%X", unsigned int i)
 *   Note -- use all capital letters
 * Format("%x", unsigned int i)
 *   Append the unsigned integer "i", in hexidecimal.
 *
 * Format("%c", char c)
 *   Append the character "c".
 *   Do not use the "+" or "0" flags.
 *
 * Format("%s", cptr s)
 *   Append the string "s".
 *   Do not use the "+" or "0" flags.
 *   Note that a "NULL" value of "s" is converted to the empty string.
 *
 * Format("%V", vptr v)
 *   Note -- possibly significant mode flag
 * Format("%v", vptr v)
 *   Append the object "v", using the current "user defined print routine".
 *   User specified modifiers, often ignored.
 *
 * Format("%r", vstrnfmt_aux_func *fp)
 *   Set the "user defined print routine" (vstrnfmt_aux) to "fp".
 *   No legal modifiers.
 *
 *
 * For examples below, assume "int n = 0; int m = 100; char buf[100];",
 * plus "char *s = NULL;", and unknown values "char *txt; int i;".
 *
 * For example: "n = strnfmt(buf, -1, "(Max %d)", i);" will have a
 * similar effect as "sprintf(buf, "(Max %d)", i); n = strlen(buf);".
 *
 * For example: "(void)strnfmt(buf, 16, "%s", txt);" will have a similar
 * effect as "strncpy(buf, txt, 16); buf[15] = '\0';".
 *
 * For example: "if (strnfmt(buf, 16, "%s", txt) < 16) ..." will have
 * a similar effect as "strcpy(buf, txt)" but with bounds checking.
 *
 * For example: "s = buf; s += vstrnfmt(s, -1, ...); ..." will allow
 * multiple "appends" to "buf" (at the cost of losing the max-length info).
 *
 * For example: "s = buf; n = vstrnfmt(s+n, 100-n, ...); ..." will allow
 * multiple bounded "appends" to "buf", with constant access to "strlen(buf)".
 *
 * For example: "format("The %r%v was destroyed!", obj_desc, obj);"
 * (where "obj_desc(buf, max, fmt, obj)" will "append" a "description"
 * of the given object to the given buffer, and return the total length)
 * will return a "useful message" about the object "obj", for example,
 * "The Large Shield was destroyed!".
 *
 * For example: "format("%^-.*s", i, txt)" will produce a string containing
 * the first "i" characters of "txt", left justified, with the first non-space
 * character capitilized, if reasonable.
 */
/*
 * The "type" of the "user defined print routine" pointer
 */
pub type vstrnfmt_aux_func
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_char, _: uint_hack, _: cptr,
                                _: vptr) -> uint_hack>;
/*
 * The "default" user defined print routine.  Ignore the "fmt" string.
 */
unsafe extern "C" fn vstrnfmt_aux_dflt(mut buf: *mut libc::c_char,
                                       mut max: uint_hack, mut fmt: cptr,
                                       mut arg: vptr) -> uint_hack {
    let mut len: uint_hack = 0;
    let mut tmp: [libc::c_char; 32] = [0; 32];
    /* XXX XXX */
    fmt = if !fmt.is_null() { fmt } else { 0 as cptr };
    /* Pointer display */
    sprintf(tmp.as_mut_ptr(),
            b"<<%p>>\x00" as *const u8 as *const libc::c_char, arg);
    len = strlen(tmp.as_mut_ptr()) as uint_hack;
    if len >= max { len = max.wrapping_sub(1 as libc::c_int as libc::c_uint) }
    tmp[len as usize] = '\u{0}' as i32 as libc::c_char;
    strcpy(buf, tmp.as_mut_ptr());
    return len;
}
/*
 * The "current" user defined print routine.  It can be changed
 * dynamically by sending the proper "%r" sequence to "vstrnfmt()"
 */
static mut vstrnfmt_aux: vstrnfmt_aux_func =
    unsafe {
        Some(vstrnfmt_aux_dflt as
                 unsafe extern "C" fn(_: *mut libc::c_char, _: uint_hack,
                                      _: cptr, _: vptr) -> uint_hack)
    };
/*
 * Basic "vararg" format function.
 *
 * This function takes a buffer, a max byte count, a format string, and
 * a va_list of arguments to the format string, and uses the format string
 * and the arguments to create a string to the buffer.  The string is
 * derived from the format string and the arguments in the manner of the
 * "sprintf()" function, but with some extra "format" commands.  Note that
 * this function will never use more than the given number of bytes in the
 * buffer, preventing messy invalid memory references.  This function then
 * returns the total number of non-null bytes written into the buffer.
 *
 * Method: Let "str" be the (unlimited) created string, and let "len" be the
 * smaller of "max-1" and "strlen(str)".  We copy the first "len" chars of
 * "str" into "buf", place "\0" into buf[len], and return "len".
 *
 * In English, we do a sprintf() into "buf", a buffer with size "max",
 * and we return the resulting value of "strlen(buf)", but we allow some
 * special format commands, and we are more careful than "sprintf()".
 *
 * Typically, "max" is in fact the "size" of "buf", and thus represents
 * the "number" of chars in "buf" which are ALLOWED to be used.  An
 * alternative definition would have required "buf" to hold at least
 * "max+1" characters, and would have used that extra character only
 * in the case where "buf" was too short for the result.  This would
 * give an easy test for "overflow", but a less "obvious" semantics.
 *
 * Note that if the buffer was "too short" to hold the result, we will
 * always return "max-1", but we also return "max-1" if the buffer was
 * "just long enough".  We could have returned "max" if the buffer was
 * too short, not written a null, and forced the programmer to deal with
 * this special case, but I felt that it is better to at least give a
 * "usable" result when the buffer was too long instead of either giving
 * a memory overwrite like "sprintf()" or a non-terminted string like
 * "strncpy()".  Note that "strncpy()" also "null-pads" the result.
 *
 * Note that in most cases "just long enough" is probably "too short".
 *
 * We should also consider extracting and processing the "width" and other
 * "flags" by hand, it might be more "accurate", and it would allow us to
 * remove the limit (1000 chars) on the result of format sequences.
 *
 * Also, some sequences, such as "%+d" by hand, do not work on all machines,
 * and could thus be correctly handled here.
 *
 * Error detection in this routine is not very graceful, in particular,
 * if an error is detected in the format string, we simply "pre-terminate"
 * the given buffer to a length of zero, and return a "length" of zero.
 * The contents of "buf", except for "buf[0]", may then be undefined.
 */
#[no_mangle]
pub unsafe extern "C" fn vstrnfmt(mut buf: *mut libc::c_char,
                                  mut max: uint_hack, mut fmt: cptr,
                                  mut vp: ::std::ffi::VaList) -> uint_hack {
    let mut s: cptr = 0 as *const libc::c_char;
    /* The argument is "long" */
    let mut do_long: bool_ = 0;
    /* The argument needs "processing" */
    let mut do_xtra: bool_ = 0;
    /* Bytes used in buffer */
    let mut n: uint_hack = 0;
    /* Bytes used in format sequence */
    let mut q: uint_hack = 0;
    /* Format sequence */
    let mut aux: [libc::c_char; 128] = [0; 128];
    /* Resulting string */
    let mut tmp: [libc::c_char; 1024] = [0; 1024];
    /* Mega-Hack -- treat "illegal" length as "infinite" */
    if max == 0 { max = 32767 as libc::c_int as uint_hack }
    /* Mega-Hack -- treat "no format" as "empty string" */
    if fmt.is_null() { fmt = b"\x00" as *const u8 as *const libc::c_char }
    /* Begin the buffer */
    n = 0 as libc::c_int as uint_hack;
    /* Begin the format string */
    s = fmt;
    /* Scan the format string */
    /* All done */
    while !(*s == 0) {
        /* Normal character */
        if *s as libc::c_int != '%' as i32 {
            /* Check total length */
            if n == max.wrapping_sub(1 as libc::c_int as libc::c_uint) {
                break ;
            }
            /* Save the character */
            let fresh0 = s;
            s = s.offset(1);
            let fresh1 = n;
            n = n.wrapping_add(1);
            *buf.offset(fresh1 as isize) = *fresh0
        } else {
            /* Skip the "percent" */
            s = s.offset(1);
            /* Pre-process "%%" */
            if *s as libc::c_int == '%' as i32 {
                /* Check total length */
                if n == max.wrapping_sub(1 as libc::c_int as libc::c_uint) {
                    break ;
                }
                /* Save the percent */
                let fresh2 = n;
                n = n.wrapping_add(1);
                *buf.offset(fresh2 as isize) = '%' as i32 as libc::c_char;
                /* Skip the "%" */
                s = s.offset(1)
            } else if *s as libc::c_int == 'n' as i32 {
                let mut arg: *mut libc::c_int = 0 as *mut libc::c_int;
                /* Pre-process "%n" */
                /* Access the next argument */
                arg = vp.arg::<*mut libc::c_int>();
                /* Save the current length */
                *arg = n as libc::c_int;
                /* Skip the "n" */
                s = s.offset(1)
            } else if *s as libc::c_int == 'r' as i32 {
                /* Hack -- Pre-process "%r" */
                /* Extract the next argument, and save it (globally) */
                vstrnfmt_aux =
                    ::std::mem::transmute(vp.arg::<*mut unsafe extern "C" fn(_:
                                                                                 *mut libc::c_char,
                                                                             _:
                                                                                 uint_hack,
                                                                             _:
                                                                                 cptr,
                                                                             _:
                                                                                 vptr)
                                                            -> uint_hack>());
                /* Skip the "r" */
                s = s.offset(1)
            } else {
                /* Begin the "aux" string */
                q = 0 as libc::c_int as uint_hack;
                /* Save the "percent" */
                let fresh3 = q;
                q = q.wrapping_add(1);
                aux[fresh3 as usize] = '%' as i32 as libc::c_char;
                /* Assume no "long" argument */
                do_long = 0 as libc::c_int as bool_;
                /* Assume no "xtra" processing */
                do_xtra = 0 as libc::c_int as bool_;
                loop 
                     /* Build the "aux" string */
                     /* Error -- format sequence is not terminated */
                     {
                    if *s == 0 {
                        /* Terminate the buffer */
                        *buf.offset(0 as libc::c_int as isize) =
                            '\u{0}' as i32 as libc::c_char;
                        /* Return "error" */
                        return 0 as libc::c_int as uint_hack
                    }
                    /* Error -- format sequence may be too long */
                    if q > 100 as libc::c_int as libc::c_uint {
                        /* Terminate the buffer */
                        *buf.offset(0 as libc::c_int as isize) =
                            '\u{0}' as i32 as libc::c_char;
                        /* Return "error" */
                        return 0 as libc::c_int as uint_hack
                    }
                    /* Handle "alphabetic" chars */
                    if *(*__ctype_b_loc()).offset(*s as libc::c_int as isize)
                           as libc::c_int &
                           _ISalpha as libc::c_int as libc::c_ushort as
                               libc::c_int != 0 {
                        /* Hack -- handle "long" request */
                        if *s as libc::c_int == 'l' as i32 {
                            /* Save the character */
                            let fresh4 = s;
                            s = s.offset(1);
                            let fresh5 = q;
                            q = q.wrapping_add(1);
                            aux[fresh5 as usize] = *fresh4;
                            /* Note the "long" flag */
                            do_long = 1 as libc::c_int as bool_
                        } else if *s as libc::c_int == 'L' as i32 {
                            /* Mega-Hack -- handle "extra-long" request */
                            /* Error -- illegal format char */
                            *buf.offset(0 as libc::c_int as isize) =
                                '\u{0}' as i32 as libc::c_char;
                            /* Return "error" */
                            return 0 as libc::c_int as uint_hack
                        } else {
                            /* Handle normal end of format sequence */
                            /* Save the character */
                            let fresh6 = s;
                            s = s.offset(1);
                            let fresh7 = q;
                            q = q.wrapping_add(1);
                            aux[fresh7 as usize] = *fresh6;
                            break ;
                        }
                    } else if *s as libc::c_int == '*' as i32 {
                        let mut arg_0: libc::c_int = 0;
                        /* Handle "non-alphabetic" chars */
                        /* Hack -- Handle 'star' (for "variable length" argument) */
                        /* Access the next argument */
                        arg_0 = vp.arg::<libc::c_int>();
                        /* Hack -- append the "length" */
                        sprintf(aux.as_mut_ptr().offset(q as isize),
                                b"%d\x00" as *const u8 as *const libc::c_char,
                                arg_0);
                        /* Hack -- accept the "length" */
                        while aux[q as usize] != 0 { q = q.wrapping_add(1) }
                        /* Skip the "*" */
                        s = s.offset(1)
                    } else if *s as libc::c_int == '^' as i32 {
                        /* Mega-Hack -- Handle 'caret' (for "uppercase" request) */
                        /* Note the "xtra" flag */
                        do_xtra = 1 as libc::c_int as bool_;
                        /* Skip the "^" */
                        s = s.offset(1)
                    } else {
                        /* Collect "normal" characters (digits, "-", "+", ".", etc) */
                        /* Save the character */
                        let fresh8 = s;
                        s = s.offset(1);
                        let fresh9 = q;
                        q = q.wrapping_add(1);
                        aux[fresh9 as usize] = *fresh8
                    }
                }
                /* Terminate "aux" */
                aux[q as usize] = '\u{0}' as i32 as libc::c_char;
                /* Clear "tmp" */
                tmp[0 as libc::c_int as usize] =
                    '\u{0}' as i32 as libc::c_char;
                /* Process the "format" char */
                match aux[q.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                              usize] as libc::c_int {
                    99 => {
                        /* Simple Character -- standard format */
                        let mut arg_1: libc::c_int = 0;
                        /* Access next argument */
                        arg_1 = vp.arg::<libc::c_int>();
                        /* Format the argument */
                        sprintf(tmp.as_mut_ptr(), aux.as_mut_ptr(), arg_1);
                    }
                    100 | 105 => {
                        /* Signed Integers -- standard format */
                        if do_long != 0 {
                            let mut arg_2: libc::c_long = 0;
                            /* Access next argument */
                            arg_2 = vp.arg::<libc::c_long>();
                            /* Format the argument */
                            sprintf(tmp.as_mut_ptr(), aux.as_mut_ptr(),
                                    arg_2);
                        } else {
                            let mut arg_3: libc::c_int = 0;
                            /* Access next argument */
                            arg_3 = vp.arg::<libc::c_int>();
                            /* Format the argument */
                            sprintf(tmp.as_mut_ptr(), aux.as_mut_ptr(),
                                    arg_3);
                        }
                    }
                    117 | 111 | 120 | 88 => {
                        /* Unsigned Integers -- various formats */
                        if do_long != 0 {
                            let mut arg_4: libc::c_ulong = 0;
                            /* Access next argument */
                            arg_4 = vp.arg::<libc::c_ulong>();
                            /* Format the argument */
                            sprintf(tmp.as_mut_ptr(), aux.as_mut_ptr(),
                                    arg_4);
                        } else {
                            let mut arg_5: libc::c_uint = 0;
                            /* Access next argument */
                            arg_5 = vp.arg::<libc::c_uint>();
                            /* Format the argument */
                            sprintf(tmp.as_mut_ptr(), aux.as_mut_ptr(),
                                    arg_5);
                        }
                    }
                    102 | 101 | 69 | 103 | 71 => {
                        /* Floating Point -- various formats */
                        let mut arg_6: libc::c_double = 0.;
                        /* Access next argument */
                        arg_6 = vp.arg::<libc::c_double>();
                        /* Format the argument */
                        sprintf(tmp.as_mut_ptr(), aux.as_mut_ptr(), arg_6);
                    }
                    112 => {
                        /* Pointer -- implementation varies */
                        let mut arg_7: vptr = 0 as *mut libc::c_void;
                        /* Access next argument */
                        arg_7 = vp.arg::<vptr>();
                        /* Format the argument */
                        sprintf(tmp.as_mut_ptr(), aux.as_mut_ptr(), arg_7);
                    }
                    115 => {
                        /* String */
                        let mut arg_8: cptr = 0 as *const libc::c_char;
                        /* Access next argument */
                        arg_8 = vp.arg::<cptr>();
                        /* Hack -- convert NULL to EMPTY */
                        if arg_8.is_null() {
                            arg_8 =
                                b"\x00" as *const u8 as *const libc::c_char
                        }
                        /* Format the argument */
                        sprintf(tmp.as_mut_ptr(), aux.as_mut_ptr(), arg_8);
                    }
                    86 | 118 => {
                        /* User defined data */
                        let mut arg_9: vptr = 0 as *mut libc::c_void;
                        /* Access next argument */
                        arg_9 = vp.arg::<vptr>();
                        /* Format the "user data" */
                        vstrnfmt_aux.expect("non-null function pointer")(tmp.as_mut_ptr(),
                                                                         1000
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             uint_hack,
                                                                         aux.as_mut_ptr()
                                                                             as
                                                                             cptr,
                                                                         arg_9);
                    }
                    _ => {
                        /* Oops */
                        /* Error -- illegal format char */
                        *buf.offset(0 as libc::c_int as isize) =
                            '\u{0}' as i32 as libc::c_char;
                        /* Return "error" */
                        return 0 as libc::c_int as uint_hack
                    }
                }
                /* Mega-Hack -- handle "capitilization" */
                if do_xtra != 0 {
                    /* Now append "tmp" to "buf" */
                    q = 0 as libc::c_int as uint_hack;
                    while tmp[q as usize] != 0 {
                        /* Notice first non-space */
                        if *(*__ctype_b_loc()).offset(tmp[q as usize] as
                                                          libc::c_int as
                                                          isize) as
                               libc::c_int &
                               _ISspace as libc::c_int as libc::c_ushort as
                                   libc::c_int == 0 {
                            /* Capitalize if possible */
                            if *(*__ctype_b_loc()).offset(tmp[q as usize] as
                                                              libc::c_int as
                                                              isize) as
                                   libc::c_int &
                                   _ISlower as libc::c_int as libc::c_ushort
                                       as libc::c_int != 0 {
                                tmp[q as usize] =
                                    toupper(tmp[q as usize] as libc::c_int) as
                                        libc::c_char
                            }
                            break ;
                        } else { q = q.wrapping_add(1) }
                    }
                }
                /* Now append "tmp" to "buf" */
                q = 0 as libc::c_int as uint_hack;
                while tmp[q as usize] != 0 {
                    /* Check total length */
                    if n == max.wrapping_sub(1 as libc::c_int as libc::c_uint)
                       {
                        break ;
                    }
                    /* Save the character */
                    let fresh10 = n;
                    n = n.wrapping_add(1);
                    *buf.offset(fresh10 as isize) = tmp[q as usize];
                    q = q.wrapping_add(1)
                }
            }
        }
    }
    /* Terminate buffer */
    *buf.offset(n as isize) = '\u{0}' as i32 as libc::c_char;
    return n;
}
/* Return length */
/*
 * Do a vstrnfmt (see above) into a (growable) static buffer.
 * This buffer is usable for very short term formatting of results.
 */
#[no_mangle]
pub unsafe extern "C" fn vformat(mut fmt: cptr, mut vp: ::std::ffi::VaList)
 -> *mut libc::c_char {
    static mut format_buf: *mut libc::c_char =
        0 as *const libc::c_char as *mut libc::c_char;
    static mut format_len: huge_hack = 0 as libc::c_int as huge_hack;
    /* Initial allocation */
    if format_buf.is_null() {
        format_len = 1024 as libc::c_int as huge_hack;
        format_buf =
            memset(ralloc(format_len.wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                      as libc::c_ulong)) as
                       *mut libc::c_char as *mut libc::c_void,
                   0 as libc::c_int,
                   format_len.wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                               as libc::c_ulong)) as
                *mut libc::c_char
    }
    /* Null format yields last result */
    if fmt.is_null() { return format_buf }
    loop 
         /* Keep going until successful */
         {
        let mut len: uint_hack = 0;
        /* Build the string */
        len =
            vstrnfmt(format_buf, format_len as uint_hack, fmt,
                     vp.as_va_list());
        /* Success */
        if (len as libc::c_ulong) <
               format_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            break ;
        }
        /* Grow the buffer */
        format_buf =
            rnfree(format_buf as vptr,
                   format_len.wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                               as libc::c_ulong)) as
                *mut libc::c_char;
        format_len =
            format_len.wrapping_mul(2 as libc::c_int as libc::c_ulong);
        format_buf =
            memset(ralloc(format_len.wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                      as libc::c_ulong)) as
                       *mut libc::c_char as *mut libc::c_void,
                   0 as libc::c_int,
                   format_len.wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                               as libc::c_ulong)) as
                *mut libc::c_char
    }
    /* Return the new buffer */
    return format_buf;
}
/*
 * Do a vstrnfmt (see above) into a buffer of a given size.
 */
#[no_mangle]
pub unsafe extern "C" fn strnfmt(mut buf: *mut libc::c_char,
                                 mut max: uint_hack, mut fmt: cptr,
                                 mut args: ...) -> uint_hack {
    let mut len: uint_hack = 0;
    let mut vp: ::std::ffi::VaListImpl;
    /* Begin the Varargs Stuff */
    vp = args.clone();
    /* Do a virtual fprintf to stderr */
    len = vstrnfmt(buf, max, fmt, vp.as_va_list());
    /* End the Varargs Stuff */
    /* Return the number of bytes written */
    return len;
}
/*
 * Do a vstrnfmt (see above) into a buffer of unknown size.
 * Since the buffer size is unknown, the user better verify the args.
 */
#[no_mangle]
pub unsafe extern "C" fn strfmt(mut buf: *mut libc::c_char, mut fmt: cptr,
                                mut args: ...) -> uint_hack {
    let mut len: uint_hack = 0;
    let mut vp: ::std::ffi::VaListImpl;
    /* Begin the Varargs Stuff */
    vp = args.clone();
    /* Build the string, assume 32K buffer */
    len =
        vstrnfmt(buf, 32767 as libc::c_int as uint_hack, fmt,
                 vp.as_va_list());
    /* End the Varargs Stuff */
    /* Return the number of bytes written */
    return len;
}
/*
 * Do a vstrnfmt() into (see above) into a (growable) static buffer.
 * This buffer is usable for very short term formatting of results.
 * Note that the buffer is (technically) writable, but only up to
 * the length of the string contained inside it.
 */
#[no_mangle]
pub unsafe extern "C" fn format(mut fmt: cptr, mut args: ...)
 -> *mut libc::c_char {
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vp: ::std::ffi::VaListImpl;
    /* Begin the Varargs Stuff */
    vp = args.clone();
    /* Format the args */
    res = vformat(fmt, vp.as_va_list());
    /* End the Varargs Stuff */
    /* Return the result */
    return res;
}
/*
 * Vararg interface to plog()
 */
#[no_mangle]
pub unsafe extern "C" fn plog_fmt(mut fmt: cptr, mut args: ...) {
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vp: ::std::ffi::VaListImpl;
    /* Begin the Varargs Stuff */
    vp = args.clone();
    /* Format the args */
    res = vformat(fmt, vp.as_va_list());
    /* End the Varargs Stuff */
    /* Call plog */
    plog(res as cptr);
}
/*
 * Vararg interface to quit()
 */
#[no_mangle]
pub unsafe extern "C" fn quit_fmt(mut fmt: cptr, mut args: ...) {
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vp: ::std::ffi::VaListImpl;
    /* Begin the Varargs Stuff */
    vp = args.clone();
    /* Format */
    res = vformat(fmt, vp.as_va_list());
    /* End the Varargs Stuff */
    /* Call quit() */
    quit(res as cptr);
}
/*
 * Vararg interface to core()
 */
#[no_mangle]
pub unsafe extern "C" fn core_fmt(mut fmt: cptr, mut args: ...) {
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vp: ::std::ffi::VaListImpl;
    /* Begin the Varargs Stuff */
    vp = args.clone();
    /* If requested, Do a virtual fprintf to stderr */
    res = vformat(fmt, vp.as_va_list());
    /* End the Varargs Stuff */
    /* Call core() */
    core(res as cptr);
}
