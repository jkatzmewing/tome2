use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut monster_powers: [monster_power; 96];
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut energy_use: s32b;
    #[no_mangle]
    static mut num_repro: s16b;
    #[no_mangle]
    static mut shimmer_monsters: bool_;
    #[no_mangle]
    static mut m_max: s16b;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strnfmt(buf: *mut libc::c_char, max: uint_hack, fmt: cptr, _: ...)
     -> uint_hack;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn damroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn maxroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn Term_save() -> errr;
    #[no_mangle]
    fn Term_load() -> errr;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut o_list: *mut object_type;
    #[no_mangle]
    static mut m_list: *mut monster_type;
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    fn show_monster_inven(m_idx: libc::c_int, monst_list: *mut libc::c_int)
     -> byte_hack;
    #[no_mangle]
    static mut monst_spell_monst_spell: libc::c_int;
    #[no_mangle]
    fn race_info_idx(r_idx: libc::c_int, ego: libc::c_int)
     -> *mut monster_race;
    #[no_mangle]
    fn delete_monster_idx(i: libc::c_int);
    #[no_mangle]
    fn monster_desc(desc: *mut libc::c_char, m_ptr: *mut monster_type,
                    mode: libc::c_int);
    #[no_mangle]
    fn monster_race_desc(desc: *mut libc::c_char, r_idx: libc::c_int,
                         ego: libc::c_int);
    #[no_mangle]
    fn update_mon(m_idx: libc::c_int, full: bool_);
    #[no_mangle]
    fn multiply_monster(m_idx: libc::c_int, charm: bool_, clone: bool_)
     -> bool_;
    #[no_mangle]
    fn monster_drop_carried_objects(m_ptr: *mut monster_type);
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn tgt_pt(x: *mut libc::c_int, y: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn get_skill_scale(skill: libc::c_int, scale: u32b) -> s16b;
    #[no_mangle]
    fn get_check(prompt: cptr) -> bool_;
    #[no_mangle]
    fn bell();
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn get_com(prompt: cptr, command: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn excise_object_idx(o_idx: libc::c_int);
    #[no_mangle]
    fn screen_load();
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn screen_save();
    #[no_mangle]
    fn get_rep_dir(dp: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn delete_object_idx(o_idx: libc::c_int);
    #[no_mangle]
    fn inc_piety(god: libc::c_int, amt: s32b);
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
pub struct monster_power {
    pub power: u32b,
    pub name: cptr,
    pub mana: libc::c_int,
    pub great: bool_,
}
/* File: monster3.c */
/* Purpose: Monsters AI */
/*
 * Copyright (c) 1989 James E. Wilson, Robert A. Koeneke
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Is the mon,ster in friendly state(pet, friend, ..)
 * -1 = enemy, 0 = neutral, 1 = friend
 */
#[no_mangle]
pub unsafe extern "C" fn is_friend(mut m_ptr: *mut monster_type)
 -> libc::c_int {
    match (*m_ptr).status as libc::c_int {
        1 | 2 | 3 | 4 => {
            /* pets/friends/companion attacks monsters */
            return 1 as libc::c_int
        }
        -1 | -2 => { return -(1 as libc::c_int) }
        0 => { return 0 as libc::c_int }
        _ => { }
    }
    /* OUPS */
    return 0 as libc::c_int;
}
/* Should they attack each others */
#[no_mangle]
pub unsafe extern "C" fn is_enemy(mut m_ptr: *mut monster_type,
                                  mut t_ptr: *mut monster_type) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*m_ptr).r_idx as isize) as *mut monster_race;
    let mut rt_ptr: *mut monster_race =
        &mut *r_info.offset((*t_ptr).r_idx as isize) as *mut monster_race;
    let mut s1: libc::c_int = is_friend(m_ptr);
    let mut s2: libc::c_int = is_friend(t_ptr);
    /* Monsters hates breeders */
    if (*m_ptr).status as libc::c_int != 0 as libc::c_int &&
           (*rt_ptr).flags4 & 0x2 as libc::c_int as libc::c_uint != 0 &&
           num_repro as libc::c_int >
               100 as libc::c_int * 2 as libc::c_int / 3 as libc::c_int &&
           (*r_ptr).d_char as libc::c_int != (*rt_ptr).d_char as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    if (*t_ptr).status as libc::c_int != 0 as libc::c_int &&
           (*r_ptr).flags4 & 0x2 as libc::c_int as libc::c_uint != 0 &&
           num_repro as libc::c_int >
               100 as libc::c_int * 2 as libc::c_int / 3 as libc::c_int &&
           (*r_ptr).d_char as libc::c_int != (*rt_ptr).d_char as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* No special conditions, lets test normal flags */
    if s1 != 0 && s2 != 0 && s1 == -s2 { return 1 as libc::c_int as bool_ }
    /* Not ennemy */
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn change_side(mut m_ptr: *mut monster_type) -> bool_ {
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    /* neutrals and companions  */
    match (*m_ptr).status as libc::c_int {
        2 => {
            (*m_ptr).status = -(2 as libc::c_int) as s16b;
            if (*r_ptr).flags3 & 0x80 as libc::c_int as libc::c_uint != 0 &&
                   (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint == 0
               {
                inc_piety(5 as libc::c_int,
                          -((*m_ptr).level as libc::c_int) *
                              4 as libc::c_int);
            }
        }
        1 => { (*m_ptr).status = -(1 as libc::c_int) as s16b }
        -1 => { (*m_ptr).status = 1 as libc::c_int as s16b }
        3 => {
            (*m_ptr).status = -(2 as libc::c_int) as s16b;
            if (*r_ptr).flags3 & 0x80 as libc::c_int as libc::c_uint != 0 &&
                   (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint == 0
               {
                inc_piety(5 as libc::c_int,
                          -((*m_ptr).level as libc::c_int) *
                              4 as libc::c_int);
            }
        }
        4 => { return 0 as libc::c_int as bool_ }
        _ => { }
    }
    /* changed */
    return 1 as libc::c_int as bool_;
}
/* Multiply !! */
#[no_mangle]
pub unsafe extern "C" fn ai_multiply(mut m_idx: libc::c_int) -> bool_ {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*m_ptr).r_idx as isize) as *mut monster_race;
    let mut k: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut oy: libc::c_int = (*m_ptr).fy as libc::c_int;
    let mut ox: libc::c_int = (*m_ptr).fx as libc::c_int;
    let mut is_frien: bool_ =
        (is_friend(m_ptr) > 0 as libc::c_int) as libc::c_int as bool_;
    /* Count the adjacent monsters */
    k = 0 as libc::c_int;
    y = oy - 1 as libc::c_int;
    while y <= oy + 1 as libc::c_int {
        x = ox - 1 as libc::c_int;
        while x <= ox + 1 as libc::c_int {
            if (*cave[y as usize].offset(x as isize)).m_idx != 0 { k += 1 }
            x += 1
        }
        y += 1
    }
    if is_friend(m_ptr) > 0 as libc::c_int {
        is_frien = 1 as libc::c_int as bool_
    } else { is_frien = 0 as libc::c_int as bool_ }
    /* Hack -- multiply slower in crowded areas */
    if k < 4 as libc::c_int &&
           (k == 0 || Rand_div(k * 10 as libc::c_int) == 0) {
        /* Try to multiply */
        if multiply_monster(m_idx, is_frien, 0 as libc::c_int as bool_) != 0 {
            /* Take note if visible */
            if (*m_ptr).ml != 0 {
                (*r_ptr).r_flags4 |= 0x2 as libc::c_int as libc::c_uint
            }
            /* Multiplying takes energy */
            return 1 as libc::c_int as bool_
        }
    }
    return 0 as libc::c_int as bool_;
}
/* Possessor incarnates */
#[no_mangle]
pub unsafe extern "C" fn ai_possessor(mut m_idx: libc::c_int,
                                      mut o_idx: libc::c_int) -> bool_ {
    let mut o_ptr: *mut object_type =
        &mut *o_list.offset(o_idx as isize) as *mut object_type;
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_idx: libc::c_int = (*m_ptr).r_idx as libc::c_int;
    let mut r2_idx: libc::c_int = (*o_ptr).pval2 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r2_idx as isize) as *mut monster_race;
    let mut m_name: [libc::c_char; 80] = [0; 80];
    let mut m_name2: [libc::c_char; 80] = [0; 80];
    monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
    monster_race_desc(m_name2.as_mut_ptr(), r2_idx, 0 as libc::c_int);
    if (*m_ptr).ml != 0 {
        msg_format(b"%^s incarnates into a %s!\x00" as *const u8 as
                       *const libc::c_char, m_name.as_mut_ptr(),
                   m_name2.as_mut_ptr());
    }
    /* Remove the old one */
    delete_object_idx(o_idx);
    (*m_ptr).r_idx = r2_idx as s16b;
    (*m_ptr).ego = 0 as libc::c_int as u16b;
    /* No "damage" yet */
    (*m_ptr).stunned = 0 as libc::c_int as byte_hack;
    (*m_ptr).confused = 0 as libc::c_int as byte_hack;
    (*m_ptr).monfear = 0 as libc::c_int as byte_hack;
    /* No target yet */
    (*m_ptr).target = -(1 as libc::c_int) as s16b;
    /* Assume no sleeping */
    (*m_ptr).csleep = 0 as libc::c_int as s16b;
    /* Assign maximal hitpoints */
    if (*r_ptr).flags1 & 0x200 as libc::c_int as libc::c_uint != 0 {
        (*m_ptr).maxhp =
            maxroll((*r_ptr).hdice as s16b, (*r_ptr).hside as s16b)
    } else {
        (*m_ptr).maxhp =
            damroll((*r_ptr).hdice as s16b, (*r_ptr).hside as s16b)
    }
    /* And start out fully healthy */
    (*m_ptr).hp = (*m_ptr).maxhp;
    /* Some basic info */
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        (*m_ptr).blow[i as usize].method = (*r_ptr).blow[i as usize].method;
        (*m_ptr).blow[i as usize].effect = (*r_ptr).blow[i as usize].effect;
        (*m_ptr).blow[i as usize].d_dice = (*r_ptr).blow[i as usize].d_dice;
        (*m_ptr).blow[i as usize].d_side = (*r_ptr).blow[i as usize].d_side;
        i += 1
    }
    (*m_ptr).ac = (*r_ptr).ac;
    (*m_ptr).level = (*r_ptr).level;
    (*m_ptr).speed = (*r_ptr).speed;
    (*m_ptr).exp =
        ((if (*m_ptr).level as libc::c_int > 150 as libc::c_int {
              150 as libc::c_int
          } else { (*m_ptr).level as libc::c_int }) *
             (if (*m_ptr).level as libc::c_int > 150 as libc::c_int {
                  150 as libc::c_int
              } else { (*m_ptr).level as libc::c_int }) *
             (if (*m_ptr).level as libc::c_int > 150 as libc::c_int {
                  150 as libc::c_int
              } else { (*m_ptr).level as libc::c_int }) * 6 as libc::c_int) as
            u32b;
    /* Extract the monster base speed */
    (*m_ptr).mspeed = (*m_ptr).speed;
    (*m_ptr).energy = 0 as libc::c_int as byte_hack;
    /* Hack -- Count the number of "reproducers" */
    if (*r_ptr).flags4 & 0x2 as libc::c_int as libc::c_uint != 0 {
        num_repro += 1
    }
    /* Hack -- Notice new multi-hued monsters */
    if (*r_ptr).flags1 & 0x80 as libc::c_int as libc::c_uint != 0 {
        shimmer_monsters = 1 as libc::c_int as bool_
    }
    /* Hack -- Count the monsters on the level */
    (*r_ptr).cur_num = (*r_ptr).cur_num.wrapping_add(1);
    let ref mut fresh0 = (*r_info.offset(r_idx as isize)).cur_num;
    *fresh0 = (*fresh0).wrapping_sub(1);
    (*m_ptr).possessor = r_idx as s16b;
    /* Update the monster */
    update_mon(m_idx, 1 as libc::c_int as bool_);
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn ai_deincarnate(mut m_idx: libc::c_int) {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r2_idx: libc::c_int = (*m_ptr).possessor as libc::c_int;
    let mut r_idx: libc::c_int = (*m_ptr).r_idx as libc::c_int;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r2_idx as isize) as *mut monster_race;
    let mut i: libc::c_int = 0;
    let mut m_name: [libc::c_char; 80] = [0; 80];
    monster_desc(m_name.as_mut_ptr(), m_ptr, 0x4 as libc::c_int);
    if (*m_ptr).ml != 0 {
        msg_format(b"The soul of %s deincarnates!\x00" as *const u8 as
                       *const libc::c_char, m_name.as_mut_ptr());
    }
    (*m_ptr).r_idx = r2_idx as s16b;
    (*m_ptr).ego = 0 as libc::c_int as u16b;
    /* No "damage" yet */
    (*m_ptr).stunned = 0 as libc::c_int as byte_hack;
    (*m_ptr).confused = 0 as libc::c_int as byte_hack;
    (*m_ptr).monfear = 0 as libc::c_int as byte_hack;
    /* No target yet */
    (*m_ptr).target = -(1 as libc::c_int) as s16b;
    /* Assume no sleeping */
    (*m_ptr).csleep = 0 as libc::c_int as s16b;
    /* Assign maximal hitpoints */
    if (*r_ptr).flags1 & 0x200 as libc::c_int as libc::c_uint != 0 {
        (*m_ptr).maxhp =
            maxroll((*r_ptr).hdice as s16b, (*r_ptr).hside as s16b)
    } else {
        (*m_ptr).maxhp =
            damroll((*r_ptr).hdice as s16b, (*r_ptr).hside as s16b)
    }
    /* And start out fully healthy */
    (*m_ptr).hp = (*m_ptr).maxhp;
    /* Some basic info */
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        (*m_ptr).blow[i as usize].method = (*r_ptr).blow[i as usize].method;
        (*m_ptr).blow[i as usize].effect = (*r_ptr).blow[i as usize].effect;
        (*m_ptr).blow[i as usize].d_dice = (*r_ptr).blow[i as usize].d_dice;
        (*m_ptr).blow[i as usize].d_side = (*r_ptr).blow[i as usize].d_side;
        i += 1
    }
    (*m_ptr).ac = (*r_ptr).ac;
    (*m_ptr).level = (*r_ptr).level;
    (*m_ptr).speed = (*r_ptr).speed;
    (*m_ptr).exp =
        ((if (*m_ptr).level as libc::c_int > 150 as libc::c_int {
              150 as libc::c_int
          } else { (*m_ptr).level as libc::c_int }) *
             (if (*m_ptr).level as libc::c_int > 150 as libc::c_int {
                  150 as libc::c_int
              } else { (*m_ptr).level as libc::c_int }) *
             (if (*m_ptr).level as libc::c_int > 150 as libc::c_int {
                  150 as libc::c_int
              } else { (*m_ptr).level as libc::c_int }) * 6 as libc::c_int) as
            u32b;
    /* Extract the monster base speed */
    (*m_ptr).mspeed = (*m_ptr).speed;
    (*m_ptr).energy = 0 as libc::c_int as byte_hack;
    /* Hack -- Count the number of "reproducers" */
    if (*r_ptr).flags4 & 0x2 as libc::c_int as libc::c_uint != 0 {
        num_repro += 1
    }
    /* Hack -- Notice new multi-hued monsters */
    if (*r_ptr).flags1 & 0x80 as libc::c_int as libc::c_uint != 0 {
        shimmer_monsters = 1 as libc::c_int as bool_
    }
    /* Hack -- Count the monsters on the level */
    (*r_ptr).cur_num = (*r_ptr).cur_num.wrapping_add(1);
    let ref mut fresh1 = (*r_info.offset(r_idx as isize)).cur_num;
    *fresh1 = (*fresh1).wrapping_sub(1);
    (*m_ptr).possessor = 0 as libc::c_int as s16b;
    /* Update the monster */
    update_mon(m_idx, 1 as libc::c_int as bool_);
}
/* Returns if a new companion is allowed */
#[no_mangle]
pub unsafe extern "C" fn can_create_companion() -> bool_ {
    let mut i: libc::c_int = 0;
    let mut mcnt: libc::c_int = 0 as libc::c_int;
    i = m_max as libc::c_int - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        /* Access the monster */
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        /* Ignore "dead" monsters */
        if !((*m_ptr).r_idx == 0) {
            if (*m_ptr).status as libc::c_int == 4 as libc::c_int {
                mcnt += 1
            }
        }
        i -= 1
    }
    if mcnt <
           1 as libc::c_int +
               get_skill_scale(48 as libc::c_int, 6 as libc::c_int as u32b) as
                   libc::c_int {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
/* Player controlled monsters */
#[no_mangle]
pub unsafe extern "C" fn do_control_walk() -> bool_ {
    /* Get a "repeated" direction */
    if (*p_ptr).control != 0 {
        let mut dir: libc::c_int = 0;
        if get_rep_dir(&mut dir) != 0 {
            /* Take a turn */
            energy_use = 100 as libc::c_int;
            /* Actually move the monster */
            (*p_ptr).control_dir = dir as byte_hack
        }
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
#[no_mangle]
pub unsafe extern "C" fn do_control_inven() -> bool_ {
    let mut monst_list: [libc::c_int; 23] = [0; 23];
    if (*p_ptr).control == 0 { return 0 as libc::c_int as bool_ }
    screen_save();
    prt(b"Carried items\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int, 0 as libc::c_int);
    show_monster_inven((*p_ptr).control as libc::c_int,
                       monst_list.as_mut_ptr());
    inkey();
    screen_load();
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn do_control_pickup() -> bool_ {
    let mut this_o_idx: libc::c_int = 0;
    let mut next_o_idx: libc::c_int = 0 as libc::c_int;
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset((*p_ptr).control as isize) as *mut monster_type;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut done: bool_ = 0 as libc::c_int as bool_;
    if (*p_ptr).control == 0 { return 0 as libc::c_int as bool_ }
    /* Scan all objects in the grid */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset((*m_ptr).fy as
                                             isize)).offset((*m_ptr).fx as
                                                                isize) as
            *mut cave_type;
    this_o_idx = (*c_ptr).o_idx as libc::c_int;
    while this_o_idx != 0 {
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
        /* Acquire object */
        o_ptr = &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
        /* Acquire next object */
        next_o_idx = (*o_ptr).next_o_idx as libc::c_int;
        /* Skip gold */
        if !((*o_ptr).tval as libc::c_int == 100 as libc::c_int) {
            /* Excise the object */
            excise_object_idx(this_o_idx);
            /* Forget mark */
            (*o_ptr).marked = 0 as libc::c_int as byte_hack;
            /* Forget location */
            (*o_ptr).ix = 0 as libc::c_int as byte_hack;
            (*o_ptr).iy = (*o_ptr).ix;
            /* Memorize monster */
            (*o_ptr).held_m_idx = (*p_ptr).control;
            /* Build a stack */
            (*o_ptr).next_o_idx = (*m_ptr).hold_o_idx;
            /* Carry object */
            (*m_ptr).hold_o_idx = this_o_idx as s16b;
            done = 1 as libc::c_int as bool_
        }
        this_o_idx = next_o_idx
    }
    if done != 0 {
        msg_print(b"You pick up all objects on the floor.\x00" as *const u8 as
                      *const libc::c_char);
    }
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn do_control_drop() -> bool_ {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset((*p_ptr).control as isize) as *mut monster_type;
    if (*p_ptr).control == 0 { return 0 as libc::c_int as bool_ }
    monster_drop_carried_objects(m_ptr);
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn do_control_magic() -> bool_ {
    let mut power: libc::c_int = -(1 as libc::c_int);
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut powers: [libc::c_int; 96] = [0; 96];
    let mut flag: bool_ = 0;
    let mut redraw: bool_ = 0;
    let mut ask: libc::c_int = 0;
    let mut choice: libc::c_char = 0;
    let mut out_val: [libc::c_char; 160] = [0; 160];
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*m_list.offset((*p_ptr).control as isize)).r_idx
                                as isize) as *mut monster_race;
    let mut label: libc::c_int = 0;
    if (*p_ptr).control == 0 { return 0 as libc::c_int as bool_ }
    if get_check(b"Do you want to abandon the creature?\x00" as *const u8 as
                     *const libc::c_char) != 0 {
        if get_check(b"Abandon it permanently?\x00" as *const u8 as
                         *const libc::c_char) != 0 {
            delete_monster_idx((*p_ptr).control as libc::c_int);
        }
        (*p_ptr).control = 0 as libc::c_int as s16b;
        return 1 as libc::c_int as bool_
    }
    /* List the monster powers -- RF4_* */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if (*r_ptr).flags4 as libc::c_long & (1 as libc::c_long) << i != 0 {
            if !(monster_powers[i as usize].power == 0) {
                let fresh2 = num;
                num = num + 1;
                powers[fresh2 as usize] = i
            }
        }
        i += 1
    }
    /* List the monster powers -- RF5_* */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if (*r_ptr).flags5 as libc::c_long & (1 as libc::c_long) << i != 0 {
            if !(monster_powers[(i + 32 as libc::c_int) as usize].power == 0)
               {
                let fresh3 = num;
                num = num + 1;
                powers[fresh3 as usize] = i + 32 as libc::c_int
            }
        }
        i += 1
    }
    /* List the monster powers -- RF6_* */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if (*r_ptr).flags6 as libc::c_long & (1 as libc::c_long) << i != 0 {
            if !(monster_powers[(i + 64 as libc::c_int) as usize].power == 0)
               {
                let fresh4 = num;
                num = num + 1;
                powers[fresh4 as usize] = i + 64 as libc::c_int
            }
        }
        i += 1
    }
    if num == 0 {
        msg_print(b"You have no powers you can use.\x00" as *const u8 as
                      *const libc::c_char);
        return 1 as libc::c_int as bool_
    }
    /* Nothing chosen yet */
    flag = 0 as libc::c_int as bool_;
    /* No redraw yet */
    redraw = 0 as libc::c_int as bool_;
    /* Get the last label */
    label =
        if num <= 26 as libc::c_int {
            (num - 1 as libc::c_int) + 'a' as i32
        } else { (num - 1 as libc::c_int - 26 as libc::c_int) + '0' as i32 };
    /* Build a prompt (accept all spells) */
    strnfmt(out_val.as_mut_ptr(), 78 as libc::c_int as uint_hack,
            b"(Powers a-%c, *=List, ESC=exit) Use which power of your golem? \x00"
                as *const u8 as *const libc::c_char, label);
    /* Get a spell from the user */
    while flag == 0 &&
              get_com(out_val.as_mut_ptr() as cptr, &mut choice) as
                  libc::c_int != 0 {
        /* Request redraw */
        if choice as libc::c_int == ' ' as i32 ||
               choice as libc::c_int == '*' as i32 ||
               choice as libc::c_int == '?' as i32 {
            /* Show the list */
            if redraw == 0 {
                let mut y: byte_hack = 1 as libc::c_int as byte_hack;
                let mut x: byte_hack = 0 as libc::c_int as byte_hack;
                let mut ctr: libc::c_int = 0 as libc::c_int;
                let mut dummy: [libc::c_char; 80] = [0; 80];
                strcpy(dummy.as_mut_ptr(),
                       b"\x00" as *const u8 as *const libc::c_char);
                /* Show list */
                redraw = 1 as libc::c_int as bool_;
                /* Save the screen */
                character_icky = 1 as libc::c_int as bool_;
                Term_save();
                let fresh5 = y;
                y = y.wrapping_add(1);
                prt(b"\x00" as *const u8 as *const libc::c_char,
                    fresh5 as libc::c_int, x as libc::c_int);
                while ctr < num {
                    let mut mp_ptr: *mut monster_power =
                        &mut *monster_powers.as_mut_ptr().offset(*powers.as_mut_ptr().offset(ctr
                                                                                                 as
                                                                                                 isize)
                                                                     as isize)
                            as *mut monster_power;
                    label =
                        if ctr < 26 as libc::c_int {
                            (ctr) + 'a' as i32
                        } else { (ctr - 26 as libc::c_int) + '0' as i32 };
                    strnfmt(dummy.as_mut_ptr(),
                            80 as libc::c_int as uint_hack,
                            b" %c) %s\x00" as *const u8 as
                                *const libc::c_char, label, (*mp_ptr).name);
                    if ctr < 17 as libc::c_int {
                        prt(dummy.as_mut_ptr() as cptr,
                            y as libc::c_int + ctr, x as libc::c_int);
                    } else {
                        prt(dummy.as_mut_ptr() as cptr,
                            y as libc::c_int + ctr - 17 as libc::c_int,
                            x as libc::c_int + 40 as libc::c_int);
                    }
                    ctr += 1
                }
                if ctr < 17 as libc::c_int {
                    prt(b"\x00" as *const u8 as *const libc::c_char,
                        y as libc::c_int + ctr, x as libc::c_int);
                } else {
                    prt(b"\x00" as *const u8 as *const libc::c_char,
                        y as libc::c_int + 17 as libc::c_int,
                        x as libc::c_int);
                }
            } else {
                /* Hide the list */
                /* Hide list */
                redraw = 0 as libc::c_int as bool_;
                Term_load();
                character_icky = 0 as libc::c_int as bool_
            }
        } else {
            if choice as libc::c_int == '\r' as i32 && num == 1 as libc::c_int
               {
                choice = 'a' as i32 as libc::c_char
            }
            if *(*__ctype_b_loc()).offset(choice as libc::c_int as isize) as
                   libc::c_int &
                   _ISalpha as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 {
                /* Restore the screen */
                /* Note verify */
                ask =
                    *(*__ctype_b_loc()).offset(choice as libc::c_int as isize)
                        as libc::c_int &
                        _ISupper as libc::c_int as libc::c_ushort as
                            libc::c_int;
                /* Lowercase */
                if ask != 0 {
                    choice = tolower(choice as libc::c_int) as libc::c_char
                }
                /* Extract request */
                i =
                    if *(*__ctype_b_loc()).offset(choice as libc::c_int as
                                                      isize) as libc::c_int &
                           _ISlower as libc::c_int as libc::c_ushort as
                               libc::c_int != 0 {
                        (choice as libc::c_int) - 'a' as i32
                    } else { -(1 as libc::c_int) }
            } else {
                /* Can't uppercase digits XXX XXX XXX */
                ask = 0 as libc::c_int;
                i = choice as libc::c_int - '0' as i32 + 26 as libc::c_int
            }
            /* Totally Illegal */
            if i < 0 as libc::c_int || i >= num {
                bell();
            } else {
                /* Save the spell index */
                power = powers[i as usize];
                /* Verify it */
                if ask != 0 {
                    let mut tmp_val: [libc::c_char; 160] = [0; 160];
                    /* Prompt */
                    strnfmt(tmp_val.as_mut_ptr(),
                            78 as libc::c_int as uint_hack,
                            b"Use %s? \x00" as *const u8 as
                                *const libc::c_char,
                            monster_powers[power as usize].name);
                    /* Belay that order */
                    if get_check(tmp_val.as_mut_ptr() as cptr) == 0 {
                        continue ;
                    }
                }
                /* Stop the loop */
                flag = 1 as libc::c_int as bool_
            }
        }
    }
    /* Restore the screen */
    if redraw != 0 { Term_load(); character_icky = 0 as libc::c_int as bool_ }
    /* Take a turn */
    if flag != 0 {
        energy_use = 100 as libc::c_int;
        monst_spell_monst_spell = power + 96 as libc::c_int
    }
    return 1 as libc::c_int as bool_;
}
/* Finds the controlled monster and "reconnect" to it */
#[no_mangle]
pub unsafe extern "C" fn do_control_reconnect() -> bool_ {
    let mut i: libc::c_int = 0;
    i = m_max as libc::c_int - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        /* Access the monster */
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        /* Ignore "dead" monsters */
        if !((*m_ptr).r_idx == 0) {
            if (*m_ptr).mflag & 0x8 as libc::c_int != 0 {
                (*p_ptr).control = i as s16b;
                return 1 as libc::c_int as bool_
            }
        }
        i -= 1
    }
    return 0 as libc::c_int as bool_;
}
/*
 * Turns a simple pet into a faithful companion
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_companion() {
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    let mut ii: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    if can_create_companion() == 0 {
        msg_print(b"You cannot have any more companions.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    if tgt_pt(&mut ii, &mut jj) == 0 {
        msg_print(b"You must target a pet.\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    if (*cave[jj as usize].offset(ii as isize)).m_idx != 0 {
        let mut m_name: [libc::c_char; 100] = [0; 100];
        m_ptr =
            &mut *m_list.offset((*(*cave.as_mut_ptr().offset(jj as
                                                                 isize)).offset(ii
                                                                                    as
                                                                                    isize)).m_idx
                                    as isize) as *mut monster_type;
        monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
        if (*m_ptr).status as libc::c_int == 3 as libc::c_int {
            (*m_ptr).status = 4 as libc::c_int as s16b;
            msg_format(b"%^s agrees to follow you.\x00" as *const u8 as
                           *const libc::c_char, m_name.as_mut_ptr());
        } else {
            msg_format(b"%^s is not your pet!\x00" as *const u8 as
                           *const libc::c_char, m_name.as_mut_ptr());
        }
    } else {
        msg_print(b"You must target a pet.\x00" as *const u8 as
                      *const libc::c_char);
    };
}
/*
 * List companions to the character sheet.
 */
#[no_mangle]
pub unsafe extern "C" fn dump_companions(mut outfile: *mut FILE) {
    let mut i: libc::c_int = 0;
    let mut done_hdr: libc::c_int = 0 as libc::c_int;
    /* Process the monsters (backwards) */
    i = m_max as libc::c_int - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        /* Access the monster */
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        /* Ignore "dead" monsters */
        if !((*m_ptr).r_idx == 0) {
            if (*m_ptr).status as libc::c_int == 4 as libc::c_int {
                let mut pet_name: [libc::c_char; 80] = [0; 80];
                /* Output the header if we haven't yet. */
                if done_hdr == 0 {
                    done_hdr = 1 as libc::c_int;
                    fprintf(outfile,
                            b"\n\n  [Current companions]\n\n\x00" as *const u8
                                as *const libc::c_char);
                }
                /* List the monster. */
                monster_desc(pet_name.as_mut_ptr(), m_ptr,
                             0x88 as libc::c_int);
                fprintf(outfile,
                        b"%s (level %d)\n\x00" as *const u8 as
                            *const libc::c_char, pet_name.as_mut_ptr(),
                        (*m_ptr).level as libc::c_int);
            }
        }
        i -= 1
    };
}
