use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stat_names: [cptr; 6];
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut command_arg: s16b;
    #[no_mangle]
    static mut cur_hgt: s16b;
    #[no_mangle]
    static mut cur_wid: s16b;
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut inkey_scan: bool_;
    #[no_mangle]
    static mut m_max: s16b;
    #[no_mangle]
    static mut leaving_quest: libc::c_int;
    #[no_mangle]
    static mut target_who: s16b;
    #[no_mangle]
    static mut target_col: s16b;
    #[no_mangle]
    static mut target_row: s16b;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn maxroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn Term_fresh() -> errr;
    #[no_mangle]
    fn Term_putch(x: libc::c_int, y: libc::c_int, a: byte_hack,
                  c: libc::c_char) -> errr;
    #[no_mangle]
    fn Term_clear() -> errr;
    #[no_mangle]
    fn Term_save() -> errr;
    #[no_mangle]
    fn Term_load() -> errr;
    #[no_mangle]
    fn atol(__nptr: *const libc::c_char) -> libc::c_long;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut m_list: *mut monster_type;
    #[no_mangle]
    static mut max_real_towns: u16b;
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut player_hp: [s16b; 50];
    #[no_mangle]
    static mut f_info: *mut feature_type;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut k_name: *mut libc::c_char;
    #[no_mangle]
    static mut a_info: *mut artifact_type;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut d_info: *mut dungeon_info_type;
    #[no_mangle]
    static mut d_text: *mut libc::c_char;
    #[no_mangle]
    static mut wf_info: *mut wilderness_type_info;
    #[no_mangle]
    static mut max_wild_x: u16b;
    #[no_mangle]
    static mut max_wild_y: u16b;
    #[no_mangle]
    static mut wild_map: *mut *mut wilderness_map;
    #[no_mangle]
    static mut max_r_idx: u16b;
    #[no_mangle]
    static mut max_k_idx: u16b;
    #[no_mangle]
    static mut max_a_idx: u16b;
    #[no_mangle]
    static mut max_d_idx: u16b;
    #[no_mangle]
    static mut random_artifacts: [random_artifact; 84];
    #[no_mangle]
    static mut fates: [fate; 200];
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut max_dlv: *mut s16b;
    #[no_mangle]
    static mut m_allow_special: *mut bool_;
    #[no_mangle]
    static mut quest: *mut quest_type;
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn map_area();
    #[no_mangle]
    fn wiz_lite();
    #[no_mangle]
    fn cave_set_feat(y: libc::c_int, x: libc::c_int, feat: libc::c_int);
    #[no_mangle]
    fn scatter(yp: *mut libc::c_int, xp: *mut libc::c_int, y: libc::c_int,
               x: libc::c_int, d: libc::c_int);
    #[no_mangle]
    fn do_cmd_redraw();
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn handle_stuff();
    #[no_mangle]
    fn do_cmd_help();
    #[no_mangle]
    fn autosave_checkpoint();
    #[no_mangle]
    fn monster_check_experience(m_idx: libc::c_int, silent: bool_);
    #[no_mangle]
    fn delete_monster_idx(i: libc::c_int);
    #[no_mangle]
    fn place_monster_aux(y: libc::c_int, x: libc::c_int, r_idx: libc::c_int,
                         slp: bool_, grp: bool_, status: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn alloc_horde(y: libc::c_int, x: libc::c_int) -> bool_;
    #[no_mangle]
    fn summon_specific(y1: libc::c_int, x1: libc::c_int, lev: libc::c_int,
                       type_0: libc::c_int) -> bool_;
    #[no_mangle]
    fn get_object(item: libc::c_int) -> *mut object_type;
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn object_desc_store(buf: *mut libc::c_char, o_ptr: *mut object_type,
                         pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn get_item(cp: *mut libc::c_int, pmt: cptr, str: cptr, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn object_known(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_aware(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_value(o_ptr: *mut object_type) -> s32b;
    #[no_mangle]
    fn lookup_kind(tval: libc::c_int, sval: libc::c_int) -> s16b;
    #[no_mangle]
    fn object_wipe(o_ptr: *mut object_type);
    #[no_mangle]
    fn object_prep(o_ptr: *mut object_type, k_idx: libc::c_int);
    #[no_mangle]
    fn object_copy(o_ptr: *mut object_type, j_ptr: *mut object_type);
    #[no_mangle]
    static mut hack_apply_magic_power: libc::c_int;
    #[no_mangle]
    fn apply_magic(o_ptr: *mut object_type, lev: libc::c_int, okay: bool_,
                   good: bool_, great: bool_);
    #[no_mangle]
    fn make_object(j_ptr: *mut object_type, good: bool_, great: bool_,
                   theme: obj_theme) -> bool_;
    #[no_mangle]
    fn drop_near(o_ptr: *mut object_type, chance: libc::c_int, y: libc::c_int,
                 x: libc::c_int) -> s16b;
    #[no_mangle]
    fn acquirement(y1: libc::c_int, x1: libc::c_int, num: libc::c_int,
                   great: bool_, known: bool_);
    #[no_mangle]
    fn wiz_place_trap(y: libc::c_int, x: libc::c_int, idx: libc::c_int);
    #[no_mangle]
    static mut teleport_player_bypass: bool_;
    #[no_mangle]
    fn teleport_player(dis: libc::c_int);
    #[no_mangle]
    fn teleport_player_to(ny: libc::c_int, nx: libc::c_int);
    #[no_mangle]
    fn res_stat(stat: libc::c_int, full: bool_) -> bool_;
    #[no_mangle]
    fn remove_all_curse() -> bool_;
    #[no_mangle]
    fn restore_level() -> bool_;
    #[no_mangle]
    fn self_knowledge(fff: *mut FILE);
    #[no_mangle]
    fn detect_all(rad: libc::c_int) -> bool_;
    #[no_mangle]
    fn ident_spell() -> bool_;
    #[no_mangle]
    fn identify_fully() -> bool_;
    #[no_mangle]
    fn create_artifact(o_ptr: *mut object_type, a_scroll: bool_,
                       get_name: bool_) -> bool_;
    #[no_mangle]
    fn flush();
    #[no_mangle]
    fn inkey() -> libc::c_char;
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn get_string(prompt: cptr, buf: *mut libc::c_char, len: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn get_com(prompt: cptr, command: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn gain_fate(fate: byte_hack);
    #[no_mangle]
    fn set_blind(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_confused(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_poisoned(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_afraid(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_paralyzed(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_image(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_slow(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_stun(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_cut(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_food(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn check_experience();
    #[no_mangle]
    fn check_experience_obj(o_ptr: *mut object_type);
    #[no_mangle]
    fn gain_exp(amount: s32b);
    #[no_mangle]
    fn gain_level_reward(chosen_reward: libc::c_int);
    #[no_mangle]
    fn tgt_pt(x: *mut libc::c_int, y: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn gain_random_corruption(choose_mut: libc::c_int) -> bool_;
    #[no_mangle]
    fn make_wish();
    #[no_mangle]
    fn exec_lua(file: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn get_lua_var(name: cptr, type_0: libc::c_char, arg: *mut libc::c_void)
     -> bool_;
    #[no_mangle]
    fn status_main();
    /*
 * External function
 */
    #[no_mangle]
    fn do_cmd_spoilers();
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
pub struct artifact_type {
    pub name: u32b,
    pub text: u32b,
    pub tval: byte_hack,
    pub sval: byte_hack,
    pub pval: s16b,
    pub to_h: s16b,
    pub to_d: s16b,
    pub to_a: s16b,
    pub activate: s16b,
    pub ac: s16b,
    pub dd: byte_hack,
    pub ds: byte_hack,
    pub weight: s16b,
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
    pub level: byte_hack,
    pub rarity: byte_hack,
    pub cur_num: byte_hack,
    pub max_num: byte_hack,
    pub esp: u32b,
    pub oesp: u32b,
    pub power: s16b,
    pub set: s16b,
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
pub struct tval_desc2 {
    pub tval: libc::c_int,
    pub desc: cptr,
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
pub struct fate {
    pub fate: byte_hack,
    pub level: byte_hack,
    pub serious: byte_hack,
    pub o_idx: s16b,
    pub e_idx: s16b,
    pub a_idx: s16b,
    pub v_idx: s16b,
    pub r_idx: s16b,
    pub count: s16b,
    pub time: s16b,
    pub know: bool_,
    pub icky: bool_,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rule_type {
    pub mode: byte_hack,
    pub percent: byte_hack,
    pub mflags1: u32b,
    pub mflags2: u32b,
    pub mflags3: u32b,
    pub mflags4: u32b,
    pub mflags5: u32b,
    pub mflags6: u32b,
    pub mflags7: u32b,
    pub mflags8: u32b,
    pub mflags9: u32b,
    pub r_char: [libc::c_char; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dungeon_info_type {
    pub name: u32b,
    pub text: u32b,
    pub short_name: [libc::c_char; 3],
    pub generator: [libc::c_char; 30],
    pub floor1: s16b,
    pub floor_percent1: [byte_hack; 2],
    pub floor2: s16b,
    pub floor_percent2: [byte_hack; 2],
    pub floor3: s16b,
    pub floor_percent3: [byte_hack; 2],
    pub outer_wall: s16b,
    pub inner_wall: s16b,
    pub fill_type1: s16b,
    pub fill_percent1: [byte_hack; 2],
    pub fill_type2: s16b,
    pub fill_percent2: [byte_hack; 2],
    pub fill_type3: s16b,
    pub fill_percent3: [byte_hack; 2],
    pub fill_method: byte_hack,
    pub mindepth: s16b,
    pub maxdepth: s16b,
    pub principal: bool_,
    pub next: byte_hack,
    pub min_plev: byte_hack,
    pub min_m_alloc_level: libc::c_int,
    pub max_m_alloc_chance: libc::c_int,
    pub flags1: u32b,
    pub flags2: u32b,
    pub size_x: libc::c_int,
    pub size_y: libc::c_int,
    pub rule_percents: [byte_hack; 100],
    pub rules: [rule_type; 5],
    pub final_object: libc::c_int,
    pub final_artifact: libc::c_int,
    pub final_guardian: libc::c_int,
    pub ix: libc::c_int,
    pub iy: libc::c_int,
    pub ox: libc::c_int,
    pub oy: libc::c_int,
    pub objs: obj_theme,
    pub d_dice: [libc::c_int; 4],
    pub d_side: [libc::c_int; 4],
    pub d_frequency: [libc::c_int; 4],
    pub d_type: [libc::c_int; 4],
    pub t_idx: [s16b; 4],
    pub t_level: [s16b; 4],
    pub t_num: s16b,
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
/*
 * Adds a lvl to a monster
 */
#[no_mangle]
pub unsafe extern "C" fn wiz_inc_monster_level(mut level: libc::c_int) {
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    let mut ii: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    if tgt_pt(&mut ii, &mut jj) == 0 { return }
    if (*cave[jj as usize].offset(ii as isize)).m_idx != 0 {
        m_ptr =
            &mut *m_list.offset((*(*cave.as_mut_ptr().offset(jj as
                                                                 isize)).offset(ii
                                                                                    as
                                                                                    isize)).m_idx
                                    as isize) as *mut monster_type;
        (*m_ptr).exp =
            ((if (*m_ptr).level as libc::c_int + level > 150 as libc::c_int {
                  150 as libc::c_int
              } else { ((*m_ptr).level as libc::c_int) + level }) *
                 (if (*m_ptr).level as libc::c_int + level >
                         150 as libc::c_int {
                      150 as libc::c_int
                  } else { ((*m_ptr).level as libc::c_int) + level }) *
                 (if (*m_ptr).level as libc::c_int + level >
                         150 as libc::c_int {
                      150 as libc::c_int
                  } else { ((*m_ptr).level as libc::c_int) + level }) *
                 6 as libc::c_int) as u32b;
        monster_check_experience((*cave[jj as
                                            usize].offset(ii as isize)).m_idx
                                     as libc::c_int,
                                 0 as libc::c_int as bool_);
    };
}
#[no_mangle]
pub unsafe extern "C" fn wiz_align_monster(mut status: libc::c_int) {
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    let mut ii: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    if tgt_pt(&mut ii, &mut jj) == 0 { return }
    if (*cave[jj as usize].offset(ii as isize)).m_idx != 0 {
        m_ptr =
            &mut *m_list.offset((*(*cave.as_mut_ptr().offset(jj as
                                                                 isize)).offset(ii
                                                                                    as
                                                                                    isize)).m_idx
                                    as isize) as *mut monster_type;
        (*m_ptr).status = status as s16b
    };
}
/*
 * Teleport directly to a town
 */
#[no_mangle]
pub unsafe extern "C" fn teleport_player_town(mut town: libc::c_int) {
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    autosave_checkpoint();
    /* Change town */
    dun_level = 0 as libc::c_int as s16b;
    (*p_ptr).town_num = town as s16b;
    x = 0 as libc::c_int;
    's_18:
        while x < max_wild_x as libc::c_int {
            y = 0 as libc::c_int;
            while y < max_wild_y as libc::c_int {
                if (*p_ptr).town_num as libc::c_int ==
                       (*wf_info.offset((*(*wild_map.offset(y as
                                                                isize)).offset(x
                                                                                   as
                                                                                   isize)).feat
                                            as isize)).entrance as libc::c_int
                   {
                    break 's_18 ;
                }
                y += 1
            }
            x += 1
        }
    (*p_ptr).wilderness_y = y;
    (*p_ptr).wilderness_x = x;
    (*p_ptr).inside_arena = 0 as libc::c_int as s16b;
    leaving_quest = (*p_ptr).inside_quest as libc::c_int;
    (*p_ptr).inside_quest = 0 as libc::c_int as s16b;
    /* Leaving */
    (*p_ptr).leaving = 1 as libc::c_int as bool_;
}
/*
 * Hack -- Rerate Hitpoints
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_rerate() {
    let mut min_value: libc::c_int = 0;
    let mut max_value: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut percent: libc::c_int = 0;
    min_value =
        50 as libc::c_int * 3 as libc::c_int *
            ((*p_ptr).hitdie as libc::c_int - 1 as libc::c_int) /
            8 as libc::c_int;
    min_value += 50 as libc::c_int;
    max_value =
        50 as libc::c_int * 5 as libc::c_int *
            ((*p_ptr).hitdie as libc::c_int - 1 as libc::c_int) /
            8 as libc::c_int;
    max_value += 50 as libc::c_int;
    player_hp[0 as libc::c_int as usize] = (*p_ptr).hitdie as s16b;
    loop 
         /* Rerate */
         /* Collect values */
         {
        i = 1 as libc::c_int;
        while i < 50 as libc::c_int {
            player_hp[i as usize] =
                (Rand_div((*p_ptr).hitdie as s32b) + 1 as libc::c_int) as
                    s16b;
            player_hp[i as usize] =
                (player_hp[i as usize] as libc::c_int +
                     player_hp[(i - 1 as libc::c_int) as usize] as
                         libc::c_int) as s16b;
            i += 1
        }
        /* Legal values */
        if player_hp[(50 as libc::c_int - 1 as libc::c_int) as usize] as
               libc::c_int >= min_value &&
               player_hp[(50 as libc::c_int - 1 as libc::c_int) as usize] as
                   libc::c_int <= max_value {
            break ;
        }
    }
    percent =
        (player_hp[(50 as libc::c_int - 1 as libc::c_int) as usize] as
             libc::c_long * 200 as libc::c_long /
             ((*p_ptr).hitdie as libc::c_int +
                  (50 as libc::c_int - 1 as libc::c_int) *
                      (*p_ptr).hitdie as libc::c_int) as libc::c_long) as
            libc::c_int;
    /* Update and redraw hitpoints */
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x10 as libc::c_long) as u32b;
    (*p_ptr).redraw =
        ((*p_ptr).redraw as libc::c_long | 0x40 as libc::c_long) as u32b;
    /* Window stuff */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as u32b;
    /* Handle stuff */
    handle_stuff();
    /* Message */
    msg_format(b"Current Life Rating is %d/100.\x00" as *const u8 as
                   *const libc::c_char, percent);
}
/*
 * Create the artifact of the specified number -- DAN
 *
 */
unsafe extern "C" fn wiz_create_named_art() {
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
    let mut q_ptr: *mut object_type = 0 as *mut object_type;
    let mut i: libc::c_int = 0;
    let mut a_idx: libc::c_int = 0;
    let mut p: cptr =
        b"Number of the artifact :\x00" as *const u8 as *const libc::c_char;
    let mut out_val: [libc::c_char; 80] =
        *::std::mem::transmute::<&[u8; 80],
                                 &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let mut a_ptr: *mut artifact_type = 0 as *mut artifact_type;
    if get_string(p, out_val.as_mut_ptr(), 4 as libc::c_int) == 0 { return }
    a_idx = atoi(out_val.as_mut_ptr());
    /* Return if out-of-bounds */
    if a_idx <= 0 as libc::c_int || a_idx >= max_a_idx as libc::c_int {
        return
    }
    a_ptr = &mut *a_info.offset(a_idx as isize) as *mut artifact_type;
    /* Get local object */
    q_ptr = &mut forge;
    /* Wipe the object */
    object_wipe(q_ptr);
    /* Ignore "empty" artifacts */
    if (*a_ptr).name == 0 { return }
    /* Acquire the "kind" index */
    i =
        lookup_kind((*a_ptr).tval as libc::c_int,
                    (*a_ptr).sval as libc::c_int) as libc::c_int;
    /* Oops */
    if i == 0 { return }
    /* Create the artifact */
    object_prep(q_ptr, i);
    /* Save the name */
    (*q_ptr).name1 = a_idx as byte_hack;
    apply_magic(q_ptr, -(1 as libc::c_int), 1 as libc::c_int as bool_,
                1 as libc::c_int as bool_, 1 as libc::c_int as bool_);
    /* Identify it fully */
    object_aware(q_ptr);
    object_known(q_ptr);
    /* Mark the item as fully known */
    (*q_ptr).ident =
        ((*q_ptr).ident as libc::c_int | 0x20 as libc::c_int) as byte_hack;
    /* Drop the artifact from heaven */
    drop_near(q_ptr, -(1 as libc::c_int), (*p_ptr).py as libc::c_int,
              (*p_ptr).px as libc::c_int);
    /* All done */
    msg_print(b"Allocated.\x00" as *const u8 as *const libc::c_char);
}
/*
 * Hack -- quick debugging hook
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_wiz_hack_ben(mut num: libc::c_int) {
    let mut a: s32b = 0;
    /*        MAKE(r_ptr, monster_race);
	        COPY(r_ptr, &r_info[500], monster_race);

	        r_ptr->level = 1;
	        r_ptr->flags6 |= RF6_BLINK;
	        r_ptr->freq_inate = r_ptr->freq_spell = 90;

	        place_monster_one_race = r_ptr;
	        place_monster_one(p_ptr->py - 1, p_ptr->px, 500, 0, TRUE, MSTATUS_PET);*/
    get_lua_var(b"a\x00" as *const u8 as *const libc::c_char,
                'd' as i32 as libc::c_char,
                &mut a as *mut s32b as *mut libc::c_void);
    msg_format(b"a: %d\x00" as *const u8 as *const libc::c_char, a);
}
#[no_mangle]
pub unsafe extern "C" fn do_cmd_lua_script() {
    let mut script: [libc::c_char; 80] =
        *::std::mem::transmute::<&[u8; 80],
                                 &mut [libc::c_char; 80]>(b"tome_dofile_anywhere(ANGBAND_DIR_CORE, \'gen_idx.lua\')\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    if get_string(b"Script:\x00" as *const u8 as *const libc::c_char,
                  script.as_mut_ptr(), 80 as libc::c_int) == 0 {
        return
    }
    exec_lua(script.as_mut_ptr());
}
/* Summon a horde of monsters */
unsafe extern "C" fn do_cmd_summon_horde() {
    let mut wy: libc::c_int = (*p_ptr).py as libc::c_int;
    let mut wx: libc::c_int = (*p_ptr).px as libc::c_int;
    let mut attempts: libc::c_int = 1000 as libc::c_int;
    loop  {
        attempts -= 1;
        if !(attempts != 0) { break ; }
        scatter(&mut wy, &mut wx, (*p_ptr).py as libc::c_int,
                (*p_ptr).px as libc::c_int, 3 as libc::c_int);
        if (*f_info.offset((*cave[wy as usize].offset(wx as isize)).feat as
                               isize)).flags1 as libc::c_long &
               0x10 as libc::c_long != 0 &&
               (*cave[wy as usize].offset(wx as isize)).feat as libc::c_int !=
                   0xaf as libc::c_int &&
               (*f_info.offset((*cave[wy as usize].offset(wx as isize)).feat
                                   as isize)).flags1 as libc::c_long &
                   0x40 as libc::c_long == 0 &&
               (*cave[wy as usize].offset(wx as isize)).o_idx as libc::c_int
                   == 0 as libc::c_int &&
               (*cave[wy as usize].offset(wx as isize)).m_idx as libc::c_int
                   == 0 as libc::c_int {
            break ;
        }
    }
    alloc_horde(wy, wx);
}
/* MONSTER_HORDES */
/*
 * Output a long int in binary format.
 */
unsafe extern "C" fn prt_binary(mut flags: u32b, mut row: libc::c_int,
                                mut col: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut bitmask: u32b = 0;
    /* Scan the flags */
    bitmask = 1 as libc::c_int as u32b;
    i = bitmask as libc::c_int;
    while i <= 32 as libc::c_int {
        /* Dump set bits */
        if flags & bitmask != 0 {
            let fresh0 = col;
            col = col + 1;
            Term_putch(fresh0, row, 6 as libc::c_int as byte_hack,
                       '*' as i32 as libc::c_char);
        } else {
            /* Dump unset bits */
            let fresh1 = col;
            col = col + 1;
            Term_putch(fresh1, row, 1 as libc::c_int as byte_hack,
                       '-' as i32 as libc::c_char);
        }
        i += 1;
        bitmask =
            (bitmask as
                 libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint)
                as u32b as u32b
    };
}
/*
 * Hack -- Teleport to the target
 */
unsafe extern "C" fn do_cmd_wiz_bamf() {
    /* Must have a target */
    if target_who == 0 { return }
    /* Teleport to the target */
    teleport_player_to(target_row as libc::c_int, target_col as libc::c_int);
}
/*
 * Aux function for "do_cmd_wiz_change()".	-RAK-
 */
unsafe extern "C" fn do_cmd_wiz_change_aux() {
    let mut i: libc::c_int = 0;
    let mut tmp_int: libc::c_int = 0;
    let mut tmp_long: libc::c_long = 0;
    let mut tmp_val: [libc::c_char; 160] = [0; 160];
    let mut ppp: [libc::c_char; 80] = [0; 80];
    /* Query the stats */
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        /* Prompt */
        sprintf(ppp.as_mut_ptr(),
                b"%s:  (3-118): \x00" as *const u8 as *const libc::c_char,
                stat_names[i as usize]);
        /* Default */
        sprintf(tmp_val.as_mut_ptr(),
                b"%d\x00" as *const u8 as *const libc::c_char,
                (*p_ptr).stat_max[i as usize] as libc::c_int);
        /* Query */
        if get_string(ppp.as_mut_ptr() as cptr, tmp_val.as_mut_ptr(),
                      3 as libc::c_int) == 0 {
            return
        }
        /* Extract */
        tmp_int = atoi(tmp_val.as_mut_ptr());
        /* Verify */
        if tmp_int > 18 as libc::c_int + 100 as libc::c_int {
            tmp_int = 18 as libc::c_int + 100 as libc::c_int
        } else if tmp_int < 3 as libc::c_int { tmp_int = 3 as libc::c_int }
        /* Save it */
        (*p_ptr).stat_max[i as usize] = tmp_int as s16b;
        (*p_ptr).stat_cur[i as usize] = (*p_ptr).stat_max[i as usize];
        i += 1
    }
    /* Default */
    sprintf(tmp_val.as_mut_ptr(),
            b"%ld\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).au as libc::c_long);
    /* Query */
    if get_string(b"Gold: \x00" as *const u8 as *const libc::c_char,
                  tmp_val.as_mut_ptr(), 9 as libc::c_int) == 0 {
        return
    }
    /* Extract */
    tmp_long = atol(tmp_val.as_mut_ptr());
    /* Verify */
    if tmp_long < 0 as libc::c_int as libc::c_long {
        tmp_long = 0 as libc::c_long
    }
    /* Save */
    (*p_ptr).au = tmp_long as s32b;
    /* Default */
    sprintf(tmp_val.as_mut_ptr(),
            b"%ld\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).max_exp as libc::c_long);
    /* Query */
    if get_string(b"Experience: \x00" as *const u8 as *const libc::c_char,
                  tmp_val.as_mut_ptr(), 9 as libc::c_int) == 0 {
        return
    }
    /* Extract */
    tmp_long = atol(tmp_val.as_mut_ptr());
    /* Verify */
    if tmp_long < 0 as libc::c_int as libc::c_long {
        tmp_long = 0 as libc::c_long
    }
    /* Save */
    (*p_ptr).max_exp = tmp_long as s32b;
    (*p_ptr).exp = tmp_long as s32b;
    /* Update */
    check_experience();
    /* Default */
    sprintf(tmp_val.as_mut_ptr(),
            b"%d\x00" as *const u8 as *const libc::c_char,
            (*p_ptr).luck_base as libc::c_int);
    /* Query */
    if get_string(b"Luck(base): \x00" as *const u8 as *const libc::c_char,
                  tmp_val.as_mut_ptr(), 3 as libc::c_int) == 0 {
        return
    }
    /* Extract */
    tmp_long = atol(tmp_val.as_mut_ptr());
    /* Save */
    (*p_ptr).luck_base = tmp_long as s16b;
    (*p_ptr).luck_max = tmp_long as s16b;
}
/*
 * Change various "permanent" player variables.
 */
unsafe extern "C" fn do_cmd_wiz_change() {
    /* Interact */
    do_cmd_wiz_change_aux();
    /* Redraw everything */
    do_cmd_redraw();
}
/*
 * Wizard routines for creating objects		-RAK-
 * And for manipulating them!                   -Bernd-
 *
 * This has been rewritten to make the whole procedure
 * of debugging objects much easier and more comfortable.
 *
 * The following functions are meant to play with objects:
 * Create, modify, roll for them (for statistic purposes) and more.
 * The original functions were by RAK.
 * The function to show an item's debug information was written
 * by David Reeve Sward <sward+@CMU.EDU>.
 *                             Bernd (wiebelt@mathematik.hu-berlin.de)
 *
 * Here are the low-level functions
 * - wiz_display_item()
 *     display an item's debug-info
 * - wiz_create_itemtype()
 *     specify tval and sval (type and subtype of object)
 * - wiz_tweak_item()
 *     specify pval, +AC, +tohit, +todam
 *     Note that the wizard can leave this function anytime,
 *     thus accepting the default-values for the remaining values.
 *     pval comes first now, since it is most important.
 * - wiz_reroll_item()
 *     apply some magic to the item or turn it into an artifact.
 * - wiz_roll_item()
 *     Get some statistics about the rarity of an item:
 *     We create a lot of fake items and see if they are of the
 *     same type (tval and sval), then we compare pval and +AC.
 *     If the fake-item is better or equal it is counted.
 *     Note that cursed items that are better or equal (absolute values)
 *     are counted, too.
 *     HINT: This is *very* useful for balancing the game!
 * - wiz_quantity_item()
 *     change the quantity of an item, but be sane about it.
 *
 * And now the high-level functions
 * - do_cmd_wiz_play()
 *     play with an existing object
 * - wiz_create_item()
 *     create a new object
 *
 * Note -- You do not have to specify "pval" and other item-properties
 * directly. Just apply magic until you are satisfied with the item.
 *
 * Note -- For some items (such as wands, staffs, some rings, etc), you
 * must apply magic, or you will get "broken" or "uncharged" objects.
 *
 * Note -- Redefining artifacts via "do_cmd_wiz_play()" may destroy
 * the artifact.  Be careful.
 *
 * Hack -- this function will allow you to create multiple artifacts.
 * This "feature" may induce crashes or other nasty effects.
 */
/*
 * Just display an item's properties (debug-info)
 * Originally by David Reeve Sward <sward+@CMU.EDU>
 * Verbose item flags by -Bernd-
 */
unsafe extern "C" fn wiz_display_item(mut o_ptr: *mut object_type) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 13 as libc::c_int;
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut buf: [libc::c_char; 256] = [0; 256];
    /* Extract the flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Clear the screen */
    i = 1 as libc::c_int;
    while i <= 23 as libc::c_int {
        prt(b"\x00" as *const u8 as *const libc::c_char, i,
            j - 2 as libc::c_int);
        i += 1
    }
    /* Describe fully */
    object_desc_store(buf.as_mut_ptr(), o_ptr, 1 as libc::c_int,
                      3 as libc::c_int);
    prt(buf.as_mut_ptr() as cptr, 2 as libc::c_int, j);
    prt(format(b"kind = %-5d  level = %-4d  tval = %-5d  sval = %-5d\x00" as
                   *const u8 as *const libc::c_char,
               (*o_ptr).k_idx as libc::c_int,
               (*k_info.offset((*o_ptr).k_idx as isize)).level as libc::c_int,
               (*o_ptr).tval as libc::c_int, (*o_ptr).sval as libc::c_int) as
            cptr, 4 as libc::c_int, j);
    prt(format(b"number = %-3d  wgt = %-6d  ac = %-5d    damage = %dd%d\x00"
                   as *const u8 as *const libc::c_char,
               (*o_ptr).number as libc::c_int, (*o_ptr).weight,
               (*o_ptr).ac as libc::c_int, (*o_ptr).dd as libc::c_int,
               (*o_ptr).ds as libc::c_int) as cptr, 5 as libc::c_int, j);
    prt(format(b"pval = %-5d  toac = %-5d  tohit = %-4d  todam = %-4d\x00" as
                   *const u8 as *const libc::c_char, (*o_ptr).pval,
               (*o_ptr).to_a as libc::c_int, (*o_ptr).to_h as libc::c_int,
               (*o_ptr).to_d as libc::c_int) as cptr, 6 as libc::c_int, j);
    prt(format(b"name1 = %-4d  name2 = %-4d  cost = %ld  pval2 = %-5d\x00" as
                   *const u8 as *const libc::c_char,
               (*o_ptr).name1 as libc::c_int, (*o_ptr).name2 as libc::c_int,
               object_value(o_ptr) as libc::c_long,
               (*o_ptr).pval2 as libc::c_int) as cptr, 7 as libc::c_int, j);
    prt(format(b"ident = %04x  timeout = %-d\x00" as *const u8 as
                   *const libc::c_char, (*o_ptr).ident as libc::c_int,
               (*o_ptr).timeout as libc::c_int) as cptr, 8 as libc::c_int, j);
    prt(b"+------------FLAGS1------------+\x00" as *const u8 as
            *const libc::c_char, 10 as libc::c_int, j);
    prt(b"AFFECT........SLAY........BRAND.\x00" as *const u8 as
            *const libc::c_char, 11 as libc::c_int, j);
    prt(b"              cvae      xsqpaefc\x00" as *const u8 as
            *const libc::c_char, 12 as libc::c_int, j);
    prt(b"siwdcc  ssidsahanvudotgddhuoclio\x00" as *const u8 as
            *const libc::c_char, 13 as libc::c_int, j);
    prt(b"tnieoh  trnipttmiinmrrnrrraiierl\x00" as *const u8 as
            *const libc::c_char, 14 as libc::c_int, j);
    prt(b"rtsxna..lcfgdkcpmldncltggpksdced\x00" as *const u8 as
            *const libc::c_char, 15 as libc::c_int, j);
    prt_binary(f1, 16 as libc::c_int, j);
    prt(b"+------------FLAGS2------------+\x00" as *const u8 as
            *const libc::c_char, 17 as libc::c_int, j);
    prt(b"SUST....IMMUN.RESIST............\x00" as *const u8 as
            *const libc::c_char, 18 as libc::c_int, j);
    prt(b"        aefcprpsaefcpfldbc sn   \x00" as *const u8 as
            *const libc::c_char, 19 as libc::c_int, j);
    prt(b"siwdcc  cliooeatcliooeialoshtncd\x00" as *const u8 as
            *const libc::c_char, 20 as libc::c_int, j);
    prt(b"tnieoh  ierlifraierliatrnnnrhehi\x00" as *const u8 as
            *const libc::c_char, 21 as libc::c_int, j);
    prt(b"rtsxna..dcedslatdcedsrekdfddrxss\x00" as *const u8 as
            *const libc::c_char, 22 as libc::c_int, j);
    prt_binary(f2, 23 as libc::c_int, j);
    prt(b"+------------FLAGS3------------+\x00" as *const u8 as
            *const libc::c_char, 10 as libc::c_int, j + 32 as libc::c_int);
    prt(b"fe      ehsi  st    iiiiadta  hp\x00" as *const u8 as
            *const libc::c_char, 11 as libc::c_int, j + 32 as libc::c_int);
    prt(b"il   n taihnf ee    ggggcregb vr\x00" as *const u8 as
            *const libc::c_char, 12 as libc::c_int, j + 32 as libc::c_int);
    prt(b"re  nowysdose eld   nnnntalrl ym\x00" as *const u8 as
            *const libc::c_char, 13 as libc::c_int, j + 32 as libc::c_int);
    prt(b"ec  omrcyewta ieirmsrrrriieaeccc\x00" as *const u8 as
            *const libc::c_char, 14 as libc::c_int, j + 32 as libc::c_int);
    prt(b"aa  taauktmatlnpgeihaefcvnpvsuuu\x00" as *const u8 as
            *const libc::c_char, 15 as libc::c_int, j + 32 as libc::c_int);
    prt(b"uu  egirnyoahivaeggoclioaeoasrrr\x00" as *const u8 as
            *const libc::c_char, 16 as libc::c_int, j + 32 as libc::c_int);
    prt(b"rr  litsopdretitsehtierltxrtesss\x00" as *const u8 as
            *const libc::c_char, 17 as libc::c_int, j + 32 as libc::c_int);
    prt(b"aa  echewestreshtntsdcedeptedeee\x00" as *const u8 as
            *const libc::c_char, 18 as libc::c_int, j + 32 as libc::c_int);
    prt_binary(f3, 19 as libc::c_int, j + 32 as libc::c_int);
}
/*
 * A list of tvals and their textual names
 */
#[no_mangle]
pub static mut tvals: [tval_desc2; 47] =
    [{
         let mut init =
             tval_desc2{tval: 23 as libc::c_int,
                        desc:
                            b"Sword\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 22 as libc::c_int,
                        desc:
                            b"Polearm\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 21 as libc::c_int,
                        desc:
                            b"Hafted Weapon\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 24 as libc::c_int,
                        desc:
                            b"Axe\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 19 as libc::c_int,
                        desc:
                            b"Bow\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 15 as libc::c_int,
                        desc:
                            b"Boomerang\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 17 as libc::c_int,
                        desc:
                            b"Arrows\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 18 as libc::c_int,
                        desc:
                            b"Bolts\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 16 as libc::c_int,
                        desc:
                            b"Shots\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 34 as libc::c_int,
                        desc:
                            b"Shield\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 33 as libc::c_int,
                        desc:
                            b"Crown\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 32 as libc::c_int,
                        desc:
                            b"Helm\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 31 as libc::c_int,
                        desc:
                            b"Gloves\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 30 as libc::c_int,
                        desc:
                            b"Boots\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 35 as libc::c_int,
                        desc:
                            b"Cloak\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 38 as libc::c_int,
                        desc:
                            b"Dragon Scale Mail\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 37 as libc::c_int,
                        desc:
                            b"Hard Armor\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 36 as libc::c_int,
                        desc:
                            b"Soft Armor\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 45 as libc::c_int,
                        desc:
                            b"Ring\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 40 as libc::c_int,
                        desc:
                            b"Amulet\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 39 as libc::c_int,
                        desc:
                            b"Lite\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 71 as libc::c_int,
                        desc:
                            b"Potion\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 72 as libc::c_int,
                        desc:
                            b"Potion\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 70 as libc::c_int,
                        desc:
                            b"Scroll\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 65 as libc::c_int,
                        desc:
                            b"Wand\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 55 as libc::c_int,
                        desc:
                            b"Staff\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 67 as libc::c_int,
                        desc:
                            b"Rod\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 66 as libc::c_int,
                        desc:
                            b"Rod Tip\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 111 as libc::c_int,
                        desc:
                            b"Schools Spellbook\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 112 as libc::c_int,
                        desc:
                            b"Symbiotic Spellbook\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 114 as libc::c_int,
                        desc:
                            b"Elemental Stone\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 113 as libc::c_int,
                        desc:
                            b"Music Book\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 115 as libc::c_int,
                        desc:
                            b"Daemon Book\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 5 as libc::c_int,
                        desc:
                            b"Spikes\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 20 as libc::c_int,
                        desc:
                            b"Digger\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 7 as libc::c_int,
                        desc:
                            b"Chest\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 80 as libc::c_int,
                        desc:
                            b"Food\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 77 as libc::c_int,
                        desc:
                            b"Flask\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 6 as libc::c_int,
                        desc:
                            b"Mage Staff\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 4 as libc::c_int,
                        desc:
                            b"Essence\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 8 as libc::c_int,
                        desc:
                            b"Parchment\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 14 as libc::c_int,
                        desc:
                            b"Musical Instrument\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 104 as libc::c_int,
                        desc:
                            b"Rune 1\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 105 as libc::c_int,
                        desc:
                            b"Rune 2\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 11 as libc::c_int,
                        desc:
                            b"Junk\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             tval_desc2{tval: 46 as libc::c_int,
                        desc:
                            b"Trapping Kit\x00" as *const u8 as
                                *const libc::c_char,};
         init
     },
     {
         let mut init = tval_desc2{tval: 0 as libc::c_int, desc: 0 as cptr,};
         init
     }];
/*
 * Strip an "object name" into a buffer
 */
unsafe extern "C" fn strip_name(mut buf: *mut libc::c_char,
                                mut k_idx: libc::c_int) {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset(k_idx as isize) as *mut object_kind;
    let mut str: cptr = k_name.offset((*k_ptr).name as isize) as cptr;
    /* Skip past leading characters */
    while *str as libc::c_int == ' ' as i32 ||
              *str as libc::c_int == '&' as i32 {
        str = str.offset(1)
    }
    /* Copy useful chars */
    t = buf;
    while *str != 0 {
        if *str as libc::c_int != '~' as i32 {
            let fresh2 = t;
            t = t.offset(1);
            *fresh2 = *str
        }
        str = str.offset(1)
    }
    /* Terminate the new name */
    *t = '\u{0}' as i32 as libc::c_char;
}
/*
 * Hack -- title for each column
 *
 * XXX XXX XXX This will not work with "EBCDIC", I would think.
 */
static mut head: [libc::c_char; 4] =
    ['a' as i32 as libc::c_char, 'A' as i32 as libc::c_char,
     '0' as i32 as libc::c_char, ':' as i32 as libc::c_char];
/*
 * Print a string as required by wiz_create_itemtype().
 * Trims characters from the beginning until it fits in the space
 * before the next row or the edge of the screen.
 */
unsafe extern "C" fn wci_string(mut string: cptr, mut num: libc::c_int) {
    let mut row: libc::c_int = 2 as libc::c_int + num % 20 as libc::c_int;
    let mut col: libc::c_int = 30 as libc::c_int * (num / 20 as libc::c_int);
    let mut ch: libc::c_int =
        head[(num / 20 as libc::c_int) as usize] as libc::c_int +
            num % 20 as libc::c_int;
    let mut max_len: libc::c_int = 0 as libc::c_int;
    if 76 as libc::c_int - col < max_len {
        max_len = 76 as libc::c_int - col
    } else { max_len = 30 as libc::c_int - 6 as libc::c_int }
    if strlen(string) > max_len as libc::c_uint as libc::c_ulong {
        string =
            string.offset(strlen(string).wrapping_sub(max_len as
                                                          libc::c_ulong) as
                              isize)
    }
    prt(format(b"[%c] %s\x00" as *const u8 as *const libc::c_char, ch, string)
            as cptr, row, col);
}
/*
 * Specify tval and sval (type and subtype of object) originally
 * by RAK, heavily modified by -Bernd-
 *
 * This function returns the k_idx of an object type, or zero if failed
 *
 * List up to 50 choices in three columns
 */
unsafe extern "C" fn wiz_create_itemtype() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut max_num: libc::c_int = 0;
    let mut tval: libc::c_int = 0;
    let mut tval_desc2: cptr = 0 as *const libc::c_char;
    let mut ch: libc::c_char = 0;
    let mut choice: [libc::c_int; 60] = [0; 60];
    let mut buf: [libc::c_char; 160] = [0; 160];
    /* Clear screen */
    Term_clear();
    /* Print all tval's and their descriptions */
    num = 0 as libc::c_int;
    while num < 60 as libc::c_int && tvals[num as usize].tval != 0 {
        wci_string(tvals[num as usize].desc, num);
        num += 1
    }
    /* Me need to know the maximal possible tval_index */
    max_num = num;
    /* Choose! */
    if get_com(b"Get what type of object? \x00" as *const u8 as
                   *const libc::c_char, &mut ch) == 0 {
        return 0 as libc::c_int
    }
    /* Analyze choice */
    num = -(1 as libc::c_int);
    if ch as libc::c_int >= head[0 as libc::c_int as usize] as libc::c_int &&
           (ch as libc::c_int) <
               head[0 as libc::c_int as usize] as libc::c_int +
                   20 as libc::c_int {
        num =
            ch as libc::c_int - head[0 as libc::c_int as usize] as libc::c_int
    }
    if ch as libc::c_int >= head[1 as libc::c_int as usize] as libc::c_int &&
           (ch as libc::c_int) <
               head[1 as libc::c_int as usize] as libc::c_int +
                   20 as libc::c_int {
        num =
            ch as libc::c_int - head[1 as libc::c_int as usize] as libc::c_int
                + 20 as libc::c_int
    }
    if ch as libc::c_int >= head[2 as libc::c_int as usize] as libc::c_int &&
           (ch as libc::c_int) <
               head[2 as libc::c_int as usize] as libc::c_int +
                   17 as libc::c_int {
        num =
            ch as libc::c_int - head[2 as libc::c_int as usize] as libc::c_int
                + 40 as libc::c_int
    }
    /* Bail out if choice is illegal */
    if num < 0 as libc::c_int || num >= max_num { return 0 as libc::c_int }
    /* Base object type chosen, fill in tval */
    tval = tvals[num as usize].tval;
    tval_desc2 = tvals[num as usize].desc;
    /* ** And now we go for k_idx ***/
    /* Clear screen */
    Term_clear();
    /* We have to search the whole itemlist. */
    num = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while num < 60 as libc::c_int && i < max_k_idx as libc::c_int {
        let mut k_ptr: *mut object_kind =
            &mut *k_info.offset(i as isize) as *mut object_kind;
        /* Analyze matching items */
        if (*k_ptr).tval as libc::c_int == tval {
            /* Hack -- Skip instant artifacts */
            if !((*k_ptr).flags3 as libc::c_long & 0x800 as libc::c_long != 0)
               {
                /* Acquire the "name" of object "i" */
                strip_name(buf.as_mut_ptr(), i);
                /* Print it */
                wci_string(buf.as_mut_ptr() as cptr, num);
                /* Remember the object index */
                let fresh3 = num;
                num = num + 1;
                choice[fresh3 as usize] = i
            }
        }
        i += 1
    }
    /* Me need to know the maximal possible remembered object_index */
    max_num = num;
    /* Choose! */
    if get_com(format(b"What Kind of %s? \x00" as *const u8 as
                          *const libc::c_char, tval_desc2) as cptr, &mut ch)
           == 0 {
        return 0 as libc::c_int
    }
    /* Analyze choice */
    num = -(1 as libc::c_int);
    if ch as libc::c_int >= head[0 as libc::c_int as usize] as libc::c_int &&
           (ch as libc::c_int) <
               head[0 as libc::c_int as usize] as libc::c_int +
                   20 as libc::c_int {
        num =
            ch as libc::c_int - head[0 as libc::c_int as usize] as libc::c_int
    }
    if ch as libc::c_int >= head[1 as libc::c_int as usize] as libc::c_int &&
           (ch as libc::c_int) <
               head[1 as libc::c_int as usize] as libc::c_int +
                   20 as libc::c_int {
        num =
            ch as libc::c_int - head[1 as libc::c_int as usize] as libc::c_int
                + 20 as libc::c_int
    }
    if ch as libc::c_int >= head[2 as libc::c_int as usize] as libc::c_int &&
           (ch as libc::c_int) <
               head[2 as libc::c_int as usize] as libc::c_int +
                   17 as libc::c_int {
        num =
            ch as libc::c_int - head[2 as libc::c_int as usize] as libc::c_int
                + 40 as libc::c_int
    }
    /* Bail out if choice is "illegal" */
    if num < 0 as libc::c_int || num >= max_num { return 0 as libc::c_int }
    /* And return successful */
    return choice[num as usize];
}
/*
 * Tweak an item
 */
unsafe extern "C" fn wiz_tweak_item(mut o_ptr: *mut object_type) {
    let mut p: cptr = 0 as *const libc::c_char;
    let mut tmp_val: [libc::c_char; 80] = [0; 80];
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    /* Extract the flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    p =
        b"Enter new \'pval\' setting: \x00" as *const u8 as
            *const libc::c_char;
    sprintf(tmp_val.as_mut_ptr(),
            b"%ld\x00" as *const u8 as *const libc::c_char,
            (*o_ptr).pval as libc::c_long);
    if get_string(p, tmp_val.as_mut_ptr(), 5 as libc::c_int) == 0 { return }
    (*o_ptr).pval = atoi(tmp_val.as_mut_ptr());
    wiz_display_item(o_ptr);
    p =
        b"Enter new \'pval2\' setting: \x00" as *const u8 as
            *const libc::c_char;
    sprintf(tmp_val.as_mut_ptr(),
            b"%d\x00" as *const u8 as *const libc::c_char,
            (*o_ptr).pval2 as libc::c_int);
    if get_string(p, tmp_val.as_mut_ptr(), 5 as libc::c_int) == 0 { return }
    (*o_ptr).pval2 = atoi(tmp_val.as_mut_ptr()) as s16b;
    wiz_display_item(o_ptr);
    p =
        b"Enter new \'pval3\' setting: \x00" as *const u8 as
            *const libc::c_char;
    sprintf(tmp_val.as_mut_ptr(),
            b"%ld\x00" as *const u8 as *const libc::c_char,
            (*o_ptr).pval3 as libc::c_long);
    if get_string(p, tmp_val.as_mut_ptr(), 5 as libc::c_int) == 0 { return }
    (*o_ptr).pval3 = atoi(tmp_val.as_mut_ptr());
    wiz_display_item(o_ptr);
    p =
        b"Enter new \'to_a\' setting: \x00" as *const u8 as
            *const libc::c_char;
    sprintf(tmp_val.as_mut_ptr(),
            b"%d\x00" as *const u8 as *const libc::c_char,
            (*o_ptr).to_a as libc::c_int);
    if get_string(p, tmp_val.as_mut_ptr(), 5 as libc::c_int) == 0 { return }
    (*o_ptr).to_a = atoi(tmp_val.as_mut_ptr()) as s16b;
    wiz_display_item(o_ptr);
    p =
        b"Enter new \'to_h\' setting: \x00" as *const u8 as
            *const libc::c_char;
    sprintf(tmp_val.as_mut_ptr(),
            b"%d\x00" as *const u8 as *const libc::c_char,
            (*o_ptr).to_h as libc::c_int);
    if get_string(p, tmp_val.as_mut_ptr(), 5 as libc::c_int) == 0 { return }
    (*o_ptr).to_h = atoi(tmp_val.as_mut_ptr()) as s16b;
    wiz_display_item(o_ptr);
    p =
        b"Enter new \'to_d\' setting: \x00" as *const u8 as
            *const libc::c_char;
    sprintf(tmp_val.as_mut_ptr(),
            b"%d\x00" as *const u8 as *const libc::c_char,
            (*o_ptr).to_d as libc::c_int);
    if get_string(p, tmp_val.as_mut_ptr(), 5 as libc::c_int) == 0 { return }
    (*o_ptr).to_d = atoi(tmp_val.as_mut_ptr()) as s16b;
    wiz_display_item(o_ptr);
    p =
        b"Enter new \'name2\' setting: \x00" as *const u8 as
            *const libc::c_char;
    sprintf(tmp_val.as_mut_ptr(),
            b"%d\x00" as *const u8 as *const libc::c_char,
            (*o_ptr).name2 as libc::c_int);
    if get_string(p, tmp_val.as_mut_ptr(), 5 as libc::c_int) == 0 { return }
    (*o_ptr).name2 = atoi(tmp_val.as_mut_ptr()) as s16b;
    wiz_display_item(o_ptr);
    p =
        b"Enter new \'name2b\' setting: \x00" as *const u8 as
            *const libc::c_char;
    sprintf(tmp_val.as_mut_ptr(),
            b"%d\x00" as *const u8 as *const libc::c_char,
            (*o_ptr).name2b as libc::c_int);
    if get_string(p, tmp_val.as_mut_ptr(), 5 as libc::c_int) == 0 { return }
    (*o_ptr).name2b = atoi(tmp_val.as_mut_ptr()) as s16b;
    wiz_display_item(o_ptr);
    p =
        b"Enter new \'sval\' setting: \x00" as *const u8 as
            *const libc::c_char;
    sprintf(tmp_val.as_mut_ptr(),
            b"%d\x00" as *const u8 as *const libc::c_char,
            (*o_ptr).sval as libc::c_int);
    if get_string(p, tmp_val.as_mut_ptr(), 5 as libc::c_int) == 0 { return }
    (*o_ptr).sval = atoi(tmp_val.as_mut_ptr()) as byte_hack;
    wiz_display_item(o_ptr);
    p =
        b"Enter new \'obj exp\' setting: \x00" as *const u8 as
            *const libc::c_char;
    sprintf(tmp_val.as_mut_ptr(),
            b"%ld\x00" as *const u8 as *const libc::c_char,
            (*o_ptr).exp as libc::c_long);
    if get_string(p, tmp_val.as_mut_ptr(), 9 as libc::c_int) == 0 { return }
    wiz_display_item(o_ptr);
    (*o_ptr).exp = atoi(tmp_val.as_mut_ptr());
    if f4 as libc::c_long & 0x100 as libc::c_long != 0 {
        check_experience_obj(o_ptr);
    }
    p =
        b"Enter new \'timeout\' setting: \x00" as *const u8 as
            *const libc::c_char;
    sprintf(tmp_val.as_mut_ptr(),
            b"%d\x00" as *const u8 as *const libc::c_char,
            (*o_ptr).timeout as libc::c_int);
    if get_string(p, tmp_val.as_mut_ptr(), 5 as libc::c_int) == 0 { return }
    (*o_ptr).timeout = atoi(tmp_val.as_mut_ptr()) as s16b;
    wiz_display_item(o_ptr);
}
/*
 * Apply magic to an item or turn it into an artifact. -Bernd-
 */
unsafe extern "C" fn wiz_reroll_item(mut o_ptr: *mut object_type) {
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
    let mut q_ptr: *mut object_type = 0 as *mut object_type;
    let mut ch: libc::c_char = 0;
    let mut changed: bool_ = 0 as libc::c_int as bool_;
    /* Hack -- leave artifacts alone */
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
            } else { 0 as libc::c_int }) != 0 ||
           (*o_ptr).art_name as libc::c_int != 0 {
        return
    }
    /* Get local object */
    q_ptr = &mut forge;
    /* Copy the object */
    object_copy(q_ptr, o_ptr);
    loop 
         /* Main loop. Ask for magification and artifactification */
         /* Display full item debug information */
         {
        wiz_display_item(q_ptr);
        /* Ask wizard what to do. */
        if get_com(b"[a]ccept, [b]ad, [n]ormal, [g]ood, [e]xcellent, [r]andart? \x00"
                       as *const u8 as *const libc::c_char, &mut ch) == 0 {
            changed = 0 as libc::c_int as bool_;
            break ;
        } else if ch as libc::c_int == 'A' as i32 ||
                      ch as libc::c_int == 'a' as i32 {
            changed = 1 as libc::c_int as bool_;
            break ;
        } else if ch as libc::c_int == 'b' as i32 ||
                      ch as libc::c_int == 'B' as i32 {
            object_prep(q_ptr, (*o_ptr).k_idx as libc::c_int);
            hack_apply_magic_power = -(2 as libc::c_int);
            apply_magic(q_ptr, dun_level as libc::c_int,
                        0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
                        0 as libc::c_int as bool_);
        } else if ch as libc::c_int == 'n' as i32 ||
                      ch as libc::c_int == 'N' as i32 {
            object_prep(q_ptr, (*o_ptr).k_idx as libc::c_int);
            apply_magic(q_ptr, dun_level as libc::c_int,
                        0 as libc::c_int as bool_, 0 as libc::c_int as bool_,
                        0 as libc::c_int as bool_);
        } else if ch as libc::c_int == 'g' as i32 ||
                      ch as libc::c_int == 'g' as i32 {
            object_prep(q_ptr, (*o_ptr).k_idx as libc::c_int);
            apply_magic(q_ptr, dun_level as libc::c_int,
                        0 as libc::c_int as bool_, 1 as libc::c_int as bool_,
                        0 as libc::c_int as bool_);
        } else if ch as libc::c_int == 'e' as i32 ||
                      ch as libc::c_int == 'e' as i32 {
            object_prep(q_ptr, (*o_ptr).k_idx as libc::c_int);
            apply_magic(q_ptr, dun_level as libc::c_int,
                        0 as libc::c_int as bool_, 1 as libc::c_int as bool_,
                        1 as libc::c_int as bool_);
        } else if ch as libc::c_int == 'r' as i32 ||
                      ch as libc::c_int == 'r' as i32 {
            object_prep(q_ptr, (*o_ptr).k_idx as libc::c_int);
            create_artifact(q_ptr, 0 as libc::c_int as bool_,
                            1 as libc::c_int as bool_);
        }
    }
    /* Create/change it! */
    /* Apply bad magic, but first clear object */
    /* Apply normal magic, but first clear object */
    /* Apply good magic, but first clear object */
    /* Apply great magic, but first clear object */
    /* Apply great magic, but first clear object */
    /* Notice change */
    if changed != 0 {
        /* Apply changes */
        object_copy(o_ptr, q_ptr);
        /* Recalculate bonuses */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
        /* Combine / Reorder the pack (later) */
        (*p_ptr).notice =
            ((*p_ptr).notice as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long |
                      0x8 as libc::c_long)) as u32b
    };
}
/*
 * Try to create an item again. Output some statistics.    -Bernd-
 *
 * The statistics are correct now.  We acquire a clean grid, and then
 * repeatedly place an object in this grid, copying it into an item
 * holder, and then deleting the object.  We fiddle with the artifact
 * counter flags to prevent weirdness.  We use the items to collect
 * statistics on item creation relative to the initial item.
 */
unsafe extern "C" fn wiz_statistics(mut o_ptr: *mut object_type) {
    let mut i: libc::c_long = 0;
    let mut matches: libc::c_long = 0;
    let mut better: libc::c_long = 0;
    let mut worse: libc::c_long = 0;
    let mut other: libc::c_long = 0;
    let mut ch: libc::c_char = 0;
    let mut quality: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut good: bool_ = 0;
    let mut great: bool_ = 0;
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
    let mut q_ptr: *mut object_type = 0 as *mut object_type;
    let mut theme: obj_theme =
        obj_theme{treasure: 0, combat: 0, magic: 0, tools: 0,};
    let mut q: cptr =
        b"Rolls: %ld, Matches: %ld, Better: %ld, Worse: %ld, Other: %ld\x00"
            as *const u8 as *const libc::c_char;
    /* We can have everything */
    theme.treasure = 20 as libc::c_int as byte_hack;
    theme.combat = 20 as libc::c_int as byte_hack;
    theme.magic = 20 as libc::c_int as byte_hack;
    theme.tools = 20 as libc::c_int as byte_hack;
    /* XXX XXX XXX Mega-Hack -- allow multiple artifacts */
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
        if (*o_ptr).tval as libc::c_int == 102 as libc::c_int {
            random_artifacts[(*o_ptr).sval as usize].generated =
                0 as libc::c_int as byte_hack
        } else {
            (*a_info.offset((*o_ptr).name1 as isize)).cur_num =
                0 as libc::c_int as byte_hack
        }
    }
    loop 
         /* Interact */
         {
        let mut pmt: cptr =
            b"Roll for [n]ormal, [g]ood, or [e]xcellent treasure? \x00" as
                *const u8 as *const libc::c_char;
        /* Display item */
        wiz_display_item(o_ptr);
        /* Get choices */
        if get_com(pmt, &mut ch) == 0 { break ; }
        if ch as libc::c_int == 'n' as i32 || ch as libc::c_int == 'N' as i32
           {
            good = 0 as libc::c_int as bool_;
            great = 0 as libc::c_int as bool_;
            quality =
                b"normal\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        } else if ch as libc::c_int == 'g' as i32 ||
                      ch as libc::c_int == 'G' as i32 {
            good = 1 as libc::c_int as bool_;
            great = 0 as libc::c_int as bool_;
            quality =
                b"good\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        } else if ch as libc::c_int == 'e' as i32 ||
                      ch as libc::c_int == 'E' as i32 {
            good = 1 as libc::c_int as bool_;
            great = 1 as libc::c_int as bool_;
            quality =
                b"excellent\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        } else {
            good = 0 as libc::c_int as bool_;
            great = 0 as libc::c_int as bool_;
            break ;
        }
        /* Let us know what we are doing */
        msg_format(b"Creating a lot of %s items. Base level = %d.\x00" as
                       *const u8 as *const libc::c_char, quality,
                   dun_level as libc::c_int);
        msg_print(0 as cptr);
        /* Set counters to zero */
        other = 0 as libc::c_int as libc::c_long;
        worse = other;
        better = worse;
        matches = better;
        /* Let's rock and roll */
        i = 0 as libc::c_int as libc::c_long;
        while i <= 100000 as libc::c_int as libc::c_long {
            /* Output every few rolls */
            if i < 100 as libc::c_int as libc::c_long ||
                   i % 100 as libc::c_int as libc::c_long ==
                       0 as libc::c_int as libc::c_long {
                /* Do not wait */
                inkey_scan = 1 as libc::c_int as bool_;
                /* Allow interupt */
                if inkey() != 0 {
                    /* Flush */
                    flush();
                    break ;
                } else {
                    /* Dump the stats */
                    prt(format(q, i, matches, better, worse, other) as cptr,
                        0 as libc::c_int, 0 as libc::c_int);
                    Term_fresh();
                }
            }
            /* Get local object */
            q_ptr = &mut forge;
            /* Wipe the object */
            object_wipe(q_ptr);
            /* Create an object */
            make_object(q_ptr, good, great, theme);
            /* XXX XXX XXX Mega-Hack -- allow multiple artifacts */
            if (*q_ptr).tval as libc::c_int == 102 as libc::c_int ||
                   (if (*q_ptr).name1 as libc::c_int != 0 {
                        1 as libc::c_int
                    } else { 0 as libc::c_int }) != 0 ||
                   (if (*q_ptr).art_name as libc::c_int != 0 {
                        1 as libc::c_int
                    } else { 0 as libc::c_int }) != 0 ||
                   (if (*k_info.offset((*q_ptr).k_idx as isize)).flags3 as
                           libc::c_long & 0x8000 as libc::c_long != 0 {
                        1 as libc::c_int
                    } else { 0 as libc::c_int }) != 0 {
                if (*q_ptr).tval as libc::c_int == 102 as libc::c_int {
                    random_artifacts[(*q_ptr).sval as usize].generated =
                        0 as libc::c_int as byte_hack
                } else {
                    (*a_info.offset((*q_ptr).name1 as isize)).cur_num =
                        0 as libc::c_int as byte_hack
                }
            }
            /* Test for the same tval and sval. */
            if !((*o_ptr).tval as libc::c_int != (*q_ptr).tval as libc::c_int)
               {
                if !((*o_ptr).sval as libc::c_int !=
                         (*q_ptr).sval as libc::c_int) {
                    /* Check for match */
                    if (*q_ptr).pval == (*o_ptr).pval &&
                           (*q_ptr).to_a as libc::c_int ==
                               (*o_ptr).to_a as libc::c_int &&
                           (*q_ptr).to_h as libc::c_int ==
                               (*o_ptr).to_h as libc::c_int &&
                           (*q_ptr).to_d as libc::c_int ==
                               (*o_ptr).to_d as libc::c_int {
                        matches += 1
                    } else if (*q_ptr).pval >= (*o_ptr).pval &&
                                  (*q_ptr).to_a as libc::c_int >=
                                      (*o_ptr).to_a as libc::c_int &&
                                  (*q_ptr).to_h as libc::c_int >=
                                      (*o_ptr).to_h as libc::c_int &&
                                  (*q_ptr).to_d as libc::c_int >=
                                      (*o_ptr).to_d as libc::c_int {
                        better += 1
                    } else if (*q_ptr).pval <= (*o_ptr).pval &&
                                  (*q_ptr).to_a as libc::c_int <=
                                      (*o_ptr).to_a as libc::c_int &&
                                  (*q_ptr).to_h as libc::c_int <=
                                      (*o_ptr).to_h as libc::c_int &&
                                  (*q_ptr).to_d as libc::c_int <=
                                      (*o_ptr).to_d as libc::c_int {
                        worse += 1
                    } else {
                        /* Check for better */
                        /* Check for worse */
                        /* Assume different */
                        other += 1
                    }
                }
            }
            i += 1
        }
        /* Final dump */
        msg_format(q, i, matches, better, worse, other);
        msg_print(0 as cptr);
    }
    /* Hack -- Normally only make a single artifact */
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
        if (*o_ptr).tval as libc::c_int == 102 as libc::c_int {
            random_artifacts[(*o_ptr).sval as usize].generated =
                1 as libc::c_int as byte_hack
        } else {
            (*a_info.offset((*o_ptr).name1 as isize)).cur_num =
                1 as libc::c_int as byte_hack
        }
    };
}
/*
 * Change the quantity of a the item
 */
unsafe extern "C" fn wiz_quantity_item(mut o_ptr: *mut object_type) {
    let mut tmp_int: libc::c_int = 0;
    let mut tmp_val: [libc::c_char; 100] = [0; 100];
    /* Default */
    sprintf(tmp_val.as_mut_ptr(),
            b"%d\x00" as *const u8 as *const libc::c_char,
            (*o_ptr).number as libc::c_int);
    /* Query */
    if get_string(b"Quantity: \x00" as *const u8 as *const libc::c_char,
                  tmp_val.as_mut_ptr(), 2 as libc::c_int) != 0 {
        /* Extract */
        tmp_int = atoi(tmp_val.as_mut_ptr());
        /* Paranoia */
        if tmp_int < 1 as libc::c_int { tmp_int = 1 as libc::c_int }
        if tmp_int > 99 as libc::c_int { tmp_int = 99 as libc::c_int }
        /* Accept modifications */
        (*o_ptr).number = tmp_int as byte_hack
    };
}
/*
 * Play with an item. Options include:
 *   - Output statistics (via wiz_roll_item)
 *   - Reroll item (via wiz_reroll_item)
 *   - Change properties (via wiz_tweak_item)
 *   - Change the number of items (via wiz_quantity_item)
 */
unsafe extern "C" fn do_cmd_wiz_play() {
    let mut item: libc::c_int = 0;
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
    let mut q_ptr: *mut object_type = 0 as *mut object_type;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut ch: libc::c_char = 0;
    let mut changed: bool_ = 0;
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    /* Get an item */
    q = b"Play with which object? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have nothing to play with.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s,
                0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int)
           == 0 {
        return
    }
    /* Get the item */
    o_ptr = get_object(item);
    /* The item was not changed */
    changed = 0 as libc::c_int as bool_;
    /* Icky */
    character_icky = 1 as libc::c_int as bool_;
    /* Save the screen */
    Term_save();
    /* Get local object */
    q_ptr = &mut forge;
    /* Copy object */
    object_copy(q_ptr, o_ptr);
    loop 
         /* The main loop */
         /* Display the item */
         {
        wiz_display_item(q_ptr);
        /* Get choice */
        if get_com(b"[a]ccept [s]tatistics [r]eroll [t]weak [q]uantity apply[m]agic? \x00"
                       as *const u8 as *const libc::c_char, &mut ch) == 0 {
            changed = 0 as libc::c_int as bool_;
            break ;
        } else if ch as libc::c_int == 'A' as i32 ||
                      ch as libc::c_int == 'a' as i32 {
            changed = 1 as libc::c_int as bool_;
            break ;
        } else {
            if ch as libc::c_int == 's' as i32 ||
                   ch as libc::c_int == 'S' as i32 {
                wiz_statistics(q_ptr);
            }
            if ch as libc::c_int == 'r' as i32 ||
                   ch as libc::c_int == 'r' as i32 {
                wiz_reroll_item(q_ptr);
            }
            if ch as libc::c_int == 't' as i32 ||
                   ch as libc::c_int == 'T' as i32 {
                wiz_tweak_item(q_ptr);
            }
            if ch as libc::c_int == 'q' as i32 ||
                   ch as libc::c_int == 'Q' as i32 {
                wiz_quantity_item(q_ptr);
            }
            if ch as libc::c_int == 'm' as i32 ||
                   ch as libc::c_int == 'M' as i32 {
                let mut e: libc::c_int = (*q_ptr).name2 as libc::c_int;
                let mut eb: libc::c_int = (*q_ptr).name2b as libc::c_int;
                object_prep(q_ptr, (*q_ptr).k_idx as libc::c_int);
                (*q_ptr).name2 = e as s16b;
                (*q_ptr).name2b = eb as s16b;
                apply_magic(q_ptr, dun_level as libc::c_int,
                            0 as libc::c_int as bool_,
                            0 as libc::c_int as bool_,
                            0 as libc::c_int as bool_);
            }
        }
    }
    /* Restore the screen */
    Term_load();
    /* Not Icky */
    character_icky = 0 as libc::c_int as bool_;
    /* Accept change */
    if changed != 0 {
        /* Message */
        msg_print(b"Changes accepted.\x00" as *const u8 as
                      *const libc::c_char);
        /* Change */
        object_copy(o_ptr, q_ptr);
        /* Recalculate bonuses */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
        /* Combine / Reorder the pack (later) */
        (*p_ptr).notice =
            ((*p_ptr).notice as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long)) as u32b;
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long |
                 (0x1 as libc::c_long | 0x2 as libc::c_long |
                      0x8 as libc::c_long)) as u32b
    } else {
        /* Ignore change */
        msg_print(b"Changes ignored.\x00" as *const u8 as
                      *const libc::c_char);
    };
}
/*
 * Wizard routine for creating objects		-RAK-
 * Heavily modified to allow magification and artifactification  -Bernd-
 *
 * Note that wizards cannot create objects on top of other objects.
 *
 * Hack -- this routine always makes a "dungeon object", and applies
 * magic to it, and attempts to decline cursed items.
 */
unsafe extern "C" fn wiz_create_item() {
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
    let mut q_ptr: *mut object_type = 0 as *mut object_type;
    let mut k_idx: libc::c_int = 0;
    /* Icky */
    character_icky = 1 as libc::c_int as bool_;
    /* Save the screen */
    Term_save();
    /* Get object base type */
    k_idx = wiz_create_itemtype();
    /* Restore the screen */
    Term_load();
    /* Not Icky */
    character_icky = 0 as libc::c_int as bool_;
    /* Return if failed */
    if k_idx == 0 { return }
    /* Get local object */
    q_ptr = &mut forge;
    /* Create the item */
    object_prep(q_ptr, k_idx);
    /* Apply magic (no messages, no artifacts) */
    apply_magic(q_ptr, dun_level as libc::c_int, 0 as libc::c_int as bool_,
                0 as libc::c_int as bool_, 0 as libc::c_int as bool_);
    /* Drop the object from heaven */
    drop_near(q_ptr, -(1 as libc::c_int), (*p_ptr).py as libc::c_int,
              (*p_ptr).px as libc::c_int);
    /* All done */
    msg_print(b"Allocated.\x00" as *const u8 as *const libc::c_char);
}
/*
 * As above, but takes the k_idx as a parameter instead of using menus.
 */
unsafe extern "C" fn wiz_create_item_2() {
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
    let mut q_ptr: *mut object_type = 0 as *mut object_type;
    let mut a_idx: libc::c_int = 0;
    let mut p: cptr =
        b"Number of the object :\x00" as *const u8 as *const libc::c_char;
    let mut out_val: [libc::c_char; 80] =
        *::std::mem::transmute::<&[u8; 80],
                                 &mut [libc::c_char; 80]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    if get_string(p, out_val.as_mut_ptr(), 4 as libc::c_int) == 0 { return }
    a_idx = atoi(out_val.as_mut_ptr());
    /* Return if failed or out-of-bounds */
    if a_idx <= 0 as libc::c_int || a_idx >= max_k_idx as libc::c_int {
        return
    }
    /* Get local object */
    q_ptr = &mut forge;
    /* Create the item */
    object_prep(q_ptr, a_idx);
    /* Apply magic (no messages, no artifacts) */
    apply_magic(q_ptr, dun_level as libc::c_int, 0 as libc::c_int as bool_,
                0 as libc::c_int as bool_, 0 as libc::c_int as bool_);
    /* Drop the object from heaven */
    drop_near(q_ptr, -(1 as libc::c_int), (*p_ptr).py as libc::c_int,
              (*p_ptr).px as libc::c_int);
    /* All done */
    msg_print(b"Allocated.\x00" as *const u8 as *const libc::c_char);
}
/*
 * Cure everything instantly
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_wiz_cure_all() {
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    /* Remove curses */
    remove_all_curse();
    /* Restore stats */
    res_stat(0 as libc::c_int, 1 as libc::c_int as bool_);
    res_stat(1 as libc::c_int, 1 as libc::c_int as bool_);
    res_stat(2 as libc::c_int, 1 as libc::c_int as bool_);
    res_stat(4 as libc::c_int, 1 as libc::c_int as bool_);
    res_stat(3 as libc::c_int, 1 as libc::c_int as bool_);
    res_stat(5 as libc::c_int, 1 as libc::c_int as bool_);
    /* Restore the level */
    restore_level();
    /* Heal the player */
    (*p_ptr).chp = (*p_ptr).mhp;
    (*p_ptr).chp_frac = 0 as libc::c_int as u16b;
    /* Cure insanity of player */
    (*p_ptr).csane = (*p_ptr).msane;
    (*p_ptr).csane_frac = 0 as libc::c_int as u16b;
    /* Heal the player monster */
	/* Get the carried monster */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(49 as libc::c_int as
                                                         isize) as
            *mut object_type;
    if (*o_ptr).k_idx != 0 { (*o_ptr).pval2 = (*o_ptr).pval3 as s16b }
    /* Restore mana */
    (*p_ptr).csp = (*p_ptr).msp;
    (*p_ptr).csp_frac = 0 as libc::c_int as u16b;
    /* Cure stuff */
    set_blind(0 as libc::c_int);
    set_confused(0 as libc::c_int);
    set_poisoned(0 as libc::c_int);
    set_afraid(0 as libc::c_int);
    set_paralyzed(0 as libc::c_int);
    set_image(0 as libc::c_int);
    set_stun(0 as libc::c_int);
    set_cut(0 as libc::c_int);
    set_slow(0 as libc::c_int);
    (*p_ptr).black_breath = 0 as libc::c_int as bool_;
    /* No longer hungry */
    set_food(15000 as libc::c_int - 1 as libc::c_int);
    /* Redraw everything */
    do_cmd_redraw();
}
/*
 * Go to any level
 */
unsafe extern "C" fn do_cmd_wiz_jump() {
    /* Ask for level */
    if command_arg as libc::c_int <= 0 as libc::c_int {
        let mut ppp: [libc::c_char; 80] = [0; 80];
        let mut tmp_val: [libc::c_char; 160] = [0; 160];
        /* Prompt */
        msg_format(b"dungeon_type : %d\x00" as *const u8 as
                       *const libc::c_char, dungeon_type as libc::c_int);
        sprintf(ppp.as_mut_ptr(),
                b"Jump to level (0-%d): \x00" as *const u8 as
                    *const libc::c_char,
                (*d_info.offset(dungeon_type as isize)).maxdepth as
                    libc::c_int);
        /* Default */
        sprintf(tmp_val.as_mut_ptr(),
                b"%d\x00" as *const u8 as *const libc::c_char,
                dun_level as libc::c_int);
        /* Ask for a level */
        if get_string(ppp.as_mut_ptr() as cptr, tmp_val.as_mut_ptr(),
                      10 as libc::c_int) == 0 {
            return
        }
        /* Extract request */
        command_arg = atoi(tmp_val.as_mut_ptr()) as s16b
    }
    /* Paranoia */
    if (command_arg as libc::c_int) < 0 as libc::c_int {
        command_arg = 0 as libc::c_int as s16b
    }
    /* Paranoia */
    if command_arg as libc::c_int >
           (*d_info.offset(dungeon_type as isize)).maxdepth as libc::c_int {
        command_arg = (*d_info.offset(dungeon_type as isize)).maxdepth
    }
    /* Accept request */
    msg_format(b"You jump to dungeon level %d.\x00" as *const u8 as
                   *const libc::c_char, command_arg as libc::c_int);
    autosave_checkpoint();
    /* Change level */
    dun_level = command_arg;
    (*p_ptr).inside_arena = 0 as libc::c_int as s16b;
    leaving_quest = (*p_ptr).inside_quest as libc::c_int;
    (*p_ptr).inside_quest = 0 as libc::c_int as s16b;
    /* Leaving */
    (*p_ptr).leaving = 1 as libc::c_int as bool_;
}
/*
 * Become aware of a lot of objects
 */
unsafe extern "C" fn do_cmd_wiz_learn() {
    let mut i: libc::c_int = 0;
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
    let mut q_ptr: *mut object_type = 0 as *mut object_type;
    /* Scan every object */
    i = 1 as libc::c_int;
    while i < max_k_idx as libc::c_int {
        let mut k_ptr: *mut object_kind =
            &mut *k_info.offset(i as isize) as *mut object_kind;
        /* Induce awareness */
        if (*k_ptr).level as libc::c_int <= command_arg as libc::c_int {
            /* Get local object */
            q_ptr = &mut forge;
            /* Prepare object */
            object_prep(q_ptr, i);
            /* Awareness */
            object_aware(q_ptr);
        }
        i += 1
    };
}
/*
 * Summon some creatures
 */
unsafe extern "C" fn do_cmd_wiz_summon(mut num: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < num {
        summon_specific((*p_ptr).py as libc::c_int,
                        (*p_ptr).px as libc::c_int, dun_level as libc::c_int,
                        0 as libc::c_int);
        i += 1
    };
}
/*
 * Summon a creature of the specified type
 *
 * XXX XXX XXX This function is rather dangerous
 */
unsafe extern "C" fn do_cmd_wiz_named(mut r_idx: libc::c_int,
                                      mut slp: bool_) {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    /* Paranoia */
	/* if (!r_idx) return; */
    /* Prevent illegal monsters */
    if r_idx >= max_r_idx as libc::c_int { return }
    /* Try 10 times */
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        let mut d: libc::c_int = 1 as libc::c_int;
        /* Pick a location */
        scatter(&mut y, &mut x, (*p_ptr).py as libc::c_int,
                (*p_ptr).px as libc::c_int, d);
        /* Require empty grids */
        if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                               isize)).flags1 as libc::c_long &
               0x10 as libc::c_long != 0 &&
               (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
                   0xaf as libc::c_int &&
               (*cave[y as usize].offset(x as isize)).m_idx == 0 &&
               !(y == (*p_ptr).py as libc::c_int &&
                     x == (*p_ptr).px as libc::c_int) {
            /* Place it (allow groups) */
            *m_allow_special.offset(r_idx as isize) =
                1 as libc::c_int as bool_;
            if place_monster_aux(y, x, r_idx, slp, 1 as libc::c_int as bool_,
                                 -(2 as libc::c_int)) != 0 {
                break ;
            }
            *m_allow_special.offset(r_idx as isize) =
                0 as libc::c_int as bool_
        }
        i += 1
    };
}
/*
 * Summon a creature of the specified type
 *
 * XXX XXX XXX This function is rather dangerous
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_wiz_named_friendly(mut r_idx: libc::c_int,
                                                   mut slp: bool_) {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    /* Paranoia */
	/* if (!r_idx) return; */
    /* Prevent illegal monsters */
    if r_idx >= max_r_idx as libc::c_int { return }
    /* Try 10 times */
    *m_allow_special.offset(r_idx as isize) = 1 as libc::c_int as bool_;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        let mut d: libc::c_int = 1 as libc::c_int;
        /* Pick a location */
        scatter(&mut y, &mut x, (*p_ptr).py as libc::c_int,
                (*p_ptr).px as libc::c_int, d);
        /* Require empty grids */
        if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                               isize)).flags1 as libc::c_long &
               0x10 as libc::c_long != 0 &&
               (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
                   0xaf as libc::c_int &&
               (*cave[y as usize].offset(x as isize)).m_idx == 0 &&
               !(y == (*p_ptr).py as libc::c_int &&
                     x == (*p_ptr).px as libc::c_int) {
            /* Place it (allow groups) */
            if place_monster_aux(y, x, r_idx, slp, 1 as libc::c_int as bool_,
                                 3 as libc::c_int) != 0 {
                break ;
            }
        }
        i += 1
    }
    *m_allow_special.offset(r_idx as isize) = 0 as libc::c_int as bool_;
}
/*
 * Hack -- Delete all nearby monsters
 */
unsafe extern "C" fn do_cmd_wiz_zap() {
    let mut i: libc::c_int = 0;
    /* Genocide everyone nearby */
    i = 1 as libc::c_int;
    while i < m_max as libc::c_int {
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        /* Paranoia -- Skip dead monsters */
        if !((*m_ptr).r_idx == 0) {
            /* Delete nearby monsters */
            if (*m_ptr).cdis as libc::c_int <= 20 as libc::c_int {
                delete_monster_idx(i);
            }
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_cmd_wiz_body(mut bidx: s16b) 
 /* Might create problems with equipment slots. For safety,
	be nude when calling this function */
 {
    (*p_ptr).body_monster = bidx as u16b;
    (*p_ptr).disembodied = 0 as libc::c_int as bool_;
    (*p_ptr).chp =
        maxroll((*r_info.offset(bidx as isize)).hdice as s16b,
                (*r_info.offset(bidx as isize)).hside as s16b) as s16b;
    do_cmd_redraw();
}
/*
 * Hack -- declare external function
 */
/*
 * Ask for and parse a "debug command"
 * The "command_arg" may have been set.
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_debug() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut cmd: libc::c_char = 0;
    /* Get a "debug command" */
    get_com(b"Debug Command: \x00" as *const u8 as *const libc::c_char,
            &mut cmd);
    /* Analyze the command */
    match cmd as libc::c_int {
        27 | 32 | 10 | 13 => { }
        34 => {
            /* Hack -- Generate Spoilers */
            do_cmd_spoilers();
        }
        65 => { status_main(); }
        63 => {
            /* Hack -- Help */
            do_cmd_help();
        }
        97 => {
            /* Cure all maladies */
            do_cmd_wiz_cure_all();
        }
        98 => {
            /* Teleport to target */
            do_cmd_wiz_bamf();
        }
        66 => { do_cmd_wiz_body(command_arg); }
        45 => {
            /* Create any object */
            wiz_create_item_2();
        }
        99 => {
            /* Create any object */
            wiz_create_item();
        }
        67 => {
            /* Create a named artifact */
            wiz_create_named_art();
        }
        100 => {
            /* Detect everything */
            detect_all(25 as libc::c_int);
        }
        68 => {
            /* Change of Dungeon type */
            if command_arg as libc::c_int >= 0 as libc::c_int &&
                   (command_arg as libc::c_int) < max_d_idx as libc::c_int {
                dungeon_type = command_arg as byte_hack;
                dun_level = (*d_info.offset(dungeon_type as isize)).mindepth;
                msg_format(b"You go into %s\x00" as *const u8 as
                               *const libc::c_char,
                           d_text.offset((*d_info.offset(dungeon_type as
                                                             isize)).text as
                                             isize));
                /* Leaving */
                (*p_ptr).leaving = 1 as libc::c_int as bool_
            }
        }
        101 => {
            /* Edit character */
            do_cmd_wiz_change();
        }
        69 => {
            /* Change grid's mana */
            (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).mana =
                command_arg as byte_hack
        }
        102 => {
            /* View item info */
            identify_fully();
        }
        103 => {
            /* Good Objects */
            if command_arg as libc::c_int <= 0 as libc::c_int {
                command_arg = 1 as libc::c_int as s16b
            }
            acquirement((*p_ptr).py as libc::c_int,
                        (*p_ptr).px as libc::c_int,
                        command_arg as libc::c_int, 0 as libc::c_int as bool_,
                        1 as libc::c_int as bool_);
        }
        104 => {
            /* Hitpoint rerating */
            do_cmd_rerate();
        }
        72 => { do_cmd_summon_horde(); }
        105 => {
            /* MONSTER_HORDES */
            /* Identify */
            ident_spell();
        }
        106 => {
            /* Go up or down in the dungeon */
            do_cmd_wiz_jump();
        }
        107 => {
            /* Self-Knowledge */
            self_knowledge(0 as *mut FILE);
        }
        108 => {
            /* Learn about objects */
            do_cmd_wiz_learn();
        }
        109 => {
            /* Magic Mapping */
            map_area();
        }
        77 => {
            /* corruption */
            gain_random_corruption(command_arg as libc::c_int);
        }
        114 => {
            /* Specific reward */
            gain_level_reward(command_arg as libc::c_int);
        }
        82 => {
            /* Create a trap */
            wiz_place_trap((*p_ptr).py as libc::c_int,
                           (*p_ptr).px as libc::c_int,
                           command_arg as libc::c_int);
        }
        78 => {
            /* Summon _friendly_ named monster */
            do_cmd_wiz_named_friendly(command_arg as libc::c_int,
                                      1 as libc::c_int as bool_);
        }
        110 => {
            /* Summon Named Monster */
            do_cmd_wiz_named(command_arg as libc::c_int,
                             1 as libc::c_int as bool_);
        }
        111 => {
            /* Object playing routines */
            do_cmd_wiz_play();
        }
        112 => {
            /* Phase Door */
            teleport_player(10 as libc::c_int);
        }
        113 => {
            /*                        if (quest[command_arg].status == QUEST_STATUS_UNTAKEN)*/
            if command_arg as libc::c_int >= 1 as libc::c_int &&
                   (command_arg as libc::c_int) < 26 as libc::c_int &&
                   command_arg as libc::c_int != 5 as libc::c_int {
                (*quest.offset(command_arg as isize)).status =
                    1 as libc::c_int as s16b;
                *(*quest.offset(command_arg as isize)).plot = command_arg;
                if (*quest.offset(command_arg as isize)).type_0 as libc::c_int
                       == 0 as libc::c_int {
                    (*quest.offset(command_arg as
                                       isize)).init.expect("non-null function pointer")(command_arg
                                                                                            as
                                                                                            libc::c_int);
                }
            }
        }
        117 => {
            /* Make every dungeon square "known" to test streamers -KMW- */
            y = 0 as libc::c_int;
            while y < cur_hgt as libc::c_int {
                x = 0 as libc::c_int;
                while x < cur_wid as libc::c_int {
                    let ref mut fresh4 =
                        (*cave[y as usize].offset(x as isize)).info;
                    *fresh4 =
                        (*fresh4 as libc::c_int |
                             (0x2 as libc::c_int | 0x1 as libc::c_int)) as
                            u16b;
                    x += 1
                }
                y += 1
            }
            wiz_lite();
        }
        85 => {
            (*p_ptr).necro_extra |= 0x8 as libc::c_int as libc::c_uint;
            do_cmd_wiz_named(5 as libc::c_int, 1 as libc::c_int as bool_);
            (*p_ptr).necro_extra2 = 1 as libc::c_int as u32b;
            /* Display the hitpoints */
            (*p_ptr).update =
                ((*p_ptr).update as libc::c_long | 0x10 as libc::c_long) as
                    u32b;
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x40 as libc::c_long) as
                    u32b;
            /* Window stuff */
            (*p_ptr).window =
                ((*p_ptr).window as libc::c_long | 0x8 as libc::c_long) as
                    u32b
        }
        115 => {
            /* Summon Random Monster(s) */
            if command_arg as libc::c_int <= 0 as libc::c_int {
                command_arg = 1 as libc::c_int as s16b
            }
            do_cmd_wiz_summon(command_arg as libc::c_int);
        }
        83 => {
            /* Change the feature of the map */
            (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).special
                = command_arg
        }
        116 => {
            /* Teleport */
            teleport_player_bypass = 1 as libc::c_int as bool_;
            teleport_player(100 as libc::c_int);
            teleport_player_bypass = 0 as libc::c_int as bool_
        }
        84 => {
            /* Teleport to a town */
            if command_arg as libc::c_int >= 1 as libc::c_int &&
                   command_arg as libc::c_int <= max_real_towns as libc::c_int
               {
                teleport_player_town(command_arg as libc::c_int);
            }
        }
        118 => {
            /* Very Good Objects */
            if command_arg as libc::c_int <= 0 as libc::c_int {
                command_arg = 1 as libc::c_int as s16b
            }
            acquirement((*p_ptr).py as libc::c_int,
                        (*p_ptr).px as libc::c_int,
                        command_arg as libc::c_int, 1 as libc::c_int as bool_,
                        1 as libc::c_int as bool_);
        }
        119 => {
            /* Wizard Light the Level */
            wiz_lite();
        }
        87 => {
            /* Make a wish */
            make_wish();
        }
        120 => {
            /* Increase Experience */
            if command_arg != 0 {
                gain_exp(command_arg as s32b);
            } else { gain_exp((*p_ptr).exp + 1 as libc::c_int); }
        }
        122 => {
            /* Zap Monsters (Genocide) */
            do_cmd_wiz_zap();
        }
        95 => {
            /* Hack -- whatever I desire */
            do_cmd_wiz_hack_ben(command_arg as libc::c_int);
        }
        42 => {
            /* Mimic shape changing */
            (*p_ptr).tim_mimic = 100 as libc::c_int as s16b;
            (*p_ptr).mimic_form = command_arg as byte_hack;
            /* Redraw title */
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x2 as libc::c_long) as
                    u32b;
            /* Recalculate bonuses */
            (*p_ptr).update =
                ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as
                    u32b
        }
        43 => {
            /* Gain a fate */
            let mut i: libc::c_int = 0;
            gain_fate(command_arg as byte_hack);
            i = 0 as libc::c_int;
            while i < 200 as libc::c_int {
                fates[i as usize].know = 1 as libc::c_int as bool_;
                i += 1
            }
        }
        70 => {
            /* Change the feature of the map */
            msg_format(b"Trap: %d\x00" as *const u8 as *const libc::c_char,
                       (*cave[(*p_ptr).py as
                                  usize].offset((*p_ptr).px as isize)).t_idx
                           as libc::c_int);
            msg_format(b"Old feature: %d\x00" as *const u8 as
                           *const libc::c_char,
                       (*cave[(*p_ptr).py as
                                  usize].offset((*p_ptr).px as isize)).feat as
                           libc::c_int);
            msg_format(b"Special: %d\x00" as *const u8 as *const libc::c_char,
                       (*cave[(*p_ptr).py as
                                  usize].offset((*p_ptr).px as isize)).special
                           as libc::c_int);
            cave_set_feat((*p_ptr).py as libc::c_int,
                          (*p_ptr).px as libc::c_int,
                          command_arg as libc::c_int);
        }
        61 => { wiz_align_monster(command_arg as libc::c_int); }
        64 => { wiz_inc_monster_level(command_arg as libc::c_int); }
        47 => {
            summon_specific((*p_ptr).py as libc::c_int,
                            (*p_ptr).px as libc::c_int,
                            *max_dlv.offset(dungeon_type as isize) as
                                libc::c_int, command_arg as libc::c_int);
        }
        62 => { do_cmd_lua_script(); }
        _ => {
            /* Not a Wizard Command */
            if process_hooks(76 as libc::c_int,
                             b"(d)\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char, cmd as libc::c_int) == 0 {
                msg_print(b"That is not a valid debug command.\x00" as
                              *const u8 as *const libc::c_char);
            }
        }
    };
}
