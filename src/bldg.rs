use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut arena_monsters: [s16b; 29];
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut command_arg: s16b;
    #[no_mangle]
    static mut command_rep: s16b;
    #[no_mangle]
    static mut command_new: s16b;
    #[no_mangle]
    static mut cur_hgt: s16b;
    #[no_mangle]
    static mut cur_wid: s16b;
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut turn: s32b;
    #[no_mangle]
    static mut leaving_quest: libc::c_int;
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
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
    static mut spp_ptr: *mut player_spec;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut r_name: *mut libc::c_char;
    #[no_mangle]
    static mut st_info: *mut store_info_type;
    #[no_mangle]
    static mut ba_info: *mut store_action_type;
    #[no_mangle]
    static mut ba_name: *mut libc::c_char;
    #[no_mangle]
    static mut ow_info: *mut owner_type;
    #[no_mangle]
    static mut item_tester_hook:
           Option<unsafe extern "C" fn(_: *mut object_type) -> bool_>;
    #[no_mangle]
    static mut get_mon_num_hook:
           Option<unsafe extern "C" fn(_: libc::c_int) -> bool_>;
    #[no_mangle]
    static mut bounties: [[s16b; 2]; 24];
    #[no_mangle]
    static mut fates: [fate; 200];
    #[no_mangle]
    static mut total_bounties: u32b;
    #[no_mangle]
    static mut plots: [s16b; 7];
    #[no_mangle]
    static mut quest: *mut quest_type;
    #[no_mangle]
    static mut process_hooks_return: [hook_return; 20];
    #[no_mangle]
    fn process_hooks_ret(h_idx: libc::c_int, ret: *mut libc::c_char,
                         fmt: *mut libc::c_char, _: ...) -> bool_;
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn forget_view();
    #[no_mangle]
    fn research_mon() -> bool_;
    #[no_mangle]
    fn show_file(name: cptr, what: cptr, line: libc::c_int, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn get_rnd_line(file_name: *mut libc::c_char, output: *mut libc::c_char)
     -> errr;
    #[no_mangle]
    fn race_legends();
    #[no_mangle]
    fn show_highclass(building: libc::c_int);
    #[no_mangle]
    fn wilderness_gen(refresh: libc::c_int);
    #[no_mangle]
    fn get_mon_num_prep() -> errr;
    #[no_mangle]
    fn get_mon_num(level: libc::c_int) -> s16b;
    #[no_mangle]
    fn inc_stack_size(item: libc::c_int, delta: libc::c_int);
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn wield_slot(o_ptr: *mut object_type) -> s16b;
    #[no_mangle]
    fn atol(__nptr: *const libc::c_char) -> libc::c_long;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn Term_clear() -> errr;
    #[no_mangle]
    fn randnor(mean: libc::c_int, stand: libc::c_int) -> s16b;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn strnfmt(buf: *mut libc::c_char, max: uint_hack, fmt: cptr, _: ...)
     -> uint_hack;
    #[no_mangle]
    fn get_item(cp: *mut libc::c_int, pmt: cptr, str: cptr, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn object_value_real(o_ptr: *mut object_type) -> s32b;
    #[no_mangle]
    fn object_wipe(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_copy(o_ptr: *mut object_type, j_ptr: *mut object_type);
    #[no_mangle]
    fn hp_player(num: libc::c_int) -> bool_;
    #[no_mangle]
    fn do_res_stat(stat: libc::c_int, full: bool_) -> bool_;
    #[no_mangle]
    fn identify_pack();
    #[no_mangle]
    fn identify_fully() -> bool_;
    #[no_mangle]
    fn recharge(num: libc::c_int) -> bool_;
    #[no_mangle]
    fn reset_recall(no_trepas_max_depth: bool_) -> bool_;
    #[no_mangle]
    fn store_sell();
    #[no_mangle]
    fn store_purchase();
    #[no_mangle]
    fn store_examine();
    #[no_mangle]
    fn store_stole();
    #[no_mangle]
    fn store_prt_gold();
    #[no_mangle]
    fn store_request_item();
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn get_quantity(prompt: cptr, max: s32b) -> s32b;
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn cmsg_print(color: byte_hack, msg: cptr);
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn c_prt(attr: byte_hack, str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn set_mimic(v: libc::c_int, p: libc::c_int, level: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_stun(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_cut(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_confused(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_blind(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_poisoned(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn put_str(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn calc_bonuses(silent: bool_);
    #[no_mangle]
    fn c_put_str(attr: byte_hack, str: cptr, row: libc::c_int,
                 col: libc::c_int);
    #[no_mangle]
    fn bst(what: s32b, t: s32b) -> s32b;
    #[no_mangle]
    fn set_food(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn resolve_mimic_name(name: cptr) -> libc::c_int;
    #[no_mangle]
    fn screen_load();
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn move_cursor(row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn get_string(prompt: cptr, buf: *mut libc::c_char, len: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn screen_save();
    #[no_mangle]
    fn handle_stuff();
    #[no_mangle]
    fn notice_stuff();
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
pub type uint_hack = libc::c_uint;
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
pub struct fate {
    pub fate: byte_hack,
    pub level: byte_hack,
    pub serious: byte_hack,
    pub o_idx: s16b,
    pub e_idx: s16b,
    pub a_idx: s16b,
    pub v_idx: s16b,
    pub r_idx: s16b,
    pub count: s16b,
    pub time: s16b,
    pub know: bool_,
    pub icky: bool_,
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
pub union hook_return {
    pub num: s32b,
    pub str_0: cptr,
    pub o_ptr: *mut object_type,
    pub m_ptr: *mut monster_type,
}
/* File: bldg.c */
/*
 * Purpose: Building commands
 * Created by Ken Wigle for Kangband - a variant of Angband 2.8.3
 * -KMW-
 *
 * Rewritten for Kangband 2.8.3i using Kamband's version of
 * bldg.c as written by Ivan Tkatchev
 *
 * Changed for ZAngband by Robert Ruehlmann
 *
 * Heavily modified for ToME by DarkGod
 */
/* hack as in leave_store in store.c */
static mut leave_bldg: bool_ = 0 as libc::c_int as bool_;
/* remember building location */
static mut building_loc: libc::c_int = 0 as libc::c_int;
/*
 * A helper function for is_state
 */
#[no_mangle]
pub unsafe extern "C" fn is_state_aux(mut s_ptr: *mut store_type,
                                      mut state: libc::c_int) -> bool_ {
    let mut ow_ptr: *mut owner_type =
        &mut *ow_info.offset((*s_ptr).owner as isize) as *mut owner_type;
    /* Check race */
    if (*ow_ptr).races[state as
                           usize][((*p_ptr).prace as libc::c_int /
                                       32 as libc::c_int) as usize] &
           ((1 as libc::c_int) << (*p_ptr).prace as libc::c_int) as
               libc::c_uint != 0 {
        return 1 as libc::c_int as bool_
    }
    /* Check class */
    if (*ow_ptr).classes[state as
                             usize][((*p_ptr).prace as libc::c_int /
                                         32 as libc::c_int) as usize] &
           ((1 as libc::c_int) << (*p_ptr).pclass as libc::c_int) as
               libc::c_uint != 0 {
        return 1 as libc::c_int as bool_
    }
    /* All failed */
    return 0 as libc::c_int as bool_;
}
/*
 * Test if the state accords with the player
 */
#[no_mangle]
pub unsafe extern "C" fn is_state(mut s_ptr: *mut store_type,
                                  mut state: libc::c_int) -> bool_ {
    if state == 2 as libc::c_int {
        if is_state_aux(s_ptr, 1 as libc::c_int) != 0 {
            return 0 as libc::c_int as bool_
        }
        if is_state_aux(s_ptr, 0 as libc::c_int) != 0 {
            return 0 as libc::c_int as bool_
        }
        return 1 as libc::c_int as bool_
    } else { return is_state_aux(s_ptr, state) };
}
/*
 * Clear the building information
 */
unsafe extern "C" fn clear_bldg(mut min_row: libc::c_int,
                                mut max_row: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = min_row;
    while i <= max_row {
        prt(b"\x00" as *const u8 as *const libc::c_char, i, 0 as libc::c_int);
        i += 1
    };
}
/*
 * Display a building.
 */
#[no_mangle]
pub unsafe extern "C" fn show_building(mut s_ptr: *mut store_type) {
    let mut buff: [libc::c_char; 20] = [0; 20];
    let mut i: libc::c_int = 0;
    let mut action_color: byte_hack = 0;
    let mut tmp_str: [libc::c_char; 80] = [0; 80];
    let mut st_ptr: *mut store_info_type =
        &mut *st_info.offset((*s_ptr).st_idx as isize) as
            *mut store_info_type;
    let mut ba_ptr: *mut store_action_type = 0 as *mut store_action_type;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        ba_ptr =
            &mut *ba_info.offset(*(*st_ptr).actions.as_mut_ptr().offset(i as
                                                                            isize)
                                     as isize) as *mut store_action_type;
        if (*ba_ptr).letter as libc::c_int != '.' as i32 {
            if (*ba_ptr).action_restr as libc::c_int == 0 as libc::c_int {
                if is_state(s_ptr, 1 as libc::c_int) as libc::c_int != 0 &&
                       (*ba_ptr).costs[1 as libc::c_int as usize] as
                           libc::c_int == 0 as libc::c_int ||
                       is_state(s_ptr, 0 as libc::c_int) as libc::c_int != 0
                           &&
                           (*ba_ptr).costs[0 as libc::c_int as usize] as
                               libc::c_int == 0 as libc::c_int ||
                       is_state(s_ptr, 2 as libc::c_int) as libc::c_int != 0
                           &&
                           (*ba_ptr).costs[2 as libc::c_int as usize] as
                               libc::c_int == 0 as libc::c_int {
                    action_color = 1 as libc::c_int as byte_hack;
                    buff[0 as libc::c_int as usize] =
                        '\u{0}' as i32 as libc::c_char
                } else if is_state(s_ptr, 1 as libc::c_int) != 0 {
                    action_color = 13 as libc::c_int as byte_hack;
                    strnfmt(buff.as_mut_ptr(), 20 as libc::c_int as uint_hack,
                            b"(%dgp)\x00" as *const u8 as *const libc::c_char,
                            (*ba_ptr).costs[1 as libc::c_int as usize] as
                                libc::c_int);
                } else if is_state(s_ptr, 0 as libc::c_int) != 0 {
                    action_color = 4 as libc::c_int as byte_hack;
                    strnfmt(buff.as_mut_ptr(), 20 as libc::c_int as uint_hack,
                            b"(%dgp)\x00" as *const u8 as *const libc::c_char,
                            (*ba_ptr).costs[0 as libc::c_int as usize] as
                                libc::c_int);
                } else {
                    action_color = 11 as libc::c_int as byte_hack;
                    strnfmt(buff.as_mut_ptr(), 20 as libc::c_int as uint_hack,
                            b"(%dgp)\x00" as *const u8 as *const libc::c_char,
                            (*ba_ptr).costs[2 as libc::c_int as usize] as
                                libc::c_int);
                }
            } else if (*ba_ptr).action_restr as libc::c_int ==
                          1 as libc::c_int {
                if is_state(s_ptr, 1 as libc::c_int) as libc::c_int != 0 &&
                       (*ba_ptr).costs[1 as libc::c_int as usize] as
                           libc::c_int == 0 as libc::c_int ||
                       is_state(s_ptr, 2 as libc::c_int) as libc::c_int != 0
                           &&
                           (*ba_ptr).costs[2 as libc::c_int as usize] as
                               libc::c_int == 0 as libc::c_int {
                    action_color = 1 as libc::c_int as byte_hack;
                    buff[0 as libc::c_int as usize] =
                        '\u{0}' as i32 as libc::c_char
                } else if is_state(s_ptr, 1 as libc::c_int) != 0 {
                    action_color = 13 as libc::c_int as byte_hack;
                    strnfmt(buff.as_mut_ptr(), 20 as libc::c_int as uint_hack,
                            b"(%dgp)\x00" as *const u8 as *const libc::c_char,
                            (*ba_ptr).costs[1 as libc::c_int as usize] as
                                libc::c_int);
                } else if is_state(s_ptr, 0 as libc::c_int) != 0 {
                    action_color = 8 as libc::c_int as byte_hack;
                    strnfmt(buff.as_mut_ptr(), 20 as libc::c_int as uint_hack,
                            b"(closed)\x00" as *const u8 as
                                *const libc::c_char);
                } else {
                    action_color = 11 as libc::c_int as byte_hack;
                    strnfmt(buff.as_mut_ptr(), 20 as libc::c_int as uint_hack,
                            b"(%dgp)\x00" as *const u8 as *const libc::c_char,
                            (*ba_ptr).costs[2 as libc::c_int as usize] as
                                libc::c_int);
                }
            } else if is_state(s_ptr, 1 as libc::c_int) as libc::c_int != 0 &&
                          (*ba_ptr).costs[1 as libc::c_int as usize] as
                              libc::c_int == 0 as libc::c_int {
                action_color = 1 as libc::c_int as byte_hack;
                buff[0 as libc::c_int as usize] =
                    '\u{0}' as i32 as libc::c_char
            } else if is_state(s_ptr, 1 as libc::c_int) != 0 {
                action_color = 13 as libc::c_int as byte_hack;
                strnfmt(buff.as_mut_ptr(), 20 as libc::c_int as uint_hack,
                        b"(%dgp)\x00" as *const u8 as *const libc::c_char,
                        (*ba_ptr).costs[1 as libc::c_int as usize] as
                            libc::c_int);
            } else {
                action_color = 8 as libc::c_int as byte_hack;
                strnfmt(buff.as_mut_ptr(), 20 as libc::c_int as uint_hack,
                        b"(closed)\x00" as *const u8 as *const libc::c_char);
            }
            strnfmt(tmp_str.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                    b" %c\x00" as *const u8 as *const libc::c_char,
                    (*ba_ptr).letter as libc::c_int);
            c_put_str(11 as libc::c_int as byte_hack,
                      tmp_str.as_mut_ptr() as cptr,
                      21 as libc::c_int + i / 2 as libc::c_int,
                      17 as libc::c_int +
                          30 as libc::c_int * (i % 2 as libc::c_int));
            strnfmt(tmp_str.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                    b") %s %s\x00" as *const u8 as *const libc::c_char,
                    ba_name.offset((*ba_ptr).name as isize),
                    buff.as_mut_ptr());
            c_put_str(action_color, tmp_str.as_mut_ptr() as cptr,
                      21 as libc::c_int + i / 2 as libc::c_int,
                      2 as libc::c_int + 17 as libc::c_int +
                          30 as libc::c_int * (i % 2 as libc::c_int));
        }
        i += 1
    };
}
/* reset timed flags */
unsafe extern "C" fn reset_tim_flags() {
    (*p_ptr).fast = 0 as libc::c_int as s16b; /* Timed -- Fast */
    (*p_ptr).slow = 0 as libc::c_int as s16b; /* Timed -- Slow */
    (*p_ptr).blind = 0 as libc::c_int as s16b; /* Timed -- Blindness */
    (*p_ptr).paralyzed = 0 as libc::c_int as s16b; /* Timed -- Paralysis */
    (*p_ptr).confused = 0 as libc::c_int as s16b; /* Timed -- Confusion */
    (*p_ptr).afraid = 0 as libc::c_int as s16b; /* Timed -- Fear */
    (*p_ptr).image = 0 as libc::c_int as s16b; /* Timed -- Hallucination */
    (*p_ptr).poisoned = 0 as libc::c_int as s16b; /* Timed -- Poisoned */
    (*p_ptr).cut = 0 as libc::c_int as s16b; /* Timed -- Cut */
    (*p_ptr).stun = 0 as libc::c_int as s16b; /* Timed -- Stun */
    (*p_ptr).protevil = 0 as libc::c_int as s16b; /* Timed -- Protection */
    (*p_ptr).protgood = 0 as libc::c_int as s16b; /* Timed -- Protection */
    (*p_ptr).invuln = 0 as libc::c_int as s16b; /* Timed -- Invulnerable */
    (*p_ptr).hero = 0 as libc::c_int as s16b; /* Timed -- Heroism */
    (*p_ptr).shero = 0 as libc::c_int as s16b; /* Timed -- Super Heroism */
    (*p_ptr).shield = 0 as libc::c_int as s16b; /* Timed -- Shield Spell */
    (*p_ptr).blessed = 0 as libc::c_int as s16b; /* Timed -- Blessed */
    (*p_ptr).tim_invis = 0 as libc::c_int as s16b; /* Timed -- Invisibility */
    (*p_ptr).tim_infra = 0 as libc::c_int as s16b; /* Timed -- Infra Vision */
    (*p_ptr).oppose_acid =
        0 as libc::c_int as s16b; /* Timed -- oppose acid */
    (*p_ptr).oppose_elec =
        0 as libc::c_int as s16b; /* Timed -- oppose lightning */
    (*p_ptr).oppose_fire =
        0 as libc::c_int as s16b; /* Timed -- oppose heat */
    (*p_ptr).oppose_cold =
        0 as libc::c_int as s16b; /* Timed -- oppose cold */
    (*p_ptr).oppose_pois =
        0 as libc::c_int as s16b; /* Timed -- oppose poison */
    (*p_ptr).confusing = 0 as libc::c_int as byte_hack;
    /* Touch of Confusion */
}
/*
 * arena commands
 */
unsafe extern "C" fn arena_comm(mut cmd: libc::c_int) {
    let mut tmp_str: [libc::c_char; 80] = [0; 80];
    let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
    let mut name: cptr = 0 as *const libc::c_char;
    match cmd {
        10 => {
            if (*p_ptr).arena_number as libc::c_int == 29 as libc::c_int {
                clear_bldg(5 as libc::c_int, 19 as libc::c_int);
                prt(b"               Arena Victor!\x00" as *const u8 as
                        *const libc::c_char, 5 as libc::c_int,
                    0 as libc::c_int);
                prt(b"Congratulations!  You have defeated all before you.\x00"
                        as *const u8 as *const libc::c_char, 7 as libc::c_int,
                    0 as libc::c_int);
                prt(b"For that, receive the prize: 10,000 gold pieces\x00" as
                        *const u8 as *const libc::c_char, 8 as libc::c_int,
                    0 as libc::c_int);
                prt(b"\x00" as *const u8 as *const libc::c_char,
                    10 as libc::c_int, 0 as libc::c_int);
                prt(b"\x00" as *const u8 as *const libc::c_char,
                    11 as libc::c_int, 0 as libc::c_int);
                (*p_ptr).au += 10000 as libc::c_int;
                msg_print(b"Press the space bar to continue\x00" as *const u8
                              as *const libc::c_char);
                msg_print(0 as cptr);
                (*p_ptr).arena_number += 1
            } else if (*p_ptr).arena_number as libc::c_int > 29 as libc::c_int
             {
                msg_print(b"You enter the arena briefly and bask in your glory.\x00"
                              as *const u8 as *const libc::c_char);
                msg_print(0 as cptr);
            } else {
                (*p_ptr).inside_arena = 1 as libc::c_int as s16b;
                (*p_ptr).exit_bldg = 0 as libc::c_int as bool_;
                reset_tim_flags();
                (*p_ptr).leaving = 1 as libc::c_int as bool_;
                (*p_ptr).oldpx = (*p_ptr).px;
                (*p_ptr).oldpy = (*p_ptr).py;
                leave_bldg = 1 as libc::c_int as bool_
            }
        }
        8 => {
            if (*p_ptr).arena_number as libc::c_int == 29 as libc::c_int {
                msg_print(b"You are victorious. Enter the arena for the ceremony.\x00"
                              as *const u8 as *const libc::c_char);
            } else if (*p_ptr).arena_number as libc::c_int > 29 as libc::c_int
             {
                msg_print(b"You have won against all foes.\x00" as *const u8
                              as *const libc::c_char);
            } else {
                r_ptr =
                    &mut *r_info.offset(*arena_monsters.as_mut_ptr().offset((*p_ptr).arena_number
                                                                                as
                                                                                isize)
                                            as isize) as *mut monster_race;
                name = r_name.offset((*r_ptr).name as isize) as cptr;
                strnfmt(tmp_str.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                        b"Do I hear any challenges against: %s\x00" as
                            *const u8 as *const libc::c_char, name);
                msg_print(tmp_str.as_mut_ptr() as cptr);
                msg_print(0 as cptr);
            }
        }
        9 => {
            /* Save screen */
            screen_save();
            /* Peruse the arena help file */
            show_file(b"arena.txt\x00" as *const u8 as *const libc::c_char,
                      0 as cptr, 0 as libc::c_int, 0 as libc::c_int);
            /* Load screen */
            screen_load();
        }
        _ => { }
    };
}
/*
 * display fruit for dice slots
 */
unsafe extern "C" fn display_fruit(mut row: libc::c_int, mut col: libc::c_int,
                                   mut fruit: libc::c_int) {
    match fruit {
        0 => {
            /* lemon */
            c_put_str(11 as libc::c_int as byte_hack,
                      b"   ####.\x00" as *const u8 as *const libc::c_char,
                      row, col);
            c_put_str(11 as libc::c_int as byte_hack,
                      b"  #    #\x00" as *const u8 as *const libc::c_char,
                      row + 1 as libc::c_int, col);
            c_put_str(11 as libc::c_int as byte_hack,
                      b" #     #\x00" as *const u8 as *const libc::c_char,
                      row + 2 as libc::c_int, col);
            c_put_str(11 as libc::c_int as byte_hack,
                      b"#      #\x00" as *const u8 as *const libc::c_char,
                      row + 3 as libc::c_int, col);
            c_put_str(11 as libc::c_int as byte_hack,
                      b"#      #\x00" as *const u8 as *const libc::c_char,
                      row + 4 as libc::c_int, col);
            c_put_str(11 as libc::c_int as byte_hack,
                      b"#     # \x00" as *const u8 as *const libc::c_char,
                      row + 5 as libc::c_int, col);
            c_put_str(11 as libc::c_int as byte_hack,
                      b"#    #  \x00" as *const u8 as *const libc::c_char,
                      row + 6 as libc::c_int, col);
            c_put_str(11 as libc::c_int as byte_hack,
                      b".####   \x00" as *const u8 as *const libc::c_char,
                      row + 7 as libc::c_int, col);
            prt(b" Lemon  \x00" as *const u8 as *const libc::c_char,
                row + 8 as libc::c_int, col);
        }
        1 => {
            /* orange */
            c_put_str(3 as libc::c_int as byte_hack,
                      b"   ##   \x00" as *const u8 as *const libc::c_char,
                      row, col);
            c_put_str(3 as libc::c_int as byte_hack,
                      b"  #..#  \x00" as *const u8 as *const libc::c_char,
                      row + 1 as libc::c_int, col);
            c_put_str(3 as libc::c_int as byte_hack,
                      b" #....# \x00" as *const u8 as *const libc::c_char,
                      row + 2 as libc::c_int, col);
            c_put_str(3 as libc::c_int as byte_hack,
                      b"#......#\x00" as *const u8 as *const libc::c_char,
                      row + 3 as libc::c_int, col);
            c_put_str(3 as libc::c_int as byte_hack,
                      b"#......#\x00" as *const u8 as *const libc::c_char,
                      row + 4 as libc::c_int, col);
            c_put_str(3 as libc::c_int as byte_hack,
                      b" #....# \x00" as *const u8 as *const libc::c_char,
                      row + 5 as libc::c_int, col);
            c_put_str(3 as libc::c_int as byte_hack,
                      b"  #..#  \x00" as *const u8 as *const libc::c_char,
                      row + 6 as libc::c_int, col);
            c_put_str(3 as libc::c_int as byte_hack,
                      b"   ##   \x00" as *const u8 as *const libc::c_char,
                      row + 7 as libc::c_int, col);
            prt(b" Orange \x00" as *const u8 as *const libc::c_char,
                row + 8 as libc::c_int, col);
        }
        2 => {
            /* sword */
            c_put_str(2 as libc::c_int as byte_hack,
                      b"   /\\   \x00" as *const u8 as *const libc::c_char,
                      row, col);
            c_put_str(2 as libc::c_int as byte_hack,
                      b"   ##   \x00" as *const u8 as *const libc::c_char,
                      row + 1 as libc::c_int, col);
            c_put_str(2 as libc::c_int as byte_hack,
                      b"   ##   \x00" as *const u8 as *const libc::c_char,
                      row + 2 as libc::c_int, col);
            c_put_str(2 as libc::c_int as byte_hack,
                      b"   ##   \x00" as *const u8 as *const libc::c_char,
                      row + 3 as libc::c_int, col);
            c_put_str(2 as libc::c_int as byte_hack,
                      b"   ##   \x00" as *const u8 as *const libc::c_char,
                      row + 4 as libc::c_int, col);
            c_put_str(2 as libc::c_int as byte_hack,
                      b"   ##   \x00" as *const u8 as *const libc::c_char,
                      row + 5 as libc::c_int, col);
            c_put_str(7 as libc::c_int as byte_hack,
                      b" ###### \x00" as *const u8 as *const libc::c_char,
                      row + 6 as libc::c_int, col);
            c_put_str(7 as libc::c_int as byte_hack,
                      b"   ##   \x00" as *const u8 as *const libc::c_char,
                      row + 7 as libc::c_int, col);
            prt(b" Sword  \x00" as *const u8 as *const libc::c_char,
                row + 8 as libc::c_int, col);
        }
        3 => {
            /* shield */
            c_put_str(2 as libc::c_int as byte_hack,
                      b" ###### \x00" as *const u8 as *const libc::c_char,
                      row, col);
            c_put_str(2 as libc::c_int as byte_hack,
                      b"#      #\x00" as *const u8 as *const libc::c_char,
                      row + 1 as libc::c_int, col);
            c_put_str(2 as libc::c_int as byte_hack,
                      b"# ++++ #\x00" as *const u8 as *const libc::c_char,
                      row + 2 as libc::c_int, col);
            c_put_str(2 as libc::c_int as byte_hack,
                      b"# +==+ #\x00" as *const u8 as *const libc::c_char,
                      row + 3 as libc::c_int, col);
            c_put_str(2 as libc::c_int as byte_hack,
                      b"#  ++  #\x00" as *const u8 as *const libc::c_char,
                      row + 4 as libc::c_int, col);
            c_put_str(2 as libc::c_int as byte_hack,
                      b" #    # \x00" as *const u8 as *const libc::c_char,
                      row + 5 as libc::c_int, col);
            c_put_str(2 as libc::c_int as byte_hack,
                      b"  #  #  \x00" as *const u8 as *const libc::c_char,
                      row + 6 as libc::c_int, col);
            c_put_str(2 as libc::c_int as byte_hack,
                      b"   ##   \x00" as *const u8 as *const libc::c_char,
                      row + 7 as libc::c_int, col);
            prt(b" Shield \x00" as *const u8 as *const libc::c_char,
                row + 8 as libc::c_int, col);
        }
        4 => {
            /* plum */
            c_put_str(10 as libc::c_int as byte_hack,
                      b"   ##   \x00" as *const u8 as *const libc::c_char,
                      row, col);
            c_put_str(10 as libc::c_int as byte_hack,
                      b" ###### \x00" as *const u8 as *const libc::c_char,
                      row + 1 as libc::c_int, col);
            c_put_str(10 as libc::c_int as byte_hack,
                      b"########\x00" as *const u8 as *const libc::c_char,
                      row + 2 as libc::c_int, col);
            c_put_str(10 as libc::c_int as byte_hack,
                      b"########\x00" as *const u8 as *const libc::c_char,
                      row + 3 as libc::c_int, col);
            c_put_str(10 as libc::c_int as byte_hack,
                      b"########\x00" as *const u8 as *const libc::c_char,
                      row + 4 as libc::c_int, col);
            c_put_str(10 as libc::c_int as byte_hack,
                      b" ###### \x00" as *const u8 as *const libc::c_char,
                      row + 5 as libc::c_int, col);
            c_put_str(10 as libc::c_int as byte_hack,
                      b"  ####  \x00" as *const u8 as *const libc::c_char,
                      row + 6 as libc::c_int, col);
            c_put_str(10 as libc::c_int as byte_hack,
                      b"   ##   \x00" as *const u8 as *const libc::c_char,
                      row + 7 as libc::c_int, col);
            prt(b"  Plum  \x00" as *const u8 as *const libc::c_char,
                row + 8 as libc::c_int, col);
        }
        5 => {
            /* cherry */
            c_put_str(4 as libc::c_int as byte_hack,
                      b"      ##\x00" as *const u8 as *const libc::c_char,
                      row, col);
            c_put_str(4 as libc::c_int as byte_hack,
                      b"   ###  \x00" as *const u8 as *const libc::c_char,
                      row + 1 as libc::c_int, col);
            c_put_str(4 as libc::c_int as byte_hack,
                      b"  #..#  \x00" as *const u8 as *const libc::c_char,
                      row + 2 as libc::c_int, col);
            c_put_str(4 as libc::c_int as byte_hack,
                      b"  #..#  \x00" as *const u8 as *const libc::c_char,
                      row + 3 as libc::c_int, col);
            c_put_str(4 as libc::c_int as byte_hack,
                      b" ###### \x00" as *const u8 as *const libc::c_char,
                      row + 4 as libc::c_int, col);
            c_put_str(4 as libc::c_int as byte_hack,
                      b"#..##..#\x00" as *const u8 as *const libc::c_char,
                      row + 5 as libc::c_int, col);
            c_put_str(4 as libc::c_int as byte_hack,
                      b"#..##..#\x00" as *const u8 as *const libc::c_char,
                      row + 6 as libc::c_int, col);
            c_put_str(4 as libc::c_int as byte_hack,
                      b" ##  ## \x00" as *const u8 as *const libc::c_char,
                      row + 7 as libc::c_int, col);
            prt(b" Cherry \x00" as *const u8 as *const libc::c_char,
                row + 8 as libc::c_int, col);
        }
        _ => { }
    };
}
/*
 * gamble_comm
 */
unsafe extern "C" fn gamble_comm(mut cmd: libc::c_int) -> bool_ {
    let mut roll1: libc::c_int = 0;
    let mut roll2: libc::c_int = 0;
    let mut roll3: libc::c_int = 0;
    let mut choice: libc::c_int = 0;
    let mut odds: libc::c_int = 0;
    let mut win: libc::c_int = 0;
    let mut wager: s32b = 0;
    let mut maxbet: s32b = 0;
    let mut oldgold: s32b = 0;
    static mut fruit: [*const libc::c_char; 6] =
        [b"Lemon\x00" as *const u8 as *const libc::c_char,
         b"Orange\x00" as *const u8 as *const libc::c_char,
         b"Sword\x00" as *const u8 as *const libc::c_char,
         b"Shield\x00" as *const u8 as *const libc::c_char,
         b"Plum\x00" as *const u8 as *const libc::c_char,
         b"Cherry\x00" as *const u8 as *const libc::c_char];
    let mut out_val: [libc::c_char; 160] = [0; 160];
    let mut tmp_str: [libc::c_char; 80] = [0; 80];
    let mut again: libc::c_char = 0;
    let mut p: cptr = 0 as *const libc::c_char;
    screen_save();
    if cmd == 13 as libc::c_int {
        /* Peruse the gambling help file */
        show_file(b"gambling.txt\x00" as *const u8 as *const libc::c_char,
                  0 as cptr, 0 as libc::c_int, 0 as libc::c_int);
    } else {
        clear_bldg(5 as libc::c_int, 23 as libc::c_int);
        /* Set maximum bet */
        if ((*p_ptr).lev as libc::c_int) < 10 as libc::c_int {
            maxbet = (*p_ptr).lev as libc::c_int * 100 as libc::c_int
        } else { maxbet = (*p_ptr).lev as libc::c_int * 1000 as libc::c_int }
        /* Get the wager */
        strcpy(out_val.as_mut_ptr(),
               b"\x00" as *const u8 as *const libc::c_char);
        strnfmt(tmp_str.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"Your wager (1-%ld) ? \x00" as *const u8 as
                    *const libc::c_char, maxbet);
        get_string(tmp_str.as_mut_ptr() as cptr, out_val.as_mut_ptr(),
                   32 as libc::c_int);
        /* Strip spaces */
        p = out_val.as_mut_ptr() as cptr;
        while *p as libc::c_int == ' ' as i32 { p = p.offset(1) }
        wager = atol(p) as s32b;
        if wager > (*p_ptr).au {
            msg_print(b"Hey! You don\'t have the gold - get out of here!\x00"
                          as *const u8 as *const libc::c_char);
            msg_print(0 as cptr);
            screen_load();
            return 0 as libc::c_int as bool_
        } else {
            if wager > maxbet {
                msg_format(b"I\'ll take $%ld of that. Keep the rest.\x00" as
                               *const u8 as *const libc::c_char, maxbet);
                wager = maxbet
            } else if wager < 1 as libc::c_int {
                msg_print(b"Ok, we\'ll start with $1.\x00" as *const u8 as
                              *const libc::c_char);
                wager = 1 as libc::c_int
            }
        }
        msg_print(0 as cptr);
        win = 0 as libc::c_int;
        odds = 0 as libc::c_int;
        oldgold = (*p_ptr).au;
        strnfmt(tmp_str.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"Gold before game: %9ld\x00" as *const u8 as
                    *const libc::c_char, oldgold);
        prt(tmp_str.as_mut_ptr() as cptr, 20 as libc::c_int,
            2 as libc::c_int);
        strnfmt(tmp_str.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"Current Wager:    %9ld\x00" as *const u8 as
                    *const libc::c_char, wager);
        prt(tmp_str.as_mut_ptr() as cptr, 21 as libc::c_int,
            2 as libc::c_int);
        loop  {
            match cmd {
                12 => {
                    /* Game of In-Between */
                    c_put_str(5 as libc::c_int as byte_hack,
                              b"In Between\x00" as *const u8 as
                                  *const libc::c_char, 5 as libc::c_int,
                              2 as libc::c_int);
                    odds = 3 as libc::c_int;
                    win = 0 as libc::c_int;
                    roll1 = Rand_div(10 as libc::c_int) + 1 as libc::c_int;
                    roll2 = Rand_div(10 as libc::c_int) + 1 as libc::c_int;
                    choice = Rand_div(10 as libc::c_int) + 1 as libc::c_int;
                    strnfmt(tmp_str.as_mut_ptr(),
                            80 as libc::c_int as uint_hack,
                            b"Black die: %d       Black Die: %d\x00" as
                                *const u8 as *const libc::c_char, roll1,
                            roll2);
                    prt(tmp_str.as_mut_ptr() as cptr, 8 as libc::c_int,
                        3 as libc::c_int);
                    strnfmt(tmp_str.as_mut_ptr(),
                            80 as libc::c_int as uint_hack,
                            b"Red die: %d\x00" as *const u8 as
                                *const libc::c_char, choice);
                    prt(tmp_str.as_mut_ptr() as cptr, 11 as libc::c_int,
                        14 as libc::c_int);
                    if choice > roll1 && choice < roll2 ||
                           choice < roll1 && choice > roll2 {
                        win = 1 as libc::c_int
                    }
                }
                14 => {
                    /* Game of Craps */
                    c_put_str(5 as libc::c_int as byte_hack,
                              b"Craps\x00" as *const u8 as
                                  *const libc::c_char, 5 as libc::c_int,
                              2 as libc::c_int);
                    win = 3 as libc::c_int;
                    odds = 1 as libc::c_int;
                    roll1 = Rand_div(6 as libc::c_int) + 1 as libc::c_int;
                    roll2 = Rand_div(6 as libc::c_int) + 1 as libc::c_int;
                    roll3 = roll1 + roll2;
                    choice = roll3;
                    strnfmt(tmp_str.as_mut_ptr(),
                            80 as libc::c_int as uint_hack,
                            b"First roll: %d %d    Total: %d\x00" as *const u8
                                as *const libc::c_char, roll1, roll2, roll3);
                    prt(tmp_str.as_mut_ptr() as cptr, 7 as libc::c_int,
                        5 as libc::c_int);
                    if roll3 == 7 as libc::c_int || roll3 == 11 as libc::c_int
                       {
                        win = 1 as libc::c_int
                    } else if roll3 == 2 as libc::c_int ||
                                  roll3 == 3 as libc::c_int ||
                                  roll3 == 12 as libc::c_int {
                        win = 0 as libc::c_int
                    } else {
                        loop  {
                            msg_print(b"Hit any key to roll again\x00" as
                                          *const u8 as *const libc::c_char);
                            msg_print(0 as cptr);
                            roll1 =
                                Rand_div(6 as libc::c_int) + 1 as libc::c_int;
                            roll2 =
                                Rand_div(6 as libc::c_int) + 1 as libc::c_int;
                            roll3 = roll1 + roll2;
                            strnfmt(tmp_str.as_mut_ptr(),
                                    80 as libc::c_int as uint_hack,
                                    b"Roll result: %d %d   Total:     %d\x00"
                                        as *const u8 as *const libc::c_char,
                                    roll1, roll2, roll3);
                            prt(tmp_str.as_mut_ptr() as cptr,
                                8 as libc::c_int, 5 as libc::c_int);
                            if roll3 == choice {
                                win = 1 as libc::c_int
                            } else if roll3 == 7 as libc::c_int {
                                win = 0 as libc::c_int
                            }
                            if !(win != 1 as libc::c_int &&
                                     win != 0 as libc::c_int) {
                                break ;
                            }
                        }
                    }
                }
                15 => {
                    /* Spin the Wheel Game */
                    win = 0 as libc::c_int;
                    odds = 10 as libc::c_int;
                    c_put_str(5 as libc::c_int as byte_hack,
                              b"Wheel\x00" as *const u8 as
                                  *const libc::c_char, 5 as libc::c_int,
                              2 as libc::c_int);
                    prt(b"0  1  2  3  4  5  6  7  8  9\x00" as *const u8 as
                            *const libc::c_char, 7 as libc::c_int,
                        5 as libc::c_int);
                    prt(b"--------------------------------\x00" as *const u8
                            as *const libc::c_char, 8 as libc::c_int,
                        3 as libc::c_int);
                    strcpy(out_val.as_mut_ptr(),
                           b"\x00" as *const u8 as *const libc::c_char);
                    get_string(b"Pick a number (1-9): \x00" as *const u8 as
                                   *const libc::c_char, out_val.as_mut_ptr(),
                               32 as libc::c_int);
                    p = out_val.as_mut_ptr() as cptr;
                    while *p as libc::c_int == ' ' as i32 { p = p.offset(1) }
                    choice = atol(p) as libc::c_int;
                    if choice < 0 as libc::c_int {
                        msg_print(b"I\'ll put you down for 0.\x00" as
                                      *const u8 as *const libc::c_char);
                        choice = 0 as libc::c_int
                    } else if choice > 9 as libc::c_int {
                        msg_print(b"Ok, I\'ll put you down for 9.\x00" as
                                      *const u8 as *const libc::c_char);
                        choice = 9 as libc::c_int
                    }
                    msg_print(0 as cptr);
                    roll1 =
                        Rand_div(10 as libc::c_int) + 1 as libc::c_int -
                            1 as libc::c_int;
                    strnfmt(tmp_str.as_mut_ptr(),
                            80 as libc::c_int as uint_hack,
                            b"The wheel spins to a stop and the winner is %d\x00"
                                as *const u8 as *const libc::c_char, roll1);
                    prt(tmp_str.as_mut_ptr() as cptr, 13 as libc::c_int,
                        3 as libc::c_int);
                    prt(b"\x00" as *const u8 as *const libc::c_char,
                        9 as libc::c_int, 0 as libc::c_int);
                    prt(b"*\x00" as *const u8 as *const libc::c_char,
                        9 as libc::c_int,
                        3 as libc::c_int * roll1 + 5 as libc::c_int);
                    if roll1 == choice { win = 1 as libc::c_int }
                }
                16 => {
                    /* The Dice Slots */
                    c_put_str(5 as libc::c_int as byte_hack,
                              b"Dice Slots\x00" as *const u8 as
                                  *const libc::c_char, 5 as libc::c_int,
                              2 as libc::c_int);
                    win = 0 as libc::c_int;
                    roll1 = Rand_div(6 as libc::c_int) + 1 as libc::c_int;
                    roll2 = Rand_div(6 as libc::c_int) + 1 as libc::c_int;
                    choice = Rand_div(6 as libc::c_int) + 1 as libc::c_int;
                    strnfmt(tmp_str.as_mut_ptr(),
                            80 as libc::c_int as uint_hack,
                            b"%s %s %s\x00" as *const u8 as
                                *const libc::c_char,
                            fruit[(roll1 - 1 as libc::c_int) as usize],
                            fruit[(roll2 - 1 as libc::c_int) as usize],
                            fruit[(choice - 1 as libc::c_int) as usize]);
                    prt(tmp_str.as_mut_ptr() as cptr, 15 as libc::c_int,
                        37 as libc::c_int);
                    prt(b"/--------------------------\\\x00" as *const u8 as
                            *const libc::c_char, 7 as libc::c_int,
                        2 as libc::c_int);
                    prt(b"\\--------------------------/\x00" as *const u8 as
                            *const libc::c_char, 17 as libc::c_int,
                        2 as libc::c_int);
                    display_fruit(8 as libc::c_int, 3 as libc::c_int,
                                  roll1 - 1 as libc::c_int);
                    display_fruit(8 as libc::c_int, 12 as libc::c_int,
                                  roll2 - 1 as libc::c_int);
                    display_fruit(8 as libc::c_int, 21 as libc::c_int,
                                  choice - 1 as libc::c_int);
                    if roll1 == roll2 && roll2 == choice {
                        win = 1 as libc::c_int;
                        if roll1 == 1 as libc::c_int {
                            odds = 4 as libc::c_int
                        } else if roll1 == 2 as libc::c_int {
                            odds = 6 as libc::c_int
                        } else { odds = roll1 * roll1 }
                    } else if roll1 == 6 as libc::c_int &&
                                  roll2 == 6 as libc::c_int {
                        win = 1 as libc::c_int;
                        odds = choice + 1 as libc::c_int
                    }
                }
                _ => { }
            }
            if win != 0 {
                prt(b"YOU WON\x00" as *const u8 as *const libc::c_char,
                    16 as libc::c_int, 37 as libc::c_int);
                (*p_ptr).au = (*p_ptr).au + odds * wager;
                strnfmt(tmp_str.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                        b"Payoff: %d\x00" as *const u8 as *const libc::c_char,
                        odds);
                prt(tmp_str.as_mut_ptr() as cptr, 17 as libc::c_int,
                    37 as libc::c_int);
            } else {
                prt(b"You Lost\x00" as *const u8 as *const libc::c_char,
                    16 as libc::c_int, 37 as libc::c_int);
                (*p_ptr).au = (*p_ptr).au - wager;
                prt(b"\x00" as *const u8 as *const libc::c_char,
                    17 as libc::c_int, 37 as libc::c_int);
            }
            strnfmt(tmp_str.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                    b"Current Gold:     %9ld\x00" as *const u8 as
                        *const libc::c_char, (*p_ptr).au);
            prt(tmp_str.as_mut_ptr() as cptr, 22 as libc::c_int,
                2 as libc::c_int);
            prt(b"Again(Y/N)?\x00" as *const u8 as *const libc::c_char,
                18 as libc::c_int, 37 as libc::c_int);
            move_cursor(18 as libc::c_int, 49 as libc::c_int);
            again = inkey();
            if wager > (*p_ptr).au {
                msg_print(b"Hey! You don\'t have the gold - get out of here!\x00"
                              as *const u8 as *const libc::c_char);
                msg_print(0 as cptr);
                screen_load();
                return 0 as libc::c_int as bool_
                /*				strnfmt(tmp_str, 80, "Current Wager:    %9ld",wager);
								prt(tmp_str, 17, 2); */
            }
            if !(again as libc::c_int == 'y' as i32 ||
                     again as libc::c_int == 'Y' as i32) {
                break ;
            }
        }
        prt(b"\x00" as *const u8 as *const libc::c_char, 18 as libc::c_int,
            37 as libc::c_int);
        if (*p_ptr).au >= oldgold {
            msg_print(b"You came out a winner! We\'ll win next time, I\'m sure.\x00"
                          as *const u8 as *const libc::c_char);
        } else {
            msg_print(b"You lost gold! Haha, better head home.\x00" as
                          *const u8 as *const libc::c_char);
        }
        msg_print(0 as cptr);
    }
    screen_load();
    return 1 as libc::c_int as bool_;
}
/*
 * inn commands
 * Note that resting for the night was a perfect way to avoid player
 * ghosts in the town *if* you could only make it to the inn in time (-:
 * Now that the ghosts are temporarily disabled in 2.8.X, this function
 * will not be that useful.  I will keep it in the hopes the player
 * ghost code does become a reality again. Does help to avoid filthy urchins.
 * Resting at night is also a quick way to restock stores -KMW- 
 */
unsafe extern "C" fn inn_comm(mut cmd: libc::c_int) -> bool_ {
    let mut vampire: bool_ = 0;
    /* Extract race info */
    vampire =
        (((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
              (*spp_ptr).flags1) as libc::c_long & 0x200 as libc::c_long != 0
             ||
             (*p_ptr).mimic_form as libc::c_int ==
                 resolve_mimic_name(b"Vampire\x00" as *const u8 as
                                        *const libc::c_char)) as libc::c_int
            as bool_;
    match cmd {
        18 => {
            /* Buy food & drink */
            if vampire == 0 {
                msg_print(b"The barkeep gives you some gruel and a beer.\x00"
                              as *const u8 as *const libc::c_char);
                msg_print(0 as cptr);
                set_food(15000 as libc::c_int - 1 as libc::c_int);
            } else {
                msg_print(b"You\'re a vampire and I don\'t have any food for you!\x00"
                              as *const u8 as *const libc::c_char);
            }
        }
        17 => {
            /* Rest for the night */
            let mut nighttime: bool_ = 0;
            /* Extract the current time */
            nighttime =
                (bst(11520 as libc::c_int / 24 as libc::c_int, turn) <
                     6 as libc::c_int ||
                     bst(11520 as libc::c_int / 24 as libc::c_int, turn) >=
                         18 as libc::c_int) as libc::c_int as bool_;
            /* Normal races rest at night */
            if vampire == 0 && nighttime == 0 {
                msg_print(b"The rooms are available only at night.\x00" as
                              *const u8 as *const libc::c_char);
                msg_print(0 as cptr);
                return 0 as libc::c_int as bool_
            }
            /* Vampires rest during daytime */
            if vampire as libc::c_int != 0 && nighttime as libc::c_int != 0 {
                msg_print(b"The rooms are available only during daylight for your kind.\x00"
                              as *const u8 as *const libc::c_char);
                msg_print(0 as cptr);
                return 0 as libc::c_int as bool_
            }
            /* Must cure HP draining status first */
            if (*p_ptr).poisoned as libc::c_int > 0 as libc::c_int ||
                   (*p_ptr).cut as libc::c_int > 0 as libc::c_int {
                msg_print(b"You need a healer, not a room.\x00" as *const u8
                              as *const libc::c_char);
                msg_print(0 as cptr);
                msg_print(b"Sorry, but I don\'t want anyone dying in here.\x00"
                              as *const u8 as *const libc::c_char);
                return 0 as libc::c_int as bool_
            }
            /* Let the time pass XXX XXX XXX */
            if vampire != 0 {
                /* Wait for sunset */
                while bst(11520 as libc::c_int / 24 as libc::c_int, turn) >=
                          6 as libc::c_int &&
                          bst(11520 as libc::c_int / 24 as libc::c_int, turn)
                              < 18 as libc::c_int {
                    turn =
                        (turn as libc::c_long +
                             10 as libc::c_long *
                                 (11520 as libc::c_int / 24 as libc::c_int /
                                      60 as libc::c_int) as libc::c_long) as
                            s32b
                }
            } else {
                /* Wait for sunrise */
                while bst(11520 as libc::c_int / 24 as libc::c_int, turn) <
                          6 as libc::c_int ||
                          bst(11520 as libc::c_int / 24 as libc::c_int, turn)
                              >= 18 as libc::c_int {
                    turn =
                        (turn as libc::c_long +
                             10 as libc::c_long *
                                 (11520 as libc::c_int / 24 as libc::c_int /
                                      60 as libc::c_int) as libc::c_long) as
                            s32b
                }
            }
            /* Regen */
            (*p_ptr).chp = (*p_ptr).mhp;
            (*p_ptr).csp = (*p_ptr).msp;
            /* Restore status */
            set_blind(0 as libc::c_int);
            set_confused(0 as libc::c_int);
            (*p_ptr).stun = 0 as libc::c_int as s16b;
            /* Message */
            if vampire != 0 {
                msg_print(b"You awake refreshed for the new night.\x00" as
                              *const u8 as *const libc::c_char);
            } else {
                msg_print(b"You awake refreshed for the new day.\x00" as
                              *const u8 as *const libc::c_char);
            }
            /* Dungeon stuff */
            (*p_ptr).leaving = 1 as libc::c_int as bool_;
            (*p_ptr).oldpx = (*p_ptr).px;
            (*p_ptr).oldpy = (*p_ptr).py;
            /* Select new bounties. */
            select_bounties();
        }
        19 => {
            /* Listen for rumors */
            let mut rumor: [libc::c_char; 80] = [0; 80];
            get_rnd_line(b"rumors.txt\x00" as *const u8 as *const libc::c_char
                             as *mut libc::c_char, rumor.as_mut_ptr());
            msg_format(b"%s\x00" as *const u8 as *const libc::c_char,
                       rumor.as_mut_ptr());
            msg_print(0 as cptr);
        }
        _ => { }
    }
    return 1 as libc::c_int as bool_;
}
/*
 * Display quest information
 */
unsafe extern "C" fn get_questinfo(mut questnum: libc::c_int) {
    let mut i: libc::c_int = 0;
    /* Print the quest info */
    prt(format(b"Quest Information (Danger level: %d)\x00" as *const u8 as
                   *const libc::c_char,
               (*quest.offset(questnum as isize)).level as libc::c_int) as
            cptr, 5 as libc::c_int, 0 as libc::c_int);
    prt((*quest.offset(questnum as isize)).name.as_mut_ptr() as cptr,
        7 as libc::c_int, 0 as libc::c_int);
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int &&
              (*quest.offset(questnum as
                                 isize)).desc[i as
                                                  usize][0 as libc::c_int as
                                                             usize] as
                  libc::c_int != '\u{0}' as i32 {
        c_put_str(11 as libc::c_int as byte_hack,
                  (*quest.offset(questnum as
                                     isize)).desc[i as usize].as_mut_ptr() as
                      cptr, i + 8 as libc::c_int, 0 as libc::c_int);
        i += 1
    };
}
/*
 * Request a quest from the Lord.
 */
unsafe extern "C" fn castle_quest(mut y: libc::c_int, mut x: libc::c_int)
 -> bool_ {
    let mut plot: libc::c_int = 0 as libc::c_int;
    let mut q_ptr: *mut quest_type = 0 as *mut quest_type;
    clear_bldg(7 as libc::c_int, 18 as libc::c_int);
    /* Current plot of the building */
    plot = (*cave[y as usize].offset(x as isize)).special as libc::c_int;
    /* Is there a quest available at the building? */
    if plot == 0 || plots[plot as usize] as libc::c_int == 0 as libc::c_int {
        put_str(b"I don\'t have a quest for you at the moment.\x00" as
                    *const u8 as *const libc::c_char, 8 as libc::c_int,
                0 as libc::c_int);
        return 0 as libc::c_int as bool_
    }
    q_ptr =
        &mut *quest.offset(*plots.as_mut_ptr().offset(plot as isize) as isize)
            as *mut quest_type;
    /* Quest is completed */
    if (*q_ptr).status as libc::c_int == 2 as libc::c_int {
        /* Rewarded quest */
        (*q_ptr).status = 5 as libc::c_int as s16b;
        process_hooks(9 as libc::c_int,
                      b"(d)\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      plots[plot as usize] as libc::c_int);
        return 1 as libc::c_int as bool_
    } else {
        /* Quest is still unfinished */
        if (*q_ptr).status as libc::c_int == 1 as libc::c_int {
            put_str(b"You have not completed your current quest yet!\x00" as
                        *const u8 as *const libc::c_char, 8 as libc::c_int,
                    0 as libc::c_int);
            put_str(b"Use CTRL-Q to check the status of your quest.\x00" as
                        *const u8 as *const libc::c_char, 9 as libc::c_int,
                    0 as libc::c_int);
            put_str(b"Return when you have completed your quest.\x00" as
                        *const u8 as *const libc::c_char, 12 as libc::c_int,
                    0 as libc::c_int);
            return 0 as libc::c_int as bool_
        } else {
            /* Failed quest */
            if (*q_ptr).status as libc::c_int == 4 as libc::c_int {
                /* Mark quest as done (but failed) */
                (*q_ptr).status = 6 as libc::c_int as s16b;
                process_hooks(10 as libc::c_int,
                              b"(d)\x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char,
                              plots[plot as usize] as libc::c_int);
                return 0 as libc::c_int as bool_
            } else {
                /* No quest yet */
                if (*q_ptr).status as libc::c_int == 0 as libc::c_int {
                    if process_hooks(13 as libc::c_int,
                                     b"(d)\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char,
                                     plots[plot as usize] as libc::c_int) != 0
                       {
                        return 0 as libc::c_int as bool_
                    }
                    (*q_ptr).status = 1 as libc::c_int as s16b;
                    /* Assign a new quest */
                    get_questinfo(plots[plot as usize] as libc::c_int);
                    /* Add the hooks */
                    if (*quest.offset(plots[plot as usize] as isize)).type_0
                           as libc::c_int == 0 as libc::c_int {
                        (*quest.offset(plots[plot as usize] as
                                           isize)).init.expect("non-null function pointer")(plots[plot
                                                                                                      as
                                                                                                      usize]
                                                                                                as
                                                                                                libc::c_int);
                    }
                    return 1 as libc::c_int as bool_
                }
            }
        }
    }
    return 0 as libc::c_int as bool_;
}
/*
 * Displaying town history -KMW-
 */
unsafe extern "C" fn town_history() {
    /* Save screen */
    screen_save();
    /* Peruse the building help file */
    show_file(b"bldg.txt\x00" as *const u8 as *const libc::c_char, 0 as cptr,
              0 as libc::c_int, 0 as libc::c_int);
    /* Load screen */
    screen_load();
}
/*
 * compare_weapon_aux2 -KMW-
 */
unsafe extern "C" fn compare_weapon_aux2(mut o_ptr: *mut object_type,
                                         mut numblows: libc::c_int,
                                         mut r: libc::c_int,
                                         mut c: libc::c_int,
                                         mut mult: libc::c_int,
                                         mut attr: *mut libc::c_char,
                                         mut f1: u32b, mut f2: u32b,
                                         mut f3: u32b, mut color: byte_hack) {
    let mut tmp_str: [libc::c_char; 80] = [0; 80];
    c_put_str(color, attr as cptr, r, c);
    strnfmt(tmp_str.as_mut_ptr(), 80 as libc::c_int as uint_hack,
            b"Attack: %d-%d damage\x00" as *const u8 as *const libc::c_char,
            numblows *
                ((*o_ptr).dd as libc::c_int * mult +
                     (*o_ptr).to_d as libc::c_int),
            numblows *
                ((*o_ptr).ds as libc::c_int * (*o_ptr).dd as libc::c_int *
                     mult + (*o_ptr).to_d as libc::c_int));
    put_str(tmp_str.as_mut_ptr() as cptr, r, c + 8 as libc::c_int);
    r += 1;
}
/*
 * compare_weapon_aux1 -KMW-
 */
unsafe extern "C" fn compare_weapon_aux1(mut o_ptr: *mut object_type,
                                         mut col: libc::c_int,
                                         mut r: libc::c_int) {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    if f1 as libc::c_long & 0x10000 as libc::c_long != 0 {
        let fresh0 = r;
        r = r + 1;
        compare_weapon_aux2(o_ptr, (*p_ptr).num_blow as libc::c_int, fresh0,
                            col, 2 as libc::c_int,
                            b"Animals:\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char, f1,
                            f2, f3, 11 as libc::c_int as byte_hack);
    }
    if f1 as libc::c_long & 0x20000 as libc::c_long != 0 {
        let fresh1 = r;
        r = r + 1;
        compare_weapon_aux2(o_ptr, (*p_ptr).num_blow as libc::c_int, fresh1,
                            col, 2 as libc::c_int,
                            b"Evil:\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char, f1, f2, f3,
                            11 as libc::c_int as byte_hack);
    }
    if f1 as libc::c_long & 0x40000 as libc::c_long != 0 {
        let fresh2 = r;
        r = r + 1;
        compare_weapon_aux2(o_ptr, (*p_ptr).num_blow as libc::c_int, fresh2,
                            col, 3 as libc::c_int,
                            b"Undead:\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char, f1, f2, f3,
                            11 as libc::c_int as byte_hack);
    }
    if f1 as libc::c_long & 0x80000 as libc::c_long != 0 {
        let fresh3 = r;
        r = r + 1;
        compare_weapon_aux2(o_ptr, (*p_ptr).num_blow as libc::c_int, fresh3,
                            col, 3 as libc::c_int,
                            b"Demons:\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char, f1, f2, f3,
                            11 as libc::c_int as byte_hack);
    }
    if f1 as libc::c_long & 0x100000 as libc::c_long != 0 {
        let fresh4 = r;
        r = r + 1;
        compare_weapon_aux2(o_ptr, (*p_ptr).num_blow as libc::c_int, fresh4,
                            col, 3 as libc::c_int,
                            b"Orcs:\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char, f1, f2, f3,
                            11 as libc::c_int as byte_hack);
    }
    if f1 as libc::c_long & 0x200000 as libc::c_long != 0 {
        let fresh5 = r;
        r = r + 1;
        compare_weapon_aux2(o_ptr, (*p_ptr).num_blow as libc::c_int, fresh5,
                            col, 3 as libc::c_int,
                            b"Trolls:\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char, f1, f2, f3,
                            11 as libc::c_int as byte_hack);
    }
    if f1 as libc::c_long & 0x400000 as libc::c_long != 0 {
        let fresh6 = r;
        r = r + 1;
        compare_weapon_aux2(o_ptr, (*p_ptr).num_blow as libc::c_int, fresh6,
                            col, 3 as libc::c_int,
                            b"Giants:\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char, f1, f2, f3,
                            11 as libc::c_int as byte_hack);
    }
    if f1 as libc::c_long & 0x800000 as libc::c_long != 0 {
        let fresh7 = r;
        r = r + 1;
        compare_weapon_aux2(o_ptr, (*p_ptr).num_blow as libc::c_int, fresh7,
                            col, 3 as libc::c_int,
                            b"Dragons:\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char, f1,
                            f2, f3, 11 as libc::c_int as byte_hack);
    }
    if f1 as libc::c_long & 0x1000000 as libc::c_long != 0 {
        let fresh8 = r;
        r = r + 1;
        compare_weapon_aux2(o_ptr, (*p_ptr).num_blow as libc::c_int, fresh8,
                            col, 5 as libc::c_int,
                            b"Dragons:\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char, f1,
                            f2, f3, 11 as libc::c_int as byte_hack);
    }
    if f1 as libc::c_long & 0x10000000 as libc::c_long != 0 {
        let fresh9 = r;
        r = r + 1;
        compare_weapon_aux2(o_ptr, (*p_ptr).num_blow as libc::c_int, fresh9,
                            col, 3 as libc::c_int,
                            b"Acid:\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char, f1, f2, f3,
                            4 as libc::c_int as byte_hack);
    }
    if f1 as libc::c_long & 0x20000000 as libc::c_long != 0 {
        let fresh10 = r;
        r = r + 1;
        compare_weapon_aux2(o_ptr, (*p_ptr).num_blow as libc::c_int, fresh10,
                            col, 3 as libc::c_int,
                            b"Elec:\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char, f1, f2, f3,
                            4 as libc::c_int as byte_hack);
    }
    if f1 as libc::c_long & 0x40000000 as libc::c_long != 0 {
        let fresh11 = r;
        r = r + 1;
        compare_weapon_aux2(o_ptr, (*p_ptr).num_blow as libc::c_int, fresh11,
                            col, 3 as libc::c_int,
                            b"Fire:\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char, f1, f2, f3,
                            4 as libc::c_int as byte_hack);
    }
    if f1 as libc::c_long & 0x80000000 as libc::c_long != 0 {
        let fresh12 = r;
        r = r + 1;
        compare_weapon_aux2(o_ptr, (*p_ptr).num_blow as libc::c_int, fresh12,
                            col, 3 as libc::c_int,
                            b"Cold:\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char, f1, f2, f3,
                            4 as libc::c_int as byte_hack);
    }
    if f1 as libc::c_long & 0x8000000 as libc::c_long != 0 {
        let fresh13 = r;
        r = r + 1;
        compare_weapon_aux2(o_ptr, (*p_ptr).num_blow as libc::c_int, fresh13,
                            col, 3 as libc::c_int,
                            b"Poison:\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char, f1, f2, f3,
                            4 as libc::c_int as byte_hack);
    };
}
/*
 * list_weapon -KMW-
 */
unsafe extern "C" fn list_weapon(mut o_ptr: *mut object_type,
                                 mut row: libc::c_int, mut col: libc::c_int) {
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut tmp_str: [libc::c_char; 80] = [0; 80];
    object_desc(o_name.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                0 as libc::c_int);
    c_put_str(11 as libc::c_int as byte_hack, o_name.as_mut_ptr() as cptr,
              row, col);
    strnfmt(tmp_str.as_mut_ptr(), 80 as libc::c_int as uint_hack,
            b"To Hit: %d   To Damage: %d\x00" as *const u8 as
                *const libc::c_char, (*o_ptr).to_h as libc::c_int,
            (*o_ptr).to_d as libc::c_int);
    put_str(tmp_str.as_mut_ptr() as cptr, row + 1 as libc::c_int, col);
    strnfmt(tmp_str.as_mut_ptr(), 80 as libc::c_int as uint_hack,
            b"Dice: %d   Sides: %d\x00" as *const u8 as *const libc::c_char,
            (*o_ptr).dd as libc::c_int, (*o_ptr).ds as libc::c_int);
    put_str(tmp_str.as_mut_ptr() as cptr, row + 2 as libc::c_int, col);
    strnfmt(tmp_str.as_mut_ptr(), 80 as libc::c_int as uint_hack,
            b"Number of Blows: %d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).num_blow as libc::c_int);
    put_str(tmp_str.as_mut_ptr() as cptr, row + 3 as libc::c_int, col);
    c_put_str(11 as libc::c_int as byte_hack,
              b"Possible Damage:\x00" as *const u8 as *const libc::c_char,
              row + 5 as libc::c_int, col);
    strnfmt(tmp_str.as_mut_ptr(), 80 as libc::c_int as uint_hack,
            b"One Strike: %d-%d damage\x00" as *const u8 as
                *const libc::c_char,
            (*o_ptr).dd as libc::c_int + (*o_ptr).to_d as libc::c_int,
            (*o_ptr).ds as libc::c_int * (*o_ptr).dd as libc::c_int +
                (*o_ptr).to_d as libc::c_int);
    put_str(tmp_str.as_mut_ptr() as cptr, row + 6 as libc::c_int,
            col + 1 as libc::c_int);
    strnfmt(tmp_str.as_mut_ptr(), 80 as libc::c_int as uint_hack,
            b"One Attack: %d-%d damage\x00" as *const u8 as
                *const libc::c_char,
            (*p_ptr).num_blow as libc::c_int *
                ((*o_ptr).dd as libc::c_int + (*o_ptr).to_d as libc::c_int),
            (*p_ptr).num_blow as libc::c_int *
                ((*o_ptr).ds as libc::c_int * (*o_ptr).dd as libc::c_int +
                     (*o_ptr).to_d as libc::c_int));
    put_str(tmp_str.as_mut_ptr() as cptr, row + 7 as libc::c_int,
            col + 1 as libc::c_int);
}
/*
 * Select melee weapons
 */
unsafe extern "C" fn item_tester_hook_melee_weapon(mut o_ptr:
                                                       *mut object_type)
 -> bool_ {
    return (wield_slot(o_ptr) as libc::c_int == 24 as libc::c_int) as
               libc::c_int as bool_;
}
/*
 * compare_weapons -KMW-
 */
unsafe extern "C" fn compare_weapons() -> bool_ {
    let mut item: libc::c_int = 0;
    let mut item2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut o1_ptr: *mut object_type = 0 as *mut object_type;
    let mut o2_ptr: *mut object_type = 0 as *mut object_type;
    let mut orig_ptr: *mut object_type = 0 as *mut object_type;
    let mut i_ptr: *mut object_type = 0 as *mut object_type;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    clear_bldg(6 as libc::c_int, 18 as libc::c_int);
    o1_ptr = 0 as *mut object_type;
    o2_ptr = 0 as *mut object_type;
    i_ptr = 0 as *mut object_type;
    /* Store copy of original wielded weapon in pack slot */
    i_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(24 as libc::c_int as
                                                         isize) as
            *mut object_type;
    orig_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(23 as libc::c_int as
                                                         isize) as
            *mut object_type;
    object_copy(orig_ptr, i_ptr);
    i = 6 as libc::c_int;
    /* Get first weapon */
	/* Restrict choices to meele weapons */
    item_tester_hook =
        Some(item_tester_hook_melee_weapon as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    q =
        b"What is your first melee weapon? \x00" as *const u8 as
            *const libc::c_char;
    s =
        b"You have nothing to compare.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s, 0x1 as libc::c_int | 0x2 as libc::c_int) == 0
       {
        object_wipe(orig_ptr);
        return 0 as libc::c_int as bool_
    }
    /* Get the item (in the pack) */
    if item >= 0 as libc::c_int {
        o1_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(item as isize) as
                *mut object_type
    }
    /* Get second weapon */
	/* Restrict choices to melee weapons */
    item_tester_hook =
        Some(item_tester_hook_melee_weapon as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    q =
        b"What is your second melee weapon? \x00" as *const u8 as
            *const libc::c_char;
    s =
        b"You have nothing to compare.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item2, q, s, 0x1 as libc::c_int | 0x2 as libc::c_int) ==
           0 {
        object_wipe(orig_ptr);
        return 0 as libc::c_int as bool_
    }
    /* Get the item (in the pack) */
    if item2 >= 0 as libc::c_int {
        o2_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(item2 as isize) as
                *mut object_type
    }
    put_str(b"Based on your current abilities, here is what your weapons will do\x00"
                as *const u8 as *const libc::c_char, 4 as libc::c_int,
            2 as libc::c_int);
    i_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(24 as libc::c_int as
                                                         isize) as
            *mut object_type;
    object_copy(i_ptr, o1_ptr);
    calc_bonuses(1 as libc::c_int as bool_);
    list_weapon(o1_ptr, i, 2 as libc::c_int);
    compare_weapon_aux1(o1_ptr, 2 as libc::c_int, i + 8 as libc::c_int);
    i_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(24 as libc::c_int as
                                                         isize) as
            *mut object_type;
    if item2 == 24 as libc::c_int {
        object_copy(i_ptr, orig_ptr);
    } else { object_copy(i_ptr, o2_ptr); }
    calc_bonuses(1 as libc::c_int as bool_);
    list_weapon(o2_ptr, i, 40 as libc::c_int);
    compare_weapon_aux1(o2_ptr, 40 as libc::c_int, i + 8 as libc::c_int);
    i_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(24 as libc::c_int as
                                                         isize) as
            *mut object_type;
    object_copy(i_ptr, orig_ptr);
    calc_bonuses(1 as libc::c_int as bool_);
    object_wipe(orig_ptr);
    put_str(b"(Only highest damage applies per monster. Special damage not cumulative)\x00"
                as *const u8 as *const libc::c_char, 20 as libc::c_int,
            0 as libc::c_int);
    return 1 as libc::c_int as bool_;
}
/*
 * general all-purpose fixing routine for items from building personnel
 * sharpen arrows, repair armor, repair weapon
 * -KMW-
 */
unsafe extern "C" fn fix_item(mut istart: libc::c_int, mut iend: libc::c_int,
                              mut ispecific: libc::c_int, mut iac: bool_,
                              mut ireward: libc::c_int, mut set_reward: bool_)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 9 as libc::c_int;
    let mut maxenchant: libc::c_int =
        (*p_ptr).lev as libc::c_int / 5 as libc::c_int;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut out_val: [libc::c_char; 80] = [0; 80];
    let mut tmp_str: [libc::c_char; 80] = [0; 80];
    let mut repaired: bool_ = 0 as libc::c_int as bool_;
    clear_bldg(5 as libc::c_int, 18 as libc::c_int);
    strnfmt(tmp_str.as_mut_ptr(), 80 as libc::c_int as uint_hack,
            b"  Based on your skill, we can improve up to +%d\x00" as
                *const u8 as *const libc::c_char, maxenchant);
    prt(tmp_str.as_mut_ptr() as cptr, 5 as libc::c_int, 0 as libc::c_int);
    prt(b"Status\x00" as *const u8 as *const libc::c_char, 7 as libc::c_int,
        30 as libc::c_int);
    let mut current_block_34: u64;
    i = istart;
    while i <= iend {
        o_ptr =
            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i as isize) as
                *mut object_type;
        if ispecific > 0 as libc::c_int {
            if (*o_ptr).tval as libc::c_int != ispecific {
                current_block_34 = 7815301370352969686;
            } else { current_block_34 = 5399440093318478209; }
        } else { current_block_34 = 5399440093318478209; }
        match current_block_34 {
            5399440093318478209 => {
                if (*o_ptr).tval != 0 {
                    object_desc(tmp_str.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                                1 as libc::c_int);
                    if (*o_ptr).name1 as libc::c_int != 0 &&
                           (*o_ptr).ident as libc::c_int & 0x8 as libc::c_int
                               != 0 {
                        strnfmt(out_val.as_mut_ptr(),
                                80 as libc::c_int as uint_hack,
                                b"%-40s: beyond our skills!\x00" as *const u8
                                    as *const libc::c_char,
                                tmp_str.as_mut_ptr());
                    } else if (*o_ptr).name1 != 0 {
                        strnfmt(out_val.as_mut_ptr(),
                                80 as libc::c_int as uint_hack,
                                b"%-40s: in fine condition\x00" as *const u8
                                    as *const libc::c_char,
                                tmp_str.as_mut_ptr());
                    } else if iac as libc::c_int != 0 &&
                                  (*o_ptr).to_a as libc::c_int <=
                                      -(3 as libc::c_int) {
                        strnfmt(out_val.as_mut_ptr(),
                                80 as libc::c_int as uint_hack,
                                b"%-40s: beyond repair, buy a new one\x00" as
                                    *const u8 as *const libc::c_char,
                                tmp_str.as_mut_ptr());
                    } else if iac as libc::c_int != 0 &&
                                  ((*o_ptr).to_a as libc::c_int) < maxenchant
                     {
                        (*o_ptr).to_a += 1;
                        strnfmt(out_val.as_mut_ptr(),
                                80 as libc::c_int as uint_hack,
                                b"%-40s: polished -> (%d)\x00" as *const u8 as
                                    *const libc::c_char, tmp_str.as_mut_ptr(),
                                (*o_ptr).to_a as libc::c_int);
                        repaired = 1 as libc::c_int as bool_
                    } else if iac == 0 &&
                                  ((*o_ptr).to_h as libc::c_int <=
                                       -(3 as libc::c_int) ||
                                       (*o_ptr).to_d as libc::c_int <=
                                           -(3 as libc::c_int)) {
                        strnfmt(out_val.as_mut_ptr(),
                                80 as libc::c_int as uint_hack,
                                b"%-40s: beyond repair, buy a new one\x00" as
                                    *const u8 as *const libc::c_char,
                                tmp_str.as_mut_ptr());
                    } else if iac == 0 &&
                                  (((*o_ptr).to_h as libc::c_int) < maxenchant
                                       ||
                                       ((*o_ptr).to_d as libc::c_int) <
                                           maxenchant) {
                        if ((*o_ptr).to_h as libc::c_int) < maxenchant {
                            (*o_ptr).to_h += 1
                        }
                        if ((*o_ptr).to_d as libc::c_int) < maxenchant {
                            (*o_ptr).to_d += 1
                        }
                        strnfmt(out_val.as_mut_ptr(),
                                80 as libc::c_int as uint_hack,
                                b"%-40s: sharpened -> (%d,%d)\x00" as
                                    *const u8 as *const libc::c_char,
                                tmp_str.as_mut_ptr(),
                                (*o_ptr).to_h as libc::c_int,
                                (*o_ptr).to_d as libc::c_int);
                        repaired = 1 as libc::c_int as bool_
                    } else {
                        strnfmt(out_val.as_mut_ptr(),
                                80 as libc::c_int as uint_hack,
                                b"%-40s: in fine condition\x00" as *const u8
                                    as *const libc::c_char,
                                tmp_str.as_mut_ptr());
                    }
                    let fresh14 = j;
                    j = j + 1;
                    prt(out_val.as_mut_ptr() as cptr, fresh14,
                        0 as libc::c_int);
                }
            }
            _ => { }
        }
        i += 1
    }
    if repaired == 0 {
        msg_print(b"You don\'t have anything appropriate.\x00" as *const u8 as
                      *const libc::c_char);
        msg_print(0 as cptr);
    } else {
        msg_print(b"Press the spacebar to continue\x00" as *const u8 as
                      *const libc::c_char);
        msg_print(0 as cptr);
        /* Sharpen a weapon */
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long |
                      0x8 as libc::c_long)) as u32b;
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b
    }
    clear_bldg(5 as libc::c_int, 18 as libc::c_int);
    return repaired;
}
/*
 * Research Item
 */
unsafe extern "C" fn research_item() -> bool_ {
    clear_bldg(5 as libc::c_int, 18 as libc::c_int);
    return identify_fully();
}
/*
 * Show the current quest monster. 
 */
unsafe extern "C" fn show_quest_monster() {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(*(*bounties.as_mut_ptr().offset(0 as libc::c_int
                                                                as
                                                                isize)).as_mut_ptr().offset(0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                isize)
                                as isize) as *mut monster_race;
    msg_format(b"Quest monster: %s. Need to turn in %d corpse%s to receive reward.\x00"
                   as *const u8 as *const libc::c_char,
               r_name.offset((*r_ptr).name as isize),
               bounties[0 as libc::c_int as usize][1 as libc::c_int as usize]
                   as libc::c_int,
               if bounties[0 as libc::c_int as
                               usize][1 as libc::c_int as usize] as
                      libc::c_int > 1 as libc::c_int {
                   b"s\x00" as *const u8 as *const libc::c_char
               } else { b"\x00" as *const u8 as *const libc::c_char });
    msg_print(0 as cptr);
}
/*
 * Show the current bounties.
 */
unsafe extern "C" fn show_bounties() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 6 as libc::c_int;
    let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
    let mut buff: [libc::c_char; 80] = [0; 80];
    clear_bldg(7 as libc::c_int, 18 as libc::c_int);
    c_prt(11 as libc::c_int as byte_hack,
          b"Currently active bounties:\x00" as *const u8 as
              *const libc::c_char, 4 as libc::c_int, 2 as libc::c_int);
    i = 1 as libc::c_int;
    while i < 24 as libc::c_int {
        r_ptr =
            &mut *r_info.offset(*(*bounties.as_mut_ptr().offset(i as
                                                                    isize)).as_mut_ptr().offset(0
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    isize)
                                    as isize) as *mut monster_race;
        strnfmt(buff.as_mut_ptr(), 80 as libc::c_int as uint_hack,
                b"%-30s (%d gp)\x00" as *const u8 as *const libc::c_char,
                r_name.offset((*r_ptr).name as isize),
                bounties[i as usize][1 as libc::c_int as usize] as
                    libc::c_int);
        prt(buff.as_mut_ptr() as cptr, j, 2 as libc::c_int);
        if j >= 17 as libc::c_int {
            msg_print(b"Press space for more.\x00" as *const u8 as
                          *const libc::c_char);
            msg_print(0 as cptr);
            clear_bldg(7 as libc::c_int, 18 as libc::c_int);
            j = 5 as libc::c_int
        }
        i += 1;
        j += 1
    };
}
/*
 * Filter for corpses that currently have a bounty on them.
 */
unsafe extern "C" fn item_tester_hook_bounty(mut o_ptr: *mut object_type)
 -> bool_ {
    let mut i: libc::c_int = 0;
    if (*o_ptr).tval as libc::c_int == 9 as libc::c_int {
        i = 1 as libc::c_int;
        while i < 24 as libc::c_int {
            if bounties[i as usize][0 as libc::c_int as usize] as libc::c_int
                   == (*o_ptr).pval2 as libc::c_int {
                return 1 as libc::c_int as bool_
            }
            i += 1
        }
    }
    return 0 as libc::c_int as bool_;
}
/* Filter to match the quest monster's corpse. */
unsafe extern "C" fn item_tester_hook_quest_monster(mut o_ptr:
                                                        *mut object_type)
 -> bool_ {
    if (*o_ptr).tval as libc::c_int == 9 as libc::c_int &&
           (*o_ptr).pval2 as libc::c_int ==
               bounties[0 as libc::c_int as usize][0 as libc::c_int as usize]
                   as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    return 0 as libc::c_int as bool_;
}
/*
 * Return the boost in the corpse's value depending on how rare the body
 * part is.
 */
unsafe extern "C" fn corpse_value_boost(mut sval: libc::c_int)
 -> libc::c_int {
    match sval {
        3 | 4 => { return 1 as libc::c_int }
        _ => {
            /* Default to no boost. */
            return 0 as libc::c_int
        }
    };
}
/*
 * Sell a corpse, if there's currently a bounty on it.
 */
unsafe extern "C" fn sell_corpses() {
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut i: libc::c_int = 0;
    let mut boost: libc::c_int = 0 as libc::c_int;
    let mut value: s16b = 0;
    let mut item: libc::c_int = 0;
    /* Set the hook. */
    item_tester_hook =
        Some(item_tester_hook_bounty as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Select a corpse to sell. */
    if get_item(&mut item,
                b"Sell which corpse\x00" as *const u8 as *const libc::c_char,
                b"You have no corpses you can sell.\x00" as *const u8 as
                    *const libc::c_char, 0x2 as libc::c_int) == 0 {
        return
    }
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(item as isize) as
            *mut object_type;
    /* Exotic body parts are worth more. */
    boost = corpse_value_boost((*o_ptr).sval as libc::c_int);
    /* Try to find a match. */
    i = 1 as libc::c_int;
    while i < 24 as libc::c_int {
        if (*o_ptr).pval2 as libc::c_int ==
               bounties[i as usize][0 as libc::c_int as usize] as libc::c_int
           {
            value =
                (bounties[i as usize][1 as libc::c_int as usize] as
                     libc::c_int +
                     boost *
                         (*r_info.offset((*o_ptr).pval2 as isize)).level as
                             libc::c_int) as s16b;
            msg_format(b"Sold for %ld gold pieces.\x00" as *const u8 as
                           *const libc::c_char, value as libc::c_int);
            msg_print(0 as cptr);
            (*p_ptr).au += value as libc::c_int;
            /* Increase the number of collected bounties */
            total_bounties = total_bounties.wrapping_add(1);
            inc_stack_size(item, -(1 as libc::c_int));
            return
        }
        i += 1
    }
    msg_print(b"Sorry, but that monster does not have a bounty on it.\x00" as
                  *const u8 as *const libc::c_char);
    msg_print(0 as cptr);
}
/*
 * Hook for bounty monster selection.
 */
unsafe extern "C" fn mon_hook_bounty(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    /* Reject uniques */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Reject those who cannot leave anything */
    if (*r_ptr).flags9 & 0x1 as libc::c_int as libc::c_uint == 0 &&
           (*r_ptr).flags9 & 0x2 as libc::c_int as libc::c_uint == 0 {
        return 0 as libc::c_int as bool_
    }
    /* Reject pets */
    if (*r_ptr).flags7 & 0x10 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Reject friendly creatures */
    if (*r_ptr).flags7 & 0x8 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* The rest are acceptable */
    return 1 as libc::c_int as bool_;
}
unsafe extern "C" fn select_quest_monster() {
    let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
    let mut amt: libc::c_int = 0;
    /*
	 * Set up the hooks -- no bounties on uniques or monsters
	 * with no corpses
	 */
    get_mon_num_hook =
        Some(mon_hook_bounty as
                 unsafe extern "C" fn(_: libc::c_int) -> bool_);
    get_mon_num_prep();
    /* Set up the quest monster. */
    bounties[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        get_mon_num((*p_ptr).lev as libc::c_int);
    r_ptr =
        &mut *r_info.offset(*(*bounties.as_mut_ptr().offset(0 as libc::c_int
                                                                as
                                                                isize)).as_mut_ptr().offset(0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                isize)
                                as isize) as *mut monster_race;
    /*
	 * Select the number of monsters needed to kill. Groups and
	 * breeders require more
	 */
    amt = randnor(5 as libc::c_int, 3 as libc::c_int) as libc::c_int;
    if amt < 2 as libc::c_int { amt = 2 as libc::c_int }
    if (*r_ptr).flags1 & 0x1000 as libc::c_int as libc::c_uint != 0 {
        amt *= 3 as libc::c_int
    }
    amt /= 2 as libc::c_int;
    if (*r_ptr).flags1 & 0x2000 as libc::c_int as libc::c_uint != 0 {
        amt *= 2 as libc::c_int
    }
    if (*r_ptr).flags4 & 0x2 as libc::c_int as libc::c_uint != 0 {
        amt *= 3 as libc::c_int
    }
    if (*r_ptr).flags7 & 0x1 as libc::c_int as libc::c_uint != 0 {
        amt /= 2 as libc::c_int
    }
    bounties[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        amt as s16b;
    /* Undo the filters */
    get_mon_num_hook = None;
    get_mon_num_prep();
}
/*
 * Sell a corpse for a reward.
 */
unsafe extern "C" fn sell_quest_monster() {
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut item: libc::c_int = 0;
    /* Set the hook. */
    item_tester_hook =
        Some(item_tester_hook_quest_monster as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Select a corpse to sell. */
    if get_item(&mut item,
                b"Sell which corpse\x00" as *const u8 as *const libc::c_char,
                b"You have no corpses you can sell.\x00" as *const u8 as
                    *const libc::c_char, 0x2 as libc::c_int) == 0 {
        return
    }
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(item as isize) as
            *mut object_type;
    bounties[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        (bounties[0 as libc::c_int as usize][1 as libc::c_int as usize] as
             libc::c_int - (*o_ptr).number as libc::c_int) as s16b;
    /* Completed the quest. */
    if bounties[0 as libc::c_int as usize][1 as libc::c_int as usize] as
           libc::c_int <= 0 as libc::c_int {
        let mut m: libc::c_int = 0;
        let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
        cmsg_print(11 as libc::c_int as byte_hack,
                   b"You have completed your quest!\x00" as *const u8 as
                       *const libc::c_char);
        msg_print(0 as cptr);
        /* Give full knowledge */
        /* Hack -- Maximal info */
        r_ptr =
            &mut *r_info.offset(*(*bounties.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as
                                                                    isize)).as_mut_ptr().offset(0
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    isize)
                                    as isize) as *mut monster_race;
        msg_print(format(b"Well done! As a reward I\'ll teach you everything about the %s, (check your recall)\x00"
                             as *const u8 as *const libc::c_char,
                         r_name.offset((*r_ptr).name as isize)) as cptr);
        (*r_ptr).r_ignore = 255 as libc::c_int as byte_hack;
        (*r_ptr).r_wake = (*r_ptr).r_ignore;
        /* Observe "maximal" attacks */
        m = 0 as libc::c_int;
        while m < 4 as libc::c_int {
            /* Examine "actual" blows */
            if (*r_ptr).blow[m as usize].effect as libc::c_int != 0 ||
                   (*r_ptr).blow[m as usize].method as libc::c_int != 0 {
                /* Hack -- maximal observations */
                (*r_ptr).r_blows[m as usize] = 255 as libc::c_int as byte_hack
            }
            m += 1
        }
        /* Hack -- maximal drops */
        (*r_ptr).r_drop_item =
            ((if (*r_ptr).flags1 & 0x8000000 as libc::c_int as libc::c_uint !=
                     0 {
                  8 as libc::c_int
              } else { 0 as libc::c_int }) +
                 (if (*r_ptr).flags1 &
                         0x4000000 as libc::c_int as libc::c_uint != 0 {
                      6 as libc::c_int
                  } else { 0 as libc::c_int }) +
                 (if (*r_ptr).flags1 &
                         0x2000000 as libc::c_int as libc::c_uint != 0 {
                      4 as libc::c_int
                  } else { 0 as libc::c_int }) +
                 (if (*r_ptr).flags1 &
                         0x1000000 as libc::c_int as libc::c_uint != 0 {
                      2 as libc::c_int
                  } else { 0 as libc::c_int }) +
                 (if (*r_ptr).flags1 & 0x800000 as libc::c_int as libc::c_uint
                         != 0 {
                      1 as libc::c_int
                  } else { 0 as libc::c_int }) +
                 (if (*r_ptr).flags1 & 0x400000 as libc::c_int as libc::c_uint
                         != 0 {
                      1 as libc::c_int
                  } else { 0 as libc::c_int })) as byte_hack;
        (*r_ptr).r_drop_gold = (*r_ptr).r_drop_item;
        /* Hack -- but only "valid" drops */
        if (*r_ptr).flags1 & 0x100000 as libc::c_int as libc::c_uint != 0 {
            (*r_ptr).r_drop_item = 0 as libc::c_int as byte_hack
        }
        if (*r_ptr).flags1 & 0x200000 as libc::c_int as libc::c_uint != 0 {
            (*r_ptr).r_drop_gold = 0 as libc::c_int as byte_hack
        }
        /* Hack -- observe many spells */
        (*r_ptr).r_cast_inate = 255 as libc::c_int as byte_hack;
        (*r_ptr).r_cast_spell = 255 as libc::c_int as byte_hack;
        /* Hack -- know all the flags */
        (*r_ptr).r_flags1 = (*r_ptr).flags1;
        (*r_ptr).r_flags2 = (*r_ptr).flags2;
        (*r_ptr).r_flags3 = (*r_ptr).flags3;
        (*r_ptr).r_flags4 = (*r_ptr).flags4;
        (*r_ptr).r_flags5 = (*r_ptr).flags5;
        (*r_ptr).r_flags6 = (*r_ptr).flags6;
        (*r_ptr).r_flags4 = (*r_ptr).flags7;
        (*r_ptr).r_flags5 = (*r_ptr).flags8;
        (*r_ptr).r_flags6 = (*r_ptr).flags9;
        msg_print(0 as cptr);
        select_quest_monster();
    } else {
        msg_format(b"Well done, only %d more to go.\x00" as *const u8 as
                       *const libc::c_char,
                   bounties[0 as libc::c_int as
                                usize][1 as libc::c_int as usize] as
                       libc::c_int);
        msg_print(0 as cptr);
    }
    inc_stack_size(item, -(1 as libc::c_int));
}
/*
 * Fill the bounty list with monsters.
 */
#[no_mangle]
pub unsafe extern "C" fn select_bounties() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    select_quest_monster();
    /*
	 * Set up the hooks -- no bounties on uniques or monsters
	 * with no corpses
	 */
    get_mon_num_hook =
        Some(mon_hook_bounty as
                 unsafe extern "C" fn(_: libc::c_int) -> bool_);
    get_mon_num_prep();
    i = 1 as libc::c_int;
    while i < 24 as libc::c_int {
        let mut lev: libc::c_int =
            i * 5 as libc::c_int +
                randnor(0 as libc::c_int, 2 as libc::c_int) as libc::c_int;
        let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
        let mut r_idx: s16b = 0;
        let mut val: s16b = 0;
        if lev < 1 as libc::c_int { lev = 1 as libc::c_int }
        if lev >= 128 as libc::c_int {
            lev = 128 as libc::c_int - 1 as libc::c_int
        }
        /* We don't want to duplicate entries in the list */
        r_idx = get_mon_num(lev);
        j = 0 as libc::c_int;
        while j < i {
            (bounties[j as usize][0 as libc::c_int as usize] as libc::c_int)
                == r_idx as libc::c_int;
            j += 1
        }
        bounties[i as usize][0 as libc::c_int as usize] = r_idx;
        r_ptr = &mut *r_info.offset(r_idx as isize) as *mut monster_race;
        val =
            ((*r_ptr).mexp + (*r_ptr).level as libc::c_int * 20 as libc::c_int
                 +
                 randnor(0 as libc::c_int,
                         (*r_ptr).level as libc::c_int * 2 as libc::c_int) as
                     libc::c_int) as s16b;
        if (val as libc::c_int) < 1 as libc::c_int {
            val = 1 as libc::c_int as s16b
        }
        bounties[i as usize][1 as libc::c_int as usize] = val;
        i += 1
    }
    /* Undo the filters. */
    get_mon_num_hook = None;
    get_mon_num_prep();
}
/*
 * Execute a building command
 */
#[no_mangle]
pub unsafe extern "C" fn bldg_process_command(mut s_ptr: *mut store_type,
                                              mut i: libc::c_int) -> bool_ {
    let mut ba_ptr: *mut store_action_type =
        &mut *ba_info.offset(*(*st_info.offset((*s_ptr).st_idx as
                                                   isize)).actions.as_mut_ptr().offset(i
                                                                                           as
                                                                                           isize)
                                 as isize) as *mut store_action_type;
    let mut bact: libc::c_int = (*ba_ptr).action as libc::c_int;
    let mut bcost: libc::c_int = 0;
    let mut paid: bool_ = 0 as libc::c_int as bool_;
    let mut set_reward: bool_ = 0 as libc::c_int as bool_;
    let mut recreate: bool_ = 0 as libc::c_int as bool_;
    if is_state(s_ptr, 1 as libc::c_int) != 0 {
        bcost = (*ba_ptr).costs[1 as libc::c_int as usize] as libc::c_int
    } else if is_state(s_ptr, 0 as libc::c_int) != 0 {
        bcost = (*ba_ptr).costs[0 as libc::c_int as usize] as libc::c_int
    } else {
        bcost = (*ba_ptr).costs[2 as libc::c_int as usize] as libc::c_int
    }
    /* action restrictions */
    if (*ba_ptr).action_restr as libc::c_int == 1 as libc::c_int &&
           is_state(s_ptr, 1 as libc::c_int) as libc::c_int != 0 ||
           (*ba_ptr).action_restr as libc::c_int == 2 as libc::c_int &&
               is_state(s_ptr, 1 as libc::c_int) == 0 {
        msg_print(b"You have no right to choose that!\x00" as *const u8 as
                      *const libc::c_char);
        msg_print(0 as cptr);
        return 0 as libc::c_int as bool_
    }
    /* If player has loan and the time is out, few things work in stores */
    if (*p_ptr).loan != 0 && (*p_ptr).loan_time == 0 {
        if bact != 43 as libc::c_int && bact != 38 as libc::c_int &&
               bact != 39 as libc::c_int && bact != 40 as libc::c_int &&
               bact != 41 as libc::c_int && bact != 45 as libc::c_int &&
               bact != 46 as libc::c_int && bact != 53 as libc::c_int {
            msg_print(b"You are not allowed to do that until you have paid back your loan.\x00"
                          as *const u8 as *const libc::c_char);
            msg_print(0 as cptr);
            return 0 as libc::c_int as bool_
        }
    }
    /* check gold */
    if bcost > (*p_ptr).au {
        msg_print(b"You do not have the gold!\x00" as *const u8 as
                      *const libc::c_char);
        msg_print(0 as cptr);
        return 0 as libc::c_int as bool_
    }
    if bcost == 0 { set_reward = 1 as libc::c_int as bool_ }
    match bact {
        1 => { paid = research_item() }
        2 => { town_history(); }
        3 => { race_legends(); }
        6 | 47 | 48 | 49 => {
            let mut y: libc::c_int = 1 as libc::c_int;
            let mut x: libc::c_int = 1 as libc::c_int;
            let mut ok: bool_ = 0 as libc::c_int as bool_;
            while x < cur_wid as libc::c_int - 1 as libc::c_int && ok == 0 {
                y = 1 as libc::c_int;
                while y < cur_hgt as libc::c_int - 1 as libc::c_int && ok == 0
                      {
                    /* Found the location of the quest info ? */
                    if bact - 6 as libc::c_int + 0x4b as libc::c_int ==
                           (*cave[y as usize].offset(x as isize)).feat as
                               libc::c_int {
                        /* Stop the loop */
                        ok = 1 as libc::c_int as bool_
                    }
                    y += 1
                }
                x += 1
            }
            if ok != 0 {
                recreate =
                    castle_quest(y - 1 as libc::c_int, x - 1 as libc::c_int)
            } else {
                msg_format(b"ERROR: no quest info feature found: %d\x00" as
                               *const u8 as *const libc::c_char,
                           bact - 6 as libc::c_int + 0x4b as libc::c_int);
            }
        }
        5 | 11 | 22 => { show_highclass(building_loc); }
        8 | 9 | 10 => { arena_comm(bact); }
        12 | 14 | 15 | 16 | 13 => { gamble_comm(bact); }
        17 | 19 | 18 => { paid = inn_comm(bact) }
        20 => { paid = (research_mon() == 0) as libc::c_int as bool_ }
        21 => { paid = compare_weapons() }
        23 => {
            paid =
                fix_item(24 as libc::c_int, 24 as libc::c_int,
                         0 as libc::c_int, 0 as libc::c_int as bool_,
                         23 as libc::c_int, set_reward)
        }
        24 => {
            paid =
                fix_item(37 as libc::c_int, 47 as libc::c_int,
                         0 as libc::c_int, 1 as libc::c_int as bool_,
                         24 as libc::c_int, set_reward)
        }
        25 => {
            /* needs work */
            if recharge(80 as libc::c_int) != 0 {
                paid = 1 as libc::c_int as bool_
            }
        }
        26 => {
            /* needs work */
            identify_pack();
            msg_print(b"Your possessions have been identified.\x00" as
                          *const u8 as *const libc::c_char);
            msg_print(0 as cptr);
            paid = 1 as libc::c_int as bool_
        }
        50 => {
            /* needs work */
            hp_player(200 as libc::c_int);
            set_poisoned(0 as libc::c_int);
            set_blind(0 as libc::c_int);
            set_confused(0 as libc::c_int);
            set_cut(0 as libc::c_int);
            set_stun(0 as libc::c_int);
            if (*p_ptr).black_breath != 0 {
                msg_print(b"The hold of the Black Breath on you is broken!\x00"
                              as *const u8 as *const libc::c_char);
                (*p_ptr).black_breath = 0 as libc::c_int as bool_
            }
            paid = 1 as libc::c_int as bool_
        }
        28 => {
            /* needs work */
            hp_player(200 as libc::c_int);
            set_poisoned(0 as libc::c_int);
            set_blind(0 as libc::c_int);
            set_confused(0 as libc::c_int);
            set_cut(0 as libc::c_int);
            set_stun(0 as libc::c_int);
            paid = 1 as libc::c_int as bool_
        }
        29 => {
            /* needs work */
            if do_res_stat(0 as libc::c_int, 1 as libc::c_int as bool_) != 0 {
                paid = 1 as libc::c_int as bool_
            }
            if do_res_stat(1 as libc::c_int, 1 as libc::c_int as bool_) != 0 {
                paid = 1 as libc::c_int as bool_
            }
            if do_res_stat(2 as libc::c_int, 1 as libc::c_int as bool_) != 0 {
                paid = 1 as libc::c_int as bool_
            }
            if do_res_stat(3 as libc::c_int, 1 as libc::c_int as bool_) != 0 {
                paid = 1 as libc::c_int as bool_
            }
            if do_res_stat(4 as libc::c_int, 1 as libc::c_int as bool_) != 0 {
                paid = 1 as libc::c_int as bool_
            }
            if do_res_stat(5 as libc::c_int, 1 as libc::c_int as bool_) != 0 {
                paid = 1 as libc::c_int as bool_
            }
        }
        30 => {
            paid =
                fix_item(0 as libc::c_int, 24 as libc::c_int,
                         17 as libc::c_int, 0 as libc::c_int as bool_,
                         30 as libc::c_int, set_reward)
        }
        31 => {
            paid =
                fix_item(27 as libc::c_int, 27 as libc::c_int,
                         19 as libc::c_int, 0 as libc::c_int as bool_,
                         31 as libc::c_int, set_reward)
        }
        33 => {
            (*p_ptr).word_recall = 1 as libc::c_int as s16b;
            msg_print(b"The air about you becomes charged...\x00" as *const u8
                          as *const libc::c_char);
            paid = 1 as libc::c_int as bool_
        }
        34 => {
            if reset_recall(0 as libc::c_int as bool_) != 0 {
                (*p_ptr).word_recall = 1 as libc::c_int as s16b;
                msg_print(b"The air about you becomes charged...\x00" as
                              *const u8 as *const libc::c_char);
                paid = 1 as libc::c_int as bool_
            }
        }
        37 => {
            set_mimic(0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
            paid = 1 as libc::c_int as bool_
        }
        38 => { show_bounties(); }
        40 => { show_quest_monster(); }
        41 => { sell_quest_monster(); }
        39 => { sell_corpses(); }
        42 => {
            let mut i_0: libc::c_int = 0;
            let mut count: libc::c_int = 0 as libc::c_int;
            let mut something: bool_ = 0 as libc::c_int as bool_;
            while count < 1000 as libc::c_int {
                count += 1;
                i_0 = Rand_div(200 as libc::c_int);
                if fates[i_0 as usize].fate == 0 { continue ; }
                if fates[i_0 as usize].know != 0 { continue ; }
                msg_print(b"You know a little more of your fate.\x00" as
                              *const u8 as *const libc::c_char);
                fates[i_0 as usize].know = 1 as libc::c_int as bool_;
                something = 1 as libc::c_int as bool_;
                break ;
            }
            if something == 0 {
                msg_print(b"Well, you have no fate, but I\'ll keep your money anyway!\x00"
                              as *const u8 as *const libc::c_char);
            }
            paid = 1 as libc::c_int as bool_
        }
        44 => { store_purchase(); }
        43 => { store_sell(); }
        45 => { store_examine(); }
        46 => { store_stole(); }
        51 => { store_request_item(); paid = 1 as libc::c_int as bool_ }
        52 => {
            let mut i_1: s32b = 0;
            let mut req: s32b = 0;
            let mut prompt: [libc::c_char; 80] = [0; 80];
            if (*p_ptr).loan != 0 {
                msg_print(b"You already have a loan!\x00" as *const u8 as
                              *const libc::c_char);
            } else {
                req = (*p_ptr).au;
                i_1 = 0 as libc::c_int;
                while i_1 < 52 as libc::c_int {
                    req +=
                        object_value_real(&mut *(*p_ptr).inventory.as_mut_ptr().offset(i_1
                                                                                           as
                                                                                           isize));
                    i_1 += 1
                }
                if req > 100000 as libc::c_int { req = 100000 as libc::c_int }
                if (req + (*p_ptr).au) as libc::c_long >
                       999999999 as libc::c_long {
                    req =
                        (999999999 as libc::c_long -
                             (*p_ptr).au as libc::c_long) as s32b
                }
                strnfmt(prompt.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 80]>() as
                            libc::c_ulong as uint_hack,
                        b"How much would you like to get (0-%ld) ?\x00" as
                            *const u8 as *const libc::c_char, req);
                req = get_quantity(prompt.as_mut_ptr() as cptr, req);
                if req != 0 {
                    (*p_ptr).loan += req;
                    (*p_ptr).au += req;
                    (*p_ptr).loan_time += req;
                    msg_format(b"You receive %i gold pieces\x00" as *const u8
                                   as *const libc::c_char, req);
                    paid = 1 as libc::c_int as bool_
                } else {
                    msg_format(b"You did not request any money!\x00" as
                                   *const u8 as *const libc::c_char);
                }
            }
        }
        53 => {
            let mut req_0: s32b = 0;
            let mut prompt_0: [libc::c_char; 80] = [0; 80];
            if (*p_ptr).loan != 0 {
                msg_format(b"You have nothing to payback!\x00" as *const u8 as
                               *const libc::c_char);
            } else {
                msg_format(b"You have a loan of %i.\x00" as *const u8 as
                               *const libc::c_char, (*p_ptr).loan);
                req_0 =
                    if (*p_ptr).loan + bcost > (*p_ptr).au {
                        ((*p_ptr).au) - bcost
                    } else { (*p_ptr).loan };
                strnfmt(prompt_0.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 80]>() as
                            libc::c_ulong as uint_hack,
                        b"How much would you like to pay back (0-%ld) ?\x00"
                            as *const u8 as *const libc::c_char, req_0);
                req_0 = get_quantity(prompt_0.as_mut_ptr() as cptr, req_0);
                (*p_ptr).loan -= req_0;
                (*p_ptr).au -= req_0;
                if (*p_ptr).loan == 0 {
                    (*p_ptr).loan_time = 0 as libc::c_int
                }
                msg_format(b"You pay back %i gold pieces\x00" as *const u8 as
                               *const libc::c_char, req_0);
                paid = 1 as libc::c_int as bool_
            }
        }
        _ => {
            if process_hooks_ret(44 as libc::c_int,
                                 b"dd\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char,
                                 b"(d)\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 bact) != 0 {
                paid =
                    process_hooks_return[0 as libc::c_int as usize].num as
                        bool_;
                recreate =
                    process_hooks_return[1 as libc::c_int as usize].num as
                        bool_
            }
        }
    }
    if paid != 0 {
        (*p_ptr).au -= bcost;
        /* Display the current gold */
        store_prt_gold();
    }
    return recreate;
}
/*
 * Enter quest level
 */
#[no_mangle]
pub unsafe extern "C" fn enter_quest() {
    if !((*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat as
             libc::c_int == 0x8 as libc::c_int) {
        msg_print(b"You see no quest level here.\x00" as *const u8 as
                      *const libc::c_char);
        return
    } else {
        /* Player enters a new quest */
        (*p_ptr).oldpy = (*p_ptr).py;
        (*p_ptr).oldpx = (*p_ptr).px;
        leaving_quest = (*p_ptr).inside_quest as libc::c_int;
        (*p_ptr).inside_quest =
            (*cave[(*p_ptr).py as
                       usize].offset((*p_ptr).px as isize)).special;
        dun_level = 1 as libc::c_int as s16b;
        (*p_ptr).leaving = 1 as libc::c_int as bool_;
        (*p_ptr).oldpx = (*p_ptr).px;
        (*p_ptr).oldpy = (*p_ptr).py
    };
}
/*
 * Do building commands
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_bldg() {
    let mut i: libc::c_int = 0;
    let mut which: libc::c_int = 0;
    let mut x: libc::c_int = (*p_ptr).px as libc::c_int;
    let mut y: libc::c_int = (*p_ptr).py as libc::c_int;
    let mut command: libc::c_char = 0;
    let mut validcmd: bool_ = 0;
    let mut s_ptr: *mut store_type = 0 as *mut store_type;
    let mut ba_ptr: *mut store_action_type = 0 as *mut store_action_type;
    if (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).feat as
           libc::c_int != 0x4a as libc::c_int {
        msg_print(b"You see no building here.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    which =
        (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).special as
            libc::c_int;
    building_loc = which;
    s_ptr =
        &mut *(*town_info.offset((*p_ptr).town_num as
                                     isize)).store.offset(which as isize) as
            *mut store_type;
    (*p_ptr).oldpy = (*p_ptr).py;
    (*p_ptr).oldpx = (*p_ptr).px;
    /* Forget the lite */
	/* forget_lite(); */
    /* Forget the view */
    forget_view();
    /* Hack -- Increase "icky" depth */
    character_icky += 1;
    command_arg = 0 as libc::c_int as s16b;
    command_rep = 0 as libc::c_int as s16b;
    command_new = 0 as libc::c_int as s16b;
    show_building(s_ptr);
    leave_bldg = 0 as libc::c_int as bool_;
    while leave_bldg == 0 {
        validcmd = 0 as libc::c_int as bool_;
        prt(b"\x00" as *const u8 as *const libc::c_char, 1 as libc::c_int,
            0 as libc::c_int);
        command = inkey();
        if command as libc::c_int == '\u{1b}' as i32 {
            leave_bldg = 1 as libc::c_int as bool_;
            (*p_ptr).inside_arena = 0 as libc::c_int as s16b;
            break ;
        } else {
            i = 0 as libc::c_int;
            while i < 6 as libc::c_int {
                ba_ptr =
                    &mut *ba_info.offset(*(*st_info).actions.as_mut_ptr().offset(i
                                                                                     as
                                                                                     isize)
                                             as isize) as
                        *mut store_action_type;
                if (*ba_ptr).letter != 0 {
                    if (*ba_ptr).letter as libc::c_int ==
                           command as libc::c_int {
                        validcmd = 1 as libc::c_int as bool_;
                        break ;
                    }
                }
                if (*ba_ptr).letter_aux != 0 {
                    if (*ba_ptr).letter_aux as libc::c_int ==
                           command as libc::c_int {
                        validcmd = 1 as libc::c_int as bool_;
                        break ;
                    }
                }
                i += 1
            }
            if validcmd != 0 { bldg_process_command(s_ptr, i); }
            /* Notice stuff */
            notice_stuff();
            /* Handle stuff */
            handle_stuff();
        }
    }
    /* Flush messages XXX XXX XXX */
    msg_print(0 as cptr);
    /* Reinit wilderness to activate quests ... */
    wilderness_gen(1 as libc::c_int);
    (*p_ptr).py = y as s16b;
    (*p_ptr).px = x as s16b;
    /* Hack -- Decrease "icky" depth */
    character_icky -= 1;
    /* Clear the screen */
    Term_clear();
    /* Update the visuals */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x100000 as libc::c_long | 0x200000 as libc::c_long |
                  0x1000000 as libc::c_long | 0x1 as libc::c_long)) as u32b;
    /* Redraw entire screen */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long |
             (0x2000000 as libc::c_long | 0x1000000 as libc::c_long |
                  0x4000000 as libc::c_long)) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
}
