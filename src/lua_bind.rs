use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type lua_State;
    #[no_mangle]
    static mut ddx: [s16b; 10];
    #[no_mangle]
    static mut ddy: [s16b; 10];
    #[no_mangle]
    static mut adj_mag_fail: [byte_hack; 0];
    #[no_mangle]
    static mut adj_mag_stat: [byte_hack; 0];
    #[no_mangle]
    static mut cur_hgt: s16b;
    #[no_mangle]
    static mut cur_wid: s16b;
    #[no_mangle]
    static mut inkey_scan: bool_;
    #[no_mangle]
    static mut flush_failure: bool_;
    #[no_mangle]
    static mut floor_type: [s16b; 100];
    #[no_mangle]
    static mut target_who: s16b;
    #[no_mangle]
    static mut target_col: s16b;
    #[no_mangle]
    static mut target_row: s16b;
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut f_info: *mut feature_type;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut item_tester_tval: byte_hack;
    #[no_mangle]
    static mut item_tester_hook:
           Option<unsafe extern "C" fn(_: *mut object_type) -> bool_>;
    #[no_mangle]
    static mut get_mon_num_hook:
           Option<unsafe extern "C" fn(_: libc::c_int) -> bool_>;
    #[no_mangle]
    static mut init_flags: libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn Term_xtra(n: libc::c_int, v: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_putch(x: libc::c_int, y: libc::c_int, a: byte_hack,
                  c: libc::c_char) -> errr;
    #[no_mangle]
    fn Term_redraw_section(x1: libc::c_int, y1: libc::c_int, x2: libc::c_int,
                           y2: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_get_size(w: *mut libc::c_int, h: *mut libc::c_int) -> errr;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut powers_type: *mut power_type;
    #[no_mangle]
    static mut power_max: s16b;
    #[no_mangle]
    static mut max_q_idx: s16b;
    #[no_mangle]
    static mut quest: *mut quest_type;
    #[no_mangle]
    static mut school_spells: *mut spell_type;
    #[no_mangle]
    static mut schools: *mut school_type;
    #[no_mangle]
    static mut deity_info: *mut deity_type;
    #[no_mangle]
    static mut max_gods: s32b;
    #[no_mangle]
    static mut hook_file: *mut FILE;
    #[no_mangle]
    fn cave_set_feat(y: libc::c_int, x: libc::c_int, feat: libc::c_int);
    #[no_mangle]
    fn scatter(yp: *mut libc::c_int, xp: *mut libc::c_int, y: libc::c_int,
               x: libc::c_int, d: libc::c_int);
    #[no_mangle]
    fn clamp_failure_chance(chance: libc::c_int, minfail: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn room_alloc(x: libc::c_int, y: libc::c_int, crowded: bool_,
                  by0: libc::c_int, bx0: libc::c_int, xx: *mut libc::c_int,
                  yy: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn process_dungeon_file(name: cptr, yval: *mut libc::c_int,
                            xval: *mut libc::c_int, ymax: libc::c_int,
                            xmax: libc::c_int, init: bool_, full: bool_)
     -> errr;
    #[no_mangle]
    fn reinit_gods(new_size: s16b);
    #[no_mangle]
    fn reinit_quests(new_size: s16b);
    #[no_mangle]
    fn reinit_powers_type(new_size: s16b);
    #[no_mangle]
    fn get_mon_num_prep() -> errr;
    #[no_mangle]
    fn get_mon_num(level: libc::c_int) -> s16b;
    #[no_mangle]
    fn summon_specific(y1: libc::c_int, x1: libc::c_int, lev: libc::c_int,
                       type_0: libc::c_int) -> bool_;
    #[no_mangle]
    fn summon_specific_friendly(y1: libc::c_int, x1: libc::c_int,
                                lev: libc::c_int, type_0: libc::c_int,
                                Group_ok: bool_) -> bool_;
    #[no_mangle]
    fn set_mon_num_hook();
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn input_box(text: cptr, y: libc::c_int, x: libc::c_int,
                 buf: *mut libc::c_char, max: libc::c_int) -> bool_;
    #[no_mangle]
    fn display_list(y: libc::c_int, x: libc::c_int, h: libc::c_int,
                    w: libc::c_int, title: cptr, list: *mut cptr,
                    max: libc::c_int, begin: libc::c_int, sel: libc::c_int,
                    sel_color: byte_hack);
    #[no_mangle]
    fn flush();
    #[no_mangle]
    fn sound(num: libc::c_int);
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn get_com(prompt: cptr, command: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn msg_box(text: cptr, y: libc::c_int, x: libc::c_int) -> libc::c_char;
    #[no_mangle]
    fn target_okay() -> bool_;
    #[no_mangle]
    fn exec_lua(file: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn forbid_non_blessed() -> bool_;
    #[no_mangle]
    fn has_ability(ab: libc::c_int) -> bool_;
    #[no_mangle]
    fn get_skill_scale(skill: libc::c_int, scale: u32b) -> s16b;
    #[no_mangle]
    fn lua_gettop(L_0: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L_0: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_tonumber(L_0: *mut lua_State, index: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn lua_getglobal(L_0: *mut lua_State, name: *const libc::c_char);
    #[no_mangle]
    fn lua_call(L_0: *mut lua_State, nargs: libc::c_int,
                nresults: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tolua_tag(L_0: *mut lua_State, type_0: *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn tolua_pushnumber(L_0: *mut lua_State, value: libc::c_long);
    #[no_mangle]
    fn tolua_pushusertype(L_0: *mut lua_State, value: *mut libc::c_void,
                          tag: libc::c_int);
    /* File: lua_bind.c */
    /* Purpose: various lua bindings */
    /*
 * Copyright (c) 2001 DarkGod
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
    #[no_mangle]
    static mut L: *mut lua_State;
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
pub struct magic_power {
    pub min_lev: libc::c_int,
    pub mana_cost: libc::c_int,
    pub fail: libc::c_int,
    pub name: cptr,
    pub desc: cptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deity_type {
    pub name: cptr,
    pub desc: [[libc::c_char; 80]; 10],
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
pub struct list_type {
    pub list: *mut cptr,
}
#[no_mangle]
pub unsafe extern "C" fn grab_magic_power(mut m_ptr: *mut magic_power,
                                          mut num: libc::c_int)
 -> *mut magic_power {
    return &mut *m_ptr.offset(num as isize) as *mut magic_power;
}
#[no_mangle]
pub unsafe extern "C" fn lua_spell_success(mut spell: *mut magic_power,
                                           mut stat: libc::c_int,
                                           mut oups_fct: *mut libc::c_char)
 -> bool_ {
    let mut chance: libc::c_int = 0;
    let mut minfail: libc::c_int = 0 as libc::c_int;
    /* Spell failure chance */
    chance = (*spell).fail;
    /* Reduce failure rate by "effective" level adjustment */
    chance -=
        3 as libc::c_int * ((*p_ptr).lev as libc::c_int - (*spell).min_lev);
    /* Reduce failure rate by INT/WIS adjustment */
    chance -=
        3 as libc::c_int *
            (*adj_mag_stat.as_mut_ptr().offset((*p_ptr).stat_ind[stat as
                                                                     usize] as
                                                   isize) as libc::c_int -
                 1 as libc::c_int);
    /* Not enough mana to cast */
    if (*spell).mana_cost > (*p_ptr).csp as libc::c_int {
        chance +=
            5 as libc::c_int *
                ((*spell).mana_cost - (*p_ptr).csp as libc::c_int)
    }
    /* Extract the minimum failure rate */
    minfail =
        *adj_mag_fail.as_mut_ptr().offset((*p_ptr).stat_ind[stat as usize] as
                                              isize) as libc::c_int;
    /* Failure rate */
    chance = clamp_failure_chance(chance, minfail);
    /* Failed spell */
    if Rand_div(100 as libc::c_int) < chance {
        if flush_failure != 0 { flush(); }
        msg_format(b"You failed to concentrate hard enough!\x00" as *const u8
                       as *const libc::c_char);
        sound(55 as libc::c_int);
        if !oups_fct.is_null() {
            exec_lua(format(b"%s(%d)\x00" as *const u8 as *const libc::c_char,
                            oups_fct, chance));
        }
        return 0 as libc::c_int as bool_
    }
    return 1 as libc::c_int as bool_;
}
/*
 * Create objects
 */
#[no_mangle]
pub unsafe extern "C" fn new_object() -> *mut object_type {
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    o_ptr =
        memset(ralloc(::std::mem::size_of::<object_type>() as libc::c_ulong)
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<object_type>() as libc::c_ulong) as
            *mut object_type;
    return o_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn end_object(mut o_ptr: *mut object_type) {
    rnfree(o_ptr as vptr,
           ::std::mem::size_of::<object_type>() as libc::c_ulong);
}
/*
 * Powers
 */
#[no_mangle]
pub unsafe extern "C" fn add_new_power(mut name: cptr, mut desc: cptr,
                                       mut gain: cptr, mut lose: cptr,
                                       mut level: byte_hack,
                                       mut cost: byte_hack,
                                       mut stat: byte_hack,
                                       mut diff: byte_hack) -> s16b {
    /* Increase the size */
    reinit_powers_type((power_max as libc::c_int + 1 as libc::c_int) as s16b);
    /* Copy the strings */
    let ref mut fresh0 =
        (*powers_type.offset((power_max as libc::c_int - 1 as libc::c_int) as
                                 isize)).name;
    *fresh0 =
        memset(ralloc(strlen(name).wrapping_add(1 as libc::c_int as
                                                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                                    as
                                                                                    libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               strlen(name).wrapping_add(1 as libc::c_int as
                                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                             as
                                                                             libc::c_ulong))
            as *mut libc::c_char;
    strcpy((*powers_type.offset((power_max as libc::c_int - 1 as libc::c_int)
                                    as isize)).name, name);
    let ref mut fresh1 =
        (*powers_type.offset((power_max as libc::c_int - 1 as libc::c_int) as
                                 isize)).desc_text;
    *fresh1 =
        memset(ralloc(strlen(desc).wrapping_add(1 as libc::c_int as
                                                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                                    as
                                                                                    libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               strlen(desc).wrapping_add(1 as libc::c_int as
                                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                             as
                                                                             libc::c_ulong))
            as *mut libc::c_char;
    strcpy((*powers_type.offset((power_max as libc::c_int - 1 as libc::c_int)
                                    as isize)).desc_text, desc);
    let ref mut fresh2 =
        (*powers_type.offset((power_max as libc::c_int - 1 as libc::c_int) as
                                 isize)).gain_text;
    *fresh2 =
        memset(ralloc(strlen(gain).wrapping_add(1 as libc::c_int as
                                                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                                    as
                                                                                    libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               strlen(gain).wrapping_add(1 as libc::c_int as
                                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                             as
                                                                             libc::c_ulong))
            as *mut libc::c_char;
    strcpy((*powers_type.offset((power_max as libc::c_int - 1 as libc::c_int)
                                    as isize)).gain_text, gain);
    let ref mut fresh3 =
        (*powers_type.offset((power_max as libc::c_int - 1 as libc::c_int) as
                                 isize)).lose_text;
    *fresh3 =
        memset(ralloc(strlen(lose).wrapping_add(1 as libc::c_int as
                                                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                                    as
                                                                                    libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               strlen(lose).wrapping_add(1 as libc::c_int as
                                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                             as
                                                                             libc::c_ulong))
            as *mut libc::c_char;
    strcpy((*powers_type.offset((power_max as libc::c_int - 1 as libc::c_int)
                                    as isize)).lose_text, lose);
    /* Copy the other stuff */
    (*powers_type.offset((power_max as libc::c_int - 1 as libc::c_int) as
                             isize)).level = level;
    (*powers_type.offset((power_max as libc::c_int - 1 as libc::c_int) as
                             isize)).cost = cost;
    (*powers_type.offset((power_max as libc::c_int - 1 as libc::c_int) as
                             isize)).stat = stat;
    (*powers_type.offset((power_max as libc::c_int - 1 as libc::c_int) as
                             isize)).diff = diff;
    return (power_max as libc::c_int - 1 as libc::c_int) as s16b;
}
static mut lua_item_tester_fct: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn lua_item_tester(mut o_ptr: *mut object_type) -> bool_ {
    let mut oldtop: libc::c_int = lua_gettop(L);
    let mut ret: bool_ = 0;
    lua_getglobal(L, lua_item_tester_fct);
    tolua_pushusertype(L, o_ptr as *mut libc::c_void,
                       tolua_tag(L,
                                 b"object_type\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char));
    lua_call(L, 1 as libc::c_int, 1 as libc::c_int);
    ret = lua_tonumber(L, -(1 as libc::c_int)) as bool_;
    lua_settop(L, oldtop);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn lua_set_item_tester(mut tval: libc::c_int,
                                             mut fct: *mut libc::c_char) {
    if tval != 0 {
        item_tester_tval = tval as byte_hack
    } else {
        lua_item_tester_fct = fct;
        item_tester_hook =
            Some(lua_item_tester as
                     unsafe extern "C" fn(_: *mut object_type) -> bool_)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_object_desc(mut o_ptr: *mut object_type,
                                         mut pref: libc::c_int,
                                         mut mode: libc::c_int)
 -> *mut libc::c_char {
    static mut buf: [libc::c_char; 150] = [0; 150];
    object_desc(buf.as_mut_ptr(), o_ptr, pref, mode);
    return buf.as_mut_ptr();
}
/*
 * Monsters
 */
#[no_mangle]
pub unsafe extern "C" fn find_position(mut y: libc::c_int, mut x: libc::c_int,
                                       mut yy: *mut libc::c_int,
                                       mut xx: *mut libc::c_int) {
    let mut attempts: libc::c_int = 500 as libc::c_int;
    loop  {
        scatter(yy, xx, y, x, 6 as libc::c_int);
        if !(!(*yy > 0 as libc::c_int && *xx > 0 as libc::c_int &&
                   *yy < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   *xx < cur_wid as libc::c_int - 1 as libc::c_int &&
                   ((*f_info.offset((*cave[*yy as
                                               usize].offset(*xx as
                                                                 isize)).feat
                                        as isize)).flags1 as libc::c_long &
                        0x10 as libc::c_long != 0 &&
                        (*cave[*yy as usize].offset(*xx as isize)).feat as
                            libc::c_int != 0xaf as libc::c_int)) &&
                 { attempts -= 1; (attempts) != 0 }) {
            break ;
        }
    };
}
static mut summon_lua_okay_fct: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn summon_lua_okay(mut r_idx: libc::c_int) -> bool_ {
    let mut oldtop: libc::c_int = lua_gettop(L);
    let mut ret: bool_ = 0;
    lua_getglobal(L, lua_item_tester_fct);
    tolua_pushnumber(L, r_idx as libc::c_long);
    lua_call(L, 1 as libc::c_int, 1 as libc::c_int);
    ret = lua_tonumber(L, -(1 as libc::c_int)) as bool_;
    lua_settop(L, oldtop);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn lua_summon_monster(mut y: libc::c_int,
                                            mut x: libc::c_int,
                                            mut lev: libc::c_int,
                                            mut friend_: bool_,
                                            mut fct: *mut libc::c_char)
 -> bool_ {
    summon_lua_okay_fct = fct;
    if friend_ == 0 {
        return summon_specific(y, x, lev, 58 as libc::c_int)
    } else {
        return summon_specific_friendly(y, x, lev, 58 as libc::c_int,
                                        1 as libc::c_int as bool_)
    };
}
/*
 * Quests
 */
#[no_mangle]
pub unsafe extern "C" fn add_new_quest(mut name: *mut libc::c_char) -> s16b {
    let mut i: libc::c_int = 0;
    /* Increase the size */
    reinit_quests((max_q_idx as libc::c_int + 1 as libc::c_int) as s16b);
    (*quest.offset((max_q_idx as libc::c_int - 1 as libc::c_int) as
                       isize)).type_0 = 1 as libc::c_int as byte_hack;
    strncpy((*quest.offset((max_q_idx as libc::c_int - 1 as libc::c_int) as
                               isize)).name.as_mut_ptr(), name,
            39 as libc::c_int as libc::c_ulong);
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        strncpy((*quest.offset((max_q_idx as libc::c_int - 1 as libc::c_int)
                                   as isize)).desc[i as usize].as_mut_ptr(),
                b"\x00" as *const u8 as *const libc::c_char,
                39 as libc::c_int as libc::c_ulong);
        i += 1
    }
    return (max_q_idx as libc::c_int - 1 as libc::c_int) as s16b;
}
#[no_mangle]
pub unsafe extern "C" fn desc_quest(mut q_idx: libc::c_int,
                                    mut d: libc::c_int,
                                    mut desc: *mut libc::c_char) {
    if d >= 0 as libc::c_int && d < 10 as libc::c_int {
        strncpy((*quest.offset(q_idx as isize)).desc[d as usize].as_mut_ptr(),
                desc, 79 as libc::c_int as libc::c_ulong);
    };
}
/*
 * Misc
 */
#[no_mangle]
pub unsafe extern "C" fn get_com_lua(mut prompt: cptr,
                                     mut com: *mut libc::c_int) -> bool_ {
    let mut c: libc::c_char = 0;
    if get_com(prompt, &mut c) == 0 { return 0 as libc::c_int as bool_ }
    *com = c as libc::c_int;
    return 1 as libc::c_int as bool_;
}
/* Spell schools */
#[no_mangle]
pub unsafe extern "C" fn new_school(mut i: libc::c_int, mut name: cptr,
                                    mut skill: s16b) -> s16b {
    let ref mut fresh4 = (*schools.offset(i as isize)).name;
    *fresh4 = string_make(name);
    (*schools.offset(i as isize)).skill = skill;
    return i as s16b;
}
#[no_mangle]
pub unsafe extern "C" fn new_spell(mut i: libc::c_int, mut name: cptr)
 -> s16b {
    let ref mut fresh5 = (*school_spells.offset(i as isize)).name;
    *fresh5 = string_make(name);
    (*school_spells.offset(i as isize)).level = 0 as libc::c_int as s16b;
    (*school_spells.offset(i as isize)).level = 0 as libc::c_int as s16b;
    return i as s16b;
}
#[no_mangle]
pub unsafe extern "C" fn grab_spell_type(mut num: s16b) -> *mut spell_type {
    return &mut *school_spells.offset(num as isize) as *mut spell_type;
}
#[no_mangle]
pub unsafe extern "C" fn grab_school_type(mut num: s16b) -> *mut school_type {
    return &mut *schools.offset(num as isize) as *mut school_type;
}
/* Change this fct if I want to switch to learnable spells */
#[no_mangle]
pub unsafe extern "C" fn lua_get_level(mut s: s32b, mut lvl: s32b,
                                       mut max: s32b, mut min: s32b,
                                       mut bonus: s32b) -> s32b {
    let mut tmp: s32b = 0;
    tmp =
        lvl -
            ((*school_spells.offset(s as isize)).skill_level as libc::c_int -
                 1 as libc::c_int) *
                (1000 as libc::c_int / 10 as libc::c_int);
    if tmp >= 1000 as libc::c_int / 10 as libc::c_int {
        /* We require at least one spell level */
        tmp += bonus
    }
    tmp =
        tmp * (max * (1000 as libc::c_int / 10 as libc::c_int)) /
            (50000 as libc::c_int / 10 as libc::c_int);
    if tmp < 0 as libc::c_int {
        /* Shift all negative values, so they map to appropriate integer */
        tmp -= 1000 as libc::c_int / 10 as libc::c_int - 1 as libc::c_int
    }
    /* Now, we can safely divide */
    lvl = tmp / (1000 as libc::c_int / 10 as libc::c_int);
    if lvl < min { lvl = min }
    return lvl;
}
#[no_mangle]
pub unsafe extern "C" fn lua_spell_chance(mut chance: s32b,
                                          mut level: libc::c_int,
                                          mut skill_level: libc::c_int,
                                          mut mana: libc::c_int,
                                          mut cur_mana: libc::c_int,
                                          mut stat: libc::c_int) -> s32b {
    let mut minfail: libc::c_int = 0;
    /* Reduce failure rate by "effective" level adjustment */
    chance -= 3 as libc::c_int * (level - 1 as libc::c_int);
    /* Reduce failure rate by INT/WIS adjustment */
    chance -=
        3 as libc::c_int *
            (*adj_mag_stat.as_mut_ptr().offset((*p_ptr).stat_ind[stat as
                                                                     usize] as
                                                   isize) as libc::c_int -
                 1 as libc::c_int);
    /* Not enough mana to cast */
    if chance < 0 as libc::c_int { chance = 0 as libc::c_int }
    if mana > cur_mana { chance += 15 as libc::c_int * (mana - cur_mana) }
    /* Extract the minimum failure rate */
    minfail =
        *adj_mag_fail.as_mut_ptr().offset((*p_ptr).stat_ind[stat as usize] as
                                              isize) as libc::c_int;
    /*
	        * Non mage characters never get too good
	 */
    if has_ability(2 as libc::c_int) == 0 {
        if minfail < 5 as libc::c_int { minfail = 5 as libc::c_int }
    }
    /* Hack -- Priest prayer penalty for "edged" weapons  -DGK */
    if forbid_non_blessed() as libc::c_int != 0 &&
           (*p_ptr).icky_wield as libc::c_int != 0 {
        chance += 25 as libc::c_int
    }
    /* Return the chance */
    return clamp_failure_chance(chance, minfail);
}
#[no_mangle]
pub unsafe extern "C" fn lua_spell_device_chance(mut chance: s32b,
                                                 mut level: libc::c_int,
                                                 mut base_level: libc::c_int)
 -> s32b {
    let mut minfail: libc::c_int = 0;
    /* Reduce failure rate by "effective" level adjustment */
    chance -= level - 1 as libc::c_int;
    /* Extract the minimum failure rate */
    minfail =
        15 as libc::c_int -
            get_skill_scale(56 as libc::c_int, 25 as libc::c_int as u32b) as
                libc::c_int;
    /* Return the chance */
    return clamp_failure_chance(chance, minfail);
}
/* Cave */
#[no_mangle]
pub unsafe extern "C" fn lua_get_cave(mut y: libc::c_int, mut x: libc::c_int)
 -> *mut cave_type {
    return &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
               *mut cave_type;
}
#[no_mangle]
pub unsafe extern "C" fn set_target(mut y: libc::c_int, mut x: libc::c_int) {
    target_who = -(1 as libc::c_int) as s16b;
    target_col = x as s16b;
    target_row = y as s16b;
}
#[no_mangle]
pub unsafe extern "C" fn get_target(mut dir: libc::c_int,
                                    mut y: *mut libc::c_int,
                                    mut x: *mut libc::c_int) {
    let mut ty: libc::c_int = 0;
    let mut tx: libc::c_int = 0;
    /* Use the given direction */
    tx =
        (*p_ptr).px as libc::c_int +
            ddx[dir as usize] as libc::c_int * 100 as libc::c_int;
    ty =
        (*p_ptr).py as libc::c_int +
            ddy[dir as usize] as libc::c_int * 100 as libc::c_int;
    /* Hack -- Use an actual "target" */
    if dir == 5 as libc::c_int && target_okay() as libc::c_int != 0 {
        tx = target_col as libc::c_int;
        ty = target_row as libc::c_int
    }
    *y = ty;
    *x = tx;
}
/* Level gen */
#[no_mangle]
pub unsafe extern "C" fn get_map_size(mut name: *mut libc::c_char,
                                      mut ysize: *mut libc::c_int,
                                      mut xsize: *mut libc::c_int) {
    *xsize = 0 as libc::c_int;
    *ysize = 0 as libc::c_int;
    init_flags = 0x8 as libc::c_int;
    process_dungeon_file(name as cptr, ysize, xsize, cur_hgt as libc::c_int,
                         cur_wid as libc::c_int, 1 as libc::c_int as bool_,
                         1 as libc::c_int as bool_);
}
#[no_mangle]
pub unsafe extern "C" fn load_map(mut name: *mut libc::c_char,
                                  mut y: *mut libc::c_int,
                                  mut x: *mut libc::c_int) {
    /* Set the correct monster hook */
    set_mon_num_hook();
    /* Prepare allocation table */
    get_mon_num_prep();
    init_flags = 0x4 as libc::c_int;
    process_dungeon_file(name as cptr, y, x, cur_hgt as libc::c_int,
                         cur_wid as libc::c_int, 1 as libc::c_int as bool_,
                         1 as libc::c_int as bool_);
}
#[no_mangle]
pub unsafe extern "C" fn alloc_room(mut by0: libc::c_int,
                                    mut bx0: libc::c_int,
                                    mut ysize: libc::c_int,
                                    mut xsize: libc::c_int,
                                    mut y1: *mut libc::c_int,
                                    mut x1: *mut libc::c_int,
                                    mut y2: *mut libc::c_int,
                                    mut x2: *mut libc::c_int) -> bool_ {
    let mut xval: libc::c_int = 0;
    let mut yval: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    /* Try to allocate space for room.  If fails, exit */
    if room_alloc(xsize + 2 as libc::c_int, ysize + 2 as libc::c_int,
                  0 as libc::c_int as bool_, by0, bx0, &mut xval, &mut yval)
           == 0 {
        return 0 as libc::c_int as bool_
    }
    /* Get corner values */
    *y1 = yval - ysize / 2 as libc::c_int;
    *x1 = xval - xsize / 2 as libc::c_int;
    *y2 = yval + ysize / 2 as libc::c_int;
    *x2 = xval + xsize / 2 as libc::c_int;
    /* Place a full floor under the room */
    y = *y1 - 1 as libc::c_int;
    while y <= *y2 + 1 as libc::c_int {
        x = *x1 - 1 as libc::c_int;
        while x <= *x2 + 1 as libc::c_int {
            let mut c_ptr: *mut cave_type =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type;
            cave_set_feat(y, x,
                          floor_type[Rand_div(100 as libc::c_int) as usize] as
                              libc::c_int);
            (*c_ptr).info =
                ((*c_ptr).info as libc::c_int | 0x8 as libc::c_int) as u16b;
            (*c_ptr).info =
                ((*c_ptr).info as libc::c_int | 0x2 as libc::c_int) as u16b;
            x += 1
        }
        y += 1
    }
    return 1 as libc::c_int as bool_;
}
/* Files */
#[no_mangle]
pub unsafe extern "C" fn lua_print_hook(mut str: cptr) {
    fprintf(hook_file, b"%s\x00" as *const u8 as *const libc::c_char, str);
}
/*
 * Finds a good random bounty monster
 * Im too lazy to write it in lua since the lua API for monsters is not very well yet
 */
/*
 * Hook for bounty monster selection.
 */
unsafe extern "C" fn lua_mon_hook_bounty(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    /* Reject uniques */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Reject those who cannot leave anything */
    if (*r_ptr).flags9 & 0x1 as libc::c_int as libc::c_uint == 0 {
        return 0 as libc::c_int as bool_
    }
    /* Accept only monsters that can be generated */
    if (*r_ptr).flags9 & 0x2000 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    if (*r_ptr).flags9 & 0x4000 as libc::c_int as libc::c_uint != 0 {
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
    /* Accept only monsters that are not breeders */
    if (*r_ptr).flags4 & 0x2 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Forbid joke monsters */
    if (*r_ptr).flags8 & 0x8000 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Accept only monsters that are not good */
    if (*r_ptr).flags3 & 0x200 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* The rest are acceptable */
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn lua_get_new_bounty_monster(mut lev: libc::c_int)
 -> libc::c_int {
    let mut r_idx: libc::c_int = 0;
    /*
	 * Set up the hooks -- no bounties on uniques or monsters
	 * with no corpses
	 */
    get_mon_num_hook =
        Some(lua_mon_hook_bounty as
                 unsafe extern "C" fn(_: libc::c_int) -> bool_);
    get_mon_num_prep();
    /* Set up the quest monster. */
    r_idx = get_mon_num(lev) as libc::c_int;
    /* Undo the filters */
    get_mon_num_hook = None;
    get_mon_num_prep();
    return r_idx;
}
/*
 * Some misc functions
 */
#[no_mangle]
pub unsafe extern "C" fn lua_input_box(mut title: cptr, mut max: libc::c_int)
 -> *mut libc::c_char {
    static mut buf: [libc::c_char; 80] = [0; 80];
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    strcpy(buf.as_mut_ptr(), b"\x00" as *const u8 as *const libc::c_char);
    Term_get_size(&mut wid, &mut hgt);
    if input_box(title, hgt / 2 as libc::c_int, wid / 2 as libc::c_int,
                 buf.as_mut_ptr(),
                 if max > 79 as libc::c_int {
                     79 as libc::c_int
                 } else { max }) == 0 {
        return b"\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    return buf.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn lua_msg_box(mut title: cptr) -> libc::c_char {
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    Term_get_size(&mut wid, &mut hgt);
    return msg_box(title, hgt / 2 as libc::c_int, wid / 2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn lua_create_list(mut size: libc::c_int)
 -> *mut list_type {
    let mut l: *mut list_type = 0 as *mut list_type;
    let mut list: *mut cptr = 0 as *mut cptr;
    l =
        memset(ralloc(::std::mem::size_of::<list_type>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<list_type>() as libc::c_ulong) as
            *mut list_type;
    list =
        memset(ralloc((size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<cptr>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<cptr>()
                                                    as libc::c_ulong)) as
            *mut cptr;
    (*l).list = list;
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn lua_delete_list(mut l: *mut list_type,
                                         mut size: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size { string_free(*(*l).list.offset(i as isize)); i += 1 }
    rnfree((*l).list as vptr,
           (size as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<cptr>() as
                                                libc::c_ulong));
    rnfree(l as vptr, ::std::mem::size_of::<list_type>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn lua_add_to_list(mut l: *mut list_type,
                                         mut idx: libc::c_int,
                                         mut str: cptr) {
    let ref mut fresh6 = *(*l).list.offset(idx as isize);
    *fresh6 = string_make(str);
}
#[no_mangle]
pub unsafe extern "C" fn lua_display_list(mut y: libc::c_int,
                                          mut x: libc::c_int,
                                          mut h: libc::c_int,
                                          mut w: libc::c_int, mut title: cptr,
                                          mut list: *mut list_type,
                                          mut max: libc::c_int,
                                          mut begin: libc::c_int,
                                          mut sel: libc::c_int,
                                          mut sel_color: byte_hack) {
    display_list(y, x, h, w, title, (*list).list, max, begin, sel, sel_color);
}
/*
 * Gods
 */
#[no_mangle]
pub unsafe extern "C" fn add_new_gods(mut name: *mut libc::c_char) -> s16b {
    let mut i: libc::c_int = 0;
    /* Increase the size */
    reinit_gods((max_gods + 1 as libc::c_int) as s16b);
    let ref mut fresh7 =
        (*deity_info.offset((max_gods - 1 as libc::c_int) as isize)).name;
    *fresh7 = string_make(name as cptr);
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        strncpy((*deity_info.offset((max_gods - 1 as libc::c_int) as
                                        isize)).desc[i as usize].as_mut_ptr(),
                b"\x00" as *const u8 as *const libc::c_char,
                39 as libc::c_int as libc::c_ulong);
        i += 1
    }
    return (max_gods - 1 as libc::c_int) as s16b;
}
#[no_mangle]
pub unsafe extern "C" fn desc_god(mut g_idx: libc::c_int, mut d: libc::c_int,
                                  mut desc: *mut libc::c_char) {
    if d >= 0 as libc::c_int && d < 10 as libc::c_int {
        strncpy((*deity_info.offset(g_idx as
                                        isize)).desc[d as usize].as_mut_ptr(),
                desc, 79 as libc::c_int as libc::c_ulong);
    };
}
/*
 * Returns the direction of the compass that y2, x2 is from y, x
 * the return value will be one of the following: north, south,
 * east, west, north-east, south-east, south-west, north-west,
 * or "close" if it is within 2 tiles.
 */
#[no_mangle]
pub unsafe extern "C" fn compass(mut y: libc::c_int, mut x: libc::c_int,
                                 mut y2: libc::c_int, mut x2: libc::c_int)
 -> cptr {
    static mut compass_dir: [libc::c_char; 64] = [0; 64];
    // is it close to the north/south meridian?
    let mut y_diff: libc::c_int = y2 - y;
    // determine if y2, x2 is to the north or south of y, x
    let mut y_axis: *const libc::c_char = 0 as *const libc::c_char;
    if y_diff > -(3 as libc::c_int) && y_diff < 3 as libc::c_int {
        y_axis = 0 as *const libc::c_char
    } else if y2 > y {
        y_axis = b"south\x00" as *const u8 as *const libc::c_char
    } else { y_axis = b"north\x00" as *const u8 as *const libc::c_char }
    // is it close to the east/west meridian?
    let mut x_diff: libc::c_int = x2 - x;
    // determine if y2, x2 is to the east or west of y, x
    let mut x_axis: *const libc::c_char = 0 as *const libc::c_char;
    if x_diff > -(3 as libc::c_int) && x_diff < 3 as libc::c_int {
        x_axis = 0 as *const libc::c_char
    } else if x2 > x {
        x_axis = b"east\x00" as *const u8 as *const libc::c_char
    } else { x_axis = b"west\x00" as *const u8 as *const libc::c_char }
    // Maybe it is very close
    if x_axis.is_null() && y_axis.is_null() {
        strcpy(compass_dir.as_mut_ptr(),
               b"close\x00" as *const u8 as *const libc::c_char);
    } else if x_axis.is_null() {
        strcpy(compass_dir.as_mut_ptr(), y_axis);
    } else if y_axis.is_null() {
        strcpy(compass_dir.as_mut_ptr(), x_axis);
    } else {
        // Maybe it is (almost) due N/S
        // Maybe it is (almost) due E/W
        //  or if it is neither
        sprintf(compass_dir.as_mut_ptr(),
                b"%s-%s\x00" as *const u8 as *const libc::c_char, y_axis,
                x_axis);
    }
    return compass_dir.as_mut_ptr() as cptr;
}
/* Returns a relative approximation of the 'distance' of y2, x2 from y, x. */
#[no_mangle]
pub unsafe extern "C" fn approximate_distance(mut y: libc::c_int,
                                              mut x: libc::c_int,
                                              mut y2: libc::c_int,
                                              mut x2: libc::c_int) -> cptr {
    //  how far to away to the north/south?
    let mut y_diff: libc::c_int = abs(y2 - y);
    // how far to away to the east/west?
    let mut x_diff: libc::c_int = abs(x2 - x);
    // find which one is the larger distance
    let mut most_dist: libc::c_int = x_diff;
    if y_diff > most_dist { most_dist = y_diff }
    // how far away then?
    if most_dist >= 41 as libc::c_int {
        return b"a very long way\x00" as *const u8 as *const libc::c_char
    } else if most_dist >= 25 as libc::c_int {
        return b"a long way\x00" as *const u8 as *const libc::c_char
    } else if most_dist >= 8 as libc::c_int {
        return b"quite some way\x00" as *const u8 as *const libc::c_char
    } else { return b"not very far\x00" as *const u8 as *const libc::c_char };
}
#[no_mangle]
pub unsafe extern "C" fn drop_text_left(mut c: byte_hack, mut str: cptr,
                                        mut y: libc::c_int,
                                        mut o: libc::c_int) -> bool_ {
    let mut i: libc::c_int = strlen(str) as libc::c_int;
    let mut x: libc::c_int =
        (39 as libc::c_int as
             libc::c_ulong).wrapping_sub(strlen(str).wrapping_div(2 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong)).wrapping_add(o
                                                                                                       as
                                                                                                       libc::c_ulong)
            as libc::c_int;
    while i > 0 as libc::c_int {
        let mut a: libc::c_int = 0 as libc::c_int;
        let mut time: libc::c_int = 0 as libc::c_int;
        if *str.offset((i - 1 as libc::c_int) as isize) as libc::c_int !=
               ' ' as i32 {
            while a < x + i - 1 as libc::c_int {
                Term_putch(a - 1 as libc::c_int, y, c,
                           32 as libc::c_int as libc::c_char);
                Term_putch(a, y, c,
                           *str.offset((i - 1 as libc::c_int) as isize));
                time = time + 1 as libc::c_int;
                if time >= 4 as libc::c_int {
                    Term_xtra(13 as libc::c_int, 1 as libc::c_int);
                    time = 0 as libc::c_int
                }
                Term_redraw_section(a - 1 as libc::c_int, y, a, y);
                a = a + 1 as libc::c_int;
                inkey_scan = 1 as libc::c_int as bool_;
                if inkey() != 0 { return 1 as libc::c_int as bool_ }
            }
        }
        i = i - 1 as libc::c_int
    }
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn drop_text_right(mut c: byte_hack, mut str: cptr,
                                         mut y: libc::c_int,
                                         mut o: libc::c_int) -> bool_ {
    let mut x: libc::c_int =
        (39 as libc::c_int as
             libc::c_ulong).wrapping_sub(strlen(str).wrapping_div(2 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong)).wrapping_add(o
                                                                                                       as
                                                                                                       libc::c_ulong)
            as libc::c_int;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i as libc::c_ulong <= strlen(str) {
        let mut a: libc::c_int = 79 as libc::c_int;
        let mut time: libc::c_int = 0 as libc::c_int;
        if *str.offset((i - 1 as libc::c_int) as isize) as libc::c_int !=
               ' ' as i32 {
            while a >= x + i - 1 as libc::c_int {
                Term_putch(a + 1 as libc::c_int, y, c,
                           32 as libc::c_int as libc::c_char);
                Term_putch(a, y, c,
                           *str.offset((i - 1 as libc::c_int) as isize));
                time = time + 1 as libc::c_int;
                if time >= 4 as libc::c_int {
                    Term_xtra(13 as libc::c_int, 1 as libc::c_int);
                    time = 0 as libc::c_int
                }
                Term_redraw_section(a, y, a + 1 as libc::c_int, y);
                a = a - 1 as libc::c_int;
                inkey_scan = 1 as libc::c_int as bool_;
                if inkey() != 0 { return 1 as libc::c_int as bool_ }
            }
        }
        i = i + 1 as libc::c_int
    }
    return 0 as libc::c_int as bool_;
}
