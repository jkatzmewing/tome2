use ::libc;
extern "C" {
    #[no_mangle]
    static mut arg_fiddle: bool_;
    #[no_mangle]
    static mut arg_wizard: bool_;
    #[no_mangle]
    static mut arg_sound: bool_;
    #[no_mangle]
    static mut arg_graphics: bool_;
    #[no_mangle]
    static mut arg_force_original: bool_;
    #[no_mangle]
    static mut arg_force_roguelike: bool_;
    #[no_mangle]
    fn quit_fmt(fmt: cptr, _: ...);
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn getuid() -> __uid_t;
    #[no_mangle]
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn umask(__mask: __mode_t) -> __mode_t;
    #[no_mangle]
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    #[no_mangle]
    static mut argv0: cptr;
    #[no_mangle]
    static mut quit_aux: Option<unsafe extern "C" fn(_: cptr) -> ()>;
    #[no_mangle]
    fn streq(s: cptr, t: cptr) -> bool_;
    #[no_mangle]
    fn suffix(s: cptr, t: cptr) -> bool_;
    #[no_mangle]
    fn quit(str: cptr);
    #[no_mangle]
    fn string_make(str: cptr) -> cptr;
    #[no_mangle]
    fn string_free(str: cptr) -> errr;
    #[no_mangle]
    fn term_nuke(t: *mut term) -> errr;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut player_uid: libc::c_int;
    #[no_mangle]
    static mut player_name: [libc::c_char; 32];
    #[no_mangle]
    static mut player_base: [libc::c_char; 32];
    #[no_mangle]
    static mut angband_term: [*mut term; 8];
    #[no_mangle]
    static mut ANGBAND_SYS: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_APEX: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_DATA: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_EDIT: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_FILE: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_HELP: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_INFO: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_SAVE: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_USER: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_XTRA: cptr;
    #[no_mangle]
    static mut no_begin_screen: bool_;
    #[no_mangle]
    fn play_game(new_game: bool_);
    #[no_mangle]
    fn txt_to_html(head: cptr, food: cptr, base: cptr, ext: cptr,
                   force: bool_, recur: bool_) -> bool_;
    #[no_mangle]
    fn process_player_name(sf: bool_);
    #[no_mangle]
    fn display_scores(from: libc::c_int, to: libc::c_int);
    #[no_mangle]
    fn signals_init();
    #[no_mangle]
    fn init_file_paths(path: *mut libc::c_char);
    #[no_mangle]
    fn init_angband();
    #[no_mangle]
    fn path_parse(buf: *mut libc::c_char, max: libc::c_int, file: cptr)
     -> errr;
    #[no_mangle]
    fn pause_line(row: libc::c_int);
    #[no_mangle]
    fn user_name(buf: *mut libc::c_char, id: libc::c_int);
    #[no_mangle]
    fn init_lua();
    #[no_mangle]
    static mut force_module: cptr;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type vptr = *mut libc::c_void;
pub type cptr = *const libc::c_char;
pub type errr = libc::c_int;
pub type byte_hack = libc::c_uchar;
pub type bool_ = libc::c_char;
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
/* File: main.c */
/*
 * Copyright (c) 1997 Ben Harrison, and others
 *
 * This software may be copied and distributed for educational, research,
 * and not for profit purposes provided that this copyright and statement
 * are included in all such copies.
 */
/*
 * Some machines have a "main()" function in their "main-xxx.c" file,
 * all the others use this file for their "main()" function.
 */
/*
 * A hook for "quit()".
 *
 * Close down, then fall back into "quit()".
 */
unsafe extern "C" fn quit_hook(mut s: cptr) {
    let mut j: libc::c_int = 0;
    /* Scan windows */
    j = 8 as libc::c_int - 1 as libc::c_int;
    while j >= 0 as libc::c_int {
        /* Unused */
        if !angband_term[j as usize].is_null() {
            /* Nuke it */
            term_nuke(angband_term[j as usize]);
        }
        j -= 1
    };
}
/*
 * Check and create if needed the directory dirpath
 */
#[no_mangle]
pub unsafe extern "C" fn private_check_user_directory(mut dirpath: cptr)
 -> bool_ {
    /* Is this used anywhere else in *bands? */
    let mut stat_buf: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut ret: libc::c_int = 0;
    /* See if it already exists */
    ret = stat(dirpath, &mut stat_buf);
    /* It does */
    if ret == 0 as libc::c_int {
        /* Now we see if it's a directory */
        if stat_buf.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
               0o40000 as libc::c_int as libc::c_uint {
            return 1 as libc::c_int as bool_
        }
        /*
		 * Something prevents us from create a directory with
		 * the same pathname
		 */
        return 0 as libc::c_int as bool_
    } else {
        /* No - this maybe the first time. Try to create a directory */
        /* Create the ~/.ToME directory */
        ret = mkdir(dirpath, 0o700 as libc::c_int as __mode_t);
        if ret == -(1 as libc::c_int) { return 0 as libc::c_int as bool_ }
        return 1 as libc::c_int as bool_
    };
}
/* An error occured */
/* Success */
/*
 * Check existence of ".ToME/" directory in the user's
 * home directory or try to create it if it doesn't exist.
 * Returns FALSE if all the attempts fail.
 */
unsafe extern "C" fn check_create_user_dir() -> bool_ {
    let mut dirpath: [libc::c_char; 1024] = [0; 1024];
    let mut versionpath: [libc::c_char; 1024] = [0; 1024];
    let mut savepath: [libc::c_char; 1024] = [0; 1024];
    /* Get an absolute path from the filename */
    path_parse(dirpath.as_mut_ptr(), 1024 as libc::c_int,
               b"~/.tome\x00" as *const u8 as *const libc::c_char);
    strcpy(versionpath.as_mut_ptr(), dirpath.as_mut_ptr());
    strcat(versionpath.as_mut_ptr(),
           b"/2.3\x00" as *const u8 as *const libc::c_char);
    strcpy(savepath.as_mut_ptr(), versionpath.as_mut_ptr());
    strcat(savepath.as_mut_ptr(),
           b"/save\x00" as *const u8 as *const libc::c_char);
    return (private_check_user_directory(dirpath.as_mut_ptr() as cptr) as
                libc::c_int != 0 &&
                private_check_user_directory(versionpath.as_mut_ptr() as cptr)
                    as libc::c_int != 0 &&
                private_check_user_directory(savepath.as_mut_ptr() as cptr) as
                    libc::c_int != 0) as libc::c_int as bool_;
}
/*
 * Initialize and verify the file paths, and the score file.
 *
 * Use the ANGBAND_PATH environment var if possible, else use
 * DEFAULT_PATH, and in either case, branch off appropriately.
 *
 * First, we'll look for the ANGBAND_PATH environment variable,
 * and then look for the files in there.  If that doesn't work,
 * we'll try the DEFAULT_PATH constant.  So be sure that one of
 * these two things works...
 *
 * We must ensure that the path ends with "PATH_SEP" if needed,
 * since the "init_file_paths()" function will simply append the
 * relevant "sub-directory names" to the given path.
 */
unsafe extern "C" fn init_stuff() {
    let mut path: [libc::c_char; 1024] = [0; 1024];
    let mut tail: cptr = 0 as *const libc::c_char;
    /* Get the environment variable */
    tail =
        getenv(b"TOME_PATH\x00" as *const u8 as *const libc::c_char) as cptr;
    /* Use the angband_path, or a default */
    strcpy(path.as_mut_ptr(),
           if !tail.is_null() {
               tail
           } else { b"./lib\x00" as *const u8 as *const libc::c_char });
    /* Hack -- Add a path separator (only if needed) */
    if suffix(path.as_mut_ptr() as cptr,
              b"/\x00" as *const u8 as *const libc::c_char) == 0 {
        strcat(path.as_mut_ptr(),
               b"/\x00" as *const u8 as *const libc::c_char);
    }
    /* Initialize */
    init_file_paths(path.as_mut_ptr());
}
/*
 * Handle a "-d<what>=<path>" option
 *
 * The "<what>" can be any string starting with the same letter as the
 * name of a subdirectory of the "lib" folder (i.e. "i" or "info").
 *
 * The "<path>" can be any legal path for the given system, and should
 * not end in any special path separator (i.e. "/tmp" or "~/.ang-info").
 */
unsafe extern "C" fn change_path(mut info: cptr) {
    let mut s: cptr = 0 as *const libc::c_char;
    /* Find equal sign */
    s = strchr(info, '=' as i32) as cptr;
    /* Verify equal sign */
    if s.is_null() {
        quit_fmt(b"Try \'-d<what>=<path>\' not \'-d%s\'\x00" as *const u8 as
                     *const libc::c_char, info);
    }
    /* Analyze */
    match tolower(*info.offset(0 as libc::c_int as isize) as libc::c_int) {
        97 => {
            string_free(ANGBAND_DIR_APEX);
            ANGBAND_DIR_APEX =
                string_make(s.offset(1 as libc::c_int as isize))
        }
        102 => {
            string_free(ANGBAND_DIR_FILE);
            ANGBAND_DIR_FILE =
                string_make(s.offset(1 as libc::c_int as isize))
        }
        104 => {
            string_free(ANGBAND_DIR_HELP);
            ANGBAND_DIR_HELP =
                string_make(s.offset(1 as libc::c_int as isize))
        }
        105 => {
            string_free(ANGBAND_DIR_INFO);
            ANGBAND_DIR_INFO =
                string_make(s.offset(1 as libc::c_int as isize))
        }
        117 => {
            string_free(ANGBAND_DIR_USER);
            ANGBAND_DIR_USER =
                string_make(s.offset(1 as libc::c_int as isize))
        }
        120 => {
            string_free(ANGBAND_DIR_XTRA);
            ANGBAND_DIR_XTRA =
                string_make(s.offset(1 as libc::c_int as isize))
        }
        100 => {
            string_free(ANGBAND_DIR_DATA);
            ANGBAND_DIR_DATA =
                string_make(s.offset(1 as libc::c_int as isize))
        }
        101 => {
            string_free(ANGBAND_DIR_EDIT);
            ANGBAND_DIR_EDIT =
                string_make(s.offset(1 as libc::c_int as isize))
        }
        115 => {
            string_free(ANGBAND_DIR_SAVE);
            ANGBAND_DIR_SAVE =
                string_make(s.offset(1 as libc::c_int as isize))
        }
        _ => {
            quit_fmt(b"Bad semantics in \'-d%s\'\x00" as *const u8 as
                         *const libc::c_char, info);
        }
    };
}
/*
 * Simple "main" function for multiple platforms.
 *
 * Note the special "--" option which terminates the processing of
 * standard options.  All non-standard options (if any) are passed
 * directly to the "init_xxx()" function.
 */
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut done: bool_ = 0 as libc::c_int as bool_;
    let mut new_game: bool_ = 0 as libc::c_int as bool_;
    let mut show_score: libc::c_int = 0 as libc::c_int;
    let mut mstr: cptr = 0 as cptr;
    let mut args: bool_ = 1 as libc::c_int as bool_;
    /* CHECK_MEMORY_LEAKS */
    /* Save the "program name" XXX XXX XXX */
    argv0 = *argv.offset(0 as libc::c_int as isize) as cptr;
    /* Default permissions on files */
    umask(0o22 as libc::c_int as __mode_t);
    /* Get the file paths */
    init_stuff();
    /* Get the user id (?) */
    player_uid = getuid() as libc::c_int;
    /* Acquire the "user name" as a default player name */
    user_name(player_name.as_mut_ptr(), player_uid);
    /*
	 * On multiuser systems, users' private directories are
	 * used to store pref files, chardumps etc.
	 */
    let mut ret: bool_ = 0;
    /* Create a directory for the user's files */
    ret = check_create_user_dir();
    /* Oops */
    if ret as libc::c_int == 0 as libc::c_int {
        quit(b"Cannot create directory ~/.tome\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Process the command line arguments */
    i = 1 as libc::c_int;
    while args as libc::c_int != 0 && i < argc {
        /* Require proper options */
        if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as
               libc::c_int != '-' as i32 {
            current_block = 3934757609181694121;
        } else {
            /* Analyze option */
            match *(*argv.offset(i as
                                     isize)).offset(1 as libc::c_int as isize)
                      as libc::c_int {
                78 | 110 => {
                    current_block = 1054647088692577877;
                    match current_block {
                        6545907279487748450 => {
                            change_path(&mut *(*argv.offset(i as
                                                                isize)).offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                            as *mut libc::c_char as cptr);
                            current_block = 1856101646708284338;
                        }
                        790185930182612747 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                force_module =
                                    string_make(&mut *(*argv.offset(i as
                                                                        isize)).offset(2
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize)
                                                    as *mut libc::c_char as
                                                    cptr);
                                current_block = 1856101646708284338;
                            }
                        }
                        7226443171521532240 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                mstr =
                                    &mut *(*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                        as *mut libc::c_char as cptr;
                                current_block = 1856101646708284338;
                            }
                        }
                        7245201122033322888 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                strcpy(player_name.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                strcpy(player_base.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                no_begin_screen = 1 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                        17281240262373992796 => {
                            show_score =
                                atoi(&mut *(*argv.offset(i as
                                                             isize)).offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize));
                            if show_score <= 0 as libc::c_int {
                                show_score = 10 as libc::c_int
                            }
                            current_block = 1856101646708284338;
                        }
                        1054647088692577877 => {
                            new_game = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        5689001924483802034 => {
                            arg_fiddle = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        26972500619410423 => {
                            arg_wizard = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        14576567515993809846 => {
                            arg_sound = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        2719512138335094285 => {
                            arg_graphics = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        15125582407903384992 => {
                            arg_force_roguelike = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        4775909272756257391 => {
                            arg_force_original = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        9423205874568575888 => {
                            let mut s: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut j: libc::c_int = 0;
                            init_lua();
                            j = i + 1 as libc::c_int;
                            while j < argc {
                                s = *argv.offset(j as isize);
                                while *s as libc::c_int != '.' as i32 {
                                    s = s.offset(1)
                                }
                                *s = '\u{0}' as i32 as libc::c_char;
                                s = s.offset(1);
                                txt_to_html(b"head.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"foot.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            *argv.offset(j as isize) as cptr,
                                            s as cptr,
                                            0 as libc::c_int as bool_,
                                            0 as libc::c_int as bool_);
                                j += 1
                            }
                            return 0 as libc::c_int
                        }
                        _ => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   as libc::c_int == 'h' as i32 &&
                                   strcmp((*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                          b"help\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                let ref mut fresh0 = *argv.offset(i as isize);
                                *fresh0 =
                                    *argv.offset(0 as libc::c_int as isize);
                                argc = argc - i;
                                argv = argv.offset(i as isize);
                                args = 0 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                    }
                }
                70 | 102 => {
                    current_block = 5689001924483802034;
                    match current_block {
                        6545907279487748450 => {
                            change_path(&mut *(*argv.offset(i as
                                                                isize)).offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                            as *mut libc::c_char as cptr);
                            current_block = 1856101646708284338;
                        }
                        790185930182612747 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                force_module =
                                    string_make(&mut *(*argv.offset(i as
                                                                        isize)).offset(2
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize)
                                                    as *mut libc::c_char as
                                                    cptr);
                                current_block = 1856101646708284338;
                            }
                        }
                        7226443171521532240 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                mstr =
                                    &mut *(*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                        as *mut libc::c_char as cptr;
                                current_block = 1856101646708284338;
                            }
                        }
                        7245201122033322888 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                strcpy(player_name.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                strcpy(player_base.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                no_begin_screen = 1 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                        17281240262373992796 => {
                            show_score =
                                atoi(&mut *(*argv.offset(i as
                                                             isize)).offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize));
                            if show_score <= 0 as libc::c_int {
                                show_score = 10 as libc::c_int
                            }
                            current_block = 1856101646708284338;
                        }
                        1054647088692577877 => {
                            new_game = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        5689001924483802034 => {
                            arg_fiddle = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        26972500619410423 => {
                            arg_wizard = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        14576567515993809846 => {
                            arg_sound = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        2719512138335094285 => {
                            arg_graphics = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        15125582407903384992 => {
                            arg_force_roguelike = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        4775909272756257391 => {
                            arg_force_original = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        9423205874568575888 => {
                            let mut s: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut j: libc::c_int = 0;
                            init_lua();
                            j = i + 1 as libc::c_int;
                            while j < argc {
                                s = *argv.offset(j as isize);
                                while *s as libc::c_int != '.' as i32 {
                                    s = s.offset(1)
                                }
                                *s = '\u{0}' as i32 as libc::c_char;
                                s = s.offset(1);
                                txt_to_html(b"head.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"foot.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            *argv.offset(j as isize) as cptr,
                                            s as cptr,
                                            0 as libc::c_int as bool_,
                                            0 as libc::c_int as bool_);
                                j += 1
                            }
                            return 0 as libc::c_int
                        }
                        _ => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   as libc::c_int == 'h' as i32 &&
                                   strcmp((*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                          b"help\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                let ref mut fresh0 = *argv.offset(i as isize);
                                *fresh0 =
                                    *argv.offset(0 as libc::c_int as isize);
                                argc = argc - i;
                                argv = argv.offset(i as isize);
                                args = 0 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                    }
                }
                87 | 119 => {
                    current_block = 26972500619410423;
                    match current_block {
                        6545907279487748450 => {
                            change_path(&mut *(*argv.offset(i as
                                                                isize)).offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                            as *mut libc::c_char as cptr);
                            current_block = 1856101646708284338;
                        }
                        790185930182612747 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                force_module =
                                    string_make(&mut *(*argv.offset(i as
                                                                        isize)).offset(2
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize)
                                                    as *mut libc::c_char as
                                                    cptr);
                                current_block = 1856101646708284338;
                            }
                        }
                        7226443171521532240 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                mstr =
                                    &mut *(*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                        as *mut libc::c_char as cptr;
                                current_block = 1856101646708284338;
                            }
                        }
                        7245201122033322888 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                strcpy(player_name.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                strcpy(player_base.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                no_begin_screen = 1 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                        17281240262373992796 => {
                            show_score =
                                atoi(&mut *(*argv.offset(i as
                                                             isize)).offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize));
                            if show_score <= 0 as libc::c_int {
                                show_score = 10 as libc::c_int
                            }
                            current_block = 1856101646708284338;
                        }
                        1054647088692577877 => {
                            new_game = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        5689001924483802034 => {
                            arg_fiddle = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        26972500619410423 => {
                            arg_wizard = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        14576567515993809846 => {
                            arg_sound = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        2719512138335094285 => {
                            arg_graphics = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        15125582407903384992 => {
                            arg_force_roguelike = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        4775909272756257391 => {
                            arg_force_original = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        9423205874568575888 => {
                            let mut s: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut j: libc::c_int = 0;
                            init_lua();
                            j = i + 1 as libc::c_int;
                            while j < argc {
                                s = *argv.offset(j as isize);
                                while *s as libc::c_int != '.' as i32 {
                                    s = s.offset(1)
                                }
                                *s = '\u{0}' as i32 as libc::c_char;
                                s = s.offset(1);
                                txt_to_html(b"head.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"foot.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            *argv.offset(j as isize) as cptr,
                                            s as cptr,
                                            0 as libc::c_int as bool_,
                                            0 as libc::c_int as bool_);
                                j += 1
                            }
                            return 0 as libc::c_int
                        }
                        _ => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   as libc::c_int == 'h' as i32 &&
                                   strcmp((*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                          b"help\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                let ref mut fresh0 = *argv.offset(i as isize);
                                *fresh0 =
                                    *argv.offset(0 as libc::c_int as isize);
                                argc = argc - i;
                                argv = argv.offset(i as isize);
                                args = 0 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                    }
                }
                86 | 118 => {
                    current_block = 14576567515993809846;
                    match current_block {
                        6545907279487748450 => {
                            change_path(&mut *(*argv.offset(i as
                                                                isize)).offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                            as *mut libc::c_char as cptr);
                            current_block = 1856101646708284338;
                        }
                        790185930182612747 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                force_module =
                                    string_make(&mut *(*argv.offset(i as
                                                                        isize)).offset(2
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize)
                                                    as *mut libc::c_char as
                                                    cptr);
                                current_block = 1856101646708284338;
                            }
                        }
                        7226443171521532240 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                mstr =
                                    &mut *(*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                        as *mut libc::c_char as cptr;
                                current_block = 1856101646708284338;
                            }
                        }
                        7245201122033322888 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                strcpy(player_name.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                strcpy(player_base.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                no_begin_screen = 1 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                        17281240262373992796 => {
                            show_score =
                                atoi(&mut *(*argv.offset(i as
                                                             isize)).offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize));
                            if show_score <= 0 as libc::c_int {
                                show_score = 10 as libc::c_int
                            }
                            current_block = 1856101646708284338;
                        }
                        1054647088692577877 => {
                            new_game = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        5689001924483802034 => {
                            arg_fiddle = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        26972500619410423 => {
                            arg_wizard = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        14576567515993809846 => {
                            arg_sound = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        2719512138335094285 => {
                            arg_graphics = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        15125582407903384992 => {
                            arg_force_roguelike = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        4775909272756257391 => {
                            arg_force_original = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        9423205874568575888 => {
                            let mut s: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut j: libc::c_int = 0;
                            init_lua();
                            j = i + 1 as libc::c_int;
                            while j < argc {
                                s = *argv.offset(j as isize);
                                while *s as libc::c_int != '.' as i32 {
                                    s = s.offset(1)
                                }
                                *s = '\u{0}' as i32 as libc::c_char;
                                s = s.offset(1);
                                txt_to_html(b"head.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"foot.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            *argv.offset(j as isize) as cptr,
                                            s as cptr,
                                            0 as libc::c_int as bool_,
                                            0 as libc::c_int as bool_);
                                j += 1
                            }
                            return 0 as libc::c_int
                        }
                        _ => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   as libc::c_int == 'h' as i32 &&
                                   strcmp((*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                          b"help\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                let ref mut fresh0 = *argv.offset(i as isize);
                                *fresh0 =
                                    *argv.offset(0 as libc::c_int as isize);
                                argc = argc - i;
                                argv = argv.offset(i as isize);
                                args = 0 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                    }
                }
                71 | 103 => {
                    current_block = 2719512138335094285;
                    match current_block {
                        6545907279487748450 => {
                            change_path(&mut *(*argv.offset(i as
                                                                isize)).offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                            as *mut libc::c_char as cptr);
                            current_block = 1856101646708284338;
                        }
                        790185930182612747 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                force_module =
                                    string_make(&mut *(*argv.offset(i as
                                                                        isize)).offset(2
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize)
                                                    as *mut libc::c_char as
                                                    cptr);
                                current_block = 1856101646708284338;
                            }
                        }
                        7226443171521532240 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                mstr =
                                    &mut *(*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                        as *mut libc::c_char as cptr;
                                current_block = 1856101646708284338;
                            }
                        }
                        7245201122033322888 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                strcpy(player_name.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                strcpy(player_base.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                no_begin_screen = 1 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                        17281240262373992796 => {
                            show_score =
                                atoi(&mut *(*argv.offset(i as
                                                             isize)).offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize));
                            if show_score <= 0 as libc::c_int {
                                show_score = 10 as libc::c_int
                            }
                            current_block = 1856101646708284338;
                        }
                        1054647088692577877 => {
                            new_game = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        5689001924483802034 => {
                            arg_fiddle = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        26972500619410423 => {
                            arg_wizard = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        14576567515993809846 => {
                            arg_sound = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        2719512138335094285 => {
                            arg_graphics = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        15125582407903384992 => {
                            arg_force_roguelike = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        4775909272756257391 => {
                            arg_force_original = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        9423205874568575888 => {
                            let mut s: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut j: libc::c_int = 0;
                            init_lua();
                            j = i + 1 as libc::c_int;
                            while j < argc {
                                s = *argv.offset(j as isize);
                                while *s as libc::c_int != '.' as i32 {
                                    s = s.offset(1)
                                }
                                *s = '\u{0}' as i32 as libc::c_char;
                                s = s.offset(1);
                                txt_to_html(b"head.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"foot.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            *argv.offset(j as isize) as cptr,
                                            s as cptr,
                                            0 as libc::c_int as bool_,
                                            0 as libc::c_int as bool_);
                                j += 1
                            }
                            return 0 as libc::c_int
                        }
                        _ => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   as libc::c_int == 'h' as i32 &&
                                   strcmp((*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                          b"help\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                let ref mut fresh0 = *argv.offset(i as isize);
                                *fresh0 =
                                    *argv.offset(0 as libc::c_int as isize);
                                argc = argc - i;
                                argv = argv.offset(i as isize);
                                args = 0 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                    }
                }
                82 | 114 => {
                    current_block = 15125582407903384992;
                    match current_block {
                        6545907279487748450 => {
                            change_path(&mut *(*argv.offset(i as
                                                                isize)).offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                            as *mut libc::c_char as cptr);
                            current_block = 1856101646708284338;
                        }
                        790185930182612747 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                force_module =
                                    string_make(&mut *(*argv.offset(i as
                                                                        isize)).offset(2
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize)
                                                    as *mut libc::c_char as
                                                    cptr);
                                current_block = 1856101646708284338;
                            }
                        }
                        7226443171521532240 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                mstr =
                                    &mut *(*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                        as *mut libc::c_char as cptr;
                                current_block = 1856101646708284338;
                            }
                        }
                        7245201122033322888 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                strcpy(player_name.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                strcpy(player_base.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                no_begin_screen = 1 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                        17281240262373992796 => {
                            show_score =
                                atoi(&mut *(*argv.offset(i as
                                                             isize)).offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize));
                            if show_score <= 0 as libc::c_int {
                                show_score = 10 as libc::c_int
                            }
                            current_block = 1856101646708284338;
                        }
                        1054647088692577877 => {
                            new_game = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        5689001924483802034 => {
                            arg_fiddle = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        26972500619410423 => {
                            arg_wizard = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        14576567515993809846 => {
                            arg_sound = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        2719512138335094285 => {
                            arg_graphics = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        15125582407903384992 => {
                            arg_force_roguelike = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        4775909272756257391 => {
                            arg_force_original = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        9423205874568575888 => {
                            let mut s: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut j: libc::c_int = 0;
                            init_lua();
                            j = i + 1 as libc::c_int;
                            while j < argc {
                                s = *argv.offset(j as isize);
                                while *s as libc::c_int != '.' as i32 {
                                    s = s.offset(1)
                                }
                                *s = '\u{0}' as i32 as libc::c_char;
                                s = s.offset(1);
                                txt_to_html(b"head.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"foot.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            *argv.offset(j as isize) as cptr,
                                            s as cptr,
                                            0 as libc::c_int as bool_,
                                            0 as libc::c_int as bool_);
                                j += 1
                            }
                            return 0 as libc::c_int
                        }
                        _ => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   as libc::c_int == 'h' as i32 &&
                                   strcmp((*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                          b"help\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                let ref mut fresh0 = *argv.offset(i as isize);
                                *fresh0 =
                                    *argv.offset(0 as libc::c_int as isize);
                                argc = argc - i;
                                argv = argv.offset(i as isize);
                                args = 0 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                    }
                }
                79 | 111 => {
                    current_block = 4775909272756257391;
                    match current_block {
                        6545907279487748450 => {
                            change_path(&mut *(*argv.offset(i as
                                                                isize)).offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                            as *mut libc::c_char as cptr);
                            current_block = 1856101646708284338;
                        }
                        790185930182612747 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                force_module =
                                    string_make(&mut *(*argv.offset(i as
                                                                        isize)).offset(2
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize)
                                                    as *mut libc::c_char as
                                                    cptr);
                                current_block = 1856101646708284338;
                            }
                        }
                        7226443171521532240 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                mstr =
                                    &mut *(*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                        as *mut libc::c_char as cptr;
                                current_block = 1856101646708284338;
                            }
                        }
                        7245201122033322888 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                strcpy(player_name.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                strcpy(player_base.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                no_begin_screen = 1 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                        17281240262373992796 => {
                            show_score =
                                atoi(&mut *(*argv.offset(i as
                                                             isize)).offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize));
                            if show_score <= 0 as libc::c_int {
                                show_score = 10 as libc::c_int
                            }
                            current_block = 1856101646708284338;
                        }
                        1054647088692577877 => {
                            new_game = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        5689001924483802034 => {
                            arg_fiddle = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        26972500619410423 => {
                            arg_wizard = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        14576567515993809846 => {
                            arg_sound = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        2719512138335094285 => {
                            arg_graphics = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        15125582407903384992 => {
                            arg_force_roguelike = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        4775909272756257391 => {
                            arg_force_original = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        9423205874568575888 => {
                            let mut s: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut j: libc::c_int = 0;
                            init_lua();
                            j = i + 1 as libc::c_int;
                            while j < argc {
                                s = *argv.offset(j as isize);
                                while *s as libc::c_int != '.' as i32 {
                                    s = s.offset(1)
                                }
                                *s = '\u{0}' as i32 as libc::c_char;
                                s = s.offset(1);
                                txt_to_html(b"head.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"foot.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            *argv.offset(j as isize) as cptr,
                                            s as cptr,
                                            0 as libc::c_int as bool_,
                                            0 as libc::c_int as bool_);
                                j += 1
                            }
                            return 0 as libc::c_int
                        }
                        _ => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   as libc::c_int == 'h' as i32 &&
                                   strcmp((*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                          b"help\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                let ref mut fresh0 = *argv.offset(i as isize);
                                *fresh0 =
                                    *argv.offset(0 as libc::c_int as isize);
                                argc = argc - i;
                                argv = argv.offset(i as isize);
                                args = 0 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                    }
                }
                83 | 115 => {
                    current_block = 17281240262373992796;
                    match current_block {
                        6545907279487748450 => {
                            change_path(&mut *(*argv.offset(i as
                                                                isize)).offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                            as *mut libc::c_char as cptr);
                            current_block = 1856101646708284338;
                        }
                        790185930182612747 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                force_module =
                                    string_make(&mut *(*argv.offset(i as
                                                                        isize)).offset(2
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize)
                                                    as *mut libc::c_char as
                                                    cptr);
                                current_block = 1856101646708284338;
                            }
                        }
                        7226443171521532240 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                mstr =
                                    &mut *(*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                        as *mut libc::c_char as cptr;
                                current_block = 1856101646708284338;
                            }
                        }
                        7245201122033322888 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                strcpy(player_name.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                strcpy(player_base.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                no_begin_screen = 1 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                        17281240262373992796 => {
                            show_score =
                                atoi(&mut *(*argv.offset(i as
                                                             isize)).offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize));
                            if show_score <= 0 as libc::c_int {
                                show_score = 10 as libc::c_int
                            }
                            current_block = 1856101646708284338;
                        }
                        1054647088692577877 => {
                            new_game = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        5689001924483802034 => {
                            arg_fiddle = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        26972500619410423 => {
                            arg_wizard = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        14576567515993809846 => {
                            arg_sound = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        2719512138335094285 => {
                            arg_graphics = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        15125582407903384992 => {
                            arg_force_roguelike = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        4775909272756257391 => {
                            arg_force_original = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        9423205874568575888 => {
                            let mut s: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut j: libc::c_int = 0;
                            init_lua();
                            j = i + 1 as libc::c_int;
                            while j < argc {
                                s = *argv.offset(j as isize);
                                while *s as libc::c_int != '.' as i32 {
                                    s = s.offset(1)
                                }
                                *s = '\u{0}' as i32 as libc::c_char;
                                s = s.offset(1);
                                txt_to_html(b"head.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"foot.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            *argv.offset(j as isize) as cptr,
                                            s as cptr,
                                            0 as libc::c_int as bool_,
                                            0 as libc::c_int as bool_);
                                j += 1
                            }
                            return 0 as libc::c_int
                        }
                        _ => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   as libc::c_int == 'h' as i32 &&
                                   strcmp((*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                          b"help\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                let ref mut fresh0 = *argv.offset(i as isize);
                                *fresh0 =
                                    *argv.offset(0 as libc::c_int as isize);
                                argc = argc - i;
                                argv = argv.offset(i as isize);
                                args = 0 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                    }
                }
                117 | 85 => {
                    current_block = 7245201122033322888;
                    match current_block {
                        6545907279487748450 => {
                            change_path(&mut *(*argv.offset(i as
                                                                isize)).offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                            as *mut libc::c_char as cptr);
                            current_block = 1856101646708284338;
                        }
                        790185930182612747 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                force_module =
                                    string_make(&mut *(*argv.offset(i as
                                                                        isize)).offset(2
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize)
                                                    as *mut libc::c_char as
                                                    cptr);
                                current_block = 1856101646708284338;
                            }
                        }
                        7226443171521532240 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                mstr =
                                    &mut *(*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                        as *mut libc::c_char as cptr;
                                current_block = 1856101646708284338;
                            }
                        }
                        7245201122033322888 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                strcpy(player_name.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                strcpy(player_base.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                no_begin_screen = 1 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                        17281240262373992796 => {
                            show_score =
                                atoi(&mut *(*argv.offset(i as
                                                             isize)).offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize));
                            if show_score <= 0 as libc::c_int {
                                show_score = 10 as libc::c_int
                            }
                            current_block = 1856101646708284338;
                        }
                        1054647088692577877 => {
                            new_game = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        5689001924483802034 => {
                            arg_fiddle = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        26972500619410423 => {
                            arg_wizard = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        14576567515993809846 => {
                            arg_sound = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        2719512138335094285 => {
                            arg_graphics = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        15125582407903384992 => {
                            arg_force_roguelike = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        4775909272756257391 => {
                            arg_force_original = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        9423205874568575888 => {
                            let mut s: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut j: libc::c_int = 0;
                            init_lua();
                            j = i + 1 as libc::c_int;
                            while j < argc {
                                s = *argv.offset(j as isize);
                                while *s as libc::c_int != '.' as i32 {
                                    s = s.offset(1)
                                }
                                *s = '\u{0}' as i32 as libc::c_char;
                                s = s.offset(1);
                                txt_to_html(b"head.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"foot.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            *argv.offset(j as isize) as cptr,
                                            s as cptr,
                                            0 as libc::c_int as bool_,
                                            0 as libc::c_int as bool_);
                                j += 1
                            }
                            return 0 as libc::c_int
                        }
                        _ => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   as libc::c_int == 'h' as i32 &&
                                   strcmp((*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                          b"help\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                let ref mut fresh0 = *argv.offset(i as isize);
                                *fresh0 =
                                    *argv.offset(0 as libc::c_int as isize);
                                argc = argc - i;
                                argv = argv.offset(i as isize);
                                args = 0 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                    }
                }
                109 => {
                    current_block = 7226443171521532240;
                    match current_block {
                        6545907279487748450 => {
                            change_path(&mut *(*argv.offset(i as
                                                                isize)).offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                            as *mut libc::c_char as cptr);
                            current_block = 1856101646708284338;
                        }
                        790185930182612747 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                force_module =
                                    string_make(&mut *(*argv.offset(i as
                                                                        isize)).offset(2
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize)
                                                    as *mut libc::c_char as
                                                    cptr);
                                current_block = 1856101646708284338;
                            }
                        }
                        7226443171521532240 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                mstr =
                                    &mut *(*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                        as *mut libc::c_char as cptr;
                                current_block = 1856101646708284338;
                            }
                        }
                        7245201122033322888 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                strcpy(player_name.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                strcpy(player_base.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                no_begin_screen = 1 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                        17281240262373992796 => {
                            show_score =
                                atoi(&mut *(*argv.offset(i as
                                                             isize)).offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize));
                            if show_score <= 0 as libc::c_int {
                                show_score = 10 as libc::c_int
                            }
                            current_block = 1856101646708284338;
                        }
                        1054647088692577877 => {
                            new_game = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        5689001924483802034 => {
                            arg_fiddle = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        26972500619410423 => {
                            arg_wizard = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        14576567515993809846 => {
                            arg_sound = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        2719512138335094285 => {
                            arg_graphics = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        15125582407903384992 => {
                            arg_force_roguelike = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        4775909272756257391 => {
                            arg_force_original = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        9423205874568575888 => {
                            let mut s: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut j: libc::c_int = 0;
                            init_lua();
                            j = i + 1 as libc::c_int;
                            while j < argc {
                                s = *argv.offset(j as isize);
                                while *s as libc::c_int != '.' as i32 {
                                    s = s.offset(1)
                                }
                                *s = '\u{0}' as i32 as libc::c_char;
                                s = s.offset(1);
                                txt_to_html(b"head.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"foot.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            *argv.offset(j as isize) as cptr,
                                            s as cptr,
                                            0 as libc::c_int as bool_,
                                            0 as libc::c_int as bool_);
                                j += 1
                            }
                            return 0 as libc::c_int
                        }
                        _ => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   as libc::c_int == 'h' as i32 &&
                                   strcmp((*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                          b"help\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                let ref mut fresh0 = *argv.offset(i as isize);
                                *fresh0 =
                                    *argv.offset(0 as libc::c_int as isize);
                                argc = argc - i;
                                argv = argv.offset(i as isize);
                                args = 0 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                    }
                }
                77 => {
                    current_block = 790185930182612747;
                    match current_block {
                        6545907279487748450 => {
                            change_path(&mut *(*argv.offset(i as
                                                                isize)).offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                            as *mut libc::c_char as cptr);
                            current_block = 1856101646708284338;
                        }
                        790185930182612747 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                force_module =
                                    string_make(&mut *(*argv.offset(i as
                                                                        isize)).offset(2
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize)
                                                    as *mut libc::c_char as
                                                    cptr);
                                current_block = 1856101646708284338;
                            }
                        }
                        7226443171521532240 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                mstr =
                                    &mut *(*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                        as *mut libc::c_char as cptr;
                                current_block = 1856101646708284338;
                            }
                        }
                        7245201122033322888 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                strcpy(player_name.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                strcpy(player_base.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                no_begin_screen = 1 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                        17281240262373992796 => {
                            show_score =
                                atoi(&mut *(*argv.offset(i as
                                                             isize)).offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize));
                            if show_score <= 0 as libc::c_int {
                                show_score = 10 as libc::c_int
                            }
                            current_block = 1856101646708284338;
                        }
                        1054647088692577877 => {
                            new_game = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        5689001924483802034 => {
                            arg_fiddle = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        26972500619410423 => {
                            arg_wizard = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        14576567515993809846 => {
                            arg_sound = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        2719512138335094285 => {
                            arg_graphics = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        15125582407903384992 => {
                            arg_force_roguelike = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        4775909272756257391 => {
                            arg_force_original = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        9423205874568575888 => {
                            let mut s: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut j: libc::c_int = 0;
                            init_lua();
                            j = i + 1 as libc::c_int;
                            while j < argc {
                                s = *argv.offset(j as isize);
                                while *s as libc::c_int != '.' as i32 {
                                    s = s.offset(1)
                                }
                                *s = '\u{0}' as i32 as libc::c_char;
                                s = s.offset(1);
                                txt_to_html(b"head.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"foot.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            *argv.offset(j as isize) as cptr,
                                            s as cptr,
                                            0 as libc::c_int as bool_,
                                            0 as libc::c_int as bool_);
                                j += 1
                            }
                            return 0 as libc::c_int
                        }
                        _ => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   as libc::c_int == 'h' as i32 &&
                                   strcmp((*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                          b"help\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                let ref mut fresh0 = *argv.offset(i as isize);
                                *fresh0 =
                                    *argv.offset(0 as libc::c_int as isize);
                                argc = argc - i;
                                argv = argv.offset(i as isize);
                                args = 0 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                    }
                }
                72 => {
                    current_block = 9423205874568575888;
                    match current_block {
                        6545907279487748450 => {
                            change_path(&mut *(*argv.offset(i as
                                                                isize)).offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                            as *mut libc::c_char as cptr);
                            current_block = 1856101646708284338;
                        }
                        790185930182612747 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                force_module =
                                    string_make(&mut *(*argv.offset(i as
                                                                        isize)).offset(2
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize)
                                                    as *mut libc::c_char as
                                                    cptr);
                                current_block = 1856101646708284338;
                            }
                        }
                        7226443171521532240 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                mstr =
                                    &mut *(*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                        as *mut libc::c_char as cptr;
                                current_block = 1856101646708284338;
                            }
                        }
                        7245201122033322888 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                strcpy(player_name.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                strcpy(player_base.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                no_begin_screen = 1 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                        17281240262373992796 => {
                            show_score =
                                atoi(&mut *(*argv.offset(i as
                                                             isize)).offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize));
                            if show_score <= 0 as libc::c_int {
                                show_score = 10 as libc::c_int
                            }
                            current_block = 1856101646708284338;
                        }
                        1054647088692577877 => {
                            new_game = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        5689001924483802034 => {
                            arg_fiddle = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        26972500619410423 => {
                            arg_wizard = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        14576567515993809846 => {
                            arg_sound = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        2719512138335094285 => {
                            arg_graphics = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        15125582407903384992 => {
                            arg_force_roguelike = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        4775909272756257391 => {
                            arg_force_original = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        9423205874568575888 => {
                            let mut s: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut j: libc::c_int = 0;
                            init_lua();
                            j = i + 1 as libc::c_int;
                            while j < argc {
                                s = *argv.offset(j as isize);
                                while *s as libc::c_int != '.' as i32 {
                                    s = s.offset(1)
                                }
                                *s = '\u{0}' as i32 as libc::c_char;
                                s = s.offset(1);
                                txt_to_html(b"head.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"foot.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            *argv.offset(j as isize) as cptr,
                                            s as cptr,
                                            0 as libc::c_int as bool_,
                                            0 as libc::c_int as bool_);
                                j += 1
                            }
                            return 0 as libc::c_int
                        }
                        _ => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   as libc::c_int == 'h' as i32 &&
                                   strcmp((*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                          b"help\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                let ref mut fresh0 = *argv.offset(i as isize);
                                *fresh0 =
                                    *argv.offset(0 as libc::c_int as isize);
                                argc = argc - i;
                                argv = argv.offset(i as isize);
                                args = 0 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                    }
                }
                100 | 68 => {
                    current_block = 6545907279487748450;
                    match current_block {
                        6545907279487748450 => {
                            change_path(&mut *(*argv.offset(i as
                                                                isize)).offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                            as *mut libc::c_char as cptr);
                            current_block = 1856101646708284338;
                        }
                        790185930182612747 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                force_module =
                                    string_make(&mut *(*argv.offset(i as
                                                                        isize)).offset(2
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize)
                                                    as *mut libc::c_char as
                                                    cptr);
                                current_block = 1856101646708284338;
                            }
                        }
                        7226443171521532240 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                mstr =
                                    &mut *(*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                        as *mut libc::c_char as cptr;
                                current_block = 1856101646708284338;
                            }
                        }
                        7245201122033322888 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                strcpy(player_name.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                strcpy(player_base.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                no_begin_screen = 1 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                        17281240262373992796 => {
                            show_score =
                                atoi(&mut *(*argv.offset(i as
                                                             isize)).offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize));
                            if show_score <= 0 as libc::c_int {
                                show_score = 10 as libc::c_int
                            }
                            current_block = 1856101646708284338;
                        }
                        1054647088692577877 => {
                            new_game = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        5689001924483802034 => {
                            arg_fiddle = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        26972500619410423 => {
                            arg_wizard = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        14576567515993809846 => {
                            arg_sound = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        2719512138335094285 => {
                            arg_graphics = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        15125582407903384992 => {
                            arg_force_roguelike = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        4775909272756257391 => {
                            arg_force_original = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        9423205874568575888 => {
                            let mut s: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut j: libc::c_int = 0;
                            init_lua();
                            j = i + 1 as libc::c_int;
                            while j < argc {
                                s = *argv.offset(j as isize);
                                while *s as libc::c_int != '.' as i32 {
                                    s = s.offset(1)
                                }
                                *s = '\u{0}' as i32 as libc::c_char;
                                s = s.offset(1);
                                txt_to_html(b"head.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"foot.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            *argv.offset(j as isize) as cptr,
                                            s as cptr,
                                            0 as libc::c_int as bool_,
                                            0 as libc::c_int as bool_);
                                j += 1
                            }
                            return 0 as libc::c_int
                        }
                        _ => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   as libc::c_int == 'h' as i32 &&
                                   strcmp((*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                          b"help\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                let ref mut fresh0 = *argv.offset(i as isize);
                                *fresh0 =
                                    *argv.offset(0 as libc::c_int as isize);
                                argc = argc - i;
                                argv = argv.offset(i as isize);
                                args = 0 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                    }
                }
                45 => {
                    current_block = 4888910987971495881;
                    match current_block {
                        6545907279487748450 => {
                            change_path(&mut *(*argv.offset(i as
                                                                isize)).offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                            as *mut libc::c_char as cptr);
                            current_block = 1856101646708284338;
                        }
                        790185930182612747 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                force_module =
                                    string_make(&mut *(*argv.offset(i as
                                                                        isize)).offset(2
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize)
                                                    as *mut libc::c_char as
                                                    cptr);
                                current_block = 1856101646708284338;
                            }
                        }
                        7226443171521532240 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                mstr =
                                    &mut *(*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                        as *mut libc::c_char as cptr;
                                current_block = 1856101646708284338;
                            }
                        }
                        7245201122033322888 => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                strcpy(player_name.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                strcpy(player_base.as_mut_ptr(),
                                       &mut *(*argv.offset(i as
                                                               isize)).offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
                                no_begin_screen = 1 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                        17281240262373992796 => {
                            show_score =
                                atoi(&mut *(*argv.offset(i as
                                                             isize)).offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize));
                            if show_score <= 0 as libc::c_int {
                                show_score = 10 as libc::c_int
                            }
                            current_block = 1856101646708284338;
                        }
                        1054647088692577877 => {
                            new_game = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        5689001924483802034 => {
                            arg_fiddle = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        26972500619410423 => {
                            arg_wizard = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        14576567515993809846 => {
                            arg_sound = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        2719512138335094285 => {
                            arg_graphics = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        15125582407903384992 => {
                            arg_force_roguelike = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        4775909272756257391 => {
                            arg_force_original = 1 as libc::c_int as bool_;
                            current_block = 1856101646708284338;
                        }
                        9423205874568575888 => {
                            let mut s: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut j: libc::c_int = 0;
                            init_lua();
                            j = i + 1 as libc::c_int;
                            while j < argc {
                                s = *argv.offset(j as isize);
                                while *s as libc::c_int != '.' as i32 {
                                    s = s.offset(1)
                                }
                                *s = '\u{0}' as i32 as libc::c_char;
                                s = s.offset(1);
                                txt_to_html(b"head.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"foot.aux\x00" as *const u8 as
                                                *const libc::c_char,
                                            *argv.offset(j as isize) as cptr,
                                            s as cptr,
                                            0 as libc::c_int as bool_,
                                            0 as libc::c_int as bool_);
                                j += 1
                            }
                            return 0 as libc::c_int
                        }
                        _ => {
                            if *(*argv.offset(i as
                                                  isize)).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                   as libc::c_int == 'h' as i32 &&
                                   strcmp((*argv.offset(i as
                                                            isize)).offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                          b"help\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                current_block = 3934757609181694121;
                            } else {
                                let ref mut fresh0 = *argv.offset(i as isize);
                                *fresh0 =
                                    *argv.offset(0 as libc::c_int as isize);
                                argc = argc - i;
                                argv = argv.offset(i as isize);
                                args = 0 as libc::c_int as bool_;
                                current_block = 1856101646708284338;
                            }
                        }
                    }
                }
                104 | _ => { current_block = 3934757609181694121; }
            }
        }
        match current_block {
            3934757609181694121 => {
                let mut j_0: libc::c_int = 0;
                /* Dump usage information */
                j_0 = 0 as libc::c_int;
                while j_0 < argc {
                    printf(b"%s \x00" as *const u8 as *const libc::c_char,
                           *argv.offset(j_0 as isize));
                    j_0 += 1
                }
                printf(b"\n\x00" as *const u8 as *const libc::c_char);
                puts(b"Usage: tome [options] [-- subopts]\x00" as *const u8 as
                         *const libc::c_char);
                puts(b"  -h                 This help\x00" as *const u8 as
                         *const libc::c_char);
                puts(b"  -n                 Start a new character\x00" as
                         *const u8 as *const libc::c_char);
                puts(b"  -f                 Request fiddle mode\x00" as
                         *const u8 as *const libc::c_char);
                puts(b"  -w                 Request wizard mode\x00" as
                         *const u8 as *const libc::c_char);
                puts(b"  -v                 Request sound mode\x00" as
                         *const u8 as *const libc::c_char);
                puts(b"  -g                 Request graphics mode\x00" as
                         *const u8 as *const libc::c_char);
                puts(b"  -o                 Request original keyset\x00" as
                         *const u8 as *const libc::c_char);
                puts(b"  -r                 Request rogue-like keyset\x00" as
                         *const u8 as *const libc::c_char);
                puts(b"  -H <list of files> Convert helpfile to html\x00" as
                         *const u8 as *const libc::c_char);
                puts(b"  -s<num>            Show <num> high scores\x00" as
                         *const u8 as *const libc::c_char);
                puts(b"  -u<who>            Use your <who> savefile\x00" as
                         *const u8 as *const libc::c_char);
                puts(b"  -M<which>            Use the <which> module\x00" as
                         *const u8 as *const libc::c_char);
                puts(b"  -m<sys>            Force \'main-<sys>.c\' usage\x00"
                         as *const u8 as *const libc::c_char);
                puts(b"  -d<def>            Define a \'lib\' dir sub-path\x00"
                         as *const u8 as *const libc::c_char);
                /* USE_GTK2 */
                /* USE_XAW */
                puts(b"  -mx11              To use X11\x00" as *const u8 as
                         *const libc::c_char);
                puts(b"  --                 Sub options\x00" as *const u8 as
                         *const libc::c_char);
                puts(b"  -- -n#             Number of terms to use\x00" as
                         *const u8 as *const libc::c_char);
                puts(b"  -- -d<name>        Display to use\x00" as *const u8
                         as *const libc::c_char);
                puts(b"  -- -s              Turn off smoothscaling graphics\x00"
                         as *const u8 as *const libc::c_char);
                puts(b"  -- -o              Requests \"old\" graphics\x00" as
                         *const u8 as *const libc::c_char);
                puts(b"  -- -b              Requests double-width tiles\x00"
                         as *const u8 as *const libc::c_char);
                /* USE_GRAPHICS */
                /* USE_X11 */
                puts(b"  -mgcu              To use curses\x00" as *const u8 as
                         *const libc::c_char);
                puts(b"  --                 Sub options\x00" as *const u8 as
                         *const libc::c_char);
                puts(b"  -- -b              Requests big screen\x00" as
                         *const u8 as *const libc::c_char);
                /* USE_GCU */
                /* USE_SLA */
                /* USE_SDL */
                /* Actually abort the process */
                quit(0 as cptr);
            }
            _ => { }
        }
        i += 1
    }
    /* Hack -- Forget standard args */
    if args != 0 {
        argc = 1 as libc::c_int;
        let ref mut fresh1 = *argv.offset(1 as libc::c_int as isize);
        *fresh1 = 0 as *mut libc::c_char
    }
    /* Process the player name */
    process_player_name(1 as libc::c_int as bool_);
    /* Install "quit" hook */
    quit_aux = Some(quit_hook as unsafe extern "C" fn(_: cptr) -> ());
    /* Attempt to use the "main-x11.c" support */
    if done == 0 &&
           (mstr.is_null() ||
                streq(mstr, b"x11\x00" as *const u8 as *const libc::c_char) as
                    libc::c_int != 0) {
        extern "C" {
            #[link_name = "init_x11"]
            fn init_x11_0(_: libc::c_int, _: *mut *mut libc::c_char) -> errr;
        }
        if 0 as libc::c_int == init_x11_0(argc, argv) {
            ANGBAND_SYS = b"x11\x00" as *const u8 as *const libc::c_char;
            done = 1 as libc::c_int as bool_
        }
    }
    /* Attempt to use the "main-gcu.c" support */
    if done == 0 &&
           (mstr.is_null() ||
                streq(mstr, b"gcu\x00" as *const u8 as *const libc::c_char) as
                    libc::c_int != 0) {
        extern "C" {
            #[link_name = "init_gcu"]
            fn init_gcu_0(_: libc::c_int, _: *mut *mut libc::c_char) -> errr;
        }
        if 0 as libc::c_int == init_gcu_0(argc, argv) {
            ANGBAND_SYS = b"gcu\x00" as *const u8 as *const libc::c_char;
            done = 1 as libc::c_int as bool_
        }
    }
    /* Make sure we have a display! */
    if done == 0 {
        quit(b"Unable to prepare any \'display module\'!\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Catch nasty signals */
    signals_init();
    /* Initialize */
    init_angband();
    /* Hack -- If requested, display scores and quit */
    if show_score > 0 as libc::c_int {
        display_scores(0 as libc::c_int, show_score);
    }
    /* Wait for response */
    pause_line(23 as libc::c_int);
    /* Play the game */
    play_game(new_game);
    /* Quit */
    quit(0 as cptr);
    /* Exit */
    return 0 as libc::c_int;
}
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
