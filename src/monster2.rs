use ::libc;
extern "C" {
    #[no_mangle]
    static mut ddx_ddd: [s16b; 9];
    #[no_mangle]
    static mut ddy_ddd: [s16b; 9];
    #[no_mangle]
    static mut extract_energy: [byte_hack; 300];
    #[no_mangle]
    static mut character_dungeon: bool_;
    #[no_mangle]
    static mut cur_hgt: s16b;
    #[no_mangle]
    static mut cur_wid: s16b;
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut num_repro: s16b;
    #[no_mangle]
    static mut object_level: s16b;
    #[no_mangle]
    static mut monster_level: s16b;
    #[no_mangle]
    static mut wizard: bool_;
    #[no_mangle]
    static mut coin_type: s16b;
    #[no_mangle]
    static mut shimmer_monsters: bool_;
    #[no_mangle]
    static mut repair_monsters: bool_;
    #[no_mangle]
    static mut m_max: s16b;
    #[no_mangle]
    static mut m_cnt: s16b;
    #[no_mangle]
    static mut hack_m_idx: s16b;
    #[no_mangle]
    static mut hack_m_idx_ii: s16b;
    #[no_mangle]
    static mut summon_kin_type: libc::c_char;
    #[no_mangle]
    static mut hack_mind: bool_;
    #[no_mangle]
    static mut disturb_near: bool_;
    #[no_mangle]
    static mut disturb_move: bool_;
    #[no_mangle]
    static mut disturb_pets: bool_;
    #[no_mangle]
    static mut smart_learn: bool_;
    #[no_mangle]
    static mut cheat_hear: bool_;
    #[no_mangle]
    static mut cheat_xtra: bool_;
    #[no_mangle]
    static mut rating: s16b;
    #[no_mangle]
    static mut panel_row_min: s16b;
    #[no_mangle]
    static mut panel_row_max: s16b;
    #[no_mangle]
    static mut panel_col_min: s16b;
    #[no_mangle]
    static mut panel_col_max: s16b;
    #[no_mangle]
    static mut target_who: s16b;
    #[no_mangle]
    static mut health_who: s16b;
    #[no_mangle]
    static mut monster_race_idx: s16b;
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut o_list: *mut object_type;
    #[no_mangle]
    static mut m_list: *mut monster_type;
    #[no_mangle]
    static mut alloc_kind_table_valid: bool_;
    #[no_mangle]
    static mut alloc_race_size: s16b;
    #[no_mangle]
    static mut alloc_race_table: *mut alloc_entry;
    #[no_mangle]
    static mut p_ptr: *mut player_type;
    #[no_mangle]
    static mut rp_ptr: *mut player_race;
    #[no_mangle]
    static mut rmp_ptr: *mut player_race_mod;
    #[no_mangle]
    static mut cp_ptr: *mut player_class;
    #[no_mangle]
    static mut spp_ptr: *mut player_spec;
    #[no_mangle]
    static mut f_info: *mut feature_type;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut a_info: *mut artifact_type;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    static mut r_name: *mut libc::c_char;
    #[no_mangle]
    static mut re_info: *mut monster_ego;
    #[no_mangle]
    static mut re_name: *mut libc::c_char;
    #[no_mangle]
    static mut d_info: *mut dungeon_info_type;
    #[no_mangle]
    static mut wf_info: *mut wilderness_type_info;
    #[no_mangle]
    static mut get_mon_num_hook:
           Option<unsafe extern "C" fn(_: libc::c_int) -> bool_>;
    #[no_mangle]
    static mut get_mon_num2_hook:
           Option<unsafe extern "C" fn(_: libc::c_int) -> bool_>;
    #[no_mangle]
    static mut get_obj_num_hook:
           Option<unsafe extern "C" fn(_: libc::c_int) -> bool_>;
    #[no_mangle]
    static mut wild_map: *mut *mut wilderness_map;
    #[no_mangle]
    static mut max_r_idx: u16b;
    #[no_mangle]
    static mut max_re_idx: u16b;
    #[no_mangle]
    static mut max_m_idx: u16b;
    #[no_mangle]
    static mut no_breeds: s16b;
    #[no_mangle]
    static mut random_artifacts: [random_artifact; 84];
    #[no_mangle]
    static mut dungeon_type: byte_hack;
    #[no_mangle]
    static mut doppleganger: s16b;
    #[no_mangle]
    static mut m_allow_special: *mut bool_;
    #[no_mangle]
    static mut joke_monsters: bool_;
    #[no_mangle]
    static mut dungeon_flags1: u32b;
    #[no_mangle]
    static mut dungeon_flags2: u32b;
    #[no_mangle]
    fn rnfree(p: vptr, len: huge_hack) -> vptr;
    #[no_mangle]
    fn vstrnfmt(buf: *mut libc::c_char, max: uint_hack, fmt: cptr,
                vp: ::std::ffi::VaList) -> uint_hack;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn damroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn maxroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut game_module: cptr;
    #[no_mangle]
    static mut DUNGEON_DEATH: s32b;
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn distance(y1: libc::c_int, x1: libc::c_int, y2: libc::c_int,
                x2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lite_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn scatter(yp: *mut libc::c_int, xp: *mut libc::c_int, y: libc::c_int,
               x: libc::c_int, d: libc::c_int);
    #[no_mangle]
    fn health_track(m_idx: libc::c_int);
    #[no_mangle]
    fn disturb(stop_search: libc::c_int, flush_output: libc::c_int);
    #[no_mangle]
    fn get_rnd_line(file_name: *mut libc::c_char, output: *mut libc::c_char)
     -> errr;
    #[no_mangle]
    fn cmsg_format(color: byte_hack, fmt: cptr, _: ...);
    #[no_mangle]
    fn is_a_vowel(ch: libc::c_int) -> bool_;
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn monster_msg(fmt: cptr, _: ...);
    #[no_mangle]
    fn handle_stuff();
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn lose_all_info() -> bool_;
    #[no_mangle]
    fn dec_stat(stat: libc::c_int, amount: libc::c_int, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn set_image(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn do_dec_stat(stat: libc::c_int, mode: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_paralyzed(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_confused(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn resolve_mimic_name(name: cptr) -> libc::c_int;
    #[no_mangle]
    fn is_friend(m_ptr: *mut monster_type) -> libc::c_int;
    #[no_mangle]
    fn delete_object_idx(o_idx: libc::c_int);
    #[no_mangle]
    fn o_pop() -> s16b;
    #[no_mangle]
    fn object_copy(o_ptr: *mut object_type, j_ptr: *mut object_type);
    #[no_mangle]
    fn test_mego_name(name: cptr) -> libc::c_int;
    #[no_mangle]
    fn get_dungeon_save(buf: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn make_object(j_ptr: *mut object_type, good: bool_, great: bool_,
                   theme: obj_theme) -> bool_;
    #[no_mangle]
    fn make_gold(j_ptr: *mut object_type) -> bool_;
    #[no_mangle]
    fn object_wipe(o_ptr: *mut object_type);
    #[no_mangle]
    fn create_artifact(o_ptr: *mut object_type, a_scroll: bool_,
                       get_name: bool_) -> bool_;
    #[no_mangle]
    fn object_prep(o_ptr: *mut object_type, k_idx: libc::c_int);
    #[no_mangle]
    fn kind_is_legal(k_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn get_obj_num(level: libc::c_int) -> s16b;
    #[no_mangle]
    fn get_obj_num_prep() -> errr;
    #[no_mangle]
    fn init_match_theme(theme: obj_theme);
    #[no_mangle]
    fn get_coin_type(r_ptr: *mut monster_race) -> libc::c_int;
    #[no_mangle]
    fn cmsg_print(color: byte_hack, msg: cptr);
    #[no_mangle]
    fn monster_can_cross_terrain(feat: byte_hack, r_ptr: *mut monster_race)
     -> bool_;
    #[no_mangle]
    fn set_mon_num2_hook(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn monster_dungeon(r_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn summon_lua_okay(r_idx: libc::c_int) -> bool_;
    #[no_mangle]
    fn verify_panel();
    #[no_mangle]
    fn drop_near(o_ptr: *mut object_type, chance: libc::c_int, y: libc::c_int,
                 x: libc::c_int) -> s16b;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type vptr = *mut libc::c_void;
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
pub struct alloc_entry {
    pub index: s16b,
    pub level: byte_hack,
    pub prob1: byte_hack,
    pub prob2: byte_hack,
    pub prob3: byte_hack,
    pub total: u16b,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_race {
    pub title: s32b,
    pub desc: s32b,
    pub r_adj: [s16b; 6],
    pub luck: libc::c_char,
    pub r_dis: s16b,
    pub r_dev: s16b,
    pub r_sav: s16b,
    pub r_stl: s16b,
    pub r_srh: s16b,
    pub r_fos: s16b,
    pub r_thn: s16b,
    pub r_thb: s16b,
    pub r_mhp: byte_hack,
    pub r_exp: u16b,
    pub b_age: byte_hack,
    pub m_age: byte_hack,
    pub m_b_ht: byte_hack,
    pub m_m_ht: byte_hack,
    pub m_b_wt: byte_hack,
    pub m_m_wt: byte_hack,
    pub f_b_ht: byte_hack,
    pub f_m_ht: byte_hack,
    pub f_b_wt: byte_hack,
    pub f_m_wt: byte_hack,
    pub infra: byte_hack,
    pub choice: [u32b; 2],
    pub powers: [s16b; 4],
    pub body_parts: [byte_hack; 6],
    pub chart: s16b,
    pub flags1: u32b,
    pub flags2: u32b,
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
    pub abilities: [C2RustUnnamed_1; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
    pub abilities: [C2RustUnnamed_2; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
/* File: monster2.c */
/* Purpose: misc code for monsters */
/*
 * Copyright (c) 1989 James E. Wilson, Robert A. Koeneke
 *
 * This software may be copied and distributed for educational, research, and
 * not for profit purposes provided that this copyright and statement are
 * included in all such copies.
 */
/* Monster gain a few levels ? */
#[no_mangle]
pub unsafe extern "C" fn monster_check_experience(mut m_idx: libc::c_int,
                                                  mut silent: bool_) {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*m_ptr).r_idx as isize) as *mut monster_race;
    let mut m_name: [libc::c_char; 80] = [0; 80];
    /* Get the name */
    monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
    /* Gain levels while possible */
    while ((*m_ptr).level as libc::c_int) < 150 as libc::c_int &&
              (*m_ptr).exp >=
                  ((if (*m_ptr).level as libc::c_int + 1 as libc::c_int >
                           150 as libc::c_int {
                        150 as libc::c_int
                    } else {
                        ((*m_ptr).level as libc::c_int) + 1 as libc::c_int
                    }) *
                       (if (*m_ptr).level as libc::c_int + 1 as libc::c_int >
                               150 as libc::c_int {
                            150 as libc::c_int
                        } else {
                            ((*m_ptr).level as libc::c_int) + 1 as libc::c_int
                        }) *
                       (if (*m_ptr).level as libc::c_int + 1 as libc::c_int >
                               150 as libc::c_int {
                            150 as libc::c_int
                        } else {
                            ((*m_ptr).level as libc::c_int) + 1 as libc::c_int
                        }) * 6 as libc::c_int) as u32b {
        /* Gain a level */
        (*m_ptr).level = (*m_ptr).level.wrapping_add(1);
        if (*m_ptr).ml as libc::c_int != 0 && silent == 0 {
            cmsg_format(14 as libc::c_int as byte_hack,
                        b"%^s gains a level.\x00" as *const u8 as
                            *const libc::c_char, m_name.as_mut_ptr());
        }
        /* Gain hp */
        if Rand_div(100 as libc::c_int) < 80 as libc::c_int {
            (*m_ptr).maxhp += (*r_ptr).hside as libc::c_int;
            (*m_ptr).hp += (*r_ptr).hside as libc::c_int
        }
        /* Gain speed */
        if Rand_div(100 as libc::c_int) < 40 as libc::c_int {
            let mut speed: libc::c_int =
                Rand_div(2 as libc::c_int) + 1 as libc::c_int;
            (*m_ptr).speed =
                ((*m_ptr).speed as libc::c_int + speed) as byte_hack;
            (*m_ptr).mspeed =
                ((*m_ptr).mspeed as libc::c_int + speed) as byte_hack
        }
        /* Gain ac */
        if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
            (*m_ptr).ac =
                ((*m_ptr).ac as libc::c_int +
                     if (*r_ptr).ac as libc::c_int / 10 as libc::c_int != 0 {
                         ((*r_ptr).ac as libc::c_int) / 10 as libc::c_int
                     } else { 1 as libc::c_int }) as s16b
        }
        /* Gain melee power */
        if Rand_div(100 as libc::c_int) < 30 as libc::c_int {
            let mut i: libc::c_int = Rand_div(3 as libc::c_int);
            let mut tries: libc::c_int = 20 as libc::c_int;
            loop  {
                let fresh0 = tries;
                tries = tries - 1;
                if !(fresh0 != 0 && (*m_ptr).blow[i as usize].d_dice == 0) {
                    break ;
                }
                i = Rand_div(3 as libc::c_int)
            }
            (*m_ptr).blow[i as usize].d_dice =
                (*m_ptr).blow[i as usize].d_dice.wrapping_add(1)
        }
    };
}
/* Monster gain some xp */
#[no_mangle]
pub unsafe extern "C" fn monster_gain_exp(mut m_idx: libc::c_int,
                                          mut exp: u32b, mut silent: bool_) {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    (*m_ptr).exp =
        ((*m_ptr).exp as libc::c_uint).wrapping_add(exp) as u32b as u32b;
    if wizard != 0 {
        let mut m_name: [libc::c_char; 80] = [0; 80];
        /* Get the name */
        monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
        if silent == 0 {
            msg_format(b"%^s gains %ld exp.\x00" as *const u8 as
                           *const libc::c_char, m_name.as_mut_ptr(), exp);
        }
    }
    monster_check_experience(m_idx, silent);
}
#[no_mangle]
pub unsafe extern "C" fn monster_set_level(mut m_idx: libc::c_int,
                                           mut level: libc::c_int) {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    if level > 150 as libc::c_int { level = 150 as libc::c_int }
    if ((*m_ptr).level as libc::c_int) < level {
        (*m_ptr).exp =
            ((if level > 150 as libc::c_int {
                  150 as libc::c_int
              } else { level }) *
                 (if level > 150 as libc::c_int {
                      150 as libc::c_int
                  } else { level }) *
                 (if level > 150 as libc::c_int {
                      150 as libc::c_int
                  } else { level }) * 6 as libc::c_int) as u32b;
        monster_check_experience(m_idx, 1 as libc::c_int as bool_);
    };
}
/* Will add, sub, .. */
#[no_mangle]
pub unsafe extern "C" fn modify_aux(mut a: s32b, mut b: s32b,
                                    mut mod_0: libc::c_char) -> s32b {
    match mod_0 as libc::c_int {
        0 => { return a + b }
        1 => { return a - b }
        2 => { return b }
        3 => { return a * b / 100 as libc::c_int }
        _ => {
            msg_format(b"WARNING, unmatching MEGO(%d).\x00" as *const u8 as
                           *const libc::c_char, mod_0 as libc::c_int);
            return 0 as libc::c_int
        }
    };
}
/* Is this ego ok for this monster ? */
#[no_mangle]
pub unsafe extern "C" fn mego_ok(mut r_idx: libc::c_int, mut ego: libc::c_int)
 -> bool_ {
    let mut re_ptr: *mut monster_ego =
        &mut *re_info.offset(ego as isize) as *mut monster_ego;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    let mut ok: bool_ = 0 as libc::c_int as bool_;
    let mut i: libc::c_int = 0;
    /* needed flags */
    if (*re_ptr).flags1 != 0 &&
           (*re_ptr).flags1 & (*r_ptr).flags1 != (*re_ptr).flags1 {
        return 0 as libc::c_int as bool_
    }
    if (*re_ptr).flags2 != 0 &&
           (*re_ptr).flags2 & (*r_ptr).flags2 != (*re_ptr).flags2 {
        return 0 as libc::c_int as bool_
    }
    if (*re_ptr).flags3 != 0 &&
           (*re_ptr).flags3 & (*r_ptr).flags3 != (*re_ptr).flags3 {
        return 0 as libc::c_int as bool_
    }
    if (*re_ptr).flags7 != 0 &&
           (*re_ptr).flags7 & (*r_ptr).flags7 != (*re_ptr).flags7 {
        return 0 as libc::c_int as bool_
    }
    if (*re_ptr).flags8 != 0 &&
           (*re_ptr).flags8 & (*r_ptr).flags8 != (*re_ptr).flags8 {
        return 0 as libc::c_int as bool_
    }
    if (*re_ptr).flags9 != 0 &&
           (*re_ptr).flags9 & (*r_ptr).flags9 != (*re_ptr).flags9 {
        return 0 as libc::c_int as bool_
    }
    /* unwanted flags */
    if (*re_ptr).hflags1 != 0 && (*re_ptr).hflags1 & (*r_ptr).flags1 != 0 {
        return 0 as libc::c_int as bool_
    }
    if (*re_ptr).hflags2 != 0 && (*re_ptr).hflags2 & (*r_ptr).flags2 != 0 {
        return 0 as libc::c_int as bool_
    }
    if (*re_ptr).hflags3 != 0 && (*re_ptr).hflags3 & (*r_ptr).flags3 != 0 {
        return 0 as libc::c_int as bool_
    }
    if (*re_ptr).hflags7 != 0 && (*re_ptr).hflags7 & (*r_ptr).flags7 != 0 {
        return 0 as libc::c_int as bool_
    }
    if (*re_ptr).hflags8 != 0 && (*re_ptr).hflags8 & (*r_ptr).flags8 != 0 {
        return 0 as libc::c_int as bool_
    }
    if (*re_ptr).hflags9 != 0 && (*re_ptr).hflags9 & (*r_ptr).flags9 != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Need good race -- IF races are specified */
    if (*re_ptr).r_char[0 as libc::c_int as usize] != 0 {
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            if (*r_ptr).d_char as libc::c_int ==
                   (*re_ptr).r_char[i as usize] as libc::c_int {
                ok = 1 as libc::c_int as bool_
            }
            i += 1
        }
        if ok == 0 { return 0 as libc::c_int as bool_ }
    }
    if (*re_ptr).nr_char[0 as libc::c_int as usize] != 0 {
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            if (*r_ptr).d_char as libc::c_int ==
                   (*re_ptr).nr_char[i as usize] as libc::c_int {
                return 0 as libc::c_int as bool_
            }
            i += 1
        }
    }
    /* Passed all tests ? */
    return 1 as libc::c_int as bool_;
}
/* Choose an ego type */
#[no_mangle]
pub unsafe extern "C" fn pick_ego_monster(mut r_idx: libc::c_int)
 -> libc::c_int {
    /* Assume no ego */
    let mut ego: libc::c_int = 0 as libc::c_int;
    let mut lvl: libc::c_int = 0;
    let mut tries: libc::c_int =
        max_re_idx as libc::c_int + 10 as libc::c_int;
    let mut re_ptr: *mut monster_ego = 0 as *mut monster_ego;
    if dungeon_flags2 as libc::c_long & 0x1000 as libc::c_long == 0 &&
           dungeon_flags2 as libc::c_long & 0x2000 as libc::c_long == 0 {
        /* No townspeople ego */
        if (*r_info.offset(r_idx as isize)).level == 0 {
            return 0 as libc::c_int
        }
        /* First are we allowed to find an ego */
        if !(Rand_div(100 as libc::c_int) < 18 as libc::c_int) {
            return 0 as libc::c_int
        }
        loop 
             /* Lets look for one */
             {
            let fresh1 = tries;
            tries = tries - 1;
            if !(fresh1 != 0) { break ; }
            /* Pick one */
            ego =
                1 as libc::c_int +
                    Rand_div(1 as libc::c_int +
                                 (max_re_idx as libc::c_int -
                                      1 as libc::c_int) - 1 as libc::c_int);
            re_ptr = &mut *re_info.offset(ego as isize) as *mut monster_ego;
            /*  No hope so far */
            if mego_ok(r_idx, ego) == 0 { continue ; }
            /* Not too much OoD */
            lvl = (*r_info.offset(r_idx as isize)).level as libc::c_int;
            lvl =
                modify_aux(lvl,
                           (*re_ptr).level as libc::c_int >> 2 as libc::c_int,
                           ((*re_ptr).level as libc::c_int & 3 as libc::c_int)
                               as libc::c_char);
            lvl = if lvl < 0 as libc::c_int { 0 as libc::c_int } else { lvl };
            lvl -=
                dun_level as libc::c_int / 2 as libc::c_int +
                    Rand_div(dun_level as libc::c_int / 2 as libc::c_int);
            if lvl < 1 as libc::c_int { lvl = 1 as libc::c_int }
            if Rand_div(lvl) != 0 { continue ; }
            /* Each ego types have a rarity */
            if Rand_div((*re_ptr).rarity as s32b) != 0 { continue ; }
            /* We finally got one ? GREAT */
            return ego
        }
    } else {
        /* Bypass restrictions for themed townspeople */
        if dungeon_flags2 as libc::c_long & 0x1000 as libc::c_long != 0 {
            ego =
                test_mego_name(b"Elven\x00" as *const u8 as
                                   *const libc::c_char)
        } else if dungeon_flags2 as libc::c_long & 0x2000 as libc::c_long != 0
         {
            ego =
                test_mego_name(b"Dwarven\x00" as *const u8 as
                                   *const libc::c_char)
        }
        if mego_ok(r_idx, ego) != 0 { return ego }
    }
    /* Found none ? so sad, well no ego for the time being */
    return 0 as libc::c_int;
}
/*
 * Return a (monster_race*) with the combination of the monster
 * properties and the ego type
 */
#[no_mangle]
pub unsafe extern "C" fn race_info_idx(mut r_idx: libc::c_int,
                                       mut ego: libc::c_int)
 -> *mut monster_race {
    static mut race: monster_race =
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
    let mut re_ptr: *mut monster_ego =
        &mut *re_info.offset(ego as isize) as *mut monster_ego;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    let mut nr_ptr: *mut monster_race = &mut race;
    let mut i: libc::c_int = 0;
    /* No work needed */
    if ego == 0 { return r_ptr }
    /* Copy the base monster */
    memcpy(nr_ptr as *mut libc::c_char as *mut libc::c_void,
           r_ptr as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<monster_race>() as libc::c_ulong);
    /* Adjust the values */
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut j: s32b = 0;
        let mut k: s32b = 0;
        j =
            modify_aux((*nr_ptr).blow[i as usize].d_dice as s32b,
                       (*re_ptr).blow[i as usize].d_dice as s32b,
                       (*re_ptr).blowm[i as usize][0 as libc::c_int as usize]
                           as libc::c_char);
        if j < 0 as libc::c_int { j = 0 as libc::c_int }
        k =
            modify_aux((*nr_ptr).blow[i as usize].d_side as s32b,
                       (*re_ptr).blow[i as usize].d_side as s32b,
                       (*re_ptr).blowm[i as usize][1 as libc::c_int as usize]
                           as libc::c_char);
        if k < 0 as libc::c_int { k = 0 as libc::c_int }
        (*nr_ptr).blow[i as usize].d_dice = j as byte_hack;
        (*nr_ptr).blow[i as usize].d_side = k as byte_hack;
        if (*re_ptr).blow[i as usize].method != 0 {
            (*nr_ptr).blow[i as usize].method =
                (*re_ptr).blow[i as usize].method
        }
        if (*re_ptr).blow[i as usize].effect != 0 {
            (*nr_ptr).blow[i as usize].effect =
                (*re_ptr).blow[i as usize].effect
        }
        i += 1
    }
    (*nr_ptr).hdice =
        modify_aux((*nr_ptr).hdice as s32b,
                   (*re_ptr).hdice as libc::c_int >> 2 as libc::c_int,
                   ((*re_ptr).hdice as libc::c_int & 3 as libc::c_int) as
                       libc::c_char) as u16b;
    (*nr_ptr).hdice =
        if ((*nr_ptr).hdice as libc::c_int) < 1 as libc::c_int {
            1 as libc::c_int
        } else { (*nr_ptr).hdice as libc::c_int } as u16b;
    (*nr_ptr).hside =
        modify_aux((*nr_ptr).hside as s32b,
                   (*re_ptr).hside as libc::c_int >> 2 as libc::c_int,
                   ((*re_ptr).hside as libc::c_int & 3 as libc::c_int) as
                       libc::c_char) as u16b;
    (*nr_ptr).hside =
        if ((*nr_ptr).hside as libc::c_int) < 1 as libc::c_int {
            1 as libc::c_int
        } else { (*nr_ptr).hside as libc::c_int } as u16b;
    (*nr_ptr).ac =
        modify_aux((*nr_ptr).ac as s32b,
                   (*re_ptr).ac as libc::c_int >> 2 as libc::c_int,
                   ((*re_ptr).ac as libc::c_int & 3 as libc::c_int) as
                       libc::c_char) as s16b;
    (*nr_ptr).ac =
        if ((*nr_ptr).ac as libc::c_int) < 0 as libc::c_int {
            0 as libc::c_int
        } else { (*nr_ptr).ac as libc::c_int } as s16b;
    (*nr_ptr).sleep =
        modify_aux((*nr_ptr).sleep as s32b,
                   (*re_ptr).sleep as libc::c_int >> 2 as libc::c_int,
                   ((*re_ptr).sleep as libc::c_int & 3 as libc::c_int) as
                       libc::c_char) as s16b;
    (*nr_ptr).sleep =
        if ((*nr_ptr).sleep as libc::c_int) < 0 as libc::c_int {
            0 as libc::c_int
        } else { (*nr_ptr).sleep as libc::c_int } as s16b;
    (*nr_ptr).aaf =
        modify_aux((*nr_ptr).aaf as s32b,
                   (*re_ptr).aaf as libc::c_int >> 2 as libc::c_int,
                   ((*re_ptr).aaf as libc::c_int & 3 as libc::c_int) as
                       libc::c_char) as byte_hack;
    (*nr_ptr).aaf =
        if ((*nr_ptr).aaf as libc::c_int) < 1 as libc::c_int {
            1 as libc::c_int
        } else { (*nr_ptr).aaf as libc::c_int } as byte_hack;
    (*nr_ptr).speed =
        modify_aux((*nr_ptr).speed as s32b,
                   (*re_ptr).speed as libc::c_int >> 2 as libc::c_int,
                   ((*re_ptr).speed as libc::c_int & 3 as libc::c_int) as
                       libc::c_char) as byte_hack;
    (*nr_ptr).speed =
        if ((*nr_ptr).speed as libc::c_int) < 50 as libc::c_int {
            50 as libc::c_int
        } else { (*nr_ptr).speed as libc::c_int } as byte_hack;
    (*nr_ptr).mexp =
        modify_aux((*nr_ptr).mexp, (*re_ptr).mexp >> 2 as libc::c_int,
                   ((*re_ptr).mexp & 3 as libc::c_int) as libc::c_char);
    (*nr_ptr).mexp =
        if (*nr_ptr).mexp < 0 as libc::c_int {
            0 as libc::c_int
        } else { (*nr_ptr).mexp };
    (*nr_ptr).weight =
        modify_aux((*nr_ptr).weight, (*re_ptr).weight >> 2 as libc::c_int,
                   ((*re_ptr).weight & 3 as libc::c_int) as libc::c_char);
    (*nr_ptr).weight =
        if (*nr_ptr).weight < 10 as libc::c_int {
            10 as libc::c_int
        } else { (*nr_ptr).weight };
    (*nr_ptr).freq_inate =
        if (*nr_ptr).freq_inate as libc::c_int >
               (*re_ptr).freq_inate as libc::c_int {
            (*nr_ptr).freq_inate as libc::c_int
        } else { (*re_ptr).freq_inate as libc::c_int } as byte_hack;
    (*nr_ptr).freq_spell =
        if (*nr_ptr).freq_spell as libc::c_int >
               (*re_ptr).freq_spell as libc::c_int {
            (*nr_ptr).freq_spell as libc::c_int
        } else { (*re_ptr).freq_spell as libc::c_int } as byte_hack;
    (*nr_ptr).level =
        modify_aux((*nr_ptr).level as s32b,
                   (*re_ptr).level as libc::c_int >> 2 as libc::c_int,
                   ((*re_ptr).level as libc::c_int & 3 as libc::c_int) as
                       libc::c_char) as byte_hack;
    (*nr_ptr).level =
        if ((*nr_ptr).level as libc::c_int) < 1 as libc::c_int {
            1 as libc::c_int
        } else { (*nr_ptr).level as libc::c_int } as byte_hack;
    /* Take off some flags */
    (*nr_ptr).flags1 &= !(*re_ptr).nflags1;
    (*nr_ptr).flags2 &= !(*re_ptr).nflags2;
    (*nr_ptr).flags3 &= !(*re_ptr).nflags3;
    (*nr_ptr).flags4 &= !(*re_ptr).nflags4;
    (*nr_ptr).flags5 &= !(*re_ptr).nflags5;
    (*nr_ptr).flags6 &= !(*re_ptr).nflags6;
    (*nr_ptr).flags7 &= !(*re_ptr).nflags7;
    (*nr_ptr).flags8 &= !(*re_ptr).nflags8;
    (*nr_ptr).flags9 &= !(*re_ptr).nflags9;
    /* Add some flags */
    (*nr_ptr).flags1 |= (*re_ptr).mflags1;
    (*nr_ptr).flags2 |= (*re_ptr).mflags2;
    (*nr_ptr).flags3 |= (*re_ptr).mflags3;
    (*nr_ptr).flags4 |= (*re_ptr).mflags4;
    (*nr_ptr).flags5 |= (*re_ptr).mflags5;
    (*nr_ptr).flags6 |= (*re_ptr).mflags6;
    (*nr_ptr).flags7 |= (*re_ptr).mflags7;
    (*nr_ptr).flags8 |= (*re_ptr).mflags8;
    (*nr_ptr).flags9 |= (*re_ptr).mflags9;
    /* Change the char/attr is needed */
    if (*re_ptr).d_char as libc::c_int != 127 as libc::c_int {
        (*nr_ptr).d_char = (*re_ptr).d_char;
        (*nr_ptr).x_char = (*re_ptr).d_char
    }
    if (*re_ptr).d_attr as libc::c_int != 127 as libc::c_int {
        (*nr_ptr).d_attr = (*re_ptr).d_attr;
        (*nr_ptr).x_attr = (*re_ptr).d_attr
    }
    /* And finanly return a pointer to a fully working monster race */
    return nr_ptr;
}
static mut horror_desc: [cptr; 20] =
    [b"abominable\x00" as *const u8 as *const libc::c_char,
     b"abysmal\x00" as *const u8 as *const libc::c_char,
     b"appalling\x00" as *const u8 as *const libc::c_char,
     b"baleful\x00" as *const u8 as *const libc::c_char,
     b"blasphemous\x00" as *const u8 as *const libc::c_char,
     b"disgusting\x00" as *const u8 as *const libc::c_char,
     b"dreadful\x00" as *const u8 as *const libc::c_char,
     b"filthy\x00" as *const u8 as *const libc::c_char,
     b"grisly\x00" as *const u8 as *const libc::c_char,
     b"hideous\x00" as *const u8 as *const libc::c_char,
     b"hellish\x00" as *const u8 as *const libc::c_char,
     b"horrible\x00" as *const u8 as *const libc::c_char,
     b"infernal\x00" as *const u8 as *const libc::c_char,
     b"loathsome\x00" as *const u8 as *const libc::c_char,
     b"nightmarish\x00" as *const u8 as *const libc::c_char,
     b"repulsive\x00" as *const u8 as *const libc::c_char,
     b"sacrilegious\x00" as *const u8 as *const libc::c_char,
     b"terrible\x00" as *const u8 as *const libc::c_char,
     b"unclean\x00" as *const u8 as *const libc::c_char,
     b"unspeakable\x00" as *const u8 as *const libc::c_char];
static mut funny_desc: [cptr; 22] =
    [b"silly\x00" as *const u8 as *const libc::c_char,
     b"hilarious\x00" as *const u8 as *const libc::c_char,
     b"absurd\x00" as *const u8 as *const libc::c_char,
     b"insipid\x00" as *const u8 as *const libc::c_char,
     b"ridiculous\x00" as *const u8 as *const libc::c_char,
     b"laughable\x00" as *const u8 as *const libc::c_char,
     b"ludicrous\x00" as *const u8 as *const libc::c_char,
     b"far-out\x00" as *const u8 as *const libc::c_char,
     b"groovy\x00" as *const u8 as *const libc::c_char,
     b"postmodern\x00" as *const u8 as *const libc::c_char,
     b"fantastic\x00" as *const u8 as *const libc::c_char,
     b"dadaistic\x00" as *const u8 as *const libc::c_char,
     b"cubistic\x00" as *const u8 as *const libc::c_char,
     b"cosmic\x00" as *const u8 as *const libc::c_char,
     b"awesome\x00" as *const u8 as *const libc::c_char,
     b"incomprehensible\x00" as *const u8 as *const libc::c_char,
     b"fabulous\x00" as *const u8 as *const libc::c_char,
     b"amazing\x00" as *const u8 as *const libc::c_char,
     b"incredible\x00" as *const u8 as *const libc::c_char,
     b"chaotic\x00" as *const u8 as *const libc::c_char,
     b"wild\x00" as *const u8 as *const libc::c_char,
     b"preposterous\x00" as *const u8 as *const libc::c_char];
static mut funny_comments: [cptr; 5] =
    [b"Wow, cosmic, man!\x00" as *const u8 as *const libc::c_char,
     b"Rad!\x00" as *const u8 as *const libc::c_char,
     b"Groovy!\x00" as *const u8 as *const libc::c_char,
     b"Cool!\x00" as *const u8 as *const libc::c_char,
     b"Far out!\x00" as *const u8 as *const libc::c_char];
#[no_mangle]
pub unsafe extern "C" fn get_wilderness_flag() -> libc::c_int {
    let mut x: libc::c_int = (*p_ptr).wilderness_x;
    let mut y: libc::c_int = (*p_ptr).wilderness_y;
    if dun_level != 0 {
        return 0x1 as libc::c_int
    } else {
        return ((1 as libc::c_long) <<
                    (*wf_info.offset((*(*wild_map.offset(y as
                                                             isize)).offset(x
                                                                                as
                                                                                isize)).feat
                                         as isize)).terrain_idx as
                        libc::c_int) as libc::c_int
    };
}
/*
 * Delete a monster by index.
 *
 * When a monster is deleted, all of its objects are deleted.
 */
#[no_mangle]
pub unsafe extern "C" fn delete_monster_idx(mut i: libc::c_int) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(i as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    let mut this_o_idx: s16b = 0;
    let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
    let mut had_lite: bool_ = 0 as libc::c_int as bool_;
    /* Get location */
    y = (*m_ptr).fy as libc::c_int;
    x = (*m_ptr).fx as libc::c_int;
    /* Hack -- Reduce the racial counter */
    (*r_ptr).cur_num = (*r_ptr).cur_num.wrapping_sub(1);
    (*r_ptr).on_saved = 0 as libc::c_int as bool_;
    /* Hack -- count the number of "reproducers" */
    if (*r_ptr).flags4 & 0x2 as libc::c_int as libc::c_uint != 0 {
        num_repro -= 1
    }
    /* XXX XXX XXX remove monster light source */
    if (*r_ptr).flags9 & 0x4 as libc::c_int as libc::c_uint != 0 {
        had_lite = 1 as libc::c_int as bool_
    }
    /* Hack -- remove target monster */
    if i == target_who as libc::c_int {
        target_who = 0 as libc::c_int as s16b
    }
    /* Hack -- remove tracked monster */
    if i == health_who as libc::c_int { health_track(0 as libc::c_int); }
    /* Hack -- remove tracked monster */
    if i == (*p_ptr).control as libc::c_int {
        (*p_ptr).control = 0 as libc::c_int as s16b
    }
    j = m_max as libc::c_int - 1 as libc::c_int;
    while j >= 1 as libc::c_int {
        /* Access the monster */
        let mut t_ptr: *mut monster_type =
            &mut *m_list.offset(j as isize) as *mut monster_type;
        /* Ignore "dead" monsters */
        if !((*t_ptr).r_idx == 0) {
            if (*t_ptr).target as libc::c_int == i {
                (*t_ptr).target = -(1 as libc::c_int) as s16b
            }
        }
        j -= 1
    }
    /* Monster is gone */
    (*cave[y as usize].offset(x as isize)).m_idx = 0 as libc::c_int as s16b;
    /* Delete objects */
    this_o_idx = (*m_ptr).hold_o_idx;
    while this_o_idx != 0 {
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
        /* Acquire object */
        o_ptr = &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
        /* Acquire next object */
        next_o_idx = (*o_ptr).next_o_idx;
        /* Hack -- efficiency */
        (*o_ptr).held_m_idx = 0 as libc::c_int as s16b;
        if (*p_ptr).preserve != 0 {
            /* Hack -- Preserve unknown artifacts */
            if ((*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
                    (if (*o_ptr).name1 as libc::c_int != 0 {
                         1 as libc::c_int
                     } else { 0 as libc::c_int }) != 0 ||
                    (if (*o_ptr).art_name as libc::c_int != 0 {
                         1 as libc::c_int
                     } else { 0 as libc::c_int }) != 0 ||
                    (if (*k_info.offset((*o_ptr).k_idx as isize)).flags3 as
                            libc::c_long & 0x8000 as libc::c_long != 0 {
                         1 as libc::c_int
                     } else { 0 as libc::c_int }) != 0) &&
                   !((*o_ptr).ident as libc::c_int & 0x8 as libc::c_int != 0
                         ||
                         (*k_info.offset((*o_ptr).k_idx as isize)).easy_know
                             as libc::c_int != 0 &&
                             (*k_info.offset((*o_ptr).k_idx as isize)).aware
                                 as libc::c_int != 0) {
                /* Mega-Hack -- Preserve the artifact */
                if (*o_ptr).tval as libc::c_int == 102 as libc::c_int {
                    random_artifacts[(*o_ptr).sval as usize].generated =
                        0 as libc::c_int as byte_hack
                } else if (*k_info.offset((*o_ptr).k_idx as isize)).flags3 as
                              libc::c_long & 0x8000 as libc::c_long != 0 {
                    (*k_info.offset((*o_ptr).k_idx as isize)).artifact =
                        0 as libc::c_int as bool_
                } else {
                    (*a_info.offset((*o_ptr).name1 as isize)).cur_num =
                        0 as libc::c_int as byte_hack
                }
            }
        }
        /* Delete the object */
        delete_object_idx(this_o_idx as libc::c_int);
        this_o_idx = next_o_idx
    }
    /* Delete mind & special race if needed */
    if !(*m_ptr).sr_ptr.is_null() {
        (*m_ptr).sr_ptr =
            rnfree((*m_ptr).sr_ptr as vptr,
                   ::std::mem::size_of::<monster_race>() as libc::c_ulong) as
                *mut monster_race
    }
    if !(*m_ptr).mind.is_null() {
        (*m_ptr).mind =
            rnfree((*m_ptr).mind as vptr,
                   ::std::mem::size_of::<monster_mind>() as libc::c_ulong) as
                *mut monster_mind
    }
    /* Wipe the Monster */
    m_ptr =
        memset(m_ptr as *mut libc::c_char as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<monster_type>() as libc::c_ulong) as
            *mut monster_type;
    /* Count monsters */
    m_cnt -= 1;
    /* Do we survided our fate ? */
    if dungeon_type as libc::c_int == DUNGEON_DEATH && m_cnt == 0 {
        msg_print(b"You overcome your fate, mortal!\x00" as *const u8 as
                      *const libc::c_char);
        dungeon_type = 0 as libc::c_int as byte_hack;
        dun_level = 0 as libc::c_int as s16b;
        (*p_ptr).leaving = 1 as libc::c_int as bool_
    }
    /* Update monster light */
    if had_lite != 0 {
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x200000 as libc::c_long) as
                u32b
    }
    /* Update monster list window */
    (*p_ptr).window =
        ((*p_ptr).window as libc::c_long | 0x10 as libc::c_long) as u32b;
    /* Visual update */
    lite_spot(y, x);
}
/*
 * Delete the monster, if any, at a given location
 */
#[no_mangle]
pub unsafe extern "C" fn delete_monster(mut y: libc::c_int,
                                        mut x: libc::c_int) {
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Paranoia */
    if !(y > 0 as libc::c_int && x > 0 as libc::c_int &&
             y < cur_hgt as libc::c_int - 1 as libc::c_int &&
             x < cur_wid as libc::c_int - 1 as libc::c_int) {
        return
    }
    /* Check the grid */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Delete the monster (if any) */
    if (*c_ptr).m_idx != 0 {
        delete_monster_idx((*c_ptr).m_idx as libc::c_int);
    };
}
/*
 * Move an object from index i1 to index i2 in the object list
 */
unsafe extern "C" fn compact_monsters_aux(mut i1: libc::c_int,
                                          mut i2: libc::c_int) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    let mut this_o_idx: s16b = 0;
    let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
    /* Do nothing */
    if i1 == i2 { return }
    /* Old monster */
    m_ptr = &mut *m_list.offset(i1 as isize) as *mut monster_type;
    /* Location */
    y = (*m_ptr).fy as libc::c_int;
    x = (*m_ptr).fx as libc::c_int;
    /* Cave grid */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Update the cave */
    (*c_ptr).m_idx = i2 as s16b;
    /* Repair objects being carried by monster */
    this_o_idx = (*m_ptr).hold_o_idx;
    while this_o_idx != 0 {
        let mut o_ptr: *mut object_type = 0 as *mut object_type;
        /* Acquire object */
        o_ptr = &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
        /* Acquire next object */
        next_o_idx = (*o_ptr).next_o_idx;
        /* Reset monster pointer */
        (*o_ptr).held_m_idx = i2 as s16b;
        this_o_idx = next_o_idx
    }
    /* Hack -- Update the control */
    if (*p_ptr).control as libc::c_int == i1 { (*p_ptr).control = i2 as s16b }
    /* Hack -- Update the doppleganger */
    if doppleganger as libc::c_int == i1 { doppleganger = i2 as s16b }
    /* Hack -- Update the target */
    if target_who as libc::c_int == i1 { target_who = i2 as s16b }
    /* Hack -- Update the health bar */
    if health_who as libc::c_int == i1 { health_track(i2); }
    j = m_max as libc::c_int - 1 as libc::c_int;
    while j >= 1 as libc::c_int {
        /* Access the monster */
        let mut t_ptr: *mut monster_type =
            &mut *m_list.offset(j as isize) as *mut monster_type;
        /* Ignore "dead" monsters */
        if !((*t_ptr).r_idx == 0) {
            if (*t_ptr).target as libc::c_int == i1 {
                (*t_ptr).target = i2 as s16b
            }
        }
        j -= 1
    }
    /* Structure copy */
    memcpy(&mut *m_list.offset(i2 as isize) as *mut monster_type as
               *mut libc::c_char as *mut libc::c_void,
           &mut *m_list.offset(i1 as isize) as *mut monster_type as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<monster_type>() as libc::c_ulong);
    /* Delete mind & special race if needed */
    if !(*m_list.offset(i1 as isize)).sr_ptr.is_null() {
        let ref mut fresh2 = (*m_list.offset(i1 as isize)).sr_ptr;
        *fresh2 =
            rnfree((*m_list.offset(i1 as isize)).sr_ptr as vptr,
                   ::std::mem::size_of::<monster_race>() as libc::c_ulong) as
                *mut monster_race
    }
    if !(*m_list.offset(i1 as isize)).mind.is_null() {
        let ref mut fresh3 = (*m_list.offset(i1 as isize)).mind;
        *fresh3 =
            rnfree((*m_list.offset(i1 as isize)).mind as vptr,
                   ::std::mem::size_of::<monster_mind>() as libc::c_ulong) as
                *mut monster_mind
    }
    /* Wipe the hole */
    m_ptr =
        memset(&mut *m_list.offset(i1 as isize) as *mut monster_type as
                   *mut libc::c_char as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<monster_type>() as libc::c_ulong) as
            *mut monster_type;
}
/*
 * Compact and Reorder the monster list
 *
 * This function can be very dangerous, use with caution!
 *
 * When actually "compacting" monsters, we base the saving throw
 * on a combination of monster level, distance from player, and
 * current "desperation".
 *
 * After "compacting" (if needed), we "reorder" the monsters into a more
 * compact order, and we reset the allocation info, and the "live" array.
 */
#[no_mangle]
pub unsafe extern "C" fn compact_monsters(mut size: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut cur_lev: libc::c_int = 0;
    let mut cur_dis: libc::c_int = 0;
    let mut chance: libc::c_int = 0;
    /* Message (only if compacting) */
    if size != 0 {
        msg_print(b"Compacting monsters...\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Compact at least 'size' objects */
    num = 0 as libc::c_int;
    cnt = 1 as libc::c_int;
    while num < size {
        /* Get more vicious each iteration */
        cur_lev = 5 as libc::c_int * cnt;
        /* Get closer each iteration */
        cur_dis = 5 as libc::c_int * (20 as libc::c_int - cnt);
        /* Check all the monsters */
        i = 1 as libc::c_int;
        while i < m_max as libc::c_int {
            let mut m_ptr: *mut monster_type =
                &mut *m_list.offset(i as isize) as *mut monster_type;
            let mut r_ptr: *mut monster_race =
                if !(*m_ptr).sr_ptr.is_null() {
                    (*m_ptr).sr_ptr
                } else {
                    race_info_idx((*m_ptr).r_idx as libc::c_int,
                                  (*m_ptr).ego as libc::c_int)
                };
            /* Paranoia -- skip "dead" monsters */
            if !((*m_ptr).r_idx == 0) {
                /* Hack -- High level monsters start out "immune" */
                if !((*m_ptr).level as libc::c_int > cur_lev) {
                    /* Ignore nearby monsters */
                    if !(cur_dis > 0 as libc::c_int &&
                             ((*m_ptr).cdis as libc::c_int) < cur_dis) {
                        /* Saving throw chance */
                        chance = 90 as libc::c_int;
                        /* Only compact "Quest" Monsters in emergencies */
                        if (*m_ptr).mflag & 0x2 as libc::c_int != 0 &&
                               cnt < 1000 as libc::c_int {
                            chance = 100 as libc::c_int
                        }
                        /* Try not to compact Unique Monsters */
                        if (*r_ptr).flags1 &
                               0x1 as libc::c_int as libc::c_uint != 0 {
                            chance = 99 as libc::c_int
                        }
                        /* All monsters get a saving throw */
                        if !(Rand_div(100 as libc::c_int) < chance) {
                            /* Delete the monster */
                            delete_monster_idx(i);
                            /* Count the monster */
                            num += 1
                        }
                    }
                }
            }
            i += 1
        }
        cnt += 1
    }
    /* Excise dead monsters (backwards!) */
    i = m_max as libc::c_int - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        /* Get the i'th monster */
        let mut m_ptr_0: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        /* Skip real monsters */
        if !((*m_ptr_0).r_idx != 0) {
            /* Move last monster into open hole */
            compact_monsters_aux(m_max as libc::c_int - 1 as libc::c_int, i);
            /* Compress "m_max" */
            m_max -= 1
        }
        i -= 1
    };
}
/*
 * Delete/Remove all the monsters when the player leaves the level
 *
 * This is an efficient method of simulating multiple calls to the
 * "delete_monster()" function, with no visual effects.
 */
#[no_mangle]
pub unsafe extern "C" fn wipe_m_list() {
    let mut i: libc::c_int = 0;
    /* Delete all the monsters */
    i = m_max as libc::c_int - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        let mut r_ptr: *mut monster_race =
            if !(*m_ptr).sr_ptr.is_null() {
                (*m_ptr).sr_ptr
            } else {
                race_info_idx((*m_ptr).r_idx as libc::c_int,
                              (*m_ptr).ego as libc::c_int)
            };
        /* Skip dead monsters */
        if !((*m_ptr).r_idx == 0) {
            /* Mega-Hack -- preserve Unique's XXX XXX XXX */
            /* Hack -- Reduce the racial counter */
            (*r_ptr).cur_num = (*r_ptr).cur_num.wrapping_sub(1);
            /* Monster is gone */
            (*cave[(*m_ptr).fy as usize].offset((*m_ptr).fx as isize)).m_idx =
                0 as libc::c_int as s16b;
            /* Delete mind & special race if needed */
            if !(*m_ptr).sr_ptr.is_null() {
                (*m_ptr).sr_ptr =
                    rnfree((*m_ptr).sr_ptr as vptr,
                           ::std::mem::size_of::<monster_race>() as
                               libc::c_ulong) as *mut monster_race
            }
            if !(*m_ptr).mind.is_null() {
                (*m_ptr).mind =
                    rnfree((*m_ptr).mind as vptr,
                           ::std::mem::size_of::<monster_mind>() as
                               libc::c_ulong) as *mut monster_mind
            }
            /* Wipe the Monster */
            m_ptr =
                memset(m_ptr as *mut libc::c_char as *mut libc::c_void,
                       0 as libc::c_int,
                       ::std::mem::size_of::<monster_type>() as libc::c_ulong)
                    as *mut monster_type
        }
        i -= 1
    }
    /* Reset "m_max" */
    m_max = 1 as libc::c_int as s16b;
    /* Reset "m_cnt" */
    m_cnt = 0 as libc::c_int as s16b;
    /* Hack -- reset "reproducer" count */
    num_repro = 0 as libc::c_int as s16b;
    /* Hack -- no more target */
    target_who = 0 as libc::c_int as s16b;
    /* Reset control */
    (*p_ptr).control = 0 as libc::c_int as s16b;
    /* Hack -- no more tracking */
    health_track(0 as libc::c_int);
}
/*
 * Acquires and returns the index of a "free" monster.
 *
 * This routine should almost never fail, but it *can* happen.
 */
#[no_mangle]
pub unsafe extern "C" fn m_pop() -> s16b {
    let mut i: libc::c_int = 0;
    /* Normal allocation */
    if (m_max as libc::c_int) < max_m_idx as libc::c_int {
        /* Access the next hole */
        i = m_max as libc::c_int;
        /* Expand the array */
        m_max += 1;
        /* Count monsters */
        m_cnt += 1;
        /* Return the index */
        return i as s16b
    }
    /* Recycle dead monsters */
    i = 1 as libc::c_int;
    while i < m_max as libc::c_int {
        let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
        /* Acquire monster */
        m_ptr = &mut *m_list.offset(i as isize) as *mut monster_type;
        /* Skip live monsters */
        if (*m_ptr).r_idx != 0 {
            i += 1
        } else {
            /* Count monsters */
            m_cnt += 1;
            /* Use this monster */
            return i as s16b
        }
    }
    /* Warn the player (except during dungeon creation) */
    if character_dungeon != 0 {
        msg_print(b"Too many monsters!\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* Try not to crash */
    return 0 as libc::c_int as s16b;
}
/*
 * Apply a "monster restriction function" to the "monster allocation table"
 */
#[no_mangle]
pub unsafe extern "C" fn get_mon_num_prep() -> errr {
    let mut i: libc::c_int = 0;
    /* Scan the allocation table */
    i = 0 as libc::c_int;
    while i < alloc_race_size as libc::c_int {
        /* Get the entry */
        let mut entry: *mut alloc_entry =
            &mut *alloc_race_table.offset(i as isize) as *mut alloc_entry;
        /* Accept monsters which pass the restriction, if any */
        if (get_mon_num_hook.is_none() ||
                Some(get_mon_num_hook.expect("non-null function pointer")).expect("non-null function pointer")((*entry).index
                                                                                                                   as
                                                                                                                   libc::c_int)
                    as libc::c_int != 0) &&
               (get_mon_num2_hook.is_none() ||
                    Some(get_mon_num2_hook.expect("non-null function pointer")).expect("non-null function pointer")((*entry).index
                                                                                                                        as
                                                                                                                        libc::c_int)
                        as libc::c_int != 0) {
            /* Accept this monster */
            (*entry).prob2 = (*entry).prob1
        } else {
            /* Do not use this monster */
            /* Decline this monster */
            (*entry).prob2 = 0 as libc::c_int as byte_hack
        }
        i += 1
    }
    /* Success */
    return 0 as libc::c_int;
}
/*
 * Some dungeon types restrict the possible monsters.
 * Return TRUE is the monster is OK and FALSE otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn apply_rule(mut r_ptr: *mut monster_race,
                                    mut rule: byte_hack) -> bool_ {
    let mut d_ptr: *mut dungeon_info_type =
        &mut *d_info.offset(dungeon_type as isize) as *mut dungeon_info_type;
    if (*d_ptr).rules[rule as usize].mode as libc::c_int == 0 as libc::c_int {
        return 1 as libc::c_int as bool_
    } else {
        if (*d_ptr).rules[rule as usize].mode as libc::c_int ==
               1 as libc::c_int ||
               (*d_ptr).rules[rule as usize].mode as libc::c_int ==
                   2 as libc::c_int {
            let mut a: libc::c_int = 0;
            if (*d_ptr).rules[rule as usize].mflags1 != 0 {
                if (*d_ptr).rules[rule as usize].mflags1 & (*r_ptr).flags1 !=
                       (*d_ptr).rules[rule as usize].mflags1 {
                    return 0 as libc::c_int as bool_
                }
            }
            if (*d_ptr).rules[rule as usize].mflags2 != 0 {
                if (*d_ptr).rules[rule as usize].mflags2 & (*r_ptr).flags2 !=
                       (*d_ptr).rules[rule as usize].mflags2 {
                    return 0 as libc::c_int as bool_
                }
            }
            if (*d_ptr).rules[rule as usize].mflags3 != 0 {
                if (*d_ptr).rules[rule as usize].mflags3 & (*r_ptr).flags3 !=
                       (*d_ptr).rules[rule as usize].mflags3 {
                    return 0 as libc::c_int as bool_
                }
            }
            if (*d_ptr).rules[rule as usize].mflags4 != 0 {
                if (*d_ptr).rules[rule as usize].mflags4 & (*r_ptr).flags4 !=
                       (*d_ptr).rules[rule as usize].mflags4 {
                    return 0 as libc::c_int as bool_
                }
            }
            if (*d_ptr).rules[rule as usize].mflags5 != 0 {
                if (*d_ptr).rules[rule as usize].mflags5 & (*r_ptr).flags5 !=
                       (*d_ptr).rules[rule as usize].mflags5 {
                    return 0 as libc::c_int as bool_
                }
            }
            if (*d_ptr).rules[rule as usize].mflags6 != 0 {
                if (*d_ptr).rules[rule as usize].mflags6 & (*r_ptr).flags6 !=
                       (*d_ptr).rules[rule as usize].mflags6 {
                    return 0 as libc::c_int as bool_
                }
            }
            if (*d_ptr).rules[rule as usize].mflags7 != 0 {
                if (*d_ptr).rules[rule as usize].mflags7 & (*r_ptr).flags7 !=
                       (*d_ptr).rules[rule as usize].mflags7 {
                    return 0 as libc::c_int as bool_
                }
            }
            if (*d_ptr).rules[rule as usize].mflags8 != 0 {
                if (*d_ptr).rules[rule as usize].mflags8 & (*r_ptr).flags8 !=
                       (*d_ptr).rules[rule as usize].mflags8 {
                    return 0 as libc::c_int as bool_
                }
            }
            if (*d_ptr).rules[rule as usize].mflags9 != 0 {
                if (*d_ptr).rules[rule as usize].mflags9 & (*r_ptr).flags9 !=
                       (*d_ptr).rules[rule as usize].mflags9 {
                    return 0 as libc::c_int as bool_
                }
            }
            a = 0 as libc::c_int;
            while a < 5 as libc::c_int {
                if (*d_ptr).rules[rule as usize].r_char[a as usize] as
                       libc::c_int != 0 &&
                       (*d_ptr).rules[rule as usize].r_char[a as usize] as
                           libc::c_int != (*r_ptr).d_char as libc::c_int {
                    return 0 as libc::c_int as bool_
                }
                a += 1
            }
            /* All checks passed ? lets go ! */
            return 1 as libc::c_int as bool_
        } else {
            if (*d_ptr).rules[rule as usize].mode as libc::c_int ==
                   3 as libc::c_int ||
                   (*d_ptr).rules[rule as usize].mode as libc::c_int ==
                       4 as libc::c_int {
                let mut a_0: libc::c_int = 0;
                if (*d_ptr).rules[rule as usize].mflags1 != 0 &&
                       (*r_ptr).flags1 & (*d_ptr).rules[rule as usize].mflags1
                           != 0 {
                    return 1 as libc::c_int as bool_
                }
                if (*d_ptr).rules[rule as usize].mflags2 != 0 &&
                       (*r_ptr).flags2 & (*d_ptr).rules[rule as usize].mflags2
                           != 0 {
                    return 1 as libc::c_int as bool_
                }
                if (*d_ptr).rules[rule as usize].mflags3 != 0 &&
                       (*r_ptr).flags3 & (*d_ptr).rules[rule as usize].mflags3
                           != 0 {
                    return 1 as libc::c_int as bool_
                }
                if (*d_ptr).rules[rule as usize].mflags4 != 0 &&
                       (*r_ptr).flags4 & (*d_ptr).rules[rule as usize].mflags4
                           != 0 {
                    return 1 as libc::c_int as bool_
                }
                if (*d_ptr).rules[rule as usize].mflags5 != 0 &&
                       (*r_ptr).flags5 & (*d_ptr).rules[rule as usize].mflags5
                           != 0 {
                    return 1 as libc::c_int as bool_
                }
                if (*d_ptr).rules[rule as usize].mflags6 != 0 &&
                       (*r_ptr).flags6 & (*d_ptr).rules[rule as usize].mflags6
                           != 0 {
                    return 1 as libc::c_int as bool_
                }
                if (*d_ptr).rules[rule as usize].mflags7 != 0 &&
                       (*r_ptr).flags7 & (*d_ptr).rules[rule as usize].mflags7
                           != 0 {
                    return 1 as libc::c_int as bool_
                }
                if (*d_ptr).rules[rule as usize].mflags8 != 0 &&
                       (*r_ptr).flags8 & (*d_ptr).rules[rule as usize].mflags8
                           != 0 {
                    return 1 as libc::c_int as bool_
                }
                if (*d_ptr).rules[rule as usize].mflags9 != 0 &&
                       (*r_ptr).flags9 & (*d_ptr).rules[rule as usize].mflags9
                           != 0 {
                    return 1 as libc::c_int as bool_
                }
                a_0 = 0 as libc::c_int;
                while a_0 < 5 as libc::c_int {
                    if (*d_ptr).rules[rule as usize].r_char[a_0 as usize] as
                           libc::c_int == (*r_ptr).d_char as libc::c_int {
                        return 1 as libc::c_int as bool_
                    }
                    a_0 += 1
                }
                /* All checks failled ? Sorry ... */
                return 0 as libc::c_int as bool_
            }
        }
    }
    /* Should NEVER happen */
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn restrict_monster_to_dungeon(mut r_idx: libc::c_int)
 -> bool_ {
    let mut d_ptr: *mut dungeon_info_type =
        &mut *d_info.offset(dungeon_type as isize) as *mut dungeon_info_type;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    /* Select a random rule */
    let mut rule: byte_hack =
        (*d_ptr).rule_percents[Rand_div(100 as libc::c_int) as usize];
    /* Apply the rule */
    let mut rule_ret: bool_ = apply_rule(r_ptr, rule);
    /* Should the rule be right or not ? */
    if (*d_ptr).rules[rule as usize].mode as libc::c_int == 2 as libc::c_int
           ||
           (*d_ptr).rules[rule as usize].mode as libc::c_int ==
               4 as libc::c_int {
        rule_ret = (rule_ret == 0) as libc::c_int as bool_
    }
    /* Rule ok ? */
    if rule_ret != 0 { return 1 as libc::c_int as bool_ }
    /* Not allowed */
    return 0 as libc::c_int as bool_;
}
/* Ugly hack, let summon unappropriate monsters */
#[no_mangle]
pub static mut summon_hack: bool_ = 0 as libc::c_int as bool_;
/*
 * Choose a monster race that seems "appropriate" to the given level
 *
 * This function uses the "prob2" field of the "monster allocation table",
 * and various local information, to calculate the "prob3" field of the
 * same table, which is then used to choose an "appropriate" monster, in
 * a relatively efficient manner.
 *
 * Note that "town" monsters will *only* be created in the town, and
 * "normal" monsters will *never* be created in the town, unless the
 * "level" is "modified", for example, by polymorph or summoning.
 *
 * There is a small chance (1/50) of "boosting" the given depth by
 * a small amount (up to four levels), except in the town.
 *
 * It is (slightly) more likely to acquire a monster of the given level
 * than one of a lower level.  This is done by choosing several monsters
 * appropriate to the given level and keeping the "hardest" one.
 *
 * Note that if no monsters are "appropriate", then this function will
 * fail, and return zero, but this should *almost* never happen.
 */
#[no_mangle]
pub unsafe extern "C" fn get_mon_num(mut level: libc::c_int) -> s16b {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut r_idx: libc::c_int = 0;
    let mut value: libc::c_long = 0;
    let mut total: libc::c_long = 0;
    let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
    let mut table: *mut alloc_entry = alloc_race_table;
    let mut in_tome: libc::c_int = 0;
    /* Boost the level */
    if level > 0 as libc::c_int {
        /* Occasional "nasty" monster */
        if Rand_div(50 as libc::c_int) == 0 as libc::c_int {
            /* Pick a level bonus */
            let mut d: libc::c_int =
                level / 4 as libc::c_int + 2 as libc::c_int;
            /* Boost the level */
            level += if d < 5 as libc::c_int { d } else { 5 as libc::c_int }
        }
        /* Occasional "nasty" monster */
        if Rand_div(50 as libc::c_int) == 0 as libc::c_int {
            /* Pick a level bonus */
            let mut d_0: libc::c_int =
                level / 4 as libc::c_int + 2 as libc::c_int;
            /* Boost the level */
            level +=
                if d_0 < 5 as libc::c_int { d_0 } else { 5 as libc::c_int }
        }
    }
    /* Reset total */
    total = 0 as libc::c_long;
    /* Check whether this is ToME or a module */
    in_tome =
        (strcmp(game_module, b"ToME\x00" as *const u8 as *const libc::c_char)
             == 0 as libc::c_int) as libc::c_int;
    let mut current_block_15: u64;
    /* Process probabilities */
    i = 0 as libc::c_int;
    while i < alloc_race_size as libc::c_int {
        /* Monsters are sorted by depth */
        if (*table.offset(i as isize)).level as libc::c_int > level {
            break ;
        }
        /* Default */
        (*table.offset(i as isize)).prob3 = 0 as libc::c_int as byte_hack;
        /* Access the "r_idx" of the chosen monster */
        r_idx = (*table.offset(i as isize)).index as libc::c_int;
        /* Access the actual race */
        r_ptr = &mut *r_info.offset(r_idx as isize) as *mut monster_race;
        /* Hack -- "unique" monsters must be "unique" */
        if !((*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 &&
                 (*r_ptr).cur_num as libc::c_int >=
                     (*r_ptr).max_num as libc::c_int) {
            /* Depth Monsters never appear out of depth */
            if !((*r_ptr).flags1 & 0x100 as libc::c_int as libc::c_uint != 0
                     &&
                     (*r_ptr).level as libc::c_int > dun_level as libc::c_int)
               {
                /* Depth Monsters never appear out of their depth */
                if !((*r_ptr).flags9 & 0x1000 as libc::c_int as libc::c_uint
                         != 0 &&
                         (*r_ptr).level as libc::c_int !=
                             dun_level as libc::c_int) {
                    if in_tome != 0 {
                        /* Zangbandish monsters not allowed */
                        if (*r_ptr).flags8 &
                               0x4000 as libc::c_int as libc::c_uint != 0 {
                            current_block_15 = 8236137900636309791;
                        } else if (*r_ptr).flags8 &
                                      0x1000 as libc::c_int as libc::c_uint !=
                                      0 {
                            current_block_15 = 8236137900636309791;
                        } else { current_block_15 = 14359455889292382949; }
                    } else { current_block_15 = 14359455889292382949; }
                    match current_block_15 {
                        8236137900636309791 => { }
                        _ =>
                        /* Lovecraftian monsters not allowed */
                        /* Joke monsters allowed ? or not ? */
                        {
                            if !(joke_monsters == 0 &&
                                     (*r_ptr).flags8 &
                                         0x8000 as libc::c_int as libc::c_uint
                                         != 0) {
                                /* Some dungeon types restrict the possible monsters */
                                if !(summon_hack == 0 &&
                                         restrict_monster_to_dungeon(r_idx) ==
                                             0 &&
                                         dun_level as libc::c_int != 0) {
                                    /* Accept */
                                    (*table.offset(i as isize)).prob3 =
                                        (*table.offset(i as isize)).prob2;
                                    /* Total */
                                    total +=
                                        (*table.offset(i as isize)).prob3 as
                                            libc::c_long
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    /* No legal monsters */
    if total <= 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as s16b
    }
    /* Pick a monster */
    value = Rand_div(total as s32b) as libc::c_long;
    /* Find the monster */
    i = 0 as libc::c_int;
    while i < alloc_race_size as libc::c_int {
        /* Found the entry */
        if value < (*table.offset(i as isize)).prob3 as libc::c_long {
            break ;
        }
        /* Decrement */
        value = value - (*table.offset(i as isize)).prob3 as libc::c_long;
        i += 1
    }
    /* Power boost */
    p = Rand_div(100 as libc::c_int);
    /* Try for a "harder" monster once (50%) or twice (10%) */
    if p < 60 as libc::c_int {
        /* Save old */
        j = i;
        /* Pick a monster */
        value = Rand_div(total as s32b) as libc::c_long;
        /* Find the monster */
        i = 0 as libc::c_int;
        while i < alloc_race_size as libc::c_int {
            /* Found the entry */
            if value < (*table.offset(i as isize)).prob3 as libc::c_long {
                break ;
            }
            /* Decrement */
            value = value - (*table.offset(i as isize)).prob3 as libc::c_long;
            i += 1
        }
        /* Keep the "best" one */
        if ((*table.offset(i as isize)).level as libc::c_int) <
               (*table.offset(j as isize)).level as libc::c_int {
            i = j
        }
    }
    /* Try for a "harder" monster twice (10%) */
    if p < 10 as libc::c_int {
        /* Save old */
        j = i;
        /* Pick a monster */
        value = Rand_div(total as s32b) as libc::c_long;
        /* Find the monster */
        i = 0 as libc::c_int;
        while i < alloc_race_size as libc::c_int {
            /* Found the entry */
            if value < (*table.offset(i as isize)).prob3 as libc::c_long {
                break ;
            }
            /* Decrement */
            value = value - (*table.offset(i as isize)).prob3 as libc::c_long;
            i += 1
        }
        /* Keep the "best" one */
        if ((*table.offset(i as isize)).level as libc::c_int) <
               (*table.offset(j as isize)).level as libc::c_int {
            i = j
        }
    }
    /* Result */
    return (*table.offset(i as isize)).index;
}
/*
 * Build a string describing a monster in some way.
 *
 * We can correctly describe monsters based on their visibility.
 * We can force all monsters to be treated as visible or invisible.
 * We can build nominatives, objectives, possessives, or reflexives.
 * We can selectively pronominalize hidden, visible, or all monsters.
 * We can use definite or indefinite descriptions for hidden monsters.
 * We can use definite or indefinite descriptions for visible monsters.
 *
 * Pronominalization involves the gender whenever possible and allowed,
 * so that by cleverly requesting pronominalization / visibility, you
 * can get messages like "You hit someone.  She screams in agony!".
 *
 * Reflexives are acquired by requesting Objective plus Possessive.
 *
 * If no m_ptr arg is given (?), the monster is assumed to be hidden,
 * unless the "Assume Visible" mode is requested.
 *
 * If no r_ptr arg is given, it is extracted from m_ptr and r_info
 * If neither m_ptr nor r_ptr is given, the monster is assumed to
 * be neuter, singular, and hidden (unless "Assume Visible" is set),
 * in which case you may be in trouble... :-)
 *
 * I am assuming that no monster name is more than 70 characters long,
 * so that "char desc[80];" is sufficiently large for any result.
 *
 * Mode Flags:
 *   0x01 --> Objective (or Reflexive)
 *   0x02 --> Possessive (or Reflexive)
 *   0x04 --> Use indefinites for hidden monsters ("something")
 *   0x08 --> Use indefinites for visible monsters ("a kobold")
 *   0x10 --> Pronominalize hidden monsters
 *   0x20 --> Pronominalize visible monsters
 *   0x40 --> Assume the monster is hidden
 *   0x80 --> Assume the monster is visible
 *  0x100 --> Ignore insanity
 *
 * Useful Modes:
 *   0x00 --> Full nominative name ("the kobold") or "it"
 *   0x04 --> Full nominative name ("the kobold") or "something"
 *   0x80 --> Genocide resistance name ("the kobold")
 *   0x88 --> Killing name ("a kobold")
 *   0x22 --> Possessive, genderized if visible ("his") or "its"
 *   0x23 --> Reflexive, genderized if visible ("himself") or "itself"
 */
#[no_mangle]
pub unsafe extern "C" fn monster_desc(mut desc: *mut libc::c_char,
                                      mut m_ptr: *mut monster_type,
                                      mut mode: libc::c_int) {
    let mut res: cptr = 0 as *const libc::c_char;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    let mut b_name: cptr = r_name.offset((*r_ptr).name as isize) as cptr;
    let mut silly_name: [libc::c_char; 80] = [0; 80];
    let mut name: [libc::c_char; 100] = [0; 100];
    let mut seen: bool_ = 0;
    let mut pron: bool_ = 0;
    let mut insanity: libc::c_int =
        ((*p_ptr).msane as libc::c_int - (*p_ptr).csane as libc::c_int) *
            100 as libc::c_int / (*p_ptr).msane as libc::c_int;
    if (*m_ptr).ego != 0 {
        if (*re_info.offset((*m_ptr).ego as isize)).before != 0 {
            sprintf(name.as_mut_ptr(),
                    b"%s %s\x00" as *const u8 as *const libc::c_char,
                    re_name.offset((*re_info.offset((*m_ptr).ego as
                                                        isize)).name as
                                       isize), b_name);
        } else {
            sprintf(name.as_mut_ptr(),
                    b"%s %s\x00" as *const u8 as *const libc::c_char, b_name,
                    re_name.offset((*re_info.offset((*m_ptr).ego as
                                                        isize)).name as
                                       isize));
        }
    } else {
        sprintf(name.as_mut_ptr(),
                b"%s\x00" as *const u8 as *const libc::c_char, b_name);
    }
    /*
	 * Are we hallucinating? (Idea from Nethack...)
	 * insanity roll added by pelpel
	 */
    if mode & 0x100 as libc::c_int == 0 &&
           ((*p_ptr).image as libc::c_int != 0 ||
                Rand_div(300 as libc::c_int) < insanity) {
        if Rand_div(2 as libc::c_int) == 0 as libc::c_int {
            let mut hallu_race: *mut monster_race = 0 as *mut monster_race;
            loop  {
                hallu_race =
                    &mut *r_info.offset(((Rand_div as
                                              unsafe extern "C" fn(_: s32b)
                                                  ->
                                                      s32b)(max_r_idx as
                                                                libc::c_int -
                                                                2 as
                                                                    libc::c_int)
                                             + 1 as libc::c_int) as isize) as
                        *mut monster_race;
                if !((*hallu_race).flags1 & 0x1 as libc::c_int as libc::c_uint
                         != 0) {
                    break ;
                }
            }
            strcpy(silly_name.as_mut_ptr(),
                   r_name.offset((*hallu_race).name as isize));
        } else {
            get_rnd_line(b"silly.txt\x00" as *const u8 as *const libc::c_char
                             as *mut libc::c_char, silly_name.as_mut_ptr());
        }
        strcpy(name.as_mut_ptr(), silly_name.as_mut_ptr());
    }
    /* Can we "see" it (exists + forced, or visible + not unforced) */
    seen =
        (!m_ptr.is_null() &&
             (mode & 0x80 as libc::c_int != 0 ||
                  mode & 0x40 as libc::c_int == 0 &&
                      (*m_ptr).ml as libc::c_int != 0)) as libc::c_int as
            bool_;
    /* Sexed Pronouns (seen and allowed, or unseen and allowed) */
    pron =
        (!m_ptr.is_null() &&
             (seen as libc::c_int != 0 && mode & 0x20 as libc::c_int != 0 ||
                  seen == 0 && mode & 0x10 as libc::c_int != 0)) as
            libc::c_int as bool_;
    /* First, try using pronouns, or describing hidden monsters */
    if seen == 0 || pron as libc::c_int != 0 {
        /* an encoding of the monster "sex" */
        let mut kind: libc::c_int = 0 as libc::c_int;
        /* Extract the gender (if applicable) */
        if (*r_ptr).flags1 & 0x8 as libc::c_int as libc::c_uint != 0 {
            kind = 0x20 as libc::c_int
        } else if (*r_ptr).flags1 & 0x4 as libc::c_int as libc::c_uint != 0 {
            kind = 0x10 as libc::c_int
        }
        /* Ignore the gender (if desired) */
        if m_ptr.is_null() || pron == 0 { kind = 0 as libc::c_int }
        /* Assume simple result */
        res = b"it\x00" as *const u8 as *const libc::c_char;
        /* Brute force: split on the possibilities */
        match kind | mode & 0x7 as libc::c_int {
            0 => {
                /* Neuter, or unknown */
                res = b"it\x00" as *const u8 as *const libc::c_char
            }
            1 => { res = b"it\x00" as *const u8 as *const libc::c_char }
            2 => { res = b"its\x00" as *const u8 as *const libc::c_char }
            3 => { res = b"itself\x00" as *const u8 as *const libc::c_char }
            4 => {
                res = b"something\x00" as *const u8 as *const libc::c_char
            }
            5 => {
                res = b"something\x00" as *const u8 as *const libc::c_char
            }
            6 => {
                res = b"something\'s\x00" as *const u8 as *const libc::c_char
            }
            7 => { res = b"itself\x00" as *const u8 as *const libc::c_char }
            16 => {
                /* Male (assume human if vague) */
                res = b"he\x00" as *const u8 as *const libc::c_char
            }
            17 => { res = b"him\x00" as *const u8 as *const libc::c_char }
            18 => { res = b"his\x00" as *const u8 as *const libc::c_char }
            19 => { res = b"himself\x00" as *const u8 as *const libc::c_char }
            20 => { res = b"someone\x00" as *const u8 as *const libc::c_char }
            21 => { res = b"someone\x00" as *const u8 as *const libc::c_char }
            22 => {
                res = b"someone\'s\x00" as *const u8 as *const libc::c_char
            }
            23 => { res = b"himself\x00" as *const u8 as *const libc::c_char }
            32 => {
                /* Female (assume human if vague) */
                res = b"she\x00" as *const u8 as *const libc::c_char
            }
            33 => { res = b"her\x00" as *const u8 as *const libc::c_char }
            34 => { res = b"her\x00" as *const u8 as *const libc::c_char }
            35 => { res = b"herself\x00" as *const u8 as *const libc::c_char }
            36 => { res = b"someone\x00" as *const u8 as *const libc::c_char }
            37 => { res = b"someone\x00" as *const u8 as *const libc::c_char }
            38 => {
                res = b"someone\'s\x00" as *const u8 as *const libc::c_char
            }
            39 => { res = b"herself\x00" as *const u8 as *const libc::c_char }
            _ => { }
        }
        /* Copy the result */
        strcpy(desc, res);
    } else if mode & 0x2 as libc::c_int != 0 && mode & 0x1 as libc::c_int != 0
     {
        /* Handle visible monsters, "reflexive" request */
        /* The monster is visible, so use its gender */
        if (*r_ptr).flags1 & 0x8 as libc::c_int as libc::c_uint != 0 {
            strcpy(desc, b"herself\x00" as *const u8 as *const libc::c_char);
        } else if (*r_ptr).flags1 & 0x4 as libc::c_int as libc::c_uint != 0 {
            strcpy(desc, b"himself\x00" as *const u8 as *const libc::c_char);
        } else {
            strcpy(desc, b"itself\x00" as *const u8 as *const libc::c_char);
        }
    } else {
        /* Handle all other visible monster requests */
        /* It could be a Unique */
        if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 &&
               (*p_ptr).image == 0 {
            /* Start with the name (thus nominative and objective) */
            strcpy(desc, name.as_mut_ptr());
        } else if mode & 0x8 as libc::c_int != 0 {
            /* It could be an indefinite monster */
            /* XXX Check plurality for "some" */
            /* Indefinite monsters need an indefinite article */
            strcpy(desc,
                   if is_a_vowel(name[0 as libc::c_int as usize] as
                                     libc::c_int) as libc::c_int != 0 {
                       b"an \x00" as *const u8 as *const libc::c_char
                   } else { b"a \x00" as *const u8 as *const libc::c_char });
            strcat(desc, name.as_mut_ptr());
        } else {
            /* It could be a normal, definite, monster */
            /* Definite monsters need a definite article */
            if (*m_ptr).status as libc::c_int >= 3 as libc::c_int {
                strcpy(desc,
                       b"your \x00" as *const u8 as *const libc::c_char);
            } else {
                strcpy(desc, b"the \x00" as *const u8 as *const libc::c_char);
            }
            strcat(desc, name.as_mut_ptr());
        }
        if mode & 0x2 as libc::c_int != 0 {
            /* XXX Check for trailing "s" */
            /* Simply append "apostrophe" and "s" */
            strcat(desc, b"\'s\x00" as *const u8 as *const libc::c_char);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn monster_race_desc(mut desc: *mut libc::c_char,
                                           mut r_idx: libc::c_int,
                                           mut ego: libc::c_int) {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    let mut b_name: cptr = r_name.offset((*r_ptr).name as isize) as cptr;
    let mut name: [libc::c_char; 80] = [0; 80];
    if ego != 0 {
        if (*re_info.offset(ego as isize)).before != 0 {
            sprintf(name.as_mut_ptr(),
                    b"%s %s\x00" as *const u8 as *const libc::c_char,
                    re_name.offset((*re_info.offset(ego as isize)).name as
                                       isize), b_name);
        } else {
            sprintf(name.as_mut_ptr(),
                    b"%s %s\x00" as *const u8 as *const libc::c_char, b_name,
                    re_name.offset((*re_info.offset(ego as isize)).name as
                                       isize));
        }
    } else {
        sprintf(name.as_mut_ptr(),
                b"%s\x00" as *const u8 as *const libc::c_char, b_name);
    }
    /* Handle the Possessive as a special afterthought */
    /* It could be a Unique */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        /* Start with the name (thus nominative and objective) */
        strcpy(desc, name.as_mut_ptr());
    } else {
        /* It could be a normal, definite, monster */
        /* Definite monsters need a definite article */
        strcpy(desc,
               if is_a_vowel(name[0 as libc::c_int as usize] as libc::c_int)
                      as libc::c_int != 0 {
                   b"an \x00" as *const u8 as *const libc::c_char
               } else { b"a \x00" as *const u8 as *const libc::c_char });
        strcat(desc, name.as_mut_ptr());
    };
}
/*
 * Learn about a monster (by "probing" it)
 */
#[no_mangle]
pub unsafe extern "C" fn lore_do_probe(mut m_idx: libc::c_int) {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*m_ptr).r_idx as isize) as *mut monster_race;
    /* Hack -- Memorize some flags */
    (*r_ptr).r_flags1 = (*r_ptr).flags1;
    (*r_ptr).r_flags2 = (*r_ptr).flags2;
    (*r_ptr).r_flags3 = (*r_ptr).flags3;
    /* Update monster recall window */
    if monster_race_idx as libc::c_int == (*m_ptr).r_idx as libc::c_int {
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x100 as libc::c_long) as u32b
    };
}
/*
 * Take note that the given monster just dropped some treasure
 *
 * Note that learning the "GOOD"/"GREAT" flags gives information
 * about the treasure (even when the monster is killed for the first
 * time, such as uniques, and the treasure has not been examined yet).
 *
 * This "indirect" method is used to prevent the player from learning
 * exactly how much treasure a monster can drop from observing only
 * a single example of a drop.  This method actually observes how much
 * gold and items are dropped, and remembers that information to be
 * described later by the monster recall code.
 */
#[no_mangle]
pub unsafe extern "C" fn lore_treasure(mut m_idx: libc::c_int,
                                       mut num_item: libc::c_int,
                                       mut num_gold: libc::c_int) {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*m_ptr).r_idx as isize) as *mut monster_race;
    /* Note the number of things dropped */
    if num_item > (*r_ptr).r_drop_item as libc::c_int {
        (*r_ptr).r_drop_item = num_item as byte_hack
    }
    if num_gold > (*r_ptr).r_drop_gold as libc::c_int {
        (*r_ptr).r_drop_gold = num_gold as byte_hack
    }
    /* Hack -- memorize the good/great flags */
    if (*r_ptr).flags1 & 0x10000000 as libc::c_int as libc::c_uint != 0 {
        (*r_ptr).r_flags1 |= 0x10000000 as libc::c_int as libc::c_uint
    }
    if (*r_ptr).flags1 & 0x20000000 as libc::c_int as libc::c_uint != 0 {
        (*r_ptr).r_flags1 |= 0x20000000 as libc::c_int as libc::c_uint
    }
    /* Update monster recall window */
    if monster_race_idx as libc::c_int == (*m_ptr).r_idx as libc::c_int {
        /* Window stuff */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x100 as libc::c_long) as u32b
    }; /* No effect yet, just loaded... */
}
#[no_mangle]
pub unsafe extern "C" fn sanity_blast(mut m_ptr: *mut monster_type,
                                      mut necro: bool_) {
    let mut happened: bool_ =
        0 as libc::c_int as bool_; /* Cannot see it for some reason */
    let mut power: libc::c_int = 100 as libc::c_int; /* oops */
    if necro == 0 {
        let mut m_name: [libc::c_char; 80] =
            [0; 80]; /* Pet eldritch horrors are safe most of the time */
        let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
        if !m_ptr.is_null() {
            r_ptr =
                if !(*m_ptr).sr_ptr.is_null() {
                    (*m_ptr).sr_ptr
                } else {
                    race_info_idx((*m_ptr).r_idx as libc::c_int,
                                  (*m_ptr).ego as libc::c_int)
                }
        } else { return }
        power = (*m_ptr).level as libc::c_int + 10 as libc::c_int;
        if !m_ptr.is_null() {
            monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
            if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint == 0 {
                if (*r_ptr).flags1 & 0x2000 as libc::c_int as libc::c_uint !=
                       0 {
                    power /= 2 as libc::c_int
                }
            } else { power *= 2 as libc::c_int }
            if hack_mind == 0 { return }
            if (*m_ptr).ml == 0 { return }
            if (*r_ptr).flags2 & 0x2000 as libc::c_int as libc::c_uint == 0 {
                return
            }
            if is_friend(m_ptr) > 0 as libc::c_int &&
                   Rand_div(8 as libc::c_int) + 1 as libc::c_int !=
                       1 as libc::c_int {
                return
            }
            if (Rand_div(power) + 1 as libc::c_int) <
                   (*p_ptr).skill_sav as libc::c_int {
                return
                /* Save, no adverse effects */
            }
            if (*p_ptr).image != 0 {
                /* Something silly happens... */
                msg_format(b"You behold the %s visage of %s!\x00" as *const u8
                               as *const libc::c_char,
                           funny_desc[(Rand_div(22 as libc::c_int) +
                                           1 as libc::c_int -
                                           1 as libc::c_int) as usize],
                           m_name.as_mut_ptr());
                if Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                       1 as libc::c_int {
                    msg_print(funny_comments[(Rand_div(5 as libc::c_int) +
                                                  1 as libc::c_int -
                                                  1 as libc::c_int) as
                                                 usize]);
                    (*p_ptr).image =
                        ((*p_ptr).image as libc::c_int +
                             (Rand_div((*m_ptr).level as s32b) +
                                  1 as libc::c_int)) as s16b
                }
                return
                /* Never mind; we can't see it clearly enough */
            }
            /* Something frightening happens... */
            msg_format(b"You behold the %s visage of %s!\x00" as *const u8 as
                           *const libc::c_char,
                       horror_desc[(Rand_div(20 as libc::c_int) +
                                        1 as libc::c_int - 1 as libc::c_int)
                                       as usize], m_name.as_mut_ptr());
            (*r_ptr).r_flags2 |= 0x2000 as libc::c_int as libc::c_uint
        }
        /* Undead characters are 50% likely to be unaffected */
        if ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
                (*spp_ptr).flags1) as libc::c_long & 0x400 as libc::c_long !=
               0 ||
               (*p_ptr).mimic_form as libc::c_int ==
                   resolve_mimic_name(b"Vampire\x00" as *const u8 as
                                          *const libc::c_char) {
            if (Rand_div(100 as libc::c_int) + 1 as libc::c_int) <
                   25 as libc::c_int + (*p_ptr).lev as libc::c_int {
                return
            }
        }
    } else {
        msg_print(b"Your sanity is shaken by reading the Necronomicon!\x00" as
                      *const u8 as *const libc::c_char);
    }
    if (Rand_div(power) + 1 as libc::c_int) <
           (*p_ptr).skill_sav as libc::c_int {
        /* Mind blast */
        if (*p_ptr).resist_conf == 0 {
            set_confused((*p_ptr).confused as libc::c_int +
                             Rand_div(4 as libc::c_int) + 4 as libc::c_int);
        }
        if (*p_ptr).resist_chaos == 0 &&
               Rand_div(3 as libc::c_int) + 1 as libc::c_int ==
                   1 as libc::c_int {
            set_image((*p_ptr).image as libc::c_int +
                          Rand_div(250 as libc::c_int) + 150 as libc::c_int);
        }
        return
    }
    if (Rand_div(power) + 1 as libc::c_int) <
           (*p_ptr).skill_sav as libc::c_int {
        /* Lose int & wis */
        do_dec_stat(1 as libc::c_int, 2 as libc::c_int);
        do_dec_stat(2 as libc::c_int, 2 as libc::c_int);
        return
    }
    if (Rand_div(power) + 1 as libc::c_int) <
           (*p_ptr).skill_sav as libc::c_int {
        /* Brain smash */
        if (*p_ptr).resist_conf == 0 {
            set_confused((*p_ptr).confused as libc::c_int +
                             Rand_div(4 as libc::c_int) + 4 as libc::c_int);
        }
        if (*p_ptr).free_act == 0 {
            set_paralyzed((*p_ptr).paralyzed as libc::c_int +
                              Rand_div(4 as libc::c_int) + 4 as libc::c_int);
        }
        while Rand_div(100 as libc::c_int) > (*p_ptr).skill_sav as libc::c_int
              {
            do_dec_stat(1 as libc::c_int, 2 as libc::c_int);
        }
        while Rand_div(100 as libc::c_int) > (*p_ptr).skill_sav as libc::c_int
              {
            do_dec_stat(2 as libc::c_int, 2 as libc::c_int);
        }
        if (*p_ptr).resist_chaos == 0 {
            set_image((*p_ptr).image as libc::c_int +
                          Rand_div(250 as libc::c_int) + 150 as libc::c_int);
        }
        return
    }
    if (Rand_div(power) + 1 as libc::c_int) <
           (*p_ptr).skill_sav as libc::c_int {
        /* Permanent lose int & wis */
        if dec_stat(1 as libc::c_int, 10 as libc::c_int, 1 as libc::c_int) !=
               0 {
            happened = 1 as libc::c_int as bool_
        }
        if dec_stat(2 as libc::c_int, 10 as libc::c_int, 1 as libc::c_int) !=
               0 {
            happened = 1 as libc::c_int as bool_
        }
        if happened != 0 {
            msg_print(b"You feel much less sane than before.\x00" as *const u8
                          as *const libc::c_char);
        }
        return
    }
    if (Rand_div(power) + 1 as libc::c_int) <
           (*p_ptr).skill_sav as libc::c_int {
        /* Amnesia */
        if lose_all_info() != 0 {
            msg_print(b"You forget everything in your utmost terror!\x00" as
                          *const u8 as *const libc::c_char);
        }
        return
    }
    (*p_ptr).update =
        ((*p_ptr).update as libc::c_long | 0x1 as libc::c_long) as u32b;
    handle_stuff();
}
/*
 * This function updates the monster record of the given monster
 *
 * This involves extracting the distance to the player, checking
 * for visibility (natural, infravision, see-invis, telepathy),
 * updating the monster visibility flag, redrawing or erasing the
 * monster when the visibility changes, and taking note of any
 * "visual" features of the monster (cold-blooded, invisible, etc).
 *
 * The only monster fields that are changed here are "cdis" (the
 * distance from the player), "los" (clearly visible to player),
 * and "ml" (visible to the player in any way).
 *
 * There are a few cases where the calling routine knows that the
 * distance from the player to the monster has not changed, and so
 * we have a special parameter "full" to request distance computation.
 * This lets many calls to this function run very quickly.
 *
 * Note that every time a monster moves, we must call this function
 * for that monster, and update distance.  Note that every time the
 * player moves, we must call this function for every monster, and
 * update distance.  Note that every time the player "state" changes
 * in certain ways (including "blindness", "infravision", "telepathy",
 * and "see invisible"), we must call this function for every monster.
 *
 * The routines that actually move the monsters call this routine
 * directly, and the ones that move the player, or notice changes
 * in the player state, call "update_monsters()".
 *
 * Routines that change the "illumination" of grids must also call
 * this function, since the "visibility" of some monsters may be
 * based on the illumination of their grid.
 *
 * Note that this function is called once per monster every time the
 * player moves, so it is important to optimize it for monsters which
 * are far away.  Note the optimization which skips monsters which
 * are far away and were completely invisible last turn.
 *
 * Note the optimized "inline" version of the "distance()" function.
 *
 * Note that only monsters on the current panel can be "visible",
 * and then only if they are (1) in line of sight and illuminated
 * by light or infravision, or (2) nearby and detected by telepathy.
 *
 * The player can choose to be disturbed by several things, including
 * "disturb_move" (monster which is viewable moves in some way), and
 * "disturb_near" (monster which is "easily" viewable moves in some
 * way).  Note that "moves" includes "appears" and "disappears".
 *
 * Note the new "xtra" field which encodes several state flags such
 * as "detected last turn", and "detected this turn", and "currently
 * in line of sight", all of which are used for visibility testing.
 */
#[no_mangle]
pub unsafe extern "C" fn update_mon(mut m_idx: libc::c_int, mut full: bool_) {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    /* The current monster location */
    let mut fy: libc::c_int = (*m_ptr).fy as libc::c_int;
    let mut fx: libc::c_int = (*m_ptr).fx as libc::c_int;
    let mut old_ml: bool_ = (*m_ptr).ml;
    /* Seen at all */
    let mut flag: bool_ = 0 as libc::c_int as bool_;
    /* Seen by vision */
    let mut easy: bool_ = 0 as libc::c_int as bool_;
    /* Seen by telepathy */
    let mut hard: bool_ = 0 as libc::c_int as bool_;
    /* Various extra flags */
    let mut do_empty_mind: bool_ = 0 as libc::c_int as bool_;
    let mut do_weird_mind: bool_ = 0 as libc::c_int as bool_;
    let mut do_invisible: bool_ = 0 as libc::c_int as bool_;
    let mut do_cold_blood: bool_ = 0 as libc::c_int as bool_;
    /* Calculate distance */
    if full != 0 {
        let mut d: libc::c_int = 0;
        let mut dy: libc::c_int = 0;
        let mut dx: libc::c_int = 0;
        /* Distance components */
        dy =
            if (*p_ptr).py as libc::c_int > fy {
                ((*p_ptr).py as libc::c_int) - fy
            } else { (fy) - (*p_ptr).py as libc::c_int };
        dx =
            if (*p_ptr).px as libc::c_int > fx {
                ((*p_ptr).px as libc::c_int) - fx
            } else { (fx) - (*p_ptr).px as libc::c_int };
        /* Approximate distance */
        d =
            if dy > dx {
                (dy) + (dx >> 1 as libc::c_int)
            } else { (dx) + (dy >> 1 as libc::c_int) };
        /* Save the distance (in a byte) */
        (*m_ptr).cdis =
            if d < 255 as libc::c_int { d } else { 255 as libc::c_int } as
                byte_hack
    }
    /* Process "distant" monsters */
    if (*m_ptr).cdis as libc::c_int > 20 as libc::c_int {
        /* Ignore unseen monsters */
        if (*m_ptr).ml == 0 { return }
        /* Detected */
        if (*m_ptr).mflag & 0x80 as libc::c_int != 0 {
            flag = 1 as libc::c_int as bool_
        }
    } else if fy >= panel_row_min as libc::c_int &&
                  fy <= panel_row_max as libc::c_int &&
                  fx >= panel_col_min as libc::c_int &&
                  fx <= panel_col_max as libc::c_int {
        /* Process "nearby" monsters on the current "panel" */
        /* Normal line of sight, and player is not blind */
        if (*cave[fy as usize].offset(fx as isize)).info as libc::c_int &
               0x20 as libc::c_int != 0 as libc::c_int && (*p_ptr).blind == 0
           {
            /* Use "infravision" */
            if (*m_ptr).cdis as libc::c_int <=
                   (*p_ptr).see_infra as byte_hack as libc::c_int {
                /* Infravision only works on "warm" creatures */
				/* Below, we will need to know that infravision failed */
                if (*r_ptr).flags2 & 0x20 as libc::c_int as libc::c_uint != 0
                   {
                    do_cold_blood = 1 as libc::c_int as bool_
                }
                /* Infravision works */
                if do_cold_blood == 0 {
                    flag = 1 as libc::c_int as bool_;
                    easy = flag
                }
            }
            /* Use "illumination" */
            if (*cave[fy as usize].offset(fx as isize)).info as libc::c_int &
                   0x10 as libc::c_int != 0 as libc::c_int {
                /* Take note of invisibility */
                if (*r_ptr).flags2 & 0x10 as libc::c_int as libc::c_uint != 0
                   {
                    do_invisible = 1 as libc::c_int as bool_
                }
                /* Visible, or detectable, monsters get seen */
                if do_invisible == 0 || (*p_ptr).see_inv as libc::c_int != 0 {
                    flag = 1 as libc::c_int as bool_;
                    easy = flag
                }
            }
        }
        /* Telepathy can see all "nearby" monsters with "minds" */
        if (*p_ptr).telepathy != 0 {
            /* Assume we cant see */
            let mut can_esp: bool_ = 0 as libc::c_int as bool_;
            /* Different ESP */
            if (*p_ptr).telepathy as libc::c_long & 0x1 as libc::c_long != 0
                   &&
                   (*r_ptr).flags3 & 0x1 as libc::c_int as libc::c_uint != 0 {
                can_esp = 1 as libc::c_int as bool_
            }
            if (*p_ptr).telepathy as libc::c_long & 0x1000 as libc::c_long !=
                   0 &&
                   (*r_ptr).flags7 & 0x40 as libc::c_int as libc::c_uint != 0
               {
                can_esp = 1 as libc::c_int as bool_
            }
            if (*p_ptr).telepathy as libc::c_long & 0x2 as libc::c_long != 0
                   &&
                   (*r_ptr).flags3 & 0x2 as libc::c_int as libc::c_uint != 0 {
                can_esp = 1 as libc::c_int as bool_
            }
            if (*p_ptr).telepathy as libc::c_long & 0x4 as libc::c_long != 0
                   &&
                   (*r_ptr).flags3 & 0x8 as libc::c_int as libc::c_uint != 0 {
                can_esp = 1 as libc::c_int as bool_
            }
            if (*p_ptr).telepathy as libc::c_long & 0x8 as libc::c_long != 0
                   &&
                   (*r_ptr).flags3 & 0x4 as libc::c_int as libc::c_uint != 0 {
                can_esp = 1 as libc::c_int as bool_
            }
            if (*p_ptr).telepathy as libc::c_long & 0x10 as libc::c_long != 0
                   &&
                   (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0
               {
                can_esp = 1 as libc::c_int as bool_
            }
            if (*p_ptr).telepathy as libc::c_long & 0x20 as libc::c_long != 0
                   &&
                   (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0
               {
                can_esp = 1 as libc::c_int as bool_
            }
            if (*p_ptr).telepathy as libc::c_long & 0x40 as libc::c_long != 0
                   &&
                   (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint != 0
               {
                can_esp = 1 as libc::c_int as bool_
            }
            if (*p_ptr).telepathy as libc::c_long & 0x80 as libc::c_long != 0
                   &&
                   (*r_ptr).flags3 & 0x80 as libc::c_int as libc::c_uint != 0
               {
                can_esp = 1 as libc::c_int as bool_
            }
            if (*p_ptr).telepathy as libc::c_long & 0x100 as libc::c_long != 0
                   &&
                   (*r_ptr).flags3 & 0x100 as libc::c_int as libc::c_uint != 0
               {
                can_esp = 1 as libc::c_int as bool_
            }
            if (*p_ptr).telepathy as libc::c_long & 0x200 as libc::c_long != 0
                   &&
                   (*r_ptr).flags3 & 0x200 as libc::c_int as libc::c_uint != 0
               {
                can_esp = 1 as libc::c_int as bool_
            }
            if (*p_ptr).telepathy as libc::c_long & 0x400 as libc::c_long != 0
                   &&
                   (*r_ptr).flags3 & 0x800 as libc::c_int as libc::c_uint != 0
               {
                can_esp = 1 as libc::c_int as bool_
            }
            if (*p_ptr).telepathy as libc::c_long & 0x800 as libc::c_long != 0
                   &&
                   ((*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0
                        ||
                        (*r_ptr).flags3 &
                            0x8000000 as libc::c_int as libc::c_uint != 0) {
                can_esp = 1 as libc::c_int as bool_
            }
            if (*p_ptr).telepathy as libc::c_long & 0x80000000 as libc::c_long
                   != 0 {
                can_esp = 1 as libc::c_int as bool_
            }
            /* Only do this when we can really detect monster */
            if can_esp != 0 {
                /* Empty mind, no telepathy */
                if (*r_ptr).flags2 & 0x40 as libc::c_int as libc::c_uint != 0
                   {
                    do_empty_mind = 1 as libc::c_int as bool_
                } else if (*r_ptr).flags2 &
                              0x80 as libc::c_int as libc::c_uint != 0 {
                    do_weird_mind = 1 as libc::c_int as bool_;
                    if Rand_div(100 as libc::c_int) < 10 as libc::c_int {
                        hard = 1 as libc::c_int as bool_;
                        flag = 1 as libc::c_int as bool_
                    }
                } else {
                    /* Weird mind, occasional telepathy */
                    /* Normal mind, allow telepathy */
                    hard = 1 as libc::c_int as bool_;
                    flag = 1 as libc::c_int as bool_
                }
            }
        }
        /* Apply "detection" spells */
        if (*m_ptr).mflag & 0x80 as libc::c_int != 0 {
            flag = 1 as libc::c_int as bool_
        }
        /* Hack -- Wizards have "perfect telepathy" */
        if wizard != 0 { flag = 1 as libc::c_int as bool_ }
    }
    /* The monster is now visible */
    if flag != 0 {
        /* It was previously unseen */
        if (*m_ptr).ml == 0 {
            /* Mark as visible */
            (*m_ptr).ml = 1 as libc::c_int as bool_;
            /* Draw the monster */
            lite_spot(fy, fx);
            /* Update health bar as needed */
            if health_who as libc::c_int == m_idx {
                (*p_ptr).redraw =
                    ((*p_ptr).redraw as libc::c_long | 0x800 as libc::c_long)
                        as u32b
            }
            /* Update monster list window */
            (*p_ptr).window =
                ((*p_ptr).window as libc::c_long | 0x10 as libc::c_long) as
                    u32b;
            /* Hack -- Count "fresh" sightings */
            if ((*r_ptr).r_sights as libc::c_int) < 32767 as libc::c_int {
                (*r_ptr).r_sights += 1
            }
            /* Disturb on appearance */
            if disturb_move != 0 {
                if disturb_pets as libc::c_int != 0 ||
                       is_friend(m_ptr) <= 0 as libc::c_int {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                }
            }
        }
        /* Apply telepathy */
        if hard != 0 {
            /* Hack -- Memorize mental flags */
            if (*r_ptr).flags2 & 0x2 as libc::c_int as libc::c_uint != 0 {
                (*r_ptr).r_flags2 |= 0x2 as libc::c_int as libc::c_uint
            }
            if (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint != 0 {
                (*r_ptr).r_flags2 |= 0x1 as libc::c_int as libc::c_uint
            }
        }
        /* Memorize various observable flags */
        if do_empty_mind != 0 {
            (*r_ptr).r_flags2 |= 0x40 as libc::c_int as libc::c_uint
        }
        if do_weird_mind != 0 {
            (*r_ptr).r_flags2 |= 0x80 as libc::c_int as libc::c_uint
        }
        if do_cold_blood != 0 {
            (*r_ptr).r_flags2 |= 0x20 as libc::c_int as libc::c_uint
        }
        if do_invisible != 0 {
            (*r_ptr).r_flags2 |= 0x10 as libc::c_int as libc::c_uint
        }
    } else if (*m_ptr).ml != 0 {
        /* The monster is not visible */
        /* It was previously seen */
        /* Mark as not visible */
        (*m_ptr).ml = 0 as libc::c_int as bool_;
        /* Erase the monster */
        lite_spot(fy, fx);
        /* Update health bar as needed */
        if health_who as libc::c_int == m_idx {
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x800 as libc::c_long) as
                    u32b
        }
        /* Update monster list window */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x10 as libc::c_long) as u32b;
        /* Disturb on disappearance*/
        if disturb_move != 0 {
            if disturb_pets as libc::c_int != 0 ||
                   is_friend(m_ptr) <= 0 as libc::c_int {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
        }
    }
    /* The monster is now easily visible */
    if easy != 0 {
        if (*m_ptr).ml as libc::c_int != old_ml as libc::c_int {
            if (*r_ptr).flags2 & 0x2000 as libc::c_int as libc::c_uint != 0 {
                sanity_blast(m_ptr, 0 as libc::c_int as bool_);
            }
        }
        /* Change */
        if (*m_ptr).mflag & 0x1 as libc::c_int == 0 {
            /* Mark as easily visible */
            (*m_ptr).mflag |= 0x1 as libc::c_int;
            /* Disturb on appearance */
            if disturb_near != 0 {
                if disturb_pets as libc::c_int != 0 ||
                       is_friend(m_ptr) <= 0 as libc::c_int {
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                }
            }
        }
    } else if (*m_ptr).mflag & 0x1 as libc::c_int != 0 {
        /* The monster is not easily visible */
        /* Change */
        /* Mark as not easily visible */
        (*m_ptr).mflag &= !(0x1 as libc::c_int);
        /* Update monster list window */
        (*p_ptr).window =
            ((*p_ptr).window as libc::c_long | 0x10 as libc::c_long) as u32b;
        /* Disturb on disappearance */
        if disturb_near != 0 {
            if disturb_pets as libc::c_int != 0 ||
                   is_friend(m_ptr) <= 0 as libc::c_int {
                disturb(1 as libc::c_int, 0 as libc::c_int);
            }
        }
    };
}
/*
 * This function simply updates all the (non-dead) monsters (see above).
 */
#[no_mangle]
pub unsafe extern "C" fn update_monsters(mut full: bool_) {
    let mut i: libc::c_int = 0;
    /* Update each (live) monster */
    i = 1 as libc::c_int;
    while i < m_max as libc::c_int {
        let mut m_ptr: *mut monster_type =
            &mut *m_list.offset(i as isize) as *mut monster_type;
        /* Skip dead monsters */
        if !((*m_ptr).r_idx == 0) {
            /* Update the monster */
            update_mon(i, full);
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn monster_carry(mut m_ptr: *mut monster_type,
                                       mut m_idx: libc::c_int,
                                       mut q_ptr: *mut object_type) {
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    /* Get new object */
    let mut o_idx: libc::c_int = o_pop() as libc::c_int;
    if o_idx != 0 {
        /* Get the item */
        o_ptr = &mut *o_list.offset(o_idx as isize) as *mut object_type;
        /* Structure copy */
        object_copy(o_ptr, q_ptr);
        /* Build a stack */
        (*o_ptr).next_o_idx = (*m_ptr).hold_o_idx;
        (*o_ptr).held_m_idx = m_idx as s16b;
        (*o_ptr).ix = 0 as libc::c_int as byte_hack;
        (*o_ptr).iy = 0 as libc::c_int as byte_hack;
        (*m_ptr).hold_o_idx = o_idx as s16b
    } else if (*q_ptr).name1 != 0 {
        (*a_info.offset((*q_ptr).name1 as isize)).cur_num =
            0 as libc::c_int as byte_hack
    } else if (*k_info.offset((*q_ptr).k_idx as isize)).flags3 as libc::c_long
                  & 0x8000 as libc::c_long != 0 {
        (*k_info.offset((*q_ptr).k_idx as isize)).artifact =
            0 as libc::c_int as bool_
    } else if (*q_ptr).tval as libc::c_int == 102 as libc::c_int {
        random_artifacts[(*q_ptr).sval as usize].generated =
            0 as libc::c_int as byte_hack
    };
}
static mut possible_randart: [libc::c_int; 19] =
    [6 as libc::c_int, 15 as libc::c_int, 20 as libc::c_int,
     21 as libc::c_int, 22 as libc::c_int, 24 as libc::c_int,
     23 as libc::c_int, 30 as libc::c_int, 31 as libc::c_int,
     32 as libc::c_int, 33 as libc::c_int, 34 as libc::c_int,
     35 as libc::c_int, 36 as libc::c_int, 37 as libc::c_int,
     39 as libc::c_int, 40 as libc::c_int, 45 as libc::c_int,
     -(1 as libc::c_int)];
#[no_mangle]
pub unsafe extern "C" fn kind_is_randart(mut k_idx: libc::c_int) -> bool_ {
    let mut max: libc::c_int = 0;
    let mut k_ptr: *mut object_kind =
        &mut *k_info.offset(k_idx as isize) as *mut object_kind;
    if kind_is_legal(k_idx) == 0 { return 0 as libc::c_int as bool_ }
    max = 0 as libc::c_int;
    while possible_randart[max as usize] != -(1 as libc::c_int) {
        if (*k_ptr).tval as libc::c_int == possible_randart[max as usize] {
            return 1 as libc::c_int as bool_
        }
        max += 1
    }
    return 0 as libc::c_int as bool_;
}
/* Hack -- Preserve artifacts */
/*
 * Attempt to place a monster of the given race at the given location.
 *
 * To give the player a sporting chance, any monster that appears in
 * line-of-sight and is extremely dangerous can be marked as
 * "FORCE_SLEEP", which will cause them to be placed with low energy,
 * which often (but not always) lets the player move before they do.
 *
 * This routine refuses to place out-of-depth "FORCE_DEPTH" monsters.
 *
 * XXX XXX XXX Use special "here" and "dead" flags for unique monsters,
 * remove old "cur_num" and "max_num" fields.
 *
 * XXX XXX XXX Actually, do something similar for artifacts, to simplify
 * the "preserve" mode, and to make the "what artifacts" flag more useful.
 *
 * This is the only function which may place a monster in the dungeon,
 * except for the savefile loading code.
 */
#[no_mangle]
pub static mut bypass_r_ptr_max_num: bool_ = 0 as libc::c_int as bool_;
#[no_mangle]
pub static mut place_monster_result: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut place_monster_one_no_drop: bool_ = 0 as libc::c_int as bool_;
#[no_mangle]
pub static mut place_monster_one_race: *mut monster_race =
    0 as *const monster_race as *mut monster_race;
#[no_mangle]
pub unsafe extern "C" fn place_monster_one(mut y: libc::c_int,
                                           mut x: libc::c_int,
                                           mut r_idx: libc::c_int,
                                           mut ego: libc::c_int,
                                           mut slp: bool_,
                                           mut status: libc::c_int) -> s16b {
    let mut i: libc::c_int = 0;
    let mut dummy: [libc::c_char; 5] = [0; 5];
    let mut add_level: bool_ = 0 as libc::c_int as bool_;
    let mut min_level: libc::c_int = 0 as libc::c_int;
    let mut max_level: libc::c_int = 0 as libc::c_int;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    let mut name: cptr = r_name.offset((*r_ptr).name as isize) as cptr;
    /* Grab the special race if needed */
    if !place_monster_one_race.is_null() { r_ptr = place_monster_one_race }
    /* DO NOT PLACE A MONSTER IN THE SMALL SCALE WILDERNESS !!! */
    if (*p_ptr).wild_mode != 0 {
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* Verify location */
    if !(y > 0 as libc::c_int && x > 0 as libc::c_int &&
             y < cur_hgt as libc::c_int - 1 as libc::c_int &&
             x < cur_wid as libc::c_int - 1 as libc::c_int) {
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* Require empty space */
    if !((*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                             isize)).flags1 as libc::c_long &
             0x10 as libc::c_long != 0 &&
             (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
                 0xaf as libc::c_int &&
             (*cave[y as usize].offset(x as isize)).m_idx == 0 &&
             !(y == (*p_ptr).py as libc::c_int &&
                   x == (*p_ptr).px as libc::c_int)) {
        if wizard != 0 {
            cmsg_format(12 as libc::c_int as byte_hack,
                        b"WARNING: Refused monster(%d): EMPTY BOLD\x00" as
                            *const u8 as *const libc::c_char, r_idx);
        }
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* Require no monster free grid, or special permission */
    if (*cave[y as usize].offset(x as isize)).info as libc::c_int &
           0x800 as libc::c_int != 0 &&
           *m_allow_special.offset(r_idx as isize) == 0 {
        if wizard != 0 {
            cmsg_format(12 as libc::c_int as byte_hack,
                        b"WARNING: Refused monster(%d): CAVE_FREE\x00" as
                            *const u8 as *const libc::c_char, r_idx);
        }
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* Hack -- no creation on glyph of warding */
    if (*cave[y as usize].offset(x as isize)).feat as libc::c_int ==
           0x3 as libc::c_int {
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    if (*cave[y as usize].offset(x as isize)).feat as libc::c_int ==
           0x40 as libc::c_int {
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* Nor on the between */
    if (*cave[y as usize].offset(x as isize)).feat as libc::c_int ==
           0xa0 as libc::c_int {
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* Nor on the altars */
    if (*cave[y as usize].offset(x as isize)).feat as libc::c_int >=
           0xa1 as libc::c_int &&
           (*cave[y as usize].offset(x as isize)).feat as libc::c_int <=
               0xab as libc::c_int {
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* Nor on the Pattern */
    if (*cave[y as usize].offset(x as isize)).feat as libc::c_int >=
           0x41 as libc::c_int &&
           (*cave[y as usize].offset(x as isize)).feat as libc::c_int <=
               0x49 as libc::c_int {
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* Paranoia */
    if r_idx == 0 {
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* Paranoia */
    if (*r_ptr).name == 0 {
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* Are we allowed to continue ? */
    if process_hooks(5 as libc::c_int,
                     b"(d)\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char, r_idx) != 0 {
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* Ego Uniques are NOT to be created */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 && ego != 0 {
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* Now could we generate an Ego Monster */
	/* Grab the special race if needed */
    if !place_monster_one_race.is_null() {
        r_ptr = place_monster_one_race
    } else { r_ptr = race_info_idx(r_idx, ego) }
    if monster_can_cross_terrain((*cave[y as usize].offset(x as isize)).feat,
                                 r_ptr) == 0 {
        if wizard != 0 {
            cmsg_print(12 as libc::c_int as byte_hack,
                       b"WARNING: Refused monster: cannot cross terrain\x00"
                           as *const u8 as *const libc::c_char);
        }
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* Unallow some uniques to be generated outside of their quests/special levels/dungeons */
    if (*r_ptr).flags9 & 0x2000 as libc::c_int as libc::c_uint != 0 &&
           *m_allow_special.offset(r_idx as isize) == 0 {
        if wizard != 0 {
            cmsg_format(12 as libc::c_int as byte_hack,
                        b"WARNING: Refused monster(%d): SPECIAL_GENE\x00" as
                            *const u8 as *const libc::c_char, r_idx);
        }
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* Disallow Spirits in The Void, now this *IS* an ugly hack, I hate to do it ... */
    if (*r_ptr).flags7 & 0x80000 as libc::c_int as libc::c_uint != 0 &&
           dungeon_type as libc::c_int != 11 as libc::c_int {
        if wizard != 0 {
            cmsg_format(12 as libc::c_int as byte_hack,
                        b"WARNING: Refused monster(%d): SPIRIT in non VOID\x00"
                            as *const u8 as *const libc::c_char, r_idx);
        }
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* Fully forbid it */
    if (*r_ptr).flags9 & 0x4000 as libc::c_int as libc::c_uint != 0 {
        if wizard != 0 {
            cmsg_print(12 as libc::c_int as byte_hack,
                       b"WARNING: Refused monster: NEVER_GENE\x00" as
                           *const u8 as *const libc::c_char);
        }
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* Hack -- "unique" monsters must be "unique" */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 &&
           (*r_ptr).max_num as libc::c_int == -(1 as libc::c_int) &&
           *m_allow_special.offset(r_idx as isize) == 0 {
        /* Cannot create */
        if wizard != 0 {
            cmsg_format(12 as libc::c_int as byte_hack,
                        b"WARNING: Refused monster %d: unique not unique\x00"
                            as *const u8 as *const libc::c_char, r_idx);
        }
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* The monster is already on an unique level */
    if (*r_ptr).on_saved != 0 {
        if wizard != 0 {
            cmsg_print(12 as libc::c_int as byte_hack,
                       b"WARNING: Refused monster: unique already on saved level\x00"
                           as *const u8 as *const libc::c_char);
        }
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* Hack -- "unique" monsters must be "unique" */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 &&
           (*r_ptr).cur_num as libc::c_int >= (*r_ptr).max_num as libc::c_int
           && (*r_ptr).max_num as libc::c_int != -(1 as libc::c_int) &&
           bypass_r_ptr_max_num == 0 {
        /* Cannot create */
        if wizard != 0 {
            cmsg_format(12 as libc::c_int as byte_hack,
                        b"WARNING: Refused monster %d: cur_num >= max_num\x00"
                            as *const u8 as *const libc::c_char, r_idx);
        }
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* Depth monsters may NOT be created out of depth */
    if (*r_ptr).flags1 & 0x100 as libc::c_int as libc::c_uint != 0 &&
           (dun_level as libc::c_int) < (*r_ptr).level as libc::c_int {
        /* Cannot create */
        if wizard != 0 {
            cmsg_print(12 as libc::c_int as byte_hack,
                       b"WARNING: FORCE_DEPTH\x00" as *const u8 as
                           *const libc::c_char);
        }
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* Powerful monster */
    if (*r_ptr).level as libc::c_int > dun_level as libc::c_int {
        /* Unique monsters */
        if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
            /* Message for cheaters */
            if cheat_hear as libc::c_int != 0 ||
                   (*p_ptr).precognition as libc::c_int != 0 {
                msg_format(b"Deep Unique (%s).\x00" as *const u8 as
                               *const libc::c_char, name);
            }
            /* Boost rating by twice delta-depth */
            rating =
                (rating as libc::c_int +
                     ((*r_ptr).level as libc::c_int -
                          dun_level as libc::c_int) * 2 as libc::c_int) as
                    s16b
        } else {
            /* Normal monsters */
            /* Message for cheaters */
            if cheat_hear as libc::c_int != 0 ||
                   (*p_ptr).precognition as libc::c_int != 0 {
                msg_format(b"Deep Monster (%s).\x00" as *const u8 as
                               *const libc::c_char, name);
            }
            rating =
                (rating as libc::c_int +
                     ((*r_ptr).level as libc::c_int -
                          dun_level as libc::c_int)) as s16b
        }
    } else if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        /* Boost rating by delta-depth */
        /* Note the monster */
        /* Unique monsters induce message */
        if cheat_hear as libc::c_int != 0 ||
               (*p_ptr).precognition as libc::c_int != 0 {
            msg_format(b"Unique (%s).\x00" as *const u8 as
                           *const libc::c_char, name);
        }
    }
    /* Access the location */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Make a new monster */
    (*c_ptr).m_idx = m_pop();
    hack_m_idx_ii = (*c_ptr).m_idx;
    /* Mega-Hack -- catch "failure" */
    if (*c_ptr).m_idx == 0 {
        if !place_monster_one_race.is_null() {
            place_monster_one_race =
                rnfree(place_monster_one_race as vptr,
                       ::std::mem::size_of::<monster_race>() as libc::c_ulong)
                    as *mut monster_race
        }
        return 0 as libc::c_int as s16b
    }
    /* Get a new monster record */
    m_ptr = &mut *m_list.offset((*c_ptr).m_idx as isize) as *mut monster_type;
    /* Save the race */
    (*m_ptr).r_idx = r_idx as s16b;
    (*m_ptr).ego = ego as u16b;
    /* No special, no mind */
    (*m_ptr).sr_ptr = place_monster_one_race;
    (*m_ptr).mind = 0 as *mut monster_mind;
    /* Place the monster at the location */
    (*m_ptr).fy = y as byte_hack;
    (*m_ptr).fx = x as byte_hack;
    /* No "damage" yet */
    (*m_ptr).stunned = 0 as libc::c_int as byte_hack;
    (*m_ptr).confused = 0 as libc::c_int as byte_hack;
    (*m_ptr).monfear = 0 as libc::c_int as byte_hack;
    /* No target yet */
    (*m_ptr).target = -(1 as libc::c_int) as s16b;
    /* Unknown distance */
    (*m_ptr).cdis = 0 as libc::c_int as byte_hack;
    /* No flags */
    (*m_ptr).mflag = 0 as libc::c_int;
    /* Not visible */
    (*m_ptr).ml = 0 as libc::c_int as bool_;
    /* No objects yet */
    (*m_ptr).hold_o_idx = 0 as libc::c_int as s16b;
    (*m_ptr).status = status as s16b;
    /* Friendly? */
    if ((*m_ptr).status as libc::c_int) < 2 as libc::c_int &&
           (*r_ptr).flags7 & 0x10 as libc::c_int as libc::c_uint != 0 {
        (*m_ptr).status = 2 as libc::c_int as s16b
    }
    if ((*m_ptr).status as libc::c_int) < 0 as libc::c_int &&
           (*r_ptr).flags7 & 0x4000 as libc::c_int as libc::c_uint != 0 {
        (*m_ptr).status = 0 as libc::c_int as s16b
    }
    /* Assume no sleeping */
    (*m_ptr).csleep = 0 as libc::c_int as s16b;
    /* Enforce sleeping if needed */
    if slp as libc::c_int != 0 && (*r_ptr).sleep as libc::c_int != 0 {
        let mut val: libc::c_int = (*r_ptr).sleep as libc::c_int;
        (*m_ptr).csleep =
            (val * 2 as libc::c_int +
                 (Rand_div(val * 10 as libc::c_int) + 1 as libc::c_int)) as
                s16b
    }
    /* Generate the monster's inventory(if any) */
	/* Only if not fated to die */
    if dungeon_type as libc::c_int != DUNGEON_DEATH &&
           place_monster_one_no_drop == 0 {
        let mut good: bool_ =
            if (*r_ptr).flags1 & 0x10000000 as libc::c_int as libc::c_uint !=
                   0 {
                1 as libc::c_int
            } else { 0 as libc::c_int } as bool_;
        let mut great: bool_ =
            if (*r_ptr).flags1 & 0x20000000 as libc::c_int as libc::c_uint !=
                   0 {
                1 as libc::c_int
            } else { 0 as libc::c_int } as bool_;
        let mut do_gold: bool_ =
            ((*r_ptr).flags1 & 0x200000 as libc::c_int as libc::c_uint == 0)
                as libc::c_int as bool_;
        let mut do_item: bool_ =
            ((*r_ptr).flags1 & 0x100000 as libc::c_int as libc::c_uint == 0)
                as libc::c_int as bool_;
        let mut do_mimic: bool_ =
            ((*r_ptr).flags9 & 0x8 as libc::c_int as libc::c_uint) as bool_;
        let mut j: libc::c_int = 0;
        let mut force_coin: libc::c_int = get_coin_type(r_ptr);
        let mut dump_item: libc::c_int = 0 as libc::c_int;
        let mut dump_gold: libc::c_int = 0 as libc::c_int;
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
        let mut number: libc::c_int = 0 as libc::c_int;
        /* Average dungeon and monster levels */
        object_level =
            ((dun_level as libc::c_int + (*r_ptr).level as libc::c_int) /
                 2 as libc::c_int) as s16b;
        /* Determine how much we can drop */
        if (*r_ptr).flags1 & 0x400000 as libc::c_int as libc::c_uint != 0 &&
               Rand_div(100 as libc::c_int) < 60 as libc::c_int {
            number += 1
        }
        if (*r_ptr).flags1 & 0x800000 as libc::c_int as libc::c_uint != 0 &&
               Rand_div(100 as libc::c_int) < 90 as libc::c_int {
            number += 1
        }
        if (*r_ptr).flags1 & 0x1000000 as libc::c_int as libc::c_uint != 0 {
            number +=
                damroll(1 as libc::c_int as s16b, 2 as libc::c_int as s16b)
        }
        if (*r_ptr).flags1 & 0x2000000 as libc::c_int as libc::c_uint != 0 {
            number +=
                damroll(2 as libc::c_int as s16b, 2 as libc::c_int as s16b)
        }
        if (*r_ptr).flags1 & 0x4000000 as libc::c_int as libc::c_uint != 0 {
            number +=
                damroll(3 as libc::c_int as s16b, 2 as libc::c_int as s16b)
        }
        if (*r_ptr).flags1 & 0x8000000 as libc::c_int as libc::c_uint != 0 {
            number +=
                damroll(4 as libc::c_int as s16b, 2 as libc::c_int as s16b)
        }
        if (*r_ptr).flags9 & 0x8 as libc::c_int as libc::c_uint != 0 {
            number = 1 as libc::c_int
        }
        /* Hack -- handle creeping coins */
        coin_type = force_coin as s16b;
        if (*r_ptr).flags7 & 0x10000 as libc::c_int as libc::c_uint != 0 {
            let mut tries: libc::c_int = 1000 as libc::c_int;
            let mut theme: obj_theme =
                obj_theme{treasure: 0, combat: 0, magic: 0, tools: 0,};
            let mut i_0: libc::c_int = 0;
            /* Get local object */
            q_ptr = &mut forge;
            theme.treasure = 101 as libc::c_int as byte_hack;
            theme.combat = 101 as libc::c_int as byte_hack;
            theme.magic = 101 as libc::c_int as byte_hack;
            theme.tools = 101 as libc::c_int as byte_hack;
            init_match_theme(theme);
            /* Apply restriction */
            get_obj_num_hook =
                Some(kind_is_legal as
                         unsafe extern "C" fn(_: libc::c_int) -> bool_);
            /* Rebuild allocation table */
            get_obj_num_prep();
            i_0 = 0 as libc::c_int;
            while tries != 0 {
                tries -= 1;
                i_0 = get_obj_num(dun_level as libc::c_int) as libc::c_int;
                if i_0 == 0 { continue ; }
                if !(kind_is_randart(i_0) == 0) { break ; }
            }
            /* Invalidate the cached allocation table */
            alloc_kind_table_valid = 0 as libc::c_int as bool_;
            if tries != 0 {
                object_prep(q_ptr, i_0);
                create_artifact(q_ptr, 0 as libc::c_int as bool_,
                                1 as libc::c_int as bool_);
                (*q_ptr).found = 1 as libc::c_int as byte_hack;
                (*q_ptr).found_aux1 = (*m_ptr).r_idx;
                (*q_ptr).found_aux2 = (*m_ptr).ego as s16b;
                (*q_ptr).found_aux3 = dungeon_type as s16b;
                (*q_ptr).found_aux4 =
                    if dungeon_type as libc::c_int == 0 as libc::c_int {
                        (*(*wild_map.offset((*p_ptr).wilderness_y as
                                                isize)).offset((*p_ptr).wilderness_x
                                                                   as
                                                                   isize)).feat
                    } else { dun_level as libc::c_int } as s16b;
                monster_carry(m_ptr, (*c_ptr).m_idx as libc::c_int, q_ptr);
            }
        }
        let mut current_block_234: u64;
        /* Drop some objects */
        j = 0 as libc::c_int;
        while j < number {
            /* Get local object */
            q_ptr = &mut forge;
            /* Wipe the object */
            object_wipe(q_ptr);
            /* Make Gold */
            if do_mimic == 0 && do_gold as libc::c_int != 0 &&
                   (do_item == 0 ||
                        Rand_div(100 as libc::c_int) < 50 as libc::c_int) {
                /* Make some gold */
                if make_gold(q_ptr) == 0 {
                    current_block_234 = 11978944022089568707;
                } else {
                    /* XXX XXX XXX */
                    dump_gold += 1;
                    current_block_234 = 4402115142504265260;
                }
            } else {
                /* Make Object */
                /* Make an object */
                if do_mimic == 0 {
                    if make_object(q_ptr, good, great, (*r_ptr).drops) == 0 {
                        current_block_234 = 11978944022089568707;
                    } else { current_block_234 = 10644040035716118461; }
                } else {
                    /* Try hard for mimics */
                    let mut tries_0: libc::c_int = 1000 as libc::c_int;
                    loop  {
                        let fresh4 = tries_0;
                        tries_0 = tries_0 - 1;
                        if !(fresh4 != 0) { break ; }
                        if make_object(q_ptr, good, great, (*r_ptr).drops) !=
                               0 {
                            break ;
                        }
                    }
                    /* BAD */
                    if tries_0 == 0 {
                        current_block_234 = 11978944022089568707;
                    } else { current_block_234 = 10644040035716118461; }
                }
                match current_block_234 {
                    11978944022089568707 => { }
                    _ => {
                        /* XXX XXX XXX */
                        dump_item += 1;
                        current_block_234 = 4402115142504265260;
                    }
                }
            }
            match current_block_234 {
                4402115142504265260 => {
                    (*q_ptr).found = 1 as libc::c_int as byte_hack;
                    (*q_ptr).found_aux1 = (*m_ptr).r_idx;
                    (*q_ptr).found_aux2 = (*m_ptr).ego as s16b;
                    (*q_ptr).found_aux3 = dungeon_type as s16b;
                    (*q_ptr).found_aux4 =
                        if dungeon_type as libc::c_int == 0 as libc::c_int {
                            (*(*wild_map.offset((*p_ptr).wilderness_y as
                                                    isize)).offset((*p_ptr).wilderness_x
                                                                       as
                                                                       isize)).feat
                        } else { dun_level as libc::c_int } as s16b;
                    monster_carry(m_ptr, (*c_ptr).m_idx as libc::c_int,
                                  q_ptr);
                }
                _ => { }
            }
            j += 1
        }
        /* Reset the object level */
        object_level = dun_level;
        /* Reset "coin" type */
        coin_type = 0 as libc::c_int as s16b
    }
    place_monster_one_no_drop = 0 as libc::c_int as bool_;
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
    /* Hack -- small racial variety */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint == 0 {
        /* Allow some small variation per monster */
        i =
            extract_energy[(*m_ptr).speed as usize] as libc::c_int /
                10 as libc::c_int;
        if i != 0 {
            (*m_ptr).mspeed =
                ((*m_ptr).mspeed as libc::c_int +
                     (0 as libc::c_int + Rand_div(1 as libc::c_int + i + i) -
                          i)) as byte_hack
        }
    }
    if dungeon_flags2 as libc::c_long & 0x1 as libc::c_long != 0 {
        max_level = dun_level as libc::c_int / 2 as libc::c_int;
        min_level = max_level;
        add_level = 1 as libc::c_int as bool_
    }
    if dungeon_flags1 as libc::c_long & 0x10000000 as libc::c_long != 0 {
        if min_level == 0 { min_level = dun_level as libc::c_int }
        max_level = dun_level as libc::c_int;
        add_level = 1 as libc::c_int as bool_
    }
    if dungeon_flags1 as libc::c_long & 0x20000000 as libc::c_long != 0 {
        if min_level == 0 {
            min_level = dun_level as libc::c_int * 2 as libc::c_int
        }
        max_level = dun_level as libc::c_int * 2 as libc::c_int;
        add_level = 1 as libc::c_int as bool_
    }
    if add_level != 0 {
        monster_set_level((*c_ptr).m_idx as libc::c_int,
                          min_level +
                              Rand_div(1 as libc::c_int + max_level -
                                           min_level));
    }
    /* Give a random starting energy */
    (*m_ptr).energy = Rand_div(100 as libc::c_int) as byte_hack;
    /* Force monster to wait for player */
    if (*r_ptr).flags1 & 0x400 as libc::c_int as libc::c_uint != 0 {
        /* Monster is still being nice */
        (*m_ptr).mflag |= 0x20 as libc::c_int;
        /* Must repair monsters */
        repair_monsters = 1 as libc::c_int as bool_
    }
    /* Hack -- see "process_monsters()" */
    if ((*c_ptr).m_idx as libc::c_int) < hack_m_idx as libc::c_int {
        /* Monster is still being born */
        (*m_ptr).mflag |= 0x10 as libc::c_int
    }
    /* Update the monster */
    update_mon((*c_ptr).m_idx as libc::c_int, 1 as libc::c_int as bool_);
    /* Hack -- Count the number of "reproducers" */
    if (*r_ptr).flags4 & 0x2 as libc::c_int as libc::c_uint != 0 {
        num_repro += 1
    }
    /* Hack -- Notice new multi-hued monsters */
    if (*r_ptr).flags1 & 0x80 as libc::c_int as libc::c_uint != 0 {
        shimmer_monsters = 1 as libc::c_int as bool_
    }
    /* Hack -- we need to modify the REAL r_info, not the fake one */
    r_ptr = &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    /* Hack -- Count the monsters on the level */
    (*r_ptr).cur_num = (*r_ptr).cur_num.wrapping_add(1);
    /* Unique monsters on saved levels should be "marked" */
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 &&
           get_dungeon_save(dummy.as_mut_ptr()) as libc::c_int != 0 {
        (*r_ptr).on_saved = 1 as libc::c_int as bool_
    }
    place_monster_one_race = 0 as *mut monster_race;
    /* Success */
    place_monster_result = (*c_ptr).m_idx as libc::c_int;
    return (*c_ptr).m_idx;
}
/*
 * Attempt to place a "group" of monsters around the given location
 */
unsafe extern "C" fn place_monster_group(mut y: libc::c_int,
                                         mut x: libc::c_int,
                                         mut r_idx: libc::c_int,
                                         mut slp: bool_,
                                         mut status: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    let mut old: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut total: libc::c_int = 0 as libc::c_int;
    let mut extra: libc::c_int = 0 as libc::c_int;
    let mut hack_n: libc::c_int = 0 as libc::c_int;
    let mut hack_y: [byte_hack; 32] = [0; 32];
    let mut hack_x: [byte_hack; 32] = [0; 32];
    /* Pick a group size */
    total = Rand_div(13 as libc::c_int) + 1 as libc::c_int;
    /* Hard monsters, small groups */
    if (*r_ptr).level as libc::c_int > dun_level as libc::c_int {
        extra = (*r_ptr).level as libc::c_int - dun_level as libc::c_int;
        extra = 0 as libc::c_int - (Rand_div(extra) + 1 as libc::c_int)
    } else if ((*r_ptr).level as libc::c_int) < dun_level as libc::c_int {
        extra = dun_level as libc::c_int - (*r_ptr).level as libc::c_int;
        extra = Rand_div(extra) + 1 as libc::c_int
    }
    /* Easy monsters, large groups */
    /* Hack -- limit group reduction */
    if extra > 12 as libc::c_int { extra = 12 as libc::c_int }
    /* Modify the group size */
    total += extra;
    /* Minimum size */
    if total < 1 as libc::c_int { total = 1 as libc::c_int }
    /* Maximum size */
    if total > 32 as libc::c_int { total = 32 as libc::c_int }
    /* Save the rating */
    old = rating as libc::c_int;
    /* Start on the monster */
    hack_n = 1 as libc::c_int;
    hack_x[0 as libc::c_int as usize] = x as byte_hack;
    hack_y[0 as libc::c_int as usize] = y as byte_hack;
    /* Puddle monsters, breadth first, up to total */
    n = 0 as libc::c_int;
    while n < hack_n && hack_n < total {
        /* Grab the location */
        let mut hx: libc::c_int = hack_x[n as usize] as libc::c_int;
        let mut hy: libc::c_int = hack_y[n as usize] as libc::c_int;
        /* Check each direction, up to total */
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int && hack_n < total {
            let mut mx: libc::c_int = hx + ddx_ddd[i as usize] as libc::c_int;
            let mut my: libc::c_int = hy + ddy_ddd[i as usize] as libc::c_int;
            /* Walls and Monsters block flow */
            if (*f_info.offset((*cave[my as usize].offset(mx as isize)).feat
                                   as isize)).flags1 as libc::c_long &
                   0x10 as libc::c_long != 0 &&
                   (*cave[my as usize].offset(mx as isize)).feat as
                       libc::c_int != 0xaf as libc::c_int &&
                   (*cave[my as usize].offset(mx as isize)).m_idx == 0 &&
                   !(my == (*p_ptr).py as libc::c_int &&
                         mx == (*p_ptr).px as libc::c_int) {
                /* Attempt to place another monster */
                if place_monster_one(my, mx, r_idx, pick_ego_monster(r_idx),
                                     slp, status) != 0 {
                    /* Add it to the "hack" set */
                    hack_y[hack_n as usize] = my as byte_hack;
                    hack_x[hack_n as usize] = mx as byte_hack;
                    hack_n += 1
                }
            }
            i += 1
        }
        n += 1
    }
    /* Hack -- restore the rating */
    rating = old as s16b;
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
 * Hack -- help pick an escort type
 */
static mut place_monster_idx: libc::c_int = 0 as libc::c_int;
/*
 * Hack -- help pick an escort type
 */
unsafe extern "C" fn place_monster_okay(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(place_monster_idx as isize) as *mut monster_race;
    let mut z_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    /* Hack - Escorts have to have the same dungeon flag */
    if monster_dungeon(place_monster_idx) as libc::c_int !=
           monster_dungeon(r_idx) as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Require similar "race" */
    if (*z_ptr).d_char as libc::c_int != (*r_ptr).d_char as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Skip more advanced monsters */
    if (*z_ptr).level as libc::c_int > (*r_ptr).level as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Skip unique monsters */
    if (*z_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int as bool_
    }
    /* Paranoia -- Skip identical monsters */
    if place_monster_idx == r_idx { return 0 as libc::c_int as bool_ }
    /* Okay */
    return 1 as libc::c_int as bool_;
}
/*
 * Attempt to place a monster of the given race at the given location
 *
 * Note that certain monsters are now marked as requiring "friends".
 * These monsters, if successfully placed, and if the "grp" parameter
 * is TRUE, will be surrounded by a "group" of identical monsters.
 *
 * Note that certain monsters are now marked as requiring an "escort",
 * which is a collection of monsters with similar "race" but lower level.
 *
 * Some monsters induce a fake "group" flag on their escorts.
 *
 * Note the "bizarre" use of non-recursion to prevent annoying output
 * when running a code profiler.
 *
 * Note the use of the new "monster allocation table" code to restrict
 * the "get_mon_num()" function to "legal" escort types.
 */
#[no_mangle]
pub unsafe extern "C" fn place_monster_aux(mut y: libc::c_int,
                                           mut x: libc::c_int,
                                           mut r_idx: libc::c_int,
                                           mut slp: bool_, mut grp: bool_,
                                           mut status: libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    let mut old_get_mon_num_hook:
            Option<unsafe extern "C" fn(_: libc::c_int) -> bool_> = None;
    /* Place one monster, or fail */
    if place_monster_one(y, x, r_idx, pick_ego_monster(r_idx), slp, status) ==
           0 {
        return 0 as libc::c_int as bool_
    }
    /* Require the "group" flag */
    if grp == 0 { return 1 as libc::c_int as bool_ }
    /* Friends for certain monsters */
    if (*r_ptr).flags1 & 0x2000 as libc::c_int as libc::c_uint != 0 {
        /* Attempt to place a group */
        place_monster_group(y, x, r_idx, slp, status);
    }
    /* Escorts for certain monsters */
    if (*r_ptr).flags1 & 0x4000 as libc::c_int as libc::c_uint != 0 {
        old_get_mon_num_hook = get_mon_num_hook;
        /* Set the escort index */
        place_monster_idx = r_idx;
        /* Set the escort hook */
        get_mon_num_hook =
            Some(place_monster_okay as
                     unsafe extern "C" fn(_: libc::c_int) -> bool_);
        /* Prepare allocation table */
        get_mon_num_prep();
        /* Try to place several "escorts" */
        i = 0 as libc::c_int;
        while i < 50 as libc::c_int {
            let mut nx: libc::c_int = 0;
            let mut ny: libc::c_int = 0;
            let mut z: libc::c_int = 0;
            let mut d: libc::c_int = 3 as libc::c_int;
            /* Pick a location */
            scatter(&mut ny, &mut nx, y, x, d);
            /* Require empty grids */
            if (*f_info.offset((*cave[ny as usize].offset(nx as isize)).feat
                                   as isize)).flags1 as libc::c_long &
                   0x10 as libc::c_long != 0 &&
                   (*cave[ny as usize].offset(nx as isize)).feat as
                       libc::c_int != 0xaf as libc::c_int &&
                   (*cave[ny as usize].offset(nx as isize)).m_idx == 0 &&
                   !(ny == (*p_ptr).py as libc::c_int &&
                         nx == (*p_ptr).px as libc::c_int) {
                set_mon_num2_hook(ny, nx);
                /* Prepare allocation table */
                get_mon_num_prep();
                /* Pick a random race */
                z = get_mon_num((*r_ptr).level as libc::c_int) as libc::c_int;
                /* Reset restriction */
                get_mon_num2_hook = None;
                /* Prepare allocation table */
                get_mon_num_prep();
                /* Handle failure */
                if z == 0 { break ; }
                /* Place a single escort */
                place_monster_one(ny, nx, z, pick_ego_monster(z), slp,
                                  status);
                /* Place a "group" of escorts if needed */
                if (*r_info.offset(z as isize)).flags1 &
                       0x2000 as libc::c_int as libc::c_uint != 0 ||
                       (*r_ptr).flags1 & 0x8000 as libc::c_int as libc::c_uint
                           != 0 {
                    /* Place a group of monsters */
                    place_monster_group(ny, nx, z, slp, status);
                }
            }
            i += 1
        }
        /* Reset restriction */
        get_mon_num_hook = old_get_mon_num_hook;
        /* Prepare allocation table */
        get_mon_num_prep();
    }
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
 * Hack -- attempt to place a monster at the given location
 *
 * Attempt to find a monster appropriate to the "monster_level"
 */
#[no_mangle]
pub unsafe extern "C" fn place_monster(mut y: libc::c_int, mut x: libc::c_int,
                                       mut slp: bool_, mut grp: bool_)
 -> bool_ {
    let mut r_idx: libc::c_int = 0;
    /* Set monster restriction */
    set_mon_num2_hook(y, x);
    /* Prepare allocation table */
    get_mon_num_prep();
    /* Pick a monster */
    r_idx = get_mon_num(monster_level as libc::c_int) as libc::c_int;
    /* Reset restriction */
    get_mon_num2_hook = None;
    /* Prepare allocation table */
    get_mon_num_prep();
    /* Handle failure */
    if r_idx == 0 { return 0 as libc::c_int as bool_ }
    /* Attempt to place the monster */
    if place_monster_aux(y, x, r_idx, slp, grp, -(2 as libc::c_int)) != 0 {
        return 1 as libc::c_int as bool_
    }
    /* Oops */
    return 0 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn alloc_horde(mut y: libc::c_int, mut x: libc::c_int)
 -> bool_ {
    let mut r_idx: libc::c_int = 0 as libc::c_int;
    let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    let mut attempts: libc::c_int = 1000 as libc::c_int;
    set_mon_num2_hook(y, x);
    /* Prepare allocation table */
    get_mon_num_prep();
    loop  {
        attempts -= 1;
        if !(attempts != 0) { break ; }
        /* Pick a monster */
        r_idx = get_mon_num(monster_level as libc::c_int) as libc::c_int;
        /* Handle failure */
        if r_idx == 0 { return 0 as libc::c_int as bool_ }
        r_ptr = &mut *r_info.offset(r_idx as isize) as *mut monster_race;
        if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint == 0 &&
               (*r_ptr).flags1 & 0x8000 as libc::c_int as libc::c_uint == 0 {
            break ;
        }
    }
    get_mon_num2_hook = None;
    /* Prepare allocation table */
    get_mon_num_prep();
    if attempts < 1 as libc::c_int { return 0 as libc::c_int as bool_ }
    attempts = 1000 as libc::c_int;
    loop  {
        attempts -= 1;
        if !(attempts != 0) { break ; }
        /* Attempt to place the monster */
        if place_monster_aux(y, x, r_idx, 0 as libc::c_int as bool_,
                             0 as libc::c_int as bool_, -(2 as libc::c_int))
               != 0 {
            break ;
        }
    }
    if attempts < 1 as libc::c_int { return 0 as libc::c_int as bool_ }
    m_ptr = &mut *m_list.offset(hack_m_idx_ii as isize) as *mut monster_type;
    summon_kin_type = (*r_ptr).d_char;
    attempts =
        Rand_div(10 as libc::c_int) + 1 as libc::c_int + 5 as libc::c_int;
    while attempts != 0 {
        summon_specific((*m_ptr).fy as libc::c_int,
                        (*m_ptr).fx as libc::c_int, dun_level as libc::c_int,
                        40 as libc::c_int);
        attempts -= 1
    }
    return 1 as libc::c_int as bool_;
}
/* MONSTER_HORDES */
/*
 * Attempt to allocate a random monster in the dungeon.
 *
 * Place the monster at least "dis" distance from the player.
 *
 * Use "slp" to choose the initial "sleep" status
 *
 * Use "monster_level" for the monster level
 */
#[no_mangle]
pub unsafe extern "C" fn alloc_monster(mut dis: libc::c_int, mut slp: bool_)
 -> bool_ {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut attempts_left: libc::c_int = 10000 as libc::c_int;
    loop 
         /* Find a legal, distant, unoccupied, space */
         {
        let fresh5 = attempts_left;
        attempts_left = attempts_left - 1;
        if !(fresh5 != 0) { break ; }
        /* Pick a location */
        y = Rand_div(cur_hgt as s32b);
        x = Rand_div(cur_wid as s32b);
        /* Require empty floor grid (was "naked") */
        if !((*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                 isize)).flags1 as libc::c_long &
                 0x10 as libc::c_long != 0 &&
                 (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
                     0xaf as libc::c_int &&
                 (*cave[y as usize].offset(x as isize)).m_idx == 0 &&
                 !(y == (*p_ptr).py as libc::c_int &&
                       x == (*p_ptr).px as libc::c_int)) {
            continue ;
        }
        /* Accept far away grids */
        if distance(y, x, (*p_ptr).py as libc::c_int,
                    (*p_ptr).px as libc::c_int) > dis {
            break ;
        }
    }
    if attempts_left == 0 {
        if cheat_xtra as libc::c_int != 0 || cheat_hear as libc::c_int != 0 {
            msg_print(b"Warning! Could not allocate a new monster. Small level?\x00"
                          as *const u8 as *const libc::c_char);
        }
        return 0 as libc::c_int as bool_
    }
    if Rand_div(5000 as libc::c_int) + 1 as libc::c_int <=
           dun_level as libc::c_int {
        if alloc_horde(y, x) != 0 {
            if cheat_hear as libc::c_int != 0 ||
                   (*p_ptr).precognition as libc::c_int != 0 {
                msg_print(b"Monster horde.\x00" as *const u8 as
                              *const libc::c_char);
            }
            return 1 as libc::c_int as bool_
        }
    } else if place_monster(y, x, slp, 1 as libc::c_int as bool_) != 0 {
        return 1 as libc::c_int as bool_
    }
    /* MONSTER_HORDES */
    /* Attempt to place the monster, allow groups */
    /* MONSTER_HORDES */
    /* Nope */
    return 0 as libc::c_int as bool_;
}
/*
 * Hack -- the "type" of the current "summon specific"
 */
static mut summon_specific_type: libc::c_int = 0 as libc::c_int;
/*
 * Hack -- help decide if a monster race is "okay" to summon
 */
#[no_mangle]
pub unsafe extern "C" fn summon_specific_okay(mut r_idx: libc::c_int)
 -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    /* Hack - Only summon dungeon monsters */
    if monster_dungeon(r_idx) == 0 { return 0 as libc::c_int as bool_ }
    /* Hack -- no specific type specified */
    if summon_specific_type == 0 { return 1 as libc::c_int as bool_ }
    /* Check our requirements */
    match summon_specific_type {
        11 => {
            okay =
                ((*r_ptr).d_char as libc::c_int == 'a' as i32 &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        12 => {
            okay =
                ((*r_ptr).d_char as libc::c_int == 'S' as i32 &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        13 => {
            okay =
                (((*r_ptr).d_char as libc::c_int == 'C' as i32 ||
                      (*r_ptr).d_char as libc::c_int == 'Z' as i32) &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        14 => {
            okay =
                ((*r_ptr).d_char as libc::c_int == 'M' as i32 &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        15 => {
            okay =
                ((*r_ptr).d_char as libc::c_int == 'A' as i32 &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        16 => {
            okay =
                ((*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0 &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        17 => {
            okay =
                ((*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0 &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        18 => {
            okay =
                ((*r_ptr).flags3 & 0x8 as libc::c_int as libc::c_uint != 0 &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        21 => {
            okay =
                ((*r_ptr).d_char as libc::c_int == 'L' as i32 ||
                     (*r_ptr).d_char as libc::c_int == 'V' as i32 ||
                     (*r_ptr).d_char as libc::c_int == 'W' as i32) as
                    libc::c_int as bool_
        }
        22 => {
            okay =
                ((*r_ptr).d_char as libc::c_int == 'D' as i32) as libc::c_int
                    as bool_
        }
        31 => {
            okay =
                ((*r_ptr).d_char as libc::c_int == 'W' as i32) as libc::c_int
                    as bool_
        }
        56 => {
            okay =
                ((*r_ptr).d_char as libc::c_int == 'G' as i32) as libc::c_int
                    as bool_
        }
        32 => {
            okay =
                if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
                    1 as libc::c_int
                } else { 0 as libc::c_int } as bool_
        }
        33 => {
            okay =
                ((*r_ptr).d_char as libc::c_int == 'm' as i32 &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        34 => {
            okay =
                ((*r_ptr).d_char as libc::c_int == 'b' as i32 &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        35 => {
            okay =
                ((*r_ptr).d_char as libc::c_int == 'Q' as i32 &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        36 => {
            okay =
                ((*r_ptr).d_char as libc::c_int == 'v' as i32 &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        37 => {
            okay =
                ((*r_ptr).d_char as libc::c_int == '$' as i32 &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        38 => {
            okay =
                (((*r_ptr).d_char as libc::c_int == '!' as i32 ||
                      (*r_ptr).d_char as libc::c_int == '?' as i32 ||
                      (*r_ptr).d_char as libc::c_int == '=' as i32 ||
                      (*r_ptr).d_char as libc::c_int == '$' as i32 ||
                      (*r_ptr).d_char as libc::c_int == '|' as i32) &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        39 => {
            okay =
                ((*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0 &&
                     (*r_ptr).d_char as libc::c_int == 'U' as i32 &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        40 => {
            okay =
                ((*r_ptr).d_char as libc::c_int ==
                     summon_kin_type as libc::c_int &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        41 => {
            okay =
                (!strstr(r_name.offset((*r_ptr).name as isize),
                         b"the Dawn\x00" as *const u8 as
                             *const libc::c_char).is_null() &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        42 => {
            okay =
                ((*r_ptr).flags3 & 0x80 as libc::c_int as libc::c_uint != 0 &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        43 => {
            okay =
                ((*r_ptr).flags3 & 0x80 as libc::c_int as libc::c_uint != 0 &&
                     !strchr(b"abcflqrwBCIJKMRS\x00" as *const u8 as
                                 *const libc::c_char,
                             (*r_ptr).d_char as libc::c_int).is_null() &&
                     (*r_ptr).flags3 & 0x8 as libc::c_int as libc::c_uint == 0
                     &&
                     (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint ==
                         0 &&
                     (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint ==
                         0 &&
                     (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint ==
                         0 &&
                     !((*r_ptr).flags4 != 0 || (*r_ptr).flags5 != 0 ||
                           (*r_ptr).flags6 != 0) &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        44 => {
            okay =
                (((*r_ptr).d_char as libc::c_int == 'L' as i32 ||
                      (*r_ptr).d_char as libc::c_int == 'V' as i32 ||
                      (*r_ptr).d_char as libc::c_int == 'W' as i32) &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        45 => {
            okay =
                ((*r_ptr).d_char as libc::c_int == 'D' as i32 &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        46 => {
            okay =
                ((*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint == 0) as
                    libc::c_int as bool_
        }
        47 => {
            okay =
                (!strstr(r_name.offset((*r_ptr).name as isize),
                         b"Phantom\x00" as *const u8 as
                             *const libc::c_char).is_null() &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        48 => {
            okay =
                (!strstr(r_name.offset((*r_ptr).name as isize),
                         b"lemental\x00" as *const u8 as
                             *const libc::c_char).is_null() &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        49 => {
            okay =
                if (*r_ptr).flags3 & 0x100 as libc::c_int as libc::c_uint != 0
                   {
                    1 as libc::c_int
                } else { 0 as libc::c_int } as bool_
        }
        50 => {
            okay =
                (!strstr(r_name.offset((*r_ptr).name as isize),
                         b"lue horror\x00" as *const u8 as
                             *const libc::c_char).is_null() &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        51 => {
            okay =
                (!strstr(r_name.offset((*r_ptr).name as isize),
                         b"Software bug\x00" as *const u8 as
                             *const libc::c_char).is_null() &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        52 => {
            okay =
                (!strstr(r_name.offset((*r_ptr).name as isize),
                         b"Random Number Generator\x00" as *const u8 as
                             *const libc::c_char).is_null() &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        53 => {
            okay =
                if (*r_ptr).flags1 & 0x20000 as libc::c_int as libc::c_uint !=
                       0 {
                    1 as libc::c_int
                } else { 0 as libc::c_int } as bool_
        }
        54 => {
            okay =
                ((*r_ptr).d_char as libc::c_int == 'p' as i32 &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        55 => {
            okay =
                ((*r_ptr).d_char as libc::c_int == 'G' as i32 &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        57 => {
            okay =
                ((*r_ptr).d_char as libc::c_int == 'Q' as i32 &&
                     (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint ==
                         0) as libc::c_int as bool_
        }
        58 => { okay = summon_lua_okay(r_idx) }
        _ => { }
    }
    /* Result */
    return okay;
}
/*
 * Place a monster (of the specified "type") near the given
 * location.  Return TRUE if a monster was actually summoned.
 *
 * We will attempt to place the monster up to 10 times before giving up.
 *
 * Note: SUMMON_UNIQUE and SUMMON_WRAITH (XXX) will summon Unique's
 * Note: SUMMON_HI_UNDEAD and SUMMON_HI_DRAGON may summon Unique's
 * Note: None of the other summon codes will ever summon Unique's.
 *
 * This function has been changed.  We now take the "monster level"
 * of the summoning monster as a parameter, and use that, along with
 * the current dungeon level, to help determine the level of the
 * desired monster.  Note that this is an upper bound, and also
 * tends to "prefer" monsters of that level.  Currently, we use
 * the average of the dungeon and monster levels, and then add
 * five to allow slight increases in monster power.
 *
 * Note that we use the new "monster allocation table" creation code
 * to restrict the "get_mon_num()" function to the set of "legal"
 * monsters, making this function much faster and more reliable.
 *
 * Note that this function may not succeed, though this is very rare.
 */
#[no_mangle]
pub static mut summon_specific_level: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn summon_specific(mut y1: libc::c_int,
                                         mut x1: libc::c_int,
                                         mut lev: libc::c_int,
                                         mut type_0: libc::c_int) -> bool_ {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut r_idx: libc::c_int = 0;
    let mut Group_ok: bool_ = 1 as libc::c_int as bool_;
    let mut old_get_mon_num_hook:
            Option<unsafe extern "C" fn(_: libc::c_int) -> bool_> = None;
    /* Look for a location */
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        /* Pick a distance */
        let mut d: libc::c_int = i / 15 as libc::c_int + 1 as libc::c_int;
        /* Pick a location */
        scatter(&mut y, &mut x, y1, x1, d);
        /* Require "empty" floor grid */
        if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                               isize)).flags1 as libc::c_long &
               0x10 as libc::c_long != 0 &&
               (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
                   0xaf as libc::c_int &&
               (*cave[y as usize].offset(x as isize)).m_idx == 0 &&
               !(y == (*p_ptr).py as libc::c_int &&
                     x == (*p_ptr).px as libc::c_int) {
            /* Hack -- no summon on glyph of warding */
            if !((*cave[y as usize].offset(x as isize)).feat as libc::c_int ==
                     0x3 as libc::c_int) {
                if !((*cave[y as usize].offset(x as isize)).feat as
                         libc::c_int == 0x40 as libc::c_int) {
                    /* Nor on the between */
                    if (*cave[y as usize].offset(x as isize)).feat as
                           libc::c_int == 0xa0 as libc::c_int {
                        return 0 as libc::c_int as bool_
                    }
                    /* ... nor on the Pattern */
                    if !((*cave[y as usize].offset(x as isize)).feat as
                             libc::c_int >= 0x41 as libc::c_int &&
                             (*cave[y as usize].offset(x as isize)).feat as
                                 libc::c_int <= 0x49 as libc::c_int) {
                        break ;
                    }
                }
            }
        }
        i += 1
    }
    /* Failure */
    if i == 20 as libc::c_int { return 0 as libc::c_int as bool_ }
    /* Save the "summon" type */
    summon_specific_type = type_0;
    /* Backup the old hook */
    old_get_mon_num_hook = get_mon_num_hook;
    /* Require "okay" monsters */
    get_mon_num_hook =
        Some(summon_specific_okay as
                 unsafe extern "C" fn(_: libc::c_int) -> bool_);
    /* Prepare allocation table */
    get_mon_num_prep();
    /* Pick a monster, using the level calculation */
    summon_hack = 1 as libc::c_int as bool_;
    r_idx =
        get_mon_num((dun_level as libc::c_int + lev) / 2 as libc::c_int +
                        5 as libc::c_int) as libc::c_int;
    summon_hack = 0 as libc::c_int as bool_;
    /* Reset restriction */
    get_mon_num_hook = old_get_mon_num_hook;
    /* Prepare allocation table */
    get_mon_num_prep();
    /* Handle failure */
    if r_idx == 0 { return 0 as libc::c_int as bool_ }
    if type_0 == 41 as libc::c_int || type_0 == 50 as libc::c_int {
        Group_ok = 0 as libc::c_int as bool_
    }
    /* Attempt to place the monster (awake, allow groups) */
    if place_monster_aux(y, x, r_idx, 0 as libc::c_int as bool_, Group_ok,
                         -(2 as libc::c_int)) == 0 {
        return 0 as libc::c_int as bool_
    }
    if summon_specific_level != 0 {
        monster_set_level(place_monster_result, summon_specific_level);
        summon_specific_level = 0 as libc::c_int
    }
    /* Success */
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn summon_specific_friendly(mut y1: libc::c_int,
                                                  mut x1: libc::c_int,
                                                  mut lev: libc::c_int,
                                                  mut type_0: libc::c_int,
                                                  mut Group_ok: bool_)
 -> bool_ {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut r_idx: libc::c_int = 0;
    let mut old_get_mon_num_hook:
            Option<unsafe extern "C" fn(_: libc::c_int) -> bool_> = None;
    /* Look for a location */
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        /* Pick a distance */
        let mut d: libc::c_int = i / 15 as libc::c_int + 1 as libc::c_int;
        /* Pick a location */
        scatter(&mut y, &mut x, y1, x1, d);
        /* Require "empty" floor grid */
        if (*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                               isize)).flags1 as libc::c_long &
               0x10 as libc::c_long != 0 &&
               (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
                   0xaf as libc::c_int &&
               (*cave[y as usize].offset(x as isize)).m_idx == 0 &&
               !(y == (*p_ptr).py as libc::c_int &&
                     x == (*p_ptr).px as libc::c_int) {
            /* Hack -- no summon on glyph of warding */
            if !((*cave[y as usize].offset(x as isize)).feat as libc::c_int ==
                     0x3 as libc::c_int) {
                if !((*cave[y as usize].offset(x as isize)).feat as
                         libc::c_int == 0x40 as libc::c_int) {
                    /* Nor on the between */
                    if (*cave[y as usize].offset(x as isize)).feat as
                           libc::c_int == 0xa0 as libc::c_int {
                        return 0 as libc::c_int as bool_
                    }
                    /* ... nor on the Pattern */
                    if !((*cave[y as usize].offset(x as isize)).feat as
                             libc::c_int >= 0x41 as libc::c_int &&
                             (*cave[y as usize].offset(x as isize)).feat as
                                 libc::c_int <= 0x49 as libc::c_int) {
                        break ;
                    }
                }
            }
        }
        i += 1
    }
    /* Failure */
    if i == 20 as libc::c_int { return 0 as libc::c_int as bool_ }
    old_get_mon_num_hook = get_mon_num_hook;
    /* Save the "summon" type */
    summon_specific_type = type_0;
    /* Require "okay" monsters */
    get_mon_num_hook =
        Some(summon_specific_okay as
                 unsafe extern "C" fn(_: libc::c_int) -> bool_);
    /* Prepare allocation table */
    get_mon_num_prep();
    /* Pick a monster, using the level calculation */
    r_idx =
        get_mon_num((dun_level as libc::c_int + lev) / 2 as libc::c_int +
                        5 as libc::c_int) as libc::c_int;
    /* Reset restriction */
    get_mon_num_hook = old_get_mon_num_hook;
    /* Prepare allocation table */
    get_mon_num_prep();
    /* Handle failure */
    if r_idx == 0 { return 0 as libc::c_int as bool_ }
    /* Attempt to place the monster (awake, allow groups) */
    if place_monster_aux(y, x, r_idx, 0 as libc::c_int as bool_, Group_ok,
                         3 as libc::c_int) == 0 {
        return 0 as libc::c_int as bool_
    }
    if summon_specific_level != 0 {
        monster_set_level(place_monster_result, summon_specific_level);
        summon_specific_level = 0 as libc::c_int
    }
    /* Success */
    return 1 as libc::c_int as bool_;
}
/*
 * Swap the players/monsters (if any) at two locations XXX XXX XXX
 */
#[no_mangle]
pub unsafe extern "C" fn monster_swap(mut y1: libc::c_int,
                                      mut x1: libc::c_int,
                                      mut y2: libc::c_int,
                                      mut x2: libc::c_int) {
    let mut m1: libc::c_int = 0;
    let mut m2: libc::c_int = 0;
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    let mut c_ptr1: *mut cave_type = 0 as *mut cave_type;
    let mut c_ptr2: *mut cave_type = 0 as *mut cave_type;
    c_ptr1 =
        &mut *(*cave.as_mut_ptr().offset(y1 as isize)).offset(x1 as isize) as
            *mut cave_type;
    c_ptr2 =
        &mut *(*cave.as_mut_ptr().offset(y2 as isize)).offset(x2 as isize) as
            *mut cave_type;
    /* Monsters */
    m1 = (*c_ptr1).m_idx as libc::c_int;
    m2 = (*c_ptr2).m_idx as libc::c_int;
    /* Update grids */
    (*c_ptr1).m_idx = m2 as s16b;
    (*c_ptr2).m_idx = m1 as s16b;
    /* Monster 1 */
    if m1 > 0 as libc::c_int {
        m_ptr = &mut *m_list.offset(m1 as isize) as *mut monster_type;
        /* Move monster */
        (*m_ptr).fy = y2 as byte_hack;
        (*m_ptr).fx = x2 as byte_hack;
        /* Update monster */
        update_mon(m1, 1 as libc::c_int as bool_);
    } else if m1 < 0 as libc::c_int {
        /* Player 1 */
        /* Move player */
        (*p_ptr).py = y2 as s16b;
        (*p_ptr).px = x2 as s16b;
        /* Window stuff */
		/* It's probably not a good idea to recalculate the
		 * overhead view each turn.

		 p_ptr->window |= (PW_OVERHEAD);
		*/
        verify_panel();
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x100000 as libc::c_long | 0x10000000 as libc::c_long |
                      0x200000 as libc::c_long)) as u32b;
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x2000000 as libc::c_long) as
                u32b
    }
    /* Check for new panel (redraw map) */
    /* Update stuff */
    /* Update the monsters */
    /* Monster 2 */
    if m2 > 0 as libc::c_int {
        m_ptr = &mut *m_list.offset(m2 as isize) as *mut monster_type;
        /* Move monster */
        (*m_ptr).fy = y1 as byte_hack;
        (*m_ptr).fx = x1 as byte_hack;
        /* Update monster */
        update_mon(m2, 1 as libc::c_int as bool_);
    } else if m2 > 0 as libc::c_int {
        /* Player 2 */
        /* Move player */
        (*p_ptr).py = y1 as s16b;
        (*p_ptr).px = x1 as s16b;
        /* Window stuff */
		/* It's probably not a good idea to recalculate the
		 * overhead view each turn.

		 p_ptr->window |= (PW_OVERHEAD);
		*/
        verify_panel();
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x100000 as libc::c_long | 0x10000000 as libc::c_long |
                      0x200000 as libc::c_long)) as u32b;
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x2000000 as libc::c_long) as
                u32b
    }
    /* Check for new panel (redraw map) */
    /* Update stuff */
    /* Update the monsters */
    /* Redraw */
    lite_spot(y1, x1);
    lite_spot(y2, x2);
}
/*
 * Hack -- help decide if a monster race is "okay" to summon
 */
unsafe extern "C" fn mutate_monster_okay(mut r_idx: libc::c_int) -> bool_ {
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset(r_idx as isize) as *mut monster_race;
    let mut okay: bool_ = 0 as libc::c_int as bool_;
    /* Hack - Only summon dungeon monsters */
    if monster_dungeon(r_idx) == 0 { return 0 as libc::c_int as bool_ }
    okay =
        ((*r_ptr).d_char as libc::c_int == summon_kin_type as libc::c_int &&
             (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint == 0 &&
             (*r_ptr).level as libc::c_int >= dun_level as libc::c_int) as
            libc::c_int as bool_;
    return okay;
}
/*
 * Let the given monster attempt to reproduce.
 *
 * Note that "reproduction" REQUIRES empty space.
 */
#[no_mangle]
pub unsafe extern "C" fn multiply_monster(mut m_idx: libc::c_int,
                                          mut charm: bool_, mut clone: bool_)
 -> bool_ {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut new_race: libc::c_int = 0;
    let mut result: bool_ = 0 as libc::c_int as bool_;
    if no_breeds != 0 {
        msg_print(b"It tries to breed but it fails!\x00" as *const u8 as
                      *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    /* Try up to 18 times */
    i = 0 as libc::c_int;
    while i < 18 as libc::c_int {
        let mut d: libc::c_int = 1 as libc::c_int;
        /* Pick a location */
        scatter(&mut y, &mut x, (*m_ptr).fy as libc::c_int,
                (*m_ptr).fx as libc::c_int, d);
        /* Require an "empty" floor grid */
        if !((*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                                 isize)).flags1 as libc::c_long &
                 0x10 as libc::c_long != 0 &&
                 (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
                     0xaf as libc::c_int &&
                 (*cave[y as usize].offset(x as isize)).m_idx == 0 &&
                 !(y == (*p_ptr).py as libc::c_int &&
                       x == (*p_ptr).px as libc::c_int)) {
            i += 1
        } else {
            new_race = (*m_ptr).r_idx as libc::c_int;
            /* It can mutate into a nastier monster */
            if Rand_div(100 as libc::c_int) < 3 as libc::c_int && clone == 0 {
                let mut old_get_mon_num_hook:
                        Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> bool_> = None;
                /* Backup the old hook */
                old_get_mon_num_hook = get_mon_num_hook;
                /* Require "okay" monsters */
                get_mon_num_hook =
                    Some(mutate_monster_okay as
                             unsafe extern "C" fn(_: libc::c_int) -> bool_);
                /* Prepare allocation table */
                get_mon_num_prep();
                summon_kin_type = (*r_ptr).d_char;
                /* Pick a monster, using the level calculation */
                new_race =
                    get_mon_num(dun_level as libc::c_int + 5 as libc::c_int)
                        as libc::c_int;
                /* Reset restriction */
                get_mon_num_hook = old_get_mon_num_hook;
                /* Prepare allocation table */
                get_mon_num_prep();
            }
            /* Create a new monster (awake, no groups) */
            result =
                place_monster_aux(y, x, new_race, 0 as libc::c_int as bool_,
                                  0 as libc::c_int as bool_,
                                  if charm as libc::c_int != 0 {
                                      3 as libc::c_int
                                  } else { -(2 as libc::c_int) });
            break ;
        }
    }
    if clone as libc::c_int != 0 && result as libc::c_int != 0 {
        let ref mut fresh6 = (*m_list.offset(hack_m_idx_ii as isize)).smart;
        *fresh6 |= 0x400000 as libc::c_int as libc::c_uint
    }
    /* Result */
    return result;
}
/*
 * Dump a message describing a monster's reaction to damage
 *
 * Technically should attempt to treat "Beholder"'s as jelly's
 */
#[no_mangle]
pub static mut hack_message_pain_may_silent: bool_ =
    0 as libc::c_int as bool_;
#[no_mangle]
pub unsafe extern "C" fn message_pain_hook(mut fmt: cptr, mut args: ...) {
    let mut vp: ::std::ffi::VaListImpl;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    /* Begin the Varargs Stuff */
    vp = args.clone();
    /* Format the args, save the length */
    vstrnfmt(buf.as_mut_ptr(), 1024 as libc::c_int as uint_hack, fmt,
             vp.as_va_list());
    /* End the Varargs Stuff */
    if hack_message_pain_may_silent != 0 {
        monster_msg(buf.as_mut_ptr() as cptr);
    } else { msg_print(buf.as_mut_ptr() as cptr); };
}
#[no_mangle]
pub unsafe extern "C" fn message_pain(mut m_idx: libc::c_int,
                                      mut dam: libc::c_int) {
    let mut oldhp: libc::c_long = 0;
    let mut newhp: libc::c_long = 0;
    let mut tmp: libc::c_long = 0;
    let mut percentage: libc::c_int = 0;
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    let mut m_name: [libc::c_char; 80] = [0; 80];
    /* Get the monster name */
    monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
    /* Notice non-damage */
    if dam == 0 as libc::c_int {
        message_pain_hook(b"%^s is unharmed.\x00" as *const u8 as
                              *const libc::c_char, m_name.as_mut_ptr());
        return
    }
    /* Note -- subtle fix -CFT */
    newhp = (*m_ptr).hp as libc::c_long;
    oldhp = newhp + dam as libc::c_long;
    tmp = newhp * 100 as libc::c_long / oldhp;
    percentage = tmp as libc::c_int;
    /* Jelly's, Mold's, Vortex's, Quthl's */
    if !strchr(b"jmvQ\x00" as *const u8 as *const libc::c_char,
               (*r_ptr).d_char as libc::c_int).is_null() {
        if percentage > 95 as libc::c_int {
            message_pain_hook(b"%^s barely notices.\x00" as *const u8 as
                                  *const libc::c_char, m_name.as_mut_ptr());
        } else if percentage > 75 as libc::c_int {
            message_pain_hook(b"%^s flinches.\x00" as *const u8 as
                                  *const libc::c_char, m_name.as_mut_ptr());
        } else if percentage > 50 as libc::c_int {
            message_pain_hook(b"%^s squelches.\x00" as *const u8 as
                                  *const libc::c_char, m_name.as_mut_ptr());
        } else if percentage > 35 as libc::c_int {
            message_pain_hook(b"%^s quivers in pain.\x00" as *const u8 as
                                  *const libc::c_char, m_name.as_mut_ptr());
        } else if percentage > 20 as libc::c_int {
            message_pain_hook(b"%^s writhes about.\x00" as *const u8 as
                                  *const libc::c_char, m_name.as_mut_ptr());
        } else if percentage > 10 as libc::c_int {
            message_pain_hook(b"%^s writhes in agony.\x00" as *const u8 as
                                  *const libc::c_char, m_name.as_mut_ptr());
        } else {
            message_pain_hook(b"%^s jerks limply.\x00" as *const u8 as
                                  *const libc::c_char, m_name.as_mut_ptr());
        }
    } else if !strchr(b"CZ\x00" as *const u8 as *const libc::c_char,
                      (*r_ptr).d_char as libc::c_int).is_null() {
        if percentage > 95 as libc::c_int {
            message_pain_hook(b"%^s shrugs off the attack.\x00" as *const u8
                                  as *const libc::c_char,
                              m_name.as_mut_ptr());
        } else if percentage > 75 as libc::c_int {
            message_pain_hook(b"%^s snarls with pain.\x00" as *const u8 as
                                  *const libc::c_char, m_name.as_mut_ptr());
        } else if percentage > 50 as libc::c_int {
            message_pain_hook(b"%^s yelps in pain.\x00" as *const u8 as
                                  *const libc::c_char, m_name.as_mut_ptr());
        } else if percentage > 35 as libc::c_int {
            message_pain_hook(b"%^s howls in pain.\x00" as *const u8 as
                                  *const libc::c_char, m_name.as_mut_ptr());
        } else if percentage > 20 as libc::c_int {
            message_pain_hook(b"%^s howls in agony.\x00" as *const u8 as
                                  *const libc::c_char, m_name.as_mut_ptr());
        } else if percentage > 10 as libc::c_int {
            message_pain_hook(b"%^s writhes in agony.\x00" as *const u8 as
                                  *const libc::c_char, m_name.as_mut_ptr());
        } else {
            message_pain_hook(b"%^s yelps feebly.\x00" as *const u8 as
                                  *const libc::c_char, m_name.as_mut_ptr());
        }
    } else if !strchr(b"FIKMRSXabclqrst\x00" as *const u8 as
                          *const libc::c_char,
                      (*r_ptr).d_char as libc::c_int).is_null() {
        if percentage > 95 as libc::c_int {
            message_pain_hook(b"%^s ignores the attack.\x00" as *const u8 as
                                  *const libc::c_char, m_name.as_mut_ptr());
        } else if percentage > 75 as libc::c_int {
            message_pain_hook(b"%^s grunts with pain.\x00" as *const u8 as
                                  *const libc::c_char, m_name.as_mut_ptr());
        } else if percentage > 50 as libc::c_int {
            message_pain_hook(b"%^s squeals in pain.\x00" as *const u8 as
                                  *const libc::c_char, m_name.as_mut_ptr());
        } else if percentage > 35 as libc::c_int {
            message_pain_hook(b"%^s shrieks in pain.\x00" as *const u8 as
                                  *const libc::c_char, m_name.as_mut_ptr());
        } else if percentage > 20 as libc::c_int {
            message_pain_hook(b"%^s shrieks in agony.\x00" as *const u8 as
                                  *const libc::c_char, m_name.as_mut_ptr());
        } else if percentage > 10 as libc::c_int {
            message_pain_hook(b"%^s writhes in agony.\x00" as *const u8 as
                                  *const libc::c_char, m_name.as_mut_ptr());
        } else {
            message_pain_hook(b"%^s cries out feebly.\x00" as *const u8 as
                                  *const libc::c_char, m_name.as_mut_ptr());
        }
    } else if percentage > 95 as libc::c_int {
        message_pain_hook(b"%^s shrugs off the attack.\x00" as *const u8 as
                              *const libc::c_char, m_name.as_mut_ptr());
    } else if percentage > 75 as libc::c_int {
        message_pain_hook(b"%^s grunts with pain.\x00" as *const u8 as
                              *const libc::c_char, m_name.as_mut_ptr());
    } else if percentage > 50 as libc::c_int {
        message_pain_hook(b"%^s cries out in pain.\x00" as *const u8 as
                              *const libc::c_char, m_name.as_mut_ptr());
    } else if percentage > 35 as libc::c_int {
        message_pain_hook(b"%^s screams in pain.\x00" as *const u8 as
                              *const libc::c_char, m_name.as_mut_ptr());
    } else if percentage > 20 as libc::c_int {
        message_pain_hook(b"%^s screams in agony.\x00" as *const u8 as
                              *const libc::c_char, m_name.as_mut_ptr());
    } else if percentage > 10 as libc::c_int {
        message_pain_hook(b"%^s writhes in agony.\x00" as *const u8 as
                              *const libc::c_char, m_name.as_mut_ptr());
    } else {
        message_pain_hook(b"%^s cries out feebly.\x00" as *const u8 as
                              *const libc::c_char, m_name.as_mut_ptr());
    };
}
/* Dogs and Hounds */
/* One type of monsters (ignore,squeal,shriek) */
/* Another type of monsters (shrug,cry,scream) */
/*
 * Learn about an "observed" resistance.
 */
#[no_mangle]
pub unsafe extern "C" fn update_smart_learn(mut m_idx: libc::c_int,
                                            mut what: libc::c_int) {
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    /* Not allowed to learn */
    if smart_learn == 0 { return }
    /* Too stupid to learn anything */
    if (*r_ptr).flags2 & 0x1 as libc::c_int as libc::c_uint != 0 { return }
    /* Not intelligent, only learn sometimes */
    if (*r_ptr).flags2 & 0x2 as libc::c_int as libc::c_uint == 0 &&
           Rand_div(100 as libc::c_int) < 50 as libc::c_int {
        return
    }
    /* XXX XXX XXX */
    /* Analyze the knowledge */
    match what {
        1 => {
            if (*p_ptr).resist_acid != 0 {
                (*m_ptr).smart |= 0x1 as libc::c_int as libc::c_uint
            }
            if (*p_ptr).oppose_acid != 0 {
                (*m_ptr).smart |= 0x10000 as libc::c_int as libc::c_uint
            }
            if (*p_ptr).immune_acid != 0 {
                (*m_ptr).smart |= 0x1000000 as libc::c_int as libc::c_uint
            }
        }
        2 => {
            if (*p_ptr).resist_elec != 0 {
                (*m_ptr).smart |= 0x2 as libc::c_int as libc::c_uint
            }
            if (*p_ptr).oppose_elec != 0 {
                (*m_ptr).smart |= 0x20000 as libc::c_int as libc::c_uint
            }
            if (*p_ptr).immune_elec != 0 {
                (*m_ptr).smart |= 0x2000000 as libc::c_int as libc::c_uint
            }
        }
        3 => {
            if (*p_ptr).resist_fire != 0 {
                (*m_ptr).smart |= 0x4 as libc::c_int as libc::c_uint
            }
            if (*p_ptr).oppose_fire != 0 {
                (*m_ptr).smart |= 0x40000 as libc::c_int as libc::c_uint
            }
            if (*p_ptr).immune_fire != 0 {
                (*m_ptr).smart |= 0x4000000 as libc::c_int as libc::c_uint
            }
        }
        4 => {
            if (*p_ptr).resist_cold != 0 {
                (*m_ptr).smart |= 0x8 as libc::c_int as libc::c_uint
            }
            if (*p_ptr).oppose_cold != 0 {
                (*m_ptr).smart |= 0x80000 as libc::c_int as libc::c_uint
            }
            if (*p_ptr).immune_cold != 0 {
                (*m_ptr).smart |= 0x8000000 as libc::c_int as libc::c_uint
            }
        }
        5 => {
            if (*p_ptr).resist_pois != 0 {
                (*m_ptr).smart |= 0x10 as libc::c_int as libc::c_uint
            }
            if (*p_ptr).oppose_pois != 0 {
                (*m_ptr).smart |= 0x100000 as libc::c_int as libc::c_uint
            }
        }
        6 => {
            if (*p_ptr).resist_neth != 0 {
                (*m_ptr).smart |= 0x20 as libc::c_int as libc::c_uint
            }
        }
        7 => {
            if (*p_ptr).resist_lite != 0 {
                (*m_ptr).smart |= 0x40 as libc::c_int as libc::c_uint
            }
        }
        8 => {
            if (*p_ptr).resist_dark != 0 {
                (*m_ptr).smart |= 0x80 as libc::c_int as libc::c_uint
            }
        }
        9 => {
            if (*p_ptr).resist_fear != 0 {
                (*m_ptr).smart |= 0x100 as libc::c_int as libc::c_uint
            }
        }
        10 => {
            if (*p_ptr).resist_conf != 0 {
                (*m_ptr).smart |= 0x200 as libc::c_int as libc::c_uint
            }
        }
        11 => {
            if (*p_ptr).resist_chaos != 0 {
                (*m_ptr).smart |= 0x400 as libc::c_int as libc::c_uint
            }
        }
        12 => {
            if (*p_ptr).resist_disen != 0 {
                (*m_ptr).smart |= 0x800 as libc::c_int as libc::c_uint
            }
        }
        13 => {
            if (*p_ptr).resist_blind != 0 {
                (*m_ptr).smart |= 0x1000 as libc::c_int as libc::c_uint
            }
        }
        14 => {
            if (*p_ptr).resist_nexus != 0 {
                (*m_ptr).smart |= 0x2000 as libc::c_int as libc::c_uint
            }
        }
        15 => {
            if (*p_ptr).resist_sound != 0 {
                (*m_ptr).smart |= 0x4000 as libc::c_int as libc::c_uint
            }
        }
        16 => {
            if (*p_ptr).resist_shard != 0 {
                (*m_ptr).smart |= 0x8000 as libc::c_int as libc::c_uint
            }
        }
        30 => {
            if (*p_ptr).free_act != 0 {
                (*m_ptr).smart |= 0x40000000 as libc::c_int as libc::c_uint
            }
        }
        31 => {
            if (*p_ptr).msp == 0 {
                (*m_ptr).smart |= 0x80000000 as libc::c_uint
            }
        }
        32 => {
            if (*p_ptr).reflect != 0 {
                (*m_ptr).smart |= 0x20000000 as libc::c_int as libc::c_uint
            }
        }
        _ => { }
    };
}
/*
 * Place the player in the dungeon XXX XXX
 */
#[no_mangle]
pub unsafe extern "C" fn player_place(mut y: libc::c_int, mut x: libc::c_int)
 -> s16b {
    /* Paranoia XXX XXX */
    if (*cave[y as usize].offset(x as isize)).m_idx as libc::c_int !=
           0 as libc::c_int {
        return 0 as libc::c_int as s16b
    }
    /* Save player location */
    (*p_ptr).py = y as s16b;
    (*p_ptr).px = x as s16b;
    /* Success */
    return -(1 as libc::c_int) as s16b;
}
/*
 * Drop all items carried by a monster
 */
#[no_mangle]
pub unsafe extern "C" fn monster_drop_carried_objects(mut m_ptr:
                                                          *mut monster_type) {
    let mut this_o_idx: s16b = 0;
    let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
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
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut q_ptr: *mut object_type = 0 as *mut object_type;
    /* Drop objects being carried */
    this_o_idx = (*m_ptr).hold_o_idx;
    while this_o_idx != 0 {
        /* Acquire object */
        o_ptr = &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
        /* Acquire next object */
        next_o_idx = (*o_ptr).next_o_idx;
        /* Paranoia */
        (*o_ptr).held_m_idx = 0 as libc::c_int as s16b;
        /* Get local object */
        q_ptr = &mut forge;
        /* Copy the object */
        object_copy(q_ptr, o_ptr);
        /* Delete the object */
        delete_object_idx(this_o_idx as libc::c_int);
        /* Drop it */
        drop_near(q_ptr, -(1 as libc::c_int), (*m_ptr).fy as libc::c_int,
                  (*m_ptr).fx as libc::c_int);
        this_o_idx = next_o_idx
    }
    /* Forget objects */
    (*m_ptr).hold_o_idx = 0 as libc::c_int as s16b;
}
