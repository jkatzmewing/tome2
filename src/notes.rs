use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut turn: s32b;
    #[no_mangle]
    static mut depth_in_feet: bool_;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn strftime(__s: *mut libc::c_char, __maxsize: size_t,
                __format: *const libc::c_char, __tp: *const tm) -> size_t;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strnfmt(buf: *mut libc::c_char, max: uint_hack, fmt: cptr, _: ...)
     -> uint_hack;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    static mut player_name: [libc::c_char; 32];
    #[no_mangle]
    static mut player_base: [libc::c_char; 32];
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut class_info: *mut player_class;
    #[no_mangle]
    static mut c_name: *mut libc::c_char;
    #[no_mangle]
    static mut ANGBAND_DIR_NOTE: cptr;
    #[no_mangle]
    fn show_file(name: cptr, what: cptr, line: libc::c_int, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn get_player_race_name(pr: libc::c_int, ps: libc::c_int) -> cptr;
    #[no_mangle]
    fn get_month_name(month: libc::c_int, full: bool_, compact: bool_)
     -> cptr;
    #[no_mangle]
    fn bst(what: s32b, t: s32b) -> s32b;
    #[no_mangle]
    fn path_build(buf: *mut libc::c_char, max: libc::c_int, path: cptr,
                  file: cptr) -> errr;
    #[no_mangle]
    fn my_fopen(file: cptr, mode: cptr) -> *mut FILE;
    #[no_mangle]
    fn my_fputs(fff: *mut FILE, buf: cptr, n: huge_hack) -> errr;
    #[no_mangle]
    fn my_fclose(fff: *mut FILE) -> errr;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
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
/* File: notes.c */
/* Purpose: Note taking to a file */
/*
 * Copyright (c) 1989, 1999 James E. Wilson, Robert A. Koeneke,
 * Robert Ruehlmann
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Show the notes file on the screen
 */
#[no_mangle]
pub unsafe extern "C" fn show_notes_file() {
    let mut basename: [libc::c_char; 13] = [0; 13];
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut caption: [libc::c_char; 23] = [0; 23];
    /* Hack -- extract first 8 characters of name and append an extension */
    strnfmt(basename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as
                uint_hack,
            b"%.8s.nte\x00" as *const u8 as *const libc::c_char,
            player_base.as_mut_ptr());
    basename[(::std::mem::size_of::<[libc::c_char; 13]>() as
                  libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                  libc::c_ulong) as usize] =
        '\u{0}' as i32 as libc::c_char;
    /* Build the path */
    path_build(buf.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int, ANGBAND_DIR_NOTE,
               basename.as_mut_ptr() as cptr);
    /* Use a caption, forcing direct access to the note file */
    sprintf(caption.as_mut_ptr(),
            b"Note file %s\x00" as *const u8 as *const libc::c_char,
            basename.as_mut_ptr());
    /* Invoke show_file */
    show_file(buf.as_mut_ptr() as cptr, caption.as_mut_ptr() as cptr,
              0 as libc::c_int, 0 as libc::c_int);
}
/*
 * Output a string to the notes file.
 * This is the only function that references that file.
 */
#[no_mangle]
pub unsafe extern "C" fn output_note(mut final_note: *mut libc::c_char) {
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut basename: [libc::c_char; 13] = [0; 13];
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* File type is "TEXT" */
    /* Hack -- extract first 8 characters of name and append an extension */
    strnfmt(basename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as
                uint_hack,
            b"%.8s.nte\x00" as *const u8 as *const libc::c_char,
            player_base.as_mut_ptr());
    basename[(::std::mem::size_of::<[libc::c_char; 13]>() as
                  libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                  libc::c_ulong) as usize] =
        '\u{0}' as i32 as libc::c_char;
    /* Build the path */
    path_build(buf.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int, ANGBAND_DIR_NOTE,
               basename.as_mut_ptr() as cptr);
    /* Open notes file */
    fff =
        my_fopen(buf.as_mut_ptr() as cptr,
                 b"a\x00" as *const u8 as *const libc::c_char);
    /* Failure */
    if fff.is_null() { return }
    /* Add note, and close note file */
    my_fputs(fff, final_note as cptr, 0 as libc::c_int as huge_hack);
    /* Close the handle */
    my_fclose(fff);
}
/*
 * Add note to file using a string + character symbol
 * to specify its type so that the notes file can be
 * searched easily by external utilities.
 */
#[no_mangle]
pub unsafe extern "C" fn add_note(mut note: *mut libc::c_char,
                                  mut code: libc::c_char) {
    let mut buf: [libc::c_char; 100] = [0; 100];
    let mut final_note: [libc::c_char; 100] = [0; 100];
    let mut long_day: [libc::c_char; 50] = [0; 50];
    let mut depths: [libc::c_char; 32] = [0; 32];
    /* Get the first 60 chars - so do not have an overflow */
    memset(buf.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (100 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                as libc::c_ulong));
    strncpy(buf.as_mut_ptr(), note, 60 as libc::c_int as libc::c_ulong);
    /* Get date and time */
    sprintf(long_day.as_mut_ptr(),
            b"%ld:%02ld %s, %s\x00" as *const u8 as *const libc::c_char,
            if bst(11520 as libc::c_int / 24 as libc::c_int, turn) %
                   12 as libc::c_int == 0 as libc::c_int {
                12 as libc::c_int
            } else {
                (bst(11520 as libc::c_int / 24 as libc::c_int, turn)) %
                    12 as libc::c_int
            } as libc::c_long,
            bst(11520 as libc::c_int / 24 as libc::c_int / 60 as libc::c_int,
                turn) as libc::c_long,
            if bst(11520 as libc::c_int / 24 as libc::c_int, turn) <
                   12 as libc::c_int {
                b"AM\x00" as *const u8 as *const libc::c_char
            } else { b"PM\x00" as *const u8 as *const libc::c_char },
            get_month_name(bst(11520 as libc::c_int, turn),
                           0 as libc::c_int as bool_,
                           0 as libc::c_int as bool_));
    /* Get depth  */
    if dun_level == 0 {
        strcpy(depths.as_mut_ptr(),
               b"  Town\x00" as *const u8 as *const libc::c_char);
    } else if depth_in_feet != 0 {
        sprintf(depths.as_mut_ptr(),
                b"%4dft\x00" as *const u8 as *const libc::c_char,
                dun_level as libc::c_int * 50 as libc::c_int);
    } else {
        sprintf(depths.as_mut_ptr(),
                b"Lev%3d\x00" as *const u8 as *const libc::c_char,
                dun_level as libc::c_int);
    }
    /* Make note */
    sprintf(final_note.as_mut_ptr(),
            b"%-20s %s %c: %s\x00" as *const u8 as *const libc::c_char,
            long_day.as_mut_ptr(), depths.as_mut_ptr(), code as libc::c_int,
            buf.as_mut_ptr());
    /* Output to the notes file */
    output_note(final_note.as_mut_ptr());
}
/*
 * Append a note to the notes file using a "type".
 */
#[no_mangle]
pub unsafe extern "C" fn add_note_type(mut note_number: libc::c_int) {
    let mut long_day: [libc::c_char; 50] = [0; 50];
    let mut true_long_day: [libc::c_char; 50] = [0; 50];
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut ct: time_t = time(0 as *mut time_t);
    /* Get the date */
    strftime(true_long_day.as_mut_ptr(), 30 as libc::c_int as size_t,
             b"%Y-%m-%d at %H:%M:%S\x00" as *const u8 as *const libc::c_char,
             localtime(&mut ct));
    /* Get the date */
    sprintf(buf.as_mut_ptr(), b"%ld\x00" as *const u8 as *const libc::c_char,
            (bst(11520 as libc::c_int * 365 as libc::c_int, turn) +
                 2890 as libc::c_int) as libc::c_long);
    sprintf(long_day.as_mut_ptr(),
            b"%ld:%02ld %s the %s of III %s\x00" as *const u8 as
                *const libc::c_char,
            if bst(11520 as libc::c_int / 24 as libc::c_int, turn) %
                   12 as libc::c_int == 0 as libc::c_int {
                12 as libc::c_int
            } else {
                (bst(11520 as libc::c_int / 24 as libc::c_int, turn)) %
                    12 as libc::c_int
            } as libc::c_long,
            bst(11520 as libc::c_int / 24 as libc::c_int / 60 as libc::c_int,
                turn) as libc::c_long,
            if bst(11520 as libc::c_int / 24 as libc::c_int, turn) <
                   12 as libc::c_int {
                b"AM\x00" as *const u8 as *const libc::c_char
            } else { b"PM\x00" as *const u8 as *const libc::c_char },
            get_month_name(bst(11520 as libc::c_int, turn),
                           0 as libc::c_int as bool_,
                           0 as libc::c_int as bool_), buf.as_mut_ptr());
    /* Work out what to do */
    match note_number {
        1 => {
            /* Player has just been born */
            let mut player: [libc::c_char; 100] = [0; 100];
            /* Build the string containing the player information */
            sprintf(player.as_mut_ptr(),
                    b"the %s %s\x00" as *const u8 as *const libc::c_char,
                    get_player_race_name((*p_ptr).prace as libc::c_int,
                                         (*p_ptr).pracem as libc::c_int),
                    c_name.offset((*class_info.offset((*p_ptr).pclass as
                                                          isize)).spec[(*p_ptr).pspec
                                                                           as
                                                                           usize].title
                                      as isize));
            /* Add in "character start" information */
            sprintf(buf.as_mut_ptr(),
                    b"\n================================================\n%s %s\nBorn on %s\n================================================\n\x00"
                        as *const u8 as *const libc::c_char,
                    player_name.as_mut_ptr(), player.as_mut_ptr(),
                    true_long_day.as_mut_ptr());
        }
        2 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s slew Morgoth on %s\nLong live %s!\n================================================\x00"
                        as *const u8 as *const libc::c_char,
                    player_name.as_mut_ptr(), long_day.as_mut_ptr(),
                    player_name.as_mut_ptr());
        }
        3 => {
            /* Saving the game */
            sprintf(buf.as_mut_ptr(),
                    b"\nSession end: %s\x00" as *const u8 as
                        *const libc::c_char, true_long_day.as_mut_ptr());
        }
        4 => {
            /* Entering the game after a break. */
            sprintf(buf.as_mut_ptr(),
                    b"================================================\nNew session start: %s\n\x00"
                        as *const u8 as *const libc::c_char,
                    true_long_day.as_mut_ptr());
        }
        _ => { return }
    }
    /* Output the notes to the file */
    output_note(buf.as_mut_ptr());
}
