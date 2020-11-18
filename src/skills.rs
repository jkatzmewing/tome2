use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut command_arg: s16b;
    #[no_mangle]
    static mut energy_use: s32b;
    #[no_mangle]
    static mut wizard: bool_;
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
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn streq(s: cptr, t: cptr) -> bool_;
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
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn Term_clear() -> errr;
    #[no_mangle]
    fn Term_get_size(w: *mut libc::c_int, h: *mut libc::c_int) -> errr;
    #[no_mangle]
    fn Term_save() -> errr;
    #[no_mangle]
    fn Term_load() -> errr;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut rp_ptr: *mut player_race;
    #[no_mangle]
    static mut rmp_ptr: *mut player_race_mod;
    #[no_mangle]
    static mut cp_ptr: *mut player_class;
    #[no_mangle]
    static mut spp_ptr: *mut player_spec;
    #[no_mangle]
    static mut ab_info: *mut ability_type;
    #[no_mangle]
    static mut ab_name: *mut libc::c_char;
    #[no_mangle]
    static mut ab_text: *mut libc::c_char;
    #[no_mangle]
    static mut s_info: *mut skill_type;
    #[no_mangle]
    static mut s_name: *mut libc::c_char;
    #[no_mangle]
    static mut s_text: *mut libc::c_char;
    #[no_mangle]
    static mut max_ab_idx: u16b;
    #[no_mangle]
    static mut max_s_idx: u16b;
    #[no_mangle]
    static mut spell_num: s16b;
    #[no_mangle]
    static mut gen_skill_basem: [libc::c_char; 200];
    #[no_mangle]
    static mut gen_skill_base: [u32b; 200];
    #[no_mangle]
    static mut gen_skill_modm: [libc::c_char; 200];
    #[no_mangle]
    static mut gen_skill_mod: [s16b; 200];
    #[no_mangle]
    static mut game_module: cptr;
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn print_desc_aux(txt: cptr, y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn do_cmd_steal();
    #[no_mangle]
    fn do_cmd_portable_hole();
    #[no_mangle]
    fn do_cmd_copy_spell();
    #[no_mangle]
    fn cast_school_spell();
    #[no_mangle]
    fn do_cmd_create_boulder();
    #[no_mangle]
    fn do_cmd_summoner();
    #[no_mangle]
    fn do_cmd_mindcraft();
    #[no_mangle]
    fn do_cmd_mimic();
    #[no_mangle]
    fn do_cmd_blade();
    #[no_mangle]
    fn use_ability_blade();
    #[no_mangle]
    fn do_cmd_alchemist();
    #[no_mangle]
    fn do_cmd_powermage();
    #[no_mangle]
    fn do_cmd_possessor();
    #[no_mangle]
    fn do_cmd_archer();
    #[no_mangle]
    fn do_cmd_set_piercing();
    #[no_mangle]
    fn do_cmd_necromancer();
    #[no_mangle]
    fn do_cmd_unbeliever();
    #[no_mangle]
    fn do_cmd_runecrafter();
    #[no_mangle]
    fn do_cmd_symbiotic();
    #[no_mangle]
    fn modify_aux(a: s32b, b: s32b, mod_0: libc::c_char) -> s32b;
    #[no_mangle]
    fn do_cmd_companion();
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn inven_takeoff(item: libc::c_int, amt: libc::c_int, force_drop: bool_)
     -> s16b;
    #[no_mangle]
    fn do_cmd_set_trap();
    #[no_mangle]
    fn generate_spell(plev: libc::c_int);
    #[no_mangle]
    fn ask_menu(ask: cptr, items: *mut *mut libc::c_char, max: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn flush();
    #[no_mangle]
    fn bell();
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn display_message(x: libc::c_int, y: libc::c_int, split: libc::c_int,
                       color: byte_hack, t: cptr);
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
    fn c_prt(attr: byte_hack, str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn get_string(prompt: cptr, buf: *mut libc::c_char, len: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn get_check(prompt: cptr) -> bool_;
    #[no_mangle]
    fn get_keymap_dir(ch: libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn msg_box(text: cptr, y: libc::c_int, x: libc::c_int) -> libc::c_char;
    #[no_mangle]
    fn set_disrupt_shield(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_invuln(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn repeat_push(what: libc::c_int);
    #[no_mangle]
    fn repeat_pull(what: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn exec_lua(file: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn call_lua(function: cptr, args: cptr, ret: cptr, _: ...) -> bool_;
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
pub type s32b = libc::c_int;
pub type u32b = libc::c_uint;
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
/* File: skills.c */
/* Purpose: player skills */
/*
 * Copyright (c) 2001 DarkGod
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Advance the skill point of the skill specified by i and
 * modify related skills
 */
#[no_mangle]
pub unsafe extern "C" fn increase_skill(mut i: libc::c_int,
                                        mut invest: *mut s16b) {
    let mut max_skill_overage: s32b = 0;
    /* No skill points to be allocated */
    if (*p_ptr).skill_points == 0 { return }
    /* The skill cannot be increased */
    if (*s_info.offset(i as isize)).mod_0 == 0 { return }
    /* The skill is already maxed */
    if (*s_info.offset(i as isize)).value >= 50000 as libc::c_int { return }
    /* Cannot allocate more than player level + max_skill_overage levels */
    call_lua(b"get_module_info\x00" as *const u8 as *const libc::c_char,
             b"(s)\x00" as *const u8 as *const libc::c_char,
             b"d\x00" as *const u8 as *const libc::c_char,
             b"max_skill_overage\x00" as *const u8 as *const libc::c_char,
             &mut max_skill_overage as *mut s32b);
    if ((*s_info.offset(i as isize)).value +
            (*s_info.offset(i as isize)).mod_0) / 1000 as libc::c_int >=
           (*p_ptr).lev as libc::c_int + max_skill_overage + 1 as libc::c_int
       {
        let mut hgt: libc::c_int = 0;
        let mut wid: libc::c_int = 0;
        Term_get_size(&mut wid, &mut hgt);
        msg_box(format(b"Cannot raise a skill value above %i + player level.\x00"
                           as *const u8 as *const libc::c_char,
                       max_skill_overage) as cptr, hgt / 2 as libc::c_int,
                wid / 2 as libc::c_int);
        return
    }
    /* Spend an unallocated skill point */
    (*p_ptr).skill_points -= 1;
    /* Increase the skill */
    let ref mut fresh0 = (*s_info.offset(i as isize)).value;
    *fresh0 += (*s_info.offset(i as isize)).mod_0;
    let ref mut fresh1 = *invest.offset(i as isize);
    *fresh1 += 1;
}
/*
 * Descrease the skill point of the skill specified by i and
 * modify related skills
 */
#[no_mangle]
pub unsafe extern "C" fn decrease_skill(mut i: libc::c_int,
                                        mut invest: *mut s16b) {
    /* Cannot decrease more */
    if *invest.offset(i as isize) == 0 { return }
    /* The skill cannot be decreased */
    if (*s_info.offset(i as isize)).mod_0 == 0 { return }
    /* The skill already has minimal value */
    if (*s_info.offset(i as isize)).value == 0 { return }
    /* Free a skill point */
    (*p_ptr).skill_points += 1;
    /* Decrease the skill */
    let ref mut fresh2 = (*s_info.offset(i as isize)).value;
    *fresh2 -= (*s_info.offset(i as isize)).mod_0;
    let ref mut fresh3 = *invest.offset(i as isize);
    *fresh3 -= 1;
}
/*
 * Given the name of a skill, returns skill index or -1 if no
 * such skill is found
 */
#[no_mangle]
pub unsafe extern "C" fn find_skill(mut name: cptr) -> s16b {
    let mut i: u16b = 0;
    /* Scan skill list */
    i = 1 as libc::c_int as u16b;
    while (i as libc::c_int) < max_s_idx as libc::c_int {
        /* The name matches */
        if streq(s_name.offset((*s_info.offset(i as isize)).name as isize) as
                     cptr, name) != 0 {
            return i as s16b
        }
        i = i.wrapping_add(1)
    }
    /* No match found */
    return -(1 as libc::c_int) as s16b;
}
#[no_mangle]
pub unsafe extern "C" fn find_skill_i(mut name: cptr) -> s16b {
    let mut i: u16b = 0;
    /* Scan skill list */
    i = 1 as libc::c_int as u16b;
    while (i as libc::c_int) < max_s_idx as libc::c_int {
        /* The name matches */
        if 0 as libc::c_int ==
               strcasecmp(s_name.offset((*s_info.offset(i as isize)).name as
                                            isize), name) {
            return i as s16b
        }
        i = i.wrapping_add(1)
    }
    /* No match found */
    return -(1 as libc::c_int) as s16b;
}
/*
 *
 */
#[no_mangle]
pub unsafe extern "C" fn get_skill(mut skill: libc::c_int) -> s16b {
    return ((*s_info.offset(skill as isize)).value / 1000 as libc::c_int) as
               s16b;
}
/*
 * Return "scale" (a misnomer -- this is max value) * (current skill value)
 * / (max skill value)
 */
#[no_mangle]
pub unsafe extern "C" fn get_skill_scale(mut skill: libc::c_int,
                                         mut scale: u32b) -> s16b {
    let mut temp: s32b = 0;
    /*
	* SKILL_STEP shouldn't matter here because the second parameter is
	* relatively small (the largest one being somewhere around 200),
	* AND because we could have used much simpler 0--50 if the ability
	* progression were only possible at step boundaries.
	*
	* Because I'm not at all certain about my interpretation of the mysterious
	* formula given above, I verified this works the same by using a tiny
	* scheme program... -- pelpel
	*/
    temp =
        scale.wrapping_mul((*s_info.offset(skill as isize)).value as
                               libc::c_uint) as s32b;
    return (temp / 50000 as libc::c_int) as s16b;
}
/*
 *
 */
#[no_mangle]
pub unsafe extern "C" fn get_idx(mut i: libc::c_int) -> libc::c_int {
    let mut j: libc::c_int = 0;
    j = 1 as libc::c_int;
    while j < max_s_idx as libc::c_int {
        if (*s_info.offset(j as isize)).order as libc::c_int == i { return j }
        j += 1
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn is_known(mut s_idx: libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0;
    if wizard != 0 { return 1 as libc::c_int as bool_ }
    if (*s_info.offset(s_idx as isize)).value != 0 ||
           (*s_info.offset(s_idx as isize)).mod_0 != 0 {
        return 1 as libc::c_int as bool_
    }
    i = 0 as libc::c_int;
    while i < max_s_idx as libc::c_int {
        /* It is our child, if we don't know it we continue to search, if we know it it is enough*/
        if (*s_info.offset(i as isize)).father as libc::c_int == s_idx {
            if is_known(i) != 0 { return 1 as libc::c_int as bool_ }
        }
        i += 1
    }
    /* Ok know none */
    return 0 as libc::c_int as bool_;
}
/*
 *
 */
#[no_mangle]
pub unsafe extern "C" fn init_table_aux(mut table: *mut [libc::c_int; 2],
                                        mut idx: *mut libc::c_int,
                                        mut father: libc::c_int,
                                        mut lev: libc::c_int,
                                        mut full: bool_) {
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    j = 1 as libc::c_int;
    while j < max_s_idx as libc::c_int {
        i = get_idx(j);
        if !((*s_info.offset(i as isize)).father as libc::c_int != father) {
            if !((*s_info.offset(i as isize)).hidden != 0) {
                if !(is_known(i) == 0) {
                    (*table.offset(*idx as isize))[0 as libc::c_int as usize]
                        = i;
                    (*table.offset(*idx as isize))[1 as libc::c_int as usize]
                        = lev;
                    *idx += 1;
                    if (*s_info.offset(i as isize)).dev as libc::c_int != 0 ||
                           full as libc::c_int != 0 {
                        init_table_aux(table, idx, i, lev + 1 as libc::c_int,
                                       full);
                    }
                }
            }
        }
        j += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn init_table(mut table: *mut [libc::c_int; 2],
                                    mut max: *mut libc::c_int,
                                    mut full: bool_) {
    *max = 0 as libc::c_int;
    init_table_aux(table, max, -(1 as libc::c_int), 0 as libc::c_int, full);
}
#[no_mangle]
pub unsafe extern "C" fn has_child(mut sel: libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < max_s_idx as libc::c_int {
        if (*s_info.offset(i as isize)).father as libc::c_int == sel &&
               is_known(i) as libc::c_int != 0 {
            return 1 as libc::c_int as bool_
        }
        i += 1
    }
    return 0 as libc::c_int as bool_;
}
/*
 * Dump the skill tree
 */
#[no_mangle]
pub unsafe extern "C" fn dump_skills(mut fff: *mut FILE) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut max: libc::c_int = 0 as libc::c_int;
    let mut table: [[libc::c_int; 2]; 200] = [[0; 2]; 200];
    let mut buf: [libc::c_char; 80] = [0; 80];
    init_table(table.as_mut_ptr(), &mut max, 1 as libc::c_int as bool_);
    fprintf(fff,
            b"\nSkills (points left: %d)\x00" as *const u8 as
                *const libc::c_char, (*p_ptr).skill_points as libc::c_int);
    let mut current_block_13: u64;
    j = 0 as libc::c_int;
    while j < max {
        let mut z: libc::c_int = 0;
        i = table[j as usize][0 as libc::c_int as usize];
        if (*s_info.offset(i as isize)).value == 0 as libc::c_int &&
               i != 30 as libc::c_int {
            if (*s_info.offset(i as isize)).mod_0 == 0 as libc::c_int {
                current_block_13 = 17179679302217393232;
            } else { current_block_13 = 13183875560443969876; }
        } else { current_block_13 = 13183875560443969876; }
        match current_block_13 {
            13183875560443969876 => {
                sprintf(buf.as_mut_ptr(),
                        b"\n\x00" as *const u8 as *const libc::c_char);
                z = 0 as libc::c_int;
                while z < table[j as usize][1 as libc::c_int as usize] {
                    strcat(buf.as_mut_ptr(),
                           b"         \x00" as *const u8 as
                               *const libc::c_char);
                    z += 1
                }
                if has_child(i) == 0 {
                    strcat(buf.as_mut_ptr(),
                           format(b" . %s\x00" as *const u8 as
                                      *const libc::c_char,
                                  s_name.offset((*s_info.offset(i as
                                                                    isize)).name
                                                    as isize)));
                } else {
                    strcat(buf.as_mut_ptr(),
                           format(b" - %s\x00" as *const u8 as
                                      *const libc::c_char,
                                  s_name.offset((*s_info.offset(i as
                                                                    isize)).name
                                                    as isize)));
                }
                fprintf(fff,
                        b"%-49s%s%06.3f [%05.3f]\x00" as *const u8 as
                            *const libc::c_char, buf.as_mut_ptr(),
                        if (*s_info.offset(i as isize)).value <
                               0 as libc::c_int {
                            b"-\x00" as *const u8 as *const libc::c_char
                        } else {
                            b" \x00" as *const u8 as *const libc::c_char
                        },
                        (if (*s_info.offset(i as isize)).value <
                                0 as libc::c_int {
                             -(*s_info.offset(i as isize)).value
                         } else { (*s_info.offset(i as isize)).value }) as
                            libc::c_double /
                            1000 as libc::c_int as libc::c_double,
                        (*s_info.offset(i as isize)).mod_0 as libc::c_double /
                            1000 as libc::c_int as libc::c_double);
            }
            _ => { }
        }
        j += 1
    }
    fprintf(fff, b"\n\x00" as *const u8 as *const libc::c_char);
}
/*
 * Draw the skill tree
 */
#[no_mangle]
pub unsafe extern "C" fn print_skills(mut table: *mut [libc::c_int; 2],
                                      mut max: libc::c_int,
                                      mut sel: libc::c_int,
                                      mut start: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    let mut keys: cptr = 0 as *const libc::c_char;
    Term_clear();
    Term_get_size(&mut wid, &mut hgt);
    c_prt(1 as libc::c_int as byte_hack,
          format(b"%s Skills Screen\x00" as *const u8 as *const libc::c_char,
                 game_module) as cptr, 0 as libc::c_int, 28 as libc::c_int);
    keys =
        format(b"#BEnter#W to develop a branch, #Bup#W/#Bdown#W to move, #Bright#W/#Bleft#W to modify, #B?#W for help\x00"
                   as *const u8 as *const libc::c_char) as cptr;
    display_message(0 as libc::c_int, 1 as libc::c_int,
                    strlen(keys) as libc::c_int,
                    1 as libc::c_int as byte_hack, keys);
    c_prt(if (*p_ptr).skill_points as libc::c_int != 0 {
              14 as libc::c_int
          } else { 12 as libc::c_int } as byte_hack,
          format(b"Skill points left: %d\x00" as *const u8 as
                     *const libc::c_char,
                 (*p_ptr).skill_points as libc::c_int) as cptr,
          2 as libc::c_int, 0 as libc::c_int);
    print_desc_aux(s_text.offset((*s_info.offset((*table.offset(sel as
                                                                    isize))[0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                                                     as isize)).desc as isize)
                       as cptr, 3 as libc::c_int, 0 as libc::c_int);
    j = start;
    while j < start + (hgt - 7 as libc::c_int) {
        let mut color: byte_hack = 1 as libc::c_int as byte_hack;
        let mut deb: libc::c_char = ' ' as i32 as libc::c_char;
        let mut end: libc::c_char = ' ' as i32 as libc::c_char;
        if j >= max { break ; }
        i = (*table.offset(j as isize))[0 as libc::c_int as usize];
        if (*s_info.offset(i as isize)).value == 0 as libc::c_int &&
               i != 30 as libc::c_int {
            if (*s_info.offset(i as isize)).mod_0 == 0 as libc::c_int {
                color = 8 as libc::c_int as byte_hack
            } else { color = 3 as libc::c_int as byte_hack }
        } else if (*s_info.offset(i as isize)).value == 50000 as libc::c_int {
            color = 14 as libc::c_int as byte_hack
        }
        if (*s_info.offset(i as isize)).hidden != 0 {
            color = 12 as libc::c_int as byte_hack
        }
        if j == sel {
            color = 13 as libc::c_int as byte_hack;
            deb = '[' as i32 as libc::c_char;
            end = ']' as i32 as libc::c_char
        }
        if has_child(i) == 0 {
            c_prt(color,
                  format(b"%c.%c%s\x00" as *const u8 as *const libc::c_char,
                         deb as libc::c_int, end as libc::c_int,
                         s_name.offset((*s_info.offset(i as isize)).name as
                                           isize)) as cptr,
                  j + 7 as libc::c_int - start,
                  (*table.offset(j as isize))[1 as libc::c_int as usize] *
                      4 as libc::c_int);
        } else if (*s_info.offset(i as isize)).dev != 0 {
            c_prt(color,
                  format(b"%c-%c%s\x00" as *const u8 as *const libc::c_char,
                         deb as libc::c_int, end as libc::c_int,
                         s_name.offset((*s_info.offset(i as isize)).name as
                                           isize)) as cptr,
                  j + 7 as libc::c_int - start,
                  (*table.offset(j as isize))[1 as libc::c_int as usize] *
                      4 as libc::c_int);
        } else {
            c_prt(color,
                  format(b"%c+%c%s\x00" as *const u8 as *const libc::c_char,
                         deb as libc::c_int, end as libc::c_int,
                         s_name.offset((*s_info.offset(i as isize)).name as
                                           isize)) as cptr,
                  j + 7 as libc::c_int - start,
                  (*table.offset(j as isize))[1 as libc::c_int as usize] *
                      4 as libc::c_int);
        }
        c_prt(color,
              format(b"%s%02ld.%03ld [%01d.%03d]\x00" as *const u8 as
                         *const libc::c_char,
                     if (*s_info.offset(i as isize)).value < 0 as libc::c_int
                        {
                         b"-\x00" as *const u8 as *const libc::c_char
                     } else { b" \x00" as *const u8 as *const libc::c_char },
                     (if (*s_info.offset(i as isize)).value < 0 as libc::c_int
                         {
                          -(*s_info.offset(i as isize)).value
                      } else { (*s_info.offset(i as isize)).value }) /
                         1000 as libc::c_int,
                     (if (*s_info.offset(i as isize)).value < 0 as libc::c_int
                         {
                          -(*s_info.offset(i as isize)).value
                      } else { (*s_info.offset(i as isize)).value }) %
                         1000 as libc::c_int,
                     (if (*s_info.offset(i as isize)).mod_0 < 0 as libc::c_int
                         {
                          -(*s_info.offset(i as isize)).mod_0
                      } else { (*s_info.offset(i as isize)).mod_0 }) /
                         1000 as libc::c_int,
                     (if (*s_info.offset(i as isize)).mod_0 < 0 as libc::c_int
                         {
                          -(*s_info.offset(i as isize)).mod_0
                      } else { (*s_info.offset(i as isize)).mod_0 }) %
                         1000 as libc::c_int) as cptr,
              j + 7 as libc::c_int - start, 60 as libc::c_int);
        j += 1
    };
}
/*
 * Checks various stuff to do when skills change, like new spells, ...
 */
#[no_mangle]
pub unsafe extern "C" fn recalc_skills(mut init: bool_) {
    static mut thaum_level: libc::c_int = 0 as libc::c_int;
    /* TODO: This should be a hook in ToME's lua */
    if init != 0 {
        thaum_level =
            get_skill_scale(43 as libc::c_int, 100 as libc::c_int as u32b) as
                libc::c_int
    } else {
        let mut thaum_gain: libc::c_int = 0 as libc::c_int;
        /* Gain thaum spells */
        while thaum_level <
                  get_skill_scale(43 as libc::c_int,
                                  100 as libc::c_int as u32b) as libc::c_int {
            if spell_num as libc::c_int == 100 as libc::c_int { break ; }
            thaum_level += 1;
            generate_spell((thaum_level + 1 as libc::c_int) /
                               2 as libc::c_int);
            thaum_gain += 1
        }
        if thaum_gain != 0 {
            if thaum_gain == 1 as libc::c_int {
                msg_print(b"You have gained one new thaumaturgy spell.\x00" as
                              *const u8 as *const libc::c_char);
            } else {
                msg_format(b"You have gained %d new thaumaturgy spells.\x00"
                               as *const u8 as *const libc::c_char,
                           thaum_gain);
            }
        }
        process_hooks(52 as libc::c_int,
                      b"()\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char);
        /* Update stuffs */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x1 as libc::c_long | 0x10 as libc::c_long |
                      0x20 as libc::c_long | 0x40 as libc::c_long |
                      0x80 as libc::c_long | 0x8 as libc::c_long |
                      0x4 as libc::c_long)) as u32b;
        /* Redraw various info */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long |
                 (0x8000000 as libc::c_long | 0x2000000 as libc::c_long |
                      0x1000000 as libc::c_long | 0x4000000 as libc::c_long))
                as u32b
    };
}
/*
 * Recalc the skill value
 */
#[no_mangle]
pub unsafe extern "C" fn recalc_skills_theory(mut invest: *mut s16b,
                                              mut base_val: *mut s32b,
                                              mut base_mod: *mut s32b,
                                              mut bonus: *mut s32b) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    /* First we assign the normal points */
    i = 0 as libc::c_int;
    while i < max_s_idx as libc::c_int {
        /* Calc the base */
        (*s_info.offset(i as isize)).value =
            *base_val.offset(i as isize) +
                *base_mod.offset(i as isize) *
                    *invest.offset(i as isize) as libc::c_int +
                *bonus.offset(i as isize);
        /* It cannot exceed SKILL_MAX */
        if (*s_info.offset(i as isize)).value > 50000 as libc::c_int {
            (*s_info.offset(i as isize)).value = 50000 as libc::c_int
        }
        i += 1
    }
    /* Then we modify related skills */
    i = 0 as libc::c_int;
    while i < max_s_idx as libc::c_int {
        j = 1 as libc::c_int;
        while j < max_s_idx as libc::c_int {
            /* Ignore self */
            if !(j == i) {
                /* Exclusive skills */
                if (*s_info.offset(i as isize)).action[j as usize] as
                       libc::c_int == 9999 as libc::c_int &&
                       *invest.offset(i as isize) as libc::c_int != 0 {
                    /* Turn it off */
                    (*p_ptr).skill_points =
                        ((*p_ptr).skill_points as libc::c_int +
                             *invest.offset(j as isize) as libc::c_int) as
                            s16b;
                    *invest.offset(j as isize) = 0 as libc::c_int as s16b;
                    (*s_info.offset(j as isize)).value = 0 as libc::c_int
                } else if (*s_info.offset(i as isize)).action[j as usize] != 0
                 {
                    /* Non-exclusive skills */
                    /* Increase / decrease with a % */
                    let mut val: s32b =
                        (*s_info.offset(j as isize)).value +
                            *invest.offset(i as isize) as libc::c_int *
                                (*s_info.offset(j as isize)).mod_0 *
                                (*s_info.offset(i as
                                                    isize)).action[j as usize]
                                    as libc::c_int / 100 as libc::c_int;
                    /* It cannot exceed SKILL_MAX */
                    if val > 50000 as libc::c_int {
                        val = 50000 as libc::c_int
                    }
                    /* Save the modified value */
                    (*s_info.offset(j as isize)).value = val
                }
            }
            j += 1
        }
        i += 1
    };
}
/*
 * Interreact with skills
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_skill() {
    let mut sel: libc::c_int = 0 as libc::c_int;
    let mut start: libc::c_int = 0 as libc::c_int;
    let mut max: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    let mut table: [[libc::c_int; 2]; 200] = [[0; 2]; 200];
    let mut i: libc::c_int = 0;
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    let mut skill_points_save: s16b = 0;
    let mut skill_values_save: *mut s32b = 0 as *mut s32b;
    let mut skill_mods_save: *mut s32b = 0 as *mut s32b;
    let mut skill_rates_save: *mut s16b = 0 as *mut s16b;
    let mut skill_invest: *mut s16b = 0 as *mut s16b;
    let mut skill_bonus: *mut s32b = 0 as *mut s32b;
    recalc_skills(1 as libc::c_int as bool_);
    /* Save the screen */
    screen_save();
    /* Allocate arrays to save skill values */
    skill_values_save =
        memset(ralloc((200 as libc::c_int as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<s32b>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (200 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<s32b>()
                                                    as libc::c_ulong)) as
            *mut s32b;
    skill_mods_save =
        memset(ralloc((200 as libc::c_int as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<s32b>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (200 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<s32b>()
                                                    as libc::c_ulong)) as
            *mut s32b;
    skill_rates_save =
        memset(ralloc((200 as libc::c_int as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<s16b>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (200 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<s16b>()
                                                    as libc::c_ulong)) as
            *mut s16b;
    skill_invest =
        memset(ralloc((200 as libc::c_int as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<s16b>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (200 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<s16b>()
                                                    as libc::c_ulong)) as
            *mut s16b;
    skill_bonus =
        memset(ralloc((200 as libc::c_int as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<s32b>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (200 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<s32b>()
                                                    as libc::c_ulong)) as
            *mut s32b;
    /* Save skill points */
    skill_points_save = (*p_ptr).skill_points;
    /* Save skill values */
    i = 0 as libc::c_int;
    while i < max_s_idx as libc::c_int {
        let mut s_ptr: *mut skill_type =
            &mut *s_info.offset(i as isize) as *mut skill_type;
        *skill_values_save.offset(i as isize) = (*s_ptr).value;
        *skill_mods_save.offset(i as isize) = (*s_ptr).mod_0;
        *skill_rates_save.offset(i as isize) = (*s_ptr).rate;
        *skill_invest.offset(i as isize) = 0 as libc::c_int as s16b;
        i += 1
    }
    /* Clear the screen */
    Term_clear();
    /* Initialise the skill list */
    init_table(table.as_mut_ptr(), &mut max, 0 as libc::c_int as bool_);
    loop  {
        Term_get_size(&mut wid, &mut hgt);
        /* Display list of skills */
        recalc_skills_theory(skill_invest, skill_values_save, skill_mods_save,
                             skill_bonus);
        print_skills(table.as_mut_ptr(), max, sel, start);
        /* Wait for user input */
        c = inkey();
        /* Leave the skill screen */
        if c as libc::c_int == '\u{1b}' as i32 { break ; }
        /* Expand / collapse list of skills */
        if c as libc::c_int == '\r' as i32 {
            if (*s_info.offset(table[sel as usize][0 as libc::c_int as usize]
                                   as isize)).dev != 0 {
                (*s_info.offset(table[sel as usize][0 as libc::c_int as usize]
                                    as isize)).dev = 0 as libc::c_int as bool_
            } else {
                (*s_info.offset(table[sel as usize][0 as libc::c_int as usize]
                                    as isize)).dev = 1 as libc::c_int as bool_
            }
            init_table(table.as_mut_ptr(), &mut max,
                       0 as libc::c_int as bool_);
        } else if c as libc::c_int == 'n' as i32 {
            sel += hgt - 7 as libc::c_int;
            if sel >= max { sel = max - 1 as libc::c_int }
        } else if c as libc::c_int == 'p' as i32 {
            sel -= hgt - 7 as libc::c_int;
            if sel < 0 as libc::c_int { sel = 0 as libc::c_int }
        } else {
            /* Next page */
            /* Previous page */
            /* Select / increase a skill */
            let mut dir: libc::c_int = 0;
            /* Allow use of numpad / arrow keys / roguelike keys */
            dir = get_keymap_dir(c);
            /* Move cursor down */
            if dir == 2 as libc::c_int { sel += 1 }
            /* Move cursor up */
            if dir == 8 as libc::c_int { sel -= 1 }
            /* Miscellaneous skills cannot be increased/decreased as a group */
            if table[sel as usize][0 as libc::c_int as usize] ==
                   30 as libc::c_int {
                continue ;
            }
            /* Increase the current skill */
            if dir == 6 as libc::c_int {
                increase_skill(table[sel as usize][0 as libc::c_int as usize],
                               skill_invest);
            }
            /* Decrease the current skill */
            if dir == 4 as libc::c_int {
                decrease_skill(table[sel as usize][0 as libc::c_int as usize],
                               skill_invest);
            }
            /* XXX XXX XXX Wizard mode commands outside of wizard2.c */
            /* Increase the skill */
            if wizard as libc::c_int != 0 && c as libc::c_int == '+' as i32 {
                let ref mut fresh4 =
                    *skill_bonus.offset(table[sel as
                                                  usize][0 as libc::c_int as
                                                             usize] as isize);
                *fresh4 += 1000 as libc::c_int
            }
            /* Decrease the skill */
            if wizard as libc::c_int != 0 && c as libc::c_int == '-' as i32 {
                let ref mut fresh5 =
                    *skill_bonus.offset(table[sel as
                                                  usize][0 as libc::c_int as
                                                             usize] as isize);
                *fresh5 -= 1000 as libc::c_int
            }
            /* Contextual help */
            if c as libc::c_int == '?' as i32 {
                exec_lua(format(b"ingame_help(\'select_context\', \'skill\', \'%s\')\x00"
                                    as *const u8 as *const libc::c_char,
                                s_name.offset((*s_info.offset(table[sel as
                                                                        usize][0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   usize]
                                                                  as
                                                                  isize)).name
                                                  as isize)));
            }
            /* Handle boundaries and scrolling */
            if sel < 0 as libc::c_int { sel = max - 1 as libc::c_int }
            if sel >= max { sel = 0 as libc::c_int }
            if sel < start { start = sel }
            if sel >= start + (hgt - 7 as libc::c_int) {
                start = sel - (hgt - 7 as libc::c_int) + 1 as libc::c_int
            }
        }
    }
    /* Some skill points are spent */
    if (*p_ptr).skill_points as libc::c_int !=
           skill_points_save as libc::c_int {
        /* Flush input as we ask an important and irreversible question */
        flush();
        /* Ask we can commit the change */
        if msg_box(b"Save and use these skill values? (y/n)\x00" as *const u8
                       as *const libc::c_char, hgt / 2 as libc::c_int,
                   wid / 2 as libc::c_int) as libc::c_int != 'y' as i32 {
            /* User declines -- restore the skill values before exiting */
            /* Restore skill points */
            (*p_ptr).skill_points = skill_points_save;
            /* Restore skill values */
            i = 0 as libc::c_int;
            while i < max_s_idx as libc::c_int {
                let mut s_ptr_0: *mut skill_type =
                    &mut *s_info.offset(i as isize) as *mut skill_type;
                (*s_ptr_0).value = *skill_values_save.offset(i as isize);
                (*s_ptr_0).mod_0 = *skill_mods_save.offset(i as isize);
                (*s_ptr_0).rate = *skill_rates_save.offset(i as isize);
                i += 1
            }
        }
    }
    /* Free arrays to save skill values */
    rnfree(skill_values_save as vptr,
           (200 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<s32b>() as
                                                libc::c_ulong));
    rnfree(skill_mods_save as vptr,
           (200 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<s32b>() as
                                                libc::c_ulong));
    rnfree(skill_rates_save as vptr,
           (200 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<s16b>() as
                                                libc::c_ulong));
    rnfree(skill_invest as vptr,
           (200 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<s16b>() as
                                                libc::c_ulong));
    rnfree(skill_bonus as vptr,
           (200 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<s32b>() as
                                                libc::c_ulong));
    /* Load the screen */
    screen_load();
    recalc_skills(0 as libc::c_int as bool_);
}
/*
 * List of melee skills
 */
#[no_mangle]
pub static mut melee_skills: [s16b; 3] =
    [17 as libc::c_int as s16b, 42 as libc::c_int as s16b,
     47 as libc::c_int as s16b];
#[no_mangle]
pub static mut melee_names: [*mut libc::c_char; 3] =
    [b"Weapon combat\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"Barehanded combat\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"Bearform combat\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char];
static mut melee_bool: [bool_; 3] = [0; 3];
static mut melee_num: [libc::c_int; 3] = [0; 3];
#[no_mangle]
pub unsafe extern "C" fn get_melee_skill() -> s16b {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if (*p_ptr).melee_style as libc::c_int ==
               melee_skills[i as usize] as libc::c_int {
            return i as s16b
        }
        i += 1
    }
    return 0 as libc::c_int as s16b;
}
#[no_mangle]
pub unsafe extern "C" fn get_melee_skills() -> s16b {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if (*s_info.offset(melee_skills[i as usize] as isize)).value >
               0 as libc::c_int &&
               (*s_info.offset(melee_skills[i as usize] as isize)).hidden == 0
           {
            melee_bool[i as usize] = 1 as libc::c_int as bool_;
            j += 1
        } else { melee_bool[i as usize] = 0 as libc::c_int as bool_ }
        i += 1
    }
    return j as s16b;
}
unsafe extern "C" fn choose_melee() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut z: libc::c_int = 0 as libc::c_int;
    let mut force_drop: libc::c_int = 0 as libc::c_int;
    let mut style_unchanged: libc::c_int = 0 as libc::c_int;
    character_icky = 1 as libc::c_int as bool_;
    Term_save();
    Term_clear();
    j = get_melee_skills() as libc::c_int;
    prt(b"Choose a melee style:\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int, 0 as libc::c_int);
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if melee_bool[i as usize] != 0 {
            prt(format(b"%c) %s\x00" as *const u8 as *const libc::c_char,
                       z + 'a' as i32, melee_names[i as usize]) as cptr,
                z + 1 as libc::c_int, 0 as libc::c_int);
            melee_num[z as usize] = i;
            z += 1
        }
        i += 1
    }
    loop  {
        let mut c: libc::c_char = inkey();
        if c as libc::c_int == '\u{1b}' as i32 { break ; }
        if (c as libc::c_int - 'a' as i32) < 0 as libc::c_int { continue ; }
        if c as libc::c_int - 'a' as i32 >= j { continue ; }
        i = 0 as libc::c_int;
        z = 0 as libc::c_int;
        while z < c as libc::c_int - 'a' as i32 {
            if melee_bool[i as usize] != 0 { z += 1 }
            i += 1
        }
        if (*p_ptr).melee_style as libc::c_int ==
               melee_skills[melee_num[z as usize] as usize] as libc::c_int {
            style_unchanged = 1 as libc::c_int;
            break ;
        } else {
            i = 24 as libc::c_int;
            while (*p_ptr).body_parts[(i - 24 as libc::c_int) as usize] as
                      libc::c_int == 24 as libc::c_int {
                if (*p_ptr).inventory[i as usize].k_idx != 0 {
                    if (*p_ptr).inventory[i as usize].ident as libc::c_int &
                           0x40 as libc::c_int != 0 {
                        let mut name: [libc::c_char; 80] = [0; 80];
                        object_desc(name.as_mut_ptr(),
                                    &mut *(*p_ptr).inventory.as_mut_ptr().offset(i
                                                                                     as
                                                                                     isize),
                                    0 as libc::c_int, 0 as libc::c_int);
                        msg_format(b"Hmmm, your %s seems to be cursed.\x00" as
                                       *const u8 as *const libc::c_char,
                                   name.as_mut_ptr());
                        break ;
                    } else if 23 as libc::c_int ==
                                  inven_takeoff(i, 255 as libc::c_int,
                                                force_drop as bool_) as
                                      libc::c_int {
                        force_drop = 1 as libc::c_int
                    }
                }
                i += 1
            }
            (*p_ptr).melee_style =
                melee_skills[melee_num[z as usize] as usize];
            energy_use = 100 as libc::c_int;
            break ;
        }
    }
    /* Recalculate bonuses */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    /* Recalculate hitpoint */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x10 as libc::c_long) as u32b;
    /* Redraw monster hitpoint */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x10000000 as libc::c_long) as
            u32b;
    Term_load();
    character_icky = 0 as libc::c_int as bool_;
    if style_unchanged != 0 {
        msg_format(b"You are already using %s.\x00" as *const u8 as
                       *const libc::c_char,
                   melee_names[melee_num[z as usize] as usize]);
    };
}
#[no_mangle]
pub unsafe extern "C" fn select_default_melee() {
    let mut i: libc::c_int = 0;
    get_melee_skills();
    (*p_ptr).melee_style = 17 as libc::c_int as s16b;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if melee_bool[i as usize] != 0 {
            (*p_ptr).melee_style = melee_skills[i as usize];
            break ;
        } else { i += 1 }
    };
}
/*
 * Print a batch of skills.
 */
unsafe extern "C" fn print_skill_batch(mut p: *mut libc::c_int,
                                       mut p_desc: *mut cptr,
                                       mut start: libc::c_int,
                                       mut max: libc::c_int,
                                       mut mode: bool_) {
    let mut buff: [libc::c_char; 80] = [0; 80];
    let mut i: libc::c_int = start;
    let mut j: libc::c_int = 0 as libc::c_int;
    if mode != 0 {
        prt(format(b"         %-31s\x00" as *const u8 as *const libc::c_char,
                   b"Name\x00" as *const u8 as *const libc::c_char) as cptr,
            1 as libc::c_int, 20 as libc::c_int);
    }
    i = start;
    while i < start + 20 as libc::c_int {
        if i >= max { break ; }
        if *p.offset(i as isize) > 0 as libc::c_int {
            sprintf(buff.as_mut_ptr(),
                    b"  %c - %d) %-30s\x00" as *const u8 as
                        *const libc::c_char, j + 'a' as i32,
                    *p.offset(i as isize), *p_desc.offset(i as isize));
        } else {
            sprintf(buff.as_mut_ptr(),
                    b"  %c - %d) %-30s\x00" as *const u8 as
                        *const libc::c_char, j + 'a' as i32,
                    *p.offset(i as isize),
                    b"Change melee style\x00" as *const u8 as
                        *const libc::c_char);
        }
        if mode != 0 {
            prt(buff.as_mut_ptr() as cptr, 2 as libc::c_int + j,
                20 as libc::c_int);
        }
        j += 1;
        i += 1
    }
    if mode != 0 {
        prt(b"\x00" as *const u8 as *const libc::c_char, 2 as libc::c_int + j,
            20 as libc::c_int);
    }
    prt(format(b"Select a skill (a-%c), @ to select by name, +/- to scroll:\x00"
                   as *const u8 as *const libc::c_char,
               j - 1 as libc::c_int + 'a' as i32) as cptr, 0 as libc::c_int,
        0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn do_cmd_activate_skill_aux() -> libc::c_int {
    let mut which: libc::c_char = 0;
    let mut max: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut start: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut mode: bool_ = 0 as libc::c_int as bool_;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut p_desc: *mut cptr = 0 as *mut cptr;
    p =
        memset(ralloc(((max_s_idx as libc::c_int + max_ab_idx as libc::c_int)
                           as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((max_s_idx as libc::c_int + max_ab_idx as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong)) as
            *mut libc::c_int;
    p_desc =
        memset(ralloc(((max_s_idx as libc::c_int + max_ab_idx as libc::c_int)
                           as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<cptr>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((max_s_idx as libc::c_int + max_ab_idx as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<cptr>()
                                                    as libc::c_ulong)) as
            *mut cptr;
    /* Count the max */
    /* More than 1 melee skill ? */
    if get_melee_skills() as libc::c_int > 1 as libc::c_int {
        let ref mut fresh6 = *p_desc.offset(max as isize);
        *fresh6 =
            b"Change melee mode\x00" as *const u8 as *const libc::c_char;
        let fresh7 = max;
        max = max + 1;
        *p.offset(fresh7 as isize) = 0 as libc::c_int
    }
    i = 1 as libc::c_int;
    while i < max_s_idx as libc::c_int {
        if (*s_info.offset(i as isize)).action_mkey as libc::c_int != 0 &&
               (*s_info.offset(i as isize)).value != 0 &&
               ((*s_info.offset(i as isize)).hidden == 0 ||
                    i == 54 as libc::c_int) {
            let mut j: libc::c_int = 0;
            let mut next: bool_ = 0 as libc::c_int as bool_;
            /* Already got it ? */
            j = 0 as libc::c_int;
            while j < max {
                if (*s_info.offset(i as isize)).action_mkey as libc::c_int ==
                       *p.offset(j as isize) {
                    next = 1 as libc::c_int as bool_;
                    break ;
                } else { j += 1 }
            }
            if !(next != 0) {
                let ref mut fresh8 = *p_desc.offset(max as isize);
                *fresh8 =
                    s_text.offset((*s_info.offset(i as isize)).action_desc as
                                      isize) as cptr;
                let fresh9 = max;
                max = max + 1;
                *p.offset(fresh9 as isize) =
                    (*s_info.offset(i as isize)).action_mkey as libc::c_int
            }
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < max_ab_idx as libc::c_int {
        if (*ab_info.offset(i as isize)).action_mkey as libc::c_int != 0 &&
               (*ab_info.offset(i as isize)).acquired as libc::c_int != 0 {
            let mut j_0: libc::c_int = 0;
            let mut next_0: bool_ = 0 as libc::c_int as bool_;
            /* Already got it ? */
            j_0 = 0 as libc::c_int;
            while j_0 < max {
                if (*ab_info.offset(i as isize)).action_mkey as libc::c_int ==
                       *p.offset(j_0 as isize) {
                    next_0 = 1 as libc::c_int as bool_;
                    break ;
                } else { j_0 += 1 }
            }
            if !(next_0 != 0) {
                let ref mut fresh10 = *p_desc.offset(max as isize);
                *fresh10 =
                    ab_text.offset((*ab_info.offset(i as isize)).action_desc
                                       as isize) as cptr;
                let fresh11 = max;
                max = max + 1;
                *p.offset(fresh11 as isize) =
                    (*ab_info.offset(i as isize)).action_mkey as libc::c_int
            }
        }
        i += 1
    }
    if max == 0 {
        msg_print(b"You don\'t have any activable skills or abilities.\x00" as
                      *const u8 as *const libc::c_char);
        return -(1 as libc::c_int)
    }
    character_icky = 1 as libc::c_int as bool_;
    Term_save();
    loop  {
        print_skill_batch(p, p_desc, start, max, mode);
        which = inkey();
        if which as libc::c_int == '\u{1b}' as i32 {
            ret = -(1 as libc::c_int);
            break ;
        } else if which as libc::c_int == '*' as i32 ||
                      which as libc::c_int == '?' as i32 ||
                      which as libc::c_int == ' ' as i32 {
            mode =
                if mode as libc::c_int != 0 {
                    0 as libc::c_int
                } else { 1 as libc::c_int } as bool_;
            Term_load();
            character_icky = 0 as libc::c_int as bool_
        } else if which as libc::c_int == '+' as i32 {
            start += 20 as libc::c_int;
            if start >= max { start -= 20 as libc::c_int }
            Term_load();
            character_icky = 0 as libc::c_int as bool_
        } else if which as libc::c_int == '-' as i32 {
            start -= 20 as libc::c_int;
            if start < 0 as libc::c_int { start += 20 as libc::c_int }
            Term_load();
            character_icky = 0 as libc::c_int as bool_
        } else if which as libc::c_int == '@' as i32 {
            let mut buf: [libc::c_char; 80] = [0; 80];
            strcpy(buf.as_mut_ptr(),
                   b"Cast a spell\x00" as *const u8 as *const libc::c_char);
            if get_string(b"Skill action? \x00" as *const u8 as
                              *const libc::c_char, buf.as_mut_ptr(),
                          79 as libc::c_int) == 0 {
                return 0 as libc::c_int
            }
            /* Find the skill it is related to */
            i = 0 as libc::c_int;
            while i < max {
                if strcmp(buf.as_mut_ptr(), *p_desc.offset(i as isize)) == 0 {
                    break ;
                }
                i += 1
            }
            if !(i < max) { continue ; }
            ret = *p.offset(i as isize);
            break ;
        } else {
            which = tolower(which as libc::c_int) as libc::c_char;
            if start + (which as libc::c_int - 'a' as i32) >= max {
                bell();
            } else if (start + (which as libc::c_int - 'a' as i32)) <
                          0 as libc::c_int {
                bell();
            } else {
                ret =
                    *p.offset((start + (which as libc::c_int - 'a' as i32)) as
                                  isize);
                break ;
            }
        }
    }
    Term_load();
    character_icky = 0 as libc::c_int as bool_;
    rnfree(p as vptr,
           ((max_s_idx as libc::c_int + max_ab_idx as libc::c_int) as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong));
    rnfree(p_desc as vptr,
           ((max_s_idx as libc::c_int + max_ab_idx as libc::c_int) as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<cptr>() as
                                                libc::c_ulong));
    return ret;
}
/* Ask & execute a skill */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_activate_skill() {
    let mut x_idx: libc::c_int = 0;
    let mut push: bool_ = 1 as libc::c_int as bool_;
    /* Get the skill, if available */
    if repeat_pull(&mut x_idx) != 0 {
        push = 0 as libc::c_int as bool_
    } else if command_arg == 0 {
        x_idx = do_cmd_activate_skill_aux()
    } else {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        x_idx = command_arg as libc::c_int;
        /* Check validity */
        i = 1 as libc::c_int;
        while i < max_s_idx as libc::c_int {
            if (*s_info.offset(i as isize)).value != 0 &&
                   (*s_info.offset(i as isize)).action_mkey as libc::c_int ==
                       x_idx {
                break ;
            }
            i += 1
        }
        j = 0 as libc::c_int;
        while j < max_ab_idx as libc::c_int {
            if (*ab_info.offset(j as isize)).acquired as libc::c_int != 0 &&
                   (*ab_info.offset(j as isize)).action_mkey as libc::c_int ==
                       x_idx {
                break ;
            }
            j += 1
        }
        if j == max_ab_idx as libc::c_int && i == max_s_idx as libc::c_int {
            msg_print(b"Uh?\x00" as *const u8 as *const libc::c_char);
            return
        }
    }
    if x_idx == -(1 as libc::c_int) { return }
    if push != 0 { repeat_push(x_idx); }
    if x_idx == 0 { choose_melee(); return }
    /* Break goi/manashield */
    if (*p_ptr).invuln != 0 { set_invuln(0 as libc::c_int); }
    if (*p_ptr).disrupt_shield != 0 { set_disrupt_shield(0 as libc::c_int); }
    match x_idx {
        3 => { do_cmd_unbeliever(); }
        2 => { do_cmd_mindcraft(); }
        5 => { do_cmd_alchemist(); }
        6 => { do_cmd_mimic(); }
        8 => { do_cmd_powermage(); }
        9 => { do_cmd_runecrafter(); }
        10 => { do_cmd_archer(); }
        11 => { do_cmd_possessor(); }
        12 => { do_cmd_portable_hole(); }
        4 => { do_cmd_blade(); }
        13 => { do_cmd_summoner(); }
        7 => { do_cmd_necromancer(); }
        20 => { do_cmd_symbiotic(); }
        14 => { do_cmd_set_trap(); }
        15 => { do_cmd_steal(); }
        16 => { use_ability_blade(); }
        17 => { cast_school_spell(); }
        19 => { do_cmd_copy_spell(); }
        21 => { do_cmd_create_boulder(); }
        22 => {
            if get_skill(48 as libc::c_int) as libc::c_int >=
                   12 as libc::c_int {
                do_cmd_companion();
            } else {
                msg_print(b"You need a skill level of at least 12.\x00" as
                              *const u8 as *const libc::c_char);
            }
        }
        23 => { do_cmd_set_piercing(); }
        _ => {
            process_hooks(34 as libc::c_int,
                          b"(d)\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char, x_idx);
        }
    };
}
/* Which magic forbids non FA gloves */
#[no_mangle]
pub unsafe extern "C" fn forbid_gloves() -> bool_ {
    if get_skill(41 as libc::c_int) != 0 { return 1 as libc::c_int as bool_ }
    if get_skill(2 as libc::c_int) != 0 { return 1 as libc::c_int as bool_ }
    if get_skill(3 as libc::c_int) != 0 { return 1 as libc::c_int as bool_ }
    if get_skill(4 as libc::c_int) != 0 { return 1 as libc::c_int as bool_ }
    if get_skill(5 as libc::c_int) != 0 { return 1 as libc::c_int as bool_ }
    if get_skill(7 as libc::c_int) != 0 { return 1 as libc::c_int as bool_ }
    if get_skill(43 as libc::c_int) != 0 { return 1 as libc::c_int as bool_ }
    return 0 as libc::c_int as bool_;
}
/* Which gods forbid edged weapons */
#[no_mangle]
pub unsafe extern "C" fn forbid_non_blessed() -> bool_ {
    if (*p_ptr).pgod as libc::c_int == 1 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
/*
 * Gets the base value of a skill, given a race/class/...
 */
#[no_mangle]
pub unsafe extern "C" fn compute_skills(mut v: *mut s32b, mut m: *mut s32b,
                                        mut i: libc::c_int) {
    let mut value: s32b = 0;
    let mut mod_0: s32b = 0;
    /* **** general skills *****/
    /* If the skill correspond to the magic school lets pump them a bit */
    value = gen_skill_base[i as usize] as s32b;
    mod_0 = gen_skill_mod[i as usize] as s32b;
    *v = modify_aux(*v, value, gen_skill_basem[i as usize]);
    *m = modify_aux(*m, mod_0, gen_skill_modm[i as usize]);
    /* **** race skills *****/
    value = (*rp_ptr).skill_base[i as usize] as s32b;
    mod_0 = (*rp_ptr).skill_mod[i as usize] as s32b;
    *v = modify_aux(*v, value, (*rp_ptr).skill_basem[i as usize]);
    *m = modify_aux(*m, mod_0, (*rp_ptr).skill_modm[i as usize]);
    /* **** race mod skills *****/
    value = (*rmp_ptr).skill_base[i as usize] as s32b;
    mod_0 = (*rmp_ptr).skill_mod[i as usize] as s32b;
    *v = modify_aux(*v, value, (*rmp_ptr).skill_basem[i as usize]);
    *m = modify_aux(*m, mod_0, (*rmp_ptr).skill_modm[i as usize]);
    /* **** class skills *****/
    value = (*cp_ptr).skill_base[i as usize] as s32b;
    mod_0 = (*cp_ptr).skill_mod[i as usize] as s32b;
    *v = modify_aux(*v, value, (*cp_ptr).skill_basem[i as usize]);
    *m = modify_aux(*m, mod_0, (*cp_ptr).skill_modm[i as usize]);
    /* **** class spec skills *****/
    value = (*spp_ptr).skill_base[i as usize] as s32b;
    mod_0 = (*spp_ptr).skill_mod[i as usize] as s32b;
    *v = modify_aux(*v, value, (*spp_ptr).skill_basem[i as usize]);
    *m = modify_aux(*m, mod_0, (*spp_ptr).skill_modm[i as usize]);
}
/*
 * Initialize a skill with given values
 */
#[no_mangle]
pub unsafe extern "C" fn init_skill(mut value: s32b, mut mod_0: s32b,
                                    mut i: libc::c_int) {
    (*s_info.offset(i as isize)).value = value;
    (*s_info.offset(i as isize)).mod_0 = mod_0;
    if (*s_info.offset(i as isize)).flags1 &
           0x1 as libc::c_int as libc::c_uint != 0 {
        (*s_info.offset(i as isize)).hidden = 1 as libc::c_int as bool_
    } else {
        (*s_info.offset(i as isize)).hidden = 0 as libc::c_int as bool_
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_get_new_skill() {
    let mut items: [*mut libc::c_char; 4] = [0 as *mut libc::c_char; 4];
    let mut skl: [libc::c_int; 4] = [0; 4];
    let mut val: [s32b; 4] = [0; 4];
    let mut mod_0: [s32b; 4] = [0; 4];
    let mut used: [bool_; 200] = [0; 200];
    let mut available_skills: [libc::c_int; 200] = [0; 200];
    let mut max: libc::c_int = 0 as libc::c_int;
    let mut max_a: libc::c_int = 0 as libc::c_int;
    let mut res: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    /* Check if some skills didn't influence other stuff */
    recalc_skills(1 as libc::c_int as bool_);
    /* Grab the ones we can gain */
    max = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < max_s_idx as libc::c_int {
        if (*s_info.offset(i as isize)).flags1 &
               0x4 as libc::c_int as libc::c_uint != 0 {
            let fresh12 = max;
            max = max + 1;
            available_skills[fresh12 as usize] = i
        }
        i += 1
    }
    let fresh13 = max;
    max = max + 1;
    available_skills[fresh13 as usize] = -(1 as libc::c_int);
    /* Init */
    max = 0 as libc::c_int;
    while max < 200 as libc::c_int {
        used[max as usize] = 0 as libc::c_int as bool_;
        max += 1
    }
    /* Count the number of available skills */
    while available_skills[max_a as usize] != -(1 as libc::c_int) {
        max_a += 1
    }
    /* Get LOST_SWORD_NSKILLS skills */
    max = 0 as libc::c_int;
    while max < 4 as libc::c_int {
        let mut i_0: libc::c_int = 0;
        let mut s_ptr: *mut skill_type = 0 as *mut skill_type;
        loop 
             /* Get an non used skill */
             {
            i_0 = Rand_div(max_a);
            /* Does it pass the check? */
            !(Rand_div(100 as libc::c_int) <
                  (*s_info.offset(available_skills[i_0 as usize] as
                                      isize)).random_gain_chance as
                      libc::c_int);
            if !(used[available_skills[i_0 as usize] as usize] != 0) {
                break ;
            }
        }
        s_ptr =
            &mut *s_info.offset(*available_skills.as_mut_ptr().offset(i_0 as
                                                                          isize)
                                    as isize) as *mut skill_type;
        used[available_skills[i_0 as usize] as usize] =
            1 as libc::c_int as bool_;
        if (*s_ptr).mod_0 != 0 {
            if (*s_ptr).mod_0 < 300 as libc::c_int {
                val[max as usize] = 1000 as libc::c_int;
                mod_0[max as usize] = 300 as libc::c_int - (*s_ptr).mod_0
            } else if (*s_ptr).mod_0 < 500 as libc::c_int {
                val[max as usize] = (*s_ptr).mod_0 * 1 as libc::c_int;
                mod_0[max as usize] = 100 as libc::c_int;
                if mod_0[max as usize] + (*s_ptr).mod_0 > 500 as libc::c_int {
                    mod_0[max as usize] = 500 as libc::c_int - (*s_ptr).mod_0
                }
            } else {
                val[max as usize] = (*s_ptr).mod_0 * 3 as libc::c_int;
                mod_0[max as usize] = 0 as libc::c_int
            }
        } else {
            mod_0[max as usize] = 300 as libc::c_int;
            val[max as usize] = 1000 as libc::c_int
        }
        if (*s_ptr).value + val[max as usize] > 50000 as libc::c_int {
            val[max as usize] = 50000 as libc::c_int - (*s_ptr).value
        }
        skl[max as usize] = available_skills[i_0 as usize];
        items[max as usize] =
            string_make(format(b"%-40s: +%02ld.%03ld value, +%01d.%03d modifier\x00"
                                   as *const u8 as *const libc::c_char,
                               s_name.offset((*s_ptr).name as isize),
                               val[max as usize] / 1000 as libc::c_int,
                               val[max as usize] % 1000 as libc::c_int,
                               mod_0[max as usize] / 1000 as libc::c_int,
                               mod_0[max as usize] % 1000 as libc::c_int) as
                            cptr) as *mut libc::c_char;
        max += 1
    }
    loop  {
        let mut last: libc::c_char =
            ('a' as i32 + (4 as libc::c_int - 1 as libc::c_int)) as
                libc::c_char;
        let mut buf: [libc::c_char; 80] = [0; 80];
        sprintf(buf.as_mut_ptr(),
                b"Choose a skill to learn(a-%c to choose, ESC to cancel)?\x00"
                    as *const u8 as *const libc::c_char, last as libc::c_int);
        res =
            ask_menu(buf.as_mut_ptr() as cptr, items.as_mut_ptr(),
                     4 as libc::c_int);
        /* Ok ? lets learn ! */
        if !(res > -(1 as libc::c_int)) { continue ; }
        let mut s_ptr_0: *mut skill_type = 0 as *mut skill_type;
        let mut oppose: bool_ = 0 as libc::c_int as bool_;
        let mut oppose_skill: libc::c_int = -(1 as libc::c_int);
        /* Check we don't oppose an existing skill */
        i = 0 as libc::c_int;
        while i < max_s_idx as libc::c_int {
            if (*s_info.offset(i as isize)).action[skl[res as usize] as usize]
                   as libc::c_int == 9999 as libc::c_int &&
                   (*s_info.offset(i as isize)).value != 0 as libc::c_int {
                oppose = 1 as libc::c_int as bool_;
                oppose_skill = i;
                break ;
            } else { i += 1 }
        }
        /* Ok we oppose, so be sure */
        if oppose != 0 {
            let mut msg: cptr = 0 as *const libc::c_char;
            /*
				 * Because this is SO critical a question, we must flush
				 * input to prevent killing character off -- pelpel
				 */
            flush();
            /* Prepare prompt */
            msg =
                format(b"This skill is mutually exclusive with at least %s, continue?\x00"
                           as *const u8 as *const libc::c_char,
                       s_name.offset((*s_info.offset(oppose_skill as
                                                         isize)).name as
                                         isize)) as cptr;
            /* The player rejected the choice */
            if get_check(msg) == 0 { continue ; }
        }
        s_ptr_0 =
            &mut *s_info.offset(*skl.as_mut_ptr().offset(res as isize) as
                                    isize) as *mut skill_type;
        (*s_ptr_0).value += val[res as usize];
        (*s_ptr_0).mod_0 += mod_0[res as usize];
        if mod_0[res as usize] != 0 {
            msg_format(b"You can now learn the %s skill.\x00" as *const u8 as
                           *const libc::c_char,
                       s_name.offset((*s_ptr_0).name as isize));
        } else {
            msg_format(b"Your knowledge of the %s skill increases.\x00" as
                           *const u8 as *const libc::c_char,
                       s_name.offset((*s_ptr_0).name as isize));
        }
        break ;
    }
    /* Free them ! */
    max = 0 as libc::c_int;
    while max < 4 as libc::c_int {
        string_free(items[max as usize] as cptr);
        max += 1
    }
    /* Check if some skills didn't influence other stuff */
    recalc_skills(0 as libc::c_int as bool_);
}
/* *************************************** ABILITIES *****************************************/
/*
 * Given the name of an ability, returns ability index or -1 if no
 * such ability is found
 */
#[no_mangle]
pub unsafe extern "C" fn find_ability(mut name: cptr) -> s16b {
    let mut i: u16b = 0;
    /* Scan ability list */
    i = 0 as libc::c_int as u16b;
    while (i as libc::c_int) < max_ab_idx as libc::c_int {
        /* The name matches */
        if streq(ab_name.offset((*ab_info.offset(i as isize)).name as isize)
                     as cptr, name) != 0 {
            return i as s16b
        }
        i = i.wrapping_add(1)
    }
    /* No match found */
    return -(1 as libc::c_int) as s16b;
}
/*
 * Do the player have the ability
 */
#[no_mangle]
pub unsafe extern "C" fn has_ability(mut ab: libc::c_int) -> bool_ {
    return (*ab_info.offset(ab as isize)).acquired;
}
/* Do we meet the requirements */
#[no_mangle]
pub unsafe extern "C" fn can_learn_ability(mut ab: libc::c_int) -> bool_ {
    let mut ab_ptr: *mut ability_type =
        &mut *ab_info.offset(ab as isize) as *mut ability_type;
    let mut i: libc::c_int = 0;
    if (*ab_ptr).acquired != 0 { return 0 as libc::c_int as bool_ }
    if ((*p_ptr).skill_points as libc::c_int) <
           (*ab_info.offset(ab as isize)).cost as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        /* Must have skill level */
        if (*ab_ptr).skills[i as usize] as libc::c_int > -(1 as libc::c_int) {
            if (get_skill((*ab_ptr).skills[i as usize] as libc::c_int) as
                    libc::c_int) <
                   (*ab_ptr).skill_levels[i as usize] as libc::c_int {
                return 0 as libc::c_int as bool_
            }
        }
        /* Must have ability */
        if (*ab_ptr).need_abilities[i as usize] as libc::c_int >
               -(1 as libc::c_int) {
            if (*ab_info.offset((*ab_ptr).need_abilities[i as usize] as
                                    isize)).acquired == 0 {
                return 0 as libc::c_int as bool_
            }
        }
        /* Must not have ability */
        if (*ab_ptr).forbid_abilities[i as usize] as libc::c_int >
               -(1 as libc::c_int) {
            if (*ab_info.offset((*ab_ptr).forbid_abilities[i as usize] as
                                    isize)).acquired != 0 {
                return 0 as libc::c_int as bool_
            }
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        /* Must have stat */
        if (*ab_ptr).stat[i as usize] as libc::c_int > -(1 as libc::c_int) {
            if ((*p_ptr).stat_ind[i as usize] as libc::c_int) <
                   (*ab_ptr).stat[i as usize] as libc::c_int -
                       3 as libc::c_int {
                return 0 as libc::c_int as bool_
            }
        }
        i += 1
    }
    /* Do the script allow us? */
    if process_hooks(70 as libc::c_int,
                     b"(d)\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char, ab) != 0 {
        return 0 as libc::c_int as bool_
    }
    return 1 as libc::c_int as bool_;
}
/* Learn an ability */
#[no_mangle]
pub unsafe extern "C" fn gain_ability(mut ab: libc::c_int) {
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    Term_get_size(&mut wid, &mut hgt);
    if can_learn_ability(ab) == 0 {
        msg_box(b"You cannot learn this ability.\x00" as *const u8 as
                    *const libc::c_char, hgt / 2 as libc::c_int,
                wid / 2 as libc::c_int);
        return
    }
    /* Flush input as we ask an important and irreversible question */
    flush();
    /* Ask we can commit the change */
    if msg_box(b"Learn this ability(this is permanent)? (y/n)\x00" as
                   *const u8 as *const libc::c_char, hgt / 2 as libc::c_int,
               wid / 2 as libc::c_int) as libc::c_int != 'y' as i32 {
        return
    }
    (*ab_info.offset(ab as isize)).acquired = 1 as libc::c_int as bool_;
    (*p_ptr).skill_points =
        ((*p_ptr).skill_points as libc::c_int -
             (*ab_info.offset(ab as isize)).cost as libc::c_int) as s16b;
}
/* helper function to generate a sorted table */
unsafe extern "C" fn add_sorted_ability(mut table: *mut libc::c_int,
                                        mut max: *mut libc::c_int,
                                        mut ab: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < *max {
        if strcmp(ab_name.offset((*ab_info.offset(ab as isize)).name as
                                     isize),
                  ab_name.offset((*ab_info.offset(*table.offset(i as isize) as
                                                      isize)).name as isize))
               < 0 as libc::c_int {
            let mut z: libc::c_int = 0;
            /* Move all indexes up */
            z = *max;
            while z > i {
                *table.offset(z as isize) =
                    *table.offset((z - 1 as libc::c_int) as isize);
                z -= 1
            }
            break ;
        } else { i += 1 }
    }
    *table.offset(i as isize) = ab;
    *max += 1;
}
/*
 * Print the abilities list
 */
#[no_mangle]
pub unsafe extern "C" fn dump_abilities(mut fff: *mut FILE) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut table: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut max: libc::c_int = 0 as libc::c_int;
    table =
        memset(ralloc((max_ab_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_ab_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong)) as
            *mut libc::c_int;
    /* Initialise the abilities list */
    i = 0 as libc::c_int;
    while i < max_ab_idx as libc::c_int {
        if (*ab_info.offset(i as isize)).name != 0 &&
               has_ability(i) as libc::c_int != 0 {
            add_sorted_ability(table, &mut max, i);
        }
        i += 1
    }
    if max != 0 {
        fprintf(fff, b"\nAbilities\x00" as *const u8 as *const libc::c_char);
        j = 0 as libc::c_int;
        while j < max {
            i = *table.offset(j as isize);
            fprintf(fff, b"\n * %s\x00" as *const u8 as *const libc::c_char,
                    ab_name.offset((*ab_info.offset(i as isize)).name as
                                       isize));
            j += 1
        }
        fprintf(fff, b"\n\x00" as *const u8 as *const libc::c_char);
    };
}
/*
 * Draw the abilities list
 */
#[no_mangle]
pub unsafe extern "C" fn print_abilities(mut table: *mut libc::c_int,
                                         mut max: libc::c_int,
                                         mut sel: libc::c_int,
                                         mut start: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    let mut keys: cptr = 0 as *const libc::c_char;
    Term_clear();
    Term_get_size(&mut wid, &mut hgt);
    c_prt(1 as libc::c_int as byte_hack,
          format(b"%s Abilities Screen\x00" as *const u8 as
                     *const libc::c_char, game_module) as cptr,
          0 as libc::c_int, 28 as libc::c_int);
    keys =
        format(b"#Bup#W/#Bdown#W to move, #Bright#W to buy, #B?#W for help\x00"
                   as *const u8 as *const libc::c_char) as cptr;
    display_message(0 as libc::c_int, 1 as libc::c_int,
                    strlen(keys) as libc::c_int,
                    1 as libc::c_int as byte_hack, keys);
    c_prt(if (*p_ptr).skill_points as libc::c_int != 0 {
              14 as libc::c_int
          } else { 12 as libc::c_int } as byte_hack,
          format(b"Skill points left: %d\x00" as *const u8 as
                     *const libc::c_char,
                 (*p_ptr).skill_points as libc::c_int) as cptr,
          2 as libc::c_int, 0 as libc::c_int);
    print_desc_aux(ab_text.offset((*ab_info.offset(*table.offset(sel as isize)
                                                       as isize)).desc as
                                      isize) as cptr, 3 as libc::c_int,
                   0 as libc::c_int);
    j = start;
    while j < start + (hgt - 7 as libc::c_int) {
        let mut color: byte_hack = 1 as libc::c_int as byte_hack;
        let mut deb: libc::c_char = ' ' as i32 as libc::c_char;
        let mut end: libc::c_char = ' ' as i32 as libc::c_char;
        if j >= max { break ; }
        i = *table.offset(j as isize);
        if (*ab_info.offset(i as isize)).acquired != 0 {
            color = 14 as libc::c_int as byte_hack
        } else if can_learn_ability(i) != 0 {
            color = 1 as libc::c_int as byte_hack
        } else { color = 8 as libc::c_int as byte_hack }
        if j == sel {
            color = 13 as libc::c_int as byte_hack;
            deb = '[' as i32 as libc::c_char;
            end = ']' as i32 as libc::c_char
        }
        c_prt(color,
              format(b"%c.%c%s\x00" as *const u8 as *const libc::c_char,
                     deb as libc::c_int, end as libc::c_int,
                     ab_name.offset((*ab_info.offset(i as isize)).name as
                                        isize)) as cptr,
              j + 7 as libc::c_int - start, 0 as libc::c_int);
        if (*ab_info.offset(i as isize)).acquired == 0 {
            c_prt(color,
                  format(b"%d\x00" as *const u8 as *const libc::c_char,
                         (*ab_info.offset(i as isize)).cost as libc::c_int) as
                      cptr, j + 7 as libc::c_int - start, 60 as libc::c_int);
        } else {
            c_prt(color, b"Known\x00" as *const u8 as *const libc::c_char,
                  j + 7 as libc::c_int - start, 60 as libc::c_int);
        }
        j += 1
    };
}
/*
 * Interreact with abilitiess
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_ability() {
    let mut sel: libc::c_int = 0 as libc::c_int;
    let mut start: libc::c_int = 0 as libc::c_int;
    let mut max: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_char = 0;
    let mut table: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    table =
        memset(ralloc((max_ab_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_ab_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong)) as
            *mut libc::c_int;
    /* Save the screen */
    screen_save();
    /* Clear the screen */
    Term_clear();
    /* Initialise the abilities list */
    i = 0 as libc::c_int;
    while i < max_ab_idx as libc::c_int {
        if (*ab_info.offset(i as isize)).name != 0 {
            add_sorted_ability(table, &mut max, i);
        }
        i += 1
    }
    loop  {
        Term_get_size(&mut wid, &mut hgt);
        /* Display list of skills */
        print_abilities(table, max, sel, start);
        /* Wait for user input */
        c = inkey();
        /* Leave the skill screen */
        if c as libc::c_int == '\u{1b}' as i32 { break ; }
        /* Next page */
        if c as libc::c_int == 'n' as i32 {
            sel += hgt - 7 as libc::c_int;
            if sel >= max { sel = max - 1 as libc::c_int }
        } else if c as libc::c_int == 'p' as i32 {
            sel -= hgt - 7 as libc::c_int;
            if sel < 0 as libc::c_int { sel = 0 as libc::c_int }
        } else {
            /* Previous page */
            /* Select / increase a skill */
            let mut dir: libc::c_int = 0;
            /* Allow use of numpad / arrow keys / roguelike keys */
            dir = get_keymap_dir(c);
            /* Move cursor down */
            if dir == 2 as libc::c_int { sel += 1 }
            /* Move cursor up */
            if dir == 8 as libc::c_int { sel -= 1 }
            /* gain ability */
            if dir == 6 as libc::c_int {
                gain_ability(*table.offset(sel as isize));
            }
            /* XXX XXX XXX Wizard mode commands outside of wizard2.c */
            if wizard as libc::c_int != 0 && c as libc::c_int == '+' as i32 {
                (*ab_info.offset(*table.offset(sel as isize) as
                                     isize)).acquired =
                    1 as libc::c_int as bool_
            }
            if wizard as libc::c_int != 0 && c as libc::c_int == '-' as i32 {
                (*ab_info.offset(*table.offset(sel as isize) as
                                     isize)).acquired =
                    0 as libc::c_int as bool_
            }
            /* Contextual help */
            if c as libc::c_int == '?' as i32 {
                exec_lua(format(b"ingame_help(\'select_context\', \'ability\', \'%s\')\x00"
                                    as *const u8 as *const libc::c_char,
                                ab_name.offset((*ab_info.offset(*table.offset(sel
                                                                                  as
                                                                                  isize)
                                                                    as
                                                                    isize)).name
                                                   as isize)));
            }
            /* Handle boundaries and scrolling */
            if sel < 0 as libc::c_int { sel = max - 1 as libc::c_int }
            if sel >= max { sel = 0 as libc::c_int }
            if sel < start { start = sel }
            if sel >= start + (hgt - 7 as libc::c_int) {
                start = sel - (hgt - 7 as libc::c_int) + 1 as libc::c_int
            }
        }
    }
    /* Load the screen */
    screen_load();
    rnfree(table as vptr,
           (max_ab_idx as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong));
    /* Update stuffs */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x1 as libc::c_long | 0x10 as libc::c_long |
                  0x20 as libc::c_long | 0x40 as libc::c_long |
                  0x80 as libc::c_long | 0x8 as libc::c_long |
                  0x4 as libc::c_long)) as u32b;
    /* Redraw various info */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long |
             (0x8000000 as libc::c_long | 0x2000000 as libc::c_long |
                  0x1000000 as libc::c_long | 0x4000000 as libc::c_long)) as
            u32b;
}
/*
 * Apply abilities to be granted this level
 */
#[no_mangle]
pub unsafe extern "C" fn apply_level_abilities(mut level: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        if (*cp_ptr).abilities[i as usize].level as libc::c_int == level {
            if level > 1 as libc::c_int &&
                   (*ab_info.offset((*cp_ptr).abilities[i as usize].ability as
                                        isize)).acquired == 0 {
                cmsg_format(13 as libc::c_int as byte_hack,
                            b"You have learned the ability \'%s\'.\x00" as
                                *const u8 as *const libc::c_char,
                            ab_name.offset((*ab_info.offset((*cp_ptr).abilities[i
                                                                                    as
                                                                                    usize].ability
                                                                as
                                                                isize)).name
                                               as isize));
            }
            (*ab_info.offset((*cp_ptr).abilities[i as usize].ability as
                                 isize)).acquired = 1 as libc::c_int as bool_
        }
        if (*spp_ptr).abilities[i as usize].level as libc::c_int == level {
            if level > 1 as libc::c_int &&
                   (*ab_info.offset((*spp_ptr).abilities[i as usize].ability
                                        as isize)).acquired == 0 {
                cmsg_format(13 as libc::c_int as byte_hack,
                            b"You have learned the ability \'%s\'.\x00" as
                                *const u8 as *const libc::c_char,
                            ab_name.offset((*ab_info.offset((*spp_ptr).abilities[i
                                                                                     as
                                                                                     usize].ability
                                                                as
                                                                isize)).name
                                               as isize));
            }
            (*ab_info.offset((*spp_ptr).abilities[i as usize].ability as
                                 isize)).acquired = 1 as libc::c_int as bool_
        }
        if (*rp_ptr).abilities[i as usize].level as libc::c_int == level {
            if level > 1 as libc::c_int &&
                   (*ab_info.offset((*rp_ptr).abilities[i as usize].ability as
                                        isize)).acquired == 0 {
                cmsg_format(13 as libc::c_int as byte_hack,
                            b"You have learned the ability \'%s\'.\x00" as
                                *const u8 as *const libc::c_char,
                            ab_name.offset((*ab_info.offset((*rp_ptr).abilities[i
                                                                                    as
                                                                                    usize].ability
                                                                as
                                                                isize)).name
                                               as isize));
            }
            (*ab_info.offset((*rp_ptr).abilities[i as usize].ability as
                                 isize)).acquired = 1 as libc::c_int as bool_
        }
        if (*rmp_ptr).abilities[i as usize].level as libc::c_int == level {
            if level > 1 as libc::c_int &&
                   (*ab_info.offset((*rmp_ptr).abilities[i as usize].ability
                                        as isize)).acquired == 0 {
                cmsg_format(13 as libc::c_int as byte_hack,
                            b"You have learned the ability \'%s\'.\x00" as
                                *const u8 as *const libc::c_char,
                            ab_name.offset((*ab_info.offset((*rmp_ptr).abilities[i
                                                                                     as
                                                                                     usize].ability
                                                                as
                                                                isize)).name
                                               as isize));
            }
            (*ab_info.offset((*rmp_ptr).abilities[i as usize].ability as
                                 isize)).acquired = 1 as libc::c_int as bool_
        }
        i += 1
    };
}
