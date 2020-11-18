use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut ddx: [s16b; 10];
    #[no_mangle]
    static mut ddy: [s16b; 10];
    #[no_mangle]
    static mut cli_info: *mut cli_comm;
    #[no_mangle]
    static mut cli_total: libc::c_int;
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut command_arg: s16b;
    #[no_mangle]
    static mut command_wrk: s16b;
    #[no_mangle]
    static mut command_new: s16b;
    #[no_mangle]
    static mut energy_use: s32b;
    #[no_mangle]
    static mut wizard: bool_;
    #[no_mangle]
    static mut use_bigtile: bool_;
    #[no_mangle]
    static mut equip_cnt: s16b;
    #[no_mangle]
    static mut o_max: s16b;
    #[no_mangle]
    static mut flush_failure: bool_;
    #[no_mangle]
    static mut auto_destroy: bool_;
    #[no_mangle]
    static mut wear_confirm: bool_;
    #[no_mangle]
    static mut expand_list: bool_;
    #[no_mangle]
    static mut cheat_know: bool_;
    #[no_mangle]
    static mut panel_row_min: s16b;
    #[no_mangle]
    static mut panel_col_min: s16b;
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut o_list: *mut object_type;
    #[no_mangle]
    static mut max_towns: u16b;
    #[no_mangle]
    static mut town_info: *mut town_type;
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut a_info: *mut artifact_type;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut r_name: *mut libc::c_char;
    #[no_mangle]
    static mut item_tester_full: bool_;
    #[no_mangle]
    static mut item_tester_hook:
           Option<unsafe extern "C" fn(_: *mut object_type) -> bool_>;
    #[no_mangle]
    static mut ang_sort_comp:
           Option<unsafe extern "C" fn(_: vptr, _: vptr, _: libc::c_int,
                                       _: libc::c_int) -> bool_>;
    #[no_mangle]
    static mut ang_sort_swap:
           Option<unsafe extern "C" fn(_: vptr, _: vptr, _: libc::c_int,
                                       _: libc::c_int) -> ()>;
    #[no_mangle]
    static mut max_r_idx: u16b;
    #[no_mangle]
    static mut max_st_idx: u16b;
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn monster_race_track(r_idx: libc::c_int, ego: libc::c_int);
    #[no_mangle]
    fn html_screenshot(name: cptr);
    #[no_mangle]
    fn help_file_screenshot(name: cptr);
    #[no_mangle]
    fn show_file(name: cptr, what: cptr, line: libc::c_int, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn screen_roff(r_idx: libc::c_int, ego: libc::c_int,
                   remember: libc::c_int);
    #[no_mangle]
    fn inc_stack_size(item: libc::c_int, delta: libc::c_int);
    #[no_mangle]
    fn inc_stack_size_ex(item: libc::c_int, delta: libc::c_int,
                         opt: optimize_flag, desc: describe_flag);
    #[no_mangle]
    fn wield_set(a_idx: s16b, set_idx: s16b, silent: bool_) -> bool_;
    #[no_mangle]
    fn get_object(item: libc::c_int) -> *mut object_type;
    #[no_mangle]
    fn calc_total_weight() -> s32b;
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn object_out_desc(o_ptr: *mut object_type, fff: *mut FILE,
                       trim_down: bool_, wait_for_it: bool_) -> bool_;
    #[no_mangle]
    fn index_to_label(i: libc::c_int) -> libc::c_char;
    #[no_mangle]
    fn wield_slot(o_ptr: *mut object_type) -> s16b;
    #[no_mangle]
    fn describe_use(i: libc::c_int) -> cptr;
    #[no_mangle]
    fn inven_takeoff(item: libc::c_int, amt: libc::c_int, force_drop: bool_)
     -> s16b;
    #[no_mangle]
    fn inven_drop(item: libc::c_int, amt: libc::c_int, dy: libc::c_int,
                  dx: libc::c_int, silent: bool_);
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
              _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn Term_addstr(n: libc::c_int, a: byte_hack, s: cptr) -> errr;
    #[no_mangle]
    fn Term_addch(a: byte_hack, c: libc::c_char) -> errr;
    #[no_mangle]
    fn Term_gotoxy(x: libc::c_int, y: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_fresh() -> errr;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn strnfmt(buf: *mut libc::c_char, max: uint_hack, fmt: cptr, _: ...)
     -> uint_hack;
    #[no_mangle]
    fn string_make(str: cptr) -> cptr;
    #[no_mangle]
    fn ralloc(len: huge_hack) -> vptr;
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn fix_message();
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn get_string(prompt: cptr, buf: *mut libc::c_char, len: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn get_check(prompt: cptr) -> bool_;
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    static mut askfor_aux_complete: bool_;
    #[no_mangle]
    fn askfor_aux(buf: *mut libc::c_char, len: libc::c_int) -> bool_;
    #[no_mangle]
    fn fd_kill(file: cptr) -> errr;
    #[no_mangle]
    fn my_fclose(fff: *mut FILE) -> errr;
    #[no_mangle]
    fn my_fopen(file: cptr, mode: cptr) -> *mut FILE;
    #[no_mangle]
    fn path_temp(buf: *mut libc::c_char, max: libc::c_int) -> errr;
    #[no_mangle]
    static mut request_command_inven_mode: bool_;
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn weight_limit() -> libc::c_int;
    #[no_mangle]
    fn show_inven();
    #[no_mangle]
    fn show_equip();
    #[no_mangle]
    fn object_copy(o_ptr: *mut object_type, j_ptr: *mut object_type);
    #[no_mangle]
    fn object_similar(o_ptr: *mut object_type, j_ptr: *mut object_type)
     -> bool_;
    #[no_mangle]
    fn get_item(cp: *mut libc::c_int, pmt: cptr, str: cptr, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn get_quantity(prompt: cptr, max: s32b) -> s32b;
    #[no_mangle]
    fn inc_piety(god: libc::c_int, amt: s32b);
    #[no_mangle]
    fn automatizer_add_rule(o_ptr: *mut object_type, destroy: bool_);
    #[no_mangle]
    static mut automatizer_create: bool_;
    #[no_mangle]
    fn sound(num: libc::c_int);
    #[no_mangle]
    fn object_value(o_ptr: *mut object_type) -> s32b;
    #[no_mangle]
    fn cmsg_format(color: byte_hack, fmt: cptr, _: ...);
    #[no_mangle]
    fn quark_add(str: cptr) -> s16b;
    #[no_mangle]
    fn quark_str(num: s16b) -> cptr;
    #[no_mangle]
    fn target_set(mode: libc::c_int) -> bool_;
    #[no_mangle]
    fn handle_stuff();
    #[no_mangle]
    fn verify_panel();
    #[no_mangle]
    fn change_panel(dy: libc::c_int, dx: libc::c_int) -> bool_;
    #[no_mangle]
    fn bell();
    #[no_mangle]
    fn get_keymap_dir(ch: libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn get_com(prompt: cptr, command: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn ang_sort(u: vptr, v: vptr, n: libc::c_int);
    #[no_mangle]
    fn put_str(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn strlower(buf: *mut libc::c_char);
    #[no_mangle]
    fn flush();
    #[no_mangle]
    fn do_cmd_store();
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Term_erase(x: libc::c_int, y: libc::c_int, n: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_load_from(save: *mut term_win, final_0: bool_) -> errr;
    #[no_mangle]
    fn Term_load() -> errr;
    #[no_mangle]
    fn Term_save_to() -> *mut term_win;
    #[no_mangle]
    fn Term_save() -> errr;
    #[no_mangle]
    fn Term_get_size(w: *mut libc::c_int, h: *mut libc::c_int) -> errr;
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
pub type uint_hack = libc::c_uint;
pub type huge_hack = libc::c_ulong;
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
pub struct cli_comm {
    pub comm: cptr,
    pub descrip: cptr,
    pub key: s16b,
}
pub type optimize_flag = libc::c_uint;
pub const NO_OPTIMIZE: optimize_flag = 1;
pub const OPTIMIZE: optimize_flag = 0;
pub type describe_flag = libc::c_uint;
pub const NO_DESCRIBE: describe_flag = 1;
pub const DESCRIBE: describe_flag = 0;
/* File: cmd3.c */
/* Purpose: Inventory commands */
/*
 * Copyright (c) 1989 James E. Wilson, Robert A. Koeneke
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Display p_ptr->inventory
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_inven() {
    let mut out_val: [libc::c_char; 160] = [0; 160];
    /* Note that we are in "p_ptr->inventory" mode */
    command_wrk = 0 as libc::c_int as s16b;
    /* Save the screen */
    character_icky = 1 as libc::c_int as bool_;
    Term_save();
    /* Hack -- show empty slots */
    item_tester_full = 1 as libc::c_int as bool_;
    /* Display the p_ptr->inventory */
    show_inven();
    /* Hack -- hide empty slots */
    item_tester_full = 0 as libc::c_int as bool_;
    let mut total_weight: s32b = calc_total_weight();
    strnfmt(out_val.as_mut_ptr(), 160 as libc::c_int as uint_hack,
            b"Inventory: carrying %ld.%ld pounds (%ld%% of capacity). Command: \x00"
                as *const u8 as *const libc::c_char,
            total_weight / 10 as libc::c_int,
            total_weight % 10 as libc::c_int,
            total_weight * 100 as libc::c_int /
                (weight_limit() / 2 as libc::c_int));
    /* Get a command */
    prt(out_val.as_mut_ptr() as cptr, 0 as libc::c_int, 0 as libc::c_int);
    /* Get a new command */
    command_new = inkey() as s16b;
    /* Restore the screen */
    Term_load();
    character_icky = 0 as libc::c_int as bool_;
    /* Process "Escape" */
    if command_new as libc::c_int == '\u{1b}' as i32 {
        /* Reset stuff */
        command_new = 0 as libc::c_int as s16b
    } else {
        /* Process normal keys */
        /* Mega-Hack -- Don't disable keymaps for this key */
        request_command_inven_mode = 1 as libc::c_int as bool_
    };
}
/*
 * Display equipment
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_equip() {
    let mut out_val: [libc::c_char; 160] = [0; 160];
    /* Note that we are in "equipment" mode */
    command_wrk = 1 as libc::c_int as s16b;
    /* Save the screen */
    character_icky = 1 as libc::c_int as bool_;
    Term_save();
    /* Hack -- show empty slots */
    item_tester_full = 1 as libc::c_int as bool_;
    /* Display the equipment */
    show_equip();
    /* Hack -- undo the hack above */
    item_tester_full = 0 as libc::c_int as bool_;
    /* Build a prompt */
    let mut total_weight: s32b = calc_total_weight();
    /* Build a prompt */
    strnfmt(out_val.as_mut_ptr(), 160 as libc::c_int as uint_hack,
            b"Equipment: carrying %ld.%ld pounds (%ld%% of capacity). Command: \x00"
                as *const u8 as *const libc::c_char,
            total_weight / 10 as libc::c_int,
            total_weight % 10 as libc::c_int,
            total_weight * 100 as libc::c_int /
                (weight_limit() / 2 as libc::c_int));
    /* Get a command */
    prt(out_val.as_mut_ptr() as cptr, 0 as libc::c_int, 0 as libc::c_int);
    /* Get a new command */
    command_new = inkey() as s16b;
    /* Restore the screen */
    Term_load();
    character_icky = 0 as libc::c_int as bool_;
    /* Process "Escape" */
    if command_new as libc::c_int == '\u{1b}' as i32 {
        /* Reset stuff */
        command_new = 0 as libc::c_int as s16b
    } else {
        /* Process normal keys */
        /* Mega-Hack -- Don't disable keymaps for this key */
        request_command_inven_mode = 1 as libc::c_int as bool_
    };
}
/*
 * The "wearable" tester
 */
unsafe extern "C" fn item_tester_hook_wear(mut o_ptr: *mut object_type)
 -> bool_ {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut slot: libc::c_int = wield_slot(o_ptr) as libc::c_int;
    /* Extract the flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Only one ultimate at a time */
    if f4 as libc::c_long & 0x1000000 as libc::c_long != 0 {
        let mut i: libc::c_int = 0;
        i = 24 as libc::c_int;
        while i < 52 as libc::c_int {
            let mut q_ptr: *mut object_type =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                    *mut object_type;
            /* Extract the flags */
            object_flags(q_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                         &mut esp);
            if !((*q_ptr).k_idx == 0) {
                if f4 as libc::c_long & 0x1000000 as libc::c_long != 0 {
                    return 0 as libc::c_int as bool_
                }
            }
            i += 1
        }
    }
    if slot < 24 as libc::c_int ||
           (*p_ptr).body_parts[(slot - 24 as libc::c_int) as usize] as
               libc::c_int == 24 as libc::c_int &&
               (*p_ptr).melee_style as libc::c_int != 17 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Check for a usable slot */
    if slot >= 24 as libc::c_int { return 1 as libc::c_int as bool_ }
    /* Assume not wearable */
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn is_slot_ok(mut slot: libc::c_int) -> bool_ {
    if slot >= 24 as libc::c_int && slot < 52 as libc::c_int {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
/*
 * Wield or wear a single item from the pack or floor
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_wield() {
    let mut item: libc::c_int = 0;
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
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut i_ptr: *mut object_type = 0 as *mut object_type;
    let mut act: cptr = 0 as *const libc::c_char;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    /* Restrict the choices */
    item_tester_hook =
        Some(item_tester_hook_wear as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Get an item */
    q = b"Wear/Wield which item? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have nothing you can wear or wield.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s, 0x2 as libc::c_int | 0x4 as libc::c_int) == 0
       {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* Check the slot */
    slot = wield_slot(o_ptr) as libc::c_int;
    /* Prevent wielding into a cursed slot */
    if (*p_ptr).inventory[slot as usize].ident as libc::c_int &
           0x40 as libc::c_int != 0 {
        /* Describe it */
        object_desc(o_name.as_mut_ptr(),
                    &mut *(*p_ptr).inventory.as_mut_ptr().offset(slot as
                                                                     isize),
                    0 as libc::c_int, 0 as libc::c_int);
        /* Message */
        msg_format(b"The %s you are %s appears to be cursed.\x00" as *const u8
                       as *const libc::c_char, o_name.as_mut_ptr(),
                   describe_use(slot));
        /* Cancel the command */
        return
    }
    if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 &&
           wear_confirm as libc::c_int != 0 &&
           ((*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0 ||
                (*k_info.offset((*o_ptr).k_idx as isize)).easy_know as
                    libc::c_int != 0 &&
                    (*k_info.offset((*o_ptr).k_idx as isize)).aware as
                        libc::c_int != 0 ||
                (*o_ptr).ident as libc::c_int & 0x1 as libc::c_int != 0) {
        let mut dummy: [libc::c_char; 512] = [0; 512];
        /* Describe it */
        object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                    0 as libc::c_int);
        strnfmt(dummy.as_mut_ptr(), 512 as libc::c_int as uint_hack,
                b"Really use the %s {cursed}? \x00" as *const u8 as
                    *const libc::c_char, o_name.as_mut_ptr());
        if get_check(dummy.as_mut_ptr() as cptr) == 0 { return }
    }
    /* Can we wield */
    if process_hooks(21 as libc::c_int,
                     b"(d)\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char, item) != 0 {
        return
    }
    /* Extract the flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Two handed weapons can't be wielded with a shield */
    if is_slot_ok(slot - 24 as libc::c_int + 39 as libc::c_int) as libc::c_int
           != 0 && f4 as libc::c_long & 0x80 as libc::c_long != 0 &&
           (*p_ptr).inventory[(slot - 24 as libc::c_int + 39 as libc::c_int)
                                  as usize].k_idx as libc::c_int !=
               0 as libc::c_int {
        object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                    0 as libc::c_int);
        msg_format(b"You cannot wield your %s with a shield.\x00" as *const u8
                       as *const libc::c_char, o_name.as_mut_ptr());
        return
    }
    if is_slot_ok(slot - 39 as libc::c_int + 24 as libc::c_int) != 0 {
        i_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset((slot -
                                                              39 as
                                                                  libc::c_int
                                                              +
                                                              24 as
                                                                  libc::c_int)
                                                             as isize) as
                *mut object_type;
        /* Extract the flags */
        object_flags(i_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                     &mut esp);
        /* Prevent shield from being put on if wielding 2H */
        if f4 as libc::c_long & 0x80 as libc::c_long != 0 &&
               (*i_ptr).k_idx as libc::c_int != 0 &&
               (*p_ptr).body_parts[(slot - 24 as libc::c_int) as usize] as
                   libc::c_int == 39 as libc::c_int {
            object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                        0 as libc::c_int);
            msg_format(b"You cannot wield your %s with a two-handed weapon.\x00"
                           as *const u8 as *const libc::c_char,
                       o_name.as_mut_ptr());
            return
        }
        if (*p_ptr).body_parts[(slot - 24 as libc::c_int) as usize] as
               libc::c_int == 39 as libc::c_int &&
               f4 as libc::c_long & 0x40 as libc::c_long != 0 {
            if get_check(b"Are you sure you want to restrict your fighting? \x00"
                             as *const u8 as *const libc::c_char) == 0 {
                return
            }
        }
    }
    /* Extract the flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    if is_slot_ok(slot - 24 as libc::c_int + 39 as libc::c_int) as libc::c_int
           != 0 &&
           (*p_ptr).inventory[(slot - 24 as libc::c_int + 39 as libc::c_int)
                                  as usize].k_idx as libc::c_int !=
               0 as libc::c_int &&
           f4 as libc::c_long & 0x40 as libc::c_long != 0 {
        if get_check(b"Are you sure you want to use this weapon with a shield?\x00"
                         as *const u8 as *const libc::c_char) == 0 {
            return
        }
    }
    /* Can we take off existing item */
    if slot != 50 as libc::c_int {
        if (*p_ptr).inventory[slot as usize].k_idx != 0 {
            if process_hooks(73 as libc::c_int,
                             b"(d)\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char, slot) != 0 {
                return
            }
        }
    } else if (*p_ptr).inventory[slot as usize].k_idx != 0 {
        if object_similar(&mut *(*p_ptr).inventory.as_mut_ptr().offset(slot as
                                                                           isize),
                          o_ptr) == 0 {
            if process_hooks(73 as libc::c_int,
                             b"(d)\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char, slot) != 0 {
                return
            }
        }
    }
    /* Take a turn */
    energy_use = 100 as libc::c_int;
    /* Get local object */
    q_ptr = &mut forge;
    /* Obtain local object */
    object_copy(q_ptr, o_ptr);
    if slot == 50 as libc::c_int { num = (*o_ptr).number as libc::c_int }
    /* Modify quantity */
    (*q_ptr).number = num as byte_hack;
    /* Decrease the item */
    inc_stack_size_ex(item, -num, OPTIMIZE, NO_DESCRIBE);
    /* Access the wield slot */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(slot as isize) as
            *mut object_type;
    /* Take off existing item */
    if slot != 50 as libc::c_int {
        if (*o_ptr).k_idx != 0 {
            /* Take off existing item */
            inven_takeoff(slot, 255 as libc::c_int,
                          0 as libc::c_int as bool_);
        }
    } else if (*o_ptr).k_idx != 0 {
        if object_similar(o_ptr, q_ptr) == 0 {
            /* Take off existing item */
            inven_takeoff(slot, 255 as libc::c_int,
                          0 as libc::c_int as bool_);
        } else {
            (*q_ptr).number =
                ((*q_ptr).number as libc::c_int +
                     (*o_ptr).number as libc::c_int) as byte_hack
        }
    }
    /* Wear the new stuff */
    object_copy(o_ptr, q_ptr);
    /* Increment the equip counter by hand */
    equip_cnt += 1;
    /* Where is the item now */
    if slot == 24 as libc::c_int {
        act = b"You are wielding\x00" as *const u8 as *const libc::c_char
    } else if slot == 27 as libc::c_int &&
                  (*o_ptr).tval as libc::c_int == 14 as libc::c_int {
        act = b"You are holding\x00" as *const u8 as *const libc::c_char
    } else if slot == 27 as libc::c_int {
        act = b"You are shooting with\x00" as *const u8 as *const libc::c_char
    } else if slot == 36 as libc::c_int {
        act = b"Your light source is\x00" as *const u8 as *const libc::c_char
    } else if slot == 50 as libc::c_int {
        act =
            b"In your quiver you have\x00" as *const u8 as *const libc::c_char
    } else if slot == 51 as libc::c_int {
        act = b"You are using\x00" as *const u8 as *const libc::c_char
    } else {
        act = b"You are wearing\x00" as *const u8 as *const libc::c_char
    }
    /* Describe the result */
    object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                3 as libc::c_int);
    /* Message */
    msg_format(b"%s %s (%c).\x00" as *const u8 as *const libc::c_char, act,
               o_name.as_mut_ptr(), index_to_label(slot) as libc::c_int);
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
    /* Take care of item sets */
    if (*o_ptr).name1 != 0 {
        wield_set((*o_ptr).name1 as s16b,
                  (*a_info.offset((*o_ptr).name1 as isize)).set,
                  0 as libc::c_int as bool_);
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
        ((*p_ptr).update as libc::c_long |
             (0x20 as libc::c_long | 0x40 as libc::c_long)) as u32b;
    /* Redraw monster hitpoint */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x10000000 as libc::c_long) as
            u32b;
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long |
                  0x8 as libc::c_long)) as u32b;
}
/*
 * Take off an item
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_takeoff() {
    let mut item: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Get an item */
    q = b"Take off which item? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You are not wearing anything to take off.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s, 0x1 as libc::c_int) == 0 { return }
    /* Get the item */
    o_ptr = get_object(item);
    /* Can we take it off */
    if process_hooks(73 as libc::c_int,
                     b"(d)\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char, item) != 0 {
        return
    }
    /* Item is cursed */
    if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 && wizard == 0
       {
        /* Oops */
        msg_print(b"Hmmm, it seems to be cursed.\x00" as *const u8 as
                      *const libc::c_char);
        /* Nope */
        return
    }
    /* Take a partial turn */
    energy_use = 50 as libc::c_int;
    /* Take off the item */
    inven_takeoff(item, 255 as libc::c_int, 0 as libc::c_int as bool_);
    /* Recalculate hitpoint */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x10 as libc::c_long) as u32b;
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x10000000 as libc::c_long) as
            u32b;
}
/*
 * Drop an item
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_drop() {
    let mut item: libc::c_int = 0;
    let mut amt: libc::c_int = 1 as libc::c_int;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Get an item */
    q = b"Drop which item? \x00" as *const u8 as *const libc::c_char;
    s = b"You have nothing to drop.\x00" as *const u8 as *const libc::c_char;
    if get_item(&mut item, q, s, 0x1 as libc::c_int | 0x2 as libc::c_int) == 0
       {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Can we drop */
    if process_hooks(15 as libc::c_int,
                     b"(d)\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char, item) != 0 {
        return
    }
    /* Hack -- Cannot remove cursed items */
    if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
        if item >= 24 as libc::c_int {
            /* Oops */
            msg_print(b"Hmmm, it seems to be cursed.\x00" as *const u8 as
                          *const libc::c_char);
            /* Nope */
            return
        } else {
            if f4 as libc::c_long & 0x40000000 as libc::c_long != 0 {
                /* Oops */
                msg_print(b"Hmmm, you seem to be unable to drop it.\x00" as
                              *const u8 as *const libc::c_char);
                /* Nope */
                return
            }
        }
    }
    /* See how many items */
    if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
        /* Get a quantity */
        amt = get_quantity(0 as cptr, (*o_ptr).number as s32b);
        /* Allow user abort */
        if amt <= 0 as libc::c_int { return }
    }
    /* Take a partial turn */
    energy_use = 50 as libc::c_int;
    /* Drop (some of) the item */
    inven_drop(item, amt, (*p_ptr).py as libc::c_int,
               (*p_ptr).px as libc::c_int, 0 as libc::c_int as bool_);
}
/*
 * Destroy an item
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_destroy() {
    let mut item: libc::c_int = 0;
    let mut amt: libc::c_int = 1 as libc::c_int;
    let mut old_number: libc::c_int = 0;
    let mut force: bool_ = 0 as libc::c_int as bool_;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut out_val: [libc::c_char; 160] = [0; 160];
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    /* Hack -- force destruction */
    if command_arg as libc::c_int > 0 as libc::c_int {
        force = 1 as libc::c_int as bool_
    }
    /* Get an item */
    q = b"Destroy which item? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have nothing to destroy.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s,
                0x2 as libc::c_int | 0x4 as libc::c_int | 0x10 as libc::c_int)
           == 0 {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* See how many items */
    if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
        /* Get a quantity */
        amt = get_quantity(0 as cptr, (*o_ptr).number as s32b);
        /* Allow user abort */
        if amt <= 0 as libc::c_int { return }
    }
    /* Describe the object */
    old_number = (*o_ptr).number as libc::c_int;
    (*o_ptr).number = amt as byte_hack;
    object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                3 as libc::c_int);
    (*o_ptr).number = old_number as byte_hack;
    /* Verify unless quantity given */
    if force == 0 {
        if !(auto_destroy as libc::c_int != 0 &&
                 object_value(o_ptr) < 1 as libc::c_int) {
            /* Make a verification */
            strnfmt(out_val.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                    b"Really destroy %s? \x00" as *const u8 as
                        *const libc::c_char, o_name.as_mut_ptr());
            if get_check(out_val.as_mut_ptr() as cptr) == 0 { return }
        }
    }
    /* Take no time, just like the automatizer */
    energy_use = 0 as libc::c_int;
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    if f4 as libc::c_long & 0x40000000 as libc::c_long != 0 &&
           (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
        /* Oops */
        msg_print(b"Hmmm, you seem to be unable to destroy it.\x00" as
                      *const u8 as *const libc::c_char);
        /* Nope */
        return
    }
    /* Artifacts cannot be destroyed */
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
            } else { 0 as libc::c_int }) != 0 ||
           (*o_ptr).art_name as libc::c_int != 0 {
        let mut feel: byte_hack = 8 as libc::c_int as byte_hack;
        energy_use = 0 as libc::c_int;
        /* Message */
        msg_format(b"You cannot destroy %s.\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr());
        /* Hack -- Handle icky artifacts */
        if (*o_ptr).ident as libc::c_int & 0x40 as libc::c_int != 0 {
            feel = 7 as libc::c_int as byte_hack
        }
        /* Hack -- inscribe the artifact */
        (*o_ptr).sense = feel;
        /* We have "felt" it (again) */
        (*o_ptr).ident =
            ((*o_ptr).ident as libc::c_int | 0x1 as libc::c_int) as byte_hack;
        /* Combine the pack */
        (*p_ptr).notice =
            ((*p_ptr).notice as libc::c_long | 0x1 as libc::c_long) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
        /* Done */
        return
    }
    /* Message */
    msg_format(b"You destroy %s.\x00" as *const u8 as *const libc::c_char,
               o_name.as_mut_ptr());
    sound(60 as libc::c_int);
    /* Create an automatizer rule */
    if automatizer_create != 0 {
        automatizer_add_rule(o_ptr, 1 as libc::c_int as bool_);
    }
    /*
	 * Hack -- If rods or wand are destroyed, the total maximum timeout or
	 * charges of the stack needs to be reduced, unless all the items are
	 * being destroyed. -LM-
	 */
    if (*o_ptr).tval as libc::c_int == 65 as libc::c_int &&
           amt < (*o_ptr).number as libc::c_int {
        (*o_ptr).pval -= (*o_ptr).pval * amt / (*o_ptr).number as libc::c_int
    }
    /* Eru wont be happy */
    if f3 as libc::c_long & 0x10000000 as libc::c_long != 0 {
        inc_piety(1 as libc::c_int,
                  -(10 as libc::c_int) *
                      (*k_info.offset((*o_ptr).k_idx as isize)).level as
                          libc::c_int);
    }
    /* Eliminate the item */
    inc_stack_size(item, -amt);
}
/*
 * Observe an item which has been *identify*-ed
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_observe() {
    let mut item: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Get an item */
    q = b"Examine which item? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have nothing to examine.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s,
                0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int)
           == 0 {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* Description */
    object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                3 as libc::c_int);
    /* Describe */
    cmsg_format(14 as libc::c_int as byte_hack,
                b"%s\x00" as *const u8 as *const libc::c_char,
                o_name.as_mut_ptr());
    /* Describe it fully */
    if object_out_desc(o_ptr, 0 as *mut FILE, 0 as libc::c_int as bool_,
                       1 as libc::c_int as bool_) == 0 {
        msg_print(b"You see nothing special.\x00" as *const u8 as
                      *const libc::c_char);
    };
}
/*
 * Remove the inscription from an object
 * XXX Mention item (when done)?
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_uninscribe() {
    let mut item: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Get an item */
    q = b"Un-inscribe which item? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have nothing to un-inscribe.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s,
                0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int)
           == 0 {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* Nothing to remove */
    if (*o_ptr).note == 0 {
        msg_print(b"That item had no inscription to remove.\x00" as *const u8
                      as *const libc::c_char);
        return
    }
    /* Message */
    msg_print(b"Inscription removed.\x00" as *const u8 as
                  *const libc::c_char);
    /* Remove the incription */
    (*o_ptr).note = 0 as libc::c_int as u16b;
    /* Combine the pack */
    (*p_ptr).notice =
        ((*p_ptr).notice as libc::c_long | 0x1 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
}
/*
 * Inscribe an object with a comment
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_inscribe() {
    let mut item: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut out_val: [libc::c_char; 80] = [0; 80];
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Get an item */
    q = b"Inscribe which item? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have nothing to inscribe.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s,
                0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int)
           == 0 {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* Describe the activity */
    object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                3 as libc::c_int);
    /* Message */
    msg_format(b"Inscribing %s.\x00" as *const u8 as *const libc::c_char,
               o_name.as_mut_ptr());
    msg_print(0 as cptr);
    /* Start with nothing */
    strcpy(out_val.as_mut_ptr(), b"\x00" as *const u8 as *const libc::c_char);
    /* Use old inscription */
    if (*o_ptr).note != 0 {
        /* Start with the old inscription */
        strcpy(out_val.as_mut_ptr(), quark_str((*o_ptr).note as s16b));
    }
    /* Get a new inscription (possibly empty) */
    if get_string(b"Inscription: \x00" as *const u8 as *const libc::c_char,
                  out_val.as_mut_ptr(), 80 as libc::c_int) != 0 {
        /* Save the inscription */
        (*o_ptr).note = quark_add(out_val.as_mut_ptr() as cptr) as u16b;
        /* Combine the pack */
        (*p_ptr).notice =
            ((*p_ptr).notice as libc::c_long | 0x1 as libc::c_long) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b
    };
}
/*
 * An "item_tester_hook" for refilling lanterns
 */
unsafe extern "C" fn item_tester_refill_lantern(mut o_ptr: *mut object_type)
 -> bool_ {
    /* Flasks of oil are okay */
    if (*o_ptr).tval as libc::c_int == 77 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Lanterns are okay */
    if (*o_ptr).tval as libc::c_int == 39 as libc::c_int &&
           (*o_ptr).sval as libc::c_int == 1 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Assume not okay */
    return 0 as libc::c_int as bool_;
}
/*
 * Refill the players lamp (from the pack or floor)
 */
unsafe extern "C" fn do_cmd_refill_lamp() {
    let mut item: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut j_ptr: *mut object_type = 0 as *mut object_type;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Restrict the choices */
    item_tester_hook =
        Some(item_tester_refill_lantern as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Get an item */
    q = b"Refill with which flask? \x00" as *const u8 as *const libc::c_char;
    s = b"You have no flasks of oil.\x00" as *const u8 as *const libc::c_char;
    if get_item(&mut item, q, s, 0x2 as libc::c_int | 0x4 as libc::c_int) == 0
       {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* Take a partial turn */
    energy_use = 50 as libc::c_int;
    /* Access the lantern */
    j_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(36 as libc::c_int as
                                                         isize) as
            *mut object_type;
    /* Refuel */
    if (*o_ptr).tval as libc::c_int == 77 as libc::c_int {
        (*j_ptr).timeout =
            ((*j_ptr).timeout as libc::c_int + (*o_ptr).pval) as s16b
    } else {
        (*j_ptr).timeout =
            ((*j_ptr).timeout as libc::c_int +
                 (*o_ptr).timeout as libc::c_int) as s16b
    }
    /* Message */
    msg_print(b"You fuel your lamp.\x00" as *const u8 as *const libc::c_char);
    /* Comment */
    if (*j_ptr).timeout as libc::c_int >= 15000 as libc::c_int {
        (*j_ptr).timeout = 15000 as libc::c_int as s16b;
        msg_print(b"Your lamp is full.\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Decrease the item stack */
    inc_stack_size(item, -(1 as libc::c_int));
    /* Recalculate torch */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x2 as libc::c_long) as u32b;
}
/*
 * An "item_tester_hook" for refilling torches
 */
unsafe extern "C" fn item_tester_refill_torch(mut o_ptr: *mut object_type)
 -> bool_ {
    /* Torches are okay */
    if (*o_ptr).tval as libc::c_int == 39 as libc::c_int &&
           (*o_ptr).sval as libc::c_int == 0 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Assume not okay */
    return 0 as libc::c_int as bool_;
}
/*
 * Refuel the players torch (from the pack or floor)
 */
unsafe extern "C" fn do_cmd_refill_torch() {
    let mut item: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut j_ptr: *mut object_type = 0 as *mut object_type;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Restrict the choices */
    item_tester_hook =
        Some(item_tester_refill_torch as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Get an item */
    q = b"Refuel with which torch? \x00" as *const u8 as *const libc::c_char;
    s = b"You have no extra torches.\x00" as *const u8 as *const libc::c_char;
    if get_item(&mut item, q, s, 0x2 as libc::c_int | 0x4 as libc::c_int) == 0
       {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* Take a partial turn */
    energy_use = 50 as libc::c_int;
    /* Access the primary torch */
    j_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(36 as libc::c_int as
                                                         isize) as
            *mut object_type;
    /* Refuel */
    (*j_ptr).timeout =
        ((*j_ptr).timeout as libc::c_int +
             ((*o_ptr).timeout as libc::c_int + 5 as libc::c_int)) as s16b;
    /* Message */
    msg_print(b"You combine the torches.\x00" as *const u8 as
                  *const libc::c_char);
    /* Over-fuel message */
    if (*j_ptr).timeout as libc::c_int >= 5000 as libc::c_int {
        (*j_ptr).timeout = 5000 as libc::c_int as s16b;
        msg_print(b"Your torch is fully fueled.\x00" as *const u8 as
                      *const libc::c_char);
    } else {
        /* Refuel message */
        msg_print(b"Your torch glows more brightly.\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Decrease the item stack */
    inc_stack_size(item, -(1 as libc::c_int));
    /* Recalculate torch */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x2 as libc::c_long) as u32b;
}
/*
 * Refill the players lamp, or restock his torches
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_refill() {
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    /* Get the light */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(36 as libc::c_int as
                                                         isize) as
            *mut object_type;
    /* It is nothing */
    if (*o_ptr).tval as libc::c_int != 39 as libc::c_int {
        msg_print(b"You are not wielding a light.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    if f4 as libc::c_long & 0x10000000 as libc::c_long != 0 {
        /* It's a torch */
        if (*o_ptr).sval as libc::c_int == 0 as libc::c_int ||
               (*o_ptr).sval as libc::c_int == 2 as libc::c_int {
            do_cmd_refill_torch();
        } else if (*o_ptr).sval as libc::c_int == 1 as libc::c_int ||
                      (*o_ptr).sval as libc::c_int == 3 as libc::c_int ||
                      (*o_ptr).sval as libc::c_int == 4 as libc::c_int {
            do_cmd_refill_lamp();
        }
    } else {
        /* It's a lamp */
        /* No torch to refill */
        msg_print(b"Your light cannot be refilled.\x00" as *const u8 as
                      *const libc::c_char);
    };
}
/*
 * Target command
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_target() {
    /* Target set */
    if target_set(0x1 as libc::c_int) != 0 {
        msg_print(b"Target Selected.\x00" as *const u8 as
                      *const libc::c_char);
    } else {
        /* Target aborted */
        msg_print(b"Target Aborted.\x00" as *const u8 as *const libc::c_char);
    };
}
/*
 * Look command
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_look() {
    /* Look around */
    if target_set(0x2 as libc::c_int) != 0 {
        msg_print(b"Target Selected.\x00" as *const u8 as
                      *const libc::c_char);
    };
}
/*
 * Allow the player to examine other sectors on the map
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_locate() {
    let mut dir: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut panel_hgt: libc::c_int = 0;
    let mut panel_wid: libc::c_int = 0;
    let mut tmp_val: [libc::c_char; 80] = [0; 80];
    let mut out_val: [libc::c_char; 160] = [0; 160];
    /* Retrieve size of the Angband window */
    Term_get_size(&mut panel_wid, &mut panel_hgt);
    /* Calcurate size of the dungeon map area */
    panel_hgt =
        (panel_hgt - (1 as libc::c_int + 1 as libc::c_int)) /
            2 as libc::c_int;
    panel_wid =
        (panel_wid - (13 as libc::c_int + 1 as libc::c_int)) /
            2 as libc::c_int;
    /* Start at current panel */
    y1 = panel_row_min as libc::c_int;
    y2 = y1;
    x1 = panel_col_min as libc::c_int;
    x2 = x1;
    loop 
         /* Show panels until done */
         /* Describe the location */
         {
        if y2 == y1 && x2 == x1 {
            tmp_val[0 as libc::c_int as usize] =
                '\u{0}' as i32 as libc::c_char
        } else {
            strnfmt(tmp_val.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                    b"%s%s of\x00" as *const u8 as *const libc::c_char,
                    if y2 < y1 {
                        b" North\x00" as *const u8 as *const libc::c_char
                    } else if y2 > y1 {
                        b" South\x00" as *const u8 as *const libc::c_char
                    } else { b"\x00" as *const u8 as *const libc::c_char },
                    if x2 < x1 {
                        b" West\x00" as *const u8 as *const libc::c_char
                    } else if x2 > x1 {
                        b" East\x00" as *const u8 as *const libc::c_char
                    } else { b"\x00" as *const u8 as *const libc::c_char });
        }
        /* Prepare to ask which way to look */
        if panel_hgt == 11 as libc::c_int && panel_wid == 33 as libc::c_int {
            /* Avoid surprising the standard screen users */
            strnfmt(out_val.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                    b"Map sector [%d,%d], which is%s your sector. Direction?\x00"
                        as *const u8 as *const libc::c_char, y2 / panel_hgt,
                    x2 / panel_wid, tmp_val.as_mut_ptr());
        } else {
            /* Big screen */
            /* Panels are measured by current map area size */
            strnfmt(out_val.as_mut_ptr(), 160 as libc::c_int as uint_hack,
                    b"Map sector [%d(%02d),%d(%02d)], which is%s your sector. Direction?\x00"
                        as *const u8 as *const libc::c_char, y2 / panel_hgt,
                    y2 % panel_hgt, x2 / panel_wid, x2 % panel_wid,
                    tmp_val.as_mut_ptr());
        }
        /* Assume no direction */
        dir = 0 as libc::c_int;
        /* Get a direction */
        while dir == 0 {
            let mut ch: libc::c_char = 0;
            /* Get a command (or cancel) */
            if get_com(out_val.as_mut_ptr() as cptr, &mut ch) == 0 { break ; }
            /* Extract the action (if any) */
            dir = get_keymap_dir(ch);
            /* Error */
            if dir == 0 { bell(); }
        }
        /* No direction */
        if dir == 0 { break ; }
        /* Apply the motion */
        if change_panel(ddy[dir as usize] as libc::c_int,
                        ddx[dir as usize] as libc::c_int) != 0 {
            y2 = panel_row_min as libc::c_int;
            x2 = panel_col_min as libc::c_int
        }
    }
    /* Recenter the map around the player */
    verify_panel();
    /* Update stuff */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1000000 as libc::c_long) as u32b;
    /* Redraw map */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
    /* Handle stuff */
    handle_stuff();
}
/*
 * The table of "symbol info" -- each entry is a string of the form
 * "X:desc" where "X" is the trigger, and "desc" is the "info".
 */
static mut ident_info: [cptr; 92] =
    [b" :A dark grid\x00" as *const u8 as *const libc::c_char,
     b"!:A potion (or oil)\x00" as *const u8 as *const libc::c_char,
     b"\":An amulet (or necklace)\x00" as *const u8 as *const libc::c_char,
     b"#:A wall (or secret door)\x00" as *const u8 as *const libc::c_char,
     b"$:Treasure (gold or gems)\x00" as *const u8 as *const libc::c_char,
     b"%:A vein (magma or quartz)\x00" as *const u8 as *const libc::c_char,
     b"\':An open door\x00" as *const u8 as *const libc::c_char,
     b"(:Soft armor\x00" as *const u8 as *const libc::c_char,
     b"):A shield\x00" as *const u8 as *const libc::c_char,
     b"*:A vein with treasure\x00" as *const u8 as *const libc::c_char,
     b"+:A closed door\x00" as *const u8 as *const libc::c_char,
     b",:Food (or mushroom patch)\x00" as *const u8 as *const libc::c_char,
     b"-:A wand (or rod)\x00" as *const u8 as *const libc::c_char,
     b".:Floor\x00" as *const u8 as *const libc::c_char,
     b"/:A polearm (Axe/Pike/etc)\x00" as *const u8 as *const libc::c_char,
     b"0:An altar\x00" as *const u8 as *const libc::c_char,
     b"1:Entrance to General Store\x00" as *const u8 as *const libc::c_char,
     b"2:Entrance to Armory\x00" as *const u8 as *const libc::c_char,
     b"3:Entrance to Weaponsmith\x00" as *const u8 as *const libc::c_char,
     b"4:Entrance to Temple\x00" as *const u8 as *const libc::c_char,
     b"5:Entrance to Alchemy shop\x00" as *const u8 as *const libc::c_char,
     b"6:Entrance to Magic store\x00" as *const u8 as *const libc::c_char,
     b"7:Entrance to Black Market\x00" as *const u8 as *const libc::c_char,
     b"8:Entrance to your home\x00" as *const u8 as *const libc::c_char,
     b"9:Entrance to Bookstore\x00" as *const u8 as *const libc::c_char,
     b"::Rubble\x00" as *const u8 as *const libc::c_char,
     b";:A glyph of warding / explosive rune\x00" as *const u8 as
         *const libc::c_char,
     b"<:An up staircase\x00" as *const u8 as *const libc::c_char,
     b"=:A ring\x00" as *const u8 as *const libc::c_char,
     b">:A down staircase\x00" as *const u8 as *const libc::c_char,
     b"?:A scroll\x00" as *const u8 as *const libc::c_char,
     b"@:You\x00" as *const u8 as *const libc::c_char,
     b"A:Angel\x00" as *const u8 as *const libc::c_char,
     b"B:Bird\x00" as *const u8 as *const libc::c_char,
     b"C:Canine\x00" as *const u8 as *const libc::c_char,
     b"D:Ancient Dragon/Wyrm\x00" as *const u8 as *const libc::c_char,
     b"E:Elemental\x00" as *const u8 as *const libc::c_char,
     b"F:Dragon Fly\x00" as *const u8 as *const libc::c_char,
     b"G:Ghost\x00" as *const u8 as *const libc::c_char,
     b"H:Hybrid\x00" as *const u8 as *const libc::c_char,
     b"I:Insect\x00" as *const u8 as *const libc::c_char,
     b"J:Snake\x00" as *const u8 as *const libc::c_char,
     b"K:Killer Beetle\x00" as *const u8 as *const libc::c_char,
     b"L:Lich\x00" as *const u8 as *const libc::c_char,
     b"M:Multi-Headed Reptile\x00" as *const u8 as *const libc::c_char,
     b"O:Ogre\x00" as *const u8 as *const libc::c_char,
     b"P:Giant Humanoid\x00" as *const u8 as *const libc::c_char,
     b"Q:Quylthulg (Pulsing Flesh Mound)\x00" as *const u8 as
         *const libc::c_char,
     b"R:Reptile/Amphibian\x00" as *const u8 as *const libc::c_char,
     b"S:Spider/Scorpion/Tick\x00" as *const u8 as *const libc::c_char,
     b"T:Troll\x00" as *const u8 as *const libc::c_char,
     b"U:Major Demon\x00" as *const u8 as *const libc::c_char,
     b"V:Vampire\x00" as *const u8 as *const libc::c_char,
     b"W:Wight/Wraith/etc\x00" as *const u8 as *const libc::c_char,
     b"X:Xorn/Xaren/etc\x00" as *const u8 as *const libc::c_char,
     b"Y:Yeti\x00" as *const u8 as *const libc::c_char,
     b"Z:Zephyr Hound\x00" as *const u8 as *const libc::c_char,
     b"[:Hard armor\x00" as *const u8 as *const libc::c_char,
     b"\\:A hafted weapon (mace/whip/etc)\x00" as *const u8 as
         *const libc::c_char,
     b"]:Misc. armor\x00" as *const u8 as *const libc::c_char,
     b"^:A trap\x00" as *const u8 as *const libc::c_char,
     b"_:A staff\x00" as *const u8 as *const libc::c_char,
     b"a:Ant\x00" as *const u8 as *const libc::c_char,
     b"b:Bat\x00" as *const u8 as *const libc::c_char,
     b"c:Centipede\x00" as *const u8 as *const libc::c_char,
     b"d:Dragon\x00" as *const u8 as *const libc::c_char,
     b"e:Floating Eye\x00" as *const u8 as *const libc::c_char,
     b"f:Feline\x00" as *const u8 as *const libc::c_char,
     b"g:Golem\x00" as *const u8 as *const libc::c_char,
     b"h:Hobbit/Elf/Dwarf\x00" as *const u8 as *const libc::c_char,
     b"i:Icky Thing\x00" as *const u8 as *const libc::c_char,
     b"j:Jelly\x00" as *const u8 as *const libc::c_char,
     b"k:Kobold\x00" as *const u8 as *const libc::c_char,
     b"l:Louse\x00" as *const u8 as *const libc::c_char,
     b"m:Mold\x00" as *const u8 as *const libc::c_char,
     b"n:Naga\x00" as *const u8 as *const libc::c_char,
     b"o:Orc\x00" as *const u8 as *const libc::c_char,
     b"p:Person/Human\x00" as *const u8 as *const libc::c_char,
     b"q:Quadruped\x00" as *const u8 as *const libc::c_char,
     b"r:Rodent\x00" as *const u8 as *const libc::c_char,
     b"s:Skeleton\x00" as *const u8 as *const libc::c_char,
     b"t:Townsperson\x00" as *const u8 as *const libc::c_char,
     b"u:Minor Demon\x00" as *const u8 as *const libc::c_char,
     b"v:Vortex\x00" as *const u8 as *const libc::c_char,
     b"w:Worm/Worm-Mass\x00" as *const u8 as *const libc::c_char,
     b"y:Yeek\x00" as *const u8 as *const libc::c_char,
     b"z:Zombie/Mummy\x00" as *const u8 as *const libc::c_char,
     b"{:A missile (arrow/bolt/shot)\x00" as *const u8 as *const libc::c_char,
     b"|:An edged weapon (sword/dagger/etc)\x00" as *const u8 as
         *const libc::c_char,
     b"}:A launcher (bow/crossbow/sling)\x00" as *const u8 as
         *const libc::c_char,
     b"~:A tool (or miscellaneous item)\x00" as *const u8 as
         *const libc::c_char, 0 as cptr];
/*
 * Sorting hook -- Comp function -- see below
 *
 * We use "u" to point to array of monster indexes,
 * and "v" to select the type of sorting to perform on "u".
 */
unsafe extern "C" fn ang_sort_comp_hook(mut u: vptr, mut v: vptr,
                                        mut a: libc::c_int,
                                        mut b: libc::c_int) -> bool_ {
    let mut who: *mut u16b = u as *mut u16b;
    let mut why: *mut u16b = v as *mut u16b;
    let mut w1: libc::c_int = *who.offset(a as isize) as libc::c_int;
    let mut w2: libc::c_int = *who.offset(b as isize) as libc::c_int;
    let mut z1: libc::c_int = 0;
    let mut z2: libc::c_int = 0;
    /* Sort by player kills */
    if *why as libc::c_int >= 4 as libc::c_int {
        /* Extract player kills */
        z1 = (*r_info.offset(w1 as isize)).r_pkills as libc::c_int;
        z2 = (*r_info.offset(w2 as isize)).r_pkills as libc::c_int;
        /* Compare player kills */
        if z1 < z2 { return 1 as libc::c_int as bool_ }
        if z1 > z2 { return 0 as libc::c_int as bool_ }
    }
    /* Sort by total kills */
    if *why as libc::c_int >= 3 as libc::c_int {
        /* Extract total kills */
        z1 = (*r_info.offset(w1 as isize)).r_tkills as libc::c_int;
        z2 = (*r_info.offset(w2 as isize)).r_tkills as libc::c_int;
        /* Compare total kills */
        if z1 < z2 { return 1 as libc::c_int as bool_ }
        if z1 > z2 { return 0 as libc::c_int as bool_ }
    }
    /* Sort by monster level */
    if *why as libc::c_int >= 2 as libc::c_int {
        /* Extract levels */
        z1 = (*r_info.offset(w1 as isize)).level as libc::c_int;
        z2 = (*r_info.offset(w2 as isize)).level as libc::c_int;
        /* Compare levels */
        if z1 < z2 { return 1 as libc::c_int as bool_ }
        if z1 > z2 { return 0 as libc::c_int as bool_ }
    }
    /* Sort by monster experience */
    if *why as libc::c_int >= 1 as libc::c_int {
        /* Extract experience */
        z1 = (*r_info.offset(w1 as isize)).mexp;
        z2 = (*r_info.offset(w2 as isize)).mexp;
        /* Compare experience */
        if z1 < z2 { return 1 as libc::c_int as bool_ }
        if z1 > z2 { return 0 as libc::c_int as bool_ }
    }
    /* Compare indexes */
    return (w1 <= w2) as libc::c_int as bool_;
}
/*
 * Sorting hook -- Swap function -- see below
 *
 * We use "u" to point to array of monster indexes,
 * and "v" to select the type of sorting to perform.
 */
unsafe extern "C" fn ang_sort_swap_hook(mut u: vptr, mut v: vptr,
                                        mut a: libc::c_int,
                                        mut b: libc::c_int) {
    let mut who: *mut u16b = u as *mut u16b;
    let mut holder: u16b = 0;
    /* XXX XXX */
    v = if !v.is_null() { v } else { 0 as vptr };
    /* Swap */
    holder = *who.offset(a as isize);
    *who.offset(a as isize) = *who.offset(b as isize);
    *who.offset(b as isize) = holder;
}
/*
 * Hack -- Display the "name" and "attr/chars" of a monster race
 */
unsafe extern "C" fn roff_top(mut r_idx: libc::c_int) {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    let mut a1: byte_hack = 0;
    let mut a2: byte_hack = 0;
    let mut c1: libc::c_char = 0;
    let mut c2: libc::c_char = 0;
    /* Access the chars */
    c1 = (*r_ptr).d_char;
    c2 = (*r_ptr).x_char;
    /* Access the attrs */
    a1 = (*r_ptr).d_attr;
    a2 = (*r_ptr).x_attr;
    /* Clear the top line */
    Term_erase(0 as libc::c_int, 0 as libc::c_int, 255 as libc::c_int);
    /* Reset the cursor */
    Term_gotoxy(0 as libc::c_int, 0 as libc::c_int);
    /* A title (use "The" for non-uniques) */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint == 0 {
        Term_addstr(-(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                    b"The \x00" as *const u8 as *const libc::c_char);
    }
    /* Dump the name */
    Term_addstr(-(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                r_name.offset((*r_ptr).name as isize) as cptr);
    /* Append the "standard" attr/char info */
    Term_addstr(-(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                b" (\'\x00" as *const u8 as *const libc::c_char);
    Term_addch(a1, c1);
    if use_bigtile as libc::c_int != 0 &&
           a1 as libc::c_int & 0x80 as libc::c_int != 0 {
        Term_addch(255 as libc::c_int as byte_hack,
                   255 as libc::c_int as libc::c_char);
    }
    Term_addstr(-(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                b"\')\x00" as *const u8 as *const libc::c_char);
    /* Append the "optional" attr/char info */
    Term_addstr(-(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                b"/(\'\x00" as *const u8 as *const libc::c_char);
    Term_addch(a2, c2);
    if use_bigtile as libc::c_int != 0 &&
           a2 as libc::c_int & 0x80 as libc::c_int != 0 {
        Term_addch(255 as libc::c_int as byte_hack,
                   255 as libc::c_int as libc::c_char);
    }
    Term_addstr(-(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                b"\'):\x00" as *const u8 as *const libc::c_char);
}
/*
 * Identify a character, allow recall of monsters
 *
 * Several "special" responses recall "multiple" monsters:
 *   ^A (all monsters)
 *   ^U (all unique monsters)
 *   ^N (all non-unique monsters)
 *   ^M (case insensitive name search)
 *
 * The responses may be sorted in several ways, see below.
 *
 * Note that the player ghosts are ignored. XXX XXX XXX
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_query_symbol() {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut r_idx: libc::c_int = 0;
    let mut sym: libc::c_char = 0;
    let mut query: libc::c_char = 0;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut all: bool_ = 0 as libc::c_int as bool_;
    let mut uniq: bool_ = 0 as libc::c_int as bool_;
    let mut norm: bool_ = 0 as libc::c_int as bool_;
    let mut name: bool_ = 0 as libc::c_int as bool_;
    let mut temp: [libc::c_char; 80] =
        *::std::mem::transmute::<&[u8; 80],
                                 &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let mut recall: bool_ = 0 as libc::c_int as bool_;
    let mut why: u16b = 0 as libc::c_int as u16b;
    let mut who: *mut u16b = 0 as *mut u16b;
    /* Get a character, or abort */
    if get_com(b"Enter character to be identified, or (Ctrl-A, Ctrl-U, Ctrl-N, Ctrl-M):\x00"
                   as *const u8 as *const libc::c_char, &mut sym) == 0 {
        return
    }
    /* Find that character info, and describe it */
    i = 0 as libc::c_int;
    while !ident_info[i as usize].is_null() {
        if sym as libc::c_int ==
               *ident_info[i as usize].offset(0 as libc::c_int as isize) as
                   libc::c_int {
            break ;
        }
        i += 1
    }
    /* Describe */
    if sym as libc::c_int == 'A' as i32 & 0x1f as libc::c_int {
        all = 1 as libc::c_int as bool_;
        strcpy(buf.as_mut_ptr(),
               b"Full monster list.\x00" as *const u8 as *const libc::c_char);
    } else if sym as libc::c_int == 'U' as i32 & 0x1f as libc::c_int {
        uniq = 1 as libc::c_int as bool_;
        all = uniq;
        strcpy(buf.as_mut_ptr(),
               b"Unique monster list.\x00" as *const u8 as
                   *const libc::c_char);
    } else if sym as libc::c_int == 'N' as i32 & 0x1f as libc::c_int {
        norm = 1 as libc::c_int as bool_;
        all = norm;
        strcpy(buf.as_mut_ptr(),
               b"Non-unique monster list.\x00" as *const u8 as
                   *const libc::c_char);
    } else if sym as libc::c_int == 'M' as i32 & 0x1f as libc::c_int {
        name = 1 as libc::c_int as bool_;
        all = name;
        if get_string(b"Name:\x00" as *const u8 as *const libc::c_char,
                      temp.as_mut_ptr(), 70 as libc::c_int) == 0 {
            return
        }
        strnfmt(buf.as_mut_ptr(), 128 as libc::c_int as uint_hack,
                b"Monsters with a name \"%s\"\x00" as *const u8 as
                    *const libc::c_char, temp.as_mut_ptr());
        strlower(temp.as_mut_ptr());
    } else if !ident_info[i as usize].is_null() {
        strnfmt(buf.as_mut_ptr(), 128 as libc::c_int as uint_hack,
                b"%c - %s.\x00" as *const u8 as *const libc::c_char,
                sym as libc::c_int,
                ident_info[i as usize].offset(2 as libc::c_int as isize));
    } else {
        strnfmt(buf.as_mut_ptr(), 128 as libc::c_int as uint_hack,
                b"%c - %s.\x00" as *const u8 as *const libc::c_char,
                sym as libc::c_int,
                b"Unknown Symbol\x00" as *const u8 as *const libc::c_char);
    }
    /* Display the result */
    prt(buf.as_mut_ptr() as cptr, 0 as libc::c_int, 0 as libc::c_int);
    /* Allocate the "who" array */
    who =
        memset(ralloc((max_r_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<u16b>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_r_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<u16b>()
                                                    as libc::c_ulong)) as
            *mut u16b;
    let mut current_block_34: u64;
    /* Collect matching monsters */
    n = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < max_r_idx as libc::c_int {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset(i as isize) as *mut monster_race;
        /* Nothing to recall */
        if !(cheat_know == 0 && (*r_ptr).r_sights == 0) {
            /* Require non-unique monsters if needed */
            if !(norm as libc::c_int != 0 &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint !=
                         0) {
                /* Require unique monsters if needed */
                if !(uniq as libc::c_int != 0 &&
                         (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint
                             == 0) {
                    /* Require monsters with the name requested if needed */
                    if name != 0 {
                        let mut mon_name: [libc::c_char; 80] = [0; 80];
                        strcpy(mon_name.as_mut_ptr(),
                               r_name.offset((*r_ptr).name as isize));
                        strlower(mon_name.as_mut_ptr());
                        if strstr(mon_name.as_mut_ptr(),
                                  temp.as_mut_ptr()).is_null() {
                            current_block_34 = 10692455896603418738;
                        } else { current_block_34 = 1434579379687443766; }
                    } else { current_block_34 = 1434579379687443766; }
                    match current_block_34 {
                        10692455896603418738 => { }
                        _ => {
                            /* Collect "appropriate" monsters */
                            if all as libc::c_int != 0 ||
                                   (*r_ptr).d_char as libc::c_int ==
                                       sym as libc::c_int {
                                let fresh0 = n;
                                n = n + 1;
                                *who.offset(fresh0 as isize) = i as u16b
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    /* Nothing to recall */
    if n == 0 {
        /* Free the "who" array */
        who =
            rnfree(who as vptr,
                   (max_r_idx as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<u16b>()
                                                        as libc::c_ulong)) as
                *mut u16b;
        return
    }
    /* Prompt XXX XXX XXX */
    put_str(b"Recall details? (k/p/y/n): \x00" as *const u8 as
                *const libc::c_char, 0 as libc::c_int, 40 as libc::c_int);
    /* Query */
    query = inkey();
    /* Restore */
    prt(buf.as_mut_ptr() as cptr, 0 as libc::c_int, 0 as libc::c_int);
    /* Sort by kills (and level) */
    if query as libc::c_int == 'k' as i32 {
        why = 4 as libc::c_int as u16b;
        query = 'y' as i32 as libc::c_char
    }
    /* Sort by level */
    if query as libc::c_int == 'p' as i32 {
        why = 2 as libc::c_int as u16b;
        query = 'y' as i32 as libc::c_char
    }
    /* Catch "escape" */
    if query as libc::c_int != 'y' as i32 {
        /* Free the "who" array */
        who =
            rnfree(who as vptr,
                   (max_r_idx as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<u16b>()
                                                        as libc::c_ulong)) as
                *mut u16b;
        return
    }
    /* Sort if needed */
    if why != 0 {
        /* Select the sort method */
        ang_sort_comp =
            Some(ang_sort_comp_hook as
                     unsafe extern "C" fn(_: vptr, _: vptr, _: libc::c_int,
                                          _: libc::c_int) -> bool_);
        ang_sort_swap =
            Some(ang_sort_swap_hook as
                     unsafe extern "C" fn(_: vptr, _: vptr, _: libc::c_int,
                                          _: libc::c_int) -> ());
        /* Sort the array */
        ang_sort(who as vptr, &mut why as *mut u16b as vptr, n);
    }
    /* Start at the end */
    i = n - 1 as libc::c_int;
    loop 
         /* Scan the monster memory */
         /* Extract a race */
         {
        r_idx = *who.offset(i as isize) as libc::c_int;
        /* Hack -- Auto-recall */
        monster_race_track(r_idx, 0 as libc::c_int);
        /* Hack -- Handle stuff */
        handle_stuff();
        /* Hack -- Begin the prompt */
        roff_top(r_idx);
        /* Hack -- Complete the prompt */
        Term_addstr(-(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                    b" [(r)ecall, ESC]\x00" as *const u8 as
                        *const libc::c_char);
        loop 
             /* Interact */
             /* Recall */
             {
            if recall != 0 {
                /* Save the screen */
                character_icky = 1 as libc::c_int as bool_;
                Term_save();
                /* Recall on screen */
                screen_roff(*who.offset(i as isize) as libc::c_int,
                            0 as libc::c_int, 0 as libc::c_int);
                /* Hack -- Complete the prompt (again) */
                Term_addstr(-(1 as libc::c_int),
                            1 as libc::c_int as byte_hack,
                            b" [(r)ecall, ESC]\x00" as *const u8 as
                                *const libc::c_char);
            }
            /* Command */
            query = inkey();
            /* Unrecall */
            if recall != 0 {
                /* Restore */
                Term_load();
                character_icky = 0 as libc::c_int as bool_
            }
            /* Normal commands */
            if query as libc::c_int != 'r' as i32 { break ; }
            /* Toggle recall */
            recall = (recall == 0) as libc::c_int as bool_
        }
        /* Stop scanning */
        if query as libc::c_int == '\u{1b}' as i32 { break ; }
        /* Move to "prev" monster */
        if query as libc::c_int == '-' as i32 {
            i += 1;
            if !(i == n) { continue ; }
            i = 0 as libc::c_int;
            if expand_list == 0 { break ; }
        } else {
            /* Move to "next" monster */
            let fresh1 = i;
            i = i - 1;
            if !(fresh1 == 0 as libc::c_int) { continue ; }
            i = n - 1 as libc::c_int;
            if expand_list == 0 { break ; }
        }
    }
    /* Re-display the identity */
    prt(buf.as_mut_ptr() as cptr, 0 as libc::c_int, 0 as libc::c_int);
    /* Free the "who" array */
    who =
        rnfree(who as vptr,
               (max_r_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<u16b>()
                                                    as libc::c_ulong)) as
            *mut u16b;
}
/*
 *  research_mon
 *  -KMW-
 */
#[no_mangle]
pub unsafe extern "C" fn research_mon() -> bool_ {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut r_idx: libc::c_int = 0;
    let mut sym: libc::c_char = 0;
    let mut query: libc::c_char = 0;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut oldkills: s16b = 0;
    let mut oldwake: byte_hack = 0;
    let mut oldcheat: bool_ = 0;
    let mut all: bool_ = 0 as libc::c_int as bool_;
    let mut uniq: bool_ = 0 as libc::c_int as bool_;
    let mut norm: bool_ = 0 as libc::c_int as bool_;
    let mut notpicked: bool_ = 0;
    let mut recall: bool_ = 0 as libc::c_int as bool_;
    let mut why: u16b = 0 as libc::c_int as u16b;
    let mut r2_ptr: *mut monster_race = 0 as *mut monster_race;
    let mut who: *mut u16b = 0 as *mut u16b;
    /* Hack -- Remember "cheat_know" flag */
    oldcheat = cheat_know;
    /* Get a character, or abort */
    if get_com(b"Enter character of monster: \x00" as *const u8 as
                   *const libc::c_char, &mut sym) == 0 {
        return 1 as libc::c_int as bool_
    }
    /* Allocate the "who" array */
    who =
        memset(ralloc((max_r_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<u16b>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_r_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<u16b>()
                                                    as libc::c_ulong)) as
            *mut u16b;
    /* Find that character info, and describe it */
    i = 0 as libc::c_int;
    while !ident_info[i as usize].is_null() {
        if sym as libc::c_int ==
               *ident_info[i as usize].offset(0 as libc::c_int as isize) as
                   libc::c_int {
            break ;
        }
        i += 1
    }
    if !ident_info[i as usize].is_null() {
        strnfmt(buf.as_mut_ptr(), 128 as libc::c_int as uint_hack,
                b"%c - %s.\x00" as *const u8 as *const libc::c_char,
                sym as libc::c_int,
                ident_info[i as usize].offset(2 as libc::c_int as isize));
    } else {
        strnfmt(buf.as_mut_ptr(), 128 as libc::c_int as uint_hack,
                b"%c - %s.\x00" as *const u8 as *const libc::c_char,
                sym as libc::c_int,
                b"Unknown Symbol\x00" as *const u8 as *const libc::c_char);
    }
    /* Display the result */
    prt(buf.as_mut_ptr() as cptr, 16 as libc::c_int, 10 as libc::c_int);
    /* Collect matching monsters */
    n = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < max_r_idx as libc::c_int {
        let mut r_ptr: *mut monster_race =
            &mut *r_info.offset(i as isize) as *mut monster_race;
        /* Hack -- Force "cheat_know" */
        cheat_know = 1 as libc::c_int as bool_;
        /* Nothing to recall */
        if !(cheat_know == 0 && (*r_ptr).r_sights == 0) {
            /* Require non-unique monsters if needed */
            if !(norm as libc::c_int != 0 &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint !=
                         0) {
                /* Require unique monsters if needed */
                if !(uniq as libc::c_int != 0 &&
                         (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint
                             == 0) {
                    /* Collect "appropriate" monsters */
                    if all as libc::c_int != 0 ||
                           (*r_ptr).d_char as libc::c_int ==
                               sym as libc::c_int {
                        let fresh2 = n;
                        n = n + 1;
                        *who.offset(fresh2 as isize) = i as u16b
                    }
                }
            }
        }
        i += 1
    }
    /* Nothing to recall */
    if n == 0 {
        /* Free the "who" array */
        who =
            rnfree(who as vptr,
                   (max_r_idx as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<u16b>()
                                                        as libc::c_ulong)) as
                *mut u16b;
        /* Restore the "cheat_know" flag */
        cheat_know = oldcheat;
        return 1 as libc::c_int as bool_
    }
    /* Sort by level */
    why = 2 as libc::c_int as u16b;
    query = 'y' as i32 as libc::c_char;
    /* Sort if needed */
    if why != 0 {
        /* Select the sort method */
        ang_sort_comp =
            Some(ang_sort_comp_hook as
                     unsafe extern "C" fn(_: vptr, _: vptr, _: libc::c_int,
                                          _: libc::c_int) -> bool_);
        ang_sort_swap =
            Some(ang_sort_swap_hook as
                     unsafe extern "C" fn(_: vptr, _: vptr, _: libc::c_int,
                                          _: libc::c_int) -> ());
        /* Sort the array */
        ang_sort(who as vptr, &mut why as *mut u16b as vptr, n);
    }
    /* Start at the end */
    i = n - 1 as libc::c_int;
    notpicked = 1 as libc::c_int as bool_;
    /* Scan the monster memory */
    while notpicked != 0 {
        /* Extract a race */
        r_idx = *who.offset(i as isize) as libc::c_int;
        /* Hack -- Auto-recall */
        monster_race_track(r_idx, 0 as libc::c_int);
        /* Hack -- Handle stuff */
        handle_stuff();
        /* Hack -- Begin the prompt */
        roff_top(r_idx);
        /* Hack -- Complete the prompt */
        Term_addstr(-(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                    b" [(r)ecall, ESC, space to continue]\x00" as *const u8 as
                        *const libc::c_char);
        /* Interact */
        loop 
             /* Recall */
             {
            if recall != 0 {
                /* Save the screen */
                character_icky = 1 as libc::c_int as bool_;
                Term_save();
                /* Recall on screen */
                r2_ptr =
                    &mut *r_info.offset(r_idx as isize) as *mut monster_race;
                oldkills = (*r2_ptr).r_tkills;
                oldwake = (*r2_ptr).r_wake;
                screen_roff(*who.offset(i as isize) as libc::c_int,
                            0 as libc::c_int, 1 as libc::c_int);
                (*r2_ptr).r_tkills = oldkills;
                (*r2_ptr).r_wake = oldwake;
                (*r2_ptr).r_sights = 1 as libc::c_int as s16b;
                cheat_know = oldcheat;
                notpicked = 0 as libc::c_int as bool_;
                break ;
            } else {
                /* Command */
                query = inkey();
                /* Unrecall */
                if recall != 0 {
                    /* Restore */
                    Term_load();
                    character_icky = 0 as libc::c_int as bool_
                }
                /* Normal commands */
                if query as libc::c_int != 'r' as i32 { break ; }
                /* Toggle recall */
                recall = (recall == 0) as libc::c_int as bool_
            }
        }
        /* Stop scanning */
        if query as libc::c_int == '\u{1b}' as i32 { break ; }
        /* Move to "prev" monster */
        if query as libc::c_int == '-' as i32 {
            i += 1;
            if !(i == n) { continue ; }
            i = 0 as libc::c_int;
            if expand_list == 0 { break ; }
        } else {
            /* Move to "next" monster */
            let fresh3 = i;
            i = i - 1;
            if !(fresh3 == 0 as libc::c_int) { continue ; }
            i = n - 1 as libc::c_int;
            if expand_list == 0 { break ; }
        }
    }
    /* Re-display the identity */
	/* prt(buf, 5, 5);*/
    /* Free the "who" array */
    who =
        rnfree(who as vptr,
               (max_r_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<u16b>()
                                                    as libc::c_ulong)) as
            *mut u16b;
    /* Restore the "cheat_know" flag */
    cheat_know = oldcheat;
    return notpicked;
}
/*
 * Try to "sense" the grid's mana
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_sense_grid_mana() -> bool_ {
    let mut chance: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    /* Take (a lot of) time */
    energy_use = 200 as libc::c_int;
    /* Base chance of success */
    chance = (*p_ptr).skill_dev as libc::c_int;
    /* Confusion hurts skill */
    if (*p_ptr).confused != 0 { chance = chance / 2 as libc::c_int }
    /* Hight mana grids are harder */
    chance =
        chance -
            (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).mana as
                libc::c_int / 10 as libc::c_int;
    /* Give everyone a (slight) chance */
    if chance < 3 as libc::c_int &&
           Rand_div(3 as libc::c_int - chance + 1 as libc::c_int) ==
               0 as libc::c_int {
        chance = 3 as libc::c_int
    }
    /* Roll for usage */
    if chance < 3 as libc::c_int ||
           (Rand_div(chance) + 1 as libc::c_int) < 3 as libc::c_int {
        if flush_failure != 0 { flush(); }
        msg_print(b"You failed to sense the grid\'s mana.\x00" as *const u8 as
                      *const libc::c_char);
        sound(55 as libc::c_int);
        return 0 as libc::c_int as bool_
    }
    /* Try to give an "average" value */
    i =
        (101 as libc::c_int - (*p_ptr).skill_dev as libc::c_int) /
            2 as libc::c_int;
    i =
        if i < 1 as libc::c_int {
            1 as libc::c_int
        } else if i > 50 as libc::c_int { 50 as libc::c_int } else { i };
    if wizard != 0 {
        msg_format(b"Grid\'s mana: %d.\x00" as *const u8 as
                       *const libc::c_char,
                   (*cave[(*p_ptr).py as
                              usize].offset((*p_ptr).px as isize)).mana as
                       libc::c_int);
        msg_format(b"Average grid\'s mana: %d.\x00" as *const u8 as
                       *const libc::c_char,
                   (*cave[(*p_ptr).py as
                              usize].offset((*p_ptr).px as isize)).mana as
                       libc::c_int / i * i);
    } else {
        msg_format(b"Average Area\'s mana: %d\x00" as *const u8 as
                       *const libc::c_char,
                   (*cave[(*p_ptr).py as
                              usize].offset((*p_ptr).px as isize)).mana as
                       libc::c_int / i * i);
    }
    return 1 as libc::c_int as bool_;
}
/*
 * Calculate the weight of the portable holes
 */
#[no_mangle]
pub unsafe extern "C" fn portable_hole_weight() -> s32b {
    let mut weight: s32b = 0;
    let mut i: s32b = 0;
    let mut st_ptr: *mut store_type =
        &mut *(*town_info.offset(20 as libc::c_int as
                                     isize)).store.offset(7 as libc::c_int as
                                                              isize) as
            *mut store_type;
    /* Sum the objects in the appropriate home */
    i = 0 as libc::c_int;
    weight = 0 as libc::c_int;
    while i < (*st_ptr).stock_num as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *(*st_ptr).stock.offset(i as isize) as *mut object_type;
        weight += (*o_ptr).weight * (*o_ptr).number as libc::c_int;
        i += 1
    }
    /* Multiply the sum with 1.5 */
    weight = weight * 3 as libc::c_int / 2 as libc::c_int + 2 as libc::c_int;
    return weight;
}
/*
 * Calculate and set the weight of the portable holes
 */
#[no_mangle]
pub unsafe extern "C" fn set_portable_hole_weight() {
    let mut weight: s32b = 0;
    let mut i: s32b = 0;
    let mut j: s32b = 0;
    /* Calculate the weight of items in home */
    weight = portable_hole_weight();
    /* Set the weight of portable holes in the shops, ... */
    i = 1 as libc::c_int;
    while i < max_towns as libc::c_int {
        j = 0 as libc::c_int;
        while j < max_st_idx as libc::c_int {
            let mut st_ptr: *mut store_type =
                &mut *(*town_info.offset(i as isize)).store.offset(j as isize)
                    as *mut store_type;
            let mut k: libc::c_int = 0;
            k = 0 as libc::c_int;
            while k < (*st_ptr).stock_num as libc::c_int {
                let mut o_ptr: *mut object_type =
                    &mut *(*st_ptr).stock.offset(k as isize) as
                        *mut object_type;
                if (*o_ptr).tval as libc::c_int == 12 as libc::c_int &&
                       (*o_ptr).sval as libc::c_int == 1 as libc::c_int {
                    (*o_ptr).weight = weight
                }
                k += 1
            }
            j += 1
        }
        i += 1
    }
    /* ... in the object list, ... */
    i = 1 as libc::c_int;
    while i < o_max as libc::c_int {
        let mut o_ptr_0: *mut object_type =
            &mut *o_list.offset(i as isize) as *mut object_type;
        if (*o_ptr_0).tval as libc::c_int == 12 as libc::c_int &&
               (*o_ptr_0).sval as libc::c_int == 1 as libc::c_int {
            (*o_ptr_0).weight = weight
        }
        i += 1
    }
    /* ... and in the p_ptr->inventory to the appropriate value */
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int {
        let mut o_ptr_1: *mut object_type =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        /* Skip non-objects */
        if (*o_ptr_1).tval as libc::c_int == 12 as libc::c_int &&
               (*o_ptr_1).sval as libc::c_int == 1 as libc::c_int {
            (*o_ptr_1).weight = weight
        }
        i += 1
    };
}
/*
 * Use a portable hole
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_portable_hole() {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset((*p_ptr).py as
                                             isize)).offset((*p_ptr).px as
                                                                isize) as
            *mut cave_type;
    let mut feat: libc::c_int = 0;
    let mut special: libc::c_int = 0;
    let mut town_num: libc::c_int = 0;
    /* Is it currently wielded? */
    if (*p_ptr).inventory[51 as libc::c_int as usize].k_idx == 0 ||
           (*p_ptr).inventory[51 as libc::c_int as usize].tval as libc::c_int
               != 12 as libc::c_int ||
           (*p_ptr).inventory[51 as libc::c_int as usize].sval as libc::c_int
               != 1 as libc::c_int {
        /* No, it isn't */
        msg_print(b"You have to wield a portable hole to use your abilities\x00"
                      as *const u8 as *const libc::c_char);
        return
    }
    /* Mega-hack: Saving the old values, and then... */
    feat = (*c_ptr).feat as libc::c_int;
    special = (*c_ptr).special as libc::c_int;
    town_num = (*p_ptr).town_num as libc::c_int;
    /* ... change the current grid to the home in town #1 */
	/* DG -- use the first random town, since random towns cannot have houses */
	/*
	 * pelpel -- This doesn't affect LoS, so we can manipulate
	 * terrain feature without calling cave_set_feat()
	 */
    (*c_ptr).feat = 0x4a as libc::c_int as byte_hack;
    (*c_ptr).special = 7 as libc::c_int as s16b;
    (*p_ptr).town_num = 20 as libc::c_int as s16b;
    /* Now use the portable hole */
    do_cmd_store();
    /* Mega-hack part II: change the current grid to the original value */
    (*c_ptr).feat = feat as byte_hack;
    (*c_ptr).special = special as s16b;
    (*p_ptr).town_num = town_num as s16b;
    set_portable_hole_weight();
    /* Recalculate bonuses */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long |
                  0x8 as libc::c_long)) as u32b;
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
}
/*
 * Try to add a CLI action.
 */
#[no_mangle]
pub unsafe extern "C" fn cli_add(mut active: cptr, mut trigger: cptr,
                                 mut descr: cptr) {
    let mut num: s16b = 0;
    let mut cli_ptr: *mut cli_comm = 0 as *mut cli_comm;
    let mut old_ptr: *mut cli_comm = 0 as *mut cli_comm;
    /* Too many macros. */
    if cli_total >= 128 as libc::c_int { return }
    /* First try to read active as a number. */
    if strtol(active, 0 as *mut *mut libc::c_char, 0 as libc::c_int) != 0 {
        num =
            strtol(active, 0 as *mut *mut libc::c_char, 0 as libc::c_int) as
                s16b
    } else if strlen(active) == 1 as libc::c_int as libc::c_ulong {
        num = *active.offset(0 as libc::c_int as isize) as s16b
    } else {
        /* Then try to read it as a character. */
        /* Give up if it doesn't work. */
        return
    }
    /* Dump the macro. */
    cli_ptr = cli_info.offset(cli_total as isize);
    old_ptr =
        cli_info.offset(cli_total as
                            isize).offset(-(1 as libc::c_int as isize));
    /*
	 * Trim 's from the ends of a token. This turns '@' into @ and
	 * ''' into '. This may be the intent of the code in tokenize(),
	 * but I've left it for lack of comments to back me up.
	 */
    if !strchr(trigger, '\'' as i32).is_null() {
        let mut temp: [libc::c_char; 80] = [0; 80];
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut s: cptr = 0 as *const libc::c_char;
        s = trigger;
        t = temp.as_mut_ptr();
        loop  {
            /* tokenize() causes each ' to be followed by another character,
			 * and then another '. Trim the 's here. */
            if *s as libc::c_int == '\'' as i32 {
                s = s.offset(1);
                *t = *s;
                s = s.offset(1)
            } else { *t = *s }
            if *t as libc::c_int == '\u{0}' as i32 { break ; }
            s = s.offset(1);
            t = t.offset(1)
        }
        (*cli_ptr).comm = string_make(temp.as_mut_ptr() as cptr)
    } else { (*cli_ptr).comm = string_make(trigger) }
    /* First try copying everything across. */
    (*cli_ptr).key = num;
    (*cli_ptr).descrip = string_make(descr);
    /* Take description for the previous record if appropriate. */
    if cli_total > 0 as libc::c_int &&
           (*old_ptr).key as libc::c_int == (*cli_ptr).key as libc::c_int &&
           (*cli_ptr).descrip.is_null() {
        (*cli_ptr).descrip = (*old_ptr).descrip
    }
    /* Accept the macro. */
    if (*cli_ptr).key as libc::c_int != 0 && !(*cli_ptr).comm.is_null() &&
           !(*cli_ptr).descrip.is_null() {
        cli_total += 1
    };
}
/*
 * Get a string using CLI completion.
 */
#[no_mangle]
pub unsafe extern "C" fn get_string_cli(mut prompt: cptr,
                                        mut buf: *mut libc::c_char,
                                        mut len: libc::c_int) -> bool_ {
    let mut res: bool_ = 0;
    /* Paranoia XXX XXX XXX */
    msg_print(0 as cptr);
    /* Display prompt */
    prt(prompt, 0 as libc::c_int, 0 as libc::c_int);
    /* Ask the user for a string */
    askfor_aux_complete = 1 as libc::c_int as bool_;
    res = askfor_aux(buf, len);
    askfor_aux_complete = 0 as libc::c_int as bool_;
    /* Clear prompt */
    prt(b"\x00" as *const u8 as *const libc::c_char, 0 as libc::c_int,
        0 as libc::c_int);
    /* Result */
    return res;
}
/*
 * Do a command line command
 *
 * This is a wrapper around process command to provide a "reverse keymap"
 * whereby a set of keypresses is mapped to one.
 *
 * This is useful because command_cmd is a s16b, and so allows each command a
 * unique representation.
 *
 * See defines.h for a list of the codes used.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_cli() {
    let mut buff: [libc::c_char; 80] = [0; 80];
    let mut cli_ptr: *mut cli_comm = 0 as *mut cli_comm;
    /* Clear the input buffer */
    strcpy(buff.as_mut_ptr(), b"\x00" as *const u8 as *const libc::c_char);
    /* Accept command */
    if get_string_cli(b"Command: \x00" as *const u8 as *const libc::c_char,
                      buff.as_mut_ptr(), 30 as libc::c_int) == 0 {
        return
    }
    /* Analyse the input */
    cli_ptr = cli_info;
    while !(*cli_ptr).comm.is_null() {
        if strcmp(buff.as_mut_ptr(), (*cli_ptr).comm) == 0 {
            /* Process the command without keymaps or macros. */
            command_new = (*cli_ptr).key;
            return
        }
        cli_ptr = cli_ptr.offset(1)
    }
    msg_format(b"No such command: %s\x00" as *const u8 as *const libc::c_char,
               buff.as_mut_ptr());
}
/*
 * Display on-line help for the CLI commands
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_cli_help() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut file_name: [libc::c_char; 1024] = [0; 1024];
    /* Temporary file */
    if path_temp(file_name.as_mut_ptr(), 1024 as libc::c_int) != 0 { return }
    /* Open a new file */
    fff =
        my_fopen(file_name.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    j = -(1 as libc::c_int);
    while i < cli_total {
        if j < i - 1 as libc::c_int {
            fprintf(fff, b"/\x00" as *const u8 as *const libc::c_char);
        }
        fprintf(fff, b"[[[[[G%s]\x00" as *const u8 as *const libc::c_char,
                (*cli_info.offset(i as isize)).comm);
        if (*cli_info.offset(i as isize)).descrip !=
               (*cli_info.offset((i + 1 as libc::c_int) as isize)).descrip {
            fprintf(fff, b"   %s\n\x00" as *const u8 as *const libc::c_char,
                    (*cli_info.offset(i as isize)).descrip);
            j = i
        }
        i += 1
    }
    /* Close the file */
    my_fclose(fff);
    /* Enter "icky" mode */
    character_icky = 1 as libc::c_int as bool_;
    /* Save the screen */
    Term_save();
    /* Display the file contents */
    show_file(file_name.as_mut_ptr() as cptr,
              b"Command line help\x00" as *const u8 as *const libc::c_char,
              0 as libc::c_int, 0 as libc::c_int);
    /* Restore the screen */
    Term_load();
    /* Leave "icky" mode */
    character_icky = 0 as libc::c_int as bool_;
    /* Remove the file */
    fd_kill(file_name.as_mut_ptr() as cptr);
}
/*
 * Dump screen shot in HTML
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_html_dump() {
    let mut tmp_val: [libc::c_char; 81] = [0; 81];
    let mut html: bool_ = 1 as libc::c_int as bool_;
    let mut save: *mut term_win = 0 as *mut term_win;
    /* Save the screen */
    save = Term_save_to();
    if wizard as libc::c_int != 0 &&
           get_check(b"WIZARD MODE: Do an help file dump?\x00" as *const u8 as
                         *const libc::c_char) as libc::c_int != 0 {
        html = 0 as libc::c_int as bool_
    }
    /* Ask for a file */
    if html != 0 {
        strcpy(tmp_val.as_mut_ptr(),
               b"dummy.htm\x00" as *const u8 as *const libc::c_char);
        if get_string(b"File(you can post it to http://angband.oook.cz/): \x00"
                          as *const u8 as *const libc::c_char,
                      tmp_val.as_mut_ptr(), 80 as libc::c_int) == 0 {
            /* Now restore the screen to initial state */
            Term_load_from(save, 1 as libc::c_int as bool_);
            Term_fresh();
            return
        }
    } else {
        strcpy(tmp_val.as_mut_ptr(),
               b"dummy.txt\x00" as *const u8 as *const libc::c_char);
        if get_string(b"File: \x00" as *const u8 as *const libc::c_char,
                      tmp_val.as_mut_ptr(), 80 as libc::c_int) == 0 {
            /* Now restore the screen to initial state */
            Term_load_from(save, 1 as libc::c_int as bool_);
            Term_fresh();
            return
        }
    }
    /* Now restore the screen to dump it */
    Term_load_from(save, 1 as libc::c_int as bool_);
    if html != 0 {
        html_screenshot(tmp_val.as_mut_ptr() as cptr);
    } else { help_file_screenshot(tmp_val.as_mut_ptr() as cptr); }
    Term_erase(0 as libc::c_int, 0 as libc::c_int, 255 as libc::c_int);
    msg_print(b"Dump saved.\x00" as *const u8 as *const libc::c_char);
    Term_fresh();
    fix_message();
}
