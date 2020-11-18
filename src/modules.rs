use ::libc;
extern "C" {
    #[no_mangle]
    static mut version_major: byte_hack;
    #[no_mangle]
    static mut version_minor: byte_hack;
    #[no_mangle]
    static mut version_patch: byte_hack;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn quit(str: cptr);
    #[no_mangle]
    fn string_make(str: cptr) -> cptr;
    #[no_mangle]
    fn string_free(str: cptr) -> errr;
    #[no_mangle]
    fn strnfmt(buf: *mut libc::c_char, max: uint_hack, fmt: cptr, _: ...)
     -> uint_hack;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Term_xtra(n: libc::c_int, v: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_clear() -> errr;
    #[no_mangle]
    fn toupper(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    static mut angband_term_name: [[libc::c_char; 80]; 8];
    #[no_mangle]
    static mut ANGBAND_DIR_APEX: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_CORE: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_DNGN: cptr;
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
    static mut ANGBAND_DIR_MODULES: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_NOTE: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_SAVE: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_SCPT: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_PATCH: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_PREF: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_USER: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_XTRA: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_CMOV: cptr;
    #[no_mangle]
    static mut RANDART_WEAPON: s32b;
    #[no_mangle]
    static mut RANDART_ARMOR: s32b;
    #[no_mangle]
    static mut RANDART_JEWEL: s32b;
    #[no_mangle]
    static mut game_module: cptr;
    #[no_mangle]
    static mut VERSION_MAJOR: s32b;
    #[no_mangle]
    static mut VERSION_MINOR: s32b;
    #[no_mangle]
    static mut VERSION_PATCH: s32b;
    #[no_mangle]
    static mut max_plev: s32b;
    #[no_mangle]
    static mut DUNGEON_DEATH: s32b;
    #[no_mangle]
    fn print_desc_aux(txt: cptr, y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn process_pref_file(name: cptr) -> errr;
    #[no_mangle]
    fn process_player_base();
    #[no_mangle]
    fn path_parse(buf: *mut libc::c_char, max: libc::c_int, file: cptr)
     -> errr;
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn c_put_str(attr: byte_hack, str: cptr, row: libc::c_int,
                 col: libc::c_int);
    #[no_mangle]
    fn put_str(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn private_check_user_directory(dirpath: cptr) -> bool_;
    #[no_mangle]
    fn init_lua();
    #[no_mangle]
    fn tome_dofile_anywhere(dir: cptr, file: *mut libc::c_char,
                            test_exist: bool_) -> bool_;
    #[no_mangle]
    fn call_lua(function: cptr, args: cptr, ret: cptr, _: ...) -> bool_;
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
pub type cptr = *const libc::c_char;
pub type errr = libc::c_int;
pub type byte_hack = libc::c_uchar;
pub type bool_ = libc::c_char;
pub type uint_hack = libc::c_uint;
pub type s32b = libc::c_int;
/* File: modules.c */
/* Purpose: T-engine modules */
/*
 * Copyright (c) 2003 DarkGod
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
unsafe extern "C" fn module_reset_dir_aux(mut dir: *mut cptr,
                                          mut new_path: cptr) {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Build the new path */
    strnfmt(buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as
                uint_hack, b"%s%s%s\x00" as *const u8 as *const libc::c_char,
            *dir, b"/\x00" as *const u8 as *const libc::c_char, new_path);
    string_free(*dir);
    *dir = string_make(buf.as_mut_ptr() as cptr);
    /* Make it if needed */
    if private_check_user_directory(*dir) == 0 {
        quit(format(b"Unable to create module dir %s\n\x00" as *const u8 as
                        *const libc::c_char, *dir) as cptr);
    };
}
#[no_mangle]
pub unsafe extern "C" fn module_reset_dir(mut dir: cptr, mut new_path: cptr) {
    let mut d: *mut cptr = 0 as *mut cptr;
    let mut buf: [libc::c_char; 1025] = [0; 1025];
    if strcmp(dir, b"apex\x00" as *const u8 as *const libc::c_char) == 0 {
        d = &mut ANGBAND_DIR_APEX
    }
    if strcmp(dir, b"core\x00" as *const u8 as *const libc::c_char) == 0 {
        d = &mut ANGBAND_DIR_CORE
    }
    if strcmp(dir, b"dngn\x00" as *const u8 as *const libc::c_char) == 0 {
        d = &mut ANGBAND_DIR_DNGN
    }
    if strcmp(dir, b"data\x00" as *const u8 as *const libc::c_char) == 0 {
        d = &mut ANGBAND_DIR_DATA
    }
    if strcmp(dir, b"edit\x00" as *const u8 as *const libc::c_char) == 0 {
        d = &mut ANGBAND_DIR_EDIT
    }
    if strcmp(dir, b"file\x00" as *const u8 as *const libc::c_char) == 0 {
        d = &mut ANGBAND_DIR_FILE
    }
    if strcmp(dir, b"help\x00" as *const u8 as *const libc::c_char) == 0 {
        d = &mut ANGBAND_DIR_HELP
    }
    if strcmp(dir, b"info\x00" as *const u8 as *const libc::c_char) == 0 {
        d = &mut ANGBAND_DIR_INFO
    }
    if strcmp(dir, b"scpt\x00" as *const u8 as *const libc::c_char) == 0 {
        d = &mut ANGBAND_DIR_SCPT
    }
    if strcmp(dir, b"patch\x00" as *const u8 as *const libc::c_char) == 0 {
        d = &mut ANGBAND_DIR_PATCH
    }
    if strcmp(dir, b"pref\x00" as *const u8 as *const libc::c_char) == 0 {
        d = &mut ANGBAND_DIR_PREF
    }
    if strcmp(dir, b"xtra\x00" as *const u8 as *const libc::c_char) == 0 {
        d = &mut ANGBAND_DIR_XTRA
    }
    if strcmp(dir, b"user\x00" as *const u8 as *const libc::c_char) == 0 {
        d = &mut ANGBAND_DIR_USER
    }
    if strcmp(dir, b"note\x00" as *const u8 as *const libc::c_char) == 0 {
        d = &mut ANGBAND_DIR_NOTE
    }
    if strcmp(dir, b"cmov\x00" as *const u8 as *const libc::c_char) == 0 {
        d = &mut ANGBAND_DIR_CMOV
    }
    if strcmp(dir, b"apex\x00" as *const u8 as *const libc::c_char) == 0 ||
           strcmp(dir, b"user\x00" as *const u8 as *const libc::c_char) == 0
           ||
           strcmp(dir, b"note\x00" as *const u8 as *const libc::c_char) == 0
           ||
           strcmp(dir, b"cmov\x00" as *const u8 as *const libc::c_char) == 0 {
        let mut user_path: [libc::c_char; 1024] = [0; 1024];
        /* copied from init_file_paths */
        path_parse(user_path.as_mut_ptr(), 1024 as libc::c_int,
                   b"~/.tome\x00" as *const u8 as *const libc::c_char);
        strcat(user_path.as_mut_ptr(),
               b"/2.3\x00" as *const u8 as *const libc::c_char);
        strnfmt(buf.as_mut_ptr(), 1024 as libc::c_int as uint_hack,
                b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                user_path.as_mut_ptr(),
                b"/\x00" as *const u8 as *const libc::c_char, new_path);
        string_free(*d);
        *d = string_make(buf.as_mut_ptr() as cptr)
    } else if strcmp(dir, b"save\x00" as *const u8 as *const libc::c_char) ==
                  0 {
        module_reset_dir_aux(&mut ANGBAND_DIR_SAVE, new_path);
    } else {
        /* Build the new path */
        strnfmt(buf.as_mut_ptr(), 1024 as libc::c_int as uint_hack,
                b"%s%s%s%s%s\x00" as *const u8 as *const libc::c_char,
                ANGBAND_DIR_MODULES,
                b"/\x00" as *const u8 as *const libc::c_char, new_path,
                b"/\x00" as *const u8 as *const libc::c_char, dir);
        string_free(*d);
        *d = string_make(buf.as_mut_ptr() as cptr)
    };
}
unsafe extern "C" fn dump_modules(mut sel: libc::c_int,
                                  mut max: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 40] = [0; 40];
    let mut pre: libc::c_char = ' ' as i32 as libc::c_char;
    let mut post: libc::c_char = ')' as i32 as libc::c_char;
    let mut name: cptr = 0 as *const libc::c_char;
    let mut ind: libc::c_char = 0;
    i = 0 as libc::c_int;
    while i < max {
        ind = (i % 26 as libc::c_int + 'a' as i32) as libc::c_char;
        if i >= 26 as libc::c_int {
            ind = toupper(ind as libc::c_int) as libc::c_char
        }
        if sel == i {
            pre = '[' as i32 as libc::c_char;
            post = ']' as i32 as libc::c_char
        } else {
            pre = ' ' as i32 as libc::c_char;
            post = ')' as i32 as libc::c_char
        }
        call_lua(b"get_module_name\x00" as *const u8 as *const libc::c_char,
                 b"(d)\x00" as *const u8 as *const libc::c_char,
                 b"s\x00" as *const u8 as *const libc::c_char, i,
                 &mut name as *mut cptr);
        strnfmt(buf.as_mut_ptr(), 40 as libc::c_int as uint_hack,
                b"%c%c%c %s\x00" as *const u8 as *const libc::c_char,
                pre as libc::c_int, ind as libc::c_int, post as libc::c_int,
                name);
        if sel == i {
            call_lua(b"get_module_desc\x00" as *const u8 as
                         *const libc::c_char,
                     b"(d)\x00" as *const u8 as *const libc::c_char,
                     b"s\x00" as *const u8 as *const libc::c_char, i,
                     &mut name as *mut cptr);
            print_desc_aux(name, 5 as libc::c_int, 0 as libc::c_int);
            c_put_str(14 as libc::c_int as byte_hack,
                      buf.as_mut_ptr() as cptr,
                      10 as libc::c_int + i / 4 as libc::c_int,
                      20 as libc::c_int * (i % 4 as libc::c_int));
        } else {
            put_str(buf.as_mut_ptr() as cptr,
                    10 as libc::c_int + i / 4 as libc::c_int,
                    20 as libc::c_int * (i % 4 as libc::c_int));
        }
        i += 1
    };
}
unsafe extern "C" fn activate_module() {
    /* Initialize the module table */
    call_lua(b"assign_current_module\x00" as *const u8 as *const libc::c_char,
             b"(s)\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char, game_module);
    /* Do misc inits  */
    call_lua(b"get_module_info\x00" as *const u8 as *const libc::c_char,
             b"(s)\x00" as *const u8 as *const libc::c_char,
             b"d\x00" as *const u8 as *const libc::c_char,
             b"max_plev\x00" as *const u8 as *const libc::c_char,
             &mut max_plev as *mut s32b);
    call_lua(b"get_module_info\x00" as *const u8 as *const libc::c_char,
             b"(s)\x00" as *const u8 as *const libc::c_char,
             b"d\x00" as *const u8 as *const libc::c_char,
             b"death_dungeon\x00" as *const u8 as *const libc::c_char,
             &mut DUNGEON_DEATH as *mut s32b);
    call_lua(b"get_module_info\x00" as *const u8 as *const libc::c_char,
             b"(s)\x00" as *const u8 as *const libc::c_char,
             b"d\x00" as *const u8 as *const libc::c_char,
             b"random_artifact_weapon_chance\x00" as *const u8 as
                 *const libc::c_char, &mut RANDART_WEAPON as *mut s32b);
    call_lua(b"get_module_info\x00" as *const u8 as *const libc::c_char,
             b"(s)\x00" as *const u8 as *const libc::c_char,
             b"d\x00" as *const u8 as *const libc::c_char,
             b"random_artifact_armor_chance\x00" as *const u8 as
                 *const libc::c_char, &mut RANDART_ARMOR as *mut s32b);
    call_lua(b"get_module_info\x00" as *const u8 as *const libc::c_char,
             b"(s)\x00" as *const u8 as *const libc::c_char,
             b"d\x00" as *const u8 as *const libc::c_char,
             b"random_artifact_jewelry_chance\x00" as *const u8 as
                 *const libc::c_char, &mut RANDART_JEWEL as *mut s32b);
    call_lua(b"get_module_info\x00" as *const u8 as *const libc::c_char,
             b"(s,d)\x00" as *const u8 as *const libc::c_char,
             b"d\x00" as *const u8 as *const libc::c_char,
             b"version\x00" as *const u8 as *const libc::c_char,
             1 as libc::c_int, &mut VERSION_MAJOR as *mut s32b);
    call_lua(b"get_module_info\x00" as *const u8 as *const libc::c_char,
             b"(s,d)\x00" as *const u8 as *const libc::c_char,
             b"d\x00" as *const u8 as *const libc::c_char,
             b"version\x00" as *const u8 as *const libc::c_char,
             2 as libc::c_int, &mut VERSION_MINOR as *mut s32b);
    call_lua(b"get_module_info\x00" as *const u8 as *const libc::c_char,
             b"(s,d)\x00" as *const u8 as *const libc::c_char,
             b"d\x00" as *const u8 as *const libc::c_char,
             b"version\x00" as *const u8 as *const libc::c_char,
             3 as libc::c_int, &mut VERSION_PATCH as *mut s32b);
    version_major = VERSION_MAJOR as byte_hack;
    version_minor = VERSION_MINOR as byte_hack;
    version_patch = VERSION_PATCH as byte_hack;
    /* Change window name if needed */
    if strcmp(game_module, b"ToME\x00" as *const u8 as *const libc::c_char) !=
           0 {
        strnfmt(angband_term_name[0 as libc::c_int as usize].as_mut_ptr(),
                79 as libc::c_int as uint_hack,
                b"T-Engine: %s\x00" as *const u8 as *const libc::c_char,
                game_module);
        Term_xtra(16 as libc::c_int, 0 as libc::c_int);
    }
    /* Reprocess the player name, just in case */
    process_player_base();
}
/* Did the player force a module on command line */
#[no_mangle]
pub static mut force_module: cptr = 0 as cptr;
/* Display possible modules and select one */
#[no_mangle]
pub unsafe extern "C" fn select_module() -> bool_ {
    let mut k: s32b = 0;
    let mut sel: s32b = 0;
    let mut max: s32b = 0;
    /* Init some lua */
    init_lua();
    /* Some ports need to separate the module scripts from the installed mods,
	   so we need to check for these in two different places */
    if tome_dofile_anywhere(ANGBAND_DIR_CORE,
                            b"mods_aux.lua\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            0 as libc::c_int as bool_) == 0 {
        tome_dofile_anywhere(ANGBAND_DIR_MODULES,
                             b"mods_aux.lua\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             1 as libc::c_int as bool_);
    }
    if tome_dofile_anywhere(ANGBAND_DIR_CORE,
                            b"modules.lua\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            0 as libc::c_int as bool_) == 0 {
        tome_dofile_anywhere(ANGBAND_DIR_MODULES,
                             b"modules.lua\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             1 as libc::c_int as bool_);
    }
    /* Grab the savefiles */
    call_lua(b"max_modules\x00" as *const u8 as *const libc::c_char,
             b"()\x00" as *const u8 as *const libc::c_char,
             b"d\x00" as *const u8 as *const libc::c_char,
             &mut max as *mut s32b);
    /* No need to bother the player if there is only one module */
    sel = -(1 as libc::c_int);
    if !force_module.is_null() {
        call_lua(b"find_module\x00" as *const u8 as *const libc::c_char,
                 b"(s)\x00" as *const u8 as *const libc::c_char,
                 b"d\x00" as *const u8 as *const libc::c_char, force_module,
                 &mut sel as *mut s32b);
    }
    if max == 1 as libc::c_int { sel = 0 as libc::c_int }
    if sel != -(1 as libc::c_int) {
        let mut tmp: cptr = 0 as *const libc::c_char;
        /* Process the module */
        call_lua(b"init_module\x00" as *const u8 as *const libc::c_char,
                 b"(d)\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char, sel);
        call_lua(b"get_module_name\x00" as *const u8 as *const libc::c_char,
                 b"(d)\x00" as *const u8 as *const libc::c_char,
                 b"s\x00" as *const u8 as *const libc::c_char, sel,
                 &mut tmp as *mut cptr);
        game_module = string_make(tmp);
        activate_module();
        return 0 as libc::c_int as bool_
    }
    sel = 0 as libc::c_int;
    /* Preprocess the basic prefs, we need them to have movement keys */
    process_pref_file(b"pref.prf\x00" as *const u8 as *const libc::c_char);
    loop  {
        /* Clear screen */
        Term_clear();
        /* Let the user choose */
        c_put_str(11 as libc::c_int as byte_hack,
                  b"Welcome to ToME, you must select a module to play,\x00" as
                      *const u8 as *const libc::c_char, 1 as libc::c_int,
                  12 as libc::c_int);
        c_put_str(11 as libc::c_int as byte_hack,
                  b"either ToME official module or third party ones.\x00" as
                      *const u8 as *const libc::c_char, 2 as libc::c_int,
                  13 as libc::c_int);
        put_str(b"Press 8/2/4/6 to move, Return to select and Esc to quit.\x00"
                    as *const u8 as *const libc::c_char, 4 as libc::c_int,
                3 as libc::c_int);
        dump_modules(sel, max);
        k = inkey() as s32b;
        if k == '\u{1b}' as i32 { quit(0 as cptr); }
        if k == '6' as i32 {
            sel += 1;
            if sel >= max { sel = 0 as libc::c_int }
        } else if k == '4' as i32 {
            sel -= 1;
            if sel < 0 as libc::c_int { sel = max - 1 as libc::c_int }
        } else if k == '2' as i32 {
            sel += 4 as libc::c_int;
            if sel >= max { sel = sel % max }
        } else if k == '8' as i32 {
            sel -= 4 as libc::c_int;
            if sel < 0 as libc::c_int {
                sel = (sel + max - 1 as libc::c_int) % max
            }
        } else {
            if k == '\r' as i32 {
                if sel < 26 as libc::c_int {
                    k = sel + 'a' as i32
                } else { k = toupper(sel + 'a' as i32) }
            }
            let mut x: libc::c_int = 0;
            let mut tmp_0: cptr = 0 as *const libc::c_char;
            if *(*__ctype_b_loc()).offset(k as isize) as libc::c_int &
                   _ISlower as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 {
                x = k - 'a' as i32
            } else { x = tolower(k) - 'a' as i32 + 26 as libc::c_int }
            if x < 0 as libc::c_int || x >= max { continue ; }
            /* Process the module */
            call_lua(b"init_module\x00" as *const u8 as *const libc::c_char,
                     b"(d)\x00" as *const u8 as *const libc::c_char,
                     b"\x00" as *const u8 as *const libc::c_char, x);
            call_lua(b"get_module_name\x00" as *const u8 as
                         *const libc::c_char,
                     b"(d)\x00" as *const u8 as *const libc::c_char,
                     b"s\x00" as *const u8 as *const libc::c_char, x,
                     &mut tmp_0 as *mut cptr);
            game_module = string_make(tmp_0);
            activate_module();
            return 0 as libc::c_int as bool_
        }
    };
}
