use ::libc;
extern "C" {
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut use_bigtile: bool_;
    #[no_mangle]
    static mut depth_in_feet: bool_;
    #[no_mangle]
    static mut show_details: bool_;
    #[no_mangle]
    static mut cheat_know: bool_;
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    static mut Term: *mut term;
    #[no_mangle]
    fn Term_gotoxy(x: libc::c_int, y: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_addch(a: byte_hack, c: libc::c_char) -> errr;
    #[no_mangle]
    fn Term_addstr(n: libc::c_int, a: byte_hack, s: cptr) -> errr;
    #[no_mangle]
    fn Term_erase(x: libc::c_int, y: libc::c_int, n: libc::c_int) -> errr;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut r_name: *mut libc::c_char;
    #[no_mangle]
    static mut r_text: *mut libc::c_char;
    #[no_mangle]
    static mut re_info: *mut monster_ego;
    #[no_mangle]
    static mut re_name: *mut libc::c_char;
    #[no_mangle]
    static mut wf_info: *mut wilderness_type_info;
    #[no_mangle]
    static mut get_mon_num_hook:
           Option<unsafe extern "C" fn(_: libc::c_int) -> bool_>;
    #[no_mangle]
    static mut get_mon_num2_hook:
           Option<unsafe extern "C" fn(_: libc::c_int) -> bool_>;
    #[no_mangle]
    static mut wild_map: *mut *mut wilderness_map;
    #[no_mangle]
    fn race_info_idx(r_idx: libc::c_int, ego: libc::c_int)
     -> *mut monster_race;
    #[no_mangle]
    fn text_out(str: cptr);
    #[no_mangle]
    fn text_out_c(a: byte_hack, str: cptr);
    #[no_mangle]
    fn msg_print(msg: cptr);
}
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
/* File: monster1.c */
/* Purpose: describe monsters (using monster memory) */
/*
 * Copyright (c) 1989 James E. Wilson, Christopher J. Stuart
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Pronoun arrays, by gender.
 */
static mut wd_he: [cptr; 3] =
    [b"it\x00" as *const u8 as *const libc::c_char,
     b"he\x00" as *const u8 as *const libc::c_char,
     b"she\x00" as *const u8 as *const libc::c_char];
static mut wd_his: [cptr; 3] =
    [b"its\x00" as *const u8 as *const libc::c_char,
     b"his\x00" as *const u8 as *const libc::c_char,
     b"her\x00" as *const u8 as *const libc::c_char];
/*
 * Determine if the "armor" is known
 * The higher the level, the fewer kills needed.
 */
unsafe extern "C" fn know_armour(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    let mut level: s32b = (*r_ptr).level as s32b;
    let mut kills: s32b = (*r_ptr).r_tkills as s32b;
    /* Normal monsters */
    if kills > 304 as libc::c_int / (4 as libc::c_int + level) {
        return 1 as libc::c_int as bool_
    }
    /* Skip non-uniques */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint == 0 {
        return 0 as libc::c_int as bool_
    }
    /* Unique monsters */
    if kills >
           304 as libc::c_int /
               (38 as libc::c_int +
                    5 as libc::c_int * level / 4 as libc::c_int) {
        return 1 as libc::c_int as bool_
    }
    /* Assume false */
    return 0 as libc::c_int as bool_;
}
/*
 * Determine if the "damage" of the given attack is known
 * the higher the level of the monster, the fewer the attacks you need,
 * the more damage an attack does, the more attacks you need
 */
unsafe extern "C" fn know_damage(mut r_idx: libc::c_int, mut i: libc::c_int)
 -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    let mut level: s32b = (*r_ptr).level as s32b;
    let mut a: s32b = (*r_ptr).r_blows[i as usize] as s32b;
    let mut d1: s32b = (*r_ptr).blow[i as usize].d_dice as s32b;
    let mut d2: s32b = (*r_ptr).blow[i as usize].d_side as s32b;
    let mut d: s32b = d1 * d2;
    /* Normal monsters */
    if (4 as libc::c_int + level) * a > 80 as libc::c_int * d {
        return 1 as libc::c_int as bool_
    }
    /* Skip non-uniques */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint == 0 {
        return 0 as libc::c_int as bool_
    }
    /* Unique monsters */
    if (4 as libc::c_int + level) * (2 as libc::c_int * a) >
           80 as libc::c_int * d {
        return 1 as libc::c_int as bool_
    }
    /* Assume false */
    return 0 as libc::c_int as bool_;
}
/*
 * Hack -- display monster information using "text_out()"
 *
 * Note that there is now a compiler option to only read the monster
 * descriptions from the raw file when they are actually needed, which
 * saves about 60K of memory at the cost of disk access during monster
 * recall, which is optional to the user.
 *
 * This function should only be called with the cursor placed at the
 * left edge of the screen, on a cleared line, in which the recall is
 * to take place.  One extra blank line is left after the recall.
 */
unsafe extern "C" fn roff_aux(mut r_idx: libc::c_int, mut ego: libc::c_int,
                              mut remem: libc::c_int) {
    let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
    let mut old: bool_ = 0 as libc::c_int as bool_;
    let mut sin: bool_ = 0 as libc::c_int as bool_;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut p: cptr = 0 as *const libc::c_char;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut msex: libc::c_int = 0 as libc::c_int;
    let mut breath: bool_ = 0 as libc::c_int as bool_;
    let mut magic: bool_ = 0 as libc::c_int as bool_;
    let mut flags1: u32b = 0;
    let mut flags2: u32b = 0;
    let mut flags3: u32b = 0;
    let mut flags4: u32b = 0;
    let mut flags5: u32b = 0;
    let mut flags6: u32b = 0;
    let mut flags7: u32b = 0;
    let mut flags9: u32b = 0;
    let mut vn: libc::c_int = 0 as libc::c_int;
    let mut color: [byte_hack; 64] = [0; 64];
    let mut vp: [cptr; 64] = [0 as *const libc::c_char; 64];
    let mut save_mem: monster_race =
        monster_race{name: 0,
                     text: 0,
                     hdice: 0,
                     hside: 0,
                     ac: 0,
                     sleep: 0,
                     aaf: 0,
                     speed: 0,
                     mexp: 0,
                     weight: 0,
                     freq_inate: 0,
                     freq_spell: 0,
                     flags1: 0,
                     flags2: 0,
                     flags3: 0,
                     flags4: 0,
                     flags5: 0,
                     flags6: 0,
                     flags7: 0,
                     flags8: 0,
                     flags9: 0,
                     blow:
                         [monster_blow{method: 0,
                                       effect: 0,
                                       d_dice: 0,
                                       d_side: 0,}; 4],
                     body_parts: [0; 6],
                     level: 0,
                     rarity: 0,
                     d_attr: 0,
                     d_char: 0,
                     x_attr: 0,
                     x_char: 0,
                     max_num: 0,
                     cur_num: 0,
                     r_sights: 0,
                     r_deaths: 0,
                     r_pkills: 0,
                     r_tkills: 0,
                     r_wake: 0,
                     r_ignore: 0,
                     r_xtra1: 0,
                     r_xtra2: 0,
                     r_drop_gold: 0,
                     r_drop_item: 0,
                     r_cast_inate: 0,
                     r_cast_spell: 0,
                     r_blows: [0; 4],
                     r_flags1: 0,
                     r_flags2: 0,
                     r_flags3: 0,
                     r_flags4: 0,
                     r_flags5: 0,
                     r_flags6: 0,
                     r_flags7: 0,
                     r_flags8: 0,
                     r_flags9: 0,
                     on_saved: 0,
                     total_visible: 0,
                     drops:
                         obj_theme{treasure: 0,
                                   combat: 0,
                                   magic: 0,
                                   tools: 0,},};
    /* Access the race and lore */
    r_ptr = race_info_idx(r_idx, ego);
    /* Cheat -- Know everything */
    if cheat_know != 0 {
        /* XXX XXX XXX */
        /* Save the "old" memory */
        save_mem = *r_ptr;
        /* Hack -- Maximal kills */
        (*r_ptr).r_tkills = 32767 as libc::c_int as s16b;
        /* Hack -- Maximal info */
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
        (*r_ptr).r_flags7 = (*r_ptr).flags7;
        (*r_ptr).r_flags8 = (*r_ptr).flags8;
        (*r_ptr).r_flags9 = (*r_ptr).flags9
    }
    /* Extract a gender (if applicable) */
    if (*r_ptr).flags1 & 0x8 as libc::c_int as libc::c_uint != 0 {
        msex = 2 as libc::c_int
    } else if (*r_ptr).flags1 & 0x4 as libc::c_int as libc::c_uint != 0 {
        msex = 1 as libc::c_int
    }
    /* Obtain a copy of the "known" flags */
    flags1 = (*r_ptr).flags1 & (*r_ptr).r_flags1;
    flags2 = (*r_ptr).flags2 & (*r_ptr).r_flags2;
    flags3 = (*r_ptr).flags3 & (*r_ptr).r_flags3;
    flags4 = (*r_ptr).flags4 & (*r_ptr).r_flags4;
    flags5 = (*r_ptr).flags5 & (*r_ptr).r_flags5;
    flags6 = (*r_ptr).flags6 & (*r_ptr).r_flags6;
    flags7 = (*r_ptr).flags7 & (*r_ptr).r_flags7;
    flags9 = (*r_ptr).flags9 & (*r_ptr).r_flags9;
    /* Assume some "obvious" flags */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        flags1 |= 0x1 as libc::c_int as libc::c_uint
    }
    if (*r_ptr).flags1 & 0x4 as libc::c_int as libc::c_uint != 0 {
        flags1 |= 0x4 as libc::c_int as libc::c_uint
    }
    if (*r_ptr).flags1 & 0x8 as libc::c_int as libc::c_uint != 0 {
        flags1 |= 0x8 as libc::c_int as libc::c_uint
    }
    /* Assume some "creation" flags */
    if (*r_ptr).flags1 & 0x1000 as libc::c_int as libc::c_uint != 0 {
        flags1 |= 0x1000 as libc::c_int as libc::c_uint
    }
    if (*r_ptr).flags1 & 0x2000 as libc::c_int as libc::c_uint != 0 {
        flags1 |= 0x2000 as libc::c_int as libc::c_uint
    }
    if (*r_ptr).flags1 & 0x4000 as libc::c_int as libc::c_uint != 0 {
        flags1 |= 0x4000 as libc::c_int as libc::c_uint
    }
    if (*r_ptr).flags1 & 0x8000 as libc::c_int as libc::c_uint != 0 {
        flags1 |= 0x8000 as libc::c_int as libc::c_uint
    }
    /* Killing a monster reveals some properties */
    if (*r_ptr).r_tkills != 0 {
        /* Know "race" flags */
        if (*r_ptr).flags3 & 0x1 as libc::c_int as libc::c_uint != 0 {
            flags3 |= 0x1 as libc::c_int as libc::c_uint
        }
        if (*r_ptr).flags3 & 0x2 as libc::c_int as libc::c_uint != 0 {
            flags3 |= 0x2 as libc::c_int as libc::c_uint
        }
        if (*r_ptr).flags3 & 0x4 as libc::c_int as libc::c_uint != 0 {
            flags3 |= 0x4 as libc::c_int as libc::c_uint
        }
        if (*r_ptr).flags3 & 0x8 as libc::c_int as libc::c_uint != 0 {
            flags3 |= 0x8 as libc::c_int as libc::c_uint
        }
        if (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0 {
            flags3 |= 0x10 as libc::c_int as libc::c_uint
        }
        if (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0 {
            flags3 |= 0x20 as libc::c_int as libc::c_uint
        }
        if (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint != 0 {
            flags3 |= 0x40 as libc::c_int as libc::c_uint
        }
        if (*r_ptr).flags3 & 0x200 as libc::c_int as libc::c_uint != 0 {
            flags3 |= 0x200 as libc::c_int as libc::c_uint
        }
        if (*r_ptr).flags3 & 0x80 as libc::c_int as libc::c_uint != 0 {
            flags3 |= 0x80 as libc::c_int as libc::c_uint
        }
        if (*r_ptr).flags3 & 0x100 as libc::c_int as libc::c_uint != 0 {
            flags3 |= 0x100 as libc::c_int as libc::c_uint
        }
        if (*r_ptr).flags7 & 0x40 as libc::c_int as libc::c_uint != 0 {
            flags7 |= 0x40 as libc::c_int as libc::c_uint
        }
        /* Know "forced" flags */
        if (*r_ptr).flags1 & 0x100 as libc::c_int as libc::c_uint != 0 {
            flags1 |= 0x100 as libc::c_int as libc::c_uint
        }
        if (*r_ptr).flags1 & 0x200 as libc::c_int as libc::c_uint != 0 {
            flags1 |= 0x200 as libc::c_int as libc::c_uint
        }
    }
    /* Require a flag to show kills */
    if !(show_details == 0) {
        /* Treat uniques differently */
        if flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
            /* Hack -- Determine if the unique is "dead" */
            let mut dead: bool_ =
                if (*r_ptr).max_num as libc::c_int == 0 as libc::c_int {
                    1 as libc::c_int
                } else { 0 as libc::c_int } as bool_;
            /* We've been killed... */
            if (*r_ptr).r_deaths != 0 {
                /* Killed ancestors */
                text_out(format(b"%^s has slain %d of your ancestors\x00" as
                                    *const u8 as *const libc::c_char,
                                wd_he[msex as usize],
                                (*r_ptr).r_deaths as libc::c_int) as cptr);
                /* But we've also killed it */
                if dead != 0 {
                    text_out(format(b", but you have avenged them!  \x00" as
                                        *const u8 as *const libc::c_char) as
                                 cptr);
                } else {
                    /* Unavenged (ever) */
                    text_out(format(b", who %s unavenged.  \x00" as *const u8
                                        as *const libc::c_char,
                                    if (*r_ptr).r_deaths as libc::c_int ==
                                           1 as libc::c_int {
                                        b"remains\x00" as *const u8 as
                                            *const libc::c_char
                                    } else {
                                        b"remain\x00" as *const u8 as
                                            *const libc::c_char
                                    }) as cptr);
                }
            } else if dead != 0 {
                text_out(b"You have slain this foe.  \x00" as *const u8 as
                             *const libc::c_char);
            }
        } else if (*r_ptr).r_deaths != 0 {
            /* Dead unique who never hurt us */
            /* Not unique, but killed us */
            /* Dead ancestors */
            text_out(format(b"%d of your ancestors %s been killed by this creature, \x00"
                                as *const u8 as *const libc::c_char,
                            (*r_ptr).r_deaths as libc::c_int,
                            if (*r_ptr).r_deaths as libc::c_int ==
                                   1 as libc::c_int {
                                b"has\x00" as *const u8 as *const libc::c_char
                            } else {
                                b"have\x00" as *const u8 as
                                    *const libc::c_char
                            }) as cptr);
            /* Some kills this life */
            if (*r_ptr).r_pkills != 0 {
                text_out(b"and you have exterminated at least \x00" as
                             *const u8 as *const libc::c_char);
                text_out_c(13 as libc::c_int as byte_hack,
                           format(b"%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  (*r_ptr).r_pkills as libc::c_int) as cptr);
                text_out(b" of the creatures.  \x00" as *const u8 as
                             *const libc::c_char);
            } else if (*r_ptr).r_tkills != 0 {
                text_out(format(b"and %s have exterminated at least %d of the creatures.  \x00"
                                    as *const u8 as *const libc::c_char,
                                b"your ancestors\x00" as *const u8 as
                                    *const libc::c_char,
                                (*r_ptr).r_tkills as libc::c_int) as cptr);
            } else {
                /* Some kills past lives */
                /* No kills */
                text_out(format(b"and %s is not ever known to have been defeated.  \x00"
                                    as *const u8 as *const libc::c_char,
                                wd_he[msex as usize]) as cptr);
            }
        } else if (*r_ptr).r_pkills != 0 {
            text_out(b"You have killed at least \x00" as *const u8 as
                         *const libc::c_char);
            text_out_c(13 as libc::c_int as byte_hack,
                       format(b"%d\x00" as *const u8 as *const libc::c_char,
                              (*r_ptr).r_pkills as libc::c_int) as cptr);
            text_out(b" of these creatures.  \x00" as *const u8 as
                         *const libc::c_char);
        } else if (*r_ptr).r_tkills != 0 {
            text_out(format(b"Your ancestors have killed at least %d of these creatures.  \x00"
                                as *const u8 as *const libc::c_char,
                            (*r_ptr).r_tkills as libc::c_int) as cptr);
        } else {
            /* Normal monsters */
            /* Killed some this life */
            /* Killed some last life */
            /* Killed none */
            text_out(b"No battles to the death are recalled.  \x00" as
                         *const u8 as *const libc::c_char);
        }
    }
    /* Descriptions */
    if show_details != 0 {
        let mut buf: [libc::c_char; 2048] = [0; 2048];
        /* Simple method */
        strcpy(buf.as_mut_ptr(), r_text.offset((*r_ptr).text as isize));
        /* Dump it */
        text_out(buf.as_mut_ptr() as cptr);
        text_out(b"  \x00" as *const u8 as *const libc::c_char);
    }
    /* Nothing yet */
    old = 0 as libc::c_int as bool_;
    /* Describe location */
    if (*r_ptr).flags7 & 0x10 as libc::c_int as libc::c_uint != 0 {
        text_out(format(b"%^s is \x00" as *const u8 as *const libc::c_char,
                        wd_he[msex as usize]) as cptr);
        text_out_c(14 as libc::c_int as byte_hack,
                   b"friendly\x00" as *const u8 as *const libc::c_char);
        text_out(b" to you\x00" as *const u8 as *const libc::c_char);
        old = 1 as libc::c_int as bool_
    }
    /* Describe location */
    if (*r_ptr).level as libc::c_int == 0 as libc::c_int {
        if old != 0 {
            text_out(b", \x00" as *const u8 as *const libc::c_char);
        } else {
            text_out(format(b"%^s \x00" as *const u8 as *const libc::c_char,
                            wd_he[msex as usize]) as cptr);
        }
        text_out_c(13 as libc::c_int as byte_hack,
                   b"lives in the town or the wilderness\x00" as *const u8 as
                       *const libc::c_char);
        old = 1 as libc::c_int as bool_
    } else if (*r_ptr).r_tkills != 0 {
        if old != 0 {
            text_out(b", \x00" as *const u8 as *const libc::c_char);
        } else {
            text_out(format(b"%^s \x00" as *const u8 as *const libc::c_char,
                            wd_he[msex as usize]) as cptr);
        }
        if depth_in_feet != 0 {
            text_out(format(b"is normally found at depths of \x00" as
                                *const u8 as *const libc::c_char,
                            wd_he[msex as usize]) as cptr);
            if (dun_level as libc::c_int) < (*r_ptr).level as libc::c_int {
                /* out of depth monster */
                text_out_c(12 as libc::c_int as byte_hack,
                           format(b"%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  (*r_ptr).level as libc::c_int *
                                      50 as libc::c_int) as cptr);
            } else {
                text_out_c(13 as libc::c_int as byte_hack,
                           format(b"%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  (*r_ptr).level as libc::c_int *
                                      50 as libc::c_int) as cptr);
            }
            text_out(b" feet\x00" as *const u8 as *const libc::c_char);
        } else {
            text_out(format(b"is normally found on level \x00" as *const u8 as
                                *const libc::c_char, wd_he[msex as usize]) as
                         cptr);
            if (dun_level as libc::c_int) < (*r_ptr).level as libc::c_int {
                /* out of depth monster */
                text_out_c(12 as libc::c_int as byte_hack,
                           format(b"%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  (*r_ptr).level as libc::c_int) as cptr);
            } else {
                text_out_c(13 as libc::c_int as byte_hack,
                           format(b"%d\x00" as *const u8 as
                                      *const libc::c_char,
                                  (*r_ptr).level as libc::c_int) as cptr);
            }
        }
        old = 1 as libc::c_int as bool_
    }
    /* Describe movement */
    /* Introduction */
    if old != 0 {
        text_out(b", and \x00" as *const u8 as *const libc::c_char);
    } else {
        text_out(format(b"%^s \x00" as *const u8 as *const libc::c_char,
                        wd_he[msex as usize]) as cptr);
        old = 1 as libc::c_int as bool_
    }
    text_out(b"moves\x00" as *const u8 as *const libc::c_char);
    if flags1 & 0x80000 as libc::c_int as libc::c_uint != 0 ||
           flags1 & 0x40000 as libc::c_int as libc::c_uint != 0 {
        /* Adverb */
        if flags1 & 0x80000 as libc::c_int as libc::c_uint != 0 &&
               flags1 & 0x40000 as libc::c_int as libc::c_uint != 0 {
            text_out(b" extremely\x00" as *const u8 as *const libc::c_char);
        } else if flags1 & 0x80000 as libc::c_int as libc::c_uint != 0 {
            text_out(b" somewhat\x00" as *const u8 as *const libc::c_char);
        } else if flags1 & 0x40000 as libc::c_int as libc::c_uint != 0 {
            text_out(b" a bit\x00" as *const u8 as *const libc::c_char);
        }
        /* Adjective */
        text_out(b" erratically\x00" as *const u8 as *const libc::c_char);
        /* Hack -- Occasional conjunction */
        if (*r_ptr).speed as libc::c_int != 110 as libc::c_int {
            text_out(b", and\x00" as *const u8 as *const libc::c_char);
        }
    }
    if (*r_ptr).speed as libc::c_int > 110 as libc::c_int {
        if (*r_ptr).speed as libc::c_int > 130 as libc::c_int {
            text_out_c(4 as libc::c_int as byte_hack,
                       b" incredibly\x00" as *const u8 as
                           *const libc::c_char);
        } else if (*r_ptr).speed as libc::c_int > 120 as libc::c_int {
            text_out_c(3 as libc::c_int as byte_hack,
                       b" very\x00" as *const u8 as *const libc::c_char);
        }
        text_out_c(12 as libc::c_int as byte_hack,
                   b" quickly\x00" as *const u8 as *const libc::c_char);
    } else if ((*r_ptr).speed as libc::c_int) < 110 as libc::c_int {
        if ((*r_ptr).speed as libc::c_int) < 90 as libc::c_int {
            text_out_c(13 as libc::c_int as byte_hack,
                       b" incredibly\x00" as *const u8 as
                           *const libc::c_char);
        } else if ((*r_ptr).speed as libc::c_int) < 100 as libc::c_int {
            text_out_c(6 as libc::c_int as byte_hack,
                       b" very\x00" as *const u8 as *const libc::c_char);
        }
        text_out_c(14 as libc::c_int as byte_hack,
                   b" slowly\x00" as *const u8 as *const libc::c_char);
    } else {
        text_out(b" at normal speed\x00" as *const u8 as *const libc::c_char);
    }
    /* Random-ness */
    /* Speed */
    /* The code above includes "attack speed" */
    if flags1 & 0x20000 as libc::c_int as libc::c_uint != 0 {
        /* Introduce */
        if old != 0 {
            text_out(b", but \x00" as *const u8 as *const libc::c_char);
        } else {
            text_out(format(b"%^s \x00" as *const u8 as *const libc::c_char,
                            wd_he[msex as usize]) as cptr);
            old = 1 as libc::c_int as bool_
        }
        /* Describe */
        text_out(b"does not deign to chase intruders\x00" as *const u8 as
                     *const libc::c_char);
    }
    /* End this sentence */
    if old != 0 {
        text_out(b".  \x00" as *const u8 as *const libc::c_char);
        old = 0 as libc::c_int as bool_
    }
    /* Describe experience if known */
    if (*r_ptr).r_tkills != 0 {
        /* Introduction */
        if flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
            text_out(b"Killing this\x00" as *const u8 as *const libc::c_char);
        } else {
            text_out(b"A kill of this\x00" as *const u8 as
                         *const libc::c_char);
        }
        /* Describe the "quality" */
        if flags2 & 0x2000 as libc::c_int as libc::c_uint != 0 {
            text_out_c(10 as libc::c_int as byte_hack,
                       b" sanity-blasting\x00" as *const u8 as
                           *const libc::c_char);
        }
        if flags3 & 0x80 as libc::c_int as libc::c_uint != 0 {
            text_out_c(10 as libc::c_int as byte_hack,
                       b" natural\x00" as *const u8 as *const libc::c_char);
        }
        if flags3 & 0x40 as libc::c_int as libc::c_uint != 0 {
            text_out_c(10 as libc::c_int as byte_hack,
                       b" evil\x00" as *const u8 as *const libc::c_char);
        }
        if flags3 & 0x200 as libc::c_int as libc::c_uint != 0 {
            text_out_c(10 as libc::c_int as byte_hack,
                       b" good\x00" as *const u8 as *const libc::c_char);
        }
        if flags3 & 0x20 as libc::c_int as libc::c_uint != 0 {
            text_out_c(10 as libc::c_int as byte_hack,
                       b" undead\x00" as *const u8 as *const libc::c_char);
        }
        /* Describe the "race" */
        if flags3 & 0x8 as libc::c_int as libc::c_uint != 0 {
            text_out_c(10 as libc::c_int as byte_hack,
                       b" dragon\x00" as *const u8 as *const libc::c_char);
        } else if flags3 & 0x10 as libc::c_int as libc::c_uint != 0 {
            text_out_c(10 as libc::c_int as byte_hack,
                       b" demon\x00" as *const u8 as *const libc::c_char);
        } else if flags3 & 0x4 as libc::c_int as libc::c_uint != 0 {
            text_out_c(10 as libc::c_int as byte_hack,
                       b" giant\x00" as *const u8 as *const libc::c_char);
        } else if flags3 & 0x2 as libc::c_int as libc::c_uint != 0 {
            text_out_c(10 as libc::c_int as byte_hack,
                       b" troll\x00" as *const u8 as *const libc::c_char);
        } else if flags3 & 0x1 as libc::c_int as libc::c_uint != 0 {
            text_out_c(10 as libc::c_int as byte_hack,
                       b" orc\x00" as *const u8 as *const libc::c_char);
        } else if flags3 & 0x100 as libc::c_int as libc::c_uint != 0 {
            text_out_c(10 as libc::c_int as byte_hack,
                       b" Thunderlord\x00" as *const u8 as
                           *const libc::c_char);
        } else if flags7 & 0x40 as libc::c_int as libc::c_uint != 0 {
            text_out_c(10 as libc::c_int as byte_hack,
                       b" spider\x00" as *const u8 as *const libc::c_char);
        } else if flags7 & 0x80 as libc::c_int as libc::c_uint != 0 {
            text_out_c(10 as libc::c_int as byte_hack,
                       b" Nazgul\x00" as *const u8 as *const libc::c_char);
        } else {
            text_out(b" creature\x00" as *const u8 as *const libc::c_char);
        }
        /* Group some variables */
        let mut i: libc::c_long = 0;
        let mut j: libc::c_long = 0;
        /* calculate the integer exp part */
        i =
            (*r_ptr).mexp as libc::c_long * (*r_ptr).level as libc::c_long /
                (*p_ptr).lev as libc::c_long;
        /* calculate the fractional exp part scaled by 100, */
			/* must use long arithmetic to avoid overflow  */
        j =
            ((*r_ptr).mexp as libc::c_long * (*r_ptr).level as libc::c_long %
                 (*p_ptr).lev as libc::c_long *
                 1000 as libc::c_int as libc::c_long /
                 (*p_ptr).lev as libc::c_long +
                 5 as libc::c_int as libc::c_long) /
                10 as libc::c_int as libc::c_long;
        /* Mention the experience */
        text_out(b" is worth \x00" as *const u8 as *const libc::c_char);
        text_out_c(3 as libc::c_int as byte_hack,
                   format(b"%ld.%02ld\x00" as *const u8 as
                              *const libc::c_char, i, j) as cptr);
        text_out(b" point\x00" as *const u8 as *const libc::c_char);
        text_out(if i == 1 as libc::c_int as libc::c_long &&
                        j == 0 as libc::c_int as libc::c_long {
                     b"\x00" as *const u8 as *const libc::c_char
                 } else { b"s\x00" as *const u8 as *const libc::c_char });
        /* Take account of annoying English */
        p = b"th\x00" as *const u8 as *const libc::c_char;
        i = ((*p_ptr).lev as libc::c_int % 10 as libc::c_int) as libc::c_long;
        if !((*p_ptr).lev as libc::c_int / 10 as libc::c_int ==
                 1 as libc::c_int) {
            if i == 1 as libc::c_int as libc::c_long {
                p = b"st\x00" as *const u8 as *const libc::c_char
            } else if i == 2 as libc::c_int as libc::c_long {
                p = b"nd\x00" as *const u8 as *const libc::c_char
            } else if i == 3 as libc::c_int as libc::c_long {
                p = b"rd\x00" as *const u8 as *const libc::c_char
            }
        }
        /* Take account of "leading vowels" in numbers */
        q = b"\x00" as *const u8 as *const libc::c_char;
        i = (*p_ptr).lev as libc::c_long;
        if i == 8 as libc::c_int as libc::c_long ||
               i == 11 as libc::c_int as libc::c_long ||
               i == 18 as libc::c_int as libc::c_long {
            q = b"n\x00" as *const u8 as *const libc::c_char
        }
        /* Mention the dependance on the player's level */
        text_out(format(b" for a%s %lu%s level character.  \x00" as *const u8
                            as *const libc::c_char, q, i, p) as cptr);
    }
    if flags2 & 0x4000 as libc::c_int as libc::c_uint != 0 &&
           flags2 & 0x8000 as libc::c_int as libc::c_uint != 0 {
        text_out(format(b"%^s is surrounded by \x00" as *const u8 as
                            *const libc::c_char, wd_he[msex as usize]) as
                     cptr);
        text_out_c(10 as libc::c_int as byte_hack,
                   b"flames and electricity\x00" as *const u8 as
                       *const libc::c_char);
        text_out(b".  \x00" as *const u8 as *const libc::c_char);
    } else if flags2 & 0x4000 as libc::c_int as libc::c_uint != 0 {
        text_out(format(b"%^s is surrounded by \x00" as *const u8 as
                            *const libc::c_char, wd_he[msex as usize]) as
                     cptr);
        text_out_c(3 as libc::c_int as byte_hack,
                   b"flames\x00" as *const u8 as *const libc::c_char);
        text_out(b".  \x00" as *const u8 as *const libc::c_char);
    } else if flags2 & 0x8000 as libc::c_int as libc::c_uint != 0 {
        text_out(format(b"%^s is surrounded by \x00" as *const u8 as
                            *const libc::c_char, wd_he[msex as usize]) as
                     cptr);
        text_out_c(14 as libc::c_int as byte_hack,
                   b"electricity\x00" as *const u8 as *const libc::c_char);
        text_out(b".  \x00" as *const u8 as *const libc::c_char);
    }
    if flags2 & 0x8 as libc::c_int as libc::c_uint != 0 {
        text_out(format(b"%^s \x00" as *const u8 as *const libc::c_char,
                        wd_he[msex as usize]) as cptr);
        text_out_c(15 as libc::c_int as byte_hack,
                   b"reflects\x00" as *const u8 as *const libc::c_char);
        text_out(b" bolt spells.  \x00" as *const u8 as *const libc::c_char);
    }
    /* Describe escorts */
    if flags1 & 0x4000 as libc::c_int as libc::c_uint != 0 ||
           flags1 & 0x8000 as libc::c_int as libc::c_uint != 0 {
        text_out(format(b"%^s usually appears with escorts.  \x00" as
                            *const u8 as *const libc::c_char,
                        wd_he[msex as usize]) as cptr);
    } else if flags1 & 0x1000 as libc::c_int as libc::c_uint != 0 ||
                  flags1 & 0x2000 as libc::c_int as libc::c_uint != 0 {
        text_out(format(b"%^s usually appears in groups.  \x00" as *const u8
                            as *const libc::c_char, wd_he[msex as usize]) as
                     cptr);
    }
    /* Describe friends */
    /* Collect inate attacks */
    vn = 0 as libc::c_int;
    if flags4 & 0x1 as libc::c_int as libc::c_uint != 0 {
        let fresh0 = vn;
        vn = vn + 1;
        vp[fresh0 as usize] =
            b"shriek for help\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x8 as libc::c_int as libc::c_uint != 0 {
        let fresh1 = vn;
        vn = vn + 1;
        vp[fresh1 as usize] =
            b"shoot a rocket\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x10 as libc::c_int as libc::c_uint != 0 {
        let fresh2 = vn;
        vn = vn + 1;
        vp[fresh2 as usize] =
            b"fire an arrow\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x20 as libc::c_int as libc::c_uint != 0 {
        let fresh3 = vn;
        vn = vn + 1;
        vp[fresh3 as usize] =
            b"fire arrows\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x40 as libc::c_int as libc::c_uint != 0 {
        let fresh4 = vn;
        vn = vn + 1;
        vp[fresh4 as usize] =
            b"fire a missile\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x80 as libc::c_int as libc::c_uint != 0 {
        let fresh5 = vn;
        vn = vn + 1;
        vp[fresh5 as usize] =
            b"fire missiles\x00" as *const u8 as *const libc::c_char
    }
    /* Describe inate attacks */
    if vn != 0 {
        /* Intro */
        text_out(format(b"%^s\x00" as *const u8 as *const libc::c_char,
                        wd_he[msex as usize]) as cptr);
        /* Scan */
        n = 0 as libc::c_int;
        while n < vn {
            /* Intro */
            if n == 0 as libc::c_int {
                text_out(b" may \x00" as *const u8 as *const libc::c_char);
            } else if n < vn - 1 as libc::c_int {
                text_out(b", \x00" as *const u8 as *const libc::c_char);
            } else {
                text_out(b" or \x00" as *const u8 as *const libc::c_char);
            }
            /* Dump */
            text_out_c(11 as libc::c_int as byte_hack, vp[n as usize]);
            n += 1
        }
        /* End */
        text_out(b".  \x00" as *const u8 as *const libc::c_char);
    }
    /* Collect breaths */
    vn = 0 as libc::c_int;
    if flags4 & 0x100 as libc::c_int as libc::c_uint != 0 {
        let fresh6 = vn;
        vn = vn + 1;
        vp[fresh6 as usize] = b"acid\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x200 as libc::c_int as libc::c_uint != 0 {
        let fresh7 = vn;
        vn = vn + 1;
        vp[fresh7 as usize] =
            b"lightning\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x400 as libc::c_int as libc::c_uint != 0 {
        let fresh8 = vn;
        vn = vn + 1;
        vp[fresh8 as usize] = b"fire\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x800 as libc::c_int as libc::c_uint != 0 {
        let fresh9 = vn;
        vn = vn + 1;
        vp[fresh9 as usize] = b"frost\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x1000 as libc::c_int as libc::c_uint != 0 {
        let fresh10 = vn;
        vn = vn + 1;
        vp[fresh10 as usize] =
            b"poison\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x2000 as libc::c_int as libc::c_uint != 0 {
        let fresh11 = vn;
        vn = vn + 1;
        vp[fresh11 as usize] =
            b"nether\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x4000 as libc::c_int as libc::c_uint != 0 {
        let fresh12 = vn;
        vn = vn + 1;
        vp[fresh12 as usize] =
            b"light\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x8000 as libc::c_int as libc::c_uint != 0 {
        let fresh13 = vn;
        vn = vn + 1;
        vp[fresh13 as usize] =
            b"darkness\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x10000 as libc::c_int as libc::c_uint != 0 {
        let fresh14 = vn;
        vn = vn + 1;
        vp[fresh14 as usize] =
            b"confusion\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x20000 as libc::c_int as libc::c_uint != 0 {
        let fresh15 = vn;
        vn = vn + 1;
        vp[fresh15 as usize] =
            b"sound\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x40000 as libc::c_int as libc::c_uint != 0 {
        let fresh16 = vn;
        vn = vn + 1;
        vp[fresh16 as usize] =
            b"chaos\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x80000 as libc::c_int as libc::c_uint != 0 {
        let fresh17 = vn;
        vn = vn + 1;
        vp[fresh17 as usize] =
            b"disenchantment\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x100000 as libc::c_int as libc::c_uint != 0 {
        let fresh18 = vn;
        vn = vn + 1;
        vp[fresh18 as usize] =
            b"nexus\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x200000 as libc::c_int as libc::c_uint != 0 {
        let fresh19 = vn;
        vn = vn + 1;
        vp[fresh19 as usize] = b"time\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x400000 as libc::c_int as libc::c_uint != 0 {
        let fresh20 = vn;
        vn = vn + 1;
        vp[fresh20 as usize] =
            b"inertia\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x800000 as libc::c_int as libc::c_uint != 0 {
        let fresh21 = vn;
        vn = vn + 1;
        vp[fresh21 as usize] =
            b"gravity\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x1000000 as libc::c_int as libc::c_uint != 0 {
        let fresh22 = vn;
        vn = vn + 1;
        vp[fresh22 as usize] =
            b"shards\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x2000000 as libc::c_int as libc::c_uint != 0 {
        let fresh23 = vn;
        vn = vn + 1;
        vp[fresh23 as usize] =
            b"plasma\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x4000000 as libc::c_int as libc::c_uint != 0 {
        let fresh24 = vn;
        vn = vn + 1;
        vp[fresh24 as usize] =
            b"force\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x8000000 as libc::c_int as libc::c_uint != 0 {
        let fresh25 = vn;
        vn = vn + 1;
        vp[fresh25 as usize] = b"mana\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x20000000 as libc::c_int as libc::c_uint != 0 {
        let fresh26 = vn;
        vn = vn + 1;
        vp[fresh26 as usize] =
            b"toxic waste\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x80000000 as libc::c_uint != 0 {
        let fresh27 = vn;
        vn = vn + 1;
        vp[fresh27 as usize] =
            b"disintegration\x00" as *const u8 as *const libc::c_char
    }
    /* Describe breaths */
    if vn != 0 {
        /* Note breath */
        breath = 1 as libc::c_int as bool_;
        /* Intro */
        text_out(format(b"%^s\x00" as *const u8 as *const libc::c_char,
                        wd_he[msex as usize]) as cptr);
        /* Scan */
        n = 0 as libc::c_int;
        while n < vn {
            /* Intro */
            if n == 0 as libc::c_int {
                text_out(b" may breathe \x00" as *const u8 as
                             *const libc::c_char);
            } else if n < vn - 1 as libc::c_int {
                text_out(b", \x00" as *const u8 as *const libc::c_char);
            } else {
                text_out(b" or \x00" as *const u8 as *const libc::c_char);
            }
            /* Dump */
            text_out_c(11 as libc::c_int as byte_hack, vp[n as usize]);
            n += 1
        }
    }
    /* Collect spells */
    vn = 0 as libc::c_int;
    if flags5 & 0x1 as libc::c_int as libc::c_uint != 0 {
        let fresh28 = vn;
        vn = vn + 1;
        vp[fresh28 as usize] =
            b"produce acid balls\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x2 as libc::c_int as libc::c_uint != 0 {
        let fresh29 = vn;
        vn = vn + 1;
        vp[fresh29 as usize] =
            b"produce lightning balls\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x4 as libc::c_int as libc::c_uint != 0 {
        let fresh30 = vn;
        vn = vn + 1;
        vp[fresh30 as usize] =
            b"produce fire balls\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x8 as libc::c_int as libc::c_uint != 0 {
        let fresh31 = vn;
        vn = vn + 1;
        vp[fresh31 as usize] =
            b"produce frost balls\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x10 as libc::c_int as libc::c_uint != 0 {
        let fresh32 = vn;
        vn = vn + 1;
        vp[fresh32 as usize] =
            b"produce poison balls\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x20 as libc::c_int as libc::c_uint != 0 {
        let fresh33 = vn;
        vn = vn + 1;
        vp[fresh33 as usize] =
            b"produce nether balls\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x40 as libc::c_int as libc::c_uint != 0 {
        let fresh34 = vn;
        vn = vn + 1;
        vp[fresh34 as usize] =
            b"produce water balls\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x10000000 as libc::c_int as libc::c_uint != 0 {
        let fresh35 = vn;
        vn = vn + 1;
        vp[fresh35 as usize] =
            b"produce balls of radiation\x00" as *const u8 as
                *const libc::c_char
    }
    if flags5 & 0x80 as libc::c_int as libc::c_uint != 0 {
        let fresh36 = vn;
        vn = vn + 1;
        vp[fresh36 as usize] =
            b"invoke mana storms\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x100 as libc::c_int as libc::c_uint != 0 {
        let fresh37 = vn;
        vn = vn + 1;
        vp[fresh37 as usize] =
            b"invoke darkness storms\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x40000000 as libc::c_int as libc::c_uint != 0 {
        let fresh38 = vn;
        vn = vn + 1;
        vp[fresh38 as usize] =
            b"invoke raw chaos\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x2 as libc::c_int as libc::c_uint != 0 {
        let fresh39 = vn;
        vn = vn + 1;
        vp[fresh39 as usize] =
            b"invoke the Hand of Doom\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x200 as libc::c_int as libc::c_uint != 0 {
        let fresh40 = vn;
        vn = vn + 1;
        vp[fresh40 as usize] =
            b"drain mana\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x400 as libc::c_int as libc::c_uint != 0 {
        let fresh41 = vn;
        vn = vn + 1;
        vp[fresh41 as usize] =
            b"cause mind blasting\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x800 as libc::c_int as libc::c_uint != 0 {
        let fresh42 = vn;
        vn = vn + 1;
        vp[fresh42 as usize] =
            b"cause brain smashing\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x1000 as libc::c_int as libc::c_uint != 0 {
        let fresh43 = vn;
        vn = vn + 1;
        vp[fresh43 as usize] =
            b"cause light wounds and cursing\x00" as *const u8 as
                *const libc::c_char
    }
    if flags5 & 0x2000 as libc::c_int as libc::c_uint != 0 {
        let fresh44 = vn;
        vn = vn + 1;
        vp[fresh44 as usize] =
            b"cause serious wounds and cursing\x00" as *const u8 as
                *const libc::c_char
    }
    if flags5 & 0x4000 as libc::c_int as libc::c_uint != 0 {
        let fresh45 = vn;
        vn = vn + 1;
        vp[fresh45 as usize] =
            b"cause critical wounds and cursing\x00" as *const u8 as
                *const libc::c_char
    }
    if flags5 & 0x8000 as libc::c_int as libc::c_uint != 0 {
        let fresh46 = vn;
        vn = vn + 1;
        vp[fresh46 as usize] =
            b"cause mortal wounds\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x10000 as libc::c_int as libc::c_uint != 0 {
        let fresh47 = vn;
        vn = vn + 1;
        vp[fresh47 as usize] =
            b"produce acid bolts\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x20000 as libc::c_int as libc::c_uint != 0 {
        let fresh48 = vn;
        vn = vn + 1;
        vp[fresh48 as usize] =
            b"produce lightning bolts\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x40000 as libc::c_int as libc::c_uint != 0 {
        let fresh49 = vn;
        vn = vn + 1;
        vp[fresh49 as usize] =
            b"produce fire bolts\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x80000 as libc::c_int as libc::c_uint != 0 {
        let fresh50 = vn;
        vn = vn + 1;
        vp[fresh50 as usize] =
            b"produce frost bolts\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x100000 as libc::c_int as libc::c_uint != 0 {
        let fresh51 = vn;
        vn = vn + 1;
        vp[fresh51 as usize] =
            b"produce poison bolts\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x200000 as libc::c_int as libc::c_uint != 0 {
        let fresh52 = vn;
        vn = vn + 1;
        vp[fresh52 as usize] =
            b"produce nether bolts\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x400000 as libc::c_int as libc::c_uint != 0 {
        let fresh53 = vn;
        vn = vn + 1;
        vp[fresh53 as usize] =
            b"produce water bolts\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x800000 as libc::c_int as libc::c_uint != 0 {
        let fresh54 = vn;
        vn = vn + 1;
        vp[fresh54 as usize] =
            b"produce mana bolts\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x1000000 as libc::c_int as libc::c_uint != 0 {
        let fresh55 = vn;
        vn = vn + 1;
        vp[fresh55 as usize] =
            b"produce plasma bolts\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x2000000 as libc::c_int as libc::c_uint != 0 {
        let fresh56 = vn;
        vn = vn + 1;
        vp[fresh56 as usize] =
            b"produce ice bolts\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x4000000 as libc::c_int as libc::c_uint != 0 {
        let fresh57 = vn;
        vn = vn + 1;
        vp[fresh57 as usize] =
            b"produce magic missiles\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x8000000 as libc::c_int as libc::c_uint != 0 {
        let fresh58 = vn;
        vn = vn + 1;
        vp[fresh58 as usize] =
            b"terrify\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x10000000 as libc::c_int as libc::c_uint != 0 {
        let fresh59 = vn;
        vn = vn + 1;
        vp[fresh59 as usize] =
            b"blind\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x20000000 as libc::c_int as libc::c_uint != 0 {
        let fresh60 = vn;
        vn = vn + 1;
        vp[fresh60 as usize] =
            b"confuse\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x40000000 as libc::c_int as libc::c_uint != 0 {
        let fresh61 = vn;
        vn = vn + 1;
        vp[fresh61 as usize] = b"slow\x00" as *const u8 as *const libc::c_char
    }
    if flags5 & 0x80000000 as libc::c_uint != 0 {
        let fresh62 = vn;
        vn = vn + 1;
        vp[fresh62 as usize] =
            b"paralyze\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x1 as libc::c_int as libc::c_uint != 0 {
        let fresh63 = vn;
        vn = vn + 1;
        vp[fresh63 as usize] =
            b"haste-self\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x4 as libc::c_int as libc::c_uint != 0 {
        let fresh64 = vn;
        vn = vn + 1;
        vp[fresh64 as usize] =
            b"heal-self\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x10 as libc::c_int as libc::c_uint != 0 {
        let fresh65 = vn;
        vn = vn + 1;
        vp[fresh65 as usize] =
            b"blink-self\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x20 as libc::c_int as libc::c_uint != 0 {
        let fresh66 = vn;
        vn = vn + 1;
        vp[fresh66 as usize] =
            b"teleport-self\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x2000 as libc::c_int as libc::c_uint != 0 {
        let fresh67 = vn;
        vn = vn + 1;
        vp[fresh67 as usize] =
            b"summon software bugs\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x4000 as libc::c_int as libc::c_uint != 0 {
        let fresh68 = vn;
        vn = vn + 1;
        vp[fresh68 as usize] =
            b"summon RNG\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x40 as libc::c_int as libc::c_uint != 0 {
        let fresh69 = vn;
        vn = vn + 1;
        vp[fresh69 as usize] =
            b"teleport to\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x80 as libc::c_int as libc::c_uint != 0 {
        let fresh70 = vn;
        vn = vn + 1;
        vp[fresh70 as usize] =
            b"teleport away\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x100 as libc::c_int as libc::c_uint != 0 {
        let fresh71 = vn;
        vn = vn + 1;
        vp[fresh71 as usize] =
            b"teleport level\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x8000 as libc::c_int as libc::c_uint != 0 {
        let fresh72 = vn;
        vn = vn + 1;
        vp[fresh72 as usize] =
            b"summon a Thunderlord\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x200 as libc::c_int as libc::c_uint != 0 {
        let fresh73 = vn;
        vn = vn + 1;
        vp[fresh73 as usize] =
            b"create darkness\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x400 as libc::c_int as libc::c_uint != 0 {
        let fresh74 = vn;
        vn = vn + 1;
        vp[fresh74 as usize] =
            b"create traps\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x800 as libc::c_int as libc::c_uint != 0 {
        let fresh75 = vn;
        vn = vn + 1;
        vp[fresh75 as usize] =
            b"cause amnesia\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x1000 as libc::c_int as libc::c_uint != 0 {
        let fresh76 = vn;
        vn = vn + 1;
        vp[fresh76 as usize] =
            b"raise dead\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x40000 as libc::c_int as libc::c_uint != 0 {
        let fresh77 = vn;
        vn = vn + 1;
        vp[fresh77 as usize] =
            b"summon a monster\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x80000 as libc::c_int as libc::c_uint != 0 {
        let fresh78 = vn;
        vn = vn + 1;
        vp[fresh78 as usize] =
            b"summon monsters\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x10000 as libc::c_int as libc::c_uint != 0 {
        let fresh79 = vn;
        vn = vn + 1;
        vp[fresh79 as usize] =
            b"summon aid\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x100000 as libc::c_int as libc::c_uint != 0 {
        let fresh80 = vn;
        vn = vn + 1;
        vp[fresh80 as usize] =
            b"summon ants\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x200000 as libc::c_int as libc::c_uint != 0 {
        let fresh81 = vn;
        vn = vn + 1;
        vp[fresh81 as usize] =
            b"summon spiders\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x400000 as libc::c_int as libc::c_uint != 0 {
        let fresh82 = vn;
        vn = vn + 1;
        vp[fresh82 as usize] =
            b"summon hounds\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x800000 as libc::c_int as libc::c_uint != 0 {
        let fresh83 = vn;
        vn = vn + 1;
        vp[fresh83 as usize] =
            b"summon hydras\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x1000000 as libc::c_int as libc::c_uint != 0 {
        let fresh84 = vn;
        vn = vn + 1;
        vp[fresh84 as usize] =
            b"summon an angel\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x2000000 as libc::c_int as libc::c_uint != 0 {
        let fresh85 = vn;
        vn = vn + 1;
        vp[fresh85 as usize] =
            b"summon a demon\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x4000000 as libc::c_int as libc::c_uint != 0 {
        let fresh86 = vn;
        vn = vn + 1;
        vp[fresh86 as usize] =
            b"summon an undead\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x8000000 as libc::c_int as libc::c_uint != 0 {
        let fresh87 = vn;
        vn = vn + 1;
        vp[fresh87 as usize] =
            b"summon a dragon\x00" as *const u8 as *const libc::c_char
    }
    if flags4 & 0x4 as libc::c_int as libc::c_uint != 0 {
        let fresh88 = vn;
        vn = vn + 1;
        vp[fresh88 as usize] =
            b"summon animal\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x8 as libc::c_int as libc::c_uint != 0 {
        let fresh89 = vn;
        vn = vn + 1;
        vp[fresh89 as usize] =
            b"summon animals\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x10000000 as libc::c_int as libc::c_uint != 0 {
        let fresh90 = vn;
        vn = vn + 1;
        vp[fresh90 as usize] =
            b"summon Greater Undead\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x20000000 as libc::c_int as libc::c_uint != 0 {
        let fresh91 = vn;
        vn = vn + 1;
        vp[fresh91 as usize] =
            b"summon Ancient Dragons\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x20000 as libc::c_int as libc::c_uint != 0 {
        let fresh92 = vn;
        vn = vn + 1;
        vp[fresh92 as usize] =
            b"summon Greater Demons\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x40000000 as libc::c_int as libc::c_uint != 0 {
        let fresh93 = vn;
        vn = vn + 1;
        vp[fresh93 as usize] =
            b"summon Ringwraith\x00" as *const u8 as *const libc::c_char
    }
    if flags6 & 0x80000000 as libc::c_uint != 0 {
        let fresh94 = vn;
        vn = vn + 1;
        vp[fresh94 as usize] =
            b"summon Unique Monsters\x00" as *const u8 as *const libc::c_char
    }
    /* Describe spells */
    if vn != 0 {
        /* Note magic */
        magic = 1 as libc::c_int as bool_;
        /* Intro */
        if breath != 0 {
            text_out(b", and is also\x00" as *const u8 as
                         *const libc::c_char);
        } else {
            text_out(format(b"%^s is\x00" as *const u8 as *const libc::c_char,
                            wd_he[msex as usize]) as cptr);
        }
        /* Verb Phrase */
        text_out(b" magical, casting spells\x00" as *const u8 as
                     *const libc::c_char);
        /* Adverb */
        if flags2 & 0x2 as libc::c_int as libc::c_uint != 0 {
            text_out_c(11 as libc::c_int as byte_hack,
                       b" intelligently\x00" as *const u8 as
                           *const libc::c_char);
        }
        /* Scan */
        n = 0 as libc::c_int;
        while n < vn {
            /* Intro */
            if n == 0 as libc::c_int {
                text_out(b" which \x00" as *const u8 as *const libc::c_char);
            } else if n < vn - 1 as libc::c_int {
                text_out(b", \x00" as *const u8 as *const libc::c_char);
            } else {
                text_out(b" or \x00" as *const u8 as *const libc::c_char);
            }
            /* Dump */
            text_out_c(11 as libc::c_int as byte_hack, vp[n as usize]);
            n += 1
        }
    }
    /* End the sentence about inate/other spells */
    if breath as libc::c_int != 0 || magic as libc::c_int != 0 {
        /* Total casting */
        m =
            (*r_ptr).r_cast_inate as libc::c_int +
                (*r_ptr).r_cast_spell as libc::c_int;
        /* Average frequency */
        n =
            ((*r_ptr).freq_inate as libc::c_int +
                 (*r_ptr).freq_spell as libc::c_int) / 2 as libc::c_int;
        /* Describe the spell frequency */
        if m > 100 as libc::c_int {
            text_out(b"; \x00" as *const u8 as *const libc::c_char);
            text_out_c(13 as libc::c_int as byte_hack,
                       b"1\x00" as *const u8 as *const libc::c_char);
            text_out(b" time in \x00" as *const u8 as *const libc::c_char);
            text_out_c(13 as libc::c_int as byte_hack,
                       format(b"%d\x00" as *const u8 as *const libc::c_char,
                              100 as libc::c_int / n) as cptr);
        } else if m != 0 {
            n =
                (n + 9 as libc::c_int) / 10 as libc::c_int *
                    10 as libc::c_int;
            text_out(b"; about \x00" as *const u8 as *const libc::c_char);
            text_out_c(13 as libc::c_int as byte_hack,
                       b"1\x00" as *const u8 as *const libc::c_char);
            text_out(b" time in \x00" as *const u8 as *const libc::c_char);
            text_out_c(13 as libc::c_int as byte_hack,
                       format(b"%d\x00" as *const u8 as *const libc::c_char,
                              100 as libc::c_int / n) as cptr);
        }
        /* Guess at the frequency */
        /* End this sentence */
        text_out(b".  \x00" as *const u8 as *const libc::c_char);
    }
    /* Describe monster "toughness" */
    if know_armour(r_idx) != 0 {
        /* Armor */
        text_out(format(b"%^s has an armor rating of \x00" as *const u8 as
                            *const libc::c_char, wd_he[msex as usize]) as
                     cptr);
        text_out_c(13 as libc::c_int as byte_hack,
                   format(b"%d\x00" as *const u8 as *const libc::c_char,
                          (*r_ptr).ac as libc::c_int) as cptr);
        /* Maximized hitpoints */
        if flags1 & 0x200 as libc::c_int as libc::c_uint != 0 {
            text_out(b" and a life rating of \x00" as *const u8 as
                         *const libc::c_char);
            text_out_c(13 as libc::c_int as byte_hack,
                       format(b"%d\x00" as *const u8 as *const libc::c_char,
                              (*r_ptr).hdice as libc::c_int *
                                  (*r_ptr).hside as libc::c_int) as cptr);
            text_out(b".  \x00" as *const u8 as *const libc::c_char);
        } else {
            /* Variable hitpoints */
            text_out(b" and a life rating of \x00" as *const u8 as
                         *const libc::c_char);
            text_out_c(13 as libc::c_int as byte_hack,
                       format(b"%dd%d\x00" as *const u8 as
                                  *const libc::c_char,
                              (*r_ptr).hdice as libc::c_int,
                              (*r_ptr).hside as libc::c_int) as cptr);
            text_out(b".  \x00" as *const u8 as *const libc::c_char);
        }
    }
    /* Collect special abilities. */
    vn = 0 as libc::c_int;
    if flags2 & 0x10000 as libc::c_int as libc::c_uint != 0 {
        let fresh95 = vn;
        vn = vn + 1;
        vp[fresh95 as usize] =
            b"open doors\x00" as *const u8 as *const libc::c_char
    }
    if flags2 & 0x20000 as libc::c_int as libc::c_uint != 0 {
        let fresh96 = vn;
        vn = vn + 1;
        vp[fresh96 as usize] =
            b"bash down doors\x00" as *const u8 as *const libc::c_char
    }
    if flags2 & 0x40000 as libc::c_int as libc::c_uint != 0 {
        let fresh97 = vn;
        vn = vn + 1;
        vp[fresh97 as usize] =
            b"pass through walls\x00" as *const u8 as *const libc::c_char
    }
    if flags2 & 0x80000 as libc::c_int as libc::c_uint != 0 {
        let fresh98 = vn;
        vn = vn + 1;
        vp[fresh98 as usize] =
            b"bore through walls\x00" as *const u8 as *const libc::c_char
    }
    if flags2 & 0x100000 as libc::c_int as libc::c_uint != 0 {
        let fresh99 = vn;
        vn = vn + 1;
        vp[fresh99 as usize] =
            b"push past weaker monsters\x00" as *const u8 as
                *const libc::c_char
    }
    if flags2 & 0x200000 as libc::c_int as libc::c_uint != 0 {
        let fresh100 = vn;
        vn = vn + 1;
        vp[fresh100 as usize] =
            b"destroy weaker monsters\x00" as *const u8 as *const libc::c_char
    }
    if flags2 & 0x400000 as libc::c_int as libc::c_uint != 0 {
        let fresh101 = vn;
        vn = vn + 1;
        vp[fresh101 as usize] =
            b"pick up objects\x00" as *const u8 as *const libc::c_char
    }
    if flags2 & 0x800000 as libc::c_int as libc::c_uint != 0 {
        let fresh102 = vn;
        vn = vn + 1;
        vp[fresh102 as usize] =
            b"destroy objects\x00" as *const u8 as *const libc::c_char
    }
    if flags9 & 0x4 as libc::c_int as libc::c_uint != 0 {
        let fresh103 = vn;
        vn = vn + 1;
        vp[fresh103 as usize] =
            b"illuminate the dungeon\x00" as *const u8 as *const libc::c_char
    }
    /* Describe special abilities. */
    if vn != 0 {
        /* Intro */
        text_out(format(b"%^s\x00" as *const u8 as *const libc::c_char,
                        wd_he[msex as usize]) as cptr);
        /* Scan */
        n = 0 as libc::c_int;
        while n < vn {
            /* Intro */
            if n == 0 as libc::c_int {
                text_out(b" can \x00" as *const u8 as *const libc::c_char);
            } else if n < vn - 1 as libc::c_int {
                text_out(b", \x00" as *const u8 as *const libc::c_char);
            } else {
                text_out(b" and \x00" as *const u8 as *const libc::c_char);
            }
            /* Dump */
            text_out(vp[n as usize]);
            n += 1
        }
        /* End */
        text_out(b".  \x00" as *const u8 as *const libc::c_char);
    }
    /* Describe special abilities. */
    if flags2 & 0x10 as libc::c_int as libc::c_uint != 0 {
        text_out_c(5 as libc::c_int as byte_hack,
                   format(b"%^s is invisible.  \x00" as *const u8 as
                              *const libc::c_char, wd_he[msex as usize]) as
                       cptr);
    }
    if flags2 & 0x20 as libc::c_int as libc::c_uint != 0 {
        text_out(format(b"%^s is cold blooded.  \x00" as *const u8 as
                            *const libc::c_char, wd_he[msex as usize]) as
                     cptr);
    }
    if flags2 & 0x40 as libc::c_int as libc::c_uint != 0 {
        text_out(format(b"%^s is not detected by telepathy.  \x00" as
                            *const u8 as *const libc::c_char,
                        wd_he[msex as usize]) as cptr);
    }
    if flags2 & 0x80 as libc::c_int as libc::c_uint != 0 {
        text_out(format(b"%^s is rarely detected by telepathy.  \x00" as
                            *const u8 as *const libc::c_char,
                        wd_he[msex as usize]) as cptr);
    }
    if flags4 & 0x2 as libc::c_int as libc::c_uint != 0 {
        text_out_c(15 as libc::c_int as byte_hack,
                   format(b"%^s breeds explosively.  \x00" as *const u8 as
                              *const libc::c_char, wd_he[msex as usize]) as
                       cptr);
    }
    if flags2 & 0x200 as libc::c_int as libc::c_uint != 0 {
        text_out_c(9 as libc::c_int as byte_hack,
                   format(b"%^s regenerates quickly.  \x00" as *const u8 as
                              *const libc::c_char, wd_he[msex as usize]) as
                       cptr);
    }
    if (*r_ptr).flags7 & 0x20 as libc::c_int as libc::c_uint != 0 {
        text_out_c(4 as libc::c_int as byte_hack,
                   format(b"%^s is a mortal being.  \x00" as *const u8 as
                              *const libc::c_char, wd_he[msex as usize]) as
                       cptr);
    } else {
        text_out_c(14 as libc::c_int as byte_hack,
                   format(b"%^s is an immortal being.  \x00" as *const u8 as
                              *const libc::c_char, wd_he[msex as usize]) as
                       cptr);
    }
    /* Collect susceptibilities */
    vn = 0 as libc::c_int;
    if flags3 & 0x2000 as libc::c_int as libc::c_uint != 0 {
        let fresh104 = vn;
        vn = vn + 1;
        vp[fresh104 as usize] =
            b"rock remover\x00" as *const u8 as *const libc::c_char;
        color[(vn - 1 as libc::c_int) as usize] =
            7 as libc::c_int as byte_hack
    }
    if flags3 & 0x1000 as libc::c_int as libc::c_uint != 0 {
        let fresh105 = vn;
        vn = vn + 1;
        vp[fresh105 as usize] =
            b"bright light\x00" as *const u8 as *const libc::c_char;
        color[(vn - 1 as libc::c_int) as usize] =
            11 as libc::c_int as byte_hack
    }
    if flags3 & 0x4000 as libc::c_int as libc::c_uint != 0 {
        let fresh106 = vn;
        vn = vn + 1;
        vp[fresh106 as usize] =
            b"fire\x00" as *const u8 as *const libc::c_char;
        color[(vn - 1 as libc::c_int) as usize] =
            4 as libc::c_int as byte_hack
    }
    if flags3 & 0x8000 as libc::c_int as libc::c_uint != 0 {
        let fresh107 = vn;
        vn = vn + 1;
        vp[fresh107 as usize] =
            b"cold\x00" as *const u8 as *const libc::c_char;
        color[(vn - 1 as libc::c_int) as usize] =
            9 as libc::c_int as byte_hack
    }
    if flags9 & 0x40 as libc::c_int as libc::c_uint != 0 {
        let fresh108 = vn;
        vn = vn + 1;
        vp[fresh108 as usize] =
            b"acid\x00" as *const u8 as *const libc::c_char;
        color[(vn - 1 as libc::c_int) as usize] =
            5 as libc::c_int as byte_hack
    }
    if flags9 & 0x80 as libc::c_int as libc::c_uint != 0 {
        let fresh109 = vn;
        vn = vn + 1;
        vp[fresh109 as usize] =
            b"lightning\x00" as *const u8 as *const libc::c_char;
        color[(vn - 1 as libc::c_int) as usize] =
            14 as libc::c_int as byte_hack
    }
    if flags9 & 0x100 as libc::c_int as libc::c_uint != 0 {
        let fresh110 = vn;
        vn = vn + 1;
        vp[fresh110 as usize] =
            b"poison\x00" as *const u8 as *const libc::c_char;
        color[(vn - 1 as libc::c_int) as usize] =
            13 as libc::c_int as byte_hack
    }
    /* Describe susceptibilities */
    if vn != 0 {
        /* Intro */
        text_out(format(b"%^s\x00" as *const u8 as *const libc::c_char,
                        wd_he[msex as usize]) as cptr);
        /* Scan */
        n = 0 as libc::c_int;
        while n < vn {
            /* Intro */
            if n == 0 as libc::c_int {
                text_out(b" is hurt by \x00" as *const u8 as
                             *const libc::c_char);
            } else if n < vn - 1 as libc::c_int {
                text_out(b", \x00" as *const u8 as *const libc::c_char);
            } else {
                text_out(b" and \x00" as *const u8 as *const libc::c_char);
            }
            /* Dump */
            text_out_c(color[n as usize], vp[n as usize]);
            n += 1
        }
        /* End */
        text_out(b".  \x00" as *const u8 as *const libc::c_char);
    }
    /* Collect immunities */
    vn = 0 as libc::c_int;
    if flags3 & 0x10000 as libc::c_int as libc::c_uint != 0 {
        let fresh111 = vn;
        vn = vn + 1;
        vp[fresh111 as usize] =
            b"acid\x00" as *const u8 as *const libc::c_char;
        color[(vn - 1 as libc::c_int) as usize] =
            13 as libc::c_int as byte_hack
    }
    if flags3 & 0x20000 as libc::c_int as libc::c_uint != 0 {
        let fresh112 = vn;
        vn = vn + 1;
        vp[fresh112 as usize] =
            b"lightning\x00" as *const u8 as *const libc::c_char;
        color[(vn - 1 as libc::c_int) as usize] =
            14 as libc::c_int as byte_hack
    }
    if flags3 & 0x40000 as libc::c_int as libc::c_uint != 0 {
        let fresh113 = vn;
        vn = vn + 1;
        vp[fresh113 as usize] =
            b"fire\x00" as *const u8 as *const libc::c_char;
        color[(vn - 1 as libc::c_int) as usize] =
            12 as libc::c_int as byte_hack
    }
    if flags3 & 0x80000 as libc::c_int as libc::c_uint != 0 {
        let fresh114 = vn;
        vn = vn + 1;
        vp[fresh114 as usize] =
            b"cold\x00" as *const u8 as *const libc::c_char;
        color[(vn - 1 as libc::c_int) as usize] =
            14 as libc::c_int as byte_hack
    }
    if flags3 & 0x100000 as libc::c_int as libc::c_uint != 0 {
        let fresh115 = vn;
        vn = vn + 1;
        vp[fresh115 as usize] =
            b"poison\x00" as *const u8 as *const libc::c_char;
        color[(vn - 1 as libc::c_int) as usize] =
            13 as libc::c_int as byte_hack
    }
    /* Describe immunities */
    if vn != 0 {
        /* Intro */
        text_out(format(b"%^s\x00" as *const u8 as *const libc::c_char,
                        wd_he[msex as usize]) as cptr);
        /* Scan */
        n = 0 as libc::c_int;
        while n < vn {
            /* Intro */
            if n == 0 as libc::c_int {
                text_out(b" resists \x00" as *const u8 as
                             *const libc::c_char);
            } else if n < vn - 1 as libc::c_int {
                text_out(b", \x00" as *const u8 as *const libc::c_char);
            } else {
                text_out(b" and \x00" as *const u8 as *const libc::c_char);
            }
            /* Dump */
            text_out_c(color[n as usize], vp[n as usize]);
            n += 1
        }
        /* End */
        text_out(b".  \x00" as *const u8 as *const libc::c_char);
    }
    /* Collect resistances */
    vn = 0 as libc::c_int;
    if flags3 & 0x400000 as libc::c_int as libc::c_uint != 0 {
        let fresh116 = vn;
        vn = vn + 1;
        vp[fresh116 as usize] =
            b"nether\x00" as *const u8 as *const libc::c_char
    }
    if flags3 & 0x800000 as libc::c_int as libc::c_uint != 0 {
        let fresh117 = vn;
        vn = vn + 1;
        vp[fresh117 as usize] =
            b"water\x00" as *const u8 as *const libc::c_char
    }
    if flags3 & 0x1000000 as libc::c_int as libc::c_uint != 0 {
        let fresh118 = vn;
        vn = vn + 1;
        vp[fresh118 as usize] =
            b"plasma\x00" as *const u8 as *const libc::c_char
    }
    if flags3 & 0x2000000 as libc::c_int as libc::c_uint != 0 {
        let fresh119 = vn;
        vn = vn + 1;
        vp[fresh119 as usize] =
            b"nexus\x00" as *const u8 as *const libc::c_char
    }
    if flags3 & 0x4000000 as libc::c_int as libc::c_uint != 0 {
        let fresh120 = vn;
        vn = vn + 1;
        vp[fresh120 as usize] =
            b"disenchantment\x00" as *const u8 as *const libc::c_char
    }
    if flags3 & 0x200000 as libc::c_int as libc::c_uint != 0 {
        let fresh121 = vn;
        vn = vn + 1;
        vp[fresh121 as usize] =
            b"teleportation\x00" as *const u8 as *const libc::c_char
    }
    /* Describe resistances */
    if vn != 0 {
        /* Intro */
        text_out(format(b"%^s\x00" as *const u8 as *const libc::c_char,
                        wd_he[msex as usize]) as cptr);
        /* Scan */
        n = 0 as libc::c_int;
        while n < vn {
            /* Intro */
            if n == 0 as libc::c_int {
                text_out(b" resists \x00" as *const u8 as
                             *const libc::c_char);
            } else if n < vn - 1 as libc::c_int {
                text_out(b", \x00" as *const u8 as *const libc::c_char);
            } else {
                text_out(b" and \x00" as *const u8 as *const libc::c_char);
            }
            /* Dump */
            text_out_c(14 as libc::c_int as byte_hack, vp[n as usize]);
            n += 1
        }
        /* End */
        text_out(b".  \x00" as *const u8 as *const libc::c_char);
    }
    /* Collect non-effects */
    vn = 0 as libc::c_int;
    if flags3 & 0x20000000 as libc::c_int as libc::c_uint != 0 {
        let fresh122 = vn;
        vn = vn + 1;
        vp[fresh122 as usize] =
            b"stunned\x00" as *const u8 as *const libc::c_char
    }
    if flags3 & 0x10000000 as libc::c_int as libc::c_uint != 0 {
        let fresh123 = vn;
        vn = vn + 1;
        vp[fresh123 as usize] =
            b"frightened\x00" as *const u8 as *const libc::c_char
    }
    if flags3 & 0x40000000 as libc::c_int as libc::c_uint != 0 {
        let fresh124 = vn;
        vn = vn + 1;
        vp[fresh124 as usize] =
            b"confused\x00" as *const u8 as *const libc::c_char
    }
    if flags3 & 0x80000000 as libc::c_uint != 0 {
        let fresh125 = vn;
        vn = vn + 1;
        vp[fresh125 as usize] =
            b"slept\x00" as *const u8 as *const libc::c_char
    }
    /* Describe non-effects */
    if vn != 0 {
        /* Intro */
        text_out(format(b"%^s\x00" as *const u8 as *const libc::c_char,
                        wd_he[msex as usize]) as cptr);
        /* Scan */
        n = 0 as libc::c_int;
        while n < vn {
            /* Intro */
            if n == 0 as libc::c_int {
                text_out(b" cannot be \x00" as *const u8 as
                             *const libc::c_char);
            } else if n < vn - 1 as libc::c_int {
                text_out(b", \x00" as *const u8 as *const libc::c_char);
            } else {
                text_out(b" or \x00" as *const u8 as *const libc::c_char);
            }
            /* Dump */
            text_out(vp[n as usize]);
            n += 1
        }
        /* End */
        text_out(b".  \x00" as *const u8 as *const libc::c_char);
    }
    /* Do we know how aware it is? */
    if (*r_ptr).r_wake as libc::c_int * (*r_ptr).r_wake as libc::c_int >
           (*r_ptr).sleep as libc::c_int ||
           (*r_ptr).r_ignore as libc::c_int == 255 as libc::c_int ||
           (*r_ptr).sleep as libc::c_int == 0 as libc::c_int &&
               (*r_ptr).r_tkills as libc::c_int >= 10 as libc::c_int {
        let mut act: cptr = 0 as *const libc::c_char;
        if (*r_ptr).sleep as libc::c_int > 200 as libc::c_int {
            act = b"prefers to ignore\x00" as *const u8 as *const libc::c_char
        } else if (*r_ptr).sleep as libc::c_int > 95 as libc::c_int {
            act =
                b"pays very little attention to\x00" as *const u8 as
                    *const libc::c_char
        } else if (*r_ptr).sleep as libc::c_int > 75 as libc::c_int {
            act =
                b"pays little attention to\x00" as *const u8 as
                    *const libc::c_char
        } else if (*r_ptr).sleep as libc::c_int > 45 as libc::c_int {
            act = b"tends to overlook\x00" as *const u8 as *const libc::c_char
        } else if (*r_ptr).sleep as libc::c_int > 25 as libc::c_int {
            act =
                b"takes quite a while to see\x00" as *const u8 as
                    *const libc::c_char
        } else if (*r_ptr).sleep as libc::c_int > 10 as libc::c_int {
            act =
                b"takes a while to see\x00" as *const u8 as
                    *const libc::c_char
        } else if (*r_ptr).sleep as libc::c_int > 5 as libc::c_int {
            act =
                b"is fairly observant of\x00" as *const u8 as
                    *const libc::c_char
        } else if (*r_ptr).sleep as libc::c_int > 3 as libc::c_int {
            act = b"is observant of\x00" as *const u8 as *const libc::c_char
        } else if (*r_ptr).sleep as libc::c_int > 1 as libc::c_int {
            act =
                b"is very observant of\x00" as *const u8 as
                    *const libc::c_char
        } else if (*r_ptr).sleep as libc::c_int > 0 as libc::c_int {
            act = b"is vigilant for\x00" as *const u8 as *const libc::c_char
        } else {
            act =
                b"is ever vigilant for\x00" as *const u8 as
                    *const libc::c_char
        }
        text_out(format(b"%^s %s intruders, which %s may notice from %d feet.  \x00"
                            as *const u8 as *const libc::c_char,
                        wd_he[msex as usize], act, wd_he[msex as usize],
                        10 as libc::c_int * (*r_ptr).aaf as libc::c_int) as
                     cptr);
    }
    /* Drops gold and/or items */
    if (*r_ptr).r_drop_gold as libc::c_int != 0 ||
           (*r_ptr).r_drop_item as libc::c_int != 0 {
        /* No "n" needed */
        sin = 0 as libc::c_int as bool_;
        /* Intro */
        text_out(format(b"%^s may carry\x00" as *const u8 as
                            *const libc::c_char, wd_he[msex as usize]) as
                     cptr);
        /* Count maximum drop */
        n =
            if ((*r_ptr).r_drop_gold as libc::c_int) <
                   (*r_ptr).r_drop_item as libc::c_int {
                (*r_ptr).r_drop_item as libc::c_int
            } else { (*r_ptr).r_drop_gold as libc::c_int };
        /* One drop (may need an "n") */
        if n == 1 as libc::c_int {
            text_out(b" a\x00" as *const u8 as *const libc::c_char);
            sin = 1 as libc::c_int as bool_
        } else if n == 2 as libc::c_int {
            text_out(b" one or two\x00" as *const u8 as *const libc::c_char);
        } else {
            /* Two drops */
            /* Many drops */
            text_out(format(b" up to %d\x00" as *const u8 as
                                *const libc::c_char, n) as cptr);
        }
        /* Great */
        if flags1 & 0x20000000 as libc::c_int as libc::c_uint != 0 {
            p = b" exceptional\x00" as *const u8 as *const libc::c_char
        } else if flags1 & 0x10000000 as libc::c_int as libc::c_uint != 0 {
            p = b" good\x00" as *const u8 as *const libc::c_char;
            sin = 0 as libc::c_int as bool_
        } else {
            /* Good (no "n" needed) */
            /* Okay */
            p = 0 as cptr
        }
        /* Objects */
        if (*r_ptr).r_drop_item != 0 {
            /* Handle singular "an" */
            if sin != 0 {
                text_out(b"n\x00" as *const u8 as *const libc::c_char);
            }
            sin = 0 as libc::c_int as bool_;
            /* Dump "object(s)" */
            if !p.is_null() { text_out_c(3 as libc::c_int as byte_hack, p); }
            text_out(b" object\x00" as *const u8 as *const libc::c_char);
            if n != 1 as libc::c_int {
                text_out(b"s\x00" as *const u8 as *const libc::c_char);
            }
            /* Conjunction replaces variety, if needed for "gold" below */
            p = b" or\x00" as *const u8 as *const libc::c_char
        }
        /* Treasures */
        if (*r_ptr).r_drop_gold != 0 {
            /* Cancel prefix */
            if p.is_null() { sin = 0 as libc::c_int as bool_ }
            /* Handle singular "an" */
            if sin != 0 {
                text_out(b"n\x00" as *const u8 as *const libc::c_char);
            }
            sin = 0 as libc::c_int as bool_;
            /* Dump "treasure(s)" */
            if !p.is_null() { text_out(p); }
            text_out(b" treasure\x00" as *const u8 as *const libc::c_char);
            if n != 1 as libc::c_int {
                text_out(b"s\x00" as *const u8 as *const libc::c_char);
            }
        }
        /* End this sentence */
        text_out(b".  \x00" as *const u8 as *const libc::c_char);
    }
    /* Count the number of "known" attacks */
    n = 0 as libc::c_int;
    m = 0 as libc::c_int;
    while m < 4 as libc::c_int {
        /* Skip non-attacks */
        if !((*r_ptr).blow[m as usize].method == 0) {
            /* Count known attacks */
            if (*r_ptr).r_blows[m as usize] != 0 { n += 1 }
        }
        m += 1
    }
    /* Examine (and count) the actual attacks */
    r = 0 as libc::c_int;
    m = 0 as libc::c_int;
    while m < 4 as libc::c_int {
        let mut method: libc::c_int = 0;
        let mut effect: libc::c_int = 0;
        let mut d1: libc::c_int = 0;
        let mut d2: libc::c_int = 0;
        /* Skip non-attacks */
        if !((*r_ptr).blow[m as usize].method == 0) {
            /* Skip unknown attacks */
            if !((*r_ptr).r_blows[m as usize] == 0) {
                /* Extract the attack info */
                method = (*r_ptr).blow[m as usize].method as libc::c_int;
                effect = (*r_ptr).blow[m as usize].effect as libc::c_int;
                d1 = (*r_ptr).blow[m as usize].d_dice as libc::c_int;
                d2 = (*r_ptr).blow[m as usize].d_side as libc::c_int;
                /* No method yet */
                p = 0 as cptr;
                /* Acquire the method */
                match method {
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
                q = 0 as cptr;
                /* Acquire the effect */
                match effect {
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
                            b"blast sanity\x00" as *const u8 as
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
                /* Introduce the attack description */
                if r == 0 {
                    text_out(format(b"%^s can \x00" as *const u8 as
                                        *const libc::c_char,
                                    wd_he[msex as usize]) as cptr);
                } else if r < n - 1 as libc::c_int {
                    text_out(b", \x00" as *const u8 as *const libc::c_char);
                } else {
                    text_out(b", and \x00" as *const u8 as
                                 *const libc::c_char);
                }
                /* Hack -- force a method */
                if p.is_null() {
                    p =
                        b"do something weird\x00" as *const u8 as
                            *const libc::c_char
                }
                /* Describe the method */
                text_out(p);
                /* Describe the effect (if any) */
                if !q.is_null() {
                    /* Describe the attack type */
                    text_out(b" to \x00" as *const u8 as *const libc::c_char);
                    text_out_c(11 as libc::c_int as byte_hack, q);
                    /* Describe damage (if known) */
                    if d1 != 0 && d2 != 0 &&
                           know_damage(r_idx, m) as libc::c_int != 0 {
                        /* Display the damage */
                        text_out(b" with damage\x00" as *const u8 as
                                     *const libc::c_char);
                        text_out_c(13 as libc::c_int as byte_hack,
                                   format(b" %dd%d\x00" as *const u8 as
                                              *const libc::c_char, d1, d2) as
                                       cptr);
                    }
                }
                /* Count the attacks as printed */
                r += 1
            }
        }
        m += 1
    }
    /* Finish sentence above */
    if r != 0 {
        text_out(b".  \x00" as *const u8 as *const libc::c_char);
    } else if flags1 & 0x10000 as libc::c_int as libc::c_uint != 0 {
        text_out(format(b"%^s has no physical attacks.  \x00" as *const u8 as
                            *const libc::c_char, wd_he[msex as usize]) as
                     cptr);
    } else {
        /* Notice lack of attacks */
        /* Or describe the lack of knowledge */
        text_out(format(b"Nothing is known about %s attack.  \x00" as
                            *const u8 as *const libc::c_char,
                        wd_his[msex as usize]) as cptr);
    }
    /* All done */
    text_out(b"\n\x00" as *const u8 as *const libc::c_char);
    /* Cheat -- know everything */
    if cheat_know as libc::c_int != 0 && remem == 0 as libc::c_int {
        /* Hack -- restore memory */
        memcpy(r_ptr as *mut libc::c_char as *mut libc::c_void,
               &mut save_mem as *mut monster_race as *mut libc::c_char as
                   *const libc::c_void,
               ::std::mem::size_of::<monster_race>() as libc::c_ulong);
    };
}
/*
 * Hack -- Display the "name" and "attr/chars" of a monster race
 */
unsafe extern "C" fn roff_name(mut r_idx: libc::c_int, mut ego: libc::c_int) {
    let mut r_ptr: *mut monster_race = race_info_idx(r_idx, ego);
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
    /* A title (use "The" for non-uniques) */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint == 0 {
        Term_addstr(-(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                    b"The \x00" as *const u8 as *const libc::c_char);
    }
    /* Dump the name */
    if ego != 0 {
        if (*re_info.offset(ego as isize)).before != 0 {
            Term_addstr(-(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                        format(b"%s %s\x00" as *const u8 as
                                   *const libc::c_char,
                               re_name.offset((*re_info.offset(ego as
                                                                   isize)).name
                                                  as isize),
                               r_name.offset((*r_ptr).name as isize)) as
                            cptr);
        } else {
            Term_addstr(-(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                        format(b"%s %s\x00" as *const u8 as
                                   *const libc::c_char,
                               r_name.offset((*r_ptr).name as isize),
                               re_name.offset((*re_info.offset(ego as
                                                                   isize)).name
                                                  as isize)) as cptr);
        }
    } else {
        Term_addstr(-(1 as libc::c_int), 1 as libc::c_int as byte_hack,
                    r_name.offset((*r_ptr).name as isize) as cptr);
    }
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
 * Hack -- Display the "name" and "attr/chars" of a monster race on top
 */
unsafe extern "C" fn roff_top(mut r_idx: libc::c_int, mut ego: libc::c_int) {
    /* Clear the top line */
    Term_erase(0 as libc::c_int, 0 as libc::c_int, 255 as libc::c_int);
    /* Reset the cursor */
    Term_gotoxy(0 as libc::c_int, 0 as libc::c_int);
    roff_name(r_idx, ego);
}
/*
 * Hack -- describe the given monster race at the top of the screen
 */
#[no_mangle]
pub unsafe extern "C" fn screen_roff(mut r_idx: libc::c_int,
                                     mut ego: libc::c_int,
                                     mut remember: libc::c_int) {
    /* Flush messages */
    msg_print(0 as cptr);
    /* Begin recall */
    Term_erase(0 as libc::c_int, 1 as libc::c_int, 255 as libc::c_int);
    /* Recall monster */
    roff_aux(r_idx, ego, remember);
    /* Describe monster */
    roff_top(r_idx, ego);
}
/*
 * Ddescribe the given monster race at the current pos of the "term" window
 */
#[no_mangle]
pub unsafe extern "C" fn monster_description_out(mut r_idx: libc::c_int,
                                                 mut ego: libc::c_int) {
    roff_name(r_idx, ego);
    roff_aux(r_idx, ego, 0 as libc::c_int);
}
/*
 * Hack -- describe the given monster race in the current "term" window
 */
#[no_mangle]
pub unsafe extern "C" fn display_roff(mut r_idx: libc::c_int,
                                      mut ego: libc::c_int) {
    let mut y: libc::c_int = 0;
    /* Erase the window */
    y = 0 as libc::c_int;
    while y < (*Term).hgt as libc::c_int {
        /* Erase the line */
        Term_erase(0 as libc::c_int, y, 255 as libc::c_int);
        y += 1
    }
    /* Begin recall */
    Term_gotoxy(0 as libc::c_int, 1 as libc::c_int);
    /* Recall monster */
    roff_aux(r_idx, ego, 0 as libc::c_int);
    /* Describe monster */
    roff_top(r_idx, ego);
}
#[no_mangle]
pub unsafe extern "C" fn monster_quest(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    /* Random quests are in the dungeon */
    if (*r_ptr).flags8 & 0x1 as libc::c_int as libc::c_uint == 0 {
        return 0 as libc::c_int as bool_
    }
    /* No random quests for aquatic monsters */
    if (*r_ptr).flags7 & 0x1 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* No random quests for multiplying monsters */
    if (*r_ptr).flags4 & 0x2 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn monster_dungeon(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    if (*r_ptr).flags8 & 0x1 as libc::c_int as libc::c_uint != 0 {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
#[no_mangle]
pub unsafe extern "C" fn monster_ocean(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    if (*r_ptr).flags8 & 0x10 as libc::c_int as libc::c_uint != 0 {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
#[no_mangle]
pub unsafe extern "C" fn monster_shore(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    if (*r_ptr).flags8 & 0x8 as libc::c_int as libc::c_uint != 0 {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
#[no_mangle]
pub unsafe extern "C" fn monster_waste(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    if (*r_ptr).flags8 & 0x20 as libc::c_int as libc::c_uint != 0 {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
#[no_mangle]
pub unsafe extern "C" fn monster_town(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    if (*r_ptr).flags8 & 0x2 as libc::c_int as libc::c_uint != 0 {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
#[no_mangle]
pub unsafe extern "C" fn monster_wood(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    if (*r_ptr).flags8 & 0x40 as libc::c_int as libc::c_uint != 0 {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
#[no_mangle]
pub unsafe extern "C" fn monster_volcano(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    if (*r_ptr).flags8 & 0x80 as libc::c_int as libc::c_uint != 0 {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
#[no_mangle]
pub unsafe extern "C" fn monster_mountain(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    if (*r_ptr).flags8 & 0x200 as libc::c_int as libc::c_uint != 0 {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
#[no_mangle]
pub unsafe extern "C" fn monster_grass(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    if (*r_ptr).flags8 & 0x400 as libc::c_int as libc::c_uint != 0 {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
#[no_mangle]
pub unsafe extern "C" fn monster_deep_water(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    if monster_dungeon(r_idx) == 0 { return 0 as libc::c_int as bool_ }
    if (*r_ptr).flags7 & 0x1 as libc::c_int as libc::c_uint != 0 {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
#[no_mangle]
pub unsafe extern "C" fn monster_shallow_water(mut r_idx: libc::c_int)
 -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    if monster_dungeon(r_idx) == 0 { return 0 as libc::c_int as bool_ }
    if (*r_ptr).flags2 & 0x4000 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    } else { return 1 as libc::c_int as bool_ };
}
#[no_mangle]
pub unsafe extern "C" fn monster_lava(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    if monster_dungeon(r_idx) == 0 { return 0 as libc::c_int as bool_ }
    if ((*r_ptr).flags3 & 0x40000 as libc::c_int as libc::c_uint != 0 ||
            (*r_ptr).flags7 & 0x4 as libc::c_int as libc::c_uint != 0) &&
           (*r_ptr).flags3 & 0x400 as libc::c_int as libc::c_uint == 0 {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
#[no_mangle]
pub unsafe extern "C" fn set_mon_num_hook() {
    if dun_level == 0 {
        match (*wf_info.offset((*(*wild_map.offset((*p_ptr).wilderness_y as
                                                       isize)).offset((*p_ptr).wilderness_x
                                                                          as
                                                                          isize)).feat
                                   as isize)).terrain_idx as libc::c_int {
            1 => {
                get_mon_num_hook =
                    Some(monster_town as
                             unsafe extern "C" fn(_: libc::c_int) -> bool_)
            }
            2 => {
                get_mon_num_hook =
                    Some(monster_ocean as
                             unsafe extern "C" fn(_: libc::c_int) -> bool_)
            }
            3 => {
                get_mon_num_hook =
                    Some(monster_shore as
                             unsafe extern "C" fn(_: libc::c_int) -> bool_)
            }
            5 => {
                get_mon_num_hook =
                    Some(monster_waste as
                             unsafe extern "C" fn(_: libc::c_int) -> bool_)
            }
            6 => {
                get_mon_num_hook =
                    Some(monster_grass as
                             unsafe extern "C" fn(_: libc::c_int) -> bool_)
            }
            7 => {
                get_mon_num_hook =
                    Some(monster_wood as
                             unsafe extern "C" fn(_: libc::c_int) -> bool_)
            }
            9 | 10 => {
                get_mon_num_hook =
                    Some(monster_volcano as
                             unsafe extern "C" fn(_: libc::c_int) -> bool_)
            }
            11 => {
                get_mon_num_hook =
                    Some(monster_mountain as
                             unsafe extern "C" fn(_: libc::c_int) -> bool_)
            }
            _ => {
                get_mon_num_hook =
                    Some(monster_dungeon as
                             unsafe extern "C" fn(_: libc::c_int) -> bool_)
            }
        }
    } else {
        get_mon_num_hook =
            Some(monster_dungeon as
                     unsafe extern "C" fn(_: libc::c_int) -> bool_)
    };
}
/*
 * Check if monster can cross terrain
 */
#[no_mangle]
pub unsafe extern "C" fn monster_can_cross_terrain(mut feat: byte_hack,
                                                   mut r_ptr:
                                                       *mut monster_race)
 -> bool_ {
    /* Deep water */
    if feat as libc::c_int == 0xbb as libc::c_int {
        if (*r_ptr).flags7 & 0x1 as libc::c_int as libc::c_uint != 0 ||
               (*r_ptr).flags7 & 0x4 as libc::c_int as libc::c_uint != 0 ||
               (*r_ptr).flags7 & 0x2 as libc::c_int as libc::c_uint != 0 {
            return 1 as libc::c_int as bool_
        } else { return 0 as libc::c_int as bool_ }
    } else {
        /* Shallow water */
        if feat as libc::c_int == 0x54 as libc::c_int {
            if (*r_ptr).flags2 & 0x4000 as libc::c_int as libc::c_uint != 0 {
                return 0 as libc::c_int as bool_
            } else { return 1 as libc::c_int as bool_ }
        } else {
            /* Aquatic monster */
            if (*r_ptr).flags7 & 0x1 as libc::c_int as libc::c_uint != 0 &&
                   (*r_ptr).flags7 & 0x4 as libc::c_int as libc::c_uint == 0 {
                return 0 as libc::c_int as bool_
            } else {
                /* Lava */
                if feat as libc::c_int == 0x56 as libc::c_int ||
                       feat as libc::c_int == 0x55 as libc::c_int {
                    if (*r_ptr).flags3 &
                           0x40000 as libc::c_int as libc::c_uint != 0 ||
                           (*r_ptr).flags7 &
                               0x4 as libc::c_int as libc::c_uint != 0 {
                        return 1 as libc::c_int as bool_
                    } else { return 0 as libc::c_int as bool_ }
                }
            }
        }
    }
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn set_mon_num2_hook(mut y: libc::c_int,
                                           mut x: libc::c_int) {
    /* Set the monster list */
    match (*cave[y as usize].offset(x as isize)).feat as libc::c_int {
        84 => {
            get_mon_num2_hook =
                Some(monster_shallow_water as
                         unsafe extern "C" fn(_: libc::c_int) -> bool_)
        }
        187 => {
            get_mon_num2_hook =
                Some(monster_deep_water as
                         unsafe extern "C" fn(_: libc::c_int) -> bool_)
        }
        85 | 86 => {
            get_mon_num2_hook =
                Some(monster_lava as
                         unsafe extern "C" fn(_: libc::c_int) -> bool_)
        }
        _ => { get_mon_num2_hook = None }
    };
}
