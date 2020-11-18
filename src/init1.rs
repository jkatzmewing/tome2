use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stat_names: [cptr; 6];
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut object_level: s16b;
    #[no_mangle]
    static mut monster_level: s16b;
    #[no_mangle]
    static mut turn: s32b;
    #[no_mangle]
    static mut leaving_quest: libc::c_int;
    #[no_mangle]
    static mut player_base: [libc::c_char; 32];
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut m_list: *mut monster_type;
    #[no_mangle]
    static mut max_real_towns: u16b;
    #[no_mangle]
    static mut max_towns: u16b;
    #[no_mangle]
    static mut town_info: *mut town_type;
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut rp_ptr: *mut player_race;
    #[no_mangle]
    static mut rmp_ptr: *mut player_race_mod;
    #[no_mangle]
    static mut cp_ptr: *mut player_class;
    #[no_mangle]
    static mut alchemist_recipes: *mut alchemist_recipe;
    #[no_mangle]
    static mut al_head: *mut header;
    #[no_mangle]
    static mut al_name: *mut libc::c_char;
    #[no_mangle]
    static mut a_select_flags: *mut artifact_select_flag;
    #[no_mangle]
    static mut ab_head: *mut header;
    #[no_mangle]
    static mut ab_info: *mut ability_type;
    #[no_mangle]
    static mut ab_name: *mut libc::c_char;
    #[no_mangle]
    static mut ab_text: *mut libc::c_char;
    #[no_mangle]
    static mut s_head: *mut header;
    #[no_mangle]
    static mut s_info: *mut skill_type;
    #[no_mangle]
    static mut s_name: *mut libc::c_char;
    #[no_mangle]
    static mut s_text: *mut libc::c_char;
    #[no_mangle]
    static mut v_head: *mut header;
    #[no_mangle]
    static mut v_info: *mut vault_type;
    #[no_mangle]
    static mut v_name: *mut libc::c_char;
    #[no_mangle]
    static mut v_text: *mut libc::c_char;
    #[no_mangle]
    static mut f_head: *mut header;
    #[no_mangle]
    static mut f_info: *mut feature_type;
    #[no_mangle]
    static mut f_name: *mut libc::c_char;
    #[no_mangle]
    static mut f_text: *mut libc::c_char;
    #[no_mangle]
    static mut k_head: *mut header;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut k_name: *mut libc::c_char;
    #[no_mangle]
    static mut k_text: *mut libc::c_char;
    #[no_mangle]
    static mut a_head: *mut header;
    #[no_mangle]
    static mut a_info: *mut artifact_type;
    #[no_mangle]
    static mut a_name: *mut libc::c_char;
    #[no_mangle]
    static mut a_text: *mut libc::c_char;
    #[no_mangle]
    static mut e_head: *mut header;
    #[no_mangle]
    static mut e_info: *mut ego_item_type;
    #[no_mangle]
    static mut e_name: *mut libc::c_char;
    #[no_mangle]
    static mut ra_head: *mut header;
    #[no_mangle]
    static mut ra_info: *mut randart_part_type;
    #[no_mangle]
    static mut ra_gen: [randart_gen_type; 30];
    #[no_mangle]
    static mut r_head: *mut header;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut r_name: *mut libc::c_char;
    #[no_mangle]
    static mut r_text: *mut libc::c_char;
    #[no_mangle]
    static mut re_head: *mut header;
    #[no_mangle]
    static mut re_info: *mut monster_ego;
    #[no_mangle]
    static mut re_name: *mut libc::c_char;
    #[no_mangle]
    static mut d_head: *mut header;
    #[no_mangle]
    static mut d_info: *mut dungeon_info_type;
    #[no_mangle]
    static mut d_name: *mut libc::c_char;
    #[no_mangle]
    static mut d_text: *mut libc::c_char;
    #[no_mangle]
    static mut c_head: *mut header;
    #[no_mangle]
    static mut class_info: *mut player_class;
    #[no_mangle]
    static mut c_name: *mut libc::c_char;
    #[no_mangle]
    static mut c_text: *mut libc::c_char;
    #[no_mangle]
    static mut meta_class_info: *mut meta_class_type;
    #[no_mangle]
    static mut rp_head: *mut header;
    #[no_mangle]
    static mut race_info: *mut player_race;
    #[no_mangle]
    static mut rp_name: *mut libc::c_char;
    #[no_mangle]
    static mut rp_text: *mut libc::c_char;
    #[no_mangle]
    static mut rmp_head: *mut header;
    #[no_mangle]
    static mut race_mod_info: *mut player_race_mod;
    #[no_mangle]
    static mut rmp_name: *mut libc::c_char;
    #[no_mangle]
    static mut rmp_text: *mut libc::c_char;
    #[no_mangle]
    static mut t_head: *mut header;
    #[no_mangle]
    static mut t_info: *mut trap_type;
    #[no_mangle]
    static mut t_name: *mut libc::c_char;
    #[no_mangle]
    static mut t_text: *mut libc::c_char;
    #[no_mangle]
    static mut wf_head: *mut header;
    #[no_mangle]
    static mut wf_info: *mut wilderness_type_info;
    #[no_mangle]
    static mut wf_name: *mut libc::c_char;
    #[no_mangle]
    static mut wf_text: *mut libc::c_char;
    #[no_mangle]
    static mut wildc2i: [libc::c_int; 256];
    #[no_mangle]
    static mut st_head: *mut header;
    #[no_mangle]
    static mut st_info: *mut store_info_type;
    #[no_mangle]
    static mut st_name: *mut libc::c_char;
    #[no_mangle]
    static mut ba_head: *mut header;
    #[no_mangle]
    static mut ba_info: *mut store_action_type;
    #[no_mangle]
    static mut ba_name: *mut libc::c_char;
    #[no_mangle]
    static mut ow_head: *mut header;
    #[no_mangle]
    static mut ow_info: *mut owner_type;
    #[no_mangle]
    static mut ow_name: *mut libc::c_char;
    #[no_mangle]
    static mut set_head: *mut header;
    #[no_mangle]
    static mut set_info: *mut set_type;
    #[no_mangle]
    static mut set_name: *mut libc::c_char;
    #[no_mangle]
    static mut set_text: *mut libc::c_char;
    #[no_mangle]
    static mut ANGBAND_SYS: cptr;
    #[no_mangle]
    static mut ANGBAND_GRAF: cptr;
    #[no_mangle]
    static mut ANGBAND_DIR_EDIT: cptr;
    #[no_mangle]
    static mut pref_tmp_value: [libc::c_char; 8];
    #[no_mangle]
    static mut max_wild_x: u16b;
    #[no_mangle]
    static mut max_wild_y: u16b;
    #[no_mangle]
    static mut wild_map: *mut *mut wilderness_map;
    #[no_mangle]
    static mut max_ab_idx: u16b;
    #[no_mangle]
    static mut max_s_idx: u16b;
    #[no_mangle]
    static mut max_al_idx: u16b;
    #[no_mangle]
    static mut max_r_idx: u16b;
    #[no_mangle]
    static mut max_re_idx: u16b;
    #[no_mangle]
    static mut max_k_idx: u16b;
    #[no_mangle]
    static mut max_v_idx: u16b;
    #[no_mangle]
    static mut max_f_idx: u16b;
    #[no_mangle]
    static mut max_a_idx: u16b;
    #[no_mangle]
    static mut max_e_idx: u16b;
    #[no_mangle]
    static mut max_ra_idx: u16b;
    #[no_mangle]
    static mut max_d_idx: u16b;
    #[no_mangle]
    static mut max_o_idx: u16b;
    #[no_mangle]
    static mut max_m_idx: u16b;
    #[no_mangle]
    static mut max_t_idx: u16b;
    #[no_mangle]
    static mut max_rp_idx: u16b;
    #[no_mangle]
    static mut max_c_idx: u16b;
    #[no_mangle]
    static mut max_mc_idx: u16b;
    #[no_mangle]
    static mut max_rmp_idx: u16b;
    #[no_mangle]
    static mut max_st_idx: u16b;
    #[no_mangle]
    static mut max_ba_idx: u16b;
    #[no_mangle]
    static mut max_ow_idx: u16b;
    #[no_mangle]
    static mut max_wf_idx: u16b;
    #[no_mangle]
    static mut max_set_idx: s16b;
    #[no_mangle]
    static mut init_flags: libc::c_int;
    #[no_mangle]
    static mut m_allow_special: *mut bool_;
    #[no_mangle]
    static mut k_allow_special: *mut bool_;
    #[no_mangle]
    static mut a_allow_special: *mut bool_;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn streq(s: cptr, t: cptr) -> bool_;
    #[no_mangle]
    fn prefix(s: cptr, t: cptr) -> bool_;
    #[no_mangle]
    fn quit(str: cptr);
    #[no_mangle]
    fn strnfmt(buf: *mut libc::c_char, max: uint_hack, fmt: cptr, _: ...)
     -> uint_hack;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    static mut dungeon_flags1: u32b;
    #[no_mangle]
    static mut dungeon_flags2: u32b;
    #[no_mangle]
    static mut bg: *mut hist_type;
    #[no_mangle]
    static mut max_bg_idx: libc::c_int;
    #[no_mangle]
    static mut powers_type: *mut power_type;
    #[no_mangle]
    static mut power_max: s16b;
    #[no_mangle]
    static mut max_q_idx: s16b;
    #[no_mangle]
    static mut quest: *mut quest_type;
    #[no_mangle]
    static mut gen_skill_basem: [libc::c_char; 200];
    #[no_mangle]
    static mut gen_skill_base: [u32b; 200];
    #[no_mangle]
    static mut gen_skill_modm: [libc::c_char; 200];
    #[no_mangle]
    static mut gen_skill_mod: [s16b; 200];
    #[no_mangle]
    fn cave_set_feat(y: libc::c_int, x: libc::c_int, feat: libc::c_int);
    #[no_mangle]
    fn find_spell(name: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn tokenize(buf: *mut libc::c_char, num: s16b,
                tokens: *mut *mut libc::c_char, delim1: libc::c_char,
                delim2: libc::c_char) -> s16b;
    #[no_mangle]
    static mut error_idx: s16b;
    #[no_mangle]
    fn find_skill(name: cptr) -> s16b;
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn find_ability(name: cptr) -> s16b;
    #[no_mangle]
    fn find_god(name: cptr) -> libc::c_int;
    #[no_mangle]
    static mut fake_text_size: u32b;
    #[no_mangle]
    static mut fake_name_size: u32b;
    #[no_mangle]
    fn my_fopen(file: cptr, mode: cptr) -> *mut FILE;
    #[no_mangle]
    fn path_build(buf: *mut libc::c_char, max: libc::c_int, path: cptr,
                  file: cptr) -> errr;
    #[no_mangle]
    static mut error_line: s16b;
    #[no_mangle]
    fn my_fclose(fff: *mut FILE) -> errr;
    #[no_mangle]
    fn my_fgets(fff: *mut FILE, buf: *mut libc::c_char, n: huge_hack) -> errr;
    #[no_mangle]
    fn lookup_kind(tval: libc::c_int, sval: libc::c_int) -> s16b;
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn test_item_name(name: cptr) -> libc::c_int;
    #[no_mangle]
    fn drop_near(o_ptr: *mut object_type, chance: libc::c_int, y: libc::c_int,
                 x: libc::c_int) -> s16b;
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn random_artifact_resistance(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_prep(o_ptr: *mut object_type, k_idx: libc::c_int);
    #[no_mangle]
    fn object_wipe(o_ptr: *mut object_type);
    #[no_mangle]
    fn apply_magic(o_ptr: *mut object_type, lev: libc::c_int, okay: bool_,
                   good: bool_, great: bool_);
    #[no_mangle]
    fn place_trap(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn place_object(y: libc::c_int, x: libc::c_int, good: bool_, great: bool_,
                    where_0: libc::c_int);
    #[no_mangle]
    fn place_monster_aux(y: libc::c_int, x: libc::c_int, r_idx: libc::c_int,
                         slp: bool_, grp: bool_, status: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn place_monster(y: libc::c_int, x: libc::c_int, slp: bool_, grp: bool_)
     -> bool_;
    #[no_mangle]
    fn bst(what: s32b, t: s32b) -> s32b;
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
pub type cptr = *const libc::c_char;
pub type errr = libc::c_int;
pub type byte_hack = libc::c_uchar;
pub type bool_ = libc::c_char;
pub type uint_hack = libc::c_uint;
pub type huge_hack = libc::c_ulong;
pub type s16b = libc::c_short;
pub type u16b = libc::c_ushort;
pub type s32b = libc::c_int;
pub type u32b = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct header {
    pub info_num: u16b,
    pub name_size: u32b,
    pub text_size: u32b,
}
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
pub struct feature_type {
    pub name: u32b,
    pub text: u32b,
    pub tunnel: u32b,
    pub block: u32b,
    pub mimic: byte_hack,
    pub flags1: u32b,
    pub extra: byte_hack,
    pub unused: s16b,
    pub d_attr: byte_hack,
    pub d_char: libc::c_char,
    pub x_attr: byte_hack,
    pub x_char: libc::c_char,
    pub shimmer: [byte_hack; 7],
    pub d_dice: [libc::c_int; 4],
    pub d_side: [libc::c_int; 4],
    pub d_frequency: [libc::c_int; 4],
    pub d_type: [libc::c_int; 4],
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
pub struct randart_part_type {
    pub tval: [byte_hack; 20],
    pub min_sval: [byte_hack; 20],
    pub max_sval: [byte_hack; 20],
    pub level: byte_hack,
    pub rarity: byte_hack,
    pub mrarity: byte_hack,
    pub max_to_h: s16b,
    pub max_to_d: s16b,
    pub max_to_a: s16b,
    pub max_pval: s32b,
    pub value: s32b,
    pub max: s16b,
    pub flags1: u32b,
    pub flags2: u32b,
    pub flags3: u32b,
    pub flags4: u32b,
    pub flags5: u32b,
    pub esp: u32b,
    pub fego: u32b,
    pub aflags1: u32b,
    pub aflags2: u32b,
    pub aflags3: u32b,
    pub aflags4: u32b,
    pub aflags5: u32b,
    pub aesp: u32b,
    pub power: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct randart_gen_type {
    pub chance: libc::c_int,
    pub dd: libc::c_int,
    pub ds: libc::c_int,
    pub plus: libc::c_int,
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
pub struct monster_ego {
    pub name: u32b,
    pub before: bool_,
    pub blow: [monster_blow; 4],
    pub blowm: [[byte_hack; 2]; 4],
    pub hdice: s16b,
    pub hside: s16b,
    pub ac: s16b,
    pub sleep: s16b,
    pub aaf: s16b,
    pub speed: s16b,
    pub mexp: s32b,
    pub weight: s32b,
    pub freq_inate: byte_hack,
    pub freq_spell: byte_hack,
    pub flags1: u32b,
    pub flags2: u32b,
    pub flags3: u32b,
    pub flags7: u32b,
    pub flags8: u32b,
    pub flags9: u32b,
    pub hflags1: u32b,
    pub hflags2: u32b,
    pub hflags3: u32b,
    pub hflags7: u32b,
    pub hflags8: u32b,
    pub hflags9: u32b,
    pub mflags1: u32b,
    pub mflags2: u32b,
    pub mflags3: u32b,
    pub mflags4: u32b,
    pub mflags5: u32b,
    pub mflags6: u32b,
    pub mflags7: u32b,
    pub mflags8: u32b,
    pub mflags9: u32b,
    pub nflags1: u32b,
    pub nflags2: u32b,
    pub nflags3: u32b,
    pub nflags4: u32b,
    pub nflags5: u32b,
    pub nflags6: u32b,
    pub nflags7: u32b,
    pub nflags8: u32b,
    pub nflags9: u32b,
    pub level: s16b,
    pub rarity: s16b,
    pub d_attr: byte_hack,
    pub d_char: libc::c_char,
    pub g_attr: byte_hack,
    pub g_char: libc::c_char,
    pub r_char: [libc::c_char; 5],
    pub nr_char: [libc::c_char; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vault_type {
    pub name: u32b,
    pub text: u32b,
    pub typ: byte_hack,
    pub rat: byte_hack,
    pub hgt: byte_hack,
    pub wid: byte_hack,
    pub lvl: s16b,
    pub dun_type: byte_hack,
    pub mon: [s16b; 10],
    pub item: [libc::c_int; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trap_type {
    pub probability: s16b,
    pub another: s16b,
    pub p1valinc: s16b,
    pub difficulty: byte_hack,
    pub minlevel: byte_hack,
    pub color: byte_hack,
    pub flags: u32b,
    pub ident: bool_,
    pub known: s16b,
    pub name: s16b,
    pub dd: s16b,
    pub ds: s16b,
    pub text: s16b,
    pub g_attr: byte_hack,
    pub g_char: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cave_type {
    pub info: u16b,
    pub feat: byte_hack,
    pub o_idx: s16b,
    pub m_idx: s16b,
    pub t_idx: s16b,
    pub special: s16b,
    pub special2: s16b,
    pub inscription: s16b,
    pub mana: byte_hack,
    pub mimic: byte_hack,
    pub cost: byte_hack,
    pub when: byte_hack,
    pub effect: s16b,
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
pub struct monster_mind {
    pub dummy: byte_hack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monster_type {
    pub r_idx: s16b,
    pub ego: u16b,
    pub fy: byte_hack,
    pub fx: byte_hack,
    pub hp: s32b,
    pub maxhp: s32b,
    pub blow: [monster_blow; 4],
    pub speed: byte_hack,
    pub level: byte_hack,
    pub ac: s16b,
    pub exp: u32b,
    pub csleep: s16b,
    pub mspeed: byte_hack,
    pub energy: byte_hack,
    pub stunned: byte_hack,
    pub confused: byte_hack,
    pub monfear: byte_hack,
    pub bleeding: s16b,
    pub poisoned: s16b,
    pub cdis: byte_hack,
    pub mflag: s32b,
    pub ml: bool_,
    pub hold_o_idx: s16b,
    pub smart: u32b,
    pub status: s16b,
    pub target: s16b,
    pub possessor: s16b,
    pub sr_ptr: *mut monster_race,
    pub mind: *mut monster_mind,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct owner_type {
    pub name: u32b,
    pub max_cost: s16b,
    pub max_inflate: byte_hack,
    pub min_inflate: byte_hack,
    pub haggle_per: byte_hack,
    pub insult_max: byte_hack,
    pub races: [[u32b; 2]; 2],
    pub classes: [[u32b; 2]; 2],
    pub costs: [s16b; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct store_type {
    pub st_idx: u16b,
    pub owner: u16b,
    pub insult_cur: s16b,
    pub good_buy: s16b,
    pub bad_buy: s16b,
    pub store_open: s32b,
    pub last_visit: s32b,
    pub stock_num: byte_hack,
    pub stock_size: s16b,
    pub stock: *mut object_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct store_info_type {
    pub name: u32b,
    pub table: [[s16b; 2]; 56],
    pub table_num: byte_hack,
    pub max_obj: s16b,
    pub owners: [u16b; 4],
    pub actions: [u16b; 6],
    pub d_attr: byte_hack,
    pub d_char: libc::c_char,
    pub x_attr: byte_hack,
    pub x_char: libc::c_char,
    pub flags1: u32b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct store_action_type {
    pub name: u32b,
    pub costs: [s16b; 3],
    pub letter: libc::c_char,
    pub letter_aux: libc::c_char,
    pub action: s16b,
    pub action_restr: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_race {
    pub title: s32b,
    pub desc: s32b,
    pub r_adj: [s16b; 6],
    pub luck: libc::c_char,
    pub r_dis: s16b,
    pub r_dev: s16b,
    pub r_sav: s16b,
    pub r_stl: s16b,
    pub r_srh: s16b,
    pub r_fos: s16b,
    pub r_thn: s16b,
    pub r_thb: s16b,
    pub r_mhp: byte_hack,
    pub r_exp: u16b,
    pub b_age: byte_hack,
    pub m_age: byte_hack,
    pub m_b_ht: byte_hack,
    pub m_m_ht: byte_hack,
    pub m_b_wt: byte_hack,
    pub m_m_wt: byte_hack,
    pub f_b_ht: byte_hack,
    pub f_m_ht: byte_hack,
    pub f_b_wt: byte_hack,
    pub f_m_wt: byte_hack,
    pub infra: byte_hack,
    pub choice: [u32b; 2],
    pub powers: [s16b; 4],
    pub body_parts: [byte_hack; 6],
    pub chart: s16b,
    pub flags1: u32b,
    pub flags2: u32b,
    pub oflags1: [u32b; 51],
    pub oflags2: [u32b; 51],
    pub oflags3: [u32b; 51],
    pub oflags4: [u32b; 51],
    pub oflags5: [u32b; 51],
    pub oesp: [u32b; 51],
    pub opval: [s16b; 51],
    pub skill_basem: [libc::c_char; 200],
    pub skill_base: [u32b; 200],
    pub skill_modm: [libc::c_char; 200],
    pub skill_mod: [s16b; 200],
    pub obj_tval: [s16b; 5],
    pub obj_sval: [s16b; 5],
    pub obj_pval: [s16b; 5],
    pub obj_dd: [s16b; 5],
    pub obj_ds: [s16b; 5],
    pub obj_num: s16b,
    pub abilities: [C2RustUnnamed_0; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub ability: s16b,
    pub level: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_race_mod {
    pub title: s32b,
    pub desc: s32b,
    pub place: bool_,
    pub r_adj: [s16b; 6],
    pub luck: libc::c_char,
    pub mana: s16b,
    pub r_dis: s16b,
    pub r_dev: s16b,
    pub r_sav: s16b,
    pub r_stl: s16b,
    pub r_srh: s16b,
    pub r_fos: s16b,
    pub r_thn: s16b,
    pub r_thb: s16b,
    pub r_mhp: libc::c_char,
    pub r_exp: s16b,
    pub b_age: libc::c_char,
    pub m_age: libc::c_char,
    pub m_b_ht: libc::c_char,
    pub m_m_ht: libc::c_char,
    pub m_b_wt: libc::c_char,
    pub m_m_wt: libc::c_char,
    pub f_b_ht: libc::c_char,
    pub f_m_ht: libc::c_char,
    pub f_b_wt: libc::c_char,
    pub f_m_wt: libc::c_char,
    pub infra: libc::c_char,
    pub choice: [u32b; 2],
    pub pclass: [u32b; 2],
    pub mclass: [u32b; 2],
    pub powers: [s16b; 4],
    pub body_parts: [libc::c_char; 6],
    pub flags1: u32b,
    pub flags2: u32b,
    pub oflags1: [u32b; 51],
    pub oflags2: [u32b; 51],
    pub oflags3: [u32b; 51],
    pub oflags4: [u32b; 51],
    pub oflags5: [u32b; 51],
    pub oesp: [u32b; 51],
    pub opval: [s16b; 51],
    pub g_attr: byte_hack,
    pub g_char: libc::c_char,
    pub skill_basem: [libc::c_char; 200],
    pub skill_base: [u32b; 200],
    pub skill_modm: [libc::c_char; 200],
    pub skill_mod: [s16b; 200],
    pub obj_tval: [s16b; 5],
    pub obj_sval: [s16b; 5],
    pub obj_pval: [s16b; 5],
    pub obj_dd: [s16b; 5],
    pub obj_ds: [s16b; 5],
    pub obj_num: s16b,
    pub abilities: [C2RustUnnamed_1; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub ability: s16b,
    pub level: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_spec {
    pub title: s32b,
    pub desc: s32b,
    pub skill_basem: [libc::c_char; 200],
    pub skill_base: [u32b; 200],
    pub skill_modm: [libc::c_char; 200],
    pub skill_mod: [s16b; 200],
    pub skill_ideal: [u32b; 200],
    pub obj_tval: [s16b; 5],
    pub obj_sval: [s16b; 5],
    pub obj_pval: [s16b; 5],
    pub obj_dd: [s16b; 5],
    pub obj_ds: [s16b; 5],
    pub obj_num: s16b,
    pub gods: u32b,
    pub flags1: u32b,
    pub flags2: u32b,
    pub abilities: [C2RustUnnamed_2; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub ability: s16b,
    pub level: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_class {
    pub title: s32b,
    pub desc: s32b,
    pub titles: [s32b; 10],
    pub c_adj: [s16b; 6],
    pub c_dis: s16b,
    pub c_dev: s16b,
    pub c_sav: s16b,
    pub c_stl: s16b,
    pub c_srh: s16b,
    pub c_fos: s16b,
    pub c_thn: s16b,
    pub c_thb: s16b,
    pub x_dis: s16b,
    pub x_dev: s16b,
    pub x_sav: s16b,
    pub x_stl: s16b,
    pub x_srh: s16b,
    pub x_fos: s16b,
    pub x_thn: s16b,
    pub x_thb: s16b,
    pub c_mhp: s16b,
    pub c_exp: s16b,
    pub powers: [s16b; 4],
    pub spell_book: s16b,
    pub spell_stat: s16b,
    pub spell_lev: s16b,
    pub spell_fail: s16b,
    pub spell_mana: s16b,
    pub spell_first: s16b,
    pub spell_weight: s16b,
    pub max_spell_level: byte_hack,
    pub magic_max_spell: byte_hack,
    pub flags1: u32b,
    pub flags2: u32b,
    pub mana: s16b,
    pub blow_num: s16b,
    pub blow_wgt: s16b,
    pub blow_mul: s16b,
    pub extra_blows: s16b,
    pub sense_base: s32b,
    pub sense_pl: s32b,
    pub sense_plus: s32b,
    pub sense_heavy: byte_hack,
    pub sense_heavy_magic: byte_hack,
    pub obj_tval: [s16b; 5],
    pub obj_sval: [s16b; 5],
    pub obj_pval: [s16b; 5],
    pub obj_dd: [s16b; 5],
    pub obj_ds: [s16b; 5],
    pub obj_num: s16b,
    pub body_parts: [libc::c_char; 6],
    pub oflags1: [u32b; 51],
    pub oflags2: [u32b; 51],
    pub oflags3: [u32b; 51],
    pub oflags4: [u32b; 51],
    pub oflags5: [u32b; 51],
    pub oesp: [u32b; 51],
    pub opval: [s16b; 51],
    pub skill_basem: [libc::c_char; 200],
    pub skill_base: [u32b; 200],
    pub skill_modm: [libc::c_char; 200],
    pub skill_mod: [s16b; 200],
    pub gods: u32b,
    pub spec: [player_spec; 20],
    pub abilities: [C2RustUnnamed_3; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub ability: s16b,
    pub level: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct meta_class_type {
    pub name: [libc::c_char; 80],
    pub color: byte_hack,
    pub classes: *mut s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct help_info {
    pub enabled: bool_,
    pub help1: u32b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_type {
    pub lives: s32b,
    pub oldpy: s16b,
    pub oldpx: s16b,
    pub py: s16b,
    pub px: s16b,
    pub psex: byte_hack,
    pub prace: byte_hack,
    pub pracem: byte_hack,
    pub pclass: byte_hack,
    pub pspec: byte_hack,
    pub mimic_form: byte_hack,
    pub mimic_level: s16b,
    pub oops: byte_hack,
    pub inventory: [object_type; 52],
    pub hitdie: byte_hack,
    pub expfact: u16b,
    pub maximize: byte_hack,
    pub preserve: byte_hack,
    pub special: byte_hack,
    pub allow_one_death: byte_hack,
    pub age: s16b,
    pub ht: s16b,
    pub wt: s16b,
    pub sc: s16b,
    pub au: s32b,
    pub max_exp: s32b,
    pub exp: s32b,
    pub exp_frac: u16b,
    pub lev: s16b,
    pub town_num: s16b,
    pub arena_number: s16b,
    pub inside_arena: s16b,
    pub inside_quest: s16b,
    pub exit_bldg: bool_,
    pub wilderness_x: s32b,
    pub wilderness_y: s32b,
    pub wild_mode: bool_,
    pub old_wild_mode: bool_,
    pub mhp: s16b,
    pub chp: s16b,
    pub chp_frac: u16b,
    pub hp_mod: s16b,
    pub msp: s16b,
    pub csp: s16b,
    pub csp_frac: u16b,
    pub msane: s16b,
    pub csane: s16b,
    pub csane_frac: u16b,
    pub grace: s32b,
    pub pgod: byte_hack,
    pub praying: bool_,
    pub melkor_sacrifice: s16b,
    pub max_plv: s16b,
    pub stat_max: [s16b; 6],
    pub stat_cur: [s16b; 6],
    pub luck_cur: s16b,
    pub luck_max: s16b,
    pub luck_base: s16b,
    pub speed_factor: s16b,
    pub fast: s16b,
    pub lightspeed: s16b,
    pub slow: s16b,
    pub blind: s16b,
    pub paralyzed: s16b,
    pub confused: s16b,
    pub afraid: s16b,
    pub image: s16b,
    pub poisoned: s16b,
    pub cut: s16b,
    pub stun: s16b,
    pub protevil: s16b,
    pub protgood: s16b,
    pub protundead: s16b,
    pub invuln: s16b,
    pub hero: s16b,
    pub shero: s16b,
    pub shield: s16b,
    pub shield_power: s16b,
    pub shield_opt: s16b,
    pub shield_power_opt: s16b,
    pub shield_power_opt2: s16b,
    pub blessed: s16b,
    pub tim_invis: s16b,
    pub tim_infra: s16b,
    pub oppose_acid: s16b,
    pub oppose_elec: s16b,
    pub oppose_fire: s16b,
    pub oppose_cold: s16b,
    pub oppose_pois: s16b,
    pub oppose_ld: s16b,
    pub oppose_cc: s16b,
    pub oppose_ss: s16b,
    pub oppose_nex: s16b,
    pub rush: s16b,
    pub tim_esp: s16b,
    pub tim_wraith: s16b,
    pub tim_ffall: s16b,
    pub tim_fly: s16b,
    pub tim_fire_aura: s16b,
    pub tim_poison: s16b,
    pub tim_thunder: s16b,
    pub tim_thunder_p1: s16b,
    pub tim_thunder_p2: s16b,
    pub tim_project: s16b,
    pub tim_project_dam: s16b,
    pub tim_project_gf: s16b,
    pub tim_project_rad: s16b,
    pub tim_project_flag: s16b,
    pub tim_roots: s16b,
    pub tim_roots_ac: s16b,
    pub tim_roots_dam: s16b,
    pub resist_magic: s16b,
    pub tim_invisible: s16b,
    pub tim_inv_pow: s16b,
    pub tim_mimic: s16b,
    pub tim_lite: s16b,
    pub tim_regen: s16b,
    pub tim_regen_pow: s16b,
    pub holy: s16b,
    pub walk_water: s16b,
    pub tim_mental_barrier: s16b,
    pub strike: s16b,
    pub meditation: s16b,
    pub tim_reflect: s16b,
    pub tim_res_time: s16b,
    pub tim_deadly: s16b,
    pub prob_travel: s16b,
    pub disrupt_shield: s16b,
    pub parasite: s16b,
    pub parasite_r_idx: s16b,
    pub loan: s32b,
    pub loan_time: s32b,
    pub absorb_soul: s16b,
    pub tim_magic_breath: s16b,
    pub tim_water_breath: s16b,
    pub immov_cntr: s16b,
    pub chaos_patron: s16b,
    pub recall_dungeon: s16b,
    pub word_recall: s16b,
    pub energy: s32b,
    pub food: s16b,
    pub confusing: byte_hack,
    pub searching: byte_hack,
    pub new_spells: s16b,
    pub old_spells: s16b,
    pub xtra_spells: s16b,
    pub old_cumber_armor: bool_,
    pub old_cumber_glove: bool_,
    pub old_heavy_wield: bool_,
    pub old_heavy_shoot: bool_,
    pub old_icky_wield: bool_,
    pub old_lite: s16b,
    pub old_view: s16b,
    pub old_food_aux: s16b,
    pub cumber_armor: bool_,
    pub cumber_glove: bool_,
    pub heavy_wield: bool_,
    pub heavy_shoot: bool_,
    pub icky_wield: bool_,
    pub immovable: bool_,
    pub cur_lite: s16b,
    pub notice: u32b,
    pub update: u32b,
    pub redraw: u32b,
    pub window: u32b,
    pub stat_use: [s16b; 6],
    pub stat_top: [s16b; 6],
    pub stat_add: [s16b; 6],
    pub stat_ind: [s16b; 6],
    pub stat_cnt: [s16b; 6],
    pub stat_los: [s16b; 6],
    pub immune_acid: bool_,
    pub immune_elec: bool_,
    pub immune_fire: bool_,
    pub immune_cold: bool_,
    pub immune_neth: bool_,
    pub resist_acid: bool_,
    pub resist_elec: bool_,
    pub resist_fire: bool_,
    pub resist_cold: bool_,
    pub resist_pois: bool_,
    pub resist_conf: bool_,
    pub resist_sound: bool_,
    pub resist_lite: bool_,
    pub resist_dark: bool_,
    pub resist_chaos: bool_,
    pub resist_disen: bool_,
    pub resist_shard: bool_,
    pub resist_nexus: bool_,
    pub resist_blind: bool_,
    pub resist_neth: bool_,
    pub resist_fear: bool_,
    pub resist_continuum: bool_,
    pub sensible_fire: bool_,
    pub sensible_lite: bool_,
    pub reflect: bool_,
    pub sh_fire: bool_,
    pub sh_elec: bool_,
    pub wraith_form: bool_,
    pub anti_magic: bool_,
    pub anti_tele: bool_,
    pub sustain_str: bool_,
    pub sustain_int: bool_,
    pub sustain_wis: bool_,
    pub sustain_dex: bool_,
    pub sustain_con: bool_,
    pub sustain_chr: bool_,
    pub aggravate: bool_,
    pub teleport: bool_,
    pub exp_drain: bool_,
    pub drain_mana: byte_hack,
    pub drain_life: byte_hack,
    pub magical_breath: bool_,
    pub water_breath: bool_,
    pub climb: bool_,
    pub fly: bool_,
    pub ffall: bool_,
    pub lite: bool_,
    pub free_act: bool_,
    pub see_inv: bool_,
    pub regenerate: bool_,
    pub hold_life: bool_,
    pub telepathy: u32b,
    pub slow_digest: bool_,
    pub bless_blade: bool_,
    pub xtra_might: byte_hack,
    pub impact: bool_,
    pub auto_id: bool_,
    pub invis: s16b,
    pub dis_to_h: s16b,
    pub dis_to_d: s16b,
    pub dis_to_a: s16b,
    pub dis_ac: s16b,
    pub to_l: s16b,
    pub to_m: s16b,
    pub to_s: s16b,
    pub to_h: s16b,
    pub to_d: s16b,
    pub to_h_melee: s16b,
    pub to_d_melee: s16b,
    pub to_h_ranged: s16b,
    pub to_d_ranged: s16b,
    pub to_a: s16b,
    pub ac: s16b,
    pub antimagic: byte_hack,
    pub antimagic_dis: byte_hack,
    pub see_infra: s16b,
    pub skill_dis: s16b,
    pub skill_dev: s16b,
    pub skill_sav: s16b,
    pub skill_stl: s16b,
    pub skill_srh: s16b,
    pub skill_fos: s16b,
    pub skill_thn: s16b,
    pub skill_thb: s16b,
    pub skill_tht: s16b,
    pub skill_dig: s16b,
    pub num_blow: s16b,
    pub num_fire: s16b,
    pub xtra_crit: s16b,
    pub throw_mult: byte_hack,
    pub tval_xtra: byte_hack,
    pub tval_ammo: byte_hack,
    pub pspeed: s16b,
    pub mimic_extra: u32b,
    pub antimagic_extra: u32b,
    pub druid_extra: u32b,
    pub druid_extra2: u32b,
    pub druid_extra3: u32b,
    pub music_extra: u32b,
    pub music_extra2: u32b,
    pub necro_extra: u32b,
    pub necro_extra2: u32b,
    pub race_extra1: u32b,
    pub race_extra2: u32b,
    pub race_extra3: u32b,
    pub race_extra4: u32b,
    pub race_extra5: u32b,
    pub race_extra6: u32b,
    pub race_extra7: u32b,
    pub dodge_chance: s16b,
    pub maintain_sum: u32b,
    pub spellbinder_num: byte_hack,
    pub spellbinder: [u32b; 4],
    pub spellbinder_trigger: byte_hack,
    pub mimic_name: cptr,
    pub tactic: libc::c_char,
    pub movement: libc::c_char,
    pub companion_killed: s16b,
    pub no_mortal: bool_,
    pub black_breath: bool_,
    pub precognition: bool_,
    pub xtra_f1: u32b,
    pub xtra_f2: u32b,
    pub xtra_f3: u32b,
    pub xtra_f4: u32b,
    pub xtra_f5: u32b,
    pub xtra_esp: u32b,
    pub corruptions: *mut bool_,
    pub pet_follow_distance: byte_hack,
    pub pet_open_doors: byte_hack,
    pub pet_pickup_items: byte_hack,
    pub control: s16b,
    pub control_dir: byte_hack,
    pub body_monster: u16b,
    pub disembodied: bool_,
    pub body_parts: [byte_hack; 28],
    pub astral: bool_,
    pub powers: *mut bool_,
    pub powers_mod: [bool_; 62],
    pub skill_points: s16b,
    pub skill_last_level: s16b,
    pub melee_style: s16b,
    pub use_piercing_shots: s16b,
    pub help: help_info,
    pub did_nothing: bool_,
    pub leaving: bool_,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wilderness_type_info {
    pub name: u32b,
    pub text: u32b,
    pub entrance: u16b,
    pub wild_x: s32b,
    pub wild_y: s32b,
    pub road: byte_hack,
    pub level: libc::c_int,
    pub flags1: u32b,
    pub feat: byte_hack,
    pub terrain_idx: byte_hack,
    pub terrain: [byte_hack; 18],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wilderness_map {
    pub feat: libc::c_int,
    pub seed: u32b,
    pub entrance: u16b,
    pub known: bool_,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct town_type {
    pub name: cptr,
    pub seed: u32b,
    pub store: *mut store_type,
    pub numstores: byte_hack,
    pub flags: byte_hack,
    pub stocked: bool_,
    pub destroyed: bool_,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct power_type {
    pub name: *mut libc::c_char,
    pub desc_text: *mut libc::c_char,
    pub gain_text: *mut libc::c_char,
    pub lose_text: *mut libc::c_char,
    pub level: byte_hack,
    pub cost: byte_hack,
    pub stat: byte_hack,
    pub diff: byte_hack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quest_type {
    pub silent: bool_,
    pub dynamic_desc: bool_,
    pub name: [libc::c_char; 40],
    pub desc: [[libc::c_char; 80]; 10],
    pub status: s16b,
    pub level: s16b,
    pub plot: *mut s16b,
    pub type_0: byte_hack,
    pub init: Option<unsafe extern "C" fn(_: libc::c_int) -> bool_>,
    pub data: [s32b; 4],
    pub gen_desc: Option<unsafe extern "C" fn(_: *mut FILE) -> bool_>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hist_type {
    pub info: s32b,
    pub roll: byte_hack,
    pub chart: s16b,
    pub next: s16b,
    pub bonus: byte_hack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct set_type {
    pub name: u32b,
    pub desc: u32b,
    pub num: byte_hack,
    pub num_use: byte_hack,
    pub arts: [C2RustUnnamed_4; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub present: bool_,
    pub a_idx: s16b,
    pub pval: [s16b; 6],
    pub flags1: [u32b; 6],
    pub flags2: [u32b; 6],
    pub flags3: [u32b; 6],
    pub flags4: [u32b; 6],
    pub flags5: [u32b; 6],
    pub esp: [u32b; 6],
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
pub struct ability_type {
    pub name: u32b,
    pub desc: u32b,
    pub action_desc: u32b,
    pub action_mkey: s16b,
    pub cost: s16b,
    pub acquired: bool_,
    pub skills: [s16b; 10],
    pub skill_levels: [s16b; 10],
    pub stat: [s16b; 6],
    pub need_abilities: [s16b; 10],
    pub forbid_abilities: [s16b; 10],
}
/*
 * Dungeon effect types (used in E:damage:frequency:type entry in d_info.txt)
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub name: cptr,
    pub feat: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dungeon_grid {
    pub feature: libc::c_int,
    pub monster: libc::c_int,
    pub object: libc::c_int,
    pub ego: libc::c_int,
    pub artifact: libc::c_int,
    pub trap: libc::c_int,
    pub cave_info: libc::c_int,
    pub special: libc::c_int,
    pub random: libc::c_int,
    pub bx: libc::c_int,
    pub by: libc::c_int,
    pub mimic: libc::c_int,
    pub mflag: s32b,
    pub ok: bool_,
    pub defined: bool_,
}
/* File: init1.c */
/* Purpose: Initialization (part 1) -BEN- */
/*
 * This file is used to initialize various variables and arrays for the
 * Angband game.  Note the use of "fd_read()" and "fd_write()" to bypass
 * the common limitation of "read()" and "write()" to only 32767 bytes
 * at a time.
 *
 * Several of the arrays for Angband are built from "template" files in
 * the "lib/file" directory, from which quick-load binary "image" files
 * are constructed whenever they are not present in the "lib/data"
 * directory, or if those files become obsolete, if we are allowed.
 *
 * Warning -- the "ascii" file parsers use a minor hack to collect the
 * name and text information in a single pass.  Thus, the game will not
 * be able to load any template file with more than 20K of names or 60K
 * of text, even though technically, up to 64K should be legal.
 */
/* ** Helper arrays for parsing ascii template files ***/
/*
 * Monster Blow Methods
 */
static mut r_info_blow_method: [cptr; 26] =
    [b"*\x00" as *const u8 as *const libc::c_char,
     b"HIT\x00" as *const u8 as *const libc::c_char,
     b"TOUCH\x00" as *const u8 as *const libc::c_char,
     b"PUNCH\x00" as *const u8 as *const libc::c_char,
     b"KICK\x00" as *const u8 as *const libc::c_char,
     b"CLAW\x00" as *const u8 as *const libc::c_char,
     b"BITE\x00" as *const u8 as *const libc::c_char,
     b"STING\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"BUTT\x00" as *const u8 as *const libc::c_char,
     b"CRUSH\x00" as *const u8 as *const libc::c_char,
     b"ENGULF\x00" as *const u8 as *const libc::c_char,
     b"CHARGE\x00" as *const u8 as *const libc::c_char,
     b"CRAWL\x00" as *const u8 as *const libc::c_char,
     b"DROOL\x00" as *const u8 as *const libc::c_char,
     b"SPIT\x00" as *const u8 as *const libc::c_char,
     b"EXPLODE\x00" as *const u8 as *const libc::c_char,
     b"GAZE\x00" as *const u8 as *const libc::c_char,
     b"WAIL\x00" as *const u8 as *const libc::c_char,
     b"SPORE\x00" as *const u8 as *const libc::c_char,
     b"XXX4\x00" as *const u8 as *const libc::c_char,
     b"BEG\x00" as *const u8 as *const libc::c_char,
     b"INSULT\x00" as *const u8 as *const libc::c_char,
     b"MOAN\x00" as *const u8 as *const libc::c_char,
     b"SHOW\x00" as *const u8 as *const libc::c_char, 0 as cptr];
/*
 * Monster Blow Effects
 */
static mut r_info_blow_effect: [cptr; 36] =
    [b"*\x00" as *const u8 as *const libc::c_char,
     b"HURT\x00" as *const u8 as *const libc::c_char,
     b"POISON\x00" as *const u8 as *const libc::c_char,
     b"UN_BONUS\x00" as *const u8 as *const libc::c_char,
     b"UN_POWER\x00" as *const u8 as *const libc::c_char,
     b"EAT_GOLD\x00" as *const u8 as *const libc::c_char,
     b"EAT_ITEM\x00" as *const u8 as *const libc::c_char,
     b"EAT_FOOD\x00" as *const u8 as *const libc::c_char,
     b"EAT_LITE\x00" as *const u8 as *const libc::c_char,
     b"ACID\x00" as *const u8 as *const libc::c_char,
     b"ELEC\x00" as *const u8 as *const libc::c_char,
     b"FIRE\x00" as *const u8 as *const libc::c_char,
     b"COLD\x00" as *const u8 as *const libc::c_char,
     b"BLIND\x00" as *const u8 as *const libc::c_char,
     b"CONFUSE\x00" as *const u8 as *const libc::c_char,
     b"TERRIFY\x00" as *const u8 as *const libc::c_char,
     b"PARALYZE\x00" as *const u8 as *const libc::c_char,
     b"LOSE_STR\x00" as *const u8 as *const libc::c_char,
     b"LOSE_INT\x00" as *const u8 as *const libc::c_char,
     b"LOSE_WIS\x00" as *const u8 as *const libc::c_char,
     b"LOSE_DEX\x00" as *const u8 as *const libc::c_char,
     b"LOSE_CON\x00" as *const u8 as *const libc::c_char,
     b"LOSE_CHR\x00" as *const u8 as *const libc::c_char,
     b"LOSE_ALL\x00" as *const u8 as *const libc::c_char,
     b"SHATTER\x00" as *const u8 as *const libc::c_char,
     b"EXP_10\x00" as *const u8 as *const libc::c_char,
     b"EXP_20\x00" as *const u8 as *const libc::c_char,
     b"EXP_40\x00" as *const u8 as *const libc::c_char,
     b"EXP_80\x00" as *const u8 as *const libc::c_char,
     b"DISEASE\x00" as *const u8 as *const libc::c_char,
     b"TIME\x00" as *const u8 as *const libc::c_char,
     b"INSANITY\x00" as *const u8 as *const libc::c_char,
     b"HALLU\x00" as *const u8 as *const libc::c_char,
     b"PARASITE\x00" as *const u8 as *const libc::c_char,
     b"ABOMINATION\x00" as *const u8 as *const libc::c_char, 0 as cptr];
/*
 * Monster race flags
 */
static mut r_info_flags1: [cptr; 32] =
    [b"UNIQUE\x00" as *const u8 as *const libc::c_char,
     b"QUESTOR\x00" as *const u8 as *const libc::c_char,
     b"MALE\x00" as *const u8 as *const libc::c_char,
     b"FEMALE\x00" as *const u8 as *const libc::c_char,
     b"CHAR_CLEAR\x00" as *const u8 as *const libc::c_char,
     b"CHAR_MULTI\x00" as *const u8 as *const libc::c_char,
     b"ATTR_CLEAR\x00" as *const u8 as *const libc::c_char,
     b"ATTR_MULTI\x00" as *const u8 as *const libc::c_char,
     b"FORCE_DEPTH\x00" as *const u8 as *const libc::c_char,
     b"FORCE_MAXHP\x00" as *const u8 as *const libc::c_char,
     b"FORCE_SLEEP\x00" as *const u8 as *const libc::c_char,
     b"FORCE_EXTRA\x00" as *const u8 as *const libc::c_char,
     b"FRIEND\x00" as *const u8 as *const libc::c_char,
     b"FRIENDS\x00" as *const u8 as *const libc::c_char,
     b"ESCORT\x00" as *const u8 as *const libc::c_char,
     b"ESCORTS\x00" as *const u8 as *const libc::c_char,
     b"NEVER_BLOW\x00" as *const u8 as *const libc::c_char,
     b"NEVER_MOVE\x00" as *const u8 as *const libc::c_char,
     b"RAND_25\x00" as *const u8 as *const libc::c_char,
     b"RAND_50\x00" as *const u8 as *const libc::c_char,
     b"ONLY_GOLD\x00" as *const u8 as *const libc::c_char,
     b"ONLY_ITEM\x00" as *const u8 as *const libc::c_char,
     b"DROP_60\x00" as *const u8 as *const libc::c_char,
     b"DROP_90\x00" as *const u8 as *const libc::c_char,
     b"DROP_1D2\x00" as *const u8 as *const libc::c_char,
     b"DROP_2D2\x00" as *const u8 as *const libc::c_char,
     b"DROP_3D2\x00" as *const u8 as *const libc::c_char,
     b"DROP_4D2\x00" as *const u8 as *const libc::c_char,
     b"DROP_GOOD\x00" as *const u8 as *const libc::c_char,
     b"DROP_GREAT\x00" as *const u8 as *const libc::c_char,
     b"DROP_USEFUL\x00" as *const u8 as *const libc::c_char,
     b"DROP_CHOSEN\x00" as *const u8 as *const libc::c_char];
/*
 * Monster race flags
 */
static mut r_info_flags2: [cptr; 32] =
    [b"STUPID\x00" as *const u8 as *const libc::c_char,
     b"SMART\x00" as *const u8 as *const libc::c_char,
     b"CAN_SPEAK\x00" as *const u8 as *const libc::c_char,
     b"REFLECTING\x00" as *const u8 as *const libc::c_char,
     b"INVISIBLE\x00" as *const u8 as *const libc::c_char,
     b"COLD_BLOOD\x00" as *const u8 as *const libc::c_char,
     b"EMPTY_MIND\x00" as *const u8 as *const libc::c_char,
     b"WEIRD_MIND\x00" as *const u8 as *const libc::c_char,
     b"DEATH_ORB\x00" as *const u8 as *const libc::c_char,
     b"REGENERATE\x00" as *const u8 as *const libc::c_char,
     b"SHAPECHANGER\x00" as *const u8 as *const libc::c_char,
     b"ATTR_ANY\x00" as *const u8 as *const libc::c_char,
     b"POWERFUL\x00" as *const u8 as *const libc::c_char,
     b"ELDRITCH_HORROR\x00" as *const u8 as *const libc::c_char,
     b"AURA_FIRE\x00" as *const u8 as *const libc::c_char,
     b"AURA_ELEC\x00" as *const u8 as *const libc::c_char,
     b"OPEN_DOOR\x00" as *const u8 as *const libc::c_char,
     b"BASH_DOOR\x00" as *const u8 as *const libc::c_char,
     b"PASS_WALL\x00" as *const u8 as *const libc::c_char,
     b"KILL_WALL\x00" as *const u8 as *const libc::c_char,
     b"MOVE_BODY\x00" as *const u8 as *const libc::c_char,
     b"KILL_BODY\x00" as *const u8 as *const libc::c_char,
     b"TAKE_ITEM\x00" as *const u8 as *const libc::c_char,
     b"KILL_ITEM\x00" as *const u8 as *const libc::c_char,
     b"BRAIN_1\x00" as *const u8 as *const libc::c_char,
     b"BRAIN_2\x00" as *const u8 as *const libc::c_char,
     b"BRAIN_3\x00" as *const u8 as *const libc::c_char,
     b"BRAIN_4\x00" as *const u8 as *const libc::c_char,
     b"BRAIN_5\x00" as *const u8 as *const libc::c_char,
     b"BRAIN_6\x00" as *const u8 as *const libc::c_char,
     b"BRAIN_7\x00" as *const u8 as *const libc::c_char,
     b"BRAIN_8\x00" as *const u8 as *const libc::c_char];
/*
 * Monster race flags
 */
static mut r_info_flags3: [cptr; 32] =
    [b"ORC\x00" as *const u8 as *const libc::c_char,
     b"TROLL\x00" as *const u8 as *const libc::c_char,
     b"GIANT\x00" as *const u8 as *const libc::c_char,
     b"DRAGON\x00" as *const u8 as *const libc::c_char,
     b"DEMON\x00" as *const u8 as *const libc::c_char,
     b"UNDEAD\x00" as *const u8 as *const libc::c_char,
     b"EVIL\x00" as *const u8 as *const libc::c_char,
     b"ANIMAL\x00" as *const u8 as *const libc::c_char,
     b"THUNDERLORD\x00" as *const u8 as *const libc::c_char,
     b"GOOD\x00" as *const u8 as *const libc::c_char,
     b"AURA_COLD\x00" as *const u8 as *const libc::c_char,
     b"NONLIVING\x00" as *const u8 as *const libc::c_char,
     b"HURT_LITE\x00" as *const u8 as *const libc::c_char,
     b"HURT_ROCK\x00" as *const u8 as *const libc::c_char,
     b"SUSCEP_FIRE\x00" as *const u8 as *const libc::c_char,
     b"SUSCEP_COLD\x00" as *const u8 as *const libc::c_char,
     b"IM_ACID\x00" as *const u8 as *const libc::c_char,
     b"IM_ELEC\x00" as *const u8 as *const libc::c_char,
     b"IM_FIRE\x00" as *const u8 as *const libc::c_char,
     b"IM_COLD\x00" as *const u8 as *const libc::c_char,
     b"IM_POIS\x00" as *const u8 as *const libc::c_char,
     b"RES_TELE\x00" as *const u8 as *const libc::c_char,
     b"RES_NETH\x00" as *const u8 as *const libc::c_char,
     b"RES_WATE\x00" as *const u8 as *const libc::c_char,
     b"RES_PLAS\x00" as *const u8 as *const libc::c_char,
     b"RES_NEXU\x00" as *const u8 as *const libc::c_char,
     b"RES_DISE\x00" as *const u8 as *const libc::c_char,
     b"UNIQUE_4\x00" as *const u8 as *const libc::c_char,
     b"NO_FEAR\x00" as *const u8 as *const libc::c_char,
     b"NO_STUN\x00" as *const u8 as *const libc::c_char,
     b"NO_CONF\x00" as *const u8 as *const libc::c_char,
     b"NO_SLEEP\x00" as *const u8 as *const libc::c_char];
/*
 * Monster race flags
 */
static mut r_info_flags4: [cptr; 32] =
    [b"SHRIEK\x00" as *const u8 as *const libc::c_char,
     b"MULTIPLY\x00" as *const u8 as *const libc::c_char,
     b"S_ANIMAL\x00" as *const u8 as *const libc::c_char,
     b"ROCKET\x00" as *const u8 as *const libc::c_char,
     b"ARROW_1\x00" as *const u8 as *const libc::c_char,
     b"ARROW_2\x00" as *const u8 as *const libc::c_char,
     b"ARROW_3\x00" as *const u8 as *const libc::c_char,
     b"ARROW_4\x00" as *const u8 as *const libc::c_char,
     b"BR_ACID\x00" as *const u8 as *const libc::c_char,
     b"BR_ELEC\x00" as *const u8 as *const libc::c_char,
     b"BR_FIRE\x00" as *const u8 as *const libc::c_char,
     b"BR_COLD\x00" as *const u8 as *const libc::c_char,
     b"BR_POIS\x00" as *const u8 as *const libc::c_char,
     b"BR_NETH\x00" as *const u8 as *const libc::c_char,
     b"BR_LITE\x00" as *const u8 as *const libc::c_char,
     b"BR_DARK\x00" as *const u8 as *const libc::c_char,
     b"BR_CONF\x00" as *const u8 as *const libc::c_char,
     b"BR_SOUN\x00" as *const u8 as *const libc::c_char,
     b"BR_CHAO\x00" as *const u8 as *const libc::c_char,
     b"BR_DISE\x00" as *const u8 as *const libc::c_char,
     b"BR_NEXU\x00" as *const u8 as *const libc::c_char,
     b"BR_TIME\x00" as *const u8 as *const libc::c_char,
     b"BR_INER\x00" as *const u8 as *const libc::c_char,
     b"BR_GRAV\x00" as *const u8 as *const libc::c_char,
     b"BR_SHAR\x00" as *const u8 as *const libc::c_char,
     b"BR_PLAS\x00" as *const u8 as *const libc::c_char,
     b"BR_WALL\x00" as *const u8 as *const libc::c_char,
     b"BR_MANA\x00" as *const u8 as *const libc::c_char,
     b"BA_NUKE\x00" as *const u8 as *const libc::c_char,
     b"BR_NUKE\x00" as *const u8 as *const libc::c_char,
     b"BA_CHAO\x00" as *const u8 as *const libc::c_char,
     b"BR_DISI\x00" as *const u8 as *const libc::c_char];
/*
 * Monster race flags
 */
static mut r_info_flags5: [cptr; 32] =
    [b"BA_ACID\x00" as *const u8 as *const libc::c_char,
     b"BA_ELEC\x00" as *const u8 as *const libc::c_char,
     b"BA_FIRE\x00" as *const u8 as *const libc::c_char,
     b"BA_COLD\x00" as *const u8 as *const libc::c_char,
     b"BA_POIS\x00" as *const u8 as *const libc::c_char,
     b"BA_NETH\x00" as *const u8 as *const libc::c_char,
     b"BA_WATE\x00" as *const u8 as *const libc::c_char,
     b"BA_MANA\x00" as *const u8 as *const libc::c_char,
     b"BA_DARK\x00" as *const u8 as *const libc::c_char,
     b"DRAIN_MANA\x00" as *const u8 as *const libc::c_char,
     b"MIND_BLAST\x00" as *const u8 as *const libc::c_char,
     b"BRAIN_SMASH\x00" as *const u8 as *const libc::c_char,
     b"CAUSE_1\x00" as *const u8 as *const libc::c_char,
     b"CAUSE_2\x00" as *const u8 as *const libc::c_char,
     b"CAUSE_3\x00" as *const u8 as *const libc::c_char,
     b"CAUSE_4\x00" as *const u8 as *const libc::c_char,
     b"BO_ACID\x00" as *const u8 as *const libc::c_char,
     b"BO_ELEC\x00" as *const u8 as *const libc::c_char,
     b"BO_FIRE\x00" as *const u8 as *const libc::c_char,
     b"BO_COLD\x00" as *const u8 as *const libc::c_char,
     b"BO_POIS\x00" as *const u8 as *const libc::c_char,
     b"BO_NETH\x00" as *const u8 as *const libc::c_char,
     b"BO_WATE\x00" as *const u8 as *const libc::c_char,
     b"BO_MANA\x00" as *const u8 as *const libc::c_char,
     b"BO_PLAS\x00" as *const u8 as *const libc::c_char,
     b"BO_ICEE\x00" as *const u8 as *const libc::c_char,
     b"MISSILE\x00" as *const u8 as *const libc::c_char,
     b"SCARE\x00" as *const u8 as *const libc::c_char,
     b"BLIND\x00" as *const u8 as *const libc::c_char,
     b"CONF\x00" as *const u8 as *const libc::c_char,
     b"SLOW\x00" as *const u8 as *const libc::c_char,
     b"HOLD\x00" as *const u8 as *const libc::c_char];
/*
 * Monster race flags
 */
static mut r_info_flags6: [cptr; 32] =
    [b"HASTE\x00" as *const u8 as *const libc::c_char,
     b"HAND_DOOM\x00" as *const u8 as *const libc::c_char,
     b"HEAL\x00" as *const u8 as *const libc::c_char,
     b"S_ANIMALS\x00" as *const u8 as *const libc::c_char,
     b"BLINK\x00" as *const u8 as *const libc::c_char,
     b"TPORT\x00" as *const u8 as *const libc::c_char,
     b"TELE_TO\x00" as *const u8 as *const libc::c_char,
     b"TELE_AWAY\x00" as *const u8 as *const libc::c_char,
     b"TELE_LEVEL\x00" as *const u8 as *const libc::c_char,
     b"DARKNESS\x00" as *const u8 as *const libc::c_char,
     b"TRAPS\x00" as *const u8 as *const libc::c_char,
     b"FORGET\x00" as *const u8 as *const libc::c_char,
     b"ANIM_DEAD\x00" as *const u8 as *const libc::c_char,
     b"S_BUG\x00" as *const u8 as *const libc::c_char,
     b"S_RNG\x00" as *const u8 as *const libc::c_char,
     b"S_THUNDERLORD\x00" as *const u8 as *const libc::c_char,
     b"S_KIN\x00" as *const u8 as *const libc::c_char,
     b"S_HI_DEMON\x00" as *const u8 as *const libc::c_char,
     b"S_MONSTER\x00" as *const u8 as *const libc::c_char,
     b"S_MONSTERS\x00" as *const u8 as *const libc::c_char,
     b"S_ANT\x00" as *const u8 as *const libc::c_char,
     b"S_SPIDER\x00" as *const u8 as *const libc::c_char,
     b"S_HOUND\x00" as *const u8 as *const libc::c_char,
     b"S_HYDRA\x00" as *const u8 as *const libc::c_char,
     b"S_ANGEL\x00" as *const u8 as *const libc::c_char,
     b"S_DEMON\x00" as *const u8 as *const libc::c_char,
     b"S_UNDEAD\x00" as *const u8 as *const libc::c_char,
     b"S_DRAGON\x00" as *const u8 as *const libc::c_char,
     b"S_HI_UNDEAD\x00" as *const u8 as *const libc::c_char,
     b"S_HI_DRAGON\x00" as *const u8 as *const libc::c_char,
     b"S_WRAITH\x00" as *const u8 as *const libc::c_char,
     b"S_UNIQUE\x00" as *const u8 as *const libc::c_char];
/*
 * Monster race flags
 */
static mut r_info_flags7: [cptr; 32] =
    [b"AQUATIC\x00" as *const u8 as *const libc::c_char,
     b"CAN_SWIM\x00" as *const u8 as *const libc::c_char,
     b"CAN_FLY\x00" as *const u8 as *const libc::c_char,
     b"FRIENDLY\x00" as *const u8 as *const libc::c_char,
     b"PET\x00" as *const u8 as *const libc::c_char,
     b"MORTAL\x00" as *const u8 as *const libc::c_char,
     b"SPIDER\x00" as *const u8 as *const libc::c_char,
     b"NAZGUL\x00" as *const u8 as *const libc::c_char,
     b"DG_CURSE\x00" as *const u8 as *const libc::c_char,
     b"POSSESSOR\x00" as *const u8 as *const libc::c_char,
     b"NO_DEATH\x00" as *const u8 as *const libc::c_char,
     b"NO_TARGET\x00" as *const u8 as *const libc::c_char,
     b"AI_ANNOY\x00" as *const u8 as *const libc::c_char,
     b"AI_SPECIAL\x00" as *const u8 as *const libc::c_char,
     b"NEUTRAL\x00" as *const u8 as *const libc::c_char,
     b"DROP_ART\x00" as *const u8 as *const libc::c_char,
     b"DROP_RANDART\x00" as *const u8 as *const libc::c_char,
     b"AI_PLAYER\x00" as *const u8 as *const libc::c_char,
     b"NO_THEFT\x00" as *const u8 as *const libc::c_char,
     b"SPIRIT\x00" as *const u8 as *const libc::c_char,
     b"IM_MELEE\x00" as *const u8 as *const libc::c_char,
     b"XXX7X21\x00" as *const u8 as *const libc::c_char,
     b"XXX7X22\x00" as *const u8 as *const libc::c_char,
     b"XXX7X23\x00" as *const u8 as *const libc::c_char,
     b"XXX7X24\x00" as *const u8 as *const libc::c_char,
     b"XXX7X25\x00" as *const u8 as *const libc::c_char,
     b"XXX7X26\x00" as *const u8 as *const libc::c_char,
     b"XXX7X27\x00" as *const u8 as *const libc::c_char,
     b"XXX7X28\x00" as *const u8 as *const libc::c_char,
     b"XXX7X29\x00" as *const u8 as *const libc::c_char,
     b"XXX7X30\x00" as *const u8 as *const libc::c_char,
     b"XXX7X31\x00" as *const u8 as *const libc::c_char];
/*
 * Monster race flags
 */
static mut r_info_flags8: [cptr; 32] =
    [b"WILD_ONLY\x00" as *const u8 as *const libc::c_char,
     b"WILD_TOWN\x00" as *const u8 as *const libc::c_char,
     b"XXX8X02\x00" as *const u8 as *const libc::c_char,
     b"WILD_SHORE\x00" as *const u8 as *const libc::c_char,
     b"WILD_OCEAN\x00" as *const u8 as *const libc::c_char,
     b"WILD_WASTE\x00" as *const u8 as *const libc::c_char,
     b"WILD_WOOD\x00" as *const u8 as *const libc::c_char,
     b"WILD_VOLCANO\x00" as *const u8 as *const libc::c_char,
     b"XXX8X08\x00" as *const u8 as *const libc::c_char,
     b"WILD_MOUNTAIN\x00" as *const u8 as *const libc::c_char,
     b"WILD_GRASS\x00" as *const u8 as *const libc::c_char,
     b"NO_CUT\x00" as *const u8 as *const libc::c_char,
     b"CTHANGBAND\x00" as *const u8 as *const libc::c_char,
     b"XXX8X13\x00" as *const u8 as *const libc::c_char,
     b"ZANGBAND\x00" as *const u8 as *const libc::c_char,
     b"JOKEANGBAND\x00" as *const u8 as *const libc::c_char,
     b"BASEANGBAND\x00" as *const u8 as *const libc::c_char,
     b"XXX8X17\x00" as *const u8 as *const libc::c_char,
     b"XXX8X18\x00" as *const u8 as *const libc::c_char,
     b"XXX8X19\x00" as *const u8 as *const libc::c_char,
     b"XXX8X20\x00" as *const u8 as *const libc::c_char,
     b"XXX8X21\x00" as *const u8 as *const libc::c_char,
     b"XXX8X22\x00" as *const u8 as *const libc::c_char,
     b"XXX8X23\x00" as *const u8 as *const libc::c_char,
     b"XXX8X24\x00" as *const u8 as *const libc::c_char,
     b"XXX8X25\x00" as *const u8 as *const libc::c_char,
     b"XXX8X26\x00" as *const u8 as *const libc::c_char,
     b"XXX8X27\x00" as *const u8 as *const libc::c_char,
     b"XXX8X28\x00" as *const u8 as *const libc::c_char,
     b"XXX8X29\x00" as *const u8 as *const libc::c_char,
     b"WILD_SWAMP\x00" as *const u8 as *const libc::c_char,
     b"WILD_TOO\x00" as *const u8 as *const libc::c_char];
/*
 * Monster race flags - Drops
 */
static mut r_info_flags9: [cptr; 32] =
    [b"DROP_CORPSE\x00" as *const u8 as *const libc::c_char,
     b"DROP_SKELETON\x00" as *const u8 as *const libc::c_char,
     b"HAS_LITE\x00" as *const u8 as *const libc::c_char,
     b"MIMIC\x00" as *const u8 as *const libc::c_char,
     b"HAS_EGG\x00" as *const u8 as *const libc::c_char,
     b"IMPRESED\x00" as *const u8 as *const libc::c_char,
     b"SUSCEP_ACID\x00" as *const u8 as *const libc::c_char,
     b"SUSCEP_ELEC\x00" as *const u8 as *const libc::c_char,
     b"SUSCEP_POIS\x00" as *const u8 as *const libc::c_char,
     b"KILL_TREES\x00" as *const u8 as *const libc::c_char,
     b"WYRM_PROTECT\x00" as *const u8 as *const libc::c_char,
     b"DOPPLEGANGER\x00" as *const u8 as *const libc::c_char,
     b"ONLY_DEPTH\x00" as *const u8 as *const libc::c_char,
     b"SPECIAL_GENE\x00" as *const u8 as *const libc::c_char,
     b"NEVER_GENE\x00" as *const u8 as *const libc::c_char,
     b"XXX9X15\x00" as *const u8 as *const libc::c_char,
     b"XXX9X16\x00" as *const u8 as *const libc::c_char,
     b"XXX9X17\x00" as *const u8 as *const libc::c_char,
     b"XXX9X18\x00" as *const u8 as *const libc::c_char,
     b"XXX9X19\x00" as *const u8 as *const libc::c_char,
     b"XXX9X20\x00" as *const u8 as *const libc::c_char,
     b"XXX9X21\x00" as *const u8 as *const libc::c_char,
     b"XXX9X22\x00" as *const u8 as *const libc::c_char,
     b"XXX9X23\x00" as *const u8 as *const libc::c_char,
     b"XXX9X24\x00" as *const u8 as *const libc::c_char,
     b"XXX9X25\x00" as *const u8 as *const libc::c_char,
     b"XXX9X26\x00" as *const u8 as *const libc::c_char,
     b"XXX9X27\x00" as *const u8 as *const libc::c_char,
     b"XXX9X28\x00" as *const u8 as *const libc::c_char,
     b"XXX9X29\x00" as *const u8 as *const libc::c_char,
     b"XXX9X30\x00" as *const u8 as *const libc::c_char,
     b"XXX9X31\x00" as *const u8 as *const libc::c_char];
/*
 * Object flags
 */
#[no_mangle]
pub static mut k_info_flags1: [cptr; 32] =
    [b"STR\x00" as *const u8 as *const libc::c_char,
     b"INT\x00" as *const u8 as *const libc::c_char,
     b"WIS\x00" as *const u8 as *const libc::c_char,
     b"DEX\x00" as *const u8 as *const libc::c_char,
     b"CON\x00" as *const u8 as *const libc::c_char,
     b"CHR\x00" as *const u8 as *const libc::c_char,
     b"MANA\x00" as *const u8 as *const libc::c_char,
     b"SPELL\x00" as *const u8 as *const libc::c_char,
     b"STEALTH\x00" as *const u8 as *const libc::c_char,
     b"SEARCH\x00" as *const u8 as *const libc::c_char,
     b"INFRA\x00" as *const u8 as *const libc::c_char,
     b"TUNNEL\x00" as *const u8 as *const libc::c_char,
     b"SPEED\x00" as *const u8 as *const libc::c_char,
     b"BLOWS\x00" as *const u8 as *const libc::c_char,
     b"CHAOTIC\x00" as *const u8 as *const libc::c_char,
     b"VAMPIRIC\x00" as *const u8 as *const libc::c_char,
     b"SLAY_ANIMAL\x00" as *const u8 as *const libc::c_char,
     b"SLAY_EVIL\x00" as *const u8 as *const libc::c_char,
     b"SLAY_UNDEAD\x00" as *const u8 as *const libc::c_char,
     b"SLAY_DEMON\x00" as *const u8 as *const libc::c_char,
     b"SLAY_ORC\x00" as *const u8 as *const libc::c_char,
     b"SLAY_TROLL\x00" as *const u8 as *const libc::c_char,
     b"SLAY_GIANT\x00" as *const u8 as *const libc::c_char,
     b"SLAY_DRAGON\x00" as *const u8 as *const libc::c_char,
     b"KILL_DRAGON\x00" as *const u8 as *const libc::c_char,
     b"VORPAL\x00" as *const u8 as *const libc::c_char,
     b"IMPACT\x00" as *const u8 as *const libc::c_char,
     b"BRAND_POIS\x00" as *const u8 as *const libc::c_char,
     b"BRAND_ACID\x00" as *const u8 as *const libc::c_char,
     b"BRAND_ELEC\x00" as *const u8 as *const libc::c_char,
     b"BRAND_FIRE\x00" as *const u8 as *const libc::c_char,
     b"BRAND_COLD\x00" as *const u8 as *const libc::c_char];
/*
 * Object flags
 */
#[no_mangle]
pub static mut k_info_flags2: [cptr; 32] =
    [b"SUST_STR\x00" as *const u8 as *const libc::c_char,
     b"SUST_INT\x00" as *const u8 as *const libc::c_char,
     b"SUST_WIS\x00" as *const u8 as *const libc::c_char,
     b"SUST_DEX\x00" as *const u8 as *const libc::c_char,
     b"SUST_CON\x00" as *const u8 as *const libc::c_char,
     b"SUST_CHR\x00" as *const u8 as *const libc::c_char,
     b"INVIS\x00" as *const u8 as *const libc::c_char,
     b"LIFE\x00" as *const u8 as *const libc::c_char,
     b"IM_ACID\x00" as *const u8 as *const libc::c_char,
     b"IM_ELEC\x00" as *const u8 as *const libc::c_char,
     b"IM_FIRE\x00" as *const u8 as *const libc::c_char,
     b"IM_COLD\x00" as *const u8 as *const libc::c_char,
     b"SENS_FIRE\x00" as *const u8 as *const libc::c_char,
     b"REFLECT\x00" as *const u8 as *const libc::c_char,
     b"FREE_ACT\x00" as *const u8 as *const libc::c_char,
     b"HOLD_LIFE\x00" as *const u8 as *const libc::c_char,
     b"RES_ACID\x00" as *const u8 as *const libc::c_char,
     b"RES_ELEC\x00" as *const u8 as *const libc::c_char,
     b"RES_FIRE\x00" as *const u8 as *const libc::c_char,
     b"RES_COLD\x00" as *const u8 as *const libc::c_char,
     b"RES_POIS\x00" as *const u8 as *const libc::c_char,
     b"RES_FEAR\x00" as *const u8 as *const libc::c_char,
     b"RES_LITE\x00" as *const u8 as *const libc::c_char,
     b"RES_DARK\x00" as *const u8 as *const libc::c_char,
     b"RES_BLIND\x00" as *const u8 as *const libc::c_char,
     b"RES_CONF\x00" as *const u8 as *const libc::c_char,
     b"RES_SOUND\x00" as *const u8 as *const libc::c_char,
     b"RES_SHARDS\x00" as *const u8 as *const libc::c_char,
     b"RES_NETHER\x00" as *const u8 as *const libc::c_char,
     b"RES_NEXUS\x00" as *const u8 as *const libc::c_char,
     b"RES_CHAOS\x00" as *const u8 as *const libc::c_char,
     b"RES_DISEN\x00" as *const u8 as *const libc::c_char];
/*
 * Trap flags
 */
#[no_mangle]
pub static mut k_info_flags2_trap: [cptr; 32] =
    [b"AUTOMATIC_5\x00" as *const u8 as *const libc::c_char,
     b"AUTOMATIC_99\x00" as *const u8 as *const libc::c_char,
     b"KILL_GHOST\x00" as *const u8 as *const libc::c_char,
     b"TELEPORT_TO\x00" as *const u8 as *const libc::c_char,
     b"ONLY_DRAGON\x00" as *const u8 as *const libc::c_char,
     b"ONLY_DEMON\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"ONLY_ANIMAL\x00" as *const u8 as *const libc::c_char,
     b"ONLY_UNDEAD\x00" as *const u8 as *const libc::c_char,
     b"ONLY_EVIL\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char,
     b"XXX3\x00" as *const u8 as *const libc::c_char];
/*
 * Object flags
 */
#[no_mangle]
pub static mut k_info_flags3: [cptr; 32] =
    [b"SH_FIRE\x00" as *const u8 as *const libc::c_char,
     b"SH_ELEC\x00" as *const u8 as *const libc::c_char,
     b"AUTO_CURSE\x00" as *const u8 as *const libc::c_char,
     b"DECAY\x00" as *const u8 as *const libc::c_char,
     b"NO_TELE\x00" as *const u8 as *const libc::c_char,
     b"NO_MAGIC\x00" as *const u8 as *const libc::c_char,
     b"WRAITH\x00" as *const u8 as *const libc::c_char,
     b"TY_CURSE\x00" as *const u8 as *const libc::c_char,
     b"EASY_KNOW\x00" as *const u8 as *const libc::c_char,
     b"HIDE_TYPE\x00" as *const u8 as *const libc::c_char,
     b"SHOW_MODS\x00" as *const u8 as *const libc::c_char,
     b"INSTA_ART\x00" as *const u8 as *const libc::c_char,
     b"FEATHER\x00" as *const u8 as *const libc::c_char,
     b"LITE1\x00" as *const u8 as *const libc::c_char,
     b"SEE_INVIS\x00" as *const u8 as *const libc::c_char,
     b"NORM_ART\x00" as *const u8 as *const libc::c_char,
     b"SLOW_DIGEST\x00" as *const u8 as *const libc::c_char,
     b"REGEN\x00" as *const u8 as *const libc::c_char,
     b"XTRA_MIGHT\x00" as *const u8 as *const libc::c_char,
     b"XTRA_SHOTS\x00" as *const u8 as *const libc::c_char,
     b"IGNORE_ACID\x00" as *const u8 as *const libc::c_char,
     b"IGNORE_ELEC\x00" as *const u8 as *const libc::c_char,
     b"IGNORE_FIRE\x00" as *const u8 as *const libc::c_char,
     b"IGNORE_COLD\x00" as *const u8 as *const libc::c_char,
     b"ACTIVATE\x00" as *const u8 as *const libc::c_char,
     b"DRAIN_EXP\x00" as *const u8 as *const libc::c_char,
     b"TELEPORT\x00" as *const u8 as *const libc::c_char,
     b"AGGRAVATE\x00" as *const u8 as *const libc::c_char,
     b"BLESSED\x00" as *const u8 as *const libc::c_char,
     b"CURSED\x00" as *const u8 as *const libc::c_char,
     b"HEAVY_CURSE\x00" as *const u8 as *const libc::c_char,
     b"PERMA_CURSE\x00" as *const u8 as *const libc::c_char];
/*
 * Object flags
 */
#[no_mangle]
pub static mut k_info_flags4: [cptr; 32] =
    [b"NEVER_BLOW\x00" as *const u8 as *const libc::c_char,
     b"PRECOGNITION\x00" as *const u8 as *const libc::c_char,
     b"BLACK_BREATH\x00" as *const u8 as *const libc::c_char,
     b"RECHARGE\x00" as *const u8 as *const libc::c_char,
     b"FLY\x00" as *const u8 as *const libc::c_char,
     b"DG_CURSE\x00" as *const u8 as *const libc::c_char,
     b"COULD2H\x00" as *const u8 as *const libc::c_char,
     b"MUST2H\x00" as *const u8 as *const libc::c_char,
     b"LEVELS\x00" as *const u8 as *const libc::c_char,
     b"CLONE\x00" as *const u8 as *const libc::c_char,
     b"SPECIAL_GENE\x00" as *const u8 as *const libc::c_char,
     b"CLIMB\x00" as *const u8 as *const libc::c_char,
     b"FAST_CAST\x00" as *const u8 as *const libc::c_char,
     b"CAPACITY\x00" as *const u8 as *const libc::c_char,
     b"CHARGING\x00" as *const u8 as *const libc::c_char,
     b"CHEAPNESS\x00" as *const u8 as *const libc::c_char,
     b"FOUNTAIN\x00" as *const u8 as *const libc::c_char,
     b"ANTIMAGIC_50\x00" as *const u8 as *const libc::c_char,
     b"ANTIMAGIC_30\x00" as *const u8 as *const libc::c_char,
     b"ANTIMAGIC_20\x00" as *const u8 as *const libc::c_char,
     b"ANTIMAGIC_10\x00" as *const u8 as *const libc::c_char,
     b"EASY_USE\x00" as *const u8 as *const libc::c_char,
     b"IM_NETHER\x00" as *const u8 as *const libc::c_char,
     b"RECHARGED\x00" as *const u8 as *const libc::c_char,
     b"ULTIMATE\x00" as *const u8 as *const libc::c_char,
     b"AUTO_ID\x00" as *const u8 as *const libc::c_char,
     b"LITE2\x00" as *const u8 as *const libc::c_char,
     b"LITE3\x00" as *const u8 as *const libc::c_char,
     b"FUEL_LITE\x00" as *const u8 as *const libc::c_char,
     b"ART_EXP\x00" as *const u8 as *const libc::c_char,
     b"CURSE_NO_DROP\x00" as *const u8 as *const libc::c_char,
     b"NO_RECHARGE\x00" as *const u8 as *const libc::c_char];
/*
 * Object flags
 */
#[no_mangle]
pub static mut k_info_flags5: [cptr; 32] =
    [b"TEMPORARY\x00" as *const u8 as *const libc::c_char,
     b"DRAIN_MANA\x00" as *const u8 as *const libc::c_char,
     b"DRAIN_HP\x00" as *const u8 as *const libc::c_char,
     b"KILL_DEMON\x00" as *const u8 as *const libc::c_char,
     b"KILL_UNDEAD\x00" as *const u8 as *const libc::c_char,
     b"CRIT\x00" as *const u8 as *const libc::c_char,
     b"ATTR_MULTI\x00" as *const u8 as *const libc::c_char,
     b"WOUNDING\x00" as *const u8 as *const libc::c_char,
     b"FULL_NAME\x00" as *const u8 as *const libc::c_char,
     b"LUCK\x00" as *const u8 as *const libc::c_char,
     b"IMMOVABLE\x00" as *const u8 as *const libc::c_char,
     b"SPELL_CONTAIN\x00" as *const u8 as *const libc::c_char,
     b"RES_MORGUL\x00" as *const u8 as *const libc::c_char,
     b"ACTIVATE_NO_WIELD\x00" as *const u8 as *const libc::c_char,
     b"MAGIC_BREATH\x00" as *const u8 as *const libc::c_char,
     b"WATER_BREATH\x00" as *const u8 as *const libc::c_char,
     b"WIELD_CAST\x00" as *const u8 as *const libc::c_char,
     b"XXX8X17\x00" as *const u8 as *const libc::c_char,
     b"XXX8X18\x00" as *const u8 as *const libc::c_char,
     b"XXX8X19\x00" as *const u8 as *const libc::c_char,
     b"XXX8X20\x00" as *const u8 as *const libc::c_char,
     b"XXX8X21\x00" as *const u8 as *const libc::c_char,
     b"XXX8X22\x00" as *const u8 as *const libc::c_char,
     b"XXX8X23\x00" as *const u8 as *const libc::c_char,
     b"XXX8X24\x00" as *const u8 as *const libc::c_char,
     b"XXX8X25\x00" as *const u8 as *const libc::c_char,
     b"XXX8X26\x00" as *const u8 as *const libc::c_char,
     b"XXX8X27\x00" as *const u8 as *const libc::c_char,
     b"XXX8X28\x00" as *const u8 as *const libc::c_char,
     b"XXX8X29\x00" as *const u8 as *const libc::c_char,
     b"XXX8X02\x00" as *const u8 as *const libc::c_char,
     b"XXX8X22\x00" as *const u8 as *const libc::c_char];
/*
 * ESP flags
 */
#[no_mangle]
pub static mut esp_flags: [cptr; 32] =
    [b"ESP_ORC\x00" as *const u8 as *const libc::c_char,
     b"ESP_TROLL\x00" as *const u8 as *const libc::c_char,
     b"ESP_DRAGON\x00" as *const u8 as *const libc::c_char,
     b"ESP_GIANT\x00" as *const u8 as *const libc::c_char,
     b"ESP_DEMON\x00" as *const u8 as *const libc::c_char,
     b"ESP_UNDEAD\x00" as *const u8 as *const libc::c_char,
     b"ESP_EVIL\x00" as *const u8 as *const libc::c_char,
     b"ESP_ANIMAL\x00" as *const u8 as *const libc::c_char,
     b"ESP_THUNDERLORD\x00" as *const u8 as *const libc::c_char,
     b"ESP_GOOD\x00" as *const u8 as *const libc::c_char,
     b"ESP_NONLIVING\x00" as *const u8 as *const libc::c_char,
     b"ESP_UNIQUE\x00" as *const u8 as *const libc::c_char,
     b"ESP_SPIDER\x00" as *const u8 as *const libc::c_char,
     b"XXX8X02\x00" as *const u8 as *const libc::c_char,
     b"XXX8X02\x00" as *const u8 as *const libc::c_char,
     b"XXX8X02\x00" as *const u8 as *const libc::c_char,
     b"XXX8X02\x00" as *const u8 as *const libc::c_char,
     b"XXX8X17\x00" as *const u8 as *const libc::c_char,
     b"XXX8X18\x00" as *const u8 as *const libc::c_char,
     b"XXX8X19\x00" as *const u8 as *const libc::c_char,
     b"XXX8X20\x00" as *const u8 as *const libc::c_char,
     b"XXX8X21\x00" as *const u8 as *const libc::c_char,
     b"XXX8X22\x00" as *const u8 as *const libc::c_char,
     b"XXX8X23\x00" as *const u8 as *const libc::c_char,
     b"XXX8X24\x00" as *const u8 as *const libc::c_char,
     b"XXX8X25\x00" as *const u8 as *const libc::c_char,
     b"XXX8X26\x00" as *const u8 as *const libc::c_char,
     b"XXX8X27\x00" as *const u8 as *const libc::c_char,
     b"XXX8X28\x00" as *const u8 as *const libc::c_char,
     b"XXX8X29\x00" as *const u8 as *const libc::c_char,
     b"XXX8X02\x00" as *const u8 as *const libc::c_char,
     b"ESP_ALL\x00" as *const u8 as *const libc::c_char];
/* Specially handled properties for ego-items */
static mut ego_flags: [cptr; 32] =
    [b"SUSTAIN\x00" as *const u8 as *const libc::c_char,
     b"OLD_RESIST\x00" as *const u8 as *const libc::c_char,
     b"ABILITY\x00" as *const u8 as *const libc::c_char,
     b"R_ELEM\x00" as *const u8 as *const libc::c_char,
     b"R_LOW\x00" as *const u8 as *const libc::c_char,
     b"R_HIGH\x00" as *const u8 as *const libc::c_char,
     b"R_ANY\x00" as *const u8 as *const libc::c_char,
     b"R_DRAGON\x00" as *const u8 as *const libc::c_char,
     b"SLAY_WEAP\x00" as *const u8 as *const libc::c_char,
     b"DAM_DIE\x00" as *const u8 as *const libc::c_char,
     b"DAM_SIZE\x00" as *const u8 as *const libc::c_char,
     b"PVAL_M1\x00" as *const u8 as *const libc::c_char,
     b"PVAL_M2\x00" as *const u8 as *const libc::c_char,
     b"PVAL_M3\x00" as *const u8 as *const libc::c_char,
     b"PVAL_M5\x00" as *const u8 as *const libc::c_char,
     b"AC_M1\x00" as *const u8 as *const libc::c_char,
     b"AC_M2\x00" as *const u8 as *const libc::c_char,
     b"AC_M3\x00" as *const u8 as *const libc::c_char,
     b"AC_M5\x00" as *const u8 as *const libc::c_char,
     b"TH_M1\x00" as *const u8 as *const libc::c_char,
     b"TH_M2\x00" as *const u8 as *const libc::c_char,
     b"TH_M3\x00" as *const u8 as *const libc::c_char,
     b"TH_M5\x00" as *const u8 as *const libc::c_char,
     b"TD_M1\x00" as *const u8 as *const libc::c_char,
     b"TD_M2\x00" as *const u8 as *const libc::c_char,
     b"TD_M3\x00" as *const u8 as *const libc::c_char,
     b"TD_M5\x00" as *const u8 as *const libc::c_char,
     b"R_P_ABILITY\x00" as *const u8 as *const libc::c_char,
     b"R_STAT\x00" as *const u8 as *const libc::c_char,
     b"R_STAT_SUST\x00" as *const u8 as *const libc::c_char,
     b"R_IMMUNITY\x00" as *const u8 as *const libc::c_char,
     b"LIMIT_BLOWS\x00" as *const u8 as *const libc::c_char];
/*
 * Feature flags
 */
static mut f_info_flags1: [cptr; 32] =
    [b"NO_WALK\x00" as *const u8 as *const libc::c_char,
     b"NO_VISION\x00" as *const u8 as *const libc::c_char,
     b"CAN_LEVITATE\x00" as *const u8 as *const libc::c_char,
     b"CAN_PASS\x00" as *const u8 as *const libc::c_char,
     b"FLOOR\x00" as *const u8 as *const libc::c_char,
     b"WALL\x00" as *const u8 as *const libc::c_char,
     b"PERMANENT\x00" as *const u8 as *const libc::c_char,
     b"CAN_FLY\x00" as *const u8 as *const libc::c_char,
     b"REMEMBER\x00" as *const u8 as *const libc::c_char,
     b"NOTICE\x00" as *const u8 as *const libc::c_char,
     b"DONT_NOTICE_RUNNING\x00" as *const u8 as *const libc::c_char,
     b"CAN_RUN\x00" as *const u8 as *const libc::c_char,
     b"DOOR\x00" as *const u8 as *const libc::c_char,
     b"SUPPORT_LIGHT\x00" as *const u8 as *const libc::c_char,
     b"CAN_CLIMB\x00" as *const u8 as *const libc::c_char,
     b"TUNNELABLE\x00" as *const u8 as *const libc::c_char,
     b"WEB\x00" as *const u8 as *const libc::c_char,
     b"ATTR_MULTI\x00" as *const u8 as *const libc::c_char,
     b"SUPPORT_GROWTH\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char];
/*
 * Dungeon flags
 */
static mut d_info_flags1: [cptr; 32] =
    [b"PRINCIPAL\x00" as *const u8 as *const libc::c_char,
     b"MAZE\x00" as *const u8 as *const libc::c_char,
     b"SMALLEST\x00" as *const u8 as *const libc::c_char,
     b"SMALL\x00" as *const u8 as *const libc::c_char,
     b"BIG\x00" as *const u8 as *const libc::c_char,
     b"NO_DOORS\x00" as *const u8 as *const libc::c_char,
     b"WATER_RIVER\x00" as *const u8 as *const libc::c_char,
     b"LAVA_RIVER\x00" as *const u8 as *const libc::c_char,
     b"WATER_RIVERS\x00" as *const u8 as *const libc::c_char,
     b"LAVA_RIVERS\x00" as *const u8 as *const libc::c_char,
     b"CAVE\x00" as *const u8 as *const libc::c_char,
     b"CAVERN\x00" as *const u8 as *const libc::c_char,
     b"NO_UP\x00" as *const u8 as *const libc::c_char,
     b"HOT\x00" as *const u8 as *const libc::c_char,
     b"COLD\x00" as *const u8 as *const libc::c_char,
     b"FORCE_DOWN\x00" as *const u8 as *const libc::c_char,
     b"FORGET\x00" as *const u8 as *const libc::c_char,
     b"NO_DESTROY\x00" as *const u8 as *const libc::c_char,
     b"SAND_VEIN\x00" as *const u8 as *const libc::c_char,
     b"CIRCULAR_ROOMS\x00" as *const u8 as *const libc::c_char,
     b"EMPTY\x00" as *const u8 as *const libc::c_char,
     b"DAMAGE_FEAT\x00" as *const u8 as *const libc::c_char,
     b"FLAT\x00" as *const u8 as *const libc::c_char,
     b"TOWER\x00" as *const u8 as *const libc::c_char,
     b"RANDOM_TOWNS\x00" as *const u8 as *const libc::c_char,
     b"DOUBLE\x00" as *const u8 as *const libc::c_char,
     b"LIFE_LEVEL\x00" as *const u8 as *const libc::c_char,
     b"EVOLVE\x00" as *const u8 as *const libc::c_char,
     b"ADJUST_LEVEL_1\x00" as *const u8 as *const libc::c_char,
     b"ADJUST_LEVEL_2\x00" as *const u8 as *const libc::c_char,
     b"NO_RECALL\x00" as *const u8 as *const libc::c_char,
     b"NO_STREAMERS\x00" as *const u8 as *const libc::c_char];
static mut d_info_flags2: [cptr; 33] =
    [b"ADJUST_LEVEL_1_2\x00" as *const u8 as *const libc::c_char,
     b"NO_SHAFT\x00" as *const u8 as *const libc::c_char,
     b"ADJUST_LEVEL_PLAYER\x00" as *const u8 as *const libc::c_char,
     b"NO_TELEPORT\x00" as *const u8 as *const libc::c_char,
     b"ASK_LEAVE\x00" as *const u8 as *const libc::c_char,
     b"NO_STAIR\x00" as *const u8 as *const libc::c_char,
     b"SPECIAL\x00" as *const u8 as *const libc::c_char,
     b"NO_NEW_MONSTER\x00" as *const u8 as *const libc::c_char,
     b"DESC\x00" as *const u8 as *const libc::c_char,
     b"NO_GENO\x00" as *const u8 as *const libc::c_char,
     b"NO_BREATH\x00" as *const u8 as *const libc::c_char,
     b"WATER_BREATH\x00" as *const u8 as *const libc::c_char,
     b"ELVEN\x00" as *const u8 as *const libc::c_char,
     b"DWARVEN\x00" as *const u8 as *const libc::c_char,
     b"NO_EASY_MOVE\x00" as *const u8 as *const libc::c_char,
     b"NO_RECALL_OUT\x00" as *const u8 as *const libc::c_char,
     b"DESC_ALWAYS\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char];
/*
 * Trap flags
 */
static mut t_info_flags: [cptr; 32] =
    [b"CHEST\x00" as *const u8 as *const libc::c_char,
     b"DOOR\x00" as *const u8 as *const libc::c_char,
     b"FLOOR\x00" as *const u8 as *const libc::c_char,
     b"XXX4\x00" as *const u8 as *const libc::c_char,
     b"XXX5\x00" as *const u8 as *const libc::c_char,
     b"XXX6\x00" as *const u8 as *const libc::c_char,
     b"XXX7\x00" as *const u8 as *const libc::c_char,
     b"XXX8\x00" as *const u8 as *const libc::c_char,
     b"XXX9\x00" as *const u8 as *const libc::c_char,
     b"XXX10\x00" as *const u8 as *const libc::c_char,
     b"XXX11\x00" as *const u8 as *const libc::c_char,
     b"XXX12\x00" as *const u8 as *const libc::c_char,
     b"XXX13\x00" as *const u8 as *const libc::c_char,
     b"XXX14\x00" as *const u8 as *const libc::c_char,
     b"XXX15\x00" as *const u8 as *const libc::c_char,
     b"XXX16\x00" as *const u8 as *const libc::c_char,
     b"LEVEL1\x00" as *const u8 as *const libc::c_char,
     b"LEVEL2\x00" as *const u8 as *const libc::c_char,
     b"LEVEL3\x00" as *const u8 as *const libc::c_char,
     b"LEVEL4\x00" as *const u8 as *const libc::c_char,
     b"XXX21\x00" as *const u8 as *const libc::c_char,
     b"XXX22\x00" as *const u8 as *const libc::c_char,
     b"XXX23\x00" as *const u8 as *const libc::c_char,
     b"XXX24\x00" as *const u8 as *const libc::c_char,
     b"XXX25\x00" as *const u8 as *const libc::c_char,
     b"XXX26\x00" as *const u8 as *const libc::c_char,
     b"XXX27\x00" as *const u8 as *const libc::c_char,
     b"XXX28\x00" as *const u8 as *const libc::c_char,
     b"XXX29\x00" as *const u8 as *const libc::c_char,
     b"XXX30\x00" as *const u8 as *const libc::c_char,
     b"XXX31\x00" as *const u8 as *const libc::c_char,
     b"XXX32\x00" as *const u8 as *const libc::c_char];
/*
 * Wilderness feature flags
 */
static mut wf_info_flags1: [cptr; 33] =
    [b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char];
/*
 * Stores flags
 */
static mut st_info_flags1: [cptr; 33] =
    [b"DEPEND_LEVEL\x00" as *const u8 as *const libc::c_char,
     b"SHALLOW_LEVEL\x00" as *const u8 as *const libc::c_char,
     b"MEDIUM_LEVEL\x00" as *const u8 as *const libc::c_char,
     b"DEEP_LEVEL\x00" as *const u8 as *const libc::c_char,
     b"RARE\x00" as *const u8 as *const libc::c_char,
     b"VERY_RARE\x00" as *const u8 as *const libc::c_char,
     b"COMMON\x00" as *const u8 as *const libc::c_char,
     b"ALL_ITEM\x00" as *const u8 as *const libc::c_char,
     b"RANDOM\x00" as *const u8 as *const libc::c_char,
     b"FORCE_LEVEL\x00" as *const u8 as *const libc::c_char,
     b"MUSEUM\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char];
/*
 * Race flags
 */
#[no_mangle]
pub static mut rp_info_flags1: [cptr; 32] =
    [b"EXPERIMENTAL\x00" as *const u8 as *const libc::c_char,
     b"XXX\x00" as *const u8 as *const libc::c_char,
     b"RESIST_BLACK_BREATH\x00" as *const u8 as *const libc::c_char,
     b"NO_STUN\x00" as *const u8 as *const libc::c_char,
     b"XTRA_MIGHT_BOW\x00" as *const u8 as *const libc::c_char,
     b"XTRA_MIGHT_XBOW\x00" as *const u8 as *const libc::c_char,
     b"XTRA_MIGHT_SLING\x00" as *const u8 as *const libc::c_char,
     b"AC_LEVEL\x00" as *const u8 as *const libc::c_char,
     b"HURT_LITE\x00" as *const u8 as *const libc::c_char,
     b"VAMPIRE\x00" as *const u8 as *const libc::c_char,
     b"UNDEAD\x00" as *const u8 as *const libc::c_char,
     b"NO_CUT\x00" as *const u8 as *const libc::c_char,
     b"CORRUPT\x00" as *const u8 as *const libc::c_char,
     b"NO_FOOD\x00" as *const u8 as *const libc::c_char,
     b"NO_GOD\x00" as *const u8 as *const libc::c_char,
     b"XXX\x00" as *const u8 as *const libc::c_char,
     b"ELF\x00" as *const u8 as *const libc::c_char,
     b"SEMI_WRAITH\x00" as *const u8 as *const libc::c_char,
     b"NO_SUBRACE_CHANGE\x00" as *const u8 as *const libc::c_char,
     b"XXX\x00" as *const u8 as *const libc::c_char,
     b"XXX\x00" as *const u8 as *const libc::c_char,
     b"MOLD_FRIEND\x00" as *const u8 as *const libc::c_char,
     b"GOD_FRIEND\x00" as *const u8 as *const libc::c_char,
     b"XXX\x00" as *const u8 as *const libc::c_char,
     b"INNATE_SPELLS\x00" as *const u8 as *const libc::c_char,
     b"XXX\x00" as *const u8 as *const libc::c_char,
     b"XXX\x00" as *const u8 as *const libc::c_char,
     b"EASE_STEAL\x00" as *const u8 as *const libc::c_char,
     b"XXX\x00" as *const u8 as *const libc::c_char,
     b"XXX\x00" as *const u8 as *const libc::c_char,
     b"XXX\x00" as *const u8 as *const libc::c_char,
     b"XXX\x00" as *const u8 as *const libc::c_char];
/*
 * Race flags
 */
#[no_mangle]
pub static mut rp_info_flags2: [cptr; 33] =
    [b"XXX\x00" as *const u8 as *const libc::c_char,
     b"ASTRAL\x00" as *const u8 as *const libc::c_char,
     b"XXX\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char];
/* Skill flags */
static mut s_info_flags1: [cptr; 33] =
    [b"HIDDEN\x00" as *const u8 as *const libc::c_char,
     b"AUTO_HIDE\x00" as *const u8 as *const libc::c_char,
     b"RANDOM_GAIN\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char,
     b"XXX1\x00" as *const u8 as *const libc::c_char];
static mut d_info_dtypes: [C2RustUnnamed_5; 35] =
    [{
         let mut init =
             C2RustUnnamed_5{name:
                                 b"ELEC\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"POISON\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"ACID\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"COLD\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"FIRE\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"MISSILE\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 10 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"ARROW\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 11 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"PLASMA\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 12 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"WATER\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 14 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"LITE\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 15 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"DARK\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 16 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"LITE_WEAK\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 17 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"LITE_DARK\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 18 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"SHARDS\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 20 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"SOUND\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 21 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"CONFUSION\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 22 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"FORCE\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 23 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"INERTIA\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 24 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"MANA\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 26 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"METEOR\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 27 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"ICE\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 28 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"CHAOS\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 30 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"NETHER\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 31 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"DISENCHANT\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 32 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"NEXUS\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 33 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"TIME\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 34 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"GRAVITY\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 35 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"ROCKET\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 72 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"NUKE\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 73 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"HOLY_FIRE\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 79 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"HELL_FIRE\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 80 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"DISINTEGRATE\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 81 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"DESTRUCTION\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 94 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name:
                                 b"RAISE\x00" as *const u8 as
                                     *const libc::c_char,
                             feat: 92 as libc::c_int,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{name: 0 as cptr, feat: 0 as libc::c_int,};
         init
     }];
/* Essence names for al_info.txt */
static mut essence_names: [*const libc::c_char; 20] =
    [b"No name here\x00" as *const u8 as *const libc::c_char,
     b"POISON\x00" as *const u8 as *const libc::c_char,
     b"EXPLOSION\x00" as *const u8 as *const libc::c_char,
     b"TELEPORT\x00" as *const u8 as *const libc::c_char,
     b"COLD\x00" as *const u8 as *const libc::c_char,
     b"FIRE\x00" as *const u8 as *const libc::c_char,
     b"ACID\x00" as *const u8 as *const libc::c_char,
     b"LIFE\x00" as *const u8 as *const libc::c_char,
     b"CONFUSION\x00" as *const u8 as *const libc::c_char,
     b"LITE\x00" as *const u8 as *const libc::c_char,
     b"CHAOS\x00" as *const u8 as *const libc::c_char,
     b"TIME\x00" as *const u8 as *const libc::c_char,
     b"MAGIC\x00" as *const u8 as *const libc::c_char,
     b"EXTRALIFE\x00" as *const u8 as *const libc::c_char,
     b"DARKNESS\x00" as *const u8 as *const libc::c_char,
     b"KNOWLEDGE\x00" as *const u8 as *const libc::c_char,
     b"FORCE\x00" as *const u8 as *const libc::c_char,
     b"LIGHTNING\x00" as *const u8 as *const libc::c_char,
     b"MANA\x00" as *const u8 as *const libc::c_char,
     b"\x00" as *const u8 as *const libc::c_char];
static mut activation_names: [*const libc::c_char; 202] =
    [b"NO_ACTIVATION\x00" as *const u8 as *const libc::c_char,
     b"SUNLIGHT\x00" as *const u8 as *const libc::c_char,
     b"BO_MISS_1\x00" as *const u8 as *const libc::c_char,
     b"BA_POIS_1\x00" as *const u8 as *const libc::c_char,
     b"BO_ELEC_1\x00" as *const u8 as *const libc::c_char,
     b"BO_ACID_1\x00" as *const u8 as *const libc::c_char,
     b"BO_COLD_1\x00" as *const u8 as *const libc::c_char,
     b"BO_FIRE_1\x00" as *const u8 as *const libc::c_char,
     b"BA_COLD_1\x00" as *const u8 as *const libc::c_char,
     b"BA_FIRE_1\x00" as *const u8 as *const libc::c_char,
     b"DRAIN_1\x00" as *const u8 as *const libc::c_char,
     b"BA_COLD_2\x00" as *const u8 as *const libc::c_char,
     b"BA_ELEC_2\x00" as *const u8 as *const libc::c_char,
     b"DRAIN_2\x00" as *const u8 as *const libc::c_char,
     b"VAMPIRE_1\x00" as *const u8 as *const libc::c_char,
     b"BO_MISS_2\x00" as *const u8 as *const libc::c_char,
     b"BA_FIRE_2\x00" as *const u8 as *const libc::c_char,
     b"BA_COLD_3\x00" as *const u8 as *const libc::c_char,
     b"BA_ELEC_3\x00" as *const u8 as *const libc::c_char,
     b"WHIRLWIND\x00" as *const u8 as *const libc::c_char,
     b"VAMPIRE_2\x00" as *const u8 as *const libc::c_char,
     b"CALL_CHAOS\x00" as *const u8 as *const libc::c_char,
     b"ROCKET\x00" as *const u8 as *const libc::c_char,
     b"DISP_EVIL\x00" as *const u8 as *const libc::c_char,
     b"BA_MISS_3\x00" as *const u8 as *const libc::c_char,
     b"DISP_GOOD\x00" as *const u8 as *const libc::c_char,
     b"GILGALAD\x00" as *const u8 as *const libc::c_char,
     b"CELEBRIMBOR\x00" as *const u8 as *const libc::c_char,
     b"SKULLCLEAVER\x00" as *const u8 as *const libc::c_char,
     b"HARADRIM\x00" as *const u8 as *const libc::c_char,
     b"FUNDIN\x00" as *const u8 as *const libc::c_char,
     b"EOL\x00" as *const u8 as *const libc::c_char,
     b"UMBAR\x00" as *const u8 as *const libc::c_char,
     b"NUMENOR\x00" as *const u8 as *const libc::c_char,
     b"KNOWLEDGE\x00" as *const u8 as *const libc::c_char,
     b"UNDEATH\x00" as *const u8 as *const libc::c_char,
     b"THRAIN\x00" as *const u8 as *const libc::c_char,
     b"BARAHIR\x00" as *const u8 as *const libc::c_char,
     b"TULKAS\x00" as *const u8 as *const libc::c_char,
     b"NARYA\x00" as *const u8 as *const libc::c_char,
     b"NENYA\x00" as *const u8 as *const libc::c_char,
     b"VILYA\x00" as *const u8 as *const libc::c_char,
     b"POWER\x00" as *const u8 as *const libc::c_char,
     b"STONE_LORE\x00" as *const u8 as *const libc::c_char,
     b"RAZORBACK\x00" as *const u8 as *const libc::c_char,
     b"BLADETURNER\x00" as *const u8 as *const libc::c_char,
     b"MEDIATOR\x00" as *const u8 as *const libc::c_char,
     b"BELEGENNON\x00" as *const u8 as *const libc::c_char,
     b"GORLIM\x00" as *const u8 as *const libc::c_char,
     b"COLLUIN\x00" as *const u8 as *const libc::c_char,
     b"BELANGIL\x00" as *const u8 as *const libc::c_char,
     b"CONFUSE\x00" as *const u8 as *const libc::c_char,
     b"SLEEP\x00" as *const u8 as *const libc::c_char,
     b"QUAKE\x00" as *const u8 as *const libc::c_char,
     b"TERROR\x00" as *const u8 as *const libc::c_char,
     b"TELE_AWAY\x00" as *const u8 as *const libc::c_char,
     b"BANISH_EVIL\x00" as *const u8 as *const libc::c_char,
     b"GENOCIDE\x00" as *const u8 as *const libc::c_char,
     b"MASS_GENO\x00" as *const u8 as *const libc::c_char,
     b"ANGUIREL\x00" as *const u8 as *const libc::c_char,
     b"ERU\x00" as *const u8 as *const libc::c_char,
     b"DAWN\x00" as *const u8 as *const libc::c_char,
     b"FIRESTAR\x00" as *const u8 as *const libc::c_char,
     b"TURMIL\x00" as *const u8 as *const libc::c_char,
     b"CUBRAGOL\x00" as *const u8 as *const libc::c_char,
     b"CHARM_ANIMAL\x00" as *const u8 as *const libc::c_char,
     b"CHARM_UNDEAD\x00" as *const u8 as *const libc::c_char,
     b"CHARM_OTHER\x00" as *const u8 as *const libc::c_char,
     b"CHARM_ANIMALS\x00" as *const u8 as *const libc::c_char,
     b"CHARM_OTHERS\x00" as *const u8 as *const libc::c_char,
     b"SUMMON_ANIMAL\x00" as *const u8 as *const libc::c_char,
     b"SUMMON_PHANTOM\x00" as *const u8 as *const libc::c_char,
     b"SUMMON_ELEMENTAL\x00" as *const u8 as *const libc::c_char,
     b"SUMMON_DEMON\x00" as *const u8 as *const libc::c_char,
     b"SUMMON_UNDEAD\x00" as *const u8 as *const libc::c_char,
     b"ELESSAR\x00" as *const u8 as *const libc::c_char,
     b"GANDALF\x00" as *const u8 as *const libc::c_char,
     b"MARDA\x00" as *const u8 as *const libc::c_char,
     b"PALANTIR\x00" as *const u8 as *const libc::c_char,
     b"XXX79\x00" as *const u8 as *const libc::c_char,
     b"XXX80\x00" as *const u8 as *const libc::c_char,
     b"CURE_LW\x00" as *const u8 as *const libc::c_char,
     b"CURE_MW\x00" as *const u8 as *const libc::c_char,
     b"CURE_POISON\x00" as *const u8 as *const libc::c_char,
     b"REST_LIFE\x00" as *const u8 as *const libc::c_char,
     b"REST_ALL\x00" as *const u8 as *const libc::c_char,
     b"CURE_700\x00" as *const u8 as *const libc::c_char,
     b"CURE_1000\x00" as *const u8 as *const libc::c_char,
     b"XXX88\x00" as *const u8 as *const libc::c_char,
     b"EREBOR\x00" as *const u8 as *const libc::c_char,
     b"DRUEDAIN\x00" as *const u8 as *const libc::c_char,
     b"ESP\x00" as *const u8 as *const libc::c_char,
     b"BERSERK\x00" as *const u8 as *const libc::c_char,
     b"PROT_EVIL\x00" as *const u8 as *const libc::c_char,
     b"RESIST_ALL\x00" as *const u8 as *const libc::c_char,
     b"SPEED\x00" as *const u8 as *const libc::c_char,
     b"XTRA_SPEED\x00" as *const u8 as *const libc::c_char,
     b"WRAITH\x00" as *const u8 as *const libc::c_char,
     b"INVULN\x00" as *const u8 as *const libc::c_char,
     b"ROHAN\x00" as *const u8 as *const libc::c_char,
     b"HELM\x00" as *const u8 as *const libc::c_char,
     b"BOROMIR\x00" as *const u8 as *const libc::c_char,
     b"HURIN\x00" as *const u8 as *const libc::c_char,
     b"AXE_GOTHMOG\x00" as *const u8 as *const libc::c_char,
     b"MELKOR\x00" as *const u8 as *const libc::c_char,
     b"GROND\x00" as *const u8 as *const libc::c_char,
     b"NATUREBANE\x00" as *const u8 as *const libc::c_char,
     b"NIGHT\x00" as *const u8 as *const libc::c_char,
     b"ORCHAST\x00" as *const u8 as *const libc::c_char,
     b"XXX109\x00" as *const u8 as *const libc::c_char,
     b"XXX110\x00" as *const u8 as *const libc::c_char,
     b"LIGHT\x00" as *const u8 as *const libc::c_char,
     b"MAP_LIGHT\x00" as *const u8 as *const libc::c_char,
     b"DETECT_ALL\x00" as *const u8 as *const libc::c_char,
     b"DETECT_XTRA\x00" as *const u8 as *const libc::c_char,
     b"ID_FULL\x00" as *const u8 as *const libc::c_char,
     b"ID_PLAIN\x00" as *const u8 as *const libc::c_char,
     b"RUNE_EXPLO\x00" as *const u8 as *const libc::c_char,
     b"RUNE_PROT\x00" as *const u8 as *const libc::c_char,
     b"SATIATE\x00" as *const u8 as *const libc::c_char,
     b"DEST_DOOR\x00" as *const u8 as *const libc::c_char,
     b"STONE_MUD\x00" as *const u8 as *const libc::c_char,
     b"RECHARGE\x00" as *const u8 as *const libc::c_char,
     b"ALCHEMY\x00" as *const u8 as *const libc::c_char,
     b"DIM_DOOR\x00" as *const u8 as *const libc::c_char,
     b"TELEPORT\x00" as *const u8 as *const libc::c_char,
     b"RECALL\x00" as *const u8 as *const libc::c_char,
     b"DEATH\x00" as *const u8 as *const libc::c_char,
     b"RUINATION\x00" as *const u8 as *const libc::c_char,
     b"DESTRUC\x00" as *const u8 as *const libc::c_char,
     b"UNINT\x00" as *const u8 as *const libc::c_char,
     b"UNSTR\x00" as *const u8 as *const libc::c_char,
     b"UNCON\x00" as *const u8 as *const libc::c_char,
     b"UNCHR\x00" as *const u8 as *const libc::c_char,
     b"UNDEX\x00" as *const u8 as *const libc::c_char,
     b"UNWIS\x00" as *const u8 as *const libc::c_char,
     b"STATLOSS\x00" as *const u8 as *const libc::c_char,
     b"HISTATLOSS\x00" as *const u8 as *const libc::c_char,
     b"EXPLOSS\x00" as *const u8 as *const libc::c_char,
     b"HIEXPLOSS\x00" as *const u8 as *const libc::c_char,
     b"SUMMON_MONST\x00" as *const u8 as *const libc::c_char,
     b"PARALYZE\x00" as *const u8 as *const libc::c_char,
     b"HALLU\x00" as *const u8 as *const libc::c_char,
     b"POISON\x00" as *const u8 as *const libc::c_char,
     b"HUNGER\x00" as *const u8 as *const libc::c_char,
     b"STUN\x00" as *const u8 as *const libc::c_char,
     b"CUTS\x00" as *const u8 as *const libc::c_char,
     b"PARANO\x00" as *const u8 as *const libc::c_char,
     b"CONFUSION\x00" as *const u8 as *const libc::c_char,
     b"BLIND\x00" as *const u8 as *const libc::c_char,
     b"PET_SUMMON\x00" as *const u8 as *const libc::c_char,
     b"CURE_PARA\x00" as *const u8 as *const libc::c_char,
     b"CURE_HALLU\x00" as *const u8 as *const libc::c_char,
     b"CURE_POIS\x00" as *const u8 as *const libc::c_char,
     b"CURE_HUNGER\x00" as *const u8 as *const libc::c_char,
     b"CURE_STUN\x00" as *const u8 as *const libc::c_char,
     b"CURE_CUTS\x00" as *const u8 as *const libc::c_char,
     b"CURE_FEAR\x00" as *const u8 as *const libc::c_char,
     b"CURE_CONF\x00" as *const u8 as *const libc::c_char,
     b"CURE_BLIND\x00" as *const u8 as *const libc::c_char,
     b"CURING\x00" as *const u8 as *const libc::c_char,
     b"DARKNESS\x00" as *const u8 as *const libc::c_char,
     b"LEV_TELE\x00" as *const u8 as *const libc::c_char,
     b"ACQUIREMENT\x00" as *const u8 as *const libc::c_char,
     b"WEIRD\x00" as *const u8 as *const libc::c_char,
     b"AGGRAVATE\x00" as *const u8 as *const libc::c_char,
     b"MUT\x00" as *const u8 as *const libc::c_char,
     b"CURE_INSANITY\x00" as *const u8 as *const libc::c_char,
     b"CURE_MUT\x00" as *const u8 as *const libc::c_char,
     b"LIGHT_ABSORBTION\x00" as *const u8 as *const libc::c_char,
     b"BA_FIRE_H\x00" as *const u8 as *const libc::c_char,
     b"BA_COLD_H\x00" as *const u8 as *const libc::c_char,
     b"BA_ELEC_H\x00" as *const u8 as *const libc::c_char,
     b"BA_ACID_H\x00" as *const u8 as *const libc::c_char,
     b"SPIN\x00" as *const u8 as *const libc::c_char,
     b"NOLDOR\x00" as *const u8 as *const libc::c_char,
     b"SPECTRAL\x00" as *const u8 as *const libc::c_char,
     b"JUMP\x00" as *const u8 as *const libc::c_char,
     b"DEST_TELE\x00" as *const u8 as *const libc::c_char,
     b"BA_POIS_4\x00" as *const u8 as *const libc::c_char,
     b"BA_COLD_4\x00" as *const u8 as *const libc::c_char,
     b"BA_FIRE_4\x00" as *const u8 as *const libc::c_char,
     b"BA_ACID_4\x00" as *const u8 as *const libc::c_char,
     b"BA_ELEC_4\x00" as *const u8 as *const libc::c_char,
     b"BR_ELEC\x00" as *const u8 as *const libc::c_char,
     b"BR_COLD\x00" as *const u8 as *const libc::c_char,
     b"BR_FIRE\x00" as *const u8 as *const libc::c_char,
     b"BR_ACID\x00" as *const u8 as *const libc::c_char,
     b"BR_POIS\x00" as *const u8 as *const libc::c_char,
     b"BR_MANY\x00" as *const u8 as *const libc::c_char,
     b"BR_CONF\x00" as *const u8 as *const libc::c_char,
     b"BR_SOUND\x00" as *const u8 as *const libc::c_char,
     b"BR_CHAOS\x00" as *const u8 as *const libc::c_char,
     b"BR_SHARD\x00" as *const u8 as *const libc::c_char,
     b"BR_BALANCE\x00" as *const u8 as *const libc::c_char,
     b"BR_LIGHT\x00" as *const u8 as *const libc::c_char,
     b"BR_POWER\x00" as *const u8 as *const libc::c_char,
     b"GROW_MOLD\x00" as *const u8 as *const libc::c_char,
     b"XXX198\x00" as *const u8 as *const libc::c_char,
     b"XXX199\x00" as *const u8 as *const libc::c_char,
     b"MUSIC\x00" as *const u8 as *const libc::c_char,
     b"\x00" as *const u8 as *const libc::c_char];
/*
 * Convert a "color letter" into an "actual" color
 * The colors are: dwsorgbuDWvyRGBU, as shown below
 */
#[no_mangle]
pub unsafe extern "C" fn color_char_to_attr(mut c: libc::c_char)
 -> libc::c_int {
    match c as libc::c_int {
        100 => { return 0 as libc::c_int }
        119 => { return 1 as libc::c_int }
        115 => { return 2 as libc::c_int }
        111 => { return 3 as libc::c_int }
        114 => { return 4 as libc::c_int }
        103 => { return 5 as libc::c_int }
        98 => { return 6 as libc::c_int }
        117 => { return 7 as libc::c_int }
        68 => { return 8 as libc::c_int }
        87 => { return 9 as libc::c_int }
        118 => { return 10 as libc::c_int }
        121 => { return 11 as libc::c_int }
        82 => { return 12 as libc::c_int }
        71 => { return 13 as libc::c_int }
        66 => { return 14 as libc::c_int }
        85 => { return 15 as libc::c_int }
        _ => { }
    }
    return -(1 as libc::c_int);
}
/*
 * Attr value-to-char convertion table
 */
#[no_mangle]
pub static mut conv_color: [byte_hack; 16] =
    ['d' as i32 as byte_hack, 'w' as i32 as byte_hack,
     's' as i32 as byte_hack, 'o' as i32 as byte_hack,
     'r' as i32 as byte_hack, 'g' as i32 as byte_hack,
     'b' as i32 as byte_hack, 'u' as i32 as byte_hack,
     'D' as i32 as byte_hack, 'W' as i32 as byte_hack,
     'v' as i32 as byte_hack, 'y' as i32 as byte_hack,
     'R' as i32 as byte_hack, 'G' as i32 as byte_hack,
     'B' as i32 as byte_hack, 'U' as i32 as byte_hack];
/* Values in re_info can be fixed, added, substracted or percented */
unsafe extern "C" fn monster_ego_modify(mut c: libc::c_char) -> byte_hack {
    match c as libc::c_int {
        43 => { return 0 as libc::c_int as byte_hack }
        45 => { return 1 as libc::c_int as byte_hack }
        61 => { return 2 as libc::c_int as byte_hack }
        37 => { return 3 as libc::c_int as byte_hack }
        _ => {
            msg_format(b"Unknown mego value modifier %c.\x00" as *const u8 as
                           *const libc::c_char, c as libc::c_int);
            return 0 as libc::c_int as byte_hack
        }
    };
}
/*
 * Implements fp stacks, for included files
 */
static mut fp_stack: [*mut FILE; 10] = [0 as *const FILE as *mut FILE; 10];
static mut fp_stack_idx: libc::c_int = 0 as libc::c_int;
/*
 * Must be caleld before the main loop
 */
unsafe extern "C" fn fp_stack_init(mut fp: *mut FILE) {
    fp_stack[0 as libc::c_int as usize] = fp;
    fp_stack_idx = 0 as libc::c_int;
}
unsafe extern "C" fn fp_stack_push(mut name: cptr) {
    if fp_stack_idx < 9 as libc::c_int {
        let mut buf: [libc::c_char; 1024] = [0; 1024];
        let mut fp: *mut FILE = 0 as *mut FILE;
        /* Build the filename */
        path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT,
                   name);
        /* Open the file */
        fp =
            my_fopen(buf.as_mut_ptr() as cptr,
                     b"r\x00" as *const u8 as *const libc::c_char);
        /* Parse it */
        if fp.is_null() {
            quit(format(b"Cannot open \'%s\' file.\x00" as *const u8 as
                            *const libc::c_char, name) as cptr);
        }
        printf(b"ibncluding %s\n\x00" as *const u8 as *const libc::c_char,
               name);
        fp_stack_idx += 1;
        fp_stack[fp_stack_idx as usize] = fp
    };
}
unsafe extern "C" fn fp_stack_pop() -> bool_ {
    if fp_stack_idx > 0 as libc::c_int {
        let fresh0 = fp_stack_idx;
        fp_stack_idx = fp_stack_idx - 1;
        let mut fp: *mut FILE = fp_stack[fresh0 as usize];
        my_fclose(fp);
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
/*
 * Must be used instead of my_fgets for teh main loop
 */
unsafe extern "C" fn my_fgets_dostack(mut buf: *mut libc::c_char,
                                      mut len: libc::c_int) -> libc::c_int {
    // End of a file
    if 0 as libc::c_int !=
           my_fgets(fp_stack[fp_stack_idx as usize], buf, len as huge_hack) {
        // If any left, use them
        if fp_stack_pop() != 0 {
            return my_fgets_dostack(buf, len)
        } else {
            // If not, this is the end
            return 1 as libc::c_int
        }
    } else { return 0 as libc::c_int };
}
/* ** Initialize from ascii template files ***/
/*
 * Grab one race flag from a textual string
 */
static mut unknown_shut_up: bool_ = 0 as libc::c_int as bool_;
unsafe extern "C" fn grab_one_class_flag(mut choice: *mut u32b,
                                         mut what: cptr) -> errr {
    let mut i: libc::c_int = 0;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Scan classes flags */
    i = 0 as libc::c_int;
    while i < max_c_idx as libc::c_int &&
              {
                  s =
                      c_name.offset((*class_info.offset(i as isize)).title as
                                        isize) as cptr;
                  !s.is_null()
              } {
        if streq(what, s) != 0 {
            let ref mut fresh1 =
                *choice.offset((i / 32 as libc::c_int) as isize);
            *fresh1 =
                (*fresh1 as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Oops */
    if unknown_shut_up == 0 {
        msg_format(b"Unknown class flag \'%s\'.\x00" as *const u8 as
                       *const libc::c_char, what);
    }
    /* Failure */
    return 1 as libc::c_int;
}
unsafe extern "C" fn grab_one_race_allow_flag(mut choice: *mut u32b,
                                              mut what: cptr) -> errr {
    let mut i: libc::c_int = 0;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Scan classes flags */
    i = 0 as libc::c_int;
    while i < max_rp_idx as libc::c_int &&
              {
                  s =
                      rp_name.offset((*race_info.offset(i as isize)).title as
                                         isize) as cptr;
                  !s.is_null()
              } {
        if streq(what, s) != 0 {
            let ref mut fresh2 =
                *choice.offset((i / 32 as libc::c_int) as isize);
            *fresh2 =
                (*fresh2 as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Oops */
    if unknown_shut_up == 0 {
        msg_format(b"(1)Unknown race flag \'%s\'.\x00" as *const u8 as
                       *const libc::c_char, what);
    }
    /* Failure */
    return 1 as libc::c_int;
}
/*
 * Grab one flag from a textual string
 */
unsafe extern "C" fn grab_one_skill_flag(mut f1: *mut u32b, mut what: cptr)
 -> errr {
    let mut i: libc::c_int = 0;
    /* Check flags1 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, s_info_flags1[i as usize]) != 0 {
            *f1 = (*f1 as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Oops */
    msg_format(b"(2)Unknown skill flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Error */
    return 1 as libc::c_int;
}
/*
 * Grab one flag from a textual string
 */
unsafe extern "C" fn grab_one_player_race_flag(mut f1: *mut u32b,
                                               mut f2: *mut u32b,
                                               mut what: cptr) -> errr {
    let mut i: libc::c_int = 0;
    /* Check flags1 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, rp_info_flags1[i as usize]) != 0 {
            *f1 = (*f1 as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Check flags2 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, rp_info_flags2[i as usize]) != 0 {
            *f2 = (*f2 as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Oops */
    msg_format(b"(2)Unknown race flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Error */
    return 1 as libc::c_int;
}
/* Get an activation number (good for artifacts, recipes, egos, and object kinds) */
#[no_mangle]
pub unsafe extern "C" fn get_activation(mut activation: *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while *activation_names[i as usize].offset(0 as libc::c_int as isize) != 0
          {
        if strncmp(activation_names[i as usize], activation,
                   19 as libc::c_int as libc::c_ulong) == 0 {
            return i
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
/*
 * Grab one flag in an object_kind from a textual string
 */
unsafe extern "C" fn grab_one_race_kind_flag(mut f1: *mut u32b,
                                             mut f2: *mut u32b,
                                             mut f3: *mut u32b,
                                             mut f4: *mut u32b,
                                             mut f5: *mut u32b,
                                             mut esp: *mut u32b,
                                             mut what: cptr) -> errr {
    let mut i: libc::c_int = 0;
    /* Check flags1 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags1[i as usize]) != 0 {
            *f1 = (*f1 as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Check flags2 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags2[i as usize]) != 0 {
            *f2 = (*f2 as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Check flags2 -- traps*/
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags2_trap[i as usize]) != 0 {
            *f3 = (*f3 as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Check flags3 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags3[i as usize]) != 0 {
            *f3 = (*f3 as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Check flags4 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags4[i as usize]) != 0 {
            *f4 = (*f4 as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Check flags5 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags5[i as usize]) != 0 {
            *f5 = (*f5 as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Check esp_flags */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, esp_flags[i as usize]) != 0 {
            *esp = (*esp as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Oops */
    msg_format(b"Unknown object flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Error */
    return 1 as libc::c_int;
}
/*
 * Initialize the "player" arrays, by parsing an ascii "template" file
 */
#[no_mangle]
pub unsafe extern "C" fn init_player_info_txt(mut fp: *mut FILE,
                                              mut buf: *mut libc::c_char)
 -> errr {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut z: libc::c_int = 0;
    let mut powers: libc::c_int = 0 as libc::c_int;
    let mut lev: libc::c_int = 1 as libc::c_int;
    let mut tit_idx: libc::c_int = 0 as libc::c_int;
    let mut spec_idx: libc::c_int = 0 as libc::c_int;
    let mut cur_ab: libc::c_int = -(1 as libc::c_int);
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Not ready yet */
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    /* Current entry */
    let mut rp_ptr_0: *mut player_race = 0 as *mut player_race;
    let mut rmp_ptr_0: *mut player_race_mod = 0 as *mut player_race_mod;
    let mut c_ptr: *mut player_class = 0 as *mut player_class;
    let mut s_ptr: *mut player_spec = 0 as *mut player_spec;
    let mut mc_ptr: *mut meta_class_type = 0 as *mut meta_class_type;
    /* Just before the first record */
    error_idx = -(1 as libc::c_int) as s16b;
    /* Just before the first line */
    error_line = -(1 as libc::c_int) as s16b;
    /* Start the "fake" stuff */
    (*rp_head).name_size = 0 as libc::c_int as u32b;
    (*rp_head).text_size = 0 as libc::c_int as u32b;
    (*rmp_head).name_size = 0 as libc::c_int as u32b;
    (*rmp_head).text_size = 0 as libc::c_int as u32b;
    (*c_head).name_size = 0 as libc::c_int as u32b;
    (*c_head).text_size = 0 as libc::c_int as u32b;
    /* Init general skills */
    z = 0 as libc::c_int;
    while z < 200 as libc::c_int {
        gen_skill_basem[z as usize] = 0 as libc::c_int as libc::c_char;
        gen_skill_base[z as usize] = 0 as libc::c_int as u32b;
        gen_skill_modm[z as usize] = 0 as libc::c_int as libc::c_char;
        gen_skill_mod[z as usize] = 0 as libc::c_int as s16b;
        z += 1
    }
    /* Parse */
    fp_stack_init(fp);
    /* Next... */
    while 0 as libc::c_int == my_fgets_dostack(buf, 1024 as libc::c_int) {
        /* Advance the line number */
        error_line += 1;
        /* Skip comments and blank lines */
        if *buf.offset(0 as libc::c_int as isize) == 0 ||
               *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '#' as i32 {
            continue ;
        }
        /* Verify correct "colon" format */
        if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
           {
            return 1 as libc::c_int
        }
        /* Hack -- Process 'V' for "Version" */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'V' as i32
           {
            let mut v1: libc::c_int = 0;
            let mut v2: libc::c_int = 0;
            let mut v3: libc::c_int = 0;
            /* Scan for the values */
            if 3 as libc::c_int !=
                   sscanf(buf.offset(2 as libc::c_int as isize),
                          b"%d.%d.%d\x00" as *const u8 as *const libc::c_char,
                          &mut v1 as *mut libc::c_int,
                          &mut v2 as *mut libc::c_int,
                          &mut v3 as *mut libc::c_int) {
                return 2 as libc::c_int
            }
            /* Okay to proceed */
            okay = 1 as libc::c_int as bool_
        } else {
            /* No version yet */
            if okay == 0 { return 2 as libc::c_int }
            /* Included file */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '<' as i32 {
                fp_stack_push(buf.offset(2 as libc::c_int as isize) as cptr);
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'I' as i32 {
                error_idx = -(1 as libc::c_int) as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'H' as i32 {
                let mut idx: libc::c_int = 0;
                let mut zz: [*mut libc::c_char; 6] =
                    [0 as *mut libc::c_char; 6];
                /* Reinit error_idx */
                /* Process 'H' for "History" */
                /* Scan for the values */
                if tokenize(buf.offset(2 as libc::c_int as isize),
                            6 as libc::c_int as s16b, zz.as_mut_ptr(),
                            ':' as i32 as libc::c_char,
                            ':' as i32 as libc::c_char) as libc::c_int !=
                       6 as libc::c_int {
                    return 1 as libc::c_int
                }
                idx = atoi(zz[0 as libc::c_int as usize]);
                (*bg.offset(idx as isize)).roll =
                    atoi(zz[1 as libc::c_int as usize]) as byte_hack;
                (*bg.offset(idx as isize)).chart =
                    atoi(zz[2 as libc::c_int as usize]) as s16b;
                (*bg.offset(idx as isize)).next =
                    atoi(zz[3 as libc::c_int as usize]) as s16b;
                (*bg.offset(idx as isize)).bonus =
                    atoi(zz[4 as libc::c_int as usize]) as byte_hack;
                (*rp_head).text_size = (*rp_head).text_size.wrapping_add(1);
                (*bg.offset(idx as isize)).info =
                    (*rp_head).text_size as s32b;
                /* Append chars to the name */
                strcpy(rp_text.offset((*rp_head).text_size as isize),
                       zz[5 as libc::c_int as usize]);
                /* Advance the index */
                (*rp_head).text_size =
                    ((*rp_head).text_size as
                         libc::c_ulong).wrapping_add(strlen(zz[5 as
                                                                   libc::c_int
                                                                   as usize]))
                        as u32b as u32b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'G' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'k' as i32 {
                let mut val: libc::c_long = 0;
                let mut mod_0: libc::c_long = 0;
                let mut i_0: libc::c_long = 0;
                let mut name: [libc::c_char; 200] = [0; 200];
                let mut v: libc::c_char = 0;
                let mut m: libc::c_char = 0;
                /* Process 'G:k' for "General skills" */
                /* Scan for the values */
                if 5 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%c%ld:%c%ld:%s\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut v as *mut libc::c_char,
                              &mut val as *mut libc::c_long,
                              &mut m as *mut libc::c_char,
                              &mut mod_0 as *mut libc::c_long,
                              name.as_mut_ptr()) {
                    return 1 as libc::c_int
                }
                i_0 = find_skill(name.as_mut_ptr() as cptr) as libc::c_long;
                if i_0 == -(1 as libc::c_int) as libc::c_long {
                    return 1 as libc::c_int
                }
                gen_skill_basem[i_0 as usize] =
                    monster_ego_modify(v) as libc::c_char;
                gen_skill_base[i_0 as usize] = val as u32b;
                gen_skill_modm[i_0 as usize] =
                    monster_ego_modify(m) as libc::c_char;
                gen_skill_mod[i_0 as usize] = mod_0 as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'R' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'N' as i32 {
                /* Process 'N' for "New/Number/Name" */
                /* Find the colon before the name */
                s = strchr(buf.offset(4 as libc::c_int as isize), ':' as i32);
                /* Verify that colon */
                if s.is_null() { return 1 as libc::c_int }
                /* Nuke the colon, advance to the name */
                let fresh3 = s;
                s = s.offset(1);
                *fresh3 = '\u{0}' as i32 as libc::c_char;
                /* Paranoia -- require a name */
                if *s == 0 { return 1 as libc::c_int }
                /* Get the index */
                i = atoi(buf.offset(4 as libc::c_int as isize));
                /* Verify information */
                if i < error_idx as libc::c_int { return 4 as libc::c_int }
                /* Verify information */
                if i >= (*rp_head).info_num as libc::c_int {
                    return 2 as libc::c_int
                }
                /* Save the index */
                error_idx = i as s16b;
                /* Point at the "info" */
                rp_ptr_0 =
                    &mut *race_info.offset(i as isize) as *mut player_race;
                /* Hack -- Verify space */
                if ((*rp_head).name_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_name_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the name index */
                if (*rp_ptr_0).title == 0 {
                    (*rp_head).name_size =
                        (*rp_head).name_size.wrapping_add(1);
                    (*rp_ptr_0).title = (*rp_head).name_size as s32b
                }
                /* Append chars to the name */
                strcpy(rp_name.offset((*rp_head).name_size as isize), s);
                /* Advance the index */
                (*rp_head).name_size =
                    ((*rp_head).name_size as
                         libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                        u32b;
                (*rp_ptr_0).powers[3 as libc::c_int as usize] =
                    -(1 as libc::c_int) as s16b;
                (*rp_ptr_0).powers[2 as libc::c_int as usize] =
                    (*rp_ptr_0).powers[3 as libc::c_int as usize];
                (*rp_ptr_0).powers[1 as libc::c_int as usize] =
                    (*rp_ptr_0).powers[2 as libc::c_int as usize];
                (*rp_ptr_0).powers[0 as libc::c_int as usize] =
                    (*rp_ptr_0).powers[1 as libc::c_int as usize];
                powers = 0 as libc::c_int;
                lev = 1 as libc::c_int;
                cur_ab = 0 as libc::c_int;
                z = 0 as libc::c_int;
                while z < 10 as libc::c_int {
                    (*rp_ptr_0).abilities[z as usize].level =
                        -(1 as libc::c_int) as s16b;
                    z += 1
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'R' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'D' as i32 {
                /* Process 'D' for "Description" */
                /* Acquire the text */
                s = buf.offset(4 as libc::c_int as isize);
                /* Hack -- Verify space */
                if ((*rp_head).text_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_text_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the text index */
                if (*rp_ptr_0).desc == 0 {
                    (*rp_head).text_size =
                        (*rp_head).text_size.wrapping_add(1);
                    (*rp_ptr_0).desc = (*rp_head).text_size as s32b;
                    /* Append chars to the name */
                    strcpy(rp_text.offset((*rp_head).text_size as isize), s);
                    /* Advance the index */
                    (*rp_head).text_size =
                        ((*rp_head).text_size as
                             libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                            u32b
                } else {
                    /* Append chars to the name */
                    strcpy(rp_text.offset((*rp_head).text_size as isize),
                           format(b"\n%s\x00" as *const u8 as
                                      *const libc::c_char, s));
                    /* Advance the index */
                    (*rp_head).text_size =
                        ((*rp_head).text_size as
                             libc::c_ulong).wrapping_add(strlen(s).wrapping_add(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong))
                            as u32b as u32b
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'R' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'E' as i32 {
                let mut s_0: [libc::c_int; 6] = [0; 6];
                let mut z_0: libc::c_int = 0;
                /* Process 'E' for "body parts" */
                /* Scan for the values */
                if 6 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d:%d:%d:%d:%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut *s_0.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_0.as_mut_ptr().offset(1 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_0.as_mut_ptr().offset(2 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_0.as_mut_ptr().offset(3 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_0.as_mut_ptr().offset(4 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_0.as_mut_ptr().offset(5 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                z_0 = 0 as libc::c_int;
                while z_0 < 6 as libc::c_int {
                    (*rp_ptr_0).body_parts[z_0 as usize] =
                        s_0[z_0 as usize] as byte_hack;
                    z_0 += 1
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'R' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'R' as i32 {
                let mut s_1: [libc::c_int; 2] = [0; 2];
                /* Process 'R' for "flag level" */
                /* Scan for the values */
                if 2 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut *s_1.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_1.as_mut_ptr().offset(1 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                lev = s_1[0 as libc::c_int as usize];
                (*rp_ptr_0).opval[lev as usize] =
                    s_1[1 as libc::c_int as usize] as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'R' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'S' as i32 {
                let mut s_2: [libc::c_int; 7] = [0; 7];
                let mut z_1: libc::c_int = 0;
                /* Process 'S' for "Stats" */
                /* Scan for the values */
                if 7 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d:%d:%d:%d:%d:%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut *s_2.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_2.as_mut_ptr().offset(1 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_2.as_mut_ptr().offset(2 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_2.as_mut_ptr().offset(3 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_2.as_mut_ptr().offset(4 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_2.as_mut_ptr().offset(5 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_2.as_mut_ptr().offset(6 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                (*rp_ptr_0).luck =
                    s_2[6 as libc::c_int as usize] as libc::c_char;
                z_1 = 0 as libc::c_int;
                while z_1 < 6 as libc::c_int {
                    (*rp_ptr_0).r_adj[z_1 as usize] =
                        s_2[z_1 as usize] as s16b;
                    z_1 += 1
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'R' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'Z' as i32 {
                let mut i_1: libc::c_int = 0;
                /* Process 'Z' for "powers" */
                /* Acquire the text */
                s = buf.offset(4 as libc::c_int as isize);
                /* Find it in the list */
                i_1 = 0 as libc::c_int;
                while i_1 < power_max as libc::c_int {
                    if strcasecmp(s, (*powers_type.offset(i_1 as isize)).name)
                           == 0 {
                        break ;
                    }
                    i_1 += 1
                }
                if i_1 == power_max as libc::c_int { return 6 as libc::c_int }
                let fresh4 = powers;
                powers = powers + 1;
                (*rp_ptr_0).powers[fresh4 as usize] = i_1 as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'R' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'K' as i32 {
                let mut s_3: [libc::c_int; 8] = [0; 8];
                /* Process 'K' for "sKills" */
                /* Scan for the values */
                if 8 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d:%d:%d:%d:%d:%d:%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut *s_3.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_3.as_mut_ptr().offset(1 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_3.as_mut_ptr().offset(2 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_3.as_mut_ptr().offset(3 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_3.as_mut_ptr().offset(4 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_3.as_mut_ptr().offset(5 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_3.as_mut_ptr().offset(6 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_3.as_mut_ptr().offset(7 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                (*rp_ptr_0).r_dis = s_3[0 as libc::c_int as usize] as s16b;
                (*rp_ptr_0).r_dev = s_3[1 as libc::c_int as usize] as s16b;
                (*rp_ptr_0).r_sav = s_3[2 as libc::c_int as usize] as s16b;
                (*rp_ptr_0).r_stl = s_3[3 as libc::c_int as usize] as s16b;
                (*rp_ptr_0).r_srh = s_3[4 as libc::c_int as usize] as s16b;
                (*rp_ptr_0).r_fos = s_3[5 as libc::c_int as usize] as s16b;
                (*rp_ptr_0).r_thn = s_3[6 as libc::c_int as usize] as s16b;
                (*rp_ptr_0).r_thb = s_3[7 as libc::c_int as usize] as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'R' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'k' as i32 {
                let mut val_0: libc::c_long = 0;
                let mut mod_1: libc::c_long = 0;
                let mut i_2: libc::c_long = 0;
                let mut name_0: [libc::c_char; 200] = [0; 200];
                let mut v_0: libc::c_char = 0;
                let mut m_0: libc::c_char = 0;
                /* Process 'k' for "skills" */
                /* Scan for the values */
                if 5 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%c%ld:%c%ld:%s\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut v_0 as *mut libc::c_char,
                              &mut val_0 as *mut libc::c_long,
                              &mut m_0 as *mut libc::c_char,
                              &mut mod_1 as *mut libc::c_long,
                              name_0.as_mut_ptr()) {
                    return 1 as libc::c_int
                }
                i_2 = find_skill(name_0.as_mut_ptr() as cptr) as libc::c_long;
                if i_2 == -(1 as libc::c_int) as libc::c_long {
                    return 1 as libc::c_int
                }
                (*rp_ptr_0).skill_basem[i_2 as usize] =
                    monster_ego_modify(v_0) as libc::c_char;
                (*rp_ptr_0).skill_base[i_2 as usize] = val_0 as u32b;
                (*rp_ptr_0).skill_modm[i_2 as usize] =
                    monster_ego_modify(m_0) as libc::c_char;
                (*rp_ptr_0).skill_mod[i_2 as usize] = mod_1 as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'R' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'b' as i32 {
                let mut sec: *mut libc::c_char = 0 as *mut libc::c_char;
                /* Process 'b' for "abilities" */
                /* Scan for the values */
                sec =
                    strchr(buf.offset(4 as libc::c_int as isize), ':' as i32);
                if sec.is_null() { return 1 as libc::c_int }
                *sec = '\u{0}' as i32 as libc::c_char;
                sec = sec.offset(1);
                if *sec == 0 { return 1 as libc::c_int }
                i = find_ability(sec as cptr) as libc::c_int;
                if i == -(1 as libc::c_int) { return 1 as libc::c_int }
                (*rp_ptr_0).abilities[cur_ab as usize].ability = i as s16b;
                (*rp_ptr_0).abilities[cur_ab as usize].level =
                    atoi(buf.offset(4 as libc::c_int as isize)) as s16b;
                cur_ab += 1
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'R' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'M' as i32 {
                let mut s_4: [libc::c_int; 10] = [0; 10];
                /* Process 'M' for "Mods" */
                /* Scan for the values */
                if 10 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d:%d:%d:%d:%d:%d:%d:%d:%d\x00" as
                                  *const u8 as *const libc::c_char,
                              &mut *s_4.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_4.as_mut_ptr().offset(1 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_4.as_mut_ptr().offset(2 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_4.as_mut_ptr().offset(3 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_4.as_mut_ptr().offset(4 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_4.as_mut_ptr().offset(5 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_4.as_mut_ptr().offset(6 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_4.as_mut_ptr().offset(7 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_4.as_mut_ptr().offset(8 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_4.as_mut_ptr().offset(9 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                (*rp_ptr_0).b_age =
                    s_4[0 as libc::c_int as usize] as byte_hack;
                (*rp_ptr_0).m_age =
                    s_4[1 as libc::c_int as usize] as byte_hack;
                (*rp_ptr_0).m_b_ht =
                    s_4[2 as libc::c_int as usize] as byte_hack;
                (*rp_ptr_0).m_m_ht =
                    s_4[3 as libc::c_int as usize] as byte_hack;
                (*rp_ptr_0).m_b_wt =
                    s_4[4 as libc::c_int as usize] as byte_hack;
                (*rp_ptr_0).m_m_wt =
                    s_4[5 as libc::c_int as usize] as byte_hack;
                (*rp_ptr_0).f_b_ht =
                    s_4[6 as libc::c_int as usize] as byte_hack;
                (*rp_ptr_0).f_m_ht =
                    s_4[7 as libc::c_int as usize] as byte_hack;
                (*rp_ptr_0).f_b_wt =
                    s_4[8 as libc::c_int as usize] as byte_hack;
                (*rp_ptr_0).f_m_wt =
                    s_4[9 as libc::c_int as usize] as byte_hack
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'R' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'P' as i32 {
                let mut s_5: [libc::c_int; 4] = [0; 4];
                /* Process 'P' for "xtra" */
                /* Scan for the values */
                if 4 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d:%d:%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut *s_5.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_5.as_mut_ptr().offset(1 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_5.as_mut_ptr().offset(2 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_5.as_mut_ptr().offset(3 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                (*rp_ptr_0).r_mhp =
                    s_5[0 as libc::c_int as usize] as byte_hack;
                (*rp_ptr_0).r_exp = s_5[1 as libc::c_int as usize] as u16b;
                (*rp_ptr_0).infra =
                    s_5[2 as libc::c_int as usize] as byte_hack;
                (*rp_ptr_0).chart = s_5[3 as libc::c_int as usize] as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'R' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'G' as i32 {
                /* Process 'G' for "Player flags" (multiple lines) */
                /* Parse every entry */
                s = buf.offset(4 as libc::c_int as isize);
                while *s != 0 {
                    /* Find the end of this entry */
                    t = s;
                    while *t as libc::c_int != 0 &&
                              *t as libc::c_int != ' ' as i32 &&
                              *t as libc::c_int != '|' as i32 {
                        /* loop */
                        t = t.offset(1)
                    }
                    /* Nuke and skip any dividers */
                    if *t != 0 {
                        let fresh5 = t;
                        t = t.offset(1);
                        *fresh5 = '\u{0}' as i32 as libc::c_char;
                        while *t as libc::c_int == ' ' as i32 ||
                                  *t as libc::c_int == '|' as i32 {
                            t = t.offset(1)
                        }
                    }
                    /* Parse this entry */
                    if 0 as libc::c_int !=
                           grab_one_player_race_flag(&mut (*rp_ptr_0).flags1,
                                                     &mut (*rp_ptr_0).flags2,
                                                     s as cptr) {
                        return 5 as libc::c_int
                    }
                    /* Start the next entry */
                    s = t
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'R' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'F' as i32 {
                /* Process 'F' for "level Flags" (multiple lines) */
                /* Parse every entry */
                s = buf.offset(4 as libc::c_int as isize);
                while *s != 0 {
                    /* Find the end of this entry */
                    t = s;
                    while *t as libc::c_int != 0 &&
                              *t as libc::c_int != ' ' as i32 &&
                              *t as libc::c_int != '|' as i32 {
                        /* loop */
                        t = t.offset(1)
                    }
                    /* Nuke and skip any dividers */
                    if *t != 0 {
                        let fresh6 = t;
                        t = t.offset(1);
                        *fresh6 = '\u{0}' as i32 as libc::c_char;
                        while *t as libc::c_int == ' ' as i32 ||
                                  *t as libc::c_int == '|' as i32 {
                            t = t.offset(1)
                        }
                    }
                    /* Parse this entry */
                    if 0 as libc::c_int !=
                           grab_one_race_kind_flag(&mut *(*rp_ptr_0).oflags1.as_mut_ptr().offset(lev
                                                                                                     as
                                                                                                     isize),
                                                   &mut *(*rp_ptr_0).oflags2.as_mut_ptr().offset(lev
                                                                                                     as
                                                                                                     isize),
                                                   &mut *(*rp_ptr_0).oflags3.as_mut_ptr().offset(lev
                                                                                                     as
                                                                                                     isize),
                                                   &mut *(*rp_ptr_0).oflags4.as_mut_ptr().offset(lev
                                                                                                     as
                                                                                                     isize),
                                                   &mut *(*rp_ptr_0).oflags5.as_mut_ptr().offset(lev
                                                                                                     as
                                                                                                     isize),
                                                   &mut *(*rp_ptr_0).oesp.as_mut_ptr().offset(lev
                                                                                                  as
                                                                                                  isize),
                                                   s as cptr) {
                        return 5 as libc::c_int
                    }
                    /* Start the next entry */
                    s = t
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'R' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'O' as i32 {
                let mut s_6: [libc::c_int; 5] = [0; 5];
                /* Process 'O' for "Object birth" */
                /* Scan for the values */
                if 5 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d:%d:%dd%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut *s_6.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_6.as_mut_ptr().offset(1 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_6.as_mut_ptr().offset(4 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_6.as_mut_ptr().offset(2 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_6.as_mut_ptr().offset(3 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int) {
                    s_6[4 as libc::c_int as usize] = 0 as libc::c_int;
                    if 4 as libc::c_int !=
                           sscanf(buf.offset(4 as libc::c_int as isize),
                                  b"%d:%d:%dd%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut *s_6.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                                      as *mut libc::c_int,
                                  &mut *s_6.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize)
                                      as *mut libc::c_int,
                                  &mut *s_6.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as isize)
                                      as *mut libc::c_int,
                                  &mut *s_6.as_mut_ptr().offset(3 as
                                                                    libc::c_int
                                                                    as isize)
                                      as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                }
                (*rp_ptr_0).obj_pval[(*rp_ptr_0).obj_num as usize] =
                    s_6[4 as libc::c_int as usize] as s16b;
                (*rp_ptr_0).obj_tval[(*rp_ptr_0).obj_num as usize] =
                    s_6[0 as libc::c_int as usize] as s16b;
                (*rp_ptr_0).obj_sval[(*rp_ptr_0).obj_num as usize] =
                    s_6[1 as libc::c_int as usize] as s16b;
                (*rp_ptr_0).obj_dd[(*rp_ptr_0).obj_num as usize] =
                    s_6[2 as libc::c_int as usize] as s16b;
                let fresh7 = (*rp_ptr_0).obj_num;
                (*rp_ptr_0).obj_num = (*rp_ptr_0).obj_num + 1;
                (*rp_ptr_0).obj_ds[fresh7 as usize] =
                    s_6[3 as libc::c_int as usize] as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'R' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'C' as i32 {
                /* Process 'C' for "Class choice flags" (multiple lines) */
                /* Parse every entry */
                s = buf.offset(4 as libc::c_int as isize);
                while *s != 0 {
                    /* Find the end of this entry */
                    t = s;
                    while *t as libc::c_int != 0 &&
                              *t as libc::c_int != ' ' as i32 &&
                              *t as libc::c_int != '|' as i32 {
                        /* loop */
                        t = t.offset(1)
                    }
                    /* Nuke and skip any dividers */
                    if *t != 0 {
                        let fresh8 = t;
                        t = t.offset(1);
                        *fresh8 = '\u{0}' as i32 as libc::c_char;
                        while *t as libc::c_int == ' ' as i32 ||
                                  *t as libc::c_int == '|' as i32 {
                            t = t.offset(1)
                        }
                    }
                    /* Parse this entry */
                    if 0 as libc::c_int !=
                           grab_one_class_flag((*rp_ptr_0).choice.as_mut_ptr(),
                                               s as cptr) {
                        return 5 as libc::c_int
                    }
                    /* Start the next entry */
                    s = t
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'S' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'N' as i32 {
                /* Process 'N' for "New/Number/Name" */
                /* Find the colon before the name */
                s = strchr(buf.offset(4 as libc::c_int as isize), ':' as i32);
                /* Verify that colon */
                if s.is_null() { return 1 as libc::c_int }
                /* Nuke the colon, advance to the name */
                let fresh9 = s;
                s = s.offset(1);
                *fresh9 = '\u{0}' as i32 as libc::c_char;
                /* Paranoia -- require a name */
                if *s == 0 { return 1 as libc::c_int }
                /* Get the index */
                i = atoi(buf.offset(4 as libc::c_int as isize));
                /* Verify information */
                if i < error_idx as libc::c_int { return 4 as libc::c_int }
                /* Verify information */
                if i >= (*rmp_head).info_num as libc::c_int {
                    return 2 as libc::c_int
                }
                /* Save the index */
                error_idx = i as s16b;
                /* Point at the "info" */
                rmp_ptr_0 =
                    &mut *race_mod_info.offset(i as isize) as
                        *mut player_race_mod;
                /* Hack -- Verify space */
                if ((*rmp_head).name_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_name_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the name index */
                if (*rmp_ptr_0).title == 0 {
                    (*rmp_head).name_size =
                        (*rmp_head).name_size.wrapping_add(1);
                    (*rmp_ptr_0).title = (*rmp_head).name_size as s32b
                }
                /* Append chars to the name */
                strcpy(rmp_name.offset((*rmp_head).name_size as isize), s);
                /* Advance the index */
                (*rmp_head).name_size =
                    ((*rmp_head).name_size as
                         libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                        u32b;
                (*rmp_ptr_0).powers[3 as libc::c_int as usize] =
                    -(1 as libc::c_int) as s16b;
                (*rmp_ptr_0).powers[2 as libc::c_int as usize] =
                    (*rmp_ptr_0).powers[3 as libc::c_int as usize];
                (*rmp_ptr_0).powers[1 as libc::c_int as usize] =
                    (*rmp_ptr_0).powers[2 as libc::c_int as usize];
                (*rmp_ptr_0).powers[0 as libc::c_int as usize] =
                    (*rmp_ptr_0).powers[1 as libc::c_int as usize];
                powers = 0 as libc::c_int;
                lev = 1 as libc::c_int;
                cur_ab = 0 as libc::c_int;
                z = 0 as libc::c_int;
                while z < 10 as libc::c_int {
                    (*rmp_ptr_0).abilities[z as usize].level =
                        -(1 as libc::c_int) as s16b;
                    z += 1
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'S' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'D' as i32 {
                /* Process 'D' for "Description" */
                /* Acquire the text */
                s = buf.offset(6 as libc::c_int as isize);
                if *buf.offset(4 as libc::c_int as isize) as libc::c_int ==
                       'A' as i32 {
                    (*rmp_ptr_0).place = 1 as libc::c_int as bool_
                } else { (*rmp_ptr_0).place = 0 as libc::c_int as bool_ }
                /* Hack -- Verify space */
                if ((*rmp_head).text_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_text_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the text index */
                if (*rmp_ptr_0).desc == 0 {
                    (*rmp_head).text_size =
                        (*rmp_head).text_size.wrapping_add(1);
                    (*rmp_ptr_0).desc = (*rmp_head).text_size as s32b;
                    /* Append chars to the name */
                    strcpy(rmp_text.offset((*rmp_head).text_size as isize),
                           s);
                    /* Advance the index */
                    (*rmp_head).text_size =
                        ((*rmp_head).text_size as
                             libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                            u32b
                } else {
                    /* Append chars to the name */
                    strcpy(rmp_text.offset((*rmp_head).text_size as isize),
                           format(b"\n%s\x00" as *const u8 as
                                      *const libc::c_char, s));
                    /* Advance the index */
                    (*rmp_head).text_size =
                        ((*rmp_head).text_size as
                             libc::c_ulong).wrapping_add(strlen(s).wrapping_add(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong))
                            as u32b as u32b
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'S' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'E' as i32 {
                let mut s_7: [libc::c_int; 6] = [0; 6];
                let mut z_2: libc::c_int = 0;
                /* Process 'E' for "body parts" */
                /* Scan for the values */
                if 6 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d:%d:%d:%d:%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut *s_7.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_7.as_mut_ptr().offset(1 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_7.as_mut_ptr().offset(2 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_7.as_mut_ptr().offset(3 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_7.as_mut_ptr().offset(4 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_7.as_mut_ptr().offset(5 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                z_2 = 0 as libc::c_int;
                while z_2 < 6 as libc::c_int {
                    (*rmp_ptr_0).body_parts[z_2 as usize] =
                        s_7[z_2 as usize] as libc::c_char;
                    z_2 += 1
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'S' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'R' as i32 {
                let mut s_8: [libc::c_int; 2] = [0; 2];
                /* Process 'R' for "flag level" */
                /* Scan for the values */
                if 2 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut *s_8.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_8.as_mut_ptr().offset(1 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                lev = s_8[0 as libc::c_int as usize];
                (*rmp_ptr_0).opval[lev as usize] =
                    s_8[1 as libc::c_int as usize] as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'S' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'S' as i32 {
                let mut s_9: [libc::c_int; 8] = [0; 8];
                let mut z_3: libc::c_int = 0;
                /* Process 'S' for "Stats" */
                /* Scan for the values */
                if 8 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d:%d:%d:%d:%d:%d:%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut *s_9.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_9.as_mut_ptr().offset(1 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_9.as_mut_ptr().offset(2 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_9.as_mut_ptr().offset(3 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_9.as_mut_ptr().offset(4 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_9.as_mut_ptr().offset(5 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_9.as_mut_ptr().offset(6 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int,
                              &mut *s_9.as_mut_ptr().offset(7 as libc::c_int
                                                                as isize) as
                                  *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                (*rmp_ptr_0).mana = s_9[7 as libc::c_int as usize] as s16b;
                (*rmp_ptr_0).luck =
                    s_9[6 as libc::c_int as usize] as libc::c_char;
                z_3 = 0 as libc::c_int;
                while z_3 < 6 as libc::c_int {
                    (*rmp_ptr_0).r_adj[z_3 as usize] =
                        s_9[z_3 as usize] as s16b;
                    z_3 += 1
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'S' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'Z' as i32 {
                let mut i_3: libc::c_int = 0;
                /* Process 'Z' for "powers" */
                /* Acquire the text */
                s = buf.offset(4 as libc::c_int as isize);
                /* Find it in the list */
                i_3 = 0 as libc::c_int;
                while i_3 < power_max as libc::c_int {
                    if strcasecmp(s, (*powers_type.offset(i_3 as isize)).name)
                           == 0 {
                        break ;
                    }
                    i_3 += 1
                }
                if i_3 == power_max as libc::c_int { return 6 as libc::c_int }
                let fresh10 = powers;
                powers = powers + 1;
                (*rmp_ptr_0).powers[fresh10 as usize] = i_3 as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'S' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'k' as i32 {
                let mut val_1: libc::c_long = 0;
                let mut mod_2: libc::c_long = 0;
                let mut i_4: libc::c_long = 0;
                let mut name_1: [libc::c_char; 200] = [0; 200];
                let mut v_1: libc::c_char = 0;
                let mut m_1: libc::c_char = 0;
                /* Process 'k' for "skills" */
                /* Scan for the values */
                if 5 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%c%ld:%c%ld:%s\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut v_1 as *mut libc::c_char,
                              &mut val_1 as *mut libc::c_long,
                              &mut m_1 as *mut libc::c_char,
                              &mut mod_2 as *mut libc::c_long,
                              name_1.as_mut_ptr()) {
                    return 1 as libc::c_int
                }
                i_4 = find_skill(name_1.as_mut_ptr() as cptr) as libc::c_long;
                if i_4 == -(1 as libc::c_int) as libc::c_long {
                    return 1 as libc::c_int
                }
                (*rmp_ptr_0).skill_basem[i_4 as usize] =
                    monster_ego_modify(v_1) as libc::c_char;
                (*rmp_ptr_0).skill_base[i_4 as usize] = val_1 as u32b;
                (*rmp_ptr_0).skill_modm[i_4 as usize] =
                    monster_ego_modify(m_1) as libc::c_char;
                (*rmp_ptr_0).skill_mod[i_4 as usize] = mod_2 as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'S' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'b' as i32 {
                let mut sec_0: *mut libc::c_char = 0 as *mut libc::c_char;
                /* Process 'b' for "abilities" */
                /* Scan for the values */
                sec_0 =
                    strchr(buf.offset(4 as libc::c_int as isize), ':' as i32);
                if sec_0.is_null() { return 1 as libc::c_int }
                *sec_0 = '\u{0}' as i32 as libc::c_char;
                sec_0 = sec_0.offset(1);
                if *sec_0 == 0 { return 1 as libc::c_int }
                i = find_ability(sec_0 as cptr) as libc::c_int;
                if i == -(1 as libc::c_int) { return 1 as libc::c_int }
                (*rmp_ptr_0).abilities[cur_ab as usize].ability = i as s16b;
                (*rmp_ptr_0).abilities[cur_ab as usize].level =
                    atoi(buf.offset(4 as libc::c_int as isize)) as s16b;
                cur_ab += 1
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'S' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'K' as i32 {
                let mut s_10: [libc::c_int; 8] = [0; 8];
                /* Process 'K' for "sKills" */
                /* Scan for the values */
                if 8 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d:%d:%d:%d:%d:%d:%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut *s_10.as_mut_ptr().offset(0 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_10.as_mut_ptr().offset(1 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_10.as_mut_ptr().offset(2 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_10.as_mut_ptr().offset(3 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_10.as_mut_ptr().offset(4 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_10.as_mut_ptr().offset(5 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_10.as_mut_ptr().offset(6 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_10.as_mut_ptr().offset(7 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                (*rmp_ptr_0).r_dis = s_10[0 as libc::c_int as usize] as s16b;
                (*rmp_ptr_0).r_dev = s_10[1 as libc::c_int as usize] as s16b;
                (*rmp_ptr_0).r_sav = s_10[2 as libc::c_int as usize] as s16b;
                (*rmp_ptr_0).r_stl = s_10[3 as libc::c_int as usize] as s16b;
                (*rmp_ptr_0).r_srh = s_10[4 as libc::c_int as usize] as s16b;
                (*rmp_ptr_0).r_fos = s_10[5 as libc::c_int as usize] as s16b;
                (*rmp_ptr_0).r_thn = s_10[6 as libc::c_int as usize] as s16b;
                (*rmp_ptr_0).r_thb = s_10[7 as libc::c_int as usize] as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'S' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'M' as i32 {
                let mut s_11: [libc::c_int; 10] = [0; 10];
                /* Process 'M' for "Mods" */
                /* Scan for the values */
                if 10 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d:%d:%d:%d:%d:%d:%d:%d:%d\x00" as
                                  *const u8 as *const libc::c_char,
                              &mut *s_11.as_mut_ptr().offset(0 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_11.as_mut_ptr().offset(1 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_11.as_mut_ptr().offset(2 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_11.as_mut_ptr().offset(3 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_11.as_mut_ptr().offset(4 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_11.as_mut_ptr().offset(5 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_11.as_mut_ptr().offset(6 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_11.as_mut_ptr().offset(7 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_11.as_mut_ptr().offset(8 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_11.as_mut_ptr().offset(9 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                (*rmp_ptr_0).b_age =
                    s_11[0 as libc::c_int as usize] as libc::c_char;
                (*rmp_ptr_0).m_age =
                    s_11[1 as libc::c_int as usize] as libc::c_char;
                (*rmp_ptr_0).m_b_ht =
                    s_11[2 as libc::c_int as usize] as libc::c_char;
                (*rmp_ptr_0).m_m_ht =
                    s_11[3 as libc::c_int as usize] as libc::c_char;
                (*rmp_ptr_0).m_b_wt =
                    s_11[4 as libc::c_int as usize] as libc::c_char;
                (*rmp_ptr_0).m_m_wt =
                    s_11[5 as libc::c_int as usize] as libc::c_char;
                (*rmp_ptr_0).f_b_ht =
                    s_11[6 as libc::c_int as usize] as libc::c_char;
                (*rmp_ptr_0).f_m_ht =
                    s_11[7 as libc::c_int as usize] as libc::c_char;
                (*rmp_ptr_0).f_b_wt =
                    s_11[8 as libc::c_int as usize] as libc::c_char;
                (*rmp_ptr_0).f_m_wt =
                    s_11[9 as libc::c_int as usize] as libc::c_char
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'S' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'P' as i32 {
                let mut s_12: [libc::c_int; 3] = [0; 3];
                /* Process 'P' for "xtra" */
                /* Scan for the values */
                if 3 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d:%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut *s_12.as_mut_ptr().offset(0 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_12.as_mut_ptr().offset(1 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_12.as_mut_ptr().offset(2 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                (*rmp_ptr_0).r_mhp =
                    s_12[0 as libc::c_int as usize] as libc::c_char;
                (*rmp_ptr_0).r_exp = s_12[1 as libc::c_int as usize] as s16b;
                (*rmp_ptr_0).infra =
                    s_12[2 as libc::c_int as usize] as libc::c_char
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'S' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'G' as i32 {
                /* Process 'G' for "Player flags" (multiple lines) */
                /* Parse every entry */
                s = buf.offset(4 as libc::c_int as isize);
                while *s != 0 {
                    /* Find the end of this entry */
                    t = s;
                    while *t as libc::c_int != 0 &&
                              *t as libc::c_int != ' ' as i32 &&
                              *t as libc::c_int != '|' as i32 {
                        /* loop */
                        t = t.offset(1)
                    }
                    /* Nuke and skip any dividers */
                    if *t != 0 {
                        let fresh11 = t;
                        t = t.offset(1);
                        *fresh11 = '\u{0}' as i32 as libc::c_char;
                        while *t as libc::c_int == ' ' as i32 ||
                                  *t as libc::c_int == '|' as i32 {
                            t = t.offset(1)
                        }
                    }
                    /* Parse this entry */
                    if 0 as libc::c_int !=
                           grab_one_player_race_flag(&mut (*rmp_ptr_0).flags1,
                                                     &mut (*rmp_ptr_0).flags2,
                                                     s as cptr) {
                        return 5 as libc::c_int
                    }
                    /* Start the next entry */
                    s = t
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'S' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'F' as i32 {
                /* Process 'F' for "level Flags" (multiple lines) */
                /* Parse every entry */
                s = buf.offset(4 as libc::c_int as isize);
                while *s != 0 {
                    /* Find the end of this entry */
                    t = s;
                    while *t as libc::c_int != 0 &&
                              *t as libc::c_int != ' ' as i32 &&
                              *t as libc::c_int != '|' as i32 {
                        /* loop */
                        t = t.offset(1)
                    }
                    /* Nuke and skip any dividers */
                    if *t != 0 {
                        let fresh12 = t;
                        t = t.offset(1);
                        *fresh12 = '\u{0}' as i32 as libc::c_char;
                        while *t as libc::c_int == ' ' as i32 ||
                                  *t as libc::c_int == '|' as i32 {
                            t = t.offset(1)
                        }
                    }
                    /* Parse this entry */
                    if 0 as libc::c_int !=
                           grab_one_race_kind_flag(&mut *(*rmp_ptr_0).oflags1.as_mut_ptr().offset(lev
                                                                                                      as
                                                                                                      isize),
                                                   &mut *(*rmp_ptr_0).oflags2.as_mut_ptr().offset(lev
                                                                                                      as
                                                                                                      isize),
                                                   &mut *(*rmp_ptr_0).oflags3.as_mut_ptr().offset(lev
                                                                                                      as
                                                                                                      isize),
                                                   &mut *(*rmp_ptr_0).oflags4.as_mut_ptr().offset(lev
                                                                                                      as
                                                                                                      isize),
                                                   &mut *(*rmp_ptr_0).oflags5.as_mut_ptr().offset(lev
                                                                                                      as
                                                                                                      isize),
                                                   &mut *(*rmp_ptr_0).oesp.as_mut_ptr().offset(lev
                                                                                                   as
                                                                                                   isize),
                                                   s as cptr) {
                        return 5 as libc::c_int
                    }
                    /* Start the next entry */
                    s = t
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'S' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'O' as i32 {
                let mut s_13: [libc::c_int; 5] = [0; 5];
                /* Process 'O' for "Object birth" */
                /* Scan for the values */
                if 5 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d:%d:%dd%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut *s_13.as_mut_ptr().offset(0 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_13.as_mut_ptr().offset(1 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_13.as_mut_ptr().offset(4 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_13.as_mut_ptr().offset(2 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_13.as_mut_ptr().offset(3 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int) {
                    s_13[4 as libc::c_int as usize] = 0 as libc::c_int;
                    if 4 as libc::c_int !=
                           sscanf(buf.offset(4 as libc::c_int as isize),
                                  b"%d:%d:%dd%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut *s_13.as_mut_ptr().offset(0 as
                                                                     libc::c_int
                                                                     as isize)
                                      as *mut libc::c_int,
                                  &mut *s_13.as_mut_ptr().offset(1 as
                                                                     libc::c_int
                                                                     as isize)
                                      as *mut libc::c_int,
                                  &mut *s_13.as_mut_ptr().offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                      as *mut libc::c_int,
                                  &mut *s_13.as_mut_ptr().offset(3 as
                                                                     libc::c_int
                                                                     as isize)
                                      as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                }
                (*rmp_ptr_0).obj_pval[(*rmp_ptr_0).obj_num as usize] =
                    s_13[4 as libc::c_int as usize] as s16b;
                (*rmp_ptr_0).obj_tval[(*rmp_ptr_0).obj_num as usize] =
                    s_13[0 as libc::c_int as usize] as s16b;
                (*rmp_ptr_0).obj_sval[(*rmp_ptr_0).obj_num as usize] =
                    s_13[1 as libc::c_int as usize] as s16b;
                (*rmp_ptr_0).obj_dd[(*rmp_ptr_0).obj_num as usize] =
                    s_13[2 as libc::c_int as usize] as s16b;
                let fresh13 = (*rmp_ptr_0).obj_num;
                (*rmp_ptr_0).obj_num = (*rmp_ptr_0).obj_num + 1;
                (*rmp_ptr_0).obj_ds[fresh13 as usize] =
                    s_13[3 as libc::c_int as usize] as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'S' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'A' as i32 {
                /* Process 'A' for "Allowed races" (multiple lines) */
                /* Parse every entry */
                s = buf.offset(4 as libc::c_int as isize);
                while *s != 0 {
                    /* Find the end of this entry */
                    t = s;
                    while *t as libc::c_int != 0 &&
                              *t as libc::c_int != ' ' as i32 &&
                              *t as libc::c_int != '|' as i32 {
                        /* loop */
                        t = t.offset(1)
                    }
                    /* Nuke and skip any dividers */
                    if *t != 0 {
                        let fresh14 = t;
                        t = t.offset(1);
                        *fresh14 = '\u{0}' as i32 as libc::c_char;
                        while *t as libc::c_int == ' ' as i32 ||
                                  *t as libc::c_int == '|' as i32 {
                            t = t.offset(1)
                        }
                    }
                    /* Parse this entry */
                    if 0 as libc::c_int !=
                           grab_one_race_allow_flag((*rmp_ptr_0).choice.as_mut_ptr(),
                                                    s as cptr) {
                        return 5 as libc::c_int
                    }
                    /* Start the next entry */
                    s = t
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'S' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'C' as i32 {
                let mut choice: [u32b; 2] =
                    [0 as libc::c_int as u32b, 0 as libc::c_int as u32b];
                let mut z_4: u32b = 0;
                /* Process 'C' for "Class choice flags" (multiple lines) */
                /* Parse every entry */
                s = buf.offset(6 as libc::c_int as isize);
                while *s != 0 {
                    /* Find the end of this entry */
                    t = s;
                    while *t as libc::c_int != 0 &&
                              *t as libc::c_int != ' ' as i32 &&
                              *t as libc::c_int != '|' as i32 {
                        /* loop */
                        t = t.offset(1)
                    }
                    /* Nuke and skip any dividers */
                    if *t != 0 {
                        let fresh15 = t;
                        t = t.offset(1);
                        *fresh15 = '\u{0}' as i32 as libc::c_char;
                        while *t as libc::c_int == ' ' as i32 ||
                                  *t as libc::c_int == '|' as i32 {
                            t = t.offset(1)
                        }
                    }
                    /* Parse this entry */
                    if 0 as libc::c_int !=
                           grab_one_class_flag(choice.as_mut_ptr(), s as cptr)
                       {
                        return 5 as libc::c_int
                    }
                    /* Start the next entry */
                    s = t
                }
                z_4 = 0 as libc::c_int as u32b;
                while z_4 < 2 as libc::c_int as libc::c_uint {
                    if *buf.offset(4 as libc::c_int as isize) as libc::c_int
                           == 'A' as i32 {
                        (*rmp_ptr_0).pclass[z_4 as usize] |=
                            choice[z_4 as usize]
                    } else {
                        (*rmp_ptr_0).mclass[z_4 as usize] |=
                            choice[z_4 as usize]
                    }
                    z_4 = z_4.wrapping_add(1)
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'C' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'N' as i32 {
                let mut z_5: libc::c_int = 0;
                /* Process 'N' for "New/Number/Name" */
                /* Find the colon before the name */
                s = strchr(buf.offset(4 as libc::c_int as isize), ':' as i32);
                /* Verify that colon */
                if s.is_null() { return 1 as libc::c_int }
                /* Nuke the colon, advance to the name */
                let fresh16 = s;
                s = s.offset(1);
                *fresh16 = '\u{0}' as i32 as libc::c_char;
                /* Paranoia -- require a name */
                if *s == 0 { return 1 as libc::c_int }
                /* Get the index */
                i = atoi(buf.offset(4 as libc::c_int as isize));
                /* Verify information */
                if i < error_idx as libc::c_int { return 4 as libc::c_int }
                /* Verify information */
                if i >= (*c_head).info_num as libc::c_int {
                    return 2 as libc::c_int
                }
                /* Save the index */
                error_idx = i as s16b;
                /* Point at the "info" */
                c_ptr =
                    &mut *class_info.offset(i as isize) as *mut player_class;
                /* Hack -- Verify space */
                if ((*c_head).name_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_name_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the name index */
                if (*c_ptr).title == 0 {
                    (*c_head).name_size = (*c_head).name_size.wrapping_add(1);
                    (*c_ptr).title = (*c_head).name_size as s32b
                }
                /* Append chars to the name */
                strcpy(c_name.offset((*c_head).name_size as isize), s);
                /* Advance the index */
                (*c_head).name_size =
                    ((*c_head).name_size as
                         libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                        u32b;
                (*c_ptr).powers[3 as libc::c_int as usize] =
                    -(1 as libc::c_int) as s16b;
                (*c_ptr).powers[2 as libc::c_int as usize] =
                    (*c_ptr).powers[3 as libc::c_int as usize];
                (*c_ptr).powers[1 as libc::c_int as usize] =
                    (*c_ptr).powers[2 as libc::c_int as usize];
                (*c_ptr).powers[0 as libc::c_int as usize] =
                    (*c_ptr).powers[1 as libc::c_int as usize];
                powers = 0 as libc::c_int;
                lev = 1 as libc::c_int;
                z_5 = 0 as libc::c_int;
                while z_5 < 10 as libc::c_int {
                    (*c_ptr).abilities[z_5 as usize].level =
                        -(1 as libc::c_int) as s16b;
                    z_5 += 1
                }
                cur_ab = 0 as libc::c_int;
                (*c_ptr).obj_num = 0 as libc::c_int as s16b;
                tit_idx = 0 as libc::c_int;
                spec_idx = -(1 as libc::c_int);
                z_5 = 0 as libc::c_int;
                while z_5 < 20 as libc::c_int {
                    (*c_ptr).spec[z_5 as usize].title = 0 as libc::c_int;
                    z_5 += 1
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'C' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'D' as i32 {
                /* Process 'D' for "Description" */
                /* Acquire the text */
                s = buf.offset(6 as libc::c_int as isize);
                /* Hack -- Verify space */
                if ((*c_head).text_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_text_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                match *buf.offset(4 as libc::c_int as isize) as libc::c_int {
                    48 => {
                        /* Advance and Save the text index */
                        if (*c_ptr).desc == 0 {
                            (*c_head).text_size =
                                (*c_head).text_size.wrapping_add(1);
                            (*c_ptr).desc = (*c_head).text_size as s32b;
                            /* Append chars to the name */
                            strcpy(c_text.offset((*c_head).text_size as
                                                     isize), s);
                            /* Advance the index */
                            (*c_head).text_size =
                                ((*c_head).text_size as
                                     libc::c_ulong).wrapping_add(strlen(s)) as
                                    u32b as u32b
                        } else {
                            /* Append chars to the name */
                            strcpy(c_text.offset((*c_head).text_size as
                                                     isize),
                                   format(b"\n%s\x00" as *const u8 as
                                              *const libc::c_char, s));
                            /* Advance the index */
                            (*c_head).text_size =
                                ((*c_head).text_size as
                                     libc::c_ulong).wrapping_add(strlen(s).wrapping_add(1
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_ulong))
                                    as u32b as u32b
                        }
                    }
                    49 => {
                        /* Advance and Save the text index */
                        (*c_head).text_size =
                            (*c_head).text_size.wrapping_add(1);
                        let fresh17 = tit_idx;
                        tit_idx = tit_idx + 1;
                        (*c_ptr).titles[fresh17 as usize] =
                            (*c_head).text_size as s32b;
                        /* Append chars to the name */
                        strcpy(c_text.offset((*c_head).text_size as isize),
                               s);
                        /* Advance the index */
                        (*c_head).text_size =
                            ((*c_head).text_size as
                                 libc::c_ulong).wrapping_add(strlen(s)) as
                                u32b as u32b
                    }
                    _ => { return 6 as libc::c_int }
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'C' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'O' as i32 {
                let mut s_14: [libc::c_int; 5] = [0; 5];
                /* Process 'O' for "Object birth" */
                /* Scan for the values */
                if 5 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d:%d:%dd%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut *s_14.as_mut_ptr().offset(0 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_14.as_mut_ptr().offset(1 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_14.as_mut_ptr().offset(4 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_14.as_mut_ptr().offset(2 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_14.as_mut_ptr().offset(3 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int) {
                    s_14[4 as libc::c_int as usize] = 0 as libc::c_int;
                    if 4 as libc::c_int !=
                           sscanf(buf.offset(4 as libc::c_int as isize),
                                  b"%d:%d:%dd%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut *s_14.as_mut_ptr().offset(0 as
                                                                     libc::c_int
                                                                     as isize)
                                      as *mut libc::c_int,
                                  &mut *s_14.as_mut_ptr().offset(1 as
                                                                     libc::c_int
                                                                     as isize)
                                      as *mut libc::c_int,
                                  &mut *s_14.as_mut_ptr().offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                      as *mut libc::c_int,
                                  &mut *s_14.as_mut_ptr().offset(3 as
                                                                     libc::c_int
                                                                     as isize)
                                      as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                }
                (*c_ptr).obj_pval[(*c_ptr).obj_num as usize] =
                    s_14[4 as libc::c_int as usize] as s16b;
                (*c_ptr).obj_tval[(*c_ptr).obj_num as usize] =
                    s_14[0 as libc::c_int as usize] as s16b;
                (*c_ptr).obj_sval[(*c_ptr).obj_num as usize] =
                    s_14[1 as libc::c_int as usize] as s16b;
                (*c_ptr).obj_dd[(*c_ptr).obj_num as usize] =
                    s_14[2 as libc::c_int as usize] as s16b;
                let fresh18 = (*c_ptr).obj_num;
                (*c_ptr).obj_num = (*c_ptr).obj_num + 1;
                (*c_ptr).obj_ds[fresh18 as usize] =
                    s_14[3 as libc::c_int as usize] as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'C' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'E' as i32 {
                let mut s_15: [libc::c_int; 6] = [0; 6];
                let mut z_6: libc::c_int = 0;
                /* Process 'E' for "body parts" */
                /* Scan for the values */
                if 6 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d:%d:%d:%d:%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut *s_15.as_mut_ptr().offset(0 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_15.as_mut_ptr().offset(1 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_15.as_mut_ptr().offset(2 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_15.as_mut_ptr().offset(3 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_15.as_mut_ptr().offset(4 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_15.as_mut_ptr().offset(5 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                z_6 = 0 as libc::c_int;
                while z_6 < 6 as libc::c_int {
                    (*c_ptr).body_parts[z_6 as usize] =
                        s_15[z_6 as usize] as libc::c_char;
                    z_6 += 1
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'C' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'R' as i32 {
                let mut s_16: [libc::c_int; 2] = [0; 2];
                /* Process 'R' for "flag level" */
                /* Scan for the values */
                if 2 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut *s_16.as_mut_ptr().offset(0 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_16.as_mut_ptr().offset(1 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                lev = s_16[0 as libc::c_int as usize];
                (*c_ptr).opval[lev as usize] =
                    s_16[1 as libc::c_int as usize] as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'C' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'S' as i32 {
                let mut s_17: [libc::c_int; 8] = [0; 8];
                let mut z_7: libc::c_int = 0;
                /* Process 'C' for "Stats" */
                /* Scan for the values */
                if 8 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d:%d:%d:%d:%d:%d:%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut *s_17.as_mut_ptr().offset(0 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_17.as_mut_ptr().offset(1 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_17.as_mut_ptr().offset(2 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_17.as_mut_ptr().offset(3 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_17.as_mut_ptr().offset(4 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_17.as_mut_ptr().offset(5 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_17.as_mut_ptr().offset(6 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_17.as_mut_ptr().offset(7 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                (*c_ptr).mana = s_17[6 as libc::c_int as usize] as s16b;
                (*c_ptr).extra_blows =
                    s_17[7 as libc::c_int as usize] as s16b;
                z_7 = 0 as libc::c_int;
                while z_7 < 6 as libc::c_int {
                    (*c_ptr).c_adj[z_7 as usize] = s_17[z_7 as usize] as s16b;
                    z_7 += 1
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'C' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'k' as i32 {
                let mut val_2: libc::c_long = 0;
                let mut mod_3: libc::c_long = 0;
                let mut i_5: libc::c_long = 0;
                let mut name_2: [libc::c_char; 200] = [0; 200];
                let mut v_2: libc::c_char = 0;
                let mut m_2: libc::c_char = 0;
                /* Process 'k' for "skills" */
                /* Scan for the values */
                if 5 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%c%ld:%c%ld:%s\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut v_2 as *mut libc::c_char,
                              &mut val_2 as *mut libc::c_long,
                              &mut m_2 as *mut libc::c_char,
                              &mut mod_3 as *mut libc::c_long,
                              name_2.as_mut_ptr()) {
                    return 1 as libc::c_int
                }
                i_5 = find_skill(name_2.as_mut_ptr() as cptr) as libc::c_long;
                if i_5 == -(1 as libc::c_int) as libc::c_long {
                    return 1 as libc::c_int
                }
                (*c_ptr).skill_basem[i_5 as usize] =
                    monster_ego_modify(v_2) as libc::c_char;
                (*c_ptr).skill_base[i_5 as usize] = val_2 as u32b;
                (*c_ptr).skill_modm[i_5 as usize] =
                    monster_ego_modify(m_2) as libc::c_char;
                (*c_ptr).skill_mod[i_5 as usize] = mod_3 as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'C' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'b' as i32 {
                let mut sec_1: *mut libc::c_char = 0 as *mut libc::c_char;
                /* Process 'b' for "abilities" */
                /* Scan for the values */
                sec_1 =
                    strchr(buf.offset(4 as libc::c_int as isize), ':' as i32);
                if sec_1.is_null() { return 1 as libc::c_int }
                *sec_1 = '\u{0}' as i32 as libc::c_char;
                sec_1 = sec_1.offset(1);
                if *sec_1 == 0 { return 1 as libc::c_int }
                i = find_ability(sec_1 as cptr) as libc::c_int;
                if i == -(1 as libc::c_int) { return 1 as libc::c_int }
                (*c_ptr).abilities[cur_ab as usize].ability = i as s16b;
                (*c_ptr).abilities[cur_ab as usize].level =
                    atoi(buf.offset(4 as libc::c_int as isize)) as s16b;
                cur_ab += 1
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'C' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'g' as i32 {
                let mut i_6: libc::c_int = 0;
                if streq(buf.offset(4 as libc::c_int as isize) as cptr,
                         b"All Gods\x00" as *const u8 as *const libc::c_char)
                       != 0 {
                    (*c_ptr).gods = 0xffffffff as libc::c_uint
                } else {
                    i_6 =
                        find_god(buf.offset(4 as libc::c_int as isize) as
                                     cptr);
                    if i_6 == -(1 as libc::c_int) { return 1 as libc::c_int }
                    (*c_ptr).gods =
                        ((*c_ptr).gods as libc::c_long |
                             (1 as libc::c_long) << i_6) as u32b
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'C' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'Z' as i32 {
                let mut i_7: libc::c_int = 0;
                /* Process 'g' for "gods" */
                /* Process 'Z' for "powers" */
                /* Acquire the text */
                s = buf.offset(4 as libc::c_int as isize);
                /* Find it in the list */
                i_7 = 0 as libc::c_int;
                while i_7 < power_max as libc::c_int {
                    if strcasecmp(s, (*powers_type.offset(i_7 as isize)).name)
                           == 0 {
                        break ;
                    }
                    i_7 += 1
                }
                if i_7 == power_max as libc::c_int { return 6 as libc::c_int }
                let fresh19 = powers;
                powers = powers + 1;
                (*c_ptr).powers[fresh19 as usize] = i_7 as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'C' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'K' as i32 {
                let mut s_18: [libc::c_int; 8] = [0; 8];
                /* Process 'K' for "sKills" */
                /* Scan for the values */
                if 8 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d:%d:%d:%d:%d:%d:%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut *s_18.as_mut_ptr().offset(0 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_18.as_mut_ptr().offset(1 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_18.as_mut_ptr().offset(2 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_18.as_mut_ptr().offset(3 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_18.as_mut_ptr().offset(4 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_18.as_mut_ptr().offset(5 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_18.as_mut_ptr().offset(6 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_18.as_mut_ptr().offset(7 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                (*c_ptr).c_dis = s_18[0 as libc::c_int as usize] as s16b;
                (*c_ptr).c_dev = s_18[1 as libc::c_int as usize] as s16b;
                (*c_ptr).c_sav = s_18[2 as libc::c_int as usize] as s16b;
                (*c_ptr).c_stl = s_18[3 as libc::c_int as usize] as s16b;
                (*c_ptr).c_srh = s_18[4 as libc::c_int as usize] as s16b;
                (*c_ptr).c_fos = s_18[5 as libc::c_int as usize] as s16b;
                (*c_ptr).c_thn = s_18[6 as libc::c_int as usize] as s16b;
                (*c_ptr).c_thb = s_18[7 as libc::c_int as usize] as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'C' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'X' as i32 {
                let mut s_19: [libc::c_int; 8] = [0; 8];
                /* Process 'x' for "Xtra skills" */
                /* Scan for the values */
                if 8 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d:%d:%d:%d:%d:%d:%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut *s_19.as_mut_ptr().offset(0 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_19.as_mut_ptr().offset(1 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_19.as_mut_ptr().offset(2 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_19.as_mut_ptr().offset(3 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_19.as_mut_ptr().offset(4 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_19.as_mut_ptr().offset(5 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_19.as_mut_ptr().offset(6 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_19.as_mut_ptr().offset(7 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                (*c_ptr).x_dis = s_19[0 as libc::c_int as usize] as s16b;
                (*c_ptr).x_dev = s_19[1 as libc::c_int as usize] as s16b;
                (*c_ptr).x_sav = s_19[2 as libc::c_int as usize] as s16b;
                (*c_ptr).x_stl = s_19[3 as libc::c_int as usize] as s16b;
                (*c_ptr).x_srh = s_19[4 as libc::c_int as usize] as s16b;
                (*c_ptr).x_fos = s_19[5 as libc::c_int as usize] as s16b;
                (*c_ptr).x_thn = s_19[6 as libc::c_int as usize] as s16b;
                (*c_ptr).x_thb = s_19[7 as libc::c_int as usize] as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'C' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'P' as i32 {
                let mut s_20: [libc::c_int; 2] = [0; 2];
                /* Process 'P' for "xtra" */
                /* Scan for the values */
                if 2 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut *s_20.as_mut_ptr().offset(0 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_20.as_mut_ptr().offset(1 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                (*c_ptr).c_mhp = s_20[0 as libc::c_int as usize] as s16b;
                (*c_ptr).c_exp = s_20[1 as libc::c_int as usize] as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'C' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'C' as i32 {
                let mut s_21: [libc::c_long; 3] = [0; 3];
                let mut h: libc::c_char = 0;
                let mut m_3: libc::c_char = 0;
                /* Process 'C' for "sensing" */
                /* Scan for the values */
                if 5 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%c:%c:%ld:%ld:%ld\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut h as *mut libc::c_char,
                              &mut m_3 as *mut libc::c_char,
                              &mut *s_21.as_mut_ptr().offset(0 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_long,
                              &mut *s_21.as_mut_ptr().offset(1 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_long,
                              &mut *s_21.as_mut_ptr().offset(2 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_long) {
                    return 1 as libc::c_int
                }
                (*c_ptr).sense_heavy =
                    if h as libc::c_int == 'H' as i32 {
                        1 as libc::c_int
                    } else { 0 as libc::c_int } as byte_hack;
                (*c_ptr).sense_heavy_magic =
                    if m_3 as libc::c_int == 'H' as i32 {
                        1 as libc::c_int
                    } else { 0 as libc::c_int } as byte_hack;
                (*c_ptr).sense_base = s_21[0 as libc::c_int as usize] as s32b;
                (*c_ptr).sense_pl = s_21[1 as libc::c_int as usize] as s32b;
                (*c_ptr).sense_plus = s_21[2 as libc::c_int as usize] as s32b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'C' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'B' as i32 {
                let mut s_22: [libc::c_int; 3] = [0; 3];
                /* Process 'B' for "blows" */
                /* Scan for the values */
                if 3 as libc::c_int !=
                       sscanf(buf.offset(4 as libc::c_int as isize),
                              b"%d:%d:%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut *s_22.as_mut_ptr().offset(0 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_22.as_mut_ptr().offset(1 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int,
                              &mut *s_22.as_mut_ptr().offset(2 as libc::c_int
                                                                 as isize) as
                                  *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                (*c_ptr).blow_num = s_22[0 as libc::c_int as usize] as s16b;
                (*c_ptr).blow_wgt = s_22[1 as libc::c_int as usize] as s16b;
                (*c_ptr).blow_mul = s_22[2 as libc::c_int as usize] as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'C' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'G' as i32 {
                /* Process 'G' for "Player flags" (multiple lines) */
                /* Parse every entry */
                s = buf.offset(4 as libc::c_int as isize);
                while *s != 0 {
                    /* Find the end of this entry */
                    t = s;
                    while *t as libc::c_int != 0 &&
                              *t as libc::c_int != ' ' as i32 &&
                              *t as libc::c_int != '|' as i32 {
                        /* loop */
                        t = t.offset(1)
                    }
                    /* Nuke and skip any dividers */
                    if *t != 0 {
                        let fresh20 = t;
                        t = t.offset(1);
                        *fresh20 = '\u{0}' as i32 as libc::c_char;
                        while *t as libc::c_int == ' ' as i32 ||
                                  *t as libc::c_int == '|' as i32 {
                            t = t.offset(1)
                        }
                    }
                    /* Parse this entry */
                    if 0 as libc::c_int !=
                           grab_one_player_race_flag(&mut (*c_ptr).flags1,
                                                     &mut (*c_ptr).flags2,
                                                     s as cptr) {
                        return 5 as libc::c_int
                    }
                    /* Start the next entry */
                    s = t
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'C' as i32 &&
                          *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int == 'F' as i32 {
                /* Process 'F' for "level Flags" (multiple lines) */
                /* Parse every entry */
                s = buf.offset(4 as libc::c_int as isize);
                while *s != 0 {
                    /* Find the end of this entry */
                    t = s;
                    while *t as libc::c_int != 0 &&
                              *t as libc::c_int != ' ' as i32 &&
                              *t as libc::c_int != '|' as i32 {
                        /* loop */
                        t = t.offset(1)
                    }
                    /* Nuke and skip any dividers */
                    if *t != 0 {
                        let fresh21 = t;
                        t = t.offset(1);
                        *fresh21 = '\u{0}' as i32 as libc::c_char;
                        while *t as libc::c_int == ' ' as i32 ||
                                  *t as libc::c_int == '|' as i32 {
                            t = t.offset(1)
                        }
                    }
                    /* Parse this entry */
                    if 0 as libc::c_int !=
                           grab_one_race_kind_flag(&mut *(*c_ptr).oflags1.as_mut_ptr().offset(lev
                                                                                                  as
                                                                                                  isize),
                                                   &mut *(*c_ptr).oflags2.as_mut_ptr().offset(lev
                                                                                                  as
                                                                                                  isize),
                                                   &mut *(*c_ptr).oflags3.as_mut_ptr().offset(lev
                                                                                                  as
                                                                                                  isize),
                                                   &mut *(*c_ptr).oflags4.as_mut_ptr().offset(lev
                                                                                                  as
                                                                                                  isize),
                                                   &mut *(*c_ptr).oflags5.as_mut_ptr().offset(lev
                                                                                                  as
                                                                                                  isize),
                                                   &mut *(*c_ptr).oesp.as_mut_ptr().offset(lev
                                                                                               as
                                                                                               isize),
                                                   s as cptr) {
                        return 5 as libc::c_int
                    }
                    /* Start the next entry */
                    s = t
                }
            } else {
                /* Specialities  */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'C' as i32 &&
                       *buf.offset(2 as libc::c_int as isize) as libc::c_int
                           == 'a' as i32 {
                    /* Process 'N' for "New/Number/Name" */
                    if *buf.offset(4 as libc::c_int as isize) as libc::c_int
                           == 'N' as i32 {
                        /* Find the colon before the name */
                        s = buf.offset(6 as libc::c_int as isize);
                        /* Paranoia -- require a name */
                        if *s == 0 { return 1 as libc::c_int }
                        /* Get the index */
                        spec_idx += 1;
                        /* Verify information */
                        if spec_idx >= 20 as libc::c_int {
                            return 2 as libc::c_int
                        }
                        /* Point at the "info" */
                        s_ptr =
                            &mut *(*c_ptr).spec.as_mut_ptr().offset(spec_idx
                                                                        as
                                                                        isize)
                                as *mut player_spec;
                        /* Hack -- Verify space */
                        if ((*c_head).name_size as
                                libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong)
                               > fake_name_size as libc::c_ulong {
                            return 7 as libc::c_int
                        }
                        /* Advance and Save the name index */
                        if (*s_ptr).title == 0 {
                            (*c_head).name_size =
                                (*c_head).name_size.wrapping_add(1);
                            (*s_ptr).title = (*c_head).name_size as s32b
                        }
                        /* Append chars to the name */
                        strcpy(c_name.offset((*c_head).name_size as isize),
                               s);
                        /* Advance the index */
                        (*c_head).name_size =
                            ((*c_head).name_size as
                                 libc::c_ulong).wrapping_add(strlen(s)) as
                                u32b as u32b;
                        (*s_ptr).obj_num = 0 as libc::c_int as s16b;
                        cur_ab = 0 as libc::c_int;
                        z = 0 as libc::c_int;
                        while z < 10 as libc::c_int {
                            (*s_ptr).abilities[z as usize].level =
                                -(1 as libc::c_int) as s16b;
                            z += 1
                        }
                        /* Next... */
                        continue ;
                    } else if *buf.offset(4 as libc::c_int as isize) as
                                  libc::c_int == 'D' as i32 {
                        /* Process 'D' for "Description" */
                        /* Acquire the text */
                        s = buf.offset(6 as libc::c_int as isize);
                        /* Hack -- Verify space */
                        if ((*c_head).text_size as
                                libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong)
                               > fake_text_size as libc::c_ulong {
                            return 7 as libc::c_int
                        }
                        /* Advance and Save the text index */
                        if (*s_ptr).desc == 0 {
                            (*c_head).text_size =
                                (*c_head).text_size.wrapping_add(1);
                            (*s_ptr).desc = (*c_head).text_size as s32b;
                            /* Append chars to the name */
                            strcpy(c_text.offset((*c_head).text_size as
                                                     isize), s);
                            /* Advance the index */
                            (*c_head).text_size =
                                ((*c_head).text_size as
                                     libc::c_ulong).wrapping_add(strlen(s)) as
                                    u32b as u32b
                        } else {
                            /* Append chars to the name */
                            strcpy(c_text.offset((*c_head).text_size as
                                                     isize),
                                   format(b"\n%s\x00" as *const u8 as
                                              *const libc::c_char, s));
                            /* Advance the index */
                            (*c_head).text_size =
                                ((*c_head).text_size as
                                     libc::c_ulong).wrapping_add(strlen(s).wrapping_add(1
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_ulong))
                                    as u32b as u32b
                        }
                        /* Next... */
                        continue ;
                    } else if *buf.offset(4 as libc::c_int as isize) as
                                  libc::c_int == 'O' as i32 {
                        let mut s_23: [libc::c_int; 5] = [0; 5];
                        /* Process 'O' for "Object birth" */
                        /* Scan for the values */
                        if 5 as libc::c_int !=
                               sscanf(buf.offset(6 as libc::c_int as isize),
                                      b"%d:%d:%d:%dd%d\x00" as *const u8 as
                                          *const libc::c_char,
                                      &mut *s_23.as_mut_ptr().offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                          as *mut libc::c_int,
                                      &mut *s_23.as_mut_ptr().offset(1 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                          as *mut libc::c_int,
                                      &mut *s_23.as_mut_ptr().offset(4 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                          as *mut libc::c_int,
                                      &mut *s_23.as_mut_ptr().offset(2 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                          as *mut libc::c_int,
                                      &mut *s_23.as_mut_ptr().offset(3 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                          as *mut libc::c_int) {
                            s_23[4 as libc::c_int as usize] =
                                0 as libc::c_int;
                            if 4 as libc::c_int !=
                                   sscanf(buf.offset(6 as libc::c_int as
                                                         isize),
                                          b"%d:%d:%dd%d\x00" as *const u8 as
                                              *const libc::c_char,
                                          &mut *s_23.as_mut_ptr().offset(0 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                                              as *mut libc::c_int,
                                          &mut *s_23.as_mut_ptr().offset(1 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                                              as *mut libc::c_int,
                                          &mut *s_23.as_mut_ptr().offset(2 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                                              as *mut libc::c_int,
                                          &mut *s_23.as_mut_ptr().offset(3 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                                              as *mut libc::c_int) {
                                return 1 as libc::c_int
                            }
                        }
                        (*s_ptr).obj_pval[(*s_ptr).obj_num as usize] =
                            s_23[4 as libc::c_int as usize] as s16b;
                        (*s_ptr).obj_tval[(*s_ptr).obj_num as usize] =
                            s_23[0 as libc::c_int as usize] as s16b;
                        (*s_ptr).obj_sval[(*s_ptr).obj_num as usize] =
                            s_23[1 as libc::c_int as usize] as s16b;
                        (*s_ptr).obj_dd[(*s_ptr).obj_num as usize] =
                            s_23[2 as libc::c_int as usize] as s16b;
                        let fresh22 = (*s_ptr).obj_num;
                        (*s_ptr).obj_num = (*s_ptr).obj_num + 1;
                        (*s_ptr).obj_ds[fresh22 as usize] =
                            s_23[3 as libc::c_int as usize] as s16b;
                        /* Next... */
                        continue ;
                    } else if *buf.offset(4 as libc::c_int as isize) as
                                  libc::c_int == 'g' as i32 {
                        let mut i_8: libc::c_int = 0;
                        if streq(buf.offset(6 as libc::c_int as isize) as
                                     cptr,
                                 b"All Gods\x00" as *const u8 as
                                     *const libc::c_char) != 0 {
                            (*s_ptr).gods = 0xffffffff as libc::c_uint
                        } else {
                            i_8 =
                                find_god(buf.offset(6 as libc::c_int as isize)
                                             as cptr);
                            if i_8 == -(1 as libc::c_int) {
                                return 1 as libc::c_int
                            }
                            (*s_ptr).gods =
                                ((*s_ptr).gods as libc::c_long |
                                     (1 as libc::c_long) << i_8) as u32b
                        }
                        /* Process 'g' for "gods" */
                        /* Next... */
                        continue ;
                    } else if *buf.offset(4 as libc::c_int as isize) as
                                  libc::c_int == 'k' as i32 {
                        let mut val_3: libc::c_long = 0;
                        let mut mod_4: libc::c_long = 0;
                        let mut i_9: libc::c_long = 0;
                        let mut name_3: [libc::c_char; 200] = [0; 200];
                        let mut v_3: libc::c_char = 0;
                        let mut m_4: libc::c_char = 0;
                        /* Process 'k' for "skills" */
                        /* Scan for the values */
                        if 5 as libc::c_int !=
                               sscanf(buf.offset(6 as libc::c_int as isize),
                                      b"%c%ld:%c%ld:%s\x00" as *const u8 as
                                          *const libc::c_char,
                                      &mut v_3 as *mut libc::c_char,
                                      &mut val_3 as *mut libc::c_long,
                                      &mut m_4 as *mut libc::c_char,
                                      &mut mod_4 as *mut libc::c_long,
                                      name_3.as_mut_ptr()) {
                            return 1 as libc::c_int
                        }
                        i_9 =
                            find_skill(name_3.as_mut_ptr() as cptr) as
                                libc::c_long;
                        if i_9 == -(1 as libc::c_int) as libc::c_long {
                            return 1 as libc::c_int
                        }
                        (*s_ptr).skill_basem[i_9 as usize] =
                            monster_ego_modify(v_3) as libc::c_char;
                        (*s_ptr).skill_base[i_9 as usize] = val_3 as u32b;
                        (*s_ptr).skill_modm[i_9 as usize] =
                            monster_ego_modify(m_4) as libc::c_char;
                        (*s_ptr).skill_mod[i_9 as usize] = mod_4 as s16b;
                        /* Next... */
                        continue ;
                    } else if *buf.offset(4 as libc::c_int as isize) as
                                  libc::c_int == 'b' as i32 {
                        let mut sec_2: *mut libc::c_char =
                            0 as *mut libc::c_char;
                        /* Process 'b' for "abilities" */
                        /* Scan for the values */
                        sec_2 =
                            strchr(buf.offset(6 as libc::c_int as isize),
                                   ':' as i32);
                        if sec_2.is_null() { return 1 as libc::c_int }
                        *sec_2 = '\u{0}' as i32 as libc::c_char;
                        sec_2 = sec_2.offset(1);
                        if *sec_2 == 0 { return 1 as libc::c_int }
                        i = find_ability(sec_2 as cptr) as libc::c_int;
                        if i == -(1 as libc::c_int) {
                            return 1 as libc::c_int
                        }
                        (*s_ptr).abilities[cur_ab as usize].ability =
                            i as s16b;
                        (*s_ptr).abilities[cur_ab as usize].level =
                            atoi(buf.offset(6 as libc::c_int as isize)) as
                                s16b;
                        cur_ab += 1;
                        /* Next... */
                        continue ;
                    } else if *buf.offset(4 as libc::c_int as isize) as
                                  libc::c_int == 'G' as i32 {
                        /* Process 'G' for "Player flags" (multiple lines) */
                        /* Parse every entry */
                        s = buf.offset(6 as libc::c_int as isize);
                        while *s != 0 {
                            /* Find the end of this entry */
                            t = s;
                            while *t as libc::c_int != 0 &&
                                      *t as libc::c_int != ' ' as i32 &&
                                      *t as libc::c_int != '|' as i32 {
                                /* loop */
                                t = t.offset(1)
                            }
                            /* Nuke and skip any dividers */
                            if *t != 0 {
                                let fresh23 = t;
                                t = t.offset(1);
                                *fresh23 = '\u{0}' as i32 as libc::c_char;
                                while *t as libc::c_int == ' ' as i32 ||
                                          *t as libc::c_int == '|' as i32 {
                                    t = t.offset(1)
                                }
                            }
                            /* Parse this entry */
                            if 0 as libc::c_int !=
                                   grab_one_player_race_flag(&mut (*s_ptr).flags1,
                                                             &mut (*s_ptr).flags2,
                                                             s as cptr) {
                                return 5 as libc::c_int
                            }
                            /* Start the next entry */
                            s = t
                        }
                        /* Next... */
                        continue ;
                    } else if *buf.offset(4 as libc::c_int as isize) as
                                  libc::c_int == 'K' as i32 {
                        let mut val_4: libc::c_long = 0;
                        let mut name_4: [libc::c_char; 200] = [0; 200];
                        /* Process 'K' for "desired skills" */
                        /* Scan for the values */
                        if 2 as libc::c_int !=
                               sscanf(buf.offset(6 as libc::c_int as isize),
                                      b"%ld:%s\x00" as *const u8 as
                                          *const libc::c_char,
                                      &mut val_4 as *mut libc::c_long,
                                      name_4.as_mut_ptr()) {
                            return 1 as libc::c_int
                        }
                        i =
                            find_skill(name_4.as_mut_ptr() as cptr) as
                                libc::c_int;
                        if i == -(1 as libc::c_int) {
                            return 1 as libc::c_int
                        }
                        (*s_ptr).skill_ideal[i as usize] = val_4 as u32b;
                        /* Next... */
                        continue ;
                    }
                }
                /* Process 'N' for "New/Number/Name" */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'M' as i32 &&
                       *buf.offset(2 as libc::c_int as isize) as libc::c_int
                           == 'N' as i32 {
                    /* Find the colon before the name */
                    s =
                        strchr(buf.offset(4 as libc::c_int as isize),
                               ':' as i32);
                    /* Verify that colon */
                    if s.is_null() { return 1 as libc::c_int }
                    /* Nuke the colon, advance to the name */
                    let fresh24 = s;
                    s = s.offset(1);
                    *fresh24 = '\u{0}' as i32 as libc::c_char;
                    /* Paranoia -- require a name */
                    if *s == 0 { return 1 as libc::c_int }
                    /* Get the index */
                    i = atoi(buf.offset(4 as libc::c_int as isize));
                    /* Verify information */
                    if i < error_idx as libc::c_int {
                        return 4 as libc::c_int
                    }
                    /* Verify information */
                    if i >= max_mc_idx as libc::c_int {
                        return 2 as libc::c_int
                    }
                    /* Save the index */
                    error_idx = i as s16b;
                    /* Point at the "info" */
                    mc_ptr =
                        &mut *meta_class_info.offset(i as isize) as
                            *mut meta_class_type;
                    /* Append chars to the name */
                    strcpy((*mc_ptr).name.as_mut_ptr(),
                           s.offset(2 as libc::c_int as isize));
                    (*mc_ptr).color =
                        color_char_to_attr(*s.offset(0 as libc::c_int as
                                                         isize)) as byte_hack;
                    powers = 0 as libc::c_int;
                    while powers < max_c_idx as libc::c_int {
                        *(*mc_ptr).classes.offset(powers as isize) =
                            -(1 as libc::c_int) as s16b;
                        powers += 1
                    }
                    powers = 0 as libc::c_int
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'M' as i32 &&
                              *buf.offset(2 as libc::c_int as isize) as
                                  libc::c_int == 'C' as i32 {
                    let mut i_10: libc::c_int = 0;
                    /* Process 'C' for "Classes" */
                    /* Acquire the text */
                    s = buf.offset(4 as libc::c_int as isize);
                    /* Find it in the list */
                    i_10 = 0 as libc::c_int;
                    while i_10 < max_c_idx as libc::c_int {
                        if strcasecmp(s,
                                      c_name.offset((*class_info.offset(i_10
                                                                            as
                                                                            isize)).title
                                                        as isize)) == 0 {
                            break ;
                        }
                        i_10 += 1
                    }
                    if i_10 == max_c_idx as libc::c_int {
                        return 6 as libc::c_int
                    }
                    let fresh25 = powers;
                    powers = powers + 1;
                    *(*mc_ptr).classes.offset(fresh25 as isize) = i_10 as s16b
                } else {
                    /* Oops */
                    return 6 as libc::c_int
                }
            }
        }
    }
    /* Complete the "name" and "text" sizes */
    (*rp_head).name_size = (*rp_head).name_size.wrapping_add(1);
    (*rp_head).text_size = (*rp_head).text_size.wrapping_add(1);
    (*rmp_head).name_size = (*rmp_head).name_size.wrapping_add(1);
    (*rmp_head).text_size = (*rmp_head).text_size.wrapping_add(1);
    (*c_head).name_size = (*c_head).name_size.wrapping_add(1);
    (*c_head).text_size = (*c_head).text_size.wrapping_add(1);
    /* No version yet */
    if okay == 0 { return 2 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialize the "v_info" array, by parsing an ascii "template" file
 */
#[no_mangle]
pub unsafe extern "C" fn init_v_info_txt(mut fp: *mut FILE,
                                         mut buf: *mut libc::c_char,
                                         mut start: bool_) -> errr {
    let mut i: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Not ready yet */
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    /* Current entry */
    let mut v_ptr: *mut vault_type = 0 as *mut vault_type;
    if start != 0 {
        /* Just before the first record */
        error_idx = -(1 as libc::c_int) as s16b;
        /* Just before the first line */
        error_line = -(1 as libc::c_int) as s16b;
        /* Prepare the "fake" stuff */
        (*v_head).name_size = 0 as libc::c_int as u32b;
        (*v_head).text_size = 0 as libc::c_int as u32b
    }
    /* Parse */
    fp_stack_init(fp);
    /* Next... */
    while 0 as libc::c_int == my_fgets_dostack(buf, 1024 as libc::c_int) {
        /* Advance the line number */
        error_line += 1;
        /* Skip comments and blank lines */
        if *buf.offset(0 as libc::c_int as isize) == 0 ||
               *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '#' as i32 {
            continue ;
        }
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'Q' as i32
               ||
               *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   'T' as i32 {
            continue ;
        }
        /* Verify correct "colon" format */
        if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
           {
            return 1 as libc::c_int
        }
        /* Hack -- Process 'V' for "Version" */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'V' as i32
           {
            let mut v1: libc::c_int = 0;
            let mut v2: libc::c_int = 0;
            let mut v3: libc::c_int = 0;
            /* Scan for the values */
            if 3 as libc::c_int !=
                   sscanf(buf,
                          b"V:%d.%d.%d\x00" as *const u8 as
                              *const libc::c_char,
                          &mut v1 as *mut libc::c_int,
                          &mut v2 as *mut libc::c_int,
                          &mut v3 as *mut libc::c_int) {
                return 2 as libc::c_int
            }
            /* Okay to proceed */
            okay = 1 as libc::c_int as bool_
        } else {
            /* No version yet */
            if okay == 0 { return 2 as libc::c_int }
            /* Included file */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '<' as i32 {
                fp_stack_push(buf.offset(2 as libc::c_int as isize) as cptr);
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'N' as i32 {
                /* Process 'N' for "New/Number/Name" */
                /* Find the colon before the name */
                s = strchr(buf.offset(2 as libc::c_int as isize), ':' as i32);
                /* Verify that colon */
                if s.is_null() { return 1 as libc::c_int }
                /* Nuke the colon, advance to the name */
                let fresh26 = s;
                s = s.offset(1);
                *fresh26 = '\u{0}' as i32 as libc::c_char;
                /* Paranoia -- require a name */
                if *s == 0 { return 1 as libc::c_int }
                /* Get the index */
                i = atoi(buf.offset(2 as libc::c_int as isize));
                /* Verify information */
                if i <= error_idx as libc::c_int { return 4 as libc::c_int }
                /* Verify information */
                if i >= (*v_head).info_num as libc::c_int {
                    return 2 as libc::c_int
                }
                /* Save the index */
                error_idx = i as s16b;
                /* Point at the "info" */
                v_ptr = &mut *v_info.offset(i as isize) as *mut vault_type;
                /* Hack -- Verify space */
                if ((*v_head).name_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_name_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the name index */
                if (*v_ptr).name == 0 {
                    (*v_head).name_size = (*v_head).name_size.wrapping_add(1);
                    (*v_ptr).name = (*v_head).name_size
                }
                /* Append chars to the name */
                strcpy(v_name.offset((*v_head).name_size as isize), s);
                /* Advance the index */
                (*v_head).name_size =
                    ((*v_head).name_size as
                         libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                        u32b
            } else {
                /* There better be a current v_ptr */
                if v_ptr.is_null() { return 3 as libc::c_int }
                /* Process 'D' for "Description" */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'D' as i32 {
                    /* Acquire the text */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Hack -- Verify space */
                    if ((*v_head).text_size as
                            libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                           > fake_text_size as libc::c_ulong {
                        return 7 as libc::c_int
                    }
                    /* Advance and Save the text index */
                    if (*v_ptr).text == 0 {
                        (*v_head).text_size =
                            (*v_head).text_size.wrapping_add(1);
                        (*v_ptr).text = (*v_head).text_size
                    }
                    /* Append chars to the name */
                    strcpy(v_text.offset((*v_head).text_size as isize), s);
                    /* Advance the index */
                    (*v_head).text_size =
                        ((*v_head).text_size as
                             libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                            u32b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'X' as i32 {
                    let mut typ: libc::c_int = 0;
                    let mut rat: libc::c_int = 0;
                    let mut hgt: libc::c_int = 0;
                    let mut wid: libc::c_int = 0;
                    /* Process 'X' for "Extra info" (one line only) */
                    /* Scan for the values */
                    if 4 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut typ as *mut libc::c_int,
                                  &mut rat as *mut libc::c_int,
                                  &mut hgt as *mut libc::c_int,
                                  &mut wid as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*v_ptr).typ = typ as byte_hack;
                    (*v_ptr).rat = rat as byte_hack;
                    (*v_ptr).hgt = hgt as byte_hack;
                    (*v_ptr).wid = wid as byte_hack
                } else {
                    /* There better be a current v_ptr */
                    if v_ptr.is_null() { return 3 as libc::c_int }
                    /* Process monster, item and level info for special levels */
                    if *buf.offset(0 as libc::c_int as isize) as libc::c_int
                           == 'Y' as i32 {
                        let mut mon1: libc::c_int = 0;
                        let mut mon2: libc::c_int = 0;
                        let mut mon3: libc::c_int = 0;
                        let mut mon4: libc::c_int = 0;
                        let mut mon5: libc::c_int = 0;
                        let mut mon6: libc::c_int = 0;
                        let mut mon7: libc::c_int = 0;
                        let mut mon8: libc::c_int = 0;
                        let mut mon9: libc::c_int = 0;
                        let mut mon10: libc::c_int = 0;
                        let mut item1: libc::c_int = 0;
                        let mut item2: libc::c_int = 0;
                        let mut item3: libc::c_int = 0;
                        let mut lvl: libc::c_int = 0;
                        let mut dun_type: libc::c_int = 0;
                        /* Scan for the values */
                        if 15 as libc::c_int !=
                               sscanf(buf.offset(2 as libc::c_int as isize),
                                      b"%d:%d:%d:%d:%d:%d:%d:%d:%d:%d:%d:%d:%d:%d:%d\x00"
                                          as *const u8 as *const libc::c_char,
                                      &mut mon1 as *mut libc::c_int,
                                      &mut mon2 as *mut libc::c_int,
                                      &mut mon3 as *mut libc::c_int,
                                      &mut mon4 as *mut libc::c_int,
                                      &mut mon5 as *mut libc::c_int,
                                      &mut mon6 as *mut libc::c_int,
                                      &mut mon7 as *mut libc::c_int,
                                      &mut mon8 as *mut libc::c_int,
                                      &mut mon9 as *mut libc::c_int,
                                      &mut mon10 as *mut libc::c_int,
                                      &mut item1 as *mut libc::c_int,
                                      &mut item2 as *mut libc::c_int,
                                      &mut item3 as *mut libc::c_int,
                                      &mut lvl as *mut libc::c_int,
                                      &mut dun_type as *mut libc::c_int) {
                            return 1 as libc::c_int
                        }
                        /* Save the values */
                        (*v_ptr).mon[0 as libc::c_int as usize] =
                            mon1 as s16b;
                        (*v_ptr).mon[1 as libc::c_int as usize] =
                            mon2 as s16b;
                        (*v_ptr).mon[2 as libc::c_int as usize] =
                            mon3 as s16b;
                        (*v_ptr).mon[3 as libc::c_int as usize] =
                            mon4 as s16b;
                        (*v_ptr).mon[4 as libc::c_int as usize] =
                            mon5 as s16b;
                        (*v_ptr).mon[5 as libc::c_int as usize] =
                            mon6 as s16b;
                        (*v_ptr).mon[6 as libc::c_int as usize] =
                            mon7 as s16b;
                        (*v_ptr).mon[7 as libc::c_int as usize] =
                            mon8 as s16b;
                        (*v_ptr).mon[8 as libc::c_int as usize] =
                            mon9 as s16b;
                        (*v_ptr).mon[9 as libc::c_int as usize] =
                            mon10 as s16b;
                        (*v_ptr).item[0 as libc::c_int as usize] = item1;
                        (*v_ptr).item[1 as libc::c_int as usize] = item2;
                        (*v_ptr).item[2 as libc::c_int as usize] = item3;
                        (*v_ptr).lvl = lvl as s16b;
                        (*v_ptr).dun_type = dun_type as byte_hack
                    } else {
                        /* Oops */
                        return 6 as libc::c_int
                    }
                }
            }
        }
    }
    /* Complete the "name" and "text" sizes */
    if start == 0 {
        (*v_head).name_size = (*v_head).name_size.wrapping_add(1);
        (*v_head).text_size = (*v_head).text_size.wrapping_add(1)
    }
    /* No version yet */
    if okay == 0 { return 2 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Grab one flag in an feature_type from a textual string
 */
unsafe extern "C" fn grab_one_feature_flag(mut f_ptr: *mut feature_type,
                                           mut what: cptr) -> errr {
    let mut i: libc::c_int = 0;
    /* Check flags1 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, f_info_flags1[i as usize]) != 0 {
            (*f_ptr).flags1 =
                ((*f_ptr).flags1 as libc::c_long | (1 as libc::c_long) << i)
                    as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Oops */
    msg_format(b"Unknown object flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Error */
    return 1 as libc::c_int;
}
/*
 * Initialize the "f_info" array, by parsing an ascii "template" file
 */
#[no_mangle]
pub unsafe extern "C" fn init_f_info_txt(mut fp: *mut FILE,
                                         mut buf: *mut libc::c_char) -> errr {
    let mut i: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Not ready yet */
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    let mut default_desc: u32b = 0 as libc::c_int as u32b;
    let mut default_tunnel: u32b = 0 as libc::c_int as u32b;
    let mut default_block: u32b = 0 as libc::c_int as u32b;
    /* Current entry */
    let mut f_ptr: *mut feature_type = 0 as *mut feature_type;
    /* Just before the first record */
    error_idx = -(1 as libc::c_int) as s16b;
    /* Just before the first line */
    error_line = -(1 as libc::c_int) as s16b;
    /* Prepare the "fake" stuff */
    (*f_head).name_size = 0 as libc::c_int as u32b;
    (*f_head).text_size = 0 as libc::c_int as u32b;
    /* Add some fake descs */
    (*f_head).text_size = (*f_head).text_size.wrapping_add(1);
    default_desc = (*f_head).text_size;
    strcpy(f_text.offset((*f_head).text_size as isize),
           b"a wall blocking your way\x00" as *const u8 as
               *const libc::c_char);
    (*f_head).text_size =
        ((*f_head).text_size as
             libc::c_ulong).wrapping_add(strlen(b"a wall blocking your way\x00"
                                                    as *const u8 as
                                                    *const libc::c_char)) as
            u32b as u32b;
    (*f_head).text_size = (*f_head).text_size.wrapping_add(1);
    default_tunnel = (*f_head).text_size;
    strcpy(f_text.offset((*f_head).text_size as isize),
           b"You cannot tunnel through that.\x00" as *const u8 as
               *const libc::c_char);
    (*f_head).text_size =
        ((*f_head).text_size as
             libc::c_ulong).wrapping_add(strlen(b"You cannot tunnel through that.\x00"
                                                    as *const u8 as
                                                    *const libc::c_char)) as
            u32b as u32b;
    (*f_head).text_size = (*f_head).text_size.wrapping_add(1);
    default_block = (*f_head).text_size;
    strcpy(f_text.offset((*f_head).text_size as isize),
           b"a wall blocking your way\x00" as *const u8 as
               *const libc::c_char);
    (*f_head).text_size =
        ((*f_head).text_size as
             libc::c_ulong).wrapping_add(strlen(b"a wall blocking your way\x00"
                                                    as *const u8 as
                                                    *const libc::c_char)) as
            u32b as u32b;
    /* Parse */
    fp_stack_init(fp);
    /* Next... */
    while 0 as libc::c_int == my_fgets_dostack(buf, 1024 as libc::c_int) {
        /* Advance the line number */
        error_line += 1;
        /* Skip comments and blank lines */
        if *buf.offset(0 as libc::c_int as isize) == 0 ||
               *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '#' as i32 {
            continue ;
        }
        /* Verify correct "colon" format */
        if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
           {
            return 1 as libc::c_int
        }
        /* Hack -- Process 'V' for "Version" */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'V' as i32
           {
            let mut v1: libc::c_int = 0;
            let mut v2: libc::c_int = 0;
            let mut v3: libc::c_int = 0;
            /* Scan for the values */
            if 3 as libc::c_int !=
                   sscanf(buf.offset(2 as libc::c_int as isize),
                          b"%d.%d.%d\x00" as *const u8 as *const libc::c_char,
                          &mut v1 as *mut libc::c_int,
                          &mut v2 as *mut libc::c_int,
                          &mut v3 as *mut libc::c_int) {
                return 2 as libc::c_int
            }
            /* Okay to proceed */
            okay = 1 as libc::c_int as bool_
        } else {
            /* No version yet */
            if okay == 0 { return 2 as libc::c_int }
            /* Included file */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '<' as i32 {
                fp_stack_push(buf.offset(2 as libc::c_int as isize) as cptr);
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'N' as i32 {
                /* Process 'N' for "New/Number/Name" */
                /* Find the colon before the name */
                s = strchr(buf.offset(2 as libc::c_int as isize), ':' as i32);
                /* Verify that colon */
                if s.is_null() { return 1 as libc::c_int }
                /* Nuke the colon, advance to the name */
                let fresh27 = s;
                s = s.offset(1);
                *fresh27 = '\u{0}' as i32 as libc::c_char;
                /* Paranoia -- require a name */
                if *s == 0 { return 1 as libc::c_int }
                /* Get the index */
                i = atoi(buf.offset(2 as libc::c_int as isize));
                /* Verify information */
                if i <= error_idx as libc::c_int { return 4 as libc::c_int }
                /* Verify information */
                if i >= (*f_head).info_num as libc::c_int {
                    return 2 as libc::c_int
                }
                /* Save the index */
                error_idx = i as s16b;
                /* Point at the "info" */
                f_ptr = &mut *f_info.offset(i as isize) as *mut feature_type;
                /* Hack -- Verify space */
                if ((*f_head).name_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_name_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the name index */
                if (*f_ptr).name == 0 {
                    (*f_head).name_size = (*f_head).name_size.wrapping_add(1);
                    (*f_ptr).name = (*f_head).name_size
                }
                /* Append chars to the name */
                strcpy(f_name.offset((*f_head).name_size as isize), s);
                /* Advance the index */
                (*f_head).name_size =
                    ((*f_head).name_size as
                         libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                        u32b;
                /* Default "mimic" */
                (*f_ptr).mimic = i as byte_hack;
                (*f_ptr).text = default_desc;
                (*f_ptr).block = default_desc;
                (*f_ptr).tunnel = default_tunnel;
                (*f_ptr).block = default_block
            } else {
                /* There better be a current f_ptr */
                if f_ptr.is_null() { return 3 as libc::c_int }
                /* Process 'D' for "Descriptions" */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'D' as i32 {
                    /* Acquire the text */
                    s = buf.offset(4 as libc::c_int as isize);
                    /* Hack -- Verify space */
                    if ((*f_head).text_size as
                            libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                           > fake_text_size as libc::c_ulong {
                        return 7 as libc::c_int
                    }
                    match *buf.offset(2 as libc::c_int as isize) as
                              libc::c_int {
                        48 => {
                            /* Advance and Save the text index */
                            (*f_head).text_size =
                                (*f_head).text_size.wrapping_add(1);
                            (*f_ptr).text = (*f_head).text_size
                        }
                        49 => {
                            /* Advance and Save the text index */
                            (*f_head).text_size =
                                (*f_head).text_size.wrapping_add(1);
                            (*f_ptr).tunnel = (*f_head).text_size
                        }
                        50 => {
                            /* Advance and Save the text index */
                            (*f_head).text_size =
                                (*f_head).text_size.wrapping_add(1);
                            (*f_ptr).block = (*f_head).text_size
                        }
                        _ => { return 6 as libc::c_int }
                    }
                    /* Append chars to the name */
                    strcpy(f_text.offset((*f_head).text_size as isize), s);
                    /* Advance the index */
                    (*f_head).text_size =
                        ((*f_head).text_size as
                             libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                            u32b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'M' as i32 {
                    let mut mimic: libc::c_int = 0;
                    /* Process 'M' for "Mimic" (one line only) */
                    /* Scan for the values */
                    if 1 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut mimic as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*f_ptr).mimic = mimic as byte_hack
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'S' as i32 {
                    let mut s0: libc::c_char = 0;
                    let mut s1: libc::c_char = 0;
                    let mut s2: libc::c_char = 0;
                    let mut s3: libc::c_char = 0;
                    let mut s4: libc::c_char = 0;
                    let mut s5: libc::c_char = 0;
                    let mut s6: libc::c_char = 0;
                    /* Process 'S' for "Shimmer" (one line only) */
                    /* Scan for the values */
                    if 7 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%c:%c:%c:%c:%c:%c:%c\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut s0 as *mut libc::c_char,
                                  &mut s1 as *mut libc::c_char,
                                  &mut s2 as *mut libc::c_char,
                                  &mut s3 as *mut libc::c_char,
                                  &mut s4 as *mut libc::c_char,
                                  &mut s5 as *mut libc::c_char,
                                  &mut s6 as *mut libc::c_char) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*f_ptr).shimmer[0 as libc::c_int as usize] =
                        color_char_to_attr(s0) as byte_hack;
                    (*f_ptr).shimmer[1 as libc::c_int as usize] =
                        color_char_to_attr(s1) as byte_hack;
                    (*f_ptr).shimmer[2 as libc::c_int as usize] =
                        color_char_to_attr(s2) as byte_hack;
                    (*f_ptr).shimmer[3 as libc::c_int as usize] =
                        color_char_to_attr(s3) as byte_hack;
                    (*f_ptr).shimmer[4 as libc::c_int as usize] =
                        color_char_to_attr(s4) as byte_hack;
                    (*f_ptr).shimmer[5 as libc::c_int as usize] =
                        color_char_to_attr(s5) as byte_hack;
                    (*f_ptr).shimmer[6 as libc::c_int as usize] =
                        color_char_to_attr(s6) as byte_hack
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'G' as i32 {
                    let mut tmp: libc::c_int = 0;
                    /* Process 'G' for "Graphics" (one line only) */
                    /* Paranoia */
                    if *buf.offset(2 as libc::c_int as isize) == 0 {
                        return 1 as libc::c_int
                    }
                    if *buf.offset(3 as libc::c_int as isize) == 0 {
                        return 1 as libc::c_int
                    }
                    if *buf.offset(4 as libc::c_int as isize) == 0 {
                        return 1 as libc::c_int
                    }
                    /* Extract the color */
                    tmp =
                        color_char_to_attr(*buf.offset(4 as libc::c_int as
                                                           isize));
                    /* Paranoia */
                    if tmp < 0 as libc::c_int { return 1 as libc::c_int }
                    /* Save the values */
                    (*f_ptr).d_attr = tmp as byte_hack;
                    (*f_ptr).d_char = *buf.offset(2 as libc::c_int as isize)
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'E' as i32 {
                    let mut side: libc::c_int = 0;
                    let mut dice: libc::c_int = 0;
                    let mut freq: libc::c_int = 0;
                    let mut type_0: libc::c_int = 0;
                    let mut tmp_0: cptr = 0 as *const libc::c_char;
                    /* Process 'E' for "Effects" (up to four lines) -SC- */
                    /* Find the next empty blow slot (if any) */
                    i = 0 as libc::c_int;
                    while i < 4 as libc::c_int {
                        if (*f_ptr).d_side[i as usize] == 0 &&
                               (*f_ptr).d_dice[i as usize] == 0 {
                            break ;
                        }
                        i += 1
                    }
                    /* Oops, no more slots */
                    if i == 4 as libc::c_int { return 1 as libc::c_int }
                    /* Scan for the values */
                    if 4 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%dd%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut dice as *mut libc::c_int,
                                  &mut side as *mut libc::c_int,
                                  &mut freq as *mut libc::c_int,
                                  &mut type_0 as *mut libc::c_int) {
                        let mut j: libc::c_int = 0;
                        if 3 as libc::c_int !=
                               sscanf(buf.offset(2 as libc::c_int as isize),
                                      b"%dd%d:%d\x00" as *const u8 as
                                          *const libc::c_char,
                                      &mut dice as *mut libc::c_int,
                                      &mut side as *mut libc::c_int,
                                      &mut freq as *mut libc::c_int) {
                            return 1 as libc::c_int
                        }
                        tmp_0 = buf.offset(2 as libc::c_int as isize) as cptr;
                        j = 0 as libc::c_int;
                        while j < 2 as libc::c_int {
                            tmp_0 = strchr(tmp_0, ':' as i32) as cptr;
                            if tmp_0.is_null() { return 1 as libc::c_int }
                            tmp_0 = tmp_0.offset(1);
                            j += 1
                        }
                        j = 0 as libc::c_int;
                        while !d_info_dtypes[j as usize].name.is_null() {
                            if strcmp(d_info_dtypes[j as usize].name, tmp_0)
                                   == 0 as libc::c_int {
                                (*f_ptr).d_type[i as usize] =
                                    d_info_dtypes[j as usize].feat;
                                break ;
                            } else { j += 1 }
                        }
                        if d_info_dtypes[j as usize].name.is_null() {
                            return 1 as libc::c_int
                        }
                    } else { (*f_ptr).d_type[i as usize] = type_0 }
                    freq *= 10 as libc::c_int;
                    /* Save the values */
                    (*f_ptr).d_side[i as usize] = side;
                    (*f_ptr).d_dice[i as usize] = dice;
                    (*f_ptr).d_frequency[i as usize] = freq
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'F' as i32 {
                    /* Hack -- Process 'F' for flags */
                    /* Parse every entry textually */
                    s = buf.offset(2 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh28 = t;
                            t = t.offset(1);
                            *fresh28 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_feature_flag(f_ptr, s as cptr) {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else {
                    /* Oops */
                    return 6 as libc::c_int
                }
            }
        }
    }
    /* Complete the "name" and "text" sizes */
    (*f_head).name_size = (*f_head).name_size.wrapping_add(1);
    (*f_head).text_size = (*f_head).text_size.wrapping_add(1);
    /* No version yet */
    if okay == 0 { return 2 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Grab one flag in an object_kind from a textual string
 */
unsafe extern "C" fn grab_one_kind_flag(mut k_ptr: *mut object_kind,
                                        mut what: cptr, mut obvious: bool_)
 -> errr {
    let mut i: libc::c_int = 0;
    /* Check flags1 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags1[i as usize]) != 0 {
            if obvious != 0 {
                (*k_ptr).oflags1 =
                    ((*k_ptr).oflags1 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*k_ptr).flags1 =
                    ((*k_ptr).flags1 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Check flags2 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags2[i as usize]) != 0 {
            if obvious != 0 {
                (*k_ptr).oflags2 =
                    ((*k_ptr).oflags2 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*k_ptr).flags2 =
                    ((*k_ptr).flags2 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Check flags2 -- traps*/
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags2_trap[i as usize]) != 0 {
            if obvious != 0 {
                (*k_ptr).oflags2 =
                    ((*k_ptr).oflags2 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*k_ptr).flags2 =
                    ((*k_ptr).flags2 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Check flags3 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags3[i as usize]) != 0 {
            if obvious != 0 {
                (*k_ptr).oflags3 =
                    ((*k_ptr).oflags3 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*k_ptr).flags3 =
                    ((*k_ptr).flags3 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Check flags4 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags4[i as usize]) != 0 {
            if obvious != 0 {
                (*k_ptr).oflags4 =
                    ((*k_ptr).oflags4 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*k_ptr).flags4 =
                    ((*k_ptr).flags4 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Check flags5 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags5[i as usize]) != 0 {
            if obvious != 0 {
                (*k_ptr).oflags5 =
                    ((*k_ptr).oflags5 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*k_ptr).flags5 =
                    ((*k_ptr).flags5 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Check esp_flags */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, esp_flags[i as usize]) != 0 {
            if obvious != 0 {
                (*k_ptr).oesp =
                    ((*k_ptr).oesp as libc::c_long | (1 as libc::c_long) << i)
                        as u32b
            } else {
                (*k_ptr).esp =
                    ((*k_ptr).esp as libc::c_long | (1 as libc::c_long) << i)
                        as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Oops */
    msg_format(b"Unknown object flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Error */
    return 1 as libc::c_int;
}
/*
 * Initialize the "k_info" array, by parsing an ascii "template" file
 */
#[no_mangle]
pub unsafe extern "C" fn init_k_info_txt(mut fp: *mut FILE,
                                         mut buf: *mut libc::c_char) -> errr {
    let mut i: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Not ready yet */
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    /* Current entry */
    let mut k_ptr: *mut object_kind = 0 as *mut object_kind;
    /* Just before the first record */
    error_idx = -(1 as libc::c_int) as s16b;
    /* Just before the first line */
    error_line = -(1 as libc::c_int) as s16b;
    /* Prepare the "fake" stuff */
    (*k_head).name_size = 0 as libc::c_int as u32b;
    (*k_head).text_size = 0 as libc::c_int as u32b;
    /* Parse */
    fp_stack_init(fp);
    /* Next... */
    while 0 as libc::c_int == my_fgets_dostack(buf, 1024 as libc::c_int) {
        /* Advance the line number */
        error_line += 1;
        /* Skip comments and blank lines */
        if *buf.offset(0 as libc::c_int as isize) == 0 ||
               *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '#' as i32 {
            continue ;
        }
        /* Verify correct "colon" format */
        if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
           {
            return 1 as libc::c_int
        }
        /* Hack -- Process 'V' for "Version" */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'V' as i32
           {
            let mut v1: libc::c_int = 0;
            let mut v2: libc::c_int = 0;
            let mut v3: libc::c_int = 0;
            /* Scan for the values */
            if 3 as libc::c_int !=
                   sscanf(buf.offset(2 as libc::c_int as isize),
                          b"%d.%d.%d\x00" as *const u8 as *const libc::c_char,
                          &mut v1 as *mut libc::c_int,
                          &mut v2 as *mut libc::c_int,
                          &mut v3 as *mut libc::c_int) {
                return 2 as libc::c_int
            }
            /* Okay to proceed */
            okay = 1 as libc::c_int as bool_
        } else {
            /* No version yet */
            if okay == 0 { return 2 as libc::c_int }
            /* Included file */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '<' as i32 {
                fp_stack_push(buf.offset(2 as libc::c_int as isize) as cptr);
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'N' as i32 {
                /* Process 'N' for "New/Number/Name" */
                /* Find the colon before the name */
                s = strchr(buf.offset(2 as libc::c_int as isize), ':' as i32);
                /* Verify that colon */
                if s.is_null() { return 1 as libc::c_int }
                /* Nuke the colon, advance to the name */
                let fresh29 = s;
                s = s.offset(1);
                *fresh29 = '\u{0}' as i32 as libc::c_char;
                /* Paranoia -- require a name */
                if *s == 0 { return 1 as libc::c_int }
                /* Get the index */
                i = atoi(buf.offset(2 as libc::c_int as isize));
                /* Verify information */
                if i <= error_idx as libc::c_int { return 4 as libc::c_int }
                /* Verify information */
                if i >= (*k_head).info_num as libc::c_int {
                    return 2 as libc::c_int
                }
                /* Save the index */
                error_idx = i as s16b;
                /* Point at the "info" */
                k_ptr = &mut *k_info.offset(i as isize) as *mut object_kind;
                /* Hack -- Verify space */
                if ((*k_head).name_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_name_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the name index */
                if (*k_ptr).name == 0 {
                    (*k_head).name_size = (*k_head).name_size.wrapping_add(1);
                    (*k_ptr).name = (*k_head).name_size
                }
                /* Append chars to the name */
                strcpy(k_name.offset((*k_head).name_size as isize), s);
                /* Advance the index */
                (*k_head).name_size =
                    ((*k_head).name_size as
                         libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                        u32b;
                /* Needed hack */
                (*k_ptr).esp = 0 as libc::c_int as u32b;
                (*k_ptr).power = -(1 as libc::c_int) as s16b
            } else {
                /* There better be a current k_ptr */
                if k_ptr.is_null() { return 3 as libc::c_int }
                /* Process 'D' for "Description" */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'D' as i32 {
                    /* Acquire the text */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Hack -- Verify space */
                    if ((*k_head).text_size as
                            libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                           > fake_text_size as libc::c_ulong {
                        return 7 as libc::c_int
                    }
                    /* Advance and Save the text index */
                    if (*k_ptr).text == 0 {
                        (*k_head).text_size =
                            (*k_head).text_size.wrapping_add(1);
                        (*k_ptr).text = (*k_head).text_size
                    } else if *k_text.offset((*k_head).text_size.wrapping_sub(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
                                                 as isize) as libc::c_int !=
                                  ' ' as i32 {
                        /* Append a space if needed */
                        /* Append chars to the name */
                        strcpy(k_text.offset((*k_head).text_size as isize),
                               b" \x00" as *const u8 as *const libc::c_char);
                        /* Advance the index */
                        (*k_head).text_size =
                            ((*k_head).text_size as
                                 libc::c_uint).wrapping_add(1 as libc::c_int
                                                                as
                                                                libc::c_uint)
                                as u32b as u32b
                    }
                    /* Append chars to the name */
                    strcpy(k_text.offset((*k_head).text_size as isize), s);
                    /* Advance the index */
                    (*k_head).text_size =
                        ((*k_head).text_size as
                             libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                            u32b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'G' as i32 {
                    let mut sym: libc::c_char = 0;
                    let mut tmp: libc::c_int = 0;
                    /* Process 'G' for "Graphics" (one line only) */
                    /* Paranoia */
                    if *buf.offset(2 as libc::c_int as isize) == 0 {
                        return 1 as libc::c_int
                    }
                    if *buf.offset(3 as libc::c_int as isize) == 0 {
                        return 1 as libc::c_int
                    }
                    if *buf.offset(4 as libc::c_int as isize) == 0 {
                        return 1 as libc::c_int
                    }
                    /* Extract the char */
                    sym = *buf.offset(2 as libc::c_int as isize);
                    /* Extract the attr */
                    tmp =
                        color_char_to_attr(*buf.offset(4 as libc::c_int as
                                                           isize));
                    /* Paranoia */
                    if tmp < 0 as libc::c_int { return 1 as libc::c_int }
                    /* Save the values */
                    (*k_ptr).d_attr = tmp as byte_hack;
                    (*k_ptr).d_char = sym
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'I' as i32 {
                    let mut tval: libc::c_int = 0;
                    let mut sval: libc::c_int = 0;
                    let mut pval: libc::c_int = 0;
                    let mut pval2: libc::c_int = 0 as libc::c_int;
                    /* Process 'I' for "Info" (one line only) */
                    /* Scan for the values */
                    if 4 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut tval as *mut libc::c_int,
                                  &mut sval as *mut libc::c_int,
                                  &mut pval as *mut libc::c_int,
                                  &mut pval2 as *mut libc::c_int) {
                        let mut spl: [libc::c_char; 70] = [0; 70];
                        if 4 as libc::c_int !=
                               sscanf(buf.offset(2 as libc::c_int as isize),
                                      b"%d:%d:%d:SPELL=%s\x00" as *const u8 as
                                          *const libc::c_char,
                                      &mut tval as *mut libc::c_int,
                                      &mut sval as *mut libc::c_int,
                                      &mut pval as *mut libc::c_int,
                                      spl.as_mut_ptr()) {
                            if 3 as libc::c_int !=
                                   sscanf(buf.offset(2 as libc::c_int as
                                                         isize),
                                          b"%d:%d:%d\x00" as *const u8 as
                                              *const libc::c_char,
                                          &mut tval as *mut libc::c_int,
                                          &mut sval as *mut libc::c_int,
                                          &mut pval as *mut libc::c_int) {
                                return 1 as libc::c_int
                            }
                        } else {
                            let mut spl_0: *mut libc::c_char =
                                strchr(buf.offset(2 as libc::c_int as isize),
                                       '=' as
                                           i32).offset(1 as libc::c_int as
                                                           isize);
                            pval2 = find_spell(spl_0)
                        }
                    }
                    /* Save the values */
                    (*k_ptr).tval = tval as byte_hack;
                    (*k_ptr).sval = sval as byte_hack;
                    (*k_ptr).pval = pval;
                    (*k_ptr).pval2 = pval2
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'W' as i32 {
                    let mut level: libc::c_int = 0;
                    let mut extra: libc::c_int = 0;
                    let mut wgt: libc::c_int = 0;
                    let mut cost: libc::c_long = 0;
                    /* Process 'W' for "More Info" (one line only) */
                    /* Scan for the values */
                    if 4 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d:%ld\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut level as *mut libc::c_int,
                                  &mut extra as *mut libc::c_int,
                                  &mut wgt as *mut libc::c_int,
                                  &mut cost as *mut libc::c_long) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*k_ptr).level = level as byte_hack;
                    (*k_ptr).extra = extra as byte_hack;
                    (*k_ptr).weight = wgt;
                    (*k_ptr).cost = cost as s32b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'T' as i32 {
                    let mut btval: libc::c_int = 0;
                    let mut bsval: libc::c_int = 0;
                    /* Process 'T' for "arTifact Info" (one line only) */
                    /* Scan for the values */
                    if 2 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut btval as *mut libc::c_int,
                                  &mut bsval as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*k_ptr).btval = btval as byte_hack;
                    (*k_ptr).bsval = bsval as byte_hack
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'Z' as i32 {
                    let mut i_0: libc::c_int = 0;
                    /* Process 'Z' for "Granted power" */
                    /* Acquire the text */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Find it in the list */
                    i_0 = 0 as libc::c_int;
                    while i_0 < power_max as libc::c_int {
                        if strcasecmp(s,
                                      (*powers_type.offset(i_0 as
                                                               isize)).name)
                               == 0 {
                            break ;
                        }
                        i_0 += 1
                    }
                    if i_0 == power_max as libc::c_int {
                        return 6 as libc::c_int
                    }
                    (*k_ptr).power = i_0 as s16b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'a' as i32 {
                    if prefix(buf.offset(2 as libc::c_int as isize) as cptr,
                              b"HARDCORE=\x00" as *const u8 as
                                  *const libc::c_char) != 0 {
                        (*k_ptr).activate =
                            get_activation(buf.offset(11 as libc::c_int as
                                                          isize)) as s16b;
                        if (*k_ptr).activate as libc::c_int ==
                               -(1 as libc::c_int) {
                            return 1 as libc::c_int
                        }
                    } else if prefix(buf.offset(2 as libc::c_int as isize) as
                                         cptr,
                                     b"SPELL=\x00" as *const u8 as
                                         *const libc::c_char) != 0 {
                        (*k_ptr).activate =
                            -find_spell(buf.offset(8 as libc::c_int as isize))
                                as s16b;
                        if (*k_ptr).activate as libc::c_int ==
                               --(1 as libc::c_int) {
                            return 1 as libc::c_int
                        }
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'A' as i32 {
                    let mut i_1: libc::c_int = 0;
                    /* Process 'a' for Activation */
                    /* Process 'A' for "Allocation" (one line only) */
                    /* XXX XXX XXX Simply read each number following a colon */
                    i_1 = 0 as libc::c_int;
                    s = buf.offset(1 as libc::c_int as isize);
                    while !s.is_null() &&
                              *s.offset(0 as libc::c_int as isize) as
                                  libc::c_int == ':' as i32 &&
                              *s.offset(1 as libc::c_int as isize) as
                                  libc::c_int != 0 {
                        /* Default chance */
                        (*k_ptr).chance[i_1 as usize] =
                            1 as libc::c_int as byte_hack;
                        /* Store the level */
                        (*k_ptr).locale[i_1 as usize] =
                            atoi(s.offset(1 as libc::c_int as isize)) as
                                byte_hack;
                        /* Find the slash */
                        t =
                            strchr(s.offset(1 as libc::c_int as isize),
                                   '/' as i32);
                        /* Find the next colon */
                        s =
                            strchr(s.offset(1 as libc::c_int as isize),
                                   ':' as i32);
                        /* If the slash is "nearby", use it */
                        if !t.is_null() && (s.is_null() || t < s) {
                            let mut chance: libc::c_int =
                                atoi(t.offset(1 as libc::c_int as isize));
                            if chance > 0 as libc::c_int {
                                (*k_ptr).chance[i_1 as usize] =
                                    chance as byte_hack
                            }
                        }
                        i_1 += 1
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'P' as i32 {
                    let mut ac: libc::c_int = 0;
                    let mut hd1: libc::c_int = 0;
                    let mut hd2: libc::c_int = 0;
                    let mut th: libc::c_int = 0;
                    let mut td: libc::c_int = 0;
                    let mut ta: libc::c_int = 0;
                    /* Hack -- Process 'P' for "power" and such */
                    /* Scan for the values */
                    if 6 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%dd%d:%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut ac as *mut libc::c_int,
                                  &mut hd1 as *mut libc::c_int,
                                  &mut hd2 as *mut libc::c_int,
                                  &mut th as *mut libc::c_int,
                                  &mut td as *mut libc::c_int,
                                  &mut ta as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    (*k_ptr).ac = ac as s16b;
                    (*k_ptr).dd = hd1 as byte_hack;
                    (*k_ptr).ds = hd2 as byte_hack;
                    (*k_ptr).to_h = th as s16b;
                    (*k_ptr).to_d = td as s16b;
                    (*k_ptr).to_a = ta as s16b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'F' as i32 {
                    /* Hack -- Process 'F' for flags */
                    /* Parse every entry textually */
                    s = buf.offset(2 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh30 = t;
                            t = t.offset(1);
                            *fresh30 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_kind_flag(k_ptr, s as cptr,
                                                  0 as libc::c_int as bool_) {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'f' as i32 {
                    /* Hack -- Process 'f' for obvious flags */
                    /* Parse every entry textually */
                    s = buf.offset(2 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh31 = t;
                            t = t.offset(1);
                            *fresh31 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_kind_flag(k_ptr, s as cptr,
                                                  1 as libc::c_int as bool_) {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else {
                    /* Oops */
                    return 6 as libc::c_int
                }
            }
        }
    }
    /* Complete the "name" and "text" sizes */
    (*k_head).name_size = (*k_head).name_size.wrapping_add(1);
    (*k_head).text_size = (*k_head).text_size.wrapping_add(1);
    /* No version yet */
    if okay == 0 { return 2 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
/*Get a kind flag, return flag*32+bit number or -1 for unknown*/
#[no_mangle]
pub unsafe extern "C" fn get_k_flag(mut what: *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    /* Check flags1 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what as cptr, k_info_flags1[i as usize]) != 0 { return i }
        if streq(what as cptr, k_info_flags2[i as usize]) != 0 {
            return 1 as libc::c_int * 32 as libc::c_int + i
        }
        if streq(what as cptr, k_info_flags2_trap[i as usize]) != 0 {
            return 1 as libc::c_int * 32 as libc::c_int + i
        }
        if streq(what as cptr, k_info_flags3[i as usize]) != 0 {
            return 2 as libc::c_int * 32 as libc::c_int + i
        }
        if streq(what as cptr, k_info_flags4[i as usize]) != 0 {
            return 3 as libc::c_int * 32 as libc::c_int + i
        }
        if streq(what as cptr, k_info_flags5[i as usize]) != 0 {
            return 4 as libc::c_int * 32 as libc::c_int + i
        }
        if streq(what as cptr, esp_flags[i as usize]) != 0 {
            return 5 as libc::c_int * 32 as libc::c_int + i
        }
        i += 1
    }
    /* Oops */
    msg_format(b"Unknown object flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Error */
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn get_r_flag(mut what: *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    /* Check flags */
	/* this processes all r_info_flag arrays in parallel.
	   Seemed like a good idea at the time..
	*/
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what as cptr, r_info_flags1[i as usize]) != 0 { return i }
        if streq(what as cptr, r_info_flags2[i as usize]) != 0 {
            return 1 as libc::c_int * 32 as libc::c_int + i
        }
        if streq(what as cptr, r_info_flags3[i as usize]) != 0 {
            return 2 as libc::c_int * 32 as libc::c_int + i
        }
        if streq(what as cptr, r_info_flags4[i as usize]) != 0 {
            return 3 as libc::c_int * 32 as libc::c_int + i
        }
        if streq(what as cptr, r_info_flags5[i as usize]) != 0 {
            return 4 as libc::c_int * 32 as libc::c_int + i
        }
        if streq(what as cptr, r_info_flags6[i as usize]) != 0 {
            return 5 as libc::c_int * 32 as libc::c_int + i
        }
        if streq(what as cptr, r_info_flags7[i as usize]) != 0 {
            return 6 as libc::c_int * 32 as libc::c_int + i
        }
        if streq(what as cptr, r_info_flags8[i as usize]) != 0 {
            return 7 as libc::c_int * 32 as libc::c_int + i
        }
        if streq(what as cptr, r_info_flags9[i as usize]) != 0 {
            return 8 as libc::c_int * 32 as libc::c_int + i
        }
        i += 1
    }
    /* Oops */
    msg_format(b"Unknown race flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Error */
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn init_al_info_essence(mut essence: *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while *essence_names[i as usize].offset(0 as libc::c_int as isize) != 0 {
        if strncmp(essence_names[i as usize], essence,
                   9 as libc::c_int as libc::c_ulong) == 0 {
            return i
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
/*
 * Initialize the "al_info" array, by parsing an ascii "template" file
 */
#[no_mangle]
pub unsafe extern "C" fn init_al_info_txt(mut fp: *mut FILE,
                                          mut buf: *mut libc::c_char)
 -> errr {
    let mut al_idx: libc::c_int = 0 as libc::c_int;
    let mut a_idx: libc::c_int = 0 as libc::c_int;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut a_ptr: *mut artifact_select_flag = 0 as *mut artifact_select_flag;
    /* Not ready yet */
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    /* Just before the first record */
    error_idx = -(1 as libc::c_int) as s16b;
    /* Just before the first line */
    error_line = -(1 as libc::c_int) as s16b;
    /* Fun! */
    (*al_head).name_size = 0 as libc::c_int as u32b;
    *al_name = 0 as libc::c_int as libc::c_char;
    /* Parse */
    fp_stack_init(fp);
    /* Next... */
    while 0 as libc::c_int == my_fgets_dostack(buf, 1024 as libc::c_int) {
        /* Advance the line number */
        error_line += 1;
        /* Skip comments and blank lines */
        if *buf.offset(0 as libc::c_int as isize) == 0 ||
               *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '#' as i32 {
            continue ;
        }
        /* Verify correct "colon" format */
        if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
           {
            return 1 as libc::c_int
        }
        /* Hack -- Process 'V' for "Version" */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'V' as i32
           {
            let mut v1: libc::c_int = 0;
            let mut v2: libc::c_int = 0;
            let mut v3: libc::c_int = 0;
            /* Scan for the values */
            if 3 as libc::c_int !=
                   sscanf(buf.offset(2 as libc::c_int as isize),
                          b"%d.%d.%d\x00" as *const u8 as *const libc::c_char,
                          &mut v1 as *mut libc::c_int,
                          &mut v2 as *mut libc::c_int,
                          &mut v3 as *mut libc::c_int) {
                return 2 as libc::c_int
            }
            /* Okay to proceed */
            okay = 1 as libc::c_int as bool_
        } else {
            /* No version yet */
            if okay == 0 { return 2 as libc::c_int }
            /* Included file */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '<' as i32 {
                fp_stack_push(buf.offset(2 as libc::c_int as isize) as cptr);
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'I' as i32 {
                let mut tval: libc::c_int = 0;
                let mut sval: libc::c_int = 0;
                let mut qty: libc::c_int = 0;
                /* Process 'I' for "Info" (one line only) */
                /* Scan for the values */
                if 3 as libc::c_int !=
                       sscanf(buf.offset(2 as libc::c_int as isize),
                              b"%d:%d:%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut tval as *mut libc::c_int,
                              &mut sval as *mut libc::c_int,
                              &mut qty as *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                /* ignore everything after the first space. */
                s = strchr(buf, ' ' as i32);
                if !s.is_null() { *s = 0 as libc::c_int as libc::c_char }
                /* Save the values */
                (*alchemist_recipes.offset(al_idx as isize)).tval =
                    tval as byte_hack; /* 7 is an 'out of memory' error */
                (*alchemist_recipes.offset(al_idx as isize)).sval =
                    sval as byte_hack;
                (*alchemist_recipes.offset(al_idx as isize)).qty =
                    qty as byte_hack;
                (*alchemist_recipes.offset(al_idx as isize)).sval_essence =
                    init_al_info_essence(strrchr(buf,
                                                 ':' as
                                                     i32).offset(1 as
                                                                     libc::c_int
                                                                     as
                                                                     isize));
                if (*alchemist_recipes.offset(al_idx as isize)).sval_essence <
                       0 as libc::c_int {
                    return 5 as libc::c_int
                }
                al_idx += 1;
                if al_idx >= max_al_idx as libc::c_int {
                    return 7 as libc::c_int
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'a' as i32 {
                let mut qty_0: libc::c_int = 0;
                if 1 as libc::c_int !=
                       sscanf(buf.offset(2 as libc::c_int as isize),
                              b"%d\x00" as *const u8 as *const libc::c_char,
                              &mut qty_0 as *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                s = strrchr(buf, ':' as i32);
                let fresh32 = s;
                s = s.offset(1);
                *fresh32 = 0 as libc::c_int as libc::c_char;
                t = strchr(s, ' ' as i32);
                let fresh33 = t;
                t = t.offset(1);
                *fresh33 = 0 as libc::c_int as libc::c_char;
                (*alchemist_recipes.offset(al_idx as isize)).tval =
                    0 as libc::c_int as byte_hack;
                (*alchemist_recipes.offset(al_idx as isize)).sval =
                    get_k_flag(s) as byte_hack;
                (*alchemist_recipes.offset(al_idx as isize)).qty =
                    qty_0 as byte_hack;
                (*alchemist_recipes.offset(al_idx as isize)).sval_essence =
                    init_al_info_essence(t);
                if (*alchemist_recipes.offset(al_idx as isize)).sval_essence <
                       0 as libc::c_int {
                    return 1 as libc::c_int
                }
                al_idx += 1;
                if al_idx >= max_al_idx as libc::c_int {
                    return 7 as libc::c_int
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'A' as i32 {
                let mut group: libc::c_int = 0;
                let mut level: libc::c_int = 0;
                let mut pval: libc::c_int = 0;
                let mut rtval: libc::c_int = 0;
                let mut rsval: libc::c_int = 0;
                let mut rpval: libc::c_int = 0;
                let mut xp: libc::c_long = 0;
                /*Verify that complete description information is
			  Recorded for previous Artifact flag
			*/
                if !a_ptr.is_null() &&
                       ((*a_ptr).group == 0 || (*a_ptr).desc == 0 ||
                            ((*a_ptr).item_desc == 0) as libc::c_int !=
                                ((*a_ptr).rtval == 0) as libc::c_int) {
                    return 1 as libc::c_int
                }
                let fresh34 = a_idx;
                a_idx = a_idx + 1;
                a_ptr =
                    &mut *a_select_flags.offset(fresh34 as isize) as
                        *mut artifact_select_flag;
                if 7 as libc::c_int !=
                       sscanf(buf.offset(2 as libc::c_int as isize),
                              b"%d:%d:%d:%d:%d:%d:%ld\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut group as *mut libc::c_int,
                              &mut rtval as *mut libc::c_int,
                              &mut rsval as *mut libc::c_int,
                              &mut rpval as *mut libc::c_int,
                              &mut pval as *mut libc::c_int,
                              &mut level as *mut libc::c_int,
                              &mut xp as *mut libc::c_long) {
                    return 1 as libc::c_int
                }
                (*a_ptr).group = group as byte_hack;
                (*a_ptr).rtval = rtval as byte_hack;
                (*a_ptr).rsval = rsval as byte_hack;
                (*a_ptr).rpval = rpval;
                (*a_ptr).pval = pval as bool_;
                (*a_ptr).level = level as byte_hack;
                (*a_ptr).xp = xp as u32b
            } else {
                /*Anything else here MUST be a artifact flag line*/
                if a_ptr.is_null() { return 3 as libc::c_int }
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'F' as i32 {
                    /* Get the Item flag associated with this */
                    (*a_ptr).flag =
                        get_k_flag(buf.offset(2 as libc::c_int as isize));
                    if (*a_ptr).flag == -(1 as libc::c_int) {
                        return 1 as libc::c_int
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'x' as i32 {
                    /* Get the activation name associated with this */
                    (*a_ptr).flag =
                        -get_activation(buf.offset(2 as libc::c_int as
                                                       isize));
                    if (*a_ptr).flag == 1 as libc::c_int {
                        return 1 as libc::c_int
                    }
                    (*a_ptr).group = 88 as libc::c_int as byte_hack;
                    (*a_ptr).pval = 0 as libc::c_int as bool_
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'f' as i32 {
                    let mut s_0: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut t_0: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut idx: libc::c_int = 0 as libc::c_int;
                    if (*a_ptr).rflag[0 as libc::c_int as usize] != 0 {
                        msg_print(b"duplicate f: entries for one corpse\x00"
                                      as *const u8 as *const libc::c_char);
                        return 5 as libc::c_int
                    }
                    if (*a_ptr).rtval as libc::c_int != 9 as libc::c_int {
                        msg_print(b"f: section for corpse flags only\x00" as
                                      *const u8 as *const libc::c_char);
                        return 5 as libc::c_int
                    }
                    if (*a_ptr).rpval != 0 {
                        msg_print(b"Can\'t specify r_info.txt index with f: section\x00"
                                      as *const u8 as *const libc::c_char);
                        return 5 as libc::c_int
                    }
                    /* Get the race flags associated with this */
                    /* Parse every entry textually */
                    s_0 = buf.offset(2 as libc::c_int as isize);
                    while *s_0 != 0 {
                        /* Find the end of this entry */
                        t_0 = s_0;
                        while *t_0 as libc::c_int != 0 &&
                                  *t_0 as libc::c_int != ' ' as i32 &&
                                  *t_0 as libc::c_int != '|' as i32 {
                            /* loop */
                            t_0 = t_0.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t_0 != 0 {
                            let fresh35 = t_0;
                            t_0 = t_0.offset(1);
                            *fresh35 = '\u{0}' as i32 as libc::c_char;
                            while *t_0 as libc::c_int == ' ' as i32 ||
                                      *t_0 as libc::c_int == '|' as i32 {
                                t_0 = t_0.offset(1)
                            }
                        }
                        if idx > 5 as libc::c_int {
                            msg_print(b"Limit on race flags is currently 6\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                            return 5 as libc::c_int
                        }
                        /* Parse this entry */
                        (*a_ptr).rflag[idx as usize] = get_r_flag(s_0);
                        let fresh36 = idx;
                        idx = idx + 1;
                        if (*a_ptr).rflag[fresh36 as usize] ==
                               -(1 as libc::c_int) {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s_0 = t_0
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'p' as i32 {
                    /* Process 'p' for "Plural Description" */
		/* Only valid for flag which depend on pval */
                    /* Reject if doesn't depend on pval */
                    if (*a_ptr).pval == 0 { return 1 as libc::c_int }
                    /* Acquire the description */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Hack -- Verify space */
                    if ((*al_head).name_size as
                            libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                           > fake_name_size as libc::c_ulong {
                        return 7 as libc::c_int
                    }
                    /* Advance and Save the name index */
                    (*al_head).name_size =
                        (*al_head).name_size.wrapping_add(1);
                    (*a_ptr).item_descp = (*al_head).name_size as libc::c_int;
                    /* Append chars to the name */
                    strcpy(al_name.offset((*al_head).name_size as isize), s);
                    /* Advance the index */
                    (*al_head).name_size =
                        ((*al_head).name_size as
                             libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                            u32b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'D' as i32 {
                    /* Process 'D' for "Description" */
                    /* Acquire the description */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Hack -- Verify space */
                    if ((*al_head).name_size as
                            libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                           > fake_name_size as libc::c_ulong {
                        return 7 as libc::c_int
                    }
                    /* Advance and Save the name index */
                    (*al_head).name_size =
                        (*al_head).name_size.wrapping_add(1);
                    (*a_ptr).desc = (*al_head).name_size as libc::c_int;
                    /* Append chars to the name */
                    strcpy(al_name.offset((*al_head).name_size as isize), s);
                    /* Advance the index */
                    (*al_head).name_size =
                        ((*al_head).name_size as
                             libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                            u32b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'd' as i32 {
                    /* Process 'd' for "Item Description" */
                    /* Acquire the name */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Hack -- Verify space */
                    if ((*al_head).name_size as
                            libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                           > fake_name_size as libc::c_ulong {
                        return 7 as libc::c_int
                    }
                    if (*a_ptr).item_desc != 0 { return 7 as libc::c_int }
                    /* Advance and Save the name index */
                    (*al_head).name_size =
                        (*al_head).name_size.wrapping_add(1);
                    (*a_ptr).item_desc = (*al_head).name_size as libc::c_int;
                    /* Append chars to the name */
                    strcpy(al_name.offset((*al_head).name_size as isize), s);
                    /* Advance the index */
                    (*al_head).name_size =
                        ((*al_head).name_size as
                             libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                            u32b
                } else {
                    /* Oops */
                    return 6 as libc::c_int
                }
            }
        }
    }
    /* No version yet */
    if okay == 0 { return 2 as libc::c_int }
    /* Hack - set the al_head->text_size to byte size of array */
    (*al_head).text_size =
        ((a_idx + 1 as libc::c_int) as
             libc::c_ulong).wrapping_mul(::std::mem::size_of::<artifact_select_flag>()
                                             as libc::c_ulong) as u32b;
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Grab one flag in an artifact_type from a textual string
 */
unsafe extern "C" fn grab_one_artifact_flag(mut a_ptr: *mut artifact_type,
                                            mut what: cptr,
                                            mut obvious: bool_) -> errr {
    let mut i: libc::c_int = 0;
    /* Check flags1 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags1[i as usize]) != 0 {
            if obvious != 0 {
                (*a_ptr).oflags1 =
                    ((*a_ptr).oflags1 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*a_ptr).flags1 =
                    ((*a_ptr).flags1 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Check flags2 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags2[i as usize]) != 0 {
            if obvious != 0 {
                (*a_ptr).oflags2 =
                    ((*a_ptr).oflags2 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*a_ptr).flags2 =
                    ((*a_ptr).flags2 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Check flags2 -- traps*/
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags2_trap[i as usize]) != 0 {
            if obvious != 0 {
                (*a_ptr).oflags2 =
                    ((*a_ptr).oflags2 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*a_ptr).flags2 =
                    ((*a_ptr).flags2 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Check flags3 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags3[i as usize]) != 0 {
            if obvious != 0 {
                (*a_ptr).oflags3 =
                    ((*a_ptr).oflags3 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*a_ptr).flags3 =
                    ((*a_ptr).flags3 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Check flags4 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags4[i as usize]) != 0 {
            if obvious != 0 {
                (*a_ptr).oflags4 =
                    ((*a_ptr).oflags4 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*a_ptr).flags4 =
                    ((*a_ptr).flags4 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Check flags5 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags5[i as usize]) != 0 {
            if obvious != 0 {
                (*a_ptr).oflags5 =
                    ((*a_ptr).oflags5 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*a_ptr).flags5 =
                    ((*a_ptr).flags5 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Check esp_flags */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, esp_flags[i as usize]) != 0 {
            if obvious != 0 {
                (*a_ptr).oesp =
                    ((*a_ptr).oesp as libc::c_long | (1 as libc::c_long) << i)
                        as u32b
            } else {
                (*a_ptr).esp =
                    ((*a_ptr).esp as libc::c_long | (1 as libc::c_long) << i)
                        as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Oops */
    msg_format(b"Unknown artifact flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Error */
    return 1 as libc::c_int;
}
/*
 * Initialize the "a_info" array, by parsing an ascii "template" file
 */
#[no_mangle]
pub unsafe extern "C" fn init_a_info_txt(mut fp: *mut FILE,
                                         mut buf: *mut libc::c_char) -> errr {
    let mut i: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Not ready yet */
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    /* Current entry */
    let mut a_ptr: *mut artifact_type = 0 as *mut artifact_type;
    /* Just before the first record */
    error_idx = -(1 as libc::c_int) as s16b;
    /* Just before the first line */
    error_line = -(1 as libc::c_int) as s16b;
    /* Parse */
    fp_stack_init(fp);
    /* Next... */
    while 0 as libc::c_int == my_fgets_dostack(buf, 1024 as libc::c_int) {
        /* Advance the line number */
        error_line += 1;
        /* Skip comments and blank lines */
        if *buf.offset(0 as libc::c_int as isize) == 0 ||
               *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '#' as i32 {
            continue ;
        }
        /* Verify correct "colon" format */
        if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
           {
            return 1 as libc::c_int
        }
        /* Hack -- Process 'V' for "Version" */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'V' as i32
           {
            let mut v1: libc::c_int = 0;
            let mut v2: libc::c_int = 0;
            let mut v3: libc::c_int = 0;
            /* Scan for the values */
            if 3 as libc::c_int !=
                   sscanf(buf.offset(2 as libc::c_int as isize),
                          b"%d.%d.%d\x00" as *const u8 as *const libc::c_char,
                          &mut v1 as *mut libc::c_int,
                          &mut v2 as *mut libc::c_int,
                          &mut v3 as *mut libc::c_int) {
                return 2 as libc::c_int
            }
            /* Okay to proceed */
            okay = 1 as libc::c_int as bool_
        } else {
            /* No version yet */
            if okay == 0 { return 2 as libc::c_int }
            /* Included file */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '<' as i32 {
                fp_stack_push(buf.offset(2 as libc::c_int as isize) as cptr);
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'N' as i32 {
                /* Process 'N' for "New/Number/Name" */
                /* Find the colon before the name */
                s = strchr(buf.offset(2 as libc::c_int as isize), ':' as i32);
                /* Verify that colon */
                if s.is_null() { return 1 as libc::c_int }
                /* Nuke the colon, advance to the name */
                let fresh37 = s;
                s = s.offset(1);
                *fresh37 = '\u{0}' as i32 as libc::c_char;
                /* Paranoia -- require a name */
                if *s == 0 { return 1 as libc::c_int }
                /* Get the index */
                i = atoi(buf.offset(2 as libc::c_int as isize));
                /* Verify information */
                if i < error_idx as libc::c_int { return 4 as libc::c_int }
                /* Verify information */
                if i >= (*a_head).info_num as libc::c_int {
                    return 2 as libc::c_int
                }
                /* Save the index */
                error_idx = i as s16b;
                /* Point at the "info" */
                a_ptr = &mut *a_info.offset(i as isize) as *mut artifact_type;
                /* Hack -- Verify space */
                if ((*a_head).name_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_name_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the name index */
                if (*a_ptr).name == 0 {
                    (*a_head).name_size = (*a_head).name_size.wrapping_add(1);
                    (*a_ptr).name = (*a_head).name_size
                }
                /* Append chars to the name */
                strcpy(a_name.offset((*a_head).name_size as isize), s);
                /* Advance the index */
                (*a_head).name_size =
                    ((*a_head).name_size as
                         libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                        u32b;
                /* Ignore everything */
                (*a_ptr).flags3 =
                    ((*a_ptr).flags3 as libc::c_long |
                         0x100000 as libc::c_long) as u32b;
                (*a_ptr).flags3 =
                    ((*a_ptr).flags3 as libc::c_long |
                         0x200000 as libc::c_long) as u32b;
                (*a_ptr).flags3 =
                    ((*a_ptr).flags3 as libc::c_long |
                         0x400000 as libc::c_long) as u32b;
                (*a_ptr).flags3 =
                    ((*a_ptr).flags3 as libc::c_long |
                         0x800000 as libc::c_long) as u32b;
                /* Needed hack */
                (*a_ptr).esp = 0 as libc::c_int as u32b;
                (*a_ptr).power = -(1 as libc::c_int) as s16b;
                /*Require activating artifacts to have a activation type */
                if !a_ptr.is_null() &&
                       (*a_ptr).flags3 as libc::c_long &
                           0x1000000 as libc::c_long != 0 &&
                       (*a_ptr).activate == 0 {
                    msg_print(b"Activate flag without activate type\x00" as
                                  *const u8 as *const libc::c_char);
                    return 1 as libc::c_int
                }
            } else {
                /* There better be a current a_ptr */
                if a_ptr.is_null() { return 3 as libc::c_int }
                /* Process 'D' for "Description" */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'D' as i32 {
                    /* Acquire the text */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Hack -- Verify space */
                    if ((*a_head).text_size as
                            libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                           > fake_text_size as libc::c_ulong {
                        return 7 as libc::c_int
                    }
                    /* Advance and Save the text index */
                    if (*a_ptr).text == 0 {
                        (*a_head).text_size =
                            (*a_head).text_size.wrapping_add(1);
                        (*a_ptr).text = (*a_head).text_size
                    } else if *a_text.offset((*a_head).text_size.wrapping_sub(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
                                                 as isize) as libc::c_int !=
                                  ' ' as i32 {
                        /* Append a space at the end of the line, if needed */
                        /* Append the space */
                        strcpy(a_text.offset((*a_head).text_size as isize),
                               b" \x00" as *const u8 as *const libc::c_char);
                        /* Advance the index */
                        (*a_head).text_size =
                            ((*a_head).text_size as
                                 libc::c_uint).wrapping_add(1 as libc::c_int
                                                                as
                                                                libc::c_uint)
                                as u32b as u32b
                    }
                    /* Append chars to the name */
                    strcpy(a_text.offset((*a_head).text_size as isize), s);
                    /* Advance the index */
                    (*a_head).text_size =
                        ((*a_head).text_size as
                             libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                            u32b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'I' as i32 {
                    let mut tval: libc::c_int = 0;
                    let mut sval: libc::c_int = 0;
                    let mut pval: libc::c_int = 0;
                    /* Process 'I' for "Info" (one line only) */
                    /* Scan for the values */
                    if 3 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut tval as *mut libc::c_int,
                                  &mut sval as *mut libc::c_int,
                                  &mut pval as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*a_ptr).tval = tval as byte_hack;
                    (*a_ptr).sval = sval as byte_hack;
                    (*a_ptr).pval = pval as s16b;
                    /* Verify */
                    if lookup_kind(tval, sval) == 0 {
                        return 6 as libc::c_int
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'W' as i32 {
                    let mut level: libc::c_int = 0;
                    let mut rarity: libc::c_int = 0;
                    let mut wgt: libc::c_int = 0;
                    let mut cost: libc::c_long = 0;
                    /* Process 'W' for "More Info" (one line only) */
                    /* Scan for the values */
                    if 4 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d:%ld\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut level as *mut libc::c_int,
                                  &mut rarity as *mut libc::c_int,
                                  &mut wgt as *mut libc::c_int,
                                  &mut cost as *mut libc::c_long) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*a_ptr).level = level as byte_hack;
                    (*a_ptr).rarity = rarity as byte_hack;
                    (*a_ptr).weight = wgt as s16b;
                    (*a_ptr).cost = cost as s32b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'P' as i32 {
                    let mut ac: libc::c_int = 0;
                    let mut hd1: libc::c_int = 0;
                    let mut hd2: libc::c_int = 0;
                    let mut th: libc::c_int = 0;
                    let mut td: libc::c_int = 0;
                    let mut ta: libc::c_int = 0;
                    /* Hack -- Process 'P' for "power" and such */
                    /* Scan for the values */
                    if 6 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%dd%d:%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut ac as *mut libc::c_int,
                                  &mut hd1 as *mut libc::c_int,
                                  &mut hd2 as *mut libc::c_int,
                                  &mut th as *mut libc::c_int,
                                  &mut td as *mut libc::c_int,
                                  &mut ta as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    (*a_ptr).ac = ac as s16b;
                    (*a_ptr).dd = hd1 as byte_hack;
                    (*a_ptr).ds = hd2 as byte_hack;
                    (*a_ptr).to_h = th as s16b;
                    (*a_ptr).to_d = td as s16b;
                    (*a_ptr).to_a = ta as s16b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'Z' as i32 {
                    let mut i_0: libc::c_int = 0;
                    /* Process 'Z' for "Granted power" */
                    /* Acquire the text */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Find it in the list */
                    i_0 = 0 as libc::c_int;
                    while i_0 < power_max as libc::c_int {
                        if strcasecmp(s,
                                      (*powers_type.offset(i_0 as
                                                               isize)).name)
                               == 0 {
                            break ;
                        }
                        i_0 += 1
                    }
                    if i_0 == power_max as libc::c_int {
                        return 6 as libc::c_int
                    }
                    (*a_ptr).power = i_0 as s16b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'F' as i32 {
                    /* Hack -- Process 'F' for flags */
                    /* Parse every entry textually */
                    s = buf.offset(2 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh38 = t;
                            t = t.offset(1);
                            *fresh38 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_artifact_flag(a_ptr, s as cptr,
                                                      0 as libc::c_int as
                                                          bool_) {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'f' as i32 {
                    /* Hack -- Process 'f' for obvious flags */
                    /* Parse every entry textually */
                    s = buf.offset(2 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh39 = t;
                            t = t.offset(1);
                            *fresh39 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_artifact_flag(a_ptr, s as cptr,
                                                      1 as libc::c_int as
                                                          bool_) {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'a' as i32 {
                    if prefix(buf.offset(2 as libc::c_int as isize) as cptr,
                              b"HARDCORE=\x00" as *const u8 as
                                  *const libc::c_char) != 0 {
                        (*a_ptr).activate =
                            get_activation(buf.offset(11 as libc::c_int as
                                                          isize)) as s16b;
                        if (*a_ptr).activate as libc::c_int ==
                               -(1 as libc::c_int) {
                            return 1 as libc::c_int
                        }
                    } else if prefix(buf.offset(2 as libc::c_int as isize) as
                                         cptr,
                                     b"SPELL=\x00" as *const u8 as
                                         *const libc::c_char) != 0 {
                        (*a_ptr).activate =
                            -find_spell(buf.offset(8 as libc::c_int as isize))
                                as s16b;
                        if (*a_ptr).activate as libc::c_int ==
                               --(1 as libc::c_int) {
                            return 1 as libc::c_int
                        }
                    }
                } else {
                    /* Read activation type. */
                    /* Oops */
                    return 6 as libc::c_int
                }
            }
        }
    }
    /* Complete the "name" and "text" sizes */
    (*a_head).name_size = (*a_head).name_size.wrapping_add(1);
    (*a_head).text_size = (*a_head).text_size.wrapping_add(1);
    /* No version yet */
    if okay == 0 { return 2 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
/*
* Initialize the "set_info" array, by parsing an ascii "template" file
*/
#[no_mangle]
pub unsafe extern "C" fn init_set_info_txt(mut fp: *mut FILE,
                                           mut buf: *mut libc::c_char)
 -> errr {
    let mut i: libc::c_int = 0;
    let mut cur_art: libc::c_int = 0 as libc::c_int;
    let mut cur_num: libc::c_int = 0 as libc::c_int;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Not ready yet */
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    /* Current entry */
    let mut set_ptr: *mut set_type = 0 as *mut set_type;
    /* Just before the first record */
    error_idx = -(1 as libc::c_int) as s16b;
    /* Just before the first line */
    error_line = -(1 as libc::c_int) as s16b;
    /* Parse */
    fp_stack_init(fp);
    /* Next... */
    while 0 as libc::c_int == my_fgets_dostack(buf, 1024 as libc::c_int) {
        /* Advance the line number */
        error_line += 1;
        /* Skip comments and blank lines */
        if *buf.offset(0 as libc::c_int as isize) == 0 ||
               *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '#' as i32 {
            continue ;
        }
        /* Verify correct "colon" format */
        if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
           {
            return 1 as libc::c_int
        }
        /* Hack -- Process 'V' for "Version" */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'V' as i32
           {
            let mut v1: libc::c_int = 0;
            let mut v2: libc::c_int = 0;
            let mut v3: libc::c_int = 0;
            /* Scan for the values */
            if 3 as libc::c_int !=
                   sscanf(buf.offset(2 as libc::c_int as isize),
                          b"%d.%d.%d\x00" as *const u8 as *const libc::c_char,
                          &mut v1 as *mut libc::c_int,
                          &mut v2 as *mut libc::c_int,
                          &mut v3 as *mut libc::c_int) {
                return 2 as libc::c_int
            }
            /* Okay to proceed */
            okay = 1 as libc::c_int as bool_
        } else {
            /* No version yet */
            if okay == 0 { return 2 as libc::c_int }
            /* Included file */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '<' as i32 {
                fp_stack_push(buf.offset(2 as libc::c_int as isize) as cptr);
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'N' as i32 {
                let mut z: libc::c_int = 0;
                let mut y: libc::c_int = 0;
                /* Process 'N' for "New/Number/Name" */
                /* Find the colon before the name */
                s = strchr(buf.offset(2 as libc::c_int as isize), ':' as i32);
                /* Verify that colon */
                if s.is_null() { return 1 as libc::c_int }
                /* Nuke the colon, advance to the name */
                let fresh40 = s;
                s = s.offset(1);
                *fresh40 = '\u{0}' as i32 as libc::c_char;
                /* Paranoia -- require a name */
                if *s == 0 { return 1 as libc::c_int }
                /* Get the index */
                i = atoi(buf.offset(2 as libc::c_int as isize));
                /* Verify information */
                if i < error_idx as libc::c_int { return 4 as libc::c_int }
                /* Verify information */
                if i >= (*set_head).info_num as libc::c_int {
                    return 2 as libc::c_int
                }
                /* Save the index */
                error_idx = i as s16b;
                /* Point at the "info" */
                set_ptr = &mut *set_info.offset(i as isize) as *mut set_type;
                /* Hack -- Verify space */
                if ((*set_head).name_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_name_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the name index */
                if (*set_ptr).name == 0 {
                    (*set_head).name_size =
                        (*set_head).name_size.wrapping_add(1);
                    (*set_ptr).name = (*set_head).name_size
                }
                /* Append chars to the name */
                strcpy(set_name.offset((*set_head).name_size as isize), s);
                /* Advance the index */
                (*set_head).name_size =
                    ((*set_head).name_size as
                         libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                        u32b;
                /* Needed hack */
                (*set_ptr).num = 0 as libc::c_int as byte_hack;
                (*set_ptr).num_use = 0 as libc::c_int as byte_hack;
                z = 0 as libc::c_int;
                while z < 6 as libc::c_int {
                    (*set_ptr).arts[z as usize].a_idx =
                        0 as libc::c_int as s16b;
                    (*set_ptr).arts[z as usize].present =
                        0 as libc::c_int as bool_;
                    y = 0 as libc::c_int;
                    while y < 6 as libc::c_int {
                        (*set_ptr).arts[z as usize].flags1[y as usize] =
                            0 as libc::c_int as u32b;
                        (*set_ptr).arts[z as usize].flags2[y as usize] =
                            0 as libc::c_int as u32b;
                        (*set_ptr).arts[z as usize].flags3[y as usize] =
                            0 as libc::c_int as u32b;
                        (*set_ptr).arts[z as usize].flags4[y as usize] =
                            0 as libc::c_int as u32b;
                        (*set_ptr).arts[z as usize].flags5[y as usize] =
                            0 as libc::c_int as u32b;
                        (*set_ptr).arts[z as usize].esp[y as usize] =
                            0 as libc::c_int as u32b;
                        (*set_ptr).arts[z as usize].pval[y as usize] =
                            0 as libc::c_int as s16b;
                        y += 1
                    }
                    z += 1
                }
            } else {
                /* There better be a current set_ptr */
                if set_ptr.is_null() { return 3 as libc::c_int }
                /* Process 'D' for "Description" */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'D' as i32 {
                    /* Acquire the text */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Hack -- Verify space */
                    if ((*set_head).text_size as
                            libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                           > fake_text_size as libc::c_ulong {
                        return 7 as libc::c_int
                    }
                    /* Advance and Save the text index */
                    if (*set_ptr).desc == 0 {
                        (*set_head).text_size =
                            (*set_head).text_size.wrapping_add(1);
                        (*set_ptr).desc = (*set_head).text_size
                    }
                    /* Append chars to the name */
                    strcpy(set_text.offset((*set_head).text_size as isize),
                           s);
                    /* Advance the index */
                    (*set_head).text_size =
                        ((*set_head).text_size as
                             libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                            u32b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'P' as i32 {
                    let mut a_idx: libc::c_int = 0;
                    let mut num: libc::c_int = 0;
                    let mut pval: libc::c_int = 0;
                    let mut z_0: libc::c_int = 0;
                    /* Process 'P' for "Power" (up to 6) */
                    /* Scan for the values */
                    if 3 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut a_idx as *mut libc::c_int,
                                  &mut num as *mut libc::c_int,
                                  &mut pval as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    z_0 = 0 as libc::c_int;
                    while z_0 < (*set_ptr).num as libc::c_int {
                        if (*set_ptr).arts[z_0 as usize].a_idx as libc::c_int
                               == a_idx {
                            break ;
                        }
                        z_0 += 1
                    }
                    if z_0 == (*set_ptr).num as libc::c_int {
                        (*set_ptr).num = (*set_ptr).num.wrapping_add(1);
                        (*set_ptr).arts[z_0 as usize].a_idx = a_idx as s16b
                    }
                    /* Save the values */
                    (*set_ptr).arts[z_0 as
                                        usize].pval[(num - 1 as libc::c_int)
                                                        as usize] =
                        pval as s16b;
                    cur_art = z_0;
                    cur_num = num - 1 as libc::c_int
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'F' as i32 {
                    /* Process 'F' for flags */
                    /* Parse every entry textually */
                    s = buf.offset(2 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh41 = t;
                            t = t.offset(1);
                            *fresh41 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_race_kind_flag(&mut *(*(*set_ptr).arts.as_mut_ptr().offset(cur_art
                                                                                                       as
                                                                                                       isize)).flags1.as_mut_ptr().offset(cur_num
                                                                                                                                              as
                                                                                                                                              isize),
                                                       &mut *(*(*set_ptr).arts.as_mut_ptr().offset(cur_art
                                                                                                       as
                                                                                                       isize)).flags2.as_mut_ptr().offset(cur_num
                                                                                                                                              as
                                                                                                                                              isize),
                                                       &mut *(*(*set_ptr).arts.as_mut_ptr().offset(cur_art
                                                                                                       as
                                                                                                       isize)).flags3.as_mut_ptr().offset(cur_num
                                                                                                                                              as
                                                                                                                                              isize),
                                                       &mut *(*(*set_ptr).arts.as_mut_ptr().offset(cur_art
                                                                                                       as
                                                                                                       isize)).flags4.as_mut_ptr().offset(cur_num
                                                                                                                                              as
                                                                                                                                              isize),
                                                       &mut *(*(*set_ptr).arts.as_mut_ptr().offset(cur_art
                                                                                                       as
                                                                                                       isize)).flags5.as_mut_ptr().offset(cur_num
                                                                                                                                              as
                                                                                                                                              isize),
                                                       &mut *(*(*set_ptr).arts.as_mut_ptr().offset(cur_art
                                                                                                       as
                                                                                                       isize)).esp.as_mut_ptr().offset(cur_num
                                                                                                                                           as
                                                                                                                                           isize),
                                                       s as cptr) {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else {
                    /* Oops */
                    return 6 as libc::c_int
                }
            }
        }
    }
    /* Complete the "name" and "text" sizes */
    (*set_head).name_size = (*set_head).name_size.wrapping_add(1);
    (*set_head).text_size = (*set_head).text_size.wrapping_add(1);
    /* No version yet */
    if okay == 0 { return 2 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialize the "s_info" array, by parsing an ascii "template" file
 */
#[no_mangle]
pub unsafe extern "C" fn init_s_info_txt(mut fp: *mut FILE,
                                         mut buf: *mut libc::c_char) -> errr {
    let mut i: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut order: libc::c_int = 1 as libc::c_int;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Not ready yet */
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    /* Current entry */
    let mut s_ptr: *mut skill_type = 0 as *mut skill_type;
    /* Just before the first record */
    error_idx = -(1 as libc::c_int) as s16b;
    /* Just before the first line */
    error_line = -(1 as libc::c_int) as s16b;
    /* Parse */
    fp_stack_init(fp);
    /* Next... */
    while 0 as libc::c_int == my_fgets_dostack(buf, 1024 as libc::c_int) {
        /* Advance the line number */
        error_line += 1;
        /* Skip comments and blank lines */
        if *buf.offset(0 as libc::c_int as isize) == 0 ||
               *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '#' as i32 {
            continue ;
        }
        /* Verify correct "colon" format */
        if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
           {
            return 1 as libc::c_int
        }
        /* Hack -- Process 'V' for "Version" */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'V' as i32
           {
            let mut v1: libc::c_int = 0;
            let mut v2: libc::c_int = 0;
            let mut v3: libc::c_int = 0;
            /* Scan for the values */
            if 3 as libc::c_int !=
                   sscanf(buf.offset(2 as libc::c_int as isize),
                          b"%d.%d.%d\x00" as *const u8 as *const libc::c_char,
                          &mut v1 as *mut libc::c_int,
                          &mut v2 as *mut libc::c_int,
                          &mut v3 as *mut libc::c_int) {
                return 2 as libc::c_int
            }
            /* Okay to proceed */
            okay = 1 as libc::c_int as bool_
        } else {
            /* No version yet */
            if okay == 0 { return 2 as libc::c_int }
            /* Included file */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '<' as i32 {
                fp_stack_push(buf.offset(2 as libc::c_int as isize) as cptr);
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'T' as i32 {
                let mut sec: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut s1: s16b = 0;
                let mut s2: s16b = 0;
                /* Process 'T' for "skill Tree" */
                /* Scan for the values */
                sec =
                    strchr(buf.offset(2 as libc::c_int as isize), ':' as i32);
                if sec.is_null() { return 1 as libc::c_int }
                *sec = '\u{0}' as i32 as libc::c_char;
                sec = sec.offset(1);
                if *sec == 0 { return 1 as libc::c_int }
                s1 =
                    find_skill(buf.offset(2 as libc::c_int as isize) as cptr);
                s2 = find_skill(sec as cptr);
                if s2 as libc::c_int == -(1 as libc::c_int) {
                    return 1 as libc::c_int
                }
                (*s_info.offset(s2 as isize)).father = s1;
                let fresh42 = order;
                order = order + 1;
                (*s_info.offset(s2 as isize)).order = fresh42 as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'E' as i32 {
                let mut sec_0: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut s1_0: s16b = 0;
                let mut s2_0: s16b = 0;
                /* Process 'E' for "Exclusive" */
                /* Scan for the values */
                sec_0 =
                    strchr(buf.offset(2 as libc::c_int as isize), ':' as i32);
                if sec_0.is_null() { return 1 as libc::c_int }
                *sec_0 = '\u{0}' as i32 as libc::c_char;
                sec_0 = sec_0.offset(1);
                if *sec_0 == 0 { return 1 as libc::c_int }
                s1_0 =
                    find_skill(buf.offset(2 as libc::c_int as isize) as cptr);
                s2_0 = find_skill(sec_0 as cptr);
                if s1_0 as libc::c_int == -(1 as libc::c_int) ||
                       s2_0 as libc::c_int == -(1 as libc::c_int) {
                    return 1 as libc::c_int
                }
                (*s_info.offset(s1_0 as isize)).action[s2_0 as usize] =
                    9999 as libc::c_int as s16b;
                (*s_info.offset(s2_0 as isize)).action[s1_0 as usize] =
                    9999 as libc::c_int as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'O' as i32 {
                let mut sec_1: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut cval: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut s1_1: s16b = 0;
                let mut s2_1: s16b = 0;
                /* Process 'O' for "Opposite" */
                /* Scan for the values */
                sec_1 =
                    strchr(buf.offset(2 as libc::c_int as isize), ':' as i32);
                if sec_1.is_null() { return 1 as libc::c_int }
                *sec_1 = '\u{0}' as i32 as libc::c_char;
                sec_1 = sec_1.offset(1);
                if *sec_1 == 0 { return 1 as libc::c_int }
                cval = strchr(sec_1, '%' as i32);
                if cval.is_null() { return 1 as libc::c_int }
                *cval = '\u{0}' as i32 as libc::c_char;
                cval = cval.offset(1);
                if *cval == 0 { return 1 as libc::c_int }
                s1_1 =
                    find_skill(buf.offset(2 as libc::c_int as isize) as cptr);
                s2_1 = find_skill(sec_1 as cptr);
                if s1_1 as libc::c_int == -(1 as libc::c_int) ||
                       s2_1 as libc::c_int == -(1 as libc::c_int) {
                    return 1 as libc::c_int
                }
                (*s_info.offset(s1_1 as isize)).action[s2_1 as usize] =
                    -atoi(cval) as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'f' as i32 {
                let mut sec_2: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut cval_0: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut s1_2: s16b = 0;
                let mut s2_2: s16b = 0;
                /* Process 'A' for "Amical/friendly" */
                /* Scan for the values */
                sec_2 =
                    strchr(buf.offset(2 as libc::c_int as isize), ':' as i32);
                if sec_2.is_null() { return 1 as libc::c_int }
                *sec_2 = '\u{0}' as i32 as libc::c_char;
                sec_2 = sec_2.offset(1);
                if *sec_2 == 0 { return 1 as libc::c_int }
                cval_0 = strchr(sec_2, '%' as i32);
                if cval_0.is_null() { return 1 as libc::c_int }
                *cval_0 = '\u{0}' as i32 as libc::c_char;
                cval_0 = cval_0.offset(1);
                if *cval_0 == 0 { return 1 as libc::c_int }
                s1_2 =
                    find_skill(buf.offset(2 as libc::c_int as isize) as cptr);
                s2_2 = find_skill(sec_2 as cptr);
                if s1_2 as libc::c_int == -(1 as libc::c_int) ||
                       s2_2 as libc::c_int == -(1 as libc::c_int) {
                    return 1 as libc::c_int
                }
                (*s_info.offset(s1_2 as isize)).action[s2_2 as usize] =
                    atoi(cval_0) as s16b
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'N' as i32 {
                /* Process 'N' for "New/Number/Name" */
                /* Find the colon before the name */
                s = strchr(buf.offset(2 as libc::c_int as isize), ':' as i32);
                /* Verify that colon */
                if s.is_null() { return 1 as libc::c_int }
                /* Nuke the colon, advance to the name */
                let fresh43 = s;
                s = s.offset(1);
                *fresh43 = '\u{0}' as i32 as libc::c_char;
                /* Paranoia -- require a name */
                if *s == 0 { return 1 as libc::c_int }
                /* Get the index */
                i = atoi(buf.offset(2 as libc::c_int as isize));
                /* Verify information */
                if i >= (*s_head).info_num as libc::c_int {
                    return 2 as libc::c_int
                }
                /* Save the index */
                error_idx = i as s16b;
                /* Point at the "info" */
                s_ptr = &mut *s_info.offset(i as isize) as *mut skill_type;
                /* Hack -- Verify space */
                if ((*s_head).name_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_name_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the name index */
                if (*s_ptr).name == 0 {
                    (*s_head).name_size = (*s_head).name_size.wrapping_add(1);
                    (*s_ptr).name = (*s_head).name_size
                }
                /* Append chars to the name */
                strcpy(s_name.offset((*s_head).name_size as isize), s);
                /* Advance the index */
                (*s_head).name_size =
                    ((*s_head).name_size as
                         libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                        u32b;
                /* Init */
                (*s_ptr).action_mkey = 0 as libc::c_int as s16b;
                (*s_ptr).dev = 0 as libc::c_int as bool_;
                (*s_ptr).random_gain_chance = 100 as libc::c_int as byte_hack;
                z = 0 as libc::c_int;
                while z < max_s_idx as libc::c_int {
                    (*s_ptr).action[z as usize] = 0 as libc::c_int as s16b;
                    z += 1
                }
            } else {
                /* There better be a current s_ptr */
                if s_ptr.is_null() { return 3 as libc::c_int }
                /* Process 'D' for "Description" */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'D' as i32 {
                    /* Acquire the text */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Hack -- Verify space */
                    if ((*s_head).text_size as
                            libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                           > fake_text_size as libc::c_ulong {
                        return 7 as libc::c_int
                    }
                    /* Advance and Save the text index */
                    if (*s_ptr).desc == 0 {
                        (*s_head).text_size =
                            (*s_head).text_size.wrapping_add(1);
                        (*s_ptr).desc = (*s_head).text_size;
                        /* Append chars to the name */
                        strcpy(s_text.offset((*s_head).text_size as isize),
                               s);
                        /* Advance the index */
                        (*s_head).text_size =
                            ((*s_head).text_size as
                                 libc::c_ulong).wrapping_add(strlen(s)) as
                                u32b as u32b
                    } else {
                        /* Append chars to the name */
                        strcpy(s_text.offset((*s_head).text_size as isize),
                               format(b"\n%s\x00" as *const u8 as
                                          *const libc::c_char, s));
                        /* Advance the index */
                        (*s_head).text_size =
                            ((*s_head).text_size as
                                 libc::c_ulong).wrapping_add(strlen(s).wrapping_add(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong))
                                as u32b as u32b
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'A' as i32 {
                    let mut txt: *mut libc::c_char = 0 as *mut libc::c_char;
                    /* Process 'A' for "Activation Description" */
                    /* Acquire the text */
                    s = buf.offset(2 as libc::c_int as isize);
                    txt = strchr(s, ':' as i32);
                    if txt.is_null() { return 1 as libc::c_int }
                    *txt = '\u{0}' as i32 as libc::c_char;
                    txt = txt.offset(1);
                    /* Hack -- Verify space */
                    if ((*s_head).text_size as
                            libc::c_ulong).wrapping_add(strlen(txt)).wrapping_add(8
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong)
                           > fake_text_size as libc::c_ulong {
                        return 7 as libc::c_int
                    }
                    /* Advance and Save the text index */
                    if (*s_ptr).action_desc == 0 {
                        (*s_head).text_size =
                            (*s_head).text_size.wrapping_add(1);
                        (*s_ptr).action_desc = (*s_head).text_size
                    }
                    /* Append chars to the name */
                    strcpy(s_text.offset((*s_head).text_size as isize), txt);
                    (*s_ptr).action_mkey = atoi(s) as s16b;
                    /* Advance the index */
                    (*s_head).text_size =
                        ((*s_head).text_size as
                             libc::c_ulong).wrapping_add(strlen(txt)) as u32b
                            as u32b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'I' as i32 {
                    let mut rate: libc::c_int = 0;
                    /* Process 'I' for "Info" (one line only) */
                    /* Scan for the values */
                    if 1 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut rate as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*s_ptr).rate = rate as s16b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'G' as i32 {
                    let mut chance: libc::c_int = 0;
                    /* Process 'G' for "random Gain" (one line only) */
                    /* Scan for the values */
                    if 1 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut chance as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*s_ptr).random_gain_chance = chance as byte_hack
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'F' as i32 {
                    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
                    /* Process 'F' for flags */
                    /* Parse every entry textually */
                    s = buf.offset(2 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh44 = t;
                            t = t.offset(1);
                            *fresh44 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_skill_flag(&mut (*s_ptr).flags1,
                                                   s as cptr) {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else {
                    /* Oops */
                    return 6 as libc::c_int
                }
            }
        }
    }
    /* Complete the "name" and "text" sizes */
    (*s_head).name_size = (*s_head).name_size.wrapping_add(1);
    (*s_head).text_size = (*s_head).text_size.wrapping_add(1);
    /* No version yet */
    if okay == 0 { return 2 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialize the "ab_info" array, by parsing an ascii "template" file
 */
#[no_mangle]
pub unsafe extern "C" fn init_ab_info_txt(mut fp: *mut FILE,
                                          mut buf: *mut libc::c_char)
 -> errr {
    let mut i: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Not ready yet */
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    /* Current entry */
    let mut ab_ptr: *mut ability_type = 0 as *mut ability_type;
    /* Just before the first record */
    error_idx = -(1 as libc::c_int) as s16b;
    /* Just before the first line */
    error_line = -(1 as libc::c_int) as s16b;
    /* Parse */
    fp_stack_init(fp);
    /* Next... */
    while 0 as libc::c_int == my_fgets_dostack(buf, 1024 as libc::c_int) {
        /* Advance the line number */
        error_line += 1;
        /* Skip comments and blank lines */
        if *buf.offset(0 as libc::c_int as isize) == 0 ||
               *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '#' as i32 {
            continue ;
        }
        /* Verify correct "colon" format */
        if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
           {
            return 1 as libc::c_int
        }
        /* Hack -- Process 'V' for "Version" */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'V' as i32
           {
            let mut v1: libc::c_int = 0;
            let mut v2: libc::c_int = 0;
            let mut v3: libc::c_int = 0;
            /* Scan for the values */
            if 3 as libc::c_int !=
                   sscanf(buf.offset(2 as libc::c_int as isize),
                          b"%d.%d.%d\x00" as *const u8 as *const libc::c_char,
                          &mut v1 as *mut libc::c_int,
                          &mut v2 as *mut libc::c_int,
                          &mut v3 as *mut libc::c_int) {
                return 2 as libc::c_int
            }
            /* Okay to proceed */
            okay = 1 as libc::c_int as bool_
        } else {
            /* No version yet */
            if okay == 0 { return 2 as libc::c_int }
            /* Included file */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '<' as i32 {
                fp_stack_push(buf.offset(2 as libc::c_int as isize) as cptr);
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'N' as i32 {
                /* Process 'N' for "New/Number/Name" */
                /* Find the colon before the name */
                s = strchr(buf.offset(2 as libc::c_int as isize), ':' as i32);
                /* Verify that colon */
                if s.is_null() { return 1 as libc::c_int }
                /* Nuke the colon, advance to the name */
                let fresh45 = s;
                s = s.offset(1);
                *fresh45 = '\u{0}' as i32 as libc::c_char;
                /* Paranoia -- require a name */
                if *s == 0 { return 1 as libc::c_int }
                /* Get the index */
                i = atoi(buf.offset(2 as libc::c_int as isize));
                /* Verify information */
                if i >= (*ab_head).info_num as libc::c_int {
                    return 2 as libc::c_int
                }
                /* Save the index */
                error_idx = i as s16b;
                /* Point at the "info" */
                ab_ptr =
                    &mut *ab_info.offset(i as isize) as *mut ability_type;
                /* Hack -- Verify space */
                if ((*ab_head).name_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_name_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the name index */
                if (*ab_ptr).name == 0 {
                    (*ab_head).name_size =
                        (*ab_head).name_size.wrapping_add(1);
                    (*ab_ptr).name = (*ab_head).name_size
                }
                /* Append chars to the name */
                strcpy(ab_name.offset((*ab_head).name_size as isize), s);
                /* Advance the index */
                (*ab_head).name_size =
                    ((*ab_head).name_size as
                         libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                        u32b;
                /* Init */
                (*ab_ptr).action_mkey = 0 as libc::c_int as s16b;
                (*ab_ptr).acquired = 0 as libc::c_int as bool_;
                z = 0 as libc::c_int;
                while z < 10 as libc::c_int {
                    (*ab_ptr).skills[z as usize] =
                        -(1 as libc::c_int) as s16b;
                    (*ab_ptr).need_abilities[z as usize] =
                        -(1 as libc::c_int) as s16b;
                    (*ab_ptr).forbid_abilities[z as usize] =
                        -(1 as libc::c_int) as s16b;
                    z += 1
                }
                z = 0 as libc::c_int;
                while z < 6 as libc::c_int {
                    (*ab_ptr).stat[z as usize] = -(1 as libc::c_int) as s16b;
                    z += 1
                }
            } else {
                /* There better be a current ab_ptr */
                if ab_ptr.is_null() { return 3 as libc::c_int }
                /* Process 'D' for "Description" */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'D' as i32 {
                    /* Acquire the text */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Hack -- Verify space */
                    if ((*ab_head).text_size as
                            libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                           > fake_text_size as libc::c_ulong {
                        return 7 as libc::c_int
                    }
                    /* Advance and Save the text index */
                    if (*ab_ptr).desc == 0 {
                        (*ab_head).text_size =
                            (*ab_head).text_size.wrapping_add(1);
                        (*ab_ptr).desc = (*ab_head).text_size;
                        /* Append chars to the name */
                        strcpy(ab_text.offset((*ab_head).text_size as isize),
                               s);
                        /* Advance the index */
                        (*ab_head).text_size =
                            ((*ab_head).text_size as
                                 libc::c_ulong).wrapping_add(strlen(s)) as
                                u32b as u32b
                    } else {
                        /* Append chars to the name */
                        strcpy(ab_text.offset((*ab_head).text_size as isize),
                               format(b"\n%s\x00" as *const u8 as
                                          *const libc::c_char, s));
                        /* Advance the index */
                        (*ab_head).text_size =
                            ((*ab_head).text_size as
                                 libc::c_ulong).wrapping_add(strlen(s).wrapping_add(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong))
                                as u32b as u32b
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'A' as i32 {
                    let mut txt: *mut libc::c_char = 0 as *mut libc::c_char;
                    /* Process 'A' for "Activation Description" */
                    /* Acquire the text */
                    s = buf.offset(2 as libc::c_int as isize);
                    txt = strchr(s, ':' as i32);
                    if txt.is_null() { return 1 as libc::c_int }
                    *txt = '\u{0}' as i32 as libc::c_char;
                    txt = txt.offset(1);
                    /* Hack -- Verify space */
                    if ((*ab_head).text_size as
                            libc::c_ulong).wrapping_add(strlen(txt)).wrapping_add(8
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong)
                           > fake_text_size as libc::c_ulong {
                        return 7 as libc::c_int
                    }
                    /* Advance and Save the text index */
                    if (*ab_ptr).action_desc == 0 {
                        (*ab_head).text_size =
                            (*ab_head).text_size.wrapping_add(1);
                        (*ab_ptr).action_desc = (*ab_head).text_size
                    }
                    /* Append chars to the name */
                    strcpy(ab_text.offset((*ab_head).text_size as isize),
                           txt);
                    (*ab_ptr).action_mkey = atoi(s) as s16b;
                    /* Advance the index */
                    (*ab_head).text_size =
                        ((*ab_head).text_size as
                             libc::c_ulong).wrapping_add(strlen(txt)) as u32b
                            as u32b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'I' as i32 {
                    let mut cost: libc::c_int = 0;
                    /* Process 'I' for "Info" (one line only) */
                    /* Scan for the values */
                    if 1 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut cost as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*ab_ptr).cost = cost as s16b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'k' as i32 {
                    let mut sec: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut level: s16b = 0;
                    let mut skill: s16b = 0;
                    /* Process 'k' for "Skill" */
                    /* Scan for the values */
                    sec =
                        strchr(buf.offset(2 as libc::c_int as isize),
                               ':' as i32);
                    if sec.is_null() { return 1 as libc::c_int }
                    *sec = '\u{0}' as i32 as libc::c_char;
                    sec = sec.offset(1);
                    if *sec == 0 { return 1 as libc::c_int }
                    level =
                        atoi(buf.offset(2 as libc::c_int as isize)) as s16b;
                    skill = find_skill(sec as cptr);
                    if skill as libc::c_int == -(1 as libc::c_int) {
                        return 1 as libc::c_int
                    }
                    z = 0 as libc::c_int;
                    while z < 10 as libc::c_int {
                        if (*ab_ptr).skills[z as usize] as libc::c_int ==
                               -(1 as libc::c_int) {
                            break ;
                        }
                        z += 1
                    }
                    if z < 10 as libc::c_int {
                        (*ab_ptr).skills[z as usize] = skill;
                        (*ab_ptr).skill_levels[z as usize] = level
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'a' as i32 {
                    let mut ab: s16b = 0;
                    ab =
                        find_ability(buf.offset(2 as libc::c_int as isize) as
                                         cptr);
                    if ab as libc::c_int == -(1 as libc::c_int) {
                        return 1 as libc::c_int
                    }
                    z = 0 as libc::c_int;
                    while z < 10 as libc::c_int {
                        if (*ab_ptr).need_abilities[z as usize] as libc::c_int
                               == -(1 as libc::c_int) {
                            break ;
                        }
                        z += 1
                    }
                    if z < 10 as libc::c_int {
                        (*ab_ptr).need_abilities[z as usize] = ab
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'S' as i32 {
                    let mut sec_0: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut stat: s16b = 0;
                    /* Process 'a' for "needed ability" */
                    /* Process 'S' for "Stat" */
                    /* Scan for the values */
                    sec_0 =
                        strchr(buf.offset(2 as libc::c_int as isize),
                               ':' as i32);
                    if sec_0.is_null() { return 1 as libc::c_int }
                    *sec_0 = '\u{0}' as i32 as libc::c_char;
                    sec_0 = sec_0.offset(1);
                    if *sec_0 == 0 { return 1 as libc::c_int }
                    stat = 0 as libc::c_int as s16b;
                    while (stat as libc::c_int) < 6 as libc::c_int {
                        if strcmp(stat_names[stat as usize], sec_0) == 0 {
                            break ;
                        }
                        stat += 1
                    }
                    if stat as libc::c_int == 6 as libc::c_int {
                        return 1 as libc::c_int
                    }
                    (*ab_ptr).stat[stat as usize] =
                        atoi(buf.offset(2 as libc::c_int as isize)) as s16b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'E' as i32 {
                    let mut sec_1: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut ab1: s16b = 0;
                    let mut ab2: s16b = 0;
                    /* Process 'E' for "Excluding ability" */
                    /* Scan for the values */
                    sec_1 =
                        strchr(buf.offset(2 as libc::c_int as isize),
                               ':' as i32);
                    if sec_1.is_null() { return 1 as libc::c_int }
                    *sec_1 = '\u{0}' as i32 as libc::c_char;
                    sec_1 = sec_1.offset(1);
                    if *sec_1 == 0 { return 1 as libc::c_int }
                    ab1 =
                        find_ability(buf.offset(2 as libc::c_int as isize) as
                                         cptr);
                    ab2 = find_ability(sec_1 as cptr);
                    if ab1 as libc::c_int == -(1 as libc::c_int) ||
                           ab2 as libc::c_int == -(1 as libc::c_int) {
                        return 1 as libc::c_int
                    }
                    z = 0 as libc::c_int;
                    while z < 10 as libc::c_int {
                        if (*ab_info.offset(ab1 as
                                                isize)).forbid_abilities[z as
                                                                             usize]
                               as libc::c_int == -(1 as libc::c_int) {
                            break ;
                        }
                        z += 1
                    }
                    if z < 10 as libc::c_int {
                        (*ab_info.offset(ab1 as
                                             isize)).forbid_abilities[z as
                                                                          usize]
                            = ab2
                    }
                    z = 0 as libc::c_int;
                    while z < 10 as libc::c_int {
                        if (*ab_info.offset(ab2 as
                                                isize)).forbid_abilities[z as
                                                                             usize]
                               as libc::c_int == -(1 as libc::c_int) {
                            break ;
                        }
                        z += 1
                    }
                    if z < 10 as libc::c_int {
                        (*ab_info.offset(ab2 as
                                             isize)).forbid_abilities[z as
                                                                          usize]
                            = ab1
                    }
                } else {
                    /* Oops */
                    return 6 as libc::c_int
                }
            }
        }
    }
    /* Complete the "name" and "text" sizes */
    (*ab_head).name_size = (*ab_head).name_size.wrapping_add(1);
    (*ab_head).text_size = (*ab_head).text_size.wrapping_add(1);
    /* No version yet */
    if okay == 0 { return 2 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Grab one flag in a ego-item_type from a textual string
 */
unsafe extern "C" fn grab_one_ego_item_flag(mut e_ptr: *mut ego_item_type,
                                            mut what: cptr,
                                            mut n: libc::c_int,
                                            mut obvious: bool_) -> bool_ {
    let mut i: libc::c_int = 0;
    /* Check flags1 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags1[i as usize]) != 0 {
            if obvious != 0 {
                (*e_ptr).oflags1[n as usize] =
                    ((*e_ptr).oflags1[n as usize] as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*e_ptr).flags1[n as usize] =
                    ((*e_ptr).flags1[n as usize] as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Check flags2 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags2[i as usize]) != 0 {
            if obvious != 0 {
                (*e_ptr).oflags2[n as usize] =
                    ((*e_ptr).oflags2[n as usize] as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*e_ptr).flags2[n as usize] =
                    ((*e_ptr).flags2[n as usize] as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Check flags2 -- traps */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags2_trap[i as usize]) != 0 {
            if obvious != 0 {
                (*e_ptr).oflags2[n as usize] =
                    ((*e_ptr).oflags2[n as usize] as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*e_ptr).flags2[n as usize] =
                    ((*e_ptr).flags2[n as usize] as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Check flags3 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags3[i as usize]) != 0 {
            if obvious != 0 {
                (*e_ptr).oflags3[n as usize] =
                    ((*e_ptr).oflags3[n as usize] as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*e_ptr).flags3[n as usize] =
                    ((*e_ptr).flags3[n as usize] as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Check flags4 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags4[i as usize]) != 0 {
            if obvious != 0 {
                (*e_ptr).oflags4[n as usize] =
                    ((*e_ptr).oflags4[n as usize] as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*e_ptr).flags4[n as usize] =
                    ((*e_ptr).flags4[n as usize] as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Check flags5 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags5[i as usize]) != 0 {
            if obvious != 0 {
                (*e_ptr).oflags5[n as usize] =
                    ((*e_ptr).oflags5[n as usize] as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*e_ptr).flags5[n as usize] =
                    ((*e_ptr).flags5[n as usize] as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Check esp_flags */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, esp_flags[i as usize]) != 0 {
            if obvious != 0 {
                (*e_ptr).oesp[n as usize] =
                    ((*e_ptr).oesp[n as usize] as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*e_ptr).esp[n as usize] =
                    ((*e_ptr).esp[n as usize] as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Check ego_flags */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, ego_flags[i as usize]) != 0 {
            (*e_ptr).fego[n as usize] =
                ((*e_ptr).fego[n as usize] as libc::c_long |
                     (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Oops */
    msg_format(b"Unknown ego-item flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Error */
    return 1 as libc::c_int as bool_;
}
unsafe extern "C" fn grab_one_ego_item_flag_restrict(mut e_ptr:
                                                         *mut ego_item_type,
                                                     mut what: cptr,
                                                     mut need: bool_)
 -> bool_ {
    let mut i: libc::c_int = 0;
    /* Check flags1 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags1[i as usize]) != 0 {
            if need != 0 {
                (*e_ptr).need_flags1 =
                    ((*e_ptr).need_flags1 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*e_ptr).forbid_flags1 =
                    ((*e_ptr).forbid_flags1 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Check flags2 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags2[i as usize]) != 0 {
            if need != 0 {
                (*e_ptr).need_flags2 =
                    ((*e_ptr).need_flags2 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*e_ptr).forbid_flags2 =
                    ((*e_ptr).forbid_flags2 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Check flags2 -- traps */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags2_trap[i as usize]) != 0 {
            if need != 0 {
                (*e_ptr).need_flags2 =
                    ((*e_ptr).need_flags2 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*e_ptr).forbid_flags2 =
                    ((*e_ptr).forbid_flags2 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Check flags3 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags3[i as usize]) != 0 {
            if need != 0 {
                (*e_ptr).need_flags3 =
                    ((*e_ptr).need_flags3 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*e_ptr).forbid_flags3 =
                    ((*e_ptr).forbid_flags3 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Check flags4 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags4[i as usize]) != 0 {
            if need != 0 {
                (*e_ptr).need_flags4 =
                    ((*e_ptr).need_flags4 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*e_ptr).forbid_flags4 =
                    ((*e_ptr).forbid_flags4 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Check flags5 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags5[i as usize]) != 0 {
            if need != 0 {
                (*e_ptr).need_flags5 =
                    ((*e_ptr).need_flags5 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*e_ptr).forbid_flags5 =
                    ((*e_ptr).forbid_flags5 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Check esp_flags */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, esp_flags[i as usize]) != 0 {
            if need != 0 {
                (*e_ptr).need_esp =
                    ((*e_ptr).need_esp as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*e_ptr).forbid_esp =
                    ((*e_ptr).forbid_esp as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Oops */
    msg_format(b"Unknown ego-item restrict flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Error */
    return 1 as libc::c_int as bool_;
}
/*
 * Initialize the "e_info" array, by parsing an ascii "template" file
 */
#[no_mangle]
pub unsafe extern "C" fn init_e_info_txt(mut fp: *mut FILE,
                                         mut buf: *mut libc::c_char) -> errr {
    let mut i: libc::c_int = 0;
    let mut cur_r: libc::c_int = -(1 as libc::c_int);
    let mut cur_t: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Not ready yet */
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    /* Current entry */
    let mut e_ptr: *mut ego_item_type = 0 as *mut ego_item_type;
    /* Just before the first record */
    error_idx = -(1 as libc::c_int) as s16b;
    /* Just before the first line */
    error_line = -(1 as libc::c_int) as s16b;
    /* Parse */
    fp_stack_init(fp);
    /* Next... */
    while 0 as libc::c_int == my_fgets_dostack(buf, 1024 as libc::c_int) {
        /* Advance the line number */
        error_line += 1;
        /* Skip comments and blank lines */
        if *buf.offset(0 as libc::c_int as isize) == 0 ||
               *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '#' as i32 {
            continue ;
        }
        /* Verify correct "colon" format */
        if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
           {
            return 1 as libc::c_int
        }
        /* Hack -- Process 'V' for "Version" */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'V' as i32
           {
            let mut v1: libc::c_int = 0;
            let mut v2: libc::c_int = 0;
            let mut v3: libc::c_int = 0;
            /* Scan for the values */
            if 3 as libc::c_int !=
                   sscanf(buf.offset(2 as libc::c_int as isize),
                          b"%d.%d.%d\x00" as *const u8 as *const libc::c_char,
                          &mut v1 as *mut libc::c_int,
                          &mut v2 as *mut libc::c_int,
                          &mut v3 as *mut libc::c_int) {
                return 2 as libc::c_int
            }
            /* Okay to proceed */
            okay = 1 as libc::c_int as bool_
        } else {
            /* No version yet */
            if okay == 0 { return 2 as libc::c_int }
            /* Included file */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '<' as i32 {
                fp_stack_push(buf.offset(2 as libc::c_int as isize) as cptr);
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'N' as i32 {
                /* Process 'N' for "New/Number/Name" */
                /* Find the colon before the name */
                s = strchr(buf.offset(2 as libc::c_int as isize), ':' as i32);
                /* Verify that colon */
                if s.is_null() { return 1 as libc::c_int }
                /* Nuke the colon, advance to the name */
                let fresh46 = s;
                s = s.offset(1);
                *fresh46 = '\u{0}' as i32 as libc::c_char;
                /* Paranoia -- require a name */
                if *s == 0 { return 1 as libc::c_int }
                /* Get the index */
                i = atoi(buf.offset(2 as libc::c_int as isize));
                /* Verify information */
                if i < error_idx as libc::c_int { return 4 as libc::c_int }
                /* Verify information */
                if i >= (*e_head).info_num as libc::c_int {
                    return 2 as libc::c_int
                }
                /* Save the index */
                error_idx = i as s16b;
                /* Point at the "info" */
                e_ptr = &mut *e_info.offset(i as isize) as *mut ego_item_type;
                /* Hack -- Verify space */
                if ((*e_head).name_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_name_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the name index */
                if (*e_ptr).name == 0 {
                    (*e_head).name_size = (*e_head).name_size.wrapping_add(1);
                    (*e_ptr).name = (*e_head).name_size
                }
                /* Append chars to the name */
                strcpy(e_name.offset((*e_head).name_size as isize), s);
                /* Advance the index */
                (*e_head).name_size =
                    ((*e_head).name_size as
                         libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                        u32b;
                /* Needed hack */
                (*e_ptr).power = -(1 as libc::c_int) as s16b;
                cur_r = -(1 as libc::c_int);
                cur_t = 0 as libc::c_int;
                j = 0 as libc::c_int;
                while j < 10 as libc::c_int {
                    (*e_ptr).tval[j as usize] =
                        255 as libc::c_int as byte_hack;
                    j += 1
                }
                j = 0 as libc::c_int;
                while j < 5 as libc::c_int {
                    (*e_ptr).rar[j as usize] = 0 as libc::c_int as byte_hack;
                    (*e_ptr).flags1[j as usize] = 0 as libc::c_int as u32b;
                    (*e_ptr).flags2[j as usize] = 0 as libc::c_int as u32b;
                    (*e_ptr).flags3[j as usize] = 0 as libc::c_int as u32b;
                    (*e_ptr).flags4[j as usize] = 0 as libc::c_int as u32b;
                    (*e_ptr).flags5[j as usize] = 0 as libc::c_int as u32b;
                    (*e_ptr).esp[j as usize] = 0 as libc::c_int as u32b;
                    j += 1
                }
            } else {
                /* There better be a current e_ptr */
                if e_ptr.is_null() { return 3 as libc::c_int }
                /* Process 'T' for "Tval/Sval" (up to 5 lines) */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'T' as i32 {
                    let mut tv: libc::c_int = 0;
                    let mut minsv: libc::c_int = 0;
                    let mut maxsv: libc::c_int = 0;
                    if cur_t == 10 as libc::c_int { return 1 as libc::c_int }
                    /* Scan for the values */
                    if 3 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut tv as *mut libc::c_int,
                                  &mut minsv as *mut libc::c_int,
                                  &mut maxsv as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*e_ptr).tval[cur_t as usize] = tv as byte_hack;
                    (*e_ptr).min_sval[cur_t as usize] = minsv as byte_hack;
                    (*e_ptr).max_sval[cur_t as usize] = maxsv as byte_hack;
                    cur_t += 1
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'R' as i32 {
                    let mut rar: libc::c_int = 0;
                    if cur_r == 5 as libc::c_int { return 1 as libc::c_int }
                    /* Process 'R' for "flags rarity" (up to 5 lines) */
                    /* Scan for the values */
                    if 1 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut rar as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    cur_r += 1;
                    /* Save the values */
                    (*e_ptr).rar[cur_r as usize] = rar as byte_hack
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'X' as i32 {
                    let mut slot: libc::c_int = 0;
                    let mut rating: libc::c_int = 0;
                    let mut pos: libc::c_char = 0;
                    /* Process 'X' for "Xtra" (one line only) */
                    /* Scan for the values */
                    if 3 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%c:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut pos as *mut libc::c_char,
                                  &mut slot as *mut libc::c_int,
                                  &mut rating as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
			/* e_ptr->slot = slot; */
                    (*e_ptr).rating = rating as byte_hack;
                    (*e_ptr).before =
                        if pos as libc::c_int == 'B' as i32 {
                            1 as libc::c_int
                        } else { 0 as libc::c_int } as bool_
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'W' as i32 {
                    let mut level: libc::c_int = 0;
                    let mut rarity: libc::c_int = 0;
                    let mut rarity2: libc::c_int = 0;
                    let mut cost: libc::c_long = 0;
                    /* Process 'W' for "More Info" (one line only) */
                    /* Scan for the values */
                    if 4 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d:%ld\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut level as *mut libc::c_int,
                                  &mut rarity as *mut libc::c_int,
                                  &mut rarity2 as *mut libc::c_int,
                                  &mut cost as *mut libc::c_long) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*e_ptr).level = level as byte_hack;
                    (*e_ptr).rarity = rarity as byte_hack;
                    (*e_ptr).mrarity = rarity2 as byte_hack;
                    (*e_ptr).cost = cost as s32b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'C' as i32 {
                    let mut th: libc::c_int = 0;
                    let mut td: libc::c_int = 0;
                    let mut ta: libc::c_int = 0;
                    let mut pv: libc::c_int = 0;
                    /* Hack -- Process 'C' for "creation" */
                    /* Scan for the values */
                    if 4 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut th as *mut libc::c_int,
                                  &mut td as *mut libc::c_int,
                                  &mut ta as *mut libc::c_int,
                                  &mut pv as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    (*e_ptr).max_to_h = th as s16b;
                    (*e_ptr).max_to_d = td as s16b;
                    (*e_ptr).max_to_a = ta as s16b;
                    (*e_ptr).max_pval = pv
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'Z' as i32 {
                    let mut i_0: libc::c_int = 0;
                    /* Process 'Z' for "Granted power" */
                    /* Acquire the text */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Find it in the list */
                    i_0 = 0 as libc::c_int;
                    while i_0 < power_max as libc::c_int {
                        if strcasecmp(s,
                                      (*powers_type.offset(i_0 as
                                                               isize)).name)
                               == 0 {
                            break ;
                        }
                        i_0 += 1
                    }
                    if i_0 == power_max as libc::c_int {
                        return 6 as libc::c_int
                    }
                    (*e_ptr).power = i_0 as s16b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'a' as i32 {
                    if prefix(buf.offset(2 as libc::c_int as isize) as cptr,
                              b"HARDCORE=\x00" as *const u8 as
                                  *const libc::c_char) != 0 {
                        (*e_ptr).activate =
                            get_activation(buf.offset(11 as libc::c_int as
                                                          isize)) as s16b;
                        if (*e_ptr).activate as libc::c_int ==
                               -(1 as libc::c_int) {
                            return 1 as libc::c_int
                        }
                    } else if prefix(buf.offset(2 as libc::c_int as isize) as
                                         cptr,
                                     b"SPELL=\x00" as *const u8 as
                                         *const libc::c_char) != 0 {
                        (*e_ptr).activate =
                            -find_spell(buf.offset(8 as libc::c_int as isize))
                                as s16b;
                        if (*e_ptr).activate as libc::c_int ==
                               --(1 as libc::c_int) {
                            return 1 as libc::c_int
                        }
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'r' as i32 &&
                              *buf.offset(2 as libc::c_int as isize) as
                                  libc::c_int == 'N' as i32 {
                    /* Hack -- Process 'r:N' for needed flags */
                    /* Parse every entry textually */
                    s = buf.offset(4 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh47 = t;
                            t = t.offset(1);
                            *fresh47 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_ego_item_flag_restrict(e_ptr,
                                                               s as cptr,
                                                               1 as
                                                                   libc::c_int
                                                                   as bool_)
                                   as libc::c_int {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'r' as i32 &&
                              *buf.offset(2 as libc::c_int as isize) as
                                  libc::c_int == 'F' as i32 {
                    /* Hack -- Process 'r:F' for forbidden flags */
                    /* Parse every entry textually */
                    s = buf.offset(4 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh48 = t;
                            t = t.offset(1);
                            *fresh48 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_ego_item_flag_restrict(e_ptr,
                                                               s as cptr,
                                                               0 as
                                                                   libc::c_int
                                                                   as bool_)
                                   as libc::c_int {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'F' as i32 {
                    if cur_r == -(1 as libc::c_int) {
                        return 6 as libc::c_int
                    }
                    /* Hack -- Process 'F' for flags */
                    /* Parse every entry textually */
                    s = buf.offset(2 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh49 = t;
                            t = t.offset(1);
                            *fresh49 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_ego_item_flag(e_ptr, s as cptr, cur_r,
                                                      0 as libc::c_int as
                                                          bool_) as
                                   libc::c_int {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'f' as i32 {
                    if cur_r == -(1 as libc::c_int) {
                        return 6 as libc::c_int
                    }
                    /* Hack -- Process 'f' for obvious flags */
                    /* Parse every entry textually */
                    s = buf.offset(2 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh50 = t;
                            t = t.offset(1);
                            *fresh50 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_ego_item_flag(e_ptr, s as cptr, cur_r,
                                                      1 as libc::c_int as
                                                          bool_) as
                                   libc::c_int {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else {
                    /* Oops */
                    return 6 as libc::c_int
                }
            }
        }
    }
    /* Complete the "name" and "text" sizes */
    (*e_head).name_size = (*e_head).name_size.wrapping_add(1);
    (*e_head).text_size = (*e_head).text_size.wrapping_add(1);
    /* No version yet */
    if okay == 0 { return 2 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Grab one flag in a randart_part_type from a textual string
 */
unsafe extern "C" fn grab_one_randart_item_flag(mut ra_ptr:
                                                    *mut randart_part_type,
                                                mut what: cptr,
                                                mut c: libc::c_char)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut f1: *mut u32b = 0 as *mut u32b;
    let mut f2: *mut u32b = 0 as *mut u32b;
    let mut f3: *mut u32b = 0 as *mut u32b;
    let mut f4: *mut u32b = 0 as *mut u32b;
    let mut f5: *mut u32b = 0 as *mut u32b;
    let mut esp: *mut u32b = 0 as *mut u32b;
    if c as libc::c_int == 'F' as i32 {
        f1 = &mut (*ra_ptr).flags1;
        f2 = &mut (*ra_ptr).flags2;
        f3 = &mut (*ra_ptr).flags3;
        f4 = &mut (*ra_ptr).flags4;
        f5 = &mut (*ra_ptr).flags5;
        esp = &mut (*ra_ptr).esp
    } else {
        f1 = &mut (*ra_ptr).aflags1;
        f2 = &mut (*ra_ptr).aflags2;
        f3 = &mut (*ra_ptr).aflags3;
        f4 = &mut (*ra_ptr).aflags4;
        f5 = &mut (*ra_ptr).aflags5;
        esp = &mut (*ra_ptr).aesp
    }
    /* Check flags1 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags1[i as usize]) != 0 {
            *f1 = (*f1 as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Check flags2 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags2[i as usize]) != 0 {
            *f2 = (*f2 as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Check flags2 -- traps */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags2_trap[i as usize]) != 0 {
            *f2 = (*f2 as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Check flags3 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags3[i as usize]) != 0 {
            *f3 = (*f3 as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Check flags4 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags4[i as usize]) != 0 {
            *f4 = (*f4 as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Check flags5 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, k_info_flags5[i as usize]) != 0 {
            *f5 = (*f5 as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Check esp_flags */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, esp_flags[i as usize]) != 0 {
            *esp = (*esp as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    /* Check ego_flags */
    if c as libc::c_int == 'F' as i32 {
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if streq(what, ego_flags[i as usize]) != 0 {
                (*ra_ptr).fego =
                    ((*ra_ptr).fego as libc::c_long |
                         (1 as libc::c_long) << i) as u32b;
                return 0 as libc::c_int as bool_
            }
            i += 1
        }
    }
    /* Oops */
    msg_format(b"Unknown ego-item flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Error */
    return 1 as libc::c_int as bool_;
}
/*
 * Initialize the "ra_info" array, by parsing an ascii "template" file
 */
#[no_mangle]
pub unsafe extern "C" fn init_ra_info_txt(mut fp: *mut FILE,
                                          mut buf: *mut libc::c_char)
 -> errr {
    let mut i: libc::c_int = 0;
    let mut cur_t: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut cur_g: libc::c_int = 0 as libc::c_int;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Not ready yet */
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    /* Current entry */
    let mut ra_ptr: *mut randart_part_type = 0 as *mut randart_part_type;
    /* Just before the first record */
    error_idx = -(1 as libc::c_int) as s16b;
    /* Just before the first line */
    error_line = -(1 as libc::c_int) as s16b;
    /* Parse */
    fp_stack_init(fp);
    /* Next... */
    while 0 as libc::c_int == my_fgets_dostack(buf, 1024 as libc::c_int) {
        /* Advance the line number */
        error_line += 1;
        /* Skip comments and blank lines */
        if *buf.offset(0 as libc::c_int as isize) == 0 ||
               *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '#' as i32 {
            continue ;
        }
        /* Verify correct "colon" format */
        if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
           {
            return 1 as libc::c_int
        }
        /* Hack -- Process 'V' for "Version" */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'V' as i32
           {
            let mut v1: libc::c_int = 0;
            let mut v2: libc::c_int = 0;
            let mut v3: libc::c_int = 0;
            /* Scan for the values */
            if 3 as libc::c_int !=
                   sscanf(buf.offset(2 as libc::c_int as isize),
                          b"%d.%d.%d\x00" as *const u8 as *const libc::c_char,
                          &mut v1 as *mut libc::c_int,
                          &mut v2 as *mut libc::c_int,
                          &mut v3 as *mut libc::c_int) {
                return 2 as libc::c_int
            }
            /* Okay to proceed */
            okay = 1 as libc::c_int as bool_
        } else {
            /* No version yet */
            if okay == 0 { return 2 as libc::c_int }
            /* Included file */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '<' as i32 {
                fp_stack_push(buf.offset(2 as libc::c_int as isize) as cptr);
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'G' as i32 {
                let mut chance: libc::c_int = 0;
                let mut dd: libc::c_int = 0;
                let mut ds: libc::c_int = 0;
                let mut plus: libc::c_int = 0;
                /* Process 'G' for "General" (up to 30 lines) */
                /* Scan for the values */
                if 4 as libc::c_int !=
                       sscanf(buf.offset(2 as libc::c_int as isize),
                              b"%d:%dd%d:%d\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut chance as *mut libc::c_int,
                              &mut dd as *mut libc::c_int,
                              &mut ds as *mut libc::c_int,
                              &mut plus as *mut libc::c_int) {
                    return 1 as libc::c_int
                }
                /* Save the values */
                ra_gen[cur_g as usize].chance = chance;
                ra_gen[cur_g as usize].dd = dd;
                ra_gen[cur_g as usize].ds = ds;
                ra_gen[cur_g as usize].plus = plus;
                cur_g += 1
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'N' as i32 {
                /* Process 'N' for "New/Number" */
                /* Get the index */
                i = atoi(buf.offset(2 as libc::c_int as isize));
                /* Verify information */
                if i < error_idx as libc::c_int { return 4 as libc::c_int }
                /* Verify information */
                if i >= (*ra_head).info_num as libc::c_int {
                    return 2 as libc::c_int
                }
                /* Save the index */
                error_idx = i as s16b;
                /* Point at the "info" */
                ra_ptr =
                    &mut *ra_info.offset(i as isize) as
                        *mut randart_part_type;
                /* Needed hack */
                (*ra_ptr).power = -(1 as libc::c_int) as s16b;
                cur_t = 0 as libc::c_int;
                j = 0 as libc::c_int;
                while j < 20 as libc::c_int {
                    (*ra_ptr).tval[j as usize] =
                        255 as libc::c_int as byte_hack;
                    j += 1
                }
                (*ra_ptr).flags1 = 0 as libc::c_int as u32b;
                (*ra_ptr).flags2 = 0 as libc::c_int as u32b;
                (*ra_ptr).flags3 = 0 as libc::c_int as u32b;
                (*ra_ptr).flags4 = 0 as libc::c_int as u32b;
                (*ra_ptr).flags5 = 0 as libc::c_int as u32b;
                (*ra_ptr).esp = 0 as libc::c_int as u32b;
                (*ra_ptr).fego = 0 as libc::c_int as u32b
            } else {
                /* There better be a current ra_ptr */
                if ra_ptr.is_null() { return 3 as libc::c_int }
                /* Process 'T' for "Tval/Sval" (up to 5 lines) */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'T' as i32 {
                    let mut tv: libc::c_int = 0;
                    let mut minsv: libc::c_int = 0;
                    let mut maxsv: libc::c_int = 0;
                    if cur_t == 20 as libc::c_int { return 1 as libc::c_int }
                    /* Scan for the values */
                    if 3 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut tv as *mut libc::c_int,
                                  &mut minsv as *mut libc::c_int,
                                  &mut maxsv as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*ra_ptr).tval[cur_t as usize] = tv as byte_hack;
                    (*ra_ptr).min_sval[cur_t as usize] = minsv as byte_hack;
                    (*ra_ptr).max_sval[cur_t as usize] = maxsv as byte_hack;
                    cur_t += 1
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'X' as i32 {
                    let mut power: libc::c_int = 0;
                    let mut max: libc::c_int = 0;
                    /* Process 'X' for "Xtra" (one line only) */
                    /* Scan for the values */
                    if 2 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut power as *mut libc::c_int,
                                  &mut max as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*ra_ptr).value = power;
                    (*ra_ptr).max = max as s16b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'W' as i32 {
                    let mut level: libc::c_int = 0;
                    let mut rarity: libc::c_int = 0;
                    let mut rarity2: libc::c_int = 0;
                    /* Process 'W' for "More Info" (one line only) */
                    /* Scan for the values */
                    if 3 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut level as *mut libc::c_int,
                                  &mut rarity as *mut libc::c_int,
                                  &mut rarity2 as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*ra_ptr).level = level as byte_hack;
                    (*ra_ptr).rarity = rarity as byte_hack;
                    (*ra_ptr).mrarity = rarity2 as byte_hack
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'C' as i32 {
                    let mut th: libc::c_int = 0;
                    let mut td: libc::c_int = 0;
                    let mut ta: libc::c_int = 0;
                    let mut pv: libc::c_int = 0;
                    /* Hack -- Process 'C' for "creation" */
                    /* Scan for the values */
                    if 4 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut th as *mut libc::c_int,
                                  &mut td as *mut libc::c_int,
                                  &mut ta as *mut libc::c_int,
                                  &mut pv as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    (*ra_ptr).max_to_h = th as s16b;
                    (*ra_ptr).max_to_d = td as s16b;
                    (*ra_ptr).max_to_a = ta as s16b;
                    (*ra_ptr).max_pval = pv
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'Z' as i32 {
                    let mut i_0: libc::c_int = 0;
                    /* Process 'Z' for "Granted power" */
                    /* Acquire the text */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Find it in the list */
                    i_0 = 0 as libc::c_int;
                    while i_0 < power_max as libc::c_int {
                        if strcasecmp(s,
                                      (*powers_type.offset(i_0 as
                                                               isize)).name)
                               == 0 {
                            break ;
                        }
                        i_0 += 1
                    }
                    if i_0 == power_max as libc::c_int {
                        return 6 as libc::c_int
                    }
                    (*ra_ptr).power = i_0 as s16b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'F' as i32 {
                    /* Hack -- Process 'F' for flags */
                    /* Parse every entry textually */
                    s = buf.offset(2 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh51 = t;
                            t = t.offset(1);
                            *fresh51 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_randart_item_flag(ra_ptr, s as cptr,
                                                          'F' as i32 as
                                                              libc::c_char) as
                                   libc::c_int {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'A' as i32 {
                    /* Hack -- Process 'A' for antagonic flags */
                    /* Parse every entry textually */
                    s = buf.offset(2 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh52 = t;
                            t = t.offset(1);
                            *fresh52 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_randart_item_flag(ra_ptr, s as cptr,
                                                          'A' as i32 as
                                                              libc::c_char) as
                                   libc::c_int {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else {
                    /* Oops */
                    return 6 as libc::c_int
                }
            }
        }
    }
    /* No version yet */
    if okay == 0 { return 2 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Grab one (basic) flag in a monster_race from a textual string
 */
unsafe extern "C" fn grab_one_basic_flag(mut r_ptr: *mut monster_race,
                                         mut what: cptr) -> errr {
    let mut i: libc::c_int = 0;
    /* Scan flags1 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags1[i as usize]) != 0 {
            (*r_ptr).flags1 =
                ((*r_ptr).flags1 as libc::c_long | (1 as libc::c_long) << i)
                    as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags2 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags2[i as usize]) != 0 {
            (*r_ptr).flags2 =
                ((*r_ptr).flags2 as libc::c_long | (1 as libc::c_long) << i)
                    as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags3 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags3[i as usize]) != 0 {
            (*r_ptr).flags3 =
                ((*r_ptr).flags3 as libc::c_long | (1 as libc::c_long) << i)
                    as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags7 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags7[i as usize]) != 0 {
            (*r_ptr).flags7 =
                ((*r_ptr).flags7 as libc::c_long | (1 as libc::c_long) << i)
                    as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags8 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags8[i as usize]) != 0 {
            (*r_ptr).flags8 =
                ((*r_ptr).flags8 as libc::c_long | (1 as libc::c_long) << i)
                    as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags9 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags9[i as usize]) != 0 {
            (*r_ptr).flags9 =
                ((*r_ptr).flags9 as libc::c_long | (1 as libc::c_long) << i)
                    as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Oops */
    msg_format(b"Unknown monster flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Failure */
    return 1 as libc::c_int;
}
/*
 * Grab one (spell) flag in a monster_race from a textual string
 */
unsafe extern "C" fn grab_one_spell_flag(mut r_ptr: *mut monster_race,
                                         mut what: cptr) -> errr {
    let mut i: libc::c_int = 0;
    /* Scan flags4 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags4[i as usize]) != 0 {
            (*r_ptr).flags4 =
                ((*r_ptr).flags4 as libc::c_long | (1 as libc::c_long) << i)
                    as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags5 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags5[i as usize]) != 0 {
            (*r_ptr).flags5 =
                ((*r_ptr).flags5 as libc::c_long | (1 as libc::c_long) << i)
                    as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags6 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags6[i as usize]) != 0 {
            (*r_ptr).flags6 =
                ((*r_ptr).flags6 as libc::c_long | (1 as libc::c_long) << i)
                    as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Oops */
    msg_format(b"Unknown monster flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Failure */
    return 1 as libc::c_int;
}
/*
 * Initialize the "r_info" array, by parsing an ascii "template" file
 */
#[no_mangle]
pub unsafe extern "C" fn init_r_info_txt(mut fp: *mut FILE,
                                         mut buf: *mut libc::c_char) -> errr {
    let mut i: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Not ready yet */
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    /* Current entry */
    let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
    /* Just before the first record */
    error_idx = -(1 as libc::c_int) as s16b;
    /* Just before the first line */
    error_line = -(1 as libc::c_int) as s16b;
    /* Start the "fake" stuff */
    (*r_head).name_size = 0 as libc::c_int as u32b;
    (*r_head).text_size = 0 as libc::c_int as u32b;
    /* Parse */
    fp_stack_init(fp);
    /* Next... */
    while 0 as libc::c_int == my_fgets_dostack(buf, 1024 as libc::c_int) {
        /* Advance the line number */
        error_line += 1;
        /* Skip comments and blank lines */
        if *buf.offset(0 as libc::c_int as isize) == 0 ||
               *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '#' as i32 {
            continue ;
        }
        /* Verify correct "colon" format */
        if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
           {
            return 1 as libc::c_int
        }
        /* Hack -- Process 'V' for "Version" */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'V' as i32
           {
            let mut v1: libc::c_int = 0;
            let mut v2: libc::c_int = 0;
            let mut v3: libc::c_int = 0;
            /* Scan for the values */
            if 3 as libc::c_int !=
                   sscanf(buf.offset(2 as libc::c_int as isize),
                          b"%d.%d.%d\x00" as *const u8 as *const libc::c_char,
                          &mut v1 as *mut libc::c_int,
                          &mut v2 as *mut libc::c_int,
                          &mut v3 as *mut libc::c_int) {
                return 2 as libc::c_int
            }
            /* Okay to proceed */
            okay = 1 as libc::c_int as bool_
        } else {
            /* No version yet */
            if okay == 0 { return 2 as libc::c_int }
            /* Included file */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '<' as i32 {
                fp_stack_push(buf.offset(2 as libc::c_int as isize) as cptr);
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'N' as i32 {
                /* Process 'N' for "New/Number/Name" */
                /* Find the colon before the name */
                s = strchr(buf.offset(2 as libc::c_int as isize), ':' as i32);
                /* Verify that colon */
                if s.is_null() { return 1 as libc::c_int }
                /* Nuke the colon, advance to the name */
                let fresh53 = s;
                s = s.offset(1);
                *fresh53 = '\u{0}' as i32 as libc::c_char;
                /* Paranoia -- require a name */
                if *s == 0 { return 1 as libc::c_int }
                /* Get the index */
                i = atoi(buf.offset(2 as libc::c_int as isize));
                /* Verify information */
                if i < error_idx as libc::c_int { return 4 as libc::c_int }
                /* Verify information */
                if i >= (*r_head).info_num as libc::c_int {
                    return 2 as libc::c_int
                }
                /* Save the index */
                error_idx = i as s16b;
                /* Point at the "info" */
                r_ptr = &mut *r_info.offset(i as isize) as *mut monster_race;
                /* Hack -- Verify space */
                if ((*r_head).name_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_name_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the name index */
                if (*r_ptr).name == 0 {
                    (*r_head).name_size = (*r_head).name_size.wrapping_add(1);
                    (*r_ptr).name = (*r_head).name_size
                }
                /* Append chars to the name */
                strcpy(r_name.offset((*r_head).name_size as isize), s);
                /* Advance the index */
                (*r_head).name_size =
                    ((*r_head).name_size as
                         libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                        u32b;
                /* HACK -- Those ones HAVE to have a set default value */
                (*r_ptr).drops.treasure = 20 as libc::c_int as byte_hack;
                (*r_ptr).drops.combat = 20 as libc::c_int as byte_hack;
                (*r_ptr).drops.magic = 20 as libc::c_int as byte_hack;
                (*r_ptr).drops.tools = 20 as libc::c_int as byte_hack;
                (*r_ptr).freq_spell = 0 as libc::c_int as byte_hack;
                (*r_ptr).freq_inate = (*r_ptr).freq_spell
            } else {
                /* There better be a current r_ptr */
                if r_ptr.is_null() { return 3 as libc::c_int }
                /* Process 'D' for "Description" */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'D' as i32 {
                    /* Acquire the text */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Hack -- Verify space */
                    if ((*r_head).text_size as
                            libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                           > fake_text_size as libc::c_ulong {
                        return 7 as libc::c_int
                    }
                    /* Advance and Save the text index */
                    if (*r_ptr).text == 0 {
                        (*r_head).text_size =
                            (*r_head).text_size.wrapping_add(1);
                        (*r_ptr).text = (*r_head).text_size
                    }
                    /* Append chars to the name */
                    strcpy(r_text.offset((*r_head).text_size as isize), s);
                    /* Advance the index */
                    (*r_head).text_size =
                        ((*r_head).text_size as
                             libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                            u32b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'G' as i32 {
                    let mut sym: libc::c_char = 0;
                    let mut tmp: libc::c_int = 0;
                    /* Process 'G' for "Graphics" (one line only) */
                    /* Paranoia */
                    if *buf.offset(2 as libc::c_int as isize) == 0 {
                        return 1 as libc::c_int
                    }
                    if *buf.offset(3 as libc::c_int as isize) == 0 {
                        return 1 as libc::c_int
                    }
                    if *buf.offset(4 as libc::c_int as isize) == 0 {
                        return 1 as libc::c_int
                    }
                    /* Extract the char */
                    sym = *buf.offset(2 as libc::c_int as isize);
                    /* Extract the attr */
                    tmp =
                        color_char_to_attr(*buf.offset(4 as libc::c_int as
                                                           isize));
                    /* Paranoia */
                    if tmp < 0 as libc::c_int { return 1 as libc::c_int }
                    /* Save the values */
                    (*r_ptr).d_char = sym;
                    (*r_ptr).d_attr = tmp as byte_hack
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'I' as i32 {
                    let mut spd: libc::c_int = 0;
                    let mut hp1: libc::c_int = 0;
                    let mut hp2: libc::c_int = 0;
                    let mut aaf: libc::c_int = 0;
                    let mut ac: libc::c_int = 0;
                    let mut slp: libc::c_int = 0;
                    /* Process 'I' for "Info" (one line only) */
                    /* Scan for the other values */
                    if 6 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%dd%d:%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut spd as *mut libc::c_int,
                                  &mut hp1 as *mut libc::c_int,
                                  &mut hp2 as *mut libc::c_int,
                                  &mut aaf as *mut libc::c_int,
                                  &mut ac as *mut libc::c_int,
                                  &mut slp as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*r_ptr).speed = spd as byte_hack;
                    (*r_ptr).hdice = hp1 as u16b;
                    (*r_ptr).hside = hp2 as u16b;
                    (*r_ptr).aaf = aaf as byte_hack;
                    (*r_ptr).ac = ac as s16b;
                    (*r_ptr).sleep = slp as s16b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'E' as i32 {
                    let mut weap: libc::c_int = 0;
                    let mut tors: libc::c_int = 0;
                    let mut fing: libc::c_int = 0;
                    let mut head: libc::c_int = 0;
                    let mut arms: libc::c_int = 0;
                    let mut legs: libc::c_int = 0;
                    /* Process 'E' for "Body Parts" (one line only) */
                    /* Scan for the other values */
                    if 6 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d:%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut weap as *mut libc::c_int,
                                  &mut tors as *mut libc::c_int,
                                  &mut arms as *mut libc::c_int,
                                  &mut fing as *mut libc::c_int,
                                  &mut head as *mut libc::c_int,
                                  &mut legs as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*r_ptr).body_parts[0 as libc::c_int as usize] =
                        weap as byte_hack;
                    (*r_ptr).body_parts[1 as libc::c_int as usize] =
                        tors as byte_hack;
                    (*r_ptr).body_parts[2 as libc::c_int as usize] =
                        arms as byte_hack;
                    (*r_ptr).body_parts[3 as libc::c_int as usize] =
                        fing as byte_hack;
                    (*r_ptr).body_parts[4 as libc::c_int as usize] =
                        head as byte_hack;
                    (*r_ptr).body_parts[5 as libc::c_int as usize] =
                        legs as byte_hack;
                    /* Mega debugging hack */
                    if weap > arms {
                        quit(format(b"monster %d, %d weapon(s), %d arm(s) !\x00"
                                        as *const u8 as *const libc::c_char,
                                    error_idx as libc::c_int, weap, arms) as
                                 cptr);
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'O' as i32 {
                    let mut treasure: libc::c_int = 0;
                    let mut combat: libc::c_int = 0;
                    let mut magic: libc::c_int = 0;
                    let mut tools: libc::c_int = 0;
                    /* Process 'O' for "Object type" (one line only) */
                    /* Scan for the values */
                    if 4 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut treasure as *mut libc::c_int,
                                  &mut combat as *mut libc::c_int,
                                  &mut magic as *mut libc::c_int,
                                  &mut tools as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*r_ptr).drops.treasure = treasure as byte_hack;
                    (*r_ptr).drops.combat = combat as byte_hack;
                    (*r_ptr).drops.magic = magic as byte_hack;
                    (*r_ptr).drops.tools = tools as byte_hack
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'W' as i32 {
                    let mut lev: libc::c_int = 0;
                    let mut rar: libc::c_int = 0;
                    let mut wt: libc::c_int = 0;
                    let mut exp: libc::c_long = 0;
                    /* Process 'W' for "More Info" (one line only) */
                    /* Scan for the values */
                    if 4 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d:%ld\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut lev as *mut libc::c_int,
                                  &mut rar as *mut libc::c_int,
                                  &mut wt as *mut libc::c_int,
                                  &mut exp as *mut libc::c_long) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*r_ptr).level = lev as byte_hack;
                    (*r_ptr).rarity = rar as byte_hack;
                    /* MEGA HACK */
                    if wt == 0 { wt = 100 as libc::c_int }
                    (*r_ptr).weight = wt;
                    (*r_ptr).mexp = exp as s32b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'B' as i32 {
                    let mut n1: libc::c_int = 0;
                    let mut n2: libc::c_int = 0;
                    /* Process 'B' for "Blows" (up to four lines) */
                    /* Find the next empty blow slot (if any) */
                    i = 0 as libc::c_int;
                    while i < 4 as libc::c_int {
                        if (*r_ptr).blow[i as usize].method == 0 { break ; }
                        i += 1
                    }
                    /* Oops, no more slots */
                    if i == 4 as libc::c_int { return 1 as libc::c_int }
                    /* Analyze the first field */
                    t = buf.offset(2 as libc::c_int as isize);
                    s = t;
                    while *t as libc::c_int != 0 &&
                              *t as libc::c_int != ':' as i32 {
                        /* loop */
                        t = t.offset(1)
                    }
                    /* Terminate the field (if necessary) */
                    if *t as libc::c_int == ':' as i32 {
                        let fresh54 = t;
                        t = t.offset(1);
                        *fresh54 = '\u{0}' as i32 as libc::c_char
                    }
                    /* Analyze the method */
                    n1 = 0 as libc::c_int;
                    while !r_info_blow_method[n1 as usize].is_null() {
                        if streq(s as cptr, r_info_blow_method[n1 as usize])
                               != 0 {
                            break ;
                        }
                        n1 += 1
                    }
                    /* Invalid method */
                    if r_info_blow_method[n1 as usize].is_null() {
                        return 1 as libc::c_int
                    }
                    /* Analyze the second field */
                    s = t;
                    while *t as libc::c_int != 0 &&
                              *t as libc::c_int != ':' as i32 {
                        /* loop */
                        t = t.offset(1)
                    }
                    /* Terminate the field (if necessary) */
                    if *t as libc::c_int == ':' as i32 {
                        let fresh55 = t;
                        t = t.offset(1);
                        *fresh55 = '\u{0}' as i32 as libc::c_char
                    }
                    /* Analyze effect */
                    n2 = 0 as libc::c_int;
                    while !r_info_blow_effect[n2 as usize].is_null() {
                        if streq(s as cptr, r_info_blow_effect[n2 as usize])
                               != 0 {
                            break ;
                        }
                        n2 += 1
                    }
                    /* Invalid effect */
                    if r_info_blow_effect[n2 as usize].is_null() {
                        return 1 as libc::c_int
                    }
                    /* Analyze the third field */
                    s = t;
                    while *t as libc::c_int != 0 &&
                              *t as libc::c_int != 'd' as i32 {
                        /* loop */
                        t = t.offset(1)
                    }
                    /* Terminate the field (if necessary) */
                    if *t as libc::c_int == 'd' as i32 {
                        let fresh56 = t;
                        t = t.offset(1);
                        *fresh56 = '\u{0}' as i32 as libc::c_char
                    }
                    /* Save the method */
                    (*r_ptr).blow[i as usize].method = n1 as byte_hack;
                    /* Save the effect */
                    (*r_ptr).blow[i as usize].effect = n2 as byte_hack;
                    /* Extract the damage dice and sides */
                    (*r_ptr).blow[i as usize].d_dice = atoi(s) as byte_hack;
                    (*r_ptr).blow[i as usize].d_side = atoi(t) as byte_hack
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'F' as i32 {
                    /* Process 'F' for "Basic Flags" (multiple lines) */
                    /* Parse every entry */
                    s = buf.offset(2 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh57 = t;
                            t = t.offset(1);
                            *fresh57 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_basic_flag(r_ptr, s as cptr) {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'S' as i32 {
                    /* Process 'S' for "Spell Flags" (multiple lines) */
                    /* Parse every entry */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Continue */
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh58 = t;
                            t = t.offset(1);
                            *fresh58 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* XXX XXX XXX Hack -- Read spell frequency */
                        if 1 as libc::c_int ==
                               sscanf(s,
                                      b"1_IN_%d\x00" as *const u8 as
                                          *const libc::c_char,
                                      &mut i as *mut libc::c_int) {
                            /* Extract a "frequency" */
                            (*r_ptr).freq_inate =
                                (100 as libc::c_int / i) as byte_hack;
                            (*r_ptr).freq_spell = (*r_ptr).freq_inate;
                            /* Start at next entry */
                            s = t
                        } else {
                            /* Parse this entry */
                            if 0 as libc::c_int !=
                                   grab_one_spell_flag(r_ptr, s as cptr) {
                                return 5 as libc::c_int
                            }
                            /* Start the next entry */
                            s = t
                        }
                    }
                } else {
                    /* Oops */
                    return 6 as libc::c_int
                }
            }
        }
    }
    /* Complete the "name" and "text" sizes */
    (*r_head).name_size = (*r_head).name_size.wrapping_add(1);
    (*r_head).text_size = (*r_head).text_size.wrapping_add(1);
    i = 1 as libc::c_int;
    while i < max_r_idx as libc::c_int {
        /* Invert flag WILD_ONLY <-> RF8_DUNGEON */
        let ref mut fresh59 = (*r_info.offset(i as isize)).flags8;
        *fresh59 = (*fresh59 as libc::c_long ^ 1 as libc::c_long) as u32b;
        /* WILD_TOO without any other wilderness flags enables all flags */
        if (*r_info.offset(i as isize)).flags8 & 0x80000000 as libc::c_uint !=
               0 &&
               (*r_info.offset(i as isize)).flags8 &
                   0x7ffffffe as libc::c_int as libc::c_uint == 0 {
            (*r_info.offset(i as isize)).flags8 = 0x463 as libc::c_int as u32b
        }
        i += 1
    }
    /* No version yet */
    if okay == 0 { return 2 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Grab one (basic) flag in a monster_race from a textual string
 */
unsafe extern "C" fn grab_one_basic_ego_flag(mut re_ptr: *mut monster_ego,
                                             mut what: cptr, mut add: bool_)
 -> errr {
    let mut i: libc::c_int = 0;
    /* Scan flags1 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags1[i as usize]) != 0 {
            if add != 0 {
                (*re_ptr).mflags1 =
                    ((*re_ptr).mflags1 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*re_ptr).nflags1 =
                    ((*re_ptr).nflags1 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags2 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags2[i as usize]) != 0 {
            if add != 0 {
                (*re_ptr).mflags2 =
                    ((*re_ptr).mflags2 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*re_ptr).nflags2 =
                    ((*re_ptr).nflags2 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags3 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags3[i as usize]) != 0 {
            if add != 0 {
                (*re_ptr).mflags3 =
                    ((*re_ptr).mflags3 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*re_ptr).nflags3 =
                    ((*re_ptr).nflags3 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags7 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags7[i as usize]) != 0 {
            if add != 0 {
                (*re_ptr).mflags7 =
                    ((*re_ptr).mflags7 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*re_ptr).nflags7 =
                    ((*re_ptr).nflags7 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags8 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags8[i as usize]) != 0 {
            if add != 0 {
                (*re_ptr).mflags8 =
                    ((*re_ptr).mflags8 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*re_ptr).nflags8 =
                    ((*re_ptr).nflags8 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags9 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags9[i as usize]) != 0 {
            if add != 0 {
                (*re_ptr).mflags9 =
                    ((*re_ptr).mflags9 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*re_ptr).nflags9 =
                    ((*re_ptr).nflags9 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Oops */
    msg_format(b"Unknown monster flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Failure */
    return 1 as libc::c_int;
}
/*
 * Grab one (spell) flag in a monster_race from a textual string
 */
unsafe extern "C" fn grab_one_spell_ego_flag(mut re_ptr: *mut monster_ego,
                                             mut what: cptr, mut add: bool_)
 -> errr {
    let mut i: libc::c_int = 0;
    /* Scan flags4 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags4[i as usize]) != 0 {
            if add != 0 {
                (*re_ptr).mflags4 =
                    ((*re_ptr).mflags4 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*re_ptr).nflags4 =
                    ((*re_ptr).nflags4 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags5 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags5[i as usize]) != 0 {
            if add != 0 {
                (*re_ptr).mflags5 =
                    ((*re_ptr).mflags5 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*re_ptr).nflags5 =
                    ((*re_ptr).nflags5 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags6 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags6[i as usize]) != 0 {
            if add != 0 {
                (*re_ptr).mflags6 =
                    ((*re_ptr).mflags6 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*re_ptr).nflags6 =
                    ((*re_ptr).nflags6 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Oops */
    msg_format(b"Unknown monster flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Failure */
    return 1 as libc::c_int;
}
/*
 * Grab one (basic) flag in a monster_race from a textual string
 */
unsafe extern "C" fn grab_one_ego_flag(mut re_ptr: *mut monster_ego,
                                       mut what: cptr, mut must: bool_)
 -> errr {
    let mut i: libc::c_int = 0;
    /* Scan flags1 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags1[i as usize]) != 0 {
            if must != 0 {
                (*re_ptr).flags1 =
                    ((*re_ptr).flags1 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*re_ptr).hflags1 =
                    ((*re_ptr).hflags1 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags2 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags2[i as usize]) != 0 {
            if must != 0 {
                (*re_ptr).flags2 =
                    ((*re_ptr).flags2 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*re_ptr).hflags2 =
                    ((*re_ptr).hflags2 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags3 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags3[i as usize]) != 0 {
            if must != 0 {
                (*re_ptr).flags3 =
                    ((*re_ptr).flags3 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*re_ptr).hflags3 =
                    ((*re_ptr).hflags3 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags7 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags7[i as usize]) != 0 {
            if must != 0 {
                (*re_ptr).flags7 =
                    ((*re_ptr).flags7 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*re_ptr).hflags7 =
                    ((*re_ptr).hflags7 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags8 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags8[i as usize]) != 0 {
            if must != 0 {
                (*re_ptr).flags8 =
                    ((*re_ptr).flags8 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*re_ptr).hflags8 =
                    ((*re_ptr).hflags8 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags9 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags9[i as usize]) != 0 {
            if must != 0 {
                (*re_ptr).flags9 =
                    ((*re_ptr).flags9 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            } else {
                (*re_ptr).hflags9 =
                    ((*re_ptr).hflags9 as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Oops */
    msg_format(b"Unknown monster flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Failure */
    return 1 as libc::c_int;
}
/*
 * Initialize the "re_info" array, by parsing an ascii "template" file
 */
#[no_mangle]
pub unsafe extern "C" fn init_re_info_txt(mut fp: *mut FILE,
                                          mut buf: *mut libc::c_char)
 -> errr {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut blow_num: byte_hack = 0 as libc::c_int as byte_hack;
    let mut r_char_number: libc::c_int = 0 as libc::c_int;
    let mut nr_char_number: libc::c_int = 0 as libc::c_int;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Not ready yet */
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    /* Current entry */
    let mut re_ptr: *mut monster_ego = 0 as *mut monster_ego;
    /* Just before the first record */
    error_idx = -(1 as libc::c_int) as s16b;
    /* Just before the first line */
    error_line = -(1 as libc::c_int) as s16b;
    /* Start the "fake" stuff */
    (*re_head).name_size = 0 as libc::c_int as u32b;
    (*re_head).text_size = 0 as libc::c_int as u32b;
    /* Parse */
    fp_stack_init(fp);
    /* Next... */
    while 0 as libc::c_int == my_fgets_dostack(buf, 1024 as libc::c_int) {
        /* Advance the line number */
        error_line += 1;
        /* Skip comments and blank lines */
        if *buf.offset(0 as libc::c_int as isize) == 0 ||
               *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '#' as i32 {
            continue ;
        }
        /* Verify correct "colon" format */
        if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
           {
            return 1 as libc::c_int
        }
        /* Hack -- Process 'V' for "Version" */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'V' as i32
           {
            let mut v1: libc::c_int = 0;
            let mut v2: libc::c_int = 0;
            let mut v3: libc::c_int = 0;
            /* Scan for the values */
            if 3 as libc::c_int !=
                   sscanf(buf.offset(2 as libc::c_int as isize),
                          b"%d.%d.%d\x00" as *const u8 as *const libc::c_char,
                          &mut v1 as *mut libc::c_int,
                          &mut v2 as *mut libc::c_int,
                          &mut v3 as *mut libc::c_int) {
                return 2 as libc::c_int
            }
            /* Okay to proceed */
            okay = 1 as libc::c_int as bool_
        } else {
            /* No version yet */
            if okay == 0 { return 2 as libc::c_int }
            /* Included file */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '<' as i32 {
                fp_stack_push(buf.offset(2 as libc::c_int as isize) as cptr);
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'N' as i32 {
                /* Process 'N' for "New/Number/Name" */
                /* Find the colon before the name */
                s = strchr(buf.offset(2 as libc::c_int as isize), ':' as i32);
                /* Verify that colon */
                if s.is_null() { return 1 as libc::c_int }
                /* Nuke the colon, advance to the name */
                let fresh60 = s;
                s = s.offset(1);
                *fresh60 = '\u{0}' as i32 as libc::c_char;
                /* Paranoia -- require a name */
                if *s == 0 { return 1 as libc::c_int }
                /* Get the index */
                i = atoi(buf.offset(2 as libc::c_int as isize));
                /* Verify information */
                if i < error_idx as libc::c_int { return 4 as libc::c_int }
                /* Verify information */
                if i >= (*re_head).info_num as libc::c_int {
                    return 2 as libc::c_int
                }
                /* Save the index */
                error_idx = i as s16b;
                /* Point at the "info" */
                re_ptr = &mut *re_info.offset(i as isize) as *mut monster_ego;
                /* Hack -- Verify space */
                if ((*re_head).name_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_name_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the name index */
                if (*re_ptr).name == 0 {
                    (*re_head).name_size =
                        (*re_head).name_size.wrapping_add(1);
                    (*re_ptr).name = (*re_head).name_size
                }
                /* Append chars to the name */
                strcpy(re_name.offset((*re_head).name_size as isize), s);
                /* Advance the index */
                (*re_head).name_size =
                    ((*re_head).name_size as
                         libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                        u32b;
                /* Some inits */
                blow_num = 0 as libc::c_int as byte_hack;
                r_char_number = 0 as libc::c_int;
                nr_char_number = 0 as libc::c_int;
                j = 0 as libc::c_int;
                while j < 5 as libc::c_int {
                    (*re_ptr).r_char[j as usize] =
                        0 as libc::c_int as libc::c_char;
                    j += 1
                }
                j = 0 as libc::c_int;
                while j < 5 as libc::c_int {
                    (*re_ptr).nr_char[j as usize] =
                        0 as libc::c_int as libc::c_char;
                    j += 1
                }
                j = 0 as libc::c_int;
                while j < 4 as libc::c_int {
                    (*re_ptr).blow[j as usize].method =
                        0 as libc::c_int as byte_hack;
                    (*re_ptr).blow[j as usize].effect =
                        0 as libc::c_int as byte_hack;
                    (*re_ptr).blow[j as usize].d_dice =
                        0 as libc::c_int as byte_hack;
                    (*re_ptr).blow[j as usize].d_side =
                        0 as libc::c_int as byte_hack;
                    (*re_ptr).blowm[j as usize][0 as libc::c_int as usize] =
                        0 as libc::c_int as byte_hack;
                    (*re_ptr).blowm[j as usize][1 as libc::c_int as usize] =
                        0 as libc::c_int as byte_hack;
                    j += 1
                }
            } else {
                /* There better be a current re_ptr */
                if re_ptr.is_null() { return 3 as libc::c_int }
                /* Process 'G' for "Graphics" (one line only) */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'G' as i32 {
                    let mut sym: libc::c_char = 0;
                    let mut tmp: libc::c_int = 0;
                    /* Paranoia */
                    if *buf.offset(2 as libc::c_int as isize) == 0 {
                        return 1 as libc::c_int
                    }
                    if *buf.offset(3 as libc::c_int as isize) == 0 {
                        return 1 as libc::c_int
                    }
                    if *buf.offset(4 as libc::c_int as isize) == 0 {
                        return 1 as libc::c_int
                    }
                    /* Extract the char */
                    if *buf.offset(2 as libc::c_int as isize) as libc::c_int
                           != '*' as i32 {
                        sym = *buf.offset(2 as libc::c_int as isize)
                    } else { sym = 127 as libc::c_int as libc::c_char }
                    /* Extract the attr */
                    if *buf.offset(4 as libc::c_int as isize) as libc::c_int
                           != '*' as i32 {
                        tmp =
                            color_char_to_attr(*buf.offset(4 as libc::c_int as
                                                               isize))
                    } else { tmp = 127 as libc::c_int }
                    /* Paranoia */
                    if tmp < 0 as libc::c_int { return 1 as libc::c_int }
                    /* Save the values */
                    (*re_ptr).d_char = sym;
                    (*re_ptr).d_attr = tmp as byte_hack
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'I' as i32 {
                    let mut spd: libc::c_int = 0;
                    let mut hp1: libc::c_int = 0;
                    let mut hp2: libc::c_int = 0;
                    let mut aaf: libc::c_int = 0;
                    let mut ac: libc::c_int = 0;
                    let mut slp: libc::c_int = 0;
                    let mut mspd: libc::c_char = 0;
                    let mut mhp1: libc::c_char = 0;
                    let mut mhp2: libc::c_char = 0;
                    let mut maaf: libc::c_char = 0;
                    let mut mac: libc::c_char = 0;
                    let mut mslp: libc::c_char = 0;
                    /* Process 'I' for "Info" (one line only) */
                    /* Scan for the other values */
                    if 12 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%c%d:%c%dd%c%d:%c%d:%c%d:%c%d\x00" as
                                      *const u8 as *const libc::c_char,
                                  &mut mspd as *mut libc::c_char,
                                  &mut spd as *mut libc::c_int,
                                  &mut mhp1 as *mut libc::c_char,
                                  &mut hp1 as *mut libc::c_int,
                                  &mut mhp2 as *mut libc::c_char,
                                  &mut hp2 as *mut libc::c_int,
                                  &mut maaf as *mut libc::c_char,
                                  &mut aaf as *mut libc::c_int,
                                  &mut mac as *mut libc::c_char,
                                  &mut ac as *mut libc::c_int,
                                  &mut mslp as *mut libc::c_char,
                                  &mut slp as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*re_ptr).speed =
                        ((spd << 2 as libc::c_int) +
                             monster_ego_modify(mspd) as libc::c_int) as s16b;
                    (*re_ptr).hdice =
                        ((hp1 << 2 as libc::c_int) +
                             monster_ego_modify(mhp1) as libc::c_int) as s16b;
                    (*re_ptr).hside =
                        ((hp2 << 2 as libc::c_int) +
                             monster_ego_modify(mhp2) as libc::c_int) as s16b;
                    (*re_ptr).aaf =
                        ((aaf << 2 as libc::c_int) +
                             monster_ego_modify(maaf) as libc::c_int) as s16b;
                    (*re_ptr).ac =
                        ((ac << 2 as libc::c_int) +
                             monster_ego_modify(mac) as libc::c_int) as s16b;
                    (*re_ptr).sleep =
                        ((slp << 2 as libc::c_int) +
                             monster_ego_modify(mslp) as libc::c_int) as s16b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'W' as i32 {
                    let mut lev: libc::c_int = 0;
                    let mut rar: libc::c_int = 0;
                    let mut wt: libc::c_int = 0;
                    let mut mlev: libc::c_char = 0;
                    let mut mwt: libc::c_char = 0;
                    let mut mexp: libc::c_char = 0;
                    let mut pos: libc::c_char = 0;
                    let mut exp: libc::c_long = 0;
                    /* Process 'W' for "More Info" (one line only) */
                    /* Scan for the values */
                    if 8 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%c%d:%d:%c%d:%c%ld:%c\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut mlev as *mut libc::c_char,
                                  &mut lev as *mut libc::c_int,
                                  &mut rar as *mut libc::c_int,
                                  &mut mwt as *mut libc::c_char,
                                  &mut wt as *mut libc::c_int,
                                  &mut mexp as *mut libc::c_char,
                                  &mut exp as *mut libc::c_long,
                                  &mut pos as *mut libc::c_char) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*re_ptr).level =
                        ((lev << 2 as libc::c_int) +
                             monster_ego_modify(mlev) as libc::c_int) as s16b;
                    (*re_ptr).rarity = rar as s16b;
                    (*re_ptr).weight =
                        (wt << 2 as libc::c_int) +
                            monster_ego_modify(mwt) as libc::c_int;
                    (*re_ptr).mexp =
                        ((exp << 2 as libc::c_int) +
                             monster_ego_modify(mexp) as libc::c_long) as
                            s32b;
                    (*re_ptr).before =
                        if pos as libc::c_int == 'B' as i32 {
                            1 as libc::c_int
                        } else { 0 as libc::c_int } as bool_
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'B' as i32 {
                    let mut n1: libc::c_int = 0;
                    let mut n2: libc::c_int = 0;
                    let mut dice: libc::c_int = 0;
                    let mut side: libc::c_int = 0;
                    let mut mdice: libc::c_char = 0;
                    let mut mside: libc::c_char = 0;
                    /* Process 'B' for "Blows" (up to four lines) */
                    /* Oops, no more slots */
                    if blow_num as libc::c_int == 4 as libc::c_int {
                        return 1 as libc::c_int
                    }
                    /* Analyze the first field */
                    t = buf.offset(2 as libc::c_int as isize);
                    s = t;
                    while *t as libc::c_int != 0 &&
                              *t as libc::c_int != ':' as i32 {
                        /* loop */
                        t = t.offset(1)
                    }
                    /* Terminate the field (if necessary) */
                    if *t as libc::c_int == ':' as i32 {
                        let fresh61 = t;
                        t = t.offset(1);
                        *fresh61 = '\u{0}' as i32 as libc::c_char
                    }
                    /* Analyze the method */
                    n1 = 0 as libc::c_int;
                    while !r_info_blow_method[n1 as usize].is_null() {
                        if streq(s as cptr, r_info_blow_method[n1 as usize])
                               != 0 {
                            break ;
                        }
                        n1 += 1
                    }
                    /* Invalid method */
                    if r_info_blow_method[n1 as usize].is_null() {
                        return 1 as libc::c_int
                    }
                    /* Analyze the second field */
                    s = t;
                    while *t as libc::c_int != 0 &&
                              *t as libc::c_int != ':' as i32 {
                        /* loop */
                        t = t.offset(1)
                    }
                    /* Terminate the field (if necessary) */
                    if *t as libc::c_int == ':' as i32 {
                        let fresh62 = t;
                        t = t.offset(1);
                        *fresh62 = '\u{0}' as i32 as libc::c_char
                    }
                    /* Analyze effect */
                    n2 = 0 as libc::c_int;
                    while !r_info_blow_effect[n2 as usize].is_null() {
                        if streq(s as cptr, r_info_blow_effect[n2 as usize])
                               != 0 {
                            break ;
                        }
                        n2 += 1
                    }
                    /* Invalid effect */
                    if r_info_blow_effect[n2 as usize].is_null() {
                        return 1 as libc::c_int
                    }
                    /* Save the method */
                    (*re_ptr).blow[blow_num as usize].method =
                        n1 as byte_hack;
                    /* Save the effect */
                    (*re_ptr).blow[blow_num as usize].effect =
                        n2 as byte_hack;
                    /* Extract the damage dice and sides */
                    if 4 as libc::c_int !=
                           sscanf(t,
                                  b"%c%dd%c%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut mdice as *mut libc::c_char,
                                  &mut dice as *mut libc::c_int,
                                  &mut mside as *mut libc::c_char,
                                  &mut side as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    (*re_ptr).blow[blow_num as usize].d_dice =
                        dice as byte_hack;
                    (*re_ptr).blow[blow_num as usize].d_side =
                        side as byte_hack;
                    (*re_ptr).blowm[blow_num as
                                        usize][0 as libc::c_int as usize] =
                        monster_ego_modify(mdice);
                    (*re_ptr).blowm[blow_num as
                                        usize][1 as libc::c_int as usize] =
                        monster_ego_modify(mside);
                    blow_num = blow_num.wrapping_add(1)
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'F' as i32 {
                    let mut r_char: libc::c_char = 0;
                    /* Process 'F' for "Flags monster must have" (multiple lines) */
                    /* Parse every entry */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Continue */
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh63 = t;
                            t = t.offset(1);
                            *fresh63 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* XXX XXX XXX Hack -- Read monster symbols */
                        if 1 as libc::c_int ==
                               sscanf(s,
                                      b"R_CHAR_%c\x00" as *const u8 as
                                          *const libc::c_char,
                                      &mut r_char as *mut libc::c_char) {
                            /* Limited to 5 races */
                            if r_char_number >= 5 as libc::c_int {
                                continue ;
                            }
                            /* Extract a "frequency" */
                            let fresh64 = r_char_number;
                            r_char_number = r_char_number + 1;
                            (*re_ptr).r_char[fresh64 as usize] = r_char;
                            /* Start at next entry */
                            s = t
                        } else {
                            /* Parse this entry */
                            if 0 as libc::c_int !=
                                   grab_one_ego_flag(re_ptr, s as cptr,
                                                     1 as libc::c_int as
                                                         bool_) {
                                return 5 as libc::c_int
                            }
                            /* Start the next entry */
                            s = t
                        }
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'H' as i32 {
                    let mut r_char_0: libc::c_char = 0;
                    /* Process 'H' for "Flags monster must not have" (multiple lines) */
                    /* Parse every entry */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Continue */
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh65 = t;
                            t = t.offset(1);
                            *fresh65 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* XXX XXX XXX Hack -- Read monster symbols */
                        if 1 as libc::c_int ==
                               sscanf(s,
                                      b"R_CHAR_%c\x00" as *const u8 as
                                          *const libc::c_char,
                                      &mut r_char_0 as *mut libc::c_char) {
                            /* Limited to 5 races */
                            if nr_char_number >= 5 as libc::c_int {
                                continue ;
                            }
                            /* Extract a "frequency" */
                            let fresh66 = nr_char_number;
                            nr_char_number = nr_char_number + 1;
                            (*re_ptr).nr_char[fresh66 as usize] = r_char_0;
                            /* Start at next entry */
                            s = t
                        } else {
                            /* Parse this entry */
                            if 0 as libc::c_int !=
                                   grab_one_ego_flag(re_ptr, s as cptr,
                                                     0 as libc::c_int as
                                                         bool_) {
                                return 5 as libc::c_int
                            }
                            /* Start the next entry */
                            s = t
                        }
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'M' as i32 {
                    /* Process 'M' for "Basic Monster Flags" (multiple lines) */
                    /* Parse every entry */
                    s = buf.offset(2 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh67 = t;
                            t = t.offset(1);
                            *fresh67 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_basic_ego_flag(re_ptr, s as cptr,
                                                       1 as libc::c_int as
                                                           bool_) {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'O' as i32 {
                    /* Process 'O' for "Basic Monster -Flags" (multiple lines) */
                    /* Parse every entry */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Continue */
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh68 = t;
                            t = t.offset(1);
                            *fresh68 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* XXX XXX XXX Hack -- Read no flags */
                        if strcmp(s,
                                  b"MF_ALL\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                            /* No flags */
                            (*re_ptr).nflags9 = 0xffffffff as libc::c_uint;
                            (*re_ptr).nflags8 = (*re_ptr).nflags9;
                            (*re_ptr).nflags7 = (*re_ptr).nflags8;
                            (*re_ptr).nflags3 = (*re_ptr).nflags7;
                            (*re_ptr).nflags2 = (*re_ptr).nflags3;
                            (*re_ptr).nflags1 = (*re_ptr).nflags2;
                            /* Start at next entry */
                            s = t
                        } else {
                            /* Parse this entry */
                            if 0 as libc::c_int !=
                                   grab_one_basic_ego_flag(re_ptr, s as cptr,
                                                           0 as libc::c_int as
                                                               bool_) {
                                return 5 as libc::c_int
                            }
                            /* Start the next entry */
                            s = t
                        }
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'S' as i32 {
                    /* Process 'S' for "Spell Flags" (multiple lines) */
                    /* Parse every entry */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Continue */
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh69 = t;
                            t = t.offset(1);
                            *fresh69 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* XXX XXX XXX Hack -- Read spell frequency */
                        if 1 as libc::c_int ==
                               sscanf(s,
                                      b"1_IN_%d\x00" as *const u8 as
                                          *const libc::c_char,
                                      &mut i as *mut libc::c_int) {
                            /* Extract a "frequency" */
                            (*re_ptr).freq_inate =
                                (100 as libc::c_int / i) as byte_hack;
                            (*re_ptr).freq_spell = (*re_ptr).freq_inate;
                            /* Start at next entry */
                            s = t
                        } else {
                            /* Parse this entry */
                            if 0 as libc::c_int !=
                                   grab_one_spell_ego_flag(re_ptr, s as cptr,
                                                           1 as libc::c_int as
                                                               bool_) {
                                return 5 as libc::c_int
                            }
                            /* Start the next entry */
                            s = t
                        }
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'T' as i32 {
                    /* Process 'T' for "Spell -Flags" (multiple lines) */
                    /* Parse every entry */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Continue */
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh70 = t;
                            t = t.offset(1);
                            *fresh70 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* XXX XXX XXX Hack -- Read no flags */
                        if strcmp(s,
                                  b"MF_ALL\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                            /* No flags */
                            (*re_ptr).nflags6 = 0xffffffff as libc::c_uint;
                            (*re_ptr).nflags5 = (*re_ptr).nflags6;
                            (*re_ptr).nflags4 = (*re_ptr).nflags5;
                            /* Start at next entry */
                            s = t
                        } else {
                            /* Parse this entry */
                            if 0 as libc::c_int !=
                                   grab_one_spell_ego_flag(re_ptr, s as cptr,
                                                           0 as libc::c_int as
                                                               bool_) {
                                return 5 as libc::c_int
                            }
                            /* Start the next entry */
                            s = t
                        }
                    }
                } else {
                    /* Oops */
                    return 6 as libc::c_int
                }
            }
        }
    }
    /* Complete the "name" and "text" sizes */
    (*re_head).name_size = (*re_head).name_size.wrapping_add(1);
    /* No version yet */
    if okay == 0 { return 2 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Grab one flag in an trap_type from a textual string
 */
unsafe extern "C" fn grab_one_trap_type_flag(mut t_ptr: *mut trap_type,
                                             mut what: cptr) -> errr {
    let mut i: s16b = 0;
    /* Check flags1 */
    i = 0 as libc::c_int as s16b;
    while (i as libc::c_int) < 32 as libc::c_int {
        if streq(what, t_info_flags[i as usize]) != 0 {
            (*t_ptr).flags =
                ((*t_ptr).flags as libc::c_long |
                     (1 as libc::c_long) << i as libc::c_int) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Oops */
    msg_format(b"Unknown trap_type flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Error */
    return 1 as libc::c_int;
}
/*
 * Initialize the "tr_info" array, by parsing an ascii "template" file
 */
#[no_mangle]
pub unsafe extern "C" fn init_t_info_txt(mut fp: *mut FILE,
                                         mut buf: *mut libc::c_char) -> errr {
    let mut i: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Not ready yet */
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    /* Current entry */
    let mut t_ptr: *mut trap_type = 0 as *mut trap_type;
    /* Just before the first record */
    error_idx = -(1 as libc::c_int) as s16b;
    /* Just before the first line */
    error_line = -(1 as libc::c_int) as s16b;
    /* Prepare the "fake" stuff */
    (*t_head).name_size = 0 as libc::c_int as u32b;
    (*t_head).text_size = 0 as libc::c_int as u32b;
    /* Parse */
    fp_stack_init(fp);
    /* Next... */
    while 0 as libc::c_int == my_fgets_dostack(buf, 1024 as libc::c_int) {
        /* Advance the line number */
        error_line += 1;
        /* Skip comments and blank lines */
        if *buf.offset(0 as libc::c_int as isize) == 0 ||
               *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '#' as i32 {
            continue ;
        }
        /* Verify correct "colon" format */
        if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
           {
            return 1 as libc::c_int
        }
        /* Hack -- Process 'V' for "Version" */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'V' as i32
           {
            let mut v1: libc::c_int = 0;
            let mut v2: libc::c_int = 0;
            let mut v3: libc::c_int = 0;
            /* Scan for the values */
            if 3 as libc::c_int !=
                   sscanf(buf.offset(2 as libc::c_int as isize),
                          b"%d.%d.%d\x00" as *const u8 as *const libc::c_char,
                          &mut v1 as *mut libc::c_int,
                          &mut v2 as *mut libc::c_int,
                          &mut v3 as *mut libc::c_int) {
                return 2 as libc::c_int
            }
            /* Okay to proceed */
            okay = 1 as libc::c_int as bool_
        } else {
            /* No version yet */
            if okay == 0 { return 2 as libc::c_int }
            /* Included file */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '<' as i32 {
                fp_stack_push(buf.offset(2 as libc::c_int as isize) as cptr);
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'N' as i32 {
                /* Process 'N' for "New/Number/Name" */
                /* Find the colon before the name */
                s = strchr(buf.offset(2 as libc::c_int as isize), ':' as i32);
                /* Verify that colon */
                if s.is_null() { return 1 as libc::c_int }
                /* Nuke the colon, advance to the name */
                let fresh71 = s;
                s = s.offset(1);
                *fresh71 = '\u{0}' as i32 as libc::c_char;
                /* Paranoia -- require a name */
                if *s == 0 { return 1 as libc::c_int }
                /* Get the index */
                i = atoi(buf.offset(2 as libc::c_int as isize));
                /* Verify information */
                if i <= error_idx as libc::c_int { return 4 as libc::c_int }
                /* Verify information */
                if i >= (*t_head).info_num as libc::c_int {
                    return 2 as libc::c_int
                }
                /* Save the index */
                error_idx = i as s16b;
                /* Point at the "info" */
                t_ptr = &mut *t_info.offset(i as isize) as *mut trap_type;
                /* Hack -- Verify space */
                if ((*t_head).name_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_name_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the name index */
                if (*t_ptr).name == 0 {
                    (*t_head).name_size = (*t_head).name_size.wrapping_add(1);
                    (*t_ptr).name = (*t_head).name_size as s16b
                }
                /* Append chars to the name */
                strcpy(t_name.offset((*t_head).name_size as isize), s);
                /* Advance the index */
                (*t_head).name_size =
                    ((*t_head).name_size as
                         libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                        u32b
            } else {
                /* There better be a current t_ptr */
                if t_ptr.is_null() { return 3 as libc::c_int }
                /* Process 'I' for "Information" */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'I' as i32 {
                    let mut probability: libc::c_int = 0;
                    let mut another: libc::c_int = 0;
                    let mut p1valinc: libc::c_int = 0;
                    let mut difficulty: libc::c_int = 0;
                    let mut minlevel: libc::c_int = 0;
                    let mut dd: libc::c_int = 0;
                    let mut ds: libc::c_int = 0;
                    let mut color: libc::c_char = 0;
                    /* Scan for the values */
                    if 8 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d:%d:%d:%dd%d:%c\x00" as *const u8
                                      as *const libc::c_char,
                                  &mut difficulty as *mut libc::c_int,
                                  &mut probability as *mut libc::c_int,
                                  &mut another as *mut libc::c_int,
                                  &mut p1valinc as *mut libc::c_int,
                                  &mut minlevel as *mut libc::c_int,
                                  &mut dd as *mut libc::c_int,
                                  &mut ds as *mut libc::c_int,
                                  &mut color as *mut libc::c_char) {
                        return 1 as libc::c_int
                    }
                    (*t_ptr).difficulty = difficulty as byte_hack;
                    (*t_ptr).probability = probability as s16b;
                    (*t_ptr).another = another as s16b;
                    (*t_ptr).p1valinc = p1valinc as s16b;
                    (*t_ptr).minlevel = minlevel as byte_hack;
                    (*t_ptr).dd = dd as s16b;
                    (*t_ptr).ds = ds as s16b;
                    (*t_ptr).color = color_char_to_attr(color) as byte_hack
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'D' as i32 {
                    /* Process 'D' for "Description" */
                    /* Acquire the text */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Hack -- Verify space */
                    if ((*t_head).text_size as
                            libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                           > fake_text_size as libc::c_ulong {
                        return 7 as libc::c_int
                    }
                    /* Advance and Save the text index */
                    if (*t_ptr).text == 0 {
                        (*t_head).text_size =
                            (*t_head).text_size.wrapping_add(1);
                        (*t_ptr).text = (*t_head).text_size as s16b
                    }
                    /* Append chars to the name */
                    strcpy(t_text.offset((*t_head).text_size as isize), s);
                    /* Advance the index */
                    (*t_head).text_size =
                        ((*t_head).text_size as
                             libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                            u32b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'F' as i32 {
                    (*t_ptr).flags = 0 as libc::c_int as u32b;
                    /* Hack -- Process 'F' for flags */
                    /* Parse every entry textually */
                    s = buf.offset(2 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh72 = t;
                            t = t.offset(1);
                            *fresh72 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_trap_type_flag(t_ptr, s as cptr) {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else {
                    /* Oops */
                    return 6 as libc::c_int
                }
            }
        }
    }
    /* Complete the "name" and "text" sizes */
    (*t_head).name_size = (*t_head).name_size.wrapping_add(1);
    (*t_head).text_size = (*t_head).text_size.wrapping_add(1);
    /* No version yet */
    if okay == 0 { return 2 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Grab one flag for a dungeon type from a textual string
 */
#[no_mangle]
pub unsafe extern "C" fn grab_one_dungeon_flag(mut flags1: *mut u32b,
                                               mut flags2: *mut u32b,
                                               mut what: cptr) -> errr {
    let mut i: libc::c_int = 0;
    /* Scan flags1 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, d_info_flags1[i as usize]) != 0 {
            *flags1 =
                (*flags1 as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags2 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, d_info_flags2[i as usize]) != 0 {
            *flags2 =
                (*flags2 as libc::c_long | (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Oops */
    msg_format(b"Unknown dungeon type flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Failure */
    return 1 as libc::c_int;
}
/*
 * Grab one (basic) flag in a monster_race from a textual string
 */
unsafe extern "C" fn grab_one_basic_monster_flag(mut d_ptr:
                                                     *mut dungeon_info_type,
                                                 mut what: cptr,
                                                 mut rule: byte_hack)
 -> errr {
    let mut i: libc::c_int = 0;
    /* Scan flags1 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags1[i as usize]) != 0 {
            (*d_ptr).rules[rule as usize].mflags1 =
                ((*d_ptr).rules[rule as usize].mflags1 as libc::c_long |
                     (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags2 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags2[i as usize]) != 0 {
            (*d_ptr).rules[rule as usize].mflags2 =
                ((*d_ptr).rules[rule as usize].mflags2 as libc::c_long |
                     (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags3 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags3[i as usize]) != 0 {
            (*d_ptr).rules[rule as usize].mflags3 =
                ((*d_ptr).rules[rule as usize].mflags3 as libc::c_long |
                     (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags7 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags7[i as usize]) != 0 {
            (*d_ptr).rules[rule as usize].mflags7 =
                ((*d_ptr).rules[rule as usize].mflags7 as libc::c_long |
                     (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags8 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags8[i as usize]) != 0 {
            (*d_ptr).rules[rule as usize].mflags8 =
                ((*d_ptr).rules[rule as usize].mflags8 as libc::c_long |
                     (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags9 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags9[i as usize]) != 0 {
            (*d_ptr).rules[rule as usize].mflags9 =
                ((*d_ptr).rules[rule as usize].mflags9 as libc::c_long |
                     (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Oops */
    msg_format(b"Unknown monster flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Failure */
    return 1 as libc::c_int;
}
/*
 * Grab one (spell) flag in a monster_race from a textual string
 */
unsafe extern "C" fn grab_one_spell_monster_flag(mut d_ptr:
                                                     *mut dungeon_info_type,
                                                 mut what: cptr,
                                                 mut rule: byte_hack)
 -> errr {
    let mut i: libc::c_int = 0;
    /* Scan flags4 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags4[i as usize]) != 0 {
            (*d_ptr).rules[rule as usize].mflags4 =
                ((*d_ptr).rules[rule as usize].mflags4 as libc::c_long |
                     (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags5 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags5[i as usize]) != 0 {
            (*d_ptr).rules[rule as usize].mflags5 =
                ((*d_ptr).rules[rule as usize].mflags5 as libc::c_long |
                     (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Scan flags6 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, r_info_flags6[i as usize]) != 0 {
            (*d_ptr).rules[rule as usize].mflags6 =
                ((*d_ptr).rules[rule as usize].mflags6 as libc::c_long |
                     (1 as libc::c_long) << i) as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Oops */
    msg_format(b"Unknown monster flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Failure */
    return 1 as libc::c_int;
}
/*
 * Initialize the "d_info" array, by parsing an ascii "template" file
 */
#[no_mangle]
pub unsafe extern "C" fn init_d_info_txt(mut fp: *mut FILE,
                                         mut buf: *mut libc::c_char) -> errr {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut rule_num: s16b = 0 as libc::c_int as s16b;
    let mut r_char_number: byte_hack = 0 as libc::c_int as byte_hack;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Not ready yet */
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    /* Current entry */
    let mut d_ptr: *mut dungeon_info_type = 0 as *mut dungeon_info_type;
    /* Just before the first record */
    error_idx = -(1 as libc::c_int) as s16b;
    /* Just before the first line */
    error_line = -(1 as libc::c_int) as s16b;
    /* Start the "fake" stuff */
    (*d_head).name_size = 0 as libc::c_int as u32b;
    (*d_head).text_size = 0 as libc::c_int as u32b;
    /* Parse */
    fp_stack_init(fp);
    /* Next... */
    while 0 as libc::c_int == my_fgets_dostack(buf, 1024 as libc::c_int) {
        /* Advance the line number */
        error_line += 1;
        /* Skip comments and blank lines */
        if *buf.offset(0 as libc::c_int as isize) == 0 ||
               *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '#' as i32 {
            continue ;
        }
        /* Verify correct "colon" format */
        if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
           {
            return 1 as libc::c_int
        }
        /* Hack -- Process 'V' for "Version" */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'V' as i32
           {
            let mut v1: libc::c_int = 0;
            let mut v2: libc::c_int = 0;
            let mut v3: libc::c_int = 0;
            /* Scan for the values */
            if 3 as libc::c_int !=
                   sscanf(buf.offset(2 as libc::c_int as isize),
                          b"%d.%d.%d\x00" as *const u8 as *const libc::c_char,
                          &mut v1 as *mut libc::c_int,
                          &mut v2 as *mut libc::c_int,
                          &mut v3 as *mut libc::c_int) {
                return 2 as libc::c_int
            }
            /* Okay to proceed */
            okay = 1 as libc::c_int as bool_
        } else {
            /* No version yet */
            if okay == 0 { return 2 as libc::c_int }
            /* Included file */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '<' as i32 {
                fp_stack_push(buf.offset(2 as libc::c_int as isize) as cptr);
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'N' as i32 {
                /* Process 'N' for "New/Number/Name" */
                /* Find the colon before the name */
                s = strchr(buf.offset(2 as libc::c_int as isize), ':' as i32);
                /* Verify that colon */
                if s.is_null() { return 1 as libc::c_int }
                /* Nuke the colon, advance to the name */
                let fresh73 = s;
                s = s.offset(1);
                *fresh73 = '\u{0}' as i32 as libc::c_char;
                /* Paranoia -- require a name */
                if *s == 0 { return 1 as libc::c_int }
                /* Get the index */
                i = atoi(buf.offset(2 as libc::c_int as isize));
                /* Verify information */
                if i < error_idx as libc::c_int { return 4 as libc::c_int }
                /* Verify information */
                if i >= (*d_head).info_num as libc::c_int {
                    return 2 as libc::c_int
                }
                /* Save the index */
                error_idx = i as s16b;
                /* Point at the "info" */
                d_ptr =
                    &mut *d_info.offset(i as isize) as *mut dungeon_info_type;
                /* Hack -- Verify space */
                if ((*d_head).name_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_name_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the name index */
                if (*d_ptr).name == 0 {
                    (*d_head).name_size = (*d_head).name_size.wrapping_add(1);
                    (*d_ptr).name = (*d_head).name_size
                }
                /* Append chars to the name */
                strcpy(d_name.offset((*d_head).name_size as isize), s);
                /* Advance the index */
                (*d_head).name_size =
                    ((*d_head).name_size as
                         libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                        u32b;
                /* HACK -- Those ones HAVE to have a set default value */
                (*d_ptr).size_x = -(1 as libc::c_int);
                (*d_ptr).size_y = -(1 as libc::c_int);
                (*d_ptr).ix = -(1 as libc::c_int);
                (*d_ptr).iy = -(1 as libc::c_int);
                (*d_ptr).ox = -(1 as libc::c_int);
                (*d_ptr).oy = -(1 as libc::c_int);
                (*d_ptr).fill_method = 1 as libc::c_int as byte_hack;
                rule_num = -(1 as libc::c_int) as s16b;
                r_char_number = 0 as libc::c_int as byte_hack;
                j = 0 as libc::c_int;
                while j < 5 as libc::c_int {
                    let mut k: libc::c_int = 0;
                    (*d_ptr).rules[j as usize].mode =
                        0 as libc::c_int as byte_hack;
                    (*d_ptr).rules[j as usize].percent =
                        0 as libc::c_int as byte_hack;
                    k = 0 as libc::c_int;
                    while k < 5 as libc::c_int {
                        (*d_ptr).rules[j as usize].r_char[k as usize] =
                            0 as libc::c_int as libc::c_char;
                        k += 1
                    }
                    j += 1
                }
                /* HACK -- Those ones HAVE to have a set default value */
                (*d_ptr).objs.treasure = 20 as libc::c_int as byte_hack;
                (*d_ptr).objs.combat = 20 as libc::c_int as byte_hack;
                (*d_ptr).objs.magic = 20 as libc::c_int as byte_hack;
                (*d_ptr).objs.tools = 20 as libc::c_int as byte_hack;
                /* The default generator */
                strcpy((*d_ptr).generator.as_mut_ptr(),
                       b"dungeon\x00" as *const u8 as *const libc::c_char);
            } else {
                /* There better be a current d_ptr */
                if d_ptr.is_null() { return 3 as libc::c_int }
                /* Process 'D' for "Description */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'D' as i32 {
                    /* Acquire short name */
                    (*d_ptr).short_name[0 as libc::c_int as usize] =
                        *buf.offset(2 as libc::c_int as isize);
                    (*d_ptr).short_name[1 as libc::c_int as usize] =
                        *buf.offset(3 as libc::c_int as isize);
                    (*d_ptr).short_name[2 as libc::c_int as usize] =
                        *buf.offset(4 as libc::c_int as isize);
                    /* Acquire the text */
                    s = buf.offset(6 as libc::c_int as isize);
                    /* Hack -- Verify space */
                    if ((*d_head).text_size as
                            libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                           > fake_text_size as libc::c_ulong {
                        return 7 as libc::c_int
                    }
                    /* Advance and Save the text index */
                    if (*d_ptr).text == 0 {
                        (*d_head).text_size =
                            (*d_head).text_size.wrapping_add(1);
                        (*d_ptr).text = (*d_head).text_size
                    }
                    /* Append chars to the name */
                    strcpy(d_text.offset((*d_head).text_size as isize), s);
                    /* Advance the index */
                    (*d_head).text_size =
                        ((*d_head).text_size as
                             libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                            u32b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'W' as i32 {
                    let mut min_lev: libc::c_int = 0;
                    let mut max_lev: libc::c_int = 0;
                    let mut min_plev: libc::c_int = 0;
                    let mut next: libc::c_int = 0;
                    let mut min_alloc: libc::c_int = 0;
                    let mut max_chance: libc::c_int = 0;
                    /* Process 'W' for "More Info" (one line only) */
                    /* Scan for the values */
                    if 6 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d:%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut min_lev as *mut libc::c_int,
                                  &mut max_lev as *mut libc::c_int,
                                  &mut min_plev as *mut libc::c_int,
                                  &mut next as *mut libc::c_int,
                                  &mut min_alloc as *mut libc::c_int,
                                  &mut max_chance as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*d_ptr).mindepth = min_lev as s16b;
                    (*d_ptr).maxdepth = max_lev as s16b;
                    (*d_ptr).min_plev = min_plev as byte_hack;
                    (*d_ptr).next = next as byte_hack;
                    (*d_ptr).min_m_alloc_level = min_alloc;
                    (*d_ptr).max_m_alloc_chance = max_chance
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'L' as i32 {
                    let mut f1: libc::c_int = 0;
                    let mut f2: libc::c_int = 0;
                    let mut f3: libc::c_int = 0;
                    let mut p1: libc::c_int = 0;
                    let mut p2: libc::c_int = 0;
                    let mut p3: libc::c_int = 0;
                    let mut i_0: libc::c_int = 0;
                    /* Process 'L' for "fLoor type" (one line only) */
                    /* Scan for the values */
                    if 6 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d:%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut f1 as *mut libc::c_int,
                                  &mut p1 as *mut libc::c_int,
                                  &mut f2 as *mut libc::c_int,
                                  &mut p2 as *mut libc::c_int,
                                  &mut f3 as *mut libc::c_int,
                                  &mut p3 as *mut libc::c_int) {
                        /* Scan for the values - part ii*/
                        if 3 as libc::c_int !=
                               sscanf(buf.offset(2 as libc::c_int as isize),
                                      b"%d:%d:%d\x00" as *const u8 as
                                          *const libc::c_char,
                                      &mut p1 as *mut libc::c_int,
                                      &mut p2 as *mut libc::c_int,
                                      &mut p3 as *mut libc::c_int) {
                            return 1 as libc::c_int
                        }
                        /* Save the values */
                        (*d_ptr).floor_percent1[1 as libc::c_int as usize] =
                            p1 as byte_hack;
                        (*d_ptr).floor_percent2[1 as libc::c_int as usize] =
                            p2 as byte_hack;
                        (*d_ptr).floor_percent3[1 as libc::c_int as usize] =
                            p3 as byte_hack
                    } else {
                        /* Save the values */
                        (*d_ptr).floor1 = f1 as s16b;
                        (*d_ptr).floor2 = f2 as s16b;
                        (*d_ptr).floor3 = f3 as s16b;
                        i_0 = 0 as libc::c_int;
                        while i_0 < 2 as libc::c_int {
                            (*d_ptr).floor_percent1[i_0 as usize] =
                                p1 as byte_hack;
                            (*d_ptr).floor_percent2[i_0 as usize] =
                                p2 as byte_hack;
                            (*d_ptr).floor_percent3[i_0 as usize] =
                                p3 as byte_hack;
                            i_0 += 1
                        }
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'O' as i32 {
                    let mut treasure: libc::c_int = 0;
                    let mut combat: libc::c_int = 0;
                    let mut magic: libc::c_int = 0;
                    let mut tools: libc::c_int = 0;
                    /* Process 'O' for "Object type" (one line only) */
                    /* Scan for the values */
                    if 4 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut treasure as *mut libc::c_int,
                                  &mut combat as *mut libc::c_int,
                                  &mut magic as *mut libc::c_int,
                                  &mut tools as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*d_ptr).objs.treasure = treasure as byte_hack;
                    (*d_ptr).objs.combat = combat as byte_hack;
                    (*d_ptr).objs.magic = magic as byte_hack;
                    (*d_ptr).objs.tools = tools as byte_hack
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'G' as i32 {
                    strnfmt((*d_ptr).generator.as_mut_ptr(),
                            30 as libc::c_int as uint_hack,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            buf.offset(2 as libc::c_int as isize));
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'A' as i32 {
                    let mut w1: libc::c_int = 0;
                    let mut w2: libc::c_int = 0;
                    let mut w3: libc::c_int = 0;
                    let mut outer: libc::c_int = 0;
                    let mut inner: libc::c_int = 0;
                    let mut p1_0: libc::c_int = 0;
                    let mut p2_0: libc::c_int = 0;
                    let mut p3_0: libc::c_int = 0;
                    let mut i_1: libc::c_int = 0;
                    /* Process 'G' for "Generator" (one line only) */
                    /* Process 'A' for "wAll type" (one line only) */
                    /* Scan for the values */
                    if 8 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d:%d:%d:%d:%d:%d\x00" as *const u8
                                      as *const libc::c_char,
                                  &mut w1 as *mut libc::c_int,
                                  &mut p1_0 as *mut libc::c_int,
                                  &mut w2 as *mut libc::c_int,
                                  &mut p2_0 as *mut libc::c_int,
                                  &mut w3 as *mut libc::c_int,
                                  &mut p3_0 as *mut libc::c_int,
                                  &mut outer as *mut libc::c_int,
                                  &mut inner as *mut libc::c_int) {
                        /* Scan for the values - part ii*/
                        if 3 as libc::c_int !=
                               sscanf(buf.offset(2 as libc::c_int as isize),
                                      b"%d:%d:%d\x00" as *const u8 as
                                          *const libc::c_char,
                                      &mut p1_0 as *mut libc::c_int,
                                      &mut p2_0 as *mut libc::c_int,
                                      &mut p3_0 as *mut libc::c_int) {
                            return 1 as libc::c_int
                        }
                        /* Save the values */
                        (*d_ptr).fill_percent1[1 as libc::c_int as usize] =
                            p1_0 as byte_hack;
                        (*d_ptr).fill_percent2[1 as libc::c_int as usize] =
                            p2_0 as byte_hack;
                        (*d_ptr).fill_percent3[1 as libc::c_int as usize] =
                            p3_0 as byte_hack
                    } else {
                        /* Save the values */
                        (*d_ptr).fill_type1 = w1 as s16b;
                        (*d_ptr).fill_type2 = w2 as s16b;
                        (*d_ptr).fill_type3 = w3 as s16b;
                        i_1 = 0 as libc::c_int;
                        while i_1 < 2 as libc::c_int {
                            (*d_ptr).fill_percent1[i_1 as usize] =
                                p1_0 as byte_hack;
                            (*d_ptr).fill_percent2[i_1 as usize] =
                                p2_0 as byte_hack;
                            (*d_ptr).fill_percent3[i_1 as usize] =
                                p3_0 as byte_hack;
                            i_1 += 1
                        }
                        (*d_ptr).outer_wall = outer as s16b;
                        (*d_ptr).inner_wall = inner as s16b
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'E' as i32 {
                    let mut side: libc::c_int = 0;
                    let mut dice: libc::c_int = 0;
                    let mut freq: libc::c_int = 0;
                    let mut type_0: libc::c_int = 0;
                    let mut tmp: cptr = 0 as *const libc::c_char;
                    /* Process 'E' for "Effects" (up to four lines) -SC- */
                    /* Find the next empty blow slot (if any) */
                    i = 0 as libc::c_int;
                    while i < 4 as libc::c_int {
                        if (*d_ptr).d_side[i as usize] == 0 &&
                               (*d_ptr).d_dice[i as usize] == 0 {
                            break ;
                        }
                        i += 1
                    }
                    /* Oops, no more slots */
                    if i == 4 as libc::c_int { return 1 as libc::c_int }
                    /* Scan for the values */
                    if 4 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%dd%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut dice as *mut libc::c_int,
                                  &mut side as *mut libc::c_int,
                                  &mut freq as *mut libc::c_int,
                                  &mut type_0 as *mut libc::c_int) {
                        let mut j_0: libc::c_int = 0;
                        if 3 as libc::c_int !=
                               sscanf(buf.offset(2 as libc::c_int as isize),
                                      b"%dd%d:%d\x00" as *const u8 as
                                          *const libc::c_char,
                                      &mut dice as *mut libc::c_int,
                                      &mut side as *mut libc::c_int,
                                      &mut freq as *mut libc::c_int) {
                            return 1 as libc::c_int
                        }
                        tmp = buf.offset(2 as libc::c_int as isize) as cptr;
                        j_0 = 0 as libc::c_int;
                        while j_0 < 2 as libc::c_int {
                            tmp = strchr(tmp, ':' as i32) as cptr;
                            if tmp.is_null() { return 1 as libc::c_int }
                            tmp = tmp.offset(1);
                            j_0 += 1
                        }
                        j_0 = 0 as libc::c_int;
                        while !d_info_dtypes[j_0 as usize].name.is_null() {
                            if strcmp(d_info_dtypes[j_0 as usize].name, tmp)
                                   == 0 as libc::c_int {
                                (*d_ptr).d_type[i as usize] =
                                    d_info_dtypes[j_0 as usize].feat;
                                break ;
                            } else { j_0 += 1 }
                        }
                        if d_info_dtypes[j_0 as usize].name.is_null() {
                            return 1 as libc::c_int
                        }
                    } else { (*d_ptr).d_type[i as usize] = type_0 }
                    freq *= 10 as libc::c_int;
                    /* Save the values */
                    (*d_ptr).d_side[i as usize] = side;
                    (*d_ptr).d_dice[i as usize] = dice;
                    (*d_ptr).d_frequency[i as usize] = freq
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'F' as i32 {
                    let mut artif: libc::c_int = 0 as libc::c_int;
                    let mut monst: libc::c_int = 0 as libc::c_int;
                    let mut obj: libc::c_int = 0 as libc::c_int;
                    let mut ix: libc::c_int = -(1 as libc::c_int);
                    let mut iy: libc::c_int = -(1 as libc::c_int);
                    let mut ox: libc::c_int = -(1 as libc::c_int);
                    let mut oy: libc::c_int = -(1 as libc::c_int);
                    let mut fill_method: libc::c_int = 0;
                    /* Process 'F' for "Dungeon Flags" (multiple lines) */
                    /* Parse every entry */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Continue */
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh74 = t;
                            t = t.offset(1);
                            *fresh74 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Read dungeon in/out coords */
                        if 4 as libc::c_int ==
                               sscanf(s,
                                      b"WILD_%d_%d__%d_%d\x00" as *const u8 as
                                          *const libc::c_char,
                                      &mut ix as *mut libc::c_int,
                                      &mut iy as *mut libc::c_int,
                                      &mut ox as *mut libc::c_int,
                                      &mut oy as *mut libc::c_int) {
                            (*d_ptr).ix = ix;
                            (*d_ptr).iy = iy;
                            (*d_ptr).ox = ox;
                            (*d_ptr).oy = oy;
                            /* Start at next entry */
                            s = t
                        } else if 2 as libc::c_int ==
                                      sscanf(s,
                                             b"SIZE_%d_%d\x00" as *const u8 as
                                                 *const libc::c_char,
                                             &mut ix as *mut libc::c_int,
                                             &mut iy as *mut libc::c_int) {
                            (*d_ptr).size_x = ix;
                            (*d_ptr).size_y = iy;
                            /* Read dungeon size */
                            /* Start at next entry */
                            s = t
                        } else if 1 as libc::c_int ==
                                      sscanf(s,
                                             b"FILL_METHOD_%d\x00" as
                                                 *const u8 as
                                                 *const libc::c_char,
                                             &mut fill_method as
                                                 *mut libc::c_int) {
                            (*d_ptr).fill_method = fill_method as byte_hack;
                            /* Read dungeon fill method */
                            /* Start at next entry */
                            s = t
                        } else if 1 as libc::c_int ==
                                      sscanf(s,
                                             b"FINAL_OBJECT_%d\x00" as
                                                 *const u8 as
                                                 *const libc::c_char,
                                             &mut obj as *mut libc::c_int) {
                            /* Read Final Object */
                            /* Extract a "Final Artifact" */
                            (*d_ptr).final_object = obj;
                            /* Start at next entry */
                            s = t
                        } else if 1 as libc::c_int ==
                                      sscanf(s,
                                             b"FINAL_ARTIFACT_%d\x00" as
                                                 *const u8 as
                                                 *const libc::c_char,
                                             &mut artif as *mut libc::c_int) {
                            /* Read Final Artifact */
                            /* Extract a "Final Artifact" */
                            (*d_ptr).final_artifact = artif;
                            /* Start at next entry */
                            s = t
                        } else if 1 as libc::c_int ==
                                      sscanf(s,
                                             b"FINAL_GUARDIAN_%d\x00" as
                                                 *const u8 as
                                                 *const libc::c_char,
                                             &mut monst as *mut libc::c_int) {
                            /* Read Artifact Guardian */
                            /* Extract a "Artifact Guardian" */
                            (*d_ptr).final_guardian = monst;
                            /* Start at next entry */
                            s = t
                        } else {
                            /* Parse this entry */
                            if 0 as libc::c_int !=
                                   grab_one_dungeon_flag(&mut (*d_ptr).flags1,
                                                         &mut (*d_ptr).flags2,
                                                         s as cptr) {
                                return 5 as libc::c_int
                            }
                            /* Start the next entry */
                            s = t
                        }
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'R' as i32 {
                    let mut percent: libc::c_int = 0;
                    let mut mode: libc::c_int = 0;
                    let mut z: libc::c_int = 0;
                    let mut y: libc::c_int = 0;
                    let mut lims: [libc::c_int; 5] = [0; 5];
                    /* Process 'R' for "monster generation Rule" (up to 5 lines) */
                    /* Scan for the values */
                    if 2 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut percent as *mut libc::c_int,
                                  &mut mode as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    r_char_number = 0 as libc::c_int as byte_hack;
                    rule_num += 1;
                    (*d_ptr).rules[rule_num as usize].percent =
                        percent as byte_hack;
                    (*d_ptr).rules[rule_num as usize].mode =
                        mode as byte_hack;
                    /* Lets remap the flat percents */
                    lims[0 as libc::c_int as usize] =
                        (*d_ptr).rules[0 as libc::c_int as usize].percent as
                            libc::c_int;
                    y = 1 as libc::c_int;
                    while y <= rule_num as libc::c_int {
                        lims[y as usize] =
                            lims[(y - 1 as libc::c_int) as usize] +
                                (*d_ptr).rules[y as usize].percent as
                                    libc::c_int;
                        y += 1
                    }
                    z = 0 as libc::c_int;
                    while z < 100 as libc::c_int {
                        y = rule_num as libc::c_int;
                        while y >= 0 as libc::c_int {
                            if z < lims[y as usize] {
                                (*d_ptr).rule_percents[z as usize] =
                                    y as byte_hack
                            }
                            y -= 1
                        }
                        z += 1
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'M' as i32 {
                    let mut r_char: byte_hack = 0;
                    /* Process 'M' for "Basic Flags" (multiple lines) */
                    /* Parse every entry */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Continue */
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh75 = t;
                            t = t.offset(1);
                            *fresh75 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Read monster symbols */
                        if 1 as libc::c_int ==
                               sscanf(s,
                                      b"R_CHAR_%c\x00" as *const u8 as
                                          *const libc::c_char,
                                      &mut r_char as *mut byte_hack) {
                            /* Limited to 5 races */
                            if r_char_number as libc::c_int >=
                                   5 as libc::c_int {
                                continue ;
                            }
                            /* Extract a "frequency" */
                            let fresh76 = r_char_number;
                            r_char_number = r_char_number.wrapping_add(1);
                            (*d_ptr).rules[rule_num as
                                               usize].r_char[fresh76 as usize]
                                = r_char as libc::c_char;
                            /* Start at next entry */
                            s = t
                        } else {
                            /* Parse this entry */
                            if 0 as libc::c_int !=
                                   grab_one_basic_monster_flag(d_ptr,
                                                               s as cptr,
                                                               rule_num as
                                                                   byte_hack)
                               {
                                return 5 as libc::c_int
                            }
                            /* Start the next entry */
                            s = t
                        }
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'S' as i32 {
                    /* Process 'S' for "Spell Flags" (multiple lines) */
                    /* Parse every entry */
                    s = buf.offset(2 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh77 = t;
                            t = t.offset(1);
                            *fresh77 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_spell_monster_flag(d_ptr, s as cptr,
                                                           rule_num as
                                                               byte_hack) {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else {
                    /* Oops */
                    return 6 as libc::c_int
                }
            }
        }
    }
    /* Complete the "name" and "text" sizes */
    (*d_head).name_size = (*d_head).name_size.wrapping_add(1);
    (*d_head).text_size = (*d_head).text_size.wrapping_add(1);
    /* No version yet */
    if okay == 0 { return 2 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Grab one race flag from a textual string
 */
unsafe extern "C" fn grab_one_race_flag(mut ow_ptr: *mut owner_type,
                                        mut state: libc::c_int,
                                        mut what: cptr) -> errr {
    /* int i;
	cptr s; */
    /* Scan race flags */
    unknown_shut_up = 1 as libc::c_int as bool_;
    if grab_one_race_allow_flag((*ow_ptr).races[state as usize].as_mut_ptr(),
                                what) == 0 {
        unknown_shut_up = 0 as libc::c_int as bool_;
        return 0 as libc::c_int
    }
    /* Scan classes flags */
    if grab_one_class_flag((*ow_ptr).classes[state as usize].as_mut_ptr(),
                           what) == 0 {
        unknown_shut_up = 0 as libc::c_int as bool_;
        return 0 as libc::c_int
    }
    /* Oops */
    unknown_shut_up = 0 as libc::c_int as bool_;
    msg_format(b"Unknown race/class flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Failure */
    return 1 as libc::c_int;
}
/*
 * Grab one store flag from a textual string
 */
unsafe extern "C" fn grab_one_store_flag(mut st_ptr: *mut store_info_type,
                                         mut what: cptr) -> errr {
    let mut i: libc::c_int = 0;
    /* Scan store flags */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, st_info_flags1[i as usize]) != 0 {
            (*st_ptr).flags1 =
                ((*st_ptr).flags1 as libc::c_long | (1 as libc::c_long) << i)
                    as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Oops */
    msg_format(b"Unknown store flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Failure */
    return 1 as libc::c_int;
}
/*
 * Initialize the "st_info" array, by parsing an ascii "template" file
 */
#[no_mangle]
pub unsafe extern "C" fn init_st_info_txt(mut fp: *mut FILE,
                                          mut buf: *mut libc::c_char)
 -> errr {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut item_idx: libc::c_int = 0 as libc::c_int;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Not ready yet */
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    /* Current entry */
    let mut st_ptr: *mut store_info_type = 0 as *mut store_info_type;
    /* Just before the first record */
    error_idx = -(1 as libc::c_int) as s16b;
    /* Just before the first line */
    error_line = -(1 as libc::c_int) as s16b;
    /* Start the "fake" stuff */
    (*st_head).name_size = 0 as libc::c_int as u32b;
    (*st_head).text_size = 0 as libc::c_int as u32b;
    /* Parse */
    fp_stack_init(fp);
    /* Next... */
    while 0 as libc::c_int == my_fgets_dostack(buf, 1024 as libc::c_int) {
        /* Advance the line number */
        error_line += 1;
        /* Skip comments and blank lines */
        if *buf.offset(0 as libc::c_int as isize) == 0 ||
               *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '#' as i32 {
            continue ;
        }
        /* Verify correct "colon" format */
        if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
           {
            return 1 as libc::c_int
        }
        /* Hack -- Process 'V' for "Version" */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'V' as i32
           {
            let mut v1: libc::c_int = 0;
            let mut v2: libc::c_int = 0;
            let mut v3: libc::c_int = 0;
            /* Scan for the values */
            if 3 as libc::c_int !=
                   sscanf(buf.offset(2 as libc::c_int as isize),
                          b"%d.%d.%d\x00" as *const u8 as *const libc::c_char,
                          &mut v1 as *mut libc::c_int,
                          &mut v2 as *mut libc::c_int,
                          &mut v3 as *mut libc::c_int) {
                return 2 as libc::c_int
            }
            /* Okay to proceed */
            okay = 1 as libc::c_int as bool_
        } else {
            /* No version yet */
            if okay == 0 { return 2 as libc::c_int }
            /* Included file */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '<' as i32 {
                fp_stack_push(buf.offset(2 as libc::c_int as isize) as cptr);
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'N' as i32 {
                /* Process 'N' for "New/Number/Name" */
                /* Find the colon before the name */
                s = strchr(buf.offset(2 as libc::c_int as isize), ':' as i32);
                /* Verify that colon */
                if s.is_null() { return 1 as libc::c_int }
                /* Nuke the colon, advance to the name */
                let fresh78 = s;
                s = s.offset(1);
                *fresh78 = '\u{0}' as i32 as libc::c_char;
                /* Paranoia -- require a name */
                if *s == 0 { return 1 as libc::c_int }
                /* Get the index */
                i = atoi(buf.offset(2 as libc::c_int as isize));
                /* Verify information */
                if i < error_idx as libc::c_int { return 4 as libc::c_int }
                /* Verify information */
                if i >= (*st_head).info_num as libc::c_int {
                    return 2 as libc::c_int
                }
                /* Save the index */
                error_idx = i as s16b;
                /* Point at the "info" */
                st_ptr =
                    &mut *st_info.offset(i as isize) as *mut store_info_type;
                /* Hack -- Verify space */
                if ((*st_head).name_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_name_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the name index */
                if (*st_ptr).name == 0 {
                    (*st_head).name_size =
                        (*st_head).name_size.wrapping_add(1);
                    (*st_ptr).name = (*st_head).name_size
                }
                /* Append chars to the name */
                strcpy(st_name.offset((*st_head).name_size as isize), s);
                /* Advance the index */
                (*st_head).name_size =
                    ((*st_head).name_size as
                         libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                        u32b;
                /* We are ready for a new set of objects */
                item_idx = 0 as libc::c_int
            } else {
                /* There better be a current st_ptr */
                if st_ptr.is_null() { return 3 as libc::c_int }
                /* Process 'I' for "Items" (multiple lines) */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'I' as i32 {
                    /* Find the colon before the name */
                    s =
                        strchr(buf.offset(2 as libc::c_int as isize),
                               ':' as i32);
                    /* Verify that colon */
                    if s.is_null() { return 1 as libc::c_int }
                    /* Nuke the colon, advance to the name */
                    let fresh79 = s;
                    s = s.offset(1);
                    *fresh79 = '\u{0}' as i32 as libc::c_char;
                    /* Paranoia -- require a name */
                    if *s == 0 { return 1 as libc::c_int }
                    /* Get the index */
                    (*st_ptr).table[item_idx as
                                        usize][1 as libc::c_int as usize] =
                        atoi(buf.offset(2 as libc::c_int as isize)) as s16b;
                    /* Append chars to the name */
                    let fresh80 = item_idx;
                    item_idx = item_idx + 1;
                    (*st_ptr).table[fresh80 as
                                        usize][0 as libc::c_int as usize] =
                        test_item_name(s as cptr) as s16b;
                    (*st_ptr).table_num = item_idx as byte_hack
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'T' as i32 {
                    let mut tv1: libc::c_int = 0;
                    let mut sv1: libc::c_int = 0;
                    let mut rar1: libc::c_int = 0;
                    /* Process 'T' for "Tval/sval" */
                    /* Scan for the values */
                    if 3 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut rar1 as *mut libc::c_int,
                                  &mut tv1 as *mut libc::c_int,
                                  &mut sv1 as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Get the index */
                    (*st_ptr).table[item_idx as
                                        usize][1 as libc::c_int as usize] =
                        rar1 as s16b;
                    /* Hack -- 256 as a sval means all possible items */
                    let fresh81 = item_idx;
                    item_idx = item_idx + 1;
                    (*st_ptr).table[fresh81 as
                                        usize][0 as libc::c_int as usize] =
                        if sv1 < 256 as libc::c_int {
                            lookup_kind(tv1, sv1) as libc::c_int
                        } else { (tv1) + 10000 as libc::c_int } as s16b;
                    (*st_ptr).table_num = item_idx as byte_hack
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'G' as i32 {
                    let mut c: libc::c_char = 0;
                    let mut a: libc::c_char = 0;
                    let mut attr: libc::c_int = 0;
                    /* Process 'G' for "Graphics" one line only) */
                    /* Scan for the values */
                    if 2 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%c:%c\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut c as *mut libc::c_char,
                                  &mut a as *mut libc::c_char) {
                        return 1 as libc::c_int
                    }
                    /* Extract the color */
                    attr = color_char_to_attr(a);
                    /* Paranoia */
                    if attr < 0 as libc::c_int { return 1 as libc::c_int }
                    /* Save the values */
                    (*st_ptr).d_char = c;
                    (*st_ptr).d_attr = attr as byte_hack
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'A' as i32 {
                    let mut a1: libc::c_int = 0;
                    let mut a2: libc::c_int = 0;
                    let mut a3: libc::c_int = 0;
                    let mut a4: libc::c_int = 0;
                    let mut a5: libc::c_int = 0;
                    let mut a6: libc::c_int = 0;
                    /* Process 'A' for "Actions" (one line only) */
                    /* Scan for the values */
                    if 6 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d:%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut a1 as *mut libc::c_int,
                                  &mut a2 as *mut libc::c_int,
                                  &mut a3 as *mut libc::c_int,
                                  &mut a4 as *mut libc::c_int,
                                  &mut a5 as *mut libc::c_int,
                                  &mut a6 as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*st_ptr).actions[0 as libc::c_int as usize] = a1 as u16b;
                    (*st_ptr).actions[1 as libc::c_int as usize] = a2 as u16b;
                    (*st_ptr).actions[2 as libc::c_int as usize] = a3 as u16b;
                    (*st_ptr).actions[3 as libc::c_int as usize] = a4 as u16b;
                    (*st_ptr).actions[4 as libc::c_int as usize] = a5 as u16b;
                    (*st_ptr).actions[5 as libc::c_int as usize] = a6 as u16b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'F' as i32 {
                    /* Process 'F' for "store Flags" (multiple lines) */
                    /* Parse every entry */
                    s = buf.offset(2 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh82 = t;
                            t = t.offset(1);
                            *fresh82 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_store_flag(st_ptr, s as cptr) {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'O' as i32 {
                    let mut a1_0: libc::c_int = 0;
                    let mut a2_0: libc::c_int = 0;
                    let mut a3_0: libc::c_int = 0;
                    let mut a4_0: libc::c_int = 0;
                    /* Process 'O' for "Owners" (one line only) */
                    /* Scan for the values */
                    if 4 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut a1_0 as *mut libc::c_int,
                                  &mut a2_0 as *mut libc::c_int,
                                  &mut a3_0 as *mut libc::c_int,
                                  &mut a4_0 as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*st_ptr).owners[0 as libc::c_int as usize] =
                        a1_0 as u16b;
                    (*st_ptr).owners[1 as libc::c_int as usize] =
                        a2_0 as u16b;
                    (*st_ptr).owners[2 as libc::c_int as usize] =
                        a3_0 as u16b;
                    (*st_ptr).owners[3 as libc::c_int as usize] = a4_0 as u16b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'W' as i32 {
                    let mut max_obj: libc::c_int = 0;
                    /* Process 'W' for "Extra info" (one line only) */
                    /* Scan for the values */
                    if 1 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut max_obj as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    if max_obj > 255 as libc::c_int {
                        max_obj = 255 as libc::c_int
                    }
                    (*st_ptr).max_obj = max_obj as s16b
                } else {
                    /* Oops */
                    return 6 as libc::c_int
                }
            }
        }
    }
    /* Complete the "name" and "text" sizes */
    (*st_head).name_size = (*st_head).name_size.wrapping_add(1);
    (*st_head).text_size = (*st_head).text_size.wrapping_add(1);
    /* No version yet */
    if okay == 0 { return 2 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialize the "ba_info" array, by parsing an ascii "template" file
 */
#[no_mangle]
pub unsafe extern "C" fn init_ba_info_txt(mut fp: *mut FILE,
                                          mut buf: *mut libc::c_char)
 -> errr {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Not ready yet */
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    /* Current entry */
    let mut ba_ptr: *mut store_action_type = 0 as *mut store_action_type;
    /* Just before the first record */
    error_idx = -(1 as libc::c_int) as s16b;
    /* Just before the first line */
    error_line = -(1 as libc::c_int) as s16b;
    /* Start the "fake" stuff */
    (*ba_head).name_size = 0 as libc::c_int as u32b;
    (*ba_head).text_size = 0 as libc::c_int as u32b;
    /* Parse */
    fp_stack_init(fp);
    /* Next... */
    while 0 as libc::c_int == my_fgets_dostack(buf, 1024 as libc::c_int) {
        /* Advance the line number */
        error_line += 1;
        /* Skip comments and blank lines */
        if *buf.offset(0 as libc::c_int as isize) == 0 ||
               *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '#' as i32 {
            continue ;
        }
        /* Verify correct "colon" format */
        if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
           {
            return 1 as libc::c_int
        }
        /* Hack -- Process 'V' for "Version" */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'V' as i32
           {
            let mut v1: libc::c_int = 0;
            let mut v2: libc::c_int = 0;
            let mut v3: libc::c_int = 0;
            /* Scan for the values */
            if 3 as libc::c_int !=
                   sscanf(buf.offset(2 as libc::c_int as isize),
                          b"%d.%d.%d\x00" as *const u8 as *const libc::c_char,
                          &mut v1 as *mut libc::c_int,
                          &mut v2 as *mut libc::c_int,
                          &mut v3 as *mut libc::c_int) {
                return 2 as libc::c_int
            }
            /* Okay to proceed */
            okay = 1 as libc::c_int as bool_
        } else {
            /* No version yet */
            if okay == 0 { return 2 as libc::c_int }
            /* Included file */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '<' as i32 {
                fp_stack_push(buf.offset(2 as libc::c_int as isize) as cptr);
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'N' as i32 {
                /* Process 'N' for "New/Number/Name" */
                /* Find the colon before the name */
                s = strchr(buf.offset(2 as libc::c_int as isize), ':' as i32);
                /* Verify that colon */
                if s.is_null() { return 1 as libc::c_int }
                /* Nuke the colon, advance to the name */
                let fresh83 = s;
                s = s.offset(1);
                *fresh83 = '\u{0}' as i32 as libc::c_char;
                /* Paranoia -- require a name */
                if *s == 0 { return 1 as libc::c_int }
                /* Get the index */
                i = atoi(buf.offset(2 as libc::c_int as isize));
                /* Verify information */
                if i < error_idx as libc::c_int { return 4 as libc::c_int }
                /* Verify information */
                if i >= (*ba_head).info_num as libc::c_int {
                    return 2 as libc::c_int
                }
                /* Save the index */
                error_idx = i as s16b;
                /* Point at the "info" */
                ba_ptr =
                    &mut *ba_info.offset(i as isize) as
                        *mut store_action_type;
                /* Hack -- Verify space */
                if ((*ba_head).name_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_name_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the name index */
                if (*ba_ptr).name == 0 {
                    (*ba_head).name_size =
                        (*ba_head).name_size.wrapping_add(1);
                    (*ba_ptr).name = (*ba_head).name_size
                }
                /* Append chars to the name */
                strcpy(ba_name.offset((*ba_head).name_size as isize), s);
                /* Advance the index */
                (*ba_head).name_size =
                    ((*ba_head).name_size as
                         libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                        u32b
            } else {
                /* There better be a current ba_ptr */
                if ba_ptr.is_null() { return 3 as libc::c_int }
                /* Process 'C' for "Costs" (one line only) */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'C' as i32 {
                    let mut ch: libc::c_int = 0;
                    let mut cn: libc::c_int = 0;
                    let mut cl: libc::c_int = 0;
                    /* Scan for the values */
                    if 3 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut ch as *mut libc::c_int,
                                  &mut cn as *mut libc::c_int,
                                  &mut cl as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*ba_ptr).costs[0 as libc::c_int as usize] = ch as s16b;
                    (*ba_ptr).costs[2 as libc::c_int as usize] = cn as s16b;
                    (*ba_ptr).costs[1 as libc::c_int as usize] = cl as s16b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'I' as i32 {
                    let mut act: libc::c_int = 0;
                    let mut act_res: libc::c_int = 0;
                    let mut letter_0: libc::c_char = 0;
                    let mut letter_aux: libc::c_char =
                        0 as libc::c_int as libc::c_char;
                    /* Process 'I' for "Infos" (one line only) */
                    /* Scan for the values */
                    if 4 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%c:%c\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut act as *mut libc::c_int,
                                  &mut act_res as *mut libc::c_int,
                                  &mut letter_0 as *mut libc::c_char,
                                  &mut letter_aux as *mut libc::c_char) {
                        if 3 as libc::c_int !=
                               sscanf(buf.offset(2 as libc::c_int as isize),
                                      b"%d:%d:%c\x00" as *const u8 as
                                          *const libc::c_char,
                                      &mut act as *mut libc::c_int,
                                      &mut act_res as *mut libc::c_int,
                                      &mut letter_0 as *mut libc::c_char) {
                            return 1 as libc::c_int
                        }
                    }
                    /* Save the values */
                    (*ba_ptr).action = act as s16b;
                    (*ba_ptr).action_restr = act_res as s16b;
                    (*ba_ptr).letter = letter_0;
                    (*ba_ptr).letter_aux = letter_aux
                } else {
                    /* Oops */
                    return 6 as libc::c_int
                }
            }
        }
    }
    /* Complete the "name" and "text" sizes */
    (*ba_head).name_size = (*ba_head).name_size.wrapping_add(1);
    (*ba_head).text_size = (*ba_head).text_size.wrapping_add(1);
    /* No version yet */
    if okay == 0 { return 2 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialize the "ow_info" array, by parsing an ascii "template" file
 */
#[no_mangle]
pub unsafe extern "C" fn init_ow_info_txt(mut fp: *mut FILE,
                                          mut buf: *mut libc::c_char)
 -> errr {
    let mut i: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Not ready yet */
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    /* Current entry */
    let mut ow_ptr: *mut owner_type = 0 as *mut owner_type;
    /* Just before the first record */
    error_idx = -(1 as libc::c_int) as s16b;
    /* Just before the first line */
    error_line = -(1 as libc::c_int) as s16b;
    /* Start the "fake" stuff */
    (*ow_head).name_size = 0 as libc::c_int as u32b;
    (*ow_head).text_size = 0 as libc::c_int as u32b;
    /* Parse */
    fp_stack_init(fp);
    /* Next... */
    while 0 as libc::c_int == my_fgets_dostack(buf, 1024 as libc::c_int) {
        /* Advance the line number */
        error_line += 1;
        /* Skip comments and blank lines */
        if *buf.offset(0 as libc::c_int as isize) == 0 ||
               *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '#' as i32 {
            continue ;
        }
        /* Verify correct "colon" format */
        if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
           {
            return 1 as libc::c_int
        }
        /* Hack -- Process 'V' for "Version" */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'V' as i32
           {
            let mut v1: libc::c_int = 0;
            let mut v2: libc::c_int = 0;
            let mut v3: libc::c_int = 0;
            /* Scan for the values */
            if 3 as libc::c_int !=
                   sscanf(buf.offset(2 as libc::c_int as isize),
                          b"%d.%d.%d\x00" as *const u8 as *const libc::c_char,
                          &mut v1 as *mut libc::c_int,
                          &mut v2 as *mut libc::c_int,
                          &mut v3 as *mut libc::c_int) {
                return 2 as libc::c_int
            }
            /* Okay to proceed */
            okay = 1 as libc::c_int as bool_
        } else {
            /* No version yet */
            if okay == 0 { return 2 as libc::c_int }
            /* Included file */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '<' as i32 {
                fp_stack_push(buf.offset(2 as libc::c_int as isize) as cptr);
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'N' as i32 {
                /* Process 'N' for "New/Number/Name" */
                /* Find the colon before the name */
                s = strchr(buf.offset(2 as libc::c_int as isize), ':' as i32);
                /* Verify that colon */
                if s.is_null() { return 1 as libc::c_int }
                /* Nuke the colon, advance to the name */
                let fresh84 = s;
                s = s.offset(1);
                *fresh84 = '\u{0}' as i32 as libc::c_char;
                /* Paranoia -- require a name */
                if *s == 0 { return 1 as libc::c_int }
                /* Get the index */
                i = atoi(buf.offset(2 as libc::c_int as isize));
                /* Verify information */
                if i < error_idx as libc::c_int { return 4 as libc::c_int }
                /* Verify information */
                if i >= (*ow_head).info_num as libc::c_int {
                    return 2 as libc::c_int
                }
                /* Save the index */
                error_idx = i as s16b;
                /* Point at the "info" */
                ow_ptr = &mut *ow_info.offset(i as isize) as *mut owner_type;
                /* Hack -- Verify space */
                if ((*ow_head).name_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_name_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the name index */
                if (*ow_ptr).name == 0 {
                    (*ow_head).name_size =
                        (*ow_head).name_size.wrapping_add(1);
                    (*ow_ptr).name = (*ow_head).name_size
                }
                /* Append chars to the name */
                strcpy(ow_name.offset((*ow_head).name_size as isize), s);
                /* Advance the index */
                (*ow_head).name_size =
                    ((*ow_head).name_size as
                         libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                        u32b
            } else {
                /* There better be a current ow_ptr */
                if ow_ptr.is_null() { return 3 as libc::c_int }
                /* Process 'C' for "Costs" (one line only) */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'C' as i32 {
                    let mut ch: libc::c_int = 0;
                    let mut cn: libc::c_int = 0;
                    let mut cl: libc::c_int = 0;
                    /* Scan for the values */
                    if 3 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut ch as *mut libc::c_int,
                                  &mut cn as *mut libc::c_int,
                                  &mut cl as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*ow_ptr).costs[0 as libc::c_int as usize] = ch as s16b;
                    (*ow_ptr).costs[2 as libc::c_int as usize] = cn as s16b;
                    (*ow_ptr).costs[1 as libc::c_int as usize] = cl as s16b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'I' as i32 {
                    let mut cost: libc::c_int = 0;
                    let mut max_inf: libc::c_int = 0;
                    let mut min_inf: libc::c_int = 0;
                    let mut haggle: libc::c_int = 0;
                    let mut insult: libc::c_int = 0;
                    /* Process 'I' for "Info" (multiple lines line only) */
                    /* Scan for the values */
                    if 5 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d:%d:%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut cost as *mut libc::c_int,
                                  &mut max_inf as *mut libc::c_int,
                                  &mut min_inf as *mut libc::c_int,
                                  &mut haggle as *mut libc::c_int,
                                  &mut insult as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*ow_ptr).max_cost = cost as s16b;
                    (*ow_ptr).max_inflate = max_inf as byte_hack;
                    (*ow_ptr).min_inflate = min_inf as byte_hack;
                    (*ow_ptr).haggle_per = haggle as byte_hack;
                    (*ow_ptr).insult_max = insult as byte_hack
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'L' as i32 {
                    /* Process 'L' for "Liked races/classes" (multiple lines) */
                    /* Parse every entry */
                    s = buf.offset(2 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh85 = t;
                            t = t.offset(1);
                            *fresh85 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_race_flag(ow_ptr, 1 as libc::c_int,
                                                  s as cptr) {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'H' as i32 {
                    /* Process 'H' for "Hated races/classes" (multiple lines) */
                    /* Parse every entry */
                    s = buf.offset(2 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh86 = t;
                            t = t.offset(1);
                            *fresh86 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_race_flag(ow_ptr, 0 as libc::c_int,
                                                  s as cptr) {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else {
                    /* Oops */
                    return 6 as libc::c_int
                }
            }
        }
    }
    /* Complete the "name" and "text" sizes */
    (*ow_head).name_size = (*ow_head).name_size.wrapping_add(1);
    (*ow_head).text_size = (*ow_head).text_size.wrapping_add(1);
    /* No version yet */
    if okay == 0 { return 2 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Grab one flag for a dungeon type from a textual string
 */
unsafe extern "C" fn grab_one_wf_info_flag(mut wf_ptr:
                                               *mut wilderness_type_info,
                                           mut what: cptr) -> errr {
    let mut i: libc::c_int = 0;
    /* Scan flags1 */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if streq(what, wf_info_flags1[i as usize]) != 0 {
            (*wf_ptr).flags1 =
                ((*wf_ptr).flags1 as libc::c_long | (1 as libc::c_long) << i)
                    as u32b;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Oops */
    msg_format(b"Unknown monster flag \'%s\'.\x00" as *const u8 as
                   *const libc::c_char, what);
    /* Failure */
    return 1 as libc::c_int;
}
/*
 * Initialize the "wf_info" array, by parsing an ascii "template" file
 */
#[no_mangle]
pub unsafe extern "C" fn init_wf_info_txt(mut fp: *mut FILE,
                                          mut buf: *mut libc::c_char)
 -> errr {
    let mut i: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Not ready yet */
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    /* Current entry */
    let mut wf_ptr: *mut wilderness_type_info =
        0 as *mut wilderness_type_info;
    /* Just before the first record */
    error_idx = -(1 as libc::c_int) as s16b;
    /* Just before the first line */
    error_line = -(1 as libc::c_int) as s16b;
    /* Start the "fake" stuff */
    (*wf_head).name_size = 0 as libc::c_int as u32b;
    (*wf_head).text_size = 0 as libc::c_int as u32b;
    /* Parse */
    fp_stack_init(fp);
    /* Next... */
    while 0 as libc::c_int == my_fgets_dostack(buf, 1024 as libc::c_int) {
        /* Advance the line number */
        error_line += 1;
        /* Skip comments and blank lines */
        if *buf.offset(0 as libc::c_int as isize) == 0 ||
               *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '#' as i32 {
            continue ;
        }
        /* Verify correct "colon" format */
        if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
           {
            return 1 as libc::c_int
        }
        /* Hack -- Process 'V' for "Version" */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'V' as i32
           {
            let mut v1: libc::c_int = 0;
            let mut v2: libc::c_int = 0;
            let mut v3: libc::c_int = 0;
            /* Scan for the values */
            if 3 as libc::c_int !=
                   sscanf(buf.offset(2 as libc::c_int as isize),
                          b"%d.%d.%d\x00" as *const u8 as *const libc::c_char,
                          &mut v1 as *mut libc::c_int,
                          &mut v2 as *mut libc::c_int,
                          &mut v3 as *mut libc::c_int) {
                return 2 as libc::c_int
            }
            /* Okay to proceed */
            okay = 1 as libc::c_int as bool_
        } else {
            /* No version yet */
            if okay == 0 { return 2 as libc::c_int }
            /* Included file */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '<' as i32 {
                fp_stack_push(buf.offset(2 as libc::c_int as isize) as cptr);
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'N' as i32 {
                /* Process 'N' for "New/Number/Name" */
                /* Find the colon before the name */
                s = strchr(buf.offset(2 as libc::c_int as isize), ':' as i32);
                /* Verify that colon */
                if s.is_null() { return 1 as libc::c_int }
                /* Nuke the colon, advance to the name */
                let fresh87 = s;
                s = s.offset(1);
                *fresh87 = '\u{0}' as i32 as libc::c_char;
                /* Paranoia -- require a name */
                if *s == 0 { return 1 as libc::c_int }
                /* Get the index */
                i = atoi(buf.offset(2 as libc::c_int as isize));
                /* Verify information */
                if i < error_idx as libc::c_int { return 4 as libc::c_int }
                /* Verify information */
                if i >= (*wf_head).info_num as libc::c_int {
                    return 2 as libc::c_int
                }
                /* Save the index */
                error_idx = i as s16b;
                /* Point at the "info" */
                wf_ptr =
                    &mut *wf_info.offset(i as isize) as
                        *mut wilderness_type_info;
                /* Hack -- Verify space */
                if ((*wf_head).name_size as
                        libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                       > fake_name_size as libc::c_ulong {
                    return 7 as libc::c_int
                }
                /* Advance and Save the name index */
                if (*wf_ptr).name == 0 {
                    (*wf_head).name_size =
                        (*wf_head).name_size.wrapping_add(1);
                    (*wf_ptr).name = (*wf_head).name_size
                }
                /* Append chars to the name */
                strcpy(wf_name.offset((*wf_head).name_size as isize), s);
                /* Advance the index */
                (*wf_head).name_size =
                    ((*wf_head).name_size as
                         libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                        u32b
            } else {
                /* There better be a current wf_ptr */
                if wf_ptr.is_null() { return 3 as libc::c_int }
                /* Process 'D' for "Description */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'D' as i32 {
                    /* Acquire the text */
                    s = buf.offset(2 as libc::c_int as isize);
                    /* Hack -- Verify space */
                    if ((*wf_head).text_size as
                            libc::c_ulong).wrapping_add(strlen(s)).wrapping_add(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                           > fake_text_size as libc::c_ulong {
                        return 7 as libc::c_int
                    }
                    /* Advance and Save the text index */
                    if (*wf_ptr).text == 0 {
                        (*wf_head).text_size =
                            (*wf_head).text_size.wrapping_add(1);
                        (*wf_ptr).text = (*wf_head).text_size
                    }
                    /* Append chars to the name */
                    strcpy(wf_text.offset((*wf_head).text_size as isize), s);
                    /* Advance the index */
                    (*wf_head).text_size =
                        ((*wf_head).text_size as
                             libc::c_ulong).wrapping_add(strlen(s)) as u32b as
                            u32b
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'W' as i32 {
                    let mut entrance: libc::c_int = 0;
                    let mut level: libc::c_int = 0;
                    let mut road: libc::c_int = 0;
                    let mut feat: libc::c_int = 0;
                    let mut ter_idx: libc::c_int = 0;
                    let mut car: libc::c_char = 0;
                    /* Process 'W' for "More Info" (one line only) */
                    /* Scan for the values */
                    if 6 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d:%d:%d:%c\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut level as *mut libc::c_int,
                                  &mut entrance as *mut libc::c_int,
                                  &mut road as *mut libc::c_int,
                                  &mut feat as *mut libc::c_int,
                                  &mut ter_idx as *mut libc::c_int,
                                  &mut car as *mut libc::c_char) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    (*wf_ptr).level = level;
                    (*wf_ptr).entrance = entrance as u16b;
                    (*wf_ptr).road = road as byte_hack;
                    (*wf_ptr).feat = feat as byte_hack;
                    (*wf_ptr).terrain_idx = ter_idx as byte_hack;
                    /* To acces it easily from the map structure */
                    wildc2i[car as libc::c_int as usize] =
                        error_idx as libc::c_int
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'X' as i32 {
                    let mut terrain: [libc::c_int; 18] = [0; 18];
                    let mut i_0: libc::c_int = 0;
                    /* Process 'X' for "More Info" (one line only) */
                    /* Scan for the values */
                    if 18 as libc::c_int !=
                           sscanf(buf.offset(2 as libc::c_int as isize),
                                  b"%d:%d:%d:%d:%d:%d:%d:%d:%d:%d:%d:%d:%d:%d:%d:%d:%d:%d\x00"
                                      as *const u8 as *const libc::c_char,
                                  &mut *terrain.as_mut_ptr().offset(0 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                      as *mut libc::c_int,
                                  &mut *terrain.as_mut_ptr().offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                      as *mut libc::c_int,
                                  &mut *terrain.as_mut_ptr().offset(2 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                      as *mut libc::c_int,
                                  &mut *terrain.as_mut_ptr().offset(3 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                      as *mut libc::c_int,
                                  &mut *terrain.as_mut_ptr().offset(4 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                      as *mut libc::c_int,
                                  &mut *terrain.as_mut_ptr().offset(5 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                      as *mut libc::c_int,
                                  &mut *terrain.as_mut_ptr().offset(6 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                      as *mut libc::c_int,
                                  &mut *terrain.as_mut_ptr().offset(7 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                      as *mut libc::c_int,
                                  &mut *terrain.as_mut_ptr().offset(8 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                      as *mut libc::c_int,
                                  &mut *terrain.as_mut_ptr().offset(9 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                      as *mut libc::c_int,
                                  &mut *terrain.as_mut_ptr().offset(10 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                      as *mut libc::c_int,
                                  &mut *terrain.as_mut_ptr().offset(11 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                      as *mut libc::c_int,
                                  &mut *terrain.as_mut_ptr().offset(12 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                      as *mut libc::c_int,
                                  &mut *terrain.as_mut_ptr().offset(13 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                      as *mut libc::c_int,
                                  &mut *terrain.as_mut_ptr().offset(14 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                      as *mut libc::c_int,
                                  &mut *terrain.as_mut_ptr().offset(15 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                      as *mut libc::c_int,
                                  &mut *terrain.as_mut_ptr().offset(16 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                      as *mut libc::c_int,
                                  &mut *terrain.as_mut_ptr().offset(17 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                      as *mut libc::c_int) {
                        return 1 as libc::c_int
                    }
                    /* Save the values */
                    i_0 = 0 as libc::c_int;
                    while i_0 < 18 as libc::c_int {
                        (*wf_ptr).terrain[i_0 as usize] =
                            terrain[i_0 as usize] as byte_hack;
                        i_0 += 1
                    }
                } else if *buf.offset(0 as libc::c_int as isize) as
                              libc::c_int == 'F' as i32 {
                    /* Process 'F' for "Wilderness feature Flags" (multiple lines) */
                    /* Parse every entry */
                    s = buf.offset(2 as libc::c_int as isize);
                    while *s != 0 {
                        /* Find the end of this entry */
                        t = s;
                        while *t as libc::c_int != 0 &&
                                  *t as libc::c_int != ' ' as i32 &&
                                  *t as libc::c_int != '|' as i32 {
                            /* loop */
                            t = t.offset(1)
                        }
                        /* Nuke and skip any dividers */
                        if *t != 0 {
                            let fresh88 = t;
                            t = t.offset(1);
                            *fresh88 = '\u{0}' as i32 as libc::c_char;
                            while *t as libc::c_int == ' ' as i32 ||
                                      *t as libc::c_int == '|' as i32 {
                                t = t.offset(1)
                            }
                        }
                        /* Parse this entry */
                        if 0 as libc::c_int !=
                               grab_one_wf_info_flag(wf_ptr, s as cptr) {
                            return 5 as libc::c_int
                        }
                        /* Start the next entry */
                        s = t
                    }
                } else {
                    /* Oops */
                    return 6 as libc::c_int
                }
            }
        }
    }
    /* Complete the "name" and "text" sizes */
    (*wf_head).name_size = (*wf_head).name_size.wrapping_add(1);
    (*wf_head).text_size = (*wf_head).text_size.wrapping_add(1);
    /* No version yet */
    if okay == 0 { return 2 as libc::c_int }
    /* Success */
    return 0 as libc::c_int;
}
static mut meta_sleep: bool_ = 1 as libc::c_int as bool_;
static mut letter: [dungeon_grid; 255] =
    [dungeon_grid{feature: 0,
                  monster: 0,
                  object: 0,
                  ego: 0,
                  artifact: 0,
                  trap: 0,
                  cave_info: 0,
                  special: 0,
                  random: 0,
                  bx: 0,
                  by: 0,
                  mimic: 0,
                  mflag: 0,
                  ok: 0,
                  defined: 0,}; 255];
/*
 * Parse a sub-file of the "extra info"
 */
unsafe extern "C" fn process_dungeon_file_aux(mut buf: *mut libc::c_char,
                                              mut yval: *mut libc::c_int,
                                              mut xval: *mut libc::c_int,
                                              mut xvalstart: libc::c_int,
                                              mut ymax: libc::c_int,
                                              mut xmax: libc::c_int,
                                              mut full: bool_) -> errr {
    let mut i: libc::c_int = 0;
    let mut zz: [*mut libc::c_char; 33] = [0 as *mut libc::c_char; 33];
    /* Skip "empty" lines */
    if *buf.offset(0 as libc::c_int as isize) == 0 { return 0 as libc::c_int }
    /* Skip "blank" lines */
    if *(*__ctype_b_loc()).offset(*buf.offset(0 as libc::c_int as isize) as
                                      libc::c_int as isize) as libc::c_int &
           _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        return 0 as libc::c_int
    }
    /* Skip comments */
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32 {
        return 0 as libc::c_int
    }
    /* Require "?:*" format */
    if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32 {
        return 1 as libc::c_int
    }
    /* Process "%:<fname>" */
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32 {
        /* Attempt to Process the given file */
        return process_dungeon_file(buf.offset(2 as libc::c_int as isize) as
                                        cptr, yval, xval, ymax, xmax,
                                    0 as libc::c_int as bool_, full)
    }
    /* Process "N:<sleep>" */
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'N' as i32 {
        let mut num: libc::c_int = 0;
        num =
            tokenize(buf.offset(2 as libc::c_int as isize),
                     1 as libc::c_int as s16b, zz.as_mut_ptr(),
                     ':' as i32 as libc::c_char, '/' as i32 as libc::c_char)
                as libc::c_int;
        if num > 0 as libc::c_int {
            meta_sleep = atoi(zz[0 as libc::c_int as usize]) as bool_
        }
        return 0 as libc::c_int
    }
    /* Process "F:<letter>:<terrain>:<cave_info>:<monster>:<object>:<ego>:<artifact>:<trap>:<special>:<mimic>:<mflag>" -- info for dungeon grid */
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'F' as i32 {
        let mut num_0: libc::c_int = 0;
        num_0 =
            tokenize(buf.offset(2 as libc::c_int as isize),
                     11 as libc::c_int as s16b, zz.as_mut_ptr(),
                     ':' as i32 as libc::c_char, '/' as i32 as libc::c_char)
                as libc::c_int;
        if num_0 > 1 as libc::c_int {
            let mut index: libc::c_int =
                *zz[0 as libc::c_int as
                        usize].offset(0 as libc::c_int as isize) as
                    libc::c_int;
            /* Reset the feature */
            letter[index as usize].feature = 0 as libc::c_int;
            letter[index as usize].monster = 0 as libc::c_int;
            letter[index as usize].object = 0 as libc::c_int;
            letter[index as usize].ego = 0 as libc::c_int;
            letter[index as usize].artifact = 0 as libc::c_int;
            letter[index as usize].trap = 0 as libc::c_int;
            letter[index as usize].cave_info = 0 as libc::c_int;
            letter[index as usize].special = 0 as libc::c_int;
            letter[index as usize].random = 0 as libc::c_int;
            letter[index as usize].mimic = 0 as libc::c_int;
            letter[index as usize].mflag = 0 as libc::c_int;
            letter[index as usize].ok = 1 as libc::c_int as bool_;
            letter[index as usize].defined = 1 as libc::c_int as bool_;
            if num_0 > 1 as libc::c_int {
                if *zz[1 as libc::c_int as
                           usize].offset(0 as libc::c_int as isize) as
                       libc::c_int == '*' as i32 {
                    letter[index as usize].random |= 0x1 as libc::c_int;
                    if *zz[1 as libc::c_int as
                               usize].offset(1 as libc::c_int as isize) != 0 {
                        zz[1 as libc::c_int as usize] =
                            zz[1 as libc::c_int as usize].offset(1);
                        letter[index as usize].feature =
                            atoi(zz[1 as libc::c_int as usize])
                    }
                } else {
                    letter[index as usize].feature =
                        atoi(zz[1 as libc::c_int as usize])
                }
            }
            if num_0 > 2 as libc::c_int {
                letter[index as usize].cave_info =
                    atoi(zz[2 as libc::c_int as usize])
            }
            /* Monster */
            if num_0 > 3 as libc::c_int {
                if *zz[3 as libc::c_int as
                           usize].offset(0 as libc::c_int as isize) as
                       libc::c_int == '*' as i32 {
                    letter[index as usize].random |= 0x2 as libc::c_int;
                    if *zz[3 as libc::c_int as
                               usize].offset(1 as libc::c_int as isize) != 0 {
                        zz[3 as libc::c_int as usize] =
                            zz[3 as libc::c_int as usize].offset(1);
                        letter[index as usize].monster =
                            atoi(zz[3 as libc::c_int as usize])
                    }
                } else {
                    letter[index as usize].monster =
                        atoi(zz[3 as libc::c_int as usize])
                }
            }
            /* Object */
            if num_0 > 4 as libc::c_int {
                if *zz[4 as libc::c_int as
                           usize].offset(0 as libc::c_int as isize) as
                       libc::c_int == '*' as i32 {
                    letter[index as usize].random |= 0x4 as libc::c_int;
                    if *zz[4 as libc::c_int as
                               usize].offset(1 as libc::c_int as isize) != 0 {
                        zz[4 as libc::c_int as usize] =
                            zz[4 as libc::c_int as usize].offset(1);
                        letter[index as usize].object =
                            atoi(zz[4 as libc::c_int as usize])
                    }
                } else {
                    letter[index as usize].object =
                        atoi(zz[4 as libc::c_int as usize])
                }
            }
            /* Ego-Item */
            if num_0 > 5 as libc::c_int {
                if *zz[5 as libc::c_int as
                           usize].offset(0 as libc::c_int as isize) as
                       libc::c_int == '*' as i32 {
                    letter[index as usize].random |= 0x8 as libc::c_int;
                    if *zz[5 as libc::c_int as
                               usize].offset(1 as libc::c_int as isize) != 0 {
                        zz[5 as libc::c_int as usize] =
                            zz[5 as libc::c_int as usize].offset(1);
                        letter[index as usize].ego =
                            atoi(zz[5 as libc::c_int as usize])
                    }
                } else {
                    letter[index as usize].ego =
                        atoi(zz[5 as libc::c_int as usize])
                }
            }
            /* Artifact */
            if num_0 > 6 as libc::c_int {
                if *zz[6 as libc::c_int as
                           usize].offset(0 as libc::c_int as isize) as
                       libc::c_int == '*' as i32 {
                    letter[index as usize].random |= 0x10 as libc::c_int;
                    if *zz[6 as libc::c_int as
                               usize].offset(1 as libc::c_int as isize) != 0 {
                        zz[6 as libc::c_int as usize] =
                            zz[6 as libc::c_int as usize].offset(1);
                        letter[index as usize].artifact =
                            atoi(zz[6 as libc::c_int as usize])
                    }
                } else {
                    letter[index as usize].artifact =
                        atoi(zz[6 as libc::c_int as usize])
                }
            }
            if num_0 > 7 as libc::c_int {
                if *zz[7 as libc::c_int as
                           usize].offset(0 as libc::c_int as isize) as
                       libc::c_int == '*' as i32 {
                    letter[index as usize].random |= 0x20 as libc::c_int;
                    if *zz[7 as libc::c_int as
                               usize].offset(1 as libc::c_int as isize) != 0 {
                        zz[7 as libc::c_int as usize] =
                            zz[7 as libc::c_int as usize].offset(1);
                        letter[index as usize].trap =
                            atoi(zz[7 as libc::c_int as usize])
                    }
                } else {
                    letter[index as usize].trap =
                        atoi(zz[7 as libc::c_int as usize])
                }
            }
            if num_0 > 8 as libc::c_int {
                /* Quests can be defined by name only */
                if *zz[8 as libc::c_int as
                           usize].offset(0 as libc::c_int as isize) as
                       libc::c_int == '\"' as i32 {
                    let mut i_0: libc::c_int = 0;
                    /* Hunt & shoot the ending " */
                    i_0 =
                        strlen(zz[8 as libc::c_int as
                                      usize]).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                            as libc::c_int;
                    if *zz[8 as libc::c_int as usize].offset(i_0 as isize) as
                           libc::c_int == '\"' as i32 {
                        *zz[8 as libc::c_int as usize].offset(i_0 as isize) =
                            '\u{0}' as i32 as libc::c_char
                    }
                    letter[index as usize].special = 0 as libc::c_int;
                    i_0 = 0 as libc::c_int;
                    while i_0 < max_q_idx as libc::c_int {
                        if strcmp(&mut *(*zz.as_mut_ptr().offset(8 as
                                                                     libc::c_int
                                                                     as
                                                                     isize)).offset(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize),
                                  (*quest.offset(i_0 as
                                                     isize)).name.as_mut_ptr())
                               == 0 {
                            letter[index as usize].special = i_0;
                            break ;
                        } else { i_0 += 1 }
                    }
                } else {
                    letter[index as usize].special =
                        atoi(zz[8 as libc::c_int as usize])
                }
            }
            if num_0 > 9 as libc::c_int {
                letter[index as usize].mimic =
                    atoi(zz[9 as libc::c_int as usize])
            }
            if num_0 > 10 as libc::c_int {
                letter[index as usize].mflag =
                    atoi(zz[10 as libc::c_int as usize])
            }
            return 0 as libc::c_int
        }
    } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                  'f' as i32 {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        /* Process "f:flags" -- level flags */
        /* Parse every entry textually */
        s = buf.offset(2 as libc::c_int as isize);
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
                let fresh89 = t;
                t = t.offset(1);
                *fresh89 = '\u{0}' as i32 as libc::c_char;
                while *t as libc::c_int == ' ' as i32 ||
                          *t as libc::c_int == '|' as i32 {
                    t = t.offset(1)
                }
            }
            /* Parse this entry */
            if 0 as libc::c_int !=
                   grab_one_dungeon_flag(&mut dungeon_flags1,
                                         &mut dungeon_flags2, s as cptr) {
                return 1 as libc::c_int
            }
            /* Start the next entry */
            s = t
        }
        return 0 as libc::c_int
    } else {
        /* Process "D:<dungeon>" -- info for the cave grids */
        if *buf.offset(0 as libc::c_int as isize) as libc::c_int == 'D' as i32
           {
            let mut x: libc::c_int = 0;
            let mut m_idx: libc::c_int = 0 as libc::c_int;
            let mut object_type_body: object_type =
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
            /* Acquire the text */
            let mut s_0: *mut libc::c_char =
                buf.offset(2 as libc::c_int as isize);
            /* Length of the text */
            let mut len: libc::c_int = strlen(s_0) as libc::c_int;
            let mut y: libc::c_int = *yval;
            *xval = xvalstart;
            x = *xval;
            i = 0 as libc::c_int;
            while x < xmax && i < len {
                /* Access the grid */
                let mut c_ptr: *mut cave_type =
                    &mut *(*cave.as_mut_ptr().offset(y as
                                                         isize)).offset(x as
                                                                            isize)
                        as *mut cave_type;
                let mut idx: libc::c_int =
                    *s_0.offset(0 as libc::c_int as isize) as libc::c_int;
                let mut object_index: libc::c_int =
                    letter[idx as usize].object;
                let mut monster_index: libc::c_int =
                    letter[idx as usize].monster;
                let mut random: libc::c_int = letter[idx as usize].random;
                let mut artifact_index: libc::c_int =
                    letter[idx as usize].artifact;
                if letter[idx as usize].ok == 0 {
                    msg_format(b"Warning \'%c\' not defined but used.\x00" as
                                   *const u8 as *const libc::c_char, idx);
                }
                if !(init_flags & 0x8 as libc::c_int != 0) {
                    /* use the plasma generator wilderness */
                    if !((dun_level == 0 || letter[idx as usize].defined == 0)
                             && idx == ' ' as i32) {
                        /* Clear some info */
                        (*c_ptr).info = 0 as libc::c_int as u16b;
                        /* Lay down a floor */
                        (*c_ptr).mimic =
                            letter[idx as usize].mimic as byte_hack;
                        cave_set_feat(y, x, letter[idx as usize].feature);
                        /* Cave info */
                        (*c_ptr).info =
                            ((*c_ptr).info as libc::c_int |
                                 letter[idx as usize].cave_info) as u16b;
                        /* Create a monster */
                        if random & 0x2 as libc::c_int != 0 {
                            let mut level: libc::c_int =
                                monster_level as libc::c_int;
                            monster_level =
                                ((*quest.offset((*p_ptr).inside_quest as
                                                    isize)).level as
                                     libc::c_int + monster_index) as s16b;
                            m_idx =
                                place_monster(y, x, meta_sleep,
                                              0 as libc::c_int as bool_) as
                                    libc::c_int;
                            monster_level = level as s16b
                        } else if monster_index != 0 {
                            /* Place it */
                            *m_allow_special.offset(monster_index as isize) =
                                1 as libc::c_int as bool_;
                            m_idx =
                                place_monster_aux(y, x, monster_index,
                                                  meta_sleep,
                                                  0 as libc::c_int as bool_,
                                                  -(2 as libc::c_int)) as
                                    libc::c_int;
                            *m_allow_special.offset(monster_index as isize) =
                                0 as libc::c_int as bool_
                        }
                        /* Set the mflag of the monster */
                        if m_idx != 0 {
                            let ref mut fresh90 =
                                (*m_list.offset(m_idx as isize)).mflag;
                            *fresh90 |= letter[idx as usize].mflag
                        }
                        /* Object (and possible trap) */
                        if random & 0x4 as libc::c_int != 0 &&
                               random & 0x20 as libc::c_int != 0 {
                            let mut level_0: libc::c_int =
                                object_level as libc::c_int;
                            object_level =
                                (*quest.offset((*p_ptr).inside_quest as
                                                   isize)).level;
                            /*
				 * Random trap and random treasure defined
				 * 25% chance for trap and 75% chance for object
				 */
                            if Rand_div(100 as libc::c_int) <
                                   75 as libc::c_int {
                                place_object(y, x, 0 as libc::c_int as bool_,
                                             0 as libc::c_int as bool_,
                                             4 as libc::c_int);
                            } else { place_trap(y, x); }
                            object_level = level_0 as s16b
                        } else if random & 0x4 as libc::c_int != 0 {
                            /* Create an out of deep object */
                            if object_index != 0 {
                                let mut level_1: libc::c_int =
                                    object_level as libc::c_int;
                                object_level =
                                    ((*quest.offset((*p_ptr).inside_quest as
                                                        isize)).level as
                                         libc::c_int + object_index) as s16b;
                                if Rand_div(100 as libc::c_int) <
                                       75 as libc::c_int {
                                    place_object(y, x,
                                                 0 as libc::c_int as bool_,
                                                 0 as libc::c_int as bool_,
                                                 4 as libc::c_int);
                                } else if Rand_div(100 as libc::c_int) <
                                              80 as libc::c_int {
                                    place_object(y, x,
                                                 1 as libc::c_int as bool_,
                                                 0 as libc::c_int as bool_,
                                                 4 as libc::c_int);
                                } else {
                                    place_object(y, x,
                                                 1 as libc::c_int as bool_,
                                                 1 as libc::c_int as bool_,
                                                 4 as libc::c_int);
                                }
                                object_level = level_1 as s16b
                            } else if Rand_div(100 as libc::c_int) <
                                          75 as libc::c_int {
                                place_object(y, x, 0 as libc::c_int as bool_,
                                             0 as libc::c_int as bool_,
                                             4 as libc::c_int);
                            } else if Rand_div(100 as libc::c_int) <
                                          80 as libc::c_int {
                                place_object(y, x, 1 as libc::c_int as bool_,
                                             0 as libc::c_int as bool_,
                                             4 as libc::c_int);
                            } else {
                                place_object(y, x, 1 as libc::c_int as bool_,
                                             1 as libc::c_int as bool_,
                                             4 as libc::c_int);
                            }
                        } else if random & 0x20 as libc::c_int != 0 {
                            place_trap(y, x);
                        } else if object_index != 0 {
                            /* Random trap */
                            /* Get local object */
                            let mut o_ptr: *mut object_type =
                                &mut object_type_body;
                            *k_allow_special.offset(object_index as isize) =
                                1 as libc::c_int as bool_;
                            /* Create the item */
                            object_prep(o_ptr, object_index);
                            /* Apply magic (no messages, no artifacts) */
                            apply_magic(o_ptr, dun_level as libc::c_int,
                                        0 as libc::c_int as bool_,
                                        1 as libc::c_int as bool_,
                                        0 as libc::c_int as bool_);
                            (*o_ptr).found = 4 as libc::c_int as byte_hack;
                            *k_allow_special.offset(object_index as isize) =
                                0 as libc::c_int as bool_;
                            drop_near(o_ptr, -(1 as libc::c_int), y, x);
                        }
                        /* Artifact */
                        if artifact_index != 0 {
                            let mut I_kind: libc::c_int = 0 as libc::c_int;
                            let mut a_ptr: *mut artifact_type =
                                &mut *a_info.offset(artifact_index as isize)
                                    as *mut artifact_type;
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
                            /* Get local object */
                            let mut q_ptr: *mut object_type = &mut forge;
                            *a_allow_special.offset(artifact_index as isize) =
                                1 as libc::c_int as bool_;
                            /* Wipe the object */
                            object_wipe(q_ptr);
                            /* Acquire the "kind" index */
                            I_kind =
                                lookup_kind((*a_ptr).tval as libc::c_int,
                                            (*a_ptr).sval as libc::c_int) as
                                    libc::c_int;
                            /* Create the artifact */
                            object_prep(q_ptr, I_kind);
                            /* Save the name */
                            (*q_ptr).name1 = artifact_index as byte_hack;
                            /* Extract the fields */
                            (*q_ptr).pval = (*a_ptr).pval as s32b;
                            (*q_ptr).ac = (*a_ptr).ac;
                            (*q_ptr).dd = (*a_ptr).dd;
                            (*q_ptr).ds = (*a_ptr).ds;
                            (*q_ptr).to_a = (*a_ptr).to_a;
                            (*q_ptr).to_h = (*a_ptr).to_h;
                            (*q_ptr).to_d = (*a_ptr).to_d;
                            (*q_ptr).weight = (*a_ptr).weight as s32b;
                            (*q_ptr).found = 4 as libc::c_int as byte_hack;
                            random_artifact_resistance(q_ptr);
                            (*a_info.offset(artifact_index as isize)).cur_num
                                = 1 as libc::c_int as byte_hack;
                            *a_allow_special.offset(artifact_index as isize) =
                                0 as libc::c_int as bool_;
                            /* It's amazing that this "creating objects anywhere"
				   junk ever worked.
				   Let's just HACK around one observed bug: Shadow Cloak
				   of Luthien [Globe of Light] */
                            let mut f1: u32b = 0;
                            let mut f2: u32b = 0;
                            let mut f3: u32b = 0;
                            let mut f4: u32b = 0;
                            let mut f5: u32b = 0;
                            let mut esp: u32b = 0;
                            object_flags(q_ptr, &mut f1, &mut f2, &mut f3,
                                         &mut f4, &mut f5, &mut esp);
                            if f5 as libc::c_long & 0x800 as libc::c_long != 0
                               {
                                (*q_ptr).pval2 = -(1 as libc::c_int) as s16b
                            }
                            /* Drop the artifact */
                            drop_near(q_ptr, -(1 as libc::c_int), y, x);
                        }
                        /* Terrain special */
                        if letter[idx as usize].special == -(1 as libc::c_int)
                           {
                            if letter[idx as usize].bx == 0 {
                                letter[idx as usize].bx = x;
                                letter[idx as usize].by = y
                            } else {
                                (*c_ptr).special =
                                    ((letter[idx as usize].by <<
                                          8 as libc::c_int) +
                                         letter[idx as usize].bx) as s16b;
                                (*cave[letter[idx as usize].by as
                                           usize].offset(letter[idx as
                                                                    usize].bx
                                                             as
                                                             isize)).special =
                                    ((y << 8 as libc::c_int) + x) as s16b
                            }
                        } else {
                            (*c_ptr).special =
                                letter[idx as usize].special as s16b
                        }
                    }
                }
                x += 1;
                s_0 = s_0.offset(1);
                i += 1
            }
            if full as libc::c_int != 0 && *xval < x { *xval = x }
            *yval += 1;
            return 0 as libc::c_int
        } else {
            /* Process "W:<command>: ..." -- info for the wilderness */
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                   'W' as i32 {
                /* Process "W:D:<layout> */
		/* Layout of the wilderness */
                if *buf.offset(2 as libc::c_int as isize) as libc::c_int ==
                       'D' as i32 {
                    let mut x_0: libc::c_int = 0;
                    let mut i_1: libc::c_char = 0;
                    /* Acquire the text */
                    let mut s_1: *mut libc::c_char =
                        buf.offset(4 as libc::c_int as isize);
                    let mut y_0: libc::c_int = *yval;
                    x_0 = 0 as libc::c_int;
                    while x_0 < max_wild_x as libc::c_int {
                        if 1 as libc::c_int !=
                               sscanf(s_1.offset(x_0 as isize),
                                      b"%c\x00" as *const u8 as
                                          *const libc::c_char,
                                      &mut i_1 as *mut libc::c_char) {
                            return 1 as libc::c_int
                        }
                        (*(*wild_map.offset(y_0 as
                                                isize)).offset(x_0 as
                                                                   isize)).feat
                            = wildc2i[i_1 as libc::c_int as usize];
                        /*
				 * If this is a town/dungeon entrance, note
				 * its coordinates.  (Have to check for
				 * duplicate Morias...)
				 */
                        if (*wf_info.offset(wildc2i[i_1 as libc::c_int as
                                                        usize] as
                                                isize)).entrance as
                               libc::c_int != 0 &&
                               (*wf_info.offset(wildc2i[i_1 as libc::c_int as
                                                            usize] as
                                                    isize)).wild_x ==
                                   0 as libc::c_int {
                            (*wf_info.offset(wildc2i[i_1 as libc::c_int as
                                                         usize] as
                                                 isize)).wild_x = x_0;
                            (*wf_info.offset(wildc2i[i_1 as libc::c_int as
                                                         usize] as
                                                 isize)).wild_y = y_0
                        }
                        x_0 += 1
                    }
                    *yval += 1;
                    return 0 as libc::c_int
                } else {
                    /* Process "M:<plus>:<line>" -- move line lines */
                    if *buf.offset(2 as libc::c_int as isize) as libc::c_int
                           == 'M' as i32 {
                        if tokenize(buf.offset(4 as libc::c_int as isize),
                                    2 as libc::c_int as s16b, zz.as_mut_ptr(),
                                    ':' as i32 as libc::c_char,
                                    '/' as i32 as libc::c_char) as libc::c_int
                               == 2 as libc::c_int {
                            if atoi(zz[0 as libc::c_int as usize]) != 0 {
                                *yval += atoi(zz[1 as libc::c_int as usize])
                            } else {
                                *yval -= atoi(zz[1 as libc::c_int as usize])
                            }
                        } else { return 1 as libc::c_int }
                        return 0 as libc::c_int
                    } else {
                        /* Process "W:P:<x>:<y> - starting position in the wilderness */
                        if *buf.offset(2 as libc::c_int as isize) as
                               libc::c_int == 'P' as i32 {
                            if (*p_ptr).wilderness_x == 0 as libc::c_int &&
                                   (*p_ptr).wilderness_y == 0 as libc::c_int {
                                if tokenize(buf.offset(4 as libc::c_int as
                                                           isize),
                                            2 as libc::c_int as s16b,
                                            zz.as_mut_ptr(),
                                            ':' as i32 as libc::c_char,
                                            '/' as i32 as libc::c_char) as
                                       libc::c_int == 2 as libc::c_int {
                                    (*p_ptr).wilderness_x =
                                        atoi(zz[0 as libc::c_int as usize]);
                                    (*p_ptr).wilderness_y =
                                        atoi(zz[1 as libc::c_int as usize])
                                } else { return 1 as libc::c_int }
                            }
                            return 0 as libc::c_int
                        } else {
                            /* Process "W:E:<dungeon>:<y>:<x> - entrance to the dungeon <dungeon> */
                            if *buf.offset(2 as libc::c_int as isize) as
                                   libc::c_int == 'E' as i32 {
                                if tokenize(buf.offset(4 as libc::c_int as
                                                           isize),
                                            3 as libc::c_int as s16b,
                                            zz.as_mut_ptr(),
                                            ':' as i32 as libc::c_char,
                                            '/' as i32 as libc::c_char) as
                                       libc::c_int == 3 as libc::c_int {
                                    (*(*wild_map.offset(atoi(zz[1 as
                                                                    libc::c_int
                                                                    as usize])
                                                            as
                                                            isize)).offset(atoi(zz[2
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       usize])
                                                                               as
                                                                               isize)).entrance
                                        =
                                        (1000 as libc::c_int +
                                             atoi(zz[0 as libc::c_int as
                                                         usize])) as u16b
                                } else { return 1 as libc::c_int }
                                return 0 as libc::c_int
                            }
                        }
                    }
                }
            } else if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                          'P' as i32 {
                if init_flags & 0x4 as libc::c_int != 0 {
                    if tokenize(buf.offset(2 as libc::c_int as isize),
                                2 as libc::c_int as s16b, zz.as_mut_ptr(),
                                ':' as i32 as libc::c_char,
                                '/' as i32 as libc::c_char) as libc::c_int ==
                           2 as libc::c_int {
                        /* Process "P:<y>:<x>" -- player position */
                        /* Place player in a quest level */
                        if (*p_ptr).inside_quest as libc::c_int != 0 ||
                               init_flags & 0x10 as libc::c_int != 0 {
                            (*p_ptr).py =
                                atoi(zz[0 as libc::c_int as usize]) as s16b;
                            (*p_ptr).px =
                                atoi(zz[1 as libc::c_int as usize]) as s16b
                        } else if (*p_ptr).oldpx as libc::c_int ==
                                      0 as libc::c_int &&
                                      (*p_ptr).oldpy as libc::c_int ==
                                          0 as libc::c_int {
                            (*p_ptr).oldpy =
                                atoi(zz[0 as libc::c_int as usize]) as s16b;
                            (*p_ptr).oldpx =
                                atoi(zz[1 as libc::c_int as usize]) as s16b
                        }
                    }
                }
                return 0 as libc::c_int
            } else {
                /* Place player in the town */
                /* Process "M:<type>:<maximum>" -- set maximum values */
                if *buf.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'M' as i32 {
                    if tokenize(buf.offset(2 as libc::c_int as isize),
                                3 as libc::c_int as s16b, zz.as_mut_ptr(),
                                ':' as i32 as libc::c_char,
                                '/' as i32 as libc::c_char) as libc::c_int >=
                           2 as libc::c_int {
                        /* Maximum towns */
                        if *zz[0 as libc::c_int as
                                   usize].offset(0 as libc::c_int as isize) as
                               libc::c_int == 'T' as i32 {
                            max_towns =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        }
                        /* Maximum real towns */
                        if *zz[0 as libc::c_int as
                                   usize].offset(0 as libc::c_int as isize) as
                               libc::c_int == 't' as i32 {
                            max_real_towns =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'R' as i32 {
                            max_r_idx =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'r' as i32 {
                            max_re_idx =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'k' as i32 {
                            max_s_idx =
                                atoi(zz[1 as libc::c_int as usize]) as u16b;
                            if max_s_idx as libc::c_int > 200 as libc::c_int {
                                return 1 as libc::c_int
                            }
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'b' as i32 {
                            max_ab_idx =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'K' as i32 {
                            max_k_idx =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'V' as i32 {
                            max_v_idx =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'F' as i32 {
                            max_f_idx =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'A' as i32 {
                            max_a_idx =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'a' as i32 {
                            max_al_idx =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'E' as i32 {
                            max_e_idx =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'Z' as i32 {
                            max_ra_idx =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'O' as i32 {
                            max_o_idx =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'P' as i32 {
                            if *zz[1 as libc::c_int as
                                       usize].offset(0 as libc::c_int as
                                                         isize) as libc::c_int
                                   == 'R' as i32 {
                                max_rp_idx =
                                    atoi(zz[2 as libc::c_int as usize]) as
                                        u16b
                            } else if *zz[1 as libc::c_int as
                                              usize].offset(0 as libc::c_int
                                                                as isize) as
                                          libc::c_int == 'S' as i32 {
                                max_rmp_idx =
                                    atoi(zz[2 as libc::c_int as usize]) as
                                        u16b
                            } else if *zz[1 as libc::c_int as
                                              usize].offset(0 as libc::c_int
                                                                as isize) as
                                          libc::c_int == 'C' as i32 {
                                max_c_idx =
                                    atoi(zz[2 as libc::c_int as usize]) as
                                        u16b
                            } else if *zz[1 as libc::c_int as
                                              usize].offset(0 as libc::c_int
                                                                as isize) as
                                          libc::c_int == 'M' as i32 {
                                max_mc_idx =
                                    atoi(zz[2 as libc::c_int as usize]) as
                                        u16b
                            } else if *zz[1 as libc::c_int as
                                              usize].offset(0 as libc::c_int
                                                                as isize) as
                                          libc::c_int == 'H' as i32 {
                                max_bg_idx =
                                    atoi(zz[2 as libc::c_int as usize])
                            }
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'M' as i32 {
                            max_m_idx =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'U' as i32 {
                            max_t_idx =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'W' as i32 {
                            max_wf_idx =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'B' as i32 {
                            max_ba_idx =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'S' as i32 {
                            max_st_idx =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 's' as i32 {
                            max_set_idx =
                                atoi(zz[1 as libc::c_int as usize]) as s16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'N' as i32 {
                            max_ow_idx =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'X' as i32 {
                            max_wild_x =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'Y' as i32 {
                            max_wild_y =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        } else if *zz[0 as libc::c_int as
                                          usize].offset(0 as libc::c_int as
                                                            isize) as
                                      libc::c_int == 'D' as i32 {
                            max_d_idx =
                                atoi(zz[1 as libc::c_int as usize]) as u16b
                        }
                        return 0 as libc::c_int
                    }
                }
            }
        }
    }
    /* Maximum r_idx */
    /* Maximum re_idx */
    /* Maximum s_idx */
    /* Maximum ab_idx */
    /* Maximum k_idx */
    /* Maximum v_idx */
    /* Maximum f_idx */
    /* Maximum a_idx */
    /* Maximum al_idx */
    /* Maximum e_idx */
    /* Maximum ra_idx */
    /* Maximum o_idx */
    /* Maximum player types */
    /* Maximum m_idx */
    /* Maximum tr_idx */
    /* Maximum wf_idx */
    /* Maximum ba_idx */
    /* Maximum st_idx */
    /* Maximum set_idx */
    /* Maximum ow_idx */
    /* Maximum wilderness x size */
    /* Maximum wilderness y size */
    /* Maximum d_idx */
    /* Failure */
    return 1 as libc::c_int;
}
/*
 * Helper function for "process_dungeon_file()"
 */
unsafe extern "C" fn process_dungeon_file_expr(mut sp: *mut *mut libc::c_char,
                                               mut fp: *mut libc::c_char)
 -> cptr {
    let mut v: cptr = 0 as *const libc::c_char;
    let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b1: libc::c_char = '[' as i32 as libc::c_char;
    let mut b2: libc::c_char = ']' as i32 as libc::c_char;
    let mut f: libc::c_char = ' ' as i32 as libc::c_char;
    /* Initial */
    s = *sp;
    /* Skip spaces */
    while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as
              libc::c_int &
              _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        s = s.offset(1)
    }
    /* Save start */
    b = s;
    /* Default */
    v = b"?o?o?\x00" as *const u8 as *const libc::c_char;
    /* Analyze */
    if *s as libc::c_int == b1 as libc::c_int {
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        let mut t: *const libc::c_char = 0 as *const libc::c_char;
        /* Skip b1 */
        s = s.offset(1);
        /* First */
        t = process_dungeon_file_expr(&mut s, &mut f);
        /* Oops */
        if !(*t == 0) {
            /* Function: IOR */
            if streq(t, b"IOR\x00" as *const u8 as *const libc::c_char) != 0 {
                v = b"0\x00" as *const u8 as *const libc::c_char;
                while *s as libc::c_int != 0 &&
                          f as libc::c_int != b2 as libc::c_int {
                    t = process_dungeon_file_expr(&mut s, &mut f);
                    if *t as libc::c_int != 0 &&
                           streq(t,
                                 b"0\x00" as *const u8 as *const libc::c_char)
                               == 0 {
                        v = b"1\x00" as *const u8 as *const libc::c_char
                    }
                }
            } else if streq(t, b"AND\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                v = b"1\x00" as *const u8 as *const libc::c_char;
                while *s as libc::c_int != 0 &&
                          f as libc::c_int != b2 as libc::c_int {
                    t = process_dungeon_file_expr(&mut s, &mut f);
                    if *t as libc::c_int != 0 &&
                           streq(t,
                                 b"0\x00" as *const u8 as *const libc::c_char)
                               as libc::c_int != 0 {
                        v = b"0\x00" as *const u8 as *const libc::c_char
                    }
                }
            } else if streq(t, b"NOT\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                v = b"1\x00" as *const u8 as *const libc::c_char;
                while *s as libc::c_int != 0 &&
                          f as libc::c_int != b2 as libc::c_int {
                    t = process_dungeon_file_expr(&mut s, &mut f);
                    if *t as libc::c_int != 0 &&
                           streq(t,
                                 b"1\x00" as *const u8 as *const libc::c_char)
                               as libc::c_int != 0 {
                        v = b"0\x00" as *const u8 as *const libc::c_char
                    }
                }
            } else if streq(t, b"EQU\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                v = b"1\x00" as *const u8 as *const libc::c_char;
                if *s as libc::c_int != 0 &&
                       f as libc::c_int != b2 as libc::c_int {
                    t = process_dungeon_file_expr(&mut s, &mut f)
                }
                while *s as libc::c_int != 0 &&
                          f as libc::c_int != b2 as libc::c_int {
                    p = t;
                    t = process_dungeon_file_expr(&mut s, &mut f);
                    if *t as libc::c_int != 0 && streq(p, t) == 0 {
                        v = b"0\x00" as *const u8 as *const libc::c_char
                    }
                }
            } else if streq(t, b"LEQ\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                v = b"1\x00" as *const u8 as *const libc::c_char;
                if *s as libc::c_int != 0 &&
                       f as libc::c_int != b2 as libc::c_int {
                    t = process_dungeon_file_expr(&mut s, &mut f)
                }
                while *s as libc::c_int != 0 &&
                          f as libc::c_int != b2 as libc::c_int {
                    p = t;
                    t = process_dungeon_file_expr(&mut s, &mut f);
                    if *t as libc::c_int != 0 &&
                           strcmp(p, t) > 0 as libc::c_int {
                        v = b"0\x00" as *const u8 as *const libc::c_char
                    }
                }
            } else if streq(t, b"GEQ\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                v = b"1\x00" as *const u8 as *const libc::c_char;
                if *s as libc::c_int != 0 &&
                       f as libc::c_int != b2 as libc::c_int {
                    t = process_dungeon_file_expr(&mut s, &mut f)
                }
                while *s as libc::c_int != 0 &&
                          f as libc::c_int != b2 as libc::c_int {
                    p = t;
                    t = process_dungeon_file_expr(&mut s, &mut f);
                    if *t as libc::c_int != 0 &&
                           strcmp(p, t) < 0 as libc::c_int {
                        v = b"0\x00" as *const u8 as *const libc::c_char
                    }
                }
            } else {
                /* Function: AND */
                /* Function: NOT */
                /* Function: EQU */
                /* Function: LEQ */
                /* Function: GEQ */
                /* Oops */
                while *s as libc::c_int != 0 &&
                          f as libc::c_int != b2 as libc::c_int {
                    t = process_dungeon_file_expr(&mut s, &mut f)
                }
            }
        }
        /* Verify ending */
        if f as libc::c_int != b2 as libc::c_int {
            v = b"?x?x?\x00" as *const u8 as *const libc::c_char
        }
        /* Extract final and Terminate */
        f = *s;
        if f as libc::c_int != '\u{0}' as i32 {
            let fresh91 = s;
            s = s.offset(1);
            *fresh91 = '\u{0}' as i32 as libc::c_char
        }
    } else {
        /* Other */
        let mut text_mode: bool_ = 0 as libc::c_int as bool_;
        /* Accept all printables except spaces and brackets */
        while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as
                  libc::c_int &
                  _ISprint as libc::c_int as libc::c_ushort as libc::c_int !=
                  0 {
            if *s as libc::c_int == '\"' as i32 {
                text_mode = (text_mode == 0) as libc::c_int as bool_
            }
            if text_mode == 0 {
                if !strchr(b" []\x00" as *const u8 as *const libc::c_char,
                           *s as libc::c_int).is_null() {
                    break ;
                }
            } else if !strchr(b"[]\x00" as *const u8 as *const libc::c_char,
                              *s as libc::c_int).is_null() {
                break ;
            }
            s = s.offset(1)
        }
        /* Extract final and Terminate */
        f = *s;
        if f as libc::c_int != '\u{0}' as i32 {
            let fresh92 = s;
            s = s.offset(1);
            *fresh92 = '\u{0}' as i32 as libc::c_char
        }
        /* Variable */
        if *b as libc::c_int == '$' as i32 {
            /* System */
            if streq(b.offset(1 as libc::c_int as isize) as cptr,
                     b"SYS\x00" as *const u8 as *const libc::c_char) != 0 {
                v = ANGBAND_SYS
            } else if streq(b.offset(1 as libc::c_int as isize) as cptr,
                            b"GRAF\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                v = ANGBAND_GRAF
            } else if streq(b.offset(1 as libc::c_int as isize) as cptr,
                            b"RACE\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                v = rp_name.offset((*rp_ptr).title as isize) as cptr
            } else if streq(b.offset(1 as libc::c_int as isize) as cptr,
                            b"RACEMOD\x00" as *const u8 as
                                *const libc::c_char) != 0 {
                v = rmp_name.offset((*rmp_ptr).title as isize) as cptr
            } else if streq(b.offset(1 as libc::c_int as isize) as cptr,
                            b"CLASS\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                v = c_name.offset((*cp_ptr).title as isize) as cptr
            } else if streq(b.offset(1 as libc::c_int as isize) as cptr,
                            b"PLAYER\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                v = player_base.as_mut_ptr() as cptr
            } else if streq(b.offset(1 as libc::c_int as isize) as cptr,
                            b"TOWN\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                strnfmt(pref_tmp_value.as_mut_ptr(),
                        8 as libc::c_int as uint_hack,
                        b"%d\x00" as *const u8 as *const libc::c_char,
                        (*p_ptr).town_num as libc::c_int);
                v = pref_tmp_value.as_mut_ptr() as cptr
            } else if prefix(b.offset(1 as libc::c_int as isize) as cptr,
                             b"TOWN_DESTROY\x00" as *const u8 as
                                 *const libc::c_char) != 0 {
                strnfmt(pref_tmp_value.as_mut_ptr(),
                        8 as libc::c_int as uint_hack,
                        b"%d\x00" as *const u8 as *const libc::c_char,
                        (*town_info.offset(atoi(b.offset(13 as libc::c_int as
                                                             isize)) as
                                               isize)).destroyed as
                            libc::c_int);
                v = pref_tmp_value.as_mut_ptr() as cptr
            } else if streq(b.offset(1 as libc::c_int as isize) as cptr,
                            b"QUEST_NUMBER\x00" as *const u8 as
                                *const libc::c_char) != 0 {
                strnfmt(pref_tmp_value.as_mut_ptr(),
                        8 as libc::c_int as uint_hack,
                        b"%d\x00" as *const u8 as *const libc::c_char,
                        (*p_ptr).inside_quest as libc::c_int);
                v = pref_tmp_value.as_mut_ptr() as cptr
            } else if streq(b.offset(1 as libc::c_int as isize) as cptr,
                            b"LEAVING_QUEST\x00" as *const u8 as
                                *const libc::c_char) != 0 {
                strnfmt(pref_tmp_value.as_mut_ptr(),
                        8 as libc::c_int as uint_hack,
                        b"%d\x00" as *const u8 as *const libc::c_char,
                        leaving_quest);
                v = pref_tmp_value.as_mut_ptr() as cptr
            } else if prefix(b.offset(1 as libc::c_int as isize) as cptr,
                             b"DAYTIME\x00" as *const u8 as
                                 *const libc::c_char) != 0 {
                if bst(11520 as libc::c_int / 24 as libc::c_int, turn) >=
                       6 as libc::c_int &&
                       bst(11520 as libc::c_int / 24 as libc::c_int, turn) <
                           18 as libc::c_int {
                    v = b"1\x00" as *const u8 as *const libc::c_char
                } else { v = b"0\x00" as *const u8 as *const libc::c_char }
            } else if prefix(b.offset(1 as libc::c_int as isize) as cptr,
                             b"QUEST\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                /* Graphics */
                /* Race */
                /* Race Mod */
                /* Class */
                /* Player */
                /* Town */
                /* Town destroyed */
                /* Current quest number */
                /* Number of last quest */
                /* DAYTIME status */
                /* Quest status */
                /* "QUEST" uses a special parameter to determine the number of the quest */
                if *b.offset(6 as libc::c_int as isize) as libc::c_int !=
                       '\"' as i32 {
                    strnfmt(pref_tmp_value.as_mut_ptr(),
                            8 as libc::c_int as uint_hack,
                            b"%d\x00" as *const u8 as *const libc::c_char,
                            (*quest.offset(atoi(b.offset(6 as libc::c_int as
                                                             isize)) as
                                               isize)).status as libc::c_int);
                } else {
                    let mut c: [libc::c_char; 80] = [0; 80];
                    let mut i: libc::c_int = 0;
                    /* Copy it to temp array, so that we can modify it safely */
                    strcpy(c.as_mut_ptr(),
                           b.offset(7 as libc::c_int as isize));
                    /* Hunt & shoot the ending " */
                    i = 0 as libc::c_int;
                    while c[i as usize] as libc::c_int != '\"' as i32 &&
                              c[i as usize] as libc::c_int != '\u{0}' as i32 {
                        i += 1
                    }
                    if c[i as usize] as libc::c_int == '\"' as i32 {
                        c[i as usize] = '\u{0}' as i32 as libc::c_char
                    }
                    strcpy(pref_tmp_value.as_mut_ptr(),
                           b"-1\x00" as *const u8 as *const libc::c_char);
                    i = 0 as libc::c_int;
                    while i < max_q_idx as libc::c_int {
                        if streq(c.as_mut_ptr() as cptr,
                                 (*quest.offset(i as isize)).name.as_mut_ptr()
                                     as cptr) != 0 {
                            strnfmt(pref_tmp_value.as_mut_ptr(),
                                    8 as libc::c_int as uint_hack,
                                    b"%d\x00" as *const u8 as
                                        *const libc::c_char,
                                    (*quest.offset(i as isize)).status as
                                        libc::c_int);
                            break ;
                        } else { i += 1 }
                    }
                }
                v = pref_tmp_value.as_mut_ptr() as cptr
            } else if streq(b.offset(1 as libc::c_int as isize) as cptr,
                            b"VARIANT\x00" as *const u8 as
                                *const libc::c_char) != 0 {
                v = b"ToME\x00" as *const u8 as *const libc::c_char
            } else if streq(b.offset(1 as libc::c_int as isize) as cptr,
                            b"WILDERNESS\x00" as *const u8 as
                                *const libc::c_char) != 0 {
                v = b"NORMAL\x00" as *const u8 as *const libc::c_char
            }
        } else {
            /* Variant name */
            /* Wilderness */
            /* Constant */
            v = b as cptr
        }
    }
    /* Save */
    *fp = f;
    /* Save */
    *sp = s;
    /* Result */
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn process_dungeon_file(mut name: cptr,
                                              mut yval: *mut libc::c_int,
                                              mut xval: *mut libc::c_int,
                                              mut ymax: libc::c_int,
                                              mut xmax: libc::c_int,
                                              mut init: bool_,
                                              mut full: bool_) -> errr {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut num: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0;
    let mut err: errr = 0 as libc::c_int;
    let mut bypass: bool_ = 0 as libc::c_int as bool_;
    /* Save the start since it ought to be modified */
    let mut xmin: libc::c_int = *xval;
    if init != 0 {
        meta_sleep = 1 as libc::c_int as bool_;
        i = 0 as libc::c_int;
        while i < 255 as libc::c_int {
            letter[i as usize].defined = 0 as libc::c_int as bool_;
            if i == ' ' as i32 {
                letter[i as usize].ok = 1 as libc::c_int as bool_
            } else { letter[i as usize].ok = 0 as libc::c_int as bool_ }
            letter[i as usize].bx = 0 as libc::c_int;
            letter[i as usize].by = 0 as libc::c_int;
            i += 1
        }
    }
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT, name);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* No such file */
    if fp.is_null() {
        msg_format(b"Cannot find file %s at %s\x00" as *const u8 as
                       *const libc::c_char, name, buf.as_mut_ptr());
        return -(1 as libc::c_int)
    }
    /* Process the file */
    while 0 as libc::c_int ==
              my_fgets(fp, buf.as_mut_ptr(), 1024 as libc::c_int as huge_hack)
          {
        /* Count lines */
        num += 1;
        /* Skip "empty" lines */
        if buf[0 as libc::c_int as usize] == 0 { continue ; }
        /* Skip "blank" lines */
        if *(*__ctype_b_loc()).offset(buf[0 as libc::c_int as usize] as
                                          libc::c_int as isize) as libc::c_int
               & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
           {
            continue ;
        }
        /* Skip comments */
        if buf[0 as libc::c_int as usize] as libc::c_int == '#' as i32 {
            continue ;
        }
        /* Process "?:<expr>" */
        if buf[0 as libc::c_int as usize] as libc::c_int == '?' as i32 &&
               buf[1 as libc::c_int as usize] as libc::c_int == ':' as i32 {
            let mut f: libc::c_char = 0;
            let mut v: cptr = 0 as *const libc::c_char;
            let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
            /* Start */
            s = buf.as_mut_ptr().offset(2 as libc::c_int as isize);
            /* Parse the expr */
            v = process_dungeon_file_expr(&mut s, &mut f);
            /* Set flag */
            bypass =
                if streq(v, b"0\x00" as *const u8 as *const libc::c_char) as
                       libc::c_int != 0 {
                    1 as libc::c_int
                } else { 0 as libc::c_int } as bool_
        } else {
            /* Apply conditionals */
            if bypass != 0 { continue ; }
            /* Process "%:<file>" */
            if buf[0 as libc::c_int as usize] as libc::c_int == '%' as i32 {
                /* Process that file if allowed */
                process_dungeon_file(buf.as_mut_ptr().offset(2 as libc::c_int
                                                                 as isize) as
                                         cptr, yval, xval, ymax, xmax,
                                     0 as libc::c_int as bool_, full);
            } else {
                /* Process the line */
                err =
                    process_dungeon_file_aux(buf.as_mut_ptr(), yval, xval,
                                             xmin, ymax, xmax, full);
                /* Oops */
                if err != 0 { break ; }
            }
        }
    }
    /* Error */
    if err != 0 {
        /* Useful error message */
        msg_format(b"Error %d in line %d of file \'%s\'.\x00" as *const u8 as
                       *const libc::c_char, err, num, name);
        msg_format(b"Parsing \'%s\'\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
    }
    /* Close the file */
    my_fclose(fp);
    /* Result */
    return err;
}
