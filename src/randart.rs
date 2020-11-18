use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut wizard: bool_;
    #[no_mangle]
    static mut flush_failure: bool_;
    #[no_mangle]
    static mut cheat_peek: bool_;
    #[no_mangle]
    static mut player_name: [libc::c_char; 32];
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut ra_info: *mut randart_part_type;
    #[no_mangle]
    static mut ra_gen: [randart_gen_type; 30];
    #[no_mangle]
    static mut item_tester_hook:
           Option<unsafe extern "C" fn(_: *mut object_type) -> bool_>;
    #[no_mangle]
    static mut max_ra_idx: u16b;
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn ralloc(len: huge_hack) -> vptr;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn damroll(num: s16b, sides: s16b) -> s32b;
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
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn get_object(item: libc::c_int) -> *mut object_type;
    #[no_mangle]
    fn add_random_ego_flag(o_ptr: *mut object_type, fego: libc::c_int,
                           limit_blows: *mut bool_);
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
    fn get_item(cp: *mut libc::c_int, pmt: cptr, str: cptr, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn object_known(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_aware(o_ptr: *mut object_type);
    #[no_mangle]
    fn flag_cost(o_ptr: *mut object_type, plusses: libc::c_int) -> s32b;
    #[no_mangle]
    fn curse_artifact(o_ptr: *mut object_type);
    #[no_mangle]
    fn item_tester_hook_artifactable(o_ptr: *mut object_type) -> bool_;
    #[no_mangle]
    fn call_lua(function: cptr, args: cptr, ret: cptr, _: ...) -> bool_;
    #[no_mangle]
    fn quark_add(str: cptr) -> s16b;
    #[no_mangle]
    fn is_a_vowel(ch: libc::c_int) -> bool_;
    #[no_mangle]
    fn get_string(prompt: cptr, buf: *mut libc::c_char, len: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn flush();
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
pub type byte_hack = libc::c_uchar;
pub type bool_ = libc::c_char;
pub type huge_hack = libc::c_ulong;
pub type s16b = libc::c_short;
pub type u16b = libc::c_ushort;
pub type s32b = libc::c_int;
pub type u32b = libc::c_uint;
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
pub struct randart_gen_type {
    pub chance: libc::c_int,
    pub dd: libc::c_int,
    pub ds: libc::c_int,
    pub plus: libc::c_int,
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
/* File: randart.c */
/* Purpose: Randart creation code */
/*
 * Copyright (c) 2001 DarkGod
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/* Chance of using syllables to form the name instead of the "template" files */
/*
 * Attempt to add a power to a randart
 */
unsafe extern "C" fn grab_one_power(mut ra_idx: *mut libc::c_int,
                                    mut o_ptr: *mut object_type,
                                    mut good: bool_, mut max_times: *mut s16b)
 -> bool_ {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut ok_ra: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ok_num: libc::c_int = 0 as libc::c_int;
    let mut ret: bool_ = 0 as libc::c_int as bool_;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    ok_ra =
        memset(ralloc((max_ra_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_ra_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong)) as
            *mut libc::c_int;
    /* Grab the ok randart */
    i = 0 as libc::c_int;
    while i < max_ra_idx as libc::c_int {
        let mut ra_ptr: *mut randart_part_type =
            &mut *ra_info.offset(i as isize) as *mut randart_part_type;
        let mut ok: bool_ = 0 as libc::c_int as bool_;
        /* Must have the correct fields */
        j = 0 as libc::c_int;
        while j < 20 as libc::c_int {
            if (*ra_ptr).tval[j as usize] as libc::c_int ==
                   (*o_ptr).tval as libc::c_int {
                if (*ra_ptr).min_sval[j as usize] as libc::c_int <=
                       (*o_ptr).sval as libc::c_int &&
                       (*ra_ptr).max_sval[j as usize] as libc::c_int >=
                           (*o_ptr).sval as libc::c_int {
                    ok = 1 as libc::c_int as bool_
                }
            }
            if ok != 0 { break ; }
            j += 1
        }
        if (0 as libc::c_int) < (*ra_ptr).max_pval &&
               (*ra_ptr).max_pval < (*o_ptr).pval {
            ok = 0 as libc::c_int as bool_
        }
        if !(ok == 0) {
            /* Good should be good, bad should be bad */
            if !(good as libc::c_int != 0 &&
                     (*ra_ptr).value <= 0 as libc::c_int) {
                if !(good == 0 && (*ra_ptr).value > 0 as libc::c_int) {
                    if !(*max_times.offset(i as isize) as libc::c_int >=
                             (*ra_ptr).max as libc::c_int) {
                        /* Must NOT have the antagonic flags */
                        object_flags(o_ptr, &mut f1, &mut f2, &mut f3,
                                     &mut f4, &mut f5, &mut esp);
                        if !(f1 & (*ra_ptr).aflags1 != 0) {
                            if !(f2 & (*ra_ptr).aflags2 != 0) {
                                if !(f3 & (*ra_ptr).aflags3 != 0) {
                                    if !(f4 & (*ra_ptr).aflags4 != 0) {
                                        if !(f5 & (*ra_ptr).aflags5 != 0) {
                                            if !(esp & (*ra_ptr).aesp != 0) {
                                                /* ok */
                                                let fresh0 = ok_num;
                                                ok_num = ok_num + 1;
                                                *ok_ra.offset(fresh0 as isize)
                                                    = i
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        /* Doesnt count as a try*/
        i += 1
    }
    let mut current_block_18: u64;
    /* Now test them a few times */
    i = 0 as libc::c_int;
    while i < ok_num * 10 as libc::c_int {
        let mut ra_ptr_0: *mut randart_part_type =
            0 as *mut randart_part_type;
        i = *ok_ra.offset(Rand_div(ok_num) as isize);
        ra_ptr_0 = &mut *ra_info.offset(i as isize) as *mut randart_part_type;
        /* XXX XXX Enforce minimum player level (loosely) */
        if (*ra_ptr_0).level as libc::c_int > (*p_ptr).lev as libc::c_int {
            /* Acquire the "out-of-depth factor" */
            let mut d: libc::c_int =
                (*ra_ptr_0).level as libc::c_int -
                    (*p_ptr).lev as libc::c_int;
            /* Roll for out-of-depth creation */
            if Rand_div(d) != 0 as libc::c_int {
                current_block_18 = 652864300344834934;
            } else { current_block_18 = 980989089337379490; }
        } else { current_block_18 = 980989089337379490; }
        match current_block_18 {
            980989089337379490 =>
            /* We must make the "rarity roll" */
            {
                if !(Rand_div((*ra_ptr_0).mrarity as s32b) <
                         (*ra_ptr_0).rarity as libc::c_int) {
                    /* Hack -- mark the item as an ego */
                    *ra_idx = i;
                    let ref mut fresh1 = *max_times.offset(i as isize);
                    *fresh1 += 1;
                    /* Success */
                    ret = 1 as libc::c_int as bool_;
                    break ;
                }
            }
            _ => { }
        }
        i += 1
    }
    rnfree(ok_ra as vptr,
           (max_ra_idx as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong));
    /* Return */
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn give_activation_power(mut o_ptr: *mut object_type) {
    (*o_ptr).xtra2 = 0 as libc::c_int as s16b;
    (*o_ptr).art_flags3 =
        ((*o_ptr).art_flags3 as libc::c_long & !(0x1000000 as libc::c_long))
            as u32b;
    (*o_ptr).timeout = 0 as libc::c_int as s16b;
}
#[no_mangle]
pub unsafe extern "C" fn get_activation_power() -> libc::c_int {
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
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
    o_ptr = &mut forge;
    give_activation_power(o_ptr);
    return (*o_ptr).xtra2 as libc::c_int;
}
static mut lprobs: [[[libc::c_long; 27]; 27]; 27] = [[[0; 27]; 27]; 27];
static mut ltotal: [[libc::c_long; 27]; 27] = [[0; 27]; 27];
/*
 * Use W. Sheldon Simms' random name generator.  This function builds
 * probability tables which are used later on for letter selection.  It
 * relies on the ASCII character set.
 */
#[no_mangle]
pub unsafe extern "C" fn build_prob(mut learn: cptr) {
    let mut c_prev: libc::c_int = 0;
    let mut c_cur: libc::c_int = 0;
    let mut c_next: libc::c_int = 0;
    loop 
         /* Build raw frequencies */
         {
        c_cur = 26 as libc::c_int;
        c_prev = c_cur;
        loop  {
            let fresh2 = learn;
            learn = learn.offset(1);
            c_next = *fresh2 as libc::c_int;
            if !(*(*__ctype_b_loc()).offset(c_next as isize) as libc::c_int &
                     _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                     == 0 && c_next != '\u{0}' as i32) {
                break ;
            }
        }
        if c_next == '\u{0}' as i32 { break ; }
        loop  {
            c_next = tolower(c_next) - 'a' as i32;
            lprobs[c_prev as usize][c_cur as usize][c_next as usize] += 1;
            ltotal[c_prev as usize][c_cur as usize] += 1;
            c_prev = c_cur;
            c_cur = c_next;
            let fresh3 = learn;
            learn = learn.offset(1);
            c_next = *fresh3 as libc::c_int;
            if !(*(*__ctype_b_loc()).offset(c_next as isize) as libc::c_int &
                     _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                     != 0) {
                break ;
            }
        }
        lprobs[c_prev as usize][c_cur as usize][26 as libc::c_int as usize] +=
            1;
        ltotal[c_prev as usize][c_cur as usize] += 1
    };
}
/*
 * Use W. Sheldon Simms' random name generator.  Generate a random word using
 * the probability tables we built earlier.  Relies on the ASCII character
 * set.  Relies on European vowels (a, e, i, o, u).  The generated name should
 * be copied/used before calling this function again.
 */
unsafe extern "C" fn make_word() -> *mut libc::c_char {
    static mut word_buf: [libc::c_char; 90] = [0; 90];
    let mut r: libc::c_int = 0;
    let mut totalfreq: libc::c_int = 0;
    let mut tries: libc::c_int = 0;
    let mut lnum: libc::c_int = 0;
    let mut vow: libc::c_int = 0;
    let mut c_prev: libc::c_int = 0;
    let mut c_cur: libc::c_int = 0;
    let mut c_next: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    'c_8646:
        loop  {
            vow = 0 as libc::c_int;
            lnum = 0 as libc::c_int;
            tries = 0 as libc::c_int;
            cp = word_buf.as_mut_ptr();
            c_cur = 26 as libc::c_int;
            c_prev = c_cur;
            loop  {
                c_next = 0 as libc::c_int;
                r = Rand_div(ltotal[c_prev as usize][c_cur as usize] as s32b);
                totalfreq =
                    lprobs[c_prev as usize][c_cur as usize][c_next as usize]
                        as libc::c_int;
                while totalfreq <= r {
                    c_next += 1;
                    totalfreq =
                        (totalfreq as libc::c_long +
                             lprobs[c_prev as
                                        usize][c_cur as
                                                   usize][c_next as usize]) as
                            libc::c_int
                }
                if c_next == 26 as libc::c_int {
                    if !(lnum < 5 as libc::c_int || vow == 0 as libc::c_int) {
                        break 'c_8646 ;
                    }
                    tries += 1;
                    if !(tries < 10 as libc::c_int) { break ; }
                } else {
                    if lnum >= 9 as libc::c_int { break ; }
                    *cp = (c_next + 'a' as i32) as libc::c_char;
                    if is_a_vowel(*cp as libc::c_int) != 0 { vow += 1 }
                    cp = cp.offset(1);
                    lnum += 1;
                    c_prev = c_cur;
                    c_cur = c_next
                }
            }
        }
    *cp = '\u{0}' as i32 as libc::c_char;
    word_buf[0 as libc::c_int as usize] =
        toupper(word_buf[0 as libc::c_int as usize] as libc::c_int) as
            libc::c_char;
    return word_buf.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn get_random_name(mut return_name: *mut libc::c_char) {
    let mut word: *mut libc::c_char = make_word();
    if Rand_div(3 as libc::c_int) == 0 as libc::c_int {
        sprintf(return_name,
                b"\'%s\'\x00" as *const u8 as *const libc::c_char, word);
    } else {
        sprintf(return_name, b"of %s\x00" as *const u8 as *const libc::c_char,
                word);
    };
}
#[no_mangle]
pub unsafe extern "C" fn create_artifact(mut o_ptr: *mut object_type,
                                         mut a_scroll: bool_,
                                         mut get_name: bool_) -> bool_ {
    let mut new_name: [libc::c_char; 80] = [0; 80];
    let mut powers: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut total_flags: s32b = 0;
    let mut total_power: s32b = 0 as libc::c_int;
    let mut a_cursed: bool_ = 0 as libc::c_int as bool_;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut max_times: *mut s16b = 0 as *mut s16b;
    let mut pval: s16b = 0 as libc::c_int as s16b;
    let mut limit_blows: bool_ = 0 as libc::c_int as bool_;
    strcpy(new_name.as_mut_ptr(),
           b"\x00" as *const u8 as *const libc::c_char);
    if a_scroll == 0 &&
           Rand_div(13 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int
       {
        a_cursed = 1 as libc::c_int as bool_
    }
    i = 0 as libc::c_int;
    while ra_gen[i as usize].chance != 0 {
        powers +=
            damroll(ra_gen[i as usize].dd as s16b,
                    ra_gen[i as usize].ds as s16b) + ra_gen[i as usize].plus;
        i += 1
    }
    if a_cursed == 0 &&
           Rand_div(30 as libc::c_int) + 1 as libc::c_int == 1 as libc::c_int
       {
        powers *= 2 as libc::c_int
    }
    if a_cursed != 0 { powers /= 2 as libc::c_int }
    max_times =
        memset(ralloc((max_ra_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<s16b>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_ra_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<s16b>()
                                                    as libc::c_ulong)) as
            *mut s16b;
    /* Main loop */
    while powers != 0 {
        let mut ra_idx: libc::c_int = 0;
        let mut ra_ptr: *mut randart_part_type = 0 as *mut randart_part_type;
        powers -= 1;
        if grab_one_power(&mut ra_idx, o_ptr, 1 as libc::c_int as bool_,
                          max_times) == 0 {
            continue ;
        }
        ra_ptr =
            &mut *ra_info.offset(ra_idx as isize) as *mut randart_part_type;
        if wizard != 0 {
            msg_format(b"Adding randart power: %d\x00" as *const u8 as
                           *const libc::c_char, ra_idx);
        }
        total_power += (*ra_ptr).value;
        (*o_ptr).art_flags1 |= (*ra_ptr).flags1;
        (*o_ptr).art_flags2 |= (*ra_ptr).flags2;
        (*o_ptr).art_flags3 |= (*ra_ptr).flags3;
        (*o_ptr).art_flags4 |= (*ra_ptr).flags4;
        (*o_ptr).art_flags5 |= (*ra_ptr).flags5;
        (*o_ptr).art_esp |= (*ra_ptr).esp;
        add_random_ego_flag(o_ptr, (*ra_ptr).fego as libc::c_int,
                            &mut limit_blows);
        /* get flags */
        object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                     &mut esp);
        /* Hack -- acquire "cursed" flag */
        if f3 as libc::c_long & 0x20000000 as libc::c_long != 0 {
            (*o_ptr).ident =
                ((*o_ptr).ident as libc::c_int | 0x40 as libc::c_int) as
                    byte_hack
        }
        /* Hack -- obtain bonuses */
        if (*ra_ptr).max_to_h as libc::c_int > 0 as libc::c_int {
            (*o_ptr).to_h =
                ((*o_ptr).to_h as libc::c_int +
                     (Rand_div((*ra_ptr).max_to_h as s32b) +
                          1 as libc::c_int)) as s16b
        }
        if ((*ra_ptr).max_to_h as libc::c_int) < 0 as libc::c_int {
            (*o_ptr).to_h =
                ((*o_ptr).to_h as libc::c_int -
                     (Rand_div(-((*ra_ptr).max_to_h as libc::c_int)) +
                          1 as libc::c_int)) as s16b
        }
        if (*ra_ptr).max_to_d as libc::c_int > 0 as libc::c_int {
            (*o_ptr).to_d =
                ((*o_ptr).to_d as libc::c_int +
                     (Rand_div((*ra_ptr).max_to_d as s32b) +
                          1 as libc::c_int)) as s16b
        }
        if ((*ra_ptr).max_to_d as libc::c_int) < 0 as libc::c_int {
            (*o_ptr).to_d =
                ((*o_ptr).to_d as libc::c_int -
                     (Rand_div(-((*ra_ptr).max_to_d as libc::c_int)) +
                          1 as libc::c_int)) as s16b
        }
        if (*ra_ptr).max_to_a as libc::c_int > 0 as libc::c_int {
            (*o_ptr).to_a =
                ((*o_ptr).to_a as libc::c_int +
                     (Rand_div((*ra_ptr).max_to_a as s32b) +
                          1 as libc::c_int)) as s16b
        }
        if ((*ra_ptr).max_to_a as libc::c_int) < 0 as libc::c_int {
            (*o_ptr).to_a =
                ((*o_ptr).to_a as libc::c_int -
                     (Rand_div(-((*ra_ptr).max_to_a as libc::c_int)) +
                          1 as libc::c_int)) as s16b
        }
        /* Hack -- obtain pval */
        if pval as libc::c_int > (*ra_ptr).max_pval && (*ra_ptr).max_pval != 0
               || pval == 0 {
            pval = (*ra_ptr).max_pval as s16b
        }
    }
    rnfree(max_times as vptr,
           (max_ra_idx as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<s16b>() as
                                                libc::c_ulong));
    if pval as libc::c_int > 0 as libc::c_int {
        (*o_ptr).pval = Rand_div(pval as s32b) + 1 as libc::c_int
    }
    if (pval as libc::c_int) < 0 as libc::c_int {
        (*o_ptr).pval = Rand_div(-(pval as libc::c_int)) + 1 as libc::c_int
    }
    /* No insane number of blows */
    if limit_blows as libc::c_int != 0 &&
           (*o_ptr).art_flags1 as libc::c_long & 0x2000 as libc::c_long != 0 {
        if (*o_ptr).pval > 2 as libc::c_int {
            (*o_ptr).pval = Rand_div(2 as libc::c_int) + 1 as libc::c_int
        }
    }
    /* Just to be sure */
    (*o_ptr).art_flags3 =
        ((*o_ptr).art_flags3 as libc::c_long |
             (0x100000 as libc::c_long | 0x200000 as libc::c_long |
                  0x400000 as libc::c_long | 0x800000 as libc::c_long)) as
            u32b;
    total_flags = flag_cost(o_ptr, (*o_ptr).pval);
    if cheat_peek != 0 {
        msg_format(b"%ld\x00" as *const u8 as *const libc::c_char,
                   total_flags);
    }
    if a_cursed != 0 { curse_artifact(o_ptr); }
    /* Extract the flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    if get_name != 0 {
        if a_scroll != 0 {
            let mut dummy_name: [libc::c_char; 80] = [0; 80];
            /* Identify it fully */
            object_aware(o_ptr);
            object_known(o_ptr);
            (*o_ptr).ident =
                ((*o_ptr).ident as libc::c_int |
                     (0x10 as libc::c_int | 0x20 as libc::c_int)) as
                    byte_hack;
            strcpy(dummy_name.as_mut_ptr(),
                   b"\x00" as *const u8 as *const libc::c_char);
            object_out_desc(o_ptr, 0 as *mut FILE, 0 as libc::c_int as bool_,
                            1 as libc::c_int as bool_);
            if get_string(b"What do you want to call the artifact? \x00" as
                              *const u8 as *const libc::c_char,
                          dummy_name.as_mut_ptr(), 80 as libc::c_int) != 0 {
                strcpy(new_name.as_mut_ptr(),
                       b"called \'\x00" as *const u8 as *const libc::c_char);
                strcat(new_name.as_mut_ptr(), dummy_name.as_mut_ptr());
                strcat(new_name.as_mut_ptr(),
                       b"\'\x00" as *const u8 as *const libc::c_char);
            } else {
                /* Default name = of 'player name' */
                sprintf(new_name.as_mut_ptr(),
                        b"of \'%s\'\x00" as *const u8 as *const libc::c_char,
                        player_name.as_mut_ptr());
            }
        } else { get_random_name(new_name.as_mut_ptr()); }
    }
    /* Save the inscription */
    (*o_ptr).art_name = quark_add(new_name.as_mut_ptr() as cptr) as u16b;
    (*o_ptr).name2b = 0 as libc::c_int as s16b;
    (*o_ptr).name2 = (*o_ptr).name2b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long |
             (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
    /* HACKS for ToME */
    if (*o_ptr).tval as libc::c_int == 35 as libc::c_int &&
           (*o_ptr).sval as libc::c_int == 100 as libc::c_int {
        let mut mimic: s32b = 0;
        call_lua(b"find_random_mimic_shape\x00" as *const u8 as
                     *const libc::c_char,
                 b"(d,d)\x00" as *const u8 as *const libc::c_char,
                 b"d\x00" as *const u8 as *const libc::c_char,
                 127 as libc::c_int, 1 as libc::c_int,
                 &mut mimic as *mut s32b);
        (*o_ptr).pval2 = mimic as s16b
    } else if f5 as libc::c_long & 0x800 as libc::c_long != 0 {
        (*o_ptr).pval2 = -(1 as libc::c_int) as s16b
    }
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn artifact_scroll() -> bool_ {
    let mut item: libc::c_int = 0;
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_name: [libc::c_char; 80] = [0; 80];
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Enchant weapon/armour */
    item_tester_hook =
        Some(item_tester_hook_artifactable as
                 unsafe extern "C" fn(_: *mut object_type) -> bool_);
    /* Get an item */
    q = b"Enchant which item? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have nothing to enchant.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s,
                0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int)
           == 0 {
        return 0 as libc::c_int as bool_
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* Description */
    object_desc(o_name.as_mut_ptr(), o_ptr, 0 as libc::c_int,
                0 as libc::c_int);
    /* Describe */
    msg_format(b"%s %s radiate%s a blinding light!\x00" as *const u8 as
                   *const libc::c_char,
               if item >= 0 as libc::c_int {
                   b"Your\x00" as *const u8 as *const libc::c_char
               } else { b"The\x00" as *const u8 as *const libc::c_char },
               o_name.as_mut_ptr(),
               if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                   b"\x00" as *const u8 as *const libc::c_char
               } else { b"s\x00" as *const u8 as *const libc::c_char });
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
        msg_format(b"The %s %s already %s!\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr(),
                   if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                       b"are\x00" as *const u8 as *const libc::c_char
                   } else { b"is\x00" as *const u8 as *const libc::c_char },
                   if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                       b"artifacts\x00" as *const u8 as *const libc::c_char
                   } else {
                       b"an artifact\x00" as *const u8 as *const libc::c_char
                   });
        okay = 0 as libc::c_int as bool_
    } else if (*o_ptr).name2 != 0 {
        msg_format(b"The %s %s already %s!\x00" as *const u8 as
                       *const libc::c_char, o_name.as_mut_ptr(),
                   if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                       b"are\x00" as *const u8 as *const libc::c_char
                   } else { b"is\x00" as *const u8 as *const libc::c_char },
                   if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                       b"ego items\x00" as *const u8 as *const libc::c_char
                   } else {
                       b"an ego item\x00" as *const u8 as *const libc::c_char
                   });
        okay = 0 as libc::c_int as bool_
    } else {
        if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
            msg_print(b"Not enough enough energy to enchant more than one object!\x00"
                          as *const u8 as *const libc::c_char);
            msg_format(b"%d of your %s %s destroyed!\x00" as *const u8 as
                           *const libc::c_char,
                       (*o_ptr).number as libc::c_int - 1 as libc::c_int,
                       o_name.as_mut_ptr(),
                       if (*o_ptr).number as libc::c_int > 2 as libc::c_int {
                           b"were\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"was\x00" as *const u8 as *const libc::c_char
                       });
            (*o_ptr).number = 1 as libc::c_int as byte_hack
        }
        okay =
            create_artifact(o_ptr, 1 as libc::c_int as bool_,
                            1 as libc::c_int as bool_)
    }
    /* Failure */
    if okay == 0 {
        /* Flush */
        if flush_failure != 0 { flush(); }
        /* Message */
        msg_print(b"The enchantment failed.\x00" as *const u8 as
                      *const libc::c_char);
    } else { (*o_ptr).found = 9 as libc::c_int as byte_hack }
    /* Something happened */
    return 1 as libc::c_int as bool_;
}
