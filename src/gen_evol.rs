use ::libc;
extern "C" {
    #[no_mangle]
    static mut cur_hgt: s16b;
    #[no_mangle]
    static mut cur_wid: s16b;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut f_info: *mut feature_type;
    #[no_mangle]
    fn place_floor(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn place_filler(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn get_branch() -> libc::c_int;
    #[no_mangle]
    fn new_player_spot(branch: libc::c_int) -> bool_;
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
/*
 * Generate a game of life level and make it evolve
 */
/*
 * Copyright (c) 2003 DarkGod.
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Generate a game of life level :) and make it evolve
 */
#[no_mangle]
pub unsafe extern "C" fn evolve_level(mut noise: bool_) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cw: libc::c_int = 0 as libc::c_int;
    let mut cf: libc::c_int = 0 as libc::c_int;
    /* Add a bit of noise */
    if noise != 0 {
        i = 1 as libc::c_int;
        while i < cur_wid as libc::c_int - 1 as libc::c_int {
            j = 1 as libc::c_int;
            while j < cur_hgt as libc::c_int - 1 as libc::c_int {
                if (*f_info.offset((*cave[j as usize].offset(i as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x20 as libc::c_long != 0 {
                    cw += 1
                }
                if (*f_info.offset((*cave[j as usize].offset(i as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x10 as libc::c_long != 0 {
                    cf += 1
                }
                j += 1
            }
            i += 1
        }
        i = 1 as libc::c_int;
        while i < cur_wid as libc::c_int - 1 as libc::c_int {
            j = 1 as libc::c_int;
            while j < cur_hgt as libc::c_int - 1 as libc::c_int {
                let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
                /* Access the grid */
                c_ptr =
                    &mut *(*cave.as_mut_ptr().offset(j as
                                                         isize)).offset(i as
                                                                            isize)
                        as *mut cave_type;
                /* Permanent features should stay */
                if !((*f_info.offset((*c_ptr).feat as isize)).flags1 as
                         libc::c_long & 0x40 as libc::c_long != 0) {
                    /* Avoid evolving grids with object or monster */
                    if !((*c_ptr).o_idx as libc::c_int != 0 ||
                             (*c_ptr).m_idx as libc::c_int != 0) {
                        /* Avoid evolving player grid */
                        if !(j == (*p_ptr).py as libc::c_int &&
                                 i == (*p_ptr).px as libc::c_int) {
                            if Rand_div(100 as libc::c_int) < 7 as libc::c_int
                               {
                                if cw > cf {
                                    place_floor(j, i);
                                } else { place_filler(j, i); }
                            }
                        }
                    }
                }
                j += 1
            }
            i += 1
        }
    }
    i = 1 as libc::c_int;
    while i < cur_wid as libc::c_int - 1 as libc::c_int {
        j = 1 as libc::c_int;
        while j < cur_hgt as libc::c_int - 1 as libc::c_int {
            let mut x: libc::c_int = 0;
            let mut y: libc::c_int = 0;
            let mut c: libc::c_int = 0;
            let mut c_ptr_0: *mut cave_type = 0 as *mut cave_type;
            /* Access the grid */
            c_ptr_0 =
                &mut *(*cave.as_mut_ptr().offset(j as
                                                     isize)).offset(i as
                                                                        isize)
                    as *mut cave_type;
            /* Permanent features should stay */
            if !((*f_info.offset((*c_ptr_0).feat as isize)).flags1 as
                     libc::c_long & 0x40 as libc::c_long != 0) {
                /* Avoid evolving grids with object or monster */
                if !((*c_ptr_0).o_idx as libc::c_int != 0 ||
                         (*c_ptr_0).m_idx as libc::c_int != 0) {
                    /* Avoid evolving player grid */
                    if !(j == (*p_ptr).py as libc::c_int &&
                             i == (*p_ptr).px as libc::c_int) {
                        /* Reset tally */
                        c = 0 as libc::c_int;
                        /* Count number of surrounding walls */
                        x = i - 1 as libc::c_int;
                        while x <= i + 1 as libc::c_int {
                            y = j - 1 as libc::c_int;
                            while y <= j + 1 as libc::c_int {
                                if !(x == i && y == j) {
                                    if (*f_info.offset((*cave[y as
                                                                  usize].offset(x
                                                                                    as
                                                                                    isize)).feat
                                                           as isize)).flags1
                                           as libc::c_long &
                                           0x20 as libc::c_long != 0 {
                                        c += 1
                                    }
                                }
                                y += 1
                            }
                            x += 1
                        }
                        /*
			 * Changed these parameters a bit, so that it doesn't produce
			 * too open or too narrow levels -- pelpel
			 */
			/* Starved or suffocated */
                        if c < 4 as libc::c_int || c >= 7 as libc::c_int {
                            if (*f_info.offset((*c_ptr_0).feat as
                                                   isize)).flags1 as
                                   libc::c_long & 0x20 as libc::c_long != 0 {
                                place_floor(j, i);
                            }
                        } else if c == 4 as libc::c_int ||
                                      c == 5 as libc::c_int {
                            if (*f_info.offset((*c_ptr_0).feat as
                                                   isize)).flags1 as
                                   libc::c_long & 0x20 as libc::c_long == 0 {
                                place_filler(j, i);
                            }
                        }
                    }
                }
            }
            j += 1
        }
        i += 1
    }
    /* Spawned */
    /* Notice changes */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x100000 as libc::c_long | 0x1000000 as libc::c_long |
                  0x10000000 as libc::c_long | 0x200000 as libc::c_long)) as
            u32b;
}
#[no_mangle]
pub unsafe extern "C" fn level_generate_life() -> bool_ {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < cur_wid as libc::c_int - 1 as libc::c_int {
        j = 1 as libc::c_int;
        while j < cur_hgt as libc::c_int - 1 as libc::c_int {
            (*cave[j as usize].offset(i as isize)).info =
                (0x8 as libc::c_int | 0x2 as libc::c_int | 0x1 as libc::c_int)
                    as u16b;
            if Rand_div(100 as libc::c_int) < 45 as libc::c_int {
                place_floor(j, i);
            } else { place_filler(j, i); }
            j += 1
        }
        i += 1
    }
    evolve_level(0 as libc::c_int as bool_);
    evolve_level(0 as libc::c_int as bool_);
    evolve_level(0 as libc::c_int as bool_);
    /* Determine the character location */
    if new_player_spot(get_branch()) == 0 { return 0 as libc::c_int as bool_ }
    return 1 as libc::c_int as bool_;
}
