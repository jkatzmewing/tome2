use ::libc;
extern "C" {
    #[no_mangle]
    static mut adj_dex_safe: [byte_hack; 0];
    #[no_mangle]
    static mut death: bool_;
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut testing_carry: bool_;
    #[no_mangle]
    static mut cheat_peek: bool_;
    #[no_mangle]
    static mut cheat_xtra: bool_;
    #[no_mangle]
    static mut health_who: s16b;
    #[no_mangle]
    static mut o_list: *mut object_type;
    #[no_mangle]
    static mut m_list: *mut monster_type;
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut r_name: *mut libc::c_char;
    #[no_mangle]
    static mut carried_monster_hit: bool_;
    #[no_mangle]
    fn disturb(stop_search: libc::c_int, flush_output: libc::c_int);
    #[no_mangle]
    fn is_quest(level: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn symbiote_name(capitalize: bool_) -> cptr;
    #[no_mangle]
    fn race_info_idx(r_idx: libc::c_int, ego: libc::c_int)
     -> *mut monster_race;
    #[no_mangle]
    fn monster_desc(desc: *mut libc::c_char, m_ptr: *mut monster_type,
                    mode: libc::c_int);
    #[no_mangle]
    fn damroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn set_stun(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_cut(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn take_hit(damage: libc::c_int, kb_str: cptr);
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn lose_exp(amount: s32b);
    #[no_mangle]
    fn set_image(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_parasite(v: libc::c_int, r: libc::c_int) -> bool_;
    #[no_mangle]
    fn dec_stat(stat: libc::c_int, amount: libc::c_int, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn set_poisoned(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn earthquake(cy: libc::c_int, cx: libc::c_int, r: libc::c_int);
    #[no_mangle]
    fn do_dec_stat(stat: libc::c_int, mode: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_paralyzed(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_afraid(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_confused(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_blind(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn cold_dam(dam: libc::c_int, kb_str: cptr);
    #[no_mangle]
    fn fire_dam(dam: libc::c_int, kb_str: cptr);
    #[no_mangle]
    fn elec_dam(dam: libc::c_int, kb_str: cptr);
    #[no_mangle]
    fn acid_dam(dam: libc::c_int, kb_str: cptr);
    #[no_mangle]
    fn apply_disenchant(mode: libc::c_int) -> bool_;
    #[no_mangle]
    fn take_sanity_hit(damage: libc::c_int, hit_from: cptr);
    #[no_mangle]
    fn cmsg_print(color: byte_hack, msg: cptr);
    #[no_mangle]
    fn resolve_mimic_name(name: cptr) -> libc::c_int;
    #[no_mangle]
    fn set_mimic(v: libc::c_int, p: libc::c_int, level: libc::c_int) -> bool_;
    #[no_mangle]
    fn get_skill(skill: libc::c_int) -> s16b;
    #[no_mangle]
    fn sound(num: libc::c_int);
    #[no_mangle]
    fn luck(min: libc::c_int, max: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn teleport_away(m_idx: libc::c_int, dis: libc::c_int);
    #[no_mangle]
    fn mon_take_hit(m_idx: libc::c_int, dam: libc::c_int, fear: *mut bool_,
                    note: cptr) -> bool_;
    #[no_mangle]
    fn update_smart_learn(m_idx: libc::c_int, what: libc::c_int);
    #[no_mangle]
    fn inc_stack_size_ex(item: libc::c_int, delta: libc::c_int,
                         opt: optimize_flag, desc: describe_flag);
    #[no_mangle]
    fn index_to_label(i: libc::c_int) -> libc::c_char;
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn move_to_black_market(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_copy(o_ptr: *mut object_type, j_ptr: *mut object_type);
    #[no_mangle]
    fn o_pop() -> s16b;
    #[no_mangle]
    fn monster_carry(m_ptr: *mut monster_type, m_idx: libc::c_int,
                     q_ptr: *mut object_type);
    #[no_mangle]
    fn lookup_kind(tval: libc::c_int, sval: libc::c_int) -> s16b;
    #[no_mangle]
    fn object_prep(o_ptr: *mut object_type, k_idx: libc::c_int);
    #[no_mangle]
    fn object_wipe(o_ptr: *mut object_type);
    #[no_mangle]
    fn swap_position(lty: libc::c_int, ltx: libc::c_int);
    #[no_mangle]
    fn is_friend(m_ptr: *mut monster_type) -> libc::c_int;
}
pub type cptr = *const libc::c_char;
pub type byte_hack = libc::c_uchar;
pub type bool_ = libc::c_char;
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
pub type optimize_flag = libc::c_uint;
pub const NO_OPTIMIZE: optimize_flag = 1;
pub const OPTIMIZE: optimize_flag = 0;
pub type describe_flag = libc::c_uint;
pub const NO_DESCRIBE: describe_flag = 1;
pub const DESCRIBE: describe_flag = 0;
/* File: melee1.c */
/* Purpose: Monster attacks */
/*
 * Copyright (c) 1989 James E. Wilson, Robert A. Koeneke
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Critical blow.  All hits that do 95% of total possible damage,
 * and which also do at least 20 damage, or, sometimes, N damage.
 * This is used only to determine "cuts" and "stuns".
 */
unsafe extern "C" fn monster_critical(mut dice: libc::c_int,
                                      mut sides: libc::c_int,
                                      mut dam: libc::c_int) -> libc::c_int {
    let mut max: libc::c_int = 0 as libc::c_int;
    let mut total: libc::c_int = dice * sides;
    /* Must do at least 95% of perfect */
    if dam < total * 19 as libc::c_int / 20 as libc::c_int {
        return 0 as libc::c_int
    }
    /* Weak blows rarely work */
    if dam < 20 as libc::c_int && Rand_div(100 as libc::c_int) >= dam {
        return 0 as libc::c_int
    }
    /* Perfect damage */
    if dam == total { max += 1 }
    /* Super-charge */
    if dam >= 20 as libc::c_int {
        while Rand_div(100 as libc::c_int) < 2 as libc::c_int { max += 1 }
    }
    /* Critical damage */
    if dam > 45 as libc::c_int { return 6 as libc::c_int + max }
    if dam > 33 as libc::c_int { return 5 as libc::c_int + max }
    if dam > 25 as libc::c_int { return 4 as libc::c_int + max }
    if dam > 18 as libc::c_int { return 3 as libc::c_int + max }
    if dam > 11 as libc::c_int { return 2 as libc::c_int + max }
    return 1 as libc::c_int + max;
}
/*
 * Determine if a monster attack against the player succeeds.
 * Always miss 5% of the time, Always hit 5% of the time.
 * Otherwise, match monster power against player armor.
 */
unsafe extern "C" fn check_hit(mut power: libc::c_int, mut level: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ac: libc::c_int = 0;
    /* Percentile dice */
    k = Rand_div(100 as libc::c_int);
    /* Hack -- Always miss or hit */
    if k < 10 as libc::c_int { return (k < 5 as libc::c_int) as libc::c_int }
    /* Calculate the "attack quality" */
    i = power + level * 3 as libc::c_int;
    /* Total armor */
    ac = (*p_ptr).ac as libc::c_int + (*p_ptr).to_a as libc::c_int;
    /* Power and Level compete against Armor */
    if i > 0 as libc::c_int &&
           Rand_div(i - luck(-(10 as libc::c_int), 10 as libc::c_int)) +
               1 as libc::c_int > ac * 3 as libc::c_int / 4 as libc::c_int {
        return 1 as libc::c_int
    }
    /* Assume miss */
    return 0 as libc::c_int;
}
/*
 * Hack -- possible "insult" messages
 */
static mut desc_insult: [cptr; 8] =
    [b"insults you!\x00" as *const u8 as *const libc::c_char,
     b"insults your mother!\x00" as *const u8 as *const libc::c_char,
     b"jumps around you!\x00" as *const u8 as *const libc::c_char,
     b"humiliates you!\x00" as *const u8 as *const libc::c_char,
     b"defiles you!\x00" as *const u8 as *const libc::c_char,
     b"dances around you!\x00" as *const u8 as *const libc::c_char,
     b"makes obnoxious gestures!\x00" as *const u8 as *const libc::c_char,
     b"pokes you!!!\x00" as *const u8 as *const libc::c_char];
/*
 * Hack -- possible "insult" messages
 */
static mut desc_moan: [cptr; 7] =
    [b"seems sad about something.\x00" as *const u8 as *const libc::c_char,
     b"asks if you have seen his dogs.\x00" as *const u8 as
         *const libc::c_char,
     b"tells you to get off his land.\x00" as *const u8 as
         *const libc::c_char,
     b"mumbles something about mushrooms.\x00" as *const u8 as
         *const libc::c_char,
     b"giggles at you.\x00" as *const u8 as *const libc::c_char,
     b"asks you if you want to giggle with her.\x00" as *const u8 as
         *const libc::c_char,
     b"says she is always happy.\x00" as *const u8 as *const libc::c_char];
/*
 * Get the "power" of an attack of given effect type.
 */
#[no_mangle]
pub unsafe extern "C" fn get_attack_power(mut effect: libc::c_int)
 -> libc::c_int {
    match effect {
        1 => { return 60 as libc::c_int }
        2 => { return 5 as libc::c_int }
        3 => { return 20 as libc::c_int }
        4 => { return 15 as libc::c_int }
        5 => { return 5 as libc::c_int }
        6 => { return 5 as libc::c_int }
        7 => { return 5 as libc::c_int }
        8 => { return 5 as libc::c_int }
        9 => { return 0 as libc::c_int }
        10 => { return 10 as libc::c_int }
        11 => { return 10 as libc::c_int }
        12 => { return 10 as libc::c_int }
        13 => { return 2 as libc::c_int }
        14 => { return 10 as libc::c_int }
        15 => { return 10 as libc::c_int }
        16 => { return 2 as libc::c_int }
        17 => { return 0 as libc::c_int }
        20 => { return 0 as libc::c_int }
        21 => { return 0 as libc::c_int }
        18 => { return 0 as libc::c_int }
        19 => { return 0 as libc::c_int }
        22 => { return 0 as libc::c_int }
        23 => { return 2 as libc::c_int }
        24 => { return 60 as libc::c_int }
        25 => { return 5 as libc::c_int }
        26 => { return 5 as libc::c_int }
        27 => { return 5 as libc::c_int }
        28 => { return 5 as libc::c_int }
        29 => { return 5 as libc::c_int }
        30 => { return 5 as libc::c_int }
        31 => { return 60 as libc::c_int }
        32 => { return 10 as libc::c_int }
        33 => { return 5 as libc::c_int }
        34 => { return 30 as libc::c_int }
        _ => { }
    }
    /* Unknown effects have no power */
    return 0 as libc::c_int;
}
/*
 * Attack the player via physical attacks.
 */
#[no_mangle]
pub unsafe extern "C" fn carried_make_attack_normal(mut r_idx: libc::c_int)
 -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    let mut ap_cnt: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut ac: libc::c_int = 0;
    let mut rlev: libc::c_int = 0;
    let mut do_cut: libc::c_int = 0;
    let mut do_stun: libc::c_int = 0;
    let mut ddesc: [libc::c_char; 80] =
        *::std::mem::transmute::<&[u8; 80],
                                 &mut [libc::c_char; 80]>(b"your symbiote\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let mut sym_name: cptr = symbiote_name(1 as libc::c_int as bool_);
    let mut touched: bool_ = 0 as libc::c_int as bool_;
    let mut alive: bool_ = 1 as libc::c_int as bool_;
    /* Not allowed to attack */
    if (*r_ptr).flags1 & 0x10000 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Total armor */
    ac = (*p_ptr).ac as libc::c_int + (*p_ptr).to_a as libc::c_int;
    /* Extract the effective monster level */
    rlev =
        if (*r_ptr).level as libc::c_int >= 1 as libc::c_int {
            (*r_ptr).level as libc::c_int
        } else { 1 as libc::c_int };
    let mut current_block_383: u64;
    /* Scan through all four blows */
    ap_cnt = 0 as libc::c_int;
    while ap_cnt < 4 as libc::c_int {
        let mut visible: bool_ = 0 as libc::c_int as bool_;
        let mut obvious: bool_ = 0 as libc::c_int as bool_;
        let mut power: libc::c_int = 0 as libc::c_int;
        let mut damage: libc::c_int = 0 as libc::c_int;
        let mut act: cptr = 0 as cptr;
        /* Extract the attack infomation */
        let mut effect: libc::c_int =
            (*r_ptr).blow[ap_cnt as usize].effect as libc::c_int;
        let mut method: libc::c_int =
            (*r_ptr).blow[ap_cnt as usize].method as libc::c_int;
        let mut d_dice: libc::c_int =
            (*r_ptr).blow[ap_cnt as usize].d_dice as libc::c_int;
        let mut d_side: libc::c_int =
            (*r_ptr).blow[ap_cnt as usize].d_side as libc::c_int;
        /* Hack -- no more attacks */
        if method == 0 { break ; }
        /* Stop if player is dead or gone */
        if alive == 0 || death as libc::c_int != 0 { break ; }
        /* Handle "leaving" */
        if (*p_ptr).leaving != 0 { break ; }
        /* Extract visibility (before blink) */
        visible = 1 as libc::c_int as bool_;
        /* Extract the attack "power" */
        power = get_attack_power(effect);
        /* Monster hits player */
        if effect == 0 || check_hit(power, rlev) != 0 {
            /* Always disturbing */
            disturb(1 as libc::c_int, 0 as libc::c_int);
            /* Hack -- Apply "protection from evil" */
            if (*p_ptr).protevil as libc::c_int > 0 as libc::c_int &&
                   (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint != 0
                   && (*p_ptr).lev as libc::c_int >= rlev &&
                   Rand_div(100 as libc::c_int) + (*p_ptr).lev as libc::c_int
                       > 50 as libc::c_int {
                /* Remember the Evil-ness */
                (*r_ptr).r_flags3 |= 0x40 as libc::c_int as libc::c_uint;
                /* Message */
                msg_format(b"%s is repelled.\x00" as *const u8 as
                               *const libc::c_char, sym_name);
                /* Hack -- Next attack */
                current_block_383 = 14523784380283086299;
            } else if (*p_ptr).protgood as libc::c_int > 0 as libc::c_int &&
                          (*r_ptr).flags3 &
                              0x200 as libc::c_int as libc::c_uint != 0 &&
                          (*p_ptr).lev as libc::c_int >= rlev &&
                          Rand_div(100 as libc::c_int) +
                              (*p_ptr).lev as libc::c_int > 50 as libc::c_int
             {
                /* Hack -- Apply "protection from good" */
                /* Remember the Good-ness */
                (*r_ptr).r_flags3 |= 0x200 as libc::c_int as libc::c_uint;
                /* Message */
                msg_format(b"%s is repelled.\x00" as *const u8 as
                               *const libc::c_char, sym_name);
                current_block_383 = 14523784380283086299;
            } else {
                /* Assume no cut or stun */
                do_stun = 0 as libc::c_int;
                do_cut = do_stun;
                /* Describe the attack method */
                match method {
                    1 => {
                        act =
                            b"hits you.\x00" as *const u8 as
                                *const libc::c_char; /* Note! This is "charges", not "charges at". */
                        do_stun = 1 as libc::c_int;
                        do_cut = do_stun;
                        touched = 1 as libc::c_int as bool_;
                        sound(1 as libc::c_int);
                    }
                    2 => {
                        act =
                            b"touches you.\x00" as *const u8 as
                                *const libc::c_char;
                        touched = 1 as libc::c_int as bool_;
                        sound(44 as libc::c_int);
                    }
                    3 => {
                        act =
                            b"punches you.\x00" as *const u8 as
                                *const libc::c_char;
                        touched = 1 as libc::c_int as bool_;
                        do_stun = 1 as libc::c_int;
                        sound(1 as libc::c_int);
                    }
                    4 => {
                        act =
                            b"kicks you.\x00" as *const u8 as
                                *const libc::c_char;
                        touched = 1 as libc::c_int as bool_;
                        do_stun = 1 as libc::c_int;
                        sound(1 as libc::c_int);
                    }
                    5 => {
                        act =
                            b"claws you.\x00" as *const u8 as
                                *const libc::c_char;
                        touched = 1 as libc::c_int as bool_;
                        do_cut = 1 as libc::c_int;
                        sound(36 as libc::c_int);
                    }
                    6 => {
                        act =
                            b"bites you.\x00" as *const u8 as
                                *const libc::c_char;
                        do_cut = 1 as libc::c_int;
                        touched = 1 as libc::c_int as bool_;
                        sound(35 as libc::c_int);
                    }
                    7 => {
                        act =
                            b"stings you.\x00" as *const u8 as
                                *const libc::c_char;
                        touched = 1 as libc::c_int as bool_;
                        sound(45 as libc::c_int);
                    }
                    8 => {
                        act =
                            b"XXX1\'s you.\x00" as *const u8 as
                                *const libc::c_char
                    }
                    9 => {
                        act =
                            b"butts you.\x00" as *const u8 as
                                *const libc::c_char;
                        do_stun = 1 as libc::c_int;
                        touched = 1 as libc::c_int as bool_;
                        sound(1 as libc::c_int);
                    }
                    10 => {
                        act =
                            b"crushes you.\x00" as *const u8 as
                                *const libc::c_char;
                        do_stun = 1 as libc::c_int;
                        touched = 1 as libc::c_int as bool_;
                        sound(46 as libc::c_int);
                    }
                    11 => {
                        act =
                            b"engulfs you.\x00" as *const u8 as
                                *const libc::c_char;
                        touched = 1 as libc::c_int as bool_;
                        sound(46 as libc::c_int);
                    }
                    12 => {
                        act =
                            b"charges you.\x00" as *const u8 as
                                *const libc::c_char;
                        touched = 1 as libc::c_int as bool_;
                        sound(26 as libc::c_int);
                    }
                    13 => {
                        act =
                            b"crawls on you.\x00" as *const u8 as
                                *const libc::c_char;
                        touched = 1 as libc::c_int as bool_;
                        sound(47 as libc::c_int);
                    }
                    14 => {
                        act =
                            b"drools on you.\x00" as *const u8 as
                                *const libc::c_char;
                        sound(47 as libc::c_int);
                    }
                    15 => {
                        act =
                            b"spits on you.\x00" as *const u8 as
                                *const libc::c_char;
                        sound(47 as libc::c_int);
                    }
                    16 => {
                        act =
                            b"explodes.\x00" as *const u8 as
                                *const libc::c_char
                    }
                    17 => {
                        act =
                            b"gazes at you.\x00" as *const u8 as
                                *const libc::c_char
                    }
                    18 => {
                        act =
                            b"wails at you.\x00" as *const u8 as
                                *const libc::c_char;
                        sound(48 as libc::c_int);
                    }
                    19 => {
                        act =
                            b"releases spores at you.\x00" as *const u8 as
                                *const libc::c_char;
                        sound(47 as libc::c_int);
                    }
                    20 => {
                        act =
                            b"projects XXX4\'s at you.\x00" as *const u8 as
                                *const libc::c_char
                    }
                    21 => {
                        act =
                            b"begs you for money.\x00" as *const u8 as
                                *const libc::c_char;
                        sound(61 as libc::c_int);
                    }
                    22 => {
                        act =
                            desc_insult[Rand_div(8 as libc::c_int) as usize];
                        sound(61 as libc::c_int);
                    }
                    23 => {
                        act = desc_moan[Rand_div(4 as libc::c_int) as usize];
                        sound(61 as libc::c_int);
                    }
                    24 => {
                        if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                               1 as libc::c_int {
                            act =
                                b"sings \'We are a happy family.\'\x00" as
                                    *const u8 as *const libc::c_char
                        } else {
                            act =
                                b"sings \'I love you, you love me.\'\x00" as
                                    *const u8 as *const libc::c_char
                        }
                        sound(62 as libc::c_int);
                    }
                    _ => { }
                }
                /* Message */
                if !act.is_null() {
                    msg_format(b"%s %s\x00" as *const u8 as
                                   *const libc::c_char, sym_name, act);
                }
                /* Hack -- assume all attacks are obvious */
                obvious = 1 as libc::c_int as bool_;
                /* Roll out the damage */
                damage = damroll(d_dice as s16b, d_side as s16b);
                /* Apply appropriate damage */
                match effect {
                    0 => {
                        /* Hack -- Assume obvious */
                        obvious = 1 as libc::c_int as bool_;
                        /* Hack -- No damage */
                        damage = 0 as libc::c_int
                    }
                    1 => {
                        /* Obvious */
                        obvious = 1 as libc::c_int as bool_;
                        /* Hack -- Player armor reduces total damage */
                        damage -=
                            damage *
                                (if ac < 150 as libc::c_int {
                                     ac
                                 } else { 150 as libc::c_int }) /
                                250 as libc::c_int;
                        /* Take damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                    }
                    34 => {
                        /* Obvious */
                        obvious = 1 as libc::c_int as bool_;
                        /* Morph, but let mimicry skill have a chance to stop this */
                        if Rand_div(100 as libc::c_int) <
                               60 as libc::c_int -
                                   get_skill(32 as libc::c_int) as libc::c_int
                           {
                            /* Message */
                            cmsg_print(10 as libc::c_int as byte_hack,
                                       b"You feel the dark powers twisting your body!\x00"
                                           as *const u8 as
                                           *const libc::c_char);
                            set_mimic(damage,
                                      resolve_mimic_name(b"Abomination\x00" as
                                                             *const u8 as
                                                             *const libc::c_char),
                                      50 as libc::c_int);
                        } else {
                            /* Message */
                            cmsg_print(10 as libc::c_int as byte_hack,
                                       b"You feel the dark powers trying to twisting your body, but they fail.\x00"
                                           as *const u8 as
                                           *const libc::c_char);
                        }
                    }
                    31 => {
                        obvious = 1 as libc::c_int as bool_;
                        take_sanity_hit(damage, ddesc.as_mut_ptr() as cptr);
                    }
                    2 => {
                        /* Take some damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                        /* Take "poison" effect */
                        if !((*p_ptr).resist_pois as libc::c_int != 0 ||
                                 (*p_ptr).oppose_pois as libc::c_int != 0) {
                            if set_poisoned((*p_ptr).poisoned as libc::c_int +
                                                (Rand_div(rlev) +
                                                     1 as libc::c_int) +
                                                5 as libc::c_int) != 0 {
                                obvious = 1 as libc::c_int as bool_
                            }
                        }
                    }
                    3 => {
                        /* Take some damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                        /* Allow complete resist */
                        if (*p_ptr).resist_disen == 0 {
                            /* Apply disenchantment */
                            if apply_disenchant(0 as libc::c_int) != 0 {
                                obvious = 1 as libc::c_int as bool_
                            }
                        }
                    }
                    4 => {
                        /* Take some damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                    }
                    5 => {
                        /* Take some damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                    }
                    6 => {
                        /* Take some damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                    }
                    7 => {
                        /* Take some damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                    }
                    8 => {
                        /* Take some damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                    }
                    9 => {
                        /* Obvious */
                        obvious = 1 as libc::c_int as bool_;
                        /* Message */
                        msg_print(b"You are covered in acid!\x00" as *const u8
                                      as *const libc::c_char);
                        /* Special damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        acid_dam(damage, ddesc.as_mut_ptr() as cptr);
                    }
                    10 => {
                        /* Obvious */
                        obvious = 1 as libc::c_int as bool_;
                        /* Message */
                        msg_print(b"You are struck by electricity!\x00" as
                                      *const u8 as *const libc::c_char);
                        /* Special damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        elec_dam(damage, ddesc.as_mut_ptr() as cptr);
                    }
                    11 => {
                        /* Obvious */
                        obvious = 1 as libc::c_int as bool_;
                        /* Message */
                        msg_print(b"You are enveloped in flames!\x00" as
                                      *const u8 as *const libc::c_char);
                        /* Special damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        fire_dam(damage, ddesc.as_mut_ptr() as cptr);
                    }
                    12 => {
                        /* Obvious */
                        obvious = 1 as libc::c_int as bool_;
                        /* Message */
                        msg_print(b"You are covered with frost!\x00" as
                                      *const u8 as *const libc::c_char);
                        /* Special damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        cold_dam(damage, ddesc.as_mut_ptr() as cptr);
                    }
                    13 => {
                        /* Take damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                        /* Increase "blind" */
                        if (*p_ptr).resist_blind == 0 {
                            if set_blind((*p_ptr).blind as libc::c_int +
                                             10 as libc::c_int +
                                             (Rand_div(rlev) +
                                                  1 as libc::c_int)) != 0 {
                                obvious = 1 as libc::c_int as bool_
                            }
                        }
                    }
                    14 => {
                        /* Take damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                        /* Increase "confused" */
                        if (*p_ptr).resist_conf == 0 {
                            if set_confused((*p_ptr).confused as libc::c_int +
                                                3 as libc::c_int +
                                                (Rand_div(rlev) +
                                                     1 as libc::c_int)) != 0 {
                                obvious = 1 as libc::c_int as bool_
                            }
                        }
                    }
                    15 => {
                        /* Take damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                        /* Increase "afraid" */
                        if (*p_ptr).resist_fear != 0 {
                            msg_print(b"You stand your ground!\x00" as
                                          *const u8 as *const libc::c_char);
                            obvious = 1 as libc::c_int as bool_
                        } else if Rand_div(100 as libc::c_int) <
                                      (*p_ptr).skill_sav as libc::c_int {
                            msg_print(b"You stand your ground!\x00" as
                                          *const u8 as *const libc::c_char);
                            obvious = 1 as libc::c_int as bool_
                        } else if set_afraid((*p_ptr).afraid as libc::c_int +
                                                 3 as libc::c_int +
                                                 (Rand_div(rlev) +
                                                      1 as libc::c_int)) != 0
                         {
                            obvious = 1 as libc::c_int as bool_
                        }
                    }
                    16 => {
                        /* Hack -- Prevent perma-paralysis via damage */
                        if (*p_ptr).paralyzed as libc::c_int != 0 &&
                               damage < 1 as libc::c_int {
                            damage = 1 as libc::c_int
                        }
                        /* Take damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                        /* Increase "paralyzed" */
                        if (*p_ptr).free_act != 0 {
                            msg_print(b"You are unaffected!\x00" as *const u8
                                          as *const libc::c_char);
                            obvious = 1 as libc::c_int as bool_
                        } else if Rand_div(100 as libc::c_int) <
                                      (*p_ptr).skill_sav as libc::c_int {
                            msg_print(b"You resist the effects!\x00" as
                                          *const u8 as *const libc::c_char);
                            obvious = 1 as libc::c_int as bool_
                        } else if set_paralyzed((*p_ptr).paralyzed as
                                                    libc::c_int +
                                                    3 as libc::c_int +
                                                    (Rand_div(rlev) +
                                                         1 as libc::c_int)) !=
                                      0 {
                            obvious = 1 as libc::c_int as bool_
                        }
                    }
                    17 => {
                        /* Damage (physical) */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                        /* Damage (stat) */
                        if do_dec_stat(0 as libc::c_int, 2 as libc::c_int) !=
                               0 {
                            obvious = 1 as libc::c_int as bool_
                        }
                    }
                    18 => {
                        /* Damage (physical) */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                        /* Damage (stat) */
                        if do_dec_stat(1 as libc::c_int, 2 as libc::c_int) !=
                               0 {
                            obvious = 1 as libc::c_int as bool_
                        }
                    }
                    19 => {
                        /* Damage (physical) */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                        /* Damage (stat) */
                        if do_dec_stat(2 as libc::c_int, 2 as libc::c_int) !=
                               0 {
                            obvious = 1 as libc::c_int as bool_
                        }
                    }
                    20 => {
                        /* Damage (physical) */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                        /* Damage (stat) */
                        if do_dec_stat(3 as libc::c_int, 2 as libc::c_int) !=
                               0 {
                            obvious = 1 as libc::c_int as bool_
                        }
                    }
                    21 => {
                        /* Damage (physical) */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                        /* Damage (stat) */
                        if do_dec_stat(4 as libc::c_int, 2 as libc::c_int) !=
                               0 {
                            obvious = 1 as libc::c_int as bool_
                        }
                    }
                    22 => {
                        /* Damage (physical) */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                        /* Damage (stat) */
                        if do_dec_stat(5 as libc::c_int, 2 as libc::c_int) !=
                               0 {
                            obvious = 1 as libc::c_int as bool_
                        }
                    }
                    23 => {
                        /* Damage (physical) */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                        /* Damage (stats) */
                        if do_dec_stat(0 as libc::c_int, 2 as libc::c_int) !=
                               0 {
                            obvious = 1 as libc::c_int as bool_
                        }
                        if do_dec_stat(3 as libc::c_int, 2 as libc::c_int) !=
                               0 {
                            obvious = 1 as libc::c_int as bool_
                        }
                        if do_dec_stat(4 as libc::c_int, 2 as libc::c_int) !=
                               0 {
                            obvious = 1 as libc::c_int as bool_
                        }
                        if do_dec_stat(1 as libc::c_int, 2 as libc::c_int) !=
                               0 {
                            obvious = 1 as libc::c_int as bool_
                        }
                        if do_dec_stat(2 as libc::c_int, 2 as libc::c_int) !=
                               0 {
                            obvious = 1 as libc::c_int as bool_
                        }
                        if do_dec_stat(5 as libc::c_int, 2 as libc::c_int) !=
                               0 {
                            obvious = 1 as libc::c_int as bool_
                        }
                    }
                    24 => {
                        /* Obvious */
                        obvious = 1 as libc::c_int as bool_;
                        /* Hack -- Reduce damage based on the player armor class */
                        damage -=
                            damage *
                                (if ac < 150 as libc::c_int {
                                     ac
                                 } else { 150 as libc::c_int }) /
                                250 as libc::c_int;
                        /* Take damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                        /* Radius 8 earthquake centered at the monster */
                        if damage > 23 as libc::c_int {
                            /* Prevent destruction of quest levels and town */
                            if is_quest(dun_level as libc::c_int) == 0 &&
                                   dun_level as libc::c_int != 0 {
                                earthquake((*p_ptr).py as libc::c_int,
                                           (*p_ptr).px as libc::c_int,
                                           8 as libc::c_int);
                            }
                        }
                    }
                    25 => {
                        /* Obvious */
                        obvious = 1 as libc::c_int as bool_;
                        /* Take damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                        if (*p_ptr).hold_life as libc::c_int != 0 &&
                               Rand_div(100 as libc::c_int) <
                                   95 as libc::c_int {
                            msg_print(b"You keep hold of your life force!\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                        } else {
                            let mut d: s32b =
                                damroll(10 as libc::c_int as s16b,
                                        6 as libc::c_int as s16b) +
                                    (*p_ptr).exp / 100 as libc::c_int *
                                        2 as libc::c_int;
                            if (*p_ptr).hold_life != 0 {
                                msg_print(b"You feel your life slipping away!\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                lose_exp(d / 10 as libc::c_int);
                            } else {
                                msg_print(b"You feel your life draining away!\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                lose_exp(d);
                            }
                        }
                    }
                    26 => {
                        /* Obvious */
                        obvious = 1 as libc::c_int as bool_;
                        /* Take damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                        if (*p_ptr).hold_life as libc::c_int != 0 &&
                               Rand_div(100 as libc::c_int) <
                                   90 as libc::c_int {
                            msg_print(b"You keep hold of your life force!\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                        } else {
                            let mut d_0: s32b =
                                damroll(20 as libc::c_int as s16b,
                                        6 as libc::c_int as s16b) +
                                    (*p_ptr).exp / 100 as libc::c_int *
                                        2 as libc::c_int;
                            if (*p_ptr).hold_life != 0 {
                                msg_print(b"You feel your life slipping away!\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                lose_exp(d_0 / 10 as libc::c_int);
                            } else {
                                msg_print(b"You feel your life draining away!\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                lose_exp(d_0);
                            }
                        }
                    }
                    27 => {
                        /* Obvious */
                        obvious = 1 as libc::c_int as bool_;
                        /* Take damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                        if (*p_ptr).hold_life as libc::c_int != 0 &&
                               Rand_div(100 as libc::c_int) <
                                   75 as libc::c_int {
                            msg_print(b"You keep hold of your life force!\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                        } else {
                            let mut d_1: s32b =
                                damroll(40 as libc::c_int as s16b,
                                        6 as libc::c_int as s16b) +
                                    (*p_ptr).exp / 100 as libc::c_int *
                                        2 as libc::c_int;
                            if (*p_ptr).hold_life != 0 {
                                msg_print(b"You feel your life slipping away!\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                lose_exp(d_1 / 10 as libc::c_int);
                            } else {
                                msg_print(b"You feel your life draining away!\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                lose_exp(d_1);
                            }
                        }
                    }
                    28 => {
                        /* Obvious */
                        obvious = 1 as libc::c_int as bool_;
                        /* Take damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                        if (*p_ptr).hold_life as libc::c_int != 0 &&
                               Rand_div(100 as libc::c_int) <
                                   50 as libc::c_int {
                            msg_print(b"You keep hold of your life force!\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                        } else {
                            let mut d_2: s32b =
                                damroll(80 as libc::c_int as s16b,
                                        6 as libc::c_int as s16b) +
                                    (*p_ptr).exp / 100 as libc::c_int *
                                        2 as libc::c_int;
                            if (*p_ptr).hold_life != 0 {
                                msg_print(b"You feel your life slipping away!\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                lose_exp(d_2 / 10 as libc::c_int);
                            } else {
                                msg_print(b"You feel your life draining away!\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                lose_exp(d_2);
                            }
                        }
                    }
                    29 => {
                        /* Take some damage */
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                        /* Take "poison" effect */
                        if !((*p_ptr).resist_pois as libc::c_int != 0 ||
                                 (*p_ptr).oppose_pois as libc::c_int != 0) {
                            if set_poisoned((*p_ptr).poisoned as libc::c_int +
                                                (Rand_div(rlev) +
                                                     1 as libc::c_int) +
                                                5 as libc::c_int) != 0 {
                                obvious = 1 as libc::c_int as bool_
                            }
                        }
                        /* Damage CON (10% chance)*/
                        if (Rand_div(100 as libc::c_int) + 1 as libc::c_int) <
                               11 as libc::c_int {
                            /* 1% chance for perm. damage */
                            let mut perm: bool_ =
                                (Rand_div(10 as libc::c_int) +
                                     1 as libc::c_int == 1 as libc::c_int) as
                                    libc::c_int as bool_;
                            if dec_stat(4 as libc::c_int,
                                        Rand_div(10 as libc::c_int) +
                                            1 as libc::c_int,
                                        perm as libc::c_int) != 0 {
                                obvious = 1 as libc::c_int as bool_
                            }
                        }
                    }
                    33 => {
                        /* Obvious */
                        obvious = 1 as libc::c_int as bool_;
                        if (*p_ptr).parasite == 0 {
                            set_parasite(damage, r_idx);
                        }
                    }
                    32 => {
                        /* Take damage */
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                        /* Increase "image" */
                        if (*p_ptr).resist_chaos == 0 {
                            if set_image((*p_ptr).image as libc::c_int +
                                             3 as libc::c_int +
                                             (Rand_div(rlev /
                                                           2 as libc::c_int) +
                                                  1 as libc::c_int)) != 0 {
                                obvious = 1 as libc::c_int as bool_
                            }
                        }
                    }
                    30 => {
                        match Rand_div(10 as libc::c_int) + 1 as libc::c_int {
                            1 | 2 | 3 | 4 | 5 => {
                                msg_print(b"You feel life has clocked back.\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                lose_exp(100 as libc::c_int +
                                             (*p_ptr).exp / 100 as libc::c_int
                                                 * 2 as libc::c_int);
                            }
                            6 | 7 | 8 | 9 => {
                                let mut stat: libc::c_int =
                                    Rand_div(6 as libc::c_int);
                                match stat {
                                    0 => {
                                        act =
                                            b"strong\x00" as *const u8 as
                                                *const libc::c_char
                                    }
                                    1 => {
                                        act =
                                            b"bright\x00" as *const u8 as
                                                *const libc::c_char
                                    }
                                    2 => {
                                        act =
                                            b"wise\x00" as *const u8 as
                                                *const libc::c_char
                                    }
                                    3 => {
                                        act =
                                            b"agile\x00" as *const u8 as
                                                *const libc::c_char
                                    }
                                    4 => {
                                        act =
                                            b"hardy\x00" as *const u8 as
                                                *const libc::c_char
                                    }
                                    5 => {
                                        act =
                                            b"beautiful\x00" as *const u8 as
                                                *const libc::c_char
                                    }
                                    _ => { }
                                }
                                msg_format(b"You\'re not as %s as you used to be...\x00"
                                               as *const u8 as
                                               *const libc::c_char, act);
                                (*p_ptr).stat_cur[stat as usize] =
                                    ((*p_ptr).stat_cur[stat as usize] as
                                         libc::c_int * 3 as libc::c_int /
                                         4 as libc::c_int) as s16b;
                                if ((*p_ptr).stat_cur[stat as usize] as
                                        libc::c_int) < 3 as libc::c_int {
                                    (*p_ptr).stat_cur[stat as usize] =
                                        3 as libc::c_int as s16b
                                }
                                (*p_ptr).update =
                                    ((*p_ptr).update as libc::c_long |
                                         0x1 as libc::c_long) as u32b
                            }
                            10 => {
                                msg_print(b"You\'re not as powerful as you used to be...\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                k = 0 as libc::c_int;
                                while k < 6 as libc::c_int {
                                    (*p_ptr).stat_cur[k as usize] =
                                        ((*p_ptr).stat_cur[k as usize] as
                                             libc::c_int * 3 as libc::c_int /
                                             4 as libc::c_int) as s16b;
                                    if ((*p_ptr).stat_cur[k as usize] as
                                            libc::c_int) < 3 as libc::c_int {
                                        (*p_ptr).stat_cur[k as usize] =
                                            3 as libc::c_int as s16b
                                    }
                                    k += 1
                                }
                                (*p_ptr).update =
                                    ((*p_ptr).update as libc::c_long |
                                         0x1 as libc::c_long) as u32b
                            }
                            _ => { }
                        }
                        carried_monster_hit = 1 as libc::c_int as bool_;
                        take_hit(damage, ddesc.as_mut_ptr() as cptr);
                    }
                    _ => { }
                }
                /* Hack -- only one of cut or stun */
                if do_cut != 0 && do_stun != 0 {
                    /* Cancel cut */
                    if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
                        do_cut = 0 as libc::c_int
                    } else {
                        /* Cancel stun */
                        do_stun = 0 as libc::c_int
                    }
                }
                /* Handle cut */
                if do_cut != 0 {
                    let mut k_0: libc::c_int = 0 as libc::c_int;
                    /* Critical hit (zero if non-critical) */
                    tmp = monster_critical(d_dice, d_side, damage);
                    /* Roll for damage */
                    match tmp {
                        0 => { k_0 = 0 as libc::c_int }
                        1 => {
                            k_0 =
                                Rand_div(5 as libc::c_int) + 1 as libc::c_int
                        }
                        2 => {
                            k_0 =
                                Rand_div(5 as libc::c_int) + 1 as libc::c_int
                                    + 5 as libc::c_int
                        }
                        3 => {
                            k_0 =
                                Rand_div(20 as libc::c_int) + 1 as libc::c_int
                                    + 20 as libc::c_int
                        }
                        4 => {
                            k_0 =
                                Rand_div(50 as libc::c_int) + 1 as libc::c_int
                                    + 50 as libc::c_int
                        }
                        5 => {
                            k_0 =
                                Rand_div(100 as libc::c_int) +
                                    1 as libc::c_int + 100 as libc::c_int
                        }
                        6 => { k_0 = 300 as libc::c_int }
                        _ => { k_0 = 500 as libc::c_int }
                    }
                    /* Apply the cut */
                    if k_0 != 0 {
                        set_cut((*p_ptr).cut as libc::c_int + k_0);
                    }
                }
                /* Handle stun */
                if do_stun != 0 {
                    let mut k_1: libc::c_int = 0 as libc::c_int;
                    /* Critical hit (zero if non-critical) */
                    tmp = monster_critical(d_dice, d_side, damage);
                    /* Roll for damage */
                    match tmp {
                        0 => { k_1 = 0 as libc::c_int }
                        1 => {
                            k_1 =
                                Rand_div(5 as libc::c_int) + 1 as libc::c_int
                        }
                        2 => {
                            k_1 =
                                Rand_div(10 as libc::c_int) + 1 as libc::c_int
                                    + 10 as libc::c_int
                        }
                        3 => {
                            k_1 =
                                Rand_div(20 as libc::c_int) + 1 as libc::c_int
                                    + 20 as libc::c_int
                        }
                        4 => {
                            k_1 =
                                Rand_div(30 as libc::c_int) + 1 as libc::c_int
                                    + 30 as libc::c_int
                        }
                        5 => {
                            k_1 =
                                Rand_div(40 as libc::c_int) + 1 as libc::c_int
                                    + 40 as libc::c_int
                        }
                        6 => { k_1 = 100 as libc::c_int }
                        _ => { k_1 = 200 as libc::c_int }
                    }
                    /* Apply the stun */
                    if k_1 != 0 {
                        set_stun((*p_ptr).stun as libc::c_int + k_1);
                    }
                }
                if touched != 0 {
                    if (*p_ptr).sh_fire as libc::c_int != 0 &&
                           alive as libc::c_int != 0 {
                        (*r_ptr).r_flags3 |=
                            0x40000 as libc::c_int as libc::c_uint
                    }
                    if (*p_ptr).sh_elec as libc::c_int != 0 &&
                           alive as libc::c_int != 0 {
                        (*r_ptr).r_flags3 |=
                            0x20000 as libc::c_int as libc::c_uint
                    }
                    touched = 0 as libc::c_int as bool_
                }
                current_block_383 = 8889999340123292593;
            }
        } else {
            /* Monster missed player */
            /* Analyze failed attacks */
            match method {
                1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 => {
                    /* Disturbing */
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    /* Message */
                    msg_format(b"%s misses you.\x00" as *const u8 as
                                   *const libc::c_char, sym_name);
                }
                _ => { }
            }
            current_block_383 = 8889999340123292593;
        }
        match current_block_383 {
            8889999340123292593 => {
                /* Analyze "visible" monsters only */
                if visible != 0 {
                    /* Count "obvious" attacks (and ones that cause damage) */
                    if obvious as libc::c_int != 0 || damage != 0 ||
                           (*r_ptr).r_blows[ap_cnt as usize] as libc::c_int >
                               10 as libc::c_int {
                        /* Count attacks of this type */
                        if ((*r_ptr).r_blows[ap_cnt as usize] as libc::c_int)
                               < 255 as libc::c_int {
                            (*r_ptr).r_blows[ap_cnt as usize] =
                                (*r_ptr).r_blows[ap_cnt as
                                                     usize].wrapping_add(1)
                        }
                    }
                }
            }
            _ => { }
        }
        /* Hack -- Next attack */
        ap_cnt += 1
    }
    /* Assume we attacked */
    return 1 as libc::c_int as bool_;
}
/*
 * Give unprotected player the Black Breath with a 1 in (chance) probability
 *
 */
#[no_mangle]
pub unsafe extern "C" fn black_breath_attack(mut chance: libc::c_int) {
    if (*p_ptr).protundead == 0 &&
           Rand_div(chance) + 1 as libc::c_int == 1 as libc::c_int {
        msg_print(b"Your foe calls upon your soul!\x00" as *const u8 as
                      *const libc::c_char);
        msg_print(b"You feel the Black Breath slowly draining you of life...\x00"
                      as *const u8 as *const libc::c_char);
        (*p_ptr).black_breath = 1 as libc::c_int as bool_
    };
}
/*
 * Attack the player via physical attacks.
 */
#[no_mangle]
pub unsafe extern "C" fn make_attack_normal(mut m_idx: libc::c_int,
                                            mut divis: byte_hack) -> bool_ {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    let mut ap_cnt: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut ac: libc::c_int = 0;
    let mut rlev: libc::c_int = 0;
    let mut do_cut: libc::c_int = 0;
    let mut do_stun: libc::c_int = 0;
    let mut do_vampire: libc::c_int = 0;
    let mut gold: s32b = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut m_name: [libc::c_char; 80] = [0; 80];
    let mut ddesc: [libc::c_char; 80] = [0; 80];
    let mut blinked: bool_ = 0;
    let mut touched: bool_ = 0 as libc::c_int as bool_;
    let mut fear: bool_ = 0 as libc::c_int as bool_;
    let mut alive: bool_ = 1 as libc::c_int as bool_;
    let mut explode: bool_ = 0 as libc::c_int as bool_;
    /* Not allowed to attack */
    if (*r_ptr).flags1 & 0x10000 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* ...nor if friendly */
    if is_friend(m_ptr) >= 0 as libc::c_int {
        if (*p_ptr).control as libc::c_int == m_idx {
            swap_position((*m_ptr).fy as libc::c_int,
                          (*m_ptr).fx as libc::c_int);
        }
        return 0 as libc::c_int as bool_
    }
    /* Cannot attack the player if mortal and player fated to never die by the ... */
    if (*r_ptr).flags7 & 0x20 as libc::c_int as libc::c_uint != 0 &&
           (*p_ptr).no_mortal as libc::c_int != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Total armor */
    ac = (*p_ptr).ac as libc::c_int + (*p_ptr).to_a as libc::c_int;
    /* Extract the effective monster level */
    rlev =
        if (*m_ptr).level as libc::c_int >= 1 as libc::c_int {
            (*m_ptr).level as libc::c_int
        } else { 1 as libc::c_int };
    /* Get the monster name (or "it") */
    monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
    /* Get the "died from" information (i.e. "a kobold") */
    monster_desc(ddesc.as_mut_ptr(), m_ptr, 0x88 as libc::c_int);
    /* Assume no blink */
    blinked = 0 as libc::c_int as bool_;
    let mut current_block_608: u64;
    /* Scan through all four blows */
    ap_cnt = 0 as libc::c_int;
    while ap_cnt < 4 as libc::c_int {
        let mut visible: bool_ = 0 as libc::c_int as bool_;
        let mut obvious: bool_ = 0 as libc::c_int as bool_;
        let mut power: libc::c_int = 0 as libc::c_int;
        let mut damage: libc::c_int = 0 as libc::c_int;
        let mut act: cptr = 0 as cptr;
        /* Extract the attack infomation */
        let mut effect: libc::c_int =
            (*m_ptr).blow[ap_cnt as usize].effect as libc::c_int;
        let mut method: libc::c_int =
            (*m_ptr).blow[ap_cnt as usize].method as libc::c_int;
        let mut d_dice: libc::c_int =
            (*m_ptr).blow[ap_cnt as usize].d_dice as libc::c_int;
        let mut d_side: libc::c_int =
            (*m_ptr).blow[ap_cnt as usize].d_side as libc::c_int;
        /* Hack -- no more attacks */
        if method == 0 { break ; }
        /* Stop if player is dead or gone */
        if alive == 0 || death as libc::c_int != 0 { break ; }
        /* Handle "leaving" */
        if (*p_ptr).leaving != 0 { break ; }
        /* Extract visibility (before blink) */
        if (*m_ptr).ml != 0 { visible = 1 as libc::c_int as bool_ }
        /* Extract the attack "power" */
        match effect {
            1 => { power = 60 as libc::c_int }
            2 => { power = 5 as libc::c_int }
            3 => { power = 20 as libc::c_int }
            4 => { power = 15 as libc::c_int }
            5 => { power = 5 as libc::c_int }
            6 => { power = 5 as libc::c_int }
            7 => { power = 5 as libc::c_int }
            8 => { power = 5 as libc::c_int }
            9 => { power = 0 as libc::c_int }
            10 => { power = 10 as libc::c_int }
            11 => { power = 10 as libc::c_int }
            12 => { power = 10 as libc::c_int }
            13 => { power = 2 as libc::c_int }
            14 => { power = 10 as libc::c_int }
            15 => { power = 10 as libc::c_int }
            16 => { power = 2 as libc::c_int }
            17 => { power = 0 as libc::c_int }
            20 => { power = 0 as libc::c_int }
            21 => { power = 0 as libc::c_int }
            18 => { power = 0 as libc::c_int }
            19 => { power = 0 as libc::c_int }
            22 => { power = 0 as libc::c_int }
            23 => { power = 2 as libc::c_int }
            24 => { power = 60 as libc::c_int }
            25 => { power = 5 as libc::c_int }
            26 => { power = 5 as libc::c_int }
            27 => { power = 5 as libc::c_int }
            28 => { power = 5 as libc::c_int }
            29 => { power = 5 as libc::c_int }
            30 => { power = 5 as libc::c_int }
            31 => { power = 60 as libc::c_int }
            32 => { power = 10 as libc::c_int }
            33 => { power = 5 as libc::c_int }
            34 => { power = 20 as libc::c_int }
            _ => { }
        }
        /* Monster hits player */
        if effect == 0 || check_hit(power, rlev) != 0 {
            let mut chance: libc::c_int =
                (*p_ptr).dodge_chance as libc::c_int -
                    rlev * 5 as libc::c_int / 6 as libc::c_int;
            /* Always disturbing */
            disturb(1 as libc::c_int, 0 as libc::c_int);
            if chance > 0 as libc::c_int &&
                   Rand_div(100 as libc::c_int) < chance {
                let mut m_poss: [libc::c_char; 80] = [0; 80];
                monster_desc(m_poss.as_mut_ptr(), m_ptr, 0x6 as libc::c_int);
                msg_format(b"You dodge %s attack!\x00" as *const u8 as
                               *const libc::c_char, m_poss.as_mut_ptr());
                current_block_608 = 13797916685926291137;
            } else {
                /* Eru can help you */
                if (*p_ptr).pgod as libc::c_int == 1 as libc::c_int &&
                       (*p_ptr).praying as libc::c_int != 0 {
                    let mut chance_0: s32b = (*p_ptr).grace;
                    if chance_0 > 50000 as libc::c_int {
                        chance_0 = 50000 as libc::c_int
                    }
                    chance_0 -= rlev * 300 as libc::c_int;
                    if (Rand_div(100000 as libc::c_int) + 1 as libc::c_int) <
                           chance_0 &&
                           (*r_ptr).flags3 &
                               0x40 as libc::c_int as libc::c_uint != 0 {
                        /* Remember the Evil-ness */
                        (*r_ptr).r_flags3 |=
                            0x40 as libc::c_int as libc::c_uint;
                        /* Message */
                        msg_format(b"The hand of Eru Iluvatar stops %s blow.\x00"
                                       as *const u8 as *const libc::c_char,
                                   m_name.as_mut_ptr());
                        /* Hack -- Next attack */
                        current_block_608 = 13797916685926291137;
                    } else { current_block_608 = 5706507068631705000; }
                } else { current_block_608 = 5706507068631705000; }
                match current_block_608 {
                    13797916685926291137 => { }
                    _ =>
                    /* Hack -- Apply "protection from evil" */
                    {
                        if (*p_ptr).protevil as libc::c_int > 0 as libc::c_int
                               &&
                               (*r_ptr).flags3 &
                                   0x40 as libc::c_int as libc::c_uint != 0 &&
                               (*p_ptr).lev as libc::c_int >= rlev &&
                               Rand_div(100 as libc::c_int) +
                                   (*p_ptr).lev as libc::c_int >
                                   50 as libc::c_int {
                            /* Remember the Evil-ness */
                            if (*m_ptr).ml != 0 {
                                (*r_ptr).r_flags3 |=
                                    0x40 as libc::c_int as libc::c_uint
                            }
                            /* Message */
                            msg_format(b"%^s is repelled.\x00" as *const u8 as
                                           *const libc::c_char,
                                       m_name.as_mut_ptr());
                            /* Hack -- Next attack */
                            current_block_608 = 13797916685926291137;
                        } else if (*p_ptr).protgood as libc::c_int >
                                      0 as libc::c_int &&
                                      (*r_ptr).flags3 &
                                          0x200 as libc::c_int as libc::c_uint
                                          != 0 &&
                                      (*p_ptr).lev as libc::c_int >= rlev &&
                                      Rand_div(100 as libc::c_int) +
                                          (*p_ptr).lev as libc::c_int >
                                          50 as libc::c_int {
                            /* Hack -- Apply "protection from good" */
                            /* Remember the Good-ness */
                            if (*m_ptr).ml != 0 {
                                (*r_ptr).r_flags3 |=
                                    0x200 as libc::c_int as libc::c_uint
                            }
                            /* Message */
                            msg_format(b"%^s is repelled.\x00" as *const u8 as
                                           *const libc::c_char,
                                       m_name.as_mut_ptr());
                            current_block_608 = 13797916685926291137;
                        } else {
                            /* Assume no cut or stun */
                            do_vampire = 0 as libc::c_int;
                            do_stun = do_vampire;
                            do_cut = do_stun;
                            /* Describe the attack method */
                            match method {
                                1 => {
                                    act =
                                        b"hits you.\x00" as *const u8 as
                                            *const libc::c_char; /* Note! This is "charges", not "charges at". */
                                    do_stun = 1 as libc::c_int;
                                    do_cut = do_stun;
                                    touched = 1 as libc::c_int as bool_;
                                    sound(1 as libc::c_int);
                                }
                                2 => {
                                    act =
                                        b"touches you.\x00" as *const u8 as
                                            *const libc::c_char;
                                    touched = 1 as libc::c_int as bool_;
                                    sound(44 as libc::c_int);
                                }
                                3 => {
                                    act =
                                        b"punches you.\x00" as *const u8 as
                                            *const libc::c_char;
                                    touched = 1 as libc::c_int as bool_;
                                    do_stun = 1 as libc::c_int;
                                    sound(1 as libc::c_int);
                                }
                                4 => {
                                    act =
                                        b"kicks you.\x00" as *const u8 as
                                            *const libc::c_char;
                                    touched = 1 as libc::c_int as bool_;
                                    do_stun = 1 as libc::c_int;
                                    sound(1 as libc::c_int);
                                }
                                5 => {
                                    act =
                                        b"claws you.\x00" as *const u8 as
                                            *const libc::c_char;
                                    touched = 1 as libc::c_int as bool_;
                                    do_cut = 1 as libc::c_int;
                                    sound(36 as libc::c_int);
                                }
                                6 => {
                                    act =
                                        b"bites you.\x00" as *const u8 as
                                            *const libc::c_char;
                                    do_cut = 1 as libc::c_int;
                                    if Rand_div(100 as libc::c_int) <
                                           5 as libc::c_int &&
                                           (!strstr(r_name.offset((*r_ptr).name
                                                                      as
                                                                      isize),
                                                    b"Vampire\x00" as
                                                        *const u8 as
                                                        *const libc::c_char).is_null()
                                                ||
                                                !strstr(r_name.offset((*r_ptr).name
                                                                          as
                                                                          isize),
                                                        b"vampire\x00" as
                                                            *const u8 as
                                                            *const libc::c_char).is_null())
                                       {
                                        do_vampire = 1 as libc::c_int
                                    }
                                    touched = 1 as libc::c_int as bool_;
                                    sound(35 as libc::c_int);
                                }
                                7 => {
                                    act =
                                        b"stings you.\x00" as *const u8 as
                                            *const libc::c_char;
                                    touched = 1 as libc::c_int as bool_;
                                    sound(45 as libc::c_int);
                                }
                                8 => {
                                    act =
                                        b"XXX1\'s you.\x00" as *const u8 as
                                            *const libc::c_char
                                }
                                9 => {
                                    act =
                                        b"butts you.\x00" as *const u8 as
                                            *const libc::c_char;
                                    do_stun = 1 as libc::c_int;
                                    touched = 1 as libc::c_int as bool_;
                                    sound(1 as libc::c_int);
                                }
                                10 => {
                                    act =
                                        b"crushes you.\x00" as *const u8 as
                                            *const libc::c_char;
                                    do_stun = 1 as libc::c_int;
                                    touched = 1 as libc::c_int as bool_;
                                    sound(46 as libc::c_int);
                                }
                                11 => {
                                    act =
                                        b"engulfs you.\x00" as *const u8 as
                                            *const libc::c_char;
                                    touched = 1 as libc::c_int as bool_;
                                    sound(46 as libc::c_int);
                                }
                                12 => {
                                    act =
                                        b"charges you.\x00" as *const u8 as
                                            *const libc::c_char;
                                    touched = 1 as libc::c_int as bool_;
                                    sound(26 as libc::c_int);
                                }
                                13 => {
                                    act =
                                        b"crawls on you.\x00" as *const u8 as
                                            *const libc::c_char;
                                    touched = 1 as libc::c_int as bool_;
                                    sound(47 as libc::c_int);
                                }
                                14 => {
                                    act =
                                        b"drools on you.\x00" as *const u8 as
                                            *const libc::c_char;
                                    sound(47 as libc::c_int);
                                }
                                15 => {
                                    act =
                                        b"spits on you.\x00" as *const u8 as
                                            *const libc::c_char;
                                    sound(47 as libc::c_int);
                                }
                                16 => {
                                    act =
                                        b"explodes.\x00" as *const u8 as
                                            *const libc::c_char;
                                    explode = 1 as libc::c_int as bool_
                                }
                                17 => {
                                    act =
                                        b"gazes at you.\x00" as *const u8 as
                                            *const libc::c_char
                                }
                                18 => {
                                    act =
                                        b"wails at you.\x00" as *const u8 as
                                            *const libc::c_char;
                                    sound(48 as libc::c_int);
                                }
                                19 => {
                                    act =
                                        b"releases spores at you.\x00" as
                                            *const u8 as *const libc::c_char;
                                    sound(47 as libc::c_int);
                                }
                                20 => {
                                    act =
                                        b"projects XXX4\'s at you.\x00" as
                                            *const u8 as *const libc::c_char
                                }
                                21 => {
                                    act =
                                        b"begs you for money.\x00" as
                                            *const u8 as *const libc::c_char;
                                    sound(61 as libc::c_int);
                                }
                                22 => {
                                    act =
                                        desc_insult[Rand_div(8 as libc::c_int)
                                                        as usize];
                                    sound(61 as libc::c_int);
                                }
                                23 => {
                                    if !strstr(r_name.offset((*r_ptr).name as
                                                                 isize),
                                               b"Mathilde, the Science Student\x00"
                                                   as *const u8 as
                                                   *const libc::c_char).is_null()
                                       {
                                        act =
                                            desc_moan[(Rand_div(3 as
                                                                    libc::c_int)
                                                           + 4 as libc::c_int)
                                                          as usize]
                                    } else {
                                        act =
                                            desc_moan[Rand_div(4 as
                                                                   libc::c_int)
                                                          as usize]
                                    }
                                    sound(61 as libc::c_int);
                                }
                                24 => {
                                    if Rand_div(3 as libc::c_int) +
                                           1 as libc::c_int ==
                                           1 as libc::c_int {
                                        act =
                                            b"sings \'We are a happy family.\'\x00"
                                                as *const u8 as
                                                *const libc::c_char
                                    } else {
                                        act =
                                            b"sings \'I love you, you love me.\'\x00"
                                                as *const u8 as
                                                *const libc::c_char
                                    }
                                    sound(62 as libc::c_int);
                                }
                                _ => { }
                            }
                            /* Message */
                            if !act.is_null() {
                                msg_format(b"%^s %s\x00" as *const u8 as
                                               *const libc::c_char,
                                           m_name.as_mut_ptr(), act);
                            }
                            /* The undead can give the player the Black Breath with
			 * a successful blow. Uniques have a better chance. -LM-
			 * Nazgul have a 25% chance
			 */
                            if (*r_ptr).flags7 &
                                   0x80 as libc::c_int as libc::c_uint != 0 {
                                black_breath_attack(4 as libc::c_int);
                            } else if (*m_ptr).level as libc::c_int >=
                                          35 as libc::c_int &&
                                          (*r_ptr).flags3 &
                                              0x20 as libc::c_int as
                                                  libc::c_uint != 0 &&
                                          (*r_ptr).flags1 &
                                              0x1 as libc::c_int as
                                                  libc::c_uint != 0 {
                                black_breath_attack(300 as libc::c_int -
                                                        (*m_ptr).level as
                                                            libc::c_int);
                            } else if (*m_ptr).level as libc::c_int >=
                                          40 as libc::c_int &&
                                          (*r_ptr).flags3 &
                                              0x20 as libc::c_int as
                                                  libc::c_uint != 0 {
                                black_breath_attack(450 as libc::c_int -
                                                        (*m_ptr).level as
                                                            libc::c_int);
                            }
                            /* Hack -- assume all attacks are obvious */
                            obvious = 1 as libc::c_int as bool_;
                            /* Roll out the damage */
                            damage = damroll(d_dice as s16b, d_side as s16b);
                            /* Sometime reduce the damage */
                            damage /= divis as libc::c_int;
                            /* Apply appropriate damage */
                            match effect {
                                0 => {
                                    /* Hack -- Assume obvious */
                                    obvious = 1 as libc::c_int as bool_;
                                    /* Hack -- No damage */
                                    damage = 0 as libc::c_int
                                }
                                1 => {
                                    /* Obvious */
                                    obvious = 1 as libc::c_int as bool_;
                                    /* Hack -- Player armor reduces total damage */
                                    damage -=
                                        damage *
                                            (if ac < 150 as libc::c_int {
                                                 ac
                                             } else { 150 as libc::c_int }) /
                                            250 as libc::c_int;
                                    /* Take damage */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                }
                                34 => {
                                    /* Obvious */
                                    obvious = 1 as libc::c_int as bool_;
                                    /* Morph, but let mimicry skill have a chance to stop this */
                                    if Rand_div(100 as libc::c_int) <
                                           60 as libc::c_int -
                                               get_skill(32 as libc::c_int) as
                                                   libc::c_int {
                                        /* Message */
                                        cmsg_print(10 as libc::c_int as
                                                       byte_hack,
                                                   b"You feel the dark powers twisting your body!\x00"
                                                       as *const u8 as
                                                       *const libc::c_char);
                                        set_mimic(damage,
                                                  resolve_mimic_name(b"Abomination\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char),
                                                  50 as libc::c_int);
                                    } else {
                                        /* Message */
                                        cmsg_print(10 as libc::c_int as
                                                       byte_hack,
                                                   b"You feel the dark powers trying to twisting your body, but they fail.\x00"
                                                       as *const u8 as
                                                       *const libc::c_char);
                                    }
                                }
                                31 => {
                                    obvious = 1 as libc::c_int as bool_;
                                    take_sanity_hit(damage,
                                                    ddesc.as_mut_ptr() as
                                                        cptr);
                                }
                                2 => {
                                    /* Take some damage */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Take "poison" effect */
                                    if !((*p_ptr).resist_pois as libc::c_int
                                             != 0 ||
                                             (*p_ptr).oppose_pois as
                                                 libc::c_int != 0) {
                                        if set_poisoned((*p_ptr).poisoned as
                                                            libc::c_int +
                                                            (Rand_div(rlev) +
                                                                 1 as
                                                                     libc::c_int)
                                                            +
                                                            5 as libc::c_int)
                                               != 0 {
                                            obvious =
                                                1 as libc::c_int as bool_
                                        }
                                    }
                                    /* Learn about the player */
                                    update_smart_learn(m_idx,
                                                       5 as libc::c_int);
                                }
                                3 => {
                                    /* Take some damage */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Allow complete resist */
                                    if (*p_ptr).resist_disen == 0 {
                                        /* Apply disenchantment */
                                        if apply_disenchant(0 as libc::c_int)
                                               != 0 {
                                            obvious =
                                                1 as libc::c_int as bool_
                                        }
                                    }
                                    /* Learn about the player */
                                    update_smart_learn(m_idx,
                                                       12 as libc::c_int);
                                }
                                4 => {
                                    /* Take some damage */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Find an item */
                                    k = 0 as libc::c_int;
                                    while k < 10 as libc::c_int {
                                        /* Pick an item */
                                        i = Rand_div(23 as libc::c_int);
                                        /* Obtain the item */
                                        o_ptr =
                                            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i
                                                                                             as
                                                                                             isize)
                                                as *mut object_type;
                                        /* Skip non-objects */
                                        if !((*o_ptr).k_idx == 0) {
                                            /* Drain charged wands/staffs
						   Hack -- don't let artifacts get drained */
                                            if ((*o_ptr).tval as libc::c_int
                                                    == 55 as libc::c_int ||
                                                    (*o_ptr).tval as
                                                        libc::c_int ==
                                                        65 as libc::c_int) &&
                                                   (*o_ptr).pval != 0 &&
                                                   !((*o_ptr).tval as
                                                         libc::c_int ==
                                                         102 as libc::c_int ||
                                                         (if (*o_ptr).name1 as
                                                                 libc::c_int
                                                                 != 0 {
                                                              1 as libc::c_int
                                                          } else {
                                                              0 as libc::c_int
                                                          }) != 0 ||
                                                         (if (*o_ptr).art_name
                                                                 as
                                                                 libc::c_int
                                                                 != 0 {
                                                              1 as libc::c_int
                                                          } else {
                                                              0 as libc::c_int
                                                          }) != 0 ||
                                                         (if (*k_info.offset((*o_ptr).k_idx
                                                                                 as
                                                                                 isize)).flags3
                                                                 as
                                                                 libc::c_long
                                                                 &
                                                                 0x8000 as
                                                                     libc::c_long
                                                                 != 0 {
                                                              1 as libc::c_int
                                                          } else {
                                                              0 as libc::c_int
                                                          }) != 0) {
                                                /* Message */
                                                msg_print(b"Energy drains from your pack!\x00"
                                                              as *const u8 as
                                                              *const libc::c_char);
                                                /* Obvious */
                                                obvious =
                                                    1 as libc::c_int as bool_;
                                                /* Heal */
                                                j = rlev;
                                                (*m_ptr).hp +=
                                                    j * (*o_ptr).pval *
                                                        (*o_ptr).number as
                                                            libc::c_int;
                                                if (*m_ptr).hp >
                                                       (*m_ptr).maxhp {
                                                    (*m_ptr).hp =
                                                        (*m_ptr).maxhp
                                                }
                                                /* Redraw (later) if needed */
                                                if health_who as libc::c_int
                                                       == m_idx {
                                                    (*p_ptr).redraw =
                                                        ((*p_ptr).redraw as
                                                             libc::c_long |
                                                             0x800 as
                                                                 libc::c_long)
                                                            as u32b
                                                }
                                                /* Uncharge */
                                                (*o_ptr).pval =
                                                    0 as libc::c_int;
                                                /* Combine / Reorder the pack */
                                                (*p_ptr).notice =
                                                    ((*p_ptr).notice as
                                                         libc::c_long |
                                                         (0x1 as libc::c_long
                                                              |
                                                              0x2 as
                                                                  libc::c_long))
                                                        as u32b;
                                                /* Window stuff */
                                                (*p_ptr).window =
                                                    ((*p_ptr).window as
                                                         libc::c_long |
                                                         0x1 as libc::c_long)
                                                        as u32b;
                                                break ;
                                            }
                                        }
                                        k += 1
                                    }
                                }
                                5 => {
                                    /* Take some damage */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Obvious */
                                    obvious = 1 as libc::c_int as bool_;
                                    /* Saving throw (unless paralyzed) based on dex and level */
                                    if (*p_ptr).paralyzed == 0 &&
                                           Rand_div(100 as libc::c_int) <
                                               *adj_dex_safe.as_mut_ptr().offset((*p_ptr).stat_ind[3
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       usize]
                                                                                     as
                                                                                     isize)
                                                   as libc::c_int +
                                                   (*p_ptr).lev as libc::c_int
                                       {
                                        /* Saving throw message */
                                        msg_print(b"You quickly protect your money pouch!\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                        /* Occasional blink anyway */
                                        if Rand_div(3 as libc::c_int) != 0 {
                                            blinked =
                                                1 as libc::c_int as bool_
                                        }
                                    } else {
                                        /* Eat gold */
                                        gold =
                                            (*p_ptr).au / 10 as libc::c_int +
                                                (Rand_div(25 as libc::c_int) +
                                                     1 as libc::c_int);
                                        if gold < 2 as libc::c_int {
                                            gold = 2 as libc::c_int
                                        }
                                        if gold > 5000 as libc::c_int {
                                            gold =
                                                (*p_ptr).au /
                                                    20 as libc::c_int +
                                                    (Rand_div(3000 as
                                                                  libc::c_int)
                                                         + 1 as libc::c_int)
                                        }
                                        if gold > (*p_ptr).au {
                                            gold = (*p_ptr).au
                                        }
                                        (*p_ptr).au -= gold;
                                        if gold <= 0 as libc::c_int {
                                            msg_print(b"Nothing was stolen.\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                        } else if (*p_ptr).au != 0 {
                                            msg_print(b"Your purse feels lighter.\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                            msg_format(b"%ld coins were stolen!\x00"
                                                           as *const u8 as
                                                           *const libc::c_char,
                                                       gold as libc::c_long);
                                        } else {
                                            msg_print(b"Your purse feels lighter.\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                            msg_print(b"All of your coins were stolen!\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                        }
                                        while gold > 0 as libc::c_int {
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
                                            let mut j_ptr: *mut object_type =
                                                &mut forge;
                                            /* Wipe the object */
                                            object_wipe(j_ptr);
                                            /* Prepare a gold object */
                                            object_prep(j_ptr,
                                                        lookup_kind(100 as
                                                                        libc::c_int,
                                                                    9 as
                                                                        libc::c_int)
                                                            as libc::c_int);
                                            /* Determine how much the treasure is "worth" */
                                            (*j_ptr).pval =
                                                if gold >=
                                                       15000 as libc::c_int {
                                                    15000 as libc::c_int
                                                } else { gold };
                                            monster_carry(m_ptr, m_idx,
                                                          j_ptr);
                                            gold -= 15000 as libc::c_int
                                        }
                                        /* Redraw gold */
                                        (*p_ptr).redraw =
                                            ((*p_ptr).redraw as libc::c_long |
                                                 0x100 as libc::c_long) as
                                                u32b;
                                        /* Window stuff */
                                        (*p_ptr).window =
                                            ((*p_ptr).window as libc::c_long |
                                                 0x8 as libc::c_long) as u32b;
                                        /* Blink away */
                                        blinked = 1 as libc::c_int as bool_
                                    }
                                }
                                6 => {
                                    /* Take some damage */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Saving throw (unless paralyzed) based on dex and level */
                                    if (*p_ptr).paralyzed == 0 &&
                                           Rand_div(100 as libc::c_int) <
                                               *adj_dex_safe.as_mut_ptr().offset((*p_ptr).stat_ind[3
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       usize]
                                                                                     as
                                                                                     isize)
                                                   as libc::c_int +
                                                   (*p_ptr).lev as libc::c_int
                                       {
                                        /* Saving throw message */
                                        msg_print(b"You grab hold of your backpack!\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                        /* Occasional "blink" anyway */
                                        blinked = 1 as libc::c_int as bool_;
                                        /* Obvious */
                                        obvious = 1 as libc::c_int as bool_
                                    } else {
                                        /* Find an item */
                                        k = 0 as libc::c_int;
                                        while k < 10 as libc::c_int {
                                            /* Pick an item */
                                            i = Rand_div(23 as libc::c_int);
                                            /* Obtain the item */
                                            o_ptr =
                                                &mut *(*p_ptr).inventory.as_mut_ptr().offset(i
                                                                                                 as
                                                                                                 isize)
                                                    as *mut object_type;
                                            /* Skip non-objects */
                                            if !((*o_ptr).k_idx == 0) {
                                                /* Skip artifacts */
                                                if !((*o_ptr).tval as
                                                         libc::c_int ==
                                                         102 as libc::c_int ||
                                                         (if (*o_ptr).name1 as
                                                                 libc::c_int
                                                                 != 0 {
                                                              1 as libc::c_int
                                                          } else {
                                                              0 as libc::c_int
                                                          }) != 0 ||
                                                         (if (*o_ptr).art_name
                                                                 as
                                                                 libc::c_int
                                                                 != 0 {
                                                              1 as libc::c_int
                                                          } else {
                                                              0 as libc::c_int
                                                          }) != 0 ||
                                                         (if (*k_info.offset((*o_ptr).k_idx
                                                                                 as
                                                                                 isize)).flags3
                                                                 as
                                                                 libc::c_long
                                                                 &
                                                                 0x8000 as
                                                                     libc::c_long
                                                                 != 0 {
                                                              1 as libc::c_int
                                                          } else {
                                                              0 as libc::c_int
                                                          }) != 0 ||
                                                         (*o_ptr).art_name as
                                                             libc::c_int != 0)
                                                   {
                                                    /* Get a description */
                                                    object_desc(o_name.as_mut_ptr(),
                                                                o_ptr,
                                                                0 as
                                                                    libc::c_int,
                                                                3 as
                                                                    libc::c_int);
                                                    /* Message */
                                                    msg_format(b"%sour %s (%c) was stolen!\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               if (*o_ptr).number
                                                                      as
                                                                      libc::c_int
                                                                      >
                                                                      1 as
                                                                          libc::c_int
                                                                  {
                                                                   b"One of y\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char
                                                               } else {
                                                                   b"Y\x00" as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char
                                                               },
                                                               o_name.as_mut_ptr(),
                                                               index_to_label(i)
                                                                   as
                                                                   libc::c_int);
                                                    /* Option */
                                                    if testing_carry != 0 {
                                                        let mut o_idx: s16b =
                                                            0;
                                                        /* Make an object */
                                                        o_idx = o_pop();
                                                        /* Success */
                                                        if o_idx != 0 {
                                                            let mut j_ptr_0:
                                                                    *mut object_type =
                                                                0 as
                                                                    *mut object_type;
                                                            /* Get new object */
                                                            j_ptr_0 =
                                                                &mut *o_list.offset(o_idx
                                                                                        as
                                                                                        isize)
                                                                    as
                                                                    *mut object_type;
                                                            /* Copy object */
                                                            object_copy(j_ptr_0,
                                                                        o_ptr);
                                                            /* Modify number */
                                                            (*j_ptr_0).number
                                                                =
                                                                1 as
                                                                    libc::c_int
                                                                    as
                                                                    byte_hack;
                                                            /* Hack -- If a wand, allocate total
								 * maximum timeouts or charges between those
								 * stolen and those missed. -LM-
								 */
                                                            if (*o_ptr).tval
                                                                   as
                                                                   libc::c_int
                                                                   ==
                                                                   65 as
                                                                       libc::c_int
                                                               {
                                                                (*j_ptr_0).pval
                                                                    =
                                                                    (*o_ptr).pval
                                                                        /
                                                                        (*o_ptr).number
                                                                            as
                                                                            libc::c_int;
                                                                (*o_ptr).pval
                                                                    -=
                                                                    (*j_ptr_0).pval
                                                            }
                                                            /* Forget mark */
                                                            (*j_ptr_0).marked
                                                                =
                                                                0 as
                                                                    libc::c_int
                                                                    as
                                                                    byte_hack;
                                                            /* Memorize monster */
                                                            (*j_ptr_0).held_m_idx
                                                                =
                                                                m_idx as s16b;
                                                            /* Build stack */
                                                            (*j_ptr_0).next_o_idx
                                                                =
                                                                (*m_ptr).hold_o_idx;
                                                            /* Build stack */
                                                            (*m_ptr).hold_o_idx
                                                                = o_idx
                                                        }
                                                    } else if !strstr(r_name.offset((*r_ptr).name
                                                                                        as
                                                                                        isize),
                                                                      b"black market\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char).is_null()
                                                                  &&
                                                                  Rand_div(2
                                                                               as
                                                                               libc::c_int)
                                                                      +
                                                                      1 as
                                                                          libc::c_int
                                                                      !=
                                                                      1 as
                                                                          libc::c_int
                                                     {
                                                        let mut o_idx_0:
                                                                s16b = 0;
                                                        /* Make an object */
                                                        o_idx_0 = o_pop();
                                                        /* Success */
                                                        if o_idx_0 != 0 {
                                                            let mut j_ptr_1:
                                                                    *mut object_type =
                                                                0 as
                                                                    *mut object_type;
                                                            if cheat_xtra as
                                                                   libc::c_int
                                                                   != 0 ||
                                                                   cheat_peek
                                                                       as
                                                                       libc::c_int
                                                                       != 0 {
                                                                msg_print(b"Moving object to black market...\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char);
                                                            }
                                                            /* Get new object */
                                                            j_ptr_1 =
                                                                &mut *o_list.offset(o_idx_0
                                                                                        as
                                                                                        isize)
                                                                    as
                                                                    *mut object_type;
                                                            /* Copy object */
                                                            object_copy(j_ptr_1,
                                                                        o_ptr);
                                                            /* Modify number */
                                                            (*j_ptr_1).number
                                                                =
                                                                1 as
                                                                    libc::c_int
                                                                    as
                                                                    byte_hack;
                                                            /* Forget mark */
                                                            (*j_ptr_1).marked
                                                                =
                                                                0 as
                                                                    libc::c_int
                                                                    as
                                                                    byte_hack;
                                                            move_to_black_market(j_ptr_1);
                                                        }
                                                    }
                                                    /* Steal the items */
                                                    inc_stack_size_ex(i,
                                                                      -(1 as
                                                                            libc::c_int),
                                                                      OPTIMIZE,
                                                                      NO_DESCRIBE);
                                                    /* Obvious */
                                                    obvious =
                                                        1 as libc::c_int as
                                                            bool_;
                                                    /* Blink away */
                                                    blinked =
                                                        1 as libc::c_int as
                                                            bool_;
                                                    break ;
                                                }
                                            }
                                            k += 1
                                        }
                                    }
                                }
                                7 => {
                                    /* Take some damage */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Steal some food */
                                    k = 0 as libc::c_int;
                                    while k < 10 as libc::c_int {
                                        /* Pick an item from the pack */
                                        i = Rand_div(23 as libc::c_int);
                                        /* Get the item */
                                        o_ptr =
                                            &mut *(*p_ptr).inventory.as_mut_ptr().offset(i
                                                                                             as
                                                                                             isize)
                                                as *mut object_type;
                                        /* Skip non-objects */
                                        if !((*o_ptr).k_idx == 0) {
                                            /* Skip non-food objects */
                                            if !((*o_ptr).tval as libc::c_int
                                                     != 80 as libc::c_int) {
                                                /* Get a description */
                                                object_desc(o_name.as_mut_ptr(),
                                                            o_ptr,
                                                            0 as libc::c_int,
                                                            0 as libc::c_int);
                                                /* Message */
                                                msg_format(b"%sour %s (%c) was eaten!\x00"
                                                               as *const u8 as
                                                               *const libc::c_char,
                                                           if (*o_ptr).number
                                                                  as
                                                                  libc::c_int
                                                                  >
                                                                  1 as
                                                                      libc::c_int
                                                              {
                                                               b"One of y\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                           } else {
                                                               b"Y\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                           },
                                                           o_name.as_mut_ptr(),
                                                           index_to_label(i)
                                                               as
                                                               libc::c_int);
                                                /* Steal the items */
                                                inc_stack_size_ex(i,
                                                                  -(1 as
                                                                        libc::c_int),
                                                                  OPTIMIZE,
                                                                  NO_DESCRIBE);
                                                /* Obvious */
                                                obvious =
                                                    1 as libc::c_int as bool_;
                                                break ;
                                            }
                                        }
                                        k += 1
                                    }
                                }
                                8 => {
                                    /* Take some damage */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Access the lite */
                                    o_ptr =
                                        &mut *(*p_ptr).inventory.as_mut_ptr().offset(36
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         isize)
                                            as *mut object_type;
                                    /* Drain fuel */
                                    if (*o_ptr).pval > 0 as libc::c_int &&
                                           !((*o_ptr).tval as libc::c_int ==
                                                 102 as libc::c_int ||
                                                 (if (*o_ptr).name1 as
                                                         libc::c_int != 0 {
                                                      1 as libc::c_int
                                                  } else { 0 as libc::c_int })
                                                     != 0 ||
                                                 (if (*o_ptr).art_name as
                                                         libc::c_int != 0 {
                                                      1 as libc::c_int
                                                  } else { 0 as libc::c_int })
                                                     != 0 ||
                                                 (if (*k_info.offset((*o_ptr).k_idx
                                                                         as
                                                                         isize)).flags3
                                                         as libc::c_long &
                                                         0x8000 as
                                                             libc::c_long != 0
                                                     {
                                                      1 as libc::c_int
                                                  } else { 0 as libc::c_int })
                                                     != 0) {
                                        /* Reduce fuel */
                                        (*o_ptr).pval -=
                                            250 as libc::c_int +
                                                (Rand_div(250 as libc::c_int)
                                                     + 1 as libc::c_int);
                                        if (*o_ptr).pval < 1 as libc::c_int {
                                            (*o_ptr).pval = 1 as libc::c_int
                                        }
                                        /* Notice */
                                        if (*p_ptr).blind == 0 {
                                            msg_print(b"Your light dims.\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                            obvious =
                                                1 as libc::c_int as bool_
                                        }
                                        /* Window stuff */
                                        (*p_ptr).window =
                                            ((*p_ptr).window as libc::c_long |
                                                 0x2 as libc::c_long) as u32b
                                    }
                                }
                                9 => {
                                    /* Obvious */
                                    obvious = 1 as libc::c_int as bool_;
                                    /* Message */
                                    msg_print(b"You are covered in acid!\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                                    /* Special damage */
                                    acid_dam(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Learn about the player */
                                    update_smart_learn(m_idx,
                                                       1 as libc::c_int);
                                }
                                10 => {
                                    /* Obvious */
                                    obvious = 1 as libc::c_int as bool_;
                                    /* Message */
                                    msg_print(b"You are struck by electricity!\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                                    /* Special damage */
                                    elec_dam(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Learn about the player */
                                    update_smart_learn(m_idx,
                                                       2 as libc::c_int);
                                }
                                11 => {
                                    /* Obvious */
                                    obvious = 1 as libc::c_int as bool_;
                                    /* Message */
                                    msg_print(b"You are enveloped in flames!\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                                    /* Special damage */
                                    fire_dam(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Learn about the player */
                                    update_smart_learn(m_idx,
                                                       3 as libc::c_int);
                                }
                                12 => {
                                    /* Obvious */
                                    obvious = 1 as libc::c_int as bool_;
                                    /* Message */
                                    msg_print(b"You are covered with frost!\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                                    /* Special damage */
                                    cold_dam(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Learn about the player */
                                    update_smart_learn(m_idx,
                                                       4 as libc::c_int);
                                }
                                13 => {
                                    /* Take damage */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Increase "blind" */
                                    if (*p_ptr).resist_blind == 0 {
                                        if set_blind((*p_ptr).blind as
                                                         libc::c_int +
                                                         10 as libc::c_int +
                                                         (Rand_div(rlev) +
                                                              1 as
                                                                  libc::c_int))
                                               != 0 {
                                            obvious =
                                                1 as libc::c_int as bool_
                                        }
                                    }
                                    /* Learn about the player */
                                    update_smart_learn(m_idx,
                                                       13 as libc::c_int);
                                }
                                14 => {
                                    /* Take damage */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Increase "confused" */
                                    if (*p_ptr).resist_conf == 0 {
                                        if set_confused((*p_ptr).confused as
                                                            libc::c_int +
                                                            3 as libc::c_int +
                                                            (Rand_div(rlev) +
                                                                 1 as
                                                                     libc::c_int))
                                               != 0 {
                                            obvious =
                                                1 as libc::c_int as bool_
                                        }
                                    }
                                    /* Learn about the player */
                                    update_smart_learn(m_idx,
                                                       10 as libc::c_int);
                                }
                                15 => {
                                    /* Take damage */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Increase "afraid" */
                                    if (*p_ptr).resist_fear != 0 {
                                        msg_print(b"You stand your ground!\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                        obvious = 1 as libc::c_int as bool_
                                    } else if Rand_div(100 as libc::c_int) <
                                                  (*p_ptr).skill_sav as
                                                      libc::c_int {
                                        msg_print(b"You stand your ground!\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                        obvious = 1 as libc::c_int as bool_
                                    } else if set_afraid((*p_ptr).afraid as
                                                             libc::c_int +
                                                             3 as libc::c_int
                                                             +
                                                             (Rand_div(rlev) +
                                                                  1 as
                                                                      libc::c_int))
                                                  != 0 {
                                        obvious = 1 as libc::c_int as bool_
                                    }
                                    /* Learn about the player */
                                    update_smart_learn(m_idx,
                                                       9 as libc::c_int);
                                }
                                16 => {
                                    /* Hack -- Prevent perma-paralysis via damage */
                                    if (*p_ptr).paralyzed as libc::c_int != 0
                                           && damage < 1 as libc::c_int {
                                        damage = 1 as libc::c_int
                                    }
                                    /* Take damage */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Increase "paralyzed" */
                                    if (*p_ptr).free_act != 0 {
                                        msg_print(b"You are unaffected!\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                        obvious = 1 as libc::c_int as bool_
                                    } else if Rand_div(100 as libc::c_int) <
                                                  (*p_ptr).skill_sav as
                                                      libc::c_int {
                                        msg_print(b"You resist the effects!\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                        obvious = 1 as libc::c_int as bool_
                                    } else if set_paralyzed((*p_ptr).paralyzed
                                                                as libc::c_int
                                                                +
                                                                3 as
                                                                    libc::c_int
                                                                +
                                                                (Rand_div(rlev)
                                                                     +
                                                                     1 as
                                                                         libc::c_int))
                                                  != 0 {
                                        obvious = 1 as libc::c_int as bool_
                                    }
                                    /* Learn about the player */
                                    update_smart_learn(m_idx,
                                                       30 as libc::c_int);
                                }
                                17 => {
                                    /* Damage (physical) */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Damage (stat) */
                                    if do_dec_stat(0 as libc::c_int,
                                                   2 as libc::c_int) != 0 {
                                        obvious = 1 as libc::c_int as bool_
                                    }
                                }
                                18 => {
                                    /* Damage (physical) */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Damage (stat) */
                                    if do_dec_stat(1 as libc::c_int,
                                                   2 as libc::c_int) != 0 {
                                        obvious = 1 as libc::c_int as bool_
                                    }
                                }
                                19 => {
                                    /* Damage (physical) */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Damage (stat) */
                                    if do_dec_stat(2 as libc::c_int,
                                                   2 as libc::c_int) != 0 {
                                        obvious = 1 as libc::c_int as bool_
                                    }
                                }
                                20 => {
                                    /* Damage (physical) */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Damage (stat) */
                                    if do_dec_stat(3 as libc::c_int,
                                                   2 as libc::c_int) != 0 {
                                        obvious = 1 as libc::c_int as bool_
                                    }
                                }
                                21 => {
                                    /* Damage (physical) */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Damage (stat) */
                                    if do_dec_stat(4 as libc::c_int,
                                                   2 as libc::c_int) != 0 {
                                        obvious = 1 as libc::c_int as bool_
                                    }
                                }
                                22 => {
                                    /* Damage (physical) */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Damage (stat) */
                                    if do_dec_stat(5 as libc::c_int,
                                                   2 as libc::c_int) != 0 {
                                        obvious = 1 as libc::c_int as bool_
                                    }
                                }
                                23 => {
                                    /* Damage (physical) */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Damage (stats) */
                                    if do_dec_stat(0 as libc::c_int,
                                                   2 as libc::c_int) != 0 {
                                        obvious = 1 as libc::c_int as bool_
                                    }
                                    if do_dec_stat(3 as libc::c_int,
                                                   2 as libc::c_int) != 0 {
                                        obvious = 1 as libc::c_int as bool_
                                    }
                                    if do_dec_stat(4 as libc::c_int,
                                                   2 as libc::c_int) != 0 {
                                        obvious = 1 as libc::c_int as bool_
                                    }
                                    if do_dec_stat(1 as libc::c_int,
                                                   2 as libc::c_int) != 0 {
                                        obvious = 1 as libc::c_int as bool_
                                    }
                                    if do_dec_stat(2 as libc::c_int,
                                                   2 as libc::c_int) != 0 {
                                        obvious = 1 as libc::c_int as bool_
                                    }
                                    if do_dec_stat(5 as libc::c_int,
                                                   2 as libc::c_int) != 0 {
                                        obvious = 1 as libc::c_int as bool_
                                    }
                                }
                                24 => {
                                    /* Obvious */
                                    obvious = 1 as libc::c_int as bool_;
                                    /* Hack -- Reduce damage based on the player armor class */
                                    damage -=
                                        damage *
                                            (if ac < 150 as libc::c_int {
                                                 ac
                                             } else { 150 as libc::c_int }) /
                                            250 as libc::c_int;
                                    /* Take damage */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Radius 8 earthquake centered at the monster */
                                    if damage > 23 as libc::c_int {
                                        /* Prevent destruction of quest levels and town */
                                        if is_quest(dun_level as libc::c_int)
                                               == 0 &&
                                               dun_level as libc::c_int != 0 {
                                            earthquake((*m_ptr).fy as
                                                           libc::c_int,
                                                       (*m_ptr).fx as
                                                           libc::c_int,
                                                       8 as libc::c_int);
                                        }
                                    }
                                }
                                25 => {
                                    /* Obvious */
                                    obvious = 1 as libc::c_int as bool_;
                                    /* Take damage */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    if (*p_ptr).hold_life as libc::c_int != 0
                                           &&
                                           Rand_div(100 as libc::c_int) <
                                               95 as libc::c_int {
                                        msg_print(b"You keep hold of your life force!\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                    } else {
                                        let mut d: s32b =
                                            damroll(10 as libc::c_int as s16b,
                                                    6 as libc::c_int as s16b)
                                                +
                                                (*p_ptr).exp /
                                                    100 as libc::c_int *
                                                    2 as libc::c_int;
                                        if (*p_ptr).hold_life != 0 {
                                            msg_print(b"You feel your life slipping away!\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                            lose_exp(d / 10 as libc::c_int);
                                        } else {
                                            msg_print(b"You feel your life draining away!\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                            lose_exp(d);
                                        }
                                    }
                                }
                                26 => {
                                    /* Obvious */
                                    obvious = 1 as libc::c_int as bool_;
                                    /* Take damage */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    if (*p_ptr).hold_life as libc::c_int != 0
                                           &&
                                           Rand_div(100 as libc::c_int) <
                                               90 as libc::c_int {
                                        msg_print(b"You keep hold of your life force!\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                    } else {
                                        let mut d_0: s32b =
                                            damroll(20 as libc::c_int as s16b,
                                                    6 as libc::c_int as s16b)
                                                +
                                                (*p_ptr).exp /
                                                    100 as libc::c_int *
                                                    2 as libc::c_int;
                                        if (*p_ptr).hold_life != 0 {
                                            msg_print(b"You feel your life slipping away!\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                            lose_exp(d_0 / 10 as libc::c_int);
                                        } else {
                                            msg_print(b"You feel your life draining away!\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                            lose_exp(d_0);
                                        }
                                    }
                                }
                                27 => {
                                    /* Obvious */
                                    obvious = 1 as libc::c_int as bool_;
                                    /* Take damage */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    if (*p_ptr).hold_life as libc::c_int != 0
                                           &&
                                           Rand_div(100 as libc::c_int) <
                                               75 as libc::c_int {
                                        msg_print(b"You keep hold of your life force!\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                    } else {
                                        let mut d_1: s32b =
                                            damroll(40 as libc::c_int as s16b,
                                                    6 as libc::c_int as s16b)
                                                +
                                                (*p_ptr).exp /
                                                    100 as libc::c_int *
                                                    2 as libc::c_int;
                                        if (*p_ptr).hold_life != 0 {
                                            msg_print(b"You feel your life slipping away!\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                            lose_exp(d_1 / 10 as libc::c_int);
                                        } else {
                                            msg_print(b"You feel your life draining away!\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                            lose_exp(d_1);
                                        }
                                    }
                                }
                                28 => {
                                    /* Obvious */
                                    obvious = 1 as libc::c_int as bool_;
                                    /* Take damage */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    if (*p_ptr).hold_life as libc::c_int != 0
                                           &&
                                           Rand_div(100 as libc::c_int) <
                                               50 as libc::c_int {
                                        msg_print(b"You keep hold of your life force!\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                    } else {
                                        let mut d_2: s32b =
                                            damroll(80 as libc::c_int as s16b,
                                                    6 as libc::c_int as s16b)
                                                +
                                                (*p_ptr).exp /
                                                    100 as libc::c_int *
                                                    2 as libc::c_int;
                                        if (*p_ptr).hold_life != 0 {
                                            msg_print(b"You feel your life slipping away!\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                            lose_exp(d_2 / 10 as libc::c_int);
                                        } else {
                                            msg_print(b"You feel your life draining away!\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                            lose_exp(d_2);
                                        }
                                    }
                                }
                                29 => {
                                    /* Take some damage */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Take "poison" effect */
                                    if !((*p_ptr).resist_pois as libc::c_int
                                             != 0 ||
                                             (*p_ptr).oppose_pois as
                                                 libc::c_int != 0) {
                                        if set_poisoned((*p_ptr).poisoned as
                                                            libc::c_int +
                                                            (Rand_div(rlev) +
                                                                 1 as
                                                                     libc::c_int)
                                                            +
                                                            5 as libc::c_int)
                                               != 0 {
                                            obvious =
                                                1 as libc::c_int as bool_
                                        }
                                    }
                                    /* Damage CON (10% chance)*/
                                    if (Rand_div(100 as libc::c_int) +
                                            1 as libc::c_int) <
                                           11 as libc::c_int {
                                        /* 1% chance for perm. damage */
                                        let mut perm: bool_ =
                                            (Rand_div(10 as libc::c_int) +
                                                 1 as libc::c_int ==
                                                 1 as libc::c_int) as
                                                libc::c_int as bool_;
                                        if dec_stat(4 as libc::c_int,
                                                    Rand_div(10 as
                                                                 libc::c_int)
                                                        + 1 as libc::c_int,
                                                    perm as libc::c_int) != 0
                                           {
                                            obvious =
                                                1 as libc::c_int as bool_
                                        }
                                    }
                                }
                                32 => {
                                    /* Take damage */
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                    /* Increase "image" */
                                    if (*p_ptr).resist_chaos == 0 {
                                        if set_image((*p_ptr).image as
                                                         libc::c_int +
                                                         3 as libc::c_int +
                                                         (Rand_div(rlev /
                                                                       2 as
                                                                           libc::c_int)
                                                              +
                                                              1 as
                                                                  libc::c_int))
                                               != 0 {
                                            obvious =
                                                1 as libc::c_int as bool_
                                        }
                                    }
                                    /* Learn about the player */
                                    update_smart_learn(m_idx,
                                                       11 as libc::c_int);
                                }
                                30 => {
                                    match Rand_div(10 as libc::c_int) +
                                              1 as libc::c_int {
                                        1 | 2 | 3 | 4 | 5 => {
                                            msg_print(b"You feel life has clocked back.\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                            lose_exp(100 as libc::c_int +
                                                         (*p_ptr).exp /
                                                             100 as
                                                                 libc::c_int *
                                                             2 as
                                                                 libc::c_int);
                                        }
                                        6 | 7 | 8 | 9 => {
                                            let mut stat: libc::c_int =
                                                Rand_div(6 as libc::c_int);
                                            match stat {
                                                0 => {
                                                    act =
                                                        b"strong\x00" as
                                                            *const u8 as
                                                            *const libc::c_char
                                                }
                                                1 => {
                                                    act =
                                                        b"bright\x00" as
                                                            *const u8 as
                                                            *const libc::c_char
                                                }
                                                2 => {
                                                    act =
                                                        b"wise\x00" as
                                                            *const u8 as
                                                            *const libc::c_char
                                                }
                                                3 => {
                                                    act =
                                                        b"agile\x00" as
                                                            *const u8 as
                                                            *const libc::c_char
                                                }
                                                4 => {
                                                    act =
                                                        b"hardy\x00" as
                                                            *const u8 as
                                                            *const libc::c_char
                                                }
                                                5 => {
                                                    act =
                                                        b"beautiful\x00" as
                                                            *const u8 as
                                                            *const libc::c_char
                                                }
                                                _ => { }
                                            }
                                            msg_format(b"You\'re not as %s as you used to be...\x00"
                                                           as *const u8 as
                                                           *const libc::c_char,
                                                       act);
                                            (*p_ptr).stat_cur[stat as usize] =
                                                ((*p_ptr).stat_cur[stat as
                                                                       usize]
                                                     as libc::c_int *
                                                     3 as libc::c_int /
                                                     4 as libc::c_int) as
                                                    s16b;
                                            if ((*p_ptr).stat_cur[stat as
                                                                      usize]
                                                    as libc::c_int) <
                                                   3 as libc::c_int {
                                                (*p_ptr).stat_cur[stat as
                                                                      usize] =
                                                    3 as libc::c_int as s16b
                                            }
                                            (*p_ptr).update =
                                                ((*p_ptr).update as
                                                     libc::c_long |
                                                     0x1 as libc::c_long) as
                                                    u32b
                                        }
                                        10 => {
                                            msg_print(b"You\'re not as powerful as you used to be...\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                            k = 0 as libc::c_int;
                                            while k < 6 as libc::c_int {
                                                (*p_ptr).stat_cur[k as usize]
                                                    =
                                                    ((*p_ptr).stat_cur[k as
                                                                           usize]
                                                         as libc::c_int *
                                                         3 as libc::c_int /
                                                         4 as libc::c_int) as
                                                        s16b;
                                                if ((*p_ptr).stat_cur[k as
                                                                          usize]
                                                        as libc::c_int) <
                                                       3 as libc::c_int {
                                                    (*p_ptr).stat_cur[k as
                                                                          usize]
                                                        =
                                                        3 as libc::c_int as
                                                            s16b
                                                }
                                                k += 1
                                            }
                                            (*p_ptr).update =
                                                ((*p_ptr).update as
                                                     libc::c_long |
                                                     0x1 as libc::c_long) as
                                                    u32b
                                        }
                                        _ => { }
                                    }
                                    take_hit(damage,
                                             ddesc.as_mut_ptr() as cptr);
                                }
                                33 => {
                                    /* Obvious */
                                    obvious = 1 as libc::c_int as bool_;
                                    if (*p_ptr).parasite == 0 {
                                        set_parasite(damage,
                                                     (*m_ptr).r_idx as
                                                         libc::c_int);
                                    }
                                }
                                _ => { }
                            }
                            /* Hack -- only one of cut or stun */
                            if do_cut != 0 && do_stun != 0 {
                                /* Cancel cut */
                                if Rand_div(100 as libc::c_int) <
                                       50 as libc::c_int {
                                    do_cut = 0 as libc::c_int
                                } else {
                                    /* Cancel stun */
                                    do_stun = 0 as libc::c_int
                                }
                            }
                            /* Handle cut */
                            if do_cut != 0 {
                                let mut k_0: libc::c_int = 0 as libc::c_int;
                                /* Critical hit (zero if non-critical) */
                                tmp =
                                    monster_critical(d_dice, d_side, damage);
                                /* Roll for damage */
                                match tmp {
                                    0 => { k_0 = 0 as libc::c_int }
                                    1 => {
                                        k_0 =
                                            Rand_div(5 as libc::c_int) +
                                                1 as libc::c_int
                                    }
                                    2 => {
                                        k_0 =
                                            Rand_div(5 as libc::c_int) +
                                                1 as libc::c_int +
                                                5 as libc::c_int
                                    }
                                    3 => {
                                        k_0 =
                                            Rand_div(20 as libc::c_int) +
                                                1 as libc::c_int +
                                                20 as libc::c_int
                                    }
                                    4 => {
                                        k_0 =
                                            Rand_div(50 as libc::c_int) +
                                                1 as libc::c_int +
                                                50 as libc::c_int
                                    }
                                    5 => {
                                        k_0 =
                                            Rand_div(100 as libc::c_int) +
                                                1 as libc::c_int +
                                                100 as libc::c_int
                                    }
                                    6 => { k_0 = 300 as libc::c_int }
                                    _ => { k_0 = 500 as libc::c_int }
                                }
                                /* Apply the cut */
                                if k_0 != 0 {
                                    set_cut((*p_ptr).cut as libc::c_int +
                                                k_0);
                                }
                            }
                            /* Handle stun */
                            if do_stun != 0 {
                                let mut k_1: libc::c_int = 0 as libc::c_int;
                                /* Critical hit (zero if non-critical) */
                                tmp =
                                    monster_critical(d_dice, d_side, damage);
                                /* Roll for damage */
                                match tmp {
                                    0 => { k_1 = 0 as libc::c_int }
                                    1 => {
                                        k_1 =
                                            Rand_div(5 as libc::c_int) +
                                                1 as libc::c_int
                                    }
                                    2 => {
                                        k_1 =
                                            Rand_div(10 as libc::c_int) +
                                                1 as libc::c_int +
                                                10 as libc::c_int
                                    }
                                    3 => {
                                        k_1 =
                                            Rand_div(20 as libc::c_int) +
                                                1 as libc::c_int +
                                                20 as libc::c_int
                                    }
                                    4 => {
                                        k_1 =
                                            Rand_div(30 as libc::c_int) +
                                                1 as libc::c_int +
                                                30 as libc::c_int
                                    }
                                    5 => {
                                        k_1 =
                                            Rand_div(40 as libc::c_int) +
                                                1 as libc::c_int +
                                                40 as libc::c_int
                                    }
                                    6 => { k_1 = 100 as libc::c_int }
                                    _ => { k_1 = 200 as libc::c_int }
                                }
                                /* Apply the stun */
                                if k_1 != 0 {
                                    set_stun((*p_ptr).stun as libc::c_int +
                                                 k_1);
                                }
                            }
                            /* Do vampiric thingies */
                            (do_vampire) != 0;
                            if explode != 0 {
                                sound(64 as libc::c_int);
                                if mon_take_hit(m_idx,
                                                (*m_ptr).hp +
                                                    1 as libc::c_int,
                                                &mut fear, 0 as cptr) != 0 {
                                    blinked = 0 as libc::c_int as bool_;
                                    alive = 0 as libc::c_int as bool_
                                }
                            }
                            if touched != 0 {
                                if (*p_ptr).sh_fire as libc::c_int != 0 &&
                                       alive as libc::c_int != 0 {
                                    if (*r_ptr).flags3 &
                                           0x40000 as libc::c_int as
                                               libc::c_uint == 0 {
                                        msg_format(b"%^s is suddenly very hot!\x00"
                                                       as *const u8 as
                                                       *const libc::c_char,
                                                   m_name.as_mut_ptr());
                                        if mon_take_hit(m_idx,
                                                        damroll(2 as
                                                                    libc::c_int
                                                                    as s16b,
                                                                6 as
                                                                    libc::c_int
                                                                    as s16b),
                                                        &mut fear,
                                                        b" turns into a pile of ash.\x00"
                                                            as *const u8 as
                                                            *const libc::c_char)
                                               != 0 {
                                            blinked =
                                                0 as libc::c_int as bool_;
                                            alive = 0 as libc::c_int as bool_
                                        }
                                    } else if (*m_ptr).ml != 0 {
                                        (*r_ptr).r_flags3 |=
                                            0x40000 as libc::c_int as
                                                libc::c_uint
                                    }
                                }
                                if (*p_ptr).sh_elec as libc::c_int != 0 &&
                                       alive as libc::c_int != 0 {
                                    if (*r_ptr).flags3 &
                                           0x20000 as libc::c_int as
                                               libc::c_uint == 0 {
                                        msg_format(b"%^s gets zapped!\x00" as
                                                       *const u8 as
                                                       *const libc::c_char,
                                                   m_name.as_mut_ptr());
                                        if mon_take_hit(m_idx,
                                                        damroll(2 as
                                                                    libc::c_int
                                                                    as s16b,
                                                                6 as
                                                                    libc::c_int
                                                                    as s16b),
                                                        &mut fear,
                                                        b" turns into a pile of cinder.\x00"
                                                            as *const u8 as
                                                            *const libc::c_char)
                                               != 0 {
                                            blinked =
                                                0 as libc::c_int as bool_;
                                            alive = 0 as libc::c_int as bool_
                                        }
                                    } else if (*m_ptr).ml != 0 {
                                        (*r_ptr).r_flags3 |=
                                            0x20000 as libc::c_int as
                                                libc::c_uint
                                    }
                                }
                                if (*p_ptr).shield as libc::c_int != 0 &&
                                       (*p_ptr).shield_opt as libc::c_int &
                                           0x1 as libc::c_int != 0 &&
                                       alive as libc::c_int != 0 {
                                    msg_format(b"%^s gets bashed by your mystic shield!\x00"
                                                   as *const u8 as
                                                   *const libc::c_char,
                                               m_name.as_mut_ptr());
                                    if mon_take_hit(m_idx,
                                                    damroll((*p_ptr).shield_power_opt,
                                                            (*p_ptr).shield_power_opt2),
                                                    &mut fear,
                                                    b" is bashed by your mystic shield.\x00"
                                                        as *const u8 as
                                                        *const libc::c_char)
                                           != 0 {
                                        blinked = 0 as libc::c_int as bool_;
                                        alive = 0 as libc::c_int as bool_
                                    }
                                }
                                if (*p_ptr).shield as libc::c_int != 0 &&
                                       (*p_ptr).shield_opt as libc::c_int &
                                           0x2 as libc::c_int != 0 &&
                                       alive as libc::c_int != 0 {
                                    if (*r_ptr).flags3 &
                                           0x40000 as libc::c_int as
                                               libc::c_uint == 0 {
                                        msg_format(b"%^s gets burned by your fiery shield!\x00"
                                                       as *const u8 as
                                                       *const libc::c_char,
                                                   m_name.as_mut_ptr());
                                        if mon_take_hit(m_idx,
                                                        damroll((*p_ptr).shield_power_opt,
                                                                (*p_ptr).shield_power_opt2),
                                                        &mut fear,
                                                        b" is burned by your fiery shield.\x00"
                                                            as *const u8 as
                                                            *const libc::c_char)
                                               != 0 {
                                            blinked =
                                                0 as libc::c_int as bool_;
                                            alive = 0 as libc::c_int as bool_
                                        }
                                    } else if (*m_ptr).ml != 0 {
                                        (*r_ptr).r_flags3 |=
                                            0x40000 as libc::c_int as
                                                libc::c_uint
                                    }
                                }
                                if (*p_ptr).shield as libc::c_int != 0 &&
                                       (*p_ptr).shield_opt as libc::c_int &
                                           0x4 as libc::c_int != 0 &&
                                       alive as libc::c_int != 0 {
                                    msg_format(b"%^s gets burned by your fiery shield!\x00"
                                                   as *const u8 as
                                                   *const libc::c_char,
                                               m_name.as_mut_ptr());
                                    if mon_take_hit(m_idx,
                                                    damroll((*p_ptr).shield_power_opt,
                                                            (*p_ptr).shield_power_opt2),
                                                    &mut fear,
                                                    b" is burned by your fiery shield.\x00"
                                                        as *const u8 as
                                                        *const libc::c_char)
                                           != 0 {
                                        blinked = 0 as libc::c_int as bool_;
                                        alive = 0 as libc::c_int as bool_
                                    }
                                }
                                if (*p_ptr).shield as libc::c_int != 0 &&
                                       (*p_ptr).shield_opt as libc::c_int &
                                           0x8 as libc::c_int != 0 &&
                                       alive as libc::c_int != 0 {
                                    let mut tmp_0: libc::c_int = 0;
                                    if (*r_ptr).flags1 &
                                           0x1 as libc::c_int as libc::c_uint
                                           == 0 &&
                                           damroll((*p_ptr).shield_power_opt,
                                                   (*p_ptr).shield_power_opt2)
                                               - (*m_ptr).level as libc::c_int
                                               > 0 as libc::c_int {
                                        msg_format(b"%^s gets scared away!\x00"
                                                       as *const u8 as
                                                       *const libc::c_char,
                                                   m_name.as_mut_ptr());
                                        /* Increase fear */
                                        tmp_0 =
                                            (*m_ptr).monfear as libc::c_int +
                                                (*p_ptr).shield_power_opt as
                                                    libc::c_int;
                                        fear = 1 as libc::c_int as bool_;
                                        /* Set fear */
                                        (*m_ptr).monfear =
                                            if tmp_0 < 200 as libc::c_int {
                                                tmp_0
                                            } else { 200 as libc::c_int } as
                                                byte_hack
                                    }
                                }
                                touched = 0 as libc::c_int as bool_
                            }
                            current_block_608 = 15033393841840802250;
                        }
                    }
                }
            }
        } else {
            /* Monster missed player */
            /* Analyze failed attacks */
            match method {
                1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 => {
                    /* Visible monsters */
                    if (*m_ptr).ml != 0 {
                        /* Disturbing */
                        disturb(1 as libc::c_int, 0 as libc::c_int);
                        /* Message */
                        msg_format(b"%^s misses you.\x00" as *const u8 as
                                       *const libc::c_char,
                                   m_name.as_mut_ptr());
                    }
                }
                _ => { }
            }
            current_block_608 = 15033393841840802250;
        }
        match current_block_608 {
            15033393841840802250 => {
                /* Analyze "visible" monsters only */
                if visible != 0 {
                    /* Count "obvious" attacks (and ones that cause damage) */
                    if obvious as libc::c_int != 0 || damage != 0 ||
                           (*r_ptr).r_blows[ap_cnt as usize] as libc::c_int >
                               10 as libc::c_int {
                        /* Count attacks of this type */
                        if ((*r_ptr).r_blows[ap_cnt as usize] as libc::c_int)
                               < 255 as libc::c_int {
                            (*r_ptr).r_blows[ap_cnt as usize] =
                                (*r_ptr).r_blows[ap_cnt as
                                                     usize].wrapping_add(1)
                        }
                    }
                }
            }
            _ => { }
        }
        /* Hack -- Next attack */
        ap_cnt += 1
    }
    /* Blink away */
    if blinked != 0 {
        msg_print(b"The thief flees laughing!\x00" as *const u8 as
                      *const libc::c_char);
        teleport_away(m_idx,
                      20 as libc::c_int * 2 as libc::c_int +
                          5 as libc::c_int);
    }
    /* Always notice cause of death */
    if death as libc::c_int != 0 &&
           ((*r_ptr).r_deaths as libc::c_int) < 32767 as libc::c_int {
        (*r_ptr).r_deaths += 1
    }
    if (*m_ptr).ml as libc::c_int != 0 && fear as libc::c_int != 0 {
        sound(3 as libc::c_int);
        msg_format(b"%^s flees in terror!\x00" as *const u8 as
                       *const libc::c_char, m_name.as_mut_ptr());
    }
    /* Assume we attacked */
    return 1 as libc::c_int as bool_;
}
