use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut m_max: s16b;
    #[no_mangle]
    static mut m_list: *mut monster_type;
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Term_fresh() -> errr;
    #[no_mangle]
    fn Term_clear() -> errr;
    #[no_mangle]
    fn Term_save() -> errr;
    #[no_mangle]
    fn Term_load() -> errr;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn handle_stuff();
    #[no_mangle]
    static mut game_module: cptr;
    #[no_mangle]
    fn player_flags(f1: *mut u32b, f2: *mut u32b, f3: *mut u32b,
                    f4: *mut u32b, f5: *mut u32b, esp: *mut u32b);
    #[no_mangle]
    fn show_file(name: cptr, what: cptr, line: libc::c_int, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn monster_desc(desc: *mut libc::c_char, m_ptr: *mut monster_type,
                    mode: libc::c_int);
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn path_temp(buf: *mut libc::c_char, max: libc::c_int) -> errr;
    #[no_mangle]
    fn my_fopen(file: cptr, mode: cptr) -> *mut FILE;
    #[no_mangle]
    fn my_fclose(fff: *mut FILE) -> errr;
    #[no_mangle]
    fn fd_kill(file: cptr) -> errr;
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn c_put_str(attr: byte_hack, str: cptr, row: libc::c_int,
                 col: libc::c_int);
    #[no_mangle]
    fn clear_from(row: libc::c_int);
    #[no_mangle]
    fn cnv_stat(i: libc::c_int, out_val: *mut libc::c_char);
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
static mut row_x_start: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn status_attr() {
    let mut flag_arr: [[u32b; 7]; 30] = [[0; 7]; 30];
    let mut yo: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_char = 0;
    clear_from(0 as libc::c_int);
    let fresh0 = yo;
    yo = yo + 1;
    c_put_str(14 as libc::c_int as byte_hack,
              b"Statistics\x00" as *const u8 as *const libc::c_char, fresh0,
              1 as libc::c_int);
    yo += 2 as libc::c_int;
    az_line(11 as libc::c_int, flag_arr.as_mut_ptr());
    let fresh1 = yo;
    yo = yo + 1;
    statline(b"Str\x00" as *const u8 as *const libc::c_char as
                 *mut libc::c_char, 0 as libc::c_int,
             0x1 as libc::c_long as u32b, fresh1, flag_arr.as_mut_ptr());
    let fresh2 = yo;
    yo = yo + 1;
    statline(b"Int\x00" as *const u8 as *const libc::c_char as
                 *mut libc::c_char, 1 as libc::c_int,
             0x2 as libc::c_long as u32b, fresh2, flag_arr.as_mut_ptr());
    let fresh3 = yo;
    yo = yo + 1;
    statline(b"Wis\x00" as *const u8 as *const libc::c_char as
                 *mut libc::c_char, 1 as libc::c_int,
             0x4 as libc::c_long as u32b, fresh3, flag_arr.as_mut_ptr());
    let fresh4 = yo;
    yo = yo + 1;
    statline(b"Con\x00" as *const u8 as *const libc::c_char as
                 *mut libc::c_char, 4 as libc::c_int,
             0x10 as libc::c_long as u32b, fresh4, flag_arr.as_mut_ptr());
    let fresh5 = yo;
    yo = yo + 1;
    statline(b"Dex\x00" as *const u8 as *const libc::c_char as
                 *mut libc::c_char, 3 as libc::c_int,
             0x8 as libc::c_long as u32b, fresh5, flag_arr.as_mut_ptr());
    let fresh6 = yo;
    yo = yo + 1;
    statline(b"Chr\x00" as *const u8 as *const libc::c_char as
                 *mut libc::c_char, 5 as libc::c_int,
             0x20 as libc::c_long as u32b, fresh6, flag_arr.as_mut_ptr());
    let fresh7 = yo;
    yo = yo + 1;
    row_npval(b"Luck\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 5 as libc::c_int as s16b,
              0x200 as libc::c_long as u32b, fresh7, flag_arr.as_mut_ptr());
    yo += 1;
    let fresh8 = yo;
    yo = yo + 1;
    row_npval(b"Life\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 2 as libc::c_int as s16b,
              0x80 as libc::c_long as u32b, fresh8, flag_arr.as_mut_ptr());
    let fresh9 = yo;
    yo = yo + 1;
    row_npval(b"Mana\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x40 as libc::c_long as u32b, fresh9, flag_arr.as_mut_ptr());
    c_put_str(1 as libc::c_int as byte_hack,
              b"Press ESC to continue\x00" as *const u8 as
                  *const libc::c_char, 23 as libc::c_int, 0 as libc::c_int);
    Term_fresh();
    loop  { c = inkey(); if c as libc::c_int == '\u{1b}' as i32 { break ; } };
}
#[no_mangle]
pub unsafe extern "C" fn status_move() {
    let mut flag_arr: [[u32b; 7]; 30] = [[0; 7]; 30];
    let mut yo: libc::c_int = 3 as libc::c_int;
    clear_from(0 as libc::c_int);
    c_put_str(14 as libc::c_int as byte_hack,
              b"Movement\x00" as *const u8 as *const libc::c_char,
              0 as libc::c_int, 1 as libc::c_int);
    az_line(11 as libc::c_int, flag_arr.as_mut_ptr());
    let fresh10 = yo;
    yo = yo + 1;
    row_trival(b"Fly/Lev\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 4 as libc::c_int as s16b,
               0x10 as libc::c_long as u32b, 3 as libc::c_int as s16b,
               0x1000 as libc::c_long as u32b, fresh10,
               flag_arr.as_mut_ptr());
    let fresh11 = yo;
    yo = yo + 1;
    row_bival(b"Climb\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 4 as libc::c_int as s16b,
              0x800 as libc::c_long as u32b, fresh11, flag_arr.as_mut_ptr());
    let fresh12 = yo;
    yo = yo + 1;
    row_npval(b"Dig\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x800 as libc::c_long as u32b, fresh12, flag_arr.as_mut_ptr());
    let fresh13 = yo;
    yo = yo + 1;
    row_npval(b"Speed\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x1000 as libc::c_long as u32b, fresh13, flag_arr.as_mut_ptr());
    let fresh14 = yo;
    yo = yo + 1;
    row_bival(b"Wraith\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x40 as libc::c_long as u32b, fresh14, flag_arr.as_mut_ptr());
    yo += 1;
    let fresh15 = yo;
    yo = yo + 1;
    row_npval(b"Stealth\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x100 as libc::c_long as u32b, fresh15, flag_arr.as_mut_ptr());
    let fresh16 = yo;
    yo = yo + 1;
    row_bival(b"Telep\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x4000000 as libc::c_long as u32b, fresh16,
              flag_arr.as_mut_ptr());
    c_put_str(1 as libc::c_int as byte_hack,
              b"Press ESC to continue\x00" as *const u8 as
                  *const libc::c_char, 23 as libc::c_int, 0 as libc::c_int);
    Term_fresh();
    loop  {
        let mut loop_exit: bool_ = 0 as libc::c_int as bool_;
        let mut c: libc::c_char = 0;
        c = inkey();
        match c as libc::c_int {
            27 => { loop_exit = 1 as libc::c_int as bool_ }
            _ => { }
        }
        if loop_exit != 0 { break ; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn status_sight() 
 /* Tell player about ESP, infravision, auto-id, see invis, and similar */
 {
    let mut flag_arr: [[u32b; 7]; 30] =
        [[0; 7]; 30]; /* Schedule leaving the outer loop */
    let mut yo: libc::c_int =
        3 as libc::c_int; /* Leave room for description */
    clear_from(0 as libc::c_int);
    c_put_str(14 as libc::c_int as byte_hack,
              b"Sight\x00" as *const u8 as *const libc::c_char,
              0 as libc::c_int, 1 as libc::c_int);
    az_line(11 as libc::c_int, flag_arr.as_mut_ptr());
    let fresh17 = yo;
    yo = yo + 1;
    row_bival(b"SeeInvis\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x4000 as libc::c_long as u32b, fresh17, flag_arr.as_mut_ptr());
    let fresh18 = yo;
    yo = yo + 1;
    row_npval(b"Invis\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 2 as libc::c_int as s16b,
              0x40 as libc::c_long as u32b, fresh18, flag_arr.as_mut_ptr());
    let fresh19 = yo;
    yo = yo + 1;
    row_npval(b"Infra\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x400 as libc::c_long as u32b, fresh19, flag_arr.as_mut_ptr());
    let fresh20 = yo;
    yo = yo + 1;
    row_npval(b"Search\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x200 as libc::c_long as u32b, fresh20, flag_arr.as_mut_ptr());
    let fresh21 = yo;
    yo = yo + 1;
    row_bival(b"AutoID\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 4 as libc::c_int as s16b,
              0x2000000 as libc::c_long as u32b, fresh21,
              flag_arr.as_mut_ptr());
    let fresh22 = yo;
    yo = yo + 1;
    row_count(b"Light\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x2000 as libc::c_long as u32b, 1 as libc::c_int,
              4 as libc::c_int as s16b, 0x4000000 as libc::c_long as u32b,
              2 as libc::c_int, 4 as libc::c_int as s16b,
              0x8000000 as libc::c_long as u32b, 3 as libc::c_int,
              0 as libc::c_int as s16b, 0 as libc::c_int as u32b,
              0 as libc::c_int, fresh22, flag_arr.as_mut_ptr());
    let fresh23 = yo;
    yo = yo + 1;
    row_bival(b"Full ESP\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 0 as libc::c_int as s16b,
              0x80000000 as libc::c_long as u32b, fresh23,
              flag_arr.as_mut_ptr());
    let fresh24 = yo;
    yo = yo + 1;
    row_bival(b"Orc  ESP\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 0 as libc::c_int as s16b,
              0x1 as libc::c_long as u32b, fresh24, flag_arr.as_mut_ptr());
    let fresh25 = yo;
    yo = yo + 1;
    row_bival(b"Trol ESP\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 0 as libc::c_int as s16b,
              0x2 as libc::c_long as u32b, fresh25, flag_arr.as_mut_ptr());
    let fresh26 = yo;
    yo = yo + 1;
    row_bival(b"Drag ESP\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 0 as libc::c_int as s16b,
              0x4 as libc::c_long as u32b, fresh26, flag_arr.as_mut_ptr());
    let fresh27 = yo;
    yo = yo + 1;
    row_bival(b"GiantESP\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 0 as libc::c_int as s16b,
              0x8 as libc::c_long as u32b, fresh27, flag_arr.as_mut_ptr());
    let fresh28 = yo;
    yo = yo + 1;
    row_bival(b"DemonESP\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 0 as libc::c_int as s16b,
              0x10 as libc::c_long as u32b, fresh28, flag_arr.as_mut_ptr());
    let fresh29 = yo;
    yo = yo + 1;
    row_bival(b"Undd ESP\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 0 as libc::c_int as s16b,
              0x20 as libc::c_long as u32b, fresh29, flag_arr.as_mut_ptr());
    let fresh30 = yo;
    yo = yo + 1;
    row_bival(b"Evil ESP\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 0 as libc::c_int as s16b,
              0x40 as libc::c_long as u32b, fresh30, flag_arr.as_mut_ptr());
    let fresh31 = yo;
    yo = yo + 1;
    row_bival(b"Anim ESP\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 0 as libc::c_int as s16b,
              0x80 as libc::c_long as u32b, fresh31, flag_arr.as_mut_ptr());
    let fresh32 = yo;
    yo = yo + 1;
    row_bival(b"Drid ESP\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 0 as libc::c_int as s16b,
              0x100 as libc::c_long as u32b, fresh32, flag_arr.as_mut_ptr());
    let fresh33 = yo;
    yo = yo + 1;
    row_bival(b"Good ESP\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 0 as libc::c_int as s16b,
              0x200 as libc::c_long as u32b, fresh33, flag_arr.as_mut_ptr());
    let fresh34 = yo;
    yo = yo + 1;
    row_bival(b"SpidrESP\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 0 as libc::c_int as s16b,
              0x1000 as libc::c_long as u32b, fresh34, flag_arr.as_mut_ptr());
    let fresh35 = yo;
    yo = yo + 1;
    row_bival(b"NonlvESP\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 0 as libc::c_int as s16b,
              0x400 as libc::c_long as u32b, fresh35, flag_arr.as_mut_ptr());
    let fresh36 = yo;
    yo = yo + 1;
    row_bival(b"Uniq ESP\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 0 as libc::c_int as s16b,
              0x800 as libc::c_long as u32b, fresh36, flag_arr.as_mut_ptr());
    c_put_str(1 as libc::c_int as byte_hack,
              b"Press ESC to continue\x00" as *const u8 as
                  *const libc::c_char, 23 as libc::c_int, 0 as libc::c_int);
    Term_fresh();
    loop  {
        let mut loop_exit: bool_ = 0 as libc::c_int as bool_;
        let mut c: libc::c_char = 0;
        c = inkey();
        match c as libc::c_int {
            27 => { loop_exit = 1 as libc::c_int as bool_ }
            _ => { }
        }
        if loop_exit != 0 { break ; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn status_item() {
    let mut flag_arr: [[u32b; 7]; 30] = [[0; 7]; 30];
    let mut yo: libc::c_int = 3 as libc::c_int;
    clear_from(0 as libc::c_int);
    c_put_str(14 as libc::c_int as byte_hack,
              b"Misc\x00" as *const u8 as *const libc::c_char,
              0 as libc::c_int, 1 as libc::c_int);
    row_x_start = 40 as libc::c_int;
    az_line(11 as libc::c_int + row_x_start, flag_arr.as_mut_ptr());
    let fresh37 = yo;
    yo = yo + 1;
    row_bival(b"Blessed\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x10000000 as libc::c_long as u32b, fresh37,
              flag_arr.as_mut_ptr());
    let fresh38 = yo;
    yo = yo + 1;
    row_bival(b"Activate\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x1000000 as libc::c_long as u32b, fresh38,
              flag_arr.as_mut_ptr());
    let fresh39 = yo;
    yo = yo + 1;
    row_bival(b"EasyKnow\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x100 as libc::c_long as u32b, fresh39, flag_arr.as_mut_ptr());
    let fresh40 = yo;
    yo = yo + 1;
    row_bival(b"HideType\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x200 as libc::c_long as u32b, fresh40, flag_arr.as_mut_ptr());
    yo += 1;
    let fresh41 = yo;
    yo = yo + 1;
    row_bival(b"SafeAcid\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x100000 as libc::c_long as u32b, fresh41,
              flag_arr.as_mut_ptr());
    let fresh42 = yo;
    yo = yo + 1;
    row_bival(b"SafeElec\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x200000 as libc::c_long as u32b, fresh42,
              flag_arr.as_mut_ptr());
    let fresh43 = yo;
    yo = yo + 1;
    row_bival(b"SafeFire\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x400000 as libc::c_long as u32b, fresh43,
              flag_arr.as_mut_ptr());
    let fresh44 = yo;
    yo = yo + 1;
    row_bival(b"SafeCold\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x800000 as libc::c_long as u32b, fresh44,
              flag_arr.as_mut_ptr());
    let fresh45 = yo;
    yo = yo + 1;
    row_bival(b"ResMorgul\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 5 as libc::c_int as s16b,
              0x1000 as libc::c_long as u32b, fresh45, flag_arr.as_mut_ptr());
    yo = 3 as libc::c_int;
    row_x_start = 0 as libc::c_int;
    az_line(11 as libc::c_int, flag_arr.as_mut_ptr());
    let fresh46 = yo;
    yo = yo + 1;
    row_bival(b"Sh.fire\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x1 as libc::c_long as u32b, fresh46, flag_arr.as_mut_ptr());
    let fresh47 = yo;
    yo = yo + 1;
    row_bival(b"Sh.elec\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x2 as libc::c_long as u32b, fresh47, flag_arr.as_mut_ptr());
    let fresh48 = yo;
    yo = yo + 1;
    row_bival(b"Regen\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x20000 as libc::c_long as u32b, fresh48,
              flag_arr.as_mut_ptr());
    let fresh49 = yo;
    yo = yo + 1;
    row_bival(b"SlowDigest\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x10000 as libc::c_long as u32b, fresh49,
              flag_arr.as_mut_ptr());
    let fresh50 = yo;
    yo = yo + 1;
    row_bival(b"Precog\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 4 as libc::c_int as s16b,
              0x2 as libc::c_long as u32b, fresh50, flag_arr.as_mut_ptr());
    let fresh51 = yo;
    yo = yo + 1;
    row_bival(b"Auto.Id\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 4 as libc::c_int as s16b,
              0x2000000 as libc::c_long as u32b, fresh51,
              flag_arr.as_mut_ptr());
    let fresh52 = yo;
    yo = yo + 1;
    row_bival(b"Spell.In\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 5 as libc::c_int as s16b,
              0x800 as libc::c_long as u32b, fresh52, flag_arr.as_mut_ptr());
    c_put_str(1 as libc::c_int as byte_hack,
              b"Press ESC to continue\x00" as *const u8 as
                  *const libc::c_char, 23 as libc::c_int, 0 as libc::c_int);
    Term_fresh();
    loop  {
        let mut loop_exit: bool_ = 0 as libc::c_int as bool_;
        let mut c: libc::c_char = 0;
        c = inkey();
        match c as libc::c_int {
            27 => { loop_exit = 1 as libc::c_int as bool_ }
            _ => { }
        }
        if loop_exit != 0 { break ; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn status_combat() {
    let mut flag_arr: [[u32b; 7]; 30] = [[0; 7]; 30];
    let mut yo: libc::c_int = 3 as libc::c_int;
    clear_from(0 as libc::c_int);
    c_put_str(14 as libc::c_int as byte_hack,
              b"Combat\x00" as *const u8 as *const libc::c_char,
              0 as libc::c_int, 1 as libc::c_int);
    az_line(11 as libc::c_int, flag_arr.as_mut_ptr());
    let fresh53 = yo;
    yo = yo + 1;
    row_npval(b"Spell\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x80 as libc::c_long as u32b, fresh53, flag_arr.as_mut_ptr());
    let fresh54 = yo;
    yo = yo + 1;
    row_npval(b"Blows\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x2000 as libc::c_long as u32b, fresh54, flag_arr.as_mut_ptr());
    let fresh55 = yo;
    yo = yo + 1;
    row_npval(b"Crits\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 5 as libc::c_int as s16b,
              0x20 as libc::c_long as u32b, fresh55, flag_arr.as_mut_ptr());
    let fresh56 = yo;
    yo = yo + 1;
    row_npval(b"Ammo_Mgt\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x40000 as libc::c_long as u32b, fresh56,
              flag_arr.as_mut_ptr());
    let fresh57 = yo;
    yo = yo + 1;
    row_npval(b"Ammo_Sht\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x80000 as libc::c_long as u32b, fresh57,
              flag_arr.as_mut_ptr());
    let fresh58 = yo;
    yo = yo + 1;
    row_bival(b"Vorpal\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x2000000 as libc::c_long as u32b, fresh58,
              flag_arr.as_mut_ptr());
    let fresh59 = yo;
    yo = yo + 1;
    row_bival(b"Quake\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x4000000 as libc::c_long as u32b, fresh59,
              flag_arr.as_mut_ptr());
    let fresh60 = yo;
    yo = yo + 1;
    row_bival(b"Chaotic\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x4000 as libc::c_long as u32b, fresh60, flag_arr.as_mut_ptr());
    let fresh61 = yo;
    yo = yo + 1;
    row_bival(b"Vampiric\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x8000 as libc::c_long as u32b, fresh61, flag_arr.as_mut_ptr());
    let fresh62 = yo;
    yo = yo + 1;
    row_bival(b"Poison\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x8000000 as libc::c_long as u32b, fresh62,
              flag_arr.as_mut_ptr());
    let fresh63 = yo;
    yo = yo + 1;
    row_bival(b"Acidic\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x10000000 as libc::c_long as u32b, fresh63,
              flag_arr.as_mut_ptr());
    let fresh64 = yo;
    yo = yo + 1;
    row_bival(b"Shocks\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x20000000 as libc::c_long as u32b, fresh64,
              flag_arr.as_mut_ptr());
    let fresh65 = yo;
    yo = yo + 1;
    row_bival(b"Burns\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x40000000 as libc::c_long as u32b, fresh65,
              flag_arr.as_mut_ptr());
    let fresh66 = yo;
    yo = yo + 1;
    row_bival(b"Chills\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x80000000 as libc::c_long as u32b, fresh66,
              flag_arr.as_mut_ptr());
    let fresh67 = yo;
    yo = yo + 1;
    row_bival(b"Wound\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 5 as libc::c_int as s16b,
              0x80 as libc::c_long as u32b, fresh67, flag_arr.as_mut_ptr());
    row_x_start = 40 as libc::c_int;
    yo = 3 as libc::c_int;
    az_line(row_x_start + 11 as libc::c_int, flag_arr.as_mut_ptr());
    let fresh68 = yo;
    yo = yo + 1;
    row_bival(b"No.Blow\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 4 as libc::c_int as s16b,
              0x1 as libc::c_long as u32b, fresh68, flag_arr.as_mut_ptr());
    let fresh69 = yo;
    yo = yo + 1;
    row_trival(b"S/K Undd\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 1 as libc::c_int as s16b,
               0x40000 as libc::c_long as u32b, 5 as libc::c_int as s16b,
               0x10 as libc::c_long as u32b, fresh69, flag_arr.as_mut_ptr());
    let fresh70 = yo;
    yo = yo + 1;
    row_trival(b"S/K Dmn\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 1 as libc::c_int as s16b,
               0x80000 as libc::c_long as u32b, 5 as libc::c_int as s16b,
               0x8 as libc::c_long as u32b, fresh70, flag_arr.as_mut_ptr());
    let fresh71 = yo;
    yo = yo + 1;
    row_trival(b"S/K Drag\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 1 as libc::c_int as s16b,
               0x800000 as libc::c_long as u32b, 1 as libc::c_int as s16b,
               0x1000000 as libc::c_long as u32b, fresh71,
               flag_arr.as_mut_ptr());
    let fresh72 = yo;
    yo = yo + 1;
    row_bival(b"Sl.Orc\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x100000 as libc::c_long as u32b, fresh72,
              flag_arr.as_mut_ptr());
    let fresh73 = yo;
    yo = yo + 1;
    row_bival(b"Sl.Troll\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x200000 as libc::c_long as u32b, fresh73,
              flag_arr.as_mut_ptr());
    let fresh74 = yo;
    yo = yo + 1;
    row_bival(b"Sl.Giant\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x400000 as libc::c_long as u32b, fresh74,
              flag_arr.as_mut_ptr());
    let fresh75 = yo;
    yo = yo + 1;
    row_bival(b"Sl.Evil\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x20000 as libc::c_long as u32b, fresh75,
              flag_arr.as_mut_ptr());
    let fresh76 = yo;
    yo = yo + 1;
    row_bival(b"Sl.Animal\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 1 as libc::c_int as s16b,
              0x10000 as libc::c_long as u32b, fresh76,
              flag_arr.as_mut_ptr());
    let fresh77 = yo;
    yo = yo + 1;
    row_hd_bon(0 as libc::c_int, fresh77, flag_arr.as_mut_ptr());
    let fresh78 = yo;
    yo = yo + 1;
    row_hd_bon(1 as libc::c_int, fresh78, flag_arr.as_mut_ptr());
    row_x_start = 0 as libc::c_int;
    c_put_str(1 as libc::c_int as byte_hack,
              b"Press ESC to continue\x00" as *const u8 as
                  *const libc::c_char, 23 as libc::c_int, 0 as libc::c_int);
    Term_fresh();
    loop  {
        let mut loop_exit: bool_ = 0 as libc::c_int as bool_;
        let mut c: libc::c_char = 0;
        c = inkey();
        match c as libc::c_int {
            27 => { loop_exit = 1 as libc::c_int as bool_ }
            _ => { }
        }
        if loop_exit != 0 { break ; }
    };
}
unsafe extern "C" fn status_curses() {
    let mut flag_arr: [[u32b; 7]; 30] = [[0; 7]; 30];
    let mut yo: libc::c_int = 3 as libc::c_int;
    clear_from(0 as libc::c_int);
    c_put_str(14 as libc::c_int as byte_hack,
              b"Curses\x00" as *const u8 as *const libc::c_char,
              0 as libc::c_int, 1 as libc::c_int);
    az_line(11 as libc::c_int, flag_arr.as_mut_ptr());
    let fresh79 = yo;
    yo = yo + 1;
    row_trival(b"Hvy/Nrm\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 3 as libc::c_int as s16b,
               0x40000000 as libc::c_long as u32b, 3 as libc::c_int as s16b,
               0x20000000 as libc::c_long as u32b, fresh79,
               flag_arr.as_mut_ptr());
    let fresh80 = yo;
    yo = yo + 1;
    row_bival(b"Perma\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x80000000 as libc::c_long as u32b, fresh80,
              flag_arr.as_mut_ptr());
    let fresh81 = yo;
    yo = yo + 1;
    row_trival(b"DG/Ty\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 4 as libc::c_int as s16b,
               0x20 as libc::c_long as u32b, 3 as libc::c_int as s16b,
               0x80 as libc::c_long as u32b, fresh81, flag_arr.as_mut_ptr());
    let fresh82 = yo;
    yo = yo + 1;
    row_trival(b"Prm/Auto\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 3 as libc::c_int as s16b,
               0x80000000 as libc::c_long as u32b, 3 as libc::c_int as s16b,
               0x4 as libc::c_long as u32b, fresh82, flag_arr.as_mut_ptr());
    let fresh83 = yo;
    yo = yo + 1;
    row_bival(b"NoDrop\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 4 as libc::c_int as s16b,
              0x40000000 as libc::c_long as u32b, fresh83,
              flag_arr.as_mut_ptr());
    yo += 1;
    let fresh84 = yo;
    yo = yo + 1;
    row_bival(b"B.Breath\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 4 as libc::c_int as s16b,
              0x4 as libc::c_long as u32b, fresh84, flag_arr.as_mut_ptr());
    let fresh85 = yo;
    yo = yo + 1;
    row_bival(b"Dr.Exp\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x2000000 as libc::c_long as u32b, fresh85,
              flag_arr.as_mut_ptr());
    let fresh86 = yo;
    yo = yo + 1;
    row_bival(b"Dr.Mana\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 5 as libc::c_int as s16b,
              0x2 as libc::c_long as u32b, fresh86, flag_arr.as_mut_ptr());
    let fresh87 = yo;
    yo = yo + 1;
    row_bival(b"Dr.HP\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 5 as libc::c_int as s16b,
              0x4 as libc::c_long as u32b, fresh87, flag_arr.as_mut_ptr());
    let fresh88 = yo;
    yo = yo + 1;
    row_bival(b"No Hit\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 4 as libc::c_int as s16b,
              0x1 as libc::c_long as u32b, fresh88, flag_arr.as_mut_ptr());
    let fresh89 = yo;
    yo = yo + 1;
    row_bival(b"NoTelep\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x10 as libc::c_long as u32b, fresh89, flag_arr.as_mut_ptr());
    let fresh90 = yo;
    yo = yo + 1;
    row_bival(b"NoMagic\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x20 as libc::c_long as u32b, fresh90, flag_arr.as_mut_ptr());
    let fresh91 = yo;
    yo = yo + 1;
    row_bival(b"Aggrav\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 3 as libc::c_int as s16b,
              0x8000000 as libc::c_long as u32b, fresh91,
              flag_arr.as_mut_ptr());
    let fresh92 = yo;
    yo = yo + 1;
    row_bival(b"Clone\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 4 as libc::c_int as s16b,
              0x200 as libc::c_long as u32b, fresh92, flag_arr.as_mut_ptr());
    let fresh93 = yo;
    yo = yo + 1;
    row_bival(b"Temp\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 5 as libc::c_int as s16b,
              0x1 as libc::c_long as u32b, fresh93, flag_arr.as_mut_ptr());
    yo += 1;
    let fresh94 = yo;
    yo = yo + 1;
    row_count(b"Antimagic\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 4 as libc::c_int as s16b,
              0x20000 as libc::c_long as u32b, 5 as libc::c_int,
              4 as libc::c_int as s16b, 0x40000 as libc::c_long as u32b,
              3 as libc::c_int, 4 as libc::c_int as s16b,
              0x80000 as libc::c_long as u32b, 2 as libc::c_int,
              4 as libc::c_int as s16b, 0x100000 as libc::c_long as u32b,
              1 as libc::c_int, fresh94, flag_arr.as_mut_ptr());
    c_put_str(1 as libc::c_int as byte_hack,
              b"Press ESC to continue\x00" as *const u8 as
                  *const libc::c_char, 23 as libc::c_int, 0 as libc::c_int);
    Term_fresh();
    loop  {
        let mut loop_exit: bool_ = 0 as libc::c_int as bool_;
        let mut c: libc::c_char = 0;
        c = inkey();
        match c as libc::c_int {
            27 => { loop_exit = 1 as libc::c_int as bool_ }
            _ => { }
        }
        if loop_exit as libc::c_int == 1 as libc::c_int { break ; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn status_res() {
    let mut flag_arr: [[u32b; 7]; 30] = [[0; 7]; 30];
    let mut yo: libc::c_int = 3 as libc::c_int;
    clear_from(0 as libc::c_int);
    c_put_str(14 as libc::c_int as byte_hack,
              b"Resistances\x00" as *const u8 as *const libc::c_char,
              0 as libc::c_int, 1 as libc::c_int);
    az_line(11 as libc::c_int, flag_arr.as_mut_ptr());
    let fresh95 = yo;
    yo = yo + 1;
    row_trival(b"Fire\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 2 as libc::c_int as s16b,
               0x400 as libc::c_long as u32b, 2 as libc::c_int as s16b,
               0x40000 as libc::c_long as u32b, fresh95,
               flag_arr.as_mut_ptr());
    let fresh96 = yo;
    yo = yo + 1;
    row_trival(b"Cold\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 2 as libc::c_int as s16b,
               0x800 as libc::c_long as u32b, 2 as libc::c_int as s16b,
               0x80000 as libc::c_long as u32b, fresh96,
               flag_arr.as_mut_ptr());
    let fresh97 = yo;
    yo = yo + 1;
    row_trival(b"Acid\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 2 as libc::c_int as s16b,
               0x100 as libc::c_long as u32b, 2 as libc::c_int as s16b,
               0x10000 as libc::c_long as u32b, fresh97,
               flag_arr.as_mut_ptr());
    let fresh98 = yo;
    yo = yo + 1;
    row_trival(b"Lightning\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 2 as libc::c_int as s16b,
               0x200 as libc::c_long as u32b, 2 as libc::c_int as s16b,
               0x20000 as libc::c_long as u32b, fresh98,
               flag_arr.as_mut_ptr());
    let fresh99 = yo;
    yo = yo + 1;
    row_bival(b"Poison\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 2 as libc::c_int as s16b,
              0x100000 as libc::c_long as u32b, fresh99,
              flag_arr.as_mut_ptr());
    let fresh100 = yo;
    yo = yo + 1;
    row_bival(b"Lite\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 2 as libc::c_int as s16b,
              0x400000 as libc::c_long as u32b, fresh100,
              flag_arr.as_mut_ptr());
    let fresh101 = yo;
    yo = yo + 1;
    row_bival(b"Dark\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 2 as libc::c_int as s16b,
              0x800000 as libc::c_long as u32b, fresh101,
              flag_arr.as_mut_ptr());
    let fresh102 = yo;
    yo = yo + 1;
    row_bival(b"Sound\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 2 as libc::c_int as s16b,
              0x4000000 as libc::c_long as u32b, fresh102,
              flag_arr.as_mut_ptr());
    let fresh103 = yo;
    yo = yo + 1;
    row_bival(b"Shards\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 2 as libc::c_int as s16b,
              0x8000000 as libc::c_long as u32b, fresh103,
              flag_arr.as_mut_ptr());
    let fresh104 = yo;
    yo = yo + 1;
    row_trival(b"Nether\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 4 as libc::c_int as s16b,
               0x400000 as libc::c_long as u32b, 2 as libc::c_int as s16b,
               0x10000000 as libc::c_long as u32b, fresh104,
               flag_arr.as_mut_ptr());
    let fresh105 = yo;
    yo = yo + 1;
    row_bival(b"Nexus\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 2 as libc::c_int as s16b,
              0x20000000 as libc::c_long as u32b, fresh105,
              flag_arr.as_mut_ptr());
    let fresh106 = yo;
    yo = yo + 1;
    row_bival(b"Chaos\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 2 as libc::c_int as s16b,
              0x40000000 as libc::c_long as u32b, fresh106,
              flag_arr.as_mut_ptr());
    let fresh107 = yo;
    yo = yo + 1;
    row_bival(b"Disen.\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 2 as libc::c_int as s16b,
              0x80000000 as libc::c_long as u32b, fresh107,
              flag_arr.as_mut_ptr());
    let fresh108 = yo;
    yo = yo + 1;
    row_bival(b"Confusion\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 2 as libc::c_int as s16b,
              0x2000000 as libc::c_long as u32b, fresh108,
              flag_arr.as_mut_ptr());
    let fresh109 = yo;
    yo = yo + 1;
    row_bival(b"Blindness\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 2 as libc::c_int as s16b,
              0x1000000 as libc::c_long as u32b, fresh109,
              flag_arr.as_mut_ptr());
    let fresh110 = yo;
    yo = yo + 1;
    row_bival(b"Fear\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 2 as libc::c_int as s16b,
              0x200000 as libc::c_long as u32b, fresh110,
              flag_arr.as_mut_ptr());
    let fresh111 = yo;
    yo = yo + 1;
    row_bival(b"Free Act\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 2 as libc::c_int as s16b,
              0x4000 as libc::c_long as u32b, fresh111,
              flag_arr.as_mut_ptr());
    let fresh112 = yo;
    yo = yo + 1;
    row_bival(b"Reflect\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 2 as libc::c_int as s16b,
              0x2000 as libc::c_long as u32b, fresh112,
              flag_arr.as_mut_ptr());
    let fresh113 = yo;
    yo = yo + 1;
    row_bival(b"Hold Life\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 2 as libc::c_int as s16b,
              0x8000 as libc::c_long as u32b, fresh113,
              flag_arr.as_mut_ptr());
    c_put_str(1 as libc::c_int as byte_hack,
              b"Press ESC to continue\x00" as *const u8 as
                  *const libc::c_char, 23 as libc::c_int, 0 as libc::c_int);
    Term_fresh();
    loop  {
        let mut loop_exit: bool_ = 0 as libc::c_int as bool_;
        let mut c: libc::c_char = 0;
        c = inkey();
        match c as libc::c_int {
            27 => { loop_exit = 1 as libc::c_int as bool_ }
            _ => { }
        }
        if loop_exit as libc::c_int == 1 as libc::c_int { break ; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn status_main() {
    let mut do_quit: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_char = 0;
    character_icky = 1 as libc::c_int as bool_;
    Term_save();
    loop  {
        clear_from(0 as libc::c_int);
        c_put_str(1 as libc::c_int as byte_hack,
                  format(b"%s Character Status screen\x00" as *const u8 as
                             *const libc::c_char, game_module) as cptr,
                  0 as libc::c_int, 10 as libc::c_int);
        c_put_str(1 as libc::c_int as byte_hack,
                  b"1) Statistics\x00" as *const u8 as *const libc::c_char,
                  2 as libc::c_int, 5 as libc::c_int);
        c_put_str(1 as libc::c_int as byte_hack,
                  b"2) Movement\x00" as *const u8 as *const libc::c_char,
                  3 as libc::c_int, 5 as libc::c_int);
        c_put_str(1 as libc::c_int as byte_hack,
                  b"3) Combat\x00" as *const u8 as *const libc::c_char,
                  4 as libc::c_int, 5 as libc::c_int);
        c_put_str(1 as libc::c_int as byte_hack,
                  b"4) Resistances\x00" as *const u8 as *const libc::c_char,
                  5 as libc::c_int, 5 as libc::c_int);
        c_put_str(1 as libc::c_int as byte_hack,
                  b"5) Misc\x00" as *const u8 as *const libc::c_char,
                  6 as libc::c_int, 5 as libc::c_int);
        c_put_str(1 as libc::c_int as byte_hack,
                  b"6) Curses\x00" as *const u8 as *const libc::c_char,
                  7 as libc::c_int, 5 as libc::c_int);
        c_put_str(1 as libc::c_int as byte_hack,
                  b"7) Sight\x00" as *const u8 as *const libc::c_char,
                  8 as libc::c_int, 5 as libc::c_int);
        c_put_str(1 as libc::c_int as byte_hack,
                  b"8) Companions\x00" as *const u8 as *const libc::c_char,
                  9 as libc::c_int, 5 as libc::c_int);
        c_put_str(4 as libc::c_int as byte_hack,
                  b"Press \'q\' to Quit\x00" as *const u8 as
                      *const libc::c_char, 23 as libc::c_int,
                  5 as libc::c_int);
        c = inkey();
        match c as libc::c_int {
            49 => { status_attr(); }
            50 => { status_move(); }
            51 => { status_combat(); }
            52 => { status_res(); }
            53 => { status_item(); }
            54 => { status_curses(); }
            55 => { status_sight(); }
            56 => { status_companion(); }
            113 | 27 => { do_quit = 1 as libc::c_int }
            _ => { }
        }
        Term_fresh();
        if do_quit != 0 { break ; }
    }
    Term_load();
    character_icky = 0 as libc::c_int as bool_;
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long |
             (0x8000000 as libc::c_long | 0x2000000 as libc::c_long |
                  0x1000000 as libc::c_long | 0x4000000 as libc::c_long)) as
            u32b;
    handle_stuff();
}
#[no_mangle]
pub unsafe extern "C" fn az_line(mut xo: libc::c_int,
                                 mut flag_arr: *mut [u32b; 7]) {
    let mut index: libc::c_int = xo;
    let mut i: libc::c_int = 0;
    i = 24 as libc::c_int;
    while i < 52 as libc::c_int {
        if (*p_ptr).inventory[i as usize].k_idx != 0 {
            /* Wearing anything here? */
            let mut cstrng: [libc::c_char; 2] = [0; 2];
            /* And mark it to display */
            cstrng[0 as libc::c_int as usize] =
                (i - 24 as libc::c_int + 'a' as i32) as
                    libc::c_char; /* Assumes ASCII */
            cstrng[1 as libc::c_int as usize] =
                '\u{0}' as i32 as libc::c_char; /* terminate it */
            let fresh114 = index; /* Assumes ASCII */
            index = index + 1;
            c_put_str(1 as libc::c_int as byte_hack,
                      cstrng.as_mut_ptr() as cptr, 2 as libc::c_int,
                      fresh114);
            object_flags(&mut *(*p_ptr).inventory.as_mut_ptr().offset(i as
                                                                          isize),
                         &mut *(*flag_arr.offset((i - 24 as libc::c_int) as
                                                     isize)).as_mut_ptr().offset(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize),
                         &mut *(*flag_arr.offset((i - 24 as libc::c_int) as
                                                     isize)).as_mut_ptr().offset(2
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize),
                         &mut *(*flag_arr.offset((i - 24 as libc::c_int) as
                                                     isize)).as_mut_ptr().offset(3
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize),
                         &mut *(*flag_arr.offset((i - 24 as libc::c_int) as
                                                     isize)).as_mut_ptr().offset(4
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize),
                         &mut *(*flag_arr.offset((i - 24 as libc::c_int) as
                                                     isize)).as_mut_ptr().offset(5
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize),
                         &mut *(*flag_arr.offset((i - 24 as libc::c_int) as
                                                     isize)).as_mut_ptr().offset(0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize));
            (*flag_arr.offset((i - 24 as libc::c_int) as
                                  isize))[6 as libc::c_int as usize] =
                1 as libc::c_int as u32b
        } else {
            (*flag_arr.offset((i - 24 as libc::c_int) as
                                  isize))[6 as libc::c_int as usize] =
                0 as libc::c_int as u32b
        }
        i += 1
        /* DGDGDGDG */
			/*                        object_flags_known(&inventory[i],*/
        /* esp */
        /* Otherwise don't display it */
    } /* default */
    let fresh115 = index;
    index = index + 1;
    c_put_str(1 as libc::c_int as byte_hack,
              b"@\x00" as *const u8 as *const libc::c_char, 2 as libc::c_int,
              fresh115);
    player_flags(&mut *(*flag_arr.offset((52 as libc::c_int -
                                              24 as libc::c_int +
                                              1 as libc::c_int) as
                                             isize)).as_mut_ptr().offset(1 as
                                                                             libc::c_int
                                                                             as
                                                                             isize),
                 &mut *(*flag_arr.offset((52 as libc::c_int -
                                              24 as libc::c_int +
                                              1 as libc::c_int) as
                                             isize)).as_mut_ptr().offset(2 as
                                                                             libc::c_int
                                                                             as
                                                                             isize),
                 &mut *(*flag_arr.offset((52 as libc::c_int -
                                              24 as libc::c_int +
                                              1 as libc::c_int) as
                                             isize)).as_mut_ptr().offset(3 as
                                                                             libc::c_int
                                                                             as
                                                                             isize),
                 &mut *(*flag_arr.offset((52 as libc::c_int -
                                              24 as libc::c_int +
                                              1 as libc::c_int) as
                                             isize)).as_mut_ptr().offset(4 as
                                                                             libc::c_int
                                                                             as
                                                                             isize),
                 &mut *(*flag_arr.offset((52 as libc::c_int -
                                              24 as libc::c_int +
                                              1 as libc::c_int) as
                                             isize)).as_mut_ptr().offset(5 as
                                                                             libc::c_int
                                                                             as
                                                                             isize),
                 &mut *(*flag_arr.offset((52 as libc::c_int -
                                              24 as libc::c_int +
                                              1 as libc::c_int) as
                                             isize)).as_mut_ptr().offset(0 as
                                                                             libc::c_int
                                                                             as
                                                                             isize));
    (*flag_arr.offset((52 as libc::c_int - 24 as libc::c_int +
                           1 as libc::c_int) as
                          isize))[6 as libc::c_int as usize] =
        1 as libc::c_int as u32b;
}
unsafe extern "C" fn status_trival(mut val1: s32b, mut val2: s32b,
                                   mut ypos: byte_hack, mut xpos: byte_hack) {
    if val1 != 0 as libc::c_int {
        c_put_str(14 as libc::c_int as byte_hack,
                  b"*\x00" as *const u8 as *const libc::c_char,
                  ypos as libc::c_int, xpos as libc::c_int);
    } else if val2 != 0 as libc::c_int {
        c_put_str(14 as libc::c_int as byte_hack,
                  b"+\x00" as *const u8 as *const libc::c_char,
                  ypos as libc::c_int, xpos as libc::c_int);
    } else {
        c_put_str(1 as libc::c_int as byte_hack,
                  b".\x00" as *const u8 as *const libc::c_char,
                  ypos as libc::c_int, xpos as libc::c_int);
    };
}
unsafe extern "C" fn status_bival(mut val: s32b, mut ypos: byte_hack,
                                  mut xpos: byte_hack) {
    if val != 0 as libc::c_int {
        c_put_str(14 as libc::c_int as byte_hack,
                  b"+\x00" as *const u8 as *const libc::c_char,
                  ypos as libc::c_int, xpos as libc::c_int);
    } else {
        c_put_str(1 as libc::c_int as byte_hack,
                  b".\x00" as *const u8 as *const libc::c_char,
                  ypos as libc::c_int, xpos as libc::c_int);
    };
}
unsafe extern "C" fn status_numeric(mut val: s32b, mut ypos: byte_hack,
                                    mut xpos: byte_hack) {
    let mut magnitude: u32b =
        if val < 0 as libc::c_int { -val } else { val } as u32b;
    let mut color: libc::c_int = 1 as libc::c_int;
    let mut strnum: [libc::c_char; 2] = [0; 2];
    if val < 0 as libc::c_int { color = 4 as libc::c_int }
    if val > 0 as libc::c_int { color = 5 as libc::c_int }
    if magnitude == 0 as libc::c_int as libc::c_uint {
        sprintf(strnum.as_mut_ptr(),
                b".\x00" as *const u8 as *const libc::c_char);
    }
    if magnitude > 9 as libc::c_int as libc::c_uint {
        sprintf(strnum.as_mut_ptr(),
                b"*\x00" as *const u8 as *const libc::c_char);
    } else {
        sprintf(strnum.as_mut_ptr(),
                b"%lu\x00" as *const u8 as *const libc::c_char,
                magnitude as libc::c_ulong);
    }
    c_put_str(color as byte_hack, strnum.as_mut_ptr() as cptr,
              ypos as libc::c_int, xpos as libc::c_int);
}
unsafe extern "C" fn status_count(mut val1: s32b, mut v1: libc::c_int,
                                  mut val2: s32b, mut v2: libc::c_int,
                                  mut val3: s32b, mut v3: libc::c_int,
                                  mut val4: s32b, mut v4: libc::c_int,
                                  mut ypos: byte_hack, mut xpos: byte_hack) {
    let mut v: libc::c_int = 0 as libc::c_int;
    if val1 != 0 as libc::c_int { v += v1 }
    if val2 != 0 as libc::c_int { v += v2 }
    if val3 != 0 as libc::c_int { v += v3 }
    if val4 != 0 as libc::c_int { v += v4 }
    status_numeric(v, ypos, xpos);
}
unsafe extern "C" fn row_count(mut statname: *mut libc::c_char,
                               mut row1: s16b, mut flag1: u32b,
                               mut v1: libc::c_int, mut row2: s16b,
                               mut flag2: u32b, mut v2: libc::c_int,
                               mut row3: s16b, mut flag3: u32b,
                               mut v3: libc::c_int, mut row4: s16b,
                               mut flag4: u32b, mut v4: libc::c_int,
                               mut yo: libc::c_int,
                               mut flag_arr: *mut [u32b; 7]) {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = row_x_start;
    c_put_str(13 as libc::c_int as byte_hack, statname as cptr, yo,
              row_x_start);
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int - 24 as libc::c_int + 2 as libc::c_int {
        if (*flag_arr.offset(i as isize))[6 as libc::c_int as usize] ==
               1 as libc::c_int as libc::c_uint {
            status_count(((*flag_arr.offset(i as isize))[row1 as usize] &
                              flag1) as s32b, v1,
                         ((*flag_arr.offset(i as isize))[row2 as usize] &
                              flag2) as s32b, v2,
                         ((*flag_arr.offset(i as isize))[row3 as usize] &
                              flag3) as s32b, v3,
                         ((*flag_arr.offset(i as isize))[row4 as usize] &
                              flag4) as s32b, v4, yo as byte_hack,
                         (x + 11 as libc::c_int) as byte_hack);
            x += 1
        }
        i += 1
    };
}
/* File status.c */
/* Purpose: Status information */
/* Written by Pat Gunn <qc@apk.net> for ToME
 * This file is released into the public domain
 */
/* Throughout this file, I make use of 2-d arrays called flag_arr.
 * I fill them out with esp as the 0th element because then the second
 * index is equal to the f-number that they are in the attributes, and
 * that makes it more intuitive to use. I do need to fill them out in an
 * odd order, but that's all abstracted nicely away. The 6th element is used
 * to mark if the rest of the array is filled, that is, if there's an object
 * there.
 */
unsafe extern "C" fn row_trival(mut statname: *mut libc::c_char,
                                mut row: s16b, mut flag: u32b, mut row2: s16b,
                                mut flag2: u32b, mut yo: libc::c_int,
                                mut flag_arr: *mut [u32b; 7]) {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = row_x_start;
    c_put_str(13 as libc::c_int as byte_hack, statname as cptr, yo,
              row_x_start);
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int - 24 as libc::c_int + 2 as libc::c_int {
        if (*flag_arr.offset(i as isize))[6 as libc::c_int as usize] ==
               1 as libc::c_int as libc::c_uint {
            status_trival(((*flag_arr.offset(i as isize))[row as usize] &
                               flag) as s32b,
                          ((*flag_arr.offset(i as isize))[row2 as usize] &
                               flag2) as s32b, yo as byte_hack,
                          (x + 11 as libc::c_int) as byte_hack);
            x += 1
        }
        i += 1
    };
}
unsafe extern "C" fn row_bival(mut statname: *mut libc::c_char, mut row: s16b,
                               mut flag: u32b, mut yo: libc::c_int,
                               mut flag_arr: *mut [u32b; 7]) {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = row_x_start;
    c_put_str(13 as libc::c_int as byte_hack, statname as cptr, yo,
              row_x_start);
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int - 24 as libc::c_int + 2 as libc::c_int {
        if (*flag_arr.offset(i as isize))[6 as libc::c_int as usize] ==
               1 as libc::c_int as libc::c_uint {
            status_bival(((*flag_arr.offset(i as isize))[row as usize] & flag)
                             as s32b, yo as byte_hack,
                         (x + 11 as libc::c_int) as byte_hack);
            x += 1
        }
        i += 1
    };
}
unsafe extern "C" fn row_npval(mut statname: *mut libc::c_char, mut row: s16b,
                               mut flag: u32b, mut yo: libc::c_int,
                               mut flag_arr: *mut [u32b; 7]) 
 /* Displays nicely a pval-based status row */
 {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = row_x_start;
    c_put_str(13 as libc::c_int as byte_hack, statname as cptr, yo,
              row_x_start);
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int - 24 as libc::c_int + 2 as libc::c_int {
        if (*flag_arr.offset(i as isize))[6 as libc::c_int as usize] ==
               1 as libc::c_int as libc::c_uint {
            if i == 52 as libc::c_int - 24 as libc::c_int + 1 as libc::c_int {
                /* Special case, player_flags */
				/* Players lack a pval, no way to calc value */
                if (*flag_arr.offset(i as isize))[row as usize] & flag != 0 {
                    c_put_str(11 as libc::c_int as byte_hack,
                              b"*\x00" as *const u8 as *const libc::c_char,
                              yo, x + 11 as libc::c_int);
                } else {
                    c_put_str(1 as libc::c_int as byte_hack,
                              b".\x00" as *const u8 as *const libc::c_char,
                              yo, x + 11 as libc::c_int);
                }
                x += 1
            } else {
                if (*flag_arr.offset(i as isize))[row as usize] & flag != 0 {
                    status_numeric((*p_ptr).inventory[(i + 24 as libc::c_int)
                                                          as usize].pval,
                                   yo as byte_hack,
                                   (x + 11 as libc::c_int) as byte_hack);
                } else {
                    c_put_str(1 as libc::c_int as byte_hack,
                              b".\x00" as *const u8 as *const libc::c_char,
                              yo, x + 11 as libc::c_int);
                }
                x += 1
            }
        }
        i += 1
    };
}
unsafe extern "C" fn statline(mut statname: *mut libc::c_char,
                              mut statidx: libc::c_int, mut flag: u32b,
                              mut yo: libc::c_int,
                              mut flag_arr: *mut [u32b; 7]) 
 /* Displays a status row for a primary stat */
 {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = row_x_start;
    let mut statstr: [libc::c_char; 8] = [0; 8];
    let mut stat_color: byte_hack = 12 as libc::c_int as byte_hack;
    cnv_stat((*p_ptr).stat_use[statidx as usize] as libc::c_int,
             statstr.as_mut_ptr());
    c_put_str(13 as libc::c_int as byte_hack, statstr.as_mut_ptr() as cptr,
              yo, 4 as libc::c_int + row_x_start);
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int - 24 as libc::c_int + 2 as libc::c_int {
        let mut color: byte_hack = 12 as libc::c_int as byte_hack;
        if (*flag_arr.offset(i as isize))[6 as libc::c_int as usize] ==
               1 as libc::c_int as libc::c_uint {
            match statidx {
                0 => {
                    if (*flag_arr.offset(i as
                                             isize))[2 as libc::c_int as
                                                         usize] as
                           libc::c_long & 0x1 as libc::c_long != 0 {
                        color = 14 as libc::c_int as byte_hack
                    }
                }
                1 => {
                    if (*flag_arr.offset(i as
                                             isize))[2 as libc::c_int as
                                                         usize] as
                           libc::c_long & 0x2 as libc::c_long != 0 {
                        color = 14 as libc::c_int as byte_hack
                    }
                }
                2 => {
                    if (*flag_arr.offset(i as
                                             isize))[2 as libc::c_int as
                                                         usize] as
                           libc::c_long & 0x4 as libc::c_long != 0 {
                        color = 14 as libc::c_int as byte_hack
                    }
                }
                3 => {
                    if (*flag_arr.offset(i as
                                             isize))[2 as libc::c_int as
                                                         usize] as
                           libc::c_long & 0x8 as libc::c_long != 0 {
                        color = 14 as libc::c_int as byte_hack
                    }
                }
                4 => {
                    if (*flag_arr.offset(i as
                                             isize))[2 as libc::c_int as
                                                         usize] as
                           libc::c_long & 0x10 as libc::c_long != 0 {
                        color = 14 as libc::c_int as byte_hack
                    }
                }
                5 => {
                    if (*flag_arr.offset(i as
                                             isize))[2 as libc::c_int as
                                                         usize] as
                           libc::c_long & 0x20 as libc::c_long != 0 {
                        color = 14 as libc::c_int as byte_hack
                    }
                }
                _ => { }
            }
            if i == 52 as libc::c_int - 24 as libc::c_int + 1 as libc::c_int {
                /* Player flags */
                if (*flag_arr.offset(i as isize))[1 as libc::c_int as usize] &
                       flag != 0 {
                    c_put_str(if color as libc::c_int == 12 as libc::c_int {
                                  11 as libc::c_int
                              } else { color as libc::c_int } as byte_hack,
                              b"*\x00" as *const u8 as *const libc::c_char,
                              yo, x + 11 as libc::c_int);
                } else {
                    c_put_str(if color as libc::c_int == 12 as libc::c_int {
                                  1 as libc::c_int
                              } else { color as libc::c_int } as byte_hack,
                              b".\x00" as *const u8 as *const libc::c_char,
                              yo, x + 11 as libc::c_int);
                }
                x += 1
            } else {
                if (*flag_arr.offset(i as isize))[1 as libc::c_int as usize] &
                       flag != 0 {
                    status_numeric((*p_ptr).inventory[(i + 24 as libc::c_int)
                                                          as usize].pval,
                                   yo as byte_hack,
                                   (x + 11 as libc::c_int) as byte_hack);
                } else {
                    c_put_str(if color as libc::c_int == 12 as libc::c_int {
                                  1 as libc::c_int
                              } else { color as libc::c_int } as byte_hack,
                              b".\x00" as *const u8 as *const libc::c_char,
                              yo, x + 11 as libc::c_int);
                }
                if color as libc::c_int != 12 as libc::c_int {
                    stat_color = color
                }
                x += 1
            }
        }
        i += 1
    }
    c_put_str(stat_color, statname as cptr, yo, row_x_start);
}
unsafe extern "C" fn row_hd_bon(mut which: libc::c_int, mut yo: libc::c_int,
                                mut flag_arr: *mut [u32b; 7]) 
 /* To-hit/dmg modifiers, selected by 1st argument */
 {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = row_x_start;
    if which != 0 as libc::c_int && which != 1 as libc::c_int { return }
    c_put_str(13 as libc::c_int as byte_hack,
              if which == 0 as libc::c_int {
                  b"To-Hit\x00" as *const u8 as *const libc::c_char
              } else { b"To-Dmg\x00" as *const u8 as *const libc::c_char },
              yo, row_x_start);
    i = 0 as libc::c_int;
    while i < 52 as libc::c_int - 24 as libc::c_int + 2 as libc::c_int {
        if (*flag_arr.offset(i as isize))[6 as libc::c_int as usize] ==
               1 as libc::c_int as libc::c_uint {
            if i == 52 as libc::c_int - 24 as libc::c_int + 1 as libc::c_int {
                /* Player? */
                c_put_str(1 as libc::c_int as byte_hack,
                          b".\x00" as *const u8 as *const libc::c_char, yo,
                          x + 11 as libc::c_int);
                x += 1
            } else if which == 0 as libc::c_int &&
                          (*p_ptr).inventory[(24 as libc::c_int + i) as
                                                 usize].to_h as libc::c_int !=
                              0 as libc::c_int {
                status_numeric((*p_ptr).inventory[(24 as libc::c_int + i) as
                                                      usize].to_h as s32b,
                               yo as byte_hack,
                               (x + 11 as libc::c_int) as byte_hack);
                x += 1
            } else if which == 1 as libc::c_int &&
                          (*p_ptr).inventory[(24 as libc::c_int + i) as
                                                 usize].to_d as libc::c_int !=
                              0 as libc::c_int {
                status_numeric((*p_ptr).inventory[(24 as libc::c_int + i) as
                                                      usize].to_d as s32b,
                               yo as byte_hack,
                               (x + 11 as libc::c_int) as byte_hack);
                x += 1
            } else {
                c_put_str(1 as libc::c_int as byte_hack,
                          b".\x00" as *const u8 as *const libc::c_char, yo,
                          x + 11 as libc::c_int);
                x += 1
            }
        }
        i += 1
    };
}
unsafe extern "C" fn status_companion() {
    let mut i: libc::c_int = 0;
    let mut fff: *mut FILE = 0 as *mut FILE;
    let mut file_name: [libc::c_char; 1024] = [0; 1024];
    Term_clear();
    /* Temporary file */
    if path_temp(file_name.as_mut_ptr(), 1024 as libc::c_int) != 0 { return }
    /* Open a new file */
    fff =
        my_fopen(file_name.as_mut_ptr() as cptr,
                 b"w\x00" as *const u8 as *const libc::c_char);
    /* Calculate companions */
	/* Process the monsters (backwards) */
    i = m_max as libc::c_int - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        /* Access the monster */
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        if (*m_ptr).status as libc::c_int == 4 as libc::c_int {
            let mut m_name: [libc::c_char; 80] = [0; 80];
            let mut b: libc::c_int = 0;
            let mut y: libc::c_int = 0 as libc::c_int;
            /* Extract monster name */
            monster_desc(m_name.as_mut_ptr(), m_ptr, 0x80 as libc::c_int);
            fprintf(fff,
                    b"#####BCompanion: %s\n\x00" as *const u8 as
                        *const libc::c_char, m_name.as_mut_ptr());
            fprintf(fff,
                    b"  Lev/Exp : [[[[[G%d / %ld]\n\x00" as *const u8 as
                        *const libc::c_char, (*m_ptr).level as libc::c_int,
                    (*m_ptr).exp as libc::c_long);
            if ((*m_ptr).level as libc::c_int) < 150 as libc::c_int {
                fprintf(fff,
                        b"  Next lvl: [[[[[G%ld]\n\x00" as *const u8 as
                            *const libc::c_char,
                        ((if (*m_ptr).level as s32b + 1 as libc::c_int >
                                 150 as libc::c_int {
                              150 as libc::c_int
                          } else {
                              ((*m_ptr).level as s32b) + 1 as libc::c_int
                          }) *
                             (if (*m_ptr).level as s32b + 1 as libc::c_int >
                                     150 as libc::c_int {
                                  150 as libc::c_int
                              } else {
                                  ((*m_ptr).level as s32b) + 1 as libc::c_int
                              }) *
                             (if (*m_ptr).level as s32b + 1 as libc::c_int >
                                     150 as libc::c_int {
                                  150 as libc::c_int
                              } else {
                                  ((*m_ptr).level as s32b) + 1 as libc::c_int
                              }) * 6 as libc::c_int) as libc::c_long);
            } else {
                fprintf(fff,
                        b"  Next lvl: [[[[[G****]\n\x00" as *const u8 as
                            *const libc::c_char);
            }
            fprintf(fff,
                    b"  HP      : [[[[[G%ld / %ld]\n\x00" as *const u8 as
                        *const libc::c_char, (*m_ptr).hp as libc::c_long,
                    (*m_ptr).maxhp as libc::c_long);
            fprintf(fff,
                    b"  AC      : [[[[[G%d]\n\x00" as *const u8 as
                        *const libc::c_char, (*m_ptr).ac as libc::c_int);
            fprintf(fff,
                    b"  Speed   : [[[[[G%d]\n\x00" as *const u8 as
                        *const libc::c_char,
                    (*m_ptr).mspeed as libc::c_int - 110 as libc::c_int);
            b = 0 as libc::c_int;
            while b < 4 as libc::c_int {
                if !((*m_ptr).blow[b as usize].d_dice == 0) {
                    if !((*m_ptr).blow[b as usize].d_side == 0) {
                        fprintf(fff,
                                b"  Blow %1d  : [[[[[G%dd%d]\n\x00" as
                                    *const u8 as *const libc::c_char,
                                y + 1 as libc::c_int,
                                (*m_ptr).blow[b as usize].d_dice as
                                    libc::c_int,
                                (*m_ptr).blow[b as usize].d_side as
                                    libc::c_int);
                        y += 1
                    }
                }
                b += 1
            }
            fprintf(fff, b"\n\x00" as *const u8 as *const libc::c_char);
        }
        i -= 1
    }
    /* Close the file */
    my_fclose(fff);
    /* Display the file contents */
    show_file(file_name.as_mut_ptr() as cptr,
              b"Companion List\x00" as *const u8 as *const libc::c_char,
              0 as libc::c_int, 0 as libc::c_int);
    /* Remove the file */
    fd_kill(file_name.as_mut_ptr() as cptr);
}
