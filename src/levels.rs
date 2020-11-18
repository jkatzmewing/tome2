use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    static mut d_info: *mut dungeon_info_type;
    #[no_mangle]
    static mut ANGBAND_DIR_DNGN: cptr;
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut dungeon_flags1: u32b;
    #[no_mangle]
    static mut dungeon_flags2: u32b;
    #[no_mangle]
    fn grab_one_dungeon_flag(flags1: *mut u32b, flags2: *mut u32b, what: cptr)
     -> errr;
    #[no_mangle]
    fn path_build(buf: *mut libc::c_char, max: libc::c_int, path: cptr,
                  file: cptr) -> errr;
    #[no_mangle]
    fn my_fopen(file: cptr, mode: cptr) -> *mut FILE;
    #[no_mangle]
    fn my_fgets(fff: *mut FILE, buf: *mut libc::c_char, n: huge_hack) -> errr;
    #[no_mangle]
    fn my_fclose(fff: *mut FILE) -> errr;
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
pub type cptr = *const libc::c_char;
pub type errr = libc::c_int;
pub type byte_hack = libc::c_uchar;
pub type bool_ = libc::c_char;
pub type huge_hack = libc::c_ulong;
pub type s16b = libc::c_short;
pub type u32b = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct obj_theme {
    pub treasure: byte_hack,
    pub combat: byte_hack,
    pub magic: byte_hack,
    pub tools: byte_hack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rule_type {
    pub mode: byte_hack,
    pub percent: byte_hack,
    pub mflags1: u32b,
    pub mflags2: u32b,
    pub mflags3: u32b,
    pub mflags4: u32b,
    pub mflags5: u32b,
    pub mflags6: u32b,
    pub mflags7: u32b,
    pub mflags8: u32b,
    pub mflags9: u32b,
    pub r_char: [libc::c_char; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dungeon_info_type {
    pub name: u32b,
    pub text: u32b,
    pub short_name: [libc::c_char; 3],
    pub generator: [libc::c_char; 30],
    pub floor1: s16b,
    pub floor_percent1: [byte_hack; 2],
    pub floor2: s16b,
    pub floor_percent2: [byte_hack; 2],
    pub floor3: s16b,
    pub floor_percent3: [byte_hack; 2],
    pub outer_wall: s16b,
    pub inner_wall: s16b,
    pub fill_type1: s16b,
    pub fill_percent1: [byte_hack; 2],
    pub fill_type2: s16b,
    pub fill_percent2: [byte_hack; 2],
    pub fill_type3: s16b,
    pub fill_percent3: [byte_hack; 2],
    pub fill_method: byte_hack,
    pub mindepth: s16b,
    pub maxdepth: s16b,
    pub principal: bool_,
    pub next: byte_hack,
    pub min_plev: byte_hack,
    pub min_m_alloc_level: libc::c_int,
    pub max_m_alloc_chance: libc::c_int,
    pub flags1: u32b,
    pub flags2: u32b,
    pub size_x: libc::c_int,
    pub size_y: libc::c_int,
    pub rule_percents: [byte_hack; 100],
    pub rules: [rule_type; 5],
    pub final_object: libc::c_int,
    pub final_artifact: libc::c_int,
    pub final_guardian: libc::c_int,
    pub ix: libc::c_int,
    pub iy: libc::c_int,
    pub ox: libc::c_int,
    pub oy: libc::c_int,
    pub objs: obj_theme,
    pub d_dice: [libc::c_int; 4],
    pub d_side: [libc::c_int; 4],
    pub d_frequency: [libc::c_int; 4],
    pub d_type: [libc::c_int; 4],
    pub t_idx: [s16b; 4],
    pub t_level: [s16b; 4],
    pub t_num: s16b,
}
/* File: levels.c */
/* Purpose: Levels functions */
/*
 * Copyright (c) 1989 James E. Wilson, Robert A. Koeneke
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Return the parameter of the given command in the given file
 */
static mut start_line: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn get_command(mut file: *const libc::c_char,
                                     mut comm: libc::c_char,
                                     mut param: *mut libc::c_char) -> bool_ {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_int = -(1 as libc::c_int);
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_DNGN, file);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* The file exists ? */
	/* no ? then command not found */
    if fp.is_null() { return 0 as libc::c_int as bool_ }
    /* Parse to the end of the file or when the command is found */
    while 0 as libc::c_int ==
              my_fgets(fp, buf.as_mut_ptr(), 1024 as libc::c_int as huge_hack)
          {
        /* Advance the line number */
        i += 1;
        /* Skip comments and blank lines */
        if buf[0 as libc::c_int as usize] == 0 ||
               buf[0 as libc::c_int as usize] as libc::c_int == '#' as i32 {
            continue ;
        }
        /* Is it the command we are looking for ? */
        if i > start_line &&
               buf[0 as libc::c_int as usize] as libc::c_int ==
                   comm as libc::c_int {
            /* Acquire the text */
            s = buf.as_mut_ptr().offset(2 as libc::c_int as isize);
            start_line = i;
            /* Get the parameter */
            strcpy(param, s);
            /* Close it */
            my_fclose(fp);
            return 1 as libc::c_int as bool_
        }
    }
    /* Close it */
    my_fclose(fp);
    /* Assume command not found */
    return 0 as libc::c_int as bool_;
}
/*
 * Return the dungeon branch starting form the current dungeon/level
 */
#[no_mangle]
pub unsafe extern "C" fn get_branch() -> libc::c_int {
    let mut file: [libc::c_char; 20] = [0; 20];
    let mut buf: [libc::c_char; 5] = [0; 5];
    sprintf(file.as_mut_ptr(),
            b"dun%d.%d\x00" as *const u8 as *const libc::c_char,
            dungeon_type as libc::c_int,
            dun_level as libc::c_int -
                (*d_info.offset(dungeon_type as isize)).mindepth as
                    libc::c_int);
    /* Get and return the branch */
    start_line = -(1 as libc::c_int);
    if get_command(file.as_mut_ptr(), 'B' as i32 as libc::c_char,
                   buf.as_mut_ptr()) != 0 {
        return atoi(buf.as_mut_ptr())
    } else {
        /* No branch ? return 0 */
        return 0 as libc::c_int
    };
}
/*
 * Return the father dungeon branch
 */
#[no_mangle]
pub unsafe extern "C" fn get_fbranch() -> libc::c_int {
    let mut file: [libc::c_char; 20] = [0; 20];
    let mut buf: [libc::c_char; 5] = [0; 5];
    sprintf(file.as_mut_ptr(),
            b"dun%d.%d\x00" as *const u8 as *const libc::c_char,
            dungeon_type as libc::c_int,
            dun_level as libc::c_int -
                (*d_info.offset(dungeon_type as isize)).mindepth as
                    libc::c_int);
    /* Get and return the branch */
    start_line = -(1 as libc::c_int);
    if get_command(file.as_mut_ptr(), 'A' as i32 as libc::c_char,
                   buf.as_mut_ptr()) != 0 {
        return atoi(buf.as_mut_ptr())
    } else {
        /* No branch ? return 0 */
        return 0 as libc::c_int
    };
}
/*
 * Return the father dungeon level
 */
#[no_mangle]
pub unsafe extern "C" fn get_flevel() -> libc::c_int {
    let mut file: [libc::c_char; 20] = [0; 20];
    let mut buf: [libc::c_char; 5] = [0; 5];
    sprintf(file.as_mut_ptr(),
            b"dun%d.%d\x00" as *const u8 as *const libc::c_char,
            dungeon_type as libc::c_int,
            dun_level as libc::c_int -
                (*d_info.offset(dungeon_type as isize)).mindepth as
                    libc::c_int);
    /* Get and return the level */
    start_line = -(1 as libc::c_int);
    if get_command(file.as_mut_ptr(), 'L' as i32 as libc::c_char,
                   buf.as_mut_ptr()) != 0 {
        return atoi(buf.as_mut_ptr())
    } else {
        /* No level ? return 0 */
        return 0 as libc::c_int
    };
}
/*
 * Return the extension of the savefile for the level
 */
#[no_mangle]
pub unsafe extern "C" fn get_dungeon_save(mut buf: *mut libc::c_char)
 -> bool_ {
    let mut file: [libc::c_char; 20] = [0; 20];
    sprintf(file.as_mut_ptr(),
            b"dun%d.%d\x00" as *const u8 as *const libc::c_char,
            dungeon_type as libc::c_int,
            dun_level as libc::c_int -
                (*d_info.offset(dungeon_type as isize)).mindepth as
                    libc::c_int);
    /* Get and return the level */
    start_line = -(1 as libc::c_int);
    if get_command(file.as_mut_ptr(), 'S' as i32 as libc::c_char, buf) != 0 {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
/*
 * Return the level generator
 */
#[no_mangle]
pub unsafe extern "C" fn get_dungeon_generator(mut buf: *mut libc::c_char)
 -> bool_ {
    let mut file: [libc::c_char; 20] = [0; 20];
    sprintf(file.as_mut_ptr(),
            b"dun%d.%d\x00" as *const u8 as *const libc::c_char,
            dungeon_type as libc::c_int,
            dun_level as libc::c_int -
                (*d_info.offset(dungeon_type as isize)).mindepth as
                    libc::c_int);
    /* Get and return the level */
    start_line = -(1 as libc::c_int);
    if get_command(file.as_mut_ptr(), 'G' as i32 as libc::c_char, buf) != 0 {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
/*
 * Return the special level
 */
#[no_mangle]
pub unsafe extern "C" fn get_dungeon_special(mut buf: *mut libc::c_char)
 -> bool_ {
    let mut file: [libc::c_char; 20] = [0; 20];
    sprintf(file.as_mut_ptr(),
            b"dun%d.%d\x00" as *const u8 as *const libc::c_char,
            dungeon_type as libc::c_int,
            dun_level as libc::c_int -
                (*d_info.offset(dungeon_type as isize)).mindepth as
                    libc::c_int);
    /* Get and return the level */
    start_line = -(1 as libc::c_int);
    if get_command(file.as_mut_ptr(), 'U' as i32 as libc::c_char, buf) != 0 {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
/*
 * Return the special level name
 */
#[no_mangle]
pub unsafe extern "C" fn get_dungeon_name(mut buf: *mut libc::c_char)
 -> bool_ {
    let mut file: [libc::c_char; 20] = [0; 20];
    sprintf(file.as_mut_ptr(),
            b"dun%d.%d\x00" as *const u8 as *const libc::c_char,
            dungeon_type as libc::c_int,
            dun_level as libc::c_int -
                (*d_info.offset(dungeon_type as isize)).mindepth as
                    libc::c_int);
    /* Get and return the level */
    start_line = -(1 as libc::c_int);
    if get_command(file.as_mut_ptr(), 'N' as i32 as libc::c_char, buf) != 0 {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
/*
 * Return the special level name
 */
#[no_mangle]
pub unsafe extern "C" fn get_level_flags() {
    let mut file: [libc::c_char; 20] = [0; 20];
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    sprintf(file.as_mut_ptr(),
            b"dun%d.%d\x00" as *const u8 as *const libc::c_char,
            dungeon_type as libc::c_int,
            dun_level as libc::c_int -
                (*d_info.offset(dungeon_type as isize)).mindepth as
                    libc::c_int);
    start_line = -(1 as libc::c_int);
    /* Parse until done */
    while get_command(file.as_mut_ptr(), 'F' as i32 as libc::c_char,
                      buf.as_mut_ptr()) != 0 {
        /* Parse every entry textually */
        s = buf.as_mut_ptr();
        while *s != 0 {
            /* Find the end of this entry */
            t = s;
            while *t as libc::c_int != 0 && *t as libc::c_int != ' ' as i32 &&
                      *t as libc::c_int != '|' as i32 {
                /* loop */
                t = t.offset(1)
            }
            /* Nuke and skip any dividers */
            if *t != 0 {
                let fresh0 = t;
                t = t.offset(1);
                *fresh0 = '\u{0}' as i32 as libc::c_char;
                while *t as libc::c_int == ' ' as i32 ||
                          *t as libc::c_int == '|' as i32 {
                    t = t.offset(1)
                }
            }
            /* Parse this entry */
            if 0 as libc::c_int !=
                   grab_one_dungeon_flag(&mut dungeon_flags1,
                                         &mut dungeon_flags2, s as cptr) {
                return
            }
            /* Start the next entry */
            s = t
        }
    };
}
/*
 * Return the special level desc
 */
#[no_mangle]
pub unsafe extern "C" fn get_level_desc(mut buf: *mut libc::c_char) -> bool_ {
    let mut file: [libc::c_char; 20] = [0; 20];
    sprintf(file.as_mut_ptr(),
            b"dun%d.%d\x00" as *const u8 as *const libc::c_char,
            dungeon_type as libc::c_int,
            dun_level as libc::c_int -
                (*d_info.offset(dungeon_type as isize)).mindepth as
                    libc::c_int);
    /* Get and return the level */
    start_line = -(1 as libc::c_int);
    if get_command(file.as_mut_ptr(), 'D' as i32 as libc::c_char, buf) != 0 {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
