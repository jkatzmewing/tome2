use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut cur_hgt: s16b;
    #[no_mangle]
    static mut cur_wid: s16b;
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut object_level: s16b;
    #[no_mangle]
    static mut monster_level: s16b;
    #[no_mangle]
    static mut turn: s32b;
    #[no_mangle]
    static mut wizard: bool_;
    #[no_mangle]
    static mut view_perma_grids: bool_;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn ralloc(len: huge_hack) -> vptr;
    #[no_mangle]
    static mut Rand_quick: bool_;
    #[no_mangle]
    static mut Rand_value: u32b;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    static mut floor_type: [s16b; 100];
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut m_list: *mut monster_type;
    #[no_mangle]
    static mut town_info: *mut town_type;
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut f_info: *mut feature_type;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut wf_info: *mut wilderness_type_info;
    #[no_mangle]
    static mut st_info: *mut store_info_type;
    #[no_mangle]
    static mut get_mon_num_hook:
           Option<unsafe extern "C" fn(_: libc::c_int) -> bool_>;
    #[no_mangle]
    static mut max_wild_x: u16b;
    #[no_mangle]
    static mut max_wild_y: u16b;
    #[no_mangle]
    static mut wild_map: *mut *mut wilderness_map;
    #[no_mangle]
    static mut max_st_idx: u16b;
    #[no_mangle]
    static mut init_flags: libc::c_int;
    #[no_mangle]
    static mut ambush_flag: bool_;
    #[no_mangle]
    static mut generate_encounter: bool_;
    #[no_mangle]
    static mut m_allow_special: *mut bool_;
    #[no_mangle]
    static mut max_q_idx: s16b;
    #[no_mangle]
    static mut quest: *mut quest_type;
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn distance(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int,
                x2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lite_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn cave_set_feat(y: libc::c_int, x: libc::c_int, feat: libc::c_int);
    #[no_mangle]
    fn process_dungeon_file(name: cptr, yval: *mut libc::c_int,
                            xval: *mut libc::c_int, ymax: libc::c_int,
                            xmax: libc::c_int, init: bool_, full: bool_)
     -> errr;
    #[no_mangle]
    fn alloc_monster(dis: libc::c_int, slp: bool_) -> bool_;
    #[no_mangle]
    fn player_place(y: libc::c_int, x: libc::c_int) -> s16b;
    #[no_mangle]
    fn get_mon_num_prep() -> errr;
    #[no_mangle]
    fn set_mon_num_hook();
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn monster_check_experience(m_idx: libc::c_int, silent: bool_);
    #[no_mangle]
    fn place_monster_one(y: libc::c_int, x: libc::c_int, r_idx: libc::c_int,
                         ego: libc::c_int, slp: bool_, status: libc::c_int)
     -> s16b;
    #[no_mangle]
    fn get_mon_num(level: libc::c_int) -> s16b;
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
pub struct border_type {
    pub north: [byte_hack; 198],
    pub south: [byte_hack; 198],
    pub east: [byte_hack; 66],
    pub west: [byte_hack; 66],
    pub north_west: byte_hack,
    pub north_east: byte_hack,
    pub south_west: byte_hack,
    pub south_east: byte_hack,
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
/* File: generate.c */
/* Purpose: Wilderness & Town related things */
/*
 * Copyright (c) 2001 James E. Wilson, Robert A. Koeneke, DarkGod
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/*
 * Various defines for the wilderness
 */
/* Chance of finding a wilderness vault. */
/*
 * Various defines for the towns
 */
/*
 * Helper for plasma generation.
 */
unsafe extern "C" fn perturb_point_mid(mut x1: libc::c_int,
                                       mut x2: libc::c_int,
                                       mut x3: libc::c_int,
                                       mut x4: libc::c_int,
                                       mut xmid: libc::c_int,
                                       mut ymid: libc::c_int,
                                       mut rough: libc::c_int,
                                       mut depth_max: libc::c_int) {
    /*
	 * Average the four corners & perturb it a bit.
	 * tmp is a random int +/- rough
	 */
    let mut tmp2: libc::c_int = rough * 2 as libc::c_int + 1 as libc::c_int;
    let mut tmp: libc::c_int =
        Rand_div(tmp2) + 1 as libc::c_int - (rough + 1 as libc::c_int);
    let mut avg: libc::c_int = (x1 + x2 + x3 + x4) / 4 as libc::c_int + tmp;
    /* Division always rounds down, so we round up again */
    if (x1 + x2 + x3 + x4) % 4 as libc::c_int > 1 as libc::c_int { avg += 1 }
    /* Normalize */
    if avg < 0 as libc::c_int { avg = 0 as libc::c_int }
    if avg > depth_max { avg = depth_max }
    /* Set the new value. */
    (*cave[ymid as usize].offset(xmid as isize)).feat = avg as byte_hack;
}
unsafe extern "C" fn perturb_point_end(mut x1: libc::c_int,
                                       mut x2: libc::c_int,
                                       mut x3: libc::c_int,
                                       mut xmid: libc::c_int,
                                       mut ymid: libc::c_int,
                                       mut rough: libc::c_int,
                                       mut depth_max: libc::c_int) {
    /*
	 * Average the three corners & perturb it a bit.
	 * tmp is a random int +/- rough
	 */
    let mut tmp2: libc::c_int = rough * 2 as libc::c_int + 1 as libc::c_int;
    let mut tmp: libc::c_int =
        Rand_div(tmp2) + 1 as libc::c_int - (rough + 1 as libc::c_int);
    let mut avg: libc::c_int = (x1 + x2 + x3) / 3 as libc::c_int + tmp;
    /* Division always rounds down, so we round up again */
    if (x1 + x2 + x3) % 3 as libc::c_int != 0 { avg += 1 }
    /* Normalize */
    if avg < 0 as libc::c_int { avg = 0 as libc::c_int }
    if avg > depth_max { avg = depth_max }
    /* Set the new value. */
    (*cave[ymid as usize].offset(xmid as isize)).feat = avg as byte_hack;
}
/*
 * A generic function to generate the plasma fractal.
 * Note that it uses ``cave_feat'' as temporary storage.
 * The values in ``cave_feat'' after this function
 * are NOT actual features; They are raw heights which
 * need to be converted to features.
 *
 * So we shouldn't call cave_set_feat in the helper functions
 * above.
 */
unsafe extern "C" fn plasma_recursive(mut x1: libc::c_int,
                                      mut y1: libc::c_int,
                                      mut x2: libc::c_int,
                                      mut y2: libc::c_int,
                                      mut depth_max: libc::c_int,
                                      mut rough: libc::c_int) {
    /* Find middle */
    let mut xmid: libc::c_int = (x2 - x1) / 2 as libc::c_int + x1;
    let mut ymid: libc::c_int = (y2 - y1) / 2 as libc::c_int + y1;
    /* Are we done? */
    if x1 + 1 as libc::c_int == x2 { return }
    perturb_point_mid((*cave[y1 as usize].offset(x1 as isize)).feat as
                          libc::c_int,
                      (*cave[y2 as usize].offset(x1 as isize)).feat as
                          libc::c_int,
                      (*cave[y1 as usize].offset(x2 as isize)).feat as
                          libc::c_int,
                      (*cave[y2 as usize].offset(x2 as isize)).feat as
                          libc::c_int, xmid, ymid, rough, depth_max);
    perturb_point_end((*cave[y1 as usize].offset(x1 as isize)).feat as
                          libc::c_int,
                      (*cave[y1 as usize].offset(x2 as isize)).feat as
                          libc::c_int,
                      (*cave[ymid as usize].offset(xmid as isize)).feat as
                          libc::c_int, xmid, y1, rough, depth_max);
    perturb_point_end((*cave[y1 as usize].offset(x2 as isize)).feat as
                          libc::c_int,
                      (*cave[y2 as usize].offset(x2 as isize)).feat as
                          libc::c_int,
                      (*cave[ymid as usize].offset(xmid as isize)).feat as
                          libc::c_int, x2, ymid, rough, depth_max);
    perturb_point_end((*cave[y2 as usize].offset(x2 as isize)).feat as
                          libc::c_int,
                      (*cave[y2 as usize].offset(x1 as isize)).feat as
                          libc::c_int,
                      (*cave[ymid as usize].offset(xmid as isize)).feat as
                          libc::c_int, xmid, y2, rough, depth_max);
    perturb_point_end((*cave[y2 as usize].offset(x1 as isize)).feat as
                          libc::c_int,
                      (*cave[y1 as usize].offset(x1 as isize)).feat as
                          libc::c_int,
                      (*cave[ymid as usize].offset(xmid as isize)).feat as
                          libc::c_int, x1, ymid, rough, depth_max);
    /* Recurse the four quadrants */
    plasma_recursive(x1, y1, xmid, ymid, depth_max, rough);
    plasma_recursive(xmid, y1, x2, ymid, depth_max, rough);
    plasma_recursive(x1, ymid, xmid, y2, depth_max, rough);
    plasma_recursive(xmid, ymid, x2, y2, depth_max, rough);
}
/*
 * Load a town or generate a terrain level using "plasma" fractals.
 *
 * x and y are the coordinates of the area in the wilderness.
 * Border and corner are optimization flags to speed up the
 * generation of the fractal terrain.
 * If border is set then only the border of the terrain should
 * be generated (for initializing the border structure).
 * If corner is set then only the corners of the area are needed.
 *
 * Return the number of floor grids
 */
#[no_mangle]
pub unsafe extern "C" fn generate_area(mut y: libc::c_int, mut x: libc::c_int,
                                       mut border_0: bool_, mut corner: bool_,
                                       mut refresh: bool_) -> libc::c_int {
    let mut road: libc::c_int = 0;
    let mut entrance: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut hack_floor: libc::c_int = 0 as libc::c_int;
    /* Number of the town (if any) */
    (*p_ptr).town_num =
        (*wf_info.offset((*(*wild_map.offset(y as
                                                 isize)).offset(x as
                                                                    isize)).feat
                             as isize)).entrance as
            s16b; /* The roughness of the level. */
    if (*p_ptr).town_num == 0 {
        (*p_ptr).town_num =
            (*(*wild_map.offset(y as isize)).offset(x as isize)).entrance as
                s16b
    } /* The terrain around the current area */
    let mut roughness: libc::c_int = 1 as libc::c_int;
    let mut terrain: [[libc::c_int; 3]; 3] = [[0; 3]; 3];
    let mut ym: libc::c_int = 0;
    let mut xm: libc::c_int = 0;
    let mut yp: libc::c_int = 0;
    let mut xp: libc::c_int = 0;
    /* Place the player at the center */
    if (*p_ptr).oldpx == 0 {
        (*p_ptr).oldpx = (198 as libc::c_int / 2 as libc::c_int) as s16b
    }
    if (*p_ptr).oldpy == 0 {
        (*p_ptr).oldpy = (66 as libc::c_int / 2 as libc::c_int) as s16b
    }
    /* Initialize the terrain array */
    ym =
        if (y - 1 as libc::c_int) < 0 as libc::c_int {
            0 as libc::c_int
        } else { (y) - 1 as libc::c_int };
    xm =
        if (x - 1 as libc::c_int) < 0 as libc::c_int {
            0 as libc::c_int
        } else { (x) - 1 as libc::c_int };
    yp =
        if y + 1 as libc::c_int >= max_wild_y as libc::c_int {
            (max_wild_y as libc::c_int) - 1 as libc::c_int
        } else { (y) + 1 as libc::c_int };
    xp =
        if x + 1 as libc::c_int >= max_wild_x as libc::c_int {
            (max_wild_x as libc::c_int) - 1 as libc::c_int
        } else { (x) + 1 as libc::c_int };
    terrain[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*(*wild_map.offset(ym as isize)).offset(xm as isize)).feat;
    terrain[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*(*wild_map.offset(ym as isize)).offset(x as isize)).feat;
    terrain[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        (*(*wild_map.offset(ym as isize)).offset(xp as isize)).feat;
    terrain[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*(*wild_map.offset(y as isize)).offset(xm as isize)).feat;
    terrain[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*(*wild_map.offset(y as isize)).offset(x as isize)).feat;
    terrain[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        (*(*wild_map.offset(y as isize)).offset(xp as isize)).feat;
    terrain[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*(*wild_map.offset(yp as isize)).offset(xm as isize)).feat;
    terrain[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*(*wild_map.offset(yp as isize)).offset(x as isize)).feat;
    terrain[2 as libc::c_int as usize][2 as libc::c_int as usize] =
        (*(*wild_map.offset(yp as isize)).offset(xp as isize)).feat;
    /* Hack -- Use the "simple" RNG */
    Rand_quick = 1 as libc::c_int as bool_;
    /* Hack -- Induce consistant town layout */
    Rand_value = (*(*wild_map.offset(y as isize)).offset(x as isize)).seed;
    if corner == 0 {
        /* Create level background */
        y1 = 0 as libc::c_int;
        while y1 < 66 as libc::c_int {
            x1 = 0 as libc::c_int;
            while x1 < 198 as libc::c_int {
                cave_set_feat(y1, x1, 18 as libc::c_int / 2 as libc::c_int);
                x1 += 1
            }
            y1 += 1
        }
    }
    /*
		 * Initialize the four corners
		 * ToDo: calculate the medium height of the adjacent
		 * terrains for every corner.
		 */
    cave_set_feat(1 as libc::c_int, 1 as libc::c_int,
                  Rand_div(18 as libc::c_int) as byte_hack as libc::c_int);
    cave_set_feat(66 as libc::c_int - 2 as libc::c_int, 1 as libc::c_int,
                  Rand_div(18 as libc::c_int) as byte_hack as libc::c_int);
    cave_set_feat(1 as libc::c_int, 198 as libc::c_int - 2 as libc::c_int,
                  Rand_div(18 as libc::c_int) as byte_hack as libc::c_int);
    cave_set_feat(66 as libc::c_int - 2 as libc::c_int,
                  198 as libc::c_int - 2 as libc::c_int,
                  Rand_div(18 as libc::c_int) as byte_hack as libc::c_int);
    if corner == 0 {
        /* x1, y1, x2, y2, num_depths, roughness */
        plasma_recursive(1 as libc::c_int, 1 as libc::c_int,
                         198 as libc::c_int - 2 as libc::c_int,
                         66 as libc::c_int - 2 as libc::c_int,
                         18 as libc::c_int - 1 as libc::c_int, roughness);
    }
    /* Use the complex RNG */
    Rand_quick = 0 as libc::c_int as bool_;
    y1 = 1 as libc::c_int;
    while y1 < 66 as libc::c_int - 1 as libc::c_int {
        x1 = 1 as libc::c_int;
        while x1 < 198 as libc::c_int - 1 as libc::c_int {
            cave_set_feat(y1, x1,
                          (*wf_info.offset(terrain[1 as libc::c_int as
                                                       usize][1 as libc::c_int
                                                                  as usize] as
                                               isize)).terrain[(*cave[y1 as
                                                                          usize].offset(x1
                                                                                            as
                                                                                            isize)).feat
                                                                   as usize]
                              as libc::c_int);
            x1 += 1
        }
        y1 += 1
    }
    /* Should we create a town ? */
    if (*p_ptr).town_num as libc::c_int > 0 as libc::c_int &&
           ((*p_ptr).town_num as libc::c_int) < 1000 as libc::c_int {
        /* Create the town */
        let mut xstart: libc::c_int = 0 as libc::c_int;
        let mut ystart: libc::c_int = 0 as libc::c_int;
        /* Initialize the town */
        init_flags = 0x4 as libc::c_int;
        process_dungeon_file(b"t_info.txt\x00" as *const u8 as
                                 *const libc::c_char, &mut ystart,
                             &mut xstart, cur_hgt as libc::c_int,
                             cur_wid as libc::c_int,
                             1 as libc::c_int as bool_,
                             0 as libc::c_int as bool_);
    } else {
        /* Reset the town flag */
        (*p_ptr).town_num = 0 as libc::c_int as s16b
    }
    if corner == 0 {
        /*
		 * Place roads in the wilderness
		 * ToDo: make the road a bit more interresting
		 */
        road =
            (*wf_info.offset((*(*wild_map.offset(y as
                                                     isize)).offset(x as
                                                                        isize)).feat
                                 as isize)).road as libc::c_int;
        if road & 1 as libc::c_int != 0 {
            /* North road */
            y1 = 1 as libc::c_int;
            while y1 < 66 as libc::c_int / 2 as libc::c_int {
                x1 = 198 as libc::c_int / 2 as libc::c_int;
                cave_set_feat(y1, x1, 0x1 as libc::c_int);
                y1 += 1
            }
        }
        if road & 2 as libc::c_int != 0 {
            /* North road */
            y1 = 66 as libc::c_int / 2 as libc::c_int;
            while y1 < 66 as libc::c_int - 1 as libc::c_int {
                x1 = 198 as libc::c_int / 2 as libc::c_int;
                cave_set_feat(y1, x1, 0x1 as libc::c_int);
                y1 += 1
            }
        }
        if road & 4 as libc::c_int != 0 {
            /* East road */
            x1 = 198 as libc::c_int / 2 as libc::c_int;
            while x1 < 198 as libc::c_int - 1 as libc::c_int {
                y1 = 66 as libc::c_int / 2 as libc::c_int;
                cave_set_feat(y1, x1, 0x1 as libc::c_int);
                x1 += 1
            }
        }
        if road & 8 as libc::c_int != 0 {
            /* West road */
            x1 = 1 as libc::c_int;
            while x1 < 198 as libc::c_int / 2 as libc::c_int {
                y1 = 66 as libc::c_int / 2 as libc::c_int;
                cave_set_feat(y1, x1, 0x1 as libc::c_int);
                x1 += 1
            }
        }
    }
    /* Hack -- Use the "simple" RNG */
    Rand_quick = 1 as libc::c_int as bool_;
    /* Hack -- Induce consistant town layout */
    Rand_value = (*(*wild_map.offset(y as isize)).offset(x as isize)).seed;
    entrance =
        (*wf_info.offset((*(*wild_map.offset(y as
                                                 isize)).offset(x as
                                                                    isize)).feat
                             as isize)).entrance as libc::c_int;
    if entrance == 0 {
        entrance =
            (*(*wild_map.offset(y as isize)).offset(x as isize)).entrance as
                libc::c_int
    }
    /* Create the dungeon if requested on the map */
    if entrance >= 1000 as libc::c_int {
        let mut dy: libc::c_int = 0;
        let mut dx: libc::c_int = 0;
        dy =
            6 as libc::c_int +
                Rand_div(1 as libc::c_int +
                             (cur_hgt as libc::c_int - 6 as libc::c_int) -
                             6 as libc::c_int);
        dx =
            6 as libc::c_int +
                Rand_div(1 as libc::c_int +
                             (cur_wid as libc::c_int - 6 as libc::c_int) -
                             6 as libc::c_int);
        cave_set_feat(dy, dx, 0x7 as libc::c_int);
        (*cave[dy as usize].offset(dx as isize)).special =
            (entrance - 1000 as libc::c_int) as s16b;
        let ref mut fresh0 = (*cave[dy as usize].offset(dx as isize)).info;
        *fresh0 =
            (*fresh0 as libc::c_int |
                 (0x2 as libc::c_int | 0x1 as libc::c_int)) as u16b
    }
    /* Use the complex RNG */
    Rand_quick = 0 as libc::c_int as bool_;
    /* MEGA HACK -- set at least one floor grid */
    y1 = 1 as libc::c_int;
    while y1 < cur_hgt as libc::c_int - 1 as libc::c_int {
        x1 = 1 as libc::c_int;
        while x1 < cur_wid as libc::c_int - 1 as libc::c_int {
            if (*f_info.offset((*cave[y1 as usize].offset(x1 as isize)).feat
                                   as isize)).flags1 as libc::c_long &
                   0x10 as libc::c_long != 0 &&
                   (*cave[y1 as usize].offset(x1 as isize)).feat as
                       libc::c_int != 0xaf as libc::c_int {
                hack_floor += 1
            }
            x1 += 1
        }
        y1 += 1
    }
    /* NO floor ? put one */
    if hack_floor == 0 {
        cave_set_feat(cur_hgt as libc::c_int / 2 as libc::c_int,
                      cur_wid as libc::c_int / 2 as libc::c_int,
                      0x59 as libc::c_int);
        (*cave[(cur_hgt as libc::c_int / 2 as libc::c_int) as
                   usize].offset((cur_wid as libc::c_int / 2 as libc::c_int)
                                     as isize)).special =
            0 as libc::c_int as s16b;
        hack_floor = 1 as libc::c_int
    }
    /* Set the monster generation level to the wilderness level */
    monster_level =
        (*wf_info.offset((*(*wild_map.offset(y as
                                                 isize)).offset(x as
                                                                    isize)).feat
                             as isize)).level as s16b;
    /* Set the object generation level to the wilderness level */
    object_level =
        (*wf_info.offset((*(*wild_map.offset(y as
                                                 isize)).offset(x as
                                                                    isize)).feat
                             as isize)).level as s16b;
    return hack_floor;
}
/*
 * Border of the wilderness area
 */
static mut border: border_type =
    border_type{north: [0; 198],
                south: [0; 198],
                east: [0; 66],
                west: [0; 66],
                north_west: 0,
                north_east: 0,
                south_west: 0,
                south_east: 0,};
/*
 * Build the wilderness area outside of the town.
 * -KMW-
 */
#[no_mangle]
pub unsafe extern "C" fn wilderness_gen(mut refresh: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut hack_floor: libc::c_int = 0;
    let mut daytime: bool_ = 0;
    let mut xstart: libc::c_int = 0 as libc::c_int;
    let mut ystart: libc::c_int = 0 as libc::c_int;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Init the wilderness */
    process_dungeon_file(b"w_info.txt\x00" as *const u8 as
                             *const libc::c_char, &mut ystart, &mut xstart,
                         cur_hgt as libc::c_int, cur_wid as libc::c_int,
                         1 as libc::c_int as bool_,
                         0 as libc::c_int as bool_);
    x = (*p_ptr).wilderness_x;
    y = (*p_ptr).wilderness_y;
    /* Set the correct monster hook */
    set_mon_num_hook();
    /* Prepare allocation table */
    get_mon_num_prep();
    /* North border */
    generate_area(y - 1 as libc::c_int, x, 1 as libc::c_int as bool_,
                  0 as libc::c_int as bool_, refresh as bool_);
    i = 1 as libc::c_int;
    while i < 198 as libc::c_int - 1 as libc::c_int {
        border.north[i as usize] =
            (*cave[(66 as libc::c_int - 2 as libc::c_int) as
                       usize].offset(i as isize)).feat;
        i += 1
    }
    /* South border */
    generate_area(y + 1 as libc::c_int, x, 1 as libc::c_int as bool_,
                  0 as libc::c_int as bool_, refresh as bool_);
    i = 1 as libc::c_int;
    while i < 198 as libc::c_int - 1 as libc::c_int {
        border.south[i as usize] =
            (*cave[1 as libc::c_int as usize].offset(i as isize)).feat;
        i += 1
    }
    /* West border */
    generate_area(y, x - 1 as libc::c_int, 1 as libc::c_int as bool_,
                  0 as libc::c_int as bool_, refresh as bool_);
    i = 1 as libc::c_int;
    while i < 66 as libc::c_int - 1 as libc::c_int {
        border.west[i as usize] =
            (*cave[i as
                       usize].offset((198 as libc::c_int - 2 as libc::c_int)
                                         as isize)).feat;
        i += 1
    }
    /* East border */
    generate_area(y, x + 1 as libc::c_int, 1 as libc::c_int as bool_,
                  0 as libc::c_int as bool_, refresh as bool_);
    i = 1 as libc::c_int;
    while i < 66 as libc::c_int - 1 as libc::c_int {
        border.east[i as usize] =
            (*cave[i as usize].offset(1 as libc::c_int as isize)).feat;
        i += 1
    }
    /* North west corner */
    generate_area(y - 1 as libc::c_int, x - 1 as libc::c_int,
                  0 as libc::c_int as bool_, 1 as libc::c_int as bool_,
                  refresh as bool_);
    border.north_west =
        (*cave[(66 as libc::c_int - 2 as libc::c_int) as
                   usize].offset((198 as libc::c_int - 2 as libc::c_int) as
                                     isize)).feat;
    /* North east corner */
    generate_area(y - 1 as libc::c_int, x + 1 as libc::c_int,
                  0 as libc::c_int as bool_, 1 as libc::c_int as bool_,
                  refresh as bool_);
    border.north_east =
        (*cave[(66 as libc::c_int - 2 as libc::c_int) as
                   usize].offset(1 as libc::c_int as isize)).feat;
    /* South west corner */
    generate_area(y + 1 as libc::c_int, x - 1 as libc::c_int,
                  0 as libc::c_int as bool_, 1 as libc::c_int as bool_,
                  refresh as bool_);
    border.south_west =
        (*cave[1 as libc::c_int as
                   usize].offset((198 as libc::c_int - 2 as libc::c_int) as
                                     isize)).feat;
    /* South east corner */
    generate_area(y + 1 as libc::c_int, x + 1 as libc::c_int,
                  0 as libc::c_int as bool_, 1 as libc::c_int as bool_,
                  refresh as bool_);
    border.south_east =
        (*cave[1 as libc::c_int as
                   usize].offset(1 as libc::c_int as isize)).feat;
    /* Create terrain of the current area */
    hack_floor =
        generate_area(y, x, 0 as libc::c_int as bool_,
                      0 as libc::c_int as bool_, refresh as bool_);
    /* Special boundary walls -- North */
    i = 0 as libc::c_int;
    while i < 198 as libc::c_int {
        (*cave[0 as libc::c_int as usize].offset(i as isize)).mimic =
            border.north[i as usize];
        cave_set_feat(0 as libc::c_int, i, 0x3f as libc::c_int);
        i += 1
    }
    /* Special boundary walls -- South */
    i = 0 as libc::c_int;
    while i < 198 as libc::c_int {
        (*cave[(66 as libc::c_int - 1 as libc::c_int) as
                   usize].offset(i as isize)).mimic =
            border.south[i as usize];
        cave_set_feat(66 as libc::c_int - 1 as libc::c_int, i,
                      0x3f as libc::c_int);
        i += 1
    }
    /* Special boundary walls -- West */
    i = 0 as libc::c_int;
    while i < 66 as libc::c_int {
        (*cave[i as usize].offset(0 as libc::c_int as isize)).mimic =
            border.west[i as usize];
        cave_set_feat(i, 0 as libc::c_int, 0x3f as libc::c_int);
        i += 1
    }
    /* Special boundary walls -- East */
    i = 0 as libc::c_int;
    while i < 66 as libc::c_int {
        (*cave[i as
                   usize].offset((198 as libc::c_int - 1 as libc::c_int) as
                                     isize)).mimic = border.east[i as usize];
        cave_set_feat(i, 198 as libc::c_int - 1 as libc::c_int,
                      0x3f as libc::c_int);
        i += 1
    }
    /* North west corner */
    (*cave[0 as libc::c_int as usize].offset(0 as libc::c_int as isize)).mimic
        = border.north_west;
    /* Make sure it has correct CAVE_WALL flag set */
    cave_set_feat(0 as libc::c_int, 0 as libc::c_int,
                  (*cave[0 as libc::c_int as
                             usize].offset(0 as libc::c_int as isize)).feat as
                      libc::c_int);
    /* North east corner */
    (*cave[0 as libc::c_int as
               usize].offset((198 as libc::c_int - 1 as libc::c_int) as
                                 isize)).mimic = border.north_east;
    /* Make sure it has correct CAVE_WALL flag set */
    cave_set_feat(0 as libc::c_int, 198 as libc::c_int - 1 as libc::c_int,
                  (*cave[0 as libc::c_int as
                             usize].offset((198 as libc::c_int -
                                                1 as libc::c_int) as
                                               isize)).feat as libc::c_int);
    /* South west corner */
    (*cave[(66 as libc::c_int - 1 as libc::c_int) as
               usize].offset(0 as libc::c_int as isize)).mimic =
        border.south_west;
    /* Make sure it has correct CAVE_WALL flag set */
    cave_set_feat(66 as libc::c_int - 1 as libc::c_int, 0 as libc::c_int,
                  (*cave[(66 as libc::c_int - 1 as libc::c_int) as
                             usize].offset(0 as libc::c_int as isize)).feat as
                      libc::c_int);
    /* South east corner */
    (*cave[(66 as libc::c_int - 1 as libc::c_int) as
               usize].offset((198 as libc::c_int - 1 as libc::c_int) as
                                 isize)).mimic = border.south_east;
    /* Make sure it has correct CAVE_WALL flag set */
    cave_set_feat(66 as libc::c_int - 1 as libc::c_int,
                  198 as libc::c_int - 1 as libc::c_int,
                  (*cave[(66 as libc::c_int - 1 as libc::c_int) as
                             usize].offset((198 as libc::c_int -
                                                1 as libc::c_int) as
                                               isize)).feat as libc::c_int);
    /* Day time */
    if (turn as libc::c_long %
            (10 as libc::c_long * 11520 as libc::c_int as libc::c_long)) <
           10 as libc::c_long * 11520 as libc::c_int as libc::c_long /
               2 as libc::c_int as libc::c_long {
        daytime = 1 as libc::c_int as bool_
    } else { daytime = 0 as libc::c_int as bool_ }
    /* Light up or darken the area */
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            /* Get the cave grid */
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type;
            if daytime != 0 {
                /* Assume lit */
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int | 0x2 as libc::c_int) as
                        u16b;
                /* Hack -- Memorize lit grids if allowed */
                if view_perma_grids != 0 {
                    (*c_ptr).info =
                        ((*c_ptr).info as libc::c_int | 0x1 as libc::c_int) as
                            u16b
                }
            } else if (*f_info.offset((*c_ptr).feat as isize)).flags1 as
                          libc::c_long & 0x100 as libc::c_long == 0 {
                /* Darken "boring" features */
                /* Forget the grid */
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int &
                         !(0x2 as libc::c_int | 0x1 as libc::c_int)) as u16b
            }
            x += 1
        }
        y += 1
    }
    player_place((*p_ptr).oldpy as libc::c_int,
                 (*p_ptr).oldpx as libc::c_int);
    if refresh == 0 {
        let mut lim: libc::c_int =
            if generate_encounter as libc::c_int == 1 as libc::c_int {
                60 as libc::c_int
            } else { 8 as libc::c_int };
        /*
		 * Can't have more monsters than floor grids -1(for the player,
		 * not needed but safer
		 */
        if lim > hack_floor - 1 as libc::c_int {
            lim = hack_floor - 1 as libc::c_int
        }
        /* Make some residents */
        i = 0 as libc::c_int;
        while i < lim {
            /* Make a resident */
            alloc_monster(if generate_encounter as libc::c_int ==
                                 1 as libc::c_int {
                              0 as libc::c_int
                          } else { 3 as libc::c_int },
                          if generate_encounter as libc::c_int ==
                                 1 as libc::c_int {
                              0 as libc::c_int
                          } else { 1 as libc::c_int } as bool_);
            i += 1
        }
        if generate_encounter != 0 { ambush_flag = 1 as libc::c_int as bool_ }
        generate_encounter = 0 as libc::c_int as bool_
    }
    /* Set rewarded quests to finished */
    i = 0 as libc::c_int;
    while i < max_q_idx as libc::c_int {
        if (*quest.offset(i as isize)).status as libc::c_int ==
               3 as libc::c_int {
            (*quest.offset(i as isize)).status = 5 as libc::c_int as s16b
        }
        i += 1
    }
    process_hooks(14 as libc::c_int,
                  b"(d)\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, 0 as libc::c_int);
}
/*
 * Build the wilderness area.
 * -DG-
 */
#[no_mangle]
pub unsafe extern "C" fn wilderness_gen_small() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut entrance: libc::c_int = 0;
    let mut xstart: libc::c_int = 0 as libc::c_int;
    let mut ystart: libc::c_int = 0 as libc::c_int;
    /* To prevent stupid things */
    i = 0 as libc::c_int;
    while i < 198 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 66 as libc::c_int {
            cave_set_feat(j, i, 0xb6 as libc::c_int);
            j += 1
        }
        i += 1
    }
    /* Init the wilderness */
    process_dungeon_file(b"w_info.txt\x00" as *const u8 as
                             *const libc::c_char, &mut ystart, &mut xstart,
                         cur_hgt as libc::c_int, cur_wid as libc::c_int,
                         1 as libc::c_int as bool_,
                         0 as libc::c_int as bool_);
    /* Fill the map */
    i = 0 as libc::c_int;
    while i < max_wild_x as libc::c_int {
        j = 0 as libc::c_int;
        while j < max_wild_y as libc::c_int {
            entrance =
                (*wf_info.offset((*(*wild_map.offset(j as
                                                         isize)).offset(i as
                                                                            isize)).feat
                                     as isize)).entrance as libc::c_int;
            if entrance == 0 {
                entrance =
                    (*(*wild_map.offset(j as
                                            isize)).offset(i as
                                                               isize)).entrance
                        as libc::c_int
            }
            if (*(*wild_map.offset(j as isize)).offset(i as isize)).entrance
                   != 0 {
                cave_set_feat(j, i, 0x7 as libc::c_int);
            } else {
                cave_set_feat(j, i,
                              (*wf_info.offset((*(*wild_map.offset(j as
                                                                       isize)).offset(i
                                                                                          as
                                                                                          isize)).feat
                                                   as isize)).feat as
                                  libc::c_int);
            }
            if (*cave[j as usize].offset(i as isize)).feat as libc::c_int ==
                   0x7 as libc::c_int && entrance >= 1000 as libc::c_int {
                (*cave[j as usize].offset(i as isize)).special =
                    (entrance - 1000 as libc::c_int) as s16b
            }
            /* Show it if we know it */
            if (*(*wild_map.offset(j as isize)).offset(i as isize)).known != 0
               {
                let ref mut fresh1 =
                    (*cave[j as usize].offset(i as isize)).info;
                *fresh1 =
                    (*fresh1 as libc::c_int |
                         (0x2 as libc::c_int | 0x1 as libc::c_int)) as u16b
            }
            j += 1
        }
        i += 1
    }
    /* Place the player */
    (*p_ptr).px = (*p_ptr).wilderness_x as s16b;
    (*p_ptr).py = (*p_ptr).wilderness_y as s16b;
    /* Set rewarded quests to finished */
    i = 0 as libc::c_int;
    while i < max_q_idx as libc::c_int {
        if (*quest.offset(i as isize)).status as libc::c_int ==
               3 as libc::c_int {
            (*quest.offset(i as isize)).status = 5 as libc::c_int as s16b
        }
        i += 1
    }
    process_hooks(14 as libc::c_int,
                  b"(d)\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, 1 as libc::c_int);
}
/* Show a small radius of wilderness around the player */
#[no_mangle]
pub unsafe extern "C" fn reveal_wilderness_around_player(mut y: libc::c_int,
                                                         mut x: libc::c_int,
                                                         mut h: libc::c_int,
                                                         mut w: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    /* Circle or square ? */
    if h == 0 as libc::c_int {
        i = x - w;
        while i < x + w {
            j = y - w;
            while j < y + w {
                /* Bound checking */
                if j > 0 as libc::c_int && i > 0 as libc::c_int &&
                       j < cur_hgt as libc::c_int - 1 as libc::c_int &&
                       i < cur_wid as libc::c_int - 1 as libc::c_int {
                    /* Severe bound checking */
                    if !(i < 0 as libc::c_int ||
                             i >= max_wild_x as libc::c_int ||
                             j < 0 as libc::c_int ||
                             j >= max_wild_y as libc::c_int) {
                        /* We want a radius, not a "squarus" :) */
                        if !(distance(y, x, j, i) >= w) {
                            /* New we know here */
                            (*(*wild_map.offset(j as
                                                    isize)).offset(i as
                                                                       isize)).known
                                = 1 as libc::c_int as bool_;
                            /* Only if we are in overview */
                            if (*p_ptr).wild_mode != 0 {
                                let ref mut fresh2 =
                                    (*cave[j as
                                               usize].offset(i as
                                                                 isize)).info;
                                *fresh2 =
                                    (*fresh2 as libc::c_int |
                                         (0x2 as libc::c_int |
                                              0x1 as libc::c_int)) as u16b;
                                /* Show it */
                                lite_spot(j, i);
                            }
                        }
                    }
                }
                j += 1
            }
            i += 1
        }
    } else {
        i = x;
        while i < x + w {
            j = y;
            while j < y + h {
                /* Bound checking */
                if j > 0 as libc::c_int && i > 0 as libc::c_int &&
                       j < cur_hgt as libc::c_int - 1 as libc::c_int &&
                       i < cur_wid as libc::c_int - 1 as libc::c_int {
                    /* New we know here */
                    (*(*wild_map.offset(j as isize)).offset(i as isize)).known
                        = 1 as libc::c_int as bool_;
                    /* Only if we are in overview */
                    if (*p_ptr).wild_mode != 0 {
                        let ref mut fresh3 =
                            (*cave[j as usize].offset(i as isize)).info;
                        *fresh3 =
                            (*fresh3 as libc::c_int |
                                 (0x2 as libc::c_int | 0x1 as libc::c_int)) as
                                u16b;
                        /* Show it */
                        lite_spot(j, i);
                    }
                }
                j += 1
            }
            i += 1
        }
    };
}
/*
 * Builds a store at a given pseudo-location
 *
 * As of 2.8.1 (?) the town is actually centered in the middle of a
 * complete level, and thus the top left corner of the town itself
 * is no longer at (0,0), but rather, at (qy,qx), so the constants
 * in the comments below should be mentally modified accordingly.
 *
 * As of 2.7.4 (?) the stores are placed in a more "user friendly"
 * configuration, such that the four "center" buildings always
 * have at least four grids between them, to allow easy running,
 * and the store doors tend to face the middle of town.
 *
 * The stores now lie inside boxes from 3-9 and 12-18 vertically,
 * and from 7-17, 21-31, 35-45, 49-59.  Note that there are thus
 * always at least 2 open grids between any disconnected walls.
 *
 * Note the use of "town_illuminate()" to handle all "illumination"
 * and "memorization" issues.
 */
unsafe extern "C" fn build_store(mut qy: libc::c_int, mut qx: libc::c_int,
                                 mut n: libc::c_int, mut yy: libc::c_int,
                                 mut xx: libc::c_int) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y0: libc::c_int = 0;
    let mut x0: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    /* Find the "center" of the store */
    y0 = qy + yy * 9 as libc::c_int + 6 as libc::c_int;
    x0 = qx + xx * 14 as libc::c_int + 12 as libc::c_int;
    /* Determine the store boundaries */
    y1 =
        y0 -
            (Rand_div((if yy == 0 as libc::c_int {
                           3 as libc::c_int
                       } else { 2 as libc::c_int })) + 1 as libc::c_int);
    y2 =
        y0 +
            (Rand_div((if yy == 1 as libc::c_int {
                           3 as libc::c_int
                       } else { 2 as libc::c_int })) + 1 as libc::c_int);
    x1 = x0 - (Rand_div(5 as libc::c_int) + 1 as libc::c_int);
    x2 = x0 + (Rand_div(5 as libc::c_int) + 1 as libc::c_int);
    /* Build an invulnerable rectangular building */
    y = y1;
    while y <= y2 {
        x = x1;
        while x <= x2 {
            /* Create the building */
            cave_set_feat(y, x, 0x3c as libc::c_int);
            x += 1
        }
        y += 1
    }
    /* Pick a door direction (S,N,E,W) */
    tmp = Rand_div(4 as libc::c_int);
    /* Re-roll "annoying" doors */
    if tmp == 0 as libc::c_int && yy == 1 as libc::c_int ||
           tmp == 1 as libc::c_int && yy == 0 as libc::c_int ||
           tmp == 2 as libc::c_int && xx == 3 as libc::c_int ||
           tmp == 3 as libc::c_int && xx == 0 as libc::c_int {
        /* Pick a new direction */
        tmp = Rand_div(4 as libc::c_int)
    }
    /* Extract a "door location" */
    match tmp {
        0 => {
            /* Bottom side */
            y = y2;
            x = x1 + Rand_div(1 as libc::c_int + x2 - x1)
        }
        1 => {
            /* Top side */
            y = y1;
            x = x1 + Rand_div(1 as libc::c_int + x2 - x1)
        }
        2 => {
            /* Right side */
            y = y1 + Rand_div(1 as libc::c_int + y2 - y1);
            x = x2
        }
        _ => {
            /* Left side */
            y = y1 + Rand_div(1 as libc::c_int + y2 - y1);
            x = x1
        }
    }
    /* Clear previous contents, add a store door */
    cave_set_feat(y, x, 0x4a as libc::c_int);
    (*cave[y as usize].offset(x as isize)).special = n as s16b;
    let ref mut fresh4 = (*cave[y as usize].offset(x as isize)).info;
    *fresh4 = (*fresh4 as libc::c_int | 0x800 as libc::c_int) as u16b;
}
unsafe extern "C" fn build_store_circle(mut qy: libc::c_int,
                                        mut qx: libc::c_int,
                                        mut n: libc::c_int,
                                        mut yy: libc::c_int,
                                        mut xx: libc::c_int) {
    let mut tmp: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y0: libc::c_int = 0;
    let mut x0: libc::c_int = 0;
    let mut rad: libc::c_int = 2 as libc::c_int + Rand_div(2 as libc::c_int);
    /* Find the "center" of the store */
    y0 = qy + yy * 9 as libc::c_int + 6 as libc::c_int;
    x0 = qx + xx * 14 as libc::c_int + 12 as libc::c_int;
    /* Determine the store boundaries */
    /* Build an invulnerable circular building */
    y = y0 - rad;
    while y <= y0 + rad {
        x = x0 - rad;
        while x <= x0 + rad {
            if !(distance(y0, x0, y, x) > rad) {
                /* Create the building */
                cave_set_feat(y, x, 0x3c as libc::c_int);
            }
            x += 1
        }
        y += 1
    }
    /* Pick a door direction (S,N,E,W) */
    tmp = Rand_div(4 as libc::c_int);
    /* Re-roll "annoying" doors */
    if tmp == 0 as libc::c_int && yy == 1 as libc::c_int ||
           tmp == 1 as libc::c_int && yy == 0 as libc::c_int ||
           tmp == 2 as libc::c_int && xx == 3 as libc::c_int ||
           tmp == 3 as libc::c_int && xx == 0 as libc::c_int {
        /* Pick a new direction */
        tmp = Rand_div(4 as libc::c_int)
    }
    /* Extract a "door location" */
    match tmp {
        0 => {
            /* Bottom side */
            y = y0;
            while y <= y0 + rad {
                cave_set_feat(y, x0, 0x1 as libc::c_int);
                y += 1
            }
        }
        1 => {
            /* Top side */
            y = y0 - rad;
            while y <= y0 { cave_set_feat(y, x0, 0x1 as libc::c_int); y += 1 }
        }
        2 => {
            /* Right side */
            x = x0;
            while x <= x0 + rad {
                cave_set_feat(y0, x, 0x1 as libc::c_int);
                x += 1
            }
        }
        _ => {
            /* Left side */
            x = x0 - rad;
            while x <= x0 { cave_set_feat(y0, x, 0x1 as libc::c_int); x += 1 }
        }
    }
    /* Clear previous contents, add a store door */
    cave_set_feat(y0, x0, 0x4a as libc::c_int);
    (*cave[y0 as usize].offset(x0 as isize)).special = n as s16b;
    let ref mut fresh5 = (*cave[y0 as usize].offset(x0 as isize)).info;
    *fresh5 = (*fresh5 as libc::c_int | 0x800 as libc::c_int) as u16b;
}
unsafe extern "C" fn build_store_hidden(mut n: libc::c_int,
                                        mut yy: libc::c_int,
                                        mut xx: libc::c_int) {
    /* Clear previous contents, add a store door */
    cave_set_feat(yy, xx, 0x4a as libc::c_int);
    (*cave[yy as usize].offset(xx as isize)).special = n as s16b;
    let ref mut fresh6 = (*cave[yy as usize].offset(xx as isize)).info;
    *fresh6 = (*fresh6 as libc::c_int | 0x800 as libc::c_int) as u16b;
}
/* Return a list of stores */
unsafe extern "C" fn get_shops(mut rooms: *mut libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut num: libc::c_int = 0 as libc::c_int;
    n = 0 as libc::c_int;
    while n < max_st_idx as libc::c_int {
        *rooms.offset(n as isize) = 0 as libc::c_int;
        n += 1
    }
    n = 0 as libc::c_int;
    while n < max_st_idx as libc::c_int {
        let mut chance: libc::c_int = 50 as libc::c_int;
        if (*st_info.offset(n as isize)).flags1 as libc::c_long &
               0x40 as libc::c_long != 0 {
            chance += 30 as libc::c_int
        }
        if (*st_info.offset(n as isize)).flags1 as libc::c_long &
               0x10 as libc::c_long != 0 {
            chance -= 20 as libc::c_int
        }
        if (*st_info.offset(n as isize)).flags1 as libc::c_long &
               0x20 as libc::c_long != 0 {
            chance -= 30 as libc::c_int
        }
        if Rand_div(100 as libc::c_int) < chance {
            i = 0 as libc::c_int;
            while i < num { (*rooms.offset(i as isize)) == n; i += 1 }
            if (*st_info.offset(n as isize)).flags1 as libc::c_long &
                   0x100 as libc::c_long != 0 {
                let fresh7 = num;
                num = num + 1;
                *rooms.offset(fresh7 as isize) = n
            }
        }
        n += 1
    }
    return num;
}
/* Generate town borders */
unsafe extern "C" fn set_border(mut y: libc::c_int, mut x: libc::c_int) {
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Paranoia */
    if !(y > 0 as libc::c_int && x > 0 as libc::c_int &&
             y < cur_hgt as libc::c_int - 1 as libc::c_int &&
             x < cur_wid as libc::c_int - 1 as libc::c_int) {
        return
    }
    /* Was a floor */
    if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                           isize)).flags1 as libc::c_long &
           0x10 as libc::c_long != 0 &&
           (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
               0xaf as libc::c_int ||
           (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                               isize)).flags1 as libc::c_long &
               0x1000 as libc::c_long != 0 {
        cave_set_feat(y, x, 0x20 as libc::c_int);
    } else {
        /* Was a wall */
        cave_set_feat(y, x, 0x3f as libc::c_int);
    }
    /* Access grid */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Clear special attrs */
    (*c_ptr).mimic = 0 as libc::c_int as byte_hack;
    (*c_ptr).special = 0 as libc::c_int as s16b;
    (*c_ptr).info =
        ((*c_ptr).info as libc::c_int | 0x8 as libc::c_int) as u16b;
}
unsafe extern "C" fn town_borders(mut t_idx: libc::c_int, mut qy: libc::c_int,
                                  mut qx: libc::c_int) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    x = qx;
    y = qy;
    while y < qy + 22 as libc::c_int - 1 as libc::c_int {
        set_border(y, x);
        y += 1
    }
    x = qx + 66 as libc::c_int - 1 as libc::c_int;
    y = qy;
    while y < qy + 22 as libc::c_int - 1 as libc::c_int {
        set_border(y, x);
        y += 1
    }
    y = qy;
    x = qx;
    while x < qx + 66 as libc::c_int - 1 as libc::c_int {
        set_border(y, x);
        x += 1
    }
    y = qy + 22 as libc::c_int - 1 as libc::c_int;
    x = qx;
    while x < qx + 66 as libc::c_int { set_border(y, x); x += 1 };
}
unsafe extern "C" fn create_townpeople_hook(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    if (*r_ptr).d_char as libc::c_int == 't' as i32 {
        return 1 as libc::c_int as bool_
    } else { return 0 as libc::c_int as bool_ };
}
/*
 * Generate the "consistent" town features, and place the player
 *
 * Hack -- play with the R.N.G. to always yield the same town
 * layout, including the size and shape of the buildings, the
 * locations of the doorways, and the location of the stairs.
 */
unsafe extern "C" fn town_gen_hack(mut t_idx: libc::c_int,
                                   mut qy: libc::c_int, mut qx: libc::c_int) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut floor: libc::c_int = 0;
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut old_get_mon_num_hook:
            Option<unsafe extern "C" fn(_: libc::c_int) -> bool_> = None;
    let mut rooms: *mut libc::c_int = 0 as *mut libc::c_int;
    /* Do we use dungeon floor or normal one */
    if Rand_div(100 as libc::c_int) < 70 as libc::c_int {
        floor = 0x1 as libc::c_int
    } else { floor = 0 as libc::c_int }
    /* Place some floors */
    y = qy + 1 as libc::c_int;
    while y < qy + 22 as libc::c_int - 1 as libc::c_int {
        x = qx + 1 as libc::c_int;
        while x < qx + 66 as libc::c_int - 1 as libc::c_int {
            /* Create empty floor */
            cave_set_feat(y, x,
                          if floor != 0 {
                              floor
                          } else {
                              floor_type[Rand_div(100 as libc::c_int) as
                                             usize] as libc::c_int
                          });
            let ref mut fresh8 = (*cave[y as usize].offset(x as isize)).info;
            *fresh8 =
                (*fresh8 as libc::c_int |
                     (0x8 as libc::c_int | 0x800 as libc::c_int)) as u16b;
            x += 1
        }
        y += 1
    }
    /* Prepare an Array of "remaining stores", and count them */
    rooms =
        memset(ralloc((max_st_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_st_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong)) as
            *mut libc::c_int;
    num = get_shops(rooms);
    /* Place two rows of stores */
    y = 0 as libc::c_int;
    while y < 2 as libc::c_int {
        /* Place four stores per row */
        x = 0 as libc::c_int;
        while x < 4 as libc::c_int {
            num -= 1;
            if num > -(1 as libc::c_int) {
                /* Build that store at the proper location */
                build_store(qy, qx, *rooms.offset(num as isize), y, x);
            }
            x += 1
        }
        y += 1
    }
    rnfree(rooms as vptr,
           (max_st_idx as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong));
    /* Generates the town's borders */
    if Rand_div(100 as libc::c_int) < 70 as libc::c_int {
        town_borders(t_idx, qy, qx);
    }
    /* Some inhabitants(leveled .. hehe :) */
    /* Backup the old hook */
    old_get_mon_num_hook = get_mon_num_hook;
    /* Require "okay" monsters */
    get_mon_num_hook =
        Some(create_townpeople_hook as
                 unsafe extern "C" fn(_: libc::c_int) -> bool_);
    /* Prepare allocation table */
    get_mon_num_prep();
    x = qx;
    while x < qx + 66 as libc::c_int {
        y = qy;
        while y < qy + 22 as libc::c_int {
            let mut m_idx: libc::c_int = 0;
            let mut r_idx: libc::c_int = 0;
            /* Only in town */
            if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                   y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   x < cur_wid as libc::c_int - 1 as libc::c_int {
                if !((*cave[y as usize].offset(x as isize)).info as
                         libc::c_int & 0x800 as libc::c_int == 0) {
                    if (*f_info.offset((*cave[y as
                                                  usize].offset(x as
                                                                    isize)).feat
                                           as isize)).flags1 as libc::c_long &
                           0x10 as libc::c_long != 0 &&
                           (*cave[y as usize].offset(x as isize)).feat as
                               libc::c_int != 0xaf as libc::c_int &&
                           (*cave[y as usize].offset(x as isize)).m_idx == 0
                           &&
                           !(y == (*p_ptr).py as libc::c_int &&
                                 x == (*p_ptr).px as libc::c_int) {
                        if !(Rand_div(100 as libc::c_int) != 0) {
                            r_idx =
                                get_mon_num(0 as libc::c_int) as libc::c_int;
                            *m_allow_special.offset(r_idx as isize) =
                                1 as libc::c_int as bool_;
                            m_idx =
                                place_monster_one(y, x, r_idx,
                                                  0 as libc::c_int,
                                                  1 as libc::c_int as bool_,
                                                  -(2 as libc::c_int)) as
                                    libc::c_int;
                            *m_allow_special.offset(r_idx as isize) =
                                0 as libc::c_int as bool_;
                            if m_idx != 0 {
                                let mut m_ptr: *mut monster_type =
                                    &mut *m_list.offset(m_idx as isize) as
                                        *mut monster_type;
                                if ((*m_ptr).level as libc::c_int) <
                                       dun_level as libc::c_int /
                                           2 as libc::c_int {
                                    (*m_ptr).exp =
                                        ((if (*m_ptr).level as libc::c_int +
                                                 dun_level as libc::c_int /
                                                     2 as libc::c_int +
                                                 (Rand_div(dun_level as
                                                               libc::c_int /
                                                               2 as
                                                                   libc::c_int)
                                                      + 1 as libc::c_int) >
                                                 150 as libc::c_int {
                                              150 as libc::c_int
                                          } else {
                                              ((*m_ptr).level as libc::c_int +
                                                   dun_level as libc::c_int /
                                                       2 as libc::c_int) +
                                                  (Rand_div(dun_level as
                                                                libc::c_int /
                                                                2 as
                                                                    libc::c_int)
                                                       + 1 as libc::c_int)
                                          }) *
                                             (if (*m_ptr).level as libc::c_int
                                                     +
                                                     dun_level as libc::c_int
                                                         / 2 as libc::c_int +
                                                     (Rand_div(dun_level as
                                                                   libc::c_int
                                                                   /
                                                                   2 as
                                                                       libc::c_int)
                                                          + 1 as libc::c_int)
                                                     > 150 as libc::c_int {
                                                  150 as libc::c_int
                                              } else {
                                                  ((*m_ptr).level as
                                                       libc::c_int +
                                                       dun_level as
                                                           libc::c_int /
                                                           2 as libc::c_int) +
                                                      (Rand_div(dun_level as
                                                                    libc::c_int
                                                                    /
                                                                    2 as
                                                                        libc::c_int)
                                                           + 1 as libc::c_int)
                                              }) *
                                             (if (*m_ptr).level as libc::c_int
                                                     +
                                                     dun_level as libc::c_int
                                                         / 2 as libc::c_int +
                                                     (Rand_div(dun_level as
                                                                   libc::c_int
                                                                   /
                                                                   2 as
                                                                       libc::c_int)
                                                          + 1 as libc::c_int)
                                                     > 150 as libc::c_int {
                                                  150 as libc::c_int
                                              } else {
                                                  ((*m_ptr).level as
                                                       libc::c_int +
                                                       dun_level as
                                                           libc::c_int /
                                                           2 as libc::c_int) +
                                                      (Rand_div(dun_level as
                                                                    libc::c_int
                                                                    /
                                                                    2 as
                                                                        libc::c_int)
                                                           + 1 as libc::c_int)
                                              }) * 6 as libc::c_int) as u32b;
                                    monster_check_experience(m_idx,
                                                             1 as libc::c_int
                                                                 as bool_);
                                }
                            }
                        }
                    }
                }
            }
            y += 1
        }
        x += 1
    }
    /* Reset restriction */
    get_mon_num_hook = old_get_mon_num_hook;
    /* Prepare allocation table */
    get_mon_num_prep();
}
unsafe extern "C" fn town_gen_circle(mut t_idx: libc::c_int,
                                     mut qy: libc::c_int,
                                     mut qx: libc::c_int) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut cy: libc::c_int = 0;
    let mut cx: libc::c_int = 0;
    let mut rad: libc::c_int = 0;
    let mut floor: libc::c_int = 0;
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut old_get_mon_num_hook:
            Option<unsafe extern "C" fn(_: libc::c_int) -> bool_> = None;
    let mut rooms: *mut libc::c_int = 0 as *mut libc::c_int;
    /* Do we use dungeon floor or normal one */
    if Rand_div(100 as libc::c_int) < 70 as libc::c_int {
        floor = 0x1 as libc::c_int
    } else { floor = 0 as libc::c_int }
    rad = 22 as libc::c_int / 2 as libc::c_int;
    y = qy;
    x = qx + rad;
    while x < qx + 66 as libc::c_int - rad { set_border(y, x); x += 1 }
    y = qy + 22 as libc::c_int - 1 as libc::c_int;
    x = qx + rad;
    while x < qx + 66 as libc::c_int - rad { set_border(y, x); x += 1 }
    /* Place some floors */
    y = qy + 1 as libc::c_int;
    while y < qy + 22 as libc::c_int - 1 as libc::c_int {
        x = qx + rad;
        while x < qx + 66 as libc::c_int - rad {
            /* Create empty floor */
            cave_set_feat(y, x,
                          if floor != 0 {
                              floor
                          } else {
                              floor_type[Rand_div(100 as libc::c_int) as
                                             usize] as libc::c_int
                          });
            let ref mut fresh9 = (*cave[y as usize].offset(x as isize)).info;
            *fresh9 =
                (*fresh9 as libc::c_int |
                     (0x8 as libc::c_int | 0x800 as libc::c_int)) as u16b;
            x += 1
        }
        y += 1
    }
    cy = qy + 22 as libc::c_int / 2 as libc::c_int;
    cx = qx + rad;
    y = cy - rad;
    while y < cy + rad {
        x = cx - rad;
        while x < cx + 1 as libc::c_int {
            let mut d: libc::c_int = distance(cy, cx, y, x);
            if d == rad || d == rad - 1 as libc::c_int { set_border(y, x); }
            if d < rad - 1 as libc::c_int {
                cave_set_feat(y, x,
                              if floor != 0 {
                                  floor
                              } else {
                                  floor_type[Rand_div(100 as libc::c_int) as
                                                 usize] as libc::c_int
                              });
                let ref mut fresh10 =
                    (*cave[y as usize].offset(x as isize)).info;
                *fresh10 =
                    (*fresh10 as libc::c_int |
                         (0x8 as libc::c_int | 0x800 as libc::c_int)) as u16b
            }
            x += 1
        }
        y += 1
    }
    cx = qx + 66 as libc::c_int - rad - 1 as libc::c_int;
    y = cy - rad;
    while y < cy + rad {
        x = cx;
        while x < cx + rad + 1 as libc::c_int {
            let mut d_0: libc::c_int = distance(cy, cx, y, x);
            if d_0 == rad || d_0 == rad - 1 as libc::c_int {
                set_border(y, x);
            }
            if d_0 < rad - 1 as libc::c_int {
                cave_set_feat(y, x,
                              if floor != 0 {
                                  floor
                              } else {
                                  floor_type[Rand_div(100 as libc::c_int) as
                                                 usize] as libc::c_int
                              });
                let ref mut fresh11 =
                    (*cave[y as usize].offset(x as isize)).info;
                *fresh11 =
                    (*fresh11 as libc::c_int |
                         (0x8 as libc::c_int | 0x800 as libc::c_int)) as u16b
            }
            x += 1
        }
        y += 1
    }
    /* Prepare an Array of "remaining stores", and count them */
    rooms =
        memset(ralloc((max_st_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_st_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong)) as
            *mut libc::c_int;
    num = get_shops(rooms);
    /* Place two rows of stores */
    y = 0 as libc::c_int;
    while y < 2 as libc::c_int {
        /* Place four stores per row */
        x = 0 as libc::c_int;
        while x < 4 as libc::c_int {
            num -= 1;
            if num > -(1 as libc::c_int) {
                /* Build that store at the proper location */
                build_store_circle(qy, qx, *rooms.offset(num as isize), y, x);
            }
            x += 1
        }
        y += 1
    }
    rnfree(rooms as vptr,
           (max_st_idx as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong));
    /* Some inhabitants(leveled .. hehe :) */
    /* Backup the old hook */
    old_get_mon_num_hook = get_mon_num_hook;
    /* Require "okay" monsters */
    get_mon_num_hook =
        Some(create_townpeople_hook as
                 unsafe extern "C" fn(_: libc::c_int) -> bool_);
    /* Prepare allocation table */
    get_mon_num_prep();
    x = qx;
    while x < qx + 66 as libc::c_int {
        y = qy;
        while y < qy + 22 as libc::c_int {
            let mut m_idx: libc::c_int = 0;
            let mut r_idx: libc::c_int = 0;
            /* Only in town */
            if y > 0 as libc::c_int && x > 0 as libc::c_int &&
                   y < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   x < cur_wid as libc::c_int - 1 as libc::c_int {
                if !((*cave[y as usize].offset(x as isize)).info as
                         libc::c_int & 0x800 as libc::c_int == 0) {
                    if (*f_info.offset((*cave[y as
                                                  usize].offset(x as
                                                                    isize)).feat
                                           as isize)).flags1 as libc::c_long &
                           0x10 as libc::c_long != 0 &&
                           (*cave[y as usize].offset(x as isize)).feat as
                               libc::c_int != 0xaf as libc::c_int &&
                           (*cave[y as usize].offset(x as isize)).m_idx == 0
                           &&
                           !(y == (*p_ptr).py as libc::c_int &&
                                 x == (*p_ptr).px as libc::c_int) {
                        if !(Rand_div(100 as libc::c_int) != 0) {
                            r_idx =
                                get_mon_num(0 as libc::c_int) as libc::c_int;
                            *m_allow_special.offset(r_idx as isize) =
                                1 as libc::c_int as bool_;
                            m_idx =
                                place_monster_one(y, x, r_idx,
                                                  0 as libc::c_int,
                                                  1 as libc::c_int as bool_,
                                                  -(2 as libc::c_int)) as
                                    libc::c_int;
                            *m_allow_special.offset(r_idx as isize) =
                                0 as libc::c_int as bool_;
                            if m_idx != 0 {
                                let mut m_ptr: *mut monster_type =
                                    &mut *m_list.offset(m_idx as isize) as
                                        *mut monster_type;
                                if ((*m_ptr).level as libc::c_int) <
                                       dun_level as libc::c_int /
                                           2 as libc::c_int {
                                    (*m_ptr).exp =
                                        ((if (*m_ptr).level as libc::c_int +
                                                 dun_level as libc::c_int /
                                                     2 as libc::c_int +
                                                 (Rand_div(dun_level as
                                                               libc::c_int /
                                                               2 as
                                                                   libc::c_int)
                                                      + 1 as libc::c_int) >
                                                 150 as libc::c_int {
                                              150 as libc::c_int
                                          } else {
                                              ((*m_ptr).level as libc::c_int +
                                                   dun_level as libc::c_int /
                                                       2 as libc::c_int) +
                                                  (Rand_div(dun_level as
                                                                libc::c_int /
                                                                2 as
                                                                    libc::c_int)
                                                       + 1 as libc::c_int)
                                          }) *
                                             (if (*m_ptr).level as libc::c_int
                                                     +
                                                     dun_level as libc::c_int
                                                         / 2 as libc::c_int +
                                                     (Rand_div(dun_level as
                                                                   libc::c_int
                                                                   /
                                                                   2 as
                                                                       libc::c_int)
                                                          + 1 as libc::c_int)
                                                     > 150 as libc::c_int {
                                                  150 as libc::c_int
                                              } else {
                                                  ((*m_ptr).level as
                                                       libc::c_int +
                                                       dun_level as
                                                           libc::c_int /
                                                           2 as libc::c_int) +
                                                      (Rand_div(dun_level as
                                                                    libc::c_int
                                                                    /
                                                                    2 as
                                                                        libc::c_int)
                                                           + 1 as libc::c_int)
                                              }) *
                                             (if (*m_ptr).level as libc::c_int
                                                     +
                                                     dun_level as libc::c_int
                                                         / 2 as libc::c_int +
                                                     (Rand_div(dun_level as
                                                                   libc::c_int
                                                                   /
                                                                   2 as
                                                                       libc::c_int)
                                                          + 1 as libc::c_int)
                                                     > 150 as libc::c_int {
                                                  150 as libc::c_int
                                              } else {
                                                  ((*m_ptr).level as
                                                       libc::c_int +
                                                       dun_level as
                                                           libc::c_int /
                                                           2 as libc::c_int) +
                                                      (Rand_div(dun_level as
                                                                    libc::c_int
                                                                    /
                                                                    2 as
                                                                        libc::c_int)
                                                           + 1 as libc::c_int)
                                              }) * 6 as libc::c_int) as u32b;
                                    monster_check_experience(m_idx,
                                                             1 as libc::c_int
                                                                 as bool_);
                                }
                            }
                        }
                    }
                }
            }
            y += 1
        }
        x += 1
    }
    /* Reset restriction */
    get_mon_num_hook = old_get_mon_num_hook;
    /* Prepare allocation table */
    get_mon_num_prep();
}
unsafe extern "C" fn town_gen_hidden(mut t_idx: libc::c_int,
                                     mut qy: libc::c_int,
                                     mut qx: libc::c_int) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut rooms: *mut libc::c_int = 0 as *mut libc::c_int;
    /* Prepare an Array of "remaining stores", and count them */
    rooms =
        memset(ralloc((max_st_idx as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               (max_st_idx as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong)) as
            *mut libc::c_int;
    num = get_shops(rooms);
    /* Get a number of stores to place */
    n = Rand_div(num / 2 as libc::c_int) + num / 2 as libc::c_int;
    /* Place k stores */
    i = 0 as libc::c_int;
    while i < n {
        loop 
             /* Find a good spot */
             {
            y =
                1 as libc::c_int +
                    Rand_div(1 as libc::c_int +
                                 (cur_hgt as libc::c_int - 2 as libc::c_int) -
                                 1 as libc::c_int);
            x =
                1 as libc::c_int +
                    Rand_div(1 as libc::c_int +
                                 (cur_wid as libc::c_int - 2 as libc::c_int) -
                                 1 as libc::c_int);
            if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                   isize)).flags1 as libc::c_long &
                   0x10 as libc::c_long != 0 &&
                   (*cave[y as usize].offset(x as isize)).feat as libc::c_int
                       != 0xaf as libc::c_int &&
                   (*cave[y as usize].offset(x as isize)).m_idx == 0 &&
                   !(y == (*p_ptr).py as libc::c_int &&
                         x == (*p_ptr).px as libc::c_int) {
                break ;
            }
        }
        num -= 1;
        if num > -(1 as libc::c_int) {
            /* Build that store at the proper location */
            build_store_hidden(*rooms.offset(num as isize), y, x);
        }
        i += 1
    }
    rnfree(rooms as vptr,
           (max_st_idx as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong));
}
/*
 * Town logic flow for generation of new town
 *
 * We start with a fully wiped cave of normal floors.
 *
 * Note that town_gen_hack() plays games with the R.N.G.
 *
 * This function does NOT do anything about the owners of the stores,
 * nor the contents thereof.  It only handles the physical layout.
 *
 * We place the player on the stairs at the same time we make them.
 *
 * Hack -- since the player always leaves the dungeon by the stairs,
 * he is always placed on the stairs, even if he left the dungeon via
 * word of recall or teleport level.
 */
#[no_mangle]
pub unsafe extern "C" fn town_gen(mut t_idx: libc::c_int) {
    let mut qy: libc::c_int = 0;
    let mut qx: libc::c_int = 0;
    /* Level too small to contain a town */
    if (cur_hgt as libc::c_int) < 22 as libc::c_int { return }
    if (cur_wid as libc::c_int) < 66 as libc::c_int { return }
    /* Center fo the level */
    qy = (cur_hgt as libc::c_int - 22 as libc::c_int) / 2 as libc::c_int;
    qx = (cur_wid as libc::c_int - 66 as libc::c_int) / 2 as libc::c_int;
    /* Build stuff */
    match Rand_div(3 as libc::c_int) {
        0 => {
            town_gen_hack(t_idx, qy, qx);
            if wizard != 0 {
                msg_format(b"Town level(normal) (%d, seed %d)\x00" as
                               *const u8 as *const libc::c_char, t_idx,
                           (*town_info.offset(t_idx as isize)).seed);
            }
        }
        1 => {
            town_gen_circle(t_idx, qy, qx);
            if wizard != 0 {
                msg_format(b"Town level(circle)(%d, seed %d)\x00" as *const u8
                               as *const libc::c_char, t_idx,
                           (*town_info.offset(t_idx as isize)).seed);
            }
        }
        2 => {
            town_gen_hidden(t_idx, qy, qx);
            if wizard != 0 {
                msg_format(b"Town level(hidden)(%d, seed %d)\x00" as *const u8
                               as *const libc::c_char, t_idx,
                           (*town_info.offset(t_idx as isize)).seed);
            }
        }
        _ => { }
    }
    (*p_ptr).town_num = t_idx as s16b;
}
