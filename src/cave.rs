use ::libc;
extern "C" {
    #[no_mangle]
    static mut ddx_ddd: [s16b; 9];
    #[no_mangle]
    static mut ddy_ddd: [s16b; 9];
    #[no_mangle]
    static mut character_dungeon: bool_;
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut command_rep: s16b;
    #[no_mangle]
    static mut running: s16b;
    #[no_mangle]
    static mut resting: s16b;
    #[no_mangle]
    static mut cur_hgt: s16b;
    #[no_mangle]
    static mut cur_wid: s16b;
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut use_graphics: bool_;
    #[no_mangle]
    static mut use_bigtile: bool_;
    #[no_mangle]
    static mut graphics_mode: byte_hack;
    #[no_mangle]
    static mut o_max: s16b;
    #[no_mangle]
    static mut m_max: s16b;
    #[no_mangle]
    static mut avoid_shimmer: bool_;
    #[no_mangle]
    static mut avoid_other: bool_;
    #[no_mangle]
    static mut flush_disturb: bool_;
    #[no_mangle]
    static mut view_yellow_lite: bool_;
    #[no_mangle]
    static mut view_bright_lite: bool_;
    #[no_mangle]
    static mut view_granite_lite: bool_;
    #[no_mangle]
    static mut view_special_lite: bool_;
    #[no_mangle]
    static mut view_perma_grids: bool_;
    #[no_mangle]
    static mut view_torch_grids: bool_;
    #[no_mangle]
    static mut flow_by_sound: bool_;
    #[no_mangle]
    static mut player_symbols: bool_;
    #[no_mangle]
    static mut panel_row_min: s16b;
    #[no_mangle]
    static mut panel_row_max: s16b;
    #[no_mangle]
    static mut panel_col_min: s16b;
    #[no_mangle]
    static mut panel_col_max: s16b;
    #[no_mangle]
    static mut panel_row_prt: s16b;
    #[no_mangle]
    static mut floor_type: [s16b; 100];
    #[no_mangle]
    static mut fill_type: [s16b; 100];
    #[no_mangle]
    static mut health_who: s16b;
    #[no_mangle]
    static mut monster_race_idx: s16b;
    #[no_mangle]
    static mut monster_ego_idx: s16b;
    #[no_mangle]
    static mut tracked_object: *mut object_type;
    #[no_mangle]
    static mut lite_n: s16b;
    #[no_mangle]
    static mut lite_y: [s16b; 1536];
    #[no_mangle]
    static mut lite_x: [s16b; 1536];
    #[no_mangle]
    static mut view_n: s16b;
    #[no_mangle]
    static mut view_y: [byte_hack; 1536];
    #[no_mangle]
    static mut view_x: [byte_hack; 1536];
    #[no_mangle]
    static mut temp_n: s16b;
    #[no_mangle]
    static mut temp_y: [byte_hack; 16384];
    #[no_mangle]
    static mut temp_x: [byte_hack; 16384];
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut o_list: *mut object_type;
    #[no_mangle]
    static mut m_list: *mut monster_type;
    #[no_mangle]
    static mut misc_to_attr: [byte_hack; 256];
    #[no_mangle]
    static mut misc_to_char: [libc::c_char; 256];
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut f_info: *mut feature_type;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut re_info: *mut monster_ego;
    #[no_mangle]
    static mut race_mod_info: *mut player_race_mod;
    #[no_mangle]
    static mut t_info: *mut trap_type;
    #[no_mangle]
    static mut st_info: *mut store_info_type;
    #[no_mangle]
    static mut ang_sort_comp:
           Option<unsafe extern "C" fn(_: vptr, _: vptr, _: libc::c_int,
                                       _: libc::c_int) -> bool_>;
    #[no_mangle]
    static mut ang_sort_swap:
           Option<unsafe extern "C" fn(_: vptr, _: vptr, _: libc::c_int,
                                       _: libc::c_int) -> ()>;
    #[no_mangle]
    static mut max_r_idx: u16b;
    #[no_mangle]
    static mut max_k_idx: u16b;
    #[no_mangle]
    static mut max_f_idx: u16b;
    #[no_mangle]
    static mut random_artifacts: [random_artifact; 84];
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut random_quests: [random_quest; 99];
    #[no_mangle]
    static mut dungeon_flags1: u32b;
    #[no_mangle]
    static mut player_char_health: bool_;
    #[no_mangle]
    static mut effects: [effect_type; 128];
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn quit(str: cptr);
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn ralloc(len: huge_hack) -> vptr;
    #[no_mangle]
    fn quit_fmt(fmt: cptr, _: ...);
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn Term_queue_char(x: libc::c_int, y: libc::c_int, a: byte_hack,
                       c: libc::c_char, ta: byte_hack, tc: libc::c_char,
                       ea: byte_hack, ec: libc::c_char);
    #[no_mangle]
    fn Term_fresh() -> errr;
    #[no_mangle]
    fn Term_set_cursor(v: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_gotoxy(x: libc::c_int, y: libc::c_int) -> errr;
    #[no_mangle]
    fn Term_draw(x: libc::c_int, y: libc::c_int, a: byte_hack,
                 c: libc::c_char) -> errr;
    #[no_mangle]
    fn Term_addch(a: byte_hack, c: libc::c_char) -> errr;
    #[no_mangle]
    fn Term_clear() -> errr;
    #[no_mangle]
    fn Term_get_cursor(v: *mut libc::c_int) -> errr;
    #[no_mangle]
    fn Term_get_size(w: *mut libc::c_int, h: *mut libc::c_int) -> errr;
    #[no_mangle]
    fn Term_save() -> errr;
    #[no_mangle]
    fn Term_load() -> errr;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn race_info_idx(r_idx: libc::c_int, ego: libc::c_int)
     -> *mut monster_race;
    #[no_mangle]
    fn spell_color(type_0: libc::c_int) -> byte_hack;
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn move_cursor(row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn put_str(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn ang_sort(u: vptr, v: vptr, n: libc::c_int);
    #[no_mangle]
    fn update_mon(m_idx: libc::c_int, full: bool_);
    #[no_mangle]
    fn flush();
    #[no_mangle]
    fn is_randhero(level: libc::c_int) -> bool_;
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
pub struct trap_type {
    pub probability: s16b,
    pub another: s16b,
    pub p1valinc: s16b,
    pub difficulty: byte_hack,
    pub minlevel: byte_hack,
    pub color: byte_hack,
    pub flags: u32b,
    pub ident: bool_,
    pub known: s16b,
    pub name: s16b,
    pub dd: s16b,
    pub ds: s16b,
    pub text: s16b,
    pub g_attr: byte_hack,
    pub g_char: libc::c_char,
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
pub struct effect_type {
    pub time: s16b,
    pub dam: s16b,
    pub type_0: s16b,
    pub cy: s16b,
    pub cx: s16b,
    pub rad: s16b,
    pub flags: u32b,
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
pub struct random_artifact {
    pub name_full: [libc::c_char; 80],
    pub name_short: [libc::c_char; 80],
    pub level: byte_hack,
    pub attr: byte_hack,
    pub cost: u32b,
    pub activation: byte_hack,
    pub timeout: s16b,
    pub generated: byte_hack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct random_quest {
    pub type_0: byte_hack,
    pub r_idx: s16b,
    pub done: bool_,
}
/*
 * Temporary data used by "vinfo_init()"
 *
 *	- Number of grids
 *
 *	- Number of slopes
 *
 *	- Slope values
 *
 *	- Slope range per grid
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vinfo_hack {
    pub num_slopes: libc::c_int,
    pub slopes: [libc::c_long; 126],
    pub slopes_min: [[libc::c_long; 21]; 21],
    pub slopes_max: [[libc::c_long; 21]; 21],
}
/*
 * The 'vinfo_type' structure
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vinfo_type {
    pub grid_y: [s16b; 8],
    pub grid_x: [s16b; 8],
    pub bits_3: u32b,
    pub bits_2: u32b,
    pub bits_1: u32b,
    pub bits_0: u32b,
    pub next_0: *mut vinfo_type,
    pub next_1: *mut vinfo_type,
    pub y: byte_hack,
    pub x: byte_hack,
    pub d: byte_hack,
    pub r: byte_hack,
}
/* File: cave.c */
/* Purpose: low level dungeon routines -BEN- */
/*
 * Support for Adam Bolt's tileset, lighting and transparency effects
 * by Robert Ruehlmann (rr9@angband.org)
 */
/*
 * Approximate Distance between two points.
 *
 * When either the X or Y component dwarfs the other component,
 * this function is almost perfect, and otherwise, it tends to
 * over-estimate about one grid per fifteen grids of distance.
 *
 * Algorithm: hypot(dy,dx) = max(dy,dx) + min(dy,dx) / 2
 */
#[no_mangle]
pub unsafe extern "C" fn distance(mut y1: libc::c_int, mut x1: libc::c_int,
                                  mut y2: libc::c_int, mut x2: libc::c_int)
 -> libc::c_int {
    let mut dy: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    /* Find the absolute y/x distance components */
    dy = if y1 > y2 { (y1) - y2 } else { (y2) - y1 };
    dx = if x1 > x2 { (x1) - x2 } else { (x2) - x1 };
    /* Hack -- approximate the distance */
    d =
        if dy > dx {
            (dy) + (dx >> 1 as libc::c_int)
        } else { (dx) + (dy >> 1 as libc::c_int) };
    /* Return the distance */
    return d;
}
/*
 * Returns TRUE if a grid is considered to be a wall for the purpose
 * of magic mapping / clairvoyance
 */
unsafe extern "C" fn is_wall(mut c_ptr: *mut cave_type) -> bool_ {
    let mut feat: byte_hack = 0;
    /* Handle feature mimics */
    if (*c_ptr).mimic != 0 {
        feat = (*c_ptr).mimic
    } else { feat = (*c_ptr).feat }
    /* Paranoia */
    if feat as libc::c_int >= max_f_idx as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Vanilla floors and doors aren't considered to be walls */
    if (feat as libc::c_int) < 0x30 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Exception #1: a glass wall is a wall but doesn't prevent LOS */
    if feat as libc::c_int == 0xbc as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Exception #2: an illusion wall is not a wall but obstructs view */
    if feat as libc::c_int == 0xbd as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Exception #3: a small tree is a floor but obstructs view */
    if feat as libc::c_int == 0xca as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Normal cases: use the WALL flag in f_info.txt */
    return if (*f_info.offset(feat as isize)).flags1 as libc::c_long &
                  0x20 as libc::c_long != 0 {
               1 as libc::c_int
           } else { 0 as libc::c_int } as bool_;
}
/*
 * A simple, fast, integer-based line-of-sight algorithm.  By Joseph Hall,
 * 4116 Brewster Drive, Raleigh NC 27606.  Email to jnh@ecemwl.ncsu.edu.
 *
 * Returns TRUE if a line of sight can be traced from (x1,y1) to (x2,y2).
 *
 * The LOS begins at the center of the tile (x1,y1) and ends at the center of
 * the tile (x2,y2).  If los() is to return TRUE, all of the tiles this line
 * passes through must be floor tiles, except for (x1,y1) and (x2,y2).
 *
 * We assume that the "mathematical corner" of a non-floor tile does not
 * block line of sight.
 *
 * Because this function uses (short) ints for all calculations, overflow may
 * occur if dx and dy exceed 90.
 *
 * Once all the degenerate cases are eliminated, the values "qx", "qy", and
 * "m" are multiplied by a scale factor "f1 = abs(dx * dy * 2)", so that
 * we can use integer arithmetic.
 *
 * We travel from start to finish along the longer axis, starting at the border
 * between the first and second tiles, where the y offset = .5 * slope, taking
 * into account the scale factor.  See below.
 *
 * Also note that this function and the "move towards target" code do NOT
 * share the same properties.  Thus, you can see someone, target them, and
 * then fire a bolt at them, but the bolt may hit a wall, not them.  However,
 * by clever choice of target locations, you can sometimes throw a "curve".
 *
 * Note that "line of sight" is not "reflexive" in all cases.
 *
 * Use the "projectable()" routine to test "spell/missile line of sight".
 *
 * Use the "update_view()" function to determine player line-of-sight.
 */
#[no_mangle]
pub unsafe extern "C" fn los(mut y1: libc::c_int, mut x1: libc::c_int,
                             mut y2: libc::c_int, mut x2: libc::c_int)
 -> bool_ {
    /* Delta */
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    /* Absolute */
    let mut ax: libc::c_int = 0;
    let mut ay: libc::c_int = 0;
    /* Signs */
    let mut sx: libc::c_int = 0;
    let mut sy: libc::c_int = 0;
    /* Fractions */
    let mut qx: libc::c_int = 0;
    let mut qy: libc::c_int = 0;
    /* Scanners */
    let mut tx: libc::c_int = 0;
    let mut ty: libc::c_int = 0;
    /* Scale factors */
    let mut f1: libc::c_int = 0;
    let mut f2: libc::c_int = 0;
    /* Slope, or 1/Slope, of LOS */
    let mut m: libc::c_int = 0;
    /* Extract the offset */
    dy = y2 - y1;
    dx = x2 - x1;
    /* Extract the absolute offset */
    ay = if dy < 0 as libc::c_int { -dy } else { dy };
    ax = if dx < 0 as libc::c_int { -dx } else { dx };
    /* Handle adjacent (or identical) grids */
    if ax < 2 as libc::c_int && ay < 2 as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Paranoia -- require "safe" origin */
	/* if (!in_bounds(y1, x1)) return (FALSE); */
    /* Directly South/North */
    if dx == 0 {
        /* South -- check for walls */
        if dy > 0 as libc::c_int {
            ty = y1 + 1 as libc::c_int;
            while ty < y2 {
                if (*f_info.offset((*cave[ty as
                                              usize].offset(x1 as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x2 as libc::c_long != 0 {
                    return 0 as libc::c_int as bool_
                }
                ty += 1
            }
        } else {
            /* North -- check for walls */
            ty = y1 - 1 as libc::c_int;
            while ty > y2 {
                if (*f_info.offset((*cave[ty as
                                              usize].offset(x1 as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x2 as libc::c_long != 0 {
                    return 0 as libc::c_int as bool_
                }
                ty -= 1
            }
        }
        /* Assume los */
        return 1 as libc::c_int as bool_
    }
    /* Directly East/West */
    if dy == 0 {
        /* East -- check for walls */
        if dx > 0 as libc::c_int {
            tx = x1 + 1 as libc::c_int;
            while tx < x2 {
                if (*f_info.offset((*cave[y1 as
                                              usize].offset(tx as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x2 as libc::c_long != 0 {
                    return 0 as libc::c_int as bool_
                }
                tx += 1
            }
        } else {
            /* West -- check for walls */
            tx = x1 - 1 as libc::c_int;
            while tx > x2 {
                if (*f_info.offset((*cave[y1 as
                                              usize].offset(tx as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x2 as libc::c_long != 0 {
                    return 0 as libc::c_int as bool_
                }
                tx -= 1
            }
        }
        /* Assume los */
        return 1 as libc::c_int as bool_
    }
    /* Extract some signs */
    sx =
        if dx < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else { 1 as libc::c_int };
    sy =
        if dy < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else { 1 as libc::c_int };
    /* Vertical "knights" */
    if ax == 1 as libc::c_int {
        if ay == 2 as libc::c_int {
            if (*f_info.offset((*cave[(y1 + sy) as
                                          usize].offset(x1 as isize)).feat as
                                   isize)).flags1 as libc::c_long &
                   0x2 as libc::c_long == 0 {
                return 1 as libc::c_int as bool_
            }
        }
    } else if ay == 1 as libc::c_int {
        if ax == 2 as libc::c_int {
            if (*f_info.offset((*cave[y1 as
                                          usize].offset((x1 + sx) as
                                                            isize)).feat as
                                   isize)).flags1 as libc::c_long &
                   0x2 as libc::c_long == 0 {
                return 1 as libc::c_int as bool_
            }
        }
    }
    /* Horizontal "knights" */
    /* Calculate scale factor div 2 */
    f2 = ax * ay;
    /* Calculate scale factor */
    f1 = f2 << 1 as libc::c_int;
    /* Travel horizontally */
    if ax >= ay {
        /* Let m = dy / dx * 2 * (dy * dx) = 2 * dy * dy */
        qy = ay * ay;
        m = qy << 1 as libc::c_int;
        tx = x1 + sx;
        /* Consider the special case where slope == 1. */
        if qy == f2 { ty = y1 + sy; qy -= f1 } else { ty = y1 }
        /* Note (below) the case (qy == f2), where */
		/* the LOS exactly meets the corner of a tile. */
        while x2 - tx != 0 {
            if (*f_info.offset((*cave[ty as usize].offset(tx as isize)).feat
                                   as isize)).flags1 as libc::c_long &
                   0x2 as libc::c_long != 0 {
                return 0 as libc::c_int as bool_
            }
            qy += m;
            if qy < f2 {
                tx += sx
            } else if qy > f2 {
                ty += sy;
                if (*f_info.offset((*cave[ty as
                                              usize].offset(tx as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x2 as libc::c_long != 0 {
                    return 0 as libc::c_int as bool_
                }
                qy -= f1;
                tx += sx
            } else { ty += sy; qy -= f1; tx += sx }
        }
    } else {
        /* Travel vertically */
        /* Let m = dx / dy * 2 * (dx * dy) = 2 * dx * dx */
        qx = ax * ax;
        m = qx << 1 as libc::c_int;
        ty = y1 + sy;
        if qx == f2 { tx = x1 + sx; qx -= f1 } else { tx = x1 }
        while y2 - ty != 0
              /* Note (below) the case (qx == f2), where */
		/* the LOS exactly meets the corner of a tile. */
              {
            if (*f_info.offset((*cave[ty as usize].offset(tx as isize)).feat
                                   as isize)).flags1 as libc::c_long &
                   0x2 as libc::c_long != 0 {
                return 0 as libc::c_int as bool_
            }
            qx += m;
            if qx < f2 {
                ty += sy
            } else if qx > f2 {
                tx += sx;
                if (*f_info.offset((*cave[ty as
                                              usize].offset(tx as isize)).feat
                                       as isize)).flags1 as libc::c_long &
                       0x2 as libc::c_long != 0 {
                    return 0 as libc::c_int as bool_
                }
                qx -= f1;
                ty += sy
            } else { tx += sx; qx -= f1; ty += sy }
        }
    }
    /* Assume los */
    return 1 as libc::c_int as bool_;
}
/*
 * Returns true if the player's grid is dark
 */
#[no_mangle]
pub unsafe extern "C" fn no_lite() -> bool_ {
    return !((*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).info
                 as libc::c_int & 0x10 as libc::c_int != 0 as libc::c_int) as
               libc::c_int as bool_;
}
/*
 * Determine if a given location may be "destroyed"
 *
 * Used by destruction spells, and for placing stairs, etc.
 */
#[no_mangle]
pub unsafe extern "C" fn cave_valid_bold(mut y: libc::c_int,
                                         mut x: libc::c_int) -> bool_ {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    let mut this_o_idx: s16b = 0;
    let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
    /* Forbid perma-grids */
    if (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long &
           0x40 as libc::c_long != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Check objects */
    this_o_idx = (*c_ptr).o_idx;
    while this_o_idx != 0 {
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
        /* Acquire object */
        o_ptr = &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
        /* Acquire next object */
        next_o_idx = (*o_ptr).next_o_idx;
        /* Forbid artifact grids */
        if (*o_ptr).art_name as libc::c_int != 0 ||
               ((*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
                    (if (*o_ptr).name1 as libc::c_int != 0 {
                         1 as libc::c_int
                     } else { 0 as libc::c_int }) != 0 ||
                    (if (*o_ptr).art_name as libc::c_int != 0 {
                         1 as libc::c_int
                     } else { 0 as libc::c_int }) != 0 ||
                    (if (*k_info.offset((*o_ptr).k_idx as isize)).flags3 as
                            libc::c_long & 0x8000 as libc::c_long != 0 {
                         1 as libc::c_int
                     } else { 0 as libc::c_int }) != 0) {
            return 0 as libc::c_int as bool_
        }
        this_o_idx = next_o_idx
    }
    /* Accept */
    return 1 as libc::c_int as bool_;
}
/*
 * Hack -- Legal monster codes
 */
static mut image_monster_hack: cptr =
    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ\x00" as *const u8
        as *const libc::c_char;
/*
 * Hack -- Legal monster codes for IBM pseudo-graphics
 *
 * Dropped. Although this option has long left unmaintained, hardcoding
 * code points makes it impossible to update the font and prf files
 * flexibly. And the normal graphics code still works with it -- pelpel
 */
/*
 * Mega-Hack -- Hallucinatory monster
 */
unsafe extern "C" fn image_monster(mut ap: *mut byte_hack,
                                   mut cp: *mut libc::c_char) {
    let mut n: libc::c_int = 0;
    match graphics_mode as libc::c_int {
        0 => {
            /* Text mode */
            n = strlen(image_monster_hack) as libc::c_int;
            /* Random symbol from set above */
            *cp = *image_monster_hack.offset(Rand_div(n) as isize);
            /* Random color */
            *ap =
                (Rand_div(15 as libc::c_int) + 1 as libc::c_int) as byte_hack
        }
        _ => {
            /* Normal graphics */
            /* Avoid player ghost */
            n = Rand_div(max_r_idx as s32b) + 1 as libc::c_int;
            *cp = (*r_info.offset(n as isize)).x_char;
            *ap = (*r_info.offset(n as isize)).x_attr
        }
    };
}
/*
 * Hack -- Legal object codes
 */
static mut image_object_hack: cptr =
    b"?/|\\\"!$()_-=[]{},~\x00" as *const u8 as *const libc::c_char;
/*
 * Hardcoded IBM pseudo-graphics code points have been removed
 * for the same reason as stated above -- pelpel
 */
/*
 * Mega-Hack -- Hallucinatory object
 */
unsafe extern "C" fn image_object(mut ap: *mut byte_hack,
                                  mut cp: *mut libc::c_char) {
    let mut n: libc::c_int = 0;
    match graphics_mode as libc::c_int {
        0 => {
            /* Text mode */
            n = strlen(image_object_hack) as libc::c_int;
            /* Random symbol from set above */
            *cp = *image_object_hack.offset(Rand_div(n) as isize);
            /* Random color */
            *ap =
                (Rand_div(15 as libc::c_int) + 1 as libc::c_int) as byte_hack
        }
        _ => {
            /* Normal graphics */
            n =
                Rand_div(max_k_idx as libc::c_int - 1 as libc::c_int) +
                    1 as libc::c_int;
            *cp = (*k_info.offset(n as isize)).x_char;
            *ap = (*k_info.offset(n as isize)).x_attr
        }
    };
}
/*
 * Hack -- Random hallucination
 */
unsafe extern "C" fn image_random(mut ap: *mut byte_hack,
                                  mut cp: *mut libc::c_char) {
    /* Normally, assume monsters */
    if Rand_div(100 as libc::c_int) < 75 as libc::c_int {
        image_monster(ap, cp);
    } else {
        /* Otherwise, assume objects */
        image_object(ap, cp);
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_shimmer_color() -> libc::c_char {
    match Rand_div(7 as libc::c_int) + 1 as libc::c_int {
        1 => { return 4 as libc::c_int as libc::c_char }
        2 => { return 12 as libc::c_int as libc::c_char }
        3 => { return 1 as libc::c_int as libc::c_char }
        4 => { return 13 as libc::c_int as libc::c_char }
        5 => { return 6 as libc::c_int as libc::c_char }
        6 => { return 8 as libc::c_int as libc::c_char }
        7 => { return 5 as libc::c_int as libc::c_char }
        _ => { }
    }
    return 10 as libc::c_int as libc::c_char;
}
/*
 * Table of breath colors.  Must match listings in a single set of 
 * monster spell flags.
 *
 * The value "255" is special.  Monsters with that kind of breath 
 * may be any color.
 */
static mut breath_to_attr: [[byte_hack; 2]; 32] =
    [[0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack],
     [2 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack],
     [6 as libc::c_int as byte_hack, 14 as libc::c_int as byte_hack],
     [4 as libc::c_int as byte_hack, 12 as libc::c_int as byte_hack],
     [1 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack],
     [5 as libc::c_int as byte_hack, 13 as libc::c_int as byte_hack],
     [13 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack],
     [11 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack],
     [8 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack],
     [15 as libc::c_int as byte_hack, 7 as libc::c_int as byte_hack],
     [11 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack],
     [255 as libc::c_int as byte_hack, 255 as libc::c_int as byte_hack],
     [10 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack],
     [12 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack],
     [14 as libc::c_int as byte_hack, 14 as libc::c_int as byte_hack],
     [9 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack],
     [9 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack],
     [7 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack],
     [3 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack],
     [7 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack],
     [14 as libc::c_int as byte_hack, 1 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack],
     [5 as libc::c_int as byte_hack, 13 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack],
     [1 as libc::c_int as byte_hack, 12 as libc::c_int as byte_hack]];
/*
 * Multi-hued monsters shimmer acording to their breaths.
 *
 * If a monster has only one kind of breath, it uses both colors 
 * associated with that breath.  Otherwise, it just uses the first 
 * color for any of its breaths.
 *
 * If a monster does not breath anything, it can be any color.
 */
unsafe extern "C" fn multi_hued_attr(mut r_ptr: *mut monster_race)
 -> byte_hack {
    let mut allowed_attrs: [byte_hack; 15] = [0; 15];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut stored_colors: libc::c_int = 0 as libc::c_int;
    let mut breaths: libc::c_int = 0 as libc::c_int;
    let mut first_color: libc::c_int = 0 as libc::c_int;
    let mut second_color: libc::c_int = 0 as libc::c_int;
    /* Monsters with no ranged attacks can be any color */
    if (*r_ptr).freq_inate == 0 { return get_shimmer_color() as byte_hack }
    /* Check breaths */
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        let mut stored: bool_ = 0 as libc::c_int as bool_;
        /* Don't have that breath */
        if !((*r_ptr).flags4 as libc::c_long & (1 as libc::c_long) << i == 0)
           {
            /* Get the first color of this breath */
            first_color =
                breath_to_attr[i as usize][0 as libc::c_int as usize] as
                    libc::c_int;
            /* Breath has no color associated with it */
            if !(first_color == 0 as libc::c_int) {
                /* Monster can be of any color */
                if first_color == 255 as libc::c_int {
                    return (Rand_div(15 as libc::c_int) + 1 as libc::c_int) as
                               byte_hack
                }
                /* Increment the number of breaths */
                breaths += 1;
                /* Monsters with lots of breaths may be any color. */
                if breaths == 6 as libc::c_int {
                    return (Rand_div(15 as libc::c_int) + 1 as libc::c_int) as
                               byte_hack
                }
                /* Always store the first color */
                j = 0 as libc::c_int;
                while j < stored_colors {
                    /* Already stored */
                    if allowed_attrs[j as usize] as libc::c_int == first_color
                       {
                        stored = 1 as libc::c_int as bool_
                    }
                    j += 1
                }
                if stored == 0 {
                    allowed_attrs[stored_colors as usize] =
                        first_color as byte_hack;
                    stored_colors += 1
                }
                /*
		 * Remember (but do not immediately store) the second color 
		 * of the first breath.
		 */
                if breaths == 1 as libc::c_int {
                    second_color =
                        breath_to_attr[i as usize][1 as libc::c_int as usize]
                            as libc::c_int
                }
            }
        }
        i += 1
    }
    /* Monsters with no breaths may be of any color. */
    if breaths == 0 as libc::c_int { return get_shimmer_color() as byte_hack }
    /* If monster has one breath, store the second color too. */
    if breaths == 1 as libc::c_int {
        allowed_attrs[stored_colors as usize] = second_color as byte_hack;
        stored_colors += 1
    }
    /* Pick a color at random */
    return allowed_attrs[Rand_div(stored_colors) as usize];
}
/*
 * Extract the attr/char to display at the given (legal) map location
 *
 * Note that this function, since it is called by "lite_spot()" which
 * is called by "update_view()", is a major efficiency concern.
 *
 * Basically, we examine each "layer" of the world (terrain, objects,
 * monsters/players), from the bottom up, extracting a new attr/char
 * if necessary at each layer, and defaulting to "darkness".  This is
 * not the fastest method, but it is very simple, and it is about as
 * fast as it could be for grids which contain no "marked" objects or
 * "visible" monsters.
 *
 * We apply the effects of hallucination during each layer.  Objects will
 * always appear as random "objects", monsters will always appear as random
 * "monsters", and normal grids occasionally appear as random "monsters" or
 * "objects", but note that these random "monsters" and "objects" are really
 * just "colored ascii symbols" (which may look silly on some machines).
 *
 * The hallucination functions avoid taking any pointers to local variables
 * because some compilers refuse to use registers for any local variables
 * whose address is taken anywhere in the function.
 *
 * As an optimization, we can handle the "player" grid as a special case.
 *
 * Note that the memorization of "objects" and "monsters" is not related
 * to the memorization of "terrain".  This allows the player to memorize
 * the terrain of a grid without memorizing any objects in that grid, and
 * to detect monsters without detecting anything about the terrain of the
 * grid containing the monster.
 *
 * The fact that all interesting "objects" and "terrain features" are
 * memorized as soon as they become visible for the first time means
 * that we only have to check the "CAVE_SEEN" flag for "boring" grids.
 *
 * Note that bizarre things must be done when the "attr" and/or "char"
 * codes have the "high-bit" set, since these values are used to encode
 * various "special" pictures in some versions, and certain situations,
 * such as "multi-hued" or "clear" monsters, cause the attr/char codes
 * to be "scrambled" in various ways.
 *
 * Note that the "zero" entry in the feature/object/monster arrays are
 * used to provide "special" attr/char codes, with "monster zero" being
 * used for the player attr/char, "object zero" being used for the "stack"
 * attr/char, and "feature zero" being used for the "nothing" attr/char.
 *
 * Note that eventually we may want to use the "&" symbol for embedded
 * treasure, and use the "*" symbol to indicate multiple objects, but
 * currently, we simply use the attr/char of the first "marked" object
 * in the stack, if any, and so "object zero" is unused.  XXX XXX XXX
 *
 * Note the assumption that doing "x_ptr = &x_info[x]" plus a few of
 * "x_ptr->xxx", is quicker than "x_info[x].xxx", even if "x" is a fixed
 * constant.  If this is incorrect then a lot of code should be changed.
 *
 *
 * Some comments on the "terrain" layer...
 *
 * Note that "boring" grids (floors, invisible traps, and any illegal grids)
 * are very different from "interesting" grids (all other terrain features),
 * and the two types of grids are handled completely separately.  The most
 * important distinction is that "boring" grids may or may not be memorized
 * when they are first encountered, and so we must use the "CAVE_SEEN" flag
 * to see if they are "see-able".
 *
 *
 * Some comments on the "terrain" layer (boring grids)...
 *
 * Note that "boring" grids are always drawn using the picture for "empty
 * floors", which is stored in "f_info[FEAT_FLOOR]".  Sometimes, special
 * lighting effects may cause this picture to be modified.
 *
 * Note that "invisible traps" are always displayes exactly like "empty
 * floors", which prevents various forms of "cheating", with no loss of
 * efficiency.  There are still a few ways to "guess" where traps may be
 * located, for example, objects will never fall into a grid containing
 * an invisible trap.  XXX XXX
 *
 * To determine if a "boring" grid should be displayed, we simply check to
 * see if it is either memorized ("CAVE_MARK"), or currently "see-able" by
 * the player ("CAVE_SEEN").  Note that "CAVE_SEEN" is now maintained by the
 * "update_view()" function.
 *
 * Note the "special lighting effects" which can be activated for "boring"
 * grids using the "view_special_lite" option, causing certain such grids
 * to be displayed using special colors. If the grid is "see-able" by
 * the player, we will use the normal (except that, if the "view_yellow_lite"
 * option is set, and the grid is *only* "see-able" because of the player's
 * torch, then we will use "yellow"), else if the player is "blind", we will
 * use greyscale, else if the grid is not "illuminated", we will use "dark
 * gray", if the "view_bright_lite" option is set, we will use "darker" colour
 * else we will use the normal colour.
 *
 *
 * Some comments on the "terrain" layer (non-boring grids)...
 *
 * Note the use of the "mimic" field in the "terrain feature" processing,
 * which allows any feature to "pretend" to be another feature.  This is
 * used to "hide" secret doors, and to make all "doors" appear the same,
 * and all "walls" appear the same, and "hidden" treasure stay hidden.
 * Note that it is possible to use this field to make a feature "look"
 * like a floor, but the "view_special_lite" flag only affects actual
 * "boring" grids.
 *
 * Since "interesting" grids are always memorized as soon as they become
 * "see-able" by the player ("CAVE_SEEN"), such a grid only needs to be
 * displayed if it is memorized ("CAVE_MARK").  Most "interesting" grids
 * are in fact non-memorized, non-see-able, wall grids, so the fact that
 * we do not have to check the "CAVE_SEEN" flag adds some efficiency, at
 * the cost of *forcing* the memorization of all "interesting" grids when
 * they are first seen.  Since the "CAVE_SEEN" flag is now maintained by
 * the "update_view()" function, this efficiency is not as significant as
 * it was in previous versions, and could perhaps be removed.
 * (so I removed this to simplify the terrain feature handling -- pelpel)
 *
 * Note the "special lighting effects" which can be activated for "wall"
 * grids using the "view_granite_lite" option, causing certain such grids
 * to be displayed using special colors.
 * If the grid is "see-able" by the player, we will use the normal colour
 * else if the player is "blind", we will use grey scale, else if the
 * "view_bright_lite" option is set, we will use reduced colour, else we
 * will use the normal one.
 *
 * Note that "wall" grids are more complicated than "boring" grids, due to
 * the fact that "CAVE_GLOW" for a "wall" grid means that the grid *might*
 * be glowing, depending on where the player is standing in relation to the
 * wall.  In particular, the wall of an illuminated room should look just
 * like any other (dark) wall unless the player is actually inside the room.
 *
 * Thus, we do not support as many visual special effects for "wall" grids
 * as we do for "boring" grids, since many of them would give the player
 * information about the "CAVE_GLOW" flag of the wall grid, in particular,
 * it would allow the player to notice the walls of illuminated rooms from
 * a dark hallway that happened to run beside the room.
 *
 *
 * Some comments on the "object" layer...
 *
 * Currently, we do nothing with multi-hued objects, because there are
 * not any.  If there were, they would have to set "shimmer_objects"
 * when they were created, and then new "shimmer" code in "dungeon.c"
 * would have to be created handle the "shimmer" effect, and the code
 * in "cave.c" would have to be updated to create the shimmer effect.
 * This did not seem worth the effort.  XXX XXX
 *
 *
 * Some comments on the "monster"/"player" layer...
 *
 * Note that monsters can have some "special" flags, including "ATTR_MULTI",
 * which means their color changes, and "ATTR_CLEAR", which means they take
 * the color of whatever is under them, and "CHAR_CLEAR", which means that
 * they take the symbol of whatever is under them.  Technically, the flag
 * "CHAR_MULTI" is supposed to indicate that a monster looks strange when
 * examined, but this flag is currently ignored.  All of these flags are
 * ignored if the "avoid_other" option is set, since checking for these
 * conditions is expensive (and annoying) on some systems.
 *
 * Normally, players could be handled just like monsters, except that the
 * concept of the "torch lite" of others player would add complications.
 * For efficiency, however, we handle the (only) player first, since the
 * "player" symbol always "pre-empts" any other facts about the grid.
 *
 * The "hidden_player" efficiency option, which only makes sense with a
 * single player, allows the player symbol to be hidden while running.
 */
/*
 * Alternative colours for unseen grids
 *
 * Reduced colours - remembered interesting grids and perma-lit floors
 * B&W - currently only used by blindness effect
 */
/* Colour */
static mut dark_attrs: [byte_hack; 16] =
    [0 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     8 as libc::c_int as byte_hack, 3 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     6 as libc::c_int as byte_hack, 7 as libc::c_int as byte_hack,
     8 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     10 as libc::c_int as byte_hack, 11 as libc::c_int as byte_hack,
     4 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack,
     6 as libc::c_int as byte_hack, 7 as libc::c_int as byte_hack];
/* B&W */
static mut darker_attrs: [byte_hack; 16] =
    [0 as libc::c_int as byte_hack, 9 as libc::c_int as byte_hack,
     8 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     8 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     8 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     8 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     8 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     2 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack];
#[no_mangle]
pub unsafe extern "C" fn map_info(mut y: libc::c_int, mut x: libc::c_int,
                                  mut ap: *mut byte_hack,
                                  mut cp: *mut libc::c_char,
                                  mut tap: *mut byte_hack,
                                  mut tcp: *mut libc::c_char,
                                  mut eap: *mut byte_hack,
                                  mut ecp: *mut libc::c_char) {
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut f_ptr: *mut feature_type = 0 as *mut feature_type;
    let mut this_o_idx: s16b = 0;
    let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
    let mut info: u16b = 0;
    let mut t_idx: s16b = 0;
    let mut feat: byte_hack = 0;
    let mut a: byte_hack = 0;
    let mut c: byte_hack = 0;
    /*
	 * This means that a port supports graphics overlay as well as lighting
	 * effects. See the step 3 below for the detailed information about
	 * lighting. Basically, it requires "darker" tiles for those terrain
	 * features with SUPPORT_LIGHT flag set, and they must be arranged
	 * this way:
	 *     col  col+1  col+2
	 * row base darker brighter
	 */
    let mut graf_new: bool_ =
        (graphics_mode as libc::c_int == 5 as libc::c_int ||
             graphics_mode as libc::c_int == 4 as libc::c_int) as libc::c_int
            as bool_;
    /*
	 * I never understand why some coders like shimmering so much.
	 * It just serves to hurt my eyes, IMHO.  If one feels like to show off,
	 * go for better graphics support... Anyway this means a port allows
	 * changing attr independently from its char -- pelpel
	 */
    let mut attr_mutable: bool_ =
        (use_graphics == 0 ||
             graphics_mode as libc::c_int == 2 as libc::c_int) as libc::c_int
            as bool_;
    /* *** Preparation ****/
    /* Access the grid */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Cache some frequently used values */
    /* Grid info */
    info = (*c_ptr).info;
    /* Feature code */
    feat = (*c_ptr).feat;
    /* Apply "mimic" field */
    if (*c_ptr).mimic != 0 {
        feat = (*c_ptr).mimic
    } else { feat = (*f_info.offset(feat as isize)).mimic }
    /* Access floor */
    f_ptr = &mut *f_info.offset(feat as isize) as *mut feature_type;
    /* Reset attr/char */
    *eap = 0 as libc::c_int as byte_hack;
    *ecp = 0 as libc::c_int as libc::c_char;
    /* *** Layer 1 -- Terrain feature ****/
    /* Only memorised or visible grids are displayed */
    if info as libc::c_int & (0x1 as libc::c_int | 0x10 as libc::c_int) != 0 {
        /* *** Step 1 -- Retrieve base attr/char ****/
        /* 'Sane' terrain features */
        if feat as libc::c_int != 0x4a as libc::c_int {
            /* Normal char */
            c = (*f_ptr).x_char as byte_hack;
            /* Normal attr */
            a = (*f_ptr).x_attr
        } else {
            /* Mega-Hack 1 -- Building don't conform to f_info */
            c =
                (*st_info.offset((*c_ptr).special as isize)).x_char as
                    byte_hack;
            a = (*st_info.offset((*c_ptr).special as isize)).x_attr
        }
        /* Mega-Hack 2 -- stair to dungeon branch are purple */
        if (*c_ptr).special as libc::c_int != 0 &&
               attr_mutable as libc::c_int != 0 &&
               (feat as libc::c_int == 0x7 as libc::c_int ||
                    feat as libc::c_int == 0x6 as libc::c_int) {
            a = 10 as libc::c_int as byte_hack
        }
        /* Mega-Hack 3 -- Traps don't have f_info entries either */
        if info as libc::c_int & 0x100 as libc::c_int != 0 &&
               feat as libc::c_int != 0xbd as libc::c_int {
            /* Trap index */
            t_idx = (*c_ptr).t_idx;
            if use_graphics as libc::c_int != 0 &&
                   (*t_info.offset(t_idx as isize)).g_attr as libc::c_int !=
                       0 as libc::c_int &&
                   (*t_info.offset(t_idx as isize)).g_char as libc::c_int !=
                       0 as libc::c_int {
                if graf_new != 0 {
                    *eap = (*t_info.offset(t_idx as isize)).g_attr;
                    *ecp = (*t_info.offset(t_idx as isize)).g_char
                } else {
                    a = (*t_info.offset(t_idx as isize)).g_attr;
                    c = (*t_info.offset(t_idx as isize)).g_char as byte_hack
                }
            } else if attr_mutable == 0 {
                a = (*f_info.offset(0x11 as libc::c_int as isize)).x_attr;
                c =
                    (*f_info.offset(0x11 as libc::c_int as isize)).x_char as
                        byte_hack
            } else {
                if (*f_ptr).flags1 as libc::c_long &
                       (0x10 as libc::c_long | 0x100 as libc::c_long) ==
                       0x10 as libc::c_long {
                    c =
                        (*f_info.offset(0x11 as libc::c_int as isize)).x_char
                            as byte_hack
                }
                /*
				 * If trap is set on a floor grid that is not
				 * one of "interesting" features, use a special
				 * symbol to display it. Check for doors is no longer
				 * necessary because they have REMEMBER flag now.
				 *
				 * Cave macros cannot be used safely here, because of
				 * c_ptr->mimic XXX XXX
				 */
                /* Add attr XXX XXX XXX */
                a = (*t_info.offset(t_idx as isize)).color;
                /* Get a new color with a strange formula :) XXX XXX XXX */
                if (*t_info.offset(t_idx as isize)).flags &
                       0x8 as libc::c_int as libc::c_uint != 0 {
                    let mut tmp: s32b = 0;
                    tmp =
                        dun_level as libc::c_int + dungeon_type as libc::c_int
                            + feat as libc::c_int;
                    a = (tmp % 16 as libc::c_int) as byte_hack
                }
            }
        }
        /* *** Step 2 -- Apply special random effects ****/
        if avoid_other == 0 && avoid_shimmer == 0 &&
               attr_mutable as libc::c_int != 0 {
            /* Special terrain effect */
            if (*c_ptr).effect != 0 {
                a =
                    spell_color(effects[(*c_ptr).effect as usize].type_0 as
                                    libc::c_int)
            } else if (*f_ptr).flags1 as libc::c_long &
                          0x20000 as libc::c_long != 0 {
                a = (*f_ptr).shimmer[Rand_div(7 as libc::c_int) as usize]
            }
        }
        /* Multi-hued attr */
        /*
		 * Step 3
		 *
		 * Special lighting effects, if specified and applicable
		 * This will never happen for
		 * - any grids in the overhead map
		 * - traps
		 * - (graphics modes) terrain features without corresponding
		 *   "darker" tiles.
		 *
		 * Note the use of f_ptr->flags1 to avoid problems with
		 * c_ptr->mimic.
		 */
        /* view_special_lite: lighting effects for boring features */
        if view_special_lite as libc::c_int != 0 &&
               (*f_ptr).flags1 as libc::c_long &
                   (0x10 as libc::c_long | 0x100 as libc::c_long) ==
                   0x10 as libc::c_long {
            if (*p_ptr).wild_mode == 0 &&
                   info as libc::c_int & 0x100 as libc::c_int == 0 &&
                   (attr_mutable as libc::c_int != 0 ||
                        graf_new as libc::c_int != 0 &&
                            (*f_info.offset(feat as isize)).flags1 as
                                libc::c_long & 0x2000 as libc::c_long !=
                                0 as libc::c_int as libc::c_long) {
                /* Handle "seen" grids */
                if info as libc::c_int & 0x10 as libc::c_int != 0 {
                    /* Only lit by "torch" light */
                    if view_yellow_lite as libc::c_int != 0 &&
                           info as libc::c_int & 0x2 as libc::c_int == 0 {
                        if graf_new != 0 {
                            /* Use a brightly lit tile */
                            c =
                                (c as libc::c_int + 2 as libc::c_int) as
                                    byte_hack
                        } else {
                            /* Use "yellow" */
                            a = 11 as libc::c_int as byte_hack
                        }
                    }
                } else if (*p_ptr).blind != 0 {
                    if graf_new != 0 {
                        /* Handle "blind" */
                        /* Use a dark tile */
                        c = c.wrapping_add(1)
                    } else {
                        /* Use darker colour */
                        a =
                            darker_attrs[(a as libc::c_int &
                                              0xf as libc::c_int) as usize]
                    }
                } else if info as libc::c_int & 0x2 as libc::c_int == 0 {
                    if graf_new != 0 {
                        /* Handle "dark" grids */
                        /* Use a dark tile */
                        c = c.wrapping_add(1)
                    } else {
                        /* Use darkest colour */
                        a = 8 as libc::c_int as byte_hack
                    }
                } else if view_bright_lite != 0 {
                    if graf_new != 0 {
                        /* "Out-of-sight" glowing grids -- handle "view_bright_lite" */
                        /* Use a dark tile */
                        c = c.wrapping_add(1)
                    } else {
                        /* Use darker colour */
                        a =
                            dark_attrs[(a as libc::c_int & 0xf as libc::c_int)
                                           as usize]
                    }
                }
            }
        } else if view_granite_lite as libc::c_int != 0 &&
                      (*f_ptr).flags1 as libc::c_long &
                          (0x2 as libc::c_long | 0x1000 as libc::c_long) != 0
         {
            if (*p_ptr).wild_mode == 0 &&
                   info as libc::c_int & 0x100 as libc::c_int == 0 &&
                   (attr_mutable as libc::c_int != 0 ||
                        graf_new as libc::c_int != 0 &&
                            (*f_info.offset(feat as isize)).flags1 as
                                libc::c_long & 0x2000 as libc::c_long !=
                                0 as libc::c_int as libc::c_long) {
                /* view_granite_lite: lighting effects for walls and doors */
                /* Handle "seen" grids */
                if !(info as libc::c_int & 0x10 as libc::c_int != 0) {
                    /* Handle "blind" */
                    if (*p_ptr).blind != 0 {
                        if graf_new != 0 {
                            /* Use a dark tile */
                            c = c.wrapping_add(1)
                        } else {
                            /* Use darker colour */
                            a =
                                darker_attrs[(a as libc::c_int &
                                                  0xf as libc::c_int) as
                                                 usize]
                        }
                    } else if view_bright_lite != 0 {
                        if graf_new != 0 {
                            /* Handle "view_bright_lite" */
                            /* Use a dark tile */
                            c = c.wrapping_add(1)
                        } else {
                            /* Use darker colour */
                            a =
                                dark_attrs[(a as libc::c_int &
                                                0xf as libc::c_int) as usize]
                        }
                    } else if graf_new != 0 {
                        /* Use a brightly lit tile */
                        c = (c as libc::c_int + 2 as libc::c_int) as byte_hack
                    }
                }
            }
        }
    } else {
        /* Unknown grids */
        /* Access darkness */
        f_ptr =
            &mut *f_info.offset(0 as libc::c_int as isize) as
                *mut feature_type;
        a = (*f_ptr).x_attr;
        c = (*f_ptr).x_char as byte_hack
    }
    /* Normal attr */
    /* Normal char */
    /*
	 * Hack -- rare random hallucination
	 * Because we cannot be sure which is outer dungeon walls,
	 * the check for 'feat' has been removed
	 */
    if (*p_ptr).image as libc::c_int != 0 &&
           Rand_div(256 as libc::c_int) == 0 as libc::c_int {
        /* Hallucinate */
        image_random(ap, cp);
    }
    /* Save the terrain info for the transparency effects */
    *tap = a;
    *tcp = c as libc::c_char;
    /* Save the info */
    *ap = a;
    *cp = c as libc::c_char;
    /* *** Layer 2 -- Objects ****/
    if feat as libc::c_int != 0xaf as libc::c_int {
        this_o_idx = (*c_ptr).o_idx;
        while this_o_idx != 0 {
            let mut o_ptr: *mut object_type = 0 as *mut object_type;
            /* Acquire object */
            o_ptr =
                &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
            /* Acquire next object */
            next_o_idx = (*o_ptr).next_o_idx;
            /* Memorized objects */
            if (*o_ptr).marked != 0 {
                /* Normal char */
                *cp =
                    if (*k_info.offset((*o_ptr).k_idx as isize)).flavor as
                           libc::c_int != 0 {
                        misc_to_char[(*k_info.offset((*o_ptr).k_idx as
                                                         isize)).flavor as
                                         usize] as libc::c_int
                    } else {
                        (*k_info.offset((*o_ptr).k_idx as isize)).x_char as
                            libc::c_int
                    } as libc::c_char;
                /* Normal attr */
                *ap =
                    if (*o_ptr).tval as libc::c_int == 102 as libc::c_int {
                        random_artifacts[(*o_ptr).sval as usize].attr as
                            libc::c_int
                    } else if (*k_info.offset((*o_ptr).k_idx as isize)).flavor
                                  as libc::c_int != 0 {
                        misc_to_attr[(*k_info.offset((*o_ptr).k_idx as
                                                         isize)).flavor as
                                         usize] as libc::c_int
                    } else {
                        (*k_info.offset((*o_ptr).k_idx as isize)).x_attr as
                            libc::c_int
                    } as byte_hack;
                /* Multi-hued attr */
                if avoid_other == 0 && attr_mutable as libc::c_int != 0 &&
                       (*k_info.offset((*o_ptr).k_idx as isize)).flags5 as
                           libc::c_long & 0x40 as libc::c_long != 0 {
                    *ap = get_shimmer_color() as byte_hack
                }
                /* Hack -- hallucination */
                if (*p_ptr).image != 0 { image_object(ap, cp); }
                break ;
            } else { this_o_idx = next_o_idx }
        }
    }
    /* *** Layer 3 -- Handle monsters ****/
    if (*c_ptr).m_idx != 0 {
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset((*c_ptr).m_idx as isize) as *mut monster_type;
        let mut r_ptr: *mut monster_race =
            if !(*m_ptr).sr_ptr.is_null() {
                (*m_ptr).sr_ptr
            } else {
                race_info_idx((*m_ptr).r_idx as libc::c_int,
                              (*m_ptr).ego as libc::c_int)
            };
        if (*r_ptr).flags9 & 0x8 as libc::c_int as libc::c_uint != 0 {
            let mut o_ptr_0: *mut object_type = 0 as *mut object_type;
            /* Acquire object */
            o_ptr_0 =
                &mut *o_list.offset((*m_ptr).hold_o_idx as isize) as
                    *mut object_type;
            /* Memorized objects */
            if (*o_ptr_0).marked != 0 {
                /* Normal char */
                *cp =
                    if (*k_info.offset((*o_ptr_0).k_idx as isize)).flavor as
                           libc::c_int != 0 {
                        misc_to_char[(*k_info.offset((*o_ptr_0).k_idx as
                                                         isize)).flavor as
                                         usize] as libc::c_int
                    } else {
                        (*k_info.offset((*o_ptr_0).k_idx as isize)).x_char as
                            libc::c_int
                    } as libc::c_char;
                /* Normal attr */
                *ap =
                    if (*o_ptr_0).tval as libc::c_int == 102 as libc::c_int {
                        random_artifacts[(*o_ptr_0).sval as usize].attr as
                            libc::c_int
                    } else if (*k_info.offset((*o_ptr_0).k_idx as
                                                  isize)).flavor as
                                  libc::c_int != 0 {
                        misc_to_attr[(*k_info.offset((*o_ptr_0).k_idx as
                                                         isize)).flavor as
                                         usize] as libc::c_int
                    } else {
                        (*k_info.offset((*o_ptr_0).k_idx as isize)).x_attr as
                            libc::c_int
                    } as byte_hack;
                /* Multi-hued attr */
                if avoid_other == 0 && attr_mutable as libc::c_int != 0 &&
                       (*k_info.offset((*o_ptr_0).k_idx as isize)).flags5 as
                           libc::c_long & 0x40 as libc::c_long != 0 {
                    *ap = get_shimmer_color() as byte_hack
                }
                /* Hack -- hallucination */
                if (*p_ptr).image != 0 { image_object(ap, cp); }
            }
        } else if (*m_ptr).ml != 0 {
            let mut r_ptr_0: *mut monster_race =
                if !(*m_ptr).sr_ptr.is_null() {
                    (*m_ptr).sr_ptr
                } else {
                    race_info_idx((*m_ptr).r_idx as libc::c_int,
                                  (*m_ptr).ego as libc::c_int)
                };
            /* Visible monster */
            /* Reset attr/char */
            *eap = 0 as libc::c_int as byte_hack;
            *ecp = 0 as libc::c_int as libc::c_char;
            if use_graphics != 0 {
                if graf_new != 0 {
                    let mut re_ptr: *mut monster_ego =
                        &mut *re_info.offset((*m_ptr).ego as isize) as
                            *mut monster_ego;
                    /* Desired attr */
                    *eap = (*re_ptr).g_attr;
                    /* Desired char */
                    *ecp = (*re_ptr).g_char
                }
                /* Use base monster */
                r_ptr_0 =
                    &mut *r_info.offset((*m_ptr).r_idx as isize) as
                        *mut monster_race
            }
            /* Desired attr/char */
            c = (*r_ptr_0).x_char as byte_hack;
            a = (*r_ptr_0).x_attr;
            /* Ignore weird codes */
            if avoid_other != 0 {
                /* Use char */
                *cp = c as libc::c_char;
                /* Use attr */
                *ap = a
            } else if attr_mutable == 0 {
                /* Special attr/char codes */
                /* Use char */
                *cp = c as libc::c_char;
                /* Use attr */
                *ap = a
            } else if (*r_ptr_0).flags1 & 0x80 as libc::c_int as libc::c_uint
                          != 0 {
                /* Multi-hued monster */
                /* Is it a shapechanger? */
                if (*r_ptr_0).flags2 & 0x400 as libc::c_int as libc::c_uint !=
                       0 {
                    image_random(ap, cp);
                } else { *cp = c as libc::c_char }
                /* Multi-hued attr */
                if (*r_ptr_0).flags2 & 0x800 as libc::c_int as libc::c_uint !=
                       0 {
                    *ap =
                        (Rand_div(15 as libc::c_int) + 1 as libc::c_int) as
                            byte_hack
                } else { *ap = multi_hued_attr(r_ptr_0) }
            } else if (*r_ptr_0).flags1 &
                          (0x40 as libc::c_int | 0x10 as libc::c_int) as
                              libc::c_uint == 0 {
                /* Normal monster (not "clear" in any way) */
                /* Use char */
                *cp = c as libc::c_char;
                /* Use attr */
                *ap = a
            } else if *ap as libc::c_int & 0x80 as libc::c_int != 0 {
                /*
				 * Hack -- Bizarre grid under monster
				 * WAS: else if (*ap & 0x80) || (*cp & 0x80) -- pelpel
				 */
                /* Use char */
                *cp = c as libc::c_char;
                /* Use attr */
                *ap = a
            } else if (*r_ptr_0).flags1 & 0x10 as libc::c_int as libc::c_uint
                          == 0 {
                /* Normal */
                /* Normal (non-clear char) monster */
                /* Normal char */
                *cp = c as libc::c_char
            } else if (*r_ptr_0).flags1 & 0x40 as libc::c_int as libc::c_uint
                          == 0 {
                /* Normal (non-clear attr) monster */
                /* Normal attr */
                *ap = a
            }
            /* Hack -- hallucination */
            if (*p_ptr).image != 0 {
                /* Hallucinatory monster */
                image_monster(ap, cp);
            }
        }
    }
    /* Handle "player" */
    if y == (*p_ptr).py as libc::c_int && x == (*p_ptr).px as libc::c_int &&
           ((*p_ptr).invis == 0 || (*p_ptr).see_inv as libc::c_int != 0) {
        let mut r_ptr_1: *mut monster_race =
            &mut *r_info.offset((*p_ptr).body_monster as isize) as
                *mut monster_race;
        /* Reset attr/char */
        *eap = 0 as libc::c_int as byte_hack;
        *ecp = 0 as libc::c_int as libc::c_char;
        /* Get the "player" attr */
        if avoid_other == 0 && attr_mutable as libc::c_int != 0 &&
               (*r_ptr_1).flags1 & 0x80 as libc::c_int as libc::c_uint != 0 {
            a = get_shimmer_color() as byte_hack
        } else { a = (*r_ptr_1).x_attr }
        /* Get the "player" char */
        c = (*r_ptr_1).x_char as byte_hack;
        /* Mega-Hack -- Apply modifications to player graphics XXX XXX XXX */
        match graphics_mode as libc::c_int {
            0 | 2 => {
                if player_char_health != 0 {
                    let mut percent: libc::c_int =
                        (*p_ptr).chp as libc::c_int * 10 as libc::c_int /
                            (*p_ptr).mhp as libc::c_int;
                    if percent < 7 as libc::c_int {
                        c = (percent + '0' as i32) as byte_hack;
                        if percent < 3 as libc::c_int {
                            a = 12 as libc::c_int as byte_hack
                        }
                    }
                }
            }
            3 => {
                if player_symbols != 0 {
                    a =
                        (164 as libc::c_int + (*p_ptr).pclass as libc::c_int)
                            as byte_hack;
                    c =
                        (128 as libc::c_int + (*p_ptr).prace as libc::c_int)
                            as byte_hack
                }
            }
            5 | 4 => {
                if (*p_ptr).pracem != 0 {
                    let mut rmp_ptr: *mut player_race_mod =
                        &mut *race_mod_info.offset((*p_ptr).pracem as isize)
                            as *mut player_race_mod;
                    /* Desired attr */
                    *eap = (*rmp_ptr).g_attr;
                    /* Desired char */
                    *ecp = (*rmp_ptr).g_char
                }
                /* +AKH 20020421 - Health dispay for graphics, too */
                if player_char_health as libc::c_int != 0 &&
                       graphics_mode as libc::c_int == 4 as libc::c_int {
                    let mut percent_0: libc::c_int =
                        (*p_ptr).chp as libc::c_int * 14 as libc::c_int /
                            (*p_ptr).mhp as libc::c_int;
                    if percent_0 < 10 as libc::c_int {
                        *eap = 10 as libc::c_int as byte_hack;
                        *ecp =
                            (32 as libc::c_int + 14 as libc::c_int -
                                 percent_0) as libc::c_char
                    }
                }
            }
            _ => { }
        }
        /* Save the info */
        *ap = a;
        *cp = c as libc::c_char
    };
}
/*
 * Special version of map_info, for use by cmovie and HTML converter
 * to obtain pure-ASCII image of dungeon map
 */
#[no_mangle]
pub unsafe extern "C" fn map_info_default(mut y: libc::c_int,
                                          mut x: libc::c_int,
                                          mut ap: *mut byte_hack,
                                          mut cp: *mut libc::c_char) {
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut f_ptr: *mut feature_type = 0 as *mut feature_type;
    let mut this_o_idx: s16b = 0;
    let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
    let mut info: u16b = 0;
    let mut t_idx: s16b = 0;
    let mut feat: byte_hack = 0;
    let mut a: byte_hack = 0;
    let mut c: byte_hack = 0;
    let mut use_graphics_hack: bool_ = use_graphics;
    let mut graphics_mode_hack: byte_hack = graphics_mode;
    /* Temporarily disable graphics mode -- for some random effects XXX */
    use_graphics = 0 as libc::c_int as bool_;
    graphics_mode = 0 as libc::c_int as byte_hack;
    /* *** Preparation ****/
    /* Access the grid */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Cache some frequently used values */
    /* Grid info */
    info = (*c_ptr).info;
    /* Feature code */
    feat = (*c_ptr).feat;
    /* Apply "mimic" field */
    if (*c_ptr).mimic != 0 {
        feat = (*c_ptr).mimic
    } else { feat = (*f_info.offset(feat as isize)).mimic }
    /* Access floor */
    f_ptr = &mut *f_info.offset(feat as isize) as *mut feature_type;
    /* *** Layer 1 -- Terrain feature ****/
    /* Only memorised or visible grids are displayed */
    if info as libc::c_int & (0x1 as libc::c_int | 0x10 as libc::c_int) != 0 {
        /* *** Step 1 -- Retrieve base attr/char ****/
        /* 'Sane' terrain features */
        if feat as libc::c_int != 0x4a as libc::c_int {
            /* Default char */
            c = (*f_ptr).d_char as byte_hack;
            /* Default attr */
            a = (*f_ptr).d_attr
        } else {
            /* Mega-Hack 1 -- Building don't conform to f_info */
            c =
                (*st_info.offset((*c_ptr).special as isize)).d_char as
                    byte_hack;
            a = (*st_info.offset((*c_ptr).special as isize)).d_attr
        }
        /* Mega-Hack 2 -- stair to dungeon branch are purple */
        if (*c_ptr).special as libc::c_int != 0 &&
               (feat as libc::c_int == 0x7 as libc::c_int ||
                    feat as libc::c_int == 0x6 as libc::c_int) {
            a = 10 as libc::c_int as byte_hack
        }
        /* Mega-Hack 3 -- Traps don't have f_info entries either */
        if info as libc::c_int & 0x100 as libc::c_int != 0 &&
               feat as libc::c_int != 0xbd as libc::c_int {
            /* Trap index */
            t_idx = (*c_ptr).t_idx;
            /*
			 * If trap is set on a floor grid that is not
			 * one of "interesting" features, use a special
			 * symbol to display it. Check for doors is no longer
			 * necessary because they have REMEMBER flag now.
			 *
			 * Cave macros cannot be used safely here, because of
			 * c_ptr->mimic XXX XXX
			 */
            if (*f_ptr).flags1 as libc::c_long &
                   (0x10 as libc::c_long | 0x100 as libc::c_long) ==
                   0x10 as libc::c_long {
                c =
                    (*f_info.offset(0x11 as libc::c_int as isize)).d_char as
                        byte_hack
            }
            /* Add attr */
            a = (*t_info.offset(t_idx as isize)).color;
            /* Get a new color with a strange formula :) */
            if (*t_info.offset(t_idx as isize)).flags &
                   0x8 as libc::c_int as libc::c_uint != 0 {
                let mut tmp: s32b = 0;
                tmp =
                    dun_level as libc::c_int + dungeon_type as libc::c_int +
                        feat as libc::c_int;
                a = (tmp % 16 as libc::c_int) as byte_hack
            }
        }
        /* *** Step 2 -- Apply special random effects ****/
        if avoid_other == 0 {
            /* Special terrain effect */
            if (*c_ptr).effect != 0 {
                a =
                    spell_color(effects[(*c_ptr).effect as usize].type_0 as
                                    libc::c_int)
            } else if (*f_ptr).flags1 as libc::c_long &
                          0x20000 as libc::c_long != 0 {
                a = (*f_ptr).shimmer[Rand_div(7 as libc::c_int) as usize]
            }
        }
        /* Multi-hued attr */
        /*
		 * Step 3
		 *
		 * Special lighting effects, if specified and applicable
		 * This will never happen for
		 * - any grids in the overhead map
		 * - traps
		 * - (graphics modes) terrain features without corresponding
		 *   "darker" tiles.
		 *
		 * All the if's here are flag checks, so changed order shouldn't
		 * affect performance a lot, I hope...
		 */
        /* view_special_lite: lighting effects for boring features */
        if view_special_lite as libc::c_int != 0 &&
               (*f_ptr).flags1 as libc::c_long &
                   (0x10 as libc::c_long | 0x100 as libc::c_long) ==
                   0x10 as libc::c_long {
            if (*p_ptr).wild_mode == 0 &&
                   info as libc::c_int & 0x100 as libc::c_int == 0 {
                /* Handle "seen" grids */
                if info as libc::c_int & 0x10 as libc::c_int != 0 {
                    /* Only lit by "torch" light */
                    if view_yellow_lite as libc::c_int != 0 &&
                           info as libc::c_int & 0x2 as libc::c_int == 0 {
                        /* Use "yellow" */
                        a = 11 as libc::c_int as byte_hack
                    }
                } else if (*p_ptr).blind != 0 {
                    /* Handle "blind" */
                    /* Use darker colour */
                    a =
                        darker_attrs[(a as libc::c_int & 0xf as libc::c_int)
                                         as usize]
                } else if info as libc::c_int & 0x2 as libc::c_int == 0 {
                    /* Handle "dark" grids */
                    /* Use darkest colour */
                    a = 8 as libc::c_int as byte_hack
                } else if view_bright_lite != 0 {
                    /* "Out-of-sight" glowing grids -- handle "view_bright_lite" */
                    /* Use darker colour */
                    a =
                        dark_attrs[(a as libc::c_int & 0xf as libc::c_int) as
                                       usize]
                }
            }
        } else if view_granite_lite as libc::c_int != 0 &&
                      (*f_ptr).flags1 as libc::c_long &
                          (0x2 as libc::c_long | 0x1000 as libc::c_long) != 0
         {
            if (*p_ptr).wild_mode == 0 &&
                   info as libc::c_int & 0x100 as libc::c_int == 0 {
                /* view_granite_lite: lighting effects for walls and doors */
                /* Handle "seen" grids */
                if !(info as libc::c_int & 0x10 as libc::c_int != 0) {
                    /* Handle "blind" */
                    if (*p_ptr).blind != 0 {
                        /* Use darker colour */
                        a =
                            darker_attrs[(a as libc::c_int &
                                              0xf as libc::c_int) as usize]
                    } else if view_bright_lite != 0 {
                        /* Handle "view_bright_lite" */
                        /* Use darker colour */
                        a =
                            dark_attrs[(a as libc::c_int & 0xf as libc::c_int)
                                           as usize]
                    }
                }
            }
        }
    } else {
        /* Unknown grids */
        /* Access darkness */
        f_ptr =
            &mut *f_info.offset(0 as libc::c_int as isize) as
                *mut feature_type;
        a = (*f_ptr).d_attr;
        c = (*f_ptr).d_char as byte_hack
    }
    /* Default attr */
    /* Default char */
    /*
	 * Hack -- rare random hallucination
	 * Because we cannot be sure which is outer dungeon walls,
	 * the check for 'feat' has been removed
	 */
    if (*p_ptr).image as libc::c_int != 0 &&
           Rand_div(256 as libc::c_int) == 0 as libc::c_int {
        /* Hallucinate */
        image_random(ap, cp);
    }
    /* Save the info */
    *ap = a;
    *cp = c as libc::c_char;
    /* *** Layer 2 -- Objects ****/
    if feat as libc::c_int != 0xaf as libc::c_int {
        this_o_idx = (*c_ptr).o_idx;
        while this_o_idx != 0 {
            let mut o_ptr: *mut object_type = 0 as *mut object_type;
            /* Acquire object */
            o_ptr =
                &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
            /* Acquire next object */
            next_o_idx = (*o_ptr).next_o_idx;
            /* Memorized objects */
            if (*o_ptr).marked != 0 {
                /* Normal char */
                *cp =
                    if (*k_info.offset((*o_ptr).k_idx as isize)).flavor as
                           libc::c_int != 0 {
                        misc_to_char[(*k_info.offset((*o_ptr).k_idx as
                                                         isize)).flavor as
                                         usize] as libc::c_int
                    } else {
                        (*k_info.offset((*o_ptr).k_idx as isize)).d_char as
                            libc::c_int
                    } as libc::c_char;
                /* Normal attr */
                *ap =
                    if (*o_ptr).tval as libc::c_int == 102 as libc::c_int {
                        random_artifacts[(*o_ptr).sval as usize].attr as
                            libc::c_int
                    } else if (*k_info.offset((*o_ptr).k_idx as isize)).flavor
                                  as libc::c_int != 0 {
                        misc_to_attr[(*k_info.offset((*o_ptr).k_idx as
                                                         isize)).flavor as
                                         usize] as libc::c_int
                    } else {
                        (*k_info.offset((*o_ptr).k_idx as isize)).d_attr as
                            libc::c_int
                    } as byte_hack;
                /* Multi-hued attr */
                if avoid_other == 0 &&
                       (*k_info.offset((*o_ptr).k_idx as isize)).flags5 as
                           libc::c_long & 0x40 as libc::c_long != 0 {
                    *ap = get_shimmer_color() as byte_hack
                }
                /* Hack -- hallucination */
                if (*p_ptr).image != 0 { image_object(ap, cp); }
                break ;
            } else { this_o_idx = next_o_idx }
        }
    }
    /* *** Layer 3 -- Handle monsters ****/
    if (*c_ptr).m_idx != 0 {
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset((*c_ptr).m_idx as isize) as *mut monster_type;
        let mut r_ptr: *mut monster_race =
            if !(*m_ptr).sr_ptr.is_null() {
                (*m_ptr).sr_ptr
            } else {
                race_info_idx((*m_ptr).r_idx as libc::c_int,
                              (*m_ptr).ego as libc::c_int)
            };
        if (*r_ptr).flags9 & 0x8 as libc::c_int as libc::c_uint != 0 {
            let mut o_ptr_0: *mut object_type = 0 as *mut object_type;
            /* Acquire object */
            o_ptr_0 =
                &mut *o_list.offset((*m_ptr).hold_o_idx as isize) as
                    *mut object_type;
            /* Memorized objects */
            if (*o_ptr_0).marked != 0 {
                /* Normal char */
                *cp =
                    if (*k_info.offset((*o_ptr_0).k_idx as isize)).flavor as
                           libc::c_int != 0 {
                        misc_to_char[(*k_info.offset((*o_ptr_0).k_idx as
                                                         isize)).flavor as
                                         usize] as libc::c_int
                    } else {
                        (*k_info.offset((*o_ptr_0).k_idx as isize)).d_char as
                            libc::c_int
                    } as libc::c_char;
                /* Normal attr */
                *ap =
                    if (*o_ptr_0).tval as libc::c_int == 102 as libc::c_int {
                        random_artifacts[(*o_ptr_0).sval as usize].attr as
                            libc::c_int
                    } else if (*k_info.offset((*o_ptr_0).k_idx as
                                                  isize)).flavor as
                                  libc::c_int != 0 {
                        misc_to_attr[(*k_info.offset((*o_ptr_0).k_idx as
                                                         isize)).flavor as
                                         usize] as libc::c_int
                    } else {
                        (*k_info.offset((*o_ptr_0).k_idx as isize)).d_attr as
                            libc::c_int
                    } as byte_hack;
                /* Multi-hued attr */
                if avoid_other == 0 && use_graphics == 0 &&
                       (*k_info.offset((*o_ptr_0).k_idx as isize)).flags5 as
                           libc::c_long & 0x40 as libc::c_long != 0 {
                    *ap = get_shimmer_color() as byte_hack
                }
                /* Hack -- hallucination */
                if (*p_ptr).image != 0 { image_object(ap, cp); }
            }
        } else if (*m_ptr).ml != 0 {
            let mut r_ptr_0: *mut monster_race =
                if !(*m_ptr).sr_ptr.is_null() {
                    (*m_ptr).sr_ptr
                } else {
                    race_info_idx((*m_ptr).r_idx as libc::c_int,
                                  (*m_ptr).ego as libc::c_int)
                };
            /* Visible monster */
            /* Default attr/char */
            c = (*r_ptr_0).d_char as byte_hack;
            a = (*r_ptr_0).d_attr;
            /* Ignore weird codes */
            if avoid_other != 0 {
                /* Use char */
                *cp = c as libc::c_char;
                /* Use attr */
                *ap = a
            } else if (*r_ptr_0).flags1 & 0x80 as libc::c_int as libc::c_uint
                          != 0 {
                /* Multi-hued monster */
                /* Is it a shapechanger? */
                if (*r_ptr_0).flags2 & 0x400 as libc::c_int as libc::c_uint !=
                       0 {
                    image_random(ap, cp);
                } else { *cp = c as libc::c_char }
                /* Multi-hued attr */
                if (*r_ptr_0).flags2 & 0x800 as libc::c_int as libc::c_uint !=
                       0 {
                    *ap =
                        (Rand_div(15 as libc::c_int) + 1 as libc::c_int) as
                            byte_hack
                } else { *ap = multi_hued_attr(r_ptr_0) }
            } else if (*r_ptr_0).flags1 &
                          (0x40 as libc::c_int | 0x10 as libc::c_int) as
                              libc::c_uint == 0 {
                /* Normal monster (not "clear" in any way) */
                /* Use char */
                *cp = c as libc::c_char;
                /* Use attr */
                *ap = a
            } else if *ap as libc::c_int & 0x80 as libc::c_int != 0 ||
                          *cp as libc::c_int & 0x80 as libc::c_int != 0 {
                /* Hack -- Bizarre grid under monster */
                /* Use char */
                *cp = c as libc::c_char;
                /* Use attr */
                *ap = a
            } else if (*r_ptr_0).flags1 & 0x10 as libc::c_int as libc::c_uint
                          == 0 {
                /* Normal */
                /* Normal (non-clear char) monster */
                /* Normal char */
                *cp = c as libc::c_char
            } else if (*r_ptr_0).flags1 & 0x40 as libc::c_int as libc::c_uint
                          == 0 {
                /* Normal (non-clear attr) monster */
                /* Normal attr */
                *ap = a
            }
            /* Hack -- hallucination */
            if (*p_ptr).image != 0 {
                /* Hallucinatory monster */
                image_monster(ap, cp);
            }
        }
    }
    /* Handle "player" */
    if y == (*p_ptr).py as libc::c_int && x == (*p_ptr).px as libc::c_int &&
           ((*p_ptr).invis == 0 ||
                (*p_ptr).invis as libc::c_int != 0 &&
                    (*p_ptr).see_inv as libc::c_int != 0) {
        let mut r_ptr_1: *mut monster_race =
            &mut *r_info.offset((*p_ptr).body_monster as isize) as
                *mut monster_race;
        /* Get the "player" attr */
        if avoid_other == 0 &&
               (*r_ptr_1).flags1 & 0x80 as libc::c_int as libc::c_uint != 0 {
            a = get_shimmer_color() as byte_hack
        } else { a = (*r_ptr_1).d_attr }
        /* Get the "player" char */
        c = (*r_ptr_1).d_char as byte_hack;
        /* Save the info */
        *ap = a;
        *cp = c as libc::c_char
    }
    /* XXX Restore the graphics mode */
    use_graphics = use_graphics_hack;
    graphics_mode = graphics_mode_hack;
}
/*
 * Calculate panel colum of a location in the map
 */
unsafe extern "C" fn panel_col_of(mut col: libc::c_int) -> libc::c_int {
    col -= panel_col_min as libc::c_int;
    if use_bigtile != 0 { col *= 2 as libc::c_int }
    return col + 13 as libc::c_int;
}
/*
 * Moves the cursor to a given MAP (y,x) location
 */
#[no_mangle]
pub unsafe extern "C" fn move_cursor_relative(mut row: libc::c_int,
                                              mut col: libc::c_int) {
    /* Real co-ords convert to screen positions */
    row -= panel_row_prt as libc::c_int;
    /* Go there */
    Term_gotoxy(panel_col_of(col), row);
}
/*
 * Place an attr/char pair at the given map coordinate, if legal.
 */
#[no_mangle]
pub unsafe extern "C" fn print_rel(mut c: libc::c_char, mut a: byte_hack,
                                   mut y: libc::c_int, mut x: libc::c_int) {
    /* Paranoia -- Only do "legal" locations */
    if !(y >= panel_row_min as libc::c_int &&
             y <= panel_row_max as libc::c_int &&
             x >= panel_col_min as libc::c_int &&
             x <= panel_col_max as libc::c_int) {
        return
    }
    /* Draw the char using the attr */
    Term_draw(panel_col_of(x), y - panel_row_prt as libc::c_int, a, c);
    if use_bigtile != 0 {
        let mut c2: libc::c_char = 0;
        let mut a2: byte_hack = 0;
        if a as libc::c_int & 0x80 as libc::c_int != 0 {
            a2 = 255 as libc::c_int as byte_hack;
            c2 = 255 as libc::c_int as libc::c_char
        } else {
            a2 = 1 as libc::c_int as byte_hack;
            c2 = ' ' as i32 as libc::c_char
        }
        Term_draw(panel_col_of(x) + 1 as libc::c_int,
                  y - panel_row_prt as libc::c_int, a2, c2);
    };
}
/*
 * Memorize interesting viewable object/features in the given grid
 *
 * This function should only be called on "legal" grids.
 *
 * This function will memorize the object and/or feature in the given
 * grid, if they are (1) viewable and (2) interesting.  Note that all
 * objects are interesting, all terrain features except floors (and
 * invisible traps) are interesting, and floors (and invisible traps)
 * are interesting sometimes (depending on various options involving
 * the illumination of floor grids).
 *
 * The automatic memorization of all objects and non-floor terrain
 * features as soon as they are displayed allows incredible amounts
 * of optimization in various places, especially "map_info()".
 *
 * Note that the memorization of objects is completely separate from
 * the memorization of terrain features, preventing annoying floor
 * memorization when a detected object is picked up from a dark floor,
 * and object memorization when an object is dropped into a floor grid
 * which is memorized but out-of-sight.
 *
 * This function should be called every time the "memorization" of
 * a grid (or the object in a grid) is called into question, such
 * as when an object is created in a grid, when a terrain feature
 * "changes" from "floor" to "non-floor", when any grid becomes
 * "illuminated" or "viewable", and when a "floor" grid becomes
 * "torch-lit".
 *
 * Note the relatively efficient use of this function by the various
 * "update_view()" and "update_lite()" calls, to allow objects and
 * terrain features to be memorized (and drawn) whenever they become
 * viewable or illuminated in any way, but not when they "maintain"
 * or "lose" their previous viewability or illumination.
 *
 * Note the butchered "internal" version of "player_can_see_bold()",
 * optimized primarily for the most common cases, that is, for the
 * non-marked floor grids.
 */
#[no_mangle]
pub unsafe extern "C" fn note_spot(mut y: libc::c_int, mut x: libc::c_int) {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    let mut info: u16b = (*c_ptr).info;
    let mut this_o_idx: s16b = 0;
    let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
    /* Require "seen" flag */
    if info as libc::c_int & 0x10 as libc::c_int == 0 { return }
    /* Hack -- memorize objects */
    this_o_idx = (*c_ptr).o_idx;
    while this_o_idx != 0 {
        let mut o_ptr: *mut object_type =
            &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
        /* Acquire next object */
        next_o_idx = (*o_ptr).next_o_idx;
        /* Memorize objects */
        (*o_ptr).marked = 1 as libc::c_int as byte_hack;
        this_o_idx = next_o_idx
    }
    if (*c_ptr).m_idx != 0 {
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset((*c_ptr).m_idx as isize) as *mut monster_type;
        let mut r_ptr: *mut monster_race =
            if !(*m_ptr).sr_ptr.is_null() {
                (*m_ptr).sr_ptr
            } else {
                race_info_idx((*m_ptr).r_idx as libc::c_int,
                              (*m_ptr).ego as libc::c_int)
            };
        if (*r_ptr).flags9 & 0x8 as libc::c_int as libc::c_uint != 0 {
            let mut o_ptr_0: *mut object_type =
                &mut *o_list.offset((*m_ptr).hold_o_idx as isize) as
                    *mut object_type;
            (*o_ptr_0).marked = 1 as libc::c_int as byte_hack
        }
    }
    /* Hack -- memorize grids */
    if info as libc::c_int & 0x1 as libc::c_int == 0 {
        /* Memorise some "boring" grids */
        if (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long &
               0x10 as libc::c_long != 0 &&
               (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long
                   & 0x100 as libc::c_long == 0 {
            /* Option -- memorise certain floors */
            if info as libc::c_int & 0x100 as libc::c_int != 0 ||
                   info as libc::c_int & 0x2 as libc::c_int != 0 &&
                       view_perma_grids as libc::c_int != 0 ||
                   view_torch_grids as libc::c_int != 0 {
                /* Memorize */
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int | 0x1 as libc::c_int) as
                        u16b
            }
        } else {
            /* Memorise all "interesting" grids */
            /* Memorize */
            (*c_ptr).info =
                ((*c_ptr).info as libc::c_int | 0x1 as libc::c_int) as u16b
        }
    };
}
/*
 * Redraw (on the screen) a given MAP location
 *
 * This function should only be called on "legal" grids
 */
#[no_mangle]
pub unsafe extern "C" fn lite_spot(mut y: libc::c_int, mut x: libc::c_int) {
    let mut a: byte_hack = 0;
    let mut a2: byte_hack = 0;
    let mut c: byte_hack = 0;
    let mut c2: byte_hack = 0;
    let mut ta: byte_hack = 0;
    let mut tc: libc::c_char = 0;
    let mut ea: byte_hack = 0;
    let mut ec: libc::c_char = 0;
    /* Redraw if on screen */
    if y >= panel_row_min as libc::c_int && y <= panel_row_max as libc::c_int
           && x >= panel_col_min as libc::c_int &&
           x <= panel_col_max as libc::c_int {
        /* Examine the grid */
        map_info(y, x, &mut a, &mut c as *mut byte_hack as *mut libc::c_char,
                 &mut ta, &mut tc, &mut ea, &mut ec);
        /* Hack -- Queue it */
        Term_queue_char(panel_col_of(x), y - panel_row_prt as libc::c_int, a,
                        c as libc::c_char, ta, tc, ea, ec);
        if use_bigtile != 0 {
            if a as libc::c_int & 0x80 as libc::c_int != 0 {
                a2 = 255 as libc::c_int as byte_hack;
                c2 = 255 as libc::c_int as byte_hack
            } else {
                a2 = 1 as libc::c_int as byte_hack;
                c2 = ' ' as i32 as byte_hack
            }
            Term_queue_char(panel_col_of(x) + 1 as libc::c_int,
                            y - panel_row_prt as libc::c_int, a2,
                            c2 as libc::c_char, 0 as libc::c_int as byte_hack,
                            0 as libc::c_int as libc::c_char,
                            0 as libc::c_int as byte_hack,
                            0 as libc::c_int as libc::c_char);
        }
    };
}
/*
 * Prints the map of the dungeon
 *
 * Note that, for efficiency, we contain an "optimized" version
 * of both "lite_spot()" and "print_rel()", and that we use the
 * "lite_spot()" function to display the player grid, if needed.
 */
#[no_mangle]
pub unsafe extern "C" fn prt_map() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    /* Access the cursor state */
    Term_get_cursor(&mut v);
    /* Hide the cursor */
    Term_set_cursor(0 as libc::c_int);
    /* Dump the map */
    y = panel_row_min as libc::c_int;
    while y <= panel_row_max as libc::c_int {
        /* Scan the columns of row "y" */
        x = panel_col_min as libc::c_int;
        while x <= panel_col_max as libc::c_int {
            let mut a: byte_hack = 0;
            let mut a2: byte_hack = 0;
            let mut c: libc::c_char = 0;
            let mut c2: libc::c_char = 0;
            let mut ta: byte_hack = 0;
            let mut tc: libc::c_char = 0;
            let mut ea: byte_hack = 0;
            let mut ec: libc::c_char = 0;
            /* Determine what is there */
            map_info(y, x, &mut a, &mut c, &mut ta, &mut tc, &mut ea,
                     &mut ec);
            /* Efficiency -- Redraw that grid of the map */
            Term_queue_char(panel_col_of(x), y - panel_row_prt as libc::c_int,
                            a, c, ta, tc, ea, ec);
            if use_bigtile != 0 {
                if a as libc::c_int & 0x80 as libc::c_int != 0 {
                    a2 = 255 as libc::c_int as byte_hack;
                    c2 = 255 as libc::c_int as libc::c_char
                } else {
                    a2 = 1 as libc::c_int as byte_hack;
                    c2 = ' ' as i32 as libc::c_char
                }
                Term_queue_char(panel_col_of(x) + 1 as libc::c_int,
                                y - panel_row_prt as libc::c_int, a2, c2,
                                0 as libc::c_int as byte_hack,
                                0 as libc::c_int as libc::c_char,
                                0 as libc::c_int as byte_hack,
                                0 as libc::c_int as libc::c_char);
            }
            x += 1
        }
        y += 1
    }
    /* Display player */
    lite_spot((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int);
    /* Restore the cursor */
    Term_set_cursor(v);
}
/*
 * Display highest priority object in the RATIO by RATIO area
 */
/*
 * Display the entire map
 */
/*
 * Hack -- priority array (see below)
 *
 * Note that all "walls" always look like "secret doors" (see "map_info()").
 */
static mut priority_table: [[byte_hack; 2]; 36] =
    [[0 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack],
     [0x1 as libc::c_int as byte_hack, 5 as libc::c_int as byte_hack],
     [0x30 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack],
     [0x33 as libc::c_int as byte_hack, 11 as libc::c_int as byte_hack],
     [0x32 as libc::c_int as byte_hack, 12 as libc::c_int as byte_hack],
     [0x31 as libc::c_int as byte_hack, 13 as libc::c_int as byte_hack],
     [0x62 as libc::c_int as byte_hack, 14 as libc::c_int as byte_hack],
     [0x4 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack],
     [0x5 as libc::c_int as byte_hack, 15 as libc::c_int as byte_hack],
     [(0x20 as libc::c_int + 0 as libc::c_int) as byte_hack,
      17 as libc::c_int as byte_hack],
     [0x37 as libc::c_int as byte_hack, 19 as libc::c_int as byte_hack],
     [0x36 as libc::c_int as byte_hack, 19 as libc::c_int as byte_hack],
     [0x64 as libc::c_int as byte_hack, 19 as libc::c_int as byte_hack],
     [0xbb as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack],
     [0x54 as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack],
     [0x55 as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack],
     [0x56 as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack],
     [0x58 as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack],
     [0x59 as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack],
     [0x57 as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack],
     [0x60 as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack],
     [0x61 as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack],
     [0x5a as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack],
     [0x5b as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack],
     [0x5c as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack],
     [0x5d as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack],
     [0x5e as libc::c_int as byte_hack, 20 as libc::c_int as byte_hack],
     [0x2 as libc::c_int as byte_hack, 22 as libc::c_int as byte_hack],
     [0xf as libc::c_int as byte_hack, 22 as libc::c_int as byte_hack],
     [0x6 as libc::c_int as byte_hack, 25 as libc::c_int as byte_hack],
     [0x7 as libc::c_int as byte_hack, 25 as libc::c_int as byte_hack],
     [0xb4 as libc::c_int as byte_hack, 25 as libc::c_int as byte_hack],
     [0xb3 as libc::c_int as byte_hack, 25 as libc::c_int as byte_hack],
     [0xe as libc::c_int as byte_hack, 25 as libc::c_int as byte_hack],
     [0xd as libc::c_int as byte_hack, 25 as libc::c_int as byte_hack],
     [0 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack]];
/*
 * Hack -- a priority function (see below)
 */
unsafe extern "C" fn priority(mut a: byte_hack, mut c: libc::c_char)
 -> byte_hack {
    let mut i: libc::c_int = 0;
    let mut p0: libc::c_int = 0;
    let mut p1: libc::c_int = 0;
    let mut f_ptr: *mut feature_type = 0 as *mut feature_type;
    /* Scan the table */
    i = 0 as libc::c_int;
    loop  {
        /* Priority level */
        p1 =
            priority_table[i as usize][1 as libc::c_int as usize] as
                libc::c_int;
        /* End of table */
        if p1 == 0 { break ; }
        /* Feature index */
        p0 =
            priority_table[i as usize][0 as libc::c_int as usize] as
                libc::c_int;
        /* Access the feature */
        f_ptr = &mut *f_info.offset(p0 as isize) as *mut feature_type;
        /* Check character and attribute, accept matches */
        if (*f_ptr).x_char as libc::c_int == c as libc::c_int &&
               (*f_ptr).x_attr as libc::c_int == a as libc::c_int {
            return p1 as byte_hack
        }
        i += 1
    }
    /* Default */
    return 20 as libc::c_int as byte_hack;
}
/*
 * Display a "small-scale" map of the dungeon in the active Term
 *
 * Note that the "map_info()" function must return fully colorized
 * data or this function will not work correctly.
 *
 * Note that this function must "disable" the special lighting
 * effects so that the "priority" function will work.
 *
 * Note the use of a specialized "priority" function to allow this
 * function to work with any graphic attr/char mappings, and the
 * attempts to optimize this function where possible.
 */
#[no_mangle]
pub unsafe extern "C" fn display_map(mut cy: *mut libc::c_int,
                                     mut cx: *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut ta: byte_hack = 0;
    let mut tc: libc::c_char = 0;
    let mut tp: byte_hack = 0;
    let mut ma: *mut *mut byte_hack = 0 as *mut *mut byte_hack;
    let mut mc: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut mp: *mut *mut byte_hack = 0 as *mut *mut byte_hack;
    let mut old_view_special_lite: bool_ = 0;
    let mut old_view_granite_lite: bool_ = 0;
    let mut hgt: libc::c_int = 0;
    let mut wid: libc::c_int = 0;
    let mut yrat: libc::c_int = 0;
    let mut xrat: libc::c_int = 0;
    let mut yfactor: libc::c_int = 0;
    let mut xfactor: libc::c_int = 0;
    /* Obtain current size of the Angband window */
    Term_get_size(&mut wid, &mut hgt);
    /* Use two characters as one tile in Bigtile mode */
    if use_bigtile != 0 { wid /= 2 as libc::c_int }
    /*
	 * Calculate the size of the dungeon map area
	 */
    hgt -= 1 as libc::c_int + 2 as libc::c_int;
    wid -= 13 as libc::c_int + 1 as libc::c_int;
    /* Paranoia */
    if hgt < 3 as libc::c_int || wid < 3 as libc::c_int {
        /* Map is too small, but place the player anyway */
        *cy = 1 as libc::c_int;
        *cx = 13 as libc::c_int;
        return
    }
    /* Save lighting effects */
    old_view_special_lite = view_special_lite;
    old_view_granite_lite = view_granite_lite;
    /* Disable lighting effects */
    view_special_lite = 0 as libc::c_int as bool_;
    view_granite_lite = 0 as libc::c_int as bool_;
    /* Allocate temporary memory for the maps */
    ma =
        memset(ralloc(((hgt + 2 as libc::c_int) as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut byte_hack>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((hgt + 2 as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut byte_hack>()
                                                    as libc::c_ulong)) as
            *mut *mut byte_hack;
    mc =
        memset(ralloc(((hgt + 2 as libc::c_int) as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((hgt + 2 as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut *mut libc::c_char;
    mp =
        memset(ralloc(((hgt + 2 as libc::c_int) as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut byte_hack>()
                                                           as libc::c_ulong))
                   as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ((hgt + 2 as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut byte_hack>()
                                                    as libc::c_ulong)) as
            *mut *mut byte_hack;
    /* Allocate each line in the maps */
    i = 0 as libc::c_int;
    while i < hgt + 2 as libc::c_int {
        let ref mut fresh0 = *ma.offset(i as isize);
        *fresh0 =
            memset(ralloc(((wid + 2 as libc::c_int) as
                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                               as
                                                               libc::c_ulong))
                       as *mut libc::c_char as *mut libc::c_void,
                   0 as libc::c_int,
                   ((wid + 2 as libc::c_int) as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                        as libc::c_ulong)) as
                *mut byte_hack;
        let ref mut fresh1 = *mc.offset(i as isize);
        *fresh1 =
            memset(ralloc(((wid + 2 as libc::c_int) as
                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                               as
                                                               libc::c_ulong))
                       as *mut libc::c_char as *mut libc::c_void,
                   0 as libc::c_int,
                   ((wid + 2 as libc::c_int) as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                        as libc::c_ulong)) as
                *mut libc::c_char;
        let ref mut fresh2 = *mp.offset(i as isize);
        *fresh2 =
            memset(ralloc(((wid + 2 as libc::c_int) as
                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                               as
                                                               libc::c_ulong))
                       as *mut libc::c_char as *mut libc::c_void,
                   0 as libc::c_int,
                   ((wid + 2 as libc::c_int) as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                        as libc::c_ulong)) as
                *mut byte_hack;
        i += 1
    }
    /* Clear the chars and attributes */
    y = 0 as libc::c_int;
    while y < hgt + 2 as libc::c_int {
        x = 0 as libc::c_int;
        while x < wid + 2 as libc::c_int {
            /* Nothing here */
            *(*ma.offset(y as isize)).offset(x as isize) =
                1 as libc::c_int as byte_hack;
            *(*mc.offset(y as isize)).offset(x as isize) =
                ' ' as i32 as libc::c_char;
            /* No priority */
            *(*mp.offset(y as isize)).offset(x as isize) =
                0 as libc::c_int as byte_hack;
            x += 1
        }
        y += 1
    }
    /* Calculate scaling factors */
    yfactor =
        if cur_hgt as libc::c_int / hgt < 4 as libc::c_int &&
               cur_hgt as libc::c_int > hgt {
            10 as libc::c_int
        } else { 1 as libc::c_int };
    xfactor =
        if cur_wid as libc::c_int / wid < 4 as libc::c_int &&
               cur_wid as libc::c_int > wid {
            10 as libc::c_int
        } else { 1 as libc::c_int };
    yrat =
        (cur_hgt as libc::c_int * yfactor + (hgt - 1 as libc::c_int)) / hgt;
    xrat =
        (cur_wid as libc::c_int * xfactor + (wid - 1 as libc::c_int)) / wid;
    /* Fill in the map */
    j = 0 as libc::c_int;
    while j < cur_hgt as libc::c_int {
        i = 0 as libc::c_int;
        while i < cur_wid as libc::c_int {
            /* Location */
            y = j * yfactor / yrat + 1 as libc::c_int;
            x = i * xfactor / xrat + 1 as libc::c_int;
            /* Extract the current attr/char at that map location */
            map_info(j, i, &mut ta, &mut tc, &mut ta, &mut tc, &mut ta,
                     &mut tc);
            /* Extract the priority of that attr/char */
            tp = priority(ta, tc);
            /* Player location has the highest priority */
            if (*p_ptr).py as libc::c_int == j &&
                   (*p_ptr).px as libc::c_int == i {
                tp = 255 as libc::c_int as byte_hack
            }
            /* Save "best" */
            if (*(*mp.offset(y as isize)).offset(x as isize) as libc::c_int) <
                   tp as libc::c_int {
                /* Save the char */
                *(*mc.offset(y as isize)).offset(x as isize) = tc;
                /* Save the attr */
                *(*ma.offset(y as isize)).offset(x as isize) = ta;
                /* Save priority */
                *(*mp.offset(y as isize)).offset(x as isize) = tp
            }
            i += 1
        }
        j += 1
    }
    /* Corners */
    y = hgt + 1 as libc::c_int;
    x = wid + 1 as libc::c_int;
    /* Draw the corners */
    let ref mut fresh3 = *(*mc.offset(y as isize)).offset(x as isize);
    *fresh3 = '+' as i32 as libc::c_char;
    let ref mut fresh4 =
        *(*mc.offset(y as isize)).offset(0 as libc::c_int as isize);
    *fresh4 = *fresh3;
    let ref mut fresh5 =
        *(*mc.offset(0 as libc::c_int as isize)).offset(x as isize);
    *fresh5 = *fresh4;
    *(*mc.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
        = *fresh5;
    /* Draw the horizontal edges */
    x = 1 as libc::c_int;
    while x <= wid {
        let ref mut fresh6 = *(*mc.offset(y as isize)).offset(x as isize);
        *fresh6 = '-' as i32 as libc::c_char;
        *(*mc.offset(0 as libc::c_int as isize)).offset(x as isize) = *fresh6;
        x += 1
    }
    /* Draw the vertical edges */
    y = 1 as libc::c_int;
    while y <= hgt {
        let ref mut fresh7 = *(*mc.offset(y as isize)).offset(x as isize);
        *fresh7 = '|' as i32 as libc::c_char;
        *(*mc.offset(y as isize)).offset(0 as libc::c_int as isize) = *fresh7;
        y += 1
    }
    /* Display each map line in order */
    y = 0 as libc::c_int;
    while y < hgt + 2 as libc::c_int {
        /* Start a new line */
        Term_gotoxy(13 as libc::c_int - 1 as libc::c_int, y);
        /* Display the line */
        x = 0 as libc::c_int;
        while x < wid + 2 as libc::c_int {
            ta = *(*ma.offset(y as isize)).offset(x as isize);
            tc = *(*mc.offset(y as isize)).offset(x as isize);
            /* Add the character */
            Term_addch(ta, tc);
            /* Double width tile mode requires filler */
            if use_bigtile != 0 {
                let mut a2: byte_hack = 0;
                let mut c2: libc::c_char = 0;
                if ta as libc::c_int & 0x80 as libc::c_int != 0 {
                    /* Mega-Hack */
                    a2 = 255 as libc::c_int as byte_hack;
                    c2 = 255 as libc::c_int as libc::c_char
                } else {
                    a2 = 1 as libc::c_int as byte_hack;
                    c2 = ' ' as i32 as libc::c_char
                }
                Term_addch(a2, c2);
            }
            x += 1
        }
        y += 1
    }
    /* Player location in dungeon */
    *cy = (*p_ptr).py as libc::c_int * yfactor / yrat + 1 as libc::c_int;
    if use_bigtile == 0 {
        *cx = (*p_ptr).px as libc::c_int * xfactor / xrat + 13 as libc::c_int
    } else {
        *cx =
            ((*p_ptr).px as libc::c_int * xfactor / xrat + 1 as libc::c_int) *
                2 as libc::c_int - 1 as libc::c_int + 13 as libc::c_int
    }
    /* Free each line in the maps */
    i = 0 as libc::c_int;
    while i < hgt + 2 as libc::c_int {
        rnfree(*ma.offset(i as isize) as vptr,
               ((wid + 2 as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                    as libc::c_ulong));
        rnfree(*mc.offset(i as isize) as vptr,
               ((wid + 2 as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong));
        rnfree(*mp.offset(i as isize) as vptr,
               ((wid + 2 as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte_hack>()
                                                    as libc::c_ulong));
        i += 1
    }
    /* Allocate temporary memory for the maps */
    rnfree(ma as vptr,
           ((hgt + 2 as libc::c_int) as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut byte_hack>()
                                                as libc::c_ulong));
    rnfree(mc as vptr,
           ((hgt + 2 as libc::c_int) as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                as libc::c_ulong));
    rnfree(mp as vptr,
           ((hgt + 2 as libc::c_int) as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut byte_hack>()
                                                as libc::c_ulong));
    /* Restore lighting effects */
    view_special_lite = old_view_special_lite;
    view_granite_lite = old_view_granite_lite;
}
/*
 * Display a "small-scale" map of the dungeon for the player
 *
 * Currently, the "player" is displayed on the map.  XXX XXX XXX
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_view_map() {
    let mut cy: libc::c_int = 0;
    let mut cx: libc::c_int = 0;
    let mut wid: libc::c_int = 0;
    let mut hgt: libc::c_int = 0;
    /* Retrive current screen size */
    Term_get_size(&mut wid, &mut hgt);
    /* Enter "icky" mode */
    character_icky = 1 as libc::c_int as bool_;
    /* Save the screen */
    Term_save();
    /* Note */
    prt(b"Please wait...\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int, 0 as libc::c_int);
    /* Flush */
    Term_fresh();
    /* Clear the screen */
    Term_clear();
    /* Display the map */
    display_map(&mut cy, &mut cx);
    /* Wait for it */
    put_str(b"Hit any key to continue\x00" as *const u8 as
                *const libc::c_char, hgt - 1 as libc::c_int,
            (wid - 13 as libc::c_int) / 2 as libc::c_int);
    /* Hilite the player */
    move_cursor(cy, cx);
    /* Get any key */
    inkey();
    /* Restore the screen */
    Term_load();
    /* Leave "icky" mode */
    character_icky = 0 as libc::c_int as bool_;
}
/*
 * The array of "vinfo" objects, initialized by "vinfo_init()"
 */
static mut vinfo: [vinfo_type; 161] =
    [vinfo_type{grid_y: [0; 8],
                grid_x: [0; 8],
                bits_3: 0,
                bits_2: 0,
                bits_1: 0,
                bits_0: 0,
                next_0: 0 as *const vinfo_type as *mut vinfo_type,
                next_1: 0 as *const vinfo_type as *mut vinfo_type,
                y: 0,
                x: 0,
                d: 0,
                r: 0,}; 161];
/*
 * Sorting hook -- comp function -- array of long's (see below)
 *
 * We use "u" to point to an array of long integers.
 */
unsafe extern "C" fn ang_sort_comp_hook_longs(mut u: vptr, mut v: vptr,
                                              mut a: libc::c_int,
                                              mut b: libc::c_int) -> bool_ {
    let mut x: *mut libc::c_long = u as *mut libc::c_long;
    return (*x.offset(a as isize) <= *x.offset(b as isize)) as libc::c_int as
               bool_;
}
/*
 * Sorting hook -- comp function -- array of long's (see below)
 *
 * We use "u" to point to an array of long integers.
 */
unsafe extern "C" fn ang_sort_swap_hook_longs(mut u: vptr, mut v: vptr,
                                              mut a: libc::c_int,
                                              mut b: libc::c_int) {
    let mut x: *mut libc::c_long = u as *mut libc::c_long;
    let mut temp: libc::c_long = 0;
    /* Swap */
    temp = *x.offset(a as isize);
    *x.offset(a as isize) = *x.offset(b as isize);
    *x.offset(b as isize) = temp;
}
/*
 * Save a slope
 */
unsafe extern "C" fn vinfo_init_aux(mut hack: *mut vinfo_hack,
                                    mut y: libc::c_int, mut x: libc::c_int,
                                    mut m: libc::c_long) {
    let mut i: libc::c_int = 0;
    /* Handle "legal" slopes */
    if m > 0 as libc::c_int as libc::c_long && m <= 100000 as libc::c_long {
        /* Look for that slope */
        i = 0 as libc::c_int;
        while i < (*hack).num_slopes {
            if (*hack).slopes[i as usize] == m { break ; }
            i += 1
        }
        /* New slope */
        if i == (*hack).num_slopes {
            /* Paranoia */
            if (*hack).num_slopes >= 126 as libc::c_int {
                quit_fmt(b"Too many slopes (%d)!\x00" as *const u8 as
                             *const libc::c_char, 126 as libc::c_int);
            }
            /* Save the slope, and advance */
            let fresh8 = (*hack).num_slopes;
            (*hack).num_slopes = (*hack).num_slopes + 1;
            (*hack).slopes[fresh8 as usize] = m
        }
    }
    /* Track slope range */
    if (*hack).slopes_min[y as usize][x as usize] > m {
        (*hack).slopes_min[y as usize][x as usize] = m
    }
    if (*hack).slopes_max[y as usize][x as usize] < m {
        (*hack).slopes_max[y as usize][x as usize] = m
    };
}
/*
 * Initialize the "vinfo" array
 *
 * Full Octagon (radius 20), Grids=1149
 *
 * Quadrant (south east), Grids=308, Slopes=251
 *
 * Octant (east then south), Grids=161, Slopes=126
 *
 * This function assumes that VINFO_MAX_GRIDS and VINFO_MAX_SLOPES
 * have the correct values, which can be derived by setting them to
 * a number which is too high, running this function, and using the
 * error messages to obtain the correct values.
 */
#[no_mangle]
pub unsafe extern "C" fn vinfo_init() -> errr {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut m: libc::c_long = 0;
    let mut hack: *mut vinfo_hack = 0 as *mut vinfo_hack;
    let mut num_grids: libc::c_int = 0 as libc::c_int;
    let mut queue_head: libc::c_int = 0 as libc::c_int;
    let mut queue_tail: libc::c_int = 0 as libc::c_int;
    let mut queue: [*mut vinfo_type; 322] = [0 as *mut vinfo_type; 322];
    /* Make hack */
    hack =
        memset(ralloc(::std::mem::size_of::<vinfo_hack>() as libc::c_ulong) as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<vinfo_hack>() as libc::c_ulong) as
            *mut vinfo_hack;
    /* Analyze grids */
    y = 0 as libc::c_int;
    while y <= 20 as libc::c_int {
        x = y;
        while x <= 20 as libc::c_int {
            /* Skip grids which are out of sight range */
            if !(distance(0 as libc::c_int, 0 as libc::c_int, y, x) >
                     20 as libc::c_int) {
                /* Default slope range */
                (*hack).slopes_min[y as usize][x as usize] =
                    999999999 as libc::c_int as libc::c_long;
                (*hack).slopes_max[y as usize][x as usize] =
                    0 as libc::c_int as libc::c_long;
                /* Paranoia */
                if num_grids >= 161 as libc::c_int {
                    quit_fmt(b"Too many grids (%d >= %d)!\x00" as *const u8 as
                                 *const libc::c_char, num_grids,
                             161 as libc::c_int);
                }
                /* Count grids */
                num_grids += 1;
                /* Slope to the top right corner */
                m =
                    100000 as libc::c_long *
                        (1000 as libc::c_long * y as libc::c_long -
                             500 as libc::c_int as libc::c_long) /
                        (1000 as libc::c_long * x as libc::c_long +
                             500 as libc::c_int as libc::c_long);
                /* Handle "legal" slopes */
                vinfo_init_aux(hack, y, x, m);
                /* Slope to top left corner */
                m =
                    100000 as libc::c_long *
                        (1000 as libc::c_long * y as libc::c_long -
                             500 as libc::c_int as libc::c_long) /
                        (1000 as libc::c_long * x as libc::c_long -
                             500 as libc::c_int as libc::c_long);
                /* Handle "legal" slopes */
                vinfo_init_aux(hack, y, x, m);
                /* Slope to bottom right corner */
                m =
                    100000 as libc::c_long *
                        (1000 as libc::c_long * y as libc::c_long +
                             500 as libc::c_int as libc::c_long) /
                        (1000 as libc::c_long * x as libc::c_long +
                             500 as libc::c_int as libc::c_long);
                /* Handle "legal" slopes */
                vinfo_init_aux(hack, y, x, m);
                /* Slope to bottom left corner */
                m =
                    100000 as libc::c_long *
                        (1000 as libc::c_long * y as libc::c_long +
                             500 as libc::c_int as libc::c_long) /
                        (1000 as libc::c_long * x as libc::c_long -
                             500 as libc::c_int as libc::c_long);
                /* Handle "legal" slopes */
                vinfo_init_aux(hack, y, x, m);
            }
            x += 1
        }
        y += 1
    }
    /* Enforce maximal efficiency */
    if num_grids < 161 as libc::c_int {
        quit_fmt(b"Too few grids (%d < %d)!\x00" as *const u8 as
                     *const libc::c_char, num_grids, 161 as libc::c_int);
    }
    /* Enforce maximal efficiency */
    if (*hack).num_slopes < 126 as libc::c_int {
        quit_fmt(b"Too few slopes (%d < %d)!\x00" as *const u8 as
                     *const libc::c_char, (*hack).num_slopes,
                 126 as libc::c_int);
    }
    /* Sort slopes numerically */
    ang_sort_comp =
        Some(ang_sort_comp_hook_longs as
                 unsafe extern "C" fn(_: vptr, _: vptr, _: libc::c_int,
                                      _: libc::c_int) -> bool_);
    /* Sort slopes numerically */
    ang_sort_swap =
        Some(ang_sort_swap_hook_longs as
                 unsafe extern "C" fn(_: vptr, _: vptr, _: libc::c_int,
                                      _: libc::c_int) -> ());
    /* Sort the (unique) slopes */
    ang_sort((*hack).slopes.as_mut_ptr() as vptr, 0 as *mut libc::c_void,
             (*hack).num_slopes);
    /* Enqueue player grid */
    let fresh9 = queue_tail;
    queue_tail = queue_tail + 1;
    queue[fresh9 as usize] =
        &mut *vinfo.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut vinfo_type;
    /* Process queue */
    while queue_head < queue_tail {
        let mut e: libc::c_int = 0;
        /* Index */
        e = queue_head;
        /* Dequeue next grid */
        queue_head += 1;
        /* Location of main grid */
        y =
            vinfo[e as usize].grid_y[0 as libc::c_int as usize] as
                libc::c_int;
        x =
            vinfo[e as usize].grid_x[0 as libc::c_int as usize] as
                libc::c_int;
        /* Compute grid offsets */
        vinfo[e as usize].grid_y[0 as libc::c_int as usize] = y as s16b;
        vinfo[e as usize].grid_x[0 as libc::c_int as usize] = x as s16b;
        vinfo[e as usize].grid_y[1 as libc::c_int as usize] = x as s16b;
        vinfo[e as usize].grid_x[1 as libc::c_int as usize] = y as s16b;
        vinfo[e as usize].grid_y[2 as libc::c_int as usize] = x as s16b;
        vinfo[e as usize].grid_x[2 as libc::c_int as usize] = -y as s16b;
        vinfo[e as usize].grid_y[3 as libc::c_int as usize] = y as s16b;
        vinfo[e as usize].grid_x[3 as libc::c_int as usize] = -x as s16b;
        vinfo[e as usize].grid_y[4 as libc::c_int as usize] = -y as s16b;
        vinfo[e as usize].grid_x[4 as libc::c_int as usize] = -x as s16b;
        vinfo[e as usize].grid_y[5 as libc::c_int as usize] = -x as s16b;
        vinfo[e as usize].grid_x[5 as libc::c_int as usize] = -y as s16b;
        vinfo[e as usize].grid_y[6 as libc::c_int as usize] = -x as s16b;
        vinfo[e as usize].grid_x[6 as libc::c_int as usize] = y as s16b;
        vinfo[e as usize].grid_y[7 as libc::c_int as usize] = -y as s16b;
        vinfo[e as usize].grid_x[7 as libc::c_int as usize] = x as s16b;
        /* Analyze slopes */
        i = 0 as libc::c_int;
        while i < (*hack).num_slopes {
            m = (*hack).slopes[i as usize];
            /* Memorize intersection slopes (for non-player-grids) */
            if e > 0 as libc::c_int &&
                   (*hack).slopes_min[y as usize][x as usize] < m &&
                   m < (*hack).slopes_max[y as usize][x as usize] {
                match i / 32 as libc::c_int {
                    3 => {
                        vinfo[e as usize].bits_3 =
                            (vinfo[e as usize].bits_3 as libc::c_long |
                                 (1 as libc::c_long) << i % 32 as libc::c_int)
                                as u32b
                    }
                    2 => {
                        vinfo[e as usize].bits_2 =
                            (vinfo[e as usize].bits_2 as libc::c_long |
                                 (1 as libc::c_long) << i % 32 as libc::c_int)
                                as u32b
                    }
                    1 => {
                        vinfo[e as usize].bits_1 =
                            (vinfo[e as usize].bits_1 as libc::c_long |
                                 (1 as libc::c_long) << i % 32 as libc::c_int)
                                as u32b
                    }
                    0 => {
                        vinfo[e as usize].bits_0 =
                            (vinfo[e as usize].bits_0 as libc::c_long |
                                 (1 as libc::c_long) << i % 32 as libc::c_int)
                                as u32b
                    }
                    _ => { }
                }
            }
            i += 1
        }
        /* Default */
        vinfo[e as usize].next_0 =
            &mut *vinfo.as_mut_ptr().offset(0 as libc::c_int as isize) as
                *mut vinfo_type;
        /* Grid next child */
        if distance(0 as libc::c_int, 0 as libc::c_int, y,
                    x + 1 as libc::c_int) <= 20 as libc::c_int {
            if (*queue[(queue_tail - 1 as libc::c_int) as
                           usize]).grid_y[0 as libc::c_int as usize] as
                   libc::c_int != y ||
                   (*queue[(queue_tail - 1 as libc::c_int) as
                               usize]).grid_x[0 as libc::c_int as usize] as
                       libc::c_int != x + 1 as libc::c_int {
                vinfo[queue_tail as usize].grid_y[0 as libc::c_int as usize] =
                    y as s16b;
                vinfo[queue_tail as usize].grid_x[0 as libc::c_int as usize] =
                    (x + 1 as libc::c_int) as s16b;
                queue[queue_tail as usize] =
                    &mut *vinfo.as_mut_ptr().offset(queue_tail as isize) as
                        *mut vinfo_type;
                queue_tail += 1
            }
            vinfo[e as usize].next_0 =
                &mut *vinfo.as_mut_ptr().offset((queue_tail -
                                                     1 as libc::c_int) as
                                                    isize) as *mut vinfo_type
        }
        /* Default */
        vinfo[e as usize].next_1 =
            &mut *vinfo.as_mut_ptr().offset(0 as libc::c_int as isize) as
                *mut vinfo_type;
        /* Grid diag child */
        if distance(0 as libc::c_int, 0 as libc::c_int, y + 1 as libc::c_int,
                    x + 1 as libc::c_int) <= 20 as libc::c_int {
            if (*queue[(queue_tail - 1 as libc::c_int) as
                           usize]).grid_y[0 as libc::c_int as usize] as
                   libc::c_int != y + 1 as libc::c_int ||
                   (*queue[(queue_tail - 1 as libc::c_int) as
                               usize]).grid_x[0 as libc::c_int as usize] as
                       libc::c_int != x + 1 as libc::c_int {
                vinfo[queue_tail as usize].grid_y[0 as libc::c_int as usize] =
                    (y + 1 as libc::c_int) as s16b;
                vinfo[queue_tail as usize].grid_x[0 as libc::c_int as usize] =
                    (x + 1 as libc::c_int) as s16b;
                queue[queue_tail as usize] =
                    &mut *vinfo.as_mut_ptr().offset(queue_tail as isize) as
                        *mut vinfo_type;
                queue_tail += 1
            }
            vinfo[e as usize].next_1 =
                &mut *vinfo.as_mut_ptr().offset((queue_tail -
                                                     1 as libc::c_int) as
                                                    isize) as *mut vinfo_type
        }
        /* Hack -- main diagonal has special children */
        if y == x { vinfo[e as usize].next_0 = vinfo[e as usize].next_1 }
        /* Extra values */
        vinfo[e as usize].y = y as byte_hack;
        vinfo[e as usize].x = x as byte_hack;
        vinfo[e as usize].d =
            if y > x {
                (y) + x / 2 as libc::c_int
            } else { (x) + y / 2 as libc::c_int } as byte_hack;
        vinfo[e as usize].r =
            if y == 0 {
                x
            } else if x == 0 {
                y
            } else if y == x { y } else { 0 as libc::c_int } as byte_hack
    }
    /* Verify maximal bits XXX XXX XXX */
    if vinfo[1 as libc::c_int as usize].bits_3 |
           vinfo[2 as libc::c_int as usize].bits_3 !=
           0x3fffffff as libc::c_int as libc::c_uint ||
           vinfo[1 as libc::c_int as usize].bits_2 |
               vinfo[2 as libc::c_int as usize].bits_2 !=
               0xffffffff as libc::c_uint ||
           vinfo[1 as libc::c_int as usize].bits_1 |
               vinfo[2 as libc::c_int as usize].bits_1 !=
               0xffffffff as libc::c_uint ||
           vinfo[1 as libc::c_int as usize].bits_0 |
               vinfo[2 as libc::c_int as usize].bits_0 !=
               0xffffffff as libc::c_uint {
        quit(b"Incorrect bit masks!\x00" as *const u8 as *const libc::c_char);
    }
    /* Kill hack */
    hack =
        rnfree(hack as vptr,
               ::std::mem::size_of::<vinfo_hack>() as libc::c_ulong) as
            *mut vinfo_hack;
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Forget the "CAVE_VIEW" grids, redrawing as needed
 */
#[no_mangle]
pub unsafe extern "C" fn forget_view() {
    let mut i: libc::c_int = 0;
    let mut fast_view_n: libc::c_int = view_n as libc::c_int;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* None to forget */
    if fast_view_n == 0 { return }
    /* Clear them all */
    i = 0 as libc::c_int;
    while i < fast_view_n {
        let mut y: libc::c_int = view_y[i as usize] as libc::c_int;
        let mut x: libc::c_int = view_x[i as usize] as libc::c_int;
        /* Access the grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* Clear "CAVE_VIEW", "CAVE_SEEN" and player torch flags */
        (*c_ptr).info =
            ((*c_ptr).info as libc::c_int &
                 !(0x20 as libc::c_int | 0x10 as libc::c_int |
                       0x2000 as libc::c_int)) as u16b;
        /* Redraw */
        lite_spot(y, x);
        i += 1
    }
    /* None left */
    view_n = 0 as libc::c_int as s16b;
}
/*
 * Calculate the complete field of view using a new algorithm
 *
 * If "view_y/x" and "temp_y/x" were global pointers to arrays of grids, as
 * opposed to actual arrays of grids, then we could be more efficient by
 * using "pointer swapping".
 *
 * Normally, vision along the major axes is more likely than vision
 * along the diagonal axes, so we check the bits corresponding to
 * the lines of sight near the major axes first.
 *
 * We use the "temp_y/x" array (and the "CAVE_TEMP" flag) to keep track of
 * which grids were previously marked "CAVE_SEEN", since only those grids
 * whose "CAVE_SEEN" value changes during this routine must be redrawn.
 *
 * This function is now responsible for maintaining the "CAVE_SEEN"
 * flags as well as the "CAVE_VIEW" flags, which is good, because
 * the only grids which normally need to be memorized and/or redrawn
 * are the ones whose "CAVE_SEEN" flag changes during this routine.
 *
 * Basically, this function divides the "octagon of view" into octants of
 * grids (where grids on the main axes and diagonal axes are "shared" by
 * two octants), and processes each octant one at a time, processing each
 * octant one grid at a time, processing only those grids which "might" be
 * viewable, and setting the "CAVE_VIEW" flag for each grid for which there
 * is an (unobstructed) line of sight from the center of the player grid to
 * any internal point in the grid (and collecting these "CAVE_VIEW" grids
 * into the "view_y/x" array), and setting the "CAVE_SEEN" flag for the grid
 * if, in addition, the grid is "illuminated" in some way.
 *
 * This function relies on a theorem (suggested and proven by Mat Hostetter)
 * which states that in each octant of a field of view, a given grid will
 * be "intersected" by one or more unobstructed "lines of sight" from the
 * center of the player grid if and only if it is "intersected" by at least
 * one such unobstructed "line of sight" which passes directly through some
 * corner of some grid in the octant which is not shared by any other octant.
 * The proof is based on the fact that there are at least three significant
 * lines of sight involving any non-shared grid in any octant, one which
 * intersects the grid and passes though the corner of the grid closest to
 * the player, and two which "brush" the grid, passing through the "outer"
 * corners of the grid, and that any line of sight which intersects a grid
 * without passing through the corner of a grid in the octant can be "slid"
 * slowly towards the corner of the grid closest to the player, until it
 * either reaches it or until it brushes the corner of another grid which
 * is closer to the player, and in either case, the existanc of a suitable
 * line of sight is thus demonstrated.
 *
 * It turns out that in each octant of the radius 20 "octagon of view",
 * there are 161 grids (with 128 not shared by any other octant), and there
 * are exactly 126 distinct "lines of sight" passing from the center of the
 * player grid through any corner of any non-shared grid in the octant.  To
 * determine if a grid is "viewable" by the player, therefore, you need to
 * simply show that one of these 126 lines of sight intersects the grid but
 * does not intersect any wall grid closer to the player.  So we simply use
 * a bit vector with 126 bits to represent the set of interesting lines of
 * sight which have not yet been obstructed by wall grids, and then we scan
 * all the grids in the octant, moving outwards from the player grid.  For
 * each grid, if any of the lines of sight which intersect that grid have not
 * yet been obstructed, then the grid is viewable.  Furthermore, if the grid
 * is a wall grid, then all of the lines of sight which intersect the grid
 * should be marked as obstructed for future reference.  Also, we only need
 * to check those grids for whom at least one of the "parents" was a viewable
 * non-wall grid, where the parents include the two grids touching the grid
 * but closer to the player grid (one adjacent, and one diagonal).  For the
 * bit vector, we simply use 4 32-bit integers.  All of the static values
 * which are needed by this function are stored in the large "vinfo" array
 * (above), which is machine generated by another program.  XXX XXX XXX
 *
 * Hack -- The queue must be able to hold more than VINFO_MAX_GRIDS grids
 * because the grids at the edge of the field of view use "grid zero" as
 * their children, and the queue must be able to hold several of these
 * special grids.  Because the actual number of required grids is bizarre,
 * we simply allocate twice as many as we would normally need.  XXX XXX XXX
 */
#[no_mangle]
pub unsafe extern "C" fn update_view() {
    let mut i: libc::c_int = 0;
    let mut o: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut radius: libc::c_int = 0;
    let mut fast_view_n: libc::c_int = view_n as libc::c_int;
    let mut fast_temp_n: libc::c_int = 0 as libc::c_int;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut info: u16b = 0;
    /* ** Step 0 -- Begin ***/
    /* Save the old "view" grids for later */
    i = 0 as libc::c_int;
    while i < fast_view_n {
        /* Location */
        y = view_y[i as usize] as libc::c_int;
        x = view_x[i as usize] as libc::c_int;
        /* Grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* Get grid info */
        info = (*c_ptr).info;
        /* Save "CAVE_SEEN" grids */
        if info as libc::c_int & 0x10 as libc::c_int != 0 {
            /* Set "CAVE_TEMP" flag */
            info = (info as libc::c_int | 0x40 as libc::c_int) as u16b;
            /* Save grid for later */
            temp_y[fast_temp_n as usize] = y as byte_hack;
            let fresh10 = fast_temp_n;
            fast_temp_n = fast_temp_n + 1;
            temp_x[fresh10 as usize] = x as byte_hack
        }
        /* Clear "CAVE_VIEW", "CAVE_SEEN" and player torch flags */
        info =
            (info as libc::c_int &
                 !(0x20 as libc::c_int | 0x10 as libc::c_int |
                       0x2000 as libc::c_int)) as u16b;
        /* Save cave info */
        (*c_ptr).info = info;
        i += 1
    }
    /* Reset the "view" array */
    fast_view_n = 0 as libc::c_int;
    /* Extract "radius" value */
    radius = (*p_ptr).cur_lite as libc::c_int;
    /* Handle real light */
    if radius > 0 as libc::c_int { radius += 1 }
    /* ** Step 1 -- player grid ***/
    /* Player grid */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset((*p_ptr).py as
                                             isize)).offset((*p_ptr).px as
                                                                isize) as
            *mut cave_type;
    /* Get grid info */
    info = (*c_ptr).info;
    /* Assume viewable */
    info = (info as libc::c_int | 0x20 as libc::c_int) as u16b;
    /* Torch-lit grid */
    if (0 as libc::c_int) < radius {
        /* Mark as "CAVE_SEEN" and torch-lit */
        info =
            (info as libc::c_int |
                 (0x10 as libc::c_int | 0x2000 as libc::c_int)) as u16b
    } else if info as libc::c_int & 0x2 as libc::c_int != 0 {
        /* Perma-lit grid */
        /* Mark as "CAVE_SEEN" */
        info = (info as libc::c_int | 0x10 as libc::c_int) as u16b
    }
    /* Save cave info */
    (*c_ptr).info = info;
    /* Save in array */
    view_y[fast_view_n as usize] = (*p_ptr).py as byte_hack;
    let fresh11 = fast_view_n;
    fast_view_n = fast_view_n + 1;
    view_x[fresh11 as usize] = (*p_ptr).px as byte_hack;
    /* ** Step 2 -- octants ***/
    /* Scan each octant */
    o = 0 as libc::c_int;
    while o < 8 as libc::c_int {
        let mut p: *mut vinfo_type = 0 as *mut vinfo_type;
        /* Last added */
        let mut last: *mut vinfo_type =
            &mut *vinfo.as_mut_ptr().offset(0 as libc::c_int as isize) as
                *mut vinfo_type;
        /* Grid queue */
        let mut queue_head: libc::c_int = 0 as libc::c_int;
        let mut queue_tail: libc::c_int = 0 as libc::c_int;
        let mut queue: [*mut vinfo_type; 322] = [0 as *mut vinfo_type; 322];
        /* Slope bit vector */
        let mut bits0: u32b = 0xffffffff as libc::c_uint;
        let mut bits1: u32b = 0xffffffff as libc::c_uint;
        let mut bits2: u32b = 0xffffffff as libc::c_uint;
        let mut bits3: u32b = 0x3fffffff as libc::c_int as u32b;
        /* Reset queue */
        queue_tail = 0 as libc::c_int;
        queue_head = queue_tail;
        /* Initial grids */
        let fresh12 = queue_tail;
        queue_tail = queue_tail + 1;
        queue[fresh12 as usize] =
            &mut *vinfo.as_mut_ptr().offset(1 as libc::c_int as isize) as
                *mut vinfo_type;
        let fresh13 = queue_tail;
        queue_tail = queue_tail + 1;
        queue[fresh13 as usize] =
            &mut *vinfo.as_mut_ptr().offset(2 as libc::c_int as isize) as
                *mut vinfo_type;
        /* Process queue */
        while queue_head < queue_tail {
            /* Dequeue next grid */
            let fresh14 = queue_head;
            queue_head = queue_head + 1;
            p = queue[fresh14 as usize];
            /* Check bits */
            if bits0 & (*p).bits_0 != 0 || bits1 & (*p).bits_1 != 0 ||
                   bits2 & (*p).bits_2 != 0 || bits3 & (*p).bits_3 != 0 {
                /* Extract coordinate value */
                y =
                    (*p_ptr).py as libc::c_int +
                        (*p).grid_y[o as usize] as libc::c_int;
                x =
                    (*p_ptr).px as libc::c_int +
                        (*p).grid_x[o as usize] as libc::c_int;
                /* Access the grid */
                c_ptr =
                    &mut *(*cave.as_mut_ptr().offset(y as
                                                         isize)).offset(x as
                                                                            isize)
                        as *mut cave_type;
                /* Get grid info */
                info = (*c_ptr).info;
                /* Handle wall */
                if info as libc::c_int & 0x80 as libc::c_int != 0 {
                    /* Clear bits */
                    bits0 &= !(*p).bits_0;
                    bits1 &= !(*p).bits_1;
                    bits2 &= !(*p).bits_2;
                    bits3 &= !(*p).bits_3;
                    /* Newly viewable wall */
                    if info as libc::c_int & 0x20 as libc::c_int == 0 {
                        /* Mark as viewable */
                        info =
                            (info as libc::c_int | 0x20 as libc::c_int) as
                                u16b;
                        /* Torch-lit grids */
                        if ((*p).d as libc::c_int) < radius {
                            /* Mark as "CAVE_SEEN" and torch-lit */
                            info =
                                (info as libc::c_int |
                                     (0x10 as libc::c_int |
                                          0x2000 as libc::c_int)) as u16b
                        } else if info as libc::c_int & 0x4000 as libc::c_int
                                      != 0 {
                            /* Monster-lit grids */
                            /* Mark as "CAVE_SEEN" */
                            info =
                                (info as libc::c_int | 0x10 as libc::c_int) as
                                    u16b
                        } else if info as libc::c_int & 0x2 as libc::c_int !=
                                      0 {
                            /* Perma-lit grids */
                            /* Hack -- move towards player */
                            let mut yy: libc::c_int =
                                if y < (*p_ptr).py as libc::c_int {
                                    (y) + 1 as libc::c_int
                                } else if y > (*p_ptr).py as libc::c_int {
                                    (y) - 1 as libc::c_int
                                } else { y };
                            let mut xx: libc::c_int =
                                if x < (*p_ptr).px as libc::c_int {
                                    (x) + 1 as libc::c_int
                                } else if x > (*p_ptr).px as libc::c_int {
                                    (x) - 1 as libc::c_int
                                } else { x };
                            /* UPDATE_VIEW_COMPLEX_WALL_ILLUMINATION */
                            /* Check for "simple" illumination */
                            if (*cave[yy as usize].offset(xx as isize)).info
                                   as libc::c_int & 0x2 as libc::c_int != 0 {
                                /* Mark as seen */
                                info =
                                    (info as libc::c_int |
                                         0x10 as libc::c_int) as u16b
                            }
                        }
                        /* Save cave info */
                        (*c_ptr).info = info;
                        /* Save in array */
                        view_y[fast_view_n as usize] = y as byte_hack;
                        let fresh15 = fast_view_n;
                        fast_view_n = fast_view_n + 1;
                        view_x[fresh15 as usize] = x as byte_hack
                    }
                } else {
                    /* Handle non-wall */
                    /* Enqueue child */
                    if last != (*p).next_0 {
                        last = (*p).next_0;
                        let fresh16 = queue_tail;
                        queue_tail = queue_tail + 1;
                        queue[fresh16 as usize] = last
                    }
                    if last != (*p).next_1 {
                        last = (*p).next_1;
                        let fresh17 = queue_tail;
                        queue_tail = queue_tail + 1;
                        queue[fresh17 as usize] = last
                    }
                    if info as libc::c_int & 0x20 as libc::c_int == 0 {
                        /* Enqueue child */
                        /* Mark as "viewable" */
                        info =
                            (info as libc::c_int | 0x20 as libc::c_int) as
                                u16b;
                        /* Torch-lit grids */
                        if ((*p).d as libc::c_int) < radius {
                            /* Mark as "CAVE_SEEN" and torch-lit */
                            info =
                                (info as libc::c_int |
                                     (0x10 as libc::c_int |
                                          0x2000 as libc::c_int)) as u16b
                        } else if info as libc::c_int &
                                      (0x2 as libc::c_int |
                                           0x4000 as libc::c_int) != 0 {
                            /* Perma-lit or monster-lit grids */
                            /* Mark as "CAVE_SEEN" */
                            info =
                                (info as libc::c_int | 0x10 as libc::c_int) as
                                    u16b
                        }
                        /* Save cave info */
                        (*c_ptr).info = info;
                        /* Save in array */
                        view_y[fast_view_n as usize] = y as byte_hack;
                        let fresh18 = fast_view_n;
                        fast_view_n = fast_view_n + 1;
                        view_x[fresh18 as usize] = x as byte_hack
                    }
                }
            }
        }
        o += 1
    }
    /* Newly viewable non-wall */
    /* ** Step 3 -- Complete the algorithm ***/
    /* Handle blindness */
    if (*p_ptr).blind != 0 {
        /* Process "new" grids */
        i = 0 as libc::c_int;
        while i < fast_view_n {
            /* Location */
            y = view_y[i as usize] as libc::c_int;
            x = view_x[i as usize] as libc::c_int;
            /* Grid cannot be "CAVE_SEEN" */
            let ref mut fresh19 = (*cave[y as usize].offset(x as isize)).info;
            *fresh19 =
                (*fresh19 as libc::c_int & !(0x10 as libc::c_int)) as u16b;
            i += 1
        }
    }
    /* Process "new" grids */
    i = 0 as libc::c_int;
    while i < fast_view_n {
        /* Location */
        y = view_y[i as usize] as libc::c_int;
        x = view_x[i as usize] as libc::c_int;
        /* Get grid info */
        info = (*cave[y as usize].offset(x as isize)).info;
        /* Was not "CAVE_SEEN", is now "CAVE_SEEN" */
        if info as libc::c_int & 0x10 as libc::c_int != 0 &&
               info as libc::c_int & 0x40 as libc::c_int == 0 {
            /* Note */
            note_spot(y, x);
            /* Redraw */
            lite_spot(y, x);
        }
        i += 1
    }
    /* Process "old" grids */
    i = 0 as libc::c_int;
    while i < fast_temp_n {
        /* Location */
        y = temp_y[i as usize] as libc::c_int;
        x = temp_x[i as usize] as libc::c_int;
        /* Grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* Get grid info */
        info = (*c_ptr).info;
        /* Clear "CAVE_TEMP" flag */
        info = (info as libc::c_int & !(0x40 as libc::c_int)) as u16b;
        /* Save cave info */
        (*c_ptr).info = info;
        /* Was "CAVE_SEEN", is now not "CAVE_SEEN" */
        if info as libc::c_int & 0x10 as libc::c_int == 0 {
            /* Redraw */
            lite_spot(y, x);
        }
        i += 1
    }
    /* Save 'view_n' */
    view_n = fast_view_n as s16b;
}
/*
 * Clear monster light 
 */
#[no_mangle]
pub unsafe extern "C" fn forget_mon_lite() {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    /* Process all the monster-lit grids */
    i = 0 as libc::c_int;
    while i < lite_n as libc::c_int {
        /* Access location */
        y = lite_y[i as usize] as libc::c_int;
        x = lite_x[i as usize] as libc::c_int;
        /* Clear monster light flag */
        let ref mut fresh20 = (*cave[y as usize].offset(x as isize)).info;
        *fresh20 =
            (*fresh20 as libc::c_int & !(0x4000 as libc::c_int)) as u16b;
        i += 1
    }
    /* Forget light array */
    lite_n = 0 as libc::c_int as s16b;
}
/*
 * Update squares illuminated by monsters
 *
 * Code taken from Steven Fuerst's work for ZAngband, without support
 * for multiple lite radii, and with necessary modifications for different
 * internal representation of dungeon/wilderness. Other minor changes
 * are mine...
 *
 * I'm not sure if I can handle wide radius well. Consider the following
 * example, with p carrying a radius 3 light source:
 *
 *     ##%#
 *     .x..
 *     p##@
 *
 * % should be illuminated, although the beam path is entirely out of
 * player's los (because of grid-based nature of cave representation)...
 * And I'm extremely reluctant to introduce symmetrical los. The current
 * asymmetrical system has its own merit, and all the rules of games are
 * asymmetrical, in some way or another...
 *
 * The code below exploits special characteristics of radius one light
 * where one can fairly safely use light source's visibility (in terms of los)
 * to determine if we can illuminate walls XXX
 *
 * This function works within the current player's field of view
 * calculated by update_view(), so it should normally be called
 * whenever FoV is updated (== PU_VIEW | PU_MON_LITE). The other
 * case is when RF9_HAS_LITE monsters have moved or dead. Monster
 * creation occurs out of LoS, so I chose not to take this into
 * consideration.
 *
 * The CAVE_TEMP flag is used by the function to remember "old" monster-lit
 * grids so that it can only redraw squares whose visibility has changed.
 *
 * Doing this in the update_view() order (update "new" grids, then "old")
 * would result in bizarre lighting effects XXX XXX
 *
 * It has been made possible again to draw torch/monster-lit grids in
 * different colours, even when they are in permanently lit locations
 * by using (CAVE_PLIT|CAVE_MLIT) as if it were old CAVE_LITE, but I don't
 * think it's appropriate for torch lights to be visible under the Sun :)
 * or brighter light, and it doesn't work well with PernAngband's already
 * colourful terrain features in aesthetically pleasing ways... -- pelpel
 */
#[no_mangle]
pub unsafe extern "C" fn update_mon_lite() {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut fy: libc::c_int = 0;
    let mut fx: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut info: u16b = 0;
    let mut invis: bool_ = 0;
    let mut fast_lite_n: s16b = lite_n;
    let mut fast_temp_n: s16b = 0;
    /* Mega-Hack -- It's unnecessary there */
    if (*p_ptr).wild_mode != 0 { return }
    /* Handle special case -- Blindness */
    if (*p_ptr).blind != 0 {
        i = 0 as libc::c_int;
        while i < fast_lite_n as libc::c_int {
            /* Light location */
            y = lite_y[i as usize] as libc::c_int;
            x = lite_x[i as usize] as libc::c_int;
            /* Redraw spot */
			/* lite_spot(y, x); */
            let ref mut fresh21 = (*cave[y as usize].offset(x as isize)).info;
            *fresh21 =
                (*fresh21 as libc::c_int &
                     !(0x4000 as libc::c_int | 0x10 as libc::c_int)) as u16b;
            i += 1
        }
        /* Forget monster light and view */
        /* Clear the light list */
        lite_n = 0 as libc::c_int as s16b;
        /* Done */
        return
    }
    /* Remember and clear all monster-lit grids */
    i = 0 as libc::c_int;
    while i < fast_lite_n as libc::c_int {
        /* Lit location */
        y = lite_y[i as usize] as libc::c_int;
        x = lite_x[i as usize] as libc::c_int;
        /* Access grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* Access cave info of the grid */
        info = (*c_ptr).info;
        /* Remember it, by setting the CAVE_TEMP flag */
        info = (info as libc::c_int | 0x40 as libc::c_int) as u16b;
        /* Forget monster light */
        info = (info as libc::c_int & !(0x4000 as libc::c_int)) as u16b;
        /* Unseen unless it's glowing or illuminated by player light source */
        if info as libc::c_int & (0x2 as libc::c_int | 0x2000 as libc::c_int)
               == 0 {
            info = (info as libc::c_int & !(0x10 as libc::c_int)) as u16b
        }
        /* Save cave info flags */
        (*c_ptr).info = info;
        i += 1
    }
    /* Clear the temp list */
    fast_temp_n = 0 as libc::c_int as s16b;
    /* Loop through monsters, adding newly lit grids to changes list */
    i = 1 as libc::c_int;
    while i < m_max as libc::c_int {
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
        /* Skip dead monsters */
        if !((*m_ptr).r_idx == 0) {
            /* Skip out-of-sight monsters (MAX_SIGHT + max radius) */
            if !((*m_ptr).cdis as libc::c_int >
                     20 as libc::c_int + 1 as libc::c_int) {
                /* Access monster race info (with possible ego mods) */
                r_ptr =
                    race_info_idx((*m_ptr).r_idx as libc::c_int,
                                  (*m_ptr).ego as libc::c_int);
                /* Skip monsters not carrying light source */
                if !((*r_ptr).flags9 & 0x4 as libc::c_int as libc::c_uint ==
                         0) {
                    /* Access the location */
                    fy = (*m_ptr).fy as libc::c_int;
                    fx = (*m_ptr).fx as libc::c_int;
                    /* Extract monster grid visibility */
                    invis =
                        !((*cave[fy as usize].offset(fx as isize)).info as
                              libc::c_int & 0x20 as libc::c_int !=
                              0 as libc::c_int) as libc::c_int as bool_;
                    /* Nested loops may be a bad idea here XXX */
                    d = 0 as libc::c_int;
                    while d < 9 as libc::c_int {
                        y = fy + ddy_ddd[d as usize] as libc::c_int;
                        x = fx + ddx_ddd[d as usize] as libc::c_int;
                        /* Paranoia */
			/* if (!in_bounds(y, x)) continue; */
                        /* Access the grid */
                        c_ptr =
                            &mut *(*cave.as_mut_ptr().offset(y as
                                                                 isize)).offset(x
                                                                                    as
                                                                                    isize)
                                as *mut cave_type;
                        /* Access cave info flags */
                        info = (*c_ptr).info;
                        /* Don't care grids out of player's los */
                        if !(info as libc::c_int & 0x20 as libc::c_int == 0) {
                            /*
			 * Avoid processing already monster-lit grids,
			 * for efficiency and to avoid temp array overflow
			 */
                            if !(info as libc::c_int & 0x4000 as libc::c_int
                                     != 0) {
                                /*
			 * Hack XXX XXX -- light shouldn't penetrate walls
			 *
			 *     OK          NG
			 *  .#.  p#.  | p.   .p.  p..
			 *  p.@  ..@  | .#   .#.  .#.
			 *            | .@   .@.  ..@
			 *
			 * So if a monster carrying light source is out of player LoS,
			 * walls aren't illuminated.
			 *
			 * CAVEAT: % will be illuminated in cases like this:
			 *
			 *  #%..@
			 *  p....
			 *
			 * We don't have four sides for a wall grid, so...
			 */
                                if !(invis as libc::c_int != 0 &&
                                         (*f_info.offset((*c_ptr).feat as
                                                             isize)).flags1 as
                                             libc::c_long &
                                             0x2 as libc::c_long != 0) {
                                    /* Give monster light to the location */
                                    (*c_ptr).info =
                                        ((*c_ptr).info as libc::c_int |
                                             (0x4000 as libc::c_int |
                                                  0x10 as libc::c_int)) as
                                            u16b;
                                    /* Save the location */
                                    temp_y[fast_temp_n as usize] =
                                        y as byte_hack;
                                    temp_x[fast_temp_n as usize] =
                                        x as byte_hack;
                                    fast_temp_n += 1
                                }
                            }
                        }
                        d += 1
                    }
                }
            }
        }
        i += 1
    }
    /* Process old grids */
    i = 0 as libc::c_int;
    while i < fast_lite_n as libc::c_int {
        /* Access location */
        y = lite_y[i as usize] as libc::c_int;
        x = lite_x[i as usize] as libc::c_int;
        /* Access grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* Was lit, is no longer lit */
        if (*c_ptr).info as libc::c_int & 0x4000 as libc::c_int == 0 {
            /* Clear the temp flag */
            (*c_ptr).info =
                ((*c_ptr).info as libc::c_int & !(0x40 as libc::c_int)) as
                    u16b;
            /* See if there was a visible monster */
            if (*cave[y as usize].offset(x as isize)).info as libc::c_int &
                   0x20 as libc::c_int != 0 as libc::c_int &&
                   (*c_ptr).m_idx as libc::c_int != 0 {
                /* Hide the monster */
                update_mon((*c_ptr).m_idx as libc::c_int,
                           0 as libc::c_int as bool_);
            } else {
                /* Redraw */
                lite_spot(y, x);
            }
        }
        i += 1
    }
    /* Copy the temp array into the light array */
    i = 0 as libc::c_int;
    while i < fast_temp_n as libc::c_int {
        /* Access location */
        y = temp_y[i as usize] as libc::c_int;
        x = temp_x[i as usize] as libc::c_int;
        /* Access grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize)
                as *mut cave_type;
        /* No changes in illumination */
        if (*c_ptr).info as libc::c_int & 0x40 as libc::c_int != 0 {
            /* Clear the temp flag */
            (*c_ptr).info =
                ((*c_ptr).info as libc::c_int & !(0x40 as libc::c_int)) as
                    u16b
        } else {
            /* Was not lit, is now lit */
            /* Remember the location, if appropriate */
            note_spot(y, x);
            if (*c_ptr).m_idx != 0 {
                /* Show it */
                update_mon((*c_ptr).m_idx as libc::c_int,
                           0 as libc::c_int as bool_);
            } else {
                /* Redraw */
                lite_spot(y, x);
            }
        }
        /* See if there is a monster */
        /* Save the location */
        lite_y[i as usize] = y as s16b;
        lite_x[i as usize] = x as s16b;
        i += 1
    }
    /* Save lite_n */
    lite_n = fast_temp_n;
    /* Forget temp */
    temp_n = 0 as libc::c_int as s16b;
}
/*
 * Hack -- provide some "speed" for the "flow" code
 * This entry is the "current index" for the "when" field
 * Note that a "when" value of "zero" means "not used".
 *
 * Note that the "cost" indexes from 1 to 127 are for
 * "old" data, and from 128 to 255 are for "new" data.
 *
 * This means that as long as the player does not "teleport",
 * then any monster up to 128 + MONSTER_FLOW_DEPTH will be
 * able to track down the player, and in general, will be
 * able to track down either the player or a position recently
 * occupied by the player.
 */
static mut flow_n: libc::c_int = 0 as libc::c_int;
/*
 * Hack -- forget the "flow" information
 */
#[no_mangle]
pub unsafe extern "C" fn forget_flow() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    /* Nothing to forget */
    if flow_n == 0 { return }
    /* Check the entire dungeon */
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            /* Forget the old data */
            (*cave[y as usize].offset(x as isize)).cost =
                0 as libc::c_int as byte_hack;
            (*cave[y as usize].offset(x as isize)).when =
                0 as libc::c_int as byte_hack;
            x += 1
        }
        y += 1
    }
    /* Start over */
    flow_n = 0 as libc::c_int;
}
/*
 * Hack -- Allow us to treat the "seen" array as a queue
 */
static mut flow_head: libc::c_int = 0 as libc::c_int;
static mut flow_tail: libc::c_int = 0 as libc::c_int;
/*
 * Take note of a reachable grid.  Assume grid is legal.
 */
unsafe extern "C" fn update_flow_aux(mut y: libc::c_int, mut x: libc::c_int,
                                     mut n: libc::c_int) {
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut old_head: libc::c_int = flow_head;
    /* Get the grid */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Ignore "pre-stamped" entries */
    if (*c_ptr).when as libc::c_int == flow_n { return }
    /* Ignore "walls" and "rubble" */
    if (*c_ptr).feat as libc::c_int >= 0x31 as libc::c_int { return }
    /* Save the time-stamp */
    (*c_ptr).when = flow_n as byte_hack;
    /* Save the flow cost */
    (*c_ptr).cost = n as byte_hack;
    /* Hack -- limit flow depth */
    if n == 32 as libc::c_int { return }
    /* Enqueue that entry */
    temp_y[flow_head as usize] = y as byte_hack;
    temp_x[flow_head as usize] = x as byte_hack;
    /* Advance the queue */
    flow_head += 1;
    if flow_head == 16384 as libc::c_int { flow_head = 0 as libc::c_int }
    /* Hack -- notice overflow by forgetting new entry */
    if flow_head == flow_tail { flow_head = old_head };
}
/*
 * Hack -- fill in the "cost" field of every grid that the player
 * can "reach" with the number of steps needed to reach that grid.
 * This also yields the "distance" of the player from every grid.
 *
 * In addition, mark the "when" of the grids that can reach
 * the player with the incremented value of "flow_n".
 *
 * Hack -- use the "seen" array as a "circular queue".
 *
 * We do not need a priority queue because the cost from grid
 * to grid is always "one" and we process them in order.
 */
#[no_mangle]
pub unsafe extern "C" fn update_flow() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    /* Hack -- disabled */
    if flow_by_sound == 0 { return }
    /* Paranoia -- make sure the array is empty */
    if temp_n != 0 { return }
    /* Cycle the old entries (once per 128 updates) */
    if flow_n == 255 as libc::c_int {
        /* Rotate the time-stamps */
        y = 0 as libc::c_int;
        while y < cur_hgt as libc::c_int {
            x = 0 as libc::c_int;
            while x < cur_wid as libc::c_int {
                let mut w: libc::c_int =
                    (*cave[y as usize].offset(x as isize)).when as
                        libc::c_int;
                (*cave[y as usize].offset(x as isize)).when =
                    if w > 128 as libc::c_int {
                        (w) - 128 as libc::c_int
                    } else { 0 as libc::c_int } as byte_hack;
                x += 1
            }
            y += 1
        }
        /* Restart */
        flow_n = 127 as libc::c_int
    }
    /* Start a new flow (never use "zero") */
    flow_n += 1;
    /* Reset the "queue" */
    flow_tail = 0 as libc::c_int;
    flow_head = flow_tail;
    /* Add the player's grid to the queue */
    update_flow_aux((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                    0 as libc::c_int);
    /* Now process the queue */
    while flow_head != flow_tail {
        /* Extract the next entry */
        y = temp_y[flow_tail as usize] as libc::c_int;
        x = temp_x[flow_tail as usize] as libc::c_int;
        /* Forget that entry */
        flow_tail += 1;
        if flow_tail == 16384 as libc::c_int { flow_tail = 0 as libc::c_int }
        /* Add the "children" */
        d = 0 as libc::c_int;
        while d < 8 as libc::c_int {
            /* Add that child if "legal" */
            update_flow_aux(y + ddy_ddd[d as usize] as libc::c_int,
                            x + ddx_ddd[d as usize] as libc::c_int,
                            (*cave[y as usize].offset(x as isize)).cost as
                                libc::c_int + 1 as libc::c_int);
            d += 1
        }
    }
    /* Forget the flow info */
    flow_tail = 0 as libc::c_int;
    flow_head = flow_tail;
}
/*
 * Hack -- map the current panel (plus some) ala "magic mapping"
 */
#[no_mangle]
pub unsafe extern "C" fn map_area() {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Pick an area to map */
    y1 =
        panel_row_min as libc::c_int -
            (Rand_div(10 as libc::c_int) + 1 as libc::c_int);
    y2 =
        panel_row_max as libc::c_int +
            (Rand_div(10 as libc::c_int) + 1 as libc::c_int);
    x1 =
        panel_col_min as libc::c_int -
            (Rand_div(20 as libc::c_int) + 1 as libc::c_int);
    x2 =
        panel_col_max as libc::c_int +
            (Rand_div(20 as libc::c_int) + 1 as libc::c_int);
    /* Speed -- shrink to fit legal bounds */
    if y1 < 1 as libc::c_int { y1 = 1 as libc::c_int }
    if y2 > cur_hgt as libc::c_int - 2 as libc::c_int {
        y2 = cur_hgt as libc::c_int - 2 as libc::c_int
    }
    if x1 < 1 as libc::c_int { x1 = 1 as libc::c_int }
    if x2 > cur_wid as libc::c_int - 2 as libc::c_int {
        x2 = cur_wid as libc::c_int - 2 as libc::c_int
    }
    /* Scan that area */
    y = y1;
    while y <= y2 {
        x = x1;
        while x <= x2 {
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type;
            /* All non-walls are "checked" */
            if is_wall(c_ptr) == 0 {
                /* Memorize normal features */
                if !((*f_info.offset((*c_ptr).feat as isize)).flags1 as
                         libc::c_long & 0x10 as libc::c_long != 0 &&
                         (*f_info.offset((*c_ptr).feat as isize)).flags1 as
                             libc::c_long & 0x100 as libc::c_long == 0) {
                    /* Memorize the object */
                    (*c_ptr).info =
                        ((*c_ptr).info as libc::c_int | 0x1 as libc::c_int) as
                            u16b
                }
                /* Memorize known walls */
                i = 0 as libc::c_int;
                while i < 8 as libc::c_int {
                    c_ptr =
                        &mut *(*cave.as_mut_ptr().offset((y +
                                                              *ddy_ddd.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)
                                                                  as
                                                                  libc::c_int)
                                                             as
                                                             isize)).offset((x
                                                                                 +
                                                                                 *ddx_ddd.as_mut_ptr().offset(i
                                                                                                                  as
                                                                                                                  isize)
                                                                                     as
                                                                                     libc::c_int)
                                                                                as
                                                                                isize)
                            as *mut cave_type;
                    /* Memorize walls (etc) */
                    if is_wall(c_ptr) != 0 {
                        /* Memorize the walls */
                        (*c_ptr).info =
                            ((*c_ptr).info as libc::c_int |
                                 0x1 as libc::c_int) as u16b
                    }
                    i += 1
                }
            }
            x += 1
        }
        y += 1
    }
    /* Redraw map */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
}
/*
 * Light up the dungeon using "clairvoyance"
 *
 * This function "illuminates" every grid in the dungeon, memorizes all
 * "objects", memorizes all grids as with magic mapping, and, under the
 * standard option settings (view_perma_grids but not view_torch_grids)
 * memorizes all floor grids too.
 *
 * Note that if "view_perma_grids" is not set, we do not memorize floor
 * grids, since this would defeat the purpose of "view_perma_grids", not
 * that anyone seems to play without this option.
 *
 * Note that if "view_torch_grids" is set, we do not memorize floor grids,
 * since this would prevent the use of "view_torch_grids" as a method to
 * keep track of what grids have been observed directly.
 */
#[no_mangle]
pub unsafe extern "C" fn wiz_lite() {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    /* Memorize objects */
    i = 1 as libc::c_int;
    while i < o_max as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *o_list.offset(i as isize) as *mut object_type;
        /* Skip dead objects */
        if !((*o_ptr).k_idx == 0) {
            /* Skip held objects */
            if !((*o_ptr).held_m_idx != 0) {
                /* Memorize */
                (*o_ptr).marked = 1 as libc::c_int as byte_hack
            }
        }
        i += 1
    }
    /* Scan all normal grids */
    y = 1 as libc::c_int;
    while y < cur_hgt as libc::c_int - 1 as libc::c_int {
        /* Scan all normal grids */
        x = 1 as libc::c_int;
        while x < cur_wid as libc::c_int - 1 as libc::c_int {
            let mut c_ptr: *mut cave_type =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type;
            if (*c_ptr).m_idx != 0 {
                let mut m_ptr: *mut monster_type =
                    &mut *m_list.offset((*c_ptr).m_idx as isize) as
                        *mut monster_type;
                let mut r_ptr: *mut monster_race =
                    if !(*m_ptr).sr_ptr.is_null() {
                        (*m_ptr).sr_ptr
                    } else {
                        race_info_idx((*m_ptr).r_idx as libc::c_int,
                                      (*m_ptr).ego as libc::c_int)
                    };
                if (*r_ptr).flags9 & 0x8 as libc::c_int as libc::c_uint != 0 {
                    let mut o_ptr_0: *mut object_type =
                        &mut *o_list.offset((*m_ptr).hold_o_idx as isize) as
                            *mut object_type;
                    (*o_ptr_0).marked = 1 as libc::c_int as byte_hack
                }
            }
            /* Process all non-walls */
			/* if (c_ptr->feat < FEAT_SECRET) */
            /* Scan all neighbors */
            i = 0 as libc::c_int;
            while i < 9 as libc::c_int {
                let mut yy: libc::c_int =
                    y + ddy_ddd[i as usize] as libc::c_int;
                let mut xx: libc::c_int =
                    x + ddx_ddd[i as usize] as libc::c_int;
                /* Get the grid */
                c_ptr =
                    &mut *(*cave.as_mut_ptr().offset(yy as
                                                         isize)).offset(xx as
                                                                            isize)
                        as *mut cave_type;
                /* Perma-lite the grid */
                (*c_ptr).info =
                    ((*c_ptr).info as libc::c_int | 0x2 as libc::c_int) as
                        u16b;
                /* Memorize normal features */
                if !((*f_info.offset((*c_ptr).feat as isize)).flags1 as
                         libc::c_long & 0x10 as libc::c_long != 0 &&
                         (*f_info.offset((*c_ptr).feat as isize)).flags1 as
                             libc::c_long & 0x100 as libc::c_long == 0) {
                    /* Memorize the grid */
                    (*c_ptr).info =
                        ((*c_ptr).info as libc::c_int | 0x1 as libc::c_int) as
                            u16b
                }
                /* Normally, memorize floors (see above) */
                if view_perma_grids as libc::c_int != 0 &&
                       view_torch_grids == 0 {
                    /* Memorize the grid */
                    (*c_ptr).info =
                        ((*c_ptr).info as libc::c_int | 0x1 as libc::c_int) as
                            u16b
                }
                i += 1
            }
            x += 1
        }
        y += 1
    }
    /* Fully update the visuals */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x10000 as libc::c_long | 0x100000 as libc::c_long |
                  0x1000000 as libc::c_long | 0x200000 as libc::c_long)) as
            u32b;
    /* Redraw map */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
}
#[no_mangle]
pub unsafe extern "C" fn wiz_lite_extra() {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            let ref mut fresh22 = (*cave[y as usize].offset(x as isize)).info;
            *fresh22 =
                (*fresh22 as libc::c_int |
                     (0x2 as libc::c_int | 0x1 as libc::c_int)) as u16b;
            x += 1
        }
        y += 1
    }
    wiz_lite();
}
/*
 * Forget the dungeon map (ala "Thinking of Maud...").
 */
#[no_mangle]
pub unsafe extern "C" fn wiz_dark() {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    /* Forget every grid */
    y = 0 as libc::c_int;
    while y < cur_hgt as libc::c_int {
        x = 0 as libc::c_int;
        while x < cur_wid as libc::c_int {
            let mut c_ptr: *mut cave_type =
                &mut *(*cave.as_mut_ptr().offset(y as
                                                     isize)).offset(x as
                                                                        isize)
                    as *mut cave_type;
            /* Process the grid */
            (*c_ptr).info =
                ((*c_ptr).info as libc::c_int & !(0x1 as libc::c_int)) as
                    u16b;
            x += 1
        }
        y += 1
    }
    /* Forget all objects */
    i = 1 as libc::c_int;
    while i < o_max as libc::c_int {
        let mut o_ptr: *mut object_type =
            &mut *o_list.offset(i as isize) as *mut object_type;
        /* Skip dead objects */
        if !((*o_ptr).k_idx == 0) {
            /* Skip held objects */
            if !((*o_ptr).held_m_idx != 0) {
                /* Forget the object */
                (*o_ptr).marked = 0 as libc::c_int as byte_hack
            }
        }
        i += 1
    }
    /* Fully update the visuals */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long |
             (0x10000 as libc::c_long | 0x100000 as libc::c_long |
                  0x1000000 as libc::c_long | 0x200000 as libc::c_long)) as
            u32b;
    /* Redraw map */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x4000000 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as u32b;
}
/*
 * Change the "feat" flag for a grid, and notice/redraw the grid
 */
#[no_mangle]
pub unsafe extern "C" fn cave_set_feat(mut y: libc::c_int, mut x: libc::c_int,
                                       mut feat: libc::c_int) {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Change the feature */
    (*c_ptr).feat = feat as byte_hack;
    /*
	 * Handle "wall/door" grids
	 *
	 * XXX XXX XXX This assumes c_ptr->mimic doesn't mimic terrain
	 * features whose LoS behaviour is different from its own, in
	 * most cases. Level boundaries are the most notable exception,
	 * where "real" terrain is always FEAT_PERM_SOLID, and the fact
	 * is (ab)used to prevent out-of-range access to the cave array.
	 * If we were going to implement an evil dungeon type in which
	 * everything is mimicked, then this function, los(), projectable(),
	 * project_path() and maybe some functions in melee2.c might
	 * better use c_ptr->mimic when it's set -- pelpel
	 */
    if (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long &
           0x2 as libc::c_long != 0 {
        (*c_ptr).info =
            ((*c_ptr).info as libc::c_int | 0x80 as libc::c_int) as u16b
    } else {
        /* Handle "floor"/etc grids */
        (*c_ptr).info =
            ((*c_ptr).info as libc::c_int & !(0x80 as libc::c_int)) as u16b
    }
    /* Notice & Redraw */
    if character_dungeon != 0 {
        /* Notice */
        note_spot(y, x);
        /* Redraw */
        lite_spot(y, x);
    };
}
/*
 * Place floor terrain at (y, x) according to dungeon info
 */
#[no_mangle]
pub unsafe extern "C" fn place_floor(mut y: libc::c_int, mut x: libc::c_int) {
    cave_set_feat(y, x,
                  floor_type[Rand_div(100 as libc::c_int) as usize] as
                      libc::c_int);
}
/*
 * This routine is used when the current feature gets convert to a floor and
 * the possible floor types include glass which is permanent. An unpassable
 * feature is undesirable, so the glass gets convert to molten glass which
 * is passable.
 */
#[no_mangle]
pub unsafe extern "C" fn place_floor_convert_glass(mut y: libc::c_int,
                                                   mut x: libc::c_int) {
    place_floor(y, x);
    if (*cave[y as usize].offset(x as isize)).feat as libc::c_int ==
           188 as libc::c_int {
        (*cave[y as usize].offset(x as isize)).feat =
            103 as libc::c_int as byte_hack
    };
}
/*
 * Place a cave filler at (y, x)
 */
#[no_mangle]
pub unsafe extern "C" fn place_filler(mut y: libc::c_int,
                                      mut x: libc::c_int) {
    cave_set_feat(y, x,
                  fill_type[Rand_div(100 as libc::c_int) as usize] as
                      libc::c_int);
}
/*
 * Calculate "incremental motion". Used by project() and shoot().
 * Assumes that (*y,*x) lies on the path from (y1,x1) to (y2,x2).
 */
#[no_mangle]
pub unsafe extern "C" fn mmove2(mut y: *mut libc::c_int,
                                mut x: *mut libc::c_int, mut y1: libc::c_int,
                                mut x1: libc::c_int, mut y2: libc::c_int,
                                mut x2: libc::c_int) {
    let mut dy: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut dist: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    /* Extract the distance travelled */
    dy = if *y < y1 { (y1) - *y } else { (*y) - y1 };
    dx = if *x < x1 { (x1) - *x } else { (*x) - x1 };
    /* Number of steps */
    dist = if dy > dx { dy } else { dx };
    /* We are calculating the next location */
    dist += 1;
    /* Calculate the total distance along each axis */
    dy = if y2 < y1 { (y1) - y2 } else { (y2) - y1 };
    dx = if x2 < x1 { (x1) - x2 } else { (x2) - x1 };
    /* Paranoia -- Hack -- no motion */
    if dy == 0 && dx == 0 { return }
    /* Move mostly vertically */
    if dy > dx {
        /* Extract a shift factor */
        shift = (dist * dx + (dy - 1 as libc::c_int) / 2 as libc::c_int) / dy;
        /* Sometimes move along the minor axis */
        *x = if x2 < x1 { (x1) - shift } else { (x1) + shift };
        /* Always move along major axis */
        *y = if y2 < y1 { (y1) - dist } else { (y1) + dist }
    } else {
        /* Move mostly horizontally */
        /* Extract a shift factor */
        shift = (dist * dy + (dx - 1 as libc::c_int) / 2 as libc::c_int) / dx;
        *y = if y2 < y1 { (y1) - shift } else { (y1) + shift };
        *x = if x2 < x1 { (x1) - dist } else { (x1) + dist }
    };
}
/* Sometimes move along the minor axis */
/* Always move along major axis */
/*
 * Determine if a bolt spell cast from (y1,x1) to (y2,x2) will arrive
 * at the final destination, assuming no monster gets in the way.
 *
 * This is slightly (but significantly) different from "los(y1,x1,y2,x2)".
 */
#[no_mangle]
pub unsafe extern "C" fn projectable(mut y1: libc::c_int, mut x1: libc::c_int,
                                     mut y2: libc::c_int, mut x2: libc::c_int)
 -> bool_ {
    let mut dist: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    /* Start at the initial location */
    y = y1;
    x = x1;
    /* See "project()" */
    dist = 0 as libc::c_int;
    while dist <= 18 as libc::c_int {
        /* Check for arrival at "final target" */
		/*
		 * NB: this check was AFTER the 'never pass
		 * thru walls' clause, below. Switching them
		 * lets monsters shoot a the player if s/he is
		 * visible but in a wall
		 */
        if x == x2 && y == y2 { return 1 as libc::c_int as bool_ }
        /* Never pass through walls */
        if dist != 0 &&
               ((*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                    isize)).flags1 as libc::c_long &
                    0x2 as libc::c_long != 0 ||
                    !((*f_info.offset((*cave[y as
                                                 usize].offset(x as
                                                                   isize)).feat
                                          as isize)).flags1 as libc::c_long &
                          0x10 as libc::c_long != 0 &&
                          (*cave[y as usize].offset(x as isize)).feat as
                              libc::c_int != 0xaf as libc::c_int)) {
            break ;
        }
        /* Calculate the new location */
        mmove2(&mut y, &mut x, y1, x1, y2, x2);
        dist += 1
    }
    /* Assume obstruction */
    return 0 as libc::c_int as bool_;
}
/*
 * Standard "find me a location" function
 *
 * Obtains a legal location within the given distance of the initial
 * location, and with "los()" from the source to destination location.
 *
 * This function is often called from inside a loop which searches for
 * locations while increasing the "d" distance.
 *
 * Currently the "m" parameter is unused.
 */
#[no_mangle]
pub unsafe extern "C" fn scatter(mut yp: *mut libc::c_int,
                                 mut xp: *mut libc::c_int, mut y: libc::c_int,
                                 mut x: libc::c_int, mut d: libc::c_int) {
    let mut nx: libc::c_int = 0;
    let mut ny: libc::c_int = 0;
    let mut attempts_left: libc::c_int = 5000 as libc::c_int;
    loop 
         /* Pick a location */
         {
        attempts_left -= 1;
        if !(attempts_left != 0) { break ; }
        /* Pick a new location */
        ny = y + Rand_div(1 as libc::c_int + d + d) - d;
        nx = x + Rand_div(1 as libc::c_int + d + d) - d;
        /* Ignore illegal locations and outer walls */
        if !(ny > 0 as libc::c_int && nx > 0 as libc::c_int &&
                 ny < cur_hgt as libc::c_int - 1 as libc::c_int &&
                 nx < cur_wid as libc::c_int - 1 as libc::c_int) {
            continue ;
        }
        /* Ignore "excessively distant" locations */
        if d > 1 as libc::c_int && distance(y, x, ny, nx) > d { continue ; }
        /* Require "line of sight" */
        if los(y, x, ny, nx) != 0 { break ; }
    }
    if attempts_left > 0 as libc::c_int {
        /* Save the location */
        *yp = ny;
        *xp = nx
    };
}
/*
 * Track a new monster
 */
#[no_mangle]
pub unsafe extern "C" fn health_track(mut m_idx: libc::c_int) {
    /* Track a new guy */
    health_who = m_idx as s16b;
    /* Redraw (later) */
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x800 as libc::c_long) as u32b;
}
/*
 * Hack -- track the given monster race
 */
#[no_mangle]
pub unsafe extern "C" fn monster_race_track(mut r_idx: libc::c_int,
                                            mut ego: libc::c_int) {
    /* Save this monster ID */
    monster_race_idx = r_idx as s16b;
    monster_ego_idx = ego as s16b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x100 as libc::c_long) as u32b;
}
/*
 * Hack -- track the given object kind
 */
#[no_mangle]
pub unsafe extern "C" fn object_track(mut o_ptr: *mut object_type) {
    /* Save this monster ID */
    tracked_object = o_ptr;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x200 as libc::c_long) as u32b;
}
/*
 * Something has happened to disturb the player.
 *
 * The first arg indicates a major disturbance, which affects search.
 *
 * The second arg is currently unused, but could induce output flush.
 *
 * All disturbance cancels repeated commands, resting, and running.
 */
#[no_mangle]
pub unsafe extern "C" fn disturb(mut stop_search: libc::c_int,
                                 mut unused_flag: libc::c_int) {
    /* Unused */
    unused_flag = unused_flag;
    /* Cancel auto-commands */
	/* command_new = 0; */
    /* Cancel repeated commands */
    if command_rep != 0 {
        /* Cancel */
        command_rep = 0 as libc::c_int as s16b;
        /* Redraw the state (later) */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x100000 as libc::c_long) as
                u32b
    }
    /* Cancel Resting */
    if resting != 0 {
        /* Cancel */
        resting = 0 as libc::c_int as s16b;
        /* Redraw the state (later) */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x100000 as libc::c_long) as
                u32b
    }
    /* Cancel running */
    if running != 0 {
        /* Cancel */
        running = 0 as libc::c_int as s16b;
        /* Calculate torch radius */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x2 as libc::c_long) as u32b
    }
    /* Cancel searching if requested */
    if stop_search != 0 && (*p_ptr).searching as libc::c_int != 0 {
        /* Cancel */
        (*p_ptr).searching = 0 as libc::c_int as byte_hack;
        /* Recalculate bonuses */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
        /* Redraw the state */
        (*p_ptr).redraw =
            ((*p_ptr).redraw as libc::c_long | 0x100000 as libc::c_long) as
                u32b
    }
    /* Flush the input if requested */
    if flush_disturb != 0 { flush(); };
}
/*
 * Hack -- Check if a level is a "quest" level
 */
#[no_mangle]
pub unsafe extern "C" fn is_quest(mut level: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = random_quest_number();
    /* Check quests */
    if (*p_ptr).inside_quest != 0 {
        return (*p_ptr).inside_quest as libc::c_int
    }
    if i != 0 { return 5 as libc::c_int }
    /* Nope */
    return 0 as libc::c_int;
}
/*
 * Return the index of the random quest on this level
 * (or zero)
 */
#[no_mangle]
pub unsafe extern "C" fn random_quest_number() -> libc::c_int {
    if dun_level as libc::c_int >= 1 as libc::c_int &&
           (dun_level as libc::c_int) < 99 as libc::c_int &&
           dungeon_flags1 as libc::c_long & 0x1 as libc::c_long != 0 &&
           random_quests[dun_level as usize].type_0 as libc::c_int != 0 &&
           random_quests[dun_level as usize].done == 0 &&
           is_randhero(dun_level as libc::c_int) == 0 {
        return dun_level as libc::c_int
    }
    /* Nope */
    return 0 as libc::c_int;
}
/*
 * handle spell effects
 */
#[no_mangle]
pub unsafe extern "C" fn effect_pop() -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < 128 as libc::c_int {
        if effects[i as usize].time == 0 { return i }
        i += 1
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn new_effect(mut type_0: libc::c_int,
                                    mut dam: libc::c_int,
                                    mut time: libc::c_int,
                                    mut cy: libc::c_int, mut cx: libc::c_int,
                                    mut rad: libc::c_int, mut flags: s32b)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = effect_pop();
    if i == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    effects[i as usize].type_0 = type_0 as s16b;
    effects[i as usize].dam = dam as s16b;
    effects[i as usize].time = time as s16b;
    effects[i as usize].flags = flags as u32b;
    effects[i as usize].cx = cx as s16b;
    effects[i as usize].cy = cy as s16b;
    effects[i as usize].rad = rad as s16b;
    return i;
}
