use ::libc;
extern "C" {
    #[no_mangle]
    static mut ddx: [s16b; 10];
    #[no_mangle]
    static mut ddy: [s16b; 10];
    #[no_mangle]
    static mut bear_blows: [martial_arts; 8];
    #[no_mangle]
    static mut ma_blows: [martial_arts; 17];
    #[no_mangle]
    static mut inscription_info: [inscription_info_type; 8];
    #[no_mangle]
    static mut character_icky: bool_;
    #[no_mangle]
    static mut command_new: s16b;
    #[no_mangle]
    static mut energy_use: s32b;
    #[no_mangle]
    static mut running: s16b;
    #[no_mangle]
    static mut cur_hgt: s16b;
    #[no_mangle]
    static mut cur_wid: s16b;
    #[no_mangle]
    static mut dun_level: s16b;
    #[no_mangle]
    static mut wizard: bool_;
    #[no_mangle]
    static mut m_max: s16b;
    #[no_mangle]
    static mut always_pickup: bool_;
    #[no_mangle]
    static mut find_ignore_stairs: bool_;
    #[no_mangle]
    static mut find_ignore_doors: bool_;
    #[no_mangle]
    static mut find_cut: bool_;
    #[no_mangle]
    static mut find_examine: bool_;
    #[no_mangle]
    static mut disturb_detect: bool_;
    #[no_mangle]
    static mut cheat_xtra: bool_;
    #[no_mangle]
    static mut cave: [*mut cave_type; 66];
    #[no_mangle]
    static mut o_list: *mut object_type;
    #[no_mangle]
    static mut m_list: *mut monster_type;
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
    static mut f_name: *mut libc::c_char;
    #[no_mangle]
    static mut f_text: *mut libc::c_char;
    #[no_mangle]
    static mut k_info: *mut object_kind;
    #[no_mangle]
    static mut r_info: *mut monster_race;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn streq(s: cptr, t: cptr) -> bool_;
    #[no_mangle]
    fn strnfmt(buf: *mut libc::c_char, max: uint_hack, fmt: cptr, _: ...)
     -> uint_hack;
    #[no_mangle]
    fn strfmt(buf: *mut libc::c_char, fmt: cptr, _: ...) -> uint_hack;
    #[no_mangle]
    fn format(fmt: cptr, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Rand_div(m: s32b) -> s32b;
    #[no_mangle]
    fn damroll(num: s16b, sides: s16b) -> s32b;
    #[no_mangle]
    fn Term_save() -> errr;
    #[no_mangle]
    fn Term_load() -> errr;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    static mut d_info: *mut dungeon_info_type;
    #[no_mangle]
    static mut d_text: *mut libc::c_char;
    #[no_mangle]
    static mut t_info: *mut trap_type;
    #[no_mangle]
    static mut t_name: *mut libc::c_char;
    #[no_mangle]
    static mut item_tester_tval: byte_hack;
    #[no_mangle]
    static mut ambush_flag: bool_;
    #[no_mangle]
    fn process_hooks(h_idx: libc::c_int, fmt: *mut libc::c_char, _: ...)
     -> bool_;
    #[no_mangle]
    fn no_lite() -> bool_;
    #[no_mangle]
    fn lite_spot(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn cave_set_feat(y: libc::c_int, x: libc::c_int, feat: libc::c_int);
    #[no_mangle]
    fn scatter(yp: *mut libc::c_int, xp: *mut libc::c_int, y: libc::c_int,
               x: libc::c_int, d: libc::c_int);
    #[no_mangle]
    fn health_track(m_idx: libc::c_int);
    #[no_mangle]
    fn monster_race_track(r_idx: libc::c_int, ego: libc::c_int);
    #[no_mangle]
    fn disturb(stop_search: libc::c_int, flush_output: libc::c_int);
    #[no_mangle]
    fn is_quest(level: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn msg_format(fmt: cptr, _: ...);
    #[no_mangle]
    fn race_info_idx(r_idx: libc::c_int, ego: libc::c_int)
     -> *mut monster_race;
    #[no_mangle]
    fn monster_desc(desc: *mut libc::c_char, m_ptr: *mut monster_type,
                    mode: libc::c_int);
    #[no_mangle]
    fn luck(min: libc::c_int, max: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn msg_print(msg: cptr);
    #[no_mangle]
    fn get_skill_scale(skill: libc::c_int, scale: u32b) -> s16b;
    #[no_mangle]
    fn get_skill(skill: libc::c_int) -> s16b;
    #[no_mangle]
    fn set_tim_deadly(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn object_flags(o_ptr: *mut object_type, f1: *mut u32b, f2: *mut u32b,
                    f3: *mut u32b, f4: *mut u32b, f5: *mut u32b,
                    esp: *mut u32b);
    #[no_mangle]
    fn object_known(o_ptr: *mut object_type);
    #[no_mangle]
    fn pick_trap(y: libc::c_int, x: libc::c_int);
    #[no_mangle]
    fn py_pickup_floor(pickup: libc::c_int);
    #[no_mangle]
    fn earthquake(cy: libc::c_int, cx: libc::c_int, r: libc::c_int);
    #[no_mangle]
    fn sound(num: libc::c_int);
    #[no_mangle]
    fn teleport_player(dis: libc::c_int);
    #[no_mangle]
    fn symbiote_name(capitalize: bool_) -> cptr;
    #[no_mangle]
    fn project(who: libc::c_int, rad: libc::c_int, y: libc::c_int,
               x: libc::c_int, dam: libc::c_int, typ: libc::c_int,
               flg: libc::c_int) -> bool_;
    #[no_mangle]
    fn check_hit2(power: libc::c_int, level: libc::c_int, ac: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn get_attack_power(effect: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn do_poly_monster(y: libc::c_int, x: libc::c_int) -> s16b;
    #[no_mangle]
    fn teleport_away(m_idx: libc::c_int, dis: libc::c_int);
    #[no_mangle]
    fn hp_player(num: libc::c_int) -> bool_;
    #[no_mangle]
    fn handle_stuff();
    #[no_mangle]
    fn take_hit(damage: libc::c_int, kb_str: cptr);
    #[no_mangle]
    fn change_side(m_ptr: *mut monster_type) -> bool_;
    #[no_mangle]
    fn is_friend(m_ptr: *mut monster_type) -> libc::c_int;
    #[no_mangle]
    fn has_ability(ab: libc::c_int) -> bool_;
    #[no_mangle]
    fn mon_take_hit(m_idx: libc::c_int, dam: libc::c_int, fear: *mut bool_,
                    note: cptr) -> bool_;
    #[no_mangle]
    fn monster_race_desc(desc: *mut libc::c_char, r_idx: libc::c_int,
                         ego: libc::c_int);
    #[no_mangle]
    fn get_rnd_line(file_name: *mut libc::c_char, output: *mut libc::c_char)
     -> errr;
    #[no_mangle]
    fn multiply_monster(m_idx: libc::c_int, charm: bool_, clone: bool_)
     -> bool_;
    #[no_mangle]
    fn exec_lua(file: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn wisdom_scale(max: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn inc_stack_size_ex(item: libc::c_int, delta: libc::c_int,
                         opt: optimize_flag, desc: describe_flag);
    #[no_mangle]
    fn apply_disenchant(mode: libc::c_int) -> bool_;
    #[no_mangle]
    fn resolve_mimic_name(name: cptr) -> libc::c_int;
    #[no_mangle]
    fn set_disrupt_shield(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn set_invuln(v: libc::c_int) -> bool_;
    #[no_mangle]
    fn quark_str(num: s16b) -> cptr;
    #[no_mangle]
    fn weight_limit() -> libc::c_int;
    #[no_mangle]
    fn calc_total_weight() -> s32b;
    #[no_mangle]
    fn reveal_wilderness_around_player(y: libc::c_int, x: libc::c_int,
                                       h: libc::c_int, w: libc::c_int);
    #[no_mangle]
    fn delete_object_idx(o_idx: libc::c_int);
    #[no_mangle]
    fn object_desc(buf: *mut libc::c_char, o_ptr: *mut object_type,
                   pref: libc::c_int, mode: libc::c_int);
    #[no_mangle]
    fn delete_monster_idx(i: libc::c_int);
    #[no_mangle]
    fn test_monster_name(name: cptr) -> libc::c_int;
    #[no_mangle]
    fn place_monster_one(y: libc::c_int, x: libc::c_int, r_idx: libc::c_int,
                         ego: libc::c_int, slp: bool_, status: libc::c_int)
     -> s16b;
    #[no_mangle]
    fn unlite_room(y1: libc::c_int, x1: libc::c_int);
    #[no_mangle]
    fn lite_room(y1: libc::c_int, x1: libc::c_int);
    #[no_mangle]
    fn player_activate_trap_type(y: s16b, x: s16b, i_ptr: *mut object_type,
                                 item: s16b) -> bool_;
    #[no_mangle]
    fn is_a_vowel(ch: libc::c_int) -> bool_;
    #[no_mangle]
    fn cmsg_print(color: byte_hack, msg: cptr);
    #[no_mangle]
    fn verify_panel();
    #[no_mangle]
    fn get_check(prompt: cptr) -> bool_;
    #[no_mangle]
    fn do_cmd_tunnel_aux(y: libc::c_int, x: libc::c_int, dir: libc::c_int)
     -> bool_;
    #[no_mangle]
    static mut easy_tunnel: bool_;
    #[no_mangle]
    fn easy_open_door(y: libc::c_int, x: libc::c_int) -> bool_;
    #[no_mangle]
    static mut easy_open: bool_;
    #[no_mangle]
    fn passwall(dir: libc::c_int, safe: bool_) -> bool_;
    #[no_mangle]
    fn do_cmd_disarm_aux(y: libc::c_int, x: libc::c_int, dir: libc::c_int,
                         do_pickup: libc::c_int) -> bool_;
    #[no_mangle]
    static mut easy_disarm: bool_;
    #[no_mangle]
    fn update_mon(m_idx: libc::c_int, full: bool_);
    #[no_mangle]
    fn monster_drop_carried_objects(m_ptr: *mut monster_type);
    #[no_mangle]
    fn bell();
    #[no_mangle]
    fn get_com(prompt: cptr, command: *mut libc::c_char) -> bool_;
    #[no_mangle]
    fn tgt_pt(x: *mut libc::c_int, y: *mut libc::c_int) -> bool_;
    #[no_mangle]
    fn prt(str: cptr, row: libc::c_int, col: libc::c_int);
    #[no_mangle]
    fn do_cmd_redraw();
    #[no_mangle]
    fn floor_item_optimize(item: libc::c_int);
    #[no_mangle]
    fn floor_item_describe(item: libc::c_int);
    #[no_mangle]
    fn floor_item_increase(item: libc::c_int, num: libc::c_int);
    #[no_mangle]
    fn get_item(cp: *mut libc::c_int, pmt: cptr, str: cptr, mode: libc::c_int)
     -> bool_;
    #[no_mangle]
    fn drop_near(o_ptr: *mut object_type, chance: libc::c_int, y: libc::c_int,
                 x: libc::c_int) -> s16b;
    #[no_mangle]
    fn object_aware(o_ptr: *mut object_type);
    #[no_mangle]
    fn lookup_kind(tval: libc::c_int, sval: libc::c_int) -> s16b;
    #[no_mangle]
    fn object_prep(o_ptr: *mut object_type, k_idx: libc::c_int);
    #[no_mangle]
    fn get_string(prompt: cptr, buf: *mut libc::c_char, len: libc::c_int)
     -> bool_;
}
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
    pub abilities: [C2RustUnnamed_3; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
pub struct martial_arts {
    pub desc: cptr,
    pub min_level: libc::c_int,
    pub chance: libc::c_int,
    pub dd: libc::c_int,
    pub ds: libc::c_int,
    pub effect: s16b,
    pub power: s16b,
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
pub struct inscription_info_type {
    pub text: [libc::c_char; 40],
    pub when: byte_hack,
    pub know: bool_,
    pub mana: byte_hack,
}
pub type optimize_flag = libc::c_uint;
pub const NO_OPTIMIZE: optimize_flag = 1;
pub const OPTIMIZE: optimize_flag = 0;
pub type describe_flag = libc::c_uint;
pub const NO_DESCRIBE: describe_flag = 1;
pub const DESCRIBE: describe_flag = 0;
/*
 * Determine if the player "hits" a monster (normal combat).
 * Note -- Always miss 5%, always hit 5%, otherwise random.
 */
#[no_mangle]
pub unsafe extern "C" fn test_hit_fire(mut chance: libc::c_int,
                                       mut ac: libc::c_int,
                                       mut vis: libc::c_int) -> bool_ {
    let mut k: libc::c_int = 0;
    /* Percentile dice */
    k = Rand_div(100 as libc::c_int);
    /* Hack -- Instant miss or hit */
    if k < 10 as libc::c_int {
        return (k < 5 as libc::c_int) as libc::c_int as bool_
    }
    /* Never hit */
    if chance <= 0 as libc::c_int { return 0 as libc::c_int as bool_ }
    /* Invisible monsters are harder to hit */
    if vis == 0 { chance = (chance + 1 as libc::c_int) / 2 as libc::c_int }
    /* Power competes against armor */
    if Rand_div(chance + luck(-(10 as libc::c_int), 10 as libc::c_int)) <
           ac * 3 as libc::c_int / 4 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Assume hit */
    return 1 as libc::c_int as bool_;
}
/*
 * Determine if the player "hits" a monster (normal combat).
 *
 * Note -- Always miss 5%, always hit 5%, otherwise random.
 */
#[no_mangle]
pub unsafe extern "C" fn test_hit_norm(mut chance: libc::c_int,
                                       mut ac: libc::c_int,
                                       mut vis: libc::c_int) -> bool_ {
    let mut k: libc::c_int = 0;
    /* Percentile dice */
    k = Rand_div(100 as libc::c_int);
    /* Hack -- Instant miss or hit */
    if k < 10 as libc::c_int {
        return (k < 5 as libc::c_int) as libc::c_int as bool_
    }
    /* Wimpy attack never hits */
    if chance <= 0 as libc::c_int { return 0 as libc::c_int as bool_ }
    /* Penalize invisible targets */
    if vis == 0 { chance = (chance + 1 as libc::c_int) / 2 as libc::c_int }
    /* Power must defeat armor */
    if Rand_div(chance + luck(-(10 as libc::c_int), 10 as libc::c_int)) <
           ac * 3 as libc::c_int / 4 as libc::c_int {
        return 0 as libc::c_int as bool_
    }
    /* Assume hit */
    return 1 as libc::c_int as bool_;
}
/*
 * Critical hits (from objects thrown by player)
 * Factor in item weight, total plusses, and player level.
 */
#[no_mangle]
pub unsafe extern "C" fn critical_shot(mut weight: libc::c_int,
                                       mut plus: libc::c_int,
                                       mut dam: libc::c_int,
                                       mut skill: libc::c_int) -> s16b {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    /* Extract "shot" power */
    i =
        weight + ((*p_ptr).to_h as libc::c_int + plus) * 4 as libc::c_int +
            get_skill_scale(skill, 100 as libc::c_int as u32b) as libc::c_int;
    i += 50 as libc::c_int * (*p_ptr).xtra_crit as libc::c_int;
    i += luck(-(100 as libc::c_int), 100 as libc::c_int);
    /* Critical hit */
    if Rand_div(5000 as libc::c_int) + 1 as libc::c_int <= i {
        k = weight + (Rand_div(500 as libc::c_int) + 1 as libc::c_int);
        if k < 500 as libc::c_int {
            msg_print(b"It was a good hit!\x00" as *const u8 as
                          *const libc::c_char);
            dam = 2 as libc::c_int * dam + 5 as libc::c_int
        } else if k < 1000 as libc::c_int {
            msg_print(b"It was a great hit!\x00" as *const u8 as
                          *const libc::c_char);
            dam = 2 as libc::c_int * dam + 10 as libc::c_int
        } else {
            msg_print(b"It was a superb hit!\x00" as *const u8 as
                          *const libc::c_char);
            dam = 3 as libc::c_int * dam + 15 as libc::c_int
        }
    }
    return dam as s16b;
}
/*
 * Critical hits (by player)
 *
 * Factor in weapon weight, total plusses, player level.
 */
#[no_mangle]
pub unsafe extern "C" fn critical_norm(mut weight: libc::c_int,
                                       mut plus: libc::c_int,
                                       mut dam: libc::c_int,
                                       mut weapon_tval: libc::c_int,
                                       mut done_crit: *mut bool_) -> s16b {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut num: libc::c_int =
        Rand_div(5000 as libc::c_int) + 1 as libc::c_int;
    *done_crit = 0 as libc::c_int as bool_;
    /* Extract "blow" power */
    i =
        weight + ((*p_ptr).to_h as libc::c_int + plus) * 5 as libc::c_int +
            get_skill_scale((*p_ptr).melee_style as libc::c_int,
                            150 as libc::c_int as u32b) as libc::c_int;
    i += 50 as libc::c_int * (*p_ptr).xtra_crit as libc::c_int;
    if weapon_tval == 23 as libc::c_int && weight < 50 as libc::c_int &&
           get_skill(52 as libc::c_int) as libc::c_int != 0 {
        i +=
            get_skill_scale(52 as libc::c_int,
                            (40 as libc::c_int * 50 as libc::c_int) as u32b)
                as libc::c_int
    }
    i += luck(-(100 as libc::c_int), 100 as libc::c_int);
    /* Force good strikes */
    if (*p_ptr).tim_deadly != 0 {
        set_tim_deadly((*p_ptr).tim_deadly as libc::c_int - 1 as libc::c_int);
        msg_print(b"It was a *GREAT* hit!\x00" as *const u8 as
                      *const libc::c_char);
        dam = 3 as libc::c_int * dam + 20 as libc::c_int;
        *done_crit = 1 as libc::c_int as bool_
    } else if num <= i {
        k = weight + (Rand_div(650 as libc::c_int) + 1 as libc::c_int);
        if weapon_tval == 23 as libc::c_int && weight < 50 as libc::c_int &&
               get_skill(52 as libc::c_int) as libc::c_int != 0 {
            k +=
                get_skill_scale(52 as libc::c_int, 400 as libc::c_int as u32b)
                    as libc::c_int
        }
        if k < 400 as libc::c_int {
            msg_print(b"It was a good hit!\x00" as *const u8 as
                          *const libc::c_char);
            dam = 2 as libc::c_int * dam + 5 as libc::c_int
        } else if k < 700 as libc::c_int {
            msg_print(b"It was a great hit!\x00" as *const u8 as
                          *const libc::c_char);
            dam = 2 as libc::c_int * dam + 10 as libc::c_int
        } else if k < 900 as libc::c_int {
            msg_print(b"It was a superb hit!\x00" as *const u8 as
                          *const libc::c_char);
            dam = 3 as libc::c_int * dam + 15 as libc::c_int
        } else if k < 1300 as libc::c_int {
            msg_print(b"It was a *GREAT* hit!\x00" as *const u8 as
                          *const libc::c_char);
            dam = 3 as libc::c_int * dam + 20 as libc::c_int
        } else {
            msg_print(b"It was a *SUPERB* hit!\x00" as *const u8 as
                          *const libc::c_char);
            dam =
                7 as libc::c_int * dam / 2 as libc::c_int + 25 as libc::c_int
        }
        *done_crit = 1 as libc::c_int as bool_
    }
    return dam as s16b;
}
/* Chance */
/*
 * Extract the "total damage" from a given object hitting a given monster.
 *
 * Note that "flasks of oil" do NOT do fire damage, although they
 * certainly could be made to do so.  XXX XXX
 *
 * Note that most brands and slays are x3, except Slay Animal (x2),
 * Slay Evil (x2), and Kill dragon (x5).
 */
#[no_mangle]
pub unsafe extern "C" fn tot_dam_aux(mut o_ptr: *mut object_type,
                                     mut tdam: libc::c_int,
                                     mut m_ptr: *mut monster_type,
                                     mut special: *mut s32b) -> s16b {
    let mut mult: libc::c_int = 1 as libc::c_int;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    /* Extract the flags */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* Some "weapons" and "ammo" do extra damage */
    match (*o_ptr).tval as libc::c_int {
        16 | 17 | 18 | 15 | 21 | 22 | 23 | 24 | 20 => {
            /* Slay Animal */
            if f1 as libc::c_long & 0x10000 as libc::c_long != 0 &&
                   (*r_ptr).flags3 & 0x80 as libc::c_int as libc::c_uint != 0
               {
                if (*m_ptr).ml != 0 {
                    (*r_ptr).r_flags3 |= 0x80 as libc::c_int as libc::c_uint
                }
                if mult < 2 as libc::c_int { mult = 2 as libc::c_int }
            }
            /* Slay Evil */
            if f1 as libc::c_long & 0x20000 as libc::c_long != 0 &&
                   (*r_ptr).flags3 & 0x40 as libc::c_int as libc::c_uint != 0
               {
                if (*m_ptr).ml != 0 {
                    (*r_ptr).r_flags3 |= 0x40 as libc::c_int as libc::c_uint
                }
                if mult < 2 as libc::c_int { mult = 2 as libc::c_int }
            }
            /* Slay Undead */
            if f1 as libc::c_long & 0x40000 as libc::c_long != 0 &&
                   (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0
               {
                if (*m_ptr).ml != 0 {
                    (*r_ptr).r_flags3 |= 0x20 as libc::c_int as libc::c_uint
                }
                if mult < 3 as libc::c_int { mult = 3 as libc::c_int }
            }
            /* Slay Demon */
            if f1 as libc::c_long & 0x80000 as libc::c_long != 0 &&
                   (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0
               {
                if (*m_ptr).ml != 0 {
                    (*r_ptr).r_flags3 |= 0x10 as libc::c_int as libc::c_uint
                }
                if mult < 3 as libc::c_int { mult = 3 as libc::c_int }
            }
            /* Slay Orc */
            if f1 as libc::c_long & 0x100000 as libc::c_long != 0 &&
                   (*r_ptr).flags3 & 0x1 as libc::c_int as libc::c_uint != 0 {
                if (*m_ptr).ml != 0 {
                    (*r_ptr).r_flags3 |= 0x1 as libc::c_int as libc::c_uint
                }
                if mult < 3 as libc::c_int { mult = 3 as libc::c_int }
            }
            /* Slay Troll */
            if f1 as libc::c_long & 0x200000 as libc::c_long != 0 &&
                   (*r_ptr).flags3 & 0x2 as libc::c_int as libc::c_uint != 0 {
                if (*m_ptr).ml != 0 {
                    (*r_ptr).r_flags3 |= 0x2 as libc::c_int as libc::c_uint
                }
                if mult < 3 as libc::c_int { mult = 3 as libc::c_int }
            }
            /* Slay Giant */
            if f1 as libc::c_long & 0x400000 as libc::c_long != 0 &&
                   (*r_ptr).flags3 & 0x4 as libc::c_int as libc::c_uint != 0 {
                if (*m_ptr).ml != 0 {
                    (*r_ptr).r_flags3 |= 0x4 as libc::c_int as libc::c_uint
                }
                if mult < 3 as libc::c_int { mult = 3 as libc::c_int }
            }
            /* Slay Dragon  */
            if f1 as libc::c_long & 0x800000 as libc::c_long != 0 &&
                   (*r_ptr).flags3 & 0x8 as libc::c_int as libc::c_uint != 0 {
                if (*m_ptr).ml != 0 {
                    (*r_ptr).r_flags3 |= 0x8 as libc::c_int as libc::c_uint
                }
                if mult < 3 as libc::c_int { mult = 3 as libc::c_int }
            }
            /* Execute Dragon */
            if f1 as libc::c_long & 0x1000000 as libc::c_long != 0 &&
                   (*r_ptr).flags3 & 0x8 as libc::c_int as libc::c_uint != 0 {
                if (*m_ptr).ml != 0 {
                    (*r_ptr).r_flags3 |= 0x8 as libc::c_int as libc::c_uint
                }
                if mult < 5 as libc::c_int { mult = 5 as libc::c_int }
            }
            /* Execute Undead */
            if f5 as libc::c_long & 0x10 as libc::c_long != 0 &&
                   (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0
               {
                if (*m_ptr).ml != 0 {
                    (*r_ptr).r_flags3 |= 0x20 as libc::c_int as libc::c_uint
                }
                if mult < 5 as libc::c_int { mult = 5 as libc::c_int }
            }
            /* Execute Demon */
            if f5 as libc::c_long & 0x8 as libc::c_long != 0 &&
                   (*r_ptr).flags3 & 0x10 as libc::c_int as libc::c_uint != 0
               {
                if (*m_ptr).ml != 0 {
                    (*r_ptr).r_flags3 |= 0x10 as libc::c_int as libc::c_uint
                }
                if mult < 5 as libc::c_int { mult = 5 as libc::c_int }
            }
            /* Brand (Acid) */
            if f1 as libc::c_long & 0x10000000 as libc::c_long != 0 {
                /* Notice immunity */
                if (*r_ptr).flags3 & 0x10000 as libc::c_int as libc::c_uint !=
                       0 {
                    if (*m_ptr).ml != 0 {
                        (*r_ptr).r_flags3 |=
                            0x10000 as libc::c_int as libc::c_uint
                    }
                } else if (*r_ptr).flags9 &
                              0x40 as libc::c_int as libc::c_uint != 0 {
                    if (*m_ptr).ml != 0 {
                        (*r_ptr).r_flags9 |=
                            0x40 as libc::c_int as libc::c_uint
                    }
                    if mult < 6 as libc::c_int { mult = 6 as libc::c_int }
                } else if mult < 3 as libc::c_int { mult = 3 as libc::c_int }
            }
            /* Notice susceptibility */
            /* Otherwise, take the damage */
            /* Brand (Elec) */
            if f1 as libc::c_long & 0x20000000 as libc::c_long != 0 {
                /* Notice immunity */
                if (*r_ptr).flags3 & 0x20000 as libc::c_int as libc::c_uint !=
                       0 {
                    if (*m_ptr).ml != 0 {
                        (*r_ptr).r_flags3 |=
                            0x20000 as libc::c_int as libc::c_uint
                    }
                } else if (*r_ptr).flags9 &
                              0x80 as libc::c_int as libc::c_uint != 0 {
                    if (*m_ptr).ml != 0 {
                        (*r_ptr).r_flags9 |=
                            0x80 as libc::c_int as libc::c_uint
                    }
                    if mult < 6 as libc::c_int { mult = 6 as libc::c_int }
                } else if mult < 3 as libc::c_int { mult = 3 as libc::c_int }
            }
            /* Notice susceptibility */
            /* Otherwise, take the damage */
            /* Brand (Fire) */
            if f1 as libc::c_long & 0x40000000 as libc::c_long != 0 {
                /* Notice immunity */
                if (*r_ptr).flags3 & 0x40000 as libc::c_int as libc::c_uint !=
                       0 {
                    if (*m_ptr).ml != 0 {
                        (*r_ptr).r_flags3 |=
                            0x40000 as libc::c_int as libc::c_uint
                    }
                } else if (*r_ptr).flags3 &
                              0x4000 as libc::c_int as libc::c_uint != 0 {
                    if (*m_ptr).ml != 0 {
                        (*r_ptr).r_flags3 |=
                            0x4000 as libc::c_int as libc::c_uint
                    }
                    if mult < 6 as libc::c_int { mult = 6 as libc::c_int }
                } else if mult < 3 as libc::c_int { mult = 3 as libc::c_int }
            }
            /* Notice susceptibility */
            /* Otherwise, take the damage */
            /* Brand (Cold) */
            if f1 as libc::c_long & 0x80000000 as libc::c_long != 0 {
                /* Notice immunity */
                if (*r_ptr).flags3 & 0x80000 as libc::c_int as libc::c_uint !=
                       0 {
                    if (*m_ptr).ml != 0 {
                        (*r_ptr).r_flags3 |=
                            0x80000 as libc::c_int as libc::c_uint
                    }
                } else if (*r_ptr).flags3 &
                              0x8000 as libc::c_int as libc::c_uint != 0 {
                    if (*m_ptr).ml != 0 {
                        (*r_ptr).r_flags3 |=
                            0x8000 as libc::c_int as libc::c_uint
                    }
                    if mult < 6 as libc::c_int { mult = 6 as libc::c_int }
                } else if mult < 3 as libc::c_int { mult = 3 as libc::c_int }
            }
            /* Notice susceptibility */
            /* Otherwise, take the damage */
            /* Brand (Poison) */
            if f1 as libc::c_long & 0x8000000 as libc::c_long != 0 ||
                   (*p_ptr).tim_poison as libc::c_int != 0 {
                /* Notice immunity */
                if (*r_ptr).flags3 & 0x100000 as libc::c_int as libc::c_uint
                       != 0 {
                    if (*m_ptr).ml != 0 {
                        (*r_ptr).r_flags3 |=
                            0x100000 as libc::c_int as libc::c_uint
                    }
                } else if (*r_ptr).flags9 &
                              0x100 as libc::c_int as libc::c_uint != 0 {
                    if (*m_ptr).ml != 0 {
                        (*r_ptr).r_flags9 |=
                            0x100 as libc::c_int as libc::c_uint
                    }
                    if mult < 6 as libc::c_int { mult = 6 as libc::c_int }
                    if Rand_div(100 as libc::c_int) < 95 as libc::c_int {
                        *special =
                            (*special as libc::c_long | 0x1 as libc::c_long)
                                as s32b
                    }
                } else {
                    /* Notice susceptibility */
                    /* Otherwise, take the damage */
                    if mult < 3 as libc::c_int { mult = 3 as libc::c_int }
                    if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
                        *special =
                            (*special as libc::c_long | 0x1 as libc::c_long)
                                as s32b
                    }
                }
            }
            /* Wounding */
            if f5 as libc::c_long & 0x80 as libc::c_long != 0 {
                /* Notice immunity */
                if (*r_ptr).flags8 & 0x800 as libc::c_int as libc::c_uint != 0
                   {
                    if (*m_ptr).ml != 0 {
                        let ref mut fresh0 =
                            (*r_info.offset((*m_ptr).r_idx as
                                                isize)).r_flags8;
                        *fresh0 |= 0x800 as libc::c_int as libc::c_uint
                    }
                } else if Rand_div(100 as libc::c_int) < 50 as libc::c_int {
                    *special =
                        (*special as libc::c_long | 0x2 as libc::c_long) as
                            s32b
                }
            }
        }
        _ => { }
    }
    /* Otherwise, take the damage */
    /* Return the total damage */
    return (tdam * mult) as s16b;
}
/*
 * Search for hidden things
 */
#[no_mangle]
pub unsafe extern "C" fn search() {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut chance: libc::c_int = 0;
    let mut this_o_idx: s16b = 0;
    let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Start with base search ability */
    chance = (*p_ptr).skill_srh as libc::c_int;
    /* Penalize various conditions */
    if (*p_ptr).blind as libc::c_int != 0 || no_lite() as libc::c_int != 0 {
        chance = chance / 10 as libc::c_int
    }
    if (*p_ptr).confused as libc::c_int != 0 ||
           (*p_ptr).image as libc::c_int != 0 {
        chance = chance / 10 as libc::c_int
    }
    /* Search the nearby grids, which are always in bounds */
    y = (*p_ptr).py as libc::c_int - 1 as libc::c_int;
    while y <= (*p_ptr).py as libc::c_int + 1 as libc::c_int {
        x = (*p_ptr).px as libc::c_int - 1 as libc::c_int;
        while x <= (*p_ptr).px as libc::c_int + 1 as libc::c_int {
            /* Sometimes, notice things */
            if Rand_div(100 as libc::c_int) < chance {
                /* Access the grid */
                c_ptr =
                    &mut *(*cave.as_mut_ptr().offset(y as
                                                         isize)).offset(x as
                                                                            isize)
                        as *mut cave_type;
                /* Invisible trap */
                if (*c_ptr).t_idx as libc::c_int != 0 as libc::c_int &&
                       (*c_ptr).info as libc::c_int & 0x100 as libc::c_int ==
                           0 {
                    /* Pick a trap */
                    pick_trap(y, x);
                    /* Message */
                    msg_print(b"You have found a trap.\x00" as *const u8 as
                                  *const libc::c_char);
                    /* Disturb */
                    disturb(0 as libc::c_int, 0 as libc::c_int);
                }
                /* Secret door */
                if (*c_ptr).feat as libc::c_int == 0x30 as libc::c_int {
                    /* Message */
                    msg_print(b"You have found a secret door.\x00" as
                                  *const u8 as *const libc::c_char);
                    /* Pick a door XXX XXX XXX */
                    cave_set_feat(y, x,
                                  0x20 as libc::c_int + 0 as libc::c_int);
                    (*cave[y as usize].offset(x as isize)).mimic =
                        0 as libc::c_int as byte_hack;
                    lite_spot(y, x);
                    /* Disturb */
                    disturb(0 as libc::c_int, 0 as libc::c_int);
                }
                /* Scan all objects in the grid */
                this_o_idx = (*c_ptr).o_idx;
                while this_o_idx != 0 {
                    let mut o_ptr: *mut object_type = 0 as *mut object_type;
                    /* Acquire object */
                    o_ptr =
                        &mut *o_list.offset(this_o_idx as isize) as
                            *mut object_type;
                    /* Acquire next object */
                    next_o_idx = (*o_ptr).next_o_idx;
                    /* Skip non-chests */
                    if !((*o_ptr).tval as libc::c_int != 7 as libc::c_int) {
                        /* Skip non-trapped chests */
                        if !((*o_ptr).pval == 0) {
                            /* Identify once */
                            if !((*o_ptr).ident as libc::c_int &
                                     0x8 as libc::c_int != 0 ||
                                     (*k_info.offset((*o_ptr).k_idx as
                                                         isize)).easy_know as
                                         libc::c_int != 0 &&
                                         (*k_info.offset((*o_ptr).k_idx as
                                                             isize)).aware as
                                             libc::c_int != 0) {
                                /* Message */
                                msg_print(b"You have discovered a trap on the chest!\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                /* Know the trap */
                                object_known(o_ptr);
                                /* Notice it */
                                disturb(0 as libc::c_int, 0 as libc::c_int);
                            }
                        }
                    }
                    this_o_idx = next_o_idx
                }
            }
            x += 1
        }
        y += 1
    };
}
/*
 * Player "wants" to pick up an object or gold.
 * Note that we ONLY handle things that can be picked up.
 * See "move_player()" for handling of other things.
 */
#[no_mangle]
pub unsafe extern "C" fn carry(mut pickup: libc::c_int) {
    if (*p_ptr).disembodied == 0 { py_pickup_floor(pickup); };
}
/*
 * Handle player hitting a real trap
 */
unsafe extern "C" fn hit_trap() {
    let mut ident: bool_ = 0 as libc::c_int as bool_;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Disturb the player */
    disturb(0 as libc::c_int, 0 as libc::c_int);
    /* Get the cave grid */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset((*p_ptr).py as
                                             isize)).offset((*p_ptr).px as
                                                                isize) as
            *mut cave_type;
    if (*c_ptr).t_idx as libc::c_int != 0 as libc::c_int {
        ident =
            player_activate_trap_type((*p_ptr).py, (*p_ptr).px,
                                      0 as *mut object_type,
                                      -(1 as libc::c_int) as s16b);
        if ident != 0 {
            (*t_info.offset((*c_ptr).t_idx as isize)).ident =
                1 as libc::c_int as bool_;
            msg_format(b"You identified the trap as %s.\x00" as *const u8 as
                           *const libc::c_char,
                       t_name.offset((*t_info.offset((*c_ptr).t_idx as
                                                         isize)).name as
                                         libc::c_int as isize));
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn touch_zap_player(mut m_ptr: *mut monster_type) {
    let mut aura_damage: libc::c_int = 0 as libc::c_int;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    if (*r_ptr).flags2 & 0x4000 as libc::c_int as libc::c_uint != 0 {
        if (*p_ptr).immune_fire == 0 {
            let mut aura_dam: [libc::c_char; 80] = [0; 80];
            aura_damage =
                damroll((1 as libc::c_int +
                             (*m_ptr).level as libc::c_int /
                                 26 as libc::c_int) as s16b,
                        (1 as libc::c_int +
                             (*m_ptr).level as libc::c_int /
                                 17 as libc::c_int) as s16b);
            /* Hack -- Get the "died from" name */
            monster_desc(aura_dam.as_mut_ptr(), m_ptr, 0x88 as libc::c_int);
            msg_print(b"You are suddenly very hot!\x00" as *const u8 as
                          *const libc::c_char);
            if (*p_ptr).oppose_fire != 0 {
                aura_damage =
                    (aura_damage + 2 as libc::c_int) / 3 as libc::c_int
            }
            if (*p_ptr).resist_fire != 0 {
                aura_damage =
                    (aura_damage + 2 as libc::c_int) / 3 as libc::c_int
            }
            if (*p_ptr).sensible_fire != 0 {
                aura_damage =
                    (aura_damage + 2 as libc::c_int) * 2 as libc::c_int
            }
            take_hit(aura_damage, aura_dam.as_mut_ptr() as cptr);
            (*r_ptr).r_flags2 |= 0x4000 as libc::c_int as libc::c_uint;
            handle_stuff();
        }
    }
    if (*r_ptr).flags2 & 0x8000 as libc::c_int as libc::c_uint != 0 {
        if (*p_ptr).immune_elec == 0 {
            let mut aura_dam_0: [libc::c_char; 80] = [0; 80];
            aura_damage =
                damroll((1 as libc::c_int +
                             (*m_ptr).level as libc::c_int /
                                 26 as libc::c_int) as s16b,
                        (1 as libc::c_int +
                             (*m_ptr).level as libc::c_int /
                                 17 as libc::c_int) as s16b);
            /* Hack -- Get the "died from" name */
            monster_desc(aura_dam_0.as_mut_ptr(), m_ptr, 0x88 as libc::c_int);
            if (*p_ptr).oppose_elec != 0 {
                aura_damage =
                    (aura_damage + 2 as libc::c_int) / 3 as libc::c_int
            }
            if (*p_ptr).resist_elec != 0 {
                aura_damage =
                    (aura_damage + 2 as libc::c_int) / 3 as libc::c_int
            }
            msg_print(b"You get zapped!\x00" as *const u8 as
                          *const libc::c_char);
            take_hit(aura_damage, aura_dam_0.as_mut_ptr() as cptr);
            (*r_ptr).r_flags2 |= 0x8000 as libc::c_int as libc::c_uint;
            handle_stuff();
        }
    };
}
/*
 * Carried monster can attack too.
 * Based on monst_attack_monst.
 */
unsafe extern "C" fn carried_monster_attack(mut m_idx: s16b,
                                            mut fear: *mut bool_,
                                            mut mdeath: *mut bool_,
                                            mut x: libc::c_int,
                                            mut y: libc::c_int) {
    let mut t_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
    let mut tr_ptr: *mut monster_race =
        if !(*t_ptr).sr_ptr.is_null() {
            (*t_ptr).sr_ptr
        } else {
            race_info_idx((*t_ptr).r_idx as libc::c_int,
                          (*t_ptr).ego as libc::c_int)
        };
    let mut ap_cnt: libc::c_int = 0;
    let mut ac: libc::c_int = 0;
    let mut rlev: libc::c_int = 0;
    let mut pt: libc::c_int = 0;
    let mut t_name_0: [libc::c_char; 80] = [0; 80];
    let mut sym_name: cptr = symbiote_name(1 as libc::c_int as bool_);
    let mut temp: [libc::c_char; 80] = [0; 80];
    let mut blinked: bool_ = 0 as libc::c_int as bool_;
    let mut touched: bool_ = 0 as libc::c_int as bool_;
    let mut y_saver: byte_hack = (*t_ptr).fy;
    let mut x_saver: byte_hack = (*t_ptr).fx;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    /* Get the carried monster */
    o_ptr =
        &mut *(*p_ptr).inventory.as_mut_ptr().offset(49 as libc::c_int as
                                                         isize) as
            *mut object_type;
    if (*o_ptr).k_idx == 0 { return }
    r_ptr = &mut *r_info.offset((*o_ptr).pval as isize) as *mut monster_race;
    /* Not allowed to attack */
    if (*r_ptr).flags1 & 0x10000 as libc::c_int as libc::c_uint != 0 {
        return
    }
    /* Total armor */
    ac = (*t_ptr).ac as libc::c_int;
    /* Extract the effective monster level */
    rlev =
        if (*o_ptr).elevel as libc::c_int >= 1 as libc::c_int {
            (*o_ptr).elevel as libc::c_int
        } else { 1 as libc::c_int };
    /* Get the monster name (or "it") */
    monster_desc(t_name_0.as_mut_ptr(), t_ptr, 0 as libc::c_int);
    /* Assume no blink */
    blinked = 0 as libc::c_int as bool_;
    if (*t_ptr).ml == 0 {
        msg_print(b"You hear noise.\x00" as *const u8 as *const libc::c_char);
    }
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
        /* Stop attacking if the target dies! */
        if (*t_ptr).fx as libc::c_int != x_saver as libc::c_int ||
               (*t_ptr).fy as libc::c_int != y_saver as libc::c_int {
            break ;
        }
        /* Hack -- no more attacks */
        if method == 0 { break ; }
        (blinked) != 0;
        /* Extract visibility (before blink) */
        visible = 1 as libc::c_int as bool_;
        /* Extract the attack "power" */
        power = get_attack_power(effect);
        /* Monster hits */
        if effect == 0 || check_hit2(power, rlev, ac) != 0 {
            /* Always disturbing */
            disturb(1 as libc::c_int, 0 as libc::c_int);
            /* Describe the attack method */
            match method {
                1 => {
                    act = b"hits %s.\x00" as *const u8 as *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                2 => {
                    act =
                        b"touches %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                3 => {
                    act =
                        b"punches %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                4 => {
                    act =
                        b"kicks %s.\x00" as *const u8 as *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                5 => {
                    act =
                        b"claws %s.\x00" as *const u8 as *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                6 => {
                    act =
                        b"bites %s.\x00" as *const u8 as *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                7 => {
                    act =
                        b"stings %s.\x00" as *const u8 as *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                8 => {
                    act =
                        b"XXX1\'s %s.\x00" as *const u8 as *const libc::c_char
                }
                9 => {
                    act =
                        b"butts %s.\x00" as *const u8 as *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                10 => {
                    act =
                        b"crushes %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                11 => {
                    act =
                        b"engulfs %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                12 => {
                    act =
                        b"charges %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                13 => {
                    act =
                        b"crawls on %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                14 => {
                    act =
                        b"drools on %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 0 as libc::c_int as bool_
                }
                15 => {
                    act =
                        b"spits on %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 0 as libc::c_int as bool_
                }
                17 => {
                    act =
                        b"gazes at %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 0 as libc::c_int as bool_
                }
                18 => {
                    act =
                        b"wails at %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 0 as libc::c_int as bool_
                }
                19 => {
                    act =
                        b"releases spores at %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 0 as libc::c_int as bool_
                }
                20 => {
                    act =
                        b"projects XXX4\'s at %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 0 as libc::c_int as bool_
                }
                21 => {
                    act =
                        b"begs %s for money.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 0 as libc::c_int as bool_;
                    (*t_ptr).csleep = 0 as libc::c_int as s16b
                }
                22 => {
                    act =
                        b"insults %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 0 as libc::c_int as bool_;
                    (*t_ptr).csleep = 0 as libc::c_int as s16b
                }
                23 => {
                    act =
                        b"moans at %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 0 as libc::c_int as bool_;
                    (*t_ptr).csleep = 0 as libc::c_int as s16b
                }
                24 => {
                    act =
                        b"sings to %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 0 as libc::c_int as bool_;
                    (*t_ptr).csleep = 0 as libc::c_int as s16b
                }
                _ => { }
            }
            /* Message */
            if !act.is_null() {
                strfmt(temp.as_mut_ptr(), act, t_name_0.as_mut_ptr());
                if (*t_ptr).ml != 0 {
                    msg_format(b"%s %s\x00" as *const u8 as
                                   *const libc::c_char, sym_name,
                               temp.as_mut_ptr());
                }
            }
            /* Hack -- assume all attacks are obvious */
            obvious = 1 as libc::c_int as bool_;
            /* Roll out the damage */
            damage = damroll(d_dice as s16b, d_side as s16b);
            pt = 10 as libc::c_int;
            /* Apply appropriate damage */
            match effect {
                0 => {
                    damage = 0 as libc::c_int; /* sort of close... */
                    pt = 0 as libc::c_int
                }
                1 | 31 => {
                    damage -=
                        damage *
                            (if ac < 150 as libc::c_int {
                                 ac
                             } else { 150 as libc::c_int }) /
                            250 as libc::c_int
                }
                2 | 29 => { pt = 2 as libc::c_int }
                3 | 4 | 34 => { pt = 32 as libc::c_int }
                7 | 8 => { damage = 0 as libc::c_int; pt = damage }
                6 | 5 => {
                    damage = 0 as libc::c_int;
                    pt = damage;
                    if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                        blinked = 1 as libc::c_int as bool_
                    }
                }
                9 => { pt = 3 as libc::c_int }
                10 => { pt = 1 as libc::c_int }
                11 => { pt = 5 as libc::c_int }
                12 => { pt = 4 as libc::c_int }
                14 | 32 => { pt = 22 as libc::c_int }
                15 => { pt = 66 as libc::c_int }
                16 => { pt = 57 as libc::c_int }
                13 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 33 => { }
                24 => {
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
                25 | 26 | 27 | 28 => { pt = 31 as libc::c_int }
                30 => { pt = 34 as libc::c_int }
                _ => { pt = 0 as libc::c_int }
            }
            if pt != 0 {
                /* Do damage if not exploding */
                project(0 as libc::c_int, 0 as libc::c_int,
                        (*t_ptr).fy as libc::c_int,
                        (*t_ptr).fx as libc::c_int,
                        if pt == 57 as libc::c_int {
                            (*r_ptr).level as libc::c_int
                        } else { damage }, pt,
                        0x40 as libc::c_int | 0x8 as libc::c_int);
                if touched != 0 {
                    /* Aura fire */
                    if (*tr_ptr).flags2 &
                           0x4000 as libc::c_int as libc::c_uint != 0 &&
                           (*r_ptr).flags3 &
                               0x40000 as libc::c_int as libc::c_uint == 0 {
                        if (*t_ptr).ml != 0 {
                            blinked = 0 as libc::c_int as bool_;
                            msg_format(b"You are suddenly very hot!\x00" as
                                           *const u8 as *const libc::c_char);
                            if (*t_ptr).ml != 0 {
                                (*tr_ptr).r_flags2 |=
                                    0x4000 as libc::c_int as libc::c_uint
                            }
                        }
                        project(m_idx as libc::c_int, 0 as libc::c_int,
                                (*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                damroll((1 as libc::c_int +
                                             (*t_ptr).level as libc::c_int /
                                                 26 as libc::c_int) as s16b,
                                        (1 as libc::c_int +
                                             (*t_ptr).level as libc::c_int /
                                                 17 as libc::c_int) as s16b),
                                5 as libc::c_int,
                                0x40 as libc::c_int | 0x8 as libc::c_int);
                    }
                    /* Aura elec */
                    if (*tr_ptr).flags2 &
                           0x8000 as libc::c_int as libc::c_uint != 0 &&
                           (*r_ptr).flags3 &
                               0x20000 as libc::c_int as libc::c_uint == 0 {
                        if (*t_ptr).ml != 0 {
                            blinked = 0 as libc::c_int as bool_;
                            msg_format(b"You get zapped!\x00" as *const u8 as
                                           *const libc::c_char);
                            if (*t_ptr).ml != 0 {
                                (*tr_ptr).r_flags2 |=
                                    0x8000 as libc::c_int as libc::c_uint
                            }
                        }
                        project(m_idx as libc::c_int, 0 as libc::c_int,
                                (*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                damroll((1 as libc::c_int +
                                             (*t_ptr).level as libc::c_int /
                                                 26 as libc::c_int) as s16b,
                                        (1 as libc::c_int +
                                             (*t_ptr).level as libc::c_int /
                                                 17 as libc::c_int) as s16b),
                                1 as libc::c_int,
                                0x40 as libc::c_int | 0x8 as libc::c_int);
                    }
                }
            }
        } else {
            /* Monster missed player */
            /* Analyze failed attacks */
            match method {
                1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 => {
                    /* Disturb */
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    /* Message */
                    msg_format(b"%s misses %s.\x00" as *const u8 as
                                   *const libc::c_char, sym_name,
                               t_name_0.as_mut_ptr());
                }
                _ => { }
            }
        }
        /* Analyze "visible" monsters only */
        if visible != 0 {
            /* Count "obvious" attacks (and ones that cause damage) */
            if obvious as libc::c_int != 0 || damage != 0 ||
                   (*r_ptr).r_blows[ap_cnt as usize] as libc::c_int >
                       10 as libc::c_int {
                /* Count attacks of this type */
                if ((*r_ptr).r_blows[ap_cnt as usize] as libc::c_int) <
                       255 as libc::c_int {
                    (*r_ptr).r_blows[ap_cnt as usize] =
                        (*r_ptr).r_blows[ap_cnt as usize].wrapping_add(1)
                }
            }
        }
        ap_cnt += 1
    }
    /* Blink away */
    if blinked != 0 {
        msg_format(b"You and %s flee laughing!\x00" as *const u8 as
                       *const libc::c_char,
                   symbiote_name(0 as libc::c_int as bool_));
        teleport_player(20 as libc::c_int * 2 as libc::c_int +
                            5 as libc::c_int);
    };
}
/*
 * Carried monster can attack too.
 * Based on monst_attack_monst.
 */
unsafe extern "C" fn incarnate_monster_attack(mut m_idx: s16b,
                                              mut fear: *mut bool_,
                                              mut mdeath: *mut bool_,
                                              mut x: libc::c_int,
                                              mut y: libc::c_int) {
    let mut t_ptr: *mut monster_type =
        &mut *m_list.offset(m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
    let mut tr_ptr: *mut monster_race =
        if !(*t_ptr).sr_ptr.is_null() {
            (*t_ptr).sr_ptr
        } else {
            race_info_idx((*t_ptr).r_idx as libc::c_int,
                          (*t_ptr).ego as libc::c_int)
        };
    let mut ap_cnt: libc::c_int = 0;
    let mut ac: libc::c_int = 0;
    let mut rlev: libc::c_int = 0;
    let mut pt: libc::c_int = 0;
    let mut t_name_0: [libc::c_char; 80] = [0; 80];
    let mut temp: [libc::c_char; 80] = [0; 80];
    let mut blinked: bool_ = 0 as libc::c_int as bool_;
    let mut touched: bool_ = 0 as libc::c_int as bool_;
    let mut y_saver: byte_hack = (*t_ptr).fy;
    let mut x_saver: byte_hack = (*t_ptr).fx;
    if (*p_ptr).body_monster == 0 { return }
    r_ptr =
        race_info_idx((*p_ptr).body_monster as libc::c_int, 0 as libc::c_int);
    /* Not allowed to attack */
    if (*r_ptr).flags1 & 0x10000 as libc::c_int as libc::c_uint != 0 {
        return
    }
    /* Total armor */
    ac = (*t_ptr).ac as libc::c_int;
    /* Extract the effective monster level */
    rlev =
        if (*r_ptr).level as libc::c_int >= 1 as libc::c_int {
            (*r_ptr).level as libc::c_int
        } else { 1 as libc::c_int };
    /* Get the monster name (or "it") */
    monster_desc(t_name_0.as_mut_ptr(), t_ptr, 0 as libc::c_int);
    /* Assume no blink */
    blinked = 0 as libc::c_int as bool_;
    if (*t_ptr).ml == 0 {
        msg_print(b"You hear noise.\x00" as *const u8 as *const libc::c_char);
    }
    /* Scan through all four blows */
    ap_cnt = 0 as libc::c_int;
    while if ap_cnt <
                 ((*p_ptr).num_blow as libc::c_int > 4 as libc::c_int) as
                     libc::c_int {
              4 as libc::c_int
          } else { (*p_ptr).num_blow as libc::c_int } != 0 {
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
        /* Stop attacking if the target dies! */
        if (*t_ptr).fx as libc::c_int != x_saver as libc::c_int ||
               (*t_ptr).fy as libc::c_int != y_saver as libc::c_int {
            break ;
        }
        /* Hack -- no more attacks */
        if method == 0 { break ; }
        (blinked) != 0;
        /* Extract visibility (before blink) */
        visible = 1 as libc::c_int as bool_;
        /* Extract the attack "power" */
        effect = get_attack_power(effect);
        /* Monster hits */
        if effect == 0 || check_hit2(power, rlev, ac) != 0 {
            /* Always disturbing */
            disturb(1 as libc::c_int, 0 as libc::c_int);
            /* Describe the attack method */
            match method {
                1 => {
                    act = b"hit %s.\x00" as *const u8 as *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                2 => {
                    act =
                        b"touch %s.\x00" as *const u8 as *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                3 => {
                    act =
                        b"punch %s.\x00" as *const u8 as *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                4 => {
                    act = b"kick %s.\x00" as *const u8 as *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                5 => {
                    act = b"claw %s.\x00" as *const u8 as *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                6 => {
                    act = b"bite %s.\x00" as *const u8 as *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                7 => {
                    act =
                        b"sting %s.\x00" as *const u8 as *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                8 => {
                    act =
                        b"XXX1\'s %s.\x00" as *const u8 as *const libc::c_char
                }
                9 => {
                    act = b"butt %s.\x00" as *const u8 as *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                10 => {
                    act =
                        b"crush %s.\x00" as *const u8 as *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                11 => {
                    act =
                        b"engulf %s.\x00" as *const u8 as *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                12 => {
                    act =
                        b"charge %s.\x00" as *const u8 as *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                13 => {
                    act =
                        b"crawl on %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 1 as libc::c_int as bool_
                }
                14 => {
                    act =
                        b"drool on %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 0 as libc::c_int as bool_
                }
                15 => {
                    act =
                        b"spit on %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 0 as libc::c_int as bool_
                }
                17 => {
                    act =
                        b"gaze at %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 0 as libc::c_int as bool_
                }
                18 => {
                    act =
                        b"wail at %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 0 as libc::c_int as bool_
                }
                19 => {
                    act =
                        b"release spores at %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 0 as libc::c_int as bool_
                }
                20 => {
                    act =
                        b"project XXX4\'s at %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 0 as libc::c_int as bool_
                }
                21 => {
                    act =
                        b"beg %s for money.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 0 as libc::c_int as bool_;
                    (*t_ptr).csleep = 0 as libc::c_int as s16b
                }
                22 => {
                    act =
                        b"insult %s.\x00" as *const u8 as *const libc::c_char;
                    touched = 0 as libc::c_int as bool_;
                    (*t_ptr).csleep = 0 as libc::c_int as s16b
                }
                23 => {
                    act =
                        b"moan at %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 0 as libc::c_int as bool_;
                    (*t_ptr).csleep = 0 as libc::c_int as s16b
                }
                24 => {
                    act =
                        b"sing to %s.\x00" as *const u8 as
                            *const libc::c_char;
                    touched = 0 as libc::c_int as bool_;
                    (*t_ptr).csleep = 0 as libc::c_int as s16b
                }
                _ => { }
            }
            /* Message */
            if !act.is_null() {
                strfmt(temp.as_mut_ptr(), act, t_name_0.as_mut_ptr());
                if (*t_ptr).ml != 0 {
                    msg_format(b"You %s\x00" as *const u8 as
                                   *const libc::c_char, temp.as_mut_ptr());
                }
            }
            /* Hack -- assume all attacks are obvious */
            obvious = 1 as libc::c_int as bool_;
            /* Roll out the damage */
            damage =
                damroll(d_dice as s16b, d_side as s16b) +
                    (*p_ptr).to_d as libc::c_int;
            pt = 10 as libc::c_int;
            /* Apply appropriate damage */
            match effect {
                0 => {
                    damage = 0 as libc::c_int; /* sort of close... */
                    pt = 0 as libc::c_int
                }
                1 | 31 => {
                    damage -=
                        damage *
                            (if ac < 150 as libc::c_int {
                                 ac
                             } else { 150 as libc::c_int }) /
                            250 as libc::c_int
                }
                2 | 29 => { pt = 2 as libc::c_int }
                3 | 4 => { pt = 32 as libc::c_int }
                7 | 8 => { damage = 0 as libc::c_int; pt = damage }
                6 | 5 => {
                    damage = 0 as libc::c_int;
                    pt = damage;
                    if Rand_div(2 as libc::c_int) + 1 as libc::c_int ==
                           1 as libc::c_int {
                        blinked = 1 as libc::c_int as bool_
                    }
                }
                9 => { pt = 3 as libc::c_int }
                10 => { pt = 1 as libc::c_int }
                11 => { pt = 5 as libc::c_int }
                12 => { pt = 4 as libc::c_int }
                32 | 14 => { pt = 22 as libc::c_int }
                15 => { pt = 66 as libc::c_int }
                16 => { pt = 57 as libc::c_int }
                13 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 33 => { }
                24 => {
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
                25 | 26 | 27 | 28 => { pt = 31 as libc::c_int }
                30 => { pt = 34 as libc::c_int }
                _ => { pt = 0 as libc::c_int }
            }
            if pt != 0 {
                /* Do damage if not exploding */
                project(0 as libc::c_int, 0 as libc::c_int,
                        (*t_ptr).fy as libc::c_int,
                        (*t_ptr).fx as libc::c_int,
                        if pt == 57 as libc::c_int {
                            ((*p_ptr).lev as libc::c_int) * 2 as libc::c_int
                        } else { damage }, pt,
                        0x40 as libc::c_int | 0x8 as libc::c_int);
                if touched != 0 {
                    /* Aura fire */
                    if (*tr_ptr).flags2 &
                           0x4000 as libc::c_int as libc::c_uint != 0 &&
                           (*r_ptr).flags3 &
                               0x40000 as libc::c_int as libc::c_uint == 0 {
                        if (*t_ptr).ml != 0 {
                            blinked = 0 as libc::c_int as bool_;
                            msg_format(b"You are suddenly very hot!\x00" as
                                           *const u8 as *const libc::c_char);
                            if (*t_ptr).ml != 0 {
                                (*tr_ptr).r_flags2 |=
                                    0x4000 as libc::c_int as libc::c_uint
                            }
                        }
                        project(m_idx as libc::c_int, 0 as libc::c_int,
                                (*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                damroll((1 as libc::c_int +
                                             (*t_ptr).level as libc::c_int /
                                                 26 as libc::c_int) as s16b,
                                        (1 as libc::c_int +
                                             (*t_ptr).level as libc::c_int /
                                                 17 as libc::c_int) as s16b),
                                5 as libc::c_int,
                                0x40 as libc::c_int | 0x8 as libc::c_int);
                    }
                    /* Aura elec */
                    if (*tr_ptr).flags2 &
                           0x8000 as libc::c_int as libc::c_uint != 0 &&
                           (*r_ptr).flags3 &
                               0x20000 as libc::c_int as libc::c_uint == 0 {
                        if (*t_ptr).ml != 0 {
                            blinked = 0 as libc::c_int as bool_;
                            msg_format(b"You get zapped!\x00" as *const u8 as
                                           *const libc::c_char);
                            if (*t_ptr).ml != 0 {
                                (*tr_ptr).r_flags2 |=
                                    0x8000 as libc::c_int as libc::c_uint
                            }
                        }
                        project(m_idx as libc::c_int, 0 as libc::c_int,
                                (*p_ptr).py as libc::c_int,
                                (*p_ptr).px as libc::c_int,
                                damroll((1 as libc::c_int +
                                             (*t_ptr).level as libc::c_int /
                                                 26 as libc::c_int) as s16b,
                                        (1 as libc::c_int +
                                             (*t_ptr).level as libc::c_int /
                                                 17 as libc::c_int) as s16b),
                                1 as libc::c_int,
                                0x40 as libc::c_int | 0x8 as libc::c_int);
                    }
                }
            }
        } else {
            /* Monster missed player */
            /* Analyze failed attacks */
            match method {
                1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 => {
                    /* Disturb */
                    disturb(1 as libc::c_int, 0 as libc::c_int);
                    /* Message */
                    msg_format(b"You miss %s.\x00" as *const u8 as
                                   *const libc::c_char,
                               t_name_0.as_mut_ptr());
                }
                _ => { }
            }
        }
        /* Analyze "visible" monsters only */
        if visible != 0 {
            /* Count "obvious" attacks (and ones that cause damage) */
            if obvious as libc::c_int != 0 || damage != 0 ||
                   (*r_ptr).r_blows[ap_cnt as usize] as libc::c_int >
                       10 as libc::c_int {
                /* Count attacks of this type */
                if ((*r_ptr).r_blows[ap_cnt as usize] as libc::c_int) <
                       255 as libc::c_int {
                    (*r_ptr).r_blows[ap_cnt as usize] =
                        (*r_ptr).r_blows[ap_cnt as usize].wrapping_add(1)
                }
            }
        }
        ap_cnt += 1
    }
    /* Blink away */
    if blinked != 0 {
        msg_print(b"You flee laughing!\x00" as *const u8 as
                      *const libc::c_char);
        teleport_player(20 as libc::c_int * 2 as libc::c_int +
                            5 as libc::c_int);
    };
}
/*
 * Fetch an attack description from dam_*.txt files.
 */
unsafe extern "C" fn flavored_attack(mut percent: libc::c_int,
                                     mut output: *mut libc::c_char) {
    let mut insanity: libc::c_int =
        ((*p_ptr).msane as libc::c_int - (*p_ptr).csane as libc::c_int) *
            100 as libc::c_int / (*p_ptr).msane as libc::c_int;
    let mut insane: bool_ =
        (Rand_div(100 as libc::c_int) < insanity) as libc::c_int as bool_;
    if percent < 5 as libc::c_int {
        if insane == 0 {
            strcpy(output,
                   b"You scratch %s.\x00" as *const u8 as
                       *const libc::c_char);
        } else {
            get_rnd_line(b"dam_none.txt\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         output);
        }
    } else if percent < 30 as libc::c_int {
        if insane == 0 {
            strcpy(output,
                   b"You hit %s.\x00" as *const u8 as *const libc::c_char);
        } else {
            get_rnd_line(b"dam_med.txt\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         output);
        }
    } else if percent < 60 as libc::c_int {
        if insane == 0 {
            strcpy(output,
                   b"You wound %s.\x00" as *const u8 as *const libc::c_char);
        } else {
            get_rnd_line(b"dam_lots.txt\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         output);
        }
    } else if percent < 95 as libc::c_int {
        if insane == 0 {
            strcpy(output,
                   b"You cripple %s.\x00" as *const u8 as
                       *const libc::c_char);
        } else {
            get_rnd_line(b"dam_huge.txt\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         output);
        }
    } else if insane == 0 {
        strcpy(output,
               b"You demolish %s.\x00" as *const u8 as *const libc::c_char);
    } else {
        get_rnd_line(b"dam_xxx.txt\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char, output);
    };
}
/*
 * Apply the special effects of an attack
 */
#[no_mangle]
pub unsafe extern "C" fn attack_special(mut m_ptr: *mut monster_type,
                                        mut special: s32b,
                                        mut dam: libc::c_int) {
    let mut m_name: [libc::c_char; 80] = [0; 80];
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    /* Extract monster name (or "it") */
    monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
    /* Special - Cut monster */
    if special as libc::c_long & 0x2 as libc::c_long != 0 {
        /* Cut the monster */
        if (*r_ptr).flags8 & 0x800 as libc::c_int as libc::c_uint != 0 {
            if (*m_ptr).ml != 0 {
                let ref mut fresh1 =
                    (*r_info.offset((*m_ptr).r_idx as isize)).r_flags8;
                *fresh1 |= 0x800 as libc::c_int as libc::c_uint
            }
        } else if Rand_div(100 as libc::c_int) >=
                      (*r_ptr).level as libc::c_int {
            /* Already partially poisoned */
            if (*m_ptr).bleeding != 0 {
                msg_format(b"%^s is bleeding more strongly.\x00" as *const u8
                               as *const libc::c_char, m_name.as_mut_ptr());
            } else {
                /* Was not poisoned */
                msg_format(b"%^s is bleeding.\x00" as *const u8 as
                               *const libc::c_char, m_name.as_mut_ptr());
            }
            (*m_ptr).bleeding =
                ((*m_ptr).bleeding as libc::c_int + dam * 2 as libc::c_int) as
                    s16b
        }
    }
    /* Special - Poison monster */
    if special as libc::c_long & 0x1 as libc::c_long != 0 {
        /* Poison the monster */
        if (*r_ptr).flags3 & 0x100000 as libc::c_int as libc::c_uint != 0 {
            if (*m_ptr).ml != 0 {
                (*r_ptr).r_flags3 |= 0x100000 as libc::c_int as libc::c_uint
            }
        } else if (*r_ptr).flags9 & 0x100 as libc::c_int as libc::c_uint != 0
         {
            if (*m_ptr).ml != 0 {
                (*r_ptr).r_flags9 |= 0x100 as libc::c_int as libc::c_uint
            }
            /* Notice susceptibility */
            /* Already partially poisoned */
            if (*m_ptr).poisoned != 0 {
                msg_format(b"%^s is more poisoned.\x00" as *const u8 as
                               *const libc::c_char, m_name.as_mut_ptr());
            } else {
                /* Was not poisoned */
                msg_format(b"%^s is poisoned.\x00" as *const u8 as
                               *const libc::c_char, m_name.as_mut_ptr());
            }
            (*m_ptr).poisoned =
                ((*m_ptr).poisoned as libc::c_int + dam * 2 as libc::c_int) as
                    s16b
        } else if Rand_div(100 as libc::c_int) >=
                      (*r_ptr).level as libc::c_int {
            /* Already partially poisoned */
            if (*m_ptr).poisoned != 0 {
                msg_format(b"%^s is more poisoned.\x00" as *const u8 as
                               *const libc::c_char, m_name.as_mut_ptr());
            } else {
                /* Was not poisoned */
                msg_format(b"%^s is poisoned.\x00" as *const u8 as
                               *const libc::c_char, m_name.as_mut_ptr());
            }
            (*m_ptr).poisoned =
                ((*m_ptr).poisoned as libc::c_int + dam) as s16b
        }
    };
}
/*
 * Bare handed attacks
 */
unsafe extern "C" fn py_attack_hand(mut k: *mut libc::c_int,
                                    mut m_ptr: *mut monster_type,
                                    mut special: *mut s32b) {
    let mut special_effect: s16b = 0 as libc::c_int as s16b;
    let mut stun_effect: s16b = 0 as libc::c_int as s16b;
    let mut times: s16b = 0 as libc::c_int as s16b;
    let mut ma_ptr: *mut martial_arts = 0 as *mut martial_arts;
    let mut old_ptr: *mut martial_arts = 0 as *mut martial_arts;
    let mut blow_table: *mut martial_arts = ma_blows.as_mut_ptr();
    let mut resist_stun: libc::c_int = 0 as libc::c_int;
    let mut max: libc::c_int = 17 as libc::c_int;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    let mut m_name: [libc::c_char; 80] = [0; 80];
    let mut desc: bool_ = 0 as libc::c_int as bool_;
    let mut done_crit: bool_ = 0;
    let mut plev: libc::c_int = (*p_ptr).lev as libc::c_int;
    if (*p_ptr).body_monster == 0 &&
           (*p_ptr).mimic_form as libc::c_int ==
               resolve_mimic_name(b"Bear\x00" as *const u8 as
                                      *const libc::c_char) &&
           (*p_ptr).melee_style as libc::c_int == 47 as libc::c_int {
        blow_table = bear_blows.as_mut_ptr();
        max = 8 as libc::c_int;
        plev = get_skill(47 as libc::c_int) as libc::c_int
    }
    if (*p_ptr).melee_style as libc::c_int == 42 as libc::c_int {
        blow_table = ma_blows.as_mut_ptr();
        max = 17 as libc::c_int;
        plev = get_skill(42 as libc::c_int) as libc::c_int
    }
    ma_ptr =
        &mut *blow_table.offset(0 as libc::c_int as isize) as
            *mut martial_arts;
    old_ptr =
        &mut *blow_table.offset(0 as libc::c_int as isize) as
            *mut martial_arts;
    /* Extract monster name (or "it") */
    monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
    if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
        resist_stun += 88 as libc::c_int
    }
    if (*r_ptr).flags3 & 0x40000000 as libc::c_int as libc::c_uint != 0 {
        resist_stun += 44 as libc::c_int
    }
    if (*r_ptr).flags3 & 0x80000000 as libc::c_uint != 0 {
        resist_stun += 44 as libc::c_int
    }
    if (*r_ptr).flags3 & 0x20 as libc::c_int as libc::c_uint != 0 ||
           (*r_ptr).flags3 & 0x800 as libc::c_int as libc::c_uint != 0 {
        resist_stun += 88 as libc::c_int
    }
    if plev != 0 {
        times = 0 as libc::c_int as s16b;
        while (times as libc::c_int) <
                  (if plev < 7 as libc::c_int {
                       1 as libc::c_int
                   } else { (plev) / 7 as libc::c_int }) {
            loop  {
                ma_ptr =
                    &mut *blow_table.offset(((Rand_div as
                                                  unsafe extern "C" fn(_:
                                                                           s32b)
                                                      -> s32b)(max) +
                                                 1 as libc::c_int -
                                                 1 as libc::c_int) as isize)
                        as *mut martial_arts;
                if !((*ma_ptr).min_level > plev ||
                         (Rand_div(plev) + 1 as libc::c_int) <
                             (*ma_ptr).chance) {
                    break ;
                }
            }
            /* keep the highest level attack available we found */
            if (*ma_ptr).min_level > (*old_ptr).min_level &&
                   !((*p_ptr).stun as libc::c_int != 0 ||
                         (*p_ptr).confused as libc::c_int != 0) {
                old_ptr = ma_ptr;
                if wizard as libc::c_int != 0 &&
                       cheat_xtra as libc::c_int != 0 {
                    msg_print(b"Attack re-selected.\x00" as *const u8 as
                                  *const libc::c_char);
                }
            } else { ma_ptr = old_ptr }
            times += 1
        }
    }
    *k = damroll((*ma_ptr).dd as s16b, (*ma_ptr).ds as s16b);
    if (*ma_ptr).effect as libc::c_int & 0x1 as libc::c_int != 0 {
        if (*r_ptr).flags1 & 0x4 as libc::c_int as libc::c_uint != 0 {
            if desc == 0 {
                msg_format(b"You hit %s in the groin with your knee!\x00" as
                               *const u8 as *const libc::c_char,
                           m_name.as_mut_ptr());
            }
            sound(59 as libc::c_int);
            special_effect = 0x1 as libc::c_int as s16b
        } else if desc == 0 {
            msg_format((*ma_ptr).desc, m_name.as_mut_ptr());
        }
        desc = 1 as libc::c_int as bool_
    }
    if (*ma_ptr).effect as libc::c_int & 0x10 as libc::c_int != 0 {
        special_effect = 0x2 as libc::c_int as s16b;
        if desc == 0 { msg_format((*ma_ptr).desc, m_name.as_mut_ptr()); }
        desc = 1 as libc::c_int as bool_
    }
    if (*ma_ptr).effect as libc::c_int & 0x2 as libc::c_int != 0 {
        if !((*r_ptr).flags1 & 0x20000 as libc::c_int as libc::c_uint != 0 ||
                 !strchr(b"UjmeEv$,DdsbBFIJQSXclnw!=?\x00" as *const u8 as
                             *const libc::c_char,
                         (*r_ptr).d_char as libc::c_int).is_null()) {
            if desc == 0 {
                msg_format(b"You kick %s in the ankle.\x00" as *const u8 as
                               *const libc::c_char, m_name.as_mut_ptr());
            }
            special_effect = 0x2 as libc::c_int as s16b
        } else if desc == 0 {
            msg_format((*ma_ptr).desc, m_name.as_mut_ptr());
        }
        desc = 1 as libc::c_int as bool_
    }
    if (*ma_ptr).effect as libc::c_int & 0x8 as libc::c_int != 0 {
        if (*ma_ptr).power != 0 {
            stun_effect =
                ((*ma_ptr).power as libc::c_int / 2 as libc::c_int +
                     (Rand_div((*ma_ptr).power as libc::c_int /
                                   2 as libc::c_int) + 1 as libc::c_int)) as
                    s16b
        }
        if desc == 0 { msg_format((*ma_ptr).desc, m_name.as_mut_ptr()); }
        desc = 1 as libc::c_int as bool_
    }
    if (*ma_ptr).effect as libc::c_int & 0x4 as libc::c_int != 0 {
        if Rand_div(100 as libc::c_int) < (*ma_ptr).power as libc::c_int {
            *special =
                (*special as libc::c_long | 0x2 as libc::c_long) as s32b
        }
        if desc == 0 { msg_format((*ma_ptr).desc, m_name.as_mut_ptr()); }
        desc = 1 as libc::c_int as bool_
    }
    *k =
        critical_norm(plev * (Rand_div(10 as libc::c_int) + 1 as libc::c_int),
                      (*ma_ptr).min_level, *k, -(1 as libc::c_int),
                      &mut done_crit) as libc::c_int;
    if special_effect as libc::c_int & 0x1 as libc::c_int != 0 &&
           (*k + (*p_ptr).to_d as libc::c_int) < (*m_ptr).hp {
        msg_format(b"%^s moans in agony!\x00" as *const u8 as
                       *const libc::c_char, m_name.as_mut_ptr());
        stun_effect =
            (7 as libc::c_int +
                 (Rand_div(13 as libc::c_int) + 1 as libc::c_int)) as s16b;
        resist_stun /= 3 as libc::c_int
    }
    if (special_effect as libc::c_int & 0x10 as libc::c_int != 0 ||
            special_effect as libc::c_int & 0x2 as libc::c_int != 0) &&
           (*k + (*p_ptr).to_d as libc::c_int) < (*m_ptr).hp {
        if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint == 0 &&
               Rand_div(plev) + 1 as libc::c_int >
                   (*m_ptr).level as libc::c_int &&
               (*m_ptr).mspeed as libc::c_int > 60 as libc::c_int {
            msg_format(b"%^s starts limping slower.\x00" as *const u8 as
                           *const libc::c_char, m_name.as_mut_ptr());
            (*m_ptr).mspeed =
                ((*m_ptr).mspeed as libc::c_int - 10 as libc::c_int) as
                    byte_hack
        }
    }
    if stun_effect as libc::c_int != 0 &&
           (*k + (*p_ptr).to_d as libc::c_int) < (*m_ptr).hp {
        if plev >
               Rand_div((*m_ptr).level as libc::c_int + resist_stun +
                            10 as libc::c_int) + 1 as libc::c_int {
            if (*m_ptr).stunned != 0 {
                msg_format(b"%^s is still stunned.\x00" as *const u8 as
                               *const libc::c_char, m_name.as_mut_ptr());
            } else {
                msg_format(b"%^s is stunned.\x00" as *const u8 as
                               *const libc::c_char, m_name.as_mut_ptr());
            }
            (*m_ptr).stunned =
                ((*m_ptr).stunned as libc::c_int + stun_effect as libc::c_int)
                    as byte_hack
        }
    };
}
/*
 * Apply nazgul effects
 */
#[no_mangle]
pub unsafe extern "C" fn do_nazgul(mut k: *mut libc::c_int,
                                   mut num: *mut libc::c_int,
                                   mut num_blow: libc::c_int,
                                   mut weap: libc::c_int,
                                   mut r_ptr: *mut monster_race,
                                   mut o_ptr: *mut object_type) {
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut mundane: bool_ = 0;
    let mut allow_shatter: bool_ = 1 as libc::c_int as bool_;
    /* Extract mundane-ness of the current weapon */
    object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                 &mut esp);
    /* It should be Slay Evil, Slay Undead, or *Slay Undead* */
    mundane =
        (f1 as libc::c_long & 0x20000 as libc::c_long == 0 &&
             f1 as libc::c_long & 0x40000 as libc::c_long == 0 &&
             f5 as libc::c_long & 0x10 as libc::c_long == 0) as libc::c_int as
            bool_;
    /* Some blades can resist shattering */
    if f5 as libc::c_long & 0x1000 as libc::c_long != 0 {
        allow_shatter = 0 as libc::c_int as bool_
    }
    /* Mega Hack -- Hitting Nazgul is REALY dangerous (ideas from Akhronath) */
    if (*r_ptr).flags7 & 0x80 as libc::c_int as libc::c_uint != 0 {
        if (*o_ptr).name2 == 0 &&
               !((*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
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
               allow_shatter as libc::c_int != 0 {
            msg_print(b"Your weapon *DISINTEGRATES*!\x00" as *const u8 as
                          *const libc::c_char);
            *k = 0 as libc::c_int;
            inc_stack_size_ex(24 as libc::c_int + weap, -(1 as libc::c_int),
                              OPTIMIZE, NO_DESCRIBE);
            /* To stop attacking */
            *num = num_blow
        } else if (*o_ptr).name2 != 0 {
            if mundane != 0 {
                msg_print(b"The Ringwraith is IMPERVIOUS to the mundane weapon.\x00"
                              as *const u8 as *const libc::c_char);
                *k = 0 as libc::c_int
            }
            /* 25% chance of getting destroyed */
            if Rand_div(100 as libc::c_int) < 25 as libc::c_int &&
                   allow_shatter as libc::c_int != 0 {
                msg_print(b"Your weapon is destroyed!\x00" as *const u8 as
                              *const libc::c_char);
                inc_stack_size_ex(24 as libc::c_int + weap,
                                  -(1 as libc::c_int), OPTIMIZE, NO_DESCRIBE);
                /* To stop attacking */
                *num = num_blow
            }
        } else if (*o_ptr).tval as libc::c_int == 102 as libc::c_int ||
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
            if mundane != 0 {
                msg_print(b"The Ringwraith is IMPERVIOUS to the mundane weapon.\x00"
                              as *const u8 as *const libc::c_char);
                *k = 0 as libc::c_int
            }
            apply_disenchant(24 as libc::c_int + weap);
            /* 1/1000 chance of getting destroyed */
            if Rand_div(1000 as libc::c_int) == 0 &&
                   allow_shatter as libc::c_int != 0 {
                msg_print(b"Your weapon is destroyed!\x00" as *const u8 as
                              *const libc::c_char);
                inc_stack_size_ex(24 as libc::c_int + weap,
                                  -(1 as libc::c_int), OPTIMIZE, NO_DESCRIBE);
                /* To stop attacking */
                *num = num_blow
            }
        }
        /* If any damage is done, then 25% chance of getting the Black Breath */
        if *k != 0 {
            if Rand_div(100 as libc::c_int) < 25 as libc::c_int {
                msg_print(b"Your foe calls upon your soul!\x00" as *const u8
                              as *const libc::c_char);
                msg_print(b"You feel the Black Breath slowly draining you of life...\x00"
                              as *const u8 as *const libc::c_char);
                (*p_ptr).black_breath = 1 as libc::c_int as bool_
            }
        }
    };
}
/*
 * Player attacks a (poor, defenseless) creature        -RAK-
 *
 * If no "weapon" is available, then "punch" the monster one time.
 */
#[no_mangle]
pub unsafe extern "C" fn py_attack(mut y: libc::c_int, mut x: libc::c_int,
                                   mut max_blow: libc::c_int) {
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut bonus: libc::c_int = 0;
    let mut chance: libc::c_int = 0;
    let mut special: s32b = 0 as libc::c_int;
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    let mut m_ptr: *mut monster_type =
        &mut *m_list.offset((*c_ptr).m_idx as isize) as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    let mut m_name: [libc::c_char; 80] = [0; 80];
    let mut fear: bool_ = 0 as libc::c_int as bool_;
    let mut mdeath: bool_ = 0 as libc::c_int as bool_;
    let mut backstab: bool_ = 0 as libc::c_int as bool_;
    let mut vorpal_cut: bool_ = 0 as libc::c_int as bool_;
    let mut chaos_effect: libc::c_int = 0 as libc::c_int;
    let mut stab_fleeing: bool_ = 0 as libc::c_int as bool_;
    let mut do_quake: bool_ = 0 as libc::c_int as bool_;
    let mut done_crit: bool_ = 0 as libc::c_int as bool_;
    let mut drain_msg: bool_ = 1 as libc::c_int as bool_;
    let mut drain_result: libc::c_int = 0 as libc::c_int;
    let mut drain_heal: libc::c_int = 0 as libc::c_int;
    let mut drain_left: libc::c_int = 100 as libc::c_int;
    /* A massive hack -- life-draining weapons */
    let mut f1: u32b = 0;
    let mut f2: u32b = 0;
    let mut f3: u32b = 0;
    let mut f4: u32b = 0;
    let mut f5: u32b = 0;
    let mut esp: u32b = 0;
    let mut weap: libc::c_int = 0;
    /* Disturb the player */
    disturb(0 as libc::c_int, 0 as libc::c_int);
    if (*r_info.offset((*p_ptr).body_monster as isize)).flags1 &
           0x10000 as libc::c_int as libc::c_uint != 0 {
        msg_print(b"You cannot attack in this form!\x00" as *const u8 as
                      *const libc::c_char);
        return
    }
    if get_skill(22 as libc::c_int) != 0 {
        if (*m_ptr).csleep as libc::c_int != 0 &&
               (*m_ptr).ml as libc::c_int != 0 {
            /* Can't backstab creatures that we can't see, right? */
            backstab = 1 as libc::c_int as bool_
        } else if (*m_ptr).monfear as libc::c_int != 0 &&
                      (*m_ptr).ml as libc::c_int != 0 {
            stab_fleeing = 1 as libc::c_int as bool_
        }
    }
    /* Disturb the monster */
    (*m_ptr).csleep = 0 as libc::c_int as s16b;
    /* Extract monster name (or "it") */
    monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
    /* Dont even bother */
    if (*r_ptr).flags7 & 0x100000 as libc::c_int as libc::c_uint != 0 {
        msg_format(b"%^s is immune to melee attacks.\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    /* Auto-Recall if possible and visible */
    if (*m_ptr).ml != 0 {
        monster_race_track((*m_ptr).r_idx as libc::c_int,
                           (*m_ptr).ego as libc::c_int);
    }
    /* Track a new monster */
    if (*m_ptr).ml != 0 { health_track((*c_ptr).m_idx as libc::c_int); }
    /* Stop if friendly */
    if is_friend(m_ptr) >= 0 as libc::c_int &&
           !((*p_ptr).stun as libc::c_int != 0 ||
                 (*p_ptr).confused as libc::c_int != 0 ||
                 (*p_ptr).image as libc::c_int != 0 || (*m_ptr).ml == 0) {
        if (*p_ptr).inventory[24 as libc::c_int as usize].art_name == 0 {
            msg_format(b"You stop to avoid hitting %s.\x00" as *const u8 as
                           *const libc::c_char, m_name.as_mut_ptr());
            return
        }
        if streq(quark_str((*p_ptr).inventory[24 as libc::c_int as
                                                  usize].art_name as s16b),
                 b"\'Stormbringer\'\x00" as *const u8 as *const libc::c_char)
               == 0 {
            msg_format(b"You stop to avoid hitting %s.\x00" as *const u8 as
                           *const libc::c_char, m_name.as_mut_ptr());
            return
        }
        msg_format(b"Your black blade greedily attacks %s!\x00" as *const u8
                       as *const libc::c_char, m_name.as_mut_ptr());
    }
    /* Break goi/manashield */
    if (*p_ptr).invuln != 0 { set_invuln(0 as libc::c_int); }
    if (*p_ptr).disrupt_shield != 0 { set_disrupt_shield(0 as libc::c_int); }
    /* Handle player fear */
    if (*p_ptr).afraid != 0 {
        /* Message */
        if (*m_ptr).ml != 0 {
            msg_format(b"You are too afraid to attack %s!\x00" as *const u8 as
                           *const libc::c_char, m_name.as_mut_ptr());
        } else {
            msg_format(b"There is something scary in your way!\x00" as
                           *const u8 as *const libc::c_char);
        }
        /* Done */
        return
    }
    /* Monsters can use barehanded combat, but not weapon combat */
    if (*p_ptr).body_monster as libc::c_int != 0 &&
           (*r_info.offset((*p_ptr).body_monster as
                               isize)).body_parts[0 as libc::c_int as usize]
               == 0 &&
           !((*p_ptr).melee_style as libc::c_int == 42 as libc::c_int) {
        incarnate_monster_attack((*c_ptr).m_idx, &mut fear, &mut mdeath, y,
                                 x);
    } else {
        /* Otherwise use your weapon(s) */
        let mut weapons: libc::c_int = 0;
        if (*p_ptr).melee_style as libc::c_int == 17 as libc::c_int {
            weapons =
                (*r_info.offset((*p_ptr).body_monster as
                                    isize)).body_parts[0 as libc::c_int as
                                                           usize] as
                    libc::c_int
        } else {
            /* SKILL_HAND */
            weapons = 1 as libc::c_int
        }
        /* Attack with ALL the weapons !!!!! -- ooh that's gonna hurt YOU */
        weap = 0 as libc::c_int;
        while weap < weapons {
            /* Monster is already dead ? oh :( */
            if mdeath != 0 { break ; }
            /* Reset the blows counter */
            num = 0 as libc::c_int;
            /* Access the weapon */
            o_ptr =
                &mut *(*p_ptr).inventory.as_mut_ptr().offset((24 as
                                                                  libc::c_int
                                                                  + weap) as
                                                                 isize) as
                    *mut object_type;
            /* Calculate the "attack quality" */
            bonus =
                (*p_ptr).to_h as libc::c_int +
                    (*p_ptr).to_h_melee as libc::c_int +
                    (*o_ptr).to_h as libc::c_int;
            chance =
                (*p_ptr).skill_thn as libc::c_int + bonus * 3 as libc::c_int;
            object_flags(o_ptr, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5,
                         &mut esp);
            if f4 as libc::c_long & 0x1 as libc::c_long == 0 {
                let mut num_blow: libc::c_int =
                    (*p_ptr).num_blow as libc::c_int;
                /* Restrict to max_blow(if max_blow >= 0) */
                if max_blow >= 0 as libc::c_int && num_blow > max_blow {
                    num_blow = max_blow
                }
                loop 
                     /* Attack once for each legal blow */
                     {
                    let fresh2 = num;
                    num = num + 1;
                    if !(fresh2 < num_blow) { break ; }
                    /* Test for hit */
                    if test_hit_norm(chance, (*m_ptr).ac as libc::c_int,
                                     (*m_ptr).ml as libc::c_int) != 0 {
                        /* Sound */
                        sound(1 as libc::c_int);
                        /* Hack -- bare hands do one damage */
                        k = 1 as libc::c_int;
                        /* Select a chaotic effect (50% chance) */
                        if f1 as libc::c_long & 0x4000 as libc::c_long != 0 &&
                               Rand_div(2 as libc::c_int) == 0 as libc::c_int
                           {
                            if (Rand_div(5 as libc::c_int) + 1 as libc::c_int)
                                   < 3 as libc::c_int {
                                /* Vampiric (20%) */
                                chaos_effect = 1 as libc::c_int
                            } else if Rand_div(250 as libc::c_int) ==
                                          0 as libc::c_int {
                                /* Quake (0.12%) */
                                chaos_effect = 2 as libc::c_int
                            } else if Rand_div(10 as libc::c_int) != 0 {
                                /* Confusion (26.892%) */
                                chaos_effect = 3 as libc::c_int
                            } else if Rand_div(2 as libc::c_int) ==
                                          0 as libc::c_int {
                                /* Teleport away (1.494%) */
                                chaos_effect = 4 as libc::c_int
                            } else {
                                /* Polymorph (1.494%) */
                                chaos_effect = 5 as libc::c_int
                            }
                        }
                        /* Vampiric drain */
                        if f1 as libc::c_long & 0x8000 as libc::c_long != 0 ||
                               chaos_effect == 1 as libc::c_int {
                            if !((*r_ptr).flags3 &
                                     0x20 as libc::c_int as libc::c_uint != 0
                                     ||
                                     (*r_ptr).flags3 &
                                         0x800 as libc::c_int as libc::c_uint
                                         != 0) {
                                drain_result = (*m_ptr).hp
                            } else { drain_result = 0 as libc::c_int }
                        }
                        if f1 as libc::c_long & 0x2000000 as libc::c_long != 0
                               &&
                               Rand_div(6 as libc::c_int) + 1 as libc::c_int
                                   == 1 as libc::c_int {
                            vorpal_cut = 1 as libc::c_int as bool_
                        } else { vorpal_cut = 0 as libc::c_int as bool_ }
                        /* Should we attack with hands or not ? */
                        if (*p_ptr).melee_style as libc::c_int !=
                               17 as libc::c_int {
                            py_attack_hand(&mut k, m_ptr, &mut special);
                        } else if (*o_ptr).k_idx != 0 {
                            k =
                                damroll((*o_ptr).dd as s16b,
                                        (*o_ptr).ds as s16b);
                            k =
                                tot_dam_aux(o_ptr, k, m_ptr, &mut special) as
                                    libc::c_int;
                            if backstab != 0 {
                                k +=
                                    k *
                                        get_skill_scale(22 as libc::c_int,
                                                        100 as libc::c_int as
                                                            u32b) as
                                            libc::c_int / 100 as libc::c_int
                            } else if stab_fleeing != 0 {
                                k +=
                                    k *
                                        get_skill_scale(22 as libc::c_int,
                                                        70 as libc::c_int as
                                                            u32b) as
                                            libc::c_int / 100 as libc::c_int
                            }
                            if (*p_ptr).impact as libc::c_int != 0 &&
                                   (k > 50 as libc::c_int ||
                                        Rand_div(7 as libc::c_int) +
                                            1 as libc::c_int ==
                                            1 as libc::c_int) ||
                                   chaos_effect == 2 as libc::c_int {
                                do_quake = 1 as libc::c_int as bool_
                            }
                            k =
                                critical_norm((*o_ptr).weight,
                                              (*o_ptr).to_h as libc::c_int, k,
                                              (*o_ptr).tval as libc::c_int,
                                              &mut done_crit) as libc::c_int;
                            /* Handle normal weapon */
                            /* Stunning blow */
                            if Rand_div(100 as libc::c_int) <
                                   get_skill(57 as libc::c_int) as libc::c_int
                                   &&
                                   (*o_ptr).tval as libc::c_int ==
                                       21 as libc::c_int &&
                                   (*o_ptr).weight > 50 as libc::c_int &&
                                   done_crit as libc::c_int != 0 {
                                if (*r_ptr).flags4 &
                                       0x20000 as libc::c_int as libc::c_uint
                                       == 0 &&
                                       (*r_ptr).flags4 &
                                           0x4000000 as libc::c_int as
                                               libc::c_uint == 0 && k != 0 {
                                    let mut tmp: libc::c_int = 0;
                                    /* Get stunned */
                                    if (*m_ptr).stunned != 0 {
                                        msg_format(b"%^s is more dazed.\x00"
                                                       as *const u8 as
                                                       *const libc::c_char,
                                                   m_name.as_mut_ptr());
                                        tmp =
                                            (*m_ptr).stunned as libc::c_int +
                                                get_skill_scale(57 as
                                                                    libc::c_int,
                                                                30 as
                                                                    libc::c_int
                                                                    as u32b)
                                                    as libc::c_int +
                                                10 as libc::c_int
                                    } else {
                                        msg_format(b"%^s is dazed.\x00" as
                                                       *const u8 as
                                                       *const libc::c_char,
                                                   m_name.as_mut_ptr());
                                        tmp =
                                            get_skill_scale(57 as libc::c_int,
                                                            60 as libc::c_int
                                                                as u32b) as
                                                libc::c_int +
                                                20 as libc::c_int
                                    }
                                    /* Apply stun */
                                    (*m_ptr).stunned =
                                        if tmp < 200 as libc::c_int {
                                            tmp
                                        } else { 200 as libc::c_int } as
                                            byte_hack
                                }
                            }
                            if vorpal_cut != 0 {
                                let mut step_k: libc::c_int = k;
                                msg_format(b"Your weapon cuts deep into %s!\x00"
                                               as *const u8 as
                                               *const libc::c_char,
                                           m_name.as_mut_ptr());
                                loop  {
                                    k += step_k;
                                    if !(Rand_div(4 as libc::c_int) +
                                             1 as libc::c_int ==
                                             1 as libc::c_int) {
                                        break ;
                                    }
                                }
                            }
                            if (*p_ptr).pgod as libc::c_int ==
                                   3 as libc::c_int &&
                                   (*p_ptr).praying as libc::c_int != 0 {
                                if Rand_div(100 as libc::c_int) <
                                       wisdom_scale(130 as libc::c_int) -
                                           (*m_ptr).level as libc::c_int &&
                                       (*p_ptr).grace > 1000 as libc::c_int {
                                    msg_print(b"You feel the hand of Tulkas helping your blow.\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                                    k +=
                                        ((*o_ptr).to_d as libc::c_int +
                                             (*p_ptr).to_d_melee as
                                                 libc::c_int) *
                                            2 as libc::c_int
                                } else {
                                    k +=
                                        (*o_ptr).to_d as libc::c_int +
                                            (*p_ptr).to_d_melee as libc::c_int
                                }
                            } else { k += (*o_ptr).to_d as libc::c_int }
                            /* Project some more nasty stuff? */
                            if (*p_ptr).tim_project != 0 {
                                project(0 as libc::c_int,
                                        (*p_ptr).tim_project_rad as
                                            libc::c_int, y, x,
                                        (*p_ptr).tim_project_dam as
                                            libc::c_int,
                                        (*p_ptr).tim_project_gf as
                                            libc::c_int,
                                        (*p_ptr).tim_project_flag as
                                            libc::c_int | 0x1 as libc::c_int);
                                if (*c_ptr).m_idx == 0 {
                                    mdeath = 1 as libc::c_int as bool_;
                                    break ;
                                }
                            }
                            do_nazgul(&mut k, &mut num, num_blow, weap, r_ptr,
                                      o_ptr);
                        }
                        /* Melkor can cast curse for you*/
                        if (*p_ptr).pgod as libc::c_int == 4 as libc::c_int &&
                               (*p_ptr).praying as libc::c_int != 0 {
                            let mut lv: libc::c_int =
                                exec_lua(b"return get_level(MELKOR_CURSE, 100)\x00"
                                             as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char);
                            if lv >= 10 as libc::c_int {
                                let mut chance_0: libc::c_int =
                                    wisdom_scale(30 as libc::c_int) * lv /
                                        (if ((*m_ptr).level as libc::c_int) <
                                                1 as libc::c_int {
                                             1 as libc::c_int
                                         } else {
                                             (*m_ptr).level as libc::c_int
                                         });
                                if chance_0 < 1 as libc::c_int {
                                    chance_0 = 1 as libc::c_int
                                }
                                if (*p_ptr).grace > 5000 as libc::c_int &&
                                       Rand_div(100 as libc::c_int) < chance_0
                                   {
                                    exec_lua(format(b"do_melkor_curse(%d)\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    (*c_ptr).m_idx as
                                                        libc::c_int));
                                }
                            }
                        }
                        /* May it clone the monster ? */
                        if f4 as libc::c_long & 0x200 as libc::c_long != 0 &&
                               Rand_div(100 as libc::c_int) <
                                   30 as libc::c_int {
                            msg_format(b"Oh no! Your weapon clones %^s!\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       m_name.as_mut_ptr());
                            multiply_monster((*c_ptr).m_idx as libc::c_int,
                                             0 as libc::c_int as bool_,
                                             1 as libc::c_int as bool_);
                        }
                        /* Apply the player damage bonuses */
                        k +=
                            (*p_ptr).to_d as libc::c_int +
                                (*p_ptr).to_d_melee as libc::c_int;
                        /* No negative damage */
                        if k < 0 as libc::c_int { k = 0 as libc::c_int }
                        /* Message */
                        if !(backstab as libc::c_int != 0 ||
                                 stab_fleeing as libc::c_int != 0) {
                            /* These monsters never have flavoured combat msgs */
                            if !strchr(b"vwjmelX,.*\x00" as *const u8 as
                                           *const libc::c_char,
                                       (*r_ptr).d_char as
                                           libc::c_int).is_null() {
                                msg_format(b"You hit %s.\x00" as *const u8 as
                                               *const libc::c_char,
                                           m_name.as_mut_ptr());
                            } else {
                                /* Print flavoured messages if requested */
                                let mut buff: [libc::c_char; 255] = [0; 255];
                                flavored_attack(100 as libc::c_int * k /
                                                    (*m_ptr).maxhp,
                                                buff.as_mut_ptr());
                                msg_format(buff.as_mut_ptr() as cptr,
                                           m_name.as_mut_ptr());
                            }
                        } else if backstab != 0 {
                            let mut buf: [libc::c_char; 80] = [0; 80];
                            monster_race_desc(buf.as_mut_ptr(),
                                              (*m_ptr).r_idx as libc::c_int,
                                              (*m_ptr).ego as libc::c_int);
                            backstab = 0 as libc::c_int as bool_;
                            msg_format(b"You cruelly stab the helpless, sleeping %s!\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       buf.as_mut_ptr());
                        } else {
                            let mut buf_0: [libc::c_char; 80] = [0; 80];
                            monster_race_desc(buf_0.as_mut_ptr(),
                                              (*m_ptr).r_idx as libc::c_int,
                                              (*m_ptr).ego as libc::c_int);
                            msg_format(b"You backstab the fleeing %s!\x00" as
                                           *const u8 as *const libc::c_char,
                                       buf_0.as_mut_ptr());
                        }
                        /* Complex message */
                        if wizard != 0 {
                            msg_format(b"You do %d (out of %d) damage.\x00" as
                                           *const u8 as *const libc::c_char,
                                       k, (*m_ptr).hp);
                        }
                        if special != 0 { attack_special(m_ptr, special, k); }
                        /* Damage, check for fear and death */
                        if mon_take_hit((*c_ptr).m_idx as libc::c_int, k,
                                        &mut fear, 0 as cptr) != 0 {
                            /* Hack -- High-level warriors can spread their attacks out
							 * among weaker foes.
							 */
                            if has_ability(0 as libc::c_int) as libc::c_int !=
                                   0 && num < num_blow && energy_use != 0 {
                                energy_use = energy_use * num / num_blow
                            }
                            mdeath = 1 as libc::c_int as bool_;
                            break ;
                        } else {
                            match is_friend(m_ptr) {
                                1 => {
                                    msg_format(b"%^s gets angry!\x00" as
                                                   *const u8 as
                                                   *const libc::c_char,
                                               m_name.as_mut_ptr());
                                    change_side(m_ptr);
                                }
                                0 => {
                                    msg_format(b"%^s gets angry!\x00" as
                                                   *const u8 as
                                                   *const libc::c_char,
                                               m_name.as_mut_ptr());
                                    (*m_ptr).status =
                                        -(1 as libc::c_int) as s16b
                                }
                                _ => { }
                            }
                            touch_zap_player(m_ptr);
                            /* Are we draining it?  A little note: If the monster is
						   dead, the drain does not work... */
                            if drain_result != 0 {
                                drain_result -=
                                    (*m_ptr).hp; /* Calculate the difference */
                                if drain_result > 0 as libc::c_int {
                                    /* Did we really hurt it? */
                                    drain_heal =
                                        damroll(4 as libc::c_int as s16b,
                                                (drain_result /
                                                     6 as libc::c_int) as
                                                    s16b);
                                    if cheat_xtra != 0 {
                                        msg_format(b"Draining left: %d\x00" as
                                                       *const u8 as
                                                       *const libc::c_char,
                                                   drain_left);
                                    }
                                    if drain_left != 0 {
                                        if drain_heal < drain_left {
                                            drain_left -= drain_heal
                                        } else {
                                            drain_heal = drain_left;
                                            drain_left = 0 as libc::c_int
                                        }
                                        if drain_msg != 0 {
                                            msg_format(b"Your weapon drains life from %s!\x00"
                                                           as *const u8 as
                                                           *const libc::c_char,
                                                       m_name.as_mut_ptr());
                                            drain_msg =
                                                0 as libc::c_int as bool_
                                        }
                                        hp_player(drain_heal);
                                        /* We get to keep some of it! */
                                    }
                                }
                            }
                            /* Confusion attack */
                            if (*p_ptr).confusing as libc::c_int != 0 ||
                                   chaos_effect == 3 as libc::c_int {
                                /* Cancel glowing hands */
                                if (*p_ptr).confusing != 0 {
                                    (*p_ptr).confusing =
                                        0 as libc::c_int as byte_hack;
                                    msg_print(b"Your hands stop glowing.\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                                }
                                /* Confuse the monster */
                                if (*r_ptr).flags3 &
                                       0x40000000 as libc::c_int as
                                           libc::c_uint != 0 {
                                    if (*m_ptr).ml != 0 {
                                        (*r_ptr).r_flags3 |=
                                            0x40000000 as libc::c_int as
                                                libc::c_uint
                                    }
                                    msg_format(b"%^s is unaffected.\x00" as
                                                   *const u8 as
                                                   *const libc::c_char,
                                               m_name.as_mut_ptr());
                                } else if Rand_div(100 as libc::c_int) <
                                              (*m_ptr).level as libc::c_int {
                                    msg_format(b"%^s is unaffected.\x00" as
                                                   *const u8 as
                                                   *const libc::c_char,
                                               m_name.as_mut_ptr());
                                } else {
                                    msg_format(b"%^s appears confused.\x00" as
                                                   *const u8 as
                                                   *const libc::c_char,
                                               m_name.as_mut_ptr());
                                    (*m_ptr).confused =
                                        ((*m_ptr).confused as libc::c_int +
                                             (10 as libc::c_int +
                                                  Rand_div(get_skill(16 as
                                                                         libc::c_int)
                                                               as s32b) /
                                                      5 as libc::c_int)) as
                                            byte_hack
                                }
                            } else if chaos_effect == 4 as libc::c_int {
                                msg_format(b"%^s disappears!\x00" as *const u8
                                               as *const libc::c_char,
                                           m_name.as_mut_ptr());
                                teleport_away((*c_ptr).m_idx as libc::c_int,
                                              50 as libc::c_int);
                                num = num_blow + 1 as libc::c_int
                                /* Can't hit it anymore! */
                            } else if chaos_effect == 5 as libc::c_int &&
                                          ((*f_info.offset((*cave[y as
                                                                      usize].offset(x
                                                                                        as
                                                                                        isize)).feat
                                                               as
                                                               isize)).flags1
                                               as libc::c_long &
                                               0x10 as libc::c_long != 0 &&
                                               (*cave[y as
                                                          usize].offset(x as
                                                                            isize)).feat
                                                   as libc::c_int !=
                                                   0xaf as libc::c_int) &&
                                          Rand_div(90 as libc::c_int) +
                                              1 as libc::c_int >
                                              (*m_ptr).level as libc::c_int {
                                if !((*r_ptr).flags1 &
                                         0x1 as libc::c_int as libc::c_uint !=
                                         0 ||
                                         (*r_ptr).flags4 &
                                             0x40000 as libc::c_int as
                                                 libc::c_uint != 0 ||
                                         (*m_ptr).mflag & 0x2 as libc::c_int
                                             != 0) {
                                    /* Handle polymorph */
                                    if do_poly_monster(y, x) != 0 {
                                        /* Polymorph succeeded */
                                        msg_format(b"%^s changes!\x00" as
                                                       *const u8 as
                                                       *const libc::c_char,
                                                   m_name.as_mut_ptr());
                                        /* Hack -- Get new monster */
                                        m_ptr =
                                            &mut *m_list.offset((*c_ptr).m_idx
                                                                    as isize)
                                                as *mut monster_type;
                                        /* Oops, we need a different name... */
                                        monster_desc(m_name.as_mut_ptr(),
                                                     m_ptr, 0 as libc::c_int);
                                        /* Hack -- Get new race */
                                        r_ptr =
                                            if !(*m_ptr).sr_ptr.is_null() {
                                                (*m_ptr).sr_ptr
                                            } else {
                                                race_info_idx((*m_ptr).r_idx
                                                                  as
                                                                  libc::c_int,
                                                              (*m_ptr).ego as
                                                                  libc::c_int)
                                            };
                                        fear = 0 as libc::c_int as bool_
                                    } else {
                                        msg_format(b"%^s resists.\x00" as
                                                       *const u8 as
                                                       *const libc::c_char,
                                                   m_name.as_mut_ptr());
                                    }
                                } else {
                                    msg_format(b"%^s is unaffected.\x00" as
                                                   *const u8 as
                                                   *const libc::c_char,
                                               m_name.as_mut_ptr());
                                }
                            }
                        }
                    } else {
                        /* Player misses */
                        /* Sound */
                        sound(2 as libc::c_int); /* Clumsy! */
                        backstab = 0 as libc::c_int as bool_;
                        msg_format(b"You miss %s.\x00" as *const u8 as
                                       *const libc::c_char,
                                   m_name.as_mut_ptr());
                    }
                }
            } else {
                msg_print(b"You can\'t attack with that weapon.\x00" as
                              *const u8 as *const libc::c_char);
            }
            weap += 1
        }
    }
    /* Message */
    /* Carried monster can attack too */
    if mdeath == 0 && (*m_list.offset((*c_ptr).m_idx as isize)).hp != 0 {
        carried_monster_attack((*c_ptr).m_idx, &mut fear, &mut mdeath, y, x);
    }
    /* Hack -- delay fear messages */
    if fear as libc::c_int != 0 && (*m_ptr).ml as libc::c_int != 0 {
        /* Sound */
        sound(3 as libc::c_int);
        /* Message */
        msg_format(b"%^s flees in terror!\x00" as *const u8 as
                       *const libc::c_char, m_name.as_mut_ptr());
    }
    /* Mega-Hack -- apply earthquake brand */
    if do_quake != 0 {
        /* Prevent destruction of quest levels and town */
        if is_quest(dun_level as libc::c_int) == 0 &&
               dun_level as libc::c_int != 0 {
            earthquake((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int,
                       10 as libc::c_int);
        }
    };
}
unsafe extern "C" fn pattern_tile(mut y: libc::c_int, mut x: libc::c_int)
 -> bool_ {
    return ((*cave[y as usize].offset(x as isize)).feat as libc::c_int <=
                0x49 as libc::c_int &&
                (*cave[y as usize].offset(x as isize)).feat as libc::c_int >=
                    0x41 as libc::c_int) as libc::c_int as bool_;
}
unsafe extern "C" fn pattern_seq(mut c_y: libc::c_int, mut c_x: libc::c_int,
                                 mut n_y: libc::c_int, mut n_x: libc::c_int)
 -> bool_ {
    if pattern_tile(c_y, c_x) == 0 && pattern_tile(n_y, n_x) == 0 {
        return 1 as libc::c_int as bool_
    }
    if (*cave[n_y as usize].offset(n_x as isize)).feat as libc::c_int ==
           0x41 as libc::c_int {
        if pattern_tile(c_y, c_x) == 0 &&
               !((*p_ptr).confused as libc::c_int != 0 ||
                     (*p_ptr).stun as libc::c_int != 0 ||
                     (*p_ptr).image as libc::c_int != 0) {
            if get_check(b"If you start walking the Straight Road, you must walk the whole way. Ok? \x00"
                             as *const u8 as *const libc::c_char) != 0 {
                return 1 as libc::c_int as bool_
            } else { return 0 as libc::c_int as bool_ }
        } else { return 1 as libc::c_int as bool_ }
    } else if (*cave[n_y as usize].offset(n_x as isize)).feat as libc::c_int
                  == 0x47 as libc::c_int ||
                  (*cave[n_y as usize].offset(n_x as isize)).feat as
                      libc::c_int == 0x46 as libc::c_int ||
                  (*cave[n_y as usize].offset(n_x as isize)).feat as
                      libc::c_int == 0x49 as libc::c_int {
        if pattern_tile(c_y, c_x) != 0 {
            return 1 as libc::c_int as bool_
        } else {
            msg_print(b"You must start walking the Straight Road from the startpoint.\x00"
                          as *const u8 as *const libc::c_char);
            return 0 as libc::c_int as bool_
        }
    } else if (*cave[n_y as usize].offset(n_x as isize)).feat as libc::c_int
                  == 0x48 as libc::c_int ||
                  (*cave[c_y as usize].offset(c_x as isize)).feat as
                      libc::c_int == 0x48 as libc::c_int {
        return 1 as libc::c_int as bool_
    } else if (*cave[c_y as usize].offset(c_x as isize)).feat as libc::c_int
                  == 0x41 as libc::c_int {
        if pattern_tile(n_y, n_x) != 0 {
            return 1 as libc::c_int as bool_
        } else {
            msg_print(b"You must walk the Straight Road in correct order.\x00"
                          as *const u8 as *const libc::c_char);
            return 0 as libc::c_int as bool_
        }
    } else if (*cave[c_y as usize].offset(c_x as isize)).feat as libc::c_int
                  == 0x47 as libc::c_int ||
                  (*cave[c_y as usize].offset(c_x as isize)).feat as
                      libc::c_int == 0x46 as libc::c_int ||
                  (*cave[c_y as usize].offset(c_x as isize)).feat as
                      libc::c_int == 0x49 as libc::c_int {
        if pattern_tile(n_y, n_x) == 0 {
            msg_print(b"You may not step off from the Straight Road.\x00" as
                          *const u8 as *const libc::c_char);
            return 0 as libc::c_int as bool_
        } else { return 1 as libc::c_int as bool_ }
    } else if pattern_tile(c_y, c_x) == 0 {
        msg_print(b"You must start walking the Straight Road from the startpoint.\x00"
                      as *const u8 as *const libc::c_char);
        return 0 as libc::c_int as bool_
    } else {
        let mut ok_move: byte_hack = 0x41 as libc::c_int as byte_hack;
        match (*cave[c_y as usize].offset(c_x as isize)).feat as libc::c_int {
            66 => {
                ok_move = 0x43 as libc::c_int as byte_hack
                /* Goof-up */
            }
            67 => { ok_move = 0x44 as libc::c_int as byte_hack }
            68 => { ok_move = 0x45 as libc::c_int as byte_hack }
            69 => { ok_move = 0x42 as libc::c_int as byte_hack }
            _ => {
                if wizard != 0 {
                    msg_format(b"Funny Straight Road walking, %d.\x00" as
                                   *const u8 as *const libc::c_char,
                               *cave[c_y as usize].offset(c_x as isize));
                }
                return 1 as libc::c_int as bool_
            }
        }
        if (*cave[n_y as usize].offset(n_x as isize)).feat as libc::c_int ==
               ok_move as libc::c_int ||
               (*cave[n_y as usize].offset(n_x as isize)).feat as libc::c_int
                   ==
                   (*cave[c_y as usize].offset(c_x as isize)).feat as
                       libc::c_int {
            return 1 as libc::c_int as bool_
        } else {
            if pattern_tile(n_y, n_x) == 0 {
                msg_print(b"You may not step off from the Straight Road.\x00"
                              as *const u8 as *const libc::c_char);
            } else {
                msg_print(b"You must walk the Straight Road in correct order.\x00"
                              as *const u8 as *const libc::c_char);
            }
            return 0 as libc::c_int as bool_
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn player_can_enter(mut feature: byte_hack) -> bool_ {
    let mut pass_wall: bool_ = 0;
    let mut only_wall: bool_ = 0 as libc::c_int as bool_;
    /* Player can not walk through "walls" unless in Shadow Form */
    if (*p_ptr).wraith_form as libc::c_int != 0 ||
           ((*rp_ptr).flags1 | (*rmp_ptr).flags1 | (*cp_ptr).flags1 |
                (*spp_ptr).flags1) as libc::c_long & 0x20000 as libc::c_long
               != 0 {
        pass_wall = 1 as libc::c_int as bool_
    } else { pass_wall = 0 as libc::c_int as bool_ }
    /* Wall mimicry force the player to stay in walls */
    if (*p_ptr).mimic_extra & 0x80 as libc::c_int as libc::c_uint != 0 {
        only_wall = 1 as libc::c_int as bool_
    }
    /* Don't let the player kill himself with one keystroke */
    if (*p_ptr).wild_mode != 0 {
        if feature as libc::c_int == 0xbb as libc::c_int {
            let mut wt: libc::c_int = weight_limit() / 2 as libc::c_int;
            if calc_total_weight() >= wt && (*p_ptr).ffall == 0 {
                return 0 as libc::c_int as bool_
            }
        } else if feature as libc::c_int == 0x56 as libc::c_int ||
                      feature as libc::c_int == 0x55 as libc::c_int {
            if !((*p_ptr).resist_fire as libc::c_int != 0 ||
                     (*p_ptr).immune_fire as libc::c_int != 0 ||
                     (*p_ptr).oppose_fire as libc::c_int != 0 ||
                     (*p_ptr).ffall as libc::c_int != 0) {
                return 0 as libc::c_int as bool_
            }
        }
    }
    if feature as libc::c_int == 0x60 as libc::c_int {
        if (*p_ptr).fly as libc::c_int != 0 || pass_wall as libc::c_int != 0
               || has_ability(1 as libc::c_int) as libc::c_int != 0 ||
               (*p_ptr).mimic_form as libc::c_int ==
                   resolve_mimic_name(b"Ent\x00" as *const u8 as
                                          *const libc::c_char) ||
               (*p_ptr).grace >= 9000 as libc::c_int &&
                   (*p_ptr).praying as libc::c_int != 0 &&
                   (*p_ptr).pgod as libc::c_int == 5 as libc::c_int {
            return 1 as libc::c_int as bool_
        }
    }
    if (*p_ptr).climb as libc::c_int != 0 &&
           (*f_info.offset(feature as isize)).flags1 as libc::c_long &
               0x4000 as libc::c_long != 0 {
        return 1 as libc::c_int as bool_
    }
    if (*p_ptr).fly as libc::c_int != 0 &&
           ((*f_info.offset(feature as isize)).flags1 as libc::c_long &
                0x80 as libc::c_long != 0 ||
                (*f_info.offset(feature as isize)).flags1 as libc::c_long &
                    0x4 as libc::c_long != 0) {
        return 1 as libc::c_int as bool_
    } else {
        if only_wall as libc::c_int != 0 &&
               (*f_info.offset(feature as isize)).flags1 as libc::c_long &
                   0x10 as libc::c_long != 0 {
            return 0 as libc::c_int as bool_
        } else {
            if (*p_ptr).ffall as libc::c_int != 0 &&
                   (*f_info.offset(feature as isize)).flags1 as libc::c_long &
                       0x4 as libc::c_long != 0 {
                return 1 as libc::c_int as bool_
            } else {
                if (pass_wall as libc::c_int != 0 ||
                        only_wall as libc::c_int != 0) &&
                       (*f_info.offset(feature as isize)).flags1 as
                           libc::c_long & 0x8 as libc::c_long != 0 {
                    return 1 as libc::c_int as bool_
                } else {
                    if (*f_info.offset(feature as isize)).flags1 as
                           libc::c_long & 0x1 as libc::c_long != 0 {
                        return 0 as libc::c_int as bool_
                    } else {
                        if (*f_info.offset(feature as isize)).flags1 as
                               libc::c_long & 0x10000 as libc::c_long != 0 &&
                               ((*r_info.offset((*p_ptr).body_monster as
                                                    isize)).flags7 &
                                    0x40 as libc::c_int as libc::c_uint == 0
                                    &&
                                    (*p_ptr).mimic_form as libc::c_int !=
                                        resolve_mimic_name(b"Spider\x00" as
                                                               *const u8 as
                                                               *const libc::c_char))
                           {
                            return 0 as libc::c_int as bool_
                        }
                    }
                }
            }
        }
    }
    return 1 as libc::c_int as bool_;
}
/*
 * Move player in the given direction, with the given "pickup" flag.
 *
 * This routine should (probably) always induce energy expenditure.
 *
 * Note that moving will *always* take a turn, and will *always* hit
 * any monster which might be in the destination grid.  Previously,
 * moving into walls was "free" and did NOT hit invisible monsters.
 */
#[no_mangle]
pub unsafe extern "C" fn move_player_aux(mut dir: libc::c_int,
                                         mut do_pickup: libc::c_int,
                                         mut run: libc::c_int,
                                         mut disarm: bool_) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset((*p_ptr).py as
                                             isize)).offset((*p_ptr).px as
                                                                isize) as
            *mut cave_type;
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*p_ptr).body_monster as isize) as
            *mut monster_race;
    let mut mr_ptr: *mut monster_race = 0 as *mut monster_race;
    let mut m_name: [libc::c_char; 80] = [0; 80];
    let mut stormbringer: bool_ = 0 as libc::c_int as bool_;
    let mut old_dtrap: bool_ = 0;
    let mut new_dtrap: bool_ = 0;
    let mut oktomove: bool_ = 1 as libc::c_int as bool_;
    /* Hack - random movement */
    if (*p_ptr).disembodied != 0 {
        tmp = dir
    } else if (*r_ptr).flags1 & 0x40000 as libc::c_int as libc::c_uint != 0 &&
                  (*r_ptr).flags1 & 0x80000 as libc::c_int as libc::c_uint !=
                      0 {
        if (Rand_div(100 as libc::c_int) + 1 as libc::c_int) <
               75 as libc::c_int {
            tmp = Rand_div(9 as libc::c_int) + 1 as libc::c_int
        } else { tmp = dir }
    } else if (*r_ptr).flags1 & 0x80000 as libc::c_int as libc::c_uint != 0 {
        if (Rand_div(100 as libc::c_int) + 1 as libc::c_int) <
               50 as libc::c_int {
            tmp = Rand_div(9 as libc::c_int) + 1 as libc::c_int
        } else { tmp = dir }
    } else if (*r_ptr).flags1 & 0x40000 as libc::c_int as libc::c_uint != 0 {
        if (Rand_div(100 as libc::c_int) + 1 as libc::c_int) <
               25 as libc::c_int {
            tmp = Rand_div(9 as libc::c_int) + 1 as libc::c_int
        } else { tmp = dir }
    } else { tmp = dir }
    if (*c_ptr).feat as libc::c_int == 0x5a as libc::c_int &&
           ((*p_ptr).ffall == 0 && (*p_ptr).fly == 0) {
        if Rand_div(100 as libc::c_int) <
               70 as libc::c_int - (*p_ptr).lev as libc::c_int {
            tmp = Rand_div(9 as libc::c_int) + 1 as libc::c_int;
            msg_print(b"You slip on the icy floor.\x00" as *const u8 as
                          *const libc::c_char);
        } else { tmp = dir }
    }
    /* Find the result of moving */
    y = (*p_ptr).py as libc::c_int + ddy[tmp as usize] as libc::c_int;
    x = (*p_ptr).px as libc::c_int + ddx[tmp as usize] as libc::c_int;
    /* Examine the destination */
    c_ptr =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Change oldpx and oldpy to place the player well when going back to big mode */
    if (*p_ptr).wild_mode != 0 {
        if ddy[tmp as usize] as libc::c_int > 0 as libc::c_int {
            (*p_ptr).oldpy = 1 as libc::c_int as s16b
        }
        if (ddy[tmp as usize] as libc::c_int) < 0 as libc::c_int {
            (*p_ptr).oldpy = (66 as libc::c_int - 2 as libc::c_int) as s16b
        }
        if ddy[tmp as usize] as libc::c_int == 0 as libc::c_int {
            (*p_ptr).oldpy = (66 as libc::c_int / 2 as libc::c_int) as s16b
        }
        if ddx[tmp as usize] as libc::c_int > 0 as libc::c_int {
            (*p_ptr).oldpx = 1 as libc::c_int as s16b
        }
        if (ddx[tmp as usize] as libc::c_int) < 0 as libc::c_int {
            (*p_ptr).oldpx = (198 as libc::c_int - 2 as libc::c_int) as s16b
        }
        if ddx[tmp as usize] as libc::c_int == 0 as libc::c_int {
            (*p_ptr).oldpx = (198 as libc::c_int / 2 as libc::c_int) as s16b
        }
    }
    /* Exit the area */
    if dun_level == 0 && (*p_ptr).wild_mode == 0 &&
           is_quest(dun_level as libc::c_int) == 0 &&
           (x == 0 as libc::c_int ||
                x == cur_wid as libc::c_int - 1 as libc::c_int ||
                y == 0 as libc::c_int ||
                y == cur_hgt as libc::c_int - 1 as libc::c_int) {
        /* Can the player enter the grid? */
        if player_can_enter((*c_ptr).mimic) != 0 {
            /* Hack: move to new area */
            if y == 0 as libc::c_int && x == 0 as libc::c_int {
                (*p_ptr).wilderness_y -= 1;
                (*p_ptr).wilderness_x -= 1;
                (*p_ptr).oldpy =
                    (cur_hgt as libc::c_int - 2 as libc::c_int) as s16b;
                (*p_ptr).oldpx =
                    (cur_wid as libc::c_int - 2 as libc::c_int) as s16b;
                ambush_flag = 0 as libc::c_int as bool_
            } else if y == 0 as libc::c_int &&
                          x == 198 as libc::c_int - 1 as libc::c_int {
                (*p_ptr).wilderness_y -= 1;
                (*p_ptr).wilderness_x += 1;
                (*p_ptr).oldpy =
                    (cur_hgt as libc::c_int - 2 as libc::c_int) as s16b;
                (*p_ptr).oldpx = 1 as libc::c_int as s16b;
                ambush_flag = 0 as libc::c_int as bool_
            } else if y == 66 as libc::c_int - 1 as libc::c_int &&
                          x == 0 as libc::c_int {
                (*p_ptr).wilderness_y += 1;
                (*p_ptr).wilderness_x -= 1;
                (*p_ptr).oldpy = 1 as libc::c_int as s16b;
                (*p_ptr).oldpx =
                    (cur_wid as libc::c_int - 2 as libc::c_int) as s16b;
                ambush_flag = 0 as libc::c_int as bool_
            } else if y == 66 as libc::c_int - 1 as libc::c_int &&
                          x == 198 as libc::c_int - 1 as libc::c_int {
                (*p_ptr).wilderness_y += 1;
                (*p_ptr).wilderness_x += 1;
                (*p_ptr).oldpy = 1 as libc::c_int as s16b;
                (*p_ptr).oldpx = 1 as libc::c_int as s16b;
                ambush_flag = 0 as libc::c_int as bool_
            } else if y == 0 as libc::c_int {
                (*p_ptr).wilderness_y -= 1;
                (*p_ptr).oldpy =
                    (cur_hgt as libc::c_int - 2 as libc::c_int) as s16b;
                (*p_ptr).oldpx = x as s16b;
                ambush_flag = 0 as libc::c_int as bool_
            } else if y == cur_hgt as libc::c_int - 1 as libc::c_int {
                (*p_ptr).wilderness_y += 1;
                (*p_ptr).oldpy = 1 as libc::c_int as s16b;
                (*p_ptr).oldpx = x as s16b;
                ambush_flag = 0 as libc::c_int as bool_
            } else if x == 0 as libc::c_int {
                (*p_ptr).wilderness_x -= 1;
                (*p_ptr).oldpx =
                    (cur_wid as libc::c_int - 2 as libc::c_int) as s16b;
                (*p_ptr).oldpy = y as s16b;
                ambush_flag = 0 as libc::c_int as bool_
            } else if x == cur_wid as libc::c_int - 1 as libc::c_int {
                (*p_ptr).wilderness_x += 1;
                (*p_ptr).oldpx = 1 as libc::c_int as s16b;
                (*p_ptr).oldpy = y as s16b;
                ambush_flag = 0 as libc::c_int as bool_
            }
            (*p_ptr).leaving = 1 as libc::c_int as bool_;
            return
        }
    }
    /* Some hooks */
    if process_hooks(17 as libc::c_int,
                     b"(d,d)\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char, y, x) != 0 {
        return
    }
    /* Get the monster */
    m_ptr = &mut *m_list.offset((*c_ptr).m_idx as isize) as *mut monster_type;
    mr_ptr =
        if !(*m_ptr).sr_ptr.is_null() {
            (*m_ptr).sr_ptr
        } else {
            race_info_idx((*m_ptr).r_idx as libc::c_int,
                          (*m_ptr).ego as libc::c_int)
        };
    if (*p_ptr).inventory[24 as libc::c_int as usize].art_name != 0 {
        if streq(quark_str((*p_ptr).inventory[24 as libc::c_int as
                                                  usize].art_name as s16b),
                 b"\'Stormbringer\'\x00" as *const u8 as *const libc::c_char)
               != 0 {
            stormbringer = 1 as libc::c_int as bool_
        }
    }
    /* Hack -- attack monsters */
    if (*c_ptr).m_idx as libc::c_int != 0 &&
           ((*m_ptr).ml as libc::c_int != 0 ||
                player_can_enter((*c_ptr).feat) as libc::c_int != 0) {
        /* Attack -- only if we can see it OR it is not in a wall */
        if is_friend(m_ptr) > 0 as libc::c_int &&
               !((*p_ptr).confused as libc::c_int != 0 ||
                     (*p_ptr).image as libc::c_int != 0 || (*m_ptr).ml == 0 ||
                     (*p_ptr).stun as libc::c_int != 0) &&
               pattern_seq((*p_ptr).py as libc::c_int,
                           (*p_ptr).px as libc::c_int, y, x) as libc::c_int !=
                   0 &&
               player_can_enter((*cave[y as usize].offset(x as isize)).feat)
                   as libc::c_int != 0 {
            (*m_ptr).csleep = 0 as libc::c_int as s16b;
            /* now continue on to 'movement' */
            monster_desc(m_name.as_mut_ptr(), m_ptr, 0 as libc::c_int);
            if (*m_ptr).ml != 0 {
                monster_race_track((*m_ptr).r_idx as libc::c_int,
                                   (*m_ptr).ego as libc::c_int);
            }
            if (*m_ptr).ml != 0 {
                health_track((*c_ptr).m_idx as libc::c_int);
            }
            if stormbringer as libc::c_int != 0 &&
                   Rand_div(1000 as libc::c_int) + 1 as libc::c_int >
                       666 as libc::c_int {
                py_attack(y, x, -(1 as libc::c_int));
            } else if (*f_info.offset((*cave[(*p_ptr).py as
                                                 usize].offset((*p_ptr).px as
                                                                   isize)).feat
                                          as isize)).flags1 as libc::c_long &
                          0x10 as libc::c_long != 0 &&
                          (*cave[(*p_ptr).py as
                                     usize].offset((*p_ptr).px as isize)).feat
                              as libc::c_int != 0xaf as libc::c_int ||
                          (*mr_ptr).flags2 &
                              0x40000 as libc::c_int as libc::c_uint != 0 {
                msg_format(b"You push past %s.\x00" as *const u8 as
                               *const libc::c_char, m_name.as_mut_ptr());
                (*m_ptr).fy = (*p_ptr).py as byte_hack;
                (*m_ptr).fx = (*p_ptr).px as byte_hack;
                (*cave[(*p_ptr).py as
                           usize].offset((*p_ptr).px as isize)).m_idx =
                    (*c_ptr).m_idx;
                (*c_ptr).m_idx = 0 as libc::c_int as s16b;
                update_mon((*cave[(*p_ptr).py as
                                      usize].offset((*p_ptr).px as
                                                        isize)).m_idx as
                               libc::c_int, 1 as libc::c_int as bool_);
            } else {
                msg_format(b"%^s is in your way!\x00" as *const u8 as
                               *const libc::c_char, m_name.as_mut_ptr());
                energy_use = 0 as libc::c_int;
                oktomove = 0 as libc::c_int as bool_
            }
        } else {
            py_attack(y, x, -(1 as libc::c_int));
            oktomove = 0 as libc::c_int as bool_
        }
    } else if (*c_ptr).feat as libc::c_int == 0x57 as libc::c_int &&
                  (*p_ptr).ffall == 0 {
        msg_print(b"You can\'t cross the chasm.\x00" as *const u8 as
                      *const libc::c_char);
        running = 0 as libc::c_int as s16b;
        oktomove = 0 as libc::c_int as bool_
    } else if easy_disarm as libc::c_int != 0 && disarm as libc::c_int != 0 &&
                  (*c_ptr).info as libc::c_int & 0x100 as libc::c_int != 0 {
        do_cmd_disarm_aux(y, x, tmp, do_pickup);
        return
    } else {
        /* Extract monster name (or "it") */
        /* Auto-Recall if possible and visible */
        /* Track a new monster */
        /* displace? */
        /* Disarm a visible trap */
        /* Don't step on known traps. */
        if disarm as libc::c_int != 0 &&
               (*c_ptr).info as libc::c_int & 0x100 as libc::c_int != 0 &&
               !((*p_ptr).confused as libc::c_int != 0 ||
                     (*p_ptr).stun as libc::c_int != 0 ||
                     (*p_ptr).image as libc::c_int != 0) {
            msg_print(b"You stop to avoid triggering the trap.\x00" as
                          *const u8 as *const libc::c_char);
            energy_use = 0 as libc::c_int;
            oktomove = 0 as libc::c_int as bool_
        } else if player_can_enter((*c_ptr).feat) == 0 {
            oktomove = 0 as libc::c_int as bool_;
            /* Player can't enter ? soo bad for him/her ... */
            /* Disturb the player */
            disturb(0 as libc::c_int, 0 as libc::c_int);
            if (*p_ptr).prob_travel != 0 {
                if passwall(tmp, 1 as libc::c_int as bool_) != 0 { return }
            }
            /* Notice things in the dark */
            if (*c_ptr).info as libc::c_int & 0x1 as libc::c_int == 0 &&
                   (*c_ptr).info as libc::c_int & 0x10 as libc::c_int == 0 {
                /* Rubble */
                if (*c_ptr).feat as libc::c_int == 0x31 as libc::c_int {
                    msg_print(b"You feel some rubble blocking your way.\x00"
                                  as *const u8 as *const libc::c_char);
                    (*c_ptr).info =
                        ((*c_ptr).info as libc::c_int | 0x1 as libc::c_int) as
                            u16b;
                    lite_spot(y, x);
                } else if ((*c_ptr).feat as libc::c_int) < 0x30 as libc::c_int
                 {
                    msg_print(b"You feel a closed door blocking your way.\x00"
                                  as *const u8 as *const libc::c_char);
                    (*c_ptr).info =
                        ((*c_ptr).info as libc::c_int | 0x1 as libc::c_int) as
                            u16b;
                    lite_spot(y, x);
                } else {
                    /* Closed door */
                    /* Wall (or secret door) */
                    let mut feat: libc::c_int = 0;
                    if (*c_ptr).mimic != 0 {
                        feat = (*c_ptr).mimic as libc::c_int
                    } else {
                        feat =
                            (*f_info.offset((*c_ptr).feat as isize)).mimic as
                                libc::c_int
                    }
                    msg_format(b"You feel %s.\x00" as *const u8 as
                                   *const libc::c_char,
                               f_text.offset((*f_info.offset(feat as
                                                                 isize)).block
                                                 as isize));
                    (*c_ptr).info =
                        ((*c_ptr).info as libc::c_int | 0x1 as libc::c_int) as
                            u16b;
                    lite_spot(y, x);
                }
            } else if (*c_ptr).feat as libc::c_int == 0x31 as libc::c_int {
                if easy_tunnel == 0 {
                    msg_print(b"There is rubble blocking your way.\x00" as
                                  *const u8 as *const libc::c_char);
                    if !((*p_ptr).confused as libc::c_int != 0 ||
                             (*p_ptr).stun as libc::c_int != 0 ||
                             (*p_ptr).image as libc::c_int != 0) {
                        energy_use = 0 as libc::c_int
                    }
                    /* Notice things */
                    /* Rubble */
                    /*
					 * Well, it makes sense that you lose time bumping into
					 * a wall _if_ you are confused, stunned or blind; but
					 * typing mistakes should not cost you a turn...
					 */
                } else { do_cmd_tunnel_aux(y, x, dir); return }
            } else if (*c_ptr).feat as libc::c_int >= 0x20 as libc::c_int &&
                          (*c_ptr).feat as libc::c_int <= 0x2f as libc::c_int
             {
                if easy_open != 0 {
                    if easy_open_door(y, x) != 0 { return }
                } else {
                    msg_print(b"There is a closed door blocking your way.\x00"
                                  as *const u8 as *const libc::c_char);
                    if !((*p_ptr).confused as libc::c_int != 0 ||
                             (*p_ptr).stun as libc::c_int != 0 ||
                             (*p_ptr).image as libc::c_int != 0) {
                        energy_use = 0 as libc::c_int
                    }
                }
            } else if easy_tunnel == 0 {
                let mut feat_0: libc::c_int = 0;
                if (*c_ptr).mimic != 0 {
                    feat_0 = (*c_ptr).mimic as libc::c_int
                } else {
                    feat_0 =
                        (*f_info.offset((*c_ptr).feat as isize)).mimic as
                            libc::c_int
                }
                msg_format(b"There is %s.\x00" as *const u8 as
                               *const libc::c_char,
                           f_text.offset((*f_info.offset(feat_0 as
                                                             isize)).block as
                                             isize));
                if !((*p_ptr).confused as libc::c_int != 0 ||
                         (*p_ptr).stun as libc::c_int != 0 ||
                         (*p_ptr).image as libc::c_int != 0) {
                    energy_use = 0 as libc::c_int
                }
            } else { do_cmd_tunnel_aux(y, x, dir); return }
            /* Closed doors */
            /* Wall (or secret door) */
            /* Sound */
            sound(15 as libc::c_int);
        }
    }
    /* Normal movement */
    if pattern_seq((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int, y,
                   x) == 0 {
        if !((*p_ptr).confused as libc::c_int != 0 ||
                 (*p_ptr).stun as libc::c_int != 0 ||
                 (*p_ptr).image as libc::c_int != 0) {
            energy_use = 0 as libc::c_int
        } /* To avoid a loop with running */
        disturb(0 as libc::c_int, 0 as libc::c_int);
        oktomove = 0 as libc::c_int as bool_
    }
    /*
	 * Check trap detection status -- retrieve them here
	 * because they are used by the movement code as well
	 */
    old_dtrap =
        ((*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).info as
             libc::c_int & 0x1000 as libc::c_int != 0 as libc::c_int) as
            libc::c_int as bool_;
    new_dtrap =
        ((*cave[y as usize].offset(x as isize)).info as libc::c_int &
             0x1000 as libc::c_int != 0 as libc::c_int) as libc::c_int as
            bool_;
    /* Normal movement */
    if oktomove as libc::c_int != 0 && running as libc::c_int != 0 &&
           disturb_detect as libc::c_int != 0 {
        /*
		 * Disturb the player when about to leave the trap detected
		 * area
		 */
        if old_dtrap as libc::c_int != 0 && new_dtrap == 0 {
            /* Disturb player */
            disturb(0 as libc::c_int, 0 as libc::c_int);
            /* but don't take a turn */
            energy_use = 0 as libc::c_int;
            /* Tell player why */
            cmsg_print(10 as libc::c_int as byte_hack,
                       b"You are about to leave a trap detected zone.\x00" as
                           *const u8 as *const libc::c_char);
            /* Flush */
			/* msg_print(NULL); */
            oktomove = 0 as libc::c_int as bool_
        }
    }
    /* Normal movement */
    if oktomove != 0 {
        let mut oy: libc::c_int = 0;
        let mut ox: libc::c_int = 0;
        let mut feat_1: libc::c_int = 0;
        /* Rooted means no move */
        if (*p_ptr).tim_roots != 0 { return }
        /* Save old location */
        oy = (*p_ptr).py as libc::c_int;
        ox = (*p_ptr).px as libc::c_int;
        /* Move the player */
        (*p_ptr).py = y as s16b;
        (*p_ptr).px = x as s16b;
        if (*cave[(*p_ptr).py as usize].offset((*p_ptr).px as isize)).mimic !=
               0 {
            feat_1 =
                (*cave[(*p_ptr).py as
                           usize].offset((*p_ptr).px as isize)).mimic as
                    libc::c_int
        } else {
            feat_1 =
                (*cave[(*p_ptr).py as
                           usize].offset((*p_ptr).px as isize)).feat as
                    libc::c_int
        }
        /* Some hooks */
        if process_hooks(71 as libc::c_int,
                         b"(d,d)\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char, oy, ox) != 0 {
            return
        }
        /* Redraw new spot */
        lite_spot((*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int);
        /* Redraw old spot */
        lite_spot(oy, ox);
        /* Sound */
		/* sound(SOUND_WALK); */
        /* Check for new panel (redraw map) */
        verify_panel();
        /* Check detection status */
        if old_dtrap as libc::c_int != 0 && new_dtrap == 0 {
            cmsg_print(10 as libc::c_int as byte_hack,
                       b"You leave a trap detected zone.\x00" as *const u8 as
                           *const libc::c_char);
            if running != 0 { msg_print(0 as cptr); }
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x20000000 as libc::c_long)
                    as u32b
        } else if old_dtrap == 0 && new_dtrap as libc::c_int != 0 {
            cmsg_print(14 as libc::c_int as byte_hack,
                       b"You enter a trap detected zone.\x00" as *const u8 as
                           *const libc::c_char);
            if running != 0 { msg_print(0 as cptr); }
            (*p_ptr).redraw =
                ((*p_ptr).redraw as libc::c_long | 0x20000000 as libc::c_long)
                    as u32b
        }
        /* Update stuff */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long |
                 (0x100000 as libc::c_long | 0x10000000 as libc::c_long |
                      0x200000 as libc::c_long)) as u32b;
        /* Update the monsters */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x2000000 as libc::c_long) as
                u32b;
        /* Window stuff */
        if run == 0 {
            (*p_ptr).window =
                ((*p_ptr).window as libc::c_long | 0x80 as libc::c_long) as
                    u32b
        }
        /* Some feature descs */
        if (*f_info.offset((*cave[(*p_ptr).py as
                                      usize].offset((*p_ptr).px as
                                                        isize)).feat as
                               isize)).text > 1 as libc::c_int as libc::c_uint
           {
            /* Mega-hack for dungeon branches */
            if feat_1 == 0x7 as libc::c_int &&
                   (*c_ptr).special as libc::c_int != 0 {
                msg_format(b"There is %s\x00" as *const u8 as
                               *const libc::c_char,
                           d_text.offset((*d_info.offset((*c_ptr).special as
                                                             isize)).text as
                                             isize));
            } else {
                msg_print(f_text.offset((*f_info.offset(feat_1 as isize)).text
                                            as isize) as cptr);
            }
            /* Flush message while running */
            if running != 0 { msg_print(0 as cptr); }
        }
        /* Spontaneous Searching */
        if (*p_ptr).skill_fos as libc::c_int >= 50 as libc::c_int ||
               0 as libc::c_int ==
                   Rand_div(50 as libc::c_int -
                                (*p_ptr).skill_fos as libc::c_int) {
            search();
        }
        /* Continuous Searching */
        if (*p_ptr).searching != 0 { search(); }
        /* Handle "objects" */
        carry(do_pickup);
        /* Handle "store doors" */
        if (*c_ptr).feat as libc::c_int == 0x4a as libc::c_int {
            /* Disturb */
            disturb(0 as libc::c_int, 0 as libc::c_int);
            /* Hack -- Enter store */
            command_new = '_' as i32 as s16b
        } else if (*cave[y as usize].offset(x as isize)).feat as libc::c_int
                      >= 0xa1 as libc::c_int &&
                      (*cave[y as usize].offset(x as isize)).feat as
                          libc::c_int <= 0xab as libc::c_int {
            let mut name: cptr =
                f_name.offset((*f_info.offset((*cave[y as
                                                         usize].offset(x as
                                                                           isize)).feat
                                                  as isize)).name as isize) as
                    cptr;
            let mut pref: cptr =
                if is_a_vowel(*name.offset(0 as libc::c_int as isize) as
                                  libc::c_int) as libc::c_int != 0 {
                    b"an\x00" as *const u8 as *const libc::c_char
                } else { b"a\x00" as *const u8 as *const libc::c_char };
            msg_format(b"You see %s %s.\x00" as *const u8 as
                           *const libc::c_char, pref, name);
            /* Flush message while running */
            if running != 0 { msg_print(0 as cptr); }
        } else if (*c_ptr).t_idx as libc::c_int != 0 as libc::c_int &&
                      (*f_info.offset((*cave[y as
                                                 usize].offset(x as
                                                                   isize)).feat
                                          as isize)).flags1 as libc::c_long &
                          0x1000 as libc::c_long == 0 {
            /* Discover invisible traps */
            /* Disturb */
            disturb(0 as libc::c_int, 0 as libc::c_int);
            if (*c_ptr).info as libc::c_int & 0x100 as libc::c_int == 0 {
                /* Message */
                msg_print(b"You found a trap!\x00" as *const u8 as
                              *const libc::c_char);
                /* Pick a trap */
                pick_trap((*p_ptr).py as libc::c_int,
                          (*p_ptr).px as libc::c_int);
            }
            /* Hit the trap */
            hit_trap();
        } else if (*c_ptr).inscription != 0 {
            /* Execute the inscription */
            /* Disturb */
            disturb(0 as libc::c_int, 0 as libc::c_int);
            msg_format(b"There is an inscription here: %s\x00" as *const u8 as
                           *const libc::c_char,
                       inscription_info[(*c_ptr).inscription as
                                            usize].text.as_mut_ptr());
            if inscription_info[(*c_ptr).inscription as usize].when as
                   libc::c_int & 0x2 as libc::c_int != 0 {
                execute_inscription((*c_ptr).inscription as byte_hack,
                                    (*p_ptr).py as byte_hack,
                                    (*p_ptr).px as byte_hack);
            }
        }
    }
    /* Update wilderness knowledge */
    if (*p_ptr).wild_mode != 0 {
        if wizard != 0 {
            msg_format(b"y:%d, x:%d\x00" as *const u8 as *const libc::c_char,
                       (*p_ptr).py as libc::c_int,
                       (*p_ptr).px as libc::c_int);
        }
        /* Update the known wilderness */
        reveal_wilderness_around_player((*p_ptr).py as libc::c_int,
                                        (*p_ptr).px as libc::c_int,
                                        0 as libc::c_int, 3 as libc::c_int);
        /* Walking the wild isnt meaningfull */
        (*p_ptr).did_nothing = 1 as libc::c_int as bool_
    };
}
#[no_mangle]
pub unsafe extern "C" fn move_player(mut dir: libc::c_int,
                                     mut do_pickup: libc::c_int,
                                     mut disarm: bool_) {
    move_player_aux(dir, do_pickup, 0 as libc::c_int, disarm);
}
/*
 * Hack -- Grid-based version of see_obstacle
 */
unsafe extern "C" fn see_obstacle_grid(mut c_ptr: *mut cave_type)
 -> libc::c_int {
    /*
	 * Hack -- Avoid hitting detected traps, because we cannot rely on
	 * the CAVE_MARK check below, and traps can be set to nearly
	 * everything the player can move on to XXX XXX XXX
	 */
    if (*c_ptr).info as libc::c_int & 0x100 as libc::c_int != 0 {
        return 1 as libc::c_int
    }
    let mut current_block_7: u64;
    /* Hack -- Handle special cases XXX XXX */
    match (*c_ptr).feat as libc::c_int {
        87 | 187 | 90 => {
            /* Require levitation */
            if (*p_ptr).ffall as libc::c_int != 0 ||
                   (*p_ptr).fly as libc::c_int != 0 {
                return 0 as libc::c_int
            }
            current_block_7 = 12053711620828533692;
        }
        85 | 86 => { current_block_7 = 12053711620828533692; }
        _ => { current_block_7 = 11812396948646013369; }
    }
    match current_block_7 {
        12053711620828533692 =>
        /* Require immunity */
        {
            if (*p_ptr).invuln as libc::c_int != 0 ||
                   (*p_ptr).immune_fire as libc::c_int != 0 {
                return 0 as libc::c_int
            }
        }
        _ => { }
    }
    /* "Safe" floor grids aren't obstacles */
    if (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long &
           0x800 as libc::c_long != 0 {
        return 0 as libc::c_int
    }
    /* Must be known to the player */
    if (*c_ptr).info as libc::c_int & 0x1 as libc::c_int == 0 {
        return 0 as libc::c_int
    }
    /* Default */
    return 1 as libc::c_int;
}
/*
 * Hack -- Check for a "known wall" or "dangerous" feature (see below)
 */
unsafe extern "C" fn see_obstacle(mut dir: libc::c_int, mut y: libc::c_int,
                                  mut x: libc::c_int) -> libc::c_int {
    /* Get the new location */
    y += ddy[dir as usize] as libc::c_int;
    x += ddx[dir as usize] as libc::c_int;
    /* Illegal grids are not known walls */
    if !(y >= 0 as libc::c_int && x >= 0 as libc::c_int &&
             y < cur_hgt as libc::c_int && x < cur_wid as libc::c_int) {
        return 0 as libc::c_int
    }
    /* Analyse the grid */
    return see_obstacle_grid(&mut *(*cave.as_mut_ptr().offset(y as
                                                                  isize)).offset(x
                                                                                     as
                                                                                     isize));
}
/*
 * Hack -- Check for an "unknown corner" (see below)
 */
unsafe extern "C" fn see_nothing(mut dir: libc::c_int, mut y: libc::c_int,
                                 mut x: libc::c_int) -> libc::c_int {
    /* Get the new location */
    y += ddy[dir as usize] as libc::c_int;
    x += ddx[dir as usize] as libc::c_int;
    /* Illegal grids are unknown */
    if !(y >= 0 as libc::c_int && x >= 0 as libc::c_int &&
             y < cur_hgt as libc::c_int && x < cur_wid as libc::c_int) {
        return 1 as libc::c_int
    }
    /* Memorized grids are always known */
    if (*cave[y as usize].offset(x as isize)).info as libc::c_int &
           0x1 as libc::c_int != 0 {
        return 0 as libc::c_int
    }
    /* Non-floor grids are unknown */
    if !((*f_info.offset((*cave[y as usize].offset(x as isize)).feat as
                             isize)).flags1 as libc::c_long &
             0x10 as libc::c_long != 0 &&
             (*cave[y as usize].offset(x as isize)).feat as libc::c_int !=
                 0xaf as libc::c_int) {
        return 1 as libc::c_int
    }
    /* Viewable door/wall grids are known */
    if (*cave[y as usize].offset(x as isize)).info as libc::c_int &
           0x10 as libc::c_int != 0 as libc::c_int {
        return 0 as libc::c_int
    }
    /* Default */
    return 1 as libc::c_int;
}
/*
 * The running algorithm:                       -CJS-
 *
 * In the diagrams below, the player has just arrived in the
 * grid marked as '@', and he has just come from a grid marked
 * as 'o', and he is about to enter the grid marked as 'x'.
 *
 * Of course, if the "requested" move was impossible, then you
 * will of course be blocked, and will stop.
 *
 * Overview: You keep moving until something interesting happens.
 * If you are in an enclosed space, you follow corners. This is
 * the usual corridor scheme. If you are in an open space, you go
 * straight, but stop before entering enclosed space. This is
 * analogous to reaching doorways. If you have enclosed space on
 * one side only (that is, running along side a wall) stop if
 * your wall opens out, or your open space closes in. Either case
 * corresponds to a doorway.
 *
 * What happens depends on what you can really SEE. (i.e. if you
 * have no light, then running along a dark corridor is JUST like
 * running in a dark room.) The algorithm works equally well in
 * corridors, rooms, mine tailings, earthquake rubble, etc, etc.
 *
 * These conditions are kept in static memory:
 * find_openarea         You are in the open on at least one
 * side.
 * find_breakleft        You have a wall on the left, and will
 * stop if it opens
 * find_breakright       You have a wall on the right, and will
 * stop if it opens
 *
 * To initialize these conditions, we examine the grids adjacent
 * to the grid marked 'x', two on each side (marked 'L' and 'R').
 * If either one of the two grids on a given side is seen to be
 * closed, then that side is considered to be closed. If both
 * sides are closed, then it is an enclosed (corridor) run.
 *
 * LL           L
 * @x          LxR
 * RR          @R
 *
 * Looking at more than just the immediate squares is
 * significant. Consider the following case. A run along the
 * corridor will stop just before entering the center point,
 * because a choice is clearly established. Running in any of
 * three available directions will be defined as a corridor run.
 * Note that a minor hack is inserted to make the angled corridor
 * entry (with one side blocked near and the other side blocked
 * further away from the runner) work correctly. The runner moves
 * diagonally, but then saves the previous direction as being
 * straight into the gap. Otherwise, the tail end of the other
 * entry would be perceived as an alternative on the next move.
 *
 * #.#
 * ##.##
 * .@x..
 * ##.##
 * #.#
 *
 * Likewise, a run along a wall, and then into a doorway (two
 * runs) will work correctly. A single run rightwards from @ will
 * stop at 1. Another run right and down will enter the corridor
 * and make the corner, stopping at the 2.
 *
 * #@x    1
 * ########### ######
 * 2        #
 * #############
 * #
 *
 * After any move, the function area_affect is called to
 * determine the new surroundings, and the direction of
 * subsequent moves. It examines the current player location
 * (at which the runner has just arrived) and the previous
 * direction (from which the runner is considered to have come).
 *
 * Moving one square in some direction places you adjacent to
 * three or five new squares (for straight and diagonal moves
 * respectively) to which you were not previously adjacent,
 * marked as '!' in the diagrams below.
 *
 * ...!   ...
 * .o@!   .o.!
 * ...!   ..@!
 * !!!
 *
 * You STOP if any of the new squares are interesting in any way:
 * for example, if they contain visible monsters or treasure.
 *
 * You STOP if any of the newly adjacent squares seem to be open,
 * and you are also looking for a break on that side. (that is,
 * find_openarea AND find_break).
 *
 * You STOP if any of the newly adjacent squares do NOT seem to be
 * open and you are in an open area, and that side was previously
 * entirely open.
 *
 * Corners: If you are not in the open (i.e. you are in a corridor)
 * and there is only one way to go in the new squares, then turn in
 * that direction. If there are more than two new ways to go, STOP.
 * If there are two ways to go, and those ways are separated by a
 * square which does not seem to be open, then STOP.
 *
 * Otherwise, we have a potential corner. There are two new open
 * squares, which are also adjacent. One of the new squares is
 * diagonally located, the other is straight on (as in the diagram).
 * We consider two more squares further out (marked below as ?).
 *
 * We assign "option" to the straight-on grid, and "option2" to the
 * diagonal grid, and "check_dir" to the grid marked 's'.
 *
 * .s
 * @x?
 * #?
 *
 * If they are both seen to be closed, then it is seen that no
 * benefit is gained from moving straight. It is a known corner.
 * To cut the corner, go diagonally, otherwise go straight, but
 * pretend you stepped diagonally into that next location for a
 * full view next time. Conversely, if one of the ? squares is
 * not seen to be closed, then there is a potential choice. We check
 * to see whether it is a potential corner or an intersection/room entrance.
 * If the square two spaces straight ahead, and the space marked with 's'
 * are both blank, then it is a potential corner and enter if find_examine
 * is set, otherwise must stop because it is not a corner.
 */
/*
 * Hack -- allow quick "cycling" through the legal directions
 */
static mut cycle: [byte_hack; 17] =
    [1 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack, 2 as libc::c_int as byte_hack,
     3 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack,
     1 as libc::c_int as byte_hack];
/*
 * Hack -- map each direction into the "middle" of the "cycle[]" array
 */
static mut chome: [byte_hack; 10] =
    [0 as libc::c_int as byte_hack, 8 as libc::c_int as byte_hack,
     9 as libc::c_int as byte_hack, 10 as libc::c_int as byte_hack,
     7 as libc::c_int as byte_hack, 0 as libc::c_int as byte_hack,
     11 as libc::c_int as byte_hack, 6 as libc::c_int as byte_hack,
     5 as libc::c_int as byte_hack, 4 as libc::c_int as byte_hack];
/*
 * The direction we are running
 */
static mut find_current: byte_hack = 0;
/*
 * The direction we came from
 */
static mut find_prevdir: byte_hack = 0;
/*
 * We are looking for open area
 */
static mut find_openarea: bool_ = 0;
/*
 * We are looking for a break
 */
static mut find_breakright: bool_ = 0;
static mut find_breakleft: bool_ = 0;
/*
 * Initialize the running algorithm for a new direction.
 *
 * Diagonal Corridor -- allow diaginal entry into corridors.
 *
 * Blunt Corridor -- If there is a wall two spaces ahead and
 * we seem to be in a corridor, then force a turn into the side
 * corridor, must be moving straight into a corridor here. ???
 *
 * Diagonal Corridor    Blunt Corridor (?)
 *       # #                  #
 *       #x#                 @x#
 *       @p.                  p
 */
unsafe extern "C" fn run_init(mut dir: libc::c_int) {
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut deepleft: libc::c_int = 0;
    let mut deepright: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut shortleft: libc::c_int = 0;
    let mut shortright: libc::c_int = 0;
    /* Save the direction */
    find_current = dir as byte_hack;
    /* Assume running straight */
    find_prevdir = dir as byte_hack;
    /* Assume looking for open area */
    find_openarea = 1 as libc::c_int as bool_;
    /* Assume not looking for breaks */
    find_breakleft = 0 as libc::c_int as bool_;
    find_breakright = find_breakleft;
    /* Assume no nearby walls */
    deepright = 0 as libc::c_int;
    deepleft = deepright;
    shortleft = 0 as libc::c_int;
    shortright = shortleft;
    /* Find the destination grid */
    row = (*p_ptr).py as libc::c_int + ddy[dir as usize] as libc::c_int;
    col = (*p_ptr).px as libc::c_int + ddx[dir as usize] as libc::c_int;
    /* Extract cycle index */
    i = chome[dir as usize] as libc::c_int;
    /* Check for walls */
    if see_obstacle(cycle[(i + 1 as libc::c_int) as usize] as libc::c_int,
                    (*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int) !=
           0 {
        find_breakleft = 1 as libc::c_int as bool_;
        shortleft = 1 as libc::c_int
    } else if see_obstacle(cycle[(i + 1 as libc::c_int) as usize] as
                               libc::c_int, row, col) != 0 {
        find_breakleft = 1 as libc::c_int as bool_;
        deepleft = 1 as libc::c_int
    }
    /* Check for walls */
    if see_obstacle(cycle[(i - 1 as libc::c_int) as usize] as libc::c_int,
                    (*p_ptr).py as libc::c_int, (*p_ptr).px as libc::c_int) !=
           0 {
        find_breakright = 1 as libc::c_int as bool_;
        shortright = 1 as libc::c_int
    } else if see_obstacle(cycle[(i - 1 as libc::c_int) as usize] as
                               libc::c_int, row, col) != 0 {
        find_breakright = 1 as libc::c_int as bool_;
        deepright = 1 as libc::c_int
    }
    /* Looking for a break */
    if find_breakleft as libc::c_int != 0 &&
           find_breakright as libc::c_int != 0 {
        /* Not looking for open area */
        find_openarea = 0 as libc::c_int as bool_;
        /* Hack -- allow angled corridor entry */
        if dir & 0x1 as libc::c_int != 0 {
            if deepleft != 0 && deepright == 0 {
                find_prevdir = cycle[(i - 1 as libc::c_int) as usize]
            } else if deepright != 0 && deepleft == 0 {
                find_prevdir = cycle[(i + 1 as libc::c_int) as usize]
            }
        } else if see_obstacle(cycle[i as usize] as libc::c_int, row, col) !=
                      0 {
            if shortleft != 0 && shortright == 0 {
                find_prevdir = cycle[(i - 2 as libc::c_int) as usize]
            } else if shortright != 0 && shortleft == 0 {
                find_prevdir = cycle[(i + 2 as libc::c_int) as usize]
            }
        }
    };
}
/* Hack -- allow blunt corridor entry */
/*
 * Update the current "run" path
 *
 * Return TRUE if the running should be stopped
 */
unsafe extern "C" fn run_test() -> bool_ {
    let mut prev_dir: libc::c_int = 0;
    let mut new_dir: libc::c_int = 0;
    let mut check_dir: libc::c_int = 0 as libc::c_int;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut inv: libc::c_int = 0;
    let mut option: libc::c_int = 0 as libc::c_int;
    let mut option2: libc::c_int = 0 as libc::c_int;
    let mut c_ptr: *mut cave_type = 0 as *mut cave_type;
    /* Where we came from */
    prev_dir = find_prevdir as libc::c_int;
    /* Range of newly adjacent grids */
    max = (prev_dir & 0x1 as libc::c_int) + 1 as libc::c_int;
    /* Look at every newly adjacent square. */
    i = -max;
    while i <= max {
        let mut this_o_idx: s16b = 0;
        let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
        /* New direction */
        new_dir =
            cycle[(chome[prev_dir as usize] as libc::c_int + i) as usize] as
                libc::c_int;
        /* New location */
        row =
            (*p_ptr).py as libc::c_int + ddy[new_dir as usize] as libc::c_int;
        col =
            (*p_ptr).px as libc::c_int + ddx[new_dir as usize] as libc::c_int;
        /* Access grid */
        c_ptr =
            &mut *(*cave.as_mut_ptr().offset(row as
                                                 isize)).offset(col as isize)
                as *mut cave_type;
        /* Visible monsters abort running */
        if (*c_ptr).m_idx != 0 {
            let mut m_ptr: *mut monster_type =
                &mut *m_list.offset((*c_ptr).m_idx as isize) as
                    *mut monster_type;
            /* Visible monster */
            if (*m_ptr).ml != 0 { return 1 as libc::c_int as bool_ }
        }
        /* Visible objects abort running */
        this_o_idx = (*c_ptr).o_idx;
        while this_o_idx != 0 {
            let mut o_ptr: *mut object_type = 0 as *mut object_type;
            /* Acquire object */
            o_ptr =
                &mut *o_list.offset(this_o_idx as isize) as *mut object_type;
            /* Acquire next object */
            next_o_idx = (*o_ptr).next_o_idx;
            /* Visible object */
            if (*o_ptr).marked != 0 { return 1 as libc::c_int as bool_ }
            this_o_idx = next_o_idx
        }
        /* Assume unknown */
        inv = 1 as libc::c_int;
        /* Check memorized grids */
        if (*c_ptr).info as libc::c_int & 0x1 as libc::c_int != 0 {
            let mut notice: bool_ = 1 as libc::c_int as bool_;
            /*
			 * Examine the terrain -- conditional disturbance
			 * If we had more flags, we could make these customisable too
			 */
            match (*c_ptr).feat as libc::c_int {
                85 | 86 => {
                    /* Ignore */
                    if (*p_ptr).invuln as libc::c_int != 0 ||
                           (*p_ptr).immune_fire as libc::c_int != 0 {
                        notice = 0 as libc::c_int as bool_
                    }
                }
                187 | 90 => {
                    /* Ignore */
                    if (*p_ptr).ffall as libc::c_int != 0 ||
                           (*p_ptr).fly as libc::c_int != 0 {
                        notice = 0 as libc::c_int as bool_
                    }
                }
                4 | 5 => {
                    /* Open doors */
                    /* Option -- ignore */
                    if find_ignore_doors != 0 {
                        notice = 0 as libc::c_int as bool_
                    }
                }
                6 | 7 | 8 | 9 | 10 | 11 | 14 | 13 | 180 | 179 | 160 | 176 => {
                    /* XXX */
                    /* Option -- ignore */
                    if find_ignore_stairs != 0 {
                        notice = 0 as libc::c_int as bool_
                    }
                }
                _ => { }
            }
            /* Check the "don't notice running" flag */
            if (*f_info.offset((*c_ptr).feat as isize)).flags1 as libc::c_long
                   & 0x400 as libc::c_long != 0 {
                notice = 0 as libc::c_int as bool_
            }
            /* A detected trap is interesting */
            if (*c_ptr).info as libc::c_int & 0x100 as libc::c_int != 0 {
                notice = 1 as libc::c_int as bool_
            }
            /* Interesting feature */
            if notice != 0 { return 1 as libc::c_int as bool_ }
            /* The grid is "visible" */
            inv = 0 as libc::c_int
        }
        /* Mega-Hack -- Maze code removes CAVE_MARK XXX XXX XXX */
        if (*c_ptr).info as libc::c_int & 0x100 as libc::c_int != 0 {
            return 1 as libc::c_int as bool_
        }
        /* Analyze unknown grids and floors */
        if inv != 0 ||
               (*f_info.offset((*cave[row as usize].offset(col as isize)).feat
                                   as isize)).flags1 as libc::c_long &
                   0x10 as libc::c_long != 0 &&
                   (*cave[row as usize].offset(col as isize)).feat as
                       libc::c_int != 0xaf as libc::c_int {
            /* Looking for open area */
            if !(find_openarea != 0) {
                /* The first new direction. */
                if option == 0 {
                    option = new_dir
                } else if option2 != 0 {
                    return 1 as libc::c_int as bool_
                } else {
                    /* Three new directions. Stop running. */
                    /* Two non-adjacent new directions.  Stop running. */
                    if option !=
                           cycle[(chome[prev_dir as usize] as libc::c_int + i
                                      - 1 as libc::c_int) as usize] as
                               libc::c_int {
                        return 1 as libc::c_int as bool_
                    } else {
                        /* Two new (adjacent) directions (case 1) */
                        if new_dir & 0x1 as libc::c_int != 0 {
                            check_dir =
                                cycle[(chome[prev_dir as usize] as libc::c_int
                                           + i - 2 as libc::c_int) as usize]
                                    as libc::c_int;
                            option2 = new_dir
                        } else {
                            /* Two new (adjacent) directions (case 2) */
                            check_dir =
                                cycle[(chome[prev_dir as usize] as libc::c_int
                                           + i + 1 as libc::c_int) as usize]
                                    as libc::c_int;
                            option2 = option;
                            option = new_dir
                        }
                    }
                }
            }
        } else if find_openarea != 0 {
            if i < 0 as libc::c_int {
                /* Obstacle, while looking for open area */
                /* Break to the right */
                find_breakright = 1 as libc::c_int as bool_
            } else if i > 0 as libc::c_int {
                /* Break to the left */
                find_breakleft = 1 as libc::c_int as bool_
            }
        }
        i += 1
    }
    /* Looking for open area */
    if find_openarea != 0 {
        /* Hack -- look again */
        i = -max;
        while i < 0 as libc::c_int {
            new_dir =
                cycle[(chome[prev_dir as usize] as libc::c_int + i) as usize]
                    as libc::c_int;
            row =
                (*p_ptr).py as libc::c_int +
                    ddy[new_dir as usize] as libc::c_int;
            col =
                (*p_ptr).px as libc::c_int +
                    ddx[new_dir as usize] as libc::c_int;
            /* Access grid */
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset(row as
                                                     isize)).offset(col as
                                                                        isize)
                    as *mut cave_type;
            /* Unknown grids or non-obstacle */
            if see_obstacle_grid(c_ptr) == 0 {
                /* Looking to break right */
                if find_breakright != 0 { return 1 as libc::c_int as bool_ }
            } else if find_breakleft != 0 { return 1 as libc::c_int as bool_ }
            i += 1
        }
        /* Obstacle */
        /* Looking to break left */
        /* Hack -- look again */
        i = max;
        while i > 0 as libc::c_int {
            new_dir =
                cycle[(chome[prev_dir as usize] as libc::c_int + i) as usize]
                    as libc::c_int;
            row =
                (*p_ptr).py as libc::c_int +
                    ddy[new_dir as usize] as libc::c_int;
            col =
                (*p_ptr).px as libc::c_int +
                    ddx[new_dir as usize] as libc::c_int;
            /* Access grid */
            c_ptr =
                &mut *(*cave.as_mut_ptr().offset(row as
                                                     isize)).offset(col as
                                                                        isize)
                    as *mut cave_type;
            /* Unknown grid or non-obstacle */
            if see_obstacle_grid(c_ptr) == 0 {
                /* Looking to break left */
                if find_breakleft != 0 { return 1 as libc::c_int as bool_ }
            } else if find_breakright != 0 {
                return 1 as libc::c_int as bool_
            }
            i -= 1
        }
    } else if option == 0 {
        return 1 as libc::c_int as bool_
    } else {
        /* Obstacle */
        /* Looking to break right */
        /* Not looking for open area */
        /* No options */
        /* One option */
        if option2 == 0 {
            /* Primary option */
            find_current = option as byte_hack;
            /* No other options */
            find_prevdir = option as byte_hack
        } else if find_examine as libc::c_int != 0 && find_cut == 0 {
            /* Two options, examining corners */
            /* Primary option */
            find_current = option as byte_hack;
            /* Hack -- allow curving */
            find_prevdir = option2 as byte_hack
        } else {
            /* Two options, pick one */
            /* Get next location */
            row =
                (*p_ptr).py as libc::c_int +
                    ddy[option as usize] as libc::c_int;
            col =
                (*p_ptr).px as libc::c_int +
                    ddx[option as usize] as libc::c_int;
            if see_obstacle(option, row, col) == 0 ||
                   see_obstacle(check_dir, row, col) == 0 {
                /* Can not see anything ahead and in the direction we */
				/* are turning, assume that it is a potential corner. */
                if find_examine as libc::c_int != 0 &&
                       see_nothing(option, row, col) != 0 &&
                       see_nothing(option2, row, col) != 0 {
                    find_current = option as byte_hack;
                    find_prevdir = option2 as byte_hack
                } else {
                    /* STOP: we are next to an intersection or a room */
                    return 1 as libc::c_int as bool_
                }
            } else if find_cut != 0 {
                find_current = option2 as byte_hack;
                find_prevdir = option2 as byte_hack
            } else {
                /* This corner is seen to be enclosed; we cut the corner. */
                /* This corner is seen to be enclosed, and we */
			/* deliberately go the long way. */
                find_current = option as byte_hack;
                find_prevdir = option2 as byte_hack
            }
        }
    }
    /* Don't see that it is closed off. */
			/* This could be a potential corner or an intersection. */
    /* About to hit a known wall, stop */
    if see_obstacle(find_current as libc::c_int, (*p_ptr).py as libc::c_int,
                    (*p_ptr).px as libc::c_int) != 0 {
        return 1 as libc::c_int as bool_
    }
    /* Failure */
    return 0 as libc::c_int as bool_;
}
/*
 * Take one step along the current "run" path
 */
#[no_mangle]
pub unsafe extern "C" fn run_step(mut dir: libc::c_int) {
    /* Start running */
    if dir != 0 {
        /* Hack -- do not start silly run */
        if see_obstacle(dir, (*p_ptr).py as libc::c_int,
                        (*p_ptr).px as libc::c_int) != 0 &&
               (*cave[((*p_ptr).py as libc::c_int +
                           ddy[dir as usize] as libc::c_int) as
                          usize].offset(((*p_ptr).px as libc::c_int +
                                             ddx[dir as usize] as libc::c_int)
                                            as isize)).feat as libc::c_int !=
                   0x60 as libc::c_int {
            /* Message */
            msg_print(b"You cannot run in that direction.\x00" as *const u8 as
                          *const libc::c_char);
            /* Disturb */
            disturb(0 as libc::c_int, 0 as libc::c_int);
            /* Done */
            return
        }
        /* Calculate torch radius */
        (*p_ptr).update =
            ((*p_ptr).update as libc::c_long | 0x2 as libc::c_long) as u32b;
        /* Initialize */
        run_init(dir);
    } else if run_test() != 0 {
        /* Keep running */
        /* Update run */
        /* Disturb */
        disturb(0 as libc::c_int, 0 as libc::c_int);
        /* Done */
        return
    }
    /* Decrease the run counter */
    running -= 1;
    if running as libc::c_int <= 0 as libc::c_int { return }
    /* Take time */
    energy_use = 100 as libc::c_int;
    /* Move the player, using the "pickup" flag */
    move_player_aux(find_current as libc::c_int, always_pickup as libc::c_int,
                    1 as libc::c_int, 1 as libc::c_int as bool_);
}
/*
 * Take care of the various things that can happen when you step
 * into a space. (Objects, traps, and stores.)
 */
#[no_mangle]
pub unsafe extern "C" fn step_effects(mut y: libc::c_int, mut x: libc::c_int,
                                      mut do_pickup: libc::c_int) {
    /* Handle "objects" */
    py_pickup_floor(do_pickup);
    /* Handle "store doors" */
    if (*cave[y as usize].offset(x as isize)).feat as libc::c_int ==
           0x4a as libc::c_int {
        /* Disturb */
        disturb(0 as libc::c_int, 0 as libc::c_int);
        /* Hack -- Enter store */
        command_new = ('V' as i32 & 0x1f as libc::c_int) as s16b
    } else if (*cave[y as usize].offset(x as isize)).t_idx as libc::c_int !=
                  0 as libc::c_int {
        /* Discover/set off traps */
        /* Disturb */
        disturb(0 as libc::c_int, 0 as libc::c_int);
        if (*cave[y as usize].offset(x as isize)).info as libc::c_int &
               0x100 as libc::c_int == 0 {
            /* Message */
            msg_print(b"You found a trap!\x00" as *const u8 as
                          *const libc::c_char);
            /* Pick a trap */
            pick_trap(y, x);
        }
        /* Hit the trap */
        hit_trap();
    };
}
/*
 * Issue a pet command
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_pet() {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut powers: [libc::c_int; 36] = [0; 36];
    let mut power_desc: [[libc::c_char; 80]; 36] = [[0; 80]; 36];
    let mut flag: bool_ = 0;
    let mut redraw: bool_ = 0;
    let mut ask: libc::c_int = 0;
    let mut choice: libc::c_char = 0;
    let mut out_val: [libc::c_char; 160] = [0; 160];
    let mut pets: libc::c_int = 0 as libc::c_int;
    let mut pet_ctr: libc::c_int = 0 as libc::c_int;
    let mut all_pets: bool_ = 0 as libc::c_int as bool_;
    let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
    num = 0 as libc::c_int;
    while num < 36 as libc::c_int {
        powers[num as usize] = 0 as libc::c_int;
        strcpy(power_desc[num as usize].as_mut_ptr(),
               b"\x00" as *const u8 as *const libc::c_char);
        num += 1
    }
    num = 0 as libc::c_int;
    if (*p_ptr).confused != 0 {
        msg_print(b"You are too confused to command your pets.\x00" as
                      *const u8 as *const libc::c_char);
        energy_use = 0 as libc::c_int;
        return
    }
    /* Calculate pets */
	/* Process the monsters (backwards) */
    pet_ctr = m_max as libc::c_int - 1 as libc::c_int;
    while pet_ctr >= 1 as libc::c_int {
        /* Access the monster */
        m_ptr = &mut *m_list.offset(pet_ctr as isize) as *mut monster_type;
        if (*m_ptr).status as libc::c_int >= 2 as libc::c_int { pets += 1 }
        pet_ctr -= 1
    }
    if pets == 0 as libc::c_int {
        msg_print(b"You have no pets/companions.\x00" as *const u8 as
                      *const libc::c_char);
        energy_use = 0 as libc::c_int;
        return
    } else {
        strcpy(power_desc[num as usize].as_mut_ptr(),
               b"dismiss pets\x00" as *const u8 as *const libc::c_char);
        let fresh3 = num;
        num = num + 1;
        powers[fresh3 as usize] = 1 as libc::c_int;
        strcpy(power_desc[num as usize].as_mut_ptr(),
               b"dismiss companions\x00" as *const u8 as *const libc::c_char);
        let fresh4 = num;
        num = num + 1;
        powers[fresh4 as usize] = 10 as libc::c_int;
        strcpy(power_desc[num as usize].as_mut_ptr(),
               b"call pets\x00" as *const u8 as *const libc::c_char);
        let fresh5 = num;
        num = num + 1;
        powers[fresh5 as usize] = 2 as libc::c_int;
        strcpy(power_desc[num as usize].as_mut_ptr(),
               b"follow me\x00" as *const u8 as *const libc::c_char);
        let fresh6 = num;
        num = num + 1;
        powers[fresh6 as usize] = 6 as libc::c_int;
        strcpy(power_desc[num as usize].as_mut_ptr(),
               b"seek and destroy\x00" as *const u8 as *const libc::c_char);
        let fresh7 = num;
        num = num + 1;
        powers[fresh7 as usize] = 3 as libc::c_int;
        if (*p_ptr).pet_open_doors != 0 {
            strcpy(power_desc[num as usize].as_mut_ptr(),
                   b"disallow open doors\x00" as *const u8 as
                       *const libc::c_char);
        } else {
            strcpy(power_desc[num as usize].as_mut_ptr(),
                   b"allow open doors\x00" as *const u8 as
                       *const libc::c_char);
        }
        let fresh8 = num;
        num = num + 1;
        powers[fresh8 as usize] = 4 as libc::c_int;
        if (*p_ptr).pet_pickup_items != 0 {
            strcpy(power_desc[num as usize].as_mut_ptr(),
                   b"disallow pickup items\x00" as *const u8 as
                       *const libc::c_char);
        } else {
            strcpy(power_desc[num as usize].as_mut_ptr(),
                   b"allow pickup items\x00" as *const u8 as
                       *const libc::c_char);
        }
        let fresh9 = num;
        num = num + 1;
        powers[fresh9 as usize] = 5 as libc::c_int;
        strcpy(power_desc[num as usize].as_mut_ptr(),
               b"give target to a friend\x00" as *const u8 as
                   *const libc::c_char);
        let fresh10 = num;
        num = num + 1;
        powers[fresh10 as usize] = 7 as libc::c_int;
        strcpy(power_desc[num as usize].as_mut_ptr(),
               b"give target to all friends\x00" as *const u8 as
                   *const libc::c_char);
        let fresh11 = num;
        num = num + 1;
        powers[fresh11 as usize] = 8 as libc::c_int;
        strcpy(power_desc[num as usize].as_mut_ptr(),
               b"friend forget target\x00" as *const u8 as
                   *const libc::c_char);
        let fresh12 = num;
        num = num + 1;
        powers[fresh12 as usize] = 9 as libc::c_int
    }
    /* Nothing chosen yet */
    flag = 0 as libc::c_int as bool_;
    /* No redraw yet */
    redraw = 0 as libc::c_int as bool_;
    /* Build a prompt (accept all spells) */
    if num <= 26 as libc::c_int {
        /* Build a prompt (accept all spells) */
        strnfmt(out_val.as_mut_ptr(), 78 as libc::c_int as uint_hack,
                b"(Command %c-%c, *=List, ESC=exit) Select a command: \x00" as
                    *const u8 as *const libc::c_char,
                0 as libc::c_int + 'a' as i32,
                num - 1 as libc::c_int + 'a' as i32);
    } else {
        strnfmt(out_val.as_mut_ptr(), 78 as libc::c_int as uint_hack,
                b"(Command %c-%c, *=List, ESC=exit) Select a command: \x00" as
                    *const u8 as *const libc::c_char,
                0 as libc::c_int + 'a' as i32,
                '0' as i32 + num - 27 as libc::c_int);
    }
    /* Get a command from the user */
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
                let fresh13 = y;
                y = y.wrapping_add(1);
                prt(b"\x00" as *const u8 as *const libc::c_char,
                    fresh13 as libc::c_int, x as libc::c_int);
                while ctr < num {
                    strnfmt(dummy.as_mut_ptr(),
                            80 as libc::c_int as uint_hack,
                            b"%c) %s\x00" as *const u8 as *const libc::c_char,
                            ctr + 'a' as i32,
                            power_desc[ctr as usize].as_mut_ptr());
                    prt(dummy.as_mut_ptr() as cptr, y as libc::c_int + ctr,
                        x as libc::c_int);
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
                ask = 0 as libc::c_int; /* Can't uppercase digits */
                i = choice as libc::c_int - '0' as i32 + 26 as libc::c_int
            }
            /* Totally Illegal */
            if i < 0 as libc::c_int || i >= num {
                bell();
            } else {
                /* Verify it */
                if ask != 0 {
                    let mut tmp_val: [libc::c_char; 160] = [0; 160];
                    /* Prompt */
                    strnfmt(tmp_val.as_mut_ptr(),
                            78 as libc::c_int as uint_hack,
                            b"Use %s? \x00" as *const u8 as
                                *const libc::c_char,
                            power_desc[i as usize].as_mut_ptr());
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
    /* Abort if needed */
    if flag == 0 { energy_use = 0 as libc::c_int; return }
    match powers[i as usize] {
        9 => {
            /* forget target */
            let mut m_ptr_0: *mut monster_type = 0 as *mut monster_type;
            let mut ii: libc::c_int = 0;
            let mut jj: libc::c_int = 0;
            msg_print(b"Select the friendly monster:\x00" as *const u8 as
                          *const libc::c_char);
            if tgt_pt(&mut ii, &mut jj) == 0 { return }
            if (*cave[jj as usize].offset(ii as isize)).m_idx != 0 {
                m_ptr_0 =
                    &mut *m_list.offset((*(*cave.as_mut_ptr().offset(jj as
                                                                         isize)).offset(ii
                                                                                            as
                                                                                            isize)).m_idx
                                            as isize) as *mut monster_type;
                if ((*m_ptr_0).status as libc::c_int) < 3 as libc::c_int {
                    msg_print(b"You cannot give orders to this monster.\x00"
                                  as *const u8 as *const libc::c_char);
                    return
                }
                (*m_ptr_0).target = -(1 as libc::c_int) as s16b
            }
        }
        8 => {
            /* Give target to all */
            let mut m_ptr_1: *mut monster_type = 0 as *mut monster_type;
            let mut ii_0: libc::c_int = 0;
            let mut jj_0: libc::c_int = 0;
            let mut i_0: libc::c_int = 0;
            msg_print(b"Select the target monster:\x00" as *const u8 as
                          *const libc::c_char);
            if tgt_pt(&mut ii_0, &mut jj_0) == 0 { return }
            if (*cave[jj_0 as usize].offset(ii_0 as isize)).m_idx != 0 {
                msg_print(b"Target selected.\x00" as *const u8 as
                              *const libc::c_char);
                i_0 = m_max as libc::c_int - 1 as libc::c_int;
                while i_0 >= 1 as libc::c_int {
                    /* Access the monster */
                    m_ptr_1 =
                        &mut *m_list.offset(i_0 as isize) as
                            *mut monster_type;
                    if !((*m_ptr_1).r_idx == 0) {
                        if !(((*m_ptr_1).status as libc::c_int) <
                                 3 as libc::c_int) {
                            (*m_ptr_1).target =
                                (*cave[jj_0 as
                                           usize].offset(ii_0 as isize)).m_idx
                        }
                    }
                    i_0 -= 1
                }
            } else {
                msg_print(b"This is not a correct target.\x00" as *const u8 as
                              *const libc::c_char);
                return
            }
        }
        1 => {
            /* Dismiss pets */
            let mut Dismissed: libc::c_int = 0 as libc::c_int;
            if get_check(b"Dismiss all pets? \x00" as *const u8 as
                             *const libc::c_char) != 0 {
                all_pets = 1 as libc::c_int as bool_
            }
            /* Process the monsters (backwards) */
            pet_ctr = m_max as libc::c_int - 1 as libc::c_int;
            while pet_ctr >= 1 as libc::c_int {
                let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
                /* Access the monster */
                m_ptr =
                    &mut *m_list.offset(pet_ctr as isize) as
                        *mut monster_type;
                r_ptr =
                    &mut *r_info.offset((*m_ptr).r_idx as isize) as
                        *mut monster_race;
                if (*r_ptr).flags7 & 0x400 as libc::c_int as libc::c_uint == 0
                       &&
                       ((*m_ptr).status as libc::c_int == 3 as libc::c_int ||
                            (*m_ptr).status as libc::c_int ==
                                2 as libc::c_int) {
                    /* Get rid of it! */
                    let mut checked: bool_ = 0 as libc::c_int as bool_;
                    let mut command: libc::c_char = 0;
                    let mut delete_this: bool_ = 0 as libc::c_int as bool_;
                    if all_pets != 0 {
                        delete_this = 1 as libc::c_int as bool_
                    } else {
                        let mut friend_name: [libc::c_char; 80] = [0; 80];
                        let mut check_friend: [libc::c_char; 80] = [0; 80];
                        monster_desc(friend_name.as_mut_ptr(), m_ptr,
                                     0x80 as libc::c_int);
                        strnfmt(check_friend.as_mut_ptr(),
                                80 as libc::c_int as uint_hack,
                                b"Dismiss %s? (Escape to cancel)\x00" as
                                    *const u8 as *const libc::c_char,
                                friend_name.as_mut_ptr());
                        while checked == 0 {
                            if get_com(check_friend.as_mut_ptr() as cptr,
                                       &mut command) == 0 {
                                /* get out of loop */
                                checked = 1 as libc::c_int as bool_;
                                pet_ctr = 0 as libc::c_int
                            } else {
                                match command as libc::c_int {
                                    89 | 121 => {
                                        delete_this =
                                            1 as libc::c_int as bool_;
                                        checked = 1 as libc::c_int as bool_
                                    }
                                    110 | 78 => {
                                        checked = 1 as libc::c_int as bool_
                                    }
                                    _ => { bell(); }
                                }
                            }
                        }
                    }
                    if delete_this != 0 {
                        delete_monster_idx(pet_ctr);
                        Dismissed += 1
                    }
                }
                pet_ctr -= 1
            }
            msg_format(b"You have dismissed %d pet%s.\x00" as *const u8 as
                           *const libc::c_char, Dismissed,
                       if Dismissed == 1 as libc::c_int {
                           b"\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"s\x00" as *const u8 as *const libc::c_char
                       });
        }
        10 => {
            /* Dismiss companions */
            let mut Dismissed_0: libc::c_int = 0 as libc::c_int;
            if get_check(b"Dismiss all companions? \x00" as *const u8 as
                             *const libc::c_char) != 0 {
                all_pets = 1 as libc::c_int as bool_
            }
            /* Process the monsters (backwards) */
            pet_ctr = m_max as libc::c_int - 1 as libc::c_int;
            while pet_ctr >= 1 as libc::c_int {
                let mut r_ptr_0: *mut monster_race = 0 as *mut monster_race;
                /* Access the monster */
                m_ptr =
                    &mut *m_list.offset(pet_ctr as isize) as
                        *mut monster_type;
                r_ptr_0 =
                    &mut *r_info.offset((*m_ptr).r_idx as isize) as
                        *mut monster_race;
                if (*r_ptr_0).flags7 & 0x400 as libc::c_int as libc::c_uint ==
                       0 && (*m_ptr).status as libc::c_int == 4 as libc::c_int
                   {
                    /* Get rid of it! */
                    let mut delete_this_0: bool_ = 0 as libc::c_int as bool_;
                    if all_pets != 0 {
                        delete_this_0 = 1 as libc::c_int as bool_
                    } else {
                        let mut friend_name_0: [libc::c_char; 80] = [0; 80];
                        let mut check_friend_0: [libc::c_char; 80] = [0; 80];
                        monster_desc(friend_name_0.as_mut_ptr(), m_ptr,
                                     0x80 as libc::c_int);
                        strnfmt(check_friend_0.as_mut_ptr(),
                                80 as libc::c_int as uint_hack,
                                b"Dismiss %s? \x00" as *const u8 as
                                    *const libc::c_char,
                                friend_name_0.as_mut_ptr());
                        if get_check(check_friend_0.as_mut_ptr() as cptr) != 0
                           {
                            delete_this_0 = 1 as libc::c_int as bool_
                        }
                    }
                    if delete_this_0 != 0 {
                        delete_monster_idx(pet_ctr);
                        Dismissed_0 += 1
                    }
                }
                pet_ctr -= 1
            }
            msg_format(b"You have dismissed %d companion%s.\x00" as *const u8
                           as *const libc::c_char, Dismissed_0,
                       if Dismissed_0 == 1 as libc::c_int {
                           b"\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"s\x00" as *const u8 as *const libc::c_char
                       });
        }
        2 => {
            /* Call pets */
            (*p_ptr).pet_follow_distance = 1 as libc::c_int as byte_hack
        }
        3 => {
            /* "Seek and destroy" */
            (*p_ptr).pet_follow_distance = 255 as libc::c_int as byte_hack
        }
        4 => {
            /* flag - allow pets to open doors */
            (*p_ptr).pet_open_doors =
                ((*p_ptr).pet_open_doors == 0) as libc::c_int as byte_hack
        }
        5 => {
            /* flag - allow pets to pickup items */
            (*p_ptr).pet_pickup_items =
                ((*p_ptr).pet_pickup_items == 0) as libc::c_int as byte_hack;
            /* Drop objects being carried by pets */
            if (*p_ptr).pet_pickup_items == 0 {
                pet_ctr = m_max as libc::c_int - 1 as libc::c_int;
                while pet_ctr >= 1 as libc::c_int {
                    /* Access the monster */
                    m_ptr =
                        &mut *m_list.offset(pet_ctr as isize) as
                            *mut monster_type;
                    if (*m_ptr).status as libc::c_int >= 3 as libc::c_int {
                        monster_drop_carried_objects(m_ptr);
                    }
                    pet_ctr -= 1
                }
            }
        }
        6 => {
            /* "Follow Me" */
            (*p_ptr).pet_follow_distance = 6 as libc::c_int as byte_hack
        }
        _ => { }
    };
}
/*
 * Incarnate into a body
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_integrate_body() -> bool_ {
    let mut q: cptr = 0 as *const libc::c_char;
    let mut s: cptr = 0 as *const libc::c_char;
    let mut item: libc::c_int = 0;
    let mut o_ptr: *mut object_type = 0 as *mut object_type;
    if (*p_ptr).disembodied == 0 {
        msg_print(b"You are already in a body.\x00" as *const u8 as
                      *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    /* Restrict choices to monsters */
    item_tester_tval = 9 as libc::c_int as byte_hack;
    /* Get an item */
    q = b"Incarnate in which body? \x00" as *const u8 as *const libc::c_char;
    s =
        b"You have no corpse to incarnate in.\x00" as *const u8 as
            *const libc::c_char;
    if get_item(&mut item, q, s, 0x4 as libc::c_int) == 0 {
        return 0 as libc::c_int as bool_
    }
    o_ptr =
        &mut *o_list.offset((0 as libc::c_int - item) as isize) as
            *mut object_type;
    if (*o_ptr).sval as libc::c_int != 1 as libc::c_int {
        msg_print(b"You must select a corpse.\x00" as *const u8 as
                      *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    (*p_ptr).body_monster = (*o_ptr).pval2 as u16b;
    (*p_ptr).chp = (*o_ptr).pval3 as s16b;
    floor_item_increase(0 as libc::c_int - item, -(1 as libc::c_int));
    floor_item_describe(0 as libc::c_int - item);
    floor_item_optimize(0 as libc::c_int - item);
    msg_print(b"Your spirit is incarnated in your new body.\x00" as *const u8
                  as *const libc::c_char);
    (*p_ptr).wraith_form = 0 as libc::c_int as bool_;
    (*p_ptr).disembodied = 0 as libc::c_int as bool_;
    do_cmd_redraw();
    return 1 as libc::c_int as bool_;
}
/*
 * Leave a body
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_leave_body(mut drop_body: bool_) -> bool_ {
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
    let mut r_ptr: *mut monster_race =
        &mut *r_info.offset((*p_ptr).body_monster as isize) as
            *mut monster_race;
    let mut i: libc::c_int = 0;
    if (*p_ptr).disembodied != 0 {
        msg_print(b"You are already disembodied.\x00" as *const u8 as
                      *const libc::c_char);
        return 0 as libc::c_int as bool_
    }
    i = 24 as libc::c_int;
    while i < 52 as libc::c_int {
        if (*p_ptr).body_parts[(i - 24 as libc::c_int) as usize] as
               libc::c_int != 0 &&
               (*p_ptr).inventory[i as usize].k_idx as libc::c_int != 0 &&
               (*p_ptr).inventory[i as usize].ident as libc::c_int &
                   0x40 as libc::c_int != 0 {
            msg_print(b"A cursed object is preventing you from leaving your body.\x00"
                          as *const u8 as *const libc::c_char);
            return 0 as libc::c_int as bool_
        }
        i += 1
    }
    if drop_body != 0 {
        if Rand_div(100 as libc::c_int) <
               25 as libc::c_int +
                   get_skill_scale(50 as libc::c_int,
                                   25 as libc::c_int as u32b) as libc::c_int +
                   get_skill(49 as libc::c_int) as libc::c_int {
            o_ptr = &mut forge;
            object_prep(o_ptr,
                        lookup_kind(9 as libc::c_int, 1 as libc::c_int) as
                            libc::c_int);
            (*o_ptr).number = 1 as libc::c_int as byte_hack;
            (*o_ptr).pval = 0 as libc::c_int;
            (*o_ptr).pval2 = (*p_ptr).body_monster as s16b;
            (*o_ptr).pval3 = (*p_ptr).chp as s32b;
            (*o_ptr).weight =
                (*r_ptr).weight +
                    Rand_div((*r_ptr).weight) / 10 as libc::c_int +
                    1 as libc::c_int;
            object_aware(o_ptr);
            object_known(o_ptr);
            (*o_ptr).ident =
                ((*o_ptr).ident as libc::c_int | 0x10 as libc::c_int) as
                    byte_hack;
            /* Unique corpses are unique */
            if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint != 0 {
                (*o_ptr).name1 = 201 as libc::c_int as byte_hack
            }
            drop_near(o_ptr, -(1 as libc::c_int), (*p_ptr).py as libc::c_int,
                      (*p_ptr).px as libc::c_int);
        } else {
            msg_print(b"You do not manage to keep the corpse from rotting away.\x00"
                          as *const u8 as *const libc::c_char);
        }
    }
    msg_print(b"Your spirit leaves your body.\x00" as *const u8 as
                  *const libc::c_char);
    (*p_ptr).disembodied = 1 as libc::c_int as bool_;
    /* Turn into a lost soul(just for the picture) */
    (*p_ptr).body_monster =
        test_monster_name(b"Lost soul\x00" as *const u8 as
                              *const libc::c_char) as u16b;
    do_cmd_redraw();
    return 1 as libc::c_int as bool_;
}
#[no_mangle]
pub unsafe extern "C" fn execute_inscription(mut i: byte_hack,
                                             mut y: byte_hack,
                                             mut x: byte_hack) -> bool_ {
    let mut c_ptr: *mut cave_type =
        &mut *(*cave.as_mut_ptr().offset(y as isize)).offset(x as isize) as
            *mut cave_type;
    /* Not enough mana in the current grid */
    if ((*c_ptr).mana as libc::c_int) <
           inscription_info[i as usize].mana as libc::c_int {
        return 1 as libc::c_int as bool_
    }
    /* Reduce the grid mana -- note: it can't be restored */
    (*c_ptr).mana =
        ((*c_ptr).mana as libc::c_int -
             inscription_info[i as usize].mana as libc::c_int) as byte_hack;
    /* Analyse inscription type */
    match i as libc::c_int {
        1 => {
            msg_print(b"The inscription shines in a bright light!\x00" as
                          *const u8 as *const libc::c_char);
            lite_room(y as libc::c_int, x as libc::c_int);
        }
        2 => {
            msg_print(b"The inscription is enveloped in a dark aura!\x00" as
                          *const u8 as *const libc::c_char);
            unlite_room(y as libc::c_int, x as libc::c_int);
        }
        3 => {
            msg_print(b"The inscription releases a powerful storm!\x00" as
                          *const u8 as *const libc::c_char);
            project(0 as libc::c_int, 3 as libc::c_int, y as libc::c_int,
                    x as libc::c_int,
                    damroll(10 as libc::c_int as s16b,
                            10 as libc::c_int as s16b), 1 as libc::c_int,
                    0x8 as libc::c_int | 0x10 as libc::c_int |
                        0x20 as libc::c_int | 0x40 as libc::c_int |
                        0x1 as libc::c_int);
        }
        4 => { return 0 as libc::c_int as bool_ }
        5 => {
            let mut yy: libc::c_int = y as libc::c_int;
            let mut xx: libc::c_int = x as libc::c_int;
            scatter(&mut yy, &mut xx, y as libc::c_int, x as libc::c_int,
                    3 as libc::c_int);
            place_monster_one(yy, xx,
                              test_monster_name(b"Dwarven Warrior\x00" as
                                                    *const u8 as
                                                    *const libc::c_char),
                              0 as libc::c_int, 0 as libc::c_int as bool_,
                              2 as libc::c_int);
        }
        6 => {
            let mut m_ptr: *mut monster_type = 0 as *mut monster_type;
            let mut r_ptr: *mut monster_race = 0 as *mut monster_race;
            let mut c_ptr_0: *mut cave_type = 0 as *mut cave_type;
            let mut ii: libc::c_int = x as libc::c_int;
            let mut ij: libc::c_int = y as libc::c_int;
            cave_set_feat(ij, ii, 0x57 as libc::c_int);
            msg_print(b"A chasm appears in the floor!\x00" as *const u8 as
                          *const libc::c_char);
            if (*cave[ij as usize].offset(ii as isize)).m_idx != 0 {
                m_ptr =
                    &mut *m_list.offset((*(*cave.as_mut_ptr().offset(ij as
                                                                         isize)).offset(ii
                                                                                            as
                                                                                            isize)).m_idx
                                            as isize) as *mut monster_type;
                r_ptr =
                    if !(*m_ptr).sr_ptr.is_null() {
                        (*m_ptr).sr_ptr
                    } else {
                        race_info_idx((*m_ptr).r_idx as libc::c_int,
                                      (*m_ptr).ego as libc::c_int)
                    };
                if (*r_ptr).flags7 & 0x4 as libc::c_int as libc::c_uint != 0 {
                    msg_print(b"The monster simply flies over the chasm.\x00"
                                  as *const u8 as *const libc::c_char);
                } else if (*r_ptr).flags1 & 0x1 as libc::c_int as libc::c_uint
                              == 0 {
                    msg_print(b"The monster falls in the chasm!\x00" as
                                  *const u8 as *const libc::c_char);
                    delete_monster_idx((*cave[ij as
                                                  usize].offset(ii as
                                                                    isize)).m_idx
                                           as libc::c_int);
                }
            }
            if (*cave[ij as usize].offset(ii as isize)).o_idx != 0 {
                let mut this_o_idx: s16b = 0;
                let mut next_o_idx: s16b = 0 as libc::c_int as s16b;
                c_ptr_0 =
                    &mut *(*cave.as_mut_ptr().offset(ij as
                                                         isize)).offset(ii as
                                                                            isize)
                        as *mut cave_type;
                /* Scan all objects in the grid */
                this_o_idx = (*c_ptr_0).o_idx;
                while this_o_idx != 0 {
                    let mut o_ptr: *mut object_type = 0 as *mut object_type;
                    let mut plural: bool_ = 0 as libc::c_int as bool_;
                    let mut o_name: [libc::c_char; 80] = [0; 80];
                    /* Acquire object */
                    o_ptr =
                        &mut *o_list.offset(this_o_idx as isize) as
                            *mut object_type;
                    if (*o_ptr).number as libc::c_int > 1 as libc::c_int {
                        plural = 1 as libc::c_int as bool_
                    }
                    /* Acquire next object */
                    next_o_idx = (*o_ptr).next_o_idx;
                    /* Effect "observed" */
                    if (*o_ptr).marked != 0 {
                        object_desc(o_name.as_mut_ptr(), o_ptr,
                                    0 as libc::c_int, 0 as libc::c_int);
                    }
                    /* Artifacts get to resist */
                    if (*o_ptr).name1 != 0 {
                        /* Observe the resist */
                        if (*o_ptr).marked != 0 {
                            msg_format(b"The %s %s simply fly over the chasm!\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       o_name.as_mut_ptr(),
                                       if plural as libc::c_int != 0 {
                                           b"are\x00" as *const u8 as
                                               *const libc::c_char
                                       } else {
                                           b"is\x00" as *const u8 as
                                               *const libc::c_char
                                       });
                        }
                    } else {
                        /* Kill it */
                        /* Delete the object */
                        delete_object_idx(this_o_idx as libc::c_int);
                        lite_spot(ij, ii);
                    }
                    this_o_idx = next_o_idx
                }
            }
        }
        7 => {
            msg_print(b"The inscription releases a blast of hellfire!\x00" as
                          *const u8 as *const libc::c_char);
            project(0 as libc::c_int, 3 as libc::c_int, y as libc::c_int,
                    x as libc::c_int, 200 as libc::c_int, 80 as libc::c_int,
                    0x8 as libc::c_int | 0x10 as libc::c_int |
                        0x20 as libc::c_int | 0x40 as libc::c_int |
                        0x1 as libc::c_int);
        }
        _ => { }
    }
    return 1 as libc::c_int as bool_;
}
/* Redraw */
/*
 * Choose an inscription and engrave it
 */
#[no_mangle]
pub unsafe extern "C" fn do_cmd_engrave() {
    let mut buf: [libc::c_char; 41] =
        *::std::mem::transmute::<&[u8; 41],
                                 &mut [libc::c_char; 41]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let mut i: byte_hack = 0;
    strnfmt(buf.as_mut_ptr(), 41 as libc::c_int as uint_hack,
            b"%s\x00" as *const u8 as *const libc::c_char,
            inscription_info[(*cave[(*p_ptr).py as
                                        usize].offset((*p_ptr).px as
                                                          isize)).inscription
                                 as usize].text.as_mut_ptr());
    get_string(b"Engrave what? \x00" as *const u8 as *const libc::c_char,
               buf.as_mut_ptr(), 40 as libc::c_int);
    /* Silently do nothing when player his escape or enters an empty string */
    if buf[0 as libc::c_int as usize] == 0 { return }
    i = 0 as libc::c_int as byte_hack;
    while (i as libc::c_int) < 8 as libc::c_int {
        if strcmp(inscription_info[i as usize].text.as_mut_ptr(),
                  buf.as_mut_ptr()) == 0 {
            if inscription_info[i as usize].know != 0 {
                /* Save the inscription */
                (*cave[(*p_ptr).py as
                           usize].offset((*p_ptr).px as isize)).inscription =
                    i as s16b
            } else {
                msg_print(b"You can\'t use this inscription for now.\x00" as
                              *const u8 as *const libc::c_char);
            }
        }
        i = i.wrapping_add(1)
    }
    /* Execute the inscription */
    if inscription_info[(*cave[(*p_ptr).py as
                                   usize].offset((*p_ptr).px as
                                                     isize)).inscription as
                            usize].when as libc::c_int & 0x1 as libc::c_int !=
           0 {
        execute_inscription((*cave[(*p_ptr).py as
                                       usize].offset((*p_ptr).px as
                                                         isize)).inscription
                                as byte_hack, (*p_ptr).py as byte_hack,
                            (*p_ptr).px as byte_hack);
    }
    energy_use += 300 as libc::c_int;
}
/*
 * Let's do a spinning around attack:                   -- DG --
 *     aDb
 *     y@k
 *     ooT
 * Ah ... all of those will get hit.
 */
#[no_mangle]
pub unsafe extern "C" fn do_spin() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    msg_print(b"You start spinning around...\x00" as *const u8 as
                  *const libc::c_char);
    j = (*p_ptr).py as libc::c_int - 1 as libc::c_int;
    while j <= (*p_ptr).py as libc::c_int + 1 as libc::c_int {
        i = (*p_ptr).px as libc::c_int - 1 as libc::c_int;
        while i <= (*p_ptr).px as libc::c_int + 1 as libc::c_int {
            /* Avoid stupid bugs */
            if j > 0 as libc::c_int && i > 0 as libc::c_int &&
                   j < cur_hgt as libc::c_int - 1 as libc::c_int &&
                   i < cur_wid as libc::c_int - 1 as libc::c_int &&
                   (*cave[j as usize].offset(i as isize)).m_idx as libc::c_int
                       != 0 {
                py_attack(j, i, 1 as libc::c_int);
            }
            i += 1
        }
        j += 1
    };
}
