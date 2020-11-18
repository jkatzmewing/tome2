use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut mindcraft_powers: [magic_power; 12];
    #[no_mangle]
    static mut necro_powers: [magic_power; 6];
    #[no_mangle]
    static mut mimic_powers: [magic_power; 5];
    #[no_mangle]
    static mut symbiotic_powers: [magic_power; 9];
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut text_out_indent: libc::c_int;
    #[no_mangle]
    static mut alchemist_recipes: *mut alchemist_recipe;
    #[no_mangle]
    static mut al_name: *mut libc::c_char;
    #[no_mangle]
    static mut a_select_flags: *mut artifact_select_flag;
    #[no_mangle]
    static mut s_info: *mut skill_type;
    #[no_mangle]
    static mut s_name: *mut libc::c_char;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut k_name: *mut libc::c_char;
    #[no_mangle]
    static mut a_info: *mut artifact_type;
    #[no_mangle]
    static mut e_info: *mut ego_item_type;
    #[no_mangle]
    static mut e_name: *mut libc::c_char;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut r_name: *mut libc::c_char;
    #[no_mangle]
    static mut r_text: *mut libc::c_char;
    #[no_mangle]
    static mut ANGBAND_DIR_USER: cptr;
    #[no_mangle]
    static mut max_al_idx: u16b;
    #[no_mangle]
    static mut max_r_idx: u16b;
    #[no_mangle]
    static mut max_k_idx: u16b;
    #[no_mangle]
    static mut max_a_idx: u16b;
    #[no_mangle]
    fn get_version_string() -> *const libc::c_char;
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn ralloc(len: huge_hack) -> vptr;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Term_clear() -> errr;
    #[no_mangle]
    fn Term_save() -> errr;
    #[no_mangle]
    fn Term_load() -> errr;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn necro_info(p: *mut libc::c_char, power: libc::c_int);
    #[no_mangle]
    fn mindcraft_info(p: *mut libc::c_char, power: libc::c_int);
    #[no_mangle]
    fn symbiotic_info(p: *mut libc::c_char, power: libc::c_int);
    #[no_mangle]
    fn mimic_info(p: *mut libc::c_char, power: libc::c_int);
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn object_desc_store(buf: *mut libc::c_char, o_ptr: *mut object_type,
                         pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn object_out_desc(o_ptr: *mut object_type, fff_0: *mut FILE,
                       trim_down: bool_, wait_for_it: bool_) -> bool_;
    #[no_mangle]
    fn object_known(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_aware(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_value(o_ptr: *mut object_type) -> s32b;
    #[no_mangle]
    fn lookup_kind(tval: libc::c_int, sval: libc::c_int) -> s16b;
    #[no_mangle]
    fn object_wipe(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_prep(o_ptr: *mut object_type, k_idx: libc::c_int);
    #[no_mangle]
    static mut hack_apply_magic_power: libc::c_int;
    #[no_mangle]
    fn apply_magic(o_ptr: *mut object_type, lev: libc::c_int, okay: bool_,
                   good: bool_, great: bool_);
    #[no_mangle]
    fn item_activation(o_ptr: *mut object_type, num: byte_hack) -> cptr;
    #[no_mangle]
    fn path_build(buf: *mut libc::c_char, max: libc::c_int, path: cptr,
                  file: cptr) -> errr;
    #[no_mangle]
    fn my_fopen(file: cptr, mode: cptr) -> *mut FILE;
    #[no_mangle]
    fn my_fclose(fff_0: *mut FILE) -> errr;
    #[no_mangle]
    fn bell();
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
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
pub type errr = libc::c_int;
pub type byte_hack = libc::c_uchar;
pub type bool_ = libc::c_char;
pub type huge_hack = libc::c_ulong;
pub type s16b = libc::c_short;
pub type u16b = libc::c_ushort;
pub type s32b = libc::c_int;
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
pub struct object_kind {
    pub name: u32b,
    pub text: u32b,
    pub tval: byte_hack,
    pub sval: byte_hack,
    pub pval: s32b,
    pub pval2: s32b,
    pub to_h: s16b,
    pub to_d: s16b,
    pub to_a: s16b,
    pub activate: s16b,
    pub ac: s16b,
    pub dd: byte_hack,
    pub ds: byte_hack,
    pub weight: s32b,
    pub cost: s32b,
    pub flags1: u32b,
    pub flags2: u32b,
    pub flags3: u32b,
    pub flags4: u32b,
    pub flags5: u32b,
    pub oflags1: u32b,
    pub oflags2: u32b,
    pub oflags3: u32b,
    pub oflags4: u32b,
    pub oflags5: u32b,
    pub locale: [byte_hack; 4],
    pub chance: [byte_hack; 4],
    pub level: byte_hack,
    pub extra: byte_hack,
    pub d_attr: byte_hack,
    pub d_char: libc::c_char,
    pub x_attr: byte_hack,
    pub x_char: libc::c_char,
    pub flavor: byte_hack,
    pub easy_know: bool_,
    pub aware: bool_,
    pub tried: bool_,
    pub know: bool_,
    pub esp: u32b,
    pub oesp: u32b,
    pub btval: byte_hack,
    pub bsval: byte_hack,
    pub artifact: bool_,
    pub power: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct artifact_type {
    pub name: u32b,
    pub text: u32b,
    pub tval: byte_hack,
    pub sval: byte_hack,
    pub pval: s16b,
    pub to_h: s16b,
    pub to_d: s16b,
    pub to_a: s16b,
    pub activate: s16b,
    pub ac: s16b,
    pub dd: byte_hack,
    pub ds: byte_hack,
    pub weight: s16b,
    pub cost: s32b,
    pub flags1: u32b,
    pub flags2: u32b,
    pub flags3: u32b,
    pub flags4: u32b,
    pub flags5: u32b,
    pub oflags1: u32b,
    pub oflags2: u32b,
    pub oflags3: u32b,
    pub oflags4: u32b,
    pub oflags5: u32b,
    pub level: byte_hack,
    pub rarity: byte_hack,
    pub cur_num: byte_hack,
    pub max_num: byte_hack,
    pub esp: u32b,
    pub oesp: u32b,
    pub power: s16b,
    pub set: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ego_item_type {
    pub name: u32b,
    pub text: u32b,
    pub before: bool_,
    pub tval: [byte_hack; 10],
    pub min_sval: [byte_hack; 10],
    pub max_sval: [byte_hack; 10],
    pub rating: byte_hack,
    pub level: byte_hack,
    pub rarity: byte_hack,
    pub mrarity: byte_hack,
    pub max_to_h: s16b,
    pub max_to_d: s16b,
    pub max_to_a: s16b,
    pub activate: s16b,
    pub max_pval: s32b,
    pub cost: s32b,
    pub rar: [byte_hack; 5],
    pub flags1: [u32b; 5],
    pub flags2: [u32b; 5],
    pub flags3: [u32b; 5],
    pub flags4: [u32b; 5],
    pub flags5: [u32b; 5],
    pub esp: [u32b; 5],
    pub oflags1: [u32b; 5],
    pub oflags2: [u32b; 5],
    pub oflags3: [u32b; 5],
    pub oflags4: [u32b; 5],
    pub oflags5: [u32b; 5],
    pub oesp: [u32b; 5],
    pub fego: [u32b; 5],
    pub need_flags1: u32b,
    pub need_flags2: u32b,
    pub need_flags3: u32b,
    pub need_flags4: u32b,
    pub need_flags5: u32b,
    pub need_esp: u32b,
    pub forbid_flags1: u32b,
    pub forbid_flags2: u32b,
    pub forbid_flags3: u32b,
    pub forbid_flags4: u32b,
    pub forbid_flags5: u32b,
    pub forbid_esp: u32b,
    pub power: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_blow {
    pub method: byte_hack,
    pub effect: byte_hack,
    pub d_dice: byte_hack,
    pub d_side: byte_hack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_race {
    pub name: u32b,
    pub text: u32b,
    pub hdice: u16b,
    pub hside: u16b,
    pub ac: s16b,
    pub sleep: s16b,
    pub aaf: byte_hack,
    pub speed: byte_hack,
    pub mexp: s32b,
    pub weight: s32b,
    pub freq_inate: byte_hack,
    pub freq_spell: byte_hack,
    pub flags1: u32b,
    pub flags2: u32b,
    pub flags3: u32b,
    pub flags4: u32b,
    pub flags5: u32b,
    pub flags6: u32b,
    pub flags7: u32b,
    pub flags8: u32b,
    pub flags9: u32b,
    pub blow: [monster_blow; 4],
    pub body_parts: [byte_hack; 6],
    pub level: byte_hack,
    pub rarity: byte_hack,
    pub d_attr: byte_hack,
    pub d_char: libc::c_char,
    pub x_attr: byte_hack,
    pub x_char: libc::c_char,
    pub max_num: s16b,
    pub cur_num: byte_hack,
    pub r_sights: s16b,
    pub r_deaths: s16b,
    pub r_pkills: s16b,
    pub r_tkills: s16b,
    pub r_wake: byte_hack,
    pub r_ignore: byte_hack,
    pub r_xtra1: byte_hack,
    pub r_xtra2: byte_hack,
    pub r_drop_gold: byte_hack,
    pub r_drop_item: byte_hack,
    pub r_cast_inate: byte_hack,
    pub r_cast_spell: byte_hack,
    pub r_blows: [byte_hack; 4],
    pub r_flags1: u32b,
    pub r_flags2: u32b,
    pub r_flags3: u32b,
    pub r_flags4: u32b,
    pub r_flags5: u32b,
    pub r_flags6: u32b,
    pub r_flags7: u32b,
    pub r_flags8: u32b,
    pub r_flags9: u32b,
    pub on_saved: bool_,
    pub total_visible: byte_hack,
    pub drops: obj_theme,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct object_type {
    pub k_idx: s16b,
    pub iy: byte_hack,
    pub ix: byte_hack,
    pub tval: byte_hack,
    pub sval: byte_hack,
    pub pval: s32b,
    pub pval2: s16b,
    pub pval3: s32b,
    pub discount: byte_hack,
    pub number: byte_hack,
    pub weight: s32b,
    pub elevel: byte_hack,
    pub exp: s32b,
    pub name1: byte_hack,
    pub name2: s16b,
    pub name2b: s16b,
    pub xtra1: byte_hack,
    pub xtra2: s16b,
    pub to_h: s16b,
    pub to_d: s16b,
    pub to_a: s16b,
    pub ac: s16b,
    pub dd: byte_hack,
    pub ds: byte_hack,
    pub timeout: s16b,
    pub ident: byte_hack,
    pub marked: byte_hack,
    pub note: u16b,
    pub art_name: u16b,
    pub art_flags1: u32b,
    pub art_flags2: u32b,
    pub art_flags3: u32b,
    pub art_flags4: u32b,
    pub art_flags5: u32b,
    pub art_esp: u32b,
    pub art_oflags1: u32b,
    pub art_oflags2: u32b,
    pub art_oflags3: u32b,
    pub art_oflags4: u32b,
    pub art_oflags5: u32b,
    pub art_oesp: u32b,
    pub next_o_idx: s16b,
    pub held_m_idx: s16b,
    pub sense: byte_hack,
    pub found: byte_hack,
    pub found_aux1: s16b,
    pub found_aux2: s16b,
    pub found_aux3: s16b,
    pub found_aux4: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct magic_power {
    pub min_lev: libc::c_int,
    pub mana_cost: libc::c_int,
    pub fail: libc::c_int,
    pub name: cptr,
    pub desc: cptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alchemist_recipe {
    pub sval_essence: libc::c_int,
    pub tval: byte_hack,
    pub sval: byte_hack,
    pub qty: byte_hack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct artifact_select_flag {
    pub group: byte_hack,
    pub flag: libc::c_int,
    pub level: byte_hack,
    pub desc: libc::c_int,
    pub xp: u32b,
    pub pval: bool_,
    pub item_desc: libc::c_int,
    pub item_descp: libc::c_int,
    pub rtval: byte_hack,
    pub rsval: byte_hack,
    pub rpval: libc::c_int,
    pub rflag: [libc::c_int; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct skill_type {
    pub name: u32b,
    pub desc: u32b,
    pub action_desc: u32b,
    pub action_mkey: s16b,
    pub i_value: s32b,
    pub i_mod: s32b,
    pub value: s32b,
    pub mod_0: s32b,
    pub rate: s16b,
    pub uses: u32b,
    pub action: [s16b; 200],
    pub father: s16b,
    pub dev: bool_,
    pub order: s16b,
    pub hidden: bool_,
    pub random_gain_chance: byte_hack,
    pub flags1: u32b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct grouper {
    pub tval: byte_hack,
    pub name: cptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flag_desc {
    pub flag: u32b,
    pub desc: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pval_info_type {
    pub pval_desc: [libc::c_char; 12],
    pub pval_affects: [cptr; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct obj_desc_list {
    pub description: [libc::c_char; 160],
    pub pval_info: pval_info_type,
    pub slays: [cptr; 10],
    pub brands: [cptr; 10],
    pub immunities: [cptr; 5],
    pub resistances: [cptr; 17],
    pub sustains: [cptr; 6],
    pub misc_magic: [cptr; 19],
    pub activation: cptr,
    pub misc_desc: [libc::c_char; 80],
}
/* File: wizard1.c */
/* Purpose: Spoiler generation -BEN- */
/*
 * The spoiler file being created
 */
static mut fff: *mut FILE = 0 as *const FILE as *mut FILE;
/*
 * Write out `n' of the character `c' to the spoiler file
 */
unsafe extern "C" fn spoiler_out_n_chars(mut n: libc::c_int,
                                         mut c: libc::c_char) {
    loop  {
        n -= 1;
        if !(n >= 0 as libc::c_int) { break ; }
        fputc(c as libc::c_int, fff);
    };
}
/*
 * Write out `n' blank lines to the spoiler file
 */
unsafe extern "C" fn spoiler_blanklines(mut n: libc::c_int) {
    spoiler_out_n_chars(n, '\n' as i32 as libc::c_char);
}
/*
 * Write a line to the spoiler file and then "underline" it with hyphens
 */
unsafe extern "C" fn spoiler_underline(mut str: cptr) {
    fprintf(fff, b"%s\n\x00" as *const u8 as *const libc::c_char, str);
    spoiler_out_n_chars(strlen(str) as libc::c_int,
                        '-' as i32 as libc::c_char);
    fprintf(fff, b"\n\x00" as *const u8 as *const libc::c_char);
}
/*
 * Buffer text to the given file. (-SHAWN-)
 * This is basically c_roff() from mon-desc.c with a few changes.
 */
unsafe extern "C" fn spoil_out(mut str: cptr) {
    let mut r: cptr = 0 as *const libc::c_char;
    /* Line buffer */
    static mut roff_buf: [libc::c_char; 256] = [0; 256];
    /* Current pointer into line roff_buf */
    static mut roff_p: *mut libc::c_char =
        unsafe { roff_buf.as_ptr() as *mut _ };
    /* Last space saved into roff_buf */
    static mut roff_s: *mut libc::c_char =
        0 as *const libc::c_char as *mut libc::c_char;
    /* Special handling for "new sequence" */
    if str.is_null() {
        if roff_p != roff_buf.as_mut_ptr() { roff_p = roff_p.offset(-1) }
        while *roff_p as libc::c_int == ' ' as i32 &&
                  roff_p != roff_buf.as_mut_ptr() {
            roff_p = roff_p.offset(-1)
        }
        if roff_p == roff_buf.as_mut_ptr() {
            fprintf(fff, b"\n\x00" as *const u8 as *const libc::c_char);
        } else {
            *roff_p.offset(1 as libc::c_int as isize) =
                '\u{0}' as i32 as libc::c_char;
            fprintf(fff, b"%s\n\n\x00" as *const u8 as *const libc::c_char,
                    roff_buf.as_mut_ptr());
        }
        roff_p = roff_buf.as_mut_ptr();
        roff_s = 0 as *mut libc::c_char;
        roff_buf[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        return
    }
    /* Scan the given string, character at a time */
    while *str != 0 {
        let mut ch: libc::c_char = *str;
        let mut wrap: libc::c_int =
            (ch as libc::c_int == '\n' as i32) as libc::c_int;
        if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as
               libc::c_int &
               _ISprint as libc::c_int as libc::c_ushort as libc::c_int == 0 {
            ch = ' ' as i32 as libc::c_char
        }
        if roff_p >= roff_buf.as_mut_ptr().offset(75 as libc::c_int as isize)
           {
            wrap = 1 as libc::c_int
        }
        if ch as libc::c_int == ' ' as i32 &&
               roff_p.offset(2 as libc::c_int as isize) >=
                   roff_buf.as_mut_ptr().offset(75 as libc::c_int as isize) {
            wrap = 1 as libc::c_int
        }
        /* Handle line-wrap */
        if wrap != 0 {
            *roff_p = '\u{0}' as i32 as libc::c_char;
            r = roff_p as cptr;
            if !roff_s.is_null() && ch as libc::c_int != ' ' as i32 {
                *roff_s = '\u{0}' as i32 as libc::c_char;
                r = roff_s.offset(1 as libc::c_int as isize) as cptr
            }
            fprintf(fff, b"%s\n\x00" as *const u8 as *const libc::c_char,
                    roff_buf.as_mut_ptr());
            roff_s = 0 as *mut libc::c_char;
            roff_p = roff_buf.as_mut_ptr();
            while *r != 0 {
                let fresh0 = r;
                r = r.offset(1);
                let fresh1 = roff_p;
                roff_p = roff_p.offset(1);
                *fresh1 = *fresh0
            }
        }
        /* Save the char */
        if roff_p > roff_buf.as_mut_ptr() || ch as libc::c_int != ' ' as i32 {
            if ch as libc::c_int == ' ' as i32 { roff_s = roff_p }
            let fresh2 = roff_p;
            roff_p = roff_p.offset(1);
            *fresh2 = ch
        }
        str = str.offset(1)
    };
}
/*
 * Extract a textual representation of an attribute
 */
unsafe extern "C" fn attr_to_text(mut a: byte_hack) -> cptr {
    match a as libc::c_int {
        0 => { return b"xxx\x00" as *const u8 as *const libc::c_char }
        1 => { return b"White\x00" as *const u8 as *const libc::c_char }
        2 => { return b"Slate\x00" as *const u8 as *const libc::c_char }
        3 => { return b"Orange\x00" as *const u8 as *const libc::c_char }
        4 => { return b"Red\x00" as *const u8 as *const libc::c_char }
        5 => { return b"Green\x00" as *const u8 as *const libc::c_char }
        6 => { return b"Blue\x00" as *const u8 as *const libc::c_char }
        7 => { return b"Umber\x00" as *const u8 as *const libc::c_char }
        8 => { return b"L.Dark\x00" as *const u8 as *const libc::c_char }
        9 => { return b"L.Slate\x00" as *const u8 as *const libc::c_char }
        10 => { return b"Violet\x00" as *const u8 as *const libc::c_char }
        11 => { return b"Yellow\x00" as *const u8 as *const libc::c_char }
        12 => { return b"L.Red\x00" as *const u8 as *const libc::c_char }
        13 => { return b"L.Green\x00" as *const u8 as *const libc::c_char }
        14 => { return b"L.Blue\x00" as *const u8 as *const libc::c_char }
        15 => { return b"L.Umber\x00" as *const u8 as *const libc::c_char }
        _ => { }
    }
    /* Oops */
    return b"Icky\x00" as *const u8 as *const libc::c_char;
}
/*
 * Item Spoilers by: benh@phial.com (Ben Harrison)
 */
/*
 * The basic items categorized by type
 */
static mut group_item: [grouper; 49] =
    [{
         let mut init =
             grouper{tval: 23 as libc::c_int as byte_hack,
                     name:
                         b"Melee Weapons\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 22 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 21 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 24 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 6 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 19 as libc::c_int as byte_hack,
                     name:
                         b"Bows and Slings\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 16 as libc::c_int as byte_hack,
                     name: b"Ammo\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 17 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 18 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 15 as libc::c_int as byte_hack,
                     name:
                         b"Boomerangs\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 14 as libc::c_int as byte_hack,
                     name:
                         b"Instruments\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 36 as libc::c_int as byte_hack,
                     name:
                         b"Armour (Body)\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 37 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 38 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 34 as libc::c_int as byte_hack,
                     name:
                         b"Armour (Misc)\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 32 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 33 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 31 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 30 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 35 as libc::c_int as byte_hack,
                     name:
                         b"Cloaks\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 40 as libc::c_int as byte_hack,
                     name:
                         b"Amulets\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 45 as libc::c_int as byte_hack,
                     name: b"Rings\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 70 as libc::c_int as byte_hack,
                     name:
                         b"Scrolls\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 71 as libc::c_int as byte_hack,
                     name:
                         b"Potions\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 72 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 80 as libc::c_int as byte_hack,
                     name: b"Food\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 67 as libc::c_int as byte_hack,
                     name: b"Rods\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 66 as libc::c_int as byte_hack,
                     name:
                         b"Rod Tips\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 65 as libc::c_int as byte_hack,
                     name: b"Wands\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 55 as libc::c_int as byte_hack,
                     name:
                         b"Staves\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 111 as libc::c_int as byte_hack,
                     name:
                         b"Books (Magic, Gods, Music)\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 115 as libc::c_int as byte_hack,
                     name:
                         b"Demonic Equipment\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 104 as libc::c_int as byte_hack,
                     name: b"Runes\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 105 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 4 as libc::c_int as byte_hack,
                     name:
                         b"Essences\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 8 as libc::c_int as byte_hack,
                     name:
                         b"Parchments\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 20 as libc::c_int as byte_hack,
                     name: b"Tools\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 12 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 46 as libc::c_int as byte_hack,
                     name:
                         b"Trapping Kits\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 7 as libc::c_int as byte_hack,
                     name:
                         b"Chests\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 5 as libc::c_int as byte_hack,
                     name:
                         b"Various\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 39 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 77 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 2 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 11 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 1 as libc::c_int as byte_hack,
                     name:
                         b"Corpses and Eggs\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 9 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 10 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 0 as libc::c_int as byte_hack,
                     name: b"\x00" as *const u8 as *const libc::c_char,};
         init
     }];
/*
 * Describe the kind
 */
unsafe extern "C" fn kind_info(mut buf: *mut libc::c_char,
                               mut dam: *mut libc::c_char,
                               mut wgt: *mut libc::c_char,
                               mut lev: *mut libc::c_int, mut val: *mut s32b,
                               mut k: libc::c_int) {
    let mut forge: object_type =
        object_type{k_idx: 0,
                    iy: 0,
                    ix: 0,
                    tval: 0,
                    sval: 0,
                    pval: 0,
                    pval2: 0,
                    pval3: 0,
                    discount: 0,
                    number: 0,
                    weight: 0,
                    elevel: 0,
                    exp: 0,
                    name1: 0,
                    name2: 0,
                    name2b: 0,
                    xtra1: 0,
                    xtra2: 0,
                    to_h: 0,
                    to_d: 0,
                    to_a: 0,
                    ac: 0,
                    dd: 0,
                    ds: 0,
                    timeout: 0,
                    ident: 0,
                    marked: 0,
                    note: 0,
                    art_name: 0,
                    art_flags1: 0,
                    art_flags2: 0,
                    art_flags3: 0,
                    art_flags4: 0,
                    art_flags5: 0,
                    art_esp: 0,
                    art_oflags1: 0,
                    art_oflags2: 0,
                    art_oflags3: 0,
                    art_oflags4: 0,
                    art_oflags5: 0,
                    art_oesp: 0,
                    next_o_idx: 0,
                    held_m_idx: 0,
                    sense: 0,
                    found: 0,
                    found_aux1: 0,
                    found_aux2: 0,
                    found_aux3: 0,
                    found_aux4: 0,};
    let mut q_ptr: *mut object_type = 0 as *mut object_type;
    let mut k_ptr: *mut object_kind = 0 as *mut object_kind;
    /* Get local object */
    q_ptr = &mut forge;
    /* Prepare a fake item */
    object_prep(q_ptr, k);
    /* Obtain the "kind" info */
    k_ptr = &mut *k_info.offset((*q_ptr).k_idx as isize) as *mut object_kind;
    /* It is known */
    (*q_ptr).ident =
        ((*q_ptr).ident as libc::c_int | 0x8 as libc::c_int) as byte_hack;
    /* Cancel bonuses */
    (*q_ptr).to_a = 0 as libc::c_int as s16b;
    (*q_ptr).to_h = 0 as libc::c_int as s16b;
    (*q_ptr).to_d = 0 as libc::c_int as s16b;
    if (*k_ptr).tval as libc::c_int == 65 as libc::c_int ||
           (*k_ptr).tval as libc::c_int == 55 as libc::c_int {
        hack_apply_magic_power = -(99 as libc::c_int);
        apply_magic(q_ptr, 0 as libc::c_int, 0 as libc::c_int as bool_,
                    0 as libc::c_int as bool_, 0 as libc::c_int as bool_);
    }
    /* Level */
    *lev = (*k_ptr).level as libc::c_int;
    /* Value */
    *val = object_value(q_ptr);
    /* Hack */
    if buf.is_null() || dam.is_null() || wgt.is_null() { return }
    /* Description (too brief) */
    object_desc_store(buf, q_ptr, 0 as libc::c_int, 0 as libc::c_int);
    /* Misc info */
    strcpy(dam, b"\x00" as *const u8 as *const libc::c_char);
    let mut current_block_18: u64;
    /* Damage */
    match (*q_ptr).tval as libc::c_int {
        16 | 18 | 17 => {
            /* Boomerangs */
            current_block_18 = 138501003661741223;
        }
        15 => { current_block_18 = 138501003661741223; }
        21 | 22 | 23 | 24 | 6 | 20 => {
            current_block_18 = 6009453772311597924;
        }
        30 | 31 | 35 | 33 | 32 | 34 | 36 | 37 | 38 => {
            /* Armour */
            sprintf(dam, b"%d\x00" as *const u8 as *const libc::c_char,
                    (*q_ptr).ac as libc::c_int);
            current_block_18 = 16203760046146113240;
        }
        19 | _ => { current_block_18 = 16203760046146113240; }
    }
    match current_block_18 {
        138501003661741223 =>
        /* Weapons */
        {
            current_block_18 = 6009453772311597924;
        }
        _ => { }
    }
    match current_block_18 {
        6009453772311597924 =>
        /* Tools */
        {
            sprintf(dam, b"%dd%d\x00" as *const u8 as *const libc::c_char,
                    (*q_ptr).dd as libc::c_int, (*q_ptr).ds as libc::c_int);
        }
        _ => { }
    }
    /* Weight */
    sprintf(wgt, b"%3ld.%ld\x00" as *const u8 as *const libc::c_char,
            ((*q_ptr).weight / 10 as libc::c_int) as libc::c_long,
            ((*q_ptr).weight % 10 as libc::c_int) as libc::c_long);
}
/*
 * Create a spoiler file for items
 */
unsafe extern "C" fn spoil_obj_desc(mut fname: cptr) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut who: [u16b; 200] = [0; 200];
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut wgt: [libc::c_char; 80] = [0; 80];
    let mut dam: [libc::c_char; 80] = [0; 80];
    /* Build the filename */
    path_build(buf.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int, ANGBAND_DIR_USER, fname);
    /* File type is "TEXT" */
    /* Open the file */
    fff =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    /* Oops */
    if fff.is_null() {
        msg_print(b"Cannot create spoiler file.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Header */
    sprintf(buf.as_mut_ptr(),
            b"Basic Items Spoilers for %s\x00" as *const u8 as
                *const libc::c_char, get_version_string());
    spoiler_underline(buf.as_mut_ptr() as cptr);
    spoiler_blanklines(2 as libc::c_int);
    /* More Header */
    fprintf(fff,
            b"%-45s     %8s%7s%5s%9s\n\x00" as *const u8 as
                *const libc::c_char,
            b"Description\x00" as *const u8 as *const libc::c_char,
            b"Dam/AC\x00" as *const u8 as *const libc::c_char,
            b"Wgt\x00" as *const u8 as *const libc::c_char,
            b"Lev\x00" as *const u8 as *const libc::c_char,
            b"Cost\x00" as *const u8 as *const libc::c_char);
    fprintf(fff,
            b"%-45s     %8s%7s%5s%9s\n\x00" as *const u8 as
                *const libc::c_char,
            b"----------------------------------------\x00" as *const u8 as
                *const libc::c_char,
            b"------\x00" as *const u8 as *const libc::c_char,
            b"---\x00" as *const u8 as *const libc::c_char,
            b"---\x00" as *const u8 as *const libc::c_char,
            b"----\x00" as *const u8 as *const libc::c_char);
    /* List the groups */
    i = 0 as libc::c_int;
    loop 
         /* Write out the group title */
         {
        if !group_item[i as usize].name.is_null() {
            /* Hack -- bubble-sort by cost and then level */
            s = 0 as libc::c_int;
            while s < n - 1 as libc::c_int {
                t = 0 as libc::c_int;
                while t < n - 1 as libc::c_int {
                    let mut i1: libc::c_int = t;
                    let mut i2: libc::c_int = t + 1 as libc::c_int;
                    let mut e1: libc::c_int = 0;
                    let mut e2: libc::c_int = 0;
                    let mut t1: s32b = 0;
                    let mut t2: s32b = 0;
                    kind_info(0 as *mut libc::c_char, 0 as *mut libc::c_char,
                              0 as *mut libc::c_char, &mut e1, &mut t1,
                              who[i1 as usize] as libc::c_int);
                    kind_info(0 as *mut libc::c_char, 0 as *mut libc::c_char,
                              0 as *mut libc::c_char, &mut e2, &mut t2,
                              who[i2 as usize] as libc::c_int);
                    if t1 > t2 || t1 == t2 && e1 > e2 {
                        let mut tmp: libc::c_int =
                            who[i1 as usize] as libc::c_int;
                        who[i1 as usize] = who[i2 as usize];
                        who[i2 as usize] = tmp as u16b
                    }
                    t += 1
                }
                s += 1
            }
            /* Spoil each item */
            s = 0 as libc::c_int;
            while s < n {
                let mut e: libc::c_int = 0;
                let mut v: s32b = 0;
                /* Describe the kind */
                kind_info(buf.as_mut_ptr(), dam.as_mut_ptr(),
                          wgt.as_mut_ptr(), &mut e, &mut v,
                          who[s as usize] as libc::c_int);
                /* Dump it */
                fprintf(fff,
                        b"     %-45s%8s%7s%5d%9ld\n\x00" as *const u8 as
                            *const libc::c_char, buf.as_mut_ptr(),
                        dam.as_mut_ptr(), wgt.as_mut_ptr(), e,
                        v as libc::c_long);
                s += 1
            }
            /* Start a new set */
            n = 0 as libc::c_int;
            /* Notice the end */
            if group_item[i as usize].tval == 0 { break ; }
            /* Start a new set */
            fprintf(fff,
                    b"\n\n%s\n\n\x00" as *const u8 as *const libc::c_char,
                    group_item[i as usize].name);
        }
        /* Acquire legal item types */
        k = 1 as libc::c_int;
        while k < max_k_idx as libc::c_int {
            let mut k_ptr: *mut object_kind =
                &mut *k_info.offset(k as isize) as *mut object_kind;
            /* Skip wrong tval's */
            if !((*k_ptr).tval as libc::c_int !=
                     group_item[i as usize].tval as libc::c_int) {
                /* Hack -- Skip artifacts */
                if !((*k_ptr).flags3 as libc::c_long &
                         (0x800 as libc::c_long | 0x8000 as libc::c_long) !=
                         0) {
                    /* Hack -- Skip Ring of Powers */
                    if !(k == 785 as libc::c_int) {
                        /* Save the index */
                        let fresh3 = n;
                        n = n + 1;
                        who[fresh3 as usize] = k as u16b
                    }
                }
            }
            k += 1
        }
        i += 1
    }
    /* Check for errors */
    if ferror(fff) != 0 || my_fclose(fff) != 0 {
        msg_print(b"Cannot close spoiler file.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Message */
    msg_print(b"Successfully created a spoiler file.\x00" as *const u8 as
                  *const libc::c_char);
}
/*
 * The artifacts categorized by type
 */
static mut group_artifact: [grouper; 28] =
    [{
         let mut init =
             grouper{tval: 23 as libc::c_int as byte_hack,
                     name:
                         b"Edged Weapons\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 22 as libc::c_int as byte_hack,
                     name:
                         b"Polearms\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 21 as libc::c_int as byte_hack,
                     name:
                         b"Hafted Weapons\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 24 as libc::c_int as byte_hack,
                     name: b"Axes\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 6 as libc::c_int as byte_hack,
                     name:
                         b"Mage Staffs\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 19 as libc::c_int as byte_hack,
                     name: b"Bows\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 16 as libc::c_int as byte_hack,
                     name: b"Ammo\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 17 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 18 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 15 as libc::c_int as byte_hack,
                     name:
                         b"Boomerangs\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 14 as libc::c_int as byte_hack,
                     name:
                         b"Instruments\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 36 as libc::c_int as byte_hack,
                     name:
                         b"Body Armor\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 37 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 38 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 35 as libc::c_int as byte_hack,
                     name:
                         b"Cloaks\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 34 as libc::c_int as byte_hack,
                     name:
                         b"Shields\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 32 as libc::c_int as byte_hack,
                     name:
                         b"Helms/Crowns\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 33 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 31 as libc::c_int as byte_hack,
                     name:
                         b"Gloves\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 30 as libc::c_int as byte_hack,
                     name: b"Boots\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 115 as libc::c_int as byte_hack,
                     name:
                         b"Demonic Equipment\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 39 as libc::c_int as byte_hack,
                     name:
                         b"Light Sources\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 40 as libc::c_int as byte_hack,
                     name:
                         b"Amulets\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 45 as libc::c_int as byte_hack,
                     name: b"Rings\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 12 as libc::c_int as byte_hack,
                     name: b"Tools\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 20 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     },
     {
         let mut init =
             grouper{tval: 46 as libc::c_int as byte_hack,
                     name:
                         b"Trapping Kits\x00" as *const u8 as
                             *const libc::c_char,};
         init
     },
     {
         let mut init =
             grouper{tval: 0 as libc::c_int as byte_hack, name: 0 as cptr,};
         init
     }];
/*
 * These are used for "+3 to STR, DEX", etc. These are separate from
 * the other pval affected traits to simplify the case where an object
 * affects all stats.  In this case, "All stats" is used instead of
 * listing each stat individually.
 */
static mut stat_flags_desc: [flag_desc; 6] =
    [{
         let mut init =
             flag_desc{flag: 0x1 as libc::c_long as u32b,
                       desc: b"STR\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x2 as libc::c_long as u32b,
                       desc: b"INT\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x4 as libc::c_long as u32b,
                       desc: b"WIS\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x8 as libc::c_long as u32b,
                       desc: b"DEX\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x10 as libc::c_long as u32b,
                       desc: b"CON\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x20 as libc::c_long as u32b,
                       desc: b"CHR\x00" as *const u8 as *const libc::c_char,};
         init
     }];
/*
 * Besides stats, these are the other player traits
 * which may be affected by an object's pval
 */
static mut pval_flags1_desc: [flag_desc; 6] =
    [{
         let mut init =
             flag_desc{flag: 0x100 as libc::c_long as u32b,
                       desc:
                           b"Stealth\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x200 as libc::c_long as u32b,
                       desc:
                           b"Searching\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x400 as libc::c_long as u32b,
                       desc:
                           b"Infravision\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x800 as libc::c_long as u32b,
                       desc:
                           b"Tunnelling\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x2000 as libc::c_long as u32b,
                       desc:
                           b"Attacks\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x1000 as libc::c_long as u32b,
                       desc:
                           b"Speed\x00" as *const u8 as *const libc::c_char,};
         init
     }];
/*
 * Slaying preferences for weapons
 */
static mut slay_flags_desc: [flag_desc; 9] =
    [{
         let mut init =
             flag_desc{flag: 0x10000 as libc::c_long as u32b,
                       desc:
                           b"Animal\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x20000 as libc::c_long as u32b,
                       desc:
                           b"Evil\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x40000 as libc::c_long as u32b,
                       desc:
                           b"Undead\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x80000 as libc::c_long as u32b,
                       desc:
                           b"Demon\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x100000 as libc::c_long as u32b,
                       desc: b"Orc\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x200000 as libc::c_long as u32b,
                       desc:
                           b"Troll\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x400000 as libc::c_long as u32b,
                       desc:
                           b"Giant\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x800000 as libc::c_long as u32b,
                       desc:
                           b"Dragon\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x1000000 as libc::c_long as u32b,
                       desc:
                           b"Xdragon\x00" as *const u8 as
                               *const libc::c_char,};
         init
     }];
/*
 * Elemental brands for weapons
 *
 * Clearly, TR1_IMPACT is a bit out of place here. To simplify
 * coding, it has been included here along with the elemental
 * brands. It does seem to fit in with the brands and slaying
 * more than the miscellaneous section.
 */
static mut brand_flags_desc: [flag_desc; 9] =
    [{
         let mut init =
             flag_desc{flag: 0x10000000 as libc::c_long as u32b,
                       desc:
                           b"Acid Brand\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x20000000 as libc::c_long as u32b,
                       desc:
                           b"Lightning Brand\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x40000000 as libc::c_long as u32b,
                       desc:
                           b"Flame Tongue\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x80000000 as libc::c_long as u32b,
                       desc:
                           b"Frost Brand\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x8000000 as libc::c_long as u32b,
                       desc:
                           b"Poisoned\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x4000 as libc::c_long as u32b,
                       desc:
                           b"Mark of Chaos\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x8000 as libc::c_long as u32b,
                       desc:
                           b"Vampiric\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x4000000 as libc::c_long as u32b,
                       desc:
                           b"Earthquake impact on hit\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x2000000 as libc::c_long as u32b,
                       desc:
                           b"Very sharp\x00" as *const u8 as
                               *const libc::c_char,};
         init
     }];
/*
 * The 15 resistables
 */
static mut resist_flags_desc: [flag_desc; 16] =
    [{
         let mut init =
             flag_desc{flag: 0x10000 as libc::c_long as u32b,
                       desc:
                           b"Acid\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x20000 as libc::c_long as u32b,
                       desc:
                           b"Lightning\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x40000 as libc::c_long as u32b,
                       desc:
                           b"Fire\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x80000 as libc::c_long as u32b,
                       desc:
                           b"Cold\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x100000 as libc::c_long as u32b,
                       desc:
                           b"Poison\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x200000 as libc::c_long as u32b,
                       desc:
                           b"Fear\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x400000 as libc::c_long as u32b,
                       desc:
                           b"Light\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x800000 as libc::c_long as u32b,
                       desc:
                           b"Dark\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x1000000 as libc::c_long as u32b,
                       desc:
                           b"Blindness\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x2000000 as libc::c_long as u32b,
                       desc:
                           b"Confusion\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x4000000 as libc::c_long as u32b,
                       desc:
                           b"Sound\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x8000000 as libc::c_long as u32b,
                       desc:
                           b"Shards\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x10000000 as libc::c_long as u32b,
                       desc:
                           b"Nether\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x20000000 as libc::c_long as u32b,
                       desc:
                           b"Nexus\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x40000000 as libc::c_long as u32b,
                       desc:
                           b"Chaos\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x80000000 as libc::c_long as u32b,
                       desc:
                           b"Disenchantment\x00" as *const u8 as
                               *const libc::c_char,};
         init
     }];
/*
 * Elemental immunities (along with poison)
 */
static mut immune_flags_desc: [flag_desc; 4] =
    [{
         let mut init =
             flag_desc{flag: 0x100 as libc::c_long as u32b,
                       desc:
                           b"Acid\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x200 as libc::c_long as u32b,
                       desc:
                           b"Lightning\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x400 as libc::c_long as u32b,
                       desc:
                           b"Fire\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x800 as libc::c_long as u32b,
                       desc:
                           b"Cold\x00" as *const u8 as *const libc::c_char,};
         init
     }];
/*
 * Sustain stats -  these are given their "own" line in the
 * spoiler file, mainly for simplicity
 */
static mut sustain_flags_desc: [flag_desc; 6] =
    [{
         let mut init =
             flag_desc{flag: 0x1 as libc::c_long as u32b,
                       desc: b"STR\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x2 as libc::c_long as u32b,
                       desc: b"INT\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x4 as libc::c_long as u32b,
                       desc: b"WIS\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x8 as libc::c_long as u32b,
                       desc: b"DEX\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x10 as libc::c_long as u32b,
                       desc: b"CON\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x20 as libc::c_long as u32b,
                       desc: b"CHR\x00" as *const u8 as *const libc::c_char,};
         init
     }];
/*
 * Miscellaneous magic given by an object's "flags2" field
 */
static mut misc_flags2_desc: [flag_desc; 3] =
    [{
         let mut init =
             flag_desc{flag: 0x2000 as libc::c_long as u32b,
                       desc:
                           b"Reflection\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x4000 as libc::c_long as u32b,
                       desc:
                           b"Free Action\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x8000 as libc::c_long as u32b,
                       desc:
                           b"Hold Life\x00" as *const u8 as
                               *const libc::c_char,};
         init
     }];
/*
 * Miscellaneous magic given by an object's "flags3" field
 *
 * Note that cursed artifacts and objects with permanent light
 * are handled "directly" -- see analyze_misc_magic()
 */
static mut misc_flags3_desc: [flag_desc; 13] =
    [{
         let mut init =
             flag_desc{flag: 0x1 as libc::c_long as u32b,
                       desc:
                           b"Fiery Aura\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x2 as libc::c_long as u32b,
                       desc:
                           b"Electric Aura\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x10 as libc::c_long as u32b,
                       desc:
                           b"Prevent Teleportation\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x20 as libc::c_long as u32b,
                       desc:
                           b"Anti-Magic\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x40 as libc::c_long as u32b,
                       desc:
                           b"Wraith Form\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x1000 as libc::c_long as u32b,
                       desc:
                           b"Levitation\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x4000 as libc::c_long as u32b,
                       desc:
                           b"See Invisible\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x10000 as libc::c_long as u32b,
                       desc:
                           b"Slow Digestion\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x20000 as libc::c_long as u32b,
                       desc:
                           b"Regeneration\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x80000 as libc::c_long as u32b,
                       desc:
                           b"+1 Extra Shot\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x2000000 as libc::c_long as u32b,
                       desc:
                           b"Drains Experience\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x8000000 as libc::c_long as u32b,
                       desc:
                           b"Aggravates\x00" as *const u8 as
                               *const libc::c_char,};
         init
     },
     {
         let mut init =
             flag_desc{flag: 0x10000000 as libc::c_long as u32b,
                       desc:
                           b"Blessed Blade\x00" as *const u8 as
                               *const libc::c_char,};
         init
     }];
/*
 * This function does most of the actual "analysis". Given a set of bit flags
 * (which will be from one of the flags fields from the object in question),
 * a "flag description structure", a "description list", and the number of
 * elements in the "flag description structure", this function sets the
 * "description list" members to the appropriate descriptions contained in
 * the "flag description structure".
 *
 * The possibly updated description pointer is returned.
 */
unsafe extern "C" fn spoiler_flag_aux(art_flags: u32b,
                                      mut flag_ptr: *const flag_desc,
                                      mut desc_ptr: *mut cptr,
                                      n_elmnts: libc::c_int) -> *mut cptr {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n_elmnts {
        if art_flags & (*flag_ptr.offset(i as isize)).flag != 0 {
            let fresh4 = desc_ptr;
            desc_ptr = desc_ptr.offset(1);
            *fresh4 = (*flag_ptr.offset(i as isize)).desc
        }
        i += 1
    }
    return desc_ptr;
}
/*
 * Acquire a "basic" description "The Cloak of Death [1,+10]"
 */
unsafe extern "C" fn analyze_general(mut o_ptr: *mut object_type,
                                     mut desc_ptr: *mut libc::c_char) {
    /* Get a "useful" description of the object */
    object_desc_store(desc_ptr, o_ptr, 1 as libc::c_int, 1 as libc::c_int);
}
/*
 * List "player traits" altered by an artifact's pval. These include stats,
 * speed, infravision, tunnelling, stealth, searching, and extra attacks.
 */
unsafe extern "C" fn analyze_pval(mut o_ptr: *mut object_type,
                                  mut p_ptr: *mut pval_info_type) {
    let all_stats: u32b =
        (0x1 as libc::c_long | 0x2 as libc::c_long | 0x4 as libc::c_long |
             0x8 as libc::c_long | 0x10 as libc::c_long |
             0x20 as libc::c_long) as u32b;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut affects_list: *mut cptr = 0 as *mut cptr;
    /* If pval == 0, there is nothing to do. */
    if (*o_ptr).pval == 0 {
        /* An "empty" pval description indicates that pval == 0 */
        (*p_ptr).pval_desc[0 as libc::c_int as usize] =
            '\u{0}' as i32 as libc::c_char;
        return
    }
    /* Extract the flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    affects_list = (*p_ptr).pval_affects.as_mut_ptr();
    /* Create the "+N" string */
    sprintf((*p_ptr).pval_desc.as_mut_ptr(),
            b"%s%ld\x00" as *const u8 as *const libc::c_char,
            if (*o_ptr).pval >= 0 as libc::c_int {
                b"+\x00" as *const u8 as *const libc::c_char
            } else { b"\x00" as *const u8 as *const libc::c_char },
            (*o_ptr).pval as libc::c_long);
    /* First, check to see if the pval affects all stats */
    if f1 & all_stats == all_stats {
        let fresh5 = affects_list;
        affects_list = affects_list.offset(1);
        *fresh5 = b"All stats\x00" as *const u8 as *const libc::c_char
    } else if f1 & all_stats != 0 {
        affects_list =
            spoiler_flag_aux(f1, stat_flags_desc.as_mut_ptr(), affects_list,
                             (::std::mem::size_of::<[flag_desc; 6]>() as
                                  libc::c_ulong).wrapping_div(::std::mem::size_of::<flag_desc>()
                                                                  as
                                                                  libc::c_ulong)
                                 as libc::c_int)
    }
    /* Are any stats affected? */
    /* And now the "rest" */
    affects_list =
        spoiler_flag_aux(f1, pval_flags1_desc.as_mut_ptr(), affects_list,
                         (::std::mem::size_of::<[flag_desc; 6]>() as
                              libc::c_ulong).wrapping_div(::std::mem::size_of::<flag_desc>()
                                                              as
                                                              libc::c_ulong)
                             as libc::c_int);
    /* Terminate the description list */
    *affects_list = 0 as cptr;
}
/* Note the slaying specialties of a weapon */
unsafe extern "C" fn analyze_slay(mut o_ptr: *mut object_type,
                                  mut slay_list: *mut cptr) {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    slay_list =
        spoiler_flag_aux(f1, slay_flags_desc.as_mut_ptr(), slay_list,
                         (::std::mem::size_of::<[flag_desc; 9]>() as
                              libc::c_ulong).wrapping_div(::std::mem::size_of::<flag_desc>()
                                                              as
                                                              libc::c_ulong)
                             as libc::c_int);
    /* Terminate the description list */
    *slay_list = 0 as cptr;
}
/* Note an object's elemental brands */
unsafe extern "C" fn analyze_brand(mut o_ptr: *mut object_type,
                                   mut brand_list: *mut cptr) {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    brand_list =
        spoiler_flag_aux(f1, brand_flags_desc.as_mut_ptr(), brand_list,
                         (::std::mem::size_of::<[flag_desc; 9]>() as
                              libc::c_ulong).wrapping_div(::std::mem::size_of::<flag_desc>()
                                                              as
                                                              libc::c_ulong)
                             as libc::c_int);
    /* Terminate the description list */
    *brand_list = 0 as cptr;
}
/* Note the resistances granted by an object */
unsafe extern "C" fn analyze_resist(mut o_ptr: *mut object_type,
                                    mut resist_list: *mut cptr) {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    resist_list =
        spoiler_flag_aux(f2, resist_flags_desc.as_ptr(), resist_list,
                         (::std::mem::size_of::<[flag_desc; 16]>() as
                              libc::c_ulong).wrapping_div(::std::mem::size_of::<flag_desc>()
                                                              as
                                                              libc::c_ulong)
                             as libc::c_int);
    /* Terminate the description list */
    *resist_list = 0 as cptr;
}
/* Note the immunities granted by an object */
unsafe extern "C" fn analyze_immune(mut o_ptr: *mut object_type,
                                    mut immune_list: *mut cptr) {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    immune_list =
        spoiler_flag_aux(f2, immune_flags_desc.as_ptr(), immune_list,
                         (::std::mem::size_of::<[flag_desc; 4]>() as
                              libc::c_ulong).wrapping_div(::std::mem::size_of::<flag_desc>()
                                                              as
                                                              libc::c_ulong)
                             as libc::c_int);
    /* Terminate the description list */
    *immune_list = 0 as cptr;
}
/* Note which stats an object sustains */
unsafe extern "C" fn analyze_sustains(mut o_ptr: *mut object_type,
                                      mut sustain_list: *mut cptr) {
    let all_sustains: u32b =
        (0x1 as libc::c_long | 0x2 as libc::c_long | 0x4 as libc::c_long |
             0x8 as libc::c_long | 0x10 as libc::c_long |
             0x20 as libc::c_long) as u32b;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Simplify things if an item sustains all stats */
    if f2 & all_sustains == all_sustains {
        let fresh6 = sustain_list;
        sustain_list = sustain_list.offset(1);
        *fresh6 = b"All stats\x00" as *const u8 as *const libc::c_char
    } else if f2 & all_sustains != 0 {
        sustain_list =
            spoiler_flag_aux(f2, sustain_flags_desc.as_ptr(), sustain_list,
                             (::std::mem::size_of::<[flag_desc; 6]>() as
                                  libc::c_ulong).wrapping_div(::std::mem::size_of::<flag_desc>()
                                                                  as
                                                                  libc::c_ulong)
                                 as libc::c_int)
    }
    /* Should we bother? */
    /* Terminate the description list */
    *sustain_list = 0 as cptr;
}
/*
 * Note miscellaneous powers bestowed by an artifact such as see invisible,
 * free action, permanent light, etc.
 */
unsafe extern "C" fn analyze_misc_magic(mut o_ptr: *mut object_type,
                                        mut misc_list: *mut cptr) {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut radius: libc::c_int = 0 as libc::c_int;
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    misc_list =
        spoiler_flag_aux(f2, misc_flags2_desc.as_ptr(), misc_list,
                         (::std::mem::size_of::<[flag_desc; 3]>() as
                              libc::c_ulong).wrapping_div(::std::mem::size_of::<flag_desc>()
                                                              as
                                                              libc::c_ulong)
                             as libc::c_int);
    misc_list =
        spoiler_flag_aux(f3, misc_flags3_desc.as_ptr(), misc_list,
                         (::std::mem::size_of::<[flag_desc; 13]>() as
                              libc::c_ulong).wrapping_div(::std::mem::size_of::<flag_desc>()
                                                              as
                                                              libc::c_ulong)
                             as libc::c_int);
    /*
	 * Glowing artifacts -- small radius light.
	 */
    if f3 as libc::c_long & 0x2000 as libc::c_long != 0 { radius += 1 }
    if f4 as libc::c_long & 0x4000000 as libc::c_long != 0 {
        radius += 2 as libc::c_int
    }
    if f4 as libc::c_long & 0x8000000 as libc::c_long != 0 {
        radius += 3 as libc::c_int
    }
    if f4 as libc::c_long & 0x10000000 as libc::c_long != 0 {
        let fresh7 = misc_list;
        misc_list = misc_list.offset(1);
        *fresh7 =
            format(b"It provides light (radius %d) forever.\x00" as *const u8
                       as *const libc::c_char, radius) as cptr
    } else {
        let fresh8 = misc_list;
        misc_list = misc_list.offset(1);
        *fresh8 =
            format(b"It provides light (radius %d) when fueled.\x00" as
                       *const u8 as *const libc::c_char, radius) as cptr
    }
    /*
	 * Handle cursed objects here to avoid redundancies such as noting
	 * that a permanently cursed object is heavily cursed as well as
	 * being "lightly cursed".
	 */
    if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
        if f3 as libc::c_long & 0x80 as libc::c_long != 0 {
            let fresh9 = misc_list;
            misc_list = misc_list.offset(1);
            *fresh9 = b"Ancient Curse\x00" as *const u8 as *const libc::c_char
        }
        if f3 as libc::c_long & 0x80000000 as libc::c_long != 0 {
            let fresh10 = misc_list;
            misc_list = misc_list.offset(1);
            *fresh10 =
                b"Permanently Cursed\x00" as *const u8 as *const libc::c_char
        } else if f3 as libc::c_long & 0x40000000 as libc::c_long != 0 {
            let fresh11 = misc_list;
            misc_list = misc_list.offset(1);
            *fresh11 =
                b"Heavily Cursed\x00" as *const u8 as *const libc::c_char
        } else {
            let fresh12 = misc_list;
            misc_list = misc_list.offset(1);
            *fresh12 = b"Cursed\x00" as *const u8 as *const libc::c_char
        }
    }
    /* Terminate the description list */
    *misc_list = 0 as cptr;
}
/*
 * Determine the minimum depth an artifact can appear, its rarity, its weight,
 * and its value in gold pieces
 */
unsafe extern "C" fn analyze_misc(mut o_ptr: *mut object_type,
                                  mut misc_desc: *mut libc::c_char) {
    let mut a_ptr: *mut artifact_type =
        &mut *a_info.offset((*o_ptr).name1 as isize) as *mut artifact_type;
    sprintf(misc_desc,
            b"Level %u, Rarity %u, %d.%d lbs, %ld Gold\x00" as *const u8 as
                *const libc::c_char, (*a_ptr).level as libc::c_int,
            (*a_ptr).rarity as libc::c_int,
            (*a_ptr).weight as libc::c_int / 10 as libc::c_int,
            (*a_ptr).weight as libc::c_int % 10 as libc::c_int,
            (*a_ptr).cost as libc::c_long);
}
/*
 * Fill in an object description structure for a given object
 */
unsafe extern "C" fn object_analyze(mut o_ptr: *mut object_type,
                                    mut desc_ptr: *mut obj_desc_list) {
    analyze_general(o_ptr, (*desc_ptr).description.as_mut_ptr());
    analyze_pval(o_ptr, &mut (*desc_ptr).pval_info);
    analyze_brand(o_ptr, (*desc_ptr).brands.as_mut_ptr());
    analyze_slay(o_ptr, (*desc_ptr).slays.as_mut_ptr());
    analyze_immune(o_ptr, (*desc_ptr).immunities.as_mut_ptr());
    analyze_resist(o_ptr, (*desc_ptr).resistances.as_mut_ptr());
    analyze_sustains(o_ptr, (*desc_ptr).sustains.as_mut_ptr());
    analyze_misc_magic(o_ptr, (*desc_ptr).misc_magic.as_mut_ptr());
    analyze_misc(o_ptr, (*desc_ptr).misc_desc.as_mut_ptr());
    (*desc_ptr).activation =
        item_activation(o_ptr, 0 as libc::c_int as byte_hack);
}
unsafe extern "C" fn print_header() {
    let mut buf: [libc::c_char; 80] = [0; 80];
    sprintf(buf.as_mut_ptr(),
            b"Artifact Spoilers for %s\x00" as *const u8 as
                *const libc::c_char, get_version_string());
    spoiler_underline(buf.as_mut_ptr() as cptr);
}
/*
 * This is somewhat ugly.
 *
 * Given a header ("Resist", e.g.), a list ("Fire", "Cold", Acid", e.g.),
 * and a separator character (',', e.g.), write the list to the spoiler file
 * in a "nice" format, such as:
 *
 *      Resist Fire, Cold, Acid
 *
 * That was a simple example, but when the list is long, a line wrap
 * should occur, and this should induce a new level of indention if
 * a list is being spread across lines. So for example, Bladeturner's
 * list of resistances should look something like this
 *
 *     Resist Acid, Lightning, Fire, Cold, Poison, Light, Dark, Blindness,
 *       Confusion, Sound, Shards, Nether, Nexus, Chaos, Disenchantment
 *
 * However, the code distinguishes between a single list of many items vs.
 * many lists. (The separator is used to make this determination.) A single
 * list of many items will not cause line wrapping (since there is no
 * apparent reason to do so). So the lists of Ulmo's miscellaneous traits
 * might look like this:
 *
 *     Free Action; Hold Life; See Invisible; Slow Digestion; Regeneration
 *     Blessed Blade
 *
 * So comparing the two, "Regeneration" has no trailing separator and
 * "Blessed Blade" was not indented. (Also, Ulmo's lists have no headers,
 * but that's not relevant to line wrapping and indention.)
 */
/* ITEM_SEP separates items within a list */
/* LIST_SEP separates lists */
/* Create a spoiler file entry for an artifact */
unsafe extern "C" fn spoiler_print_art(mut art_ptr: *mut obj_desc_list,
                                       mut name1: libc::c_int,
                                       mut set: libc::c_int,
                                       mut o_ptr: *mut object_type) {
    /* Don't indent the first line */
    fprintf(fff, b"%s\n    \x00" as *const u8 as *const libc::c_char,
            (*art_ptr).description.as_mut_ptr());
    text_out_indent = 4 as libc::c_int;
    object_out_desc(o_ptr, fff, 0 as libc::c_int as bool_,
                    1 as libc::c_int as bool_);
    text_out_indent = 0 as libc::c_int;
    /* End with the miscellaneous facts */
    fprintf(fff, b"%s%s\n\n\x00" as *const u8 as *const libc::c_char,
            b"    \x00" as *const u8 as *const libc::c_char,
            (*art_ptr).misc_desc.as_mut_ptr());
}
/*
 * Hack -- Create a "forged" artifact
 */
unsafe extern "C" fn make_fake_artifact(mut o_ptr: *mut object_type,
                                        mut name1: libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut cur: libc::c_int = 0;
    let mut a_ptr: *mut artifact_type =
        &mut *a_info.offset(name1 as isize) as *mut artifact_type;
    /* Ignore "empty" artifacts */
    if (*a_ptr).name == 0 { return 0 as libc::c_int as bool_ }
    /* Acquire the "kind" index */
    i =
        lookup_kind((*a_ptr).tval as libc::c_int,
                    (*a_ptr).sval as libc::c_int) as libc::c_int;
    /* Oops */
    if i == 0 { return 0 as libc::c_int as bool_ }
    /* Create the artifact */
    object_prep(o_ptr, i);
    /* Save the name */
    (*o_ptr).name1 = name1 as byte_hack;
    /* Keep the One Ring untouched by apply_magic */
    if name1 != 13 as libc::c_int {
        cur = (*a_ptr).cur_num as libc::c_int;
        apply_magic(o_ptr, -(1 as libc::c_int), 1 as libc::c_int as bool_,
                    1 as libc::c_int as bool_, 1 as libc::c_int as bool_);
        (*a_ptr).cur_num = cur as byte_hack
    } else { (*o_ptr).pval = (*a_ptr).pval as s32b }
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
 * Create a spoiler file for artifacts
 */
unsafe extern "C" fn spoil_artifact(mut fname: cptr) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut forge: object_type =
        object_type{k_idx: 0,
                    iy: 0,
                    ix: 0,
                    tval: 0,
                    sval: 0,
                    pval: 0,
                    pval2: 0,
                    pval3: 0,
                    discount: 0,
                    number: 0,
                    weight: 0,
                    elevel: 0,
                    exp: 0,
                    name1: 0,
                    name2: 0,
                    name2b: 0,
                    xtra1: 0,
                    xtra2: 0,
                    to_h: 0,
                    to_d: 0,
                    to_a: 0,
                    ac: 0,
                    dd: 0,
                    ds: 0,
                    timeout: 0,
                    ident: 0,
                    marked: 0,
                    note: 0,
                    art_name: 0,
                    art_flags1: 0,
                    art_flags2: 0,
                    art_flags3: 0,
                    art_flags4: 0,
                    art_flags5: 0,
                    art_esp: 0,
                    art_oflags1: 0,
                    art_oflags2: 0,
                    art_oflags3: 0,
                    art_oflags4: 0,
                    art_oflags5: 0,
                    art_oesp: 0,
                    next_o_idx: 0,
                    held_m_idx: 0,
                    sense: 0,
                    found: 0,
                    found_aux1: 0,
                    found_aux2: 0,
                    found_aux3: 0,
                    found_aux4: 0,};
    let mut q_ptr: *mut object_type = 0 as *mut object_type;
    let mut artifact: obj_desc_list =
        obj_desc_list{description: [0; 160],
                      pval_info:
                          pval_info_type{pval_desc: [0; 12],
                                         pval_affects:
                                             [0 as *const libc::c_char; 12],},
                      slays: [0 as *const libc::c_char; 10],
                      brands: [0 as *const libc::c_char; 10],
                      immunities: [0 as *const libc::c_char; 5],
                      resistances: [0 as *const libc::c_char; 17],
                      sustains: [0 as *const libc::c_char; 6],
                      misc_magic: [0 as *const libc::c_char; 19],
                      activation: 0 as *const libc::c_char,
                      misc_desc: [0; 80],};
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Build the filename */
    path_build(buf.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int, ANGBAND_DIR_USER, fname);
    /* File type is "TEXT" */
    /* Open the file */
    fff =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    /* Oops */
    if fff.is_null() {
        msg_print(b"Cannot create spoiler file.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Dump the header */
    print_header();
    /* List the artifacts by tval */
    i = 0 as libc::c_int;
    while group_artifact[i as usize].tval != 0 {
        /* Write out the group title */
        if !group_artifact[i as usize].name.is_null() {
            spoiler_blanklines(2 as libc::c_int);
            spoiler_underline(group_artifact[i as usize].name);
            spoiler_blanklines(1 as libc::c_int);
        }
        /* Now search through all of the artifacts */
        j = 1 as libc::c_int;
        while j < max_a_idx as libc::c_int {
            let mut a_ptr: *mut artifact_type =
                &mut *a_info.offset(j as isize) as *mut artifact_type;
            /* We only want objects in the current group */
            if !((*a_ptr).tval as libc::c_int !=
                     group_artifact[i as usize].tval as libc::c_int) {
                /* Get local object */
                q_ptr = &mut forge;
                /* Wipe the object */
                object_wipe(q_ptr);
                /* Attempt to "forge" the artifact */
                if !(make_fake_artifact(q_ptr, j) == 0) {
                    /* Aware and Known */
                    object_known(q_ptr);
                    /* Mark the item as fully known */
                    (*q_ptr).ident =
                        ((*q_ptr).ident as libc::c_int | 0x20 as libc::c_int)
                            as byte_hack;
                    /* Analyze the artifact */
                    object_analyze(q_ptr, &mut artifact);
                    /* Write out the artifact description to the spoiler file */
                    spoiler_print_art(&mut artifact, j,
                                      (*a_ptr).set as libc::c_int, q_ptr);
                }
            }
            j += 1
        }
        i += 1
    }
    /* Check for errors */
    if ferror(fff) != 0 || my_fclose(fff) != 0 {
        msg_print(b"Cannot close spoiler file.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Message */
    msg_print(b"Successfully created a spoiler file.\x00" as *const u8 as
                  *const libc::c_char);
}
/*
 * Create a spoiler file for monsters   -BEN-
 */
unsafe extern "C" fn spoil_mon_desc(mut fname: cptr) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut who: *mut s16b = 0 as *mut s16b;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut nam: [libc::c_char; 80] = [0; 80];
    let mut lev: [libc::c_char; 80] = [0; 80];
    let mut rar: [libc::c_char; 80] = [0; 80];
    let mut spd: [libc::c_char; 80] = [0; 80];
    let mut ac: [libc::c_char; 80] = [0; 80];
    let mut hp: [libc::c_char; 80] = [0; 80];
    let mut exp: [libc::c_char; 80] = [0; 80];
    /* Build the filename */
    path_build(buf.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int, ANGBAND_DIR_USER, fname);
    /* File type is "TEXT" */
    /* Open the file */
    fff =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    /* Oops */
    if fff.is_null() {
        msg_print(b"Cannot create spoiler file.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Allocate the "who" array */
    who =
        memset(ralloc((max_r_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<s16b>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_r_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<s16b>()
                                                    as libc::c_ulong)) as
            *mut s16b;
    /* Dump the header */
    sprintf(buf.as_mut_ptr(),
            b"Monster Spoilers for %s\x00" as *const u8 as
                *const libc::c_char, get_version_string());
    spoiler_underline(buf.as_mut_ptr() as cptr);
    spoiler_blanklines(2 as libc::c_int);
    /* Dump the header */
    fprintf(fff,
            b"%-40.40s%4s%4s%6s%8s%4s  %11.11s\n\x00" as *const u8 as
                *const libc::c_char,
            b"Name\x00" as *const u8 as *const libc::c_char,
            b"Lev\x00" as *const u8 as *const libc::c_char,
            b"Rar\x00" as *const u8 as *const libc::c_char,
            b"Spd\x00" as *const u8 as *const libc::c_char,
            b"Hp\x00" as *const u8 as *const libc::c_char,
            b"Ac\x00" as *const u8 as *const libc::c_char,
            b"Visual Info\x00" as *const u8 as *const libc::c_char);
    fprintf(fff,
            b"%-40.40s%4s%4s%6s%8s%4s  %11.11s\n\x00" as *const u8 as
                *const libc::c_char,
            b"----\x00" as *const u8 as *const libc::c_char,
            b"---\x00" as *const u8 as *const libc::c_char,
            b"---\x00" as *const u8 as *const libc::c_char,
            b"---\x00" as *const u8 as *const libc::c_char,
            b"--\x00" as *const u8 as *const libc::c_char,
            b"--\x00" as *const u8 as *const libc::c_char,
            b"-----------\x00" as *const u8 as *const libc::c_char);
    /* Scan the monsters */
    i = 1 as libc::c_int;
    while i < max_r_idx as libc::c_int {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset(i as isize) as *mut monster_race;
        /* Use that monster */
        if (*r_ptr).name != 0 {
            let fresh13 = n;
            n = n + 1;
            *who.offset(fresh13 as isize) = i as s16b
        }
        i += 1
    }
    /* Scan again */
    i = 0 as libc::c_int;
    while i < n {
        let mut r_ptr_0: *mut monster_race =
            &mut *r_info.offset(*who.offset(i as isize) as isize) as
                *mut monster_race;
        let mut name: cptr = r_name.offset((*r_ptr_0).name as isize) as cptr;
        /* Get the "name" */
        if (*r_ptr_0).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
            sprintf(nam.as_mut_ptr(),
                    b"[U] %s\x00" as *const u8 as *const libc::c_char, name);
        } else {
            sprintf(nam.as_mut_ptr(),
                    b"The %s\x00" as *const u8 as *const libc::c_char, name);
        }
        /* Level */
        sprintf(lev.as_mut_ptr(),
                b"%d\x00" as *const u8 as *const libc::c_char,
                (*r_ptr_0).level as libc::c_int);
        /* Rarity */
        sprintf(rar.as_mut_ptr(),
                b"%d\x00" as *const u8 as *const libc::c_char,
                (*r_ptr_0).rarity as libc::c_int);
        /* Speed */
        if (*r_ptr_0).speed as libc::c_int >= 110 as libc::c_int {
            sprintf(spd.as_mut_ptr(),
                    b"+%d\x00" as *const u8 as *const libc::c_char,
                    (*r_ptr_0).speed as libc::c_int - 110 as libc::c_int);
        } else {
            sprintf(spd.as_mut_ptr(),
                    b"-%d\x00" as *const u8 as *const libc::c_char,
                    110 as libc::c_int - (*r_ptr_0).speed as libc::c_int);
        }
        /* Armor Class */
        sprintf(ac.as_mut_ptr(),
                b"%d\x00" as *const u8 as *const libc::c_char,
                (*r_ptr_0).ac as libc::c_int);
        /* Hitpoints */
        if (*r_ptr_0).flags1 & 0x200 as libc::c_int as libc::c_uint != 0 ||
               (*r_ptr_0).hside as libc::c_int == 1 as libc::c_int {
            sprintf(hp.as_mut_ptr(),
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    (*r_ptr_0).hdice as libc::c_int *
                        (*r_ptr_0).hside as libc::c_int);
        } else {
            sprintf(hp.as_mut_ptr(),
                    b"%dd%d\x00" as *const u8 as *const libc::c_char,
                    (*r_ptr_0).hdice as libc::c_int,
                    (*r_ptr_0).hside as libc::c_int);
        }
        /* Experience */
        sprintf(exp.as_mut_ptr(),
                b"%ld\x00" as *const u8 as *const libc::c_char,
                (*r_ptr_0).mexp as libc::c_long);
        /* Hack -- use visual instead */
        sprintf(exp.as_mut_ptr(),
                b"%s \'%c\'\x00" as *const u8 as *const libc::c_char,
                attr_to_text((*r_ptr_0).d_attr),
                (*r_ptr_0).d_char as libc::c_int);
        /* Dump the info */
        fprintf(fff,
                b"%-40.40s%4s%4s%6s%8s%4s  %11.11s\n\x00" as *const u8 as
                    *const libc::c_char, nam.as_mut_ptr(), lev.as_mut_ptr(),
                rar.as_mut_ptr(), spd.as_mut_ptr(), hp.as_mut_ptr(),
                ac.as_mut_ptr(), exp.as_mut_ptr());
        i += 1
    }
    /* End it */
    fprintf(fff, b"\n\x00" as *const u8 as *const libc::c_char);
    /* Free the "who" array */
    who =
        rnfree(who as vptr,
               (max_r_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<s16b>()
                                                    as libc::c_ulong)) as
            *mut s16b;
    /* Check for errors */
    if ferror(fff) != 0 || my_fclose(fff) != 0 {
        msg_print(b"Cannot close spoiler file.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Worked */
    msg_print(b"Successfully created a spoiler file.\x00" as *const u8 as
                  *const libc::c_char);
}
/*
 * Monster spoilers by: smchorse@ringer.cs.utsa.edu (Shawn McHorse)
 *
 * Primarily based on code already in mon-desc.c, mostly by -BEN-
 */
/*
 * Pronoun arrays
 */
static mut wd_che: [cptr; 3] =
    [b"It\x00" as *const u8 as *const libc::c_char,
     b"He\x00" as *const u8 as *const libc::c_char,
     b"She\x00" as *const u8 as *const libc::c_char];
static mut wd_lhe: [cptr; 3] =
    [b"it\x00" as *const u8 as *const libc::c_char,
     b"he\x00" as *const u8 as *const libc::c_char,
     b"she\x00" as *const u8 as *const libc::c_char];
/*
 * Create a spoiler file for monsters (-SHAWN-)
 */
unsafe extern "C" fn spoil_mon_info(mut fname: cptr) {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut msex: libc::c_int = 0;
    let mut vn: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut breath: bool_ = 0;
    let mut magic: bool_ = 0;
    let mut sin: bool_ = 0;
    let mut p: cptr = 0 as *const libc::c_char;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut vp: [cptr; 64] = [0 as *const libc::c_char; 64];
    let mut flags1: u32b = 0;
    let mut flags2: u32b = 0;
    let mut flags3: u32b = 0;
    let mut flags4: u32b = 0;
    let mut flags5: u32b = 0;
    let mut flags6: u32b = 0;
    let mut flags9: u32b = 0;
    /* Build the filename */
    path_build(buf.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int, ANGBAND_DIR_USER, fname);
    /* File type is "TEXT" */
    /* Open the file */
    fff =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    /* Oops */
    if fff.is_null() {
        msg_print(b"Cannot create spoiler file.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Dump the header */
    sprintf(buf.as_mut_ptr(),
            b"Monster Spoilers for %s\x00" as *const u8 as
                *const libc::c_char, get_version_string());
    spoiler_underline(buf.as_mut_ptr() as cptr);
    spoiler_blanklines(2 as libc::c_int);
    /*
	 * List all monsters in order.
	 */
    n = 1 as libc::c_int;
    while n < max_r_idx as libc::c_int {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset(n as isize) as *mut monster_race;
        /* Extract the flags */
        flags1 = (*r_ptr).flags1;
        flags2 = (*r_ptr).flags2;
        flags3 = (*r_ptr).flags3;
        flags4 = (*r_ptr).flags4;
        flags5 = (*r_ptr).flags5;
        flags6 = (*r_ptr).flags6;
        flags9 = (*r_ptr).flags9;
        breath = 0 as libc::c_int as bool_;
        magic = 0 as libc::c_int as bool_;
        /* Extract a gender (if applicable) */
        if flags1 & 0x8 as libc::c_int as libc::c_uint != 0 {
            msex = 2 as libc::c_int
        } else if flags1 & 0x4 as libc::c_int as libc::c_uint != 0 {
            msex = 1 as libc::c_int
        } else { msex = 0 as libc::c_int }
        /* Prefix */
        if flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
            spoil_out(b"[U] \x00" as *const u8 as *const libc::c_char);
        } else { spoil_out(b"The \x00" as *const u8 as *const libc::c_char); }
        /* Name */
        sprintf(buf.as_mut_ptr(),
                b"%s  (\x00" as *const u8 as *const libc::c_char,
                r_name.offset((*r_ptr).name as isize)); /* ---)--- */
        spoil_out(buf.as_mut_ptr() as cptr);
        /* Color */
        spoil_out(attr_to_text((*r_ptr).d_attr));
        /* Symbol --(-- */
        sprintf(buf.as_mut_ptr(),
                b" \'%c\')\n\x00" as *const u8 as *const libc::c_char,
                (*r_ptr).d_char as libc::c_int);
        spoil_out(buf.as_mut_ptr() as cptr);
        /* Indent */
        sprintf(buf.as_mut_ptr(),
                b"=== \x00" as *const u8 as *const libc::c_char);
        spoil_out(buf.as_mut_ptr() as cptr);
        /* Number */
        sprintf(buf.as_mut_ptr(),
                b"Num:%d  \x00" as *const u8 as *const libc::c_char, n);
        spoil_out(buf.as_mut_ptr() as cptr);
        /* Level */
        sprintf(buf.as_mut_ptr(),
                b"Lev:%d  \x00" as *const u8 as *const libc::c_char,
                (*r_ptr).level as libc::c_int);
        spoil_out(buf.as_mut_ptr() as cptr);
        /* Rarity */
        sprintf(buf.as_mut_ptr(),
                b"Rar:%d  \x00" as *const u8 as *const libc::c_char,
                (*r_ptr).rarity as libc::c_int);
        spoil_out(buf.as_mut_ptr() as cptr);
        /* Speed */
        if (*r_ptr).speed as libc::c_int >= 110 as libc::c_int {
            sprintf(buf.as_mut_ptr(),
                    b"Spd:+%d  \x00" as *const u8 as *const libc::c_char,
                    (*r_ptr).speed as libc::c_int - 110 as libc::c_int);
        } else {
            sprintf(buf.as_mut_ptr(),
                    b"Spd:-%d  \x00" as *const u8 as *const libc::c_char,
                    110 as libc::c_int - (*r_ptr).speed as libc::c_int);
        }
        spoil_out(buf.as_mut_ptr() as cptr);
        /* Hitpoints */
        if flags1 & 0x200 as libc::c_int as libc::c_uint != 0 ||
               (*r_ptr).hside as libc::c_int == 1 as libc::c_int {
            sprintf(buf.as_mut_ptr(),
                    b"Hp:%d  \x00" as *const u8 as *const libc::c_char,
                    (*r_ptr).hdice as libc::c_int *
                        (*r_ptr).hside as libc::c_int);
        } else {
            sprintf(buf.as_mut_ptr(),
                    b"Hp:%dd%d  \x00" as *const u8 as *const libc::c_char,
                    (*r_ptr).hdice as libc::c_int,
                    (*r_ptr).hside as libc::c_int);
        }
        spoil_out(buf.as_mut_ptr() as cptr);
        /* Armor Class */
        sprintf(buf.as_mut_ptr(),
                b"Ac:%d  \x00" as *const u8 as *const libc::c_char,
                (*r_ptr).ac as libc::c_int);
        spoil_out(buf.as_mut_ptr() as cptr);
        /* Experience */
        sprintf(buf.as_mut_ptr(),
                b"Exp:%ld\n\x00" as *const u8 as *const libc::c_char,
                (*r_ptr).mexp as libc::c_long);
        spoil_out(buf.as_mut_ptr() as cptr);
        /* Describe */
        spoil_out(r_text.offset((*r_ptr).text as isize) as cptr);
        spoil_out(b" \x00" as *const u8 as *const libc::c_char);
        spoil_out(b"This\x00" as *const u8 as *const libc::c_char);
        if flags2 & 0x2000 as libc::c_int as libc::c_uint != 0 {
            spoil_out(b" sanity-blasting\x00" as *const u8 as
                          *const libc::c_char);
        }
        if flags3 & 0x80 as libc::c_int as libc::c_uint != 0 {
            spoil_out(b" natural\x00" as *const u8 as *const libc::c_char);
        }
        if flags3 & 0x40 as libc::c_int as libc::c_uint != 0 {
            spoil_out(b" evil\x00" as *const u8 as *const libc::c_char);
        }
        if flags3 & 0x200 as libc::c_int as libc::c_uint != 0 {
            spoil_out(b" good\x00" as *const u8 as *const libc::c_char);
        }
        if flags3 & 0x20 as libc::c_int as libc::c_uint != 0 {
            spoil_out(b" undead\x00" as *const u8 as *const libc::c_char);
        }
        if flags3 & 0x8 as libc::c_int as libc::c_uint != 0 {
            spoil_out(b" dragon\x00" as *const u8 as *const libc::c_char);
        } else if flags3 & 0x10 as libc::c_int as libc::c_uint != 0 {
            spoil_out(b" demon\x00" as *const u8 as *const libc::c_char);
        } else if flags3 & 0x4 as libc::c_int as libc::c_uint != 0 {
            spoil_out(b" giant\x00" as *const u8 as *const libc::c_char);
        } else if flags3 & 0x2 as libc::c_int as libc::c_uint != 0 {
            spoil_out(b" troll\x00" as *const u8 as *const libc::c_char);
        } else if flags3 & 0x1 as libc::c_int as libc::c_uint != 0 {
            spoil_out(b" orc\x00" as *const u8 as *const libc::c_char);
        } else if flags3 & 0x100 as libc::c_int as libc::c_uint != 0 {
            spoil_out(b" Thunderlord\x00" as *const u8 as
                          *const libc::c_char);
        } else {
            spoil_out(b" creature\x00" as *const u8 as *const libc::c_char);
        }
        spoil_out(b" moves\x00" as *const u8 as *const libc::c_char);
        if flags1 & 0x80000 as libc::c_int as libc::c_uint != 0 &&
               flags1 & 0x40000 as libc::c_int as libc::c_uint != 0 {
            spoil_out(b" extremely erratically\x00" as *const u8 as
                          *const libc::c_char);
        } else if flags1 & 0x80000 as libc::c_int as libc::c_uint != 0 {
            spoil_out(b" somewhat erratically\x00" as *const u8 as
                          *const libc::c_char);
        } else if flags1 & 0x40000 as libc::c_int as libc::c_uint != 0 {
            spoil_out(b" a bit erratically\x00" as *const u8 as
                          *const libc::c_char);
        } else {
            spoil_out(b" normally\x00" as *const u8 as *const libc::c_char);
        }
        if flags1 & 0x20000 as libc::c_int as libc::c_uint != 0 {
            spoil_out(b", but does not deign to chase intruders\x00" as
                          *const u8 as *const libc::c_char);
        }
        spoil_out(b".  \x00" as *const u8 as *const libc::c_char);
        if (*r_ptr).level == 0 ||
               flags1 & 0x100 as libc::c_int as libc::c_uint != 0 {
            sprintf(buf.as_mut_ptr(),
                    b"%s is never found out of depth.  \x00" as *const u8 as
                        *const libc::c_char, wd_che[msex as usize]);
            spoil_out(buf.as_mut_ptr() as cptr);
        }
        if flags1 & 0x400 as libc::c_int as libc::c_uint != 0 {
            sprintf(buf.as_mut_ptr(),
                    b"%s is always created sluggish.  \x00" as *const u8 as
                        *const libc::c_char, wd_che[msex as usize]);
            spoil_out(buf.as_mut_ptr() as cptr);
        }
        if flags2 & 0x4000 as libc::c_int as libc::c_uint != 0 {
            sprintf(buf.as_mut_ptr(),
                    b"%s is surrounded by flames.  \x00" as *const u8 as
                        *const libc::c_char, wd_che[msex as usize]);
            spoil_out(buf.as_mut_ptr() as cptr);
        }
        if flags2 & 0x8000 as libc::c_int as libc::c_uint != 0 {
            sprintf(buf.as_mut_ptr(),
                    b"%s is surrounded by electricity.  \x00" as *const u8 as
                        *const libc::c_char, wd_che[msex as usize]);
            spoil_out(buf.as_mut_ptr() as cptr);
        }
        if flags2 & 0x8 as libc::c_int as libc::c_uint != 0 {
            sprintf(buf.as_mut_ptr(),
                    b"%s reflects bolt spells.  \x00" as *const u8 as
                        *const libc::c_char, wd_che[msex as usize]);
            spoil_out(buf.as_mut_ptr() as cptr);
        }
        if flags1 & 0x4000 as libc::c_int as libc::c_uint != 0 {
            sprintf(buf.as_mut_ptr(),
                    b"%s usually appears with \x00" as *const u8 as
                        *const libc::c_char, wd_che[msex as usize]);
            spoil_out(buf.as_mut_ptr() as cptr);
            if flags1 & 0x8000 as libc::c_int as libc::c_uint != 0 {
                spoil_out(b"escorts.  \x00" as *const u8 as
                              *const libc::c_char);
            } else {
                spoil_out(b"an escort.  \x00" as *const u8 as
                              *const libc::c_char);
            }
        }
        if flags1 & 0x1000 as libc::c_int as libc::c_uint != 0 ||
               flags1 & 0x2000 as libc::c_int as libc::c_uint != 0 {
            sprintf(buf.as_mut_ptr(),
                    b"%s usually appears in groups.  \x00" as *const u8 as
                        *const libc::c_char, wd_che[msex as usize]);
            spoil_out(buf.as_mut_ptr() as cptr);
        }
        /* Collect innate attacks */
        vn = 0 as libc::c_int;
        if flags4 & 0x1 as libc::c_int as libc::c_uint != 0 {
            let fresh14 = vn;
            vn = vn + 1;
            vp[fresh14 as usize] =
                b"shriek for help\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x8 as libc::c_int as libc::c_uint != 0 {
            let fresh15 = vn;
            vn = vn + 1;
            vp[fresh15 as usize] =
                b"shoot a rocket\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x10 as libc::c_int as libc::c_uint != 0 {
            let fresh16 = vn;
            vn = vn + 1;
            vp[fresh16 as usize] =
                b"fire arrows\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x20 as libc::c_int as libc::c_uint != 0 {
            let fresh17 = vn;
            vn = vn + 1;
            vp[fresh17 as usize] =
                b"fire arrows\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x40 as libc::c_int as libc::c_uint != 0 {
            let fresh18 = vn;
            vn = vn + 1;
            vp[fresh18 as usize] =
                b"fire missiles\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x80 as libc::c_int as libc::c_uint != 0 {
            let fresh19 = vn;
            vn = vn + 1;
            vp[fresh19 as usize] =
                b"fire missiles\x00" as *const u8 as *const libc::c_char
        }
        if vn != 0 {
            spoil_out(wd_che[msex as usize]);
            i = 0 as libc::c_int;
            while i < vn {
                if i == 0 {
                    spoil_out(b" may \x00" as *const u8 as
                                  *const libc::c_char);
                } else if i < vn - 1 as libc::c_int {
                    spoil_out(b", \x00" as *const u8 as *const libc::c_char);
                } else {
                    spoil_out(b" or \x00" as *const u8 as
                                  *const libc::c_char);
                }
                spoil_out(vp[i as usize]);
                i += 1
            }
            spoil_out(b".  \x00" as *const u8 as *const libc::c_char);
        }
        /* Collect breaths */
        vn = 0 as libc::c_int;
        if flags4 & 0x100 as libc::c_int as libc::c_uint != 0 {
            let fresh20 = vn;
            vn = vn + 1;
            vp[fresh20 as usize] =
                b"acid\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x200 as libc::c_int as libc::c_uint != 0 {
            let fresh21 = vn;
            vn = vn + 1;
            vp[fresh21 as usize] =
                b"lightning\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x400 as libc::c_int as libc::c_uint != 0 {
            let fresh22 = vn;
            vn = vn + 1;
            vp[fresh22 as usize] =
                b"fire\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x800 as libc::c_int as libc::c_uint != 0 {
            let fresh23 = vn;
            vn = vn + 1;
            vp[fresh23 as usize] =
                b"frost\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x1000 as libc::c_int as libc::c_uint != 0 {
            let fresh24 = vn;
            vn = vn + 1;
            vp[fresh24 as usize] =
                b"poison\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x2000 as libc::c_int as libc::c_uint != 0 {
            let fresh25 = vn;
            vn = vn + 1;
            vp[fresh25 as usize] =
                b"nether\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x4000 as libc::c_int as libc::c_uint != 0 {
            let fresh26 = vn;
            vn = vn + 1;
            vp[fresh26 as usize] =
                b"light\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x8000 as libc::c_int as libc::c_uint != 0 {
            let fresh27 = vn;
            vn = vn + 1;
            vp[fresh27 as usize] =
                b"darkness\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x10000 as libc::c_int as libc::c_uint != 0 {
            let fresh28 = vn;
            vn = vn + 1;
            vp[fresh28 as usize] =
                b"confusion\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x20000 as libc::c_int as libc::c_uint != 0 {
            let fresh29 = vn;
            vn = vn + 1;
            vp[fresh29 as usize] =
                b"sound\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x40000 as libc::c_int as libc::c_uint != 0 {
            let fresh30 = vn;
            vn = vn + 1;
            vp[fresh30 as usize] =
                b"chaos\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x80000 as libc::c_int as libc::c_uint != 0 {
            let fresh31 = vn;
            vn = vn + 1;
            vp[fresh31 as usize] =
                b"disenchantment\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x100000 as libc::c_int as libc::c_uint != 0 {
            let fresh32 = vn;
            vn = vn + 1;
            vp[fresh32 as usize] =
                b"nexus\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x200000 as libc::c_int as libc::c_uint != 0 {
            let fresh33 = vn;
            vn = vn + 1;
            vp[fresh33 as usize] =
                b"time\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x400000 as libc::c_int as libc::c_uint != 0 {
            let fresh34 = vn;
            vn = vn + 1;
            vp[fresh34 as usize] =
                b"inertia\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x800000 as libc::c_int as libc::c_uint != 0 {
            let fresh35 = vn;
            vn = vn + 1;
            vp[fresh35 as usize] =
                b"gravity\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x1000000 as libc::c_int as libc::c_uint != 0 {
            let fresh36 = vn;
            vn = vn + 1;
            vp[fresh36 as usize] =
                b"shards\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x2000000 as libc::c_int as libc::c_uint != 0 {
            let fresh37 = vn;
            vn = vn + 1;
            vp[fresh37 as usize] =
                b"plasma\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x4000000 as libc::c_int as libc::c_uint != 0 {
            let fresh38 = vn;
            vn = vn + 1;
            vp[fresh38 as usize] =
                b"force\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x8000000 as libc::c_int as libc::c_uint != 0 {
            let fresh39 = vn;
            vn = vn + 1;
            vp[fresh39 as usize] =
                b"mana\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x20000000 as libc::c_int as libc::c_uint != 0 {
            let fresh40 = vn;
            vn = vn + 1;
            vp[fresh40 as usize] =
                b"toxic waste\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x80000000 as libc::c_uint != 0 {
            let fresh41 = vn;
            vn = vn + 1;
            vp[fresh41 as usize] =
                b"disintegration\x00" as *const u8 as *const libc::c_char
        }
        if vn != 0 {
            breath = 1 as libc::c_int as bool_;
            spoil_out(wd_che[msex as usize]);
            i = 0 as libc::c_int;
            while i < vn {
                if i == 0 {
                    spoil_out(b" may breathe \x00" as *const u8 as
                                  *const libc::c_char);
                } else if i < vn - 1 as libc::c_int {
                    spoil_out(b", \x00" as *const u8 as *const libc::c_char);
                } else {
                    spoil_out(b" or \x00" as *const u8 as
                                  *const libc::c_char);
                }
                spoil_out(vp[i as usize]);
                i += 1
            }
            if flags2 & 0x1000 as libc::c_int as libc::c_uint != 0 {
                spoil_out(b" powerfully\x00" as *const u8 as
                              *const libc::c_char);
            }
        }
        /* Collect spells */
        vn = 0 as libc::c_int;
        if flags5 & 0x1 as libc::c_int as libc::c_uint != 0 {
            let fresh42 = vn;
            vn = vn + 1;
            vp[fresh42 as usize] =
                b"produce acid balls\x00" as *const u8 as *const libc::c_char
        }
        if flags5 & 0x2 as libc::c_int as libc::c_uint != 0 {
            let fresh43 = vn;
            vn = vn + 1;
            vp[fresh43 as usize] =
                b"produce lightning balls\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags5 & 0x4 as libc::c_int as libc::c_uint != 0 {
            let fresh44 = vn;
            vn = vn + 1;
            vp[fresh44 as usize] =
                b"produce fire balls\x00" as *const u8 as *const libc::c_char
        }
        if flags5 & 0x8 as libc::c_int as libc::c_uint != 0 {
            let fresh45 = vn;
            vn = vn + 1;
            vp[fresh45 as usize] =
                b"produce frost balls\x00" as *const u8 as *const libc::c_char
        }
        if flags5 & 0x10 as libc::c_int as libc::c_uint != 0 {
            let fresh46 = vn;
            vn = vn + 1;
            vp[fresh46 as usize] =
                b"produce poison balls\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags5 & 0x20 as libc::c_int as libc::c_uint != 0 {
            let fresh47 = vn;
            vn = vn + 1;
            vp[fresh47 as usize] =
                b"produce nether balls\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags5 & 0x40 as libc::c_int as libc::c_uint != 0 {
            let fresh48 = vn;
            vn = vn + 1;
            vp[fresh48 as usize] =
                b"produce water balls\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x10000000 as libc::c_int as libc::c_uint != 0 {
            let fresh49 = vn;
            vn = vn + 1;
            vp[fresh49 as usize] =
                b"produce balls of radiation\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags5 & 0x80 as libc::c_int as libc::c_uint != 0 {
            let fresh50 = vn;
            vn = vn + 1;
            vp[fresh50 as usize] =
                b"produce mana storms\x00" as *const u8 as *const libc::c_char
        }
        if flags5 & 0x100 as libc::c_int as libc::c_uint != 0 {
            let fresh51 = vn;
            vn = vn + 1;
            vp[fresh51 as usize] =
                b"produce darkness storms\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags4 & 0x40000000 as libc::c_int as libc::c_uint != 0 {
            let fresh52 = vn;
            vn = vn + 1;
            vp[fresh52 as usize] =
                b"invoke raw Chaos\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x2 as libc::c_int as libc::c_uint != 0 {
            let fresh53 = vn;
            vn = vn + 1;
            vp[fresh53 as usize] =
                b"invoke the Hand of Doom\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags5 & 0x200 as libc::c_int as libc::c_uint != 0 {
            let fresh54 = vn;
            vn = vn + 1;
            vp[fresh54 as usize] =
                b"drain mana\x00" as *const u8 as *const libc::c_char
        }
        if flags5 & 0x400 as libc::c_int as libc::c_uint != 0 {
            let fresh55 = vn;
            vn = vn + 1;
            vp[fresh55 as usize] =
                b"cause mind blasting\x00" as *const u8 as *const libc::c_char
        }
        if flags5 & 0x800 as libc::c_int as libc::c_uint != 0 {
            let fresh56 = vn;
            vn = vn + 1;
            vp[fresh56 as usize] =
                b"cause brain smashing\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags5 & 0x1000 as libc::c_int as libc::c_uint != 0 {
            let fresh57 = vn;
            vn = vn + 1;
            vp[fresh57 as usize] =
                b"cause light wounds and cursing\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags5 & 0x2000 as libc::c_int as libc::c_uint != 0 {
            let fresh58 = vn;
            vn = vn + 1;
            vp[fresh58 as usize] =
                b"cause serious wounds and cursing\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags5 & 0x4000 as libc::c_int as libc::c_uint != 0 {
            let fresh59 = vn;
            vn = vn + 1;
            vp[fresh59 as usize] =
                b"cause critical wounds and cursing\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags5 & 0x8000 as libc::c_int as libc::c_uint != 0 {
            let fresh60 = vn;
            vn = vn + 1;
            vp[fresh60 as usize] =
                b"cause mortal wounds\x00" as *const u8 as *const libc::c_char
        }
        if flags5 & 0x10000 as libc::c_int as libc::c_uint != 0 {
            let fresh61 = vn;
            vn = vn + 1;
            vp[fresh61 as usize] =
                b"produce acid bolts\x00" as *const u8 as *const libc::c_char
        }
        if flags5 & 0x20000 as libc::c_int as libc::c_uint != 0 {
            let fresh62 = vn;
            vn = vn + 1;
            vp[fresh62 as usize] =
                b"produce lightning bolts\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags5 & 0x40000 as libc::c_int as libc::c_uint != 0 {
            let fresh63 = vn;
            vn = vn + 1;
            vp[fresh63 as usize] =
                b"produce fire bolts\x00" as *const u8 as *const libc::c_char
        }
        if flags5 & 0x80000 as libc::c_int as libc::c_uint != 0 {
            let fresh64 = vn;
            vn = vn + 1;
            vp[fresh64 as usize] =
                b"produce frost bolts\x00" as *const u8 as *const libc::c_char
        }
        if flags5 & 0x100000 as libc::c_int as libc::c_uint != 0 {
            let fresh65 = vn;
            vn = vn + 1;
            vp[fresh65 as usize] =
                b"produce poison bolts\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags5 & 0x200000 as libc::c_int as libc::c_uint != 0 {
            let fresh66 = vn;
            vn = vn + 1;
            vp[fresh66 as usize] =
                b"produce nether bolts\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags5 & 0x400000 as libc::c_int as libc::c_uint != 0 {
            let fresh67 = vn;
            vn = vn + 1;
            vp[fresh67 as usize] =
                b"produce water bolts\x00" as *const u8 as *const libc::c_char
        }
        if flags5 & 0x800000 as libc::c_int as libc::c_uint != 0 {
            let fresh68 = vn;
            vn = vn + 1;
            vp[fresh68 as usize] =
                b"produce mana bolts\x00" as *const u8 as *const libc::c_char
        }
        if flags5 & 0x1000000 as libc::c_int as libc::c_uint != 0 {
            let fresh69 = vn;
            vn = vn + 1;
            vp[fresh69 as usize] =
                b"produce plasma bolts\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags5 & 0x2000000 as libc::c_int as libc::c_uint != 0 {
            let fresh70 = vn;
            vn = vn + 1;
            vp[fresh70 as usize] =
                b"produce ice bolts\x00" as *const u8 as *const libc::c_char
        }
        if flags5 & 0x4000000 as libc::c_int as libc::c_uint != 0 {
            let fresh71 = vn;
            vn = vn + 1;
            vp[fresh71 as usize] =
                b"produce magic missiles\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags5 & 0x8000000 as libc::c_int as libc::c_uint != 0 {
            let fresh72 = vn;
            vn = vn + 1;
            vp[fresh72 as usize] =
                b"terrify\x00" as *const u8 as *const libc::c_char
        }
        if flags5 & 0x10000000 as libc::c_int as libc::c_uint != 0 {
            let fresh73 = vn;
            vn = vn + 1;
            vp[fresh73 as usize] =
                b"blind\x00" as *const u8 as *const libc::c_char
        }
        if flags5 & 0x20000000 as libc::c_int as libc::c_uint != 0 {
            let fresh74 = vn;
            vn = vn + 1;
            vp[fresh74 as usize] =
                b"confuse\x00" as *const u8 as *const libc::c_char
        }
        if flags5 & 0x40000000 as libc::c_int as libc::c_uint != 0 {
            let fresh75 = vn;
            vn = vn + 1;
            vp[fresh75 as usize] =
                b"slow\x00" as *const u8 as *const libc::c_char
        }
        if flags5 & 0x80000000 as libc::c_uint != 0 {
            let fresh76 = vn;
            vn = vn + 1;
            vp[fresh76 as usize] =
                b"paralyse\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x1 as libc::c_int as libc::c_uint != 0 {
            let fresh77 = vn;
            vn = vn + 1;
            vp[fresh77 as usize] =
                b"haste-self\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x4 as libc::c_int as libc::c_uint != 0 {
            let fresh78 = vn;
            vn = vn + 1;
            vp[fresh78 as usize] =
                b"heal-self\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x10 as libc::c_int as libc::c_uint != 0 {
            let fresh79 = vn;
            vn = vn + 1;
            vp[fresh79 as usize] =
                b"blink-self\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x20 as libc::c_int as libc::c_uint != 0 {
            let fresh80 = vn;
            vn = vn + 1;
            vp[fresh80 as usize] =
                b"teleport-self\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x2000 as libc::c_int as libc::c_uint != 0 {
            let fresh81 = vn;
            vn = vn + 1;
            vp[fresh81 as usize] =
                b"summon software bugs\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags6 & 0x4000 as libc::c_int as libc::c_uint != 0 {
            let fresh82 = vn;
            vn = vn + 1;
            vp[fresh82 as usize] =
                b"summon RNGs\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x40 as libc::c_int as libc::c_uint != 0 {
            let fresh83 = vn;
            vn = vn + 1;
            vp[fresh83 as usize] =
                b"teleport to\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x80 as libc::c_int as libc::c_uint != 0 {
            let fresh84 = vn;
            vn = vn + 1;
            vp[fresh84 as usize] =
                b"teleport away\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x100 as libc::c_int as libc::c_uint != 0 {
            let fresh85 = vn;
            vn = vn + 1;
            vp[fresh85 as usize] =
                b"teleport level\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x200 as libc::c_int as libc::c_uint != 0 {
            let fresh86 = vn;
            vn = vn + 1;
            vp[fresh86 as usize] =
                b"create darkness\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x400 as libc::c_int as libc::c_uint != 0 {
            let fresh87 = vn;
            vn = vn + 1;
            vp[fresh87 as usize] =
                b"create traps\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x800 as libc::c_int as libc::c_uint != 0 {
            let fresh88 = vn;
            vn = vn + 1;
            vp[fresh88 as usize] =
                b"cause amnesia\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x1000 as libc::c_int as libc::c_uint != 0 {
            let fresh89 = vn;
            vn = vn + 1;
            vp[fresh89 as usize] =
                b"raise dead\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x8000 as libc::c_int as libc::c_uint != 0 {
            let fresh90 = vn;
            vn = vn + 1;
            vp[fresh90 as usize] =
                b"summon a thunderlord\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags6 & 0x40000 as libc::c_int as libc::c_uint != 0 {
            let fresh91 = vn;
            vn = vn + 1;
            vp[fresh91 as usize] =
                b"summon a monster\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x80000 as libc::c_int as libc::c_uint != 0 {
            let fresh92 = vn;
            vn = vn + 1;
            vp[fresh92 as usize] =
                b"summon monsters\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x10000 as libc::c_int as libc::c_uint != 0 {
            let fresh93 = vn;
            vn = vn + 1;
            vp[fresh93 as usize] =
                b"summon aid\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x100000 as libc::c_int as libc::c_uint != 0 {
            let fresh94 = vn;
            vn = vn + 1;
            vp[fresh94 as usize] =
                b"summon ants\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x200000 as libc::c_int as libc::c_uint != 0 {
            let fresh95 = vn;
            vn = vn + 1;
            vp[fresh95 as usize] =
                b"summon spiders\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x400000 as libc::c_int as libc::c_uint != 0 {
            let fresh96 = vn;
            vn = vn + 1;
            vp[fresh96 as usize] =
                b"summon hounds\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x800000 as libc::c_int as libc::c_uint != 0 {
            let fresh97 = vn;
            vn = vn + 1;
            vp[fresh97 as usize] =
                b"summon hydras\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x1000000 as libc::c_int as libc::c_uint != 0 {
            let fresh98 = vn;
            vn = vn + 1;
            vp[fresh98 as usize] =
                b"summon an angel\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x2000000 as libc::c_int as libc::c_uint != 0 {
            let fresh99 = vn;
            vn = vn + 1;
            vp[fresh99 as usize] =
                b"summon a demon\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x4000000 as libc::c_int as libc::c_uint != 0 {
            let fresh100 = vn;
            vn = vn + 1;
            vp[fresh100 as usize] =
                b"summon an undead\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x8000000 as libc::c_int as libc::c_uint != 0 {
            let fresh101 = vn;
            vn = vn + 1;
            vp[fresh101 as usize] =
                b"summon a dragon\x00" as *const u8 as *const libc::c_char
        }
        if flags4 & 0x4 as libc::c_int as libc::c_uint != 0 {
            let fresh102 = vn;
            vn = vn + 1;
            vp[fresh102 as usize] =
                b"summon animal\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x8 as libc::c_int as libc::c_uint != 0 {
            let fresh103 = vn;
            vn = vn + 1;
            vp[fresh103 as usize] =
                b"summon animals\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x10000000 as libc::c_int as libc::c_uint != 0 {
            let fresh104 = vn;
            vn = vn + 1;
            vp[fresh104 as usize] =
                b"summon greater undead\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags6 & 0x20000000 as libc::c_int as libc::c_uint != 0 {
            let fresh105 = vn;
            vn = vn + 1;
            vp[fresh105 as usize] =
                b"summon ancient dragons\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags6 & 0x20000 as libc::c_int as libc::c_uint != 0 {
            let fresh106 = vn;
            vn = vn + 1;
            vp[fresh106 as usize] =
                b"summon greater demons\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags6 & 0x40000000 as libc::c_int as libc::c_uint != 0 {
            let fresh107 = vn;
            vn = vn + 1;
            vp[fresh107 as usize] =
                b"summon Ringwraith\x00" as *const u8 as *const libc::c_char
        }
        if flags6 & 0x80000000 as libc::c_uint != 0 {
            let fresh108 = vn;
            vn = vn + 1;
            vp[fresh108 as usize] =
                b"summon unique monsters\x00" as *const u8 as
                    *const libc::c_char
        }
        if vn != 0 {
            magic = 1 as libc::c_int as bool_;
            if breath != 0 {
                spoil_out(b", and is also\x00" as *const u8 as
                              *const libc::c_char);
            } else {
                spoil_out(wd_che[msex as usize]);
                spoil_out(b" is\x00" as *const u8 as *const libc::c_char);
            }
            spoil_out(b" magical, casting spells\x00" as *const u8 as
                          *const libc::c_char);
            if flags2 & 0x2 as libc::c_int as libc::c_uint != 0 {
                spoil_out(b" intelligently\x00" as *const u8 as
                              *const libc::c_char);
            }
            i = 0 as libc::c_int;
            while i < vn {
                if i == 0 {
                    spoil_out(b" which \x00" as *const u8 as
                                  *const libc::c_char);
                } else if i < vn - 1 as libc::c_int {
                    spoil_out(b", \x00" as *const u8 as *const libc::c_char);
                } else {
                    spoil_out(b" or \x00" as *const u8 as
                                  *const libc::c_char);
                }
                spoil_out(vp[i as usize]);
                i += 1
            }
        }
        if breath as libc::c_int != 0 || magic as libc::c_int != 0 {
            let mut times: libc::c_int =
                (*r_ptr).freq_inate as libc::c_int +
                    (*r_ptr).freq_spell as libc::c_int;
            sprintf(buf.as_mut_ptr(),
                    b"; 1 time in %d.  \x00" as *const u8 as
                        *const libc::c_char,
                    200 as libc::c_int /
                        (if times != 0 { times } else { 1 as libc::c_int }));
            spoil_out(buf.as_mut_ptr() as cptr);
        }
        /* Collect special abilities. */
        vn = 0 as libc::c_int;
        if flags2 & 0x10000 as libc::c_int as libc::c_uint != 0 {
            let fresh109 = vn;
            vn = vn + 1;
            vp[fresh109 as usize] =
                b"open doors\x00" as *const u8 as *const libc::c_char
        }
        if flags2 & 0x20000 as libc::c_int as libc::c_uint != 0 {
            let fresh110 = vn;
            vn = vn + 1;
            vp[fresh110 as usize] =
                b"bash down doors\x00" as *const u8 as *const libc::c_char
        }
        if flags2 & 0x40000 as libc::c_int as libc::c_uint != 0 {
            let fresh111 = vn;
            vn = vn + 1;
            vp[fresh111 as usize] =
                b"pass through walls\x00" as *const u8 as *const libc::c_char
        }
        if flags2 & 0x80000 as libc::c_int as libc::c_uint != 0 {
            let fresh112 = vn;
            vn = vn + 1;
            vp[fresh112 as usize] =
                b"bore through walls\x00" as *const u8 as *const libc::c_char
        }
        if flags2 & 0x100000 as libc::c_int as libc::c_uint != 0 {
            let fresh113 = vn;
            vn = vn + 1;
            vp[fresh113 as usize] =
                b"push past weaker monsters\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags2 & 0x200000 as libc::c_int as libc::c_uint != 0 {
            let fresh114 = vn;
            vn = vn + 1;
            vp[fresh114 as usize] =
                b"destroy weaker monsters\x00" as *const u8 as
                    *const libc::c_char
        }
        if flags2 & 0x400000 as libc::c_int as libc::c_uint != 0 {
            let fresh115 = vn;
            vn = vn + 1;
            vp[fresh115 as usize] =
                b"pick up objects\x00" as *const u8 as *const libc::c_char
        }
        if flags2 & 0x800000 as libc::c_int as libc::c_uint != 0 {
            let fresh116 = vn;
            vn = vn + 1;
            vp[fresh116 as usize] =
                b"destroy objects\x00" as *const u8 as *const libc::c_char
        }
        if flags9 & 0x4 as libc::c_int as libc::c_uint != 0 {
            let fresh117 = vn;
            vn = vn + 1;
            vp[fresh117 as usize] =
                b"illuminate the dungeon\x00" as *const u8 as
                    *const libc::c_char
        }
        if vn != 0 {
            spoil_out(wd_che[msex as usize]);
            i = 0 as libc::c_int;
            while i < vn {
                if i == 0 {
                    spoil_out(b" can \x00" as *const u8 as
                                  *const libc::c_char);
                } else if i < vn - 1 as libc::c_int {
                    spoil_out(b", \x00" as *const u8 as *const libc::c_char);
                } else {
                    spoil_out(b" and \x00" as *const u8 as
                                  *const libc::c_char);
                }
                spoil_out(vp[i as usize]);
                i += 1
            }
            spoil_out(b".  \x00" as *const u8 as *const libc::c_char);
        }
        if flags2 & 0x10 as libc::c_int as libc::c_uint != 0 {
            spoil_out(wd_che[msex as usize]);
            spoil_out(b" is invisible.  \x00" as *const u8 as
                          *const libc::c_char);
        }
        if flags2 & 0x20 as libc::c_int as libc::c_uint != 0 {
            spoil_out(wd_che[msex as usize]);
            spoil_out(b" is cold blooded.  \x00" as *const u8 as
                          *const libc::c_char);
        }
        if flags2 & 0x40 as libc::c_int as libc::c_uint != 0 {
            spoil_out(wd_che[msex as usize]);
            spoil_out(b" is not detected by telepathy.  \x00" as *const u8 as
                          *const libc::c_char);
        }
        if flags2 & 0x80 as libc::c_int as libc::c_uint != 0 {
            spoil_out(wd_che[msex as usize]);
            spoil_out(b" is rarely detected by telepathy.  \x00" as *const u8
                          as *const libc::c_char);
        }
        if flags4 & 0x2 as libc::c_int as libc::c_uint != 0 {
            spoil_out(wd_che[msex as usize]);
            spoil_out(b" breeds explosively.  \x00" as *const u8 as
                          *const libc::c_char);
        }
        if flags2 & 0x200 as libc::c_int as libc::c_uint != 0 {
            spoil_out(wd_che[msex as usize]);
            spoil_out(b" regenerates quickly.  \x00" as *const u8 as
                          *const libc::c_char);
        }
        /* Collect susceptibilities */
        vn = 0 as libc::c_int;
        if flags3 & 0x2000 as libc::c_int as libc::c_uint != 0 {
            let fresh118 = vn;
            vn = vn + 1;
            vp[fresh118 as usize] =
                b"rock remover\x00" as *const u8 as *const libc::c_char
        }
        if flags3 & 0x1000 as libc::c_int as libc::c_uint != 0 {
            let fresh119 = vn;
            vn = vn + 1;
            vp[fresh119 as usize] =
                b"bright light\x00" as *const u8 as *const libc::c_char
        }
        if flags3 & 0x4000 as libc::c_int as libc::c_uint != 0 {
            let fresh120 = vn;
            vn = vn + 1;
            vp[fresh120 as usize] =
                b"fire\x00" as *const u8 as *const libc::c_char
        }
        if flags3 & 0x8000 as libc::c_int as libc::c_uint != 0 {
            let fresh121 = vn;
            vn = vn + 1;
            vp[fresh121 as usize] =
                b"cold\x00" as *const u8 as *const libc::c_char
        }
        if vn != 0 {
            spoil_out(wd_che[msex as usize]);
            i = 0 as libc::c_int;
            while i < vn {
                if i == 0 {
                    spoil_out(b" is hurt by \x00" as *const u8 as
                                  *const libc::c_char);
                } else if i < vn - 1 as libc::c_int {
                    spoil_out(b", \x00" as *const u8 as *const libc::c_char);
                } else {
                    spoil_out(b" and \x00" as *const u8 as
                                  *const libc::c_char);
                }
                spoil_out(vp[i as usize]);
                i += 1
            }
            spoil_out(b".  \x00" as *const u8 as *const libc::c_char);
        }
        /* Collect immunities */
        vn = 0 as libc::c_int;
        if flags3 & 0x10000 as libc::c_int as libc::c_uint != 0 {
            let fresh122 = vn;
            vn = vn + 1;
            vp[fresh122 as usize] =
                b"acid\x00" as *const u8 as *const libc::c_char
        }
        if flags3 & 0x20000 as libc::c_int as libc::c_uint != 0 {
            let fresh123 = vn;
            vn = vn + 1;
            vp[fresh123 as usize] =
                b"lightning\x00" as *const u8 as *const libc::c_char
        }
        if flags3 & 0x40000 as libc::c_int as libc::c_uint != 0 {
            let fresh124 = vn;
            vn = vn + 1;
            vp[fresh124 as usize] =
                b"fire\x00" as *const u8 as *const libc::c_char
        }
        if flags3 & 0x80000 as libc::c_int as libc::c_uint != 0 {
            let fresh125 = vn;
            vn = vn + 1;
            vp[fresh125 as usize] =
                b"cold\x00" as *const u8 as *const libc::c_char
        }
        if flags3 & 0x100000 as libc::c_int as libc::c_uint != 0 {
            let fresh126 = vn;
            vn = vn + 1;
            vp[fresh126 as usize] =
                b"poison\x00" as *const u8 as *const libc::c_char
        }
        if vn != 0 {
            spoil_out(wd_che[msex as usize]);
            i = 0 as libc::c_int;
            while i < vn {
                if i == 0 {
                    spoil_out(b" resists \x00" as *const u8 as
                                  *const libc::c_char);
                } else if i < vn - 1 as libc::c_int {
                    spoil_out(b", \x00" as *const u8 as *const libc::c_char);
                } else {
                    spoil_out(b" and \x00" as *const u8 as
                                  *const libc::c_char);
                }
                spoil_out(vp[i as usize]);
                i += 1
            }
            spoil_out(b".  \x00" as *const u8 as *const libc::c_char);
        }
        /* Collect resistances */
        vn = 0 as libc::c_int;
        if flags3 & 0x400000 as libc::c_int as libc::c_uint != 0 {
            let fresh127 = vn;
            vn = vn + 1;
            vp[fresh127 as usize] =
                b"nether\x00" as *const u8 as *const libc::c_char
        }
        if flags3 & 0x800000 as libc::c_int as libc::c_uint != 0 {
            let fresh128 = vn;
            vn = vn + 1;
            vp[fresh128 as usize] =
                b"water\x00" as *const u8 as *const libc::c_char
        }
        if flags3 & 0x1000000 as libc::c_int as libc::c_uint != 0 {
            let fresh129 = vn;
            vn = vn + 1;
            vp[fresh129 as usize] =
                b"plasma\x00" as *const u8 as *const libc::c_char
        }
        if flags3 & 0x2000000 as libc::c_int as libc::c_uint != 0 {
            let fresh130 = vn;
            vn = vn + 1;
            vp[fresh130 as usize] =
                b"nexus\x00" as *const u8 as *const libc::c_char
        }
        if flags3 & 0x4000000 as libc::c_int as libc::c_uint != 0 {
            let fresh131 = vn;
            vn = vn + 1;
            vp[fresh131 as usize] =
                b"disenchantment\x00" as *const u8 as *const libc::c_char
        }
        if flags3 & 0x200000 as libc::c_int as libc::c_uint != 0 {
            let fresh132 = vn;
            vn = vn + 1;
            vp[fresh132 as usize] =
                b"teleportation\x00" as *const u8 as *const libc::c_char
        }
        if vn != 0 {
            spoil_out(wd_che[msex as usize]);
            i = 0 as libc::c_int;
            while i < vn {
                if i == 0 {
                    spoil_out(b" resists \x00" as *const u8 as
                                  *const libc::c_char);
                } else if i < vn - 1 as libc::c_int {
                    spoil_out(b", \x00" as *const u8 as *const libc::c_char);
                } else {
                    spoil_out(b" and \x00" as *const u8 as
                                  *const libc::c_char);
                }
                spoil_out(vp[i as usize]);
                i += 1
            }
            spoil_out(b".  \x00" as *const u8 as *const libc::c_char);
        }
        /* Collect non-effects */
        vn = 0 as libc::c_int;
        if flags3 & 0x20000000 as libc::c_int as libc::c_uint != 0 {
            let fresh133 = vn;
            vn = vn + 1;
            vp[fresh133 as usize] =
                b"stunned\x00" as *const u8 as *const libc::c_char
        }
        if flags3 & 0x10000000 as libc::c_int as libc::c_uint != 0 {
            let fresh134 = vn;
            vn = vn + 1;
            vp[fresh134 as usize] =
                b"frightened\x00" as *const u8 as *const libc::c_char
        }
        if flags3 & 0x40000000 as libc::c_int as libc::c_uint != 0 {
            let fresh135 = vn;
            vn = vn + 1;
            vp[fresh135 as usize] =
                b"confused\x00" as *const u8 as *const libc::c_char
        }
        if flags3 & 0x80000000 as libc::c_uint != 0 {
            let fresh136 = vn;
            vn = vn + 1;
            vp[fresh136 as usize] =
                b"slept\x00" as *const u8 as *const libc::c_char
        }
        if vn != 0 {
            spoil_out(wd_che[msex as usize]);
            i = 0 as libc::c_int;
            while i < vn {
                if i == 0 {
                    spoil_out(b" cannot be \x00" as *const u8 as
                                  *const libc::c_char);
                } else if i < vn - 1 as libc::c_int {
                    spoil_out(b", \x00" as *const u8 as *const libc::c_char);
                } else {
                    spoil_out(b" or \x00" as *const u8 as
                                  *const libc::c_char);
                }
                spoil_out(vp[i as usize]);
                i += 1
            }
            spoil_out(b".  \x00" as *const u8 as *const libc::c_char);
        }
        spoil_out(wd_che[msex as usize]);
        if (*r_ptr).sleep as libc::c_int > 200 as libc::c_int {
            spoil_out(b" prefers to ignore\x00" as *const u8 as
                          *const libc::c_char);
        } else if (*r_ptr).sleep as libc::c_int > 95 as libc::c_int {
            spoil_out(b" pays very little attention to\x00" as *const u8 as
                          *const libc::c_char);
        } else if (*r_ptr).sleep as libc::c_int > 75 as libc::c_int {
            spoil_out(b" pays little attention to\x00" as *const u8 as
                          *const libc::c_char);
        } else if (*r_ptr).sleep as libc::c_int > 45 as libc::c_int {
            spoil_out(b" tends to overlook\x00" as *const u8 as
                          *const libc::c_char);
        } else if (*r_ptr).sleep as libc::c_int > 25 as libc::c_int {
            spoil_out(b" takes quite a while to see\x00" as *const u8 as
                          *const libc::c_char);
        } else if (*r_ptr).sleep as libc::c_int > 10 as libc::c_int {
            spoil_out(b" takes a while to see\x00" as *const u8 as
                          *const libc::c_char);
        } else if (*r_ptr).sleep as libc::c_int > 5 as libc::c_int {
            spoil_out(b" is fairly observant of\x00" as *const u8 as
                          *const libc::c_char);
        } else if (*r_ptr).sleep as libc::c_int > 3 as libc::c_int {
            spoil_out(b" is observant of\x00" as *const u8 as
                          *const libc::c_char);
        } else if (*r_ptr).sleep as libc::c_int > 1 as libc::c_int {
            spoil_out(b" is very observant of\x00" as *const u8 as
                          *const libc::c_char);
        } else if (*r_ptr).sleep as libc::c_int > 0 as libc::c_int {
            spoil_out(b" is vigilant for\x00" as *const u8 as
                          *const libc::c_char);
        } else {
            spoil_out(b" is ever vigilant for\x00" as *const u8 as
                          *const libc::c_char);
        }
        sprintf(buf.as_mut_ptr(),
                b" intruders, which %s may notice from %d feet.  \x00" as
                    *const u8 as *const libc::c_char, wd_lhe[msex as usize],
                10 as libc::c_int * (*r_ptr).aaf as libc::c_int);
        spoil_out(buf.as_mut_ptr() as cptr);
        i = 0 as libc::c_int;
        if flags1 & 0x400000 as libc::c_int as libc::c_uint != 0 {
            i += 1 as libc::c_int
        }
        if flags1 & 0x800000 as libc::c_int as libc::c_uint != 0 {
            i += 2 as libc::c_int
        }
        if flags1 & 0x1000000 as libc::c_int as libc::c_uint != 0 {
            i += 2 as libc::c_int
        }
        if flags1 & 0x2000000 as libc::c_int as libc::c_uint != 0 {
            i += 4 as libc::c_int
        }
        if flags1 & 0x4000000 as libc::c_int as libc::c_uint != 0 {
            i += 6 as libc::c_int
        }
        if flags1 & 0x8000000 as libc::c_int as libc::c_uint != 0 {
            i += 8 as libc::c_int
        }
        /* Drops gold and/or items */
        if i != 0 {
            sin = 0 as libc::c_int as bool_;
            spoil_out(wd_che[msex as usize]);
            spoil_out(b" will carry\x00" as *const u8 as *const libc::c_char);
            if i == 1 as libc::c_int {
                spoil_out(b" a\x00" as *const u8 as *const libc::c_char);
                sin = 1 as libc::c_int as bool_
            } else if i == 2 as libc::c_int {
                spoil_out(b" one or two\x00" as *const u8 as
                              *const libc::c_char);
            } else {
                sprintf(buf.as_mut_ptr(),
                        b" up to %u\x00" as *const u8 as *const libc::c_char,
                        i);
                spoil_out(buf.as_mut_ptr() as cptr);
            }
            if flags1 & 0x20000000 as libc::c_int as libc::c_uint != 0 {
                if sin != 0 {
                    spoil_out(b"n\x00" as *const u8 as *const libc::c_char);
                }
                spoil_out(b" exceptional object\x00" as *const u8 as
                              *const libc::c_char);
            } else if flags1 & 0x10000000 as libc::c_int as libc::c_uint != 0
             {
                spoil_out(b" good object\x00" as *const u8 as
                              *const libc::c_char);
            } else if flags1 & 0x40000000 as libc::c_int as libc::c_uint != 0
             {
                spoil_out(b" useful object\x00" as *const u8 as
                              *const libc::c_char);
            } else if flags1 & 0x200000 as libc::c_int as libc::c_uint != 0 {
                spoil_out(b" object\x00" as *const u8 as *const libc::c_char);
            } else if flags1 & 0x100000 as libc::c_int as libc::c_uint != 0 {
                spoil_out(b" treasure\x00" as *const u8 as
                              *const libc::c_char);
            } else {
                if sin != 0 {
                    spoil_out(b"n\x00" as *const u8 as *const libc::c_char);
                }
                spoil_out(b" object\x00" as *const u8 as *const libc::c_char);
                if i > 1 as libc::c_int {
                    spoil_out(b"s\x00" as *const u8 as *const libc::c_char);
                }
                spoil_out(b" or treasure\x00" as *const u8 as
                              *const libc::c_char);
            }
            if i > 1 as libc::c_int {
                spoil_out(b"s\x00" as *const u8 as *const libc::c_char);
            }
            if flags1 & 0x80000000 as libc::c_uint != 0 {
                spoil_out(b", in addition to chosen objects\x00" as *const u8
                              as *const libc::c_char);
            }
            spoil_out(b".  \x00" as *const u8 as *const libc::c_char);
        }
        /* Count the actual attacks */
        i = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            if (*r_ptr).blow[j as usize].method != 0 { i += 1 }
            j += 1
        }
        /* Examine the actual attacks */
        k = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            if !((*r_ptr).blow[j as usize].method == 0) {
                /* No method yet */
                p = b"???\x00" as *const u8 as *const libc::c_char;
                /* Acquire the method */
                match (*r_ptr).blow[j as usize].method as libc::c_int {
                    1 => {
                        p = b"hit\x00" as *const u8 as *const libc::c_char
                    }
                    2 => {
                        p = b"touch\x00" as *const u8 as *const libc::c_char
                    }
                    3 => {
                        p = b"punch\x00" as *const u8 as *const libc::c_char
                    }
                    4 => {
                        p = b"kick\x00" as *const u8 as *const libc::c_char
                    }
                    5 => {
                        p = b"claw\x00" as *const u8 as *const libc::c_char
                    }
                    6 => {
                        p = b"bite\x00" as *const u8 as *const libc::c_char
                    }
                    7 => {
                        p = b"sting\x00" as *const u8 as *const libc::c_char
                    }
                    9 => {
                        p = b"butt\x00" as *const u8 as *const libc::c_char
                    }
                    10 => {
                        p = b"crush\x00" as *const u8 as *const libc::c_char
                    }
                    11 => {
                        p = b"engulf\x00" as *const u8 as *const libc::c_char
                    }
                    12 => {
                        p = b"charge\x00" as *const u8 as *const libc::c_char
                    }
                    13 => {
                        p =
                            b"crawl on you\x00" as *const u8 as
                                *const libc::c_char
                    }
                    14 => {
                        p =
                            b"drool on you\x00" as *const u8 as
                                *const libc::c_char
                    }
                    15 => {
                        p = b"spit\x00" as *const u8 as *const libc::c_char
                    }
                    16 => {
                        p = b"explode\x00" as *const u8 as *const libc::c_char
                    }
                    17 => {
                        p = b"gaze\x00" as *const u8 as *const libc::c_char
                    }
                    18 => {
                        p = b"wail\x00" as *const u8 as *const libc::c_char
                    }
                    19 => {
                        p =
                            b"release spores\x00" as *const u8 as
                                *const libc::c_char
                    }
                    21 => {
                        p = b"beg\x00" as *const u8 as *const libc::c_char
                    }
                    22 => {
                        p = b"insult\x00" as *const u8 as *const libc::c_char
                    }
                    23 => {
                        p = b"moan\x00" as *const u8 as *const libc::c_char
                    }
                    24 => {
                        p = b"sing\x00" as *const u8 as *const libc::c_char
                    }
                    8 | 20 | _ => { }
                }
                /* Default effect */
                q = b"???\x00" as *const u8 as *const libc::c_char;
                /* Acquire the effect */
                match (*r_ptr).blow[j as usize].effect as libc::c_int {
                    1 => {
                        q = b"attack\x00" as *const u8 as *const libc::c_char
                    }
                    2 => {
                        q = b"poison\x00" as *const u8 as *const libc::c_char
                    }
                    3 => {
                        q =
                            b"disenchant\x00" as *const u8 as
                                *const libc::c_char
                    }
                    4 => {
                        q =
                            b"drain charges\x00" as *const u8 as
                                *const libc::c_char
                    }
                    5 => {
                        q =
                            b"steal gold\x00" as *const u8 as
                                *const libc::c_char
                    }
                    6 => {
                        q =
                            b"steal items\x00" as *const u8 as
                                *const libc::c_char
                    }
                    7 => {
                        q =
                            b"eat your food\x00" as *const u8 as
                                *const libc::c_char
                    }
                    8 => {
                        q =
                            b"absorb light\x00" as *const u8 as
                                *const libc::c_char
                    }
                    9 => {
                        q =
                            b"shoot acid\x00" as *const u8 as
                                *const libc::c_char
                    }
                    10 => {
                        q =
                            b"electrocute\x00" as *const u8 as
                                *const libc::c_char
                    }
                    11 => {
                        q = b"burn\x00" as *const u8 as *const libc::c_char
                    }
                    12 => {
                        q = b"freeze\x00" as *const u8 as *const libc::c_char
                    }
                    13 => {
                        q = b"blind\x00" as *const u8 as *const libc::c_char
                    }
                    14 => {
                        q = b"confuse\x00" as *const u8 as *const libc::c_char
                    }
                    15 => {
                        q = b"terrify\x00" as *const u8 as *const libc::c_char
                    }
                    16 => {
                        q =
                            b"paralyze\x00" as *const u8 as
                                *const libc::c_char
                    }
                    17 => {
                        q =
                            b"reduce strength\x00" as *const u8 as
                                *const libc::c_char
                    }
                    18 => {
                        q =
                            b"reduce intelligence\x00" as *const u8 as
                                *const libc::c_char
                    }
                    19 => {
                        q =
                            b"reduce wisdom\x00" as *const u8 as
                                *const libc::c_char
                    }
                    20 => {
                        q =
                            b"reduce dexterity\x00" as *const u8 as
                                *const libc::c_char
                    }
                    21 => {
                        q =
                            b"reduce constitution\x00" as *const u8 as
                                *const libc::c_char
                    }
                    22 => {
                        q =
                            b"reduce charisma\x00" as *const u8 as
                                *const libc::c_char
                    }
                    23 => {
                        q =
                            b"reduce all stats\x00" as *const u8 as
                                *const libc::c_char
                    }
                    24 => {
                        q = b"shatter\x00" as *const u8 as *const libc::c_char
                    }
                    25 => {
                        q =
                            b"lower experience (by 10d6+)\x00" as *const u8 as
                                *const libc::c_char
                    }
                    26 => {
                        q =
                            b"lower experience (by 20d6+)\x00" as *const u8 as
                                *const libc::c_char
                    }
                    27 => {
                        q =
                            b"lower experience (by 40d6+)\x00" as *const u8 as
                                *const libc::c_char
                    }
                    28 => {
                        q =
                            b"lower experience (by 80d6+)\x00" as *const u8 as
                                *const libc::c_char
                    }
                    29 => {
                        q = b"disease\x00" as *const u8 as *const libc::c_char
                    }
                    30 => {
                        q = b"time\x00" as *const u8 as *const libc::c_char
                    }
                    31 => {
                        q =
                            b"make insane\x00" as *const u8 as
                                *const libc::c_char
                    }
                    32 => {
                        q =
                            b"cause hallucinations\x00" as *const u8 as
                                *const libc::c_char
                    }
                    33 => {
                        q =
                            b"parasite\x00" as *const u8 as
                                *const libc::c_char
                    }
                    _ => { }
                }
                if k == 0 {
                    spoil_out(wd_che[msex as usize]);
                    spoil_out(b" can \x00" as *const u8 as
                                  *const libc::c_char);
                } else if k < i - 1 as libc::c_int {
                    spoil_out(b", \x00" as *const u8 as *const libc::c_char);
                } else {
                    spoil_out(b", and \x00" as *const u8 as
                                  *const libc::c_char);
                }
                /* Describe the method */
                spoil_out(p);
                /* Describe the effect, if any */
                if (*r_ptr).blow[j as usize].effect != 0 {
                    spoil_out(b" to \x00" as *const u8 as
                                  *const libc::c_char);
                    spoil_out(q);
                    if (*r_ptr).blow[j as usize].d_dice as libc::c_int != 0 &&
                           (*r_ptr).blow[j as usize].d_side as libc::c_int !=
                               0 {
                        spoil_out(b" with damage\x00" as *const u8 as
                                      *const libc::c_char);
                        if (*r_ptr).blow[j as usize].d_side as libc::c_int ==
                               1 as libc::c_int {
                            sprintf(buf.as_mut_ptr(),
                                    b" %d\x00" as *const u8 as
                                        *const libc::c_char,
                                    (*r_ptr).blow[j as usize].d_dice as
                                        libc::c_int);
                        } else {
                            sprintf(buf.as_mut_ptr(),
                                    b" %dd%d\x00" as *const u8 as
                                        *const libc::c_char,
                                    (*r_ptr).blow[j as usize].d_dice as
                                        libc::c_int,
                                    (*r_ptr).blow[j as usize].d_side as
                                        libc::c_int);
                        }
                        spoil_out(buf.as_mut_ptr() as cptr);
                    }
                }
                k += 1
            }
            j += 1
        }
        if k != 0 {
            spoil_out(b".  \x00" as *const u8 as *const libc::c_char);
        } else if flags1 & 0x10000 as libc::c_int as libc::c_uint != 0 {
            sprintf(buf.as_mut_ptr(),
                    b"%s has no physical attacks.  \x00" as *const u8 as
                        *const libc::c_char, wd_che[msex as usize]);
            spoil_out(buf.as_mut_ptr() as cptr);
        }
        spoil_out(0 as cptr);
        n += 1
    }
    /* Check for errors */
    if ferror(fff) != 0 || my_fclose(fff) != 0 {
        msg_print(b"Cannot close spoiler file.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    msg_print(b"Successfully created a spoiler file.\x00" as *const u8 as
                  *const libc::c_char);
}
#[no_mangle]
pub static mut long_intro: *mut libc::c_char =
    b"Essences are the tools of the trade for Alchemists, and unfortunately are useless for any other class. Alchemists use essences to create magical items for them to use.\n\nThey can be either found on the floor while exploring the dungeon, or extracted from other magical items the alchemist finds during their adventures.\n\nTo create an artifact, the alchemist will have to sacrifice 10 hit points, and an amount of magic essence similar to his skill in alchemy. The alchemist then allows the artifact to gain experience, and when it has enough, uses that experience to add abilities to the artifact. The alchemist can allow the artifact to continue to gain experience, thus keeping open the option to add more abilities later. This requires a similar amount of magic essence, but does not require the sacrifice of other hit points.\n\nNote that the experience you gain is divided among the artifacts that you have as well as going to yourself, so you will gain levels more slowly when empowering artifacts. Also, the artifact only gets 60% of the experience. So killing a creature worth 20xp would gain 10 for you, and 6 for the artifact.\n\nYou can also modify existing artifacts when you attain skill level 50. Also at skill level 50 you will gain the ability to make temporary artifacts, which don\'t require the complex empowerments that regular items require, but also vanish after awhile.\n\nYou cannot give an artifact an ability unless you have *Identified* an artifact which has that ability.\n\nFor every four levels gained in the alchemy skill, the alchemist learns about objects of level (skill level)/4, starting by learning about level 1 objects at skill level 0. (actually 1, but who\'s counting?)\n\nAt skill level 5 you gain the ability to make ego items - but watch it! Your base failure rate will be 90%, and won\'t be 0% until you reach skill level 50. Adding gold will increase the chances of success in direct proportion to the value of the item you are trying to create. Note that this results in automatic success when the item you are trying to create happens to pick up a curse in the process.\n\nAt skill level 5 you also gain knowledge of some basic ego item recipes. These are: Acidic, Shocking, Fiery, Frozen, Venomous, and Chaotic weapons, Resist Fire armour, and light sources of Fearlessness.\n\nAt skill level 10 you will gain knowledge of digging ego items, if you have selected the option \'always  generate very unusual rooms\' (ironman_rooms).\n\nAt skill level 15 you can create ego wands, staves, rings, etc.\n\nAt skill level 25 you gain the ability to empower artifacts and double ego items.\n\nAt skill level 50 you gain the ability to create temporary artifacts, which don\'t require any exotic ingredients beyond a single corpse of any type.\n\nBetween skill levels 25 and 50, you will steadily gain the ability to set more and more flags.\n\nTo finalise an artifact, you \'P\'ower it, and select the powers you want.\nPowers are divided into the following six categories:\n*****essences.txt*03[Stats, Sustains, Luck, Speed, Vision, etc.]\n*****essences.txt*04[Misc. (Auras, Light, See Invisibility, etc.)]\n*****essences.txt*05[Weapon Brands]\n*****essences.txt*06[Resistances and Immunities]\n*****essences.txt*07[ESP and Curses]\n*****essences.txt*08[Artifact Activations]\n\x00"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
/*
 * Create a spoiler file for essences
 */
unsafe extern "C" fn spoil_bateries(mut fname: cptr) {
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tval: libc::c_int = 0;
    let mut sval: libc::c_int = 0;
    let mut group: libc::c_int = 0;
    let mut forge: object_type =
        object_type{k_idx: 0,
                    iy: 0,
                    ix: 0,
                    tval: 0,
                    sval: 0,
                    pval: 0,
                    pval2: 0,
                    pval3: 0,
                    discount: 0,
                    number: 0,
                    weight: 0,
                    elevel: 0,
                    exp: 0,
                    name1: 0,
                    name2: 0,
                    name2b: 0,
                    xtra1: 0,
                    xtra2: 0,
                    to_h: 0,
                    to_d: 0,
                    to_a: 0,
                    ac: 0,
                    dd: 0,
                    ds: 0,
                    timeout: 0,
                    ident: 0,
                    marked: 0,
                    note: 0,
                    art_name: 0,
                    art_flags1: 0,
                    art_flags2: 0,
                    art_flags3: 0,
                    art_flags4: 0,
                    art_flags5: 0,
                    art_esp: 0,
                    art_oflags1: 0,
                    art_oflags2: 0,
                    art_oflags3: 0,
                    art_oflags4: 0,
                    art_oflags5: 0,
                    art_oesp: 0,
                    next_o_idx: 0,
                    held_m_idx: 0,
                    sense: 0,
                    found: 0,
                    found_aux1: 0,
                    found_aux2: 0,
                    found_aux3: 0,
                    found_aux4: 0,};
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    group = -(1 as libc::c_int);
    sval = group;
    tval = sval;
    /* Build the filename */
    path_build(buf.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int, ANGBAND_DIR_USER, fname);
    /* File type is "TEXT" */
    fff =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    /* Oops */
    if fff.is_null() {
        msg_print(b"Cannot create spoiler file.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Dump the header */
    fprintf(fff,
            b"|||||oy\n~~~~~01|Spoilers|Essences\n~~~~~02|Alchemist|Essence Spoiler\n#####REssence Spoiler for %s\n#####R-----------------------------------\n\n\x00"
                as *const u8 as *const libc::c_char, get_version_string());
    /*New code starts here -*/
	/*print header, including artifact header*/
	/*Cycle through artifact flags*/
	/*	print desc*/
	/*	cycle through all alchemist_recipies*/
	/*		print matching*/
	/*Print items header.*/
	/*Cycle through alchemist_recipies*/
	/*	sval or tval changed?*/
	/*		skip artifacts (tval=0)*/
	/*		print item desc (ego (tval=1) or item)*/
	/*	print essences required*/
	/*Done!*/
    /*Print basic header.*/
    spoil_out(long_intro as cptr);
    /*Cycle through artifact flags*/
    i = 0 as libc::c_int;
    while (*a_select_flags.offset(i as isize)).group != 0 {
        if (*a_select_flags.offset(i as isize)).group as libc::c_int != group
           {
            group = (*a_select_flags.offset(i as isize)).group as libc::c_int;
            spoil_out(b"\n~~~~~\x00" as *const u8 as *const libc::c_char);
            match group {
                1 => {
                    spoil_out(b"03\n#####GStats, Sustains, Luck, Speed, Vision, etc.\n\x00"
                                  as *const u8 as *const libc::c_char);
                }
                2 => {
                    spoil_out(b"04\n#####GMisc. (Auras, Light, See Invisibility, etc.)\n\x00"
                                  as *const u8 as *const libc::c_char);
                }
                3 => {
                    spoil_out(b"05\n#####GWeapon Brands\n\x00" as *const u8 as
                                  *const libc::c_char);
                }
                4 => {
                    spoil_out(b"06\n#####GResistances and Immunities\n\x00" as
                                  *const u8 as *const libc::c_char);
                }
                5 => {
                    spoil_out(b"07\n#####GESP and Curses\n\x00" as *const u8
                                  as *const libc::c_char);
                }
                88 => {
                    spoil_out(b"08\n#####GArtifact Activations\n\x00" as
                                  *const u8 as *const libc::c_char);
                }
                _ => {
                    spoil_out(format(b"09\n#####GExtra Group=%d\n\x00" as
                                         *const u8 as *const libc::c_char,
                                     group) as cptr);
                }
            }
            spoil_out(b"lvl     xp   Power\n\x00" as *const u8 as
                          *const libc::c_char);
        }
        /*	print desc*/
        spoil_out(format(b"%-2d %8d  %-24s %s\n\x00" as *const u8 as
                             *const libc::c_char,
                         (*a_select_flags.offset(i as isize)).level as
                             libc::c_int,
                         (*a_select_flags.offset(i as isize)).xp,
                         al_name.offset((*a_select_flags.offset(i as
                                                                    isize)).desc
                                            as isize),
                         al_name.offset((*a_select_flags.offset(i as
                                                                    isize)).item_desc
                                            as isize)) as cptr);
        /*	cycle through all alchemist_recipies*/
        j = 0 as libc::c_int;
        while j < max_al_idx as libc::c_int {
            /*		print matching*/
            if (*alchemist_recipes.offset(j as isize)).tval as libc::c_int ==
                   0 as libc::c_int &&
                   (*alchemist_recipes.offset(j as isize)).sval as libc::c_int
                       == (*a_select_flags.offset(i as isize)).flag &&
                   (*alchemist_recipes.offset(j as isize)).qty as libc::c_int
                       != 0 {
                spoil_out(format(b"    %d essences of %s\n\x00" as *const u8
                                     as *const libc::c_char,
                                 (*alchemist_recipes.offset(j as isize)).qty
                                     as libc::c_int,
                                 k_name.offset((*k_info.offset(lookup_kind(4
                                                                               as
                                                                               libc::c_int,
                                                                           (*alchemist_recipes.offset(j
                                                                                                          as
                                                                                                          isize)).sval_essence)
                                                                   as
                                                                   isize)).name
                                                   as isize)) as cptr);
            }
            j += 1
        }
        i += 1
    }
    spoil_out(b"\n\nThe following basic item recipes also exist:\n\x00" as
                  *const u8 as *const libc::c_char);
    let mut current_block_51: u64;
    /*Cycle through alchemist_recipies*/
    i = 0 as libc::c_int;
    while i < max_al_idx as libc::c_int {
        let mut ar_ptr: *mut alchemist_recipe =
            &mut *alchemist_recipes.offset(i as isize) as
                *mut alchemist_recipe;
        /*	sval or tval changed?*/
        if tval != (*ar_ptr).tval as libc::c_int ||
               sval != (*ar_ptr).sval as libc::c_int {
            let mut o_name: [libc::c_char; 80] = [0; 80];
            /*		skip artifacts (tval=0)*/
            if (*ar_ptr).tval as libc::c_int == 0 as libc::c_int {
                current_block_51 = 7659304154607701039;
            } else {
                tval = (*ar_ptr).tval as libc::c_int;
                sval = (*ar_ptr).sval as libc::c_int;
                /*		print item desc (ego (tval=1)or item)*/
                if (*ar_ptr).tval as libc::c_int == 1 as libc::c_int {
                    strcpy(o_name.as_mut_ptr(),
                           e_name.offset((*e_info.offset((*ar_ptr).sval as
                                                             isize)).name as
                                             isize));
                } else {
                    /* Find the name of that object */
                    o_ptr = &mut forge;
                    object_wipe(o_ptr);
                    object_prep(o_ptr,
                                lookup_kind(tval, sval) as libc::c_int);
                    apply_magic(o_ptr, 1 as libc::c_int,
                                0 as libc::c_int as bool_,
                                0 as libc::c_int as bool_,
                                0 as libc::c_int as bool_);
                    object_aware(o_ptr);
                    object_known(o_ptr);
                    (*o_ptr).name2b = 0 as libc::c_int as s16b;
                    (*o_ptr).name2 = (*o_ptr).name2b;
                    /* the 0 mode means only the text, leaving off any numbers */
                    object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                                0 as libc::c_int);
                }
                spoil_out(b"\n\x00" as *const u8 as *const libc::c_char);
                spoil_out(o_name.as_mut_ptr() as cptr);
                current_block_51 = 16415152177862271243;
            }
        } else { current_block_51 = 16415152177862271243; }
        match current_block_51 {
            16415152177862271243 => {
                /*	print essence required*/
                spoil_out(format(b" %d %s\x00" as *const u8 as
                                     *const libc::c_char,
                                 (*ar_ptr).qty as libc::c_int,
                                 k_name.offset((*k_info.offset(lookup_kind(4
                                                                               as
                                                                               libc::c_int,
                                                                           (*ar_ptr).sval_essence)
                                                                   as
                                                                   isize)).name
                                                   as isize)) as cptr);
            }
            _ => { }
        }
        i += 1
    }
    spoil_out(0 as cptr);
    /* Check for errors */
    if ferror(fff) != 0 || my_fclose(fff) != 0 {
        msg_print(b"Cannot close spoiler file.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Message */
    msg_print(b"Successfully created a spoiler file.\x00" as *const u8 as
                  *const libc::c_char);
}
/*
 * Print a bookless spell list
 */
#[no_mangle]
pub unsafe extern "C" fn print_magic_powers(mut powers: *mut magic_power,
                                            mut max_powers: libc::c_int,
                                            mut power_info:
                                                Option<unsafe extern "C" fn(_:
                                                                                *mut libc::c_char,
                                                                            _:
                                                                                libc::c_int)
                                                           -> ()>,
                                            mut skill_num: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut save_skill: libc::c_int = 0;
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut spell: magic_power =
        magic_power{min_lev: 0,
                    mana_cost: 0,
                    fail: 0,
                    name: 0 as *const libc::c_char,
                    desc: 0 as *const libc::c_char,};
    /* Use a maximal skill */
    save_skill = (*s_info.offset(skill_num as isize)).value;
    (*s_info.offset(skill_num as isize)).value = 50000 as libc::c_int;
    /* Dump the header line */
    spoiler_blanklines(2 as libc::c_int);
    sprintf(buf.as_mut_ptr(), b"%s\x00" as *const u8 as *const libc::c_char,
            s_name.offset((*s_info.offset(skill_num as isize)).name as
                              isize));
    spoiler_underline(buf.as_mut_ptr() as cptr);
    spoiler_blanklines(1 as libc::c_int);
    fprintf(fff,
            b"   Name                         Lvl Mana Fail Info\n\x00" as
                *const u8 as *const libc::c_char);
    /* Dump the spells */
    i = 0 as libc::c_int;
    while i < max_powers {
        /* Access the spell */
        spell = *powers.offset(i as isize);
        /* Get the additional info */
        power_info.expect("non-null function pointer")(buf.as_mut_ptr(), i);
        /* Dump the spell */
        spoil_out(format(b"%c) %-30s%2d %4d %3d%%%s\n\x00" as *const u8 as
                             *const libc::c_char, i + 'a' as i32, spell.name,
                         spell.min_lev, spell.mana_cost, spell.fail,
                         buf.as_mut_ptr()) as cptr);
        spoil_out(format(b"%s\n\x00" as *const u8 as *const libc::c_char,
                         spell.desc) as cptr);
        i += 1
    }
    /* Restore skill */
    (*s_info.offset(skill_num as isize)).value = save_skill;
}
/*
 * Create a spoiler file for spells
 */
unsafe extern "C" fn spoil_spells(mut fname: cptr) {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Build the filename */
    path_build(buf.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int, ANGBAND_DIR_USER, fname);
    /* File type is "TEXT" */
    fff =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    /* Oops */
    if fff.is_null() {
        msg_print(b"Cannot create spoiler file.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Dump the header */
    sprintf(buf.as_mut_ptr(),
            b"Spell Spoiler (Skill Level 50) for %s\x00" as *const u8 as
                *const libc::c_char, get_version_string());
    spoiler_underline(buf.as_mut_ptr() as cptr);
    /* Dump the bookless magic powers in alphabetical order */
    /* Mimicry */
    print_magic_powers(mimic_powers.as_mut_ptr(), 5 as libc::c_int,
                       Some(mimic_info as
                                unsafe extern "C" fn(_: *mut libc::c_char,
                                                     _: libc::c_int) -> ()),
                       32 as libc::c_int);
    /* Mindcraft */
    print_magic_powers(mindcraft_powers.as_mut_ptr(), 12 as libc::c_int,
                       Some(mindcraft_info as
                                unsafe extern "C" fn(_: *mut libc::c_char,
                                                     _: libc::c_int) -> ()),
                       29 as libc::c_int);
    /* Necromancy */
    print_magic_powers(necro_powers.as_mut_ptr(), 6 as libc::c_int,
                       Some(necro_info as
                                unsafe extern "C" fn(_: *mut libc::c_char,
                                                     _: libc::c_int) -> ()),
                       31 as libc::c_int);
    /* Symbiosis */
    print_magic_powers(symbiotic_powers.as_mut_ptr(), 9 as libc::c_int,
                       Some(symbiotic_info as
                                unsafe extern "C" fn(_: *mut libc::c_char,
                                                     _: libc::c_int) -> ()),
                       8 as libc::c_int);
    /* Check for errors */
    if ferror(fff) != 0 || my_fclose(fff) != 0 {
        msg_print(b"Cannot close spoiler file.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    /* Message */
    msg_print(b"Successfully created a spoiler file.\x00" as *const u8 as
                  *const libc::c_char);
}
/*
 * Forward declare
 */
/*
 * Create Spoiler files -BEN-
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_spoilers() {
    let mut i: libc::c_int = 0;
    /* Enter "icky" mode */
    character_icky = 1 as libc::c_int as bool_;
    /* Save the screen */
    Term_save();
    loop 
         /* Interact */
         /* Clear screen */
         {
        Term_clear();
        /* Info */
        prt(b"Create a spoiler file.\x00" as *const u8 as *const libc::c_char,
            2 as libc::c_int, 0 as libc::c_int);
        /* Prompt for a file */
        prt(b"(1) Brief Object Info  (obj-desc.spo)\x00" as *const u8 as
                *const libc::c_char, 5 as libc::c_int, 5 as libc::c_int);
        prt(b"(2) Full Artifact Info (artifact.spo)\x00" as *const u8 as
                *const libc::c_char, 6 as libc::c_int, 5 as libc::c_int);
        prt(b"(3) Brief Monster Info (mon-desc.spo)\x00" as *const u8 as
                *const libc::c_char, 7 as libc::c_int, 5 as libc::c_int);
        prt(b"(4) Full Monster Info  (mon-info.spo)\x00" as *const u8 as
                *const libc::c_char, 8 as libc::c_int, 5 as libc::c_int);
        prt(b"(5) Full Essences Info (ess-info.spo)\x00" as *const u8 as
                *const libc::c_char, 9 as libc::c_int, 5 as libc::c_int);
        prt(b"(6) Spell Info         (spell.spo)\x00" as *const u8 as
                *const libc::c_char, 10 as libc::c_int, 5 as libc::c_int);
        /* Prompt */
        prt(b"Command: \x00" as *const u8 as *const libc::c_char,
            12 as libc::c_int, 0 as libc::c_int);
        /* Get a choice */
        i = inkey() as libc::c_int;
        /* Escape */
        if i == '\u{1b}' as i32 { break ; }
        /* Option (1) */
        if i == '1' as i32 {
            spoil_obj_desc(b"obj-desc.spo\x00" as *const u8 as
                               *const libc::c_char);
        } else if i == '2' as i32 {
            spoil_artifact(b"artifact.spo\x00" as *const u8 as
                               *const libc::c_char);
        } else if i == '3' as i32 {
            spoil_mon_desc(b"mon-desc.spo\x00" as *const u8 as
                               *const libc::c_char);
        } else if i == '4' as i32 {
            spoil_mon_info(b"mon-info.spo\x00" as *const u8 as
                               *const libc::c_char);
        } else if i == '5' as i32 {
            spoil_bateries(b"ess-info.spo\x00" as *const u8 as
                               *const libc::c_char);
        } else if i == '6' as i32 {
            spoil_spells(b"spell.spo\x00" as *const u8 as
                             *const libc::c_char);
        } else {
            /* Option (2) */
            /* Option (3) */
            /* Option (4) */
            /* Option (5) */
            /* Option (6) */
            /* Oops */
            bell();
        }
        /* Flush messages */
        msg_print(0 as cptr);
    }
    /* Restore the screen */
    Term_load();
    /* Leave "icky" mode */
    character_icky = 0 as libc::c_int as bool_;
}
