use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut deity_info_init: [deity_type; 6];
    #[no_mangle]
    static mut window_flag_desc: [cptr; 32];
    #[no_mangle]
    static mut option_info: [option_type; 0];
    #[no_mangle]
    static mut powers_type_init: [power_type; 62];
    #[no_mangle]
    static mut artifact_names_list: cptr;
    #[no_mangle]
    static mut cli_info: *mut cli_comm;
    #[no_mangle]
    static mut quest_init_tome: [quest_type; 26];
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    static mut rp_head: *mut header;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    static mut meta_class_info: *mut meta_class_type;
    #[no_mangle]
    fn plog(str: cptr);
    #[no_mangle]
    fn quit(str: cptr);
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn ralloc(len: huge_hack) -> vptr;
    #[no_mangle]
    fn string_make(str: cptr) -> cptr;
    #[no_mangle]
    fn string_free(str: cptr) -> errr;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    static mut scansubdir_result: [cptr; 255];
    #[no_mangle]
    fn Term_fresh() -> errr;
    #[no_mangle]
    fn Term_putstr(x: libc::c_int, y: libc::c_int, n: libc::c_int,
                   a: byte_hack, s: cptr) -> errr;
    #[no_mangle]
    fn Term_erase(x: libc::c_int, y: libc::c_int, n: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_clear() -> errr;
    #[no_mangle]
    static mut c_text: *mut libc::c_char;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    static mut macro__pat: *mut cptr;
    #[no_mangle]
    static mut macro__act: *mut cptr;
    #[no_mangle]
    static mut macro__cmd: *mut bool_;
    #[no_mangle]
    static mut macro__buf: *mut libc::c_char;
    #[no_mangle]
    static mut quark__str: *mut cptr;
    #[no_mangle]
    static mut message__tail: u16b;
    #[no_mangle]
    static mut message__ptr: *mut u16b;
    #[no_mangle]
    static mut message__color: *mut byte_hack;
    #[no_mangle]
    static mut message__type: *mut byte_hack;
    #[no_mangle]
    static mut message__count: *mut u16b;
    #[no_mangle]
    static mut message__buf: *mut libc::c_char;
    #[no_mangle]
    static mut option_flag: [u32b; 8];
    #[no_mangle]
    static mut option_mask: [u32b; 8];
    #[no_mangle]
    static mut window_mask: [u32b; 8];
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut o_list: *mut object_type;
    #[no_mangle]
    static mut m_list: *mut monster_type;
    #[no_mangle]
    static mut km_list: *mut monster_type;
    #[no_mangle]
    static mut max_real_towns: u16b;
    #[no_mangle]
    static mut max_towns: u16b;
    #[no_mangle]
    static mut town_info: *mut town_type;
    #[no_mangle]
    static mut alloc_kind_size: s16b;
    #[no_mangle]
    static mut alloc_kind_table: *mut alloc_entry;
    #[no_mangle]
    static mut alloc_race_size: s16b;
    #[no_mangle]
    static mut alloc_race_table: *mut alloc_entry;
    #[no_mangle]
    static mut p_ptr: *mut player_type;
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
    static mut e_text: *mut libc::c_char;
    #[no_mangle]
    static mut ra_head: *mut header;
    #[no_mangle]
    static mut ra_info: *mut randart_part_type;
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
    static mut ANGBAND_DIR: cptr;
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
    static mut max_dlv: *mut s16b;
    #[no_mangle]
    static mut generate_encounter: bool_;
    #[no_mangle]
    static mut m_allow_special: *mut bool_;
    #[no_mangle]
    static mut k_allow_special: *mut bool_;
    #[no_mangle]
    static mut a_allow_special: *mut bool_;
    #[no_mangle]
    static mut special_lvl: [*mut bool_; 128];
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
    static mut max_spells: s16b;
    #[no_mangle]
    static mut school_spells: *mut spell_type;
    #[no_mangle]
    static mut max_schools: s16b;
    #[no_mangle]
    static mut schools: *mut school_type;
    #[no_mangle]
    static mut effects: [effect_type; 128];
    #[no_mangle]
    static mut max_corruptions: s16b;
    #[no_mangle]
    static mut deity_info: *mut deity_type;
    #[no_mangle]
    static mut max_gods: s32b;
    #[no_mangle]
    fn wipe_hooks();
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn vinfo_init() -> errr;
    #[no_mangle]
    fn process_pref_file(name: cptr) -> errr;
    #[no_mangle]
    fn level_generate_maze() -> bool_;
    #[no_mangle]
    fn level_generate_life() -> bool_;
    #[no_mangle]
    fn add_level_generator(name: cptr,
                           generator: Option<unsafe extern "C" fn() -> bool_>,
                           stairs: bool_, monsters: bool_, objects: bool_,
                           miscs: bool_);
    #[no_mangle]
    fn level_generate_dungeon() -> bool_;
    #[no_mangle]
    fn init_player_info_txt(fp: *mut FILE, buf: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn init_ab_info_txt(fp: *mut FILE, buf: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn init_s_info_txt(fp: *mut FILE, buf: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn init_set_info_txt(fp: *mut FILE, buf: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn init_v_info_txt(fp: *mut FILE, buf: *mut libc::c_char, start: bool_)
     -> errr;
    #[no_mangle]
    fn init_f_info_txt(fp: *mut FILE, buf: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn init_k_info_txt(fp: *mut FILE, buf: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn init_a_info_txt(fp: *mut FILE, buf: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn init_al_info_txt(fp: *mut FILE, buf: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn init_ra_info_txt(fp: *mut FILE, buf: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn init_e_info_txt(fp: *mut FILE, buf: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn init_r_info_txt(fp: *mut FILE, buf: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn init_re_info_txt(fp: *mut FILE, buf: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn init_d_info_txt(fp: *mut FILE, buf: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn init_t_info_txt(fp: *mut FILE, buf: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn init_ba_info_txt(fp: *mut FILE, buf: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn init_st_info_txt(fp: *mut FILE, buf: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn init_ow_info_txt(fp: *mut FILE, buf: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn init_wf_info_txt(fp: *mut FILE, buf: *mut libc::c_char) -> errr;
    #[no_mangle]
    fn process_dungeon_file(name: cptr, yval: *mut libc::c_int,
                            xval: *mut libc::c_int, ymax: libc::c_int,
                            xmax: libc::c_int, init: bool_, full: bool_)
     -> errr;
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn my_fclose(fff: *mut FILE) -> errr;
    #[no_mangle]
    fn my_fopen(file: cptr, mode: cptr) -> *mut FILE;
    #[no_mangle]
    fn path_build(buf: *mut libc::c_char, max: libc::c_int, path: cptr,
                  file: cptr) -> errr;
    #[no_mangle]
    fn path_parse(buf: *mut libc::c_char, max: libc::c_int, file: cptr)
     -> errr;
    #[no_mangle]
    fn tome_dofile_anywhere(dir: cptr, file: *mut libc::c_char,
                            test_exist: bool_) -> bool_;
    #[no_mangle]
    fn build_prob(learn: cptr);
    #[no_mangle]
    fn quark_add(str: cptr) -> s16b;
    #[no_mangle]
    fn init_lua_init();
    #[no_mangle]
    fn init_lua();
    #[no_mangle]
    fn call_lua(function: cptr, args: cptr, ret: cptr, _: ...) -> bool_;
    #[no_mangle]
    fn fd_close(fd: libc::c_int) -> errr;
    #[no_mangle]
    fn fd_make(file: cptr, mode: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn fd_open(file: cptr, flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn display_message(x: libc::c_int, y: libc::c_int, split: libc::c_int,
                       color: byte_hack, t: cptr);
    #[no_mangle]
    fn my_fgets(fff: *mut FILE, buf: *mut libc::c_char, n: huge_hack) -> errr;
    #[no_mangle]
    fn select_module() -> bool_;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
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
pub type time_t = __time_t;
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
pub struct effect_type {
    pub time: s16b,
    pub dam: s16b,
    pub type_0: s16b,
    pub cy: s16b,
    pub cx: s16b,
    pub rad: s16b,
    pub flags: u32b,
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
pub struct alloc_entry {
    pub index: s16b,
    pub level: byte_hack,
    pub prob1: byte_hack,
    pub prob2: byte_hack,
    pub prob3: byte_hack,
    pub total: u16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option_type {
    pub o_var: *mut bool_,
    pub o_norm: byte_hack,
    pub o_page: byte_hack,
    pub o_bit: byte_hack,
    pub o_text: cptr,
    pub o_desc: cptr,
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
    pub abilities: [C2RustUnnamed; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
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
pub struct deity_type {
    pub name: cptr,
    pub desc: [[libc::c_char; 80]; 10],
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
    pub arts: [C2RustUnnamed_3; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
pub struct cli_comm {
    pub comm: cptr,
    pub descrip: cptr,
    pub key: s16b,
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
pub struct spell_type {
    pub name: cptr,
    pub skill_level: byte_hack,
    pub mana: byte_hack,
    pub mana_max: byte_hack,
    pub fail: s16b,
    pub level: s16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct school_type {
    pub name: cptr,
    pub skill: s16b,
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
 * Pointer to wilderness_map
 */
pub type wilderness_map_ptr = *mut wilderness_map;
/* File: init2.c */
/* Purpose: Initialisation (part 2) -BEN- */
/*
 * This file is used to initialise various variables and arrays for the
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
 *
 * The "init1.c" file is used only to parse the ascii template files.
 */
/*
 * Find the default paths to all of our important sub-directories.
 *
 * The purpose of each sub-directory is described in "variable.c".
 *
 * All of the sub-directories should, by default, be located inside
 * the main "lib" directory, whose location is very system dependant.
 *
 * This function takes a writable buffer, initially containing the
 * "path" to the "lib" directory, for example, "/pkg/lib/angband/",
 * or a system dependant string, for example, ":lib:".  The buffer
 * must be large enough to contain at least 32 more characters.
 *
 * Various command line options may allow some of the important
 * directories to be changed to user-specified directories, most
 * importantly, the "info" and "user" and "save" directories,
 * but this is done after this function, see "main.c".
 *
 * In general, the initial path should end in the appropriate "PATH_SEP"
 * string.  All of the "sub-directory" paths (created below or supplied
 * by the user) will NOT end in the "PATH_SEP" string, see the special
 * "path_build()" function in "util.c" for more information.
 *
 * Mega-Hack -- support fat raw files under NEXTSTEP, using special
 * "suffixed" directories for the "ANGBAND_DIR_DATA" directory, but
 * requiring the directories to be created by hand by the user.
 *
 * Hack -- first we free all the strings, since this is known
 * to succeed even if the strings have not been allocated yet,
 * as long as the variables start out as "NULL".  This allows
 * this function to be called multiple times, for example, to
 * try several base "path" values until a good one is found.
 */
#[no_mangle]
pub unsafe extern "C" fn init_file_paths(mut path: *mut libc::c_char) {
    let mut tail: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pathlen: libc::c_int = 0;
    /* ** Free everything ***/
    /* Free the main path */
    string_free(ANGBAND_DIR);
    /* Free the sub-paths */
    string_free(ANGBAND_DIR_APEX);
    string_free(ANGBAND_DIR_CORE);
    string_free(ANGBAND_DIR_DNGN);
    string_free(ANGBAND_DIR_DATA);
    string_free(ANGBAND_DIR_EDIT);
    string_free(ANGBAND_DIR_FILE);
    string_free(ANGBAND_DIR_HELP);
    string_free(ANGBAND_DIR_INFO);
    string_free(ANGBAND_DIR_MODULES);
    string_free(ANGBAND_DIR_NOTE);
    string_free(ANGBAND_DIR_SAVE);
    string_free(ANGBAND_DIR_SCPT);
    string_free(ANGBAND_DIR_PREF);
    string_free(ANGBAND_DIR_PATCH);
    string_free(ANGBAND_DIR_USER);
    string_free(ANGBAND_DIR_XTRA);
    string_free(ANGBAND_DIR_CMOV);
    /* ** Prepare the "path" ***/
    pathlen = strlen(path) as libc::c_int;
    /* Hack -- save the main directory without trailing PATH_SEP if present */
    if strlen(b"/\x00" as *const u8 as *const libc::c_char) >
           0 as libc::c_int as libc::c_ulong && pathlen > 0 as libc::c_int {
        let mut seplen: libc::c_int =
            strlen(b"/\x00" as *const u8 as *const libc::c_char) as
                libc::c_int;
        if strcmp(path.offset(pathlen as isize).offset(-(seplen as isize)),
                  b"/\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            *path.offset((pathlen - seplen) as isize) =
                '\u{0}' as i32 as libc::c_char;
            ANGBAND_DIR = string_make(path as cptr);
            *path.offset((pathlen - seplen) as isize) =
                *(b"/\x00" as *const u8 as *const libc::c_char)
        } else { ANGBAND_DIR = string_make(path as cptr) }
    } else { ANGBAND_DIR = string_make(path as cptr) }
    /* Prepare to append to the Base Path */
    tail = path.offset(pathlen as isize);
    /* ** Build the sub-directory names ***/
    /* Build a path name */
    strcpy(tail, b"apex\x00" as *const u8 as *const libc::c_char);
    ANGBAND_DIR_APEX = string_make(path as cptr);
    /* Build a path name */
    strcpy(tail, b"core\x00" as *const u8 as *const libc::c_char);
    ANGBAND_DIR_CORE = string_make(path as cptr);
    /* Build a path name */
    strcpy(tail, b"dngn\x00" as *const u8 as *const libc::c_char);
    ANGBAND_DIR_DNGN = string_make(path as cptr);
    /* Build a path name */
    strcpy(tail, b"data\x00" as *const u8 as *const libc::c_char);
    ANGBAND_DIR_DATA = string_make(path as cptr);
    /* Build a path name */
    strcpy(tail, b"edit\x00" as *const u8 as *const libc::c_char);
    ANGBAND_DIR_EDIT = string_make(path as cptr);
    /* Build a path name */
    strcpy(tail, b"file\x00" as *const u8 as *const libc::c_char);
    ANGBAND_DIR_FILE = string_make(path as cptr);
    /* Build a path name */
    strcpy(tail, b"help\x00" as *const u8 as *const libc::c_char);
    ANGBAND_DIR_HELP = string_make(path as cptr);
    /* Build a path name */
    strcpy(tail, b"info\x00" as *const u8 as *const libc::c_char);
    ANGBAND_DIR_INFO = string_make(path as cptr);
    /* Build a path name */
    strcpy(tail, b"mods\x00" as *const u8 as *const libc::c_char);
    ANGBAND_DIR_MODULES = string_make(path as cptr);
    /* Build a path name */
    strcpy(tail, b"patch\x00" as *const u8 as *const libc::c_char);
    ANGBAND_DIR_PATCH = string_make(path as cptr);
    /* Build a path name */
    strcpy(tail, b"scpt\x00" as *const u8 as *const libc::c_char);
    ANGBAND_DIR_SCPT = string_make(path as cptr);
    /* Build a path name */
    strcpy(tail, b"pref\x00" as *const u8 as *const libc::c_char);
    ANGBAND_DIR_PREF = string_make(path as cptr);
    /* synchronize with module_reset_dir */
    let mut user_path: [libc::c_char; 1024] = [0; 1024];
    /* Get an absolute path from the file name */
    path_parse(user_path.as_mut_ptr(), 1024 as libc::c_int,
               b"~/.tome\x00" as *const u8 as *const libc::c_char);
    strcat(user_path.as_mut_ptr(),
           b"/2.3\x00" as *const u8 as *const libc::c_char);
    ANGBAND_DIR_USER = string_make(user_path.as_mut_ptr() as cptr);
    ANGBAND_DIR_NOTE = string_make(user_path.as_mut_ptr() as cptr);
    ANGBAND_DIR_CMOV = string_make(user_path.as_mut_ptr() as cptr);
    ANGBAND_DIR_APEX = string_make(user_path.as_mut_ptr() as cptr);
    /* Savefiles are in user directory */
    strcat(user_path.as_mut_ptr(),
           b"/save\x00" as *const u8 as *const libc::c_char);
    ANGBAND_DIR_SAVE = string_make(user_path.as_mut_ptr() as cptr);
    /* Build a path name */
    strcpy(tail, b"xtra\x00" as *const u8 as *const libc::c_char);
    ANGBAND_DIR_XTRA = string_make(path as cptr);
    /* NeXT */
}
/* *
 * Realloc the given character array.
 */
unsafe extern "C" fn z_realloc(mut p: *mut *mut libc::c_char, mut n: size_t) {
    /* realloc doesn't really support size 0, but we want to shrink the allocated area regardless. */
    if n == 0 as libc::c_int as libc::c_ulong {
        n = 1 as libc::c_int as size_t
    }
    /* do the reallocation */
    *p = realloc(*p as *mut libc::c_void, n) as *mut libc::c_char;
    if (*p).is_null() {
        quit(b"Error during realloc.\x00" as *const u8 as
                 *const libc::c_char);
    };
}
/*
 * Hack -- help give useful error messages
 */
#[no_mangle]
pub static mut error_idx: s16b = 0;
#[no_mangle]
pub static mut error_line: s16b = 0;
/*
 * Hack -- help initialise the fake "name" and "text" arrays when
 * parsing an "ascii" template file.
 */
#[no_mangle]
pub static mut fake_name_size: u32b = 0;
#[no_mangle]
pub static mut fake_text_size: u32b = 0;
/*
 * Standard error message text
 */
static mut err_str: [cptr; 9] =
    [0 as cptr, b"parse error\x00" as *const u8 as *const libc::c_char,
     b"obsolete file\x00" as *const u8 as *const libc::c_char,
     b"missing record header\x00" as *const u8 as *const libc::c_char,
     b"non-sequential records\x00" as *const u8 as *const libc::c_char,
     b"invalid flag specification\x00" as *const u8 as *const libc::c_char,
     b"undefined directive\x00" as *const u8 as *const libc::c_char,
     b"out of memory\x00" as *const u8 as *const libc::c_char,
     b"invalid skill chart\x00" as *const u8 as *const libc::c_char];
/*
 * Hack -- take notes on line 23
 */
unsafe extern "C" fn note(mut str: cptr) {
    Term_erase(0 as libc::c_int, 23 as libc::c_int, 255 as libc::c_int);
    Term_putstr(20 as libc::c_int, 23 as libc::c_int, -(1 as libc::c_int),
                1 as libc::c_int as byte_hack, str);
    Term_fresh();
}
/*
 * Initialise the "f_info" array
 *
 * Note that we let each entry have a unique "name" and "text" string,
 * even if the string happens to be empty (everyone has a unique '\0').
 */
unsafe extern "C" fn init_f_info() -> errr {
    let mut err: errr = 0 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    /* General buffer */
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* ** Make the header ***/
    /* Allocate the "header" */
    f_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*f_head).info_num = max_f_idx;
    /* ** Make the fake arrays ***/
    /* Fake the size of "f_name" and "f_text" */
    fake_name_size =
        (40 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    fake_text_size =
        (120 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    /* Allocate the "f_info" array */
    f_info =
        memset(ralloc(((*f_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<feature_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*f_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<feature_type>()
                                                    as libc::c_ulong)) as
            *mut feature_type;
    /* Hack -- make "fake" arrays */
    f_name =
        memset(ralloc((fake_name_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_name_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    f_text =
        memset(ralloc((fake_text_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_text_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* ** Load the ascii template file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT,
               b"f_info.txt\x00" as *const u8 as *const libc::c_char);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Parse it */
    if fp.is_null() {
        quit(b"Cannot open \'f_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Parse the file */
    err = init_f_info_txt(fp, buf.as_mut_ptr());
    /* Close it */
    my_fclose(fp);
    /* Errors */
    if err != 0 {
        let mut oops: cptr = 0 as *const libc::c_char;
        /* Error string */
        oops =
            if err > 0 as libc::c_int && err < 8 as libc::c_int {
                err_str[err as usize]
            } else { b"unknown\x00" as *const u8 as *const libc::c_char };
        /* Oops */
        msg_format(b"Error %d at line %d of \'f_info.txt\'.\x00" as *const u8
                       as *const libc::c_char, err,
                   error_line as libc::c_int);
        msg_format(b"Record %d contains a \'%s\' error.\x00" as *const u8 as
                       *const libc::c_char, error_idx as libc::c_int, oops);
        msg_format(b"Parsing \'%s\'.\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        msg_print(0 as cptr);
        /* Quit */
        quit(b"Error in \'f_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Reduce sizes of the arrays */
    z_realloc(&mut f_name, (*f_head).name_size as size_t);
    z_realloc(&mut f_text, (*f_head).text_size as size_t);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialise the "k_info" array
 *
 * Note that we let each entry have a unique "name" and "text" string,
 * even if the string happens to be empty (everyone has a unique '\0').
 */
unsafe extern "C" fn init_k_info() -> errr {
    let mut err: errr = 0 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    /* General buffer */
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* ** Make the header ***/
    /* Allocate the "header" */
    k_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*k_head).info_num = max_k_idx;
    /* ** Make the fake arrays ***/
    /* Fake the size of "k_name" and "k_text" */
    fake_name_size =
        (40 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    fake_text_size =
        (120 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    /* Allocate the "k_info" array */
    k_info =
        memset(ralloc(((*k_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<object_kind>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*k_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<object_kind>()
                                                    as libc::c_ulong)) as
            *mut object_kind;
    /* Hack -- make "fake" arrays */
    k_name =
        memset(ralloc((fake_name_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_name_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    k_text =
        memset(ralloc((fake_text_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_text_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* ** Load the ascii template file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT,
               b"k_info.txt\x00" as *const u8 as *const libc::c_char);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Parse it */
    if fp.is_null() {
        quit(b"Cannot open \'k_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Parse the file */
    err = init_k_info_txt(fp, buf.as_mut_ptr());
    /* Close it */
    my_fclose(fp);
    /* Errors */
    if err != 0 {
        let mut oops: cptr = 0 as *const libc::c_char;
        /* Error string */
        oops =
            if err > 0 as libc::c_int && err < 8 as libc::c_int {
                err_str[err as usize]
            } else { b"unknown\x00" as *const u8 as *const libc::c_char };
        /* Oops */
        msg_format(b"Error %d at line %d of \'k_info.txt\'.\x00" as *const u8
                       as *const libc::c_char, err,
                   error_line as libc::c_int);
        msg_format(b"Record %d contains a \'%s\' error.\x00" as *const u8 as
                       *const libc::c_char, error_idx as libc::c_int, oops);
        msg_format(b"Parsing \'%s\'.\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        msg_print(0 as cptr);
        /* Quit */
        quit(b"Error in \'k_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Reduce sizes of the arrays */
    z_realloc(&mut k_name, (*k_head).name_size as size_t);
    z_realloc(&mut k_text, (*k_head).text_size as size_t);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialise the "set_info" array
 *
 * Note that we let each entry have a unique "name" and "text" string,
 * even if the string happens to be empty (everyone has a unique '\0').
 */
unsafe extern "C" fn init_set_info() -> errr {
    let mut err: errr = 0 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    /* General buffer */
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* ** Make the "header" ***/
    /* Allocate the "header" */
    set_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*set_head).info_num = max_set_idx as u16b;
    /* ** Make the fake arrays ***/
    /* Fake the size of "set_name" and "set_text" */
    fake_name_size =
        (40 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    fake_text_size =
        (120 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    /* Allocate the "set_info" array */
    set_info =
        memset(ralloc(((*set_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<set_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*set_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<set_type>()
                                                    as libc::c_ulong)) as
            *mut set_type;
    /* Hack -- make "fake" arrays */
    set_name =
        memset(ralloc((fake_name_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_name_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    set_text =
        memset(ralloc((fake_text_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_text_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* ** Load the ascii template file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT,
               b"set_info.txt\x00" as *const u8 as *const libc::c_char);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Parse it */
    if fp.is_null() {
        quit(b"Cannot open \'set_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Parse the file */
    err = init_set_info_txt(fp, buf.as_mut_ptr());
    /* Close it */
    my_fclose(fp);
    /* Errors */
    if err != 0 {
        let mut oops: cptr = 0 as *const libc::c_char;
        /* Error string */
        oops =
            if err > 0 as libc::c_int && err < 8 as libc::c_int {
                err_str[err as usize]
            } else { b"unknown\x00" as *const u8 as *const libc::c_char };
        /* Oops */
        msg_format(b"Error %d at line %d of \'set_info.txt\'.\x00" as
                       *const u8 as *const libc::c_char, err,
                   error_line as libc::c_int);
        msg_format(b"Record %d contains a \'%s\' error.\x00" as *const u8 as
                       *const libc::c_char, error_idx as libc::c_int, oops);
        msg_format(b"Parsing \'%s\'.\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        msg_print(0 as cptr);
        /* Quit */
        quit(b"Error in \'set_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Reduce sizes of the arrays */
    z_realloc(&mut set_name, (*set_head).name_size as size_t);
    z_realloc(&mut set_text, (*set_head).text_size as size_t);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialise the "a_info" array
 *
 * Note that we let each entry have a unique "name" and "text" string,
 * even if the string happens to be empty (everyone has a unique '\0').
 */
unsafe extern "C" fn init_a_info() -> errr {
    let mut err: errr = 0 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    /* General buffer */
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* ** Make the "header" ***/
    /* Allocate the "header" */
    a_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*a_head).info_num = max_a_idx;
    /* ** Make the fake arrays ***/
    /* Fake the size of "a_name" and "a_text" */
    fake_name_size =
        (40 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    fake_text_size =
        (120 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    /* Allocate the "a_info" array */
    a_info =
        memset(ralloc(((*a_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<artifact_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*a_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<artifact_type>()
                                                    as libc::c_ulong)) as
            *mut artifact_type;
    /* Hack -- make "fake" arrays */
    a_name =
        memset(ralloc((fake_name_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_name_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    a_text =
        memset(ralloc((fake_text_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_text_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* ** Load the ascii template file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT,
               b"a_info.txt\x00" as *const u8 as *const libc::c_char);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Parse it */
    if fp.is_null() {
        quit(b"Cannot open \'a_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Parse the file */
    err = init_a_info_txt(fp, buf.as_mut_ptr());
    /* Close it */
    my_fclose(fp);
    /* Errors */
    if err != 0 {
        let mut oops: cptr = 0 as *const libc::c_char;
        /* Error string */
        oops =
            if err > 0 as libc::c_int && err < 8 as libc::c_int {
                err_str[err as usize]
            } else { b"unknown\x00" as *const u8 as *const libc::c_char };
        /* Oops */
        msg_format(b"Error %d at line %d of \'a_info.txt\'.\x00" as *const u8
                       as *const libc::c_char, err,
                   error_line as libc::c_int);
        msg_format(b"Record %d contains a \'%s\' error.\x00" as *const u8 as
                       *const libc::c_char, error_idx as libc::c_int, oops);
        msg_format(b"Parsing \'%s\'.\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        msg_print(0 as cptr);
        /* Quit */
        quit(b"Error in \'a_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Reduce sizes of the arrays */
    z_realloc(&mut a_name, (*a_head).name_size as size_t);
    z_realloc(&mut a_text, (*a_head).text_size as size_t);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialise the "s_info" array
 *
 * Note that we let each entry have a unique "name" and "text" string,
 * even if the string happens to be empty (everyone has a unique '\0').
 */
unsafe extern "C" fn init_s_info() -> errr {
    let mut err: errr = 0 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    /* General buffer */
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* ** Make the "header" ***/
    /* Allocate the "header" */
    s_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*s_head).info_num = max_s_idx;
    /* ** Make the fake arrays ***/
    /* Fake the size of "a_name" and "a_text" */
    fake_name_size =
        (40 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    fake_text_size =
        (120 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    /* Allocate the "s_info" array */
    s_info =
        memset(ralloc(((*s_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<skill_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*s_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<skill_type>()
                                                    as libc::c_ulong)) as
            *mut skill_type;
    /* Hack -- make "fake" arrays */
    s_name =
        memset(ralloc((fake_name_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_name_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    s_text =
        memset(ralloc((fake_text_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_text_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* ** Load the ascii template file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT,
               b"s_info.txt\x00" as *const u8 as *const libc::c_char);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Parse it */
    if fp.is_null() {
        quit(b"Cannot open \'s_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Parse the file */
    err = init_s_info_txt(fp, buf.as_mut_ptr());
    /* Close it */
    my_fclose(fp);
    /* Errors */
    if err != 0 {
        let mut oops: cptr = 0 as *const libc::c_char;
        /* Error string */
        oops =
            if err > 0 as libc::c_int && err < 8 as libc::c_int {
                err_str[err as usize]
            } else { b"unknown\x00" as *const u8 as *const libc::c_char };
        /* Oops */
        msg_format(b"Error %d at line %d of \'s_info.txt\'.\x00" as *const u8
                       as *const libc::c_char, err,
                   error_line as libc::c_int);
        msg_format(b"Record %d contains a \'%s\' error.\x00" as *const u8 as
                       *const libc::c_char, error_idx as libc::c_int, oops);
        msg_format(b"Parsing \'%s\'.\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        msg_print(0 as cptr);
        /* Quit */
        quit(b"Error in \'s_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Reduce sizes of the arrays */
    z_realloc(&mut s_name, (*s_head).name_size as size_t);
    z_realloc(&mut s_text, (*s_head).text_size as size_t);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialise the "ab_info" array
 *
 * Note that we let each entry have a unique "name" and "text" string,
 * even if the string happens to be empty (everyone has a unique '\0').
 */
unsafe extern "C" fn init_ab_info() -> errr {
    let mut err: errr = 0 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    /* General buffer */
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* ** Make the "header" ***/
    /* Allocate the "header" */
    ab_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*ab_head).info_num = max_ab_idx;
    /* ** Make the fake arrays ***/
    /* Fake the size of "a_name" and "a_text" */
    fake_name_size =
        (40 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    fake_text_size =
        (120 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    /* Allocate the "ab_info" array */
    ab_info =
        memset(ralloc(((*ab_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<ability_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*ab_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<ability_type>()
                                                    as libc::c_ulong)) as
            *mut ability_type;
    /* Hack -- make "fake" arrays */
    ab_name =
        memset(ralloc((fake_name_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_name_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    ab_text =
        memset(ralloc((fake_text_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_text_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* ** Load the ascii template file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT,
               b"ab_info.txt\x00" as *const u8 as *const libc::c_char);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Parse it */
    if fp.is_null() {
        quit(b"Cannot open \'ab_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Parse the file */
    err = init_ab_info_txt(fp, buf.as_mut_ptr());
    /* Close it */
    my_fclose(fp);
    /* Errors */
    if err != 0 {
        let mut oops: cptr = 0 as *const libc::c_char;
        /* Error string */
        oops =
            if err > 0 as libc::c_int && err < 8 as libc::c_int {
                err_str[err as usize]
            } else { b"unknown\x00" as *const u8 as *const libc::c_char };
        /* Oops */
        msg_format(b"Error %d at line %d of \'ab_info.txt\'.\x00" as *const u8
                       as *const libc::c_char, err,
                   error_line as libc::c_int);
        msg_format(b"Record %d contains a \'%s\' error.\x00" as *const u8 as
                       *const libc::c_char, error_idx as libc::c_int, oops);
        msg_format(b"Parsing \'%s\'.\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        msg_print(0 as cptr);
        /* Quit */
        quit(b"Error in \'ab_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Reduce sizes of the arrays */
    z_realloc(&mut ab_name, (*ab_head).name_size as size_t);
    z_realloc(&mut ab_text, (*ab_head).text_size as size_t);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialise the "e_info" array
 *
 * Note that we let each entry have a unique "name" and "text" string,
 * even if the string happens to be empty (everyone has a unique '\0').
 */
unsafe extern "C" fn init_e_info() -> errr {
    let mut err: errr = 0 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    /* General buffer */
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* ** Make the "header" ***/
    /* Allocate the "header" */
    e_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*e_head).info_num = max_e_idx;
    /* ** Make the fake arrays ***/
    /* Fake the size of "e_name" and "e_text" */
    fake_name_size =
        (40 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    fake_text_size =
        (120 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    /* Allocate the "e_info" array */
    e_info =
        memset(ralloc(((*e_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<ego_item_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*e_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<ego_item_type>()
                                                    as libc::c_ulong)) as
            *mut ego_item_type;
    /* Hack -- make "fake" arrays */
    e_name =
        memset(ralloc((fake_name_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_name_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    e_text =
        memset(ralloc((fake_text_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_text_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* ** Load the ascii template file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT,
               b"e_info.txt\x00" as *const u8 as *const libc::c_char);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Parse it */
    if fp.is_null() {
        quit(b"Cannot open \'e_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Parse the file */
    err = init_e_info_txt(fp, buf.as_mut_ptr());
    /* Close it */
    my_fclose(fp);
    /* Errors */
    if err != 0 {
        let mut oops: cptr = 0 as *const libc::c_char;
        /* Error string */
        oops =
            if err > 0 as libc::c_int && err < 8 as libc::c_int {
                err_str[err as usize]
            } else { b"unknown\x00" as *const u8 as *const libc::c_char };
        /* Oops */
        msg_format(b"Error %d at line %d of \'e_info.txt\'.\x00" as *const u8
                       as *const libc::c_char, err,
                   error_line as libc::c_int);
        msg_format(b"Record %d contains a \'%s\' error.\x00" as *const u8 as
                       *const libc::c_char, error_idx as libc::c_int, oops);
        msg_format(b"Parsing \'%s\'.\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        msg_print(0 as cptr);
        /* Quit */
        quit(b"Error in \'e_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Reduce sizes of the arrays */
    z_realloc(&mut e_name, (*e_head).name_size as size_t);
    z_realloc(&mut e_text, (*e_head).text_size as size_t);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialise the "ra_info" array
 *
 * Note that we let each entry have a unique "name" and "text" string,
 * even if the string happens to be empty (everyone has a unique '\0').
 */
unsafe extern "C" fn init_ra_info() -> errr {
    let mut err: errr = 0 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    /* General buffer */
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* ** Make the "header" ***/
    /* Allocate the "header" */
    ra_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*ra_head).info_num = max_ra_idx;
    /* ** Make the fake arrays ***/
    /* Fake the size of "ra_name" and "ra_text" */
    fake_name_size =
        (40 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    fake_text_size =
        (120 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    /* Allocate the "ra_info" array */
    ra_info =
        memset(ralloc(((*ra_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<randart_part_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*ra_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<randart_part_type>()
                                                    as libc::c_ulong)) as
            *mut randart_part_type;
    /* ** Load the ascii template file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT,
               b"ra_info.txt\x00" as *const u8 as *const libc::c_char);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Parse it */
    if fp.is_null() {
        quit(b"Cannot open \'ra_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Parse the file */
    err = init_ra_info_txt(fp, buf.as_mut_ptr());
    /* Close it */
    my_fclose(fp);
    /* Errors */
    if err != 0 {
        let mut oops: cptr = 0 as *const libc::c_char;
        /* Error string */
        oops =
            if err > 0 as libc::c_int && err < 8 as libc::c_int {
                err_str[err as usize]
            } else { b"unknown\x00" as *const u8 as *const libc::c_char };
        /* Oops */
        msg_format(b"Error %d at line %d of \'ra_info.txt\'.\x00" as *const u8
                       as *const libc::c_char, err,
                   error_line as libc::c_int);
        msg_format(b"Record %d contains a \'%s\' error.\x00" as *const u8 as
                       *const libc::c_char, error_idx as libc::c_int, oops);
        msg_format(b"Parsing \'%s\'.\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        msg_print(0 as cptr);
        /* Quit */
        quit(b"Error in \'ra_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialise the "r_info" array
 *
 * Note that we let each entry have a unique "name" and "text" string,
 * even if the string happens to be empty (everyone has a unique '\0').
 */
unsafe extern "C" fn init_r_info() -> errr {
    let mut err: errr = 0 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    /* General buffer */
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* ** Make the header ***/
    /* Allocate the "header" */
    r_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*r_head).info_num = max_r_idx;
    /* ** Make the fake arrays ***/
    /* Assume the size of "r_name" and "r_text" */
    fake_name_size =
        (40 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    fake_text_size =
        (120 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    /* Allocate the "r_info" array */
    r_info =
        memset(ralloc(((*r_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<monster_race>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*r_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<monster_race>()
                                                    as libc::c_ulong)) as
            *mut monster_race;
    /* Hack -- make "fake" arrays */
    r_name =
        memset(ralloc((fake_name_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_name_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    r_text =
        memset(ralloc((fake_text_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_text_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* ** Load the ascii template file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT,
               b"r_info.txt\x00" as *const u8 as *const libc::c_char);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Parse it */
    if fp.is_null() {
        quit(b"Cannot open \'r_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Parse the file */
    err = init_r_info_txt(fp, buf.as_mut_ptr());
    /* Close it */
    my_fclose(fp);
    /* Errors */
    if err != 0 {
        let mut oops: cptr = 0 as *const libc::c_char;
        /* Error string */
        oops =
            if err > 0 as libc::c_int && err < 8 as libc::c_int {
                err_str[err as usize]
            } else { b"unknown\x00" as *const u8 as *const libc::c_char };
        /* Oops */
        msg_format(b"Error %d at line %d of \'r_info.txt\'.\x00" as *const u8
                       as *const libc::c_char, err,
                   error_line as libc::c_int);
        msg_format(b"Record %d contains a \'%s\' error.\x00" as *const u8 as
                       *const libc::c_char, error_idx as libc::c_int, oops);
        msg_format(b"Parsing \'%s\'.\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        msg_print(0 as cptr);
        /* Quit */
        quit(b"Error in \'r_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Reduce sizes of the arrays */
    z_realloc(&mut r_name, (*r_head).name_size as size_t);
    z_realloc(&mut r_text, (*r_head).text_size as size_t);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialise the "re_info" array
 *
 * Note that we let each entry have a unique "name" string,
 * even if the string happens to be empty (everyone has a unique '\0').
 */
unsafe extern "C" fn init_re_info() -> errr {
    let mut err: errr = 0 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    /* General buffer */
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* ** Make the header ***/
    /* Allocate the "header" */
    re_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*re_head).info_num = max_re_idx;
    /* ** Make the fake arrays ***/
    /* Assume the size of "re_name" */
    fake_name_size =
        (40 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    /* Allocate the "re_info" array */
    re_info =
        memset(ralloc(((*re_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<monster_ego>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*re_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<monster_ego>()
                                                    as libc::c_ulong)) as
            *mut monster_ego;
    /* Hack -- make "fake" arrays */
    re_name =
        memset(ralloc((fake_name_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_name_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* ** Load the ascii template file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT,
               b"re_info.txt\x00" as *const u8 as *const libc::c_char);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Parse it */
    if fp.is_null() {
        quit(b"Cannot open \'re_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Parse the file */
    err = init_re_info_txt(fp, buf.as_mut_ptr());
    /* Close it */
    my_fclose(fp);
    /* Errors */
    if err != 0 {
        let mut oops: cptr = 0 as *const libc::c_char;
        /* Error string */
        oops =
            if err > 0 as libc::c_int && err < 8 as libc::c_int {
                err_str[err as usize]
            } else { b"unknown\x00" as *const u8 as *const libc::c_char };
        /* Oops */
        msg_format(b"Error %d at line %d of \'re_info.txt\'.\x00" as *const u8
                       as *const libc::c_char, err,
                   error_line as libc::c_int);
        msg_format(b"Record %d contains a \'%s\' error.\x00" as *const u8 as
                       *const libc::c_char, error_idx as libc::c_int, oops);
        msg_format(b"Parsing \'%s\'.\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        msg_print(0 as cptr);
        /* Quit */
        quit(b"Error in \'re_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Reduce sizes of the arrays */
    z_realloc(&mut re_name, (*re_head).name_size as size_t);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialise the "d_info" array
 *
 * Note that we let each entry have a unique "name" and "short name" string,
 * even if the string happens to be empty (everyone has a unique '\0').
 */
unsafe extern "C" fn init_d_info() -> errr {
    let mut err: errr = 0 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    /* General buffer */
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* ** Make the header ***/
    /* Allocate the "header" */
    d_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*d_head).info_num = max_d_idx;
    /* ** Make the fake arrays ***/
    /* Assume the size of "d_name" and "d_text" */
    fake_name_size =
        (40 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    fake_text_size =
        (120 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    /* Allocate the "d_info" array */
    d_info =
        memset(ralloc(((*d_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<dungeon_info_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*d_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<dungeon_info_type>()
                                                    as libc::c_ulong)) as
            *mut dungeon_info_type;
    /* Hack -- make "fake" arrays */
    d_name =
        memset(ralloc((fake_name_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_name_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    d_text =
        memset(ralloc((fake_text_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_text_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* ** Load the ascii template file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT,
               b"d_info.txt\x00" as *const u8 as *const libc::c_char);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Parse it */
    if fp.is_null() {
        quit(b"Cannot open \'d_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Parse the file */
    err = init_d_info_txt(fp, buf.as_mut_ptr());
    /* Close it */
    my_fclose(fp);
    /* Errors */
    if err != 0 {
        let mut oops: cptr = 0 as *const libc::c_char;
        /* Error string */
        oops =
            if err > 0 as libc::c_int && err < 8 as libc::c_int {
                err_str[err as usize]
            } else { b"unknown\x00" as *const u8 as *const libc::c_char };
        /* Oops */
        msg_format(b"Error %d at line %d df \'d_info.txt\'.\x00" as *const u8
                       as *const libc::c_char, err,
                   error_line as libc::c_int);
        msg_format(b"Record %d contains a \'%s\' error.\x00" as *const u8 as
                       *const libc::c_char, error_idx as libc::c_int, oops);
        msg_format(b"Parsing \'%s\'.\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        msg_print(0 as cptr);
        /* Quit */
        quit(b"Error in \'d_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Reduce sizes of the arrays */
    z_realloc(&mut d_name, (*d_head).name_size as size_t);
    z_realloc(&mut d_text, (*d_head).text_size as size_t);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialise the "player" arrays
 *
 * Note that we let each entry have a unique "name" and "short name" string,
 * even if the string happens to be empty (everyone has a unique '\0').
 */
unsafe extern "C" fn init_player_info() -> errr {
    let mut i: libc::c_int = 0;
    let mut err: errr = 0 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    /* General buffer */
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* ** Make the header ***/
    /* Allocate the "header" */
    rp_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*rp_head).info_num = max_rp_idx;
    /* ** Make the header ***/
    /* Allocate the "header" */
    rmp_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*rmp_head).info_num = max_rmp_idx;
    /* ** Make the header ***/
    /* Allocate the "header" */
    c_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*c_head).info_num = max_c_idx;
    /* ** Make the fake arrays ***/
    /* Assume the size of "rp_name" and "rp_text" */
    fake_name_size =
        (40 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    fake_text_size =
        (120 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    /* Allocate the "rp_info" array */
    race_info =
        memset(ralloc(((*rp_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<player_race>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*rp_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<player_race>()
                                                    as libc::c_ulong)) as
            *mut player_race;
    /* Hack -- make "fake" arrays */
    rp_name =
        memset(ralloc((fake_name_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_name_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    rp_text =
        memset(ralloc((fake_text_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_text_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* Allocate the "rmp_info" array */
    race_mod_info =
        memset(ralloc(((*rmp_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<player_race_mod>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*rmp_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<player_race_mod>()
                                                    as libc::c_ulong)) as
            *mut player_race_mod;
    /* Hack -- make "fake" arrays */
    rmp_name =
        memset(ralloc((fake_name_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_name_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    rmp_text =
        memset(ralloc((fake_text_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_text_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* Allocate the "c_info" array */
    class_info =
        memset(ralloc(((*c_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<player_class>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*c_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<player_class>()
                                                    as libc::c_ulong)) as
            *mut player_class;
    /* Hack -- make "fake" arrays */
    c_name =
        memset(ralloc((fake_name_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_name_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    c_text =
        memset(ralloc((fake_text_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_text_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* Allocate the "bg" array */
    bg =
        memset(ralloc((max_bg_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<hist_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_bg_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<hist_type>()
                                                    as libc::c_ulong)) as
            *mut hist_type;
    /* Allocate the "meta_class" array */
    meta_class_info =
        memset(ralloc((max_mc_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<meta_class_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_mc_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<meta_class_type>()
                                                    as libc::c_ulong)) as
            *mut meta_class_type;
    i = 0 as libc::c_int;
    while i < max_mc_idx as libc::c_int {
        let ref mut fresh0 = (*meta_class_info.offset(i as isize)).classes;
        *fresh0 =
            memset(ralloc((max_c_idx as
                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<s16b>()
                                                               as
                                                               libc::c_ulong))
                       as *mut libc::c_char as *mut libc::c_void,
                   0 as libc::c_int,
                   (max_c_idx as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<s16b>()
                                                        as libc::c_ulong)) as
                *mut s16b;
        i += 1
    }
    /* ** Load the ascii template file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT,
               b"p_info.txt\x00" as *const u8 as *const libc::c_char);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Parse it */
    if fp.is_null() {
        quit(b"Cannot open \'p_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Parse the file */
    err = init_player_info_txt(fp, buf.as_mut_ptr());
    /* Close it */
    my_fclose(fp);
    /* Errors */
    if err != 0 {
        let mut oops: cptr = 0 as *const libc::c_char;
        /* Error string */
        oops =
            if err > 0 as libc::c_int && err < 8 as libc::c_int {
                err_str[err as usize]
            } else { b"unknown\x00" as *const u8 as *const libc::c_char };
        /* Oops */
        msg_format(b"Error %d at line %d df \'p_info.txt\'.\x00" as *const u8
                       as *const libc::c_char, err,
                   error_line as libc::c_int);
        msg_format(b"Record %d contains a \'%s\' error.\x00" as *const u8 as
                       *const libc::c_char, error_idx as libc::c_int, oops);
        msg_format(b"Parsing \'%s\'.\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        msg_print(0 as cptr);
        /* Quit */
        quit(b"Error in \'p_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Reallocate arrays. */
    z_realloc(&mut rp_name, (*rp_head).name_size as size_t);
    z_realloc(&mut rp_text, (*rp_head).text_size as size_t);
    z_realloc(&mut rmp_name, (*rmp_head).name_size as size_t);
    z_realloc(&mut rmp_text, (*rmp_head).text_size as size_t);
    z_realloc(&mut c_name, (*c_head).name_size as size_t);
    z_realloc(&mut c_text, (*c_head).text_size as size_t);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialise the "st_info" array
 *
 * Note that we let each entry have a unique "name" and "short name" string,
 * even if the string happens to be empty (everyone has a unique '\0').
 */
unsafe extern "C" fn init_st_info() -> errr {
    let mut err: errr = 0 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    /* General buffer */
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* ** Make the header ***/
    /* Allocate the "header" */
    st_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*st_head).info_num = max_st_idx;
    /* ** Make the fake arrays ***/
    /* Assume the size of "st_name" and "st_text" */
    fake_name_size =
        (40 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    /* Allocate the "st_info" array */
    st_info =
        memset(ralloc(((*st_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<store_info_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*st_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<store_info_type>()
                                                    as libc::c_ulong)) as
            *mut store_info_type;
    /* Hack -- make "fake" arrays */
    st_name =
        memset(ralloc((fake_name_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_name_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* ** Load the ascii template file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT,
               b"st_info.txt\x00" as *const u8 as *const libc::c_char);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Parse it */
    if fp.is_null() {
        quit(b"Cannot open \'st_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Parse the file */
    err = init_st_info_txt(fp, buf.as_mut_ptr());
    /* Close it */
    my_fclose(fp);
    /* Errors */
    if err != 0 {
        let mut oops: cptr = 0 as *const libc::c_char;
        /* Error string */
        oops =
            if err > 0 as libc::c_int && err < 8 as libc::c_int {
                err_str[err as usize]
            } else { b"unknown\x00" as *const u8 as *const libc::c_char };
        /* Oops */
        msg_format(b"Error %d at line %d df \'st_info.txt\'.\x00" as *const u8
                       as *const libc::c_char, err,
                   error_line as libc::c_int);
        msg_format(b"Record %d contains a \'%s\' error.\x00" as *const u8 as
                       *const libc::c_char, error_idx as libc::c_int, oops);
        msg_format(b"Parsing \'%s\'.\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        msg_print(0 as cptr);
        /* Quit */
        quit(b"Error in \'st_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Reduce sizes of the arrays */
    z_realloc(&mut st_name, (*st_head).name_size as size_t);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialise the "ow_info" array
 *
 * Note that we let each entry have a unique "name" and "short name" string,
 * even if the string happens to be empty (everyone has a unique '\0').
 */
unsafe extern "C" fn init_ow_info() -> errr {
    let mut err: errr = 0 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    /* General buffer */
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* ** Make the header ***/
    /* Allocate the "header" */
    ow_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*ow_head).info_num = max_ow_idx;
    /* ** Make the fake arrays ***/
    /* Assume the size of "ow_name" and "ow_text" */
    fake_name_size =
        (40 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    /* Allocate the "ow_info" array */
    ow_info =
        memset(ralloc(((*ow_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<owner_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*ow_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<owner_type>()
                                                    as libc::c_ulong)) as
            *mut owner_type;
    /* Hack -- make "fake" arrays */
    ow_name =
        memset(ralloc((fake_name_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_name_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* ** Load the ascii template file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT,
               b"ow_info.txt\x00" as *const u8 as *const libc::c_char);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Parse it */
    if fp.is_null() {
        quit(b"Cannot open \'ow_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Parse the file */
    err = init_ow_info_txt(fp, buf.as_mut_ptr());
    /* Close it */
    my_fclose(fp);
    /* Errors */
    if err != 0 {
        let mut oops: cptr = 0 as *const libc::c_char;
        /* Error string */
        oops =
            if err > 0 as libc::c_int && err < 8 as libc::c_int {
                err_str[err as usize]
            } else { b"unknown\x00" as *const u8 as *const libc::c_char };
        /* Oops */
        msg_format(b"Error %d at line %d df \'ow_info.txt\'.\x00" as *const u8
                       as *const libc::c_char, err,
                   error_line as libc::c_int);
        msg_format(b"Record %d contains a \'%s\' error.\x00" as *const u8 as
                       *const libc::c_char, error_idx as libc::c_int, oops);
        msg_format(b"Parsing \'%s\'.\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        msg_print(0 as cptr);
        /* Quit */
        quit(b"Error in \'ow_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Reduce sizes of the arrays */
    z_realloc(&mut ow_name, (*ow_head).name_size as size_t);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialise the "ba_info" array
 *
 * Note that we let each entry have a unique "name" and "short name" string,
 * even if the string happens to be empty (everyone has a unique '\0').
 */
unsafe extern "C" fn init_ba_info() -> errr {
    let mut err: errr = 0 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    /* General buffer */
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* ** Make the header ***/
    /* Allocate the "header" */
    ba_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*ba_head).info_num = max_ba_idx;
    /* ** Make the fake arrays ***/
    /* Assume the size of "ba_name" and "ba_text" */
    fake_name_size =
        (40 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    /* Allocate the "ba_info" array */
    ba_info =
        memset(ralloc(((*ba_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<store_action_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*ba_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<store_action_type>()
                                                    as libc::c_ulong)) as
            *mut store_action_type;
    /* Hack -- make "fake" arrays */
    ba_name =
        memset(ralloc((fake_name_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_name_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* ** Load the ascii template file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT,
               b"ba_info.txt\x00" as *const u8 as *const libc::c_char);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Parse it */
    if fp.is_null() {
        quit(b"Cannot open \'ba_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Parse the file */
    err = init_ba_info_txt(fp, buf.as_mut_ptr());
    /* Close it */
    my_fclose(fp);
    /* Errors */
    if err != 0 {
        let mut oops: cptr = 0 as *const libc::c_char;
        /* Error string */
        oops =
            if err > 0 as libc::c_int && err < 8 as libc::c_int {
                err_str[err as usize]
            } else { b"unknown\x00" as *const u8 as *const libc::c_char };
        /* Oops */
        msg_format(b"Error %d at line %d df \'ba_info.txt\'.\x00" as *const u8
                       as *const libc::c_char, err,
                   error_line as libc::c_int);
        msg_format(b"Record %d contains a \'%s\' error.\x00" as *const u8 as
                       *const libc::c_char, error_idx as libc::c_int, oops);
        msg_format(b"Parsing \'%s\'.\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        msg_print(0 as cptr);
        /* Quit */
        quit(b"Error in \'ba_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Reduce sizes of the arrays */
    z_realloc(&mut ba_name, (*ba_head).name_size as size_t);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialise the "wf_info" array
 *
 * Note that we let each entry have a unique "name" and "short name" string,
 * even if the string happens to be empty (everyone has a unique '\0').
 */
unsafe extern "C" fn init_wf_info() -> errr {
    let mut err: errr = 0 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    /* General buffer */
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* ** Make the header ***/
    /* Allocate the "header" */
    wf_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*wf_head).info_num = max_wf_idx;
    /* ** Make the fake arrays ***/
    /* Assume the size of "wf_name" and "wf_text" */
    fake_name_size =
        (40 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    fake_text_size =
        (120 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    /* Allocate the "r_info" array */
    wf_info =
        memset(ralloc(((*wf_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<wilderness_type_info>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*wf_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<wilderness_type_info>()
                                                    as libc::c_ulong)) as
            *mut wilderness_type_info;
    /* Hack -- make "fake" arrays */
    wf_name =
        memset(ralloc((fake_name_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_name_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    wf_text =
        memset(ralloc((fake_text_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_text_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* ** Load the ascii template file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT,
               b"wf_info.txt\x00" as *const u8 as *const libc::c_char);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Parse it */
    if fp.is_null() {
        quit(b"Cannot open \'wf_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Parse the file */
    err = init_wf_info_txt(fp, buf.as_mut_ptr());
    /* Close it */
    my_fclose(fp);
    /* Errors */
    if err != 0 {
        let mut oops: cptr = 0 as *const libc::c_char;
        /* Error string */
        oops =
            if err > 0 as libc::c_int && err < 8 as libc::c_int {
                err_str[err as usize]
            } else { b"unknown\x00" as *const u8 as *const libc::c_char };
        /* Oops */
        msg_format(b"Error %d at line %d df \'wf_info.txt\'.\x00" as *const u8
                       as *const libc::c_char, err,
                   error_line as libc::c_int);
        msg_format(b"Record %d contains a \'%s\' error.\x00" as *const u8 as
                       *const libc::c_char, error_idx as libc::c_int, oops);
        msg_format(b"Parsing \'%s\'.\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        msg_print(0 as cptr);
        /* Quit */
        quit(b"Error in \'wf_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Reduce sizes of the arrays */
    z_realloc(&mut wf_name, (*wf_head).name_size as size_t);
    z_realloc(&mut wf_text, (*wf_head).text_size as size_t);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialise the "t_info" array
 *
 * Note that we let each entry have a unique "name" and "text" string,
 * even if the string happens to be empty (everyone has a unique '\0').
 */
unsafe extern "C" fn init_t_info() -> errr {
    let mut err: errr = 0 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    /* General buffer */
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* ** Make the header ***/
    /* Allocate the "header" */
    t_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*t_head).info_num = max_t_idx;
    /* ** Make the fake arrays ***/
    /* Fake the size of "t_name" and "t_text" */
    fake_name_size =
        (40 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    fake_text_size =
        (120 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    /* Allocate the "t_info" array */
    t_info =
        memset(ralloc(((*t_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<trap_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*t_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<trap_type>()
                                                    as libc::c_ulong)) as
            *mut trap_type;
    /* Hack -- make "fake" arrays */
    t_name =
        memset(ralloc((fake_name_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_name_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    t_text =
        memset(ralloc((fake_text_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_text_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* ** Load the ascii template file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT,
               b"tr_info.txt\x00" as *const u8 as *const libc::c_char);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Parse it */
    if fp.is_null() {
        quit(b"Cannot open \'tr_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Parse the file */
    err = init_t_info_txt(fp, buf.as_mut_ptr());
    /* Close it */
    my_fclose(fp);
    /* Errors */
    if err != 0 {
        let mut oops: cptr = 0 as *const libc::c_char;
        /* Error string */
        oops =
            if err > 0 as libc::c_int && err < 8 as libc::c_int {
                err_str[err as usize]
            } else { b"unknown\x00" as *const u8 as *const libc::c_char };
        /* Oops */
        msg_format(b"Error %d at line %d of \'tr_info.txt\'.\x00" as *const u8
                       as *const libc::c_char, err,
                   error_line as libc::c_int);
        msg_format(b"Record %d contains a \'%s\' error.\x00" as *const u8 as
                       *const libc::c_char, error_idx as libc::c_int, oops);
        msg_format(b"Parsing \'%s\'.\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        msg_print(0 as cptr);
        /* Quit */
        quit(b"Error in \'tr_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Reduce sizes of the arrays */
    z_realloc(&mut t_name, (*t_head).name_size as size_t);
    z_realloc(&mut t_text, (*t_head).text_size as size_t);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialise the "al_info" array
 *
 * Not a flat array, but an array none the less
 */
#[no_mangle]
pub unsafe extern "C" fn init_al_info() -> errr {
    let mut err: errr = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    /* General buffer */
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* ** Make the header ***/
    /* Allocate the "header" */
    al_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*al_head).info_num = max_al_idx;
    fake_text_size =
        (120 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    fake_name_size =
        (40 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    /* Allocate the "al_info" array */
    alchemist_recipes =
        memset(ralloc(((*al_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<alchemist_recipe>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*al_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<alchemist_recipe>()
                                                    as libc::c_ulong)) as
            *mut alchemist_recipe;
    /* Allocate the fake arrays */
	/* ok, so we fudge a bit, but
	   fake text size will ALWAYS be larger
	   than 32*5*sizeof(artifact_select_flag) = 10 int and 5 bytes
	   which is the maximum size of the a_select_flags array
	   */
    al_name =
        memset(ralloc((fake_name_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_name_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    let mut hack: *mut libc::c_char = 0 as *mut libc::c_char;
    hack =
        memset(ralloc((fake_text_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_text_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    a_select_flags = hack as *mut artifact_select_flag;
    /* ** Load the ascii template file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT,
               b"al_info.txt\x00" as *const u8 as *const libc::c_char);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Parse it */
    if fp.is_null() {
        quit(b"Cannot open \'al_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Parse the file */
    err = init_al_info_txt(fp, buf.as_mut_ptr());
    /* Close it */
    my_fclose(fp);
    /* Errors */
    if err != 0 {
        let mut oops: cptr = 0 as *const libc::c_char;
        /* Error string */
        oops =
            if err > 0 as libc::c_int && err < 8 as libc::c_int {
                err_str[err as usize]
            } else { b"unknown\x00" as *const u8 as *const libc::c_char };
        /* Oops */
        msg_format(b"Error %d at line %d of \'al_info.txt\'.\x00" as *const u8
                       as *const libc::c_char, err,
                   error_line as libc::c_int);
        msg_format(b"Record %d contains a \'%s\' error.\x00" as *const u8 as
                       *const libc::c_char, error_idx as libc::c_int, oops);
        msg_format(b"Parsing \'%s\'.\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        msg_print(0 as cptr);
        /* Quit */
        quit(b"Error in \'al_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Reduce sizes of the arrays */
    z_realloc(&mut al_name, (*al_head).name_size as size_t);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialise the "v_info" array
 *
 * Note that we let each entry have a unique "name" and "text" string,
 * even if the string happens to be empty (everyone has a unique '\0').
 */
#[no_mangle]
pub unsafe extern "C" fn init_v_info() -> errr {
    let mut err: errr = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    /* General buffer */
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* ** Make the header ***/
    /* Allocate the "header" */
    v_head =
        memset(ralloc(::std::mem::size_of::<header>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<header>() as libc::c_ulong) as
            *mut header;
    /* Save the "record" information */
    (*v_head).info_num = max_v_idx;
    /* ** Make the fake arrays ***/
    /* Fake the size of "v_name" and "v_text" */
    fake_name_size =
        (40 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    fake_text_size =
        (120 as libc::c_int as libc::c_long * 1024 as libc::c_long) as u32b;
    /* Allocate the "k_info" array */
    v_info =
        memset(ralloc(((*v_head).info_num as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<vault_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((*v_head).info_num as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<vault_type>()
                                                    as libc::c_ulong)) as
            *mut vault_type;
    /* Hack -- make "fake" arrays */
    v_name =
        memset(ralloc((fake_name_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_name_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    v_text =
        memset(ralloc((fake_text_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (fake_text_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* ** Load the ascii template file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_EDIT,
               b"v_info.txt\x00" as *const u8 as *const libc::c_char);
    /* Open the file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Parse it */
    if fp.is_null() {
        quit(b"Cannot open \'v_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Parse the file */
    err = init_v_info_txt(fp, buf.as_mut_ptr(), 1 as libc::c_int as bool_);
    /* Close it */
    my_fclose(fp);
    /* Errors */
    if err != 0 {
        let mut oops: cptr = 0 as *const libc::c_char;
        /* Error string */
        oops =
            if err > 0 as libc::c_int && err < 8 as libc::c_int {
                err_str[err as usize]
            } else { b"unknown\x00" as *const u8 as *const libc::c_char };
        /* Oops */
        msg_format(b"Error %d at line %d of \'v_info.txt\'.\x00" as *const u8
                       as *const libc::c_char, err,
                   error_line as libc::c_int);
        msg_format(b"Record %d contains a \'%s\' error.\x00" as *const u8 as
                       *const libc::c_char, error_idx as libc::c_int, oops);
        msg_format(b"Parsing \'%s\'.\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        msg_print(0 as cptr);
        /* Quit */
        quit(b"Error in \'v_info.txt\' file.\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Reduce sizes of the arrays */
    z_realloc(&mut v_name, (*v_head).name_size as size_t);
    z_realloc(&mut v_text, (*v_head).text_size as size_t);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialize the very basic arrays
 */
unsafe extern "C" fn init_basic() {
    let mut i: libc::c_int = 0;
    /* Macro variables */
    macro__pat =
        memset(ralloc((256 as libc::c_int as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<cptr>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (256 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<cptr>()
                                                    as libc::c_ulong)) as
            *mut cptr;
    macro__act =
        memset(ralloc((256 as libc::c_int as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<cptr>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (256 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<cptr>()
                                                    as libc::c_ulong)) as
            *mut cptr;
    macro__cmd =
        memset(ralloc((256 as libc::c_int as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (256 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                    as libc::c_ulong)) as
            *mut bool_;
    /* Macro action buffer */
    macro__buf =
        memset(ralloc((1024 as libc::c_int as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (1024 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* Extended trigger macros */
    cli_info =
        memset(ralloc((128 as libc::c_int as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<cli_comm>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (128 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<cli_comm>()
                                                    as libc::c_ulong)) as
            *mut cli_comm;
    /* Wipe the directory list */
    i = 0 as libc::c_int;
    while i < 255 as libc::c_int {
        scansubdir_result[i as usize] = 0 as cptr;
        i += 1
    };
}
/*
 * Pseudo, dummy quest initializer, to actualy disable them
 */
unsafe extern "C" fn quest_disable_init_hook(mut q_idx: libc::c_int)
 -> bool_ {
    q_idx = q_idx;
    return 0 as libc::c_int as bool_;
}
/*
 * Initialise misc. values
 */
unsafe extern "C" fn init_misc() -> errr {
    let mut xstart: libc::c_int = 0 as libc::c_int;
    let mut ystart: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut allow_quest: s32b = 0;
    let mut allow_rquest: s32b = 0;
    /* ** Prepare the various "bizarre" arrays ***/
    /* Quark variables */
    quark__str =
        memset(ralloc((768 as libc::c_int as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<cptr>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (768 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<cptr>()
                                                    as libc::c_ulong)) as
            *mut cptr;
    /* Message variables */
    message__ptr =
        memset(ralloc((2048 as libc::c_int as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<u16b>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (2048 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<u16b>()
                                                    as libc::c_ulong)) as
            *mut u16b;
    message__color =
        memset(ralloc((2048 as libc::c_int as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (2048 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                    as libc::c_ulong)) as
            *mut byte_hack;
    message__type =
        memset(ralloc((2048 as libc::c_int as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (2048 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                    as libc::c_ulong)) as
            *mut byte_hack;
    message__count =
        memset(ralloc((2048 as libc::c_int as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<u16b>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (2048 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<u16b>()
                                                    as libc::c_ulong)) as
            *mut u16b;
    message__buf =
        memset(ralloc((32768 as libc::c_int as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (32768 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    /* Hack -- No messages yet */
    message__tail = 32768 as libc::c_int as u16b;
    /* Prepare powers */
    (*p_ptr).powers = 0 as *mut bool_;
    powers_type = 0 as *mut power_type;
    power_max = 62 as libc::c_int as s16b;
    reinit_powers_type(power_max);
    memcpy(powers_type as *mut libc::c_char as *mut libc::c_void,
           powers_type_init.as_mut_ptr() as *mut libc::c_char as
               *const libc::c_void,
           (62 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<power_type>()
                                                as libc::c_ulong));
    /* Prepare quests */
    call_lua(b"get_module_info\x00" as *const u8 as *const libc::c_char,
             b"(s)\x00" as *const u8 as *const libc::c_char,
             b"d\x00" as *const u8 as *const libc::c_char,
             b"C_quest\x00" as *const u8 as *const libc::c_char,
             &mut allow_quest as *mut s32b);
    call_lua(b"get_module_info\x00" as *const u8 as *const libc::c_char,
             b"(s)\x00" as *const u8 as *const libc::c_char,
             b"d\x00" as *const u8 as *const libc::c_char,
             b"rand_quest\x00" as *const u8 as *const libc::c_char,
             &mut allow_rquest as *mut s32b);
    quest = 0 as *mut quest_type;
    max_q_idx = 26 as libc::c_int as s16b;
    reinit_quests(max_q_idx);
    memcpy(quest as *mut libc::c_char as *mut libc::c_void,
           quest_init_tome.as_mut_ptr() as *mut libc::c_char as
               *const libc::c_void,
           (26 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<quest_type>()
                                                as libc::c_ulong));
    /* If we dont allow C quests, we dont let them init */
    if allow_quest == 0 {
        i = 0 as libc::c_int;
        while i < 26 as libc::c_int {
            if !(allow_rquest != 0 && i == 5 as libc::c_int) {
                let ref mut fresh1 = (*quest.offset(i as isize)).init;
                *fresh1 =
                    Some(quest_disable_init_hook as
                             unsafe extern "C" fn(_: libc::c_int) -> bool_)
            }
            i += 1
        }
    }
    /* Prepare gods */
    deity_info = 0 as *mut deity_type;
    max_gods = 6 as libc::c_int;
    reinit_gods(max_gods as s16b);
    memcpy(deity_info as *mut libc::c_char as *mut libc::c_void,
           deity_info_init.as_mut_ptr() as *mut libc::c_char as
               *const libc::c_void,
           (6 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<deity_type>()
                                                as libc::c_ulong));
    /* Prepare schools */
    max_spells = 0 as libc::c_int as s16b;
    max_schools = 0 as libc::c_int as s16b;
    schools = 0 as *mut school_type;
    school_spells = 0 as *mut spell_type;
    process_hooks(37 as libc::c_int,
                  b"(s)\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char,
                  b"begin\x00" as *const u8 as *const libc::c_char);
    /* Initialise the values */
    process_dungeon_file(b"misc.txt\x00" as *const u8 as *const libc::c_char,
                         &mut ystart, &mut xstart, 0 as libc::c_int,
                         0 as libc::c_int, 1 as libc::c_int as bool_,
                         0 as libc::c_int as bool_);
    /* Init the spell effects */
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        effects[i as usize].time = 0 as libc::c_int as s16b;
        i += 1
    }
    return 0 as libc::c_int;
}
/*
 * Initialise town array
 */
unsafe extern "C" fn init_towns() -> errr {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    /* ** Prepare the Towns ***/
    /* Allocate the towns */
    town_info =
        memset(ralloc((max_towns as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<town_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_towns as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<town_type>()
                                                    as libc::c_ulong)) as
            *mut town_type;
    i = 1 as libc::c_int;
    while i < max_towns as libc::c_int {
        if i <= max_real_towns as libc::c_int {
            let ref mut fresh2 = (*town_info.offset(i as isize)).flags;
            *fresh2 =
                (*fresh2 as libc::c_int | 0x1 as libc::c_int) as byte_hack
        }
        /* Allocate the stores */
        let ref mut fresh3 = (*town_info.offset(i as isize)).store;
        *fresh3 =
            memset(ralloc((max_st_idx as
                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<store_type>()
                                                               as
                                                               libc::c_ulong))
                       as *mut libc::c_char as *mut libc::c_void,
                   0 as libc::c_int,
                   (max_st_idx as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<store_type>()
                                                        as libc::c_ulong)) as
                *mut store_type;
        /* Fill in each store */
        j = 0 as libc::c_int;
        while j < max_st_idx as libc::c_int {
            /* Access the store */
            let mut st_ptr: *mut store_type =
                &mut *(*town_info.offset(i as isize)).store.offset(j as isize)
                    as *mut store_type;
            /* Know who we are */
            (*st_ptr).st_idx = j as u16b;
            /* Assume full stock */
            (*st_ptr).stock_size = 0 as libc::c_int as s16b;
            j += 1
        }
        i += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn create_stores_stock(mut t: libc::c_int) {
    let mut j: libc::c_int = 0;
    let mut t_ptr: *mut town_type =
        &mut *town_info.offset(t as isize) as *mut town_type;
    if (*t_ptr).stocked != 0 { return }
    j = 0 as libc::c_int;
    while j < max_st_idx as libc::c_int {
        let mut st_ptr: *mut store_type =
            &mut *(*t_ptr).store.offset(j as isize) as *mut store_type;
        /* Assume full stock */
        (*st_ptr).stock_size = (*st_info.offset(j as isize)).max_obj;
        /* Allocate the stock */
        (*st_ptr).stock =
            memset(ralloc(((*st_ptr).stock_size as
                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<object_type>()
                                                               as
                                                               libc::c_ulong))
                       as *mut libc::c_char as *mut libc::c_void,
                   0 as libc::c_int,
                   ((*st_ptr).stock_size as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<object_type>()
                                                        as libc::c_ulong)) as
                *mut object_type;
        j += 1
    }
    (*t_ptr).stocked = 1 as libc::c_int as bool_;
}
/*
 * Initialise wilderness map array
 */
unsafe extern "C" fn init_wilderness() -> errr {
    let mut i: libc::c_int = 0;
    /* Allocate the wilderness (two-dimension array) */
    wild_map =
        memset(ralloc((max_wild_y as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<wilderness_map_ptr>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_wild_y as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<wilderness_map_ptr>()
                                                    as libc::c_ulong)) as
            *mut *mut wilderness_map;
    let ref mut fresh4 = *wild_map.offset(0 as libc::c_int as isize);
    *fresh4 =
        memset(ralloc(((max_wild_x as libc::c_int * max_wild_y as libc::c_int)
                           as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<wilderness_map>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((max_wild_x as libc::c_int * max_wild_y as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<wilderness_map>()
                                                    as libc::c_ulong)) as
            *mut wilderness_map;
    /* Init the other pointers */
    i = 1 as libc::c_int;
    while i < max_wild_y as libc::c_int {
        let ref mut fresh5 = *wild_map.offset(i as isize);
        *fresh5 =
            (*wild_map.offset(0 as libc::c_int as
                                  isize)).offset((i *
                                                      max_wild_x as
                                                          libc::c_int) as
                                                     isize);
        i += 1
    }
    /* No encounter right now */
    generate_encounter = 0 as libc::c_int as bool_;
    return 0 as libc::c_int;
}
/*
 * XXX XXX XXX XXX XXX Realloc is not guaranteed to work (see main-gtk.c
 * and main-mac.c.
 */
#[no_mangle]
pub unsafe extern "C" fn reinit_powers_type(mut new_size: s16b) {
    let mut new_powers_type: *mut power_type = 0 as *mut power_type;
    let mut new_powers: *mut bool_ = 0 as *mut bool_;
    new_powers_type =
        memset(ralloc((new_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<power_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (new_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<power_type>()
                                                    as libc::c_ulong)) as
            *mut power_type;
    new_powers =
        memset(ralloc((new_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (new_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                    as libc::c_ulong)) as
            *mut bool_;
    /* Reallocate the extra memory */
    if !powers_type.is_null() && !(*p_ptr).powers.is_null() {
        memcpy(new_powers_type as *mut libc::c_char as *mut libc::c_void,
               powers_type as *mut libc::c_char as *const libc::c_void,
               (power_max as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<power_type>()
                                                    as libc::c_ulong));
        memcpy(new_powers as *mut libc::c_char as *mut libc::c_void,
               (*p_ptr).powers as *mut libc::c_char as *const libc::c_void,
               (power_max as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                    as libc::c_ulong));
        rnfree(powers_type as vptr,
               (power_max as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<power_type>()
                                                    as libc::c_ulong));
        rnfree((*p_ptr).powers as vptr,
               (power_max as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                    as libc::c_ulong));
    }
    powers_type = new_powers_type;
    (*p_ptr).powers = new_powers;
    power_max = new_size;
}
#[no_mangle]
pub unsafe extern "C" fn reinit_quests(mut new_size: s16b) {
    let mut new_quest: *mut quest_type = 0 as *mut quest_type;
    new_quest =
        memset(ralloc((new_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<quest_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (new_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<quest_type>()
                                                    as libc::c_ulong)) as
            *mut quest_type;
    /* Reallocate the extra memory */
    if !quest.is_null() {
        memcpy(new_quest as *mut libc::c_char as *mut libc::c_void,
               quest as *mut libc::c_char as *const libc::c_void,
               (max_q_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<quest_type>()
                                                    as libc::c_ulong));
        rnfree(quest as vptr,
               (max_q_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<quest_type>()
                                                    as libc::c_ulong));
    }
    quest = new_quest;
    max_q_idx = new_size;
}
#[no_mangle]
pub unsafe extern "C" fn reinit_gods(mut new_size: s16b) {
    let mut new_deity: *mut deity_type = 0 as *mut deity_type;
    new_deity =
        memset(ralloc((new_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<deity_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (new_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<deity_type>()
                                                    as libc::c_ulong)) as
            *mut deity_type;
    /* Reallocate the extra memory */
    if !deity_info.is_null() {
        memcpy(new_deity as *mut libc::c_char as *mut libc::c_void,
               deity_info as *mut libc::c_char as *const libc::c_void,
               (max_gods as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<deity_type>()
                                                    as libc::c_ulong));
        rnfree(deity_info as vptr,
               (max_gods as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<deity_type>()
                                                    as libc::c_ulong));
    }
    deity_info = new_deity;
    max_gods = new_size as s32b;
}
#[no_mangle]
pub unsafe extern "C" fn init_spells(mut new_size: s16b) {
    /* allocate the extra memory */
    school_spells =
        memset(ralloc((new_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<spell_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (new_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<spell_type>()
                                                    as libc::c_ulong)) as
            *mut spell_type;
    max_spells = new_size;
}
#[no_mangle]
pub unsafe extern "C" fn init_schools(mut new_size: s16b) {
    /* allocate the extra memory */
    schools =
        memset(ralloc((new_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<school_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (new_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<school_type>()
                                                    as libc::c_ulong)) as
            *mut school_type;
    max_schools = new_size;
}
#[no_mangle]
pub unsafe extern "C" fn init_corruptions(mut new_size: s16b) {
    /* allocate the extra memory */
    (*p_ptr).corruptions =
        memset(ralloc((new_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (new_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                    as libc::c_ulong)) as
            *mut bool_;
    max_corruptions = new_size;
}
/*
 * Initialise some other arrays
 */
unsafe extern "C" fn init_other() -> errr {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    /* ** Prepare the "dungeon" information ***/
    /* Allocate and Wipe the special gene flags */
    m_allow_special =
        memset(ralloc((max_r_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_r_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                    as libc::c_ulong)) as
            *mut bool_;
    k_allow_special =
        memset(ralloc((max_k_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_k_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                    as libc::c_ulong)) as
            *mut bool_;
    a_allow_special =
        memset(ralloc((max_a_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_a_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                    as libc::c_ulong)) as
            *mut bool_;
    /* ** Prepare "vinfo" array ***/
    /* Used by "update_view()" */
    vinfo_init();
    /* Allocate and Wipe the object list */
    o_list =
        memset(ralloc((max_o_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<object_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_o_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<object_type>()
                                                    as libc::c_ulong)) as
            *mut object_type;
    /* Allocate and Wipe the monster list */
    m_list =
        memset(ralloc((max_m_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<monster_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_m_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<monster_type>()
                                                    as libc::c_ulong)) as
            *mut monster_type;
    /* Allocate and Wipe the to keep monster list */
    km_list =
        memset(ralloc((max_m_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<monster_type>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_m_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<monster_type>()
                                                    as libc::c_ulong)) as
            *mut monster_type;
    /* Allocate and Wipe the max dungeon level */
    max_dlv =
        memset(ralloc((max_d_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<s16b>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_d_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<s16b>()
                                                    as libc::c_ulong)) as
            *mut s16b;
    /* Allocate and Wipe the special levels */
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        special_lvl[i as usize] =
            memset(ralloc((max_d_idx as
                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                               as
                                                               libc::c_ulong))
                       as *mut libc::c_char as *mut libc::c_void,
                   0 as libc::c_int,
                   (max_d_idx as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<bool_>()
                                                        as libc::c_ulong)) as
                *mut bool_;
        i += 1
    }
    /* Allocate and wipe each line of the cave */
    i = 0 as libc::c_int;
    while i < 66 as libc::c_int {
        /* Allocate one row of the cave */
        cave[i as usize] =
            memset(ralloc((198 as libc::c_int as
                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<cave_type>()
                                                               as
                                                               libc::c_ulong))
                       as *mut libc::c_char as *mut libc::c_void,
                   0 as libc::c_int,
                   (198 as libc::c_int as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<cave_type>()
                                                        as libc::c_ulong)) as
                *mut cave_type;
        i += 1
    }
    /* ** Pre-allocate the basic "auto-inscriptions" ***/
    /* The "basic" feelings */
    quark_add(b"cursed\x00" as *const u8 as *const libc::c_char);
    quark_add(b"broken\x00" as *const u8 as *const libc::c_char);
    quark_add(b"average\x00" as *const u8 as *const libc::c_char);
    quark_add(b"good\x00" as *const u8 as *const libc::c_char);
    /* The "extra" feelings */
    quark_add(b"excellent\x00" as *const u8 as *const libc::c_char);
    quark_add(b"worthless\x00" as *const u8 as *const libc::c_char);
    quark_add(b"special\x00" as *const u8 as *const libc::c_char);
    quark_add(b"terrible\x00" as *const u8 as *const libc::c_char);
    /* Some extra strings */
    quark_add(b"uncursed\x00" as *const u8 as *const libc::c_char);
    quark_add(b"on sale\x00" as *const u8 as *const libc::c_char);
    /* ** Prepare the options ***/
    /* Scan the options */
    i = 0 as libc::c_int;
    while !(*option_info.as_mut_ptr().offset(i as isize)).o_desc.is_null() {
        let mut os: libc::c_int =
            (*option_info.as_mut_ptr().offset(i as isize)).o_page as
                libc::c_int;
        let mut ob: libc::c_int =
            (*option_info.as_mut_ptr().offset(i as isize)).o_bit as
                libc::c_int;
        /* Set the "default" options */
        if !(*option_info.as_mut_ptr().offset(i as isize)).o_var.is_null() {
            /* Accept */
            option_mask[os as usize] =
                (option_mask[os as usize] as libc::c_long |
                     (1 as libc::c_long) << ob) as u32b;
            /* Set */
            if (*option_info.as_mut_ptr().offset(i as isize)).o_norm != 0 {
                /* Set */
                option_flag[os as usize] =
                    (option_flag[os as usize] as libc::c_long |
                         (1 as libc::c_long) << ob) as u32b
            } else {
                /* Clear */
                /* Clear */
                option_flag[os as usize] =
                    (option_flag[os as usize] as libc::c_long &
                         !((1 as libc::c_long) << ob)) as u32b
            }
        }
        i += 1
    }
    /* Analyze the windows */
    n = 0 as libc::c_int;
    while n < 8 as libc::c_int {
        /* Analyze the options */
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            /* Accept */
            if !window_flag_desc[i as usize].is_null() {
                /* Accept */
                window_mask[n as usize] =
                    (window_mask[n as usize] as libc::c_long |
                         (1 as libc::c_long) << i) as u32b
            }
            i += 1
        }
        n += 1
    }
    /*
	 * Install the various level generators
	 */
    add_level_generator(b"dungeon\x00" as *const u8 as *const libc::c_char,
                        Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                         -> bool_,
                                                     unsafe extern "C" fn()
                                                         ->
                                                             bool_>(level_generate_dungeon)),
                        1 as libc::c_int as bool_, 1 as libc::c_int as bool_,
                        1 as libc::c_int as bool_, 1 as libc::c_int as bool_);
    add_level_generator(b"maze\x00" as *const u8 as *const libc::c_char,
                        Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                         -> bool_,
                                                     unsafe extern "C" fn()
                                                         ->
                                                             bool_>(level_generate_maze)),
                        1 as libc::c_int as bool_, 1 as libc::c_int as bool_,
                        1 as libc::c_int as bool_, 1 as libc::c_int as bool_);
    add_level_generator(b"life\x00" as *const u8 as *const libc::c_char,
                        Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                         -> bool_,
                                                     unsafe extern "C" fn()
                                                         ->
                                                             bool_>(level_generate_life)),
                        1 as libc::c_int as bool_, 1 as libc::c_int as bool_,
                        1 as libc::c_int as bool_, 1 as libc::c_int as bool_);
    /* ** Pre-allocate space for the "format()" buffer ***/
    /* Hack -- Just call the "format()" function */
    format(b"%s (%s).\x00" as *const u8 as *const libc::c_char,
           b"Dark God <darkgod@t-o-m-e.net>\x00" as *const u8 as
               *const libc::c_char,
           b"darkgod@t-o-m-e.net\x00" as *const u8 as *const libc::c_char);
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Initialise some other arrays
 */
unsafe extern "C" fn init_alloc() -> errr {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k_ptr: *mut object_kind = 0 as *mut object_kind;
    let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
    let mut table: *mut alloc_entry = 0 as *mut alloc_entry;
    let mut num: [s16b; 200] = [0; 200];
    let mut aux: [s16b; 200] = [0; 200];
    /* ** Analyze object allocation info ***/
    /* Clear the "aux" array */
    memset(&mut aux as *mut [s16b; 200] as *mut libc::c_char as
               *mut libc::c_void, 0 as libc::c_int,
           (200 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<s16b>() as
                                                libc::c_ulong));
    /* Clear the "num" array */
    memset(&mut num as *mut [s16b; 200] as *mut libc::c_char as
               *mut libc::c_void, 0 as libc::c_int,
           (200 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<s16b>() as
                                                libc::c_ulong));
    /* Size of "alloc_kind_table" */
    alloc_kind_size = 0 as libc::c_int as s16b;
    /* Scan the objects */
    i = 1 as libc::c_int;
    while i < max_k_idx as libc::c_int {
        k_ptr = &mut *k_info.offset(i as isize) as *mut object_kind;
        /* Scan allocation pairs */
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            /* Count the "legal" entries */
            if (*k_ptr).chance[j as usize] != 0 {
                /* Count the entries */
                alloc_kind_size += 1;
                /* Group by level */
                num[(*k_ptr).locale[j as usize] as usize] += 1
            }
            j += 1
        }
        i += 1
    }
    /* Collect the level indexes */
    i = 1 as libc::c_int;
    while i < 200 as libc::c_int {
        /* Group by level */
        num[i as usize] =
            (num[i as usize] as libc::c_int +
                 num[(i - 1 as libc::c_int) as usize] as libc::c_int) as s16b;
        i += 1
    }
    /* Paranoia */
    if num[0 as libc::c_int as usize] == 0 {
        quit(b"No town objects!\x00" as *const u8 as *const libc::c_char);
    }
    /* ** Initialise object allocation info ***/
    /* Allocate the alloc_kind_table */
    alloc_kind_table =
        memset(ralloc((alloc_kind_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<alloc_entry>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (alloc_kind_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<alloc_entry>()
                                                    as libc::c_ulong)) as
            *mut alloc_entry;
    /* Access the table entry */
    table = alloc_kind_table;
    /* Scan the objects */
    i = 1 as libc::c_int;
    while i < max_k_idx as libc::c_int {
        k_ptr = &mut *k_info.offset(i as isize) as *mut object_kind;
        /* Scan allocation pairs */
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            /* Count the "legal" entries */
            if (*k_ptr).chance[j as usize] != 0 {
                let mut p: libc::c_int = 0;
                let mut x: libc::c_int = 0;
                let mut y: libc::c_int = 0;
                let mut z: libc::c_int = 0;
                /* Extract the base level */
                x = (*k_ptr).locale[j as usize] as libc::c_int;
                /* Extract the base probability */
                p =
                    100 as libc::c_int /
                        (*k_ptr).chance[j as usize] as libc::c_int;
                /* Skip entries preceding our locale */
                y =
                    if x > 0 as libc::c_int {
                        num[(x - 1 as libc::c_int) as usize] as libc::c_int
                    } else { 0 as libc::c_int };
                /* Skip previous entries at this locale */
                z = y + aux[x as usize] as libc::c_int;
                /* Load the entry */
                (*table.offset(z as isize)).index = i as s16b;
                (*table.offset(z as isize)).level = x as byte_hack;
                (*table.offset(z as isize)).prob1 = p as byte_hack;
                (*table.offset(z as isize)).prob2 = p as byte_hack;
                (*table.offset(z as isize)).prob3 = p as byte_hack;
                /* Another entry complete for this locale */
                aux[x as usize] += 1
            }
            j += 1
        }
        i += 1
    }
    /* ** Analyze monster allocation info ***/
    /* Clear the "aux" array */
    memset(&mut aux as *mut [s16b; 200] as *mut libc::c_char as
               *mut libc::c_void, 0 as libc::c_int,
           (200 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<s16b>() as
                                                libc::c_ulong));
    /* Clear the "num" array */
    memset(&mut num as *mut [s16b; 200] as *mut libc::c_char as
               *mut libc::c_void, 0 as libc::c_int,
           (200 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<s16b>() as
                                                libc::c_ulong));
    /* Size of "alloc_race_table" */
    alloc_race_size = 0 as libc::c_int as s16b;
    /* Scan the monsters */
    i = 1 as libc::c_int;
    while i < max_r_idx as libc::c_int {
        /* Get the i'th race */
        r_ptr = &mut *r_info.offset(i as isize) as *mut monster_race;
        /* Legal monsters */
        if (*r_ptr).rarity != 0 {
            /* Count the entries */
            alloc_race_size += 1;
            /* Group by level */
            num[(*r_ptr).level as usize] += 1
        }
        i += 1
    }
    /* Collect the level indexes */
    i = 1 as libc::c_int;
    while i < 200 as libc::c_int {
        /* Group by level */
        num[i as usize] =
            (num[i as usize] as libc::c_int +
                 num[(i - 1 as libc::c_int) as usize] as libc::c_int) as s16b;
        i += 1
    }
    /* Paranoia */
    if num[0 as libc::c_int as usize] == 0 {
        quit(b"No town monsters!\x00" as *const u8 as *const libc::c_char);
    }
    /* ** Initialise monster allocation info ***/
    /* Allocate the alloc_race_table */
    alloc_race_table =
        memset(ralloc((alloc_race_size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<alloc_entry>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (alloc_race_size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<alloc_entry>()
                                                    as libc::c_ulong)) as
            *mut alloc_entry;
    /* Access the table entry */
    table = alloc_race_table;
    /* Scan the monsters */
    i = 1 as libc::c_int;
    while i < max_r_idx as libc::c_int {
        /* Get the i'th race */
        r_ptr = &mut *r_info.offset(i as isize) as *mut monster_race;
        /* Count valid pairs */
        if (*r_ptr).rarity != 0 {
            let mut p_0: libc::c_int = 0;
            let mut x_0: libc::c_int = 0;
            let mut y_0: libc::c_int = 0;
            let mut z_0: libc::c_int = 0;
            /* Extract the base level */
            x_0 = (*r_ptr).level as libc::c_int;
            /* Extract the base probability */
            p_0 = 100 as libc::c_int / (*r_ptr).rarity as libc::c_int;
            /* Skip entries preceding our locale */
            y_0 =
                if x_0 > 0 as libc::c_int {
                    num[(x_0 - 1 as libc::c_int) as usize] as libc::c_int
                } else { 0 as libc::c_int };
            /* Skip previous entries at this locale */
            z_0 = y_0 + aux[x_0 as usize] as libc::c_int;
            /* Load the entry */
            (*table.offset(z_0 as isize)).index = i as s16b;
            (*table.offset(z_0 as isize)).level = x_0 as byte_hack;
            (*table.offset(z_0 as isize)).prob1 = p_0 as byte_hack;
            (*table.offset(z_0 as isize)).prob2 = p_0 as byte_hack;
            (*table.offset(z_0 as isize)).prob3 = p_0 as byte_hack;
            /* Another entry complete for this locale */
            aux[x_0 as usize] += 1
        }
        i += 1
    }
    /* Success */
    return 0 as libc::c_int;
}
/* Init the sets in a_info */
#[no_mangle]
pub unsafe extern "C" fn init_sets_aux() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < max_a_idx as libc::c_int {
        (*a_info.offset(i as isize)).set = -(1 as libc::c_int) as s16b;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < max_set_idx as libc::c_int {
        j = 0 as libc::c_int;
        while j < (*set_info.offset(i as isize)).num as libc::c_int {
            (*a_info.offset((*set_info.offset(i as
                                                  isize)).arts[j as
                                                                   usize].a_idx
                                as isize)).set = i as s16b;
            j += 1
        }
        i += 1
    };
}
/*
 * Mark guardians and their artifacts with SPECIAL_GENE flag
 */
unsafe extern "C" fn init_guardians() {
    let mut i: libc::c_int = 0;
    /* Scan dungeons */
    i = 0 as libc::c_int;
    while i < max_d_idx as libc::c_int {
        let mut d_ptr: *mut dungeon_info_type =
            &mut *d_info.offset(i as isize) as *mut dungeon_info_type;
        /* Mark the guadian monster */
        if (*d_ptr).final_guardian != 0 {
            let mut r_ptr: *mut monster_race =
                &mut *r_info.offset((*d_ptr).final_guardian as isize) as
                    *mut monster_race;
            (*r_ptr).flags9 |= 0x2000 as libc::c_int as libc::c_uint;
            /* Mark the final artifact */
            if (*d_ptr).final_artifact != 0 {
                let mut a_ptr: *mut artifact_type =
                    &mut *a_info.offset((*d_ptr).final_artifact as isize) as
                        *mut artifact_type;
                (*a_ptr).flags4 =
                    ((*a_ptr).flags4 as libc::c_long | 0x400 as libc::c_long)
                        as u32b
            }
            /* Mark the final object */
            if (*d_ptr).final_object != 0 {
                let mut k_ptr: *mut object_kind =
                    &mut *k_info.offset((*d_ptr).final_object as isize) as
                        *mut object_kind;
                (*k_ptr).flags4 =
                    ((*k_ptr).flags4 as libc::c_long | 0x400 as libc::c_long)
                        as u32b
            }
            /* Give randart if there are no final artifacts */
            if (*d_ptr).final_artifact == 0 && (*d_ptr).final_object == 0 {
                (*r_ptr).flags7 |= 0x10000 as libc::c_int as libc::c_uint
            }
        }
        i += 1
    };
}
/*
 * Hack -- Explain a broken "lib" folder and quit (see below).
 *
 * XXX XXX XXX This function is "messy" because various things
 * may or may not be initialised, but the "plog()" and "quit()"
 * functions are "supposed" to work under any conditions.
 */
unsafe extern "C" fn init_angband_aux(mut why: cptr) {
    /* Why */
    plog(why);
    /* Explain */
    plog(b"The \'lib\' directory is probably missing or broken.\x00" as
             *const u8 as *const libc::c_char);
    /* More details */
    plog(b"Perhaps the archive was not extracted correctly.\x00" as *const u8
             as *const libc::c_char);
    /* Explain */
    plog(b"See the \'README\' file for more information.\x00" as *const u8 as
             *const libc::c_char);
    /* Quit with error */
    quit(b"Fatal Error.\x00" as *const u8 as *const libc::c_char);
}
/*
 * Hack -- main Angband initialisation entry point
 *
 * Verify some files, display the "news.txt" file, create
 * the high score file, initialise all internal arrays, and
 * load the basic "user pref files".
 *
 * Note that we blindly assume that "news2.txt" exists. XXX
 *
 * Be very careful to keep track of the order in which things
 * are initialised, in particular, the only thing *known* to
 * be available when this function is called is the "z-term.c"
 * package, and that may not be fully initialised until the
 * end of this function, when the default "user pref files"
 * are loaded and "Term_xtra(TERM_XTRA_REACT,0)" is called.
 *
 * Note that this function attempts to verify the "news" file,
 * and the game aborts (cleanly) on failure, since without the
 * "news" file, it is likely that the "lib" folder has not been
 * correctly located.  Otherwise, the news file is displayed for
 * the user.
 *
 * Note that this function attempts to verify (or create) the
 * "high score" file, and the game aborts (cleanly) on failure,
 * since one of the most common "extraction" failures involves
 * failing to extract all sub-directories (even empty ones), such
 * as by failing to use the "-d" option of "pkunzip", or failing
 * to use the "save empty directories" option with "Compact Pro".
 * This error will often be caught by the "high score" creation
 * code below, since the "lib/apex" directory, being empty in the
 * standard distributions, is most likely to be "lost", making it
 * impossible to create the high score file.
 *
 * Note that various things are initialised by this function,
 * including everything that was once done by "init_some_arrays".
 *
 * This initialisation involves the parsing of special files
 * in the "lib/data" and sometimes the "lib/edit" directories.
 *
 * Note that the "template" files are initialised first, since they
 * often contain errors.  This means that macros and message recall
 * and things like that are not available until after they are done.
 *
 * We load the default "user pref files" here in case any "color"
 * changes are needed before character creation.
 *
 * Note that the "graf-xxx.prf" file must be loaded separately,
 * if needed, in the first (?) pass through "TERM_XTRA_REACT".
 */
#[no_mangle]
pub unsafe extern "C" fn init_angband() {
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut mode: libc::c_int = 0o644 as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut news_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Init some VERY basic stuff, like macro arrays */
    init_basic();
    /* Select & init a module if needed */
    select_module();
    /* ** Choose which news.txt file to use ***/
    /* Choose the news file */
    match time(0 as *mut time_t) % 2 as libc::c_int as libc::c_long {
        0 => {
            news_file =
                b"news2.txt\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        _ => {
            news_file =
                b"news.txt\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
    }
    /* ** Verify the "news" file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int, ANGBAND_DIR_FILE, news_file as cptr);
    /* Attempt to open the file */
    fd = fd_open(buf.as_mut_ptr() as cptr, 0 as libc::c_int);
    /* Failure */
    if fd < 0 as libc::c_int {
        let mut why: [libc::c_char; 1024] = [0; 1024];
        /* Message */
        sprintf(why.as_mut_ptr(),
                b"Cannot access the \'%s\' file!\x00" as *const u8 as
                    *const libc::c_char, buf.as_mut_ptr());
        /* Crash and burn */
        init_angband_aux(why.as_mut_ptr() as cptr);
    }
    /* Close it */
    fd_close(fd);
    /* ** Display the "news" file ***/
    /* Clear screen */
    Term_clear();
    /* Build the filename */
    path_build(buf.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int, ANGBAND_DIR_FILE, news_file as cptr);
    /* Open the News file */
    fp =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"r\x00" as *const u8 as *const libc::c_char);
    /* Dump */
    if !fp.is_null() {
        let mut i: libc::c_int = 0 as libc::c_int;
        /* Dump the file to the screen */
        while 0 as libc::c_int ==
                  my_fgets(fp, buf.as_mut_ptr(),
                           1024 as libc::c_int as huge_hack) {
            /* Display and advance - we use display_message to parse colour codes XXX */
            let fresh6 = i;
            i = i + 1;
            display_message(0 as libc::c_int, fresh6,
                            strlen(buf.as_mut_ptr()) as libc::c_int,
                            1 as libc::c_int as byte_hack,
                            buf.as_mut_ptr() as cptr);
        }
        /* Close */
        my_fclose(fp);
    }
    /* Flush it */
    Term_fresh();
    /* ** Verify (or create) the "high score" file ***/
    /* Build the filename */
    path_build(buf.as_mut_ptr(), 1024 as libc::c_int, ANGBAND_DIR_APEX,
               b"scores.raw\x00" as *const u8 as *const libc::c_char);
    /* Attempt to open the high score file */
    fd = fd_open(buf.as_mut_ptr() as cptr, 0 as libc::c_int);
    /* Failure */
    if fd < 0 as libc::c_int {
        /* File type is "DATA" */
        /* Create a new high score file */
        fd = fd_make(buf.as_mut_ptr() as cptr, mode);
        if fd < 0 as libc::c_int {
            let mut why_0: [libc::c_char; 1024] = [0; 1024];
            /* Message */
            sprintf(why_0.as_mut_ptr(),
                    b"Cannot create the \'%s\' file!\x00" as *const u8 as
                        *const libc::c_char, buf.as_mut_ptr());
            /* Crash and burn */
            init_angband_aux(why_0.as_mut_ptr() as cptr);
        }
    }
    /* Failure */
    /* Close it */
    fd_close(fd);
    /* ** Initialise some arrays ***/
    /* Initialise misc. values */
    note(b"[Initialising values... (misc)]\x00" as *const u8 as
             *const libc::c_char);
    if init_misc() != 0 {
        quit(b"Cannot initialise misc. values\x00" as *const u8 as
                 *const libc::c_char);
    }
    wipe_hooks();
    /* Initialise some other arrays */
    note(b"[Initialising lua scripting... (lua)]\x00" as *const u8 as
             *const libc::c_char);
    init_lua();
    init_lua_init();
    /* Initialise skills info */
    note(b"[Initialising arrays... (skills)]\x00" as *const u8 as
             *const libc::c_char);
    if init_s_info() != 0 {
        quit(b"Cannot initialise skills\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialise abilities info */
    note(b"[Initialising arrays... (abilities)]\x00" as *const u8 as
             *const libc::c_char);
    if init_ab_info() != 0 {
        quit(b"Cannot initialise abilities\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialise alchemy info */
    note(b"[Initialising arrays... (alchemy)]\x00" as *const u8 as
             *const libc::c_char);
    if init_al_info() != 0 {
        quit(b"Cannot initialise alchemy\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialise player info */
    note(b"[Initialising arrays... (players)]\x00" as *const u8 as
             *const libc::c_char);
    if init_player_info() != 0 {
        quit(b"Cannot initialise players\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialise feature info */
    note(b"[Initialising arrays... (features)]\x00" as *const u8 as
             *const libc::c_char);
    if init_f_info() != 0 {
        quit(b"Cannot initialise features\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialise object info */
    note(b"[Initialising arrays... (objects)]\x00" as *const u8 as
             *const libc::c_char);
    if init_k_info() != 0 {
        quit(b"Cannot initialise objects\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialise artifact info */
    note(b"[Initialising arrays... (artifacts)]\x00" as *const u8 as
             *const libc::c_char);
    if init_a_info() != 0 {
        quit(b"Cannot initialise artifacts\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialise set info */
    note(b"[Initialising item sets... (sets)]\x00" as *const u8 as
             *const libc::c_char);
    if init_set_info() != 0 {
        quit(b"Cannot initialise item sets\x00" as *const u8 as
                 *const libc::c_char);
    }
    init_sets_aux();
    /* Initialise ego-item info */
    note(b"[Initialising arrays... (ego-items)]\x00" as *const u8 as
             *const libc::c_char);
    if init_e_info() != 0 {
        quit(b"Cannot initialise ego-items\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialise randart parts info */
    note(b"[Initialising arrays... (randarts)]\x00" as *const u8 as
             *const libc::c_char);
    if init_ra_info() != 0 {
        quit(b"Cannot initialise randarts\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialise monster info */
    note(b"[Initialising arrays... (monsters)]\x00" as *const u8 as
             *const libc::c_char);
    if init_r_info() != 0 {
        quit(b"Cannot initialise monsters\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialise ego monster info */
    note(b"[Initialising arrays... (ego monsters)]\x00" as *const u8 as
             *const libc::c_char);
    if init_re_info() != 0 {
        quit(b"Cannot initialise ego monsters\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialise dungeon type info */
    note(b"[Initialising arrays... (dungeon types)]\x00" as *const u8 as
             *const libc::c_char);
    if init_d_info() != 0 {
        quit(b"Cannot initialise dungeon types\x00" as *const u8 as
                 *const libc::c_char);
    }
    init_guardians();
    /* Initialise actions type info */
    note(b"[Initialising arrays... (action types)]\x00" as *const u8 as
             *const libc::c_char);
    if init_ba_info() != 0 {
        quit(b"Cannot initialise action types\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialise owners type info */
    note(b"[Initialising arrays... (owners types)]\x00" as *const u8 as
             *const libc::c_char);
    if init_ow_info() != 0 {
        quit(b"Cannot initialise owners types\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialise stores type info */
    note(b"[Initialising arrays... (stores types)]\x00" as *const u8 as
             *const libc::c_char);
    if init_st_info() != 0 {
        quit(b"Cannot initialise stores types\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialise wilderness features array */
    note(b"[Initialising arrays... (wilderness features)]\x00" as *const u8 as
             *const libc::c_char);
    if init_wf_info() != 0 {
        quit(b"Cannot initialise wilderness features\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialise wilderness map array */
    note(b"[Initialising arrays... (wilderness map)]\x00" as *const u8 as
             *const libc::c_char);
    if init_wilderness() != 0 {
        quit(b"Cannot initialise wilderness map\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialise town array */
    note(b"[Initialising arrays... (towns)]\x00" as *const u8 as
             *const libc::c_char);
    if init_towns() != 0 {
        quit(b"Cannot initialise towns\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialise trap info */
    note(b"[Initialising arrays... (traps)]\x00" as *const u8 as
             *const libc::c_char);
    if init_t_info() != 0 {
        quit(b"Cannot initialise traps\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialise some other arrays */
    note(b"[Initialising arrays... (other)]\x00" as *const u8 as
             *const libc::c_char);
    if init_other() != 0 {
        quit(b"Cannot initialise other stuff\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Initialise some other arrays */
    note(b"[Initialising arrays... (alloc)]\x00" as *const u8 as
             *const libc::c_char);
    if init_alloc() != 0 {
        quit(b"Cannot initialise alloc stuff\x00" as *const u8 as
                 *const libc::c_char);
    }
    /* Init random artifact names */
    build_prob(artifact_names_list);
    /* ** Load default user pref files ***/
    /* Initialise feature info */
    note(b"[Initialising user pref files...]\x00" as *const u8 as
             *const libc::c_char);
    /* Access the "basic" pref file */
    strcpy(buf.as_mut_ptr(),
           b"pref.prf\x00" as *const u8 as *const libc::c_char);
    /* Process that file */
    process_pref_file(buf.as_mut_ptr() as cptr);
    /* Access the "basic" system pref file */
    sprintf(buf.as_mut_ptr(),
            b"pref-%s.prf\x00" as *const u8 as *const libc::c_char,
            ANGBAND_SYS);
    /* Process that file */
    process_pref_file(buf.as_mut_ptr() as cptr);
    /* Access the "user" pref file */
    sprintf(buf.as_mut_ptr(),
            b"user.prf\x00" as *const u8 as *const libc::c_char);
    /* Process that file */
    process_pref_file(buf.as_mut_ptr() as cptr);
    /* Access the "user" system pref file */
    sprintf(buf.as_mut_ptr(),
            b"user-%s.prf\x00" as *const u8 as *const libc::c_char,
            ANGBAND_SYS);
    /* Process that file */
    process_pref_file(buf.as_mut_ptr() as cptr);
    /* Initialise the automatizer */
    tome_dofile_anywhere(ANGBAND_DIR_CORE,
                         b"auto.lua\x00" as *const u8 as *const libc::c_char
                             as *mut libc::c_char, 1 as libc::c_int as bool_);
    tome_dofile_anywhere(ANGBAND_DIR_USER,
                         b"automat.atm\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         0 as libc::c_int as bool_);
    /* Done */
    note(b"[Initialisation complete]\x00" as *const u8 as
             *const libc::c_char);
    process_hooks(37 as libc::c_int,
                  b"(s)\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char,
                  b"end\x00" as *const u8 as *const libc::c_char);
}
