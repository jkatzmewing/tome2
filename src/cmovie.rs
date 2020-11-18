use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut inkey_scan: bool_;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut Term: *mut term;
    #[no_mangle]
    static mut movfile: *mut FILE;
    #[no_mangle]
    static mut do_movies: libc::c_int;
    #[no_mangle]
    static mut last_paused: libc::c_int;
    #[no_mangle]
    fn Term_xtra(n: libc::c_int, v: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_putch(x: libc::c_int, y: libc::c_int, a: byte_hack,
                  c: libc::c_char) -> errr;
    #[no_mangle]
    fn Term_erase(x: libc::c_int, y: libc::c_int, n: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_clear() -> errr;
    #[no_mangle]
    fn Term_redraw_section(x1: libc::c_int, y1: libc::c_int, x2: libc::c_int,
                           y2: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_get_size(w: *mut libc::c_int, h: *mut libc::c_int) -> errr;
    #[no_mangle]
    fn Term_save() -> errr;
    #[no_mangle]
    fn Term_load() -> errr;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut panel_col_prt: s16b;
    #[no_mangle]
    static mut panel_row_prt: s16b;
    #[no_mangle]
    static mut player_base: [libc::c_char; 32];
    #[no_mangle]
    static mut ANGBAND_DIR_CMOV: cptr;
    #[no_mangle]
    fn get_version_string() -> *const libc::c_char;
    #[no_mangle]
    fn map_info_default(y: libc::c_int, x: libc::c_int, ap: *mut byte_hack,
                        cp: *mut libc::c_char);
    #[no_mangle]
    fn cmovie_init_second();
    #[no_mangle]
    fn my_fclose(fff: *mut FILE) -> errr;
    #[no_mangle]
    fn color_char_to_attr(c: libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn do_cmd_html_dump();
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn c_put_str(attr: byte_hack, str: cptr, row: libc::c_int,
                 col: libc::c_int);
    #[no_mangle]
    fn my_fgets(fff: *mut FILE, buf: *mut libc::c_char, n: huge_hack) -> errr;
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn my_fopen(file: cptr, mode: cptr) -> *mut FILE;
    #[no_mangle]
    fn path_build(buf: *mut libc::c_char, max: libc::c_int, path: cptr,
                  file: cptr) -> errr;
    #[no_mangle]
    static mut conv_color: [byte_hack; 16];
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn get_check(prompt: cptr) -> bool_;
    #[no_mangle]
    fn fd_close(fd: libc::c_int) -> errr;
    #[no_mangle]
    fn fd_open(file: cptr, flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn get_string(prompt: cptr, buf: *mut libc::c_char, len: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
pub type huge_hack = libc::c_ulong;
pub type s16b = libc::c_short;
pub type u16b = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct term_win {
    pub cu: bool_,
    pub cv: bool_,
    pub cx: byte_hack,
    pub cy: byte_hack,
    pub a: *mut *mut byte_hack,
    pub c: *mut *mut libc::c_char,
    pub va: *mut byte_hack,
    pub vc: *mut libc::c_char,
    pub ta: *mut *mut byte_hack,
    pub tc: *mut *mut libc::c_char,
    pub vta: *mut byte_hack,
    pub vtc: *mut libc::c_char,
    pub ea: *mut *mut byte_hack,
    pub ec: *mut *mut libc::c_char,
    pub vea: *mut byte_hack,
    pub vec: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct term {
    pub user: vptr,
    pub data: vptr,
    pub user_flag: bool_,
    pub data_flag: bool_,
    pub active_flag: bool_,
    pub mapped_flag: bool_,
    pub total_erase: bool_,
    pub fixed_shape: bool_,
    pub icky_corner: bool_,
    pub soft_cursor: bool_,
    pub always_pict: bool_,
    pub higher_pict: bool_,
    pub always_text: bool_,
    pub unused_flag: bool_,
    pub never_bored: bool_,
    pub never_frosh: bool_,
    pub attr_blank: byte_hack,
    pub char_blank: libc::c_char,
    pub key_queue: *mut libc::c_char,
    pub key_head: u16b,
    pub key_tail: u16b,
    pub key_xtra: u16b,
    pub key_size: u16b,
    pub wid: byte_hack,
    pub hgt: byte_hack,
    pub y1: byte_hack,
    pub y2: byte_hack,
    pub x1: *mut byte_hack,
    pub x2: *mut byte_hack,
    pub old: *mut term_win,
    pub scr: *mut term_win,
    pub tmp: *mut term_win,
    pub mem: *mut term_win,
    pub init_hook: Option<unsafe extern "C" fn(_: *mut term) -> ()>,
    pub nuke_hook: Option<unsafe extern "C" fn(_: *mut term) -> ()>,
    pub user_hook: Option<unsafe extern "C" fn(_: libc::c_int) -> errr>,
    pub xtra_hook: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int)
                              -> errr>,
    pub curs_hook: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int)
                              -> errr>,
    pub wipe_hook: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                               _: libc::c_int) -> errr>,
    pub text_hook: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                               _: libc::c_int, _: byte_hack,
                                               _: cptr) -> errr>,
    pub resize_hook: Option<unsafe extern "C" fn() -> ()>,
    pub pict_hook: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                               _: libc::c_int,
                                               _: *const byte_hack,
                                               _: *const libc::c_char,
                                               _: *const byte_hack,
                                               _: *const libc::c_char,
                                               _: *const byte_hack,
                                               _: *const libc::c_char)
                              -> errr>,
}
/* File: cmovie.c */
/* Purpose: play cmovie files -DarkGod-Improv- */
/*
 * Play a given cmovie
 */
#[no_mangle]
pub unsafe extern "C" fn do_play_cmovie(mut cmov_file: cptr) -> s16b {
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut y: libc::c_int = 0;
    let mut line: libc::c_int = 0 as libc::c_int;
    let mut x: libc::c_int = 0;
    let mut delay: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut cbuf: [libc::c_char; 90] = [0; 90];
    let mut ch: libc::c_char = 0;
    let mut mode: libc::c_char = 0 as libc::c_int as libc::c_char;
    /* Cmovie files are moved to the user directory on the multiuser systems */
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_CMOV,
               cmov_file);
    /* File type is "TEXT" */
    /* Read the file */
    fff =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Failure */
    if fff.is_null() { return -(1 as libc::c_int) as s16b }
    /* Save screen */
    character_icky = 1 as libc::c_int as bool_;
    Term_save();
    Term_clear();
    /* Give some usefull info */
    prt(b"While viewing the movie you can press Escape to exit, t/Space to switch between\x00"
            as *const u8 as *const libc::c_char, 0 as libc::c_int,
        0 as libc::c_int);
    prt(b"fluid more and step by step mode and any other key to step a frame in step by\x00"
            as *const u8 as *const libc::c_char, 1 as libc::c_int,
        0 as libc::c_int);
    prt(b"step mode.\x00" as *const u8 as *const libc::c_char,
        2 as libc::c_int, 0 as libc::c_int);
    prt(b"You can press D to do an html screenshot of the current frame.\x00"
            as *const u8 as *const libc::c_char, 3 as libc::c_int,
        0 as libc::c_int);
    prt(b"You can also use + and - to speed up/down the playing speed.\x00" as
            *const u8 as *const libc::c_char, 5 as libc::c_int,
        0 as libc::c_int);
    prt(b"Press any key when ready.\x00" as *const u8 as *const libc::c_char,
        8 as libc::c_int, 0 as libc::c_int);
    inkey();
    Term_clear();
    line = -(1 as libc::c_int);
    delay = 1 as libc::c_int;
    /* Init to white */
    x = 0 as libc::c_int;
    while x < 80 as libc::c_int {
        cbuf[x as usize] = 'w' as i32 as libc::c_char;
        x += 1
    }
    /* Parse */
    while 0 as libc::c_int ==
              my_fgets(fff, buf.as_mut_ptr(),
                       1024 as libc::c_int as huge_hack) {
        /* Do not wait */
        inkey_scan = 1 as libc::c_int as bool_;
        ch = inkey();
        /* Stop */
        if ch as libc::c_int == '\u{1b}' as i32 { break ; }
        /* Change mode */
        if ch as libc::c_int == 't' as i32 {
            mode = 0 as libc::c_int as libc::c_char
        } else if ch as libc::c_int == ' ' as i32 {
            mode = 1 as libc::c_int as libc::c_char
        } else if ch as libc::c_int == '+' as i32 {
            delay -= 1;
            if delay < 0 as libc::c_int { delay = 0 as libc::c_int }
        } else if ch as libc::c_int == '-' as i32 {
            delay += 1;
            if delay > 5 as libc::c_int { delay = 5 as libc::c_int }
        } else if ch as libc::c_int == 'D' as i32 { do_cmd_html_dump(); }
        line += 1;
        /* Change speed */
        /* Skip comments and blank lines */
        if buf[0 as libc::c_int as usize] == 0 ||
               buf[0 as libc::c_int as usize] as libc::c_int == '#' as i32 {
            continue ;
        }
        /* Verify correct "colon" format */
        if buf[1 as libc::c_int as usize] as libc::c_int != ':' as i32 {
            break ;
        }
        /* Clean screen */
        if buf[0 as libc::c_int as usize] as libc::c_int == 'C' as i32 {
            Term_clear();
        } else if buf[0 as libc::c_int as usize] as libc::c_int == 'B' as i32
         {
            let mut len: libc::c_int =
                strlen(buf.as_mut_ptr().offset(2 as libc::c_int as isize)) as
                    libc::c_int;
            /* Displays a textbox */
            /* Clear the line */
            Term_erase(0 as libc::c_int, 0 as libc::c_int,
                       255 as libc::c_int);
            /* Display the message */
            c_put_str(10 as libc::c_int as byte_hack,
                      b"###\x00" as *const u8 as *const libc::c_char,
                      0 as libc::c_int, 0 as libc::c_int);
            c_put_str(3 as libc::c_int as byte_hack,
                      buf.as_mut_ptr().offset(2 as libc::c_int as isize) as
                          cptr, 0 as libc::c_int, 3 as libc::c_int);
            c_put_str(10 as libc::c_int as byte_hack,
                      b"###\x00" as *const u8 as *const libc::c_char,
                      0 as libc::c_int, 3 as libc::c_int + len);
            c_put_str(1 as libc::c_int as byte_hack,
                      b"(more)\x00" as *const u8 as *const libc::c_char,
                      0 as libc::c_int, 6 as libc::c_int + len);
        } else if buf[0 as libc::c_int as usize] as libc::c_int == 'W' as i32
         {
            inkey();
        } else if buf[0 as libc::c_int as usize] as libc::c_int == 'S' as i32
         {
            let mut msec: libc::c_long = 0;
            /* Wait a key */
            /* Sleep */
            /* Scan for the values */
            if 1 as libc::c_int !=
                   sscanf(buf.as_mut_ptr().offset(2 as libc::c_int as isize),
                          b"%ld:\x00" as *const u8 as *const libc::c_char,
                          &mut msec as *mut libc::c_long) {
                return -(2 as libc::c_int) as s16b
            }
            if mode == 0 {
                Term_xtra(13 as libc::c_int, msec as libc::c_int);
            } else {
                let mut stop: bool_ = 0 as libc::c_int as bool_;
                loop  {
                    ch = inkey();
                    /* Stop */
                    if ch as libc::c_int == '\u{1b}' as i32 {
                        stop = 1 as libc::c_int as bool_;
                        break ;
                    } else if ch as libc::c_int == 't' as i32 {
                        mode = 0 as libc::c_int as libc::c_char;
                        break ;
                    } else if ch as libc::c_int == ' ' as i32 {
                        if mode != 0 { continue ; }
                        mode = 1 as libc::c_int as libc::c_char;
                        break ;
                    } else if ch as libc::c_int == '+' as i32 {
                        delay -= 1;
                        if delay < 0 as libc::c_int {
                            delay = 0 as libc::c_int
                        }
                    } else if ch as libc::c_int == '-' as i32 {
                        delay += 1;
                        if delay > 5 as libc::c_int {
                            delay = 5 as libc::c_int
                        }
                    } else {
                        if !(ch as libc::c_int == 'D' as i32) { break ; }
                        do_cmd_html_dump();
                    }
                }
                if stop != 0 { break ; }
            }
        } else if buf[0 as libc::c_int as usize] as libc::c_int == 'E' as i32
         {
            /* Change mode */
            /* Change mode */
            /* Change speed */
            /* Get color for the NEXT L line */
            /* Find the colon before the name */
            s =
                strchr(buf.as_mut_ptr().offset(2 as libc::c_int as isize),
                       ':' as i32);
            /* Verify that colon */
            if s.is_null() { return -(2 as libc::c_int) as s16b }
            /* Nuke the colon, advance to the name */
            let fresh0 = s;
            s = s.offset(1);
            *fresh0 = '\u{0}' as i32 as libc::c_char;
            /* Paranoia -- require a name */
            if *s == 0 { return -(2 as libc::c_int) as s16b }
            /* Get the index */
            y = atoi(buf.as_mut_ptr().offset(2 as libc::c_int as isize));
            memcpy(cbuf.as_mut_ptr() as *mut libc::c_void,
                   s as *const libc::c_void,
                   (80 as libc::c_int as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                        as libc::c_ulong));
        } else if buf[0 as libc::c_int as usize] as libc::c_int == 'L' as i32
         {
            /* Print a line */
            /* Find the colon before the name */
            s =
                strchr(buf.as_mut_ptr().offset(2 as libc::c_int as isize),
                       ':' as i32);
            /* Verify that colon */
            if s.is_null() { return -(2 as libc::c_int) as s16b }
            /* Nuke the colon, advance to the name */
            let fresh1 = s;
            s = s.offset(1);
            *fresh1 = '\u{0}' as i32 as libc::c_char;
            /* Paranoia -- require a name */
            if *s == 0 { return -(2 as libc::c_int) as s16b }
            /* Get the index */
            y = atoi(buf.as_mut_ptr().offset(2 as libc::c_int as isize));
            x = 0 as libc::c_int;
            while x < 80 as libc::c_int {
                Term_putch(x, y,
                           color_char_to_attr(cbuf[x as usize]) as byte_hack,
                           *s.offset(x as isize));
                /* Reinit to white */
                cbuf[x as usize] = 'w' as i32 as libc::c_char;
                x += 1
            }
            Term_redraw_section(0 as libc::c_int, y, 79 as libc::c_int, y);
        } else {
            /* Update 1 char */
            if !(buf[0 as libc::c_int as usize] as libc::c_int == 'P' as i32)
               {
                continue ;
            }
            let mut x_0: libc::c_int = 0;
            let mut y_0: libc::c_int = 0;
            let mut a: libc::c_int = 0;
            let mut c: libc::c_int = 0;
            /* Scan for the values */
            if 4 as libc::c_int !=
                   sscanf(buf.as_mut_ptr().offset(2 as libc::c_int as isize),
                          b"%d:%d:%d:%d\x00" as *const u8 as
                              *const libc::c_char,
                          &mut x_0 as *mut libc::c_int,
                          &mut y_0 as *mut libc::c_int,
                          &mut c as *mut libc::c_int,
                          &mut a as *mut libc::c_int) {
                a = 'w' as i32;
                if 3 as libc::c_int !=
                       sscanf(buf.as_mut_ptr().offset(2 as libc::c_int as
                                                          isize),
                              b"%d:%d:%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut x_0 as *mut libc::c_int,
                              &mut y_0 as *mut libc::c_int,
                              &mut c as *mut libc::c_int) {
                    return -(2 as libc::c_int) as s16b
                }
            }
            Term_putch(x_0, y_0,
                       color_char_to_attr(cbuf[x_0 as usize]) as byte_hack,
                       c as libc::c_char);
            Term_redraw_section(x_0, y_0, x_0 + 1 as libc::c_int,
                                y_0 + 1 as libc::c_int);
        }
    }
    /* Load screen */
    Term_load();
    character_icky = 0 as libc::c_int as bool_;
    /* Close */
    my_fclose(fff);
    return 0 as libc::c_int as s16b;
}
/*
 * Start the recording of a cmovie
 */
#[no_mangle]
pub unsafe extern "C" fn do_record_cmovie(mut cmovie: cptr) {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut y: libc::c_int = 0;
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_CMOV,
               cmovie);
    /* File type is "TEXT" */
    /* Check for existing file */
    fd = fd_open(buf.as_mut_ptr() as cptr, 0 as libc::c_int);
    /* Existing file */
    if fd >= 0 as libc::c_int {
        let mut out_val: [libc::c_char; 160] = [0; 160];
        /* Close the file */
        fd_close(fd);
        /* Build query */
        sprintf(out_val.as_mut_ptr(),
                b"Replace existing file %s? \x00" as *const u8 as
                    *const libc::c_char, cmovie);
        /* Ask */
        if get_check(out_val.as_mut_ptr() as cptr) != 0 {
            fd = -(1 as libc::c_int)
        }
    }
    /* Be sure */
    if get_check(b"Ready to record(Press ctrl+D to enter a textual note while recording)?\x00"
                     as *const u8 as *const libc::c_char) == 0 {
        return
    }
    /* Open the non-existing file */
    if fd < 0 as libc::c_int {
        movfile =
            my_fopen(buf.as_mut_ptr() as cptr,
                     b"w\x00" as *const u8 as *const libc::c_char)
    }
    /* Invalid file */
    if movfile.is_null() {
        msg_format(b"Cmovie recording failed!\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    /* First thing: Record clear screen then enable the recording */
    fprintf(movfile,
            b"# Generated by %s\n\x00" as *const u8 as *const libc::c_char,
            get_version_string());
    fprintf(movfile, b"C:\n\x00" as *const u8 as *const libc::c_char);
    last_paused = 0 as libc::c_int;
    do_movies = 1 as libc::c_int;
    cmovie_init_second();
    /* Mega Hack, get the screen */
    y = 0 as libc::c_int;
    while y < (*Term).hgt as libc::c_int { cmovie_record_line(y); y += 1 };
}
/*
 * Stop the recording
 */
#[no_mangle]
pub unsafe extern "C" fn do_stop_cmovie() {
    if do_movies == 1 as libc::c_int {
        do_movies = 0 as libc::c_int;
        my_fclose(movfile);
    };
}
/*
 * Start a cmovie
 */
#[no_mangle]
pub unsafe extern "C" fn do_start_cmovie() {
    let mut name: [libc::c_char; 90] = [0; 90];
    let mut rname: [libc::c_char; 94] = [0; 94];
    /* Should never happen */
    if do_movies == 1 as libc::c_int { return }
    /* Default */
    sprintf(name.as_mut_ptr(), b"%s\x00" as *const u8 as *const libc::c_char,
            player_base.as_mut_ptr());
    if get_string(b"Cmovie name: \x00" as *const u8 as *const libc::c_char,
                  name.as_mut_ptr(), 80 as libc::c_int) != 0 {
        if name[0 as libc::c_int as usize] as libc::c_int != 0 &&
               name[0 as libc::c_int as usize] as libc::c_int != ' ' as i32 {
            sprintf(rname.as_mut_ptr(),
                    b"%s.cmv\x00" as *const u8 as *const libc::c_char,
                    name.as_mut_ptr());
            if get_check(b"Record(y), Play(n)?\x00" as *const u8 as
                             *const libc::c_char) != 0 {
                do_record_cmovie(rname.as_mut_ptr() as cptr);
            } else { do_play_cmovie(rname.as_mut_ptr() as cptr); }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmovie_clean_line(mut y: libc::c_int,
                                           mut abuf: *mut libc::c_char,
                                           mut cbuf: *mut libc::c_char) {
    let mut ap: *const byte_hack = *(*(*Term).scr).a.offset(y as isize);
    let mut cp: *const libc::c_char = *(*(*Term).scr).c.offset(y as isize);
    let mut a: byte_hack = 0;
    let mut c: libc::c_char = 0;
    let mut x: libc::c_int = 0;
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    let mut screen_wid: libc::c_int = 0;
    let mut screen_hgt: libc::c_int = 0;
    /* Retrieve current screen size */
    Term_get_size(&mut wid, &mut hgt);
    /* Calculate the size of dungeon map area */
    screen_wid = wid - (13 as libc::c_int + 1 as libc::c_int);
    screen_hgt = hgt - (1 as libc::c_int + 1 as libc::c_int);
    /* For the time being, assume 80 column display XXX XXX XXX */
    x = 0 as libc::c_int;
    while x < wid {
        /* Convert dungeon map into default attr/chars */
        if character_icky == 0 && x - 13 as libc::c_int >= 0 as libc::c_int &&
               (x - 13 as libc::c_int) < screen_wid &&
               y - 1 as libc::c_int >= 0 as libc::c_int &&
               (y - 1 as libc::c_int) < screen_hgt {
            /* Retrieve default attr/char */
            map_info_default(y + panel_row_prt as libc::c_int,
                             x + panel_col_prt as libc::c_int, &mut a,
                             &mut c);
            *abuf.offset(x as isize) =
                conv_color[(a as libc::c_int & 0xf as libc::c_int) as usize]
                    as libc::c_char;
            if c as libc::c_int == '\u{0}' as i32 {
                *cbuf.offset(x as isize) = ' ' as i32 as libc::c_char
            } else { *cbuf.offset(x as isize) = c }
        } else {
            *abuf.offset(x as isize) =
                conv_color[(*ap.offset(x as isize) as libc::c_int &
                                0xf as libc::c_int) as usize] as libc::c_char;
            *cbuf.offset(x as isize) = *cp.offset(x as isize)
        }
        x += 1
    }
    /* Null-terminate the prepared strings */
    *abuf.offset(x as isize) = '\u{0}' as i32 as libc::c_char;
    *cbuf.offset(x as isize) = '\u{0}' as i32 as libc::c_char;
}
/*
 * Write a record of a screen row into a cmovie file
 */
#[no_mangle]
pub unsafe extern "C" fn cmovie_record_line(mut y: libc::c_int) {
    let mut abuf: [libc::c_char; 256] = [0; 256];
    let mut cbuf: [libc::c_char; 256] = [0; 256];
    cmovie_clean_line(y, abuf.as_mut_ptr(), cbuf.as_mut_ptr());
    /* Write a colour record */
    fprintf(movfile, b"E:%d:%.80s\n\x00" as *const u8 as *const libc::c_char,
            y, abuf.as_mut_ptr());
    /* Write a char record */
    fprintf(movfile, b"L:%d:%.80s\n\x00" as *const u8 as *const libc::c_char,
            y, cbuf.as_mut_ptr());
}
/*
 * Record a "text box"
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmovie_insert() {
    let mut buf: [libc::c_char; 81] =
        *::std::mem::transmute::<&[u8; 81],
                                 &mut [libc::c_char; 81]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    /* Dont record */
    do_movies = 2 as libc::c_int;
    while get_string(b"Textbox(ESC to end): \x00" as *const u8 as
                         *const libc::c_char, buf.as_mut_ptr(),
                     80 as libc::c_int) != 0 {
        fprintf(movfile,
                b"B:%s\nW:\n\x00" as *const u8 as *const libc::c_char,
                buf.as_mut_ptr());
        buf[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char
    }
    /* We reinit the time as to not count the time the user needed ot enter the text */
    cmovie_init_second();
    /* Continue recording */
    do_movies = 1 as libc::c_int;
}
