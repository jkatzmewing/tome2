use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut adj_str_hold: [byte_hack; 0];
    #[no_mangle]
    static mut player_exp: [s32b; 50];
    #[no_mangle]
    static mut sense_desc: [cptr; 0];
    #[no_mangle]
    static mut flags_groups: [flags_group; 12];
    #[no_mangle]
    static mut tval_descs: [tval_desc; 0];
    #[no_mangle]
    static mut gf_names: [gf_name_type; 0];
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut seed_flavor: u32b;
    #[no_mangle]
    static mut command_cmd: s16b;
    #[no_mangle]
    static mut command_wrk: s16b;
    #[no_mangle]
    static mut cur_hgt: s16b;
    #[no_mangle]
    static mut cur_wid: s16b;
    #[no_mangle]
    static mut use_graphics: bool_;
    #[no_mangle]
    static mut graphics_mode: byte_hack;
    #[no_mangle]
    static mut equip_cnt: s16b;
    #[no_mangle]
    static mut text_out_file: *mut FILE;
    #[no_mangle]
    static mut text_out_hook:
           Option<unsafe extern "C" fn(_: byte_hack, _: cptr) -> ()>;
    #[no_mangle]
    static mut show_inven_graph: bool_;
    #[no_mangle]
    static mut show_equip_graph: bool_;
    #[no_mangle]
    static mut carry_query_flag: bool_;
    #[no_mangle]
    static mut prompt_pickup_heavy: bool_;
    #[no_mangle]
    static mut plain_descriptions: bool_;
    #[no_mangle]
    static mut show_labels: bool_;
    #[no_mangle]
    static mut show_weights: bool_;
    #[no_mangle]
    static mut show_choices: bool_;
    #[no_mangle]
    static mut window_flag: [u32b; 8];
    #[no_mangle]
    static mut angband_term: [*mut term; 8];
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut o_list: *mut object_type;
    #[no_mangle]
    static mut misc_to_attr: [byte_hack; 256];
    #[no_mangle]
    static mut misc_to_char: [libc::c_char; 256];
    #[no_mangle]
    static mut tval_to_attr: [byte_hack; 128];
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut f_info: *mut feature_type;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut k_name: *mut libc::c_char;
    #[no_mangle]
    static mut k_text: *mut libc::c_char;
    #[no_mangle]
    static mut a_info: *mut artifact_type;
    #[no_mangle]
    static mut a_name: *mut libc::c_char;
    #[no_mangle]
    static mut a_text: *mut libc::c_char;
    #[no_mangle]
    static mut e_info: *mut ego_item_type;
    #[no_mangle]
    static mut e_name: *mut libc::c_char;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut r_name: *mut libc::c_char;
    #[no_mangle]
    static mut re_info: *mut monster_ego;
    #[no_mangle]
    static mut d_info: *mut dungeon_info_type;
    #[no_mangle]
    static mut d_name: *mut libc::c_char;
    #[no_mangle]
    static mut race_mod_info: *mut player_race_mod;
    #[no_mangle]
    static mut t_info: *mut trap_type;
    #[no_mangle]
    static mut t_name: *mut libc::c_char;
    #[no_mangle]
    static mut wf_info: *mut wilderness_type_info;
    #[no_mangle]
    static mut wf_name: *mut libc::c_char;
    #[no_mangle]
    static mut wf_text: *mut libc::c_char;
    #[no_mangle]
    static mut st_info: *mut store_info_type;
    #[no_mangle]
    static mut st_name: *mut libc::c_char;
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
    static mut item_tester_full: bool_;
    #[no_mangle]
    static mut item_tester_tval: byte_hack;
    #[no_mangle]
    static mut item_tester_hook:
           Option<unsafe extern "C" fn(_: *mut object_type) -> bool_>;
    #[no_mangle]
    static mut max_r_idx: u16b;
    #[no_mangle]
    static mut max_re_idx: u16b;
    #[no_mangle]
    static mut max_k_idx: u16b;
    #[no_mangle]
    static mut max_f_idx: u16b;
    #[no_mangle]
    static mut max_rmp_idx: u16b;
    #[no_mangle]
    static mut max_st_idx: u16b;
    #[no_mangle]
    static mut random_artifacts: [random_artifact; 84];
    #[no_mangle]
    static mut inventory_no_move: bool_;
    #[no_mangle]
    static mut munchkin_multipliers: bool_;
    #[no_mangle]
    static mut exp_need: bool_;
    #[no_mangle]
    static mut powers_type: *mut power_type;
    #[no_mangle]
    static mut school_spells: *mut spell_type;
    #[no_mangle]
    static mut process_hooks_return: [hook_return; 20];
    #[no_mangle]
    fn process_hooks_ret(h_idx: libc::c_int, ret: *mut libc::c_char,
                         fmt: *mut libc::c_char, _: ...) -> bool_;
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn object_track(o_ptr: *mut object_type);
    #[no_mangle]
    fn disturb(stop_search: libc::c_int, flush_output: libc::c_int);
    #[no_mangle]
    fn breakage_chance(o_ptr: *mut object_type) -> libc::c_int;
    #[no_mangle]
    fn get_shooter_mult(o_ptr: *mut object_type) -> libc::c_int;
    #[no_mangle]
    fn set_stick_mode(o_ptr: *mut object_type);
    #[no_mangle]
    fn unset_stick_mode();
    #[no_mangle]
    fn activation_aux(o_ptr: *mut object_type, desc: bool_, item: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn sense_inventory();
    #[no_mangle]
    fn process_pref_file(name: cptr) -> errr;
    #[no_mangle]
    fn get_rnd_line(file_name: *mut libc::c_char, output: *mut libc::c_char)
     -> errr;
    #[no_mangle]
    fn monster_description_out(r_idx: libc::c_int, ego: libc::c_int);
    #[no_mangle]
    fn monster_desc(desc: *mut libc::c_char, m_ptr: *mut monster_type,
                    mode: libc::c_int);
    #[no_mangle]
    fn monster_race_desc(desc: *mut libc::c_char, r_idx: libc::c_int,
                         ego: libc::c_int);
    #[no_mangle]
    fn get_object(item: libc::c_int) -> *mut object_type;
    #[no_mangle]
    fn calc_total_weight() -> s32b;
    #[no_mangle]
    fn inven_carry_okay(o_ptr: *mut object_type) -> bool_;
    #[no_mangle]
    fn inven_carry(o_ptr: *mut object_type, final_0: bool_) -> s16b;
    #[no_mangle]
    fn delete_object_idx(o_idx: libc::c_int);
    #[no_mangle]
    fn object_known(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_aware(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_similar(o_ptr: *mut object_type, j_ptr: *mut object_type)
     -> bool_;
    #[no_mangle]
    fn lookup_kind(tval: libc::c_int, sval: libc::c_int) -> s16b;
    #[no_mangle]
    fn object_copy(o_ptr: *mut object_type, j_ptr: *mut object_type);
    #[no_mangle]
    fn bell();
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn quark_str(num: s16b) -> cptr;
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn cmsg_format(color: byte_hack, fmt: cptr, _: ...);
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn screen_save();
    #[no_mangle]
    fn screen_load();
    #[no_mangle]
    fn c_put_str(attr: byte_hack, str: cptr, row: libc::c_int,
                 col: libc::c_int);
    #[no_mangle]
    fn put_str(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn toupper(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn text_out(str: cptr);
    #[no_mangle]
    fn string_exec_lua(file: *mut libc::c_char) -> cptr;
    #[no_mangle]
    fn text_out_c(a: byte_hack, str: cptr);
    #[no_mangle]
    fn exec_lua(file: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn is_a_vowel(ch: libc::c_int) -> bool_;
    #[no_mangle]
    fn call_lua(function: cptr, args: cptr, ret: cptr, _: ...) -> bool_;
    #[no_mangle]
    fn apply_flags(f1: u32b, f2: u32b, f3: u32b, f4: u32b, f5: u32b,
                   esp: u32b, pval: s16b, tval: s16b, to_h: s16b, to_d: s16b,
                   to_a: s16b);
    #[no_mangle]
    fn get_check(prompt: cptr) -> bool_;
    #[no_mangle]
    fn text_out_to_screen(a: byte_hack, str: cptr);
    #[no_mangle]
    fn calc_bonuses(silent: bool_);
    #[no_mangle]
    fn count_bits(array: u32b) -> byte_hack;
    #[no_mangle]
    fn text_out_to_file(a: byte_hack, str: cptr);
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn get_melee_skill() -> s16b;
    #[no_mangle]
    static mut melee_names: [*mut libc::c_char; 3];
    #[no_mangle]
    fn get_skill(skill: libc::c_int) -> s16b;
    #[no_mangle]
    fn repeat_push(what: libc::c_int);
    #[no_mangle]
    fn window_stuff();
    #[no_mangle]
    static mut automatizer_create: bool_;
    #[no_mangle]
    fn repeat_pull(what: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn Term_load() -> errr;
    #[no_mangle]
    fn Term_save() -> errr;
    #[no_mangle]
    fn Term_get_size(w: *mut libc::c_int, h: *mut libc::c_int) -> errr;
    #[no_mangle]
    fn Term_erase(x: libc::c_int, y: libc::c_int, n: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_draw(x: libc::c_int, y: libc::c_int, a: byte_hack,
                 c: libc::c_char) -> errr;
    #[no_mangle]
    fn weight_limit() -> libc::c_int;
    #[no_mangle]
    fn squeltch_grid();
    #[no_mangle]
    static mut Term: *mut term;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    static mut Rand_value: u32b;
    #[no_mangle]
    static mut Rand_quick: bool_;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn streq(s: cptr, t: cptr) -> bool_;
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
pub type s16b = libc::c_short;
pub type u16b = libc::c_ushort;
pub type s32b = libc::c_int;
pub type u32b = libc::c_uint;
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
pub struct random_artifact {
    pub name_full: [libc::c_char; 80],
    pub name_short: [libc::c_char; 80],
    pub level: byte_hack,
    pub attr: byte_hack,
    pub cost: u32b,
    pub activation: byte_hack,
    pub timeout: s16b,
    pub generated: byte_hack,
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
pub struct flags_group {
    pub name: [libc::c_char; 30],
    pub color: byte_hack,
    pub price: byte_hack,
    pub flags1: u32b,
    pub flags2: u32b,
    pub flags3: u32b,
    pub flags4: u32b,
    pub esp: u32b,
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
pub struct tval_desc {
    pub tval: libc::c_int,
    pub desc: cptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union hook_return {
    pub num: s32b,
    pub str_0: cptr,
    pub o_ptr: *mut object_type,
    pub m_ptr: *mut monster_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct set_type {
    pub name: u32b,
    pub desc: u32b,
    pub num: byte_hack,
    pub num_use: byte_hack,
    pub arts: [C2RustUnnamed_1; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
pub struct gf_name_type {
    pub gf: libc::c_int,
    pub name: cptr,
}
/* Used with scrolls (min 55) */
/* Used with scrolls (see below) */
/*
 * Rings (adjectives and colors)
 */
static mut ring_adj: [cptr; 62] =
    [b"Alexandrite\x00" as *const u8 as *const libc::c_char,
     b"Amethyst\x00" as *const u8 as *const libc::c_char,
     b"Aquamarine\x00" as *const u8 as *const libc::c_char,
     b"Azurite\x00" as *const u8 as *const libc::c_char,
     b"Beryl\x00" as *const u8 as *const libc::c_char,
     b"Bloodstone\x00" as *const u8 as *const libc::c_char,
     b"Calcite\x00" as *const u8 as *const libc::c_char,
     b"Carnelian\x00" as *const u8 as *const libc::c_char,
     b"Corundum\x00" as *const u8 as *const libc::c_char,
     b"Diamond\x00" as *const u8 as *const libc::c_char,
     b"Emerald\x00" as *const u8 as *const libc::c_char,
     b"Fluorite\x00" as *const u8 as *const libc::c_char,
     b"Garnet\x00" as *const u8 as *const libc::c_char,
     b"Granite\x00" as *const u8 as *const libc::c_char,
     b"Jade\x00" as *const u8 as *const libc::c_char,
     b"Jasper\x00" as *const u8 as *const libc::c_char,
     b"Lapis Lazuli\x00" as *const u8 as *const libc::c_char,
     b"Malachite\x00" as *const u8 as *const libc::c_char,
     b"Marble\x00" as *const u8 as *const libc::c_char,
     b"Moonstone\x00" as *const u8 as *const libc::c_char,
     b"Onyx\x00" as *const u8 as *const libc::c_char,
     b"Opal\x00" as *const u8 as *const libc::c_char,
     b"Pearl\x00" as *const u8 as *const libc::c_char,
     b"Quartz\x00" as *const u8 as *const libc::c_char,
     b"Quartzite\x00" as *const u8 as *const libc::c_char,
     b"Rhodonite\x00" as *const u8 as *const libc::c_char,
     b"Ruby\x00" as *const u8 as *const libc::c_char,
     b"Sapphire\x00" as *const u8 as *const libc::c_char,
     b"Tiger Eye\x00" as *const u8 as *const libc::c_char,
     b"Topaz\x00" as *const u8 as *const libc::c_char,
     b"Turquoise\x00" as *const u8 as *const libc::c_char,
     b"Zircon\x00" as *const u8 as *const libc::c_char,
     b"Platinum\x00" as *const u8 as *const libc::c_char,
     b"Bronze\x00" as *const u8 as *const libc::c_char,
     b"Gold\x00" as *const u8 as *const libc::c_char,
     b"Obsidian\x00" as *const u8 as *const libc::c_char,
     b"Silver\x00" as *const u8 as *const libc::c_char,
     b"Tortoise Shell\x00" as *const u8 as *const libc::c_char,
     b"Mithril\x00" as *const u8 as *const libc::c_char,
     b"Jet\x00" as *const u8 as *const libc::c_char,
     b"Engagement\x00" as *const u8 as *const libc::c_char,
     b"Adamantite\x00" as *const u8 as *const libc::c_char,
     b"Wire\x00" as *const u8 as *const libc::c_char,
     b"Dilithium\x00" as *const u8 as *const libc::c_char,
     b"Bone\x00" as *const u8 as *const libc::c_char,
     b"Wooden\x00" as *const u8 as *const libc::c_char,
     b"Spikard\x00" as *const u8 as *const libc::c_char,
     b"Serpent\x00" as *const u8 as *const libc::c_char,
     b"Wedding\x00" as *const u8 as *const libc::c_char,
     b"Double\x00" as *const u8 as *const libc::c_char,
     b"Plain\x00" as *const u8 as *const libc::c_char,
     b"Brass\x00" as *const u8 as *const libc::c_char,
     b"Scarab\x00" as *const u8 as *const libc::c_char,
     b"Shining\x00" as *const u8 as *const libc::c_char,
     b"Rusty\x00" as *const u8 as *const libc::c_char,
     b"Transparent\x00" as *const u8 as *const libc::c_char,
     b"Copper\x00" as *const u8 as *const libc::c_char,
     b"Black Opal\x00" as *const u8 as *const libc::c_char,
     b"Nickel\x00" as *const u8 as *const libc::c_char,
     b"Glass\x00" as *const u8 as *const libc::c_char,
     b"Fluorspar\x00" as *const u8 as *const libc::c_char,
     b"Agate\x00" as *const u8 as *const libc::c_char];
static mut ring_col: [byte_hack; 62] =
    [5 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack,
     14 as libc::c_int as byte_hack, 14 as libc::c_int as byte_hack,
     13 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 13 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     13 as libc::c_int as byte_hack, 7 as libc::c_int as byte_hack,
     6 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     12 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 12 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
     11 as libc::c_int as byte_hack, 11 as libc::c_int as byte_hack,
     14 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     11 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     14 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     11 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 7 as libc::c_int as byte_hack,
     6 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     11 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     11 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     13 as libc::c_int as byte_hack, 11 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     6 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack];
/*
 * Amulets (adjectives and colors)
 */
static mut amulet_adj: [cptr; 34] =
    [b"Amber\x00" as *const u8 as *const libc::c_char,
     b"Driftwood\x00" as *const u8 as *const libc::c_char,
     b"Coral\x00" as *const u8 as *const libc::c_char,
     b"Agate\x00" as *const u8 as *const libc::c_char,
     b"Ivory\x00" as *const u8 as *const libc::c_char,
     b"Obsidian\x00" as *const u8 as *const libc::c_char,
     b"Bone\x00" as *const u8 as *const libc::c_char,
     b"Brass\x00" as *const u8 as *const libc::c_char,
     b"Bronze\x00" as *const u8 as *const libc::c_char,
     b"Pewter\x00" as *const u8 as *const libc::c_char,
     b"Tortoise Shell\x00" as *const u8 as *const libc::c_char,
     b"Golden\x00" as *const u8 as *const libc::c_char,
     b"Azure\x00" as *const u8 as *const libc::c_char,
     b"Crystal\x00" as *const u8 as *const libc::c_char,
     b"Silver\x00" as *const u8 as *const libc::c_char,
     b"Copper\x00" as *const u8 as *const libc::c_char,
     b"Amethyst\x00" as *const u8 as *const libc::c_char,
     b"Mithril\x00" as *const u8 as *const libc::c_char,
     b"Sapphire\x00" as *const u8 as *const libc::c_char,
     b"Dragon Tooth\x00" as *const u8 as *const libc::c_char,
     b"Carved Oak\x00" as *const u8 as *const libc::c_char,
     b"Sea Shell\x00" as *const u8 as *const libc::c_char,
     b"Flint Stone\x00" as *const u8 as *const libc::c_char,
     b"Ruby\x00" as *const u8 as *const libc::c_char,
     b"Scarab\x00" as *const u8 as *const libc::c_char,
     b"Origami Paper\x00" as *const u8 as *const libc::c_char,
     b"Meteoric Iron\x00" as *const u8 as *const libc::c_char,
     b"Platinum\x00" as *const u8 as *const libc::c_char,
     b"Glass\x00" as *const u8 as *const libc::c_char,
     b"Beryl\x00" as *const u8 as *const libc::c_char,
     b"Malachite\x00" as *const u8 as *const libc::c_char,
     b"Adamantite\x00" as *const u8 as *const libc::c_char,
     b"Mother-of-pearl\x00" as *const u8 as *const libc::c_char,
     b"Runed\x00" as *const u8 as *const libc::c_char];
static mut amulet_col: [byte_hack; 34] =
    [11 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 11 as libc::c_int as byte_hack,
     14 as libc::c_int as byte_hack, 14 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     10 as libc::c_int as byte_hack, 14 as libc::c_int as byte_hack,
     6 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 14 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     13 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     8 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 13 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 7 as libc::c_int as byte_hack];
/*
 * Staffs (adjectives and colors)
 */
static mut staff_adj: [cptr; 35] =
    [b"Aspen\x00" as *const u8 as *const libc::c_char,
     b"Balsa\x00" as *const u8 as *const libc::c_char,
     b"Banyan\x00" as *const u8 as *const libc::c_char,
     b"Birch\x00" as *const u8 as *const libc::c_char,
     b"Cedar\x00" as *const u8 as *const libc::c_char,
     b"Cottonwood\x00" as *const u8 as *const libc::c_char,
     b"Cypress\x00" as *const u8 as *const libc::c_char,
     b"Dogwood\x00" as *const u8 as *const libc::c_char,
     b"Elm\x00" as *const u8 as *const libc::c_char,
     b"Eucalyptus\x00" as *const u8 as *const libc::c_char,
     b"Hemlock\x00" as *const u8 as *const libc::c_char,
     b"Hickory\x00" as *const u8 as *const libc::c_char,
     b"Ironwood\x00" as *const u8 as *const libc::c_char,
     b"Locust\x00" as *const u8 as *const libc::c_char,
     b"Mahogany\x00" as *const u8 as *const libc::c_char,
     b"Maple\x00" as *const u8 as *const libc::c_char,
     b"Mulberry\x00" as *const u8 as *const libc::c_char,
     b"Oak\x00" as *const u8 as *const libc::c_char,
     b"Pine\x00" as *const u8 as *const libc::c_char,
     b"Redwood\x00" as *const u8 as *const libc::c_char,
     b"Rosewood\x00" as *const u8 as *const libc::c_char,
     b"Spruce\x00" as *const u8 as *const libc::c_char,
     b"Sycamore\x00" as *const u8 as *const libc::c_char,
     b"Teak\x00" as *const u8 as *const libc::c_char,
     b"Walnut\x00" as *const u8 as *const libc::c_char,
     b"Mistletoe\x00" as *const u8 as *const libc::c_char,
     b"Hawthorn\x00" as *const u8 as *const libc::c_char,
     b"Bamboo\x00" as *const u8 as *const libc::c_char,
     b"Silver\x00" as *const u8 as *const libc::c_char,
     b"Runed\x00" as *const u8 as *const libc::c_char,
     b"Golden\x00" as *const u8 as *const libc::c_char,
     b"Ashen\x00" as *const u8 as *const libc::c_char,
     b"Gnarled\x00" as *const u8 as *const libc::c_char,
     b"Ivory\x00" as *const u8 as *const libc::c_char,
     b"Willow\x00" as *const u8 as *const libc::c_char];
static mut staff_col: [byte_hack; 35] =
    [15 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 7 as libc::c_int as byte_hack,
     11 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack];
/*
 * Wands (adjectives and colors)
 */
static mut wand_adj: [cptr; 39] =
    [b"Aluminium\x00" as *const u8 as *const libc::c_char,
     b"Cast Iron\x00" as *const u8 as *const libc::c_char,
     b"Chromium\x00" as *const u8 as *const libc::c_char,
     b"Copper\x00" as *const u8 as *const libc::c_char,
     b"Gold\x00" as *const u8 as *const libc::c_char,
     b"Iron\x00" as *const u8 as *const libc::c_char,
     b"Magnesium\x00" as *const u8 as *const libc::c_char,
     b"Molybdenum\x00" as *const u8 as *const libc::c_char,
     b"Nickel\x00" as *const u8 as *const libc::c_char,
     b"Rusty\x00" as *const u8 as *const libc::c_char,
     b"Silver\x00" as *const u8 as *const libc::c_char,
     b"Steel\x00" as *const u8 as *const libc::c_char,
     b"Tin\x00" as *const u8 as *const libc::c_char,
     b"Titanium\x00" as *const u8 as *const libc::c_char,
     b"Tungsten\x00" as *const u8 as *const libc::c_char,
     b"Zirconium\x00" as *const u8 as *const libc::c_char,
     b"Zinc\x00" as *const u8 as *const libc::c_char,
     b"Aluminium-Plated\x00" as *const u8 as *const libc::c_char,
     b"Copper-Plated\x00" as *const u8 as *const libc::c_char,
     b"Gold-Plated\x00" as *const u8 as *const libc::c_char,
     b"Nickel-Plated\x00" as *const u8 as *const libc::c_char,
     b"Silver-Plated\x00" as *const u8 as *const libc::c_char,
     b"Steel-Plated\x00" as *const u8 as *const libc::c_char,
     b"Tin-Plated\x00" as *const u8 as *const libc::c_char,
     b"Zinc-Plated\x00" as *const u8 as *const libc::c_char,
     b"Mithril-Plated\x00" as *const u8 as *const libc::c_char,
     b"Mithril\x00" as *const u8 as *const libc::c_char,
     b"Runed\x00" as *const u8 as *const libc::c_char,
     b"Bronze\x00" as *const u8 as *const libc::c_char,
     b"Brass\x00" as *const u8 as *const libc::c_char,
     b"Platinum\x00" as *const u8 as *const libc::c_char,
     b"Lead\x00" as *const u8 as *const libc::c_char,
     b"Lead-Plated\x00" as *const u8 as *const libc::c_char,
     b"Ivory\x00" as *const u8 as *const libc::c_char,
     b"Adamantite\x00" as *const u8 as *const libc::c_char,
     b"Uridium\x00" as *const u8 as *const libc::c_char,
     b"Long\x00" as *const u8 as *const libc::c_char,
     b"Short\x00" as *const u8 as *const libc::c_char,
     b"Hexagonal\x00" as *const u8 as *const libc::c_char];
static mut wand_col: [byte_hack; 39] =
    [14 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 7 as libc::c_int as byte_hack,
     11 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 14 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 11 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 14 as libc::c_int as byte_hack,
     14 as libc::c_int as byte_hack, 7 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     10 as libc::c_int as byte_hack, 12 as libc::c_int as byte_hack,
     14 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack];
/*
 * Rods (adjectives and colors).
 * Efficiency -- copied from wand arrays
 */
static mut rod_adj: [cptr; 39] = [0 as *const libc::c_char; 39];
static mut rod_col: [byte_hack; 39] = [0; 39];
/*
 * Mushrooms (adjectives and colors)
 */
static mut food_adj: [cptr; 20] =
    [b"Blue\x00" as *const u8 as *const libc::c_char,
     b"Black\x00" as *const u8 as *const libc::c_char,
     b"Black Spotted\x00" as *const u8 as *const libc::c_char,
     b"Brown\x00" as *const u8 as *const libc::c_char,
     b"Dark Blue\x00" as *const u8 as *const libc::c_char,
     b"Dark Green\x00" as *const u8 as *const libc::c_char,
     b"Dark Red\x00" as *const u8 as *const libc::c_char,
     b"Yellow\x00" as *const u8 as *const libc::c_char,
     b"Furry\x00" as *const u8 as *const libc::c_char,
     b"Green\x00" as *const u8 as *const libc::c_char,
     b"Grey\x00" as *const u8 as *const libc::c_char,
     b"Light Blue\x00" as *const u8 as *const libc::c_char,
     b"Light Green\x00" as *const u8 as *const libc::c_char,
     b"Violet\x00" as *const u8 as *const libc::c_char,
     b"Red\x00" as *const u8 as *const libc::c_char,
     b"Slimy\x00" as *const u8 as *const libc::c_char,
     b"Tan\x00" as *const u8 as *const libc::c_char,
     b"White\x00" as *const u8 as *const libc::c_char,
     b"White Spotted\x00" as *const u8 as *const libc::c_char,
     b"Wrinkled\x00" as *const u8 as *const libc::c_char];
static mut food_col: [byte_hack; 20] =
    [6 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     8 as libc::c_int as byte_hack, 7 as libc::c_int as byte_hack,
     6 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 11 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 14 as libc::c_int as byte_hack,
     13 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 7 as libc::c_int as byte_hack];
/*
 * Color adjectives and colors, for potions.
 * Hack -- The first four entries are hard-coded.
 * (water, apple juice, slime mold juice, something)
 */
static mut potion_adj: [cptr; 66] =
    [b"Clear\x00" as *const u8 as *const libc::c_char,
     b"Light Brown\x00" as *const u8 as *const libc::c_char,
     b"Icky Green\x00" as *const u8 as *const libc::c_char,
     b"Strangely Phosphorescent\x00" as *const u8 as *const libc::c_char,
     b"Azure\x00" as *const u8 as *const libc::c_char,
     b"Blue\x00" as *const u8 as *const libc::c_char,
     b"Blue Speckled\x00" as *const u8 as *const libc::c_char,
     b"Black\x00" as *const u8 as *const libc::c_char,
     b"Brown\x00" as *const u8 as *const libc::c_char,
     b"Brown Speckled\x00" as *const u8 as *const libc::c_char,
     b"Bubbling\x00" as *const u8 as *const libc::c_char,
     b"Chartreuse\x00" as *const u8 as *const libc::c_char,
     b"Cloudy\x00" as *const u8 as *const libc::c_char,
     b"Copper Speckled\x00" as *const u8 as *const libc::c_char,
     b"Crimson\x00" as *const u8 as *const libc::c_char,
     b"Cyan\x00" as *const u8 as *const libc::c_char,
     b"Dark Blue\x00" as *const u8 as *const libc::c_char,
     b"Dark Green\x00" as *const u8 as *const libc::c_char,
     b"Dark Red\x00" as *const u8 as *const libc::c_char,
     b"Gold Speckled\x00" as *const u8 as *const libc::c_char,
     b"Green\x00" as *const u8 as *const libc::c_char,
     b"Green Speckled\x00" as *const u8 as *const libc::c_char,
     b"Grey\x00" as *const u8 as *const libc::c_char,
     b"Grey Speckled\x00" as *const u8 as *const libc::c_char,
     b"Hazy\x00" as *const u8 as *const libc::c_char,
     b"Indigo\x00" as *const u8 as *const libc::c_char,
     b"Light Blue\x00" as *const u8 as *const libc::c_char,
     b"Light Green\x00" as *const u8 as *const libc::c_char,
     b"Magenta\x00" as *const u8 as *const libc::c_char,
     b"Metallic Blue\x00" as *const u8 as *const libc::c_char,
     b"Metallic Red\x00" as *const u8 as *const libc::c_char,
     b"Metallic Green\x00" as *const u8 as *const libc::c_char,
     b"Metallic Purple\x00" as *const u8 as *const libc::c_char,
     b"Misty\x00" as *const u8 as *const libc::c_char,
     b"Orange\x00" as *const u8 as *const libc::c_char,
     b"Orange Speckled\x00" as *const u8 as *const libc::c_char,
     b"Pink\x00" as *const u8 as *const libc::c_char,
     b"Pink Speckled\x00" as *const u8 as *const libc::c_char,
     b"Puce\x00" as *const u8 as *const libc::c_char,
     b"Purple\x00" as *const u8 as *const libc::c_char,
     b"Purple Speckled\x00" as *const u8 as *const libc::c_char,
     b"Red\x00" as *const u8 as *const libc::c_char,
     b"Red Speckled\x00" as *const u8 as *const libc::c_char,
     b"Silver Speckled\x00" as *const u8 as *const libc::c_char,
     b"Smoky\x00" as *const u8 as *const libc::c_char,
     b"Tangerine\x00" as *const u8 as *const libc::c_char,
     b"Violet\x00" as *const u8 as *const libc::c_char,
     b"Vermilion\x00" as *const u8 as *const libc::c_char,
     b"White\x00" as *const u8 as *const libc::c_char,
     b"Yellow\x00" as *const u8 as *const libc::c_char,
     b"Violet Speckled\x00" as *const u8 as *const libc::c_char,
     b"Pungent\x00" as *const u8 as *const libc::c_char,
     b"Clotted Red\x00" as *const u8 as *const libc::c_char,
     b"Viscous Pink\x00" as *const u8 as *const libc::c_char,
     b"Oily Yellow\x00" as *const u8 as *const libc::c_char,
     b"Gloopy Green\x00" as *const u8 as *const libc::c_char,
     b"Shimmering\x00" as *const u8 as *const libc::c_char,
     b"Coagulated Crimson\x00" as *const u8 as *const libc::c_char,
     b"Yellow Speckled\x00" as *const u8 as *const libc::c_char,
     b"Gold\x00" as *const u8 as *const libc::c_char,
     b"Manly\x00" as *const u8 as *const libc::c_char,
     b"Stinking\x00" as *const u8 as *const libc::c_char,
     b"Oily Black\x00" as *const u8 as *const libc::c_char,
     b"Ichor\x00" as *const u8 as *const libc::c_char,
     b"Ivory White\x00" as *const u8 as *const libc::c_char,
     b"Sky Blue\x00" as *const u8 as *const libc::c_char];
static mut potion_col: [byte_hack; 66] =
    [1 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack,
     14 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
     6 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 7 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 13 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 14 as libc::c_int as byte_hack,
     6 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 11 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack,
     14 as libc::c_int as byte_hack, 13 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     10 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     12 as libc::c_int as byte_hack, 12 as libc::c_int as byte_hack,
     10 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack,
     10 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     8 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     10 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 11 as libc::c_int as byte_hack,
     10 as libc::c_int as byte_hack, 12 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 12 as libc::c_int as byte_hack,
     11 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     10 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     11 as libc::c_int as byte_hack, 11 as libc::c_int as byte_hack,
     15 as libc::c_int as byte_hack, 7 as libc::c_int as byte_hack,
     8 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 14 as libc::c_int as byte_hack];
/*
 * Syllables for scrolls (must be 1-4 letters each)
 */
static mut syllables: [cptr; 164] =
    [b"a\x00" as *const u8 as *const libc::c_char,
     b"ab\x00" as *const u8 as *const libc::c_char,
     b"ag\x00" as *const u8 as *const libc::c_char,
     b"aks\x00" as *const u8 as *const libc::c_char,
     b"ala\x00" as *const u8 as *const libc::c_char,
     b"an\x00" as *const u8 as *const libc::c_char,
     b"ankh\x00" as *const u8 as *const libc::c_char,
     b"app\x00" as *const u8 as *const libc::c_char,
     b"arg\x00" as *const u8 as *const libc::c_char,
     b"arze\x00" as *const u8 as *const libc::c_char,
     b"ash\x00" as *const u8 as *const libc::c_char,
     b"aus\x00" as *const u8 as *const libc::c_char,
     b"ban\x00" as *const u8 as *const libc::c_char,
     b"bar\x00" as *const u8 as *const libc::c_char,
     b"bat\x00" as *const u8 as *const libc::c_char,
     b"bek\x00" as *const u8 as *const libc::c_char,
     b"bie\x00" as *const u8 as *const libc::c_char,
     b"bin\x00" as *const u8 as *const libc::c_char,
     b"bit\x00" as *const u8 as *const libc::c_char,
     b"bjor\x00" as *const u8 as *const libc::c_char,
     b"blu\x00" as *const u8 as *const libc::c_char,
     b"bot\x00" as *const u8 as *const libc::c_char,
     b"bu\x00" as *const u8 as *const libc::c_char,
     b"byt\x00" as *const u8 as *const libc::c_char,
     b"comp\x00" as *const u8 as *const libc::c_char,
     b"con\x00" as *const u8 as *const libc::c_char,
     b"cos\x00" as *const u8 as *const libc::c_char,
     b"cre\x00" as *const u8 as *const libc::c_char,
     b"dalf\x00" as *const u8 as *const libc::c_char,
     b"dan\x00" as *const u8 as *const libc::c_char,
     b"den\x00" as *const u8 as *const libc::c_char,
     b"der\x00" as *const u8 as *const libc::c_char,
     b"doe\x00" as *const u8 as *const libc::c_char,
     b"dok\x00" as *const u8 as *const libc::c_char,
     b"eep\x00" as *const u8 as *const libc::c_char,
     b"el\x00" as *const u8 as *const libc::c_char,
     b"eng\x00" as *const u8 as *const libc::c_char,
     b"er\x00" as *const u8 as *const libc::c_char,
     b"ere\x00" as *const u8 as *const libc::c_char,
     b"erk\x00" as *const u8 as *const libc::c_char,
     b"esh\x00" as *const u8 as *const libc::c_char,
     b"evs\x00" as *const u8 as *const libc::c_char,
     b"fa\x00" as *const u8 as *const libc::c_char,
     b"fid\x00" as *const u8 as *const libc::c_char,
     b"flit\x00" as *const u8 as *const libc::c_char,
     b"for\x00" as *const u8 as *const libc::c_char,
     b"fri\x00" as *const u8 as *const libc::c_char,
     b"fu\x00" as *const u8 as *const libc::c_char,
     b"gan\x00" as *const u8 as *const libc::c_char,
     b"gar\x00" as *const u8 as *const libc::c_char,
     b"glen\x00" as *const u8 as *const libc::c_char,
     b"gop\x00" as *const u8 as *const libc::c_char,
     b"gre\x00" as *const u8 as *const libc::c_char,
     b"ha\x00" as *const u8 as *const libc::c_char,
     b"he\x00" as *const u8 as *const libc::c_char,
     b"hyd\x00" as *const u8 as *const libc::c_char,
     b"i\x00" as *const u8 as *const libc::c_char,
     b"ing\x00" as *const u8 as *const libc::c_char,
     b"ion\x00" as *const u8 as *const libc::c_char,
     b"ip\x00" as *const u8 as *const libc::c_char,
     b"ish\x00" as *const u8 as *const libc::c_char,
     b"it\x00" as *const u8 as *const libc::c_char,
     b"ite\x00" as *const u8 as *const libc::c_char,
     b"iv\x00" as *const u8 as *const libc::c_char,
     b"jo\x00" as *const u8 as *const libc::c_char,
     b"kho\x00" as *const u8 as *const libc::c_char,
     b"kli\x00" as *const u8 as *const libc::c_char,
     b"klis\x00" as *const u8 as *const libc::c_char,
     b"la\x00" as *const u8 as *const libc::c_char,
     b"lech\x00" as *const u8 as *const libc::c_char,
     b"man\x00" as *const u8 as *const libc::c_char,
     b"mar\x00" as *const u8 as *const libc::c_char,
     b"me\x00" as *const u8 as *const libc::c_char,
     b"mi\x00" as *const u8 as *const libc::c_char,
     b"mic\x00" as *const u8 as *const libc::c_char,
     b"mik\x00" as *const u8 as *const libc::c_char,
     b"mon\x00" as *const u8 as *const libc::c_char,
     b"mung\x00" as *const u8 as *const libc::c_char,
     b"mur\x00" as *const u8 as *const libc::c_char,
     b"nag\x00" as *const u8 as *const libc::c_char,
     b"nej\x00" as *const u8 as *const libc::c_char,
     b"nelg\x00" as *const u8 as *const libc::c_char,
     b"nep\x00" as *const u8 as *const libc::c_char,
     b"ner\x00" as *const u8 as *const libc::c_char,
     b"nes\x00" as *const u8 as *const libc::c_char,
     b"nis\x00" as *const u8 as *const libc::c_char,
     b"nih\x00" as *const u8 as *const libc::c_char,
     b"nin\x00" as *const u8 as *const libc::c_char,
     b"o\x00" as *const u8 as *const libc::c_char,
     b"od\x00" as *const u8 as *const libc::c_char,
     b"ood\x00" as *const u8 as *const libc::c_char,
     b"org\x00" as *const u8 as *const libc::c_char,
     b"orn\x00" as *const u8 as *const libc::c_char,
     b"ox\x00" as *const u8 as *const libc::c_char,
     b"oxy\x00" as *const u8 as *const libc::c_char,
     b"pay\x00" as *const u8 as *const libc::c_char,
     b"pet\x00" as *const u8 as *const libc::c_char,
     b"ple\x00" as *const u8 as *const libc::c_char,
     b"plu\x00" as *const u8 as *const libc::c_char,
     b"po\x00" as *const u8 as *const libc::c_char,
     b"pot\x00" as *const u8 as *const libc::c_char,
     b"prok\x00" as *const u8 as *const libc::c_char,
     b"re\x00" as *const u8 as *const libc::c_char,
     b"rea\x00" as *const u8 as *const libc::c_char,
     b"rhov\x00" as *const u8 as *const libc::c_char,
     b"ri\x00" as *const u8 as *const libc::c_char,
     b"ro\x00" as *const u8 as *const libc::c_char,
     b"rog\x00" as *const u8 as *const libc::c_char,
     b"rok\x00" as *const u8 as *const libc::c_char,
     b"rol\x00" as *const u8 as *const libc::c_char,
     b"sa\x00" as *const u8 as *const libc::c_char,
     b"san\x00" as *const u8 as *const libc::c_char,
     b"sat\x00" as *const u8 as *const libc::c_char,
     b"see\x00" as *const u8 as *const libc::c_char,
     b"sef\x00" as *const u8 as *const libc::c_char,
     b"seh\x00" as *const u8 as *const libc::c_char,
     b"shu\x00" as *const u8 as *const libc::c_char,
     b"ski\x00" as *const u8 as *const libc::c_char,
     b"sna\x00" as *const u8 as *const libc::c_char,
     b"sne\x00" as *const u8 as *const libc::c_char,
     b"snik\x00" as *const u8 as *const libc::c_char,
     b"sno\x00" as *const u8 as *const libc::c_char,
     b"so\x00" as *const u8 as *const libc::c_char,
     b"sol\x00" as *const u8 as *const libc::c_char,
     b"sri\x00" as *const u8 as *const libc::c_char,
     b"sta\x00" as *const u8 as *const libc::c_char,
     b"sun\x00" as *const u8 as *const libc::c_char,
     b"ta\x00" as *const u8 as *const libc::c_char,
     b"tab\x00" as *const u8 as *const libc::c_char,
     b"tem\x00" as *const u8 as *const libc::c_char,
     b"ther\x00" as *const u8 as *const libc::c_char,
     b"ti\x00" as *const u8 as *const libc::c_char,
     b"tox\x00" as *const u8 as *const libc::c_char,
     b"trol\x00" as *const u8 as *const libc::c_char,
     b"tue\x00" as *const u8 as *const libc::c_char,
     b"turs\x00" as *const u8 as *const libc::c_char,
     b"u\x00" as *const u8 as *const libc::c_char,
     b"ulk\x00" as *const u8 as *const libc::c_char,
     b"um\x00" as *const u8 as *const libc::c_char,
     b"un\x00" as *const u8 as *const libc::c_char,
     b"uni\x00" as *const u8 as *const libc::c_char,
     b"ur\x00" as *const u8 as *const libc::c_char,
     b"val\x00" as *const u8 as *const libc::c_char,
     b"viv\x00" as *const u8 as *const libc::c_char,
     b"vly\x00" as *const u8 as *const libc::c_char,
     b"vom\x00" as *const u8 as *const libc::c_char,
     b"wah\x00" as *const u8 as *const libc::c_char,
     b"wed\x00" as *const u8 as *const libc::c_char,
     b"werg\x00" as *const u8 as *const libc::c_char,
     b"wex\x00" as *const u8 as *const libc::c_char,
     b"whon\x00" as *const u8 as *const libc::c_char,
     b"wun\x00" as *const u8 as *const libc::c_char,
     b"x\x00" as *const u8 as *const libc::c_char,
     b"yerg\x00" as *const u8 as *const libc::c_char,
     b"yp\x00" as *const u8 as *const libc::c_char,
     b"zun\x00" as *const u8 as *const libc::c_char,
     b"tri\x00" as *const u8 as *const libc::c_char,
     b"blaa\x00" as *const u8 as *const libc::c_char,
     b"jah\x00" as *const u8 as *const libc::c_char,
     b"bul\x00" as *const u8 as *const libc::c_char,
     b"on\x00" as *const u8 as *const libc::c_char,
     b"foo\x00" as *const u8 as *const libc::c_char,
     b"ju\x00" as *const u8 as *const libc::c_char,
     b"xuxu\x00" as *const u8 as *const libc::c_char];
/*
 * Hold the titles of scrolls, 6 to 14 characters each
 * Also keep an array of scroll colors (always WHITE for now)
 */
static mut scroll_adj: [[libc::c_char; 16]; 55] = [[0; 16]; 55];
static mut scroll_col: [byte_hack; 55] = [0; 55];
/*
 * Certain items have a flavor
 * This function is used only by "flavor_init()"
 */
unsafe extern "C" fn object_flavor(mut k_idx: libc::c_int) -> bool_ {
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset(k_idx as isize) as *mut object_kind;
    /* Analyze the item */
    match (*k_ptr).tval as libc::c_int {
        40 => {
            return (0x80 as libc::c_int +
                        amulet_col[(*k_ptr).sval as usize] as libc::c_int) as
                       bool_
        }
        45 => {
            return (0x90 as libc::c_int +
                        ring_col[(*k_ptr).sval as usize] as libc::c_int) as
                       bool_
        }
        55 => {
            return (0xa0 as libc::c_int +
                        staff_col[(*k_ptr).sval as usize] as libc::c_int) as
                       bool_
        }
        65 => {
            return (0xb0 as libc::c_int +
                        wand_col[(*k_ptr).sval as usize] as libc::c_int) as
                       bool_
        }
        66 => {
            return (0xc0 as libc::c_int +
                        rod_col[(*k_ptr).sval as usize] as libc::c_int) as
                       bool_
        }
        70 => {
            return (0xd0 as libc::c_int +
                        scroll_col[(*k_ptr).sval as usize] as libc::c_int) as
                       bool_
        }
        71 | 72 => {
            return (0xe0 as libc::c_int +
                        potion_col[(*k_ptr).sval as usize] as libc::c_int) as
                       bool_
        }
        80 => {
            if ((*k_ptr).sval as libc::c_int) < 32 as libc::c_int {
                return (0xf0 as libc::c_int +
                            food_col[(*k_ptr).sval as usize] as libc::c_int)
                           as bool_
            }
        }
        _ => { }
    }
    /* No flavor */
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn get_table_name(mut out_string: *mut libc::c_char) {
    let mut testcounter: libc::c_int =
        Rand_div(3 as libc::c_int) + 1 as libc::c_int + 1 as libc::c_int;
    strcpy(out_string, b"\'\x00" as *const u8 as *const libc::c_char);
    if Rand_div(3 as libc::c_int) + 1 as libc::c_int == 2 as libc::c_int {
        loop  {
            let fresh0 = testcounter;
            testcounter = testcounter - 1;
            if !(fresh0 != 0) { break ; }
            strcat(out_string,
                   syllables[(Rand_div(164 as libc::c_int) + 1 as libc::c_int
                                  - 1 as libc::c_int) as usize]);
        }
    } else {
        let mut Syllable: [libc::c_char; 80] = [0; 80];
        testcounter =
            Rand_div(2 as libc::c_int) + 1 as libc::c_int + 1 as libc::c_int;
        loop  {
            let fresh1 = testcounter;
            testcounter = testcounter - 1;
            if !(fresh1 != 0) { break ; }
            get_rnd_line(b"elvish.txt\x00" as *const u8 as *const libc::c_char
                             as *mut libc::c_char, Syllable.as_mut_ptr());
            strcat(out_string, Syllable.as_mut_ptr());
        }
    }
    *out_string.offset(1 as libc::c_int as isize) =
        toupper(*out_string.offset(1 as libc::c_int as isize) as libc::c_int)
            as libc::c_char;
    strcat(out_string, b"\'\x00" as *const u8 as *const libc::c_char);
    *out_string.offset(18 as libc::c_int as isize) =
        '\u{0}' as i32 as libc::c_char;
}
/*
 * Certain items, if aware, are known instantly
 * This function is used only by "flavor_init()"
 *
 * XXX XXX XXX Add "EASY_KNOW" flag to "k_info.txt" file
 */
unsafe extern "C" fn object_easy_know(mut i: libc::c_int) -> bool_ {
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset(i as isize) as *mut object_kind;
    /* Analyze the "tval" */
    match (*k_ptr).tval as libc::c_int {
        114 | 113 | 112 => {
            /* Spellbooks */
            return 1 as libc::c_int as bool_
        }
        77 | 10 | 2 | 1 | 9 | 99 | 5 | 11 => {
            /* Simple items */
            return 1 as libc::c_int as bool_
        }
        80 | 71 | 72 | 70 | 66 | 67 | 4 => {
            /* All Food, Potions, Scrolls, Rods */
            if (*k_ptr).flags3 as libc::c_long & 0x8000 as libc::c_long != 0 {
                return 0 as libc::c_int as bool_
            }
            return 1 as libc::c_int as bool_
        }
        45 | 40 | 39 => {
            /* Some Rings, Amulets, Lites */
            if (*k_ptr).flags3 as libc::c_long & 0x100 as libc::c_long != 0 {
                return 1 as libc::c_int as bool_
            }
            return 0 as libc::c_int as bool_
        }
        _ => { }
    }
    /* Nope */
    return 0 as libc::c_int as bool_;
}
/*
 * Prepare the "variable" part of the "k_info" array.
 *
 * The "color"/"metal"/"type" of an item is its "flavor".
 * For the most part, flavors are assigned randomly each game.
 *
 * Initialize descriptions for the "colored" objects, including:
 * Rings, Amulets, Staffs, Wands, Rods, Food, Potions, Scrolls.
 *
 * The first 4 entries for potions are fixed (Water, Apple Juice,
 * Slime Mold Juice, Unused Potion).
 *
 * Scroll titles are always between 6 and 14 letters long.  This is
 * ensured because every title is composed of whole words, where every
 * word is from 1 to 8 letters long (one or two syllables of 1 to 4
 * letters each), and that no scroll is finished until it attempts to
 * grow beyond 15 letters.  The first time this can happen is when the
 * current title has 6 letters and the new word has 8 letters, which
 * would result in a 6 letter scroll title.
 *
 * Duplicate titles are avoided by requiring that no two scrolls share
 * the same first four letters (not the most efficient method, and not
 * the least efficient method, but it will always work).
 *
 * Hack -- make sure everything stays the same for each saved game
 * This is accomplished by the use of a saved "random seed", as in
 * "town_gen()".  Since no other functions are called while the special
 * seed is in effect, so this function is pretty "safe".
 *
 * Note that the "hacked seed" may provide an RNG with alternating parity!
 */
#[no_mangle]
pub unsafe extern "C" fn flavor_init() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut temp_col: byte_hack = 0;
    let mut temp_adj: cptr = 0 as *const libc::c_char;
    /* Hack -- Use the "simple" RNG */
    Rand_quick = 1 as libc::c_int as bool_;
    /* Hack -- Induce consistant flavors */
    Rand_value = seed_flavor;
    /* Efficiency -- Rods/Wands share initial array */
    i = 0 as libc::c_int;
    while i < 39 as libc::c_int {
        rod_adj[i as usize] = wand_adj[i as usize];
        rod_col[i as usize] = wand_col[i as usize];
        i += 1
    }
    /* Rings have "ring colors" */
    i = 0 as libc::c_int;
    while i < 62 as libc::c_int {
        j = Rand_div(62 as libc::c_int);
        temp_adj = ring_adj[i as usize];
        ring_adj[i as usize] = ring_adj[j as usize];
        ring_adj[j as usize] = temp_adj;
        temp_col = ring_col[i as usize];
        ring_col[i as usize] = ring_col[j as usize];
        ring_col[j as usize] = temp_col;
        i += 1
    }
    /* Amulets have "amulet colors" */
    i = 0 as libc::c_int;
    while i < 34 as libc::c_int {
        j = Rand_div(34 as libc::c_int);
        temp_adj = amulet_adj[i as usize];
        amulet_adj[i as usize] = amulet_adj[j as usize];
        amulet_adj[j as usize] = temp_adj;
        temp_col = amulet_col[i as usize];
        amulet_col[i as usize] = amulet_col[j as usize];
        amulet_col[j as usize] = temp_col;
        i += 1
    }
    /* Staffs */
    i = 0 as libc::c_int;
    while i < 35 as libc::c_int {
        j = Rand_div(35 as libc::c_int);
        temp_adj = staff_adj[i as usize];
        staff_adj[i as usize] = staff_adj[j as usize];
        staff_adj[j as usize] = temp_adj;
        temp_col = staff_col[i as usize];
        staff_col[i as usize] = staff_col[j as usize];
        staff_col[j as usize] = temp_col;
        i += 1
    }
    /* Wands */
    i = 0 as libc::c_int;
    while i < 39 as libc::c_int {
        j = Rand_div(39 as libc::c_int);
        temp_adj = wand_adj[i as usize];
        wand_adj[i as usize] = wand_adj[j as usize];
        wand_adj[j as usize] = temp_adj;
        temp_col = wand_col[i as usize];
        wand_col[i as usize] = wand_col[j as usize];
        wand_col[j as usize] = temp_col;
        i += 1
    }
    /* Rods */
    i = 0 as libc::c_int;
    while i < 39 as libc::c_int {
        j = Rand_div(39 as libc::c_int);
        temp_adj = rod_adj[i as usize];
        rod_adj[i as usize] = rod_adj[j as usize];
        rod_adj[j as usize] = temp_adj;
        temp_col = rod_col[i as usize];
        rod_col[i as usize] = rod_col[j as usize];
        rod_col[j as usize] = temp_col;
        i += 1
    }
    /* Foods (Mushrooms) */
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        j = Rand_div(20 as libc::c_int);
        temp_adj = food_adj[i as usize];
        food_adj[i as usize] = food_adj[j as usize];
        food_adj[j as usize] = temp_adj;
        temp_col = food_col[i as usize];
        food_col[i as usize] = food_col[j as usize];
        food_col[j as usize] = temp_col;
        i += 1
    }
    /* Potions */
    i = 4 as libc::c_int;
    while i < 66 as libc::c_int {
        j = Rand_div(66 as libc::c_int - 4 as libc::c_int) + 4 as libc::c_int;
        temp_adj = potion_adj[i as usize];
        potion_adj[i as usize] = potion_adj[j as usize];
        potion_adj[j as usize] = temp_adj;
        temp_col = potion_col[i as usize];
        potion_col[i as usize] = potion_col[j as usize];
        potion_col[j as usize] = temp_col;
        i += 1
    }
    /* Scrolls (random titles, always white) */
    i = 0 as libc::c_int;
    while i < 55 as libc::c_int {
        loop 
             /* Get a new title */
             {
            let mut buf: [libc::c_char; 80] = [0; 80];
            let mut okay: bool_ = 0;
            /* Start a new title */
            buf[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
            loop 
                 /* Collect words until done */
                 {
                let mut q: libc::c_int = 0;
                let mut s: libc::c_int = 0;
                let mut tmp: [libc::c_char; 80] = [0; 80];
                /* Start a new word */
                tmp[0 as libc::c_int as usize] =
                    '\u{0}' as i32 as libc::c_char;
                /* Choose one or two syllables */
                s =
                    if Rand_div(100 as libc::c_int) < 30 as libc::c_int {
                        1 as libc::c_int
                    } else { 2 as libc::c_int };
                /* Add a one or two syllable word */
                q = 0 as libc::c_int;
                while q < s {
                    /* Add the syllable */
                    strcat(tmp.as_mut_ptr(),
                           syllables[Rand_div(164 as libc::c_int) as usize]);
                    q += 1
                }
                /* Stop before getting too long */
                if strlen(buf.as_mut_ptr()).wrapping_add(1 as libc::c_int as
                                                             libc::c_ulong).wrapping_add(strlen(tmp.as_mut_ptr()))
                       > 15 as libc::c_int as libc::c_ulong {
                    break ;
                }
                /* Add a space */
                strcat(buf.as_mut_ptr(),
                       b" \x00" as *const u8 as *const libc::c_char);
                /* Add the word */
                strcat(buf.as_mut_ptr(), tmp.as_mut_ptr());
            }
            /* Save the title */
            strcpy(scroll_adj[i as usize].as_mut_ptr(),
                   buf.as_mut_ptr().offset(1 as libc::c_int as isize));
            /* Assume okay */
            okay = 1 as libc::c_int as bool_;
            /* Check for "duplicate" scroll titles */
            j = 0 as libc::c_int;
            while j < i {
                let mut hack1: cptr =
                    scroll_adj[j as usize].as_mut_ptr() as cptr;
                let mut hack2: cptr =
                    scroll_adj[i as usize].as_mut_ptr() as cptr;
                /* Compare first four characters */
                let fresh2 = hack1;
                hack1 = hack1.offset(1);
                let fresh3 = hack2;
                hack2 = hack2.offset(1);
                if !(*fresh2 as libc::c_int != *fresh3 as libc::c_int) {
                    let fresh4 = hack1;
                    hack1 = hack1.offset(1);
                    let fresh5 = hack2;
                    hack2 = hack2.offset(1);
                    if !(*fresh4 as libc::c_int != *fresh5 as libc::c_int) {
                        let fresh6 = hack1;
                        hack1 = hack1.offset(1);
                        let fresh7 = hack2;
                        hack2 = hack2.offset(1);
                        if !(*fresh6 as libc::c_int != *fresh7 as libc::c_int)
                           {
                            let fresh8 = hack1;
                            hack1 = hack1.offset(1);
                            let fresh9 = hack2;
                            hack2 = hack2.offset(1);
                            if !(*fresh8 as libc::c_int !=
                                     *fresh9 as libc::c_int) {
                                /* Not okay */
                                okay = 0 as libc::c_int as bool_;
                                break ;
                            }
                        }
                    }
                }
                j += 1
            }
            /* Break when done */
            if okay != 0 { break ; }
        }
        /* All scrolls are white */
        scroll_col[i as usize] = 1 as libc::c_int as byte_hack;
        i += 1
    }
    /* Hack -- Use the "complex" RNG */
    Rand_quick = 0 as libc::c_int as bool_;
    /* Analyze every object */
    i = 1 as libc::c_int;
    while i < max_k_idx as libc::c_int {
        let mut k_ptr: *mut object_kind =
            &mut *k_info.offset(i as isize) as *mut object_kind;
        /* Skip "empty" objects */
        if !((*k_ptr).name == 0) {
            /* Extract "flavor" (if any) */
            (*k_ptr).flavor = object_flavor(i) as byte_hack;
            /* No flavor yields aware */
            if (*k_ptr).flavor == 0 &&
                   (*k_ptr).tval as libc::c_int != 67 as libc::c_int {
                (*k_ptr).aware = 1 as libc::c_int as bool_
            }
            /* Check for "easily known" */
            (*k_ptr).easy_know = object_easy_know(i)
        }
        i += 1
    };
}
/*
 * Reset the "visual" lists
 *
 * This involves resetting various things to their "default" state.
 *
 * If the "prefs" flag is TRUE, then we will also load the appropriate
 * "user pref file" based on the current setting of the "use_graphics"
 * flag.  This is useful for switching "graphics" on/off.
 *
 * The features, objects, and monsters, should all be encoded in the
 * relevant "font.pref" and/or "graf.prf" files.  XXX XXX XXX
 *
 * The "prefs" parameter is no longer meaningful.  XXX XXX XXX
 */
#[no_mangle]
pub unsafe extern "C" fn reset_visuals() {
    let mut i: libc::c_int = 0;
    /* Extract some info about terrain features */
    i = 0 as libc::c_int;
    while i < max_f_idx as libc::c_int {
        let mut f_ptr: *mut feature_type =
            &mut *f_info.offset(i as isize) as *mut feature_type;
        /* Assume we will use the underlying values */
        (*f_ptr).x_attr = (*f_ptr).d_attr;
        (*f_ptr).x_char = (*f_ptr).d_char;
        i += 1
    }
    /* Extract default attr/char code for stores */
    i = 0 as libc::c_int;
    while i < max_st_idx as libc::c_int {
        let mut st_ptr: *mut store_info_type =
            &mut *st_info.offset(i as isize) as *mut store_info_type;
        /* Default attr/char */
        (*st_ptr).x_attr = (*st_ptr).d_attr;
        (*st_ptr).x_char = (*st_ptr).d_char;
        i += 1
    }
    /* Extract default attr/char code for objects */
    i = 0 as libc::c_int;
    while i < max_k_idx as libc::c_int {
        let mut k_ptr: *mut object_kind =
            &mut *k_info.offset(i as isize) as *mut object_kind;
        /* Default attr/char */
        (*k_ptr).x_attr = (*k_ptr).d_attr;
        (*k_ptr).x_char = (*k_ptr).d_char;
        i += 1
    }
    /* Extract default attr/char code for monsters */
    i = 0 as libc::c_int;
    while i < max_r_idx as libc::c_int {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset(i as isize) as *mut monster_race;
        /* Default attr/char */
        (*r_ptr).x_attr = (*r_ptr).d_attr;
        (*r_ptr).x_char = (*r_ptr).d_char;
        i += 1
    }
    /* Reset attr/char code for ego monster overlay graphics */
    i = 0 as libc::c_int;
    while i < max_re_idx as libc::c_int {
        let mut re_ptr: *mut monster_ego =
            &mut *re_info.offset(i as isize) as *mut monster_ego;
        /* Default attr/char */
        (*re_ptr).g_attr = 0 as libc::c_int as byte_hack;
        (*re_ptr).g_char = 0 as libc::c_int as libc::c_char;
        i += 1
    }
    /* Reset attr/char code for race modifier overlay graphics */
    i = 0 as libc::c_int;
    while i < max_rmp_idx as libc::c_int {
        let mut rmp_ptr: *mut player_race_mod =
            &mut *race_mod_info.offset(i as isize) as *mut player_race_mod;
        /* Default attr/char */
        (*rmp_ptr).g_attr = 0 as libc::c_int as byte_hack;
        (*rmp_ptr).g_char = 0 as libc::c_int as libc::c_char;
        i += 1
    }
    /* Reset attr/char code for trap overlay graphics */
    i = 0 as libc::c_int;
    while i < max_rmp_idx as libc::c_int {
        let mut t_ptr: *mut trap_type =
            &mut *t_info.offset(i as isize) as *mut trap_type;
        /* Default attr/char */
        (*t_ptr).g_attr = 0 as libc::c_int as byte_hack;
        (*t_ptr).g_char = 0 as libc::c_int as libc::c_char;
        i += 1
    }
    if use_graphics != 0 {
        /* Process "graf.prf" */
        process_pref_file(b"graf.prf\x00" as *const u8 as
                              *const libc::c_char);
        /*
		 * Hack -- remember graphics mode as an integer value,
		 * for faster processing of map_info()
		 */
        /* IBM-PC pseudo-graphics -- not maintained, but the code is there */
        if streq(ANGBAND_SYS, b"ibm\x00" as *const u8 as *const libc::c_char)
               != 0 {
            graphics_mode = 2 as libc::c_int as byte_hack
        } else if streq(ANGBAND_GRAF,
                        b"iso\x00" as *const u8 as *const libc::c_char) != 0 {
            graphics_mode = 5 as libc::c_int as byte_hack
        } else if streq(ANGBAND_GRAF,
                        b"new\x00" as *const u8 as *const libc::c_char) != 0 {
            graphics_mode = 4 as libc::c_int as byte_hack
        } else if streq(ANGBAND_GRAF,
                        b"old\x00" as *const u8 as *const libc::c_char) != 0 {
            graphics_mode = 3 as libc::c_int as byte_hack
        } else {
            /*
		 * Isometric view. Also assumes all the attributes of the "new"
		 * graphics.
		 */
            /*
		 * "New" graphics -- supports graphics overlay for traps, ego monsters
		 * and player subraces, and has tiles for lighting effects (row + 1
		 * and row + 2 for "darker" versions of terrain features)
		 */
            /*
		 * "Old" graphics -- doesn't support graphics overlay and lighting
		 * effects
		 */
            /* ??? */
            graphics_mode = 1 as libc::c_int as byte_hack
        }
    } else {
        /* Normal symbols */
        /* Process "font.prf" */
        process_pref_file(b"font.prf\x00" as *const u8 as
                              *const libc::c_char);
        graphics_mode = 0 as libc::c_int as byte_hack
    };
}
/*
 * Extract "xtra" flags from object.
 */
#[no_mangle]
pub unsafe extern "C" fn object_flags_xtra(mut o_ptr: *mut object_type,
                                           mut f2: *mut u32b,
                                           mut f3: *mut u32b,
                                           mut esp: *mut u32b) {
    match (*o_ptr).xtra1 as libc::c_int {
        1 => {
            /* Choose a sustain */
            match (*o_ptr).xtra2 as libc::c_int % 6 as libc::c_int {
                0 => {
                    *f2 = (*f2 as libc::c_long | 0x1 as libc::c_long) as u32b
                }
                1 => {
                    *f2 = (*f2 as libc::c_long | 0x2 as libc::c_long) as u32b
                }
                2 => {
                    *f2 = (*f2 as libc::c_long | 0x4 as libc::c_long) as u32b
                }
                3 => {
                    *f2 = (*f2 as libc::c_long | 0x8 as libc::c_long) as u32b
                }
                4 => {
                    *f2 = (*f2 as libc::c_long | 0x10 as libc::c_long) as u32b
                }
                5 => {
                    *f2 = (*f2 as libc::c_long | 0x20 as libc::c_long) as u32b
                }
                _ => { }
            }
        }
        2 => {
            /* Choose a power */
            match (*o_ptr).xtra2 as libc::c_int % 11 as libc::c_int {
                0 => {
                    *f2 =
                        (*f2 as libc::c_long | 0x1000000 as libc::c_long) as
                            u32b
                }
                1 => {
                    *f2 =
                        (*f2 as libc::c_long | 0x2000000 as libc::c_long) as
                            u32b
                }
                2 => {
                    *f2 =
                        (*f2 as libc::c_long | 0x4000000 as libc::c_long) as
                            u32b
                }
                3 => {
                    *f2 =
                        (*f2 as libc::c_long | 0x8000000 as libc::c_long) as
                            u32b
                }
                4 => {
                    *f2 =
                        (*f2 as libc::c_long | 0x10000000 as libc::c_long) as
                            u32b
                }
                5 => {
                    *f2 =
                        (*f2 as libc::c_long | 0x20000000 as libc::c_long) as
                            u32b
                }
                6 => {
                    *f2 =
                        (*f2 as libc::c_long | 0x40000000 as libc::c_long) as
                            u32b
                }
                7 => {
                    *f2 =
                        (*f2 as libc::c_long | 0x80000000 as libc::c_long) as
                            u32b
                }
                8 => {
                    *f2 =
                        (*f2 as libc::c_long | 0x100000 as libc::c_long) as
                            u32b
                }
                9 => {
                    *f2 =
                        (*f2 as libc::c_long | 0x800000 as libc::c_long) as
                            u32b
                }
                10 => {
                    *f2 =
                        (*f2 as libc::c_long | 0x400000 as libc::c_long) as
                            u32b
                }
                _ => { }
            }
        }
        3 => {
            /* Choose an ability */
            match (*o_ptr).xtra2 as libc::c_int % 8 as libc::c_int {
                0 => {
                    *f3 =
                        (*f3 as libc::c_long | 0x1000 as libc::c_long) as u32b
                }
                1 => {
                    *f3 =
                        (*f3 as libc::c_long | 0x2000 as libc::c_long) as u32b
                }
                2 => {
                    *f3 =
                        (*f3 as libc::c_long | 0x4000 as libc::c_long) as u32b
                }
                3 => {
                    *esp =
                        (*esp as libc::c_long | 0x80000000 as libc::c_long) as
                            u32b
                }
                4 => {
                    *f3 =
                        (*f3 as libc::c_long | 0x10000 as libc::c_long) as
                            u32b
                }
                5 => {
                    *f3 =
                        (*f3 as libc::c_long | 0x20000 as libc::c_long) as
                            u32b
                }
                6 => {
                    *f2 =
                        (*f2 as libc::c_long | 0x4000 as libc::c_long) as u32b
                }
                7 => {
                    *f2 =
                        (*f2 as libc::c_long | 0x8000 as libc::c_long) as u32b
                }
                _ => { }
            }
        }
        _ => { }
    };
}
/*
 * Obtain the "flags" for an item
 */
#[no_mangle]
pub static mut object_flags_no_set: bool_ = 0 as libc::c_int as bool_;
#[no_mangle]
pub unsafe extern "C" fn object_flags(mut o_ptr: *mut object_type,
                                      mut f1: *mut u32b, mut f2: *mut u32b,
                                      mut f3: *mut u32b, mut f4: *mut u32b,
                                      mut f5: *mut u32b, mut esp: *mut u32b) {
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset((*o_ptr).k_idx as isize) as *mut object_kind;
    /* Base object */
    *f1 = (*k_ptr).flags1;
    *f2 = (*k_ptr).flags2;
    *f3 = (*k_ptr).flags3;
    *f4 = (*k_ptr).flags4;
    *f5 = (*k_ptr).flags5;
    *esp = (*k_ptr).esp;
    /* Artifact */
    if (*o_ptr).name1 != 0 {
        let mut a_ptr: *mut artifact_type =
            &mut *a_info.offset((*o_ptr).name1 as isize) as
                *mut artifact_type;
        *f1 = (*a_ptr).flags1;
        *f2 = (*a_ptr).flags2;
        *f3 = (*a_ptr).flags3;
        *f4 = (*a_ptr).flags4;
        *f5 = (*a_ptr).flags5;
        *esp = (*a_ptr).esp;
        if object_flags_no_set == 0 &&
               (*a_ptr).set as libc::c_int != -(1 as libc::c_int) {
            apply_flags_set((*o_ptr).name1 as s16b, (*a_ptr).set, f1, f2, f3,
                            f4, f5, esp);
        }
    }
    /* Random artifact ! */
    if (*o_ptr).art_flags1 != 0 || (*o_ptr).art_flags2 != 0 ||
           (*o_ptr).art_flags3 != 0 || (*o_ptr).art_flags4 != 0 ||
           (*o_ptr).art_flags5 != 0 || (*o_ptr).art_esp != 0 {
        *f1 |= (*o_ptr).art_flags1;
        *f2 |= (*o_ptr).art_flags2;
        *f3 |= (*o_ptr).art_flags3;
        *f4 |= (*o_ptr).art_flags4;
        *f5 |= (*o_ptr).art_flags5;
        *esp |= (*o_ptr).art_esp
    }
    /* Extra powers */
    if (*o_ptr).art_name == 0 { object_flags_xtra(o_ptr, f2, f3, esp); };
}
/* Return object granted power */
#[no_mangle]
pub unsafe extern "C" fn object_power(mut o_ptr: *mut object_type)
 -> libc::c_int {
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset((*o_ptr).k_idx as isize) as *mut object_kind;
    let mut power: libc::c_int = -(1 as libc::c_int);
    /* Base object */
    power = (*k_ptr).power as libc::c_int;
    /* Ego-item */
    if (*o_ptr).name2 != 0 {
        let mut e_ptr: *mut ego_item_type =
            &mut *e_info.offset((*o_ptr).name2 as isize) as
                *mut ego_item_type;
        if power == -(1 as libc::c_int) {
            power = (*e_ptr).power as libc::c_int
        }
        if (*o_ptr).name2b != 0 {
            let mut e_ptr_0: *mut ego_item_type =
                &mut *e_info.offset((*o_ptr).name2b as isize) as
                    *mut ego_item_type;
            if power == -(1 as libc::c_int) {
                power = (*e_ptr_0).power as libc::c_int
            }
        }
    }
    /* Artifact */
    if (*o_ptr).name1 != 0 {
        let mut a_ptr: *mut artifact_type =
            &mut *a_info.offset((*o_ptr).name1 as isize) as
                *mut artifact_type;
        if power == -(1 as libc::c_int) {
            power = (*a_ptr).power as libc::c_int
        }
    }
    return power;
}
/*
 * Obtain the "flags" for an item which are known to the player
 */
#[no_mangle]
pub unsafe extern "C" fn object_flags_known(mut o_ptr: *mut object_type,
                                            mut f1: *mut u32b,
                                            mut f2: *mut u32b,
                                            mut f3: *mut u32b,
                                            mut f4: *mut u32b,
                                            mut f5: *mut u32b,
                                            mut esp: *mut u32b) {
    let mut spoil: bool_ = 0 as libc::c_int as bool_;
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset((*o_ptr).k_idx as isize) as *mut object_kind;
    /* Clear */
    *f5 = 0 as libc::c_long as u32b;
    *esp = *f5;
    *f4 = *esp;
    *f3 = *f4;
    *f2 = *f3;
    *f1 = *f2;
    /* Must be identified */
    if !((*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
             (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as
                 libc::c_int != 0 &&
                 (*k_info.offset((*o_ptr).k_idx as isize)).aware as
                     libc::c_int != 0) {
        return
    }
    /* Base object */
    *f1 = (*k_ptr).flags1;
    *f2 = (*k_ptr).flags2;
    *f3 = (*k_ptr).flags3;
    *f4 = (*k_ptr).flags4;
    *f5 = (*k_ptr).flags5;
    *esp = (*k_ptr).esp;
    *f1 |= (*k_ptr).oflags1;
    *f2 |= (*k_ptr).oflags2;
    *f3 |= (*k_ptr).oflags3;
    *f4 |= (*k_ptr).oflags4;
    *f5 |= (*k_ptr).oflags5;
    *esp |= (*k_ptr).oesp;
    /* SPOIL_ARTIFACTS */
    /* SPOIL_EGO_ITEMS */
    /* Artifact */
    if (*o_ptr).name1 != 0 {
        let mut a_ptr: *mut artifact_type =
            &mut *a_info.offset((*o_ptr).name1 as isize) as
                *mut artifact_type;
        /* Need full knowledge or spoilers */
        if spoil as libc::c_int != 0 ||
               (*o_ptr).ident as libc::c_int & 0x20 as libc::c_int != 0 {
            *f1 = (*a_ptr).flags1;
            *f2 = (*a_ptr).flags2;
            *f3 = (*a_ptr).flags3;
            *f4 = (*a_ptr).flags4;
            *f5 = (*a_ptr).flags5;
            *esp = (*a_ptr).esp;
            if object_flags_no_set == 0 &&
                   (*a_ptr).set as libc::c_int != -(1 as libc::c_int) {
                apply_flags_set((*o_ptr).name1 as s16b, (*a_ptr).set, f1, f2,
                                f3, f4, f5, esp);
            }
        } else {
            *f5 = 0 as libc::c_long as u32b;
            *esp = *f5;
            *f4 = *esp;
            *f3 = *f4;
            *f2 = *f3;
            *f1 = *f2
        }
        *f1 |= (*a_ptr).oflags1;
        *f2 |= (*a_ptr).oflags2;
        *f3 |= (*a_ptr).oflags3;
        *f4 |= (*a_ptr).oflags4;
        *f5 |= (*a_ptr).oflags5;
        *esp |= (*a_ptr).oesp
    }
    /* Random artifact or ego item! */
    if (*o_ptr).art_flags1 != 0 || (*o_ptr).art_flags2 != 0 ||
           (*o_ptr).art_flags3 != 0 || (*o_ptr).art_flags4 != 0 ||
           (*o_ptr).art_flags5 != 0 || (*o_ptr).art_esp != 0 {
        /* Need full knowledge or spoilers */
        if spoil as libc::c_int != 0 ||
               (*o_ptr).ident as libc::c_int & 0x20 as libc::c_int != 0 {
            *f1 |= (*o_ptr).art_flags1;
            *f2 |= (*o_ptr).art_flags2;
            *f3 |= (*o_ptr).art_flags3;
            *f4 |= (*o_ptr).art_flags4;
            *f5 |= (*o_ptr).art_flags5;
            *esp |= (*o_ptr).art_esp
        }
        *f1 |= (*o_ptr).art_oflags1;
        *f2 |= (*o_ptr).art_oflags2;
        *f3 |= (*o_ptr).art_oflags3;
        *f4 |= (*o_ptr).art_oflags4;
        *f5 |= (*o_ptr).art_oflags5;
        *esp |= (*o_ptr).art_oesp
    }
    /* Full knowledge for *identified* objects */
    if (*o_ptr).ident as libc::c_int & 0x20 as libc::c_int == 0 { return }
    if (*o_ptr).art_name == 0 { object_flags_xtra(o_ptr, f2, f3, esp); }
    /* Hack - Res Chaos -> Res Confusion */
    if *f2 as libc::c_long & 0x40000000 as libc::c_long != 0 {
        *f2 = (*f2 as libc::c_long | 0x2000000 as libc::c_long) as u32b
    };
}
/*
 * Print a char "c" into a string "t", as if by sprintf(t, "%c", c),
 * and return a pointer to the terminator (t + 1).
 */
unsafe extern "C" fn object_desc_chr(mut t: *mut libc::c_char,
                                     mut c: libc::c_char)
 -> *mut libc::c_char {
    /* Copy the char */
    let fresh10 = t;
    t = t.offset(1);
    *fresh10 = c;
    /* Terminate */
    *t = '\u{0}' as i32 as libc::c_char;
    /* Result */
    return t;
}
/*
 * Print a string "s" into a string "t", as if by strcpy(t, s),
 * and return a pointer to the terminator.
 */
unsafe extern "C" fn object_desc_str(mut t: *mut libc::c_char, mut s: cptr)
 -> *mut libc::c_char {
    /* Copy the string */
    while *s != 0 {
        let fresh11 = s;
        s = s.offset(1);
        let fresh12 = t;
        t = t.offset(1);
        *fresh12 = *fresh11
    }
    /* Terminate */
    *t = '\u{0}' as i32 as libc::c_char;
    /* Result */
    return t;
}
/*
 * Do the actual conversion of a number for object_desc_num() and
 * object_desc_int().
 */
unsafe extern "C" fn convert_number(mut result: *mut libc::c_char,
                                    mut num: u32b) -> *mut libc::c_char {
    let mut tp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp: [libc::c_char; 11] = [0; 11];
    tp = temp.as_mut_ptr();
    *tp =
        ('0' as i32 as
             libc::c_uint).wrapping_add(num.wrapping_rem(10 as libc::c_int as
                                                             libc::c_uint)) as
            libc::c_char;
    num =
        (num as libc::c_uint).wrapping_div(10 as libc::c_int as libc::c_uint)
            as u32b as u32b;
    while num != 0 as libc::c_int as libc::c_uint {
        tp = tp.offset(1);
        *tp =
            ('0' as i32 as
                 libc::c_uint).wrapping_add(num.wrapping_rem(10 as libc::c_int
                                                                 as
                                                                 libc::c_uint))
                as libc::c_char;
        num =
            (num as
                 libc::c_uint).wrapping_div(10 as libc::c_int as libc::c_uint)
                as u32b as u32b
    }
    while tp != temp.as_mut_ptr() {
        let fresh13 = tp;
        tp = tp.offset(-1);
        let fresh14 = result;
        result = result.offset(1);
        *fresh14 = *fresh13
    }
    let fresh15 = result;
    result = result.offset(1);
    *fresh15 = *tp;
    *result = '\u{0}' as i32 as libc::c_char;
    return result;
}
/*
 * Print a nnumber "n" into a string "t", as if by
 * sprintf(t, "%u", n), and return a pointer to the terminator.
 */
unsafe extern "C" fn object_desc_num(mut result: *mut libc::c_char,
                                     mut num: s32b) -> *mut libc::c_char {
    let mut n: u32b = 0;
    if num < 0 as libc::c_int {
        let fresh16 = result;
        result = result.offset(1);
        *fresh16 = '-' as i32 as libc::c_char;
        n = -num as u32b
    } else { n = num as u32b }
    /* Result */
    return convert_number(result, n);
}
/*
 * Print an signed number "num" into a string "result", as if by
 * sprintf(t, "%+d", n), and return a pointer to the terminator.
 * Note that we always print a sign, either "+" or "-".
 */
unsafe extern "C" fn object_desc_int(mut result: *mut libc::c_char,
                                     mut num: s32b) -> *mut libc::c_char {
    let mut n: u32b = 0;
    /* Negative */
    if num < 0 as libc::c_int {
        /* Take the absolute value */
        n = -num as u32b;
        /* Use a "minus" sign */
        let fresh17 = result;
        result = result.offset(1);
        *fresh17 = '-' as i32 as libc::c_char
    } else {
        /* Positive (or zero) */
        /* Use the actual number */
        n = num as u32b;
        let fresh18 = result;
        result = result.offset(1);
        *fresh18 = '+' as i32 as libc::c_char
    }
    /* Use a "plus" sign */
    /* Result */
    return convert_number(result, n);
}
/*
 * Creates a description of the item "o_ptr", and stores it in "out_val".
 *
 * One can choose the "verbosity" of the description, including whether
 * or not the "number" of items should be described, and how much detail
 * should be used when describing the item.
 *
 * The given "buf" must be 80 chars long to hold the longest possible
 * description, which can get pretty long, including incriptions, such as:
 * "no more Maces of Disruption (Defender) (+10,+10) [+5] (+3 to stealth)".
 * Note that the inscription will be clipped to keep the total description
 * under 79 chars (plus a terminator).
 *
 * Note the use of "object_desc_num()" and "object_desc_int()" as hyper-efficient,
 * portable, versions of some common "sprintf()" commands.
 *
 * Note that all ego-items (when known) append an "Ego-Item Name", unless
 * the item is also an artifact, which should NEVER happen.
 *
 * Note that all artifacts (when known) append an "Artifact Name", so we
 * have special processing for "Specials" (artifact Lites, Rings, Amulets).
 * The "Specials" never use "modifiers" if they are "known", since they
 * have special "descriptions", such as "The Necklace of the Dwarves".
 *
 * Special Lite's use the "k_info" base-name (Phial, Star, or Arkenstone),
 * plus the artifact name, just like any other artifact, if known.
 *
 * Special Ring's and Amulet's, if not "aware", use the same code as normal
 * rings and amulets, and if "aware", use the "k_info" base-name (Ring or
 * Amulet or Necklace).  They will NEVER "append" the "k_info" name.  But,
 * they will append the artifact name, just like any artifact, if known.
 *
 * None of the Special Rings/Amulets are "EASY_KNOW", though they could be,
 * at least, those which have no "pluses", such as the three artifact lites.
 *
 * Hack -- Display "The One Ring" as "a Plain Gold Ring" until aware.
 *
 * If "pref" then a "numeric" prefix will be pre-pended.
 *
 * Mode:
 *   0 -- The Cloak of Death
 *   1 -- The Cloak of Death [1,+3]
 *   2 -- The Cloak of Death [1,+3] (+2 to Stealth)
 *   3 -- The Cloak of Death [1,+3] (+2 to Stealth) {nifty}
 */
#[no_mangle]
pub unsafe extern "C" fn object_desc(mut buf: *mut libc::c_char,
                                     mut o_ptr: *mut object_type,
                                     mut pref: libc::c_int,
                                     mut mode: libc::c_int) {
    let mut hack_name: bool_ = 0 as libc::c_int as bool_;
    let mut basenm: cptr = 0 as *const libc::c_char;
    let mut modstr: cptr = 0 as *const libc::c_char;
    let mut indexx: libc::c_int = 0;
    let mut aware: bool_ = 0 as libc::c_int as bool_;
    let mut known: bool_ = 0 as libc::c_int as bool_;
    let mut append_name: bool_ = 0 as libc::c_int as bool_;
    let mut show_weapon: bool_ = 0 as libc::c_int as bool_;
    let mut show_armour: bool_ = 0 as libc::c_int as bool_;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut u: cptr = 0 as *const libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p1: libc::c_char = '(' as i32 as libc::c_char;
    let mut p2: libc::c_char = ')' as i32 as libc::c_char;
    let mut b1: libc::c_char = '[' as i32 as libc::c_char;
    let mut b2: libc::c_char = ']' as i32 as libc::c_char;
    let mut c1: libc::c_char = '{' as i32 as libc::c_char;
    let mut c2: libc::c_char = '}' as i32 as libc::c_char;
    let mut tmp_val: [libc::c_char; 160] = [0; 160];
    let mut tmp_val2: [libc::c_char; 90] = [0; 90];
    let mut power: s32b = 0;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset((*o_ptr).k_idx as isize) as *mut object_kind;
    let mut str: cptr = 0 as *const libc::c_char;
    /* Extract some flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* See if the object is "aware" */
    if (*k_info.offset((*o_ptr).k_idx as isize)).aware != 0 {
        aware = 1 as libc::c_int as bool_
    }
    /* See if the object is "known" */
    if (*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
           (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as libc::c_int
               != 0 &&
               (*k_info.offset((*o_ptr).k_idx as isize)).aware as libc::c_int
                   != 0 {
        known = 1 as libc::c_int as bool_
    }
    /* Hack -- Extract the sub-type "indexx" */
    indexx = (*o_ptr).sval as libc::c_int;
    /* Extract default "base" string */
    basenm = k_name.offset((*k_ptr).name as isize) as cptr;
    /* Assume no "modifier" string */
    modstr = b"\x00" as *const u8 as *const libc::c_char;
    /* Analyze the object */
    match (*o_ptr).tval as libc::c_int {
        16 | 18 | 17 | 15 | 19 | 21 | 22 | 6 | 23 | 24 => {
            /* Missiles/ Bows/ Weapons */
            show_weapon = 1 as libc::c_int as bool_
        }
        46 => {
            /* Trapping Kits */
            modstr = basenm;
            basenm = b"& # Trap Set~\x00" as *const u8 as *const libc::c_char
        }
        30 | 31 | 33 | 32 | 34 | 36 | 37 | 38 => {
            /* Armour */
            show_armour = 1 as libc::c_int as bool_
        }
        1 | 2 | 11 | 5 | 77 | 7 | 14 | 12 | 20 | 39 => { }
        40 => {
            /* Amulets (including a few "Specials") */
            /* Color the object */
            modstr = amulet_adj[indexx as usize];
            if aware != 0 { append_name = 1 as libc::c_int as bool_ }
            if plain_descriptions as libc::c_int != 0 &&
                   aware as libc::c_int != 0 ||
                   (*o_ptr).ident as libc::c_int & 0x10 as libc::c_int != 0 {
                basenm = b"& Amulet~\x00" as *const u8 as *const libc::c_char
            } else {
                basenm =
                    if aware as libc::c_int != 0 {
                        b"& # Amulet~\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"& # Amulet~\x00" as *const u8 as *const libc::c_char
                    }
            }
            if known as libc::c_int != 0 && (*o_ptr).art_name == 0 &&
                   ((*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
                        (if (*o_ptr).name1 as libc::c_int != 0 {
                             1 as libc::c_int
                         } else { 0 as libc::c_int }) != 0 ||
                        (if (*o_ptr).art_name as libc::c_int != 0 {
                             1 as libc::c_int
                         } else { 0 as libc::c_int }) != 0 ||
                        (if (*k_info.offset((*o_ptr).k_idx as isize)).flags3
                                as libc::c_long & 0x8000 as libc::c_long != 0
                            {
                             1 as libc::c_int
                         } else { 0 as libc::c_int }) != 0) {
                basenm = k_name.offset((*k_ptr).name as isize) as cptr
            }
        }
        45 => {
            /* Rings (including a few "Specials") */
            /* Color the object */
            modstr = ring_adj[indexx as usize];
            if aware != 0 { append_name = 1 as libc::c_int as bool_ }
            if plain_descriptions as libc::c_int != 0 &&
                   aware as libc::c_int != 0 ||
                   (*o_ptr).ident as libc::c_int & 0x10 as libc::c_int != 0 {
                basenm = b"& Ring~\x00" as *const u8 as *const libc::c_char
            } else {
                basenm =
                    if aware as libc::c_int != 0 {
                        b"& # Ring~\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"& # Ring~\x00" as *const u8 as *const libc::c_char
                    }
            }
            /* Hack -- The One Ring */
            if aware == 0 && (*o_ptr).sval as libc::c_int == 37 as libc::c_int
               {
                modstr = b"Plain Gold\x00" as *const u8 as *const libc::c_char
            }
            if known as libc::c_int != 0 && (*o_ptr).art_name == 0 &&
                   ((*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
                        (if (*o_ptr).name1 as libc::c_int != 0 {
                             1 as libc::c_int
                         } else { 0 as libc::c_int }) != 0 ||
                        (if (*o_ptr).art_name as libc::c_int != 0 {
                             1 as libc::c_int
                         } else { 0 as libc::c_int }) != 0 ||
                        (if (*k_info.offset((*o_ptr).k_idx as isize)).flags3
                                as libc::c_long & 0x8000 as libc::c_long != 0
                            {
                             1 as libc::c_int
                         } else { 0 as libc::c_int }) != 0) {
                basenm = k_name.offset((*k_ptr).name as isize) as cptr
            }
        }
        55 => {
            /* Color the object */
            modstr =
                staff_adj[((*o_ptr).pval2 as libc::c_int % 35 as libc::c_int)
                              as usize];
            if aware != 0 { append_name = 1 as libc::c_int as bool_ }
            if plain_descriptions as libc::c_int != 0 &&
                   aware as libc::c_int != 0 ||
                   (*o_ptr).ident as libc::c_int & 0x10 as libc::c_int != 0 {
                basenm = b"& Staff~\x00" as *const u8 as *const libc::c_char
            } else {
                basenm = b"& # Staff~\x00" as *const u8 as *const libc::c_char
            }
        }
        65 => {
            /* Color the object */
            modstr =
                wand_adj[((*o_ptr).pval2 as libc::c_int % 39 as libc::c_int)
                             as usize];
            if aware != 0 { append_name = 1 as libc::c_int as bool_ }
            if plain_descriptions as libc::c_int != 0 &&
                   aware as libc::c_int != 0 ||
                   (*o_ptr).ident as libc::c_int & 0x10 as libc::c_int != 0 {
                basenm = b"& Wand~\x00" as *const u8 as *const libc::c_char
            } else {
                basenm = b"& # Wand~\x00" as *const u8 as *const libc::c_char
            }
        }
        66 => {
            /* Color the object */
            modstr = rod_adj[indexx as usize];
            if aware != 0 { append_name = 1 as libc::c_int as bool_ }
            if plain_descriptions as libc::c_int != 0 &&
                   aware as libc::c_int != 0 ||
                   (*o_ptr).ident as libc::c_int & 0x10 as libc::c_int != 0 {
                basenm = b"& Rod Tip~\x00" as *const u8 as *const libc::c_char
            } else {
                basenm =
                    if aware as libc::c_int != 0 {
                        b"& # Rod Tip~\x00" as *const u8 as
                            *const libc::c_char
                    } else {
                        b"& # Rod Tip~\x00" as *const u8 as
                            *const libc::c_char
                    }
            }
            if (*o_ptr).sval as libc::c_int == 30 as libc::c_int {
                basenm =
                    b"& Great Rod Tip~ of Home Summoning\x00" as *const u8 as
                        *const libc::c_char;
                hack_name = 1 as libc::c_int as bool_
            }
        }
        67 => {
            let mut tip_ptr: *mut object_kind =
                &mut *k_info.offset((lookup_kind as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _: libc::c_int)
                                             ->
                                                 s16b)(66 as libc::c_int,
                                                       (*o_ptr).pval) as
                                        isize) as *mut object_kind;
            modstr = k_name.offset((*tip_ptr).name as isize) as cptr
        }
        70 => {
            /* Color the object */
            modstr = scroll_adj[indexx as usize].as_mut_ptr() as cptr;
            if aware != 0 { append_name = 1 as libc::c_int as bool_ }
            if plain_descriptions as libc::c_int != 0 &&
                   aware as libc::c_int != 0 ||
                   (*o_ptr).ident as libc::c_int & 0x10 as libc::c_int != 0 {
                basenm = b"& Scroll~\x00" as *const u8 as *const libc::c_char
            } else {
                basenm =
                    if aware as libc::c_int != 0 {
                        b"& Scroll~ titled \"#\"\x00" as *const u8 as
                            *const libc::c_char
                    } else {
                        b"& Scroll~ titled \"#\"\x00" as *const u8 as
                            *const libc::c_char
                    }
            }
        }
        71 | 72 => {
            /* Color the object */
            if (*o_ptr).tval as libc::c_int != 72 as libc::c_int ||
                   (*o_ptr).sval as libc::c_int != 1 as libc::c_int ||
                   aware == 0 {
                modstr = potion_adj[indexx as usize];
                if aware != 0 { append_name = 1 as libc::c_int as bool_ }
            } else {
                call_lua(b"get_mimic_info\x00" as *const u8 as
                             *const libc::c_char,
                         b"(d,s)\x00" as *const u8 as *const libc::c_char,
                         b"s\x00" as *const u8 as *const libc::c_char,
                         (*o_ptr).pval2 as libc::c_int,
                         b"name\x00" as *const u8 as *const libc::c_char,
                         &mut modstr as *mut cptr);
            }
            if plain_descriptions as libc::c_int != 0 &&
                   aware as libc::c_int != 0 ||
                   (*o_ptr).ident as libc::c_int & 0x10 as libc::c_int != 0 {
                basenm = b"& Potion~\x00" as *const u8 as *const libc::c_char
            } else {
                basenm =
                    if aware as libc::c_int != 0 {
                        b"& # Potion~\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"& # Potion~\x00" as *const u8 as *const libc::c_char
                    }
            }
        }
        80 => {
            /* Ordinary food is "boring" */
            if !((*o_ptr).sval as libc::c_int >= 32 as libc::c_int) {
                /* Color the object */
                modstr = food_adj[indexx as usize];
                if aware != 0 { append_name = 1 as libc::c_int as bool_ }
                if plain_descriptions as libc::c_int != 0 &&
                       aware as libc::c_int != 0 ||
                       (*o_ptr).ident as libc::c_int & 0x10 as libc::c_int !=
                           0 {
                    basenm =
                        b"& Mushroom~\x00" as *const u8 as *const libc::c_char
                } else {
                    basenm =
                        if aware as libc::c_int != 0 {
                            b"& # Mushroom~\x00" as *const u8 as
                                *const libc::c_char
                        } else {
                            b"& # Mushroom~\x00" as *const u8 as
                                *const libc::c_char
                        }
                }
            }
        }
        35 => {
            /* Cloak of Mimicry */
            show_armour = 1 as libc::c_int as bool_;
            if (*o_ptr).sval as libc::c_int == 100 as libc::c_int {
                call_lua(b"get_mimic_info\x00" as *const u8 as
                             *const libc::c_char,
                         b"(d,s)\x00" as *const u8 as *const libc::c_char,
                         b"s\x00" as *const u8 as *const libc::c_char,
                         (*o_ptr).pval2 as libc::c_int,
                         b"obj_name\x00" as *const u8 as *const libc::c_char,
                         &mut modstr as *mut cptr);
            }
        }
        112 => {
            modstr = basenm;
            basenm =
                b"& Symbiotic Spellbook~ #\x00" as *const u8 as
                    *const libc::c_char
        }
        113 => {
            modstr = basenm;
            basenm = b"& Songbook~ #\x00" as *const u8 as *const libc::c_char
        }
        114 => {
            /* Druid Books */
            modstr = basenm;
            basenm =
                b"& Elemental Stone~ #\x00" as *const u8 as
                    *const libc::c_char
        }
        4 => {
            modstr = basenm;
            basenm =
                b"& Essence~ of #\x00" as *const u8 as *const libc::c_char
        }
        8 => {
            modstr = basenm;
            basenm =
                b"& Parchment~ - #\x00" as *const u8 as *const libc::c_char
        }
        100 => {
            /* Hack -- Gold/Gems */
            strcpy(buf, basenm);
            return
        }
        9 => {
            let mut r_ptr: *mut monster_race =
                &mut *r_info.offset((*o_ptr).pval2 as isize) as
                    *mut monster_race;
            modstr = basenm;
            if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
                basenm =
                    format(b"& %s\'s #~\x00" as *const u8 as
                               *const libc::c_char,
                           r_name.offset((*r_ptr).name as isize)) as cptr
            } else {
                basenm =
                    format(b"& %s #~\x00" as *const u8 as *const libc::c_char,
                           r_name.offset((*r_ptr).name as isize)) as cptr
            }
        }
        10 => {
            let mut r_ptr_0: *mut monster_race =
                &mut *r_info.offset((*o_ptr).pval2 as isize) as
                    *mut monster_race;
            modstr = basenm;
            basenm =
                format(b"& %s #~\x00" as *const u8 as *const libc::c_char,
                       r_name.offset((*r_ptr_0).name as isize)) as cptr
        }
        99 => {
            /* We print hit points further down. --dsb */
            let mut r_ptr_1: *mut monster_race =
                &mut *r_info.offset((*o_ptr).pval as isize) as
                    *mut monster_race;
            modstr = basenm;
            basenm =
                format(b"& %s~\x00" as *const u8 as *const libc::c_char,
                       r_name.offset((*r_ptr_1).name as isize)) as cptr
        }
        54 => {
            let mut name: [libc::c_char; 80] = [0; 80];
            let mut monster: monster_type =
                monster_type{r_idx: 0,
                             ego: 0,
                             fy: 0,
                             fx: 0,
                             hp: 0,
                             maxhp: 0,
                             blow:
                                 [monster_blow{method: 0,
                                               effect: 0,
                                               d_dice: 0,
                                               d_side: 0,}; 4],
                             speed: 0,
                             level: 0,
                             ac: 0,
                             exp: 0,
                             csleep: 0,
                             mspeed: 0,
                             energy: 0,
                             stunned: 0,
                             confused: 0,
                             monfear: 0,
                             bleeding: 0,
                             poisoned: 0,
                             cdis: 0,
                             mflag: 0,
                             ml: 0,
                             hold_o_idx: 0,
                             smart: 0,
                             status: 0,
                             target: 0,
                             possessor: 0,
                             sr_ptr: 0 as *mut monster_race,
                             mind: 0 as *mut monster_mind,};
            monster.sr_ptr = 0 as *mut monster_race;
            monster.r_idx = (*o_ptr).pval as s16b;
            monster.ego = (*o_ptr).pval2 as u16b;
            monster.ml = 1 as libc::c_int as bool_;
            monster.status = -(2 as libc::c_int) as s16b;
            monster_desc(name.as_mut_ptr(), &mut monster,
                         0x188 as libc::c_int);
            modstr = basenm;
            basenm =
                format(b"& #~ of %s\x00" as *const u8 as *const libc::c_char,
                       name.as_mut_ptr()) as cptr
        }
        102 => {
            modstr = basenm;
            if known != 0 {
                basenm =
                    random_artifacts[indexx as usize].name_full.as_mut_ptr()
                        as cptr
            } else {
                basenm =
                    random_artifacts[indexx as usize].name_short.as_mut_ptr()
                        as cptr
            }
        }
        105 => {
            if (*o_ptr).sval as libc::c_int != 0xff as libc::c_int {
                modstr = basenm;
                basenm =
                    b"& Rune~ [#]\x00" as *const u8 as *const libc::c_char
            }
        }
        104 => {
            modstr = basenm;
            basenm = b"& Rune~ [#]\x00" as *const u8 as *const libc::c_char
        }
        115 | 111 => {
            basenm = k_name.offset((*k_ptr).name as isize) as cptr;
            if (*o_ptr).sval as libc::c_int == 255 as libc::c_int {
                modstr = (*school_spells.offset((*o_ptr).pval as isize)).name
            }
        }
        _ => {
            /* Used in the "inventory" routine */
            if process_hooks_ret(39 as libc::c_int,
                                 b"ss\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char,
                                 b"(O,s,s)\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 o_ptr, basenm, modstr) != 0 {
                basenm =
                    process_hooks_return[0 as libc::c_int as usize].str_0;
                modstr = process_hooks_return[1 as libc::c_int as usize].str_0
            } else {
                strcpy(buf,
                       b"(nothing)\x00" as *const u8 as *const libc::c_char);
                return
            }
        }
    }
    /* Mega Hack */
    if hack_name == 0 && known as libc::c_int != 0 &&
           (*k_ptr).flags5 as libc::c_long & 0x100 as libc::c_long != 0 {
        basenm = k_name.offset((*k_ptr).name as isize) as cptr
    }
    /* Start dumping the result */
    t = tmp_val.as_mut_ptr();
    /* The object "expects" a "number" */
    if *basenm.offset(0 as libc::c_int as isize) as libc::c_int == '&' as i32
       {
        let mut r_ptr_2: *mut monster_race = 0 as *mut monster_race;
        let mut ego: cptr = 0 as cptr;
        if (*o_ptr).tval as libc::c_int == 9 as libc::c_int {
            r_ptr_2 =
                &mut *r_info.offset((*o_ptr).pval2 as isize) as
                    *mut monster_race
        } else {
            r_ptr_2 =
                &mut *r_info.offset((*o_ptr).pval as isize) as
                    *mut monster_race
        }
        /* Grab any ego-item name */
        if known as libc::c_int != 0 &&
               ((*o_ptr).name2 as libc::c_int != 0 ||
                    (*o_ptr).name2b as libc::c_int != 0) &&
               (*o_ptr).tval as libc::c_int != 67 as libc::c_int {
            let mut e_ptr: *mut ego_item_type =
                &mut *e_info.offset((*o_ptr).name2 as isize) as
                    *mut ego_item_type;
            let mut e2_ptr: *mut ego_item_type =
                &mut *e_info.offset((*o_ptr).name2b as isize) as
                    *mut ego_item_type;
            if (*e_ptr).before != 0 {
                ego = e_name.offset((*e_ptr).name as isize) as cptr
            } else if (*e2_ptr).before != 0 {
                ego = e_name.offset((*e2_ptr).name as isize) as cptr
            }
        }
        /* Skip the ampersand (and space) */
        s = basenm.offset(2 as libc::c_int as isize);
        /* No prefix */
        if !(pref <= 0 as libc::c_int) {
            /* Hack -- None left */
            if (*o_ptr).number as libc::c_int <= 0 as libc::c_int {
                t =
                    object_desc_str(t,
                                    b"no more \x00" as *const u8 as
                                        *const libc::c_char)
            } else if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                t = object_desc_num(t, (*o_ptr).number as s32b);
                t = object_desc_chr(t, ' ' as i32 as libc::c_char)
            } else if !((*o_ptr).tval as libc::c_int == 9 as libc::c_int &&
                            (*r_ptr_2).flags1 &
                                0x1 as libc::c_int as libc::c_uint != 0) {
                if !((*o_ptr).tval as libc::c_int == 99 as libc::c_int &&
                         (*r_ptr_2).flags1 &
                             0x1 as libc::c_int as libc::c_uint != 0) {
                    /* Extract the number */
                    /* Hack -- The only one of its kind */
                    if known as libc::c_int != 0 &&
                           ((*o_ptr).tval as libc::c_int == 102 as libc::c_int
                                ||
                                (if (*o_ptr).name1 as libc::c_int != 0 {
                                     1 as libc::c_int
                                 } else { 0 as libc::c_int }) != 0 ||
                                (if (*o_ptr).art_name as libc::c_int != 0 {
                                     1 as libc::c_int
                                 } else { 0 as libc::c_int }) != 0 ||
                                (if (*k_info.offset((*o_ptr).k_idx as
                                                        isize)).flags3 as
                                        libc::c_long & 0x8000 as libc::c_long
                                        != 0 {
                                     1 as libc::c_int
                                 } else { 0 as libc::c_int }) != 0 ||
                                (*o_ptr).art_name as libc::c_int != 0) {
                        t =
                            object_desc_str(t,
                                            b"The \x00" as *const u8 as
                                                *const libc::c_char)
                    } else if !ego.is_null() {
                        if is_a_vowel(*ego.offset(0 as libc::c_int as isize)
                                          as libc::c_int) != 0 {
                            t =
                                object_desc_str(t,
                                                b"an \x00" as *const u8 as
                                                    *const libc::c_char)
                        } else {
                            t =
                                object_desc_str(t,
                                                b"a \x00" as *const u8 as
                                                    *const libc::c_char)
                        }
                    } else if *s as libc::c_int == '#' as i32 &&
                                  is_a_vowel(*modstr.offset(0 as libc::c_int
                                                                as isize) as
                                                 libc::c_int) as libc::c_int
                                      != 0 {
                        t =
                            object_desc_str(t,
                                            b"an \x00" as *const u8 as
                                                *const libc::c_char)
                    } else if is_a_vowel(*s as libc::c_int) != 0 {
                        t =
                            object_desc_str(t,
                                            b"an \x00" as *const u8 as
                                                *const libc::c_char)
                    } else {
                        /* A single one, with a vowel in the modifier */
                        /* A single one, with a vowel */
                        /* A single one, without a vowel */
                        t =
                            object_desc_str(t,
                                            b"a \x00" as *const u8 as
                                                *const libc::c_char)
                    }
                }
            }
        }
        /* Grab any ego-item name */
        if known as libc::c_int != 0 &&
               ((*o_ptr).name2 as libc::c_int != 0 ||
                    (*o_ptr).name2b as libc::c_int != 0) &&
               (*o_ptr).tval as libc::c_int != 67 as libc::c_int {
            let mut e_ptr_0: *mut ego_item_type =
                &mut *e_info.offset((*o_ptr).name2 as isize) as
                    *mut ego_item_type;
            let mut e2_ptr_0: *mut ego_item_type =
                &mut *e_info.offset((*o_ptr).name2b as isize) as
                    *mut ego_item_type;
            if (*e_ptr_0).before != 0 {
                t =
                    object_desc_str(t,
                                    e_name.offset((*e_ptr_0).name as isize) as
                                        cptr);
                t = object_desc_chr(t, ' ' as i32 as libc::c_char)
            }
            if (*e2_ptr_0).before != 0 {
                t =
                    object_desc_str(t,
                                    e_name.offset((*e2_ptr_0).name as isize)
                                        as cptr);
                t = object_desc_chr(t, ' ' as i32 as libc::c_char)
            }
        }
        /* -TM- Hack -- Add false-artifact names */
		/* Dagger inscribed {@w0%Smelly} will be named
		 * Smelly Dagger {@w0} */
        if (*o_ptr).note != 0 {
            str =
                strchr(quark_str((*o_ptr).note as s16b), '%' as i32) as cptr;
            /* Add the false name */
            if !str.is_null() {
                t =
                    object_desc_str(t,
                                    &*str.offset(1 as libc::c_int as isize));
                t = object_desc_chr(t, ' ' as i32 as libc::c_char)
            }
        }
    } else {
        /* Hack -- objects that "never" take an article */
        /* No ampersand */
        s = basenm;
        if !(pref == 0) {
            /* Hack -- all gone */
            if (*o_ptr).number as libc::c_int <= 0 as libc::c_int {
                t =
                    object_desc_str(t,
                                    b"no more \x00" as *const u8 as
                                        *const libc::c_char)
            } else if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                t = object_desc_num(t, (*o_ptr).number as s32b);
                t = object_desc_chr(t, ' ' as i32 as libc::c_char)
            } else if !((*o_ptr).tval as libc::c_int == 102 as libc::c_int) {
                /* Prefix a number if required */
                /* Hack -- The only one of its kind */
                if known as libc::c_int != 0 &&
                       ((*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
                            (if (*o_ptr).name1 as libc::c_int != 0 {
                                 1 as libc::c_int
                             } else { 0 as libc::c_int }) != 0 ||
                            (if (*o_ptr).art_name as libc::c_int != 0 {
                                 1 as libc::c_int
                             } else { 0 as libc::c_int }) != 0 ||
                            (if (*k_info.offset((*o_ptr).k_idx as
                                                    isize)).flags3 as
                                    libc::c_long & 0x8000 as libc::c_long != 0
                                {
                                 1 as libc::c_int
                             } else { 0 as libc::c_int }) != 0 ||
                            (*o_ptr).art_name as libc::c_int != 0) {
                    t =
                        object_desc_str(t,
                                        b"The \x00" as *const u8 as
                                            *const libc::c_char)
                }
            }
        }
        if known as libc::c_int != 0 &&
               ((*o_ptr).name2 as libc::c_int != 0 ||
                    (*o_ptr).name2b as libc::c_int != 0) &&
               (*o_ptr).tval as libc::c_int != 67 as libc::c_int {
            let mut e_ptr_1: *mut ego_item_type =
                &mut *e_info.offset((*o_ptr).name2 as isize) as
                    *mut ego_item_type;
            let mut e2_ptr_1: *mut ego_item_type =
                &mut *e_info.offset((*o_ptr).name2b as isize) as
                    *mut ego_item_type;
            if (*e_ptr_1).before != 0 {
                t =
                    object_desc_str(t,
                                    e_name.offset((*e_ptr_1).name as isize) as
                                        cptr);
                t = object_desc_chr(t, ' ' as i32 as libc::c_char)
            }
            if (*e2_ptr_1).before != 0 {
                t =
                    object_desc_str(t,
                                    e_name.offset((*e2_ptr_1).name as isize)
                                        as cptr);
                t = object_desc_chr(t, ' ' as i32 as libc::c_char)
            }
        }
    }
    /* No pref */
    /* Grab any ego-item name */
    /* Paranoia -- skip illegal tildes */
	/* while (*s == '~') s++; */
    /* Copy the string */
    while *s != 0 {
        /* Pluralizer */
        if *s as libc::c_int == '~' as i32 {
            /* Add a plural if needed */
            if (*o_ptr).number as libc::c_int != 1 as libc::c_int &&
                   pref >= 0 as libc::c_int {
                let mut k: libc::c_char =
                    *t.offset(-(1 as libc::c_int) as isize);
                /* XXX XXX XXX Mega-Hack */
                /* Hack -- "Cutlass-es" and "Torch-es" */
                if k as libc::c_int == 's' as i32 ||
                       k as libc::c_int == 'h' as i32 {
                    let fresh19 = t;
                    t = t.offset(1);
                    *fresh19 = 'e' as i32 as libc::c_char
                }
                /* Add an 's' */
                let fresh20 = t;
                t = t.offset(1);
                *fresh20 = 's' as i32 as libc::c_char
            }
        } else if *s as libc::c_int == '#' as i32 {
            /* Modifier */
            /* Grab any ego-item name */
            if (*o_ptr).tval as libc::c_int == 67 as libc::c_int {
                t = object_desc_chr(t, ' ' as i32 as libc::c_char);
                if known as libc::c_int != 0 &&
                       (*o_ptr).name2 as libc::c_int != 0 {
                    let mut e_ptr_2: *mut ego_item_type =
                        &mut *e_info.offset((*o_ptr).name2 as isize) as
                            *mut ego_item_type;
                    t =
                        object_desc_str(t,
                                        e_name.offset((*e_ptr_2).name as
                                                          isize) as cptr)
                }
            }
            /* Insert the modifier */
            u = modstr;
            while *u != 0 {
                let fresh21 = t;
                t = t.offset(1);
                *fresh21 = *u;
                u = u.offset(1)
            }
        } else {
            /* Normal */
            /* Copy */
            let fresh22 = t;
            t = t.offset(1);
            *fresh22 = *s
        }
        s = s.offset(1)
    }
    /* Terminate */
    *t = '\u{0}' as i32 as libc::c_char;
    /* Append the "kind name" to the "base name" */
    if append_name as libc::c_int != 0 &&
           !((*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
                 (if (*o_ptr).name1 as libc::c_int != 0 {
                      1 as libc::c_int
                  } else { 0 as libc::c_int }) != 0 ||
                 (if (*o_ptr).art_name as libc::c_int != 0 {
                      1 as libc::c_int
                  } else { 0 as libc::c_int }) != 0 ||
                 (if (*k_info.offset((*o_ptr).k_idx as isize)).flags3 as
                         libc::c_long & 0x8000 as libc::c_long != 0 {
                      1 as libc::c_int
                  } else { 0 as libc::c_int }) != 0) {
        t =
            object_desc_str(t,
                            b" of \x00" as *const u8 as *const libc::c_char);
        if (*o_ptr).tval as libc::c_int == 65 as libc::c_int ||
               (*o_ptr).tval as libc::c_int == 55 as libc::c_int {
            t =
                object_desc_str(t,
                                (*school_spells.offset((*o_ptr).pval2 as
                                                           isize)).name);
            if mode >= 1 as libc::c_int {
                let mut bonus: s32b = (*o_ptr).pval3 & 0xffff as libc::c_int;
                let mut max: s32b = (*o_ptr).pval3 >> 16 as libc::c_int;
                t = object_desc_chr(t, '[' as i32 as libc::c_char);
                t = object_desc_num(t, bonus);
                t = object_desc_chr(t, '|' as i32 as libc::c_char);
                t = object_desc_num(t, max);
                t = object_desc_chr(t, ']' as i32 as libc::c_char)
            }
        } else {
            t =
                object_desc_str(t,
                                k_name.offset((*k_ptr).name as isize) as cptr)
        }
    }
    /* Hack -- Append "Artifact" or "Special" names */
    if known != 0 {
        /* -TM- Hack -- Add false-artifact names */
		/* Dagger inscribed {@w0#of Smell} will be named
		 * Dagger of Smell {@w0} */
        if (*o_ptr).note != 0 {
            str =
                strchr(quark_str((*o_ptr).note as s16b), '#' as i32) as cptr;
            /* Add the false name */
            if !str.is_null() {
                t = object_desc_chr(t, ' ' as i32 as libc::c_char);
                t =
                    object_desc_str(t,
                                    &*str.offset(1 as libc::c_int as isize))
            }
        }
        /* Is it a new random artifact ? */
        if (*o_ptr).art_name != 0 {
            t = object_desc_chr(t, ' ' as i32 as libc::c_char);
            t = object_desc_str(t, quark_str((*o_ptr).art_name as s16b))
        } else if (*o_ptr).name1 != 0 {
            let mut a_ptr: *mut artifact_type =
                &mut *a_info.offset((*o_ptr).name1 as isize) as
                    *mut artifact_type;
            /* Grab any artifact name */
            /* Unique corpses don't require another name */
            if (*o_ptr).tval as libc::c_int != 9 as libc::c_int {
                t = object_desc_chr(t, ' ' as i32 as libc::c_char);
                t =
                    object_desc_str(t,
                                    a_name.offset((*a_ptr).name as isize) as
                                        cptr)
            }
        } else if ((*o_ptr).name2 as libc::c_int != 0 ||
                       (*o_ptr).name2b as libc::c_int != 0) &&
                      (*o_ptr).tval as libc::c_int != 67 as libc::c_int {
            let mut e_ptr_3: *mut ego_item_type =
                &mut *e_info.offset((*o_ptr).name2 as isize) as
                    *mut ego_item_type;
            let mut e2_ptr_2: *mut ego_item_type =
                &mut *e_info.offset((*o_ptr).name2b as isize) as
                    *mut ego_item_type;
            if (*o_ptr).name2 as libc::c_int != 0 && (*e_ptr_3).before == 0 {
                t = object_desc_chr(t, ' ' as i32 as libc::c_char);
                t =
                    object_desc_str(t,
                                    e_name.offset((*e_ptr_3).name as isize) as
                                        cptr)
            }
            if (*o_ptr).name2b as libc::c_int != 0 && (*e2_ptr_2).before == 0
               {
                t = object_desc_chr(t, ' ' as i32 as libc::c_char);
                t =
                    object_desc_str(t,
                                    e_name.offset((*e2_ptr_2).name as isize)
                                        as cptr)
            }
        }
    }
    /* Grab any ego-item name */
    /* It contains a spell */
    if known as libc::c_int != 0 &&
           f5 as libc::c_long & 0x800 as libc::c_long != 0 &&
           (*o_ptr).pval2 as libc::c_int != -(1 as libc::c_int) {
        t =
            object_desc_str(t,
                            format(b" [%s]\x00" as *const u8 as
                                       *const libc::c_char,
                                   (*school_spells.offset((*o_ptr).pval2 as
                                                              isize)).name) as
                                cptr)
    }
    /* Add symbiote hp here, after the "fake-artifact" name. --dsb */
    if (*o_ptr).tval as libc::c_int == 99 as libc::c_int {
        t = object_desc_str(t, b" (\x00" as *const u8 as *const libc::c_char);
        t = object_desc_num(t, (*o_ptr).pval2 as s32b);
        t =
            object_desc_str(t,
                            b" hp)\x00" as *const u8 as *const libc::c_char)
    }
    /* No more details wanted */
    if !(mode < 1 as libc::c_int) {
        /* Hack -- Some objects can have an exp level */
        if f4 as libc::c_long & 0x100 as libc::c_long != 0 &&
               known as libc::c_int != 0 {
            t =
                object_desc_str(t,
                                b" (E:\x00" as *const u8 as
                                    *const libc::c_char);
            if exp_need != 0 {
                let mut need: s32b = 0;
                /* Formula from check_experience_obj(). */
                need =
                    player_exp[((*o_ptr).elevel as libc::c_int -
                                    1 as libc::c_int) as usize] *
                        5 as libc::c_int / 2 as libc::c_int;
                t = object_desc_num(t, need - (*o_ptr).exp)
            } else { t = object_desc_num(t, (*o_ptr).exp) }
            t =
                object_desc_str(t,
                                b", L:\x00" as *const u8 as
                                    *const libc::c_char);
            t = object_desc_num(t, (*o_ptr).elevel as s32b);
            t = object_desc_chr(t, ')' as i32 as libc::c_char)
        }
        if f4 as libc::c_long & 0x20000000 as libc::c_long != 0 &&
               known as libc::c_int != 0 {
            t =
                object_desc_str(t,
                                b" (Exp:\x00" as *const u8 as
                                    *const libc::c_char);
            t = object_desc_num(t, (*o_ptr).exp);
            t = object_desc_chr(t, ')' as i32 as libc::c_char)
        }
        /* Hack -- Chests must be described in detail */
        if (*o_ptr).tval as libc::c_int == 7 as libc::c_int {
            /* Not searched yet */
            if !(known == 0) {
                /* May be "empty" */
                if (*o_ptr).pval == 0 {
                    t =
                        object_desc_str(t,
                                        b" (empty)\x00" as *const u8 as
                                            *const libc::c_char)
                } else if (*o_ptr).pval < 0 as libc::c_int {
                    t =
                        object_desc_str(t,
                                        b" (disarmed)\x00" as *const u8 as
                                            *const libc::c_char)
                } else {
                    /* May be "disarmed" */
                    /* Describe the traps, if any */
                    /* Describe the traps */
                    t =
                        object_desc_str(t,
                                        b" (\x00" as *const u8 as
                                            *const libc::c_char);
                    if (*t_info.offset((*o_ptr).pval as isize)).ident != 0 {
                        t =
                            object_desc_str(t,
                                            t_name.offset((*t_info.offset((*o_ptr).pval
                                                                              as
                                                                              isize)).name
                                                              as libc::c_int
                                                              as isize) as
                                                cptr)
                    } else {
                        t =
                            object_desc_str(t,
                                            b"trapped\x00" as *const u8 as
                                                *const libc::c_char)
                    }
                    t =
                        object_desc_str(t,
                                        b")\x00" as *const u8 as
                                            *const libc::c_char)
                }
            }
        }
        /* Display the item like a weapon */
        if f3 as libc::c_long & 0x400 as libc::c_long != 0 {
            show_weapon = 1 as libc::c_int as bool_
        }
        /* Display the item like a weapon */
        if (*o_ptr).to_h as libc::c_int != 0 &&
               (*o_ptr).to_d as libc::c_int != 0 {
            show_weapon = 1 as libc::c_int as bool_
        }
        /* Display the item like armour */
        if (*o_ptr).ac != 0 { show_armour = 1 as libc::c_int as bool_ }
        let mut current_block_349: u64;
        /* Dump base weapon info */
        match (*o_ptr).tval as libc::c_int {
            16 | 18 | 17 => {
                /* Missiles and Weapons */
                /* Exploding arrow? */
                if (*o_ptr).pval2 as libc::c_int != 0 as libc::c_int {
                    t =
                        object_desc_str(t,
                                        b" (exploding)\x00" as *const u8 as
                                            *const libc::c_char)
                }
                current_block_349 = 14599146543692856897;
            }
            15 | 21 | 22 | 6 | 24 | 23 | 115 => {
                current_block_349 = 14599146543692856897;
            }
            19 => {
                /* Bows get a special "damage string" */
                /* Mega-Hack -- Extract the "base power" */
                power = (*o_ptr).sval as libc::c_int % 10 as libc::c_int;
                /* Apply the "Extra Might" flag */
                if f3 as libc::c_long & 0x40000 as libc::c_long != 0 {
                    power += (*o_ptr).pval
                }
                /* Append a special "damage" string */
                t = object_desc_chr(t, ' ' as i32 as libc::c_char);
                t = object_desc_chr(t, p1);
                t = object_desc_chr(t, 'x' as i32 as libc::c_char);
                t = object_desc_num(t, power);
                t = object_desc_chr(t, p2);
                /* All done */
                current_block_349 = 10249476801029275358;
            }
            _ => { current_block_349 = 10249476801029275358; }
        }
        match current_block_349 {
            14599146543692856897 =>
            /* No break, we want to continue the description */
            {
                if !((*o_ptr).tval as libc::c_int == 115 as libc::c_int &&
                         (*o_ptr).sval as libc::c_int != 55 as libc::c_int) {
                    /* Append a "damage" string */
                    t = object_desc_chr(t, ' ' as i32 as libc::c_char);
                    t = object_desc_chr(t, p1);
                    t = object_desc_num(t, (*o_ptr).dd as s32b);
                    t = object_desc_chr(t, 'd' as i32 as libc::c_char);
                    t = object_desc_num(t, (*o_ptr).ds as s32b);
                    t = object_desc_chr(t, p2)
                }
            }
            _ => { }
        }
        /* Add the weapon bonuses */
        if known != 0 {
            /* Show the tohit/todam on request */
            if show_weapon != 0 {
                t = object_desc_chr(t, ' ' as i32 as libc::c_char);
                t = object_desc_chr(t, p1);
                t = object_desc_int(t, (*o_ptr).to_h as s32b);
                t = object_desc_chr(t, ',' as i32 as libc::c_char);
                t = object_desc_int(t, (*o_ptr).to_d as s32b);
                t = object_desc_chr(t, p2)
            } else if (*o_ptr).to_h != 0 {
                t = object_desc_chr(t, ' ' as i32 as libc::c_char);
                t = object_desc_chr(t, p1);
                t = object_desc_int(t, (*o_ptr).to_h as s32b);
                if f3 as libc::c_long & 0x200 as libc::c_long == 0 ||
                       (*o_ptr).art_name as libc::c_int != 0 {
                    t =
                        object_desc_str(t,
                                        b" to accuracy\x00" as *const u8 as
                                            *const libc::c_char)
                }
                t = object_desc_chr(t, p2)
            } else if (*o_ptr).to_d != 0 {
                t = object_desc_chr(t, ' ' as i32 as libc::c_char);
                t = object_desc_chr(t, p1);
                t = object_desc_int(t, (*o_ptr).to_d as s32b);
                if f3 as libc::c_long & 0x200 as libc::c_long == 0 ||
                       (*o_ptr).art_name as libc::c_int != 0 {
                    t =
                        object_desc_str(t,
                                        b" to damage\x00" as *const u8 as
                                            *const libc::c_char)
                }
                t = object_desc_chr(t, p2)
            }
        }
        /* Show the tohit if needed */
        /* Show the todam if needed */
        /* Add the armor bonuses */
        if known != 0 {
            /* Show the armor class info */
            if show_armour != 0 {
                t = object_desc_chr(t, ' ' as i32 as libc::c_char);
                t = object_desc_chr(t, b1);
                t = object_desc_num(t, (*o_ptr).ac as s32b);
                t = object_desc_chr(t, ',' as i32 as libc::c_char);
                t = object_desc_int(t, (*o_ptr).to_a as s32b);
                t = object_desc_chr(t, b2)
            } else if (*o_ptr).to_a != 0 {
                t = object_desc_chr(t, ' ' as i32 as libc::c_char);
                t = object_desc_chr(t, b1);
                t = object_desc_int(t, (*o_ptr).to_a as s32b);
                t = object_desc_chr(t, b2)
            }
        } else if show_armour != 0 {
            t = object_desc_chr(t, ' ' as i32 as libc::c_char);
            t = object_desc_chr(t, b1);
            t = object_desc_num(t, (*o_ptr).ac as s32b);
            t = object_desc_chr(t, b2)
        }
        if f1 as libc::c_long & 0x40 as libc::c_long != 0 &&
               known as libc::c_int != 0 && (*o_ptr).pval > 0 as libc::c_int {
            t = object_desc_chr(t, '(' as i32 as libc::c_char);
            if munchkin_multipliers != 0 {
                t =
                    object_desc_num(t,
                                    100 as libc::c_int * (*o_ptr).pval /
                                        5 as libc::c_int)
            } else {
                t =
                    object_desc_num(t,
                                    100 as libc::c_int * (*o_ptr).pval /
                                        10 as libc::c_int)
            }
            t =
                object_desc_str(t,
                                b"%)\x00" as *const u8 as *const libc::c_char)
        }
        if known as libc::c_int != 0 &&
               f2 as libc::c_long & 0x80 as libc::c_long != 0 {
            /* No base armor, but does increase armor */
            /* Hack -- always show base armor */
            /* Can disp neg now -- Improv */
            t = object_desc_chr(t, '(' as i32 as libc::c_char);
            if munchkin_multipliers != 0 {
                t =
                    object_desc_num(t,
                                    100 as libc::c_int * (*o_ptr).pval /
                                        5 as libc::c_int)
            } else {
                t =
                    object_desc_num(t,
                                    100 as libc::c_int * (*o_ptr).pval /
                                        10 as libc::c_int)
            }
            t =
                object_desc_str(t,
                                b"%)\x00" as *const u8 as *const libc::c_char)
        }
        /* No more details wanted */
        if !(mode < 2 as libc::c_int) {
            /* Hack -- Wands and Staffs have charges */
            if known as libc::c_int != 0 &&
                   ((*o_ptr).tval as libc::c_int == 55 as libc::c_int ||
                        (*o_ptr).tval as libc::c_int == 65 as libc::c_int) {
                /* Dump " (N charges)" */
                t = object_desc_chr(t, ' ' as i32 as libc::c_char);
                t = object_desc_chr(t, p1);
                t = object_desc_num(t, (*o_ptr).pval);
                t =
                    object_desc_str(t,
                                    b" charge\x00" as *const u8 as
                                        *const libc::c_char);
                if (*o_ptr).pval != 1 as libc::c_int {
                    t = object_desc_chr(t, 's' as i32 as libc::c_char)
                }
                t = object_desc_chr(t, p2)
            } else if known as libc::c_int != 0 &&
                          (*o_ptr).tval as libc::c_int == 67 as libc::c_int {
                /*
	 * Hack -- Rods have a "charging" indicator.
	 */
                /* Display prettily. */
                t =
                    object_desc_str(t,
                                    b" (\x00" as *const u8 as
                                        *const libc::c_char);
                t = object_desc_num(t, (*o_ptr).timeout as s32b);
                t = object_desc_chr(t, '/' as i32 as libc::c_char);
                t = object_desc_num(t, (*o_ptr).pval2 as s32b);
                t = object_desc_chr(t, ')' as i32 as libc::c_char)
            } else if known as libc::c_int != 0 &&
                          (*o_ptr).tval as libc::c_int == 66 as libc::c_int {
                /*
	 * Hack -- Rods have a "charging" indicator.
	 */
                /* Display prettily. */
                t =
                    object_desc_str(t,
                                    b" (\x00" as *const u8 as
                                        *const libc::c_char);
                t = object_desc_num(t, (*o_ptr).pval);
                t =
                    object_desc_str(t,
                                    b" Mana to cast\x00" as *const u8 as
                                        *const libc::c_char);
                t = object_desc_chr(t, ')' as i32 as libc::c_char)
            } else if (*o_ptr).tval as libc::c_int == 39 as libc::c_int &&
                          f4 as libc::c_long & 0x10000000 as libc::c_long != 0
             {
                /* Hack -- Process Lanterns/Torches */
                /* Hack -- Turns of light for normal lites */
                t =
                    object_desc_str(t,
                                    b" (with \x00" as *const u8 as
                                        *const libc::c_char);
                t = object_desc_num(t, (*o_ptr).timeout as s32b);
                t =
                    object_desc_str(t,
                                    b" turns of light)\x00" as *const u8 as
                                        *const libc::c_char)
            }
            /* Dump "pval" flags for wearable items */
            if known as libc::c_int != 0 &&
                   (f1 as libc::c_long &
                        (0x1 as libc::c_long | 0x2 as libc::c_long |
                             0x4 as libc::c_long | 0x8 as libc::c_long |
                             0x10 as libc::c_long | 0x20 as libc::c_long |
                             0x100 as libc::c_long | 0x200 as libc::c_long |
                             0x400 as libc::c_long | 0x800 as libc::c_long |
                             0x1000 as libc::c_long | 0x2000 as libc::c_long |
                             0x80 as libc::c_long) != 0 ||
                        f5 as libc::c_long &
                            (0x20 as libc::c_long | 0x200 as libc::c_long) !=
                            0) {
                /* Start the display */
                t = object_desc_chr(t, ' ' as i32 as libc::c_char);
                t = object_desc_chr(t, p1);
                /* Dump the "pval" itself */
                t = object_desc_int(t, (*o_ptr).pval);
                /* Do not display the "pval" flags */
                if !(f3 as libc::c_long & 0x200 as libc::c_long != 0) {
                    /* Speed */
                    if f1 as libc::c_long & 0x1000 as libc::c_long != 0 {
                        /* Dump " to speed" */
                        t =
                            object_desc_str(t,
                                            b" to speed\x00" as *const u8 as
                                                *const libc::c_char)
                    } else if f1 as libc::c_long & 0x2000 as libc::c_long != 0
                     {
                        /* Attack speed */
                        /* Add " attack" */
                        t =
                            object_desc_str(t,
                                            b" attack\x00" as *const u8 as
                                                *const libc::c_char);
                        /* Add "attacks" */
                        if (if (*o_ptr).pval < 0 as libc::c_int {
                                -(*o_ptr).pval
                            } else { (*o_ptr).pval }) != 1 as libc::c_int {
                            t = object_desc_chr(t, 's' as i32 as libc::c_char)
                        }
                    } else if f5 as libc::c_long & 0x20 as libc::c_long != 0 {
                        /* Critical chance */
                        /* Add " attack" */
                        t =
                            object_desc_str(t,
                                            b"% of critical hits\x00" as
                                                *const u8 as
                                                *const libc::c_char)
                    } else if f1 as libc::c_long & 0x100 as libc::c_long != 0
                     {
                        /* Stealth */
                        /* Dump " to stealth" */
                        t =
                            object_desc_str(t,
                                            b" to stealth\x00" as *const u8 as
                                                *const libc::c_char)
                    } else if f1 as libc::c_long & 0x200 as libc::c_long != 0
                     {
                        /* Search */
                        /* Dump " to searching" */
                        t =
                            object_desc_str(t,
                                            b" to searching\x00" as *const u8
                                                as *const libc::c_char)
                    } else if f1 as libc::c_long & 0x400 as libc::c_long != 0
                     {
                        /* Infravision */
                        /* Dump " to infravision" */
                        t =
                            object_desc_str(t,
                                            b" to infravision\x00" as
                                                *const u8 as
                                                *const libc::c_char)
                    } else {
                        /* Tunneling */
                        (f1 as libc::c_long & 0x800 as libc::c_long) != 0;
                    }
                }
                /* Finish the display */
                t = object_desc_chr(t, p2)
            }
            /* Indicate "charging" artifacts XXX XXX XXX */
            if known as libc::c_int != 0 &&
                   f3 as libc::c_long & 0x1000000 as libc::c_long != 0 &&
                   (*o_ptr).timeout as libc::c_int != 0 {
                if (*o_ptr).tval as libc::c_int == 10 as libc::c_int {
                    /* Hack -- Dump " (stopped)" if relevant */
                    t =
                        object_desc_str(t,
                                        b" (stopped)\x00" as *const u8 as
                                            *const libc::c_char)
                } else {
                    /* Hack -- Dump " (charging)" if relevant */
                    t =
                        object_desc_str(t,
                                        b" (charging)\x00" as *const u8 as
                                            *const libc::c_char)
                }
            }
            /* Indicate "charging" Mage Staffs XXX XXX XXX */
            if known as libc::c_int != 0 &&
                   (*o_ptr).timeout as libc::c_int != 0 &&
                   ((*o_ptr).name2 as libc::c_int == 4 as libc::c_int ||
                        (*o_ptr).name2b as libc::c_int == 4 as libc::c_int) {
                /* Hack -- Dump " (charging spell1)" if relevant */
                t =
                    object_desc_str(t,
                                    b" (charging spell1)\x00" as *const u8 as
                                        *const libc::c_char)
            }
            if known as libc::c_int != 0 && (*o_ptr).xtra2 as libc::c_int != 0
                   &&
                   ((*o_ptr).name2 as libc::c_int == 4 as libc::c_int ||
                        (*o_ptr).name2b as libc::c_int == 4 as libc::c_int) {
                /* Hack -- Dump " (charging spell2)" if relevant */
                t =
                    object_desc_str(t,
                                    b" (charging spell2)\x00" as *const u8 as
                                        *const libc::c_char)
            }
            /* No more details wanted */
            if !(mode < 3 as libc::c_int) {
                /* No inscription yet */
                tmp_val2[0 as libc::c_int as usize] =
                    '\u{0}' as i32 as libc::c_char;
                /* Sensed stuff */
                if (*o_ptr).ident as libc::c_int & 0x1 as libc::c_int != 0 {
                    strcpy(tmp_val2.as_mut_ptr(),
                           *sense_desc.as_mut_ptr().offset((*o_ptr).sense as
                                                               isize));
                }
                /* Hack - Note "cursed" if the item is 'known' and cursed */
                if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 &&
                       known as libc::c_int != 0 &&
                       tmp_val2[0 as libc::c_int as usize] == 0 {
                    if tmp_val2[0 as libc::c_int as usize] != 0 {
                        strcat(tmp_val2.as_mut_ptr(),
                               b", \x00" as *const u8 as *const libc::c_char);
                    }
                    strcat(tmp_val2.as_mut_ptr(),
                           b"cursed\x00" as *const u8 as *const libc::c_char);
                }
                /* Use the standard inscription if available */
                if (*o_ptr).note != 0 {
                    let mut u_0: *mut libc::c_char = tmp_val2.as_mut_ptr();
                    if tmp_val2[0 as libc::c_int as usize] != 0 {
                        strcat(tmp_val2.as_mut_ptr(),
                               b", \x00" as *const u8 as *const libc::c_char);
                    }
                    strcat(tmp_val2.as_mut_ptr(),
                           quark_str((*o_ptr).note as s16b));
                    while *u_0 as libc::c_int != 0 &&
                              *u_0 as libc::c_int != '#' as i32 &&
                              *u_0 as libc::c_int != '%' as i32 {
                        u_0 = u_0.offset(1)
                    }
                    *u_0 = '\u{0}' as i32 as libc::c_char
                }
                /* Mega-Hack -- note empty wands/staffs */
                if known == 0 &&
                       (*o_ptr).ident as libc::c_int & 0x4 as libc::c_int != 0
                   {
                    if tmp_val2[0 as libc::c_int as usize] != 0 {
                        strcat(tmp_val2.as_mut_ptr(),
                               b", \x00" as *const u8 as *const libc::c_char);
                    }
                    strcat(tmp_val2.as_mut_ptr(),
                           b"empty\x00" as *const u8 as *const libc::c_char);
                }
                /* Note "tried" if the object has been tested unsuccessfully */
                if aware == 0 &&
                       (*k_info.offset((*o_ptr).k_idx as isize)).tried as
                           libc::c_int != 0 {
                    if tmp_val2[0 as libc::c_int as usize] != 0 {
                        strcat(tmp_val2.as_mut_ptr(),
                               b", \x00" as *const u8 as *const libc::c_char);
                    }
                    strcpy(tmp_val2.as_mut_ptr(),
                           b"tried\x00" as *const u8 as *const libc::c_char);
                }
                /* Note the discount, if any */
                if (*o_ptr).discount as libc::c_int != 0 &&
                       tmp_val2[0 as libc::c_int as usize] == 0 {
                    object_desc_num(tmp_val2.as_mut_ptr(),
                                    (*o_ptr).discount as s32b);
                    strcat(tmp_val2.as_mut_ptr(),
                           b"% off\x00" as *const u8 as *const libc::c_char);
                }
                /* Append the inscription, if any */
                if tmp_val2[0 as libc::c_int as usize] != 0 {
                    let mut n: libc::c_int = 0;
                    /* Hack -- How much so far */
                    n =
                        t.wrapping_offset_from(tmp_val.as_mut_ptr()) as
                            libc::c_long as libc::c_int;
                    /* Paranoia -- do not be stupid */
                    if n > 75 as libc::c_int { n = 75 as libc::c_int }
                    /* Hack -- shrink the inscription */
                    tmp_val2[(75 as libc::c_int - n) as usize] =
                        '\u{0}' as i32 as libc::c_char;
                    /* Append the inscription */
                    t = object_desc_chr(t, ' ' as i32 as libc::c_char);
                    t = object_desc_chr(t, c1);
                    t = object_desc_str(t, tmp_val2.as_mut_ptr() as cptr);
                    t = object_desc_chr(t, c2)
                }
            }
        }
    }
    /* Here's where we dump the built string into buf. */
    tmp_val[79 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    t = tmp_val.as_mut_ptr();
    loop  {
        let fresh23 = t;
        t = t.offset(1);
        let fresh24 = buf;
        buf = buf.offset(1);
        *fresh24 = *fresh23;
        if !(*fresh24 != 0) { break ; }
    };
    /* copy the string over */
}
/*
 * Hack -- describe an item currently in a store's inventory
 * This allows an item to *look* like the player is "aware" of it
 */
#[no_mangle]
pub unsafe extern "C" fn object_desc_store(mut buf: *mut libc::c_char,
                                           mut o_ptr: *mut object_type,
                                           mut pref: libc::c_int,
                                           mut mode: libc::c_int) {
    /* Save the "aware" flag */
    let mut hack_aware: bool_ =
        (*k_info.offset((*o_ptr).k_idx as isize)).aware;
    /* Save the "known" flag */
    let mut hack_known: bool_ =
        if (*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int } as bool_;
    /* Set the "known" flag */
    (*o_ptr).ident =
        ((*o_ptr).ident as libc::c_int | 0x8 as libc::c_int) as byte_hack;
    /* Force "aware" for description */
    (*k_info.offset((*o_ptr).k_idx as isize)).aware =
        1 as libc::c_int as bool_;
    /* Describe the object */
    object_desc(buf, o_ptr, pref, mode);
    /* Restore "aware" flag */
    (*k_info.offset((*o_ptr).k_idx as isize)).aware = hack_aware;
    /* Clear the known flag */
    if hack_known == 0 {
        (*o_ptr).ident =
            ((*o_ptr).ident as libc::c_int & !(0x8 as libc::c_int)) as
                byte_hack
    };
}
/*
 * Determine the "Activation" (if any) for an artifact
 * Return a string, or NULL for "no activation"
 */
#[no_mangle]
pub unsafe extern "C" fn item_activation(mut o_ptr: *mut object_type,
                                         mut num: byte_hack) -> cptr {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    /* Needed hacks */
    static mut rspell: [[libc::c_char; 80]; 2] = [[0; 80]; 2];
    /* Extract the flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Require activation ability */
    if f3 as libc::c_long & 0x1000000 as libc::c_long == 0 {
        return 0 as cptr
    }
    /*
	 * We need to deduce somehow that it is a random artifact -- one
	 * problem: It could be a random artifact which has NOT YET received
	 * a name. Thus we eliminate other possibilities instead of checking
	 * for art_name
	 */
    if (*o_ptr).name2 as libc::c_int == 4 as libc::c_int ||
           (*o_ptr).name2b as libc::c_int == 4 as libc::c_int {
        let mut gf: libc::c_int = 0;
        let mut mod_0: libc::c_int = 0;
        let mut mana: libc::c_int = 0;
        if num == 0 {
            gf = (*o_ptr).pval & 0xffff as libc::c_int;
            mod_0 = (*o_ptr).pval3 & 0xffff as libc::c_int;
            mana = (*o_ptr).pval2 as libc::c_int & 0xff as libc::c_int
        } else {
            gf = (*o_ptr).pval >> 16 as libc::c_int;
            mod_0 = (*o_ptr).pval3 >> 16 as libc::c_int;
            mana = (*o_ptr).pval2 as libc::c_int >> 8 as libc::c_int
        }
        sprintf(rspell[num as usize].as_mut_ptr(),
                b"runespell(%s, %s, %d) every %d turns\x00" as *const u8 as
                    *const libc::c_char,
                k_name.offset((*k_info.offset(lookup_kind(104 as libc::c_int,
                                                          gf) as isize)).name
                                  as isize),
                k_name.offset((*k_info.offset(lookup_kind(105 as libc::c_int,
                                                          mod_0) as
                                                  isize)).name as isize),
                mana, mana * 5 as libc::c_int);
        return rspell[num as usize].as_mut_ptr() as cptr
    }
    if (*o_ptr).tval as libc::c_int == 10 as libc::c_int {
        return b"stop or resume the egg development\x00" as *const u8 as
                   *const libc::c_char
    }
    if (*o_ptr).tval as libc::c_int == 14 as libc::c_int {
        if !((*o_ptr).name1 as libc::c_int != 0 &&
                 (*a_info.offset((*o_ptr).name1 as isize)).activate as
                     libc::c_int != 0 ||
                 (*o_ptr).name2 as libc::c_int != 0 &&
                     (*e_info.offset((*o_ptr).name2 as isize)).activate as
                         libc::c_int != 0 ||
                 (*o_ptr).name2b as libc::c_int != 0 &&
                     (*e_info.offset((*o_ptr).name2b as isize)).activate as
                         libc::c_int != 0) {
            if (*o_ptr).sval as libc::c_int == 60 as libc::c_int {
                return b"aggravate monster every 100 turns\x00" as *const u8
                           as *const libc::c_char
            }
        }
    }
    if process_hooks_ret(36 as libc::c_int,
                         b"s\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char,
                         b"(O)\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char, o_ptr) != 0 {
        return process_hooks_return[0 as libc::c_int as usize].str_0
    }
    return activation_aux(o_ptr, 0 as libc::c_int as bool_, 0 as libc::c_int);
}
/* Grab the tval desc */
#[no_mangle]
pub unsafe extern "C" fn grab_tval_desc(mut tval: libc::c_int) -> bool_ {
    let mut tv: libc::c_int = 0 as libc::c_int;
    while (*tval_descs.as_mut_ptr().offset(tv as isize)).tval != 0 &&
              (*tval_descs.as_mut_ptr().offset(tv as isize)).tval != tval {
        tv += 1
    }
    if (*tval_descs.as_mut_ptr().offset(tv as isize)).tval == 0 {
        return 0 as libc::c_int as bool_
    }
    text_out_c(14 as libc::c_int as byte_hack,
               (*tval_descs.as_mut_ptr().offset(tv as isize)).desc);
    text_out(b"\n\x00" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int as bool_;
}
/*
 * Display the damage done with a multiplier
 */
#[no_mangle]
pub unsafe extern "C" fn output_dam(mut o_ptr: *mut object_type,
                                    mut mult: libc::c_int,
                                    mut mult2: libc::c_int, mut against: cptr,
                                    mut against2: cptr,
                                    mut first: *mut bool_) {
    let mut dam: libc::c_int = 0;
    dam =
        ((*o_ptr).dd as libc::c_int +
             (*o_ptr).dd as libc::c_int * (*o_ptr).ds as libc::c_int) *
            5 as libc::c_int * mult;
    dam +=
        ((*o_ptr).to_d as libc::c_int + (*p_ptr).to_d as libc::c_int +
             (*p_ptr).to_d_melee as libc::c_int) * 10 as libc::c_int;
    dam *= (*p_ptr).num_blow as libc::c_int;
    if *first != 0 {
        *first = 0 as libc::c_int as bool_;
        text_out(b"\x00" as *const u8 as *const libc::c_char);
    } else { text_out(b", \x00" as *const u8 as *const libc::c_char); }
    if dam > 0 as libc::c_int {
        if dam % 10 as libc::c_int != 0 {
            text_out_c(13 as libc::c_int as byte_hack,
                       format(b"%d.%d\x00" as *const u8 as
                                  *const libc::c_char,
                              dam / 10 as libc::c_int,
                              dam % 10 as libc::c_int) as cptr);
        } else {
            text_out_c(13 as libc::c_int as byte_hack,
                       format(b"%d\x00" as *const u8 as *const libc::c_char,
                              dam / 10 as libc::c_int) as cptr);
        }
    } else {
        text_out_c(12 as libc::c_int as byte_hack,
                   b"0\x00" as *const u8 as *const libc::c_char);
    }
    text_out(format(b" against %s\x00" as *const u8 as *const libc::c_char,
                    against) as cptr);
    if mult2 != 0 {
        dam =
            ((*o_ptr).dd as libc::c_int +
                 (*o_ptr).dd as libc::c_int * (*o_ptr).ds as libc::c_int) *
                5 as libc::c_int * mult2;
        dam +=
            ((*o_ptr).to_d as libc::c_int + (*p_ptr).to_d as libc::c_int +
                 (*p_ptr).to_d_melee as libc::c_int) * 10 as libc::c_int;
        dam *= (*p_ptr).num_blow as libc::c_int;
        if *first != 0 {
            *first = 0 as libc::c_int as bool_;
            text_out(b"\x00" as *const u8 as *const libc::c_char);
        } else { text_out(b", \x00" as *const u8 as *const libc::c_char); }
        if dam > 0 as libc::c_int {
            if dam % 10 as libc::c_int != 0 {
                text_out_c(13 as libc::c_int as byte_hack,
                           format(b"%d.%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  dam / 10 as libc::c_int,
                                  dam % 10 as libc::c_int) as cptr);
            } else {
                text_out_c(13 as libc::c_int as byte_hack,
                           format(b"%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  dam / 10 as libc::c_int) as cptr);
            }
        } else {
            text_out_c(12 as libc::c_int as byte_hack,
                       b"0\x00" as *const u8 as *const libc::c_char);
        }
        text_out(format(b" against %s\x00" as *const u8 as
                            *const libc::c_char, against2) as cptr);
    };
}
/*
 * Outputs the damage we do/would do with the weapon
 */
#[no_mangle]
pub unsafe extern "C" fn display_weapon_damage(mut o_ptr: *mut object_type) {
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
    let mut old_ptr: *mut object_type = &mut forge;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut first: bool_ = 1 as libc::c_int as bool_;
    let mut full: bool_ =
        ((*o_ptr).ident as libc::c_int & 0x20 as libc::c_int) as bool_;
    /* Extract the flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Ok now the hackish stuff, we replace the current weapon with this one */
    object_copy(old_ptr,
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(24 as libc::c_int
                                                                 as isize));
    object_copy(&mut *(*p_ptr).inventory.as_mut_ptr().offset(24 as libc::c_int
                                                                 as isize),
                o_ptr);
    calc_bonuses(1 as libc::c_int as bool_);
    text_out(b"\nUsing it you would have \x00" as *const u8 as
                 *const libc::c_char);
    text_out_c(13 as libc::c_int as byte_hack,
               format(b"%d \x00" as *const u8 as *const libc::c_char,
                      (*p_ptr).num_blow as libc::c_int) as cptr);
    text_out(format(b"blow%s and do an average damage per turn of \x00" as
                        *const u8 as *const libc::c_char,
                    if (*p_ptr).num_blow as libc::c_int != 0 {
                        b"s\x00" as *const u8 as *const libc::c_char
                    } else { b"\x00" as *const u8 as *const libc::c_char }) as
                 cptr);
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x10000 as libc::c_long != 0 {
        output_dam(o_ptr, 2 as libc::c_int, 0 as libc::c_int,
                   b"animals\x00" as *const u8 as *const libc::c_char,
                   0 as cptr, &mut first);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x20000 as libc::c_long != 0 {
        output_dam(o_ptr, 2 as libc::c_int, 0 as libc::c_int,
                   b"evil creatures\x00" as *const u8 as *const libc::c_char,
                   0 as cptr, &mut first);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x100000 as libc::c_long != 0 {
        output_dam(o_ptr, 3 as libc::c_int, 0 as libc::c_int,
                   b"orcs\x00" as *const u8 as *const libc::c_char, 0 as cptr,
                   &mut first);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x200000 as libc::c_long != 0 {
        output_dam(o_ptr, 3 as libc::c_int, 0 as libc::c_int,
                   b"trolls\x00" as *const u8 as *const libc::c_char,
                   0 as cptr, &mut first);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x400000 as libc::c_long != 0 {
        output_dam(o_ptr, 3 as libc::c_int, 0 as libc::c_int,
                   b"giants\x00" as *const u8 as *const libc::c_char,
                   0 as cptr, &mut first);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x1000000 as libc::c_long != 0 {
        output_dam(o_ptr, 5 as libc::c_int, 0 as libc::c_int,
                   b"dragons\x00" as *const u8 as *const libc::c_char,
                   0 as cptr, &mut first);
    } else if full as libc::c_int != 0 &&
                  f1 as libc::c_long & 0x800000 as libc::c_long != 0 {
        output_dam(o_ptr, 3 as libc::c_int, 0 as libc::c_int,
                   b"dragons\x00" as *const u8 as *const libc::c_char,
                   0 as cptr, &mut first);
    }
    if full as libc::c_int != 0 &&
           f5 as libc::c_long & 0x10 as libc::c_long != 0 {
        output_dam(o_ptr, 5 as libc::c_int, 0 as libc::c_int,
                   b"undead\x00" as *const u8 as *const libc::c_char,
                   0 as cptr, &mut first);
    } else if full as libc::c_int != 0 &&
                  f1 as libc::c_long & 0x40000 as libc::c_long != 0 {
        output_dam(o_ptr, 3 as libc::c_int, 0 as libc::c_int,
                   b"undead\x00" as *const u8 as *const libc::c_char,
                   0 as cptr, &mut first);
    }
    if full as libc::c_int != 0 &&
           f5 as libc::c_long & 0x8 as libc::c_long != 0 {
        output_dam(o_ptr, 5 as libc::c_int, 0 as libc::c_int,
                   b"demons\x00" as *const u8 as *const libc::c_char,
                   0 as cptr, &mut first);
    } else if full as libc::c_int != 0 &&
                  f1 as libc::c_long & 0x80000 as libc::c_long != 0 {
        output_dam(o_ptr, 3 as libc::c_int, 0 as libc::c_int,
                   b"demons\x00" as *const u8 as *const libc::c_char,
                   0 as cptr, &mut first);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x40000000 as libc::c_long != 0 {
        output_dam(o_ptr, 3 as libc::c_int, 6 as libc::c_int,
                   b"non fire resistant creatures\x00" as *const u8 as
                       *const libc::c_char,
                   b"fire susceptible creatures\x00" as *const u8 as
                       *const libc::c_char, &mut first);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x80000000 as libc::c_long != 0 {
        output_dam(o_ptr, 3 as libc::c_int, 6 as libc::c_int,
                   b"non cold resistant creatures\x00" as *const u8 as
                       *const libc::c_char,
                   b"cold susceptible creatures\x00" as *const u8 as
                       *const libc::c_char, &mut first);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x20000000 as libc::c_long != 0 {
        output_dam(o_ptr, 3 as libc::c_int, 6 as libc::c_int,
                   b"non lightning resistant creatures\x00" as *const u8 as
                       *const libc::c_char,
                   b"lightning susceptible creatures\x00" as *const u8 as
                       *const libc::c_char, &mut first);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x10000000 as libc::c_long != 0 {
        output_dam(o_ptr, 3 as libc::c_int, 6 as libc::c_int,
                   b"non acid resistant creatures\x00" as *const u8 as
                       *const libc::c_char,
                   b"acid susceptible creatures\x00" as *const u8 as
                       *const libc::c_char, &mut first);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x8000000 as libc::c_long != 0 {
        output_dam(o_ptr, 3 as libc::c_int, 6 as libc::c_int,
                   b"non poison resistant creatures\x00" as *const u8 as
                       *const libc::c_char,
                   b"poison susceptible creatures\x00" as *const u8 as
                       *const libc::c_char, &mut first);
    }
    output_dam(o_ptr, 1 as libc::c_int, 0 as libc::c_int,
               if first as libc::c_int != 0 {
                   b"all monsters\x00" as *const u8 as *const libc::c_char
               } else {
                   b"other monsters\x00" as *const u8 as *const libc::c_char
               }, 0 as cptr, &mut first);
    text_out(b".\x00" as *const u8 as *const libc::c_char);
    /* get our weapon back */
    object_copy(&mut *(*p_ptr).inventory.as_mut_ptr().offset(24 as libc::c_int
                                                                 as isize),
                old_ptr);
    calc_bonuses(1 as libc::c_int as bool_);
}
/*
 * Display the ammo damage done with a multiplier
 */
#[no_mangle]
pub unsafe extern "C" fn output_ammo_dam(mut o_ptr: *mut object_type,
                                         mut mult: libc::c_int,
                                         mut mult2: libc::c_int,
                                         mut against: cptr,
                                         mut against2: cptr,
                                         mut first: *mut bool_) {
    let mut dam: libc::c_int = 0;
    let mut b_ptr: *mut object_type =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(27 as libc::c_int as
                                                         isize) as
            *mut object_type;
    let mut is_boomerang: libc::c_int =
        ((*o_ptr).tval as libc::c_int == 15 as libc::c_int) as libc::c_int;
    let mut tmul: libc::c_int =
        get_shooter_mult(b_ptr) + (*p_ptr).xtra_might as libc::c_int;
    if is_boomerang != 0 { tmul = (*p_ptr).throw_mult as libc::c_int }
    dam =
        ((*o_ptr).dd as libc::c_int +
             (*o_ptr).dd as libc::c_int * (*o_ptr).ds as libc::c_int) *
            5 as libc::c_int;
    dam += (*o_ptr).to_d as libc::c_int * 10 as libc::c_int;
    if is_boomerang == 0 {
        dam += (*b_ptr).to_d as libc::c_int * 10 as libc::c_int
    }
    dam *= tmul;
    if is_boomerang == 0 {
        dam += (*p_ptr).to_d_ranged as libc::c_int * 10 as libc::c_int
    }
    dam *= mult;
    if *first != 0 {
        *first = 0 as libc::c_int as bool_;
        text_out(b"\x00" as *const u8 as *const libc::c_char);
    } else { text_out(b", \x00" as *const u8 as *const libc::c_char); }
    if dam > 0 as libc::c_int {
        if dam % 10 as libc::c_int != 0 {
            text_out_c(13 as libc::c_int as byte_hack,
                       format(b"%d.%d\x00" as *const u8 as
                                  *const libc::c_char,
                              dam / 10 as libc::c_int,
                              dam % 10 as libc::c_int) as cptr);
        } else {
            text_out_c(13 as libc::c_int as byte_hack,
                       format(b"%d\x00" as *const u8 as *const libc::c_char,
                              dam / 10 as libc::c_int) as cptr);
        }
    } else {
        text_out_c(12 as libc::c_int as byte_hack,
                   b"0\x00" as *const u8 as *const libc::c_char);
    }
    text_out(format(b" against %s\x00" as *const u8 as *const libc::c_char,
                    against) as cptr);
    if mult2 != 0 {
        dam =
            ((*o_ptr).dd as libc::c_int +
                 (*o_ptr).dd as libc::c_int * (*o_ptr).ds as libc::c_int) *
                5 as libc::c_int;
        dam += (*o_ptr).to_d as libc::c_int * 10 as libc::c_int;
        if is_boomerang == 0 {
            dam += (*b_ptr).to_d as libc::c_int * 10 as libc::c_int
        }
        dam *= tmul;
        if is_boomerang == 0 {
            dam += (*p_ptr).to_d_ranged as libc::c_int * 10 as libc::c_int
        }
        dam *= mult2;
        if *first != 0 {
            *first = 0 as libc::c_int as bool_;
            text_out(b"\x00" as *const u8 as *const libc::c_char);
        } else { text_out(b", \x00" as *const u8 as *const libc::c_char); }
        if dam > 0 as libc::c_int {
            if dam % 10 as libc::c_int != 0 {
                text_out_c(13 as libc::c_int as byte_hack,
                           format(b"%d.%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  dam / 10 as libc::c_int,
                                  dam % 10 as libc::c_int) as cptr);
            } else {
                text_out_c(13 as libc::c_int as byte_hack,
                           format(b"%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  dam / 10 as libc::c_int) as cptr);
            }
        } else {
            text_out_c(12 as libc::c_int as byte_hack,
                       b"0\x00" as *const u8 as *const libc::c_char);
        }
        text_out(format(b" against %s\x00" as *const u8 as
                            *const libc::c_char, against2) as cptr);
    };
}
/*
 * Outputs the damage we do/would do with the current bow and this ammo
 */
#[no_mangle]
pub unsafe extern "C" fn display_ammo_damage(mut o_ptr: *mut object_type) {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut first: bool_ = 1 as libc::c_int as bool_;
    let mut i: libc::c_int = 0;
    let mut full: bool_ =
        ((*o_ptr).ident as libc::c_int & 0x20 as libc::c_int) as bool_;
    /* Extract the flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    if (*o_ptr).tval as libc::c_int == 15 as libc::c_int {
        text_out(b"\nUsing it you would do an average damage per throw of \x00"
                     as *const u8 as *const libc::c_char);
    } else {
        text_out(b"\nUsing it with your current shooter you would do an average damage per shot of \x00"
                     as *const u8 as *const libc::c_char);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x10000 as libc::c_long != 0 {
        output_ammo_dam(o_ptr, 2 as libc::c_int, 0 as libc::c_int,
                        b"animals\x00" as *const u8 as *const libc::c_char,
                        0 as cptr, &mut first);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x20000 as libc::c_long != 0 {
        output_ammo_dam(o_ptr, 2 as libc::c_int, 0 as libc::c_int,
                        b"evil creatures\x00" as *const u8 as
                            *const libc::c_char, 0 as cptr, &mut first);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x100000 as libc::c_long != 0 {
        output_ammo_dam(o_ptr, 3 as libc::c_int, 0 as libc::c_int,
                        b"orcs\x00" as *const u8 as *const libc::c_char,
                        0 as cptr, &mut first);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x200000 as libc::c_long != 0 {
        output_ammo_dam(o_ptr, 3 as libc::c_int, 0 as libc::c_int,
                        b"trolls\x00" as *const u8 as *const libc::c_char,
                        0 as cptr, &mut first);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x400000 as libc::c_long != 0 {
        output_ammo_dam(o_ptr, 3 as libc::c_int, 0 as libc::c_int,
                        b"giants\x00" as *const u8 as *const libc::c_char,
                        0 as cptr, &mut first);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x1000000 as libc::c_long != 0 {
        output_ammo_dam(o_ptr, 5 as libc::c_int, 0 as libc::c_int,
                        b"dragons\x00" as *const u8 as *const libc::c_char,
                        0 as cptr, &mut first);
    } else if full as libc::c_int != 0 &&
                  f1 as libc::c_long & 0x800000 as libc::c_long != 0 {
        output_ammo_dam(o_ptr, 3 as libc::c_int, 0 as libc::c_int,
                        b"dragons\x00" as *const u8 as *const libc::c_char,
                        0 as cptr, &mut first);
    }
    if full as libc::c_int != 0 &&
           f5 as libc::c_long & 0x10 as libc::c_long != 0 {
        output_ammo_dam(o_ptr, 5 as libc::c_int, 0 as libc::c_int,
                        b"undeads\x00" as *const u8 as *const libc::c_char,
                        0 as cptr, &mut first);
    } else if full as libc::c_int != 0 &&
                  f1 as libc::c_long & 0x40000 as libc::c_long != 0 {
        output_ammo_dam(o_ptr, 3 as libc::c_int, 0 as libc::c_int,
                        b"undeads\x00" as *const u8 as *const libc::c_char,
                        0 as cptr, &mut first);
    }
    if full as libc::c_int != 0 &&
           f5 as libc::c_long & 0x8 as libc::c_long != 0 {
        output_ammo_dam(o_ptr, 5 as libc::c_int, 0 as libc::c_int,
                        b"demons\x00" as *const u8 as *const libc::c_char,
                        0 as cptr, &mut first);
    } else if full as libc::c_int != 0 &&
                  f1 as libc::c_long & 0x80000 as libc::c_long != 0 {
        output_ammo_dam(o_ptr, 3 as libc::c_int, 0 as libc::c_int,
                        b"demons\x00" as *const u8 as *const libc::c_char,
                        0 as cptr, &mut first);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x40000000 as libc::c_long != 0 {
        output_ammo_dam(o_ptr, 3 as libc::c_int, 6 as libc::c_int,
                        b"non fire resistant creatures\x00" as *const u8 as
                            *const libc::c_char,
                        b"fire susceptible creatures\x00" as *const u8 as
                            *const libc::c_char, &mut first);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x80000000 as libc::c_long != 0 {
        output_ammo_dam(o_ptr, 3 as libc::c_int, 6 as libc::c_int,
                        b"non cold resistant creatures\x00" as *const u8 as
                            *const libc::c_char,
                        b"cold susceptible creatures\x00" as *const u8 as
                            *const libc::c_char, &mut first);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x20000000 as libc::c_long != 0 {
        output_ammo_dam(o_ptr, 3 as libc::c_int, 6 as libc::c_int,
                        b"non lightning resistant creatures\x00" as *const u8
                            as *const libc::c_char,
                        b"lightning susceptible creatures\x00" as *const u8 as
                            *const libc::c_char, &mut first);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x10000000 as libc::c_long != 0 {
        output_ammo_dam(o_ptr, 3 as libc::c_int, 6 as libc::c_int,
                        b"non acid resistant creatures\x00" as *const u8 as
                            *const libc::c_char,
                        b"acid susceptible creatures\x00" as *const u8 as
                            *const libc::c_char, &mut first);
    }
    if full as libc::c_int != 0 &&
           f1 as libc::c_long & 0x8000000 as libc::c_long != 0 {
        output_ammo_dam(o_ptr, 3 as libc::c_int, 6 as libc::c_int,
                        b"non poison resistant creatures\x00" as *const u8 as
                            *const libc::c_char,
                        b"poison susceptible creatures\x00" as *const u8 as
                            *const libc::c_char, &mut first);
    }
    output_ammo_dam(o_ptr, 1 as libc::c_int, 0 as libc::c_int,
                    if first as libc::c_int != 0 {
                        b"all monsters\x00" as *const u8 as
                            *const libc::c_char
                    } else {
                        b"other monsters\x00" as *const u8 as
                            *const libc::c_char
                    }, 0 as cptr, &mut first);
    text_out(b". \x00" as *const u8 as *const libc::c_char);
    if (*o_ptr).pval2 != 0 {
        text_out(b"The explosion will be \x00" as *const u8 as
                     *const libc::c_char);
        i = 0 as libc::c_int;
        while (*gf_names.as_mut_ptr().offset(i as isize)).gf !=
                  -(1 as libc::c_int) {
            if (*gf_names.as_mut_ptr().offset(i as isize)).gf ==
                   (*o_ptr).pval2 as libc::c_int {
                break ;
            }
            i += 1
        }
        text_out_c(13 as libc::c_int as byte_hack,
                   if (*gf_names.as_mut_ptr().offset(i as isize)).gf !=
                          -(1 as libc::c_int) {
                       (*gf_names.as_mut_ptr().offset(i as isize)).name
                   } else {
                       b"something weird\x00" as *const u8 as
                           *const libc::c_char
                   });
        text_out(b".\x00" as *const u8 as *const libc::c_char);
    };
}
/*
 * Describe a magic stick powers
 */
#[no_mangle]
pub unsafe extern "C" fn describe_device(mut o_ptr: *mut object_type) {
    /* Wands/... of shcool spell */
    if ((*o_ptr).tval as libc::c_int == 65 as libc::c_int ||
            (*o_ptr).tval as libc::c_int == 55 as libc::c_int) &&
           ((*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
                (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as
                    libc::c_int != 0 &&
                    (*k_info.offset((*o_ptr).k_idx as isize)).aware as
                        libc::c_int != 0) {
        /* Enter device mode  */
        set_stick_mode(o_ptr);
        text_out(b"\nSpell description:\x00" as *const u8 as
                     *const libc::c_char);
        exec_lua(format(b"print_device_desc(%d)\x00" as *const u8 as
                            *const libc::c_char,
                        (*o_ptr).pval2 as libc::c_int));
        text_out(b"\nSpell level: \x00" as *const u8 as *const libc::c_char);
        text_out_c(14 as libc::c_int as byte_hack,
                   string_exec_lua(format(b"return tostring(get_level(%d, 50, 0))\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          (*o_ptr).pval2 as libc::c_int)));
        text_out(b"\nMinimum Magic Device level to increase spell level: \x00"
                     as *const u8 as *const libc::c_char);
        text_out_c(14 as libc::c_int as byte_hack,
                   format(b"%d\x00" as *const u8 as *const libc::c_char,
                          (*school_spells.offset((*o_ptr).pval2 as
                                                     isize)).skill_level as
                              libc::c_int) as cptr);
        text_out(b"\nSpell fail: \x00" as *const u8 as *const libc::c_char);
        text_out_c(5 as libc::c_int as byte_hack,
                   string_exec_lua(format(b"return tostring(spell_chance(%d))\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          (*o_ptr).pval2 as libc::c_int)));
        text_out(b"\nSpell info: \x00" as *const u8 as *const libc::c_char);
        text_out_c(11 as libc::c_int as byte_hack,
                   string_exec_lua(format(b"return __spell_info[%d]()\x00" as
                                              *const u8 as
                                              *const libc::c_char,
                                          (*o_ptr).pval2 as libc::c_int)));
        /* Leave device mode  */
        unset_stick_mode();
        text_out(b"\n\x00" as *const u8 as *const libc::c_char);
    };
}
/*
 * Helper for object_out_desc
 *
 * Print the level something was found on
 *
 */
unsafe extern "C" fn object_out_desc_where_found(mut level: s16b,
                                                 mut dungeon: s16b) -> cptr {
    static mut str: [libc::c_char; 80] = [0; 80];
    if dungeon as libc::c_int == 0 as libc::c_int {
        /* Taking care of older objects */
        if level as libc::c_int == 0 as libc::c_int {
            sprintf(str.as_mut_ptr(),
                    b"in the wilderness or in a town\x00" as *const u8 as
                        *const libc::c_char);
        } else if (*wf_info.offset(level as isize)).terrain_idx as libc::c_int
                      == 1 as libc::c_int {
            sprintf(str.as_mut_ptr(),
                    b"in the town of %s\x00" as *const u8 as
                        *const libc::c_char,
                    wf_name.offset((*wf_info.offset(level as isize)).name as
                                       isize));
        } else {
            sprintf(str.as_mut_ptr(),
                    b"in %s\x00" as *const u8 as *const libc::c_char,
                    wf_text.offset((*wf_info.offset(level as isize)).text as
                                       isize));
        }
    } else {
        sprintf(str.as_mut_ptr(),
                b"on level %d of %s\x00" as *const u8 as *const libc::c_char,
                level as libc::c_int,
                d_name.offset((*d_info.offset(dungeon as isize)).name as
                                  isize));
    }
    return str.as_mut_ptr() as cptr;
}
/*
 * Describe an item
 */
#[no_mangle]
pub unsafe extern "C" fn object_out_desc(mut o_ptr: *mut object_type,
                                         mut fff: *mut FILE,
                                         mut trim_down: bool_,
                                         mut wait_for_it: bool_) -> bool_ {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut txt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vp: [cptr; 64] = [0 as *const libc::c_char; 64];
    let mut vc: [byte_hack; 64] = [0; 64];
    let mut vn: libc::c_int = 0;
    let mut first: bool_ = 1 as libc::c_int as bool_;
    /* Extract the flags */
    if (*o_ptr).ident as libc::c_int & 0x20 as libc::c_int == 0 &&
           fff.is_null() {
        f1 = (*o_ptr).art_oflags1;
        f2 = (*o_ptr).art_oflags2;
        f3 = (*o_ptr).art_oflags3;
        f4 = (*o_ptr).art_oflags4;
        f5 = (*o_ptr).art_oflags5;
        esp = (*o_ptr).art_oesp
    } else {
        object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                     &mut esp);
    }
    if !fff.is_null() {
        /* Set up stuff for text_out */
        text_out_file = fff;
        text_out_hook =
            Some(text_out_to_file as
                     unsafe extern "C" fn(_: byte_hack, _: cptr) -> ())
    } else {
        /* Save the screen */
        character_icky = 1 as libc::c_int as bool_;
        Term_save();
        /* Set up stuff for text_out */
        text_out_hook =
            Some(text_out_to_screen as
                     unsafe extern "C" fn(_: byte_hack, _: cptr) -> ());
        text_out(b"\n\x00" as *const u8 as *const libc::c_char);
    }
    /* No need to dump that */
    if fff.is_null() {
        if trim_down == 0 { grab_tval_desc((*o_ptr).tval as libc::c_int); }
    }
    if (*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
           (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as libc::c_int
               != 0 &&
               (*k_info.offset((*o_ptr).k_idx as isize)).aware as libc::c_int
                   != 0 {
        if (*o_ptr).k_idx as libc::c_int != 0 && trim_down == 0 {
            let mut k_ptr: *mut object_kind =
                &mut *k_info.offset((*o_ptr).k_idx as isize) as
                    *mut object_kind;
            text_out_c(3 as libc::c_int as byte_hack,
                       k_text.offset((*k_ptr).text as isize) as cptr);
            text_out(b"\n\x00" as *const u8 as *const libc::c_char);
        }
        if (*o_ptr).name1 as libc::c_int != 0 && trim_down == 0 {
            let mut a_ptr: *mut artifact_type =
                &mut *a_info.offset((*o_ptr).name1 as isize) as
                    *mut artifact_type;
            text_out_c(11 as libc::c_int as byte_hack,
                       a_text.offset((*a_ptr).text as isize) as cptr);
            text_out(b"\n\x00" as *const u8 as *const libc::c_char);
            if (*a_ptr).set as libc::c_int != -(1 as libc::c_int) {
                let mut set_ptr: *mut set_type =
                    &mut *set_info.offset((*a_ptr).set as isize) as
                        *mut set_type;
                text_out_c(5 as libc::c_int as byte_hack,
                           set_text.offset((*set_ptr).desc as isize) as cptr);
                text_out(b"\n\x00" as *const u8 as *const libc::c_char);
            }
        }
        if f4 as libc::c_long & 0x100 as libc::c_long != 0 && trim_down == 0 {
            let mut j: libc::c_int = 0 as libc::c_int;
            if count_bits((*o_ptr).pval3 as u32b) as libc::c_int ==
                   0 as libc::c_int {
                text_out(b"It is sentient\x00" as *const u8 as
                             *const libc::c_char);
            } else if count_bits((*o_ptr).pval3 as u32b) as libc::c_int >
                          1 as libc::c_int {
                text_out(b"It is sentient and can have access to the realms of \x00"
                             as *const u8 as *const libc::c_char);
            } else {
                text_out(b"It is sentient and can have access to the realm of \x00"
                             as *const u8 as *const libc::c_char);
            }
            first = 1 as libc::c_int as bool_;
            txt =
                b"\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            j = 0 as libc::c_int;
            while j < 12 as libc::c_int {
                if (1 as libc::c_long) << j & (*o_ptr).pval3 as libc::c_long
                       != 0 {
                    if first != 0 {
                        first = 0 as libc::c_int as bool_;
                        text_out(txt as cptr);
                    } else {
                        text_out(b", \x00" as *const u8 as
                                     *const libc::c_char);
                    }
                    text_out_c(flags_groups[j as usize].color,
                               flags_groups[j as usize].name.as_mut_ptr() as
                                   cptr);
                }
                j += 1
            }
            text_out(b".  \x00" as *const u8 as *const libc::c_char);
        }
        if f4 as libc::c_long & 0x1000000 as libc::c_long != 0 {
            if wield_slot(o_ptr) as libc::c_int == 24 as libc::c_int ||
                   wield_slot(o_ptr) as libc::c_int == 27 as libc::c_int {
                text_out_c(10 as libc::c_int as byte_hack,
                           b"It is part of the trinity of the ultimate weapons.  \x00"
                               as *const u8 as *const libc::c_char);
            } else {
                text_out_c(10 as libc::c_int as byte_hack,
                           b"It is the ultimate armor.  \x00" as *const u8 as
                               *const libc::c_char);
            }
        }
        if f4 as libc::c_long & 0x40 as libc::c_long != 0 {
            text_out(b"It can be wielded two-handed.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f4 as libc::c_long & 0x80 as libc::c_long != 0 {
            text_out(b"It must be wielded two-handed.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        /* Mega-Hack -- describe activation */
        if f3 as libc::c_long & 0x1000000 as libc::c_long != 0 {
            text_out(b"It can be activated for \x00" as *const u8 as
                         *const libc::c_char);
            if (*o_ptr).name2 as libc::c_int == 4 as libc::c_int ||
                   (*o_ptr).name2b as libc::c_int == 4 as libc::c_int {
                text_out(item_activation(o_ptr,
                                         0 as libc::c_int as byte_hack));
                text_out(b" and \x00" as *const u8 as *const libc::c_char);
                text_out(item_activation(o_ptr,
                                         1 as libc::c_int as byte_hack));
            } else {
                text_out(item_activation(o_ptr,
                                         0 as libc::c_int as byte_hack));
            }
            /* Mega-hack -- get rid of useless line for e.g. randarts */
            if f5 as libc::c_long & 0x2000 as libc::c_long != 0 {
                text_out(b".  \x00" as *const u8 as *const libc::c_char);
            } else {
                text_out(b" if it is being worn. \x00" as *const u8 as
                             *const libc::c_char);
            }
        }
        /* Granted power */
        if object_power(o_ptr) != -(1 as libc::c_int) {
            text_out(b"It grants you the power of \x00" as *const u8 as
                         *const libc::c_char);
            text_out((*powers_type.offset(object_power(o_ptr) as isize)).name
                         as cptr);
            text_out(b" if it is being worn.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        /* Hack -- describe lites */
        if (*o_ptr).tval as libc::c_int == 39 as libc::c_int ||
               f3 as libc::c_long & 0x2000 as libc::c_long != 0 ||
               f4 as libc::c_long & 0x4000000 as libc::c_long != 0 ||
               f4 as libc::c_long & 0x8000000 as libc::c_long != 0 {
            let mut radius: libc::c_int = 0 as libc::c_int;
            if f3 as libc::c_long & 0x2000 as libc::c_long != 0 {
                radius += 1
            }
            if f4 as libc::c_long & 0x4000000 as libc::c_long != 0 {
                radius += 2 as libc::c_int
            }
            if f4 as libc::c_long & 0x8000000 as libc::c_long != 0 {
                radius += 3 as libc::c_int
            }
            if radius > 5 as libc::c_int { radius = 5 as libc::c_int }
            if f4 as libc::c_long & 0x10000000 as libc::c_long != 0 {
                text_out(format(b"It provides light (radius %d) when fueled.  \x00"
                                    as *const u8 as *const libc::c_char,
                                radius) as cptr);
            } else {
                text_out(format(b"It provides light (radius %d) forever.  \x00"
                                    as *const u8 as *const libc::c_char,
                                radius) as cptr);
            }
        }
        /* Mega Hack^3 -- describe the Anchor of Space-time */
        if (*o_ptr).name1 as libc::c_int == 14 as libc::c_int {
            text_out(b"It prevents the space-time continuum from being disrupted.  \x00"
                         as *const u8 as *const libc::c_char);
        }
        if f4 as libc::c_long & 0x20000 as libc::c_long != 0 ||
               f4 as libc::c_long & 0x40000 as libc::c_long != 0 ||
               f4 as libc::c_long & 0x80000 as libc::c_long != 0 ||
               f4 as libc::c_long & 0x100000 as libc::c_long != 0 {
            text_out(b"It generates an antimagic field.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f5 as libc::c_long & 0x800 as libc::c_long != 0 {
            if (*o_ptr).pval2 as libc::c_int == -(1 as libc::c_int) {
                text_out(b"It can be used to store a spell.  \x00" as
                             *const u8 as *const libc::c_char);
            } else {
                text_out(b"It has a spell stored inside.  \x00" as *const u8
                             as *const libc::c_char);
            }
        }
        /* Pick up stat bonuses */
        vn = 0 as libc::c_int;
        if f1 as libc::c_long & 0x1 as libc::c_long != 0 {
            let fresh25 = vn;
            vn = vn + 1;
            vp[fresh25 as usize] =
                b"strength\x00" as *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x2 as libc::c_long != 0 {
            let fresh26 = vn;
            vn = vn + 1;
            vp[fresh26 as usize] =
                b"intelligence\x00" as *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x4 as libc::c_long != 0 {
            let fresh27 = vn;
            vn = vn + 1;
            vp[fresh27 as usize] =
                b"wisdom\x00" as *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x8 as libc::c_long != 0 {
            let fresh28 = vn;
            vn = vn + 1;
            vp[fresh28 as usize] =
                b"dexterity\x00" as *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x10 as libc::c_long != 0 {
            let fresh29 = vn;
            vn = vn + 1;
            vp[fresh29 as usize] =
                b"constitution\x00" as *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x20 as libc::c_long != 0 {
            let fresh30 = vn;
            vn = vn + 1;
            vp[fresh30 as usize] =
                b"charisma\x00" as *const u8 as *const libc::c_char
        }
        if (*o_ptr).tval as libc::c_int != 46 as libc::c_int &&
               f1 as libc::c_long & 0x100 as libc::c_long != 0 {
            let fresh31 = vn;
            vn = vn + 1;
            vp[fresh31 as usize] =
                b"stealth\x00" as *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x200 as libc::c_long != 0 {
            let fresh32 = vn;
            vn = vn + 1;
            vp[fresh32 as usize] =
                b"searching\x00" as *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x400 as libc::c_long != 0 {
            let fresh33 = vn;
            vn = vn + 1;
            vp[fresh33 as usize] =
                b"infravision\x00" as *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x800 as libc::c_long != 0 {
            let fresh34 = vn;
            vn = vn + 1;
            vp[fresh34 as usize] =
                b"ability to tunnel\x00" as *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x1000 as libc::c_long != 0 {
            let fresh35 = vn;
            vn = vn + 1;
            vp[fresh35 as usize] =
                b"speed\x00" as *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x2000 as libc::c_long != 0 {
            let fresh36 = vn;
            vn = vn + 1;
            vp[fresh36 as usize] =
                b"attack speed\x00" as *const u8 as *const libc::c_char
        }
        if f5 as libc::c_long & 0x20 as libc::c_long != 0 {
            let fresh37 = vn;
            vn = vn + 1;
            vp[fresh37 as usize] =
                b"ability to score critical hits\x00" as *const u8 as
                    *const libc::c_char
        }
        if f5 as libc::c_long & 0x200 as libc::c_long != 0 {
            let fresh38 = vn;
            vn = vn + 1;
            vp[fresh38 as usize] =
                b"luck\x00" as *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x80 as libc::c_long != 0 {
            let fresh39 = vn;
            vn = vn + 1;
            vp[fresh39 as usize] =
                b"spell power\x00" as *const u8 as *const libc::c_char
        }
        /* Describe */
        if vn != 0 {
            let mut i: libc::c_int = 0;
            /* Intro */
            text_out(b"It \x00" as *const u8 as *const libc::c_char);
            /* What it does */
            if (*o_ptr).pval > 0 as libc::c_int {
                text_out(b"increases \x00" as *const u8 as
                             *const libc::c_char);
            } else {
                text_out(b"decreases \x00" as *const u8 as
                             *const libc::c_char);
            }
            /* List */
            i = 0 as libc::c_int;
            while i < vn {
                /* Connectives */
                if i == 0 as libc::c_int {
                    text_out(b"your \x00" as *const u8 as
                                 *const libc::c_char);
                } else if i < vn - 1 as libc::c_int {
                    text_out(b", \x00" as *const u8 as *const libc::c_char);
                } else {
                    text_out(b" and \x00" as *const u8 as
                                 *const libc::c_char);
                }
                /* Dump the stat */
                text_out(vp[i as usize]);
                i += 1
            }
            text_out(b" by \x00" as *const u8 as *const libc::c_char);
            if (*o_ptr).pval > 0 as libc::c_int {
                text_out_c(13 as libc::c_int as byte_hack,
                           format(b"%i\x00" as *const u8 as
                                      *const libc::c_char, (*o_ptr).pval) as
                               cptr);
            } else {
                text_out_c(12 as libc::c_int as byte_hack,
                           format(b"%i\x00" as *const u8 as
                                      *const libc::c_char, -(*o_ptr).pval) as
                               cptr);
            }
            text_out(b".  \x00" as *const u8 as *const libc::c_char);
        }
        vn = 0 as libc::c_int;
        if f1 as libc::c_long & 0x40 as libc::c_long != 0 {
            let fresh40 = vn;
            vn = vn + 1;
            vp[fresh40 as usize] =
                b"mana capacity\x00" as *const u8 as *const libc::c_char
        }
        if f2 as libc::c_long & 0x80 as libc::c_long != 0 {
            let fresh41 = vn;
            vn = vn + 1;
            vp[fresh41 as usize] =
                b"hit points\x00" as *const u8 as *const libc::c_char
        }
        /* Describe with percentuals */
        if vn != 0 {
            let mut percent: libc::c_int = 0;
            /* What it does */
            if (*o_ptr).pval > 0 as libc::c_int {
                text_out(b"It increases\x00" as *const u8 as
                             *const libc::c_char);
            } else {
                text_out(b"It decreases\x00" as *const u8 as
                             *const libc::c_char);
            }
            text_out(b" your \x00" as *const u8 as *const libc::c_char);
            text_out(vp[0 as libc::c_int as usize]);
            if vn == 2 as libc::c_int {
                text_out(b" and \x00" as *const u8 as *const libc::c_char);
                text_out(vp[1 as libc::c_int as usize]);
            }
            text_out(b" by \x00" as *const u8 as *const libc::c_char);
            percent =
                100 as libc::c_int * (*o_ptr).pval /
                    (if munchkin_multipliers as libc::c_int != 0 {
                         5 as libc::c_int
                     } else { 10 as libc::c_int });
            if (*o_ptr).pval > 0 as libc::c_int {
                text_out_c(13 as libc::c_int as byte_hack,
                           format(b"%i%%\x00" as *const u8 as
                                      *const libc::c_char, percent) as cptr);
            } else {
                text_out_c(12 as libc::c_int as byte_hack,
                           format(b"%i%%\x00" as *const u8 as
                                      *const libc::c_char, -percent) as cptr);
            }
            text_out(b".  \x00" as *const u8 as *const libc::c_char);
        }
        if (*o_ptr).tval as libc::c_int == 46 as libc::c_int &&
               f1 as libc::c_long & 0x100 as libc::c_long != 0 {
            text_out(b"It is well-hidden. \x00" as *const u8 as
                         *const libc::c_char);
        }
        vn = 0 as libc::c_int;
        if f1 as libc::c_long & 0x10000000 as libc::c_long != 0 {
            vc[vn as usize] = 5 as libc::c_int as byte_hack;
            let fresh42 = vn;
            vn = vn + 1;
            vp[fresh42 as usize] =
                b"acid\x00" as *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x20000000 as libc::c_long != 0 {
            vc[vn as usize] = 14 as libc::c_int as byte_hack;
            let fresh43 = vn;
            vn = vn + 1;
            vp[fresh43 as usize] =
                b"electricity\x00" as *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x40000000 as libc::c_long != 0 {
            vc[vn as usize] = 4 as libc::c_int as byte_hack;
            let fresh44 = vn;
            vn = vn + 1;
            vp[fresh44 as usize] =
                b"fire\x00" as *const u8 as *const libc::c_char
        }
        if f1 as libc::c_long & 0x80000000 as libc::c_long != 0 {
            vc[vn as usize] = 9 as libc::c_int as byte_hack;
            let fresh45 = vn;
            vn = vn + 1;
            vp[fresh45 as usize] =
                b"frost\x00" as *const u8 as *const libc::c_char
        }
        /* Describe */
        if vn != 0 {
            let mut i_0: libc::c_int = 0;
            /* Intro */
            text_out(b"It does extra damage \x00" as *const u8 as
                         *const libc::c_char);
            /* List */
            i_0 = 0 as libc::c_int;
            while i_0 < vn {
                /* Connectives */
                if i_0 == 0 as libc::c_int {
                    text_out(b"from \x00" as *const u8 as
                                 *const libc::c_char);
                } else if i_0 < vn - 1 as libc::c_int {
                    text_out(b", \x00" as *const u8 as *const libc::c_char);
                } else {
                    text_out(b" and \x00" as *const u8 as
                                 *const libc::c_char);
                }
                /* Dump the stat */
                text_out_c(vc[i_0 as usize], vp[i_0 as usize]);
                i_0 += 1
            }
            text_out(b".  \x00" as *const u8 as *const libc::c_char);
        }
        if f1 as libc::c_long & 0x8000000 as libc::c_long != 0 {
            text_out(b"It \x00" as *const u8 as *const libc::c_char);
            text_out_c(13 as libc::c_int as byte_hack,
                       b"poisons your foes\x00" as *const u8 as
                           *const libc::c_char);
            text_out(b".  \x00" as *const u8 as *const libc::c_char);
        }
        if f1 as libc::c_long & 0x4000 as libc::c_long != 0 {
            text_out(b"It produces chaotic effects.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f1 as libc::c_long & 0x8000 as libc::c_long != 0 {
            text_out(b"It drains life from your foes.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f1 as libc::c_long & 0x4000000 as libc::c_long != 0 {
            text_out(b"It can cause earthquakes.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f1 as libc::c_long & 0x2000000 as libc::c_long != 0 {
            text_out(b"It is very sharp and can cut your foes.  \x00" as
                         *const u8 as *const libc::c_char);
        }
        if f5 as libc::c_long & 0x80 as libc::c_long != 0 {
            text_out(b"It is very sharp and can make your foes bleed.  \x00"
                         as *const u8 as *const libc::c_char);
        }
        if f1 as libc::c_long & 0x1000000 as libc::c_long != 0 {
            text_out(b"It is a great bane of dragons.  \x00" as *const u8 as
                         *const libc::c_char);
        } else if f1 as libc::c_long & 0x800000 as libc::c_long != 0 {
            text_out(b"It is especially deadly against dragons.  \x00" as
                         *const u8 as *const libc::c_char);
        }
        if f1 as libc::c_long & 0x100000 as libc::c_long != 0 {
            text_out(b"It is especially deadly against orcs.  \x00" as
                         *const u8 as *const libc::c_char);
        }
        if f1 as libc::c_long & 0x200000 as libc::c_long != 0 {
            text_out(b"It is especially deadly against trolls.  \x00" as
                         *const u8 as *const libc::c_char);
        }
        if f1 as libc::c_long & 0x400000 as libc::c_long != 0 {
            text_out(b"It is especially deadly against giants.  \x00" as
                         *const u8 as *const libc::c_char);
        }
        if f5 as libc::c_long & 0x8 as libc::c_long != 0 {
            text_out(b"It is a great bane of demons.  \x00" as *const u8 as
                         *const libc::c_char);
        } else if f1 as libc::c_long & 0x80000 as libc::c_long != 0 {
            text_out(b"It strikes at demons with holy wrath.  \x00" as
                         *const u8 as *const libc::c_char);
        }
        if f5 as libc::c_long & 0x10 as libc::c_long != 0 {
            text_out(b"It is a great bane of undead.  \x00" as *const u8 as
                         *const libc::c_char);
        } else if f1 as libc::c_long & 0x40000 as libc::c_long != 0 {
            text_out(b"It strikes at undead with holy wrath.  \x00" as
                         *const u8 as *const libc::c_char);
        }
        if f1 as libc::c_long & 0x20000 as libc::c_long != 0 {
            text_out(b"It fights against evil with holy fury.  \x00" as
                         *const u8 as *const libc::c_char);
        }
        if f1 as libc::c_long & 0x10000 as libc::c_long != 0 {
            text_out(b"It is especially deadly against natural creatures.  \x00"
                         as *const u8 as *const libc::c_char);
        }
        if f2 as libc::c_long & 0x40 as libc::c_long != 0 {
            text_out(b"It makes you invisible.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if (*o_ptr).tval as libc::c_int != 46 as libc::c_int {
            vn = 0 as libc::c_int;
            if f2 as libc::c_long & 0x1 as libc::c_long != 0 {
                let fresh46 = vn;
                vn = vn + 1;
                vp[fresh46 as usize] =
                    b"strength\x00" as *const u8 as *const libc::c_char
            }
            if f2 as libc::c_long & 0x2 as libc::c_long != 0 {
                let fresh47 = vn;
                vn = vn + 1;
                vp[fresh47 as usize] =
                    b"intelligence\x00" as *const u8 as *const libc::c_char
            }
            if f2 as libc::c_long & 0x4 as libc::c_long != 0 {
                let fresh48 = vn;
                vn = vn + 1;
                vp[fresh48 as usize] =
                    b"wisdom\x00" as *const u8 as *const libc::c_char
            }
            if f2 as libc::c_long & 0x8 as libc::c_long != 0 {
                let fresh49 = vn;
                vn = vn + 1;
                vp[fresh49 as usize] =
                    b"dexterity\x00" as *const u8 as *const libc::c_char
            }
            if f2 as libc::c_long & 0x10 as libc::c_long != 0 {
                let fresh50 = vn;
                vn = vn + 1;
                vp[fresh50 as usize] =
                    b"constitution\x00" as *const u8 as *const libc::c_char
            }
            if f2 as libc::c_long & 0x20 as libc::c_long != 0 {
                let fresh51 = vn;
                vn = vn + 1;
                vp[fresh51 as usize] =
                    b"charisma\x00" as *const u8 as *const libc::c_char
            }
            /* Describe */
            if vn != 0 {
                let mut i_1: libc::c_int = 0;
                /* Intro */
                text_out(b"It sustains \x00" as *const u8 as
                             *const libc::c_char);
                /* List */
                i_1 = 0 as libc::c_int;
                while i_1 < vn {
                    /* Connectives */
                    if i_1 == 0 as libc::c_int {
                        text_out(b"your \x00" as *const u8 as
                                     *const libc::c_char);
                    } else if i_1 < vn - 1 as libc::c_int {
                        text_out(b", \x00" as *const u8 as
                                     *const libc::c_char);
                    } else {
                        text_out(b" and \x00" as *const u8 as
                                     *const libc::c_char);
                    }
                    /* Dump the stat */
                    text_out(vp[i_1 as usize]);
                    i_1 += 1
                }
                text_out(b".  \x00" as *const u8 as *const libc::c_char);
            }
            vn = 0 as libc::c_int;
            if f2 as libc::c_long & 0x100 as libc::c_long != 0 {
                vc[vn as usize] = 5 as libc::c_int as byte_hack;
                let fresh52 = vn;
                vn = vn + 1;
                vp[fresh52 as usize] =
                    b"acid\x00" as *const u8 as *const libc::c_char
            }
            if f2 as libc::c_long & 0x200 as libc::c_long != 0 {
                vc[vn as usize] = 14 as libc::c_int as byte_hack;
                let fresh53 = vn;
                vn = vn + 1;
                vp[fresh53 as usize] =
                    b"electricity\x00" as *const u8 as *const libc::c_char
            }
            if f2 as libc::c_long & 0x400 as libc::c_long != 0 {
                vc[vn as usize] = 4 as libc::c_int as byte_hack;
                let fresh54 = vn;
                vn = vn + 1;
                vp[fresh54 as usize] =
                    b"fire\x00" as *const u8 as *const libc::c_char
            }
            if f2 as libc::c_long & 0x800 as libc::c_long != 0 {
                vc[vn as usize] = 9 as libc::c_int as byte_hack;
                let fresh55 = vn;
                vn = vn + 1;
                vp[fresh55 as usize] =
                    b"cold\x00" as *const u8 as *const libc::c_char
            }
            if f4 as libc::c_long & 0x400000 as libc::c_long != 0 {
                vc[vn as usize] = 13 as libc::c_int as byte_hack;
                let fresh56 = vn;
                vn = vn + 1;
                vp[fresh56 as usize] =
                    b"nether\x00" as *const u8 as *const libc::c_char
            }
            /* Describe */
            if vn != 0 {
                let mut i_2: libc::c_int = 0;
                /* Intro */
                text_out(b"It provides immunity \x00" as *const u8 as
                             *const libc::c_char);
                /* List */
                i_2 = 0 as libc::c_int;
                while i_2 < vn {
                    /* Connectives */
                    if i_2 == 0 as libc::c_int {
                        text_out(b"to \x00" as *const u8 as
                                     *const libc::c_char);
                    } else if i_2 < vn - 1 as libc::c_int {
                        text_out(b", \x00" as *const u8 as
                                     *const libc::c_char);
                    } else {
                        text_out(b" and \x00" as *const u8 as
                                     *const libc::c_char);
                    }
                    /* Dump the stat */
                    text_out_c(vc[i_2 as usize], vp[i_2 as usize]);
                    i_2 += 1
                }
                text_out(b".  \x00" as *const u8 as *const libc::c_char);
            }
        } else {
            if f2 as libc::c_long & 0x1 as libc::c_long != 0 {
                text_out(b"It can rearm itself.  \x00" as *const u8 as
                             *const libc::c_char);
            }
            if f2 as libc::c_long & 0x2 as libc::c_long != 0 {
                text_out(b"It rearms itself.  \x00" as *const u8 as
                             *const libc::c_char);
            }
            if f2 as libc::c_long & 0x4 as libc::c_long != 0 {
                text_out(b"It is effective against Ghosts.  \x00" as *const u8
                             as *const libc::c_char);
            }
            if f2 as libc::c_long & 0x8 as libc::c_long != 0 {
                text_out(b"It can teleport monsters to you.  \x00" as
                             *const u8 as *const libc::c_char);
            }
            if f2 as libc::c_long & 0x10 as libc::c_long != 0 {
                text_out(b"It can only be set off by dragons.  \x00" as
                             *const u8 as *const libc::c_char);
            }
            if f2 as libc::c_long & 0x20 as libc::c_long != 0 {
                text_out(b"It can only be set off by demons.  \x00" as
                             *const u8 as *const libc::c_char);
            }
            if f2 as libc::c_long & 0x200 as libc::c_long != 0 {
                text_out(b"It can only be set off by undead.  \x00" as
                             *const u8 as *const libc::c_char);
            }
            if f2 as libc::c_long & 0x100 as libc::c_long != 0 {
                text_out(b"It can only be set off by animals.  \x00" as
                             *const u8 as *const libc::c_char);
            }
            if f2 as libc::c_long & 0x400 as libc::c_long != 0 {
                text_out(b"It can only be set off by evil creatures.  \x00" as
                             *const u8 as *const libc::c_char);
            }
        }
        if f2 as libc::c_long & 0x4000 as libc::c_long != 0 {
            text_out(b"It provides immunity to paralysis.  \x00" as *const u8
                         as *const libc::c_char);
        }
        if f2 as libc::c_long & 0x200000 as libc::c_long != 0 {
            text_out(b"It makes you completely fearless.  \x00" as *const u8
                         as *const libc::c_char);
        }
        vn = 0 as libc::c_int;
        if f2 as libc::c_long & 0x8000 as libc::c_long != 0 {
            let fresh57 = vn;
            vn = vn + 1;
            vp[fresh57 as usize] =
                b"life draining\x00" as *const u8 as *const libc::c_char
        }
        if f2 as libc::c_long & 0x10000 as libc::c_long != 0 &&
               f2 as libc::c_long & 0x100 as libc::c_long == 0 {
            let fresh58 = vn;
            vn = vn + 1;
            vp[fresh58 as usize] =
                b"acid\x00" as *const u8 as *const libc::c_char
        }
        if f2 as libc::c_long & 0x20000 as libc::c_long != 0 &&
               f2 as libc::c_long & 0x200 as libc::c_long == 0 {
            let fresh59 = vn;
            vn = vn + 1;
            vp[fresh59 as usize] =
                b"electricity\x00" as *const u8 as *const libc::c_char
        }
        if f2 as libc::c_long & 0x40000 as libc::c_long != 0 &&
               f2 as libc::c_long & 0x400 as libc::c_long == 0 {
            let fresh60 = vn;
            vn = vn + 1;
            vp[fresh60 as usize] =
                b"fire\x00" as *const u8 as *const libc::c_char
        }
        if f2 as libc::c_long & 0x80000 as libc::c_long != 0 &&
               f2 as libc::c_long & 0x800 as libc::c_long == 0 {
            let fresh61 = vn;
            vn = vn + 1;
            vp[fresh61 as usize] =
                b"cold\x00" as *const u8 as *const libc::c_char
        }
        if f2 as libc::c_long & 0x100000 as libc::c_long != 0 {
            let fresh62 = vn;
            vn = vn + 1;
            vp[fresh62 as usize] =
                b"poison\x00" as *const u8 as *const libc::c_char
        }
        if f2 as libc::c_long & 0x400000 as libc::c_long != 0 {
            let fresh63 = vn;
            vn = vn + 1;
            vp[fresh63 as usize] =
                b"light\x00" as *const u8 as *const libc::c_char
        }
        if f2 as libc::c_long & 0x800000 as libc::c_long != 0 {
            let fresh64 = vn;
            vn = vn + 1;
            vp[fresh64 as usize] =
                b"dark\x00" as *const u8 as *const libc::c_char
        }
        if f2 as libc::c_long & 0x1000000 as libc::c_long != 0 {
            let fresh65 = vn;
            vn = vn + 1;
            vp[fresh65 as usize] =
                b"blindness\x00" as *const u8 as *const libc::c_char
        }
        if f2 as libc::c_long & 0x2000000 as libc::c_long != 0 {
            let fresh66 = vn;
            vn = vn + 1;
            vp[fresh66 as usize] =
                b"confusion\x00" as *const u8 as *const libc::c_char
        }
        if f2 as libc::c_long & 0x4000000 as libc::c_long != 0 {
            let fresh67 = vn;
            vn = vn + 1;
            vp[fresh67 as usize] =
                b"sound\x00" as *const u8 as *const libc::c_char
        }
        if f2 as libc::c_long & 0x8000000 as libc::c_long != 0 {
            let fresh68 = vn;
            vn = vn + 1;
            vp[fresh68 as usize] =
                b"shards\x00" as *const u8 as *const libc::c_char
        }
        if f2 as libc::c_long & 0x10000000 as libc::c_long != 0 &&
               f4 as libc::c_long & 0x400000 as libc::c_long == 0 {
            let fresh69 = vn;
            vn = vn + 1;
            vp[fresh69 as usize] =
                b"nether\x00" as *const u8 as *const libc::c_char
        }
        if f2 as libc::c_long & 0x20000000 as libc::c_long != 0 {
            let fresh70 = vn;
            vn = vn + 1;
            vp[fresh70 as usize] =
                b"nexus\x00" as *const u8 as *const libc::c_char
        }
        if f2 as libc::c_long & 0x40000000 as libc::c_long != 0 {
            let fresh71 = vn;
            vn = vn + 1;
            vp[fresh71 as usize] =
                b"chaos\x00" as *const u8 as *const libc::c_char
        }
        if f2 as libc::c_long & 0x80000000 as libc::c_long != 0 {
            let fresh72 = vn;
            vn = vn + 1;
            vp[fresh72 as usize] =
                b"disenchantment\x00" as *const u8 as *const libc::c_char
        }
        /* Describe */
        if vn != 0 {
            let mut i_3: libc::c_int = 0;
            /* Intro */
            text_out(b"It provides resistance \x00" as *const u8 as
                         *const libc::c_char);
            /* List */
            i_3 = 0 as libc::c_int;
            while i_3 < vn {
                /* Connectives */
                if i_3 == 0 as libc::c_int {
                    text_out(b"to \x00" as *const u8 as *const libc::c_char);
                } else if i_3 < vn - 1 as libc::c_int {
                    text_out(b", \x00" as *const u8 as *const libc::c_char);
                } else {
                    text_out(b" and \x00" as *const u8 as
                                 *const libc::c_char);
                }
                /* Dump the stat */
                text_out(vp[i_3 as usize]);
                i_3 += 1
            }
            text_out(b".  \x00" as *const u8 as *const libc::c_char);
        }
        if f2 as libc::c_long & 0x1000 as libc::c_long != 0 {
            text_out(b"It renders you especially vulnerable to fire.  \x00" as
                         *const u8 as *const libc::c_char);
        }
        if f3 as libc::c_long & 0x40 as libc::c_long != 0 {
            text_out(b"It renders you incorporeal.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f5 as libc::c_long & 0x8000 as libc::c_long != 0 {
            text_out(b"It allows you to breathe underwater.  \x00" as
                         *const u8 as *const libc::c_char);
        }
        if f5 as libc::c_long & 0x4000 as libc::c_long != 0 {
            text_out(b"It allows you to breathe without air.  \x00" as
                         *const u8 as *const libc::c_char);
        }
        if f3 as libc::c_long & 0x1000 as libc::c_long != 0 {
            text_out(b"It allows you to levitate.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f4 as libc::c_long & 0x10 as libc::c_long != 0 {
            text_out(b"It allows you to fly.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f4 as libc::c_long & 0x800 as libc::c_long != 0 {
            text_out(b"It allows you to climb mountains.  \x00" as *const u8
                         as *const libc::c_char);
        }
        if f5 as libc::c_long & 0x400 as libc::c_long != 0 {
            text_out(b"It renders you immovable.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f3 as libc::c_long & 0x4000 as libc::c_long != 0 {
            text_out(b"It allows you to see invisible monsters.  \x00" as
                         *const u8 as *const libc::c_char);
        }
        if esp != 0 {
            if esp as libc::c_long & 0x80000000 as libc::c_long != 0 {
                text_out(b"It gives telepathic powers.  \x00" as *const u8 as
                             *const libc::c_char);
            } else {
                vn = 0 as libc::c_int;
                if esp as libc::c_long & 0x1 as libc::c_long != 0 {
                    let fresh73 = vn;
                    vn = vn + 1;
                    vp[fresh73 as usize] =
                        b"orcs\x00" as *const u8 as *const libc::c_char
                }
                if esp as libc::c_long & 0x2 as libc::c_long != 0 {
                    let fresh74 = vn;
                    vn = vn + 1;
                    vp[fresh74 as usize] =
                        b"trolls\x00" as *const u8 as *const libc::c_char
                }
                if esp as libc::c_long & 0x4 as libc::c_long != 0 {
                    let fresh75 = vn;
                    vn = vn + 1;
                    vp[fresh75 as usize] =
                        b"dragons\x00" as *const u8 as *const libc::c_char
                }
                if esp as libc::c_long & 0x1000 as libc::c_long != 0 {
                    let fresh76 = vn;
                    vn = vn + 1;
                    vp[fresh76 as usize] =
                        b"spiders\x00" as *const u8 as *const libc::c_char
                }
                if esp as libc::c_long & 0x8 as libc::c_long != 0 {
                    let fresh77 = vn;
                    vn = vn + 1;
                    vp[fresh77 as usize] =
                        b"giants\x00" as *const u8 as *const libc::c_char
                }
                if esp as libc::c_long & 0x10 as libc::c_long != 0 {
                    let fresh78 = vn;
                    vn = vn + 1;
                    vp[fresh78 as usize] =
                        b"demons\x00" as *const u8 as *const libc::c_char
                }
                if esp as libc::c_long & 0x20 as libc::c_long != 0 {
                    let fresh79 = vn;
                    vn = vn + 1;
                    vp[fresh79 as usize] =
                        b"undead\x00" as *const u8 as *const libc::c_char
                }
                if esp as libc::c_long & 0x40 as libc::c_long != 0 {
                    let fresh80 = vn;
                    vn = vn + 1;
                    vp[fresh80 as usize] =
                        b"evil beings\x00" as *const u8 as *const libc::c_char
                }
                if esp as libc::c_long & 0x80 as libc::c_long != 0 {
                    let fresh81 = vn;
                    vn = vn + 1;
                    vp[fresh81 as usize] =
                        b"animals\x00" as *const u8 as *const libc::c_char
                }
                if esp as libc::c_long & 0x100 as libc::c_long != 0 {
                    let fresh82 = vn;
                    vn = vn + 1;
                    vp[fresh82 as usize] =
                        b"thunderlords\x00" as *const u8 as
                            *const libc::c_char
                }
                if esp as libc::c_long & 0x200 as libc::c_long != 0 {
                    let fresh83 = vn;
                    vn = vn + 1;
                    vp[fresh83 as usize] =
                        b"good beings\x00" as *const u8 as *const libc::c_char
                }
                if esp as libc::c_long & 0x400 as libc::c_long != 0 {
                    let fresh84 = vn;
                    vn = vn + 1;
                    vp[fresh84 as usize] =
                        b"non-living things\x00" as *const u8 as
                            *const libc::c_char
                }
                if esp as libc::c_long & 0x800 as libc::c_long != 0 {
                    let fresh85 = vn;
                    vn = vn + 1;
                    vp[fresh85 as usize] =
                        b"unique beings\x00" as *const u8 as
                            *const libc::c_char
                }
                /* Describe */
                if vn != 0 {
                    let mut i_4: libc::c_int = 0;
                    /* Intro */
                    text_out(b"It allows you to sense the presence \x00" as
                                 *const u8 as *const libc::c_char);
                    /* List */
                    i_4 = 0 as libc::c_int;
                    while i_4 < vn {
                        /* Connectives */
                        if i_4 == 0 as libc::c_int {
                            text_out(b"of \x00" as *const u8 as
                                         *const libc::c_char);
                        } else if i_4 < vn - 1 as libc::c_int {
                            text_out(b", \x00" as *const u8 as
                                         *const libc::c_char);
                        } else {
                            text_out(b" and \x00" as *const u8 as
                                         *const libc::c_char);
                        }
                        /* Dump the stat */
                        text_out(vp[i_4 as usize]);
                        i_4 += 1
                    }
                    text_out(b".  \x00" as *const u8 as *const libc::c_char);
                }
            }
        }
        if f3 as libc::c_long & 0x10000 as libc::c_long != 0 {
            text_out(b"It slows your metabolism.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f3 as libc::c_long & 0x20000 as libc::c_long != 0 {
            text_out(b"It speeds your regenerative powers.  \x00" as *const u8
                         as *const libc::c_char);
        }
        if f2 as libc::c_long & 0x2000 as libc::c_long != 0 {
            text_out(b"It reflects bolts and arrows.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f3 as libc::c_long & 0x1 as libc::c_long != 0 {
            text_out(b"It produces a fiery sheath.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f3 as libc::c_long & 0x2 as libc::c_long != 0 {
            text_out(b"It produces an electric sheath.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f3 as libc::c_long & 0x20 as libc::c_long != 0 {
            text_out(b"It produces an anti-magic shell.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f3 as libc::c_long & 0x10 as libc::c_long != 0 {
            text_out(b"It prevents teleportation.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f3 as libc::c_long & 0x40000 as libc::c_long != 0 {
            text_out(b"It fires missiles with extra might.  \x00" as *const u8
                         as *const libc::c_char);
        }
        if f3 as libc::c_long & 0x80000 as libc::c_long != 0 {
            text_out(b"It fires missiles excessively fast.  \x00" as *const u8
                         as *const libc::c_char);
        }
        vn = 0 as libc::c_int;
        if f5 as libc::c_long & 0x2 as libc::c_long != 0 {
            vc[vn as usize] = 6 as libc::c_int as byte_hack;
            let fresh86 = vn;
            vn = vn + 1;
            vp[fresh86 as usize] =
                b"mana\x00" as *const u8 as *const libc::c_char
        }
        if f5 as libc::c_long & 0x4 as libc::c_long != 0 {
            vc[vn as usize] = 4 as libc::c_int as byte_hack;
            let fresh87 = vn;
            vn = vn + 1;
            vp[fresh87 as usize] =
                b"life\x00" as *const u8 as *const libc::c_char
        }
        if f3 as libc::c_long & 0x2000000 as libc::c_long != 0 {
            vc[vn as usize] = 8 as libc::c_int as byte_hack;
            let fresh88 = vn;
            vn = vn + 1;
            vp[fresh88 as usize] =
                b"experience\x00" as *const u8 as *const libc::c_char
        }
        /* Describe */
        if vn != 0 {
            let mut i_5: libc::c_int = 0;
            /* Intro */
            text_out(b"It \x00" as *const u8 as *const libc::c_char);
            /* List */
            i_5 = 0 as libc::c_int;
            while i_5 < vn {
                /* Connectives */
                if i_5 == 0 as libc::c_int {
                    text_out(b"drains \x00" as *const u8 as
                                 *const libc::c_char);
                } else if i_5 < vn - 1 as libc::c_int {
                    text_out(b", \x00" as *const u8 as *const libc::c_char);
                } else {
                    text_out(b" and \x00" as *const u8 as
                                 *const libc::c_char);
                }
                /* Dump the stat */
                text_out_c(vc[i_5 as usize], vp[i_5 as usize]);
                i_5 += 1
            }
            text_out(b".  \x00" as *const u8 as *const libc::c_char);
        }
        if f3 as libc::c_long & 0x10000000 as libc::c_long != 0 {
            text_out(b"It has been blessed by the gods.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f4 as libc::c_long & 0x2000000 as libc::c_long != 0 {
            text_out(b"It identifies all items for you.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f3 as libc::c_long & 0x4000000 as libc::c_long != 0 {
            text_out(b"It induces random teleportation.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f3 as libc::c_long & 0x8000000 as libc::c_long != 0 {
            text_out(b"It aggravates nearby creatures.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f4 as libc::c_long & 0x1 as libc::c_long != 0 {
            text_out(b"It can\'t attack.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f4 as libc::c_long & 0x4 as libc::c_long != 0 {
            text_out(b"It fills you with the Black Breath.  \x00" as *const u8
                         as *const libc::c_char);
        }
        if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
            if f3 as libc::c_long & 0x80000000 as libc::c_long != 0 {
                text_out(b"It is permanently cursed.  \x00" as *const u8 as
                             *const libc::c_char);
            } else if f3 as libc::c_long & 0x40000000 as libc::c_long != 0 {
                text_out(b"It is heavily cursed.  \x00" as *const u8 as
                             *const libc::c_char);
            } else {
                text_out(b"It is cursed.  \x00" as *const u8 as
                             *const libc::c_char);
            }
        }
        if f3 as libc::c_long & 0x80 as libc::c_long != 0 {
            text_out(b"It carries an ancient foul curse.  \x00" as *const u8
                         as *const libc::c_char);
        }
        if f4 as libc::c_long & 0x20 as libc::c_long != 0 {
            text_out(b"It carries an ancient Morgothian curse.  \x00" as
                         *const u8 as *const libc::c_char);
        }
        if f4 as libc::c_long & 0x200 as libc::c_long != 0 {
            text_out(b"It can clone monsters.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f4 as libc::c_long & 0x40000000 as libc::c_long != 0 {
            text_out(b"It cannot be dropped while cursed.  \x00" as *const u8
                         as *const libc::c_char);
        }
        if f3 as libc::c_long & 0x4 as libc::c_long != 0 {
            text_out(b"It can re-curse itself.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f4 as libc::c_long & 0x2000 as libc::c_long != 0 {
            text_out(b"It can hold more mana.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f4 as libc::c_long & 0x8000 as libc::c_long != 0 {
            text_out(b"It can cast spells for a lesser mana cost.  \x00" as
                         *const u8 as *const libc::c_char);
        }
        if f4 as libc::c_long & 0x1000 as libc::c_long != 0 {
            text_out(b"It can cast spells faster.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f4 as libc::c_long & 0x4000 as libc::c_long != 0 {
            text_out(b"It regenerates its mana faster.  \x00" as *const u8 as
                         *const libc::c_char);
        }
        if f5 as libc::c_long & 0x1000 as libc::c_long != 0 {
            text_out(b"It can resist being shattered by morgul beings.  \x00"
                         as *const u8 as *const libc::c_char);
        }
        if f3 as libc::c_long & 0x100000 as libc::c_long != 0 &&
               f3 as libc::c_long & 0x400000 as libc::c_long != 0 &&
               f3 as libc::c_long & 0x800000 as libc::c_long != 0 &&
               f3 as libc::c_long & 0x200000 as libc::c_long != 0 {
            text_out(b"It cannot be harmed by acid, cold, lightning or fire.  \x00"
                         as *const u8 as *const libc::c_char);
        } else {
            if f3 as libc::c_long & 0x100000 as libc::c_long != 0 {
                text_out(b"It cannot be harmed by acid.  \x00" as *const u8 as
                             *const libc::c_char);
            }
            if f3 as libc::c_long & 0x200000 as libc::c_long != 0 {
                text_out(b"It cannot be harmed by electricity.  \x00" as
                             *const u8 as *const libc::c_char);
            }
            if f3 as libc::c_long & 0x400000 as libc::c_long != 0 {
                text_out(b"It cannot be harmed by fire.  \x00" as *const u8 as
                             *const libc::c_char);
            }
            if f3 as libc::c_long & 0x800000 as libc::c_long != 0 {
                text_out(b"It cannot be harmed by cold.  \x00" as *const u8 as
                             *const libc::c_char);
            }
        }
    }
    if trim_down == 0 && fff.is_null() {
        describe_device(o_ptr);
        if (*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
               (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as
                   libc::c_int != 0 &&
                   (*k_info.offset((*o_ptr).k_idx as isize)).aware as
                       libc::c_int != 0 {
            /* Damage display for weapons */
            if wield_slot(o_ptr) as libc::c_int == 24 as libc::c_int {
                display_weapon_damage(o_ptr);
            }
            /* Breakage/Damage display for boomerangs */
            if (*o_ptr).tval as libc::c_int == 15 as libc::c_int {
                if (*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
                       (if (*o_ptr).name1 as libc::c_int != 0 {
                            1 as libc::c_int
                        } else { 0 as libc::c_int }) != 0 ||
                       (if (*o_ptr).art_name as libc::c_int != 0 {
                            1 as libc::c_int
                        } else { 0 as libc::c_int }) != 0 ||
                       (if (*k_info.offset((*o_ptr).k_idx as isize)).flags3 as
                               libc::c_long & 0x8000 as libc::c_long != 0 {
                            1 as libc::c_int
                        } else { 0 as libc::c_int }) != 0 {
                    text_out(b"\nIt can never be broken.\x00" as *const u8 as
                                 *const libc::c_char);
                } else {
                    text_out(b"\nIt has 1% chance to break upon hit.\x00" as
                                 *const u8 as *const libc::c_char);
                }
                display_ammo_damage(o_ptr);
            }
            /* Breakage/Damage display for ammo */
            if wield_slot(o_ptr) as libc::c_int == 50 as libc::c_int {
                if (*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
                       (if (*o_ptr).name1 as libc::c_int != 0 {
                            1 as libc::c_int
                        } else { 0 as libc::c_int }) != 0 ||
                       (if (*o_ptr).art_name as libc::c_int != 0 {
                            1 as libc::c_int
                        } else { 0 as libc::c_int }) != 0 ||
                       (if (*k_info.offset((*o_ptr).k_idx as isize)).flags3 as
                               libc::c_long & 0x8000 as libc::c_long != 0 {
                            1 as libc::c_int
                        } else { 0 as libc::c_int }) != 0 {
                    text_out(b"\nIt can never be broken.\x00" as *const u8 as
                                 *const libc::c_char);
                } else if (*o_ptr).pval2 as libc::c_int == 0 as libc::c_int {
                    text_out(b"\nIt has \x00" as *const u8 as
                                 *const libc::c_char);
                    text_out_c(12 as libc::c_int as byte_hack,
                               format(b"%d\x00" as *const u8 as
                                          *const libc::c_char,
                                      breakage_chance(o_ptr)) as cptr);
                    text_out(b"% chance to break upon hit.\x00" as *const u8
                                 as *const libc::c_char);
                }
                display_ammo_damage(o_ptr);
            }
            /* Exclude exploding arrows */
            /* Monster recall for totems and corpses */
            if (*o_ptr).tval as libc::c_int == 54 as libc::c_int {
                monster_description_out((*o_ptr).pval, 0 as libc::c_int);
            }
            if (*o_ptr).tval as libc::c_int == 9 as libc::c_int {
                monster_description_out((*o_ptr).pval2 as libc::c_int,
                                        0 as libc::c_int);
            }
        }
        if !((*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
                 (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as
                     libc::c_int != 0 &&
                     (*k_info.offset((*o_ptr).k_idx as isize)).aware as
                         libc::c_int != 0) {
            text_out(b"\nYou might need to identify the item to know some more about it...\x00"
                         as *const u8 as *const libc::c_char);
        } else if (*o_ptr).ident as libc::c_int & 0x20 as libc::c_int == 0 {
            text_out(b"\nYou might need to *identify* the item to know more about it...\x00"
                         as *const u8 as *const libc::c_char);
        }
    }
    /* Copying how others seem to do it. -- neil */
    if (*o_ptr).tval as libc::c_int == 45 as libc::c_int ||
           (*o_ptr).tval as libc::c_int == 40 as libc::c_int || trim_down == 0
           ||
           (if (*o_ptr).name2 as libc::c_int != 0 ||
                   (*o_ptr).name2b as libc::c_int != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int }) != 0 ||
           ((*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
                (if (*o_ptr).name1 as libc::c_int != 0 {
                     1 as libc::c_int
                 } else { 0 as libc::c_int }) != 0 ||
                (if (*o_ptr).art_name as libc::c_int != 0 {
                     1 as libc::c_int
                 } else { 0 as libc::c_int }) != 0 ||
                (if (*k_info.offset((*o_ptr).k_idx as isize)).flags3 as
                        libc::c_long & 0x8000 as libc::c_long != 0 {
                     1 as libc::c_int
                 } else { 0 as libc::c_int }) != 0) {
        /* Where did we found it ? */
        if (*o_ptr).found as libc::c_int == 1 as libc::c_int {
            let mut m_name: [libc::c_char; 80] = [0; 80];
            monster_race_desc(m_name.as_mut_ptr(),
                              (*o_ptr).found_aux1 as libc::c_int,
                              (*o_ptr).found_aux2 as libc::c_int);
            text_out(format(b"\nYou found it in the remains of %s %s.\x00" as
                                *const u8 as *const libc::c_char,
                            m_name.as_mut_ptr(),
                            object_out_desc_where_found((*o_ptr).found_aux4,
                                                        (*o_ptr).found_aux3))
                         as cptr);
        } else if (*o_ptr).found as libc::c_int == 2 as libc::c_int {
            text_out(format(b"\nYou found it lying on the ground %s.\x00" as
                                *const u8 as *const libc::c_char,
                            object_out_desc_where_found((*o_ptr).found_aux2,
                                                        (*o_ptr).found_aux1))
                         as cptr);
        } else if (*o_ptr).found as libc::c_int == 3 as libc::c_int {
            text_out(format(b"\nYou found it lying in a vault %s.\x00" as
                                *const u8 as *const libc::c_char,
                            object_out_desc_where_found((*o_ptr).found_aux2,
                                                        (*o_ptr).found_aux1))
                         as cptr);
        } else if (*o_ptr).found as libc::c_int == 4 as libc::c_int {
            text_out(b"\nYou found it lying on the floor of a special level.\x00"
                         as *const u8 as *const libc::c_char);
        } else if (*o_ptr).found as libc::c_int == 5 as libc::c_int {
            text_out(b"\nYou found it while digging a rubble.\x00" as
                         *const u8 as *const libc::c_char);
        } else if (*o_ptr).found as libc::c_int == 6 as libc::c_int {
            text_out(b"\nIt was given to you as a reward.\x00" as *const u8 as
                         *const libc::c_char);
        } else if (*o_ptr).found as libc::c_int == 7 as libc::c_int {
            text_out(format(b"\nYou bought it from the %s.\x00" as *const u8
                                as *const libc::c_char,
                            st_name.offset((*st_info.offset((*o_ptr).found_aux1
                                                                as
                                                                isize)).name
                                               as isize)) as cptr);
        } else if (*o_ptr).found as libc::c_int == 8 as libc::c_int {
            text_out(format(b"\nYou stole it from the %s.\x00" as *const u8 as
                                *const libc::c_char,
                            st_name.offset((*st_info.offset((*o_ptr).found_aux1
                                                                as
                                                                isize)).name
                                               as isize)) as cptr);
        } else if (*o_ptr).found as libc::c_int == 9 as libc::c_int {
            text_out(b"\nYou made it yourself.\x00" as *const u8 as
                         *const libc::c_char);
        }
        /* useful for debugging
		else
	{
			text_out("\nYou ordered it from a catalog in the Town.");
	}*/
    }
    if !fff.is_null() {
        /* Flush the line position. */
        text_out(b"\n\x00" as *const u8 as *const libc::c_char);
        text_out_file = 0 as *mut FILE
    } else {
        if wait_for_it != 0 {
            /* Wait for it */
            inkey();
            /* Restore the screen */
            Term_load();
        }
        character_icky = 0 as libc::c_int as bool_
    }
    /* Reset stuff for text_out */
    text_out_hook =
        Some(text_out_to_screen as
                 unsafe extern "C" fn(_: byte_hack, _: cptr) -> ());
    /* Gave knowledge */
    return 1 as libc::c_int as bool_;
}
/*
 * Convert an inventory index into a one character label
 * Note that the label does NOT distinguish inven/equip.
 */
#[no_mangle]
pub unsafe extern "C" fn index_to_label(mut i: libc::c_int) -> libc::c_char {
    /* Indexes for "inven" are easy */
    if i < 24 as libc::c_int { return (i + 'a' as i32) as libc::c_char }
    /* Indexes for "equip" are offset */
    return (i - 24 as libc::c_int + 'a' as i32) as libc::c_char;
}
/*
 * Convert a label into the index of an item in the "inven"
 * Return "-1" if the label does not indicate a real item
 */
#[no_mangle]
pub unsafe extern "C" fn label_to_inven(mut c: libc::c_int) -> s16b {
    let mut i: libc::c_int = 0;
    /* Convert */
    i =
        if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
               _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0 {
            (c) - 'a' as i32
        } else { -(1 as libc::c_int) };
    /* Verify the index */
    if i < 0 as libc::c_int || i > 23 as libc::c_int {
        return -(1 as libc::c_int) as s16b
    }
    /* Empty slots can never be chosen */
    if (*p_ptr).inventory[i as usize].k_idx == 0 {
        return -(1 as libc::c_int) as s16b
    }
    /* Return the index */
    return i as s16b;
}
/*
 * Convert a label into the index of a item in the "equip"
 * Return "-1" if the label does not indicate a real item
 */
#[no_mangle]
pub unsafe extern "C" fn label_to_equip(mut c: libc::c_int) -> s16b {
    let mut i: libc::c_int = 0;
    /* Convert */
    i =
        (if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
                || c > 'z' as i32 {
             (c) - 'a' as i32
         } else { -(1 as libc::c_int) }) + 24 as libc::c_int;
    /* Verify the index */
    if i < 24 as libc::c_int || i >= 52 as libc::c_int {
        return -(1 as libc::c_int) as s16b
    }
    /* Empty slots can never be chosen */
    if (*p_ptr).inventory[i as usize].k_idx == 0 {
        return -(1 as libc::c_int) as s16b
    }
    /* Return the index */
    return i as s16b;
}
/*
 * Returns the next free slot of the given "type", return the first
 * if all are used
 */
#[no_mangle]
pub unsafe extern "C" fn get_slot(mut slot: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    /* If there are at least one body part corretsonding, the find the free one */
    if (*p_ptr).body_parts[(slot - 24 as libc::c_int) as usize] as libc::c_int
           == slot {
        /* Find a free body part */
        while i < 6 as libc::c_int && slot + i < 52 as libc::c_int &&
                  (*p_ptr).body_parts[(slot - 24 as libc::c_int + i) as usize]
                      as libc::c_int == slot {
            if !((*p_ptr).body_parts[(slot + i - 24 as libc::c_int) as usize]
                     != 0) {
                break ;
            }
            /* Free ? return the slot */
            if (*p_ptr).inventory[(slot + i) as usize].k_idx == 0 {
                return slot + i
            }
            i += 1
        }
        /* Found nothing ? return the first one */
        return slot
    } else {
        /* No body parts ? return -1 */
        return -(1 as libc::c_int)
    };
}
/*
 * Determine which equipment slot (if any) an item likes, ignoring the player's
 * current body and stuff if ideal == TRUE
 */
#[no_mangle]
pub unsafe extern "C" fn wield_slot_ideal(mut o_ptr: *mut object_type,
                                          mut ideal: bool_) -> s16b {
    /* Try for a script first */
    if process_hooks_ret(46 as libc::c_int,
                         b"d\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char,
                         b"(O,d)\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char, o_ptr, ideal as libc::c_int)
           != 0 {
        return process_hooks_return[0 as libc::c_int as usize].num as s16b
    }
    /* Slot for equipment */
    match (*o_ptr).tval as libc::c_int {
        20 | 12 => {
            return if ideal as libc::c_int != 0 {
                       51 as libc::c_int
                   } else { get_slot(51 as libc::c_int) } as s16b
        }
        21 | 22 | 6 | 23 | 24 => {
            return if ideal as libc::c_int != 0 {
                       24 as libc::c_int
                   } else { get_slot(24 as libc::c_int) } as s16b
        }
        15 | 19 | 14 => {
            return if ideal as libc::c_int != 0 {
                       27 as libc::c_int
                   } else { get_slot(27 as libc::c_int) } as s16b
        }
        45 => {
            return if ideal as libc::c_int != 0 {
                       28 as libc::c_int
                   } else { get_slot(28 as libc::c_int) } as s16b
        }
        40 => {
            return if ideal as libc::c_int != 0 {
                       34 as libc::c_int
                   } else { get_slot(34 as libc::c_int) } as s16b
        }
        39 => {
            return if ideal as libc::c_int != 0 {
                       36 as libc::c_int
                   } else { get_slot(36 as libc::c_int) } as s16b
        }
        38 | 37 | 36 => {
            return if ideal as libc::c_int != 0 {
                       37 as libc::c_int
                   } else { get_slot(37 as libc::c_int) } as s16b
        }
        35 => {
            return if ideal as libc::c_int != 0 {
                       38 as libc::c_int
                   } else { get_slot(38 as libc::c_int) } as s16b
        }
        34 => {
            return if ideal as libc::c_int != 0 {
                       39 as libc::c_int
                   } else { get_slot(39 as libc::c_int) } as s16b
        }
        33 | 32 => {
            return if ideal as libc::c_int != 0 {
                       42 as libc::c_int
                   } else { get_slot(42 as libc::c_int) } as s16b
        }
        31 => {
            return if ideal as libc::c_int != 0 {
                       44 as libc::c_int
                   } else { get_slot(44 as libc::c_int) } as s16b
        }
        30 => {
            return if ideal as libc::c_int != 0 {
                       47 as libc::c_int
                   } else { get_slot(47 as libc::c_int) } as s16b
        }
        99 => {
            return if ideal as libc::c_int != 0 {
                       49 as libc::c_int
                   } else { get_slot(49 as libc::c_int) } as s16b
        }
        16 => {
            if ideal != 0 {
                return 50 as libc::c_int as s16b
            } else {
                if (*p_ptr).inventory[50 as libc::c_int as usize].k_idx as
                       libc::c_int != 0 &&
                       object_similar(o_ptr,
                                      &mut *(*p_ptr).inventory.as_mut_ptr().offset(50
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       isize))
                           as libc::c_int != 0 &&
                       ((*p_ptr).inventory[50 as libc::c_int as usize].number
                            as libc::c_int + (*o_ptr).number as libc::c_int) <
                           100 as libc::c_int {
                    return get_slot(50 as libc::c_int) as s16b
                } else {
                    if (*p_ptr).inventory[27 as libc::c_int as usize].k_idx as
                           libc::c_int != 0 &&
                           (*p_ptr).inventory[27 as libc::c_int as usize].tval
                               as libc::c_int == 19 as libc::c_int {
                        if ((*p_ptr).inventory[27 as libc::c_int as
                                                   usize].sval as libc::c_int)
                               < 10 as libc::c_int {
                            return get_slot(50 as libc::c_int) as s16b
                        }
                    }
                }
            }
            return -(1 as libc::c_int) as s16b
        }
        17 => {
            if ideal != 0 {
                return 50 as libc::c_int as s16b
            } else {
                if (*p_ptr).inventory[50 as libc::c_int as usize].k_idx as
                       libc::c_int != 0 &&
                       object_similar(o_ptr,
                                      &mut *(*p_ptr).inventory.as_mut_ptr().offset(50
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       isize))
                           as libc::c_int != 0 &&
                       ((*p_ptr).inventory[50 as libc::c_int as usize].number
                            as libc::c_int + (*o_ptr).number as libc::c_int) <
                           100 as libc::c_int {
                    return get_slot(50 as libc::c_int) as s16b
                } else {
                    if (*p_ptr).inventory[27 as libc::c_int as usize].k_idx as
                           libc::c_int != 0 &&
                           (*p_ptr).inventory[27 as libc::c_int as usize].tval
                               as libc::c_int == 19 as libc::c_int {
                        if (*p_ptr).inventory[27 as libc::c_int as usize].sval
                               as libc::c_int >= 10 as libc::c_int &&
                               ((*p_ptr).inventory[27 as libc::c_int as
                                                       usize].sval as
                                    libc::c_int) < 20 as libc::c_int {
                            return get_slot(50 as libc::c_int) as s16b
                        }
                    }
                }
            }
            return -(1 as libc::c_int) as s16b
        }
        18 => {
            if ideal != 0 {
                return 50 as libc::c_int as s16b
            } else {
                if (*p_ptr).inventory[50 as libc::c_int as usize].k_idx as
                       libc::c_int != 0 &&
                       object_similar(o_ptr,
                                      &mut *(*p_ptr).inventory.as_mut_ptr().offset(50
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       isize))
                           as libc::c_int != 0 &&
                       ((*p_ptr).inventory[50 as libc::c_int as usize].number
                            as libc::c_int + (*o_ptr).number as libc::c_int) <
                           100 as libc::c_int {
                    return get_slot(50 as libc::c_int) as s16b
                } else {
                    if (*p_ptr).inventory[27 as libc::c_int as usize].k_idx as
                           libc::c_int != 0 &&
                           (*p_ptr).inventory[27 as libc::c_int as usize].tval
                               as libc::c_int == 19 as libc::c_int {
                        if (*p_ptr).inventory[27 as libc::c_int as usize].sval
                               as libc::c_int >= 20 as libc::c_int {
                            return get_slot(50 as libc::c_int) as s16b
                        }
                    }
                }
            }
            return -(1 as libc::c_int) as s16b
        }
        _ => { }
    }
    /* No slot available */
    return -(1 as libc::c_int) as s16b;
}
/*
 * Determine which equipment slot (if any) an item likes for the player's
 * current body and stuff
 */
#[no_mangle]
pub unsafe extern "C" fn wield_slot(mut o_ptr: *mut object_type) -> s16b {
    return wield_slot_ideal(o_ptr, 0 as libc::c_int as bool_);
}
/*
 * Return a string mentioning how a given item is carried
 */
#[no_mangle]
pub unsafe extern "C" fn mention_use(mut i: libc::c_int) -> cptr {
    let mut p: cptr = 0 as *const libc::c_char;
    /* Examine the location */
    match i {
        24 | 25 | 26 => {
            p = b"Wielding\x00" as *const u8 as *const libc::c_char
        }
        27 => { p = b"Shooting\x00" as *const u8 as *const libc::c_char }
        28 | 29 | 30 | 31 | 32 | 33 => {
            p = b"On finger\x00" as *const u8 as *const libc::c_char
        }
        34 | 35 => {
            p = b"Around neck\x00" as *const u8 as *const libc::c_char
        }
        36 => { p = b"Light source\x00" as *const u8 as *const libc::c_char }
        37 => { p = b"On body\x00" as *const u8 as *const libc::c_char }
        38 => { p = b"About body\x00" as *const u8 as *const libc::c_char }
        39 | 40 | 41 => {
            p = b"On arm\x00" as *const u8 as *const libc::c_char
        }
        42 | 43 => { p = b"On head\x00" as *const u8 as *const libc::c_char }
        44 | 45 | 46 => {
            p = b"On hands\x00" as *const u8 as *const libc::c_char
        }
        47 | 48 => { p = b"On feet\x00" as *const u8 as *const libc::c_char }
        49 => { p = b"Symbiote\x00" as *const u8 as *const libc::c_char }
        50 => { p = b"Quiver\x00" as *const u8 as *const libc::c_char }
        51 => { p = b"Using\x00" as *const u8 as *const libc::c_char }
        _ => { p = b"In pack\x00" as *const u8 as *const libc::c_char }
    }
    /* Hack -- Heavy weapons */
    if 24 as libc::c_int <= i && i <= 24 as libc::c_int + 2 as libc::c_int {
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        if (*adj_str_hold.as_mut_ptr().offset((*p_ptr).stat_ind[0 as
                                                                    libc::c_int
                                                                    as usize]
                                                  as isize) as libc::c_int) <
               (*o_ptr).weight / 10 as libc::c_int {
            p = b"Just lifting\x00" as *const u8 as *const libc::c_char
        }
    }
    /* Hack -- music instruments and heavy bow */
    if i == 27 as libc::c_int {
        let mut o_ptr_0: *mut object_type = 0 as *mut object_type;
        o_ptr_0 =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        if (*o_ptr_0).tval as libc::c_int == 14 as libc::c_int {
            p = b"Playing\x00" as *const u8 as *const libc::c_char
        } else if (*adj_str_hold.as_mut_ptr().offset((*p_ptr).stat_ind[0 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                         as isize) as
                       libc::c_int) < (*o_ptr_0).weight / 10 as libc::c_int {
            p = b"Just holding\x00" as *const u8 as *const libc::c_char
        }
    }
    /* Return the result */
    return p;
}
/*
 * Return a string describing how a given item is being worn.
 * Currently, only used for items in the equipment, not inventory.
 */
#[no_mangle]
pub unsafe extern "C" fn describe_use(mut i: libc::c_int) -> cptr {
    let mut p: cptr = 0 as *const libc::c_char;
    match i {
        24 | 25 | 26 => {
            p =
                b"attacking monsters with\x00" as *const u8 as
                    *const libc::c_char
        }
        27 => {
            p =
                b"shooting missiles with\x00" as *const u8 as
                    *const libc::c_char
        }
        28 | 29 | 30 | 31 | 32 | 33 => {
            p =
                b"wearing on your finger\x00" as *const u8 as
                    *const libc::c_char
        }
        34 | 35 => {
            p =
                b"wearing around your neck\x00" as *const u8 as
                    *const libc::c_char
        }
        36 => {
            p =
                b"using to light the way\x00" as *const u8 as
                    *const libc::c_char
        }
        37 => {
            p =
                b"wearing on your body\x00" as *const u8 as
                    *const libc::c_char
        }
        38 => {
            p =
                b"wearing on your back\x00" as *const u8 as
                    *const libc::c_char
        }
        39 | 40 | 41 => {
            p = b"wearing on your arm\x00" as *const u8 as *const libc::c_char
        }
        42 | 43 => {
            p =
                b"wearing on your head\x00" as *const u8 as
                    *const libc::c_char
        }
        44 | 45 | 46 => {
            p =
                b"wearing on your hands\x00" as *const u8 as
                    *const libc::c_char
        }
        47 | 48 => {
            p =
                b"wearing on your feet\x00" as *const u8 as
                    *const libc::c_char
        }
        49 => {
            p = b"in symbiosis with\x00" as *const u8 as *const libc::c_char
        }
        50 => {
            p =
                b"carrying in your quiver\x00" as *const u8 as
                    *const libc::c_char
        }
        51 => {
            p = b"using as a tool\x00" as *const u8 as *const libc::c_char
        }
        _ => {
            p =
                b"carrying in your pack\x00" as *const u8 as
                    *const libc::c_char
        }
    }
    /* Hack -- Heavy weapons */
    if 24 as libc::c_int <= i && i <= 24 as libc::c_int + 2 as libc::c_int {
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        if (*adj_str_hold.as_mut_ptr().offset((*p_ptr).stat_ind[0 as
                                                                    libc::c_int
                                                                    as usize]
                                                  as isize) as libc::c_int) <
               (*o_ptr).weight / 10 as libc::c_int {
            p = b"just lifting\x00" as *const u8 as *const libc::c_char
        }
    }
    /* Hack -- Music instruments and heavy bow */
    if i == 27 as libc::c_int {
        let mut o_ptr_0: *mut object_type = 0 as *mut object_type;
        o_ptr_0 =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        if (*o_ptr_0).tval as libc::c_int == 14 as libc::c_int {
            p = b"playing music with\x00" as *const u8 as *const libc::c_char
        } else if (*adj_str_hold.as_mut_ptr().offset((*p_ptr).stat_ind[0 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                         as isize) as
                       libc::c_int) < (*o_ptr_0).weight / 10 as libc::c_int {
            p = b"just holding\x00" as *const u8 as *const libc::c_char
        }
    }
    /* Return the result */
    return p;
}
/*
 * Check an item against the item tester info
 */
#[no_mangle]
pub unsafe extern "C" fn item_tester_okay(mut o_ptr: *mut object_type)
 -> bool_ {
    /* Hack -- allow listing empty slots */
    if item_tester_full != 0 { return 1 as libc::c_int as bool_ }
    /* Require an item */
    if (*o_ptr).k_idx == 0 { return 0 as libc::c_int as bool_ }
    /* Hack -- ignore "gold" */
    if (*o_ptr).tval as libc::c_int == 100 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Check the tval */
    if item_tester_tval != 0 {
        if !(item_tester_tval as libc::c_int == (*o_ptr).tval as libc::c_int)
           {
            return 0 as libc::c_int as bool_
        }
    }
    /* Check the hook */
    if item_tester_hook.is_some() {
        if Some(item_tester_hook.expect("non-null function pointer")).expect("non-null function pointer")(o_ptr)
               == 0 {
            return 0 as libc::c_int as bool_
        }
    }
    /* Assume okay */
    return 1 as libc::c_int as bool_;
}
/*
 * Choice window "shadow" of the "show_inven()" function
 */
#[no_mangle]
pub unsafe extern "C" fn display_inven() {
    show_inven_aux(1 as libc::c_int as bool_, inventory_no_move);
}
/*
 * Choice window "shadow" of the "show_equip()" function
 */
#[no_mangle]
pub unsafe extern "C" fn display_equip() {
    show_equip_aux(1 as libc::c_int as bool_, inventory_no_move);
}
/* Get the color of the letter idx */
#[no_mangle]
pub unsafe extern "C" fn get_item_letter_color(mut o_ptr: *mut object_type)
 -> byte_hack {
    let mut color: byte_hack = 1 as libc::c_int as byte_hack;
    /* Must have knowlegde */
    if !((*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
             (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as
                 libc::c_int != 0 &&
                 (*k_info.offset((*o_ptr).k_idx as isize)).aware as
                     libc::c_int != 0) {
        return 2 as libc::c_int as byte_hack
    }
    if if (*o_ptr).name2 as libc::c_int != 0 ||
              (*o_ptr).name2b as libc::c_int != 0 {
           1 as libc::c_int
       } else { 0 as libc::c_int } != 0 {
        color = 14 as libc::c_int as byte_hack
    }
    if (*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
           (if (*o_ptr).name1 as libc::c_int != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int }) != 0 ||
           (if (*o_ptr).art_name as libc::c_int != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int }) != 0 ||
           (if (*k_info.offset((*o_ptr).k_idx as isize)).flags3 as
                   libc::c_long & 0x8000 as libc::c_long != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int }) != 0 {
        color = 11 as libc::c_int as byte_hack
    }
    if (*o_ptr).name1 as libc::c_int != 0 &&
           -(1 as libc::c_int) !=
               (*a_info.offset((*o_ptr).name1 as isize)).set as libc::c_int {
        color = 5 as libc::c_int as byte_hack
    }
    if (*o_ptr).name1 as libc::c_int != 0 &&
           (*a_info.offset((*o_ptr).name1 as isize)).flags4 as libc::c_long &
               0x1000000 as libc::c_long != 0 &&
           (*o_ptr).ident as libc::c_int & 0x20 as libc::c_int != 0 {
        color = 10 as libc::c_int as byte_hack
    }
    return color;
}
/*
 * Display the inventory.
 *
 * Hack -- do not display "trailing" empty slots
 */
#[no_mangle]
pub unsafe extern "C" fn show_inven_aux(mut mirror: bool_,
                                        mut everything: bool_) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut z: libc::c_int = 0 as libc::c_int;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut lim: libc::c_int = 0;
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut tmp_val: [libc::c_char; 80] = [0; 80];
    let mut out_index: [libc::c_int; 23] = [0; 23];
    let mut out_color: [byte_hack; 23] = [0; 23];
    let mut out_desc: [[libc::c_char; 80]; 23] = [[0; 80]; 23];
    /* Retrive current screen size */
    Term_get_size(&mut wid, &mut hgt);
    /* Starting row */
    row =
        if mirror as libc::c_int != 0 {
            0 as libc::c_int
        } else { 1 as libc::c_int };
    /* Starting column */
    col =
        if mirror as libc::c_int != 0 {
            0 as libc::c_int
        } else { 50 as libc::c_int };
    /* Default "max-length" */
    len = 79 as libc::c_int - col;
    /* Maximum space allowed for descriptions */
    lim = 79 as libc::c_int - 3 as libc::c_int;
    /* Require space for weight (if needed) */
    if show_weights != 0 { lim -= 9 as libc::c_int }
    /* Require space for icon */
    if show_inven_graph != 0 { lim -= 2 as libc::c_int }
    /* Find the "final" slot */
    i = 0 as libc::c_int;
    while i < 23 as libc::c_int {
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Skip non-objects */
        if !((*o_ptr).k_idx == 0) {
            /* Track */
            z = i + 1 as libc::c_int
        }
        i += 1
    }
    let mut current_block_29: u64;
    /* Display the inventory */
    k = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < z {
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Is this item acceptable? */
        if item_tester_okay(o_ptr) == 0 {
            if everything == 0 {
                current_block_29 = 12124785117276362961;
            } else {
                out_index[k as usize] = -i - 1 as libc::c_int;
                current_block_29 = 11057878835866523405;
            }
        } else {
            /* Save the object index */
            out_index[k as usize] = i + 1 as libc::c_int;
            current_block_29 = 11057878835866523405;
        }
        match current_block_29 {
            11057878835866523405 => {
                /* Describe the object */
                object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                            3 as libc::c_int);
                /* Hack -- enforce max length */
                o_name[lim as usize] = '\u{0}' as i32 as libc::c_char;
                /* Save the object color, and description */
                out_color[k as usize] =
                    tval_to_attr[((*o_ptr).tval as libc::c_int %
                                      128 as libc::c_int) as usize];
                strcpy(out_desc[k as usize].as_mut_ptr(),
                       o_name.as_mut_ptr());
                /* Find the predicted "line length" */
                l =
                    strlen(out_desc[k as
                                        usize].as_mut_ptr()).wrapping_add(5 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong)
                        as libc::c_int;
                /* Be sure to account for the weight */
                if show_weights != 0 { l += 9 as libc::c_int }
                /* Account for icon if displayed */
                if show_inven_graph != 0 { l += 2 as libc::c_int }
                /* Maintain the maximum length */
                if l > len { len = l }
                /* Advance to next "line" */
                k += 1
            }
            _ => { }
        }
        i += 1
    }
    /* Find the column to start in */
    if mirror != 0 {
        col = 0 as libc::c_int
    } else {
        col =
            if len > wid - 4 as libc::c_int {
                0 as libc::c_int
            } else { (wid - len) - 1 as libc::c_int }
    }
    /* Output each entry */
    j = 0 as libc::c_int;
    while j < k {
        /* Get the index */
        i = out_index[j as usize];
        /* Get the item */
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(((if i <
                                                                  0 as
                                                                      libc::c_int
                                                              {
                                                               -i
                                                           } else { i }) -
                                                              1 as
                                                                  libc::c_int)
                                                             as isize) as
                *mut object_type;
        /* Clear the line */
        prt(b"\x00" as *const u8 as *const libc::c_char, row + j,
            if col != 0 { (col) - 2 as libc::c_int } else { col });
        /* Prepare an index --(-- */
		/* Prepare a blank index if not selectable */
        if i > 0 as libc::c_int {
            sprintf(tmp_val.as_mut_ptr(),
                    b"%c)\x00" as *const u8 as *const libc::c_char,
                    index_to_label(i - 1 as libc::c_int) as libc::c_int);
        } else {
            sprintf(tmp_val.as_mut_ptr(),
                    b"  \x00" as *const u8 as *const libc::c_char);
        }
        /* Clear the line with the (possibly indented) index */
        c_put_str(get_item_letter_color(o_ptr), tmp_val.as_mut_ptr() as cptr,
                  row + j, col);
        /* Display graphics for object, if desired */
        if show_inven_graph != 0 {
            let mut a: byte_hack =
                if (*o_ptr).tval as libc::c_int == 102 as libc::c_int {
                    random_artifacts[(*o_ptr).sval as usize].attr as
                        libc::c_int
                } else if (*k_info.offset((*o_ptr).k_idx as isize)).flavor as
                              libc::c_int != 0 {
                    misc_to_attr[(*k_info.offset((*o_ptr).k_idx as
                                                     isize)).flavor as usize]
                        as libc::c_int
                } else {
                    (*k_info.offset((*o_ptr).k_idx as isize)).x_attr as
                        libc::c_int
                } as byte_hack;
            let mut c: libc::c_char =
                if (*k_info.offset((*o_ptr).k_idx as isize)).flavor as
                       libc::c_int != 0 {
                    misc_to_char[(*k_info.offset((*o_ptr).k_idx as
                                                     isize)).flavor as usize]
                        as libc::c_int
                } else {
                    (*k_info.offset((*o_ptr).k_idx as isize)).x_char as
                        libc::c_int
                } as libc::c_char;
            if (*o_ptr).k_idx == 0 { c = ' ' as i32 as libc::c_char }
            Term_draw(col + 3 as libc::c_int, row + j, a, c);
        }
        /* Display the entry itself */
        c_put_str(out_color[j as usize],
                  out_desc[j as usize].as_mut_ptr() as cptr, row + j,
                  if show_inven_graph as libc::c_int != 0 {
                      (col) + 5 as libc::c_int
                  } else { (col) + 3 as libc::c_int });
        /* Display the weight if needed */
        if show_weights != 0 {
            let mut wgt: libc::c_int =
                (*o_ptr).weight * (*o_ptr).number as libc::c_int;
            sprintf(tmp_val.as_mut_ptr(),
                    b"%3d.%1d lb\x00" as *const u8 as *const libc::c_char,
                    wgt / 10 as libc::c_int, wgt % 10 as libc::c_int);
            put_str(tmp_val.as_mut_ptr() as cptr, row + j,
                    wid - 9 as libc::c_int);
        }
        j += 1
    }
    /* Shadow windows */
    if mirror != 0 {
        /* Erase the rest of the window */
        j = row + k;
        while j < (*Term).hgt as libc::c_int {
            /* Erase the line */
            Term_erase(0 as libc::c_int, j, 255 as libc::c_int);
            j += 1
        }
    } else if j != 0 && j < 23 as libc::c_int {
        prt(b"\x00" as *const u8 as *const libc::c_char, row + j,
            if col != 0 { (col) - 2 as libc::c_int } else { col });
    };
}
#[no_mangle]
pub unsafe extern "C" fn show_inven() {
    show_inven_aux(0 as libc::c_int as bool_, 0 as libc::c_int as bool_);
}
#[no_mangle]
pub unsafe extern "C" fn show_equip() {
    show_equip_aux(0 as libc::c_int as bool_, 0 as libc::c_int as bool_);
}
/* Main window */
/* Make a "shadow" below the list (only if needed) */
/*
 * Display the equipment.
 */
#[no_mangle]
pub unsafe extern "C" fn show_equip_aux(mut mirror: bool_,
                                        mut everything: bool_) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut lim: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut tmp_val: [libc::c_char; 80] = [0; 80];
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut out_index: [libc::c_int; 27] = [0; 27];
    let mut out_rindex: [libc::c_int; 27] = [0; 27];
    let mut out_color: [byte_hack; 27] = [0; 27];
    let mut out_desc: [[libc::c_char; 80]; 27] = [[0; 80]; 27];
    /* Retrive current screen size */
    Term_get_size(&mut wid, &mut hgt);
    /* Starting row */
    row =
        if mirror as libc::c_int != 0 {
            0 as libc::c_int
        } else { 1 as libc::c_int };
    /* Starting column */
    col =
        if mirror as libc::c_int != 0 {
            0 as libc::c_int
        } else { 50 as libc::c_int };
    /* Maximal length */
    len = 79 as libc::c_int - col;
    /* Maximum space allowed for descriptions */
    lim = 79 as libc::c_int - 3 as libc::c_int;
    /* Require space for labels (if needed) */
    if show_labels != 0 { lim -= 14 as libc::c_int + 2 as libc::c_int }
    /* Require space for weight (if needed) */
    if show_weights != 0 { lim -= 9 as libc::c_int }
    if show_equip_graph != 0 { lim -= 2 as libc::c_int }
    /* Scan the equipment list */
    idx = 0 as libc::c_int;
    let mut current_block_49: u64;
    k = 0 as libc::c_int;
    i = 24 as libc::c_int;
    while i < 52 as libc::c_int {
        /* Is there actualy a body part here ? */
        if !((*p_ptr).body_parts[(i - 24 as libc::c_int) as usize] == 0) {
            /* Mega Hack -- don't show symbiote slot if we can't use it */
            if !(i == 49 as libc::c_int && get_skill(8 as libc::c_int) == 0) {
                o_ptr =
                    &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize)
                        as *mut object_type;
                /* Inform the player that he/she can't use a shield */
                if (*p_ptr).body_parts[(i - 24 as libc::c_int) as usize] as
                       libc::c_int == 39 as libc::c_int && (*o_ptr).k_idx == 0
                       &&
                       (*p_ptr).inventory[(i - 39 as libc::c_int +
                                               24 as libc::c_int) as
                                              usize].k_idx as libc::c_int != 0
                   {
                    let mut f1: u32b = 0;
                    let mut f2: u32b = 0;
                    let mut f3: u32b = 0;
                    let mut f4: u32b = 0;
                    let mut f5: u32b = 0;
                    let mut esp: u32b = 0;
                    let mut q_ptr: *mut object_type =
                        &mut *(*p_ptr).inventory.as_mut_ptr().offset((i -
                                                                          39
                                                                              as
                                                                              libc::c_int
                                                                          +
                                                                          24
                                                                              as
                                                                              libc::c_int)
                                                                         as
                                                                         isize)
                            as *mut object_type;
                    let mut q_name: [libc::c_char; 80] = [0; 80];
                    /* Description */
                    object_desc(q_name.as_mut_ptr(), q_ptr, 1 as libc::c_int,
                                3 as libc::c_int);
                    /* Get weapon flags */
                    object_flags(q_ptr, &mut f1, &mut f2, &mut f3, &mut f4,
                                 &mut f5, &mut esp);
                    if f4 as libc::c_long & 0x80 as libc::c_long != 0 {
                        sprintf(o_name.as_mut_ptr(),
                                b"(two handed) %s\x00" as *const u8 as
                                    *const libc::c_char, q_name.as_mut_ptr());
                        /* Truncate the description */
                        o_name[lim as usize] =
                            0 as libc::c_int as libc::c_char;
                        /* Save the index */
                        out_index[k as usize] = idx;
                        out_rindex[k as usize] = i;
                        idx += 1;
                        /* Save the color */
                        out_color[k as usize] =
                            12 as libc::c_int as byte_hack;
                        strcpy(out_desc[k as usize].as_mut_ptr(),
                               o_name.as_mut_ptr());
                        current_block_49 = 15976848397966268834;
                    } else { current_block_49 = 14136749492126903395; }
                } else { current_block_49 = 14136749492126903395; }
                match current_block_49 {
                    15976848397966268834 => { }
                    _ => {
                        if (*p_ptr).body_parts[(i - 24 as libc::c_int) as
                                                   usize] as libc::c_int ==
                               24 as libc::c_int && (*o_ptr).k_idx == 0 {
                            sprintf(o_name.as_mut_ptr(),
                                    b"(%s)\x00" as *const u8 as
                                        *const libc::c_char,
                                    melee_names[get_melee_skill() as usize]);
                            /* Truncate the description */
                            o_name[lim as usize] =
                                0 as libc::c_int as libc::c_char;
                            /* Save the index */
                            out_index[k as usize] = idx;
                            out_rindex[k as usize] = i;
                            idx += 1;
                            /* Save the color */
                            out_color[k as usize] =
                                14 as libc::c_int as byte_hack;
                            strcpy(out_desc[k as usize].as_mut_ptr(),
                                   o_name.as_mut_ptr());
                            current_block_49 = 12556861819962772176;
                        } else {
                            idx += 1;
                            /* Description */
                            object_desc(o_name.as_mut_ptr(), o_ptr,
                                        1 as libc::c_int, 3 as libc::c_int);
                            /* Truncate the description */
                            o_name[lim as usize] =
                                0 as libc::c_int as libc::c_char;
                            /* Is this item acceptable? */
                            if item_tester_okay(o_ptr) == 0 {
                                if everything == 0 {
                                    current_block_49 = 15976848397966268834;
                                } else {
                                    out_index[k as usize] =
                                        -(1 as libc::c_int);
                                    current_block_49 = 6476622998065200121;
                                }
                            } else {
                                /* Save the index */
                                out_index[k as usize] = idx;
                                current_block_49 = 6476622998065200121;
                            }
                            match current_block_49 {
                                15976848397966268834 => { }
                                _ => {
                                    out_rindex[k as usize] = i;
                                    /* Save the color */
                                    out_color[k as usize] =
                                        tval_to_attr[((*o_ptr).tval as
                                                          libc::c_int %
                                                          128 as libc::c_int)
                                                         as usize];
                                    strcpy(out_desc[k as usize].as_mut_ptr(),
                                           o_name.as_mut_ptr());
                                    current_block_49 = 12556861819962772176;
                                }
                            }
                        }
                        match current_block_49 {
                            15976848397966268834 => { }
                            _ => {
                                /* Extract the maximal length (see below) */
                                l =
                                    strlen(out_desc[k as
                                                        usize].as_mut_ptr()).wrapping_add((2
                                                                                               as
                                                                                               libc::c_int
                                                                                               +
                                                                                               3
                                                                                                   as
                                                                                                   libc::c_int)
                                                                                              as
                                                                                              libc::c_ulong)
                                        as libc::c_int;
                                /* Increase length for labels (if needed) */
                                if show_labels != 0 {
                                    l += 14 as libc::c_int + 2 as libc::c_int
                                }
                                /* Increase length for weight (if needed) */
                                if show_weights != 0 { l += 9 as libc::c_int }
                                if show_equip_graph != 0 {
                                    l += 2 as libc::c_int
                                }
                                /* Maintain the max-length */
                                if l > len { len = l }
                                /* Advance the entry */
                                k += 1
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    /* Hack -- Find a column to start in */
    if mirror != 0 {
        col = 0 as libc::c_int
    } else {
        col =
            if len > wid - 4 as libc::c_int {
                0 as libc::c_int
            } else { (wid - len) - 1 as libc::c_int }
    }
    /* Output each entry */
    j = 0 as libc::c_int;
    while j < k {
        if j > 20 as libc::c_int { break ; }
        /* Get the index */
        i = out_index[j as usize];
        /* Get the item */
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(*out_rindex.as_mut_ptr().offset(j
                                                                                             as
                                                                                             isize)
                                                             as isize) as
                *mut object_type;
        /* Clear the line */
        prt(b"\x00" as *const u8 as *const libc::c_char, row + j,
            if col != 0 { (col) - 2 as libc::c_int } else { col });
        /* Prepare an index --(-- */
        if out_index[j as usize] >= 0 as libc::c_int {
            sprintf(tmp_val.as_mut_ptr(),
                    b"%c)\x00" as *const u8 as *const libc::c_char,
                    index_to_label(out_rindex[j as usize]) as libc::c_int);
        } else {
            sprintf(tmp_val.as_mut_ptr(),
                    b"  \x00" as *const u8 as *const libc::c_char);
        }
        /* Clear the line with the (possibly indented) index */
        c_put_str(get_item_letter_color(o_ptr), tmp_val.as_mut_ptr() as cptr,
                  row + j, col);
        if show_equip_graph != 0 {
            let mut a: byte_hack =
                if (*o_ptr).tval as libc::c_int == 102 as libc::c_int {
                    random_artifacts[(*o_ptr).sval as usize].attr as
                        libc::c_int
                } else if (*k_info.offset((*o_ptr).k_idx as isize)).flavor as
                              libc::c_int != 0 {
                    misc_to_attr[(*k_info.offset((*o_ptr).k_idx as
                                                     isize)).flavor as usize]
                        as libc::c_int
                } else {
                    (*k_info.offset((*o_ptr).k_idx as isize)).x_attr as
                        libc::c_int
                } as byte_hack;
            let mut c: libc::c_char =
                if (*k_info.offset((*o_ptr).k_idx as isize)).flavor as
                       libc::c_int != 0 {
                    misc_to_char[(*k_info.offset((*o_ptr).k_idx as
                                                     isize)).flavor as usize]
                        as libc::c_int
                } else {
                    (*k_info.offset((*o_ptr).k_idx as isize)).x_char as
                        libc::c_int
                } as libc::c_char;
            if (*o_ptr).k_idx == 0 { c = ' ' as i32 as libc::c_char }
            Term_draw(col + 3 as libc::c_int, row + j, a, c);
        }
        /* Use labels */
        if show_labels != 0 {
            /* Mention the use */
            sprintf(tmp_val.as_mut_ptr(),
                    b"%-14s: \x00" as *const u8 as *const libc::c_char,
                    mention_use(out_rindex[j as usize]));
            put_str(tmp_val.as_mut_ptr() as cptr, row + j,
                    if show_equip_graph as libc::c_int != 0 {
                        (col) + 5 as libc::c_int
                    } else { (col) + 3 as libc::c_int });
            /* Display the entry itself */
            c_put_str(out_color[j as usize],
                      out_desc[j as usize].as_mut_ptr() as cptr, row + j,
                      if show_equip_graph as libc::c_int != 0 {
                          (col) + 21 as libc::c_int
                      } else { (col) + 19 as libc::c_int });
        } else {
            /* No labels */
            /* Display the entry itself */
            c_put_str(out_color[j as usize],
                      out_desc[j as usize].as_mut_ptr() as cptr, row + j,
                      if show_equip_graph as libc::c_int != 0 {
                          (col) + 5 as libc::c_int
                      } else { (col) + 3 as libc::c_int });
        }
        /* Display the weight if needed */
        if show_weights != 0 {
            let mut wgt: libc::c_int =
                (*o_ptr).weight * (*o_ptr).number as libc::c_int;
            sprintf(tmp_val.as_mut_ptr(),
                    b"%3d.%d lb\x00" as *const u8 as *const libc::c_char,
                    wgt / 10 as libc::c_int, wgt % 10 as libc::c_int);
            put_str(tmp_val.as_mut_ptr() as cptr, row + j,
                    wid - 9 as libc::c_int);
        }
        j += 1
    }
    /* Shadow windows */
    if mirror != 0 {
        /* Erase the rest of the window */
        j = row + k;
        while j < (*Term).hgt as libc::c_int {
            /* Erase the line */
            Term_erase(0 as libc::c_int, j, 255 as libc::c_int);
            j += 1
        }
    } else if j != 0 && j < 23 as libc::c_int {
        prt(b"\x00" as *const u8 as *const libc::c_char, row + j,
            if col != 0 { (col) - 2 as libc::c_int } else { col });
    };
}
/* Main window */
/* Make a "shadow" below the list (only if needed) */
/*
 * Flip "inven" and "equip" in any sub-windows
 */
#[no_mangle]
pub unsafe extern "C" fn toggle_inven_equip() {
    let mut j: libc::c_int = 0;
    /* Scan windows */
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        /* Unused */
        if !angband_term[j as usize].is_null() {
            /* Flip inven to equip */
            if window_flag[j as usize] as libc::c_long & 0x1 as libc::c_long
                   != 0 {
                /* Flip flags */
                window_flag[j as usize] =
                    (window_flag[j as usize] as libc::c_long &
                         !(0x1 as libc::c_long)) as u32b;
                window_flag[j as usize] =
                    (window_flag[j as usize] as libc::c_long |
                         0x2 as libc::c_long) as u32b;
                /* Window stuff */
                (*p_ptr).window =
                    ((*p_ptr).window as libc::c_long | 0x2 as libc::c_long) as
                        u32b
            } else if window_flag[j as usize] as libc::c_long &
                          0x2 as libc::c_long != 0 {
                /* Flip inven to equip */
                /* Flip flags */
                window_flag[j as usize] =
                    (window_flag[j as usize] as libc::c_long &
                         !(0x2 as libc::c_long)) as u32b;
                window_flag[j as usize] =
                    (window_flag[j as usize] as libc::c_long |
                         0x1 as libc::c_long) as u32b;
                /* Window stuff */
                (*p_ptr).window =
                    ((*p_ptr).window as libc::c_long | 0x1 as libc::c_long) as
                        u32b
            }
        }
        j += 1
    };
}
/*
 * Verify the choice of an item.
 *
 * The item can be negative to mean "item on floor".
 */
#[no_mangle]
pub unsafe extern "C" fn verify(mut prompt: cptr, mut item: libc::c_int)
 -> bool_ {
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut out_val: [libc::c_char; 160] = [0; 160];
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    /* Get object */
    o_ptr = get_object(item);
    /* Describe */
    object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                3 as libc::c_int);
    /* Prompt */
    sprintf(out_val.as_mut_ptr(),
            b"%s %s? \x00" as *const u8 as *const libc::c_char, prompt,
            o_name.as_mut_ptr());
    /* Query */
    return get_check(out_val.as_mut_ptr() as cptr);
}
/*
 * Hack -- allow user to "prevent" certain choices
 *
 * The item can be negative to mean "item on floor".
 */
unsafe extern "C" fn get_item_allow(mut item: libc::c_int) -> bool_ {
    let mut s: cptr = 0 as *const libc::c_char;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    /* Get object */
    o_ptr = get_object(item);
    /* No inscription */
    if (*o_ptr).note == 0 { return 1 as libc::c_int as bool_ }
    /* Find a '!' */
    s = strchr(quark_str((*o_ptr).note as s16b), '!' as i32) as cptr;
    /* Process preventions */
    while !s.is_null() {
        /* Check the "restriction" */
        if *s.offset(1 as libc::c_int as isize) as libc::c_int ==
               command_cmd as libc::c_int ||
               *s.offset(1 as libc::c_int as isize) as libc::c_int ==
                   '*' as i32 {
            /* Verify the choice */
            if verify(b"Really try\x00" as *const u8 as *const libc::c_char,
                      item) == 0 {
                return 0 as libc::c_int as bool_
            }
        }
        /* Find another '!' */
        s = strchr(s.offset(1 as libc::c_int as isize), '!' as i32) as cptr
    }
    /* Allow it */
    return 1 as libc::c_int as bool_;
}
/*
 * Auxiliary function for "get_item()" -- test an index
 */
unsafe extern "C" fn get_item_okay(mut i: libc::c_int) -> bool_ {
    /* Illegal items */
    if i < 0 as libc::c_int || i >= 52 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Verify the item */
    if item_tester_okay(&mut *(*p_ptr).inventory.as_mut_ptr().offset(i as
                                                                         isize))
           == 0 {
        return 0 as libc::c_int as bool_
    }
    /* Assume okay */
    return 1 as libc::c_int as bool_;
}
/*
 * Find the "first" inventory object with the given "tag".
 *
 * A "tag" is a char "n" appearing as "@n" anywhere in the
 * inscription of an object.
 *
 * Also, the tag "@xn" will work as well, where "n" is a tag-char,
 * and "x" is the "current" command_cmd code.
 */
unsafe extern "C" fn get_tag(mut cp: *mut libc::c_int, mut tag: libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Check every object */
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Skip non-objects */
        if !((*o_ptr).k_idx == 0) {
            /* Skip empty inscriptions */
            if !((*o_ptr).note == 0) {
                /* Find a '@' */
                s =
                    strchr(quark_str((*o_ptr).note as s16b), '@' as i32) as
                        cptr;
                /* Process all tags */
                while !s.is_null() {
                    /* Check the normal tags */
                    if *s.offset(1 as libc::c_int as isize) as libc::c_int ==
                           tag as libc::c_int {
                        /* Save the actual inventory ID */
                        *cp = i;
                        /* Success */
                        return 1 as libc::c_int
                    }
                    /* Check the special tags */
                    if *s.offset(1 as libc::c_int as isize) as libc::c_int ==
                           command_cmd as libc::c_int &&
                           *s.offset(2 as libc::c_int as isize) as libc::c_int
                               == tag as libc::c_int {
                        /* Save the actual inventory ID */
                        *cp = i;
                        /* Success */
                        return 1 as libc::c_int
                    }
                    /* Find another '@' */
                    s =
                        strchr(s.offset(1 as libc::c_int as isize),
                               '@' as i32) as cptr
                }
            }
        }
        i += 1
    }
    /* No such tag */
    return 0 as libc::c_int;
}
/*
 * scan_floor --
 *
 * Return a list of o_list[] indexes of items at the given cave
 * location. Valid flags are:
 *
 *            mode & 0x01 -- Item tester
 *            mode & 0x02 -- Marked items only
 *            mode & 0x04 -- Stop after first
 */
#[no_mangle]
pub unsafe extern "C" fn scan_floor(mut items: *mut libc::c_int,
                                    mut item_num: *mut libc::c_int,
                                    mut y: libc::c_int, mut x: libc::c_int,
                                    mut mode: libc::c_int) -> bool_ {
    let mut this_o_idx: libc::c_int = 0;
    let mut next_o_idx: libc::c_int = 0;
    let mut num: libc::c_int = 0 as libc::c_int;
    *item_num = 0 as libc::c_int;
    /* Sanity */
    if !(y > 0 as libc::c_int && x > 0 as libc::c_int &&
             y < cur_hgt as libc::c_int - 1 as libc::c_int &&
             x < cur_wid as libc::c_int - 1 as libc::c_int) {
        return 0 as libc::c_int as bool_
    }
    /* Scan all objects in the grid */
    this_o_idx = (*cave[y as usize].offset(x as isize)).o_idx as libc::c_int;
    while this_o_idx != 0 {
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
        /* Acquire object */
        o_ptr = &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
        /* Acquire next object */
        next_o_idx = (*o_ptr).next_o_idx as libc::c_int;
        /* Item tester */
        if !(mode & 0x1 as libc::c_int != 0 && item_tester_okay(o_ptr) == 0) {
            /* Marked */
            if !(mode & 0x2 as libc::c_int != 0 && (*o_ptr).marked == 0) {
                /* Accept this item */
                let fresh89 = num;
                num = num + 1;
                *items.offset(fresh89 as isize) = this_o_idx;
                /* Only one */
                if mode & 0x4 as libc::c_int != 0 { break ; }
                /* XXX Hack -- Enforce limit */
                if num == 23 as libc::c_int { break ; }
            }
        }
        this_o_idx = next_o_idx
    }
    /* Number of items */
    *item_num = num;
    /* Result */
    return (num != 0 as libc::c_int) as libc::c_int as bool_;
}
/*
 * Display a list of the items on the floor at the given location.
 */
#[no_mangle]
pub unsafe extern "C" fn show_floor(mut y: libc::c_int, mut x: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut lim: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut tmp_val: [libc::c_char; 80] = [0; 80];
    let mut out_index: [libc::c_int; 23] = [0; 23];
    let mut out_color: [byte_hack; 23] = [0; 23];
    let mut out_desc: [[libc::c_char; 80]; 23] = [[0; 80]; 23];
    let mut floor_list: [libc::c_int; 23] = [0; 23];
    let mut floor_num: libc::c_int = 0;
    /* Default length */
    len = 79 as libc::c_int - 50 as libc::c_int;
    /* Maximum space allowed for descriptions */
    lim = 79 as libc::c_int - 3 as libc::c_int;
    /* Require space for weight (if needed) */
    if show_weights != 0 { lim -= 9 as libc::c_int }
    /* Scan for objects in the grid, using item_tester_okay() */
    scan_floor(floor_list.as_mut_ptr(), &mut floor_num, y, x,
               0x1 as libc::c_int);
    /* Display the inventory */
    k = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < floor_num {
        o_ptr =
            &mut *o_list.offset(*floor_list.as_mut_ptr().offset(i as isize) as
                                    isize) as *mut object_type;
        /* Describe the object */
        object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                    3 as libc::c_int);
        /* Hack -- enforce max length */
        o_name[lim as usize] = '\u{0}' as i32 as libc::c_char;
        /* Save the index */
        out_index[k as usize] = i;
        /* Acquire inventory color */
        out_color[k as usize] =
            tval_to_attr[((*o_ptr).tval as libc::c_int & 0x7f as libc::c_int)
                             as usize];
        /* Save the object description */
        strcpy(out_desc[k as usize].as_mut_ptr(), o_name.as_mut_ptr());
        /* Find the predicted "line length" */
        l =
            strlen(out_desc[k as
                                usize].as_mut_ptr()).wrapping_add(5 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong)
                as libc::c_int;
        /* Be sure to account for the weight */
        if show_weights != 0 { l += 9 as libc::c_int }
        /* Maintain the maximum length */
        if l > len { len = l }
        /* Advance to next "line" */
        k += 1;
        i += 1
    }
    /* Find the column to start in */
    col =
        if len > 76 as libc::c_int {
            0 as libc::c_int
        } else { (79 as libc::c_int) - len };
    /* Output each entry */
    j = 0 as libc::c_int;
    while j < k {
        /* Get the index */
        i = floor_list[out_index[j as usize] as usize];
        /* Get the item */
        o_ptr = &mut *o_list.offset(i as isize) as *mut object_type;
        /* Clear the line */
        prt(b"\x00" as *const u8 as *const libc::c_char, j + 1 as libc::c_int,
            if col != 0 { (col) - 2 as libc::c_int } else { col });
        /* Prepare an index --(-- */
        sprintf(tmp_val.as_mut_ptr(),
                b"%c)\x00" as *const u8 as *const libc::c_char,
                index_to_label(j) as libc::c_int);
        /* Clear the line with the (possibly indented) index */
        put_str(tmp_val.as_mut_ptr() as cptr, j + 1 as libc::c_int, col);
        /* Display the entry itself */
        c_put_str(out_color[j as usize],
                  out_desc[j as usize].as_mut_ptr() as cptr,
                  j + 1 as libc::c_int, col + 3 as libc::c_int);
        /* Display the weight if needed */
        if show_weights != 0 {
            let mut wgt: libc::c_int =
                (*o_ptr).weight * (*o_ptr).number as libc::c_int;
            sprintf(tmp_val.as_mut_ptr(),
                    b"%3d.%1d lb\x00" as *const u8 as *const libc::c_char,
                    wgt / 10 as libc::c_int, wgt % 10 as libc::c_int);
            put_str(tmp_val.as_mut_ptr() as cptr, j + 1 as libc::c_int,
                    71 as libc::c_int);
        }
        j += 1
    }
    /* Make a "shadow" below the list (only if needed) */
    if j != 0 && j < 23 as libc::c_int {
        prt(b"\x00" as *const u8 as *const libc::c_char, j + 1 as libc::c_int,
            if col != 0 { (col) - 2 as libc::c_int } else { col });
    };
}
/*
 * This version of get_item() is called by get_item() when
 * the easy_floor is on.
 */
#[no_mangle]
pub static mut get_item_extra_hook:
           Option<unsafe extern "C" fn(_: *mut libc::c_int) -> bool_> =
    None;
#[no_mangle]
pub unsafe extern "C" fn get_item_floor(mut cp: *mut libc::c_int,
                                        mut pmt: cptr, mut str: cptr,
                                        mut mode: libc::c_int) -> bool_ {
    let mut n1: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut n2: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut which: libc::c_char = ' ' as i32 as libc::c_char;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut i2: libc::c_int = 0;
    let mut e1: libc::c_int = 0;
    let mut e2: libc::c_int = 0;
    let mut done: bool_ = 0;
    let mut item: bool_ = 0;
    let mut oops: bool_ = 0 as libc::c_int as bool_;
    let mut equip: bool_ = 0 as libc::c_int as bool_;
    let mut inven: bool_ = 0 as libc::c_int as bool_;
    let mut floor: bool_ = 0 as libc::c_int as bool_;
    let mut extra: bool_ = 0 as libc::c_int as bool_;
    let mut automat: bool_ = 0 as libc::c_int as bool_;
    let mut allow_equip: bool_ = 0 as libc::c_int as bool_;
    let mut allow_inven: bool_ = 0 as libc::c_int as bool_;
    let mut allow_floor: bool_ = 0 as libc::c_int as bool_;
    let mut toggle: bool_ = 0 as libc::c_int as bool_;
    let mut tmp_val: [libc::c_char; 160] = [0; 160];
    let mut out_val: [libc::c_char; 160] = [0; 160];
    let mut floor_num: libc::c_int = 0;
    let mut floor_list: [libc::c_int; 23] = [0; 23];
    let mut floor_top: libc::c_int = 0 as libc::c_int;
    k = 0 as libc::c_int;
    /* Get the item index */
    if repeat_pull(cp) != 0 {
        /* Floor item? */
        if *cp < 0 as libc::c_int {
            let mut o_ptr: *mut object_type = 0 as *mut object_type;
            /* Special index */
            k = 0 as libc::c_int - *cp;
            /* Acquire object */
            o_ptr = &mut *o_list.offset(k as isize) as *mut object_type;
            /* Validate the item */
            if item_tester_okay(o_ptr) != 0 {
                /* Forget the item_tester_tval restriction */
                item_tester_tval = 0 as libc::c_int as byte_hack;
                /* Forget the item_tester_hook restriction */
                item_tester_hook = None;
                /* Success */
                return 1 as libc::c_int as bool_
            }
        } else if get_item_okay(*cp) != 0 {
            /* Verify the item */
            /* Forget the item_tester_tval restriction */
            item_tester_tval = 0 as libc::c_int as byte_hack;
            /* Forget the item_tester_hook restriction */
            item_tester_hook = None;
            /* Success */
            return 1 as libc::c_int as bool_
        }
    }
    /* Extract args */
    if mode & 0x1 as libc::c_int != 0 { equip = 1 as libc::c_int as bool_ }
    if mode & 0x2 as libc::c_int != 0 { inven = 1 as libc::c_int as bool_ }
    if mode & 0x4 as libc::c_int != 0 { floor = 1 as libc::c_int as bool_ }
    if mode & 0x8 as libc::c_int != 0 { extra = 1 as libc::c_int as bool_ }
    if mode & 0x10 as libc::c_int != 0 { automat = 1 as libc::c_int as bool_ }
    /* Paranoia XXX XXX XXX */
    msg_print(0 as cptr);
    /* Not done */
    done = 0 as libc::c_int as bool_;
    /* No item selected */
    item = 0 as libc::c_int as bool_;
    /* Full inventory */
    i1 = 0 as libc::c_int;
    i2 = 23 as libc::c_int - 1 as libc::c_int;
    /* Forbid inventory */
    if inven == 0 { i2 = -(1 as libc::c_int) }
    /* Restrict inventory indexes */
    while i1 <= i2 && get_item_okay(i1) == 0 { i1 += 1 }
    while i1 <= i2 && get_item_okay(i2) == 0 { i2 -= 1 }
    /* Full equipment */
    e1 = 24 as libc::c_int;
    e2 = 52 as libc::c_int - 1 as libc::c_int;
    /* Forbid equipment */
    if equip == 0 { e2 = -(1 as libc::c_int) }
    /* Restrict equipment indexes */
    while e1 <= e2 && get_item_okay(e1) == 0 { e1 += 1 }
    while e1 <= e2 && get_item_okay(e2) == 0 { e2 -= 1 }
    /* Count "okay" floor items */
    floor_num = 0 as libc::c_int;
    /* Restrict floor usage */
    if floor != 0 {
        /* Scan all objects in the grid */
        scan_floor(floor_list.as_mut_ptr(), &mut floor_num,
                   (*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                   0x1 as libc::c_int);
    }
    /* Accept inventory */
    if i1 <= i2 { allow_inven = 1 as libc::c_int as bool_ }
    /* Accept equipment */
    if e1 <= e2 { allow_equip = 1 as libc::c_int as bool_ }
    /* Accept floor */
    if floor_num != 0 { allow_floor = 1 as libc::c_int as bool_ }
    /* Require at least one legal choice */
    if allow_inven == 0 && allow_equip == 0 && allow_floor == 0 {
        /* Oops */
        oops = 1 as libc::c_int as bool_;
        /* Done */
        done = 1 as libc::c_int as bool_
    } else if command_wrk as libc::c_int == 0x1 as libc::c_int &&
                  allow_equip as libc::c_int != 0 {
        command_wrk = 0x1 as libc::c_int as s16b
    } else if allow_inven != 0 {
        command_wrk = 0x2 as libc::c_int as s16b
    } else if allow_equip != 0 {
        command_wrk = 0x1 as libc::c_int as s16b
    } else if allow_floor != 0 { command_wrk = 0x4 as libc::c_int as s16b }
    /* Analyze choices */
    /* Hack -- Start on equipment if requested */
    /* Use inventory if allowed */
    /* Use equipment if allowed */
    /* Use floor if allowed */
    /* Save screen */
    screen_save();
    /* Repeat until done */
    while done == 0 {
        /* Show choices */
        if show_choices != 0 {
            let mut ni: libc::c_int = 0 as libc::c_int;
            let mut ne: libc::c_int = 0 as libc::c_int;
            /* Scan windows */
            j = 0 as libc::c_int;
            while j < 8 as libc::c_int {
                /* Unused */
                if !angband_term[j as usize].is_null() {
                    /* Count windows displaying inven */
                    if window_flag[j as usize] as libc::c_long &
                           0x1 as libc::c_long != 0 {
                        ni += 1
                    }
                    /* Count windows displaying equip */
                    if window_flag[j as usize] as libc::c_long &
                           0x2 as libc::c_long != 0 {
                        ne += 1
                    }
                }
                j += 1
            }
            /* Toggle if needed */
            if command_wrk as libc::c_int == 0x1 as libc::c_int && ni != 0 &&
                   ne == 0 ||
                   command_wrk as libc::c_int == 0x2 as libc::c_int && ni == 0
                       && ne != 0 {
                /* Toggle */
                toggle_inven_equip();
                /* Track toggles */
                toggle = (toggle == 0) as libc::c_int as bool_
            }
            /* Update */
            (*p_ptr).window =
                ((*p_ptr).window as libc::c_long |
                     (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
            /* Redraw windows */
            window_stuff();
        }
        /* Inventory screen */
        if command_wrk as libc::c_int == 0x2 as libc::c_int {
            /* Extract the legal requests */
            n1 = (i1 + 'a' as i32) as libc::c_char;
            n2 = (i2 + 'a' as i32) as libc::c_char;
            /* Redraw */
            show_inven();
        } else if command_wrk as libc::c_int == 0x1 as libc::c_int {
            /* Equipment screen */
            /* Extract the legal requests */
            n1 = (e1 - 24 as libc::c_int + 'a' as i32) as libc::c_char;
            n2 = (e2 - 24 as libc::c_int + 'a' as i32) as libc::c_char;
            /* Redraw */
            show_equip();
        } else if command_wrk as libc::c_int == 0x4 as libc::c_int {
            j = floor_top;
            k =
                (if floor_top + 23 as libc::c_int > floor_num {
                     floor_num
                 } else { (floor_top) + 23 as libc::c_int }) -
                    1 as libc::c_int;
            /* Floor screen */
            /* Extract the legal requests */
            n1 = (j - floor_top + 'a' as i32) as libc::c_char;
            n2 = (k - floor_top + 'a' as i32) as libc::c_char;
            /* Redraw */
            show_floor((*p_ptr).py as libc::c_int,
                       (*p_ptr).px as libc::c_int);
        }
        /* Viewing inventory */
        if command_wrk as libc::c_int == 0x2 as libc::c_int {
            /* Begin the prompt */
            sprintf(out_val.as_mut_ptr(),
                    b"Inven:\x00" as *const u8 as *const libc::c_char);
            /* Build the prompt */
            sprintf(tmp_val.as_mut_ptr(),
                    b" %c-%c,\x00" as *const u8 as *const libc::c_char,
                    index_to_label(i1) as libc::c_int,
                    index_to_label(i2) as libc::c_int);
            /* Append */
            strcat(out_val.as_mut_ptr(), tmp_val.as_mut_ptr());
            /* Append */
            if allow_equip != 0 {
                strcat(out_val.as_mut_ptr(),
                       b" / for Equip,\x00" as *const u8 as
                           *const libc::c_char);
            }
            /* Append */
            if allow_floor != 0 {
                strcat(out_val.as_mut_ptr(),
                       b" - for floor,\x00" as *const u8 as
                           *const libc::c_char);
            }
        } else if command_wrk as libc::c_int == 0x1 as libc::c_int {
            /* Viewing equipment */
            /* Begin the prompt */
            sprintf(out_val.as_mut_ptr(),
                    b"Equip:\x00" as *const u8 as *const libc::c_char);
            /* Build the prompt */
            sprintf(tmp_val.as_mut_ptr(),
                    b" %c-%c,\x00" as *const u8 as *const libc::c_char,
                    index_to_label(e1) as libc::c_int,
                    index_to_label(e2) as libc::c_int);
            /* Append */
            strcat(out_val.as_mut_ptr(), tmp_val.as_mut_ptr());
            /* Append */
            if allow_inven != 0 {
                strcat(out_val.as_mut_ptr(),
                       b" / for Inven,\x00" as *const u8 as
                           *const libc::c_char);
            }
            /* Append */
            if allow_floor != 0 {
                strcat(out_val.as_mut_ptr(),
                       b" - for floor,\x00" as *const u8 as
                           *const libc::c_char);
            }
        } else if command_wrk as libc::c_int == 0x4 as libc::c_int {
            /* Viewing floor */
            /* Begin the prompt */
            sprintf(out_val.as_mut_ptr(),
                    b"Floor:\x00" as *const u8 as *const libc::c_char);
            /* Build the prompt */
            sprintf(tmp_val.as_mut_ptr(),
                    b" %c-%c,\x00" as *const u8 as *const libc::c_char,
                    n1 as libc::c_int, n2 as libc::c_int);
            /* Append */
            strcat(out_val.as_mut_ptr(), tmp_val.as_mut_ptr());
            /* Append */
            if allow_inven != 0 {
                strcat(out_val.as_mut_ptr(),
                       b" / for Inven,\x00" as *const u8 as
                           *const libc::c_char);
            } else if allow_equip != 0 {
                strcat(out_val.as_mut_ptr(),
                       b" / for Equip,\x00" as *const u8 as
                           *const libc::c_char);
            }
        }
        /* Extra? */
        if extra != 0 {
            strcat(out_val.as_mut_ptr(),
                   b" @ for extra selection,\x00" as *const u8 as
                       *const libc::c_char);
        }
        /* Create automatizer rule?? */
        if automat != 0 {
            if automatizer_create != 0 {
                strcat(out_val.as_mut_ptr(),
                       b" $ new automatizer rule(ON),\x00" as *const u8 as
                           *const libc::c_char);
            } else {
                strcat(out_val.as_mut_ptr(),
                       b" $ new automatizer rule(OFF),\x00" as *const u8 as
                           *const libc::c_char);
            }
        }
        /* Finish the prompt */
        strcat(out_val.as_mut_ptr(),
               b" ESC\x00" as *const u8 as *const libc::c_char);
        /* Build the prompt */
        sprintf(tmp_val.as_mut_ptr(),
                b"(%s) %s\x00" as *const u8 as *const libc::c_char,
                out_val.as_mut_ptr(), pmt);
        /* Show the prompt */
        prt(tmp_val.as_mut_ptr() as cptr, 0 as libc::c_int, 0 as libc::c_int);
        /* Get a key */
        which = inkey();
        let mut current_block_208: u64;
        /* Parse it */
        match which as libc::c_int {
            27 => { done = 1 as libc::c_int as bool_ }
            47 => {
                if command_wrk as libc::c_int == 0x2 as libc::c_int {
                    if allow_equip == 0 {
                        bell();
                        current_block_208 = 8554725522516090488;
                    } else {
                        command_wrk = 0x1 as libc::c_int as s16b;
                        current_block_208 = 9657096306311191688;
                    }
                } else if command_wrk as libc::c_int == 0x1 as libc::c_int {
                    if allow_inven == 0 {
                        bell();
                        current_block_208 = 8554725522516090488;
                    } else {
                        command_wrk = 0x2 as libc::c_int as s16b;
                        current_block_208 = 9657096306311191688;
                    }
                } else if command_wrk as libc::c_int == 0x4 as libc::c_int {
                    if allow_inven != 0 {
                        command_wrk = 0x2 as libc::c_int as s16b;
                        current_block_208 = 9657096306311191688;
                    } else if allow_equip != 0 {
                        command_wrk = 0x1 as libc::c_int as s16b;
                        current_block_208 = 9657096306311191688;
                    } else {
                        bell();
                        current_block_208 = 8554725522516090488;
                    }
                } else { current_block_208 = 9657096306311191688; }
                match current_block_208 {
                    8554725522516090488 => { }
                    _ => {
                        /* Hack -- Fix screen */
                        screen_load();
                        screen_save();
                    }
                }
            }
            45 => {
                if allow_floor == 0 {
                    bell();
                } else {
                    /*
				 * If we are already examining the floor, and there
				 * is only one item, we will always select it.
				 * If we aren't examining the floor and there is only
				 * one item, we will select it if floor_query_flag
				 * is FALSE.
				 */
                    if floor_num == 1 as libc::c_int {
                        if command_wrk as libc::c_int == 0x4 as libc::c_int {
                            /* Special index */
                            k =
                                0 as libc::c_int -
                                    floor_list[0 as libc::c_int as usize];
                            /* Allow player to "refuse" certain actions */
                            if get_item_allow(k) == 0 {
                                done = 1 as libc::c_int as bool_
                            } else {
                                /* Accept that choice */
                                *cp = k;
                                item = 1 as libc::c_int as bool_;
                                done = 1 as libc::c_int as bool_
                            }
                            current_block_208 = 8554725522516090488;
                        } else { current_block_208 = 13796196565926322821; }
                    } else { current_block_208 = 13796196565926322821; }
                    match current_block_208 {
                        8554725522516090488 => { }
                        _ => {
                            /* Hack -- Fix screen */
                            screen_load();
                            screen_save();
                            command_wrk = 0x4 as libc::c_int as s16b
                        }
                    }
                }
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                /* Look up the tag */
                if get_tag(&mut k, which) == 0 {
                    bell();
                } else if if k < 24 as libc::c_int {
                              (inven == 0) as libc::c_int
                          } else { (equip == 0) as libc::c_int } != 0 {
                    bell();
                } else if get_item_okay(k) == 0 {
                    bell();
                } else if get_item_allow(k) == 0 {
                    done = 1 as libc::c_int as bool_
                } else {
                    /* Hack -- Validate the item */
                    /* Validate the item */
                    /* Allow player to "refuse" certain actions */
                    /* Accept that choice */
                    *cp = k;
                    item = 1 as libc::c_int as bool_;
                    done = 1 as libc::c_int as bool_
                }
            }
            10 | 13 => {
                /* Choose "default" inventory item */
                if command_wrk as libc::c_int == 0x2 as libc::c_int {
                    k = if i1 == i2 { i1 } else { -(1 as libc::c_int) };
                    current_block_208 = 6497888915984600225;
                } else if command_wrk as libc::c_int == 0x1 as libc::c_int {
                    k = if e1 == e2 { e1 } else { -(1 as libc::c_int) };
                    current_block_208 = 6497888915984600225;
                } else if command_wrk as libc::c_int == 0x4 as libc::c_int {
                    if floor_num == 1 as libc::c_int {
                        /* Choose "default" equipment item */
                        /* Choose "default" floor item */
                        /* Special index */
                        k =
                            0 as libc::c_int -
                                floor_list[0 as libc::c_int as usize];
                        /* Allow player to "refuse" certain actions */
                        if get_item_allow(k) == 0 {
                            done = 1 as libc::c_int as bool_
                        } else {
                            /* Accept that choice */
                            *cp = k;
                            item = 1 as libc::c_int as bool_;
                            done = 1 as libc::c_int as bool_
                        }
                        current_block_208 = 8554725522516090488;
                    } else { current_block_208 = 8554725522516090488; }
                } else { current_block_208 = 6497888915984600225; }
                match current_block_208 {
                    8554725522516090488 => { }
                    _ =>
                    /* Validate the item */
                    {
                        if get_item_okay(k) == 0 {
                            bell();
                        } else if get_item_allow(k) == 0 {
                            done = 1 as libc::c_int as bool_
                        } else {
                            /* Allow player to "refuse" certain actions */
                            /* Accept that choice */
                            *cp = k;
                            item = 1 as libc::c_int as bool_;
                            done = 1 as libc::c_int as bool_
                        }
                    }
                }
            }
            64 => {
                let mut i: libc::c_int = 0;
                if extra as libc::c_int != 0 &&
                       get_item_extra_hook.expect("non-null function pointer")(&mut i)
                           as libc::c_int != 0 {
                    *cp = i;
                    item = 1 as libc::c_int as bool_;
                    done = 1 as libc::c_int as bool_
                }
            }
            36 => {
                automatizer_create =
                    (automatizer_create == 0) as libc::c_int as bool_
            }
            _ => {
                let mut ver: libc::c_int = 0;
                ver =
                    *(*__ctype_b_loc()).offset(which as libc::c_int as isize)
                        as libc::c_int &
                        _ISupper as libc::c_int as libc::c_ushort as
                            libc::c_int;
                which = tolower(which as libc::c_int) as libc::c_char;
                /* Convert letter to inventory index */
                if command_wrk as libc::c_int == 0x2 as libc::c_int {
                    k = label_to_inven(which as libc::c_int) as libc::c_int;
                    if k == -(1 as libc::c_int) {
                        bell();
                        current_block_208 = 8554725522516090488;
                    } else { current_block_208 = 15455430299222214173; }
                } else if command_wrk as libc::c_int == 0x1 as libc::c_int {
                    k = label_to_equip(which as libc::c_int) as libc::c_int;
                    if k == -(1 as libc::c_int) {
                        bell();
                        current_block_208 = 8554725522516090488;
                    } else { current_block_208 = 15455430299222214173; }
                } else if command_wrk as libc::c_int == 0x4 as libc::c_int {
                    k =
                        if *(*__ctype_b_loc()).offset(which as libc::c_int as
                                                          isize) as
                               libc::c_int &
                               _ISlower as libc::c_int as libc::c_ushort as
                                   libc::c_int != 0 {
                            (which as libc::c_int) - 'a' as i32
                        } else { -(1 as libc::c_int) };
                    if k < 0 as libc::c_int || k >= floor_num {
                        bell();
                        current_block_208 = 8554725522516090488;
                    } else {
                        /* Convert letter to equipment index */
                        /* Convert letter to floor index */
                        /* Special index */
                        k = 0 as libc::c_int - floor_list[k as usize];
                        current_block_208 = 15455430299222214173;
                    }
                } else { current_block_208 = 15455430299222214173; }
                match current_block_208 {
                    8554725522516090488 => { }
                    _ =>
                    /* Validate the item */
                    {
                        if k >= 0 as libc::c_int && get_item_okay(k) == 0 {
                            bell();
                        } else if ver != 0 &&
                                      verify(b"Try\x00" as *const u8 as
                                                 *const libc::c_char, k) == 0
                         {
                            done = 1 as libc::c_int as bool_
                        } else if get_item_allow(k) == 0 {
                            done = 1 as libc::c_int as bool_
                        } else {
                            /* Verify the item */
                            /* Allow player to "refuse" certain actions */
                            /* Accept that choice */
                            *cp = k;
                            item = 1 as libc::c_int as bool_;
                            done = 1 as libc::c_int as bool_
                        }
                    }
                }
            }
        }
    }
    /* Fix the screen */
    screen_load();
    /* Forget the item_tester_tval restriction */
    item_tester_tval = 0 as libc::c_int as byte_hack;
    /* Forget the item_tester_hook restriction */
    item_tester_hook = None;
    /* Track */
    if item as libc::c_int != 0 && done as libc::c_int != 0 {
        if *cp >= 0 as libc::c_int {
            object_track(&mut *(*p_ptr).inventory.as_mut_ptr().offset(*cp as
                                                                          isize));
        } else {
            object_track(&mut *o_list.offset((0 as libc::c_int - *cp) as
                                                 isize));
        }
    }
    /* Clean up */
    if show_choices != 0 {
        /* Toggle again if needed */
        if toggle != 0 { toggle_inven_equip(); }
        /* Update */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
        /* Window stuff */
        window_stuff();
    }
    /* Clear the prompt line */
    prt(b"\x00" as *const u8 as *const libc::c_char, 0 as libc::c_int,
        0 as libc::c_int);
    /* Warning if needed */
    if oops as libc::c_int != 0 && !str.is_null() { msg_print(str); }
    if item != 0 { repeat_push(*cp); }
    /* Result */
    return item;
}
/*
 * Let the user select an item, save its "index"
 *
 * Return TRUE only if an acceptable item was chosen by the user.
 *
 * The selected item must satisfy the "item_tester_hook()" function,
 * if that hook is set, and the "item_tester_tval", if that value is set.
 *
 * All "item_tester" restrictions are cleared before this function returns.
 *
 * The user is allowed to choose acceptable items from the equipment,
 * inventory, or floor, respectively, if the proper flag was given,
 * and there are any acceptable items in that location.
 *
 * The equipment or inventory are displayed (even if no acceptable
 * items are in that location) if the proper flag was given.
 *
 * If there are no acceptable items available anywhere, and "str" is
 * not NULL, then it will be used as the text of a warning message
 * before the function returns.
 *
 * Note that the user must press "-" to specify the item on the floor,
 * and there is no way to "examine" the item on the floor, while the
 * use of "capital" letters will "examine" an inventory/equipment item,
 * and prompt for its use.
 *
 * If a legal item is selected from the inventory, we save it in "cp"
 * directly (0 to 35), and return TRUE.
 *
 * If a legal item is selected from the floor, we save it in "cp" as
 * a negative (-1 to -511), and return TRUE.
 *
 * If no item is available, we do nothing to "cp", and we display a
 * warning message, using "str" if available, and return FALSE.
 *
 * If no item is selected, we do nothing to "cp", and return FALSE.
 *
 * Global "p_ptr->command_new" is used when viewing the inventory or equipment
 * to allow the user to enter a command while viewing those screens, and
 * also to induce "auto-enter" of stores, and other such stuff.
 *
 * Global "p_ptr->command_wrk" is used to choose between equip/inven listings.
 * If it is TRUE then we are viewing inventory, else equipment.
 *
 * We always erase the prompt when we are done, leaving a blank line,
 * or a warning message, if appropriate, if no items are available.
 */
#[no_mangle]
pub unsafe extern "C" fn get_item(mut cp: *mut libc::c_int, mut pmt: cptr,
                                  mut str: cptr, mut mode: libc::c_int)
 -> bool_ {
    automatizer_create = 0 as libc::c_int as bool_;
    return get_item_floor(cp, pmt, str, mode);
}
/*
 * Hook to determine if an object is getable
 */
unsafe extern "C" fn item_tester_hook_getable(mut o_ptr: *mut object_type)
 -> bool_ {
    if inven_carry_okay(o_ptr) == 0 { return 0 as libc::c_int as bool_ }
    if (*o_ptr).tval as libc::c_int == 99 as libc::c_int &&
           get_skill(8 as libc::c_int) == 0 {
        return 0 as libc::c_int as bool_
    }
    /* Assume yes */
    return 1 as libc::c_int as bool_;
}
/*
 * Wear a single item from o_ptr
 */
#[no_mangle]
pub unsafe extern "C" fn wear_ammo(mut o_ptr: *mut object_type)
 -> libc::c_int {
    let mut slot: libc::c_int = 0;
    let mut num: libc::c_int = 1 as libc::c_int;
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
    /* Check the slot */
    slot = wield_slot(o_ptr) as libc::c_int;
    if slot == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    /* Get local object */
    q_ptr = &mut forge;
    /* Obtain local object */
    object_copy(q_ptr, o_ptr);
    num = (*o_ptr).number as libc::c_int;
    /* Modify quantity */
    (*q_ptr).number = num as byte_hack;
    /* Access the wield slot */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(slot as isize) as
            *mut object_type;
    (*q_ptr).number =
        ((*q_ptr).number as libc::c_int + (*o_ptr).number as libc::c_int) as
            byte_hack;
    /* Wear the new stuff */
    object_copy(o_ptr, q_ptr);
    /* Increment the equip counter by hand */
    equip_cnt += 1;
    /* Cursed! */
    if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
        /* Warn the player */
        msg_print(b"Oops! It feels deathly cold!\x00" as *const u8 as
                      *const libc::c_char);
        /* Note the curse */
        (*o_ptr).ident =
            ((*o_ptr).ident as libc::c_int | 0x1 as libc::c_int) as byte_hack;
        (*o_ptr).sense = 1 as libc::c_int as byte_hack
    }
    /* Recalculate bonuses */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    /* Recalculate torch */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x2 as libc::c_long) as u32b;
    /* Recalculate hitpoint */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x10 as libc::c_long) as u32b;
    /* Recalculate mana */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x20 as libc::c_long) as u32b;
    /* Redraw monster hitpoint */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x10000000 as libc::c_long) as
            u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long |
                  0x8 as libc::c_long)) as u32b;
    return slot;
}
/*
 * Try to pickup arrows
 */
#[no_mangle]
pub unsafe extern "C" fn pickup_ammo() {
    let mut this_o_idx: s16b = 0;
    let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
    let mut slot: s16b = 0;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    /* Scan the pile of objects */
    this_o_idx =
        (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).o_idx;
    while this_o_idx != 0 {
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
        /* Acquire object */
        o_ptr = &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
        if object_similar(o_ptr,
                          &mut *(*p_ptr).inventory.as_mut_ptr().offset(50 as
                                                                           libc::c_int
                                                                           as
                                                                           isize))
               != 0 {
            msg_print(b"You add the ammo to your quiver.\x00" as *const u8 as
                          *const libc::c_char);
            slot = wear_ammo(o_ptr) as s16b;
            if slot as libc::c_int != -(1 as libc::c_int) {
                /* Get the item again */
                o_ptr =
                    &mut *(*p_ptr).inventory.as_mut_ptr().offset(slot as
                                                                     isize) as
                        *mut object_type;
                /* Describe the object */
                object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                            3 as libc::c_int);
                /* Message */
                msg_format(b"You have %s (%c).\x00" as *const u8 as
                               *const libc::c_char, o_name.as_mut_ptr(),
                           index_to_label(slot as libc::c_int) as
                               libc::c_int);
                /* Delete the object */
                delete_object_idx(this_o_idx as libc::c_int);
            }
        }
        /* Acquire next object */
        next_o_idx = (*o_ptr).next_o_idx;
        this_o_idx = next_o_idx
    };
}
/*
 * Make the player carry everything in a grid
 *
 * If "pickup" is FALSE then only gold will be picked up
 *
 * This is called by py_pickup() when easy_floor is TRUE.
 */
#[no_mangle]
pub unsafe extern "C" fn can_carry_heavy(mut o_ptr: *mut object_type)
 -> bool_ {
    /* Query if object is heavy */
    if prompt_pickup_heavy != 0 {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut old_enc: libc::c_int = 0 as libc::c_int;
        let mut new_enc: libc::c_int = 0 as libc::c_int;
        /* Extract the "weight limit" (in tenth pounds) */
        i = weight_limit();
        /* Calculate current encumbarance */
        j = calc_total_weight();
        /* Apply encumbarance from weight */
        if j > i / 2 as libc::c_int {
            old_enc = (j - i / 2 as libc::c_int) / (i / 10 as libc::c_int)
        }
        /* Increase the weight, recalculate encumbarance */
        j += (*o_ptr).number as libc::c_int * (*o_ptr).weight;
        /* Apply encumbarance from weight */
        if j > i / 2 as libc::c_int {
            new_enc = (j - i / 2 as libc::c_int) / (i / 10 as libc::c_int)
        }
        /* Should we query? */
        if new_enc > old_enc { return 0 as libc::c_int as bool_ }
    }
    return 1 as libc::c_int as bool_;
}
/* Do the actuall picking up */
#[no_mangle]
pub unsafe extern "C" fn object_pickup(mut this_o_idx: libc::c_int) {
    let mut slot: libc::c_int = 0 as libc::c_int;
    let mut o_name: [libc::c_char; 80] =
        *::std::mem::transmute::<&[u8; 80],
                                 &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let mut q_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    /* Access the item */
    o_ptr = &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
    if (*p_ptr).auto_id != 0 { object_aware(o_ptr); object_known(o_ptr); }
    /* Describe the object */
    object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                3 as libc::c_int);
    /* Note that the pack is too full */
    if inven_carry_okay(o_ptr) == 0 &&
           object_similar(o_ptr,
                          &mut *(*p_ptr).inventory.as_mut_ptr().offset(50 as
                                                                           libc::c_int
                                                                           as
                                                                           isize))
               == 0 {
        msg_format(b"You have no room for %s.\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr());
    } else {
        /* Pick up object */
        /* Tell the scripts */
        if process_hooks(50 as libc::c_int,
                         b"(O,d)\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char, o_ptr, this_o_idx) != 0 {
            return
        }
        q_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(50 as libc::c_int as
                                                             isize) as
                *mut object_type;
        if object_similar(o_ptr, q_ptr) != 0 {
            msg_print(b"You add the ammo to your quiver.\x00" as *const u8 as
                          *const libc::c_char);
            slot = wear_ammo(o_ptr)
        } else {
            slot =
                inven_carry(o_ptr, 0 as libc::c_int as bool_) as libc::c_int
        }
        if slot != -(1 as libc::c_int) {
            /* Carry the item */
            /* Get the item again */
            o_ptr =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(slot as isize) as
                    *mut object_type;
            object_track(o_ptr);
            /* Describe the object */
            object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                        3 as libc::c_int);
            /* Message */
            msg_format(b"You have %s (%c).\x00" as *const u8 as
                           *const libc::c_char, o_name.as_mut_ptr(),
                       index_to_label(slot) as libc::c_int);
            /* Delete the object */
            delete_object_idx(this_o_idx);
            /* Sense object. */
            sense_inventory();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn py_pickup_floor(mut pickup: libc::c_int) {
    let mut this_o_idx: s16b = 0;
    let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
    let mut o_name: [libc::c_char; 80] =
        *::std::mem::transmute::<&[u8; 80],
                                 &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut floor_num: libc::c_int = 0 as libc::c_int;
    let mut floor_o_idx: libc::c_int = 0 as libc::c_int;
    let mut do_pickup: bool_ = 1 as libc::c_int as bool_;
    let mut do_ask: bool_ = 1 as libc::c_int as bool_;
    /* Sanity check */
    /* Hack -- ignore monster traps */
    if (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat as
           libc::c_int == 0xaf as libc::c_int {
        return
    }
    /* Try to grab ammo */
    pickup_ammo();
    /* Mega Hack -- If we have auto-Id, do an ID sweep *before* squleching,
	 * so that we don't have to walk over things twice to get them
	 * squelched.  --dsb */
    if (*p_ptr).auto_id != 0 {
        this_o_idx =
            (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).o_idx;
        while this_o_idx != 0 {
            /* Aquire the object */
            o_ptr =
                &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
            /* Acquire the next object index */
            next_o_idx = (*o_ptr).next_o_idx;
            /* Identify Object */
            object_aware(o_ptr);
            object_known(o_ptr);
            this_o_idx = next_o_idx
        }
    }
    /* Squeltch the floor */
    squeltch_grid();
    /* Scan the pile of objects */
    this_o_idx =
        (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).o_idx;
    while this_o_idx != 0 {
        /* Acquire object */
        o_ptr = &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
        /* Acquire next object */
        next_o_idx = (*o_ptr).next_o_idx;
        /* Hack -- disturb */
        disturb(0 as libc::c_int, 0 as libc::c_int);
        /* Pick up gold */
        if (*o_ptr).tval as libc::c_int == 100 as libc::c_int {
            let mut goldname: [libc::c_char; 80] = [0; 80];
            object_desc(goldname.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                        3 as libc::c_int);
            /* Message */
            msg_format(b"You have found %ld gold pieces worth of %s.\x00" as
                           *const u8 as *const libc::c_char,
                       (*o_ptr).pval as libc::c_long, goldname.as_mut_ptr());
            /* Collect the gold */
            (*p_ptr).au += (*o_ptr).pval;
            /* Redraw gold */
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x100 as libc::c_long) as
                    u32b;
            /* Window stuff */
            (*p_ptr).window =
                ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as
                    u32b;
            /* Delete the gold */
            delete_object_idx(this_o_idx as libc::c_int);
        } else {
            let mut testdesc: [libc::c_char; 80] = [0; 80];
            object_desc(testdesc.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                        3 as libc::c_int);
            if 0 as libc::c_int !=
                   strncmp(testdesc.as_mut_ptr(),
                           b"(nothing)\x00" as *const u8 as
                               *const libc::c_char,
                           80 as libc::c_int as libc::c_ulong) {
                strncpy(o_name.as_mut_ptr(), testdesc.as_mut_ptr(),
                        80 as libc::c_int as libc::c_ulong);
            }
            /* Count non-gold */
            floor_num += 1;
            /* Remember this index */
            floor_o_idx = this_o_idx as libc::c_int
        }
        this_o_idx = next_o_idx
    }
    /* There were no non-gold items */
    if floor_num == 0 { return }
    /* Mention number of items */
    if pickup == 0 {
        /* One item */
        if floor_num == 1 as libc::c_int {
            /* Acquire object */
            o_ptr =
                &mut *o_list.offset(floor_o_idx as isize) as *mut object_type;
            /* Message */
            msg_format(b"You see %s.\x00" as *const u8 as *const libc::c_char,
                       o_name.as_mut_ptr());
        } else {
            /* Multiple items */
            /* Message */
            msg_format(b"You see a pile of %d items.\x00" as *const u8 as
                           *const libc::c_char, floor_num);
        }
        /* Done */
        return
    }
    /* One item */
    if floor_num == 1 as libc::c_int {
        /* Hack -- query every item */
        if carry_query_flag as libc::c_int != 0 ||
               can_carry_heavy(&mut *o_list.offset(floor_o_idx as isize)) == 0
           {
            if inven_carry_okay(o_ptr) == 0 &&
                   object_similar(o_ptr,
                                  &mut *(*p_ptr).inventory.as_mut_ptr().offset(50
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize))
                       == 0 {
                object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                            3 as libc::c_int);
                msg_format(b"You have no room for %s.\x00" as *const u8 as
                               *const libc::c_char, o_name.as_mut_ptr());
                do_pickup = 0 as libc::c_int as bool_
            } else {
                let mut out_val: [libc::c_char; 160] = [0; 160];
                sprintf(out_val.as_mut_ptr(),
                        b"Pick up %s? \x00" as *const u8 as
                            *const libc::c_char, o_name.as_mut_ptr());
                do_pickup = get_check(out_val.as_mut_ptr() as cptr)
            }
        }
        /* Don't ask */
        do_ask = 0 as libc::c_int as bool_;
        if (*o_list.offset(floor_o_idx as isize)).tval as libc::c_int ==
               99 as libc::c_int && get_skill(8 as libc::c_int) == 0 {
            do_pickup = 0 as libc::c_int as bool_
        } else { this_o_idx = floor_o_idx as s16b }
    }
    /* Ask */
    if do_ask != 0 {
        let mut q: cptr = 0 as *const libc::c_char;
        let mut s: cptr = 0 as *const libc::c_char;
        let mut item: libc::c_int = 0;
        /* Get an item */
        item_tester_hook =
            Some(item_tester_hook_getable as
                     unsafe extern "C" fn(_: *mut object_type) -> bool_);
        q = b"Get which item? \x00" as *const u8 as *const libc::c_char;
        s =
            b"You have no room in your pack for any of the items here.\x00" as
                *const u8 as *const libc::c_char;
        if get_item(&mut item, q, s, 0x4 as libc::c_int) != 0 {
            this_o_idx = (0 as libc::c_int - item) as s16b;
            if can_carry_heavy(&mut *o_list.offset(this_o_idx as isize)) == 0
               {
                let mut out_val_0: [libc::c_char; 160] = [0; 160];
                /* Describe the object */
                object_desc(o_name.as_mut_ptr(),
                            &mut *o_list.offset(this_o_idx as isize),
                            1 as libc::c_int, 3 as libc::c_int);
                sprintf(out_val_0.as_mut_ptr(),
                        b"Pick up %s? \x00" as *const u8 as
                            *const libc::c_char, o_name.as_mut_ptr());
                do_pickup = get_check(out_val_0.as_mut_ptr() as cptr)
            }
        } else { do_pickup = 0 as libc::c_int as bool_ }
    }
    /* Pick up the item */
    if do_pickup != 0 { object_pickup(this_o_idx as libc::c_int); };
}
/* Add a flags group */
#[no_mangle]
pub unsafe extern "C" fn gain_flag_group(mut o_ptr: *mut object_type,
                                         mut silent: bool_) {
    let mut grp: libc::c_int = 0 as libc::c_int;
    let mut tries: libc::c_int = 1000 as libc::c_int;
    loop  {
        let fresh90 = tries;
        tries = tries - 1;
        if !(fresh90 != 0) { break ; }
        grp = Rand_div(12 as libc::c_int);
        /* If we already got this group continue */
        if (*o_ptr).pval3 as libc::c_long & (1 as libc::c_long) << grp != 0 {
            continue ;
        }
        /* Not enough points ? */
        if !(flags_groups[grp as usize].price as libc::c_int >
                 (*o_ptr).pval2 as libc::c_int) {
            break ;
        }
    }
    /* Ack, nothing found */
    if tries <= 1 as libc::c_int { return }
    (*o_ptr).pval2 =
        ((*o_ptr).pval2 as libc::c_int -
             flags_groups[grp as usize].price as libc::c_int) as s16b;
    (*o_ptr).pval3 =
        ((*o_ptr).pval3 as libc::c_long | (1 as libc::c_long) << grp) as s32b;
    if silent == 0 {
        let mut o_name: [libc::c_char; 80] = [0; 80];
        object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                    0 as libc::c_int);
        msg_format(b"%s gains access to the %s realm.\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr(),
                   flags_groups[grp as usize].name.as_mut_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_flag(mut o_ptr: *mut object_type,
                                  mut grp: libc::c_int, mut k: libc::c_int)
 -> u32b {
    let mut f: u32b = 0 as libc::c_int as u32b;
    let mut flag_set: u32b = 0 as libc::c_int as u32b;
    let mut tries: libc::c_int = 1000 as libc::c_int;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut flag_test: u32b = 0;
    /* Extract some flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* get the corresponding flag set of the group */
    match k {
        0 => { flag_set = flags_groups[grp as usize].flags1; flag_test = f1 }
        1 => { flag_set = flags_groups[grp as usize].flags2; flag_test = f2 }
        2 => { flag_set = flags_groups[grp as usize].flags3; flag_test = f3 }
        3 => { flag_set = flags_groups[grp as usize].flags4; flag_test = f4 }
        4 => { flag_set = flags_groups[grp as usize].esp; flag_test = esp }
        _ => { flag_set = flags_groups[grp as usize].flags1; flag_test = f1 }
    }
    /* If no flags, no need to look */
    if count_bits(flag_set) == 0 { return 0 as libc::c_int as u32b }
    loop  {
        let fresh91 = tries;
        tries = tries - 1;
        if !(fresh91 != 0) { break ; }
        /* get a random flag */
        f = ((1 as libc::c_long) << Rand_div(32 as libc::c_int)) as u32b;
        /* is it part of the group */
        if f & flag_set == 0 { continue ; }
        /* Already got it */
        if !(f & flag_test != 0) { break ; }
    }
    if tries <= 1 as libc::c_int {
        return 0 as libc::c_int as u32b
    } else { return f };
}
/* Add a flags from a flag group */
#[no_mangle]
pub unsafe extern "C" fn gain_flag_group_flag(mut o_ptr: *mut object_type,
                                              mut silent: bool_) {
    let mut grp: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut f: u32b = 0 as libc::c_int as u32b;
    let mut tries: libc::c_int = 20000 as libc::c_int;
    if count_bits((*o_ptr).pval3 as u32b) == 0 { return }
    loop  {
        let fresh92 = tries;
        tries = tries - 1;
        if !(fresh92 != 0) { break ; }
        /* Get a flag set */
        k = Rand_div(5 as libc::c_int);
        /* get a flag group */
        grp = Rand_div(12 as libc::c_int);
        if (1 as libc::c_long) << grp & (*o_ptr).pval3 as libc::c_long == 0 {
            continue ;
        }
        /* Return a flag from the group/set */
        f = get_flag(o_ptr, grp, k);
        if !(f == 0) { break ; }
    }
    if tries <= 1 as libc::c_int { return }
    match k {
        0 => { (*o_ptr).art_flags1 |= f }
        1 => { (*o_ptr).art_flags2 |= f }
        2 => { (*o_ptr).art_flags3 |= f }
        3 => { (*o_ptr).art_flags4 |= f }
        4 => { (*o_ptr).art_esp |= f }
        _ => { }
    }
    if silent == 0 {
        let mut o_name: [libc::c_char; 80] = [0; 80];
        object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                    0 as libc::c_int);
        msg_format(b"%s gains a new power from the %s realm.\x00" as *const u8
                       as *const libc::c_char, o_name.as_mut_ptr(),
                   flags_groups[grp as usize].name.as_mut_ptr());
    };
}
/*
 * When an object gain a level, he can gain some attributes
 */
#[no_mangle]
pub unsafe extern "C" fn object_gain_level(mut o_ptr: *mut object_type) {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    /* Extract some flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* First it can gain some tohit and todam */
    if (*o_ptr).tval as libc::c_int == 24 as libc::c_int ||
           (*o_ptr).tval as libc::c_int == 23 as libc::c_int ||
           (*o_ptr).tval as libc::c_int == 22 as libc::c_int ||
           (*o_ptr).tval as libc::c_int == 21 as libc::c_int ||
           (*o_ptr).tval as libc::c_int == 6 as libc::c_int {
        let mut k: libc::c_int = Rand_div(100 as libc::c_int);
        /* gain +2,+1 */
        if k < 33 as libc::c_int {
            (*o_ptr).to_h =
                ((*o_ptr).to_h as libc::c_int +
                     (Rand_div(2 as libc::c_int) + 1 as libc::c_int)) as s16b;
            (*o_ptr).to_d =
                ((*o_ptr).to_d as libc::c_int + 1 as libc::c_int) as s16b
        } else if k < 66 as libc::c_int {
            (*o_ptr).to_h =
                ((*o_ptr).to_h as libc::c_int + 1 as libc::c_int) as s16b;
            (*o_ptr).pval2 += 1;
            if Rand_div(100 as libc::c_int) < 40 as libc::c_int {
                gain_flag_group(o_ptr, 0 as libc::c_int as bool_);
            }
        } else {
            if (*o_ptr).pval3 == 0 {
                gain_flag_group(o_ptr, 0 as libc::c_int as bool_);
            }
            gain_flag_group_flag(o_ptr, 0 as libc::c_int as bool_);
            if (*o_ptr).pval == 0 {
                (*o_ptr).pval = 1 as libc::c_int
            } else {
                while Rand_div(100 as libc::c_int) <
                          20 as libc::c_int - (*o_ptr).pval * 2 as libc::c_int
                      {
                    (*o_ptr).pval += 1
                }
                if (*o_ptr).pval > 5 as libc::c_int {
                    (*o_ptr).pval = 5 as libc::c_int
                }
            }
        }
    };
}
/* +1 and 1 point */
/*
 * Item sets fcts
 */
#[no_mangle]
pub unsafe extern "C" fn wield_set(mut a_idx: s16b, mut set_idx: s16b,
                                   mut silent: bool_) -> bool_ {
    let mut s_ptr: *mut set_type =
        &mut *set_info.offset(set_idx as isize) as *mut set_type;
    let mut i: libc::c_int = 0;
    if -(1 as libc::c_int) ==
           (*a_info.offset(a_idx as isize)).set as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    i = 0 as libc::c_int;
    while i < (*s_ptr).num as libc::c_int {
        if a_idx as libc::c_int ==
               (*s_ptr).arts[i as usize].a_idx as libc::c_int {
            break ;
        }
        i += 1
    }
    if (*s_ptr).arts[i as usize].present == 0 {
        (*s_ptr).num_use = (*s_ptr).num_use.wrapping_add(1);
        (*s_ptr).arts[i as usize].present = 1 as libc::c_int as bool_;
        if (*s_ptr).num_use as libc::c_int > (*s_ptr).num as libc::c_int {
            msg_print(b"ERROR!! s_ptr->num_use > s_ptr->use\x00" as *const u8
                          as *const libc::c_char);
        } else if (*s_ptr).num_use as libc::c_int ==
                      (*s_ptr).num as libc::c_int && silent == 0 {
            cmsg_format(5 as libc::c_int as byte_hack,
                        b"%s item set completed.\x00" as *const u8 as
                            *const libc::c_char,
                        set_name.offset((*s_ptr).name as isize));
        }
        return 1 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn takeoff_set(mut a_idx: s16b, mut set_idx: s16b)
 -> bool_ {
    let mut s_ptr: *mut set_type =
        &mut *set_info.offset(set_idx as isize) as *mut set_type;
    let mut i: libc::c_int = 0;
    if -(1 as libc::c_int) ==
           (*a_info.offset(a_idx as isize)).set as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    i = 0 as libc::c_int;
    while i < (*s_ptr).num as libc::c_int {
        if a_idx as libc::c_int ==
               (*s_ptr).arts[i as usize].a_idx as libc::c_int {
            break ;
        }
        i += 1
    }
    if (*s_ptr).arts[i as usize].present != 0 {
        (*s_ptr).arts[i as usize].present = 0 as libc::c_int as bool_;
        (*s_ptr).num_use = (*s_ptr).num_use.wrapping_sub(1);
        if (*s_ptr).num_use as libc::c_int == 255 as libc::c_int {
            msg_print(b"ERROR!! s_ptr->num_use < 0\x00" as *const u8 as
                          *const libc::c_char);
        }
        if (*s_ptr).num_use as libc::c_int ==
               (*s_ptr).num as libc::c_int - 1 as libc::c_int {
            cmsg_format(5 as libc::c_int as byte_hack,
                        b"%s item set not complete anymore.\x00" as *const u8
                            as *const libc::c_char,
                        set_name.offset((*s_ptr).name as isize));
        }
        return 1 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn apply_set(mut a_idx: s16b, mut set_idx: s16b)
 -> bool_ {
    let mut s_ptr: *mut set_type =
        &mut *set_info.offset(set_idx as isize) as *mut set_type;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if -(1 as libc::c_int) ==
           (*a_info.offset(a_idx as isize)).set as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    i = 0 as libc::c_int;
    while i < (*s_ptr).num as libc::c_int {
        if a_idx as libc::c_int ==
               (*s_ptr).arts[i as usize].a_idx as libc::c_int {
            break ;
        }
        i += 1
    }
    if (*s_ptr).arts[i as usize].present != 0 {
        j = 0 as libc::c_int;
        while j < (*s_ptr).num_use as libc::c_int {
            apply_flags((*s_ptr).arts[i as usize].flags1[j as usize],
                        (*s_ptr).arts[i as usize].flags2[j as usize],
                        (*s_ptr).arts[i as usize].flags3[j as usize],
                        (*s_ptr).arts[i as usize].flags4[j as usize],
                        (*s_ptr).arts[i as usize].flags5[j as usize],
                        (*s_ptr).arts[i as usize].esp[j as usize],
                        (*s_ptr).arts[i as usize].pval[j as usize],
                        0 as libc::c_int as s16b, 0 as libc::c_int as s16b,
                        0 as libc::c_int as s16b, 0 as libc::c_int as s16b);
            j += 1
        }
        return 1 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn apply_flags_set(mut a_idx: s16b, mut set_idx: s16b,
                                         mut f1: *mut u32b, mut f2: *mut u32b,
                                         mut f3: *mut u32b, mut f4: *mut u32b,
                                         mut f5: *mut u32b,
                                         mut esp: *mut u32b) -> bool_ {
    let mut s_ptr: *mut set_type =
        &mut *set_info.offset(set_idx as isize) as *mut set_type;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if -(1 as libc::c_int) ==
           (*a_info.offset(a_idx as isize)).set as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    i = 0 as libc::c_int;
    while i < (*s_ptr).num as libc::c_int {
        if a_idx as libc::c_int ==
               (*s_ptr).arts[i as usize].a_idx as libc::c_int {
            break ;
        }
        i += 1
    }
    if (*s_ptr).arts[i as usize].present != 0 {
        j = 0 as libc::c_int;
        while j < (*s_ptr).num_use as libc::c_int {
            *f1 |= (*s_ptr).arts[i as usize].flags1[j as usize];
            *f2 |= (*s_ptr).arts[i as usize].flags2[j as usize];
            *f3 |= (*s_ptr).arts[i as usize].flags3[j as usize];
            *f4 |= (*s_ptr).arts[i as usize].flags4[j as usize];
            *f5 |= (*s_ptr).arts[i as usize].flags5[j as usize];
            *esp |= (*s_ptr).arts[i as usize].esp[j as usize];
            j += 1
        }
        return 1 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
